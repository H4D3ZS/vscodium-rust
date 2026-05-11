"""
Modal orchestrator for the Geometry-of-Consolidation experiments.

Design principles
-----------------
1. H100 80GB everywhere. GPU="H100" unless overridden.
2. Every experiment is a pure function: (config, input_path) -> output_path.
   Inputs and outputs live on a shared Modal Volume (persisted), so an
   experiment can resume from whatever checkpoint files already exist.
3. Every heavy function is idempotent at the file level: on invocation it
   first checks for its expected output artifact and skips if present.
4. Long runs are split into shards; each shard writes its own partial file,
   then a final reducer concatenates. Crash anywhere = resume anywhere.
5. Concurrency of fan-out is a *parameter* (MAX_CONCURRENT_H100), determined
   by the probe in scripts/h100_concurrency_probe.py.
6. Orchestration is itself checkpointed via a run-manifest JSON.

Usage
-----
    # one-shot launch
    modal run modal_app/app.py::run_all
    # resume with different fan-out
    MAX_CONCURRENT_H100=8 modal run modal_app/app.py::run_all
    # run just one experiment
    modal run modal_app/app.py::run_one --exp e1_theorem_validation
"""
from __future__ import annotations

import json
import os
import pathlib
import time
from typing import Iterable

import modal

# ---------------------------------------------------------------------------
# globals
# ---------------------------------------------------------------------------

APP_NAME = "geometry-of-consolidation"
VOLUME_NAME = "gac-data"
GPU = os.environ.get("GAC_GPU", "H100")
# Measured 2026-04-19: this account can run 8 H100s cleanly; 16 queues.
# See scripts/probe_results.json for the full ladder.
MAX_CONCURRENT_H100 = int(os.environ.get("MAX_CONCURRENT_H100", "8"))

CPU_SMALL = 2.0
CPU_LARGE = 8.0
MEM_SMALL = 8 * 1024  # MB
MEM_LARGE = 32 * 1024

# Timeouts are generous; long experiments checkpoint internally.
TIMEOUT_SHORT = 60 * 30          # 30 min  -- probes, I/O
TIMEOUT_GPU_SHARD = 60 * 60 * 4  # 4 hr   -- a single GPU shard
TIMEOUT_REDUCER = 60 * 60        # 1 hr

# ---------------------------------------------------------------------------
# image
# ---------------------------------------------------------------------------

image = (
    modal.Image.debian_slim(python_version="3.11")
    .apt_install("git", "build-essential")
    .pip_install(
        "numpy>=1.26",
        "scipy>=1.11",
        "scikit-learn>=1.3",
        "pandas>=2.1",
        "pyarrow>=14.0",
        "tqdm>=4.66",
        "hdbscan>=0.8.33",
        "sentence-transformers>=2.7",
        "datasets>=2.18",
        "transformers>=4.40",
        "torch>=2.3",
        "faiss-cpu>=1.7.4",
        "einops>=0.7",
        "matplotlib>=3.8",
        "hnswlib>=0.8.0",
        "openai>=1.30",
        "seaborn>=0.13",
    )
    # Copy the gac, experiments, data, and scripts packages into the image.
    .add_local_python_source("gac", "experiments", "data", "scripts")
)

# Separate image with vLLM for LLM downstream eval (E7). Kept separate from
# the main image because vLLM pulls in a big CUDA stack.
vllm_image = (
    modal.Image.debian_slim(python_version="3.11")
    .apt_install("git", "build-essential")
    .pip_install(
        "numpy>=1.26",
        "pandas>=2.1",
        "pyarrow>=14.0",
        "datasets>=2.18",
        "transformers>=4.44",
        "torch>=2.3",
        "vllm>=0.6.0",
        "sentence-transformers>=2.7",
    )
    .add_local_python_source("gac", "experiments", "data", "scripts")
)

HF_SECRET = modal.Secret.from_name("gac-hf-secret")
LLM_SECRET = modal.Secret.from_name("llm-api-secret")

app = modal.App(APP_NAME, image=image)
vol = modal.Volume.from_name(VOLUME_NAME, create_if_missing=True)

