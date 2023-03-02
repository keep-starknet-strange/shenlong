use inkwell::types::BasicType;

use super::DEFAULT_PRIME;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a felt addition.
    ///
    /// # Error
    ///
    /// Returns an error if the felt type has not been declared previously.
    pub fn modulo(&mut self) {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let felt_type = self.types_by_name.get("felt").expect("Can't get felt from name");
        let debug_felt_type = *self.debug.types_by_name.get("felt").expect("Can't get felt from name");
        // Max size of felt operation (Prime - 1 ) * ( Prime - 1) = 503 bits number
        let big_felt_type = self.context.custom_width_int_type(512);
        let debug_double_felt_type =
            *self.debug.types_by_name.get("double_felt").expect("Can't get double felt from name");

        // fn felt_modulo(a: double_felt) -> felt
        let func = self.module.add_function("modulo", felt_type.fn_type(&[big_felt_type.into()], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

        self.debug.create_function("modulo", &func, Some(debug_felt_type), &[debug_double_felt_type], None);

        let prime = big_felt_type.const_int_from_string(DEFAULT_PRIME, inkwell::types::StringRadix::Decimal).unwrap();
        // smod
        let modu = // We just defined the function so it shouldn't panic
            self.builder.build_int_signed_rem(func.get_first_param().unwrap().into_int_value(), prime, "modulus");
        // As we performed smod on the value it is now 0 < |res| < PRIME so it's less than 252 and we can
        // truncate the high bits
        let res = self.builder.build_int_truncate(modu, felt_type.into_int_type(), "res");
        self.builder.build_return(Some(&res));
    }
}
