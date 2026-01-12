# PRD: Refactors Batch 4 (Walkthrough & UI) - Deferred Issues

## Context
This PRD addresses **9 out of 61** deferred issues from the domain refactoring session (2026-01-11). These are code quality improvements for Walkthrough and UI Foundation domains.

**This batch**: Issues #24, #52-59 (Walkthrough + UI refactors)
**Prerequisites**: Refactors Batch 3 complete
**Remaining after this**: 8 issues

## Issues in This Batch

### Walkthrough Domain (4 issues)
- Issue #52: Incomplete test coverage (service tests)
- Issue #53: Parameter naming inconsistency (characterId vs character_id)
- Issue #54: No bounds checking on step IDs (circular reference risk)
- Issue #55: Conditional hook calls violate Rules of Hooks

### UI Foundation Domain (5 issues)
- Issue #24: Modal scroll lock memory leak (HIGH → needs global modal manager)
- Issue #56: Button no loading state
- Issue #57: Sidebar missing active link announcement
- Issue #58: Error state unsafe type coercion
- Issue #59: Time display inconsistent rounding

---

## Implementation Loop

**For EACH issue (9 total)**:

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
3. Manual verification (UI changes, accessibility)

### Step 4: Commit
1. Commit: `"refactor([domain]): [issue title] (Issue #N)"`
2. Include: `Co-Authored-By: Warp <agent@warp.dev>`

### Step 5: Document
1. Update `.ai/sessions/[date]-refactors-4-batch.md`:
   - Mark issue complete
   - Note any implementation decisions

### Step 6: Continue
Move to next issue

---

## Checkpoints

**After Issue #55 (4 walkthrough issues complete)**:
- Push commits: `git push origin HEAD`
- Update session log
- Output: `<promise>CHECKPOINT_WALKTHROUGH</promise>`
- `AskUserQuestion "Walkthrough refactors complete (4/9). Continue with UI refactors?"`

**After Issue #59 (all 9 complete)**:
- Push commits: `git push origin HEAD`
- Mark issues ✅ in `.ai/tasks/deferred-issues.md`
- Update counters (53 complete, 8 remaining)
- Final session log update
- Commit: `"docs: refactors batch 4 complete"`
- Output: `<promise>REFACTORS_4_COMPLETE</promise>`

---

## Self-Healing

**Tests failing**:
- Review test expectations
- Check if refactor changed behavior
- Update tests if needed
- Iterate up to 3 times

**React Rules of Hooks violation**:
- Move hooks to top level
- Use early returns after all hooks
- Check with ESLint

**Memory leaks (Issue #24)**:
- Verify cleanup in useEffect returns
- Check reference counting
- Test with React DevTools Profiler

**Accessibility issues**:
- Test with screen reader
- Check ARIA attributes
- Verify keyboard navigation

**Key principle**: Code quality improvements should not change functionality.

---

## Success Criteria

- [ ] All 9 issues fixed
- [ ] All tests passing (517 frontend, 423 backend)
- [ ] No linter warnings
- [ ] No React warnings (hooks, memory leaks)
- [ ] Accessibility improved (Issue #57)
- [ ] Functionality unchanged (refactors only)

---

## Important Context for Ralph

**Subagent Usage**:
- Invoke `@implementation-planner` for each issue
- Focus on preserving existing behavior
- Ask about React patterns (Issue #55)
- Ask about accessibility best practices (Issue #57)

**Testing Strategy**:
- Add service tests (Issue #52)
- Existing tests should still pass
- Manual testing for UI changes
- Accessibility testing with screen reader

**Commit Strategy**:
- One commit per issue
- Use `refactor([domain]):` prefix
- Push at checkpoints

**Issue Dependencies**:
- Issue #24 may need global modal manager (check if feasible)
- Issue #55 critical - violates React rules
- All others independent

**When Stuck**:
- Check React patterns in `.ai/memory/patterns.md`
- Look at React docs for Rules of Hooks
- Ask about accessibility standards (WCAG)

## Project Context

**Tech Stack**:
- Frontend: React 19, TypeScript, TanStack Router
- UI: Custom components with accessibility
- Walkthrough: Guide system with step progression

**Domain Patterns**:
- React hooks (must follow Rules of Hooks)
- ARIA attributes for accessibility
- Modal management patterns
- Error boundary patterns

**Key Files** (for reference only):
- Walkthrough: `packages/backend/src/domain/walkthrough/*`
- Walkthrough: `packages/frontend/src/contexts/WalkthroughContext.tsx`
- UI: `packages/frontend/src/components/ui/*`
- Sidebar: `packages/frontend/src/components/layout/sidebar.tsx`

---

## Ralph Command

```bash
/ralph-wiggum:ralph-loop "Follow .ai/tasks/prd-refactors-4-walkthrough-ui.md to refactor 9 issues (walkthrough + ui domains). For each: invoke @implementation-planner, implement, test, commit. Checkpoint after walkthrough issues, output REFACTORS_4_COMPLETE when all done." --max-iterations 300 --completion-promise "REFACTORS_4_COMPLETE"
```

## Final Completion

When `<promise>REFACTORS_4_COMPLETE</promise>` output:
1. All 9 issues complete
2. Code quality improved
3. React violations fixed
4. Accessibility improved
5. All commits pushed
6. Issues marked ✅ in deferred-issues.md
7. Counters updated (53 complete, 8 remaining)
8. This PRD archived

---

## Issue Details Reference

See `.ai/tasks/deferred-issues.md` for:
- Full issue descriptions
- Current state analysis
- Complexity assessments

---

## Session Documentation

Create `.ai/sessions/[date]-refactors-4-batch.md` with:
- Issues completed (checklist)
- Implementation approaches
- React patterns applied
- Accessibility improvements
- Testing results
- Gotchas and learnings
