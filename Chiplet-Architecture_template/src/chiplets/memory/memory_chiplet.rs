use super::memory_registers::MemoryRegisters;
use crate::chiplets::ChipletDescriptor;

#[derive(Debug, Clone)]
pub struct MemoryChiplet {
    descriptor: ChipletDescriptor,
    registers: MemoryRegisters,
}

impl MemoryChiplet {
    pub fn from_descriptor(descriptor: ChipletDescriptor) -> Self {
        Self {
            descriptor,
            registers: MemoryRegisters::default(),
        }
    }

    pub fn capacity_gb(&self) -> Option<u32> {
        self.descriptor
            .resources
            .get("capacity_gb")
            .and_then(|value| value.as_u64())
            .map(|value| value as u32)
    }

    pub fn channels(&self) -> Option<u8> {
        self.descriptor
            .resources
            .get("channels")
            .and_then(|value| value.as_u64())
            .map(|value| value as u8)
    }

    pub fn registers(&self) -> &MemoryRegisters {
        &self.registers
    }
}
