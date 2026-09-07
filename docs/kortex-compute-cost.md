# Solving compute cost — the three root-node levers

Compression (`docs/kortex-decode-throughput.md`, `kortex_harness`) attacks the
**token** axis: fewer tokens in, fewer tokens out. Necessary, but it doesn't
touch the root inefficiency of LLM inference:

> You run the **full model at full precision on every token**, whether the token
> is hard or trivial. "The" and a subtle borrow-checker fix cost the same FLOPs.

Solving compute cost means breaking that. There are exactly three root-node
levers past compression, and all three are now in the tree (opt-in, tested, no
extra model, no telemetry):

| # | Lever | Idea | Module | Env |
|---|---|---|---|---|
| 1 | **Compute ∝ difficulty** | small model answers by default; escalate to the big one only when it's unsure | `domain/ai/cascade.rs` | `KORTEX_CASCADE=1` |
| 2 | **Eliminate repeat compute** | a semantic hit returns a prior answer with *zero* inference | `domain/ai/semantic_cache.rs` | `KORTEX_SEMCACHE=1` |
| 3 | **Change the arithmetic** | ternary weights turn the multiply-accumulate into add/subtract | `libaim/src/ternary.rs` (`TernaryMatrix`) | (kernel; gated on a ternary model) |

## 1. The cascade router — compute proportional to difficulty

Most tokens (and most whole turns) are easy: the model is >99% confident and a
4B gets them right. Today the 27B reasoner pays full weight-bandwidth for all of
them. The cascade makes the **Operator** (small, Lemonade) answer by default and
escalates to the **Reasoner** (big, ROCmFPX) *only when the Operator is
uncertain*. On the easy majority the big model never runs.

This is **not** speculative decoding. Spec-decode is lossless but the big model
still reads its weights every verify step; the cascade lets the big model **not
run at all** on confident spans. The trade is a *tunable, measured* quality knob.

Two decisions, both pure and unit-tested:

- **Pre-classify** (`start_tier`): a big prompt or a hard-keyword ask
  (`refactor`, `race condition`, `borrow checker`, `prove`, …) skips the
  Operator and starts on the Reasoner — no wasted small-model call.
- **Post-check** (`confidence_from_response` → `decide`): the Operator's answer
  is accepted unless its **mean** token probability is below `p_avg_min`
  (default 0.55), a **single** token dipped below `p_tok_min` (default 0.08 — one
  real fork), it **hedged/refused**, or it came back **empty**. Logprobs are
  read from either the modern (`logprobs.content[].logprob`) or legacy
  (`token_logprobs`) shape; with no logprobs it falls back to the text signal
  and trusts a clean, non-hedged answer (escalating everything would defeat the
  point). Thresholds tune via `KORTEX_CASCADE_PAVG` / `_PTOK`.

`run_cascade` executes it over the two OpenAI-compatible endpoints and returns a
`CascadeReport` (`started_on`, `answered_by`, `escalated`, `reason`,
`operator_confidence`, per-tier token counts) so the win — *how often the big
model was skipped* — is measured, not assumed.

## 2. The semantic response cache — the cheapest token is the one never decoded

IDE traffic repeats hard: "explain this error", "format this", the same file
re-analysed after a one-character edit. `semantic_cache` keys on the *meaning* of
the request and returns a prior answer with **no prefill, no decode, no model
touched**. A hit short-circuits the entire cascade (checked before any inference
in `run_cascade`).

Deliberately **lexical-semantic**, not neural — no running embedder, no extra
model. The request is feature-hashed into a fixed-dim n-gram vector for fast
cosine ranking, and a hit must additionally clear a **token-Jaccard guard**
(default 0.80) on top of a high cosine threshold (default 0.92) so a hash
collision or a merely-similar prompt can never return the wrong answer. Per-model
(never serves model A's answer for model B) and TTL-bounded (default 30 min —
a stale answer to a changed codebase is worse than a recompute). The vector key
is an upgrade seam: drop in real embeddings later and only `embed` changes.

## 3. Ternary-weight GEMV — the arithmetic root (BitNet b1.58)

The actual cost floor is the **multiply**. A MAC (multiply-accumulate) is the
dominant FLOP and the thing GPUs are sized around. Quantise a linear layer's
*weights* to {-1, 0, +1} while activations stay f32, and `y = W·x` has **no
weight×activation multiply**: each weight only decides add, subtract, or skip an
activation, with one per-row scale (absmean) at the end. That is the
unit-cost-of-compute change the whole thesis points at.

`TernaryMatrix` (in the retrieval crate's `ternary.rs`, alongside the
coarse-to-fine `TernaryVec`) implements the BitNet b1.58 representation:
`from_f32_rows` quantises row-major weights (`scale = mean(|w|)`, then
`w → {+1 if w>scale/2, -1 if w<-scale/2, else 0}`); `matvec` is the multiply-free
GEMV via packed pos/neg bitmasks and trailing-zeros bit iteration; `packed_bytes`
is ~16× smaller than the f32 original. Tested for exactness against a
dequantized-ternary reference, and for >0.9 correlation with full-precision
matvec on structured weights.

