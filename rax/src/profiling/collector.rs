//! Profiler singleton and data collection logic.

use std::cell::Cell;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use super::memory;
use super::stats::{OpcodeKey, OpcodeStats};

// Thread-local storage for the current instruction's OpcodeKey.
// This allows dispatch functions to set the precise opcode key (including
// two-byte, three-byte, VEX, EVEX variants) which is then read by end_instruction.
thread_local! {
    static CURRENT_OPCODE_KEY: Cell<Option<OpcodeKey>> = const { Cell::new(None) };
}

/// Global profiler instance.
static PROFILER: OnceLock<Mutex<Profiler>> = OnceLock::new();

/// Flag indicating profiling is active (for fast path check).
static PROFILING_ENABLED: AtomicBool = AtomicBool::new(false);

/// Decode cache hit counter.
static CACHE_HITS: AtomicU64 = AtomicU64::new(0);

/// Decode cache miss counter.
static CACHE_MISSES: AtomicU64 = AtomicU64::new(0);

/// Total instructions profiled.
static TOTAL_INSTRUCTIONS: AtomicU64 = AtomicU64::new(0);

/// Last instruction count when live stats were output.
static LAST_LIVE_OUTPUT: AtomicU64 = AtomicU64::new(0);

/// Configuration for the profiling system.
#[derive(Clone)]
pub struct ProfilingConfig {
    /// Enable memory access tracking (adds slight overhead).
    pub track_memory: bool,
    /// Interval for live stats output (number of instructions, 0 = disabled).
    pub live_interval: u64,
    /// Path for JSON export (None = no export).
    pub json_path: Option<PathBuf>,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            track_memory: true,
            live_interval: 10_000_000, // Every 10M instructions
            json_path: None,
        }
    }
}

/// The profiler collects per-opcode statistics.
pub struct Profiler {
    /// Per-opcode statistics array, indexed by OpcodeKey::to_index().
    stats: Box<[OpcodeStats; OpcodeKey::MAX_INDEX]>,
    /// Start time for IPS calculation.
    start_time: Instant,
    /// Configuration.
    config: ProfilingConfig,
}

impl Profiler {
    /// Create a new profiler with the given configuration.
    fn new(config: ProfilingConfig) -> Self {
        // Use a Vec and convert to boxed array to avoid stack overflow
        let stats_vec: Vec<OpcodeStats> = (0..OpcodeKey::MAX_INDEX)
            .map(|_| OpcodeStats::default())
            .collect();
        let stats: Box<[OpcodeStats; OpcodeKey::MAX_INDEX]> = stats_vec
            .into_boxed_slice()
            .try_into()
            .unwrap_or_else(|_| panic!("stats array size mismatch"));

        Self {
            stats,
            start_time: Instant::now(),
            config,
        }
    }

    /// Get a reference to the stats array.
    pub fn stats(&self) -> &[OpcodeStats; OpcodeKey::MAX_INDEX] {
        &self.stats
    }

    /// Get the elapsed time since profiling started.
    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Get the configuration.
    pub fn config(&self) -> &ProfilingConfig {
        &self.config
    }
}

/// Initialize the profiling system with the given configuration.
pub fn init(config: ProfilingConfig) {
    let profiler = Profiler::new(config);
    if PROFILER.set(Mutex::new(profiler)).is_err() {
        eprintln!("[profiling] Warning: profiler already initialized");
        return;
    }
    PROFILING_ENABLED.store(true, Ordering::Release);
    eprintln!("[profiling] Instruction profiling enabled");
}

/// Check if profiling is enabled (fast path).
#[inline(always)]
pub fn is_enabled() -> bool {
    PROFILING_ENABLED.load(Ordering::Relaxed)
}

/// Set the current instruction's OpcodeKey for profiling.
/// Call this from dispatch functions that know the precise opcode type.
#[inline(always)]
pub fn set_current_opcode_key(key: OpcodeKey) {
    CURRENT_OPCODE_KEY.set(Some(key));
}

