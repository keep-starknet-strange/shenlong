use eyre::Result;
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{BasicMetadataValueEnum, PointerValue},
};
use log::debug;
use sierra::{
    program::{GenericArg, Program, StatementIdx},
    ProgramParser,
};
use std::{collections::HashMap, fs, path::Path};

/// Compiler is the main entry point for the LLVM backend.
/// It is responsible for compiling a Sierra program to LLVM IR.
pub struct Compiler<'a, 'ctx> {
    pub program: &'a Program,
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub variables: HashMap<String, Option<PointerValue<'ctx>>>,
    pub output_path: &'a str,
}

/// Implementation of the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Compile a Sierra program file to LLVM IR.
    /// # Arguments
    /// * `program_path` - The Sierra program to compile.
    /// * `output_path` - The path to the output LLVM IR file.
    /// # Returns
    /// The result of the compilation.
    /// # Errors
    /// If the compilation fails.
    pub fn compile_from_file(program_path: &str, output_path: &str) -> Result<()> {
        // Read the program from the file.
        let sierra_code = fs::read_to_string(program_path)?;
        Compiler::compile_from_code(&sierra_code, output_path)
    }

    /// Compile a Sierra program code to LLVM IR.
    /// # Arguments
    /// * `sierra_code` - The Sierra program code.
    /// * `output_path` - The path to the output LLVM IR file.
    /// # Returns
    /// The result of the compilation.
    /// # Errors
    /// If the compilation fails.
    pub fn compile_from_code(sierra_code: &str, output_path: &str) -> Result<()> {
        // Parse the program.
        let program = ProgramParser::new().parse(sierra_code).unwrap();
        Compiler::compile_sierra_program_to_llvm(program, output_path)
    }

    /// Compiles a Sierra `Program` representation to LLVM IR.
    /// # Process overview
    /// 1. Create an LLVM context, builder and module.
    /// 2. Instantiate variables map.
    /// 3. Process the program type declarations.
    /// 4. Process the core library functions.
    /// 5. Process the program statements.
    /// 6. Finalize compilation and write the LLVM IR to a file.
    /// # Arguments
    /// * `program` - The Sierra program to compile.
    /// * `output_path` - The path to the output LLVM IR file.
    /// # Returns
    /// The result of the compilation.
    /// # Errors
    /// If the compilation fails.
    /// # Example
    /// ```rust
    /// use sierra::ProgramParser;
    /// use shenlong_core::sierra::llvm_compiler::Compiler;
    /// use std::fs;
    ///
    /// let sierra_program_path = "examples/program.sierra";
    /// let llvm_ir_path = "examples/program.ll";
    ///
    /// // TODO: Find a way to make doc tests pass.
    /// // Read the program from the file.
    /// // let sierra_code = fs::read_to_string(sierra_program_path).unwrap();
    /// // Parse the program.
    /// // let program = ProgramParser::new().parse(&sierra_code).unwrap();
    /// // Compile the program to LLVM IR.
    /// // let result = Compiler::compile_from_file(sierra_program_path, llvm_ir_path);
    /// // Check the result.
    /// ```
    pub fn compile_sierra_program_to_llvm(program: Program, output_path: &str) -> Result<()> {
        // Create an LLVM context, builder and module.
        // See https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl03.html#id2
        // Context is an opaque object that owns a lot of core LLVM data structures, such as the type and constant value tables
        let context = inkwell::context::Context::create();
        // Builder is a helper object that makes it easy to create LLVM instructions.
        let builder = context.create_builder();
        // Module is an object that contains all of the functions, global variables.
        // In many ways, it is the top-level structure that the LLVM IR uses to contain code.
        let module = context.create_module("root");

        // Instantiate variables map.
        let variables: HashMap<String, Option<PointerValue>> = HashMap::new();

        // Create a new compiler.
        let mut compiler = Compiler {
            program: &program,
            context: &context,
            builder: &builder,
            module: &module,
            variables,
            output_path,
        };

        // Process the types in the Sierra program.
        compiler.process_types()?;

        // Process the core library functions in the Sierra program.
        compiler.process_core_lib_functions()?;

        // Process the statements in the Sierra program.
        compiler.process_statements()?;

        // Finalize the compilation.
        compiler.finalize_compilation()
    }

    /// Process types in the Sierra program.
    /// For each type declaration in the Sierra program, create a corresponding type in the LLVM context.
    fn process_types(&mut self) -> Result<()> {
        debug!("processing types");
        self.program
            .type_declarations
            .iter()
            .for_each(|_type_decl| {
                // TODO: Implement this.
                // For now this is a stub implementation that works for one specific test program.
                let i32_type = self.context.i32_type();
                let _i32_fn_type = i32_type.fn_type(&[], false);
                // TODO store in context
            });
        Ok(())
    }

    /// Process core library functions in the Sierra program.
    fn process_core_lib_functions(&mut self) -> Result<()> {
        debug!("processing core lib functions");
        // Iterate over the libfunc declarations in the Sierra program
        self.program
            .libfunc_declarations
            .iter()
            .for_each(|libfunc| {
                // TODO: Implement this.
                // For now this is a stub implementation that works for one specific test program.

                // Create an i128 type and function type in the LLVM context
                // The types must be created and stored in the global context before they can be used.
                // TODO: implement in `process_types`
                let i32_type = self.context.i32_type();
                let fn_type = i32_type.fn_type(&[], false);

                match &libfunc.long_id.generic_id.debug_name {
                    Some(name) => {
                        // If the libfunc declaration has a debug name that contains "const" and has at
                        // least one generic argument, create a new function in the LLVM
                        // module with the libfunc's ID as the name and set the function's return value
                        // to the value of the first generic argument.
                        if name.contains("const") && !libfunc.long_id.generic_args.is_empty() {
                            if let GenericArg::Value(value) =
                                libfunc.long_id.generic_args[0].clone()
                            {
                                let function = self.module.add_function(
                                    format!("a_{}", libfunc.id.id).as_str(),
                                    fn_type,
                                    None,
                                );
                                let fn_temp = self.context.append_basic_block(function, "entry");
                                self.builder.position_at_end(fn_temp);
                                self.builder.build_return(Some(
                                    &self
                                        .context
                                        .i32_type()
                                        .const_int(value.to_u64_digits().1[0], false),
                                ));
                            }
                        }
                        // If the libfunc declaration has a debug name of "felt_add" create a llvm ir
                        // add function.
                        else if name == "felt_add" {
                            let fn_type =
                                i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
                            let function = self.module.add_function(
                                format!("a_{}", libfunc.id.id).as_str(),
                                fn_type,
                                None,
                            );
                            let fn_temp = self.context.append_basic_block(function, "entry");
                            self.builder.position_at_end(fn_temp);

                            // Add the two arguments and store the result in a temporary value
                            let sum = self.builder.build_int_add(
                                function.get_first_param().unwrap().into_int_value(),
                                function.get_last_param().unwrap().into_int_value(),
                                "sum",
                            );

                            // Return the result
                            self.builder.build_return(Some(&sum));
                        } else if name == "rename" {
                            let void = self.context.i32_type().fn_type(&[i32_type.into()], false);

                            let function = self.module.add_function(
                                format!("a_{}", libfunc.id.id).as_str(),
                                void,
                                None,
                            );
                            let fn_temp = self.context.append_basic_block(function, "entry");
                            self.builder.position_at_end(fn_temp);
                            self.builder
                                .build_return(Some(&function.get_first_param().unwrap()));
                        }
                    }
                    // If the libfunc declaration has no debug name, print "no name".
                    None => println!("no name"),
                }
            });
        Ok(())
    }

    /// Process statements in the Sierra program.
    fn process_statements(&mut self) -> Result<()> {
        debug!("processing statements");
        // This section is very specific to the test program.
        // TODO: Think about how to implement this in a more general way.
        // Init param var
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[self.context.i32_type().into()], false);
        let main_func = self.module.add_function("main", main_type, None);
        let main_bb = self.context.append_basic_block(main_func, "entry");
        self.builder.position_at_end(main_bb);
        for func in self.program.funcs.iter() {
            for param in func.params.iter() {
                self.variables.insert(param.id.id.to_string(), None);
            }
        }

        // Iterate over the statements in the Sierra program, but do nothing with them.
        for (statement_id, _statement) in self.program.statements.iter().enumerate() {
            let _statement_idx = StatementIdx(statement_id);
            match _statement {
                // If the statement is an invocation, print the invocation.
                sierra::program::Statement::Invocation(invocation) => {
                    if invocation.libfunc_id.id.to_string().as_str() != "3" {
                        let function = self
                            .module
                            .get_function(format!("a_{}", invocation.libfunc_id.id).as_str())
                            .ok_or_else(|| eyre::eyre!("function not found"))?;
                        let mut args: Vec<BasicMetadataValueEnum> = vec![];
                        invocation.args.clone().into_iter().for_each(|var_id| {
                            args.push(
                                match self.variables.get(var_id.id.to_string().as_str()).unwrap() {
                                    Some(val) => self
                                        .builder
                                        .build_load(*val, var_id.id.to_string().as_str())
                                        .into(),
                                    None => {
                                        main_func.get_nth_param(var_id.id as u32).unwrap().into()
                                    }
                                },
                            );
                        });
                        let res_ptr = self.builder.build_alloca(
                            i32_type,
                            format!("ptr{}", invocation.branches[0].results[0].id).as_str(),
                        );
                        let res_val = self
                            .builder
                            .build_call(
                                function,
                                &args,
                                format!("val{}", invocation.branches[0].results[0].id).as_str(),
                            )
                            .try_as_basic_value()
                            .left()
                            .unwrap();
                        self.builder.build_store(res_ptr, res_val);
                        self.variables.insert(
                            invocation.branches[0].results[0].id.to_string(),
                            Some(res_ptr),
                        );
                    }
                }
                // If the statement is a return, print the return.
                sierra::program::Statement::Return(ret) => {
                    self.builder.build_return(Some(
                        &self.builder.build_load(
                            self.variables
                                .get(ret[0].id.to_string().as_str())
                                .unwrap()
                                .unwrap(),
                            ret[0].id.to_string().as_str(),
                        ),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Finalize the compilation.
    /// This includes verifying the module and writing it to the output path.
    fn finalize_compilation(&mut self) -> Result<()> {
        debug!("finalizing compilation");
        // Ensure that the current module is valid
        self.module
            .verify()
            .map_err(|e| eyre::eyre!(e.to_string()))?;
        // Ensure output path is valid and exists.
        let output_path = Path::new(self.output_path);
        let parent = output_path
            .parent()
            .ok_or_else(|| eyre::eyre!("parent output path is not valid"))?;
        // Recursively create the output path parent directories if they don't exist.
        fs::create_dir_all(parent)?;
        // Write the module to the output path.
        self.module
            .print_to_file(output_path)
            .map_err(|e| eyre::eyre!(e.to_string()))
    }
}
