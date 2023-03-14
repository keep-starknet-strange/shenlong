use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::Context;
use cairo_lang_runner::{RunResult, SierraCasmRunner};
use cairo_lang_sierra::ProgramParser;
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Num};
use shenlong_core::sierra::corelib_functions::math::DEFAULT_PRIME;
use shenlong_core::sierra::llvm_compiler::Compiler;
use test_case::test_case;

// Tests behaviour of the generated llir against the behaviour of starkware's own sierra runner
// Such tests must be an argumentless main function consisting of calls to the function in question

#[test_case("felt_addition")]
#[test_case("felt_subtraction")]
#[test_case("felt_multiplication")]
#[test_case("felt_negation")]
#[test_case("enum_flow")]
#[test_case("fib_felt_is_zero")]
fn comparison_test(test_name: &str) {
    let sierra_code = fs::read_to_string(&format!("./tests/comparison/{test_name}.sierra")).unwrap();
    let llvm_result = run_sierra_via_llvm(test_name, &sierra_code);
    let casm_result = run_sierra_via_casm(&sierra_code);

    match casm_result {
        Ok(result) => match result.value {
            cairo_lang_runner::RunResultValue::Success(casm_values) => {
                assert_eq!(
                    casm_values.len(),
                    llvm_result.len(),
                    "Casm values and llvm values are of different lengths"
                );
                let prime = DEFAULT_PRIME.parse::<BigUint>().unwrap();
                for i in 0..casm_values.len() {
                    assert!(
                        llvm_result[i] < prime,
                        "Test no. {} of {} failed. {} >= PRIME. Expected {} (-{})",
                        i + 1,
                        test_name,
                        llvm_result[i],
                        casm_values[i],
                        prime.clone() - casm_values[i].to_biguint()
                    );
                    assert_eq!(
                        casm_values[i].to_biguint(),
                        llvm_result[i],
                        "Test no. {} of {} failed. {} != {} (-{} != -{})",
                        i + 1,
                        test_name,
                        casm_values[i],
                        llvm_result[i],
                        prime.clone() - casm_values[i].to_biguint(),
                        prime.clone() - llvm_result[i].clone()
                    )
                }
            }
            cairo_lang_runner::RunResultValue::Panic(_) => {
                todo!();
            }
        },
        Err(_) => {
            todo!();
        }
    }
}

// Invokes starkware's runner that compiles sierra to casm and runs it
// This provides us with the intended results to compare against
fn run_sierra_via_casm(sierra_code: &str) -> Result<RunResult, anyhow::Error> {
    let sierra_program = ProgramParser::new().parse(sierra_code).unwrap();

    let runner = SierraCasmRunner::new(sierra_program, false).with_context(|| "Failed setting up runner.")?;

    runner.run_function("::main", &[], None).with_context(|| "Failed to run the function.")
}

// Runs the test file via compiling to llir then invoking lli to run it
fn run_sierra_via_llvm(test_name: &str, sierra_code: &str) -> Vec<BigUint> {
    let tmp = tempdir::TempDir::new("test_comparison").unwrap();
    let file = tmp.into_path().join("output.ll");

    Compiler::compile_from_code(sierra_code, Path::new(test_name), &file, None).unwrap();

    let lli_path = std::env::var("LLI_PATH").expect("LLI_PATH must exist and point to the `lli` tool from llvm 16");

    let cmd = Command::new(lli_path).arg(file).stdout(Stdio::piped()).spawn().unwrap();

    let output = cmd.wait_with_output().unwrap();
    let output = std::str::from_utf8(&output.stdout).unwrap().trim();

    return parse_llvm_result(output);
}

// Parses the human-readable output from running the llir code into a raw list of outputs
fn parse_llvm_result(res: &str) -> Vec<BigUint> {
    if res.starts_with("Return value: ") {
        let val_string = &res["Return value: ".len()..];
        return vec![BigUint::from_str_radix(val_string, 16).unwrap()];
    } else if res.starts_with("Return field") {
        return res
            .split('\n')
            .map(|line| &line[line.find("value: ").unwrap() + "value: ".len()..])
            // .map(|line| {
            //     println!("{line}");
            //     line
            // })
            .map(|val: &str| {
                // The output from the llvm ir can include values in the range (-PRIME, 0), which need to be wrapped around
                // Because the number of bits is rounded up to 256, positive numbers start with 0 and negatives start with 1
                if val.starts_with('1') {
                    let prime = DEFAULT_PRIME.parse::<BigUint>().unwrap();
                    let pos = BigUint::from_str_radix(val.strip_prefix('1').unwrap(), 16).unwrap();
                    let neg = BigUint::from_u32(2).unwrap().pow((4*(val.chars().count()-1)).try_into().unwrap());
                    prime - (neg - pos)
                } else {
                    BigUint::from_str_radix(val, 16).unwrap()
                }
            })
            .collect();
    } else {
        panic!("Unexpected output from running via llvm:\n{res}");
    }
}
