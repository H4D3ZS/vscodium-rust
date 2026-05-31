/**
 * @file hades-bridge.h
 * @brief HADES Bridge FFI Header for llama.cpp Integration
 * 
 * This header provides C-compatible FFI bindings for integrating
 * the HADES Kernel with llama.cpp's ggml backend system.
 * 
 * ## Usage
 * 
 * 1. Include this header in ggml-backend.cpp or llama.cpp files
 * 2. Call hades_bridge_init() at program startup
 * 3. Call hades_paging_hook() in the decode loop before each layer
 * 4. Call hades_bridge_shutdown() at program termination
 * 
 * ## Example
 * 
 * ```cpp
 * #include "hades-bridge.h"
 * 
 * int main() {
 *     // Initialize HADES bridge
 *     if (!hades_bridge_init()) {
 *         fprintf(stderr, "Failed to initialize HADES bridge\n");
 *         return 1;
 *     }
 * 
 *     // Check adaptive mode
 *     if (hades_is_local_8gb()) {
 *         printf("Running in 8GB local mode - active paging enabled\n");
 *     } else {
 *         printf("Running in 192GB cloud mode - parallel experts enabled\n");
 *     }
 * 
 *     // ... run inference ...
 * 
 *     hades_bridge_shutdown();
 *     return 0;
 * }
 * ```
 */

#ifndef HADES_BRIDGE_H
#define HADES_BRIDGE_H

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// =============================================================================
// Version
// =============================================================================

#define HADES_BRIDGE_VERSION "0.1.0"

// =============================================================================
// Initialization
// =============================================================================

/**
 * Initialize the HADES bridge
 * 
 * Must be called once before any other HADES functions.
 * Call from a single thread only.
 * 
 * @return true on success, false on failure
 */
bool hades_bridge_init(void);

/**
 * Shutdown the HADES bridge
 * 
 * Call once at program termination.
 * No other FFI functions should be called after this.
 */
void hades_bridge_shutdown(void);

/**
 * Check if bridge is initialized
 * 
 * @return true if initialized, false otherwise
 */
bool hades_bridge_is_initialized(void);

// =============================================================================
// VRAM Detection
// =============================================================================

/**
 * Get total VRAM in bytes
 * 
 * @return Total VRAM bytes (e.g., 8GB = 8589934592)
 */
uint64_t hades_get_total_vram(void);

/**
 * Get usable VRAM in bytes (after system overhead)
 * 
 * @return Usable VRAM bytes (e.g., 8GB GPU = ~6GB usable)
 */
uint64_t hades_get_usable_vram(void);

/**
 * Check if running in cloud burst mode (AMD MI300X 192GB)
 * 
 * @return true if cloud mode, false if local
 */
bool hades_is_cloud_burst(void);

/**
 * Check if running on local 8GB GPU (AMD RX 580)
 * 
 * @return true if local 8GB mode
 */
bool hades_is_local_8gb(void);

// =============================================================================
// Thermal Governor
// =============================================================================

/**
 * Get current throttle ratio
 * 
 * Returns a value in range [0.0, 1.0] where:
 * - 1.0 = full speed (no throttling)
 * - 0.5 = 50% throttle
 * - 0.0 = emergency stop
 * 
 * @return Throttle ratio
 */
float hades_check_throttle(void);

/**
 * Get current GPU temperature in Celsius
 * 
 * @return Temperature in °C (0.0 if not available)
 */
float hades_get_gpu_temp(void);

/**
 * Get current GPU power draw in Watts
 * 
 * @return Power in Watts (0.0 if not available)
 */
float hades_get_gpu_power(void);

/**
 * Apply thermal back-pressure (blocking delay)
 * 
 * Blocks current thread based on thermal throttle ratio.
 * Call from async context only.
 */
void hades_apply_back_pressure(void);

// =============================================================================
// Weight Streaming
// =============================================================================

/**
 * Initialize weight streamer for a GGUF model
 * 
 * @param model_path Path to GGUF model file (null-terminated string)
 * @return true on success, false on failure
 */
