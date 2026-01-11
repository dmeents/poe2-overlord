# PRD: Backend Testing - Rust/Tauri

## Goal
Investigate the Rust backend, review existing tests, improve/fix them, and create comprehensive unit tests for files that need them.

## Discovery Phase
1. Explore `packages/backend/src/` structure
2. Identify all Rust modules and their purpose
3. Find existing test files (look for `#[cfg(test)]`, `#[test]`, `#[tokio::test]`)
4. Identify files that should have tests but don't
5. Review existing tests for quality and completeness
6. Create prioritized list based on:
   - Domain importance (character, game_monitoring, economy, etc.)
   - Missing coverage
   - Test quality issues

## Testing Standards
Follow Rust best practices:

### Test Organization
- Unit tests: inline with `#[cfg(test)]` mod tests
- Integration tests: in `tests/` directory at crate root
- Test naming: `test_<function>_<scenario>_<expected_result>`

### Test Quality
- **Arrange-Act-Assert** pattern
- Test one thing per test
- Use descriptive test names
- Mock external dependencies (database, file system, Tauri commands)
- Test both happy path and error cases
- Use `#[tokio::test]` for async tests

### Coverage Areas
For EACH module, ensure tests cover:
- ✅ Public API functions
- ✅ Edge cases (empty input, invalid data, boundaries)
- ✅ Error handling paths
- ✅ State changes and side effects
- ✅ Integration points (repository, service layers)

## Implementation Loop
**For EACH file/module**:

1. **Read** the module code to understand functionality
2. **Review** existing tests (if any):
   - Are they comprehensive?
   - Do they follow best practices?
   - Are there gaps in coverage?
3. **Improve/Fix** existing tests:
   - Fix failing tests
   - Refactor poorly written tests
   - Add missing assertions
   - Improve test clarity
4. **Create** new tests for untested functionality
5. **Run** `cargo test` - must pass
6. **Run** `cargo clippy` - fix any warnings
7. **Commit**: `"test: improve/add tests for [module_name]"`
8. Continue to next file

**Every 10 modules** (checkpoint):
- Push commits: `git push origin HEAD`
- Update `.ai/sessions/2026-01-11-backend-testing.md` with:
  - Modules completed
  - Issues found and fixed
  - Test coverage improvements
  - Current progress count
- Commit session log: `"docs: update backend testing session (checkpoint)"`
- Push session log

## Self-Healing
- Tests fail → debug, fix test or code, retry
- Clippy warnings → fix warnings, retry
- Compilation errors → fix errors, retry
- Stuck after 3 attempts → document in commit, skip file, continue
- Let the loop refine the work

## Success Criteria
- All existing tests pass (`cargo test` succeeds)
- All modules have appropriate test coverage
- Tests follow Rust best practices
- No clippy warnings in test code
- Each module committed separately with clear message

## Session Documentation
Maintain `.ai/sessions/2026-01-11-backend-testing.md` with:
- Modules tested (list)
- Issues found per module
- Test improvements made
- New tests added
- Coverage gaps identified
- Final test count and pass rate

## Completion Signal
When complete:
1. Final `cargo test` - all passing
2. Update session log with summary stats
3. Update `.ai/memory/patterns.md` with Rust testing patterns
4. Archive this PRD to `.ai/archive/completed-prds/`
5. Push all commits
6. Output `<promise>BACKEND_TESTS_COMPLETE</promise>`

## Notes
- Focus on quality over quantity
- Prioritize domain logic (character, economy, game_monitoring) over infrastructure
- Mock Tauri commands appropriately - don't require running app
- Use `cargo test --lib` to run only unit tests if faster
- Reference existing test patterns in codebase as examples
