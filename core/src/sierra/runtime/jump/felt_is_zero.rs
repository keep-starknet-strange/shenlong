use cairo_lang_sierra::program::Invocation;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn felt_is_zero(&mut self, invocation: &Invocation) -> CompilerResult<()> {
        println!("{:?}", invocation);
        // self.builder.build_int_compare(IntPredicate::EQ, lhs, , name)
        // self.builder.build_conditional_branch(comparison, then_block, else_block);
        Ok(())
    }
}
