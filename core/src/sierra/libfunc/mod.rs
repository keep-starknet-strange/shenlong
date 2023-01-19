use eyre::Result;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum};

/// Add is a processor that will generate the LLVM IR for the add function.
/// It can hande any numeric types.
pub struct Add {}

/// LibfuncProcessor is a trait that will be implemented by all libfunc processors.
/// It will be used to generate the LLVM IR for the libfunc.
pub trait LibfuncProcessor<'a, 'ctx> {
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
        fn_name: &'a str,
        output_type: BasicTypeEnum<'ctx>,
        parameter_types: Vec<&Box<dyn BasicType<'ctx> + 'ctx>>,
        module: &Module<'ctx>,
        context: &Context,
        builder: &Builder,
    ) -> Result<()>;
}

impl<'a, 'ctx> LibfuncProcessor<'a, 'ctx> for Add {
    /// See the trait documentation (`LibfuncProcessor`).
    fn to_llvm(
        &self,
        fn_name: &'a str,
        output_type: BasicTypeEnum<'ctx>,
        parameter_types: Vec<&Box<dyn BasicType<'ctx> + 'ctx>>,
        module: &Module<'ctx>,
        context: &Context,
        builder: &Builder,
    ) -> Result<()> {
        // Check the parameters.
        if parameter_types.len() != 2 {
            return Err(eyre::eyre!("Add function must have 2 parameters"));
        }

        // Convert the parameters to BasicTypeEnum and store them in a vector.
        let parameters = vec![
            parameter_types[0].as_basic_type_enum().into(),
            parameter_types[1].as_basic_type_enum().into(),
        ];

        // Create the function,
        let function = module.add_function(fn_name, output_type.fn_type(&parameters, false), None);
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
