use cairo_lang_sierra::program::Invocation;
use inkwell::IntPredicate::EQ;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn felt_is_zero(&mut self, invocation: &Invocation) -> CompilerResult<()> {
        println!("{:?}", invocation);
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
        // self.builder.build_conditional_branch(comparison, then_block, else_block);
        // then case
        let func = self.module.add_function("then_block", self.context.void_type().fn_type(&[], false), None);
        let then_bb = self.context.append_basic_block(func, "then");
        let else_bb = self.context.append_basic_block(func, "else");
        let finally_bb = self.context.append_basic_block(func, "finally");

        self.builder.build_conditional_branch(comparison, then_bb, else_bb);
        self.builder.position_at_end(then_bb);
        self.builder.build_unconditional_branch(finally_bb);
        self.builder.position_at_end(else_bb);
        self.builder.build_unconditional_branch(finally_bb);

        println!("{:?}", invocation.branches[0]);
        // else case
        println!("{:?}", invocation.branches[1]);
        Ok(())
    }
}
