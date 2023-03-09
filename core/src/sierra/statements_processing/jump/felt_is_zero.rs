use cairo_lang_sierra::program::{GenBranchTarget, Invocation, StatementIdx};
use inkwell::IntPredicate::EQ;

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
        // The felt to check
        let felt_type = self
            .types_by_name
            .get("felt")
            .expect("felt type should have been registered before felt_zero is processed")
            .to_owned();
        // if felt == 0 {} else {}
        self.process_args(invocation, invocation_nb, &[felt_type]);

        // Check the two branches
        let then_bb = self
            .get_block_info_for_statement_id(match invocation.branches[0].target {
                // if then is fallthrough
                GenBranchTarget::Fallthrough => invocation_nb + 1,
                // then branch is a jump so we process from the jump until a return instruction.
                GenBranchTarget::Statement(StatementIdx(id)) => id,
            })
            .block;

        let else_bb = self
            .get_block_info_for_statement_id(match invocation.branches[0].target {
                // else is fallthrough
                GenBranchTarget::Fallthrough => invocation_nb + 1,
                // else branch is a jump
                GenBranchTarget::Statement(StatementIdx(id)) => id,
            })
            .block;

        let lhs = self
            .get_processed_variable_at_statement(&invocation.args[0], invocation_nb)
            .expect("Argument should be available as a variable after processing");
        // felt == 0
        let comparison = self.builder.build_int_compare(
            EQ,
            lhs.into_int_value(),
            felt_type.into_int_type().const_int(0, false),
            "check",
        );
        self.builder.build_conditional_branch(comparison, then_bb, else_bb);
    }
}
