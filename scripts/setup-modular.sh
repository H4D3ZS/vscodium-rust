#!/usr/bin/env bash
# Check out the Modular/Mojo fork under ./modular (git-ignored, ~1.2 GB).
#
# We maintain a fork of the OPEN parts of modular/modular — the Mojo stdlib,
# the MAX accelerator kernels, and the MAX serve/pipelines — for the kortex
# retrieval kernels and (later) a MAX inference path. The Mojo *compiler* is
# not in that repo and is not forkable; it comes prebuilt via pixi. See
# docs/mojo-fork.md for the licensing constraints and the upstream-sync flow.
#
#   scripts/setup-modular.sh                 # clone the fork (or upstream)
#   scripts/setup-modular.sh --sync          # fetch upstream, rebase our overlay
#
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIR="$ROOT/modular"

# Set this to your fork once created:  gh repo fork modular/modular --clone=false
FORK_URL="${MODULAR_FORK_URL:-https://github.com/H4D3ZS/modular.git}"
UPSTREAM_URL="https://github.com/modular/modular.git"

if [[ "${1:-}" == "--sync" ]]; then
  [[ -d "$DIR/.git" ]] || { echo "no checkout at $DIR — run without --sync first" >&2; exit 1; }
  git -C "$DIR" remote get-url upstream >/dev/null 2>&1 || git -C "$DIR" remote add upstream "$UPSTREAM_URL"
  echo ">> fetching upstream"
  git -C "$DIR" fetch --depth=1 upstream main
  echo ">> our overlay lives on branch 'kortex'; rebase it onto upstream/main"
  git -C "$DIR" checkout kortex 2>/dev/null || git -C "$DIR" checkout -b kortex
  git -C "$DIR" rebase upstream/main || {
    echo "!! rebase hit conflicts — resolve in $DIR, then: git rebase --continue" >&2
    exit 1
  }
  echo ">> synced. Pixi/bazel lockfiles moved — you may need: cd modular && pixi install"
  exit 0
fi

if [[ -d "$DIR/.git" ]]; then
  echo "modular/ already checked out. Use --sync to update."
  exit 0
fi

echo ">> shallow-cloning $FORK_URL -> $DIR"
if ! git clone --depth=1 --filter=blob:none "$FORK_URL" "$DIR" 2>/dev/null; then
  echo "!! fork clone failed — falling back to upstream (create the fork with:" >&2
  echo "     gh repo fork modular/modular --clone=false" >&2
  echo "   then re-run, or set MODULAR_FORK_URL)" >&2
  git clone --depth=1 --filter=blob:none "$UPSTREAM_URL" "$DIR"
fi

git -C "$DIR" checkout -b kortex 2>/dev/null || true
cat <<EOF

modular/ ready ($(du -sh "$DIR" 2>/dev/null | cut -f1)).

Next:
  cd modular
  # install the pinned toolchain (Mojo compiler + deps) — needs pixi:
  curl -fsSL https://pixi.sh/install.sh | bash   # if you don't have pixi
  pixi install

  # build the kortex kNN kernel against this tree (once it lands here):
  cd Mojo && pixi run mojo build ../../tools/mojo-knn/knn.mojo -o ../../tools/mojo-knn/knn

Kortex-specific kernels go in this fork under max/kernels/src/kortex/ on the
'kortex' branch, so `scripts/setup-modular.sh --sync` can rebase them.
EOF
