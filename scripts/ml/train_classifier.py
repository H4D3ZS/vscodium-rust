#!/usr/bin/env python3
"""Train a small tabular classifier from .hades/ml/manifest.json (PyTorch)."""
import json
import sys
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
    run_id = manifest.get("run_id", "latest")
    run_dir = ml / "runs" / run_id
    run_dir.mkdir(parents=True, exist_ok=True)

    train_df = pd.read_csv(train_csv)
    val_df = pd.read_csv(val_csv)
    feature_cols = [c for c in train_df.columns if c != target]
    if not feature_cols:
        raise SystemExit("No feature columns found")

    x_train = torch.tensor(train_df[feature_cols].values, dtype=torch.float32)
    y_raw = train_df[target]
    classes = sorted(y_raw.unique().tolist())
    class_to_idx = {c: i for i, c in enumerate(classes)}
    y_train = torch.tensor([class_to_idx[v] for v in y_raw], dtype=torch.long)
    x_val = torch.tensor(val_df[feature_cols].values, dtype=torch.float32)
    y_val = torch.tensor([class_to_idx[v] for v in val_df[target]], dtype=torch.long)

    model = MLP(len(feature_cols), hidden, len(classes))
    opt = torch.optim.Adam(model.parameters(), lr=lr)
    loss_fn = nn.CrossEntropyLoss()
    loader = DataLoader(TensorDataset(x_train, y_train), batch_size=32, shuffle=True)

    log = []
    for epoch in range(epochs):
        model.train()
        total = 0.0
        for xb, yb in loader:
            opt.zero_grad()
            loss = loss_fn(model(xb), yb)
            loss.backward()
            opt.step()
            total += float(loss.item())
        model.eval()
        with torch.no_grad():
            val_loss = float(loss_fn(model(x_val), y_val).item())
            preds = model(x_val).argmax(dim=1)
            acc = float((preds == y_val).float().mean().item())
        line = f"epoch={epoch+1}/{epochs} train_loss={total/len(loader):.4f} val_loss={val_loss:.4f} val_acc={acc:.4f}"
        print(line, flush=True)
        log.append(line)

    ckpt = run_dir / "model.pt"
    torch.save(
        {
            "state_dict": model.state_dict(),
            "feature_cols": feature_cols,
            "classes": classes,
            "hidden_size": hidden,
        },
        ckpt,
    )
    (run_dir / "metrics.json").write_text(
        json.dumps({"epochs": epochs, "val_acc": acc, "log": log}, indent=2),
        encoding="utf-8",
    )
    print(json.dumps({"ok": True, "model": str(ckpt), "val_acc": acc}), flush=True)


if __name__ == "__main__":
    main()
