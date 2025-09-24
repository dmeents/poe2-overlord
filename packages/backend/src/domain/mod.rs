pub mod character;
pub mod configuration;
pub mod game_monitoring;
pub mod time_tracking;

// Re-export main types for backward compatibility
pub use character::{
    Ascendency, Character, CharacterClass, CharacterData, CharacterRepository, CharacterService,
    CharacterUpdateParams, League,
};
pub use time_tracking::CharacterSessionTracker;
