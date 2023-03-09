use std::cmp::Ordering;
use std::collections::HashSet;

use cairo_lang_sierra::ids::VarId;
/// This file contains everything related to sierra statement processing.
use cairo_lang_sierra::program::{GenBranchTarget, GenStatement, Invocation};
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicMetadataValueEnum;
use itertools::Itertools;
use tracing::debug;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

/// Implementation of the statement processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process statements in the Sierra program.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra statements fails.

    pub fn process_statements(&mut self) {
        let mut statements_to_process: HashSet<usize> = self.user_functions.values().map(|f| f.entry_point).collect();
        let mut processed_statements: HashSet<usize> = HashSet::new();
        while !statements_to_process.is_empty() {
            let statement_idx = self
                .get_next_statement_to_process(&statements_to_process, &processed_statements)
                .expect("Expected statements to be serialisable");
            let statement = &self.program.statements[statement_idx];
            // Handling this now so we can safely use continue later
            statements_to_process.remove(&statement_idx);
            processed_statements.insert(statement_idx);

            // Statements are either sierra function calls or returns
            match statement {
                GenStatement::Invocation(invocation) => {
                    // Add next statements to the set to process
                    statements_to_process.extend(invocation.branches.iter().map(|b| match b.target {
                        GenBranchTarget::Fallthrough => statement_idx + 1,
                        GenBranchTarget::Statement(target_id) => target_id.0,
                    }));

                    // Find the block with the highest index below or equal to statement_idx
                    let block_info = self.get_block_info_for_statement_id(statement_idx);

                    self.builder.position_at_end(block_info.block);

                    // Get core lib function called by this instruction.
                    let fn_name = invocation.libfunc_id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).to_string();
                    debug!(fn_name, line = self.debug.get_line(), "processing statement: invocation");
                    // Since the function has only one branch, we can use the generic method of ensuring flow moves to
                    // the target

                    match invocation.branches.len().cmp(&1) {
                        // A standard instruction with one followup
                        Ordering::Equal => {
                            // First get the function to call, either a user defined function, or corelib one
                            let function = if fn_name.starts_with("function_call<") {
                                self.module
                                    .get_function(
                                        fn_name.strip_prefix("function_call<user@").unwrap().strip_suffix('>').unwrap(),
                                    )
                                    .unwrap()
                            } else {
                                // Regular corelib called.
                                self.module
                                    .get_function(fn_name.as_str())
                                    .unwrap_or_else(|| panic!("{fn_name} function is missing"))
                            };

                            // Next make sure any phis are in place such that all arguments are available in the current
                            // block
                            self.process_args(invocation, statement_idx, &function.get_type().get_param_types());

                            let processed_args: Vec<_> = invocation
                                .args
                                .iter()
                                .map(|arg| {
                                    BasicMetadataValueEnum::from(
                                        *self.get_processed_variable_at_statement(arg, statement_idx).unwrap(),
                                    )
                                })
                                .collect();

                            // Call the function.
                            let res = self
                                .builder
                                .build_call(function, &processed_args, "")
                                .try_as_basic_value()
                                .left()
                                .expect("Call should have worked");

                            if res.is_struct_value()
                                && res.into_struct_value().get_type().count_fields() > 0
                                && !fn_name.starts_with("store_temp<")
                                && !fn_name.starts_with("struct_construct<")
                                && !fn_name.starts_with("enum_init<")
                            {
                                println!("Unpacking tuple");
                                // self.unpack_tuple(&invocation.branches[0].results,
                                // res.into_struct_value())
                            } else {
                                // Just save the result.
                                println!("Not unpacking tuple");
                                // self.get_block_info_for_statement_id(statement_idx)
                                //     .variables
                                //     .insert(invocation.branches[0].results[0].id, res);
                                // self.register_variable_at_statement_id
                            }

                            self.insert_flow_control_if_necessary(statement_idx, &invocation.branches[0].target);
                        }
                        Ordering::Greater => {
                            // When control flow branches, we need special handling for each case
                            match fn_name.as_str() {
                                "felt_is_zero" => {
                                    self.felt_is_zero(invocation, statement_idx);
                                }
                                x if x.starts_with("enum_match<") => {
                                    self.enum_match(invocation, statement_idx);
                                }
                                _ => todo!("Unimplemented branching function {}", fn_name.as_str()),
                            }
                        }
                        Ordering::Less => {
                            panic!("Non-return instruction should have some number of control flow targets");
                        }
                    }
                }
                GenStatement::Return(_ret) => {
                    todo!();
                }
            }
        }
    }

    /// Ensures that all variables needed to be passed to a function are available at the current
    /// block, inserting phi instructions as necessary.
    ///
    /// # Arguments
    ///
    /// * `invocation` - The function invocation.
    /// * `invocation_nb` - The index of the statement containing the invocation.
    /// * `types` - The types of the functions arguments
    ///
    /// # Errors
    ///
    /// Panics if the number of types is not the same as the number of arguments the function
    /// expects
    pub fn process_args(&mut self, invocation: &Invocation, invocation_nb: usize, types: &[BasicTypeEnum<'ctx>]) {
        // TODO could be made safer by ensuring it's only called when the builder is at the right block for
        // the statement?
        for (arg, arg_type) in invocation.args.iter().zip_eq(types.iter()) {
            self.process_variable_at_statement(arg, invocation_nb, *arg_type)
        }

        self.builder.position_at_end(self.get_block_info_for_statement_id(invocation_nb).block);
    }

    fn process_variable_at_statement(&mut self, variable_id: &VarId, invocation_nb: usize, ty: BasicTypeEnum<'ctx>) {
        let block_info = self.get_block_info_for_statement_id(invocation_nb);
        let variable_needs_processing = block_info.variables.contains_key(&variable_id.id);

        if variable_needs_processing {
            match block_info.preds.len().cmp(&1) {
                // If there is only one predecessor, process the variable there and retrieve it
                Ordering::Equal => {
                    let pred = *block_info.preds.get(&0).unwrap();
                    self.process_variable_at_statement(variable_id, pred, ty);
                    let previous_val = self.get_processed_variable_at_statement(variable_id, pred).unwrap();
                    let new_val = previous_val.to_owned();
                    self.basic_blocks
                        .range_mut(0..invocation_nb)
                        .next_back()
                        .unwrap()
                        .1
                        .variables
                        .insert(variable_id.id, new_val);
                }
                // If there are multiple predecessors, process the variable at each one, then create a phi node to unify
                // them
                Ordering::Greater => {
                    for pred in block_info.preds.clone().into_iter() {
                        self.process_variable_at_statement(variable_id, pred, ty);
                    }

                    let block_info = self.get_block_info_for_statement_id(invocation_nb);

                    self.builder.position_before(
                        &block_info
                            .block
                            .get_first_instruction()
                            .expect("Previous block should have at least one instruction"),
                    );
                    let phi = self.builder.build_phi(ty, &format!("phi{}_", variable_id.id));

                    for pred in block_info.preds.iter() {
                        let processed_var = self.get_processed_variable_at_statement(variable_id, *pred).unwrap();
                        let block = self.get_block_info_for_statement_id(*pred).block;
                        phi.add_incoming(&[(processed_var, block)]);
                    }
                }
                // If there are no predecessors and the variable needs processing, then something has gone wrong,
                // as there's nowhere to obtain it from
                Ordering::Less => {
                    panic!(
                        "Found path to root block from statement {} with variable {} undefined",
                        invocation_nb, variable_id.id
                    )
                }
            }
        }
    }

    /// Unpack a struct into several values.
    ///
    /// # Arguments
    ///
    /// * `results` - The program variables that need to be unpacked.
    /// * `res` - The struct to unpack.
    ///
    /// # Errors
    ///
    /// Panics if there is not enough fields.
    /// Panics if the pointer to the struct field is not valid.
    // fn unpack_tuple(&mut self, results: &[VarId], res: StructValue<'ctx>) {
    //     let res_type = res.get_type();
    //     let res_ptr = self.builder.build_alloca(res_type, "res_ptr");
    //     self.builder.build_store(res_ptr, res);
    //     for (field_index, VarId { id, debug_name: _ }) in results.iter().enumerate() {
    //         let field_type = res_type.get_field_type_at_index(field_index as u32).expect("Field type
    //     should exist");     let field_ptr = self
    //             .builder
    //             .build_struct_gep(res_type, res_ptr, field_index as u32,
    // format!("{id}_ptr").as_str())             .expect("Pointer should be valid");
    //         let field = self.builder.build_load(field_type, field_ptr, &id.to_string());
    //         self.variables.insert(*id, field);
    //     }
    //     todo!();
    // }

    fn get_next_statement_to_process(
        &self,
        statements_to_process: &HashSet<usize>,
        processed_statements: &HashSet<usize>,
    ) -> Option<usize> {
        statements_to_process
            .iter()
            .find(|idx| {
                self.basic_blocks
                    .get(idx)
                    .map(|x| x.preds.iter().all(|p| processed_statements.contains(p)))
                    .unwrap_or(true)
            })
            .copied()
    }
}
