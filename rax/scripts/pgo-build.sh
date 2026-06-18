#!/usr/bin/env bash
# Profile-guided optimization (PGO) build for rax.
#
# Instruments the build, trains on representative interpreter workloads (the
# register- and memory-bound bench loops + the microkernel), merges the profile,
# and rebuilds with the profile applied. The giant opcode-dispatch matches are an
# ideal PGO target: ~+20% interpreter throughput over a plain release build.
#
# Output: target/release/rax (PGO-optimized, target-cpu=native by default).
# Override the ISA for a portable build:  PGO_TARGET_CPU=x86-64-v3 make pgo
set -euo pipefail
cd "$(dirname "$0")/.."

PROF="${PGO_PROFILE_DIR:-/tmp/rax-pgo-data}"
TARGET_CPU="${PGO_TARGET_CPU:-native}"

# Locate llvm-profdata: PATH first, then the rustup llvm-tools component.
PROFDATA="$(command -v llvm-profdata || true)"
if [ -z "$PROFDATA" ]; then
  PROFDATA="$(find "$(rustc --print sysroot)" -name 'llvm-profdata*' 2>/dev/null | head -1)"
fi
if [ -z "$PROFDATA" ]; then
  echo "error: llvm-profdata not found. Install with:" >&2
  echo "       rustup component add llvm-tools-preview" >&2
  exit 1
fi

rm -rf "$PROF" "$PROF.profdata"

echo "[pgo] 1/4 instrumented build (target-cpu=$TARGET_CPU)"
RUSTFLAGS="-Cprofile-generate=$PROF -C target-cpu=$TARGET_CPU" \
  cargo build --release --examples

echo "[pgo] 2/4 training run (representative workloads)"
./target/release/examples/bench_loop 0x2000000 >/dev/null 2>&1 || true
./target/release/examples/bench_mem 0x1000000 >/dev/null 2>&1 || true
./target/release/examples/run_microkernel >/dev/null 2>&1 || true

echo "[pgo] 3/4 merging profile data"
"$PROFDATA" merge -o "$PROF.profdata" "$PROF"

echo "[pgo] 4/4 optimized rebuild"
RUSTFLAGS="-Cprofile-use=$PROF.profdata -Cllvm-args=-pgo-warn-missing-function=0 -C target-cpu=$TARGET_CPU" \
  cargo build --release

echo "[pgo] done -> target/release/rax  (PGO, target-cpu=$TARGET_CPU)"
