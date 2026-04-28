use crate::models::ArticleDirectory;
use chrono::NaiveDate;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn find_article_directories(root_path: &Path) -> Result<Vec<ArticleDirectory>, std::io::Error> {
    let pattern = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})-([a-z0-9\-]+)$").unwrap();
    let mut articles = Vec::new();

    for entry in fs::read_dir(root_path)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let dir_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => continue,
        };

        if let Some(captures) = pattern.captures(&dir_name) {
            let year: i32 = captures[1].parse().unwrap();
            let month: u32 = captures[2].parse().unwrap();
            let day: u32 = captures[3].parse().unwrap();
            let slug = captures[4].to_string();

            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let index_path = path.join("index.md");
                if index_path.exists() {
                    let static_files = collect_static_files(&path);
                    articles.push(ArticleDirectory {
                        path,
                        date,
                        slug,
                        static_files,
                    });
                }
            }
        }
    }

    // Sort by date, newest first
    articles.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(articles)
}

fn collect_static_files(article_dir: &Path) -> Vec<std::path::PathBuf> {
    let mut static_files = Vec::new();

    if let Ok(entries) = fs::read_dir(article_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Skip index.md and directories
            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if filename != "index.md" {
                        static_files.push(path);
                    }
                }
            }
        }
    }

    static_files
}
