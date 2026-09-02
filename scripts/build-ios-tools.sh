#!/usr/bin/env bash
# Build the vendored iOS-over-USB tools from source and stage them into
# src-tauri/binaries/ios-tools/ so `npx tauri build` bundles them.
#
# Cross-platform (Linux / macOS). Run once per target platform on that platform
# (or in that platform's CI). Windows uses scripts/build-ios-tools.ps1 instead.
#
#   ./scripts/build-ios-tools.sh            # build everything that can be built
#   ./scripts/build-ios-tools.sh go-ios     # build just one
#
# Sources live as git submodules under third_party/ios-tools/. Run
#   git submodule update --init --recursive third_party/ios-tools/*
# first if they aren't checked out.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$ROOT/third_party/ios-tools"
OUT="$ROOT/src-tauri/binaries/ios-tools"
mkdir -p "$OUT"

log() { printf '\033[1;36m[ios-tools]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[ios-tools]\033[0m %s\n' "$*" >&2; }

build_go_ios() {
  if [ ! -d "$SRC/go-ios" ]; then warn "go-ios submodule missing — skipping"; return; fi
  command -v go >/dev/null || { warn "Go toolchain not found — skipping go-ios"; return; }
  log "building go-ios…"
  ( cd "$SRC/go-ios" && go build -trimpath -ldflags "-s -w" -o "$OUT/ios" . )
  log "→ $OUT/ios"
}

build_zsign() {
  if [ ! -d "$SRC/zsign" ]; then warn "zsign submodule missing — skipping"; return; fi
  if command -v cmake >/dev/null; then
    log "building zsign (cmake)…"
    cmake -S "$SRC/zsign" -B "$SRC/zsign/build" -DCMAKE_BUILD_TYPE=Release >/dev/null
    cmake --build "$SRC/zsign/build" --config Release
    # zsign's binary name/location varies by build; grab the first match.
    local bin
    bin="$(find "$SRC/zsign/build" -maxdepth 3 -type f -name 'zsign' | head -n1 || true)"
    [ -n "$bin" ] && cp "$bin" "$OUT/zsign" && log "→ $OUT/zsign" || warn "zsign binary not found after build"
  elif command -v g++ >/dev/null; then
    log "building zsign (g++)…"
    ( cd "$SRC/zsign" && g++ -O2 -std=c++11 *.cpp common/*.cpp -o "$OUT/zsign" -lcrypto ) \
      && log "→ $OUT/zsign" || warn "zsign g++ build failed (needs OpenSSL dev headers)"
  else
    warn "no cmake/g++ — skipping zsign"
  fi
}

build_idevice_tools() {
  # ideviceiproxy (USB hot-reload tunnel) comes from libimobiledevice, whose
  # autotools build needs the whole family (libplist, libusbmuxd,
  # libimobiledevice-glue). On Linux the distro package is far easier:
  #   Debian/Ubuntu: sudo apt install libimobiledevice-utils
  #   Fedora:        sudo dnf install libimobiledevice-utils
  #   Arch:          sudo pacman -S libimobiledevice
  # If ideviceiproxy is already on PATH (from a package), just stage it.
  if command -v ideviceiproxy >/dev/null; then
    cp "$(command -v ideviceiproxy)" "$OUT/ideviceiproxy"
    log "→ $OUT/ideviceiproxy (from system package)"
    return
  fi
  if [ -d "$SRC/libimobiledevice" ] && command -v make >/dev/null; then
    warn "building libimobiledevice from source needs libplist/libusbmuxd/glue first;"
    warn "prefer the distro package (see comments above). Skipping source build."
  else
    warn "ideviceiproxy not found — install libimobiledevice-utils via your package manager"
  fi
}

TARGET="${1:-all}"
case "$TARGET" in
  go-ios)  build_go_ios ;;
  zsign)   build_zsign ;;
  idevice) build_idevice_tools ;;
  all)     build_go_ios; build_zsign; build_idevice_tools ;;
  *) echo "usage: $0 [go-ios|zsign|idevice|all]"; exit 1 ;;
esac

log "staged binaries:"
ls -la "$OUT" | grep -vE '^total|README' || true
