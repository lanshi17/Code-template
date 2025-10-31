pub mod chiplets;
pub mod interconnect;
pub mod system;
pub mod utils;

use chiplets::ChipletInventory;
use interconnect::communication::MessageBus;
use system::{resource_manager::ResourceManager, scheduler::Scheduler};
use utils::error::OrchestrationError;

/// Central object coordinating chiplet initialization and orchestration.
pub struct Platform {
    pub inventory: ChipletInventory,
    pub resource_manager: ResourceManager,
    pub scheduler: Scheduler,
    pub message_bus: MessageBus,
}

impl Platform {
    pub fn new(inventory: ChipletInventory) -> Self {
        let resource_manager = ResourceManager::new(&inventory);
        let scheduler = Scheduler::new();
        let message_bus = MessageBus::default();

        Self {
            inventory,
            resource_manager,
            scheduler,
            message_bus,
        }
    }

    /// Bootstrap the platform with default policies.
    pub fn bootstrap(&mut self) -> Result<(), OrchestrationError> {
        self.resource_manager.plan_resources()?;
        self.scheduler.configure_default_domains();
        self.message_bus
            .initialize_channels(self.inventory.chiplets.len() as u32)?;
        Ok(())
    }
}

/// Convenience helper that loads a chiplet inventory from YAML bytes.
pub fn load_inventory_from_yaml(data: &[u8]) -> Result<ChipletInventory, OrchestrationError> {
    ChipletInventory::from_yaml_bytes(data)
}
