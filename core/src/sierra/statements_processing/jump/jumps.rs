use inkwell::debug_info::DIScope;
use inkwell::values::FunctionValue;

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
    pub fn jump(&mut self, func: FunctionValue<'ctx>, dest_nb: usize, scope: DIScope<'ctx>) {
        // Check if we defined the basic block we want to jump to. Can be useful if several jumps lead to
        // the same block or if we jump backward.
        let destination = match self.basic_blocks.get(&dest_nb) {
            Some(basic_block) => *basic_block,
            None => self.context.append_basic_block(func, "dest"),
        };
        // Jump to the destination block.
        self.builder.build_unconditional_branch(destination);
        // Save it (if it's already saved it'll rewrite else it'll save it).
        self.basic_blocks.insert(dest_nb, destination);
        self.builder.position_at_end(destination);
        // Keep processing the statements.
        self.process_statements_from(func, dest_nb, scope).unwrap();
    }
}
