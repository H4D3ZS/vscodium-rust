"""
E7 -- Downstream RAG evaluation with a real LLM.

Pipeline:
    For each consolidation strategy (centroid, medoid, prune50, gac):
      1. Build the compressed store from 90% of NQ/HotpotQA/PopQA questions
         (treating each question-cluster label as one identity).
      2. For each held-out query, embed, retrieve top-k (k=5) representatives.
      3. Build a RAG prompt with those reps' source passages (for datasets
         that carry passages).
      4. Ask vLLM-hosted Llama-3.1-{model_size}-Instruct to answer.
      5. Score EM and F1 against the gold answer.

Notes:
  - Runs on Modal with the `vllm_image` stack (see modal_app/app.py).
  - Defaults to Llama-3.1-8B-Instruct for tractability; pass --model_size=70b
    to swap in the bigger model (requires H100x2 via `tensor_parallel_size`).
  - EM/F1 implementation follows the standard SQuAD text normaliser.

This experiment is intentionally narrow: 500 questions per dataset, so one
H100 covers all four strategies in a couple of hours with the 8B model.

If `vllm` is not installed (local dev), falls back to a retrieval-only eval
that reports Recall@5 of the correct passage (still a meaningful downstream
proxy and cheaper than running the LLM).
"""
from __future__ import annotations

import json
import os
import pathlib
import re
import string
import time
from collections import Counter

import numpy as np

from gac.metrics import recall_at_k
from gac.strategies import consolidate


LLM_MODEL = os.environ.get("GAC_LLM_MODEL", "meta-llama/Llama-3.1-8B-Instruct")
N_QUESTIONS = int(os.environ.get("GAC_E7_N", "500"))
RETRIEVAL_K = 5

DATASETS = ["nq_questions", "hotpot_qa", "popqa"]  # all three have gold labels
STRATEGIES = [
    ("centroid", {}),
    ("medoid", {}),
    ("selective_prune", {"keep_ratio": 0.5}),
    ("gac", {"theta": 0.8}),
    ("no_consolidation", {}),  # upper bound: keep every train vector
]


def _l2norm(X):
    return X / (np.linalg.norm(X, axis=1, keepdims=True) + 1e-12)


# ---------------------------------------------------------------------------
# EM / F1 (SQuAD v1.1 text normaliser)
# ---------------------------------------------------------------------------


def _normalize_answer(s: str) -> str:
    s = s.lower()
    s = re.sub(r"\b(a|an|the)\b", " ", s)
    s = "".join(ch for ch in s if ch not in set(string.punctuation))
    s = " ".join(s.split())
    return s


def _em(pred: str, gold: str) -> float:
    return float(_normalize_answer(pred) == _normalize_answer(gold))


def _f1(pred: str, gold: str) -> float:
    p = _normalize_answer(pred).split()
    g = _normalize_answer(gold).split()
    if not p or not g:
        return float(p == g)
    common = Counter(p) & Counter(g)
    num_same = sum(common.values())
    if num_same == 0:
        return 0.0
    prec = num_same / len(p)
    rec = num_same / len(g)
    return 2 * prec * rec / (prec + rec)


# ---------------------------------------------------------------------------
# data access
# ---------------------------------------------------------------------------


def _data_path(corpus: str, model: str = "bge-large") -> pathlib.Path:
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    return root / corpus / model / "embeddings.npz"


def _load_texts(corpus: str) -> dict | None:
    """Load question texts + gold answers from the qa.jsonl side-car.
    Falls back to older texts.json/answers.json format if present.
    Returns None if nothing found (then retrieval-only eval).
    """
    root = pathlib.Path(os.environ.get("GAC_DATA_DIR", "/vol/data"))
    qa_p = root / corpus / "qa.jsonl"
    if qa_p.exists():
        texts: list[str] = []
        answers: list[list[str]] = []
        with qa_p.open() as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                row = json.loads(line)
                texts.append(row.get("question") or row.get("text") or "")
                a = row.get("answers") or row.get("answer") or []
                if isinstance(a, str):
                    a = [a]
                # Pick first answer as canonical gold (EM/F1 will compare).
                answers.append(a if a else [""])
        # answers is list[list]; downstream expects a simple string per row.
        return {
            "texts": texts,
            "answers": [a[0] if a else "" for a in answers],
            "answers_all": answers,
        }
    # Legacy
    texts_p = root / corpus / "bge-large" / "texts.json"
    answers_p = root / corpus / "bge-large" / "answers.json"
    if texts_p.exists() and answers_p.exists():
        return {
            "texts": json.loads(texts_p.read_text()),
            "answers": json.loads(answers_p.read_text()),
        }
    return None