**Honest gating:** the kernel runs and benchmarks now, but deploying it inside
the LLM's matmuls needs a **ternary-trained** model (BitNet-class or QAT) —
post-hoc rounding a normal model's weights to ternary collapses quality (the same
lesson as 2.5-bit Escha). The kernel is model-agnostic; what's blocked is the
weights. This is the north-star lever: it lands the day a vocab-matched ternary
model does.

## How they compose

```
request
  └─ semantic_cache.lookup ──hit──▶ answer (0 inference)          [#2]
        │miss
        ▼
     start_tier ── hard ─────────▶ Reasoner ──▶ answer            [#1]
        │easy
        ▼
     Operator ── confident ──────▶ answer (big model skipped)     [#1]
        │unsure
        ▼
     Reasoner ──▶ answer
```

Each token that survives to inference is then made cheaper by compression (fewer
tokens) and, eventually, by ternary arithmetic (cheaper per-token FLOPs). The
levers stack: cache removes repeats, cascade removes the big model on easy work,
compression shrinks what's left, ternary changes the cost of the arithmetic
underneath all of it. None needs new hardware.

## Search: ripgrep + tgrep side by side

`grep`/`code_search` gained a second engine in front of ripgrep:
[tgrep](https://github.com/microsoft/tgrep) (Microsoft, trigram-indexed,
client/server) claims up to ~52x rg on large repos by searching a pre-built
index instead of scanning every file per query — real leverage on a monorepo
this size.

Engine order in `ripgrep_search::ripgrep_search`: **tgrep → rg → the pure-Rust
walker.** `domain::indexing::tgrep_search::try_tgrep` runs first when a
`tgrep` binary resolves (`ide_shell::resolve_tgrep_exe`, PATH + `HADES_TGREP_PATH`
override — not bundled, unlike rg, so its absence is the default, normal case):

