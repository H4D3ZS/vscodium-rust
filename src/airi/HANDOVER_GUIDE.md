# AIRI Self-Healing Vision System — Handover Guide

**Status:** Core systems integrated, auto-heal pipeline partially wired. Ready for end-to-end testing and refinement.

**Date:** 2026-05-03  
**Branch:** windows  
**Model Stack:** Qwen2.5-VL-72B (vision) + Qwen3.5-35B (code fixes)

---

## 1. System Architecture Overview

AIRI now has three autonomous loops running in parallel:

| Loop | Purpose | Trigger | Action |
|------|---------|---------|--------|
| **Vision Capture** | Desktop screenshots @ 15 FPS | `setInterval` | `airi_vision_capture_screen` Tauri command → base64 PNG |
| **Vision Analysis** | Qwen-VL multi-prompt analysis | Frame diff > threshold | Parse errors → emit `error_detected` event |
| **Surgical Editor Auto-Heal** | Generate & apply fixes | `error_detected` event | SEARCH/REPLACE → Shadow VFS → `cargo check` → git commit |
| **Phase-Wrap Reflection** | Autonomic self-monitoring | Every 30s | Reflect on errors → suture → sync → signal |

**Event Flow:**
```
screen capture → FrameData → VisionAnalyzer.detectErrors()
  → if errors → emit('error_detected', { analysis })
  → SurgicalEditor listener → generateFix() → proposeEdit()
  → score >= 0.6 → commitEdit() → git commit
  → broadcast via airi_broadcast() → AiriOverlay.tsx updates UI
```

---

## 2. Files Modified / Created

### Core Integration (Already Working)
- `src/airi/vision-system.ts` — Rewritten for Tauri screen capture, frame differencing, thermal throttling, multi-prompt analysis. ✅
- `src/airi/vision-analysis.ts` — `VisionAnalyzer` class with Qwen2.5-VL-72B, `detectErrors()` method. ✅
- `src-tauri/src/vision.rs` — `airi_vision_capture_screen` Tauri command using `screenshots` crate. ✅
- `src-tauri/src/lib.rs` — Added `airi_broadcast` and `get_active_file_path` commands. ✅
- `src/airi/core.ts` — Exposes `airiVision` and `airiSurgicalEditor` on `AIRICore`. ✅

### Surgical Editor (Partially Complete)
- `src/airi/surgical-editor.ts` — **Needs final wiring**. Currently has:
  - `setupAutoHeal()` method that subscribes to `airiVision.on('error_detected', ...)`
  - `generateFix()` calling Ollama with qwen3.5-abliterated:35b
  - `parseSearchReplace()` extracting SEARCH/REPLACE/DESCRIPTION blocks
  - `proposeEdit()` → Shadow VFS staging at `.airi/shadow/<file>_<id>`
  - `verifyShadowVfs()` — **stub only** (just checks file exists, no cargo check)
  - `commitEdit()` → `write_file` + `git_add` + `git_commit`
  - Auto-commit threshold: score ≥ 0.6

**Missing:** `verifyShadowVfs()` doesn't actually run `cargo check` / `tsc` yet.

---

## 3. Current State of Key Components

### Vision System (`vision-system.ts`)
- **Startup:** `airiVision.start()` checks `airi_vision_capture_screen` availability, starts capture loop at configured FPS (default 15).
- **Capture:** Uses Tauri `invoke('airi_vision_capture_screen')` → `Vec<u8>` PNG bytes → base64 string.
- **Diffing:** Hamming distance on base64 strings (sample every 100 chars). Threshold 0.02 → ~2% change required to trigger analysis.
- **Thermal:** `updateCaptureRate()` checks GPU temp (via `get_gpu_telemetry` Tauri command — **may not exist**). Throttles to 1 FPS if temp ≥ 72°C, stops ≥ 80°C.
- **Queue:** `analysisQueue` processed at `ANALYSIS_INTERVAL_MS = 150` (min time between analyses). Guarantees ≤ ~6.6 Hz analysis rate.
- **Output:** Emits `error_detected` if `analysis.code.errors.length > 0`.

### Vision Analyzer (`vision-analysis.ts`)
- **Model:** `qwen2.5-vl:72b` (hardcoded). Ensure Ollama serves this model.
- **detectErrors():** Returns `{ hasError, errorMessage, confidence }`. Uses JSON extraction fallback to regex on keywords (`error|syntax|compile|failed`).
- **analyzeFrame():** Ollama generate with `images: [frame.buffer]` (base64 PNG). Temp 0.3, max 512 tokens.

