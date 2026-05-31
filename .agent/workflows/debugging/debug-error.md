---
description: Analyze error messages and stack traces to suggest fixes
---

# Debug Error

I will help you analyze errors and stack traces to identify the root cause and suggest fixes.

## Guardrails
- Focus on the actual error, not symptoms
- Check for common causes before diving deep
- Suggest fixes that match the project's patterns
- Don't modify code without user confirmation

## Steps

### 1. Gather Error Information
Ask for details:
- What is the full error message?
- What is the stack trace (if available)?
- What action triggered the error?
- Is this a new issue or regression?
- Can the error be reproduced consistently?

### 2. Analyze the Error
Parse the error information:

**From the error message:**
- Error type (TypeError, SyntaxError, NetworkError, etc.)
- Error description
- Affected file and line number

**From the stack trace:**
- Entry point of the error
- Call sequence leading to the error
- External vs internal code

### 3. Identify Common Causes

| Error Type | Common Causes |
|------------|---------------|
| TypeError | Null/undefined access, wrong type passed |
| ReferenceError | Undefined variable, scope issues |
| SyntaxError | Typos, missing brackets, invalid syntax |
| NetworkError | API down, CORS, wrong URL, auth issues |
| ModuleNotFound | Missing dependency, wrong import path |

### 4. Investigate the Code
Look at the relevant code:
- Read the file and line mentioned in the error
- Check recent changes if it's a regression
- Look for related code that might affect this

### 5. Suggest Fixes
Provide solutions ranked by likelihood:

**For each fix:**
- Explain what's wrong
- Show the problematic code
- Provide the fixed code
- Explain why this fixes the issue

### 6. Verify Fix
After applying a fix:
- Run the code again
- Check if the error is resolved
- Look for any new errors

## Principles
- Read the error message carefully—it often tells you exactly what's wrong
- Check the most recent change first for regressions
- Consider edge cases (null, empty, undefined)
- Look for typos in variable/function names

## Reference
- Search codebase for similar patterns
- Check documentation for the API/library involved
- Look at recent commits if it's a regression
