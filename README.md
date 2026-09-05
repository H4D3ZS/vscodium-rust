# VSCodium-Rust

A local-first agentic IDE built with Rust/Tauri v2 + React 19/TypeScript/Vite.

![VSCodium-Rust](pics/1.png)

---

## What This Is

An IDE for people who run their models locally and drive them with agents. The
editor is VS Code-shaped; the backend is Rust/Tauri, so the agent loop, indexing,
and process management run in a native process rather than an Electron main thread.

What it's built around:

- **Local inference, Lemonade-first.** The primary provider is
  [Lemonade](https://github.com/lemonade-sdk/lemonade) — an OpenAI-compatible
  server (`:13305`) tuned for local hardware (AMD included). Ollama (`:11434`) is
  supported as a fallback. Requests can be routed through the in-process
  **kortex** retrieval proxy (`:1536`), which augments prompts with `.aim`
  workspace context before they reach the model.
- **Agentic by default.** A multi-turn tool loop with verify-before-done, a shadow
  workspace for safe edits, and background agents for long-running work.
- **Local-first.** Your code and model traffic stay on your machine unless you
  point a provider at a remote endpoint.
- **Security & ML tooling** integrated in-IDE (offensive tooling, browser
  automation, PyTorch training) for research workflows.

It is an active overhaul, not a finished product — see `docs/overhaul/` for the
current state and conventions.

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

### iOS development on Windows / Linux

Build, sign and install iOS apps without macOS, Xcode, or a cloud runner.

| Stage | Tool | Status |
|-------|------|--------|
| Compile | `clang` / `rustc` → ARM64 Mach-O | working |
| Dart → Mach-O | `gen_snapshot --snapshot_kind=app-aot-macho-dylib` | verified |
| Package | `Payload/App.app` + `Info.plist` → `.ipa` | working |
| Sign | `zsign` (real cert) or `ldid` (ad-hoc) | working |
| Install + launch | `go-ios` | working |
| Device detection | `flutter custom-devices` + `go-ios` | `flutter devices` lists the phone |

Apple's *compilers* were never the macOS-only part — LLVM's Mach-O backend runs
anywhere. Only the packaging utilities (`xcodebuild`, `codesign`, `actool`) are
macOS-bound, and each has a portable replacement.

`iPhoneOS.sdk` is the one asset that cannot be redistributed; supply your own
via `SDKROOT` or the in-app **Import SDK**. Everything else ships with the IDE.

Not yet complete: `Flutter.xcframework` fetching and the `postBuild` artefact
swap, so `flutter run` still emits the wrong binary format. React Native's
native shell still needs `xcodebuild`; Expo Go works today with no build step.

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

### Install a model

Lemonade is the local backend — it runs llama.cpp directly.

```bash
# Measured best all-round agent model (30 tok/s, 8/8 tool calls)
lemonade pull Huihui-gemma-4-12B-agentic-abliterated-i1-Q4_K_M

# Stronger reasoning for long autonomous runs (16 tok/s, ~11k usable context)
lemonade pull Qwen3.6-35B-A3B-Abliterated-Heretic-GGUF-Q4_K_M
```

Avoid 2-bit quants: measured 0/6 on tool calling and 2.5–4x slower, because
they are compute-bound rather than bandwidth-bound.

### Start the server

```bash
lemonade serve      # :13305
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

## How This Was Built

This project is AI-assisted, and it is worth being precise about which parts
that means, because the distinction matters.

**Human — all of it that decides what the software is:**

- System architecture and module boundaries
- Every technical approach, and the judgement calls behind them
- Design review and approval of each change before it lands
- Correctness review, cleanup, and stabilisation of generated code
- Debugging against real hardware, where the actual problems live

**AI — code generation under direction:**

- Boilerplate and scaffolding from a specified design
- Mechanical refactors across many files
- First-draft implementations of already-decided approaches
- Docstrings and comments, edited by hand afterwards

Worth stating plainly: the load-bearing insights here were human. The
host-native iOS toolchain — that Apple's compilers are not the macOS-only
part, that only packaging is — was a human call, made against an AI assistant
that had twice asserted it was impossible and once insisted it needed cloud
builds. It took human pushback and a direct experiment to settle. The same
holds for the QEMU-based device work.

AI is a fast typist that has read a lot. It is not the engineer. Every
generated line here was read, corrected, tested against real hardware, and
approved by a human before merge — which is why the test suite is green and
the failure modes are documented rather than discovered by users.

If you use AI on your own projects, this is the split worth copying: let it
write what you have already decided, and never let it decide.

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
- [Lemonade](https://lemonade-server.ai/) — Local LLM runtime (llama.cpp)
- [go-ios](https://github.com/danielpaulus/go-ios) — iOS device control without Xcode
- [ldid](https://github.com/ProcursusTeam/ldid) — Mach-O code signing off macOS
- [Monaco Editor](https://microsoft.github.io/monaco-editor/) — Code editor component
# Test
