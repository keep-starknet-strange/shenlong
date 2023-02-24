use inkwell::types::BasicType;

use super::DEFAULT_PRIME;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a felt addition.
    ///
    /// # Error
    ///
    /// Returns an error if the felt type has not been declared previously.
    pub fn modulo(&self) {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let felt_type = self.get_type_from_name("felt").expect("Can't get felt from name");
        // Max size of felt operation (Prime - 1 ) * ( Prime - 1) = 503 bits number
        let big_felt_type = self.context.custom_width_int_type(512);
        // fn felt_modulo(a: double_felt) -> felt
        let func = self.module.add_function("modulo", felt_type.fn_type(&[big_felt_type.into()], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
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
