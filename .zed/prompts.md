# POE2 Overlord - Zed AI Prompts

## Project Overview

POE2 Overlord is a desktop overlay application for Path of Exile 2, providing real-time game information and character management.

**Tech Stack:**
- **Frontend**: React 19 + Tanstack Router/Query + Tailwind CSS v4
- **Backend**: Rust + Tauri 2.8 + tokio async runtime
- **Build**: Vite 7 + yarn workspaces monorepo

**Architecture:**
- Strict separation: UI logic in React, system/performance-critical code in Rust
- IPC communication via Tauri commands
- State management exclusively through Tanstack Query (no Redux/Zustand)

---

## Frontend Development (packages/frontend/**)

You are an expert full-stack developer proficient in TypeScript, React, Tanstack Router, and Tailwind CSS.

### Code Style
- Functional, declarative patterns (no classes)
- Descriptive names: `isLoading`, `hasError`
- File naming: `{name}.component.tsx`, `{name}.route.tsx`, `{name}.styles.ts`
- Co-located tests: `{name}.spec.tsx`

### Standards
- **Validation**: Zod schemas for all inputs/API responses
- **Error Handling**: Custom error types, centralized error boundaries
- **Styling**: Tailwind CSS v4
- **State**: Tanstack React Query
- **Testing**: Jest + React Testing Library
- **Accessibility**: ARIA attributes, keyboard nav, color contrast
- **Documentation**: JSDoc for functions/components

### Mandatory Deliverables
- Unit tests (`.spec.tsx`) co-located with components
- Error boundaries for all user-facing components
- Type safety with Zod for all inputs/API responses
- JSDoc comments for exported functions

---

## Backend Development (packages/backend/**)

You are an expert in Rust, async programming with tokio, and concurrent systems.

### Code Style
- `snake_case` for variables/functions, `PascalCase` for types
- Async-first with tokio runtime
- Modular architecture separating concerns

### Standards
- **Async**: tokio for all I/O, use `tokio::spawn` for concurrency
- **Channels**: `mpsc`, `broadcast`, `oneshot` for task communication
- **Errors**: `thiserror` for custom errors, propagate with `?`
- **Testing**: `tokio::test` for async tests, `mockall` for mocks
- **Documentation**: Rustdoc comments

### Mandatory Deliverables
- Unit tests with `#[tokio::test]`
- Integration tests in `tests/`
- Rustdoc comments
- Error handling with `Result<T, E>`

---

## Tauri Integration (Always Apply)

Expert in TypeScript + Rust for cross-platform desktop apps with Tauri 2.x.

### Workflow
1. Think step-by-step, write pseudo-code
2. **For interactive sessions (Zed)**: Confirm approach before implementing
3. **For autonomous sessions (Claude Code)**: Proceed directly after planning
4. Fully implement (NO todos/placeholders)
5. Ensure type safety across TypeScript ↔ Rust boundary

### Standards
- Tauri 2.8+, React 19, Vite 7
- Security-first: validate all IPC calls
- Performance-critical code in Rust
- UI/UX code in TypeScript/React

### After Implementation
1. Run `yarn format` (auto-format all code)
2. Run `yarn lint` (fix linting issues)
3. Run `yarn test` (validate changes)
4. **Claude Code only**: Auto-commit with descriptive message including `Co-Authored-By: Warp <agent@warp.dev>`
5. **Zed only**: Report completion, wait for user to commit

---

## Tool-Specific Behavior

### When to Use Each Tool

**Zed (Interactive Development)**
- Quick edits and exploration
- Real-time code review with human
- Learning new parts of codebase
- Architectural decisions requiring discussion
- Complex debugging requiring human insight

**Claude Code (Autonomous Execution)**
- Long-running refactoring tasks
- Test coverage improvements (Ralph loops)
- Fixing lint/format issues across codebase
- Batch operations (add JSDoc, update imports)
- Overnight work on mechanical tasks

### Shared Standards (Both Tools)

Both Zed and Claude Code must:
1. Follow the same code style (functional, descriptive names, file naming)
2. Use same validation (Zod for all inputs)
3. Run same commands (`yarn lint`, `yarn format`, `yarn test`)
4. Maintain same architecture decisions
5. Follow same anti-patterns (no classes, no inline styles, etc.)

### Key Difference

**Zed**: Interactive, human confirms each step
**Claude Code**: Autonomous, human reviews final result

---

## Project-Specific Decisions

### File Structure
- **Components**: `packages/frontend/src/components/{domain}/{name}/`
  - Example: `packages/frontend/src/components/character/character-card/`
- **Tauri Commands**: `packages/backend/src/commands/`
- **Tests**: Co-located with source files (same directory)
- **Styles**: Separate `.styles.ts` files when component styling is complex

### Naming Conventions
- Components: `{name}.component.tsx`
- Routes: `{name}.route.tsx`
- Tests: `{name}.spec.tsx`
- Styles: `{name}.styles.ts`

### Architecture Decisions
- **Monorepo**: yarn workspaces with frontend/backend packages
- **State Management**: Tanstack Query ONLY (no Redux, Zustand, or Context for server state)
- **Validation**: Zod for all form inputs and Tauri command parameters
- **Error Handling**: Centralized error boundaries in React, Result<T, E> in Rust
- **Styling**: Tailwind CSS v4 utility classes (no CSS modules or styled-components)

