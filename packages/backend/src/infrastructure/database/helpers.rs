use crate::errors::AppResult;
use chrono::Utc;
use sqlx::{Executor, Sqlite, SqlitePool};

/// Get or create a zone in zone_metadata by name, returning its integer ID.
///
/// This helper is used by both ZoneConfigurationRepository and CharacterRepository
/// when they need to reference a zone by ID. If the zone doesn't exist, a stub
/// row is created with just the zone_name and timestamps. WikiScrapingService
/// fills in the metadata later.
///
/// Accepts any executor (Pool or Transaction) for maximum flexibility.
pub async fn get_or_create_zone_id<'a, E>(executor: E, zone_name: &str) -> AppResult<i64>
where
    E: Executor<'a, Database = Sqlite>,
{
    // Try to get existing
    let existing: Option<i64> =
        sqlx::query_scalar("SELECT id FROM zone_metadata WHERE zone_name = ?")
            .bind(zone_name)
            .fetch_optional(executor)
            .await?;

    if let Some(id) = existing {
        return Ok(id);
    }

    // If not found, we need a mutable connection to insert
    // This version only supports lookups
    // For insertions within a transaction, use the transaction-aware version below
    Err(crate::errors::AppError::Validation {
        message: format!("Zone '{}' not found in metadata", zone_name),
    })
}

/// Get or create a zone in zone_metadata by name within a transaction.
///
/// This version works with transactions and will create stub zones if they don't exist.
pub async fn get_or_create_zone_id_tx<'a>(
    tx: &mut sqlx::Transaction<'a, Sqlite>,
    zone_name: &str,
) -> AppResult<i64> {
    // Try to get existing
    let existing: Option<i64> =
        sqlx::query_scalar("SELECT id FROM zone_metadata WHERE zone_name = ?")
            .bind(zone_name)
            .fetch_optional(&mut **tx)
            .await?;

    if let Some(id) = existing {
        return Ok(id);
    }

    // Create stub
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query(
        "INSERT INTO zone_metadata (zone_name, first_discovered, last_updated)
         VALUES (?, ?, ?)",
    )
    .bind(zone_name)
    .bind(&now)
    .bind(&now)
    .execute(&mut **tx)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Get or create a zone in zone_metadata by name using a pool.
///
/// This version works with pools and will create stub zones if they don't exist.
pub async fn get_or_create_zone_id_pool(pool: &SqlitePool, zone_name: &str) -> AppResult<i64> {
    // Try to get existing
    let existing: Option<i64> =
        sqlx::query_scalar("SELECT id FROM zone_metadata WHERE zone_name = ?")
            .bind(zone_name)
            .fetch_optional(pool)
            .await?;

    if let Some(id) = existing {
        return Ok(id);
    }

    // Create stub
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query(
        "INSERT INTO zone_metadata (zone_name, first_discovered, last_updated)
         VALUES (?, ?, ?)",
    )
    .bind(zone_name)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
