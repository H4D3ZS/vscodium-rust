#!/usr/bin/env python3
"""Reduce a live KV-cache compute trace to the naive-vs-kortex prefill table.

`model.py` predicts. This reads what actually happened: the JSON lines the
KV-cache proxy writes to `KORTEX_COMPUTE_TRACE` (one per intercepted request,
see src-tauri/src/kortex_kvcache/trace.rs), and folds them into the same
before/after shape `model.py --json` prints, so the two can be diffed.

  naive baseline = sum(tokens_in)                       # re-prefill every turn
  kortex actual  = sum(tokens_in - prefix_hit_tokens)   # restore what matched

Run:  python reduce_trace.py .aim/compute-trace.jsonl
      python reduce_trace.py .aim/compute-trace.jsonl --prefill-tps 120 --json
"""
from __future__ import annotations
import argparse
import json
import sys


def load(path: str) -> list[dict]:
    rows = []
    with open(path, "r", encoding="utf-8") as f:
        for n, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            try:
                r = json.loads(line)
            except json.JSONDecodeError:
                print(f"warn: {path}:{n}: skipping unparseable line", file=sys.stderr)
                continue
            if "tokens_in" not in r:
                continue
            r.setdefault("prefix_hit_tokens", 0)
            r.setdefault("request_id", f"line-{n}")
            r.setdefault("cache_hit", r["prefix_hit_tokens"] > 0)
            rows.append(r)
    return rows


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("trace", help="path to compute-trace.jsonl")
    ap.add_argument("--prefill-tps", type=float, default=120.0,
                    help="prompt tok/s for the wall-time column (ROCmFPX 35B ~= 120)")
    ap.add_argument("--wh-per-Mtok", type=float, default=25.0,
                    help="watt-hours per million prefill tokens (rough)")
    ap.add_argument("--json", action="store_true")
    a = ap.parse_args()

    rows = load(a.trace)
    if not rows:
        print(f"no usable records in {a.trace}", file=sys.stderr)
        return 1

    naive_tot = sum(r["tokens_in"] for r in rows)
    kx_tot = sum(max(r["tokens_in"] - r["prefix_hit_tokens"], 0) for r in rows)
    saved = naive_tot - kx_tot
    factor = naive_tot / kx_tot if kx_tot else float("inf")
    hits = sum(1 for r in rows if r.get("cache_hit"))

    def secs(tok: float) -> float: return tok / a.prefill_tps
    def wh(tok: float) -> float: return tok / 1e6 * a.wh_per_Mtok

    if a.json:
        print(json.dumps({
            "turns": len(rows),
            "cache_hits": hits,
            "naive_prefill_tokens": naive_tot,
            "kortex_prefill_tokens": kx_tot,
            "tokens_saved": saved,
            "cost_factor": round(factor, 1),
            "naive_prefill_seconds": round(secs(naive_tot), 1),
            "kortex_prefill_seconds": round(secs(kx_tot), 1),
            "naive_wh": round(wh(naive_tot), 2),
            "kortex_wh": round(wh(kx_tot), 2),
            "source": a.trace,
        }, indent=2))
        return 0

    print(f"live trace: {a.trace}  |  {len(rows)} requests, {hits} cache hits"
          f"  |  prefill {a.prefill_tps:.0f} tok/s\n")
    print(f"{'req':>4} {'tokens_in':>10} {'restored':>9} {'prefilled':>10} {'ratio':>7}")
    print("-" * 46)
    for i, r in enumerate(rows, 1):
        prefilled = max(r["tokens_in"] - r["prefix_hit_tokens"], 0)
        ratio = r["tokens_in"] / max(prefilled, 1)
        print(f"{i:>4} {r['tokens_in']:>10,} {r['prefix_hit_tokens']:>9,} {prefilled:>10,} {ratio:>6.1f}x")
    print("-" * 46)
    print(f"{'TOT':>4} {naive_tot:>10,} {saved:>9,} {kx_tot:>10,} {factor:>6.1f}x\n")
    print(f"  prefill wall time : {secs(naive_tot):8.1f} s  ->  {secs(kx_tot):7.1f} s"
          f"   ({secs(saved):.0f} s saved)")
    print(f"  prefill energy    : {wh(naive_tot):8.2f} Wh ->  {wh(kx_tot):7.2f} Wh"
          f"   ({wh(saved):.2f} Wh saved)")
    print(f"\n  {factor:.1f}x less prefill compute over this session -- measured, not modelled.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
