use cairo_lang_sierra::program::LibfuncDeclaration;
use inkwell::types::BasicType;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn felt_add(&mut self, libfunc_declaration: &LibfuncDeclaration) -> CompilerResult<()> {
        let return_type = self.get_type_from_name("felt")?;
        let func = self.module.add_function(
            libfunc_declaration
                .id
                .debug_name
                .clone()
                .expect("This compiler only works with sierra compiled with --replace-ids")
                .to_string()
                .as_str(),
            return_type.fn_type(
                &[return_type.as_basic_type_enum().into(), return_type.as_basic_type_enum().into()],
                false,
            ),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        self.builder.build_return(Some(&self.builder.build_int_add(
            func.get_first_param().expect("felt_add should have a first arg").into_int_value(),
            func.get_last_param().expect("felt_add should have a second arg").into_int_value(),
            "res",
        )));
        Ok(())
    }
}
