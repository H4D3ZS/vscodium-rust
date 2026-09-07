#!/usr/bin/env python3
"""Generate bench.bin, run a NumPy reference, invoke ./knn, compare.

Correctness for an *exact* brute-force kernel means recall@k == 1.0 against
the reference. The reference is also the speed baseline (NumPy BLAS); the
Rust `reference.rs` gives the more honest single-thread baseline.

    python bench.py --n 50000 --dim 1024 --k 8 --queries 200
"""
import argparse
import struct
import subprocess
import sys
import time
from pathlib import Path

import numpy as np

MAGIC = 0x4B4E4E31
HERE = Path(__file__).parent


def unit(x):
    return x / np.linalg.norm(x, axis=-1, keepdims=True)


def write_bin(path, corpus, queries, k):
    n, dim = corpus.shape
    nq = queries.shape[0]
    with open(path, "wb") as f:
        f.write(struct.pack("<6I", MAGIC, n, dim, nq, k, 0))
        f.write(corpus.astype("<f4").tobytes())
        f.write(queries.astype("<f4").tobytes())
        f.write(np.zeros(nq * k, dtype="<i8").tobytes())   # out_ids
        f.write(np.zeros(nq * k, dtype="<f4").tobytes())   # out_scores
    return n, dim, nq


def read_results(path, n, dim, nq, k):
    with open(path, "rb") as f:
        blob = f.read()
    off = 24 + (n * dim + nq * dim) * 4
    ids = np.frombuffer(blob, dtype="<i8", count=nq * k, offset=off).reshape(nq, k)
    off += nq * k * 8
    sc = np.frombuffer(blob, dtype="<f4", count=nq * k, offset=off).reshape(nq, k)
    return ids, sc


def reference(corpus, queries, k):
    t0 = time.perf_counter()
    sims = queries @ corpus.T                      # (nq, n)
    idx = np.argpartition(-sims, k - 1, axis=1)[:, :k]
    row = np.arange(queries.shape[0])[:, None]
    order = np.argsort(-sims[row, idx], axis=1)
    idx = idx[row, order]
    dt = time.perf_counter() - t0
    return idx, sims[row, idx], dt


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--n", type=int, default=50_000)
    ap.add_argument("--dim", type=int, default=1024)
    ap.add_argument("--k", type=int, default=8)
    ap.add_argument("--queries", type=int, default=200)
    ap.add_argument("--seed", type=int, default=1)
    a = ap.parse_args()

    rng = np.random.default_rng(a.seed)
    corpus = unit(rng.standard_normal((a.n, a.dim)).astype("f4"))
    queries = unit(rng.standard_normal((a.queries, a.dim)).astype("f4"))

    binpath = HERE / "bench.bin"
    n, dim, nq = write_bin(binpath, corpus, queries, a.k)

    ref_ids, _ref_sc, ref_dt = reference(corpus, queries, a.k)
    print(f"reference (NumPy/BLAS): {ref_dt * 1e6 / nq:8.1f} us/query")

    knn = HERE / ("knn.exe" if sys.platform == "win32" else "knn")
    if not knn.exists():
        print(f"\n{knn.name} not built — run:  mojo build knn.mojo -o {knn.name}")
        return 1

    out = subprocess.run([str(knn), str(binpath)], capture_output=True, text=True)
    print(out.stdout.strip())
    if out.returncode != 0:
        print(out.stderr, file=sys.stderr)
        return out.returncode

    got_ids, _got_sc = read_results(binpath, n, dim, nq, a.k)

    # recall@k — exact kernel must match the reference set per query
    hit = sum(len(set(g.tolist()) & set(r.tolist())) for g, r in zip(got_ids, ref_ids))
    recall = hit / (nq * a.k)
    print(f"\nrecall@{a.k}: {recall:.4f}   {'PASS' if recall > 0.999 else 'FAIL — kernel is not exact'}")
    return 0 if recall > 0.999 else 2


if __name__ == "__main__":
    raise SystemExit(main())
