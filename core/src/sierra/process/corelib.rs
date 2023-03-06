use inkwell::module::Linkage;
use inkwell::types::BasicType;
use inkwell::AddressSpace;
use tracing::debug;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::{CompilationState, Compiler};
use crate::sierra::types::felt::DOUBLE_FELT_INT_WIDTH;

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
        let felt_type = self.types_by_name.get("felt").expect("Can't get felt from name");
        self.printf_for_type(felt_type.as_basic_type_enum().into(), PRINT_FELT_FUNC, "felt");
        let double_felt = self.context.custom_width_int_type(DOUBLE_FELT_INT_WIDTH);
        self.printf_for_type(double_felt.into(), PRINT_DOUBLE_FELT_FUNC, "double_felt");
        self.modulo();

        // Iterate over the libfunc declarations in the Sierra program.
        for libfunc_declaration in self.program.libfunc_declarations.iter() {
            self.debug.next_line();

            // Get the debug name of the function.
            let libfunc_name = libfunc_declaration.long_id.generic_id.0.as_str();
            debug!(libfunc_name, "processing");
            // Each core lib function is known
            match libfunc_name {
                "array_new" => self.array_new(libfunc_declaration),
                "array_append" => self.array_append(libfunc_declaration),
                // Align branches after a match/if else or anything that creates branches.
                "branch_align" => debug!(libfunc_name, "ignored for now"),
                // Drops a variables (in sierra everything has to be used exactly once so if a variable is created and
                // then never used for anything we have to drop it to consume it).
                "drop" => (),
                // As everything has to be used exactly once, if we need to use twice the same value we need to
                // duplicate it.
                "dup" => self.dup(libfunc_declaration),
                // Addition for felt type. `felt + felt`
                "felt_add" => self.felt_add(libfunc_declaration),
                // Define a constant of type felt. `const one = 1;`
                "felt_const" => self.int_const(libfunc_declaration, "felt"),
                // Check if a felt is zero. `felt == 0`
                "felt_is_zero" => debug!(libfunc_name, "treated in the statements"),
                // Multiplication for felt type. `felt * felt`
                "felt_mul" => self.felt_mul(libfunc_declaration),
                // Subtraction for the felt type. `felt - felt`
                "felt_sub" => self.felt_sub(libfunc_declaration),
                // Division for the felt type. `felt / NonZero<felt>`
                "felt_div" => self.felt_div(libfunc_declaration),
                // Calls a user defined sierra function. `function_call<user@fib_caller::fib_caller::fib>`
                "function_call" => self.function_call(libfunc_declaration),
                // Boxes a value of type T.
                "into_box" => self.into_box(libfunc_declaration),
                // Jumps to the specified sierra statement number.
                "jump" => debug!(libfunc_name, "treated in the statements"),
                // Revoke ap tracking.
                "revoke_ap_tracking" => (),
                // Renames a variable. (After branching mostly).
                "rename" => self.rename(libfunc_declaration),
                // Stores a value in a tempvar.
                "store_temp" => self.store_temp(libfunc_declaration),
                // Constructs a struct of predefined type.
                "struct_construct" => self.struct_construct(libfunc_declaration),
                // Deconstruct a struct. (Get each struct field in its own variable).
                "struct_deconstruct" => self.struct_deconstruct(libfunc_declaration),
                "u32_const" => self.int_const(libfunc_declaration, "u32"),
                // Converts a `Box<T>` to `T`.
                "unbox" => self.unbox(libfunc_declaration),
                _ => debug!(libfunc_name, "not implemented"),
            }
        }

        self.debug.next_line();

        // Move to the next state.
        self.move_to(CompilationState::CoreLibFunctionsProcessed)
    }
}
