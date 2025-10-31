#[derive(Debug, Clone)]
pub struct IoRegisters {
    pub link_status: u32,
    pub error_reporting: u32,
    pub throughput_counter: u32,
}

impl Default for IoRegisters {
    fn default() -> Self {
        Self {
            link_status: 0x3000_0000,
            error_reporting: 0x3000_0008,
            throughput_counter: 0x3000_0010,
        }
    }
}
