use cairo_lang_sierra::program::LibfuncDeclaration;
use inkwell::types::BasicType;
use inkwell::values::BasicValue;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a felt substraction.
    ///
    /// # Arguments
    ///
    /// * `libfunc_declaration` - The corelib function declaration of felt_sub.
    ///
    /// # Error
    ///
    /// Panics if the felt type has not been declared previously or if the module felt function has
    /// not been declared previously.
    pub fn felt_sub(&mut self, libfunc_declaration: &LibfuncDeclaration) {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let felt_type = self.types_by_name.get("felt").expect("Can't get felt from name");
        let debug_felt_type = *self.debug.types_by_name.get("felt").expect("Can't get felt from name");
        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();
        // fn felt_sub(a: felt, b: felt) -> felt
        let func = self.module.add_function(
            func_name,
            felt_type.fn_type(&[felt_type.as_basic_type_enum().into(), felt_type.as_basic_type_enum().into()], false),
            None,
        );
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        let debug_func = self.debug.create_function(
            func_name,
            &func,
            Some(debug_felt_type),
            &[debug_felt_type, debug_felt_type],
            None,
        );

        // Return a - b
        // Panics if the function doesn't have enough arguments but it should happen since we just defined
        // it above.
        let sub = self.builder.build_int_sub(
            func.get_first_param().unwrap().into_int_value(),
            func.get_last_param().unwrap().into_int_value(),
            "res",
        );

        let arg = self.builder.build_int_s_extend(sub, self.context.custom_width_int_type(512), "arg");
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

        // Debug parameter values
        self.debug.insert_dbg_value(
            func.get_last_param().unwrap(),
            debug_func.params_local_vars[1],
            self.builder.get_current_debug_location().unwrap(),
            sub.as_instruction_value().unwrap(),
        );

        self.debug.insert_dbg_value(
            func.get_first_param().unwrap(),
            debug_func.params_local_vars[0],
            self.builder.get_current_debug_location().unwrap(),
            sub.as_instruction_value().unwrap(),
        );
    }
}
