# PRD: Character Detail View

**Status:** Active
**Created:** 2026-03-29

## Problem

Character metrics are scattered across three pages:
- **Dashboard**: Character card, current zone, leveling stats
- **Playtime**: Zone list, playtime insights, act distribution chart
- **Characters**: Aggregate insights (cross-character, not per-character)

Users have no single page to view all metrics for their active character.

## Solution

New `/character` route that consolidates all per-character metric components into one comprehensive view. Reuses existing components entirely -- no new metric components needed.

## Layout

### Left Column (2/3 width)
1. `CharacterStatusCard` -- Character identity and key stats
2. `CurrentZoneCard` -- Active zone with live timer (empty state if no active zone)
3. `LevelHistoryChart` -- **NEW** Expanded XP/hr chart (280px) with time/level toggle
4. `LevelHistoryTable` -- **NEW** Full table of all level-up events
5. `ZoneList` -- Filterable/sortable zone history with `useListControls`

### Right Column (1/3 width)
1. `LevelingStatsCard` -- Live leveling metrics (est. next level, XP/hr, deaths, recent levels)
2. `LevelingInsights` -- **NEW** Aggregate leveling stats (peak/avg XP/hr, fastest/slowest level)
3. `PlaytimeInsights` -- Time breakdown (active/hideout/town %), deaths/hr, top zones
4. `ActDistributionChart` -- Pie chart of time per act

## Implementation

### Create
- `packages/frontend/src/routes/character.tsx` -- Route file following `playtime.tsx` pattern

### Modify
- `packages/frontend/src/components/layout/sidebar-navigation/sidebar-navigation.tsx` -- Add `UserIcon` nav item at index 1 (between Dashboard and Walkthrough)

### Auto-generated
- `routeTree.gen.ts` -- Regenerates on dev server start

## Design Decisions

- **`/character` (singular)** vs `/characters` (plural, existing) -- singular for the "my active character" detail, plural for the character management list. Different icons: `UserIcon` vs `UserGroupIcon`.
- **Reuse over recreation** -- All 6 metric components exist and are imported directly. No wrappers or new presentation components.
- **ZoneList duplication with playtime page** -- Intentional. This is the "one-stop shop"; playtime page can later evolve into cross-character analytics.

## Acceptance Criteria

- [ ] `/character` route renders with all 6 component sections
- [ ] Sidebar shows "Character" icon between Dashboard and Walkthrough
- [ ] Empty states display when no active character is selected
- [ ] Zone list filtering and sorting works correctly
- [ ] Live timers (zone elapsed, level time) tick correctly
- [ ] `pnpm typecheck` and `pnpm lint` pass with no errors
