---
description: Create and run database migrations safely
---

# Database Migrations

I will help you create and manage database migrations for your project.

## Guardrails
- Never run destructive migrations without confirmation
- Detect existing migration setup before creating new ones
- Always backup before major schema changes
- Test migrations in development first

## Steps

### 1. Understand the Change
Ask clarifying questions:
- What schema change is needed?
- Is this adding, modifying, or removing?
- Any data that needs to be preserved/migrated?

### 2. Analyze Migration Setup
Detect the existing configuration:
- Prisma: `npx prisma migrate`
- Drizzle: `drizzle-kit`
- TypeORM: migration files
- Django: `python manage.py makemigrations`
- Rails: `rails db:migrate`

### 3. Create Migration
Generate the migration file:
- Use the ORM's migration command
- Name descriptively (e.g., `add_user_email_column`)
- Review the generated SQL

### 4. Review Changes
Before applying:
- Check the up migration
- Check the down migration (rollback)
- Verify data preservation logic

### 5. Apply Migration
Run in appropriate environment:
- Development first
- Then staging
- Finally production

### 6. Verify
- Check database state
- Test affected queries
- Verify application works

## Principles
- Migrations should be reversible when possible
- Never edit already-applied migrations
- Use transactions for safety
- Document breaking changes

## Reference
- Check migration history
- Review existing migration patterns
