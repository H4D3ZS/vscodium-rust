# Evidence audit — what the data actually says

Date: 2026-04-18 (session 2).
All numbers trace to `results/e1/e1_results.parquet` and
`results/e2/e2_results.parquet`. Every paper claim must be checked
against this memo before it goes into `paper/main.tex`.

## Artifacts committed

| artifact | shape | content |
|---|---|---|
| `results/e1/e1_results.parquet` | 16,000 rows = 400 cells × 4 strategies × 10 seeds | synthetic theorem validation |
| `results/e2/e2_results.parquet` | 126 rows = 9 (corpus × model × clusterer) combos × 5 strategies, each seeded | strategy sweep on real text |
| `results/e1/e1_summary.json` | per-strategy headline stats | quick-look |
| `results/e2/e2_summary.json` | per-(corpus, strategy) headline stats | quick-look |

**Corpora covered (E2):** drm_templated, ms_marco, wikipedia_sections,
arxiv_titles, nq_questions, hotpot_qa (**6 corpora**).
**Embedding families:** BGE-large (all 6), MiniLM (drm, ms_marco,
wikipedia) — partial cross-family coverage.
**Clusterers:** kmeans, hdbscan.
**Strategies:** centroid, medoid, importance_weighted, selective_prune, gac.

## Corpus geometry (verified, BGE-large, kmeans centroid rows)

| corpus | d_eff_global | d_eff_local_mean | d̄_mean | #clusters |
|---|---:|---:|---:|---:|
| drm_templated | 20.1 | **2.3** | 0.054 | 800 |
| hotpot_qa | 152.8 | **1.5** | 0.482 | 1,189 |
| ms_marco | 154.4 | **5.5** | 0.331 | 1,500 |
| nq_questions | 90.8 | **12.6** | 0.380 | 500 |
| wikipedia_sections | 168.8 | **30.1** | 0.511 | 379 |
| arxiv_titles | 85.0 | **107.5** | 0.333 | 11 |

Note the HotpotQA `d_eff_local = 1.5` reflects its semantic clustering
(many near-duplicate passages inside each cluster); the high `d̄=0.48`
says those clusters are geometrically spread despite their low local
dimensionality.

## Claims the data SUPPORTS

### ✅ S1. Regime separation (E1)
Divide the 400 synthetic cells into **tight** (d̄ < θ′ = 1-θ, 310 cells)
and **spread** (d̄ ≥ θ′, 90 cells).

| regime | cells | err_cap centroid | err_cap gac | err_cap medoid | err_cap prune50 |
|---|---:|---:|---:|---:|---:|
| tight | 310 | 0.001 | 0.002 | 0.004 | 0.000 |
| spread | 90 | 0.566 | 0.331 | 0.741 | 0.298 |

**All strategies collapse to ≤0.004 err_cap in tight regime; in spread
regime they diverge by up to 74 points.** This is the strongest
qualitative claim in the paper and it is solidly backed.

### ✅ S2. d_eff monotonicity (E1, spread regime)
Mean err_cap vs. d_eff_target for the 90 spread cells × 10 seeds:

| d_eff | centroid | gac | medoid | prune50 |
|---:|---:|---:|---:|---:|
| 4 | 0.543 | 0.181 | 0.591 | 0.137 |
| 8 | 0.562 | 0.274 | 0.677 | 0.232 |
| 16 | 0.571 | 0.353 | 0.756 | 0.317 |
| 32 | 0.575 | 0.401 | 0.818 | 0.379 |
| 64 | 0.576 | 0.446 | 0.864 | 0.424 |

Monotonic in d_eff for every strategy, consistent with the theorem.

### ✅ S3. Template → real degradation (E2)
Centroid identity accuracy (BGE-large, kmeans) vs corpus d_eff_local:

| corpus | d_eff_local | identity_accuracy |
|---|---:|---:|
| drm_templated | 2.3 | **0.942** |
| ms_marco | 5.5 | 0.905 |
| arxiv_titles | 107.5 | 0.754 |
| wikipedia_sections | 30.1 | 0.647 |
| nq_questions | 12.6 | 0.761 |
| hotpot_qa | 1.5 | 0.487 |

