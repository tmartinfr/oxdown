use serde::Deserialize;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub input_directory: PathBuf,
    #[serde(default = "default_output_directory")]
    pub output_directory: PathBuf,
    #[serde(default = "default_template_directory")]
    pub template_directory: String,
}

fn default_output_directory() -> PathBuf {
    PathBuf::from("dist")
}

fn default_template_directory() -> String {
    "templates/default".to_string()
}

impl Config {
    pub fn load_from_file(path: &Path) -> Result<Self, io::Error> {
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Config file not found: {}", path.display()),
            ));
        }

        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }
}
