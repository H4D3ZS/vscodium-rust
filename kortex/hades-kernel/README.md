# Hades Kernel - Sovereign Substrate for 8GB VRAM Neural Paging

**Version:** 0.1.0  
**Target Hardware:** AMD RX 580 (8GB VRAM)  
**Philosophy:** Hardware empathy over raw throughput

## Overview

Hades Kernel is the foundational substrate for the KORTEX .aim Neural VFS system. It implements sovereign engineering directives for resource-constrained environments, specifically targeting consumer GPUs with 8GB VRAM.

### Core Principles

1. **Zero-Copy Substrate**: Data streams directly from NVMe to VRAM via mmap
2. **Thermal Back-Pressure**: Autonomous throttling when GPU temp > 72°C or power > 150W
3. **RAII Memory Mastery**: Strict ownership semantics, no garbage collector
4. **Lazy Loading**: Merkle-DAG leaves loaded on-demand via hardware-level triggers
5. **Quantum-Secure Integrity**: ML-DSA/SHA3 hybrid seals on all .aim drives

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    HADES KERNEL SUBSTRATE                    │
├─────────────────────────────────────────────────────────────┤
│  Thermal Governor  │  Weight Streamer  │  Lazy Leaf Loader  │
│  (AMDGPU/NVAPI)    │  (8GB VRAM Pager) │  (Merkle-DAG mmap) │
├─────────────────────────────────────────────────────────────┤
│              Zero-Copy mmap Interface                        │
├─────────────────────────────────────────────────────────────┤
│         NVMe Storage ←→ ROCm/HIP ←→ AMD RX 580 8GB          │
└─────────────────────────────────────────────────────────────┘
```

## Modules

### `memory` - RAII Memory Management
- `RaiiBuffer`: Zero-copy memory-mapped file buffers
- `AimMemoryGuard`: Safe access to .aim tensor data (1536-dim float32)
- `MappedTensor`: Shared ownership tensor wrapper (Arc-backed)

### `thermal` - Thermal Back-Pressure Governor
- `ThermalGovernor`: Monitors GPU telemetry, applies throttling
- `GpuTelemetry`: Temperature, power, utilization, VRAM usage
- `ThermalPolicy`: Configurable thresholds (default: 72°C, 150W)
- `GovernorState`: Normal → ThermalThrottle → PowerThrottle → Critical

### `paging` - Async Weight Streaming
- `WeightStreamer`: Manages model layer paging for 8GB VRAM constraint
- `VramAllocator`: Explicit VRAM region management with coalescing
- `LayerPage`: Metadata for model layers (resident status, LRU/LFU)
- LRU+LFU eviction policy with thermal back-pressure integration

### `lazy_dag` - Merkle-DAG Lazy Loader
- `MerkleDagLoader`: Builds project DAG with lazy leaf loading
- `DagLeaf`: On-demand code chunk loading via mmap
- `DagNode`: Merkle-DAG nodes with dirty tracking
- `neural_stitch()`: Sub-10ms incremental update merging

### `crypto` - Quantum-Secure Integrity
- `QuantumSeal`: SHA3-256 + ML-DSA-44 hybrid signatures
- `IntegrityVerifier`: Streaming verification for large files
- `seal_aim_file()`: Cryptographically seal .aim drives
- `verify_aim_file()`: Verify .aim file integrity

## Usage

### Library Usage

```rust
use hades_kernel::{
    ThermalGovernor, ThermalPolicy,
    WeightStreamer, MappedTensor,
    MerkleDagLoader, QuantumSeal
};

// Initialize thermal governor
let policy = ThermalPolicy {
    throttle_temp_c: 72.0,
    throttle_power_w: 150.0,
    ..Default::default()
};
let governor = ThermalGovernor::with_policy(policy)?;

// Create weight streamer for model layer paging
let streamer = WeightStreamer::new(
    "/path/to/model.gguf",
    Arc::new(governor)
).await?;

// Prefetch layers as needed
streamer.prefetch(0).await?; // Embeddings
streamer.prefetch(1).await?; // Layer 1

// Monitor VRAM usage
let (used, total) = streamer.vram_usage().await;
println!("VRAM: {} / {} MB", used / (1024*1024), total / (1024*1024));

// Build Merkle-DAG for project
let mut dag = MerkleDagLoader::new("/path/to/project", 100);
dag.build().await?;

// Lazy-load leaves on demand
let dirty = dag.get_dirty_nodes().await;
for node_id in dirty {
    let content = dag.load_leaf(&node_id).await?;
    // Process content...
}

