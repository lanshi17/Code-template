// Integration tests for the heterogeneous computing template

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::cpu::CpuDevice;
    use crate::hardware::abstract_device::HardwareDevice;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_cpu_device_execution() {
        // Create a CPU device
        let cpu = Arc::new(CpuDevice::new(
            "test-cpu".to_string(),
            4,      // 4 cores
            3200,   // 3.2 GHz
            8192,   // 8 GB memory
        ));

        // Create a test task
        let task = crate::hardware::abstract_device::Task {
            id: "test-task".to_string(),
            data: vec![1, 2, 3, 4],
            operation: "add".to_string(),
        };

        // Execute the task
        let result = cpu.execute_task(&task).await.unwrap();

        // Verify the result
        assert_eq!(result.task_id, "test-task");
        assert_eq!(result.execution_time, 100);
    }

    #[tokio::test]
    async fn test_device_manager() {
        // Create a device manager
        let device_manager = crate::hardware::abstract_device::DeviceManager::new();

        // Create a CPU device
        let cpu = Arc::new(CpuDevice::new(
            "test-cpu".to_string(),
            4,      // 4 cores
            3200,   // 3.2 GHz
            8192,   // 8 GB memory
        ));

        // Add the device to the manager
        device_manager.add_device(cpu.clone()).await;

        // Retrieve the device
        let retrieved_device = device_manager.get_device("test-cpu").await.unwrap();

        // Verify the device
        assert_eq!(retrieved_device.id(), "test-cpu");
        assert_eq!(retrieved_device.device_type(), "CPU");
    }
}