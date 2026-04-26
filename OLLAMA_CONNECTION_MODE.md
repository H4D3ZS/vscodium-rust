# Ollama Connection Mode Toggle

## Overview

KORTEX now supports switching between two Ollama connection modes:

| Mode | Port | Use Case | Token Savings |
|------|------|----------|---------------|
| 🧠 **AIM Proxy** | 1536 | When you want 99.9% token cost reduction | ✅ Yes |
| 🏠 **Direct Ollama** | 11434 | Simple local inference (RX 580) | ❌ No |

## How to Switch

1. Open **Settings** (gear icon in activity bar)
2. Scroll to **Ollama Integration** section
3. Click one of the two toggle buttons:

   - **🧠 AIM Proxy (1536)** - Use when:
     - You want maximum token efficiency
     - You're working with large codebases
     - You have the AIM proxy running (`aim-proxy.exe`)
   
   - **🏠 Direct Ollama (11434)** - Use when:
     - You just want simple local inference
     - You're testing on your RX 580
     - You don't want to run the extra proxy process

## Mode Details

### AIM Proxy Mode (Port 1536)

**What it does:**
- Sits between your app and Ollama
- Compresses context using `.aim` neural gist tokens
- Caches prompt prefixes for 99.9% token savings

**Requirements:**
```powershell
# Terminal 1: Start Ollama
ollama serve

# Terminal 2: Start AIM Proxy
cd "C:\Users\HADES\Desktop\vscodium-rust\kortex"
.\target\release\aim-proxy.exe
```

**Best for:**
- Large projects (5000+ files)
- Complex architectural reasoning
- Long coding sessions

### Direct Ollama Mode (Port 11434)

**What it does:**
- Connects directly to your local Ollama instance
- No compression or caching
- Standard LLM inference

**Requirements:**
```powershell
# Just start Ollama
ollama serve
```

**Best for:**
- Quick tests
- Small projects
- Simple chat/conversation
- RX 580 local inference

## Troubleshooting

### AI Not Responding

**Check which mode is selected:**
- If **AIM Proxy** selected → Make sure `aim-proxy.exe` is running
- If **Direct Ollama** selected → Make sure `ollama serve` is running

**Test connection:**
```powershell
# Test direct Ollama
curl http://localhost:11434/api/tags

# Test AIM proxy
curl http://localhost:1536/api/tags
```

### Switching Not Working

1. Try clicking **Reconnect** button
2. Restart the IDE
3. Check localStorage:
   ```javascript
   // In browser console (F12)
   localStorage.getItem('ollamaConnectionMode')
   // Should return 'proxy' or 'direct'
   ```

## Technical Details

### How It Works

The toggle updates two things:
1. **Store state** - `ollamaConnectionMode` ('proxy' | 'direct')
2. **Ollama URL** - Automatically switches between:
   - `http://localhost:1536` (proxy)
   - `http://localhost:11434` (direct)

### Persistence

Your selection is saved to `localStorage` and persists across sessions.

```typescript
// Store implementation
setOllamaConnectionMode: (mode: 'proxy' | 'direct') => {
    const url = mode === 'proxy' ? 'http://localhost:1536' : 'http://localhost:11434';
    set({ ollamaConnectionMode: mode, ollamaUrl: url });
    invoke('set_ollama_url', { url });
    localStorage.setItem('ollamaConnectionMode', mode);
}
```

## Files Modified

- `src/store.ts` - Added `ollamaConnectionMode` state and setter
- `src/components/AgentSettingsView.tsx` - Added toggle UI
- All `src/airi/*.ts` modules - Use dynamic URL from store

---

**Quick Tip**: Start with **Direct Ollama** mode for testing, then switch to **AIM Proxy** when you need the token efficiency for large projects!
