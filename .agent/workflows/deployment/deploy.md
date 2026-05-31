---
description: Deploy applications to any platform
---

# Deploy

I will help you deploy your application to any platform.

## Guardrails
- Detect existing deployment config before suggesting
- Never deploy to production without confirmation
- Check for environment variables
- Ensure build completes successfully first

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- Where do you want to deploy? (Vercel, AWS, Railway, etc.)
- Is this production or staging?
- Any special requirements? (region, scaling, etc.)
- Existing CI/CD setup?

### 2. Detect Platform
Check existing configuration:
- `vercel.json` → Vercel
- `railway.json` → Railway
- `Dockerfile` → Container platforms
- `.github/workflows` → GitHub Actions

### 3. Prepare for Deployment
Ensure everything is ready:
- Build passes locally
- Environment variables configured
- Database migrations ready
- Static assets optimized

### 4. Configure Platform
Set up deployment:
- Connect repository
- Set environment variables
- Configure build commands
- Set up domains

### 5. Deploy
Execute deployment:
- Run deployment command
- Monitor build logs
- Verify deployment succeeded

### 6. Verify
- Check application is accessible
- Test critical functionality
- Monitor for errors

## Principles
- Always test before production
- Use environment variables for secrets
- Set up previews for PRs
