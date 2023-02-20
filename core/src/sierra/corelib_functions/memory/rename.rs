use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicType;

use crate::sierra::errors::{CompilerResult, DEBUG_NAME_EXPECTED};
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of the rename function.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `rename<T>`.
    ///
    /// # Error
    ///
    /// Returns an error if the type T has not been declared previously.
    pub fn rename(&self, libfunc_declaration: &LibfuncDeclaration) -> CompilerResult<()> {
        // This function just dumbly returns its input value. When it's called it just stores the value in
        // the next variable. Not sure how relevant it is in LLVM but might be useful later for branching.
        // Get the type that this rename function has to handle
        let func_type = match &libfunc_declaration.long_id.generic_args[0] {
            // Panics if the type has not been declared.
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                self.types.get(&id.to_string()).expect("store_temp type should have been declared").as_basic_type_enum()
            }
            // Not sure if rename can rename user defined types
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("store_temp only takes type or user type")
            }
        };
        // fn rename<T>(a: T) -> T
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            func_type.fn_type(&[func_type.into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // We just defined rename to have an input parameter so it shouldn't panic.
        let arg = func.get_first_param().expect("Rename function should have an input parameter");
        // Return the input value.
        self.builder.build_return(Some(&arg));
        Ok(())
    }
}
