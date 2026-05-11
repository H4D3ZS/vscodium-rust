"""
E2 -- Extended strategy sweep across real corpora.

Sweeps every consolidation strategy (centroid, medoid, importance_weighted,
selective_prune, gac) over multiple (corpus, embedding_model, cluster_size)
configurations, measuring identity retrieval, cluster-level recall,
coverage@theta, and compression ratio.

Corpora (by key, produced by data/build_*.py):
    wikipedia_sections   -- BGE-large + MiniLM
    c4_random            -- MiniLM (efficiency)
    arxiv_titles         -- BGE-large
    ms_marco             -- BGE-large
    nq_questions         -- BGE-large
    hotpot_qa            -- BGE-large
    drm_templated        -- BGE-large (synthetic identity test)

Each shard handles a disjoint slice of (corpus, model, strategy) cells.

Interface matches the standard:
    run_shard(shard_id, config, out_dir, ckpt_dir)
    reduce(shard_artifacts, config, out_dir)
"""
from __future__ import annotations

import json
import os
import pathlib
import time

import numpy as np

from gac.clustering import cluster_hdbscan, cluster_kmeans
from gac.metrics import cluster_level_recall, coverage_at_theta, identity_retrieval
from gac.strategies import consolidate
from gac.theory import cluster_spread, d_eff, spectral_bound


def _l2norm(X: np.ndarray) -> np.ndarray:
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)


def _cap_coverage_error(
    X_query: np.ndarray,
    query_cluster_ids: np.ndarray,
    store,
    theta: float,
) -> float:
    """Fraction of queries whose max cos-sim to reps of their own cluster < theta.
    This is the native quantity bounded by the Consolidation-Interference theorem.
    """
    Xn = _l2norm(X_query.astype(np.float32))
    Rn = _l2norm(store.vectors.astype(np.float32))
    S = Xn @ Rn.T
    store_cids = store.cluster_ids
    n = X_query.shape[0]
    fails = 0
    for c in np.unique(query_cluster_ids):
        member_idx = np.flatnonzero(query_cluster_ids == c)
        rep_idx = np.flatnonzero(store_cids == c)
        if rep_idx.size == 0:
            fails += member_idx.size
            continue
        sub = S[np.ix_(member_idx, rep_idx)]
        fails += int((sub.max(axis=1) < theta).sum())
    return fails / max(n, 1)


def _per_cluster_stats(X: np.ndarray, labels: np.ndarray) -> dict:
    """Mean per-cluster d_eff and d_bar (size-weighted)."""
    dbars = []
    deffs = []
    sizes = []
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
        return {"d_bar_mean": 0.0, "d_eff_local_mean": 0.0}
    sizes = np.asarray(sizes, dtype=np.float64)
    w = sizes / sizes.sum()
    return {
        "d_bar_mean": float(np.sum(w * np.asarray(dbars))),
        "d_eff_local_mean": float(np.sum(w * np.asarray(deffs))),
    }

# ---------------------------------------------------------------------------
# sweep grid
# ---------------------------------------------------------------------------

# Corpora/models whose embeddings are built on the Modal volume.
# Missing ones yield FileNotFoundError which the shard runner skips gracefully.
CORPUS_MODEL_PAIRS = [
    # BGE-large suite (primary)
    ("drm_templated", "bge-large"),
    ("ms_marco", "bge-large"),
    ("wikipedia_sections", "bge-large"),
    ("arxiv_titles", "bge-large"),
    ("nq_questions", "bge-large"),
    ("hotpot_qa", "bge-large"),
    ("popqa", "bge-large"),
    ("c4_random", "bge-large"),
    # Full encoder cross-product on core corpora
    *[("wikipedia_sections", m) for m in ("minilm", "bge-base", "e5-large", "mpnet", "nomic")],
    *[("ms_marco",           m) for m in ("minilm", "bge-base", "e5-large", "mpnet", "nomic")],
    *[("drm_templated",      m) for m in ("minilm", "bge-base", "e5-large", "mpnet", "nomic")],
    *[("nq_questions",       m) for m in ("minilm", "bge-base", "e5-large", "mpnet", "nomic")],
    *[("hotpot_qa",          m) for m in ("minilm", "bge-base", "e5-large", "mpnet", "nomic")],
    *[("arxiv_titles",       m) for m in ("minilm", "bge-base", "e5-large", "mpnet", "nomic")],
    ("popqa", "minilm"),
]

