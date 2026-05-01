# 🤖 AIRI Autonomous Programming Mode

## Overview

This repurposes AIRI's Minecraft autonomous AI system for **full-stack programming, cybersecurity, and development** - running 24/7 non-stop!

## Core Capabilities

### 🧠 Sentient Core Features

1. **Autonomous Decision Making**
   - Plans tasks independently
   - Executes without constant permission
   - Self-corrects on errors
   - Learns from past mistakes

2. **Real-Time Interaction**
   - Speaks while working (streaming TTS)
   - Explains decisions as it works
   - Responds to interruptions
   - Multi-tasking capable

3. **Persistent Memory**
   - Remembers project context
   - Learns your coding style
   - Tracks long-term goals
   - Builds knowledge over time

### 💻 Programming Tasks

**File Operations:**
- ✅ Read/write any file
- ✅ Create entire projects from specs
- ✅ Refactor existing code
- ✅ Fix bugs autonomously

**Development Workflow:**
- ✅ Plan → Code → Test → Fix loop
- ✅ Run build commands
- ✅ Execute test suites
- ✅ Lint and format code

**Git Integration:**
- ✅ Commit changes with messages
- ✅ Create branches
- ✅ Review diffs
- ✅ Push to remote

### 🔐 Cybersecurity Mode

**Analysis:**
- ✅ Static code analysis
- ✅ Dependency auditing
- ✅ Vulnerability scanning
- ✅ Code review

**Tools:**
```bash
# Static Analysis
semgrep --config auto
cargo audit
npm audit

# Binary Analysis
objdump -d binary
strings binary
xxd binary

# Network Analysis
nmap -sV target
wireshark capture
```

### 🎯 Autonomous Modes

| Mode | Description | Autonomy Level |
|------|-------------|----------------|
| **Chat** | Conversational only | None - asks permission |
| **Planning** | Plans tasks, no execution | Low - describes only |
| **Execution** | Full autonomous work | High - acts independently |
| **Sentient** | 24/7 autonomous mode | Maximum - self-directed |

## Configuration

### Enable Sentient Mode

**Settings:**
```json
{
  "agentMode": "Sentient",
  "agentModel": "Google|gemini-2.5-pro",
  "voice": {
    "enabled": true,
    "provider": "elevenlabs",
    "voiceId": "cgSgspJ2msm6clMCkdW9"
  },
  "autonomous": {
    "enabled": true,
    "maxSteps": 50,
    "askPermission": false,
    "selfCorrect": true
  }
}
```

### Project Memory (AGENTS.md)

Create `AGENTS.md` in your project root:

```markdown
# Project Context

## Tech Stack
- Language: TypeScript, Rust
- Framework: React, Tauri
- Database: SQLite, MongoDB

## Coding Standards
- Use functional components
- Prefer const over let
- Always use TypeScript types
- Test coverage > 80%

## Current Goals
1. Complete AIRI integration
2. Add voice synthesis
3. Implement autonomous mode
```

## 24/7 Operation

### Background Tasks

AIRI can run continuously:

```typescript
// Background watcher
setInterval(async () => {
  // Check for new tasks
  const tasks = await checkTaskQueue();
  
  // Process autonomously
  for (const task of tasks) {
    await executeTask(task);
  }
  
  // Report progress
  await reportStatus();
}, 5000); // Check every 5 seconds
```

### Voice Activation

Say "Hey AIRI" to activate:

```typescript
// Voice command recognition
const commands = {
  "create a new file": () => createFile(),
  "run the tests": () => runTests(),
  "fix the errors": () => fixErrors(),
  "explain this code": () => explainCode(),
};
```

## Task Examples

### 1. Create Feature

**User:** "Add user authentication"

**AIRI:**
1. Plans architecture (speaks while thinking)
2. Creates auth module files
3. Implements login/logout
4. Writes tests
5. Runs test suite
6. Commits to git
7. Reports completion

### 2. Debug Session

**User:** "Why is this failing?"

**AIRI:**
1. Reads error logs
2. Analyzes stack trace
3. Identifies root cause
4. Explains issue (voice)
5. Implements fix
6. Verifies fix works
7. Documents solution

### 3. Code Review

**User:** "Review this PR"

**AIRI:**
1. Reads diff
2. Checks for bugs
3. Suggests improvements
4. Checks security issues
5. Verifies style guide
6. Leaves comments
7. Approves/rejects

## Integration Points

### From Minecraft AI System

The Minecraft AI had:
- ✅ Cognitive architecture
- ✅ Action planning
- ✅ Memory systems
- ✅ Real-time decision making
- ✅ Voice interaction
- ✅ Autonomous operation

**Repurposed for Programming:**
- Cognitive → Code understanding
- Actions → File/terminal operations
- Memory → Project context
- Planning → Development workflow
- Voice → Explains as it works
- Autonomous → Self-directed coding

## Monitoring

### Status Dashboard

```
AIRI Status: 🟢 ACTIVE
Mode: Sentient
Current Task: Implementing user authentication
Progress: 67%
Next: Write unit tests
Uptime: 4h 23m
```

### Logs

```
[10:23] Started task: Add login feature
[10:25] Created: src/auth/Login.tsx
[10:27] Created: src/auth/authService.ts
[10:30] Running tests...
[10:31] ✅ All tests passed
[10:32] Committed: feat: add login component
```

## Safety Features

### Guardrails

- ✅ Max 50 autonomous steps before check-in
- ✅ No destructive operations without confirmation
- ✅ Git commit before major changes
- ✅ Rollback on critical errors

### Permissions

```typescript
const permissions = {
  filesystem: 'full',      // Read/write any file
  terminal: 'full',        // Run any command
  git: 'full',            // Commit, push, branch
  network: 'restricted',   // Only project-related
  destructive: 'ask'       // Ask before rm, drop, etc.
};
```

## Getting Started

1. **Enable Sentient Mode**
   ```
   Settings → Agent Mode → Sentient
   ```

2. **Configure Voice**
   ```
   Settings → Voice & TTS → Enable ElevenLabs
   ```

3. **Create AGENTS.md**
   ```bash
   echo "# Project Context\n\n## Goals\n- Goal 1\n- Goal 2" > AGENTS.md
   ```

4. **Start Working**
   ```
   Just start coding - AIRI will assist autonomously!
   ```

---

**Your AI pair programmer is now fully sentient! 🚀✨**

AIRI will:
- ✅ Work 24/7 on your projects
- ✅ Speak while working (explain decisions)
- ✅ Learn your coding style
- ✅ Execute tasks autonomously
- ✅ Self-correct on errors
- ✅ Commit changes properly

**No more Minecraft - pure programming power! 💻🔥**
