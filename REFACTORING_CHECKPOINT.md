# Character Domain Refactoring - Checkpoint Document

## Overview
This document captures the progress of refactoring the character domain to improve separation of concerns, reduce complexity, and establish cleaner domain boundaries.

## Completed Phases

### Phase 1: Create zone_tracking Domain ✅

**Goal:** Separate zone tracking statistics from character identity management.

**What was done:**
1. Created new `packages/backend/src/domain/zone_tracking/` domain with:
   - `models.rs`: Moved `ZoneStats` and `TrackingSummary` from character domain
   - `traits.rs`: Created `ZoneTrackingService` trait with methods:
     - `enter_zone(character_id, zone_name)`
     - `leave_zone(character_id, zone_name)`
     - `record_death(character_id)` - records death in active zone
     - `add_zone_time(character_id, zone_name, seconds)`
     - `get_zone_stats(character_id)`
     - `get_summary(character_id)`
     - `get_active_zone(character_id)`
     - `finalize_active_zones(character_id)`
     - `update_summary(character_id)`
   - `service.rs`: Implemented `ZoneTrackingServiceImpl`
   - `mod.rs`: Public exports

2. Updated character domain:
   - Removed 464 lines of zone tracking code from `character/models.rs`
   - Added import: `use crate::domain::zone_tracking::{TrackingSummary, ZoneStats};`
   - Character domain now re-exports these types for convenience
   - Kept enrichment types (`CharacterDataResponse`, `EnrichedZoneStats`) temporarily

3. Updated imports across codebase:
   - `domain/mod.rs`: Exports zone_tracking types
   - `lib.rs`: Imports from zone_tracking instead of character
   - `character/service.rs`: Imports ZoneStats from zone_tracking

**File Storage:** Still uses single file per character (`character_data_{id}.json`)

---

### Phase 2: Simplify CharacterData Structure ✅

**Goal:** Group character fields logically and remove zone management helper methods.

**What was done:**
1. Created new struct groupings in `character/models.rs`:
   ```rust
   pub struct CharacterProfile {
       pub name: String,
       pub class: CharacterClass,
       pub ascendency: Ascendency,
       pub league: League,
       pub hardcore: bool,
       pub solo_self_found: bool,
       pub level: u32,
   }

   pub struct CharacterTimestamps {
       pub created_at: DateTime<Utc>,
       pub last_played: Option<DateTime<Utc>>,
       pub last_updated: DateTime<Utc>,
   }

   pub struct CharacterData {
       pub id: String,
       #[serde(flatten)]  // Maintains same JSON structure
       pub profile: CharacterProfile,
       #[serde(flatten)]
       pub timestamps: CharacterTimestamps,
       pub current_location: Option<LocationState>,
       pub summary: TrackingSummary,
       pub zones: Vec<ZoneStats>,
       pub walkthrough_progress: WalkthroughProgress,
   }
   ```

2. Removed helper methods from CharacterData:
   - `find_zone()` → direct iteration: `character_data.zones.iter().find(|z| z.zone_name == name)`
   - `find_zone_mut()` → direct iteration with `iter_mut()`
   - `upsert_zone()` → direct manipulation of `character_data.zones`
   - `get_active_zone()` → `character_data.zones.iter().find(|z| z.is_active)`
   - `update_summary()` → `TrackingSummary::from_zones(&id, &zones)`

3. Updated all field access throughout codebase:
   - `character.name` → `character.profile.name`
   - `character.level` → `character.profile.level`
   - `character.created_at` → `character.timestamps.created_at`
   - `character.last_played` → `character.timestamps.last_played`

**Files modified:**
- `character/models.rs`
- `character/service.rs`
- `log_analysis/service.rs`

**Benefits:**
- Cleaner data model with logical grouping
- Same JSON serialization (thanks to `#[serde(flatten)]`)
- No convenience methods means clearer intent

---

### Phase 3: Delegate Zone Operations to ZoneTrackingService ✅

