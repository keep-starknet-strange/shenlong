use inkwell::types::BasicType;
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
                    "felt" => {
                        self.id_from_name.insert("felt".to_owned(), type_declaration.id.id);
                        self.types.insert(
                            type_declaration.id.id,
                            Box::new(self.context.custom_width_int_type(252).as_basic_type_enum()),
                        );
                    }
                    "NonZero" => (),
                    _ => println!("{type_name} is not a felt"),
                },
                _ => return Err(CompilerError::NoTypeProvided),
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::TypesProcessed)
    }
}
