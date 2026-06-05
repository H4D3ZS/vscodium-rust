#!/usr/bin/env bash
# Remove upstream Ollama tags after cyberifrit/* aliases exist.
# Blobs stay referenced by the aliases — only the ugly names disappear from `ollama list`.
#
# Usage:
#   bash cleanup-upstream-models.sh          # dry-run (print only)
#   bash cleanup-upstream-models.sh --apply  # actually remove upstream tags
#
set -euo pipefail

APPLY=false
[[ "${1:-}" == "--apply" ]] && APPLY=true

# Every upstream tag wrapped by rebrand-models.sh (must match FROM lines there).
UPSTREAM=(
  "huihui_ai/qwen3.6-abliterated:35b"
  "Jarcgon/Qwen3.6-35B-A3B-Claude-4.7-Opus-abliterated-uncenfull:latest"
  "Jarcgon/qwen3.6-abliterated-27b:latest"
  "qwen2.5-coder:32b"
  "qwen2.5:32b"
  "huihui_ai/baronllm-abliterated:latest"
  "huihui_ai/foundation-sec-abliterated:latest"
  "hf.co/BugTraceAI/BugTraceAI-Apex-G4-26B-Q4:latest"
  "igorls/gemma-4-12B-it-heretic-GGUF:latest"
)

REQUIRED_ALIASES=(
  "cyberifrit/qwen3.6:35b"
  "cyberifrit/qwen3.6:27b"
  "cyberifrit/cyberifrit-claude4.7"
  "cyberifrit/coder:32b"
  "cyberifrit/general:32b"
  "cyberifrit/baron:sec"
  "cyberifrit/foundation:sec"
  "cyberifrit/bugtrace:26b"
  "cyberifrit/gemma:12b"
)

if ! command -v ollama >/dev/null 2>&1; then
  echo "ERROR: ollama not found."
  exit 1
fi

echo "Checking cyberifrit aliases exist..."
missing=0
for alias in "${REQUIRED_ALIASES[@]}"; do
  if ollama show "$alias" >/dev/null 2>&1; then
    echo "  OK  $alias"
  else
    echo "  MISSING  $alias  — run rebrand-models.sh first"
    missing=$((missing + 1))
  fi
done

if [[ "$missing" -gt 0 ]]; then
  echo ""
  echo "Abort: $missing alias(es) missing. Run ./rebrand-models.sh before cleanup."
  exit 1
fi

echo ""
if [[ "$APPLY" == false ]]; then
  echo "DRY RUN — upstream tags that would be removed:"
  for name in "${UPSTREAM[@]}"; do
    if ollama show "$name" >/dev/null 2>&1; then
      echo "  ollama rm $name"
    else
      echo "  (skip — not installed) $name"
    fi
  done
  echo ""
  echo "Re-run with:  bash cleanup-upstream-models.sh --apply"
  exit 0
fi

echo "Removing upstream tags..."
for name in "${UPSTREAM[@]}"; do
  if ollama show "$name" >/dev/null 2>&1; then
    echo "  rm $name"
    ollama rm "$name" || echo "    WARN: failed to remove $name"
  fi
done

echo ""
echo "Smoke test: cyberifrit/qwen3.6:35b ..."
if ollama run cyberifrit/qwen3.6:35b "Reply with exactly: OK" --verbose 2>/dev/null | head -5; then
  echo ""
else
  echo "WARN: smoke test failed — upstream removal may have broken an alias."
  echo "Re-pull upstream and re-run rebrand-models.sh if needed."
fi

echo ""
echo "Remaining catalog (should be cyberifrit/* only):"
ollama list
