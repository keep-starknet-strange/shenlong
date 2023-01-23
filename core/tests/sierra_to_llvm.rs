extern crate tempdir;

/// Define a macro to get the path of a test resource file.
macro_rules! test_resource_file {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/", $fname) // assumes Linux ('/')!
    };
}

#[cfg(test)]
mod tests {
    use shenlong_core::sierra::llvm_compiler::Compiler;
    use tempdir::TempDir;

    #[test]
    fn compile_sierra_program_to_llvm() {
        let program_path = test_resource_file!("sierra/simple_test.sierra");
        let tmp_dir = TempDir::new("tmp").unwrap();
        let output_path = tmp_dir.path().join("simple_test.ll");
        let result = Compiler::compile_from_file(program_path, output_path.to_str().unwrap());
        assert!(result.is_ok());
        let _llvm_ir = std::fs::read_to_string(output_path).unwrap();
        let _expected_llvm_ir =
            std::fs::read_to_string(test_resource_file!("sierra/simple_test_expected.ll")).unwrap();
        // assert_eq!(llvm_ir, expected_llvm_ir);
        tmp_dir.close().unwrap();
    }
}
