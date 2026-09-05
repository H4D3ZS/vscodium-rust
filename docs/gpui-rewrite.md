# Native gpui Rewrite — Architecture & Migration

> Decision: replace the Tauri **WebView (Chromium) frontend** with a **native gpui
> UI in Rust**, in one process, calling the existing backend **directly as a Rust
> library** — no Bun, no FFI, no Chromium. This removes the renderer memory ceiling
> that caused the recurring `Out of Memory`, and targets a ~50–150 MB footprint
> that runs on a 10-year-old laptop.

## Why native gpui (not gpui-react/FFI)
- gpui-react routes React→gpui over **Bun FFI** — extra runtime, IPC, and an early
  reconciler. For a product, a **single Rust binary** (gpui UI + domain logic) is
  simpler, faster, and lighter.
- Our backend is **already Rust** (`vscode_rust_app_lib`, an `rlib`) with the AI
  engine, tools, indexing, vega, and **tree-sitter** (rust/ts/js/python) built in.
  The gpui UI calls it **in-process** — zero serialization, no WebView IPC.

## The one real refactor: decouple the core from Tauri
The domain logic currently leans on `tauri::State<EditorState>`, `AppHandle`, and
`Emitter` (`app.emit(...)`). To call it from gpui (no Tauri runtime) we extract a
**`core` layer** that is Tauri-free:
- Replace `app.emit("ai-content", …)` with a generic **event sink trait**
  (`trait UiEvents { fn emit(&self, ev: CoreEvent); }`). Tauri impl (old app) and
  a gpui impl (channel into the gpui app) both satisfy it.
- Replace `State<EditorState>` with a plain `Arc<EditorState>` passed in.
- Keep all logic (ai engine, patch engine, tools, indexer, vega, tree-sitter)
  unchanged — only the **binding glue** is swapped.
- Net: `core` becomes usable by BOTH the existing Tauri app (during transition)
  and the new gpui app, so we migrate incrementally without a hard cutover.

## App structure (new crate `gpui-ide/`)
```
gpui-ide/
  Cargo.toml         # gpui = "0.2.2", path-dep on the decoupled core
  src/
    main.rs          # Application::new().run → open_window → Workbench view
    workbench.rs     # layout: title / activitybar / sidebar / editor / panel / agent
    views/
      file_tree.rs   # explorer (core::workspace)
      editor.rs      # code editor (the hard part — see below)
      agent_chat.rs  # the agent panel (core::ai, streamed via the event sink)
      terminal.rs    # PTY view (core already has portable-pty)
    state.rs         # gpui Entities holding UI state; subscribes to CoreEvent
```
gpui API (verified from the clone, gpui 0.2.2): `Application::new().run(|cx| …)`,
`cx.open_window(opts, |window, cx| cx.new(|_| View))`, `impl Render { fn render(&mut
self, &mut Window, &mut Context<Self>) -> impl IntoElement }`, Tailwind-like
`div().flex().bg(rgb()).child()`.

## The make-or-break: the code editor
gpui has text layout + input, but **no drop-in code editor** (Zed's is internal).
Options, in order of pragmatism:
1. **Build a gpui text view** using gpui's text/scroll + our **existing tree-sitter**
   for syntax highlighting (we already parse rust/ts/js/python). Start **read-only
   viewer** (huge files via line-range — the paged model we already designed), then
   add cursor/selection/edit. This is the bulk of the work but uses assets we have.
2. **Hybrid bridge** (temporary): keep a tiny webview *only* for the editor while
   the rest is gpui — de-risks the timeline; drop it once the native editor lands.

## What we keep vs rebuild
| Keep (Rust, reused as-is) | Rebuild in gpui |
|---|---|
| AI engine, autonomous loop, tools, MCP | All React/TS views |
| Indexing, vector store, tree-sitter | Monaco → native editor |
| vega, apex, security | mermaid/reactflow → gpui canvas or webview island |
| git, terminal (portable-pty), memory/.aim | HeroUI/Tailwind → gpui styling |
| Zustand-equivalent state moves into gpui Entities | |

## Migration order (each milestone is runnable/verifiable)
- **Phase 1 (done):** `gpui-ide` crate opens a native window with a dark
  Workbench shell (title/sidebar/editor placeholder). Proves gpui runs on
  Windows. `cargo run --manifest-path gpui-ide/Cargo.toml`.
- **Phase 2 (done):** native **file tree** + file viewer over real `std::fs`
  (dirs-first, click expand/open, 256 KB cap). No Monaco, no WebView.
- **Phase 3 (done):** native **agentic tool loop** (`gpui-ide/src/agent.rs`) —
  a blocking ReAct loop against Ollama `/api/chat` with native function-calling,
  executing tools directly in Rust: `list_dir`, `read_file`, `write_file`,
  `run_command`, `search`. Fully autonomous (Cursor-YOLO: writes/commands run
  without confirmation). Runs on a background std thread; the UI mirrors an
  append-only run log via a 50 ms gpui poll loop. **This is the surface that
  OOM'd the WebView — here it edits files, runs builds, and searches with no
  Chromium heap and no Tauri runtime.** Release binary ~13 MB (incl. TLS/HTTP)
  vs the WebView app's 500 MB–1.6 GB heap.
  Deliberately does **not** link `vscode_rust_app_lib` (Tauri-coupled) — it
  talks HTTP so the shell stays a pure-native, zero-WebView process.
  Endpoint via env: `GPUI_MODEL_BASE` (default `http://localhost:11434`),
  `GPUI_MODEL` (default `qwen2.5-coder:7b` — use a tool-calling model:
  Qwen2.5/GLM/Llama3.1). Composer input is minimal (letters/digits/space/
  backspace/enter); full IME/text editing is Phase 4.
  Next: richer tools (apply-patch/diff, multi-file edit), inline diff view, and
  optionally a shadow-VFS guard before writes.
- **Phase 4:** **native editor** (read-only viewer + tree-sitter highlight → full
  edit). The biggest phase; hybrid webview-island fallback if needed.
- **Phase 5:** terminal, diagrams (canvas), settings; retire the WebView app.

## Risks / unknowns
- gpui Windows maturity (build deps, perf) — Phase 1 validates this first.
- Editor scope (Phase 4) — largest effort; have the hybrid fallback ready.
- Decoupling touches many `app.emit` call sites — mechanical but broad.

## Build notes
- gpui may require specific toolchain/system deps on Windows; first `cargo run`
  surfaces them. Keep `gpui-ide` a standalone crate initially (its own Cargo.toml,
  not in a workspace) so it builds independently of `src-tauri`.
