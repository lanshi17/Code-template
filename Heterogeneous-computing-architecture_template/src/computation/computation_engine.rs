// Computation engine implementation

use crate::hardware::abstract_device::DeviceManager;
use crate::scheduling::{TaskScheduler, ResourceAllocator, AllocationStrategy};
use crate::hardware::abstract_device::HardwareDevice;
use std::sync::Arc;

pub struct ComputationEngine {
    device_manager: DeviceManager,
    task_scheduler: TaskScheduler,
    resource_allocator: ResourceAllocator,
}

impl ComputationEngine {
    pub fn new(device_manager: DeviceManager, task_scheduler: TaskScheduler) -> Self {
        let resource_allocator = ResourceAllocator::new(AllocationStrategy::BestFit);
        Self {
            device_manager,
            task_scheduler,
            resource_allocator,
        }
    }

    pub async fn execute_task(&self, task: scheduling::Task) -> Result<computation::TaskResult, Box<dyn std::error::Error>> {
        // Get available devices
        let devices = self.device_manager.get_all_devices().await;
        let device_refs: Vec<&dyn HardwareDevice> = devices.iter().map(|d| d.as_ref()).collect();
        
        // Allocate resources for the task
        let requirements = &task.required_resources;
        let device = self.resource_allocator.allocate_resources(requirements, &device_refs);
        
        if let Some(device) = device {
            // Convert scheduling task to hardware task
            let hw_task = crate::hardware::abstract_device::Task {
                id: task.id,
                data: vec![], // In a real implementation, this would contain actual data
                operation: "compute".to_string(),
            };
            
            // Execute the task on the allocated device
            let result = device.execute_task(&hw_task).await?;
            
            // Convert hardware result to computation result
            let comp_result = computation::TaskResult {
                task_id: result.task_id,
                data: result.data,
                execution_time: result.execution_time,
            };
            
            Ok(comp_result)
        } else {
            Err("No suitable device found for task".into())
        }
    }
}