use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Bound::Included;

use cairo_lang_sierra::ids::VarId;
use cairo_lang_sierra::program::{GenBranchTarget, GenStatement, Program};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use tracing::debug;

use crate::sierra::errors::CompilerResult;
use crate::sierra::llvm_compiler::{CompilationState, Compiler, FunctionInfo};

/// Implementation for the dataflow processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process the control flow inside each user function.
    /// For each function, creates the basic blocks and stores which ones precede and follow which.
    /// The dataflow graph initialised with this structure will later be used to track the creation
    /// and availability of variables during statement processing Other basic blocks can be
    /// created later if necessary, but they will not be able to have sierra dataflow associated
    /// with them so this should usually be avoided
    ///
    /// # Errors
    ///
    /// If the user function signatures haven't been processed or if the compiler's state is not
    /// FunctionsProcessed
    pub fn process_dataflow(&mut self) -> CompilerResult<()> {
        debug!("processing dataflow");
        // Check that the current state is valid.
        self.check_state(&CompilationState::FunctionsProcessed)?;

        self.dataflow_graph = DataFlowGraph::create(self.program, self.context, &self.user_functions);

        // Move to the next state.
        self.move_to(CompilationState::ControlFlowProcessed)
    }
}

#[derive(Debug)]
struct BlockInfo<'ctx> {
    // the llvm basic block
    block: BasicBlock<'ctx>,
    // ids of blocks directly preceding this one
    preds: HashSet<usize>,
    // ids of blocks that can immediately follow this one
    next: HashSet<usize>,
    // llvm value associated with each variable id at this block
    variables: HashMap<u64, BasicValueEnum<'ctx>>,
    // variables dropped during the course of this block
    dropped_variables: HashSet<u64>,
    // the last statement id in the block
    endpoint: usize,
}

pub struct DataFlowGraph<'ctx> {
    blocks: BTreeMap<usize, BlockInfo<'ctx>>,
}

impl<'ctx> DataFlowGraph<'ctx> {
    /// Create an empty DataFlowGraph
    pub fn new() -> Self {
        DataFlowGraph { blocks: BTreeMap::new() }
    }

    // Create a populated DataFlowGraph for the given functions in the Sierra program
    pub fn create(program: &Program, context: &'ctx Context, functions: &HashMap<String, FunctionInfo<'ctx>>) -> Self {
        // Each function is a continuous set of statements, with no gaps between it and the other functions,
        // so we first create a BTreeMap indexed by their start point to be able to find which function each
        // statement belongs to
        let enclosing_functions: BTreeMap<_, _> = functions.values().map(|f| (f.entry_point, f)).collect();

        // This maps each statement in the program to a basic block in the pre-existing llvm function
        // associated with the Sierra function in which it resides We could make a smaller set of
        // basic blocks here by having more than 1 statement per block and the methods will still work. This
        // is just simpler and llvm should optimise it anyway
        let mut blocks: BTreeMap<_, _> = program
            .statements
            .iter()
            .enumerate()
            .map(|(statement_idx, statement)| -> (usize, BlockInfo) {
                // Find the llvm function for the sierra function containing the given statement
                let FunctionInfo { func, entry_point, args: _, debug: _ } = enclosing_functions
                    .range((Included(&0), Included(&statement_idx)))
                    .next_back()
                    .unwrap_or_else(|| {
                        panic!("Enclosing function should have been registered for statement {}", statement_idx)
                    })
                    .1;

                // Insert a fresh block into the function
                let block = context.append_basic_block(*func, &format!("statement{}", statement_idx));

                // If this is the first block in the function, include the arguments in the map of accessible
                // variables
                let variables = if statement_idx == *entry_point {
                    func.get_param_iter().enumerate().map(|(var_id, val)| (var_id as u64, val)).collect()
                } else {
                    HashMap::new()
                };

                // Create and populate the set of blocks immediately reachable from this one
                let mut next = HashSet::new();
                if let GenStatement::Invocation(invocation) = statement {
                    for branch in invocation.branches.iter() {
                        match branch.target {
                            GenBranchTarget::Fallthrough => next.insert(statement_idx + 1),
                            GenBranchTarget::Statement(target_id) => next.insert(target_id.0),
                        };
                    }
                }

                // Return the entry in the tree
                (
                    statement_idx,
                    BlockInfo {
                        block,
                        preds: HashSet::new(),
                        next,
                        variables,
                        dropped_variables: HashSet::new(),
                        endpoint: statement_idx + 1,
                    },
                )
            })
            .collect();

        // The graph is doubly linked for convenience, so now we need to go through and populate the
        // predecessors from the collected successors
        let mut preds: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (id, block_info) in blocks.iter_mut() {
            for next in block_info.next.iter() {
                if let Some(set) = preds.get_mut(next) {
                    set.insert(*id);
                } else {
                    preds.insert(*next, HashSet::from([*id]));
                }
            }
        }
        for (to, from) in preds {
            blocks.get_mut(&to).unwrap().preds = from;
        }

        DataFlowGraph { blocks }
    }

