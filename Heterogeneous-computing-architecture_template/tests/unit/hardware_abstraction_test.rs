// Unit tests for hardware abstraction

#[cfg(test)]
mod tests {
    use crate::hardware::abstract_device::{HardwareDevice, Task, TaskResult};
    use crate::errors::HardwareError;
    use async_trait::async_trait;

    // Mock hardware device for testing
    struct MockDevice {
        id: String,
        device_type: String,
        memory: u64,
        available: bool,
    }

    #[async_trait]
    impl HardwareDevice for MockDevice {
        fn id(&self) -> &str {
            &self.id
        }

        fn device_type(&self) -> &str {
            &self.device_type
        }

        fn available_memory(&self) -> u64 {
            self.memory
        }

        fn total_memory(&self) -> u64 {
            self.memory
        }

        async fn execute_task(&self, _task: &Task) -> Result<TaskResult, HardwareError> {
            if self.available {
                Ok(TaskResult {
                    task_id: "mock-task".to_string(),
                    data: vec![],
                    execution_time: 50,
                })
            } else {
                Err(HardwareError::DeviceUnavailable("Device is not available".to_string()))
            }
        }

        fn is_available(&self) -> bool {
            self.available
        }
    }

    #[tokio::test]
    async fn test_hardware_device_trait() {
        let device = MockDevice {
            id: "mock-1".to_string(),
            device_type: "MOCK".to_string(),
            memory: 1024,
            available: true,
        };

        assert_eq!(device.id(), "mock-1");
        assert_eq!(device.device_type(), "MOCK");
        assert_eq!(device.available_memory(), 1024);
        assert_eq!(device.total_memory(), 1024);
        assert!(device.is_available());

        let task = Task {
            id: "test-task".to_string(),
            data: vec![],
            operation: "test".to_string(),
        };

        let result = device.execute_task(&task).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.task_id, "mock-task");
        assert_eq!(result.execution_time, 50);
    }

    #[tokio::test]
    async fn test_hardware_device_unavailable() {
        let device = MockDevice {
            id: "mock-2".to_string(),
            device_type: "MOCK".to_string(),
            memory: 1024,
            available: false,
        };

        assert!(!device.is_available());

        let task = Task {
            id: "test-task".to_string(),
            data: vec![],
            operation: "test".to_string(),
        };

        let result = device.execute_task(&task).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            HardwareError::DeviceUnavailable(msg) => {
                assert_eq!(msg, "Device is not available");
            }
            _ => panic!("Expected DeviceUnavailable error"),
        }
    }
}