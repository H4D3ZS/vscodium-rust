---
description: Scan code for vulnerabilities, secrets, and security issues
---

# Security Audit

I will help you scan your codebase for security vulnerabilities, exposed secrets, and security best practice violations.

## Guardrails
- Never expose found secrets in output
- Check for false positives before alerting
- Prioritize high-severity issues
- Suggest fixes, not just problems

## Steps

### 1. Understand Scope
Ask clarifying questions:
- Full codebase or specific files/directories?
- Any known areas of concern?
- What type of application (web, API, CLI)?
- Are there existing security tools configured?

### 2. Check for Exposed Secrets
Search for patterns that indicate secrets:

**Common patterns:**
- API keys: `api_key`, `apiKey`, `API_KEY`
- Passwords: `password`, `passwd`, `secret`
- Tokens: `token`, `auth`, `bearer`
- Private keys: `-----BEGIN.*PRIVATE KEY-----`
- Connection strings with credentials

**Files to check:**
- `.env` files (should be gitignored)
- Config files
- Source code with hardcoded values
- Test files with real credentials

### 3. Review Authentication/Authorization
Check for common auth issues:
- Hardcoded credentials
- Missing authentication on routes
- Improper session handling
- Weak password requirements
- Missing CSRF protection

### 4. Check Input Validation
Look for injection vulnerabilities:
- SQL injection (unparameterized queries)
- XSS (unsanitized user input in HTML)
- Command injection (shell commands with user input)
- Path traversal (file paths with user input)

### 5. Review Dependencies
Check for vulnerable dependencies:
- Run `npm audit` / `yarn audit` / `pnpm audit`
- Check for outdated packages
- Look for known CVEs

### 6. Generate Report
Summarize findings:

| Severity | Issue | Location | Recommendation |
|----------|-------|----------|----------------|
| High | Exposed API key | config.js:15 | Move to env var |
| Medium | No input validation | api/users.js:42 | Add sanitization |
| Low | Outdated dependency | package.json | Update lodash |

### 7. Suggest Fixes
For each issue:
- Explain the vulnerability
- Show the problematic code
- Provide the fixed code
- Link to relevant documentation

## Severity Guidelines

| Severity | Criteria |
|----------|----------|
| **Critical** | Exposed secrets, RCE, SQL injection |
| **High** | Auth bypass, XSS, sensitive data exposure |
| **Medium** | CSRF, missing headers, weak config |
| **Low** | Info disclosure, outdated deps (no CVE) |

## Principles
- Assume all user input is malicious
- Never trust client-side validation alone
- Keep secrets out of code
- Use parameterized queries always
- Sanitize output, not just input

## Reference
- OWASP Top 10
- `npm audit` / `yarn audit`
- Check for .env.example patterns
