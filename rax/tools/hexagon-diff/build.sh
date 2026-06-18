#!/usr/bin/env bash
# Build the Hexagon differential-test oracle as a static ELF for qemu-hexagon.
# Prints the oracle path on success; exits non-zero if the toolchain is absent.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mc="${HEX_LLVM_MC:-llvm-mc}"
ld="${HEX_LD:-ld.lld}"
py="${PYTHON:-python3}"
out="$here/oracle"

for t in "$mc" "$ld" "$py"; do
    if ! command -v "$t" >/dev/null 2>&1; then
        echo "required tool '$t' not found" >&2
        exit 1
    fi
done

"$py" "$here/gen_oracle.py" "$here/oracle.s"
"$mc" -triple=hexagon -filetype=obj "$here/oracle.s" -o "$here/oracle.o"
"$ld" -static -T "$here/oracle.ld" -e _start "$here/oracle.o" -o "$out"
echo "$out"