    // Get the basic block whose first statement has index `entrypoint`
    pub fn get_block_for_entrypoint(&self, entrypoint: usize) -> Option<BasicBlock> {
        self.blocks.get(&entrypoint).map(|x| x.block)
    }

    // Data flow in Sierra (ignoring user-function calls) is a DAG,
    // this linearises the graph, providing an order in which to process the blocks of a function
    // such that a block's predecessors are always processed before it is
    pub fn get_ordered_reachable_blocks_from(&self, entrypoint: usize) -> Vec<(usize, usize)> {
        let mut processed_blocks: Vec<(usize, usize)> = Vec::new();
        let mut blocks_to_process: HashSet<usize> = HashSet::from([entrypoint]);
        while !blocks_to_process.is_empty() {
            // find a block which has all its predecessors already in processed_blocks
            let block_idx = blocks_to_process
                .iter()
                .find(|idx| {
                    self.blocks
                        .get(idx)
                        .map(|x| x.preds.iter().all(|p| processed_blocks.iter().any(|(start, _end)| start == p)))
                        .unwrap_or(false)
                })
                .copied()
                .expect("Expected blocks to be serialisable");

            blocks_to_process.remove(&block_idx);
            blocks_to_process.extend(self.blocks.get(&block_idx).unwrap().next.iter());
            processed_blocks.push((block_idx, self.blocks.get(&block_idx).unwrap().endpoint));
        }

        processed_blocks
    }

    /// Register a value and type to a given variable id at a particular point in the flow graph
    ///
    /// # Arguments
    ///
    /// * `statement_idx` - The id of the sierra statement registering the value. Note that
    ///   variables are grouped by block, so for edge cases it only matters that statement_idx is in
    ///   the correct block
    /// * `variable_id` - The sierra id of the variable to register
    /// * `value` - The llvm value to store in the variable
    /// * `ty` - The llvm type to store
    ///
    /// # Errors
    ///
    /// Panics if a variable with this id has already been declared in this block or any which
    /// precede it
    pub fn create_variable_at_statement(
        &mut self,
        statement_idx: usize,
        variable_id: &VarId,
        value: BasicValueEnum<'ctx>,
    ) {
        let entrypoint = self.get_entrypoint_for_statement(statement_idx);
        assert!(
            !self.get_usable_variables_at_entrypoint(entrypoint).contains_key(&variable_id.id),
            "Attempted to create variable {} at block {} when it was already available",
            variable_id.id,
            entrypoint
        );
        let block_info = self.blocks.get_mut(&entrypoint).unwrap();
        block_info.variables.insert(variable_id.id, value);
        // dup<felt>([2]) -> ([2], [4]); is valid sierra, so if the variable has been dropped in this block,
        // we 'undrop' it if it was dropped in a previous block then we don't need to do anything
        // because the collection algorithm starts at the earliest block and works forward
        block_info.dropped_variables.remove(&variable_id.id);
    }

