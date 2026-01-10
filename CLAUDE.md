# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

POE2 Overlord is a game overlay for Path of Exile 2 built with Tauri. It's a monorepo with:
- **packages/frontend**: React 19 + TypeScript + Vite
- **packages/backend**: Rust + Tauri 2.8

## AI Workspace (`.ai/` directory)

**IMPORTANT**: Always read `.ai/memory/` at the start of each session for architectural decisions and patterns.

### Directory Structure
- **`.ai/memory/`** - Long-term context (ALWAYS READ FIRST)
  - `decisions.md` - Architecture decisions
  - `patterns.md` - Code patterns and examples
- **`.ai/tasks/`** - Active work
  - `current-prd.md` - Current feature being built
  - `backlog.md` - Future work
- **`.ai/sessions/`** - Ralph loop logs (auto-generated)
- **`.ai/prompts/`** - Reusable prompt templates
- **`.ai/archive/`** - Completed work

### Usage Guidelines
1. **Before starting work**: Read all files in `.ai/memory/`
2. **During work**: Reference `.ai/tasks/current-prd.md` for context
3. **After decisions**: Update `.ai/memory/decisions.md`
4. **After Ralph loops**: Log session to `.ai/sessions/YYYY-MM-DD-task.md`
5. **When learning patterns**: Update `.ai/memory/patterns.md`

## Common Commands

```bash
# Development
yarn dev                    # Start Tauri dev mode (frontend + backend)
yarn build                  # Production build

# Linting & Formatting
yarn lint                   # Run all linters (TypeScript + Rust)
yarn format                 # Format all code (Prettier + rustfmt)

# Testing
yarn test                   # Run all tests
yarn test:backend           # Run Rust tests only
yarn test:backend:verbose   # Rust tests with output
yarn test:backend:watch     # Rust tests in watch mode

# Type Checking
yarn check:backend          # Run cargo check
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
