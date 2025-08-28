use crate::services::config::ConfigService;
use log::{debug, error, info, warn};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::broadcast;
use tokio::time;

/// Errors that can occur during log monitoring
#[derive(Error, Debug)]
pub enum LogMonitorError {
    #[error("Failed to open log file: {0}")]
    FileOpen(#[from] io::Error),
    #[error("Failed to watch file: {0}")]
    WatchError(#[from] notify::Error),
    #[error("Log file not found: {0}")]
    FileNotFound(String),
}

/// Zone change event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZoneChangeEvent {
    pub zone_name: String,
    pub timestamp: String,
}

/// Act change event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActChangeEvent {
    pub act_name: String,
    pub timestamp: String,
}

/// Combined scene change event that can represent either a zone or act change
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum SceneChangeEvent {
    Zone(ZoneChangeEvent),
    Act(ActChangeEvent),
}

/// Scene change parser for detecting "[SCENE] Set Source [Zone/Act Name]" patterns
#[derive(Clone)]
pub struct SceneChangeParser;

impl SceneChangeParser {
    /// Parse a log line and return a scene change event if valid
    pub fn parse_line(&self, line: &str) -> Option<SceneChangeEvent> {
        if line.contains("[SCENE] Set Source [") && line.contains("]") {
            // Extract content from "[SCENE] Set Source [Content]"
            let prefix = "[SCENE] Set Source [";
            if let Some(start) = line.find(prefix) {
                let content_start = start + prefix.len();
                if let Some(end) = line[content_start..].find("]") {
                    let content = line[content_start..content_start + end].trim();

                    // Skip null or empty content
                    if content.is_empty() || content == "(null)" || content.to_lowercase() == "null"
                    {
                        return None;
                    }

                    // Determine if this is an Act or a Zone
                    if self.is_act_content(&content) {
                        return Some(SceneChangeEvent::Act(ActChangeEvent {
                            act_name: content.to_string(),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        }));
                    } else {
                        return Some(SceneChangeEvent::Zone(ZoneChangeEvent {
                            zone_name: content.to_string(),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        }));
                    }
                }
            }
        }
        None
    }

    /// Determine if the content represents an Act
    fn is_act_content(&self, content: &str) -> bool {
        let lower_content = content.to_lowercase();
        lower_content.starts_with("act ")
            || lower_content == "prologue"
            || lower_content == "epilogue"
            || lower_content.contains("act")
    }
}

/// Legacy zone change parser for backward compatibility
#[derive(Clone)]
pub struct ZoneChangeParser;

impl ZoneChangeParser {
    pub fn parse_line(&self, line: &str) -> Option<ZoneChangeEvent> {
        let scene_parser = SceneChangeParser;
        if let Some(SceneChangeEvent::Zone(zone_event)) = scene_parser.parse_line(line) {
            Some(zone_event)
        } else {
            None
        }
    }
}

/// Log monitor service that watches POE client log files for zone changes
pub struct LogMonitorService {
    config_service: Arc<ConfigService>,
    parser: SceneChangeParser,
    event_sender: broadcast::Sender<SceneChangeEvent>,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl LogMonitorService {
    /// Create a new log monitor service
    pub fn new(config_service: Arc<ConfigService>) -> Self {
        let (event_sender, _) = broadcast::channel(1000);
        let is_running = Arc::new(tokio::sync::RwLock::new(false));

        Self {
            config_service,
            parser: SceneChangeParser,
            event_sender,
            is_running,
        }
    }

    /// Get the event receiver for subscribing to scene change events
    pub fn subscribe(&self) -> broadcast::Receiver<SceneChangeEvent> {
        self.event_sender.subscribe()
    }

    /// Get the event receiver for subscribing to zone change events (legacy compatibility)
    pub fn subscribe_zones(&self) -> broadcast::Receiver<ZoneChangeEvent> {
        let (zone_sender, zone_receiver) = broadcast::channel(1000);
        let mut scene_receiver = self.event_sender.subscribe();

        // Spawn a task to filter zone events
        tokio::spawn(async move {
            while let Ok(event) = scene_receiver.recv().await {
                if let SceneChangeEvent::Zone(zone_event) = event {
                    let _ = zone_sender.send(zone_event);
                }
            }
        });

        zone_receiver
    }

    /// Get the event receiver for subscribing to act change events
    pub fn subscribe_acts(&self) -> broadcast::Receiver<ActChangeEvent> {
        let (act_sender, act_receiver) = broadcast::channel(1000);
        let mut scene_receiver = self.event_sender.subscribe();

        // Spawn a task to filter act events
        tokio::spawn(async move {
            while let Ok(event) = scene_receiver.recv().await {
                if let SceneChangeEvent::Act(act_event) = event {
                    let _ = act_sender.send(act_event);
                }
            }
        });

        act_receiver
    }

    /// Start monitoring the log file
    pub async fn start_monitoring(&self) -> Result<(), LogMonitorError> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            warn!("Log monitoring is already running");
            return Ok(());
        }

