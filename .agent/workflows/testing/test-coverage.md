---
description: Improve test coverage for specific files or functions
---

# Test Coverage

I will help you improve test coverage for your codebase.

## Guardrails
- Focus on meaningful coverage, not just numbers
- Prioritize critical paths
- Don't test framework code
- Keep tests maintainable

## Steps

### 1. Analyze Current Coverage
Run coverage report:
- `npm run test -- --coverage`
- `pytest --cov`
- Identify uncovered files/lines

### 2. Understand Gaps
Ask clarifying questions:
- Which files need coverage?
- What's the target percentage?
- Any specific functions to prioritize?

### 3. Prioritize
Focus on:
- Business-critical logic
- Complex functions
- Error handling paths
- Edge cases

### 4. Write Tests
For each uncovered area:
- Understand what it does
- Identify test cases
- Write meaningful tests
- Cover edge cases

### 5. Verify Coverage
After adding tests:
- Run coverage again
- Check for improvements
- Ensure tests are meaningful

## Principles
- 100% coverage doesn't mean bug-free
- Test behavior, not implementation
- Focus on critical paths first
- Avoid testing boilerplate
