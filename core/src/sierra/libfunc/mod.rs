use eyre::Result;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum};

pub struct Add {}
pub trait LibfuncProcessor<'ctx> {
    fn to_llvm(
        &self,
        fn_name: &'ctx str,
        llvm_type: &BasicTypeEnum<'ctx>,
        module: &Module<'ctx>,
        context: &Context,
        builder: &Builder,
    ) -> Result<()>;
}
impl<'ctx> LibfuncProcessor<'ctx> for Add {
    fn to_llvm(
        &self,
        fn_name: &'ctx str,
        llvm_type: &BasicTypeEnum<'ctx>,
        module: &Module<'ctx>,
        context: &Context,
        builder: &Builder,
    ) -> Result<()> {
        let function = module.add_function(
            fn_name,
            llvm_type.fn_type(
                &[llvm_type.as_basic_type_enum().into(), llvm_type.as_basic_type_enum().into()],
                false,
            ),
            None,
        );
        builder.position_at_end(context.append_basic_block(function, "entry"));
        let sum = builder.build_int_add(
            function.get_first_param().unwrap().into_int_value(),
            function.get_last_param().unwrap().into_int_value(),
            "sum",
        );

        // Return the result
        builder.build_return(Some(&sum));
        Ok(())
    }
}