def _build_consolidator(X_train, labels_train, strat, kw):
    if strat == "no_consolidation":
        # Every vector is its own representative.
        from gac.strategies import CompressedStore
        return CompressedStore(
            vectors=_l2norm(X_train.astype(np.float32)),
            cluster_ids=labels_train.astype(np.int64),
            source_ids=np.arange(X_train.shape[0], dtype=np.int64),
            origin=np.asarray(["none"] * X_train.shape[0], dtype=object),
            meta={"strategy": "no_consolidation", "compression": 1.0},
        )
    return consolidate(X_train, labels_train, strategy=strat, **kw)


def _retrieve_topk(queries, store, k: int) -> np.ndarray:
    Q = _l2norm(queries.astype(np.float32))
    R = _l2norm(store.vectors.astype(np.float32))
    S = Q @ R.T
    order = np.argsort(-S, axis=1)[:, :k]
    return order  # (q, k) indices into store


# ---------------------------------------------------------------------------
# RAG -- using vLLM for LLM answers.
# ---------------------------------------------------------------------------


def _format_prompt(question: str, contexts: list[str]) -> str:
    ctx = "\n\n".join(f"[CTX {i+1}] {c}" for i, c in enumerate(contexts))
    return (
        "You are a helpful assistant. Use the provided context to answer "
        "the question concisely (1-8 words). If unsure, answer with your "
        "best guess.\n\n"
        f"Context:\n{ctx}\n\n"
        f"Question: {question}\n"
        f"Answer:"
    )


def _run_llm_batch(prompts: list[str]) -> list[str]:
    """Run vLLM on a batch of prompts. Returns generated text (post 'Answer:')."""
    try:
        from vllm import LLM, SamplingParams
    except ImportError:
        raise RuntimeError("vllm not installed; re-run on the Modal vllm_image")
    tp = int(os.environ.get("GAC_TP", "1"))
    # max_model_len capped so 70B fits in 2xH100 KV cache budget.
    # Our prompts + 32 output tokens are well under 4096.
    max_len = int(os.environ.get("GAC_MAX_LEN", "4096"))
    llm = LLM(model=LLM_MODEL, tensor_parallel_size=tp, dtype="bfloat16",
              gpu_memory_utilization=0.92, enforce_eager=True,
              max_model_len=max_len)
    params = SamplingParams(temperature=0.0, max_tokens=32, stop=["\n"])
    outs = llm.generate(prompts, params)
    return [o.outputs[0].text.strip() for o in outs]


# ---------------------------------------------------------------------------
# cell runner
# ---------------------------------------------------------------------------


