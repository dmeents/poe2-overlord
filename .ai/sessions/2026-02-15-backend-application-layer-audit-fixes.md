# Backend Application Layer Audit & Fix Session

**Date**: 2026-02-15
**Task**: Implement fixes from backend application layer audit
**Status**: ✅ COMPLETE

## Summary

Successfully implemented all critical, high-priority, and medium-priority fixes from the backend application layer audit. Fixed two critical data-integrity bugs where services were instantiated twice, enabled logging in release builds, fixed shutdown panic risks, and improved error handling.

## Changes Implemented

### Phase 1: Critical Service Architecture Bugs ✅

#### 1.1 Injected shared EventBus into ConfigurationService
- **File**: `packages/backend/src/domain/configuration/service.rs`
- **Change**: Modified `ConfigurationServiceImpl::new()` to accept `Arc<EventBus>` parameter instead of creating its own
- **Impact**: Configuration change events now properly reach the frontend through the shared event bus

#### 1.2 Eliminated duplicate service instances
- **File**: `packages/backend/src/application/service_registry.rs`
- **Changes**:
  - Moved `EventBus` creation before `ConfigurationService` initialization
  - Replaced `.expect()` with `?` for proper error propagation
  - Created single `Arc<dyn CharacterService>` instance (removed duplicate Box instance)
  - Created single `Arc<dyn WalkthroughService>` instance (removed duplicate Box instance)
- **Impact**: Background services and frontend commands now share the same service instances, fixing data synchronization bugs

#### 1.3 Updated command signatures
- **Files**:
  - `packages/backend/src/domain/character/commands.rs` (8 commands)
  - `packages/backend/src/domain/walkthrough/commands.rs` (3 commands)
- **Change**: Changed all command parameters from `State<'_, Box<dyn Service>>` to `State<'_, Arc<dyn Service>>`
- **Impact**: Commands now use the shared service instances

### Phase 2: High Priority Runtime Issues ✅

#### 2.1 Enabled logging in release builds
- **File**: `packages/backend/src/application/app_setup.rs`
- **Change**: Removed `if cfg!(debug_assertions)` wrapper from log plugin initialization
- **Impact**: Release builds now have proper logging for debugging production issues

#### 2.2 Fixed block_on panic risk
- **File**: `packages/backend/src/application/app_setup.rs`
- **Change**: Replaced `tauri::async_runtime::block_on()` with `tokio::task::block_in_place` + `Handle::current().block_on()`
- **Impact**: Shutdown handler no longer risks panic when already inside Tokio runtime

#### 2.3 Consolidated double window lookup
- **File**: `packages/backend/src/application/app_setup.rs`
- **Change**: Merged two separate `app.get_webview_window("main")` calls into single block
- **Impact**: Cleaner code, shutdown handler registered alongside other window setup

### Phase 3: Medium Priority Improvements ✅

#### 3.1 Comprehensive shutdown
- **File**: `packages/backend/src/application/service_registry.rs`
- **Changes**:
  - Stop game monitoring service
  - Stop log monitoring service
  - Stop server monitoring service
  - Finalize character tracking data
  - Flush configuration to disk
  - Emit `SystemShutdown` event
- **Impact**: Clean, orderly application shutdown with all resources properly released

#### 3.2 Removed double-wrapped errors
- **File**: `packages/backend/src/domain/configuration/commands.rs`
- **Change**: Simplified all 12 configuration commands to use direct `to_command_result()` calls
- **Impact**: Cleaner error handling consistent with other domains

#### 3.3 Stored background task JoinHandles
- **File**: `packages/backend/src/application/service_orchestrator.rs`
- **Change**: Modified all three orchestrator functions to return `JoinHandle<()>`
- **File**: `packages/backend/src/application/app_setup.rs`
- **Change**: Captured returned handles in variables
- **Impact**: Task handles available for future use (graceful cancellation, status checks)

## Verification

### Type Safety
```bash
pnpm check:backend
```
✅ Passed - No compilation errors

### Tests
```bash
pnpm test:backend
```
✅ All 512 tests passed

## Files Changed

| Phase | File | LOC Changed |
|-------|------|-------------|
| 1.1   | `src/domain/configuration/service.rs` | ~3 |
| 1.2   | `src/application/service_registry.rs` | ~40 |
| 1.3   | `src/domain/character/commands.rs` | ~10 |
| 1.3   | `src/domain/walkthrough/commands.rs` | ~4 |
| 2.1   | `src/application/app_setup.rs` | ~4 |
| 2.2   | `src/application/app_setup.rs` | ~4 |
| 2.3   | `src/application/app_setup.rs` | ~10 |
| 3.1   | `src/application/service_registry.rs` | ~30 |
| 3.2   | `src/domain/configuration/commands.rs` | ~100 |
| 3.3   | `src/application/service_orchestrator.rs` | ~8 |

**Total**: 8 files, ~213 lines changed

## Impact Assessment

### Critical Bugs Fixed
1. **ConfigurationService events not reaching frontend** - Events from config changes were being published to an isolated EventBus instance that nothing subscribed to
2. **Duplicate service instances with divergent state** - CharacterService and WalkthroughService had separate instances for background tasks vs frontend commands, causing data synchronization issues

### Runtime Safety Improvements
1. **Release builds now have logging** - Production issues can now be diagnosed
2. **Shutdown no longer risks panic** - Proper tokio runtime handling prevents panic on close
3. **Comprehensive cleanup on shutdown** - All services properly stopped and data flushed

### Code Quality Improvements
1. **Simplified error handling** - Configuration commands now consistent with rest of codebase
2. **Task handles available** - Future graceful cancellation and monitoring possible
3. **Better code organization** - Consolidated window setup logic

## Notes

- All changes are backward compatible
- No breaking changes to public APIs
- No database schema changes
- No frontend changes required

## Deferred Items (Low Priority)

The following issues were identified but deferred as low priority:
- Inconsistent Send+Sync bounds in ServiceInstances (cosmetic)
- Hardcoded walkthrough path (works correctly, just fragile)
- Mixed AppResult vs Result<T, AppError> (functionally equivalent)
- `[DEBUG]` prefix logs at info level (easy cleanup anytime)
- PingProvider returns Result<u64, String> (requires trait changes)
- LogAnalysisEvent dead code (safe to remove later)
- Economy domain missing trait (works, just not abstract)
- Empty price_checker directory (no impact)
