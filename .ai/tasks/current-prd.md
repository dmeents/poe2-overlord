# PRD: Debug Character List Not Appearing

## Context
A runtime bug has been reported: **the character list is not appearing when the application starts**.

**Type**: Runtime bug
**Severity**: HIGH (core feature broken)
**Suspected Cause**: Unknown - needs log-based investigation

---

## Investigation Approach

This is a **log-based debugging task**. You will autonomously:
1. **Start the app** with `yarn dev` in interact mode
2. **Capture and analyze logs** (compilation errors, runtime errors, warnings)
3. **Identify errors and inconsistencies** in the logs
4. **Add strategic debug logging** if needed to gather more evidence
5. **Analyze code** to understand why errors are occurring
6. **Implement fixes** based on log analysis
7. **Restart and verify** - logs should be clean

---

## Step-by-Step Investigation

### Step 1: Start the App in Interact Mode
Start app with interact subagent to monitor logs:
```bash
yarn dev
```

**Task for interact subagent**: Monitor logs for 15-20 seconds and capture:
- Any compilation errors (Rust or TypeScript)
- Runtime errors or panics
- Warnings about missing functions, types, or imports
- Character-related errors
- Failed command invocations

Let app fully start up before stopping.

### Step 2: Analyze Captured Logs
Review the log output for:
- **Compilation errors**: Missing imports, renamed functions, type errors
- **Runtime errors**: Panics, unwraps on None, failed operations
- **Warnings**: Unused imports, deprecations, type mismatches
- **Missing logs**: Should see character loading on startup - is it missing?

**Document all errors/warnings found.**

### Step 3: Analyze Recent Changes (if logs unclear)
If logs don't reveal obvious error, check recent commits:
```bash
git log --oneline -20 -- packages/backend/src/domain/character/ packages/frontend/src/contexts/ packages/frontend/src/hooks/
```

Look for:
- Renamed functions or types
- Changed imports
- Modified query keys or command names

### Step 4: Investigate Error Root Cause
Based on errors found in logs, read relevant files:

**If compilation error**: Read the file mentioned in error
**If runtime error**: Read the function/module where panic occurred
**If no obvious error**: Read character loading flow:
- `packages/frontend/src/hooks/use-characters.ts`
- `packages/backend/src/commands/characters.rs`
- `packages/backend/src/domain/character/service.rs`

Identify:
- What specific line is causing the error?
- Why is it failing? (missing import, wrong type, renamed function?)
- What was changed recently that could cause this?

### Step 5: Add Debug Logging (if needed)
If error not obvious from initial logs, add strategic debug logs:

**Backend** (at entry point of suspected issue):
```rust
eprintln!("[DEBUG] Function X called with args: {:?}", args);
eprintln!("[DEBUG] Result: {:?}", result);
```

**Frontend** (if frontend might be involved):
```typescript
console.error('[DEBUG] Hook/Component state:', { status, data, error });
```

**Location**: Add logs at:
- Function entry points
- Before operations that might fail
- After critical operations

### Step 6: Restart with Debug Logs
Restart app with new logs:
```bash
yarn dev
```

Capture output again. New logs should reveal:
- Where execution stops
- What values are unexpected
- Which branch is taken

### Step 7: Identify Root Cause
Based on all evidence:
- **Compilation error** → Fix syntax, imports, types
- **Missing function** → Function renamed in refactor, update call sites
- **Type mismatch** → Types changed, update usage
- **Runtime panic** → Add error handling or fix logic
- **Silent failure** → Missing command registration or broken query

### Step 8: Implement Fix
Make targeted fix based on root cause:
- **Missing import** → Add import statement
- **Renamed function** → Update all call sites
- **Type mismatch** → Fix type annotations
- **Logic error** → Correct the logic
- **Missing registration** → Register command/handler

### Step 9: Verify Fix - Check Logs Are Clean
1. Remove debug logs (if any were added)
2. Restart app: `yarn dev`
3. Monitor logs for 15-20 seconds
4. Verify:
   - **No compilation errors**
   - **No runtime errors or panics**
   - **No warnings about the fixed code**
   - App starts successfully

