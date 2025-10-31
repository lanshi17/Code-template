use log::info;

use super::ucie_protocol::UcieProtocolConfig;

#[derive(Debug, Clone)]
pub struct UcieDriver {
    protocol: UcieProtocolConfig,
    trained: bool,
}

impl UcieDriver {
    pub fn new(protocol: UcieProtocolConfig) -> Self {
        Self {
            protocol,
            trained: false,
        }
    }

    pub fn train_links(&mut self) {
        info!(
            "Training UCIe links with lane width {} and speed {} GT/s",
            self.protocol.lane_width, self.protocol.link_speed_gtps
        );
        self.trained = true;
    }

    pub fn is_trained(&self) -> bool {
        self.trained
    }
}
