mod executor;
mod filter;
mod scanner;
mod watcher;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "r2")]
#[command(version, about = "Rerun commands when files change", long_about = None)]
struct Cli {
    /// Command to run when files change
    #[arg(help = "Shell command to execute (e.g., 'make', 'cargo test')")]
    command: String,

    /// Paths to monitor (files or directories)
    #[arg(
        required = true,
        num_args = 1..,
        help = "Files or directories to watch for changes"
    )]
    paths: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let files = scanner::gather_files(&cli.paths)
        .context("Failed to gather files to monitor")?;

    if files.is_empty() {
        eprintln!("Error: No files to monitor");
        std::process::exit(1);
    }

    let paths_str = cli.paths.join(" ");
    println!("r2 is watching about {} files:", files.len());
    println!("{}", paths_str);

    let (mut change_rx, _debouncer) = watcher::setup_watcher(files)
        .context("Failed to setup file watcher")?;

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    // Run command immediately on startup
    println!("FIGHT!");
    executor::execute_command(&cli.command)
        .await
        .context("Failed to execute command")?;
    println!("CONTINUE?");

    // Main event loop
    loop {
        tokio::select! {
            Some(_) = change_rx.recv() => {
                println!("FIGHT!");
                if let Err(e) = executor::execute_command(&cli.command).await {
                    eprintln!("Error executing command: {}", e);
                }
                println!("CONTINUE?");
            }
            _ = &mut ctrl_c => {
                println!("GAMEOVER");
                break;
            }
        }
    }

    Ok(())
}