### Step 10: Run Tests
Verify nothing else broke:
```bash
yarn test
```

All tests should pass.

### Step 11: Document and Commit
1. Create session log: `.ai/sessions/[date]-character-list-bug-fix.md`
2. Document:
   - Symptoms observed
   - Investigation steps taken
   - Root cause identified
   - Fix applied
   - Verification results
3. Commit: `"fix(characters): [description of root cause fix]"`
4. Include: `Co-Authored-By: Warp <agent@warp.dev>`

---

## Success Criteria

- [ ] Application starts without compilation errors
- [ ] No runtime errors or panics in logs
- [ ] All tests passing (`yarn test`)
- [ ] Root cause identified from log analysis
- [ ] Fix implemented and verified via clean logs
- [ ] Session log documents: symptoms, logs captured, root cause, fix
- [ ] Fix committed with clear message

---

## Important Context for Claude

**Debugging Workflow**:
1. Run app in interact mode, capture logs (15-20 seconds)
2. Analyze logs for errors, warnings, inconsistencies
3. Identify root cause from log analysis
4. Add debug logging if needed, restart, capture again
5. Implement fix based on evidence
6. Verify: Clean logs on restart + tests pass

**Subagent Usage**:
- Consider invoking `@debugger` for investigation guidance
- Debugger agent specializes in runtime debugging approach

**Tech Stack Reminders**:
- **Frontend**: React 19, TanStack Query (for data fetching)
- **Backend**: Rust, Tauri commands
- **Data**: File-based (character files + index)
- **State**: React Context + TanStack Query cache

**Key Files** (likely suspects):
- Frontend query: `packages/frontend/src/hooks/use-characters.ts`
- Frontend context: `packages/frontend/src/contexts/CharacterContext.tsx`
- Frontend page: `packages/frontend/src/routes/characters.tsx`
- Backend service: `packages/backend/src/domain/character/service.rs`
- Backend command: `packages/backend/src/commands/characters.rs`

---

## Recommended Approach: Ralph Loop

Since this is autonomous log-based debugging, **Ralph loop is well-suited** for this task.

**Command**:
```bash
/ralph-loop:ralph-loop "Follow .ai/tasks/current-prd.md to debug character list issue. Start app in interact mode, capture and analyze logs for errors/warnings, identify root cause, add debug logging if needed, implement fix, verify logs are clean and tests pass. Document findings. Output BUG_FIXED when complete." --max-iterations 50 --completion-promise "BUG_FIXED"
```

**Alternative: Regular Conversation**
If you prefer to observe the investigation:
```
Please debug why the character list isn't appearing.

1. Start `yarn dev` in interact mode and analyze the logs
2. Identify any errors or warnings
3. Add debug logging if needed to pinpoint the issue
4. Implement a fix
5. Verify with clean logs and passing tests
```

---

## Debugging Capability Notes

**What Claude can determine from logs:**
- Compilation errors (missing imports, type errors, renamed functions)
- Runtime errors (panics, unwraps, failed operations)
- Warning messages
- Missing functionality (commands not registered, handlers not found)

**What Claude cannot determine from logs alone:**
- UI rendering issues (if logs are clean but UI still broken)
- Visual bugs
- UX issues

**For this character list issue**: Since it's likely a code error from the refactoring pipeline, logs should reveal the problem.

---

## Session Log Template

`.ai/sessions/[date]-character-list-bug-fix.md`:

```markdown
# Character List Bug Investigation

**Date**: YYYY-MM-DD
**Issue**: Character list not appearing on app start

## Symptoms
- [what you observed]

## Investigation Steps
1. [step taken]
2. [evidence gathered]

## Root Cause
[what was broken and why]

## Fix Applied
[code changes made]

## Verification
[how fix was verified to work]

## Related Commits
- [commit SHA and message]
```