**Goal:** Character service focuses on identity, delegates tracking to zone_tracking service.

**What was done:**
1. Added `zone_tracking` dependency to `CharacterServiceImpl`:
   ```rust
   pub struct CharacterServiceImpl {
       repository: Arc<dyn CharacterRepository + Send + Sync>,
       event_bus: Arc<EventBus>,
       zone_config: Arc<dyn ZoneConfigurationService>,
       wiki_service: Arc<dyn WikiScrapingService>,
       config_service: Arc<dyn ConfigurationService>,
       zone_tracking: Arc<dyn ZoneTrackingService>,  // NEW
   }
   ```

2. Updated `with_default_repository()` to create and inject ZoneTrackingService:
   ```rust
   let zone_tracking = Arc::new(ZoneTrackingServiceImpl::new(
       repository.clone(),
       event_bus.clone(),
   ));
   ```

3. Delegated all zone operations:
   - `enter_zone()` → calls `zone_tracking.enter_zone()`
   - `enter_zone_with_metadata()` → delegates tracking, only updates `current_location`
   - `leave_zone()` → calls `zone_tracking.leave_zone()`
   - `record_death()` → calls `zone_tracking.record_death()`
   - `add_zone_time()` → calls `zone_tracking.add_zone_time()`
   - `finalize_all_active_zones()` → calls `zone_tracking.finalize_active_zones()` for each character

4. Removed direct zone manipulation from CharacterService:
   - No longer modifies `character_data.zones` directly
   - No longer calculates summaries
   - Only manages `current_location` field

**CharacterService responsibilities NOW:**
- ✅ Character CRUD (create, read, update, delete)
- ✅ Character index management
- ✅ Current location reference (just the zone name, not stats)
- ✅ Walkthrough progress
- ✅ Coordination with zone_config and wiki services
- ❌ No zone statistics management

**ZoneTrackingService responsibilities:**
- ✅ All zone visit tracking
- ✅ Zone timer management
- ✅ Death recording
- ✅ Summary calculation
- ✅ Zone statistics

**Benefits:**
- Character service ~200 lines shorter
- Clear separation: identity vs. statistics
- Services can evolve independently
- Easier to test (mock zone_tracking)

---

## In-Progress Phase

### Phase 4: Move Scene Processing to log_analysis Domain 🔄

**Goal:** Log analysis domain determines scene types and delegates to appropriate services.

**Current State:**
Scene processing logic is currently in `CharacterServiceImpl`:
- `is_act_name()` method filters out act names (lines 733-748)
- `process_scene_change()` method (lines 751-859):
  - Checks if scene is empty
  - Filters act names
  - Ensures zone metadata exists
  - Creates placeholders if needed
  - Triggers wiki fetches
  - Enters zone
  - Updates last_played timestamp
  - Emits events
  - Logs zone entry

**What needs to happen:**
1. Add dependencies to `LogAnalysisServiceImpl`:
   - `zone_config: Arc<dyn ZoneConfigurationService>`
   - `zone_tracking: Arc<dyn ZoneTrackingService>`

2. Update `LogAnalysisServiceImpl::new()` and `with_repositories()` constructors to accept these dependencies

3. Move `is_act_name()` from CharacterService to LogAnalysisService as a static helper

4. Create new scene processing in LogAnalysisService:
   ```rust
   async fn process_scene_change_with_error_handling(
       character_service: &Arc<dyn CharacterService>,
       walkthrough_service: &Arc<dyn WalkthroughService>,
       zone_config: &Arc<dyn ZoneConfigurationService>,
       zone_tracking: &Arc<dyn ZoneTrackingService>,
       content: &str,
       character_id: &str,
       zone_level: Option<u32>,
   ) {
       let zone_name = content.trim();
       
       // Filter act names
       if Self::is_act_name(zone_name) { return; }
       
       // Ensure zone metadata exists (or create placeholder)
       let zone_metadata = zone_config.get_or_create_zone(zone_name).await;
       
       // Update character location
       let mut character_data = character_service.get_character(character_id).await?;
       character_data.current_location = Some(LocationState::new(zone_name));
       character_data.timestamps.last_played = Some(Utc::now());
       character_service.save_character_data(&character_data).await?;
       
       // Track zone visit
       zone_tracking.enter_zone(character_id, zone_name).await?;
       
       // Handle walkthrough
       walkthrough_service.handle_scene_change(character_id, content).await?;
   }
   ```

