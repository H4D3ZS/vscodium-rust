//! C-Compatible FFI Exports for llama.cpp Integration
//!
//! This module exports `#[no_mangle] extern "C"` functions for direct use from C++.
//! All functions are designed for zero-copy pointer exchange and memory safety.

use hades_kernel::{ThermalGovernor, ThermalPolicy, WeightStreamer};
use std::ffi::c_char;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use tracing::{error, info, warn};

use crate::tensor::GgmlTensorWrapper;
use crate::backend::HadesBackend;
use crate::VRAM_MI300X;

// ============================================================================
// Global State
// ============================================================================

static mut HADES_RUNTIME: Option<Arc<Runtime>> = None;
static HADES_BACKEND: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static THERMAL_GOVERNOR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static WEIGHT_STREAMER: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static INITIALIZED: AtomicBool = AtomicBool::new(false);

// VRAM detection
static TOTAL_VRAM: AtomicU64 = AtomicU64::new(0);
static USABLE_VRAM: AtomicU64 = AtomicU64::new(0);
static IS_CLOUD_BURST: AtomicBool = AtomicBool::new(false);

// ============================================================================
// Initialization
// ============================================================================

/// Initialize the HADES bridge
///
/// # Safety
/// Must be called once before any other FFI functions.
/// Call from a single thread only.
#[no_mangle]
pub unsafe extern "C" fn hades_bridge_init() -> bool {
    if INITIALIZED.load(Ordering::SeqCst) {
        warn!("HADES bridge already initialized");
        return true;
    }

    // Initialize logging
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("hades_bridge=info".parse().unwrap())
                .add_directive("hades_kernel=info".parse().unwrap()),
        )
        .try_init();

    info!("Initializing HADES Bridge v{}", crate::VERSION);

    // Create Tokio runtime
    match Runtime::new() {
        Ok(runtime) => {
            HADES_RUNTIME = Some(Arc::new(runtime));
        }
        Err(e) => {
            error!("Failed to create Tokio runtime: {}", e);
            return false;
        }
    }

    // Detect VRAM
    let (total, usable, is_cloud) = detect_vram_config();
    TOTAL_VRAM.store(total, Ordering::SeqCst);
    USABLE_VRAM.store(usable, Ordering::SeqCst);
    IS_CLOUD_BURST.store(is_cloud, Ordering::SeqCst);

    info!(
        "VRAM configuration: {} total, {} usable, cloud_burst={}",
        total / (1024 * 1024 * 1024),
        usable / (1024 * 1024 * 1024),
        is_cloud
    );

    // Initialize thermal governor
    let policy = ThermalPolicy {
        throttle_temp_c: 72.0,
        critical_temp_c: 80.0,
        throttle_power_w: 150.0,
        sample_interval: Duration::from_millis(500),
        throttle_ratio: 0.5,
    };

    match ThermalGovernor::with_policy(policy) {
        Ok(governor) => {
            let boxed = Box::new(governor);
            let ptr = Box::into_raw(boxed) as *mut c_void;
            THERMAL_GOVERNOR.store(ptr, Ordering::SeqCst);
        }
        Err(e) => {
            error!("Failed to create thermal governor: {}", e);
            return false;
        }
    }

    INITIALIZED.store(true, Ordering::SeqCst);
    info!("HADES bridge initialized successfully");
    true
}

/// Shutdown the HADES bridge
///
/// # Safety
/// Call once at program termination. No other FFI functions should be called after.
#[no_mangle]
pub unsafe extern "C" fn hades_bridge_shutdown() {
    if !INITIALIZED.load(Ordering::SeqCst) {
        return;
    }

    info!("Shutting down HADES bridge");

    // Free thermal governor
    let ptr = THERMAL_GOVERNOR.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !ptr.is_null() {
        let _ = Box::from_raw(ptr as *mut ThermalGovernor);
    }

    // Free weight streamer
    let ptr = WEIGHT_STREAMER.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !ptr.is_null() {
        let _ = Box::from_raw(ptr as *mut WeightStreamer);
    }

    // Free backend
    let ptr = HADES_BACKEND.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !ptr.is_null() {
        let _ = Box::from_raw(ptr as *mut HadesBackend);
    }

    // Drop runtime
    HADES_RUNTIME.take();

    INITIALIZED.store(false, Ordering::SeqCst);
}

