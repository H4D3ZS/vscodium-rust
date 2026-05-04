---
description: Manage environment variables securely
---

# Env Config

I will help you manage environment variables securely across environments.

## Guardrails
- Never commit secrets to git
- Use .env.example for documentation
- Validate required variables on startup
- Use different values per environment

## Steps

### 1. Audit Current Setup
Check existing configuration:
- `.env` files present
- `.env.example` exists
- Variables in code

### 2. Organize Variables
Group by purpose:
- **App Config**: PORT, NODE_ENV
- **Database**: DATABASE_URL
- **API Keys**: External service keys
- **Secrets**: JWT_SECRET, encryption keys

### 3. Create .env.example
Document all variables:
```
# App
NODE_ENV=development
PORT=3000

# Database
DATABASE_URL=postgresql://...

# External APIs
API_KEY=your-api-key-here
```

### 4. Secure Secrets
Best practices:
- Add `.env` to `.gitignore`
- Use secrets manager in production
- Rotate keys regularly
- Limit access to production values

### 5. Validate on Startup
Check required variables:
- Fail fast if missing
- Log which are missing
- Provide helpful error messages

### 6. Configure CI/CD
Set up for deployment:
- Use platform's secrets management
- Different values per environment
- Secure injection into builds

## Principles
- Never commit secrets
- Document all variables
- Validate early, fail fast
- Different values per environment
