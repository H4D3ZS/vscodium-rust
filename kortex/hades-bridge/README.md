# HADES Bridge - FFI Layer for llama.cpp Integration

**Version:** 0.1.0  
**Purpose:** Connect HADES Kernel Rust substrate to llama.cpp C++ inference engine  
**Target:** Zero-copy tensor exchange with thermal-governed paging

## Overview

HADES Bridge provides a C-compatible FFI interface that enables llama.cpp to leverage the HADES Kernel's:

- **Weight Streaming**: Active layer paging for 8GB VRAM constraints
- **Thermal Governance**: Autonomous throttling based on GPU temperature/power
- **Adaptive Infrastructure**: Automatic detection of 8GB local vs 192GB cloud GPU
- **Zero-Copy Tensors**: Safe pointer exchange without data duplication

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     llama.cpp (C++)                              │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  ggml-hades.cpp                                         │    │
│  │  - Paging hooks in decode loop                          │    │
│  │  - Tensor wrappers                                      │    │
│  │  - RAII LayerGuard                                      │    │
│  └────────────────────┬────────────────────────────────────┘    │
│                       │ FFI (C ABI)                              │
└───────────────────────┼─────────────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────────────┐
│                  HADES Bridge (Rust)                             │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  ffi.rs - C-compatible exports                          │    │
│  │  - hades_bridge_init()                                  │    │
│  │  - hades_check_throttle()                               │    │
│  │  - hades_page_layer()                                   │    │
│  │  - hades_tensor_wrap()                                  │    │
│  └─────────────────────────────────────────────────────────┘    │
│                          │                                       │
│  ┌───────────────────────▼─────────────────────────────────┐    │
│  │  HADES Kernel                                           │    │
│  │  - ThermalGovernor                                      │    │
│  │  - WeightStreamer                                       │    │
│  │  - MappedTensor                                         │    │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

## Building

### Rust Library

```bash
cd kortex
cargo build -p hades-bridge --release
```

This produces:
- `target/release/hades_bridge.dll` (Windows) / `libhades_bridge.so` (Linux)
- `target/release/libhades_bridge.rlib` (static library for Rust)

### C++ Integration

The C++ integration layer is designed to be built as part of llama.cpp:

```cmake
# In llama.cpp CMakeLists.txt
add_subdirectory(${HADES_DIR}/hades-bridge)
target_link_libraries(ggml PRIVATE hades_bridge)
target_include_directories(ggml PRIVATE ${HADES_DIR}/hades-bridge/include)
```

## Usage

### From C++ (llama.cpp)

```cpp
#include "hades-bridge.h"
#include "ggml.h"

int main() {
    // Initialize HADES bridge
    if (!hades_bridge_init()) {
        fprintf(stderr, "HADES init failed\n");
        return 1;
    }
    
    // Check adaptive mode
    if (hades_is_local_8gb()) {
        printf("8GB local mode - active paging enabled\n");
    }
    
    // Initialize weight streamer for model
    hades_init_weight_streamer("/path/to/model.gguf");
    
    // In decode loop, before each layer:
    for (int i = 0; i < n_layers; i++) {
        // HADES paging hook - prefetches next layers
        hades_paging_hook(i, i == 0, i == n_layers - 1);
        
        // Apply thermal back-pressure if needed
        if (hades_check_throttle() < 1.0f) {
            hades_apply_back_pressure();
        }
        
        // Compute layer
        ggml_compute_forward(...);
    }
    
    // Cleanup
    hades_bridge_shutdown();
    return 0;
}
```

### Using the C++ RAII Wrapper

```cpp
#include "ggml-hades.cpp"

int main() {
    // RAII initialization
    ggml::backend::hades::Init hades_init("/path/to/model.gguf");
    if (!hades_init) {
        return 1;
    }
    
    // Decode loop with automatic paging hooks
    for (int i = 0; i < n_layers; i++) {
        ggml::backend::hades::LayerGuard guard(i);
        // Layer compute happens here with automatic pre/post hooks
        ggml_compute_forward(...);
    }
    
    // Automatic shutdown when hades_init goes out of scope
    return 0;
}
```

### From Rust

```rust
use hades_bridge::*;

fn main() {
    // Initialize
    unsafe {
        if !hades_bridge_init() {
            eprintln!("Failed to initialize");
            return;
        }
    }
    
    // Check VRAM
    let total = unsafe { hades_get_total_vram() };
    let usable = unsafe { hades_get_usable_vram() };
    println!("VRAM: {} GB total, {} GB usable", 
             total / (1024^3), usable / (1024^3));
    
    // Check throttle
    let throttle = unsafe { hades_check_throttle() };
    println!("Throttle ratio: {}", throttle);
    
    // Shutdown
    unsafe { hades_bridge_shutdown() };
}
```

