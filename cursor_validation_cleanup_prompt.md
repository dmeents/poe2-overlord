# Step-by-Step Validation Cleanup Implementation

## Instructions for Cursor
You are implementing the validation cleanup plan one task at a time. This prompt is designed to be reusable across multiple chat sessions while maintaining state persistence for the final cleanup phase of the hooks refactor.

## Current Context
- **Project**: POE2 Overlord (Path of Exile 2 game overlay)
- **Architecture**: Tauri + Rust backend, React/TypeScript frontend
- **Working Directory**: `/home/david-meents/Documents/projects/poe2-overlord`
- **Cleanup Plan**: `HOOKS_VALIDATION_CLEANUP_PLAN.md` (contains all cleanup tasks and progress)
- **Target**: Final cleanup of frontend hooks in `packages/frontend/src/hooks/`
- **Phase**: Post-validation cleanup and polish

## Step-by-Step Cleanup Process

### 1. STATE ASSESSMENT
Before starting any cleanup work:

1. **Read the current cleanup plan**:
   ```
   Read the file HOOKS_VALIDATION_CLEANUP_PLAN.md to understand:
   - Current progress (completed tasks marked with [x])
   - Next task to implement (first unchecked [ ] task in highest priority phase)
   - Task dependencies and prerequisites
   - Current phase we're working in (Critical/Dead Code/Polish)
   ```

2. **Identify next task**:
   - Find the first incomplete task `[ ]` in the **highest priority phase**
   - Verify all prerequisites are completed `[x]`
   - Check task type: Integration Fix, Dead Code Removal, Type Safety, Performance, or Consistency
   - If prerequisites aren't met, identify blocking tasks first

3. **Confirm task safety**:
   ```
   Before implementing, confirm:
   - Task is properly scoped and independent
   - Files mentioned in the task exist and are accessible
   - No other tasks are currently modifying the same files
   - Current git state is clean for checkpoint creation
   ```

### 2. TASK IMPLEMENTATION
For the identified cleanup task:

1. **Announce the task**:
   ```
   State clearly:
   "Implementing Cleanup Task [VC-ID]: [Name]"
   "Phase: [Critical/Dead Code/Polish]"
   "Type: [Integration Fix/Dead Code/Type Safety/Performance/Consistency]"
   "Files affected: [List files and line numbers]"
   "Estimated time: [Time estimate]"
   "Risk level: [Low/Medium/High]"
   ```

2. **Create safety checkpoint**:
   ```
   Before making any changes:
   - Commit current state: `git add -A && git commit -m "Checkpoint before [VC-ID]"`
   - Create task backup if high risk: `git branch backup-[VC-ID]`
   ```

3. **Implement the cleanup task**:
   - Follow the specific implementation steps from the cleanup plan
   - For **Integration Fixes**: Focus on component compatibility and API alignment
   - For **Dead Code Removal**: Verify code is truly unused before removal
   - For **Type Safety**: Improve types without breaking existing functionality
   - For **Performance**: Fix memory leaks and optimize re-renders
   - For **Consistency**: Standardize patterns while preserving behavior
   - Make changes incrementally and test frequently

4. **Validate implementation**:
   ```
   After implementation, verify:
   - TypeScript compilation passes: `cd packages/frontend && yarn type-check`
   - No linting errors: `yarn lint`
   - Affected tests pass: `yarn test [affected-test-files]`
   - Success criteria from task plan are met
   - No new warnings or errors introduced
   ```

5. **Document completion**:
   ```
   After successful implementation:
   - Commit the changes: `git add -A && git commit -m "Complete [VC-ID]: [description]"`
   - Update HOOKS_VALIDATION_CLEANUP_PLAN.md to mark task as complete [x]
   - Add completion timestamp and any implementation notes
   - Record actual time taken vs estimate
   - Note any issues encountered or deviations from plan
   ```

### 3. STATE PERSISTENCE
After completing each cleanup task:

1. **Update the cleanup plan document**:
   ```
   In HOOKS_VALIDATION_CLEANUP_PLAN.md:
   - Change [ ] to [x] for completed task
   - Add completion timestamp: "Completed at [ISO timestamp]"
   - Record actual implementation time
   - Note any deviations from planned approach
   - Update affected file references if they changed
   - Record any new issues discovered during implementation
   ```

2. **Log cleanup progress**:
   ```
   Add to the plan file under task entry:
   - ✓ Task [VC-ID] completed at [timestamp]
   - Files modified: [list with line changes]
   - Tests updated/verified: [list]
   - Implementation notes: [any important details]
   - Issues resolved: [specific problems fixed]
   ```

### 4. SESSION CONTINUATION
If you need to stop and continue cleanup later:

1. **Session summary**:
   ```
   Before ending, document in HOOKS_VALIDATION_CLEANUP_PLAN.md:
   - Current status: "Paused after completing Task [VC-ID]"
   - Current phase progress: "[X] of [Y] Phase N tasks complete"
   - Next task ready: "Task [Next-VC-ID] ready for implementation"
   - Any context needed for continuation
   - Current git branch and commit hash
   ```

2. **Resume instructions**:
   ```
   When resuming in a new chat:
   1. Read HOOKS_VALIDATION_CLEANUP_PLAN.md for current state
   2. Check git status
   3. Identify next highest-priority incomplete task
   4. Continue with Step 1 (State Assessment)
   ```

