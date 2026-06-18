//! Instruction-level profiling for the x86_64 emulator.
//!
//! This module provides comprehensive profiling capabilities including:
//! - Per-opcode timing statistics (count, min/max/mean/median, stddev, percentiles)
//! - Memory access tracking (read/write counts and bytes)
//! - Decode cache hit/miss statistics
//! - Instruction category breakdown
//! - Live console output and JSON export
//!
//! # Usage
//!
//! Enable profiling by compiling with the `profiling` feature:
//!
//! ```bash
//! cargo build --features profiling
//! ```
//!
//! Initialize profiling before starting the VM:
//!
//! ```rust,ignore
//! use rax::profiling::{ProfilingConfig, init, shutdown};
//!
//! let config = ProfilingConfig {
//!     track_memory: true,
//!     live_interval: 10_000_000,
//!     json_path: Some("profile.json".into()),
//! };
//! init(config);
//!
//! // ... run VM ...
//!
//! shutdown();
//! ```

mod collector;
pub mod memory;
mod mnemonics;
mod reporter;
mod stats;

// Re-export public types and functions
pub use collector::{
    ProfilingConfig, cache_stats, end_instruction, init, is_enabled, record_cache_hit,
    record_cache_miss, set_current_opcode_key, should_output_live, shutdown,
    take_current_opcode_key, total_instructions,
};
pub use mnemonics::{InstructionCategory, categorize, get_mnemonic};
pub use reporter::{ProfileReport, export_json, print_live_stats, print_summary};
pub use stats::{InstructionReport, MemoryAccessStats, OpcodeKey, OpcodeStats};

use std::time::Instant;

/// Start timing an instruction execution.
/// Returns an Instant that should be passed to `end_instruction`.
#[inline(always)]
pub fn begin_instruction() -> Instant {
    Instant::now()
}

/// Build an OpcodeKey from the raw opcode byte and context.
///
/// This function determines the correct OpcodeKey variant based on:
/// - Single-byte legacy opcodes
/// - Two-byte (0x0F) opcodes
/// - Three-byte (0x0F 0x38/0x3A) opcodes
/// - FPU (D8-DF) escape codes
/// - VEX/EVEX prefixes (handled specially)
#[inline]
pub fn build_opcode_key(
    opcode: u8,
    is_twobyte: bool,
    second_opcode: Option<u8>,
    is_threebyte_38: bool,
    is_threebyte_3a: bool,
    is_vex: bool,
    vex_map: u8,
    is_evex: bool,
    evex_map: u8,
    modrm_reg: Option<u8>,
) -> OpcodeKey {
    if is_evex {
        return OpcodeKey::Evex {
            map: evex_map,
            opcode: second_opcode.unwrap_or(opcode),
        };
    }

    if is_vex {
        return OpcodeKey::Vex {
            map: vex_map,
            opcode: second_opcode.unwrap_or(opcode),
        };
    }

    if is_threebyte_38 {
        return OpcodeKey::ThreeByte38(second_opcode.unwrap_or(opcode));
    }

    if is_threebyte_3a {
        return OpcodeKey::ThreeByte3A(second_opcode.unwrap_or(opcode));
    }

    if is_twobyte {
        return OpcodeKey::TwoByte(second_opcode.unwrap_or(opcode));
    }

    // FPU instructions (D8-DF)
    if (0xD8..=0xDF).contains(&opcode) {
        return OpcodeKey::Fpu {
            escape: opcode,
            modrm_reg: modrm_reg.unwrap_or(0) & 0x7,
        };
    }

    // Legacy single-byte opcode
    OpcodeKey::Legacy(opcode)
}

/// Simplified opcode key builder for the common case.
/// Used when we only have the primary opcode byte.
#[inline]
pub fn build_simple_opcode_key(opcode: u8) -> OpcodeKey {
    // FPU instructions (D8-DF)
    if (0xD8..=0xDF).contains(&opcode) {
        return OpcodeKey::Fpu {
            escape: opcode,
            modrm_reg: 0, // Will be updated if modrm is decoded
        };
    }

    // Legacy single-byte opcode
    OpcodeKey::Legacy(opcode)
}
