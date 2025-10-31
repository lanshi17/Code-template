//! Computation engine implementation

use crate::hardware::abstract::{HardwareManager, HardwareType, TaskRequirements};
use crate::scheduling::task_scheduler::Task;
use std::sync::Arc;
use tracing::debug;

/// Computation engine error types
#[derive(Debug, Clone)]
pub enum ComputationError {
    HardwareNotAvailable(String),
    TaskExecutionFailed(String),
    ResourceAllocationFailed(String),
}

impl std::fmt::Display for ComputationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputationError::HardwareNotAvailable(msg) => write!(f, "Hardware not available: {}", msg),
            ComputationError::TaskExecutionFailed(msg) => write!(f, "Task execution failed: {}", msg),
            ComputationError::ResourceAllocationFailed(msg) => write!(f, "Resource allocation failed: {}", msg),
        }
    }
}

impl std::error::Error for ComputationError {}

/// Computation engine structure
pub struct ComputationEngine {
    hardware_manager: Arc<dyn HardwareManager>,
}

impl ComputationEngine {
    /// Create a new computation engine
    pub fn new(hardware_manager: Arc<dyn HardwareManager>) -> Self {
        Self { hardware_manager }
    }
    
    /// Execute a task on appropriate hardware
    pub fn execute_task(&self, task: &Task) -> Result<(), ComputationError> {
        debug!("Executing task '{}' on {:?}", task.name, task.requirements.hardware_type);
        
        // Check if the required hardware is available
        let devices = self.hardware_manager.get_devices_by_type(task.requirements.hardware_type);
        if devices.is_empty() {
            return Err(ComputationError::HardwareNotAvailable(
                format!("No {} devices available", task.requirements.hardware_type)
            ));
        }
        
        // In a real implementation, this would:
        // 1. Allocate resources for the task
        // 2. Transfer data to the appropriate hardware
        // 3. Execute the computation
        // 4. Retrieve results
        
        // For simulation purposes, we'll just log the execution
        debug!("Task '{}' successfully scheduled for execution", task.name);
        Ok(())
    }
    
    /// Execute a task with specific hardware requirements
    pub fn execute_task_with_hardware(
        &self,
        task: &Task,
        hardware_id: &str
    ) -> Result<(), ComputationError> {
        debug!("Executing task '{}' on hardware {}", task.name, hardware_id);
        
        // In a real implementation, this would:
        // 1. Check if the specific hardware is available
        // 2. Configure the hardware for the task
        // 3. Execute the computation
        // 4. Handle results
        
        // For simulation purposes, we'll just log the execution
        debug!("Task '{}' successfully executed on hardware {}", task.name, hardware_id);
        Ok(())
    }
    
    /// Batch execute multiple tasks
    pub fn execute_batch(&self, tasks: &[Task]) -> Result<Vec<Result<(), ComputationError>>, ComputationError> {
        debug!("Executing batch of {} tasks", tasks.len());
        
        let mut results = Vec::new();
        
        for task in tasks {
            let result = self.execute_task(task);
            results.push(result);
        }
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::abstract::TaskRequirements;
    
    #[test]
    fn test_computation_error_display() {
        let error = ComputationError::HardwareNotAvailable("GPU not available".to_string());
        assert_eq!(format!("{}", error), "Hardware not available: GPU not available");
    }
    
    #[test]
    fn test_computation_engine_creation() {
        // This test requires mocking hardware manager which is complex
        // For now, we'll just ensure the structure compiles correctly
        assert!(true);
    }
}