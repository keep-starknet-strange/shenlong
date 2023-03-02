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
use inkwell::debug_info::{DICompileUnit, DILocalVariable, DISubprogram, DIType, DebugInfoBuilder};
use inkwell::module::Module;
use inkwell::targets::TargetTriple;
use inkwell::types::BasicTypeEnum;
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
    /// The variables of the current function.
    pub variables: HashMap<String, BasicValueEnum<'ctx>>,
    /// The LLVM IR output path.
    pub llvm_output_path: PathBuf,
    /// The current compilation state.
    pub state: CompilationState,
    /// The valid state transitions.
    pub valid_state_transitions: HashMap<CompilationStateTransition, bool>,
    /// The types by sierra id.
    pub types_by_id: HashMap<u64, BasicTypeEnum<'ctx>>,
    /// The types by debug name
    pub types_by_name: HashMap<String, BasicTypeEnum<'ctx>>,
    /// Calls in the main function.
    pub basic_blocks: HashMap<usize, BasicBlock<'ctx>>,
    pub jump_dests: HashSet<usize>,
    /// A struct holding all the debug info.
    pub debug: DebugCompiler<'a, 'ctx>,
}

pub struct FunctionDebugInfo<'ctx> {
    pub function: DISubprogram<'ctx>,
    pub params: Vec<DILocalVariable<'ctx>>,
}
/// Struct holding all the data needed to produce the debug info by the compiler.
pub struct DebugCompiler<'a, 'ctx> {
    /// Debug builder
    pub debug_builder: DebugInfoBuilder<'ctx>,
    pub builder: &'a Builder<'ctx>,
    pub compile_unit: DICompileUnit<'ctx>,
    pub types_by_id: HashMap<u64, DIType<'ctx>>,
    pub types_by_name: HashMap<String, DIType<'ctx>>,
    /// The debug info variables of the current function.
    pub variables: HashMap<String, DIType<'ctx>>,
    pub functions: HashMap<String, FunctionDebugInfo<'ctx>>,
    current_line: u32,
    current_statement_line: u32,
    pub context: &'ctx Context,
}

impl<'a, 'ctx> DebugCompiler<'a, 'ctx> {
    pub fn new(
        debug_builder: DebugInfoBuilder<'ctx>,
        builder: &'a Builder<'ctx>,
        compile_unit: DICompileUnit<'ctx>,
        context: &'ctx Context,
    ) -> Self {
        Self {
            debug_builder,
            builder,
            compile_unit,
            types_by_id: HashMap::new(),
            types_by_name: HashMap::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_line: 0,
            current_statement_line: 0,
            context,
        }
    }

    /// Increases the current line by 1.
    pub fn next_line(&mut self) {
        self.current_line += 1;
        self.current_statement_line = self.current_line;
    }

    /// Sets the current statement line from a statement id.
    pub fn set_statement_line(&mut self, statement_id: usize) {
        self.current_statement_line = self.current_line + statement_id as u32;
    }

