use async_trait::async_trait;

use crate::errors::AppResult;

use super::models::{CreateNoteParams, NoteData, UpdateNoteParams};

#[async_trait]
pub trait NotesRepository: Send + Sync {
    async fn create_note(&self, note: &NoteData) -> AppResult<()>;
    async fn get_note(&self, id: &str) -> AppResult<NoteData>;
    async fn get_all_notes(&self) -> AppResult<Vec<NoteData>>;
    async fn get_pinned_notes(&self) -> AppResult<Vec<NoteData>>;
    async fn update_note(&self, note: &NoteData) -> AppResult<()>;
    async fn delete_note(&self, id: &str) -> AppResult<()>;
    async fn set_pinned(&self, id: &str, is_pinned: bool) -> AppResult<()>;
}

#[async_trait]
pub trait NotesService: Send + Sync {
    async fn create_note(&self, params: CreateNoteParams) -> AppResult<NoteData>;
    async fn get_note(&self, id: &str) -> AppResult<NoteData>;
    async fn get_all_notes(&self) -> AppResult<Vec<NoteData>>;
    async fn get_pinned_notes(&self) -> AppResult<Vec<NoteData>>;
    async fn update_note(&self, id: &str, params: UpdateNoteParams) -> AppResult<NoteData>;
    async fn delete_note(&self, id: &str) -> AppResult<()>;
    async fn toggle_pin(&self, id: &str) -> AppResult<NoteData>;
}
