use super::core_registers::CoreRegisters;
use crate::chiplets::ChipletDescriptor;

#[derive(Debug, Clone)]
pub struct CoreChiplet {
    descriptor: ChipletDescriptor,
    registers: CoreRegisters,
}

impl CoreChiplet {
    pub fn from_descriptor(descriptor: ChipletDescriptor) -> Self {
        Self {
            descriptor,
            registers: CoreRegisters::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.descriptor.name
    }

    pub fn vendor(&self) -> Option<&str> {
        self.descriptor.vendor.as_deref()
    }

    pub fn threads_per_core(&self) -> Option<u32> {
        self.descriptor
            .resources
            .get("threads_per_core")
            .and_then(|value| value.as_u64())
            .map(|value| value as u32)
    }

    pub fn registers(&self) -> &CoreRegisters {
        &self.registers
    }
}
