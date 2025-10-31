//! Scheduling module

pub mod task_scheduler;
pub mod resource_allocator;

use crate::scheduling::task_scheduler::TaskScheduler;
use crate::scheduling::resource_allocator::ResourceAllocator;
use crate::hardware::abstract::HardwareManager;
use std::sync::Arc;

/// Initialize the scheduler
pub fn init_scheduler(config: &serde_yaml::Value) -> Result<Arc<TaskScheduler>, Box<dyn std::error::Error>> {
    // In a real application, this would load configuration and create proper scheduler
    // For now, we create a simple mock scheduler
    
    let resource_allocator = Arc::new(ResourceAllocator::new(
        // This would normally be provided by the hardware manager
        // For now we'll create a mock to satisfy compilation
        Arc::new(crate::hardware::init_hardware_manager(config)?)
    ));
    
    let scheduler = TaskScheduler::new(resource_allocator);
    Ok(Arc::new(scheduler))
}

pub use task_scheduler::*;
pub use resource_allocator::*;