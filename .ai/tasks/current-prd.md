# PRD: Walkthrough Guide Refinement

## Context
The walkthrough guide is stored as JSON at `packages/backend/config/walkthrough_guide.json`. Users want to refine/improve the guide by providing **plain English instructions**, and Claude should intelligently translate those into JSON edits.

**Type**: Content editing with structured data
**File**: `packages/backend/config/walkthrough_guide.json`
**Goal**: Make it easy to update walkthrough steps without manually editing JSON

---

## Guide Structure Understanding

### JSON Schema
```json
{
  "acts": {
    "act_N": {
      "act_name": "Act N",
      "act_number": N,
      "steps": {
        "act_N_step_M": {
          "id": "act_N_step_M",
          "title": "Step Title",
          "description": "Step description",
          "objectives": [
            {
              "text": "Objective text",
              "details": "Additional details",
              "required": true/false,
              "rewards": ["Reward 1", "Reward 2"],
              "notes": "Optional notes"
            }
          ],
          "current_zone": "Zone Name",
          "completion_zone": "Next Zone Name",
          "next_step_id": "act_N_step_M+1" or null,
          "previous_step_id": "act_N_step_M-1" or null,
          "wiki_items": ["Item 1", "Item 2"]
        }
      }
    }
  }
}
```

### Key Concepts
- **Acts**: Top-level containers (Act 1, Act 2, etc.)
- **Steps**: Individual guide steps within an act
- **Objectives**: Tasks within a step (can be required or optional)
- **Zones**: Game locations (current_zone, completion_zone)
- **Step IDs**: Linked list structure (next_step_id, previous_step_id)
- **Wiki Items**: References to game encyclopedia

---

## Approach

When user provides plain English instructions, you should:

1. **Parse the request** - Understand what they want to change
2. **Read current state** - Load the walkthrough guide JSON
3. **Identify affected sections** - Which act/step/objective?
4. **Make precise edits** - Update only what's needed
5. **Validate structure** - Ensure JSON is valid and links intact
6. **Explain changes** - Summarize what was updated

---

## Example Instructions & How to Handle

### Example 1: "Add a note to Act 1 Step 3 about Beira being north of the waypoint"

**What to do**:
1. Read `walkthrough_guide.json`
2. Find `acts.act_1.steps.act_1_step_3`
3. Find objective with text containing "Beira"
4. Update the `notes` field
5. Save changes

### Example 2: "Change the reward for completing the witch hut to include a skill gem"

**What to do**:
1. Find step with "Witch Hut" objective
2. Update `rewards` array for that objective
3. Save changes

### Example 3: "Add a new optional objective to Act 1 Step 4 for finding a hidden chest"

**What to do**:
1. Find `acts.act_1.steps.act_1_step_4.objectives`
2. Add new objective object with `required: false`
3. Save changes

### Example 4: "Fix the typo in Act 1 Step 5 where it says 'Empale' instead of 'Impale'"

**What to do**:
1. Search for "Empale" in the file
2. Replace with "Impale"
3. Save changes

### Example 5: "Add 'The Grim Tangle' to wiki items for Act 1 Step 6"

**What to do**:
1. Find `acts.act_1.steps.act_1_step_6.wiki_items`
2. Add "The Grim Tangle" if not already present
3. Save changes

---

## Step-by-Step Process

### Step 1: Parse User Request
Understand:
- What act/step is affected?
- What field needs to change? (title, description, objectives, zones, wiki_items)
- Is this adding, modifying, or removing content?
- Are there any ambiguities that need clarification?

### Step 2: Read Current Guide
```bash
# Read the entire file to understand current state
```
Read: `packages/backend/config/walkthrough_guide.json`

Locate the relevant section based on user's instructions.

### Step 3: Make Edits
Use `edit_files` to make precise JSON changes:

**Important**:
- Maintain JSON formatting (2-space indentation)
- Preserve existing structure
- Keep step ID links intact (next_step_id, previous_step_id)
- Validate JSON syntax

### Step 4: Validate Changes
After editing:
1. Verify JSON is valid (no syntax errors)
2. Check step links are intact
3. Ensure no data loss in adjacent sections

### Step 5: Summarize Changes
Explain to user:
- What was changed
- Where it was changed (Act X, Step Y)
- Confirm the change matches their request

---

## Common Editing Patterns

### Adding a New Objective
```json
{
  "text": "New objective text",
  "details": "Additional details",
  "required": false,
  "rewards": [],
  "notes": ""
}
```

### Modifying Existing Text
- Find exact section
- Update the specific field
- Preserve all other fields

### Adding to Arrays
- `objectives`: Append to list
- `rewards`: Add to rewards array
- `wiki_items`: Add unique items only

### Updating Zone Information
- `current_zone`: Where step starts
- `completion_zone`: Where step ends
- Both should be valid zone names

---

## Success Criteria

- [ ] User's plain English request understood correctly
- [ ] Relevant section of JSON identified
- [ ] Changes made precisely (no unintended edits)
- [ ] JSON remains valid after changes
- [ ] Step links (next/previous) still intact
- [ ] User can verify the change matches their intent

---

## Important Guidelines

**Precision**:
- Only edit what user requested
- Don't make additional "improvements" without asking
- Preserve exact formatting and structure

**Validation**:
- Always verify JSON syntax after edits
- Check that arrays and objects are properly closed
- Ensure no trailing commas

**Communication**:
- If request is ambiguous, ask for clarification
- After changes, explain what was updated
- If request impossible (e.g., nonexistent step), explain why

**Data Integrity**:
- Don't break step ID chains
- Maintain act/step numbering consistency
- Keep wiki_items relevant to the step

---

## Example Session Flow

**User**: "Add a note to Act 1 Step 3 that Beira is always north of the waypoint"

**Claude**:
1. Reads `walkthrough_guide.json`
2. Finds `acts.act_1.steps.act_1_step_3.objectives[0]` (Beira objective)
3. Updates `notes` field: `"Always located north of the waypoint"`
4. Saves changes
5. Responds: "✅ Updated Act 1, Step 3 - Added note to Beira objective about location"

---

## Tech Stack Context

**File Format**: JSON (structured data)
**Location**: `packages/backend/config/walkthrough_guide.json`
**Used by**: Backend walkthrough service, frontend walkthrough UI
**Validation**: Must be valid JSON syntax

**Related Files**:
- Backend: `packages/backend/src/domain/walkthrough/*`
- Frontend: `packages/frontend/src/components/walkthrough/*`

---

## When to Ask for Clarification

- "Add an objective to Act 1" → Which step in Act 1?
- "Change the reward" → Which step/objective has the reward?
- "Fix that typo" → Which typo specifically?
- "Update Step 5" → Which act's Step 5?

Be specific but not pedantic - use context clues when obvious.

---

## Ready to Use

Just provide plain English instructions like:
- "Add X to step Y"
- "Change the description of Act 2 Step 1 to say..."
- "Remove the optional objective about..."
- "Fix the typo where it says..."
- "Add a new step after Act 1 Step 5 for..."

Claude will handle the JSON editing intelligently!
