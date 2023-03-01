use tracing::debug;

use crate::sierra::errors::CompilerResult;
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
        self.check_state(&CompilationState::DebugSetup)?;
        for type_declaration in self.program.type_declarations.iter() {
            self.current_line_estimate += 1;
            // All those types are known in advance. A struct is a combination of multiple "primitive" types
            let type_name = type_declaration.long_id.generic_id.0.as_str();
            match type_name {
                // Regular felt
                "felt" => self.felt(type_declaration),
                // NonZero<felt> is a felt != 0
                "NonZero" => self.non_zero(type_declaration),
                // Box<T>
                "Box" => self.sierra_box(type_declaration),
                // Regular struct
                "Struct" => self.sierra_struct(type_declaration),
                _ => debug!(type_name, "unimplemented type"),
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::TypesProcessed)
    }
}
