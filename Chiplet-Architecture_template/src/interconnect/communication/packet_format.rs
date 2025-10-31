#[derive(Debug, Clone, Default)]
pub struct PacketMetadata {
    pub virtual_channel: u8,
    pub qos_class: String,
    pub sequence_id: u32,
}

impl PacketMetadata {
    pub fn describe(&self) -> String {
        format!(
            "vc={} qos={} seq={}",
            self.virtual_channel, self.qos_class, self.sequence_id
        )
    }
}
