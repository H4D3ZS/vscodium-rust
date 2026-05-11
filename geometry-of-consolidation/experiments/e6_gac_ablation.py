"""
E6 -- GAC ablation study.

Isolates the contribution of each GAC design choice:

    'gac_full'         : the production GAC (centroid / medoid+residual / prune
                         routed by spread + spectral concentration thresholds).
    'gac_no_residual'  : same routing, but medoid branch emits just the medoid
                         (no residual directions). Tests whether residuals help.
    'gac_oracle'       : oracle routing: for each cluster, we pick the operator
                         that minimises err_cap on held-out members. Upper-bound
                         on achievable GAC performance.
    'gac_random'       : random routing across the three operators (baseline to
                         check that routing matters at all).
    'gac_fixed_medoid' : route every cluster to medoid+residual.
    'gac_fixed_centroid': route every cluster to centroid.
    'gac_fixed_prune'  : route every cluster to top-p prune (kr=0.5).

Each variant is a CustomConsolidator built on the same per-cluster primitives
as GAC, with a different routing rule.
"""
from __future__ import annotations

import json
import os
import pathlib
import time

import numpy as np

from gac.clustering import cluster_kmeans
from gac.metrics import (
    coverage_at_theta,
    identity_retrieval,
    mrr_at_k,
    recall_at_k,
)
from gac.strategies import (
    CompressedStore,
    GACConsolidator,
)
from gac.theory import cluster_spread, rho_cluster


def _l2norm(X: np.ndarray, eps: float = 1e-12) -> np.ndarray:
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + eps)


def _iter_clusters(X: np.ndarray, labels: np.ndarray):
    for lab in np.unique(labels):
        if lab < 0:
            continue
        idx = np.flatnonzero(labels == lab)
        yield int(lab), idx, X[idx]


def _op_centroid(Xc: np.ndarray, idx: np.ndarray, lab: int) -> tuple[list[np.ndarray], list[int], list[int], list[str]]:
    v = Xc.mean(axis=0)
    v = v / (np.linalg.norm(v) + 1e-12)
    return [v], [lab], [-1], ["gac_centroid"]


def _op_medoid(Xc: np.ndarray, idx: np.ndarray, lab: int, with_residual: bool = True, r: int = 3) -> tuple[list[np.ndarray], list[int], list[int], list[str]]:
    c = Xc.mean(axis=0)
    c = c / (np.linalg.norm(c) + 1e-12)
    sims = Xc @ c
    best = int(np.argmax(sims))
    vecs = [Xc[best]]
    labs = [lab]; sids = [int(idx[best])]; origs = ["gac_medoid"]
    if with_residual and Xc.shape[0] > 1:
        Xc_centered = Xc - c[None, :]
        rr = min(r, Xc.shape[0] - 1)
        if rr > 0:
            n = Xc.shape[0]
            if n <= Xc.shape[1]:
                G = Xc_centered @ Xc_centered.T
                w, V = np.linalg.eigh(G)
                order = np.argsort(w)[::-1][:rr]
                w_sel = np.clip(w[order], 0, None)
                Vsamp = V[:, order]
                d_dirs = Xc_centered.T @ Vsamp / (np.sqrt(w_sel + 1e-12))
            else:
                U, s, Vt = np.linalg.svd(Xc_centered, full_matrices=False)
                d_dirs = Vt[:rr].T
            d_dirs = _l2norm(d_dirs.T)
            anchored = _l2norm(Xc[best][None, :] + 0.1 * d_dirs)
            for v in anchored:
                vecs.append(v); labs.append(lab); sids.append(-1); origs.append("gac_residual")
    return vecs, labs, sids, origs


def _op_prune(Xc: np.ndarray, idx: np.ndarray, lab: int, keep_ratio: float = 0.5) -> tuple[list[np.ndarray], list[int], list[int], list[str]]:
    n = Xc.shape[0]
    k = max(1, int(np.ceil(n * keep_ratio)))
    sims = Xc @ Xc.T
    mean_sim = (sims.sum(axis=1) - np.diag(sims)) / max(n - 1, 1)
    keep = np.argsort(mean_sim)[:k]
    vecs = []; labs = []; sids = []; origs = []
    for j in keep:
        vecs.append(Xc[j]); labs.append(lab); sids.append(int(idx[j])); origs.append("gac_prune")
    return vecs, labs, sids, origs


def _build_store(vecs, labs, sids, origs, meta: dict) -> CompressedStore:
    V = np.asarray(vecs, dtype=np.float32)
    V = V / (np.linalg.norm(V, axis=1, keepdims=True) + 1e-12)
    return CompressedStore(
        vectors=V,
        cluster_ids=np.asarray(labs, dtype=np.int64),
        source_ids=np.asarray(sids, dtype=np.int64),
        origin=np.asarray(origs, dtype=object),
        meta=meta,
    )


