---
description: Technology migrations like JS to TS or framework upgrades
---

# Migrate

I will help you migrate your codebase to new technologies safely.

## Guardrails
- Never migrate everything at once
- Maintain backwards compatibility during migration
- Test thoroughly at each step
- Have a rollback plan

## Steps

### 1. Understand Migration
Ask clarifying questions:
- What are you migrating from/to?
- What's the scope? (full codebase or incremental)
- Are there breaking changes to handle?
- What's the timeline?

### 2. Analyze Current State
Before migrating:
- Document current dependencies
- Identify affected files
- Check for known migration issues
- Review migration guides

### 3. Create Migration Plan
Common migrations:
- **JS → TypeScript**: Add tsconfig, rename files, add types
- **Framework upgrade**: Check changelog, update deps, fix breaking changes
- **CSS → Tailwind**: Install, configure, convert styles
- **REST → GraphQL**: Add schema, create resolvers, update clients

### 4. Execute Migration
Incremental approach:
- Start with low-risk files
- Migrate in logical batches
- Test after each batch
- Fix issues as they arise

### 5. Verify
- All tests pass
- Application works correctly
- No regressions

## Principles
- Migrate incrementally, not all at once
- Keep the application working throughout
- Document breaking changes
