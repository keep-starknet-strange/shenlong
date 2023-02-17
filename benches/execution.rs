use std::path::Path;

use cairo_vm::cairo_run::CairoRunConfig;
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::program::Program;
use cairo_vm::vm::errors::vm_exception::VmException;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use cairo_vm::vm::vm_core::VirtualMachine;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

pub fn criterion_benchmark(c: &mut Criterion) {
    // LLVM IR execution benchmark
    let context = inkwell::context::Context::create();
    let module = Module::parse_bitcode_from_path("core/tests/test_data/llvm/bc/fib_main.bc", &context).unwrap();
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
    c.bench_with_input(BenchmarkId::new("Llvm", 1), &(module, execution_engine), |b, (module, execution_engine)| {
        b.iter(|| {
            run_bc_file(module, execution_engine);
        });
    });

    // Cairo execution benchmark
    let cairo_run_config = CairoRunConfig::default();
    let program = match Program::from_file(Path::new("fib0.json"), Some(cairo_run_config.entrypoint)) {
        Ok(program) => program,
        Err(_) => panic!("File not found"),
    };
    c.bench_with_input(BenchmarkId::new("Cairo", 1), &(program, cairo_run_config), |b, (program, cairo_run_config)| {
        b.iter(|| {
            cairo_run_test(program, cairo_run_config);
        });
    });
}
fn run_bc_file(module: &Module, execution_engine: &ExecutionEngine) {
    unsafe {
        execution_engine.run_function(module.get_function("main").unwrap(), &[]);
    }
}

fn cairo_run_test(program: &Program, cairo_run_config: &CairoRunConfig) {
    let mut hint_executor = BuiltinHintProcessor::new_empty();
    let mut cairo_runner = CairoRunner::new(program, cairo_run_config.layout, cairo_run_config.proof_mode).unwrap();
    let mut vm = VirtualMachine::new(cairo_run_config.trace_enabled);
    let end = cairo_runner.initialize(&mut vm).unwrap();
    cairo_runner
        .run_until_pc(end, &mut vm, &mut hint_executor)
        .map_err(|err| VmException::from_vm_error(&cairo_runner, &vm, err))
        .unwrap();
    cairo_runner.end_run(false, false, &mut vm, &mut hint_executor).unwrap();

    vm.verify_auto_deductions().unwrap();
    cairo_runner.read_return_values(&mut vm).unwrap();
    cairo_runner.relocate(&mut vm).unwrap();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
