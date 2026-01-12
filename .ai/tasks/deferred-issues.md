# Deferred Issues from Domain Refactoring

**Total**: 61 issues deferred for larger architectural changes
**Completed**: 6 (Quick Wins Batch)
**In Progress**: 0
**Remaining**: 55

**Source**: `.ai/sessions/2026-01-11-domain-refactoring.md`

---

## Status Legend
- 🚧 = In Progress (being worked on in current PRD)
- ✅ = Complete (fixed and committed)
- ⏳ = Pending (not yet started)

---

## Priority 1: CRITICAL Issues (7 deferred)

### Issue #1: Path Validation Requires Migration Strategy
**Domain**: Configuration Management
**Severity**: CRITICAL
**Type**: Security + Data Integrity
**Description**: Missing file path validation (absolute path, path traversal attacks)
**Location**: `backend/domain/configuration/repository.rs`
**Complexity**: HIGH - Requires user data migration
**Impact**: Security vulnerability, could allow path traversal
**Dependencies**: None

### Issue #2: Lost Update Prevention Architecture
**Domain**: Configuration Management
**Severity**: CRITICAL (was HIGH)
**Type**: Data Integrity
**Description**: Concurrent config updates can lose data (last-write-wins)
**Location**: `backend/domain/configuration/repository.rs`
**Complexity**: HIGH - Requires optimistic locking or versioning
**Impact**: Data loss in concurrent updates
**Dependencies**: None

### Issue #3: Transaction Safety in Character Creation
**Domain**: Character Management
**Severity**: CRITICAL
**Type**: Data Integrity
**Description**: Orphaned files on index write failure
**Location**: `backend/domain/character/service.rs:55-105`
**Complexity**: MEDIUM - Needs rollback logic
**Impact**: Data corruption, orphaned character files
**Dependencies**: None

### Issue #4: Cache Race Condition
**Domain**: Economy System
**Severity**: CRITICAL
**Type**: Data Integrity
**Description**: Concurrent cache updates can cause data loss
**Location**: `backend/domain/economy/service.rs:98-115`
**Complexity**: HIGH - Needs cache locking architecture
**Impact**: Stale or corrupted economy data
**Dependencies**: None

### Issue #5: Real-Time Zone Timer
**Domain**: Zone Tracking
**Severity**: CRITICAL (UX)
**Type**: Functional Gap
**Description**: Active zone timer not displayed in real-time
**Location**: `frontend/components/zones/current-zone-card.tsx:67`
**Complexity**: MEDIUM - Needs useEffect intervals
**Impact**: Poor UX, users can't see live time tracking
**Dependencies**: None

### Issue #6: Provider Dependency Documentation ✅
**Domain**: UI Foundation
**Severity**: CRITICAL (Architecture)
**Type**: Documentation + Architecture
**Description**: ZoneProvider depends on CharacterProvider (order matters)
**Location**: `frontend/providers.tsx:19`
**Complexity**: LOW - Documentation + optional runtime check
**Impact**: App crashes if provider order wrong
**Dependencies**: None
**Status**: ✅ Complete (Commit: 44d5f32)

### Issue #7: QueryClient Provider Access ✅
**Domain**: UI Foundation
**Severity**: CRITICAL (Architecture)
**Type**: Architecture
**Description**: No documented pattern for accessing QueryClient
**Location**: `frontend/providers.tsx:14-28`
**Complexity**: LOW - Documentation
**Impact**: Inconsistent patterns, hard to maintain
**Dependencies**: None
**Status**: ✅ Complete (Commit: ed28bde)

---

## Priority 2: HIGH Priority Issues (21 deferred)

### Issue #8: Configuration Event Listener
**Domain**: Configuration Management
**Severity**: HIGH
**Type**: Functional Gap
**Description**: Frontend doesn't listen to ConfigurationChanged events - stale data
**Location**: `frontend/components/forms/settings-form/settings-form.tsx`
**Complexity**: MEDIUM - Needs event registry integration
**Impact**: UI shows stale config after external changes
**Dependencies**: Event registry system

### Issue #9: Wiki Section Parsing Brittleness
**Domain**: Wiki Scraping
**Severity**: HIGH
**Type**: Reliability
**Description**: Section parsing assumes specific HTML structure - fragile
**Location**: `backend/domain/wiki_scraping/parsers/base.rs`
**Complexity**: HIGH - Major parser rewrite
**Impact**: Breaks when wiki HTML changes
**Dependencies**: None

### Issue #10: Connected Zones Wiki Redirects
**Domain**: Wiki Scraping
**Severity**: HIGH
**Type**: Edge Case
**Description**: Connected zones parser doesn't handle wiki redirects
**Location**: `backend/domain/wiki_scraping/parsers/connected_zones_parser.rs`
**Complexity**: MEDIUM - Needs wiki testing infrastructure
**Impact**: Missing connected zone data for redirected pages
**Dependencies**: None

