# PRD: Quick Wins Batch - Deferred Issues

## Context
This PRD addresses **6 out of 61** deferred issues from the domain refactoring session (2026-01-11). These issues are documented in `.ai/tasks/deferred-issues.md` and were deferred because they require architectural changes or larger refactors.

**This batch**: Issues #6, #7, #15, #21, #25, #26 (Quick Wins)
**Prerequisites**: None (first batch)
**Remaining after this**: 55 issues

## Issues in This Batch

### Issue #6: Provider Dependency Documentation
**Severity**: CRITICAL (Architecture)
**Domain**: UI Foundation
**Type**: Documentation + Architecture

### Issue #7: QueryClient Provider Access
**Severity**: CRITICAL (Architecture)
**Domain**: UI Foundation
**Type**: Documentation

### Issue #26: Accordion Accessibility
**Severity**: HIGH (Accessibility)
**Domain**: UI Foundation
**Type**: Accessibility

### Issue #15: Frontend Deletion Event Handling
**Severity**: HIGH (Functional Gap)
**Domain**: Character Management
**Type**: Frontend Event Handling

### Issue #25: Tooltip Scroll Repositioning Verification
**Severity**: HIGH (UX Bug)
**Domain**: UI Foundation
**Type**: UX Bug Fix

### Issue #21: Tertiary Currency Selection
**Severity**: HIGH (Logic Bug)
**Domain**: Economy System
**Type**: Logic Bug

---

## Implementation Loop

**For EACH issue (6 total)**:

### Step 1: Plan
1. Read issue from `.ai/tasks/deferred-issues.md`
2. Invoke `@implementation-planner`:
   ```
   @implementation-planner "Create implementation plan for Issue #[N]: [Title]. 
   Investigate codebase, identify affected files, create detailed plan."
   ```
3. Review plan - let planner discover files through investigation

### Step 2: Implement
1. Follow implementation plan step-by-step
2. Make code changes (frontend and/or backend)
3. Add/update tests per plan
4. Run tests: `yarn test` or `cargo test`
5. Run linters: `yarn lint && yarn format` or `cargo clippy`

### Step 3: Verify
1. All tests pass
2. No linter errors
3. Manual test critical paths if needed

### Step 4: Commit
1. Commit: `"fix: [issue title] (Issue #N)"`
2. Include: `Co-Authored-By: Warp <agent@warp.dev>`

### Step 5: Document
1. Update `.ai/sessions/[date]-quick-wins-batch.md`:
   - Mark issue complete
   - Note implementation approach
   - Document any gotchas

### Step 6: Continue
Move to next issue

---

## Checkpoints

**After Issue #26 (3 issues complete)**:
- Push commits: `git push origin HEAD`
- Update session log
- Commit session log: `"docs: checkpoint - 3 issues complete"`
- Output: `<promise>CHECKPOINT_3_COMPLETE</promise>`
- `AskUserQuestion "3 issues done (#6, #7, #26). Continue to next 3?"`

**After Issue #21 (all 6 complete)**:
- Push all commits
- Mark issues ✅ in `.ai/tasks/deferred-issues.md`
- Update counters (6 complete, 55 remaining)
- Final session log update
- Commit: `"docs: quick wins batch complete"`
- Output: `<promise>QUICK_WINS_COMPLETE</promise>`

---

## Self-Healing

**Tests fail**:
- Analyze failure, fix code or tests
- Iterate up to 3 times per issue
- If stuck: document in session log, move to next

**Plan needs adjustment**:
- Adapt as needed - plans are guides
- Document deviations in commit message

**Uncertain about approach**:
- Re-invoke `@implementation-planner` with specific question
- Or use codebase patterns from `.ai/memory/patterns.md`

**Key principle**: Keep moving forward, document issues for later

---

## Success Criteria

- [ ] All 6 issues fixed
- [ ] All tests passing (517 frontend, 423 backend)
- [ ] No new linter errors
- [ ] Documentation updated (providers.tsx JSDoc, session log)
- [ ] Each issue committed separately
- [ ] Session log documents all changes

---

## Important Context for Ralph

**Subagent Usage**:
- ALWAYS invoke `@implementation-planner` before implementing
- Don't skip planning phase even if issue seems simple
- Planner will discover files and dependencies

**Testing Strategy**:
- Frontend tests: Run `yarn test` from project root
- Backend tests: Run `cargo test` from project root  
- Both test suites must pass before moving to next issue

**Commit Strategy**:
- One commit per issue (not per file)
- Include co-author line in every commit
- Push after every 3 commits (checkpoint)

**Issue Dependencies**:
- All 6 issues are independent
- Issue #15 frontend-only (backend Issue #14 is separate)
- Issue #25 verifies existing fix
- Can be done in listed order: #6, #7, #26, #15, #25, #21

**When Stuck**:
- Try up to 3 times per issue
- Document problem in session log
- Mark issue for manual review
- Move to next issue (don't block on one)

## Project Context

**Tech Stack**:
- Frontend: React 19, TypeScript, TanStack Query, Tailwind CSS
- Backend: Rust, Tauri 2.x, serde, tokio
- Testing: Vitest + RTL (frontend), cargo test (backend)

**Patterns** (check `.ai/memory/patterns.md`):
- Frontend: Component + styles + spec pattern
- Backend: Traits + service + repository pattern
- Validation: Zod (frontend), custom validation (backend)

**Key Files** (for reference only):
- Frontend providers: `packages/frontend/src/providers.tsx`
- UI components: `packages/frontend/src/components/ui/*`
- Character context: `packages/frontend/src/contexts/CharacterContext.tsx`
- Economy models: `packages/backend/src/domain/economy/models.rs`

---

## Ralph Command

```bash
/ralph-wiggum:ralph-loop "Follow .ai/tasks/prd-quick-wins.md to fix 6 Quick Wins issues. For each issue: invoke @implementation-planner, implement the plan, test, commit, update session log. After 3 issues output CHECKPOINT_3_COMPLETE and ask to continue. After all 6 output QUICK_WINS_COMPLETE." --max-iterations 150 --completion-promise "QUICK_WINS_COMPLETE"
```

## Final Completion

When `<promise>QUICK_WINS_COMPLETE</promise>` output:
1. All 6 issues fixed and committed
2. Session log complete with summaries
3. Issues marked ✅ in deferred-issues.md
4. Counters updated (6 complete, 55 remaining)
5. This PRD archived
6. All commits pushed to remote

---

## Issue Details Reference

See `.ai/tasks/deferred-issues.md` for:
- Full issue descriptions
- Current state analysis
- Complexity assessments

---

## Session Documentation

Create `.ai/sessions/[date]-quick-wins-batch.md` with:
- Issues completed (checklist)
- Implementation summaries
- Gotchas and learnings
- Test results
- Final stats
