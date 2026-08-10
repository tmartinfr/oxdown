use crate::config::Config;
use crate::finder;
use crate::generator;
use crate::parser;
use std::io;

/// Find, parse and generate the whole site from the given config
pub fn build_site(config: &Config) -> Result<(), io::Error> {
    // Find article directories
    let article_dirs = finder::find_article_directories(&config.input_directory)
        .map_err(|e| io::Error::new(e.kind(), format!("reading input directory: {e}")))?;

    println!("Found {} article(s)", article_dirs.len());

    // Parse articles
    let mut articles = Vec::new();
    for article_dir in article_dirs.iter().rev() {
        match parser::parse_article(article_dir) {
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
    // Restore newest-first order for site generation
    articles.reverse();

    // Generate site
    generator::generate_site(
        &articles,
        &config.output_directory,
        &config.template_directory,
        config.author_name.as_deref(),
        config.author_url.as_deref(),
    )
    .map_err(|e| io::Error::new(e.kind(), format!("generating site: {e}")))?;

    println!(
        "\nSite generated successfully in {:?}",
        config.output_directory
    );

    Ok(())
}
