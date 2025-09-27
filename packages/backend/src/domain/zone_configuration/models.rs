use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main zone configuration containing all acts and their zones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConfiguration {
    /// List of all acts with their associated zones
    pub acts: Vec<ActDefinition>,
}

impl ZoneConfiguration {
    /// Creates a lookup map for fast zone-to-act resolution
    /// Returns a HashMap where the key is zone_name and the value is (act_name, is_town)
    pub fn create_zone_lookup(&self) -> HashMap<String, (String, bool)> {
        let mut lookup = HashMap::new();
        for act in &self.acts {
            for zone in &act.zones {
                lookup.insert(
                    zone.zone_name.clone(),
                    (act.act_name.clone(), zone.is_town),
                );
            }
        }
        lookup
    }

    /// Gets all zones for a specific act
    pub fn get_act_zones(&self, act_name: &str) -> Option<Vec<ZoneMapping>> {
        self.acts
            .iter()
            .find(|act| act.act_name == act_name)
            .map(|act| act.zones.clone())
    }

    /// Gets the act name for a specific zone
    pub fn get_act_for_zone(&self, zone_name: &str) -> Option<String> {
        for act in &self.acts {
            if act.zones.iter().any(|zone| zone.zone_name == zone_name) {
                return Some(act.act_name.clone());
            }
        }
        None
    }

    /// Checks if a zone is a town
    pub fn is_town_zone(&self, zone_name: &str) -> bool {
        for act in &self.acts {
            if let Some(zone) = act.zones.iter().find(|zone| zone.zone_name == zone_name) {
                return zone.is_town;
            }
        }
        false
    }
}

/// Definition of an act with its associated zones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActDefinition {
    /// Human-readable name of the act (e.g., "Act 1", "Endgame")
    pub act_name: String,
    /// Numeric identifier for the act
    pub act_number: u32,
    /// List of zones belonging to this act
    pub zones: Vec<ZoneMapping>,
}

/// Mapping of a zone to its act and town status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneMapping {
    /// Name of the zone as it appears in the game
    pub zone_name: String,
    /// Whether this zone is a town
    pub is_town: bool,
}

impl ZoneMapping {
    /// Creates a new zone mapping
    pub fn new(zone_name: String, is_town: bool) -> Self {
        Self {
            zone_name,
            is_town,
        }
    }
}

impl Default for ZoneConfiguration {
    fn default() -> Self {
        Self {
            acts: Vec::new(),
        }
    }
}