VOL_MOUNT = "/vol"
RUN_DIR = f"{VOL_MOUNT}/runs"
DATA_DIR = f"{VOL_MOUNT}/data"
CKPT_DIR = f"{VOL_MOUNT}/checkpoints"

# ---------------------------------------------------------------------------
# helpers: checkpointing
# ---------------------------------------------------------------------------


def _ensure_dirs() -> None:
    for d in (RUN_DIR, DATA_DIR, CKPT_DIR):
        pathlib.Path(d).mkdir(parents=True, exist_ok=True)


def _manifest_path(run_id: str) -> pathlib.Path:
    return pathlib.Path(RUN_DIR) / f"{run_id}.manifest.json"


def _load_manifest(run_id: str) -> dict:
    p = _manifest_path(run_id)
    if p.exists():
        return json.loads(p.read_text())
    return {"run_id": run_id, "created_at": time.time(), "steps": {}}


def _save_manifest(run_id: str, m: dict) -> None:
    _manifest_path(run_id).write_text(json.dumps(m, indent=2, default=str))
    vol.commit()


def _step_done(manifest: dict, step: str, output: str | None = None) -> bool:
    rec = manifest["steps"].get(step)
    if rec and rec.get("status") == "done":
        if output is None or pathlib.Path(rec.get("output", "")).exists():
            return True
    return False


def _mark_done(manifest: dict, step: str, output: str, meta: dict | None = None) -> None:
    manifest["steps"][step] = {
        "status": "done",
        "output": output,
        "meta": meta or {},
        "finished_at": time.time(),
    }


# ---------------------------------------------------------------------------
# GPU entrypoints
# ---------------------------------------------------------------------------


@app.function(
    image=image,
    gpu=GPU,
    volumes={VOL_MOUNT: vol},
    timeout=TIMEOUT_GPU_SHARD,
    cpu=CPU_LARGE,
    memory=MEM_LARGE,
    secrets=[HF_SECRET],
)
def gpu_shard(experiment: str, shard_id: int, config: dict) -> str:
    """
    Generic GPU shard runner.

    `experiment` must be one of the module names under `experiments/`.
    The experiment module exposes `run_shard(shard_id, config, out_dir, ckpt_dir)`
    and returns the path to its shard artifact. The shard is expected to be
    idempotent: if the artifact already exists with the right content hash,
    it returns immediately.
    """
    _ensure_dirs()
    import importlib

    mod = importlib.import_module(f"experiments.{experiment}")
    out_dir = pathlib.Path(RUN_DIR) / experiment / config.get("run_id", "default")
    out_dir.mkdir(parents=True, exist_ok=True)
    ckpt_dir = pathlib.Path(CKPT_DIR) / experiment / config.get("run_id", "default")
    ckpt_dir.mkdir(parents=True, exist_ok=True)

    artifact = mod.run_shard(
        shard_id=shard_id,
        config=config,
        out_dir=str(out_dir),
        ckpt_dir=str(ckpt_dir),
    )
    vol.commit()
    return artifact


@app.function(
    image=image,
    volumes={VOL_MOUNT: vol},
    timeout=TIMEOUT_GPU_SHARD,  # reuse long timeout
    cpu=CPU_LARGE,
    memory=MEM_LARGE,
    secrets=[HF_SECRET],
)
def cpu_shard(experiment: str, shard_id: int, config: dict) -> str:
    """CPU shard runner for CPU-bound experiments (E3, E4, E6, E8)."""
    _ensure_dirs()
    import importlib
    mod = importlib.import_module(f"experiments.{experiment}")
    out_dir = pathlib.Path(RUN_DIR) / experiment / config.get("run_id", "default")
    out_dir.mkdir(parents=True, exist_ok=True)
    ckpt_dir = pathlib.Path(CKPT_DIR) / experiment / config.get("run_id", "default")
    ckpt_dir.mkdir(parents=True, exist_ok=True)
    artifact = mod.run_shard(
        shard_id=shard_id, config=config,
        out_dir=str(out_dir), ckpt_dir=str(ckpt_dir),
    )
    vol.commit()
    return artifact


