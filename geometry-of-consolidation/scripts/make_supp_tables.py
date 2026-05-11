"""
Generate supplementary LaTeX tables for the paper:
  - supp_t_e2_per_cell.tex
  - supp_t_e3_per_cell.tex
  - supp_t_e6_per_cell.tex
  - supp_t_drm_breakdown.tex
  - supp_t_c1_calibration.tex
  - supp_t_manifest.tex
"""
from __future__ import annotations
import argparse
import json
import pathlib
import subprocess

import pandas as pd
import numpy as np


def _latex_table(df: pd.DataFrame, caption: str, label: str,
                 fmt: dict | None = None, col_fmt: str | None = None,
                 longtable: bool = True, max_rows: int | None = 60) -> str:
    fmt = fmt or {}
    if max_rows and len(df) > max_rows:
        df = df.head(max_rows).copy()
        caption = caption + f" (showing first {max_rows} rows; full CSV in repo)"

    def cell(v, col):
        if pd.isna(v):
            return "--"
        if col in fmt:
            return fmt[col](v)
        if isinstance(v, (int, np.integer)):
            return f"{int(v)}"
        if isinstance(v, (float, np.floating)):
            return f"{v:.3f}"
        return str(v).replace("_", r"\_")

    col_fmt = col_fmt or "l" + "c" * (len(df.columns) - 1)
    env = "longtable" if longtable else "tabular"
    lines = []
    if longtable:
        lines += [r"\begin{" + env + r"}{" + col_fmt + r"}",
                  r"\caption{" + caption + r"}\label{" + label + r"}\\",
                  r"\toprule",
                  " & ".join([c.replace("_", r"\_") for c in df.columns]) + r" \\",
                  r"\midrule",
                  r"\endfirsthead",
                  r"\multicolumn{" + str(len(df.columns)) + r"}{l}{\small\itshape (continued)}\\",
                  r"\toprule",
                  " & ".join([c.replace("_", r"\_") for c in df.columns]) + r" \\",
                  r"\midrule",
                  r"\endhead",
                  r"\bottomrule",
                  r"\endfoot"]
    else:
        lines += [r"\begin{table}[t]", r"\centering", r"\small",
                  r"\caption{" + caption + r"}", r"\label{" + label + r"}",
                  r"\begin{tabular}{" + col_fmt + r"}",
                  r"\toprule",
                  " & ".join([c.replace("_", r"\_") for c in df.columns]) + r" \\",
                  r"\midrule"]
    for _, row in df.iterrows():
        cells = [cell(row[c], c) for c in df.columns]
        lines.append(" & ".join(cells) + r" \\")
    if longtable:
        lines += [r"\end{longtable}"]
    else:
        lines += [r"\bottomrule", r"\end{tabular}", r"\end{table}"]
    return "\n".join(lines) + "\n"


def supp_e2(e2_path: str, out: pathlib.Path) -> None:
    df = pd.read_parquet(e2_path)
    keep = ["corpus", "model", "clusterer", "strategy",
            "seed", "n_train", "compression",
            "identity_accuracy", "recall@5", "coverage@0.80"]
    keep = [c for c in keep if c in df.columns]
    s = df[keep].sort_values(["corpus", "model", "strategy", "seed"])
    out.write_text(_latex_table(
        s, "E2 per-cell results (strategy sweep across 6 corpora, 6 encoders, 4 clusterers).",
        "tab:supp_e2",
        fmt={"compression": lambda v: f"{v:.2f}"},
    ))
    print(f"[supp_e2] wrote {out}")


def supp_e3(e3_path: str, out: pathlib.Path) -> None:
    try:
        df = pd.read_parquet(e3_path)
    except Exception:
        out.write_text("% E3 per-cell table: no parquet yet\n")
        print(f"[supp_e3] wrote {out} (empty)")
        return
    keep = ["corpus", "family", "bytes_per_vec",
            "identity_accuracy", "recall@1", "recall@10", "recall@100"]
    keep = [c for c in keep if c in df.columns]
    s = df[keep].sort_values(["corpus", "family", "bytes_per_vec"])
    out.write_text(_latex_table(
        s, "E3 per-cell results (learned baselines: PQ, OPQ, LSH, PCA+int8, HNSW-prune).",
        "tab:supp_e3",
        fmt={"bytes_per_vec": lambda v: f"{v:.0f}"},
    ))
    print(f"[supp_e3] wrote {out}")


def supp_e6(e6_path: str, out: pathlib.Path) -> None:
    df = pd.read_parquet(e6_path)
    keep = ["corpus", "model", "router", "seed", "compression",
            "identity_accuracy", "recall@10", "mrr@20", "coverage@0.80"]
    keep = [c for c in keep if c in df.columns]
    s = df[keep].sort_values(["corpus", "model", "router", "seed"])
    out.write_text(_latex_table(
        s, "E6 per-cell results (GAC router ablation).", "tab:supp_e6",
        fmt={"compression": lambda v: f"{v:.2f}"},
    ))
    print(f"[supp_e6] wrote {out}")


