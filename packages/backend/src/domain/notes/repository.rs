use async_trait::async_trait;
use chrono::DateTime;
use sqlx::SqlitePool;

use crate::errors::{AppError, AppResult};

use super::models::NoteData;
use super::traits::NotesRepository;

pub struct NotesRepositoryImpl {
    pool: SqlitePool,
}

impl NotesRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn parse_note_row(
    id: String,
    title: String,
    content: String,
    is_pinned: i64,
    character_id: Option<String>,
    created_at_str: String,
    updated_at_str: String,
) -> AppResult<NoteData> {
    let created_at = DateTime::parse_from_rfc3339(&created_at_str)
        .map_err(|e| AppError::internal_error("parse_created_at", &e.to_string()))?
        .with_timezone(&chrono::Utc);

    let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
        .map_err(|e| AppError::internal_error("parse_updated_at", &e.to_string()))?
        .with_timezone(&chrono::Utc);

    Ok(NoteData {
        id,
        title,
        content,
        is_pinned: is_pinned != 0,
        character_id,
        created_at,
        updated_at,
    })
}

#[async_trait]
impl NotesRepository for NotesRepositoryImpl {
    async fn create_note(&self, note: &NoteData) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO notes (id, title, content, is_pinned, character_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&note.id)
        .bind(&note.title)
        .bind(&note.content)
        .bind(note.is_pinned as i64)
        .bind(&note.character_id)
        .bind(note.created_at.to_rfc3339())
        .bind(note.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_note(&self, id: &str) -> AppResult<NoteData> {
        let row: Option<(String, String, String, i64, Option<String>, String, String)> =
            sqlx::query_as(
                "SELECT id, title, content, is_pinned, character_id, created_at, updated_at
                 FROM notes WHERE id = ?",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let (id, title, content, is_pinned, character_id, created_at_str, updated_at_str) = row
            .ok_or_else(|| {
                AppError::validation_error("get_note", &format!("Note '{}' not found", id))
            })?;

        parse_note_row(
            id,
            title,
            content,
            is_pinned,
            character_id,
            created_at_str,
            updated_at_str,
        )
    }

    async fn get_all_notes(&self) -> AppResult<Vec<NoteData>> {
        let rows: Vec<(String, String, String, i64, Option<String>, String, String)> =
            sqlx::query_as(
                "SELECT id, title, content, is_pinned, character_id, created_at, updated_at
                 FROM notes ORDER BY updated_at DESC",
            )
            .fetch_all(&self.pool)
            .await?;

        rows.into_iter()
            .map(
                |(id, title, content, is_pinned, character_id, created_at, updated_at)| {
                    parse_note_row(
                        id,
                        title,
                        content,
                        is_pinned,
                        character_id,
                        created_at,
                        updated_at,
                    )
                },
            )
            .collect()
    }

    async fn get_pinned_notes(&self) -> AppResult<Vec<NoteData>> {
        let rows: Vec<(String, String, String, i64, Option<String>, String, String)> =
            sqlx::query_as(
                "SELECT id, title, content, is_pinned, character_id, created_at, updated_at
                 FROM notes WHERE is_pinned = 1 ORDER BY updated_at DESC",
            )
            .fetch_all(&self.pool)
            .await?;

        rows.into_iter()
            .map(
                |(id, title, content, is_pinned, character_id, created_at, updated_at)| {
                    parse_note_row(
                        id,
                        title,
                        content,
                        is_pinned,
                        character_id,
                        created_at,
                        updated_at,
                    )
                },
            )
            .collect()
    }

    async fn update_note(&self, note: &NoteData) -> AppResult<()> {
        let rows_affected = sqlx::query(
            "UPDATE notes SET title = ?, content = ?, is_pinned = ?, character_id = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&note.title)
        .bind(&note.content)
        .bind(note.is_pinned as i64)
        .bind(&note.character_id)
        .bind(note.updated_at.to_rfc3339())
        .bind(&note.id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::validation_error(
                "update_note",
                &format!("Note '{}' not found", note.id),
            ));
        }

        Ok(())
    }

    async fn delete_note(&self, id: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn set_pinned(&self, id: &str, is_pinned: bool) -> AppResult<()> {
        let rows_affected =
            sqlx::query("UPDATE notes SET is_pinned = ?, updated_at = ? WHERE id = ?")
                .bind(is_pinned as i64)
                .bind(chrono::Utc::now().to_rfc3339())
                .bind(id)
                .execute(&self.pool)
                .await?
                .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::validation_error(
                "set_pinned",
                &format!("Note '{}' not found", id),
            ));
        }

        Ok(())
    }
}
