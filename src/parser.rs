use crate::models::{Article, ArticleDirectory};
use pulldown_cmark::{Options, Parser, html};
use std::fs;
use std::io;

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

    Ok(Article::new(
        article_dir.date,
        article_dir.slug.clone(),
        title,
        content_html,
        article_dir.static_files.clone(),
    ))
}

fn extract_title(markdown: &str) -> Option<String> {
    let first_line = markdown.lines().next()?.trim();
    first_line
        .strip_prefix("# ")
        .map(|stripped| stripped.trim().to_string())
}
