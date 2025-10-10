use crate::models::Article;
use handlebars::Handlebars;
use serde_json::json;
use std::fs;
use std::io;
use std::path::Path;

pub fn generate_site(
    articles: &[Article],
    output_dir: &Path,
    template_dir: &str,
) -> Result<(), io::Error> {
    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Initialize Handlebars registry
    let mut handlebars = Handlebars::new();
    let template_dir = Path::new(template_dir);

    // Copy static assets (CSS and JS)
    fs::copy(template_dir.join("style.css"), output_dir.join("style.css"))?;
    fs::copy(
        template_dir.join("dark-mode.js"),
        output_dir.join("dark-mode.js"),
    )?;

    // Register templates
    handlebars
        .register_template_file("base", template_dir.join("base.hbs"))
        .map_err(io::Error::other)?;
    handlebars
        .register_template_file("index", template_dir.join("index.hbs"))
        .map_err(io::Error::other)?;
    handlebars
        .register_template_file("article", template_dir.join("article.hbs"))
        .map_err(io::Error::other)?;

    // Generate index page
    generate_index(articles, output_dir, &handlebars)?;

    // Generate individual article pages
    for article in articles {
        generate_article(article, output_dir, &handlebars)?;
    }

    Ok(())
}

fn generate_index(
    articles: &[Article],
    output_dir: &Path,
    handlebars: &Handlebars,
) -> Result<(), io::Error> {
    // Prepare article data for template
    let article_data: Vec<_> = articles
        .iter()
        .map(|article| {
            json!({
                "date": article.date.format("%Y-%m-%d").to_string(),
                "url": format!("/{}/", article.url_path()),
                "title": &article.title,
            })
        })
        .collect();

    // Render index content
    let index_content = handlebars
        .render(
            "index",
            &json!({
                "articles": article_data,
            }),
        )
        .map_err(io::Error::other)?;

    // Render full page with base layout
    let html = handlebars
        .render(
            "base",
            &json!({
                "title": "Blog",
                "content": index_content,
            }),
        )
        .map_err(io::Error::other)?;

    fs::write(output_dir.join("index.html"), html)?;
    Ok(())
}

fn generate_article(
    article: &Article,
    output_dir: &Path,
    handlebars: &Handlebars,
) -> Result<(), io::Error> {
    let article_dir = output_dir.join(article.url_path());
    fs::create_dir_all(&article_dir)?;

    // Copy static files
    for static_file in &article.static_files {
        if let Some(filename) = static_file.file_name() {
            let dest = article_dir.join(filename);
            fs::copy(static_file, dest)?;
        }
    }

    // Render article content
    let article_content = handlebars
        .render(
            "article",
            &json!({
                "content": &article.content_html,
                "comment_url": &article.comment_url,
            }),
        )
        .map_err(io::Error::other)?;

    // Render full page with base layout
    let html = handlebars
        .render(
            "base",
            &json!({
                "title": &article.title,
                "content": article_content,
            }),
        )
        .map_err(io::Error::other)?;

    fs::write(article_dir.join("index.html"), html)?;
    Ok(())
}
