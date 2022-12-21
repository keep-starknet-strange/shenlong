use std::time::Instant;

use clap::Parser;
use eyre::Result;
use owo_colors::OwoColorize;
use shenlong_cli::{cli::Command, emoji};

const SHENLONG: &str = r#"
███████╗██╗  ██╗███████╗███╗   ██╗██╗      ██████╗ ███╗   ██╗ ██████╗ 
██╔════╝██║  ██║██╔════╝████╗  ██║██║     ██╔═══██╗████╗  ██║██╔════╝ 
███████╗███████║█████╗  ██╔██╗ ██║██║     ██║   ██║██╔██╗ ██║██║  ███╗
╚════██║██╔══██║██╔══╝  ██║╚██╗██║██║     ██║   ██║██║╚██╗██║██║   ██║
███████║██║  ██║███████╗██║ ╚████║███████╗╚██████╔╝██║ ╚████║╚██████╔╝
╚══════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ 
                                                                      
"#;

const CAIRO_2_LLVM: &str = r#"
██████╗ █████╗ ██╗██████╗  ██████╗     ██████╗     ██╗     ██╗    ██╗   ██╗███╗   ███╗
██╔════╝██╔══██╗██║██╔══██╗██╔═══██╗    ╚════██╗    ██║     ██║    ██║   ██║████╗ ████║
██║     ███████║██║██████╔╝██║   ██║     █████╔╝    ██║     ██║    ██║   ██║██╔████╔██║
██║     ██╔══██║██║██╔══██╗██║   ██║    ██╔═══╝     ██║     ██║    ╚██╗ ██╔╝██║╚██╔╝██║
╚██████╗██║  ██║██║██║  ██║╚██████╔╝    ███████╗    ███████╗███████╗╚████╔╝ ██║ ╚═╝ ██║
 ╚═════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝ ╚═════╝     ╚══════╝    ╚══════╝╚══════╝ ╚═══╝  ╚═╝     ╚═╝
                                                                                                                            
"#;

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n\n\n\n\n{}", SHENLONG.fg_rgb::<0x00, 0xE6, 0x76>().bold());
    println!("\n\n{}", CAIRO_2_LLVM.fg_rgb::<0x00, 0xE6, 0x76>().bold());

    // Initialize the logger.
    env_logger::init();
    // Parse the command line.
    let cli = Command::parse();

    // Start the timer.
    let started = Instant::now();

    // Run the command.
    cli.run().await?;
    println!(
        "{} Done in {} milliseconds.",
        emoji::SPARKLE,
        started.elapsed().as_millis()
    );
    Ok(())
}
