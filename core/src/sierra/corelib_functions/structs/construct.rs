use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicMetadataTypeEnum;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn struct_construct(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        let return_type = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => self
                .types
                .get(id)
                .expect("Type should have been defined before struct")
                .as_basic_type_enum()
                .into_struct_type(),
            _val => {
                panic!("Struct construct only takes predefined structs")
            }
        };
        let func = self.module.add_function(
            libfunc_declaration.id.id.to_string().as_str(),
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
        for i in 0..return_type.count_fields() {
            let value = func
                .get_nth_param(i)
                .expect("Function should have as many arguments as struct field");
            let tuple_ptr = self
                .builder
                .build_struct_gep(struct_ptr, i, format!("field_{i}_ptr").as_str())
                .expect("Pointer should be valid");
            self.builder.build_store(tuple_ptr, value);
        }
        self.builder.build_return(Some(&self.builder.build_load(struct_ptr, "res")));
    }
}
