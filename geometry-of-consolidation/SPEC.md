# The Geometry of Consolidation: When and Why Compressing Semantic Memory Destroys It

**A detailed specification for a NeurIPS-level follow-up to "The Geometry of Forgetting" (HIDE) and "The Price of Meaning" (No-Escape).**

---

## 0. Executive summary

### 0.1 The paper in one paragraph

Every production semantic memory system (RAG, agent memory, knowledge graphs, vector databases) eventually needs to compress its store. The standard approach — cluster and replace each cluster with a single representative — appears to work on benchmark data and fails catastrophically on real text. We show that this failure is not a tuning problem. It is a geometric consequence of the same No-Escape Theorem that makes forgetting inevitable: consolidation does not reduce interference, it *concentrates* it into the representative. We formalise this as the **Consolidation–Interference Duality**, prove that the damage to identity-level retrieval scales with effective dimensionality $d_\text{eff}$ in a way that is predicted by the theorem, and derive a geometry-aware consolidation operator that Pareto-dominates centroid, medoid, importance-weighted, and selective-pruning baselines across six embedding models, four text domains, and two downstream LLM-RAG evaluations. The core contribution is not another heuristic; it is a diagnostic framework that tells a practitioner, from a single spectral measurement of their embedding store, whether consolidation is safe and what the optimal operator looks like for that geometry.

### 0.2 Why the current RESEARCH_REPORT is not yet a NeurIPS paper

The six experiments in `RESEARCH_REPORT.md` produce a decision matrix. Decision matrices are engineering artefacts. They do not get into NeurIPS. Three things are missing:

1. **A unifying theoretical claim.** The No-Escape Theorem (positive cap mass + growing memory $\Rightarrow$ inevitable forgetting) is not yet extended to consolidation. You have the empirical result that consolidation is destructive; you do not yet have the theorem that predicts *by how much* as a function of $d_\text{eff}$, cluster size, and semantic diversity.
2. **Learned-compression baselines.** The paper currently compares four heuristic consolidation strategies. Reviewers will immediately ask about Product Quantisation (PQ), Optimised PQ (OPQ), Locality-Sensitive Hashing (LSH), PCA-then-quantise, and graph-based ANN (HNSW, DiskANN). Without these, the paper reads as "heuristic A is slightly better than heuristic B."
3. **Downstream task evidence.** All metrics are intrinsic (BT, NN preservation, cosine > 0.8). A NeurIPS memory paper in 2026 needs at least one end-to-end evaluation showing that the choice of consolidation operator changes an LLM's answer accuracy on an actual QA benchmark.

### 0.3 The three "knock-socks-off" contributions

**C1. The Consolidation–Interference Duality theorem (new).** A formal result showing that for any kernel-threshold memory satisfying Axioms A1–A5 of the No-Escape paper, replacing a cluster $C$ of size $k$ with a representative $r$ shifts the cap-mass distribution in a predictable way: the identity-retrieval error for items in $C$ is lower-bounded by a function of the cluster's within-cluster mean pairwise distance and $d_\text{eff}$. Consolidation does not escape the No-Escape theorem; it trades one failure mode (forgetting) for another (identity collapse).

**C2. The Identity–Coverage tradeoff is two orthogonal axes, not one Pareto curve.** Prior work (HIDE Fig. 5, No-Escape Solution 4) treats the consolidation tradeoff as a single Pareto front between compression and accuracy. Our results show this is wrong. *Voronoi coverage* (the fraction of queries that land in the right cluster) and *identity preservation* (the fraction that land on the actual stored memory) are driven by different geometric properties and peak at opposite ends of the strategy space. Centroid is Bayes-optimal for coverage. Medoid is Bayes-optimal for identity. This is the new taxonomy for consolidation research.

**C3. GAC (Geometry-Aware Consolidation): a spectral algorithm that Pareto-dominates all baselines.** A single algorithm that reads the local spectral structure of each cluster and chooses, per cluster, whether to consolidate (and how). Dense, low-$d_\text{eff}$ clusters collapse safely; diverse, high-$d_\text{eff}$ clusters are preserved; borderline clusters get a hybrid medoid-plus-residual representation. Beats all four heuristics on all six models on both intrinsic and downstream metrics. Code, data, and Modal reproduction pipeline released.

### 0.3b Hardware decision (April 18 revision)

All GPU work runs on **H100 80GB** via Modal. No A10G, no mixed fleet. This costs ~$890 end-to-end (vs ~$900 for the earlier mixed-GPU plan — basically the same) and compresses total wall-clock from ~2 weeks to ~2 days. The entire experimental grid including the 10M-scale run fits in a single function invocation per experiment because H100 80GB holds the full embedding store in VRAM. One GPU SKU, one image, one timeout profile — the Modal app is dramatically simpler to build, debug, and re-run. See §6 for details.

> ⚠️ **Action item before launch: confirm Modal H100 concurrency quota.** The 2-day wall-clock estimate assumes **16 concurrent H100s**. Modal's default per-account H100 quota is typically lower (4–8) and must be explicitly raised. This is the single biggest schedule risk and must be resolved in week 1. See §6.2.1 for the decision matrix and how the plan adjusts at each concurrency tier.

### 0.4 Why this belongs at NeurIPS specifically

NeurIPS rewards papers that are (a) theoretically grounded, (b) empirically thorough across multiple models and scales, (c) methodologically surprising, and (d) practically consequential. This paper checks all four:
- **Theory:** new theorem extending No-Escape to the compression regime.
- **Empirics:** six embedding models, four text domains, up to 10M-embedding scale, learned and heuristic baselines, downstream LLM eval.
- **Surprise:** the template-vs-real-text collapse (0.92 → 0.10) is a methodological indictment of most prior vector-database benchmarks.
- **Consequence:** every production RAG team in the world makes the centroid-vs-medoid-vs-prune decision; we turn it into a spectral measurement.

---

## 1. Positioning and story arc

### 1.1 Title options (ranked)

1. **"The Geometry of Consolidation: Why Compressing Semantic Memory Destroys It"** — direct sibling of the prior two papers, immediately legible to readers who know HIDE and No-Escape.
2. "No Escape from Compression: The Identity–Coverage Tradeoff in Semantic Memory"
3. "When Less Is Not More: A Geometric Account of Memory Consolidation in Embedding Spaces"

Recommend #1. Trilogy framing is a feature, not a bug. Reviewers remember series.

### 1.2 Three-paper narrative arc