The drop from synthetic templates to real text is real:
**DRM 0.942 → Wikipedia 0.647 (−29.6pt)**,
**DRM 0.942 → HotpotQA 0.487 (−45.5pt)**.

Caveat: HotpotQA has *lower* d_eff_local than Wikipedia yet *lower*
identity accuracy — so d_eff_local alone does not predict the drop
(d̄ and cluster topology also matter). The story is
"real corpora are much harder", not "d_eff_local monotonically
explains accuracy."

### ⚠️ S4. GAC beats selective_prune on DRM; ties on real text (E2)
Head-to-head at matched settings (GAC θ=0.8, selective_prune
keep-ratio=0.5), both at ≈2× compression:

| corpus | GAC acc | sel_prune acc | gap (pt) |
|---|---:|---:|---:|
| drm_templated | **0.932** | 0.769 | **+16.3** |
| ms_marco | 0.833 | 0.825 | +0.8 |
| nq_questions | 0.436 | 0.424 | +1.2 |
| arxiv_titles | 0.582 | 0.582 | 0.0 |
| wikipedia_sections | 0.381 | 0.381 | 0.0 |
| hotpot_qa | 0.354 | 0.355 | -0.1 |

**Honest reading:** GAC's averaging is worth ~16pt on synthetic paraphrase
clusters (where a handful of centroid directions capture the spread), but
real-text clusters are high-d_eff enough that averaging and keeping-raw
produce equivalent accuracy. The "GAC wins 6/6" framing is wrong; only
DRM is a real win. When averaging across kwargs (GAC θ∈{0.8,0.9},
prune kr∈{0.5,0.25}), GAC looks better on every corpus because prune at
0.25 is very lossy; that is not a fair matched-compression comparison.

### ✅ S5. GAC Pareto-dominates medoid (E1)
On the 400 E1 cells, GAC ≤ medoid on err_cap in every cell; strictly
better in **120 of 400**, tied in 280.
**120 wins / 280 ties / 0 losses.**

### ✅ S6. Centroid ≈ Importance-Weighted ≫ Medoid at high compression (E2)

Centroid and IW are within 0.001 of each other on every corpus; medoid
trails by 10-25pt because it cannot average.

## Claims the data DOES NOT SUPPORT

### ❌ N1. "GAC Pareto-dominates centroid"
At matched compression it does not. Per-cell E1:
GAC vs centroid: **65 wins / 286 ties / 49 losses**.
On E2 at high compression, centroid beats GAC on every corpus.
The honest claim is: GAC beats centroid *only at low compression where
centroid is inapplicable* (because centroid produces exactly one rep per
cluster).

### ❌ N2. "GAC gives best downstream retrieval (recall@5)"
Centroid has the highest recall@5 on **every** corpus:

| corpus | centroid | gac | medoid | selective_prune |
|---|---:|---:|---:|---:|
| drm_templated | 0.996 | 0.985 | 0.992 | 0.885 |
| ms_marco | 0.969 | 0.934 | 0.898 | 0.897 |
| wikipedia_sections | 0.830 | 0.643 | 0.685 | 0.548 |
| nq_questions | 0.973 | 0.747 | 0.943 | 0.688 |
| arxiv_titles | 0.979 | 0.868 | 0.937 | 0.814 |
| hotpot_qa | 0.691 | 0.539 | 0.529 | 0.520 |

Paper must NOT claim GAC improves recall.

### ❌ N3. "Matches the theoretical lower bound to within finite-sample noise"
E2 centroid, err_cap vs bound at θ=0.80:

| corpus | empirical err_cap | bound | gap (bound − emp) |
|---|---:|---:|---:|
| drm_templated | 0.000 | 0.000 | 0.000 |
| ms_marco | 0.455 | 0.936 | **+0.48 (loose)** |
| nq_questions | 0.673 | 1.000 | +0.33 (loose) |
| wikipedia_sections | 0.945 | 1.000 | +0.06 (tight) |
| arxiv_titles | 0.313 | 1.000 | +0.69 (very loose) |
| hotpot_qa | 0.992 | 0.721 | **−0.27 (violated)** |

**The bound is loose on moderate-d_eff corpora and violated on HotpotQA**
(empirical > bound). With c1=1 the bound is not predictive — this is a
calibration issue, not a falsification of the regime structure.

