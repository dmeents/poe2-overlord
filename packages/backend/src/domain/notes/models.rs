use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteData {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
    pub character_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoteParams {
    pub title: String,
    pub content: String,
    pub character_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNoteParams {
    pub title: String,
    pub content: String,
    pub character_id: Option<String>,
}
