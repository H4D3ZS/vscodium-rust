"""
Generate all figures for the paper (arXiv + NeurIPS versions).

Post-revision figure plan (memo: "takeaway-first captions, centroid in Nexus teal"):

  MAIN FIGURES
    fig0_schematic.pdf        -- Conceptual: cluster + representative + cosine cap;
                                  two cartoons (tight vs spread) framing the law.
    fig1_theorem_regimes.pdf  -- E1: theorem-holds heatmap over (d_eff, theta).
                                  Takeaway: bound holds in tight regime across strategies.
    fig3_encoder_universality.pdf  -- E8 (was fig4_encoder_universality):
                                  Takeaway: centroid ties or beats GAC on 5 of 6 encoders
                                  (Nomic annotated as the exception).
    fig6_ablation.pdf         -- E6: DRM split out as a separate panel; real-text panel
                                  shows delta from gac_full, with fixed-centroid saturated,
                                  oracle shown as thin outline (upper bound).
    fig7_pareto.pdf           -- E3 (was fig3_learned_baselines): best-quant-only baselines,
                                  corpora ordered by d_eff_global, arXiv "crossing" annotated.
    fig8_scale.pdf            -- E5: single-panel, centroid saturated, 1M markers explicit.
    fig9_downstream.pdf       -- E7 (was fig6b_downstream): EM-only, hatched missing cells,
                                  delta-from-no-consolidation bars.

  SUPPLEMENT
    figS_strategy_sweep.pdf            (was fig2)
    figS_id_coverage.pdf               (was fig4b)
    figS_theta_sweep.pdf               (was fig7)
    figS_compression_frontier.pdf      (was fig8)

Color convention (from memo):
  centroid            -> saturated Nexus teal  #20808D
  gac                 -> terra/rust secondary  #A84B2F
  medoid              -> muted gray            #9A9A95
  selective_prune     -> muted gray            #BDBDB8
  importance_weighted -> muted gray            #7A7974
  no_consolidation    -> muted gray            #4A4A47
"""
from __future__ import annotations
import argparse, json, os
from pathlib import Path
import numpy as np
import pandas as pd
import matplotlib as mpl
import matplotlib.pyplot as plt
from matplotlib.patches import Circle, FancyArrow, Patch

mpl.rcParams.update({
    "figure.dpi": 150,
    "savefig.dpi": 200,
    "savefig.bbox": "tight",
    "font.size": 10,
    "axes.titlesize": 11,
    "axes.titleweight": "bold",
    "axes.labelsize": 10,
    "axes.spines.top": False,
    "axes.spines.right": False,
    "legend.frameon": False,
    "figure.autolayout": False,
})

# ---- Color palette (post-revision) ------------------------------------------
CENTROID = "#20808D"    # Nexus teal (saturated)  -- THE HEADLINE WINNER
GAC      = "#A84B2F"    # Terra rust             -- probe strategy
MUTED_A  = "#9A9A95"    # medoid
MUTED_B  = "#BDBDB8"    # selective_prune
MUTED_C  = "#7A7974"    # importance_weighted
MUTED_D  = "#4A4A47"    # no_consolidation / prune50
ACCENT_Y = "#FFC553"    # gold -- for annotations only

PALETTE = {
    "centroid": CENTROID,
    "gac": GAC,
    "medoid": MUTED_A,
    "prune50": MUTED_D,
    "selective_prune": MUTED_B,
    "importance_weighted": MUTED_C,
    "no_consolidation": MUTED_D,
}

# Canonical local d_eff values (matches §6 template-collapse table). Used
# everywhere we label corpora with d_eff in figures to stay consistent with the body.
D_EFF_LOCAL = {
    "drm_templated":       2.3,
    "ms_marco":            5.5,
    "nq_questions":       12.6,
    "wikipedia_sections": 30.1,
    "arxiv_titles":      107.5,
    "hotpot_qa":           1.5,
}

STRATEGY_ORDER = ["centroid", "medoid", "selective_prune", "importance_weighted", "gac"]
STRATEGY_LABEL = {
    "centroid": "centroid",
    "medoid": "medoid",
    "selective_prune": "selective-prune",
    "importance_weighted": "importance-wt",
    "gac": "GAC",
    "no_consolidation": "no-consolidation",
    "prune50": "prune-50%",
}


def outdir(paper_figs: str) -> Path:
    p = Path(paper_figs)
    p.mkdir(parents=True, exist_ok=True)
    return p


def _safe(ax):
    ax.grid(alpha=0.22, linewidth=0.5)


