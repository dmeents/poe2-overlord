# PRD: Comprehensive Frontend Component Testing

## Goal
Write comprehensive unit tests for all frontend components in `packages/frontend/src/components/` that don't already have tests.

## Discovery Phase
1. Find all `*.tsx` files in `packages/frontend/src/components/`
2. Exclude files that already have corresponding `*.spec.tsx` tests
3. Create a list of components needing tests

## Implementation Loop
For EACH untested component:
1. Read the component to understand props, behavior, interactions
2. Reference `.ai/memory/patterns.md` for testing patterns
3. Reference `packages/frontend/src/components/ui/button/button.spec.tsx` as example
4. Write comprehensive `*.spec.tsx` test covering:
   - Renders without crashing
   - Props handled correctly
   - User interactions work
   - Conditional rendering
   - Event handlers
   - Edge cases
5. Run `yarn test` - if fails, debug and fix until passing
6. Run `yarn format` and `yarn lint` - fix any issues
7. Commit: `"test: add unit tests for [ComponentName]"`
8. Continue to next component

## Self-Healing
- If tests fail: analyze error, fix test or component, retry
- If lint fails: run formatter, fix issues, retry
- If stuck on one component after 5 attempts: document issue in commit message, move to next
- Let the loop refine the work - don't aim for perfect first try

## Success Criteria
- Every `.tsx` component file has a corresponding `.spec.tsx` test file
- All tests pass (`yarn test` succeeds)
- No lint/format errors
- Each component committed separately

## Completion Signal
Output `<promise>ALL_TESTS_COMPLETE</promise>` when no untested components remain

## Tech Stack
- Vitest (test runner)
- React Testing Library (component testing)
- @testing-library/user-event (user interactions)
- Follow patterns from packages/frontend/src/components/ui/button/button.spec.tsx
