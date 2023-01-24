use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, IntValue};

use crate::sierra::errors::CompilerError;
use crate::sierra::libfunc::processor::LlvmBodyProcessor;

pub struct LlvmMathAdd {}

impl<'ctx> LlvmBodyProcessor<'ctx> for LlvmMathAdd {
    fn create_body(
        &self,
        builder: &Builder<'ctx>,
        fn_type: &FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CompilerError> {
        Ok(builder.build_int_add(
            fn_type.get_first_param().unwrap().into_int_value(),
            fn_type.get_last_param().unwrap().into_int_value(),
            "res",
        ))
    }
}
