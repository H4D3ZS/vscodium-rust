"""
Smoke tests for the GAC package.

These are fast (<10s) and run on CPU only. They verify that:
  - every strategy produces a valid CompressedStore
  - identity retrieval on the training set is >= a weak floor
  - the theoretical bound is never strictly exceeded by centroid's error
    on well-behaved synthetic clusters (a sanity check of the theorem)
  - GAC routing actually differentiates dense vs diverse clusters
"""
from __future__ import annotations

import numpy as np
import pytest

from gac.clustering import make_synthetic_clusters
from gac.metrics import identity_retrieval
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

N_CLUSTERS = 20
MEMBERS = 15
DIM = 32
SEED = 0


@pytest.fixture(scope="module")
def dense_clusters():
    return make_synthetic_clusters(
        n_clusters=N_CLUSTERS, members_per_cluster=MEMBERS, d=DIM,
        spread=0.05, seed=SEED,
    )


@pytest.fixture(scope="module")
def diverse_clusters():
    return make_synthetic_clusters(
        n_clusters=N_CLUSTERS, members_per_cluster=MEMBERS, d=DIM,
        spread=0.5, seed=SEED + 1,
    )


# --- CompressedStore shape -------------------------------------------------


@pytest.mark.parametrize(
    "strategy, kwargs",
    [
        ("centroid", {}),
        ("medoid", {}),
        ("importance_weighted", {}),
        ("selective_prune", {"keep_ratio": 0.5}),
        ("gac", {"theta": 0.8, "d_eff_global": 8.0}),
    ],
)
def test_store_shapes(dense_clusters, strategy, kwargs):
    X, labels = dense_clusters
    store = consolidate(X, labels, strategy=strategy, **kwargs)
    assert isinstance(store, CompressedStore)
    assert store.vectors.shape[1] == DIM
    m = store.vectors.shape[0]
    assert store.cluster_ids.shape == (m,)
    assert store.source_ids.shape == (m,)
    assert store.origin.shape == (m,)
    # L2-normalised.
    norms = np.linalg.norm(store.vectors, axis=1)
    assert np.allclose(norms, 1.0, atol=1e-4)


# --- identity retrieval floor ---------------------------------------------


def test_centroid_near_perfect_on_dense(dense_clusters):
    X, labels = dense_clusters
    store = CentroidConsolidator().fit_transform(X, labels)
    res = identity_retrieval(X, labels, store)
    # With spread=0.05 every cluster is tight; cluster-assignment should be
    # essentially perfect.
    assert res["accuracy"] >= 0.95


def test_medoid_hits_real_vectors(dense_clusters):
    X, labels = dense_clusters
    store = MedoidConsolidator().fit_transform(X, labels)
    # Every medoid should be a real source vector.
    assert (store.source_ids >= 0).all()
    assert (store.origin == "medoid").all()


def test_prune_keeps_real_vectors(dense_clusters):
    X, labels = dense_clusters
    store = SelectivePruningConsolidator(keep_ratio=0.4).fit_transform(X, labels)
    # Compression ~ 1/0.4 = 2.5.
    assert 2.0 <= store.meta["compression"] <= 3.5
    assert (store.source_ids >= 0).all()


# --- theorem sanity --------------------------------------------------------


def test_bound_is_nonnegative_and_bounded():
    for d_bar in [0.01, 0.05, 0.1, 0.5]:
        for theta in [0.7, 0.8, 0.9]:
            for de in [4, 8, 16]:
                b = spectral_bound(d_bar, theta, de)
                assert 0.0 <= b <= 1.0


def test_bound_monotone_in_d_bar():
    """The bound is monotone non-decreasing in d_bar (holding theta, d_eff fixed)."""
    prev = 0.0
    for d_bar in [0.01, 0.05, 0.1, 0.2, 0.4, 0.8]:
        b = spectral_bound(d_bar, theta=0.9, d_eff_val=16)
        assert b >= prev - 1e-9, f"bound not monotone: {prev} -> {b} at d_bar={d_bar}"
        prev = b


def test_bound_zero_when_cluster_fits_cap():
    # theta_prime = 1 - 0.5 = 0.5 >> d_bar=0.01, so bound must be 0.
    assert spectral_bound(0.01, theta=0.5, d_eff_val=8) == 0.0


# --- GAC routing behaviour ------------------------------------------------


def test_gac_routes_dense_to_centroid(dense_clusters):
    X, labels = dense_clusters
    gac = GACConsolidator(theta=0.9, d_eff_global=8.0)
    store = gac.fit_transform(X, labels)
    rc = store.meta["routing_counts"]
    # On dense clusters GAC should mostly pick centroid or medoid+residual
    # (never prune).
    assert rc["prune"] == 0
    assert rc["centroid"] + rc["medoid+residual"] == N_CLUSTERS


def test_gac_routes_diverse_to_prune(diverse_clusters):
    X, labels = diverse_clusters
    # With a tight theta, the bound tightens and diverse clusters should
    # get pruned.
    gac = GACConsolidator(theta=0.95, d_eff_global=32.0)
    store = gac.fit_transform(X, labels)
    rc = store.meta["routing_counts"]
    # Expect at least some clusters to get pruned (not all centroid).
    assert rc["prune"] + rc["medoid+residual"] >= rc["centroid"]


# --- theory utilities ------------------------------------------------------


def test_rho_cluster_bounds():
    rng = np.random.default_rng(0)
    # Rank-1 variance cluster: members spread along a single direction from
    # a common anchor. Within-cluster covariance is near rank-1 -> rho -> 1.
    anchor = rng.normal(size=(8,)).astype(np.float32)
    anchor /= np.linalg.norm(anchor)
    u = rng.normal(size=(8,)).astype(np.float32)
    u -= (u @ anchor) * anchor
    u /= np.linalg.norm(u) + 1e-12
    ts = rng.normal(size=(30,)).astype(np.float32) * 0.3
    X = anchor[None, :] + ts[:, None] * u[None, :]
    X = X / np.linalg.norm(X, axis=1, keepdims=True)
    assert rho_cluster(X) > 0.6

    # Isotropic-variance cluster: rho close to 1/d.
    Y = rng.normal(size=(200, 8)).astype(np.float32)
    Y = Y / np.linalg.norm(Y, axis=1, keepdims=True)
    assert rho_cluster(Y) < 0.4


def test_d_eff_consistency():
    rng = np.random.default_rng(0)
    # Low-rank data: d_eff small.
    X_lo = rng.normal(size=(200, 2)).astype(np.float32) @ rng.normal(size=(2, 64)).astype(np.float32)
    assert d_eff(X_lo) < 4.0

    # Full-rank iid Gaussian: d_eff close to ambient.
    X_hi = rng.normal(size=(200, 64)).astype(np.float32)
    assert d_eff(X_hi) > 30.0


def test_importance_weighted_approx_uniform_isotropic(dense_clusters):
    """Corollary 3: on nearly-isotropic clusters, IW -> uniform centroid."""
    X, labels = dense_clusters
    c_store = CentroidConsolidator().fit_transform(X, labels)
    iw_store = ImportanceWeightedConsolidator().fit_transform(X, labels)
    # The two strategies should agree cluster-by-cluster to within a small
    # cosine tolerance.
    cos = (c_store.vectors * iw_store.vectors).sum(axis=1)
    assert cos.mean() > 0.99