## Cleanup Implementation Guidelines

### Safety First - Critical for Cleanup Phase
- **One task at a time**: Never implement multiple cleanup tasks simultaneously
- **Verify before removing**: Double-check that code is truly unused before deletion
- **Test immediately**: Run compilation and tests after every change
- **Preserve functionality**: Ensure no regressions in component behavior
- **Document changes**: Record all modifications for rollback if needed

### Quality Standards for Cleanup
- **Complete removal**: When removing dead code, remove all supporting code too
- **Type safety**: Maintain or improve TypeScript strictness
- **Integration integrity**: Ensure all component imports and usage remain correct
- **Performance maintenance**: Don't introduce new performance issues while fixing others
- **Consistency**: Apply changes uniformly across similar patterns

### Error Handling for Cleanup
If a cleanup task fails:
1. **Stop immediately** - Don't continue to dependent tasks
2. **Analyze the failure** - Understand if it's a plan issue or implementation issue
3. **Rollback safely** - Use git to revert to checkpoint before the failed task
4. **Update the plan** - Document the failure and reassess the task
5. **Consider alternatives** - Modify approach or mark task as "needs investigation"

### Cleanup Task Types - Specific Handling

#### Integration Fixes (High Priority)
- **Focus**: Restore component functionality without breaking changes
- **Testing**: Verify affected components work in isolation and integration
- **Validation**: Check imports, exports, and API usage match expectations

#### Dead Code Removal (Medium Priority) 
- **Focus**: Safe removal of truly unused code
- **Verification**: Search entire codebase to confirm code is unused
- **Testing**: Ensure removal doesn't break any functionality

#### Type Safety Improvements (Medium Priority)
- **Focus**: Improve types without breaking existing code
- **Validation**: Ensure TypeScript compilation passes with stricter types
- **Testing**: Verify no runtime type errors introduced

#### Performance Fixes (Medium Priority)
- **Focus**: Fix memory leaks and unnecessary re-renders
- **Testing**: Verify fixes don't introduce other performance issues
- **Validation**: Check component rendering behavior

#### Consistency Improvements (Low Priority)
- **Focus**: Standardize patterns and naming across hooks
- **Validation**: Ensure changes don't break existing component usage
- **Testing**: Verify all affected integrations still work

## Communication Protocol

### Start of Cleanup Session
```
"Starting validation cleanup implementation"
"Reading current state from HOOKS_VALIDATION_CLEANUP_PLAN.md..."
"Current phase: [Phase name]"
"Progress: [X] of [Y] tasks completed in current phase"
[Report next task to implement]
```

### During Task Implementation
```
"Implementing Cleanup Task [VC-ID]: [Name]"
"Type: [Task type]"
"Risk level: [Risk level]"
[Show implementation steps as you work]
[Report compilation/test results]
```

### End of Task
```
"Cleanup Task [VC-ID] completed successfully"
"Files modified: [list]"
"Tests verified: [list]"
"Updated HOOKS_VALIDATION_CLEANUP_PLAN.md with completion status"
"Ready for next task: [Next-VC-ID]"
```

### Phase Completion
```
"Phase [N] completed: [Phase name]"
"Phase results: [summary of what was accomplished]"
"Moving to Phase [N+1]: [Next phase name]"
"Overall progress: [X] of [Y] total cleanup tasks completed"
```

### Session End
```
"Cleanup session ending"
"Progress: [X] of [Y] tasks completed ([X]% complete)"
"Current phase: [Phase name] - [X] of [Y] phase tasks complete"
"Next session should start with Task [VC-ID]"
"State saved in HOOKS_VALIDATION_CLEANUP_PLAN.md"
```

## Commands for Cleanup Implementation

### Testing Commands for Cleanup
```bash
# Frontend type checking
cd packages/frontend
yarn type-check

# Linting
yarn lint

# Run specific tests
yarn test [test-file-pattern]

# Build verification
yarn build

# Full project validation
cd ../..
yarn test
yarn build
```

### Code Verification Commands
```bash
# Search for usage of code before removing
grep -r "functionName" packages/frontend/src/
grep -r "exportName" packages/frontend/src/

# Check import statements
grep -r "from.*hookName" packages/frontend/src/

# Verify file is not imported
grep -r "import.*filename" packages/frontend/src/
```

## Start Command
To begin cleanup implementation, simply say:
```
"Start implementing the validation cleanup plan"
```

I will then:
1. Read HOOKS_VALIDATION_CLEANUP_PLAN.md
2. Assess current cleanup state
3. Identify next highest-priority task
4. Begin implementation following this protocol

## Resume Command
To resume cleanup in a new chat session, say:
```
"Resume validation cleanup implementation"
```

I will then:
1. Read the current state from HOOKS_VALIDATION_CLEANUP_PLAN.md
2. Report cleanup progress and current phase
3. Identify next task to implement
4. Continue with the next incomplete task

## Final Validation Command
When all cleanup tasks are complete, say:
```
"Perform final validation cleanup review"
```

I will then:
1. Verify all tasks in cleanup plan are marked complete
2. Run full compilation and test suite
3. Check for any remaining issues
4. Generate final cleanup summary report
5. Confirm hooks refactor is fully complete and production-ready