5. Update `LogAnalysisServiceImpl::process_single_line()` to pass new dependencies

6. Update service initialization in `application/service_registry.rs`:
   ```rust
   let log_analysis_service = LogAnalysisServiceImpl::new(
       config,
       character_service.clone(),
       server_monitoring_service.clone(),
       walkthrough_service.clone(),
       zone_config_service.clone(),     // ADD THIS
       zone_tracking_service.clone(),   // ADD THIS
   )?;
   ```

7. Remove from CharacterService:
   - `process_scene_content()` method
   - `process_scene_content_with_zone_level()` method
   - `is_act_name()` method
   - `process_scene_change()` method

8. Remove from CharacterService trait (`character/traits.rs`):
   ```rust
   // REMOVE THESE:
   async fn process_scene_content(...) -> Result<Option<SceneChangeEvent>, AppError>;
   async fn process_scene_content_with_zone_level(...) -> Result<Option<SceneChangeEvent>, AppError>;
   ```

**Files to modify:**
- `log_analysis/service.rs` - add dependencies, add scene processing
- `application/service_registry.rs` - pass new dependencies to LogAnalysisService
- `character/service.rs` - remove scene processing methods
- `character/traits.rs` - remove scene processing from trait

**Challenges:**
- Need to create ZoneTrackingService before LogAnalysisService
- LogAnalysisService initialization is complex (in service_registry.rs)
- Must maintain walkthrough integration

---

## Remaining Phases

### Phase 5: Move Enrichment to Command Layer

**Goal:** Remove enrichment logic from service layer, do it at API boundary.

**Current State:**
- `CharacterDataResponse` exists in character/models.rs
- `EnrichedZoneStats` exists in character/models.rs
- `EnrichedLocationState` exists in character/models.rs
- Character service has `get_character_response()` and `get_all_characters_response()` that enrich data

**What needs to happen:**
1. Move enrichment types to `character/commands.rs` (or new `character/responses.rs`)

2. Update commands to do enrichment:
   ```rust
   #[tauri::command]
   pub async fn get_character(
       character_id: String,
       character_service: State<'_, Box<dyn CharacterService>>,
       zone_config: State<'_, Box<dyn ZoneConfigurationService>>,
   ) -> CommandResult<EnrichedCharacterResponse> {
       // Get raw character data
       let character = character_service.get_character(&character_id).await?;
       
       // Enrich zones with metadata
       let enriched_zones = enrich_zones(&character.zones, &zone_config).await?;
       
       // Build response
       Ok(EnrichedCharacterResponse {
           ...character,
           zones: enriched_zones,
       })
   }
   ```

3. Remove enrichment methods from CharacterService:
   - Remove `get_character_response()`
   - Remove `get_all_characters_response()`
   - Keep only `get_character()` and `get_all_characters()` that return raw data

4. Create helper functions for enrichment in commands:
   ```rust
   async fn enrich_zones(
       zones: &[ZoneStats],
       zone_config: &ZoneConfigurationService,
   ) -> Result<Vec<EnrichedZoneStats>, AppError> {
       let mut enriched = Vec::new();
       for zone in zones {
           if let Some(metadata) = zone_config.get_zone_metadata(&zone.zone_name).await {
               enriched.push(EnrichedZoneStats::from_stats_and_metadata(zone, &metadata));
           } else {
               enriched.push(EnrichedZoneStats::minimal_from_stats(zone));
           }
       }
       Ok(enriched)
   }
   ```

