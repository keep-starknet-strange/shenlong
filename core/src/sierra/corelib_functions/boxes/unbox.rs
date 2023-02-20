use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicType;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of unbox<T>.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `unbox<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    pub fn unbox(&self, libfunc_declaration: &LibfuncDeclaration) {
        // This function is completely irrelevant for LLVM IR but for simplicity we implement it like rename
        // for now.
        // Get the type that this unbox function has to handle
        let func_type = match &libfunc_declaration.long_id.generic_args[0] {
            // Panics if the type has not been declared.
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                self.types.get(&id.to_string()).unwrap().as_basic_type_enum()
            }
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("unbox only takes type or user type")
            }
        };

        // fn unbox<T>(a: Box<T>) -> T
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            func_type.fn_type(&[func_type.into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // We just defined unbox to have an input parameter so it shouldn't panic.
        let arg = func.get_first_param().unwrap();
        // Return the input value.
        self.builder.build_return(Some(&arg));
    }
}