@app.function(
    image=image,
    volumes={VOL_MOUNT: vol},
    timeout=TIMEOUT_REDUCER,
    cpu=CPU_LARGE,
    memory=MEM_LARGE,
    secrets=[HF_SECRET],
)
def reduce_shards(experiment: str, config: dict, shard_artifacts: list[str]) -> str:
    """Reducer: merge shard artifacts into the final experiment output."""
    _ensure_dirs()
    import importlib

    mod = importlib.import_module(f"experiments.{experiment}")
    out_dir = pathlib.Path(RUN_DIR) / experiment / config.get("run_id", "default")
    out_dir.mkdir(parents=True, exist_ok=True)
    artifact = mod.reduce(
        shard_artifacts=shard_artifacts,
        config=config,
        out_dir=str(out_dir),
    )
    vol.commit()
    return artifact


@app.function(
    image=image,
    volumes={VOL_MOUNT: vol},
    timeout=TIMEOUT_SHORT,
    cpu=CPU_SMALL,
    memory=MEM_SMALL,
    secrets=[HF_SECRET],
)
def cpu_task(name: str, module: str, fn: str, kwargs: dict | None = None) -> str:
    """Small CPU task (data builders, figure rendering)."""
    _ensure_dirs()
    import importlib

    mod = importlib.import_module(module)
    out = getattr(mod, fn)(**(kwargs or {}))
    vol.commit()
    return str(out)


# ---------------------------------------------------------------------------
# orchestration
# ---------------------------------------------------------------------------


CPU_ONLY_EXPERIMENTS = {
    "e3_learned_baselines",
    "e4_identity_metrics",
    "e6_gac_ablation",
    "e8_drm_encoders",
    "e9_temporal_mrr",
}


def _fan_out(
    experiment: str,
    shard_ids: Iterable[int],
    config: dict,
    max_concurrent: int = MAX_CONCURRENT_H100,
) -> list[str]:
    """Run shards with a concurrency cap; return ordered artifact paths."""
    shard_ids = list(shard_ids)
    artifacts: list[str | None] = [None] * len(shard_ids)
    calls: list[tuple[int, modal.FunctionCall]] = []
    runner = cpu_shard if experiment in CPU_ONLY_EXPERIMENTS else gpu_shard

    # CPU shards are cheap; allow higher concurrency.
    if experiment in CPU_ONLY_EXPERIMENTS:
        max_concurrent = min(max_concurrent * 2, 32)

    i = 0
    while i < len(shard_ids) or calls:
        while len(calls) < max_concurrent and i < len(shard_ids):
            sid = shard_ids[i]
            print(f"[orchestrator] spawning {experiment} shard {sid}")
            fc = runner.spawn(experiment=experiment, shard_id=sid, config=config)
            calls.append((i, fc))
            i += 1
        # Wait for any one to finish.
        still_running: list[tuple[int, modal.FunctionCall]] = []
        for pos, fc in calls:
            try:
                res = fc.get(timeout=5)
                artifacts[pos] = res
                print(f"[orchestrator] shard {shard_ids[pos]} -> {res}")
            except modal.exception.OutputExpiredError:  # pragma: no cover
                # Modal expired; retry once.
                print(f"[orchestrator] shard {shard_ids[pos]} expired, re-spawning")
                fc2 = runner.spawn(
                    experiment=experiment, shard_id=shard_ids[pos], config=config
                )
                still_running.append((pos, fc2))
            except TimeoutError:
                still_running.append((pos, fc))
        calls = still_running
        time.sleep(1.0)

    assert all(a is not None for a in artifacts)
    return [str(a) for a in artifacts]  # type: ignore[list-item]


EXPERIMENTS: list[tuple[str, int]] = [
    # (experiment_module, n_shards)
    # Plan B: 8 concurrent H100s. Using 8 shards for each so one wave fills
    # the account's concurrency quota exactly.
    ("e1_theorem_validation", 8),
    ("e2_strategy_sweep", 8),
    ("e3_learned_baselines", 8),
    ("e4_identity_metrics", 8),
    ("e5_scale_study", 4),
    ("e6_gac_ablation", 6),
    ("e8_drm_encoders", 6),
    ("e9_temporal_mrr", 8),
    # E7 has its own entrypoint since it needs the vllm_image.
]