# -----------------------------------------------------------------------------
# Fig 0: Conceptual schematic — cluster + representative + cosine cap
# -----------------------------------------------------------------------------
def fig0_schematic(out: Path):
    """Three panels:
       (a) anatomy of consolidation (cluster in 2D plane with representative + cap);
       (b) tight regime — compact cluster inside cap (centroid recovers);
       (c) spread regime — diffuse cluster, cap captures few (centroid fails).

    We visualise in the 2D plane (not on the unit sphere surface) so the cluster
    is legible as a cloud. The cap is drawn as a circle of radius sqrt(2(1-theta))
    around the representative, which is the chord-distance equivalent of the
    cosine constraint <x,r> >= theta for unit-norm x.
    """
    rng = np.random.default_rng(1)
    fig, axes = plt.subplots(1, 3, figsize=(12.5, 4.2))
    theta = 0.85
    # chord radius for cosine theta on unit sphere: ||x - r||^2 = 2(1 - <x,r>)
    cap_r = np.sqrt(2 * (1 - theta))

    # --- Panel (a): anatomy --------------------------------------------------
    ax = axes[0]
    cdir = np.array([0.0, 0.0])
    pts = rng.normal(cdir, 0.12, size=(20, 2))
    ax.scatter(pts[:, 0], pts[:, 1], s=46, color=MUTED_C,
               edgecolor="white", linewidths=0.6, zorder=3,
               label=r"cluster members $x_i$")
    r = pts.mean(axis=0)
    ax.scatter([r[0]], [r[1]], s=320, marker="*", color=CENTROID,
               edgecolor="black", linewidth=0.9, zorder=5,
               label=r"representative $r$")
    # Cap as circle of radius cap_r around r
    cap = plt.Circle((r[0], r[1]), cap_r, fill=True, facecolor=CENTROID,
                     alpha=0.12, edgecolor=CENTROID, linewidth=1.8, zorder=2)
    ax.add_patch(cap)
    # Arrow denoting radius = arccos(theta)
    ax.annotate("", xy=(r[0] + cap_r, r[1]), xytext=(r[0], r[1]),
                arrowprops=dict(arrowstyle="->", color=CENTROID, lw=1.2))
    ax.text(r[0] + cap_r * 0.5, r[1] - 0.07,
            r"$\arccos\theta$", fontsize=10, color=CENTROID, ha="center")
    ax.text(r[0] + 0.05, r[1] + 0.04, r"$r$", fontsize=11, color="black")
    # Illustrative individual members outside vs inside
    ax.set_xlim(-0.5, 0.7); ax.set_ylim(-0.55, 0.55)
    ax.set_aspect("equal"); ax.set_xticks([]); ax.set_yticks([])
    ax.set_title("(a) A cluster, its representative, and the cosine cap", fontsize=10)
    ax.legend(loc="lower right", fontsize=8)

    # --- Panel (b): tight regime --------------------------------------------
    ax = axes[1]
    centers_B = np.array([[-0.9, 0.6], [0.9, 0.5], [0.0, -0.9]])
    sigma_tight = 0.09
    for c in centers_B:
        pts = rng.normal(c, sigma_tight, size=(22, 2))
        ax.scatter(pts[:, 0], pts[:, 1], s=26, color=MUTED_C,
                   edgecolor="white", linewidths=0.4, alpha=0.85, zorder=3)
        r = pts.mean(axis=0)
        ax.scatter([r[0]], [r[1]], s=220, marker="*", color=CENTROID,
                   edgecolor="black", linewidth=0.7, zorder=5)
        cap = plt.Circle((r[0], r[1]), cap_r, fill=True, facecolor=CENTROID,
                         alpha=0.12, edgecolor=CENTROID, linewidth=1.4, zorder=2)
        ax.add_patch(cap)
    ax.set_xlim(-1.7, 1.7); ax.set_ylim(-1.5, 1.3)
    ax.set_aspect("equal"); ax.set_xticks([]); ax.set_yticks([])
    ax.set_title(r"(b) Tight regime ($d_\mathrm{eff}$ small): cap captures each cluster",
                 fontsize=10)

    # --- Panel (c): spread regime -------------------------------------------
    ax = axes[2]
    sigma_wide = 0.55
    for c in centers_B:
        pts = rng.normal(c, sigma_wide, size=(22, 2))
        ax.scatter(pts[:, 0], pts[:, 1], s=26, color=MUTED_C,
                   edgecolor="white", linewidths=0.4, alpha=0.85, zorder=3)
        r = pts.mean(axis=0)
        ax.scatter([r[0]], [r[1]], s=220, marker="*", color=CENTROID,
                   edgecolor="black", linewidth=0.7, zorder=5)
        cap = plt.Circle((r[0], r[1]), cap_r, fill=True, facecolor=CENTROID,
                         alpha=0.12, edgecolor=CENTROID, linewidth=1.4, zorder=2)
        ax.add_patch(cap)
    ax.set_xlim(-2.3, 2.3); ax.set_ylim(-2.2, 2.0)
    ax.set_aspect("equal"); ax.set_xticks([]); ax.set_yticks([])
    ax.set_title(r"(c) Spread regime ($d_\mathrm{eff}$ large): most members miss the cap",
                 fontsize=10)

    fig.tight_layout()
    fig.savefig(out / "fig0_schematic.pdf", bbox_inches="tight")
    plt.close(fig)
    print(f"[fig0] wrote {out/'fig0_schematic.pdf'}")


