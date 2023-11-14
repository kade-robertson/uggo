use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct Config {
    cache_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            cache_dir: match env::var("XDG_CACHE_HOME") {
                Ok(dir) => Path::new(&dir).join("uggo"),
                Err(_) => match env::consts::OS {
                    "windows" => {
                        Path::new(&env::var("APPDATA").unwrap_or_else(|_| ".".to_string()))
                            .join("uggo")
                            .join("Cache")
                    }
                    "macos" => Path::new(&env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                        .join("Library")
                        .join("Caches")
                        .join("uggo"),
                    _ => Path::new(&env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                        .join(".cache")
                        .join("uggo"),
                },
            },
        }
    }

    pub fn cache(&self) -> &str {
        if !&self.cache_dir.exists() {
            let _result = fs::create_dir_all(self.cache_dir.as_path()).ok();
        }
        self.cache_dir.to_str().unwrap()
    }
}
