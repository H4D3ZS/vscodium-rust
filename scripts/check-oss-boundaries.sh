#!/usr/bin/env bash
# Fail if SaaS billing markers appear in the community client surface.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

fail=0
check() {
  local label="$1" pattern="$2" path="$3"
  if rg -n "$pattern" "$path" --glob '!scripts/check-oss-boundaries.sh' 2>/dev/null; then
    echo "FAIL: $label matched in $path"
    fail=1
  fi
}

# Hosted billing must not ship in the open client commands/UI.
check "PayMongo in Rust client" 'PayMongo|paymongo' 'src-tauri/src/account.rs'
check "PayMongo in account UI" 'PayMongo|paymongo|QR Ph checkout' 'src/components/AccountSettingsPanel.tsx'
check "Supabase subscription sync" 'sync_from_supabase|/api/start-trial|usage_counters' 'src-tauri/src/account.rs'
check "IDE billing checkout path" '/pay\\?kind=subscription' 'src-tauri/src/account.rs'

if [[ "$fail" -ne 0 ]]; then
  echo ""
  echo "OSS boundary check failed — remove SaaS billing from the community build."
  exit 1
fi

echo "OSS boundary check passed."
