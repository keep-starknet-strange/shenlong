use cairo_lang_sierra::program::{GenBranchTarget, Invocation, StatementIdx};
use inkwell::IntPredicate::EQ;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn felt_is_zero(&mut self, invocation: &Invocation, invocation_nb: usize) -> CompilerResult<()> {
        let lhs = self.variables.get(&invocation.args[0].id.to_string()).expect(
            "Variable should
        exist",
        );
        let comparison = self.builder.build_int_compare(
            EQ,
            lhs.into_int_value(),
            self.get_type_from_name("felt")?.into_int_type().const_int(0, false),
            "check",
        );
        let func = self.module.get_last_function().unwrap();
        let then_bb = self.context.append_basic_block(func, "then");
        let else_bb = self.context.append_basic_block(func, "else");

        self.builder.build_conditional_branch(comparison, then_bb, else_bb);

        self.builder.position_at_end(then_bb);
        match invocation.branches[0].target {
            GenBranchTarget::Fallthrough => {
                let until = match invocation.branches[1].target {
                    GenBranchTarget::Fallthrough => None,
                    GenBranchTarget::Statement(StatementIdx(id)) => Some(id),
                };
                self.process_statements_from_until(invocation_nb + 1, until)?;
            }
            GenBranchTarget::Statement(StatementIdx(id)) => self.process_statements_from_until(id, None)?,
        };

        self.builder.position_at_end(else_bb);
        match invocation.branches[1].target {
            GenBranchTarget::Fallthrough => {
                let until = match invocation.branches[0].target {
                    GenBranchTarget::Fallthrough => None,
                    GenBranchTarget::Statement(StatementIdx(id)) => Some(id),
                };
                self.process_statements_from_until(invocation_nb + 1, until)?;
            }
            GenBranchTarget::Statement(StatementIdx(id)) => self.process_statements_from_until(id, None)?,
        };

        println!("{:?}", invocation.branches[0]);
        // else case
        println!("{:?}", invocation.branches[1]);
        Ok(())
    }
}
