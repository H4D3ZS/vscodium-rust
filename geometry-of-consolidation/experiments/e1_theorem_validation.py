"""
E1 -- Empirical validation of the Consolidation-Interference bound.

Theorem (SPEC §2.1):
    eps_id(C, r) >= 1 - P_{x in C}[ <phi(x), r> >= theta ]
                 >= 1 - c1 * (theta' / d_bar)^d_eff

The two inequalities give us TWO things to measure:

  (A) The *native* quantity: the fraction of cluster members whose cosine
      similarity to their assigned representative is BELOW theta.
      This is `err_cap` below. This is what the theorem literally bounds.

  (B) Downstream consequences when you treat the compressed store as a
      retrieval index:
        - err_cluster : top-1 retrieves the RIGHT cluster (easy; mostly 0
                        on well-separated synthetic data).
        - err_source  : top-1 retrieves the EXACT source item (hard; strict
                        identity recovery; lower-bounded by the same theorem).

Each row of the output has columns for all three errors so that reviewers
can see exactly which metric the bound governs and where the gap lives.

Shard schema
------------
The 3-D sweep grid:
    d_eff in [4, 8, 16, 32, 64]
    theta in [0.7, 0.8, 0.9, 0.95]
    d_bar in 20 points logarithmic between 1e-3 and 0.5

Total = 400 cells. Each cell is replicated for n_seeds seeds. Shard i handles
cell_ids with i == cell_idx % n_shards.

Interface
---------
    run_shard(shard_id, config, out_dir, ckpt_dir) -> str artifact
    reduce(shard_artifacts, config, out_dir)       -> str final artifact
"""
from __future__ import annotations

import json
import os
import pathlib
import time

import numpy as np

from gac.metrics import identity_retrieval
from gac.strategies import (
    CentroidConsolidator,
    GACConsolidator,
    MedoidConsolidator,
    SelectivePruningConsolidator,
)
from gac.theory import cluster_spread, d_eff, spectral_bound

# ---------------------------------------------------------------------------
# sweep grid
# ---------------------------------------------------------------------------

D_EFF_GRID = [4, 8, 16, 32, 64]
THETA_GRID = [0.70, 0.80, 0.90, 0.95]
DBAR_GRID = np.logspace(-3, np.log10(0.5), 20).tolist()
N_SEEDS_DEFAULT = 10
N_CLUSTERS_DEFAULT = 50
MEMBERS_PER_CLUSTER_DEFAULT = 30
AMBIENT_DIM = 128  # fixed ambient dim; d_eff is controlled via anisotropic noise

STRATEGIES = {
    "centroid": lambda theta, d_eff_val: CentroidConsolidator(),
    "medoid": lambda theta, d_eff_val: MedoidConsolidator(),
    "prune50": lambda theta, d_eff_val: SelectivePruningConsolidator(keep_ratio=0.5),
    "gac": lambda theta, d_eff_val: GACConsolidator(theta=theta, d_eff_global=d_eff_val),
}


# ---------------------------------------------------------------------------
# synthetic data w/ controllable d_bar and d_eff
# ---------------------------------------------------------------------------


def _make_cluster(
    center: np.ndarray,
    n_members: int,
    d_bar_target: float,
    d_eff_target: float,
    rng: np.random.Generator,
) -> np.ndarray:
    """
    Generate n_members points on the sphere whose mean pairwise cosine
    distance is ~= d_bar_target, using anisotropic Gaussian noise whose
    covariance spectrum has participation ratio ~= d_eff_target.
    """
    d = center.shape[0]
    d_eff_int = max(1, int(round(d_eff_target)))
    A = rng.normal(size=(d, d)).astype(np.float32)
    Q, _ = np.linalg.qr(A)
    lam = np.zeros(d, dtype=np.float32)
    lam[:d_eff_int] = 1.0
    lam += 1e-4
    lam = lam / lam.sum()
    cov_sqrt = Q * np.sqrt(lam)[None, :]
    raw = rng.normal(size=(n_members, d)).astype(np.float32)
    noise = raw @ cov_sqrt.T
    scale_lo, scale_hi = 1e-4, 2.0
    for _ in range(25):
        scale = 0.5 * (scale_lo + scale_hi)
        pts = center[None, :] + scale * noise
        pts = pts / (np.linalg.norm(pts, axis=1, keepdims=True) + 1e-12)
        spread = cluster_spread(pts, normalize=False)
        if spread < d_bar_target:
            scale_lo = scale
        else:
            scale_hi = scale
    pts = center[None, :] + 0.5 * (scale_lo + scale_hi) * noise
    pts = pts / (np.linalg.norm(pts, axis=1, keepdims=True) + 1e-12)
    return pts


