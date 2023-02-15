use log::debug;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::{CompilationState, Compiler};

/// Implementation of the corelib functions processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process core library functions in the Sierra program.
    ///
    /// # Errors
    ///
    /// if the processing of the core lib functions fails.
    pub fn process_core_lib_functions(&mut self) -> CompilerResult<()> {
        debug!("processing core lib functions");

        // Check that the current state is valid.
        self.check_state(&CompilationState::TypesProcessed)?;
        self.modulo()?;
        // Iterate over the libfunc declarations in the Sierra program.
        for libfunc_declaration in self.program.libfunc_declarations.iter() {
            // Get the debug name of the function.
            if let Some(libfunc) = &libfunc_declaration.long_id.generic_id.debug_name {
                // Each core lib function is known
                match libfunc.to_string().as_str() {
                    "felt_const" => {
                        self.felt_const(libfunc_declaration)?;
                    }
                    "felt_add" => {
                        self.felt_add(libfunc_declaration)?;
                    }
                    "felt_sub" => {
                        self.felt_sub(libfunc_declaration)?;
                    }
                    "dup" => {
                        self.dup(libfunc_declaration)?;
                    }
                    "store_temp" => {
                        self.store_temp(libfunc_declaration)?;
                    }
                    "revoke_ap_tracking" => (),
                    "drop" => (),
                    "struct_construct" => self.struct_construct(libfunc_declaration),
                    "struct_deconstruct" => self.struct_deconstruct(libfunc_declaration),
                    "felt_is_zero" => println!("Treated in the statements"),
                    "rename" => self.rename(libfunc_declaration)?,
                    "function_call" => println!("Treated in the statements"),
                    "jump" => println!("Treated in the statements"),
                    "branch_align" => println!("Ignored for now"),
                    _ => println!("{:} not implemented", libfunc.to_string()),
                }
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::CoreLibFunctionsProcessed)
    }
}