### Issue #11: Character Error Propagation
**Domain**: Server/Game Monitoring
**Severity**: HIGH
**Type**: Error Handling
**Description**: Character service errors not propagated on game stop
**Location**: `backend/domain/game_monitoring/service.rs:62`
**Complexity**: MEDIUM - Would break async loop pattern
**Impact**: Silent failures in character finalization
**Dependencies**: None

### Issue #12: Orphaned Character Cleanup
**Domain**: Character Management
**Severity**: HIGH
**Type**: Data Integrity
**Description**: Missing characters logged but index not cleaned
**Location**: `backend/domain/character/repository.rs:80-95`
**Complexity**: MEDIUM - Needs careful implementation
**Impact**: Index contains references to deleted files
**Dependencies**: None

### Issue #13: Inefficient Name Uniqueness Check
**Domain**: Character Management
**Severity**: HIGH
**Type**: Performance
**Description**: is_name_unique loads all characters
**Location**: `backend/domain/character/service.rs:244-255`
**Complexity**: MEDIUM - Needs index refactor with name lookup
**Impact**: Slow character creation with many characters
**Dependencies**: Index refactor

### Issue #14: Character Deletion Events
**Domain**: Character Management
**Severity**: HIGH
**Type**: Functional Gap
**Description**: No character deletion events published
**Location**: `backend/domain/character/service.rs:172-184`
**Complexity**: MEDIUM - Needs new AppEvent variant
**Impact**: Frontend doesn't auto-refresh on deletion
**Dependencies**: Event system

### Issue #15: Frontend Deletion Event Handling ✅
**Domain**: Character Management
**Severity**: HIGH
**Type**: Functional Gap
**Description**: Context doesn't handle character deletion events
**Location**: `frontend/contexts/CharacterContext.tsx:51-70`
**Complexity**: LOW - Depends on Issue #14
**Impact**: UI doesn't update on character deletion
**Dependencies**: Issue #14 (backend event)
**Status**: ✅ Complete (Commit: 90d3f44) - Frontend ready, awaits backend Issue #14

### Issue #16: Zone Leave Not Called on Change
**Domain**: Zone Tracking
**Severity**: HIGH
**Type**: Data Integrity
**Description**: leave_zone not called automatically on zone change
**Location**: `backend/domain/zone_tracking/service.rs:34-37`
**Complexity**: MEDIUM - Needs refactor
**Impact**: Active zone not deactivated, timing inaccurate
**Dependencies**: None

### Issue #17: Zone Metadata Overwrite
**Domain**: Zone Tracking
**Severity**: HIGH
**Type**: Design Decision
**Description**: Zone metadata overwritten on re-entry
**Location**: `backend/domain/zone_tracking/service.rs:46-47`
**Complexity**: MEDIUM - Intentional design, needs review
**Impact**: Loses manual corrections to zone metadata
**Dependencies**: None

### Issue #18: Zone Type Mismatch
**Domain**: Zone Tracking
**Severity**: HIGH
**Type**: Contract Violation
**Description**: Frontend ZoneStats has extra fields not in backend
**Location**: `frontend/types/character.ts:91-114`
**Complexity**: MEDIUM - Types are EnrichedZoneStats, needs clarity
**Impact**: Confusion about type contract
**Dependencies**: None

### Issue #19: Placeholder Zone Missing Metadata
**Domain**: Zone Tracking
**Severity**: HIGH
**Type**: Functional Gap
**Description**: Placeholder zone has empty arrays even if metadata exists
**Location**: `frontend/utils/zone-utils.ts:45-72`
**Complexity**: MEDIUM - Needs zone config integration
**Impact**: Incomplete zone display in UI
**Dependencies**: Zone configuration integration

### Issue #20: Economy Retry Logic
**Domain**: Economy System
**Severity**: HIGH
**Type**: Reliability
**Description**: No retry logic for network failures
**Location**: `backend/domain/economy/service.rs:141-183`
**Complexity**: MEDIUM - Graceful degradation exists (stale cache)
**Impact**: Poor resilience to transient network issues
**Dependencies**: None

### Issue #21: Tertiary Currency Selection ✅
**Domain**: Economy System
**Severity**: HIGH
**Type**: Logic Bug
**Description**: Incorrect tertiary currency selection - unpredictable
**Location**: `backend/domain/economy/models.rs:100-106`
**Complexity**: MEDIUM - Current logic works for 3-currency systems
**Impact**: Wrong currency shown in 4+ currency leagues
**Dependencies**: None
**Status**: ✅ Complete (Commit: 73474c5) - Deterministic rate-based selection

### Issue #22: Walkthrough Race Condition
**Domain**: Walkthrough/Guides
**Severity**: HIGH
**Type**: Data Integrity
**Description**: Concurrent scene changes could skip steps
**Location**: `backend/domain/walkthrough/service.rs:42-80`
**Complexity**: HIGH - Needs per-character mutex architecture
**Impact**: Step progression corrupted in edge cases
**Dependencies**: None

