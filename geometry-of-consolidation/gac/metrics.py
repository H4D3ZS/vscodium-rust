"""
Retrieval metrics for consolidation experiments.

Identity retrieval:
  Query = a held-out member of cluster k, compressed store has m_k representatives
  tagged with cluster_id=k. Hit iff top-1 among all representatives belongs
  to k (and, for 'strict' mode, has source_id == query index).

Coverage @ theta:
  Fraction of queries whose top-1 similarity to the compressed store is >= theta.
"""
from __future__ import annotations

import numpy as np

from gac.strategies import CompressedStore


def _l2norm(X: np.ndarray) -> np.ndarray:
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)


def identity_retrieval(
    queries: np.ndarray,
    query_cluster_ids: np.ndarray,
    store: CompressedStore,
    strict: bool = False,
    query_source_ids: np.ndarray | None = None,
) -> dict:
    """
    Identity-retrieval metric.

    Parameters
    ----------
    queries              : (q, d) query embeddings
    query_cluster_ids    : (q,) ground-truth cluster id for each query
    store                : CompressedStore
    strict               : if True, hit also requires matching source id
                           (only meaningful when the query is a stored item).
    query_source_ids     : (q,) required if strict=True.

    Returns dict with {accuracy, top1_sims}.
    """
    Q = _l2norm(queries.astype(np.float32))
    R = _l2norm(store.vectors.astype(np.float32))
    S = Q @ R.T
    top1 = np.argmax(S, axis=1)
    top1_sims = S[np.arange(S.shape[0]), top1]
    predicted_cluster = store.cluster_ids[top1]
    correct = predicted_cluster == query_cluster_ids
    if strict:
        if query_source_ids is None:
            raise ValueError("strict mode requires query_source_ids")
        predicted_source = store.source_ids[top1]
        correct = correct & (predicted_source == query_source_ids)
    return {
        "accuracy": float(correct.mean()),
        "top1_sims": top1_sims,
        "n": int(len(queries)),
    }


def coverage_at_theta(
    queries: np.ndarray,
    store: CompressedStore,
    theta: float = 0.8,
) -> float:
    """Fraction of queries with max cosine similarity to the store >= theta."""
    Q = _l2norm(queries.astype(np.float32))
    R = _l2norm(store.vectors.astype(np.float32))
    S = Q @ R.T
    top1 = S.max(axis=1)
    return float((top1 >= theta).mean())


def cluster_level_recall(
    queries: np.ndarray,
    query_cluster_ids: np.ndarray,
    store: CompressedStore,
    k: int = 5,
) -> float:
    """Fraction of queries for which the true cluster appears in top-k."""
    Q = _l2norm(queries.astype(np.float32))
    R = _l2norm(store.vectors.astype(np.float32))
    S = Q @ R.T
    k = min(k, R.shape[0])
    topk = np.argpartition(-S, kth=k - 1, axis=1)[:, :k]
    # Gather the predicted clusters for top-k.
    pred_clusters = store.cluster_ids[topk]  # (q, k)
    hit = (pred_clusters == query_cluster_ids[:, None]).any(axis=1)
    return float(hit.mean())


def recall_at_k(
    queries: np.ndarray,
    query_cluster_ids: np.ndarray,
    store: CompressedStore,
    ks: list[int] | tuple[int, ...] = (1, 10, 100),
) -> dict:
    """Recall@k for identity (true cluster appears in top-k retrieved reps).

    Returns dict {recall@1, recall@10, recall@100, ...}.
    """
    Q = _l2norm(queries.astype(np.float32))
    R = _l2norm(store.vectors.astype(np.float32))
    S = Q @ R.T  # (q, m)
    # Sort in descending similarity
    order = np.argsort(-S, axis=1)
    m = R.shape[0]
    out = {}
    for k in ks:
        kk = min(k, m)
        topk = order[:, :kk]
        pred_clusters = store.cluster_ids[topk]
        hit = (pred_clusters == query_cluster_ids[:, None]).any(axis=1)
        out[f"recall@{k}"] = float(hit.mean())
    return out


def mrr_at_k(
    queries: np.ndarray,
    query_cluster_ids: np.ndarray,
    store: CompressedStore,
    k: int = 20,
) -> float:
    """Mean Reciprocal Rank over top-k retrieved reps.

    For each query, rank is the position (1-indexed) of the FIRST rep whose
    cluster_id matches the query's true cluster; if not found in top-k, 1/rank = 0.
    """
    Q = _l2norm(queries.astype(np.float32))
    R = _l2norm(store.vectors.astype(np.float32))
    S = Q @ R.T
    m = R.shape[0]
    kk = min(k, m)
    order = np.argsort(-S, axis=1)[:, :kk]  # (q, kk)
    pred_clusters = store.cluster_ids[order]  # (q, kk)
    match = pred_clusters == query_cluster_ids[:, None]  # (q, kk)
    # First hit position per query
    rr = np.zeros(queries.shape[0], dtype=np.float32)
    any_hit = match.any(axis=1)
    first = np.argmax(match, axis=1)  # index of first True (0 if none; guarded by any_hit)
    rr[any_hit] = 1.0 / (first[any_hit] + 1.0)
    return float(rr.mean())