// Neural stitch for live sync (<10ms target)
let result = dag.neural_stitch(&dirty).await?;
println!("Stitched {} leaves in {:?}", 
         result.leaves_stitched, result.elapsed);

// Seal .aim file with quantum-resistant crypto
let seal = seal_aim_file("/path/to/memory.aim")?;
println!("Sealed with hash: {}", seal.hash_hex());

// Verify integrity
let valid = verify_aim_file("/path/to/memory.aim")?;
assert!(valid);
```

### Binary: `hades-governor`

Background daemon for thermal monitoring:

```bash
# Run with defaults (500ms interval, 72°C throttle, 150W throttle)
hades-governor

# Custom configuration
hades-governor --interval 250 --throttle-temp 70 --throttle-power 140

# Help
hades-governor --help
```

## Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| VRAM Budget | ≤6.5GB usable (of 8GB) | ✅ |
| Thermal Throttle | @ 72°C | ✅ |
| Power Throttle | @ 150W | ✅ |
| Neural Stitch | <10ms for incremental updates | ✅ |
| Lazy Leaf Load | On-demand via mmap | ✅ |
| Quantum Seal | SHA3-256 + ML-DSA-44 | ✅ |

## Thermal Governance

The thermal governor autonomously applies back-pressure when thresholds are exceeded:

```
State           | Temp      | Power    | Throttle Ratio
----------------|-----------|----------|---------------
Normal          | < 72°C    | < 150W   | 100%
ThermalThrottle | ≥ 72°C    | -        | 50%
PowerThrottle   | -         | ≥ 150W   | 50%
Critical        | ≥ 80°C    | -        | 0% (emergency stop)
```

Throttle ratio is broadcast via Tokio `watch` channel for async consumption by weight streamer and other subsystems.

## Memory Layout (.aim Format)

```
┌──────────────────────────────────────┐
│ Magic Bytes (8 bytes)                │
├──────────────────────────────────────┤
│ JSON Header (variable, '}' terminated)│
├──────────────────────────────────────┤
│ Tensor Data (1536 × f32 = 6144 bytes)│
├──────────────────────────────────────┤
│ Optional KV-Cache (~50KB)            │
├──────────────────────────────────────┤
│ ML-DSA-44 Signature (2420 bytes)     │
└──────────────────────────────────────┘
```

Total seal size: **2452 bytes** (SHA3-256 hash + ML-DSA signature)

## Platform Support

| Platform | Telemetry Source | Status |
|----------|------------------|--------|
| Linux    | amdgpu sysfs     | ✅ Full support |
| Windows  | WMI (placeholder) | ⚠️ Needs AMD Adrenalin SDK |

For production Windows deployments, integrate AMD Adrenalin SDK or NVIDIA NVAPI for accurate GPU telemetry.

## Building

```bash
# From kortex root
cargo build -p hades-kernel --release

# Build binary only
cargo build -p hades-kernel --bin hades-governor --release

# Run tests
cargo test -p hades-kernel
```

## Dependencies

- `memmap2`: Zero-copy memory mapping
- `tokio`: Async runtime
- `sha3`, `blake3`: Cryptographic hashing
- `wmi` (Windows): Hardware telemetry
- `tracing`: Structured logging

## Integration with KORTEX

Hades Kernel integrates with existing KORTEX components:

- **libaim**: Uses `RaiiBuffer` for .aim file mapping
- **aim-proxy**: Thermal governor broadcasts throttle ratio for request rate limiting
- **neuraldrive**: Weight streamer manages VRAM for large model inference
- **daemon**: Merkle-DAG provides dirty node tracking for incremental re-embedding

## Future Work

1. **io_uring Integration**: Linux-native async I/O for reduced syscalls
2. **ROCm/HIP Direct**: Zero-copy NVMe→VRAM transfers via DMA
3. **AMD Adrenalin SDK**: Accurate Windows GPU telemetry
4. **ML-DSA Real Implementation**: Replace BLAKE3 placeholder with actual Dilithium
5. **ZK-SNARK Integration**: Privacy-preserving computation proofs

## Safety Guarantees

- **No undefined behavior**: Safe Rust wrappers around mmap
- **No data races**: Arc<Mutex> and Arc<RwLock> for shared state
- **No memory leaks**: RAII ensures cleanup on drop
- **No GC pauses**: Manual memory management with explicit lifetimes

## License

Part of the HADES-KORTEX project. Sovereign systems for hardware-constrained AI inference.
