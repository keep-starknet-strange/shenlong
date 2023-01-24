use inkwell::builder::Builder;
use inkwell::values::{BasicValue, FunctionValue};

use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::libfunc::processor::LlvmBodyProcessor;

pub struct LlvmMathConst {
    pub value: u64,
}

/// Implementation of the LlvmBodyProcessor trait for int typed constants.
impl<'ctx> LlvmBodyProcessor<'ctx> for LlvmMathConst {
    fn create_body(
        &self,
        _builder: &Builder<'ctx>,
        fn_value: &FunctionValue<'ctx>,
    ) -> CompilerResult<Box<dyn BasicValue<'ctx> + 'ctx>> {
        Ok(Box::from(
            fn_value
                .get_type()
                .get_return_type()
                .ok_or(CompilerError::NoReturnType)?
                .into_int_type()
                .const_int(self.value, false),
        ))
    }
}
