use eyre::Result;
use sierra::ProgramParser;
use std::fs;

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
    pub fn compile(&self, program_path: &str, _output_path: &str) -> Result<()> {
        // Read the program from the file.
        let sierra_code = fs::read_to_string(program_path)?;
        // Parse the program.
        let program = ProgramParser::new().parse(&sierra_code).unwrap();
        println!("{program:#?}");
        // TODO: Compile the program to LLVM IR.
        // TODO: Write the LLVM IR to the output file.
        Ok(())
    }
}

/// `Default` implementation for `Compiler`.
impl Default for Compiler {
    /// Creates a new default compiler.
    /// # Returns
    /// A new default compiler.
    fn default() -> Self {
        Self::new()
    }
}
