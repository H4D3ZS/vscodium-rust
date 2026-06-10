//! ANE diagnostics: verifies the similarity kernel executes on this hardware
//! and reports measured ANE vs CPU latency for the vector-index workload.
#![cfg(all(target_os = "macos", target_arch = "aarch64"))]
use vscode_rust_app_lib::ane_inference::AneInferenceOptimizer;

#[test]
fn ane_similarity_bench() {
    let opt = AneInferenceOptimizer::new();
    let dim = 768usize;
    let n = 1024usize; // 4 full batches — realistic indexed-project size

    let make_unit = |seed: usize| -> Vec<f32> {
        let mut v: Vec<f32> = (0..dim)
            .map(|i| (((seed * 31 + i * 17) % 1000) as f32 / 500.0) - 1.0)
            .collect();
        let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        v.iter_mut().for_each(|x| *x /= norm);
        v
    };
    let query = make_unit(7);
    let embs: Vec<Vec<f32>> = (0..n).map(make_unit).collect();
    let refs: Vec<&[f32]> = embs.iter().map(|e| e.as_slice()).collect();

    // Warmup + availability check
    let Some(first) = opt.similarity_batch(&query, &refs) else {
        eprintln!("ANE unavailable on this machine — skipping bench");
        return;
    };
    assert_eq!(first.len(), n);

    let its = 10;
    let t = std::time::Instant::now();
    for _ in 0..its {
        opt.similarity_batch(&query, &refs).unwrap();
    }
    let ane_ms = t.elapsed().as_secs_f64() * 1000.0 / its as f64;

    // Prepared path: embeddings pre-packed once (how ann_index uses the ANE).
    let mut batches = opt.prepare_sim_batches(&refs).unwrap();
    let prep_scores = opt.similarity_prepared(&query, &mut batches).unwrap();
    assert_eq!(prep_scores.len(), n);
    let t = std::time::Instant::now();
    for _ in 0..its {
        opt.similarity_prepared(&query, &mut batches).unwrap();
    }
    let prep_ms = t.elapsed().as_secs_f64() * 1000.0 / its as f64;

    let t = std::time::Instant::now();
    for _ in 0..its {
        let _s: Vec<f32> = refs
            .iter()
            .map(|e| query.iter().zip(e.iter()).map(|(a, b)| a * b).sum())
            .collect();
    }
    let cpu_ms = t.elapsed().as_secs_f64() * 1000.0 / its as f64;

    println!("[ANE bench] {n} x {dim}-dim similarity: ANE cold {ane_ms:.2}ms | ANE prepared {prep_ms:.2}ms | CPU {cpu_ms:.2}ms per search");

    // Correctness across the full set, both paths
    for (i, emb) in embs.iter().enumerate() {
        let cpu: f32 = query.iter().zip(emb.iter()).map(|(a, b)| a * b).sum();
        assert!((first[i] - cpu).abs() < 0.02, "batch score {i}: ane={} cpu={cpu}", first[i]);
        assert!((prep_scores[i] - cpu).abs() < 0.02, "prepared score {i}: ane={} cpu={cpu}", prep_scores[i]);
    }
}
