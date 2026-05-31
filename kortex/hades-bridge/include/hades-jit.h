/**
 * @file hades-jit.h
 * @brief HADES JIT Decompression Engine - C++/HIP Integration
 * 
 * This header provides C++ bindings for the Infinite-Fidelity JIT
 * Decompression engine, enabling mid-inference KV-cache injection
 * via HIP kernels.
 * 
 * ## Usage
 * 
 * 1. Include in llama.cpp or ggml-hip backend
 * 2. Initialize JIT engine at startup
 * 3. Monitor attention heads via hades_jit_record_activation()
 * 4. Inject inflated blocks via hades_jit_kv_cache_inject()
 */

#ifndef HADES_JIT_H
#define HADES_JIT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifdef __HIP__
#include <hip/hip_runtime.h>
#endif

// =============================================================================
// Configuration
// =============================================================================

/**
 * JIT decompression configuration
 */
typedef struct {
    /** Size of parametric gist in bytes (default: 6144 = 6KB) */
    size_t gist_size_bytes;
    
    /** Size of VRAM scratchpad (default: 536870912 = 512MB) */
    size_t scratchpad_size_bytes;
    
    /** VRAM empathy cap (default: 6979321856 = 6.5GB) */
    uint64_t vram_empathy_cap_bytes;
    
    /** Fault threshold (default: 0.85) */
    float fault_threshold;
    
    /** Thermal throttle temp (default: 72.0) */
    float thermal_throttle_temp_c;
    
    /** Enable io_uring (Linux only, default: true) */
    bool use_io_uring;
} hades_jit_config_t;

/**
 * Default configuration
 */
static inline hades_jit_config_t hades_jit_config_default() {
    return (hades_jit_config_t){
        .gist_size_bytes = 6144,
        .scratchpad_size_bytes = 512 * 1024 * 1024,
        .vram_empathy_cap_bytes = 6ULL * 1024 * 1024 * 1024 + 512 * 1024 * 1024,
        .fault_threshold = 0.85f,
        .thermal_throttle_temp_c = 72.0f,
        .use_io_uring = true,
    };
}

// =============================================================================
// Initialization
// =============================================================================

/**
 * Initialize JIT decompression engine
 * 
 * @param config Configuration (can be NULL for defaults)
 * @return 0 on success, -1 on failure
 */
int hades_jit_init(const hades_jit_config_t* config);

/**
 * Shutdown JIT decompression engine
 */
void hades_jit_shutdown(void);

/**
 * Check if JIT engine is initialized
 */
bool hades_jit_is_initialized(void);

// =============================================================================
// Attention Head Monitoring
// =============================================================================

/**
 * Attention activation sample
 */
typedef struct {
    /** Layer index */
    uint32_t layer;
    /** Head index within layer */
    uint32_t head;
    /** Cluster ID attending to */
    uint32_t cluster_id;
    /** Activation value (0.0-1.0) */
    float activation;
} hades_activation_sample_t;

/**
 * Record attention head activation
 * 
 * Call this from the HIP attention kernel for each head.
 * Triggers fault handler when activation >= 0.85.
 * 
 * @param sample Activation sample
 */
void hades_jit_record_activation(const hades_activation_sample_t* sample);

/**
 * Get most activated cluster
 * 
 * @return Cluster ID, or -1 if none activated
 */
int32_t hades_jit_get_most_activated_cluster(void);

/**
 * Check if cluster needs inflation
 * 
 * @param cluster_id Cluster ID to check
 * @return true if activation >= threshold
 */
bool hades_jit_cluster_needs_inflation(uint32_t cluster_id);

// =============================================================================
// Inflation Engine
// =============================================================================

/**
 * Inflation result
 */
typedef struct {
    /** Cluster ID inflated */
    uint32_t cluster_id;
    /** Bytes inflated from SSD */
    size_t bytes_inflated;
    /** VRAM offset */
    size_t vram_offset;
    /** Latency in microseconds */
    uint64_t latency_us;
    /** Was thermal throttle applied? */
    bool thermal_throttled;
} hades_inflation_result_t;

/**
 * Inflate a code block from SSD to VRAM
 * 
 * @param cluster_id Cluster to inflate
 * @param file_path Path to source file
 * @param result Output result (can be NULL)
 * @return 0 on success, -1 on failure
 */
int hades_jit_inflate(uint32_t cluster_id, const char* file_path, 
                      hades_inflation_result_t* result);

/**
 * Get scratchpad usage
 * 
 * @return Bytes used in scratchpad
 */
size_t hades_jit_scratchpad_usage(void);

/**
 * Get scratchpad available space
 * 
 * @return Bytes available
 */
size_t hades_jit_scratchpad_available(void);

// =============================================================================
// KV-Cache Injection (HIP Integration)
// =============================================================================

/**
 * Neural pointer - links inflated block to gist
 */
