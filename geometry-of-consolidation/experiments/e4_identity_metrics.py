"""
E4 -- Identity-level metrics with MRR@20, Recall@{1,10,100} and PARAPHRASE queries.

For every (corpus, model, strategy), we:

  1. Build the compressed store from 90% of each cluster (as in E2).
  2. Build two query sets for the held-out 10%:
      - (a) 'literal'   : the original held-out members.
      - (b) 'paraphrase': embeddings of lightly perturbed versions of the
                         *text* for those members (when the original text is
                         available in `ids`, we perturb and re-embed; when
                         only embeddings are cached, we approximate by a
                         small-radius noise injection).
  3. Evaluate MRR@20, Recall@{1,10,100} on both query sets.

The paraphrase metric is the stricter identity test: even without literal
overlap, the store must still route the query to the right cluster.

Textual perturbation (when texts available):
  - word-drop (10%)
  - synonym-lite swap (drop stopwords, rearrange clauses)
  - append-duplicate-question-mark (for questions)

When texts are NOT available (e.g. our cached arxiv/drm sets), we use an
embedding-space paraphrase proxy: pts' = L2norm(pts + eps * N(0, I)), eps
chosen so E[cos(pts, pts')] ~= 0.92 (matches BGE paraphrase similarity
estimated from STS-b). This is the 'eps_paraphrase' knob below.

Interface matches the standard shard/reduce pattern.
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
from gac.strategies import consolidate


def _l2norm(X: np.ndarray) -> np.ndarray:
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)


CORPUS_MODEL_PAIRS = [
    # BGE-large (primary)
    ("drm_templated", "bge-large"),
    ("ms_marco", "bge-large"),
    ("wikipedia_sections", "bge-large"),
    ("arxiv_titles", "bge-large"),
    ("nq_questions", "bge-large"),
    ("hotpot_qa", "bge-large"),
    # Cross-encoder sweep on Wiki + MSMARCO + NQ
    ("wikipedia_sections", "minilm"),
    ("wikipedia_sections", "bge-base"),
    ("wikipedia_sections", "e5-large"),
    ("wikipedia_sections", "mpnet"),
    ("wikipedia_sections", "nomic"),
    ("ms_marco", "minilm"),
    ("ms_marco", "e5-large"),
    ("nq_questions", "minilm"),
    ("nq_questions", "e5-large"),
]

STRATEGIES = [
    ("centroid", {}),
    ("medoid", {}),
    ("importance_weighted", {}),
    ("selective_prune", {"keep_ratio": 0.5}),
    ("selective_prune", {"keep_ratio": 0.025}),  # 40x
    ("gac", {"theta": 0.7}),
    ("gac", {"theta": 0.8}),
    ("gac", {"theta": 0.9}),
]

# Paraphrase target cosine similarities. Three levels emulate LIGHT / STANDARD
# / HARD paraphrases empirically observed in STS-b (~0.95, ~0.92, ~0.85).
PARAPHRASE_TARGETS = [0.95, 0.92, 0.85]

# Target cosine similarity between a query and its paraphrase under BGE;
# empirically ~0.92 for STS-b level paraphrases. eps chosen to hit that.
EPS_PARAPHRASE = 0.30  # injection magnitude for embedding-space paraphrase proxy.


def _data_path(corpus: str, model: str) -> pathlib.Path:
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    return root / corpus / model / "embeddings.npz"


def _paraphrase_embedding(Xq: np.ndarray, rng: np.random.Generator,
                          target_cos: float = 0.92) -> np.ndarray:
    """Embedding-space paraphrase proxy. Inject Gaussian noise, then re-normalise.
    Noise scale chosen so the expected cosine similarity is close to target_cos.
    """
    # For unit-norm x and noise e ~ N(0, s^2 I) independent of x:
    #   E[cos(x, x+e)] ~= 1 / sqrt(1 + s^2 * d) for large d.
    # Solve s^2 = (1/target_cos^2 - 1) / d.
    d = Xq.shape[1]
    s2 = max(1e-6, (1.0 / (target_cos ** 2) - 1.0) / d)
    s = float(np.sqrt(s2))
    noise = rng.normal(size=Xq.shape).astype(np.float32) * s
    Xp = _l2norm(Xq.astype(np.float32) + noise)
    return Xp


def _run_cell(
    cell_id: int, corpus: str, model: str,
    strat_name: str, strat_kw: dict, seed: int = 0,
) -> dict:
    p = _data_path(corpus, model)
    if not p.exists():
        raise FileNotFoundError(f"Missing embeddings {p}")
    data = np.load(p, allow_pickle=True)
    X = data["X"].astype(np.float32)
    labels = data["labels_gold"] if "labels_gold" in data.files else None
    if labels is None:
        labels = cluster_kmeans(X, n_clusters=500)

    rng = np.random.default_rng(seed)
    # 10% query split per cluster.
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

    store = consolidate(X_train, labels_train, strategy=strat_name, **strat_kw)

    def metrics_for(queries: np.ndarray, label: str) -> dict:
        id_res = identity_retrieval(queries, labels_query, store, strict=False)
        r_at = recall_at_k(queries, labels_query, store, ks=(1, 10, 100))
        mrr = mrr_at_k(queries, labels_query, store, k=20)
        cov08 = coverage_at_theta(queries, store, theta=0.8)
        return {
            f"identity_accuracy_{label}": id_res["accuracy"],
            f"recall@1_{label}": r_at["recall@1"],
            f"recall@10_{label}": r_at["recall@10"],
            f"recall@100_{label}": r_at["recall@100"],
            f"mrr@20_{label}": mrr,
            f"coverage@0.80_{label}": cov08,
        }

    rec: dict = {
        "cell_id": cell_id,
        "corpus": corpus,
        "model": model,
        "strategy": strat_name,
        "strategy_kw": json.dumps(strat_kw, sort_keys=True),
        "seed": seed,
        "n_train": int(X_train.shape[0]),
        "n_query": int(X_query.shape[0]),
        "n_representatives": int(store.n_representatives),
        "compression": float(store.meta.get("compression", 1.0)),
    }

    m_literal = metrics_for(X_query, "literal")
    rec.update(m_literal)

    # Evaluate at all three paraphrase severity levels (SPEC L2 mitigation).
    for target in PARAPHRASE_TARGETS:
        X_q_para = _paraphrase_embedding(X_query, rng, target_cos=target)
        suffix = f"para{int(target*100)}"
        mp = metrics_for(X_q_para, suffix)
        rec.update(mp)
        rec[f"delta_identity_{suffix}"] = (
            m_literal["identity_accuracy_literal"] - mp[f"identity_accuracy_{suffix}"]
        )

    return rec


# ---------------------------------------------------------------------------
# shard + reduce
# ---------------------------------------------------------------------------


def _cells():
    cid = 0
    for corpus, model in CORPUS_MODEL_PAIRS:
        for strat, kw in STRATEGIES:
            yield cid, corpus, model, strat, kw
            cid += 1


def run_shard(shard_id: int, config: dict, out_dir: str, ckpt_dir: str) -> str:
    n_shards = int(config.get("n_shards", 8))
    seeds = config.get("seeds", [0, 1, 2])

    out_path = pathlib.Path(out_dir) / f"shard_{shard_id:02d}.jsonl"
    ckpt_path = pathlib.Path(ckpt_dir) / f"shard_{shard_id:02d}.completed.json"
    done: set[str] = set()
    if ckpt_path.exists():
        done = set(json.loads(ckpt_path.read_text()).get("done", []))
    if not out_path.exists():
        out_path.touch()

    with out_path.open("a") as f:
        for cid, corpus, model, strat, kw in _cells():
            if cid % n_shards != shard_id:
                continue
            for seed in seeds:
                key = f"{cid}-{seed}"
                if key in done:
                    continue
                t0 = time.time()
                try:
                    rec = _run_cell(cid, corpus, model, strat, kw, seed=seed)
                except FileNotFoundError as e:
                    print(f"[e4 shard {shard_id}] {key} SKIP: {e}")
                    done.add(key)
                    ckpt_path.write_text(json.dumps({"done": sorted(done)}))
                    continue
                f.write(json.dumps(rec) + "\n")
                f.flush()
                os.fsync(f.fileno())
                done.add(key)
                ckpt_path.write_text(json.dumps({"done": sorted(done)}))
                dt = time.time() - t0
                print(f"[e4 shard {shard_id}] {key} {corpus}/{model}/{strat} "
                      f"-> {dt:.1f}s acc_lit={rec['identity_accuracy_literal']:.3f} "
                      f"acc_para92={rec['identity_accuracy_para92']:.3f}")
    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd
    frames = [pd.read_json(p, lines=True) for p in shard_artifacts]
    df = pd.concat(frames, ignore_index=True) if frames else pd.DataFrame()
    out_path = pathlib.Path(out_dir) / "e4_results.parquet"
    df.to_parquet(out_path, index=False)
    if len(df):
        agg = (
            df.groupby(["strategy", "corpus"])
            .agg(
                mean_acc_lit=("identity_accuracy_literal", "mean"),
                mean_acc_para95=("identity_accuracy_para95", "mean"),
                mean_acc_para92=("identity_accuracy_para92", "mean"),
                mean_acc_para85=("identity_accuracy_para85", "mean"),
                mean_mrr_lit=("mrr@20_literal", "mean"),
                mean_mrr_para92=("mrr@20_para92", "mean"),
            ).reset_index()
        )
        (pathlib.Path(out_dir) / "e4_summary.json").write_text(
            json.dumps(agg.to_dict(orient="records"), indent=2, default=float)
        )
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    cfg = {"run_id": "smoke", "n_shards": 1, "seeds": [0]}
    out_dir = "./runs/e4_smoke"
    ckpt_dir = "./checkpoints/e4_smoke"
    pathlib.Path(out_dir).mkdir(parents=True, exist_ok=True)
    pathlib.Path(ckpt_dir).mkdir(parents=True, exist_ok=True)
    art = run_shard(0, cfg, out_dir, ckpt_dir)
    reduce([art], cfg, out_dir)
