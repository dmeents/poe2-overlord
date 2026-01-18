# Claude Prompt for Debugging Character List Issue

## Recommended Prompt:

```
Follow .ai/tasks/current-prd.md to debug why the character list isn't appearing.

**Your role**: Orchestrator
1. Run `yarn dev` in INTERACT mode to capture logs
2. Pass logs to @debugger for analysis
3. If @debugger needs more info, add debug logs they request
4. Re-run and pass new logs to @debugger
5. Repeat until @debugger confirms root cause
6. Pass root cause to @implementation-planner for fix design
7. Implement the fix following @implementation-planner's plan
8. Verify

You handle: app execution, code editing
Subagents handle: log analysis, fix planning
```

## For Ralph Loop:

```bash
/ralph-loop:ralph-loop "Follow .ai/tasks/current-prd.md to debug character list. Orchestrate: run app in INTERACT mode, pass logs to @debugger, iteratively add debug logs per @debugger's requests until root cause found, pass to @implementation-planner for fix plan, implement plan, verify. Document findings. Output BUG_FIXED when complete." --max-iterations 75 --completion-promise "BUG_FIXED"
```

## The Model:

**Main Agent (You)**:
- Runs `yarn dev` in interact mode
- Captures and saves logs
- Adds debug logs as requested
- Implements fix code changes
- Verifies the fix works

**@debugger**:
- Analyzes logs
- Identifies root cause
- Suggests debug logs if more info needed
- Iterates until root cause confirmed

**@implementation-planner**:
- Receives confirmed root cause
- Designs detailed fix plan
- Specifies all files and changes needed

**Flow**:
```
Main: Capture logs → @debugger: Analyze → 
  If unclear: Main: Add debug logs → Main: Re-capture → @debugger: Analyze again
  If clear: @debugger: Root cause confirmed → 
    @implementation-planner: Design fix → 
      Main: Implement → Main: Verify
```
