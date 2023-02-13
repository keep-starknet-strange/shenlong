use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::Param;
use inkwell::types::BasicMetadataTypeEnum;
use log::debug;

use crate::sierra::errors::{CompilerResult, DEBUG_NAME_EXPECTED};
use crate::sierra::llvm_compiler::{CompilationState, Compiler};

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

        // Clear the variables map in case a previous function has been processed.
        self.variables.clear();

        // Loop through the function declarations (last category of the sierra file).
        for func_declaration in self.program.funcs.iter() {
            // Arguments of the function.
            let mut args = vec![];
            for Param { id: _, ty: ConcreteTypeId { id: type_id, debug_name: _debug_name } } in &func_declaration.params
            {
                // Get the type of the argument. Panics if the type is not found because it should have been
                // declared at the begining of the sierra file.
                let ty =
                    self.types.get(&type_id.to_string()).expect("Function argument type should have been declared");
                args.push(ty);
            }
            // Create return type if the function returns something.
            let return_type = if !func_declaration.signature.ret_types.is_empty() {
                let mut ret_types = vec![];
                // In case the function returns multiple values collect all the types.
                for ret_type in &func_declaration.signature.ret_types {
                    ret_types.push(
                        self.types
                            .get(&ret_type.id.to_string())
                            .expect("Type should have been declared before function")
                            .as_basic_type_enum(),
                    );
                }
                Some(self.context.struct_type(&ret_types, false))
            } else {
                None
            };

            // Map the arguments into the correct type to define the type of the function.
            let args_metadata = &args
                .clone()
                .into_iter()
                .map(|arg_type| arg_type.as_basic_type_enum().into())
                .collect::<Vec<BasicMetadataTypeEnum>>();
            // Declare the function type (if it's the main function strip everything so it's recognised like the
            // main function)
            let func = self.module.add_function(
                if func_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().ends_with("::main") {
                    "main".to_owned()
                } else {
                    func_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string()
                }
                .as_str(),
                match return_type {
                    Some(ret) => ret.fn_type(args_metadata, false),
                    None => self.context.void_type().fn_type(args_metadata, false),
                },
                None,
            );
            self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
            // Loop through the arguments of the function. The variable id counter always starts from zero for a
            // function.
            for (var_id, _) in args.iter().enumerate() {
                // Save the function arguments in the variables map to be able to use them in the function body.
                self.variables.insert(
                    var_id.to_string(),
                    func.get_nth_param(var_id as u32).expect("Function should have enough parameters"),
                );
            }
            // process statements from the line stated in the function definition until the return instruction.
            // ex: fib_caller::fib_caller::main@21() -> (Unit); the function main starts at the statement 21.
            self.process_statements_from_until(func_declaration.entry_point.0, None)?;
        }
        // Move to the next state.
        self.move_to(CompilationState::FunctionsProcessed)
    }
}
