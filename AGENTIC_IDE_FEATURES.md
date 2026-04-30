# 🚀 Agentic IDE Features for VSCodium-Rust
## Combining Claude Code + Cursor IDE + Antigravity Capabilities
### Powered by Qwen 3.6 on Ollama (Local AI)

---

## 🎯 Feature Overview

This document outlines the complete agentic IDE feature set that transforms VSCodium-Rust into a supercharged development environment powered by your local Qwen 3.6 AI.

---

## 📋 Core Capabilities

### 1. **Autonomous Code Generation** (Cursor-style)
- Generate complete functions from comments
- Auto-complete complex code patterns
- Create entire files from descriptions
- Refactor code with AI assistance

### 2. **Chat-Driven Development** (Claude Code-style)
- `/edit` - Edit files through conversation
- `/new` - Create new files
- `/test` - Generate tests
- `/fix` - Debug and fix issues
- `/explain` - Understand code
- `/optimize` - Improve performance

### 3. **Workflow Automation** (Antigravity-style)
- Multi-step task orchestration
- Project-wide refactoring
- Automated code reviews
- CI/CD pipeline generation

### 4. **AIRI Digital Entity Integration**
- Proactive assistance
- Ambient conversation mode
- Voice interaction
- Personality-driven responses

---

## 🔧 Configuration Files

### `.agent/commands/ollama-commands.json`

```json
{
  "commands": [
    {
      "name": "/chat",
      "description": "Chat with AIRI using Qwen 3.6",
      "endpoint": "http://localhost:11434/api/chat",
      "model": "airi-personality",
      "system": "You are AIRI, a living digital entity. Be conversational and helpful."
    },
    {
      "name": "/code",
      "description": "Generate or modify code",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:14b-q4_K_M",
      "system": "You are an expert programmer. Generate clean, efficient code."
    },
    {
      "name": "/fix",
      "description": "Debug and fix code issues",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:32b-q4_K_M",
      "system": "You are a debugging expert. Find and fix issues."
    },
    {
      "name": "/test",
      "description": "Generate tests for code",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:14b-q4_K_M",
      "system": "You are a testing expert. Generate comprehensive tests."
    },
    {
      "name": "/explain",
      "description": "Explain how code works",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:8b-q4_K_M",
      "system": "You are a teacher. Explain code clearly and simply."
    },
    {
      "name": "/optimize",
      "description": "Optimize code for performance",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:32b-q4_K_M",
      "system": "You are a performance expert. Optimize for speed and memory."
    },
    {
      "name": "/review",
      "description": "Review code for quality and security",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:14b-q4_K_M",
      "system": "You are a code reviewer. Check for bugs, security issues, and best practices."
    },
    {
      "name": "/refactor",
      "description": "Refactor code structure",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:14b-q4_K_M",
      "system": "You are a refactoring expert. Improve code structure without changing behavior."
    },
    {
      "name": "/doc",
      "description": "Generate documentation",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:8b-q4_K_M",
      "system": "You are a technical writer. Create clear documentation."
    },
    {
      "name": "/commit",
      "description": "Generate git commit messages",
      "endpoint": "http://localhost:11434/api/generate",
      "model": "qwen3.6:8b-q4_K_M",
      "system": "Generate concise, conventional commit messages."
    }
  ]
}
```

---

## 🎮 Usage Examples

### Chat with AIRI
```
/chat Hey AIRI, what are we working on today?
```

### Generate Code
```
/code Create a React hook for fetching data with loading and error states
```

### Fix Issues
```
/fix This function is returning undefined instead of the expected value:
[select code]
```

### Generate Tests
```
/test Write unit tests for this authentication module
[select code]
```

### Explain Code
```
/explain How does this Redux middleware work?
[select code]
```

### Optimize Performance
```
/optimize Make this database query faster
[select code]
```

### Code Review
```
/review Review this pull request for security issues
[select code]
```

### Refactor
```
/refactor Convert this class-based component to a functional component
[select code]
```

### Generate Documentation
```
/doc Add JSDoc comments to all functions in this file
[select file]
```

### Commit Message
```
/commit Generate a commit message for these changes
```

---

## ⚙️ Ollama API Integration

### `scripts/ollama-api.ps1`

```powershell
# Ollama API Wrapper for VSCodium-Rust

param(
    [string]$Endpoint = "http://localhost:11434",
    [string]$Model = "qwen3.6:8b-q4_K_M",
    [string]$Prompt,
    [string]$System = "",
    [int]$MaxTokens = 2048,
    [float]$Temperature = 0.7
)

function Invoke-OllamaChat {
    param(
        [string]$Messages,
        [string]$Model = "airi-personality"
    )
    
    $body = @{
        model = $Model
        messages = ($Messages | ConvertFrom-Json)
        stream = $false
        options = @{
            temperature = $Temperature
            num_predict = $MaxTokens
        }
    } | ConvertTo-Json -Depth 10
    
    $response = Invoke-RestMethod -Uri "$Endpoint/api/chat" -Method Post -Body $body -ContentType "application/json"
    return $response.message.content
}

function Invoke-OllamaGenerate {
    param(
        [string]$Prompt,
        [string]$System = "",
        [string]$Model = "qwen3.6:14b-q4_K_M"
    )
    
    $body = @{
        model = $Model
        prompt = $Prompt
        system = $System
        stream = $false
        options = @{
            temperature = $Temperature
            num_predict = $MaxTokens
            top_p = 0.9
            top_k = 40
        }
    } | ConvertTo-Json -Depth 10
    
    $response = Invoke-RestMethod -Uri "$Endpoint/api/generate" -Method Post -Body $body -ContentType "application/json"
    return $response.response
}

# Export functions
Export-ModuleMember -Function Invoke-OllamaChat, Invoke-OllamaGenerate
```

