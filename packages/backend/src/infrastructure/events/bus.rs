use crate::errors::AppResult;
use crate::infrastructure::events::channels::ChannelManager;
use crate::infrastructure::events::types::{AppEvent, EventType};
use log::debug;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct EventBus {
    channel_manager: Arc<ChannelManager>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            channel_manager: Arc::new(ChannelManager::new()),
        }
    }

    pub async fn publish(&self, event: AppEvent) -> AppResult<()> {
        let event_type = event.event_type();
        let channel = self
            .channel_manager
            .get_or_create_channel(event_type)
            .await?;

        match channel.sender().send(event.clone()) {
            Ok(receiver_count) => {
                if receiver_count > 0 {
                    debug!(
                        "Published event of type {:?} to {} subscribers",
                        event_type, receiver_count
                    );
                } else {
                    debug!("Published event of type {:?} (no subscribers)", event_type);
                }
                Ok(())
            }
            Err(broadcast::error::SendError(_event)) => {
                // No receivers is normal (e.g., during startup) - not an error
                debug!("Published event of type {:?} (no receivers)", event_type);
                Ok(())
            }
        }
    }

    pub async fn get_receiver(
        &self,
        event_type: EventType,
    ) -> AppResult<broadcast::Receiver<AppEvent>> {
        let channel = self
            .channel_manager
            .get_or_create_channel(event_type)
            .await?;
        Ok(channel.sender().subscribe())
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
