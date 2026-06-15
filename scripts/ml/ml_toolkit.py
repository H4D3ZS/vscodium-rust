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
    if fmt in ("int8", "all"):
        try:
            qmodel = torch.quantization.quantize_dynamic(model, {nn.Linear}, dtype=torch.qint8)
            p = out_dir / "model.int8.pt"
            torch.save({"state_dict": qmodel.state_dict(), **{k: ckpt[k] for k in ("feature_cols", "classes", "hidden_size") if k in ckpt}}, p)
            results["files"].append(str(p))
        except Exception as e:
            results["quantize_note"] = str(e)
    if fmt in ("fp16", "all"):
        try:
            qmodel = model.half()
            p = out_dir / "model.fp16.pt"
            torch.save({"state_dict": qmodel.state_dict(), **{k: ckpt[k] for k in ("feature_cols", "classes", "hidden_size") if k in ckpt}}, p)
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
    try:
        import timm

        for name in timm.list_models(pretrained=True)[:40]:
            models.append({"id": name, "source": "timm", "task": "vision", "desc": "timm pretrained"})
    except ImportError:
        pass
    hf_models = [
        {"id": "bert-base-uncased", "source": "huggingface", "task": "nlp", "desc": "BERT base encoder"},
        {"id": "distilbert-base-uncased", "source": "huggingface", "task": "nlp", "desc": "DistilBERT"},
        {"id": "google/vit-base-patch16-224", "source": "huggingface", "task": "vision", "desc": "ViT base"},
        {"id": "microsoft/resnet-50", "source": "huggingface", "task": "vision", "desc": "ResNet-50 HF weights"},
    ]
    templates = [
        {"id": "tabular_mlp", "source": "builtin", "task": "tabular", "desc": "ML Studio tabular MLP classifier"},
        {"id": "resnet18", "source": "torchvision", "task": "vision", "desc": "Transfer learning starter"},
        {"id": "efficientnet_b0", "source": "torchvision", "task": "vision", "desc": "Efficient image classifier"},
    ]
    for t in templates + hf_models:
        if not any(m["id"] == t["id"] for m in models):
            models.append(t)
    print(json.dumps({"models": models[:120]}))


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
    if mode == "optuna":
        import optuna

        optuna.logging.set_verbosity(optuna.logging.WARNING)

        def objective(trial):
            h = trial.suggest_categorical("hidden_size", [32, 64, 96, 128, 192])
            lr_v = trial.suggest_float("learning_rate", 1e-4, 1e-2, log=True)
            ep = trial.suggest_int("epochs", 3, 8)
            acc = eval_config(h, lr_v, ep)
            return acc

        study = optuna.create_study(direction="maximize")
        study.optimize(objective, n_trials=int(args.trials))
        for i, t in enumerate(study.trials):
            if t.value is None:
                continue
            trials.append(
                {
                    "trial": i + 1,
                    "hidden_size": t.params.get("hidden_size"),
                    "learning_rate": t.params.get("learning_rate"),
                    "epochs": t.params.get("epochs"),
                    "val_acc": round(float(t.value), 4),
                }
            )
        trials.sort(key=lambda t: t["val_acc"], reverse=True)
    elif mode == "grid":
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


_LAYER_COLORS = {
    "Linear": "#30b060",
    "ReLU": "#b03030",
    "Dropout": "#c09020",
    "Sequential": "#6080a0",
    "Conv2d": "#3080c0",
    "BatchNorm2d": "#6080a0",
}


def _svg_model_graph(model: nn.Module, input_shape: list, output_shape: list) -> str:
    """TorchStudio-style vertical layer graph as inline SVG."""
    layers = []
    for name, mod in model.named_modules():
        if name == "" or len(list(mod.children())) > 0:
            continue
        n_params = sum(p.numel() for p in mod.parameters())
        if n_params == 0 and mod.__class__.__name__ not in ("ReLU", "Dropout"):
            continue
        layers.append({"name": name, "type": mod.__class__.__name__, "params": n_params})

    w, box_h, gap = 280, 44, 16
    h = 60 + len(layers) * (box_h + gap) + 60
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">',
        '<rect width="100%" height="100%" fill="transparent"/>',
    ]
    y = 20
    parts.append(
        f'<ellipse cx="{w//2}" cy="{y+16}" rx="70" ry="16" fill="#606060"/>'
        f'<text x="{w//2}" y="{y+20}" fill="#f0f0f0" font-size="11" text-anchor="middle">input {input_shape}</text>'
    )
    y += 40
    prev_y = y - 8
    for i, layer in enumerate(layers):
        color = _LAYER_COLORS.get(layer["type"], "#908070")
        parts.append(f'<line x1="{w//2}" y1="{prev_y}" x2="{w//2}" y2="{y}" stroke="#a0a0a0" stroke-width="1.5"/>')
        parts.append(
            f'<rect x="30" y="{y}" width="{w-60}" height="{box_h}" rx="6" fill="{color}"/>'
            f'<text x="{w//2}" y="{y+18}" fill="#f0f0f0" font-size="12" font-weight="600" text-anchor="middle">{layer["type"]}</text>'
            f'<text x="{w//2}" y="{y+34}" fill="#d0d0d0" font-size="10" text-anchor="middle">{layer["name"]} · {layer["params"]:,} params</text>'
        )
        prev_y = y + box_h
        y += box_h + gap
    parts.append(f'<line x1="{w//2}" y1="{prev_y}" x2="{w//2}" y2="{y+8}" stroke="#a0a0a0" stroke-width="1.5"/>')
    parts.append(
        f'<ellipse cx="{w//2}" cy="{y+24}" rx="70" ry="16" fill="#808080"/>'
        f'<text x="{w//2}" y="{y+28}" fill="#f0f0f0" font-size="11" text-anchor="middle">output {output_shape}</text>'
    )
    parts.append("</svg>")
    return "".join(parts)


