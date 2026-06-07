#!/usr/bin/env python3
"""Train a vision classifier from .hades/ml/data/images/<class>/ folders."""
import json
import subprocess
import sys
import time
from pathlib import Path

import torch
import torch.nn as nn
from torch.utils.data import DataLoader, random_split
from torchvision import datasets, models, transforms


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
    (run_dir / "live_metrics.json").write_text(json.dumps(payload, indent=2), encoding="utf-8")


def save_checkpoint(run_dir, model, opt, epoch, history, best_val_loss, best_val_acc, best_epoch, stale_epochs):
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
            "task": "vision",
        },
        run_dir / "checkpoint.pt",
    )


def main():
    root = Path(sys.argv[1])
    ml = root / ".hades" / "ml"
    manifest = json.loads((ml / "manifest.json").read_text(encoding="utf-8"))
    cfg = json.loads((ml / "config.json").read_text(encoding="utf-8")) if (ml / "config.json").exists() else {}

    image_root = ml / "data" / "images"
    if not image_root.is_dir():
        raise SystemExit("No image dataset — create .hades/ml/data/images/<class>/ folders")

    epochs = int(cfg.get("epochs", 10))
    lr = float(cfg.get("learning_rate", 0.001))
    patience = int(cfg.get("early_stop_patience", 3))
    template = cfg.get("model_template", "resnet18")
    resume_from = manifest.get("resume_from") or cfg.get("resume_from")
    run_id = manifest.get("run_id") or resume_from or "latest"
    run_dir = ml / "runs" / run_id
    run_dir.mkdir(parents=True, exist_ok=True)

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    transform = transforms.Compose([
        transforms.Resize((224, 224)),
        transforms.RandomHorizontalFlip(),
        transforms.ToTensor(),
        transforms.Normalize([0.485, 0.456, 0.406], [0.229, 0.224, 0.225]),
    ])
    full = datasets.ImageFolder(str(image_root), transform=transform)
    if len(full) < 4:
        raise SystemExit("Need at least 4 images across class folders")
    n_val = max(1, int(len(full) * float(cfg.get("val_ratio", 0.2))))
    n_train = len(full) - n_val
    train_ds, val_ds = random_split(full, [n_train, n_val], generator=torch.Generator().manual_seed(42))
    train_loader = DataLoader(train_ds, batch_size=16, shuffle=True, num_workers=0)
    val_loader = DataLoader(val_ds, batch_size=16, shuffle=False, num_workers=0)
    classes = full.classes

    factory = getattr(models, template, models.resnet18)
    try:
        model = factory(weights="DEFAULT")
    except Exception:
        model = factory(pretrained=True)
    in_features = model.fc.in_features if hasattr(model, "fc") else model.classifier[-1].in_features
    if hasattr(model, "fc"):
        model.fc = nn.Linear(in_features, len(classes))
    else:
        model.classifier[-1] = nn.Linear(in_features, len(classes))
    model = model.to(device)

    opt = torch.optim.Adam(model.parameters(), lr=lr)
    loss_fn = nn.CrossEntropyLoss()
    history = []
    best_val_loss = float("inf")
    best_val_acc = 0.0
    best_epoch = 0
    stale_epochs = 0
    start_epoch = 0

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

    live = {"status": "training", "run_id": run_id, "total_epochs": epochs, "history": history, "task": "vision"}
    write_live(run_dir, live)

    for epoch in range(start_epoch, epochs):
        model.train()
        total = 0.0
        n_batches = 0
        for xb, yb in train_loader:
            xb, yb = xb.to(device), yb.to(device)
            opt.zero_grad()
            loss = loss_fn(model(xb), yb)
            loss.backward()
            opt.step()
            total += float(loss.item())
            n_batches += 1
        train_loss = total / max(n_batches, 1)

        model.eval()
        correct = 0
        val_total = 0
        val_loss_sum = 0.0
        with torch.no_grad():
            for xb, yb in val_loader:
                xb, yb = xb.to(device), yb.to(device)
                logits = model(xb)
                val_loss_sum += float(loss_fn(logits, yb).item()) * xb.size(0)
                correct += int((logits.argmax(1) == yb).sum().item())
                val_total += xb.size(0)
        val_loss = val_loss_sum / max(val_total, 1)
        acc = correct / max(val_total, 1)

        row = {
            "epoch": epoch + 1,
            "train_loss": round(train_loss, 6),
            "val_loss": round(val_loss, 6),
            "val_acc": round(acc, 6),
            "lr": lr,
            "samples_per_sec": 0,
            "epoch_secs": 0,
        }
        history.append(row)
        print("ML_METRIC:" + json.dumps(row), flush=True)

        if val_loss < best_val_loss - 1e-6:
            best_val_loss = val_loss
            best_val_acc = acc
            best_epoch = epoch + 1
            stale_epochs = 0
            torch.save(
                {
                    "state_dict": model.state_dict(),
                    "classes": classes,
                    "model_template": template,
                    "task": "vision",
                },
                run_dir / "model.pt",
            )
            save_checkpoint(run_dir, model, opt, epoch + 1, history, best_val_loss, best_val_acc, best_epoch, stale_epochs)
        else:
            stale_epochs += 1

        live.update({"current_epoch": epoch + 1, "history": history})
        write_live(run_dir, live)
        if stale_epochs >= patience:
            break

    final = {
        "status": "done",
        "run_id": run_id,
        "total_epochs": epochs,
        "val_acc": round(best_val_acc, 6),
        "val_loss": round(best_val_loss, 6),
        "history": history,
        "task": "vision",
        "classes": classes,
        "git_hash": git_hash(root),
    }
    (run_dir / "metrics.json").write_text(json.dumps(final, indent=2), encoding="utf-8")
    write_live(run_dir, {**live, "status": "done"})
    print(json.dumps({"ok": True, "val_acc": best_val_acc}), flush=True)


if __name__ == "__main__":
    main()
