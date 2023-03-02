use std::ops::*;
use std::path::Path;
use std::process::{Command, Stdio};

use honggfuzz::fuzz;
use num_bigint::BigInt;
use num_traits::{Num, One, Zero};
use serde::Serialize;
use shenlong_core::sierra::llvm_compiler::Compiler;

use crate::{get_prime, test_template_file};

#[derive(Serialize)]
pub struct BinaryContext {
    lhs: String,
    rhs: String,
    op: String,
}

#[inline(always)]
pub fn operation(case: &str) {
    fuzz!(|data: (&[u8], &[u8])| {
        let prime = get_prime();

        let lhs = BigInt::from_bytes_be(
            if data.0.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            data.0,
        ) % &prime;
        let rhs = BigInt::from_bytes_be(
            if data.1.len() % 2 == 0 { num_bigint::Sign::Plus } else { num_bigint::Sign::Minus },
            data.1,
        ) % &prime;

        println!("{lhs:}, {rhs:}");
        let mut expected = match case {
            "add" => BigInt::add,
            "sub" => BigInt::sub,
            "mul" => BigInt::mul,
            "div" => {
                if rhs == BigInt::zero() {
                    return;
                }
                divmod
            }
            _ => panic!("invalid case: {case:}"),
        }(lhs.clone(), rhs.clone())
            % &prime;

        let ctx = BinaryContext { lhs: lhs.to_string(), rhs: rhs.to_string(), op: case.to_owned() };
        let source = test_template_file!("binary_op.sierra", ctx);
        let tmp = tempdir::TempDir::new("test_simple_operation").unwrap();
        let file = tmp.into_path().join("output.ll");

        Compiler::compile_from_code(&source, &file, Path::new(""), None).unwrap();
        let lli_path = std::env::var("LLI_PATH").expect("LLI_PATH must exist and point to the `lli` tool from llvm 16");

        let cmd = Command::new(lli_path).arg(file).stdout(Stdio::piped()).spawn().unwrap();

        let output = cmd.wait_with_output().unwrap();
        let output = std::str::from_utf8(&output.stdout).unwrap().trim();

        assert!(output.starts_with("Return value: "));
        println!("{lhs:}, {rhs:}");
        let output = &output["Return value: ".len()..];
        let mut return_value = BigInt::from_str_radix(output, 16).unwrap();
        let two = BigInt::from(2).pow(return_value.bits() as u32);
        expected = expected.modpow(&BigInt::one(), &two);
        println!("{lhs:}, {rhs:}, {return_value:}, {expected:}");
        return_value =
            if return_value > prime { (return_value - &two).modpow(&BigInt::one(), &prime) } else { return_value };
        assert_eq!(return_value, expected);
    });
}

fn divmod(lhs: BigInt, rhs: BigInt) -> BigInt {
    lhs * modinverse(rhs.modpow(&BigInt::one(), &get_prime()), get_prime())
}
fn modinverse(a: BigInt, m: BigInt) -> BigInt {
    let (g, x, _) = egcd(a.clone(), m.clone());
    println!("{a:}");
    assert_eq!(g, BigInt::one());
    (&x % &m + &m) % &m
}

fn egcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if a == BigInt::zero() {
        (b, BigInt::zero(), BigInt::one())
    } else {
        let (g, x, y) = egcd(&b % &a, a.clone());
        (g, y - (&b / &a) * &x, x)
    }
}
