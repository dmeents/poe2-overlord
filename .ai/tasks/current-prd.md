# PRD: Pipeline Orchestrator - All Deferred Issues (61 Total)

## Context

This is the **master orchestrator PRD** that executes all 8 batch PRDs in priority order to systematically resolve all 61 deferred issues from the domain refactoring session (2026-01-11).

**Pipeline Overview**:

- 8 batch PRDs to execute sequentially
- 61 total issues across all batches
- Each batch has dependencies on previous batches
- Estimated total: 4-8 weeks of work

**Source of Truth**: `.ai/tasks/deferred-issues.md`

---

## Batch Execution Plan

### Batch 1: Quick Wins (6 issues)

**PRD**: `.ai/tasks/prd-quick-wins.md`
**Completion Promise**: `QUICK_WINS_COMPLETE`
**Prerequisites**: None (first batch)
**Remaining After**: 55 issues
**Max Iterations**: 150

### Batch 2: Data Integrity (7 issues)

**PRD**: `.ai/tasks/prd-data-integrity.md`
**Completion Promise**: `DATA_INTEGRITY_COMPLETE`
**Prerequisites**: Batch 1 complete
**Remaining After**: 48 issues
**Max Iterations**: 250

### Batch 3: Event System (2 issues)

**PRD**: `.ai/tasks/prd-event-system.md`
**Completion Promise**: `EVENT_SYSTEM_COMPLETE`
**Prerequisites**: Batch 1, 2 complete
**Remaining After**: 46 issues
**Max Iterations**: 100

### Batch 4: Real-Time Features (1 issue)

**PRD**: `.ai/tasks/prd-real-time-features.md`
**Completion Promise**: `REAL_TIME_COMPLETE`
**Prerequisites**: Batch 1, 2, 3 complete
**Remaining After**: 45 issues
**Max Iterations**: 60

### Batch 5: Refactors 1 - Config & Wiki (8 issues)

**PRD**: `.ai/tasks/prd-refactors-1-config-wiki.md`
**Completion Promise**: `REFACTORS_1_COMPLETE`
**Prerequisites**: Batch 1-4 complete
**Remaining After**: 37 issues
**Max Iterations**: 250

### Batch 6: Refactors 2 - Monitoring & Character (8 issues)

**PRD**: `.ai/tasks/prd-refactors-2-monitoring-character.md`
**Completion Promise**: `REFACTORS_2_COMPLETE`
**Prerequisites**: Batch 5 complete
**Remaining After**: 29 issues
**Max Iterations**: 250

### Batch 7: Refactors 3 - Zone & Economy (12 issues)

**PRD**: `.ai/tasks/prd-refactors-3-zone-economy.md`
**Completion Promise**: `REFACTORS_3_COMPLETE`
**Prerequisites**: Batch 6 complete
**Remaining After**: 17 issues
**Max Iterations**: 350

### Batch 8: Refactors 4 - Walkthrough & UI (9 issues)

**PRD**: `.ai/tasks/prd-refactors-4-walkthrough-ui.md`
**Completion Promise**: `REFACTORS_4_COMPLETE`
**Prerequisites**: Batch 7 complete
**Remaining After**: 8 issues
**Max Iterations**: 300

---

## Orchestration Loop

**For EACH batch (1-8)**:

### Step 1: Pre-Flight Check

1. Verify prerequisites complete (check master log)
2. Verify current branch clean (`git status`)
3. Verify tests passing baseline (517 frontend, 423 backend)
4. Read batch PRD file
5. Create batch session log: `.ai/sessions/[date]-[batch-name]-batch.md`

### Step 2: Execute Batch

1. Follow batch PRD instructions **exactly**
2. Use `@implementation-planner` for each issue per batch PRD
3. Implement, test, commit per batch PRD loop
4. Follow batch checkpoints (if any)
5. Continue until batch completion promise received

### Step 3: Verify Batch Completion

When `<promise>[BATCH]_COMPLETE</promise>` received:

1. **Verify issues marked**: All issues in batch marked ✅ in `deferred-issues.md`
2. **Verify counters**: Issue count updated correctly (e.g., 6/61 → 13/61)
3. **Verify commits**: All commits for batch pushed to remote
4. **Verify tests**: Tests still passing (517 frontend, 423 backend)
5. **Verify session log**: Batch session log complete and committed

### Step 4: Update Master State

