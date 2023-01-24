use cairo_lang_sierra::program::GenericArg::Value;
use cairo_lang_sierra::program::LibfuncDeclaration;
use num_bigint::BigInt;

use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::libfunc::{Func, LlvmMathConst};
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process the constants of this sierra program.
    pub fn process_const(
        &mut self,
        libfunc_declaration: &LibfuncDeclaration,
    ) -> CompilerResult<String> {
        let converted = libfunc_declaration
            .long_id
            .generic_args
            .clone()
            .into_iter()
            .map(|arg| {
                let val = match arg {
                    Value(val) => val.iter_u64_digits().collect::<Vec<u64>>()[0],
                    _ => BigInt::from(1).iter_u64_digits().collect::<Vec<u64>>()[0],
                };
                val
            })
            .collect::<Vec<u64>>();
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

        let parameter_types = vec![];
        let const_type =
            self.types.get("felt").ok_or(CompilerError::TypeNotFound("felt".to_owned()))?;
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
