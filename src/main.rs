mod config;
mod finder;
mod generator;
mod models;
mod parser;

use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "oxdown")]
#[command(about = "Opinionated static site generator for markdown articles", long_about = None)]
struct Cli {
    /// Input directory containing article directories
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output directory for generated site
    #[arg(short, long, value_name = "OUTPUT", default_value = "dist")]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Find article directories
    let article_dirs = match finder::find_article_directories(&cli.input) {
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
    if let Err(e) = generator::generate_site(&articles, &cli.output) {
        eprintln!("Error generating site: {e}");
        process::exit(1);
    }

    println!("\nSite generated successfully in {:?}", cli.output);
}