1. Update master log: `.ai/sessions/[date]-pipeline-execution.md`:
   - Mark batch complete ✅
   - Note issues completed
   - Note any problems or deviations
   - Update overall progress counter
2. Archive batch PRD: Move to `.ai/archive/completed-prds/[batch-name].md`
3. Commit master log: `"docs: batch [N] complete - [X]/61 issues done"`
4. Push all changes

### Step 5: Checkpoint Gate

1. Output parent checkpoint: `<promise>BATCH_[N]_VERIFIED</promise>`
2. Use `AskUserQuestion`:

   ```
   "Batch [N] ([batch-name]) complete: [X]/61 issues done.

   Status:
   - Issues fixed: [list of issue numbers]
   - Tests: ✅ passing
   - Commits: ✅ pushed

   Continue to Batch [N+1] ([next-batch-name])?
   Options: 'yes' to continue, 'pause' to stop, 'skip' to skip next batch"
   ```

3. Wait for user response

### Step 6: Handle User Response

- **"yes"**: Continue to next batch (Step 1)
- **"pause"**: Output `<promise>PIPELINE_PAUSED</promise>` and stop gracefully
- **"skip"**: Mark next batch as SKIPPED, move to batch after that

### Step 7: Final Completion

After Batch 8 verified:

1. Verify all 61 issues marked ✅ in `deferred-issues.md`
2. Verify counter shows (61/61 complete)
3. Create final summary in master log
4. Output: `<promise>PIPELINE_COMPLETE</promise>` 🎉

---

## Self-Healing

### Batch Fails to Complete

**Symptoms**: No completion promise after max iterations, or verification fails

**Action**:

1. Document failure in master log
2. Save current state (what was completed)
3. Output `<promise>BATCH_[N]_FAILED</promise>`
4. `AskUserQuestion`: "Batch [N] failed. Retry batch, skip batch, or pause pipeline?"
5. Handle response:
   - **retry**: Restart batch from beginning (up to 2 retries)
   - **skip**: Mark batch as BLOCKED, continue to next
   - **pause**: Stop pipeline gracefully

### Test Regression

**Symptoms**: Tests passing before batch, failing after

**Action**:

1. Document regression in master log
2. Output `<promise>TEST_REGRESSION_DETECTED</promise>`
3. `AskUserQuestion`: "Tests regressed during batch [N]. Debug, rollback, or continue?"
4. Handle response:
   - **debug**: Investigate failing tests, fix, re-verify
   - **rollback**: `git reset --hard` to before batch, mark BLOCKED
   - **continue**: Accept regression, document reason

### Git Conflicts

**Symptoms**: Cannot push commits due to conflicts

**Action**:

1. Attempt `git pull --rebase`
2. If conflicts: Output `<promise>GIT_CONFLICT</promise>`
3. `AskUserQuestion`: "Git conflicts detected. Resolve manually?"
4. Wait for user to resolve, then continue

### Iteration Limit Reached

**Symptoms**: Batch hits max iterations without completion

**Action**:

1. Document in master log what was completed
2. Output `<promise>ITERATION_LIMIT_REACHED</promise>`
3. `AskUserQuestion`: "Batch [N] hit iteration limit. Extend limit and retry, or skip batch?"

---

## Master Session Log Structure

**File**: `.ai/sessions/[date]-pipeline-execution.md`

```markdown
# Pipeline Execution - All Deferred Issues

**Started**: YYYY-MM-DD HH:MM
**Status**: IN_PROGRESS | PAUSED | COMPLETE | BLOCKED

## Overall Progress

- Total Issues: 61
- Completed: X
- Remaining: Y
- Current Batch: [batch-name]

## Batch Summary

### ✅ Batch 1: Quick Wins (6 issues)

- Started: YYYY-MM-DD HH:MM
- Completed: YYYY-MM-DD HH:MM
- Duration: Xh Ym
- Issues: #6, #7, #15, #21, #25, #26
- Commits: [list of commit SHAs]
- Session Log: `.ai/sessions/[date]-quick-wins-batch.md`
- Notes: [any important notes]

### ✅ Batch 2: Data Integrity (7 issues)

... (same structure)

### 🚧 Batch 3: Event System (2 issues)

- Status: IN_PROGRESS
- Started: YYYY-MM-DD HH:MM
  ... (same structure)

### ⏳ Batch 4: Real-Time Features

- Status: PENDING

... (continue for all batches)

## Issues Encountered

- [Timestamp] Batch X: [description of issue]
- [Timestamp] Resolution: [how it was handled]

## Final Verification

- [ ] All 61 issues marked ✅
- [ ] Counter shows (61/61)
- [ ] All tests passing
- [ ] All commits pushed
- [ ] All batch PRDs archived
```

