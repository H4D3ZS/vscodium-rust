#!/usr/bin/env python3
"""A cost model for an agentic turn loop — naive vs. the kortex context stack.

This is a *model*, not a live run. It counts the tokens a local backend must
prefill on each turn under two policies, using the sizes an agentic IDE
actually assembles. It exists to make one claim measurable and reproducible:
> On consumer hardware, the compute cost of an agentic loop is dominated by
> re-prefilling a stable prompt every turn. Keep the prefix byte-stable and
> that cost collapses.

The rules encoded here ARE the kortex design (see src-tauri/src/domain/ai/
context_budget.rs and .../engine/prefix_cache.rs):

  * The prompt is [FROZEN prefix] + [VOLATILE tail].
    FROZEN = system rules + tool schemas + repo map — byte-identical across
    turns. VOLATILE = env line + memory gist + conversation + current turn.
  * llama.cpp reuses the KV cache for the LONGEST COMMON PREFIX with the
    previous prompt. So turn N only prefills [what changed] onward.
  * If ONE volatile byte lands in the frozen block (a clock time, a live
    file count, a uuid) the common prefix ends there and the whole prompt
    re-prefills. That's the ~78x regression prefix_cache.rs documents.
  * The context budget caps the volatile tail (old tool results shed first),
    so history can't grow without bound.

Run:  python model.py                 # default 30-turn agentic session
      python model.py --turns 60 --prefill-tps 120 --json
"""
from __future__ import annotations
import argparse
import json
import sys
from dataclasses import dataclass


@dataclass
class Sizes:
    """Tokens. Defaults ≈ what this IDE assembles for a 35B agent turn."""
    system_frozen: int = 22_000      # rules + repo map + tool prose (stable)
    tool_schemas: int = 6_000        # JSON tool defs (stable per session)
    tool_schemas_compressed: int = 1_800   # with KORTEX_HARNESS
    env_line: int = 120              # OS + root + date (stable if no clock)
    memory_gist: int = 1_500         # recent-decisions summary (grows slowly)
    per_user_turn: int = 350         # a user message
    per_assistant_turn: int = 250    # a model reply
    per_tool_result: int = 1_800     # a raw tool result before budget trim
    retrieved_context: int = 900     # .aim chunks injected when retrieval is on


