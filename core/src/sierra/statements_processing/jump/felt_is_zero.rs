use cairo_lang_sierra::program::{GenBranchTarget, Invocation, StatementIdx};
use inkwell::IntPredicate::EQ;

use crate::sierra::errors::CompilerResult;
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
    pub fn felt_is_zero(&mut self, invocation: &Invocation, invocation_nb: usize) -> CompilerResult<()> {
        // The felt to check.
        let lhs = self.variables.get(&invocation.args[0].id.to_string()).expect("Variable should exist");
        // felt == 0
        let comparison = self.builder.build_int_compare(
            EQ,
            lhs.into_int_value(),
            self.get_type_from_name("felt")?.into_int_type().const_int(0, false),
            "check",
        );
        // Get the current function.
        let func = self.module.get_last_function().unwrap();
        // if then
        let then_bb = self.context.append_basic_block(func, "then");
        // else
        let else_bb = self.context.append_basic_block(func, "else");

        // if felt == 0 {} else {}
        self.builder.build_conditional_branch(comparison, then_bb, else_bb);

        self.builder.position_at_end(then_bb);
        // Check the two branches
        match invocation.branches[0].target {
            // if then is fallthrough
            GenBranchTarget::Fallthrough => {
                self.process_statements_from(invocation_nb + 1)?;
            }
            // then branch is a jump so we process from the jump until a return instruction.
            GenBranchTarget::Statement(StatementIdx(id)) => self.jump(id),
        };

        self.builder.position_at_end(else_bb);
        match invocation.branches[1].target {
            // else is fallthrough
            GenBranchTarget::Fallthrough => {
                self.process_statements_from(invocation_nb + 1)?;
            }
            // else branch is a jump so we process from the jump until a return instruction.
            GenBranchTarget::Statement(StatementIdx(id)) => self.jump(id),
        };
        Ok(())
    }
}
