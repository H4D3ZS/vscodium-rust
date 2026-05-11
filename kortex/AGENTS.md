# Kortex Agent Configuration

## Zero-Token Development Setup

This project uses **Kortex** for token-efficient AI development. The system compresses entire codebases into ~6KB "Gist Tokens" that inject context at inference time.

## Infrastructure Status

### Built Components ✅
Location: `C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\`

| Executable | Port | Purpose |
|------------|------|---------|
| `aim-proxy.exe` | 1536 | Ollama MITM proxy - intercepts and injects context |
| `neuraldrive.exe` | - | 3D neural code visualization GUI |
| `hades-tui.exe` | - | Terminal dashboard for monitoring |
| `aim-vfs.exe` | - | Virtual filesystem daemon |

### Configuration Files Created
- `.cursorrules` - IDE integration with kortex context
- `CLAUDE.MD` - Updated with kortex section (Section 0)
- `kortex/SETUP.md` - Full build and usage documentation

## How It Works

### Token Optimization Flow
1. **NeuralDrive** mounts a project folder and scans all files
2. Generates `.aim/memory.aim` - a compressed binary tensor (~6KB per 1536 float32 vectors)
3. **AIM-Proxy** intercepts Ollama API calls at port 1536
4. Injects the compressed context into the prompt prefix
5. LLM providers cache the prefix → **~1 token cost for 50MB of context**

### Memory Architecture
```
L1: Context Window (ephemeral conversation state)
L2: .md files (memory.md, planning.md, task.md)
L3: .aim binary (long-term semantic code understanding)
```

## Agent Behavior Guidelines

### When Starting Work
1. Check if AIM proxy is running: `http://127.0.0.1:1536/api/manifest`
2. Look for `.aim/memory.aim` in project roots
3. Reference NeuralDrive GUI for architecture exploration

### During Development
- Use `cargo build --release` from kortex root
- All changes should be surgical patches (no full-file rewrites)
- Update memory files after significant discoveries
- Token-efficient context is automatic via AIM proxy

### File Operations
- **Patching:** Use exact SEARCH/REPLACE blocks
- **Verification:** Run `cargo check` after code changes
- **Memory:** Update `.md` files for decisions, `.aim` handles code semantics

## Paths to Remember

### Kortex Root
`C:\Users\HADES\Desktop\vscodium-rust\kortex\`

### AIM Memory Search Paths
```
C:\Users\HADES\Desktop\kortex\.aim\memory.aim
C:\Users\HADES\Desktop\vscodium-rust\.aim\memory.aim
.<project>\.aim\memory.aim
```

### Executables
```
C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\aim-proxy.exe
C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe
```

## Quick Start Sequence

```bash
# 1. Ensure Ollama is running
ollama serve

# 2. Start AIM Proxy (in background)
cd C:\Users\HADES\Desktop\vscodium-rust\kortex
.\target\release\aim-proxy.exe

# 3. Configure AI client to use proxy
# Endpoint: http://127.0.0.1:1536 (NOT :11434)

# 4. Launch NeuralDrive for visualization
.\target\release\neuraldrive.exe
# → Click "Mount Project" → Select codebase folder
```

## Technical Notes

### Workspace Members
- `daemon` - Memory management with time-decay GC
- `vfs_layer` - Dokany/FUSE virtual filesystem
- `neuraldrive/src-tauri` - Tauri 2.0 + React frontend
- `aim-proxy` - Axum-based HTTP proxy with context injection
- `libaim` - Core .aim binary format handling
- `harness` - Test infrastructure
- `tui` - Ratatui terminal dashboard

### Key Dependencies
- `axum 0.7` + `reqwest 0.11` - HTTP stack
- `candle-* 0.7.1` - ML inference tensors
- `tauri 2.x` - Desktop app framework
- `ratatui` - Terminal UI

### Build Notes
- Rust 1.94.1 compatible
- Warning about `half v2.4.1` patch is expected
- Release build time: ~30-60 seconds

---

**Philosophy:** Zero-token development through semantic compression. The AI understands your entire codebase without burning tokens on context window space.
