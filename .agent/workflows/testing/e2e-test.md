---
description: End-to-end browser tests with detected framework
---

# E2E Test

I will help you write end-to-end tests that adapt to your testing framework.

## Guardrails
- Detect existing E2E setup before writing tests
- Test user flows, not implementation
- Keep tests reliable and not flaky
- Use proper waiting strategies

## Steps

### 1. Understand What to Test
Ask clarifying questions:
- What user flow needs testing?
- What are the critical paths?
- Any known flaky areas?
- What browsers need support?

### 2. Detect E2E Framework
Check existing setup:
- Playwright: `playwright.config.ts`
- Cypress: `cypress.config.js`
- Selenium: webdriver setup
- Puppeteer: puppeteer usage

If none, recommend Playwright for new projects.

### 3. Write Test Cases
Structure tests around user flows:
- Describe the scenario clearly
- Set up preconditions
- Perform user actions
- Assert expected outcomes

### 4. Handle Async Operations
Use proper waiting:
- Wait for elements to be visible
- Wait for network requests
- Avoid arbitrary timeouts
- Use retry mechanisms

### 5. Run and Verify
- Execute tests in headed mode first
- Debug any failures
- Run in CI environment

## Principles
- Test what users do, not how code works
- Make tests deterministic
- Isolate test data
- Clean up after tests
