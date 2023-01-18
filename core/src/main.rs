use shenlong_core::sierra::llvm_compiler;

fn main() {
    let _ = llvm_compiler::Compiler::compile_from_file(
        "./core/tests/test_data/sierra/fib.sierra",
        "/dev/null",
    );
}