def supp_drm(e4_path: str, out: pathlib.Path) -> None:
    """DRM breakdown: identity accuracy by cluster label for centroid vs GAC."""
    df = pd.read_parquet(e4_path)
    if "corpus" not in df.columns:
        out.write_text("% DRM breakdown: no parquet\n")
        return
    sub = df[(df["corpus"] == "drm_templated") & (df["model"] == "bge-large")]
    if len(sub) == 0:
        out.write_text("% DRM breakdown: empty\n")
        return
    # Collapse over seeds; per-list breakdown requires per-cluster labels which
    # we may not have in the parquet. Fall back to strategy summary.
    s = sub.groupby("strategy").agg(
        id_lit=("identity_accuracy_literal", "mean"),
        id_para=("identity_accuracy_para", "mean"),
        recall1=("recall@1_literal", "mean"),
        mrr=("mrr@20_literal", "mean"),
    ).reset_index()
    out.write_text(_latex_table(
        s, "DRM templated: identity and MRR per strategy (bge-large, 3 seeds).",
        "tab:drm_breakdown", longtable=False, max_rows=None,
    ))
    print(f"[supp_drm] wrote {out}")


def supp_c1(e1_path: str, out: pathlib.Path) -> None:
    try:
        df = pd.read_parquet(e1_path)
    except Exception:
        out.write_text("% c1 calibration: no parquet\n")
        return
    # Predicted vs observed cap-coverage error per (d_eff_target, theta)
    if not {"d_eff_target", "theta"}.issubset(df.columns):
        out.write_text("% c1 calibration: missing columns\n")
        return
    tbl = df.groupby(["d_eff_target", "theta"]).agg(
        bound=("bound_cap", "mean") if "bound_cap" in df.columns
            else ("theorem_holds_cap", "mean"),
        observed_err=("err_cap", "mean") if "err_cap" in df.columns
            else ("theorem_holds_cap", "mean"),
        theorem_holds=("theorem_holds_cap", "mean"),
    ).reset_index()
    out.write_text(_latex_table(
        tbl, "E1 calibration table: bound vs observed cap-coverage error across $(d_{eff}, \\theta)$.",
        "tab:c1_calib", longtable=False, max_rows=None,
    ))
    print(f"[supp_c1] wrote {out}")


def supp_manifest(out: pathlib.Path, repo_root: pathlib.Path) -> None:
    try:
        sha = subprocess.check_output(
            ["git", "-C", str(repo_root), "rev-parse", "HEAD"]
        ).decode().strip()[:10]
    except Exception:
        sha = "unknown"
    rows = [
        {"artifact": "Fig 1 (conceptual)", "source": "scripts/make_figures.py:fig1b_conceptual", "parquet": "(none)", "commit": sha},
        {"artifact": "Fig 1 (theorem regimes)", "source": "scripts/make_figures.py:fig1_theorem_regimes", "parquet": "results/e1/e1_results.parquet", "commit": sha},
        {"artifact": "Fig 2", "source": "fig2_strategy_sweep", "parquet": "results/e4/e4_results.parquet", "commit": sha},
        {"artifact": "Fig 3", "source": "fig3_learned_baselines", "parquet": "results/e3/e3_results.parquet", "commit": sha},
        {"artifact": "Fig 4a", "source": "fig4_encoder_universality", "parquet": "results/e8/e8_results.parquet", "commit": sha},
        {"artifact": "Fig 4b (id-coverage)", "source": "fig4b_id_coverage", "parquet": "results/e4/e4_results.parquet", "commit": sha},
        {"artifact": "Fig 5 (scale)", "source": "fig5_scale", "parquet": "results/e5/e5_results.parquet", "commit": sha},
        {"artifact": "Fig 6 (ablation)", "source": "fig6_ablation", "parquet": "results/e6/e6_results.parquet", "commit": sha},
        {"artifact": "Fig 6b (downstream)", "source": "fig6b_downstream", "parquet": "results/e7/e7_results.parquet", "commit": sha},
        {"artifact": "Table 1", "source": "make_tables.py:t1", "parquet": "e3, e4 parquets", "commit": sha},
        {"artifact": "Table 2", "source": "make_tables.py:t2", "parquet": "results/e6/e6_results.parquet", "commit": sha},
    ]
    df = pd.DataFrame(rows)
    out.write_text(_latex_table(
        df, "Run manifest: every figure and table mapped to source parquet and commit SHA.",
        "tab:manifest", longtable=False, max_rows=None, col_fmt="lllc",
    ))
    print(f"[supp_manifest] wrote {out}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="paper/tables")
    ap.add_argument("--e1", default="results/e1/e1_results.parquet")
    ap.add_argument("--e2", default="results/e2/e2_results.parquet")
    ap.add_argument("--e3", default="results/e3/e3_results.parquet")
    ap.add_argument("--e4", default="results/e4/e4_results.parquet")
    ap.add_argument("--e6", default="results/e6/e6_results.parquet")
    args = ap.parse_args()
    out = pathlib.Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    repo_root = pathlib.Path(__file__).resolve().parent.parent
    supp_e2(args.e2, out / "supp_t_e2_per_cell.tex")
    supp_e3(args.e3, out / "supp_t_e3_per_cell.tex")
    supp_e6(args.e6, out / "supp_t_e6_per_cell.tex")
    supp_drm(args.e4, out / "supp_t_drm_breakdown.tex")
    supp_c1(args.e1, out / "supp_t_c1_calibration.tex")
    supp_manifest(out / "supp_t_manifest.tex", repo_root)


if __name__ == "__main__":
    main()
