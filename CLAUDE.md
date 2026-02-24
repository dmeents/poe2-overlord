# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

POE2 Overlord is a companion app for Path of Exile 2 built with Tauri, with a marketing website. It's a monorepo with:
- **packages/theme**: Shared Tailwind v4 design tokens + utilities (consumed by frontend & website)
- **packages/frontend**: React 19 + TypeScript + Vite (desktop app UI)
- **packages/backend**: Rust + Tauri 2.8 (desktop app backend)
- **packages/website**: Next.js 15 (marketing site for downloads & docs)

## AI Workspace (`.ai/` directory)

**IMPORTANT**: Always read `.ai/memory/` at the start of each session for architectural decisions and patterns.

### Directory Structure
- **`.ai/memory/`** - Long-term context (ALWAYS READ FIRST)
  - `decisions.md` - Architecture decisions
  - `patterns.md` - Code patterns and examples
- **`.ai/tasks/`** - Active PRDs and work items
- **`.ai/sessions/`** - Session notes and Ralph loop logs
- **`.ai/prompts/`** - Reusable prompt templates
- **`.ai/archive/`** - Completed work
  - `completed-prds/` - Archived PRDs (named `YYYY-MM-DD-prd-name.md`)
  - `old-sessions/` - Old session logs

### PRD Workflow
1. **When starting a PRD**: Create in `.ai/tasks/` (e.g., `prd-feature-name.md`)
2. **While working**: Save notes, progress, and learnings in `.ai/sessions/YYYY-MM-DD-task.md`
3. **When complete**: Move the PRD to `.ai/archive/completed-prds/YYYY-MM-DD-prd-name.md`

### Usage Guidelines
1. **Before starting work**: Read all files in `.ai/memory/`
2. **During work**: Reference the relevant PRD in `.ai/tasks/`
3. **After decisions**: Update `.ai/memory/decisions.md`
4. **After Ralph loops**: Log session to `.ai/sessions/YYYY-MM-DD-task.md`
5. **When learning patterns**: Update `.ai/memory/patterns.md`

## Common Commands

```bash
# Desktop App Development
pnpm dev                    # Start Tauri dev mode (frontend + backend)
pnpm build                  # Production build

# Website Development
pnpm dev:website            # Start Next.js dev server
pnpm build:website          # Production build for website
pnpm start:website          # Start production server
pnpm lint:website           # Lint website code
pnpm typecheck:website      # Type check website

# Linting & Formatting
pnpm lint                   # Run all linters (all packages)
pnpm format                 # Format all code (all packages)

# Testing
pnpm test                   # Run all tests
pnpm test:backend           # Run Rust tests only
pnpm test:backend:verbose   # Rust tests with output
pnpm test:backend:watch     # Rust tests in watch mode

# Type Checking
pnpm typecheck              # Type check all packages
pnpm check:backend          # Run cargo check
```

## Architecture

### Frontend (packages/frontend)

**Routing**: TanStack Router with file-based routing in `src/routes/`. Route files auto-generate `src/routeTree.gen.ts` - don't edit this file manually.

**State Management**:
- **Server state**: TanStack React Query (`src/queries/`) with centralized query keys
- **App state**: React Context providers (`src/contexts/`) wrapping app in `src/providers.tsx`
- **Real-time updates**: Event listeners via `useAppEventListener` hook subscribing to Tauri events

**Frontend-Backend Communication**:
```tsx
// Call Rust commands via invoke()
import { invoke } from '@tauri-apps/api/core';
const result = await invoke<CharacterData>('create_character', { name, class: characterClass });
```

**Path Aliases**: Use `@/` prefix for imports (`@/components`, `@/hooks`, `@/types`, `@/utils`)

### Backend (packages/backend)

**Domain Structure**: Each domain in `src/domain/` follows this pattern:
- `models.rs` - Data structures with Serde derive
- `traits.rs` - Service interface (async trait)
- `service.rs` - Implementation
- `repository.rs` - Data persistence
- `commands.rs` - Tauri IPC handlers

**Adding Tauri Commands**:
1. Add `#[tauri::command]` handler in `commands.rs`
2. Register in `src/lib.rs` in the `invoke_handler` macro
3. Initialize service in `src/application/service_registry.rs`

