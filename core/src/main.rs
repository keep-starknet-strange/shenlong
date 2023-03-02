use std::path::Path;

use shenlong_core::sierra::errors::CompilerResult;
use shenlong_core::sierra::llvm_compiler;
fn main() -> CompilerResult<()> {
    llvm_compiler::Compiler::compile_from_file(
        Path::new("./core/tests/test_data/sierra/division.sierra"),
        Path::new("./core/tests/test_data/llvm/division.ll"),
        Some("x86_64-pc-linux-gnu"),
    )?;
    Ok(())
}