/// Take the current OpcodeKey, clearing it for the next instruction.
#[inline(always)]
pub fn take_current_opcode_key() -> Option<OpcodeKey> {
    CURRENT_OPCODE_KEY.take()
}

/// Shutdown the profiler and export results if configured.
pub fn shutdown() {
    if !is_enabled() {
        return;
    }

    PROFILING_ENABLED.store(false, Ordering::Release);

    if let Some(profiler) = PROFILER.get() {
        let profiler = profiler.lock().unwrap();
        let json_path = profiler.config.json_path.clone();

        // Print final summary
        super::reporter::print_summary(&profiler);

        // Export JSON if configured
        if let Some(path) = json_path {
            drop(profiler); // Release lock before export
            if let Err(e) = super::reporter::export_json(&path) {
                eprintln!("[profiling] Error exporting JSON: {}", e);
            } else {
                eprintln!("[profiling] JSON report exported to: {}", path.display());
            }
        }
    }
}

/// Record a decode cache hit.
#[inline(always)]
pub fn record_cache_hit() {
    CACHE_HITS.fetch_add(1, Ordering::Relaxed);
}

/// Record a decode cache miss.
#[inline(always)]
pub fn record_cache_miss() {
    CACHE_MISSES.fetch_add(1, Ordering::Relaxed);
}

/// Get decode cache statistics.
pub fn cache_stats() -> (u64, u64) {
    (
        CACHE_HITS.load(Ordering::Relaxed),
        CACHE_MISSES.load(Ordering::Relaxed),
    )
}

/// Get total instructions profiled.
pub fn total_instructions() -> u64 {
    TOTAL_INSTRUCTIONS.load(Ordering::Relaxed)
}

/// Record the end of an instruction execution.
#[inline]
pub fn end_instruction(key: OpcodeKey, start: Instant) {
    let elapsed = start.elapsed().as_nanos() as u64;

    // Get memory stats (resets thread-local counters)
    let mem_stats = memory::take_stats();

    // Update statistics
    if let Some(profiler) = PROFILER.get() {
        if let Ok(mut profiler) = profiler.lock() {
            let idx = key.to_index();
            if idx < OpcodeKey::MAX_INDEX {
                profiler.stats[idx].record(elapsed, &mem_stats);
            }
        }
    }

    // Update total count
    let total = TOTAL_INSTRUCTIONS.fetch_add(1, Ordering::Relaxed) + 1;

    // Check if we should output live stats
    if let Some(profiler) = PROFILER.get() {
        if let Ok(profiler) = profiler.lock() {
            let interval = profiler.config.live_interval;
            if interval > 0 {
                let last = LAST_LIVE_OUTPUT.load(Ordering::Relaxed);
                if total - last >= interval {
                    LAST_LIVE_OUTPUT.store(total, Ordering::Relaxed);
                    let json_path = profiler.config.json_path.clone();
                    drop(profiler); // Release lock before printing
                    super::reporter::print_live_stats();
                    // Also export JSON if configured
                    if let Some(path) = json_path {
                        if let Err(e) = super::reporter::export_json(&path) {
                            eprintln!("[profiling] Error exporting JSON: {}", e);
                        }
                    }
                }
            }
        }
    }
}

/// Check if live stats should be output (for integration in run loop).
pub fn should_output_live(total_insns: u64) -> bool {
    if !is_enabled() {
        return false;
    }

    if let Some(profiler) = PROFILER.get() {
        if let Ok(profiler) = profiler.lock() {
            let interval = profiler.config.live_interval;
            if interval > 0 {
                let last = LAST_LIVE_OUTPUT.load(Ordering::Relaxed);
                return total_insns - last >= interval;
            }
        }
    }
    false
}

/// Access the profiler for reporting (internal use).
pub(super) fn with_profiler<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&Profiler) -> R,
{
    PROFILER.get().and_then(|p| p.lock().ok().map(|p| f(&p)))
}
