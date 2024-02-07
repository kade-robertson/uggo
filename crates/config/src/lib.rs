use std::path::PathBuf;

use config_better::Config as CBConfig;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Could not create app directories.")]
    CouldNotMakeDirs,
}

#[derive(Clone)]
pub struct Config {
    inner: CBConfig,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let config = CBConfig::new("uggo");

        config
            .create_all()
            .map_err(|_| ConfigError::CouldNotMakeDirs)?;

        Ok(Self { inner: config })
    }

    pub fn cache(&self) -> &PathBuf {
        &self.inner.cache.path
    }
}

#[cfg(any(feature = "async", feature = "async-tokio"))]
impl Config {
    pub async fn new_async() -> Result<Self, ConfigError> {
        let config = CBConfig::new("uggo");

        config
            .create_all_async()
            .await
            .map_err(|_| ConfigError::CouldNotMakeDirs)?;

        Ok(Self { inner: config })
    }
}
