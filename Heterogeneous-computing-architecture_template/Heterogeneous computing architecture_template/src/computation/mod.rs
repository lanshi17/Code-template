//! Computation module

pub mod computation_engine;
pub mod task_executor;

use crate::computation::computation_engine::ComputationEngine;
use crate::hardware::abstract::HardwareManager;
use std::sync::Arc;

/// Initialize the computation engine
pub fn init_computation_engine(config: &serde_yaml::Value) -> Result<Arc<ComputationEngine>, Box<dyn std::error::Error>> {
    // In a real application, this would load configuration and create proper engine
    // For now, we create a simple mock engine
    
    let computation_engine = ComputationEngine::new(
        // This would normally be provided by the hardware manager
        // For now we'll create a mock to satisfy compilation
        Arc::new(crate::hardware::init_hardware_manager(config)?)
    );
    
    Ok(Arc::new(computation_engine))
}

pub use computation_engine::*;
pub use task_executor::*;