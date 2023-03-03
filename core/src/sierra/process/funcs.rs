use tracing::debug;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::{CompilationState, Compiler, FunctionInfo};

/// Implementation for the type processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process types in the Sierra program.
    /// For each type declaration in the Sierra program, create a corresponding type in the LLVM
    /// context.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra types fails.
    pub fn process_funcs(&mut self) -> CompilerResult<()> {
        debug!("processing funcs");
        // Check that the current state is valid.
        self.check_state(&CompilationState::CoreLibFunctionsProcessed)?;

        // Loop through the function declarations (last category of the sierra file).
        for func_declaration in self.program.funcs.iter() {
            let func_name = func_declaration.id.debug_name.as_ref().unwrap().as_str();
            debug!("processing function declaration: {}", func_name);

            // Clear the variables map in case a previous function has been processed.
            self.variables.clear();
            self.debug.variables.clear();

            // Arguments of the function.
            let FunctionInfo { func, args, args_debug_types, debug_return_type } =
                self.generate_function_definition(func_declaration);

            self.builder.position_at_end(self.context.append_basic_block(func, "entry"));

            let debug_function = self.debug.create_function(
                func_name,
                &func,
                debug_return_type,
                &args_debug_types,
                Some(func_declaration.entry_point.0),
            );

            // Loop through the arguments of the function. The variable id counter always starts from zero for a
            // function.
            for (var_id, _) in args.iter().enumerate() {
                // Save the function arguments in the variables map to be able to use them in the function body.
                let variable = func.get_nth_param(var_id as u32).expect("Function should have enough parameters");
                self.variables.insert(var_id as u64, variable);
            }

            // Loop through arguments of the function, storing their debug info.
            for (var_id, ty) in args_debug_types.iter().enumerate() {
                self.debug.variables.insert(var_id as u64, *ty);
            }

            // process statements from the line stated in the function definition until the return instruction.
            // ex: fib_caller::fib_caller::main@21() -> (Unit); the function main starts at the statement 21.
            self.process_statements_from(func_declaration.entry_point.0, debug_function.scope)?;

            // The function has been processed. Valid functions have always 1 block and atleast 1 instruction.
            let first_instruction = func.get_first_basic_block().unwrap().get_first_instruction().unwrap();
            for (i, var) in func.get_param_iter().enumerate() {
                self.debug.insert_dbg_value(
                    var,
                    debug_function.params_local_vars[i],
                    debug_function.location,
                    first_instruction,
                );
            }

            self.debug.next_line();
        }
        // Move to the next state.
        self.move_to(CompilationState::FunctionsProcessed)
    }
}