def _make_dataset(
    n_clusters: int,
    members_per_cluster: int,
    d_bar: float,
    d_eff_target: float,
    rng: np.random.Generator,
    d: int = AMBIENT_DIM,
) -> tuple[np.ndarray, np.ndarray]:
    centers = rng.normal(size=(n_clusters, d)).astype(np.float32)
    centers /= np.linalg.norm(centers, axis=1, keepdims=True) + 1e-12
    Xs, labs = [], []
    for k in range(n_clusters):
        pts = _make_cluster(centers[k], members_per_cluster, d_bar, d_eff_target, rng)
        Xs.append(pts)
        labs.append(np.full(members_per_cluster, k, dtype=np.int64))
    return np.concatenate(Xs, axis=0), np.concatenate(labs, axis=0)


# ---------------------------------------------------------------------------
# cell runner
# ---------------------------------------------------------------------------


def _iter_cells():
    cid = 0
    for de in D_EFF_GRID:
        for th in THETA_GRID:
            for db in DBAR_GRID:
                yield cid, de, th, db
                cid += 1


def _l2norm(X: np.ndarray) -> np.ndarray:
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)


def _err_cap_coverage(
    X: np.ndarray,
    labels: np.ndarray,
    store,
    theta: float,
) -> float:
    """
    Fraction of cluster members x whose cosine similarity to the
    representative(s) of their own cluster is < theta.

    For each member x in cluster c:
      - Let R_c = representatives whose cluster_id == c (may be > 1 for
        pruning / GAC residuals).
      - Coverage = max cos(x, r) for r in R_c.
      - Failure if max cos < theta.

    This is the native quantity bounded by Theorem 2.1.
    """
    Xn = _l2norm(X.astype(np.float32))
    Rn = _l2norm(store.vectors.astype(np.float32))
    store_cids = store.cluster_ids
    S = Xn @ Rn.T  # (n, m)
    # Build per-member mask over representatives sharing its cluster.
    # For each member, find max similarity within its own cluster only.
    # Vectorised via grouping by cluster.
    fails = 0
    n = X.shape[0]
    unique_c = np.unique(labels)
    for c in unique_c:
        member_idx = np.flatnonzero(labels == c)
        rep_idx = np.flatnonzero(store_cids == c)
        if rep_idx.size == 0:
            # Cluster has no representative at all -> every member is a failure.
            fails += member_idx.size
            continue
        sub = S[np.ix_(member_idx, rep_idx)]
        max_sim = sub.max(axis=1)
        fails += int((max_sim < theta).sum())
    return fails / n


def _err_source_strict(
    X: np.ndarray,
    labels: np.ndarray,
    store,
) -> float:
    """
    Strict source retrieval: for each member x_i, top-1 over the ENTIRE store
    must be a representative with source_id == i.

    Centroid and IW have source_id == -1 for every rep -> error == 1.
    Medoid keeps a few source ids (one per cluster) -> error bounded below
    by (1 - 1/cluster_size).
    Prune/GAC may keep more members -> lower error.
    """
    source_ids = np.asarray(store.source_ids)
    queries = np.arange(X.shape[0], dtype=np.int64)
    res = identity_retrieval(
        queries=X,
        query_cluster_ids=labels,
        store=store,
        strict=True,
        query_source_ids=queries,
    )
    return 1.0 - res["accuracy"]


def _run_cell(
    cell_id: int,
    d_eff_target: float,
    theta: float,
    d_bar: float,
    n_seeds: int,
    n_clusters: int,
    members_per_cluster: int,
) -> list[dict]:
    records: list[dict] = []
    for seed in range(n_seeds):
        rng = np.random.default_rng(1000 * cell_id + seed)
        X, labels = _make_dataset(
            n_clusters, members_per_cluster, d_bar, d_eff_target, rng
        )
        realized_d_bar = float(
            np.mean(
                [cluster_spread(X[labels == k], normalize=False) for k in range(n_clusters)]
            )
        )
        d_eff_per_cluster = float(
            np.mean([d_eff(X[labels == k], method="participation_ratio")
                     for k in range(n_clusters)])
        )
        d_eff_whole = float(d_eff(X, method="participation_ratio"))
        realized_d_eff = d_eff_per_cluster

        bound = spectral_bound(
            d_bar=realized_d_bar,
            theta=theta,
            d_eff_val=realized_d_eff,
        )

        for strat_name, ctor in STRATEGIES.items():
            store = ctor(theta, d_eff_target).fit_transform(X, labels)
            err_cluster = 1.0 - identity_retrieval(
                queries=X,
                query_cluster_ids=labels,
                store=store,
                strict=False,
            )["accuracy"]
            err_cap = _err_cap_coverage(X, labels, store, theta)
            err_source = _err_source_strict(X, labels, store)

            # Primary "error" used for violation checking = the native
            # cap-coverage quantity. The theorem bounds THIS below.
            err_primary = err_cap
            records.append(
                {
                    "cell_id": cell_id,
                    "d_eff_target": d_eff_target,
                    "theta": theta,
                    "d_bar_target": d_bar,
                    "d_bar_realized": realized_d_bar,
                    "d_eff_realized": realized_d_eff,
                    "d_eff_per_cluster": d_eff_per_cluster,
                    "d_eff_whole": d_eff_whole,
                    "seed": seed,
                    "strategy": strat_name,
                    "err_cluster": err_cluster,
                    "err_cap": err_cap,
                    "err_source": err_source,
                    "error": err_primary,  # kept for back-compat
                    "bound": bound,
                    # err_cap is the *primary* quantity the theorem bounds.
                    # Positive gap (err_cap >= bound) = bound is valid and loose.
                    # Negative gap = bound is STRICTLY violated (signals that
                    # our synthetic data does not satisfy the Axioms A1-A5 under
                    # which the theorem holds, OR d_eff is mis-estimated).
                    "gap_cap": err_cap - bound,
                    "gap_source": err_source - bound,
                    "theorem_holds_cap": err_cap + 1e-6 >= bound,
                    "theorem_holds_source": err_source + 1e-6 >= bound,
                    # kept for back-compat
                    "violation": err_cap + 1e-6 < bound,
                    "slack_primary": bound - err_primary,
                    "slack_source": bound - err_source,
                    "n_representatives": int(store.n_representatives),
                    "compression": float(store.meta.get("compression", 0.0)),
                }
            )
    return records


