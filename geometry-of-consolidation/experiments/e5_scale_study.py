"""
E5 -- Scale study.  How does GAC's identity-retrieval advantage scale from
10K to 10M items?

Design:
  - Corpus: Wikipedia sentences embedded with BGE-large (and MiniLM as faster
    secondary study). Build sizes 10K, 100K, 1M, 10M via streaming the
    `wikimedia/wikipedia` dataset.
  - For each size, cluster with MiniBatchKMeans into N/50 clusters (target
    ~50 members/cluster), then apply every consolidation strategy at 40x
    compression (centroid, medoid @ target, prune keep=0.025, GAC @ kr=0.025).
  - Measure identity_accuracy, recall@10, mrr@20, coverage@0.80 on 5% held-out
    per cluster (capped at 100K queries for tractability).
  - Report timings (build, consolidate, query).

Sharding: each shard does one (size, model) cell. For 10M we check for a
pre-built npz under /vol/data/wiki_scale/<size>_<model>.npz (built separately
by scripts/build_wiki_scale.py on Modal).
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


def _l2norm(X: np.ndarray) -> np.ndarray:
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)


SIZE_LABELS: list[tuple[str, int]] = [
    ("10K", 10_000),
    ("100K", 100_000),
    ("1M", 1_000_000),
    ("10M", 10_000_000),  # gated by whether the artifact exists
]
MODELS = ["bge-large", "minilm"]
TARGET_COMPRESSION = 40.0  # 40x fewer representatives than members
STRATEGIES = [
    ("centroid", {}),
    ("medoid", {}),
    ("selective_prune", {"keep_ratio": 1.0 / TARGET_COMPRESSION}),
    ("gac", {"theta": 0.8}),
    # importance_weighted is provably equivalent to centroid under A3/A4
    # (see Corollary 3); include it as a sanity row.
    ("importance_weighted", {}),
]


def _scale_path(size_key: str, model: str) -> pathlib.Path:
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    return root / "wiki_scale" / f"{size_key}_{model}.npz"


def _cluster(X: np.ndarray, n_clusters: int, seed: int) -> np.ndarray:
    from sklearn.cluster import MiniBatchKMeans

    km = MiniBatchKMeans(
        n_clusters=n_clusters, batch_size=4096, random_state=seed, n_init=3,
        max_iter=30, verbose=0,
    )
    km.fit(X)
    return km.labels_.astype(np.int64)


def _run_cell(size_key: str, size: int, model: str, strat: str, strat_kw: dict, seed: int) -> dict:
    p = _scale_path(size_key, model)
    if not p.exists():
        raise FileNotFoundError(f"Missing scale embeddings {p}")
    data = np.load(p, allow_pickle=True)
    # Scale builds are stored as float16 to halve disk cost; upcast for eval.
    X = data["X"].astype(np.float32)
    # Cluster (ground-truth labels are impractical at 10M; we use clusterer
    # output as the canonical identity grouping).
    n = X.shape[0]
    n_clusters = max(10, n // 50)
    t0 = time.time()
    labels = _cluster(X, n_clusters=n_clusters, seed=seed)
    t_cluster = time.time() - t0

    rng = np.random.default_rng(seed)
    # 5% per-cluster query split, capped at 100K total queries.
    keep = np.ones(n, dtype=bool)
    q_all = []
    for lab in np.unique(labels):
        idx = np.flatnonzero(labels == lab)
        if len(idx) < 4:
            continue
        nq = max(1, int(np.ceil(len(idx) * 0.05)))
        qi = rng.choice(idx, size=nq, replace=False)
        keep[qi] = False
        q_all.append(qi)
    q_all = np.concatenate(q_all) if q_all else np.array([], dtype=np.int64)
    if len(q_all) > 100_000:
        q_all = rng.choice(q_all, size=100_000, replace=False)
    X_train = X[keep]; labels_train = labels[keep]
    X_query = X[q_all]; labels_query = labels[q_all]

    t0 = time.time()
    store = consolidate(X_train, labels_train, strategy=strat, **strat_kw)
    t_consolidate = time.time() - t0

    t0 = time.time()
    id_res = identity_retrieval(X_query, labels_query, store, strict=False)
    r_at = recall_at_k(X_query, labels_query, store, ks=(1, 10, 100))
    mrr = mrr_at_k(X_query, labels_query, store, k=20)
    cov = coverage_at_theta(X_query, store, theta=0.8)
    t_query = time.time() - t0

    return {
        "size": size, "size_key": size_key, "model": model, "strategy": strat,
        "strategy_kw": json.dumps(strat_kw, sort_keys=True),
        "seed": seed,
        "n_train": int(X_train.shape[0]),
        "n_query": int(X_query.shape[0]),
        "n_clusters": int(n_clusters),
        "n_representatives": int(store.n_representatives),
        "compression": float(store.meta.get("compression", 1.0)),
        "identity_accuracy": id_res["accuracy"],
        "recall@1": r_at["recall@1"],
        "recall@10": r_at["recall@10"],
        "recall@100": r_at["recall@100"],
        "mrr@20": mrr,
        "coverage@0.80": cov,
        "t_cluster_s": t_cluster,
        "t_consolidate_s": t_consolidate,
        "t_query_s": t_query,
    }


def _cells():
    cid = 0
    for size_key, size in SIZE_LABELS:
        for model in MODELS:
            for strat, kw in STRATEGIES:
                yield cid, size_key, size, model, strat, kw
                cid += 1


def run_shard(shard_id: int, config: dict, out_dir: str, ckpt_dir: str) -> str:
    n_shards = int(config.get("n_shards", 4))
    seed = int(config.get("seed", 0))
    out_path = pathlib.Path(out_dir) / f"shard_{shard_id:02d}.jsonl"
    ckpt = pathlib.Path(ckpt_dir) / f"shard_{shard_id:02d}.completed.json"
    done: set[int] = set(json.loads(ckpt.read_text()).get("done", [])) if ckpt.exists() else set()
    if not out_path.exists():
        out_path.touch()
    with out_path.open("a") as f:
        for cid, size_key, size, model, strat, kw in _cells():
            if cid % n_shards != shard_id:
                continue
            if cid in done:
                continue
            t0 = time.time()
            try:
                rec = _run_cell(size_key, size, model, strat, kw, seed=seed)
            except FileNotFoundError as e:
                print(f"[e5 shard {shard_id}] {cid} SKIP: {e}")
                done.add(cid); ckpt.write_text(json.dumps({"done": sorted(done)}))
                continue
            f.write(json.dumps(rec) + "\n"); f.flush(); os.fsync(f.fileno())
            done.add(cid); ckpt.write_text(json.dumps({"done": sorted(done)}))
            dt = time.time() - t0
            print(f"[e5 shard {shard_id}] size={size_key} model={model} {strat} "
                  f"-> {dt:.1f}s acc={rec['identity_accuracy']:.3f}")
    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd
    frames = [pd.read_json(p, lines=True) for p in shard_artifacts]
    df = pd.concat(frames, ignore_index=True) if frames else pd.DataFrame()
    out_path = pathlib.Path(out_dir) / "e5_results.parquet"
    df.to_parquet(out_path, index=False)
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    pathlib.Path("./runs/e5_smoke").mkdir(parents=True, exist_ok=True)
    pathlib.Path("./checkpoints/e5_smoke").mkdir(parents=True, exist_ok=True)
    run_shard(0, {"n_shards": 1}, "./runs/e5_smoke", "./checkpoints/e5_smoke")
