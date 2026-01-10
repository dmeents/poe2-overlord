# PRD: Fix Hanging Unit Tests

## Goal
Fix all hanging/timing out unit tests in `packages/frontend/src/components/` so the test suite completes quickly.

## Discovery Phase
1. Run `yarn test --reporter=verbose` to identify hanging tests
2. Note which test files timeout or take excessively long (>5 seconds per test)
3. Create list of problematic test files

## Fix Strategy
For EACH hanging test:
1. Identify root cause:
   - Missing `await` on async operations (user.click, waitFor, findBy*)
   - Tauri commands not properly mocked
   - Timer/interval not cleaned up
   - React state updates after unmount
   - Missing `cleanup()` in test teardown
   - Infinite loops in component logic
   - Unclosed promises

2. Apply fixes:
   - Wrap all user interactions in `await`
   - Mock all Tauri `invoke()` calls with proper return values
   - Use `vi.useFakeTimers()` if testing timers/intervals
   - Add `afterEach(() => cleanup())` if missing
   - Use `waitFor()` with explicit timeout for async checks
   - Ensure all async operations complete before test ends

3. Verify fix:
   - Run specific test file: `yarn test [filename]`
   - Confirm test completes in reasonable time (<2s per test)
   - Ensure test still passes

4. Commit: `"fix: resolve hanging tests in [ComponentName]"`

## Self-Healing
- If fix doesn't work: try alternative approach
- If still hanging after 3 attempts: add `.skip()` to test, document issue, move to next
- Let the loop refine the work

## Success Criteria
- All tests complete within 10 seconds per test
- Full test suite (`yarn test`) completes without hanging
- All non-skipped tests pass
- Each fix committed separately

## Completion Signal
Output `<promise>TESTS_FIXED</promise>` when `yarn test` completes successfully without timeouts
