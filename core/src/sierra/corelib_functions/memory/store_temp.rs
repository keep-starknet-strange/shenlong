use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicType;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a store_temp operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `store_temp<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    pub fn store_temp(&self, libfunc_declaration: &LibfuncDeclaration) {
        // This function is completely irrelevant for LLVM IR but for simplicity we implement it like rename
        // for now. In cairo this function is used to store something in a tempvar.
        // Get the type that this store_temp function has to handle

        let type_id = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => *id,
            // Not sure if store_temp can store_temp user defined types
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("store_temp only takes type or user type")
            }
        };

        // Panics if the type has not been declared.
        let func_and_arg_type = self.types_by_id.get(&type_id).unwrap().as_basic_type_enum();
        let debug_func_and_arg_type = *self.debug.types_by_id.get(&type_id).unwrap();

        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();

        // fn store_temp<T>(a: T) -> T
        let func =
            self.module.add_function(func_name, func_and_arg_type.fn_type(&[func_and_arg_type.into()], false), None);

        self.debug.create_function_debug(func_name, &func, Some(debug_func_and_arg_type), &[debug_func_and_arg_type]);

        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // We just defined store_temp to have an input parameter so it shouldn't panic.
        let arg = func.get_first_param().unwrap();
        // Return the input value.
        self.builder.build_return(Some(&arg));
    }
}