def _route(
    Xc: np.ndarray,
    idx: np.ndarray,
    lab: int,
    router: str,
    thresholds: tuple[float, float],
    tau_high: float = 0.55,
    seed: int = 0,
    X_all: np.ndarray | None = None,
    lbl_all: np.ndarray | None = None,
):
    """Return (vecs, labs, sids, origs, route_name)."""
    spread_safe, spread_unsafe = thresholds
    n = Xc.shape[0]
    spread = cluster_spread(Xc, normalize=False)
    rho = rho_cluster(Xc)

    if router == "gac_full":
        if rho > tau_high and spread < spread_safe:
            v, l, s, o = _op_centroid(Xc, idx, lab); route = "centroid"
        elif spread > spread_unsafe:
            v, l, s, o = _op_prune(Xc, idx, lab, keep_ratio=0.5); route = "prune"
        else:
            v, l, s, o = _op_medoid(Xc, idx, lab, with_residual=True, r=3); route = "medoid+residual"
    elif router == "gac_no_residual":
        if rho > tau_high and spread < spread_safe:
            v, l, s, o = _op_centroid(Xc, idx, lab); route = "centroid"
        elif spread > spread_unsafe:
            v, l, s, o = _op_prune(Xc, idx, lab, keep_ratio=0.5); route = "prune"
        else:
            v, l, s, o = _op_medoid(Xc, idx, lab, with_residual=False); route = "medoid"
    elif router == "gac_random":
        rng = np.random.default_rng(1000 * lab + seed)
        choice = rng.choice(["centroid", "medoid+residual", "prune"])
        if choice == "centroid":
            v, l, s, o = _op_centroid(Xc, idx, lab)
        elif choice == "medoid+residual":
            v, l, s, o = _op_medoid(Xc, idx, lab, with_residual=True, r=3)
        else:
            v, l, s, o = _op_prune(Xc, idx, lab, keep_ratio=0.5)
        route = choice
    elif router == "gac_fixed_centroid":
        v, l, s, o = _op_centroid(Xc, idx, lab); route = "centroid"
    elif router == "gac_fixed_medoid":
        v, l, s, o = _op_medoid(Xc, idx, lab, with_residual=True, r=3); route = "medoid+residual"
    elif router == "gac_fixed_prune":
        v, l, s, o = _op_prune(Xc, idx, lab, keep_ratio=0.5); route = "prune"
    elif router == "gac_oracle":
        # Evaluate every operator on the cluster's own members and pick the one
        # that minimises the cap-coverage error at theta=0.8.
        best_route = None; best_err = float("inf"); best_pack = None
        for cand_name, cand_pack in (
            ("centroid", _op_centroid(Xc, idx, lab)),
            ("medoid+residual", _op_medoid(Xc, idx, lab, with_residual=True, r=3)),
            ("prune", _op_prune(Xc, idx, lab, keep_ratio=0.5)),
        ):
            vecs_c, labs_c, sids_c, origs_c = cand_pack
            Vc = _l2norm(np.asarray(vecs_c, dtype=np.float32))
            # err_cap on THIS cluster only: fraction of members whose max cos to Vc < 0.8
            sims = Xc @ Vc.T
            err = float((sims.max(axis=1) < 0.8).mean())
            if err < best_err:
                best_err = err; best_route = cand_name; best_pack = cand_pack
        v, l, s, o = best_pack
        route = f"oracle:{best_route}"
    else:
        raise ValueError(f"Unknown router: {router}")
    return v, l, s, o, route


ROUTERS = [
    "gac_full",
    "gac_no_residual",
    "gac_random",
    "gac_fixed_centroid",
    "gac_fixed_medoid",
    "gac_fixed_prune",
    "gac_oracle",
]

CORPUS_MODEL_PAIRS = [
    # BGE-large primary suite
    ("drm_templated", "bge-large"),
    ("ms_marco", "bge-large"),
    ("wikipedia_sections", "bge-large"),
    ("arxiv_titles", "bge-large"),
    ("nq_questions", "bge-large"),
    ("hotpot_qa", "bge-large"),
    # MiniLM secondary suite (SPEC ablation requests cross-encoder check)
    ("drm_templated", "minilm"),
    ("ms_marco", "minilm"),
    ("wikipedia_sections", "minilm"),
    ("arxiv_titles", "minilm"),
    ("nq_questions", "minilm"),
    ("hotpot_qa", "minilm"),
]


def _data_path(corpus: str, model: str) -> pathlib.Path:
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    return root / corpus / model / "embeddings.npz"


