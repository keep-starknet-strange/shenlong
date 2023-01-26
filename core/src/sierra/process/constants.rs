/// This file contains everything related to sierra constant processing.
use cairo_lang_sierra::program::GenericArg::Value;
use cairo_lang_sierra::program::LibfuncDeclaration;
use num_bigint::BigInt;

use crate::sierra::corelib_functions::math::constants::LlvmMathConst;
use crate::sierra::corelib_functions::processor::Func;
use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::llvm_compiler::Compiler;

/// Implementation of the constant processing for the compiler.
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
        println!("{:?}", libfunc_declaration.long_id.generic_args.clone().iter());
        let converted: Vec<String> = libfunc_declaration
            .long_id
            .generic_args
            .clone()
            .iter()
            .map(|arg| match arg {
                Value(val) => val.to_string(),
                _ => BigInt::from(1).to_string(),
            })
            .collect();
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
            converted[0].clone()
        );

        // Constants don't need any argument.
        let parameter_types = vec![];
        // Only handles the felt constant for now.
        let const_type = self
            .types
            .get(
                self.id_from_name
                    .get("felt")
                    .ok_or(CompilerError::TypeNotFound("felt".to_owned()))?,
            )
            .expect("Felt type should have been declared");
        // Save the constant in the corelib functions HashMap.
        self.libfunc_processors.insert(
            felt_const.clone(),
            Func::new(
                parameter_types,
                const_type.as_basic_type_enum(),
                Box::from(LlvmMathConst { value: converted[0].clone() }),
            ),
        );
        Ok(felt_const)
    }
}
