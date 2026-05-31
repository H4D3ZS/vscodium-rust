---
description: Help resolve merge conflicts with context-aware suggestions
---

# Git Conflict Resolution

I will help you understand and resolve merge conflicts by analyzing both sides of the conflict.

## Guardrails
- Never auto-resolve without user confirmation
- Explain what each side of the conflict represents
- Preserve intended functionality from both branches
- Run tests after resolution if available

## Steps

### 1. Identify Conflicts
Check current conflict status:
- Run `git status` to see conflicted files
- List all files with conflicts

### 2. Analyze Each Conflict
For each conflicted file:
- Show the conflict markers and surrounding context
- Explain what the `HEAD` (current) version contains
- Explain what the incoming version contains
- Identify if changes overlap or are independent

### 3. Understand Intent
Ask clarifying questions:
- What was the goal of YOUR changes?
- What feature/fix does the incoming branch contain?
- Should both changes be kept, or one preferred?

### 4. Suggest Resolution
For each conflict, recommend one of:

| Strategy | When to Use |
|----------|-------------|
| Keep ours | Your changes should take precedence |
| Keep theirs | Incoming changes should take precedence |
| Merge both | Both changes are needed and compatible |
| Manual rewrite | Changes conflict semantically, need new approach |

### 5. Resolve Conflicts
After user confirms the approach:
- Remove conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`)
- Apply the agreed resolution
- Stage the resolved file

### 6. Verify Resolution
- Run `git diff --staged` to review changes
- Run tests if available
- Ensure the code compiles/runs

### 7. Complete Merge
Once all conflicts are resolved:
- Run `git status` to confirm no remaining conflicts
- Commit the merge

## Principles
- Understand before resolving
- When in doubt, ask
- Test after resolving
- Document complex resolutions in commit message

## Reference
- `git log --merge` shows commits involved in the conflict
- `git diff` shows current conflicts
- `git checkout --ours/--theirs <file>` for bulk resolution
