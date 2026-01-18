# Data Integrity Batch - Session Log

**Started**: 2026-01-11
**PRD**: `.ai/tasks/prd-data-integrity.md`
**Status**: COMPLETE

## Test Baseline

- Frontend: 530 tests passing
- Backend: 425 tests passing

## Issues Progress

- [x] Issue #1: Path Validation with Migration Strategy (CRITICAL) ✅
- [x] Issue #2: Lost Update Prevention Architecture (CRITICAL) ✅
- [x] Issue #3: Transaction Safety in Character Creation (CRITICAL) ✅
- [x] Issue #4: Cache Race Condition (CRITICAL) ✅
- [x] Issue #12: Orphaned Character Cleanup (HIGH) ✅
- [x] Issue #16: Zone Leave Not Called on Change (HIGH) ✅
- [x] Issue #22: Walkthrough Race Condition (HIGH) ✅

## Architecture Decisions

### Issue #1: Path Validation Security Module

**Decision**: Created a new `infrastructure/security` module with `PathValidator` for path validation.

**Key Points**:
1. **Layered defense**: Path traversal check + canonicalization + allowed roots check + extension check
2. **Platform-specific allowed roots**: Home directory, common game directories, platform-specific paths
3. **Migration strategy**: On config load, invalid paths are reset to default
4. **Config versioning**: Added `config_version` field for future migration compatibility
5. **New error type**: Added `AppError::Security` variant for security violations

**Files Created**:
- `src/infrastructure/security/mod.rs`
- `src/infrastructure/security/path_validation.rs`

**Files Modified**:
- `src/errors.rs` - Added Security error variant
- `src/infrastructure/mod.rs` - Export security module
- `src/domain/configuration/models.rs` - Added config_version, updated validate()
- `src/domain/configuration/repository.rs` - Added migration logic
- `src/domain/configuration/models_test.rs` - Updated tests for new fields

### Issue #2: Optimistic Locking for Lost Update Prevention

**Decision**: Implemented optimistic locking using version numbers to prevent lost updates from concurrent modifications.

**Key Points**:
1. **Version field**: Added `version: u64` field to AppConfig (increments on every write)
2. **Version check on save**: FileService.write_json_with_version_check validates version before writing
3. **Atomic pattern**: Read current version → validate matches → write with incremented version
4. **Error type**: Added `AppError::ConcurrentModification` for conflict detection
5. **In-memory consistency**: Only update in-memory state after successful disk write

**Files Modified**:
- `src/errors.rs` - Added ConcurrentModification error variant
- `src/domain/configuration/models.rs` - Added version field and with_incremented_version()
- `src/domain/configuration/repository.rs` - Updated save methods with version checking
- `src/infrastructure/file_management/service.rs` - Added write_json_with_version_check()
- `src/domain/configuration/models_test.rs` - Added optimistic locking tests

## Edge Cases Tested

### Issue #1 Edge Cases:
- Path traversal sequences (`../../../etc/passwd`) - REJECTED
- Empty paths - REJECTED
- Whitespace-only paths - REJECTED
- Invalid file extensions (.exe, .sh, etc.) - REJECTED
- Tilde expansion (`~/path`) - EXPANDED and validated
- Sensitive system paths (`/etc/passwd.txt`) - REJECTED (outside allowed roots)
- Valid home directory paths - ACCEPTED

### Issue #2 Edge Cases:
- Version increment on every write - VERIFIED
- Version wrapping at u64::MAX - Wraps to 0 correctly
- Old configs without version field - Default to version 0
- Concurrent modification detection - Returns ConcurrentModification error
- File deleted between read and write - Detected as version mismatch

### Issue #3: Transaction Safety in Character Creation

**Decision**: Reversed operation order in `create_character` to prevent orphaned files on failure.

**Key Points**:
1. **Index-first pattern**: Write index entry FIRST (if fails, no orphan file created)
2. **File second**: Write character file SECOND (if fails, can rollback index)
3. **Rollback logic**: On file write failure, remove character from index and clear active if needed
4. **Clean failure states**: Either both operations succeed, or both fail (atomic semantics)

**Files Modified**:
- `src/domain/character/service.rs` - Reordered operations with rollback in create_character()

### Issue #3 Edge Cases:
- Index write fails first - No orphan file, clean error state
- File write fails - Index rolled back, clean error state
- Both succeed - Normal operation
- First character (sets active) - Rollback also clears active_character_id

### Issue #4: Cache Race Condition Fix

**Decision**: Implemented per-cache-key semaphore deduplication using `Arc<RwLock<HashMap<String, Arc<Semaphore>>>>`.

**Key Points**:
1. **Fast path optimization**: Fresh cache check without acquiring any locks
2. **Request deduplication**: Per-cache-key semaphores ensure only one API fetch at a time
3. **Double-check pattern**: Re-check cache after acquiring lock (coalesced requests)
4. **Cleanup**: Remove semaphores from map after fetch completes
5. **No blocking between keys**: Different league/type combinations don't block each other

**Files Modified**:
- `src/domain/economy/service.rs` - Added in_flight tracking, semaphore acquisition, cleanup
- `src/domain/economy/service_test.rs` - Added cache_key and concurrent tests

### Issue #4 Edge Cases:
- Concurrent requests for same cache key - Coalesced into single fetch
- Different cache keys - Run in parallel, no blocking
- Semaphore cleanup - Removed after each fetch to prevent memory growth
- Deadlock prevention - Using RAII (semaphore permit auto-released)

## Commits

- `fix(security): add path validation to prevent path traversal attacks (Issue #1)`
- `fix(data): implement optimistic locking to prevent lost updates (Issue #2)`
- `fix(data): add transaction safety with rollback to character creation (Issue #3)`
- `fix(data): add request deduplication to prevent cache race condition (Issue #4)`
- `fix(data): add orphaned character cleanup with automatic startup reconciliation (Issue #12)`
- `fix(data): add explicit zone leave before zone enter for proper duration tracking (Issue #16)`
- `fix(data): add in-memory caching to prevent walkthrough race condition (Issue #22)`

## Gotchas and Learnings

1. **Test count increased**: Backend tests went from 425 → 445 (+20 tests from Issues #1-2)
2. **Serde defaults**: Using `#[serde(default)]` allows graceful migration of old configs without version field
3. **Rollback complexity**: When rollback itself can fail, need to decide whether to propagate original error or rollback error - chose to propagate original
