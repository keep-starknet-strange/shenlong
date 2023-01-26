use cairo_lang_sierra::ids::ConcreteTypeId;
use cairo_lang_sierra::program::GenericArg;
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use log::debug;

use crate::sierra::corelib_functions::math::add::LlvmMathAdd;
use crate::sierra::corelib_functions::math::sub::LlvmMathSub;
use crate::sierra::corelib_functions::memory::dup::LlvmDup;
use crate::sierra::corelib_functions::processor::{Func, LibfuncProcessor};
use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::llvm_compiler::{CompilationState, Compiler};

/// Implementation of the corelib functions processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Prepare the libfunc core processors (those are functions from the core lib).
    ///
    /// # Errors
    ///
    /// If the preparation of the processing of the core lib functions fails.
    pub fn prepare_libfunc_processors(&mut self) -> CompilerResult<()> {
        let felt_type = self
            .types
            .get(
                self.id_from_name
                    .get("felt")
                    .ok_or(CompilerError::TypeNotFound("felt".to_owned()))?,
            )
            .expect("Felt type should have been declared");
        // Add two felts and return the result.
        let felt_add = "felt_add".to_owned();
        // Convert the dync BasicType into the appropriate argument type.
        let felt_param: BasicMetadataTypeEnum = felt_type.as_basic_type_enum().into();
        // Input type of the function.
        let parameter_types = vec![felt_param, felt_param];
        // Add the felt + function to the corelib processor HashMap.
        self.libfunc_processors.insert(
            felt_add,
            Func::new(parameter_types, felt_type.as_basic_type_enum(), Box::from(LlvmMathAdd {})),
        );
        let felt_sub = "felt_sub".to_owned();
        // Input type of the function.
        let parameter_types = vec![felt_param, felt_param];
        // Add the felt - function to the corelib processor HashMap.
        self.libfunc_processors.insert(
            felt_sub,
            Func::new(parameter_types, felt_type.as_basic_type_enum(), Box::from(LlvmMathSub {})),
        );

        let felt_dup = "felt_dup".to_owned();
        let parameter_types = vec![felt_param];
        let output_type = felt_type.array_type(2);
        // Add the felt - function to the corelib processor HashMap.
        self.libfunc_processors.insert(
            felt_dup,
            Func::new(parameter_types, output_type.as_basic_type_enum(), Box::from(LlvmDup {})),
        );

        Ok(())
    }

    /// Process core library functions in the Sierra program.
    ///
    /// # Errors
    ///
    /// if the processing of the core lib functions fails.
    pub fn process_core_lib_functions(&mut self) -> CompilerResult<()> {
        debug!("processing core lib functions");
        // Check that the current state is valid.
        self.check_state(&CompilationState::TypesProcessed)?;
        // Iterate over the libfunc declarations in the Sierra program.
        for libfunc_declaration in self.program.libfunc_declarations.iter() {
            // Get the debug name of the function.
            if let Some(libfunc) = &libfunc_declaration.long_id.generic_id.debug_name {
                let mut func_name = libfunc.to_string();
                // If it's a constant process it.
                if libfunc.ends_with("const") {
                    func_name = self.process_const(libfunc_declaration)?;
                } else if libfunc == "dup" {
                    match &libfunc_declaration.long_id.generic_args[0] {
                        GenericArg::Type(ConcreteTypeId { id, debug_name: _ }) => {
                            let arg_type = self
                                .types
                                .get(id)
                                .expect("Type should have been declared")
                                .as_basic_type_enum();
                            func_name = format!("dup_{id}");
                            self.libfunc_processors.insert(
                                func_name.clone(),
                                Func::new(
                                    vec![arg_type.into()],
                                    self.context
                                        .struct_type(&[arg_type, arg_type], false)
                                        .as_basic_type_enum(),
                                    Box::from(LlvmDup {}),
                                ),
                            );
                        }
                        GenericArg::UserType(_) => todo!(),
                        GenericArg::Value(_) => todo!(),
                        GenericArg::UserFunc(_) => todo!(),
                        GenericArg::Libfunc(_) => todo!(),
                    };
                }
                // If the processor is found process the function.
                if let Some(processor) = self.libfunc_processors.get(&func_name) {
                    processor.to_llvm(
                        &self.module,
                        self.context,
                        self.builder,
                        libfunc_declaration.id.id.to_string().as_str(),
                    )?;
                }
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::CoreLibFunctionsProcessed)
    }
}
