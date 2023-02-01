use cairo_lang_sierra::ids::VarId;
/// This file contains everything related to sierra statement processing.
use cairo_lang_sierra::program::{GenBranchTarget, GenStatement, Invocation};
use inkwell::values::{BasicMetadataValueEnum, PointerValue};
use log::debug;

use crate::sierra::errors::{CompilerError, CompilerResult};
use crate::sierra::llvm_compiler::Compiler;

/// Implementation of the statement processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process statements in the Sierra program.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra statements fails.
    pub fn process_statements_from_until_return(&mut self, from: usize) -> CompilerResult<()> {
        debug!("processing statements");
        // Check that the current state is valid.
        for statement in &self.program.statements.clone()[from..] {
            match statement {
                // If the statement is a sierra function call.
                GenStatement::Invocation(invocation) => {
                    println!("Invocation {:?}", invocation.libfunc_id.debug_name);
                    self.module.print_to_file("./core/tests/test_data/llvm/test.ll")?;
                    if invocation.branches.len() == 1 && invocation.branches[0].results.is_empty() {
                        continue;
                    }
                    let fn_name = invocation
                        .libfunc_id
                        .debug_name
                        .clone()
                        .expect("This compiler only works with sierra compiled with --replace-ids")
                        .to_string();

                    if invocation.branches.len() > 1 {
                        match fn_name.as_str() {
                            "felt_is_zero" => {
                                self.felt_is_zero(invocation)?;
                                continue;
                            }
                            _ => continue,
                        }
                    }
                    if invocation.branches.len() == 1 && invocation.branches[0].target == GenBranchTarget::Fallthrough {
                        let function = self.module.get_function(fn_name.as_str()).unwrap();
                        let args = self.process_args(invocation);
                        let res = self.builder.build_call(function, &args, "call").try_as_basic_value().left();

                        let ptr = if invocation.branches[0].results.len() == 1 {
                            let id = invocation.branches[0].results[0].id.to_string();
                            match self.variables.get(&id) {
                                Some(pointer) => *pointer,
                                None => {
                                    let ptr = self.builder.build_alloca(
                                        function
                                            .get_type()
                                            .get_return_type()
                                            .expect("Function should have return type"),
                                        id.as_str(),
                                    );
                                    self.variables.insert(id, ptr);
                                    ptr
                                }
                            }
                        } else {
                            self.builder.build_alloca(
                                function.get_type().get_return_type().expect("Function should have return type"),
                                "res",
                            )
                        };
                        self.builder.build_store(ptr, res.unwrap());
                        if invocation.branches[0].results.len() > 1 {
                            self.unpack_tuple(&invocation.branches[0].results, &ptr)
                        }
                    }
                }
                // Return == return instruction
                GenStatement::Return(ret) => {
                    if ret.len() == 1 {
                        // Return the specified value.
                        self.builder.build_return(Some(
                            &self.builder.build_load(
                                *self
                                    .variables
                                    .get(&ret[0].id.to_string())
                                    .ok_or(CompilerError::VarNotFound(ret[0].id.to_string()))?,
                                &ret[0].id.to_string(),
                            ),
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    fn process_args(&self, invocation: &Invocation) -> Vec<BasicMetadataValueEnum<'ctx>> {
        let mut args = vec![];
        if !invocation.args.is_empty() {
            // Gets the saved pointer from the variables HashMap.
            // Load its value.
            // Inject it in the function.
            for argument in invocation.args.iter() {
                args.push(
                    self.builder
                        .build_load(
                            *self.variables.get(&argument.id.to_string()).unwrap_or_else(|| {
                                panic!("Variable {:} passed as argument should have been declared first", argument.id)
                            }),
                            &argument.id.to_string(),
                        )
                        .into(),
                );
            }
        }
        args
    }

    fn unpack_tuple(&mut self, results: &[VarId], res_ptr: &PointerValue<'ctx>) {
        for (field_index, VarId { id, debug_name: _ }) in results.iter().enumerate() {
            let id = id.to_string();
            let field_ptr = self
                .builder
                .build_struct_gep(*res_ptr, field_index as u32, format!("{id}_ptr").as_str())
                .expect("Pointer should be valid");
            self.variables.insert(id, field_ptr);
        }
    }
}
