# Kortex Provider-Agnostic Caching — Design


## 1. Goal (and honest scope)

Make Kortex reduce redundant local-inference work for **all** the backends our
users actually run — llama.cpp, Lemonade (AMD, both llamacpp and Ryzen-AI/NPU
recipes), and Ollama — not just raw llama.cpp.

The real problem we are attacking: self-hosted agent loops re-send and re-process
the **same large prompt prefix** (system prompt + tool catalog + conversation
history, often 10–30K tokens) on every turn. On modest hardware the repeated
prefill dominates latency and wastes compute.

**This is not "solving a global AI crisis."** Caching moves the bottleneck; it
does not repeal attention math or VRAM limits. What it credibly delivers: repeated
agent work on self-hosted hardware gets materially cheaper and faster. We state
that plainly so the real win isn't oversold into something reviewers dismiss.

## 2. Current state (what exists today)

`kortex_kvcache` (KDKVC) is a working disk-persistent **KV-slot** cache proxy:

- Fronts an upstream server; intercepts `/v1/chat/completions` and
  `/v1/completions`, passes everything else through.
- On each request: render prefix (`extract_chat_prefix_text`) → `tokenize` →
  SHA-256 over LE-u32 tokens → `longest_prefix` match → on hit, `restore_slot`
  then forward; on completion, `save_slot` + index entry.
- Correctness gates already in place and **tested** (54 tests): model-identity
  isolation (`ModelIdentity::accepts`, `ModelMatchPolicy`), LRU eviction, atomic
  index writes, v1-entry rejection, and (new) the `plan_save_count` save→match
  round-trip invariant.

**Hard constraint:** KDKVC depends on llama.cpp's slot API — `tokenize`,
`/slots/{id}?action=save|restore`, launched with `--slot-save-path`. See
`llamacpp.rs`. Anything without that API cannot use Tier 1.

## 3. Backend capability reality

| Backend | `/tokenize` | slot save/restore | in-server prefix cache | Tier reachable |
|---|---|---|---|---|
| llama.cpp (`llama-server`) | yes | yes (with `--slot-save-path`) | yes (in-mem) | **Tier 1** |
| Lemonade — `llamacpp` recipe | yes* | yes* (if launched with flag) | yes | **Tier 1** (detect) |
| Lemonade — `ryzenai-llm` / NPU | no | no | backend-specific | **Tier 2** only |
| Ollama | no external slot API | **no** | yes (automatic, in-mem, lost on swap/restart) | **Tier 2** only |

`*` = capability must be probed at runtime, not assumed.

Takeaway: we cannot make one mechanism serve everyone. We need **capability
detection** that routes each backend to the best tier it supports.

## 4. Proposed architecture — three tiers, auto-selected

```
                 ┌─────────────────────────────────────────────┐
   IDE client ──▶│              Kortex Cache Proxy               │──▶ upstream
                 │                                               │    (any OpenAI-
                 │  capability probe → tier router               │     compat server)
                 │    ├ Tier 1: KV-slot reuse (KDKVC, existing)  │
                 │    ├ Tier 2: response cache (new, agnostic)   │
                 │    └ Tier 0: residency/keep_alive (existing)  │
                 └─────────────────────────────────────────────┘
```

- **Tier 0 — Residency** (already in `ollama_offload.rs`): tune `keep_alive` and
  `OLLAMA_MAX_LOADED_MODELS` so weights don't get evicted/reloaded between turns.
  Provider-agnostic policy already exists; this tier is "don't make it worse."
- **Tier 1 — KV-slot reuse** (existing KDKVC): the deepest win — skips prefill of
  the matched prefix. Only where slot-save is detected. **No redesign needed**;
  work is *detection + wiring in front of the right backends*.
- **Tier 2 — Response cache** (NEW, the main deliverable): provider-agnostic.
  When an identical prompt prefix **and** identical sampling params recur, serve
  the previously produced completion without calling the model at all. Doesn't
  skip prefill mid-generation like Tier 1 — it eliminates the whole call. This is
  what finally helps Ollama and NPU users.

## 5. Capability detection

At proxy start (and on upstream reconnect), probe the upstream once:

1. `GET /props` (llama.cpp) → if present, derive `ModelIdentity`
   (already done in `kortex_kvcache_start`) **and** mark `tokenize_ok`.
2. `POST /tokenize` with a tiny string → confirms tokenizer endpoint.
3. `POST /slots/{id}?action=save` to a scratch filename, then delete → confirms
   slot API. (Non-destructive probe; erase after.)
4. Classify:
   - `tokenize_ok && slots_ok` → **Tier 1 eligible**.
   - else → **Tier 2 only**.
5. For Lemonade specifically, the collection `/api/v1/models/{id}` recipe field
   (`llamacpp` vs `ryzenai-llm`, see `resolve_lemonade_chat_model` in
   `providers.rs`) is a fast hint before the slot probe.

Persist the detected `CacheTier` on `ProxyState`; the request handler branches on
it. Detection failures degrade safely to Tier 2, never to "broken."

## 6. Tier 2 (response cache) design — the load-bearing new piece

### 6.1 What is cached
A completed model response, keyed by a hash of everything that determines the
output. The value is the full response body (and, for streaming, the ordered SSE
chunks so we can replay them).

### 6.2 Cache key (correctness-critical)
`key = SHA-256( canonical_render(messages) ‖ model_identity ‖ sampling_params )`