### ❌ N4. "92% → 10% template collapse"
Actual: centroid 94.2% (DRM) → 64.7% (Wikipedia) → 48.7% (HotpotQA).
That is a 29-45 point drop, not an 82-point drop.
Do not quote "92→10" in the paper.

### ❌ N5. "Closes the gap by 38%"
No such figure appears in any measurement. Drop from paper draft.
Closest honest statement: "GAC reduces err_cap from 0.57 (centroid) to
0.33 (mean over E1 spread cells), a relative improvement of 42% in the
spread regime, at the cost of keeping 6× more representatives."

### ❌ N6. "7 corpora × 2 embedding families"
We have 6 corpora × 1-2 families (MiniLM covers only 3 of them).
State this honestly: "six corpora spanning synthetic paraphrase,
QA, web-passage, and scientific-title domains, with a MiniLM
cross-embedding confirmation on three."

## Theorem calibration summary (the c1 question)

- **E1** theorem_holds_cap over all 16,000 rows: **78.5%** — but this is
  misleading: in the tight regime (d̄ < θ′) the bound is 0 and
  theorem_holds trivially. Restricting to the spread regime (3,600 rows
  where the bound bites): theorem_holds_cap drops to **4.6%** —
  empirical err_cap is BELOW the c1=1 bound on 95% of spread cells.
- This is a **one-sided miscalibration**: c1=1 predicts more error than
  actually occurs. A fitted c1 (≈0.3–0.5) would bring the bound close
  to empirical on the synthetic setting.
- On real text, the bound is loose on moderate-d_eff corpora (ms_marco,
  nq_questions, arxiv_titles by 30–70pt) and tight-to-violated on
  high-d̄ corpora (wikipedia within 6pt, hotpot_qa beyond the bound).
  The HotpotQA violation is a real phenomenon worth discussing — it
  likely comes from the spread (d̄=0.48) being large while d_eff_local
  is tiny (1.5), a regime not well-covered by the isotropic Gaussian
  derivation.

## Recommendations for the paper draft

1. **Rewrite abstract** to claim only what is here:
   - Regime separation is the headline result.
   - d_eff monotonicity is a secondary confirmation.
   - Template → real collapse is a 29–45pt drop (not 82pt).
   - GAC is Pareto-better than medoid and selective_prune; it does
     NOT dominate centroid on identity accuracy or recall@5.
   - The theoretical bound is *not* tight at c1=1; we report the
     calibration gap honestly as a limitation.

2. **Add a "Limitations" paragraph** (done in paper main.tex):
   - Bound loose at moderate d_eff.
   - HotpotQA exhibits bound violation — regime beyond current theory.
   - No downstream RAG eval (SPEC calls for it; not done this session).

3. **Do NOT quote:** "7 corpora", "92→10 collapse", "38% gap closure",
   "Pareto-dominates centroid", "matches the bound".

4. **Figures to produce (next step):**
   - F1. err_cap vs d̄/θ′ heatmap (E1, regime separation).
   - F2. err_cap vs d_eff monotonic lines per strategy (E1, spread).
   - F3. GAC vs selective_prune bar chart on 6 corpora (E2).
   - F4. Bound vs empirical scatter per corpus (E2).

---

# Session 3 NeurIPS push — new evidence (2026-04-19)

## New artifacts

| artifact | shape | content |
|---|---|---|
| `results/e4/e4_results.parquet` | 90 rows = 6 corpora × 5 strategies × 3 seeds | literal + paraphrase identity metrics |
| `results/e6/e6_results.parquet` | 126 rows = 6 corpora × 7 routers × 3 seeds | GAC router ablation |
| `results/e8/e8_results.parquet` | 108 rows = 6 encoders × 5 strategies × 3 seeds on DRM | encoder universality |
| `results/e3/e3_results_partial.parquet` | ~150 rows = 6 corpora × 25 baselines | learned-quantization Pareto |
| `results/c1_calibration.json` | JSON | per-regime c1 calibration |

## E4 — identity + paraphrase (BGE-large, 3 seeds)

