"""
E3 -- Learned / classical vector-compression baselines.

Every method in this file reduces the *per-vector memory footprint* of a
database D = (n, d) floats to ~ n * d / compression_ratio bytes (or scalar
count, when we report it that way), and we evaluate the same identity-
retrieval and cap-coverage metrics that E2 uses.

Baselines:

    pq_{m}      : Product Quantisation with m subquantisers, 256 centroids each,
                  via faiss.ProductQuantizer. Compression ~ (d * 4) / m bytes/vec.
    opq_{m}     : OPQ rotation + PQ m subquantisers.  Same ratio as PQ.
    lsh_{nbits} : Random hyperplane (cosine) LSH with nbits bits/vector.
                  Compression ~ (d * 32) / nbits bits/vec.
    pca_int8_{k}: PCA down to k components then int8 quantise.
                  Compression ~ (d * 4) / k bytes/vec (modulo quantisation).
    hnsw_prune_{kr}: build HNSW over full vectors, keep top `kr` most distinct
                     per cluster (subset -- true vectors, no distortion).
                     Compression = 1 / kr.

All are evaluated at the same train/query split as E2 so numbers are directly
comparable to centroid / medoid / GAC.

IMPORTANT: Every baseline returns a `CompressedStore`-like view so we can
reuse gac.metrics. For PQ/OPQ/LSH/PCA-int8 the store uses
*reconstructed* vectors (equivalent to what a retrieval system would produce
from the compressed codes) and cluster_ids=original per-member cluster id
(we keep ALL members, so compression is per-vector, not per-cluster).

For the compression-ratio comparison with E2 centroid/medoid/GAC (which
compress cluster -> rep), we also report `bytes_per_store` so reviewers
can compare apples-to-apples on memory.
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
from gac.strategies import CompressedStore


# ---------------------------------------------------------------------------
# helpers
# ---------------------------------------------------------------------------


def _l2norm(X: np.ndarray) -> np.ndarray:
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)


def _as_store(vectors: np.ndarray, cluster_ids: np.ndarray, origin_tag: str,
              meta: dict) -> CompressedStore:
    m = vectors.shape[0]
    return CompressedStore(
        vectors=_l2norm(vectors.astype(np.float32)),
        cluster_ids=np.asarray(cluster_ids, dtype=np.int64),
        source_ids=np.full(m, -1, dtype=np.int64),
        origin=np.asarray([origin_tag] * m, dtype=object),
        meta=meta,
    )


# ---------------------------------------------------------------------------
# Baselines
# ---------------------------------------------------------------------------


def pq_encode_reconstruct(X: np.ndarray, m: int, nbits: int = 8) -> np.ndarray:
    """Product Quantisation using faiss. Returns reconstructed vectors."""
    import faiss

    n, d = X.shape
    if d % m != 0:
        # Pad to multiple of m.
        pad = m - (d % m)
        Xp = np.concatenate([X, np.zeros((n, pad), dtype=X.dtype)], axis=1)
        d_pad = d + pad
    else:
        Xp = X
        d_pad = d
    pq = faiss.ProductQuantizer(d_pad, m, nbits)
    pq.train(Xp.astype(np.float32))
    codes = pq.compute_codes(Xp.astype(np.float32))
    rec = pq.decode(codes)
    if d_pad != d:
        rec = rec[:, :d]
    return rec.astype(np.float32)


def opq_encode_reconstruct(X: np.ndarray, m: int, nbits: int = 8) -> np.ndarray:
    """OPQ rotation + PQ via faiss OPQMatrix composed with ProductQuantizer."""
    import faiss

    n, d = X.shape
    if d % m != 0:
        pad = m - (d % m)
        Xp = np.concatenate([X, np.zeros((n, pad), dtype=X.dtype)], axis=1)
        d_pad = d + pad
    else:
        Xp = X
        d_pad = d
    opq = faiss.OPQMatrix(d_pad, m)
    opq.train(Xp.astype(np.float32))
    Xr = opq.apply_py(Xp.astype(np.float32))
    pq = faiss.ProductQuantizer(d_pad, m, nbits)
    pq.train(Xr)
    codes = pq.compute_codes(Xr)
    rec_rot = pq.decode(codes)
    # Invert OPQ rotation.
    R = faiss.vector_to_array(opq.A).reshape(d_pad, d_pad)
    rec = rec_rot @ R  # OPQ A is orthogonal, A^T = A^{-1}, apply_py does x @ A^T
    # Sanity check orientation: apply_py computes y = x @ A^T, so to invert use y @ A.
    if rec.shape[1] != d:
        rec = rec[:, :d]
    return rec.astype(np.float32)


def lsh_encode_reconstruct(X: np.ndarray, nbits: int, seed: int = 0) -> np.ndarray:
    """Random hyperplane LSH reconstructed BACK into d-dim space for cosine eval.

    Codes = sign(X @ W) with W in R^{d x nbits}. Reconstruction projects the
    sign code into d-dim: rec = signs @ W.T / nbits, which is the classical
    LSH estimator of the direction of X.
    """
    n, d = X.shape
    rng = np.random.default_rng(seed)
    W = rng.normal(size=(d, nbits)).astype(np.float32)
    W /= np.linalg.norm(W, axis=0, keepdims=True) + 1e-12
    signs = np.sign(X @ W)  # (n, nbits) in {-1, 0, +1}
    signs[signs == 0] = 1.0
    # Project codes back to d-dim: this keeps the representation comparable
    # with query vectors in the original space.
    rec = signs @ W.T / nbits  # (n, d)
    return rec.astype(np.float32)


def pca_int8_reconstruct(X: np.ndarray, k: int) -> np.ndarray:
    """PCA to k dims then int8 quantise per-dimension, then reconstruct."""
    n, d = X.shape
    mu = X.mean(axis=0, keepdims=True)
    Xc = X - mu
    # Use SVD for exact PCA.
    U, s, Vt = np.linalg.svd(Xc, full_matrices=False)
    k = min(k, Vt.shape[0])
    comp = Vt[:k]  # (k, d)
    Z = Xc @ comp.T  # (n, k)
    # Per-dim int8 quantisation.
    zmax = np.max(np.abs(Z), axis=0, keepdims=True) + 1e-12
    Zq = np.round(Z / zmax * 127).clip(-127, 127).astype(np.int8)
    Zdq = (Zq.astype(np.float32) / 127.0) * zmax
    rec = Zdq @ comp + mu
    return rec.astype(np.float32)


def hnsw_prune_subset(
    X: np.ndarray, labels: np.ndarray, keep_ratio: float
) -> tuple[np.ndarray, np.ndarray]:
    """
    HNSW + in-cluster distinctiveness prune: keep the top keep_ratio of members
    per cluster, ranked by MEAN peer similarity within the HNSW top-20 neighbours.

    This simulates "prune with a fast ANN index", which a real system would do.
    Returns (vectors_kept, cluster_ids_kept).
    """
    try:
        import hnswlib
    except ImportError:
        # Fallback to pure-numpy brute-force if hnswlib not installed locally.
        return _brute_prune_subset(X, labels, keep_ratio)

    n, d = X.shape
    p = hnswlib.Index(space="cosine", dim=d)
    p.init_index(max_elements=n, ef_construction=100, M=16)
    p.add_items(X.astype(np.float32), np.arange(n))
    p.set_ef(64)

    kept_vec = []
    kept_lab = []
    for c in np.unique(labels):
        if c < 0:
            continue
        idx = np.flatnonzero(labels == c)
        if len(idx) <= 1:
            kept_vec.append(X[idx])
            kept_lab.append(labels[idx])
            continue
        # Distinctiveness score per member: mean similarity to top-20 ANN neighbours
        # within the same cluster. Lower = more distinct.
        Xc = X[idx]
        dists_I = p.knn_query(Xc, k=min(20, n))[1]  # (m, 20) similarities (1 - cos)
        mean_sim = 1.0 - dists_I.mean(axis=1)
        k = max(1, int(np.ceil(len(idx) * keep_ratio)))
        keep_order = np.argsort(mean_sim)[:k]
        kept_vec.append(Xc[keep_order])
        kept_lab.append(labels[idx][keep_order])
    V = np.concatenate(kept_vec, axis=0)
    L = np.concatenate(kept_lab, axis=0)
    return V, L


def _brute_prune_subset(
    X: np.ndarray, labels: np.ndarray, keep_ratio: float
) -> tuple[np.ndarray, np.ndarray]:
    """Fallback prune using full O(n^2) peer similarity (like SelectivePruning)."""
    Xn = _l2norm(X.astype(np.float32))
    kept_vec = []
    kept_lab = []
    for c in np.unique(labels):
        if c < 0:
            continue
        idx = np.flatnonzero(labels == c)
        Xc = Xn[idx]
        n = Xc.shape[0]
        k = max(1, int(np.ceil(n * keep_ratio)))
        if k >= n:
            kept_vec.append(Xc)
            kept_lab.append(labels[idx])
            continue
        sims = Xc @ Xc.T
        mean_sim = (sims.sum(axis=1) - np.diag(sims)) / (n - 1)
        keep = np.argsort(mean_sim)[:k]
        kept_vec.append(Xc[keep])
        kept_lab.append(labels[idx][keep])
    V = np.concatenate(kept_vec, axis=0)
    L = np.concatenate(kept_lab, axis=0)
    return V, L


# ---------------------------------------------------------------------------
# baseline registry -- (name, builder) where builder returns a CompressedStore
# ---------------------------------------------------------------------------


def _bytes_per_vec_original(d: int) -> int:
    return d * 4  # float32


def _build_pq(X_train: np.ndarray, labels_train: np.ndarray, m: int) -> CompressedStore:
    rec = pq_encode_reconstruct(X_train, m=m, nbits=8)
    d = X_train.shape[1]
    return _as_store(
        rec, labels_train, f"pq_m{m}",
        meta={
            "strategy": f"pq_m{m}",
            "bytes_per_vec": m,  # m bytes per code at nbits=8
            "compression_bits": _bytes_per_vec_original(d) * 8 / (m * 8),
            "compression": float(X_train.shape[0] / rec.shape[0]),
        },
    )


def _build_opq(X_train: np.ndarray, labels_train: np.ndarray, m: int) -> CompressedStore:
    rec = opq_encode_reconstruct(X_train, m=m, nbits=8)
    d = X_train.shape[1]
    return _as_store(
        rec, labels_train, f"opq_m{m}",
        meta={
            "strategy": f"opq_m{m}",
            "bytes_per_vec": m,
            "compression_bits": _bytes_per_vec_original(d) * 8 / (m * 8),
            "compression": 1.0,
        },
    )


def _build_lsh(X_train: np.ndarray, labels_train: np.ndarray, nbits: int) -> CompressedStore:
    rec = lsh_encode_reconstruct(X_train, nbits=nbits, seed=0)
    d = X_train.shape[1]
    return _as_store(
        rec, labels_train, f"lsh_b{nbits}",
        meta={
            "strategy": f"lsh_b{nbits}",
            "bytes_per_vec": nbits / 8.0,
            "compression_bits": _bytes_per_vec_original(d) * 8 / nbits,
            "compression": 1.0,
        },
    )


def _build_pca_int8(X_train: np.ndarray, labels_train: np.ndarray, k: int) -> CompressedStore:
    rec = pca_int8_reconstruct(X_train, k=k)
    d = X_train.shape[1]
    return _as_store(
        rec, labels_train, f"pca_k{k}",
        meta={
            "strategy": f"pca_int8_k{k}",
            "bytes_per_vec": k,  # int8 per coordinate
            "compression_bits": _bytes_per_vec_original(d) * 8 / (k * 8),
            "compression": 1.0,
        },
    )


def _build_hnsw_prune(
    X_train: np.ndarray, labels_train: np.ndarray, keep_ratio: float
) -> CompressedStore:
    V, L = hnsw_prune_subset(X_train, labels_train, keep_ratio=keep_ratio)
    d = X_train.shape[1]
    return _as_store(
        V, L, f"hnsw_prune_kr{keep_ratio}",
        meta={
            "strategy": f"hnsw_prune_kr{keep_ratio}",
            "bytes_per_vec": _bytes_per_vec_original(d),
            "compression": float(X_train.shape[0] / V.shape[0]),
        },
    )


# Sweep grid: match memory-footprint ratios 2, 5, 10, 40, 200 approx.
# For BGE-large d=1024 (4096 bytes/vec), we get (m for PQ/OPQ):
#   ratio 2  -> m=2048/1 not feasible (limit is d/2)... use m=512 (8 bytes... ratio 8)
# Instead parameterise by raw PQ m in {4, 8, 16, 32, 64, 128}. Same for LSH
# nbits in {8, 32, 128, 512}. PCA k in {2, 4, 8, 16, 32, 64}.
BASELINES = [
    # PQ
    *[("pq",          dict(m=m))       for m in (4, 8, 16, 32, 64, 128)],
    # OPQ
    *[("opq",         dict(m=m))       for m in (4, 8, 16, 32, 64)],
    # LSH
    *[("lsh",         dict(nbits=b))   for b in (8, 32, 128, 512)],
    # PCA+int8
    *[("pca_int8",    dict(k=k))       for k in (2, 4, 8, 16, 32, 64)],
    # HNSW subset prune
    *[("hnsw_prune",  dict(keep_ratio=kr)) for kr in (0.05, 0.1, 0.25, 0.5)],
]

BUILDERS = {
    "pq": _build_pq,
    "opq": _build_opq,
    "lsh": _build_lsh,
    "pca_int8": _build_pca_int8,
    "hnsw_prune": _build_hnsw_prune,
}

# Primary BGE-large suite + cross-encoder extension on Wiki+MSMARCO to
# satisfy SPEC C3 ("encoder-universal"). Missing artifact files are skipped
# gracefully by the shard runner.
CORPUS_MODEL_PAIRS = [
    # Primary BGE-large suite
    ("drm_templated", "bge-large"),
    ("ms_marco", "bge-large"),
    ("wikipedia_sections", "bge-large"),
    ("arxiv_titles", "bge-large"),
    ("nq_questions", "bge-large"),
    ("hotpot_qa", "bge-large"),
    # Encoder cross-product on Wiki + MSMARCO (5 extra encoders each)
    ("wikipedia_sections", "minilm"),
    ("wikipedia_sections", "bge-base"),
    ("wikipedia_sections", "e5-large"),
    ("wikipedia_sections", "mpnet"),
    ("wikipedia_sections", "nomic"),
    ("ms_marco", "minilm"),
    ("ms_marco", "bge-base"),
    ("ms_marco", "e5-large"),
    ("ms_marco", "mpnet"),
    ("ms_marco", "nomic"),
]


def _cells():
    cid = 0
    for corpus, model in CORPUS_MODEL_PAIRS:
        for family, kw in BASELINES:
            yield cid, corpus, model, family, kw
            cid += 1


def _data_path(corpus: str, model: str) -> pathlib.Path:
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    return root / corpus / model / "embeddings.npz"


def _load_corpus(corpus: str, model: str) -> dict:
    p = _data_path(corpus, model)
    if not p.exists():
        raise FileNotFoundError(f"Missing embeddings {p}")
    data = np.load(p, allow_pickle=True)
    return {
        "X": data["X"],
        "labels_gold": data["labels_gold"] if "labels_gold" in data.files else None,
    }


def _run_cell(
    cell_id: int, corpus: str, model: str, family: str, kw: dict, seed: int = 0
) -> dict:
    data = _load_corpus(corpus, model)
    X = data["X"].astype(np.float32)
    labels = data["labels_gold"]
    if labels is None:
        # Use kmeans as a default labelling.
        from gac.clustering import cluster_kmeans
        labels = cluster_kmeans(X, n_clusters=500)
    rng = np.random.default_rng(seed)

    # 10% query split per cluster (same as E2).
    keep = np.ones(len(X), dtype=bool)
    query_idx = []
    for lab in np.unique(labels):
        if lab < 0:
            continue
        idx = np.flatnonzero(labels == lab)
        if len(idx) < 3:
            continue
        nq = max(1, int(np.ceil(len(idx) * 0.1)))
        qi = rng.choice(idx, size=nq, replace=False)
        keep[qi] = False
        query_idx.append(qi)
    query_idx = np.concatenate(query_idx) if query_idx else np.array([], dtype=np.int64)

    X_train = X[keep]; labels_train = labels[keep]
    X_query = X[query_idx]; labels_query = labels[query_idx]

    t0 = time.time()
    store = BUILDERS[family](X_train, labels_train, **kw)
    t_build = time.time() - t0

    id_res = identity_retrieval(X_query, labels_query, store, strict=False)
    cov08 = coverage_at_theta(X_query, store, theta=0.8)
    cov09 = coverage_at_theta(X_query, store, theta=0.9)
    r_at = recall_at_k(X_query, labels_query, store, ks=(1, 10, 100))
    mrr = mrr_at_k(X_query, labels_query, store, k=20)

    return {
        "cell_id": cell_id,
        "corpus": corpus,
        "model": model,
        "family": family,
        "strategy": store.meta.get("strategy"),
        "kw": json.dumps(kw, sort_keys=True),
        "seed": seed,
        "n_train": int(X_train.shape[0]),
        "n_query": int(X_query.shape[0]),
        "n_representatives": int(store.n_representatives),
        "compression": float(store.meta.get("compression", 1.0)),
        "bytes_per_vec": float(store.meta.get("bytes_per_vec", 0.0)),
        "identity_accuracy": float(id_res["accuracy"]),
        "err_cluster": float(1.0 - id_res["accuracy"]),
        "coverage@0.80": cov08,
        "coverage@0.90": cov09,
        "recall@1": r_at["recall@1"],
        "recall@10": r_at["recall@10"],
        "recall@100": r_at["recall@100"],
        "mrr@20": mrr,
        "t_build_s": t_build,
    }


# ---------------------------------------------------------------------------
# shard + reduce
# ---------------------------------------------------------------------------


def run_shard(shard_id: int, config: dict, out_dir: str, ckpt_dir: str) -> str:
    n_shards = int(config.get("n_shards", 8))
    seed = int(config.get("seed", 0))

    out_path = pathlib.Path(out_dir) / f"shard_{shard_id:02d}.jsonl"
    ckpt_path = pathlib.Path(ckpt_dir) / f"shard_{shard_id:02d}.completed.json"
    done: set[int] = set()
    if ckpt_path.exists():
        done = set(json.loads(ckpt_path.read_text()).get("done", []))
        print(f"[e3 shard {shard_id}] resume with {len(done)} cells done")
    if not out_path.exists():
        out_path.touch()

    with out_path.open("a") as f:
        for cid, corpus, model, family, kw in _cells():
            if cid % n_shards != shard_id:
                continue
            if cid in done:
                continue
            t0 = time.time()
            try:
                rec = _run_cell(cid, corpus, model, family, kw, seed=seed)
            except FileNotFoundError as e:
                print(f"[e3 shard {shard_id}] cell {cid} SKIP: {e}")
                done.add(cid)
                ckpt_path.write_text(json.dumps({"done": sorted(done)}))
                continue
            except Exception as e:  # pragma: no cover
                print(f"[e3 shard {shard_id}] cell {cid} FAIL: {e}")
                done.add(cid)
                ckpt_path.write_text(json.dumps({"done": sorted(done)}))
                continue
            f.write(json.dumps(rec) + "\n")
            f.flush()
            os.fsync(f.fileno())
            done.add(cid)
            ckpt_path.write_text(json.dumps({"done": sorted(done)}))
            dt = time.time() - t0
            print(f"[e3 shard {shard_id}] {cid} {corpus} {family}/{kw} "
                  f"-> {dt:.1f}s acc={rec['identity_accuracy']:.3f}")

    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd
    frames = [pd.read_json(p, lines=True) for p in shard_artifacts]
    df = pd.concat(frames, ignore_index=True) if frames else pd.DataFrame()
    out_path = pathlib.Path(out_dir) / "e3_results.parquet"
    df.to_parquet(out_path, index=False)
    if len(df):
        agg = (
            df.groupby(["family", "corpus"])
            .agg(mean_acc=("identity_accuracy", "mean"),
                 mean_cov=("coverage@0.80", "mean"),
                 mean_bytes=("bytes_per_vec", "mean"))
            .reset_index()
        )
        (pathlib.Path(out_dir) / "e3_summary.json").write_text(
            json.dumps(agg.to_dict(orient="records"), indent=2, default=float)
        )
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    cfg = {"run_id": "smoke", "n_shards": 1, "seed": 0}
    out_dir = "./runs/e3_smoke"
    ckpt_dir = "./checkpoints/e3_smoke"
    pathlib.Path(out_dir).mkdir(parents=True, exist_ok=True)
    pathlib.Path(ckpt_dir).mkdir(parents=True, exist_ok=True)
    art = run_shard(0, cfg, out_dir, ckpt_dir)
    reduce([art], cfg, out_dir)
