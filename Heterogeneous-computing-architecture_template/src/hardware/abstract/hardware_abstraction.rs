//! Abstract hardware interface for heterogeneous computing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Hardware type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HardwareType {
    Cpu,
    Gpu,
    Fpga,
}

impl fmt::Display for HardwareType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HardwareType::Cpu => write!(f, "CPU"),
            HardwareType::Gpu => write!(f, "GPU"),
            HardwareType::Fpga => write!(f, "FPGA"),
        }
    }
}

/// Base trait for all hardware devices
pub trait HardwareDevice: Send + Sync {
    /// Get the hardware type
    fn get_type(&self) -> HardwareType;
    
    /// Get device identifier
    fn get_id(&self) -> String;
    
    /// Get device name
    fn get_name(&self) -> String;
    
    /// Get device properties
    fn get_properties(&self) -> HashMap<String, String>;
    
    /// Check if device is available
    fn is_available(&self) -> bool;
    
    /// Initialize the device
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get device metrics
    fn get_metrics(&self) -> HardwareMetrics;
}

/// Hardware metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMetrics {
    /// Current utilization percentage
    pub utilization: f64,
    
    /// Temperature in Celsius
    pub temperature: f64,
    
    /// Memory usage in MB
    pub memory_used: u64,
    
    /// Total memory in MB
    pub memory_total: u64,
    
    /// Clock speed in MHz
    pub clock_speed: u64,
    
    /// Status of the device
    pub status: DeviceStatus,
}

/// Device status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DeviceStatus {
    Idle,
    Active,
    Busy,
    Error,
    Offline,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceStatus::Idle => write!(f, "idle"),
            DeviceStatus::Active => write!(f, "active"),
            DeviceStatus::Busy => write!(f, "busy"),
            DeviceStatus::Error => write!(f, "error"),
            DeviceStatus::Offline => write!(f, "offline"),
        }
    }
}

/// Hardware manager trait
pub trait HardwareManager: Send + Sync {
    /// Get all available hardware devices
    fn get_all_devices(&self) -> Vec<Box<dyn HardwareDevice>>;
    
    /// Get devices of a specific type
    fn get_devices_by_type(&self, device_type: HardwareType) -> Vec<Box<dyn HardwareDevice>>;
    
    /// Get a specific device by ID
    fn get_device_by_id(&self, id: &str) -> Option<Box<dyn HardwareDevice>>;
    
    /// Allocate resources for a task
    fn allocate_resources(&self, task_requirements: &TaskRequirements) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    
    /// Release allocated resources
    fn release_resources(&self, resource_ids: &[String]) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get hardware metrics
    fn get_hardware_metrics(&self) -> Vec<HardwareMetrics>;
}

/// Task requirements structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequirements {
    /// Required hardware type
    pub hardware_type: HardwareType,
    
    /// Required memory in MB
    pub memory_required: u64,
    
    /// Required compute units
    pub compute_units: u64,
    
    /// Priority level
    pub priority: u8,
    
    /// Timeout in milliseconds
    pub timeout: u64,
}