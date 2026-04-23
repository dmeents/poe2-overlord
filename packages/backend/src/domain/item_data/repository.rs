use async_trait::async_trait;
use chrono::Utc;
use serde_json;
use sqlx::{Row, SqlitePool};

use crate::errors::{AppError, AppResult};

use super::models::{
    AttributeRequirements, BreachstoneInfo, CurrencyData, DefenceValues, EssenceInfo, FlaskData,
    GameDataVersion, GemData, Item, ItemCategory, ItemSearchParams, ItemSearchResult, ModDisplay,
    OmenInfo, ShieldValues, SoulCoreInfo, WeaponValues,
};
use super::traits::ItemDataRepository;

pub struct ItemDataRepositoryImpl {
    pool: SqlitePool,
}

impl ItemDataRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// ---------------------------------------------------------------------------
// Trait implementation
// ---------------------------------------------------------------------------

#[async_trait]
impl ItemDataRepository for ItemDataRepositoryImpl {
    async fn get_version(&self) -> AppResult<Option<GameDataVersion>> {
        let row = sqlx::query(
            "SELECT patch_version, extracted_at, imported_at FROM game_data_version WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            AppError::internal_error(
                "get_version",
                &format!("Failed to query game_data_version: {e}"),
            )
        })?;

        Ok(row.map(|r| GameDataVersion {
            patch_version: r.get("patch_version"),
            extracted_at: r.get("extracted_at"),
            imported_at: r.get("imported_at"),
        }))
    }

    async fn import_data(
        &self,
        patch_version: &str,
        extracted_at: &str,
        categories: &[ItemCategory],
        items: &[Item],
    ) -> AppResult<()> {
        let imported_at = Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await.map_err(|e| {
            AppError::internal_error("import_data", &format!("Failed to begin transaction: {e}"))
        })?;

        // Clear existing data (CASCADE takes care of item_favorites → items)
        sqlx::query("DELETE FROM items")
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                AppError::internal_error("import_data", &format!("Failed to delete items: {e}"))
            })?;
        sqlx::query("DELETE FROM item_categories")
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                AppError::internal_error(
                    "import_data",
                    &format!("Failed to delete item_categories: {e}"),
                )
            })?;

        // Insert categories
        for cat in categories {
            sqlx::query("INSERT INTO item_categories (id, name) VALUES (?, ?)")
                .bind(&cat.id)
                .bind(&cat.name)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    AppError::internal_error(
                        "import_data",
                        &format!("Failed to insert category {}: {e}", cat.id),
                    )
                })?;
        }

        // Insert items
        // Columns beyond the 011 baseline (movement_speed, reload_time, gem_*_req_percent,
        // soul_core_*, is_corrupted, unmodifiable, flask_name) were added in later migrations.
        // This INSERT is safe because sqlx::migrate!() runs all migrations before
        // ensure_data_imported() is ever called in service_registry.rs.
        for item in items {
            let tags_json = serde_json::to_string(&item.tags).unwrap_or_else(|_| "[]".to_string());
            let implicit_json =
                serde_json::to_string(&item.implicit_mods).unwrap_or_else(|_| "[]".to_string());
            let explicit_json =
                serde_json::to_string(&item.explicit_mods).unwrap_or_else(|_| "[]".to_string());
            let essence_json = item
                .essence
                .as_ref()
                .and_then(|e| serde_json::to_string(e).ok());
            let breachstone_json = item
                .breachstone
                .as_ref()
                .and_then(|b| serde_json::to_string(b).ok());

            sqlx::query(
                "INSERT INTO items (
                    id, name, is_unique, unique_name, base_type,
                    item_class_id, category, rarity_frame, width, height, drop_level,
                    image_url, flavour_text, tags,
                    is_corrupted, unmodifiable,
                    req_str, req_dex, req_int,
                    armour, evasion, energy_shield, ward, movement_speed,
                    damage_min, damage_max, critical, attack_speed, range_max, reload_time,
                    block,
                    gem_type, gem_colour, gem_min_level, gem_tier,
                    gem_str_req_percent, gem_dex_req_percent, gem_int_req_percent,
                    stack_size, currency_description,
                    flask_type, flask_name, flask_life, flask_mana, flask_recovery_time,
                    soul_core_required_level, soul_core_limit_count, soul_core_limit_text,
                    essence,
                    omen_description,
                    map_tier, talisman_tier, breachstone, quest_description,
                    implicit_mods, explicit_mods
                ) VALUES (
                    ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?,
                    ?, ?, ?,
                    ?, ?,
                    ?, ?, ?,
                    ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?,
                    ?,
                    ?, ?, ?, ?,
                    ?, ?, ?,
                    ?, ?,
                    ?, ?, ?, ?, ?,
                    ?, ?, ?,
                    ?,
                    ?,
                    ?, ?, ?, ?,
                    ?, ?
                )",
            )
            .bind(&item.id)
            .bind(&item.name)
            .bind(i32::from(item.is_unique))
            .bind(&item.unique_name)
            .bind(&item.base_type)
            .bind(&item.item_class_id)
            .bind(&item.category)
            .bind(item.rarity_frame)
            .bind(item.width)
            .bind(item.height)
            .bind(item.drop_level)
            .bind(&item.image_url)
            .bind(&item.flavour_text)
            .bind(&tags_json)
            .bind(i32::from(item.is_corrupted))
            .bind(i32::from(item.unmodifiable))
            .bind(item.requirements.str_req)
            .bind(item.requirements.dex_req)
            .bind(item.requirements.int_req)
            // Defences
            .bind(item.defences.as_ref().map(|d| d.armour))
            .bind(item.defences.as_ref().map(|d| d.evasion))
            .bind(item.defences.as_ref().map(|d| d.energy_shield))
            .bind(item.defences.as_ref().map(|d| d.ward))
            .bind(item.defences.as_ref().map(|d| d.movement_speed))
            // Weapon
            .bind(item.weapon.as_ref().map(|w| w.damage_min))
            .bind(item.weapon.as_ref().map(|w| w.damage_max))
            .bind(item.weapon.as_ref().map(|w| w.critical))
            .bind(item.weapon.as_ref().map(|w| w.attack_speed))
            .bind(item.weapon.as_ref().map(|w| w.range_max))
            .bind(item.weapon.as_ref().map(|w| w.reload_time))
            // Shield
            .bind(item.shield.as_ref().map(|s| s.block))
            // Gem
            .bind(item.gem.as_ref().and_then(|g| g.gem_type.as_deref()))
            .bind(item.gem.as_ref().and_then(|g| g.gem_colour.as_deref()))
            .bind(item.gem.as_ref().map(|g| g.gem_min_level))
            .bind(item.gem.as_ref().and_then(|g| g.gem_tier))
            .bind(item.gem.as_ref().map(|g| g.str_req_percent))
            .bind(item.gem.as_ref().map(|g| g.dex_req_percent))
            .bind(item.gem.as_ref().map(|g| g.int_req_percent))
            // Currency
            .bind(item.currency.as_ref().map(|c| c.stack_size))
            .bind(item.currency.as_ref().and_then(|c| c.description.as_deref()))
            // Flask
            .bind(item.flask.as_ref().and_then(|f| f.flask_type.as_deref()))
            .bind(item.flask.as_ref().and_then(|f| f.flask_name.as_deref()))
            .bind(item.flask.as_ref().map(|f| f.flask_life))
            .bind(item.flask.as_ref().map(|f| f.flask_mana))
            .bind(item.flask.as_ref().map(|f| f.flask_recovery_time))
            // Soul core
            .bind(item.soul_core.as_ref().map(|s| s.required_level))
            .bind(item.soul_core.as_ref().and_then(|s| s.limit_count))
            .bind(item.soul_core.as_ref().and_then(|s| s.limit_text.as_deref()))
            // Essence (JSON blob — modifier list is variable-length)
            .bind(&essence_json)
            // Omen description (plain text column)
            .bind(item.omen.as_ref().map(|o| o.description.as_str()))
            // Per-class enrichment
            .bind(item.map_tier)
            .bind(item.talisman_tier)
            .bind(&breachstone_json)
            .bind(&item.quest_description)
            // Mods
            .bind(&implicit_json)
            .bind(&explicit_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                AppError::internal_error(
                    "import_data",
                    &format!("Failed to insert item {}: {e}", item.id),
                )
            })?;
        }

        // Upsert version
        sqlx::query(
            "INSERT INTO game_data_version (id, patch_version, extracted_at, imported_at)
             VALUES (1, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
                patch_version = excluded.patch_version,
                extracted_at  = excluded.extracted_at,
                imported_at   = excluded.imported_at",
        )
        .bind(patch_version)
        .bind(extracted_at)
        .bind(&imported_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            AppError::internal_error("import_data", &format!("Failed to upsert version: {e}"))
        })?;

        tx.commit().await.map_err(|e| {
            AppError::internal_error("import_data", &format!("Failed to commit transaction: {e}"))
        })?;

        Ok(())
    }

    async fn get_item(&self, id: &str) -> AppResult<Option<Item>> {
        let row = sqlx::query("SELECT * FROM items WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                AppError::internal_error("get_item", &format!("Failed to query item {id}: {e}"))
            })?;

        Ok(row.map(|r| row_to_item(&r)))
    }

    async fn search_items(&self, params: &ItemSearchParams) -> AppResult<ItemSearchResult> {
        // Build WHERE clause dynamically
        let search_pattern;

        if let Some(q) = &params.query {
            search_pattern = format!("%{q}%");
        } else {
            search_pattern = String::new();
        }

        // sqlx doesn't support truly dynamic binding; we build a query string.
        // All user inputs are bound as parameters (no interpolation) to prevent injection.
        let mut where_parts = Vec::new();
        if params.query.is_some() {
            where_parts.push("(name LIKE ? COLLATE NOCASE OR base_type LIKE ? COLLATE NOCASE)");
        }
        if params.category.is_some() {
            where_parts.push("category = ?");
        }
        if let Some(is_unique) = params.is_unique {
            where_parts.push(if is_unique {
                "is_unique = 1"
            } else {
                "is_unique = 0"
            });
        }
        if params.min_level.is_some() {
            where_parts.push("drop_level >= ?");
        }
        if params.max_level.is_some() {
            where_parts.push("drop_level <= ?");
        }

        let where_clause = if where_parts.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_parts.join(" AND "))
        };

        // clamp lower bound to 1 so a negative value can't produce LIMIT -1
        // (SQLite interprets LIMIT -1 as "no limit" — full-table scan).
        let limit = params.limit.unwrap_or(50).clamp(1, 500);
        let offset = params.offset.unwrap_or(0).max(0);

        let count_sql = format!("SELECT COUNT(*) FROM items {where_clause}");
        let items_sql = format!(
            "SELECT * FROM items {where_clause} ORDER BY name COLLATE NOCASE LIMIT ? OFFSET ?"
        );

        macro_rules! bind_params {
            ($q:expr) => {{
                let mut q = $q;
                if let Some(_) = &params.query {
                    q = q.bind(&search_pattern).bind(&search_pattern);
                }
                if let Some(cat) = &params.category {
                    q = q.bind(cat);
                }
                if let Some(min) = params.min_level {
                    q = q.bind(min);
                }
                if let Some(max) = params.max_level {
                    q = q.bind(max);
                }
                q
            }};
        }

        let total_count: i64 = {
            let q = bind_params!(sqlx::query_scalar(&count_sql));
            q.fetch_one(&self.pool).await.map_err(|e| {
                AppError::internal_error("search_items", &format!("Failed to count items: {e}"))
            })?
        };

        let rows = {
            let q = bind_params!(sqlx::query(&items_sql));
            q.bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| {
                    AppError::internal_error(
                        "search_items",
                        &format!("Failed to search items: {e}"),
                    )
                })?
        };

        let items = rows.iter().map(row_to_item).collect();

        Ok(ItemSearchResult { items, total_count })
    }

    async fn get_categories(&self) -> AppResult<Vec<ItemCategory>> {
        let rows = sqlx::query("SELECT id, name FROM item_categories ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                AppError::internal_error(
                    "get_categories",
                    &format!("Failed to query categories: {e}"),
                )
            })?;

        Ok(rows
            .iter()
            .map(|r| ItemCategory {
                id: r.get("id"),
                name: r.get("name"),
            })
            .collect())
    }

    async fn toggle_favorite(&self, item_id: &str) -> AppResult<bool> {
        let now = Utc::now().to_rfc3339();
        // INSERT OR IGNORE is atomic: if the row already exists the UNIQUE
        // constraint is silently skipped and rows_affected == 0. This avoids
        // the TOCTOU race of SELECT-then-INSERT that could produce a UNIQUE
        // constraint violation on rapid double-clicks.
        let inserted =
            sqlx::query("INSERT OR IGNORE INTO item_favorites (item_id, created_at) VALUES (?, ?)")
                .bind(item_id)
                .bind(&now)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    AppError::internal_error(
                        "toggle_favorite",
                        &format!("Failed to insert favourite: {e}"),
                    )
                })?
                .rows_affected()
                > 0;

        if inserted {
            return Ok(true);
        }

        // Row already existed — remove it.
        sqlx::query("DELETE FROM item_favorites WHERE item_id = ?")
            .bind(item_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                AppError::internal_error(
                    "toggle_favorite",
                    &format!("Failed to remove favourite: {e}"),
                )
            })?;

        Ok(false)
    }

    async fn get_favorites(&self) -> AppResult<Vec<Item>> {
        let rows = sqlx::query(
            "SELECT i.* FROM items i
             JOIN item_favorites f ON i.id = f.item_id
             ORDER BY f.created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            AppError::internal_error("get_favorites", &format!("Failed to load favourites: {e}"))
        })?;

        Ok(rows.iter().map(row_to_item).collect())
    }

    async fn get_item_by_name(&self, name: &str) -> AppResult<Option<Item>> {
        let row = sqlx::query("SELECT * FROM items WHERE name = ? AND is_unique = 0 LIMIT 1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                AppError::internal_error(
                    "get_item_by_name",
                    &format!("Failed to query item by name '{name}': {e}"),
                )
            })?;

        Ok(row.map(|r| row_to_item(&r)))
    }
}

// ---------------------------------------------------------------------------
// Row → Item conversion
// ---------------------------------------------------------------------------

fn row_to_item(r: &sqlx::sqlite::SqliteRow) -> Item {
    let tags_json: String = r.try_get("tags").unwrap_or_default();
    let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();

    let implicit_json: String = r.try_get("implicit_mods").unwrap_or_default();
    let implicit_mods: Vec<ModDisplay> = serde_json::from_str(&implicit_json).unwrap_or_default();

    let explicit_json: String = r.try_get("explicit_mods").unwrap_or_default();
    let explicit_mods: Vec<ModDisplay> = serde_json::from_str(&explicit_json).unwrap_or_default();

    // Use try_get::<Option<i64>, _>(...).ok().flatten() for NULL-indicator columns.
    // try_get::<i64>() on a NULL SQLite column calls sqlite3_column_int64() which
    // returns 0 — indistinguishable from a real zero value. Option<i64> decode
    // instead returns Ok(None) for NULL, giving us a reliable NULL signal.

    let defences = {
        let armour: Option<i64> = r.try_get::<Option<i64>, _>("armour").ok().flatten();
        armour.map(|a| DefenceValues {
            armour: a,
            evasion: r.try_get("evasion").unwrap_or(0),
            energy_shield: r.try_get("energy_shield").unwrap_or(0),
            ward: r.try_get("ward").unwrap_or(0),
            movement_speed: r.try_get("movement_speed").unwrap_or(0),
        })
    };

    let weapon = {
        let damage_min: Option<i64> = r.try_get::<Option<i64>, _>("damage_min").ok().flatten();
        damage_min.map(|dm| WeaponValues {
            damage_min: dm,
            damage_max: r.try_get("damage_max").unwrap_or(0),
            critical: r.try_get("critical").unwrap_or(0),
            attack_speed: r.try_get("attack_speed").unwrap_or(0),
            range_max: r.try_get("range_max").unwrap_or(0),
            reload_time: r.try_get("reload_time").unwrap_or(0),
        })
    };

    let shield = {
        let block: Option<i64> = r.try_get::<Option<i64>, _>("block").ok().flatten();
        block.map(|b| ShieldValues { block: b })
    };

    let gem = {
        // Use gem_min_level as the sentinel: gem_type is NULL for uncut/typeless
        // gems (e.g. Uncut Skill Gems), but gem_min_level is always set when a
        // SkillGems row exists. Using gem_type as the sentinel would silently
        // drop all uncut gems.
        let gem_min_level: Option<i64> =
            r.try_get::<Option<i64>, _>("gem_min_level").ok().flatten();
        gem_min_level.map(|lvl| GemData {
            gem_type: r.try_get("gem_type").ok().flatten(),
            gem_colour: r.try_get("gem_colour").ok().flatten(),
            gem_min_level: lvl,
            gem_tier: r.try_get("gem_tier").ok().flatten(),
            str_req_percent: r.try_get("gem_str_req_percent").unwrap_or(0),
            dex_req_percent: r.try_get("gem_dex_req_percent").unwrap_or(0),
            int_req_percent: r.try_get("gem_int_req_percent").unwrap_or(0),
        })
    };

    let currency = {
        let stack_size: Option<i64> = r.try_get::<Option<i64>, _>("stack_size").ok().flatten();
        stack_size.map(|s| CurrencyData {
            stack_size: s,
            description: r.try_get("currency_description").ok().flatten(),
        })
    };

    let flask = {
        let flask_type: Option<String> = r.try_get("flask_type").ok().flatten();
        if flask_type.is_some() {
            Some(FlaskData {
                flask_type,
                flask_name: r.try_get("flask_name").ok().flatten(),
                flask_life: r.try_get("flask_life").unwrap_or(0),
                flask_mana: r.try_get("flask_mana").unwrap_or(0),
                flask_recovery_time: r.try_get("flask_recovery_time").unwrap_or(0),
            })
        } else {
            None
        }
    };

    let soul_core = {
        let required_level: Option<i64> = r
            .try_get::<Option<i64>, _>("soul_core_required_level")
            .ok()
            .flatten();
        required_level.map(|lvl| SoulCoreInfo {
            required_level: lvl,
            limit_count: r.try_get("soul_core_limit_count").ok().flatten(),
            limit_text: r.try_get("soul_core_limit_text").ok().flatten(),
        })
    };

    let essence: Option<EssenceInfo> = r
        .try_get::<Option<String>, _>("essence")
        .ok()
        .flatten()
        .and_then(|s| serde_json::from_str(&s).ok());

    let omen: Option<OmenInfo> = r
        .try_get::<Option<String>, _>("omen_description")
        .ok()
        .flatten()
        .map(|d| OmenInfo { description: d });

    let map_tier: Option<i64> = r.try_get::<Option<i64>, _>("map_tier").ok().flatten();
    let talisman_tier: Option<i64> =
        r.try_get::<Option<i64>, _>("talisman_tier").ok().flatten();
    let breachstone: Option<BreachstoneInfo> = r
        .try_get::<Option<String>, _>("breachstone")
        .ok()
        .flatten()
        .and_then(|s| serde_json::from_str(&s).ok());
    let quest_description: Option<String> =
        r.try_get::<Option<String>, _>("quest_description").ok().flatten();

    Item {
        id: r.try_get("id").unwrap_or_default(),
        name: r.try_get("name").unwrap_or_default(),
        is_unique: r.try_get::<i32, _>("is_unique").unwrap_or(0) != 0,
        unique_name: r.try_get("unique_name").ok().flatten(),
        base_type: r.try_get("base_type").ok().flatten(),
        item_class_id: r.try_get("item_class_id").unwrap_or_default(),
        category: r.try_get("category").unwrap_or_default(),
        rarity_frame: r.try_get("rarity_frame").unwrap_or(0),
        width: r.try_get("width").unwrap_or(1),
        height: r.try_get("height").unwrap_or(1),
        drop_level: r.try_get("drop_level").unwrap_or(0),
        image_url: r.try_get("image_url").ok().flatten(),
        flavour_text: r.try_get("flavour_text").ok().flatten(),
        tags,
        is_corrupted: r.try_get::<i32, _>("is_corrupted").unwrap_or(0) != 0,
        unmodifiable: r.try_get::<i32, _>("unmodifiable").unwrap_or(0) != 0,
        requirements: AttributeRequirements {
            str_req: r.try_get("req_str").unwrap_or(0),
            dex_req: r.try_get("req_dex").unwrap_or(0),
            int_req: r.try_get("req_int").unwrap_or(0),
        },
        defences,
        weapon,
        shield,
        gem,
        currency,
        flask,
        soul_core,
        essence,
        omen,
        map_tier,
        talisman_tier,
        breachstone,
        quest_description,
        implicit_mods,
        explicit_mods,
    }
}