## FFI Reference

### Initialization

| Function | Description |
|----------|-------------|
| `hades_bridge_init()` | Initialize bridge (call once at startup) |
| `hades_bridge_shutdown()` | Cleanup (call once at termination) |
| `hades_bridge_is_initialized()` | Check if initialized |

### VRAM Detection

| Function | Description |
|----------|-------------|
| `hades_get_total_vram()` | Get total VRAM in bytes |
| `hades_get_usable_vram()` | Get usable VRAM (after overhead) |
| `hades_is_cloud_burst()` | Check if 192GB cloud mode |
| `hades_is_local_8gb()` | Check if 8GB local mode |

### Thermal Governor

| Function | Description |
|----------|-------------|
| `hades_check_throttle()` | Get throttle ratio (0.0-1.0) |
| `hades_get_gpu_temp()` | Get GPU temperature (°C) |
| `hades_get_gpu_power()` | Get GPU power (Watts) |
| `hades_apply_back_pressure()` | Block with thermal delay |

### Weight Streaming

| Function | Description |
|----------|-------------|
| `hades_init_weight_streamer(path)` | Initialize for GGUF model |
| `hades_page_layer(id)` | Prefetch layer to VRAM |
| `hades_evict_layer(id)` | Evict layer from VRAM |
| `hades_get_vram_usage(&used, &total)` | Get VRAM usage |

### Tensor FFI

| Function | Description |
|----------|-------------|
| `hades_tensor_wrap(ptr, n, size)` | Wrap ggml_tensor data |
| `hades_tensor_free(wrapper)` | Free wrapper (not data) |
| `hades_tensor_get_data(wrapper)` | Get data pointer |
| `hades_tensor_get_size(wrapper)` | Get size in bytes |

### Paging Hooks

| Function | Description |
|----------|-------------|
| `hades_paging_hook(id, first, last)` | Call before layer compute |
| `hades_get_adaptive_mode()` | Get mode (0=8GB, 1=192GB) |

## Adaptive Infrastructure

The bridge automatically detects the deployment environment:

| Mode | Detection | Behavior |
|------|-----------|----------|
| **8GB Local** | `HADES_CLOUD_BURST` not set | Active paging, LRU eviction, thermal throttling |
| **192GB Cloud** | `HADES_CLOUD_BURST=1` or `HADES_MODE=cloud` | All layers resident, parallel experts, no paging |

Override with environment variables:
```bash
# Force 8GB mode
export HADES_MODE=local

# Force cloud mode
export HADES_MODE=cloud
export HADES_CLOUD_BURST=1
```

## Integration with llama.cpp

### Patch Point 1: ggml-backend.cpp

Add to `ggml_backend_tensor_compute`:

```cpp
void ggml_backend_tensor_compute(ggml_tensor* tensor) {
    // HADES pre-compute hook
    ggml_backend_hades_tensor_compute_pre(tensor);
    
    // Original compute logic
    ...
    
    // HADES post-compute hook
    ggml_backend_hades_tensor_compute_post(tensor);
}
```

### Patch Point 2: llama.cpp main

Add to `main()`:

```cpp
int main(int argc, char** argv) {
    // Initialize HADES after parsing args
    ggml_backend_hades_init(params.model.c_str());
    ggml_backend_hades_set_layers(model.n_layers);
    
    // ... run inference ...
    
    // Cleanup
    ggml_backend_hades_shutdown();
    return 0;
}
```

## Performance Characteristics

| Metric | 8GB Local | 192GB Cloud |
|--------|-----------|-------------|
| VRAM Usage | 6GB (paging) | 100GB+ (all resident) |
| Layer Load | On-demand | Preloaded |
| Throttle | Active @ 72°C | Disabled |
| Batch Size | 1 | 8+ |
| Context | 4K | 32K+ |

## Safety Guarantees

- **Memory Safe**: All FFI uses `NonNull` and explicit lifetimes
- **Zero-Copy**: Tensor data never duplicated
- **Thread Safe**: Atomic state, Send+Sync wrappers
- **No UB**: Validated pointers, bounds checks

## Files

```
hades-bridge/
├── Cargo.toml          # Rust package manifest
├── build.rs            # Build script
├── README.md           # This file
├── src/
│   ├── lib.rs          # Library root
│   ├── ffi.rs          # C-compatible exports
│   ├── tensor.rs       # Zero-copy tensor wrappers
│   └── backend.rs      # ggml backend integration
├── include/
│   └── hades-bridge.h  # C header for C++
└── cpp/
    └── ggml-hades.cpp  # C++ integration layer
```

## License

Part of HADES-KORTEX. Sovereign systems for hardware-constrained AI inference.
