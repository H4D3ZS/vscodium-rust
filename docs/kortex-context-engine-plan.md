# Kortex Context Engine — Project Plan

**Owner:** Rolando H. Ferrer Jr. (H4D3ZS / Cyber Ifrit)
**Purpose:** Self-contained handoff for a CLI coding agent with no prior
conversation context. Build the layer that lets a 35B model on a 16 GB card run
a full agent harness (Claude-Code-class) with an *effective* context far larger
than its real `n_ctx`.

---

## 0. Goal, stated precisely

> A local 35B-A3B model at `n_ctx = 32768` behaves, for agent work, as if it had
> 128k–256k of context — because Kortex compresses, caches, and expands the
> pieces on demand instead of stuffing everything into the window.

Three sub-claims, never conflate them:

1. **Fits.** The request actually sent to `llama-server` stays under `n_ctx`.
2. **Cheap.** The large static parts (system prompt, tool schemas) are prefilled
   once and reused; identical requests are answered without the model.
3. **Lossless where it counts.** Compression never causes a wrong tool call or a
   hallucinated file; anything dropped is retrievable via an explicit tool.

Physics that bound this (do not plan around targets they rule out):

- KV cache for a 35B at 256k ≈ **~17 GB** even at q4 — larger than the whole
  card. **Real `n_ctx` maxes at ~32k** alongside the 2.5-bpw weights.
- So "256k" is a *budget the context manager works to*, not a llama.cpp flag.
- Compression moves the bottleneck; it does not repeal attention math.

---

## 1. Current state (built 2026-09-06, on `main`)

| Component | File(s) | State |
|---|---|---|
| KV-slot cache proxy (Tier 1, KDKVC) | `src-tauri/src/kortex_kvcache/` | working; `:1537`, longest-prefix match → `/slots` restore |
| Geometry launcher | `src-tauri/src/kortex_gac/` | working; log-drain fixed the load deadlock, `kortex_gac_log` command, `alive`/`exit_code` in status, slot-dir auto-create |
| **Harness compressor** | `src-tauri/src/kortex_harness/` | **`tool_digest.rs`** (schema → `name(a: str!, b?: int) — purpose`), **`contract.rs`** (Hermes calling contract + `expand` tool def + `tool_call_grammar()` GBNF), **`mod.rs`** (`compress_openai_request`, `rehydrate`, `HarnessConfig::from_env`). 14 tests. `KORTEX_HARNESS=1` opt-in. |
| Harness wiring | `kortex_kvcache/proxy.rs` (before prefix extract), `domain/ai/engine/autonomous.rs` (OpenAI tool path) | request `tools` array → compact block in the system message + inline core tools + synthetic `expand`; re-serialized before forward |
| Per-tool hooks | `domain/ai/engine/tool_hooks.rs` | `.claude/settings.json` `PreToolUse`/`PostToolUse`, wired at the `autonomous.rs` tool-exec point; PreToolUse can block |
| Launch defaults | `KortexLocalInferencePanel.tsx` | `-c 32768`, `--cache-type-k/v q4_0`, `--jinja` |
| Shutdown | `lib.rs` `on_window_event` | main-window close → `kortex_gac::stop_server()` |
| Embeddings routing fix | `application/commands/ai.rs` `set_lemonade_url` | no longer repoints the vector indexer's embed URL at the chat backend (that stalled indexing at 108 chunks) |

**Also:** native agent loop is the default (`useClaudeCodeAgent` off);
`MASTER_LEAN_PROMPT` rewritten tight/CC-style; `claude_code.rs` uses a valid
model alias + probes real `/props` `n_ctx` + refuses below 24k.

### Landed since (branch `chore/remove-airi-vrm`)

| Piece | Commit | State |
|---|---|---|
| §2.1 `expand()` handler | `70d89e73` | **done** — `kortex_harness/stash.rs` (model-keyed schema stash, FIFO 8 models, sticky set), wired into `compress_openai_request`; `expand` tool + `handle_expand` + dispatch. 20 tests. |
| §2.2 GBNF probe | `699a38fe` | **done** — `capability::probe_grammar`; `constrain_grammar` ANDed with the probe so a non-supporting upstream can't be broken. Probe only when `KORTEX_HARNESS_GRAMMAR=1`. |
| §2.5 recallable compaction | `9f9434c4` | **done (v1)** — `kortex_harness/turn_stash.rs` (id→text, 2 MB budget); `compress_old_tool_results` stashes the full result + leaves `recall({"id"})` when `KORTEX_HARNESS=1`; `recall` tool. |
| §3.1 skills runtime | `3b62e6c0` | **done** — `domain/skills/mod.rs` (`.claude/skills` + `.agent/skills` scan, frontmatter, dedup); `use_skill`/`search_skills` are real; `skill`/`list_skills` aliases. |
| Remote agent bridge | `40b7b4a0` | **done** — `remote_bridge.rs`: 127.0.0.1 WebSocket → `Sentient::autonomous_loop`, token auth, `KORTEX_REMOTE=1` autostart, `remote_bridge_{start,stop,status}`. |

