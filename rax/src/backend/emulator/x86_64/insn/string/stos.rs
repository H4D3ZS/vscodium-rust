//! Store string instructions: STOSB, STOSW, STOSD, STOSQ.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;
use super::super::super::mmu::AccessType;
use super::{advance_index, dec_count, index, rep_count};

/// Page size used by the MMU.
const PAGE_SIZE: u64 = 0x1000;
const PAGE_MASK: u64 = PAGE_SIZE - 1;

/// LAPIC MMIO window (mirrors the constants in mmu.rs). The bulk fast path must
/// never touch this region directly via `write_phys`; those have device side
/// effects that the per-element path routes correctly, so we fall back to the
/// slow path whenever a chunk would land in this window.
const LAPIC_BASE: u64 = 0xFEE00000;
const LAPIC_SIZE: u64 = 0x1000;

#[inline(always)]
fn paddr_is_mmio(paddr: u64) -> bool {
    paddr >= LAPIC_BASE && paddr < LAPIC_BASE + LAPIC_SIZE
}

/// STOSB (0xAA)
pub fn stosb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    stos_common(vcpu, ctx, 1)
}

/// STOSW/STOSD/STOSQ (0xAB)
pub fn stos(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    stos_common(vcpu, ctx, op_size)
}

/// Shared STOS implementation for all operand sizes (1/2/4/8).
///
/// Tries a bulk, page-wise fill for forward `REP STOS`; otherwise falls back to
/// the element-by-element loop. Both paths produce identical architectural
/// state (RDI/RCX and memory) and fault behavior.
#[inline]
fn stos_common(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    op_size: u8,
) -> Result<Option<VcpuExit>> {
    let is_rep = ctx.rep_prefix.is_some();
    let delta = op_size as u64;

    // 0x67 address-size override: RDI/RCX are used as 32-bit EDI/ECX.
    // The destination is always ES:[RDI] and is NOT segment-overridable.
    let addr32 = ctx.address_size_override && vcpu.sregs.cs.l;

    // Fast path: REP-prefixed, forward (DF==0), count > 1. Disabled under the
    // 32-bit address-size override, since the bulk path advances RDI as a full
    // 64-bit linear address with no masking.
    if is_rep && !addr32 && (vcpu.regs.rflags & flags::bits::DF) == 0 && vcpu.regs.rcx > 1 {
        stos_fast_path(vcpu, op_size)?;
        // Any remaining count (page-straddling element, code/MMIO page) falls
        // through to the slow loop below, resuming from current register state.
    }

    // Slow path: element-by-element, bit-for-bit identical to the original loop.
    // Also serves as the tail/fallback for the fast path (RCX is already 0 when
    // the fast path fully completed, so this loop is a no-op in that case).
    let count = if is_rep {
        rep_count(vcpu.regs.rcx, addr32)
    } else {
        1
    };
    for _ in 0..count {
        if is_rep && rep_count(vcpu.regs.rcx, addr32) == 0 {
            break;
        }
        let dst = index(vcpu.regs.rdi, addr32);
        vcpu.write_mem(dst, vcpu.regs.rax, op_size)?;
        let forward = vcpu.regs.rflags & flags::bits::DF == 0;
        vcpu.regs.rdi = advance_index(vcpu.regs.rdi, delta, forward, addr32);
        if is_rep {
            vcpu.regs.rcx = dec_count(vcpu.regs.rcx, addr32);
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Bulk, page-wise fill for forward `REP STOS`.
///
/// Builds the fill pattern from RAX once and writes whole page-bounded chunks
/// for as long as it is safe. It stops (leaving RCX > 0 for the slow loop to
/// finish) when:
///   * the next element would straddle a page boundary, or
///   * the destination page is marked as code (SMC) or resolves to MMIO.
/// A page fault is propagated unchanged; because chunks are processed strictly
/// in address order, the fault fires at exactly the element the slow path would
/// fault on, and RDI/RCX reflect all fully-written prior elements.
///
/// Preconditions (guaranteed by the caller): forward direction, RCX > 1.
fn stos_fast_path(vcpu: &mut X86_64Vcpu, op_size: u8) -> Result<()> {
    let delta = op_size as u64;
    debug_assert!(matches!(op_size, 1 | 2 | 4 | 8));

    // Build the fill pattern once. write_mem truncates RAX to op_size, so match
    // that here by taking the low `op_size` bytes in little-endian order.
    let value_bytes = vcpu.regs.rax.to_le_bytes();
    let elem = &value_bytes[..op_size as usize];

    // Pre-fill a one-page scratch buffer with the repeated pattern so each chunk
    // can be written with a single write_phys.
    let mut buf = [0u8; PAGE_SIZE as usize];
    if op_size == 1 {
        buf.fill(elem[0]);
    } else {
        let max_elems = (PAGE_SIZE as usize) / (op_size as usize);
        for i in 0..max_elems {
            let base = i * op_size as usize;
            buf[base..base + op_size as usize].copy_from_slice(elem);
        }
    }

    while vcpu.regs.rcx > 0 {
        let dst = vcpu.regs.rdi;
        let dst_off = dst & PAGE_MASK;

        // A single element straddling a page boundary cannot be handled with a
        // single page translation - defer to the slow per-element path.
        if dst_off + delta > PAGE_SIZE {
            return Ok(());
        }

        // Largest whole-element run staying within the page and the count.
        let dst_room = (PAGE_SIZE - dst_off) / delta;
        let elems = vcpu.regs.rcx.min(dst_room);
        debug_assert!(elems >= 1);
        let bytes = (elems * delta) as usize;

        // Translate the destination page once (also bounds/permission checks).
        let dst_paddr = vcpu.mmu.translate(dst, AccessType::Write, &vcpu.sregs)?;

        // Code page (SMC) or MMIO: defer the rest to the slow path so writes go
        // through decode-cache invalidation and device emulation.
        if vcpu.mmu.is_code_page(dst) || paddr_is_mmio(dst_paddr) {
            return Ok(());
        }

        // Bulk fill.
        vcpu.mmu.write_phys(dst_paddr, &buf[..bytes])?;

        // Advance by the whole chunk.
        vcpu.regs.rdi = vcpu.regs.rdi.wrapping_add(bytes as u64);
        vcpu.regs.rcx -= elems;
    }

    Ok(())
}
