use crate::models::Article;
use handlebars::Handlebars;
use serde_json::json;
use std::fs;
use std::io;
use std::path::Path;

/// Recursively copy a directory and its contents
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn generate_site(
    articles: &[Article],
    output_dir: &Path,
    template_directory: &str,
    author_name: Option<&str>,
    author_url: Option<&str>,
) -> Result<(), io::Error> {
    // Validate template directory exists
    let template_directory = Path::new(template_directory);
    if !template_directory.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Template directory does not exist: '{}'\nPlease check the 'template_directory' setting in your config file.",
                template_directory.display()
            ),
        ));
    }
    if !template_directory.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Template directory path is not a directory: '{}'\nPlease check the 'template_directory' setting in your config file.",
                template_directory.display()
            ),
        ));
    }

    // Validate required template files exist
    let required_files = ["base.hbs", "index.hbs", "article.hbs"];
    let mut missing_files = Vec::new();
    for file in &required_files {
        let file_path = template_directory.join(file);
        if !file_path.exists() {
            missing_files.push(*file);
        }
    }
    if !missing_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Template directory '{}' is missing required template files: {}\nPlease ensure your template directory contains all required files.",
                template_directory.display(),
                missing_files.join(", ")
            ),
        ));
    }

    // Validate required subdirectories exist
    let required_dirs = ["css", "js"];
    let mut missing_dirs = Vec::new();
    for dir in &required_dirs {
        let dir_path = template_directory.join(dir);
        if !dir_path.exists() {
            missing_dirs.push(*dir);
        } else if !dir_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Template directory '{}' contains a file named '{}' where a directory is expected.\nPlease ensure your template directory structure is correct.",
                    template_directory.display(),
                    dir
                ),
            ));
        }
    }
    if !missing_dirs.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Template directory '{}' is missing required subdirectories: {}\nPlease ensure your template directory contains all required subdirectories.",
                template_directory.display(),
                missing_dirs.join(", ")
            ),
        ));
    }

    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Initialize Handlebars registry
    let mut handlebars = Handlebars::new();

    // Copy static asset directories (CSS and JS)
    copy_dir_all(template_directory.join("css"), output_dir.join("css"))?;
    copy_dir_all(template_directory.join("js"), output_dir.join("js"))?;

    // Register templates
    handlebars
        .register_template_file("base", template_directory.join("base.hbs"))
        .map_err(io::Error::other)?;
    handlebars
        .register_template_file("index", template_directory.join("index.hbs"))
        .map_err(io::Error::other)?;
    handlebars
        .register_template_file("article", template_directory.join("article.hbs"))
        .map_err(io::Error::other)?;

    // Generate index page
    generate_index(articles, output_dir, &handlebars, author_name)?;

    // Generate individual article pages
    for article in articles {
        generate_article(article, output_dir, &handlebars, author_name, author_url)?;
    }

    Ok(())
}

fn generate_index(
    articles: &[Article],
    output_dir: &Path,
    handlebars: &Handlebars,
    author_name: Option<&str>,
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
                "title": author_name.unwrap_or("Articles"),
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
    author_name: Option<&str>,
    author_url: Option<&str>,
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
                "author_name": author_name,
                "author_url": author_url,
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
