use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::{eyre, Result};
use shenlong_core::sierra::llvm_compiler;

use crate::emoji;

/// Shenlong command line interface.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    pub config: Option<PathBuf>,
    /// List of supported commands.
    #[command(subcommand)]
    pub command: Commands,
}

/// Implementation of the Shenlong command line interface.
impl Command {
    /// Main entry point for the Shenlong command line interface.
    /// # Arguments
    /// * `cli` - The CLI arguments.
    /// # Returns
    /// The result of the execution.
    /// # Errors
    /// If the execution fails.
    pub async fn run(self) -> Result<()> {
        match self.command {
            Commands::Sierra(sierra_commands) => match sierra_commands.command {
                SierraSubCommands::CompileToLlvm { program_path, output_path } => {
                    // Compile the program.
                    // TODO: Handle the output path properly.
                    llvm_compiler::Compiler::compile_from_file(&program_path, &output_path.unwrap())
                        .map_err(|e| eyre!(e.to_string()))?;
                    println!("{} Program compiled successfully.", emoji::CHECK_MARK_BUTTON);
                    Ok(())
                }
            },
        }
    }
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
