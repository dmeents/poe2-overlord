# PRD: Data Integrity Batch - Deferred Issues

## Context
This PRD addresses **7 out of 61** deferred issues from the domain refactoring session (2026-01-11). These are high-priority data integrity and security issues that require careful implementation and testing.

**This batch**: Issues #1, #2, #3, #4, #12, #16, #22 (Data Integrity & Security)
**Prerequisites**: Quick Wins batch complete
**Remaining after this**: 48 issues

## Issues in This Batch

### Issue #1: Path Validation with Migration Strategy
**Severity**: CRITICAL (Security + Data Integrity)
**Domain**: Configuration Management
**Type**: Security vulnerability - path traversal attacks

### Issue #2: Lost Update Prevention Architecture
**Severity**: CRITICAL (Data Integrity)
**Domain**: Configuration Management
**Type**: Concurrent update race condition

### Issue #3: Transaction Safety in Character Creation
**Severity**: CRITICAL (Data Integrity)
**Domain**: Character Management
**Type**: Orphaned files on failure

### Issue #4: Cache Race Condition
**Severity**: CRITICAL (Data Integrity)
**Domain**: Economy System
**Type**: Concurrent cache updates

### Issue #12: Orphaned Character Cleanup
**Severity**: HIGH (Data Integrity)
**Domain**: Character Management
**Type**: Index corruption

### Issue #16: Zone Leave Not Called on Change
**Severity**: HIGH (Data Integrity)
**Domain**: Zone Tracking
**Type**: Timing accuracy

### Issue #22: Walkthrough Race Condition
**Severity**: HIGH (Data Integrity)
**Domain**: Walkthrough/Guides
**Type**: Step progression corruption

---

## Implementation Loop

**For EACH issue (7 total)**:

### Step 1: Plan
1. Read issue from `.ai/tasks/deferred-issues.md`
2. Invoke `@implementation-planner`:
   ```
   @implementation-planner "Create implementation plan for Issue #[N]: [Title]. 
   Investigate codebase, identify affected files, design architecture for data integrity."
   ```
3. Review plan - focus on rollback, transaction safety, data migration

### Step 2: Implement
1. Follow implementation plan step-by-step
2. Make code changes (focus on data safety)
3. Add/update tests (emphasize edge cases and race conditions)
4. Run tests: `yarn test` or `cargo test`
5. Run linters: `yarn lint && yarn format` or `cargo clippy`

### Step 3: Verify
1. All tests pass (including new edge case tests)
2. No linter errors
3. **Manual verification**: Test race conditions, failure scenarios
4. Verify rollback/recovery works

### Step 4: Commit
1. Commit: `"fix(security|data): [issue title] (Issue #N)"`
2. Include: `Co-Authored-By: Warp <agent@warp.dev>`

### Step 5: Document
1. Update `.ai/sessions/[date]-data-integrity-batch.md`:
   - Mark issue complete
   - Document architecture decisions
   - Note any migration steps needed
   - Document edge cases tested

### Step 6: Continue
Move to next issue

---

## Checkpoints

**After Issue #3 (3 issues complete)**:
- Push commits: `git push origin HEAD`
- Update session log
- Commit session log: `"docs: checkpoint - 3 data integrity issues complete"`
- Output: `<promise>DATA_CHECKPOINT_3</promise>`
- Use AskUserQuestion: "3 critical issues done (#1, #2, #3). Continue to remaining 4?"

**After Issue #22 (all 7 complete)**:
- Push all commits
- Mark issues ✅ in `.ai/tasks/deferred-issues.md`
- Update counters (13 complete, 48 remaining)
- Final session log update
- Commit: `"docs: data integrity batch complete"`
- Output: `<promise>DATA_INTEGRITY_COMPLETE</promise>`

---

## Self-Healing

**Tests fail**:
- Analyze failure, especially race conditions
- Fix implementation or add proper synchronization
- Iterate up to 5 times (data integrity is critical)
- If stuck: document thoroughly, mark for manual review

