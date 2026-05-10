/**
 * @file ggml-hades.cpp
 * @brief HADES Backend Integration for ggml
 * 
 * This file provides the integration layer between llama.cpp's ggml backend
 * system and the HADES Kernel weight streaming infrastructure.
 * 
 * ## Integration Points
 * 
 * 1. Include this file in your ggml-backend.cpp or llama.cpp build
 * 2. Call ggml_backend_hades_init() at startup
 * 3. The paging hooks are automatically called during layer compute
 * 
 * ## Build Instructions
 * 
 * Add to your CMakeLists.txt or Makefile:
 * ```cmake
 * target_link_libraries(ggml PRIVATE hades_bridge)
 * target_include_directories(ggml PRIVATE ${HADES_BRIDGE_INCLUDE_DIR})
 * ```
 */

#include "hades-bridge.h"
#include "ggml.h"
#include "ggml-backend.h"

#include <cstdio>
#include <cstdlib>
#include <cstring>

// =============================================================================
// Global State
// =============================================================================

static bool g_hades_initialized = false;
static void* g_hades_backend = nullptr;
static uint64_t g_total_layers = 0;

// =============================================================================
// Initialization
// =============================================================================

/**
 * Initialize HADES backend for ggml
 * 
 * Call this once at program startup, before any ggml operations.
 * 
 * @param model_path Path to GGUF model file (optional, can be NULL)
 * @return true on success, false on failure
 */
bool ggml_backend_hades_init(const char* model_path) {
    if (g_hades_initialized) {
        return true;
    }

    printf("[HADES] Initializing HADES bridge...\n");

    // Initialize bridge
    if (!hades_bridge_init()) {
        fprintf(stderr, "[HADES] Failed to initialize bridge\n");
        return false;
    }

    // Detect VRAM configuration
    uint64_t total_vram = hades_get_total_vram();
    uint64_t usable_vram = hades_get_usable_vram();
    
    printf("[HADES] VRAM: %.2f GB total, %.2f GB usable\n", 
           total_vram / (1024.0 * 1024.0 * 1024.0),
           usable_vram / (1024.0 * 1024.0 * 1024.0));

    if (hades_is_local_8gb()) {
        printf("[HADES] Running in LOCAL 8GB mode - Active Paging ENABLED\n");
    } else if (hades_is_cloud_burst()) {
        printf("[HADES] Running in CLOUD 192GB mode - Parallel Experts ENABLED\n");
    }

    // Initialize weight streamer if model path provided
    if (model_path != nullptr) {
        if (hades_init_weight_streamer(model_path)) {
            printf("[HADES] Weight streamer initialized for: %s\n", model_path);
        } else {
            fprintf(stderr, "[HADES] Failed to initialize weight streamer\n");
            // Non-fatal, continue without streaming
        }
    }

    // Create backend
    g_hades_backend = hades_create_backend();
    if (g_hades_backend == nullptr) {
        fprintf(stderr, "[HADES] Failed to create backend\n");
        hades_bridge_shutdown();
        return false;
    }

    g_hades_initialized = true;
    printf("[HADES] Initialization complete\n");
    return true;
}

/**
 * Shutdown HADES backend
 * 
 * Call at program termination.
 */
void ggml_backend_hades_shutdown() {
    if (!g_hades_initialized) {
        return;
    }

    printf("[HADES] Shutting down...\n");

    if (g_hades_backend != nullptr) {
        hades_free_backend(g_hades_backend);
        g_hades_backend = nullptr;
    }

    hades_bridge_shutdown();
    g_hades_initialized = false;
}

/**
 * Set total layer count for paging optimization
 * 
 * Call after loading model to enable better prefetching.
 * 
 * @param n_layers Total number of model layers
 */
void ggml_backend_hades_set_layers(uint64_t n_layers) {
    g_total_layers = n_layers;
    printf("[HADES] Model has %lu layers\n", (unsigned long)n_layers);
}

// =============================================================================
// Paging Hooks
// =============================================================================

/**
 * Pre-compute layer hook
 * 
 * Call before computing a layer to trigger prefetching.
 * This is the main integration point with llama.cpp's decode loop.
 * 
 * @param layer_id Current layer index
 */
void ggml_backend_hades_pre_compute(uint64_t layer_id) {
    if (!g_hades_initialized) {
        return;
    }

    // Apply thermal back-pressure
    float throttle = hades_check_throttle();
    if (throttle < 1.0f) {
        hades_apply_back_pressure();
    }

    // Prefetch next layers (in 8GB mode)
    if (hades_is_local_8gb()) {
        // Prefetch next 2 layers
        for (int i = 1; i <= 2; i++) {
            size_t next_layer = layer_id + i;
            if (next_layer < g_total_layers || g_total_layers == 0) {
                hades_page_layer(next_layer);
            }
        }
    }
}

