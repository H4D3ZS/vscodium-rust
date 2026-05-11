"""
Build HotpotQA supporting-facts cluster corpus AND persist the raw question +
answer + supporting-passage side-car for the downstream LLM evaluation (E7).

Two companion files land in the corpus root (model-independent):
    $GAC_DATA_DIR/hotpot_qa/qa.jsonl
    $GAC_DATA_DIR/hotpot_qa/passages.jsonl

where `qa.jsonl` rows are
    {"id": qid, "question": str, "answers": [str], "cluster_id": int,
     "support_fact_ids": [str]}
and `passages.jsonl` rows are
    {"id": f"{qid}::f{j}", "cluster_id": int, "text": str, "title": str}
"""
from __future__ import annotations

import json
import pathlib

import numpy as np

from data._common import Artifact, embed_texts, out_path, root_dir, save_artifact


def build(
    model: str = "bge-large",
    n_questions: int = 3000,
    min_facts: int = 3,
    persist_qa: bool = True,
) -> Artifact:
    try:
        from datasets import load_dataset
    except ImportError as e:  # pragma: no cover
        raise ImportError("pip install datasets") from e
    ds = load_dataset("hotpot_qa", "distractor", split=f"train[:{n_questions * 2}]")

    labels: list[int] = []
    texts: list[str] = []
    ids: list[str] = []
    qa_rows: list[dict] = []
    passage_rows: list[dict] = []
    used = 0
    for row in ds:
        qid = row.get("id")
        question = (row.get("question") or "").strip()
        answer = row.get("answer")
        sf = row.get("supporting_facts", {}) or {}
        titles = sf.get("title", []) or []
        sent_ids = sf.get("sent_id", []) or []
        ctx = row.get("context", {}) or {}
        ctx_titles = ctx.get("title", []) or []
        ctx_sents = ctx.get("sentences", []) or []

        title_to_sents: dict[str, list[str]] = dict(zip(ctx_titles, ctx_sents))
        facts: list[tuple[str, str]] = []  # (title, sentence)
        for t, sid in zip(titles, sent_ids):
            sents = title_to_sents.get(t, [])
            if 0 <= sid < len(sents):
                s = sents[sid].strip()
                if len(s) > 20:
                    facts.append((t, s))
        if len(facts) < min_facts:
            continue
        local_ids: list[str] = []
        for j, (title, fact) in enumerate(facts):
            fid = f"{qid}::f{j}"
            texts.append(fact)
            labels.append(used)
            ids.append(fid)
            local_ids.append(fid)
            passage_rows.append({
                "id": fid, "cluster_id": used, "title": title, "text": fact,
            })
        qa_rows.append({
            "id": qid,
            "cluster_id": used,
            "question": question,
            "answers": [answer] if answer else [],
            "support_fact_ids": local_ids,
        })
        used += 1
        if used >= n_questions:
            break

    labels_arr = np.asarray(labels, dtype=np.int64)
    X = embed_texts(texts, model=model)

    if persist_qa:
        corpus_root = root_dir() / "hotpot_qa"
        corpus_root.mkdir(parents=True, exist_ok=True)
        with (corpus_root / "qa.jsonl").open("w") as f:
            for r in qa_rows:
                f.write(json.dumps(r) + "\n")
        with (corpus_root / "passages.jsonl").open("w") as f:
            for r in passage_rows:
                f.write(json.dumps(r) + "\n")

    return save_artifact(
        corpus="hotpot_qa",
        model=model,
        X=X,
        labels_gold=labels_arr,
        ids=ids,
        meta_extra={
            "source": "hotpot_qa distractor",
            "n_questions": used,
            "qa_sidecar": "qa.jsonl",
            "passages_sidecar": "passages.jsonl",
        },
    )


if __name__ == "__main__":  # pragma: no cover
    build()
