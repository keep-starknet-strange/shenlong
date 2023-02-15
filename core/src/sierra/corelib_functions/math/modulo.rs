use inkwell::types::BasicType;

use super::DEFAULT_PRIME;
use crate::sierra::errors::CompilerResult;
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
    pub fn modulo(&mut self) -> CompilerResult<()> {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let felt_type = self.get_type_from_name("felt")?;
        let big_felt_type = self.context.custom_width_int_type(503);
        // fn felt_add(a: felt, b: felt) -> felt
        let func = self.module.add_function("modulo", felt_type.fn_type(&[big_felt_type.into()], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        let var_ptr = self.builder.build_alloca(big_felt_type, "val_ptr");
        self.builder.build_store(var_ptr, func.get_first_param().expect("Function should have exactly one parameter"));
        let start_bb = self.context.append_basic_block(func, "start");
        let body_bb = self.context.append_basic_block(func, "body");
        let end_bb = self.context.append_basic_block(func, "end");
        self.builder.build_unconditional_branch(start_bb);
        self.builder.position_at_end(start_bb);
        let value = self.builder.build_load(var_ptr, "val").into_int_value();
        let default_prime = big_felt_type
            .const_int_from_string(DEFAULT_PRIME, inkwell::types::StringRadix::Decimal)
            .expect("Should have been able to parse the default prime");
        let compare = self.builder.build_int_compare(inkwell::IntPredicate::ULT, default_prime, value, "compare");
        self.builder.build_conditional_branch(compare, body_bb, end_bb);

        self.builder.position_at_end(body_bb);
        let value = self.builder.build_load(var_ptr, "value");
        let sub = self.builder.build_int_sub(value.into_int_value(), default_prime, "sub");
        self.builder.build_store(var_ptr, sub);
        self.builder.build_unconditional_branch(start_bb);
        self.builder.position_at_end(end_bb);
        let val = self.builder.build_load(var_ptr, "val");
        let res = self.builder.build_int_truncate(val.into_int_value(), felt_type.into_int_type(), "res");
        self.builder.build_return(Some(&res));
        Ok(())
    }
}
