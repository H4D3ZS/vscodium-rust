---
description: Generate comprehensive unit tests with detected testing framework
---

# Unit Testing

I will help you write effective unit tests that adapt to your project's testing framework and patterns.

## Guardrails
- Never assume a specific testing framework (Jest, Vitest, pytest, etc.)
- Detect existing test setup before writing tests
- Test behavior, not implementation details
- Each test should be independent and isolated
- Use descriptive test names that explain expected behavior

## Steps

### 1. Understand What to Test
Ask clarifying questions:
- Which files or functions need tests?
- Are there specific edge cases to cover?
- What's the target code coverage?
- Are there external services that need mocking?
- Any existing test patterns to follow?

### 2. Analyze Testing Setup
Detect the existing test configuration:

**JavaScript/TypeScript projects:**
- Check for Jest (`jest.config.js`, `@jest` in package.json)
- Check for Vitest (`vitest.config.ts`, `vitest` in package.json)
- Check for Mocha (`mocha` in package.json)
- Check for Testing Library (`@testing-library/*`)

**Python projects:**
- Check for pytest (`pytest.ini`, `conftest.py`)
- Check for unittest (standard library)

**Other languages:**
- Look for test configuration files
- Check package/dependency files for test frameworks

If no testing setup exists, ask user which framework they prefer and help set it up.

### 3. Analyze Code to Test
Before writing tests:
- Understand the function's purpose and expected behavior
- Identify inputs, outputs, and side effects
- List edge cases (null, empty, boundary values)
- Identify dependencies that need mocking

### 4. Write Unit Tests
Follow the AAA pattern for each test:

**Arrange:** Set up test data and mocks
**Act:** Call the function being tested
**Assert:** Verify the expected outcome

Structure tests logically:
- Group related tests with `describe` blocks (or equivalent)
- Use clear, descriptive test names
- One primary assertion per test when possible

### 5. Handle Mocking
Mock external dependencies appropriately:
- API calls and network requests
- Database operations
- File system operations
- Third-party services
- Time-dependent operations

### 6. Run and Verify Tests
- Execute the test suite
- Check for passing/failing tests
- Review coverage report if available
- Fix any failing tests

## Principles

### Test Quality
- Test the public API, not internal implementation
- Cover happy path, error cases, and edge cases
- Keep tests fast by mocking slow operations
- Make tests deterministic (no random, no time dependencies)

### Test Organization
- Place tests near the code they test (or in `__tests__` folders)
- Name test files consistently (`*.test.ts`, `*.spec.ts`, `test_*.py`)
- Group related tests logically

### Test Coverage
- Aim for 80%+ coverage on critical paths
- Don't chase 100% coverage for its own sake
- Focus on testing complex logic and edge cases

### Mocking Best Practices
- Mock at the boundary (API calls, not internal functions)
- Reset mocks between tests
- Verify mock interactions when relevant

## Common Test Patterns

### Testing Pure Functions
- Provide input, verify output
- Test multiple input variations
- Test edge cases (null, empty, large values)

### Testing Async Functions
- Use async/await or framework's async utilities
- Mock network requests
- Test success and error scenarios

### Testing Components (UI)
- Test rendering without errors
- Test user interactions
- Test conditional rendering
- Avoid testing implementation details

### Testing Hooks/Composables
- Use framework-specific testing utilities
- Test state changes
- Test side effects

## Reference
- Check existing tests in the project for patterns
- Look at the test configuration file
- Review coverage reports if available
