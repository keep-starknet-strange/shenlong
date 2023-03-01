use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use cairo_lang_sierra::ProgramParser;
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
use shenlong_core::sierra::llvm_compiler::{CompilationState, Compiler};

pub fn criterion_benchmark(c: &mut Criterion) {
    benchmark_llvm(c, "core/tests/test_data/sierra/fib_bench.sierra");
    benchmark_cairo(c, "fib0.json")
}

fn benchmark_llvm(c: &mut Criterion, file_path: &str) {
    let context = inkwell::context::Context::create();

    let module = context.create_module("root");

    let mut parent = Path::new(file_path).parent().unwrap().to_string_lossy();
    if parent.is_empty() {
        parent = ".".into();
    }

    let (dibuilder, compile_unit) = module.create_debug_info_builder(
        true,
        inkwell::debug_info::DWARFSourceLanguage::CPlusPlus,
        file_path,
        &parent,
        "shenlong",
        false,                                        // is_optimized
        "",                                           // compiler command line flags
        0,                                            // runtime version
        "",                                           // split name
        inkwell::debug_info::DWARFEmissionKind::Full, // kind
        0,                                            // dwo_id
        false,                                        // split_debug_inlining
        false,                                        // debug_info_for_profiling
        "",                                           // The Clang system root (value of -isysroot).  ?
        "",                                           //  The SDK. On Darwin, the last component of the sysroot.  ?
    );

    let mut compiler = Compiler {
        program: &ProgramParser::new().parse(&fs::read_to_string(file_path).unwrap()).unwrap(),
        context: &context,
        builder: &context.create_builder(),
        module,
        variables: HashMap::new(),
        llvm_output_path: Path::new("").to_path_buf(),
        state: CompilationState::NotStarted,
        valid_state_transitions: Compiler::init_state_transitions(),
        types_by_id: HashMap::new(),
        types_by_name: HashMap::new(),
        debug_types_by_name: HashMap::new(),
        basic_blocks: HashMap::new(),
        jump_dests: HashSet::new(),
        dibuilder,
        compile_unit,
        debug_types_by_id: HashMap::new(),
        current_line_estimate: 0,
    };

    compiler.setup_debug().unwrap();
    compiler.process_types().unwrap();
    compiler.process_core_lib_functions().unwrap();
    compiler.collect_jumps();
    compiler.process_funcs().unwrap();
    let execution_engine = compiler.module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
    c.bench_with_input(
        BenchmarkId::new("Llvm", 1),
        &(compiler.module, execution_engine),
        |b, (module, execution_engine)| {
            b.iter(|| {
                run_llvm_module(module, execution_engine);
            });
        },
    );
}

fn benchmark_cairo(c: &mut Criterion, file_path: &str) {
    // Cairo execution benchmark
    let cairo_run_config = CairoRunConfig::default();
    let program = match Program::from_file(Path::new(file_path), Some(cairo_run_config.entrypoint)) {
        Ok(program) => program,
        Err(_) => panic!("File not found"),
    };
    c.bench_with_input(BenchmarkId::new("Cairo", 1), &(program, cairo_run_config), |b, (program, cairo_run_config)| {
        b.iter(|| {
            cairo_run(program, cairo_run_config);
        });
    });
}

fn run_llvm_module(module: &Module, execution_engine: &ExecutionEngine) {
    unsafe {
        execution_engine.run_function(module.get_function("main").unwrap(), &[]);
    }
}

fn cairo_run(program: &Program, cairo_run_config: &CairoRunConfig) {
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