    // Gets the current line.
    pub fn get_line(&self) -> u32 {
        self.current_statement_line
    }
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
    /// The necessary types for debugging have been created.
    DebugSetup,
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
        target_triple: Option<&str>,
    ) -> CompilerResult<()> {
        // Read the program from the file.
        let sierra_code = fs::read_to_string(program_path)?;
        Compiler::compile_from_code(&sierra_code, program_path, llvm_output_path, target_triple)
    }

    /// Compile a Sierra program code to LLVM IR.
    /// # Arguments
    ///
    /// * `sierra_code` - The Sierra program code.
    /// * `llvm_output_path` - The path to the output LLVM IR file.
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
        program_path: &Path,
        llvm_output_path: &Path,
        target_triple: Option<&str>,
    ) -> CompilerResult<()> {
        // Parse the program.
        let program = ProgramParser::new().parse(sierra_code).unwrap();
        Compiler::compile_sierra_program_to_llvm(program, program_path, llvm_output_path, target_triple)
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
    /// * `llvm_output_path` - The path to the output LLVM IR file.
    /// * `target_triple` - Compilation target triple.
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
    ///
    /// // TODO: Find a way to make doc tests pass.
    /// // Read the program from the file.
    /// let sierra_code = fs::read_to_string(sierra_program_path).unwrap();
    /// // Parse the program.
    /// let program = ProgramParser::new().parse(&sierra_code).unwrap();
    /// // Compile the program to LLVM IR.
    /// let result = Compiler::compile_from_file(&sierra_program_path, &llvm_ir_path, None);
    /// // Check the result.
    /// ```
    pub fn compile_sierra_program_to_llvm(
        program: Program,
        program_path: &Path,
        llvm_output_path: &Path,
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
        let debug_metadata_version = context.i32_type().const_int(3, false);
        module.add_basic_value_flag(
            "Debug Info Version",
            inkwell::module::FlagBehavior::Warning,
            debug_metadata_version,
        );

        let mut parent = program_path.parent().unwrap().to_string_lossy();
        if parent.is_empty() {
            parent = ".".into();
        }

        let (dibuilder, compile_unit) = module.create_debug_info_builder(
            true,
            inkwell::debug_info::DWARFSourceLanguage::CPlusPlus,
            &program_path.to_string_lossy(),
            &parent,
            "shenlong",
            false,                                        // is_optimized
            "",                                           // compiler command line flags
            0,                                            // runtime version
            "",                                           // split name
            inkwell::debug_info::DWARFEmissionKind::Full, // kind
            0,                                            // dwo_id
            false,                                        // split_debug_inlining
            false,                                        // debug_info_for_profiling
            "",                                           // The Clang system root (value of -isysroot).  ?
            "",                                           //  The SDK. On Darwin, the last component of the sysroot.  ?
        );

        if let Some(target_triple) = target_triple {
            module.set_triple(&TargetTriple::create(target_triple));
        }

        info!("Target triple: {}", module.get_triple());

        // Create a map of valid state transitions.
        let valid_state_transitions = Compiler::init_state_transitions();

        // Create a new compiler.
        let mut compiler = Compiler {
            program: &program,
            context: &context,
            builder: &builder,
            module,
            variables: HashMap::new(),
            llvm_output_path: llvm_output_path.to_owned(),
            state: CompilationState::NotStarted,
            valid_state_transitions,
            types_by_id: HashMap::new(),
            types_by_name: HashMap::new(),
            basic_blocks: HashMap::new(),
            jump_dests: HashSet::new(),
            debug: DebugCompiler::new(dibuilder, &builder, compile_unit, &context),
        };

        // Setup the debug info.
        compiler.setup_debug()?;

        // Process the types in the Sierra program.
        compiler.process_types()?;

        // Process the core library functions in the Sierra program.
        compiler.process_core_lib_functions()?;

        // Collect jumps.
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

        self.debug.debug_builder.finalize();
        // let parent =
        //     output_path.parent().ok_or_else(|| eyre::eyre!("parent output path is not valid"))?;
        // // Recursively create the output path parent directories if they don't exist.
        // fs::create_dir_all(parent)?;
        // // Write the module to the output path.
        self.module.print_to_file(&self.llvm_output_path)?;
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
    pub const fn state(&self) -> &CompilationState {
        &self.state
    }

    /// Initialize valid state transitions.
    pub fn init_state_transitions() -> HashMap<(CompilationState, CompilationState), bool> {
        HashMap::from([
            ((CompilationState::NotStarted, CompilationState::DebugSetup), true),
            ((CompilationState::DebugSetup, CompilationState::TypesProcessed), true),
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
    const fn err_invalid_state_transition(invalid_transition: CompilationStateTransition) -> CompilerError {
        CompilerError::InvalidStateTransition(invalid_transition.0, invalid_transition.1)
    }
}
