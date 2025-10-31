use std::fs;

use chiplet_architecture_template::utils::{error::OrchestrationError, logging};
use chiplet_architecture_template::{load_inventory_from_yaml, Platform};

fn main() -> Result<(), OrchestrationError> {
    logging::init();
    let data = fs::read("config/chiplet/chiplet_config.yaml")?;
    let inventory = load_inventory_from_yaml(&data)?;
    let mut platform = Platform::new(inventory);
    platform.bootstrap()?;
    println!(
        "Chiplet platform initialized with {} chiplets.",
        platform.inventory.chiplets.len()
    );
    Ok(())
}
