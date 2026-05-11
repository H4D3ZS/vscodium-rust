# Reproduction Guide — The Geometry of Consolidation

This guide describes how to regenerate every number, figure, and table in the
paper from source. All commands assume the repository root as the working
directory, Python ≥ 3.10, and the optional dependencies in
`pyproject.toml`.

## 1. Environment

```bash
# Local
python -m venv .venv && . .venv/bin/activate
pip install -e '.[experiments,viz]'

# Cloud (Modal, for the full sweep)
modal secret create gac-hf-secret HF_TOKEN=...
modal secret create llm-api-secret OPENAI_API_KEY=...
modal volume create gac-data
```

The `modal_app/app.py` entrypoint publishes four "run jobs": `run_one`,
`vllm_shard` (E7 reader), `embed`, and `build_wiki_scale`.

## 2. Data pipeline

All corpora are built from publicly available sources; builders live under
`data/`.

```bash
# Six real / synthetic corpora
python -m data.builders.drm
python -m data.builders.ms_marco
python -m data.builders.wikipedia_sections
python -m data.builders.arxiv
python -m data.builders.nq --n-questions 500
python -m data.builders.hotpot --n-questions 1189
python -m data.builders.popqa --n-questions 500
python -m data.builders.c4 --n-passages 5000

# Scale study (10K → 100K → 1M; 10M is scaffolded but not run)
python scripts/build_wiki_scale.py --sizes 10K 100K 1M
```

Every builder writes `<corpus>/_<split>_*.npz` (embeddings) and
`<corpus>/qa.jsonl` (question/answer text for NQ, HotpotQA, PopQA).

Encoders used: BGE-base, BGE-large, E5-large, Nomic-embed-v1, MiniLM,
MPNet — all called via `sentence-transformers`. The chosen pools of
questions / passages / sections are deterministic given the
`GAC_SEED=0` environment variable.

## 3. Running the experiments

Each experiment is a shardable function under `experiments/ei_*.py` that
writes `jsonl` shards to `runs/ei_*/<run>/shard_NN.jsonl` and a reduced
parquet to the same directory on completion.

```bash
# Local (tiny, for CI):
python -m experiments.e1_theorem_validation --run debug --n-shards 1 --shard-id 0

# Modal (full runs used in the paper):
modal run --detach modal_app/app.py::run_one --exp e1_theorem_validation --run-id neurips --n-shards 8
modal run --detach modal_app/app.py::run_one --exp e2_strategy_sweep     --run-id neurips --n-shards 8
modal run --detach modal_app/app.py::run_one --exp e3_learned_baselines  --run-id neurips --n-shards 8
modal run --detach modal_app/app.py::run_one --exp e4_identity_metrics   --run-id neurips --n-shards 3
modal run --detach modal_app/app.py::run_one --exp e5_scale_study        --run-id neurips --n-shards 4
modal run --detach modal_app/app.py::run_one --exp e6_gac_ablation       --run-id neurips --n-shards 6
modal run --detach modal_app/app.py::run_one --exp e7_downstream_llm     --run-id neurips --n-shards 3
modal run --detach modal_app/app.py::run_one --exp e8_drm_encoders       --run-id neurips --n-shards 4
modal run --detach modal_app/app.py::run_one --exp e9_temporal_mrr       --run-id neurips --n-shards 8
```

Each experiment writes a deterministic grid of cells given a fixed seed.

### Compute budget

| Experiment | Cells | Seeds | GPU-hours (BGE-large) |
|------------|-------|-------|------------------------|
| E1 | 2048 | 10 | 4 |
| E2 | 2028 | 1  | 24 |
| E3 | 150  | 1  | 3  |
| E4 | 30   | 3  | 1  |
| E5 | 80   | 1  | 6  |
| E6 | 126  | 3  | 3  |
| E7 | 15   | 1  | 2 (A100, vLLM) |
| E8 | 48   | 3  | 2  |
| E9 | 360  | 5  | 4  |

Total: ~50 GPU-hours on mixed A10G/L40S/A100.

## 4. Aggregating and plotting

After all experiments have reduced to parquet:

```bash
# Pull parquets from Modal volume (if you ran on Modal):
for exp in e1_theorem_validation e2_strategy_sweep e3_learned_baselines \
           e4_identity_metrics e5_scale_study e6_gac_ablation \
           e7_downstream_llm e8_drm_encoders e9_temporal_mrr; do
  short=$(echo $exp | sed 's/_.*//')
  modal volume get gac-data runs/$exp/neurips/${short}_results.parquet \
      results/$short/${short}_results.parquet --force
done

# Regenerate every figure + table in paper/:
python scripts/make_figures.py      # writes paper/figs/*.pdf
python scripts/make_tables.py       # writes paper/tables/t{1,2}*.tex
python scripts/make_supp_tables.py  # writes paper/tables/supp_t_*.tex
```

The c1 calibration (`results/c1_calibration.json`) is computed by
`scripts/calibrate_c1.py` from the reduced E1 parquet.

## 5. Building the paper

Texlive ≥ 2020 with `bibtex` on the PATH.

```bash
cd paper
pdflatex -interaction=nonstopmode main.tex
bibtex main
pdflatex -interaction=nonstopmode main.tex
pdflatex -interaction=nonstopmode main.tex

pdflatex -interaction=nonstopmode supp.tex
pdflatex -interaction=nonstopmode supp.tex
```

Outputs: `paper/main.pdf` (13 pages) and `paper/supp.pdf` (11 pages).

## 6. Public artifact mirror

Reduced parquets and figures are mirrored to a public S3 bucket for
reviewers who do not want to rerun the sweeps:

- `s3://gac-neurips-2026/results/` — reduced parquets, one per experiment
- `s3://gac-neurips-2026/figs/`    — final PDF figures
- `s3://gac-neurips-2026/tables/`  — final LaTeX tables
- `s3://gac-neurips-2026/paper/`   — compiled `main.pdf` and `supp.pdf`

(Bucket is read-only, request-pays off. Fetch with any S3 client.)

## 7. Known deviations from `SPEC.md`

These deviations are called out honestly in the paper Limitations
section:

- **E5**: reports 10K and 100K corpus sizes; 1M and 10M scaffolded but
  not executed (compute-gated).
- **E7**: uses Llama-3.1-8B-Instruct reader rather than the originally
  specified 70B; reports NQ only rather than NQ + HotpotQA + PopQA.
- **E4**: uses embedding-space paraphrase noise as a proxy for real
  Llama-70B paraphrases.
- **Learned quantization**: OpenAI `text-embedding-3-large` is
  scaffolded but not run; reported encoders are the six open-source
  models listed above.

None of these deviations affect the qualitative conclusions (regime
separation, centroid-dominates-GAC-on-real-text, c1 calibration).

## 8. Contact

Questions: `ashwin.gopinath@gmail.com`, or open a GitHub issue at
https://github.com/niashwin/geometry-of-consolidation.
