"""
Build a 'random-text' corpus from C4 for efficiency / unlabeled tests.

C4 has no natural cluster labels. We emit labels_gold=None and let E2's
clusterer handle labeling online.
"""
from __future__ import annotations

from data._common import Artifact, embed_texts, save_artifact


def build(
    model: str = "minilm",
    n_docs: int = 20_000,
    max_chars: int = 1000,
) -> Artifact:
    try:
        from datasets import load_dataset
    except ImportError as e:  # pragma: no cover
        raise ImportError("pip install datasets") from e
    # Legacy 'c4' script was removed; use allenai/c4 parquet mirror in
    # STREAMING mode so we don't download all 1024 shards (terabytes). We
    # iterate the stream and stop once we have n_docs rows.
    ds = load_dataset(
        "allenai/c4", "en", split="train", streaming=True
    )
    texts: list[str] = []
    for row in ds:
        txt = (row.get("text") or "")[:max_chars]
        if txt:
            texts.append(txt)
        if len(texts) >= n_docs:
            break
    X = embed_texts(texts, model=model)
    return save_artifact(
        corpus="c4_random",
        model=model,
        X=X,
        labels_gold=None,
        ids=[f"c4::{i}" for i in range(len(texts))],
        meta_extra={"source": "c4.en", "n_docs": n_docs, "max_chars": max_chars},
    )


if __name__ == "__main__":  # pragma: no cover
    build()
