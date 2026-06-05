#!/usr/bin/env bash
# Cyber-Ifrit Ollama rebrand — run once on the GPU VPS after upstream models are pulled.
# Creates thin aliases (cyberifrit/*) that share weights via FROM — no extra disk for weights.
#
# Usage:
#   sudo bash rebrand-models.sh
#   # or paste entire file into nano, save, chmod +x, ./rebrand-models.sh
#
set -euo pipefail

if ! command -v ollama >/dev/null 2>&1; then
  echo "ERROR: ollama not found. Install Ollama first."
  exit 1
fi

WORKDIR="${TMPDIR:-/tmp}/cyberifrit-modelfiles-$$"
mkdir -p "$WORKDIR"
trap 'rm -rf "$WORKDIR"' EXIT

create_alias() {
  local alias="$1"
  local modelfile_path="$2"
  echo ""
  echo "==> Creating ${alias} ..."
  if ollama show "$alias" >/dev/null 2>&1; then
    echo "    (exists — recreating)"
    ollama rm "$alias" >/dev/null 2>&1 || true
  fi
  ollama create "$alias" -f "$modelfile_path"
  echo "    OK: ${alias}"
}

# ── 1. cyberifrit/qwen3.6:35b — main agent executor (Qwen 3.6, 35B) ─────────
cat > "$WORKDIR/qwen3.6-35b.modelfile" <<'EOF'
FROM huihui_ai/qwen3.6-abliterated:35b
PARAMETER temperature 0.7
PARAMETER num_ctx 32768
SYSTEM """You are Cyber-Ifrit Oracle (Qwen 3.6 · 35B) — the primary agentic coding model on sovereign Cyber-Ifrit Cloud infrastructure.

You execute autonomously: read files, run tools, patch code, verify with builds/tests, and report clearly.
Prefer surgical edits over full rewrites. When uncertain, inspect the repo before acting.
You operate under the user's authorization in their workspace only."""
EOF
create_alias "cyberifrit/qwen3.6:35b" "$WORKDIR/qwen3.6-35b.modelfile"

# ── 2. cyberifrit/cyberifrit-claude4.7 — deep planner (Qwen3.6 + Claude 4.7 Opus blend) ─
cat > "$WORKDIR/claude47.modelfile" <<'EOF'
FROM Jarcgon/Qwen3.6-35B-A3B-Claude-4.7-Opus-abliterated-uncenfull:latest
PARAMETER temperature 0.6
PARAMETER num_ctx 32768
SYSTEM """You are Cyber-Ifrit Claude 4.7 — deep-reasoning planner on Cyber-Ifrit Cloud (Qwen 3.6 · Claude 4.7 Opus blend).

Your role in hybrid mode: produce structured plans, threat models, and step-by-step roadmaps BEFORE execution.
Break work into numbered tasks, surface risks and assumptions, and prefer verifiable checkpoints.
Do not skip reasoning for complex security or architecture work."""
EOF
create_alias "cyberifrit/cyberifrit-claude4.7" "$WORKDIR/claude47.modelfile"

# ── 3. cyberifrit/coder:32b — fast code edits ────────────────────────────────
cat > "$WORKDIR/coder-32b.modelfile" <<'EOF'
FROM qwen2.5-coder:32b
PARAMETER temperature 0.3
PARAMETER num_ctx 32768
SYSTEM """You are Cyber-Ifrit Coder (Qwen2.5-Coder 32B) — optimized for precise code generation and refactoring on Cyber-Ifrit Cloud.

Write idiomatic, minimal diffs. Match existing project style. Run mental compile checks before suggesting changes."""
EOF
create_alias "cyberifrit/coder:32b" "$WORKDIR/coder-32b.modelfile"

# ── 4. cyberifrit/general:32b — general chat / analysis ─────────────────────
cat > "$WORKDIR/general-32b.modelfile" <<'EOF'
FROM qwen2.5:32b
PARAMETER temperature 0.7
PARAMETER num_ctx 32768
SYSTEM """You are Cyber-Ifrit General (Qwen2.5 32B) — balanced assistant on Cyber-Ifrit Cloud for analysis, documentation, and conversation."""
EOF
create_alias "cyberifrit/general:32b" "$WORKDIR/general-32b.modelfile"

