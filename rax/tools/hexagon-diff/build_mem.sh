#!/usr/bin/env bash
# Build the memory-capable Hexagon differential oracle (oracle_mem).
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mc="${HEX_LLVM_MC:-llvm-mc}"; ld="${HEX_LD:-ld.lld}"; py="${PYTHON:-python3}"
out="$here/oracle_mem"
for t in "$mc" "$ld" "$py"; do command -v "$t" >/dev/null 2>&1 || { echo "missing $t" >&2; exit 1; }; done
"$py" "$here/gen_oracle_mem.py" "$here/oracle_mem.s"
"$mc" -triple=hexagon -filetype=obj "$here/oracle_mem.s" -o "$here/oracle_mem.o"
"$ld" -static -T "$here/oracle.ld" -e _start "$here/oracle_mem.o" -o "$out"
echo "$out"