# -----------------------------------------------------------------------------
# Fig 1: Theorem regimes (E1)
# -----------------------------------------------------------------------------
def fig1_theorem_regimes(e1_path: str, out: Path):
    df = pd.read_parquet(e1_path)
    fig, axes = plt.subplots(1, 4, figsize=(14, 3.6), sharey=True)
    for ax, strat in zip(axes, ["centroid", "medoid", "prune50", "gac"]):
        sub = df[df["strategy"] == strat]
        piv = sub.groupby(["d_eff_target", "theta"])["theorem_holds_cap"].mean().unstack()
        piv = piv.reindex(sorted(piv.index), axis=0)
        piv = piv.reindex(sorted(piv.columns), axis=1)
        im = ax.imshow(piv.values, aspect="auto", origin="lower",
                       cmap="viridis", vmin=0, vmax=1)
        ax.set_xticks(range(len(piv.columns)))
        ax.set_xticklabels([f"{c:.2f}" for c in piv.columns])
        ax.set_yticks(range(len(piv.index)))
        ax.set_yticklabels([str(r) for r in piv.index])
        ax.set_xlabel(r"$\theta$ (cosine threshold)")
        title_map = {"gac": "GAC (probe)", "prune50": "Prune-50%",
                     "centroid": "Centroid", "medoid": "Medoid"}
        ax.set_title(title_map[strat], color=CENTROID if strat == "centroid" else "black")
        if ax is axes[0]:
            ax.set_ylabel(r"$d_{\mathrm{eff}}$ target")
    cbar = fig.colorbar(im, ax=axes, shrink=0.85, pad=0.02)
    cbar.set_label("Fraction of cells where bound holds")
    fig.savefig(out / "fig1_theorem_regimes.pdf", bbox_inches="tight")
    plt.close(fig)
    print(f"[fig1] wrote {out/'fig1_theorem_regimes.pdf'}")


# -----------------------------------------------------------------------------
# Fig 3: Encoder universality (E8) -- was fig4
# -----------------------------------------------------------------------------
def fig3_encoder_universality(e8_path: str, out: Path):
    df = pd.read_parquet(e8_path)
    encoders = ["bge-base", "bge-large", "e5-large", "minilm", "mpnet", "nomic"]
    encoders = [e for e in encoders if e in df["model"].unique()]
    strats = [s for s in STRATEGY_ORDER if s in df["strategy"].unique()]
    piv = df.groupby(["model", "strategy"])["identity_accuracy"].mean().unstack()
    piv = piv.reindex(encoders)[strats]
    fig, ax = plt.subplots(figsize=(10, 4.4))
    x = np.arange(len(encoders))
    width = 0.15
    for i, strat in enumerate(strats):
        vals = piv[strat].values
        ax.bar(x + (i - (len(strats) - 1) / 2) * width, vals, width,
               label=STRATEGY_LABEL.get(strat, strat),
               color=PALETTE.get(strat, "#888"),
               edgecolor="white",
               linewidth=0.5,
               zorder=3 if strat == "centroid" else 2)
    ax.set_xticks(x)
    ax.set_xticklabels(encoders)
    ax.set_ylabel("Identity accuracy (DRM-templated)")
    ax.set_ylim(0, 1.05)
    ax.legend(ncol=5, fontsize=8, loc="lower right")
    # Annotate nomic as the outlier in difficulty (not winner): all strategies drop together
    if "nomic" in encoders:
        nomic_idx = encoders.index("nomic")
        c_val = piv.loc["nomic", "centroid"]
        ax.annotate("Nomic: all strategies drop together\n(centroid still leads)",
                    xy=(nomic_idx, c_val),
                    xytext=(nomic_idx - 1.4, 0.78),
                    fontsize=8, color="#28251D",
                    arrowprops=dict(arrowstyle="->", color="#28251D", lw=0.7,
                                    connectionstyle="arc3,rad=-0.2"),
                    ha="left")
    _safe(ax)
    fig.savefig(out / "fig3_encoder_universality.pdf", bbox_inches="tight")
    # Legacy alias so existing \includegraphics{figs/fig4_encoder_universality.pdf} resolves
    fig.savefig(out / "fig4_encoder_universality.pdf")
    plt.close(fig)
    print(f"[fig3] wrote {out/'fig3_encoder_universality.pdf'} (and legacy fig4_encoder_universality.pdf)")


