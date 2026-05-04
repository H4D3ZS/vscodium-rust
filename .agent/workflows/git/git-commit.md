---
description: Generate conventional commit messages from staged changes
---

# Git Commit

I will help you generate clear, conventional commit messages based on your staged changes.

## Guardrails
- Only analyze staged changes (`git diff --staged`)
- Follow Conventional Commits format
- Keep subject line under 72 characters
- Don't commit if no changes are staged

## Steps

### 1. Analyze Staged Changes
First, check what's staged:
- Run `git diff --staged` to see changes
- Run `git diff --staged --stat` for a summary
- Identify the type and scope of changes

### 2. Determine Commit Type
Based on the changes, select the appropriate type:

| Type | When to Use |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Formatting, no code change |
| `refactor` | Code change that neither fixes nor adds |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `chore` | Build, tooling, dependencies |
| `ci` | CI/CD changes |

### 3. Identify Scope (Optional)
Determine if a scope applies:
- Component name (e.g., `auth`, `api`, `ui`)
- Feature area (e.g., `login`, `dashboard`)
- File type (e.g., `deps`, `config`)

### 4. Write Commit Message
Format: `<type>(<scope>): <description>`

**Rules:**
- Use imperative mood ("add" not "added")
- Don't capitalize first letter
- No period at the end
- Be specific but concise

**Examples:**
- `feat(auth): add OAuth2 login flow`
- `fix: resolve null pointer in user service`
- `docs: update API documentation`
- `refactor(api): extract validation logic`

### 5. Add Body (If Needed)
For complex changes, add a body:
- Leave blank line after subject
- Explain WHAT and WHY, not HOW
- Wrap at 72 characters

### 6. Execute Commit
Present the suggested commit message and ask if user wants to:
- Commit with this message
- Modify the message
- Add more details in body

## Principles
- One commit = one logical change
- If you need "and" in the message, consider splitting
- Reference issues when relevant (e.g., `fixes #123`)

## Reference
- [Conventional Commits](https://www.conventionalcommits.org/)
- Run `git log --oneline -10` to see recent commit style
