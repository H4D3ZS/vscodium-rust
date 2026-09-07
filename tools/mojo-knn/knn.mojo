# mojo-knn — exact brute-force top-k cosine over unit vectors.
#
# Prototype for the kortex retrieval re-rank hot path (see README.md). Target
# Mojo SDK >= 25.x. If your SDK's pointer / SIMD API differs, the only things
# to adjust are the `UnsafePointer` types and the `sys.info.simdwidthof` call —
# the algorithm is unchanged.
#
# Build:   mojo build knn.mojo -o knn
# Run:     ./knn bench.bin        (format in format.md)

from sys import argv, simdwidthof
from sys.info import sizeof
from memory import UnsafePointer
from algorithm import vectorize, parallelize
from math import sqrt
from time import perf_counter_ns

alias F32 = DType.float32
alias I64 = DType.int64
alias SIMD_W = simdwidthof[F32]()


# ── the kernel ──────────────────────────────────────────────────────────────

fn dot(a: UnsafePointer[Scalar[F32]], b: UnsafePointer[Scalar[F32]], dim: Int) -> Float32:
    """SIMD dot product of two `dim`-length f32 rows."""
    var acc = SIMD[F32, SIMD_W](0)

    @parameter
    fn body[w: Int](i: Int):
        var av = a.load[width=w](i)
        var bv = b.load[width=w](i)
        acc = acc + (av * bv).reduce_add().cast[F32]().splat[SIMD_W]() if w != SIMD_W else acc + av * bv

    vectorize[body, SIMD_W](dim)
    return acc.reduce_add()


fn topk_cosine(
    corpus: UnsafePointer[Scalar[F32]],
    n: Int,
    dim: Int,
    queries: UnsafePointer[Scalar[F32]],
    nq: Int,
    k: Int,
    out_ids: UnsafePointer[Scalar[I64]],
    out_scores: UnsafePointer[Scalar[F32]],
):
    """For every query, the exact k nearest corpus rows by cosine (== dot,
    rows are unit-norm). Parallel over queries; SIMD over `dim`."""

    @parameter
    fn one_query(q: Int):
        var qp = queries.offset(q * dim)
        var ids = out_ids.offset(q * k)
        var sc = out_scores.offset(q * k)

        # tiny insertion-sorted top-k (k is small: 4-16)
        for j in range(k):
            ids[j] = -1
            sc[j] = -1.0e30

        for row in range(n):
            var s = dot(corpus.offset(row * dim), qp, dim)
            if s <= sc[k - 1]:
                continue
            var p = k - 1
            while p > 0 and sc[p - 1] < s:
                sc[p] = sc[p - 1]
                ids[p] = ids[p - 1]
                p -= 1
            sc[p] = s
            ids[p] = row

    parallelize[one_query](nq, nq)


# ── standalone bench runner (reads bench.bin, times the kernel) ──────────────

fn read_u32(p: UnsafePointer[UInt8], off: Int) -> Int:
    return (
        Int(p[off])
        | (Int(p[off + 1]) << 8)
        | (Int(p[off + 2]) << 16)
        | (Int(p[off + 3]) << 24)
    )


fn main() raises:
    var args = argv()
    if len(args) < 2:
        print("usage: knn <bench.bin>")
        return
    var path = String(args[1])

    with open(path, "r+") as f:
        var blob = f.read_bytes()
        var base = blob.unsafe_ptr()

        var magic = read_u32(base, 0)
        if magic != 0x4B4E4E31:
            print("bad magic:", magic)
            return
        var n = read_u32(base, 4)
        var dim = read_u32(base, 8)
        var nq = read_u32(base, 12)
        var k = read_u32(base, 16)

        var hdr = 24
        var corpus = base.offset(hdr).bitcast[Scalar[F32]]()
        var queries = corpus.offset(n * dim)
        var out_ids = queries.offset(nq * dim).bitcast[Scalar[I64]]()
        var out_scores = out_ids.offset(nq * k).bitcast[Scalar[F32]]()

        print("n=", n, " dim=", dim, " nq=", nq, " k=", k, " simd_w=", SIMD_W)

        # warm + timed
        topk_cosine(corpus, n, dim, queries, nq, k, out_ids, out_scores)
        var t0 = perf_counter_ns()
        var reps = 5
        for _ in range(reps):
            topk_cosine(corpus, n, dim, queries, nq, k, out_ids, out_scores)
        var t1 = perf_counter_ns()

        var per_query_us = Float64(t1 - t0) / Float64(reps * nq) / 1000.0
        print("median ~", per_query_us, "us/query  (", Float64(nq) / (Float64(t1 - t0) / 1e9 / Float64(reps)), "q/s )")

        # results are already written back into the mmap'd file regions
        f.write_bytes(blob)