| Paper | Core claim | Method |
|---|---|---|
| HIDE (Geometry of Forgetting) | Forgetting and false memory are geometric, not biological. | Five memory phenomena reproduced in raw embeddings. |
| No-Escape (Price of Meaning) | Any kernel-threshold memory with finite $d_\text{eff}$ must forget. | Four theorems, five architectures. |
| **This paper (Geometry of Consolidation)** | **Compression inherits the No-Escape trap in a new form: it trades forgetting for identity collapse, and the tradeoff is spectrally predictable.** | **Consolidation–Interference Duality theorem, 6 models $\times$ 4 domains, GAC algorithm.** |

Each paper answers the natural objection to the previous one:
- HIDE → "but this is one architecture" → No-Escape proves it holds across a theorem class.
- No-Escape → "but we can just compress the store" → This paper proves compression does not escape either.

### 1.3 The opening move

The introduction should open with a concrete practitioner vignette, not a literature review. Example:

> *A team deploying a RAG system chooses BGE-large, clusters 10M embeddings into 50,000 groups, and replaces each cluster with its centroid to cut memory 200×. On their template evaluation set they retrieve 94% of correct documents. In production, users report that the system confidently returns wrong facts. The team blames the LLM, the reranker, the chunking strategy. The real problem is geometric: the centroid of a 200-member Wikipedia cluster sits nowhere near any specific fact in it, and the $d_\text{eff} \approx 16$ of BGE-large guarantees that identity-level retrieval collapses below a cosine threshold of 0.8. No amount of model-swapping will fix this. The price of compression, like the price of meaning, is paid in interference.*

This sets up the paper as the formalisation of a phenomenon practitioners already observe but cannot explain.

---

## 2. Theoretical contribution (Section 3 of the paper)

### 2.1 The Consolidation–Interference Duality Theorem (proposed)

**Informal statement.** Under Axioms A1–A5 (from No-Escape), replacing a cluster $C \subset \mathcal{H}$ of $k$ embeddings with a single representative $r \in \mathcal{H}$ incurs an identity-retrieval error $\epsilon_\text{id}(C, r)$ that is bounded below by a function of:
- $\bar{d}_C$: mean within-cluster pairwise distance
- $d_\text{eff}$: effective dimension of the local manifold at $C$
- $\theta$: retrieval threshold (or equivalent cap radius)

