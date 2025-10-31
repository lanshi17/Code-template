#[derive(Debug, Clone)]
pub struct UcieProtocolConfig {
    pub lane_width: u8,
    pub link_speed_gtps: u32,
    pub enable_flow_control: bool,
}

impl Default for UcieProtocolConfig {
    fn default() -> Self {
        Self {
            lane_width: 16,
            link_speed_gtps: 32,
            enable_flow_control: true,
        }
    }
}
