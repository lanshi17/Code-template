pub mod accelerator;
pub mod core;
pub mod io;
pub mod memory;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::utils::error::OrchestrationError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChipletCategory {
    GeneralPurpose,
    Hbm2e,
    PcieRootComplex,
    MatrixEngine,
    #[serde(other)]
    Unknown,
}

impl Default for ChipletCategory {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChipletInterfaces {
    #[serde(default)]
    pub control: Option<String>,
    #[serde(default)]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FirmwareDescriptor {
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChipletDescriptor {
    pub name: String,
    #[serde(default)]
    pub category: ChipletCategory,
    #[serde(default)]
    pub vendor: Option<String>,
    #[serde(default)]
    pub revision: Option<String>,
    #[serde(default)]
    pub interfaces: ChipletInterfaces,
    #[serde(default)]
    pub resources: HashMap<String, serde_yaml::Value>,
    #[serde(default)]
    pub firmware: FirmwareDescriptor,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChipletInventory {
    #[serde(default)]
    pub chiplets: Vec<ChipletDescriptor>,
}

impl ChipletInventory {
    pub fn from_yaml_bytes(data: &[u8]) -> Result<Self, OrchestrationError> {
        let inventory: Self = serde_yaml::from_slice(data)?;
        Ok(inventory)
    }

    pub fn find(&self, name: &str) -> Result<&ChipletDescriptor, OrchestrationError> {
        self.chiplets
            .iter()
            .find(|chiplet| chiplet.name == name)
            .ok_or_else(|| OrchestrationError::ChipletMissing(name.to_string()))
    }
}
