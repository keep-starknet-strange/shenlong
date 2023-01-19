//! # LLVM compiler.
//! ## Description
//! This module contains the compiler that compiles a Sierra program to LLVM IR.
//! The compilation can be triggered by calling different functions, depending on the input source.
//! The input source can be a Sierra program file path, a Sierra program code as a string, or a
//! Sierra program as a `Program` object. If the Sierra program is compiled from a file, the file
//! will be read and the program code will be extracted. The program code will then be parsed into a
//! `Program` object. The `Program` object will then be compiled to LLVM IR and written to a file.
//! Here are the 3 functions that can be used to trigger the compilation:
//! * `compile_from_file`: When the input source is a Sierra program file path.
//! * `compile_from_code`: When the input source is a Sierra program code as a string.
//! * `compile_sierra_program_to_llvm`: When the input source is a Sierra program as a `Program`
//!   object.
//! ## Compilation steps
//! The compilation process is split into multiple steps.
//! The steps are executed in the following order:
//! 1. Process the types.
//! 2. Process the core library functions.
//! 3. Process the statements.
//! 4. Finalize the compilation.
//! ## State machine
//! The compiler is implemented as a state machine.
//! The state machine is implemented using a `HashMap` that maps a `CompilationStateTransition` to a
//! `bool`. The `CompilationStateTransition` is a tuple of two `CompilationState`s.
//! The first state is the current state.
//! The second state is the next state.
//! The `bool` indicates whether the transition is valid.
//! The state machine is initialized with all valid transitions.
//! The state machine is updated after each compilation step.
//! The state machine is used to ensure that the compilation steps are executed in the correct
//! order. The state machine is also used to ensure that the compilation steps are executed only
//! once.
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::Path;

use cairo_lang_sierra::program::Program;
use cairo_lang_sierra::ProgramParser;
use eyre::{eyre, Result};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicType;
use inkwell::values::PointerValue;
use log::debug;

use super::libfunc::{Add, LibfuncProcessor};

/// Compiler is the main entry point for the LLVM backend.
/// It is responsible for compiling a Sierra program to LLVM IR.
pub struct Compiler<'a, 'ctx> {
    /// The Sierra program.
    pub program: &'a Program,
    /// The LLVM context.
    pub context: &'ctx Context,
    /// The LLVM builder.
    pub builder: &'a Builder<'ctx>,
    /// The LLVM module.
    pub module: &'a Module<'ctx>,
    /// The variables of the program.
    pub variables: HashMap<String, Option<PointerValue<'ctx>>>,
    /// The output path.
    pub output_path: &'a str,
    /// The current compilation state.
    pub state: CompilationState,
    /// The valid state transitions.
    pub valid_state_transitions: HashMap<CompilationStateTransition, bool>,
    /// The types.
    pub types: HashMap<&'ctx str, Box<dyn BasicType<'ctx> + 'ctx>>,
    /// The library functions processors. Each processor is responsible for processing a specific
    /// libfunc and generating the corresponding LLVM IR.
    pub libfunc_processors: HashMap<&'ctx str, Box<dyn LibfuncProcessor<'a, 'ctx> + 'ctx>>,
}

/// Compilation state.
/// This is used to keep track of the current compilation state.
/// The reason is that the compilation process is split into multiple steps.
/// The state will be used to implement a state machine that will keep track of the current
/// compilation step. Goal is to ensure consistency in the order of the compilation steps.
/// This is important because the compilation steps are not independent.
/// For example, the type declarations must be processed before the statements.
/// The state machine will ensure that the compilation steps are executed in the correct order.
/// The state machine will also ensure that the compilation steps are executed only once.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum CompilationState {
    /// The compilation has not started yet.
    NotStarted,
    /// The types have been processed.
    TypesProcessed,
    /// The core library functions have been processed.
    CoreLibFunctionsProcessed,
    /// The statements have been processed.
    StatementsProcessed,
    /// The compilation has been finalized.
    /// This is the final state.
    /// The compilation process will not continue after this state.
    Finalized,
}

/// A compilation state transition.
/// This is a tuple of two compilation states.
/// The first state is the current state.
/// The second state is the next state.
pub type CompilationStateTransition = (CompilationState, CompilationState);

