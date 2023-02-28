use cairo_lang_sierra::ids::VarId;
/// This file contains everything related to sierra statement processing.
use cairo_lang_sierra::program::{GenBranchTarget, GenStatement, Invocation};
use inkwell::values::{BasicMetadataValueEnum, StructValue};
use tracing::debug;

use crate::sierra::errors::{CompilerResult, DEBUG_NAME_EXPECTED};
use crate::sierra::llvm_compiler::Compiler;
use crate::sierra::process::corelib::PRINT_RETURN;

/// Implementation of the statement processing for the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Process statements in the Sierra program.
    ///
    /// # Errors
    ///
    /// If the processing of the sierra statements fails.
    pub fn process_statements_from(&mut self, from: usize) -> CompilerResult<()> {
        debug!("processing statements");
        // Check that the current state is valid.
        for (mut statement_id, statement) in self.program.statements.iter().skip(from).enumerate() {
            // Set the statement number to the absolute statement number.
            statement_id += from;
            match statement {
                // If the statement is a sierra function call.
                GenStatement::Invocation(invocation) => {
                    // Get core lib function called by this instruction.
                    let fn_name = invocation.libfunc_id.debug_name.clone().expect(DEBUG_NAME_EXPECTED).to_string();
                    debug!(fn_name, "processing statement: invocation");
                    // Function has only one branch and doesn't return anything.
                    if invocation.branches.len() == 1 && invocation.branches[0].results.is_empty() {
                        match fn_name.as_str() {
                            // Jump needs to be treated.
                            "jump" => {
                                let to = match &invocation.branches[0].target {
                                    GenBranchTarget::Statement(id) => id.0,
                                    _ => panic!("Jump should have genbranchinfo"),
                                };
                                self.jump(to);
                                break;
                            }
                            // Sierra functions have no side effect so we can ignore the function if it doesn't return
                            // anything and it's not a jump
                            _ => continue,
                        }
                    }
                    // Function that have multiple branches and require conditional branches.
                    if invocation.branches.len() > 1 {
                        match fn_name.as_str() {
                            "felt_is_zero" => {
                                self.felt_is_zero(invocation, statement_id)?;
                                // In the felt_is_zero func we process the other statements so we have to break not to
                                // duplicate everything.
                                break;
                            }
                            _ => continue,
                        }
                    }
                    // The instruction has 1 branch and the branch is just the flow of the instructions.
                    // felt_const<2>() -> ([0]);
                    // felt_const<4>() -> ([1]);
                    // Those 2 instructions have only 1 branch and the target is fallthrough (which means next
                    // instruction).
                    if invocation.branches.len() == 1 && invocation.branches[0].target == GenBranchTarget::Fallthrough {
                        let function = if invocation.libfunc_id.debug_name.clone().unwrap().starts_with("function_call")
                        {
                            // Case where the invocation is a function call (it's probably not possible to process it
                            // before the statements because it needs the called function to
                            // be defined to call it).
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
                            // Regular corelib called.
                            self.module
                                .get_function(fn_name.as_str())
                                .unwrap_or_else(|| panic!("{fn_name} function is missing"))
                        };
                        // Get the arguments for the function call.
                        let args = self.process_args(invocation);
                        // Call the function.
                        let res = self
                            .builder
                            .build_call(function, &args, "")
                            .try_as_basic_value()
                            .left()
                            .expect("Call should have worked");
                        if res.is_struct_value()
                            && res.into_struct_value().get_type().count_fields() > 0
                            && !fn_name.starts_with("store_temp")
                            && !fn_name.starts_with("struct_construct")
                        {
                            self.unpack_tuple(&invocation.branches[0].results, res.into_struct_value())
                        } else {
                            // Just save the result.
                            self.variables.insert(invocation.branches[0].results[0].id.to_string(), res);
                        }
                        // If the next instruction is a destination of a jump.
                        if self.jump_dests.contains(&(statement_id + 1)) {
                            // Get the current function.
                            let curr_func = self.module.get_last_function().unwrap();
                            // Add a new basic block.
                            let basic_block = self.context.append_basic_block(curr_func, "dest");
                            // Save the new basic block.
                            self.basic_blocks.insert(statement_id + 1, basic_block);
                            // Branch unconditionally to this block (equivalent of jump)
                            self.builder.build_unconditional_branch(basic_block);
                            self.builder.position_at_end(basic_block);
                        }
                    }
                }
                GenStatement::Return(ret) => {
                    let func_name = self
                        .module
                        .get_last_function()
                        .expect("Current function should have been declared")
                        .get_name()
                        .to_str()
                        .unwrap()
                        .to_string();

                    debug!(func_name, "processing statement: return");
                    // If there is actually something to return.
                    if !ret.is_empty() {
                        let mut types = vec![];
                        let mut values = vec![];

                        // Get the types and values to return.
                        for ret_var in ret.iter() {
                            let value = self.variables.get(&ret_var.id.to_string()).unwrap();
                            values.push(value);
                            types.push(value.get_type());
                        }
                        // Create a struct to simulate a tuple.
                        // Ex:
                        // fn foo() -> (felt, felt, felt)
                        // Would be translated to
                        // define { i253, i253, i253 } @foo()
                        //
                        // but fn foo() -> felt
                        // define i253 @foo()

                        // If the function is the main function.
                        let return_value = if func_name == "main" {
                            let return_struct_type = self.context.struct_type(&types, false);
                            // Allocate a pointer for the return struct.
                            let return_struct_ptr = self.builder.build_alloca(return_struct_type, "ret_struct_ptr");
                            // Save each variable to return in the struct.
                            for (index, value) in values.iter().enumerate() {
                                let tuple_ptr = self
                                    .builder
                                    .build_struct_gep(
                                        return_struct_type,
                                        return_struct_ptr,
                                        index.try_into().unwrap(),
                                        format!("field_{index}_ptr").as_str(),
                                    )
                                    .expect("Pointer should be valid");
                                self.builder.build_store(tuple_ptr, **value);
                            }
                            // Load the values to return in a variable.
                            let mut return_value =
                                self.builder.build_load(return_struct_type, return_struct_ptr, "return_struct_value");

                            // Get the first field of the return type (we'll check that it's not the unit type)
                            let field_ret_type =
                                return_value.into_struct_value().get_type().get_field_type_at_index(0).unwrap();
                            // The unit type is defined like this in our case { {} } which is a struct containing an
                            // empty struct. So above we unpacked the first layer and now we're checking the second
                            // layer.
                            if field_ret_type.is_struct_type() && field_ret_type.into_struct_type().count_fields() == 0
                            {
                                // There's nothing to return we'll just return 0.
                                return_value = self.context.i32_type().const_int(0, false).into();
                            } else {
                                // If there is something to return we print it (to keep the right main signature but
                                // still see what happened).
                                // The return value is always { x }, we need to get x first.
                                let field_value_ptr = self
                                    .builder
                                    .build_struct_gep(return_struct_type, return_struct_ptr, 0, "return_value_ptr")
                                    .unwrap();
                                let field_value =
                                    self.builder.build_load(field_ret_type, field_value_ptr, "return_value");

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
                                return_value = self.context.i32_type().const_int(0, false).into();
                            }
                            return_value
                        }
                        // if its not main, return the value directly if its only 1, otherwise create a struct.
                        else if values.len() == 1 {
                            *values[0]
                        } else {
                            let return_struct_type = self.context.struct_type(&types, false);
                            // Allocate a pointer for the return struct.
                            let return_struct_ptr = self.builder.build_alloca(return_struct_type, "ret_struct_ptr");
                            // Save each variable to return in the struct.
                            for (index, value) in values.iter().enumerate() {
                                let tuple_ptr = self
                                    .builder
                                    .build_struct_gep(
                                        return_struct_type,
                                        return_struct_ptr,
                                        index.try_into().unwrap(),
                                        format!("field_{index}_ptr").as_str(),
                                    )
                                    .expect("Pointer should be valid");
                                self.builder.build_store(tuple_ptr, **value);
                            }
                            // Load the values to return in a variable.
                            let return_value =
                                self.builder.build_load(return_struct_type, return_struct_ptr, "return_struct_value");
                            return_value
                        };
                        // Return the specified value.
                        self.builder.build_return(Some(&return_value));
                    }

                    break;
                }
            }
        }
        Ok(())
    }

    /// Collect the arguments needed to call a function.
    ///
    /// # Arguments
    ///
    /// * `invocation` - The function invocation.
    ///
    /// # Returns
    ///
    /// * `Vec<BasicMetadataValueEnum<'ctx>>` - The vector with the arguments.
    ///
    /// # Errors
    ///
    /// Panics if the argument needed is not found.
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
    fn unpack_tuple(&mut self, results: &[VarId], res: StructValue<'ctx>) {
        let res_type = res.get_type();
        let res_ptr = self.builder.build_alloca(res_type, "res_ptr");
        self.builder.build_store(res_ptr, res);
        for (field_index, VarId { id, debug_name: _ }) in results.iter().enumerate() {
            let id = id.to_string();
            let field_type = res_type.get_field_type_at_index(field_index as u32).expect("Field type should exist");
            let field_ptr = self
                .builder
                .build_struct_gep(res_type, res_ptr, field_index as u32, format!("{id}_ptr").as_str())
                .expect("Pointer should be valid");
            let field = self.builder.build_load(field_type, field_ptr, id.as_str());
            self.variables.insert(id, field);
        }
    }
}
