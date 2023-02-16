use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicMetadataTypeEnum;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a struct construction operation.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of StructConstruct<T>.
    ///
    /// # Error
    ///
    /// Returns an error if the type T has not been declared previously.
    pub fn struct_construct(&self, libfunc_declaration: &LibfuncDeclaration) {
        // Type of the struct that we have to construct.
        let return_type = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => self
                .types
                .get(&id.to_string())
                .expect("Type should have been defined before struct")
                .as_basic_type_enum()
                .into_struct_type(),
            _val => {
                panic!("Struct construct only takes predefined structs")
            }
        };
        // fn StructConstruct<T>(field_1: t1, field2: t2 ...) -> T
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            return_type.fn_type(
                &return_type
                    .get_field_types()
                    .into_iter()
                    .map(|arg_type| arg_type.into())
                    .collect::<Vec<BasicMetadataTypeEnum>>(),
                false,
            ),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        let struct_ptr = self.builder.build_alloca(return_type, "res_ptr");
        // Store each field in the struct.
        for (i, param) in func.get_params().iter().enumerate() {
            let param_type = param.get_type();
            let tuple_ptr = self
                .builder
                .build_struct_gep(param_type, struct_ptr, i as u32, format!("field_{i}_ptr").as_str())
                .expect("Pointer should be valid");
            self.builder.build_store(tuple_ptr, *param);
        }
        self.builder.build_return(Some(&self.builder.build_load(return_type, struct_ptr, "res")));
    }
}
