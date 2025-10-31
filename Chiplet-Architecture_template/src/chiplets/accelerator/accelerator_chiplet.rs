use super::accelerator_registers::AcceleratorRegisters;
use crate::chiplets::ChipletDescriptor;

#[derive(Debug, Clone)]
pub struct AcceleratorChiplet {
    descriptor: ChipletDescriptor,
    registers: AcceleratorRegisters,
}

impl AcceleratorChiplet {
    pub fn from_descriptor(descriptor: ChipletDescriptor) -> Self {
        Self {
            descriptor,
            registers: AcceleratorRegisters::default(),
        }
    }

    pub fn tile_count(&self) -> Option<u32> {
        self.descriptor
            .resources
            .get("tiles")
            .and_then(|value| value.as_u64())
            .map(|value| value as u32)
    }

    pub fn supported_precisions(&self) -> Vec<String> {
        match self.descriptor.resources.get("supported_precisions") {
            Some(value) => value
                .as_sequence()
                .map(|seq| {
                    seq.iter()
                        .filter_map(|item| item.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            None => Vec::new(),
        }
    }

    pub fn registers(&self) -> &AcceleratorRegisters {
        &self.registers
    }
}
