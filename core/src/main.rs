use std::path::Path;

use shenlong_core::sierra::errors::CompilerResult;
use shenlong_core::sierra::llvm_compiler;
fn main() -> CompilerResult<()> {
    llvm_compiler::Compiler::compile_from_file(
        Path::new("./core/tests/test_data/sierra/fib_array.sierra"),
        Path::new("./core/tests/test_data/llvm/fib_array.ll"),
        Some("x86_64-pc-linux-gnu"),
    )?;
    Ok(())
}
