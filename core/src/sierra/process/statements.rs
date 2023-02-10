use cairo_lang_sierra::ids::VarId;
/// This file contains everything related to sierra statement processing.
use cairo_lang_sierra::program::{GenBranchTarget, GenStatement, Invocation};
use inkwell::values::{BasicMetadataValueEnum, StructValue};
use log::debug;

use crate::sierra::errors::{CompilerResult, DEBUG_NAME_EXPECTED};
use crate::sierra::llvm_compiler::Compiler;

/// Implementation of the statement processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process statements in the Sierra program.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra statements fails.
    pub fn process_statements_from_until(&mut self, from: usize, until: Option<usize>) -> CompilerResult<()> {
        debug!("processing statements");
        let statements =
            if let Some(end) = until { &self.program.statements[from..end] } else { &self.program.statements[from..] };
        // Check that the current state is valid.
        for (statement_id, statement) in statements.iter().enumerate() {
            match statement {
                // If the statement is a sierra function call.
                GenStatement::Invocation(invocation) => {
                    println!("Invocation {:?}", invocation.libfunc_id.debug_name);
                    self.module.print_to_file("./core/tests/test_data/llvm/test.ll")?;
                    let fn_name = invocation.libfunc_id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string();
                    if invocation.branches.len() == 1 && invocation.branches[0].results.is_empty() {
                        match fn_name.as_str() {
                            "jump" => {
                                let from = match &invocation.branches[0].target {
                                    GenBranchTarget::Statement(id) => id.0,
                                    _ => panic!("Jump should have genbranchinfo"),
                                };
                                self.process_statements_from_until(from, None)?;
                            }
                            _ => continue,
                        }
                    }
                    if invocation.branches.len() > 1 {
                        match fn_name.as_str() {
                            "felt_is_zero" => {
                                self.felt_is_zero(invocation, statement_id)?;
                                break;
                            }
                            _ => continue,
                        }
                    }
                    if invocation.branches.len() == 1 && invocation.branches[0].target == GenBranchTarget::Fallthrough {
                        let function = if invocation.libfunc_id.debug_name.clone().unwrap().starts_with("function_call")
                        {
                            self.module
                                .get_function(
                                    invocation
                                        .libfunc_id
                                        .debug_name
                                        .clone()
                                        .unwrap()
                                        .strip_prefix("function_call<user@")
                                        .unwrap()
                                        .strip_suffix('>')
                                        .unwrap(),
                                )
                                .unwrap()
                        } else {
                            self.module.get_function(fn_name.as_str()).unwrap()
                        };
                        let args = self.process_args(invocation);
                        let res = self
                            .builder
                            .build_call(function, &args, "")
                            .try_as_basic_value()
                            .left()
                            .expect("Call should have worked");
                        if res.is_struct_value() {
                            self.unpack_tuple(&invocation.branches[0].results, res.into_struct_value())
                        } else {
                            self.variables.insert(invocation.branches[0].results[0].id.to_string(), res);
                        }
                    }
                }
                // Return == return instruction
                GenStatement::Return(ret) => {
                    if !ret.is_empty() {
                        let mut types = vec![];
                        let mut values = vec![];

                        for ret_var in ret.iter() {
                            let value = self.variables.get(&ret_var.id.to_string()).unwrap();
                            values.push(value);
                            types.push(value.get_type());
                        }
                        let return_struct_ptr =
                            self.builder.build_alloca(self.context.struct_type(&types, false), "ret_struct_ptr");
                        for (index, value) in values.iter().enumerate() {
                            let tuple_ptr = self
                                .builder
                                .build_struct_gep(
                                    return_struct_ptr,
                                    index.try_into().unwrap(),
                                    format!("field_{index}_ptr").as_str(),
                                )
                                .expect("Pointer should be valid");
                            self.builder.build_store(tuple_ptr, **value);
                        }
                        // Return the specified value.
                        self.builder
                            .build_return(Some(&self.builder.build_load(return_struct_ptr, "return_struct_value")));
                    }
                }
            }
        }
        Ok(())
    }
    fn process_args(&self, invocation: &Invocation) -> Vec<BasicMetadataValueEnum<'ctx>> {
        let mut args = vec![];
        if !invocation.args.is_empty() {
            for argument in invocation.args.iter() {
                args.push(
                    (*self.variables.get(&argument.id.to_string()).unwrap_or_else(|| {
                        panic!("Variable {:} passed as argument should have been declared first", argument.id)
                    }))
                    .into(),
                );
            }
        }
        args
    }

    fn unpack_tuple(&mut self, results: &[VarId], res: StructValue<'ctx>) {
        let res_ptr = self.builder.build_alloca(res.get_type(), "res_ptr");
        self.builder.build_store(res_ptr, res);
        for (field_index, VarId { id, debug_name: _ }) in results.iter().enumerate() {
            let id = id.to_string();
            let field_ptr = self
                .builder
                .build_struct_gep(res_ptr, field_index as u32, format!("{id}_ptr").as_str())
                .expect("Pointer should be valid");
            let field = self.builder.build_load(field_ptr, id.as_str());
            self.variables.insert(id, field);
        }
    }
}
