use std::collections::HashMap;

use chiplet_architecture_template::chiplets::{core::CoreChiplet, ChipletCategory, ChipletDescriptor};
use serde_yaml::Value;

#[test]
fn exposes_threads_per_core_from_descriptor() {
    let mut resources = HashMap::new();
    resources.insert("threads_per_core".into(), Value::from(4));

    let descriptor = ChipletDescriptor {
        name: "core".into(),
        category: ChipletCategory::GeneralPurpose,
        resources,
        ..Default::default()
    };

    let chiplet = CoreChiplet::from_descriptor(descriptor);
    assert_eq!(chiplet.threads_per_core(), Some(4));
}
