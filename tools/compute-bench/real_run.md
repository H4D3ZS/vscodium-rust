# Measuring the real thing (not the model)

`model.py` predicts. `reduce_trace.py` measures — it reads a live trace the
KV-cache proxy writes and prints the same before/after table.

## Get a trace

Set `KORTEX_COMPUTE_TRACE` to a file path before the KV-cache proxy starts
(the IDE inherits the env; or export it in the shell that launches the app):

```
KORTEX_COMPUTE_TRACE=$PWD/.aim/compute-trace.jsonl
```

The proxy's `handle_intercepted` (src-tauri/src/kortex_kvcache/trace.rs +
proxy.rs) then appends one JSON line per intercepted request:

```json
{"ts_unix_ms":..., "request_id":"...", "kind":"chat",
 "tokens_in":28114, "prefix_hit_tokens":27650, "suffix_tokens_processed":464,
 "cache_hit":true}
```

`tokens_in` is the prefix token count (conversation minus the trailing turn —
the part the cache can restore). `prefix_hit_tokens` is what a slot restore
skipped. Nothing is written unless the env var is set.

## Reduce it

```
python tools/compute-bench/reduce_trace.py .aim/compute-trace.jsonl
python tools/compute-bench/reduce_trace.py .aim/compute-trace.jsonl --json
```

- Naive baseline = `sum(tokens_in)` — every turn re-prefills its whole prompt.
- Kortex actual = `sum(tokens_in - prefix_hit_tokens)`.

`--json` emits the same keys as `model.py --json` (`naive_prefill_tokens`,
`kortex_prefill_tokens`, `tokens_saved`, `cost_factor`, ...) so a CI job can
diff the measured receipt against the modelled claim.

## A/B a fixed task

Run one fixed task (e.g. "add a health endpoint + test, fix the build") twice,
each into its own trace file:

1. Stack off: `KORTEX_TIER2=0 KORTEX_HARNESS=0` and the KV cache stopped —
   only a couple of requests land, all `cache_hit:false`, so
   `reduce_trace.py` shows ~1.0x (the baseline is the trace itself).
   For the true naive number, compare against the stack-on run's
   `naive_prefill_tokens` (same `tokens_in`, since the prompts are identical).
2. Stack on: KV cache running, `KORTEX_HARNESS=1`. `cost_factor` is the real
   prefill-compute reduction for that session.

`model.py` is the argument; `reduce_trace.py` on a real trace is the receipt.
