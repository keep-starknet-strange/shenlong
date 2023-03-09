use cairo_lang_sierra::program::LibfuncDeclaration;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // TODO this may no longer be necessary now that functions are processed upfront
    /// Implementation of the LLVM IR libfunc function_call.
    ///
    /// This only creates the llvm function, it doesn't implement it, thats done during the
    /// statements processing.
    ///
    /// # Error
    ///
    /// Returns an error if the felt type has not been declared previously.
    pub fn function_call(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        let func_def = match &libfunc_declaration.long_id.generic_args[0] {
            cairo_lang_sierra::program::GenericArg::UserFunc(x) => x,
            _ => panic!("should be a user function"),
        };

        let func_declaration =
            self.program.funcs.iter().find(|x| x.id.id == func_def.id).expect("function should exist in sierra funcs");

        self.generate_function_definition(func_declaration);
        // self.functions.insert(func_declaration.id.id, func);
    }
}
