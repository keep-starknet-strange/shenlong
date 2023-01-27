use log::debug;

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
    pub fn process_funcs(&mut self) -> CompilerResult<()> {
        debug!("processing funcs");
        // Check that the current state is valid.
        self.check_state(&CompilationState::TypesProcessed)?;

        // for func_declaration in self.program.funcs.iter() {
        //     ()
        // }
        // Move to the next state.
        self.move_to(CompilationState::FunctionsProcessed)
    }
}
