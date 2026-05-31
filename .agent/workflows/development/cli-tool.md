---
description: Build command-line applications with argument parsing
---

# CLI Tool

I will help you build a command-line application that adapts to your language.

## Guardrails
- Detect existing project language before suggesting
- Follow CLI best practices (help, version, exit codes)
- Make it user-friendly with clear error messages
- Support both interactive and non-interactive modes

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What should the CLI do?
- What commands/subcommands are needed?
- What arguments and flags are required?
- Should it be interactive or purely command-based?

### 2. Detect Language
Check project setup:
- Node.js: package.json → use Commander or Yargs
- Python: use argparse or Click
- Go: use Cobra
- Rust: use Clap

### 3. Set Up CLI Structure
Create entry point and command structure:
- Main entry point with argument parsing
- Subcommands for different features
- Help text for all commands
- Version flag

### 4. Implement Commands
For each command:
- Parse arguments and options
- Validate inputs
- Execute logic
- Handle errors gracefully
- Output results clearly

### 5. Add User Experience
- Colorful output (chalk, rich, etc.)
- Progress indicators for long operations
- Clear error messages
- Interactive prompts when needed

### 6. Verify
- Test all commands
- Check help output
- Verify error handling

## Principles
- Fail fast with clear error messages
- Support both flags and environment variables
- Follow platform conventions
