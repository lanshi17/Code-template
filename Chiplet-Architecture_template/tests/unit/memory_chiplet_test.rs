use std::collections::HashMap;

use chiplet_architecture_template::chiplets::{memory::MemoryChiplet, ChipletCategory, ChipletDescriptor};
use serde_yaml::Value;

#[test]
fn reports_capacity_and_channels() {
    let mut resources = HashMap::new();
    resources.insert("capacity_gb".into(), Value::from(16));
    resources.insert("channels".into(), Value::from(8));

    let descriptor = ChipletDescriptor {
        name: "memory".into(),
        category: ChipletCategory::Hbm2e,
        resources,
        ..Default::default()
    };

    let chiplet = MemoryChiplet::from_descriptor(descriptor);
    assert_eq!(chiplet.capacity_gb(), Some(16));
    assert_eq!(chiplet.channels(), Some(8));
}
