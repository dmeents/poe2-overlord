# Code Patterns

## Theming & Styling

### Design Tokens (Shared Theme Package)

All design tokens are defined in `@poe2-overlord/theme` (`packages/theme/src/css/tokens.css`). This is the **single source of truth** for colors, shadows, and spacing shared between the desktop app and website.

```css
@theme {
  /* Colors */
  --color-ember-500: #f97316;
  --color-stone-900: #1c1917;

  /* Shadows - high opacity for dark backgrounds */
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.7);
  --shadow-right: 4px 0 6px rgba(0, 0, 0, 0.7);

  /* Spacing */
  --spacing-titlebar: 28px;
}
```

**Never hardcode values in components.** Use Tailwind classes that reference these tokens:

```tsx
// Good - uses Tailwind classes
<div className="bg-stone-900 shadow-md">

// Bad - hardcoded values
<div className="bg-[#1c1917] shadow-[0_4px_6px_rgba(0,0,0,0.7)]">

// Good - CSS variable for non-standard values
<div className="top-[--spacing-titlebar]">
```

### Color Palette

**UI Colors:**

| Token   | Purpose                              |
|---------|--------------------------------------|
| `ember` | Primary accent (volcanic orange)     |
| `molten`| Secondary accent (gold/amber)        |
| `blood` | Danger states, hardcore mode         |
| `bone`  | Muted text, subtle highlights        |
| `stone` | Neutral backgrounds (warm gray)      |
| `ash`   | Disabled/muted states (cool gray)    |

**Class Colors** (character identity):

| Token     | Class     | Concept                    |
|-----------|-----------|----------------------------|
| `blood`   | Warrior   | Martial aggression         |
| `arcane`  | Sorceress | Mystical deep blue         |
| `verdant` | Ranger    | Forest moss green          |
| `molten`  | Huntress  | Golden predator            |
| `spirit`  | Monk      | Contemplative violet       |
| `ember`   | Mercenary | Volcanic fire              |
| `hex`     | Witch     | Dark magic magenta         |
| `primal`  | Druid     | Ancient nature teal        |

### Class Colors Utility

Use `@/utils/class-colors.ts` for character-specific styling:

```tsx
import { getClassTextColor, getClassBorderColor, getClassTheme } from '@/utils/class-colors';

// Returns Tailwind classes
getClassTextColor('Warrior')   // 'text-blood-400'
getClassBorderColor('Warrior') // 'border-blood-500'
getClassTheme('Warrior')       // 'blood'

// For charts (returns hex values from CSS variables)
getClassHexColor('Warrior')    // '#dc2626'
```

### Shadow Implementation

**IMPORTANT:** Due to a WebKitGTK compositor bug in Tauri on Linux, all shadows MUST use `filter: drop-shadow()` instead of `box-shadow`. Never mix shadow types. See ADR-002 in `decisions.md`.

**Use these filter-based shadow classes (defined in `globals.css`):**

| Class                 | Use case                              |
|-----------------------|---------------------------------------|
| `.card-shadow`        | Cards, elevated content               |
| `.chrome-shadow-top`  | Bottom-docked panels (statusbar)      |
| `.chrome-shadow-right`| Left-docked panels (sidebar)          |
| `.chrome-shadow-bottom`| Top-docked panels (titlebar)         |

**DO NOT use Tailwind's `shadow-*` utilities** (e.g., `shadow-md`, `shadow-lg`) - these use `box-shadow` and will cause rendering issues when mixed with `filter: drop-shadow()`.

The theme variables (`--shadow-top`, `--shadow-right`, etc.) are defined for reference but should only be used via the filter-based utility classes above.

### Z-Index Scale

Consistent layering prevents z-index conflicts. Use these values:

| Class   | Value | Use case                                      |
|---------|-------|-----------------------------------------------|
| `z-0`   | 0     | Base content (default)                        |
| `z-10`  | 10    | Elevated cards, hover states (if needed)      |
| `z-20`  | 20    | Dropdowns, popovers, tooltips                 |
| `z-30`  | 30    | Fixed UI chrome (titlebar, sidebar, statusbar)|
| `z-40`  | 40    | Notifications, toasts                         |
| `z-50`  | 50    | Modals, dialogs (blocking UI)                 |