/// Implementation of the compiler.
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Compile a Sierra program file to LLVM IR.
    /// # Arguments
    /// * `program_path` - The Sierra program to compile.
    /// * `output_path` - The path to the output LLVM IR file.
    /// # Returns
    /// The result of the compilation.
    /// # Errors
    /// If the compilation fails.
    pub fn compile_from_file(program_path: &str, output_path: &str) -> Result<()> {
        // Read the program from the file.
        let sierra_code = fs::read_to_string(program_path)?;
        Compiler::compile_from_code(&sierra_code, output_path)
    }

    /// Compile a Sierra program code to LLVM IR.
    /// # Arguments
    /// * `sierra_code` - The Sierra program code.
    /// * `output_path` - The path to the output LLVM IR file.
    /// # Returns
    /// The result of the compilation.
    /// # Errors
    /// If the compilation fails.
    pub fn compile_from_code(sierra_code: &str, output_path: &str) -> Result<()> {
        // Parse the program.
        let program = ProgramParser::new().parse(sierra_code).unwrap();
        Compiler::compile_sierra_program_to_llvm(program, output_path)
    }

    /// Compiles a Sierra `Program` representation to LLVM IR.
    /// # Process overview
    /// 1. Create an LLVM context, builder and module.
    /// 2. Instantiate variables map.
    /// 3. Process the program type declarations.
    /// 4. Process the core library functions.
    /// 5. Process the program statements.
    /// 6. Finalize compilation and write the LLVM IR to a file.
    /// # Arguments
    /// * `program` - The Sierra program to compile.
    /// * `output_path` - The path to the output LLVM IR file.
    /// # Returns
    /// The result of the compilation.
    /// # Errors
    /// If the compilation fails.
    /// # Example
    /// ```rust
    /// use std::fs;
    ///
    /// use shenlong_core::sierra::llvm_compiler::Compiler;
    /// use sierra::ProgramParser;
    ///
    /// let sierra_program_path = "examples/program.sierra";
    /// let llvm_ir_path = "examples/program.ll";
    ///
    /// // TODO: Find a way to make doc tests pass.
    /// // Read the program from the file.
    /// let sierra_code = fs::read_to_string(sierra_program_path).unwrap();
    /// // Parse the program.
    /// let program = ProgramParser::new().parse(&sierra_code).unwrap();
    /// // Compile the program to LLVM IR.
    /// let result = Compiler::compile_from_file(sierra_program_path, llvm_ir_path);
    /// // Check the result.
    /// ```
    pub fn compile_sierra_program_to_llvm(program: Program, output_path: &str) -> Result<()> {
        // Create an LLVM context, builder and module.
        // See https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl03.html#id2
        // Context is an opaque object that owns a lot of core LLVM data structures, such as the
        // type and constant value tables
        let context = inkwell::context::Context::create();
        // Builder is a helper object that makes it easy to create LLVM instructions.
        let builder = context.create_builder();
        // Module is an object that contains all of the functions, global variables.
        // In many ways, it is the top-level structure that the LLVM IR uses to contain code.
        let module = context.create_module("root");

        // Instantiate variables map.
        let variables: HashMap<String, Option<PointerValue>> = HashMap::new();
        let types = HashMap::new();
        let libfunc_processors = HashMap::new();

        // Create a map of valid state transitions.
        let valid_state_transitions = Compiler::init_state_transitions();

        // Create a new compiler.
        let mut compiler = Compiler {
            program: &program,
            context: &context,
            builder: &builder,
            module: &module,
            variables,
            output_path,
            state: CompilationState::NotStarted,
            valid_state_transitions,
            types,
            libfunc_processors,
        };

        // Process the types in the Sierra program.
        compiler.process_types()?;

        // Prepare the libfunc processors.
        compiler.prepare_libfunc_processors()?;

        // Process the core library functions in the Sierra program.
        compiler.process_core_lib_functions()?;

        // Process the statements in the Sierra program.
        compiler.process_statements()?;

        // Finalize the compilation.
        compiler.finalize_compilation()
    }

    /// Prepare the libfunc processors.
    fn prepare_libfunc_processors(&mut self) -> Result<()> {
        // Add two felts and return the result.
        self.libfunc_processors.insert("felt_add", Box::from(Add {}));
        Ok(())
    }

    /// Process types in the Sierra program.
    /// For each type declaration in the Sierra program, create a corresponding type in the LLVM
    /// context.
    fn process_types(&mut self) -> Result<()> {
        debug!("processing types");

        // Check that the current state is valid.
        self.check_state(&CompilationState::NotStarted)?;
        for type_declaration in self.program.type_declarations.iter() {
            // Matching on the long id because it'll always ? have a debug name (see
            // fib.weird.sierra)
            match &type_declaration.long_id.generic_id.debug_name {
                Some(type_name) => match type_name.as_str() {
                    "felt" => {
                        self.types
                            .insert("felt", Box::from(self.context.custom_width_int_type(252)));
                    }
                    "NonZero" => (),
                    _ => println!("this is not a felt"),
                },
                _ => return Err(eyre!("No type name found")),
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::TypesProcessed)
    }

    /// Process core library functions in the Sierra program.
    fn process_core_lib_functions(&mut self) -> Result<()> {
        debug!("processing core lib functions");
        // Check that the current state is valid.
        self.check_state(&CompilationState::TypesProcessed)?;
        // Iterate over the libfunc declarations in the Sierra program.
        for libfunc_declaration in self.program.libfunc_declarations.iter() {
            match &libfunc_declaration.long_id.generic_id.debug_name {
                Some(libfunc) => {
                    if let Some(processor) = self.libfunc_processors.get(libfunc.as_str()) {
                        let felt_type = self.types.get("felt").unwrap();

                        processor.to_llvm(
                            libfunc.as_str(),
                            felt_type.as_basic_type_enum(),
                            vec![felt_type, felt_type],
                            self.module,
                            self.context,
                            self.builder,
                        )?;
                    }
                }
                None => (),
            }
        }
        // Move to the next state.
        self.move_to(CompilationState::CoreLibFunctionsProcessed)
    }

    /// Process statements in the Sierra program.
    fn process_statements(&mut self) -> Result<()> {
        debug!("processing statements");
        // Check that the current state is valid.
        self.check_state(&CompilationState::CoreLibFunctionsProcessed)?;
        // This section is very specific to the test program.
        // TODO: Think about how to implement this in a more general way.
        // Init param var
        // let i32_type = self.context.i32_type();
        // let main_type = i32_type.fn_type(&[self.context.i32_type().into()], false);
        // let main_func = self.module.add_function("main", main_type, None);
        // let main_bb = self.context.append_basic_block(main_func, "entry");
        // self.builder.position_at_end(main_bb);
        // for func in self.program.funcs.iter() {
        //     for param in func.params.iter() {
        //         self.variables.insert(param.id.id.to_string(), None);
        //     }
        // }

        // // Iterate over the statements in the Sierra program, but do nothing with them.
        // for (statement_id, _statement) in self.program.statements.iter().enumerate() {
        //     let _statement_idx = StatementIdx(statement_id);
        //     match _statement {
        //         // If the statement is an invocation, print the invocation.
        //         Statement::Invocation(invocation) => {
        //             if invocation.libfunc_id.id.to_string().as_str() != "3" {
        //                 let function = self
        //                     .module
        //                     .get_function(format!("a_{}", invocation.libfunc_id.id).as_str())
        //                     .ok_or_else(|| eyre::eyre!("function not found"))?;
        //                 let mut args: Vec<BasicMetadataValueEnum> = vec![];
        //                 invocation.args.clone().into_iter().for_each(|var_id| {
        //                     args.push(
        //                         match self.variables.get(var_id.id.to_string().as_str()).unwrap()
        // {                             Some(val) => self
        //                                 .builder
        //                                 .build_load(*val, var_id.id.to_string().as_str())
        //                                 .into(),
        //                             None => {
        //                                 main_func.get_nth_param(var_id.id as u32).unwrap().into()
        //                             }
        //                         },
        //                     );
        //                 });
        //                 let res_ptr = self.builder.build_alloca(
        //                     i32_type,
        //                     format!("ptr{}", invocation.branches[0].results[0].id).as_str(),
        //                 );
        //                 let res_val = self
        //                     .builder
        //                     .build_call(
        //                         function,
        //                         &args,
        //                         format!("val{}", invocation.branches[0].results[0].id).as_str(),
        //                     )
        //                     .try_as_basic_value()
        //                     .left()
        //                     .unwrap();
        //                 self.builder.build_store(res_ptr, res_val);
        //                 self.variables.insert(
        //                     invocation.branches[0].results[0].id.to_string(),
        //                     Some(res_ptr),
        //                 );
        //             }
        //         }
        //         // If the statement is a return, print the return.
        //         Statement::Return(ret) => {
        //             self.builder.build_return(Some(&self.builder.build_load(
        //                 self.variables.get(ret[0].id.to_string().as_str()).unwrap().unwrap(),
        //                 ret[0].id.to_string().as_str(),
        //             )));
        //         }
        //     }
        // }
        // Move to the next state.
        self.move_to(CompilationState::StatementsProcessed)
    }

    /// Finalize the compilation.
    /// This includes verifying the module and writing it to the output path.
    fn finalize_compilation(&mut self) -> Result<()> {
        debug!("finalizing compilation");
        // Check that the current state is valid.
        self.check_state(&CompilationState::StatementsProcessed)?;
        // Ensure that the current module is valid
        // self.module.verify().map_err(|e| eyre::eyre!(e.to_string()))?;
        // Ensure output path is valid and exists.
        let output_path = Path::new(self.output_path);
        // let parent =
        //     output_path.parent().ok_or_else(|| eyre::eyre!("parent output path is not valid"))?;
        // // Recursively create the output path parent directories if they don't exist.
        // fs::create_dir_all(parent)?;
        // // Write the module to the output path.
        self.build_main()?;
        self.module.print_to_file(output_path).map_err(|e| eyre::eyre!(e.to_string()))?;

        println!("{:?}", self.module.print_to_string());
        // Move to the next state.
        self.move_to(CompilationState::Finalized)
    }
    fn build_main(&mut self) -> Result<()> {
        let args_type = self.context.custom_width_int_type(252);
        let main_type = args_type.fn_type(&[], false);
        let main_func = self.module.add_function("main", main_type, None);
        let main_bb = self.context.append_basic_block(main_func, "entry");
        self.builder.position_at_end(main_bb);
        let function = self.module.get_function("felt_add").ok_or(eyre!("Function not found"))?;
        let res = self
            .builder
            .build_call(
                function,
                &[
                    inkwell::values::BasicMetadataValueEnum::IntValue(
                        args_type.const_int(42, false),
                    ),
                    inkwell::values::BasicMetadataValueEnum::IntValue(
                        args_type.const_int(3, false),
                    ),
                ],
                "call_felt_add",
            )
            .try_as_basic_value()
            .left()
            .ok_or(eyre!("No return value"))?;
        self.builder.build_return(Some(&res));

        Ok(())
    }

    /// Check if the compilation is in a valid state.
    /// If the compilation is not in a valid state, return an error.
    fn check_state(&self, expected_state: &CompilationState) -> Result<()> {
        if self.state() != expected_state {
            Err(eyre::eyre!("compilation is not in a valid state"))
        } else {
            Ok(())
        }
    }

    /// Move the compilation state to the next state.
    /// # Arguments
    /// * `state` - The new compilation state.
    /// # Errors
    /// If the transition is not valid, return an error.
    fn move_to(&mut self, state: CompilationState) -> Result<()> {
        Compiler::is_valid_transition(
            (self.state().clone(), state.clone()),
            &self.valid_state_transitions,
        )?;
        self.state = state;
        Ok(())
    }

    /// Return if the state transition is valid.
    /// # Arguments
    /// * `transition` - The state transition.
    /// * `valid_transitions` - The valid state transitions.
    /// # Errors
    /// If the transition is not valid, return an error.
    fn is_valid_transition(
        transition: CompilationStateTransition,
        valid_transitions: &HashMap<(CompilationState, CompilationState), bool>,
    ) -> Result<()> {
        match valid_transitions.get(&transition) {
            Some(valid) => match valid {
                true => Ok(()),
                false => Err(Compiler::err_invalid_state_transition(transition)),
            },
            None => Err(Compiler::err_invalid_state_transition(transition)),
        }
    }

    /// Get the current compilation state.
    pub fn state(&self) -> &CompilationState {
        &self.state
    }

    /// Initialize valid state transitions.
    fn init_state_transitions() -> HashMap<(CompilationState, CompilationState), bool> {
        HashMap::from([
            ((CompilationState::NotStarted, CompilationState::TypesProcessed), true),
            ((CompilationState::TypesProcessed, CompilationState::CoreLibFunctionsProcessed), true),
            (
                (
                    CompilationState::CoreLibFunctionsProcessed,
                    CompilationState::StatementsProcessed,
                ),
                true,
            ),
            ((CompilationState::StatementsProcessed, CompilationState::Finalized), true),
        ])
    }

    /// Return an error for an invalid state transition.
    /// # Arguments
    /// * `invalid_transition` - The invalid state transition.
    /// # Errors
    /// Always returns an error.
    fn err_invalid_state_transition(
        invalid_transition: CompilationStateTransition,
    ) -> eyre::Report {
        eyre::eyre!(
            "invalid state transition: {:?} -> {:?}",
            invalid_transition.0,
            invalid_transition.1
        )
    }
}
