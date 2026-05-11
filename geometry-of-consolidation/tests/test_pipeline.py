"""
Smoke test for the E1 sharded pipeline. Runs a single tiny cell end-to-end
without hitting Modal or the real embedding datasets.
"""
from __future__ import annotations

import json
import pathlib

import pytest


def test_e1_shard_runs(tmp_path):
    from experiments import e1_theorem_validation as e1

    out_dir = tmp_path / "out"
    ckpt_dir = tmp_path / "ckpt"
    out_dir.mkdir()
    ckpt_dir.mkdir()

    # Patch the sweep grid to something tiny.
    e1.D_EFF_GRID = [8]
    e1.THETA_GRID = [0.8]
    e1.DBAR_GRID = [0.01, 0.1]

    cfg = {"run_id": "smoke", "n_shards": 1, "n_seeds": 1,
           "n_clusters": 8, "members_per_cluster": 8}
    art = e1.run_shard(0, cfg, str(out_dir), str(ckpt_dir))
    assert pathlib.Path(art).exists()
    lines = pathlib.Path(art).read_text().strip().splitlines()
    assert len(lines) >= 1
    rec = json.loads(lines[0])
    assert "strategy" in rec and "error" in rec


def test_e1_resume(tmp_path):
    """Second invocation with the same checkpoint should not duplicate rows."""
    from experiments import e1_theorem_validation as e1

    e1.D_EFF_GRID = [8]
    e1.THETA_GRID = [0.8]
    e1.DBAR_GRID = [0.01]

    out_dir = tmp_path / "out"
    ckpt_dir = tmp_path / "ckpt"
    out_dir.mkdir()
    ckpt_dir.mkdir()
    cfg = {"run_id": "smoke", "n_shards": 1, "n_seeds": 1,
           "n_clusters": 4, "members_per_cluster": 6}

    art = e1.run_shard(0, cfg, str(out_dir), str(ckpt_dir))
    n_before = len(pathlib.Path(art).read_text().strip().splitlines())
    art2 = e1.run_shard(0, cfg, str(out_dir), str(ckpt_dir))
    n_after = len(pathlib.Path(art2).read_text().strip().splitlines())
    assert n_before == n_after, "resume duplicated rows"