# 7 strategies × 5 compression levels (via selective_prune keep_ratio and
# gac theta). We also include a no-consolidation baseline (keep-all) that
# pins the achievable identity ceiling.
STRATEGIES = [
    ("no_consolidation", {"keep_ratio": 1.0}),  # alias for selective_prune keep=1.0
    ("centroid", {}),
    ("medoid", {}),
    ("importance_weighted", {}),
    # selective_prune at 5 compression ratios 2x/5x/10x/40x/200x
    ("selective_prune", {"keep_ratio": 0.5}),    # 2x
    ("selective_prune", {"keep_ratio": 0.20}),   # 5x
    ("selective_prune", {"keep_ratio": 0.10}),   # 10x
    ("selective_prune", {"keep_ratio": 0.025}),  # 40x
    ("selective_prune", {"keep_ratio": 0.005}),  # 200x
    # GAC at 4 thresholds (SPEC: θ ∈ {0.6, 0.7, 0.8, 0.9})
    ("gac", {"theta": 0.6}),
    ("gac", {"theta": 0.7}),
    ("gac", {"theta": 0.8}),
    ("gac", {"theta": 0.9}),
]

CLUSTERERS = [
    ("kmeans", {"n_clusters": 500}),
]

# Report coverage @ 4 thresholds (SPEC).
THETA_COVERAGE = [0.6, 0.7, 0.8, 0.9]

# Noisy-retrieval sigmas added to queries (in embedding space, renormalised).
# SPEC: sigma ∈ {0.0, 0.4, 1.0, 2.0}. sigma=0.0 is the clean baseline.
NOISE_SIGMAS: list[float] = [0.0, 0.4, 1.0, 2.0]


def _cells():
    cid = 0
    for corpus, model in CORPUS_MODEL_PAIRS:
        for clust_name, clust_kw in CLUSTERERS:
            for strat_name, strat_kw in STRATEGIES:
                for sigma in NOISE_SIGMAS:
                    yield (
                        cid, corpus, model, clust_name, clust_kw,
                        strat_name, strat_kw, sigma,
                    )
                    cid += 1


# ---------------------------------------------------------------------------
# data loading (shared with data/)
# ---------------------------------------------------------------------------


def _data_path(corpus: str, model: str) -> pathlib.Path:
    # Embeddings are saved as .npz {X, labels_gold, ids} by data/build_<corpus>.py.
    # If the corpus comes with gold cluster labels (e.g. wikipedia_sections =
    # section titles), they live in labels_gold; otherwise we cluster online.
    # Default on-Modal path; override with $GAC_DATA_DIR for local runs.
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    return root / corpus / model / "embeddings.npz"


def _load_corpus(corpus: str, model: str) -> dict:
    p = _data_path(corpus, model)
    if not p.exists():
        raise FileNotFoundError(
            f"Missing embeddings {p}; run data/build_{corpus}.py first."
        )
    data = np.load(p, allow_pickle=True)
    return {
        "X": data["X"],
        "labels_gold": data["labels_gold"] if "labels_gold" in data.files else None,
        "ids": data["ids"] if "ids" in data.files else None,
    }


# ---------------------------------------------------------------------------
# cell runner
# ---------------------------------------------------------------------------


