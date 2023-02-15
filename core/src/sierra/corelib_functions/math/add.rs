use cairo_lang_sierra::program::LibfuncDeclaration;
use inkwell::types::BasicType;

use crate::sierra::errors::{CompilerResult, DEBUG_NAME_EXPECTED};
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a felt addition.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of felt_add.
    ///
    /// # Error
    ///
    /// Returns an error if the felt type has not been declared previously.
    pub fn felt_add(&mut self, libfunc_declaration: &LibfuncDeclaration) -> CompilerResult<()> {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let felt_type = self.get_type_from_name("felt")?;
        // fn felt_add(a: felt, b: felt) -> felt
        let func = self.module.add_function(
            libfunc_declaration.id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string().as_str(),
            felt_type.fn_type(&[felt_type.as_basic_type_enum().into(), felt_type.as_basic_type_enum().into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // Return a + b
        // Panics if the function doesn't have enough arguments but it should happen since we just defined
        // it above.
        let add = self.builder.build_int_add(
            func.get_first_param().expect("felt_add should have a first arg").into_int_value(),
            func.get_last_param().expect("felt_add should have a second arg").into_int_value(),
            "res",
        );
        let arg = self.builder.build_int_z_extend(add, self.context.custom_width_int_type(503), "arg");
        let res = self
            .builder
            .build_call(
                self.module.get_function("modulo").expect("Modulo should have been defined before"),
                &[arg.into()],
                "res",
            )
            .try_as_basic_value()
            .left()
            .expect("Should have a left return value");
        self.builder.build_return(Some(&res));
        Ok(())
    }
}