/**
 * Post-compute layer hook
 * 
 * Call after computing a layer to trigger eviction.
 * 
 * @param layer_id Current layer index
 */
void ggml_backend_hades_post_compute(uint64_t layer_id) {
    if (!g_hades_initialized) {
        return;
    }

    // Evict old layers (in 8GB mode)
    if (hades_is_local_8gb() && layer_id >= 2) {
        size_t evict_layer = layer_id - 2;
        hades_evict_layer(evict_layer);
    }
}

// =============================================================================
// Tensor Wrappers
// =============================================================================

/**
 * Wrap a ggml_tensor for zero-copy HADES access
 * 
 * @param tensor Pointer to ggml_tensor
 * @return Opaque handle, or NULL on failure
 */
void* ggml_backend_hades_wrap_tensor(struct ggml_tensor* tensor) {
    if (tensor == nullptr || tensor->data == nullptr) {
        return nullptr;
    }

    size_t nelements = ggml_nelements(tensor);
    size_t element_size = ggml_element_size(tensor);

    return hades_tensor_wrap(tensor->data, nelements, element_size);
}

/**
 * Free tensor wrapper
 * 
 * @param wrapper Handle from ggml_backend_hades_wrap_tensor
 */
void ggml_backend_hades_free_tensor(void* wrapper) {
    if (wrapper != nullptr) {
        hades_tensor_free(wrapper);
    }
}

// =============================================================================
// Integration with ggml-backend.cpp
// =============================================================================

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Patch point for ggml_backend_tensor_compute
 * 
 * Include this in your ggml-backend.cpp tensor compute function:
 * 
 * ```cpp
 * void ggml_backend_tensor_compute(ggml_tensor* tensor) {
 *     // HADES pre-compute hook
 *     ggml_backend_hades_pre_compute(tensor->id);
 *     
 *     // Original compute...
 *     ...
 *     
 *     // HADES post-compute hook
 *     ggml_backend_hades_post_compute(tensor->id);
 * }
 * ```
 */
void ggml_backend_hades_tensor_compute_pre(struct ggml_tensor* tensor) {
    if (tensor == nullptr) {
        return;
    }
    
    // Extract layer ID from tensor (depends on llama.cpp internals)
    // This is a placeholder - actual implementation depends on tensor naming
    uint64_t layer_id = 0;
    
    // Try to extract layer from tensor name or ID
    if (tensor->name != nullptr) {
        // Parse "blk.N.*" pattern
        if (strncmp(tensor->name, "blk.", 4) == 0) {
            layer_id = strtoul(tensor->name + 4, nullptr, 10);
        }
    }
    
    ggml_backend_hades_pre_compute(layer_id);
}

void ggml_backend_hades_tensor_compute_post(struct ggml_tensor* tensor) {
    if (tensor == nullptr) {
        return;
    }
    
    uint64_t layer_id = 0;
    
    if (tensor->name != nullptr) {
        if (strncmp(tensor->name, "blk.", 4) == 0) {
            layer_id = strtoul(tensor->name + 4, nullptr, 10);
        }
    }
    
    ggml_backend_hades_post_compute(layer_id);
}

#ifdef __cplusplus
}
#endif

// =============================================================================
// C++ RAII Wrapper
// =============================================================================

#ifdef __cplusplus
namespace ggml {
namespace backend {
namespace hades {

/**
 * RAII initializer for HADES backend
 * 
 * Usage:
 * ```cpp
 * int main() {
 *     ggml::backend::hades::Init hades_init("/path/to/model.gguf");
 *     if (!hades_init) {
 *         return 1;
 *     }
 *     // HADES automatically shuts down when this goes out of scope
 *     ...
 * }
 * ```
 */
class Init {
public:
    explicit Init(const char* model_path = nullptr) 
        : success_(ggml_backend_hades_init(model_path)) {}
    
    ~Init() {
        ggml_backend_hades_shutdown();
    }
    
    explicit operator bool() const { return success_; }
    
    bool success() const { return success_; }
    
private:
    bool success_;
};

/**
 * Layer compute guard for automatic paging hooks
 * 
 * Usage in decode loop:
 * ```cpp
 * for (int i = 0; i < n_layers; i++) {
 *     ggml::backend::hades::LayerGuard guard(i);
 *     // Compute layer...
 *     ggml_compute_forward(...);
 * }
 * ```
 */
class LayerGuard {
public:
    explicit LayerGuard(uint64_t layer_id) : layer_id_(layer_id) {
        ggml_backend_hades_pre_compute(layer_id_);
    }
    
    ~LayerGuard() {
        ggml_backend_hades_post_compute(layer_id_);
    }
    
private:
    uint64_t layer_id_;
};

} // namespace hades
} // namespace backend
} // namespace ggml
#endif // __cplusplus
