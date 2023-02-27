use cairo_lang_sierra::program::LibfuncDeclaration;
use inkwell::debug_info::{AsDIScope, DIFlags, DIFlagsConstants, DISubprogram};
use inkwell::types::BasicType;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
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
    /// Panics if the felt type has not been declared previously or if the module felt function has
    /// not been declared previously.
    pub fn felt_add(&self, libfunc_declaration: &LibfuncDeclaration) {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let felt_type = self.get_type_from_name("felt").expect("Can't get felt from name");
        let felt_type_id = self.get_type_id_from_name("felt").unwrap();

        // fn felt_add(a: felt, b: felt) -> felt
        let func_name = libfunc_declaration.id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).as_str();
        let func = self.module.add_function(
            func_name,
            felt_type.fn_type(&[felt_type.as_basic_type_enum().into(), felt_type.as_basic_type_enum().into()], false),
            None,
        );

        self.create_function_debug(func_name, &func, felt_type_id, &[felt_type_id.clone(), felt_type_id.clone()]);

        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        self.debug_location(None);
        // The maximum value of and addition is 2 * (prime - 1) which is 253 bits.
        let double_felt = self.context.custom_width_int_type(512);
        // Extend left hand side.
        // We just defined the function so it shouldn't panic.
        let lhs = self.builder.build_int_s_extend(
            func.get_first_param().unwrap().into_int_value(),
            double_felt,
            "extended_a",
        );
        // Extend right hand side.
        // We just defined the function so it shouldn't panic.
        let rhs =
            self.builder.build_int_s_extend(func.get_last_param().unwrap().into_int_value(), double_felt, "extended_b");
        // Compute a + b.
        let add = self.builder.build_int_add(lhs, rhs, "res");
        // Extend it to 503 bits for the modulo operation.
        // Panics if the function doesn't have enough arguments but it shouldn't happen since we just
        // defined it above.
        // Also panics if the modulo function doesn't return a value but it shouldn't happen.
        // return a + b % prime
        let res = self
            .builder
            .build_call(
                self.module.get_function("modulo").expect("Modulo should have been defined before"),
                &[add.into()],
                "res",
            )
            .try_as_basic_value()
            .left()
            .expect("Should have a left return value");
        self.builder.build_return(Some(&res));
    }
}
