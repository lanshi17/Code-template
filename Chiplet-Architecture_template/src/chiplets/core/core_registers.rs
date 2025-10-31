#[derive(Debug, Clone)]
pub struct CoreRegisters {
    pub control: u32,
    pub status: u32,
    pub performance_counter: u32,
}

impl Default for CoreRegisters {
    fn default() -> Self {
        Self {
            control: 0x0000_0000,
            status: 0x0000_0004,
            performance_counter: 0x0000_0100,
        }
    }
}

impl CoreRegisters {
    pub fn enable_flag(&self) -> u32 {
        self.control | 0x1
    }
}