**Guidelines:**
- Fixed chrome (titlebar, sidebar, statusbar) uses `z-30` to stay above content but below modals
- Dropdowns/tooltips use `z-20` - they appear above content but below fixed chrome
- Modals use `z-50` - they block everything else
- Avoid arbitrary z-index values; stick to the scale

**Note on shadows:** All shadows use `filter: drop-shadow()` with GPU hints due to a WebKitGTK compositor bug. See ADR-002 in `decisions.md` for the full investigation.

### Component Styles

Each component has a co-located `.styles.ts` file containing Tailwind class compositions:

```
components/
  ui/
    button/
      button.tsx
      button.styles.ts  # Contains buttonStyles object
```

**Guidelines:**
- Use theme colors (`stone-*`, `ember-*`) instead of default Tailwind colors
- Use theme shadows (`shadow-md`, `shadow-right`) instead of arbitrary values
- Keep styles in the `.styles.ts` file, not inline in components
- Reference CSS variables for spacing: `top-[--spacing-titlebar]`

### Background Pattern

The app uses a volcanic background image with a gradient overlay, applied via CSS multiple backgrounds in `globals.css`:

```css
.app-background {
  background:
    linear-gradient(...overlay...),
    url("/background.png") center / cover no-repeat fixed;
}
```

### Design System Reference

All design tokens are defined in `globals.css`. Refer to the `@theme` block for:
- Color palettes and their intended usage
- Shadow scale with high opacity for dark backgrounds
- Layout spacing constants

There is no separate theme.ts file - everything is CSS-native for proper Tailwind integration.

### Importing Shared Theme

**In CSS files:**
```css
@import "tailwindcss";
@import "@poe2-overlord/theme/tokens.css";
```

**CRITICAL:** `@import "tailwindcss"` must come BEFORE the theme import. Tailwind v4 merges imported `@theme` blocks with the base theme.

**In TypeScript/React files:**
```tsx
import { cn } from '@poe2-overlord/theme';
import { getThemeHexColor } from '@poe2-overlord/theme';

// cn() - merge and deduplicate Tailwind classes
const className = cn('bg-stone-900', props.className);

// getThemeHexColor() - read CSS variable value
const hexColor = getThemeHexColor('ember-500'); // Returns '#f97316'
```

**Game-domain utilities** (character/league/act colors) remain in frontend:
```tsx
import { getClassTextColor } from '@/utils/class-colors';
import { getLeagueHexColor } from '@/utils/league-colors';
import { getActHexColor } from '@/utils/act-colors';
```

---

## SQLite Repository Pattern

### Overview

All backend data persistence uses SQLite via sqlx 0.8 with runtime queries (not compile-time macros). Each domain has a `*SqliteRepository` that implements the existing trait interface.

### Repository Structure

```
domain/
  <domain_name>/
    traits.rs           # Repository trait definition
    sqlite_repository.rs # SQLite implementation
    models.rs           # Domain models (same as before)
    service.rs          # Service (uses trait, no knowledge of implementation)
```

### Basic Pattern

```rust
use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::errors::AppResult;
use super::traits::MyRepository;
use super::models::MyModel;

pub struct MySqliteRepository {
    pool: SqlitePool,
}

impl MySqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MyRepository for MySqliteRepository {
    async fn load(&self) -> AppResult<MyModel> {
        // Use runtime queries with .bind() for parameters
        let row: (String, i64) = sqlx::query_as(
            "SELECT name, value FROM my_table WHERE id = ?"
        )
        .bind(1)
        .fetch_one(&self.pool)
        .await?;

        let (name, value) = row;
        Ok(MyModel { name, value: value as u32 })
    }

    async fn save(&self, model: &MyModel) -> AppResult<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO my_table (id, name, value) VALUES (1, ?, ?)"
        )
        .bind(&model.name)
        .bind(model.value as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
```

### Key Patterns

#### 1. Runtime Queries (Not Macros)

**DO THIS:**
```rust
let result = sqlx::query("SELECT * FROM table WHERE id = ?")
    .bind(value)
    .fetch_one(&pool)
    .await?;
```

