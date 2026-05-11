"""
Emit T1 (GAC vs learned baselines) and T2 (GAC ablation) as LaTeX.

Outputs:
    paper/tables/t1_learned_baselines.tex
    paper/tables/t2_gac_ablation.tex
"""
from __future__ import annotations
import argparse
import pathlib

import numpy as np
import pandas as pd


def t1(e3_path: str, e4_path: str, out: pathlib.Path) -> None:
    """T1: at ~10x compression, compare GAC strategies vs learned baselines.

    Uses E4 for GAC strategies (identity + mrr + recall on real text)
    and E3 for learned compressors (PQ/OPQ/LSH/PCA+int8/HNSW).
    """
    out.mkdir(parents=True, exist_ok=True)

    try:
        e3 = pd.read_parquet(e3_path)
    except Exception:
        e3 = pd.DataFrame()
    e4 = pd.read_parquet(e4_path)

    # Restrict to corpora that appear in both (or just E4 if E3 is empty)
    corpora = ["drm_templated", "ms_marco", "nq_questions",
               "hotpot_qa", "wikipedia_sections", "arxiv_titles"]
    corpora = [c for c in corpora if c in e4["corpus"].unique()]

    # Collect GAC rows
    rows = []
    for corpus in corpora:
        sub = e4[(e4["corpus"] == corpus) & (e4["model"] == "bge-large")]
        for strat in ["centroid", "medoid", "selective_prune",
                      "importance_weighted", "gac"]:
            s = sub[sub["strategy"] == strat]
            if len(s) == 0:
                continue
            rows.append({
                "corpus": corpus, "family": "ours", "method": strat,
                "bytes_per_vec": 4 * 1024 / max(s["compression"].mean(), 1e-9),
                "id_acc": s["identity_accuracy_literal"].mean(),
                "mrr20": s["mrr@20_literal"].mean(),
                "r100": s["recall@100_literal"].mean(),
            })
        if len(e3) and "corpus" in e3.columns:
            e = e3[e3["corpus"] == corpus]
            for fam in ["pq", "opq", "lsh", "pca_int8", "hnsw_prune"]:
                f = e[e["family"] == fam]
                if len(f) == 0:
                    continue
                # Pick row closest to 10x compression (~410 bytes for d=1024 float32)
                f = f.copy()
                f["_diff"] = (f["bytes_per_vec"] - 4 * 1024 / 10).abs()
                best = f.sort_values("_diff").iloc[0]
                rows.append({
                    "corpus": corpus, "family": "baseline", "method": fam,
                    "bytes_per_vec": float(best["bytes_per_vec"]),
                    "id_acc": float(best["identity_accuracy"]),
                    "mrr20": float(best.get("mrr@20", np.nan)),
                    "r100": float(best.get("recall@100", np.nan)),
                })
    df = pd.DataFrame(rows)

    if df.empty:
        out_path = out / "t1_learned_baselines.tex"
        out_path.write_text("% T1: no data yet\n")
        print(f"[t1] wrote {out_path} (empty)")
        return

    # Pivot: rows = method, columns = corpus, value = id_acc
    piv = df.pivot_table(index="method", columns="corpus", values="id_acc", aggfunc="mean")
    # Order
    method_order = ["pq", "opq", "lsh", "pca_int8", "hnsw_prune",
                    "centroid", "medoid", "selective_prune",
                    "importance_weighted", "gac"]
    piv = piv.reindex([m for m in method_order if m in piv.index])
    piv = piv.reindex([c for c in corpora if c in piv.columns], axis=1)

    # LaTeX
    def _fmt(v):
        return f"{v:.3f}" if pd.notna(v) else "--"

    lines = [r"\begin{table}[t]",
             r"\centering",
             r"\small",
             r"\caption{T1: Identity accuracy at $\approx$10$\times$ compression across methods and corpora. "
             r"Top block: learned baselines (PQ, OPQ, LSH, PCA+int8, HNSW-prune) from E3. "
             r"Bottom block: geometric strategies (incl. ours: \textsc{GAC}) from E4. "
             r"BGE-large encoder, 3 seeds.}",
             r"\label{tab:t1}",
             r"\begin{tabular}{l" + "c" * len(piv.columns) + r"}",
             r"\toprule",
             "Method & " + " & ".join(c.replace("_", " ") for c in piv.columns) + r" \\",
             r"\midrule"]
    # Baselines first, then a midrule, then ours
    baselines = [m for m in ["pq", "opq", "lsh", "pca_int8", "hnsw_prune"] if m in piv.index]
    ours = [m for m in ["centroid", "medoid", "selective_prune",
                        "importance_weighted", "gac"] if m in piv.index]
    for m in baselines:
        row = [m.upper().replace("_", r"\_")] + [_fmt(piv.loc[m, c]) for c in piv.columns]
        lines.append(" & ".join(row) + r" \\")
    if baselines and ours:
        lines.append(r"\midrule")
    for m in ours:
        bold = (m == "gac")
        name = r"\textbf{GAC (ours)}" if bold else m.replace("_", r"\_")
        row = [name] + [(r"\textbf{" + _fmt(piv.loc[m, c]) + r"}" if bold else _fmt(piv.loc[m, c]))
                         for c in piv.columns]
        lines.append(" & ".join(row) + r" \\")
    lines += [r"\bottomrule",
              r"\end{tabular}",
              r"\end{table}"]
    (out / "t1_learned_baselines.tex").write_text("\n".join(lines) + "\n")
    print(f"[t1] wrote {out/'t1_learned_baselines.tex'}")