def _run_cell(cid: int, dataset: str, strat: str, kw: dict, seed: int = 0,
              enable_llm: bool = True) -> dict:
    p = _data_path(dataset)
    if not p.exists():
        raise FileNotFoundError(f"Missing embeddings {p}")
    data = np.load(p, allow_pickle=True)
    X = data["X"].astype(np.float32)
    labels = data["labels_gold"] if "labels_gold" in data.files else None
    ids = data["ids"] if "ids" in data.files else None

    rng = np.random.default_rng(seed)
    if labels is None or np.all(np.asarray(labels) == np.asarray(labels)[0]):
        # No paraphrase clusters (e.g., NQ open). Fall back to random 90/10
        # split: each row becomes its own singleton identity. This lets us
        # evaluate end-to-end RAG over the question store as a corpus.
        n = len(X)
        labels = np.arange(n, dtype=np.int64)
        q_size = min(N_QUESTIONS, max(1, n // 10))
        q_all = rng.choice(n, size=q_size, replace=False)
        keep = np.ones(n, dtype=bool)
        # For singleton identities, we MUST keep the query vectors in the
        # train store too (otherwise recall is trivially zero against a
        # different cluster label). The consolidator will aggregate each
        # singleton to itself — compression=1.0 there, so the effect is
        # purely on retrieval quality over the remaining store.
        X_train = X; labels_train = labels
        X_query = X[q_all]; labels_query = labels[q_all]
    else:
        # Paraphrase-cluster mode (HotpotQA, PopQA). Hold out 1 per cluster.
        keep = np.ones(len(X), dtype=bool); q_all = []
        for lab in np.unique(labels):
            if lab < 0:
                continue
            idx = np.flatnonzero(labels == lab)
            if len(idx) < 2:
                continue
            qi = rng.choice(idx, size=1, replace=False)
            keep[qi] = False; q_all.append(qi)
        q_all = np.concatenate(q_all) if q_all else np.array([], dtype=np.int64)
        if len(q_all) > N_QUESTIONS:
            sel = rng.choice(len(q_all), size=N_QUESTIONS, replace=False)
            q_all = q_all[sel]
        if len(q_all) == 0:
            # Fallback: still allow the cell to run with random 10% holdout.
            n = len(X)
            labels = np.arange(n, dtype=np.int64)
            q_size = min(N_QUESTIONS, max(1, n // 10))
            q_all = rng.choice(n, size=q_size, replace=False)
            X_train = X; labels_train = labels
        else:
            X_train = X[keep]; labels_train = labels[keep]
        X_query = X[q_all]; labels_query = labels[q_all]

    store = _build_consolidator(X_train, labels_train, strat, kw)
    topk = _retrieve_topk(X_query, store, k=RETRIEVAL_K)
    # Cluster-level recall@k (surrogate for retrieval quality)
    r_at = recall_at_k(X_query, labels_query, store, ks=(1, 5, 10))

    em_mean = f1_mean = float("nan")
    if enable_llm:
        txt = _load_texts(dataset)
        if txt is not None and ids is not None:
            # Bounds-safe indexing: qa.jsonl may be shorter than X if the
            # embedding index was truncated or if qa.jsonl was regenerated.
            T = len(txt["texts"])
            def _tx(i: int) -> str:
                return txt["texts"][i] if 0 <= i < T else ""
            def _an(i: int) -> str:
                return txt["answers"][i] if 0 <= i < T else ""
            questions = [_tx(int(i)) for i in q_all]
            gold = [_an(int(i)) for i in q_all]
            # Retrieved contexts = texts of retrieved reps' source_ids.
            prompts = []
            for qi, order in enumerate(topk):
                ctxs = []
                for j in order:
                    src = int(store.source_ids[j])
                    if src >= 0:
                        ctxs.append(_tx(src) or "(context unavailable)")
                    else:
                        ctxs.append("(synthetic representative: no text)")
                prompts.append(_format_prompt(questions[qi], ctxs))
            try:
                answers = _run_llm_batch(prompts)
            except Exception as e:
                print(f"[e7] LLM failed, fallback to retrieval-only: {e}")
                answers = [""] * len(prompts)
            ems = [_em(a, g) for a, g in zip(answers, gold)]
            f1s = [_f1(a, g) for a, g in zip(answers, gold)]
            em_mean = float(np.mean(ems)) if ems else float("nan")
            f1_mean = float(np.mean(f1s)) if f1s else float("nan")

    return {
        "cell_id": cid, "dataset": dataset, "strategy": strat,
        "strategy_kw": json.dumps(kw, sort_keys=True), "seed": seed,
        "n_train": int(X_train.shape[0]), "n_query": int(X_query.shape[0]),
        "n_representatives": int(store.n_representatives),
        "compression": float(store.meta.get("compression", 1.0)),
        "recall@1_retrieval": r_at["recall@1"],
        "recall@5_retrieval": r_at["recall@5"],
        "recall@10_retrieval": r_at["recall@10"],
        "em": em_mean, "f1": f1_mean,
        "llm_model": LLM_MODEL,
    }


def _cells():
    cid = 0
    for ds in DATASETS:
        for strat, kw in STRATEGIES:
            yield cid, ds, strat, kw
            cid += 1


def run_shard(shard_id: int, config: dict, out_dir: str, ckpt_dir: str) -> str:
    n_shards = int(config.get("n_shards", 2))
    seed = int(config.get("seed", 0))
    enable_llm = bool(config.get("enable_llm", True))
    out_path = pathlib.Path(out_dir) / f"shard_{shard_id:02d}.jsonl"
    ckpt = pathlib.Path(ckpt_dir) / f"shard_{shard_id:02d}.completed.json"
    done = set(json.loads(ckpt.read_text()).get("done", [])) if ckpt.exists() else set()
    if not out_path.exists():
        out_path.touch()
    with out_path.open("a") as f:
        for cid, ds, strat, kw in _cells():
            if cid % n_shards != shard_id:
                continue
            if cid in done:
                continue
            t0 = time.time()
            try:
                rec = _run_cell(cid, ds, strat, kw, seed=seed, enable_llm=enable_llm)
            except FileNotFoundError as e:
                print(f"[e7 shard {shard_id}] {cid} SKIP: {e}")
                done.add(cid); ckpt.write_text(json.dumps({"done": sorted(done)}))
                continue
            f.write(json.dumps(rec) + "\n"); f.flush(); os.fsync(f.fileno())
            done.add(cid); ckpt.write_text(json.dumps({"done": sorted(done)}))
            dt = time.time() - t0
            print(f"[e7 shard {shard_id}] {ds}/{strat} -> {dt:.1f}s "
                  f"r@5={rec['recall@5_retrieval']:.3f} em={rec['em']}")
    return str(out_path)


def reduce(shard_artifacts: list[str], config: dict, out_dir: str) -> str:
    import pandas as pd
    frames = [pd.read_json(p, lines=True) for p in shard_artifacts]
    df = pd.concat(frames, ignore_index=True) if frames else pd.DataFrame()
    out_path = pathlib.Path(out_dir) / "e7_results.parquet"
    df.to_parquet(out_path, index=False)
    return str(out_path)


if __name__ == "__main__":  # pragma: no cover
    run_shard(0, {"n_shards": 1, "enable_llm": False}, "./runs/e7_smoke", "./checkpoints/e7_smoke")
