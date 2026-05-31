---
description: Perform comprehensive code review for quality, security, and maintainability
---

# Code Review

I will help you review code for quality, security, and best practices, adapting to your project's language and conventions.

## Guardrails
- Focus on significant issues, not style nitpicks
- Provide constructive feedback with suggested fixes
- Consider the context and constraints of the codebase
- Don't suggest major refactoring unless explicitly requested
- Respect the project's existing patterns and conventions

## Steps

### 1. Understand Context
Before reviewing, ask:
- What is the purpose of this code change?
- Which files should be reviewed?
- Are there specific concerns to focus on?
- Is this a feature, bug fix, or refactor?

### 2. Analyze Project
Detect the project setup:
- **Language**: JavaScript, TypeScript, Python, Go, etc.
- **Framework**: React, Vue, Django, Express, etc.
- **Linting**: Check for ESLint, Prettier, Black, etc.
- **Existing patterns**: Look at similar code for conventions

### 3. Review Checklist

Go through each category systematically:

#### 🔒 Security
- No hardcoded secrets or credentials
- Input validation present
- SQL injection prevention (parameterized queries)
- XSS prevention (proper escaping/sanitization)
- Authentication/authorization properly handled
- Sensitive data not logged

#### 🐛 Bugs & Logic
- Edge cases handled (null, empty, boundary values)
- Null/undefined checks where needed
- Correct use of async/await
- No race conditions
- Error handling present
- No infinite loops or recursion without base case

#### 📐 Architecture
- Single Responsibility Principle followed
- No circular dependencies
- Appropriate abstraction level
- Consistent with existing patterns
- Proper separation of concerns

#### 🧪 Testing
- Tests cover new functionality
- Tests are meaningful (not just for coverage)
- Edge cases tested
- Mocks used appropriately

#### 📖 Readability
- Clear variable/function names
- Complex logic has comments
- No magic numbers (use constants)
- Consistent formatting
- Functions not too long

#### ⚡ Performance
- No N+1 query problems
- Expensive operations not in loops
- Proper memoization/caching where needed
- No memory leaks
- Efficient data structures used

### 4. Document Findings
For each issue found, provide:
- **Severity**: High, Medium, or Low
- **Location**: File and line number
- **Problem**: Clear description of the issue
- **Suggestion**: How to fix it
- **Why**: Why this matters

### 5. Summarize Review
Provide an overall summary:
- Number of issues by severity
- Overall code quality assessment
- Recommended action (approve, request changes)
- Positive feedback on good patterns found

## Severity Guidelines

| Severity | Examples |
|----------|----------|
| **High** | Security vulnerabilities, data loss risks, breaking bugs |
| **Medium** | Logic errors, missing validation, performance issues |
| **Low** | Code style, minor improvements, suggestions |

## Principles

### Be Constructive
- Assume good intent
- Explain the "why" behind suggestions
- Offer alternatives, not just criticism
- Praise good patterns you find

### Be Practical
- Consider time constraints
- Prioritize impactful changes
- Suggest incremental improvements
- Don't block on minor issues

## Reference
- Check the project's linting configuration
- Look at existing code for style conventions
- Review the project's testing patterns
