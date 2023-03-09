use cairo_lang_sierra::program::Invocation;

use crate::sierra::llvm_compiler::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Implementation of the LLVM IR conversion of match enum {<branches>}.
    ///
    /// # Arguments
    ///
    /// * `invocation` - The invocation object.
    /// * `invocation_nb` - The invocation number.
    ///
    /// # Error
    ///
    /// Returns an error if the processing of the branches statements fails.
    pub fn enum_match(
        &mut self,
        // func: FunctionValue<'ctx>,
        _invocation: &Invocation,
        _invocation_nb: usize,
        // scope: DIScope<'ctx>,
    ) {
        // println!("\n\n---{:?}---\n\n", func);
        // println!("---{:?}---", invocation);
        // println!("---{:?}---", invocation_nb);
        // println!("---{:?}---\n\n", scope);
        // let full_function_name =
        // invocation.libfunc_id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string();
        // let enum_name = full_function_name
        //     .strip_prefix("enum_match<")
        //     .expect("enum match debug name should start with 'enum_match<'")
        //     .strip_suffix('>')
        //     .expect("enum match debug name should end with '>'");
        // println!("Enum name: {enum_name}");

        // let enum_to_match = self
        //     .variables
        //     .get(&invocation.args[0].id)
        //     .expect("Variable for input to enum_match should exist")
        //     .into_struct_value();
        // println!("\n    enum_to_match {:?}", enum_to_match);

        // let enum_index_type = self
        //     .types_by_name
        //     .get(&format!("ut@{}", enum_name))
        //     .expect("Index type should have been registered before enum match")
        //     .into_int_type();
        // println!("\n    enum_index_type {:?}", enum_index_type);

        // // self.builder.build_struct_gep(index_type, enum_to_match, index, name)

        // let enum_index = self
        //     .builder
        //     .build_extract_value(enum_to_match, 0, &format!("enum_index{}", invocation_nb))
        //     .unwrap()
        //     .into_int_value();

        // let cases: Vec<_> = (0..invocation.branches.len())
        //     .map(|index| {
        //         (
        //             enum_index_type.const_int(index as u64, false),
        //             self.context.append_basic_block(func, &format!("enum_match{}branch{}",
        // invocation_nb, index)),         )
        //     })
        //     .collect();
        // let else_block = self.context.append_basic_block(func, &format!("enum_match{}default",
        // invocation_nb)); self.builder.build_switch(enum_index, else_block, &cases);

        // self.builder.position_at_end(else_block);
        // self.builder.build_unreachable();

        // for (index, (_, block)) in cases.iter().enumerate() {
        //     self.builder.position_at_end(*block);
        //     let index_to_access = self.enum_packing_index_by_name.get(enum_name).unwrap()[index];
        //     let enum_data = self
        //         .builder
        //         .build_extract_value(enum_to_match, index_to_access as u32,
        // &format!("enum_data{}", invocation_nb))         .unwrap();
        //     self.variables.insert(invocation.branches[index].results[0].id, enum_data);
        //     match invocation.branches[index].target {
        //         GenBranchTarget::Fallthrough => self.process_statements_from(func, invocation_nb
        // + 1, scope)?,         GenBranchTarget::Statement(StatementIdx(id)) => {
        //             println!("Processing jump");
        //             self.jump(func, id, scope)
        //         }
        //     }
        // }

        // Ok(())

        // self.builder.build_switch(value, else_block, cases);
        // println!("\n\n\n\n");
        // todo!("enum match");

        // // The felt to check.
        // let lhs = self.variables.get(&invocation.args[0].id).expect("Variable should exist");
        // // felt == 0
        // let comparison = self.builder.build_int_compare(
        //     EQ,
        //     lhs.into_int_value(),
        //     self.types_by_name.get("felt").unwrap().into_int_type().const_int(0, false),
        //     "check",
        // );
        // // if then
        // let then_bb = self.context.append_basic_block(func, "then");
        // // else
        // let else_bb = self.context.append_basic_block(func, "else");

        // // if felt == 0 {} else {}
        // self.builder.build_conditional_branch(comparison, then_bb, else_bb);

        // self.builder.position_at_end(then_bb);
        // // Check the two branches
        // match invocation.branches[0].target {
        //     // if then is fallthrough
        //     GenBranchTarget::Fallthrough => {
        //         self.process_statements_from(func, invocation_nb + 1, scope)?;
        //     }
        //     // then branch is a jump so we process from the jump until a return instruction.
        //     GenBranchTarget::Statement(StatementIdx(id)) => self.jump(func, id, scope),
        // };

        // self.builder.position_at_end(else_bb);
        // match invocation.branches[1].target {
        //     // else is fallthrough
        //     GenBranchTarget::Fallthrough => {
        //         self.process_statements_from(func, invocation_nb + 1, scope)?;
        //     }
        //     // else branch is a jump so we process from the jump until a return instruction.
        //     GenBranchTarget::Statement(StatementIdx(id)) => self.jump(func, id, scope),
        // };
        // Ok(())
    }
}