# -----------------------------------------------------------------------------
# Fig 6: E6 ablation — DRM split out; real corpora show delta from gac_full
# -----------------------------------------------------------------------------
def fig6_ablation(e6_path: str, out: Path):
    df = pd.read_parquet(e6_path)
    # Order by d_eff (tight -> spread)
    # Use canonical d_eff_local (matches §6 text + fig7 panels), not the
    # E6 dataframe's d_eff_global which is a different quantity.
    d_eff_order = pd.Series({c: v for c, v in D_EFF_LOCAL.items() if c != "drm_templated"}).sort_values()
    real_corpora = [c for c in d_eff_order.index if c in set(df["corpus"])]
    routers = ["gac_fixed_centroid", "gac_fixed_medoid", "gac_fixed_prune",
               "gac_no_residual", "gac_random", "gac_full", "gac_oracle"]
    routers = [r for r in routers if r in df["router"].unique()]
    piv = df.groupby(["corpus", "router"])["identity_accuracy"].mean().unstack()

    fig = plt.figure(figsize=(13, 4.8))
    gs = fig.add_gridspec(1, 4, width_ratios=[1, 3, 0.05, 0.05])
    ax_drm = fig.add_subplot(gs[0, 0])
    ax_real = fig.add_subplot(gs[0, 1])

    # --- Panel A: DRM (templated, artificial tight) ---
    drm_vals = piv.loc["drm_templated", routers] if "drm_templated" in piv.index else None
    if drm_vals is not None:
        colors_drm = []
        for r in routers:
            if r == "gac_fixed_centroid":
                colors_drm.append(CENTROID)
            elif r == "gac_full":
                colors_drm.append(GAC)
            elif r == "gac_oracle":
                colors_drm.append("white")
            else:
                colors_drm.append(MUTED_A)
        for i, (r, v) in enumerate(zip(routers, drm_vals.values)):
            if r == "gac_oracle":
                ax_drm.bar(i, v, color="white", edgecolor=ACCENT_Y, linewidth=1.8, hatch="//", alpha=0.9)
            else:
                ax_drm.bar(i, v, color=colors_drm[i], edgecolor="white", linewidth=0.5,
                           zorder=3 if r == "gac_fixed_centroid" else 2)
        ax_drm.set_xticks(range(len(routers)))
        ax_drm.set_xticklabels([r.replace("gac_","") for r in routers], rotation=45,
                               ha="right", fontsize=8)
        ax_drm.set_ylabel("Identity accuracy (DRM-templated)")
        ax_drm.set_ylim(0, 1.05)
        ax_drm.set_title("(a) DRM-templated\n(artificial tight regime)", fontsize=10)
        _safe(ax_drm)

    # --- Panel B: real corpora, delta from gac_full ---
    x = np.arange(len(real_corpora))
    width = 0.115
    for i, r in enumerate(routers):
        if "gac_full" not in piv.columns or r not in piv.columns:
            continue
        delta = piv.loc[real_corpora, r].values - piv.loc[real_corpora, "gac_full"].values
        if r == "gac_fixed_centroid":
            color, z, ec = CENTROID, 4, "white"
            lbl = "fixed-centroid"
        elif r == "gac_full":
            color, z, ec = GAC, 3, "white"
            lbl = "gac_full (reference)"
        elif r == "gac_oracle":
            # Oracle as thin outline = upper bound on router
            ax_real.bar(x + (i - (len(routers) - 1) / 2) * width, delta, width,
                        facecolor="none", edgecolor=ACCENT_Y, linewidth=1.2,
                        label="oracle (upper bound)", zorder=5)
            continue
        else:
            color, z, ec = MUTED_A if "medoid" in r else MUTED_B if "prune" in r else MUTED_C, 2, "white"
            lbl = r.replace("gac_","")
        ax_real.bar(x + (i - (len(routers) - 1) / 2) * width, delta, width,
                    label=lbl, color=color, edgecolor=ec, linewidth=0.5, zorder=z)
    # Zero line = gac_full reference
    ax_real.axhline(0, color="#28251D", linewidth=0.9, zorder=1)
    ax_real.set_xticks(x)
    # Label with corpus + d_eff
    labels = [f"{c.replace('_','\n')}\n($d_\\mathrm{{eff,local}}{{=}}${d_eff_order[c]:.1f})" for c in real_corpora]
    ax_real.set_xticklabels(labels, fontsize=8)
    ax_real.set_ylabel("$\\Delta$ identity accuracy vs. gac_full")
    ax_real.set_title("(b) Real corpora, ordered by $d_\\mathrm{eff,local}$ (tight $\\rightarrow$ spread)",
                      fontsize=10)
    ax_real.legend(ncol=4, fontsize=8, loc="upper right")
    _safe(ax_real)

    fig.tight_layout()
    fig.savefig(out / "fig6_ablation.pdf", bbox_inches="tight")
    plt.close(fig)
    print(f"[fig6] wrote {out/'fig6_ablation.pdf'}")


