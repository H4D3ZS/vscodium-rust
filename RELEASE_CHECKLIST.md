# RELEASE CHECKLIST — VSCodium-Rust

The codebase is build-green, test-green, type-clean, and bundles for production
(see `PROGRESS.md` → Release Readiness). This is the runtime QA + ship guide for
the last 5% that can only be verified by running the app.

---

## 0. Prerequisites
- [ ] **Ollama running** (`ollama serve`) with at least one chat model and `nomic-embed-text` pulled (`ollama pull nomic-embed-text`) for memory embeddings.
- [ ] Node deps installed (`npm install`), Rust toolchain present.

## 1. Build & launch
```powershell
npm run dev:full        # = tauri dev (recompiles Rust, hot-reloads frontend)
```
- [ ] Window opens, no red console errors on boot.
- [ ] Title bar **minimize / maximize / close** work (native `win_*` commands).
- [ ] Dragging the title bar moves the window.

## 2. Theming
- [ ] Install a theme (e.g. Doki) → editor, sidebar, activity bar, tabs, status bar, **title bar** all repaint.
- [ ] AIRI panel keeps its purple branding (by design).

## 3. Agent / chat (the core)
- [ ] **Agent mode**: a coding request writes files / runs tools (full loop).
- [ ] **Chat mode**: "who are you" returns a real conversational answer **instantly** (no 600s loop, no static "Ready.").
- [ ] Multi-turn: a follow-up keeps the **full prior conversation visible** (scrolls to newest).
- [ ] No `example.com` spam / phase-wrap churn in the console (autonomy off by default).

## 4. Shipped feature integrations (#60–#69)
- [ ] **Custom modes** — Settings → Sentient Core → *Custom Agent Modes*: add one (name + prompt + optional model + read-only), it appears in the mode picker, agent adopts the persona.
- [ ] **MCP marketplace** — MCP settings → *Marketplace*: search + one-click install (e.g. filesystem); it appears in the installed list.
- [ ] **Checkpoint timeline** — HISTORY tab → *Code Checkpoints*: expand one → real per-file diff; Restore / Delete work.
- [ ] **Agent manager** — spawn a `/bg <prompt>` parallel agent → tray shows status, live duration, expandable output, remove.
- [ ] **Auto hooks** — create an `on_save` hook (HooksPanel) → saving a matching file fires it; an `on_commit` hook fires after a commit (not on save).
- [ ] **Guided specs** — SpecsToCodeWizard → "✨ Guided (Requirements → Design)" generates both docs → review → Generate → tasks appear.

## 5. Vision (opt-in)
- [ ] Default: vision OFF, zero screen-capture overhead.
- [ ] Settings → *Vision System* ON (with a VL model set) → screen capture engages.

## 6. Extensions (Open VSX)
- [ ] Extensions view → search Open VSX → install a simple extension → it activates (ext-host has the common `vscode` API classes).

## 7. Offensive security (real, not simulated)
- [ ] `apex_red_team_scan` / `weaponize_env` run without permission-dialog gating.
- [ ] Web security tests (SQLi/XSS/IDOR/headers) make real HTTP requests via `http_probe`.

## 8. Memory / performance
- [ ] RSS stays roughly within 200–400 MB during normal use.
- [ ] UI feels responsive while streaming (no whole-tree re-render lag).

## 9. Ship
```powershell
npx tauri build         # produces installer in src-tauri/target/release/bundle/
```
- [ ] Installer builds; launch the installed app; repeat §1–§3 smoke test.

---

## Known limitations (documented, not bugs)
- **#63 predictive Tab (jump-to-next-edit)** — not implemented; needs a diff-prediction model + live UX tuning.
- **#68 claurst** — intentionally NOT integrated (would duplicate the Sentient agent).
- **iPhone emulator** — real iOS boot is IPSW-gated and host-dependent (JIT on x86, HVF on M1); see `HANDOFF.md`.
- 2 benign `cargo` warnings in `ai_engine.rs` (logic-adjacent, intentionally left).

## If something fails
Open devtools (Ctrl+Shift+I) → Console. Most handlers log precise errors. Paste the
error for a targeted fix.
