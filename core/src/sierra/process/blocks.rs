use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Bound::Included;

use cairo_lang_sierra::ids::VarId;
use cairo_lang_sierra::program::{GenBranchTarget, GenStatement};
use inkwell::values::BasicValueEnum;

use crate::sierra::llvm_compiler::{BasicBlockInfo, Compiler, FunctionInfo};

/// Implementation for the type processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn process_blocks(&mut self) {
        let enclosing_functions: BTreeMap<_, _> = self.user_functions.values().map(|f| (f.entry_point, f)).collect();

        self.basic_blocks = (0..self.program.statements.len())
            .map(|statement_idx| -> (usize, BasicBlockInfo) {
                let FunctionInfo { func, entry_point, args: _, debug: _ } = enclosing_functions
                    .range((Included(&0), Included(&statement_idx)))
                    .next_back()
                    .unwrap_or_else(|| {
                        panic!("Enclosing function should have been registered for statement {}", statement_idx)
                    })
                    .1;

                let block = self.context.append_basic_block(*func, &format!("statement{}", statement_idx));

                let variables = if statement_idx == *entry_point {
                    func.get_param_iter().enumerate().map(|(var_id, val)| (var_id as u64, val)).collect()
                } else {
                    HashMap::new()
                };

                (statement_idx, BasicBlockInfo { block, preds: HashSet::new(), variables })
            })
            .collect();

        for (statement_idx, statement) in self.program.statements.iter().enumerate() {
            match statement {
                GenStatement::Invocation(invocation) => {
                    for branch in invocation.branches.iter() {
                        match branch.target {
                            GenBranchTarget::Fallthrough => {
                                self.basic_blocks.get_mut(&(statement_idx + 1)).unwrap().preds.insert(statement_idx)
                            }
                            GenBranchTarget::Statement(target_id) => {
                                self.basic_blocks.get_mut(&target_id.0).unwrap().preds.insert(statement_idx)
                            }
                        };
                    }
                }
                GenStatement::Return(_) => continue,
            }
        }

        for (id, BasicBlockInfo { block, preds, variables }) in self.basic_blocks.iter() {
            println!("{id}: {}, {:?}, {:?}", block.get_name().to_str().unwrap(), preds, variables);
        }
    }

    pub fn get_block_info_for_statement_id(&self, statement_idx: usize) -> &BasicBlockInfo {
        self.basic_blocks
            .range((Included(&0), Included(&statement_idx)))
            .next_back()
            .unwrap_or_else(|| panic!("Basic block should have been registered for statement index {}", statement_idx))
            .1
    }

    /// immutable operation that just indexes into the variables map associated with the block
    /// containing the given statement id if the variable has not been defined !in this block!
    /// then this will return None
    ///
    /// # Arguments
    ///
    /// * `invocation_nb` - the index of the invocation statement to use the variable.
    ///
    /// # Returns
    ///
    /// * `Option<BasicValueEnum<'ctx>>` - The value iff it has been declared in this block, use
    ///   process_variable_at_statement to add a phi or find the value from a previous block if
    ///   neede
    ///
    /// # Errors
    /// None
    pub fn get_processed_variable_at_statement(
        &self,
        _variable_id: &VarId,
        _invocation_nb: usize,
    ) -> Option<&BasicValueEnum<'ctx>> {
        // self.get_block_info_for_statement_id(invocation_nb).variables.get(&variable_id.id)
        todo!()
    }

    pub fn register_variable_at_statement_id(
        &mut self,
        variable_id: &VarId,
        statement_idx: usize,
        val: BasicValueEnum<'ctx>,
    ) {
        self.basic_blocks
            .range_mut((Included(&0), Included(&statement_idx)))
            .next_back()
            .unwrap_or_else(|| panic!("Basic block should have been registered for statement index {}", statement_idx))
            .1
            .variables
            .insert(variable_id.id, val);
    }
}
