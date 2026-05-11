"""
E8 -- DRM x 6 encoders x 6 strategies.

Runs the Deese-Roediger-McDermott templated-identity benchmark across
six text encoders and every consolidation strategy. The DRM corpus is
designed to have tight, high-density clusters so the cap-coverage theorem
can be stress-tested (it is where GAC's centroid branch is expected to
win cleanly).

Encoders:
    bge-large, bge-base, minilm, mpnet, e5-large, nomic

Strategies:
    centroid, medoid, importance_weighted,
    selective_prune{0.5}, selective_prune{0.25}, gac{theta=0.8}

Metric: identity_accuracy, mrr@20, recall@{1,10,100}, coverage@0.80,
and the usual theorem diagnostics (err_cap_08, bound_08).

Expect DRM to be the cleanest demonstration of C1/C3 across encoder families.
"""
from __future__ import annotations

import json
import os
import pathlib
import time

import numpy as np

from gac.metrics import (
    coverage_at_theta,
    identity_retrieval,
    mrr_at_k,
    recall_at_k,
)
from gac.strategies import consolidate
from gac.theory import cluster_spread, d_eff, spectral_bound


MODELS = ["bge-large", "bge-base", "minilm", "mpnet", "e5-large", "nomic"]
STRATEGIES = [
    ("centroid", {}),
    ("medoid", {}),
    ("importance_weighted", {}),
    ("selective_prune", {"keep_ratio": 0.5}),
    ("selective_prune", {"keep_ratio": 0.25}),
    ("selective_prune", {"keep_ratio": 0.10}),
    ("gac", {"theta": 0.6}),
    ("gac", {"theta": 0.7}),
    ("gac", {"theta": 0.8}),
    ("gac", {"theta": 0.9}),
    ("no_consolidation", {}),
]


def _l2norm(X):
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)


def _data_path(model: str) -> pathlib.Path:
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    return root / "drm_templated" / model / "embeddings.npz"


def _err_cap(X_q, q_lbl, store, theta: float) -> float:
    Xn = _l2norm(X_q.astype(np.float32))
    Rn = _l2norm(store.vectors.astype(np.float32))
    S = Xn @ Rn.T
    fails = 0; n = X_q.shape[0]
    for c in np.unique(q_lbl):
        member_idx = np.flatnonzero(q_lbl == c)
        rep_idx = np.flatnonzero(store.cluster_ids == c)
        if rep_idx.size == 0:
            fails += member_idx.size; continue
        sub = S[np.ix_(member_idx, rep_idx)]
        fails += int((sub.max(axis=1) < theta).sum())
    return fails / max(n, 1)


def _per_cluster_stats(X, labels):
    dbars, deffs, sizes = [], [], []
    for c in np.unique(labels):
        if c < 0:
            continue
        sub = X[labels == c]
        if sub.shape[0] < 2:
            continue
        dbars.append(cluster_spread(sub, normalize=False))
        deffs.append(d_eff(sub, method="participation_ratio"))
        sizes.append(sub.shape[0])
    if not dbars:
        return 0.0, 0.0
    w = np.asarray(sizes, dtype=np.float64); w = w / w.sum()
    return float(np.sum(w * np.asarray(dbars))), float(np.sum(w * np.asarray(deffs)))


