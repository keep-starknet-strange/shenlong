use shenlong_core::sierra::errors::CompilerResult;
use shenlong_core::sierra::llvm_compiler;
fn main() -> CompilerResult<()> {
    llvm_compiler::Compiler::compile_from_file(
        "./core/tests/test_data/sierra/fib.sierra",
        "./core/tests/test_data/llvm/test.ll",
    )?;
    Ok(())
}
