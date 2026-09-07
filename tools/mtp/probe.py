#!/usr/bin/env python3
# Vendored verbatim from github.com/sudoingX/qwen38-mtp (Apache-2.0).
# Pair it: run once against a baseline serve, once with `--spec-type draft-mtp`,
# same config otherwise — the paired delta is the honest number.
"""Streaming speed probe for a live llama-server.

Clocks every generated token (reasoning and content deltas) against the
OpenAI-compatible endpoint and prints per-prompt medians. Run it once against
a baseline serve and once with the MTP flag, same config otherwise. The paired
delta is the honest number.

Usage:
    python3 probe.py [server_url]    # default http://127.0.0.1:8080
"""
import json
import sys
import time
import urllib.request
import statistics as st

URL = (sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:8080").rstrip("/") + "/v1/chat/completions"

PROMPTS = [
    "write a python function that merges two sorted lists into one sorted list, with docstring.",
    "explain the difference between mmap and read for loading large files, one paragraph.",
    "write a bash script that watches a directory and prints new files as they appear.",
]
RUNS = 3
MAX_TOKENS = 400


def run(prompt, max_tokens=MAX_TOKENS):
    body = {
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": max_tokens,
        "stream": True,
        "chat_template_kwargs": {"enable_thinking": False},
    }
    req = urllib.request.Request(URL, json.dumps(body).encode(), {"Content-Type": "application/json"})
    t0 = time.time()
    ttft = None
    n = 0
    last = t0
    with urllib.request.urlopen(req, timeout=300) as r:
        for line in r:
            line = line.decode().strip()
            if not line.startswith("data: ") or line == "data: [DONE]":
                continue
            delta = json.loads(line[6:])["choices"][0].get("delta", {})
            if delta.get("content") or delta.get("reasoning_content"):
                now = time.time()
                if ttft is None:
                    ttft = now - t0
                last = now
                n += 1
    span = last - t0 - (ttft or 0)
    return n / span if span > 0 else 0.0


def main():
    run("warmup", 40)
    all_runs = []
    for p in PROMPTS:
        rs = [run(p) for _ in range(RUNS)]
        all_runs += rs
        print(f"{st.median(rs):6.1f} tok/s median | runs: {[round(x, 1) for x in rs]} | {p[:50]}")
    print(f"OVERALL: mean {st.mean(all_runs):.1f} median {st.median(all_runs):.1f}")


if __name__ == "__main__":
    main()
