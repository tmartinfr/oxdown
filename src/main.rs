mod config;
mod finder;
mod generator;
mod models;
mod parser;

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

    // Find article directories
    let article_dirs = match finder::find_article_directories(&config.input_directory) {
        Ok(dirs) => dirs,
        Err(e) => {
            eprintln!("Error reading input directory: {e}");
            process::exit(1);
        }
    };

    println!("Found {} article(s)", article_dirs.len());

    // Parse articles
    let mut articles = Vec::new();
    for article_dir in article_dirs {
        match parser::parse_article(&article_dir) {
            Ok(article) => {
                println!("  - {}: {}", article.date, article.title);
                articles.push(article);
            }
            Err(e) => {
                eprintln!(
                    "Warning: Failed to parse article in {:?}: {}",
                    article_dir.path, e
                );
            }
        }
    }

    // Generate site
    if let Err(e) = generator::generate_site(
        &articles,
        &config.output_directory,
        &config.template_directory,
    ) {
        eprintln!("Error generating site: {e}");
        process::exit(1);
    }

    println!(
        "\nSite generated successfully in {:?}",
        config.output_directory
    );
}