**Files to modify:**
- `character/commands.rs` - add enrichment logic
- `character/models.rs` - move enrichment types to commands
- `character/service.rs` - remove enrichment methods
- `character/traits.rs` - remove enrichment methods from trait

**Benefits:**
- Service layer returns pure domain data
- Enrichment happens at presentation boundary
- Different commands can enrich differently
- Easier to cache enriched data

---

### Phase 6: Clean Up Comments and Documentation

**Goal:** Remove verbose, redundant comments. Keep only essential ones.

**Guidelines:**
- Remove comments that restate method names
- Remove parameter/return type descriptions (types are self-documenting)
- Remove "what" comments, keep "why" comments
- Keep explanations of complex business logic
- Keep non-obvious behavior notes

**Examples of what to remove:**
```rust
/// Creates a new character with the provided data
/// Returns the created CharacterData
/// 
/// # Arguments
/// * `name` - The character name
/// * `class` - The character class
/// ...
```

**Examples of what to keep:**
```rust
/// Act names are filtered because they represent story progression,
/// not playable areas. The game logs act transitions as scene changes.
```

**Files to review:**
- All service implementations
- All model structs
- All trait definitions

---

## Architecture After Refactoring

```
┌─────────────────────────────────────────────────────────────┐
│                     Commands Layer                          │
│              (Enrichment happens here)                       │
└────────┬────────────────────────────────────────────────────┘
         │
    ┌────┴─────┬─────────────┬──────────────┬──────────────┐
    │          │             │              │              │
┌───▼───┐ ┌───▼────┐ ┌──────▼──────┐ ┌─────▼──────┐ ┌────▼────┐
│Character│ │  Zone  │ │    Zone     │ │    Log     │ │  Wiki   │
│ Service │ │Tracking│ │Configuration│ │  Analysis  │ │Scraping │
│         │ │Service │ │   Service   │ │  Service   │ │ Service │
└─────────┘ └────────┘ └─────────────┘ └────────────┘ └─────────┘
    │            │              │              │              │
  Profile    Tracking      Metadata        Parsing       Fetching
   Data       Stats        (Act/Town)      Events          Data
```

**Domain Responsibilities:**

**Character Domain:**
- Character identity (name, class, level, league)
- Character index (list of IDs, active character)
- Current location reference (just zone name)
- Walkthrough progress
- Validation (unique names, valid ascendency combos)

**Zone Tracking Domain:**
- Zone visit history per character
- Time tracking (entry/exit timestamps)
- Death counting
- Summary statistics (total time, deaths, zones visited)
- Active zone management

**Zone Configuration Domain:**
- Zone metadata (act, town status, bosses, etc.)
- Zone discovery (create placeholders)
- Wiki data integration
- Zone refresh scheduling

**Log Analysis Domain:**
- Log file monitoring
- Event parsing (scene changes, deaths, level ups)
- Scene type determination (zone vs act vs hideout)
- Event delegation to appropriate services

**Wiki Scraping Domain:**
- Fetch zone data from wiki
- Parse HTML content
- Extract metadata (act, level, bosses, etc.)

---

## Key Design Decisions

### 1. Single File Storage
**Decision:** Keep single `character_data_{id}.json` file per character.
**Rationale:** 
- Simpler file management
- Atomic updates
- No synchronization issues
- Domain separation at service layer, not storage

### 2. Flattened JSON with serde
**Decision:** Use `#[serde(flatten)]` for profile and timestamps.
**Rationale:**
- Maintains backward compatibility
- Cleaner code structure
- Same JSON schema for frontend

### 3. Zone Name as Foreign Key
**Decision:** `zone_name` field links ZoneStats to ZoneMetadata.
**Rationale:**
- Simple string-based relationship
- No complex ID management
- Explicit and readable

