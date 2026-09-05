# VSCodium-Rust

A local-first, agentic IDE built with **Rust/Tauri v2** + **React 19/TypeScript/Vite**.

![VSCodium-Rust](pics/1.png)

> **Human-authored, AI-assisted.** Architecture, technical approach, and every
> merge decision are human. AI is used to generate code under that direction, and
> every generated line is reviewed, tested against real hardware, and approved by a
> human before it lands — feature by feature. See [How this was built](#how-this-was-built).

---

## What this is

An IDE for people who run their models locally and drive them with agents. The
editor is VS Code–shaped; the backend is a native Rust/Tauri process, so the
agent loop, indexing, and process management don't run on an Electron main thread.

- **Local inference, Lemonade-first.** Primary provider is
  [Lemonade](https://github.com/lemonade-sdk/lemonade) — an OpenAI-compatible
  server (`:13305`) tuned for local hardware (AMD included); Ollama (`:11434`) is
  a fallback. Requests can route through the in-process **kortex** retrieval proxy
  (`:1536`), which augments prompts with `.aim` workspace context.
- **Agentic by default.** Multi-turn tool loop with verify-before-done, a shadow
  workspace for safe edits, and background agents for long-running work.
- **Local-first.** Code and model traffic stay on your machine unless you point a
  provider at a remote endpoint.
- **Security & ML tooling** in-IDE (offensive tooling, browser automation, PyTorch
  training) for research workflows.

It is an active project, not a finished product.

---

## Features

| Area | Highlights |
|------|-----------|
| **AI agent** | Autonomous tool loop with verify-before-done · thinking protocol · FastContext repo-exploration subagent · shadow workspace · background agents · tool-permission gate · MCP client + server · `.cursor/rules` |
| **Editor** | Monaco · FIM tab-autocomplete · inline agent-edit diff with per-hunk accept/reject · selection-scoped quick edit · Git status/diff/commit/branches |
| **Code intelligence** | Vector semantic search (Ollama embeddings) · symbol + chunk index · LSP diagnostics · distilled knowledge briefs |
| **Security research** | APEX 7-model specialist routing · headless-Firefox browser automation · pentest tooling · entropy-based secret scanning |
| **ML studio** | PyTorch train loop + loss curves · model hub (torchvision/timm/HF) · Optuna HPO · ONNX export |
| **iOS on Windows/Linux** | ARM64 Mach-O compile · Dart→Mach-O AOT · `.ipa` packaging · `zsign`/`ldid` signing · `go-ios` install + launch — no macOS/Xcode. See [`docs/mobile.md`](docs/mobile.md). |

`iPhoneOS.sdk` is the one asset that can't be redistributed — supply your own via
`SDKROOT` or the in-app **Import SDK**.

---

## Getting started

**Prerequisites:** Node.js 18+ · Rust toolchain (rustup) · a local model backend (Lemonade or Ollama)

```bash
git clone https://github.com/H4D3ZS/vscodium-rust.git
cd vscodium-rust
git submodule update --init --recursive   # kortex, turbovec
npm install
```

```bash
npm run dev          # frontend dev server
npm run dev:tauri    # full IDE (Tauri app)
npx tauri build      # production build
cd src-tauri && cargo check
```

macOS build notes: [`docs/build-macos.md`](docs/build-macos.md).
Release/installer: [`docs/release.md`](docs/release.md).

### AI agent setup

Lemonade runs llama.cpp directly:

```bash
# Best measured all-round agent model (30 tok/s, 8/8 tool calls)
lemonade pull Huihui-gemma-4-12B-agentic-abliterated-i1-Q4_K_M
# Stronger reasoning for long autonomous runs (16 tok/s, ~11k usable context)
lemonade pull Qwen3.6-35B-A3B-Abliterated-Heretic-GGUF-Q4_K_M
lemonade serve       # :13305
```

Avoid 2-bit quants — measured 0/6 on tool calling and 2.5–4× slower (compute-bound, not bandwidth-bound).
Open the IDE, pick the model in the agent toolbar, start coding.

---

## Architecture

Clean/DDD layering on both sides — `domain → application → infrastructure`, plus
`presentation` on the frontend — enforced by `scripts/check-architecture.mjs`.

```
src/                 React 19 + TypeScript
  domain/            entities, value objects, repository ports (no React, no Tauri)
  application/       use-cases (one file per user goal)
  infrastructure/    Tauri IPC adapters, lazy legacy engines
  components/        React UI — calls application layer, never invoke() directly
  store/             Zustand slices
src-tauri/src/       Rust (Tauri v2)
  domain/            ai · tools · vcs · security · indexing · memory · editor · mobile · extensions
  application/       thin #[tauri::command] adapters (the one invoke_handler! in lib.rs)
  infrastructure/    process spawning, HTTP, FFI, git2, tree-sitter
kortex/  (submodule) retrieval proxy · .aim binary format · daemon
turbovec/ (submodule) vector kernels
```

Full map and the conventions every change follows: [`ARCHITECTURE.md`](ARCHITECTURE.md).

---

## Testing

```bash
npm test                       # frontend unit + architecture check
npm run typecheck
cd src-tauri && cargo test      # backend
```

---

## How this was built

This project is AI-assisted, and it's worth being precise about which parts,
because the distinction matters.

**Human — everything that decides what the software is:**

- System architecture and module boundaries
- Every technical approach and the judgement behind it
- Design review and approval of each change before it lands
- Correctness review, cleanup, and stabilisation of generated code
- Debugging against real hardware, where the actual problems live

**AI — code generation under direction:**

- Boilerplate and scaffolding from a specified design
- Mechanical refactors across many files
- First-draft implementations of already-decided approaches
- Docstrings and comments, edited by hand afterwards

The load-bearing insights here were human. The host-native iOS toolchain — that
Apple's compilers were never the macOS-only part, that only packaging is — was a
human call, made against an assistant that had twice asserted it was impossible
and once insisted it needed cloud builds. It took human pushback and a direct
experiment to settle.

Every generated line was read, corrected, tested against real hardware, and
approved by a human before merge — which is why the suite is green and the
failure modes are documented rather than discovered by users. If you use AI on
your own projects, this is the split worth copying: let it write what you've
already decided, and never let it decide.

---

## Contributing

1. Fork, branch, change.
2. `npm test && npm run typecheck` and `cd src-tauri && cargo test`.
3. Keep patches surgical (`patch_engine` / `diffy`) — no full-file rewrites.
4. Follow [`ARCHITECTURE.md`](ARCHITECTURE.md): logic in engines, thin commands, no `invoke()` in components.
5. Open a PR — every feature is reviewed per code and per behaviour before merge.

---

## License

MIT — see [LICENSE](LICENSE). Bundled/adjacent components keep their own licenses
(`claurst` GPL, kept at a process boundary).

## Acknowledgments

[VSCodium](https://vscodium.com/) · [Tauri](https://tauri.app/) ·
[Lemonade](https://lemonade-server.ai/) · [go-ios](https://github.com/danielpaulus/go-ios) ·
[ldid](https://github.com/ProcursusTeam/ldid) · [Monaco Editor](https://microsoft.github.io/monaco-editor/)
