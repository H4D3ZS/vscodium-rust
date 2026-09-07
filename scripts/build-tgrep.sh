#!/usr/bin/env bash
# Build tgrep (our fork, microsoft/tgrep, MIT) from source on macOS/Linux —
# the .sh counterpart to build-tgrep.ps1 (Windows). Same output path
# (src-tauri/bundles/tgrep/tgrep), same reasoning: build from the vendored
# submodule instead of redistributing Microsoft's release binary, and it's
# where "optimize tgrep to work hand in hand with kortex" lands as real
# commits on the fork.
#
# Usage (from repo root or anywhere):
#   scripts/build-tgrep.sh
#   CLEAN=1 scripts/build-tgrep.sh

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
src="$repo_root/tgrep"
stage_dir="$repo_root/src-tauri/bundles/tgrep"

if [[ ! -f "$src/Cargo.toml" ]]; then
  echo "Error: tgrep submodule not found at $src." >&2
  echo "Run: git -C \"$repo_root\" submodule update --init tgrep" >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo not found on PATH. Install Rust: https://rustup.rs" >&2
  exit 1
fi

pushd "$src" >/dev/null
if [[ "${CLEAN:-}" == "1" ]]; then
  echo "==> cargo clean"
  cargo clean
fi
echo "==> Building tgrep-cli --release from source ($src)"
cargo build --release -p tgrep-cli
popd >/dev/null

built_exe="$src/target/release/tgrep"
if [[ ! -f "$built_exe" ]]; then
  echo "Error: build succeeded but $built_exe is missing — unexpected output layout." >&2
  exit 1
fi

mkdir -p "$stage_dir"
dest_exe="$stage_dir/tgrep"
cp -f "$built_exe" "$dest_exe"
chmod +x "$dest_exe"

echo "==> OK — $dest_exe"
echo "==> Version: $("$dest_exe" --version)"
