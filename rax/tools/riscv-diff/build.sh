#!/usr/bin/env bash
# Build the RISC-V differential-test oracles as static ELFs for qemu-user.
# Builds the RV64GC oracle (always) and, when an RV32 multilib is available, the
# RV32GC oracle too. No-op friendly: prints the RV64 path on success, exits
# non-zero only if the RV64 toolchain is absent.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cc="${RISCV64_CC:-riscv64-linux-gnu-gcc}"
out="$here/oracle"
out32="$here/oracle32"
if ! command -v "$cc" >/dev/null 2>&1; then
    echo "cross compiler '$cc' not found" >&2
    exit 1
fi
"$cc" -static -O2 -march=rv64gc -mabi=lp64d -Wall -Wextra -o "$out" "$here/oracle.c"
# RV32 oracle is best-effort (needs the rv32 multilib / sysroot).
if "$cc" -static -O2 -march=rv32gc -mabi=ilp32d -Wall -Wextra -o "$out32" "$here/oracle.c" 2>/dev/null; then
    :
else
    rm -f "$out32"
fi
echo "$out"
