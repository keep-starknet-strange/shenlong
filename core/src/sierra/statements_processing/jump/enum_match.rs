use std::collections::HashMap;

use cairo_lang_sierra::program::Invocation;
use inkwell::values::FunctionValue;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of match enum {<branches>}.
    ///
    /// # Arguments
    ///
    /// * `func` - The llvm function object, needed to insert the switch default block into
    /// * `invocation` - The invocation object.
    /// * `invocation_nb` - The invocation number.
    ///
    /// # Error
    ///
    /// Returns an error if the processing of the branches statements fails.
    pub fn enum_match(&mut self, func: FunctionValue<'ctx>, invocation: &Invocation, invocation_nb: usize) {
        let full_function_name = invocation.libfunc_id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string();
        let enum_name = full_function_name
            .strip_prefix("enum_match<")
            .expect("enum match debug name should start with 'enum_match<'")
            .strip_suffix('>')
            .expect("enum match debug name should end with '>'");

        let enum_index_type = self
            .types_by_name
            .get(&format!("ut@{}", enum_name))
            .expect("Index type should have been registered before enum match")
            .into_int_type();

        let enum_to_match =
            self.dataflow_graph.use_variable_at_statement(invocation_nb, &invocation.args[0]).into_struct_value();

        let enum_index = self
            .builder
            .build_extract_value(enum_to_match, 0, &format!("enum_index{}", invocation_nb))
            .unwrap()
            .into_int_value();

        let mut payload_by_location = HashMap::new();
        for (index_value, branch) in invocation.branches.iter().enumerate() {
            let payload_location = self.enum_packing_index_by_name.get(enum_name).unwrap()[index_value];
            let value = if let Some(value) = payload_by_location.get(&payload_location) {
                *value
            } else {
                let value = self.builder.build_extract_value(enum_to_match, payload_location as u32, "").unwrap();
                payload_by_location.insert(payload_location, value);
                value
            };
            let target = match branch.target {
                cairo_lang_sierra::program::GenBranchTarget::Fallthrough => invocation_nb + 1,
                cairo_lang_sierra::program::GenBranchTarget::Statement(id) => id.0,
            };
            self.dataflow_graph.claim_variable_for_branch(invocation_nb, target, &branch.results[0], value);
        }

        let targets: Vec<_> = invocation
            .branches
            .iter()
            .map(|b| match b.target {
                cairo_lang_sierra::program::GenBranchTarget::Fallthrough => invocation_nb + 1,
                cairo_lang_sierra::program::GenBranchTarget::Statement(statement_idx) => statement_idx.0,
            })
            .map(|jump_dest| self.dataflow_graph.get_block_for_entrypoint(jump_dest).unwrap())
            .enumerate()
            .map(|(idx, block)| (enum_index_type.const_int(idx as u64, false), block))
            .collect();
        let else_block = self.context.append_basic_block(func, &format!("enum_match{}default", invocation_nb));
        self.builder.build_switch(enum_index, else_block, &targets);

        self.builder.position_at_end(else_block);
        self.builder.build_unreachable();
    }
}