def cmd_model_graph(args):
    model, ckpt = load_checkpoint(Path(args.model))
    sample = torch.zeros(1, len(ckpt["feature_cols"]))
    with torch.no_grad():
        out_shape = list(model(sample).shape)
    svg = _svg_model_graph(model, [1, len(ckpt["feature_cols"])], out_shape)
    out_path = Path(args.model).parent / "model_graph.svg"
    out_path.write_text(svg, encoding="utf-8")
    print(json.dumps({"svg": svg, "path": str(out_path)}))


def cmd_export_onnx(args):
    model, ckpt = load_checkpoint(Path(args.model))
    sample = torch.zeros(1, len(ckpt["feature_cols"]))
    onnx_path = Path(args.model).parent / "model.onnx"
    torch.onnx.export(
        model,
        sample,
        str(onnx_path),
        input_names=["input"],
        output_names=["output"],
        dynamic_axes={"input": {0: "batch"}, "output": {0: "batch"}},
        opset_version=17,
    )
    print(json.dumps({"ok": True, "onnx_path": str(onnx_path)}))


def cmd_augment_preview(args):
    root = Path(args.root)
    ml = root / ".hades" / "ml"
    manifest_path = ml / "manifest.json"
    previews = []
    image_dir = ml / "data" / "images"
    if image_dir.is_dir() and any(image_dir.iterdir()):
        try:
            from PIL import Image
            import base64
            from io import BytesIO
            from torchvision import transforms

            aug = transforms.Compose([
                transforms.RandomHorizontalFlip(p=0.5),
                transforms.ColorJitter(brightness=0.3, contrast=0.3),
                transforms.RandomRotation(10),
            ])
            count = 0
            for cls_dir in sorted(image_dir.iterdir()):
                if not cls_dir.is_dir():
                    continue
                for img_path in sorted(cls_dir.glob("*"))[:2]:
                    if img_path.suffix.lower() not in (".jpg", ".jpeg", ".png", ".webp", ".bmp"):
                        continue
                    img = Image.open(img_path).convert("RGB").resize((128, 128))
                    orig = img.copy()
                    aug_img = aug(img)

                    def _b64(im):
                        buf = BytesIO()
                        im.save(buf, format="PNG")
                        return base64.b64encode(buf.getvalue()).decode("ascii")

                    previews.append({
                        "kind": "image",
                        "class": cls_dir.name,
                        "file": img_path.name,
                        "original_b64": _b64(orig),
                        "augmented_b64": _b64(aug_img),
                    })
                    count += 1
                    if count >= int(args.samples):
                        break
                if count >= int(args.samples):
                    break
        except ImportError as e:
            previews.append({"kind": "error", "message": f"Pillow/torchvision required: {e}"})
    elif manifest_path.exists():
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        train_csv = root / manifest["train_csv"]
        target = manifest["target_column"]
        df = pd.read_csv(train_csv).head(int(args.samples))
        feature_cols = [c for c in df.columns if c != target]
        rows = []
        for _, row in df.iterrows():
            orig = {c: float(row[c]) if pd.api.types.is_numeric_dtype(df[c]) else str(row[c]) for c in feature_cols[:8]}
            aug = dict(orig)
            for c in feature_cols[:8]:
                if pd.api.types.is_numeric_dtype(df[c]):
                    noise = float(row[c]) * 0.05
                    aug[c] = round(float(row[c]) + noise, 4)
            rows.append({"original": orig, "augmented": aug, "target": str(row[target])})
        previews = [{"kind": "tabular", "rows": rows, "note": "Gaussian noise (+5%) on numeric features"}]
    else:
        previews = [{"kind": "error", "message": "Prepare dataset first or add images under .hades/ml/data/images/<class>/"}]
    out = {"previews": previews}
    (ml / "augment_preview.json").write_text(json.dumps(out, indent=2), encoding="utf-8")
    print(json.dumps(out))


