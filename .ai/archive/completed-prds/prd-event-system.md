# PRD: Event System Batch - Deferred Issues

## Context

This PRD addresses **2 out of 61** deferred issues from the domain refactoring session (2026-01-11). These issues add event infrastructure to improve frontend reactivity.

**This batch**: Issues #8, #14 (Event System Integration)
**Prerequisites**: Quick Wins + Data Integrity batches complete
**Remaining after this**: 46 issues

## Issues in This Batch

### Issue #8: Configuration Event Listener

**Severity**: HIGH (Functional Gap)
**Domain**: Configuration Management
**Type**: Frontend event handling - stale data prevention

### Issue #14: Character Deletion Events

**Severity**: HIGH (Functional Gap)
**Domain**: Character Management
**Type**: Backend event publishing + Frontend handling

---

## Implementation Loop

**For EACH issue (2 total)**:

### Step 1: Plan

1. Read issue from `.ai/tasks/deferred-issues.md`
2. Invoke `@implementation-planner`:
   ```
   @implementation-planner "Create implementation plan for Issue #[N]: [Title].
   Design event flow from backend to frontend, identify event types needed."
   ```
3. Review plan - verify full event flow (publish → transport → subscribe)

### Step 2: Implement

1. **Backend**: Add event variant to AppEvent enum
2. **Backend**: Publish event at appropriate time
3. **Frontend**: Add event type definition
4. **Frontend**: Subscribe to event in appropriate context
5. **Frontend**: Handle event (update state, invalidate queries)
6. Run tests: `yarn test` and `cargo test`
7. Run linters: `yarn lint && yarn format` and `cargo clippy`

### Step 3: Verify

1. All tests pass
2. No linter errors
3. **Manual verification**: Trigger event, verify frontend updates
4. Test event doesn't fire when it shouldn't

### Step 4: Commit

1. Commit: `"feat(events): [issue title] (Issue #N)"`
2. Include: `Co-Authored-By: Warp <agent@warp.dev>`

### Step 5: Document

1. Update `.ai/sessions/[date]-event-system-batch.md`:
   - Mark issue complete
   - Document event flow
   - Note any edge cases

### Step 6: Continue

Move to next issue

---

## Checkpoints

**After Issue #14 (both complete)**:

- Push all commits: `git push origin HEAD`
- Mark issues ✅ in `.ai/tasks/deferred-issues.md`
- Update counters (15 complete, 46 remaining)
- Final session log update
- Commit: `"docs: event system batch complete"`
- Output: `<promise>EVENT_SYSTEM_COMPLETE</promise>`

---

## Self-Healing

**Event not firing**:

- Verify event is published in backend
- Check event bus subscription in frontend
- Add debug logging to trace event flow
- Iterate up to 3 times

**Frontend not updating**:

- Check event handler logic
- Verify query invalidation
- Check React state updates
- Test with browser devtools

**Key principle**: Follow the event from publish to handler to verify full flow.

---

## Success Criteria

- [ ] Both issues fixed
- [ ] All tests passing (517 frontend, 423 backend)
- [ ] Events fire when they should
- [ ] Events don't fire when they shouldn't
- [ ] Frontend updates on events
- [ ] No memory leaks from event listeners

---

## Important Context for Ralph

**Subagent Usage**:

- Invoke `@implementation-planner` for event flow design
- Ask about event transport mechanism (Tauri events)

**Testing Strategy**:

- Backend: Test event publishing
- Frontend: Test event subscription and handling
- Integration: Manually verify end-to-end flow

**Commit Strategy**:

- One commit per issue
- Use `feat(events):` prefix (new functionality)
- Push after both commits

**Issue Dependencies**:

- Issue #14 unblocks Issue #15 from Quick Wins
- Both issues are similar pattern (can reuse approach)

**When Stuck**:

- Check existing event patterns in codebase
- Verify Tauri event system working
- Test with `invoke` tool or manual trigger

## Project Context

**Tech Stack**:

- Frontend: React 19, TypeScript, TanStack Query, Tauri events
- Backend: Rust, Tauri 2.x, EventBus pattern
- Event Transport: Tauri IPC events

**Event Patterns** (check `.ai/memory/patterns.md`):

- Backend: `event_bus.publish(AppEvent::VariantName(data))`
- Frontend: `listen('event_name', (event) => { /* handle */ })`
- Event types: Defined in both Rust and TypeScript

**Key Files** (for reference only):

- Backend events: `packages/backend/src/infrastructure/event_bus/*`
- Frontend contexts: `packages/frontend/src/contexts/*`
- Event types: TypeScript types in frontend

---

## Ralph Command

```bash
/ralph-loop:ralph-loop "Follow .ai/tasks/prd-event-system.md to add 2 event system features. For each: invoke @implementation-planner for event flow design, implement backend event + frontend listener, test end-to-end, commit. Output EVENT_SYSTEM_COMPLETE when both done." --max-iterations 100 --completion-promise "EVENT_SYSTEM_COMPLETE"
```

## Final Completion

When `<promise>EVENT_SYSTEM_COMPLETE</promise>` output:

1. Both event issues complete
2. Event flows documented
3. End-to-end verification done
4. Issues marked ✅ in deferred-issues.md
5. Counters updated (15 complete, 46 remaining)
6. This PRD archived
7. All commits pushed

---

## Issue Details Reference

See `.ai/tasks/deferred-issues.md` for:

- Full issue descriptions
- Current state analysis
- Complexity assessments

---

## Session Documentation

Create `.ai/sessions/[date]-event-system-batch.md` with:

- Issues completed (checklist)
- Event flows documented (diagrams or descriptions)
- Testing results
- Gotchas and learnings
