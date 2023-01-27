use inkwell::types::BasicTypeEnum;
use log::debug;

use crate::sierra::errors::{CompilerError, CompilerResult};
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
    pub fn process_types(&mut self) -> CompilerResult<()> {
        debug!("processing types");

        // Check that the current state is valid.
        self.check_state(&CompilationState::NotStarted)?;
        for type_declaration in self.program.type_declarations.iter() {
            // Matching on the long id because it'll always have a debug name
            match &type_declaration.long_id.generic_id.debug_name {
                Some(type_name) => match type_name.as_str() {
                    "felt" => self.felt(&type_declaration),
                    "NonZero" => self.non_zero(&type_declaration),
                    "Struct" => self.sierra_struct(&type_declaration),
                    _ => println!("{type_name} is not a felt"),
                },
                _ => return Err(CompilerError::NoTypeProvided),
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::TypesProcessed)
    }
    pub fn get_type_from_name(&self, type_name: &str) -> CompilerResult<BasicTypeEnum<'ctx>> {
        Ok(self
            .types
            .get(
                self.id_from_name
                    .get(type_name)
                    .ok_or(CompilerError::TypeNotFound(type_name.to_owned()))?,
            )
            .ok_or(CompilerError::TypeNotFound(type_name.to_owned()))?
            .as_basic_type_enum())
    }
}
