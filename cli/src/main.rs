use clap::Parser;
use eyre::Result;
use shenlong_cli::cli::Cli;
use shenlong_cli::runner;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Shenlong, make me a LLVM compiler!");
    // Initialize the logger.
    env_logger::init();
    // Parse the CLI arguments.
    let cli = Cli::parse();
    // Run the CLI.
    runner::run(cli).await
}
