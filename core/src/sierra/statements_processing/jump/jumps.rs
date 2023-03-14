use cairo_lang_sierra::program::{GenBranchTarget, StatementIdx};

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of felt == 0.
    ///
    /// # Arguments
    ///
    /// * `invocation` - The invocation object.
    /// * `invocation_nb` - The invocation number.
    ///
    /// # Error
    ///
    /// Returns an error if the processing of the branches statements fails.
    pub fn insert_flow_control_if_necessary(&mut self, from: usize, target: &GenBranchTarget<StatementIdx>) {
        let jump_target_block =
            match target {
                GenBranchTarget::Fallthrough => self.dataflow_graph.get_block_for_entrypoint(from + 1),
                GenBranchTarget::Statement(id) => Some(self.dataflow_graph.get_block_for_entrypoint(id.0).expect(
                    "Basic block for jump target should have been created before processing of jump statement",
                )),
            };

        if let Some(block) = jump_target_block {
            self.builder.build_unconditional_branch(block);
        }
    }
}
