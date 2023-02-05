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
        self.variables.clear();

        for func_declaration in self.program.funcs.iter() {
            let mut args = vec![];
            for Param { id: _, ty: ConcreteTypeId { id: type_id, debug_name: _debug_name } } in &func_declaration.params
            {
                // Save the arguments for the function to be able to retreive the variables in the
                // satements and jump to statement number entrypoint.
                let ty =
                    self.types.get(&type_id.to_string()).expect("Function argument type should have been declared");
                args.push(ty);
            }

            let return_type = if !func_declaration.signature.ret_types.is_empty() {
                let mut ret_types = vec![];
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

            let args_metadata = &args
                .clone()
                .into_iter()
                .map(|arg_type| arg_type.as_basic_type_enum().into())
                .collect::<Vec<BasicMetadataTypeEnum>>();

            let func = self.module.add_function(
                func_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
                match return_type {
                    Some(ret) => ret.fn_type(args_metadata, false),
                    None => self.context.void_type().fn_type(args_metadata, false),
                },
                None,
            );
            self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
            for (var_id, _) in args.iter().enumerate() {
                let var_id = &(var_id as u64);
                self.variables.insert(
                    var_id.to_string(),
                    func.get_nth_param((*var_id) as u32).expect("Function should have enough parameters"),
                );
            }
            // process statements until return
            self.process_statements_from_until_return(func_declaration.entry_point.0)?;
        }
        // Move to the next state.
        self.move_to(CompilationState::FunctionsProcessed)
    }
}
