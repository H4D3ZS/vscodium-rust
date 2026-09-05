//! Memory budget system with circuit breaker for the agent loop.
//!
//! The root cause of OOM crashes is the agent loop having no awareness of its
//! own memory usage. It runs tools that read entire files, capture stdout/stderr,
//! and accumulate state — all without checking if there's enough headroom.
//!
//! This module provides:
//! - `MemoryBudget`: tracks RSS and enforces per-subsystem limits
//! - `CircuitBreaker`: monitors health and stops the loop before OOM
//! - Integration into the autonomous loop via `check_budget()`

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Memory budget configuration.
pub struct BudgetConfig {
    /// Maximum RSS in bytes before the circuit breaker trips.
    pub max_rss_bytes: u64,
    /// Minimum free headroom in bytes. If RSS > (max - headroom), stop.
    pub min_headroom_bytes: u64,
    /// Maximum bytes a single tool can allocate (file reads, buffers).
    pub max_tool_alloc_bytes: u64,
    /// Maximum total bytes the agent loop can allocate per iteration.
    pub max_per_iteration_bytes: u64,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            // 6GB limit — conservative for 8GB machines
            max_rss_bytes: 6 * 1024 * 1024 * 1024,
            // 500MB headroom
            min_headroom_bytes: 500 * 1024 * 1024,
            // 2MB per tool call
            max_tool_alloc_bytes: 2 * 1024 * 1024,
            // 10MB per iteration
            max_per_iteration_bytes: 10 * 1024 * 1024,
        }
    }
}

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    /// Normal operation.
    Closed,
    /// Memory pressure detected — reduce allocation.
    HalfOpen,
    /// Memory limit exceeded — stop the loop.
    Open,
}

/// Memory budget tracker with circuit breaker.
pub struct MemoryBudget {
    config: BudgetConfig,
    /// Current RSS in bytes (updated periodically).
    current_rss: AtomicU64,
    /// Bytes allocated in the current iteration.
    iteration_alloc: AtomicU64,
    /// Total bytes allocated since last reset.
    total_alloc: AtomicU64,
    /// Number of times the circuit breaker has tripped.
    trip_count: AtomicU64,
    /// Current circuit state.
    state: std::sync::atomic::AtomicU8, // CircuitState as u8
}

impl MemoryBudget {
    pub fn new(config: BudgetConfig) -> Arc<Self> {
        Arc::new(Self {
            config,
            current_rss: AtomicU64::new(0),
            iteration_alloc: AtomicU64::new(0),
            total_alloc: AtomicU64::new(0),
            trip_count: AtomicU64::new(0),
            state: std::sync::atomic::AtomicU8::new(CircuitState::Closed as u8),
        })
    }

    /// Update the current RSS reading. Call this periodically (e.g. before each iteration).
    pub fn update_rss(&self) {
        let rss = read_rss_bytes();
        self.current_rss.store(rss, Ordering::Relaxed);

        // Check circuit breaker
        let max = self.config.max_rss_bytes;
        let headroom = self.config.min_headroom_bytes;
        if rss + headroom >= max {
            self.state.store(CircuitState::Open as u8, Ordering::Relaxed);
            self.trip_count.fetch_add(1, Ordering::Relaxed);
        } else if rss + headroom * 2 >= max {
            self.state.store(CircuitState::HalfOpen as u8, Ordering::Relaxed);
        } else {
            self.state.store(CircuitState::Closed as u8, Ordering::Relaxed);
        }
    }

    /// Check if a tool allocation is within budget. Returns true if OK.
    pub fn check_tool_alloc(&self, bytes: u64) -> bool {
        let state = CircuitState::from(self.state.load(Ordering::Relaxed));
        match state {
            CircuitState::Open => false,
            CircuitState::HalfOpen => bytes <= self.config.max_tool_alloc_bytes / 2,
            CircuitState::Closed => bytes <= self.config.max_tool_alloc_bytes,
        }
    }

    /// Record bytes allocated in the current iteration.
    pub fn record_alloc(&self, bytes: u64) {
        self.iteration_alloc.fetch_add(bytes, Ordering::Relaxed);
        self.total_alloc.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Reset the per-iteration counter. Call at the start of each iteration.
    pub fn reset_iteration(&self) {
        self.iteration_alloc.store(0, Ordering::Relaxed);
    }

    /// Check if the circuit breaker allows continuing.
    pub fn should_continue(&self) -> bool {
        let state = CircuitState::from(self.state.load(Ordering::Relaxed));
        match state {
            CircuitState::Open => false,
            CircuitState::HalfOpen => {
                // Allow one more iteration with reduced budget
                self.iteration_alloc.load(Ordering::Relaxed) < self.config.max_per_iteration_bytes / 4
            }
            CircuitState::Closed => {
                self.iteration_alloc.load(Ordering::Relaxed) < self.config.max_per_iteration_bytes
            }
        }
    }

    /// Get the current circuit state.
    pub fn state(&self) -> CircuitState {
        CircuitState::from(self.state.load(Ordering::Relaxed))
    }

    /// Get current RSS in bytes.
    pub fn current_rss(&self) -> u64 {
        self.current_rss.load(Ordering::Relaxed)
    }

    /// Get total bytes allocated since last reset.
    pub fn total_alloc(&self) -> u64 {
        self.total_alloc.load(Ordering::Relaxed)
    }

    /// Get trip count.
    pub fn trip_count(&self) -> u64 {
        self.trip_count.load(Ordering::Relaxed)
    }
}

impl CircuitState {
    fn from(v: u8) -> Self {
        match v {
            0 => CircuitState::Closed,
            1 => CircuitState::HalfOpen,
            _ => CircuitState::Open,
        }
    }
}

/// Read current RSS in bytes. Platform-specific.
fn read_rss_bytes() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(statm) = std::fs::read_to_string("/proc/self/statm") {
            if let Some(rss_pages) = statm.split_whitespace().nth(1) {
                if let Ok(pages) = rss_pages.parse::<u64>() {
                    return pages * 4096; // assume 4KB pages
                }
            }
        }
    }
    // On macOS/Windows without native crates, return 0.
    // The circuit breaker still works via allocation tracking (record_alloc/should_continue).
    // For production, add `libc` (macOS) or `windows-sys` (Windows) crate.
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker() {
        let budget = MemoryBudget::new(BudgetConfig {
            max_rss_bytes: 1000,
            min_headroom_bytes: 100,
            ..Default::default()
        });

        // Initially closed
        assert_eq!(budget.state(), CircuitState::Closed);
        assert!(budget.should_continue());

        // Simulate high RSS
        budget.current_rss.store(950, Ordering::Relaxed);
        budget.state.store(CircuitState::HalfOpen as u8, Ordering::Relaxed);
        assert!(budget.should_continue()); // still allows with reduced budget

        // Simulate over limit
        budget.state.store(CircuitState::Open as u8, Ordering::Relaxed);
        assert!(!budget.should_continue());
    }

    #[test]
    fn test_tool_alloc_check() {
        let budget = MemoryBudget::new(BudgetConfig {
            max_tool_alloc_bytes: 1000,
            ..Default::default()
        });

        assert!(budget.check_tool_alloc(500));
        assert!(!budget.check_tool_alloc(1500));
    }
}
