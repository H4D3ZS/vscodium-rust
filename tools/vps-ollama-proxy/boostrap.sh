#!/usr/bin/env bash
# Typo-friendly alias: delegates to bootstrap.sh (same directory, same .env).

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec bash "${ROOT}/bootstrap.sh" "$@"
