"""
Consolidation strategies: centroid, medoid, importance-weighted, selective pruning,
and GAC (Geometry-Aware Consolidation — the new one).

All strategies share the same Consolidator interface:

    class Consolidator:
        def fit_transform(X, labels) -> CompressedStore: ...

A CompressedStore contains the compressed vectors plus a mapping from each
representative back to the cluster id and (if known) the source member ids.
All vectors are L2-normalized on output.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal

import numpy as np

from gac.theory import cluster_spread, rho_cluster

# ---------------------------------------------------------------------------
# data container
# ---------------------------------------------------------------------------


@dataclass
class CompressedStore:
    """
    The output of any consolidation strategy.

    vectors         : (m, d) L2-normalised representatives
    cluster_ids     : (m,) int, which original cluster each representative belongs to
    source_ids      : (m,) int or -1, the index of the source embedding if this
                      representative IS one of the originals (medoid, prune, GAC
                      residuals with track_source=True). -1 if it is synthetic
                      (centroid, importance-weighted, residual directions).
    origin          : (m,) str, one of {"centroid","medoid","iw","prune",
                                        "gac_centroid","gac_medoid",
                                        "gac_residual","gac_prune"}
    meta            : dict of strategy-level info (compression ratio,
                      per-cluster routing decisions, timing).
    """

    vectors: np.ndarray
    cluster_ids: np.ndarray
    source_ids: np.ndarray
    origin: np.ndarray
    meta: dict = field(default_factory=dict)

    def __post_init__(self) -> None:
        m, _ = self.vectors.shape
        assert self.cluster_ids.shape == (m,)
        assert self.source_ids.shape == (m,)
        assert self.origin.shape == (m,)

    @property
    def n_representatives(self) -> int:
        return self.vectors.shape[0]


# ---------------------------------------------------------------------------
# helpers
# ---------------------------------------------------------------------------


def _l2norm(X: np.ndarray, eps: float = 1e-12) -> np.ndarray:
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + eps)


def _iter_clusters(X: np.ndarray, labels: np.ndarray):
    """Yield (label, member_indices, X_cluster) for every non-noise cluster."""
    for lab in np.unique(labels):
        if lab < 0:  # HDBSCAN noise
            continue
        idx = np.flatnonzero(labels == lab)
        yield int(lab), idx, X[idx]


# ---------------------------------------------------------------------------
# centroid
# ---------------------------------------------------------------------------


class CentroidConsolidator:
    """Replace each cluster with the L2-normalised mean of its members."""

    def fit_transform(self, X: np.ndarray, labels: np.ndarray) -> CompressedStore:
        vecs, cids, sids, orig = [], [], [], []
        for lab, _, Xc in _iter_clusters(X, labels):
            vecs.append(Xc.mean(axis=0))
            cids.append(lab)
            sids.append(-1)
            orig.append("centroid")
        V = _l2norm(np.asarray(vecs, dtype=np.float32))
        return CompressedStore(
            vectors=V,
            cluster_ids=np.asarray(cids, dtype=np.int64),
            source_ids=np.asarray(sids, dtype=np.int64),
            origin=np.asarray(orig, dtype=object),
            meta={"strategy": "centroid", "compression": X.shape[0] / V.shape[0]},
        )


# ---------------------------------------------------------------------------
# medoid
# ---------------------------------------------------------------------------


class MedoidConsolidator:
    """Select the cluster member with maximum cosine similarity to the centroid."""

    def fit_transform(self, X: np.ndarray, labels: np.ndarray) -> CompressedStore:
        Xn = _l2norm(X.astype(np.float32))
        vecs, cids, sids, orig = [], [], [], []
        for lab, idx, Xc in _iter_clusters(Xn, labels):
            c = Xc.mean(axis=0)
            c = c / (np.linalg.norm(c) + 1e-12)
            sims = Xc @ c
            best = int(np.argmax(sims))
            vecs.append(Xc[best])
            cids.append(lab)
            sids.append(int(idx[best]))
            orig.append("medoid")
        V = np.asarray(vecs, dtype=np.float32)
        return CompressedStore(
            vectors=V,
            cluster_ids=np.asarray(cids, dtype=np.int64),
            source_ids=np.asarray(sids, dtype=np.int64),
            origin=np.asarray(orig, dtype=object),
            meta={"strategy": "medoid", "compression": X.shape[0] / V.shape[0]},
        )


# ---------------------------------------------------------------------------
# importance-weighted
# ---------------------------------------------------------------------------


class ImportanceWeightedConsolidator:
    """
    Weighted centroid where weights are inverse mean within-cluster similarity
    (boundary / distinctive members get larger weight).

    Provably degenerate to uniform on isotropic clusters (Corollary 3 of the
    paper) -- included as a baseline.
    """

    def fit_transform(self, X: np.ndarray, labels: np.ndarray) -> CompressedStore:
        Xn = _l2norm(X.astype(np.float32))
        vecs, cids, sids, orig = [], [], [], []
        for lab, _, Xc in _iter_clusters(Xn, labels):
            n = Xc.shape[0]
            if n == 1:
                vecs.append(Xc[0])
            else:
                sims = Xc @ Xc.T
                mean_sim = (sims.sum(axis=1) - np.diag(sims)) / (n - 1)
                w = 1.0 / (mean_sim + 1e-6)
                w = w / w.sum()
                v = (Xc * w[:, None]).sum(axis=0)
                vecs.append(v)
            cids.append(lab)
            sids.append(-1)
            orig.append("iw")
        V = _l2norm(np.asarray(vecs, dtype=np.float32))
        return CompressedStore(
            vectors=V,
            cluster_ids=np.asarray(cids, dtype=np.int64),
            source_ids=np.asarray(sids, dtype=np.int64),
            origin=np.asarray(orig, dtype=object),
            meta={"strategy": "importance_weighted", "compression": X.shape[0] / V.shape[0]},
        )


# ---------------------------------------------------------------------------
# selective pruning
# ---------------------------------------------------------------------------


class SelectivePruningConsolidator:
    """
    Rank members by distinctiveness (inverse mean peer similarity) and keep
    the top `keep_ratio` fraction. Produces a strict subset of real vectors;
    compression ~ 1/keep_ratio.
    """

    def __init__(self, keep_ratio: float = 0.5):
        if not 0 < keep_ratio <= 1:
            raise ValueError("keep_ratio must be in (0, 1]")
        self.keep_ratio = keep_ratio

    def fit_transform(self, X: np.ndarray, labels: np.ndarray) -> CompressedStore:
        Xn = _l2norm(X.astype(np.float32))
        vecs, cids, sids, orig = [], [], [], []
        for lab, idx, Xc in _iter_clusters(Xn, labels):
            n = Xc.shape[0]
            k = max(1, int(np.ceil(n * self.keep_ratio)))
            if k >= n:
                keep = np.arange(n)
            else:
                sims = Xc @ Xc.T
                mean_sim = (sims.sum(axis=1) - np.diag(sims)) / (n - 1)
                keep = np.argsort(mean_sim)[:k]  # most distinct = lowest mean sim
            for j in keep:
                vecs.append(Xc[j])
                cids.append(lab)
                sids.append(int(idx[j]))
                orig.append("prune")
        V = np.asarray(vecs, dtype=np.float32)
        return CompressedStore(
            vectors=V,
            cluster_ids=np.asarray(cids, dtype=np.int64),
            source_ids=np.asarray(sids, dtype=np.int64),
            origin=np.asarray(orig, dtype=object),
            meta={
                "strategy": "selective_pruning",
                "keep_ratio": self.keep_ratio,
                "compression": X.shape[0] / V.shape[0],
            },
        )


# ---------------------------------------------------------------------------
# GAC  --- the new algorithm
# ---------------------------------------------------------------------------


class GACConsolidator:
    """
    Geometry-Aware Consolidation.

    For each cluster, compute its spectral concentration rho and cosine spread.
    Route to one of three per-cluster operators:

      - rho > tau_high AND spread < spread_safe  -> centroid  (dense, safe)
      - spread > spread_unsafe                   -> top-p prune (diverse, unsafe)
      - otherwise                                -> medoid + top-r residual directions

    Thresholds derive from the Consolidation-Interference bound (§2 of the
    paper), parameterised by the retrieval threshold `theta` and the
    global effective dimension `d_eff_global`.
    """

    def __init__(
        self,
        theta: float = 0.8,
        d_eff_global: float = 16.0,
        tau_high: float = 0.55,
        residual_rank: int = 3,
        prune_ratio: float = 0.5,
        safe_mult: float = 0.75,
        unsafe_mult: float = 1.25,
        mode: Literal["auto", "fixed"] = "auto",
    ):
        self.theta = float(theta)
        self.d_eff_global = float(d_eff_global)
        self.tau_high = float(tau_high)
        self.residual_rank = int(residual_rank)
        self.prune_ratio = float(prune_ratio)
        self.safe_mult = float(safe_mult)
        self.unsafe_mult = float(unsafe_mult)
        self.mode = mode

    def _thresholds(self) -> tuple[float, float]:
        """Derive (spread_safe, spread_unsafe) from theta and d_eff."""
        # At spread == d_bar_critical, the bound predicts ~50% id error.
        # d_bar_critical = (1-theta) * 2^(1/d_eff).
        theta_prime = max(1e-3, 1.0 - self.theta)
        d_bar_critical = theta_prime * (2.0 ** (1.0 / max(self.d_eff_global, 1.0)))
        return (self.safe_mult * d_bar_critical, self.unsafe_mult * d_bar_critical)

    def fit_transform(self, X: np.ndarray, labels: np.ndarray) -> CompressedStore:
        Xn = _l2norm(X.astype(np.float32))
        spread_safe, spread_unsafe = self._thresholds()

        vecs, cids, sids, orig = [], [], [], []
        routing = {"centroid": 0, "medoid+residual": 0, "prune": 0}

        for lab, idx, Xc in _iter_clusters(Xn, labels):
            n = Xc.shape[0]
            if n == 1:
                vecs.append(Xc[0])
                cids.append(lab)
                sids.append(int(idx[0]))
                orig.append("gac_medoid")
                routing["medoid+residual"] += 1
                continue

            spread = cluster_spread(Xc, normalize=False)
            rho = rho_cluster(Xc)

            if rho > self.tau_high and spread < spread_safe:
                # Dense cluster -- safe to collapse.
                v = Xc.mean(axis=0)
                v = v / (np.linalg.norm(v) + 1e-12)
                vecs.append(v)
                cids.append(lab)
                sids.append(-1)
                orig.append("gac_centroid")
                routing["centroid"] += 1

            elif spread > spread_unsafe:
                # Diverse cluster -- prune, keep real vectors.
                k = max(1, int(np.ceil(n * self.prune_ratio)))
                sims = Xc @ Xc.T
                mean_sim = (sims.sum(axis=1) - np.diag(sims)) / (n - 1)
                keep = np.argsort(mean_sim)[:k]
                for j in keep:
                    vecs.append(Xc[j])
                    cids.append(lab)
                    sids.append(int(idx[j]))
                    orig.append("gac_prune")
                routing["prune"] += 1

            else:
                # Borderline cluster -- medoid plus top-r residual directions.
                c = Xc.mean(axis=0)
                c = c / (np.linalg.norm(c) + 1e-12)
                sims = Xc @ c
                best = int(np.argmax(sims))
                vecs.append(Xc[best])
                cids.append(lab)
                sids.append(int(idx[best]))
                orig.append("gac_medoid")

                # Residual directions: top-r PCs of the centered cluster, scaled
                # to the median cluster magnitude (so they are real retrieval
                # keys, not arbitrary unit vectors).
                Xc_centered = Xc - c[None, :]
                # Economy SVD on (n, d) or its Gram depending on shape.
                r = min(self.residual_rank, n - 1)
                if r > 0:
                    if n <= Xc.shape[1]:
                        G = Xc_centered @ Xc_centered.T
                        w, V = np.linalg.eigh(G)
                        # Top r eigenvectors in the sample space.
                        order = np.argsort(w)[::-1][:r]
                        w = np.clip(w[order], 0.0, None)
                        Vsamp = V[:, order]
                        # Lift to feature space: d_i = X^T v_i / sqrt(lambda_i)
                        d_dirs = Xc_centered.T @ Vsamp / (np.sqrt(w + 1e-12))
                    else:
                        U, s, Vt = np.linalg.svd(Xc_centered, full_matrices=False)
                        d_dirs = Vt[:r].T
                    d_dirs = _l2norm(d_dirs.T)  # (r, d) unit directions
                    # Anchor around the medoid: representative = medoid + eps * direction.
                    # Using magnitude 0.1 keeps them close to the medoid's cap but
                    # covers the orthogonal variance that the medoid alone misses.
                    eps = 0.1
                    anchored = _l2norm(Xc[best][None, :] + eps * d_dirs)
                    for v in anchored:
                        vecs.append(v)
                        cids.append(lab)
                        sids.append(-1)
                        orig.append("gac_residual")
                routing["medoid+residual"] += 1

        V = np.asarray(vecs, dtype=np.float32)
        return CompressedStore(
            vectors=V,
            cluster_ids=np.asarray(cids, dtype=np.int64),
            source_ids=np.asarray(sids, dtype=np.int64),
            origin=np.asarray(orig, dtype=object),
            meta={
                "strategy": "gac",
                "theta": self.theta,
                "d_eff_global": self.d_eff_global,
                "spread_safe": spread_safe,
                "spread_unsafe": spread_unsafe,
                "routing_counts": routing,
                "compression": X.shape[0] / V.shape[0],
            },
        )


# ---------------------------------------------------------------------------
# top-level dispatch
# ---------------------------------------------------------------------------


_STRATEGY_REGISTRY = {
    "centroid": lambda **kw: CentroidConsolidator(),
    "medoid": lambda **kw: MedoidConsolidator(),
    "importance_weighted": lambda **kw: ImportanceWeightedConsolidator(),
    "selective_prune": lambda keep_ratio=0.5, **kw: SelectivePruningConsolidator(keep_ratio),
    "gac": lambda **kw: GACConsolidator(**kw),
}


def consolidate(
    X: np.ndarray,
    labels: np.ndarray,
    strategy: str,
    **strategy_kwargs,
) -> CompressedStore:
    """Top-level dispatch. Strategy names: centroid, medoid, importance_weighted,
    selective_prune, gac."""
    if strategy not in _STRATEGY_REGISTRY:
        raise ValueError(
            f"Unknown strategy '{strategy}'. Options: {sorted(_STRATEGY_REGISTRY)}"
        )
    consolidator = _STRATEGY_REGISTRY[strategy](**strategy_kwargs)
    return consolidator.fit_transform(X, labels)