Sampling params that MUST be in the key: `model`, `temperature`, `top_p`,
`top_k`, `seed`, `max_tokens`, `stop`, `presence/frequency_penalty`, `tools`
(serialized), `tool_choice`, `response_format`. Miss any and we can serve a
response that violates the caller's request.

Reuse the existing render + SHA scheme (`extract_chat_prefix_text`,
`sha256_tokens_hex`) but **hash the full message array including the final turn**
(unlike Tier 1, which drops the last turn — Tier 2 needs an exact match of the
whole request, not a longest-prefix).

### 6.3 The determinism problem (must not skip)
Serving a cached completion changes behavior unless the call was deterministic.
Rules:
- **Default: only cache when `temperature == 0` OR an explicit `seed` is set.**
  These are the only cases where "same input → same output" holds.
- For `temperature > 0` with no seed, Tier 2 is **disabled by default**. Optional
  opt-in flag `cache_nondeterministic` for users who explicitly want "good enough
  repeats" (e.g. re-runs of the same PoC prompt) — off by default, surfaced in UI.
- Never cache a response that returned an error or was truncated by the stop
  signal (partial output must not become a permanent wrong answer).

### 6.4 Streaming
Store the ordered chunk list; on hit, replay as SSE with the original event
framing so the client can't tell it was cached. Emit a `x-kortex-cache: hit`
header for observability (and so the IDE can badge it).

### 6.5 Invalidation & bounds
- Model-identity gate (reuse `ModelIdentity::accepts`) — a response cached for
  model A is never served for model B. Same danger class as Tier 1.
- LRU on a byte budget (reuse the `store.rs` eviction shape).
- TTL optional (default none — deterministic outputs don't stale, but tool
  results embedded in prompts might; leave TTL configurable).
- Manual clear command (mirror `kortex_kvcache_clear`).

## 7. Config surface (additions to `KvCacheOptions` or a sibling struct)
- `tier: Auto | Kv | Response | Off` (default `Auto`).
- `response_cache_max_bytes`, `response_cache_ttl_secs` (0 = none).
- `cache_nondeterministic: bool` (default false).
- Everything persists TS-side like the current options and is re-sent on start.

## 8. Correctness & safety summary (the "don't corrupt output" list)
1. Model-identity isolation on **both** tiers.
2. Tier 2 keyed on full request incl. all sampling params + tools.
3. Tier 2 default-gated to deterministic calls (`temp==0` or `seed`).
4. Never cache errors / stop-interrupted / truncated responses.
5. Tier 1 unchanged — its round-trip invariant is already tested.
6. Capability mis-detection degrades to a *less* aggressive tier, never a wrong
   one.

## 9. Observability
Extend `KvCacheStats` with `response_hits`, `response_misses`,
`calls_avoided`, `tokens_saved_estimate`. Surface tier + hit-rate in
`KortexInferencePanel` so the user sees "Tier 2 · 34% hit · 1.2M tokens saved."
A visible number is what turns an invisible optimization into trust.

## 10. Milestones

| # | Milestone | Status |
|---|---|---|
| 1 | Capability probe + `CacheTier` on ProxyState; Auto routing | **done** (2026-07-06) |
| 2 | Wire Tier 1 (KDKVC) in front of detected llama.cpp / Lemonade-llamacpp | **done** (2026-09-06) — the Kortex ROCmFPX panel launches llama-server with `--slot-save-path` and starts the `:1537` KDKVC proxy in front; the IDE backend repoints to it |
| 3 | Tier 2 response cache: key, store, non-stream hit/miss + tests | **done (v1)** (2026-09-06) — `kortex_kvcache/response_cache.rs`, in-memory LRU, `KORTEX_TIER2=1`; disk table deferred |
| 4 | Tier 2 streaming replay + `x-kortex-cache` header | **done (v1)** (2026-09-06) — SSE stored raw + replayed byte-for-byte as one chunk (re-timed chunk list deferred) |
| 5 | Determinism gating + `cache_nondeterministic` opt-in | **done** (2026-09-06) — `temp==0`/`seed` gate in `key_for`; `KORTEX_TIER2_NONDETERMINISTIC=1` opt-in |
| 6 | Config surface + KortexInferencePanel stats/tier badge | **partial** (2026-09-06) — Tier 2 + anchor counters folded into `KvCacheStats` by `kortex_kvcache_stats` and shown by `summarizeKvCache`; a dedicated tier badge / env-toggle UI is still todo |
| 7 | Ollama end-to-end validation; Lemonade (both recipes) validation | todo |

**Also landed 2026-09-06 (not in the original plan):** `kortex_harness` —
deterministic tool-schema compression (Hermes-style compact signatures + an
`expand` tool + GBNF grammar), `KORTEX_HARNESS=1` opt-in, wired into the proxy
request path and the native agent loop. See
`docs/kortex-context-engine-plan.md`.

Each milestone lands with unit tests in the `store`/`proxy` test modules (the project's first-real-tests-here precedent).

## 11. Non-goals / honest limitations
- Not modifying Ollama or contributing slot-save upstream (out of scope; huge).
- Tier 2 does not reduce first-time cost — only repeats. Cold prompts still pay
  full price on Ollama/NPU.
- Semantic (fuzzy) caching is explicitly **out** for v1: near-match serving is a
  correctness minefield. Exact-key only.
- No cross-user/shared network cache — local disk only, same trust boundary as
  today.
