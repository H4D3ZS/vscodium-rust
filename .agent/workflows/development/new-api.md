---
description: Create API endpoints for any backend framework
---

# New API Endpoint

I will help you create a robust API endpoint for your project.

## Guardrails
- Never assume a specific backend framework — detect it first
- Follow the project's existing API patterns and conventions
- Don't generate code until the stack is understood
- Always include input validation and error handling

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What HTTP methods does this endpoint support? (GET, POST, PUT, DELETE)
- What is the URL path/route?
- What data does it accept and return?
- Does it require authentication?
- What resource/entity does it operate on?

### 2. Analyze Project Stack
Detect the existing setup:
- **Framework**: Next.js (App/Pages Router), Express, NestJS, FastAPI, Django, Go, etc.
- **Language**: TypeScript, JavaScript, Python, Go, etc.
- **Validation**: Zod, Joi, Yup, Pydantic, etc.
- **Database/ORM**: Prisma, TypeORM, SQLAlchemy, GORM, etc.
- **Auth**: NextAuth, Passport, JWT, etc.

Check `package.json`, `requirements.txt`, `go.mod`, or config files.
If unclear, ask the user.

### 3. Study Existing Patterns
Before creating:
- Find 1-2 existing API routes in the codebase
- Note the file structure and naming conventions
- Match the existing error handling approach
- Follow existing response format patterns
- Check how authentication is handled in other routes

### 4. Create Validation Schema
Based on detected validation library:
- Define input validation for request body/params
- Include appropriate error messages
- Use existing validation patterns in the project

### 5. Create Route Handler
Based on detected framework and existing patterns:
- Create the route file in the appropriate directory
- Implement the requested HTTP method(s)
- Add input validation
- Include proper error handling with appropriate status codes
- Add authentication check if required
- Return consistent response format

### 6. Verify
- Endpoint responds correctly to valid requests
- Validation rejects invalid input with helpful errors
- Error cases return appropriate status codes
- Authentication works if required

## Principles
- Consistent responses — follow existing API response format
- Validate everything — never trust client input
- Fail gracefully — return helpful error messages
- Secure by default — validate auth, sanitize inputs

## Reference
- Check existing API routes for patterns
- Look at `package.json` or equivalent for framework detection
- Use `rg` to find similar endpoints in the codebase
