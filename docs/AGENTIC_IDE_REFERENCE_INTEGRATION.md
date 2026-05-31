# Agentic IDE Reference Integration

This document captures the implementation path for rebuilding Antigravity, Cursor, and Void-class agentic IDE features inside VSCodium-Rust from scratch. It uses local reference code and product behavior to guide architecture, while keeping implementation owned by this repo.

## Reference Boundary

| Reference | Local path | Use |
| --- | --- | --- |
| Antigravity IDE | `C:\Users\HADES\Desktop\vscodium-rust\Antigravity IDE` | Product and architecture reference for task-level agent orchestration, visual artifacts, visual feedback, browser scopes, terminal scopes, and agent manager UX. Do not copy bundled private code. |
| Void | `C:\Users\HADES\Desktop\vscodium-rust\void` | MIT-licensed source reference for VS Code fork integration, provider routing, apply/diff flows, inline edit, context gathering, tool service boundaries, and MCP plumbing. |
| Manus runtime copy | `C:\Users\HADES\Desktop\vscodium-rust\manus_ai_source_code` | Runtime topology reference only. The copy appears incomplete and mostly contains system/service layout, supervisor config, browser/VNC process wiring, and caches rather than full app source. |
| Invisible Playwright | `C:\Users\HADES\Desktop\vscodium-rust\invisible_playwright` | Optional local browser automation backend reference for screenshots and page interaction. Do not expose anti-detection or evasion behavior as an IDE feature. |

## Current Local Primitives

The repo already has a strong base for the target IDE:

- Agent manager UI: `src/components/AgentManager/AgentManager.tsx`
- Agent messages, artifacts, tasks, pending changes, checkpoints: `src/store.ts`
- Diff review service: `src/services/DiffService.ts`
- Floating Monaco diff view: `src/components/agent/AgentDiffView.tsx`
- Session persistence and subagent progress bridge: `src/task_manager.ts`
- Browser frontend bindings: `src/browser.ts`
- Browser backend commands: `src-tauri/src/browser.rs`
- Agent tools for artifacts, screenshots, subagents, checkpoints, git diff, browser actions: `src-tauri/src/ai_tools.rs`
- Kortex AIM memory and VFS hooks: `.aim/memory.aim`, `src/kortex/aim-vfs.ts`, `kortex/daemon/src/bin/aim-vfs.rs`

The Kortex AIM VFS is available in this workspace. The binary at `kortex/target/release/aim-vfs.exe` responds to `get-gist`, and the project memory file exists at `.aim/memory.aim`.

## Kortex AIM Trust Layer

The AIM layer should be used as the first orientation call for every agent backend, including Ollama, Qwen Code, DeepSeek, OpenAI, Anthropic, Gemini, and browser-backed WebUI sessions. The goal is to route large private code context through local compressed memory while still preserving exact-source correctness.

First trust primitive:

- Backend command: `src-tauri/src/kortex_commands.rs::aim_trust_manifest`
- Renderer helper: `src/kortex/aim-vfs.ts::getAimTrustManifest`
- Exact span command: `src-tauri/src/kortex_commands.rs::aim_query_spans`
- Renderer helper: `src/kortex/aim-vfs.ts::queryAimSpans`
- Agent tool: `src/tool_registry.ts::aim_pack_context`, which combines trust metadata and exact spans into one compact provider-neutral evidence packet.

The trust manifest reports:

- AIM file path, size, SHA-256, modified time, and parsed JSON header
- Git HEAD and dirty file count
- validity and confidence score
- reasons that lower trust, such as missing file, invalid header, stale file, or dirty workspace

Agent rule:

- If confidence is high, use AIM as the map and only read exact target spans before edits.
- If confidence is degraded, use AIM for rough orientation but verify with exact files.
- If missing or invalid, fall back to normal source inspection and trigger AIM regeneration.

Exact span retrieval returns bounded `{ file, absolute_path, line_start, line_end, hash, summary, snippet }` windows. It now prefers the persisted `.aim` symbol graph and project tree before opening files, then falls back to bounded source scanning only when the AIM index is cold. This removes the need for broad repo grep in common cases: agents can ask AIM for likely spans, inspect those exact files, and only widen search when the manifest is degraded or the spans are insufficient.

