use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

pub fn criterion_benchmark(c: &mut Criterion) {
    let context = inkwell::context::Context::create();
    let module = Module::parse_bitcode_from_path("core/tests/test_data/llvm/bc/fib_main.bc", &context).unwrap();
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
    c.bench_with_input(
        BenchmarkId::new("Test llvm", 1),
        &(module, execution_engine),
        |b, (module, execution_engine)| {
            b.iter(|| {
                sierra_add_test(module, execution_engine);
            });
        },
    );
}

fn sierra_add_test(module: &Module, execution_engine: &ExecutionEngine) {
    unsafe {
        execution_engine.run_function(module.get_function("main").unwrap(), &[]);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