### Still open

- **§2.3 `/v1/messages`** — needs Anthropic↔OpenAI translation + fixtures from a
  real Claude Code capture. Supervised.
- **§2.4 Tier 2 response cache** — the plan's main deliverable; touches the hot
  proxy path. Supervised.
- **§3.2 sub-agent nesting**, **§3.3 Tier 0 residency** — orthogonal.
- **§2.6 (new, from FreeToken)** — see below.

## 2.6 Semantic-anchor KV checkpoints (FreeToken-inspired)

FreeToken (Nvidia-only, can't run on RDNA4) has one idea in kortex's lane:
*"semantic anchor checkpoints for recurrent state and KV caches, allowing
agentic context edits (tool calls, thinking blocks) to avoid redundant context
recomputation."*

KDKVC today saves a slot only at completion and matches by **longest token
prefix**. That's optimal when turn N+1 = turn N + more, but weak when the agent
**edits mid-context** (retries a tool, prunes a `<think>` block, a tool result
changed) — longest-prefix then matches only up to the first divergence, which
can be early.

**Design.** Save extra slots at **message boundaries** (after each `tool` result
and each `assistant` turn), not just at completion:

- On save, in addition to the tail slot, checkpoint at the end of the second-to-last
  and third-to-last message boundaries → `<sha>.a1.slotbin`, `<sha>.a2.slotbin`.
- On lookup, if the longest token-prefix match ends well before the request's
  last stable message boundary, fall back to the nearest **anchor** whose token
  prefix *is* a prefix of the request → restore that instead. Message boundaries
  are far more stable than raw BPE offsets across small edits.
- Bounded: at most 2–3 anchors per index entry; they share the entry's LRU/byte
  budget and model-identity gate.

**Files.** `kortex_kvcache/store.rs` (anchor entries), `proxy.rs` (anchor-aware
lookup in `handle_intercepted`), `types.rs` (`PrefixMatch` carries which anchor).

**Done when.** An agent turn that re-runs its last tool call (identical prompt
except the final tool block) still gets a KV-slot HIT covering everything up to
that tool block, where longest-prefix-only would have missed most of it. Add a
fixture that reproduces the mid-edit.

**Not adopting** from FreeToken: expert-offload / PCIe streaming / FTW weight
format — those are serving-engine (ROCmFPX/llama.cpp) concerns, not the proxy's.

---

## 2. The gap — five pieces, in build order

### 2.1 `expand()` handler (do first — completes the harness loop)

**Problem.** `compress_openai_request` adds an `expand` tool and stashes the full
schemas under `body["_kortex_harness"]["compacted_schemas"]`. But the request
body is gone by the time the agent's `tool_invoker` runs `expand`, so a model
that calls `expand` gets "unknown tool" or nothing.

**Design.**
- New process-global in `kortex_harness`:
  `static SCHEMA_STASH: Mutex<LruMap<StashKey, HashMap<String, Value>>>`
  where `StashKey = (model_id, session_id)` (session id from the request's
  `user` field or a header the agent sets; fall back to `model_id` alone).
- `compress_openai_request` writes the compacted schemas there (keyed), in
  addition to the in-body copy.
- New tool `expand` registered in the tool registry
  (`domain/tools/`, add to `schemas.rs` + a handler). Handler args
  `{ "tool": "<name>" }`. Implementation:
  1. look up `SCHEMA_STASH[key][tool]`;
  2. return it as the tool result (`{ "schema": <full json schema> }`);
  3. mark `(key, tool)` **sticky** so the next `compress_openai_request` for
     that key keeps that tool inline (full schema) instead of compacting it.
- Sticky set: `static STICKY: Mutex<HashMap<StashKey, HashSet<String>>>`; read
  it in `compress_openai_request`'s `is_core` check.

**Files.** `kortex_harness/mod.rs` (+ `stash.rs`), `domain/tools/schemas.rs`,
a new `domain/tools/expand_tool.rs`, `domain/tools/registry.rs` (register),
`autonomous.rs` (pass a session id through).

**Done when.** With `KORTEX_HARNESS=1`, a run where the model calls
`expand({"tool":"web_fetch"})` gets the full `web_fetch` schema back in one
turn, and `web_fetch` stays inline for the rest of that session. Unit test the
stash + sticky round-trip.

### 2.2 GBNF grammar — verify it reaches the model

**Problem.** `contract::tool_call_grammar()` is generated and
`compress_openai_request` inserts it as `body["grammar"]` when
`KORTEX_HARNESS_GRAMMAR=1`, but nothing confirms `llama-server` honors a
top-level `grammar` field on `/v1/chat/completions` through the proxy.

**Design.**
- Probe: send a `/v1/chat/completions` with a trivial grammar
  (`root ::= "PONG"`) and assert the response is exactly `PONG`.
- If the field name is wrong for this llama.cpp build, it may want
  `grammar` inside `body["grammar"]` vs `body["response_format"]` vs a
  `json_schema`. Try `grammar` first, then `grammar_lazy`, then log a warning
  and disable grammar mode (don't fail the request).
- Add `grammar_ok: bool` to the harness report + a one-time capability log line.

**Files.** `kortex_kvcache/capability.rs` (add a grammar probe alongside the
slot probe), `kortex_harness/mod.rs` (gate `constrain_grammar` on the probe
result).

**Done when.** A constrained request measurably prevents a malformed tool call
that the unconstrained path produced (add a fixture that reproduces the bad
call at `temperature=0`).

### 2.3 `/v1/messages` (Anthropic) proxy path

**Problem.** The proxy routes `/v1/chat/completions` and `/v1/completions`
only. Claude Code (`ANTHROPIC_BASE_URL`) sends `/v1/messages`, so neither the
harness compressor nor the KV cache reach that path.

**Design.**
- Add `.route("/v1/messages", post(handle_messages))` to `proxy::serve`.
- `handle_messages`: translate Anthropic → OpenAI in-place
  (`system` block(s) → a `system` message; `messages[].content` blocks →
  strings/tool_use/tool_result; `tools[].input_schema` → `parameters`), then
  reuse `handle_intercepted` with `IntercepKind::Chat`, then translate the
  OpenAI response back to Anthropic SSE / JSON on the way out.
- Keep the translation in one module, `kortex_kvcache/anthropic.rs`, with
  round-trip tests against captured Claude Code request/response fixtures.
- Non-goal: full Anthropic API fidelity. Cover what Claude Code actually sends
  (measured from a real session capture — commit the fixture).

**Files.** `kortex_kvcache/proxy.rs`, new `kortex_kvcache/anthropic.rs`.

**Done when.** `claude` (CLI) pointed at `:1537` completes a multi-tool turn
against Escha, and the KV-cache `hits` counter increments on turn 2.

### 2.4 Tier 2 — response cache (the "past `n_ctx`" mechanism for repeats)

Follow `docs/kortex-cache.md` §6 verbatim. Summary:

- Key = `SHA-256(canonical_render(messages) ‖ model_identity ‖ sampling_params)`
  where sampling_params includes every field that determines output
  (`temperature, top_p, top_k, seed, max_tokens, stop, penalties, tools,
  tool_choice, response_format`).
- **Only cache when `temperature == 0` OR an explicit `seed` is set.** Opt-in
  `cache_nondeterministic` for "good enough repeats", off by default.
- Never cache an error, a stop-truncated, or an empty response.
- Streaming: store the ordered SSE chunk list; replay with original framing;
  emit `x-kortex-cache: hit`.
- Store shape mirrors `store.rs` (LRU on a byte budget, atomic index writes,
  `ModelIdentity::accepts` gate).

**Files.** `kortex_kvcache/store.rs` (+ a `response` table/dir),
`kortex_kvcache/proxy.rs` (branch before the slot path when tier == Response,
and *also* as a fast pre-check when tier == Kv).

**Done when.** Re-running the exact same `temperature=0` agent prompt returns in
<50 ms with `calls_avoided` incremented and byte-identical output (including
stream framing).

### 2.5 Semantic history compaction (the actual "big window")

**Problem.** `agent_harness::compress_old_tool_results` only truncates. Long
agent sessions still overflow 32k.

**Design.**
- When the rendered prompt approaches `n_ctx * 0.7`, take turns older than the
  last N and replace each with a compact record:
  - tool results → first + last K lines + a one-line "(… X lines, full result
    via `recall`)".
  - assistant/user prose → a 1–2 sentence extractive summary (rule-based:
    first sentence + any line containing a decision verb / file path). No LLM
    call for v1 — deterministic, cheap, reversible.
  - stash the full turn in `.aim` (or the SCHEMA_STASH-style store) keyed by a
    `turn_id`.
- New tool `recall({ "turn": "<id>" })` — same pattern as `expand`, returns the
  full turn, marks it sticky.
- Optional v2: route the summary through the local model at `temperature=0`
  when the deterministic summary is too lossy (behind a flag).

**Files.** `domain/ai/agent_harness.rs` (or a new
`domain/ai/engine/history_compactor.rs`), `autonomous.rs` (call it in the
message-assembly path, near the existing `apply_tool_result_budget`),
`domain/tools/` (`recall` tool).

**Done when.** A 60-turn session stays under 32k with no "context overflow"
error, and `recall` on an old turn returns its full content.

---

## 3. Independent pieces (slot in anywhere)

### 3.1 Skills runtime
- Scan `<workspace>/.claude/skills/*/SKILL.md` and
  `<config_dir>/skills/*/SKILL.md`. Parse frontmatter (`name`, `description`).
- Inject a one-line catalog into the system message: `skill(name) — description`.
- Tool `skill({ "name": "<name>" })` → returns the SKILL.md body (minus
  frontmatter) as a tool result; mark sticky.
- Files: new `domain/skills/mod.rs`, tool registration, `autonomous.rs` prompt
  assembly.
- Done when: a skill dropped in `.claude/skills/foo/SKILL.md` shows in the
  catalog and its body loads on `skill({"name":"foo"})`.

### 3.2 Sub-agent nesting
- `task` tool spawns a nested `Sentient::autonomous_loop` with its **own**
  message list (fresh system prompt + the task text + read-only tool subset),
  a step cap, and returns only its final text to the parent.
- Files: `domain/tools/dispatch.rs` (or wherever `task` lives), `autonomous.rs`
  (re-entrancy — the loop must be callable with an isolated `Vec<ChatMessage>`).
- Done when: `task("summarize domain/ai/")` runs a bounded child loop and the
  parent sees one clean summary, not the child's tool chatter.

### 3.3 Tier 0 residency (provider-agnostic)
- `ollama_offload.rs` is Ollama-coupled. Generalise `keep_alive` /
  max-loaded-models policy to also cover Lemonade-llamacpp and the ROCmFPX
  server. Low priority; goes with the broader Ollama removal.

---

## 4. Order & rationale

1. **`expand()` handler** — the harness is half-built without it; small,
   self-contained, unlocks aggressive tool compaction.
2. **GBNF verify** — one probe; either it works (keep) or we disable it cleanly.
3. **`/v1/messages`** — makes everything above reach the Claude Code harness,
   which is the headline use case.
4. **Tier 2** — the biggest single win for repeated work; needs the store work.
5. **Semantic compaction** — the long-session fix; depends on the stash pattern
   from step 1.

Skills / sub-agents are orthogonal — do them when they're the bottleneck.

---

## 5. Risks / non-goals

- **Correctness over compression.** A compacted schema that makes the model
  emit a wrong call is worse than no compression. Every step ships with a
  fixture that would catch that regression.
- **No fuzzy/semantic response caching** in Tier 2 v1 — exact key only.
- **`/v1/messages` is not a full Anthropic API** — only what Claude Code sends,
  proven by a committed session capture.
- **No LLM calls in the deterministic paths** (schema compaction, history
  compaction v1) — they must be fast and reversible.
- Do not touch the `kortex/` submodule for any of this; it all lives in
  `src-tauri/src/kortex_*` and `src-tauri/src/domain/`.

---

## 6. Standing instructions for the CLI agent

- `cargo check --lib` and `npx tsc --noEmit` after every step; both green before
  moving on. Baseline: ~37 cargo warnings, one pre-existing `RulesManager` tsc
  error — anything beyond that is yours.
- New Rust modules get unit tests in the same file (`#[cfg(test)]`).
- `cargo` is at `C:\Users\hades\.cargo\bin` (not on PATH — export it).
- One branch per numbered piece (`feat/kortex-expand`, `feat/kortex-tier2`, …),
  off `main`.
- Never fabricate a benchmark. Cache hit-rates and token-saved numbers come from
  `KvCacheStats` / `HarnessReport`, not estimates.
- The running llama-server must be the panel-launched one (parent =
  `vscode-rust-app.exe`); a terminal-launched one the IDE can't manage or kill.
- When done with a piece, update the milestone table in `docs/kortex-cache.md`
  and the state table in §1 here.
