#!/usr/bin/env bash
# Build the control-flow / hardware-loop Hexagon differential oracle (oracle_cf).
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mc="${HEX_LLVM_MC:-llvm-mc}"; ld="${HEX_LD:-ld.lld}"; py="${PYTHON:-python3}"
out="$here/oracle_cf"
for t in "$mc" "$ld" "$py"; do command -v "$t" >/dev/null 2>&1 || { echo "missing $t" >&2; exit 1; }; done
"$py" "$here/gen_oracle_cf.py" "$here/oracle_cf.s"
"$mc" -triple=hexagon -filetype=obj "$here/oracle_cf.s" -o "$here/oracle_cf.o"
"$ld" -static -T "$here/oracle.ld" -e _start "$here/oracle_cf.o" -o "$out"
echo "$out"
