---
description: Check for vulnerable dependencies and suggest upgrades
---

# Dependency Check

I will help you identify vulnerable dependencies and safely upgrade them.

## Guardrails
- Don't auto-upgrade without user confirmation
- Check for breaking changes before suggesting upgrades
- Prioritize by severity
- Test after upgrades

## Steps

### 1. Detect Package Manager
Check which package manager is used:
- `package-lock.json` → npm
- `yarn.lock` → yarn
- `pnpm-lock.yaml` → pnpm
- `bun.lockb` → bun
- `requirements.txt` / `Pipfile` → Python
- `go.mod` → Go

### 2. Run Security Audit

**JavaScript/Node.js:**
```bash
npm audit
# or
yarn audit
# or
pnpm audit
```

**Python:**
```bash
pip-audit
# or
safety check
```

### 3. Analyze Results
For each vulnerability:
- Package name and version
- Severity (critical, high, moderate, low)
- CVE identifier if available
- Fixed version

### 4. Prioritize Fixes

| Priority | Action |
|----------|--------|
| Critical/High | Fix immediately |
| Moderate | Fix soon |
| Low | Fix when convenient |

### 5. Check for Breaking Changes
Before upgrading:
- Check package changelog
- Look for major version bumps
- Review migration guides if available
- Check if direct or transitive dependency

### 6. Suggest Safe Upgrades
For each vulnerable package:

**Direct dependencies:**
- Show current vs fixed version
- Highlight if major version change
- Note any breaking changes

**Transitive dependencies:**
- Identify which direct dependency pulls it in
- Suggest upgrading the parent package

### 7. Apply Fixes

**For npm:**
```bash
npm audit fix           # safe fixes only
npm audit fix --force   # all fixes (may break)
```

**For manual updates:**
- Update package.json
- Run install
- Test the application

### 8. Verify Fixes
After upgrading:
- Run audit again to confirm fixes
- Run tests to catch breakages
- Check application functionality

## Principles
- Keep dependencies updated regularly
- Use lockfiles for reproducible builds
- Review what you're installing
- Prefer packages with active maintenance

## Reference
- npm/yarn/pnpm audit documentation
- CVE database for vulnerability details
- Package changelogs for upgrade notes
