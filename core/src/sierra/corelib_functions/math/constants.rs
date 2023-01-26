use inkwell::builder::Builder;
use inkwell::types::StringRadix;
use inkwell::values::{BasicValue, FunctionValue};

use crate::sierra::corelib_functions::processor::LlvmBodyProcessor;
use crate::sierra::errors::{CompilerError, CompilerResult};

/// LlvmMathConst represents a constant of a numeric type.
pub struct LlvmMathConst {
    /// The value of the constant.
    pub value: String,
}

/// Implementation of the LlvmBodyProcessor trait for int typed constants.
impl<'ctx> LlvmBodyProcessor<'ctx> for LlvmMathConst {
    fn create_body(
        &self,
        _builder: &Builder<'ctx>,
        fn_value: &FunctionValue<'ctx>,
    ) -> CompilerResult<Option<Box<dyn BasicValue<'ctx> + 'ctx>>> {
        Ok(Some(Box::from(
            fn_value
                .get_type()
                .get_return_type()
                .ok_or(CompilerError::NoReturnType)?
                .into_int_type()
                .const_int_from_string(&self.value, StringRadix::Decimal)
                .expect("Math constant should be a decimal value"),
        )))
    }
}