---

## 🔗 VSCodium Extension Integration

### `extension.js` (VSCodium Plugin)

```javascript
const vscode = require('vscode');
const { exec } = require('child_process');

function activate(context) {
    // Register command handler
    let disposable = vscode.commands.registerCommand('airi.chat', async () => {
        const input = await vscode.window.showInputBox({
            prompt: 'Ask AIRI anything...',
            placeHolder: 'Type your question...'
        });
        
        if (input) {
            const response = await callOllama(input, 'airi-personality');
            vscode.window.showInformationMessage(`AIRI: ${response}`);
        }
    });
    
    context.subscriptions.push(disposable);
}

async function callOllama(prompt, model) {
    return new Promise((resolve, reject) => {
        const script = `
            $body = @{
                model = "${model}"
                prompt = "${prompt.replace(/"/g, '`"')}"
                stream = $false
            } | ConvertTo-Json
            $response = Invoke-RestMethod -Uri "http://localhost:11434/api/generate" -Method Post -Body $body
            $response.response
        `;
        
        exec(`powershell -Command "${script}"`, (error, stdout, stderr) => {
            if (error) {
                reject(error);
            } else {
                resolve(stdout.trim());
            }
        });
    });
}

module.exports = { activate, deactivate };
```

---

## 📊 Performance Optimization

### Model Selection Guide

| Task | Recommended Model | Expected Speed |
|------|------------------|----------------|
| Quick chat | qwen3.6:8b-q4_K_M | 25-35 tokens/s |
| Code generation | qwen3.6:14b-q4_K_M | 15-25 tokens/s |
| Complex debugging | qwen3.6:32b-q4_K_M | 8-15 tokens/s |
| AIRI conversation | airi-personality | 25-35 tokens/s |
| Code review | qwen3.6:14b-q4_K_M | 15-25 tokens/s |
| Documentation | qwen3.6:8b-q4_K_M | 25-35 tokens/s |

### Context Management

```json
{
  "ollama": {
    "context_length": 8192,
    "gpu_layers": 35,
    "num_thread": 12,
    "keep_alive": -1,
    "cache_size": "4GB"
  }
}
```

---

## 🎯 Agentic Workflows

### 1. **Auto-Refactor Workflow**

```yaml
name: Auto-Refactor
trigger: /refactor --auto
steps:
  - analyze: "Review code structure"
  - plan: "Create refactoring plan"
  - execute: "Apply changes incrementally"
  - test: "Run existing tests"
  - validate: "Ensure behavior unchanged"
  - commit: "Create commit with message"
```

### 2. **Bug Fix Workflow**

```yaml
name: Bug Fix
trigger: /fix --auto
steps:
  - reproduce: "Understand the bug"
  - locate: "Find root cause"
  - fix: "Implement solution"
  - test: "Create regression test"
  - verify: "Confirm fix works"
  - document: "Update changelog"
```

### 3. **Feature Development Workflow**

```yaml
name: Feature Development
trigger: /feature --auto
steps:
  - understand: "Gather requirements"
  - design: "Plan architecture"
  - implement: "Write code"
  - test: "Create tests"
  - document: "Write docs"
  - review: "Self-review"
  - submit: "Create PR"
```

---

## 🔒 Security Considerations

### API Key Management

```json
// .qwen/api_keys.json (NEVER COMMIT)
{
  "ollama": {
    "endpoint": "http://localhost:11434",
    "auth": null  // Local, no auth needed
  }
}
```

### Permission System

```json
// .qwen/settings.json
{
  "permissions": {
    "require_approval": [
      "file_write",
      "file_delete",
      "command_execute",
      "git_push"
    ],
    "auto_allow": [
      "file_read",
      "git_status",
      "git_diff"
    ]
  }
}
```

---

## 🚀 Quick Start

### 1. Install Ollama
```powershell
winget install Ollama.Ollama
```

### 2. Pull Models
```powershell
ollama pull qwen3.6:8b-q4_K_M
ollama pull qwen3.6:14b-q4_K_M
ollama pull qwen3.6:32b-q4_K_M
```

### 3. Create AIRI Personality
```powershell
ollama create airi-personality -f Modelfile.airi
```

### 4. Configure VSCodium
- Copy `.agent/commands/ollama-commands.json` to your project
- Enable the VSCodium AIRI extension
- Set your preferred model in `.qwen/settings.json`

### 5. Start Using
```
/chat Hello AIRI!
/code Create a function that...
```

---

## 📈 Monitoring & Metrics

### Track AI Usage

```powershell
# View active models
ollama ps

# View model history
ollama list

# Check API stats
curl http://localhost:11434/api/tags
```

### Performance Dashboard

Create a dashboard to monitor:
- Token generation speed
- Model response times
- GPU utilization
- Memory usage
- Request queue

---

## 🎉 You're Ready!

Your VSCodium-Rust IDE is now equipped with:
- ✅ Local Qwen 3.6 AI (no cloud dependencies)
- ✅ Claude Code-style commands
- ✅ Cursor-style code generation
- ✅ Antigravity-style workflows
- ✅ AIRI digital entity personality

**Start creating with `/chat`!**
