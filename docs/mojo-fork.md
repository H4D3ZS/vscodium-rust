# The Modular / Mojo fork

We maintain a fork of [`modular/modular`](https://github.com/modular/modular)
for:

1. **kortex retrieval kernels** — the f32 re-rank / kNN hot path in libaim's
   search (`tools/mojo-knn/`), later the `.aim` chunk scoring.
2. **a MAX inference path** (exploratory) — MAX ships an OpenAI-compatible
   server (`max/python/max/serve`) and Mojo GPU kernels
   (`max/kernels/src`, with `gfx` AMD targets) that could serve local
   models alongside ROCmFPX.
3. **compiler-level tuning** if we ever need it — e.g. RDNA4 codegen, a
   custom quant intrinsic. Optional and expensive; see below.

The checkout lives at `./modular` (git-ignored, ~1.2 GB). Set it up with
`scripts/setup-modular.sh`; sync with `scripts/setup-modular.sh --sync`.

## What's in the repo (all of it, since 2026-08-18)

Modular open-sourced the **entire** Mojo compiler + toolchain on
2026-08-18, Apache-2.0 with LLVM exceptions. The checkout contains:

| part | path | notes |
|---|---|---|
| Mojo **compiler** | `Mojo/lib/{Compiler,MojoParser,Elaborator,Interpreter,ExecutionEngine}`, `Mojo/lib/*Dialect` (KGEN/HLCF/CO/LIT/POP), `Mojo/tools/{mojo,kgen-*}` | MLIR/LLVM C++. Forkable. Buildable from source with bazel. Internals: `Mojo/docs/compiler/MojoCompilerWalkthrough.md`, `DesignOverview.md`. |
| Mojo **stdlib** | `Mojo/stdlib` | |
| MAX **kernels** | `max/kernels/src` | GPU kernels incl. AMD. Where our `kortex/` kernels go. |
| MAX **serve / pipelines** | `max/python/max/serve`, `.../pipelines` | OpenAI-compatible endpoint + model graphs. |

**Contribution status:** Modular is not taking *upstream* PRs to the
compiler yet (stabilising). Irrelevant to a maintained fork — we don't
push changes back, we rebase ours forward.

## Two toolchain modes

- **pixi prebuilt** (default, fast) — `pixi install` in `modular/` pulls
  the pinned Mojo compiler + MAX from `conda.modular.com/max-nightly`.
  Use this for stdlib and kernel work — no compiler build.
- **bazel from source** — `./bazelw build //Mojo/tools/mojo` builds the
  compiler itself. Needed only when we change compiler code. It's an
  LLVM/MLIR C++ build: slow, disk-hungry (LLVM is fetched by bazel, not
  vendored). Don't reach for this unless a kernel win genuinely needs a
  codegen change.

## Licensing

- **Compiler + stdlib + MAX kernels** — Apache-2.0 with LLVM exceptions.
  Fork, modify, ship freely (attribution preserved).
- **MAX serving runtime** usage & distribution — the separate
  [Modular Community License](https://www.modular.com/legal/community).
  Bundling the MAX *runtime* / serve endpoint into a redistributed IDE
  build has terms (free for most use, not unconditional). Check before we
  ship a MAX-backed inference path.
- Our own Mojo source compiled to a `.so`/`.dylib` and `dlopen`ed by
  libaim is **clean** — Apache source → our binary, no MAX runtime.

## Layout of our changes

Everything ours lives on the `kortex` branch of the fork:

```
modular/max/kernels/src/kortex/
  knn.mojo          # exact top-k cosine (promoted from tools/mojo-knn/)
  BUILD.bazel       # shared-lib target: libkortex_knn.{so,dylib,dll}
```

Compiler patches, if we ever make them, go in **their own commits** on that
branch — they carry real rebase risk when upstream refactors a dialect, so
keep them minimal and documented. Kernel-only changes rebase cleanly ~always.

## Build the kNN kernel

```bash
scripts/setup-modular.sh                    # clone the fork onto branch 'kortex'
cd modular && pixi install                  # pinned compiler + deps (no source build)

pixi run mojo build max/kernels/src/kortex/knn.mojo \
    --emit shared-lib -o ../src-tauri/binaries/kortex/libkortex_knn.so
```

The IDE resolves `libkortex_knn.*` next to the other kortex binaries
(`kortex_bin::find_kortex_tool`) and `dlopen`s it from libaim's `mojo-knn`
feature; absent, libaim uses its Rust kernel. No hard dependency.

## Honest cost

Modular ships **nightly**. Even without touching the compiler, the kernel
library moves. Discipline:

1. **Pin to a release tag**, `--sync` only when a kernel we use changes or
   a MAX release we want lands. (`setup-modular.sh --sync` currently tracks
   `main` — switch it to a tag once one is chosen.)
2. Keep our overlay to **one directory**. Kernel rebases stay contained.
3. Avoid compiler patches unless a bench proves they're worth the rebase tax.

If the kNN win (bench it — `tools/mojo-knn/README.md`) doesn't outweigh the
upkeep, drop the fork and keep libaim's Rust kernel.
