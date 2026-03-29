use std::sync::Arc;
use tauri::State;

use crate::domain::notes::models::{CreateNoteParams, NoteData, UpdateNoteParams};
use crate::domain::notes::traits::NotesService;
use crate::{to_command_result, CommandResult};

#[tauri::command]
pub async fn create_note(
    title: String,
    content: String,
    character_id: Option<String>,
    notes_service: State<'_, Arc<dyn NotesService + Send + Sync>>,
) -> CommandResult<NoteData> {
    to_command_result(notes_service.create_note(CreateNoteParams { title, content, character_id }).await)
}

#[tauri::command]
pub async fn get_note(
    note_id: String,
    notes_service: State<'_, Arc<dyn NotesService + Send + Sync>>,
) -> CommandResult<NoteData> {
    to_command_result(notes_service.get_note(&note_id).await)
}

#[tauri::command]
pub async fn get_all_notes(
    notes_service: State<'_, Arc<dyn NotesService + Send + Sync>>,
) -> CommandResult<Vec<NoteData>> {
    to_command_result(notes_service.get_all_notes().await)
}

#[tauri::command]
pub async fn get_pinned_notes(
    notes_service: State<'_, Arc<dyn NotesService + Send + Sync>>,
) -> CommandResult<Vec<NoteData>> {
    to_command_result(notes_service.get_pinned_notes().await)
}

#[tauri::command]
pub async fn update_note(
    note_id: String,
    title: String,
    content: String,
    character_id: Option<String>,
    notes_service: State<'_, Arc<dyn NotesService + Send + Sync>>,
) -> CommandResult<NoteData> {
    to_command_result(
        notes_service
            .update_note(&note_id, UpdateNoteParams { title, content, character_id })
            .await,
    )
}

#[tauri::command]
pub async fn delete_note(
    note_id: String,
    notes_service: State<'_, Arc<dyn NotesService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(notes_service.delete_note(&note_id).await)
}

#[tauri::command]
pub async fn toggle_note_pin(
    note_id: String,
    notes_service: State<'_, Arc<dyn NotesService + Send + Sync>>,
) -> CommandResult<NoteData> {
    to_command_result(notes_service.toggle_pin(&note_id).await)
}
