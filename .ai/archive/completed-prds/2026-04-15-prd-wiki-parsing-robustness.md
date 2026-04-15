# PRD: Wiki Parsing Robustness

## Context

The wiki parser scrapes zone data from `poe2wiki.net` (a MediaWiki site) when players enter new zones. It works but is brittle — if the wiki's HTML structure changes, parsers silently return empty data. The original version of this PRD proposed a heavy strategy-chain architecture with confidence scores, fuzzy matching via `strsim`, and a phased rollout. That approach was over-engineered. This updated PRD achieves the same robustness goals with surgical fixes to existing code and no new abstractions. It also prunes unused data fields (`area_id`, `monsters`) and adds a `zone_type` field to fix incorrect zone tracking classification (hideouts, maps, and mechanic zones being counted as campaign time).

**Origin**: Issue #9 — wiki section parsing brittleness
**Priority**: HIGH (reliability improvement)

## Wiki HTML Structure (Ground Truth)

Analysis of real page HTML revealed several critical misconceptions in the current parser. This section documents the actual structure.

### Infobox

The infobox is NOT `table.infobox` or `table.wikitable`. The actual structure is:

```html
<div class="info-card">
  <div class="info-card__card">
    <div class="info-card__header">
      <div class="middle">
        <div class="heading">Zone Name</div>
        <div class="subheading">Town area</div>  <!-- or "area", "Map area", "Hideout area", "Tooltip" -->
      </div>
      <div class="right">
        <span typeof="mw:File">
          <span title="Waypoint">...</span>  <!-- or "No Waypoint", "Town Hub" -->
        </span>
      </div>
    </div>
    <div class="info-card__body">
      <div class="block">
        <table>  <!-- NO class on the table itself -->
          <tbody>
            <tr><th>Id</th><td><a ...>G1_11</a></td></tr>
            <tr><th>Act</th><td>1</td></tr>
            <tr><th><span class="tooltip" title="...">Area level</span></th><td>10</td></tr>
            <tr><th>Bosses</th><td></td></tr>   <!-- may be empty -->
            <tr><th>Tags</th><td>map, ...</td></tr>
            <tr><th>Connections</th><td><a>Zone A</a><br /><a>Zone B</a></td></tr>
          </tbody>
        </table>
      </div>
      <div class="block">
        <em class="tc -flavour">Flavor text here.</em>
      </div>
      <div class="block">
        <span typeof="mw:File"><a href="/wiki/File:..."><img src="/images/thumb/.../250px-name.jpg" /></a></span>
      </div>
    </div>
  </div>
</div>
```

### Zone Type Signals

| Zone Type | `subheading` text | Icon `title` attr | Notes |
|---|---|---|---|
| Town | `"Town area"` | `"Town Hub"` | Id contains `_town` |
| Campaign area | `"area"` | `"Waypoint"` or `"No Waypoint"` | |
| Map | `"Map area"` | `"No Waypoint"` | Act = 10, Tags includes `"map"` |
| Hideout | `"Hideout area"` | `"No Waypoint"` | Id starts with `Hideout` |
| Mechanic zone | `"area"` | `"No Waypoint"` | No Connections row |
| Tooltip (skip) | `"Tooltip"` | — | Not a zone info-card |

### Field Presence by Zone Type

| Field | Town | Campaign | Map | Hideout | Mechanic |
|---|---|---|---|---|---|
| Id | ✓ | ✓ | ✓ | ✓ | ✓ |
| Act | ✓ | ✓ | ✓ (= 10) | ✓ (= 1) | ✓ |
| Area level | ✓ | ✓ | ✓ | ✓ | ✓ |
| Bosses | ✗ | ✓ (often empty) | ✓ (often empty) | ✗ | ✗ |
| Tags | ✓ | ✓ | ✓ | ✓ | ✓ |
| Connections | ✓ | ✓ | ✗ | ✗ | ✗ |
| Biomes | ✗ | ✗ | ✓ | ✓ (map variant) | ✗ |

### Multi-Info-Card Pages