# -----------------------------------------------------------------------------
# Fig 7: E3 Pareto — best-quant-only, corpora ordered by d_eff, arXiv crossing
# -----------------------------------------------------------------------------
def fig7_pareto(e3_path: str, e4_path: str, e6_path: str, out: Path):
    if not Path(e3_path).exists():
        print(f"[fig7] SKIP — {e3_path} not found")
        return
    e3 = pd.read_parquet(e3_path)
    e4 = pd.read_parquet(e4_path)
    # Per-corpus LOCAL d_eff (matches §6 template-collapse table).
    # These are the cluster-conditional participation ratios (one cluster at a
    # time, averaged across clusters), not the whole-corpus global d_eff.
    # The theorem applies cluster-by-cluster, so d_eff_local is the correct
    # quantity for the panel ordering and labels. D_EFF_LOCAL is defined at
    # module scope (matches §6 template-collapse table).
    # Order corpora by ascending d_eff_local for the panel grid (matches §6 text).
    d_eff_order = pd.Series(D_EFF_LOCAL).sort_values()
    corpora = [c for c in d_eff_order.index if c in set(e3["corpus"]) and c in set(e4["corpus"])]

    # Best per quant family = max identity_accuracy at the best bytes_per_vec sample
    # For each (corpus, family) keep only the pareto-best (lowest bytes that reaches max-acc bucket).
    fam_color = {"pq": MUTED_B, "opq": MUTED_A, "lsh": MUTED_C,
                 "pca_int8": MUTED_D, "hnsw_prune": "#28251D"}

    nc = len(corpora)
    ncols = 3
    nrows = (nc + ncols - 1) // ncols
    fig, axes = plt.subplots(nrows, ncols, figsize=(13, 3.6 * nrows), sharey=False)
    axes = np.atleast_2d(axes)

    for i, corpus in enumerate(corpora):
        r, c = divmod(i, ncols)
        ax = axes[r, c]
        sub = e3[e3["corpus"] == corpus]
        # Per-family Pareto front: for each family, take top 3 accuracy points
        for fam, clr in fam_color.items():
            f = sub[sub["family"] == fam]
            if len(f) == 0:
                continue
            # sort by bytes, take pareto front
            f = f.sort_values("bytes_per_vec")
            best_acc = -np.inf
            pareto = []
            for _, row in f.iterrows():
                if row["identity_accuracy"] > best_acc:
                    pareto.append(row)
                    best_acc = row["identity_accuracy"]
            if pareto:
                pdf = pd.DataFrame(pareto)
                ax.plot(pdf["bytes_per_vec"], pdf["identity_accuracy"],
                        "-o", color=clr, markersize=4, linewidth=1.1, alpha=0.7,
                        label=fam.upper() if i == 0 else None)

        # Overlay GAC strategies from E4
        g = e4[e4["corpus"] == corpus].groupby("strategy").agg(
            id_lit=("identity_accuracy_literal", "mean"),
            cmp=("compression", "mean"),
        ).reindex(STRATEGY_ORDER)
        d_bge = 1024
        bytes_gac = 4 * d_bge / g["cmp"].astype(float)
        for strat in STRATEGY_ORDER:
            if strat not in g.index or pd.isna(g.loc[strat, "id_lit"]):
                continue
            marker = "*" if strat == "centroid" else ("s" if strat == "gac" else "D")
            size = 230 if strat == "centroid" else (130 if strat == "gac" else 70)
            ec = "black" if strat in ("centroid", "gac") else "white"
            z = 6 if strat == "centroid" else (5 if strat == "gac" else 3)
            ax.scatter(bytes_gac[strat], g.loc[strat, "id_lit"],
                       marker=marker, s=size, color=PALETTE[strat],
                       edgecolors=ec, linewidths=0.8, zorder=z,
                       label=STRATEGY_LABEL.get(strat, strat) if i == 0 else None)

        # arXiv crossing annotation: the point where tight-regime behavior breaks
        if corpus == "arxiv_titles":
            # centroid beats PQ up to some point; annotate where centroid crosses out
            c_x = bytes_gac.get("centroid", np.nan)
            c_y = g.loc["centroid", "id_lit"] if "centroid" in g.index else np.nan
            if not (np.isnan(c_x) or np.isnan(c_y)):
                ax.annotate("centroid competitive\nonly at this $d_\\mathrm{eff}$",
                            xy=(c_x, c_y),
                            xytext=(c_x * 3, c_y - 0.22),
                            fontsize=8, color=CENTROID,
                            arrowprops=dict(arrowstyle="->", color=CENTROID, lw=0.8,
                                            connectionstyle="arc3,rad=0.25"))

        ax.set_xscale("log")
        ax.set_xlabel("Bytes per vector (log)")
        ax.set_ylabel("Identity accuracy")
        # Label with LOCAL d_eff (§6 table), not global — the theorem uses d_eff_local.
        d_val = D_EFF_LOCAL.get(corpus)
        d_lbl = f" ($d_\\mathrm{{eff,local}}={d_val:.1f}$)" if d_val is not None else ""
        ax.set_title(f"{corpus.replace('_',' ')}{d_lbl}", fontsize=10)
        _safe(ax)

    for j in range(len(corpora), nrows * ncols):
        axes.flat[j].axis("off")
    handles, labels = axes.flat[0].get_legend_handles_labels()
    fig.legend(handles, labels, loc="lower center", ncol=6, fontsize=8, bbox_to_anchor=(0.5, -0.03))
    fig.tight_layout(rect=(0, 0.04, 1, 1.0))
    fig.savefig(out / "fig7_pareto.pdf", bbox_inches="tight")
    # Legacy alias for sections still pointing at fig3_learned_baselines
    fig.savefig(out / "fig3_learned_baselines.pdf")
    plt.close(fig)
    print(f"[fig7] wrote {out/'fig7_pareto.pdf'} (and legacy fig3_learned_baselines.pdf)")