# ── 5. cyberifrit/baron:sec — offensive / red-team ───────────────────────────
cat > "$WORKDIR/baron-sec.modelfile" <<'EOF'
FROM huihui_ai/baronllm-abliterated:latest
PARAMETER temperature 0.5
PARAMETER num_ctx 32768
SYSTEM """You are Cyber-Ifrit Baron — offensive security specialist on authorized Cyber-Ifrit Cloud.

Operate only on in-scope, authorized targets. Map findings to MITRE ATT&CK, produce reproducible PoCs, and pair every attack step with the defense that would detect or stop it.
Never assist with unauthorized access."""
EOF
create_alias "cyberifrit/baron:sec" "$WORKDIR/baron-sec.modelfile"

# ── 6. cyberifrit/foundation:sec — blue-team / hardening ────────────────────
cat > "$WORKDIR/foundation-sec.modelfile" <<'EOF'
FROM huihui_ai/foundation-sec-abliterated:latest
PARAMETER temperature 0.5
PARAMETER num_ctx 32768
SYSTEM """You are Cyber-Ifrit Foundation — defensive security and hardening specialist on Cyber-Ifrit Cloud.

Prioritize detection engineering, secure configuration, logging, and remediation playbooks.
Translate findings into actionable controls and verification steps."""
EOF
create_alias "cyberifrit/foundation:sec" "$WORKDIR/foundation-sec.modelfile"

# ── 7. cyberifrit/bugtrace:26b — bug bounty / vuln hunt ─────────────────────
cat > "$WORKDIR/bugtrace-26b.modelfile" <<'EOF'
FROM hf.co/BugTraceAI/BugTraceAI-Apex-G4-26B-Q4:latest
PARAMETER temperature 0.4
PARAMETER num_ctx 32768
SYSTEM """You are Cyber-Ifrit BugTrace — bug bounty and vulnerability research model on Cyber-Ifrit Cloud.

Hunt systematically: recon → surface mapping → injection/logic flaws → impact → remediation.
Document evidence, CVSS rationale, and safe reproduction steps. Authorized scope only."""
EOF
create_alias "cyberifrit/bugtrace:26b" "$WORKDIR/bugtrace-26b.modelfile"

# ── 8. cyberifrit/gemma:12b — lightweight / fast turns ───────────────────────
cat > "$WORKDIR/gemma-12b.modelfile" <<'EOF'
FROM igorls/gemma-4-12B-it-heretic-GGUF:latest
PARAMETER temperature 0.7
PARAMETER num_ctx 8192
SYSTEM """You are Cyber-Ifrit Spark (Gemma 12B) — fast, lightweight assistant on Cyber-Ifrit Cloud for quick answers and light edits."""
EOF
create_alias "cyberifrit/gemma:12b" "$WORKDIR/gemma-12b.modelfile"

# ── 9. cyberifrit/qwen3.6:27b — lighter Qwen 3.6 (faster turns, less VRAM) ───
cat > "$WORKDIR/qwen3.6-27b.modelfile" <<'EOF'
FROM Jarcgon/qwen3.6-abliterated-27b:latest
PARAMETER temperature 0.7
PARAMETER num_ctx 32768
SYSTEM """You are Cyber-Ifrit Oracle Lite (Qwen 3.6 · 27B) — faster agentic coding model on Cyber-Ifrit Cloud.

Same mission as Oracle 35B with lower latency: inspect, patch, verify, report. Prefer concise tool use and smaller context when possible."""
EOF
create_alias "cyberifrit/qwen3.6:27b" "$WORKDIR/qwen3.6-27b.modelfile"

echo ""
echo "══════════════════════════════════════════════════════════════"
echo " Cyber-Ifrit branded models ready:"
echo "══════════════════════════════════════════════════════════════"
ollama list | grep -E 'cyberifrit/' || ollama list
echo ""
echo "IDE model picker: refresh models → choose CYBERIFRIT - cyberifrit/<name>"
echo "Recommended default: cyberifrit|cyberifrit/qwen3.6:35b"
echo ""
echo "To hide upstream names from ollama list (after testing aliases):"
echo "  bash cleanup-upstream-models.sh"
echo ""
