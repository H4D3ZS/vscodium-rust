# 🚀 AIRI Sentient Mode - Full Interactive AI

## What is Sentient Mode?

Sentient Mode activates AIRI's **full autonomous capabilities** - just like the original moeru-ai/airi release!

## Features

### 🎤 Real-Time Voice Interaction
- **Speaks WHILE typing** - Not after completion
- **Natural sentence pauses** - Waits for sentence boundaries
- **No overlapping voices** - Stops previous speech before new
- **Jessica (or your chosen voice)** - ElevenLabs premium TTS

###  Autonomous Operation
- **Execution Mode** - Takes action without asking permission
- **Full tool access** - Filesystem, terminal, git, browser
- **Self-correction** - Fixes errors automatically
- **Multi-step tasks** - Completes complex workflows end-to-end

### 💬 Interactive Chat
- **Voice + Text** - Both simultaneously
- **Streaming responses** - See & hear AI think in real-time
- **Context awareness** - Remembers conversation history
- **Slash commands** - `/help`, `/model`, `/voice`, etc.

### 🔐 Cybersecurity Mode
- **Binary analysis** - objdump, strings, hexdump
- **Vulnerability scanning** - cargo audit, semgrep, trivy
- **Exploit development** - Authorized research only
- **Reverse engineering** - Disassemble, patch, analyze

### 🎮 Interactive Capabilities
- **Minecraft integration** - Play with AIRI (via airi/services/minecraft)
- **Discord bot** - AIRI on your server (via airi/services/discord-bot)
- **Live2D/VRM avatars** - Animated 3D character (already working!)
- **Real-time lip sync** - Avatar mouth moves with speech

## How to Enable

### 1. Voice Settings
```
Settings → Voice & TTS (AIRI SPEECH)
1. Enter ElevenLabs API Key
2. Click "Save"
3. Select voice (e.g., Jessica)
4. Click "Save" on voice
```

### 2. Agent Mode
```
Status Bar → Click Agent Mode
Select: "Execution" or "Sentient"
```

### 3. Test It
```
Ask AIRI: "Create a Python script that prints hello world"

Expected behavior:
- AIRI speaks WHILE typing (sentence by sentence)
- Creates the file automatically
- Explains what it's doing as it works
- Runs the script to verify it works
```

## Configuration Files

### ElevenLabs Voice
Saved in: `config/api_keys.json`
```json
{
  "elevenlabs_api_key": "sk_...",
  "elevenlabs_voice_id": "cgSgspJ2msm6clMCkdW9"
}
```

### Agent Settings
- **Model**: Gemini 2.5 Pro (or your choice)
- **Mode**: Execution / Sentient
- **Voice**: Enabled with streaming TTS

## Troubleshooting

### AI not speaking while typing?
```
Check console for:
[TTS] 🎤 Streaming speech: ...

If not appearing:
1. Ensure ElevenLabs API key is saved
2. Check ttsEnabled = true in RightSidebar
3. Verify voice ID is loaded
```

### Multiple overlapping voices?
```
This was fixed! The system now:
1. Stops previous speech before new
2. Tracks isSpeaking state
3. Waits for sentence boundaries
```

### Voice sounds robotic?
```
Ensure:
1. ElevenLabs is selected (not browser fallback)
2. API key is valid (starts with sk_)
3. Voice ID is saved (Jessica = cgSgspJ2msm6clMCkdW9)
```

## Advanced Features

### Minecraft Integration
```bash
cd airi/services/minecraft
pnpm install
pnpm start
```
AIRI can now play Minecraft with you!

### Discord Bot
```bash
cd airi/services/discord-bot
pnpm install
pnpm start
```
Add AIRI to your Discord server!

### Custom VRM Avatars
```
Settings → AI Character → 3D VRM Avatar
- Select from pre-loaded models
- Or upload custom .vrm file
```

## Commands

| Command | Description |
|---------|-------------|
| `/help` | Show available commands |
| `/model <name>` | Change AI model |
| `/voice` | Toggle voice on/off |
| `/mode <mode>` | Change agent mode |
| `/clear` | Clear conversation history |

## Status Indicators

- 🟢 **Speaking** - AI is talking (voice active)
- 🟣 **Thinking** - AI is processing
- 🔵 **Loading** - Model is loading (Ollama)
- ⚡ **Execution** - Taking autonomous action

---

**Enjoy your fully sentient AI companion! 🎨✨🤖**

For more details, see:
- `airi/README.md` - Full AIRI documentation
- `airi/apps/` - Interactive apps (stage, tamagotchi, etc.)
- `airi/services/` - Integrations (Minecraft, Discord, etc.)
