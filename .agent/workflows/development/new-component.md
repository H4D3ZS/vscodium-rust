---
description: Create reusable UI components for any frontend framework
---

# New Component

I will help you create a well-structured, reusable UI component.

## Guardrails
- Never assume a specific framework — detect it first
- Follow the project's existing component patterns and styling
- Don't generate code until the stack is understood
- Single responsibility — one component, one purpose

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What is the component's name and purpose?
- What props/inputs does it need?
- Does it need internal state?
- Are there similar components in the codebase to reference?

### 2. Analyze Project Stack
Detect the existing setup:
- **Framework**: Check for React, Vue, Angular, Svelte, etc.
- **Language**: TypeScript or JavaScript?
- **Styling**: Tailwind, CSS Modules, Styled Components, Sass, vanilla CSS?
- **Component Location**: Where do existing components live?

Look at `package.json`, config files, and existing components.
If unclear, ask the user.

### 3. Study Existing Patterns
Before creating:
- Find 1-2 similar components in the codebase
- Note their file structure, naming conventions, and prop patterns
- Match the existing export style (named vs default)
- Follow existing typing patterns if TypeScript is used

### 4. Create Component
Based on detected stack and existing patterns:
- Create the component file in the appropriate directory
- Use the project's styling approach
- Include proper typing if TypeScript is used
- Add accessibility attributes (ARIA) where appropriate
- Export the component following project conventions

### 5. Export Component
If the project uses barrel files (index.ts), update them to include the new component.

### 6. Verify
- Component renders without errors
- Props work as expected
- Styling is consistent with existing components
- Accessibility is maintained

## Principles
- Match existing patterns — consistency over personal preference
- Keep it simple — start minimal, expand if needed
- Accessibility first — keyboard navigation, screen reader support
- Type safety — use proper types if TypeScript is present

## Reference
- Check `package.json` for framework and styling dependencies
- Look at existing components for patterns to follow
- Use `rg` to find similar components in the codebase
