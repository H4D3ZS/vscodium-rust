---
description: Set up CI/CD pipelines for any platform
---

# CI/CD

I will help you set up continuous integration and deployment pipelines.

## Guardrails
- Detect existing CI/CD before adding new
- Start simple, iterate as needed
- Keep secrets secure
- Test pipelines in branches first

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What platform? (GitHub Actions, GitLab CI, etc.)
- What should the pipeline do? (test, build, deploy)
- Any existing CI/CD setup?
- Multiple environments? (staging, production)

### 2. Detect Platform
Check existing configuration:
- `.github/workflows/` → GitHub Actions
- `.gitlab-ci.yml` → GitLab CI
- `Jenkinsfile` → Jenkins
- `bitbucket-pipelines.yml` → Bitbucket

### 3. Design Pipeline
Plan stages:
- **CI**: Lint, test, build
- **CD**: Deploy to staging/production
- **Conditions**: When to run each stage

### 4. Create Pipeline
Configure jobs:
- Set triggers (push, PR, schedule)
- Define steps
- Add caching for speed
- Set up secrets

### 5. Test Pipeline
- Run on a branch first
- Verify all steps pass
- Check deployment works

### 6. Verify
- Pipeline runs on expected triggers
- Deployments succeed
- Notifications work

## Principles
- Fail fast (run quick checks first)
- Cache dependencies
- Keep pipelines DRY
- Use reusable workflows
