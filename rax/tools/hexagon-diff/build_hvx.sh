#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mc="${HEX_LLVM_MC:-llvm-mc}"; ld="${HEX_LD:-ld.lld}"; py="${PYTHON:-python3}"
out="$here/oracle_hvx"
for t in "$mc" "$ld" "$py"; do command -v "$t" >/dev/null 2>&1 || { echo "missing $t" >&2; exit 1; }; done
"$py" "$here/gen_oracle_hvx.py" "$here/oracle_hvx.s"
"$mc" -triple=hexagon -mcpu=hexagonv68 -mhvx -filetype=obj "$here/oracle_hvx.s" -o "$here/oracle_hvx.o"
"$ld" -static -T "$here/oracle.ld" -e _start "$here/oracle_hvx.o" -o "$out"
echo "$out"
