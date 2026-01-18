# PRD: Refactors Batch 2 (Monitoring & Character) - Deferred Issues

## Context

This PRD addresses **8 out of 61** deferred issues from the domain refactoring session (2026-01-11). These are code quality improvements for Monitoring and Character domains.

**This batch**: Issues #11, #13, #35-41 (Monitoring + Character refactors)
**Prerequisites**: Refactors Batch 1 complete
**Remaining after this**: 29 issues

## Issues in This Batch

### Monitoring Domain (3 issues)

- Issue #11: Character error propagation (HIGH → error handling)
- Issue #35: IP address validation (already exists in is_valid())
- Issue #36: SystemTime serialization

### Character Domain (5 issues)

- Issue #13: Inefficient name uniqueness check (HIGH → performance)
- Issue #37: Inconsistent error handling in repository
- Issue #38: Inefficient character enrichment (sequential zone metadata calls)
- Issue #39: Default CharacterData has empty ID
- Issue #40: Missing bounds check on level in update_character_level
- Issue #41: Hardcoded hideout detection logic

---

## Implementation Loop

**For EACH issue (8 total)**:

### Step 1: Plan

1. Read issue from `.ai/tasks/deferred-issues.md`
2. Invoke `@implementation-planner`:
   ```
   @implementation-planner "Create implementation plan for Issue #[N]: [Title]."
   ```
3. Review plan

### Step 2: Implement

1. Follow plan to make code changes
2. Run tests: `cargo test` (backend only)
3. Run linters: `cargo clippy`

### Step 3: Verify

1. All tests pass
2. No linter errors
3. Performance check if applicable (Issue #13, #38)

### Step 4: Commit

1. Commit: `"refactor([domain]): [issue title] (Issue #N)"`
2. Include: `Co-Authored-By: Warp <agent@warp.dev>`

### Step 5: Document

1. Update `.ai/sessions/[date]-refactors-2-batch.md`:
   - Mark issue complete
   - Note any implementation decisions
   - Document performance improvements

### Step 6: Continue

Move to next issue

---

## Checkpoints

**After Issue #36 (3 monitoring issues complete)**:

- Push commits: `git push origin HEAD`
- Update session log
- Output: `<promise>CHECKPOINT_MONITORING</promise>`
- `AskUserQuestion "Monitoring domain refactors complete (3/8). Continue with character refactors?"`

**After Issue #41 (all 8 complete)**:

- Push commits: `git push origin HEAD`
- Mark issues ✅ in `.ai/tasks/deferred-issues.md`
- Update counters (32 complete, 29 remaining)
- Final session log update
- Commit: `"docs: refactors batch 2 complete"`
- Output: `<promise>REFACTORS_2_COMPLETE</promise>`

---

## Self-Healing

**Tests failing**:

- Review test expectations
- Check if refactor changed behavior
- Update tests if needed
- Iterate up to 3 times

**Performance regression**:

- Benchmark before/after (Issue #13, #38)
- Verify optimization works as expected
- Check with large character counts

**Linter errors**:

- Apply suggested fixes
- Check Rust idioms

**Key principle**: Code quality improvements should not change functionality.

---

## Success Criteria

- [ ] All 8 issues fixed
- [ ] All tests passing (423 backend tests)
- [ ] No linter warnings
- [ ] Functionality unchanged (refactors only)
- [ ] Performance improved (Issue #13, #38)

---

## Important Context for Ralph

**Subagent Usage**:

- Invoke `@implementation-planner` for each issue
- Focus on preserving existing behavior
- Ask about performance optimization strategies

**Testing Strategy**:

- Existing tests should still pass
- Add performance benchmarks for #13, #38
- Manual testing for character operations

**Commit Strategy**:

- One commit per issue
- Use `refactor([domain]):` prefix
- Push at checkpoints

**Issue Dependencies**:

- All issues independent within batch
- Issue #13 may require index refactor (check deferred-issues.md)

**When Stuck**:

- Check domain patterns in `.ai/memory/patterns.md`
- Look at Rust optimization patterns
- Ask about trade-offs for performance vs complexity

## Project Context

**Tech Stack**:

- Backend: Rust, Tauri 2.x
- Character: File-based storage with index
- Monitoring: Network detection, game state tracking

**Domain Patterns**:

- Repository → Service → API command
- Error handling with `Result<T, E>`
- Async/await for IO operations

**Key Files** (for reference only):

- Monitoring: `packages/backend/src/domain/game_monitoring/*`
- Character: `packages/backend/src/domain/character/*`

---

## Ralph Command

```bash
/ralph-loop:ralph-loop "Follow .ai/tasks/prd-refactors-2-monitoring-character.md to refactor 8 issues (monitoring + character domains). For each: invoke @implementation-planner, implement, test, commit. Checkpoint after monitoring issues, output REFACTORS_2_COMPLETE when all done." --max-iterations 250 --completion-promise "REFACTORS_2_COMPLETE"
```

## Final Completion

When `<promise>REFACTORS_2_COMPLETE</promise>` output:

1. All 8 issues complete
2. Code quality improved
3. Performance optimizations verified
4. All commits pushed
5. Issues marked ✅ in deferred-issues.md
6. Counters updated (32 complete, 29 remaining)
7. This PRD archived

---

## Issue Details Reference

See `.ai/tasks/deferred-issues.md` for:

- Full issue descriptions
- Current state analysis
- Complexity assessments

---

## Session Documentation

Create `.ai/sessions/[date]-refactors-2-batch.md` with:

- Issues completed (checklist)
- Implementation approaches
- Performance improvements (before/after)
- Testing results
- Gotchas and learnings