typedef struct {
    /** VRAM address of inflated block */
    uint64_t block_ptr;
    /** Gist centroid (1536 floats) */
    float gist_centroid[1536];
    /** Cluster ID */
    uint32_t cluster_id;
    /** Offset in scratchpad */
    size_t offset;
    /** Block size */
    size_t size;
    /** ML-DSA seal hash (32 bytes) */
    uint8_t seal_hash[32];
} hades_neural_pointer_t;

/**
 * Create neural pointer
 * 
 * @param block_ptr VRAM address of inflated block
 * @param gist_ptr Pointer to 6KB gist (1536 floats)
 * @param cluster_id Cluster ID
 * @param offset Offset in scratchpad
 * @param size Block size
 * @param out Output pointer
 */
void hades_jit_create_neural_pointer(
    uint64_t block_ptr,
    const float* gist_ptr,
    uint32_t cluster_id,
    size_t offset,
    size_t size,
    hades_neural_pointer_t* out
);

/**
 * Inject inflated block into KV-cache
 * 
 * This is the main HIP integration point. Call from llama.cpp
 * after inflation to inject the block mid-inference.
 * 
 * @param pointer Neural pointer to inflated block
 * @param token_count Number of tokens in block
 * @param kv_cache KV-cache pointer (ggml_backend)
 * @param kv_position Current KV position
 * @return 0 on success, -1 on failure
 */
int hades_jit_kv_cache_inject(
    const hades_neural_pointer_t* pointer,
    uint32_t token_count,
    void* kv_cache,
    size_t kv_position
);

/**
 * HIP kernel for parallel token embedding injection
 * 
 * Launch this kernel to inject inflated tokens in parallel.
 * 
 * @param d_tokens Device pointer to token embeddings
 * @param d_kv_cache Device KV-cache pointer
 * @param token_count Number of tokens
 * @param kv_position KV-cache start position
 */
#ifdef __HIP__
__global__ void hades_jit_inject_tokens_kernel(
    const float* d_tokens,
    float* d_kv_cache,
    uint32_t token_count,
    size_t kv_position
);
#endif

// =============================================================================
// LRU Eviction
// =============================================================================

/**
 * Evicted block info
 */
typedef struct {
    /** Cluster ID evicted */
    uint32_t cluster_id;
    /** Bytes freed */
    size_t bytes_freed;
    /** Eviction reason (0=LRU, 1=Size, 2=Emergency, 3=Thermal) */
    uint32_t reason;
} hades_evicted_block_t;

/**
 * Perform LRU eviction
 * 
 * @param out Output evicted block info (can be NULL)
 * @return 0 if evicted, -1 if queue empty
 */
int hades_jit_evict_lru(hades_evicted_block_t* out);

/**
 * Emergency eviction to free target bytes
 * 
 * @param target_bytes Bytes to free
 * @return Total bytes freed
 */
size_t hades_jit_emergency_evict(size_t target_bytes);

/**
 * Get current VRAM usage (inflated blocks only)
 * 
 * @return Bytes used
 */
size_t hades_jit_vram_usage(void);

/**
 * Check if eviction is needed
 * 
 * @return true if VRAM usage > cap
 */
bool hades_jit_needs_eviction(void);

// =============================================================================
// Metrics & Status
// =============================================================================

/**
 * JIT engine metrics
 */
typedef struct {
    /** Total inflations */
    uint64_t total_inflations;
    /** Total bytes inflated */
    uint64_t total_bytes_inflated;
    /** Total evictions */
    uint64_t total_evictions;
    /** Total bytes evicted */
    uint64_t total_bytes_evicted;
    /** Thermal throttle events */
    uint64_t thermal_throttle_count;
    /** Average inflation latency (us) */
    double avg_inflation_latency_us;
    /** Current VRAM usage */
    uint64_t current_vram_usage_bytes;
    /** Gist hit rate (0.0-1.0) */
    double gist_hit_rate;
} hades_jit_metrics_t;

/**
 * Get JIT metrics
 * 
 * @param out Output metrics
 */
void hades_jit_get_metrics(hades_jit_metrics_t* out);

/**
 * Get engine status
 * 
 * @return Status string (idle, inflating, thermal_wait, evicting, error)
 */
const char* hades_jit_get_status(void);

// =============================================================================
// FFI Helpers
// =============================================================================

/**
 * Get 6KB gist pointer (VRAM resident)
 * 
 * @return Pointer to gist (1536 floats), or NULL if not initialized
 */
const float* hades_jit_get_gist_ptr(void);

/**
 * Seal inflated block with ML-DSA
 * 
 * @param data Block data
 * @param size Block size
 * @param hash Output hash (32 bytes)
 * @return 0 on success, -1 on failure
 */
int hades_jit_seal_block(const void* data, size_t size, uint8_t hash[32]);

/**
 * Verify ML-DSA seal
 * 
 * @param data Block data
 * @param size Block size
 * @param hash Expected hash
 * @return 0 if valid, -1 if invalid
 */
int hades_jit_verify_seal(const void* data, size_t size, const uint8_t hash[32]);

#ifdef __cplusplus
}
#endif

#endif /* HADES_JIT_H */
