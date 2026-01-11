# PRD: Domain-Driven Functionality Refactoring

## Goal

Systematically analyze and refactor the entire codebase domain-by-domain, using a code analyzer subagent for deep analysis followed by implementation of functional improvements. Prioritize bugs, gaps, and inconsistencies over style.

## Domain Refactoring Order

### Phase 1: Infrastructure Domains

#### Domain 1: Configuration Management

**Frontend**: `components/forms/settings-form`
**Backend**: `domain/configuration/*`
**Why First**: Everything depends on config
**Checkpoint**: `<promise>CONFIG_DOMAIN_COMPLETE</promise>`

#### Domain 2: Wiki Scraping (Backend only)

**Frontend**: N/A
**Backend**: `domain/wiki_scraping/*`
**Why Second**: Provides data for other domains
**Checkpoint**: `<promise>WIKI_DOMAIN_COMPLETE</promise>`

#### Domain 3: Server/Game Monitoring

**Frontend**: `components/status/*`
**Backend**: `domain/server_monitoring/*`, `domain/game_monitoring/*`
**Why Third**: Critical infrastructure
**Checkpoint**: `<promise>MONITORING_DOMAIN_COMPLETE</promise>`

### Phase 2: Core User Features

#### Domain 4: Character Management

**Frontend**: `components/character/*` (6 components)
**Backend**: `domain/character/*` (6 modules)
**Why Fourth**: Primary user feature
**Checkpoint**: `<promise>CHARACTER_DOMAIN_COMPLETE</promise>`

#### Domain 5: Zone Tracking

**Frontend**: `components/zones/*` (5 components)
**Backend**: `domain/zone_tracking/*`, `domain/zone_configuration/*`
**Why Fifth**: Core gameplay tracking
**Checkpoint**: `<promise>ZONE_DOMAIN_COMPLETE</promise>`

#### Domain 6: Economy System

**Frontend**: `components/economy/*` (5 components)
**Backend**: `domain/economy/*` (3 modules)
**Why Sixth**: Important user feature
**Checkpoint**: `<promise>ECONOMY_DOMAIN_COMPLETE</promise>`

### Phase 3: Supporting Features

#### Domain 7: Walkthrough/Guides

**Frontend**: `components/walkthrough/*` (3 components)
**Backend**: `domain/walkthrough/*`
**Why Seventh**: User guidance
**Checkpoint**: `<promise>WALKTHROUGH_DOMAIN_COMPLETE</promise>`

#### Domain 8: UI Foundation

**Frontend**: `components/ui/*`, `components/forms/*`, `components/layout/*`
**Backend**: N/A
**Why Last**: Shared UI components
**Checkpoint**: `<promise>UI_DOMAIN_COMPLETE</promise>`

## Implementation Loop

**For EACH domain**:

### Step 1: Discovery & Mapping

1. List all frontend components in domain
2. List all backend modules in domain
3. Identify shared types/contracts between FE and BE
4. Document domain boundaries

### Step 2: Deep Analysis (Use @code-analyzer)

For EACH file/module in domain:

- Invoke: `@code-analyzer "Analyze [file/module path] in the context of [domain name]. Provide structured recommendations prioritizing functional correctness."`
- Parse analyzer output
- Aggregate findings across domain

### Step 3: Prioritized Implementation

1. **Fix ALL Critical issues**:
   - Bugs
   - Data integrity problems
   - Security vulnerabilities
   - Functional gaps
   - Contract violations

2. **Fix MOST High Priority issues**:
   - Logic errors
   - State management issues
   - Error handling gaps
   - Performance bugs

3. **Selectively fix Medium Priority**:
   - High-impact code quality issues
   - Important performance improvements
   - Critical testing gaps

### Step 4: Validation

1. **Run tests**:
   - Frontend: `yarn test` (filter by domain if possible)
   - Backend: `cargo test` (filter by domain module)
2. **Run linters**:
   - Frontend: `yarn lint && yarn format`
   - Backend: `cargo clippy`
3. **Manual verification**:
   - Test critical user flows for domain
   - Verify frontend ↔ backend integration

### Step 5: Documentation

1. Update session log with:
   - Issues found (by severity)
   - Fixes implemented
   - Testing results
   - Any remaining TODOs

### Step 6: Commit & Checkpoint

1. **Commit domain changes**:
   - Group by severity: "fix(domain): critical issues in [domain]"
   - Or by feature: "fix(character): resolve data integrity issues"
2. **Update session log**
3. **HARD STOP**:
   - Output `<promise>[DOMAIN]_COMPLETE</promise>`
   - Use AskUserQuestion: "Domain [name] refactoring complete. Review changes and approve to continue to next domain?"
4. **Push to remote**: `git push origin HEAD`
5. Continue to next domain

## Self-Healing

Ralph should autonomously fix issues that arise:

### When Tests Fail
- **Analyze the failure**: Read test output to understand what broke
- **Determine root cause**: Is it the production code or the test itself?
- **Fix appropriately**:
  - If production code has a bug → fix the code
  - If test is wrong/outdated → fix the test
  - If both need changes → fix both
- **Retry**: Run tests again to verify fix
- **Iterate**: Repeat up to 3 times if needed

### When Linter/Compiler Errors Occur
- Fix linter errors in code or tests
- Fix compilation errors
- Run linter/compiler again to verify

### When Analyzer Recommendations Are Unclear
- Ask @code-analyzer for clarification with more context
- Or use best judgment based on codebase patterns

### When Stuck
- After 3 failed attempts on same issue:
  - Document the problem in commit message
  - Note in session log for manual review
  - Skip to next file/module
  - Continue with domain

**Key Principle**: Fix whatever is broken (code, tests, or both) to get tests passing. Don't skip fixes because tests break.

## Success Criteria

- All 8 domains analyzed and refactored
- All Critical issues fixed
- Most High Priority issues fixed
- All tests passing
- No critical bugs remaining
- Frontend ↔ Backend contracts aligned
- Session log documents all changes

## Session Documentation

Maintain `.ai/sessions/2026-01-11-domain-refactoring.md` with:

- Domains completed (list)
- Issues found per domain (by severity)
- Fixes implemented per domain
- Test results
- Remaining TODOs
- Patterns/learnings discovered

## Completion Signal

When all domains complete:

1. Final test suite run (frontend + backend)
2. Update session log with final summary
3. Update `.ai/memory/patterns.md` with learnings
4. Update `.ai/memory/decisions.md` if architectural changes made
5. Archive this PRD
6. Push all commits
7. Output `<promise>ALL_DOMAINS_COMPLETE</promise>`

## Notes

- Infrastructure first ensures stable foundation
- Each domain checkpoint allows for review and course correction
- Analyzer subagent gets fresh context per file/module
- Main agent implements with full domain context
- 600 iterations provides buffer for 8 domains with thorough analysis
- Focus on "does it work?" before "is it elegant?"
