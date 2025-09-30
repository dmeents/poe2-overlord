# Frontend Hooks Architecture Analysis for POE2 Overlord

## Context & Project Structure
You are analyzing a Path of Exile 2 game overlay application with a Tauri-based architecture. The project has a Rust backend exposing Tauri commands and events, with a React/TypeScript frontend consuming these via custom hooks. The project recently underwent major refactors with new services added.

**Key Architecture Components:**
- **Backend**: Rust services exposing Tauri commands and emitting events
- **Frontend**: React hooks wrapping TanStack Query + Tauri event listeners
- **Data Flow**: Commands for operations, Events for real-time updates
- **State Management**: TanStack Query for server state, React state for UI state

## Analysis Objectives

Perform a comprehensive analysis of the frontend hooks (`packages/frontend/src/hooks/`) with specific focus on:

### 1. HOOK REDUNDANCY ANALYSIS (Priority 1)
**Objective**: Identify duplicate logic, overlapping responsibilities, and consolidation opportunities.

**Specific Areas to Investigate:**
- **Character-related hooks** (`useCharacterQueries.ts`, `useCharacterManagement.ts`, `useCharacterFilters.ts`, `useCharacterFiltering.ts`, `useCharacterConfig.ts`):
  - Are there duplicate character data fetching patterns?
  - Is character filtering logic duplicated across multiple hooks?
  - Can character management operations be consolidated?
  
- **Event listener hooks** (`useGameProcessEvents.ts`, `useServerStatusEvents.ts`, `useWalkthroughEvents.ts`):
  - Do they share similar event handling patterns that could be abstracted?
  - Is there duplicate event listener setup/cleanup logic?
  - Can event payload processing be standardized?

- **Filter/State management patterns**:
  - Compare `useCharacterFilters.ts` vs `useZoneFilters.ts` - are they using similar patterns?
  - Are there repeated useState/useCallback patterns that could be abstracted?

### 2. BACKEND COMMAND MAPPING (Priority 2) 
**Objective**: Cross-reference frontend hook usage with backend commands to identify unused or consolidatable code.

**Backend Commands Available:**
```rust
// Character Management Commands
create_character, get_character, get_all_characters, update_character, 
delete_character, set_active_character, get_active_character, 
get_characters_index, is_character_name_unique, 
get_available_character_classes, get_available_leagues, 
get_available_ascendencies_for_class

// Character Tracking Commands  
get_character_tracking_data, get_character_current_location, enter_zone

// Game Monitoring Commands
get_game_process_status

// Server Monitoring Commands
get_server_status, ping_server, start_server_monitoring, stop_server_monitoring

// Walkthrough Commands (from walkthrough domain)
// Configuration Commands (from configuration domain)
```

**Backend Events Available:**
```rust
// Character events
"character-tracking-data-updated"

// Game monitoring events  
"game-process-status-changed"

// Server monitoring events
"server-status-changed"  

// Walkthrough events
"walkthrough-progress-updated", "walkthrough-step-completed", 
"walkthrough-step-advanced", "walkthrough-campaign-completed"
```

**Analysis Tasks:**
- **Unused Commands**: Which backend commands are NOT consumed by any frontend hook?
- **Unused Events**: Which backend events are NOT listened to by any frontend hook?
- **Over-fetching**: Are hooks calling commands that return more data than needed?
- **Command Consolidation**: Can multiple similar commands be replaced with fewer, more flexible ones?

### 3. HOOK ARCHITECTURE IMPROVEMENTS (Priority 3)
**Objective**: Identify opportunities to improve performance, maintainability, and developer experience.

**Performance Analysis:**
- **React Query Usage**: Are query keys consistent? Are stale times appropriate?
- **Event Listeners**: Are there memory leaks from improperly cleaned up listeners?
- **Unnecessary Re-renders**: Are hooks causing excessive re-renders due to dependency arrays?
- **State Duplication**: Is the same data stored in multiple places (React Query cache + local state)?

**Architecture Patterns:**
- **Hook Composition**: Can complex hooks be broken down into smaller, more focused hooks?
- **Custom Hook Factories**: Can similar hooks be generated from a factory pattern?
- **Error Handling**: Is error handling consistent across all hooks?
- **Loading States**: Are loading states managed consistently?

### 4. SHARED HOOK OPPORTUNITIES (Priority 4)
**Objective**: Identify reusable patterns that can eliminate bespoke implementations.

