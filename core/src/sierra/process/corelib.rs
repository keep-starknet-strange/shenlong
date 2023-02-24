use inkwell::module::Linkage;
use inkwell::AddressSpace;
use tracing::debug;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::{CompilationState, Compiler};

pub const PRINT_FELT_FUNC: &str = "print_felt";
pub const PRINT_DOUBLE_FELT_FUNC: &str = "print_double_felt";
pub const PRINT_RETURN: &str = "print_return";

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
        self.modulo();

        // Define external printf, used in call_printf calls.
        // i32 @printf(ptr, ...);
        let i8_type = self.context.i8_type();
        let i32_type = self.context.i32_type();
        let str_type = i8_type.ptr_type(AddressSpace::from(0));
        let printf_type = i32_type.fn_type(&[str_type.into()], true);
        self.module.add_function("printf", printf_type, Some(Linkage::External));

        // Generate print functions for felts and double felts.
        // Useful for debugging in the current early stage of development.
        // Should probably be removed in the future.
        let felt_type = self.get_type_from_name("felt").expect("Can't get felt from name");
        self.printf_for_type(felt_type.into(), PRINT_FELT_FUNC);
        let double_felt = self.context.custom_width_int_type(503);
        self.printf_for_type(double_felt.into(), PRINT_DOUBLE_FELT_FUNC);

        // Iterate over the libfunc declarations in the Sierra program.
        for libfunc_declaration in self.program.libfunc_declarations.iter() {
            // Get the debug name of the function.
            let libfunc_name = libfunc_declaration.long_id.generic_id.0.as_str();
            debug!(libfunc_name, "processing");
            // Each core lib function is known
            match libfunc_name {
                "branch_align" => debug!(libfunc_name, "ignored for now"),
                "drop" => (),
                "dup" => self.dup(libfunc_declaration),
                "felt_add" => self.felt_add(libfunc_declaration),
                "felt_const" => self.felt_const(libfunc_declaration),
                "felt_is_zero" => debug!(libfunc_name, "treated in the statements"),
                "felt_mul" => self.felt_mul(libfunc_declaration),
                "felt_sub" => self.felt_sub(libfunc_declaration),
                "function_call" => debug!(libfunc_name, "treated in the statements"),
                "into_box" => self.into_box(libfunc_declaration),
                "jump" => debug!(libfunc_name, "treated in the statements"),
                "revoke_ap_tracking" => (),
                "rename" => self.rename(libfunc_declaration),
                "store_temp" => self.store_temp(libfunc_declaration),
                "struct_construct" => self.struct_construct(libfunc_declaration),
                "struct_deconstruct" => self.struct_deconstruct(libfunc_declaration),
                "unbox" => self.unbox(libfunc_declaration),
                _ => debug!(libfunc_name, "not implemented"),
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::CoreLibFunctionsProcessed)
    }
}