@app.local_entrypoint()
def run_all(run_id: str = "default", experiments: str | None = None):
    """Launch the full pipeline with resumable manifest."""
    _ensure_dirs_local()  # pragma: no cover
    manifest = _load_manifest_local(run_id)

    requested = (
        [e.strip() for e in experiments.split(",") if e.strip()]
        if experiments
        else [name for name, _ in EXPERIMENTS]
    )

    for exp_name, n_shards in EXPERIMENTS:
        if exp_name not in requested:
            continue
        step_key = f"{exp_name}:reduce"
        if _step_done(manifest, step_key):
            print(f"[runner] {exp_name} already done, skipping")
            continue

        config = {"run_id": run_id, "n_shards": n_shards}
        # Fan out shards (remote runs with resume-on-disk).
        artifacts = _fan_out(exp_name, range(n_shards), config)
        # Reduce.
        final = reduce_shards.remote(
            experiment=exp_name, config=config, shard_artifacts=artifacts
        )
        _mark_done(manifest, step_key, final, {"n_shards": n_shards})
        _save_manifest_local(run_id, manifest)
        print(f"[runner] {exp_name} -> {final}")

    print("[runner] all done")


@app.local_entrypoint()
def run_one(exp: str, run_id: str = "default", n_shards: int = 4):
    """Run a single experiment by module name."""
    config = {"run_id": run_id, "n_shards": n_shards}
    artifacts = _fan_out(exp, range(n_shards), config)
    final = reduce_shards.remote(experiment=exp, config=config, shard_artifacts=artifacts)
    print(f"[runner] {exp} -> {final}")


# ---------------------------------------------------------------------------
# data building
# ---------------------------------------------------------------------------


@app.function(
    image=image,
    gpu=GPU,
    volumes={VOL_MOUNT: vol},
    timeout=TIMEOUT_GPU_SHARD,
    cpu=CPU_LARGE,
    memory=MEM_LARGE,
    secrets=[HF_SECRET],
)
def build_corpus(name: str, model: str, kwargs: dict | None = None) -> str:
    """Run a data builder inside the Modal image on an H100 (for fast embed).

    `name` is one of: wikipedia, c4, arxiv, ms_marco, nq, hotpot, drm.
    """
    _ensure_dirs()
    import importlib
    import os

    os.environ["GAC_DATA_DIR"] = DATA_DIR
    # Required so HF respects the token on private datasets.
    hf_tok = os.environ.get("HF_TOKEN") or os.environ.get("HUGGING_FACE_HUB_TOKEN")
    if hf_tok:
        os.environ["HUGGING_FACE_HUB_TOKEN"] = hf_tok
        os.environ["HF_TOKEN"] = hf_tok

    mod = importlib.import_module(f"data.build_{name}")
    art = mod.build(model=model, **(kwargs or {}))
    vol.commit()
    return str(art.path)


@app.local_entrypoint()
def build_data(
    corpora: str = "drm,wikipedia,ms_marco",
    model: str = "bge-large",
):
    """Build the specified corpora in parallel. Default = the three
    corpora needed for the main E2 claims."""
    names = [c.strip() for c in corpora.split(",") if c.strip()]
    calls = []
    for name in names:
        kwargs: dict = {}
        # Reasonable per-corpus defaults that keep total cost under control.
        if name == "wikipedia":
            kwargs = {"n_articles": 400}
        elif name == "drm":
            kwargs = {"n_facts": 800, "paraphrases_per_fact": 5}
        elif name == "ms_marco":
            kwargs = {"n_queries": 1500}
        elif name == "nq":
            kwargs = {"n_questions": 8000}
        elif name == "hotpot":
            kwargs = {"n_questions": 2000}
        elif name == "c4":
            kwargs = {"n_docs": 10000}
        elif name == "arxiv":
            kwargs = {"n_papers": 30000}
        elif name == "popqa":
            kwargs = {"n_questions": 6000}
        calls.append((name, build_corpus.spawn(name=name, model=model, kwargs=kwargs)))

    for name, fc in calls:
        try:
            out = fc.get()
            print(f"[build_data] {name} -> {out}")
        except Exception as e:
            print(f"[build_data] {name} FAILED: {e}")