## Target Architecture

### 1. Agent Run Graph

Implement an Antigravity-style task abstraction above chat messages:

- `AgentRun`: one user objective, root thread, status, model profile, checkpoints, artifacts, verification summary.
- `AgentStep`: normalized tool call, browser action, terminal action, file write, MCP call, subagent event, or verification action.
- `AgentSubtask`: background child run with parent relationship, title, progress, status, and result.
- `AgentRunEvent`: append-only stream for replay, restore, and task-level confidence.

Existing fit:

- Extend `AgentTask` and `AgentMessage` in `src/store.ts`.
- Persist runs under `.agent/runs/<run_id>.json`.
- Keep `TaskManager.saveSession()` for compatibility, but add run-oriented persistence next to it.

### 2. Artifact System

Make artifacts first-class objects instead of message attachments only:

- Screenshot artifact
- Browser DOM snapshot artifact
- Browser video or frame sequence artifact
- Terminal log artifact
- Diff artifact
- Task plan artifact
- Verification artifact
- Walkthrough artifact
- Visual comment thread artifact

Existing fit:

- Extend `Artifact` in `src/store.ts`.
- Add a global artifact index keyed by run id.
- Keep message-level artifact display as a projection of the global index.
- Update `ArtifactCard` in `src/components/AgentManager/AgentManager.tsx` to support preview, comments, approve, reject, and reopen in a review pane.

### 3. Visual Feedback

Rebuild the Antigravity visual feedback loop:

- Capture screenshots from local preview/browser surfaces.
- Let the user place anchored comments on screenshots.
- Convert comments into context items for the next agent run.
- Attach visual feedback to the originating artifact and run.

Implementation path:

- Add `VisualComment` to artifact metadata.
- Add an artifact viewer component that renders screenshots and comment pins.
- Add `visual_feedback` context item type.
- Add a tool prompt rule that visual feedback must be treated as higher-priority user direction.

### 4. Browser Harness

Use browser automation as a dev verification and artifact harness:

- Open local URLs and external docs.
- Navigate, click, type, and inspect DOM.
- Capture screenshot and DOM summary together.
- Record deterministic verification artifacts.
- Keep browser use scoped to development and testing workflows.

Existing fit:

- `src-tauri/src/browser.rs` already supports open, navigate, screenshot, click, type, DOM read, and vision context.
- `kortex/mcp-servers/browser-agent/mcp_server.py` already provides a Playwright MCP option.
- Invisible Playwright can be evaluated as an optional backend for local browser automation only, not for stealth or bypass behavior.

### 5. Diff And Apply Engine

Port the best Void concepts into this repo:

- Fast apply: search/replace or structured hunks for small edits.
- Slow apply: full file rewrite with diff review for complex edits.
- Hunk-level accept/reject.
- Per-file approval state.
- Auto-checkpoint before agent writes.
- Verification gate after write.

Existing fit:

- `src/services/DiffService.ts` has diff blocks and selective patching.
- `src/store.ts` already has `PendingChange`, accept/reject, hunk state, and auto-accept.
- `src-tauri/src/patch_engine.rs` uses exact patching and `diffy`.
- Next step is to replace random hunk ids with deterministic ids so accept/reject survives re-render and recomputation.

### 6. Context Graph

Represent all context as typed scope items:

- File
- Folder
- Symbol
- Git diff
- Browser page
- Browser DOM element
- Browser screenshot region
- Terminal command/output
- MCP resource
- Rule or memory
- Conversation
- Visual feedback
- Kortex AIM gist

Existing fit:

- `AttachedContext` in `src/store.ts` is the seed.
- Extend it into a discriminated `ContextItem`.
- Let Kortex AIM provide the compressed memory layer while explicit context items carry precise user-selected evidence.

### 7. Provider, Model, And MCP Layer

Use Void as the reference for clean service boundaries:

- Provider settings service
- Model capability map
- Send message service in backend/main process
- Tool registry
- MCP server registry
- Cost, latency, and context policy per model

Existing fit:

