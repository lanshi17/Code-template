#[derive(Debug, Clone)]
pub struct AcceleratorRegisters {
    pub command_queue: u32,
    pub status: u32,
    pub telemetry: u32,
}

impl Default for AcceleratorRegisters {
    fn default() -> Self {
        Self {
            command_queue: 0x4000_0000,
            status: 0x4000_0004,
            telemetry: 0x4000_0010,
        }
    }
}
