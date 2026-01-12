# PRD: Refactors Batch 3 (Zone & Economy) - Deferred Issues

## Context
This PRD addresses **12 out of 61** deferred issues from the domain refactoring session (2026-01-11). These are code quality improvements for Zone Tracking and Economy domains.

**This batch**: Issues #17-20, #23, #42-51 (Zone + Economy refactors)
**Prerequisites**: Refactors Batch 2 complete
**Remaining after this**: 17 issues

## Issues in This Batch

### Zone Tracking Domain (6 issues)
- Issue #17: Zone metadata overwrite (HIGH → design decision)
- Issue #18: Zone type mismatch (HIGH → contract violation)
- Issue #19: Placeholder zone missing metadata (HIGH → functional gap)
- Issue #42: ZoneContext conflates modal state with zone selection
- Issue #43: Act breakdown excludes Act 5
- Issue #44: getDisplayAct inconsistent formats
- Issue #45: No test coverage for service layer
- Issue #46: Hardcoded hideout string duplicated

### Economy Domain (6 issues)
- Issue #20: Economy retry logic (HIGH → reliability)
- Issue #23: Layout height magic numbers (HIGH → maintainability)
- Issue #47: Manual EconomyType string parsing
- Issue #48: No TTL value overflow validation
- Issue #49: Empty currencies array not distinguished in UI
- Issue #50: Excessive query invalidation (performance)
- Issue #51: Missing error handling for image failures

---

## Implementation Loop

**For EACH issue (12 total)**:

### Step 1: Plan
1. Read issue from `.ai/tasks/deferred-issues.md`
2. Invoke `@implementation-planner`:
   ```
   @implementation-planner "Create implementation plan for Issue #[N]: [Title]."
   ```
3. Review plan

### Step 2: Implement
1. Follow plan to make code changes
2. Run tests: `yarn test` and/or `cargo test`
3. Run linters: `yarn lint && yarn format` and/or `cargo clippy`

### Step 3: Verify
1. All tests pass
2. No linter errors
3. Manual verification if needed (UI changes)

### Step 4: Commit
1. Commit: `"refactor([domain]): [issue title] (Issue #N)"`
2. Include: `Co-Authored-By: Warp <agent@warp.dev>`

### Step 5: Document
1. Update `.ai/sessions/[date]-refactors-3-batch.md`:
   - Mark issue complete
   - Note any implementation decisions

### Step 6: Continue
Move to next issue

---

## Checkpoints

**After Issue #46 (6 zone issues complete)**:
- Push commits: `git push origin HEAD`
- Update session log
- Output: `<promise>CHECKPOINT_ZONE</promise>`
- `AskUserQuestion "Zone tracking refactors complete (6/12). Continue with economy refactors?"`

**After Issue #51 (all 12 complete)**:
- Push commits: `git push origin HEAD`
- Mark issues ✅ in `.ai/tasks/deferred-issues.md`
- Update counters (44 complete, 17 remaining)
- Final session log update
- Commit: `"docs: refactors batch 3 complete"`
- Output: `<promise>REFACTORS_3_COMPLETE</promise>`

---

## Self-Healing

**Tests failing**:
- Review test expectations
- Check if refactor changed behavior
- Update tests if needed
- Iterate up to 3 times

**UI changes not appearing**:
- Check React component rendering
- Verify state updates
- Test with browser devtools

**Linter errors**:
- Apply suggested fixes
- Check code style consistency

**Key principle**: Code quality improvements should not change functionality.

---

## Success Criteria

- [ ] All 12 issues fixed
- [ ] All tests passing (517 frontend, 423 backend)
- [ ] No linter warnings
- [ ] Functionality unchanged (refactors only)
- [ ] Code more maintainable

---

## Important Context for Ralph

**Subagent Usage**:
- Invoke `@implementation-planner` for each issue
- Focus on preserving existing behavior
- Ask about design decisions (Issue #17)

**Testing Strategy**:
- Existing tests should still pass
- Add tests for uncovered areas (Issue #45)
- Manual testing for UI changes
- Performance check (Issue #50)

**Commit Strategy**:
- One commit per issue
- Use `refactor([domain]):` prefix
- Push at checkpoints

**Issue Dependencies**:
- Issue #17 needs design review (keep metadata or replace?)
- Issue #18 may need type contract clarification
- All others independent

**When Stuck**:
- Check domain patterns in `.ai/memory/patterns.md`
- Look at similar refactors in git history
- Ask about trade-offs if multiple approaches

## Project Context

**Tech Stack**:
- Frontend: React 19, TypeScript, TanStack Query
- Backend: Rust, Tauri 2.x
- Zone: File-based tracking with enrichment
- Economy: HTTP API with caching

**Domain Patterns**:
- Zone enrichment with wiki metadata
- Economy caching with TTL
- TanStack Query for data fetching
- React Context for state management

**Key Files** (for reference only):
- Zone: `packages/backend/src/domain/zone_tracking/*`
- Zone: `packages/frontend/src/contexts/ZoneContext.tsx`
- Economy: `packages/backend/src/domain/economy/*`
- Economy: `packages/frontend/src/hooks/use-economy-data.ts`

---

## Ralph Command

```bash
/ralph-wiggum:ralph-loop "Follow .ai/tasks/prd-refactors-3-zone-economy.md to refactor 12 issues (zone + economy domains). For each: invoke @implementation-planner, implement, test, commit. Checkpoint after zone issues, output REFACTORS_3_COMPLETE when all done." --max-iterations 350 --completion-promise "REFACTORS_3_COMPLETE"
```

## Final Completion

When `<promise>REFACTORS_3_COMPLETE</promise>` output:
1. All 12 issues complete
2. Code quality improved
3. All commits pushed
4. Issues marked ✅ in deferred-issues.md
5. Counters updated (44 complete, 17 remaining)
6. This PRD archived

---

## Issue Details Reference

See `.ai/tasks/deferred-issues.md` for:
- Full issue descriptions
- Current state analysis
- Complexity assessments

---

## Session Documentation

Create `.ai/sessions/[date]-refactors-3-batch.md` with:
- Issues completed (checklist)
- Implementation approaches
- Design decisions (Issue #17)
- Testing results
- Gotchas and learnings
