mod executor;
mod filter;
mod scanner;
mod ui;
mod watcher;

use anyhow::{Context, Result};
use clap::Parser;
use filter::IgnoreFilter;
use std::sync::Arc;

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
    // Parse CLI arguments
    let cli = Cli::parse();

    // Create ignore filter
    let filter = Arc::new(IgnoreFilter::new());

    // Gather files to monitor
    let files = scanner::gather_files(&cli.paths, &filter)
        .context("Failed to gather files to monitor")?;

    if files.is_empty() {
        eprintln!("Error: No files to monitor");
        std::process::exit(1);
    }

    // Print watching message
    let paths_str = cli.paths.join(" ");
    ui::print_watching(files.len(), &paths_str);

    // Setup file watcher
    let (mut change_rx, _debouncer) = watcher::setup_watcher(files, Arc::clone(&filter))
        .context("Failed to setup file watcher")?;

    // Setup Ctrl+C handler
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    // Run command immediately on startup (like Python version)
    ui::print_fight();
    executor::execute_command(&cli.command)
        .await
        .context("Failed to execute command")?;
    ui::print_continue();

    // Main event loop
    loop {
        tokio::select! {
            // File changed
            Some(_) = change_rx.recv() => {
                ui::print_fight();
                if let Err(e) = executor::execute_command(&cli.command).await {
                    eprintln!("Error executing command: {}", e);
                }
                ui::print_continue();
            }
            // Ctrl+C pressed
            _ = &mut ctrl_c => {
                ui::print_gameover();
                break;
            }
        }
    }

    Ok(())
}