# -----------------------------------------------------------------------------
# Fig 8: E5 scale — single panel, centroid saturated, 1M markers explicit
# -----------------------------------------------------------------------------
def fig8_scale(e5_path: str, out: Path):
    if not Path(e5_path).exists():
        print(f"[fig8] SKIP — {e5_path} not found")
        return
    df = pd.read_parquet(e5_path)
    fig, ax = plt.subplots(figsize=(9, 4.4))
    # Pool across encoders; one line per strategy
    for strat in STRATEGY_ORDER:
        sub = df[df["strategy"] == strat]
        if len(sub) == 0:
            continue
        # Group by (n_train, model) first, then average across encoders
        g = sub.groupby(["n_train", "model"])["identity_accuracy"].mean().reset_index()
        # overall line = mean across encoders
        overall = g.groupby("n_train")["identity_accuracy"].mean().sort_index()
        lw = 2.4 if strat == "centroid" else 1.5
        alpha = 1.0 if strat == "centroid" else 0.8
        z = 6 if strat == "centroid" else 3
        ax.plot(overall.index, overall.values,
                marker="o", markersize=6, linewidth=lw, alpha=alpha,
                color=PALETTE.get(strat, "#888"),
                label=STRATEGY_LABEL.get(strat, strat), zorder=z)
        # Explicit 1M marker with numeric label
        last_n = overall.index.max()
        if last_n >= 900_000:
            v = overall.loc[last_n]
            ax.scatter([last_n], [v], marker="o", s=130,
                       color=PALETTE.get(strat, "#888"), edgecolor="black",
                       linewidth=0.9, zorder=z + 2)
            ax.text(last_n * 1.12, v, f"{v:.3f}",
                    fontsize=8, va="center", ha="left",
                    color=PALETTE.get(strat, "#444"))

    ax.set_xscale("log")
    ax.set_xlabel("Training corpus size (log)")
    ax.set_ylabel("Identity accuracy (encoders pooled)")
    ax.set_xticks([10_000, 100_000, 1_000_000])
    ax.set_xticklabels(["10K", "100K", "1M"])
    ax.set_xlim(5_000, 3_000_000)
    ax.set_ylim(0, max(0.6, df["identity_accuracy"].max() + 0.08))
    ax.legend(fontsize=9, loc="lower left", ncol=2)
    _safe(ax)
    # 1M probe annotation: only a subset of strategies were run at 1M
    df_1m = df[df["n_train"] >= 900_000]
    run_1m = sorted(df_1m["strategy"].unique().tolist())
    ax.text(0.98, 1.03, f"1M probe: {', '.join(run_1m)}",
            transform=ax.transAxes, fontsize=8, color="#28251D",
            ha="right", va="bottom", style="italic")
    fig.tight_layout()
    fig.savefig(out / "fig8_scale.pdf", bbox_inches="tight")
    # Legacy alias
    fig.savefig(out / "fig5_scale.pdf")
    plt.close(fig)
    print(f"[fig8] wrote {out/'fig8_scale.pdf'} (and legacy fig5_scale.pdf)")


# -----------------------------------------------------------------------------
# Fig 9: E7 downstream — EM-only, hatched missing cells, delta from no_consolidation
# -----------------------------------------------------------------------------
def fig9_downstream(e7_path: str, out: Path):
    if not Path(e7_path).exists():
        print(f"[fig9] SKIP — {e7_path} not found")
        return
    df = pd.read_parquet(e7_path)
    datasets = ["nq_questions", "hotpot_qa", "popqa"]
    datasets = [d for d in datasets if d in df["dataset"].unique()]
    strat_order = ["no_consolidation", "centroid", "medoid", "selective_prune", "gac"]
    strat_order = [s for s in strat_order if s in df["strategy"].unique()]

    # Build (dataset × strategy) EM matrix; NaN for missing
    em_piv = df.groupby(["dataset", "strategy"])["em"].mean().unstack()
    em_piv = em_piv.reindex(datasets)[strat_order]

    fig, axes = plt.subplots(1, len(datasets), figsize=(4.4 * len(datasets), 4.4),
                              sharey=False)
    if len(datasets) == 1:
        axes = [axes]

    # Compute a shared y-range per panel with symmetric padding
    all_deltas = []
    for ds in datasets:
        row = em_piv.loc[ds]
        base = row["no_consolidation"] if ("no_consolidation" in row.index and not pd.isna(row["no_consolidation"])) else np.nanmean(row.values)
        all_deltas.extend((row.values - base).tolist())
    all_deltas = [d for d in all_deltas if not (isinstance(d, float) and np.isnan(d))]
    max_abs = max(abs(min(all_deltas)), abs(max(all_deltas)))
    ylim = (-max_abs * 1.35, max_abs * 1.35)

    for ax, ds in zip(axes, datasets):
        row = em_piv.loc[ds]
        if "no_consolidation" in row.index and not pd.isna(row["no_consolidation"]):
            baseline = row["no_consolidation"]
        else:
            baseline = np.nanmean(row.values)
        deltas = row.values - baseline  # EM units
        x = np.arange(len(strat_order))
        pad = max_abs * 0.06
        for i, strat in enumerate(strat_order):
            v = row[strat]
            d = deltas[i]
            if pd.isna(v):
                # hatched missing cell: put a zero-height marker near top for visibility
                ax.bar(i, max_abs * 0.02, color="white", edgecolor=MUTED_C,
                       hatch="////", linewidth=1.0)
                ax.text(i, max_abs * 0.08, "n/a", ha="center", fontsize=8, color=MUTED_C)
                continue
            if strat == "centroid":
                color, ec, z = CENTROID, "black", 5
            elif strat == "gac":
                color, ec, z = GAC, "black", 4
            elif strat == "no_consolidation":
                color, ec, z = MUTED_D, "white", 2
            else:
                color, ec, z = MUTED_A if "medoid" in strat else MUTED_B, "white", 2
            ax.bar(i, d, color=color, edgecolor=ec, linewidth=0.6, zorder=z)
            # EM value label placed just outside bar end
            va = "bottom" if d >= 0 else "top"
            offs = pad * (1 if d >= 0 else -1)
            ax.text(i, d + offs, f"EM={v:.3f}", ha="center", va=va, fontsize=7,
                    color="#28251D", zorder=10)
        ax.axhline(0, color="#28251D", linewidth=0.9, zorder=1)
        ax.set_xticks(x)
        ax.set_xticklabels([STRATEGY_LABEL.get(s, s).replace("-","-\n") for s in strat_order],
                           fontsize=8)
        ax.set_title(f"{ds.replace('_',' ')}\n(no-consolidation EM={baseline:.3f})", fontsize=10)
        ax.set_ylim(ylim)
        _safe(ax)
        if ax is axes[0]:
            ax.set_ylabel("$\\Delta$ EM vs. no-consolidation")

    fig.tight_layout()
    fig.savefig(out / "fig9_downstream.pdf", bbox_inches="tight")
    # Legacy alias
    fig.savefig(out / "fig6b_downstream.pdf")
    plt.close(fig)
    print(f"[fig9] wrote {out/'fig9_downstream.pdf'} (and legacy fig6b_downstream.pdf)")


