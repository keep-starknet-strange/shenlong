use crate::{cli::Cli, cli::Commands, cli::SierraSubCommands};
use eyre::Result;
use shenlong_core::sierra::llvm_compiler;

/// Main entry point for the Shenlong CLI.
/// # Arguments
/// * `cli` - The CLI arguments.
/// # Returns
/// The result of the execution.
/// # Errors
/// If the execution fails.
pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Sierra(sierra_commands) => match sierra_commands.command {
            SierraSubCommands::CompileToLlvm {
                program_path,
                output_path,
            } => {
                // Create a new compiler.
                let compiler = llvm_compiler::Compiler::new();
                // Compile the program.
                // TODO: Handle the output path properly.
                compiler.compile(&program_path, &output_path.unwrap())
            }
        },
    }
}
