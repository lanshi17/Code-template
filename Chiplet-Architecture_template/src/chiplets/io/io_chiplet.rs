use super::io_registers::IoRegisters;
use crate::chiplets::ChipletDescriptor;

#[derive(Debug, Clone)]
pub struct IoChiplet {
    descriptor: ChipletDescriptor,
    registers: IoRegisters,
}

impl IoChiplet {
    pub fn from_descriptor(descriptor: ChipletDescriptor) -> Self {
        Self {
            descriptor,
            registers: IoRegisters::default(),
        }
    }

    pub fn pcie_lane_count(&self) -> Option<u32> {
        self.descriptor
            .resources
            .get("pcie_lanes")
            .and_then(|value| value.as_u64())
            .map(|value| value as u32)
    }

    pub fn supports_cxl(&self) -> bool {
        self.descriptor
            .resources
            .get("cxl_supported")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }

    pub fn registers(&self) -> &IoRegisters {
        &self.registers
    }
}
