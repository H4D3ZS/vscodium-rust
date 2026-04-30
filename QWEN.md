# QWEN.md - Qwen Code Configuration for HADES-KORTEX

## Zero-Token Context Integration

**This project uses Kortex for token-efficient AI development.**

As Qwen Code assisting in this project, you should:

### 1. AIM Proxy Configuration
When using Ollama for local inference, configure it to use the AIM proxy:
- **Proxy Endpoint:** `http://127.0.0.1:1536`
- **Target:** `http://127.0.0.1:11434`
- **Benefit:** ~99.9% token reduction via .aim context injection

### 2. Memory File Locations
Check these paths for `.aim` memory files that contain compressed codebase context:
```
C:\Users\HADES\Desktop\kortex\.aim\memory.aim
C:\Users\HADES\Desktop\vscodium-rust\.aim\memory.aim
.<project>\.aim\memory.aim
```

### 3. Built Tools Available
Location: `C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\`
- `aim-proxy.exe` - Context injection proxy (run alongside Ollama)
- `neuraldrive.exe` - 3D code visualization GUI
- `hades-tui.exe` - Terminal dashboard
- `aim-vfs.exe` - VFS daemon

### 4. Development Workflow
```bash
# Start Ollama + AIM Proxy (in background)
ollama serve
cd kortex && .\target\release\aim-proxy.exe

# Build commands (already permitted in permissions)
cargo build --release
cargo check
```

### 5. Context Optimization
- The AIM proxy automatically injects compressed `.aim` context into prompts
- Prefix caching reduces 50MB codebase context to ~1 token
- NeuralDrive GUI generates `.aim` files by mounting project folders

## Project Identity

**Name:** HADES-KORTEX
**Role:** Cybersecurity Research & Systems Architecture
**Philosophy:** Daoist Wu Wei (effortless action), Socratic logic
**Constraints:** Windows-native, Rust/Tauri, <150MB footprint

## Key Directories
```
kortex/           - Main workspace (AIM, NeuralDrive, VFS)
airi/             - 3D/VRM avatar integration
claurst/          - Related projects
```

## Permissions Note
All necessary build/command permissions are configured in `.qwen/settings.json`:
- `cargo build/check/clean`
- `git` operations
- `powershell` scripts
- File operations (`set-content`, `replace`, etc.)

---

**For Qwen Code:** Use this configuration to provide token-efficient assistance. The kortex system handles context compression automatically when the AIM proxy is running.
