"""
Build ArXiv titles corpus, labels by primary category.

Uses ccdv/arxiv-classification (HF parquet release) which provides abstract
text + integer class label directly. We use the first sentence of the abstract
as a proxy for "title".
"""
from __future__ import annotations

import numpy as np

from data._common import Artifact, embed_texts, save_artifact


def build(
    model: str = "bge-large",
    n_papers: int = 30_000,
    min_cat_size: int = 50,
) -> Artifact:
    try:
        from datasets import load_dataset
    except ImportError as e:  # pragma: no cover
        raise ImportError("pip install datasets") from e

    ds = load_dataset(
        "ccdv/arxiv-classification",
        "no_ref",
        split=f"train[:{n_papers}]",
        trust_remote_code=True,
    )
    # ccdv schema: 'text' (full abstract), 'label' (int).
    texts: list[str] = []
    labels_int: list[int] = []
    for row in ds:
        text = row.get("text") or ""
        lab = row.get("label")
        if not text.strip() or lab is None:
            continue
        # Use first ~250 chars = ~title + first sentence.
        snippet = text.strip()[:500]
        texts.append(snippet)
        labels_int.append(int(lab))

    # Filter rare classes.
    from collections import Counter

    counts = Counter(labels_int)
    keep = {k for k, v in counts.items() if v >= min_cat_size}
    pairs = [(t, l) for t, l in zip(texts, labels_int) if l in keep]
    texts = [t for t, _ in pairs]
    labels_int = [l for _, l in pairs]

    # Remap labels to consecutive 0..K-1.
    unique = sorted(set(labels_int))
    lab2id = {l: i for i, l in enumerate(unique)}
    labels = np.asarray([lab2id[l] for l in labels_int], dtype=np.int64)

    X = embed_texts(texts, model=model)
    return save_artifact(
        corpus="arxiv_titles",
        model=model,
        X=X,
        labels_gold=labels,
        ids=[f"arxiv::{i}" for i in range(len(texts))],
        meta_extra={
            "source": "ccdv/arxiv-classification",
            "n_categories": len(unique),
            "min_cat_size": min_cat_size,
        },
    )


if __name__ == "__main__":  # pragma: no cover
    build()
