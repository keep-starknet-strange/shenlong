use cairo_lang_sierra::program::{GenBranchTarget, GenStatement};
use inkwell::values::BasicValue;
use log::debug;

use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::llvm_compiler::{CompilationState, Compiler};

/// Implementation of the statement processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Build an artificial main function to be able to run the generated LLVM IR file.
    fn build_main(&mut self) -> CompilerResult<()> {
        let args_type =
            self.types.get("felt").ok_or(CompilerError::TypeNotFound("felt".to_owned()))?;
        let main_type = args_type.fn_type(&[], false);
        let main_func = self.module.add_function("main", main_type, None);
        let main_bb = self.context.append_basic_block(main_func, "entry");
        self.builder.position_at_end(main_bb);

        Ok(())
    }

    /// Allocate an llvm pointer variable and save it in the hashmap.
    /// It is needed to be able to track all the variables returned.
    /// We need to track them to pass them as arguments for future function calls.
    fn save_in_var(&mut self, id: &u64) -> CompilerResult<()> {
        // Currently only supports felt variables.
        let felt_type =
            self.types.get("felt").ok_or(CompilerError::TypeNotFound("felt".to_owned()))?;
        // Get the variable pointer.
        let var_ptr =
            self.builder.build_alloca(felt_type.as_basic_type_enum(), &format!("{id:}_ptr"));
        self.variables.insert(id.to_string(), Some(var_ptr));
        Ok(())
    }

    /// Process statements in the Sierra program.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra statements fails.
    pub fn process_statements(&mut self) -> CompilerResult<()> {
        debug!("processing statements");
        // Check that the current state is valid.
        self.check_state(&CompilationState::CoreLibFunctionsProcessed)?;
        self.build_main()?;
        for statement in self.program.statements.iter() {
            match statement {
                // If the statement is a sierra function call.
                GenStatement::Invocation(invocation) => {
                    // Get the LLVM IR processed function.
                    let func =
                        self.module.get_function(invocation.libfunc_id.id.to_string().as_str());
                    // Declare an empty vec for the function call arguments.
                    let mut args = vec![];
                    // If the function needs argments fill them with the right variables.
                    if !invocation.args.is_empty() {
                        // Gets the saved pointer from the variables HashMap.
                        // Load its value.
                        // Inject it in the function.
                        for argument in invocation.args.iter() {
                            args.push(
                                self.builder
                                    .build_load(
                                        self.variables
                                            .get(&argument.id.to_string())
                                            .ok_or(CompilerError::VarNotFound(
                                                argument.id.to_string(),
                                            ))?
                                            .ok_or(CompilerError::VarNotFound(
                                                argument.id.to_string(),
                                            ))?,
                                        &argument.id.to_string(),
                                    )
                                    .into(),
                            );
                        }
                    }
                    // Having only one branch == no if/else.
                    // Fallthrough == keep executing the instructions as they come.
                    if invocation.branches.len() == 1
                        && invocation.branches[0].target == GenBranchTarget::Fallthrough
                    {
                        // Create a variable for each return value (might not be needed though)
                        for result in invocation.branches[0].results.iter() {
                            self.save_in_var(&result.id)?;
                        }
                    }

                    // If a LLVM IR function was created for this sierra function call it.
                    let res = if let Some(function) = func {
                        self.builder
                            .build_call(function, &args, "")
                            .try_as_basic_value()
                            .left()
                            .ok_or(CompilerError::NoReturnValue)?
                    } else {
                        // else just get the argument value.
                        args[0].into_int_value().as_basic_value_enum()
                    };
                    // Get the pointer to the corresponding variable.
                    let ptr = self
                        .variables
                        .get(invocation.branches[0].results[0].id.to_string().as_str())
                        .ok_or(CompilerError::VarNotFound(
                            invocation.branches[0].results[0].id.to_string(),
                        ))?
                        .ok_or(CompilerError::VarNotFound(
                            invocation.branches[0].results[0].id.to_string(),
                        ))?;
                    // Save the function return value in the specified variable.
                    self.builder.build_store(ptr, res);
                }
                // Return == return instruction
                GenStatement::Return(ret) => {
                    if ret.len() == 1 {
                        // Return the specified value.
                        self.builder.build_return(Some(
                            &self.builder.build_load(
                                self.variables
                                    .get(&ret[0].id.to_string())
                                    .ok_or(CompilerError::VarNotFound(ret[0].id.to_string()))?
                                    .ok_or(CompilerError::VarNotFound(ret[0].id.to_string()))?,
                                &ret[0].id.to_string(),
                            ),
                        ));
                    }
                }
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::StatementsProcessed)
    }
}
