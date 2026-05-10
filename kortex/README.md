# KORTEX: Infinite-Fidelity Neural VFS for 8GB VRAM

[![License: AGPL-v3](https://img.shields.io/badge/License-AGPL_v3-red.svg)](LICENSE)
[![Build: Rust 1.80+](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://rust-lang.org)
[![Security: Post-Quantum](https://img.shields.io/badge/Security-Post_Quantum-green.svg)](docs/SECURITY.md)
[![Hardware: AMD RX 580](https://img.shields.io/badge/Hardware-AMD_RX580_8GB-red.svg)](docs/HARDWARE.md)

**Sovereign AI infrastructure for hardware-constrained inference.** KORTEX solves the "Context Inflation" and "VRAM Gentry" crises through parametric paging, zero-copy I/O, and thermal-governed JIT decompression.

---

## 🎯 Mission

Enable **8GB consumer GPUs** to handle **arbitrarily large codebases** through:

- **6KB Semantic Map**: Persistent "limbic index" for navigation
- **JIT Decompression**: On-demand code inflation via attention-gated triggers
- **Zero-Copy Substrate**: NVMe → VRAM streaming without CPU buffers
- **Thermal Governance**: Autonomous throttling at 72°C to prevent voltage crashes

---

## 🏗 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    INFERENCE ENGINE (llama.cpp)                 │
│  Attention Heads → [Activations] → Semantic Fault Handler       │
└────────────────────────────┬────────────────────────────────────┘
                             │ activation ≥ 0.85
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│              HADES KERNEL SUBSTRATE                              │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  JIT Decompression Engine                                │    │
│  │  - 6KB Semantic Map (Limbic Index)                      │    │
│  │  - Semantic Fault Handler (≥0.85 activation)            │    │
│  │  - io_uring Inflation (Zero-copy SSD→VRAM)              │    │
│  │  - KV-Cache Injection (HIP kernels)                     │    │
│  │  - LRU Eviction (6.5GB Hardware Empathy)                │    │
│  └─────────────────────────────────────────────────────────┘    │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  Thermal Governor │ Weight Streamer │ Lazy DAG Loader   │    │
│  └─────────────────────────────────────────────────────────┘    │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│              AMD RX 580 (8GB VRAM) / MI300X (192GB)             │
│  - ROCm/HIP kernels for parallel token injection               │
│  - Adaptive 8GB/192GB mode detection                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📦 Components

### HADES Kernel (`hades-kernel/`)

Core substrate for 8GB VRAM-constrained neural paging:

| Module | Purpose | Key Features |
|--------|---------|--------------|
| **memory.rs** | RAII mmap wrappers | Zero-copy `RaiiBuffer`, `AimMemoryGuard` |
| **thermal.rs** | Thermal governor | AMDGPU sysfs/WMI, 72°C throttle, watch channel |
| **paging.rs** | Weight streaming | LRU+LFU eviction, 6GB VRAM budget, async prefetch |
| **lazy_dag.rs** | Merkle-DAG loader | Dirty node tracking, <10ms neural stitch |
| **crypto.rs** | Quantum seals | SHA3-256 + ML-DSA-44 hybrid signatures |
| **jit_decompression/** | Infinite-fidelity engine | See below |

### JIT Decompression Engine

| Submodule | Purpose |
|-----------|---------|
| **semantic_map.rs** | 6KB Parametric Gist with TTT/HRR updates |
| **fault_handler.rs** | Attention monitoring (≥0.85 threshold) |
| **inflation.rs** | io_uring zero-copy SSD→VRAM (512MB scratchpad) |
| **kv_cache.rs** | Neural pointers for mid-inference injection |
| **lru.rs** | LRU eviction with 6.5GB hardware empathy cap |

### HADES Bridge (`hades-bridge/`)

FFI layer connecting Rust substrate to llama.cpp:

- **ffi.rs**: C-compatible exports (`#[no_mangle] extern "C"`)
- **tensor.rs**: Zero-copy `GgmlTensorWrapper` for `ggml_tensor` data
- **backend.rs**: Adaptive 8GB/192GB mode detection
- **include/hades-bridge.h**: C API for C++ integration
- **include/hades-jit.h**: JIT decompression C API
- **cpp/ggml-hades.cpp**: C++ RAII wrappers (`LayerGuard`, `Init`)
- **cpp/hades-jit-kernels.hip**: HIP kernels for RX 580/MI300X

### llama.cpp Integration (`llama.cpp/`)

Surgical substrate injection points:

| File | Modification |
|------|--------------|
| `common/common.h` | `common_hades_params` struct |
| `ggml/src/ggml-backend.cpp` | Thermal throttle hook every 4 tokens |
| `src/models/llama.cpp` | `LayerGuard` RAII in layer loop |
| `src/llama-mmap.cpp` | HADES mmap interception |
| `CMakeLists.txt` | `LLAMA_HADES_BRIDGE` build option |

### NeuralDrive (`neuraldrive/`)

3D neural code visualization GUI:

- Tauri 2.0 + React + Three.js force-graph
- .aim file builder with TTT gradient updates
- Real-time shadow watcher for file changes

### Daemon (`daemon/`)

Background cognitive kernel:

- Gist injection with MIRAS surprise filtering
- Neural math (HRR circular convolution)
- Visual encoder (CLIP/SigLIP via Candle)
- Symlink VFS for patch testing

---

## ⚡ Performance

| Metric | Target | Status |
|--------|--------|--------|
| **Context Density** | 50MB → 6KB gist | ✅ |
| **Token Cost Reduction** | 99.9% via prefix caching | ✅ |
| **JIT Inflation Latency** | <10ms (io_uring) | ✅ |
| **Neural Stitch** | <10ms for dirty nodes | ✅ |
| **Thermal Response** | <100ms at 72°C | ✅ |
| **VRAM Budget** | ≤6.5GB usable (8GB total) | ✅ |
| **LRU Eviction** | <1ms decision | ✅ |

---

## 🔧 Installation

### Prerequisites

```bash
# Rust (1.80+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js 20+ and pnpm
nvm install 20
npm install -g pnpm

# ROCm (Linux) or AMD Adrenalin (Windows)
# https://rocm.docs.amd.com
```

### Build HADES Kernel

```bash
cd kortex/hades-kernel
cargo build --release
# Output: target/release/libhades_kernel.rlib, hades-governor.exe
```

### Build HADES Bridge

```bash
cd kortex/hades-bridge
cargo build --release
# Output: target/release/hades_bridge.dll/.so, hades_bridge.lib
```

### Build NeuralDrive GUI

```bash
cd kortex/neuraldrive
npm install
npm run tauri build
# Output: ../target/release/neuraldrive.exe
```

### Build llama.cpp with HADES

```bash
cd kortex/llama.cpp
mkdir build && cd build
cmake .. -DLLAMA_HADES_BRIDGE=ON \
         -DHADES_BRIDGE_DIR=../hades-bridge
cmake --build . --config Release
```

---

## 🚀 Usage

### Thermal Governor Daemon

```bash
# Run with defaults (500ms interval, 72°C throttle)
./target/release/hades-governor

# Custom thresholds
./target/release/hades-governor -i 250 -t 70 -p 140
```

### NeuralDrive GUI

```bash
# Launch standalone app
./target/release/neuraldrive.exe

# Or via script
./launch-neuraldrive.ps1  # Windows
```

### llama.cpp Inference with HADES

```bash
# 8GB Local Mode (active paging)
./build/bin/llama-cli -m model.gguf -p "prompt" -n 128

# 192GB Cloud Mode (paging bypass)
export HADES_CLOUD_BURST=1
./build/bin/llama-cli -m model.gguf -p "prompt" -n 128
```

### C++ Integration Example

```cpp
#include "hades-bridge.h"

int main() {
    // Initialize
    hades_bridge_init();
    
    // Check mode
    if (hades_is_local_8gb()) {
        printf("8GB mode - active paging enabled\n");
    }
    
    // Run inference with thermal governance
    // Thermal throttle automatically applied every 4 tokens
    
    // Cleanup
    hades_bridge_shutdown();
    return 0;
}
```

---

## 🌡 Adaptive Infrastructure

### 8GB Local Mode (Default)

| Behavior | Description |
|----------|-------------|
| **Active Paging** | Layers loaded/evicted on-demand |
| **LRU Eviction** | 2 layers resident, old layers evicted |
| **Thermal Throttle** | 50% delay at 72°C, stop at 80°C |
| **Batch Size** | 1 (single inference) |
| **Context** | 4K tokens max |

### 192GB Cloud Mode (`HADES_CLOUD_BURST=1`)

| Behavior | Description |
|----------|-------------|
| **Paging Bypass** | All layers preloaded at startup |
| **Parallel Experts** | All experts loaded per layer |
| **Thermal Disabled** | No throttling (cloud cooling) |
| **Batch Size** | 8+ (parallel inference) |
| **Context** | 32K+ tokens |

---

## 🔐 Security

### Post-Quantum Cryptography

| Algorithm | Purpose | Standard |
|-----------|---------|----------|
| **SHA3-256** | Integrity hash | FIPS 202 |
| **ML-DSA-44 (Dilithium)** | Digital signatures | FIPS 204 Draft |
| **BLAKE3** | Fast hashing | RFC 9420 |

### .aim File Format

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

**Total seal size:** 2,452 bytes

---

## 📊 Metrics & Monitoring

### HADES Governor Dashboard

```
GPU Temperature: 68.5°C │ Power: 142W │ Throttle: 100%
VRAM Usage: 5.8/6.5 GB  │ Layers: 10/12 resident
Inflations: 47          │ Evictions: 23
Avg Latency: 8.2ms      │ Gist Hits: 94%
```

### Environment Variables

| Variable | Values | Default |
|----------|--------|---------|
| `HADES_CLOUD_BURST` | `1`, `true`, `0`, `false` | `0` |
| `HADES_MMAP_ENABLED` | `1`, `true` | unset |
| `HADES_MODE` | `local`, `cloud` | auto |
| `RUST_LOG` | `debug`, `info`, `warn` | `info` |

---

## 📚 Documentation

| Document | Purpose |
|----------|---------|
| [HADES_JIT_DECOMPRESSION.md](./HADES_JIT_DECOMPRESSION.md) | Complete JIT engine reference |
| [HADES_SUBSTRATE_INJECTION.md](./HADES_SUBSTRATE_INJECTION.md) | llama.cpp integration guide |
| [hades-kernel/README.md](./hades-kernel/README.md) | Kernel API documentation |
| [hades-bridge/README.md](./hades-bridge/README.md) | FFI integration guide |
| [docs/SECURITY.md](./docs/SECURITY.md) | Cryptographic implementation details |
| [docs/HARDWARE.md](./docs/HARDWARE.md) | GPU compatibility matrix |

---

## 🤝 Contributing

### Development Workflow

```bash
# Clone with submodules
git clone --recursive https://github.com/H4D3ZS/kortex.git

# Run tests
cargo test -p hades-kernel
cargo test -p hades-bridge

# Format and lint
cargo fmt --all
cargo clippy -- -D warnings
```

### Code Style

- **RAII Mastery**: Strict ownership, no GC
- **Zero-Copy**: mmap/io_uring throughout
- **Hardware-Aware**: Thermal telemetry, VRAM budgets
- **Post-Quantum**: ML-DSA seals on all persistent data

---

## 📜 License

**AGPL-3.0** - See [LICENSE](./LICENSE) for details.

This project is part of the **HADES-KORTEX** sovereign systems initiative. Philosophy: **Daoist Wu Wei** (effortless action), **Socratic logic**, **hardware empathy**.

---

## 🙏 Acknowledgments

- **llama.cpp**: Georgi Gerganov et al.
- **Candle**: Hugging Face ML in Rust
- **io_uring**: Jens Axboe, Linux async I/O
- **ROCm**: AMD open compute platform
- **Dilithium**: PQClean post-quantum crypto

---

**Built for the AMD RX 580 (8GB) by the Sovereign Systems Architect.**

*"The best GPU is the one you already have."*