bool hades_init_weight_streamer(const char* model_path);

/**
 * Prefetch a model layer into VRAM
 * 
 * @param layer_id Layer index (0 = embeddings, N-1 = LM head)
 * @return true on success, false on failure
 */
bool hades_page_layer(size_t layer_id);

/**
 * Evict a layer from VRAM
 * 
 * @param layer_id Layer index to evict
 * @return true on success, false on failure
 */
bool hades_evict_layer(size_t layer_id);

/**
 * Get current VRAM usage
 * 
 * @param used Output: bytes currently used
 * @param total Output: total usable bytes
 */
void hades_get_vram_usage(uint64_t* used, uint64_t* total);

// =============================================================================
// Tensor FFI (Zero-Copy)
// =============================================================================

/**
 * Wrap a ggml_tensor data pointer for zero-copy access
 * 
 * @param data_ptr Raw pointer from ggml_tensor->data
 * @param nelements Total number of elements
 * @param element_size Size per element in bytes (4 for f32, 2 for f16)
 * @return Opaque handle, or NULL on failure
 */
void* hades_tensor_wrap(void* data_ptr, size_t nelements, size_t element_size);

/**
 * Free a tensor wrapper (does NOT free underlying data)
 * 
 * @param wrapper Handle from hades_tensor_wrap
 */
void hades_tensor_free(void* wrapper);

/**
 * Get data pointer from tensor wrapper
 * 
 * @param wrapper Tensor wrapper handle
 * @return Raw data pointer (same as original ggml_tensor->data)
 */
const void* hades_tensor_get_data(void* wrapper);

/**
 * Get tensor byte size
 * 
 * @param wrapper Tensor wrapper handle
 * @return Size in bytes
 */
size_t hades_tensor_get_size(void* wrapper);

// =============================================================================
// Backend Integration (Paging Hooks)
// =============================================================================

/**
 * Create HADES backend for ggml integration
 * 
 * @return Opaque backend handle, or NULL on failure
 */
void* hades_create_backend(void);

/**
 * Free HADES backend
 * 
 * @param backend Handle from hades_create_backend
 */
void hades_free_backend(void* backend);

/**
 * Get adaptive mode
 * 
 * @return 0 = 8GB local mode, 1 = 192GB cloud mode
 */
uint32_t hades_get_adaptive_mode(void);

/**
 * Paging hook for llama.cpp decode loop
 * 
 * Call this function before computing each layer in the decode loop.
 * It triggers predictive prefetch of subsequent layers.
 * 
 * @param layer_id Current layer index
 * @param is_first_layer true if layer 0
 * @param is_last_layer true if final layer
 * @return 0 on success, -1 on failure
 * 
 * Example usage in ggml-backend.cpp:
 * ```cpp
 * for (int i = 0; i < n_layers; i++) {
 *     // HADES paging hook
 *     hades_paging_hook(i, i == 0, i == n_layers - 1);
 *     
 *     // Compute layer...
 *     ggml_compute_forward(...);
 * }
 * ```
 */
int hades_paging_hook(size_t layer_id, bool is_first_layer, bool is_last_layer);

// =============================================================================
// Convenience Macros
// =============================================================================

/**
 * Check if HADES is active (initialized and in 8GB mode)
 */
#define HADES_IS_ACTIVE() (hades_bridge_is_initialized() && hades_is_local_8gb())

/**
 * Apply thermal back-pressure only if HADES is active
 */
#define HADES_MAYBE_THROTTLE() \
    do { \
        if (HADES_IS_ACTIVE()) { \
            hades_apply_back_pressure(); \
        } \
    } while (0)

/**
 * Prefetch next layer with bounds checking
 */
#define HADES_PREFETCH_NEXT(current, total) \
    do { \
        if (HADES_IS_ACTIVE() && (current) + 1 < (total)) { \
            hades_page_layer((current) + 1); \
        } \
    } while (0)

#ifdef __cplusplus
}
#endif

#endif /* HADES_BRIDGE_H */