### Issue #23: Layout Height Magic Numbers
**Domain**: UI Foundation
**Severity**: HIGH
**Type**: Maintainability
**Description**: Layout height calculation uses magic numbers
**Location**: `frontend/routes/__root.tsx:10-17`
**Complexity**: MEDIUM - Needs CSS variable refactor
**Impact**: Hard to maintain, breaks on layout changes
**Dependencies**: None

### Issue #24: Modal Scroll Lock Memory Leak
**Domain**: UI Foundation
**Severity**: HIGH
**Type**: Memory Leak
**Description**: Body scroll lock memory leak with nested modals
**Location**: `frontend/components/ui/modal/modal.tsx:62-72`
**Complexity**: HIGH - Needs global modal manager
**Impact**: Memory leak, scroll issues with nested modals
**Dependencies**: Global modal manager architecture

### Issue #25: Tooltip Scroll Repositioning ✅
**Domain**: UI Foundation
**Severity**: HIGH
**Type**: UX Bug
**Description**: Tooltip repositioning added but may need refinement
**Location**: `frontend/components/ui/tooltip/tooltip.tsx`
**Complexity**: LOW - Verify fix works well
**Impact**: Tooltip positioned incorrectly after scroll
**Dependencies**: None
**Status**: ✅ Complete (Commit: f8f3b3a) - Verified, added tests and documentation

### Issue #26: Accordion Accessibility ✅
**Domain**: UI Foundation
**Severity**: HIGH
**Type**: Accessibility
**Description**: Accordion missing ARIA attributes
**Location**: `frontend/components/ui/accordion/accordion.tsx:26-43`
**Complexity**: LOW - Add ARIA attributes
**Impact**: Not accessible to screen readers
**Dependencies**: None
**Status**: ✅ Complete (Commit: ccdcea2) - Added full ARIA compliance

---

## Priority 3: MEDIUM Priority Issues (33 deferred)

*(All MEDIUM issues from the session log - architectural refactors, code cleanup, etc.)*

### Configuration Domain (5 issues)
- Issue #27: Disk write debouncing
- Issue #28: Event publishing on load failure
- Issue #29: Unused repository methods cleanup
- Issue #30: Double backend call on reset
- Issue #31: Error handling pattern consistency

### Wiki Scraping Domain (3 issues)
- Issue #32: URL encoding for special characters
- Issue #33: Case-sensitive redirect detection
- Issue #34: Timeout configuration flexibility

### Monitoring Domain (2 issues)
- Issue #35: IP address validation (already exists in is_valid())
- Issue #36: SystemTime serialization

### Character Domain (5 issues)
- Issue #37: Inconsistent error handling in repository
- Issue #38: Inefficient character enrichment (sequential zone metadata calls)
- Issue #39: Default CharacterData has empty ID
- Issue #40: Missing bounds check on level in update_character_level
- Issue #41: Hardcoded hideout detection logic

### Zone Tracking Domain (5 issues)
- Issue #42: ZoneContext conflates modal state with zone selection
- Issue #43: Act breakdown excludes Act 5
- Issue #44: getDisplayAct inconsistent formats
- Issue #45: No test coverage for service layer
- Issue #46: Hardcoded hideout string duplicated

### Economy Domain (5 issues)
- Issue #47: Manual EconomyType string parsing
- Issue #48: No TTL value overflow validation
- Issue #49: Empty currencies array not distinguished in UI
- Issue #50: Excessive query invalidation (performance)
- Issue #51: Missing error handling for image failures

### Walkthrough Domain (4 issues)
- Issue #52: Incomplete test coverage (service tests)
- Issue #53: Parameter naming inconsistency (characterId vs character_id)
- Issue #54: No bounds checking on step IDs (circular reference risk)
- Issue #55: Conditional hook calls violate Rules of Hooks

### UI Foundation Domain (4 issues)
- Issue #56: Button no loading state
- Issue #57: Sidebar missing active link announcement
- Issue #58: Error state unsafe type coercion
- Issue #59: Time display inconsistent rounding

---

## Issue Categories for Pipeline

### Category A: Data Integrity & Security (7 issues)
High priority, requires careful testing
- Issues: #1, #2, #3, #4, #12, #16, #22

### Category B: Real-Time Features (2 issues)
UI/UX improvements
- Issues: #5, #25

### Category C: Event System Integration (4 issues)
Dependent on event infrastructure
- Issues: #8, #14, #15, (others)

### Category D: Accessibility & Documentation (4 issues)
Important for production quality
- Issues: #6, #7, #26, #57

### Category E: Performance Optimizations (3 issues)
Can wait, measurable impact
- Issues: #13, #23, #50

### Category F: Code Quality & Refactors (41 issues)
Lower priority, technical debt
- Issues: #9-#11, #17-#21, #24, #27-#59

---

## Recommended Implementation Order

1. ~~**Quick Wins**: #6, #7, #26, #15, #25, #21~~ ✅ **COMPLETE** (Session: 2026-01-11)
2. **Data Integrity**: #1, #2, #3, #4, #12, #16, #22
3. **Event System**: #8, #14
4. **Real-Time Features**: #5
5. **Refactors**: Remaining MEDIUM issues