        *is_running = true;
        drop(is_running);

        let config_service = Arc::clone(&self.config_service);
        let event_sender = self.event_sender.clone();
        let parser = self.parser.clone();
        let is_running = Arc::clone(&self.is_running);

        info!("Starting log file monitoring for zone changes");

        tokio::spawn(async move {
            let log_path = config_service.get_poe_client_log_path();

            if let Err(e) =
                Self::monitor_log_file(&log_path, event_sender, parser, &is_running).await
            {
                error!("Log monitoring failed: {}", e);
            }
        });

        Ok(())
    }

    /// Stop monitoring the log file
    pub async fn stop_monitoring(&self) -> Result<(), LogMonitorError> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            warn!("Log monitoring is not running");
            return Ok(());
        }

        *is_running = false;
        info!("Log monitoring stopped");
        Ok(())
    }

    /// Check if monitoring is currently active
    pub async fn is_monitoring(&self) -> bool {
        *self.is_running.read().await
    }

    /// Get current log file size
    pub fn get_log_file_size(&self) -> Result<u64, LogMonitorError> {
        let log_path = self.config_service.get_poe_client_log_path();
        let path = Path::new(&log_path);

        if !path.exists() {
            return Err(LogMonitorError::FileNotFound(log_path));
        }

        let metadata = fs::metadata(path).map_err(|e| LogMonitorError::FileOpen(e))?;
        Ok(metadata.len())
    }

    /// Read the last N lines from the log file
    pub fn read_last_lines(&self, count: usize) -> Result<Vec<String>, LogMonitorError> {
        let log_path = self.config_service.get_poe_client_log_path();
        let path = Path::new(&log_path);

        if !path.exists() {
            return Err(LogMonitorError::FileNotFound(log_path));
        }

        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| LogMonitorError::FileOpen(e))?;

        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

        let start = if lines.len() > count {
            lines.len() - count
        } else {
            0
        };

        Ok(lines[start..].to_vec())
    }

    /// Main monitoring loop
    async fn monitor_log_file(
        log_path: &str,
        event_sender: broadcast::Sender<SceneChangeEvent>,
        parser: SceneChangeParser,
        is_running: &Arc<tokio::sync::RwLock<bool>>,
    ) -> Result<(), LogMonitorError> {
        let path = Path::new(log_path);

        if !path.exists() {
            return Err(LogMonitorError::FileNotFound(log_path.to_string()));
        }

        // Get initial file size and position
        let mut last_position = Self::get_file_size(path)?;

        // Create file system event watcher
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, _>| {
                if let Ok(event) = res {
                    if let EventKind::Modify(_) = event.kind {
                        debug!("Log file modified, processing changes");
                        // Note: We can't directly call async functions from this callback
                        // The actual processing will happen in the main loop
                    }
                }
            },
            Config::default(),
        )?;

        // Watch the file for changes
        watcher.watch(path, RecursiveMode::NonRecursive)?;

        // Keep the watcher alive and process events
        loop {
            tokio::select! {
                _ = time::sleep(Duration::from_millis(100)) => {
                    // Check if still running
                    if !*is_running.read().await {
                        break;
                    }

                    // Check if file has new content
                    if let Ok(current_size) = Self::get_file_size(path) {
                        if current_size > last_position {
                            // File has new content, process it
                            if let Err(e) = Self::process_new_lines(
                                path,
                                &mut last_position,
                                &parser,
                                &event_sender
                            ).await {
                                warn!("Failed to process new lines: {}", e);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get the current size of a file
    fn get_file_size(path: &Path) -> Result<u64, LogMonitorError> {
        let metadata = fs::metadata(path).map_err(|e| LogMonitorError::FileOpen(e))?;
        Ok(metadata.len())
    }

    /// Process new lines from the log file
    async fn process_new_lines(
        path: &Path,
        last_position: &mut u64,
        parser: &SceneChangeParser,
        event_sender: &broadcast::Sender<SceneChangeEvent>,
    ) -> Result<(), LogMonitorError> {
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| LogMonitorError::FileOpen(e))?;

        let mut reader = BufReader::new(file);

        // Seek to last known position
        reader
            .seek(io::SeekFrom::Start(*last_position))
            .map_err(|e| LogMonitorError::FileOpen(e))?;

        for line in reader.lines() {
            let line = line.map_err(|e| LogMonitorError::FileOpen(e))?;

            // Try to parse the line for scene changes
            if let Some(event) = parser.parse_line(&line) {
                // Send event to subscribers
                if let Err(e) = event_sender.send(event) {
                    debug!("Failed to send scene change event: {}", e);
                }
            }
        }

        // Update position to current file size
        *last_position = Self::get_file_size(path)?;

        Ok(())
    }
}