Some pages have multiple `div.info-card` elements:
- **Hideouts**: Two cards (hideout area + map area variant)
- **Mechanic zones** (e.g. Atziri's Temple): 4+ cards — zone area, sub-area, and tooltip cards with `subheading = "Tooltip"`

The correct card to parse is: the **first** `div.info-card` whose `subheading` is NOT `"Tooltip"` AND whose `heading` matches the zone name we fetched.

### Waypoint — CRITICAL MISCONCEPTION

There is **no "Waypoint" row in the infobox table**. Waypoint is indicated exclusively by the icon in `div.info-card__header .right span[title]`:
- `title="Waypoint"` → has waypoint
- `title="No Waypoint"` → no waypoint
- `title="Town Hub"` → is a town (towns always have waypoints)

The current `HasWaypointParser` and `IsTownParser` are reading the wrong place. The infobox validator also incorrectly includes "Waypoint" as a table indicator.

### Description/Flavor Text

Always `<em class="tc -flavour">text</em>` inside a `div.block` in `div.info-card__body`. NOT always present (e.g. the hideout area card has no flavor text). The current description parser scans the entire document for any `<em>`, which risks picking up nav, footnotes, or page body italic text.

### Image URL

The area screenshot is the second or third `div.block` image inside `div.info-card__body`. The `img src` is a thumbnail URL in the form `/images/thumb/h/hh/Filename.jpg/250px-Filename.jpg`. Full image = remove `/thumb` and trailing `/250px-Filename.jpg` segment.

---

## Problem Summary

Combining code review and HTML analysis, the concrete issues are:

1. **Wrong infobox selector** — Primary selectors target `table.infobox`/`table.wikitable`, but the actual structure is `div.info-card table` (table has no class). Currently only works via the last-resort `table` fallback + validation.
2. **`HasWaypointParser` reads wrong location** — Parses table rows, but waypoint is in the icon `title` attribute in the header.
3. **`IsTownParser` is a coincidence** — Checks if Id contains "town", which happens to be true but reads the wrong signal. Should use `subheading` text.
4. **Infobox validator includes "Waypoint"** — It's never a table label; the string only appears in the icon's alt/title attributes, making it a fragile false signal.
5. **Multi-info-card pages** — Parser grabs the first valid table, which may be wrong on hideout/mechanic pages with multiple info-cards.
6. **`extract_section_list_items`** — Only matches top-level `h2, h3, ul`. Content wrapped in `<div>` or using `<ol>` is invisible. Backs NPCs, bosses, and points_of_interest parsers.
7. **Heading matching** — Simple `contains()` with no synonym tolerance.
8. **`BossesParser` ignores the infobox** — There is a `Bosses` row in the infobox for some zones; it should be tried first.
9. **Description parser scans entire document** — Should scope to `div.info-card .block em[class*="flavour"]`.
10. **`extract_table_value` uses `Html` fragment but `Area level` has a tooltip span** — Text extraction must handle `<th><span>Area level</span></th>` correctly (it does via `element.text()`, but worth confirming in tests).
11. **No zone type classification** — There is no `zone_type` field on `ZoneMetadata`. The info-card `subheading` reliably classifies zones as "Town area", "area" (campaign/mechanic), "Map area", or "Hideout area", but we don't scrape it. This causes hideouts, maps, and mechanic zones to be incorrectly counted in campaign act time buckets. Currently hideout detection relies solely on `zone_name.contains("hideout")` string matching with no equivalent for maps or mechanic zones.

---

## Proposed Solution

### Batch 0: Remove `area_id` and `monsters` fields

Delete the `AreaIdParser` and `MonstersParser` modules and remove both fields from the entire data pipeline.

**Delete files:**
- `parsers/area_id_parser.rs`
- `parsers/area_id_parser_test.rs`
- `parsers/monsters_parser.rs`
- `parsers/monsters_parser_test.rs`

**Backend changes:**
- `wiki_scraping/parsers/mod.rs` — Remove module declarations, test declarations, and re-exports for both parsers
- `wiki_scraping/models.rs` — Remove `area_id` and `monsters` fields from `WikiZoneData` struct and `new()`
- `wiki_scraping/parser.rs` — Remove imports and usage of both parsers; update validation logic (remove `area_id.is_some()` check); update log message
- `wiki_scraping/parser_test.rs` — Update assertions and HTML fixtures
- `zone_configuration/models.rs` — Remove `area_id` and `monsters` from `ZoneMetadata` struct, `new()`, and `update_from_wiki_data()`
- `zone_configuration/models_test.rs` — Remove assertions and JSON fixture references
- `zone_configuration/repository.rs` — Remove from all SQL queries, tuple types, destructuring, and struct construction in `load_configuration()`, `upsert_zone()`, `get_zone_by_name()`, `get_zones_by_act()`, and `row_to_metadata()`
- `character/models.rs` — Remove `area_id` from `EnrichedLocationState` and `EnrichedZoneStats`; remove `monsters` from `EnrichedZoneStats`
- `character/models_test.rs` — Remove `area_id` assertion
- `character/repository.rs` — Remove from `get_character_zones()` SQL query and struct mapping
- `log_analysis/service.rs` — Remove `_area_id` variable and log references

**New migration** (do NOT edit `001_initial_schema.sql`):
- Create `migrations/009_remove_area_id_monsters.sql` — Recreate `zone_metadata` table without `area_id` and `monsters` columns (SQLite requires table recreation for column drops)

**Frontend changes:**
- `types/character.ts` — Remove `area_id` from `EnrichedLocationState` and `ZoneStats`; remove `monsters` from `ZoneStats`
- `queries/zones.ts` — Remove `area_id` and `monsters` from `ZoneMetadata` interface
- `utils/zone-utils.ts` — Remove from `createPlaceholderZone()`
- `utils/zone-utils.spec.ts` — Remove assertions and fixture references
- `components/zones/zone-details-modal/zone-details-modal.tsx` — Remove Area ID display block
- `test/mock-data.ts` — Remove `monsters: []` from `createMockZone()`
- `components/zones/zone-card/zone-card.spec.tsx` — Remove from fixtures
- `components/zones/zone-list/zone-list.spec.tsx` — Remove from fixtures
- `components/zones/current-zone-card/current-zone-card.spec.tsx` — Remove from fixtures
- `components/insights/playtime-insights/playtime-insights.spec.tsx` — Remove from fixtures

**Note**: `is_town_parser.rs` reads the infobox `Id` field directly via `BaseParser::extract_table_value` — it does NOT depend on `AreaIdParser`. However, it will be rewritten in Batch 1 anyway.

### Batch 1: Infobox Parser Rewrite (critical correctness fixes)

#### 1. Fix infobox selector (infobox_parser.rs)

Replace the cascade of `table.infobox`, `table.wikitable`, etc. selectors with the correct selector: `div.info-card table`.

For pages with multiple info-cards (hideouts, mechanic zones), select the correct one:
- Skip any `div.info-card` whose `div.subheading` text is `"Tooltip"`
- Use the first remaining card (this is the zone's primary area info-card)

The function signature and returned `Html` fragment type stay the same.

**File**: `packages/backend/src/domain/wiki_scraping/parsers/infobox_parser.rs`

#### 2. Fix infobox validation (infobox_parser.rs)

`is_valid_zone_infobox` currently checks raw HTML for ["Act", "Area level", "Id", "Connections", "Waypoint"]. Problems:
- "Waypoint" is never a table row label — remove it
- "Connections" is absent on maps, hideouts, and mechanic zones — remove it
- Raw HTML string matching can false-positive on attributes

Replace with: parse the fragment, select `th` elements, check their text content. A valid zone infobox must have at least 2 of: `["Id", "Act", "Area level"]`.

**File**: `packages/backend/src/domain/wiki_scraping/parsers/infobox_parser.rs`

#### 3. Rewrite HasWaypointParser (has_waypoint_parser.rs)

Currently reads table rows — there is no "Waypoint" table row. Rewrite to:
- Select `div.info-card__header .right span[title]` (or equivalent) from the **full document**, not the infobox table fragment
- `title="Waypoint"` → `true`
- `title="Town Hub"` → `true` (towns have waypoints)
- `title="No Waypoint"` or absent → `false`

This requires passing the full `document: &Html` instead of the infobox fragment. Update the call site in `parser.rs`.

**File**: `packages/backend/src/domain/wiki_scraping/parsers/has_waypoint_parser.rs`

#### 4. Rewrite IsTownParser (is_town_parser.rs)

Currently checks if the infobox `Id` value contains "town" — a coincidence that works but reads the wrong signal. Rewrite to read the correct signals from the full document:
1. Check `div.info-card__header .middle .subheading` text == `"Town area"` (primary)
2. Fallback: check icon `title="Town Hub"`
3. Fallback: Id contains `_town`

**File**: `packages/backend/src/domain/wiki_scraping/parsers/is_town_parser.rs`

#### 5. Add ZoneTypeParser + `zone_type` field (new parser)

The info-card `subheading` reliably classifies every zone. Add a new `ZoneType` enum and parser to extract it.

**New enum** (in `wiki_scraping/models.rs` or `zone_configuration/models.rs`):
```rust
pub enum ZoneType {
    Campaign,    // subheading = "area" (non-map, non-hideout)
    Town,        // subheading = "Town area"
    Map,         // subheading = "Map area"
    Hideout,     // subheading = "Hideout area"
    Unknown,     // no wiki data yet or unrecognized subheading
}
```

**New parser** (`parsers/zone_type_parser.rs`):
- Reads from the full document (not infobox fragment)
- Select the first `div.info-card` whose subheading is NOT "Tooltip"
- Map subheading text → `ZoneType` enum

**Data pipeline additions:**
- `WikiZoneData` — add `zone_type: ZoneType` field
- `ZoneMetadata` — add `zone_type: String` field (stored as text in SQLite)
- `zone_configuration/repository.rs` — add `zone_type` to all SQL queries
- `character/models.rs` — add `zone_type` to `EnrichedZoneStats`
- New migration `009_...` — add `zone_type TEXT NOT NULL DEFAULT 'Unknown'` column (this can be combined with the `area_id`/`monsters` removal migration)

**Frontend additions:**
- `types/character.ts` — add `zone_type` to `ZoneStats`
- `queries/zones.ts` — add `zone_type` to `ZoneMetadata` interface

**Zone tracking fixes** (the payoff):
- `zone_tracking/models.rs` — `TrackingSummary::from_zones()` should use `zone_type` to determine act bucketing:
  - `Campaign` and `Town` → bucket by act number as today
  - `Map` → always `play_time_endgame`
  - `Hideout` → `total_hideout_time` (already tracked, but now based on wiki data not string matching)
  - `Unknown` → bucket by act (backward compatible with zones that haven't been wiki-scraped yet)
- `zone_tracking/models.rs` — `is_hideout_zone()` can fall back to `zone_type == Hideout` when available, keeping the string-match as a fallback for zones without wiki data
- `character/repository.rs` — SQL queries for `TrackingSummary` can filter by `zone_type` instead of `LOWER(zone_name) LIKE '%hideout%'`

#### 6. Fix description parser (description_parser.rs)

Replace the full-document `<em>` scan with a targeted selector: `em[class*="flavour"]` within `div.info-card`. This matches `<em class="tc -flavour">` precisely and avoids picking up italic text from page body, nav, or tables.

Note: CSS class selector `.-flavour` is invalid (leading hyphen). Use `em[class*="flavour"]` attribute selector instead.

**File**: `packages/backend/src/domain/wiki_scraping/parsers/description_parser.rs`

### Batch 2: Data Extraction Fixes

#### 7. Fix BossesParser to check infobox first (bosses_parser.rs)

The infobox has a `Bosses` row on campaign and map zones (often empty, but present when there are named bosses). Check the infobox table via `extract_table_value(infobox, "Bosses")` first. If it has a non-empty value, parse the `<a>` links from that cell. Fall back to section heading search only if the infobox row is absent or empty.

Also remove the single-capitalized-word heuristic from the section fallback (it false-positives on generic monster names).

**File**: `packages/backend/src/domain/wiki_scraping/parsers/bosses_parser.rs`

#### 8. Improve section content extraction (base.rs)

Replace the flat CSS selector approach in `extract_section_list_items` with sibling-walking:
1. Find the heading via `document.select("h2, h3, h4")` with heading text matching
2. Walk siblings using `element.next_siblings()`
3. For each sibling:
   - Heading of equal/higher rank → stop
   - `<ul>`, `<ol>`, `<dl>` → collect `li`/`dd` items
   - `<div>` or `<section>` → descend into it looking for list elements
   - `<figure>` → skip (images in section headers)
4. Fall back to current behavior if sibling walking finds nothing

Keep same function signature.

**File**: `packages/backend/src/domain/wiki_scraping/parsers/base.rs`

#### 9. Add heading synonym matching (base.rs)

Add `matches_heading_name(heading_text: &str, target: &str) -> bool`:
- Strip MediaWiki edit-section `[edit]` artifacts
- Normalize (lowercase, trim)
- Check against static synonym array:
  - `"bosses"` → `["boss monsters", "unique monsters", "boss encounters"]`
  - `"npcs"` → `["npc list", "non-player characters", "characters"]`
  - `"points of interest"` → `["notable locations", "landmarks", "notable areas"]`
- Fall back to `contains()` as last resort

Update `is_section_heading` to use this.

**File**: `packages/backend/src/domain/wiki_scraping/parsers/base.rs`

### Batch 3: Polish

#### 10. Connected zones text fallback (connected_zones_parser.rs)

- Expand patterns beyond "connected to": include `"connects to"`, `"leads to"`, `"adjacent to"`, `"accessed from"` (use `regex` crate, already a dependency)
- When using text fallback, prefer extracting `<a>` links from the matching paragraph over comma-splitting plain text

**File**: `packages/backend/src/domain/wiki_scraping/parsers/connected_zones_parser.rs`

#### 11. Improve redirect detection (infobox_parser.rs)

Expand `is_redirect_page` to also check:
- `#REDIRECT` / `#redirect` in first `<p>` or `div.mw-parser-output`
- `div.redirectMsg` or `div.mw-redirect` elements
- Keep existing title check as one signal

**File**: `packages/backend/src/domain/wiki_scraping/parsers/infobox_parser.rs`

#### 12. Diagnostic logging (parser.rs)

- After each parser call in `parse_zone_data`, log at `debug` when a parser returns None/empty/default
- Add `warn` log when 3+ parsers return empty for a zone that has an infobox (suggests structural breakage, not a sparse page)

**File**: `packages/backend/src/domain/wiki_scraping/parser.rs`

---

## Testing

Tests must use HTML fixtures that match the actual wiki structure documented above. The old fixture HTML using `table.infobox` or `table.wikitable` must be updated to use `div.info-card` structure.

Key test cases per batch:

**Batch 0**: Compilation passes; no references to `area_id`/`monsters` remain; migration runs cleanly on existing DB.

**Batch 1**:
- Infobox found on all 5 zone type fixtures (town, campaign, map, hideout, mechanic)
- Hideout page: first non-Tooltip info-card is selected (hideout area card, not map card)
- Atziri page: first non-Tooltip info-card is selected
- `has_waypoint` correctly `true` for Hunting Grounds (Waypoint icon), `false` for Frozen Falls (No Waypoint icon), `true` for Kingsmarch (Town Hub icon)
- `is_town` correctly `true` for Kingsmarch, `false` for all others
- `zone_type` correctly `Town` for Kingsmarch, `Campaign` for Hunting Grounds, `Map` for Frozen Falls, `Hideout` for Felled Hideout, `Campaign` for Atziri's Temple
- Description extracted from `em.tc.-flavour` only; NOT from italic in page body
- Zone tracking: hideout time NOT counted in campaign act buckets; map time goes to endgame bucket

**Batch 2**:
- Section content inside `<div>` wrapper is found
- `<ol>` and `<dl>` list items are found
- `<figure>` in section does not cause early termination
- Boss from infobox row is returned when present
- Single-word heuristic removal: "Zombie", "Skeleton" are not returned as bosses

Verify with `pnpm test:backend` after each batch.

---

## What This Intentionally Excludes

- **No `ParsingStrategy` trait / `StrategyChain`** — Fallback logic inside each parser is clearer than external orchestration.
- **No `strsim` fuzzy matching** — Exact synonym lists are more predictable. "Monster Affixes" should NOT match "Monsters" despite low edit distance.
- **No confidence scores** — Parsers find data or they don't. No competing results to rank.
- **No MediaWiki API** — Would give structured data but is a major architectural change. Possible future direction.

## References

- Issue #9: Wiki section parsing brittleness
- Related: Issue #10 (redirects), Issue #32-34 (other wiki issues)
- HTML analysis: Kingsmarch (town), Felled Hideout (hideout), Frozen Falls (map), Hunting Grounds (campaign), Atziri's Temple (mechanic)