def _run_cell(
    cell_id: int,
    corpus: str,
    model: str,
    clust_name: str,
    clust_kw: dict,
    strat_name: str,
    strat_kw: dict,
    sigma: float = 0.0,
    query_split: float = 0.1,
    seed: int = 0,
) -> dict:
    data = _load_corpus(corpus, model)
    X = data["X"].astype(np.float32)
    rng = np.random.default_rng(seed)

    # Optional gold labels give us a stable evaluation set; otherwise we
    # cluster once and use those labels for both training and eval.
    if data["labels_gold"] is not None:
        labels = data["labels_gold"]
    else:
        if clust_name == "hdbscan":
            labels = cluster_hdbscan(X, **clust_kw)
        else:
            labels = cluster_kmeans(X, **clust_kw)

    # Train / query split within each cluster.
    keep = np.ones(len(X), dtype=bool)
    query_idx = []
    for lab in np.unique(labels):
        if lab < 0:
            continue
        idx = np.flatnonzero(labels == lab)
        if len(idx) < 3:
            continue
        nq = max(1, int(np.ceil(len(idx) * query_split)))
        qi = rng.choice(idx, size=nq, replace=False)
        keep[qi] = False
        query_idx.append(qi)
    query_idx = np.concatenate(query_idx) if query_idx else np.array([], dtype=np.int64)

    X_train = X[keep]
    labels_train = labels[keep]
    X_query = X[query_idx]
    labels_query = labels[query_idx]

    # Add isotropic Gaussian noise to queries (SPEC: noisy retrieval). We
    # add in ambient space then renormalise to unit norm — mimicking an
    # encoder-estimation error at different levels of severity.
    if sigma > 0.0:
        noise = rng.normal(loc=0.0, scale=sigma, size=X_query.shape).astype(np.float32)
        X_query = X_query + noise
        X_query = _l2norm(X_query)

    # If train uses gold labels, apply clusterer when asked (to test effect
    # of imperfect clustering). For simplicity we use gold labels directly
    # when present; ablation block re-runs with clusterer-produced labels.
    if strat_name == "no_consolidation":
        # Keep EVERY training vector as its own representative (the identity
        # ceiling). Implemented via selective_prune with keep_ratio=1.0.
        store = consolidate(
            X_train, labels_train, strategy="selective_prune",
            **{**strat_kw, "keep_ratio": 1.0},
        )
    else:
        store = consolidate(X_train, labels_train, strategy=strat_name, **strat_kw)

    # Identity retrieval (cluster-level: top-1 has right cluster id).
    id_res = identity_retrieval(
        queries=X_query,
        query_cluster_ids=labels_query,
        store=store,
        strict=False,
    )

    # Strict source retrieval: top-1 must be the EXACT source vector.
    # Only meaningful for training queries, but we keep queries as held-out
    # items so strict mode is expected to be 0 for non-reidentifying
    # strategies (centroid/IW). We do NOT include this row here since X_query
    # are held-out -- their source_ids cannot appear in the store.
    # Instead we report the cap-coverage which is the theorem-native metric.

    # Cap-coverage error (the native quantity bounded by the theorem).
    err_cap = {
        f"err_cap_{th:.2f}": _cap_coverage_error(
            X_query, labels_query, store, theta=th
        )
        for th in THETA_COVERAGE
    }
    err_cap_08 = err_cap["err_cap_0.80"]
    err_cap_09 = err_cap["err_cap_0.90"]

    # Cluster-level recall @ 5.
    recall5 = cluster_level_recall(X_query, labels_query, store, k=5)

    # Coverage @ theta.
    cov = {
        f"coverage@{th:.2f}": coverage_at_theta(X_query, store, theta=th)
        for th in THETA_COVERAGE
    }

    # Global + local d_eff and d_bar on the train set.
    try:
        global_d_eff = float(d_eff(X_train, method="participation_ratio"))
    except Exception:
        global_d_eff = float("nan")
    per_cluster = _per_cluster_stats(X_train, labels_train)

    # Spectral bound using the LOCAL (per-cluster) quantities -- this is the
    # correct form of the theorem's bound for a heterogeneous corpus.
    bound_08 = spectral_bound(
        d_bar=per_cluster["d_bar_mean"],
        theta=0.8,
        d_eff_val=per_cluster["d_eff_local_mean"],
    )
    bound_09 = spectral_bound(
        d_bar=per_cluster["d_bar_mean"],
        theta=0.9,
        d_eff_val=per_cluster["d_eff_local_mean"],
    )

    return {
        "cell_id": cell_id,
        "corpus": corpus,
        "model": model,
        "clusterer": clust_name,
        "clusterer_kw": json.dumps(clust_kw, sort_keys=True),
        "strategy": strat_name,
        "strategy_kw": json.dumps(strat_kw, sort_keys=True),
        "sigma": float(sigma),
        "seed": seed,
        "n_train": int(X_train.shape[0]),
        "n_query": int(X_query.shape[0]),
        "n_clusters_used": int(len(np.unique(labels_train[labels_train >= 0]))),
        "n_representatives": int(store.n_representatives),
        "compression": float(store.meta.get("compression", 0.0)),
        "identity_accuracy": id_res["accuracy"],
        "err_cluster": 1.0 - id_res["accuracy"],
        **err_cap,
        "bound_0.80": bound_08,
        "bound_0.90": bound_09,
        "theorem_holds_0.80": (err_cap_08 + 1e-6) >= bound_08,
        "theorem_holds_0.90": (err_cap_09 + 1e-6) >= bound_09,
        "recall@5": recall5,
        **cov,
        "d_eff_global": global_d_eff,
        "d_eff_local_mean": per_cluster["d_eff_local_mean"],
        "d_bar_mean": per_cluster["d_bar_mean"],
        "strategy_meta": json.dumps(
            {k: v for k, v in store.meta.items() if k != "routing_counts"},
            default=float,
        ),
        "routing_counts": json.dumps(store.meta.get("routing_counts", {})),
    }


