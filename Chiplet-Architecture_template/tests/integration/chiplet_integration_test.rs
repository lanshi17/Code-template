use chiplet_architecture_template::{load_inventory_from_yaml, Platform};

#[test]
fn platform_bootstraps_with_sample_inventory() {
    let yaml = r#"
chiplets:
  - name: core
    category: general_purpose
    resources:
      threads_per_core: 4
  - name: memory
    category: hbm2e
    resources:
      capacity_gb: 16
  - name: io
    category: pcie_root_complex
  - name: accelerator
    category: matrix_engine
"#;

    let inventory = load_inventory_from_yaml(yaml.as_bytes()).expect("inventory load");
    let mut platform = Platform::new(inventory);
    platform.bootstrap().expect("bootstrap");
}
