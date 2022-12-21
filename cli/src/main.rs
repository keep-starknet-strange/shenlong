use clap::Parser;
use eyre::Result;
use shenlong_cli::cli::Command;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Shenlong, make me a LLVM compiler!");
    // Initialize the logger.
    env_logger::init();
    // Parse the command line.
    let cli = Command::parse();
    // Run the command.
    cli.run().await
}