**Common Patterns to Extract:**
- **Generic CRUD Hooks**: Can character CRUD operations be generalized?
- **Generic Event Listener Hook**: Can event listening patterns be abstracted?
- **Generic Filter Hook**: Can filtering logic be made reusable?
- **Generic Status Hook**: Can status monitoring be made generic?

**Specific Investigation Points:**
- Are there similar data transformation patterns across different hooks?
- Can query key generation be standardized?
- Are there repeated validation or error handling patterns?
- Can event payload processing be made generic?

## Analysis Methodology

### Step 1: Hook Inventory
1. **Catalog all hooks** in `packages/frontend/src/hooks/`
2. **Map their purposes**: CRUD, Events, Filters, Configuration, etc.
3. **Identify dependencies**: Which hooks depend on others?
4. **Document exported functions**: What does each hook expose?

### Step 2: Usage Analysis  
1. **Find hook consumers**: Which components use each hook?
2. **Identify usage patterns**: How are hooks typically consumed?
3. **Spot unused exports**: Are all hook exports actually used?
4. **Map component overlap**: Do multiple components need the same data?

### Step 3: Backend Mapping
1. **Command utilization**: Map each Tauri command to consuming hooks
2. **Event utilization**: Map each Tauri event to listening hooks  
3. **Identify gaps**: Commands/events not consumed by frontend
4. **Find duplicates**: Multiple hooks calling the same commands

### Step 4: Pattern Recognition
1. **Similar implementations**: Look for near-identical logic across hooks
2. **Common abstractions**: Identify patterns that repeat
3. **Performance bottlenecks**: Find inefficient patterns
4. **Architecture violations**: Spot hooks doing too much

## Expected Findings Format

Present findings in this structure:

### REDUNDANCY FINDINGS
For each redundant pattern found:
```
#### Duplicate Pattern: [Pattern Name]
- **Files**: `path/to/hook1.ts`, `path/to/hook2.ts` 
- **Redundancy Type**: [Exact Duplication | Similar Logic | Overlapping Responsibilities]
- **Lines of Duplicate Code**: ~X lines
- **Consolidation Opportunity**: [Specific approach to combine them]
- **Risk Level**: [Low | Medium | High]
- **Code Examples**: Show the duplicated patterns
```

### UNUSED CODE FINDINGS  
For each unused command/event/function:
```
#### Unused: [Command/Event/Function Name]
- **Location**: Backend command in `path/to/file.rs` OR Hook export in `path/to/file.ts`
- **Reason**: [Not consumed | Dead code | Replaced by alternative]
- **Removal Safety**: [Safe to remove | Needs investigation | Keep for future]
- **Cleanup Impact**: [LOC reduction potential]
```

### ARCHITECTURE IMPROVEMENTS
For each improvement opportunity:
```  
#### Improvement: [Improvement Name]
- **Current State**: Description of current implementation
- **Problem**: Performance issue, maintainability issue, etc.
- **Proposed Solution**: Specific architectural change
- **Benefits**: Performance gain, code reduction, maintainability  
- **Implementation Complexity**: [Low | Medium | High]
- **Breaking Changes**: [None | Minor | Major]
```

### SHARED HOOK OPPORTUNITIES
For each consolidation opportunity:
```
#### Shared Hook: [Hook Name]
- **Current Bespoke Implementations**: List of similar hooks
- **Common Pattern**: Abstract pattern they share  
- **Proposed Generic Hook**: Interface and usage
- **Migration Path**: How to refactor existing code
- **Code Savings**: Estimated LOC reduction
```

## Implementation Guidelines

1. **NO CODE CHANGES**: Only analyze and provide recommendations
2. **Be Specific**: Reference exact line numbers and files
3. **Risk Assessment**: Evaluate breaking change potential for each suggestion  
4. **Prioritize Impact**: Focus on changes with highest code reduction / performance benefit
5. **Consider Migration**: Suggest incremental refactoring paths
6. **Preserve Functionality**: Ensure recommendations maintain existing behavior

## Success Metrics
Target outcomes from this analysis:
- **Identify 40-60% code reduction potential** in hooks directory
- **Find 80%+ of unused backend commands/events**  
- **Discover 3-5 major consolidation opportunities**
- **Propose 2-3 generic hook patterns** that can replace multiple existing hooks
- **Eliminate all memory leaks** from event listeners
- **Standardize error handling** across all hooks

Begin analysis now. Focus on identifying the most impactful opportunities first, then work through the detailed patterns.