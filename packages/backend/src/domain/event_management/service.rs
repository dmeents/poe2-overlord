use crate::domain::event_management::models::{
    EventChannel, EventChannelConfig, EventManagementSession, EventManagementStats, EventPayload,
    EventSubscription, EventType,
};
use crate::domain::event_management::traits::{
    EventChannelManager, EventManagementService, EventManagementSessionRepository,
    EventManagementStatsRepository,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

pub struct EventManagementServiceImpl {
    channels: Arc<RwLock<HashMap<EventType, Arc<EventChannel>>>>,
    subscriptions: Arc<RwLock<HashMap<String, EventSubscription>>>,
    session_repository: Arc<dyn EventManagementSessionRepository>,
    stats_repository: Arc<dyn EventManagementStatsRepository>,
    current_session: Arc<RwLock<Option<EventManagementSession>>>,
}

impl EventManagementServiceImpl {
    pub fn new(
        session_repository: Arc<dyn EventManagementSessionRepository>,
        stats_repository: Arc<dyn EventManagementStatsRepository>,
    ) -> Self {
        let channels = Arc::new(RwLock::new(HashMap::new()));
        let subscriptions = Arc::new(RwLock::new(HashMap::new()));

        Self {
            channels,
            subscriptions,
            session_repository,
            stats_repository,
            current_session: Arc::new(RwLock::new(None)),
        }
    }

    async fn get_or_create_channel(&self, event_type: EventType) -> AppResult<Arc<EventChannel>> {
        let mut channels = self.channels.write().await;

        if let Some(channel) = channels.get(&event_type) {
            return Ok(Arc::clone(channel));
        }

        let channel = Arc::new(EventChannel::new(
            event_type.clone(),
            EventChannelConfig::default(),
        ));
        channels.insert(event_type.clone(), Arc::clone(&channel));

        debug!("Created new event channel for type: {:?}", event_type);
        Ok(channel)
    }

    async fn start_management_session(&self) -> AppResult<()> {
        let session = EventManagementSession::new();
        self.session_repository.save_session(&session).await?;

        let mut current_session = self.current_session.write().await;
        *current_session = Some(session);

        info!("Started new event management session");
        Ok(())
    }

    async fn end_management_session(&self) -> AppResult<()> {
        if let Some(mut session) = self.current_session.read().await.clone() {
            session.end_session();
            self.session_repository.update_session(&session).await?;

            let mut stats = self.stats_repository.load_stats().await?;
            stats.total_sessions += 1;
            stats.total_events_published += session.total_events_published;
            stats.total_events_received += session.total_events_received;
            self.stats_repository.update_stats(&stats).await?;

            let mut current_session = self.current_session.write().await;
            *current_session = None;

            info!("Ended event management session");
        }
        Ok(())
    }
}

#[async_trait]
impl EventManagementService for EventManagementServiceImpl {
    async fn publish_event(&self, event: EventPayload) -> AppResult<()> {
        let event_type = event.get_event_type();
        let channel = self.get_or_create_channel(event_type.clone()).await?;

        if let Err(e) = channel.sender.send(event.clone()) {
            error!("Failed to send event: {}", e);
              return Err(crate::errors::AppError::event_emission_error("send_event", &format!(
                "Failed to send event: {}",
                e
            )));
        }

        if let Some(mut session) = self.current_session.read().await.clone() {
            session.increment_published_events();
            self.session_repository.update_session(&session).await?;

            let mut current_session = self.current_session.write().await;
            *current_session = Some(session);
        }

        self.stats_repository.increment_published_events().await?;

        debug!("Published event of type: {:?}", event_type);
        Ok(())
    }

    async fn subscribe_to_events(
        &self,
        event_type: EventType,
        subscriber_name: String,
    ) -> AppResult<EventSubscription> {
        let channel = self.get_or_create_channel(event_type.clone()).await?;

        if channel.is_at_capacity() {
              return Err(crate::errors::AppError::event_emission_error("create_channel", &format!(
                "Channel for event type {:?} is at capacity",
                event_type
            )));
        }

        let subscription = EventSubscription::new(event_type, subscriber_name);
        let subscription_id = subscription.subscription_id.clone();
        let subscription_event_type = subscription.event_type.clone();

        self.subscriptions
            .write()
            .await
            .insert(subscription_id.clone(), subscription.clone());

        if let Some(mut session) = self.current_session.read().await.clone() {
            session.add_subscription(subscription_event_type.clone());
            self.session_repository.update_session(&session).await?;

            let mut current_session = self.current_session.write().await;
            *current_session = Some(session);
        }

        debug!(
            "Created subscription for event type: {:?}",
            subscription_event_type
        );
        Ok(subscription)
    }

    async fn unsubscribe(&self, subscription_id: &str) -> AppResult<()> {
        let mut subscriptions = self.subscriptions.write().await;

        if let Some(mut subscription) = subscriptions.remove(subscription_id) {
            let event_type = subscription.event_type.clone();
            subscription.deactivate();

            if let Some(mut session) = self.current_session.read().await.clone() {
                session.remove_subscription(event_type.clone());
                self.session_repository.update_session(&session).await?;

                let mut current_session = self.current_session.write().await;
                *current_session = Some(session);
            }

            debug!("Unsubscribed from event type: {:?}", event_type);
        } else {
            warn!("Subscription not found: {}", subscription_id);
        }

        Ok(())
    }

    fn get_event_receiver(
        &self,
        event_type: EventType,
    ) -> Option<broadcast::Receiver<EventPayload>> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                if let Ok(channel) = self.get_or_create_channel(event_type).await {
                    Some(channel.sender.subscribe())
                } else {
                    None
                }
            })
        })
    }

    async fn get_subscriber_count(&self, event_type: EventType) -> usize {
        if let Ok(channel) = self.get_or_create_channel(event_type).await {
            channel.get_subscriber_count()
        } else {
            0
        }
    }

    async fn get_active_subscriptions(&self) -> AppResult<Vec<EventSubscription>> {
        let subscriptions = self.subscriptions.read().await;
        let active_subscriptions: Vec<EventSubscription> = subscriptions
            .values()
            .filter(|sub| sub.is_active)
            .cloned()
            .collect();

        Ok(active_subscriptions)
    }

    async fn get_stats(&self) -> AppResult<EventManagementStats> {
        self.stats_repository.load_stats().await
    }

    async fn start_session(&self) -> AppResult<()> {
        self.start_management_session().await
    }

    async fn end_session(&self) -> AppResult<()> {
        self.end_management_session().await
    }

    async fn is_session_active(&self) -> bool {
        self.current_session.read().await.is_some()
    }

    async fn get_current_session(&self) -> Option<EventManagementSession> {
        self.current_session.read().await.clone()
    }

    async fn update_channel_config(
        &self,
        event_type: EventType,
        config: EventChannelConfig,
    ) -> AppResult<()> {
        debug!(
            "Channel config update requested for {:?}: {:?}",
            event_type, config
        );
        Ok(())
    }

    async fn get_channel_config(&self, event_type: EventType) -> Option<EventChannelConfig> {
        if let Ok(channel) = self.get_or_create_channel(event_type).await {
            Some(channel.config.clone())
        } else {
            None
        }
    }
}

