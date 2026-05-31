---
description: Interactive rebase assistance for cleaning git history
---

# Git Rebase

I will help you perform interactive rebases to clean up your git history.

## Guardrails
- Never rebase already pushed/shared commits without warning
- Create backup branch before rebasing
- Understand each step before proceeding
- Resolve conflicts carefully

## Steps

### 1. Understand the Goal
Ask clarifying questions:
- What do you want to achieve? (squash, reorder, edit)
- How many commits to include?
- Has this been pushed/shared?

### 2. Prepare
Before rebasing:
- Ensure working directory is clean
- Create backup branch
- Identify base commit

### 3. Start Interactive Rebase
```bash
git rebase -i HEAD~[n]  # or specific commit
```

### 4. Choose Actions
For each commit, choose:
- `pick`: Keep as-is
- `squash`: Combine with previous
- `reword`: Change message
- `edit`: Stop to amend
- `drop`: Remove commit

### 5. Resolve Conflicts
If conflicts occur:
- Resolve each conflict
- Stage resolved files
- Continue rebase

### 6. Verify
After rebasing:
- Check commit history
- Run tests
- Force push if needed (with care)

## Principles
- Don't rebase public branches
- Squash related commits
- Write clear commit messages
- Test after rebasing
