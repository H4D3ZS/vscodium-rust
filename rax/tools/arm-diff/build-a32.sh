#!/usr/bin/env bash
# Build the AArch32 (A32/Thumb) differential-test oracle as a static ELF for
# qemu-arm. Prints the path on success, exits non-zero if the toolchain absent.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cc="${ARM32_CC:-arm-linux-gnueabihf-gcc}"
out="$here/oracle-a32"
if ! command -v "$cc" >/dev/null 2>&1; then
    echo "cross compiler '$cc' not found" >&2
    exit 1
fi
"$cc" -static -O2 -marm -march=armv7-a -mfpu=neon-vfpv4 \
    -Wall -Wextra -o "$out" "$here/oracle-a32.c"
echo "$out"
