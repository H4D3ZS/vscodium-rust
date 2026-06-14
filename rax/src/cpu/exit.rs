//! Unified vCPU exit reasons.

/// vCPU exit reasons (backend-agnostic).
#[derive(Debug, Clone)]
pub enum VcpuExit {
    /// CPU halted (HLT instruction).
    Hlt,

    /// I/O port read.
    IoIn {
        /// Port number.
        port: u16,
        /// Number of bytes to read.
        size: u8,
    },

    /// Block string-input (`rep insb/insw/insd`): read `count` elements of
    /// `size` bytes each from a single fixed `port` in one exit. The backend
    /// reads `count * size` bytes from `port` and hands them to the vCPU's
    /// `complete_io_in` (which writes the staged destination block), avoiding a
    /// per-element VM exit. Used by the emulator for fast PIO disk transfers.
    IoInString {
        /// Port number (read repeatedly — not incremented).
        port: u16,
        /// Element size in bytes (1/2/4).
        size: u8,
        /// Number of elements to transfer.
        count: u32,
    },

    /// I/O port write.
    IoOut {
        /// Port number.
        port: u16,
        /// Data written.
        data: Vec<u8>,
    },

    /// Memory-mapped I/O read.
    MmioRead {
        /// Physical address.
        addr: u64,
        /// Number of bytes to read.
        size: u8,
    },

    /// Memory-mapped I/O write.
    MmioWrite {
        /// Physical address.
        addr: u64,
        /// Data written.
        data: Vec<u8>,
    },

    /// VM shutdown requested.
    Shutdown,

    /// System event (KVM-specific, but useful for compatibility).
    SystemEvent {
        /// Event type.
        type_: u32,
        /// Event flags.
        flags: u64,
    },

    /// vCPU entry failed.
    FailEntry {
        /// Hardware entry failure reason.
        reason: u64,
    },

    /// Internal error.
    InternalError,

    /// Debug breakpoint (INT3 or debug exception).
    Debug,

    /// Software interrupt or exception.
    Exception(u8),

    /// GDB breakpoint hit.
    #[cfg(feature = "debug")]
    GdbBreakpoint {
        /// Address of the breakpoint.
        addr: u64,
    },

    /// GDB single step completed.
    #[cfg(feature = "debug")]
    GdbStep,

    /// Unknown or unhandled exit.
    Unknown(String),
}
