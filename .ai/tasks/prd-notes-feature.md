# PRD: Notes Feature

## Context

Players need a way to keep notes while playing Path of Exile 2 — build guides, boss mechanics reminders, trade notes, zone tips, etc. This feature adds a lightweight markdown text editor with the ability to pin important notes to the home dashboard for quick reference.

## Scope

- Full CRUD for notes (create, read, update, delete)
- Markdown support (textarea with preview toggle using `react-markdown`)
- Pin/unpin notes to the home screen dashboard
- Optional character association (filter notes per character)
- New `/notes` route with inline editor (left) + note list (right)
- Pinned notes card on the dashboard

## Data Model

### Database (migration 007_notes.sql)

```sql
CREATE TABLE notes (
    id           TEXT    PRIMARY KEY,
    title        TEXT    NOT NULL,
    content      TEXT    NOT NULL DEFAULT '',
    is_pinned    INTEGER NOT NULL DEFAULT 0,
    character_id TEXT    REFERENCES characters(id) ON DELETE SET NULL,
    created_at   TEXT    NOT NULL,
    updated_at   TEXT    NOT NULL
);
CREATE INDEX idx_notes_character ON notes(character_id);
CREATE INDEX idx_notes_pinned ON notes(is_pinned);
CREATE INDEX idx_notes_updated ON notes(updated_at DESC);
```

- `character_id` nullable with ON DELETE SET NULL — deleting a character preserves the note
- `is_pinned` as INTEGER (SQLite boolean convention)

### Rust Models

- `NoteData` — full note struct (id, title, content, is_pinned, character_id, created_at, updated_at)
- `CreateNoteParams` — title, content, character_id (optional)
- `UpdateNoteParams` — title, content, character_id (optional)

## Architecture Decisions

### No Context / No EventBus needed

Notes are user-initiated CRUD only — no background process updates notes. React Query handles caching and invalidation via mutations. EventBus can be added later if multi-window support is needed.

### Markdown: Textarea + Preview Toggle

Simple textarea for writing, with a Write/Preview toggle button. Uses `react-markdown` for rendering preview. No heavy editor framework — this is quick notes for gameplay, not a publishing tool.

### Page Layout: Editor Left, List Right

The editor/viewer gets the left column (2/3 width) for ample writing space. The note list lives in the right column (1/3 width) for quick navigation. Clicking a note in the list loads it into the left-column editor. No modal needed for editing — inline only.

## Implementation Plan

### Phase 1: Backend — Database Migration

**File to create:**
- `packages/backend/src/infrastructure/database/migrations/007_notes.sql`

### Phase 2: Backend — Domain Module

**Files to create** in `packages/backend/src/domain/notes/`:

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations, re-exports |
| `models.rs` | NoteData, CreateNoteParams, UpdateNoteParams with Serde derives |
| `traits.rs` | `NotesRepository` + `NotesService` async traits |
| `repository.rs` | `NotesRepositoryImpl` — sqlx raw SQL against SQLite |
| `service.rs` | `NotesServiceImpl` — UUID gen, timestamps, title validation |
| `commands.rs` | 7 Tauri command handlers |

**Tauri commands:**
- `create_note(title, content, character_id)` → `CommandResult<NoteData>`
- `get_note(note_id)` → `CommandResult<NoteData>`
- `get_all_notes()` → `CommandResult<Vec<NoteData>>`
- `get_pinned_notes()` → `CommandResult<Vec<NoteData>>`
- `update_note(note_id, title, content, character_id)` → `CommandResult<NoteData>`
- `delete_note(note_id)` → `CommandResult<()>`
- `toggle_note_pin(note_id)` → `CommandResult<NoteData>`

### Phase 3: Backend — Wiring

**Files to modify:**
- `packages/backend/src/domain/mod.rs` — add `pub mod notes;` + re-exports
- `packages/backend/src/application/service_registry.rs` — instantiate repo + service, `app.manage()`
- `packages/backend/src/lib.rs` — add `pub use domain::notes::commands::*;`, add commands to `invoke_handler`

### Phase 4: Frontend — Types & Queries

