// Hardware errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HardwareError {
    #[error("Device initialization failed: {0}")]
    InitializationError(String),
    
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    
    #[error("Insufficient memory: required {required}, available {available}")]
    InsufficientMemory { required: u64, available: u64 },
    
    #[error("Device unavailable: {0}")]
    DeviceUnavailable(String),
    
    #[error("Execution failed: {0}")]
    ExecutionError(String),
    
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}