def _thresholds(theta: float, d_eff: float, safe_mult=0.75, unsafe_mult=1.25):
    theta_prime = max(1e-3, 1.0 - theta)
    d_bar_crit = theta_prime * (2.0 ** (1.0 / max(d_eff, 1.0)))
    return (safe_mult * d_bar_crit, unsafe_mult * d_bar_crit)


def _run_cell(cid: int, corpus: str, model: str, router: str, seed: int = 0) -> dict:
    p = _data_path(corpus, model)
    if not p.exists():
        raise FileNotFoundError(f"Missing embeddings {p}")
    data = np.load(p, allow_pickle=True)
    X = data["X"].astype(np.float32)
    labels = data["labels_gold"] if "labels_gold" in data.files else cluster_kmeans(X, n_clusters=500)

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

    # Compute d_eff_global on train.
    Xc_train = X_train - X_train.mean(axis=0, keepdims=True)
    cov = (Xc_train.T @ Xc_train) / max(X_train.shape[0] - 1, 1)
    eigs = np.clip(np.linalg.eigvalsh(cov), 0, None)
    d_eff_g = float((eigs.sum() ** 2) / max((eigs ** 2).sum(), 1e-30))

    thresholds = _thresholds(theta=0.8, d_eff=d_eff_g)

    vecs, labs, sids, origs = [], [], [], []
    routes = []
    for lab, idx_c, Xc in _iter_clusters(X_train, labels_train):
        v, l, s, o, r = _route(
            Xc, idx_c, lab, router=router,
            thresholds=thresholds, seed=seed,
            X_all=X_train, lbl_all=labels_train,
        )
        vecs.extend(v); labs.extend(l); sids.extend(s); origs.extend(o); routes.append(r)

    store = _build_store(vecs, labs, sids, origs, meta={
        "strategy": router,
        "d_eff_global": d_eff_g,
        "compression": float(X_train.shape[0] / max(len(vecs), 1)),
    })

    id_res = identity_retrieval(X_query, labels_query, store, strict=False)
    r_at = recall_at_k(X_query, labels_query, store, ks=(1, 10, 100))
    mrr = mrr_at_k(X_query, labels_query, store, k=20)
    cov08 = coverage_at_theta(X_query, store, theta=0.8)

    # Route summary
    from collections import Counter
    route_counts = dict(Counter(routes))

    return {
        "cell_id": cid, "corpus": corpus, "model": model, "router": router, "seed": seed,
        "n_train": int(X_train.shape[0]), "n_query": int(X_query.shape[0]),
        "n_representatives": int(store.n_representatives),
        "compression": float(store.meta["compression"]),
        "identity_accuracy": id_res["accuracy"],
        "recall@1": r_at["recall@1"], "recall@10": r_at["recall@10"], "recall@100": r_at["recall@100"],
        "mrr@20": mrr, "coverage@0.80": cov08,
        "route_counts": json.dumps(route_counts),
        "d_eff_global": d_eff_g,
    }


def _cells():
    cid = 0
    for corpus, model in CORPUS_MODEL_PAIRS:
        for router in ROUTERS:
            yield cid, corpus, model, router
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
        for cid, corpus, model, router in _cells():
            if cid % n_shards != shard_id:
                continue
            for seed in seeds:
                key = f"{cid}-{seed}"
                if key in done:
                    continue
                t0 = time.time()
                try:
                    rec = _run_cell(cid, corpus, model, router, seed=seed)
                except FileNotFoundError as e:
                    print(f"[e6 shard {shard_id}] {key} SKIP: {e}")
                    done.add(key); ckpt.write_text(json.dumps({"done": sorted(done)}))
                    continue
                f.write(json.dumps(rec) + "\n"); f.flush(); os.fsync(f.fileno())
                done.add(key); ckpt.write_text(json.dumps({"done": sorted(done)}))
                dt = time.time() - t0
                print(f"[e6 shard {shard_id}] {corpus}/{router}/s{seed} -> "
                      f"{dt:.1f}s acc={rec['identity_accuracy']:.3f}")
    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd
    frames = [pd.read_json(p, lines=True) for p in shard_artifacts]
    df = pd.concat(frames, ignore_index=True) if frames else pd.DataFrame()
    out_path = pathlib.Path(out_dir) / "e6_results.parquet"
    df.to_parquet(out_path, index=False)
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    pathlib.Path("./runs/e6_smoke").mkdir(parents=True, exist_ok=True)
    pathlib.Path("./checkpoints/e6_smoke").mkdir(parents=True, exist_ok=True)
    run_shard(0, {"n_shards": 1, "seeds": [0]}, "./runs/e6_smoke", "./checkpoints/e6_smoke")
