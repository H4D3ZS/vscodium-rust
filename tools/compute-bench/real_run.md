# Measuring the real thing (not the model)

`model.py` predicts. To *measure* a live session you need per-turn prefill
token counts and wall time from an actual run. The plumbing already exists;
it just isn't aggregated into a before/after.

## What's already recorded

- **`src/kortex/throughput.ts`** — `ThroughputSample { input_tokens,
  output_tokens, prefill_ms, tokens_skipped, backend, cache_hit }`. The
  agent-loop / proxy path pushes one per turn. `summarize()` already
  computes `avg_prefill_tps` and `total_tokens_skipped`.
- **`src-tauri/src/kortex_kvcache/types.rs::RoutingTrace`** — the KV-cache
  proxy's per-request record: `tokens_in`, `prefix_hit_tokens`,
  `suffix_tokens_processed`, `tokens_out`, `eta`. `prefix_hit_tokens` is
  exactly "tokens NOT re-prefilled thanks to the cache".
- **`ai-context-budget`** event (from `context_budget::FitReport`) —
  `before_tokens`, `after_tokens`, `dropped_messages` per turn.

## The one missing piece

A sink that, for a tagged session, writes each turn's
`{turn, tokens_in, prefix_hit_tokens, suffix_tokens_processed, prefill_ms,
tokens_out}` to `<workspace>/.aim/compute-trace.jsonl`, then a script that
reduces it to the same table `model.py` prints — but from real bytes.

Naive baseline for the same session = `sum(tokens_in)` (every turn would
re-prefill its whole prompt). Kortex actual =
`sum(tokens_in - prefix_hit_tokens)`.

### Sketch

1. Rust: in the kvcache proxy's `handle_intercepted`, after a slot
   restore, append the `RoutingTrace` as one JSON line to
   `KORTEX_COMPUTE_TRACE` (env path) when set. ~15 lines.
2. `python reduce_trace.py .aim/compute-trace.jsonl` → the table +
   `--json` for CI, identical shape to `model.py --json` so they can be
   diffed.
3. Run a fixed task (e.g. "add a health endpoint + test, fix the build")
   twice — once with `KORTEX_TIER2=0 KORTEX_HARNESS=0` and the KV cache
   off, once with the stack on — and compare the two traces.

Until that lands, `model.py` is the argument and the numbers above are the
claim; a live trace turns the claim into a receipt.
