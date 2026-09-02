#!/usr/bin/env bash
# Air-gap quickstart for HADES IDE — local Ollama + optional AIM proxy + offline models.
set -euo pipefail

echo "=== HADES IDE air-gap stack ==="

if ! command -v ollama >/dev/null 2>&1; then
  echo "Install Ollama first: https://ollama.com"
  exit 1
fi

if ! curl -sf http://127.0.0.1:11434/api/tags >/dev/null 2>&1; then
  echo "Starting Ollama…"
  ollama serve >/tmp/ollama-serve.log 2>&1 &
  sleep 2
fi

MODELS="${AIRGAP_MODELS:-qwen3.5:2b qwen3.5:7b}"
for m in $MODELS; do
  echo "Pulling $m (skip if offline bundle already has it)…"
  ollama pull "$m" || true
done

KORTEX_ROOT="$(cd "$(dirname "$0")/../kortex" && pwd)"
AIM_PROXY="$KORTEX_ROOT/target/release/aim-proxy"
if [[ -x "$AIM_PROXY" ]]; then
  if ! curl -sf http://127.0.0.1:1536/api/tags >/dev/null 2>&1; then
    echo "Starting AIM proxy on :1536…"
  "$AIM_PROXY" >/tmp/aim-proxy.log 2>&1 &
  fi
else
  echo "AIM proxy not built — run: cd kortex && cargo build --release --bin aim-proxy"
fi

mkdir -p "$HOME/.kortex/kvcache/index" "$HOME/.kortex/kvcache/slots"
echo ""
echo "Air-gap stack ready."
echo "  Ollama:     http://127.0.0.1:11434"
echo "  AIM proxy:  http://127.0.0.1:1536 (if built)"
echo "  KV cache:   $HOME/.kortex/kvcache"
echo ""
echo "In IDE: Settings → Enterprise → Seed cyber defaults"
echo "        Settings → Inference → Local Ollama"
echo "        Agent mode → Bug Bounty"
