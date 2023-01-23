use shenlong_core::sierra::llvm_compiler;

fn main() {
    llvm_compiler::Compiler::compile_from_file(
        "./core/tests/test_data/sierra/addition.sierra",
        "./core/tests/test_data/llvm/test.ll",
    )
    .unwrap();
}
