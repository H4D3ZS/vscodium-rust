#!/usr/bin/env bash
# Vendor the iOS-over-USB tool sources as git submodules under
# third_party/ios-tools/. Run once (on a normal connection); commits the
# submodule pointers so everyone gets the same sources.
#
#   ./scripts/vendor-ios-tools.sh
#
# After this, build & stage the binaries with:
#   ./scripts/build-ios-tools.sh      (Linux/macOS)
#   pwsh ./scripts/build-ios-tools.ps1 (Windows)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

add() {
  local url="$1" path="$2"
  if [ -e "$path/.git" ] || git config -f .gitmodules --get "submodule.$path.url" >/dev/null 2>&1; then
    echo "[vendor] $path already present — skipping"
    return
  fi
  echo "[vendor] adding $path"
  # --depth 1: these repos have heavy history; we only need a buildable tree.
  git -c protocol.version=2 submodule add --depth 1 "$url" "$path"
}

# Core: mirroring + on-device install/launch. (Go — trivial cross-platform build.)
add https://github.com/danielpaulus/go-ios.git third_party/ios-tools/go-ios
# Signing for the deploy path. (C++ / cmake + OpenSSL.)
add https://github.com/zhlynn/zsign.git third_party/ios-tools/zsign
# ideviceiproxy (USB hot-reload tunnel). Optional to build from source — on
# Linux the distro package is easier (see binaries/ios-tools/README.md). Vendor
# the source anyway so it can be patched if needed.
add https://github.com/libimobiledevice/libimobiledevice.git third_party/ios-tools/libimobiledevice

echo
echo "[vendor] done. Next: run scripts/build-ios-tools.{sh,ps1} to stage binaries,"
echo "         then commit the submodule pointers."
