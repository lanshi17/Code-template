use crate::chiplets::{ChipletCategory, ChipletInventory};
use crate::utils::error::OrchestrationError;

#[derive(Debug, Default, Clone)]
pub struct ResourcePlan {
    pub compute_chiplets: usize,
    pub memory_chiplets: usize,
    pub io_chiplets: usize,
    pub accelerator_chiplets: usize,
}

#[derive(Debug, Default)]
pub struct ResourceManager {
    plan: Option<ResourcePlan>,
}

impl ResourceManager {
    pub fn new(inventory: &ChipletInventory) -> Self {
        let mut manager = Self { plan: None };
        manager.ingest_inventory(inventory);
        manager
    }

    pub fn plan_resources(&mut self) -> Result<(), OrchestrationError> {
        if self.plan.is_some() {
            return Ok(());
        }

        // Placeholder plan until real resource policies are implemented.
        self.plan = Some(ResourcePlan::default());
        Ok(())
    }

    pub fn ingest_inventory(&mut self, inventory: &ChipletInventory) {
        let mut plan = ResourcePlan::default();
        for chiplet in &inventory.chiplets {
            match chiplet.category {
                ChipletCategory::GeneralPurpose => plan.compute_chiplets += 1,
                ChipletCategory::Hbm2e => plan.memory_chiplets += 1,
                ChipletCategory::PcieRootComplex => plan.io_chiplets += 1,
                ChipletCategory::MatrixEngine => plan.accelerator_chiplets += 1,
                ChipletCategory::Unknown => {}
            }
        }
        self.plan = Some(plan);
    }

    pub fn plan(&self) -> Option<&ResourcePlan> {
        self.plan.as_ref()
    }
}
