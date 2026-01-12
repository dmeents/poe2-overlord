# PRD: Real-Time Features Batch - Deferred Issues

## Context
This PRD addresses **1 out of 61** deferred issues from the domain refactoring session (2026-01-11). This issue adds real-time updating to zone timing functionality.

**This batch**: Issue #5 (Real-Time Zone Timer)
**Prerequisites**: Quick Wins + Data Integrity + Event System batches complete
**Remaining after this**: 45 issues

## Issues in This Batch

### Issue #5: Real-Time Zone Timer Updates
**Severity**: HIGH (Functional Gap)
**Domain**: Zone Tracking
**Type**: Frontend reactivity - live elapsed time display

---

## Implementation Loop

**For this single issue**:

### Step 1: Plan
1. Read issue from `.ai/tasks/deferred-issues.md`
2. Invoke `@implementation-planner`:
   ```
   @implementation-planner "Create implementation plan for Issue #5: Real-Time Zone Timer Updates. 
   Design approach for live elapsed time updates using intervals/requestAnimationFrame."
   ```
3. Review plan - decide on timing mechanism (setInterval vs RAF vs useQuery polling)

### Step 2: Implement
1. **Frontend**: Add timer hook or component
2. **Frontend**: Connect to zone entry timestamp
3. **Frontend**: Calculate and format elapsed time
4. **Frontend**: Update UI every second (or per frame)
5. **Cleanup**: Clear intervals/timers on unmount
6. Run tests: `yarn test`
7. Run linters: `yarn lint && yarn format`

### Step 3: Verify
1. All tests pass
2. No linter errors
3. **Manual verification**: Enter zone, watch timer increment
4. **Performance**: Check CPU usage (should be minimal)
5. **Memory**: Verify no leaks (timers cleaned up)

### Step 4: Commit
1. Commit: `"feat(zone-tracking): real-time zone timer updates (Issue #5)"`
2. Include: `Co-Authored-By: Warp <agent@warp.dev>`

### Step 5: Document
1. Update `.ai/sessions/[date]-real-time-features-batch.md`:
   - Mark issue complete
   - Document timer approach chosen
   - Performance considerations

### Step 6: Continue
No more issues - proceed to checkpoint

---

## Checkpoints

**After Issue #5 (complete)**:
- Push commit: `git push origin HEAD`
- Mark issue ✅ in `.ai/tasks/deferred-issues.md`
- Update counters (16 complete, 45 remaining)
- Final session log update
- Commit: `"docs: real-time features batch complete"`
- Output: `<promise>REAL_TIME_COMPLETE</promise>`

---

## Self-Healing

**Timer not updating**:
- Verify interval/RAF is running
- Check state update logic
- Test with console.log every tick
- Iterate up to 3 times

**Performance issues**:
- Switch from RAF to 1-second interval
- Debounce state updates
- Use CSS animations for smooth updates

**Memory leaks**:
- Verify cleanup in useEffect return
- Check timer references cleared
- Test with React DevTools Profiler

**Key principle**: Real-time UI should be efficient - don't re-render entire component tree every tick.

---

## Success Criteria

- [ ] Issue #5 fixed
- [ ] All tests passing (517 frontend, 423 backend)
- [ ] Timer updates every second (smooth, no jank)
- [ ] CPU usage minimal (<1%)
- [ ] No memory leaks on zone changes
- [ ] Timer resets correctly on new zone entry

---

## Important Context for Ralph

**Subagent Usage**:
- Invoke `@implementation-planner` for timer mechanism design
- Ask about performance trade-offs (interval vs RAF)

**Testing Strategy**:
- Unit test: Timer calculation logic
- Integration test: Timer updates on zone entry
- Manual test: Watch timer for 60+ seconds
- Performance: Check CPU/memory in browser devtools

**Commit Strategy**:
- One commit for implementation
- Use `feat(zone-tracking):` prefix
- Push immediately after commit

**Issue Dependencies**:
- This issue is independent
- Improves UX for zone tracking feature

**When Stuck**:
- Check existing timer patterns in codebase
- Consider using TanStack Query refetchInterval
- Look for React hooks that handle timers

## Project Context

**Tech Stack**:
- Frontend: React 19, TypeScript, TanStack Query
- Timer patterns: useEffect + setInterval or refetchInterval
- State: React hooks (useState, useEffect)

**Timer Patterns** (check `.ai/memory/patterns.md`):
- useEffect with interval: Standard React pattern
- TanStack Query refetchInterval: Auto-polling
- requestAnimationFrame: Smooth animations (60fps)

**Key Files** (for reference only):
- Zone tracking: `packages/frontend/src/contexts/ZoneContext.tsx`
- Zone display: UI components showing zone info
- Time utilities: Any existing time formatting helpers

---

## Ralph Command

```bash
/ralph-wiggum:ralph-loop "Follow .ai/tasks/prd-real-time-features.md to add real-time zone timer. Invoke @implementation-planner for timer design, implement with React hooks, verify performance, commit. Output REAL_TIME_COMPLETE when done." --max-iterations 60 --completion-promise "REAL_TIME_COMPLETE"
```

## Final Completion

When `<promise>REAL_TIME_COMPLETE</promise>` output:
1. Issue #5 complete
2. Timer mechanism documented
3. Performance verified
4. Issue marked ✅ in deferred-issues.md
5. Counters updated (16 complete, 45 remaining)
6. This PRD archived
7. Commit pushed

---

## Issue Details Reference

See `.ai/tasks/deferred-issues.md` for:
- Full issue description
- Current state analysis
- Complexity assessment

---

## Session Documentation

Create `.ai/sessions/[date]-real-time-features-batch.md` with:
- Issue completed (checklist)
- Timer approach chosen (interval vs RAF)
- Performance results (CPU, memory)
- Testing results
- Gotchas and learnings
