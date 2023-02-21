use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of `array_new<T>` operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of `array_new<T>`.
    ///
    /// # Error
    ///
    /// Panics if the type T has not been declared previously as all types should be declared at the
    /// beginning of the sierra file.
    pub fn array_new(&self, libfunc_declaration: &LibfuncDeclaration) {
        let val_name = match &libfunc_declaration.long_id.generic_args[0] {
            // Panics if the type has not been declared.
            GenericArg::Type(ConcreteTypeId { id: _, debug_name }) => debug_name.clone().expect(DEBUG_NAME_EXPECTED),
            // Not sure if dup can array_new user defined types
            GenericArg::UserType(_) => todo!(),
            _ => panic!("Dup only takes type or user type"),
        };
        let ret_type = self.types_by_name.get(&(format!("Array<{:}", val_name) + ">")).unwrap().into_struct_type();

        let arr_size = ret_type.get_field_type_at_index(1).unwrap().into_int_type().const_int(2, false);
        let arr_ptr =
            self.builder.build_array_malloc(ret_type.get_field_type_at_index(0).unwrap(), arr_size, "ret").unwrap();
        let arr = ret_type.const_named_struct(&[arr_ptr.into(), arr_size.into()]);
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            ret_type.fn_type(&[], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        self.builder.build_return(Some(&arr));
    }
}