**Files to create:**
- `packages/frontend/src/types/notes.ts` — NoteData, CreateNoteParams, UpdateNoteParams interfaces
- `packages/frontend/src/queries/notes.ts` — query keys, useNotes, usePinnedNotes, useNote, useCreateNote, useUpdateNote, useDeleteNote, useToggleNotePin

Pattern: follows `packages/frontend/src/queries/characters.ts` exactly. Mutations invalidate `noteQueryKeys.all`.

### Phase 5: Frontend — Components

**Files to create:**

| Component | Files | Purpose |
|-----------|-------|---------|
| `note-editor` | `components/notes/note-editor/note-editor.tsx` + `.styles.ts` | Title input + textarea with Write/Preview toggle. Uses `react-markdown` for preview. Inline save/cancel. |
| `note-list-item` | `components/notes/note-list-item/note-list-item.tsx` + `.styles.ts` | Single note in the list — title, preview snippet, pin icon, timestamp |
| `note-list` | `components/notes/note-list/note-list.tsx` + `.styles.ts` | Scrollable list of note items with "New Note" button |
| `delete-note-modal` | `components/notes/delete-note-modal/delete-note-modal.tsx` + `.styles.ts` | Delete confirmation dialog (follows DeleteCharacterModal pattern) |
| `pinned-notes-card` | `components/notes/pinned-notes-card/pinned-notes-card.tsx` + `.styles.ts` | Dashboard card showing pinned note titles/previews with links |
| `note-list.config` | `hooks/configs/note-list.config.ts` | Filter (character, pinned) + sort (title, updated_at) config for useListControls |

### Phase 6: Frontend — Route & Navigation

**File to create:**
- `packages/frontend/src/routes/notes.tsx` — Notes page

Layout:
- **Left column (2/3)**: Note editor/viewer — shows selected note in editor, or empty state prompting to select/create a note
- **Right column (1/3)**: Note list card with search, filter by character/pinned, sort controls, and "+ New Note" button

State management:
- `selectedNoteId` — which note is loaded in the editor
- `isCreating` — when true, editor shows a blank form for a new note
- No modals needed for create/edit (inline in left column)
- Delete confirmation via modal

**Files to modify:**
- `packages/frontend/src/components/layout/sidebar-navigation/sidebar-navigation.tsx` — add `{ path: '/notes', title: 'Notes', icon: DocumentTextIcon }` to `primaryNavItems`
- `packages/frontend/src/routes/index.tsx` — add `<PinnedNotesCard />` to right column after `<LevelingStatsCard />`

### Phase 7: Frontend — Dependency

**File to modify:**
- `packages/frontend/package.json` — add `react-markdown` dependency

## Key Reference Files

| Purpose | File |
|---------|------|
| Backend command pattern | `packages/backend/src/domain/character/commands.rs` |
| Backend repository pattern | `packages/backend/src/domain/character/repository.rs` |
| Backend service pattern | `packages/backend/src/domain/character/service.rs` |
| Backend trait pattern | `packages/backend/src/domain/character/traits.rs` |
| Service registration | `packages/backend/src/application/service_registry.rs` |
| Command registration | `packages/backend/src/lib.rs` |
| Frontend query pattern | `packages/frontend/src/queries/characters.ts` |
| Frontend route pattern | `packages/frontend/src/routes/characters.tsx` |
| List config pattern | `packages/frontend/src/hooks/configs/character-list.config.ts` |
| Sidebar nav | `packages/frontend/src/components/layout/sidebar-navigation/sidebar-navigation.tsx` |
| Dashboard | `packages/frontend/src/routes/index.tsx` |

## Verification

1. `pnpm test:backend` — Rust compiles and existing tests pass
2. `pnpm dev` — verify:
   - Notes icon appears in sidebar navigation
   - Can create a note with title + markdown content
   - Can switch between Write/Preview modes
   - Can select a note from the right-column list to edit inline
   - Can pin/unpin notes
   - Can delete notes (with confirmation)
   - Can associate a note with a character
   - Pinned notes appear on dashboard home screen
   - Deleting a character preserves associated notes
3. `pnpm typecheck` — no TypeScript errors
4. `pnpm lint` — no lint errors
