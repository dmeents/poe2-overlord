# PRD: Comprehensive TypeScript Refactoring

## Goal
Refactor the frontend codebase to maximize TypeScript type safety, eliminate type escape hatches, and ensure optimal TS utilization.

## Discovery Phase
1. Scan all files in `packages/frontend/src/` for type issues
2. Identify files with `any`, `as any`, `@ts-ignore`, `@ts-expect-error`
3. Find missing return types, implicit any, type assertions
4. Create priority list based on impact

## Refactoring Checklist
**For EACH file**:

### 1. Eliminate Type Escape Hatches
- Replace `any` with proper types or `unknown`
- Replace `as any` with type guards or proper types
- Remove `@ts-ignore`/`@ts-expect-error` - fix root cause
- Replace unsafe `as` casts with type narrowing

### 2. Strengthen Type Definitions
- Add explicit return types to all exported functions
- Use `unknown` instead of `any` for uncertain inputs
- Use `satisfies` for type validation without widening
- Add `readonly` to immutable props/data
- Use `const` assertions for literal types

### 3. Improve Null Safety
- Use optional chaining (`?.`) consistently
- Use nullish coalescing (`??`) over `||`
- Add explicit null checks before property access
- Use discriminated unions for nullable complex types

### 4. Optimize Imports
- Use `import type` for type-only imports (better tree-shaking)
- Separate type imports from value imports
- Example: `import type { Foo } from './types';`

### 5. Function Signatures
- Explicit return types on exports (no inference)
- Use `void` explicitly for no-return functions
- Type event handlers: `React.MouseEvent<HTMLButtonElement>`
- Named params object for >3 parameters

### 6. React-Specific
- Avoid `React.FC` unless needed
- Type props explicitly with destructuring
- Extend HTML elements: `React.ComponentPropsWithoutRef<'button'>`
- Type hooks explicitly: `useState<User | null>(null)`

## Implementation Loop
**For EACH file**:
1. Read file and identify type issues
2. Apply fixes from checklist
3. Run `yarn typecheck` - fix any new errors
4. Run `yarn test` - ensure tests pass
5. Run `yarn lint && yarn format`
6. Commit: `"refactor(types): improve type safety in [filename]"`
7. Continue to next file

**Every 10 files** (checkpoint):
- Push commits to remote: `git push origin HEAD`
- Update `.ai/sessions/2026-01-10-typescript-refactoring.md` with:
  - Files completed so far
  - Issues found and fixed
  - Current progress count
- Commit session log: `"docs: update refactoring session log (checkpoint)"`
- Push session log update

## Self-Healing
- Typecheck fails → analyze error, fix properly (no suppressions)
- Tests break → update tests to match improved types
- Stuck after 3 attempts → document issue, skip file, continue
- Let the loop refine the work

## Success Criteria
- Zero `any` types (except test mocks if necessary)
- Zero `@ts-ignore`/`@ts-expect-error` suppressions
- All exports have explicit return types
- Type-only imports use `import type`
- `yarn typecheck` passes with zero errors
- All 517 tests still pass

## Priority Order
1. **`src/types/`** - Core type definitions
2. **`src/hooks/`** - 4 files found with `any`
3. **`src/utils/`** - Utility functions
4. **`src/components/`** - All components systematically

## Anti-Patterns to Eliminate
❌ `any` → use `unknown` and narrow
❌ `as SomeType` → use type guards
❌ `@ts-ignore` → fix root cause
❌ Implicit return types → add explicit types
❌ Mixed imports → separate type/value imports
❌ `!` non-null assertion → add proper checks

## Best Practices to Follow
✅ `unknown` for uncertain types, then narrow with guards
✅ `satisfies` to validate without widening
✅ `readonly` for immutable data
✅ `import type` for type-only imports
✅ Explicit return types on exports
✅ Optional chaining and nullish coalescing
✅ Type guards for runtime validation

## Session Documentation
Maintain `.ai/sessions/2026-01-10-typescript-refactoring.md` with:
- List of files refactored
- Type issues found per file
- Patterns discovered
- Breaking changes (if any)
- Final stats (any removed, types added, etc.)

## Completion Signal
When complete:
1. Update `.ai/sessions/2026-01-10-typescript-refactoring.md` with final summary
2. Update `.ai/memory/patterns.md` with TypeScript patterns learned
3. Archive this PRD to `.ai/archive/completed-prds/`
4. Push all commits to remote
5. Output `<promise>REFACTOR_COMPLETE</promise>`
