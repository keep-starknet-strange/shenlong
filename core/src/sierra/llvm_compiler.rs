use eyre::Result;

// Compiler is the main entry point for the LLVM backend.
// It is responsible for compiling a Sierra program to LLVM IR.
pub struct Compiler {
    // ...
}

impl Compiler {
    /// Creates a new compiler.
    /// # Returns
    /// A new compiler.
    pub fn new() -> Self {
        Compiler {}
    }

    /// Compiles a Sierra program to LLVM IR.
    /// # Arguments
    /// * `program_path` - The Sierra program to compile.
    /// * `output_path` - The path to the output LLVM IR file.
    /// # Returns
    /// The result of the compilation.
    /// # Errors
    /// If the compilation fails.
    pub fn compile(&self, _program_path: &str, _output_path: &str) -> Result<()> {
        todo!()
    }
}
