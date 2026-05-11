"""
GAC — Geometry-Aware Consolidation for semantic memory systems.

Public API:
    consolidate(embeddings, labels, strategy, **kwargs) -> CompressedStore
    GACConsolidator                                     -- the geometry-aware router
    CentroidConsolidator, MedoidConsolidator,
    ImportanceWeightedConsolidator, SelectivePruningConsolidator
    d_eff(X)                                            -- participation-ratio effective dim
    spectral_bound(d_bar, theta, d_eff)                 -- the Consolidation-Interference bound
"""
from gac.clustering import (
    cluster_hdbscan,
    cluster_kmeans,
    make_synthetic_clusters,
)
from gac.metrics import (
    cluster_level_recall,
    coverage_at_theta,
    identity_retrieval,
)
from gac.strategies import (
    CentroidConsolidator,
    CompressedStore,
    GACConsolidator,
    ImportanceWeightedConsolidator,
    MedoidConsolidator,
    SelectivePruningConsolidator,
    consolidate,
)
from gac.theory import cluster_spread, d_eff, rho_cluster, spectral_bound

__version__ = "0.1.0"

__all__ = [
    "consolidate",
    "CompressedStore",
    "GACConsolidator",
    "CentroidConsolidator",
    "MedoidConsolidator",
    "ImportanceWeightedConsolidator",
    "SelectivePruningConsolidator",
    "d_eff",
    "spectral_bound",
    "cluster_spread",
    "rho_cluster",
    "cluster_hdbscan",
    "cluster_kmeans",
    "make_synthetic_clusters",
    "identity_retrieval",
    "coverage_at_theta",
    "cluster_level_recall",
]
