# Session: Domain-Driven Functionality Refactoring

**Date**: 2026-01-11  
**Agent**: Claude Code (Ralph Wiggum loop + code-analyzer subagent)  
**Branch**: TBD (will be created by Ralph)  
**Status**: IN PROGRESS

## Goal
Systematically analyze and refactor entire codebase domain-by-domain, prioritizing functional correctness (bugs, gaps, inconsistencies) over style.

## Initial State
- **8 Domains** to refactor (infrastructure → core features → supporting features)
- **Subagent**: code-analyzer (functionality-first analysis)
- **Test Status**: Frontend 517 tests passing, Backend TBD

## Domain Progress

### Domains Completed: 3 / 8 (Configuration ✅, Wiki Scraping ✅, Monitoring ✅)

---

## Phase 1: Infrastructure Domains

### Domain 1: Configuration Management 🔄
**Status**: In Progress - Analysis Phase

#### Files Mapped

**Backend** (`packages/backend/src/domain/configuration/`):
- `mod.rs` - Module exports
- `models.rs` - `AppConfig`, `ZoneRefreshInterval`, `ConfigurationChangedEvent`, `ConfigurationValidationResult`, `ConfigurationFileInfo`
- `traits.rs` - `ConfigurationService` and `ConfigurationRepository` traits
- `service.rs` - `ConfigurationServiceImpl` with event publishing
- `repository.rs` - `ConfigurationRepositoryImpl` with lazy loading
- `commands.rs` - 14 Tauri commands for configuration management
- `models_test.rs` - 27 model tests

**Frontend** (`packages/frontend/src/`):
- `components/forms/settings-form/settings-form.tsx` - Main settings form component
- `components/forms/settings-form/settings-form.spec.tsx` - 19 component tests
- `components/forms/settings-form/settings-form.styles.ts` - Styles
- `types/app-config.ts` - TypeScript types for `AppConfig`, `ZoneRefreshInterval`

#### Domain Boundaries
- **FE↔BE Contract**: `AppConfig`, `ZoneRefreshInterval`, `ZoneRefreshIntervalOption` types
- **Tauri Commands**: `get_config`, `update_config`, `reset_config_to_defaults`, `get_poe_client_log_path`, `set_poe_client_log_path`, `get_default_poe_client_log_path`, `reset_poe_client_log_path_to_default`, `get_log_level`, `set_log_level`, `get_config_file_info`, `validate_config`, `get_zone_refresh_interval`, `set_zone_refresh_interval`, `get_zone_refresh_interval_options`
- **Events**: `ConfigurationChanged` event published via `EventBus`

#### Issues Found

**CRITICAL (4)**:
1. **BE**: Blocking async runtime call in `get_default_poe_client_log_path()` - can cause deadlocks (service.rs:174-176)
2. **BE**: Panic risk in Default implementation (service.rs:227-232)
3. **BE**: Race condition in repository write operations (repository.rs:117-123, 125-133)
4. **FE**: Memory leak from setTimeout not cleaned up (settings-form.tsx:71, 93)

**HIGH (8)**:
5. **BE**: Duplicated validation logic between model and repository
6. **BE**: Missing file path validation (absolute path, path traversal)
7. **BE**: Frontend-backend contract inefficiency (extra round-trips)
8. **BE**: Lost update problem in concurrent config updates
9. **FE**: Missing ConfigurationChanged event listener - stale data
10. **FE**: Weak path validation (OR logic too permissive)
11. **FE**: Poor backend validation error handling (generic messages)
12. **FE**: No pre-save validation before backend call

**MEDIUM (7)**:
13. **BE**: No debouncing on disk writes
14. **BE**: Missing lazy load error recovery flag
15. **BE**: No event publishing on load failure
16. **BE**: Unused repository methods
17. **FE**: Double backend call on reset
18. **FE**: Missing ConfigurationChangedEvent type
19. **FE**: Inconsistent error handling pattern

#### Fixes Implemented

**CRITICAL (4/4 fixed)**:
1. ✅ **BE**: Changed `get_default_poe_client_log_path()` to async (traits.rs, service.rs, commands.rs)
2. ✅ **BE**: Removed panic-prone Default impl for ConfigurationServiceImpl (service.rs)
3. ✅ **BE**: Fixed race condition in repository write operations - clone config before releasing lock (repository.rs)
4. ✅ **FE**: Fixed memory leak with useRef and cleanup effect for setTimeout (settings-form.tsx)

**HIGH (6/8 fixed)**:
5. ✅ **BE**: Centralized validation with `AppConfig::VALID_LOG_LEVELS` and `is_valid_log_level()` (models.rs, repository.rs)
6. ⏭️ **BE**: Skipped - Path validation would break existing user paths (needs migration strategy)
7. ⏭️ **BE**: Skipped - Frontend already uses updateConfig correctly for full config updates
8. ⏭️ **BE**: Skipped - Lost update prevention needs larger architectural change
9. ⏭️ **FE**: Skipped - Event listener needs event registry update (separate task)
10. ✅ **FE**: Strengthened path validation (AND logic instead of OR) (settings-form.tsx)
11. ✅ **FE**: Improved error handling with specific error messages (settings-form.tsx)
12. ✅ **FE**: Added pre-save validation before backend call (settings-form.tsx)

