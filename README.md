# VSCodium-Rust

A local-first, agentic IDE — **Rust/Tauri v2** backend, **React 19/TypeScript** frontend.
The editor is VS Code–shaped; the agent loop, indexing, and process management run in a
native process, not an Electron main thread.

![VSCodium-Rust](pics/1.png)

> **Human-authored, AI-assisted.** Architecture, technical approach, and every merge
> decision are human. AI generates code under that direction; every generated line is
> reviewed and tested against real hardware before it lands, feature by feature.

## What you get

- **Local inference via [Lemonade](https://github.com/lemonade-sdk/lemonade)** — an
  OpenAI-compatible server (`:13305`) running real llama.cpp, tuned for local hardware
  (AMD included). It's the only backend; bring your own API key for a hosted model if
  you want one. Prompts can route through the in-process **kortex** proxy (`:1536`),
  which injects `.aim` workspace context.
- **Agentic by default** — multi-turn tool loop with verify-before-done, a shadow
  workspace for safe edits, background agents for long work.
- **Local-first** — code and model traffic stay on your machine unless you point a
  provider at a remote endpoint.
- **Editor** — Monaco, FIM tab-autocomplete, inline agent-edit diff with per-hunk
  accept/reject, Git status/diff/commit.
- **Code intelligence** — vector semantic search, symbol + chunk index, LSP
  diagnostics, distilled knowledge briefs.
- **Security & ML** — APEX specialist routing, headless-browser automation, secret
  scanning, PyTorch train loop + ONNX export.
- **iOS on Windows/Linux** — ARM64 Mach-O compile, `.ipa` packaging, `zsign`/`ldid`
  signing, `go-ios` install — no macOS/Xcode. See [`docs/mobile.md`](docs/mobile.md)
  (`iPhoneOS.sdk` is the one asset you supply yourself).

## Quick start

**Prerequisites:** Node.js 18+ · Rust (rustup) · [Lemonade](https://github.com/lemonade-sdk/lemonade)

```bash
git clone https://github.com/H4D3ZS/vscodium-rust.git
cd vscodium-rust
git submodule update --init --recursive    # kortex, turbovec
npm install

lemonade pull Huihui-gemma-4-12B-agentic-abliterated-i1-Q4_K_M   # best all-round agent model
lemonade serve                                                    # :13305

npm run dev:tauri     # full IDE   (npm run dev = frontend only, npx tauri build = release)
```

Avoid 2-bit quants — measured 0/6 on tool calling and 2.5–4× slower.
macOS: [`docs/build-macos.md`](docs/build-macos.md) · Release: [`docs/release.md`](docs/release.md).

## Architecture

Clean/DDD layering on both sides (`domain → application → infrastructure`, plus
`presentation` on the frontend), enforced by `scripts/check-architecture.mjs`.
Full map and conventions: [`ARCHITECTURE.md`](ARCHITECTURE.md) · frontend entry
points: [`src/README.md`](src/README.md).

## Testing

```bash
npm test && npm run typecheck         # frontend + architecture check
cd src-tauri && cargo test            # backend
```

## Contributing

Fork, branch, change. Run the tests above. Keep patches surgical (no full-file
rewrites), logic in engines, thin commands, no `invoke()` in components — see
[`ARCHITECTURE.md`](ARCHITECTURE.md). Every feature is reviewed per code and per
behaviour before merge.

## License

MIT — see [LICENSE](LICENSE). `claurst` is GPL, kept at a process boundary.
Built on [VSCodium](https://vscodium.com/), [Tauri](https://tauri.app/),
[Lemonade](https://github.com/lemonade-sdk/lemonade),
[go-ios](https://github.com/danielpaulus/go-ios),
[ldid](https://github.com/ProcursusTeam/ldid),
[Monaco](https://microsoft.github.io/monaco-editor/).
