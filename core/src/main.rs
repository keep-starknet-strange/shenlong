use std::path::Path;

use shenlong_core::sierra::errors::CompilerResult;
use shenlong_core::sierra::llvm_compiler;
fn main() -> CompilerResult<()> {
    llvm_compiler::Compiler::compile_from_file(
        Path::new("./core/tests/test_data/sierra/fib_box_main.sierra"),
        Path::new("./core/tests/test_data/llvm/ll/fib_box_main.ll"),
        Path::new("./core/tests/test_data/llvm/bc/fib_box_main.bc"),
        Some("x86_64-pc-linux-gnu"),
    )?;
    Ok(())
}
