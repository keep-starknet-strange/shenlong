use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Variable \"{0}\" not found")]
    VarNotFound(String),
    #[error("Function \"{0}\" not found")]
    FuncNotFound(String),
    #[error("Type \"{0}\" not found")]
    TypeNotFound(String),
    #[error("No type provided")]
    NoTypeProvided,
    #[error("Variable has no debug name")]
    NoDebugName,
    #[error("No return value")]
    NoReturnValue,
    #[error("No return type")]
    NoReturnType,
    #[error(transparent)]
    LlvmPrintError(#[from] inkwell::support::LLVMString),
    #[error(transparent)]
    PathNotFound(#[from] std::io::Error),
    #[error("invalid state transition: {0:?} -> {1:?}")]
    InvalidStateTransition(
        super::llvm_compiler::CompilationState,
        super::llvm_compiler::CompilationState,
    ),
    #[error("Invalid state")]
    InvalidState,
}
pub type CompilerResult<T> = Result<T, CompilerError>;
