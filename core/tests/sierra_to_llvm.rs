extern crate tempdir;
use shenlong_core::sierra::llvm_compiler::Compiler;
use tempdir::TempDir;
use test_case::test_case;

/// Define a macro to get the path of a test resource file.
macro_rules! test_resource_file {
    ($fname:expr) => {
        format!("{}{}{}", env!("CARGO_MANIFEST_DIR"), "/tests/test_data/", $fname) // assumes Linux ('/')!
    };
}

#[test_case("addition")]
fn compile_sierra_program_to_llvm(name: &str) {
    let program_path = test_resource_file!(format!("sierra/{}.sierra", name));
    let tmp_dir = TempDir::new("tmp").unwrap();
    let output_path = tmp_dir.path().join("test.ll");
    let result = Compiler::compile_from_file(program_path.as_str(), output_path.to_str().unwrap());
    assert!(result.is_ok());
    let llvm_ir = std::fs::read_to_string(output_path).unwrap();
    let expected_llvm_ir = std::fs::read_to_string(test_resource_file!(format!("llvm/{}.ll", name))).unwrap();
    assert_eq!(llvm_ir, expected_llvm_ir);
    tmp_dir.close().unwrap();
}
