use std::collections::VecDeque;

use crate::utils::error::OrchestrationError;

#[derive(Debug, Clone)]
pub struct Message {
    pub source: String,
    pub destination: String,
    pub payload: Vec<u8>,
    pub qos: u8,
}

#[derive(Debug, Default)]
pub struct MessageBus {
    channels: Vec<VecDeque<Message>>,
}

impl MessageBus {
    pub fn initialize_channels(&mut self, channel_count: u32) -> Result<(), OrchestrationError> {
        if channel_count == 0 {
            return Err(OrchestrationError::Configuration(
                "message bus requires at least one channel".to_string(),
            ));
        }
        self.channels = (0..channel_count).map(|_| VecDeque::new()).collect();
        Ok(())
    }

    pub fn publish(&mut self, channel: usize, message: Message) -> Result<(), OrchestrationError> {
        let queue = self.channels.get_mut(channel).ok_or_else(|| {
            OrchestrationError::Configuration(format!("channel {channel} not found"))
        })?;
        queue.push_back(message);
        Ok(())
    }

    pub fn drain_channel(&mut self, channel: usize) -> Option<Vec<Message>> {
        self.channels
            .get_mut(channel)
            .map(|queue| queue.drain(..).collect())
    }
}
