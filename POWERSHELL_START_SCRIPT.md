# ✅ Updated: PowerShell Script for Multi-Service Start

## Problem
The `timeout` command in `beforeDevCommand` was failing with:
```
ERROR: Input redirection is not supported, exiting the process immediately.
```

## Solution
Created `start-all-services.ps1` PowerShell script for reliable Windows service orchestration.

---

## How It Works

The script:
1. **Starts Main IDE** (port 5173) → Waits 3 seconds
2. **Starts Qwen3-TTS** (port 8080) → Waits 2 seconds
3. **Starts AIRI 3D** (port 5175) → Keeps running

All services run in background windows.

---

## Usage

### Automatic (via Tauri)
```bash
npm run tauri dev
```

This now runs the PowerShell script automatically.

### Manual (Test Individual Services)

**Start all services**:
```powershell
.\start-all-services.ps1
```

**Test AIRI 3D alone**:
```bash
cd airi/apps/stage-web
pnpm dev
```

Then open: `http://localhost:5175/?headless=true&transparent=true&char=hiyori_pro`

**Test Qwen3-TTS alone**:
```bash
python qwen-tts-server.py
```

Then test: `curl http://localhost:8080/health`

---

## Files Modified

1. ✅ `start-all-services.ps1` - NEW: PowerShell orchestration script
2. ✅ `src-tauri/tauri.conf.json` - Updated to use PowerShell script

---

## Test Now

```bash
npm run tauri dev
```

**Expected Output**:
```
╔══════════════════════════════════════════════════════════╗
║      Starting AIRI Development Environment               ║
╚══════════════════════════════════════════════════════════╝

[1/3] Starting Main IDE on port 5173...
[2/3] Starting Qwen3-TTS Server on port 8080...
[3/3] Starting AIRI 3D App on port 5175...

✅ All services starting...

Services:
  🌐 Main IDE:      http://localhost:5173
  🎭 AIRI 3D:       http://localhost:5175
  🎤 Qwen3-TTS:     http://localhost:8080
```

**Then check right sidebar** - should show full 3D VRM avatar! 🎉

---

## Why PowerShell is Better

| Approach | Problem | PowerShell Solution |
|----------|---------|---------------------|
| `timeout /t 3` | Fails with input redirection | `Start-Sleep -Seconds 3` ✅ |
| `start /B` | Returns immediately, no wait | `Start-Process` + `Start-Sleep` ✅ |
| `cd dir && cmd` | Directory doesn't persist | `Set-Location` before `Start-Process` ✅ |
| Inline chain | Hard to debug | Colored output, clear steps ✅ |

---

**Status**: ✅ Fixed - PowerShell script handles multi-service start properly!
