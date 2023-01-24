use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, FunctionValue};

use crate::sierra::errors::{CompilerError, CompilerResult};

/// LibfuncProcessor is a trait that will be implemented by all libfunc processors.
/// It will be used to generate the LLVM IR for the libfunc.
pub trait LibfuncProcessor<'ctx> {
    /// Generate the LLVM IR for the libfunc.
    /// The function will be added to the module.
    /// The function will be named `fn_name`.
    /// # Arguments
    /// * `fn_name` - The name of the function.
    /// * `llvm_type` - The type handled by the function, as input parameter or/and return type.
    /// * `module` - The module where the function will be added.
    /// * `context` - The context used to create the function.
    /// * `builder` - The builder used to create the function.
    fn to_llvm(
        &self,
        module: &Module<'ctx>,
        context: &Context,
        builder: &Builder<'ctx>,
        func_name: &str,
    ) -> Result<(), CompilerError>;
}

impl<'ctx> LibfuncProcessor<'ctx> for Func<'ctx> {
    /// See the trait documentation (`LibfuncProcessor`).
    fn to_llvm(
        &self,
        module: &Module<'ctx>,
        context: &Context,
        builder: &Builder<'ctx>,
        func_name: &str,
    ) -> Result<(), CompilerError> {
        // Create the function,
        let function = module.add_function(
            func_name,
            self.output_type.fn_type(&self.parameter_types[..], false),
            None,
        );
        builder.position_at_end(context.append_basic_block(function, "entry"));

        // Return the result
        builder.build_return(Some(
            &self.body_creator_type.create_body(builder, &function).unwrap().as_basic_value_enum(),
        ));
        Ok(())
    }
}

/// LlvmBodyProcessor is a trait that will be implemented by all llvm types.
/// It will be used to generate the LLVM IR body of a function.
pub trait LlvmBodyProcessor<'ctx> {
    /// Generate the LLVM IR body for a core lib function.
    /// The body will be added in the function.
    ///
    /// # Arguments
    ///
    /// * `builder` - The builder used to create the function.
    /// * `fn_value` - The rust object representing the LLVM IR function.
    ///
    /// # Returns
    fn create_body(
        &self,
        builder: &Builder<'ctx>,
        fn_value: &FunctionValue<'ctx>,
    ) -> CompilerResult<Box<dyn BasicValue<'ctx> + 'ctx>>;
}

/// Add is a processor that will generate the LLVM IR for the add function.
/// It can handle any numeric types.
pub struct Func<'ctx> {
    pub parameter_types: Vec<BasicMetadataTypeEnum<'ctx>>,
    pub output_type: BasicTypeEnum<'ctx>,
    pub body_creator_type: Box<dyn LlvmBodyProcessor<'ctx> + 'ctx>,
}
impl<'ctx> Func<'ctx> {
    pub fn new(
        parameter_types: Vec<BasicMetadataTypeEnum<'ctx>>,
        output_type: BasicTypeEnum<'ctx>,
        body_creator_type: Box<dyn LlvmBodyProcessor<'ctx> + 'ctx>,
    ) -> Self {
        Self { parameter_types, output_type, body_creator_type }
    }
}
