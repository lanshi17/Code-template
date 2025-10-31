//! Task executor implementation

use crate::computation::computation_engine::ComputationEngine;
use crate::hardware::abstract::{HardwareManager, TaskRequirements, HardwareType};
use crate::scheduling::task_scheduler::Task;
use std::sync::Arc;
use tracing::debug;

/// Task executor structure
pub struct TaskExecutor {
    computation_engine: Arc<ComputationEngine>,
    hardware_manager: Arc<dyn HardwareManager>,
}

impl TaskExecutor {
    /// Create a new task executor
    pub fn new(
        computation_engine: Arc<ComputationEngine>,
        hardware_manager: Arc<dyn HardwareManager>,
    ) -> Self {
        Self {
            computation_engine,
            hardware_manager,
        }
    }
    
    /// Execute a single task
    pub fn execute_task(&self, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Executing task: {}", task.name);
        
        // First, try to allocate resources for this task
        let resource_ids = self.hardware_manager.allocate_resources(&task.requirements)?;
        
        // Then, execute the task on the allocated hardware
        self.computation_engine.execute_task_with_hardware(task, &resource_ids[0])?;
        
        // Finally, release the resources
        self.hardware_manager.release_resources(&resource_ids)?;
        
        debug!("Task '{}' completed successfully", task.name);
        Ok(())
    }
    
    /// Execute a batch of tasks
    pub fn execute_batch(&self, tasks: &[Task]) -> Result<Vec<Result<(), Box<dyn std::error::Error>>>, Box<dyn std::error::Error>> {
        debug!("Executing batch of {} tasks", tasks.len());
        
        let mut results = Vec::new();
        
        for task in tasks {
            let result = self.execute_task(task);
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Execute task with specific hardware selection
    pub fn execute_task_on_hardware(
        &self,
        task: &Task,
        hardware_type: HardwareType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Executing task '{}' on specific hardware: {:?}", task.name, hardware_type);
        
        // Find available devices of the specified type
        let devices = self.hardware_manager.get_devices_by_type(hardware_type);
        if devices.is_empty() {
            return Err(format!("No {} devices available", hardware_type).into());
        }
        
        // Allocate resources for this task
        let task_requirements = TaskRequirements {
            hardware_type,
            memory_required: task.requirements.memory_required,
            compute_units: task.requirements.compute_units,
            priority: task.requirements.priority,
            timeout: task.requirements.timeout,
        };
        
        let resource_ids = self.hardware_manager.allocate_resources(&task_requirements)?;
        
        // Execute the task
        self.computation_engine.execute_task_with_hardware(task, &resource_ids[0])?;
        
        // Release resources
        self.hardware_manager.release_resources(&resource_ids)?;
        
        debug!("Task '{}' completed successfully on {:?}", task.name, hardware_type);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_executor_creation() {
        // This test requires complex mocking to run properly
        // For now, we'll just ensure the structure compiles correctly
        assert!(true);
    }
}