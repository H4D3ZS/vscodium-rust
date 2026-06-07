#!/usr/bin/env python3
"""ML Studio toolkit — dataset stats, model summary, export, gallery, HPO, debug."""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

import pandas as pd
import torch
import torch.nn as nn


class MLP(nn.Module):
    def __init__(self, in_dim: int, hidden: int, out_dim: int):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(in_dim, hidden),
            nn.ReLU(),
            nn.Dropout(0.1),
            nn.Linear(hidden, hidden // 2),
            nn.ReLU(),
            nn.Linear(hidden // 2, out_dim),
        )

    def forward(self, x):
        return self.net(x)


def load_checkpoint(path: Path):
    ckpt = torch.load(path, map_location="cpu", weights_only=False)
    feature_cols = ckpt["feature_cols"]
    classes = ckpt["classes"]
    hidden = int(ckpt.get("hidden_size", 64))
    model = MLP(len(feature_cols), hidden, len(classes))
    model.load_state_dict(ckpt["state_dict"])
    model.eval()
    return model, ckpt


def git_hash(root: Path) -> str | None:
    try:
        out = subprocess.check_output(
            ["git", "rev-parse", "--short", "HEAD"],
            cwd=str(root),
            stderr=subprocess.DEVNULL,
            text=True,
        )
        return out.strip() or None
    except Exception:
        return None


def cmd_dataset_stats(args):
    path = Path(args.csv)
    df = pd.read_csv(path)
    target = args.target
    preview = df.head(int(args.preview_rows)).fillna("").astype(str).to_dict(orient="records")
    numeric = df.select_dtypes(include="number")
    col_stats = []
    for col in df.columns:
        s = df[col]
        entry = {"column": col, "dtype": str(s.dtype), "nulls": int(s.isna().sum()), "unique": int(s.nunique())}
        if pd.api.types.is_numeric_dtype(s):
            entry.update(
                {
                    "min": float(s.min()) if s.notna().any() else None,
                    "max": float(s.max()) if s.notna().any() else None,
                    "mean": float(s.mean()) if s.notna().any() else None,
                    "std": float(s.std()) if s.notna().any() else None,
                }
            )
        col_stats.append(entry)
    class_distribution = {}
    if target and target in df.columns:
        vc = df[target].value_counts().head(50)
        class_distribution = {str(k): int(v) for k, v in vc.items()}
    split = {}
    ml = path.parent.parent if path.parent.name == "data" else None
    if ml and (ml / "data" / "train.csv").exists():
        tr = pd.read_csv(ml / "data" / "train.csv")
        va = pd.read_csv(ml / "data" / "val.csv") if (ml / "data" / "val.csv").exists() else None
        split = {"train_rows": len(tr), "val_rows": len(va) if va is not None else 0}
        if target and target in tr.columns:
            split["train_classes"] = tr[target].nunique()
            split["val_classes"] = va[target].nunique() if va is not None else 0
    out = {
        "rows": len(df),
        "columns": list(df.columns),
        "preview": preview,
        "column_stats": col_stats,
        "class_distribution": class_distribution,
        "split": split,
        "memory_mb": round(df.memory_usage(deep=True).sum() / (1024 * 1024), 2),
    }
    print(json.dumps(out))


def cmd_model_summary(args):
    model, ckpt = load_checkpoint(Path(args.model))
    layers = []
    total = 0
    trainable = 0
    for name, mod in model.named_modules():
        if name == "":
            continue
        if len(list(mod.children())) > 0:
            continue
        n_params = sum(p.numel() for p in mod.parameters())
        n_train = sum(p.numel() for p in mod.parameters() if p.requires_grad)
        if n_params == 0:
            continue
        layers.append(
            {
                "name": name,
                "type": mod.__class__.__name__,
                "params": n_params,
                "trainable": n_train,
            }
        )
        total += n_params
        trainable += n_train
    size_mb = Path(args.model).stat().st_size / (1024 * 1024)
    sample = torch.zeros(1, len(ckpt["feature_cols"]))
    with torch.no_grad():
        out_shape = list(model(sample).shape)
    print(
        json.dumps(
            {
                "layers": layers,
                "total_params": total,
                "trainable_params": trainable,
                "non_trainable_params": total - trainable,
                "size_mb": round(size_mb, 3),
                "input_shape": [1, len(ckpt["feature_cols"])],
                "output_shape": out_shape,
                "classes": ckpt["classes"],
                "feature_cols": ckpt["feature_cols"],
            }
        )
    )


def cmd_export(args):
    model_path = Path(args.model)
    fmt = args.format.lower()
    out_dir = model_path.parent / "export"
    out_dir.mkdir(exist_ok=True)
    model, ckpt = load_checkpoint(model_path)
    n_in = len(ckpt["feature_cols"])
    dummy = torch.randn(1, n_in)
    results = {"format": fmt, "files": []}
    if fmt in ("torchscript", "all"):
        ts = torch.jit.trace(model, dummy)
        p = out_dir / "model.ts.pt"
        ts.save(str(p))
        results["files"].append(str(p))
    if fmt in ("onnx", "all"):
        p = out_dir / "model.onnx"
        torch.onnx.export(
            model,
            dummy,
            str(p),
            input_names=["features"],
            output_names=["logits"],
            dynamic_axes={"features": {0: "batch"}, "logits": {0: "batch"}},
            opset_version=17,
        )
        results["files"].append(str(p))
    if fmt in ("int8", "fp16", "all"):
        try:
            qmodel = model
            if fmt == "fp16" or fmt == "all":
                qmodel = model.half()
                p = out_dir / "model.fp16.pt"
                torch.save({"state_dict": qmodel.state_dict(), **{k: ckpt[k] for k in ("feature_cols", "classes", "hidden_size")}}, p)
                results["files"].append(str(p))
        except Exception as e:
            results["quantize_note"] = str(e)
    results["ok"] = True
    print(json.dumps(results))


def cmd_gallery(_args):
    models = []
    try:
        import torchvision.models as tvm

        for name in sorted([n for n in dir(tvm) if not n.startswith("_")]):
            obj = getattr(tvm, name, None)
            if callable(obj) and name[0].islower() and name[0].isalpha():
                models.append({"id": name, "source": "torchvision", "task": "vision"})
    except ImportError:
        pass
    templates = [
        {"id": "tabular_mlp", "source": "builtin", "task": "tabular", "desc": "Current ML Studio MLP classifier"},
        {"id": "resnet18", "source": "torchvision", "task": "vision", "desc": "Transfer learning starter"},
        {"id": "efficientnet_b0", "source": "torchvision", "task": "vision", "desc": "Efficient image classifier"},
    ]
    for t in templates:
        if not any(m["id"] == t["id"] for m in models):
            models.append(t)
    print(json.dumps({"models": models[:80]}))


def cmd_hpo(args):
    root = Path(args.root)
    ml = root / ".hades" / "ml"
    cfg = json.loads((ml / "config.json").read_text(encoding="utf-8"))
    manifest = json.loads((ml / "manifest.json").read_text(encoding="utf-8"))
    train_df = pd.read_csv(root / manifest["train_csv"])
    val_df = pd.read_csv(root / manifest["val_csv"])
    target = manifest["target_column"]
    feature_cols = [c for c in train_df.columns if c != target]
    classes = sorted(train_df[target].unique().tolist())
    class_to_idx = {c: i for i, c in enumerate(classes)}

    def eval_config(hidden, lr, epochs):
        xt = torch.tensor(train_df[feature_cols].values, dtype=torch.float32)
        yt = torch.tensor([class_to_idx[v] for v in train_df[target]], dtype=torch.long)
        xv = torch.tensor(val_df[feature_cols].values, dtype=torch.float32)
        yv = torch.tensor([class_to_idx[v] for v in val_df[target]], dtype=torch.long)
        model = MLP(len(feature_cols), int(hidden), len(classes))
        opt = torch.optim.Adam(model.parameters(), lr=float(lr))
        loss_fn = nn.CrossEntropyLoss()
        for _ in range(int(epochs)):
            model.train()
            opt.zero_grad()
            loss_fn(model(xt), yt).backward()
            opt.step()
        model.eval()
        with torch.no_grad():
            acc = float((model(xv).argmax(1) == yv).float().mean().item())
        return acc

    mode = args.mode
    trials = []
    if mode == "grid":
        grid = [
            {"hidden_size": h, "learning_rate": lr, "epochs": 5}
            for h in [32, 64, 128]
            for lr in [1e-3, 5e-4, 1e-4]
        ][: int(args.trials)]
        for i, g in enumerate(grid):
            acc = eval_config(g["hidden_size"], g["learning_rate"], g["epochs"])
            trials.append({"trial": i + 1, **g, "val_acc": round(acc, 4)})
    else:
        import random

        for i in range(int(args.trials)):
            g = {
                "hidden_size": random.choice([32, 64, 96, 128, 192]),
                "learning_rate": 10 ** random.uniform(-4, -2.3),
                "epochs": 5,
            }
            acc = eval_config(g["hidden_size"], g["learning_rate"], g["epochs"])
            trials.append({"trial": i + 1, **g, "val_acc": round(acc, 4)})
    trials.sort(key=lambda t: t["val_acc"], reverse=True)
    best = trials[0] if trials else {}
    out_path = ml / "hpo_results.json"
    out_path.write_text(json.dumps({"mode": mode, "trials": trials, "best": best}, indent=2), encoding="utf-8")
    if best:
        cfg["hidden_size"] = int(best["hidden_size"])
        cfg["learning_rate"] = float(best["learning_rate"])
        (ml / "config.json").write_text(json.dumps(cfg, indent=2), encoding="utf-8")
    print(json.dumps({"ok": True, "best": best, "trials": len(trials)}))


def cmd_lr_finder(args):
    root = Path(args.root)
    ml = root / ".hades" / "ml"
    manifest = json.loads((ml / "manifest.json").read_text(encoding="utf-8"))
    train_df = pd.read_csv(root / manifest["train_csv"])
    target = manifest["target_column"]
    feature_cols = [c for c in train_df.columns if c != target]
    classes = sorted(train_df[target].unique().tolist())
    class_to_idx = {c: i for i, c in enumerate(classes)}
    x = torch.tensor(train_df[feature_cols].values, dtype=torch.float32)
    y = torch.tensor([class_to_idx[v] for v in train_df[target]], dtype=torch.long)
    cfg = json.loads((ml / "config.json").read_text(encoding="utf-8"))
    hidden = int(cfg.get("hidden_size", 64))
    model = MLP(len(feature_cols), hidden, len(classes))
    loss_fn = nn.CrossEntropyLoss()
    lrs = []
    losses = []
    opt = torch.optim.Adam(model.parameters(), lr=1e-7)
    for i in range(int(args.steps)):
        lr = 10 ** (-7 + (6 * i / max(int(args.steps) - 1, 1)))
        for pg in opt.param_groups:
            pg["lr"] = lr
        opt.zero_grad()
        loss = loss_fn(model(x), y)
        if torch.isnan(loss) or torch.isinf(loss):
            break
        loss.backward()
        opt.step()
        lrs.append(lr)
        losses.append(float(loss.item()))
    suggested = lrs[losses.index(min(losses))] if losses else 1e-3
    out = {"lrs": lrs, "losses": losses, "suggested_lr": suggested}
    (ml / "lr_finder.json").write_text(json.dumps(out, indent=2), encoding="utf-8")
    print(json.dumps(out))


def cmd_grad_check(args):
    model_path = Path(args.model)
    model, ckpt = load_checkpoint(model_path)
    x = torch.randn(4, len(ckpt["feature_cols"]), requires_grad=True)
    y = torch.randint(0, len(ckpt["classes"]), (4,))
    out = model(x)
    loss = nn.functional.cross_entropy(out, y)
    loss.backward()
    grad_norms = []
    vanishing = 0
    exploding = 0
    for name, p in model.named_parameters():
        if p.grad is None:
            continue
        g = float(p.grad.norm().item())
        grad_norms.append({"layer": name, "grad_norm": round(g, 6)})
        if g < 1e-7:
            vanishing += 1
        if g > 1e3:
            exploding += 1
    has_nan = any(p.grad is not None and torch.isnan(p.grad).any() for p in model.parameters())
    print(
        json.dumps(
            {
                "grad_norms": grad_norms,
                "vanishing_layers": vanishing,
                "exploding_layers": exploding,
                "nan_gradients": bool(has_nan),
                "ok": not has_nan and exploding == 0,
            }
        )
    )


def cmd_benchmark(args):
    model_path = Path(args.model)
    model, ckpt = load_checkpoint(model_path)
    n = int(args.iterations)
    x = torch.randn(1, len(ckpt["feature_cols"]))
    t0 = time.perf_counter()
    with torch.no_grad():
        for _ in range(n):
            model(x)
    elapsed = time.perf_counter() - t0
    print(json.dumps({"iterations": n, "total_ms": round(elapsed * 1000, 2), "per_iter_ms": round(elapsed / n * 1000, 3)}))


def main():
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="cmd", required=True)

    ds = sub.add_parser("dataset_stats")
    ds.add_argument("--csv", required=True)
    ds.add_argument("--target", default="")
    ds.add_argument("--preview-rows", default="8")

    ms = sub.add_parser("model_summary")
    ms.add_argument("--model", required=True)

    ex = sub.add_parser("export")
    ex.add_argument("--model", required=True)
    ex.add_argument("--format", default="all")

    sub.add_parser("gallery")

    hpo = sub.add_parser("hpo")
    hpo.add_argument("--root", required=True)
    hpo.add_argument("--mode", choices=["grid", "random"], default="random")
    hpo.add_argument("--trials", default="9")

    lr = sub.add_parser("lr_finder")
    lr.add_argument("--root", required=True)
    lr.add_argument("--steps", default="20")

    gc = sub.add_parser("grad_check")
    gc.add_argument("--model", required=True)

    bm = sub.add_parser("benchmark")
    bm.add_argument("--model", required=True)
    bm.add_argument("--iterations", default="200")

    args = p.parse_args()
    cmds = {
        "dataset_stats": cmd_dataset_stats,
        "model_summary": cmd_model_summary,
        "export": cmd_export,
        "gallery": cmd_gallery,
        "hpo": cmd_hpo,
        "lr_finder": cmd_lr_finder,
        "grad_check": cmd_grad_check,
        "benchmark": cmd_benchmark,
    }
    cmds[args.cmd](args)


if __name__ == "__main__":
    main()
