use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrchestrationError {
    #[error("configuration error: {0}")]
    Configuration(String),
    #[error("chiplet not found: {0}")]
    ChipletMissing(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

impl OrchestrationError {
    pub fn config_missing(path: PathBuf) -> Self {
        Self::Configuration(format!("missing configuration file: {}", path.display()))
    }
}
