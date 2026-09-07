//! Single-thread Rust baseline the Mojo kernel must beat. Mirrors
//! `kortex/libaim/src/embed.rs::dot` (8-lane, autovectorised) + a small
//! insertion top-k — the exact code path Mojo would replace. Not part of any
//! crate; build ad hoc:
//!
//!   rustc -O reference.rs -o reference && ./reference bench.bin
//!
//! Reads/writes the same `bench.bin` format (see format.md) so `bench.py`
//! could point at this instead of `./knn` for an apples-to-apples number.
//!
//! NOTE: the libaim kernel is no longer a naive `.sum()` — it uses eight
//! independent lane accumulators so the compiler emits real SIMD. The Mojo
//! bar is ">= 2x over THIS", not over the old scalar loop.

use std::fs;
use std::time::Instant;

fn u32_le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

fn dot(a: &[f32], q: &[f32]) -> f32 {
    // Mirror of libaim::embed::dot — 8 lane accumulators, associative by
    // construction, so LLVM vectorises the loop.
    let n = a.len().min(q.len());
    let (a, q) = (&a[..n], &q[..n]);
    let mut acc = [0f32; 8];
    let mut ca = a.chunks_exact(8);
    let mut cq = q.chunks_exact(8);
    for (x, y) in ca.by_ref().zip(cq.by_ref()) {
        for l in 0..8 {
            acc[l] += x[l] * y[l];
        }
    }
    let mut s = ((acc[0] + acc[1]) + (acc[2] + acc[3])) + ((acc[4] + acc[5]) + (acc[6] + acc[7]));
    for (x, y) in ca.remainder().iter().zip(cq.remainder()) {
        s += x * y;
    }
    s
}

fn topk(corpus: &[f32], n: usize, dim: usize, q: &[f32], k: usize, ids: &mut [i64], sc: &mut [f32]) {
    for j in 0..k {
        ids[j] = -1;
        sc[j] = -1e30;
    }
    for row in 0..n {
        let s = dot(&corpus[row * dim..(row + 1) * dim], q);
        if s <= sc[k - 1] {
            continue;
        }
        let mut p = k - 1;
        while p > 0 && sc[p - 1] < s {
            sc[p] = sc[p - 1];
            ids[p] = ids[p - 1];
            p -= 1;
        }
        sc[p] = s;
        ids[p] = row as i64;
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: reference <bench.bin>");
    let mut blob = fs::read(&path).unwrap();

    assert_eq!(u32_le(&blob, 0), 0x4B4E_4E31, "bad magic");
    let n = u32_le(&blob, 4) as usize;
    let dim = u32_le(&blob, 8) as usize;
    let nq = u32_le(&blob, 12) as usize;
    let k = u32_le(&blob, 16) as usize;

    let hdr = 24;
    let corpus_bytes = n * dim * 4;
    let queries_bytes = nq * dim * 4;
    let ids_off = hdr + corpus_bytes + queries_bytes;
    let sc_off = ids_off + nq * k * 8;

    let corpus: Vec<f32> = blob[hdr..hdr + corpus_bytes]
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    let queries: Vec<f32> = blob[hdr + corpus_bytes..hdr + corpus_bytes + queries_bytes]
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();

    let mut ids = vec![0i64; nq * k];
    let mut sc = vec![0f32; nq * k];

    // warm
    for q in 0..nq {
        topk(&corpus, n, dim, &queries[q * dim..(q + 1) * dim], k, &mut ids[q * k..(q + 1) * k], &mut sc[q * k..(q + 1) * k]);
    }
    let reps = 5;
    let t0 = Instant::now();
    for _ in 0..reps {
        for q in 0..nq {
            topk(&corpus, n, dim, &queries[q * dim..(q + 1) * dim], k, &mut ids[q * k..(q + 1) * k], &mut sc[q * k..(q + 1) * k]);
        }
    }
    let us_per_query = t0.elapsed().as_micros() as f64 / (reps * nq) as f64;
    eprintln!("reference.rs (scalar, 1 thread): {us_per_query:8.1} us/query");

    // write results back
    for (i, v) in ids.iter().enumerate() {
        blob[ids_off + i * 8..ids_off + i * 8 + 8].copy_from_slice(&v.to_le_bytes());
    }
    for (i, v) in sc.iter().enumerate() {
        blob[sc_off + i * 4..sc_off + i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    fs::write(&path, &blob).unwrap();
}
