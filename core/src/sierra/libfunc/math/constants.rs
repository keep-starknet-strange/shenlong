use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, IntValue};

use crate::sierra::errors::CompilerError;
use crate::sierra::libfunc::processor::LlvmBodyProcessor;

pub struct LlvmMathConst {
    pub value: u64,
}

impl<'ctx> LlvmBodyProcessor<'ctx> for LlvmMathConst {
    fn create_body(
        &self,
        _builder: &Builder<'ctx>,
        fn_type: &FunctionValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CompilerError> {
        Ok(fn_type
            .get_type()
            .get_return_type()
            .ok_or(CompilerError::NoReturnType)?
            .into_int_type()
            .const_int(self.value, false))
    }
}
