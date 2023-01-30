use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::{GenericArg, LibfuncDeclaration};
use inkwell::types::BasicType;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn store_temp(&mut self, libfunc_declaration: &LibfuncDeclaration) -> CompilerResult<()> {
        let func_type = match &libfunc_declaration.long_id.generic_args[0] {
            GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => self
                .types
                .get(&id.to_string())
                .expect("store_temp type should have been declared")
                .as_basic_type_enum(),
            GenericArg::UserType(_) => todo!(),
            _val => {
                panic!("store_temp only takes type or user type")
            }
        };
        let func = self.module.add_function(
            libfunc_declaration
                .id
                .debug_name
                .clone()
                .expect("This compiler only works with sierra compiled with --replace-ids")
                .to_string()
                .as_str(),
            func_type.fn_type(&[func_type.into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        let arg =
            func.get_first_param().expect("store_temp function should have an input parameter");
        self.builder.build_return(Some(&arg));
        Ok(())
    }
}
