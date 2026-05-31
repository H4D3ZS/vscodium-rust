---
description: Generate seed and test data for databases
---

# Database Seeding

I will help you generate realistic seed data for your database.

## Guardrails
- Never seed production databases without explicit confirmation
- Detect existing seed patterns before creating new ones
- Use realistic but fake data (no real PII)
- Make seeds idempotent when possible

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What tables need seeding?
- How much data is needed?
- Should data be realistic or simple?
- Any specific relationships to maintain?

### 2. Analyze Seed Setup
Detect existing configuration:
- Prisma: `prisma/seed.ts`
- Drizzle: seed scripts
- Django: fixtures or management commands
- Rails: `db/seeds.rb`

### 3. Design Seed Data
Plan the data structure:
- Required entities in dependency order
- Relationships between entities
- Realistic field values
- Edge cases for testing

### 4. Generate Data
Create seed scripts that:
- Clear existing data (optional, with warning)
- Create records in correct order
- Maintain referential integrity
- Use faker/chance for realistic data

### 5. Run Seeds
Execute in development:
- Verify no errors
- Check data in database
- Test application with seeded data

## Principles
- Seed in dependency order (users before posts)
- Use consistent IDs for testing
- Include edge cases (empty strings, nulls)
- Make seeds repeatable

## Reference
- Check existing seed files
- Look at schema for required fields
