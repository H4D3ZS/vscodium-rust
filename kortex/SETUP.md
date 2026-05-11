# Kortex Setup Guide

## Overview
Kortex is a zero-token cost AI development framework featuring:
- **Daemon**: Cognitive housekeeper for memory management
- **VFS Layer**: Virtual File System adapter
- **NeuralDrive**: Tauri-based 3D neural code visualization GUI
- **AIM-Proxy**: Transparent MITM proxy for Ollama integration

## Prerequisites
- **Rust**: 1.80+ (currently installed: 1.94.1)
- **Node.js**: Latest LTS (for NeuralDrive frontend)
- **Windows**: Dokany driver (for VFS mounting, optional)

## Build Instructions

### Full Workspace Build
```bash
cd kortex
cargo build --release
```

### Built Executables (in `target/release/`)
- `aim-proxy.exe` - AI proxy interceptor (port 1536)
- `neuraldrive.exe` - 3D neural code viewer GUI
- `hades-tui.exe` - Terminal UI
- `aim-vfs.exe` - VFS daemon

### Individual Component Builds

#### AIM-Proxy
```bash
cd kortex/aim-proxy
cargo build --release
```

#### NeuralDrive (Full Stack)
```bash
# Build Rust backend
cd kortex/neuraldrive/src-tauri
cargo build --release

# Build React frontend
cd ../
npm install
npm run build
```

#### Daemon
```bash
cd kortex/daemon
cargo build --release
```

## Configuration

### AIM-Proxy Setup
The proxy runs on `http://127.0.0.1:1536` and forwards to Ollama at `http://127.0.0.1:11434`.

To use:
1. Start Ollama: `ollama serve`
2. Start AIM-Proxy: `.\target\release\aim-proxy.exe`
3. Configure your AI client to use `http://127.0.0.1:1536` instead of `:11434`

### NeuralDrive Usage
1. Launch `neuraldrive.exe`
2. Click "Mount Project" in the bottom-left sidebar
3. Select a codebase folder
4. Explore the 3D neural graph visualization

## Known Issues & Fixes

### Half Crate Patch Warning
The workspace patches `half v2.4.1` from git. This is expected and can be ignored:
```
warning: patch `half v2.4.1` was not used in the crate graph
```

### VFS Layer
The VFS layer requires Dokany (Windows) or FUSE (Linux/Mac) for mounting virtual filesystems. This is optional for basic functionality.

## Development Notes

### Workspace Members
- `daemon` - Memory management daemon
- `vfs_layer` - Virtual filesystem integration
- `neuraldrive/src-tauri` - Desktop GUI backend
- `aim-proxy` - Ollama proxy interceptor
- `libaim` - Core AIM library
- `harness` - Test harness
- `tui` - Terminal UI

### Key Dependencies
- `axum 0.7` - Web framework
- `reqwest 0.11` - HTTP client
- `tokio` - Async runtime
- `candle-*` - ML inference
- `tauri 2.x` - Desktop app framework

## Troubleshooting

### Build Failures
If you encounter dependency conflicts:
```bash
cargo clean
cargo update
cargo build --release
```

### NeuralDrive Frontend Issues
```bash
cd neuraldrive
rm -rf node_modules package-lock.json
npm install
npm run tauri build
```

## Performance Notes
- Memory footprint: <100MB idle
- Gist token size: ~6KB per 1,536 float32 vectors
- Zero-token optimization via prefix caching

---
*For more information, see README.md and whitepaper.md*
