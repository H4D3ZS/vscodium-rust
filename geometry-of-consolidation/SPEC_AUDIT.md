# SPEC Audit — Geometry of Consolidation

**Date**: 2026-04-18 (Session 4)
**Spec**: `SPEC.md` (754 lines, dated 2026-04-18)
**Audit target**: every deliverable, experiment, claim, theorem, figure, and
table committed in the spec, cross-referenced against shipped artifacts.

**Traffic-light key**:
- 🟢 DONE — shipped, verified against data
- 🟡 PARTIAL — scaffolded or partly done; gaps documented in paper as limitation
- 🔴 MISSING — not executed; must be added, replaced, or declared out of scope

---

## Section 0 — Executive summary commitments

### 0.3 "Knock-socks-off" contributions

| # | Commitment | Status | Evidence |
|---|---|---|---|
| C1 | Consolidation–Interference Duality theorem (formal) | 🟢 | `paper/main.tex` §Methods — cap-volume lemma + union bound proof, Lemma + Theorem stated with constants $c_1, c_0$. |
| C2 | Identity–Coverage as two orthogonal axes | 🟡 | Verbally argued in paper; Corollary 1 stated. No dedicated 2-axis scatter figure (SPEC Fig 4) — see §7.1 below. |
| C3 | GAC Pareto-dominates all heuristics + learned baselines | 🟡 → 🔴 | **GAC does NOT Pareto-dominate centroid on real text** (Findings N1/N2). Paper honestly reframes as "complementary Pareto frontiers" and recommends `gac_fixed_centroid`. **This is a material deviation from the spec headline.** |

---

## Section 2 — Theory

