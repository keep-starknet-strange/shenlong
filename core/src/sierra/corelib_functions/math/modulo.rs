use inkwell::types::BasicType;

use super::DEFAULT_PRIME;
use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of a felt addition.
    ///
    /// # Error
    ///
    /// Returns an error if the felt type has not been declared previously.
    pub fn modulo(&mut self) -> CompilerResult<()> {
        // We could hardcode the LLVM IR type for felt but this adds a check.
        let felt_type = self.get_type_from_name("felt")?;
        // Max size of felt operation (Prime - 1 ) * ( Prime - 1) = 503 bits number
        let big_felt_type = self.context.custom_width_int_type(503);
        // fn felt_add(a: double_felt) -> felt
        let func = self.module.add_function("modulo", felt_type.fn_type(&[big_felt_type.into()], false), None);
        self.builder.position_at_end(self.context.append_basic_block(func, "entry"));
        // Allocate a double_felt pointer to be able to load and use the value multiple times in the while
        // loop.
        let var_ptr = self.builder.build_alloca(big_felt_type, "val_ptr");
        // Store the value to perform the modulo operation on in the previously allocated pointer.
        self.builder.build_store(var_ptr, func.get_first_param().expect("Function should have exactly one parameter"));
        // As urem seem to work for i128 at most we'll implement modulo with a while loop.
        // This block of code would be equivalent to this in python:
        // while a < PRIME_NUMBER:
        //     a -= PRIME_NUMBER
        // else:
        //      return a
        // Start basic block for the while loop (condition checking).
        let start_bb = self.context.append_basic_block(func, "start");
        // Body of the while loop.
        let body_bb = self.context.append_basic_block(func, "body");
        // When condition is not true anymore.
        let end_bb = self.context.append_basic_block(func, "end");
        // We jump to the "start" basic block at the end of the entry basic block to start the while loop.
        self.builder.build_unconditional_branch(start_bb);
        self.builder.position_at_end(start_bb);
        // Load the value to apply the modulo on.
        let value = self.builder.build_load(var_ptr, "val").into_int_value();
        // Create the StarkNet prime number as an i503 const.
        let default_prime = big_felt_type
            .const_int_from_string(DEFAULT_PRIME, inkwell::types::StringRadix::Decimal)
            .expect("Should have been able to parse the default prime");
        // prime < a
        let compare = self.builder.build_int_compare(inkwell::IntPredicate::ULT, default_prime, value, "compare");
        // If true jump to the body basic block else jump to the end basic block.
        self.builder.build_conditional_branch(compare, body_bb, end_bb);
        self.builder.position_at_end(body_bb);
        // We're now in the body of the while loop.
        // Load the value to apply the modulo on.
        let value = self.builder.build_load(var_ptr, "value");
        // temp = a - prime
        let sub = self.builder.build_int_sub(value.into_int_value(), default_prime, "sub");
        // Store the result where the pointer points to.
        self.builder.build_store(var_ptr, sub);
        // Go back to the beginning of the while.
        self.builder.build_unconditional_branch(start_bb);
        self.builder.position_at_end(end_bb);
        // We're in the end basic block (if the check is false and 0 < a < prime should be true)
        let val = self.builder.build_load(var_ptr, "val");
        // As 0 < a < prime should be true the number should be at most 252 bits long so we can safely
        // truncate it back to i252.
        let res = self.builder.build_int_truncate(val.into_int_value(), felt_type.into_int_type(), "res");
        // Return the result.
        self.builder.build_return(Some(&res));
        Ok(())
    }
}
