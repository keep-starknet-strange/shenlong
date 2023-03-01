use tracing::debug;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::{CompilationState, Compiler};
use crate::sierra::types::felt::DOUBLE_FELT_INT_WIDTH;

/// Implementation for the type processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Sets up the debug info.
    pub fn setup_debug(&mut self) -> CompilerResult<()> {
        debug!("setting up the debug info");

        // Check that the current state is valid.
        self.check_state(&CompilationState::NotStarted)?;

        // self.create_debug_type("u8", 8);
        // self.create_debug_type("u32", 32);
        // self.create_debug_type("u64", 64);
        // self.create_debug_type("u128", 128);
        // self.create_debug_type("felt", FELT_INT_WIDTH.into());
        // self.create_debug_type("double_felt", DOUBLE_FELT_INT_WIDTH.into());

        // double_felt doesn't have a numeric id from sierra.
        self.debug.create_debug_type(200000, "double_felt", DOUBLE_FELT_INT_WIDTH.into());

        self.move_to(CompilationState::DebugSetup)
    }
}
