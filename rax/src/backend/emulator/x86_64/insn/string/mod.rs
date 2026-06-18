//! String instructions: MOVS, STOS, LODS, SCAS, CMPS with REP prefix support.

mod cmps;
mod lods;
mod movs;
mod scas;
mod stos;

// Re-export all instruction functions
pub use cmps::*;
pub use lods::*;
pub use movs::*;
pub use scas::*;
pub use stos::*;

// ---------------------------------------------------------------------------
// 0x67 address-size override helpers, shared by all string instructions.
//
// In 64-bit mode (CS.L=1) a 0x67 prefix selects 32-bit addressing: the index
// registers (RSI/RDI) and the REP counter (RCX) are used as the 32-bit
// ESI/EDI/ECX. The effective offset is the low 32 bits, and any write back to
// an index/counter register clears the upper 32 bits (just like writing a
// 32-bit GPR). When the override is absent, all of these are no-ops and the
// full 64-bit register value is used unchanged.
// ---------------------------------------------------------------------------

/// Effective address offset contributed by an index register (RSI/RDI):
/// the low 32 bits when address-size override is active, else the full value.
#[inline(always)]
pub(super) fn index(reg: u64, addr32: bool) -> u64 {
    if addr32 { reg & 0xFFFF_FFFF } else { reg }
}

/// Advance an index register by `delta`, honoring DF (forward => add) and the
/// 32-bit address-size override (operate on the low 32 bits, clear the top).
#[inline(always)]
pub(super) fn advance_index(reg: u64, delta: u64, forward: bool, addr32: bool) -> u64 {
    if addr32 {
        let cur = reg & 0xFFFF_FFFF;
        let next = if forward {
            cur.wrapping_add(delta)
        } else {
            cur.wrapping_sub(delta)
        };
        next & 0xFFFF_FFFF
    } else if forward {
        reg.wrapping_add(delta)
    } else {
        reg.wrapping_sub(delta)
    }
}

/// REP iteration count from RCX, masked to 32 bits under address-size override.
#[inline(always)]
pub(super) fn rep_count(rcx: u64, addr32: bool) -> u64 {
    if addr32 { rcx & 0xFFFF_FFFF } else { rcx }
}

/// Decrement the REP counter, clearing the upper 32 bits under the override.
#[inline(always)]
pub(super) fn dec_count(rcx: u64, addr32: bool) -> u64 {
    if addr32 {
        (rcx & 0xFFFF_FFFF).wrapping_sub(1) & 0xFFFF_FFFF
    } else {
        rcx.wrapping_sub(1)
    }
}
