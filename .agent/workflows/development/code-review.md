---
description: Perform thorough code review with focus on quality, security, and maintainability
---

# Code Review

I will help you review code for quality, security, and best practices.

## Guardrails
- Focus on significant issues, not style nitpicks (unless egregious)
- Provide constructive feedback with suggested fixes
- Consider the context and constraints of the codebase
- Don't suggest changes that would require major refactoring unless explicitly requested

## Steps

### 1. Understand Context

Before reviewing, understand:
- What is the purpose of this code change?
- What files are being modified?
- Are there any related tests?

### 2. Review Checklist

Go through each category systematically:

#### 🔒 Security
- [ ] No hardcoded secrets or credentials
- [ ] Input validation present
- [ ] SQL injection prevention (parameterized queries)
- [ ] XSS prevention (proper escaping)
- [ ] Authentication/authorization properly handled

#### 🐛 Bugs & Logic
- [ ] Edge cases handled
- [ ] Null/undefined checks where needed
- [ ] Correct use of async/await
- [ ] No race conditions
- [ ] Error handling present

#### 📐 Architecture
- [ ] Single Responsibility Principle followed
- [ ] No circular dependencies
- [ ] Appropriate abstraction level
- [ ] Consistent with existing patterns

#### 🧪 Testing
- [ ] Tests cover new functionality
- [ ] Tests are meaningful (not just for coverage)
- [ ] Edge cases tested

#### 📖 Readability
- [ ] Clear variable/function names
- [ ] Complex logic has comments
- [ ] No magic numbers
- [ ] Consistent formatting

#### ⚡ Performance
- [ ] No N+1 query problems
- [ ] Expensive operations not in loops
- [ ] Proper memoization where needed
- [ ] No memory leaks

### 3. Document Findings

For each issue found, provide:
```markdown
### [Severity: High/Medium/Low] Issue Title

**File:** `path/to/file.ts:L42`

**Problem:** Description of the issue

**Suggestion:** 
```typescript
// Suggested fix
```

**Why:** Explanation of why this matters
```

### 4. Summarize Review

Provide an overall summary:
- Number of issues by severity
- Overall code quality assessment
- Recommended action (approve, request changes, etc.)

## Severity Guidelines

- **High**: Security vulnerabilities, data loss risks, breaking bugs
- **Medium**: Logic errors, missing validation, performance issues
- **Low**: Code style, minor improvements, suggestions

## Guidelines
- Be kind and constructive
- Assume good intent
- Offer to pair if issues are complex
- Praise good patterns you see

## Reference
- Use `rg` to search for similar patterns in the codebase
- Check for existing linting rules in `.eslintrc`
- Look at test coverage with `npm run test:coverage`
