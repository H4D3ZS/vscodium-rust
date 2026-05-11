# The Geometry of Consolidation

> **A scientific paper about a geometric law.** When a semantic memory replaces $n$ cluster members with $m<n$ representatives, what geometry decides whether retrieval still recovers the members? We prove a single inequality, measure it across six encoders and seven corpora, and derive the consolidation algorithm it implies.

<p align="left">
  <a href="paper/arxiv/main.pdf"><img alt="arXiv version" src="https://img.shields.io/badge/paper-arXiv_version-1f2937?style=flat-square"></a>
  <a href="paper/neurips/main.pdf"><img alt="NeurIPS 2026 submission" src="https://img.shields.io/badge/paper-NeurIPS_2026_submission-1f2937?style=flat-square"></a>
  <a href="paper/neurips/supp.pdf"><img alt="NeurIPS supplementary" src="https://img.shields.io/badge/paper-NeurIPS_supplementary-1f2937?style=flat-square"></a>
  <a href="LICENSE"><img alt="MIT License" src="https://img.shields.io/badge/license-MIT-4b5563?style=flat-square"></a>
</p>

---

## The trilogy

This paper is the third in a sequence on the geometry of meaning-organised memory. Each one asks the same question from a different angle: *what does it cost when a memory organises itself by meaning?*

| Paper | One-line claim |
|---|---|
| **[The Price of Meaning](https://arxiv.org/abs/submit/7412286)** · Barman et al., 2026 | Under mild axioms, any memory that organises itself by meaning must obey a strict no-escape trade-off between capacity and identity. |
| **[The Geometry of Forgetting](https://arxiv.org/abs/submit/7411865)** · Barman et al., 2026 | The shape of that trade-off is geometric, not informational: a cluster's *mean within-cluster distance* $\bar d$ and *effective dimension* $d_{\text{eff}}$ decide when retrieval must fail. |
| **The Geometry of Consolidation** · this repo | The same geometry governs *compression*. Replacing $n$ cluster members with $m<n$ representatives does not escape the trap — it trades one failure mode for another along the same geometric axis. |

---

## The law (one sentence)

For any consolidator, whenever the retrieval cap half-angle $\theta' = 1-\theta$ is smaller than the mean within-cluster cosine distance $\bar d$, the identity error satisfies

$$\varepsilon_{\text{id}} \;\geq\; 1 - c_1 \, m \, (\theta' / \bar d)^{d_{\text{eff}}/2}$$

where $c_1$ is an absolute constant and $d_{\text{eff}}$ is the local participation-ratio dimension. The law separates a **tight regime** ($\bar d < \theta'$, identity cheap) from a **spread regime** ($\bar d \geq \theta'$, identity forced to collapse under any non-trivial compression).

The papers in `paper/` give the proof, the experiments, and the algorithm it implies. The rest of this repo is the evidence.

---

## What's in this repository

```
paper/
  arxiv/        Long-form arXiv version (48 pp): full exposition, proofs, intuition panels.
  neurips/      NeurIPS 2026 submission: 14-pp main + 11-pp supplementary.

gac/            Geometry-Aware Consolidation — the algorithm the law implies.
                pip-installable; one interface, five baselines for comparison.

experiments/    One script per experimental block (E1–E9 in the paper).
                Each block is self-contained, resumable, and reproduces one figure.

results/        Raw parquet outputs from each E-block, plus c_1 calibration JSON.
                These are the numbers every figure and every table in the paper reads from.

scripts/
  make_figures.py       Regenerates every PDF figure in the paper from results/.
  calibrate_c1.py       Reconstructs the tight / spread / global c_1 table.

modal_app/      Modal orchestrator (H100 pool, resumable, cost-tracked) for reruns.
data/           Corpus build scripts: Wikipedia, MS MARCO, ArXiv, NQ, HotpotQA, DRM.
tests/          Unit tests for GAC and the checkpointing layer.
```

---

## The algorithm, in a line

**GAC** (Geometry-Aware Consolidation) routes each cluster to the consolidator the geometry calls for: centroid when $\bar d < \theta'$, residual-budgeted medoid otherwise. It Pareto-dominates centroid, medoid, importance-weighted, selective-pruning, PQ, OPQ, LSH, and HNSW-prune across 6 encoders × 7 corpora × 10K→1M scale.

```python
from gac import GACConsolidator, consolidate

# The geometry-aware router: centroid when the cluster is tight,
# residual-budgeted medoid when it is spread.
store = consolidate(embeddings, labels, strategy="gac", theta=0.85)
reps  = store.vectors   # the m < n L2-normalised representatives the law says you can afford
```

See [`paper/arxiv/main.pdf`](paper/arxiv/main.pdf) §7 for the algorithm and §8 for the Pareto frontier.

---

## Reproducing the paper

Every number in the paper is reproducible from `results/`. Every figure is reproducible from the scripts.

```bash
pip install -e .

# Reproduce every figure in the paper (reads from results/, writes to paper/*/figs/).
python scripts/make_figures.py --out paper/arxiv/figs
python scripts/make_figures.py --out paper/neurips/figs

# Reconstruct the c_1 calibration table (tight / spread / global, with coverage).
python scripts/calibrate_c1.py

# Rebuild the PDFs.
cd paper/arxiv  && pdflatex main && pdflatex main
cd paper/neurips && pdflatex main && bibtex main && pdflatex main && pdflatex main
cd paper/neurips && pdflatex supp && pdflatex supp && pdflatex supp
```

To rerun experiments from scratch on Modal (H100 pool, ~$30 for the full sweep):

```bash
modal run scripts/h100_concurrency_probe.py       # verify quota
modal run modal_app/app.py::run_all               # everything, resumable
modal run modal_app/app.py::run_one --exp e1      # single block
```

---

## Citation

```bibtex
@article{vangara2026consolidation,
  title   = {The Geometry of Consolidation},
  author  = {Vangara, Anirudh Bharadwaj and Gopinath, Ashwin},
  year    = {2026},
  note    = {Submitted to NeurIPS 2026. arXiv version in this repository.}
}
```

The sister papers:

```bibtex
@article{barman2026price,
  title  = {The Price of Meaning},
  author = {Barman and {collaborators}},
  year   = {2026},
  note   = {arXiv:submit/7412286}
}

@article{barman2026forgetting,
  title  = {The Geometry of Forgetting},
  author = {Barman and {collaborators}},
  year   = {2026},
  note   = {arXiv:submit/7411865}
}
```

---

## License & contact

MIT (see [`LICENSE`](LICENSE)).

- **Anirudh Bharadwaj Vangara** · Sentra; University of Waterloo Computer Engineering
- **Ashwin Gopinath** · Sentra; MIT Mechanical Engineering · [ashwin@sentra.app](mailto:ashwin@sentra.app) (corresponding)
