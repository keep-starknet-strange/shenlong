use std::ops::*;
use std::process::{Command, Stdio};

use num_bigint::BigInt;
use num_traits::{Num, Zero};
use proptest::prelude::*;
use serde::Serialize;
use shenlong_core::sierra::llvm_compiler::Compiler;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct BinaryContext<'a> {
    op: &'a str,
    lhs: &'a str,
    rhs: &'a str,
}

fn get_prime() -> BigInt {
    BigInt::from_str_radix("3618502788666131213697322783095070105623107215331596699973092056135872020481", 10).unwrap()
}

fn test_binary_op<F>(op: &str, lhs: &str, rhs: &str, bigint_op: F) -> Result<(), proptest::test_runner::TestCaseError>
where
    F: FnOnce(BigInt, BigInt) -> BigInt,
{
    let ctx = BinaryContext { op, lhs, rhs };
    let template = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/templates/binary_op.sierra"));
    let mut tt = TinyTemplate::new();
    let template = std::fs::read_to_string(template).unwrap();
    tt.add_template("test", &template).unwrap();
    let source = tt.render("test", &ctx).unwrap();

    let tmp = tempdir::TempDir::new("shenlong_tests").unwrap();
    let file = tmp.into_path().join("output.ll");

    Compiler::compile_from_code(&source, &file, None).unwrap();

    let lli_path = std::env::var("LLI_PATH").expect("LLI_PATH must exist and point to the `lli` tool from llvm 16");

    let cmd = Command::new(lli_path).arg(file).stdout(Stdio::piped()).spawn().unwrap();

    let output = cmd.wait_with_output().unwrap();
    let output = std::str::from_utf8(&output.stdout).unwrap().trim();

    assert!(output.starts_with("Return value: "));
    let output = &output["Return value: ".len()..];

    let return_value = BigInt::from_str_radix(output, 16).unwrap();

    let prime = get_prime();

    let lhs = BigInt::from_str_radix(lhs, 10).unwrap();
    let rhs = BigInt::from_str_radix(rhs, 10).unwrap();
    let mut expected = bigint_op(lhs, rhs) % prime;
    let two = BigInt::from(2).pow(return_value.bits() as u32);
    expected = expected.modpow(&BigInt::from(1), &two);
    prop_assert_eq!(return_value, expected);
    Ok(())
}

#[test]
fn simple_addition() {
    test_binary_op("add", "9223372036854775807", "9223372036854775807", BigInt::add).unwrap();
}

#[test]
fn addition_overflow() {
    test_binary_op(
        "add",
        "3618502788666131213697322783095070105623107215331596699973092056135872020480",
        "2",
        BigInt::add,
    )
    .unwrap();
}

#[test]
fn substraction_negative_result() {
    test_binary_op("sub", "2", "4", BigInt::sub).unwrap();
}

proptest! {
    #[test]
    fn proptest_add(a: Vec<u8>, b: Vec<u8>) {
        let prime = get_prime();

        let lhs = BigInt::from_bytes_be(
            if a.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            &a,
        ) % &prime;
        let rhs = BigInt::from_bytes_be(
            if b.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            &b,
        ) % &prime;

        test_binary_op("add", &lhs.to_str_radix(10), &rhs.to_str_radix(10), BigInt::add)?;
    }

    #[test]
    fn proptest_sub(a: Vec<u8>, b: Vec<u8>) {
        let prime = get_prime();

        let lhs = BigInt::from_bytes_be(
            if a.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            &a,
        ) % &prime;
        let rhs = BigInt::from_bytes_be(
            if b.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            &b,
        ) % &prime;
        test_binary_op("sub", &lhs.to_str_radix(10), &rhs.to_str_radix(10), BigInt::sub)?;
    }

    #[test]
    fn proptest_mul(a: Vec<u8>, b: Vec<u8>) {
        let prime = get_prime();

        let lhs = BigInt::from_bytes_be(
            if a.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            &a,
        ) % &prime;
        let rhs = BigInt::from_bytes_be(
            if b.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            &b,
        ) % &prime;
        test_binary_op("mul", &lhs.to_str_radix(10), &rhs.to_str_radix(10), BigInt::mul)?;
    }
}