/// Check if bridge is initialized
#[no_mangle]
pub extern "C" fn hades_bridge_is_initialized() -> bool {
    INITIALIZED.load(Ordering::SeqCst)
}

// ============================================================================
// VRAM Detection
// ============================================================================

/// Detect VRAM configuration
///
/// Returns: (total_vram_bytes, usable_vram_bytes, is_cloud_burst)
fn detect_vram_config() -> (u64, u64, bool) {
    // For now, detect based on environment variable
    // In production, query actual GPU via ROCm/NVAPI

    let cloud_burst = std::env::var("HADES_CLOUD_BURST")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    if cloud_burst {
        // AMD MI300X cloud instance
        (VRAM_MI300X, VRAM_MI300X - (16 * 1024 * 1024 * 1024), true)
    } else {
        // Local AMD RX 580 8GB
        let total = 8 * 1024 * 1024 * 1024;
        let usable = 6 * 1024 * 1024 * 1024; // 6.5GB minus overhead
        (total, usable, false)
    }
}

/// Get total VRAM in bytes
#[no_mangle]
pub extern "C" fn hades_get_total_vram() -> u64 {
    TOTAL_VRAM.load(Ordering::SeqCst)
}

/// Get usable VRAM in bytes
#[no_mangle]
pub extern "C" fn hades_get_usable_vram() -> u64 {
    USABLE_VRAM.load(Ordering::SeqCst)
}

/// Check if running in cloud burst mode (192GB MI300X)
#[no_mangle]
pub extern "C" fn hades_is_cloud_burst() -> bool {
    IS_CLOUD_BURST.load(Ordering::SeqCst)
}

/// Check if running on 8GB local GPU
#[no_mangle]
pub extern "C" fn hades_is_local_8gb() -> bool {
    !IS_CLOUD_BURST.load(Ordering::SeqCst)
}

// ============================================================================
// Thermal Governor FFI
// ============================================================================

/// Get current throttle ratio (0.0 to 1.0)
///
/// # Returns
/// Float in range [0.0, 1.0] where:
/// - 1.0 = full speed (no throttling)
/// - 0.5 = 50% throttle
/// - 0.0 = emergency stop
#[no_mangle]
pub extern "C" fn hades_check_throttle() -> f32 {
    unsafe {
        let ptr = THERMAL_GOVERNOR.load(Ordering::SeqCst);
        if ptr.is_null() {
            return 1.0; // Default to full speed if not initialized
        }

        let governor = &*(ptr as *const ThermalGovernor);
        governor.throttle_ratio()
    }
}

/// Get current GPU temperature in Celsius
#[no_mangle]
pub extern "C" fn hades_get_gpu_temp() -> f32 {
    unsafe {
        let ptr = THERMAL_GOVERNOR.load(Ordering::SeqCst);
        if ptr.is_null() {
            return 0.0;
        }

        let governor = &*(ptr as *const ThermalGovernor);
        governor.telemetry().temperature_c
    }
}

/// Get current GPU power draw in Watts
#[no_mangle]
pub extern "C" fn hades_get_gpu_power() -> f32 {
    unsafe {
        let ptr = THERMAL_GOVERNOR.load(Ordering::SeqCst);
        if ptr.is_null() {
            return 0.0;
        }

        let governor = &*(ptr as *const ThermalGovernor);
        governor.telemetry().power_watts
    }
}