### 4. Enrichment at Command Layer
**Decision:** Services return raw data, commands enrich for frontend.
**Rationale:**
- Clear separation of concerns
- Service layer stays pure
- Different views can enrich differently
- Easier to test services

---

## Testing Checklist

When resuming, verify:

- [ ] All compilation errors resolved
- [ ] Character CRUD operations work
- [ ] Zone tracking records visits correctly
- [ ] Zone tracking records deaths correctly
- [ ] Scene changes update location and track zones
- [ ] Walkthrough progress still integrates
- [ ] Events are emitted correctly
- [ ] No data loss in character_data.json files
- [ ] Frontend can still render character data
- [ ] Wiki fetching still works

---

## Next Session TODO

1. **Complete Phase 4:** Move scene processing to log_analysis
   - Start with adding dependencies to LogAnalysisServiceImpl struct
   - Update constructors carefully
   - Move `is_act_name()` helper
   - Update service_registry.rs to create ZoneTrackingService and pass it
   - Test scene changes work end-to-end

2. **Start Phase 5:** Move enrichment to commands
   - Move enrichment types to commands.rs
   - Update get_character command to enrich
   - Remove enrichment from service

3. **Complete Phase 6:** Clean up comments

---

## Reference: Key Files and Line Numbers

**Character Domain:**
- `packages/backend/src/domain/character/models.rs` - CharacterData, CharacterProfile, CharacterTimestamps
- `packages/backend/src/domain/character/service.rs` - CharacterServiceImpl (~937 lines initially, ~750 after Phase 3)
- `packages/backend/src/domain/character/traits.rs` - CharacterService trait
- `packages/backend/src/domain/character/commands.rs` - Tauri commands

**Zone Tracking Domain:**
- `packages/backend/src/domain/zone_tracking/models.rs` - ZoneStats, TrackingSummary
- `packages/backend/src/domain/zone_tracking/service.rs` - ZoneTrackingServiceImpl
- `packages/backend/src/domain/zone_tracking/traits.rs` - ZoneTrackingService trait

**Log Analysis Domain:**
- `packages/backend/src/domain/log_analysis/service.rs` - LogAnalysisServiceImpl
- Line 262: `process_scene_change_with_error_handling` (currently calls character service)

**Service Registry:**
- `packages/backend/src/application/service_registry.rs` - Service initialization
- Line 55-62: CharacterService initialization
- Log analysis service needs zone_config and zone_tracking passed

**Wiki Fetching:**
- `character/service.rs` line 51-196: `trigger_wiki_fetch()` method
- Should move to zone_configuration domain eventually

---

## Notes and Observations

1. **Wiki Fetching Logic:** Currently in CharacterService (lines 51-196), should eventually move to ZoneConfiguration domain. The trigger_wiki_fetch method is 145 lines and handles placeholder creation, wiki fetching, and zone metadata updates.

2. **Event Publishing:** Character service still publishes `character_tracking_data_updated` events. Zone tracking service also publishes these. Consider consolidating event logic.

3. **Current Location:** Managed by Character domain but represents zone tracking data. This is acceptable since it's just a reference (zone name), not statistics.

4. **Summary Calculation:** Now done by ZoneTrackingService, but act-specific time tracking (play_time_act1, etc.) is not yet implemented. TrackingSummary has these fields but they're always 0.

5. **Backwards Compatibility:** JSON structure unchanged thanks to `#[serde(flatten)]`. Existing save files will load correctly.

---

## Success Metrics

After completion, we should have:

✅ Reduced CharacterService from ~937 to ~600 lines  
✅ Clear domain boundaries with single responsibilities  
✅ Zone tracking completely separate from character identity  
✅ Scene processing owned by log_analysis domain  
✅ Enrichment happens at API boundary, not service layer  
✅ Comments are concise and meaningful  
✅ Code is easier to test and maintain  
✅ No duplication between domains  

---

**Document created:** 2024
**Last updated:** Phase 3 complete, Phase 4 in progress