# ---------------------------------------------------------------------------
# shard + reduce
# ---------------------------------------------------------------------------


def run_shard(shard_id: int, config: dict, out_dir: str, ckpt_dir: str) -> str:
    n_shards = int(config.get("n_shards", 4))
    n_seeds = int(config.get("n_seeds", N_SEEDS_DEFAULT))
    n_clusters = int(config.get("n_clusters", N_CLUSTERS_DEFAULT))
    members_per_cluster = int(
        config.get("members_per_cluster", MEMBERS_PER_CLUSTER_DEFAULT)
    )

    out_path = pathlib.Path(out_dir) / f"shard_{shard_id:02d}.jsonl"
    ckpt_path = pathlib.Path(ckpt_dir) / f"shard_{shard_id:02d}.completed.json"
    done: set[int] = set()
    if ckpt_path.exists():
        done = set(json.loads(ckpt_path.read_text()).get("done", []))
        print(f"[e1 shard {shard_id}] resume with {len(done)} cells already done")

    if not out_path.exists():
        out_path.touch()

    with out_path.open("a") as f:
        for cid, de, th, db in _iter_cells():
            if cid % n_shards != shard_id:
                continue
            if cid in done:
                continue
            t0 = time.time()
            records = _run_cell(
                cid, de, th, db, n_seeds, n_clusters, members_per_cluster
            )
            for r in records:
                f.write(json.dumps(r) + "\n")
            f.flush()
            os.fsync(f.fileno())
            done.add(cid)
            ckpt_path.write_text(json.dumps({"done": sorted(done)}))
            dt = time.time() - t0
            print(f"[e1 shard {shard_id}] cell {cid} d_eff={de} theta={th:.2f} "
                  f"d_bar={db:.4f} -> {dt:.1f}s")

    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd

    frames = []
    for p in shard_artifacts:
        frames.append(pd.read_json(p, lines=True))
    df = pd.concat(frames, ignore_index=True)
    out_path = pathlib.Path(out_dir) / "e1_results.parquet"
    df.to_parquet(out_path, index=False)

    # Summary by strategy across all 3 error metrics, plus violation rate on
    # the NATIVE theorem quantity (err_cap).
    by_strat = df.groupby("strategy").agg(
        err_cluster_mean=("err_cluster", "mean"),
        err_cap_mean=("err_cap", "mean"),
        err_source_mean=("err_source", "mean"),
        bound_mean=("bound", "mean"),
        theorem_holds_cap_rate=("theorem_holds_cap", "mean"),
        theorem_holds_source_rate=("theorem_holds_source", "mean"),
        mean_gap_cap=("gap_cap", "mean"),
        mean_gap_source=("gap_source", "mean"),
    )
    summary = {
        "n_rows": int(len(df)),
        "n_cells": int(df["cell_id"].nunique()),
        "strategies": by_strat.round(4).to_dict(orient="index"),
        "n_violations_by_strategy": df.groupby("strategy")["violation"].sum().astype(int).to_dict(),
    }
    (pathlib.Path(out_dir) / "e1_summary.json").write_text(
        json.dumps(summary, indent=2)
    )
    print(f"[e1 reduce] {len(df)} rows across {df['cell_id'].nunique()} cells")
    print(f"[e1 reduce] summary by strategy:\n{by_strat.round(4).to_string()}")
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    cfg = {"run_id": "smoke", "n_shards": 1, "n_seeds": 2, "n_clusters": 20,
           "members_per_cluster": 20}
    out_dir = "./runs/e1_smoke"
    ckpt_dir = "./checkpoints/e1_smoke"
    pathlib.Path(out_dir).mkdir(parents=True, exist_ok=True)
    pathlib.Path(ckpt_dir).mkdir(parents=True, exist_ok=True)
    art = run_shard(0, cfg, out_dir, ckpt_dir)
    reduce([art], cfg, out_dir)
