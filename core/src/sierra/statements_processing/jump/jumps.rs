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
    pub fn jump(&mut self, dest_nb: usize) {
        let func = self.module.get_last_function().unwrap();
        let destination = match self.basic_blocks.get(&dest_nb) {
            Some(basic_block) => *basic_block,
            None => self.context.append_basic_block(func, "dest"),
        };
        self.builder.build_unconditional_branch(destination);
        self.basic_blocks.insert(dest_nb, destination);
        self.builder.position_at_end(destination);
        self.process_statements_from(dest_nb).unwrap();
    }
}