    /// Get the value and type of a sierra variable, and mark it as used. Sierra variables must be
    /// used exactly once during the course of the program
    ///
    /// # Arguments
    ///
    /// * `statement_idx` - The id of the sierra statement using the value. Note that variables are
    ///   grouped by block, so for edge cases it only matters that statement_idx is in the correct
    ///   block
    /// * `variable_id` - The sierra id of the variable to use
    ///
    /// # Returns
    ///
    /// # Errors
    ///
    /// Panics if a variable with this id has not been defined at this block, if it has only been
    /// defined in some but not all predecessors, or if it's already been used
    pub fn use_variable_at_statement(&mut self, statement_idx: usize, variable_id: &VarId) -> BasicValueEnum<'ctx> {
        let entrypoint = self.get_entrypoint_for_statement(statement_idx);
        assert!(
            self.get_usable_variables_at_entrypoint(entrypoint).contains_key(&variable_id.id),
            "Attempted to use variable {} at block {} when it was not available",
            variable_id.id,
            entrypoint
        );
        let block_info = self.blocks.get_mut(&entrypoint).unwrap();
        block_info.dropped_variables.insert(variable_id.id);
        let val = self.get_variable_at_block(entrypoint, variable_id);
        val
    }

    // Creates a variable at statement_idx, but marks it dropped at every successor except for
    // branch_idx
    pub fn claim_variable_for_branch(
        &mut self,
        statement_idx: usize,
        branch_idx: usize,
        variable_id: &VarId,
        value: BasicValueEnum<'ctx>,
    ) {
        let entrypoint = self.get_entrypoint_for_statement(statement_idx);
        assert!(
            !self.get_usable_variables_at_entrypoint(entrypoint).contains_key(&variable_id.id),
            "Attempted to create branch-specific variable {} at block {} when it was already available",
            variable_id.id,
            entrypoint
        );
        self.create_variable_at_statement(statement_idx, variable_id, value);
        let successors = self.blocks.get(&entrypoint).unwrap().next.clone();
        for successor in successors {
            if successor != branch_idx {
                self.blocks.get_mut(&successor).unwrap().dropped_variables.insert(variable_id.id);
            }
        }
    }

    // Recursive function that searches back through the graph for the given variable.
    // Should only be used when there is a non-branching path back to the node holding the variable
    // (a.k.a) when assert_variable_usable_at_entrypoint holds
    fn get_variable_at_block(&self, entrypoint: usize, variable_id: &VarId) -> BasicValueEnum<'ctx> {
        let block_info = self.blocks.get(&entrypoint).unwrap();
        block_info
            .variables
            .get(&variable_id.id)
            .copied()
            .unwrap_or_else(|| self.get_variable_at_block(*block_info.preds.iter().next().unwrap(), variable_id))
    }

    // Writes phi instructions to unify
    pub fn write_phis(&mut self, entrypoint: usize, builder: &Builder<'ctx>) {
        let block_info = self
            .blocks
            .get(&entrypoint)
            .expect("Write_phis should only be called with an `entrypoint` at the start of a block");

        // We're only concerned with where blocks are directly converging
        if block_info.preds.len() <= 1 {
            return;
        }

        // For each preceding block, get all the usable variables
        // This should produce a set of maps with the same keys and the same types, but potentially
        // different values
        let pred_vars_by_block =
            block_info.preds.iter().map(|pred| (pred, self.get_usable_variables_at_entrypoint(*pred)));

        // Next we transform this into a map from variable id to block id to value
        let mut phi_inputs_by_var: HashMap<u64, HashMap<usize, BasicValueEnum>> = HashMap::new();
        for (pred, vars) in pred_vars_by_block {
            for (var_id, val) in vars {
                if let Some(map) = phi_inputs_by_var.get_mut(&var_id) {
                    map.insert(*pred, val);
                } else {
                    phi_inputs_by_var.insert(var_id, HashMap::from([(*pred, val)]));
                }
            }
        }

        // Each entry now corresponds to one phi, so we check their validity then create the phi
        let mut values_to_insert = HashMap::new();
        for (var_id, value_map) in phi_inputs_by_var {
            // Assert that every predecessor returned the same usable variable ids
            assert_eq!(
                value_map.len(),
                block_info.preds.len(),
                "Expected every convergent branch to have the same number of available variables"
            );
            assert!(
                value_map.keys().all(|block| block_info.preds.contains(block)),
                "Expected every convergent branch to have the same available variable"
            );

            // Assert that every potential value for the given variable id has the same type
            let example_value = value_map.values().next().unwrap();
            let expected_type = example_value.get_type();
            assert!(value_map.values().all(|val| val.get_type() == expected_type));

            // If every previous block associates the given sierra variable with the exact same llvm value,
            // there's no need to create an actual phi
            if value_map.values().all(|val| val == example_value) {
                // It is however necessary to record this value in the graph such that the linear traversal
                // algorithms still work
                values_to_insert.insert(var_id, *example_value);
                continue;
            }

            let phi = builder.build_phi(expected_type, &format!("phi_{}_", var_id));
            values_to_insert.insert(var_id, phi.as_basic_value());
            for (block_id, val) in value_map {
                let block = self.blocks.get(&block_id).unwrap().block;
                phi.add_incoming(&[(&val, block)]);
            }
        }

        // Register these phis as values for their associated variables, findable by the linear search
        // algorithms
        self.blocks.get_mut(&entrypoint).unwrap().variables.extend(values_to_insert.iter());
    }

    // Utilises the fact that blocks are contiguous, finding the highest id less than or equal to the
    // given
    fn get_entrypoint_for_statement(&self, statement_idx: usize) -> usize {
        *self.blocks.range((Included(&0), Included(&statement_idx))).next_back().unwrap().0
    }

    // Collects all variables defined since the last point of flow convergence, minus those that were
    // dropped
    fn get_usable_variables_at_entrypoint(&self, entrypoint: usize) -> HashMap<u64, BasicValueEnum<'ctx>> {
        let block_info = self.blocks.get(&entrypoint).unwrap();

        // Note, order is important here for the drops to be applied correctly. Variables are not
        // necessarily dropped in the same block they are defined in First get those available at
        // the previous block
        let mut available_variables = if block_info.preds.len() == 1 {
            self.get_usable_variables_at_entrypoint(*block_info.preds.iter().next().unwrap())
        } else {
            HashMap::new()
        };

        // Then add in those defined at this block
        available_variables.extend(block_info.variables.iter());

        // Finally remove those dropped at this block
        available_variables.into_iter().filter(|(var_id, _)| !block_info.dropped_variables.contains(var_id)).collect()
    }

    pub fn print_dataflow_trace_from_statement(&self, statement_idx: usize) {
        let entrypoint = self.get_entrypoint_for_statement(statement_idx);
        for (index, block_info) in self.blocks.iter() {
            let usable = self.get_usable_variables_at_entrypoint(*index);
            println!(
                "block {index}: from: {:?} to: {:?} {}",
                block_info.preds,
                block_info.next,
                if *index == entrypoint { "<----" } else { "" }
            );
            println!(
                "    local vars: {:?} | local dropped: {:?} | available {:?}",
                block_info.variables.keys().collect::<Vec<_>>(),
                block_info.dropped_variables.iter().collect::<Vec<_>>(),
                usable.keys().collect::<Vec<_>>()
            );
        }
    }
}

impl<'ctx> Default for DataFlowGraph<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}
