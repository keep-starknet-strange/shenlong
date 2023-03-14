use cairo_lang_sierra::program::{GenBranchTarget, Invocation, StatementIdx};

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
    pub fn felt_is_zero(&mut self, invocation: &Invocation, invocation_nb: usize) {
        // The llvm value for the value we're checking against 0
        let arg_value = self.dataflow_graph.use_variable_at_statement(invocation_nb, &invocation.args[0]);

        // First calculate the statement indices of each branch
        // Branch 0 is if arg_value == 0
        // Branch 1 is if arg_value != 0
        let target_ids = invocation
            .branches
            .iter()
            .map(|branch| match branch.target {
                GenBranchTarget::Fallthrough => invocation_nb + 1,
                GenBranchTarget::Statement(StatementIdx(id)) => id,
            })
            .collect::<Vec<_>>();

        // The non-zero value is made available for branch 1
        // Branch 0 does not have data forwarded as the argument's value is known at compile time to be zero
        // so it's not necessary
        self.dataflow_graph.claim_variable_for_branch(
            invocation_nb,
            target_ids[1],
            &invocation.branches[1].results[0],
            arg_value,
        );

        // Now find the llvm basic blocks to jump to in each case
        let target_blocks = target_ids
            .into_iter()
            .map(|target| {
                self.dataflow_graph
                    .get_block_for_entrypoint(target)
                    .expect("Block should be defined for felt_is_zero target")
            })
            .collect::<Vec<_>>();

        // Finally we write the comparison operation and the jump
        let comparison = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            arg_value.into_int_value(),
            arg_value.get_type().into_int_type().const_zero(),
            "check",
        );
        self.builder.build_conditional_branch(comparison, target_blocks[0], target_blocks[1]);
    }
}
