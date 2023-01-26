use inkwell::builder::Builder;
use inkwell::values::{BasicValue, FunctionValue};

use crate::sierra::corelib_functions::processor::LlvmBodyProcessor;
use crate::sierra::errors::CompilerResult;

pub struct LlvmDup {}

/// Implementation of the LlvmBodyProcessor trait for the dup function for any type.
impl<'ctx> LlvmBodyProcessor<'ctx> for LlvmDup {
    fn create_body(
        &self,
        builder: &Builder<'ctx>,
        fn_value: &FunctionValue<'ctx>,
    ) -> CompilerResult<Option<Box<(dyn BasicValue<'ctx> + 'ctx)>>> {
        let arg = fn_value.get_first_param().expect("Drop function should have an input parameter");
        let return_type =
            fn_value.get_type().get_return_type().expect("Function should have a return type");
        let tuple = builder.build_alloca(return_type, "res_ptr");
        let tuple_ptr =
            builder.build_struct_gep(tuple, 0, "tuple_ptr").expect("Pointer should be valid");
        builder.build_store(tuple_ptr, arg);
        let tuple_ptr_2 =
            builder.build_struct_gep(tuple, 1, "tuple_ptr_2").expect("Pointer2 should be valid");
        builder.build_store(tuple_ptr_2, arg);
        Ok(Some(Box::from(builder.build_load(tuple, "res"))))
    }
}
