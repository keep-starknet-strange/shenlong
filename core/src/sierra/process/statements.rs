use std::cmp::Ordering;

/// This file contains everything related to sierra statement processing.
use cairo_lang_sierra::program::GenStatement;
use tracing::debug;

use crate::sierra::errors::DEBUG_NAME_EXPECTED;
use crate::sierra::llvm_compiler::{Compiler, FunctionInfo};
use crate::sierra::process::corelib::PRINT_RETURN;

/// Implementation of the statement processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process statements in the Sierra program.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra statements fails.

    pub fn process_statements(&mut self) {
        let debug_line_for_first_statement = self.debug.current_line;

        // For simplicity's sake we process one function at a time. The algorithm would work the same if we
        // took all basic blocks with no predecessors as our starting point, but this aids debugging
        for (user_func_name, FunctionInfo { func: user_func, entry_point, args: _, debug: _ }) in
            self.user_functions.clone().iter()
        {
            let processing_order = self.dataflow_graph.get_ordered_reachable_blocks_from(*entry_point);

            // Point the debug compiler at the debug scope for this function
            let function_debug_info = self
                .debug
                .functions
                .get(user_func_name)
                .expect("Debug information should have been registered for function");
            self.debug.debug_location(Some(function_debug_info.scope));

            for (block_start, block_end) in processing_order {
                // First, position the writer at the block we're processing
                self.builder.position_at_end(self.dataflow_graph.get_block_for_entrypoint(block_start).unwrap());

                // Add any phi instructions at the start of the block
                // These unify data from different control flows into a single variable
                self.dataflow_graph.write_phis(block_start, self.builder);

                // Process each statement from the block
                for statement_idx in block_start..block_end {
                    // Align the debugger to the current statement
                    self.debug.current_line = debug_line_for_first_statement + statement_idx as u32;

                    match &self.program.statements[statement_idx] {
                        GenStatement::Invocation(invocation) => {
                            let lib_fn_name =
                                invocation.libfunc_id.debug_name.as_ref().expect(DEBUG_NAME_EXPECTED).to_string();
                            match invocation.branches.len().cmp(&1) {
                                // A standard instruction with one followup
                                Ordering::Equal => {
                                    let args: Vec<_> = match &self.program.statements[statement_idx] {
                                        GenStatement::Invocation(invocation) => &invocation.args,
                                        GenStatement::Return(ret_args) => ret_args,
                                    }
                                    .iter()
                                    .map(|arg| self.dataflow_graph.use_variable_at_statement(statement_idx, arg).into())
                                    .collect();

                                    // Since sierra functions have no side effects, we only need to actually call the
                                    // function if it has results
                                    if !invocation.branches[0].results.is_empty() {
                                        // First get the function to call, either a user defined function, or corelib
                                        // one
                                        let lib_func = if lib_fn_name.starts_with("function_call<") {
                                            self.module
                                                .get_function(
                                                    lib_fn_name
                                                        .strip_prefix("function_call<user@")
                                                        .unwrap()
                                                        .strip_suffix('>')
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                        } else {
                                            // Regular corelib called.
                                            self.module
                                                .get_function(lib_fn_name.as_str())
                                                .unwrap_or_else(|| panic!("{lib_fn_name} function is missing"))
                                        };

                                        // TODO check void user functions
                                        let res = self
                                            .builder
                                            .build_call(lib_func, &args, "")
                                            .try_as_basic_value()
                                            .left()
                                            .expect("Call should have worked");

                                        let ret_ids = &invocation.branches[0].results;

                                        if ret_ids.len() == 1 {
                                            self.dataflow_graph.create_variable_at_statement(
                                                statement_idx,
                                                &ret_ids[0],
                                                res,
                                            )
                                        } else {
                                            let res_struct = res.into_struct_value();
                                            for (field_index, ret_id) in ret_ids.iter().enumerate() {
                                                let field_value = self
                                                    .builder
                                                    .build_extract_value(
                                                        res_struct,
                                                        field_index as u32,
                                                        &format!("ret_field_{}_", field_index),
                                                    )
                                                    .unwrap();
                                                self.dataflow_graph.create_variable_at_statement(
                                                    statement_idx,
                                                    ret_id,
                                                    field_value,
                                                );
                                            }
                                        }
                                    }

                                    if statement_idx == block_end - 1 {
                                        self.insert_flow_control_if_necessary(
                                            statement_idx,
                                            &invocation.branches[0].target,
                                        );
                                    }
                                }
                                // When control flow branches, we need special handling for each case
                                Ordering::Greater => match lib_fn_name.as_str() {
                                    "felt_is_zero" => {
                                        self.felt_is_zero(invocation, statement_idx);
                                    }
                                    x if x.starts_with("enum_match<") => {
                                        self.enum_match(*user_func, invocation, statement_idx);
                                    }
                                    _ => panic!("Unhandled branching function {}", lib_fn_name.as_str()),
                                },
                                Ordering::Less => {
                                    panic!("Non-return instruction should have some number of control flow targets");
                                }
                            }
                        }
                        GenStatement::Return(ret_arg_ids) => {
                            println!("Processing return {statement_idx} for {user_func_name}");

                            debug!(user_func_name, line = self.debug.current_line, "processing statement: return");

                            if ret_arg_ids.is_empty() {
                                self.builder.build_return(None);
                            } else {
                                // We need to extract the llvm function's name here rather than using `user_func_name`
                                // because `user_func_name` is the scoped sierra function name
                                let func_is_main = user_func.get_name().to_str().unwrap() == "main";

                                let ret_args: Vec<_> = ret_arg_ids
                                    .iter()
                                    .map(|id| self.dataflow_graph.use_variable_at_statement(statement_idx, id))
                                    .collect();

                                // if its not main, return the value directly if its only 1, otherwise create a struct
                                if ret_args.len() == 1 && !func_is_main {
                                    // Return the specified value.
                                    self.builder.build_return(Some(&ret_args[0]));
                                } else {
                                    // Create a struct to simulate a tuple.
                                    // Ex:
                                    // fn foo() -> (felt, felt, felt)
                                    // Would be translated to
                                    // define { i253, i253, i253 } @foo()
                                    //
                                    // but fn foo() -> felt
                                    // define i253 @foo()
                                    let return_struct_type = self
                                        .context
                                        .struct_type(&ret_args.iter().map(|x| x.get_type()).collect::<Vec<_>>(), false);
                                    // Allocate a pointer for the return struct.
                                    let return_struct_ptr =
                                        self.builder.build_alloca(return_struct_type, "ret_struct_ptr");
                                    // Save each variable to return in the struct.
                                    for (index, value) in ret_args.iter().enumerate() {
                                        let tuple_ptr = self
                                            .builder
                                            .build_struct_gep(
                                                return_struct_type,
                                                return_struct_ptr,
                                                index as u32,
                                                format!("field_{index}_ptr").as_str(),
                                            )
                                            .expect("Pointer should be valid");
                                        self.builder.build_store(tuple_ptr, *value);
                                    }
                                    // Load the values to return in a variable.
                                    let return_value = self.builder.build_load(
                                        return_struct_type,
                                        return_struct_ptr,
                                        "return_struct_value",
                                    );

                                    if !func_is_main {
                                        // Return the constructed struct.
                                        self.builder.build_return(Some(&return_value));
                                    } else {
                                        // Get the first field of the return type (we'll check that it's not the unit
                                        // type)
                                        let field_ret_type = return_value
                                            .into_struct_value()
                                            .get_type()
                                            .get_field_type_at_index(0)
                                            .unwrap();
                                        // The unit type is defined like this in our case { {} } which is a struct
                                        // containing an empty struct. So above we
                                        // unpacked the first layer and now we're checking the second
                                        // layer.
                                        if field_ret_type.is_struct_type()
                                            && field_ret_type.into_struct_type().count_fields() == 0
                                        {
                                            // There's nothing to return we'll just return 0.
                                            self.builder.build_return(Some(&self.context.i32_type().const_zero()));
                                        } else {
                                            // If there is something to return we print it (to keep the right main
                                            // signature but still see what
                                            // happened). The return value
                                            // is always { x }, we need to get x first.
                                            let field_value_ptr = self
                                                .builder
                                                .build_struct_gep(
                                                    return_struct_type,
                                                    return_struct_ptr,
                                                    0,
                                                    "return_value_ptr",
                                                )
                                                .unwrap();
                                            let field_value = self.builder.build_load(
                                                field_ret_type,
                                                field_value_ptr,
                                                "return_value",
                                            );

                                            // We have a int value, directly print it.
                                            if field_value.is_int_value() {
                                                self.call_printf("Return value: ", &[]);
                                                self.call_print_type(PRINT_RETURN, field_value.into());
                                            }
                                            // x is { y, y1... }, print each field (if they are ints for now).
                                            else if field_value.is_struct_value() {
                                                let field = field_value.into_struct_value();
                                                // Allocate a pointer for the field struct.
                                                let field_struct_ptr =
                                                    self.builder.build_alloca(field.get_type(), "field_struct_ptr");
                                                self.builder.build_store(field_struct_ptr, field);
                                                // Prints the fields of a struct.
                                                for i in 0..field.get_type().count_fields() {
                                                    let f = self
                                                        .builder
                                                        .build_struct_gep(
                                                            field.get_type(),
                                                            field_struct_ptr,
                                                            i,
                                                            &format!("field_struct_{i}_ptr"),
                                                        )
                                                        .unwrap();
                                                    let value = self.builder.build_load(
                                                        field.get_type().get_field_type_at_index(i).unwrap(),
                                                        f,
                                                        &format!("field_struct_{i}"),
                                                    );
                                                    self.call_printf(&format!("Return field {i} value: "), &[]);
                                                    self.call_print_type(PRINT_RETURN, value.into());
                                                }
                                            }
                                            self.builder.build_return(Some(&self.context.i32_type().const_zero()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
