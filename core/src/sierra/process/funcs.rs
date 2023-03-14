use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenFunction, Param, StatementIdx};
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use tracing::debug;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::{CompilationState, Compiler, FunctionInfo};
use crate::sierra::process::corelib::PRINT_RETURN;

/// Implementation for the type processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process types in the Sierra program.
    /// For each type declaration in the Sierra program, create a corresponding type in the LLVM
    /// context.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra types fails.
    pub fn process_funcs(&mut self) -> CompilerResult<()> {
        debug!("processing funcs");
        // Check that the current state is valid.
        self.check_state(&CompilationState::CoreLibFunctionsProcessed)?;

        let debug_line_before = self.debug.current_line;

        // Jump over the statement block (and the two expected lines of whitespace) to the function
        // declarations TODO obtain line numbers from sierra file
        self.debug.current_line += self.program.statements.len() as u32 + 2;

        // Loop through the function declarations (last category of the sierra file).
        for func_declaration in self.program.funcs.iter() {
            let func_name = func_declaration.id.debug_name.as_ref().unwrap().as_str();
            debug!("processing function declaration: {}", func_name);

            self.generate_function_definition(func_declaration);
            self.debug.current_line += 1;
        }

        self.debug.current_line = debug_line_before;

        // Move to the next state.
        self.move_to(CompilationState::FunctionsProcessed)
    }

    /// Generates the function "definition".
    ///
    /// libfunc_id is the real libfunc_id, used in invocations.
    pub fn generate_function_definition(&mut self, func_declaration: &GenFunction<StatementIdx>) -> FunctionInfo<'ctx> {
        let func_name = func_declaration.id.debug_name.as_ref().unwrap().as_str();

        if let Some(info) = self.user_functions.get(func_name) {
            debug!(func_name, "returning pre-generated function definition");
            return info.clone();
        }

        debug!(func_name, "generating function definition");

        self.debug.debug_location(None);

        // Arguments of the function.
        let mut args = vec![];
        let mut args_debug_types = vec![];

        for Param { id: _, ty: ConcreteTypeId { id: type_id, debug_name: _debug_name } } in &func_declaration.params {
            // Get the type of the argument. Panics if the type is not found because it should have been
            // declared at the beginning of the sierra file.
            let ty = *self.types_by_id.get(type_id).expect("Function argument type should have been declared");
            args.push(ty);
            let debug_type = self.debug.types_by_id.get(type_id).unwrap();
            args_debug_types.push(*debug_type);
        }

        // Create return type if the function returns something.

        let return_info = if !func_declaration.signature.ret_types.is_empty() {
            // If the function returns a single value, return it directly.
            if func_declaration.signature.ret_types.len() == 1 {
                let ty = *self.types_by_id.get(&func_declaration.signature.ret_types[0].id).unwrap();
                let debug_ty = *self.debug.types_by_id.get(&func_declaration.signature.ret_types[0].id).unwrap();
                Some((ty, debug_ty))
            } else {
                // In case the function returns multiple values collect all the types into a struct.
                let mut ret_types = vec![];
                let mut ret_debug_types = vec![];

                for ret_type in &func_declaration.signature.ret_types {
                    ret_types.push(
                        *self.types_by_id.get(&ret_type.id).expect("Type should have been declared before function"),
                    );
                    ret_debug_types.push(*self.debug.types_by_id.get(&ret_type.id).unwrap());
                }
                let generated_return_struct_type = self.context.struct_type(&ret_types, false);
                // Arbitrarely decided generated struct return types have id = the function id + 100_000.
                // Arbitrarely decided generated struct return types have name = "return_type_{}" where {} is the
                // function name.
                let debug_type = self.debug.create_struct(
                    self.debug.get_fn_struct_type_id(func_declaration.id.id),
                    &self.debug.get_fn_struct_type_name(func_name),
                    &generated_return_struct_type,
                    &ret_debug_types,
                );
                Some((generated_return_struct_type.as_basic_type_enum(), debug_type))
            }
        } else {
            None
        };

        // Map the arguments into the correct type to define the type of the function.
        let args_metadata = &args
            .clone()
            .into_iter()
            .map(|arg_type| arg_type.as_basic_type_enum().into())
            .collect::<Vec<BasicMetadataTypeEnum>>();

        // Declare the function type (if it's the main function strip everything so it's recognized like the
        // main function)
        let func = if let Some(ret_ty) = return_info && func_name.ends_with("::main") {
                let ret_ty = ret_ty.0;
                // Generate the print function for the return value type.

                if ret_ty.is_int_type() {
                    let ret_ty = ret_ty.into_int_type();
                    self.printf_for_type(ret_ty.into(), PRINT_RETURN, "felt");
                }
                else if ret_ty.is_struct_type() {
                    let ret_ty = ret_ty.into_struct_type();

                    if ret_ty.count_fields() > 0 {
                        // If there is a return value, it will always be 1, (tuples are a struct, they count as one).
                        let field = ret_ty.get_field_type_at_index(0).unwrap();

                        if field.is_int_type() {
                            self.printf_for_type(field.into(), PRINT_RETURN, "felt");
                        }
                        // If its a struct, print all their values
                        else if field.is_struct_type() {
                            let field = field.into_struct_type();

                            for f in field.get_field_types() {
                                if f.is_int_type() {
                                    self.printf_for_type(f.into(), PRINT_RETURN, "felt");
                                }
                            }
                        }
                    }
                }

                self.module.add_function("main", self.context.i32_type().fn_type(args_metadata, false), None)
            }
            else {
                self.module.add_function(
                    func_name,
                    match return_info {
                        Some(ret) => ret.0.fn_type(args_metadata, false),
                        None => self.context.void_type().fn_type(args_metadata, false),
                    },
                    None,
                )
            };

        let debug_function = self.debug.create_function(
            func_name,
            &func,
            return_info.map(|x| x.1),
            &args_debug_types,
            Some(func_declaration.entry_point.0),
        );

        let info = FunctionInfo { func, entry_point: func_declaration.entry_point.0, args, debug: debug_function };

        self.user_functions.insert(func_name.to_string(), info.clone());

        info
    }
}
