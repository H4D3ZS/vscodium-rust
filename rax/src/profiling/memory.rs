//! Thread-local memory access tracking for instruction profiling.
//!
//! Memory accesses are tracked per-instruction using thread-local storage.
//! This avoids synchronization overhead in the hot path.

use std::cell::Cell;

use super::stats::MemoryAccessStats;

// Thread-local storage for current instruction's memory accesses.
// Using Cell for interior mutability without runtime borrow checking overhead.
thread_local! {
    static MEM_READS: Cell<u32> = const { Cell::new(0) };
    static MEM_WRITES: Cell<u32> = const { Cell::new(0) };
    static BYTES_READ: Cell<u32> = const { Cell::new(0) };
    static BYTES_WRITTEN: Cell<u32> = const { Cell::new(0) };
}

/// Record a memory read operation.
#[inline(always)]
pub fn record_read(size: usize) {
    MEM_READS.with(|r| r.set(r.get() + 1));
    BYTES_READ.with(|b| b.set(b.get() + size as u32));
}

/// Record a memory write operation.
#[inline(always)]
pub fn record_write(size: usize) {
    MEM_WRITES.with(|w| w.set(w.get() + 1));
    BYTES_WRITTEN.with(|b| b.set(b.get() + size as u32));
}

/// Take the current memory access stats and reset counters.
/// Called at the end of instruction execution to collect stats.
#[inline(always)]
pub fn take_stats() -> MemoryAccessStats {
    let stats = MemoryAccessStats {
        reads: MEM_READS.with(|r| r.get()),
        writes: MEM_WRITES.with(|w| w.get()),
        bytes_read: BYTES_READ.with(|b| b.get()),
        bytes_written: BYTES_WRITTEN.with(|b| b.get()),
    };

    // Reset counters
    MEM_READS.with(|r| r.set(0));
    MEM_WRITES.with(|w| w.set(0));
    BYTES_READ.with(|b| b.set(0));
    BYTES_WRITTEN.with(|b| b.set(0));

    stats
}

/// Reset memory counters without returning stats.
/// Used when profiling is disabled mid-instruction.
#[inline(always)]
pub fn reset() {
    MEM_READS.with(|r| r.set(0));
    MEM_WRITES.with(|w| w.set(0));
    BYTES_READ.with(|b| b.set(0));
    BYTES_WRITTEN.with(|b| b.set(0));
}
