//! Hardware abstraction module

pub mod abstract;
pub mod cpu;
pub mod gpu;
pub mod fpga;

use crate::hardware::abstract::{HardwareManager, DeviceManager};
use crate::hardware::cpu::CpuDevice;
use crate::hardware::gpu::GpuDevice;
use crate::hardware::fpga::FpgaDevice;
use std::sync::Arc;

/// Initialize the hardware manager
pub fn init_hardware_manager(config: &serde_yaml::Value) -> Result<Arc<dyn HardwareManager>, Box<dyn std::error::Error>> {
    let manager = DeviceManager::new();
    
    // Initialize CPU devices
    let cpu_device = CpuDevice::new("cpu-0".to_string(), "Intel Xeon".to_string());
    manager.register_device(Box::new(cpu_device))?;
    
    // Initialize GPU devices
    let gpu_device = GpuDevice::new("gpu-0".to_string(), "NVIDIA RTX 3090".to_string());
    manager.register_device(Box::new(gpu_device))?;
    
    // Initialize FPGA devices (disabled by default)
    // let fpga_device = FpgaDevice::new("fpga-0".to_string(), "Xilinx VCK5000".to_string());
    // manager.register_device(Box::new(fpga_device))?;
    
    Ok(Arc::new(manager))
}

pub use abstract::*;
pub use cpu::*;
pub use gpu::*;
pub use fpga::*;