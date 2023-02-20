use cairo_lang_sierra::program::{GenBranchTarget, GenStatement};

use crate::sierra::errors::{CompilerResult, DEBUG_NAME_EXPECTED};
use crate::sierra::llvm_compiler::Compiler;

/// Implementation for the type processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process types in the Sierra program.
    /// For each type declaration in the Sierra program, create a corresponding type in the LLVM
    /// context.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra types fails.
    pub fn collect_jumps(&mut self) -> CompilerResult<()> {
        for statement in self.program.statements.iter() {
            match statement {
                GenStatement::Invocation(invocation) => {
                    if invocation.libfunc_id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).as_str() == "jump" && 
                    let GenBranchTarget::Statement(id) = &invocation.branches[0].target {
                        self.jump_dests.insert(id.0);
                    }
                    for branch in invocation.branches.iter() {
                        if let GenBranchTarget::Statement(id) = branch.target {
                            self.jump_dests.insert(id.0);
                        }
                    }
                }
                _ => continue,
            }
        }
        Ok(())
    }
}
