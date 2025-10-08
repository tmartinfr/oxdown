use chrono::NaiveDate;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ArticleDirectory {
    pub path: PathBuf,
    pub date: NaiveDate,
    pub slug: String,
    pub static_files: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub date: NaiveDate,
    pub slug: String,
    pub title: String,
    pub content_html: String,
    pub static_files: Vec<PathBuf>,
    pub comment_url: Option<String>,
}

impl Article {
    pub fn new(
        date: NaiveDate,
        slug: String,
        title: String,
        content_html: String,
        static_files: Vec<PathBuf>,
        comment_url: Option<String>,
    ) -> Self {
        Self {
            date,
            slug,
            title,
            content_html,
            static_files,
            comment_url,
        }
    }

    pub fn url_path(&self) -> String {
        format!("{}-{}", self.date.format("%Y-%m-%d"), self.slug)
    }
}
