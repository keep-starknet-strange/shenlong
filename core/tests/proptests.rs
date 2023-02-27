use std::collections::{HashMap, HashSet};
use std::path::Path;

use cairo_lang_sierra::ProgramParser;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::OptimizationLevel;
use serde::Serialize;
use shenlong_core::sierra::llvm_compiler::{CompilationState, Compiler};
use tinytemplate::TinyTemplate;

macro_rules! test_template_file {
    ($template_file:literal, $ctx:ident) => {{
        let template =
            std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/templates/", $template_file));
        let mut tt = TinyTemplate::new();
        let template = std::fs::read_to_string(template).unwrap();
        tt.add_template("test", &template).unwrap();
        tt.render("test", &$ctx).unwrap().clone()
    }};
}

fn compile_and_run<F>(source: &str, run: F)
where
    F: FnOnce(&ExecutionEngine, &Compiler),
{
    let context = inkwell::context::Context::create();
    let mut compiler = Compiler {
        program: &ProgramParser::new().parse(source).unwrap(),
        context: &context,
        builder: &context.create_builder(),
        module: context.create_module("root"),
        variables: HashMap::new(),
        llvm_output_path: Path::new("").to_path_buf(),
        state: CompilationState::NotStarted,
        valid_state_transitions: Compiler::init_state_transitions(),
        types: HashMap::new(),
        id_from_name: HashMap::new(),
        basic_blocks: HashMap::new(),
        jump_dests: HashSet::new(),
    };

    compiler.process_types().unwrap();
    compiler.process_core_lib_functions().unwrap();
    compiler.collect_jumps();
    compiler.process_funcs().unwrap();
    let execution_engine = compiler.module.create_jit_execution_engine(OptimizationLevel::Default).unwrap();
    run(&execution_engine, &compiler);
}

#[derive(Serialize)]
struct BinaryContext<'a> {
    lhs: &'a str,
    rhs: &'a str,
}

#[test]
fn simple_addition() {
    let ctx = BinaryContext { lhs: "2", rhs: "4" };
    let source = test_template_file!("addition.sierra", ctx);

    compile_and_run(&source, |engine, compiler| {
        let func = compiler.module.get_function("add::add::add").unwrap();
        let value = unsafe { engine.run_function(func, &[]) };
        dbg!(value);
    });
}
