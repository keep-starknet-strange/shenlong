use std::fs;

use clap::Parser;
use shenlong_core::sierra::llvm_compiler;
use sierra::ProgramParser;

/// Command line args parser.
/// Exits with 0/1 if the input is formatted correctly/incorrectly.
#[derive(Parser, Debug)]
#[clap(version, verbatim_doc_comment)]
struct Args {
    /// The path to the Sierra program to compile.
    #[arg(short, long, value_name = "PROGRAM_PATH")]
    program_path: String,
    /// The path to the output LLVM IR file.
    /// If not specified, the output will be printed to stdout.
    /// If specified, the output will be written to the specified file.
    #[arg(short, long, value_name = "OUTPUT_PATH")]
    output_path: Option<String>,
}

fn main() {
    println!("Shenlong, make me a LLVM compiler!");
    // Initialize the logger.
    env_logger::init();
    // Parse the CLI arguments.
    let args = Args::parse();
    let sierra_code = fs::read_to_string(args.program_path).expect("Could not read file!");
    let program = ProgramParser::new().parse(&sierra_code).unwrap();
    let compiler = llvm_compiler::Compiler::new();
    // Compile the program.
    // TODO: Handle the output path properly.
    compiler
        .compile(program, &args.output_path.unwrap())
        .unwrap();
}
