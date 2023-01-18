use criterion::{criterion_group, criterion_main, Criterion};
use shenlong_core::sierra::llvm_compiler::Compiler;
use tempdir::TempDir;

/// Define a macro to get the path of a benchmark resource file.
macro_rules! bench_resource_file {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/bench/", $fname) // assumes Linux ('/')!
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sierra-2-llvm-simple-test", |b| {
        b.iter(sierra_2_llvm_simple_test);
    });
}

fn sierra_2_llvm_simple_test() {
    let program_path = bench_resource_file!("sierra/simple_test.sierra");
    let tmp_dir = TempDir::new("tmp").unwrap();
    let output_path = tmp_dir.path().join("simple_test.ll");
    let result = Compiler::compile_from_file(program_path, output_path.to_str().unwrap());
    assert!(result.is_ok());
    tmp_dir.close().unwrap();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
