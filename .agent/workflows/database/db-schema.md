---
description: Design database schemas for any ORM or database
---

# Database Schema Design

I will help you design database schemas that adapt to your project's ORM and database.

## Guardrails
- Never assume a specific ORM (Prisma, Drizzle, TypeORM, SQLAlchemy, etc.)
- Detect existing database setup before suggesting schemas
- Consider relationships, indexes, and constraints
- Follow naming conventions from existing schema

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What entities/tables are needed?
- What are the relationships between them?
- Any specific fields or constraints required?
- What's the expected data volume?

### 2. Analyze Database Setup
Detect the existing configuration:
- Check for `prisma/schema.prisma`
- Check for `drizzle.config.ts`
- Check for SQLAlchemy models, TypeORM entities
- Look at existing tables/models for patterns

If no existing setup, ask which ORM/database they prefer.

### 3. Design Schema
For each entity, define:
- Table/model name (follow naming conventions)
- Fields with appropriate types
- Primary keys and unique constraints
- Foreign keys and relationships
- Indexes for frequently queried fields
- Timestamps (createdAt, updatedAt)

### 4. Define Relationships
Establish connections between entities:
- One-to-one
- One-to-many
- Many-to-many (with junction tables)

### 5. Add Constraints
Include data integrity rules:
- NOT NULL where required
- UNIQUE for emails, usernames
- CHECK constraints for valid values
- CASCADE rules for deletions

### 6. Verify
- Review schema for completeness
- Check for potential N+1 query issues
- Ensure indexes cover common queries

## Principles
- Normalize to 3NF unless performance requires denormalization
- Use appropriate field types (don't store numbers as strings)
- Add indexes on foreign keys and frequently filtered columns
- Consider soft deletes for important data

## Reference
- Check existing schema files
- Look at migration history
