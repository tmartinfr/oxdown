mod builder;
mod config;
mod finder;
mod generator;
mod models;
mod parser;
mod watcher;

use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "oxdown")]
#[command(about = "Opinionated static site generator for markdown articles", long_about = None)]
struct Cli {
    /// Path to config file (or set OXDOWN_CONFIG environment variable)
    #[arg(value_name = "CONFIG")]
    config: Option<PathBuf>,

    /// Keep running and rebuild the site when files change
    #[arg(short, long)]
    watch: bool,
}

fn main() {
    let cli = Cli::parse();

    // Determine config file path from CLI arg or environment variable
    let config_path = cli
        .config
        .or_else(|| env::var("OXDOWN_CONFIG").ok().map(PathBuf::from));

    let config_path = match config_path {
        Some(path) => path,
        None => {
            eprintln!("Error: No config file specified.");
            eprintln!(
                "Provide a config file as the first argument or set OXDOWN_CONFIG environment variable."
            );
            eprintln!("\nUsage: oxdown <CONFIG>");
            eprintln!("   or: OXDOWN_CONFIG=config.json oxdown");
            process::exit(1);
        }
    };

    // Load config
    let config = match config::Config::load_from_file(&config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!(
                "Error loading config file '{}': {}",
                config_path.display(),
                e
            );
            process::exit(1);
        }
    };

    println!("Using config from: {}", config_path.display());
    println!("Input directory: {}", config.input_directory.display());
    println!("Output directory: {}", config.output_directory.display());

    // Build the site once, then keep going in watch mode even if it failed
    if let Err(e) = builder::build_site(&config) {
        eprintln!("Error {e}");
        if !cli.watch {
            process::exit(1);
        }
    }

    if cli.watch {
        println!();
        if let Err(e) = watcher::watch(&config_path, config) {
            eprintln!("Error watching for changes: {e}");
            process::exit(1);
        }
    }
}