def t2(e6_path: str, out: pathlib.Path) -> None:
    """T2: GAC router ablation across corpora."""
    out.mkdir(parents=True, exist_ok=True)
    df = pd.read_parquet(e6_path)
    corpora = ["drm_templated", "ms_marco", "nq_questions",
               "hotpot_qa", "wikipedia_sections", "arxiv_titles"]
    corpora = [c for c in corpora if c in df["corpus"].unique()]
    routers = ["gac_full", "gac_no_residual", "gac_random",
               "gac_fixed_centroid", "gac_fixed_medoid",
               "gac_fixed_prune", "gac_oracle"]
    routers = [r for r in routers if r in df["router"].unique()]

    # BGE-large + MiniLM averaged (report BGE-large primary, MiniLM in parens)
    df_bge = df[df["model"] == "bge-large"] if "model" in df.columns else df
    piv = df_bge.pivot_table(index="router", columns="corpus",
                             values="identity_accuracy", aggfunc="mean")
    piv = piv.reindex(routers)[[c for c in corpora if c in piv.columns]]

    def _fmt(v):
        return f"{v:.3f}" if pd.notna(v) else "--"

    lines = [r"\begin{table}[t]",
             r"\centering",
             r"\small",
             r"\caption{T2: GAC router ablation across corpora (E6). "
             r"BGE-large, 3 seeds. \textsc{gac\_full} is the default router. "
             r"\textsc{gac\_oracle} is the router that picks the best strategy per cluster given gold identity labels "
             r"(upper-bound reference).}",
             r"\label{tab:t2}",
             r"\begin{tabular}{l" + "c" * len(piv.columns) + r"}",
             r"\toprule",
             "Router & " + " & ".join(c.replace("_", " ") for c in piv.columns) + r" \\",
             r"\midrule"]
    for r in piv.index:
        bold = (r == "gac_full")
        name = r"\textbf{\textsc{gac\_full} (ours)}" if bold else r.replace("_", r"\_")
        row = [name] + [(r"\textbf{" + _fmt(piv.loc[r, c]) + r"}" if bold else _fmt(piv.loc[r, c]))
                         for c in piv.columns]
        lines.append(" & ".join(row) + r" \\")
    lines += [r"\bottomrule",
              r"\end{tabular}",
              r"\end{table}"]
    (out / "t2_gac_ablation.tex").write_text("\n".join(lines) + "\n")
    print(f"[t2] wrote {out/'t2_gac_ablation.tex'}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="paper/tables")
    ap.add_argument("--e3", default="results/e3/e3_results.parquet")
    ap.add_argument("--e4", default="results/e4/e4_results.parquet")
    ap.add_argument("--e6", default="results/e6/e6_results.parquet")
    args = ap.parse_args()
    out = pathlib.Path(args.out)
    t1(args.e3, args.e4, out)
    t2(args.e6, out)


if __name__ == "__main__":
    main()