def _run_cell(cid: int, model: str, strat: str, kw: dict, seed: int = 0) -> dict:
    p = _data_path(model)
    if not p.exists():
        raise FileNotFoundError(f"Missing DRM embeddings {p}")
    data = np.load(p, allow_pickle=True)
    X = data["X"].astype(np.float32)
    labels = data["labels_gold"] if "labels_gold" in data.files else None
    if labels is None:
        raise ValueError(f"DRM corpus expected gold labels at {p}")

    rng = np.random.default_rng(seed)
    keep = np.ones(len(X), dtype=bool); q_all = []
    for lab in np.unique(labels):
        if lab < 0:
            continue
        idx = np.flatnonzero(labels == lab)
        if len(idx) < 3:
            continue
        nq = max(1, int(np.ceil(len(idx) * 0.1)))
        qi = rng.choice(idx, size=nq, replace=False)
        keep[qi] = False; q_all.append(qi)
    q_all = np.concatenate(q_all) if q_all else np.array([], dtype=np.int64)
    X_train = X[keep]; labels_train = labels[keep]
    X_query = X[q_all]; labels_query = labels[q_all]

    if strat == "no_consolidation":
        # Identity ceiling: keep every train vector (selective_prune keep=1.0).
        store = consolidate(X_train, labels_train, strategy="selective_prune", keep_ratio=1.0)
    else:
        store = consolidate(X_train, labels_train, strategy=strat, **kw)

    id_res = identity_retrieval(X_query, labels_query, store, strict=False)
    r_at = recall_at_k(X_query, labels_query, store, ks=(1, 10, 100))
    mrr = mrr_at_k(X_query, labels_query, store, k=20)
    cov08 = coverage_at_theta(X_query, store, theta=0.8)
    err08 = _err_cap(X_query, labels_query, store, theta=0.8)
    dbar_mean, deff_mean = _per_cluster_stats(X_train, labels_train)
    bound08 = spectral_bound(dbar_mean, theta=0.8, d_eff_val=deff_mean)

    return {
        "cell_id": cid, "model": model, "strategy": strat,
        "strategy_kw": json.dumps(kw, sort_keys=True), "seed": seed,
        "n_train": int(X_train.shape[0]), "n_query": int(X_query.shape[0]),
        "n_representatives": int(store.n_representatives),
        "compression": float(store.meta.get("compression", 1.0)),
        "identity_accuracy": id_res["accuracy"],
        "recall@1": r_at["recall@1"], "recall@10": r_at["recall@10"], "recall@100": r_at["recall@100"],
        "mrr@20": mrr, "coverage@0.80": cov08,
        "err_cap_0.80": err08, "bound_0.80": bound08,
        "d_bar_mean": dbar_mean, "d_eff_local_mean": deff_mean,
        "theorem_holds_0.80": (err08 + 1e-6) >= bound08,
    }


def _cells():
    cid = 0
    for model in MODELS:
        for strat, kw in STRATEGIES:
            yield cid, model, strat, kw
            cid += 1


def run_shard(shard_id: int, config: dict, out_dir: str, ckpt_dir: str) -> str:
    n_shards = int(config.get("n_shards", 6))
    seeds = config.get("seeds", [0, 1, 2])
    out_path = pathlib.Path(out_dir) / f"shard_{shard_id:02d}.jsonl"
    ckpt = pathlib.Path(ckpt_dir) / f"shard_{shard_id:02d}.completed.json"
    done: set[str] = set(json.loads(ckpt.read_text()).get("done", [])) if ckpt.exists() else set()
    if not out_path.exists():
        out_path.touch()
    with out_path.open("a") as f:
        for cid, model, strat, kw in _cells():
            if cid % n_shards != shard_id:
                continue
            for seed in seeds:
                key = f"{cid}-{seed}"
                if key in done:
                    continue
                t0 = time.time()
                try:
                    rec = _run_cell(cid, model, strat, kw, seed=seed)
                except FileNotFoundError as e:
                    print(f"[e8 shard {shard_id}] {key} SKIP: {e}")
                    done.add(key); ckpt.write_text(json.dumps({"done": sorted(done)}))
                    continue
                f.write(json.dumps(rec) + "\n"); f.flush(); os.fsync(f.fileno())
                done.add(key); ckpt.write_text(json.dumps({"done": sorted(done)}))
                dt = time.time() - t0
                print(f"[e8 shard {shard_id}] {model}/{strat}/s{seed} "
                      f"-> {dt:.1f}s acc={rec['identity_accuracy']:.3f}")
    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd
    frames = [pd.read_json(p, lines=True) for p in shard_artifacts]
    df = pd.concat(frames, ignore_index=True) if frames else pd.DataFrame()
    out_path = pathlib.Path(out_dir) / "e8_results.parquet"
    df.to_parquet(out_path, index=False)
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    pathlib.Path("./runs/e8_smoke").mkdir(parents=True, exist_ok=True)
    pathlib.Path("./checkpoints/e8_smoke").mkdir(parents=True, exist_ok=True)
    run_shard(0, {"n_shards": 1, "seeds": [0]}, "./runs/e8_smoke", "./checkpoints/e8_smoke")
