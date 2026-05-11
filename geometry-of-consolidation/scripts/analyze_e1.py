"""
Analyse E1 results and print every number we might cite in the paper.
Runs against results/e1/e1_results.parquet (16000 rows, 400 cells, 4 strategies).
"""
import pandas as pd
import numpy as np

df = pd.read_parquet("results/e1/e1_results.parquet")
print("==== E1 headline summary ====")
print(f"n_rows = {len(df)}")
print(f"n_cells = {df['cell_id'].nunique()}")
print(f"strategies = {sorted(df['strategy'].unique())}")
print()

print("==== By strategy ====")
g = df.groupby("strategy").agg(
    err_cluster=("err_cluster", "mean"),
    err_cap=("err_cap", "mean"),
    err_source=("err_source", "mean"),
    bound=("bound", "mean"),
    cap_holds=("theorem_holds_cap", "mean"),
    source_holds=("theorem_holds_source", "mean"),
    compression=("compression", "mean"),
).round(4)
print(g.to_string())
print()

# Is the theorem ever violated strictly (both metrics)?
print("==== Strict theorem violations ====")
# Source strict: err_source >= bound is what the theorem *actually* predicts
# when applied to strict identity retrieval (since err_source upper-bounds
# the cap-escape fraction for any SINGLE representative).
src_viol = df[df["theorem_holds_source"] == False]
print(f"Rows where err_source < bound (strict violations): {len(src_viol)} / {len(df)}")
print(f"  min gap when violated: {src_viol['gap_source'].min():.4f}")
print(f"  mean gap when violated: {src_viol['gap_source'].mean():.4f}")
print()
cap_viol = df[df["theorem_holds_cap"] == False]
print(f"Rows where err_cap < bound (cap violations): {len(cap_viol)} / {len(df)}")
print(f"  min gap when violated: {cap_viol['gap_cap'].min():.4f}")
print(f"  mean gap when violated: {cap_viol['gap_cap'].mean():.4f}")
print()

print("==== Regime separation (the qualitative claim) ====")
# Is there a clear 'tight' regime where all strategies succeed and a 'spread' regime?
tight = df[df["d_bar_realized"] < (1 - df["theta"])]  # d_bar < theta'
spread = df[df["d_bar_realized"] >= (1 - df["theta"])]
print(f"Tight regime cells (d_bar < theta'): {tight['cell_id'].nunique()}, rows={len(tight)}")
print(f"  err_cap mean by strategy:")
print(tight.groupby("strategy")["err_cap"].mean().round(4).to_string())
print(f"  bound mean in tight: {tight['bound'].mean():.4f}")
print()
print(f"Spread regime cells (d_bar >= theta'): {spread['cell_id'].nunique()}, rows={len(spread)}")
print(f"  err_cap mean by strategy:")
print(spread.groupby("strategy")["err_cap"].mean().round(4).to_string())
print(f"  bound mean in spread: {spread['bound'].mean():.4f}")
print()

print("==== Pareto dominance of GAC (the headline empirical claim) ====")
# For each (d_eff, theta, d_bar) cell, compare strategies on err_cap.
# GAC dominates if err_gac <= err_other - eps for most cells.
pivot = df.groupby(["cell_id", "strategy"])["err_cap"].mean().unstack()
n_cells = len(pivot)
for baseline in ["centroid", "medoid", "prune50"]:
    wins = (pivot["gac"] < pivot[baseline] - 0.005).sum()
    ties = ((pivot["gac"] - pivot[baseline]).abs() <= 0.005).sum()
    losses = (pivot["gac"] > pivot[baseline] + 0.005).sum()
    diff = (pivot["gac"] - pivot[baseline])
    print(f"GAC vs {baseline}: wins={wins}, ties={ties}, losses={losses} "
          f"mean_diff={diff.mean():+.4f} max_improvement={-diff.min():+.4f}")
print()

print("==== Compression story ====")
print("Mean # representatives and compression ratio by strategy:")
comp = df.groupby("strategy").agg(
    n_rep=("n_representatives", "mean"),
    compression=("compression", "mean"),
).round(2)
print(comp)
print()

print("==== Critical regime: d_bar near theta' ====")
# Where it matters most: d_bar within [0.8, 1.5] * theta'
theta_prime = 1 - df["theta"]
crit = df[(df["d_bar_realized"] >= 0.8 * theta_prime) & (df["d_bar_realized"] <= 1.5 * theta_prime)]
print(f"Critical rows: {len(crit)}")
print(crit.groupby("strategy")[["err_cap", "err_source", "bound"]].mean().round(4).to_string())
print()

print("==== Cap-coverage error vs d_eff (should scale with theorem) ====")
print("err_cap by d_eff_target (averaged over all strategies, spread regime only):")
print(spread.groupby("d_eff_target")["err_cap"].mean().round(4).to_string())
print()
print("bound by d_eff_target (spread regime only):")
print(spread.groupby("d_eff_target")["bound"].mean().round(4).to_string())