pub struct SimpleEventChannelManager {
    channels: Arc<RwLock<HashMap<EventType, Arc<EventChannel>>>>,
}

impl SimpleEventChannelManager {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl EventChannelManager for SimpleEventChannelManager {
    async fn create_channel(
        &self,
        event_type: EventType,
        config: EventChannelConfig,
    ) -> AppResult<()> {
        let channel = Arc::new(EventChannel::new(event_type.clone(), config));
        self.channels.write().await.insert(event_type, channel);
        Ok(())
    }

    async fn get_channel(&self, event_type: EventType) -> Option<Arc<EventChannel>> {
        self.channels.read().await.get(&event_type).cloned()
    }

    async fn remove_channel(&self, event_type: EventType) -> AppResult<()> {
        self.channels.write().await.remove(&event_type);
        Ok(())
    }

    async fn get_all_channels(&self) -> AppResult<Vec<EventType>> {
        let channels = self.channels.read().await;
        Ok(channels.keys().cloned().collect())
    }

    async fn update_channel_config(
        &self,
        event_type: EventType,
        config: EventChannelConfig,
    ) -> AppResult<()> {
        debug!(
            "Channel config update requested for {:?}: {:?}",
            event_type, config
        );
        Ok(())
    }

    async fn get_channel_stats(
        &self,
        event_type: EventType,
    ) -> AppResult<Option<crate::domain::event_management::traits::ChannelStats>> {
        if let Some(channel) = self.get_channel(event_type.clone()).await {
            let stats = crate::domain::event_management::traits::ChannelStats {
                event_type,
                subscriber_count: channel.get_subscriber_count(),
                messages_sent: 0,     // Would need to track this
                messages_received: 0, // Would need to track this
                created_at: channel.created_at,
                last_activity: chrono::Utc::now(),
            };
            Ok(Some(stats))
        } else {
            Ok(None)
        }
    }
}
