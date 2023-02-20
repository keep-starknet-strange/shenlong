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
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};

use cairo_lang_sierra::program::Program;
use cairo_lang_sierra::ProgramParser;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;
use inkwell::types::BasicType;
use inkwell::values::BasicValueEnum;
use tracing::{debug, info};

use super::errors::CompilerResult;
use crate::sierra::errors::CompilerError;

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
    pub module: Module<'ctx>,
    /// The variables of the program.
    pub variables: HashMap<String, BasicValueEnum<'ctx>>,
    /// The LLVM IR output path.
    pub llvm_output_path: PathBuf,
    /// The bitcode output path.
    pub bc_output_path: PathBuf,
    /// The current compilation state.
    pub state: CompilationState,
    /// The valid state transitions.
    pub valid_state_transitions: HashMap<CompilationStateTransition, bool>,
    /// The types.
    pub types: HashMap<String, Box<dyn BasicType<'ctx> + 'a>>,
    /// Mapping from type name to program id.
    pub id_from_name: HashMap<String, String>,
    /// Calls in the main function.
    pub basic_blocks: HashMap<usize, BasicBlock<'ctx>>,
    pub jump_dests: HashSet<usize>,
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
    /// The functions have been processed.
    FunctionsProcessed,
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
    ///
    /// # Arguments
    ///
    /// * `program_path` - The Sierra program to compile.
    /// * `llvm_output_path` - The path to the output LLVM IR file.
    /// * `bc_output_path` - The path to the output bitcode file.
    ///
    /// # Returns
    ///
    /// The result of the compilation.
    ///
    /// # Errors
    ///
    /// If the compilation fails.
    pub fn compile_from_file(
        program_path: &Path,
        llvm_output_path: &Path,
        bc_output_path: &Path,
        target_triple: Option<&str>,
    ) -> CompilerResult<()> {
        // Read the program from the file.
        let sierra_code = fs::read_to_string(program_path)?;
        Compiler::compile_from_code(&sierra_code, llvm_output_path, bc_output_path, target_triple)
    }

    /// Compile a Sierra program code to LLVM IR.
    /// # Arguments
    ///
    /// * `sierra_code` - The Sierra program code.
    /// * `llvm_output_path` - The path to the output LLVM IR file.
    /// * `bc_output_path` - The path to the output bitcode file.
    ///
    /// # Returns
    ///
    /// The result of the compilation.
    ///
    /// # Errors
    ///
    /// If the compilation fails.
    pub fn compile_from_code(
        sierra_code: &str,
        llvm_output_path: &Path,
        bc_output_path: &Path,
        target_triple: Option<&str>,
    ) -> CompilerResult<()> {
        // Parse the program.
        let program = ProgramParser::new().parse(sierra_code).unwrap();
        Compiler::compile_sierra_program_to_llvm(program, llvm_output_path, bc_output_path, target_triple)
    }

    /// Compiles a Sierra `Program` representation to LLVM IR.
    /// # Process overview
    ///
    /// 1. Create an LLVM context, builder and module.
    /// 2. Instantiate variables map.
    /// 3. Process the program type declarations.
    /// 4. Process the core library functions.
    /// 5. Process the program statements.
    /// 6. Finalize compilation and write the LLVM IR to a file.
    ///
    /// # Arguments
    ///
    /// * `program` - The Sierra program to compile.
    /// * `output_path` - The path to the output LLVM IR file.
    /// * `bc_output_path` - The path to the output bitcode file.
    ///
    /// # Returns
    ///
    /// The result of the compilation.
    ///
    /// # Errors
    ///
    /// If the compilation fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// use cairo_lang_sierra::ProgramParser;
    /// use shenlong_core::sierra::llvm_compiler::Compiler;
    ///
    /// let sierra_program_path = Path::new("../examples/program.sierra");
    /// let llvm_ir_path = Path::new("../examples/program.ll");
    /// let bitcode_path = Path::new("../examples/program.bc");
    ///
    /// // TODO: Find a way to make doc tests pass.
    /// // Read the program from the file.
    /// let sierra_code = fs::read_to_string(sierra_program_path).unwrap();
    /// // Parse the program.
    /// let program = ProgramParser::new().parse(&sierra_code).unwrap();
    /// // Compile the program to LLVM IR.
    /// let result =
    ///     Compiler::compile_from_file(&sierra_program_path, &llvm_ir_path, &bitcode_path, None);
    /// // Check the result.
    /// ```
    pub fn compile_sierra_program_to_llvm(
        program: Program,
        llvm_output_path: &Path,
        bc_output_path: &Path,
        target_triple: Option<&str>,
    ) -> CompilerResult<()> {
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

        if let Some(target_triple) = target_triple {
            module.set_triple(&TargetTriple::create(target_triple));
        }

        info!("Target triple: {}", module.get_triple());

        // Instantiate variables map.
        let variables = HashMap::new();
        let types = HashMap::new();
        let id_from_name = HashMap::new();
        let basic_blocks = HashMap::new();
        let jump_dests = HashSet::new();

        // Create a map of valid state transitions.
        let valid_state_transitions = Compiler::init_state_transitions();

        // Create a new compiler.
        let mut compiler = Compiler {
            program: &program,
            context: &context,
            builder: &builder,
            module,
            variables,
            llvm_output_path: llvm_output_path.to_owned(),
            bc_output_path: bc_output_path.to_owned(),
            state: CompilationState::NotStarted,
            valid_state_transitions,
            types,
            id_from_name,
            basic_blocks,
            jump_dests,
        };

        // Process the types in the Sierra program.
        compiler.process_types()?;

        // Process the core library functions in the Sierra program.
        compiler.process_core_lib_functions()?;
        compiler.collect_jumps();
        // Process the functions in the Sierra program.
        compiler.process_funcs()?;
        // Process the statements in the Sierra program.
        // compiler.process_statements()?;

        // Finalize the compilation.
        compiler.finalize_compilation()
    }

    /// Finalize the compilation.
    /// This includes verifying the module and writing it to the output path.
    ///
    /// # Errors
    ///
    /// If the compiler is not in the right state or if there is a problem in the LLVM IR file.
    fn finalize_compilation(&mut self) -> CompilerResult<()> {
        debug!("finalizing compilation");
        // Check that the current state is valid.
        self.check_state(&CompilationState::FunctionsProcessed)?;
        // let parent =
        //     output_path.parent().ok_or_else(|| eyre::eyre!("parent output path is not valid"))?;
        // // Recursively create the output path parent directories if they don't exist.
        // fs::create_dir_all(parent)?;
        // // Write the module to the output path.
        self.module.print_to_file(&self.llvm_output_path)?;
        assert!(self.module.write_bitcode_to_path(&self.bc_output_path), "Failed to write bitcode");
        // Ensure that the current module is valid
        self.module.verify()?;

        // Move to the next state.
        self.move_to(CompilationState::Finalized)
    }

    /// Check if the compilation is in a valid state.
    /// If the compilation is not in a valid state, return an error.
    ///
    /// # Arguments
    ///
    /// * `expected_state` - State the compiler should be in.
    ///
    /// # Returns
    ///
    /// Result if the state is in an existing state.
    ///
    /// # Errors
    ///
    /// If the compiler is in a state that doesn't exist
    #[inline]
    pub fn check_state(&self, expected_state: &CompilationState) -> CompilerResult<()> {
        if self.state() != expected_state { Err(CompilerError::InvalidState) } else { Ok(()) }
    }

    /// Move the compilation state to the next state.
    /// # Arguments
    /// * `state` - The new compilation state.
    /// # Errors
    /// If the transition is not valid, return an error.
    pub fn move_to(&mut self, state: CompilationState) -> CompilerResult<()> {
        Compiler::is_valid_transition((self.state().clone(), state.clone()), &self.valid_state_transitions)?;
        self.state = state;
        Ok(())
    }

    /// Return if the state transition is valid.
    ///
    /// # Arguments
    ///
    /// * `transition` - The state transition.
    /// * `valid_transitions` - The valid state transitions.
    ///
    /// # Errors
    ///
    /// If the transition is not valid, return an error.
    #[inline]
    fn is_valid_transition(
        transition: CompilationStateTransition,
        valid_transitions: &HashMap<(CompilationState, CompilationState), bool>,
    ) -> CompilerResult<()> {
        match valid_transitions.get(&transition) {
            Some(valid) => match valid {
                true => Ok(()),
                false => Err(Compiler::err_invalid_state_transition(transition)),
            },
            None => Err(Compiler::err_invalid_state_transition(transition)),
        }
    }

    /// Get the current compilation state.
    #[inline(always)]
    pub fn state(&self) -> &CompilationState {
        &self.state
    }

    /// Initialize valid state transitions.
    pub fn init_state_transitions() -> HashMap<(CompilationState, CompilationState), bool> {
        HashMap::from([
            ((CompilationState::NotStarted, CompilationState::TypesProcessed), true),
            ((CompilationState::TypesProcessed, CompilationState::CoreLibFunctionsProcessed), true),
            ((CompilationState::CoreLibFunctionsProcessed, CompilationState::FunctionsProcessed), true),
            ((CompilationState::FunctionsProcessed, CompilationState::Finalized), true),
        ])
    }

    /// Return an error for an invalid state transition.
    ///
    /// # Arguments
    ///
    /// * `invalid_transition` - The invalid state transition.
    ///
    /// # Errors
    ///
    /// Always returns an error.
    #[inline(always)]
    fn err_invalid_state_transition(invalid_transition: CompilationStateTransition) -> CompilerError {
        CompilerError::InvalidStateTransition(invalid_transition.0, invalid_transition.1)
    }
}