# =============================================================================
# Supplement figures (demoted from main text)
# =============================================================================
def figS_strategy_sweep(e4_path: str, out: Path):
    df = pd.read_parquet(e4_path)
    corpora = ["drm_templated", "ms_marco", "nq_questions", "hotpot_qa", "wikipedia_sections", "arxiv_titles"]
    fig, axes = plt.subplots(2, 3, figsize=(13, 7), sharey=True)
    for ax, corpus in zip(axes.flat, corpora):
        sub = df[df["corpus"] == corpus]
        strat_means = sub.groupby("strategy").agg(
            id_lit=("identity_accuracy_literal", "mean"),
            id_para=("identity_accuracy_para", "mean"),
            mrr_lit=("mrr@20_literal", "mean"),
        ).reindex(STRATEGY_ORDER)
        x = np.arange(len(strat_means))
        width = 0.28
        ax.bar(x - width, strat_means["id_lit"], width, label="Identity (literal)",
               color=CENTROID, edgecolor="white")
        ax.bar(x, strat_means["id_para"], width, label="Identity (paraphrase)",
               color=GAC, edgecolor="white")
        ax.bar(x + width, strat_means["mrr_lit"], width, label="MRR@20",
               color=MUTED_A, edgecolor="white")
        ax.set_xticks(x)
        ax.set_xticklabels([s.replace("_", "\n") for s in strat_means.index], fontsize=8)
        ax.set_ylim(0, 1.05)
        ax.set_title(corpus.replace("_", " "), fontsize=10)
        _safe(ax)
        if ax is axes.flat[0]:
            ax.set_ylabel("Score")
    axes.flat[2].legend(loc="upper right", fontsize=8, ncol=1)
    fig.tight_layout()
    fig.savefig(out / "figS_strategy_sweep.pdf", bbox_inches="tight")
    # Legacy alias so existing \includegraphics calls still resolve
    fig.savefig(out / "fig2_strategy_sweep.pdf")
    plt.close(fig)
    print(f"[figS] wrote figS_strategy_sweep.pdf")


def figS_theta_sweep(e8_path: str, out: Path):
    if not Path(e8_path).exists():
        print("[figS-theta] SKIP")
        return
    df = pd.read_parquet(e8_path)
    gac = df[df["strategy"] == "gac"].copy()
    if gac.empty:
        return
    gac["theta"] = gac["strategy_kw"].apply(
        lambda s: json.loads(s).get("theta") if s else None)
    pivot = gac.groupby(["model", "theta"])["identity_accuracy"].mean().unstack()
    fig, ax = plt.subplots(figsize=(8, 4.5))
    encoders = sorted(pivot.index.tolist())
    colors = [CENTROID, GAC, MUTED_A, MUTED_B, MUTED_C, ACCENT_Y]
    for i, m in enumerate(encoders):
        row = pivot.loc[m].sort_index()
        ax.plot(row.index, row.values, marker="o", linewidth=1.8, markersize=6,
                color=colors[i % len(colors)], label=m)
    centroid_mean = df[df["strategy"] == "centroid"]["identity_accuracy"].mean()
    ax.axhline(centroid_mean, linestyle="--", color="#28251D", linewidth=1.2,
               label=f"centroid mean ({centroid_mean:.2f})")
    ax.set_xlabel(r"GAC threshold $\theta$")
    ax.set_ylabel("Identity accuracy (DRM)")
    ax.set_xticks([0.6, 0.7, 0.8, 0.9])
    ax.set_ylim(0, 1.05)
    ax.legend(ncol=2, fontsize=8, loc="lower left")
    _safe(ax)
    fig.savefig(out / "figS_theta_sweep.pdf")
    fig.savefig(out / "fig7_theta_sweep.pdf")
    plt.close(fig)
    print("[figS-theta] wrote figS_theta_sweep.pdf")


