#!/usr/bin/env bash
# Mac dev smoke check — Xcode sim helpers, Rust, and frontend typecheck.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "==> Xcode / simctl"
DEV_DIR="$(xcode-select -p 2>/dev/null || true)"
if [[ -z "$DEV_DIR" ]]; then
  echo "FAIL: xcode-select not configured. Run: sudo xcode-select -s /Applications/Xcode.app"
  exit 1
fi
echo "    developer_dir=$DEV_DIR"
xcrun -find simctl >/dev/null
echo "    simctl OK"

echo "==> npm typecheck"
npm run typecheck

echo "==> cargo check (src-tauri)"
(cd src-tauri && cargo check)

echo "==> iOS sim helper compile (swiftc/clang)"
CACHE="${HOME}/Library/Caches/com.hades.vscode-rust-app/ios-simulator"
mkdir -p "$CACHE"
HELPERS="$ROOT/tools/ios-simulator/helpers"
DEV_DIR="$DEV_DIR"

swiftc -O \
  -F /Library/Developer/PrivateFrameworks \
  -F "${DEV_DIR}/Platforms/iPhoneSimulator.platform/Developer/Library/PrivateFrameworks" \
  -framework CoreImage -framework Foundation -framework IOSurface \
  "$HELPERS/sim-capture.swift" -o "$CACHE/sim-capture"

clang -fobjc-arc -O2 -framework Foundation -framework CoreGraphics \
  "$HELPERS/sim-input.m" -o "$CACHE/sim-input"

echo "    sim-capture + sim-input built in $CACHE"

echo "==> libsim_host.dylib"
NATIVE="$ROOT/tools/ios-simulator/native"
BRIDGE_O="$CACHE/ns_window_bridge.o"
clang -fobjc-arc -c "$NATIVE/ns_window_bridge.m" -o "$BRIDGE_O"
swiftc -O -emit-library -module-name SimHost \
  -F /Library/Developer/PrivateFrameworks \
  -F "${DEV_DIR}/Platforms/iPhoneSimulator.platform/Developer/Library/PrivateFrameworks" \
  -framework AppKit -framework QuartzCore -framework IOSurface -framework Foundation \
  "$NATIVE/sim_host.swift" "$BRIDGE_O" -o "$CACHE/libsim_host.dylib"
echo "    $CACHE/libsim_host.dylib"

echo "==> cargo build (src-tauri)"
export CARGO_TARGET_DIR="$ROOT/src-tauri/target"
(cd src-tauri && cargo build -q)
DYLIB="$CARGO_TARGET_DIR/debug/libsim_host.dylib"
if [[ -f "$DYLIB" ]]; then
  echo "    linked $DYLIB"
else
  echo "WARN: libsim_host.dylib not in target dir — build.rs may have skipped swiftc"
fi

echo "OK — mac_dev stack verified"
