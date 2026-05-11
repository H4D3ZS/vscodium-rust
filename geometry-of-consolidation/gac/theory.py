"""
Theoretical quantities from the Consolidation-Interference Duality theorem.

These are the numerical tools that turn the theorem (§2 of the paper) into
something you can compute on an actual embedding store.
"""
from __future__ import annotations

import numpy as np


def d_eff(X: np.ndarray, method: str = "participation_ratio") -> float:
    """
    Effective dimensionality of an embedding matrix.

    Three estimators are supported (cf. No-Escape paper, Dimensionality section):
      - 'participation_ratio' : (sum lambda_i)^2 / sum lambda_i^2  [default]
      - 'pca_95'              : # PCs needed for 95% explained variance
      - 'levina_bickel'       : local-intrinsic-dimension estimator via k-NN

    Inputs
    ------
    X : (n, d) array of embeddings (not required to be L2-normalized)

    Returns
    -------
    float : effective dimensionality
    """
    if X.ndim != 2:
        raise ValueError(f"X must be 2D, got shape {X.shape}")
    n, d = X.shape
    Xc = X - X.mean(axis=0, keepdims=True)

    if method == "participation_ratio":
        # Participation ratio on the covariance spectrum.
        # For (n, d) with n possibly < d, use the Gram / n to get a d x d cov.
        cov = (Xc.T @ Xc) / max(n - 1, 1)
        eigs = np.linalg.eigvalsh(cov)
        eigs = np.clip(eigs, 0.0, None)
        s = eigs.sum()
        if s <= 0:
            return 0.0
        return float((s ** 2) / (eigs ** 2).sum())

    if method == "pca_95":
        cov = (Xc.T @ Xc) / max(n - 1, 1)
        eigs = np.linalg.eigvalsh(cov)[::-1]
        eigs = np.clip(eigs, 0.0, None)
        cum = np.cumsum(eigs) / max(eigs.sum(), 1e-30)
        k = int(np.searchsorted(cum, 0.95) + 1)
        return float(k)

    if method == "levina_bickel":
        # MLE intrinsic-dimension estimator (Levina & Bickel 2004) using k=10.
        from sklearn.neighbors import NearestNeighbors

        k = min(10, n - 1)
        if k < 2:
            return float(d)
        nn = NearestNeighbors(n_neighbors=k + 1).fit(X)
        dists, _ = nn.kneighbors(X)
        # Drop the self-distance (first column).
        r = dists[:, 1:]
        # Avoid log(0).
        eps = 1e-12
        ratios = np.log(np.maximum(r[:, -1:], eps) / np.maximum(r[:, :-1], eps))
        m_hat = 1.0 / (ratios.mean(axis=1) + eps)
        return float(np.mean(m_hat))

    raise ValueError(f"Unknown d_eff method: {method}")


def spectral_bound(
    d_bar: float,
    theta: float,
    d_eff_val: float,
    c1: float = 1.0,
) -> float:
    """
    The Consolidation-Interference lower bound on identity-retrieval error.

    Given a cluster with mean within-cluster cosine distance `d_bar`,
    a retrieval threshold `theta` (as cosine similarity), and the local
    effective dimension `d_eff`, returns the theoretical lower bound on
    the fraction of cluster members that *cannot* be identity-retrieved
    from any single representative.

    Derivation (Theorem §2.1 of the paper, informal):
        eps_id(C, r) >= 1 - c1 * (theta' / d_bar)^d_eff

    where theta' = (1 - theta) is the angular cap radius. For theta' >= d_bar
    the cluster is entirely within the cap and the bound saturates at 0
    (i.e. no error forced).
    """
    if d_bar <= 0:
        return 0.0
    theta_prime = max(0.0, 1.0 - theta)
    if theta_prime >= d_bar:
        return 0.0  # whole cluster fits inside the cap
    ratio = theta_prime / d_bar
    # Saturate to [0, 1].
    return float(max(0.0, 1.0 - c1 * (ratio ** d_eff_val)))


def cluster_spread(X_cluster: np.ndarray, normalize: bool = True) -> float:
    """Mean pairwise cosine distance within a cluster."""
    if X_cluster.shape[0] < 2:
        return 0.0
    X = X_cluster
    if normalize:
        X = X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)
    # Compute without materialising the full n^2 matrix for large clusters.
    n = X.shape[0]
    if n > 2000:
        # Subsample for tractability.
        rng = np.random.default_rng(0)
        idx = rng.choice(n, size=2000, replace=False)
        X = X[idx]
        n = 2000
    sims = X @ X.T
    mask = ~np.eye(n, dtype=bool)
    mean_sim = float(sims[mask].mean())
    return 1.0 - mean_sim


def rho_cluster(X_cluster: np.ndarray) -> float:
    """
    Spectral concentration ratio: top eigenvalue / sum of eigenvalues
    of the within-cluster covariance. Drives GAC's routing.

    rho → 1: cluster is near rank-1 (safe to collapse to centroid).
    rho → 1/d_eff: cluster is isotropic (representatives lose information).
    """
    n = X_cluster.shape[0]
    if n < 2:
        return 1.0
    Xc = X_cluster - X_cluster.mean(axis=0, keepdims=True)
    # Use Gram trick when n < d.
    if n <= X_cluster.shape[1]:
        G = (Xc @ Xc.T) / max(n - 1, 1)
        eigs = np.linalg.eigvalsh(G)
    else:
        C = (Xc.T @ Xc) / max(n - 1, 1)
        eigs = np.linalg.eigvalsh(C)
    eigs = np.clip(eigs, 0.0, None)
    s = eigs.sum()
    if s <= 0:
        return 1.0
    return float(eigs.max() / s)