**Event System**: Backend publishes events via `EventBus`, which forwards to frontend through `TauriEventBridge`. Register frontend event handlers in `src/utils/events/registry.ts`.

**Error Handling**: Use `AppResult<T>` for service methods, convert to `CommandResult<T>` with `to_command_result()` for Tauri commands.

## Code Style

### TypeScript/React
- Single quotes, semicolons, 2-space indent
- Use `memo()`, `useMemo()`, `useCallback()` for component optimization
- Component styles in co-located `.styles.ts` files

### Rust
- Max line width: 100 chars
- Use field init shorthand and try shorthand (`?`)
- All services use `Arc<T>` for thread-safe sharing
- All service interfaces are async traits

## Theming & Styling

### Shared Theme Package
Design tokens are centralized in `@poe2-overlord/theme` (`packages/theme/src/css/tokens.css`). This package is shared between the desktop app and website for visual consistency.

**Package structure:**
- `src/css/tokens.css` - Tailwind v4 `@theme` block with all colors, shadows, spacing
- `src/cn.ts` - `cn()` utility for merging Tailwind classes
- `src/theme-utils.ts` - `getThemeHexColor()` for reading CSS variable values
- `src/index.ts` - Barrel exports

**Usage in packages:**
```css
/* In globals.css */
@import "tailwindcss";
@import "@poe2-overlord/theme/tokens.css";
```

```tsx
/* In TypeScript files */
import { cn, getThemeHexColor } from '@poe2-overlord/theme';
```

### Design Tokens
All design tokens (colors, shadows, spacing) are defined in the shared theme package. This is the **single source of truth**.

**Never hardcode values in components.** Use Tailwind classes that reference these tokens:
```tsx
// Good - uses Tailwind classes and filter-based shadows
<div className="bg-stone-900 card-shadow">

// Bad - hardcoded values
<div className="bg-[#1c1917] shadow-[0_4px_6px_rgba(0,0,0,0.7)]">
```

**Shadows:** Use filter-based shadow classes (`card-shadow`, `chrome-shadow-*`) instead of Tailwind's `shadow-*` utilities. This is required due to a WebKitGTK compositor bug - see ADR-002 in `.ai/memory/decisions.md`.

### Color Palette
| Token   | Purpose                              |
|---------|--------------------------------------|
| `ember` | Primary accent (volcanic orange)     |
| `molten`| Secondary accent (gold/amber)        |
| `blood` | Danger states, Warrior class         |
| `bone`  | Muted text, subtle highlights        |
| `stone` | Neutral backgrounds (warm gray)      |
| `ash`   | Disabled/muted states (cool gray)    |

**Class Colors** (character identity): `blood` (Warrior), `arcane` (Sorceress), `verdant` (Ranger), `molten` (Huntress), `spirit` (Monk), `ember` (Mercenary), `hex` (Witch), `primal` (Druid)

Use `@/utils/class-colors.ts` for character-specific styling:
```tsx
import { getClassTextColor, getClassTheme } from '@/utils/class-colors';
getClassTextColor('Warrior')   // 'text-blood-400'
getClassTheme('Warrior')       // 'blood'
```

### Z-Index Scale
| Class   | Value | Use case                                      |
|---------|-------|-----------------------------------------------|
| `z-0`   | 0     | Base content (default)                        |
| `z-10`  | 10    | Elevated cards, hover states                  |
| `z-20`  | 20    | Dropdowns, popovers, tooltips                 |
| `z-30`  | 30    | Fixed UI chrome (titlebar, sidebar, statusbar)|
| `z-40`  | 40    | Notifications, toasts                         |
| `z-50`  | 50    | Modals, dialogs (blocking UI)                 |

### Component Styles
Each component has a co-located `.styles.ts` file. Use theme colors (`stone-*`, `ember-*`) and filter-based shadow classes (`card-shadow`, `chrome-shadow-*`).

## Key Domains

| Domain | Purpose |
|--------|---------|
| character | Player character tracking and CRUD |
| configuration | App settings management |
| economy | Currency exchange rates |
| game_monitoring | Game process detection |
| log_analysis | POE Client.txt parsing |
| server_monitoring | POE server status |
| walkthrough | Campaign progress guide |
| zone_tracking | Zone statistics |