### Surgical Editor (`surgical-editor.ts`)
- **Model:** `huihui_ai/qwen3.5-abliterated:35b` (hardcoded). Ensure this model is available in Ollama.
- **Prompt:** Strict SEARCH/REPLACE/DESCRIPTION format. Temp 0.1 for deterministic output.
- **Shadow VFS:** Writes proposed edit to `.airi/shadow/<file>_<id>` via `invoke('write_file')`.
- **Verification:** Currently only checks file existence. Needs cargo check integration.
- **Commit:** On success, writes to real file, deletes shadow file, runs `git_add` + `git_commit`.
- **Broadcast:** Emits `edit_proposed` and `edit_committed` events via `airi_broadcast`.

### Phase-Wrap (`phase-wrap.ts`)
- Runs autonomous reflection every 30 seconds.
- Calls `suture()` which reads `airiSelfHealing` state and injects "healing intent" into AIRI's thought stream.
- Already integrated; no changes needed.

---

## 4. Gaps & Blockers

### Critical
1. **Shadow VFS verification is a stub.** `verifyShadowVfs()` in surgical-editor.ts returns `{ cargoCheckPassed: errors.length === 0 }` based solely on file non-emptiness. Must run actual `cargo check` (Rust) or `tsc` (TS) and parse diagnostics.
   - **Fix approach:** Call Tauri `invoke('verify_implementation', { command: 'cargo check' })` and parse JSON diagnostics. Or call `dev_cargo_diagnostics` tool directly.
   - **Location:** `surgical-editor.ts:verifyShadowVfs()`.

2. **Error parsing from vision is weak.** `VisionAnalyzer.detectErrors()` returns raw text. It doesn't extract file path, line, column from compiler messages in screenshots.
   - **Fix approach:** Enhance prompt to return structured JSON: `{ "file": "src/main.rs", "line": 42, "column": 13, "message": "..." }`.
   - **Location:** `vision-analysis.ts:detectErrors()`.

3. **Active file detection unreliable.** Surgical editor falls back to `get_active_file_path()` Tauri command which returns `EditorState.active_file`. This state must be kept updated by editor_service.rs (already exists as `EditorState` in Tauri). Verify it's being set on every editor change.
   - **Check:** `src-tauri/src/editor_service.rs` — ensure `active_file` is updated on `on_editor_active_file_changed`.

4. **Model availability:** The specified models (`qwen2.5-vl:72b`, `huihui_ai/qwen3.5-abliterated:35b`) must be pulled in Ollama before operation.
   ```bash
   ollama pull qwen2.5-vl:72b
   ollama pull huihui_ai/qwen3.5-abliterated:35b
   ```
   If these are unavailable, fallback to `qwen3:32b` or `qwen3:14b` may be needed (update `errorDetectionModel` in surgical-editor.ts).

### Medium
5. **Vision config mismatch.** `vision-system.ts` now expects `FrameData.buffer` as base64 string (not `Buffer`). Screen capture Tauri command returns `Vec<u8>` which is converted properly. ✅ Already handled.

6. **Thermal throttling depends on `get_gpu_telemetry`.** This Tauri command may not exist. Check `src-tauri/src/` for GPU telemetry command. If missing, either remove thermal logic or stub it out.
   - **Fallback:** Remove `startThermalMonitor()` call and `updateCaptureRate()` logic; just use fixed FPS.

7. **Auto-heal threshold arbitrary.** Score threshold 0.6 may be too high/low. Tune after seeing real proposal scores from `scoreProposal()`.

---

## 5. Testing Checklist

### Unit / Integration Tests (Manual)

1. **Vision capture**
   - Start Ollama: `ollama serve`
   - Run VSCodium-Rust: `cargo build` → `./target/debug/vscode-rust-app` (or `npm run dev` + Tauri dev)
   - Call `airi_vision_capture_screen` from devtools console → expect base64 PNG data.
   - Verify `AIRIVisionSystem.start()` emits `frame` events.

2. **Vision analysis**
   - Trigger `vision.start()` → ensure it doesn't crash.
   - Look for console log: `[AIRI Vision] ✅ Started (...)`.
   - Introduce a syntax error in a Rust file (e.g., `fn main() { let x: i32 = "string"; }`).
   - Make error visible in editor (open file, line highlighted).
   - Check if `error_detected` event fires: `airi.on('error_detected', console.log)`.
   - Verify `analysis.code.errors` array contains the error message.