**MEDIUM (2/7 fixed)**:
13. ⏭️ **BE**: Debouncing - Skipped (would need larger architectural change)
14. ✅ **BE**: Fixed lazy load error recovery - set flag on failure (repository.rs)
15. ⏭️ **BE**: Event publishing on load failure - Skipped (minor edge case)
16. ⏭️ **BE**: Unused repository methods - Skipped (cleanup task)
17. ⏭️ **FE**: Double backend call - Skipped (would need backend change)
18. ✅ **FE**: Added ConfigurationChangedEvent type (app-config.ts)
19. ⏭️ **FE**: Error handling pattern - Skipped (would need larger refactor)

**Test Results**:
- Backend: 59 configuration tests passing
- Frontend: 517 tests passing (19 for settings-form)

---

### Domain 2: Wiki Scraping ✅
**Status**: COMPLETE

#### Files Mapped

**Backend** (`packages/backend/src/domain/wiki_scraping/`):
- `mod.rs` - Module exports
- `models.rs` - `WikiZoneData` struct
- `traits.rs` - `WikiScrapingService` trait
- `service.rs` - `WikiScrapingServiceImpl`
- `repository.rs` - `WikiRepository` for HTTP fetching
- `parser.rs` - `WikiParser` main entry point
- `url_utils.rs` - URL construction and helpers
- `parsers/` - 12 specialized parsers:
  - `base.rs` - Common parsing utilities
  - `infobox_parser.rs` - Zone infobox extraction
  - `act_parser.rs`, `area_id_parser.rs`, `area_level_parser.rs`
  - `is_town_parser.rs`, `has_waypoint_parser.rs`
  - `bosses_parser.rs`, `monsters_parser.rs`, `npcs_parser.rs`
  - `connected_zones_parser.rs`, `description_parser.rs`
  - `points_of_interest_parser.rs`, `image_url_parser.rs`
- 12+ test modules (`*_test.rs`)

**Frontend**: N/A (backend-only domain)

#### Domain Boundaries
- **External Dependency**: https://www.poe2wiki.net (HTML scraping)
- **Consumer**: Zone Configuration domain uses this data
- **No Tauri commands** (used internally by other services)

#### Issues Found

**CRITICAL (3)**:
1. **BE**: HTTP client creation can panic with `expect()` (repository.rs:13-17)
2. **BE**: Boss detection heuristic loses common bosses like "Hillock", "Merveil" (bosses_parser.rs)
3. **BE**: No validation that parsed data is meaningful - silent data loss (parser.rs)

**HIGH (4)**:
4. **BE**: No explicit redirect handling policy (repository.rs)
5. **BE**: Section parsing assumes specific HTML structure - fragile (base.rs)
6. **BE**: Connected zones parser doesn't handle wiki redirects (connected_zones_parser.rs)
7. **BE**: Service layer doesn't propagate repository creation errors (service.rs)

**MEDIUM (3)**:
8. **BE**: URL encoding incomplete for special characters (url_utils.rs)
9. **BE**: Case-sensitive redirect detection (infobox_parser.rs)
10. **BE**: Missing timeout configuration flexibility (repository.rs)

#### Fixes Implemented

**CRITICAL (3/3 fixed)**:
1. ✅ **BE**: Changed `WikiRepository::new()` to return `AppResult<Self>` instead of panicking (repository.rs)
2. ✅ **BE**: Improved boss detection to catch single-name bosses like "Hillock", "Merveil" with length heuristic (bosses_parser.rs)
3. ✅ **BE**: Added validation for meaningful parsed data with warning log (parser.rs)

**HIGH (3/4 fixed)**:
4. ✅ **BE**: Added explicit redirect policy `Policy::limited(5)` (repository.rs)
5. ⏭️ **BE**: Section parsing - Skipped (would need major parser rewrite)
6. ⏭️ **BE**: Connected zones redirects - Skipped (edge case, needs wiki testing)
7. ✅ **BE**: Updated service and service_registry to propagate repository errors (service.rs, service_registry.rs)

**MEDIUM (0/3 fixed)**:
8. ⏭️ **BE**: URL encoding - Skipped (current encoding works for known zones)
9. ⏭️ **BE**: Case-sensitive redirect - Skipped (minor edge case)
10. ⏭️ **BE**: Timeout configuration - Skipped (hardcoded 30s is reasonable)

**Test Results**:
- Backend: 423 tests passing (77 wiki_scraping tests)
- All cargo checks pass

---

### Domain 3: Server/Game Monitoring ✅
**Status**: COMPLETE

#### Files Mapped

**Server Monitoring** (`packages/backend/src/domain/server_monitoring/`):
- `mod.rs` - Module exports
- `models.rs` - `ServerStatus` struct with validation
- `traits.rs` - `ServerMonitoringService`, `PingProvider`, `ServerStatusRepository` traits
- `service.rs` - Ping monitoring loop with 30s interval
- `repository.rs` - File-based JSON persistence
- `ping_provider.rs` - System ping command wrapper (cross-platform)

