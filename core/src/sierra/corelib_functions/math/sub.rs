use inkwell::builder::Builder;
use inkwell::values::{BasicValue, FunctionValue};

use crate::sierra::corelib_functions::processor::LlvmBodyProcessor;
use crate::sierra::errors::CompilerResult;

/// Body processor for the `-` operator of any numeric type.
pub struct LlvmMathSub {}

/// Implementation of the LlvmBodyProcessor trait for the - function for int types.
impl<'ctx> LlvmBodyProcessor<'ctx> for LlvmMathSub {
    fn create_body(
        &self,
        builder: &Builder<'ctx>,
        fn_value: &FunctionValue<'ctx>,
    ) -> CompilerResult<Option<Box<dyn BasicValue<'ctx> + 'ctx>>> {
        Ok(Some(Box::from(builder.build_int_sub(
            fn_value.get_first_param().unwrap().into_int_value(),
            fn_value.get_last_param().unwrap().into_int_value(),
            "res",
        ))))
    }
}
