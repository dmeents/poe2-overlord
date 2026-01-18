# PRD: Debug Character List Not Appearing

## Context
A runtime bug has been reported: **the character list is not appearing when the application starts**.

**Type**: Runtime bug
**Severity**: HIGH (core feature broken)
**Suspected Cause**: Unknown - needs log-based investigation

---

## Investigation Approach

This is an **iterative debugging task** where you orchestrate but delegate analysis:
1. **You capture logs** - Run app in interact mode, save output
2. **@debugger analyzes** - Pass logs to debugger for analysis
3. **You add debug logs** - Based on debugger's findings, add strategic logs
4. **You re-run app** - Capture new logs with debug output
5. **@debugger analyzes again** - Until root cause clear
6. **@implementation-planner designs fix** - Based on confirmed root cause
7. **You implement** - Follow the implementation plan
8. **You verify** - Clean logs, tests pass

**Key**: YOU handle app execution and code edits. Subagents analyze logs and design fixes.

---

## Step-by-Step Investigation

### Step 1: Capture Initial Logs
Run app in interact mode to capture all output:

```bash
yarn dev
```

Interact task:
```
Monitor logs for 20 seconds. Capture ALL output: compilation errors, runtime errors, warnings, panics, character-related messages. Report complete logs.
```

Save the captured logs - you'll pass them to @debugger.

### Step 2: Pass Logs to @debugger for Analysis
Invoke @debugger with captured logs:

```
@debugger "Analyze these logs to identify why character list isn't appearing:

**Captured Logs**:
```
[Paste complete log output here]
```

**What to identify**:
1. All compilation errors (missing imports, type errors, renamed functions)
2. All runtime errors/panics
3. Relevant warnings
4. Root cause hypothesis
5. Where I should add debug logs if root cause unclear

Provide detailed analysis."
```

@debugger will analyze and tell you what's wrong.

### Step 3: Add Debug Logs Based on @debugger's Findings
Based on @debugger's analysis, YOU add strategic debug logs:

**If @debugger says "add logs to trace character loading"**:
Add to backend (`packages/backend/src/domain/character/service.rs`):
```rust
eprintln!("[DEBUG] get_all_characters called");
eprintln!("[DEBUG] Found {} characters", result.len());
```

**If @debugger says "check frontend query state"**:
Add to frontend (character hook/component):
```typescript
console.error('[DEBUG] Characters query:', { status, data, error });
```

**If @debugger identifies exact error**:
Skip to Step 6 (no more debug logs needed).

### Step 4: Re-run App with Debug Logs
Restart app to capture new logs with debug output:

```bash
yarn dev
```

Capture logs again (20 seconds) - now with your debug output.

### Step 5: Pass New Logs to @debugger Again
Invoke @debugger with updated logs:

```
@debugger "Here are the new logs with debug output:

**New Logs**:
```
[Paste new log output with debug statements]
```

**Previous analysis**: [What you said before]

With this additional debug info, what's the confirmed root cause?"
```

Repeat Steps 3-5 until @debugger confirms root cause.

### Step 6: Pass Root Cause to @implementation-planner for Fix Design
Once @debugger confirms root cause, ask @implementation-planner to design the fix:

```
@implementation-planner "Design a fix for this issue:

**Root Cause** (from @debugger): [Root cause @debugger identified]

**Affected Area**: [Files/components mentioned]

**Requirements**:
1. Create step-by-step implementation plan
2. Identify all files that need changes
3. Specify exact changes needed
4. Include testing strategy
5. Note any risks

Provide detailed fix plan I can follow."
```

@implementation-planner will create detailed implementation plan.

### Step 7: Implement the Fix
YOU implement following @implementation-planner's plan:

- Make code changes step-by-step as specified
- Follow the plan exactly
- Make edits to all identified files

### Step 8: Verify Fix
1. Remove debug logs (if you added any)
2. Restart app: `yarn dev`
3. Monitor logs (20 seconds)
4. Verify:
   - ✅ No compilation errors
   - ✅ No runtime errors/panics
   - ✅ No warnings
   - ✅ App starts successfully

### Step 9: Run Tests
```bash
yarn test
```

All tests should pass.

### Step 10: Document and Commit
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

**Debugging Workflow** (Main agent orchestrates, subagents analyze/plan):
1. **Capture**: Run app, save logs
2. **Delegate Analysis**: Pass logs to @debugger
3. **Iterate**: Add debug logs per @debugger's guidance, re-run
4. **Repeat**: Until @debugger confirms root cause
5. **Delegate Planning**: Pass root cause to @implementation-planner
6. **Implement**: Follow @implementation-planner's plan
7. **Verify**: Run app, check clean logs + tests pass

**Subagent Usage**:
- `@debugger`: Analyzes logs, identifies root cause, suggests where to add debug logs
- `@implementation-planner`: Designs fix plan based on root cause
- Main agent: Runs app, adds debug logs, implements fix plan
- Iterative loop: Main captures → @debugger analyzes → Main adds logs → repeat

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
