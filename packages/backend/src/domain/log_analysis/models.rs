use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a zone change event detected in the game log
/// This occurs when the player moves between different zones in Path of Exile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneChangeEvent {
    /// The name of the zone the player entered
    pub zone_name: String,
    /// ISO 8601 timestamp when the zone change occurred
    pub timestamp: String,
}

/// Represents an act change event detected in the game log
/// This occurs when the player progresses to a new act in the campaign
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActChangeEvent {
    /// The name of the act the player entered
    pub act_name: String,
    /// ISO 8601 timestamp when the act change occurred
    pub timestamp: String,
}

/// Represents a hideout change event detected in the game log
/// This occurs when the player enters or leaves their hideout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HideoutChangeEvent {
    /// The name of the hideout the player entered
    pub hideout_name: String,
    /// ISO 8601 timestamp when the hideout change occurred
    pub timestamp: String,
}

/// Represents a server connection event detected in the game log
/// This occurs when the game connects to or disconnects from the game server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConnectionEvent {
    /// The IP address of the server
    pub ip_address: String,
    /// The port number of the server
    pub port: u16,
    /// ISO 8601 timestamp when the connection event occurred
    pub timestamp: String,
}

/// Represents a character level up event detected in the game log
/// This occurs when the player's character gains a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterLevelUpEvent {
    /// The name of the character that leveled up
    pub character_name: String,
    /// The class of the character
    pub character_class: String,
    /// The new level the character reached
    pub new_level: u32,
    /// ISO 8601 timestamp when the level up occurred
    pub timestamp: String,
}

/// Represents a character death event detected in the game log
/// This occurs when the player's character dies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDeathEvent {
    /// The name of the character that died
    pub character_name: String,
    /// ISO 8601 timestamp when the death occurred
    pub timestamp: String,
}

/// Represents different types of scene changes that can occur in the game
/// Uses tagged serialization to distinguish between event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SceneChangeEvent {
    /// Player moved to a different zone
    Zone(ZoneChangeEvent),
    /// Player progressed to a new act
    Act(ActChangeEvent),
    /// Player entered or left a hideout
    Hideout(HideoutChangeEvent),
}

/// Represents all possible events that can be detected from the game log
/// Uses tagged serialization to distinguish between event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum LogEvent {
    /// A scene change event (zone, act, or hideout)
    SceneChange(SceneChangeEvent),
    /// A server connection or disconnection event
    ServerConnection(ServerConnectionEvent),
    /// A character level up event
    CharacterLevelUp(CharacterLevelUpEvent),
    /// A character death event
    CharacterDeath(CharacterDeathEvent),
}

impl SceneChangeEvent {
    /// Returns the name associated with this scene change event
    pub fn get_name(&self) -> &str {
        match self {
            SceneChangeEvent::Zone(event) => &event.zone_name,
            SceneChangeEvent::Act(event) => &event.act_name,
            SceneChangeEvent::Hideout(event) => &event.hideout_name,
        }
    }

    /// Returns the timestamp when this scene change occurred
    pub fn get_timestamp(&self) -> &str {
        match self {
            SceneChangeEvent::Zone(event) => &event.timestamp,
            SceneChangeEvent::Act(event) => &event.timestamp,
            SceneChangeEvent::Hideout(event) => &event.timestamp,
        }
    }

    /// Returns true if this is a zone change event
    pub fn is_zone(&self) -> bool {
        matches!(self, SceneChangeEvent::Zone(_))
    }

    /// Returns true if this is an act change event
    pub fn is_act(&self) -> bool {
        matches!(self, SceneChangeEvent::Act(_))
    }

    /// Returns true if this is a hideout change event
    pub fn is_hideout(&self) -> bool {
        matches!(self, SceneChangeEvent::Hideout(_))
    }
}


/// Information about a log file being monitored
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFileInfo {
    /// The full path to the log file
    pub path: PathBuf,
    /// The current size of the file in bytes
    pub size: u64,
    /// When the file was last modified
    pub last_modified: chrono::DateTime<chrono::Utc>,
    /// Whether the file currently exists
    pub exists: bool,
}

/// Configuration settings for log analysis and monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAnalysisConfig {
    /// Path to the game log file to monitor
    pub log_file_path: String,
    /// How often to check for new log entries (in milliseconds)
    pub monitoring_interval_ms: u64,
    /// Maximum file size before rotation (in megabytes)
    pub max_file_size_mb: u64,
    /// Size of the buffer for reading log lines
    pub buffer_size: usize,
}

impl Default for LogAnalysisConfig {
    fn default() -> Self {
        Self {
            log_file_path: String::new(),
            monitoring_interval_ms: 100,
            max_file_size_mb: 100,
            buffer_size: 1000,
        }
    }
}


/// Represents the analysis result of a single log line
#[derive(Debug, Clone)]
pub struct LogLineAnalysis {
    /// The line number in the log file
    pub line_number: usize,
    /// The raw content of the log line
    pub content: String,
    /// The parsed event if one was detected, None otherwise
    pub parsed_event: Option<LogEvent>,
    /// Time taken to process this line (in milliseconds)
    pub processing_time_ms: u64,
    /// When this line was analyzed
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Errors that can occur during log analysis operations
#[derive(Debug, thiserror::Error)]
pub enum LogAnalysisError {
    /// The specified log file was not found
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    /// An error occurred while accessing the log file
    #[error("File access error: {message}")]
    FileAccessError { message: String },
    
    /// An error occurred while parsing log content
    #[error("Parsing error: {message}")]
    ParsingError { message: String },
    
    /// A configuration-related error
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    /// An error occurred during log monitoring
    #[error("Monitoring error: {message}")]
    MonitoringError { message: String },
}