**Game Monitoring** (`packages/backend/src/domain/game_monitoring/`):
- `mod.rs` - Module exports
- `models.rs` - `GameProcessStatus`, `GameMonitoringConfig`
- `traits.rs` - `GameMonitoringService`, `ProcessDetector` traits
- `service.rs` - Adaptive polling loop (3s detection, 60s monitoring)
- `process_detector.rs` - sysinfo-based process detection
- `commands.rs` - Tauri IPC handlers

**Frontend**: N/A (backend-only domains, events published to frontend)

#### Domain Boundaries
- **Server Monitoring**: Uses system `ping` command, file persistence, EventBus
- **Game Monitoring**: Uses `sysinfo` crate, CharacterService for finalization, EventBus

#### Issues Found

**CRITICAL (2)**:
1. **BE**: Task leak - spawned monitoring task never awaited on stop (service.rs:154)
2. **BE**: Windows ping timeout is 5ms instead of 5s (ping_provider.rs:15)

**HIGH (4)**:
3. **BE**: Race condition - multiple start calls can create duplicate tasks (service.rs:142)
4. **BE**: Interval reset causes immediate tick, duplicate checks (game_monitoring/service.rs:165)
5. **BE**: Process name substring matching too broad, false positives (process_detector.rs:37)
6. **BE**: Character service errors not propagated on game stop (game_monitoring/service.rs:62)

**MEDIUM (2)**:
7. **BE**: Invalid IP address silently accepted (server_monitoring/service.rs:99)
8. **BE**: SystemTime not frontend-friendly (game_monitoring/models.rs:24)

#### Fixes Implemented

**CRITICAL (2/2 fixed)**:
1. ✅ **BE**: Added `monitoring_task` field to track spawned task, await on stop (service.rs)
2. ✅ **BE**: Fixed Windows ping timeout - use 5000ms instead of 5 (ping_provider.rs)

**HIGH (4/4 fixed)**:
3. ✅ **BE**: Hold lock until task is spawned to prevent race conditions (service.rs)
4. ✅ **BE**: Consume immediate tick when interval changes (game_monitoring/service.rs)
5. ✅ **BE**: Use exact process name matching instead of substring (process_detector.rs)
6. ⏭️ **BE**: Error propagation - Skipped (would break async monitoring loop pattern)

**MEDIUM (0/2 fixed)**:
7. ⏭️ **BE**: IP validation - Skipped (validation exists in `is_valid()`, caller responsibility)
8. ⏭️ **BE**: SystemTime serialization - Skipped (not exposed to frontend directly)

**Test Results**:
- Backend: 423 tests passing (55 monitoring tests)
- All cargo checks pass

---

## Phase 2: Core User Features

### Domain 4: Character Management ⏳
**Status**: Pending
**Files**: TBD
**Issues Found**: TBD
**Fixes Implemented**: TBD

---

### Domain 5: Zone Tracking ⏳
**Status**: Pending
**Files**: TBD
**Issues Found**: TBD
**Fixes Implemented**: TBD

---

### Domain 6: Economy System ⏳
**Status**: Pending
**Files**: TBD
**Issues Found**: TBD
**Fixes Implemented**: TBD

---

## Phase 3: Supporting Features

### Domain 7: Walkthrough/Guides ⏳
**Status**: Pending
**Files**: TBD
**Issues Found**: TBD
**Fixes Implemented**: TBD

---

### Domain 8: UI Foundation ⏳
**Status**: Pending
**Files**: TBD
**Issues Found**: TBD
**Fixes Implemented**: TBD

---

## Issues Summary

### By Severity
- **Critical Issues Found**: 9 (Fixed: 9)
- **High Priority Issues Found**: 16 (Fixed: 11)
- **Medium Priority Issues Found**: 12 (Fixed: 2)

### By Category
- **Bugs**: 9 fixed (race conditions, memory leaks, panic risks, HTTP client panic, boss detection, task leak, Windows ping, duplicate tasks, process matching)
- **Data Integrity**: 2 fixed (centralized validation, parse validation)
- **Security**: 0 fixed (path validation deferred for migration)
- **Contract Violations**: 1 fixed (async trait alignment)
- **Logic Errors**: 2 fixed (strengthened path validation, interval immediate tick)
- **State Management**: 1 fixed (lazy load recovery)
- **Error Handling**: 4 fixed (specific error messages, pre-validation, error propagation, redirect policy)
- **Performance**: 1 fixed (interval immediate tick prevention)
- **Code Quality**: 1 fixed (ConfigurationChangedEvent type)

---

## Patterns & Learnings

*(Ralph will document patterns discovered during refactoring)*

---

## Test Results

### Frontend
- **Before**: 517 tests passing
- **After**: TBD

### Backend
- **Before**: TBD
- **After**: TBD

---

## Final Summary
*To be completed at end of session*

**Stats**:
- Total domains completed: 3 / 8
- Total files analyzed: ~35
- Total issues found: 37
- Total fixes implemented: 22
- Final test pass rate: 423 backend tests passing, 517 frontend tests passing
