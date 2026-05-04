use criterion::{criterion_group, criterion_main, Criterion};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct LargePayload {
    id: String,
    content: String,
    metadata: std::collections::HashMap<String, String>,
    tags: Vec<String>,
}

fn create_payload() -> LargePayload {
    let mut metadata = std::collections::HashMap::new();
    for i in 0..100 {
        metadata.insert(format!("key_{}", i), format!("value_{}", i));
    }
    LargePayload {
        id: "test-uuid-001".to_string(),
        content: "A".repeat(10000), // 10KB string
        metadata,
        tags: vec!["performance".to_string(); 50],
    }
}

fn bench_json_serialization(c: &mut Criterion) {
    let payload = create_payload();
    c.bench_function("json_serialize", |b| b.iter(|| {
        serde_json::to_string(&payload).unwrap()
    }));
}

fn bench_json_deserialization(c: &mut Criterion) {
    let payload = create_payload();
    let json_str = serde_json::to_string(&payload).unwrap();
    c.bench_function("json_deserialize", |b| b.iter(|| {
        let _: LargePayload = serde_json::from_str(&json_str).unwrap();
    }));
}

fn bench_msgpack_serialization(c: &mut Criterion) {
    let payload = create_payload();
    c.bench_function("msgpack_serialize", |b| b.iter(|| {
        rmp_serde::to_vec(&payload).unwrap()
    }));
}

fn bench_msgpack_deserialization(c: &mut Criterion) {
    let payload = create_payload();
    let bin_data = rmp_serde::to_vec(&payload).unwrap();
    c.bench_function("msgpack_deserialize", |b| b.iter(|| {
        let _: LargePayload = rmp_serde::from_slice(&bin_data).unwrap();
    }));
}

criterion_group!(
    benches,
    bench_json_serialization,
    bench_json_deserialization,
    bench_msgpack_serialization,
    bench_msgpack_deserialization
);
criterion_main!(benches);
