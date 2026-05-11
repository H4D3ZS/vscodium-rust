"""
Build Wikipedia sentence shards for the scale-study (E5).

Produces, under $GAC_DATA_DIR/wiki_scale/:
    10K_<model>.npz      -- X (n,d) float16, L2-normalised
    100K_<model>.npz
    1M_<model>.npz
    10M_<model>.npz      -- optional; guarded by --sizes
    meta.json            -- counts, model, built_at

Notes
-----
* Streams `wikimedia/wikipedia` 20231101.en and splits to sentences with a
  permissive regex; no dedup beyond exact-match drop. The corpus at 10M is
  roughly ~40GB of raw text; we budget ~50 min on H100 for BGE-large.
* Writes each size tier atomically via *.partial -> rename.
* Idempotent: skips tiers whose .npz already exists.
"""
from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import time

import numpy as np

from data._common import MODEL_ALIASES, MODEL_TEXT_PREFIX, root_dir

SENT_RE = re.compile(r"(?<=[.!?])\s+(?=[A-Z0-9\"'(\[])")

SIZES = {
    "10K": 10_000,
    "100K": 100_000,
    "1M": 1_000_000,
    "10M": 10_000_000,
}


def _iter_sentences(max_chars_per_sent: int = 400, min_chars_per_sent: int = 20):
    """Yield sentences from a streaming wikipedia dump forever."""
    from datasets import load_dataset

    ds = load_dataset(
        "wikimedia/wikipedia", "20231101.en", split="train", streaming=True
    )
    for row in ds:
        text = row.get("text") or ""
        for s in SENT_RE.split(text):
            s = s.strip()
            if min_chars_per_sent <= len(s) <= max_chars_per_sent:
                yield s


def _embed_streaming(
    texts: list[str],
    model: str,
    batch_size: int,
    device: str,
    dtype: str = "float16",
) -> np.ndarray:
    from sentence_transformers import SentenceTransformer

    model_name = MODEL_ALIASES.get(model, model)
    prefix = MODEL_TEXT_PREFIX.get(model, "")
    try:
        if "nomic" in model_name.lower():
            st = SentenceTransformer(model_name, device=device, trust_remote_code=True)
        else:
            st = SentenceTransformer(model_name, device=device)
    except TypeError:
        st = SentenceTransformer(model_name, device=device)

    if prefix:
        texts = [prefix + t for t in texts]

    X = st.encode(
        texts,
        batch_size=batch_size,
        convert_to_numpy=True,
        show_progress_bar=True,
        normalize_embeddings=True,
    )
    if dtype == "float16":
        X = X.astype(np.float16)
    else:
        X = X.astype(np.float32)
    return X


def build_tier(
    size_key: str,
    model: str,
    out_dir: pathlib.Path,
    batch_size: int = 256,
    device: str = "cuda",
    chunk: int = 50_000,
) -> pathlib.Path:
    """Build one scale tier; returns the output path.

    Memory plan: embed `chunk` sentences at a time, append to a preallocated
    (n, d) float16 array on disk-backed memmap to avoid OOM at 10M.
    """
    n = SIZES[size_key]
    out_path = out_dir / f"{size_key}_{model}.npz"
    if out_path.exists():
        print(f"[wiki_scale] {size_key}/{model} already built -> {out_path}")
        return out_path

    t0 = time.time()
    # Probe model dimensionality with a single batch.
    probe = _embed_streaming(["probe"], model=model, batch_size=1, device=device)
    d = probe.shape[1]
    print(f"[wiki_scale] model={model} d={d}; building {size_key}={n} sentences")

    # Use memmap for the big tiers to avoid blowing RAM.
    tmp_path = out_dir / f"{size_key}_{model}.dat"
    X_mm = np.memmap(tmp_path, dtype=np.float16, mode="w+", shape=(n, d))

    produced = 0
    it = _iter_sentences()
    while produced < n:
        bsz = min(chunk, n - produced)
        batch: list[str] = []
        for _ in range(bsz):
            try:
                batch.append(next(it))
            except StopIteration:
                break
        if not batch:
            break
        X = _embed_streaming(batch, model=model, batch_size=batch_size, device=device)
        X_mm[produced : produced + X.shape[0]] = X
        produced += X.shape[0]
        elapsed = time.time() - t0
        print(
            f"[wiki_scale] {size_key}/{model} {produced}/{n} "
            f"elapsed={elapsed:.1f}s rate={produced/elapsed:.0f} sent/s"
        )

    X_mm.flush()
    # Save as .npz (compact, portable) and drop the memmap. Note that
    # np.savez auto-appends .npz if the path doesn't end with .npz, so we
    # must pass a stem without .npz and then rename the produced file.
    X_final = np.asarray(X_mm)
    partial_stem = str(out_path.with_suffix("")) + ".partial"   # no .npz
    np.savez(partial_stem, X=X_final)                           # writes *.npz
    os.replace(partial_stem + ".npz", out_path)
    del X_mm
    try:
        tmp_path.unlink()
    except OSError:
        pass
    print(f"[wiki_scale] {size_key}/{model} -> {out_path}  ({produced} rows)")
    return out_path


def build(size_key: str, model: str, batch_size: int = 256) -> pathlib.Path:
    """Single-tier convenience wrapper used by modal_app."""
    device = "cuda"
    try:
        import torch

        if not torch.cuda.is_available():
            device = "cpu"
    except ImportError:
        device = "cpu"
    root = root_dir() / "wiki_scale"
    root.mkdir(parents=True, exist_ok=True)
    return build_tier(size_key, model, root, batch_size=batch_size, device=device)


def build_all(
    models: list[str],
    sizes: list[str],
    out_dir: str | None = None,
    batch_size: int = 256,
) -> None:
    device = "cuda"
    try:
        import torch

        if not torch.cuda.is_available():
            device = "cpu"
    except ImportError:
        device = "cpu"

    root = pathlib.Path(out_dir) if out_dir else (root_dir() / "wiki_scale")
    root.mkdir(parents=True, exist_ok=True)
    meta_path = root / "meta.json"
    meta: dict = {}
    if meta_path.exists():
        meta = json.loads(meta_path.read_text())
    meta["built_at"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    meta["source"] = "wikimedia/wikipedia 20231101.en (streaming)"
    meta["sizes"] = sizes
    meta["models"] = models
    meta_path.write_text(json.dumps(meta, indent=2))

    for model in models:
        for sz in sizes:
            build_tier(sz, model, root, batch_size=batch_size, device=device)


if __name__ == "__main__":  # pragma: no cover
    ap = argparse.ArgumentParser()
    ap.add_argument(
        "--models",
        default="bge-large,minilm",
        help="Comma-separated list of encoder aliases.",
    )
    ap.add_argument(
        "--sizes",
        default="10K,100K,1M",
        help="Comma-separated scale tiers. Add 10M for the full run.",
    )
    ap.add_argument("--out_dir", default=None)
    ap.add_argument("--batch_size", type=int, default=256)
    args = ap.parse_args()
    build_all(
        models=[m.strip() for m in args.models.split(",") if m.strip()],
        sizes=[s.strip() for s in args.sizes.split(",") if s.strip()],
        out_dir=args.out_dir,
        batch_size=args.batch_size,
    )
