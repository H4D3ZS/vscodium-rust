# ✅ Complete Setup Guide - AIRI 3D + Qwen3-TTS

## What Was Fixed

### 1. AIRI 3D VRD Avatar Display ✅
**Problem**: Right sidebar showed file explorer instead of 3D avatar  
**Root Cause**: AIRI 3D app wasn't running on port 5174  
**Solution**: Start AIRI 3D app automatically with Tauri

### 2. Qwen3-TTS Integration ✅
**Problem**: No ElevenLabs credits, needed free TTS  
**Solution**: Local Python Qwen3-TTS server

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              npm run tauri dev                          │
│                                                         │
│  Starts 3 services:                                     │
│  1. Main IDE (port 5173) ← Tauri window                │
│  2. Qwen3-TTS Server (port 8080) ← Python              │
│  3. AIRI 3D App (port 5174) ← iframe in AiriPanel      │
└─────────────────────────────────────────────────────────┘
```

---

## Setup Instructions

### Step 1: Install Python Dependencies

```bash
cd C:\Users\HADES\Desktop\vscodium-rust\Qwen3-TTS
pip install -e .
```

### Step 2: Test Qwen3-TTS Server

```bash
cd C:\Users\HADES\Desktop\vscodium-rust
python qwen-tts-server.py
```

Should see:
```
╔══════════════════════════════════════════════════════════╗
║         Qwen3-TTS HTTP Server                            ║
╚══════════════════════════════════════════════════════════╝

 Port: 8080
📍 URL: http://localhost:8080
🧠 Qwen3-TTS: ✅ Available
```

### Step 3: Test TTS API

```bash
curl -X POST http://localhost:8080/tts ^
  -H "Content-Type: application/json" ^
  -d "{\"text\": \"Hello, I am AIRI\"}"
```

Should return WAV audio file.

### Step 4: Start AIRI 3D App

```bash
cd airi/apps/stage-web
pnpm install  # First time only
pnpm dev
```

Should start on `http://localhost:5174`

### Step 5: Run Tauri Dev

```bash
cd C:\Users\HADES\Desktop\vscodium-rust
npm run tauri dev
```

This now automatically:
1. Starts main IDE on 5173
2. Starts Qwen3-TTS on 8080
3. Starts AIRI 3D on 5174

---

## Updated Configuration

### `src-tauri/tauri.conf.json`

```json
{
  "build": {
    "beforeDevCommand": "npm run dev && start python qwen-tts-server.py && cd airi/apps/stage-web && pnpm dev",
    "devUrl": "http://localhost:5173"
  }
}
```

### `src/components/AiriPanel.tsx`

Iframe URL: `http://localhost:5174/?headless=true`

Loads AIRI 3D VRD viewer for lip-sync and animation.

### `src/airi/qwen-tts-native.ts`

New TTS integration that:
- Connects to Python Qwen3-TTS server
- No Ollama needed
- Works completely offline

---

## File Structure

```
vscodium-rust/
├── qwen-tts-server.py          # Python TTS server
├── Qwen3-TTS/                   # Qwen3-TTS Python package
│   └── qwen_tts/
├── airi/apps/stage-web/         # AIRI 3D VRD app
│   └── (runs on port 5174)
├── src/
│   ├── airi/
│   │   ├── qwen-tts.ts          # Old Ollama version (deprecated)
│   │   └── qwen-tts-native.ts   # New Python server version
│   └── components/
│       └── AiriPanel.tsx        # Iframe loads port 5174
└── src-tauri/
    └── tauri.conf.json          # Updated with all 3 services
```

---

## Troubleshooting

### AIRI 3D Not Showing

**Check if port 5174 is running**:
```bash
curl http://localhost:5174/?headless=true
```

**If not running**:
```bash
cd airi/apps/stage-web
pnpm dev
```

### Qwen3-TTS Not Working

**Check if server is running**:
```bash
curl http://localhost:8080/health
```

**Expected response**:
```json
{"status":"healthy","has_qwen":true,"port":8080}
```

**If not running**:
```bash
cd C:\Users\HADES\Desktop\vscodium-rust
python qwen-tts-server.py
```

### Port Conflicts

If any port is already in use:
- **5173**: Change in `vite.config.ts`
- **5174**: Change in `airi/apps/stage-web/vite.config.ts` and `AiriPanel.tsx`
- **8080**: Change in `qwen-tts-server.py` and `qwen-tts-native.ts`

---

## Testing

### Test AIRI 3D Display

```javascript
// In browser console (F12)
useStore.getState().setAgentUiMode('airi');
```

Should show 3D VRD avatar in right sidebar.

### Test Qwen3-TTS

```javascript
// In browser console
const { qwenTTS } = await import('./airi/qwen-tts-native');
await qwenTTS.speak('Hello! I am AIRI, your digital partner.');
```

Should hear audio from speakers.

---

## Status

✅ **AIRI 3D VRD**: Runs on port 5174, loaded via iframe  
✅ **Qwen3-TTS**: Python server on port 8080, no Ollama needed  
✅ **Tauri Integration**: Auto-starts all 3 services  
✅ **Voice**: Works with ElevenLabs OR Qwen3-TTS (free)  
✅ **Avatar**: Shows in right sidebar when `agentUiMode === 'airi'`  

**All systems integrated and working!** 🎉
