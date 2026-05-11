# The Geometry of Consolidation: Why Every Vector Database Has a Compression Cliff, and How to See It Coming

## TL;DR

- Every AI memory system eventually tries to shrink itself. RAG pipelines merge duplicate chunks. Agent frameworks summarise old conversations. Vector databases cluster similar vectors together and throw away the rest. The folk wisdom is that you lose a little recall and save a lot of RAM. In a large fraction of real geometries, that "little loss of recall" actually destroys the memory's ability to tell items apart.
- We worked out the closed-form law for when it does. Two cheap measurements on each cluster (how spread out it is, and how many dimensions it actually uses) tell you whether compressing that cluster is safe or catastrophic.
- The math splits into two regimes with a clean boundary. On one side, a near-ideal constant from classical probability works almost everywhere. On the other, the same constant blows up by eleven orders of magnitude. Crucially, you can tell which side of the boundary each cluster is on before compressing it.
- We built a router called GAC that picks the right compression strategy cluster by cluster. Against every standard baseline — centroid merging, product quantisation, LSH, HNSW pruning, and more — it wins on the whole quality-versus-compression curve across 6 embedding models, 7 standard RAG datasets, and scale up to one million items.
- This closes a trilogy. [The Geometry of Forgetting](https://arxiv.org/abs/2604.06222) showed why embedding memories forget the same way humans do. [The Price of Meaning](https://arxiv.org/html/2603.27116v1) proved you cannot engineer that forgetting away. If you cannot escape the geometry, at least compress with it instead of against it.
- Paper, code, reproduction: [github.com/niashwin/geometry-of-consolidation](https://github.com/niashwin/geometry-of-consolidation).

## The thread that keeps pulling

There is a thread running through every paper we have put out this year, and each new one yanks us further along it.

It starts with a boring observation. Learned representations do not use the dimensionality they advertise. A 1,024-dimensional embedding concentrates its real variance in roughly 16 directions, and the other 1,008 are mostly noise. A 128-dimensional transformer attention vector uses about 4. Cortical recordings from biological brains sit somewhere in the 100 to 500 range. Across wildly different substrates you see the same pattern: a handful of directions doing the work, a long tail of filler.

In [SpectralQuant](https://github.com/Dynamis-Labs/spectralquant/blob/main/paper_output/spectralquant.pdf), that concentration was an **opportunity**. It told us where to spend bits during compression, and we used it to beat a provably near-optimal quantisation bound by 18.6%.

In the [Geometry of Forgetting](https://arxiv.org/abs/2604.06222), the same concentration became a **vulnerability**. Production embeddings, when used as memory, reproduce the quantitative signatures of human memory failure: the Ebbinghaus forgetting curve (memory decay as a power law over time, first measured in 1885), DRM false memories (people "remembering" thematically related words they never saw), and tip-of-tongue states. These were not loose analogies. We got numbers inside a few percent of data from clinical psychology, with no tuning.

In the [Price of Meaning](https://arxiv.org/html/2603.27116v1), the concentration became an **impossibility theorem**. Any memory that retrieves by meaning, under finite effective dimensionality, is trapped on a frontier between interference and usefulness. We tested five architectures: dense vector DB, knowledge graph, attention-based context window, BM25 keyword search, and parametric memory (knowledge baked into model weights). Only BM25 escaped, and only because it agreed with semantic search on 15.5% of queries. It escaped by being useless.

That left one direction we had not done properly. Anyone running a memory at real scale has to **compress** it sooner or later. You cannot keep every chunk forever.

As a side result in Geometry of Forgetting we had already shown that the laziest form of compression, averaging nearby embeddings into a single mean vector, roughly quadruples interference instead of smoothing it away. We called it the "vector averaging fallacy" and it sat in the old paper as a warning. This time we wanted a law.

## Some vocabulary, up front

A few terms come up repeatedly. Defining them once here saves us parenthetical asides later on.

- **Cell / cluster.** A group of similar items that compression is going to merge into one thing. Every consolidation strategy makes these, whether the implementation calls them clusters, buckets, or shards.
- **Representative.** The one vector you keep after merging a cell. In vanilla k-means it is the centroid of the cluster. In medoid clustering it is the actual cell member closest to that centroid. There are many flavours, and the consolidation question is always which flavour to use.
- **Effective dimensionality (d_eff).** How many dimensions the vectors in a cell actually use, measured as the participation ratio of the covariance spectrum (you can compute it with one small eigendecomposition). It is a property of the data, not the embedding model's nameplate dimensionality.
- **Cosine threshold (θ).** The similarity score above which retrieval says "yes, this matches." We write θ′ = 1 − θ for the margin it leaves. A tight threshold (θ near 1) leaves very little θ′ to play with.
- **Mean dissimilarity (d̄).** How internally spread out a cell is. A tight cell has small d̄. A loose one has big d̄.
- **Identity error.** The probability that, after compression, a query that should retrieve its true home item instead retrieves the wrong representative. This is the thing we want to bound.

That is the whole glossary, and everything below is built out of it.

## The law

The thing we prove is a single inequality. It bounds identity error in terms of four measurable numbers: how many items you squash into one (*m*), the threshold margin (θ′), the cell's spread (d̄), and the cell's effective dimensionality (d_eff). A calibration constant *c₁* sits out front.

The formula, for readers who want it:

\[ \varepsilon_{id} \;\geq\; 1 - c_1 \cdot m \cdot \left(\frac{\theta'}{\bar d}\right)^{d_{\text{eff}}/2}. \]

Everyone else can skip it. The behaviour you need from the law is simple enough to describe in words: as the cell gets more spread out (d̄ up), or as the threshold tightens (θ′ down), compression gets worse. As the effective dimensionality goes up, compression gets worse faster, because the ratio (θ′/d̄) is raised to the d_eff/2 power. More usable dimensions means more places for error to hide.

The interesting behaviour, though, lives in the constant *c₁*.

## Two regimes, or: where the cliff is

Try to calibrate a single *c₁* across every geometry we generated (a grid of 16,000 synthetic cells sweeping realistic *m*, θ, d̄, and d_eff), and you get an ugly number: about 4.6 million. That is the honest scope-wide constant. It is what you are stuck with if you refuse to look at the geometry of your data before you compress.

Split the grid on one question — is the cell's spread d̄ smaller than the threshold margin θ′? — and the whole picture resolves into two very different worlds.

| Regime | Condition | Cells | *c₁* needed for 95% coverage | How often *c₁* = 1 is enough |
| --- | --- | --- | --- | --- |
| **Tight** | d̄ < θ′ | 12,400 | **0.046** | 97.3% of cells |
| **Spread** | d̄ ≥ θ′ | 3,600 | **4.61 × 10¹⁰** | 11.3% of cells |
| Global | everything | 16,000 | 4.60 × 10⁶ | 78.0% of cells |

(The "95% coverage" column asks: what value of *c₁* do you need so that the inequality correctly holds on 95% of cells? Smaller is better. The last column asks the complementary question: if you just set *c₁* = 1 and walk away, on what fraction of cells are you correct?)

The tight-regime number is the striking one. There is a classical result in probability called the Berry–Esseen theorem, which bounds how fast sums of independent random variables converge to a normal distribution. The best known constant in that bound, due to Shevtsova, is about 0.47, and that is effectively the theoretical floor for a constant of this shape. Our empirically-calibrated *c₁* = 0.046 sits within an order of magnitude of that floor, and a plain *c₁* = 1 (meaning: ignore calibration entirely) still works on 97% of cells. That is what a scaling law looks like when it actually lands: close to the theoretical ideal across thousands of geometries, not merely fit to the data.

The spread-regime number is the other side of the same coin. The inequality does not break in the spread regime; the constant just has to jump eleven orders of magnitude to stay valid. When the cell is wider than the threshold margin allows, a handful of worst-case items sit right on the edge and dominate the failure tail, the neat picture that gave us the tight-regime bound stops holding, and *c₁* absorbs all the looseness. The practical reading is: in this regime, compressing the cell with a single representative is probably a mistake.

The punchline of the section is that you can see which side of the cliff you are on without jumping off. d̄ and θ′ are cheap to compute on the store you already have, which means the spread regime is not some mysterious failure tail. It is a geometric set of cells you can flag in advance.

## Real corpora live on both sides of the cliff

If this only happened on synthetic grids it would not matter much. It shows up on every standard RAG dataset we tried. Effective dimensionality measured locally on seven public corpora spans almost two orders of magnitude:

| Corpus | Local d_eff |
| --- | --- |
| HotpotQA | 1.5 |
| DRM templated sentences | 2.3 |
| MS MARCO | 5.5 |
| Natural Questions | 12.6 |
| Wikipedia sections | 30.1 |
| arXiv titles | 107.5 |

Tight, templated QA data lives near *d_eff* of 2. arXiv titles live near 100. These are not obscure test sets; they are backbone corpora for mainstream RAG evaluation.

Any compression strategy that treats them uniformly is pretending a two-order-of-magnitude range in geometry does not exist. The measured identity-error curves bend where the law predicts they will, on every corpus we checked.

## GAC: compress with the geometry

Everything so far is diagnostic. Knowing where a cliff is only matters if you can route around it.

We built **GAC** (Geometry-Aware Consolidation) to do exactly that. It is not a new algorithm family so much as a three-line router on top of standard compression primitives:

1. For each cell, cheaply estimate *d_eff* and d̄ from a small eigendecomposition.
2. If the cell is in the tight regime (d̄ < θ′), use a plain centroid as the representative. Classical k-means-style averaging. Near-optimal here, no reason to do anything fancier.
3. If the cell is in the spread regime, use a *residual-budgeted medoid*: pick the actual item in the cell whose worst-case distance to any other member of the cell stays inside a budget the threshold margin allows. Classical medoid clustering, except with the budget derived from θ′ rather than fixed ahead of time.

That is the entire router.

We tested it against every classical baseline we could find: centroid merging (the textbook default), medoid clustering, importance-weighted centroid, selective pruning, product quantisation (PQ, the industry-standard compressed vector format), optimised PQ (OPQ, PQ with a rotation for better bit allocation), locality-sensitive hashing (LSH, randomised sketches for approximate search), and HNSW pruning (trimming the HNSW graph, which is what FAISS and most production ANN indexes are built on). Across 6 embedding models (Nomic, MPNet, MiniLM, and three BGE variants), 7 corpora, and scale from 10,000 to 1,000,000 items, GAC sits strictly below every baseline on the quality-versus-compression curve. For any compression ratio you pick, it loses less quality than anything else we tried, on the whole curve rather than at one operating point.

The baselines are not clever and the router is not clever either. The leverage is that the law tells you, cluster by cluster, which classical method is safe to apply on that cluster.

## The follow-up experiments

One scale, one encoder, one corpus would not be enough to trust any of this, so we closed the obvious doors.

- **Scale up, 10K to 1M items.** Identity error tracks the law across two orders of magnitude in store size. The tight-versus-spread split, predicted from geometry alone, stays predictive at a million items.
- **Ablation with the router off.** Pin GAC to always use one kind of representative and performance collapses back toward the worst baseline on exactly the regime the router would have caught. The win is the routing, not the primitives.
- **Downstream RAG, end to end.** Stores compressed by GAC preserve answer quality in full RAG pipelines, on the same benchmarks where centroid and PQ compression lose several accuracy points.
- **Encoder universality.** The law calibrates to a stable constant across all six embedding models; the tight-regime *c₁* barely moves.
- **Temporal stability.** A cell in the tight regime today stays tight tomorrow with high probability, which is what lets the router amortise its cost over time rather than re-running on every insert.

All of this is reproducible from the repo, on every slice of data the paper reports.

## What we are not claiming

The scope of the paper is narrow on purpose.

The law is proved and calibrated for one setting: **contrastive** text embeddings (the training objective that powers every modern sentence encoder, from MiniLM to BGE to Nomic), **unit-norm** (vectors on the surface of a sphere, which is how cosine similarity is normally set up), **English**, retrieved with a **cosine threshold**. That covers essentially every production RAG system today, which is why the scope is useful. It is not everything.

We do not claim the law transfers without re-calibration to non-contrastive embeddings, image or audio embeddings, multimodal stores, or metrics other than cosine. GAC is not a universal consolidation algorithm either; it is the right router inside this scope.

We also do not hide the spread-regime blow-up. The *c₁* = 4.61 × 10¹⁰ number for the spread regime is in the table, and the *c₁* = 4.60 × 10⁶ scope-wide number is too. If you want a single number, those are the numbers. The regime split is what to use when you actually have the geometry in hand, which in practice you always do.

## Where this leaves the trilogy

The three papers now line up cleanly.

- [**Geometry of Forgetting**](https://arxiv.org/abs/2604.06222) — the geometry *causes* forgetting. The same spectral concentration SpectralQuant used for compression is the mechanism that makes embedding memories forget, hallucinate, and stall on tip-of-tongue states, matching human cognitive data quantitatively.
- [**The Price of Meaning**](https://arxiv.org/html/2603.27116v1) — the geometry *forbids* escape. Any semantic memory under finite effective dimensionality is trapped on a frontier between interference and usefulness. No architecture we tested escaped it.
- **The Geometry of Consolidation** (this paper) — the geometry *governs compression too*. Two regimes, one router, a calibrated law, dominant performance across the consolidation frontier. You cannot escape the frontier, but you can navigate it deliberately instead of blind.

The architectural implication follows straight from Price of Meaning. That paper argued for a two-layer memory: a semantic layer for generalisation, coupled with an exact episodic record giving the semantic layer something stable to verify against. Consolidation adds one more thing to that picture. Inside the semantic layer itself, you still do not have to walk the compression frontier in the dark; the frontier has a shape, the shape is local *d_eff* and d̄, and reading the shape lets you spend your compression budget where it actually buys you something.

## Try it

Everything here runs.

```bash
git clone https://github.com/niashwin/geometry-of-consolidation
cd geometry-of-consolidation
pip install -r requirements.txt
python scripts/e1_c1_calibration.py        # reproduces tight / spread / global c_1
python scripts/e2_per_corpus_identity.py   # per-corpus local d_eff and identity error
python scripts/e5_scale_probe.py           # 10K -> 1M scaling
python -m gac.router                       # GAC router on a toy store
```

- Paper: [arXiv preprint PDF](https://github.com/niashwin/geometry-of-consolidation/blob/main/paper/arxiv/main.pdf)
- NeurIPS 2026 main: [paper/neurips/main.pdf](https://github.com/niashwin/geometry-of-consolidation/blob/main/paper/neurips/main.pdf)
- NeurIPS 2026 supplementary: [paper/neurips/supp.pdf](https://github.com/niashwin/geometry-of-consolidation/blob/main/paper/neurips/supp.pdf)
- Code and data: [github.com/niashwin/geometry-of-consolidation](https://github.com/niashwin/geometry-of-consolidation)

---

This is the fifth in a series that started with [3% Is All You Need: Breaking TurboQuant's Compression Limit via Spectral Structure](https://x.com/ashwingop/status/2041117804615897162), continued with [No Escape: Why Every AI Memory System Fails](https://x.com/ashwingop/status/2041553652972941455), [The Geometry of Forgetting: Why Brains and LLMs Fail EXACTLY the Same Way](https://x.com/ashwingop/status/2042091130213560759), and [The Price of Meaning: Why RAG, Knowledge Graphs, and Every Semantic Memory Will Always Fail](https://arxiv.org/html/2603.27116v1). All five grew out of the research programme at [Sentra](https://sentra.app/), where we are building enterprise general intelligence: a shared AI layer that sits on every communication channel and agent trace inside an organisation to understand how work actually gets done, and to build a living world model of the entire company in near real time.
