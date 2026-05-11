"""
Empirical c1 calibration.

The theorem bound is err_cap <= c1 * d_eff_local * (1 - theta)^((d_eff_local-1)/2).
Our original c1 used worst-case cap-volume constants. We calibrate by finding the
smallest c1 such that bound >= err_cap for >= target_frac of cells.

We report:
  - Global c1 (across all E1 cells)
  - Per-regime c1 split by the PAPER'S regime definition
    (tight: d_bar_realized < theta';  spread: d_bar_realized >= theta'),
    where theta' = 1 - theta.
  - For downstream corpora: use their d_eff_local from E4/E8 to compute
    calibrated bounds, check against empirical err_cap from E8.
"""
from __future__ import annotations
import argparse, json
from pathlib import Path
import numpy as np
import pandas as pd


def bound_at(c1: float, d_eff: float, theta: float) -> float:
    return c1 * d_eff * (1.0 - theta) ** ((d_eff - 1.0) / 2.0)


def calibrate(err: np.ndarray, d_eff: np.ndarray, theta: np.ndarray, target_frac: float = 0.95) -> float:
    """Find min c1 such that bound(c1) >= err for target_frac of rows."""
    # Per-row minimum c1 needed for bound >= err:
    # err <= c1 * d * (1-theta)^((d-1)/2)  =>  c1_row = err / (d * (1-theta)^((d-1)/2))
    denom = d_eff * (1.0 - theta) ** ((d_eff - 1.0) / 2.0)
    denom = np.where(denom <= 1e-12, 1e-12, denom)
    c1_per_row = err / denom
    c1_per_row = c1_per_row[np.isfinite(c1_per_row)]
    c1_per_row = np.clip(c1_per_row, 0, None)
    if len(c1_per_row) == 0:
        return float("nan")
    # Take target_frac quantile
    return float(np.quantile(c1_per_row, target_frac))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--e1", default="results/e1/e1_results.parquet")
    ap.add_argument("--out", default="results/c1_calibration.json")
    args = ap.parse_args()

    df = pd.read_parquet(args.e1)
    # Use per-cluster d_eff (local); this is what the theorem actually uses
    df = df[df["err_cap"].notna() & df["d_eff_per_cluster"].notna()].copy()
    # GAC strategy cells carry gac's err but the geometry (d_eff, theta) is the cell's
    out = {"n_rows": int(len(df))}

    # Global
    out["global_c1_p95"] = calibrate(df["err_cap"].values, df["d_eff_per_cluster"].values, df["theta"].values, 0.95)
    out["global_c1_p99"] = calibrate(df["err_cap"].values, df["d_eff_per_cluster"].values, df["theta"].values, 0.99)

    # By strategy
    out["by_strategy"] = {}
    for strat, sub in df.groupby("strategy"):
        out["by_strategy"][strat] = {
            "c1_p95": calibrate(sub["err_cap"].values, sub["d_eff_per_cluster"].values, sub["theta"].values, 0.95),
            "c1_p99": calibrate(sub["err_cap"].values, sub["d_eff_per_cluster"].values, sub["theta"].values, 0.99),
            "n": int(len(sub)),
        }

    # By regime (paper's definition: dbar < theta' is tight)
    thetap = 1.0 - df["theta"]
    dbar = df["d_bar_realized"]
    tight = df[dbar < thetap]
    spread = df[dbar >= thetap]
    out["tight_regime"] = {
        "n": int(len(tight)),
        "definition": "dbar < theta' (theta' = 1 - theta)",
        "c1_p95": calibrate(tight["err_cap"].values, tight["d_eff_per_cluster"].values, tight["theta"].values, 0.95),
    }
    out["spread_regime"] = {
        "n": int(len(spread)),
        "definition": "dbar >= theta'",
        "c1_p95": calibrate(spread["err_cap"].values, spread["d_eff_per_cluster"].values, spread["theta"].values, 0.95),
    }

    # By theta band
    out["by_theta"] = {}
    for t in sorted(df["theta"].unique()):
        sub = df[np.isclose(df["theta"], t)]
        out["by_theta"][f"{t:.2f}"] = {
            "n": int(len(sub)),
            "c1_p95": calibrate(sub["err_cap"].values, sub["d_eff_per_cluster"].values, sub["theta"].values, 0.95),
        }

    # Compute per-row slack after calibration with p95 c1
    c1_cal = out["global_c1_p95"]
    df["bound_cal"] = c1_cal * df["d_eff_per_cluster"] * (1.0 - df["theta"]) ** ((df["d_eff_per_cluster"] - 1.0) / 2.0)
    df["holds_cal"] = df["err_cap"] <= df["bound_cal"] + 1e-9
    out["pct_holds_global_cal"] = float(df["holds_cal"].mean())
    # Spread regime coverage under global c1
    sp = df[dbar >= thetap]
    out["pct_holds_spread_cal"] = float(sp["holds_cal"].mean()) if len(sp) else float("nan")

    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    with open(args.out, "w") as f:
        json.dump(out, f, indent=2)
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