def cmd_load_pretrained(args):
    root = Path(args.root)
    ml = root / ".hades" / "ml"
    models_dir = ml / "models"
    models_dir.mkdir(parents=True, exist_ok=True)
    model_id = args.model_id
    source = args.source
    meta = {"id": model_id, "source": source, "loaded_at": int(time.time())}

    if source == "builtin" or model_id == "tabular_mlp":
        meta["task"] = "tabular"
        meta["note"] = "Uses built-in MLP for CSV training"
    elif source == "torchvision":
        import torchvision.models as tvm

        factory = getattr(tvm, model_id, None)
        if not callable(factory):
            raise SystemExit(f"Unknown torchvision model: {model_id}")
        net = factory(weights="DEFAULT")
        path = models_dir / f"{model_id}.pt"
        torch.save({"state_dict": net.state_dict(), "model_id": model_id, "source": source}, path)
        meta.update({"path": str(path), "task": "vision", "params": sum(p.numel() for p in net.parameters())})
    elif source == "timm":
        import timm

        net = timm.create_model(model_id, pretrained=True)
        path = models_dir / f"{model_id.replace('/', '_')}.pt"
        torch.save({"state_dict": net.state_dict(), "model_id": model_id, "source": source}, path)
        meta.update({"path": str(path), "task": "vision", "params": sum(p.numel() for p in net.parameters())})
    elif source == "huggingface":
        try:
            from transformers import AutoModel

            net = AutoModel.from_pretrained(model_id)
            path = models_dir / f"{model_id.replace('/', '_')}.pt"
            torch.save({"state_dict": net.state_dict(), "model_id": model_id, "source": source}, path)
            meta.update({"path": str(path), "task": "nlp", "params": sum(p.numel() for p in net.parameters())})
        except ImportError:
            cfg_path = models_dir / f"{model_id.replace('/', '_')}.json"
            cfg_path.write_text(json.dumps({"model_id": model_id, "source": source, "note": "Install transformers to load weights"}), encoding="utf-8")
            meta.update({"path": str(cfg_path), "task": "nlp", "note": "Config saved — pip install transformers"})
    else:
        raise SystemExit(f"Unknown source: {source}")

    cfg_path = ml / "config.json"
    cfg = json.loads(cfg_path.read_text(encoding="utf-8")) if cfg_path.exists() else {}
    cfg["model_template"] = model_id
    cfg["model_source"] = source
    cfg_path.write_text(json.dumps(cfg, indent=2), encoding="utf-8")
    (ml / "pretrained.json").write_text(json.dumps(meta, indent=2), encoding="utf-8")
    print(json.dumps({"ok": True, **meta}))


def cmd_compare_runs(args):
    root = Path(args.root)
    runs_dir = root / ".hades" / "ml" / "runs"
    run_ids = [r.strip() for r in args.runs.split(",") if r.strip()]
    series = []
    for rid in run_ids:
        metrics_path = runs_dir / rid / "metrics.json"
        live_path = runs_dir / rid / "live_metrics.json"
        path = metrics_path if metrics_path.exists() else live_path
        if not path.exists():
            continue
        data = json.loads(path.read_text(encoding="utf-8"))
        hist = data.get("history", [])
        series.append({
            "run_id": rid,
            "val_acc": data.get("val_acc") or data.get("best_val_acc"),
            "epochs": [h["epoch"] for h in hist],
            "val_loss": [h["val_loss"] for h in hist],
            "val_acc_curve": [h["val_acc"] for h in hist],
            "train_loss": [h["train_loss"] for h in hist],
        })
    print(json.dumps({"runs": series}))


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
    hpo.add_argument("--mode", choices=["grid", "random", "optuna"], default="random")
    hpo.add_argument("--trials", default="9")

    mg = sub.add_parser("model_graph")
    mg.add_argument("--model", required=True)

    ox = sub.add_parser("export_onnx")
    ox.add_argument("--model", required=True)

    ap = sub.add_parser("augment_preview")
    ap.add_argument("--root", required=True)
    ap.add_argument("--samples", default="4")

    lp = sub.add_parser("load_pretrained")
    lp.add_argument("--root", required=True)
    lp.add_argument("--model-id", required=True)
    lp.add_argument("--source", default="torchvision")

    cr = sub.add_parser("compare_runs")
    cr.add_argument("--root", required=True)
    cr.add_argument("--runs", required=True)

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
        "model_graph": cmd_model_graph,
        "export_onnx": cmd_export_onnx,
        "augment_preview": cmd_augment_preview,
        "load_pretrained": cmd_load_pretrained,
        "compare_runs": cmd_compare_runs,
    }
    cmds[args.cmd](args)


if __name__ == "__main__":
    main()
