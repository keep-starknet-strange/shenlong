use std::time::Instant;

use clap::Parser;
use eyre::Result;
use owo_colors::{DynColors, OwoColorize};
use shenlong_cli::cli::Command;
use shenlong_cli::emoji;

const SHENLONG: &str = r#"
███████╗██╗  ██╗███████╗███╗   ██╗██╗      ██████╗ ███╗   ██╗ ██████╗ 
██╔════╝██║  ██║██╔════╝████╗  ██║██║     ██╔═══██╗████╗  ██║██╔════╝ 
███████╗███████║█████╗  ██╔██╗ ██║██║     ██║   ██║██╔██╗ ██║██║  ███╗
╚════██║██╔══██║██╔══╝  ██║╚██╗██║██║     ██║   ██║██║╚██╗██║██║   ██║
███████║██║  ██║███████╗██║ ╚████║███████╗╚██████╔╝██║ ╚████║╚██████╔╝
╚══════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ 
                                                                      
"#;

const CAIRO_2_LLVM: &str = r#"
██████╗ *█████╗ *██╗*██████╗  *██████╗     *██████╗     *██╗     *██╗    *██╗   ██╗*███╗   ███╗
██╔════╝*██╔══██╗*██║*██╔══██╗*██╔═══██╗    *╚════██╗    *██║     *██║    *██║   ██║*████╗ ████║
██║     *███████║*██║*██████╔╝*██║   ██║     *█████╔╝    *██║     *██║    *██║   ██║*██╔████╔██║
██║     *██╔══██║*██║*██╔══██╗*██║   ██║    *██╔═══╝     *██║     *██║    *╚██╗ ██╔╝*██║╚██╔╝██║
╚██████╗*██║  ██║*██║*██║  ██║*╚██████╔╝    *███████╗    *███████╗*███████╗*╚████╔╝ *██║ ╚═╝ ██║
 ╚═════╝*╚═╝  ╚═╝*╚═╝*╚═╝  ╚═╝ *╚═════╝     *╚══════╝    *╚══════╝*╚══════╝ *╚═══╝  *╚═╝     ╚═╝
                                                                                                                            
"#;

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n\n\n\n\n{}", SHENLONG.fg_rgb::<0x00, 0xE6, 0x76>().bold());
    // println!("\n\n{}", CAIRO_2_LLVM.fg_rgb::<0x00, 0xE6, 0x76>().bold());

    let colors: [DynColors; 10] = [
        "#B80A41", "#4E4BA8", "#6EB122", "#DAAC06", "#00938A", "#E23838", "#B80A41", "#4E4BA8",
        "#6EB122", "#DAAC06",
    ]
    .map(|color| color.parse().unwrap());

    for line in CAIRO_2_LLVM.split_inclusive('\n') {
        for (text, color) in line.split('*').zip(colors.iter().copied()) {
            print!("{}", text.color(color).bold());
        }
    }

    // Initialize the logger.
    env_logger::init();
    // Parse the command line.
    let cli = Command::parse();

    // Start the timer.
    let started = Instant::now();

    // Run the command.
    cli.run().await?;
    println!("{} Done in {} milliseconds.", emoji::SPARKLE, started.elapsed().as_millis());
    Ok(())
}
