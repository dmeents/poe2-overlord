# PRD: Refactors Batch 1 (Config & Wiki) - Deferred Issues

## Context
This PRD addresses **8 out of 61** deferred issues from the domain refactoring session (2026-01-11). These are code quality improvements for Configuration and Wiki Scraping domains.

**This batch**: Issues #9, #10, #27-34 (Configuration + Wiki refactors)
**Prerequisites**: Quick Wins + Data Integrity + Event System + Real-Time batches complete
**Remaining after this**: 37 issues

## Issues in This Batch

### Configuration Domain (5 issues)
- Issue #27: Disk write debouncing
- Issue #28: Event publishing on load failure  
- Issue #29: Unused repository methods cleanup
- Issue #30: Double backend call on reset
- Issue #31: Error handling pattern consistency

### Wiki Scraping Domain (3 issues)
- Issue #9: Wiki section parsing brittleness (HIGH → refactor)
- Issue #10: Connected zones wiki redirects (HIGH → edge case)
- Issue #32: URL encoding for special characters
- Issue #33: Case-sensitive redirect detection
- Issue #34: Timeout configuration flexibility

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
2. Run tests: `yarn test` / `cargo test`
3. Run linters: `yarn lint && yarn format` / `cargo clippy`

### Step 3: Verify
1. All tests pass
2. No linter errors
3. Manual verification if needed

### Step 4: Commit
1. Commit: `"refactor([domain]): [issue title] (Issue #N)"`
2. Include: `Co-Authored-By: Warp <agent@warp.dev>`

### Step 5: Document
1. Update `.ai/sessions/[date]-refactors-1-batch.md`:
   - Mark issue complete
   - Note any implementation decisions

### Step 6: Continue
Move to next issue

---

## Checkpoints

**After Issue #30 (4 config issues complete)**:
- Push commits: `git push origin HEAD`
- Update session log
- Output: `<promise>CHECKPOINT_CONFIG</promise>`
- `AskUserQuestion "Config domain refactors complete (4/8). Continue with wiki refactors?"`

**After Issue #34 (all 8 complete)**:
- Push commits: `git push origin HEAD`
- Mark issues ✅ in `.ai/tasks/deferred-issues.md`
- Update counters (24 complete, 37 remaining)
- Final session log update
- Commit: `"docs: refactors batch 1 complete"`
- Output: `<promise>REFACTORS_1_COMPLETE</promise>`

---

## Self-Healing

**Tests failing**:
- Review test expectations
- Check if refactor changed behavior
- Update tests if needed
- Iterate up to 3 times

**Linter errors**:
- Apply suggested fixes
- Check code style consistency
- Run formatter

**Uncertain about approach**:
- Check existing patterns in domain
- Review domain model/service structure
- Invoke `@implementation-planner` again

**Key principle**: Code quality improvements should not change functionality.

---

## Success Criteria

- [ ] All 8 issues fixed
- [ ] All tests passing (517 frontend, 423 backend)
- [ ] No linter warnings
- [ ] Functionality unchanged (refactors only)
- [ ] Code more maintainable

---

## Important Context for Ralph

**Subagent Usage**:
- Invoke `@implementation-planner` for each issue
- Focus on preserving existing behavior

**Testing Strategy**:
- Existing tests should still pass
- Add tests only if coverage gap found
- Manual testing for wiki scraping changes

**Commit Strategy**:
- One commit per issue
- Use `refactor([domain]):` prefix
- Push at checkpoints

**Issue Dependencies**:
- All issues independent within batch
- Can be done in any order per domain

**When Stuck**:
- Check domain patterns in `.ai/memory/patterns.md`
- Look at similar refactors in git history
- Ask about trade-offs if multiple approaches

## Project Context

**Tech Stack**:
- Backend: Rust, Tauri 2.x
- Config: File-based storage with in-memory caching
- Wiki: HTTP client with HTML parsing

**Domain Patterns**:
- Repository → Service → API command
- Events published on state changes
- Error handling with `Result<T, E>`

**Key Files** (for reference only):
- Config: `packages/backend/src/domain/configuration/*`
- Wiki: `packages/backend/src/domain/wiki_scraping/*`

---

## Ralph Command

```bash
/ralph-wiggum:ralph-loop "Follow .ai/tasks/prd-refactors-1-config-wiki.md to refactor 8 issues (config + wiki domains). For each: invoke @implementation-planner, implement, test, commit. Checkpoint after config issues, output REFACTORS_1_COMPLETE when all done." --max-iterations 250 --completion-promise "REFACTORS_1_COMPLETE"
```

## Final Completion

When `<promise>REFACTORS_1_COMPLETE</promise>` output:
1. All 8 issues complete
2. Code quality improved
3. All commits pushed
4. Issues marked ✅ in deferred-issues.md
5. Counters updated (24 complete, 37 remaining)
6. This PRD archived

---

## Issue Details Reference

See `.ai/tasks/deferred-issues.md` for:
- Full issue descriptions
- Current state analysis
- Complexity assessments

---

## Session Documentation

Create `.ai/sessions/[date]-refactors-1-batch.md` with:
- Issues completed (checklist)
- Implementation approaches
- Testing results
- Gotchas and learnings
