---
description: Improve code quality, extract functions, reduce duplication
---

# Refactor

I will help you refactor code to improve quality while preserving functionality.

## Guardrails
- Never change behavior, only structure
- Make small, incremental changes
- Ensure tests pass after each change
- Preserve public APIs unless explicitly asked

## Steps

### 1. Understand Scope
Ask clarifying questions:
- Which files or functions to refactor?
- What problems are you seeing? (duplication, complexity, etc.)
- Are there tests covering this code?
- Any constraints to be aware of?

### 2. Analyze Code
Identify issues:
- Code duplication
- Long functions/methods
- Deep nesting
- Unclear naming
- Mixed responsibilities

### 3. Plan Refactoring
Common patterns:
- **Extract Function**: Pull out reusable logic
- **Rename**: Improve clarity of names
- **Inline**: Remove unnecessary abstractions
- **Move**: Relocate to better location
- **Simplify Conditionals**: Reduce complexity

### 4. Execute Refactoring
Make changes incrementally:
- One refactoring at a time
- Run tests after each change
- Commit frequently

### 5. Verify
- All tests still pass
- Code is more readable
- No behavior changes

## Principles
- Refactor in small steps
- Make the change easy, then make the easy change
- If it hurts, do it more often