**NOT THIS:**
```rust
// Requires DATABASE_URL at compile time - don't use
let result = sqlx::query!("SELECT * FROM table WHERE id = ?", value)
    .fetch_one(&pool)
    .await?;
```

#### 2. Type Conversions

SQLite stores everything as INTEGER, TEXT, REAL, or BLOB. Convert at the Rust boundary:

```rust
// Rust u32 → SQLite INTEGER
.bind(value as i64)

// SQLite INTEGER → Rust u32
let value = row_value as u32;

// Rust bool → SQLite INTEGER
.bind(if is_active { 1 } else { 0 })

// SQLite INTEGER → Rust bool
let is_active = row_value != 0;

// Rust DateTime<Utc> → SQLite TEXT (RFC3339)
.bind(timestamp.to_rfc3339())

// SQLite TEXT → Rust DateTime<Utc>
let timestamp = chrono::DateTime::parse_from_rfc3339(&row_value)?
    .with_timezone(&Utc);

// Rust Vec<String> → SQLite TEXT (JSON)
let json = serde_json::to_string(&vec_of_strings)?;
.bind(&json)

// SQLite TEXT → Rust Vec<String>
let vec: Vec<String> = serde_json::from_str(&row_value)?;
```

#### 3. Transactions

Use transactions for multi-step operations to ensure atomicity:

```rust
async fn save_complex_data(&self, data: &ComplexData) -> AppResult<()> {
    // Begin transaction
    let mut tx = self.pool.begin().await?;

    // Step 1: Update main table
    sqlx::query("UPDATE parent SET name = ? WHERE id = ?")
        .bind(&data.name)
        .bind(&data.id)
        .execute(&mut *tx)
        .await?;

    // Step 2: Upsert child records
    for child in &data.children {
        sqlx::query(
            "INSERT INTO child (parent_id, value) VALUES (?, ?)
             ON CONFLICT(parent_id) DO UPDATE SET value = excluded.value"
        )
        .bind(&data.id)
        .bind(&child.value)
        .execute(&mut *tx)
        .await?;
    }

    // Commit transaction (if any step fails, entire transaction rolls back)
    tx.commit().await?;
    Ok(())
}
```

#### 4. Shared Zone Lookup Helper

When referencing zones by ID, use the shared helper in `infrastructure/database/helpers.rs`:

```rust
use crate::infrastructure::database::get_or_create_zone_id_tx;

async fn enter_zone(&self, character_id: &str, zone_name: &str) -> AppResult<()> {
    let mut tx = self.pool.begin().await?;

    // Get or create zone_metadata entry, returns integer ID
    let zone_id = get_or_create_zone_id_tx(&mut tx, zone_name).await?;

    // Use zone_id in other queries
    sqlx::query(
        "UPDATE characters SET current_zone_id = ? WHERE id = ?"
    )
    .bind(zone_id)
    .bind(character_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
```

