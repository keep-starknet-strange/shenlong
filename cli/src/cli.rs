use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Main struct for the Beerus CLI args.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub config: Option<PathBuf>,
    /// List of supported commands.
    #[command(subcommand)]
    pub command: Commands,
}

/// List of supported commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Ethereum related subcommands
    #[command(about = "Sierra related subcommands")]
    Sierra(SierraCommands),
}

/// Sierra related commands.
#[derive(Parser, Debug)]
pub struct SierraCommands {
    /// Ethereum related subcommands.
    #[command(subcommand)]
    pub command: SierraSubCommands,
}

/// Sierra related subcommands.
#[derive(Subcommand, Debug)]
pub enum SierraSubCommands {
    /// Compiles a Sierra program to LLVM IR.
    CompileToLlvm {
        /// The path to the Sierra program to compile.
        #[arg(short, long, value_name = "PROGRAM_PATH")]
        program_path: String,
        /// The path to the output LLVM IR file.
        /// If not specified, the output will be printed to stdout.
        /// If specified, the output will be written to the specified file.
        #[arg(short, long, value_name = "OUTPUT_PATH")]
        output_path: Option<String>,
    },
}
