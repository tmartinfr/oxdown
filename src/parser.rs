use crate::models::{Article, ArticleDirectory};
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Deserialize)]
struct ArticleMetadata {
    comment_url: Option<String>,
}

pub fn parse_article(article_dir: &ArticleDirectory) -> Result<Article, io::Error> {
    let index_path = article_dir.path.join("index.md");
    let markdown_content = fs::read_to_string(&index_path)?;

    let title = extract_title(&markdown_content).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("No title found in article: {index_path:?}. Articles must start with a '# Title' heading."),
        )
    })?;

    // Parse markdown to HTML
    let parser = Parser::new_ext(&markdown_content, Options::all());
    let mut content_html = String::new();
    html::push_html(&mut content_html, parser);

    // Try to load optional index.json
    let comment_url = load_metadata(&article_dir.path)?;

    Ok(Article::new(
        article_dir.date,
        article_dir.slug.clone(),
        title,
        content_html,
        article_dir.static_files.clone(),
        comment_url,
    ))
}

fn extract_title(markdown: &str) -> Option<String> {
    let first_line = markdown.lines().next()?.trim();
    first_line
        .strip_prefix("# ")
        .map(|stripped| stripped.trim().to_string())
}

fn load_metadata(article_path: &std::path::Path) -> Result<Option<String>, io::Error> {
    let json_path = article_path.join("index.json");

    if !json_path.exists() {
        return Ok(None);
    }

    let json_content = fs::read_to_string(&json_path)?;
    let metadata: ArticleMetadata = serde_json::from_str(&json_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(metadata.comment_url)
}