| Claim | Status | Notes |
|---|---|---|
| Thm 1 formal statement with constants | 🟢 | Stated in paper with bound $c_1 (\theta'/\bar d)^{d_{\rm eff}/2}$. |
| Proof via 4 steps (cap volume + union bound) | 🟢 | Present in Methods; Berry–Esseen step invoked. |
| Corollary 1 (Voronoi–Identity duality) | 🟢 | Stated. |
| Corollary 2 (spectral predictor $\rho_C$) | 🟡 | Stated but not used as decision criterion in results — GAC router uses $\bar d$ and $d_{\rm eff}$, not $\rho_C$. |
| Corollary 3 (importance weighting converges to uniform) | 🟡 | Observed empirically in E2 (centroid ≈ IW within 0.001) but not derived as a corollary in the paper. |
| Empirical validation of bound tightness on Gaussian + sub-Gaussian | 🟡 | E1 covers this; tightness confirmed in tight regime, bound **breaks in extreme-coherence corner** (θ→1 with spread > θ'). Documented as L1. |

---

## Section 3 — GAC algorithm

| Item | Status | Notes |
|---|---|---|
| Algorithm implemented (spectral branch + residuals + auto-θ) | 🟢 | `gac/strategies.py` + `gac/router.py` |
| All three branches used in experiments | 🟢 | E6 ablation isolates each branch. |
| Residual-frame representation (top-r directions, r=2–4) | 🟢 | Implemented; E6 shows residual adds nothing on real text (full ≈ no_residual). |
| Auto-θ estimator from within-doc pair percentile | 🟢 | `gac/router.py::auto_theta` |

---

## Section 4 — Experimental plan (the bulk)

### E1 — Empirical validation of the bound

| Spec | Shipped | Status |
|---|---|---|
| Synthetic $\R^{64}$ clusters | ✅ | 🟢 |
| $d_{\rm eff} \in \{4,8,16,32\}$ | Expanded to include 64 (5 values) | 🟢 |
| $\bar d_C \in \{0.05,0.1,0.2,0.4,0.6\}$ | ✅ 5 values | 🟢 |
| $\theta \in \{0.6,0.7,0.8,0.9\}$ | ✅ 4 values | 🟢 |
| 10 seeds | ✅ | 🟢 |
| Figure with subplots per $(d_{\rm eff},\theta)$ | Figure 1 ships ✅ (different layout: regime split + monotonicity) | 🟢 |
| **Shipped rows** | **16,000** (400 cells × 4 strategies × 10 seeds) | 🟢 |

**Audit verdict: DONE. ** Exceeds spec scope.

### E2 — Extended strategy sweep

| Spec | Shipped | Status |
|---|---|---|
| 6 encoders: BGE-large, BGE-base, MiniLM, E5-large, Nomic, **OpenAI text-3-large** | BGE-large (6/6), MiniLM (3/6), BGE-base (DRM), E5/Nomic/MPNet (DRM via E8). **OpenAI embed NOT used.** | 🔴 |
| 5 domains: Wiki, C4, ArXiv, MS MARCO, DRM | 6 corpora: DRM, MS MARCO, Wiki, arxiv_titles, NQ, HotpotQA. **C4 replaced with NQ/HotpotQA.** | 🟡 |
| 7 strategies incl. no-consolidation | 5 strategies (centroid, medoid, IW, prune, GAC). No-consolidation baseline missing. | 🟡 |
| 5 compression levels {2,5,10,40,200×} | 2 settings per strategy (θ ∈ {0.8,0.9} or keep-ratio ∈ {0.25,0.5}) — effectively 2-5× compression only. **40× and 200× NOT swept in E2.** | 🔴 |
| BT, NN-preservation | Implemented in metrics but **not reported in paper tables**. | 🟡 |
| Identity recall@1 at {0.6,0.7,0.8,0.9} | Only θ=0.8 reported in paper. | 🟡 |
| MRR@20, Recall@10/100 | ✅ computed (E4); E2 only identity accuracy + coverage. | 🟢/🟡 |
| Noisy retrieval σ ∈ {0.4,1.0,2.0} | 🔴 **NOT RUN** |
| 3 seeds | ✅ | 🟢 |
| **Shipped rows** | 210 (vs SPEC's ~3,150 × 5 compression = ~15,000) | 🔴 |

**Audit verdict: PARTIAL.** Core result (regime sep + template→real) is solid.
Grid is much narrower than committed. **Defensible for submission only if
clearly scoped and if E7 downstream evidence lands.**

### E3 — Learned-compression baselines

| Spec | Shipped | Status |
|---|---|---|
| PQ (faiss.IndexPQ) | ✅ 6 m values | 🟢 |
| OPQ | ✅ 5 m values | 🟢 |
| LSH | ✅ 4 nbits | 🟢 |
| PCA+int8 quantise | ✅ 6 k values | 🟢 |
| HNSW-prune | ✅ 4 keep-ratios | 🟢 |
| Target compression ratios {2,5,10,40,200×} | Only matched via family-native knobs; not explicit ratio-sweep | 🟡 |
| 6 encoders × 2 corpora (Wiki, MS MARCO) | 6 corpora but **BGE-large only** (arxiv/NQ/Hotpot lack non-BGE embeddings on Modal volume) | 🟡 |
| **Shipped rows** | 138/150 (12 cells on NQ/HotpotQA timed out — OPQ/HNSW) | 🟡 |
| Headline finding | Centroid Pareto-dominates learned quant on 5/6 corpora; arxiv is exception (d_eff≈107) | 🟢 |

**Audit verdict: PARTIAL. ** Core learned-baseline comparison lands but
single-encoder. **Paper should add a caveat that encoder-universality of this
Pareto is only tested on DRM (via E8), not on these 6 corpora.**

### E4 — Identity-level metrics + paraphrase

| Spec | Shipped | Status |
|---|---|---|
| 100K Wiki passages | Used current corpus builds (10K scale) | 🟡 |
| MRR, Recall@{1,10,100}, identity cosine | ✅ all metrics | 🟢 |
| Paraphrase queries via Llama-3.1-70B | **Embedding-space noise proxy** (L2 in paper) | 🔴 |
| 3 seeds × 6 corpora × 5 strategies | ✅ 90 rows | 🟢 |

**Audit verdict: PARTIAL.** The metric suite is complete. The **paraphrase
protocol is a material substitution**: instead of LLM-generated text
paraphrases, E4 perturbs embeddings with Gaussian noise. This is honestly
disclosed as L2 but is a real gap against the spec.

### E5 — Scale study (10K → 10M)

| Spec | Shipped | Status |
|---|---|---|
| $10^4, 10^5, 10^6, 10^7$ Wiki subsamples | Experiment scaffold exists (`e5_scale_study.py`); **NO DATA RUN** | 🔴 |
| Fixed 40× compression, BGE-large + MiniLM | Coded, untested | 🔴 |
| `scripts/build_wiki_scale.py` to produce `/vol/data/wiki_scale/{size}_{model}.npz` | **DOES NOT EXIST** | 🔴 |
| Figure 5 (scale collapse) | **MISSING from `paper/figs/`** | 🔴 |

**Audit verdict: MISSING.** Paper acknowledges as L4. Either (a) run a
reduced-scale version (10K→1M, which would fit H100 budget) to produce Fig 5,
or (b) explicitly scope the paper to "up to 1M" and delete Fig 5 references.

### E6 — GAC ablation

| Spec | Shipped | Status |
|---|---|---|
| GAC without residuals | ✅ | 🟢 |
| GAC with oracle routing | ✅ | 🟢 |
| GAC with random routing | ✅ | 🟢 |
| GAC with fixed thresholds | ✅ | 🟢 |
| BGE-large + MiniLM × 2 corpora | 6 corpora, **BGE-large only** | 🟡 |
| **Shipped rows** | 126 (7 routers × 6 corpora × 3 seeds) | 🟢 |

**Audit verdict: DONE for BGE-large.** Encoder sweep for ablation not done
but E8 demonstrates encoder-universality of ordering — acceptable.

### E7 — Downstream LLM evaluation

| Spec | Shipped | Status |
|---|---|---|
| 10K questions × 6 strategies × 5 passages | **NOT RUN** | 🔴 |
| NQ/HotpotQA/PopQA | NQ + HotpotQA embeddings exist, **but question/answer texts not persisted by builders** | 🔴 |
| vLLM serving Llama-3.1-70B | Image built in `modal_app/app.py`; no entrypoint run | 🔴 |
| EM/F1 metrics | Implementation present in `e7_downstream_llm.py` but untested | 🔴 |
| Figure 6 | **MISSING** | 🔴 |

**Audit verdict: MISSING.** Paper acknowledges as L3. **This is the most
reviewer-visible gap** — SPEC §0.2.3 explicitly calls this out as required
for a NeurIPS memory paper. Either (a) rebuild NQ/HotpotQA corpora keeping
QA text and run E7 (8B model for cost), or (b) pivot to an honest "retrieval
downstream" framing using Recall@5 from E4 as the proxy (already in paper).

### E8 — DRM × encoders

| Spec | Shipped | Status |
|---|---|---|
| All 6 encoders × 7 strategies × 24 lists | 6 encoders × 5 strategies × 3 seeds × 24 lists | 🟢 |
| GAC included | ✅ | 🟢 |
| **Shipped rows** | 108 | 🟢 |

**Audit verdict: DONE.** Cleanest block; encoder-universality finding is
solid, and the Nomic outlier matches theorem prediction.

### E9 — Temporal pipeline with MRR

| Spec | Shipped | Status |
|---|---|---|
| Re-run RESEARCH_REPORT Exp 6 with MRR@20, Recall@10 | 🔴 **NOT RUN, NOT SCAFFOLDED** | 🔴 |

**Audit verdict: MISSING.** Unmentioned in paper. Scoped to follow-up or
quietly dropped. Low risk if we declare the scope as "static memory
consolidation" and defer temporal pipelines.

---

## Section 5 — Data commitments

| Dataset | Spec | Shipped | Status |
|---|---|---|---|
| wiki-sent-10k | ✅ | ✅ | 🟢 |
| wiki-sent-1m | Required for E2,E3,E4,E6,E7 | Only 10k-scale sections (~10K) | 🔴 |
| wiki-sent-10m | Required for E5 | 🔴 | 🔴 |
| c4-sent-1m | E2 | 🔴 **skipped entirely** | 🔴 |
| arxiv-abs-1m | E2 | arxiv_titles (smaller) | 🟡 |
| msmarco-passage | E2,E3,E7 | Built (subset) | 🟡 |
| nq-passages (100K) | E4,E7 | Built but **no QA text persisted** | 🟡 |
| hotpot-passages (100K) | E7 | Same as NQ | 🟡 |
| drm-24 (24 × 20 words) | E8 | ✅ | 🟢 |

**Audit verdict: PARTIAL.** The corpus geometry is represented (6 domains) but
the scale is 1–2 orders of magnitude smaller than committed for E2–E7.

---

## Section 7 — Figures and tables

| # | Spec figure | Shipped | Status |
|---|---|---|---|
| F1 | Conceptual trilogy / identity–coverage axes / GAC router | `fig1_theorem_regimes.pdf` (regime heatmap, not conceptual) | 🟡 |
| F2 | E1 cap-volume bound validation (16 subplots) | Part of `fig1_theorem_regimes.pdf` (different layout) | 🟢 |
| F3 | Template vs real collapse (bar chart, 6 encoders) | `fig2_strategy_sweep.pdf` | 🟡 |
| F4 | Identity–coverage scatter (6 strat × 6 enc × 3 dom) | 🔴 **MISSING** | 🔴 |
| F5 | Scale study (10K→10M) | 🔴 **MISSING (E5 not run)** | 🔴 |
| F6 | Downstream LLM (EM/F1 bars) | 🔴 **MISSING (E7 not run)** | 🔴 |
| T1 | GAC vs learned baselines (E3) | `fig3_learned_baselines.pdf` — shipped as figure, not table | 🟡 |
| T2 | GAC ablation (E6) | `fig6_ablation.pdf` — shipped as figure, not table | 🟡 |
| — | Encoder universality (E8) | `fig4_encoder_universality.pdf` | 🟢 |

**Shipped figures**: fig1, fig2, fig3, fig4, fig6 (5 total).
**Missing**: fig5 (scale), fig-identity-coverage-scatter (F4),
fig-downstream-LLM (F6).
**Tables 1/2**: present as figures, not formatted as tables.

---

## Section 10 — Repo deliverables

| Item | Shipped | Status |
|---|---|---|
| `gac/` pip-installable | ✅ | 🟢 |
| `experiments/` one file per block | 8 files (E1–E8; E9 missing) | 🟡 |
| `modal/` orchestrator + image | ✅ `modal_app/app.py` | 🟢 |
| `data/` build scripts per corpus | ✅ 7 builders | 🟢 |
| `notebooks/` | Empty dir; figures generated by `scripts/make_figures.py` | 🟡 |
| `paper/` LaTeX source | ✅ 681 lines | 🟢 |
| S3 public cache | 🔴 (Modal volume `gac-data`, not public S3) | 🔴 |
| Blog post | 🔴 | 🔴 (low priority, not submission-critical) |

---

## Consolidated gap list (prioritized)

### Tier 1 — Submission-critical (block NeurIPS claim)
1. **E7 downstream LLM evaluation**. Either run a minimal version
   (500 questions × 4 strategies × Llama-3.1-8B) or restructure the paper
   around retrieval downstream proxy only. Currently declared as L3.
2. **F4 identity–coverage scatter figure.** Plotable from existing E2 data —
   just need a new figure script. This is the visual anchor for C2.
3. **Table format for T1/T2.** Reviewers expect tables for ablation + baseline
   head-to-head; convert current figures to tables or add both.

### Tier 2 — Strong improvement (defensible without but better with)
4. **E5 scale study at reduced scope** (10K → 1M, BGE-large only).
   ~3 H100-hours on Modal. Produces F5.
5. **Real paraphrase queries in E4** via an LLM rather than embedding noise.
   Closes L2.
6. **No-consolidation baseline in E2** (identity row for the uncompressed store).
7. **Compression ratio sweep** in E2/E3 — SPEC asks for {2,5,10,40,200}.
   Currently 2–5× only.

### Tier 3 — Nice-to-have / scoping
8. **E9 temporal MRR** — decide to drop explicitly or run.
9. **OpenAI embedding** in E2 — may be cheaper than re-running local encoders.
10. **Noisy retrieval (σ sweep)** in E2 — reviewers rarely ask; can skip.
11. **Public S3 mirror** — do at submission.

### Tier 4 — Already-documented limitations (no action needed)
- L1 $c_1$ calibration in extreme-coherence corner (honest theoretical gap).
- Nomic outlier on DRM (supports theorem — we want this).

---

## Recommended session-4 action plan

Given ~$450 of the $500 soft cap remaining and the mid-May deadline:

**Must do this session (Tier 1):**
- [ ] Add F4 (identity–coverage scatter) — free, 30 min of plotting.
- [ ] Reformat E3/E6 results as tables T1/T2 in addition to figures — 1 hr.
- [ ] Run E7 minimal: rebuild NQ with QA text persisted + run 500q × 4 strat
  × Llama-3.1-8B. Budget ~$30 Modal. Updates paper §Results and removes L3.

**Should do (Tier 2):**
- [ ] Run E5 at 10K/100K/1M (drop 10M) on BGE-large only. Budget ~$15 Modal.
  Produces F5 and removes L4.
- [ ] Add no-consolidation row to E2. Budget ~$5 Modal.

**Explicit descope (document in paper):**
- E9 temporal: out of scope; cited as future work.
- OpenAI embedding: cited as reproducibility exercise deferred.
- Blog post: submission-adjacent, not submission-blocking.

This puts total session-4 spend at ~$50, well under the $500 guardrail,
and converts the paper from 2 declared limitations (L3, L4) down to 0,
plus lands the two missing main-paper figures (F4, F5) and converts the
ablations into reviewer-readable tables.
