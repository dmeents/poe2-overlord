use crate::errors::AppResult;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteSynchronous};
use std::path::Path;

pub struct DatabasePool {
    pool: SqlitePool,
}

impl DatabasePool {
    pub async fn new(db_path: &Path) -> AppResult<Self> {
        let options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .busy_timeout(std::time::Duration::from_secs(5));

        let pool = SqlitePoolOptions::new()
            .max_connections(5) // Desktop app, not a web server
            .connect_with(options)
            .await
            .map_err(|e| crate::errors::AppError::internal_error(
                "database_init",
                &format!("Failed to connect to SQLite: {}", e),
            ))?;

        // Run migrations
        sqlx::migrate!("src/infrastructure/database/migrations")
            .run(&pool)
            .await
            .map_err(|e| crate::errors::AppError::internal_error(
                "database_migrate",
                &format!("Failed to run migrations: {}", e),
            ))?;

        // Enable foreign keys (SQLite has them off by default)
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .map_err(|e| crate::errors::AppError::internal_error(
                "database_pragma",
                &format!("Failed to enable foreign keys: {}", e),
            ))?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
