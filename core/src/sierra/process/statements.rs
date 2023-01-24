use cairo_lang_sierra::program::{GenBranchTarget, GenStatement};
use inkwell::values::BasicValue;
use log::debug;

use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::llvm_compiler::{CompilationState, Compiler};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
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
    fn save_in_var(&mut self, id: &u64) -> CompilerResult<()> {
        let felt_type =
            self.types.get("felt").ok_or(CompilerError::TypeNotFound("felt".to_owned()))?;
        let var_ptr =
            self.builder.build_alloca(felt_type.as_basic_type_enum(), &format!("{id:}_ptr"));
        self.variables.insert(id.to_string(), Some(var_ptr));
        Ok(())
    }

    /// Process statements in the Sierra program.
    pub fn process_statements(&mut self) -> CompilerResult<()> {
        debug!("processing statements");
        // Check that the current state is valid.
        self.check_state(&CompilationState::CoreLibFunctionsProcessed)?;
        self.build_main()?;
        for statement in self.program.statements.iter() {
            match statement {
                GenStatement::Invocation(invocation) => {
                    let func =
                        self.module.get_function(invocation.libfunc_id.id.to_string().as_str());

                    let mut args = vec![];
                    if !invocation.args.is_empty() {
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
                    if invocation.branches.len() == 1
                        && invocation.branches[0].target == GenBranchTarget::Fallthrough
                    {
                        for result in invocation.branches[0].results.iter() {
                            self.save_in_var(&result.id)?;
                        }
                    }
                    let res = if let Some(function) = func {
                        self.builder
                            .build_call(function, &args, "call_felt_add")
                            .try_as_basic_value()
                            .left()
                            .ok_or(CompilerError::NoReturnValue)?
                    } else {
                        args[0].into_int_value().as_basic_value_enum()
                    };
                    let ptr = self
                        .variables
                        .get(invocation.branches[0].results[0].id.to_string().as_str())
                        .ok_or(CompilerError::VarNotFound(
                            invocation.branches[0].results[0].id.to_string(),
                        ))?
                        .ok_or(CompilerError::VarNotFound(
                            invocation.branches[0].results[0].id.to_string(),
                        ))?;
                    self.builder.build_store(ptr, res);
                }
                GenStatement::Return(ret) => {
                    if ret.len() == 1 {
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
