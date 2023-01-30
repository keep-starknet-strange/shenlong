use cairo_lang_sierra::program::GenericArg::Value;
use cairo_lang_sierra::program::LibfuncDeclaration;
use inkwell::types::{BasicType, StringRadix};

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn felt_const(&mut self, libfunc_declaration: &LibfuncDeclaration) -> CompilerResult<()> {
        let return_type = self.get_type_from_name("felt")?;
        let func = self.module.add_function(
            libfunc_declaration
                .id
                .debug_name
                .clone()
                .expect("This compiler only works with sierra compiled with --replace-ids")
                .to_string()
                .as_str(),
            return_type.fn_type(&[], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        let ret = if let Value(val) = &libfunc_declaration.long_id.generic_args[0] {
            return_type
                .as_basic_type_enum()
                .into_int_type()
                .const_int_from_string(val.to_string().as_str(), StringRadix::Decimal)
                .expect("Couldn't convert to string the felt constant value")
        } else {
            panic!("No value for felt constant")
        };
        self.builder.build_return(Some(&ret));
        Ok(())
    }
}