# ---------------------------------------------------------------------------
# local helpers (manifest runs on driver's local filesystem unless we're
# already inside a Modal container; these helpers dispatch accordingly)
# ---------------------------------------------------------------------------


def _ensure_dirs_local() -> None:
    # Locally we still create a run-manifest directory.
    pathlib.Path("./runs").mkdir(exist_ok=True)


def _load_manifest_local(run_id: str) -> dict:
    p = pathlib.Path(f"./runs/{run_id}.manifest.json")
    if p.exists():
        return json.loads(p.read_text())
    return {"run_id": run_id, "created_at": time.time(), "steps": {}}


def _save_manifest_local(run_id: str, m: dict) -> None:
    pathlib.Path(f"./runs/{run_id}.manifest.json").write_text(
        json.dumps(m, indent=2, default=str)
    )


# ---------------------------------------------------------------------------
# wiki_scale builder (10K / 100K / 1M / 10M wiki sentences, memmap float16)
# ---------------------------------------------------------------------------


@app.function(
    image=image,
    gpu=GPU,
    volumes={VOL_MOUNT: vol},
    timeout=60 * 60 * 8,  # 10M can take a while
    cpu=CPU_LARGE,
    memory=64 * 1024,
    secrets=[HF_SECRET],
)
def build_wiki_scale_shard(size_key: str, model: str) -> str:
    """Build one wiki_scale tier.

    `size_key` is "10K", "100K", "1M", or "10M".
    `model` is a sentence-transformer key recognised by scripts.build_wiki_scale.
    """
    _ensure_dirs()
    import os

    os.environ["GAC_DATA_DIR"] = DATA_DIR
    from scripts import build_wiki_scale as bws

    out = bws.build(size_key=size_key, model=model)
    vol.commit()
    return str(out)


@app.function(
    image=image,
    volumes={VOL_MOUNT: vol},
    timeout=TIMEOUT_SHORT,
    cpu=CPU_SMALL,
    memory=MEM_SMALL,
)
def fix_wiki_scale_partials() -> list[str]:
    """Rename any ``{size}_{model}.npz.partial.npz`` files to the
    canonical ``{size}_{model}.npz`` so they're consumable by E5.
    Also removes corresponding ``.dat`` memmap leftovers.
    """
    _ensure_dirs()
    root = pathlib.Path(DATA_DIR) / "wiki_scale"
    fixed: list[str] = []
    # Handle BOTH .npz.partial.npz -> .npz and .npz.npz -> .npz cases
    for p in sorted(root.glob("*.npz.npz")):
        target = p.with_name(p.name[:-4])  # strip one ".npz"
        os.replace(p, target)
        fixed.append(str(target))
    for p in sorted(root.glob("*.partial.npz")):
        target = p.with_name(p.name.replace(".npz.partial.npz", ".npz"))
        os.replace(p, target)
        fixed.append(str(target))
    for p in root.glob("*.dat"):
        try:
            p.unlink()
        except OSError:
            pass
    vol.commit()
    return fixed


@app.local_entrypoint()
def fix_wiki_scale():
    """One-shot rename for orphaned .partial.npz files."""
    out = fix_wiki_scale_partials.remote()
    print(f"[fix_wiki_scale] renamed {len(out)} files:")
    for p in out:
        print(f"  {p}")


@app.local_entrypoint()
def build_wiki_scale(
    sizes: str = "10K,100K,1M",
    models: str = "bge-large,minilm",
):
    """Build wiki_scale tiers in parallel on H100s."""
    size_list = [s.strip() for s in sizes.split(",") if s.strip()]
    model_list = [m.strip() for m in models.split(",") if m.strip()]
    calls: list[tuple[str, modal.FunctionCall]] = []
    for size in size_list:
        for model in model_list:
            tag = f"{size}:{model}"
            fc = build_wiki_scale_shard.spawn(size_key=size, model=model)
            calls.append((tag, fc))
    for tag, fc in calls:
        try:
            out = fc.get()
            print(f"[build_wiki_scale] {tag} -> {out}")
        except Exception as e:  # pragma: no cover
            print(f"[build_wiki_scale] {tag} FAILED: {e}")


