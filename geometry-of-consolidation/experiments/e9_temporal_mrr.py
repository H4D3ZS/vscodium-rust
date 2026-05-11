"""
E9 -- Temporal pipeline with rank-based metrics.

This block re-runs the RESEARCH_REPORT's temporal consolidation pipeline (Exp 6
there) with ranking metrics in place of the thresholded cosine > 0.8 metric.
The prediction is:

  * Under cosine > 0.8, every strategy looks "collapsed" once store size
    grows, because θ = 0.8 is below the mean cap coverage for most corpora.
  * Under MRR@20 and Recall@10, GAC + selective-prune recover to viable
    territory (0.4-0.7), while centroid and medoid still show identity
    collapse. This exposes that the thresholded metric was an artefact of
    threshold choice, not of the consolidation operator.

Design
------
We emulate a "temporal memory" by admitting embeddings one batch at a time,
consolidating at each step, and measuring how well the *older* items
continue to be identity-retrievable from the compressed store.

For each (corpus, model) cell:
    1. Split X randomly into N_EPOCHS batches of equal size.
    2. At each epoch:
        a. Cluster the running pool with k-means (k = pool / avg_cluster_size).
        b. Apply strategy to produce the compressed store.
        c. For every previously-admitted item, query the store with the raw
           embedding, measure rank of the true cluster.
    3. Record MRR@20, Recall@{10,100}, identity accuracy, cov@0.8 per epoch.

Interface
---------
run_shard(shard_id, config, out_dir, ckpt_dir) -> str
reduce(shard_artifacts, config, out_dir) -> str
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


# ---------------------------------------------------------------------------
# grid
# ---------------------------------------------------------------------------


CORPUS_MODEL_PAIRS = [
    ("wikipedia_sections", "bge-large"),
    ("wikipedia_sections", "minilm"),
    ("ms_marco", "bge-large"),
    ("nq_questions", "bge-large"),
    ("hotpot_qa", "bge-large"),
    ("arxiv_titles", "bge-large"),
    ("drm_templated", "bge-large"),
]

STRATEGIES = [
    ("centroid", {}),
    ("medoid", {}),
    ("selective_prune", {"keep_ratio": 0.5}),
    ("selective_prune", {"keep_ratio": 0.25}),
    ("gac", {"theta": 0.8}),
]

N_EPOCHS = 5  # batches of admitted items
AVG_CLUSTER_SIZE = 20
SEEDS = [0, 1, 2]


def _cells():
    cid = 0
    for corpus, model in CORPUS_MODEL_PAIRS:
        for strat_name, strat_kw in STRATEGIES:
            for seed in SEEDS:
                yield cid, corpus, model, strat_name, strat_kw, seed
                cid += 1


def _data_path(corpus: str, model: str) -> pathlib.Path:
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    return root / corpus / model / "embeddings.npz"


def _load(corpus: str, model: str) -> dict:
    p = _data_path(corpus, model)
    if not p.exists():
        raise FileNotFoundError(p)
    return dict(np.load(p, allow_pickle=True))


def _run_cell(
    cell_id: int,
    corpus: str,
    model: str,
    strat: str,
    strat_kw: dict,
    seed: int,
) -> list[dict]:
    data = _load(corpus, model)
    X = data["X"].astype(np.float32)
    labels_gold = data.get("labels_gold")
    rng = np.random.default_rng(seed)

    if labels_gold is None:
        # online clustering over the pool at each epoch
        use_gold = False
    else:
        labels_gold = np.asarray(labels_gold, dtype=np.int64)
        use_gold = True

    n = len(X)
    order = rng.permutation(n)
    batches = np.array_split(order, N_EPOCHS)

    out_rows: list[dict] = []
    admitted = np.zeros(n, dtype=bool)
    for epoch, batch in enumerate(batches):
        admitted[batch] = True
        idx = np.flatnonzero(admitted)
        X_pool = X[idx]

        # 10% per-cluster held-out as queries; everything else is training.
        if use_gold:
            labels_pool = labels_gold[idx]
        else:
            from gac.clustering import cluster_kmeans
            k = max(2, len(X_pool) // AVG_CLUSTER_SIZE)
            labels_pool = cluster_kmeans(X_pool, n_clusters=k, seed=seed)

        # Split queries
        keep = np.ones(len(X_pool), dtype=bool)
        q_idx_local: list[np.ndarray] = []
        for lab in np.unique(labels_pool):
            if lab < 0:
                continue
            cidx = np.flatnonzero(labels_pool == lab)
            if len(cidx) < 3:
                continue
            nq = max(1, int(np.ceil(len(cidx) * 0.1)))
            qi = rng.choice(cidx, size=nq, replace=False)
            keep[qi] = False
            q_idx_local.append(qi)
        if not q_idx_local:
            continue
        q_all = np.concatenate(q_idx_local)
        X_train = X_pool[keep]
        y_train = labels_pool[keep]
        X_q = X_pool[q_all]
        y_q = labels_pool[q_all]

        store = consolidate(X_train, y_train, strategy=strat, **strat_kw)

        t0 = time.time()
        id_res = identity_retrieval(X_q, y_q, store, strict=False)
        rk = recall_at_k(X_q, y_q, store, ks=(1, 10, 100))
        mrr = mrr_at_k(X_q, y_q, store, k=20)
        cov = coverage_at_theta(X_q, store, theta=0.8)
        dt = time.time() - t0

        out_rows.append({
            "cell_id": cell_id,
            "corpus": corpus,
            "model": model,
            "strategy": strat,
            "strategy_kw": json.dumps(strat_kw, sort_keys=True),
            "seed": seed,
            "epoch": epoch,
            "pool_size": int(len(X_pool)),
            "n_train": int(X_train.shape[0]),
            "n_query": int(X_q.shape[0]),
            "n_representatives": int(store.n_representatives),
            "compression": float(store.meta.get("compression", 0.0)),
            "identity_accuracy": id_res["accuracy"],
            "recall@1": rk["recall@1"],
            "recall@10": rk["recall@10"],
            "recall@100": rk["recall@100"],
            "mrr@20": mrr,
            "coverage@0.80": cov,
            "query_s": dt,
        })

    return out_rows


# ---------------------------------------------------------------------------
# shard + reduce
# ---------------------------------------------------------------------------


def run_shard(shard_id: int, config: dict, out_dir: str, ckpt_dir: str) -> str:
    n_shards = int(config.get("n_shards", 6))
    out_path = pathlib.Path(out_dir) / f"shard_{shard_id:02d}.jsonl"
    ckpt = pathlib.Path(ckpt_dir) / f"shard_{shard_id:02d}.completed.json"
    done: set[int] = set()
    if ckpt.exists():
        done = set(json.loads(ckpt.read_text()).get("done", []))
    if not out_path.exists():
        out_path.touch()

    with out_path.open("a") as f:
        for cid, corpus, model, strat, kw, seed in _cells():
            if cid % n_shards != shard_id:
                continue
            if cid in done:
                continue
            t0 = time.time()
            try:
                rows = _run_cell(cid, corpus, model, strat, kw, seed)
            except FileNotFoundError as e:
                print(f"[e9 shard {shard_id}] cid={cid} SKIP: {e}")
                done.add(cid); ckpt.write_text(json.dumps({"done": sorted(done)}))
                continue
            except Exception as e:
                print(f"[e9 shard {shard_id}] cid={cid} ERROR: {e}")
                done.add(cid); ckpt.write_text(json.dumps({"done": sorted(done)}))
                continue
            for r in rows:
                f.write(json.dumps(r) + "\n")
            f.flush(); os.fsync(f.fileno())
            done.add(cid); ckpt.write_text(json.dumps({"done": sorted(done)}))
            dt = time.time() - t0
            tail = rows[-1] if rows else {}
            print(
                f"[e9 shard {shard_id}] cid={cid} {corpus}/{model} {strat} "
                f"-> {dt:.1f}s mrr@20={tail.get('mrr@20', float('nan')):.3f}"
            )

    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd

    frames = []
    for p in shard_artifacts:
        if pathlib.Path(p).stat().st_size > 0:
            frames.append(pd.read_json(p, lines=True))
    df = pd.concat(frames, ignore_index=True) if frames else pd.DataFrame()
    out_path = pathlib.Path(out_dir) / "e9_results.parquet"
    df.to_parquet(out_path, index=False)
    if len(df):
        agg = (
            df.groupby(["corpus", "strategy"])
            .agg(
                mrr20=("mrr@20", "mean"),
                r10=("recall@10", "mean"),
                id_acc=("identity_accuracy", "mean"),
                cov08=("coverage@0.80", "mean"),
            )
            .reset_index()
        )
        (pathlib.Path(out_dir) / "e9_summary.json").write_text(
            json.dumps(agg.to_dict(orient="records"), indent=2, default=float)
        )
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    od = "./runs/e9_smoke"; cd = "./checkpoints/e9_smoke"
    pathlib.Path(od).mkdir(parents=True, exist_ok=True)
    pathlib.Path(cd).mkdir(parents=True, exist_ok=True)
    run_shard(0, {"n_shards": 1}, od, cd)