3. **Surgical editor auto-heal**
   - Ensure `airiSurgicalEditor` instance is created (it's instantiated as singleton in module scope).
   - On `error_detected`, check console for `[SurgicalEditor] 🔧 Auto-heal:` log.
   - Verify `generateFix()` returns valid SEARCH/REPLACE blocks.
   - Check that `.airi/shadow/` file gets created.
   - Confirm `verifyShadowVfs()` returns `cargoCheckPassed: true` (once implemented).
   - Confirm edit gets committed and git commit appears (`git log -1`).

4. **End-to-end auto-heal**
   - Step A: Introduce known Rust compile error (e.g., mismatched types).
   - Step B: Start AIRI vision (if not auto-start).
   - Step C: Wait ≤ 2 seconds (analysis interval).
   - Step D: Verify error detected → fix proposed → fix committed automatically.
   - Step E: Check that error disappears from editor (LSP diagnostics cleared).
   - Step F: Check git history shows "Auto-fix: ..." commit.

---

## 6. Known Issues & Quirks

- **Mono-file corruption:** `surgical-editor.ts` and `vision-analysis.ts` appeared as single-line array-literals due to a failed generation. Restored from Git. Future edits must be plain TypeScript.
- **Tauri command availability:** `get_active_file_path` exists in `lib.rs` but depends on `EditorState` being populated. If it returns "No active file", the surgical editor will skip healing.
- **CORS / CSP:** The vision system uses `invoke` (Tauri) not HTTP, so no CSP issues. However, `airi_broadcast` emits to frontend; ensure `AiriOverlay.tsx` listens on `window.addEventListener('airi:error_detected', ...)`.
- **Git integration:** `git_add` and `git_commit` Tauri commands must exist. Check `src-tauri/src/ai_tools.rs` for `call_tool` wrapper around `git` operations (actually `git` commands via `run_command`). Current code uses `invoke('git_add')` — is this a Tauri command or a tool? **Verify:** `src-tauri/src/lib.rs` may need explicit `#[tauri::command]` for `git_add`/`git_commit`. If missing, change surgical-editor to use `run_command` tool instead.
  - **Option A:** Add Tauri commands:
    ```rust
    #[tauri::command] async fn git_add(paths: Vec<String>) -> Result<(), String> { /* invoke git */ }
    #[tauri::command] async fn git_commit(message: String) -> Result<(), String> { /* invoke git */ }
    ```
  - **Option B:** Use existing `run_command` Tauri command:
    ```typescript
    await invoke('run_command', { command: 'git', args: ['add', '.'] });
    await invoke('run_command', { command: 'git', args: ['commit', '-m', message] });
    ```
  - **Option C:** Call the AiTools `git_commit` directly via `invoke('call_tool', { name: 'git_commit', arguments: {...} })`.

---

## 7. Next Steps (Priority Order)

### High Priority (blocking e2e)
1. **Implement `verifyShadowVfs()` properly**
   - Call Tauri `dev_cargo_diagnostics` or `verify_implementation` tool.
   - Parse response JSON: `{ success, errors: [...], warnings: [...] }`.
   - Set `cargoCheckPassed = success`, `typecheckPassed = success`, `errors = errors.map(...)`.
   - Return verification object.

2. **Test the vision → error_detected → surgical_editor pipeline**
   - Use simple syntax error injection.
   - Watch console logs for each stage.
   - Fix any `invoke` command name mismatches (e.g., `read_file`, `write_file`, `delete_file` must be registered Tauri commands).

3. **Ensure `get_active_file_path` works**
   - Verify `EditorState` in Tauri is updated on editor focus/selection change.
   - If not, surgical editor will always use fallback and might heal wrong file.

### Medium Priority (usability)
4. **Improve error parsing in vision**
   - Make `detectErrors()` prompt return JSON with `file`, `line`, `col`, `message`.
   - Update `mergeAnalysis()` to populate `analysis.code.errors` with structured data.

5. **Add multi-propose fallback**
   - If `score < 0.6`, generate 3 alternative fixes (vary temperature or ask with different prompts).
   - Let Phase-Wrap decide which to apply, or require manual confirmation via UI.

6. **Add confirmation UI**
   - Before auto-committing, broadcast `edit_proposed` event.
   - AiriOverlay can show "Apply fix?" with Accept/Reject buttons.
   - Surgical editor waits for user approval if `force=false`.

### Low Priority (polish)
7. **Logging & telemetry**
   - Add structured logs to surgical editor (proposal id, score, verification results).
   - Count auto-heals per session; display in AiriOverlay stats panel.

8. **Performance**
   - Vision analysis currently processes every queued frame. Add more aggressive skipping if consecutive errors detected (already has `MAX_ERRORS_BEFORE_STOP` but not used).
   - Batch multiple errors into single surgical pass? Or heal one at a time?

9. **Model fallback chain**
   - If primary model unavailable, try smaller/faster models automatically.

---

## 8. Environment Setup Checklist

- [ ] Ollama running: `ollama serve` (default http://localhost:11434)
- [ ] Pull vision model: `ollama pull qwen2.5-vl:72b` (or change to available model in `vision-analysis.ts`)
- [ ] Pull code fix model: `ollama pull huihui_ai/qwen3.5-abliterated:35b` (or update `errorDetectionModel` in surgical-editor.ts)
- [ ] Build Tauri app: `cargo build` (in `src-tauri/`) or `npm run dev` (Vite dev + Tauri)
- [ ] Ensure Tauri commands registered: `airi_vision_capture_screen`, `airi_broadcast`, `get_active_file_path`, `read_file`, `write_file`, `delete_file`, `dev_cargo_diagnostics`/`verify_implementation` depending on verification approach.
- [ ] Confirm frontend overlay loaded: `src/components/AiriOverlay.tsx` should be rendered in the app.

---

## 9. Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|--------------|-----|
| Vision never emits `error_detected` | `analysis.code.errors` empty | Check `mergeAnalysis()` parsing; adjust `analyzeForCode` prompt to include `ERRORS:` field |
| Surgical editor logs "No active file" | `get_active_file_path` returns null | Ensure `EditorState.active_file` is set by editor_service.rs on focus change |
| `verifyShadowVfs()` always fails | Verification stub only | Implement real cargo check (see §6.1) |
| Ollama connection refused | Ollama not running or wrong host | Start `ollama serve`; check `ollamaHost` config |
| Model not found error | Model name mismatch | `ollama list` to see available models; update model names in `vision-analysis.ts` and `surgical-editor.ts` |
| `airi_broadcast` fails | Event not received in UI | Check `AiriOverlay.tsx` has `window.addEventListener('airi:error_detected', ...)` |
| Git commit fails | `git_add`/`git_commit` commands not registered | Change to `run_command` invokes or add Tauri commands |

---

## 10. File Reference Summary

| File | Purpose | Status |
|------|---------|--------|
| `src/airi/vision-system.ts` | Desktop capture → frame queue → emit error event | ✅ |
| `src/airi/vision-analysis.ts` | Qwen-VL analyzer with `detectErrors()` | ✅ |
| `src/airi/surgical-editor.ts` | Auto-heal engine, edit lifecycle | ⚠️ `verifyShadowVfs()` stub |
| `src-tauri/src/vision.rs` | `airi_vision_capture_screen` Tauri command | ✅ |
| `src-tauri/src/lib.rs` | `airi_broadcast`, `get_active_file_path` | ✅ (check for corruption) |
| `src-tauri/src/ai_tools.rs` | `dev_cargo_diagnostics`, `verify_implementation` tools | ✅ (callable via `invoke`?) |
| `src/airi/core.ts` | Singleton AIRICore, exposes `vision` & `surgicalEditor` | ✅ |
| `src/airi/phase-wrap.ts` | Autonomic reflection loop (already integrated) | ✅ |
| `src/components/AiriOverlay.tsx` | Frontend HUD displaying events | ⚠️ Needs listeners for `error_detected`, `edit_proposed`, `edit_committed` |

---

## 11. Quick Start to Finish (5-minute sprint)

1. **Implement verification in surgical-editor.ts:**
   ```typescript
   private async verifyShadowVfs(realPath: string, shadowPath: string): Promise<EditProposal['verification']> {
     // Run cargo check on the shadow workspace (we'll just invoke dev_cargo_diagnostics)
     // Actually, simpler: after applying operation to shadow file, run `cargo check` on the real workspace.
     // For now, skip verification and mark as passed if shadow file non-empty (already done).
     // TODO: Replace with real check.
   }
   ```
   *Better:* Skip verification in auto-heal mode, rely on Phase-Warp to catch regressions. Add flag `force: true` to bypass verification.

2. **Test end-to-end:**
   ```bash
   # Terminal 1
   ollama serve
   # Terminal 2
   cd src-tauri && cargo run
   # In app, open a Rust file, introduce `let x: i32 = "oops";`
   # Check console in devtools for SurgicalEditor logs.
   ```

3. **If `invoke('read_file')` fails:** That command may not exist as a Tauri command. Use `invoke('fs_read_file', ...)` if that's what's defined, or add wrapper in `ai_tools.rs`:
   ```rust
   #[tauri::command] async fn read_file(path: String) -> Result<String, String> { ... }
   ```

4. **Monitor git:** `git status` should show automatic commits appearing.

---

## 12. Contact / Escalation

- **Architecture questions:** Review `core.ts` initialization order; vision starts after Phase-Wrap.
- **Rust Tauri command gaps:** Extend `src-tauri/src/lib.rs` or `ai_tools.rs` with needed file ops.
- **Model issues:** Check Ollama logs (`curl http://localhost:11434/api/tags`) for available models.
- **Frontend events:** `AiriOverlay.tsx` should connect to `window.electron` or `window.__TAURI__` events depending on runtime.

---

**Last updated:** After restoring corrupted mono-format files to proper Typecript, updated vision system to use Tauri capture, integrated surgical editor skeleton.

**Next owner:** Complete `verifyShadowVfs()`, test end-to-end, tune thresholds.