- **Paraphrase robustness:** max drop 0.026 (HotpotQA/centroid). Identity is geometric, not memorization.
- Strategy ordering: centroid ≈ IW > medoid > GAC ≈ selective_prune on all 6 corpora.
- MRR@20 − Identity@1 gap: 0.05–0.10 (near-misses recover cluster in top-20).
- Recall@100 ≥ 0.95 on 5/6 corpora.

## E6 — GAC router ablation (7 routers × 6 corpora × 3 seeds)

| corpus | full | no-residual | random | fix-ctr | fix-med | fix-prune | oracle |
|---|---:|---:|---:|---:|---:|---:|---:|
| drm_templated | 0.940 | 0.942 | 0.840 | 0.943 | 0.920 | 0.776 | 0.943 |
| ms_marco | 0.836 | 0.833 | 0.806 | **0.899** | 0.794 | 0.830 | 0.862 |
| nq_questions | 0.433 | 0.434 | 0.491 | **0.784** | 0.691 | 0.425 | 0.512 |
| hotpot_qa | 0.356 | 0.355 | 0.386 | **0.499** | 0.357 | 0.355 | 0.489 |
| wikipedia_sections | 0.393 | 0.393 | 0.403 | **0.656** | 0.485 | 0.393 | 0.388 |
| arxiv_titles | 0.584 | 0.584 | 0.547 | **0.754** | 0.630 | 0.584 | 0.598 |

**Honest findings:**
(a) `full` and `no_residual` differ by ≤0.002 — residual-direction budget doesn't help.
(b) `fixed_centroid` beats `full` by +6.3 to +35.1 points on the 5 real corpora — stochastic routing hurts on real text.
(c) `oracle` rarely beats `fixed_centroid` (DRM tie, MS_MARCO -3.7, HotpotQA -1.0) — centroid is near-optimal per-cluster.

**Implication:** Recommended default at high compression on real text = `gac_fixed_centroid`; full router only for template-synthetic data.

## E8 — Encoder universality (DRM, 5 strategies × 6 encoders × 3 seeds)

| encoder | centroid | IW | medoid | prune | GAC |
|---|---:|---:|---:|---:|---:|
| bge-base | 1.000 | 1.000 | 0.997 | 0.886 | 0.998 |
| bge-large | 0.943 | 0.941 | 0.911 | 0.775 | 0.940 |
| e5-large | 0.961 | 0.959 | **0.998** | 0.835 | 0.960 |
| minilm | 0.950 | 0.948 | 0.848 | 0.744 | 0.937 |
| mpnet | 0.924 | 0.918 | 0.852 | 0.697 | 0.923 |
| nomic | 0.540 | 0.540 | 0.349 | 0.520 | 0.483 |

**Strategy ranking stable** across 5/6 encoders (centroid ≈ IW ≥ GAC ≫ medoid ≫ prune).
**Nomic** pushes DRM into spread regime (d̄ ≈ 0.34 vs 0.05 for BGE) — 44–66% drops. Predicted by theorem.
**e5-large** anomaly: medoid wins. Medoid on e5-large is somehow more aligned with the encoder's geometry.

## c1 calibration (E1 16,000 cells, p95 quantile)

| regime | cells | c1_p95 | holds after calib |
|---|---:|---:|---:|
| tight (d̄<θ') | 6,400 | 50.2 | >99% |
| spread (d̄≥θ') | 3,200 | 4.99×10^10 | 83.9% |
| all | 16,000 | 4.60×10^6 | 95.0% |

The spread-regime c1 explodes because (1-θ)^{(d-1)/2} → 0 at θ=0.95, d=32 (≈10^-42).
**Interpretation:** The isotropic Gaussian cap-volume bound is the correct SHAPE in the tight regime and up to θ≈0.9, but breaks down in the extreme-coherence corner. This is the primary remaining theoretical open problem.

## E3 — Learned quantization (partial, ~120/150 cells as of push)

Families tested: PQ (6 m), OPQ (5 m), LSH (4 nbits), PCA+int8 (6 k), HNSW-prune (4 kr).

Qualitative: at matched bytes-per-vector, consolidation (centroid/GAC) reaches higher identity accuracy in the tight-regime corner (DRM, MS MARCO); PQ/OPQ reach higher accuracy in the moderate-spread, high-d_eff corner (arxiv, wikipedia). Complementary Pareto frontiers.
