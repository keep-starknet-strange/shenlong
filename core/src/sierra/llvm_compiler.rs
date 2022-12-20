use eyre::Result;
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, PointerValue};
use sierra::program::{GenericArg, Program, StatementIdx};
use std::collections::HashMap;

// Compiler is the main entry point for the LLVM backend.
// It is responsible for compiling a Sierra program to LLVM IR.
pub struct Compiler {
    // ...
}

impl Compiler {
    /// Creates a new compiler.
    /// # Returns
    /// A new compiler.
    pub fn new() -> Self {
        Compiler {}
    }

    /// Compiles a Sierra program to LLVM IR.
    /// # Arguments
    /// * `program_path` - The Sierra program to compile.
    /// * `output_path` - The path to the output LLVM IR file.
    /// # Returns
    /// The result of the compilation.
    /// # Errors
    /// If the compilation fails.
    pub fn compile(&self, program: Program, output_path: &str) -> Result<()> {
        let mut variables: HashMap<String, Option<PointerValue>> = HashMap::new();

        // Create an LLVM context and module
        let context = inkwell::context::Context::create();
        let builder = context.create_builder();
        let module = context.create_module("test");

        // Create an i128 type and function type in the LLVM context
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);

        // Iterate over the libfunc declarations in the Sierra program
        program.libfunc_declarations.iter().for_each(|libfunc| {
            match &libfunc.long_id.generic_id.debug_name {
                Some(name) => {
                    // If the libfunc declaration has a debug name that contains "const" and has at
                    // least one generic argument, create a new function in the LLVM
                    // module with the libfunc's ID as the name and set the function's return value
                    // to the value of the first generic argument.
                    if name.contains("const") && !libfunc.long_id.generic_args.is_empty() {
                        if let GenericArg::Value(value) = libfunc.long_id.generic_args[0].clone() {
                            let function = module.add_function(
                                format!("a_{}", libfunc.id.id).as_str(),
                                fn_type,
                                None,
                            );
                            let fn_temp = context.append_basic_block(function, "entry");
                            builder.position_at_end(fn_temp);
                            builder.build_return(Some(
                                &context
                                    .i32_type()
                                    .const_int(value.to_u64_digits().1[0], false),
                            ));
                        }
                    }
                    // If the libfunc declaration has a debug name of "felt_add" create a llvm ir
                    // add function.
                    else if name == "felt_add" {
                        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
                        let function = module.add_function(
                            format!("a_{}", libfunc.id.id).as_str(),
                            fn_type,
                            None,
                        );
                        let fn_temp = context.append_basic_block(function, "entry");
                        builder.position_at_end(fn_temp);

                        // Add the two arguments and store the result in a temporary value
                        let sum = builder.build_int_add(
                            function.get_first_param().unwrap().into_int_value(),
                            function.get_last_param().unwrap().into_int_value(),
                            "sum",
                        );

                        // Return the result
                        builder.build_return(Some(&sum));
                    } else if name == "rename" {
                        let void = context.i32_type().fn_type(&[i32_type.into()], false);

                        let function = module.add_function(
                            format!("a_{}", libfunc.id.id).as_str(),
                            void,
                            None,
                        );
                        let fn_temp = context.append_basic_block(function, "entry");
                        builder.position_at_end(fn_temp);
                        builder.build_return(Some(&function.get_first_param().unwrap()));
                    }
                }
                // If the libfunc declaration has no debug name, print "no name".
                None => println!("no name"),
            }
        });
        // init param var
        let main_type = i32_type.fn_type(&[context.i32_type().into()], false);
        let main_func = module.add_function("main", main_type, None);
        let main_bb = context.append_basic_block(main_func, "entry");
        builder.position_at_end(main_bb);
        program.funcs.into_iter().for_each(|func| {
            func.params.into_iter().for_each(|param| {
                variables.insert(param.id.id.to_string(), None);
            })
        });
        // Iterate over the statements in the Sierra program, but do nothing with them.
        for (statement_id, _statement) in program.statements.iter().enumerate() {
            let _statement_idx = StatementIdx(statement_id);
            match _statement {
                // If the statement is an invocation, print the invocation.
                sierra::program::Statement::Invocation(invocation) => {
                    if invocation.libfunc_id.id.to_string().as_str() != "3" {
                        let function = module
                            .get_function(format!("a_{}", invocation.libfunc_id.id).as_str())
                            .unwrap();
                        let mut args: Vec<BasicMetadataValueEnum> = vec![];
                        invocation.args.clone().into_iter().for_each(|var_id| {
                            args.push(
                                match variables.get(var_id.id.to_string().as_str()).unwrap() {
                                    Some(val) => builder
                                        .build_load(*val, var_id.id.to_string().as_str())
                                        .into(),
                                    None => {
                                        main_func.get_nth_param(var_id.id as u32).unwrap().into()
                                    }
                                },
                            );
                        });
                        let res_ptr = builder.build_alloca(
                            i32_type,
                            format!("ptr{}", invocation.branches[0].results[0].id).as_str(),
                        );
                        let res_val = builder
                            .build_call(
                                function,
                                &args,
                                format!("val{}", invocation.branches[0].results[0].id).as_str(),
                            )
                            .try_as_basic_value()
                            .left()
                            .unwrap();
                        builder.build_store(res_ptr, res_val);
                        variables.insert(
                            invocation.branches[0].results[0].id.to_string(),
                            Some(res_ptr),
                        );
                    }
                }
                // If the statement is a return, print the return.
                sierra::program::Statement::Return(ret) => {
                    builder.build_return(Some(
                        &builder.build_load(
                            variables
                                .get(ret[0].id.to_string().as_str())
                                .unwrap()
                                .unwrap(),
                            ret[0].id.to_string().as_str(),
                        ),
                    ));
                }
            }
            println!(
                "{:#?}",
                module.get_functions().collect::<Vec<FunctionValue>>()
            );
        }
        module.verify().unwrap();
        module.print_to_file(output_path).unwrap();
        Ok(())
    }
}

/// `Default` implementation for `Compiler`.
impl Default for Compiler {
    /// Creates a new default compiler.
    /// # Returns
    /// A new default compiler.
    fn default() -> Self {
        Self::new()
    }
}