/// Apply thermal back-pressure (blocking delay based on throttle ratio)
///
/// # Safety
/// Call from async context only. Will block current thread.
#[no_mangle]
pub extern "C" fn hades_apply_back_pressure() {
    unsafe {
        let ptr = THERMAL_GOVERNOR.load(Ordering::SeqCst);
        if ptr.is_null() {
            return;
        }

        let governor = &*(ptr as *const ThermalGovernor);
        let runtime = match &HADES_RUNTIME {
            Some(r) => r,
            None => return,
        };

        // Apply back-pressure based on current throttle ratio
        // Note: We don't sample here to avoid mutable borrow
        let throttle = governor.throttle_ratio();
        if throttle < 1.0 {
            let delay_ms = ((1.0 - throttle) * 50.0) as u64;
            if delay_ms > 0 {
                runtime.block_on(async {
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                });
            }
        }
    }
}

// ============================================================================
// Weight Streamer FFI
// ============================================================================

/// Initialize weight streamer for a GGUF model
///
/// # Parameters
/// - `model_path`: Null-terminated C string path to GGUF model file
///
/// # Returns
/// true on success, false on failure
///
/// # Safety
/// model_path must be a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn hades_init_weight_streamer(model_path: *const c_char) -> bool {
    if !INITIALIZED.load(Ordering::SeqCst) {
        error!("HADES bridge not initialized");
        return false;
    }

    let path_str = match std::ffi::CStr::from_ptr(model_path).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => {
            error!("Invalid model path");
            return false;
        }
    };

    let thermal_ptr = THERMAL_GOVERNOR.load(Ordering::SeqCst);
    if thermal_ptr.is_null() {
        error!("Thermal governor not initialized");
        return false;
    }

    let thermal = Arc::new((*(thermal_ptr as *const ThermalGovernor)).clone_for_ffi());

    let runtime = match &HADES_RUNTIME {
        Some(r) => r,
        None => {
            error!("Tokio runtime not available");
            return false;
        }
    };

    match runtime.block_on(WeightStreamer::new(path_str, thermal)) {
        Ok(streamer) => {
            let boxed = Box::new(streamer);
            let ptr = Box::into_raw(boxed) as *mut c_void;
            WEIGHT_STREAMER.store(ptr, Ordering::SeqCst);
            info!("Weight streamer initialized");
            true
        }
        Err(e) => {
            error!("Failed to create weight streamer: {}", e);
            false
        }
    }
}

/// Prefetch a model layer into VRAM
///
/// # Parameters
/// - `layer_id`: Layer index (0 = embeddings)
///
/// # Returns
/// true on success, false on failure
#[no_mangle]
pub extern "C" fn hades_page_layer(layer_id: usize) -> bool {
    unsafe {
        let ptr = WEIGHT_STREAMER.load(Ordering::SeqCst);
        if ptr.is_null() {
            warn!("Weight streamer not initialized");
            return false;
        }

        let streamer = &*(ptr as *const WeightStreamer);
        let runtime = match &HADES_RUNTIME {
            Some(r) => r,
            None => return false,
        };

        match runtime.block_on(streamer.prefetch(layer_id)) {
            Ok(_) => true,
            Err(e) => {
                error!("Failed to prefetch layer {}: {}", layer_id, e);
                false
            }
        }
    }
}

/// Evict a layer from VRAM
#[no_mangle]
pub extern "C" fn hades_evict_layer(layer_id: usize) -> bool {
    unsafe {
        let ptr = WEIGHT_STREAMER.load(Ordering::SeqCst);
        if ptr.is_null() {
            return false;
        }

        let streamer = &*(ptr as *const WeightStreamer);
        let runtime = match &HADES_RUNTIME {
            Some(r) => r,
            None => return false,
        };

        runtime.block_on(streamer.evict(layer_id)).is_ok()
    }
}

