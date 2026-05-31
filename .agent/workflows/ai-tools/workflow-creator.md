---
description: Create new stack-agnostic workflows for the Antigravity repository
---

# Workflow Creator

I will help you create a new workflow that follows our stack-agnostic, question-driven philosophy.

## Guardrails
- Every workflow MUST be stack-agnostic
- Never hardcode specific frameworks, libraries, or tools
- Always include a stack detection step
- Always include clarifying questions
- Keep workflows focused on a single task

## Steps

### 1. Define the Workflow
Gather information:
- **Name**: kebab-case identifier (e.g., `git-commit`, `debug-error`)
- **Category**: development, git, testing, debugging, security, documentation, deployment, database, ai-tools, or creative
- **Description**: One-line summary (5-10 words)
- **Purpose**: What problem does this workflow solve?

### 2. Follow the Template Structure

Every workflow must have these sections:

```markdown
---
description: [5-10 word description]
---

# Workflow Name

Brief intro: what this accomplishes and when to use it.

## Guardrails
- What to AVOID doing
- Scope boundaries
- Critical constraints

## Steps

### 1. Understand Context
Ask clarifying questions:
- What is the goal?
- What constraints exist?
- What's the expected outcome?

### 2. Analyze Project
Detect existing stack:
- Check relevant config files
- Identify framework, tools, patterns
- Look at existing code for conventions

If unclear, ask the user.

### 3. [Core Implementation Steps]
Describe WHAT to do, not exact code.
Let AI generate appropriate implementation.

### 4. Verify
- How to confirm success
- What to test

## Principles
- Universal best practices

## Reference
- Links to relevant documentation
```

### 3. Validate Against Core Principles

Ensure your workflow follows:

| Principle | Check |
|-----------|-------|
| Stack-Agnostic | Does it work with ANY framework? |
| Question-Driven | Does it ask clarifying questions? |
| Single Responsibility | Does it do ONE thing well? |
| Progressive Disclosure | Does it start minimal, expand on demand? |
| Composable | Can it combine with other workflows? |

### 4. Create the File
Create the workflow file at:
```
workflows/<category>/<name>.md
```

### 5. Update Registry
Add entry to `workflows/registry.json`:
```json
"<name>": {
  "category": "<category>",
  "description": "<5-10 word description>",
  "tags": ["tag1", "tag2", "tag3"]
}
```

### 6. Test the Workflow
1. Copy the file to a test project's `.agent/workflows/` directory
2. Open Antigravity in that project
3. Type `/<name>` to trigger the workflow
4. Verify it asks appropriate questions
5. Verify it detects project stack correctly

## Common Mistakes to Avoid

### ❌ DON'T: Hardcode frameworks
```markdown
### Install Dependencies
npm install react tailwindcss
```

### ✅ DO: Detect and adapt
```markdown
### Analyze Project Stack
- Check for existing UI framework
- Check for existing CSS approach
If unclear, ask the user which they prefer.
```

### ❌ DON'T: Provide boilerplate code
```markdown
Create `Button.tsx`:
import React from 'react'
export const Button = () => <button>Click</button>
```

### ✅ DO: Describe what to create
```markdown
Create a button component that:
- Accepts variant props (primary, secondary)
- Follows the project's existing component patterns
- Uses the project's styling approach
```

## Reference
- See existing workflows in `workflows/` for examples
- Check [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines
