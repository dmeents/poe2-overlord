# Step-by-Step Hooks Refactor Implementation

## Instructions for Cursor
You are implementing the hooks refactor plan one task at a time. This prompt is designed to be reusable across multiple chat sessions while maintaining state persistence.

## Current Context
- **Project**: POE2 Overlord (Path of Exile 2 game overlay)
- **Architecture**: Tauri + Rust backend, React/TypeScript frontend
- **Working Directory**: `/home/david-meents/Documents/projects/poe2-overlord`
- **Implementation Plan**: `HOOKS_REFACTOR_PLAN.md` (contains all tasks and progress)
- **Target**: Frontend hooks in `packages/frontend/src/hooks/`

## Step-by-Step Implementation Process

### 1. STATE ASSESSMENT
Before starting any work:

1. **Read the current refactor plan**:
   ```
   Read the file HOOKS_REFACTOR_PLAN.md to understand:
   - Current progress (completed tasks marked with [x])
   - Next task to implement (first unchecked [ ] task)
   - Dependencies and prerequisites
   - Phase we're currently in
   ```

2. **Identify next task**:
   - Find the first incomplete task `[ ]` in the current phase
   - Verify all prerequisites are completed `[x]`
   - If prerequisites aren't met, identify what needs to be done first

3. **Confirm task readiness**:
   ```
   Before implementing, confirm:
   - Task prerequisites are complete
   - Files mentioned in the task exist and are accessible
   - No conflicts with other ongoing work
   ```

### 2. TASK IMPLEMENTATION
For the identified task:

1. **Announce the task**:
   ```
   State clearly:
   "Implementing Task [ID]: [Name]"
   "Phase: [Phase Number]"
   "Files affected: [List files]"
   "Estimated time: [Time estimate]"
   ```

2. **Create safety checkpoint**:
   ```
   Before making any changes:
   - Create a git branch if needed: `git checkout -b hooks-refactor-[task-id]`
   - Commit current state: `git add -A && git commit -m "Checkpoint before [task-id]"`
   ```

3. **Implement the task**:
   - Follow the specific implementation steps from the plan
   - Make changes incrementally
   - Test frequently during implementation
   - Keep changes focused on the single task

4. **Validate implementation**:
   - Run tests specified in the task
   - Verify success criteria are met
   - Check that functionality still works as expected
   - Run type checking and linting

5. **Document completion**:
   ```
   After successful implementation:
   - Commit the changes: `git add -A && git commit -m "Complete [task-id]: [description]"`
   - Update HOOKS_REFACTOR_PLAN.md to mark task as complete [x]
   - Add any implementation notes or deviations to the plan
   - Update any affected line numbers or file references
   ```

### 3. STATE PERSISTENCE
After completing each task:

1. **Update the plan document**:
   ```
   In HOOKS_REFACTOR_PLAN.md:
   - Change [ ] to [x] for completed task
   - Add completion timestamp
   - Note any implementation variations
   - Update progress metrics
   - Record any issues encountered
   ```

2. **Log progress**:
   ```
   Add to the plan file:
   - Task [ID] completed at [timestamp]
   - Files modified: [list]
   - Tests passed: [list]
   - Any notes or deviations from original plan
   ```

### 4. SESSION CONTINUATION
If you need to stop and continue later:

1. **Session summary**:
   ```
   Before ending, document in HOOKS_REFACTOR_PLAN.md:
   - Current status: "Paused after completing Task [ID]"
   - Next task ready: "Task [Next-ID] ready to implement"
   - Any context needed for continuation
   - Current git branch and commit hash
   ```

2. **Resume instructions**:
   ```
   When resuming in a new chat:
   1. Read HOOKS_REFACTOR_PLAN.md for current state
   2. Check git status and current branch
   3. Identify next task from the plan
   4. Continue with Step 1 (State Assessment)
   ```

## Implementation Guidelines

### Safety First
- **One task at a time**: Never implement multiple tasks simultaneously
- **Frequent commits**: Commit after each successful change
- **Test continuously**: Run tests after each modification
- **Rollback ready**: Keep rollback commands handy for each change

### Quality Standards
- **Follow existing patterns**: Maintain consistency with existing codebase
- **Preserve functionality**: Ensure no regressions in behavior
- **Update documentation**: Keep comments and docs in sync
- **Type safety**: Maintain TypeScript strictness

### Error Handling
If a task fails:
1. **Stop immediately**: Don't continue to next tasks
2. **Rollback safely**: Use the rollback procedure from the plan
3. **Document the issue**: Update the plan with failure details
4. **Reassess**: Determine if the task needs modification or skipping

## Communication Protocol

### Start of Session
```
"Starting hooks refactor implementation"
"Reading current state from HOOKS_REFACTOR_PLAN.md..."
[Report current progress and next task]
```

### During Implementation
```
"Implementing Task [ID]: [Name]"
[Show implementation steps as you work]
[Report test results and validation]
```

### End of Task
```
"Task [ID] completed successfully"
"Updated HOOKS_REFACTOR_PLAN.md with completion status"
"Ready for next task: [Next-ID]"
```

### Session End
```
"Implementation session ending"
"Progress: [X] of [Y] tasks completed"
"Next session should start with Task [ID]"
"State saved in HOOKS_REFACTOR_PLAN.md"
```

## Commands for Implementation

### Git Management
```bash
# Create task branch
git checkout -b hooks-refactor-[task-id]

# Checkpoint before changes
git add -A && git commit -m "Checkpoint before [task-id]"

# Commit completed task
git add -A && git commit -m "Complete [task-id]: [description]"

# Check status
git status
git log --oneline -5
```

### Testing Commands
```bash
# Frontend testing
cd packages/frontend
yarn test
yarn lint
yarn type-check

# Build verification
yarn build

# Full project test
cd ../..
yarn test
```

### File Operations
```bash
# Check file exists and get info
ls -la packages/frontend/src/hooks/[filename]
wc -l packages/frontend/src/hooks/[filename]

# Backup before major changes
cp packages/frontend/src/hooks/[filename] /tmp/[filename].backup
```

## Start Command
To begin implementation, simply say:
```
"Start implementing the hooks refactor plan"
```

I will then:
1. Read HOOKS_REFACTOR_PLAN.md
2. Assess current state
3. Identify next task
4. Begin implementation following this protocol

## Resume Command
To resume in a new chat session, say:
```
"Resume hooks refactor implementation"
```

I will then:
1. Read the current state from HOOKS_REFACTOR_PLAN.md
2. Report progress and next steps
3. Continue with the next incomplete task