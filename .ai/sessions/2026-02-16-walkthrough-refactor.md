# Walkthrough Guide Refactoring Session

**Date:** 2026-02-16
**Status:** Complete
**Related ADR:** ADR-005

## Overview

Completed comprehensive refactoring of the walkthrough guide system from HashMap-based structure with linked-list navigation to array-based structure with computed navigation.

## Problem Statement

The original walkthrough system had three critical issues:

1. **Fragile Navigation**: Manual `next_step_id`/`previous_step_id` wiring was error-prone (found bug: `act_4_step_18` key had `act_4_step_22` ID)
2. **Field Redundancy**: `notes` vs `details` on objectives created unnecessary complexity
3. **Hardcoded URLs**: `wiki_items` strings required URL construction logic in `wiki-utils.ts`

These issues would have made building a future custom guide creator UI very difficult.

## Solution

### Data Structure Changes

**Before:**
```rust
WalkthroughGuide {
  acts: HashMap<String, WalkthroughAct {
    act_name: String,
    act_number: u32,
    steps: HashMap<String, WalkthroughStep {
      id: String,
      next_step_id: Option<String>,
      previous_step_id: Option<String>,
      wiki_items: Vec<String>,
      objectives: Vec<Objective {
        notes: Option<String>,
        ...
      }>,
      ...
    }>,
  }>,
}
```

**After:**
```rust
WalkthroughGuide {
  acts: Vec<WalkthroughAct {
    act_name: String,
    steps: Vec<WalkthroughStep {
      id: String,
      links: Vec<StepLink {
        text: String,
        url: String,
      }>,
      objectives: Vec<Objective {
        details: Option<String>, // merged notes
        ...
      }>,
      ...
    }>,
  }>,
}
```

### Navigation System

Added helper methods to `WalkthroughGuide` that compute navigation from array positions:

```rust
impl WalkthroughGuide {
    fn find_step(&self, step_id: &str) -> Option<(usize, usize)>
    fn next_step_id(&self, step_id: &str) -> Option<String>
    fn previous_step_id(&self, step_id: &str) -> Option<String>
    fn step_exists(&self, step_id: &str) -> bool
    fn first_step_id(&self) -> Option<&str>
}
```

Frontend equivalents in `utils/walkthrough.ts`:
```typescript
export function getNextStepId(guide: WalkthroughGuide, stepId: string): string | null
export function getPreviousStepId(guide: WalkthroughGuide, stepId: string): string | null
```

## Implementation

### Phase 0: JSON Migration
- Created `migrate-walkthrough.js` script
- Transformed 87 steps across 4 acts from HashMap to Vec
- Merged 9 non-empty notes into details
- Converted 330 `wiki_items` to structured `links`
- Fixed `act_4_step_18` ID bug
- Validated: step count, unique IDs, no orphaned fields

### Phase 1-3: Backend (Rust)
- Updated models: Added `StepLink`, removed obsolete fields
- Updated service: Use navigation helpers instead of reading step fields
- Updated tests: 48 tests all passing with new structure

### Phase 4-7: Frontend (TypeScript/React)
- Updated types to match Rust models
- Added navigation utility functions
- Updated text parser: `wikiItems`/`onWikiClick` → `links`/`onLinkClick`
- Deleted `wiki-utils.ts` (URL construction now in data)
- Updated context: Use `getPreviousStepId()` helper
- Updated 9 components:
  - `walkthrough-step-card.tsx` - Compute next step via helper
  - `step-objective-list.tsx` - Removed notes rendering
  - `walkthrough-act-accordion.tsx` - Array iteration, no sorting
  - `walkthrough-guide.tsx` - Direct link handling
  - `campaign-insights.tsx` - Array length calculations
  - Routes: `index.tsx`, `walkthrough.tsx`

### Phase 8: Frontend Tests
- Updated mock data factories
- Updated 6 test spec files
- Renamed all props: `wikiItems`→`links`, `onWikiClick`→`onLinkClick`
- Removed notes-related assertions
- All tests passing

## Results

✅ **Backend**: 48 tests pass
✅ **Frontend**: TypeScript compiles clean (0 errors)
✅ **Total Files**: 20 modified, 1 deleted, 1 script created

### Performance Impact

Navigation helpers use O(n) linear search instead of O(1) HashMap lookup:
- **Acceptable**: Only ~87 steps total
- **Not hot path**: Lookups only on manual user navigation
- **Cache-friendly**: Acts/steps are sequential in memory

### Breaking Changes

None for end users:
- Step IDs unchanged (saved progress still works)
- Navigation behavior identical
- UI rendering identical

## Documentation Updates

- Added ADR-005 to `.ai/memory/decisions.md`
- Created this session note
- Updated CLAUDE.md references (if any)

## Cleanup

Migration artifacts:
- `migrate-walkthrough.js` - Can be deleted after verification
- `walkthrough_guide.json.backup` - Original backup, can be deleted after verification

## Key Learnings

1. **Array-based navigation** eliminates entire class of bugs (broken references impossible)
2. **Structured links** (`StepLink`) vs strings makes intent explicit and removes hardcoded URL logic
3. **Linear search** (O(n)) is perfectly acceptable for small datasets with infrequent access
4. **Merged fields** (`notes` → `details`) reduces cognitive load for guide creators
5. **Pre-ordered data** eliminates need for frontend sorting logic

## Future Work

This refactoring paves the way for:
- Custom walkthrough guide creator UI (simpler to build now)
- Community-contributed guides (easier data format)
- Multi-language support (explicit URLs can point to localized wikis)
- Guide versioning (arrays are easier to diff)