**Architecture too complex**:
- Simplify where possible
- Re-invoke `@implementation-planner` with "simpler approach" request
- Balance correctness vs complexity

**Data migration needed**:
- Document migration steps clearly in commit message
- Add migration verification to tests
- Note in session log for deployment planning

**Key principle**: Data integrity > speed. Take time to get it right.

---

## Success Criteria

- [ ] All 7 issues fixed
- [ ] All tests passing (517 frontend, 423 backend + new tests)
- [ ] Race conditions prevented with proper synchronization
- [ ] Rollback/recovery mechanisms in place
- [ ] Data migration paths documented
- [ ] No new data integrity risks introduced

---

## Important Context for Ralph

**Subagent Usage**:
- ALWAYS invoke `@implementation-planner` - these are complex
- Request architecture review for race conditions
- Ask planner about migration strategies

**Testing Strategy**:
- Add tests for race conditions (concurrent operations)
- Test failure scenarios (disk full, permission denied, etc.)
- Test rollback/recovery mechanisms
- Frontend: `yarn test` Backend: `cargo test`

**Commit Strategy**:
- One commit per issue
- Use `fix(security):` for Issues #1
- Use `fix(data):` for Issues #2, #3, #4, #12, #16, #22
- Push after every 3 commits (checkpoint)

**Issue Complexity**:
- Issues #1, #2, #4 are HIGH complexity (architectural changes)
- Issues #3, #12, #16, #22 are MEDIUM-HIGH complexity
- All require careful testing and verification
- Expect multiple iterations per issue

**When Stuck**:
- Try up to 5 times per issue (higher than Quick Wins)
- Re-invoke planner with specific blocker question
- Document architecture decision in session log
- If truly blocked: mark for pairing session, move to next

## Project Context

**Tech Stack**:
- Frontend: React 19, TypeScript, TanStack Query, Tailwind CSS
- Backend: Rust, Tauri 2.x, serde, tokio, async/await
- Testing: Vitest + RTL (frontend), cargo test + tokio::test (backend)

**Data Integrity Patterns**:
- Backend transactions: Write-read-verify pattern
- Rust synchronization: Arc<Mutex<T>> for shared state
- Concurrent operations: Use tokio locks properly
- Error recovery: Rollback on failure

**Key Domains**:
- Configuration: File-based config with validation
- Character: File-based with index, needs transaction safety
- Economy: HTTP cache with TTL
- Zone Tracking: In-memory state, needs proper lifecycle
- Walkthrough: Progress tracking, needs atomicity

---

## Ralph Command

```bash
/ralph-wiggum:ralph-loop "Follow .ai/tasks/prd-data-integrity.md to fix 7 data integrity issues. For each: invoke @implementation-planner focusing on data safety, implement with proper synchronization, add edge case tests, verify thoroughly. After 3 issues output DATA_CHECKPOINT_3 and ask. After all 7 output DATA_INTEGRITY_COMPLETE." --max-iterations 250 --completion-promise "DATA_INTEGRITY_COMPLETE"
```

## Final Completion

When `<promise>DATA_INTEGRITY_COMPLETE</promise>` output:
1. All 7 critical data integrity issues fixed
2. Comprehensive testing including race conditions
3. Session log documents all architecture decisions
4. Migration paths documented where needed
5. Issues marked ✅ in deferred-issues.md
6. Counters updated (13 complete, 48 remaining)
7. This PRD archived
8. All commits pushed

---

## Issue Details Reference

See `.ai/tasks/deferred-issues.md` for:
- Full issue descriptions
- Current state analysis
- File locations (hints only)
- Complexity assessments

---

## Session Documentation

Create `.ai/sessions/[date]-data-integrity-batch.md` with:
- Issues completed (checklist)
- Architecture decisions made
- Migration steps documented
- Race condition testing results
- Edge cases covered
- Gotchas and learnings
