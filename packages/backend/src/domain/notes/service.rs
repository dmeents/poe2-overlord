use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};

use super::models::{CreateNoteParams, NoteData, UpdateNoteParams};
use super::traits::{NotesRepository, NotesService};

pub struct NotesServiceImpl {
    repository: Arc<dyn NotesRepository + Send + Sync>,
}

impl NotesServiceImpl {
    pub fn new(repository: Arc<dyn NotesRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl NotesService for NotesServiceImpl {
    async fn create_note(&self, params: CreateNoteParams) -> AppResult<NoteData> {
        let trimmed = params.title.trim().to_string();
        if trimmed.is_empty() {
            return Err(AppError::validation_error(
                "create_note",
                "Note title cannot be empty",
            ));
        }

        let now = Utc::now();
        let note = NoteData {
            id: Uuid::new_v4().to_string(),
            title: trimmed,
            content: params.content,
            is_pinned: false,
            character_id: params.character_id,
            created_at: now,
            updated_at: now,
        };

        self.repository.create_note(&note).await?;
        Ok(note)
    }

    async fn get_note(&self, id: &str) -> AppResult<NoteData> {
        self.repository.get_note(id).await
    }

    async fn get_all_notes(&self) -> AppResult<Vec<NoteData>> {
        self.repository.get_all_notes().await
    }

    async fn get_pinned_notes(&self) -> AppResult<Vec<NoteData>> {
        self.repository.get_pinned_notes().await
    }

    async fn update_note(&self, id: &str, params: UpdateNoteParams) -> AppResult<NoteData> {
        let trimmed = params.title.trim().to_string();
        if trimmed.is_empty() {
            return Err(AppError::validation_error(
                "update_note",
                "Note title cannot be empty",
            ));
        }

        let mut note = self.repository.get_note(id).await?;
        note.title = trimmed;
        note.content = params.content;
        note.character_id = params.character_id;
        note.updated_at = Utc::now();

        self.repository.update_note(&note).await?;
        Ok(note)
    }

    async fn delete_note(&self, id: &str) -> AppResult<()> {
        self.repository.delete_note(id).await
    }

    async fn toggle_pin(&self, id: &str) -> AppResult<NoteData> {
        let mut note = self.repository.get_note(id).await?;
        let new_pinned = !note.is_pinned;
        self.repository.set_pinned(id, new_pinned).await?;
        note.is_pinned = new_pinned;
        note.updated_at = Utc::now();
        Ok(note)
    }
}