---

## Resume Capability

### If Pipeline Paused or Interrupted

1. **Check master log** for last verified batch
2. **Resume from next batch**:
   - If Batch 3 verified → Resume from Batch 4
3. **Verify current state**:
   - Check deferred-issues.md counters
   - Verify tests still passing
   - Check git status
4. **Continue execution** from Step 1 of next batch

### Resume Command

```bash
/ralph-loop:ralph-loop "Resume .ai/tasks/current-prd.md from last verified batch. Check master log at .ai/sessions/[date]-pipeline-execution.md to determine resume point, verify state, continue execution." --max-iterations 2000 --completion-promise "PIPELINE_COMPLETE"
```

---

## Success Criteria

- [ ] All 8 batches executed
- [ ] All 61 issues marked ✅ in deferred-issues.md
- [ ] Issue counter shows (61/61 complete)
- [ ] All tests passing (517 frontend, 423 backend)
- [ ] No linter errors
- [ ] All commits pushed to remote
- [ ] All batch PRDs archived
- [ ] Master session log complete
- [ ] `<promise>PIPELINE_COMPLETE</promise>` output

---

## Important Context for Ralph

### Orchestration Role

- You are the **meta-orchestrator**, not implementing issues directly
- Follow each batch PRD's instructions exactly
- Use batch completion promises as gates between batches
- Update master state after each batch
- Give user control at checkpoints

### Batch Execution

- Each batch PRD is self-contained
- Follow batch's specific instructions (don't deviate)
- Use batch's subagent invocations as specified
- Honor batch's checkpoints and promises
- Let batch handle its own issue loop

### State Management

- Master log is source of truth for pipeline state
- deferred-issues.md is source of truth for issue status
- Always verify before moving to next batch
- Document everything in master log

### User Interaction

- Ask user at EVERY batch boundary
- Give clear status (issues done, tests, commits)
- Offer options (continue, pause, skip)
- Respect user's choice

### Failure Handling

- Don't stop pipeline for single issue failures
- Batch completion promises indicate batch success
- Document failures in master log
- Let user decide on retries/skips

---

## Ralph Command

```bash
/ralph-loop:ralph-loop "Follow .ai/tasks/current-prd.md to execute pipeline of 8 batch PRDs. For each batch: pre-flight check, execute batch PRD, wait for batch completion promise, verify results, update master log, output batch verified promise, ask user to continue. Handle failures gracefully. Output PIPELINE_COMPLETE when all 8 batches done." --max-iterations 2000 --completion-promise "PIPELINE_COMPLETE"
```

---

## Project Context

**Tech Stack**:

- Frontend: React 19, TypeScript, TanStack Query, Tailwind CSS
- Backend: Rust, Tauri 2.x, serde, tokio
- Testing: Vitest + RTL (frontend), cargo test (backend)

**Repository**:

- Branch: `main` (or create feature branch per batch?)
- Tests: Must pass at all times
- Commits: Use conventional commits format

**Key Files**:

- Issue tracker: `.ai/tasks/deferred-issues.md`
- Batch PRDs: `.ai/tasks/prd-*.md`
- Master log: `.ai/sessions/[date]-pipeline-execution.md`
- Batch logs: `.ai/sessions/[date]-[batch-name]-batch.md`

---

## Safety Limits

- **Max iterations per batch**: See batch PRD (150-350)
- **Max iterations total**: 2000 (safety limit)
- **Max retries per batch**: 2
- **Test regression tolerance**: 0 (must investigate)
- **Required checkpoints**: After every batch

---

## Final Notes

This is a long-running, complex pipeline. The key to success:

1. **Follow batch PRDs exactly** - they have detailed instructions
2. **Verify at every gate** - don't skip verification steps
3. **Give user control** - ask at every batch boundary
4. **Document everything** - master log is critical
5. **Handle failures gracefully** - don't let one issue block entire pipeline
6. **Respect completion promises** - they signal batch success
7. **Update state religiously** - deferred-issues.md + master log
8. **Test constantly** - verify tests pass between batches

The pipeline is designed to be pausable, resumable, and self-documenting. Trust the process.
