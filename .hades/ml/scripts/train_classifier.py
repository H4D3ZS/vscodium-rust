#!/usr/bin/env python3
"""Train a small tabular classifier from .hades/ml/manifest.json (PyTorch)."""
import json
import subprocess
import sys
import time
from pathlib import Path

import pandas as pd
import torch
import torch.nn as nn
from torch.utils.data import DataLoader, TensorDataset


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


def gpu_mem_mb() -> float | None:
    if not torch.cuda.is_available():
        return None
    return round(torch.cuda.max_memory_allocated() / (1024 * 1024), 1)


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


def write_live(run_dir: Path, payload: dict) -> None:
    (run_dir / "live_metrics.json").write_text(
        json.dumps(payload, indent=2), encoding="utf-8"
    )


def save_checkpoint(run_dir: Path, model, opt, epoch, history, best_val_loss, best_val_acc, best_epoch, stale_epochs):
    torch.save(
        {
            "epoch": epoch,
            "model": model.state_dict(),
            "optimizer": opt.state_dict(),
            "history": history,
            "best_val_loss": best_val_loss,
            "best_val_acc": best_val_acc,
            "best_epoch": best_epoch,
            "stale_epochs": stale_epochs,
        },
        run_dir / "checkpoint.pt",
    )


def main():
    root = Path(sys.argv[1])
    ml = root / ".hades" / "ml"
    manifest_path = ml / "manifest.json"
    cfg_path = ml / "config.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    cfg = json.loads(cfg_path.read_text(encoding="utf-8")) if cfg_path.exists() else {}

    train_csv = root / manifest["train_csv"]
    val_csv = root / manifest["val_csv"]
    target = manifest["target_column"]
    epochs = int(cfg.get("epochs", 20))
    lr = float(cfg.get("learning_rate", 0.001))
    hidden = int(cfg.get("hidden_size", 64))
    patience = int(cfg.get("early_stop_patience", 3))
    resume_from = manifest.get("resume_from") or cfg.get("resume_from")
    run_id = manifest.get("run_id") or resume_from or "latest"
    run_dir = ml / "runs" / run_id
    run_dir.mkdir(parents=True, exist_ok=True)

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    device_name = str(device)
    if torch.cuda.is_available():
        device_name = torch.cuda.get_device_name(0)

    train_df = pd.read_csv(train_csv)
    val_df = pd.read_csv(val_csv)
    feature_cols = [c for c in train_df.columns if c != target]
    if not feature_cols:
        raise SystemExit("No feature columns found")

    x_train = torch.tensor(train_df[feature_cols].values, dtype=torch.float32, device=device)
    y_raw = train_df[target]
    classes = sorted(y_raw.unique().tolist())
    class_to_idx = {c: i for i, c in enumerate(classes)}
    y_train = torch.tensor([class_to_idx[v] for v in y_raw], dtype=torch.long, device=device)
    x_val = torch.tensor(val_df[feature_cols].values, dtype=torch.float32, device=device)
    y_val = torch.tensor([class_to_idx[v] for v in val_df[target]], dtype=torch.long, device=device)

    model = MLP(len(feature_cols), hidden, len(classes)).to(device)
    opt = torch.optim.Adam(model.parameters(), lr=lr)
    loss_fn = nn.CrossEntropyLoss()
    loader = DataLoader(
        TensorDataset(x_train.cpu(), y_train.cpu()),
        batch_size=32,
        shuffle=True,
    )

    history: list[dict] = []
    log_lines: list[str] = []
    best_val_loss = float("inf")
    best_val_acc = 0.0
    best_epoch = 0
    stale_epochs = 0
    early_stopped = False
    start_epoch = 0
    n_train = len(loader.dataset)

    ckpt_path = run_dir / "checkpoint.pt"
    if resume_from and ckpt_path.exists():
        ckpt = torch.load(ckpt_path, map_location=device, weights_only=False)
        model.load_state_dict(ckpt["model"])
        opt.load_state_dict(ckpt["optimizer"])
        history = ckpt.get("history", [])
        best_val_loss = float(ckpt.get("best_val_loss", best_val_loss))
        best_val_acc = float(ckpt.get("best_val_acc", best_val_acc))
        best_epoch = int(ckpt.get("best_epoch", 0))
        stale_epochs = int(ckpt.get("stale_epochs", 0))
        start_epoch = int(ckpt.get("epoch", 0))
        if (run_dir / "model.pt").exists():
            prev = torch.load(run_dir / "model.pt", map_location=device, weights_only=False)
            model.load_state_dict(prev["state_dict"])
        print(f"Resuming from epoch {start_epoch} (run {run_id})", flush=True)

    live = {
        "status": "training",
        "run_id": run_id,
        "total_epochs": epochs,
        "current_epoch": 0,
        "lr": lr,
        "device": device_name,
        "best_val_acc": None,
        "best_val_loss": None,
        "early_stop": False,
        "history": history,
    }
    write_live(run_dir, live)

    for epoch in range(start_epoch, epochs):
        if torch.cuda.is_available():
            torch.cuda.reset_peak_memory_stats()

        t0 = time.perf_counter()
        model.train()
        total = 0.0
        n_batches = 0
        for xb, yb in loader:
            xb, yb = xb.to(device), yb.to(device)
            opt.zero_grad()
            loss = loss_fn(model(xb), yb)
            loss.backward()
            opt.step()
            total += float(loss.item())
            n_batches += 1
        train_loss = total / max(n_batches, 1)

        model.eval()
        with torch.no_grad():
            val_loss = float(loss_fn(model(x_val), y_val).item())
            preds = model(x_val).argmax(dim=1)
            acc = float((preds == y_val).float().mean().item())

        epoch_secs = time.perf_counter() - t0
        samples_per_sec = round(n_train / max(epoch_secs, 1e-6), 1)

        row = {
            "epoch": epoch + 1,
            "train_loss": round(train_loss, 6),
            "val_loss": round(val_loss, 6),
            "val_acc": round(acc, 6),
            "lr": lr,
            "samples_per_sec": samples_per_sec,
            "gpu_mem_mb": gpu_mem_mb(),
            "epoch_secs": round(epoch_secs, 3),
        }
        history.append(row)

        if val_loss < best_val_loss - 1e-6:
            best_val_loss = val_loss
            best_val_acc = acc
            best_epoch = epoch + 1
            stale_epochs = 0
            torch.save(
                {
                    "state_dict": model.state_dict(),
                    "feature_cols": feature_cols,
                    "classes": classes,
                    "hidden_size": hidden,
                },
                run_dir / "model.pt",
            )
            save_checkpoint(
                run_dir, model, opt, epoch + 1, history,
                best_val_loss, best_val_acc, best_epoch, stale_epochs,
            )
        else:
            stale_epochs += 1

        line = (
            f"epoch={epoch+1}/{epochs} train_loss={train_loss:.4f} "
            f"val_loss={val_loss:.4f} val_acc={acc:.4f} it/s={samples_per_sec:.0f}"
        )
        print(line, flush=True)
        print("ML_METRIC:" + json.dumps(row), flush=True)
        log_lines.append(line)

        live.update(
            {
                "current_epoch": epoch + 1,
                "best_val_acc": round(best_val_acc, 6),
                "best_val_loss": round(best_val_loss, 6),
                "best_epoch": best_epoch,
                "stale_epochs": stale_epochs,
                "early_stop_patience": patience,
            }
        )
        write_live(run_dir, live)

        if stale_epochs >= patience:
            early_stopped = True
            print(f"Early stopping at epoch {epoch+1} (no val_loss improvement for {patience} epochs)", flush=True)
            break

    model.eval()
    confusion = {}
    with torch.no_grad():
        preds = model(x_val).argmax(dim=1).cpu().numpy()
        y_true = y_val.cpu().numpy()
    try:
        from sklearn.metrics import confusion_matrix

        cm = confusion_matrix(y_true, preds, labels=list(range(len(classes))))
        confusion = {
            "labels": classes,
            "matrix": cm.tolist(),
        }
    except ImportError:
        confusion = {"labels": classes, "matrix": []}

    gh = git_hash(root)
    experiment = {
        "run_id": run_id,
        "created_at": int(time.time()),
        "git_hash": gh,
        "config": cfg,
        "target_column": target,
        "n_features": len(feature_cols),
        "n_classes": len(classes),
    }
    (run_dir / "experiment.json").write_text(json.dumps(experiment, indent=2), encoding="utf-8")
    (run_dir / "config_snapshot.json").write_text(json.dumps(cfg, indent=2), encoding="utf-8")

    final = {
        "status": "done",
        "run_id": run_id,
        "total_epochs": epochs,
        "epochs_requested": epochs,
        "epochs_run": len(history),
        "early_stopped": early_stopped,
        "best_epoch": best_epoch,
        "val_acc": round(best_val_acc, 6),
        "val_loss": round(best_val_loss, 6),
        "device": device_name,
        "log": log_lines,
        "history": history,
        "confusion_matrix": confusion,
        "git_hash": gh,
    }
    (run_dir / "metrics.json").write_text(json.dumps(final, indent=2), encoding="utf-8")
    live.update({"status": "done", "early_stop": early_stopped, **{k: final[k] for k in ("val_acc", "val_loss", "early_stopped", "best_epoch")}})
    write_live(run_dir, live)
    print(json.dumps({"ok": True, "model": str(run_dir / "model.pt"), "val_acc": best_val_acc}), flush=True)


if __name__ == "__main__":
    main()
