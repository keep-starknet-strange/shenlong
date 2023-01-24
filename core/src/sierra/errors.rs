/// This file contains everything error related.
use thiserror::Error;

/// Compiler errors.
#[derive(Error, Debug)]
pub enum CompilerError {
    /// LLVM IR variable not found.
    #[error("Variable \"{0}\" not found")]
    VarNotFound(String),
    /// LLVM IR function not found
    #[error("Function \"{0}\" not found")]
    FuncNotFound(String),
    /// LLVM IR type not found.
    #[error("Type \"{0}\" not found")]
    TypeNotFound(String),
    /// No LLVM IR type was provided.
    #[error("No type provided")]
    NoTypeProvided,
    /// Object has no debug name.
    #[error("Variable has no debug name")]
    NoDebugName,
    /// LLVM IR function has no return value.
    #[error("No return value")]
    NoReturnValue,
    /// LLVM IR function has no return type.
    #[error("No return type")]
    NoReturnType,
    /// LLVM IR Error (verify or write to file).
    #[error(transparent)]
    LlvmPrintError(#[from] inkwell::support::LLVMString),
    /// File or folder not found.
    #[error(transparent)]
    PathNotFound(#[from] std::io::Error),
    /// Invalid compiler state transition.
    #[error("invalid state transition: {0:?} -> {1:?}")]
    InvalidStateTransition(
        super::llvm_compiler::CompilationState,
        super::llvm_compiler::CompilationState,
    ),
    /// Compiler state doesn't exist.
    #[error("Invalid state")]
    InvalidState,
}
pub type CompilerResult<T> = Result<T, CompilerError>;
