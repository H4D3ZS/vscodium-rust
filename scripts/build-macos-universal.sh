#!/usr/bin/env bash
# Build a universal macOS .app + .dmg (Apple Silicon + Intel).
set -euo pipefail

export MACOSX_DEPLOYMENT_TARGET="${MACOSX_DEPLOYMENT_TARGET:-11.0}"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Error: macOS universal build must run on macOS (Darwin)." >&2
  exit 1
fi

echo "==> Ensuring Rust targets for universal-apple-darwin"
rustup target add aarch64-apple-darwin x86_64-apple-darwin

echo "==> Frontend + sidecar prebuild"
npm run prebuild:sidecar
npm run build

echo "==> Tauri universal build (aarch64 + x86_64)"
npx tauri build -- --target universal-apple-darwin

echo "==> Done. Artifacts under src-tauri/target/universal-apple-darwin/release/bundle/"
