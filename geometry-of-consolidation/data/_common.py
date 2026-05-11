"""
Shared helpers for building embedding-labeled corpora.

All data builders produce an `embeddings.npz` at
    $GAC_DATA_DIR/<corpus>/<model>/embeddings.npz
containing:
    X           : (n, d) float32 L2-normalised embeddings
    labels_gold : (n,) int64 cluster labels (-1 if unlabeled), optional
    ids         : (n,) object array of string ids (for provenance)
plus a `meta.json` with corpus- and model-level metadata.
"""
from __future__ import annotations

import json
import os
import pathlib
import time
from dataclasses import dataclass

import numpy as np

MODEL_ALIASES = {
    "bge-large": "BAAI/bge-large-en-v1.5",
    "bge-base": "BAAI/bge-base-en-v1.5",
    "bge-small": "BAAI/bge-small-en-v1.5",
    "minilm": "sentence-transformers/all-MiniLM-L6-v2",
    "mpnet": "sentence-transformers/all-mpnet-base-v2",
    "e5-large": "intfloat/e5-large-v2",
    "nomic": "nomic-ai/nomic-embed-text-v1.5",
}

# Some encoders require an instruction prefix (e.g. E5 needs "passage: ...").
MODEL_TEXT_PREFIX = {
    "e5-large": "passage: ",
    "nomic": "search_document: ",
}


@dataclass
class Artifact:
    path: pathlib.Path
    n: int
    d: int
    with_labels: bool


def root_dir() -> pathlib.Path:
    return pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))


def out_path(corpus: str, model: str) -> pathlib.Path:
    p = root_dir() / corpus / model
    p.mkdir(parents=True, exist_ok=True)
    return p


def embed_texts(
    texts: list[str],
    model: str,
    batch_size: int = 64,
    device: str | None = None,
) -> np.ndarray:
    """Embed via sentence-transformers and L2-normalise."""
    from sentence_transformers import SentenceTransformer

    model_name = MODEL_ALIASES.get(model, model)
    prefix = MODEL_TEXT_PREFIX.get(model, "")
    dev = device
    if dev is None:
        try:
            import torch

            dev = "cuda" if torch.cuda.is_available() else "cpu"
        except ImportError:
            dev = "cpu"
    # Nomic requires trust_remote_code=True.
    try:
        if "nomic" in model_name.lower():
            st = SentenceTransformer(model_name, device=dev, trust_remote_code=True)
        else:
            st = SentenceTransformer(model_name, device=dev)
    except TypeError:
        st = SentenceTransformer(model_name, device=dev)
    if prefix:
        texts = [prefix + t for t in texts]
    X = st.encode(
        texts,
        batch_size=batch_size,
        convert_to_numpy=True,
        show_progress_bar=True,
        normalize_embeddings=True,
    ).astype(np.float32)
    return X


def save_artifact(
    corpus: str,
    model: str,
    X: np.ndarray,
    labels_gold: np.ndarray | None,
    ids: list[str] | None,
    meta_extra: dict | None = None,
) -> Artifact:
    p = out_path(corpus, model)
    arrays: dict[str, np.ndarray] = {"X": X.astype(np.float32)}
    if labels_gold is not None:
        arrays["labels_gold"] = np.asarray(labels_gold, dtype=np.int64)
    if ids is not None:
        arrays["ids"] = np.asarray(ids, dtype=object)
    np.savez(p / "embeddings.npz", **arrays)
    meta = {
        "corpus": corpus,
        "model": model,
        "n": int(X.shape[0]),
        "d": int(X.shape[1]),
        "has_gold_labels": labels_gold is not None,
        "built_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        **(meta_extra or {}),
    }
    (p / "meta.json").write_text(json.dumps(meta, indent=2))
    return Artifact(
        path=p / "embeddings.npz",
        n=int(X.shape[0]),
        d=int(X.shape[1]),
        with_labels=labels_gold is not None,
    )
