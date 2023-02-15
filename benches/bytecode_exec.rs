use std::path::Path;

use cairo_vm::cairo_run::{write_output, CairoRunConfig};
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::program::Program;
use cairo_vm::vm::errors::vm_exception::VmException;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use cairo_vm::vm::security::verify_secure_runner;
use cairo_vm::vm::vm_core::VirtualMachine;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

struct BenchInputs<'a> {
    program: &'a Program,
    cairo_run_config: CairoRunConfig<'a>,
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let cairo_run_config = CairoRunConfig::default();
    let program = match Program::from_file(&Path::new("fib0.json"), Some(cairo_run_config.entrypoint)) {
        Ok(program) => program,
        Err(_) => panic!("File not found"),
    };
    let params = BenchInputs { program: &program, cairo_run_config };
    c.bench_with_input(BenchmarkId::new("Test vm", 1), &params, |b, p| {
        b.iter(|| {
            cairo_run_test(p);
        });
    });
}

fn cairo_run_test<'a>(params: &BenchInputs) {
    let mut hint_executor = BuiltinHintProcessor::new_empty();
    let secure_run = !params.cairo_run_config.proof_mode;
    let mut cairo_runner =
        CairoRunner::new(params.program, params.cairo_run_config.layout, params.cairo_run_config.proof_mode).unwrap();
    let mut vm = VirtualMachine::new(params.cairo_run_config.trace_enabled);
    let end = cairo_runner.initialize(&mut vm).unwrap();
    cairo_runner
        .run_until_pc(end, &mut vm, &mut hint_executor)
        .map_err(|err| VmException::from_vm_error(&cairo_runner, &vm, err))
        .unwrap();
    cairo_runner.end_run(false, false, &mut vm, &mut hint_executor).unwrap();

    vm.verify_auto_deductions().unwrap();
    cairo_runner.read_return_values(&mut vm).unwrap();
    if params.cairo_run_config.proof_mode {
        cairo_runner.finalize_segments(&mut vm).unwrap();
    }
    if secure_run {
        verify_secure_runner(&cairo_runner, true, &mut vm).unwrap();
    }
    cairo_runner.relocate(&mut vm).unwrap();

    if params.cairo_run_config.print_output {
        write_output(&mut cairo_runner, &mut vm).unwrap();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
