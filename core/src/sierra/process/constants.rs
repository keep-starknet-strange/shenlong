use cairo_lang_sierra::program::GenericArg::Value;
use cairo_lang_sierra::program::LibfuncDeclaration;
use num_bigint::BigInt;

use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::libfunc::math::constants::LlvmMathConst;
use crate::sierra::libfunc::processor::Func;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process the constants of this sierra program.
    ///
    /// # Arguments
    ///
    /// *`libfunc_declaration` - Declaration of the corelib function in the sierra file.
    ///
    /// # Returns
    ///
    /// The constant mangled name.
    ///
    /// # Errors
    ///
    /// If the processing of the consant fails.
    pub fn process_const(
        &mut self,
        libfunc_declaration: &LibfuncDeclaration,
    ) -> CompilerResult<String> {
        // Convert the generic arguments into an array of int. It assumes that we're only working
        // with numeric args.
        let converted = libfunc_declaration
            .long_id
            .generic_args
            .clone()
            .iter()
            .map(|arg| {
                let val = match arg {
                    Value(val) => val.iter_u64_digits().collect::<Vec<u64>>()[0],
                    _ => BigInt::from(1).iter_u64_digits().collect::<Vec<u64>>()[0],
                };
                val
            })
            .collect::<Vec<u64>>();
        // Mangle the name of the constant.
        let felt_const = format!(
            "{}_{}",
            libfunc_declaration
                .long_id
                .generic_id
                .debug_name
                .clone()
                .ok_or(CompilerError::NoDebugName)?
                .to_string(),
            converted[0]
        );

        // Constants don't need any argument.
        let parameter_types = vec![];
        // Only handles the felt constant for now.
        let const_type =
            self.types.get("felt").ok_or(CompilerError::TypeNotFound("felt".to_owned()))?;
        // Save the constant in the corelib functions HashMap.
        self.libfunc_processors.insert(
            felt_const.clone(),
            Func::new(
                parameter_types,
                const_type.as_basic_type_enum(),
                Box::from(LlvmMathConst { value: converted[0] }),
            ),
        );
        Ok(felt_const)
    }
}
