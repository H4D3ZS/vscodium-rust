"""
Build a PopQA cluster corpus AND persist the raw Q+A side-car used by E7.

PopQA (`akariasai/PopQA`) is a long-tail open-domain QA benchmark with 14K
questions about facts derived from Wikipedia. Each row has a question, a
gold answer (possessive attribute), and a list of aliases. We cluster at
the subject-entity level: questions sharing the same entity form one cluster.

Side-cars (under $GAC_DATA_DIR/popqa/):
    qa.jsonl   -- {"id": f"popqa::{i}", "question": ..., "answers": [...], "cluster_id": k}
"""
from __future__ import annotations

import json
import pathlib
from collections import defaultdict

import numpy as np

from data._common import Artifact, embed_texts, root_dir, save_artifact


def build(
    model: str = "bge-large",
    n_questions: int = 8_000,
    min_cluster: int = 2,
    persist_qa: bool = True,
) -> Artifact:
    try:
        from datasets import load_dataset
    except ImportError as e:  # pragma: no cover
        raise ImportError("pip install datasets") from e

    try:
        ds = load_dataset("akariasai/PopQA", split=f"test[:{n_questions}]")
    except Exception:
        # Fallback: some HF caches require the main-branch naming.
        ds = load_dataset("akariasai/PopQA", split="test")
        if n_questions:
            ds = ds.select(range(min(n_questions, len(ds))))

    # Group by subject entity (`subj`) so each cluster is a set of questions
    # about the same real-world entity.
    groups: dict[str, list[int]] = defaultdict(list)
    examples: list[dict] = []
    for i, row in enumerate(ds):
        q = (row.get("question") or "").strip()
        if not q:
            continue
        # PopQA gold_answer is a JSON string list; handle both cases.
        gold = row.get("obj_aliases") or row.get("possible_answers") or row.get("answer") or []
        if isinstance(gold, str):
            try:
                gold = json.loads(gold)
            except Exception:
                gold = [gold]
        subj = (row.get("subj") or row.get("subj_id") or f"_solo_{i}").strip()
        examples.append({
            "i": i, "question": q, "answers": list(gold), "subj": subj,
        })
        groups[subj].append(len(examples) - 1)

    # Keep clusters >= min_cluster; everything else becomes a singleton
    # (still contributes to evaluation, just not as a cluster).
    cluster_id_map: dict[str, int] = {}
    cid = 0
    for subj, members in groups.items():
        if len(members) >= min_cluster:
            cluster_id_map[subj] = cid
            cid += 1

    texts: list[str] = []
    ids: list[str] = []
    labels: list[int] = []
    qa_rows: list[dict] = []
    singleton_next = cid
    for ex in examples:
        c = cluster_id_map.get(ex["subj"], None)
        if c is None:
            c = singleton_next
            singleton_next += 1
        qid = f"popqa::{ex['i']}"
        texts.append(ex["question"])
        ids.append(qid)
        labels.append(c)
        qa_rows.append({
            "id": qid, "question": ex["question"], "answers": ex["answers"],
            "cluster_id": c,
        })

    X = embed_texts(texts, model=model)
    labels_arr = np.asarray(labels, dtype=np.int64)

    if persist_qa:
        corpus_root = root_dir() / "popqa"
        corpus_root.mkdir(parents=True, exist_ok=True)
        with (corpus_root / "qa.jsonl").open("w") as f:
            for r in qa_rows:
                f.write(json.dumps(r) + "\n")

    return save_artifact(
        corpus="popqa",
        model=model,
        X=X,
        labels_gold=labels_arr,
        ids=ids,
        meta_extra={
            "source": "akariasai/PopQA",
            "n_questions": len(texts),
            "n_clusters": int(cid),
            "qa_sidecar": "qa.jsonl",
        },
    )


if __name__ == "__main__":  # pragma: no cover
    build()
