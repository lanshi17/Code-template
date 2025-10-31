//! Device manager for hardware abstraction

use crate::hardware::abstract::{HardwareDevice, HardwareManager, HardwareType, TaskRequirements, HardwareMetrics};
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::debug;

/// Concrete implementation of HardwareManager
pub struct DeviceManager {
    devices: Mutex<HashMap<String, Box<dyn HardwareDevice>>>,
}

impl DeviceManager {
    /// Create a new DeviceManager
    pub fn new() -> Self {
        Self {
            devices: Mutex::new(HashMap::new()),
        }
    }
    
    /// Register a new hardware device
    pub fn register_device(&self, device: Box<dyn HardwareDevice>) -> Result<(), Box<dyn std::error::Error>> {
        let device_id = device.get_id();
        debug!("Registering device: {} ({})", device.get_name(), device.get_type());
        
        self.devices.lock().unwrap().insert(device_id, device);
        Ok(())
    }
    
    /// Unregister a hardware device
    pub fn unregister_device(&self, device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Unregistering device: {}", device_id);
        
        self.devices.lock().unwrap().remove(device_id);
        Ok(())
    }
}

impl HardwareManager for DeviceManager {
    fn get_all_devices(&self) -> Vec<Box<dyn HardwareDevice>> {
        self.devices.lock().unwrap().values().cloned().collect()
    }
    
    fn get_devices_by_type(&self, device_type: HardwareType) -> Vec<Box<dyn HardwareDevice>> {
        self.devices
            .lock()
            .unwrap()
            .values()
            .filter(|device| device.get_type() == device_type)
            .cloned()
            .collect()
    }
    
    fn get_device_by_id(&self, id: &str) -> Option<Box<dyn HardwareDevice>> {
        self.devices.lock().unwrap().get(id).cloned()
    }
    
    fn allocate_resources(&self, task_requirements: &TaskRequirements) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        debug!(
            "Allocating resources for task requiring {:?} with {} MB memory",
            task_requirements.hardware_type,
            task_requirements.memory_required
        );
        
        // Find available devices matching the requirements
        let mut available_devices = self.get_devices_by_type(task_requirements.hardware_type);
        
        // Sort by availability and utilization
        available_devices.sort_by_key(|device| {
            device.get_metrics().utilization
        });
        
        // Select one device for this task
        if let Some(device) = available_devices.first() {
            let device_id = device.get_id();
            debug!("Allocated device {} for task", device_id);
            Ok(vec![device_id])
        } else {
            Err("No suitable device found".into())
        }
    }
    
    fn release_resources(&self, resource_ids: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Releasing resources: {:?}", resource_ids);
        // In a real implementation, this would actually free up the resources
        Ok(())
    }
    
    fn get_hardware_metrics(&self) -> Vec<HardwareMetrics> {
        self.devices
            .lock()
            .unwrap()
            .values()
            .map(|device| device.get_metrics())
            .collect()
    }
}