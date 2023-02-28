extern crate tempdir;
use regex::Regex;
use shenlong_core::sierra::llvm_compiler::Compiler;
use tempdir::TempDir;
use test_case::test_case;

/// Define a macro to get the path of a test resource file.
macro_rules! test_resource_file {
    ($fname:expr) => {
        std::path::PathBuf::from(format!("{}{}{}", env!("CARGO_MANIFEST_DIR"), "/tests/test_data/", $fname)) // assumes Linux ('/')!
    };
}

#[test_case("addition")]
#[test_case("fib")]
#[test_case("fib_main")]
#[test_case("fib_box")]
#[test_case("fib_box_main")]
#[test_case("struct_construct")]
fn compile_sierra_program_to_llvm(name: &str) {
    let program_path = test_resource_file!(format!("sierra/{}.sierra", name));
    let tmp_dir = TempDir::new("tmp").unwrap();
    let llvm_output_path = tmp_dir.path().join("test.ll");
    let result = Compiler::compile_from_file(&program_path, &llvm_output_path, Some("x86_64-pc-linux-gnu"));
    assert!(result.is_ok());
    let mut llvm_ir = std::fs::read_to_string(llvm_output_path).unwrap();
    let mut expected_llvm_ir = std::fs::read_to_string(test_resource_file!(format!("llvm/{}.ll", name))).unwrap();

    // Replace the debug path.
    let debug_filename_regex = Regex::new(r#"!DIFile\(filename: ".*", directory: ".*"\)"#).unwrap();
    llvm_ir = debug_filename_regex.replace(&llvm_ir, r#"!DIFile(filename: "", directory: "")"#).to_string();
    expected_llvm_ir =
        debug_filename_regex.replace(&expected_llvm_ir, r#"!DIFile(filename: "", directory: "")"#).to_string();

    pretty_assertions::assert_eq!(llvm_ir, expected_llvm_ir);
    tmp_dir.close().unwrap();
}