def simulate(n_turns: int, s: Sizes, *,
             harness: bool, retrieval: bool, budget_tokens: int,
             cache_busted_prefix: bool) -> list[dict]:
    """Return per-turn {naive_prefill, kortex_prefill} token counts."""
    tools = s.tool_schemas_compressed if harness else s.tool_schemas
    frozen = s.system_frozen + tools
    # a cache-buster (live file count / clock) sits inside `frozen`
    stable_prefix = 0 if cache_busted_prefix else frozen

    history: list[int] = []   # token size of each prior message, oldest first
    rows = []
    for turn in range(1, n_turns + 1):
        # volatile tail assembled this turn
        volatile = s.env_line + s.memory_gist
        if retrieval:
            volatile += s.retrieved_context
        volatile += sum(history)
        volatile += s.per_user_turn   # the current turn

        # context budget: shed oldest tool results until the tail fits
        tail_cap = budget_tokens
        trimmed = list(history)
        while sum(trimmed) + s.env_line + s.memory_gist + s.per_user_turn > tail_cap and len(trimmed) > 2:
            # drop the oldest (tool results are oldest-and-biggest in practice)
            trimmed.pop(0)
        kortex_tail = (s.env_line + s.memory_gist
                       + (s.retrieved_context if retrieval else 0)
                       + sum(trimmed) + s.per_user_turn)

        naive_prefill = frozen + volatile           # re-prefill everything
        if turn == 1:
            kortex_prefill = frozen + kortex_tail   # cold: pay it once
        else:
            # reuse the stable prefix; only the tail (which changed) is new
            kortex_prefill = (frozen - stable_prefix) + kortex_tail

        rows.append({
            "turn": turn,
            "naive_prefill": naive_prefill,
            "kortex_prefill": kortex_prefill,
        })

        # advance the conversation
        history.append(s.per_user_turn)
        history.append(s.per_assistant_turn)
        history.append(s.per_tool_result)
    return rows


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--turns", type=int, default=30)
    ap.add_argument("--prefill-tps", type=float, default=120.0,
                    help="prompt tok/s (ROCmFPX 35B on RX 9060 XT ≈ 120)")
    ap.add_argument("--wh-per-Mtok", type=float, default=25.0,
                    help="watt-hours per million prefill tokens (rough)")
    ap.add_argument("--no-harness", action="store_true")
    ap.add_argument("--no-retrieval", action="store_true")
    ap.add_argument("--budget", type=int, default=12_000, help="volatile-tail token cap")
    ap.add_argument("--broken-prefix", action="store_true",
                    help="model the bug: a live file count in the frozen block")
    ap.add_argument("--json", action="store_true")
    a = ap.parse_args()

    s = Sizes()
    rows = simulate(a.turns, s,
                    harness=not a.no_harness,
                    retrieval=not a.no_retrieval,
                    budget_tokens=a.budget,
                    cache_busted_prefix=a.broken_prefix)

    naive_tot = sum(r["naive_prefill"] for r in rows)
    kx_tot = sum(r["kortex_prefill"] for r in rows)
    saved = naive_tot - kx_tot
    factor = naive_tot / kx_tot if kx_tot else float("inf")

    def secs(tok): return tok / a.prefill_tps
    def wh(tok): return tok / 1e6 * a.wh_per_Mtok

    if a.json:
        print(json.dumps({
            "turns": a.turns,
            "naive_prefill_tokens": naive_tot,
            "kortex_prefill_tokens": kx_tot,
            "tokens_saved": saved,
            "cost_factor": round(factor, 1),
            "naive_prefill_seconds": round(secs(naive_tot), 1),
            "kortex_prefill_seconds": round(secs(kx_tot), 1),
            "naive_wh": round(wh(naive_tot), 2),
            "kortex_wh": round(wh(kx_tot), 2),
            "config": {"harness": not a.no_harness, "retrieval": not a.no_retrieval,
                       "budget": a.budget, "broken_prefix": a.broken_prefix},
        }, indent=2))
        return 0

    print(f"agentic session: {a.turns} turns  |  prefill {a.prefill_tps:.0f} tok/s"
          f"  |  harness={not a.no_harness} retrieval={not a.no_retrieval}"
          f" budget={a.budget}{' BROKEN-PREFIX' if a.broken_prefix else ''}\n")
    print(f"{'turn':>4} {'naive prefill':>14} {'kortex prefill':>15} {'ratio':>7}")
    print("-" * 44)
    for r in rows:
        ratio = r["naive_prefill"] / max(r["kortex_prefill"], 1)
        mark = "  <- turn 1 cold" if r["turn"] == 1 else ""
        print(f"{r['turn']:>4} {r['naive_prefill']:>14,} {r['kortex_prefill']:>15,} {ratio:>6.1f}x{mark}")
    print("-" * 44)
    print(f"{'TOTAL':>4} {naive_tot:>14,} {kx_tot:>15,} {factor:>6.1f}x\n")
    print(f"  prefill wall time : {secs(naive_tot):8.1f} s  ->  {secs(kx_tot):7.1f} s"
          f"   ({secs(saved):.0f} s saved)")
    print(f"  prefill energy    : {wh(naive_tot):8.2f} Wh ->  {wh(kx_tot):7.2f} Wh"
          f"   ({wh(saved):.2f} Wh saved)")
    print(f"\n  {factor:.0f}x less prefill compute for the same session, same model,")
    print(f"  same outputs -- just by keeping the prompt prefix byte-stable and")
    print(f"  bounding the tail. No new hardware, no smaller model.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