/// Get current VRAM usage (used, total) in bytes
#[no_mangle]
pub extern "C" fn hades_get_vram_usage(used: *mut u64, total: *mut u64) {
    unsafe {
        let ptr = WEIGHT_STREAMER.load(Ordering::SeqCst);
        if ptr.is_null() {
            if !used.is_null() {
                *used = 0;
            }
            if !total.is_null() {
                *total = USABLE_VRAM.load(Ordering::SeqCst);
            }
            return;
        }

        let streamer = &*(ptr as *const WeightStreamer);
        let runtime = match &HADES_RUNTIME {
            Some(r) => r,
            None => return,
        };

        let (u, t) = runtime.block_on(streamer.vram_usage());
        if !used.is_null() {
            *used = u as u64;
        }
        if !total.is_null() {
            *total = t as u64;
        }
    }
}

// ============================================================================
// Tensor FFI (Zero-Copy Pointer Exchange)
// ============================================================================

/// Wrap a ggml_tensor data pointer for zero-copy access
///
/// # Parameters
/// - `data_ptr`: Raw pointer to tensor data (from ggml_tensor->data)
/// - `nelements`: Total number of elements in tensor
/// - `element_size`: Size of each element in bytes (e.g., 4 for f32)
///
/// # Returns
/// Opaque handle to tensor wrapper, or null on failure
///
/// # Safety
/// Caller must ensure data_ptr remains valid for lifetime of wrapper.
/// Call hades_tensor_free when done.
#[no_mangle]
pub unsafe extern "C" fn hades_tensor_wrap(
    data_ptr: *mut c_void,
    nelements: usize,
    element_size: usize,
) -> *mut c_void {
    if data_ptr.is_null() || nelements == 0 || element_size == 0 {
        return std::ptr::null_mut();
    }

    let wrapper = GgmlTensorWrapper::wrap_raw(data_ptr, nelements, element_size);
    Box::into_raw(Box::new(wrapper)) as *mut c_void
}

/// Free a tensor wrapper (does NOT free underlying data)
///
/// # Safety
/// Call exactly once for each hades_tensor_wrap call
#[no_mangle]
pub unsafe extern "C" fn hades_tensor_free(wrapper: *mut c_void) {
    if !wrapper.is_null() {
        let _ = Box::from_raw(wrapper as *mut GgmlTensorWrapper);
    }
}

/// Get data pointer from tensor wrapper (for zero-copy access)
///
/// # Returns
/// Raw pointer to tensor data (same as original ggml_tensor->data)
#[no_mangle]
pub unsafe extern "C" fn hades_tensor_get_data(wrapper: *mut c_void) -> *const c_void {
    if wrapper.is_null() {
        return std::ptr::null();
    }

    let w = &*(wrapper as *mut GgmlTensorWrapper);
    w.data_ptr()
}

/// Get tensor byte size
#[no_mangle]
pub unsafe extern "C" fn hades_tensor_get_size(wrapper: *mut c_void) -> usize {
    if wrapper.is_null() {
        return 0;
    }

    let w = &*(wrapper as *mut GgmlTensorWrapper);
    w.size_bytes()
}

// ============================================================================
// Backend Integration
// ============================================================================

/// Create HADES backend for ggml integration
#[no_mangle]
pub extern "C" fn hades_create_backend() -> *mut c_void {
    if !INITIALIZED.load(Ordering::SeqCst) {
        return std::ptr::null_mut();
    }

    let backend = HadesBackend::new();
    let boxed = Box::new(backend);
    Box::into_raw(boxed) as *mut c_void
}

/// Free HADES backend
#[no_mangle]
pub unsafe extern "C" fn hades_free_backend(backend: *mut c_void) {
    if !backend.is_null() {
        let _ = Box::from_raw(backend as *mut HadesBackend);
    }
}

/// Get adaptive mode (0 = 8GB local, 1 = 192GB cloud)
#[no_mangle]
pub extern "C" fn hades_get_adaptive_mode() -> u32 {
    if IS_CLOUD_BURST.load(Ordering::SeqCst) {
        1 // Cloud burst mode
    } else {
        0 // Local 8GB mode
    }
}
