# Architecture Decisions

This file records major architectural and technical decisions for POE2 Overlord.

## State Management (2026-01-10)
**Decision**: Use Tanstack Query exclusively for server state
- No Redux, Zustand, or Context API for data fetching
- Local UI state can use React useState/useReducer
- **Rationale**: Reduces boilerplate, built-in caching, simpler mental model

### QueryClient Access (2026-01-11)
**Decision**: Export QueryClient singleton for edge cases, prefer useQueryClient() hook
- Primary pattern: Use `useQueryClient()` hook in all React contexts
- Edge case: Export `queryClient` from main.tsx for non-React contexts (event listeners, utilities)
- Safe accessor: `getQueryClient()` utility with initialization check
- Testing: Create new QueryClient per test with `retry: false`
- **Rationale**:
  - SPA context makes singleton safe (no SSR data leakage)
  - React hook is primary pattern (explicit dependencies, testable)
  - Export enables event system integration without architecture changes
  - Documented patterns prevent misuse

## Validation (2026-01-10)
**Decision**: Zod for all input validation
- All form inputs must have Zod schemas
- All Tauri command parameters validated with Zod
- API responses validated with Zod
- **Rationale**: Type safety, runtime validation, single source of truth

## Styling (2026-01-10)
**Decision**: Tailwind CSS v4 utility classes only
- No CSS modules, styled-components, or inline styles
- Complex component styles in separate `.styles.ts` files
- **Rationale**: Consistency, no CSS-in-JS overhead

## Testing Strategy (2026-01-10, updated 2026-01-10)
**Decision**: Co-located tests with Vitest
- Frontend: Vitest + React Testing Library, `.spec.tsx` files (**✅ COMPLETE - 517 tests**)
- Backend: Rust `#[tokio::test]`, `tests/` directory
- No Storybook yet (future consideration)
- **Rationale**: Tests close to code, easier to maintain

### Frontend Testing Stack (2026-01-10)
**Decision**: Vitest over Jest for frontend testing
- Vitest v4.x with jsdom environment
- React Testing Library v16.x for component testing
- @testing-library/user-event for realistic user interactions
- Co-located tests in `.spec.tsx` files next to components
- **Status**: ✅ **All 47 components have comprehensive tests (517 tests total)**
- **Rationale**: Native Vite integration, shared config, faster transforms, React 19 support
- **Commands**:
  - `yarn test:frontend` - Run all tests
  - `yarn test:watch` - Watch mode
  - `yarn test:ui` - Visual UI
  - `yarn test:coverage` - Coverage report

## Monorepo Structure (2026-01-10)
**Decision**: Yarn workspaces with strict package separation
- `packages/frontend/` - React UI
- `packages/backend/` - Rust + Tauri
- No shared packages (yet)
- **Rationale**: Clear boundaries, Tauri best practice

## Model Selection (2026-01-10)
**Decision**: Claude Opus 4 for Claude Code, Claude Sonnet 4 for Zed
- Opus for autonomous long-running tasks (Ralph loops)
- Sonnet for interactive development (faster, cheaper)
- **Rationale**: Match model capability to task complexity

---

## Template for New Decisions

## [Decision Name] (YYYY-MM-DD)
**Decision**: What was decided
- Key points
- Details
- **Rationale**: Why this decision was made
