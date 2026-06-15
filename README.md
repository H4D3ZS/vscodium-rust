# VSCodium-Rust

A local-first agentic IDE built with Rust/Tauri v2 + React 19/TypeScript/Vite.

![VSCodium-Rust](pics/1.png)

---

## What This Is

VSCodium-Rust is a full-scale development environment designed for developers who want:

- **Local AI agents** — autonomous coding with Ollama, no cloud required
- **Data sovereignty** — your code never leaves your machine
- **Security research** — integrated offensive tooling and browser automation
- **ML experimentation** — in-IDE PyTorch training and model management

Built on the VS Code architecture, rewritten in Rust for performance.

---

## Features

### AI Agent

| Feature | Description |
|---------|-------------|
| Autonomous agent loop | Multi-turn tool execution with verify-before-done |
| Fable-5 thinking protocol | Model reasons before every action |
| FastContext explorer | Dedicated 4B repo exploration subagent |
| Shadow workspace | Safe code mutation before commit |
| Background agents | Long-running parallel tasks |
| Tool permissions | Approve/deny destructive operations |
| MCP integration | Client + server for external tools |
| Cursor rules | `.cursor/rules/*.mdc` project configuration |

### Editor

| Feature | Description |
|---------|-------------|
| Monaco editor | Full VS Code editor with extensions |
| Tab autocomplete | FIM-based code completion |
| Inline diff | Agent edit preview with accept/reject |
| Quick edit | Selection-scoped inline modification |
| Git integration | Status, diff, commit, branches |

### Code Intelligence

| Feature | Description |
|---------|-------------|
| Semantic search | Vector embeddings via Ollama |
| Codebase index | Symbol + chunk index with cosine similarity |
| LSP support | Language Server Protocol diagnostics |
| Knowledge briefs | Distilled project context |

### Security Research

| Feature | Description |
|---------|-------------|
| APEX orchestrator | 7-model specialist routing |
| Browser automation | Headless Firefox with DOM interaction |
| Pentest tools | SQLi, XSS, SSRF, reverse shells |
| Secret scanning | Entropy-based credential detection |

### ML Studio

| Feature | Description |
|---------|-------------|
| PyTorch training | Dataset, train, loss curves, export |
| Model hub | torchvision, timm, HuggingFace gallery |
| Optuna HPO | Hyperparameter optimization |
| ONNX export | Model conversion and deployment |

---

## Getting Started

### Prerequisites

- Node.js 18+
- Rust toolchain (rustup)
- Ollama (for local AI)

### Install

```bash
git clone https://github.com/your-org/vscodium-rust.git
cd vscodium-rust
npm install
```

### Run

```bash
# Frontend dev server
npm run dev

# Full IDE (Tauri app)
npm run dev:tauri

# Rust backend check
cd src-tauri && cargo check
```

### Build

```bash
# Production build
npx tauri build
```

---

## AI Agent Setup

### Pull a model

```bash
# Best local agent model (Gemma4 12B, Fable-5 trained)
ollama pull hf.co/yuxinlu1/gemma-4-12B-coder-fable5-composer2.5-v1-GGUF:Q4_K_M

# Fast coding model
ollama pull qwen2.5-coder:14b

# Exploration subagent
ollama pull hf.co/mitkox/FastContext-1.0-4B-SFT-Q4_K_M-GGUF:Q4_K_M
```

### Start Ollama

```bash
ollama serve
```

Open the IDE, select your model in the agent toolbar, and start coding.

---

## Architecture

```
vscodium-rust/
  src/                    # React 19 + TypeScript frontend
    components/           # UI components
    store/                # Zustand state management
    domain/               # Business logic
    application/          # Application services
    infrastructure/       # Tauri bridge adapters
  src-tauri/              # Rust backend (Tauri v2)
    src/domain/ai/        # AI engine + autonomous loop
    src/domain/tools/     # Tool registry + dispatch
    src/domain/vcs/       # Git, patches, shadow workspace
    src/domain/security/  # APEX orchestrator, pentest tools
  kortex/                 # Supporting Rust workspace
    aim-proxy/            # Ollama MITM proxy
    libaim/               # .aim binary format
```

### Key Design Decisions

- **Patch discipline**: Surgical SEARCH/REPLACE via `patch_engine.rs`, no full-file rewrites
- **Memory budget**: Core footprint under 150MB
- **Tauri IPC**: All frontend-backend calls through `#[tauri::command]` handlers
- **Tool registry**: JSON Schema-based tools compatible with OpenAI/Anthropic function calling

---

## Testing

```bash
# Frontend tests
npm test

# Type checking
npm run typecheck

# Rust tests
cd src-tauri && cargo test
```

---

## Project Structure

| Directory | Purpose |
|-----------|---------|
| `src/` | React frontend (Vite + TypeScript) |
| `src-tauri/` | Rust backend (Tauri v2) |
| `kortex/` | Supporting crates (proxy, VFS, daemon) |
| `airi/` | 3D VRM avatar (optional) |
| `claurst/` | External agent SDK |

---

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `npm test` and `cargo test`
5. Submit a pull request

### Code Standards

- No `console.log` in production code
- Use CSS classes over inline styles where possible
- Follow existing naming conventions
- Add tests for new features

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

Built on the work of:
- [VSCodium](https://vscodium.com/) — VS Code without Microsoft telemetry
- [Tauri](https://tauri.app/) — Rust backend for desktop apps
- [Ollama](https://ollama.com/) — Local LLM runtime
- [Monaco Editor](https://microsoft.github.io/monaco-editor/) — Code editor component