# ---------------------------------------------------------------------------
# vLLM server for E7 (Llama-3.1-70B generation) and E4 (paraphrases)
# ---------------------------------------------------------------------------


@app.function(
    image=vllm_image,
    gpu="H100:2",  # 70B fits in 2x H100 80GB with tensor_parallel=2
    volumes={VOL_MOUNT: vol},
    timeout=60 * 60 * 6,
    cpu=CPU_LARGE,
    memory=64 * 1024,
    secrets=[HF_SECRET],
)
def vllm_generate(
    prompts_path: str,
    output_path: str,
    model: str = "meta-llama/Meta-Llama-3.1-70B-Instruct",
    max_tokens: int = 128,
    temperature: float = 0.0,
) -> str:
    """Batch-generate completions for a JSONL of prompts.

    Input JSONL lines: {"id": str, "prompt": str}.
    Output JSONL lines: {"id": str, "completion": str, "prompt": str}.
    """
    _ensure_dirs()
    import json as _json
    import os
    import pathlib as _pl

    from vllm import LLM, SamplingParams

    os.environ["HF_HOME"] = f"{VOL_MOUNT}/hf_cache"
    _pl.Path(os.environ["HF_HOME"]).mkdir(parents=True, exist_ok=True)

    llm = LLM(
        model=model,
        tensor_parallel_size=2,
        dtype="bfloat16",
        gpu_memory_utilization=0.90,
        max_model_len=4096,
        trust_remote_code=True,
    )
    sp = SamplingParams(
        max_tokens=max_tokens, temperature=temperature, top_p=1.0
    )

    prompts: list[dict] = []
    with open(prompts_path) as f:
        for line in f:
            line = line.strip()
            if line:
                prompts.append(_json.loads(line))

    outs = llm.generate([p["prompt"] for p in prompts], sp)
    out_path = _pl.Path(output_path)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    with open(out_path, "w") as f:
        for p, o in zip(prompts, outs):
            row = {
                "id": p["id"],
                "prompt": p["prompt"],
                "completion": o.outputs[0].text if o.outputs else "",
            }
            f.write(_json.dumps(row) + "\n")
    vol.commit()
    return str(out_path)


@app.local_entrypoint()
def run_vllm(
    prompts: str,
    output: str,
    model: str = "meta-llama/Meta-Llama-3.1-70B-Instruct",
    max_tokens: int = 128,
    temperature: float = 0.0,
):
    """Local driver that dispatches vLLM generation to the remote GPU pool."""
    out = vllm_generate.remote(
        prompts_path=prompts,
        output_path=output,
        model=model,
        max_tokens=max_tokens,
        temperature=temperature,
    )
    print(f"[vllm] wrote {out}")


# ---------------------------------------------------------------------------
# E7 dedicated runner (needs vllm_image)
# ---------------------------------------------------------------------------


@app.function(
    image=vllm_image,
    gpu="H100",  # 8B fits on 1x H100 comfortably
    volumes={VOL_MOUNT: vol},
    timeout=60 * 60 * 6,
    cpu=CPU_LARGE,
    memory=64 * 1024,
    secrets=[HF_SECRET],
)
def vllm_shard(experiment: str, shard_id: int, config: dict) -> str:
    """Shard runner for experiments that need vLLM (E7, 8B model)."""
    _ensure_dirs()
    import importlib
    import os

    os.environ["HF_HOME"] = f"{VOL_MOUNT}/hf_cache"
    pathlib.Path(os.environ["HF_HOME"]).mkdir(parents=True, exist_ok=True)

    mod = importlib.import_module(f"experiments.{experiment}")
    out_dir = pathlib.Path(RUN_DIR) / experiment / config.get("run_id", "default")
    out_dir.mkdir(parents=True, exist_ok=True)
    ckpt_dir = pathlib.Path(CKPT_DIR) / experiment / config.get("run_id", "default")
    ckpt_dir.mkdir(parents=True, exist_ok=True)
    artifact = mod.run_shard(
        shard_id=shard_id, config=config,
        out_dir=str(out_dir), ckpt_dir=str(ckpt_dir),
    )
    vol.commit()
    return artifact


