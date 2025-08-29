use crate::models::events::{ActChangeEvent, SceneChangeEvent, ZoneChangeEvent};

/// Trait for parsing log lines into events
pub trait LogParser {
    type Event;
    
    /// Parse a log line and return an event if valid
    fn parse_line(&self, line: &str) -> Option<Self::Event>;
}

/// Scene change parser for detecting "[SCENE] Set Source [Zone/Act Name]" patterns
#[derive(Clone)]
pub struct SceneChangeParser;

impl LogParser for SceneChangeParser {
    type Event = SceneChangeEvent;

    /// Parse a log line and return a scene change event if valid
    fn parse_line(&self, line: &str) -> Option<SceneChangeEvent> {
        if line.contains("[SCENE] Set Source [") && line.contains("]") {
            // Extract content from "[SCENE] Set Source [Content]"
            let prefix = "[SCENE] Set Source [";
            if let Some(start) = line.find(prefix) {
                let content_start = start + prefix.len();
                if let Some(end) = line[content_start..].find("]") {
                    let content = line[content_start..content_start + end].trim();

                    // Skip null or empty content
                    if content.is_empty()
                        || content == "(null)"
                        || content == "undefined"
                        || content.to_lowercase() == "null"
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
}

impl SceneChangeParser {
    /// Determine if the content represents an Act
    fn is_act_content(&self, content: &str) -> bool {
        let lower_content = content.to_lowercase();
        lower_content.starts_with("act ")
            || lower_content == "atlas"
            || lower_content == "prologue"
            || lower_content == "epilogue"
            || lower_content.contains("act")
    }
}
