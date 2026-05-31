---
description: Add strategic logging and debugging statements
---

# Debug Log

I will help you add strategic logging to diagnose issues in your code.

## Guardrails
- Don't log sensitive data (passwords, tokens, PII)
- Use appropriate log levels
- Make logs machine-parseable when needed
- Clean up debug logs before production

## Steps

### 1. Understand the Issue
Ask clarifying questions:
- What behavior are you trying to understand?
- Where do you suspect the issue is?
- What data would help diagnose it?
- Is this for development or production?

### 2. Detect Logging Setup
Check existing configuration:
- Node.js: console, winston, pino
- Python: logging module, loguru
- Check for existing log patterns

### 3. Add Strategic Logs
Place logs at key points:
- Function entry/exit
- Before/after external calls
- Decision branches
- Error catch blocks

### 4. Include Useful Context
Log relevant data:
- Input parameters
- State changes
- Timing information
- Error details with stack traces

### 5. Use Appropriate Levels
- **DEBUG**: Detailed diagnostic info
- **INFO**: General operational events
- **WARN**: Potential issues
- **ERROR**: Actual errors

### 6. Verify
- Run the code path
- Check logs contain needed info
- Remove debug logs when done

## Principles
- Log the "why" not just the "what"
- Include correlation IDs for tracing
- Make logs searchable
