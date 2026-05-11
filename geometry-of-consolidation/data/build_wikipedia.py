"""
Build Wikipedia-section corpus.

Uses HuggingFace datasets `wikipedia` (en dump). For each of the top N articles,
splits into sections by heading and treats each section as a cluster; each
sentence within the section is a member. Gold labels = section id.
"""
from __future__ import annotations

import re

import numpy as np

from data._common import Artifact, embed_texts, save_artifact

SECTION_RE = re.compile(r"\n==+\s*(.+?)\s*==+\n")
SENT_RE = re.compile(r"(?<=[.!?])\s+")


def _split_sections(text: str) -> list[tuple[str, str]]:
    """Return [(section_title, section_body), ...] including lead as 'Lead'."""
    parts = SECTION_RE.split(text)
    out = [("Lead", parts[0])]
    for i in range(1, len(parts) - 1, 2):
        out.append((parts[i], parts[i + 1]))
    return out


def build(
    model: str = "bge-large",
    n_articles: int = 200,
    min_section_tokens: int = 40,
    min_members: int = 5,
    max_sentences_per_section: int = 60,
    corpus_name: str = "wikipedia_sections",
    seed: int = 0,
) -> Artifact:
    try:
        from datasets import load_dataset
    except ImportError as e:  # pragma: no cover
        raise ImportError("pip install datasets") from e

    # Legacy 'wikipedia' script was deprecated in HF datasets>=3.0.
    # Use the maintained 'wikimedia/wikipedia' parquet release instead.
    ds = load_dataset(
        "wikimedia/wikipedia", "20231101.en", split=f"train[:{n_articles}]"
    )
    rng = np.random.default_rng(seed)

    texts, labels, ids = [], [], []
    cid = 0
    for row in ds:
        art_text = row["text"]
        art_title = row["title"]
        for sec_title, body in _split_sections(art_text):
            if len(body.split()) < min_section_tokens:
                continue
            sents = [s.strip() for s in SENT_RE.split(body) if 20 <= len(s.strip()) <= 400]
            if len(sents) < min_members:
                continue
            if len(sents) > max_sentences_per_section:
                sents = list(rng.choice(sents, size=max_sentences_per_section, replace=False))
            for j, s in enumerate(sents):
                texts.append(s)
                labels.append(cid)
                ids.append(f"{art_title}::{sec_title}::{j}")
            cid += 1

    labels_arr = np.asarray(labels, dtype=np.int64)
    X = embed_texts(texts, model=model)
    return save_artifact(
        corpus=corpus_name,
        model=model,
        X=X,
        labels_gold=labels_arr,
        ids=ids,
        meta_extra={
            "source": "huggingface wikipedia 20220301.en",
            "n_articles": n_articles,
            "n_sections": int(labels_arr.max() + 1) if len(labels_arr) else 0,
            "min_section_tokens": min_section_tokens,
            "min_members": min_members,
        },
    )


if __name__ == "__main__":  # pragma: no cover
    build()
