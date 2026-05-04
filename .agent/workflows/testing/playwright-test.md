---
description: Write robust Playwright browser automation tests for web applications
---

# Playwright Browser Testing

I will help you set up Playwright and write effective browser tests.

## Guardrails
- Always wait for elements before interacting with them
- Use semantic selectors (role, text) over CSS selectors when possible
- Keep tests isolated and independent
- Use `data-testid` attributes for stable element selection

## Steps

### 1. Check/Install Playwright
// turbo
```bash
npm init playwright@latest
```

If already installed, skip to step 2.

### 2. Configure Playwright

Verify `playwright.config.ts` has these key settings:
```typescript
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  retries: process.env.CI ? 2 : 0,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
  ],
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
```

### 3. Create Test File

Create a new test in `tests/<feature>.spec.ts`:
```typescript
import { test, expect } from '@playwright/test';

test.describe('Feature Name', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display the main heading', async ({ page }) => {
    await expect(page.getByRole('heading', { level: 1 })).toBeVisible();
  });

  test('should navigate to about page', async ({ page }) => {
    await page.getByRole('link', { name: 'About' }).click();
    await expect(page).toHaveURL('/about');
  });
});
```

### 4. Add Common Selectors

For stable tests, use these selector strategies (in order of preference):

```typescript
// Best: Role-based
page.getByRole('button', { name: 'Submit' })
page.getByRole('textbox', { name: 'Email' })

// Good: Data-testid
page.getByTestId('submit-button')

// Good: Label-based
page.getByLabel('Email address')

// Acceptable: Text-based
page.getByText('Welcome back')

// Last resort: CSS selector
page.locator('.submit-btn')
```

### 5. Run Tests
// turbo
```bash
npx playwright test
```

Run with UI mode for debugging:
```bash
npx playwright test --ui
```

### 6. View Report
// turbo
```bash
npx playwright show-report
```

## Common Patterns

### Wait for Network Idle
```typescript
await page.goto('/', { waitUntil: 'networkidle' });
```

### Fill Form and Submit
```typescript
await page.getByLabel('Email').fill('test@example.com');
await page.getByLabel('Password').fill('password123');
await page.getByRole('button', { name: 'Sign In' }).click();
```

### Assert Toast/Notification
```typescript
await expect(page.getByRole('alert')).toContainText('Success');
```

### Take Screenshot
```typescript
await page.screenshot({ path: 'screenshot.png', fullPage: true });
```

## Guidelines
- Name tests descriptively: "should [action] when [condition]"
- Use `test.describe` blocks to group related tests
- Avoid hardcoded waits (`page.waitForTimeout`) — use proper assertions
- Run tests in CI/CD pipeline

## Reference
- [Playwright Docs](https://playwright.dev/docs/intro)
- Use `npx playwright codegen` to generate test code interactively
