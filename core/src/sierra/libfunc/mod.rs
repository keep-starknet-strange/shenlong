use eyre::Result;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicType;

pub struct Add<'ctx> {
    pub fn_name: &'ctx str,
    pub llvm_type: Box<dyn BasicType<'ctx> + 'ctx>,
}
impl<'ctx> Add<'ctx> {
    pub fn new(llvm_type: Box<dyn BasicType<'ctx>>, fn_name: &'ctx str) -> Self {
        Self { llvm_type, fn_name }
    }
}

pub trait LibfuncProcessor<'ctx> {
    fn to_llvm(&self, module: &Module<'ctx>, context: &Context, builder: &Builder) -> Result<()>;
}
impl<'ctx> LibfuncProcessor<'ctx> for Add<'ctx> {
    fn to_llvm(
        &self,
        module: &Module<'ctx>,
        context: &inkwell::context::Context,
        builder: &Builder,
    ) -> Result<()> {
        let function = module.add_function(
            self.fn_name,
            self.llvm_type.fn_type(
                &[
                    self.llvm_type.as_basic_type_enum().into(),
                    self.llvm_type.as_basic_type_enum().into(),
                ],
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
