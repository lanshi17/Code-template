#[derive(Debug, Clone)]
pub struct MemoryRegisters {
    pub error_status: u32,
    pub temperature: u32,
    pub bandwidth_counter: u32,
}

impl Default for MemoryRegisters {
    fn default() -> Self {
        Self {
            error_status: 0x2000_0000,
            temperature: 0x2000_0004,
            bandwidth_counter: 0x2000_0010,
        }
    }
}
