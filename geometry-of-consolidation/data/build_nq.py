"""
Build a Natural Questions paraphrase-cluster corpus AND persist the raw
question + answer text side-cars used by the downstream LLM evaluation (E7).

Layout on disk (under $GAC_DATA_DIR/nq_questions/<model>/):
    embeddings.npz   -- X (L2-normalised), labels_gold, ids
    meta.json        -- standard metadata
    qa.jsonl         -- one line per example with {id, question, answers:[...], passage}

`qa.jsonl` is written OUTSIDE the per-model directory (it does not depend on
the embedding model) to save space:
    $GAC_DATA_DIR/nq_questions/qa.jsonl

We use the `nq_open` split for compact Q+A pairs. Each row has:
    {"id": "nq::<idx>", "question": "...", "answers": ["...", ...]}
Supporting passages are retrieved via the long-answer variant for E7
(optional; we fall back to closed-book if not present).
"""
from __future__ import annotations

import json
import pathlib

import numpy as np

from data._common import Artifact, embed_texts, out_path, root_dir, save_artifact


def build(
    model: str = "bge-large",
    n_questions: int = 20_000,
    persist_qa: bool = True,
) -> Artifact:
    try:
        from datasets import load_dataset
    except ImportError as e:  # pragma: no cover
        raise ImportError("pip install datasets") from e

    ds = load_dataset("nq_open", split=f"train[:{n_questions}]")

    texts: list[str] = []
    ids: list[str] = []
    qa_rows: list[dict] = []
    for i, row in enumerate(ds):
        q = (row.get("question") or "").strip()
        if not q:
            continue
        answers = row.get("answer") or row.get("answers") or []
        if isinstance(answers, str):
            answers = [answers]
        qid = f"nq::{i}"
        texts.append(q)
        ids.append(qid)
        qa_rows.append({"id": qid, "question": q, "answers": list(answers)})

    X = embed_texts(texts, model=model)

    # Persist QA side-car once per corpus (model-independent).
    if persist_qa:
        corpus_root = root_dir() / "nq_questions"
        corpus_root.mkdir(parents=True, exist_ok=True)
        qa_path = corpus_root / "qa.jsonl"
        with qa_path.open("w") as f:
            for r in qa_rows:
                f.write(json.dumps(r) + "\n")

    return save_artifact(
        corpus="nq_questions",
        model=model,
        X=X,
        labels_gold=None,  # nq_open does not have paraphrase-pair labels
        ids=ids,
        meta_extra={
            "source": "nq_open",
            "n_questions": len(texts),
            "qa_sidecar": "qa.jsonl",
        },
    )


if __name__ == "__main__":  # pragma: no cover
    build()
