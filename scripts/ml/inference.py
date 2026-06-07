#!/usr/bin/env python3
"""Run inference with a trained .hades/ml model checkpoint."""
import json
import sys
from pathlib import Path

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


def main():
    model_path = Path(sys.argv[1])
    payload = json.loads(sys.argv[2])
    ckpt = torch.load(model_path, map_location="cpu", weights_only=False)
    feature_cols = ckpt["feature_cols"]
    classes = ckpt["classes"]
    hidden = int(ckpt.get("hidden_size", 64))
    values = [float(payload.get(c, 0.0)) for c in feature_cols]
    x = torch.tensor([values], dtype=torch.float32)
    model = MLP(len(feature_cols), hidden, len(classes))
    model.load_state_dict(ckpt["state_dict"])
    model.eval()
    with torch.no_grad():
        probs = torch.softmax(model(x), dim=1)[0]
        idx = int(probs.argmax().item())
    out = {
        "prediction": classes[idx],
        "confidence": float(probs[idx].item()),
        "probabilities": {classes[i]: float(probs[i].item()) for i in range(len(classes))},
    }
    print(json.dumps(out))


if __name__ == "__main__":
    main()
