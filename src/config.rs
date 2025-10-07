pub struct Config {
    pub template_dir: &'static str,
}

impl Config {
    pub const fn default() -> Self {
        Self {
            template_dir: "templates/default",
        }
    }
}

pub const CONFIG: Config = Config::default();
