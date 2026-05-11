# Qwen Code + Kortex Integration Guide

## What This Is

This setup allows **you (Qwen Code)** to assist with development while using **~99.9% fewer tokens** by leveraging the Kortex context compression system.

## How It Works

### Token Optimization Flow
```
Your Codebase (50MB+)
         ↓
NeuralDrive scans & compresses
         ↓
.aim/memory.aim (6KB gist tensor)
         ↓
AIM Proxy injects into prompt prefix
         ↓
Qwen Code receives full context
         ↓
LLM provider caches prefix
         ↓
Cost: ~1 token instead of thousands
```

## Quick Start (Before Coding Sessions)

### Option 1: Automatic Startup
```powershell
# Run this once before we start coding
cd C:\Users\HADES\Desktop\vscodium-rust\kortex
.\start-kortex.ps1
```

This script:
1. Starts Ollama (if not running)
2. Starts AIM Proxy on port 1536
3. Verifies everything is working

### Option 2: Manual Startup
```powershell
# Terminal 1: Start Ollama
ollama serve

# Terminal 2: Start AIM Proxy
cd C:\Users\HADES\Desktop\vscodium-rust\kortex
.\target\release\aim-proxy.exe
```

## Configuration Files Created

| File | Purpose |
|------|---------|
| `QWEN.md` | My configuration - tells me how to use kortex |
| `.cursorrules` | IDE integration with .aim context |
| `kortex/AGENTS.md` | General AI agent setup guide |
| `kortex/start-kortex.ps1` | One-click environment startup |
| `kortex/verify-setup.ps1` | Verification script |

## How I Use This

### When I'm Helping You Code

1. **Context Loading**: The AIM proxy automatically injects `.aim` context into my prompts
2. **Full Codebase Understanding**: I can reference any file in mounted projects
3. **Token Efficiency**: 50MB codebase = ~1 token instead of 10,000+

### My Workflow

```
1. You start kortex: .\start-kortex.ps1
2. You mount project in NeuralDrive GUI
3. You ask me to help with code
4. I receive compressed context automatically
5. I provide informed assistance
6. Token cost: minimal
```

## Available Tools

### Built Executables
- `aim-proxy.exe` - Context injection (runs in background)
- `neuraldrive.exe` - 3D code visualization GUI
- `hades-tui.exe` - Terminal dashboard (optional)
- `aim-vfs.exe` - VFS daemon

### My Permitted Commands
Already configured in `.qwen/settings.json`:
- `cargo build/check/clean`
- `git` operations
- `powershell` scripts
- File editing operations

## Memory Locations

I check these paths for `.aim` context files:
```
C:\Users\HADES\Desktop\kortex\.aim\memory.aim
C:\Users\HADES\Desktop\vscodium-rust\.aim\memory.aim
.<project>\.aim\memory.aim
```

## Verification

Run this anytime to check status:
```powershell
.\kortex\verify-setup.ps1
```

Expected output:
```
✓ aim-proxy.exe (4.3 MB)
✓ neuraldrive.exe (14.17 MB)
✓ hades-tui.exe (0.47 MB)
✓ aim-vfs.exe (0.37 MB)
✓ Ollama is running on port 11434
✓ AIM Proxy is running on port 1536
```

## For Qwen Code Specifically

**I am Qwen Code**, not Claude. The configuration is set up as:

- **`QWEN.md`** - My primary configuration file
- **`.qwen/settings.json`** - My permissions
- **`CLAUDE.MD`** - For compatibility if Claude accesses this project

### My Identity in This Project
- **Role:** AI development assistant
- **Context:** HADES-KORTEX cybersecurity research
- **Philosophy:** Wu Wei (effortless action), Socratic logic
- **Constraints:** Windows-native, Rust/Tauri, <150MB footprint

## Troubleshooting

### AIM Proxy Not Starting
```powershell
# Check if port 1536 is in use
netstat -ano | findstr :1536

# Kill process if needed
taskkill /F /PID <PID>

# Restart proxy
cd kortex && .\target\release\aim-proxy.exe
```

### Ollama Connection Issues
```powershell
# Restart Ollama
taskkill /F /IM ollama.exe
ollama serve
```

### Context Not Loading
1. Open NeuralDrive: `.\target\release\neuraldrive.exe`
2. Click "Mount Project"
3. Select the codebase folder
4. Wait for scan to complete
5. Check `.aim/memory.aim` exists

## Summary

**Before our coding sessions:**
```powershell
cd C:\Users\HADES\Desktop\vscodium-rust\kortex
.\start-kortex.ps1
```

**Then:**
- Open NeuralDrive GUI to mount projects
- Ask me to help with code
- I have full context at ~1 token cost
- We can work on massive codebases affordably

---

**Ready for zero-token development with Qwen Code!** 🚀