**Available variants:**
- `get_or_create_zone_id_tx(&mut tx, zone_name)` - For use within transactions (creates stubs)
- `get_or_create_zone_id_pool(&pool, zone_name)` - For use with pool (creates stubs)
- `get_or_create_zone_id(executor, zone_name)` - Generic (read-only, doesn't create stubs)

#### 5. Batch Loading to Avoid N+1

When loading multiple related entities, use batch queries instead of loops:

```rust
async fn load_all_characters(&self) -> AppResult<Vec<CharacterData>> {
    // Query 1: All characters
    let characters: Vec<Character> = sqlx::query_as(
        "SELECT id, name, level FROM characters"
    )
    .fetch_all(&self.pool)
    .await?;

    // Query 2: All zone_stats with metadata joined
    let zone_stats: Vec<ZoneStatsRow> = sqlx::query_as(
        "SELECT zs.character_id, zs.duration, zm.zone_name, zm.act
         FROM zone_stats zs
         JOIN zone_metadata zm ON zs.zone_id = zm.id"
    )
    .fetch_all(&self.pool)
    .await?;

    // Group zone_stats by character_id in memory
    let mut stats_by_char: HashMap<String, Vec<ZoneStats>> = HashMap::new();
    for row in zone_stats {
        stats_by_char.entry(row.character_id).or_default().push(row.into());
    }

    // Assemble CharacterData
    let mut result = Vec::new();
    for character in characters {
        let zones = stats_by_char.remove(&character.id).unwrap_or_default();
        result.push(CharacterData {
            id: character.id,
            name: character.name,
            zones,
        });
    }

    Ok(result)
}
```

#### 6. Single-Row Tables

For configuration/settings tables that only ever have one row:

```sql
CREATE TABLE app_config (
    id INTEGER PRIMARY KEY CHECK (id = 1) DEFAULT 1,
    value TEXT NOT NULL
);
```

```rust
async fn save(&self, config: &Config) -> AppResult<()> {
    // INSERT OR REPLACE ensures single row
    sqlx::query(
        "INSERT OR REPLACE INTO app_config (id, value) VALUES (1, ?)"
    )
    .bind(&config.value)
    .execute(&self.pool)
    .await?;
    Ok(())
}

async fn load(&self) -> AppResult<Config> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM app_config WHERE id = 1"
    )
    .fetch_optional(&self.pool)
    .await?;

    match row {
        Some((value,)) => Ok(Config { value }),
        None => Ok(Config::default()),
    }
}
```

#### 7. Foreign Keys and Cascades

Use foreign keys with appropriate cascade behaviors:

```sql
-- Cascading delete (child deleted when parent deleted)
CREATE TABLE zone_stats (
    character_id TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    ...
);

-- Restrict (prevent parent deletion if children exist)
CREATE TABLE characters (
    current_zone_id INTEGER REFERENCES zone_metadata(id) ON DELETE RESTRICT,
    ...
);

-- Set null (child field nulled when parent deleted)
CREATE TABLE characters (
    current_zone_id INTEGER REFERENCES zone_metadata(id) ON DELETE SET NULL,
    ...
);
```

### Service Integration

Services receive repositories via dependency injection (no knowledge of SQLite):

```rust
// service_registry.rs
let pool = DatabasePool::new(&db_path).await?;

let my_repo = Arc::new(MySqliteRepository::new(pool.clone()))
    as Arc<dyn MyRepository + Send + Sync>;

let my_service = MyServiceImpl::new(my_repo, event_bus);
app.manage(my_service);
```

Services remain database-agnostic and only depend on the trait:

```rust
// service.rs
pub struct MyServiceImpl {
    repository: Arc<dyn MyRepository + Send + Sync>,
    event_bus: Arc<EventBus>,
}

impl MyServiceImpl {
    pub fn new(
        repository: Arc<dyn MyRepository + Send + Sync>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self { repository, event_bus }
    }
}
```

### Error Handling

The `AppError` type has `From<sqlx::Error>` implemented with SQLite error code mapping:

```rust
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::Validation { ... },
            sqlx::Error::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    if code == "2067" || code == "1555" {  // UNIQUE constraint
                        return AppError::Validation { ... };
                    }
                    if code == "787" {  // Foreign key constraint
                        return AppError::Validation { ... };
                    }
                }
                AppError::Internal { ... }
            },
            _ => AppError::Internal { ... },
        }
    }
}
```

This means repository methods can use `?` and let errors propagate naturally:

```rust
async fn save(&self, model: &Model) -> AppResult<()> {
    sqlx::query("INSERT INTO table (id, name) VALUES (?, ?)")
        .bind(&model.id)
        .bind(&model.name)
        .execute(&self.pool)
        .await?;  // ? operator converts sqlx::Error to AppError

    Ok(())
}
```

### Migration Management

Migrations live in `infrastructure/database/migrations/` and run automatically on startup:

```
infrastructure/
  database/
    migrations/
      001_initial_schema.sql
      002_add_league_column.sql
```

sqlx creates a `_sqlx_migrations` table to track which migrations have been applied. Migrations are idempotent and run in order.

### Testing Pattern (Future)

Use in-memory SQLite for unit tests:

```rust
#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let pool = setup_test_db().await;
        let repo = MySqliteRepository::new(pool);

        // Test implementation...
    }
}
```

### SQLite Cache Migration Pattern (Ephemeral Data)

When migrating ephemeral TTL caches from JSON to SQLite (e.g., economy cache from poe.ninja API), follow this pattern:

#### When to Use This Pattern

Use this pattern for:
- **API response caching** with TTL (Time-To-Live)
- **Ephemeral data** that rebuilds from external sources
- **Data with lifecycles** (active/inactive states)
- **Cross-domain aggregation** needs (JOINs, search)

**Don't migrate** if:
- Data is bundled/read-only (keep as JSON)
- Cache is pure runtime (no persistence needed)

#### Migration Checklist

**1. Design Schema with Parent-Child FK Relationship**

Example from economy domain:

```sql
-- Parent: Exchange rate context per league+type
CREATE TABLE economy_exchange_rates (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    league            TEXT    NOT NULL,
    is_hardcore       INTEGER NOT NULL DEFAULT 0,
    economy_type      TEXT    NOT NULL,
    -- ... metadata fields ...
    fetched_at        TEXT    NOT NULL,  -- RFC3339, sent to frontend
    last_updated      TEXT    NOT NULL,  -- RFC3339, used for TTL check
    UNIQUE(league, is_hardcore, economy_type)
);

-- Child: Individual items with FK
CREATE TABLE currency_items (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    exchange_rate_id  INTEGER NOT NULL REFERENCES economy_exchange_rates(id) ON DELETE CASCADE,
    currency_id       TEXT    NOT NULL,
    -- ... item fields ...
    is_active         INTEGER NOT NULL DEFAULT 1,  -- Lifecycle flag
    last_updated      TEXT    NOT NULL,
    UNIQUE(exchange_rate_id, currency_id)
);

-- Indexes for queries
CREATE INDEX idx_exchange_rates_lookup ON economy_exchange_rates (league, is_hardcore, economy_type);
CREATE INDEX idx_currency_items_exchange_rate ON currency_items (exchange_rate_id);
CREATE INDEX idx_currency_items_name ON currency_items (name);
CREATE INDEX idx_currency_items_value ON currency_items (primary_value DESC);
```

**Key decisions:**
- `ON DELETE CASCADE` - Child items deleted when parent removed
- `UNIQUE` constraints enable upserts
- `is_active` flag for soft deletes (preserves history, allows manual deactivation)
- Separate `fetched_at` (display) and `last_updated` (TTL logic)

**2. Create Repository Trait**

```rust
// traits.rs
#[async_trait]
pub trait EconomyRepository: Send + Sync {
    /// Load data if fresh (within TTL)
    async fn load_fresh_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        ttl_seconds: u64,
    ) -> AppResult<Option<CurrencyExchangeData>>;

    /// Load data regardless of TTL (stale fallback)
    async fn load_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
    ) -> AppResult<Option<CurrencyExchangeData>>;

    /// Upsert data, preserving is_active
    async fn save_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        data: &CurrencyExchangeData,
    ) -> AppResult<()>;

    /// Cross-cache queries (JOINs)
    async fn load_top_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        limit: u32,
    ) -> AppResult<Vec<TopCurrencyItem>>;

    async fn search_currencies(
        &self,
        league: &str,
        is_hardcore: bool,
        query: &str,
    ) -> AppResult<Vec<CurrencySearchResult>>;
}
```

**3. Implement Repository with TTL Logic**

```rust
// repository.rs
impl EconomyRepository for EconomyRepositoryImpl {
    async fn load_fresh_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        ttl_seconds: u64,
    ) -> AppResult<Option<CurrencyExchangeData>> {
        // Query parent row
        let rate_result = sqlx::query(
            "SELECT id, last_updated FROM economy_exchange_rates
             WHERE league = ? AND is_hardcore = ? AND economy_type = ?"
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(economy_type.as_str())
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = rate_result else {
            return Ok(None); // No data
        };

        let exchange_rate_id: i64 = row.get("id");
        let last_updated: String = row.get("last_updated");

        // TTL check in Rust (not SQL)
        if !Self::is_fresh(&last_updated, ttl_seconds) {
            return Ok(None); // Stale
        }

        // Load child items and build domain model
        let data = self.build_exchange_data(exchange_rate_id, ...).await?;
        Ok(Some(data))
    }

    async fn save_exchange_data(
        &self,
        league: &str,
        is_hardcore: bool,
        economy_type: EconomyType,
        data: &CurrencyExchangeData,
    ) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now().to_rfc3339();

        // 1. Upsert parent row
        sqlx::query(
            "INSERT INTO economy_exchange_rates (league, is_hardcore, economy_type, ..., last_updated)
             VALUES (?, ?, ?, ..., ?)
             ON CONFLICT(league, is_hardcore, economy_type) DO UPDATE SET
                ..., last_updated = excluded.last_updated"
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(economy_type.as_str())
        // ... bind all fields ...
        .bind(&now)
        .execute(&mut *tx)
        .await?;

        // 2. Get parent ID
        let exchange_rate_id: i64 = sqlx::query_scalar(
            "SELECT id FROM economy_exchange_rates WHERE league = ? AND is_hardcore = ? AND economy_type = ?"
        )
        .bind(league)
        .bind(is_hardcore as i32)
        .bind(economy_type.as_str())
        .fetch_one(&mut *tx)
        .await?;

        // 3. Upsert each child (preserves is_active!)
        for item in &data.items {
            sqlx::query(
                "INSERT INTO currency_items (exchange_rate_id, currency_id, ..., is_active, last_updated)
                 VALUES (?, ?, ..., 1, ?)
                 ON CONFLICT(exchange_rate_id, currency_id) DO UPDATE SET
                    ..., last_updated = excluded.last_updated
                    -- is_active intentionally omitted from UPDATE"
            )
            .bind(exchange_rate_id)
            .bind(&item.id)
            // ... bind all fields ...
            .bind(&now)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
```

**Critical:** Omit `is_active` from the `ON CONFLICT DO UPDATE` clause. This preserves manual deactivations while still updating all other fields on API refresh.

**4. Refactor Service to Use Repository**

```rust
// service.rs
pub struct EconomyService {
    client: reqwest::Client,
    in_flight: Arc<RwLock<HashMap<String, Arc<Semaphore>>>>,
    repository: Arc<dyn EconomyRepository>,  // Added
}

impl EconomyService {
    pub fn new(repository: Arc<dyn EconomyRepository>) -> AppResult<Self> {
        // ...
        Ok(Self { client, in_flight, repository })
    }

    pub async fn fetch_data(&self, ...) -> AppResult<Data> {
        // FAST PATH: Check fresh cache (no lock)
        if let Some(data) = self.repository.load_fresh_exchange_data(..., TTL).await? {
            return Ok(data);
        }

        // Acquire semaphore for deduplication
        let key = Self::cache_key(...);
        let semaphore = { /* ... */ };
        let _permit = semaphore.acquire().await?;

        // Re-check after acquiring lock (coalescing)
        if let Some(data) = self.repository.load_fresh_exchange_data(..., TTL).await? {
            self.cleanup_semaphore(&key).await;
            return Ok(data);
        }

        // Still stale - fetch from API
        match self.fetch_from_api(...).await {
            Ok(data) => {
                // Save to DB
                self.repository.save_exchange_data(..., &data).await?;
                Ok(data)
            }
            Err(e) => {
                // Graceful degradation - return stale data if available
                if let Some(mut stale) = self.repository.load_exchange_data(...).await? {
                    stale.is_stale = Some(true);
                    Ok(stale)
                } else {
                    Err(e)
                }
            }
        }
    }
}
```

**Key service changes:**
- Remove `FileService` dependency
- Remove `get_cache_path()` and file I/O logic
- Replace cache checks with repository calls
- Keep HTTP client, retry logic, semaphore deduplication

**5. Update Service Registry (DI)**

```rust
// service_registry.rs
let economy_repo = Arc::new(EconomyRepositoryImpl::new(pool.clone()))
    as Arc<dyn EconomyRepository + Send + Sync>;

let economy_service = EconomyService::new(economy_repo)?;
app.manage(economy_service);
```

**6. Update Tests with Mock Repository**

```rust
// service_test.rs
struct MockEconomyRepository;

#[async_trait]
impl EconomyRepository for MockEconomyRepository {
    async fn load_fresh_exchange_data(...) -> AppResult<Option<Data>> {
        Ok(None) // Always miss cache
    }
    // ... implement all trait methods ...
}

#[test]
fn test_service_creation() {
    let mock_repo = Arc::new(MockEconomyRepository) as Arc<dyn EconomyRepository + Send + Sync>;
    let service = EconomyService::new(mock_repo).expect("Failed to create service");
    // ...
}
```

#### TTL Checking Pattern

**Do TTL checks in Rust, not SQL:**

```rust
fn is_fresh(last_updated: &str, ttl_seconds: u64) -> bool {
    chrono::DateTime::parse_from_rfc3339(last_updated)
        .ok()
        .map(|time| {
            let now = Utc::now();
            let cached_time_utc = time.with_timezone(&Utc);
            let elapsed = now.signed_duration_since(cached_time_utc);
            let ttl_i64 = i64::try_from(ttl_seconds).unwrap_or(i64::MAX);
            elapsed.num_seconds() < ttl_i64
        })
        .unwrap_or(false)
}
```

**Why not SQL datetime?**
- SQLite datetime functions are quirky with timezones
- Rust chrono handles RFC3339 parsing correctly
- TTL logic stays in application layer (easier to test/modify)

#### Cross-Cache Query Pattern

Use JOINs to query across all cached entries:

```rust
async fn load_top_currencies(&self, league: &str, is_hardcore: bool, limit: u32) -> AppResult<Vec<TopCurrencyItem>> {
    let rows = sqlx::query(
        "SELECT ci.currency_id, ci.name, ci.image_url, er.economy_type,
                ci.primary_value, er.primary_currency_name, ci.volume
         FROM currency_items ci
         JOIN economy_exchange_rates er ON ci.exchange_rate_id = er.id
         WHERE er.league = ? AND er.is_hardcore = ? AND ci.is_active = 1
         ORDER BY ci.primary_value DESC
         LIMIT ?"
    )
    .bind(league)
    .bind(is_hardcore as i32)
    .bind(limit)
    .fetch_all(&self.pool)
    .await?;

    // Map rows to domain models...
}
```

**Benefits:**
- Single query aggregates across all economy types
- SQL handles sorting and limiting
- `is_active = 1` filters deactivated items

#### Migration Strategy

**For ephemeral caches (economy, etc.):**
1. No data migration needed - cache is ephemeral
2. DB starts empty
3. First API fetch rebuilds cache in DB
4. Old JSON files can be manually deleted (no longer read)

**Deployment checklist:**
1. Create migration SQL file (`002_economy_cache.sql`)
2. Migration runs automatically on app startup (sqlx)
3. First use fetches from API and populates DB
4. Verify cache hit rate in logs ("Fresh cache hit")

#### Common Pitfalls

❌ **Don't update `is_active` on upsert:**
```sql
ON CONFLICT DO UPDATE SET
    value = excluded.value,
    is_active = excluded.is_active  -- Wrong! Overwrites manual deactivations
```

✅ **Omit `is_active` from UPDATE:**
```sql
ON CONFLICT DO UPDATE SET
    value = excluded.value
    -- is_active intentionally omitted
```

❌ **Don't check TTL in SQL:**
```sql
WHERE datetime(last_updated) > datetime('now', '-600 seconds')  -- Timezone issues
```

✅ **Check TTL in Rust:**
```rust
if !Self::is_fresh(&last_updated, ttl_seconds) {
    return Ok(None);
}
```

❌ **Don't use snapshot replacement:**
```sql
DELETE FROM currency_items WHERE exchange_rate_id = ?;
-- Insert all new rows
```

✅ **Use per-item upserts:**
```sql
INSERT ... ON CONFLICT(exchange_rate_id, currency_id) DO UPDATE ...
```

#### Related Documentation

- **ADR-007** in `decisions.md` - SQLite migration decision with economy domain details
- **Economy Migration** - `domain/economy/repository.rs` reference implementation
- **Schema** - `infrastructure/database/migrations/002_economy_cache.sql`

---

### Related Documentation

- **ADR-007** in `decisions.md` - Full SQLite migration decision
- **PRD** in `.ai/tasks/prd-sqlite-migration.md` - Implementation plan
- **Schema** in `infrastructure/database/migrations/001_initial_schema.sql`
