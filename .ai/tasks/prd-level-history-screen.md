# PRD: Level History Screen

**Status:** Not Started
**Created:** 2026-03-29

## Problem

The app tracks per-level-up data (timestamp, deaths, active grinding time) in the `level_events` table, but the frontend only surfaces the last 5 events in a small card on the dashboard. There's no way to view the full leveling journey for a character or analyze trends across all levels.

## Goal

Add a `/leveling` route that shows a character's complete level-up history with:
1. A large chart plotting XP/hr, deaths, and time-per-level across all levels
2. A detailed table of every level-up event
3. Summary insights (fastest level, peak XP/hr, total deaths, etc.)

## Current State

### What already exists
- **`level_events` table**: stores `character_id`, `level`, `reached_at`, `deaths_at_level`, `active_seconds` per level-up (UNIQUE on character_id + level)
- **`LevelEventResponse` DTO**: computed fields — `time_from_previous_level_seconds`, `effective_xp_earned`, `xp_per_hour`
- **`LevelingStats`**: sent via event system, contains `recent_events` (last 5) and `chart_events` (all, but stripped to level/xp_per_hour/deaths only)
- **`LevelingStatsCard`**: dashboard component with mini chart (Recharts ComposedChart) and recent levels list
- **`useLevelingStats` hook**: React Query + Tauri event-driven cache updates

### What's missing
- No way to get ALL level events with full computed fields (timestamps, time-per-level, effective XP)
- No dedicated leveling history view
- No large-format chart for the full journey
- No leveling insights/summary stats

## Implementation

### Backend Changes

1. **Add `all_events: Vec<LevelEventResponse>` to `LevelingStats`** — all events sorted ascending by level with full computed fields
2. **Remove `chart_events` / `LevelChartEvent`** — `all_events` subsumes this data; the frontend can derive chart data from the richer type
3. **Refactor `build_event_responses`** — parameterize the limit (currently hardcoded to 5) so we can call it for both `recent_events` (5) and `all_events` (unlimited)

### Frontend Changes

1. **Update types** to mirror backend (add `all_events`, remove `chart_events`/`LevelChartEvent`)
2. **Update existing `LevelingChart`** to use `all_events` instead of `chart_events`
3. **New `LevelHistoryChart`** — full-width ComposedChart with XP/hr line, deaths bars, time-per-level line
4. **New `LevelHistoryTable`** — scrollable table: Level, Reached At, Time to Level, Deaths, Effective XP, XP/hr
5. **New `LevelingInsights`** — computed summary stats from all events

**Note:** No new route. Components will be placed on the character screen (built by a separate agent).

### Data Flow

No new commands, queries, or events needed. The existing `get_leveling_stats` command and `LevelingStatsUpdated` event already carry `LevelingStats`. Adding `all_events` to that payload means the new page gets real-time updates for free.

POE2 max level is 100, so `all_events` adds at most ~100 small objects to the payload — negligible overhead.

## Design Notes

- Chart uses theme colors: ember-400 (XP/hr line), blood-400 (deaths bars), molten-400 (time line)
- Table rows use `bg-stone-800/30` alternating pattern
- Deaths column highlighted in `text-blood-400` when > 0
- Follows existing page layout patterns (PageLayout component, Card wrappers, co-located .styles.ts)
- Active character only (same pattern as /playtime)

## Out of Scope
- New route / page (components go on character screen, handled separately)
- Comparison between characters
- Export/share functionality
- Custom date range filtering
