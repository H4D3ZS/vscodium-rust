---
description: Generate API documentation (OpenAPI, JSDoc, etc.)
---

# API Docs

I will help you generate API documentation for your project.

## Guardrails
- Document from actual code, not assumptions
- Keep docs in sync with implementation
- Use standard formats (OpenAPI, JSDoc)
- Include examples for all endpoints

## Steps

### 1. Analyze API
Gather information:
- List all endpoints/functions
- Identify request/response formats
- Note authentication requirements
- Find existing documentation

### 2. Choose Format
Based on project type:
- **REST API**: OpenAPI/Swagger
- **GraphQL**: Schema + descriptions
- **Libraries**: JSDoc, TSDoc, docstrings
- **CLI**: Man pages or markdown

### 3. Document Endpoints
For each endpoint/function:
- Method and path
- Description of what it does
- Request parameters and body
- Response format and codes
- Authentication requirements
- Example requests and responses

### 4. Generate Docs
Use appropriate tools:
- OpenAPI: swagger-ui, redoc
- JSDoc: jsdoc, typedoc
- Python: Sphinx, mkdocs

### 5. Verify
- All endpoints documented
- Examples work correctly
- Types are accurate

## Principles
- Document the "why" not just the "what"
- Include realistic examples
- Keep docs close to code
