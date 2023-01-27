use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn struct_deconstruct(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        let args = match &libfunc_declaration.long_id.generic_args[0] {
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
            args.fn_type(&[args.into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        self.builder.build_return(Some(
            &func.get_first_param().expect("This functions should take exactly 1 argument "),
        ));
    }
}
