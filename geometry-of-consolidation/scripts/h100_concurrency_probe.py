"""
H100 concurrency probe.

Goal: determine how many H100 containers Modal will actually start in parallel
for this account, so the orchestrator can set MAX_CONCURRENT_H100 safely.

Method: spawn N placeholder GPU tasks that each (a) record their start
timestamp, (b) sleep for a short window, and (c) return.  Compare the
distribution of start times to the ideal "all at once" case.

We escalate N in powers of two -- 1, 2, 4, 8, 16 -- and stop at the first N
where the p90 start-latency exceeds a threshold (meaning we've hit the
account's true concurrency limit).

Decision matrix (§6.2.1 of the spec):
  >= 16 concurrent -> Plan A (full sweep, 2-day wall clock)
   8-15            -> Plan B (stagger E2 into halves)
   4-7             -> Plan C (serial experiments, 4-day wall clock)
   2-3             -> Plan D (drop E6 scale curves from v1 submission)
    1              -> Plan E (single-stream mode, consider RunPod)
    0              -> Plan F (migrate to RunPod / Lambda)

Usage:
    modal run scripts/h100_concurrency_probe.py
"""
from __future__ import annotations

import statistics
import time

import modal

APP_NAME = "gac-h100-probe"

image = modal.Image.debian_slim(python_version="3.11").pip_install("torch>=2.3")
app = modal.App(APP_NAME, image=image)


@app.function(image=image, gpu="H100", timeout=300, cpu=1.0, memory=4096)
def probe_worker(worker_id: int, sleep_s: float = 20.0) -> dict:
    """
    Records when this container actually started, verifies an H100 is present,
    then sleeps for `sleep_s` to hold the slot so later workers have to queue
    only if the account's limit is hit.
    """
    start = time.time()
    gpu_name = "unknown"
    try:
        import torch

        if torch.cuda.is_available():
            gpu_name = torch.cuda.get_device_name(0)
    except Exception as e:  # pragma: no cover
        gpu_name = f"error:{e}"
    time.sleep(sleep_s)
    end = time.time()
    return {
        "worker_id": worker_id,
        "start": start,
        "end": end,
        "gpu": gpu_name,
    }


@app.local_entrypoint()
def probe(max_n: int = 16, sleep_s: float = 20.0):
    """
    Probe escalating concurrency levels.

    At each level N, spawn N workers at time t0, wait for all to complete,
    and report the spread of their start times.
    """
    results = []
    ladder = [n for n in (1, 2, 4, 8, 16, 24, 32) if n <= max_n]

    for n in ladder:
        print(f"\n=== probing N={n} ===")
        t0 = time.time()
        calls = [probe_worker.spawn(i, sleep_s) for i in range(n)]
        outs = [c.get() for c in calls]
        t1 = time.time()

        starts = [o["start"] - t0 for o in outs]
        max_lat = max(starts)
        p50 = statistics.median(starts)
        p90 = sorted(starts)[int(0.9 * len(starts))] if len(starts) > 1 else starts[0]

        gpus = {o["gpu"] for o in outs}
        print(f"  N={n:>3}  wall={t1 - t0:6.1f}s  "
              f"start p50={p50:5.1f}s p90={p90:5.1f}s max={max_lat:5.1f}s")
        print(f"  GPUs seen: {gpus}")
        results.append(
            {
                "N": n,
                "wall_s": t1 - t0,
                "start_p50": p50,
                "start_p90": p90,
                "start_max": max_lat,
                "gpus": sorted(gpus),
            }
        )

        # If p90 start latency > sleep_s, later workers were queued behind earlier
        # ones -- we have definitely hit the limit.
        if p90 > sleep_s * 0.8 and n > 1:
            print(f"  -> hit concurrency wall at N={n}")
            break

    print("\n=== summary ===")
    for r in results:
        print(r)

    # Tier decision
    achieved = max(
        r["N"] for r in results if r["start_p90"] < sleep_s * 0.8
    ) if results else 0
    print(f"\nAchieved concurrency: {achieved}")
    if achieved >= 16:
        plan = "Plan A (full sweep, ~2 days wall-clock)"
    elif achieved >= 8:
        plan = "Plan B (stagger E2 into halves)"
    elif achieved >= 4:
        plan = "Plan C (serial experiments, ~4 days)"
    elif achieved >= 2:
        plan = "Plan D (drop E6 scale curves from v1)"
    elif achieved == 1:
        plan = "Plan E (single-stream; consider RunPod)"
    else:
        plan = "Plan F (no H100s -- migrate to RunPod/Lambda)"
    print(f"Recommended: {plan}")
