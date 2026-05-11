"""Analyse E2 results. Every number printed here is citeable in the paper."""
import json
import pandas as pd
import numpy as np

pd.set_option("display.width", 240)
pd.set_option("display.max_columns", 40)

df = pd.read_parquet("results/e2/e2_results.parquet")
print("==== E2 shape ====")
print(f"n_rows = {len(df)}")
print(f"corpora = {sorted(df['corpus'].unique())}")
print(f"strategies = {sorted(df['strategy'].unique())}")
print(f"clusterers = {sorted(df['clusterer'].unique())}")
print()

print("==== Columns ====")
print(df.columns.tolist())
print()

# Build short strategy key
def strat_key(row):
    kw = json.loads(row["strategy_kw"]) if row["strategy_kw"] else {}
    if not kw:
        return row["strategy"]
    ksig = ",".join(f"{k}={v}" for k, v in sorted(kw.items()))
    return f"{row['strategy']}({ksig})"
df["strat_full"] = df.apply(strat_key, axis=1)

print("==== Per-corpus, per-strategy headline metrics ====")
headline = df.groupby(["corpus", "strat_full"]).agg(
    identity_acc=("identity_accuracy", "mean"),
    err_cap_08=("err_cap_0.80", "mean"),
    err_cap_09=("err_cap_0.90", "mean"),
    bound_08=("bound_0.80", "mean"),
    bound_09=("bound_0.90", "mean"),
    compression=("compression", "mean"),
    d_eff_local=("d_eff_local_mean", "mean"),
    d_bar=("d_bar_mean", "mean"),
).round(4)
print(headline.to_string())
print()

print("==== Theorem holds? ====")
th = df.groupby(["corpus", "strat_full"]).agg(
    holds_08=("theorem_holds_0.80", "all"),
    holds_09=("theorem_holds_0.90", "all"),
    gap_08_min=("err_cap_0.80", "min"),
    bound_08_max=("bound_0.80", "max"),
)
th["gap_to_bound_08"] = (df.groupby(["corpus","strat_full"])["err_cap_0.80"].mean() -
                         df.groupby(["corpus","strat_full"])["bound_0.80"].mean()).round(4)
print(th.to_string())
print()

print("==== Per-corpus d_eff and d_bar ====")
cstats = df.groupby("corpus").agg(
    d_eff_global=("d_eff_global", "first"),
    d_eff_local=("d_eff_local_mean", "first"),
    d_bar=("d_bar_mean", "first"),
    n_train=("n_train", "first"),
    n_clusters=("n_clusters_used", "first"),
).round(3)
print(cstats.to_string())
print()

# Key question 1: Does GAC Pareto-dominate?
print("==== GAC vs centroid on each corpus (kmeans clusterer) ====")
for corpus in sorted(df["corpus"].unique()):
    sub = df[(df["corpus"] == corpus) & (df["clusterer"] == "kmeans")]
    g = sub[sub["strategy"] == "gac"].groupby("strat_full").agg(
        acc=("identity_accuracy", "mean"),
        err_cap_08=("err_cap_0.80", "mean"),
        cmp=("compression", "mean"),
    ).round(4)
    c = sub[sub["strategy"] == "centroid"].groupby("strat_full").agg(
        acc=("identity_accuracy", "mean"),
        err_cap_08=("err_cap_0.80", "mean"),
        cmp=("compression", "mean"),
    ).round(4)
    print(f"-- {corpus} --")
    print("GAC:")
    print(g.to_string())
    print("CENTROID:")
    print(c.to_string())
    print()

print("==== Identity-Coverage trade-off (SPEC C2) ====")
# Corollary 2: centroid is Bayes-opt for coverage; medoid for identity.
# Does our data show this?
tradeoff = df.groupby("strat_full").agg(
    identity_acc=("identity_accuracy", "mean"),
    coverage_08=("coverage@0.80", "mean"),
    coverage_09=("coverage@0.90", "mean"),
    compression=("compression", "mean"),
).round(4)
print(tradeoff.to_string())
print()

print("==== Template-vs-real collapse (SPEC Thm 2.1 prediction) ====")
# DRM (templated) should show near-perfect identity retrieval, wikipedia much worse.
pt = df[df["strategy"] == "centroid"].groupby("corpus").agg(
    identity_acc=("identity_accuracy", "mean"),
    err_cap_08=("err_cap_0.80", "mean"),
    d_eff_local=("d_eff_local_mean", "first"),
    d_bar=("d_bar_mean", "first"),
).round(4)
print(pt.to_string())
