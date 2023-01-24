use inkwell::builder::Builder;
use inkwell::values::{BasicValue, FunctionValue};

use crate::sierra::errors::CompilerResult;
use crate::sierra::libfunc::processor::LlvmBodyProcessor;

pub struct LlvmMathSub {}

/// Implementation of the LlvmBodyProcessor trait for the - function for int types.
impl<'ctx> LlvmBodyProcessor<'ctx> for LlvmMathSub {
    fn create_body(
        &self,
        builder: &Builder<'ctx>,
        fn_value: &FunctionValue<'ctx>,
    ) -> CompilerResult<Box<dyn BasicValue<'ctx> + 'ctx>> {
        Ok(Box::from(builder.build_int_sub(
            fn_value.get_first_param().unwrap().into_int_value(),
            fn_value.get_last_param().unwrap().into_int_value(),
            "res",
        )))
    }
}