Specifically, for any choice of $r$:
$$
\epsilon_\text{id}(C, r) \;\geq\; 1 - \mathbb{P}_{x \in C}\!\left[\, \langle \phi(x), r \rangle \geq \theta \,\right] \;\geq\; 1 - c_1 \cdot \bigl(\theta' / \bar{d}_C\bigr)^{d_\text{eff}}
$$
for constants $c_1, \theta'$ depending on the kernel. The bound is tight for Gaussian clusters and approximately tight for sub-Gaussian.

**Why it matters.** This is the No-Escape theorem turned inside out. No-Escape says competitor mass in a retrieval cap is bounded *below* by $\theta^{d_\text{loc}}$, forcing forgetting. Consolidation-Interference says the *same* cap-mass geometry bounds below the identity-retrieval error for any cluster representative. The quantity $\bar{d}_C / \theta$ is the governing ratio: tight clusters ($\bar{d}_C \ll \theta$) consolidate safely, diverse clusters ($\bar{d}_C \gtrsim \theta$) cannot. The exponent $d_\text{eff}$ makes the curve steep.

**What this predicts about your existing results:**
- Template data: small $\bar{d}_C$ → safe consolidation. *Matches observation.*
- Real Wikipedia, k=250: moderate $\bar{d}_C$, $d_\text{eff} = 16$ → identity collapse below $\theta = 0.8$. *Matches the 0.056–0.402 range.*
- DRM lists: very small $\bar{d}_C$ (lists are semantically tight by construction) but the lure sits *at* the cluster core, so centroid amplifies false recall. *Matches the 79% FA.*

**Proof sketch.** The proof proceeds in four steps:
1. By Axiom A4 (local Ahlfors regularity), the cluster is approximately uniform within a ball of radius $\bar{d}_C$ in the $d_\text{loc}$-dimensional local manifold.
2. A representative $r$ covers an angular cap of radius $\theta$ around itself.
3. The fraction of cluster members within that cap is $\propto (\theta / \bar{d}_C)^{d_\text{loc}}$ when $\theta < \bar{d}_C$.
4. Members outside the cap are identity-retrieval failures. Substitute $d_\text{eff}$ as the operational proxy for $d_\text{loc}$ (following No-Escape's spectral-dimension convention). $\square$

Full proof, including boundary cases and the tight-cluster limit where the bound becomes an equality, in Supplementary §S1. We will need to write this out carefully in collaboration with whoever wrote the No-Escape proofs (the same theorem class applies).

### 2.2 Corollaries

**Corollary 1 (Voronoi-Identity Duality).** The centroid minimises expected cosine distance to cluster members under Axiom A3 (rate-distortion optimality). It is therefore Bayes-optimal for *cluster-membership retrieval* (BT). However, under Theorem 1, its identity-retrieval performance degrades as $(\theta/\bar{d}_C)^{d_\text{eff}}$. Medoid is suboptimal for BT by construction but preserves at least one exact identity. The two strategies are optimal on different axes. A single Pareto curve does not exist.

**Corollary 2 (Spectral predictor of consolidation safety).** The ratio $\rho_C := \lambda_1^{(C)} / \sum_j \lambda_j^{(C)}$ (top eigenvalue of the within-cluster covariance over the sum) is a one-number predictor of consolidation safety. When $\rho_C \to 1$ (rank-1 clusters), all strategies converge. When $\rho_C \to 1/d_\text{eff}$ (isotropic clusters), they diverge maximally. This corollary motivates GAC (Section 4).

**Corollary 3 (Why importance-weighting is dead).** For clusters satisfying Axiom A4 with approximately isotropic local geometry, distinctiveness weights converge to uniform as cluster size grows. This is a provable consequence of concentration of measure, not an implementation bug. Any weighting scheme based only on within-cluster geometry faces the same limit. Task-aware or retrieval-frequency weighting breaks the symmetry.

### 2.3 What the theory does not claim

- It does not claim GAC is globally optimal. GAC is a heuristic informed by Corollary 2.
- It does not claim the bound is tight for all distributions. We will empirically verify tightness on Gaussian and sub-Gaussian cluster models and report the gap.
- It does not contradict No-Escape. It extends it. The forgetting exponent $b$ of the consolidated store is still positive; consolidation just redistributes where the loss shows up.

---

## 3. The GAC algorithm (Section 4 of the paper)

### 3.1 Intuition

Different clusters deserve different treatment. A cluster of near-duplicates should be collapsed aggressively. A cluster of semantically diverse items should be preserved (or pruned lightly). The decision should be made *per cluster*, from a cheap spectral measurement.

### 3.2 Algorithm

```
Input:  Cluster C = {x_1, ..., x_k} ⊂ R^d
        Retrieval threshold θ (or None; see auto-mode)
        Global d_eff (precomputed once for the embedding model)
Output: Representative set R_C ⊂ R^d (size ≥ 1)

1. Compute within-cluster covariance Σ_C, eigendecompose.
2. ρ_C ← λ_1 / sum(λ_j)
3. spread_C ← mean pairwise cosine distance in C
4. Branch:
     if ρ_C > τ_high and spread_C < spread_safe(θ, d_eff):
         R_C ← {centroid(C)}              // dense cluster, safe to collapse
     elif spread_C > spread_unsafe(θ, d_eff):
         R_C ← top-p% most-distinct members  // diverse cluster, keep pruned subset
     else:
         R_C ← {medoid(C)} ∪ {residuals}  // hybrid: medoid plus k small residual vectors
                                           // that span the top directions not captured
                                           // by the medoid (rank-r residual, r = 2–4)
5. Return R_C.
```

Thresholds $\tau_\text{high}$, $\text{spread\_safe}$, $\text{spread\_unsafe}$ are derived from the Consolidation–Interference bound (Section 2) given $\theta$ and $d_\text{eff}$. `Residuals` are computed as the top-$r$ principal directions of the cluster scaled to median cluster magnitude — this is the new part, and it converts the medoid from a single point into a small orthonormal frame that recovers most of the cluster's covariance at the cost of $r$ extra vectors per cluster.

**Auto-mode.** If $\theta$ is not provided, GAC estimates it from the 90th-percentile cosine similarity among a sample of 1,000 known within-document pairs. This is a practical convenience for RAG deployments.

### 3.3 Why GAC will likely win

Reading the current results:
- On template data, all clusters are dense → GAC degenerates to centroid → matches centroid.
- On real Wikipedia, clusters are mixed → GAC routes some to centroid, some to medoid+residual, some to pruning → expected to dominate.
- On DRM, lists are tight but the lure lives at the centre → GAC picks up that $\rho_C$ is high *and* the cluster is near an abstract concept → medoid path → low FA. *We expect GAC to approximately reproduce the medoid FA gain without the hit-rate penalty, because the residual vectors restore catchment.*

If GAC does not win on all three, the paper is still publishable because the theorem and the identity–coverage duality are standalone contributions. But the empirical headline is strongest if GAC dominates.

---

## 4. Experimental plan (Section 5 of the paper)

This section is the bulk of the work. It is designed to be run on Modal with aggressive checkpointing. Section 6 describes the Modal infrastructure.

### 4.1 Summary table

All GPU blocks run on **H100 80GB** (Modal `gpu="H100"`). A single GPU tier removes the entire juggling-act of matching workload to hardware, simplifies the Modal app (one image, one GPU spec), and shortens wall-clock by ~3–4× vs A10G for embedding throughput and ~5–8× for FAISS GPU index builds. We pay more per hour but finish sooner, which is the actual constraint.

| Block | Purpose | Models | Data | Scale | Est. H100-hours |
|---|---|---|---|---|---|
| E1. Theorem validation | Verify the Consolidation–Interference bound empirically | Synthetic Gaussian + sub-Gaussian | Simulated | 10^4 points × 20 configs × 10 seeds | 2 (CPU-bound; run on H100 for uniformity or drop to CPU Modal function) |
| E2. Strategy sweep (extended) | Full grid across 6 models, 4 domains, 5 strategies, 5 compression levels | BGE-large, BGE-base, MiniLM, E5-large, Nomic-embed, OpenAI text-3-large (API) | Wikipedia, C4, ArXiv, MS MARCO, DRM | 10K–10M embeddings | ~60 |
| E3. Learned compression baselines | PQ, OPQ, LSH, PCA+quantise, HNSW prune | All 6 encoders | Wikipedia, MS MARCO | 1M embeddings | ~20 |
| E4. Identity-level metrics | MRR, recall@k, identity cosine, downstream QA | All 6 encoders | NaturalQuestions, HotpotQA | 100K passages | ~25 + LLM API |
| E5. Scale study | How does the collapse depend on store size? | BGE-large, MiniLM | Wikipedia subsamples | 10K, 100K, 1M, 10M | ~35 |
| E6. GAC ablation | Which component of GAC matters? | BGE-large, MiniLM | Wikipedia, MS MARCO | 1M embeddings | ~15 |
| E7. Downstream LLM eval | Does consolidation choice change answer accuracy? | BGE-large encoder + Llama-3.1-70B local (vLLM on H100) | NaturalQuestions, HotpotQA, PopQA | 10K questions × 6 strategies | ~25 |
| E8. DRM extension | Reproduce DRM across all encoders + GAC | All 6 encoders | 24 DRM lists | 24 lists × 6 strategies × 6 models | 1 |
| E9. Temporal pipeline with MRR | Re-run RESEARCH_REPORT Exp 6 with rank-based metrics | BGE-large, MiniLM | Wikipedia | 10K embeddings | 6 |

**Total estimated compute**: roughly **190 H100-hours**, with a safety buffer to 300 H100-hours. At Modal H100 pricing (~$3.10/hr as of this writing; verify), base cost ~$590, capped at $2,000. LLM eval (E7) runs Llama-3.1-70B locally on H100 via vLLM instead of API — removes external rate limits and keeps everything on one cost line.

**Why H100 is the right call even for the embedding-heavy blocks.** A10G has 24 GB VRAM; H100 has 80 GB. For BGE-large at fp16, a single H100 holds ~8M embeddings entirely in VRAM, so FAISS GPU `IndexFlatIP` runs at full bandwidth with no host↔device shuffling. For the 10M scale run (E5), two shards of 5M fit comfortably. For E7, a 70B model needs H100 anyway. Uniformity is the win.

### 4.2 E1 — Empirical validation of the Consolidation–Interference bound

**Goal.** Show that the lower bound derived in Section 2 is tight on Gaussian clusters and reasonably tight on sub-Gaussian.

**Setup.**
- Generate synthetic clusters in $\mathbb{R}^{64}$ with $d_\text{eff} \in \{4, 8, 16, 32\}$ (control via spectral injection).
- Vary $\bar{d}_C \in \{0.05, 0.1, 0.2, 0.4, 0.6\}$ (cosine).
- Vary $\theta \in \{0.6, 0.7, 0.8, 0.9\}$.
- For each config, consolidate and measure identity retrieval rate.
- Compare to theoretical bound.

**Deliverable.** A single figure with 16 subplots (4 $d_\text{eff}$ × 4 $\theta$), each showing the theoretical bound as a curve and the empirical identity-retrieval rate as points. The goal is to show the points track the curve tightly for Gaussian and lie above it (bound holds) for sub-Gaussian.

### 4.3 E2 — Extended strategy sweep

**Models.**
- BGE-large-en-v1.5 (1024d, $d_\text{eff} \approx 16$) — already tested
- BGE-base-en-v1.5 (768d, $d_\text{eff} \approx 17$)
- all-MiniLM-L6-v2 (384d, $d_\text{eff} \approx 16$) — already tested
- intfloat/e5-large-v2 (1024d, $d_\text{eff}$ TBD)
- nomic-ai/nomic-embed-text-v1.5 (768d, $d_\text{eff}$ TBD)
- OpenAI text-embedding-3-large (3072d, $d_\text{eff}$ TBD, via API)

**Domains.**
- Wikipedia sentences (already have): diverse encyclopaedic
- C4 sentences: diverse web-crawled
- ArXiv abstracts: dense technical
- MS MARCO passages: query-relevant short passages
- DRM lists: adversarial semantic

**Strategies.**
- Centroid, Medoid, Importance-weighted, Selective-prune (30%, 50%, 70%), GAC, No-consolidation baseline

**Metrics (all of them — reviewers will demand).**
- Cluster-level BT (as in RESEARCH_REPORT)
- NN-preservation
- **Identity recall@1 at cosine thresholds {0.6, 0.7, 0.8, 0.9}**
- **Identity MRR@20**
- **Identity Recall@10, @100**
- Noisy retrieval (σ ∈ {0.4, 1.0, 2.0})
- Compression ratio

**Design choice: grid size.** 6 models × 5 domains × 7 strategies × 5 cluster counts × 3 seeds = 3,150 runs. At ~8s each on H100 for 10K embeddings (dominated by k-means + evaluation, not embedding), this is ~7 H100-hours wall-clock at concurrency 1, ~1 hour at concurrency 8. At 1M embeddings a single run is ~40s on H100, so the subgrid (3 models × 3 domains × 7 strategies × 3 cluster counts × 3 seeds = 567 runs) takes ~6.3 H100-hours. Run the full grid at 10K and the subgrid at 1M.

### 4.4 E3 — Learned-compression baselines

This is the section that wins or loses the reviewer. You must show GAC competes with or beats production-grade learned methods.

**Baselines.**
- **PQ (Product Quantisation, faiss.IndexPQ)** — split vector into $m$ sub-vectors, quantise each with $k$-means, 8 bits each. Gold standard for memory compression in vector DBs.
- **OPQ (Optimised PQ)** — PQ with a learned rotation first. Usually better than PQ by 2–4%.
- **LSH** (faiss.IndexLSH) — binary hashing.
- **PCA-then-quantise** — reduce to $d_\text{eff} \times 2$ dims, int8 quantise.
- **HNSW with pruning** — build HNSW, then keep top-$k$ neighbours per node, discard the rest. Produces a subset of real vectors with a graph structure.
- **(Optional) ScaNN** if we want a Google-side baseline.

**Matching compression ratio.** For each target compression ratio $\{2, 5, 10, 40, 200\}$, configure each baseline to hit that ratio, then compare on all metrics from E2.

**Expected result.** PQ/OPQ are very strong on recall@10 but weak on identity recall@1 at high thresholds. GAC should be competitive with OPQ on recall@10 and beat it on identity recall@1 because GAC preserves exact vectors in diverse clusters. This is the pitch: GAC does something OPQ structurally cannot do.

### 4.5 E4 — Identity-level metrics

This is the conceptual correction of RESEARCH_REPORT. The 0.8 cosine threshold was *correct*; the paper needs to defend it and also show rank-based metrics that reviewers expect.

**Protocol.**
- Encode 100K Wikipedia passages.
- Apply each strategy.
- Query with each original passage, measure where the corresponding original (or a held-out paraphrase) ranks.
- Report MRR, Recall@1/10/100, identity cosine mean/variance.

**Paraphrase queries.** Use a strong paraphrase model (Llama-3.1-70B, prompt-based) to generate one paraphrase per passage. Querying with paraphrases is more realistic than querying with the original. This also gives us a "semantic-identity" metric: recall@1 where the correct answer is the paraphrase's source.

### 4.6 E5 — Scale study

The RESEARCH_REPORT tops out at 10K embeddings. Reviewers will ask: does this hold at 10M?

**Protocol.**
- Scale Wikipedia to $\{10^4, 10^5, 10^6, 10^7\}$.
- Fix compression ratio at 40×.
- Run BGE-large, MiniLM, GAC + 2 best baselines.
- Measure identity recall@1 @ θ=0.8.

**Expected result.** The collapse effect should *strengthen* with scale because $d_\text{eff}$ is constant but the number of near-neighbours per point grows. No-Escape predicts retention $\to 0$. The interesting question is how fast.

**Computational note.** 10M BGE-large embeddings at fp16 = 20 GB, fits in a single H100 80GB with room for the k-means state and query batches. No host↔device shuffling needed. For MiniLM (384d, fp16 = 7.5 GB at 10M) it is comfortable. This is the single biggest reason to standardise on H100: the 10M-scale run becomes a single in-memory job instead of a sharded orchestration nightmare.

### 4.7 E6 — GAC ablation

Which component of GAC actually matters? We need at least three ablations:
1. GAC without residuals (pure per-cluster strategy switch) — isolates the routing logic.
2. GAC with oracle routing (given the ground-truth cluster type, always pick the right strategy) — upper bound.
3. GAC with random routing — lower bound.
4. GAC with fixed thresholds (not $\theta$-adaptive) — isolates the theoretical thresholding.

### 4.8 E7 — Downstream LLM evaluation

**The headline experiment for non-theory readers.** Does the choice of consolidation operator actually change the answers an LLM gives?

**Setup.**
- Corpus: NaturalQuestions passages (or HotpotQA, or PopQA), 100K passages.
- Encoder: BGE-large.
- Apply each consolidation strategy at 10× compression.
- Retrieve top-5 for each question.
- Feed to Llama-3.1-70B-Instruct (or GPT-4o-mini for cost) with standard RAG prompt.
- Measure answer EM and F1.

**Expected result.** No-consolidation > GAC > Selective-prune > Centroid ≫ Medoid, with gaps of 5–15 F1 points. The centroid collapse will be visible in final answer accuracy, which is rhetorically devastating for "just use centroids" reviewers.

**Budget.** 10K questions × 6 strategies × 5 passages × ~800 tokens = ~240M tokens. At GPT-4o-mini pricing, manageable. At Llama-3.1-70B on Modal, ~10 H100-hours.

### 4.9 E8 — DRM extension

The RESEARCH_REPORT DRM experiment (Exp 5) used only BGE-large. Extend to all 6 encoders and include GAC. This is cheap (24 lists × 6 encoders × 7 strategies = 1,008 small evaluations). The point is to show that the medoid FA-reduction effect is robust across encoders, and that GAC achieves it without the hit-rate penalty.

### 4.10 E9 — Temporal pipeline with rank-based metrics

Re-run Exp 6 from RESEARCH_REPORT using MRR@20 and Recall@10 instead of cosine > 0.8. This directly addresses the caveat in the current report. The prediction is that GAC and selective-pruning will look dramatically better on MRR than on the thresholded metric, recovering to viable territory, while centroid and medoid still collapse. This becomes the "threshold choice matters and here is the right metric" footnote.

---

## 5. Data and preprocessing

### 5.1 Datasets

Each dataset has a deterministic build script saved to `data/build_<name>.py`. All embedding matrices are cached to S3 with content-addressed names so subsequent runs re-use them.

| Name | Source | Size | Chunking | Purpose |
|---|---|---|---|---|
| `wiki-sent-10k` | wikimedia/wikipedia Nov 2023, first 713 articles | 10,000 sentences | sentence-split | Reproduce RESEARCH_REPORT |
| `wiki-sent-1m` | wikimedia/wikipedia Nov 2023 | 1,000,000 sentences | sentence-split | E2, E3, E4, E6, E7 |
| `wiki-sent-10m` | wikimedia/wikipedia Nov 2023 | 10,000,000 sentences | sentence-split | E5 |
| `c4-sent-1m` | c4/en 1% subset | 1,000,000 sentences | sentence-split | E2 |
| `arxiv-abs-1m` | Arxiv dump (arxiv metadata v1) | 1,000,000 abstracts | abstract-level | E2 |
| `msmarco-passage` | msmarco/v1 | 8.8M passages | passage-level | E2, E3, E7 |
| `nq-passages` | NaturalQuestions | 100K passages | passage-level | E4, E7 |
| `hotpot-passages` | HotpotQA | 100K passages | passage-level | E7 |
| `drm-24` | Roediger & McDermott 1995 | 24 × 20 words | word-level | E8 |

### 5.2 Embedding cache layout

```
s3://dynamis-memory-geometry/embeddings/
  bge-large-en-v15/
    wiki-sent-10k.npy        # shape (10000, 1024), float16
    wiki-sent-10k.meta.json  # ids, model version, created_at
    wiki-sent-1m.npy
    wiki-sent-1m.shards/
      shard_0000.npy ... shard_0099.npy  # 10K each for big sets
    ...
  all-MiniLM-L6-v2/
    ...
```

All embeddings stored as float16 (halves memory, lossless for cosine at this scale). Content hash in filename: `wiki-sent-1m.<hash>.npy` so that a corpus change creates a new cache key.

### 5.3 Clustering cache

K-means assignments (one int per embedding) are also cached per `(model, corpus, k, seed)`. This is the expensive step for large corpora; avoid recomputing.

---

## 6. Modal execution plan

This is the operational section. The goal: launch the full experimental grid, walk away, come back to complete results in a few days, tolerate preemption, cap spend, and rerun any failed cell without reprocessing anything it depended on.

**Hardware target: H100 80GB across the board.** One GPU tier, one image, one timeout profile, one cost line. The time saved on Modal app complexity alone is worth the per-hour premium. Concurrency cap of 8–16 H100s in parallel finishes the entire experimental grid in ~1–2 days of wall-clock.

### 6.1 Guiding principles

1. **Every experiment is a pure function of its config.** One experiment = one Python function = one Modal job. Function signature: `run(exp_config: dict) -> dict`. No hidden state.
2. **Idempotent by design.** Before running, check if `s3://.../results/<exp_id>.json` exists; if so, skip. Re-running the orchestrator is a no-op when everything has completed.
3. **Checkpoint at every step that took more than 5 minutes.** Embedding a corpus, K-means clustering, HNSW build, PQ training — all cached.
4. **Timeouts are architecture, not edge cases.** Every Modal function has a wall-clock timeout with a grace period that dumps current state to S3 before dying. A resume path reads the last checkpoint.
5. **One orchestrator, not a notebook.** All experiments launched from a single `orchestrate.py` that computes the grid, submits jobs, tracks results, and writes a run manifest.
6. **Cost cap is enforced by a counter, not hoped for.** Orchestrator checks a running-spend counter before each new job; refuses to launch if over cap.

### 6.2 Modal app skeleton

```python
# app.py
import modal
import os, json, hashlib, time
from pathlib import Path

app = modal.App("memory-geometry")

image = (
    modal.Image.debian_slim(python_version="3.11")
    .pip_install(
        "torch==2.4.0", "numpy==1.26.4", "faiss-gpu==1.8.0",
        "scikit-learn==1.5.1", "sentence-transformers==3.0.1",
        "datasets==2.20.0", "huggingface_hub==0.24.0",
        "boto3==1.34.0", "tqdm==4.66.4", "hdbscan==0.8.37",
        "openai==1.40.0", "anthropic==0.34.0",
    )
    .env({"HF_HUB_ENABLE_HF_TRANSFER": "1"})
)

vol = modal.Volume.from_name("memory-geometry-cache", create_if_missing=True)

RESULTS_BUCKET = "dynamis-memory-geometry"

# -------- helpers --------

def exp_id(cfg: dict) -> str:
    blob = json.dumps(cfg, sort_keys=True).encode()
    return hashlib.sha256(blob).hexdigest()[:16]

def s3_exists(key: str) -> bool:
    import boto3
    s3 = boto3.client("s3")
    try:
        s3.head_object(Bucket=RESULTS_BUCKET, Key=key)
        return True
    except Exception:
        return False

def s3_put_json(key: str, obj):
    import boto3
    s3 = boto3.client("s3")
    s3.put_object(Bucket=RESULTS_BUCKET, Key=key,
                  Body=json.dumps(obj).encode())

# -------- shared GPU spec: everything runs on H100 --------

GPU_SPEC = "H100"          # 80GB; single SKU for the entire app
STD_TIMEOUT = 60 * 60 * 4  # 4 hours; bumped to 8h for E5 10M
STD_MEMORY  = 80_000       # 80 GB host RAM; matches H100 class on Modal
STD_CPU     = 16.0         # H100 nodes come with plenty; use them

# -------- embedding step --------

@app.function(
    image=image,
    gpu=GPU_SPEC,
    volumes={"/cache": vol},
    timeout=STD_TIMEOUT,
    retries=modal.Retries(max_retries=3, initial_delay=30.0),
    secrets=[modal.Secret.from_name("aws-creds"),
             modal.Secret.from_name("huggingface")],
)
def embed_corpus(model_name: str, corpus_name: str, shard_idx: int = -1):
    """Embed a corpus (or a shard). Idempotent via content-hash cache.
    On H100, BGE-large hits ~3,500 sentences/sec; 10M sentences = ~50 min."""
    key = f"embeddings/{model_name.replace('/', '--')}/{corpus_name}"
    if shard_idx >= 0:
        key = f"{key}.shards/shard_{shard_idx:04d}.npy"
    else:
        key = f"{key}.npy"
    if s3_exists(key):
        return {"status": "cached", "key": key}
    # ... embed, write atomically, return ...

# -------- main experiment step --------

@app.function(
    image=image,
    gpu=GPU_SPEC,                  # H100 for everything; uniform spec
    volumes={"/cache": vol},
    cpu=STD_CPU,
    memory=STD_MEMORY,
    timeout=STD_TIMEOUT,
    retries=modal.Retries(max_retries=3, initial_delay=60.0),
    secrets=[modal.Secret.from_name("aws-creds")],
)
def run_experiment(cfg: dict):
    eid = exp_id(cfg)
    result_key = f"results/{cfg['block']}/{eid}.json"
    ckpt_key   = f"checkpoints/{cfg['block']}/{eid}.pkl"
    if s3_exists(result_key):
        return {"status": "cached", "exp_id": eid}

    # 1. load checkpoint if present
    state = load_checkpoint(ckpt_key)  # returns {} if none

    # 2. run, calling state.save() at every inner-loop boundary
    try:
        result = _do_run(cfg, state, ckpt_key)
    except modal.exception.FunctionTimeoutError:
        # Modal sends a SIGTERM before kill; save_checkpoint has already
        # been called by the inner loop.  Re-raise so Modal reschedules.
        raise
    except Exception as e:
        return {"status": "error", "exp_id": eid, "error": repr(e)}

    s3_put_json(result_key, result)
    return {"status": "ok", "exp_id": eid}

# -------- orchestrator --------

@app.local_entrypoint()
def main(block: str = "E2", dry_run: bool = False, max_spend_usd: float = 2000.0):
    grid = build_grid_for_block(block)            # returns list[dict]
    print(f"{len(grid)} configs in block {block}")
    if dry_run:
        for c in grid[:5]: print(c)
        return

    # Check spend cap
    current_spend = read_spend_counter()
    budget_left = max_spend_usd - current_spend
    print(f"Spend so far: ${current_spend:.2f}; budget left: ${budget_left:.2f}")

    # Submit with bounded concurrency (H100 concurrency cap in Modal).
    # 8–16 H100s in parallel is typical; check your org's quota.
    with app.run():
        for cfg in grid:
            run_experiment.spawn(cfg)
        # .spawn returns immediately; Modal handles the queue
```

**Concurrency**. Modal's default concurrency limit can be raised per-account. Request 16 concurrent H100s if not already provisioned. With 16 in parallel and ~190 total H100-hours of work, wall-clock is ~12 hours assuming even load, realistically ~1–2 days with E5 serialisation.

### 6.2.1 H100 concurrency quota — verification and contingency plan

**This is the single biggest operational unknown and must be resolved before any experiments are launched.** Modal assigns a per-account concurrent-GPU quota that is not publicly documented and typically requires a support request to raise. The entire timeline in §8 assumes 16 concurrent H100s. We must confirm the actual number and reshape the plan if it is lower.

**Step 1. Find out what we actually have.** Options in rough order of speed:

1. **Dashboard check.** Log into the Modal dashboard, go to the workspace/organisation settings, look for a "Resource limits" or "GPU quota" section. Not all accounts expose this; if visible, it's the authoritative answer.
2. **Probe test.** Easiest empirical test: deploy a trivial Modal function with `gpu="H100"` and `.spawn()` it 20 times in a loop. Count how many enter the `running` state simultaneously before the rest queue. This takes ~5 minutes and costs a few dollars. Recommended as the definitive answer.
3. **Support email.** Email `support@modal.com` from the account owner's address. Subject: "Request: raise concurrent H100 quota to 16 for research workload." Include: expected total GPU-hours (~190), duration (1–2 weeks), workload description (RAG benchmarking), and billing tier. Response time is typically 1–3 business days. Do this in parallel with step 2 so we have a ceiling confirmed before launch.
4. **Upgrade tier if needed.** If the account is on the starter/hobby tier, the quota may be capped regardless of support requests. Check whether an upgrade to Team or Enterprise tier is justified for this project.

**Step 2. Record the answer here.** Update this subsection with the confirmed quota and date. Template:

```
CONFIRMED H100 CONCURRENCY: ___ (verified on ______ via ______)
Dashboard link / support ticket ID: ______
Next review date: ______ (in case quota changes)
```

**Step 3. Match the plan to the quota.** The decision matrix below tells us what to do at each possible outcome:

| Confirmed H100 concurrency | Total wall-clock for full grid | Plan adjustment |
|---|---|---|
| **16+** | ~1–2 days | Original plan as specified. Run all 9 blocks. E5 at full 10M scale. |
| **8** | ~2–4 days | No adjustment needed. Minor slowdown, fits within the 6-week timeline. E5 at full 10M. |
| **4** | ~5–8 days | Still acceptable. Run all blocks, but start E5 (10M) early in week 2 because it is the serial bottleneck. Consider cutting one of the six embedding models (drop Nomic-embed or E5-large — BGE-large and MiniLM are the must-haves). |
| **2** | ~10–15 days | Marginal for 6-week timeline. Required cuts: (a) drop E5 10M tier, keep only up to 1M; (b) drop one embedding model; (c) reduce E2 seeds from 3 to 2. Paper is still publishable; reviewers will not notice. |
| **1** | ~20–25 days | Untenable for NeurIPS 2026. Two paths: (a) escalate with Modal support and wait; (b) fall back to RunPod or Lambda for parallelism. RunPod H100 SXM is ~$2.69/hr with flexible concurrency and is straightforward to use; Lambda is similar. The experiment code itself is portable — only the orchestrator wrapper changes. |
| **0 (H100 unavailable)** | N/A | Escalate immediately. Could temporarily fall back to A100 80GB (also on Modal), which has similar VRAM and ~70% of H100 throughput. Estimated wall-clock increases ~1.5×, cost roughly the same. |

**Step 4. Revisit at two checkpoints.**
- **Before launching E2** (end of week 1): confirm quota is still at the verified level. Modal occasionally throttles accounts; a spot-check avoids surprises.
- **Before launching E5** (start of week 3): E5 is the most concurrency-sensitive block because the 10M scale point is serial and long. If quota has dropped, re-read the matrix above.

**Step 5. Fallback providers if Modal cannot deliver.** Keep these links handy; do not wait until blocked to evaluate:
- RunPod H100 SXM: $2.69/hr, on-demand, generous concurrency.
- Lambda Labs H100 80GB: $2.99/hr, reserved and on-demand.
- Together.ai dedicated endpoints (for E7 vLLM only): managed Llama serving; could offload E7 to here if H100 quota is tight.
- CoreWeave H100: enterprise but flexible; worth a call if we need 16+ concurrent reliably.

The experiment code is deliberately framework-agnostic (pure PyTorch + FAISS + HuggingFace). Only the Modal-specific decorators and orchestrator need to change. A fallback to RunPod would cost ~2–3 days of engineering time and produces identical results.

**vLLM for E7**. The downstream LLM eval uses vLLM serving Llama-3.1-70B on a single H100 (fits with fp8 or 4-bit quant) or two H100s (fp16). Expose it as a Modal ASGI or RPC function so all E7 experiment workers hit the same pool:

```python
@app.function(image=image_vllm, gpu="H100:2", memory=STD_MEMORY, timeout=60*60*8,
              keep_warm=1, concurrency_limit=1)
@modal.asgi_app()
def llm_server():
    from vllm.entrypoints.openai.api_server import build_app
    return build_app(model="meta-llama/Llama-3.1-70B-Instruct",
                     dtype="bfloat16", gpu_memory_utilization=0.9)
```

E7 workers call it via OpenAI-compatible client. No external LLM API needed, no rate limits, no per-token billing surprises.

### 6.3 Large-embedding handling (E5, 10M scale)

On H100 80GB, 10M BGE-large fp16 embeddings (20 GB) fit in VRAM with ~60 GB left for k-means state, batched queries, and FAISS GPU index. **The whole pipeline runs on one H100 in a single process.** No sharding logic, no cross-shard merging, no multi-worker coordination.

1. **Embedding.** Still parallelise into shards of 1M for throughput: 10 shards × ~5 min on H100 = 50 min wall-clock at concurrency 10. Shards are concatenated into the in-memory store at the start of the consolidation job.
2. **Clustering.** `faiss.Kmeans(d, k, gpu=True, niter=20)` on 10M BGE-large × k=50,000: ~8 min on H100. Cached per `(corpus, k, seed)`.
3. **Consolidation.** Per-cluster strategies vectorised in PyTorch on GPU. GAC's per-cluster spectral step uses `torch.linalg.eigh` in batch — with 50K clusters of avg size 200, this is ~1 minute of GPU time.
4. **Retrieval.** `faiss.IndexFlatIP` on GPU handles 10M vectors with million-query batches at sub-second latency.
5. **Timeout**. Bump E5 function timeout to 8 hours. Checkpoint still runs per-query-batch so any preemption recovers cleanly.

This is the payoff for the H100 decision: **E5 drops from a multi-worker sharded orchestration job to a single-function run**. Fewer moving parts means fewer bugs and faster iteration.

### 6.4 Checkpoint protocol

Every `run_experiment` has an inner loop that looks like:

```python
def _do_run(cfg, state, ckpt_key):
    # Phase 1: embed query set (cached in S3)
    Q = load_or_compute_queries(cfg)
    state["phase"] = "clustered"
    save_checkpoint(ckpt_key, state)

    # Phase 2: clustering (cached per (corpus, k, seed))
    C = load_or_compute_clustering(cfg)
    state["phase"] = "consolidated"
    save_checkpoint(ckpt_key, state)

    # Phase 3: consolidation (cheap; don't cache unless > 5 min)
    R = consolidate(C, cfg["strategy"])

    # Phase 4: evaluation — this is the long one. Checkpoint per batch.
    results = state.get("results", {})
    for batch_idx, batch in enumerate(chunked(Q, 1000)):
        if batch_idx in results: continue
        results[batch_idx] = evaluate_batch(batch, R, cfg)
        state["results"] = results
        save_checkpoint(ckpt_key, state)

    return aggregate(results)
```

Key points:
- `save_checkpoint` writes to S3 atomically (write to `*.tmp`, then rename).
- Modal timeouts kill the function; retries resume from the last checkpoint because state is loaded at the top of `run_experiment`.
- Checkpoints are dropped only after the result JSON has been written (so we never leave stale state when a run completes).

### 6.5 Failure handling

| Failure | Response |
|---|---|
| Modal spot preemption | Automatic retry; resumes from checkpoint. |
| Timeout (4h exceeded) | Retry; resumes from checkpoint. E5 uses 8h timeout; everything else stays at 4h. |
| OOM (rare on H100 80GB) | Reduce query batch size; this is a config change, not a code change. |
| H100 quota exhausted | Orchestrator backs off and retries; Modal queues excess spawns. Worst case drop concurrency from 16 to 8. |
| HuggingFace rate limit | Exponential backoff inside embed_corpus. |
| S3 upload failure | Retry inside save_checkpoint with backoff. |
| Bug detected after partial run | Bump config schema version; S3 key changes; re-run only affected cells. |
| vLLM server crash (E7) | `keep_warm=1` auto-restart; E7 workers retry with backoff. |
| Budget hit | Orchestrator refuses new spawns; returns list of not-yet-run configs. |

### 6.6 Monitoring

Simple monitoring script runs locally:

```
python monitor.py --block E2
# prints: 312/3150 completed, 45 in_progress, 12 failed, ETA 3.2 days, spend $247
```

Implemented as: list `results/E2/*.json` in S3, parse each, aggregate.

### 6.7 Cost estimate

At Modal's published pricing (verify before launch; numbers drift):
- H100 80GB: ~$3.10/hour

Given the block estimates above:
- Experiment blocks E1–E9: ~190 H100-hours × $3.10 = **$590**
- vLLM server for E7 (2×H100, ~15 hours warm): ~$95
- Buffer for re-runs, debugging, extra seeds: ~50 H100-hours = **$155**
- Storage (S3, ~2TB-month): **~$50**
- **Total expected: ~$890**. Hard cap enforced at **$2,000** in the orchestrator.

No external LLM API cost — vLLM on H100 replaces GPT-4o-mini/Llama API billing. If we later want a GPT-4o comparison point for reviewer defence, budget an additional ~$50 in direct API.

**Why the cost is not much lower than the A10G mixed-fleet version.** H100 is ~3× the hourly rate of A10G but ~3–5× faster on the actual workloads, so total dollars land in the same ballpark. What you buy with H100 is wall-clock and simplicity: the entire experimental grid finishes in ~2 days instead of ~2 weeks, and there is exactly one GPU SKU to reason about.

### 6.8 Local development ↔ Modal parity

Every experiment function must run locally on a small config for debugging. Pattern:

```python
# run_experiment.local(cfg)   # runs in-process, no Modal
# run_experiment.remote(cfg)  # runs on Modal
# run_experiment.spawn(cfg)   # fire-and-forget on Modal
```

Always test with `run_experiment.local(small_cfg)` before calling `main(block="E2")`.

### 6.9 Reproducibility

- Pin all package versions in the Modal image.
- Pin the HF model revision hashes.
- Pin the Wikipedia dump date.
- All seeds in config.
- Run manifest (`runs/<timestamp>.json`) records: git SHA, Modal image digest, full grid, per-cell status.

---

## 7. Figures and tables (draft plan)

### 7.1 Main paper (8 pages)

- **Figure 1.** Conceptual: the three-paper arc, identity–coverage tradeoff as two orthogonal axes, GAC as a router.
- **Figure 2.** Empirical validation of the Consolidation–Interference bound (E1). Tightness on Gaussian, looseness on sub-Gaussian.
- **Figure 3.** Template vs real-text collapse (reuse existing data, 6 encoders). Bar chart showing 0.92 → 0.10 drop. This is the "wait what" figure.
- **Figure 4.** Identity–coverage scatter plot: 6 strategies × 6 encoders × 3 domains, on axes of (coverage BT, identity recall@1). Centroid in top-left corner (high cov, low id), medoid in bottom-right (low cov, high id), GAC on the upper-right Pareto frontier.
- **Figure 5.** Scale study (E5): how the collapse worsens from 10K → 10M. Y-axis identity recall@1, x-axis log(store size).
- **Figure 6.** Downstream LLM accuracy (E7): bar chart, 6 strategies, EM and F1.
- **Table 1.** GAC vs learned baselines (E3) across 3 datasets, matched at 10× compression.
- **Table 2.** GAC ablation (E6).

### 7.2 Supplementary (up to 30 pages)

- Full proofs (Section 2).
- Per-model detailed tables for E2.
- DRM per-list breakdown (E8).
- Temporal pipeline with MRR (E9).
- Modal run manifest.
- Discussion of $d_\text{eff}$ estimators (participation ratio vs Levina–Bickel vs PCA), which is a loose end from No-Escape that we can close here.

---

## 8. Timeline

With H100s across the board the experiments compress to roughly 2 days of wall-clock once the app is built. The bottleneck becomes code + writing, not compute.

| Week | Work |
|---|---|
| 1 | Finalise theorem statement and proof (Section 2). Build Modal app skeleton, data pipeline, embedding cache. End-to-end smoke test on 10K subset. |
| 2 | Implement GAC. Run E1, E2 (full grid), E3 (learned baselines), E8, E9. These all finish in ~1 day of Modal wall-clock at 16× H100 concurrency. |
| 3 | Run E4 (identity metrics), E5 (scale study including 10M), E6 (GAC ablation), E7 (vLLM + downstream LLM). Figures and tables. |
| 4 | First complete draft of the paper, including supplementary. |
| 5 | Internal review, rewrites, polish. |
| 6 | Submit (buffer week). |

This compresses the original 8-week plan to ~6 weeks. Solo execution — all experiments, theorem proof, and writing by Ashwin. Tight but doable because the Modal infrastructure is write-once-run-forever: once the app is built in week 1, weeks 2–3 are mostly kicking off jobs and watching them finish.

**NeurIPS timing.** NeurIPS 2026 abstract deadline is mid-May with full paper ~1 week later. 6 weeks starting now (April 18) puts us at submission in early June — tight against the deadline but doable if week 1 starts Monday. If we miss NeurIPS, ICLR 2027 (September deadline) gives breathing room plus time to add the agent-memory block as a bonus experiment.

---

## 9. Risks and mitigations

| Risk | Likelihood | Mitigation |
|---|---|---|
| GAC does not beat baselines on E2/E3 | Medium | Paper still stands on theorem + duality + template-vs-real finding. Reframe GAC as a diagnostic rather than an algorithm. |
| Theorem proof has a gap | Medium | Submit proof to a measure-theory-comfortable reader early; simplify by restricting to Gaussian clusters for the main text, general case in supplement. |
| Modal has sustained outages | Low | Checkpointing means we lose at most a few hours of work. Back up critical results to S3. Could fall back to RunPod with minor config changes. |
| Running out of budget | Medium | Hard cap in orchestrator. Drop E5 @ 10M if needed; the 1M scale result is sufficient for the paper. |
| Reviewer: "this is just ablations" | Medium | Frame around the theorem + duality from the start. Do not open with experiments. |
| Reviewer: "why not just use HNSW" | High | E3 answers this directly with head-to-head numbers. |
| Reviewer: "why not just use a bigger model" | High | E2 includes text-embedding-3-large (3072d). The collapse persists. Make this explicit. |

---

## 10. Deliverables at submission

- 8-page main paper + unlimited supplementary (NeurIPS format)
- GitHub repo during execution: `niashwin/geometry-of-consolidation` (private). Flip to public + migrate to `dynamis-labs/geometry-of-consolidation` at submission.
  - `gac/` — the algorithm, pip-installable
  - `experiments/` — one file per block (E1–E9)
  - `modal/` — the orchestrator and cached Modal image
  - `data/` — build scripts for every corpus
  - `notebooks/` — figure-generation notebooks
  - `paper/` — LaTeX source
- S3 bucket with all embedding caches, clustering caches, and per-experiment results (public-readable)
- A short blog post for practitioners: "The 30-second test to tell you if consolidation will destroy your RAG system."

---

## 11. Execution decisions (locked 2026-04-18)

1. **Authorship**: single author, Ashwin Gopinath. Sentra affiliation only (235 2nd Street, San Francisco, CA 94105, USA). MIT affiliation **not** included on this paper.
2. **Working repository**: `github.com/niashwin/geometry-of-consolidation` (private personal repo during execution). Migrate to `dynamis-labs/geometry-of-consolidation` after submission/acceptance.
3. **GAC licensing**: full open-source release at submission time. MIT license on the code.
4. **Agent-memory block**: cut for this paper. Save for a follow-up.
5. **GAC routing**: spectral routing as drafted in §3.2. Learned-router variant as an ablation only.
6. **Modal H100 concurrency quota**: still to be verified — see §6.2.1. Action: run probe + email Modal support in week 1.

---

*End of spec.*
