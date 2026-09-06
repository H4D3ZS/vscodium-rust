# The Modular / Mojo fork

We maintain a fork of the **open** parts of
[`modular/modular`](https://github.com/modular/modular) for two things:

1. **kortex retrieval kernels** — the f32 re-rank / kNN hot path in libaim's
   search (see `tools/mojo-knn/`), and later the `.aim` chunk scoring.
2. **a MAX inference path** (exploratory) — MAX ships an OpenAI-compatible
   server (`max/python/max/serve`) and Mojo GPU kernels
   (`max/kernels/src`) that could serve local models alongside ROCmFPX.

The checkout lives at `./modular` (git-ignored, ~1.2 GB). Set it up with
`scripts/setup-modular.sh`; sync with `scripts/setup-modular.sh --sync`.

## What we can and cannot fork

| part | path in the repo | can we modify it? |
|---|---|---|
| Mojo **compiler** | *not in the repo* | **No.** Not open-sourced; ships prebuilt via pixi. We are pinned to Modular's releases. |
| Mojo **stdlib** | `Mojo/stdlib` | Yes — Modular takes stdlib PRs. |
| MAX **kernels** | `max/kernels/src` | Yes — Modular takes kernel PRs. Where our `kortex/` kernels go. |
| MAX **serve / pipelines** | `max/python/max/serve`, `.../pipelines` | Yes. |

So "our fork" is really: **the open code, on a `kortex` branch, rebased onto
upstream `main`.** The compiler underneath is always Modular's.

## Licensing — read before shipping anything

- The **code** in the repo is Apache-2.0 with LLVM exceptions — fork and
  modify freely, attribution preserved.
- **MAX usage and distribution** are under the
  [Modular Community License](https://www.modular.com/legal/community), a
  *separate* license from the code. Bundling MAX binaries (the runtime, the
  serve endpoint) into the IDE for redistribution has terms — free for
  most use, but not unconditional. Check it before we ship a MAX-backed
  build. The Mojo-compiled `.so`/`.dylib` kernels we build ourselves from
  Apache-licensed source are not encumbered by that; the MAX *runtime* is.
- Practically: the **kNN kernel path is clean** (our Mojo source →
  our shared lib, `dlopen`ed by libaim). A **MAX inference path** needs a
  license review first.

## Layout of our changes

Everything kortex-specific goes on the `kortex` branch of the fork, under:

```
modular/max/kernels/src/kortex/
  knn.mojo          # exact top-k cosine (from tools/mojo-knn/, promoted)
  BUILD.bazel       # a shared-lib target: libkortex_knn.{so,dylib,dll}
```

Nothing else in the fork is touched, so `--sync` is a clean rebase 99% of
the time. When Modular moves a kernel API out from under us, the rebase
conflicts land in exactly one directory.

## Build

```bash
scripts/setup-modular.sh          # clone the fork onto branch 'kortex'
cd modular && pixi install        # pinned Mojo compiler + deps

# build the kNN kernel as a shared lib:
pixi run mojo build max/kernels/src/kortex/knn.mojo \
    --emit shared-lib -o ../src-tauri/binaries/kortex/libkortex_knn.so
```

The IDE resolves `libkortex_knn.*` next to the other kortex binaries
(`kortex_bin::find_kortex_tool`) and `dlopen`s it from libaim's
`mojo-knn` feature; absent, libaim uses its Rust kernel. No hard dependency.

## Honest cost

Modular ships **dev builds daily** and refactors the kernel library often.
Tracking `main` closely is real work. Mitigations, in order of preference:

1. **Pin to a release tag**, sync only when a kernel we use changes or a
   MAX release we want lands. `setup-modular.sh --sync` fetches `main`;
   change it to a tag once one is picked.
2. Keep our overlay to **one directory** (above) so rebases are contained.
3. The compiler pin means a MAX/Mojo bump is a lockfile change +
   `pixi install`, not a source merge — that part is cheap.

If maintenance outweighs the kNN win (bench it — `tools/mojo-knn/README.md`),
drop the fork and keep libaim's Rust kernel.
