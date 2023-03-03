use cairo_lang_sierra::program::{GenBranchTarget, GenStatement};

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

/// Implementation for the type processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Collect all the jump destinations in the sierra program. Scans all the instructions and the
    /// branches to determine at what statement the sierra program can jump.
    /// The point of doing this is to have a cleaner handling of jumps.
    pub fn collect_jumps(&mut self) {
        for statement in self.program.statements.iter() {
            match statement {
                GenStatement::Invocation(invocation) => {
                    if invocation.libfunc_id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str() == "jump" &&
                    let GenBranchTarget::Statement(id) = &invocation.branches[0].target {
                        // If it's a jump insert the destination of the jump in the map.
                        self.jump_dests.insert(id.0);
                    }
                    for branch in invocation.branches.iter() {
                        if let GenBranchTarget::Statement(id) = branch.target {
                            // If it's a jump insert the destination of the jump in the map.
                            self.jump_dests.insert(id.0);
                        }
                    }
                }
                _ => continue,
            }
        }
    }
}
