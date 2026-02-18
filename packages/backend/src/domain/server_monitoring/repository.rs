use crate::domain::server_monitoring::models::ServerStatus;
use crate::domain::server_monitoring::traits::ServerStatusRepository as ServerStatusRepositoryTrait;
use crate::errors::AppResult;
use async_trait::async_trait;
use log::debug;
use sqlx::SqlitePool;

/// Server status repository implementation using SQLite.
///
/// Stores server status in a single-row `server_status` table.
/// Much simpler than the JSON implementation - no file path management needed.
pub struct ServerStatusRepositoryImpl {
    pool: SqlitePool,
}

impl ServerStatusRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServerStatusRepositoryTrait for ServerStatusRepositoryImpl {
    async fn save(&self, status: &ServerStatus) -> AppResult<()> {
        debug!("Saving server status to SQLite");

        // INSERT OR REPLACE into single-row table
        sqlx::query(
            "INSERT OR REPLACE INTO server_status
             (id, ip_address, port, is_online, latency_ms, timestamp)
             VALUES (1, ?, ?, ?, ?, ?)",
        )
        .bind(&status.ip_address)
        .bind(status.port as i64)
        .bind(if status.is_online { 1 } else { 0 })
        .bind(status.latency_ms.map(|v| v as i64))
        .bind(&status.timestamp)
        .execute(&self.pool)
        .await?;

        debug!("Server status saved to SQLite");
        Ok(())
    }

    async fn load(&self) -> AppResult<Option<ServerStatus>> {
        debug!("Loading server status from SQLite");

        let row: Option<(String, i64, i64, Option<i64>, String)> = sqlx::query_as(
            "SELECT ip_address, port, is_online, latency_ms, timestamp
             FROM server_status
             WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await?;

        let status = row.map(
            |(ip_address, port, is_online, latency_ms, timestamp)| ServerStatus {
                ip_address,
                port: port as u16,
                is_online: is_online != 0,
                latency_ms: latency_ms.map(|v| v as u64),
                timestamp,
            },
        );

        if status.is_some() {
            debug!("Server status loaded from SQLite");
        } else {
            debug!("No server status found in SQLite");
        }

        Ok(status)
    }
}