### Anti-Patterns to Avoid

**Frontend:**
- ❌ Don't use class components (functional only)
- ❌ Don't inline styles (use Tailwind)
- ❌ Don't skip Zod validation on forms
- ❌ Don't use global state for server data (use Tanstack Query)
- ❌ Don't forget error boundaries around user interactions

**Backend:**
- ❌ Don't use blocking I/O in async functions (use tokio async APIs)
- ❌ Don't panic in Tauri commands (return Result with proper errors)
- ❌ Don't skip input validation on commands
- ❌ Don't use unwrap() in production code (handle errors properly)
- ❌ Don't forget to add #[tokio::test] for async tests

**Integration:**
- ❌ Don't bypass type safety at Tauri boundary
- ❌ Don't share mutable state across Tauri commands (use proper async patterns)
- ❌ Don't forget to validate data from frontend in Rust

---

## AI Workspace (`.ai/` directory)

**IMPORTANT**: Read `.ai/memory/` files before starting work.

### Quick Reference
- **`.ai/memory/decisions.md`** - Architecture decisions (READ FIRST)
- **`.ai/memory/patterns.md`** - Code patterns (READ FIRST)
- **`.ai/tasks/current-prd.md`** - Current work
- **`.ai/sessions/`** - Ralph loop logs

### When to Update
- **After decisions**: Add to `.ai/memory/decisions.md`
- **After establishing patterns**: Update `.ai/memory/patterns.md`
- **When learning something important**: Document in `.ai/memory/`

---

## MCP Server Usage

### When to Use Each MCP

**Filesystem MCP** (Always available)
- Reading/writing project files
- Searching codebase
- File operations

**Git MCP** (For history/changes)
- Check recent commits: "What changed in the last 3 commits?"
- View diffs: "Show me the diff for character-card.tsx"
- Branch info: "What branch am I on?"
- Commit history: "Who last modified this file?"

**GitHub MCP** (For remote/collaboration)
- Issues: "What open issues mention 'export'?"
- PRs: "Show me recent pull requests"
- Discussions: "Search discussions for architecture decisions"

**Memory MCP** (For project context)
- Store decisions: "Remember: we use Zod for all form validation"
- Recall patterns: "What's our naming convention for components?"
- Track conventions: "Store: all Tauri commands must have Rust tests"

**Sequential Thinking MCP** (For complex problems)
- Architecture decisions: "Design the character export feature"
- Debugging: "Analyze why tests are failing"
- Refactoring: "Plan refactoring character-list to use composition"

**Kagi MCP** (For external research)
- API docs: "Search for Tauri 2.8 window management examples"
- Best practices: "Find Rust async patterns for file watching"
- Troubleshooting: "Search for Tanstack Router lazy loading issues"

### MCP Best Practices

1. **Start with Memory**: Ask Memory MCP about project conventions before writing code
2. **Use Git for Context**: Check recent changes before making modifications
3. **Research with Kagi**: Look up unfamiliar APIs or patterns
4. **Think Sequentially**: Use Sequential Thinking for multi-step features
5. **Store Decisions**: Always save important decisions to Memory MCP

---

## Request Patterns (How to Ask)

### Refactoring a Component
"Refactor `{component-name}` to follow our standards: use functional patterns, add Zod validation, implement error boundaries, and write co-located tests"

### Adding a Tauri Command
"Create a Tauri command for `{feature}`: implement Rust handler in `backend/src/commands/`, register in `main.rs`, generate TypeScript types, and create a React hook with Tanstack Query"

### Building a Full-Stack Feature
"Build `{feature}` full-stack: plan the architecture, implement Rust backend with tests, create React components, and add integration tests"

### Reviewing Test Coverage
"Audit test coverage: identify untested files, run test suite with `yarn test`, and report gaps"

### Checking Accessibility
"Audit accessibility for `{component}`: check ARIA labels, test keyboard navigation, verify color contrast, and generate a report"

### Debugging an Issue
"Debug `{issue}`: check Git history for recent changes, review related code, identify the root cause, and propose a fix with tests"

---

## Development Workflow

### Before Starting Work
1. Check Memory MCP for project conventions
2. Review Git history for recent changes to related files
3. Search codebase for similar implementations

### During Implementation
1. Frontend changes: Update components in `packages/frontend/src/components/`
2. Backend changes: Add Tauri commands in `packages/backend/src/commands/`
3. Keep tests co-located with source files
4. Run `yarn lint` and `yarn format` before committing

### After Implementation
1. Run full test suite: `yarn test`
2. Check type safety: frontend builds without errors
3. Test Rust compilation: `yarn check:backend`
4. Store important decisions in Memory MCP

### Common Tasks
- **Lint**: `yarn lint` (runs both TS and Rust linters)
- **Format**: `yarn format` (formats TS and Rust code)
- **Test**: `yarn test` (runs all tests)
- **Dev**: `yarn dev` (starts Tauri dev server)
- **Build**: `yarn build` (production build)