- **Bounded double attempt.** tgrep's indexed mode auto-connects to an
  already-running `tgrep serve .` daemon the user started themselves (this IDE
  doesn't manage that lifecycle); tried first with an 800ms timeout — a
  no-server connection should fail near-instantly, so the timeout is a safety
  net, not the expected path. Any failure there retries with `--no-index`,
  tgrep's own guaranteed one-shot brute-force mode that needs no server at all.
- **Shared JSON parser.** tgrep advertises rg-compatible `--json` output, so
  both engines parse through the same `parse_rg_json_stream` — one parser, two
  backends.
- **Schema-mismatch guard.** Exit code 0 promising matches but zero parsed
  hits is treated as a parse failure (fall through to rg), never as a false
  empty result — a version drift in tgrep's JSON shape fails safe.
- **Fixed-string queries skip tgrep entirely** (its flag for literal-vs-regex
  matching isn't confirmed) rather than risk a correctness bug for an
  optimization.
- `fixed_string=false` queries take the trigram-fast path when available;
  `HADES_DISABLE_TGREP=1` forces rg-only.

Zero behavior change for the overwhelming majority without tgrep installed —
`resolve_tgrep_exe()` returns `None` and the existing rg → walker chain runs
exactly as before.

### Vendoring it (shipping it with the IDE)

tgrep is MIT-licensed (Microsoft) — permissively shippable, same posture as
bundled ripgrep. `scripts/fetch-tgrep.ts` downloads the pinned release
(`v1.0.4`, exact asset names/triples confirmed against the real GitHub
release) into `src-tauri/bundles/tgrep/` — the same "fetched at build time,
gitignored, not committed" pattern as `fetch-ripgrep.ts`. `ide_shell.rs`
mirrors every ripgrep bundling primitive for tgrep: `portable_tgrep_root`,
`repo_bundles_tgrep`, `ensure_tgrep_installed` (copies the bundle to
`%LOCALAPPDATA%\HADES\tgrep` on first launch, wired into the same startup
task as PortableGit/ripgrep in `lib.rs`), and the `ide_ensure_tgrep` Tauri
command. `resolve_tgrep_exe` checks, in order: `HADES_TGREP_PATH` override →
the installed-to-HADES-home copy → the installer's own bundle → PATH.

Verified against the **real vendored binary**, not just the documented
interface: fetched `v1.0.4` for real, confirmed its `--json` output is
byte-identical in shape to what `parse_rg_json_stream` expects (`type`,
`data.path.text`, `data.line_number`, `data.lines.text`), confirmed exit codes
0/1 for match/no-match against real searches, and ran an ignored end-to-end
test (`real_tgrep_binary_end_to_end`) that finds a real symbol in this actual
codebase through the full `ripgrep_search` → `try_tgrep` → real-process
pipeline. Run it yourself after fetching: `cargo test --lib -- --ignored
real_tgrep_binary_end_to_end`.

## Beyond compute — hallucination grounding

Compute cost is one of AI's structural problems; **hallucination** is another,
and the IDE is the right place to attack it because a coding agent's dominant
hallucination is a *reference that doesn't exist* — a file, a `path:line`, a
function it never saw — which is **deterministically checkable** against the
index and filesystem. `domain/ai/grounding.rs` (`KORTEX_GROUNDING=1`):

- `extract_claims` pulls checkable references out of an answer: file paths,
  `path:line` citations, and `func()` symbol calls (conservative — a bare
  `foo.rs` in prose isn't flagged unless backtick-wrapped or path-qualified, so
  "Node.js" and URLs don't trip it).
- `verify` checks each against a [`GroundTruth`] oracle
  (`WorkspaceGroundTruth` = filesystem + optional index symbol set): a missing
  file, a citation past a real file's end, or an unknown symbol becomes an
  `Ungrounded` verdict; anything the oracle can't decide is `Unverifiable`
  (reported, never counted as a hallucination).
- `annotate` appends a compact "⚠ Unverified references" block so the user
  *sees* which claims aren't grounded, instead of trusting a confident wrong one.
- `hallucination_risk` fuses the reference risk (hard signal) with the cascade's
  token confidence (`cascade::Confidence`, soft signal) into one score — the two
  problems' solutions compose: grounding catches wrong *names*, logprobs catch
  shaky *generation*.

Honest limit: a deterministic checker catches fabricated references, not a
fluent-but-wrong *explanation* of code that does exist. That residue is what the
confidence gate and abstention (escalate / say "I'm not sure") are for.

## Live wiring — from modules to behavior

Every lever above started as a tested, standalone module. All are now wired into
real call sites, each still behind its own env flag (default off, zero behavior
change until opted in):

- **`reliability.rs`** — the facade: `ground → annotate → abstain` in one call.
  Wired into `Sentient::single_shot_completion`'s return (`engine/streaming.rs`):
  every non-streaming completion is checked against the real workspace
  (`WorkspaceGroundTruth` rooted at `ai_tools.get_root_path()`) before it reaches
  the caller.
- **Semantic cache** — wired into `apex_orchestrator::openai_chat`: a lookup
  before the HTTP call, a store after a successful one. A repeat APEX query
  returns with zero inference.
- **`verify_implementation`** (`workflow_tools.rs`) — was a stub that always
  returned `"verified": true`; now, behind `KORTEX_VERIFY=1`, it runs
  `verify::detect_runner` (auto-detects `cargo`/`npm`/`pytest` from the
  workspace) and reports a *real* pass/fail with diagnostics, never an assumed
  one.
- **`authorization` + `provenance`** — wired into the tool dispatcher
  (`dispatch::call_tool`): every call is classified and gated (`KORTEX_AUTHZ`)
  before it runs; every external-fetch tool's result (`web_fetch`,
  `browser_subagent`, `perplexity_*`, …) is provenance-fenced (`KORTEX_PROVENANCE`)
  before it re-enters the model's context.
- **`state_ledger`** — exposed as a tool, `task_state` (actions: `goal`,
  `decide`, `fact`, `ask`, `resolve`, `status`), backed by a per-workspace
  process-global registry (`state_ledger::with_ledger`). The model can record
  its own decisions and re-ground itself against drift on a long task.

## Local by default

Once wired, the natural next question was: off by default forever, or on where
it actually matters? The answer implemented here — each lever defaults on
**exactly where it's already scoped to local inference**, and off (or
provider-agnostic) everywhere else, with an explicit env value always winning
over the default in either direction (`env_flag::on`, tri-state: `1/true/on`
forces on, `0/false/off` forces off, unset falls to the default):

| Lever | Default | Why |
|---|---|---|
| `kortex_harness` (schema compaction, tool-output crush, steer) | **on** | its only call site is gated on `is_native_api` (Antigravity/Lemonade) already — never touches a cloud request |
| Semantic cache | **on** | its call site (`apex_orchestrator::openai_chat`) serves local/Lemonade-backed engines |
| Grounding + abstain | **on for a local answer, off for a cloud one** | threaded from the caller's own `is_local` check — fact-checking a small local model against the workspace is the target failure mode; a frontier cloud model's answer isn't second-guessed by a heuristic unless asked |
| Authorization + provenance | **on regardless of backend** | tool-safety gates, not model-answer changes — an untrusted web page or a destructive command is equally dangerous whichever model is driving |
| Verification | **opt-in, deliberately** | a `cargo build`/`test` on a real workspace can run minutes; defaulting it on risks the agent stalling on every `verify_implementation` call. `CommandCheckRunner` now bounds every check with a timeout (`KORTEX_VERIFY_TIMEOUT_SECS`, default 120s) regardless, so once you turn it on it can never hang the loop |
| Cascade | on (forward-compatible) | not yet wired to a live routing decision, so this changes nothing today |

A worked example of the composition: with a local model and nothing set,
grounding **and** abstain both default on — so a single fabricated file
reference pushes the fused risk (`hallucination_risk`) past the abstain
threshold, and the answer isn't just annotated, it's fully withheld in favor of
an honest "not confident enough". Two independently-scoped defaults compounding
correctly is the intended shape of this cluster, not a coincidence.
