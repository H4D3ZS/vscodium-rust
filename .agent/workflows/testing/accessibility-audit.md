---
description: Perform a comprehensive accessibility (a11y) audit for any stack
---

# Accessibility Audit

I will help you audit your web application for accessibility issues and provide actionable steps to improve it, adapting to your project's technology and patterns.

## Guardrails
- Always detect the stack first — different frameworks have different linting/scanning tools
- Don't assume the presence of specialized audit tools (like Axe or Lighthouse) unless detected
- Prioritize semantic HTML fixes over ARIA attributes where possible
- Focus on WCAG 2.1 AA standards as the baseline
- Avoid suggesting changes that break existing project styles or patterns

## Steps

### 1. Understand Context
Ask clarifying questions to understand the scope:
- What specific page, component, or URL should be audited?
- Are there specific accessibility concerns (e.g., keyboard navigation, screen reader support, color contrast)?
- Is there a specific compliance target (e.g., Section 508, WCAG 2.1 AA)?

### 2. Analyze Project
Detect the project setup and available tools:
- **Framework**: Check for React, Vue, Svelte, or static HTML in `package.json`.
- **A11y Tools**: Look for `eslint-plugin-jsx-a11y`, `axe-core`, or `jest-axe`.
- **Component Patterns**: Identify the existing component structure and styling approach (Tailwind, CSS Modules, etc.).

### 3. Perform Audit
Based on the detected stack, perform the audit:
- **Automated Scan**: If appropriate tools are detected, run them (e.g., `npm run lint` or `axe`).
- **Semantic Review**: Inspect the HTML structure for correct use of `header`, `main`, `nav`, and heading levels.
- **Interactive Elements**: Check focus management, keyboard accessibility, and ARIA labels.
- **Findings**: Categorize issues by severity (Critical, Major, Minor).

### 4. Verify
Confirm the audit quality and suggested fixes:
- Ensure suggested fixes follow the project's coding style.
- Verify that keyboard navigation paths are logically documented.
- Confirm that suggested ARIA roles match the elements' intended behavior.

## Principles
- **Semantic First**: Use native HTML elements before reaching for ARIA.
- **Progressive Enhancement**: Ensure basic functionality is accessible to all.
- **Consistency**: Match the project's existing patterns and styling tokens.

## Reference
- Check `package.json` for accessibility-related linting rules.
- Look at existing components for how they handle focus and labels.
- Use `rg` to find existing uses of ARIA roles or landmark elements.
- [WCAG 2.1 Quick Reference](https://www.w3.org/WAI/WCAG21/quickref/)
