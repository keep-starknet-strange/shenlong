use std::process::{Command, Stdio};

use honggfuzz::fuzz;
use num_bigint::BigInt;
use num_traits::Num;
use serde::Serialize;
use shenlong_core::sierra::llvm_compiler::Compiler;
use tinytemplate::TinyTemplate;

macro_rules! test_template_file {
    ($template_file:literal, $ctx:ident) => {{
        let template =
            std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../core/tests/templates/", $template_file));
        println!("{:}", template.display());
        let mut tt = TinyTemplate::new();
        let template = std::fs::read_to_string(template).unwrap();
        tt.add_template("test", &template).unwrap();
        tt.render("test", &$ctx).unwrap().clone()
    }};
}

#[derive(Serialize)]
struct BinaryContext {
    lhs: String,
    rhs: String,
}

fn main() {
    loop {
        fuzz!(|data: (&[u8], &[u8])| {
            use inkwell::targets::{InitializationConfig, Target};
            let lhs = BigInt::from_bytes_be(num_bigint::Sign::NoSign, data.0);
            let rhs = BigInt::from_bytes_be(num_bigint::Sign::NoSign, data.1);

            Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");

            let ctx = BinaryContext { lhs: lhs.to_string(), rhs: rhs.to_string() };
            let source = test_template_file!("addition.sierra", ctx);

            let tmp = tempdir::TempDir::new("test_simple_addition").unwrap();
            let file = tmp.into_path().join("output.ll");

            Compiler::compile_from_code(&source, &file, None).unwrap();

            let lli_path =
                std::env::var("LLI_PATH").expect("LLI_PATH must exist and point to the `lli` tool from llvm 16");

            let cmd = Command::new(lli_path).arg(file).stdout(Stdio::piped()).spawn().unwrap();

            let output = cmd.wait_with_output().unwrap();
            let output = std::str::from_utf8(&output.stdout).unwrap().trim();

            assert!(output.starts_with("Return value: "));
            let output = &output["Return value: ".len()..];

            let x = BigInt::from_str_radix(output, 16).unwrap();

            let expected = lhs + rhs;

            assert!(x == expected);
        });
    }
}
