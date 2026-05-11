"""
Build MS MARCO corpus with gold cluster labels = query_id.

For each query we take its (up to K) associated passages and treat them
as a cluster. Provides a strong identity-retrieval testbed because within
a query, passages are paraphrastically related but not identical.
"""
from __future__ import annotations

import numpy as np

from data._common import Artifact, embed_texts, save_artifact


def build(
    model: str = "bge-large",
    n_queries: int = 2000,
    max_passages_per_query: int = 10,
    min_passages_per_query: int = 3,
) -> Artifact:
    try:
        from datasets import load_dataset
    except ImportError as e:  # pragma: no cover
        raise ImportError("pip install datasets") from e
    ds = load_dataset("ms_marco", "v2.1", split=f"train[:{n_queries * 4}]")

    labels, texts, ids = [], [], []
    qid_used = 0
    for row in ds:
        qid = row.get("query_id") or row.get("query")
        passages = row.get("passages", {}) or {}
        passage_texts = passages.get("passage_text", []) or []
        if len(passage_texts) < min_passages_per_query:
            continue
        pts = passage_texts[:max_passages_per_query]
        for j, p in enumerate(pts):
            texts.append(p)
            labels.append(qid_used)
            ids.append(f"q{qid}::p{j}")
        qid_used += 1
        if qid_used >= n_queries:
            break

    labels_arr = np.asarray(labels, dtype=np.int64)
    X = embed_texts(texts, model=model)
    return save_artifact(
        corpus="ms_marco",
        model=model,
        X=X,
        labels_gold=labels_arr,
        ids=ids,
        meta_extra={
            "source": "ms_marco v2.1",
            "n_queries": qid_used,
            "max_passages_per_query": max_passages_per_query,
        },
    )


if __name__ == "__main__":  # pragma: no cover
    build()
