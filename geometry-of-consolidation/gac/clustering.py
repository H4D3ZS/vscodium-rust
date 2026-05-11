"""
Clustering utilities for consolidation experiments.

Wrappers around HDBSCAN / KMeans that return the `(X, labels)` pair the
Consolidator API expects, plus some helpers for building synthetic mixtures.
"""
from __future__ import annotations

import numpy as np


def cluster_hdbscan(
    X: np.ndarray,
    min_cluster_size: int = 5,
    min_samples: int | None = None,
    metric: str = "euclidean",
) -> np.ndarray:
    """HDBSCAN on L2-normalised X. Returns int labels with -1 for noise."""
    try:
        import hdbscan  # type: ignore
    except ImportError as e:  # pragma: no cover
        raise ImportError(
            "hdbscan is required for cluster_hdbscan(); "
            "install via `pip install hdbscan`."
        ) from e
    Xn = X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)
    clusterer = hdbscan.HDBSCAN(
        min_cluster_size=min_cluster_size,
        min_samples=min_samples,
        metric=metric,
    )
    labels = clusterer.fit_predict(Xn.astype(np.float64))
    return labels.astype(np.int64)


def cluster_kmeans(X: np.ndarray, n_clusters: int, seed: int = 0) -> np.ndarray:
    """Spherical-ish K-means (cosine via L2-normalisation)."""
    from sklearn.cluster import KMeans

    Xn = X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)
    km = KMeans(n_clusters=n_clusters, random_state=seed, n_init=10)
    return km.fit_predict(Xn).astype(np.int64)


def make_synthetic_clusters(
    n_clusters: int = 50,
    members_per_cluster: int = 20,
    d: int = 64,
    spread: float = 0.1,
    seed: int = 0,
) -> tuple[np.ndarray, np.ndarray]:
    """
    Gaussian mixture on the sphere with controllable within-cluster spread.

    Returns (X, labels) where labels in [0, n_clusters).
    """
    rng = np.random.default_rng(seed)
    centers = rng.normal(size=(n_clusters, d)).astype(np.float32)
    centers /= np.linalg.norm(centers, axis=1, keepdims=True) + 1e-12
    X_list, lab_list = [], []
    for k in range(n_clusters):
        noise = rng.normal(scale=spread, size=(members_per_cluster, d)).astype(np.float32)
        pts = centers[k][None, :] + noise
        pts /= np.linalg.norm(pts, axis=1, keepdims=True) + 1e-12
        X_list.append(pts)
        lab_list.append(np.full(members_per_cluster, k, dtype=np.int64))
    return np.concatenate(X_list, axis=0), np.concatenate(lab_list, axis=0)