@app.function(
    image=vllm_image,
    gpu="H100:2",  # 70B needs 2x H100 80GB with tp=2
    volumes={VOL_MOUNT: vol},
    timeout=60 * 60 * 8,
    cpu=CPU_LARGE,
    memory=96 * 1024,
    secrets=[HF_SECRET],
)
def vllm_shard_70b(experiment: str, shard_id: int, config: dict) -> str:
    """Shard runner for E7 at 70B scale (2x H100, tensor_parallel=2)."""
    _ensure_dirs()
    import importlib
    import os

    os.environ["HF_HOME"] = f"{VOL_MOUNT}/hf_cache"
    os.environ["GAC_TP"] = "2"
    if config.get("llm_model"):
        os.environ["GAC_LLM_MODEL"] = config["llm_model"]
    if config.get("n_questions"):
        os.environ["GAC_E7_N"] = str(config["n_questions"])
    pathlib.Path(os.environ["HF_HOME"]).mkdir(parents=True, exist_ok=True)

    mod = importlib.import_module(f"experiments.{experiment}")
    out_dir = pathlib.Path(RUN_DIR) / experiment / config.get("run_id", "default")
    out_dir.mkdir(parents=True, exist_ok=True)
    ckpt_dir = pathlib.Path(CKPT_DIR) / experiment / config.get("run_id", "default")
    ckpt_dir.mkdir(parents=True, exist_ok=True)
    artifact = mod.run_shard(
        shard_id=shard_id, config=config,
        out_dir=str(out_dir), ckpt_dir=str(ckpt_dir),
    )
    vol.commit()
    return artifact


@app.local_entrypoint()
def run_e7(run_id: str = "neurips", n_shards: int = 3,
           model: str = "meta-llama/Llama-3.1-8B-Instruct",
           n_questions: int = 500):
    """Dedicated E7 runner using vllm_image. 3 shards = 1 per dataset.
    Automatically picks the 2x H100 shard if the model is a 70B model."""
    import os as _os
    _os.environ["GAC_LLM_MODEL"] = model
    _os.environ["GAC_E7_N"] = str(n_questions)

    is_70b = "70B" in model or "70b" in model
    shard_fn = vllm_shard_70b if is_70b else vllm_shard
    max_concurrent = 2 if is_70b else 3  # 2x H100 per 70B shard

    config = {
        "run_id": run_id, "n_shards": n_shards,
        "enable_llm": True,
        "llm_model": model,
        "n_questions": n_questions,
    }
    shard_ids = list(range(n_shards))
    artifacts: list[str | None] = [None] * len(shard_ids)
    calls: list[tuple[int, modal.FunctionCall]] = []

    i = 0
    while i < len(shard_ids) or calls:
        while len(calls) < max_concurrent and i < len(shard_ids):
            sid = shard_ids[i]
            print(f"[e7] spawning shard {sid} ({'70B' if is_70b else '8B'})")
            fc = shard_fn.spawn(
                experiment="e7_downstream_llm", shard_id=sid, config=config
            )
            calls.append((i, fc))
            i += 1
        still: list[tuple[int, modal.FunctionCall]] = []
        for pos, fc in calls:
            try:
                res = fc.get(timeout=5)
                artifacts[pos] = res
                print(f"[e7] shard {shard_ids[pos]} -> {res}")
            except TimeoutError:
                still.append((pos, fc))
        calls = still
        time.sleep(1.0)

    final = reduce_shards.remote(
        experiment="e7_downstream_llm", config=config,
        shard_artifacts=[str(a) for a in artifacts],
    )
    print(f"[e7] -> {final}")