# ---------------------------------------------------------------------------
# shard + reduce
# ---------------------------------------------------------------------------


def run_shard(shard_id: int, config: dict, out_dir: str, ckpt_dir: str) -> str:
    n_shards = int(config.get("n_shards", 8))
    seeds = config.get("seeds", [int(config.get("seed", 0))])

    out_path = pathlib.Path(out_dir) / f"shard_{shard_id:02d}.jsonl"
    ckpt_path = pathlib.Path(ckpt_dir) / f"shard_{shard_id:02d}.completed.json"

    done: set[str] = set()
    if ckpt_path.exists():
        done = set(json.loads(ckpt_path.read_text()).get("done", []))
        print(f"[e2 shard {shard_id}] resume with {len(done)} (cell, seed) pairs done")
    if not out_path.exists():
        out_path.touch()

    with out_path.open("a") as f:
        for cid, corpus, model, clust_name, clust_kw, strat_name, strat_kw, sigma in _cells():
            if cid % n_shards != shard_id:
                continue
            for seed in seeds:
                key = f"{cid}-{seed}"
                if key in done:
                    continue
                t0 = time.time()
                try:
                    rec = _run_cell(
                        cid, corpus, model, clust_name, clust_kw,
                        strat_name, strat_kw, sigma=sigma, seed=seed,
                    )
                except FileNotFoundError as e:
                    print(f"[e2 shard {shard_id}] {key} SKIPPED: {e}")
                    done.add(key)
                    ckpt_path.write_text(json.dumps({"done": sorted(done)}))
                    continue
                except Exception as e:
                    print(f"[e2 shard {shard_id}] {key} ERROR {type(e).__name__}: {e}")
                    done.add(key)
                    ckpt_path.write_text(json.dumps({"done": sorted(done)}))
                    continue
                f.write(json.dumps(rec) + "\n")
                f.flush()
                os.fsync(f.fileno())
                done.add(key)
                ckpt_path.write_text(json.dumps({"done": sorted(done)}))
                dt = time.time() - t0
                print(f"[e2 shard {shard_id}] {key} {corpus}/{model} "
                      f"{clust_name} {strat_name} s={sigma} -> {dt:.1f}s  "
                      f"acc={rec['identity_accuracy']:.3f} cmp={rec['compression']:.2f}")

    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd

    frames = []
    for p in shard_artifacts:
        frames.append(pd.read_json(p, lines=True))
    df = pd.concat(frames, ignore_index=True)
    out_path = pathlib.Path(out_dir) / "e2_results.parquet"
    df.to_parquet(out_path, index=False)
    # Summary: per-strategy mean identity accuracy, coverage@0.8.
    agg = (
        df.groupby("strategy")
        .agg(
            mean_acc=("identity_accuracy", "mean"),
            mean_cov_08=("coverage@0.80", "mean"),
            mean_compression=("compression", "mean"),
        )
        .reset_index()
    )
    (pathlib.Path(out_dir) / "e2_summary.json").write_text(
        json.dumps(agg.to_dict(orient="records"), indent=2, default=float)
    )
    print(f"[e2 reduce] {len(df)} cells; per-strategy summary:")
    print(agg.to_string(index=False))
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    cfg = {"run_id": "smoke", "n_shards": 1, "seed": 0}
    out_dir = "./runs/e2_smoke"
    ckpt_dir = "./checkpoints/e2_smoke"
    pathlib.Path(out_dir).mkdir(parents=True, exist_ok=True)
    pathlib.Path(ckpt_dir).mkdir(parents=True, exist_ok=True)
    art = run_shard(0, cfg, out_dir, ckpt_dir)
    reduce([art], cfg, out_dir)
