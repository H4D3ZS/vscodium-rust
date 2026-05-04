---
description: Create comprehensive pull request descriptions from commits
---

# Git PR

I will help you create a comprehensive pull request description based on your branch's commits and changes.

## Guardrails
- Analyze commits on the current branch vs target branch
- Don't modify any code, only generate PR content
- Follow the project's PR template if one exists

## Steps

### 1. Understand Context
Ask clarifying questions:
- What is the target branch? (usually `main` or `develop`)
- Is there a linked issue number?
- Is this a feature, bug fix, or something else?

### 2. Analyze Changes
Gather information about the PR:
- Run `git log main..HEAD --oneline` to see commits
- Run `git diff main --stat` for changed files summary
- Review the actual changes for context

### 3. Check for PR Template
Look for existing templates:
- `.github/PULL_REQUEST_TEMPLATE.md`
- `.github/pull_request_template.md`

If found, use that structure.

### 4. Generate PR Content

**Title:**
`<type>: <brief description>`

**Description sections:**

```markdown
## Description
Brief summary of what this PR does.

## Changes
- List of specific changes made
- Bullet points for clarity

## Related Issue
Closes #<issue-number>

## Type of Change
- [ ] New feature
- [ ] Bug fix
- [ ] Refactor
- [ ] Documentation

## Testing
How the changes were tested.

## Screenshots (if UI changes)
Before/after if applicable.
```

### 5. Present and Refine
Show the generated PR content and ask if user wants to:
- Use as-is
- Modify any section
- Add more details

## Principles
- Be concise but complete
- Highlight breaking changes prominently
- Link related issues and PRs
- Help reviewers understand the "why"

## Reference
- Check project's existing PRs for style
- Look at CONTRIBUTING.md for guidelines