- `src/tool_registry.ts`, `src/agent.ts`, `src/system_prompt.ts`, and backend `ai_tools.rs` already contain pieces.
- Move provider-specific logic behind a service boundary before adding more models or custom agents.
- Local Ollama is the default offline backend and should always be discoverable without an API key.
- BYOK cloud providers should all flow through `api_keys.json`, `list_provider_models`, and the same model picker.
- Browser-backed Claude, ChatGPT, and Gemini should become explicit `webui:*` backends with a real prompt/response bridge, artifact capture, and visible login state. Do not route these through API endpoints unless the user supplied an API key.

Supported BYOK surface now includes OpenAI, Anthropic, Google/Gemini, Groq, OpenRouter, DeepSeek, Mistral, xAI, Cerebras, Alibaba DashScope, ApiRadar, and optional Ollama bearer auth.

Initial WebUI routing is now split from API routing:

- `src-tauri/src/auth_commands.rs::send_webui_prompt` opens/focuses a visible Claude, ChatGPT, or Gemini web session and injects the prompt into the provider UI.
- `src-tauri/src/auth_commands.rs::save_webui_response` receives provider-window DOM observer output, emits `webui-response`, and stores the turn as a `record` artifact with `metadata.kind = "webui_response"`.
- `src/agent.ts` skips Ollama preflight for official WebUI model selections, performs login readiness, calls `send_webui_prompt`, and stops before the normal API/agent loop.
- OpenWebUI remains API-routed through its local OpenAI-compatible endpoint.
- DeepSeek and Qwen WebUI sessions are now first-class alongside Claude, Gemini, and ChatGPT. `src/agent.ts::buildWebUiAgentPrompt` packs Kortex AIM trust data, exact source spans, and the active-editor preview into the prompt before sending it to the browser model.

Next WebUI step: render `webui-response` artifacts inline in the agent conversation and add screenshot capture to the same artifact record.

### 8. Cursor-Style Editor Features

Reimplement Cursor-class editing inside the current Monaco/Tauri surface:

- Inline selection edit
- Command bar edit
- Tab autocomplete
- Multi-file apply with review
- Checkpoint restore per turn
- Chat references to file/symbol/git/browser artifacts

Existing fit:

- Settings for tab prediction and fast apply already exist in `src/store.ts`.
- `AgentDiffView` and pending changes provide the review path.
- Add selection context capture from Monaco, then feed that into the same apply engine.

### 9. AIRI VRM Gate

The AIRI side should keep the VRM renderer disabled unless hardware capability passes. The capability test should check WebGL2, max texture size, CPU threads, exposed memory, and software renderers. If hardware passes, the user can enable VRM manually.

Implemented files:

- `airi/packages/stage-ui/src/stores/settings/vrm-capability.ts`
- `airi/packages/stage-ui/src/stores/settings/vrm-capability.test.ts`
- `airi/packages/stage-ui/src/stores/settings/stage-model.ts`
- `airi/packages/stage-ui/src/stores/settings/index.ts`
- `airi/packages/stage-ui/src/components/scenarios/settings/model-settings/panel.vue`

## Implementation Order

1. Stabilize artifact and run data models.
2. Add deterministic hunk ids to the diff service.
3. Add global artifact store and screenshot preview.
4. Add visual comments on screenshot artifacts.
5. Persist agent runs and artifacts under `.agent/runs`.
6. Add context item taxonomy and connect visual feedback.
7. Normalize browser captures into screenshot plus DOM artifacts.
8. Port Void-style provider/tool/MCP service boundaries.
9. Add Cursor-style inline selection edit.
10. Add background subagent run graph and verification summary.

## First Patch Candidates

The safest first code changes are:

- Replace random diff hunk ids in `src/services/DiffService.ts` with deterministic ids based on line ranges and content hash.
- Extend `Artifact` with review state and optional `comments`.
- Add `src/services/AgentRunService.ts` for `.agent/runs` persistence.
- Add screenshot artifact preview and comment pins in `AgentManager`.
- Add browser capture normalization so every screenshot also stores URL, title, DOM summary, viewport, and timestamp.

## Verification

For each integration slice:

- Run the smallest relevant TypeScript test or typecheck.
- Run `cargo check` from `src-tauri` for backend command changes.
- Use browser screenshot artifacts for UI changes.
- Create a checkpoint before agent-applied file edits.
- Update Kortex AIM memory or run `aim-vfs get-gist` before long context-heavy work.
