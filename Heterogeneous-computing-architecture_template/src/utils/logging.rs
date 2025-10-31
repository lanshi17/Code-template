// Logging utilities

use tracing_subscriber::{fmt, EnvFilter};

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    fmt()
        .with_env_filter(filter)
        .init();
    
    tracing::info!("Logging initialized");
    Ok(())
}

pub fn log_info(message: &str) {
    tracing::info!("{}", message);
}

pub fn log_error(message: &str) {
    tracing::error!("{}", message);
}

pub fn log_debug(message: &str) {
    tracing::debug!("{}", message);
}