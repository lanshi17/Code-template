// Task executor implementation

use crate::hardware::abstract_device::HardwareDevice;
use crate::scheduling::Task;

pub struct TaskExecutor;

impl TaskExecutor {
    pub async fn execute_task(device: &dyn HardwareDevice, task: &Task) -> Result<computation::TaskResult, crate::errors::HardwareError> {
        // Convert scheduling task to hardware task
        let hw_task = crate::hardware::abstract_device::Task {
            id: task.id.clone(),
            data: vec![], // In a real implementation, this would contain actual data
            operation: "compute".to_string(),
        };
        
        // Execute the task on the device
        let result = device.execute_task(&hw_task).await?;
        
        // Convert hardware result to computation result
        let comp_result = computation::TaskResult {
            task_id: result.task_id,
            data: result.data,
            execution_time: result.execution_time,
        };
        
        Ok(comp_result)
    }
}