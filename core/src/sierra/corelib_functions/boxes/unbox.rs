use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicType;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of `unbox<T>`.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `unbox<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    pub fn unbox(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        // This function is completely irrelevant for LLVM IR but for simplicity we implement it like rename
        // for now.
        // Get the type that this unbox function has to handle

        let type_id = match &libfunc_declaration.long_id.generic_args[0] {
            // Panics if the type has not been declared.
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => *id,
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("unbox only takes type or user type")
            }
        };

        // Panics if the type has not been declared.
        let func_and_arg_type = self.types_by_id.get(&type_id).unwrap().as_basic_type_enum();
        let debug_func_and_arg_type = *self.debug.types_by_id.get(&type_id).unwrap();

        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();

        // fn unbox<T>(a: Box<T>) -> T
        let func =
            self.module.add_function(func_name, func_and_arg_type.fn_type(&[func_and_arg_type.into()], false), None);

        self.debug.create_function(func_name, &func, Some(debug_func_and_arg_type), &[debug_func_and_arg_type], None);

        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // We just defined unbox to have an input parameter so it shouldn't panic.
        let arg = func.get_first_param().unwrap();
        // Return the input value.
        self.builder.build_return(Some(&arg));
    }
}
