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
  you want one. Prompts can route through **kortex** (from the
  [kortex submodule](https://github.com/H4D3ZS/kortex), AGPL-3.0):
  - **KV-slot cache** (`:1537`) — an in-process reverse proxy in front of the
    backend that skips re-prefilling repeated prompt prefixes (KDKVC). Opt-in:
    the *Kortex Services* panel's **Start** button, or `kvcache.autostart=1`.
  - **AIM retrieval proxy** (`:1536`) — an in-process router (`aim-proxy`).
    On Start it builds a dense `.aim` catalog of the workspace (via the
    Lemonade embedder), then embeds each request's last user turn, searches
    the catalog, and prepends only the chunks that clear the gate — so the
    model gets *less, more relevant* context instead of the whole repo.
    Falls back to a plain pass-through if the catalog is empty or a search
    overruns its latency budget. Opt-in (panel **Start**, or
    `kortex.retrieval.autostart=1`).
  - **VFS daemon** (`:1538`) — a sidecar process managing `.aim` memory and file
    watching. Auto-starts on boot.
  - **Speculative decoding** — for the bundled ROCmFPX backend, the *Kortex
    ROCmFPX* panel exposes a decode-speed picker (prompt-lookup n-grams — no
    model, no VRAM — and/or the model's MTP head). The full model verifies
    every drafted token, so output is unchanged; the panel shows the live
    acceptance rate. See [`docs/kortex-decode-throughput.md`](docs/kortex-decode-throughput.md).
    Set `KORTEX_COMPUTE_TRACE=<path>` and `tools/compute-bench/reduce_trace.py`
    turns a session into a measured prefill-savings receipt.
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
- **iOS build on Windows/Linux** — ARM64 Mach-O compile, `.ipa` packaging,
  `zsign`/`ldid` signing, `go-ios` install — no macOS/Xcode (`iPhoneOS.sdk` is the
  one asset you supply). The iPhone **emulator** is a separate in-progress Rust
  component; on Apple Silicon the Devices panel drives
  [vphone-cli](https://github.com/Lakr233/vphone-cli) /
  [darwin-vm](https://github.com/jprx/darwin-vm) today. See [`docs/mobile.md`](docs/mobile.md).

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

iPhone emulation on Apple Silicon builds on two MIT projects — credit to their
authors: [vphone-cli](https://github.com/Lakr233/vphone-cli) by
[Lakr233](https://github.com/Lakr233), and
[darwin-vm](https://github.com/jprx/darwin-vm) by [jprx](https://github.com/jprx).