def figS_compression_frontier(e8_path: str, out: Path):
    if not Path(e8_path).exists():
        return
    df = pd.read_parquet(e8_path)
    fam_color = {"centroid": CENTROID, "medoid": MUTED_A, "gac": GAC,
                 "selective_prune": MUTED_B, "importance_weighted": MUTED_C,
                 "no_consolidation": MUTED_D}
    fig, ax = plt.subplots(figsize=(8, 4.8))
    for fam, g in df.groupby("strategy"):
        ax.scatter(g["compression"], g["identity_accuracy"],
                   s=38, alpha=0.8, color=fam_color.get(fam, "#888"),
                   edgecolor="white", linewidth=0.6, label=fam)
    ax.set_xscale("log")
    ax.set_xlabel("Compression ratio (n_train / n_representatives)")
    ax.set_ylabel("Identity accuracy (DRM)")
    ax.set_ylim(0, 1.05)
    ax.legend(ncol=3, fontsize=8, loc="lower left")
    _safe(ax)
    fig.savefig(out / "figS_compression_frontier.pdf")
    fig.savefig(out / "fig8_compression_frontier.pdf")
    plt.close(fig)
    print("[figS-compression] wrote figS_compression_frontier.pdf")


def figS_id_coverage(e4_path: str, out: Path):
    if not Path(e4_path).exists():
        return
    df = pd.read_parquet(e4_path)
    fig, ax = plt.subplots(figsize=(8, 5.2))
    markers = {"drm_templated": "o", "ms_marco": "s", "nq_questions": "^",
               "hotpot_qa": "D", "wikipedia_sections": "P", "arxiv_titles": "X"}
    for strat in STRATEGY_ORDER:
        for corpus in df["corpus"].unique():
            sub = df[(df["strategy"] == strat) & (df["corpus"] == corpus)]
            if len(sub) == 0:
                continue
            x = sub["identity_accuracy_literal"].mean()
            y = sub["coverage@0.80_literal"].mean() if "coverage@0.80_literal" in sub.columns \
                else sub["mrr@20_literal"].mean()
            ax.scatter(x, y, marker=markers.get(corpus, "o"), s=90,
                       color=PALETTE.get(strat, "#888"),
                       edgecolor="black", linewidth=0.6)
    for strat in STRATEGY_ORDER:
        ax.scatter([], [], c=PALETTE.get(strat, "#888"), s=70,
                   label=STRATEGY_LABEL.get(strat, strat))
    for corpus, m in markers.items():
        ax.scatter([], [], marker=m, c="gray", s=60, label=corpus.replace("_", " "))
    ax.set_xlabel("Identity accuracy (literal)")
    ax.set_ylabel("Coverage @ $\\theta$=0.80 (literal)")
    ax.set_xlim(0, 1.05); ax.set_ylim(0, 1.05)
    ax.plot([0, 1], [0, 1], ls=":", c="gray", lw=0.7, alpha=0.6)
    ax.legend(fontsize=7, loc="lower right", ncol=2)
    _safe(ax)
    fig.tight_layout()
    fig.savefig(out / "figS_id_coverage.pdf", bbox_inches="tight")
    fig.savefig(out / "fig4b_id_coverage.pdf")
    plt.close(fig)
    print("[figS-idcov] wrote figS_id_coverage.pdf")


# =============================================================================
# Main
# =============================================================================
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="paper/figs")
    ap.add_argument("--e1", default="results/e1/e1_results.parquet")
    ap.add_argument("--e3", default="results/e3/e3_results.parquet")
    ap.add_argument("--e4", default="results/e4/e4_results.parquet")
    ap.add_argument("--e5", default="results/e5/e5_results.parquet")
    ap.add_argument("--e6", default="results/e6/e6_results.parquet")
    ap.add_argument("--e7", default="results/e7/e7_results.parquet")
    ap.add_argument("--e8", default="results/e8/e8_results.parquet")
    args = ap.parse_args()

    out = outdir(args.out)

    # Main text figures
    fig0_schematic(out)
    fig1_theorem_regimes(args.e1, out)
    fig3_encoder_universality(args.e8, out)
    fig6_ablation(args.e6, out)
    fig7_pareto(args.e3, args.e4, args.e6, out)
    fig8_scale(args.e5, out)
    fig9_downstream(args.e7, out)

    # Supplement
    figS_strategy_sweep(args.e4, out)
    figS_id_coverage(args.e4, out)
    figS_theta_sweep(args.e8, out)
    figS_compression_frontier(args.e8, out)


if __name__ == "__main__":
    main()
