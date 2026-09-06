# Kortex + Lemonade — the local AMD inference stack

Everything here runs on one consumer AMD GPU (tested: RX 9060 XT, 16 GB,
~322 GB/s). No cloud, no NVIDIA, no per-token cost.

## Two models, two jobs

| | Model | Server | Job |
|---|---|---|---|
| **Reasoner** | Escha / Qwen3.5-30B-A3B class | **ROCmFPX** `llama-server` on `:8081` | Long-context planning, code reasoning, the agent's main loop |
| **Operator** | Qwen3.5-4B (Q5) | **Lemonade** on `:13305` | Tool-call formatting, structured/JSON output, fast APEX sweeps, sub-agents |

They are separate processes — one `llama-server` holds one model — so the
big model keeps its VRAM + KV budget while the small model does the
high-frequency structured work it's actually good at. The 4B streams from
RAM on CPU/partial-offload if VRAM is tight; a 3 GB weight read per short
tool turn is not a bottleneck.

**Who runs on the Operator** (resolved in `gpu_offload.rs` —
`operator_model()` / `operator_url()`, overridable via
`KORTEX_OPERATOR_MODEL` / `KORTEX_OPERATOR_URL`):

- **Sub-agents** — the `task` tool spawns a bounded child loop; that whole
  loop runs on the Operator against Lemonade directly (not the reasoner's
  URL, even when it's been repointed at the Kortex proxy).
- **APEX engines** on Lite/Mid tier — the whole specialist bank collapses
  onto the one Operator model.
- **The reasoner** keeps the main agent loop and anything the user drives
  from the chat box.

**Lemonade** (AMD's own OpenAI-compatible llama.cpp server) is the backbone
for the Operator: it serves the GGUF over `/v1/chat/completions`, which is
the one wire protocol the whole stack speaks.

## Kortex sits in front — hand in hand with Lemonade

Kortex is a set of opt-in, in-process proxies that make the *same* requests
cost less, without changing a single output token:

```
IDE / agent ─▶ Kortex KV-slot cache (:1537) ─▶ Lemonade (:13305)  ── Operator (4B)
            └▶ Kortex KV-slot cache (:1537) ─▶ ROCmFPX  (:8081)   ── Reasoner (30–35B)
                     │
                     ├─ AIM retrieval proxy (:1536)  — inject only the relevant .aim slices
                     ├─ tool-schema compression       — 6k tokens of tool JSON → ~1.8k
                     └─ compute trace (KORTEX_COMPUTE_TRACE) — measured prefill savings
```

- **KV-slot cache (KDKVC)** — a coding agent resends a ~28 k-token system
  prompt every turn. The proxy longest-prefix-matches it against llama.cpp
  slot state on disk and skips the re-prefill. `reduce_trace.py` on a live
  trace prints the measured factor (modelled ~5× fewer prefill tokens over
  a 30-turn session).
- **AIM retrieval proxy** builds a dense `.aim` catalog of the workspace
  (via Lemonade's embedding endpoint) and prepends only the chunks that
  clear a relevance gate — smaller prompts, same answers.
- **Speculative decoding** (ROCmFPX fork) — `ngram-*` guessers (no model,
  no VRAM) + the model's MTP head. The full model verifies every drafted
  token, so output is identical; the panel shows the live acceptance rate.
- **HIP graph capture** (`-DGGML_HIP_GRAPHS=ON`) — one graph launch per
  decode step instead of hundreds of kernel launches.

Every piece is off by default and flips on from the *Kortex Services* panel.
Turn them all off and you have plain Lemonade + plain llama.cpp; turn them
on and the same session does measurably less work.

## APEX routing

APEX (the security-specialist engine bank) resolves its model per RAM tier
(`gpu_offload.rs`): Lite/Mid machines collapse every engine onto the single
resident Operator (`qwen3.5:4b`); Full tier keeps a per-specialist split and
sends the 27B `exploit` model to Lemonade directly. Both paths use one HTTP
shape — `openai_chat()` → `/v1/chat/completions` on `inference_url` (the
Kortex proxy when it's up, Lemonade otherwise). The old Ollama-native
`/api/generate` call is gone.

## The pitch, in one line

A consumer AMD box runs a 30-B-class agent locally, and the Kortex layer
cuts its per-turn compute several-fold — measured, reproducible, output
unchanged. If a 16 GB card can do this, the pressure to stop re-prefilling
stable prompts applies to everyone.
