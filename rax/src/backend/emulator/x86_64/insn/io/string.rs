//! String I/O instructions: INSB, INSW, OUTSB, OUTSW.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// INSB (0x6C) - Input byte from port DX into ES:[RDI]
pub fn insb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    ins_common(vcpu, ctx, 1)
}

/// INSW/INSD (0x6D) - Input word/dword from port DX into ES:[RDI]
pub fn insw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let size = if ctx.op_size == 2 { 2 } else { 4 };
    ins_common(vcpu, ctx, size)
}

/// Cap on elements transferred per batched `rep ins` exit (bounds the backend's
/// temporary buffer; a larger rep re-enters for the remainder).
const REP_INS_BLOCK_MAX: u64 = 16384;

fn ins_common(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext, size: u8) -> Result<Option<VcpuExit>> {
    let port = vcpu.regs.rdx as u16;
    let df = (vcpu.regs.rflags & flags::bits::DF) != 0;
    let rep = matches!(ctx.rep_prefix, Some(0xF3) | Some(0xF2));
    let addr_size = addr_size_bytes(vcpu, ctx);

    if rep && rep_count(vcpu, addr_size) == 0 {
        vcpu.regs.rip += ctx.cursor as u64;
        return Ok(None);
    }

    // Fast path: a forward (DF=0) `rep ins` transfers all its elements in one
    // exit instead of one VM exit per element. The destination is `count`
    // consecutive `size`-byte slots from ES:[RDI]; we advance RDI/RCX here and
    // let `complete_io_in` write the whole block from the bytes the backend
    // reads off the (fixed) port. (DF=1 is rare and falls through to the
    // per-element path so the reverse stride is handled correctly.)
    if rep && !df {
        let full = rep_count(vcpu, addr_size);
        let count = full.min(REP_INS_BLOCK_MAX);
        let addr = di_addr(vcpu, addr_size);
        // Advance RDI by count*size (forward) and RCX by -count.
        advance_di_block(vcpu, addr_size, size, count);
        sub_rep_count(vcpu, addr_size, count);
        // Only retire the instruction once the whole rep is done; otherwise the
        // capped remainder re-executes (RCX still non-zero).
        if count == full {
            vcpu.regs.rip += ctx.cursor as u64;
        }
        vcpu.set_io_pending_block(size, addr, count as u32);
        return Ok(Some(VcpuExit::IoInString {
            port,
            size,
            count: count as u32,
        }));
    }

    let addr = di_addr(vcpu, addr_size);
    vcpu.set_io_pending_mem(size, addr);
    update_di(vcpu, addr_size, size, df);

    if rep {
        let remaining = dec_rep_count(vcpu, addr_size);
        if remaining == 0 {
            vcpu.regs.rip += ctx.cursor as u64;
        }
    } else {
        vcpu.regs.rip += ctx.cursor as u64;
    }

    Ok(Some(VcpuExit::IoIn { port, size }))
}

/// Advance RDI forward by `count * size` bytes, honoring the address size.
fn advance_di_block(vcpu: &mut X86_64Vcpu, addr_size: u8, size: u8, count: u64) {
    let delta = count.wrapping_mul(size as u64);
    match addr_size {
        2 => {
            let di = (vcpu.regs.rdi as u16).wrapping_add(delta as u16);
            vcpu.regs.rdi = (vcpu.regs.rdi & !0xFFFF) | di as u64;
        }
        4 => {
            let edi = (vcpu.regs.rdi as u32).wrapping_add(delta as u32);
            vcpu.regs.rdi = edi as u64;
        }
        _ => vcpu.regs.rdi = vcpu.regs.rdi.wrapping_add(delta),
    }
}

/// Subtract `count` from RCX, honoring the address size.
fn sub_rep_count(vcpu: &mut X86_64Vcpu, addr_size: u8, count: u64) {
    match addr_size {
        2 => {
            let cx = (vcpu.regs.rcx as u16).wrapping_sub(count as u16);
            vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF) | cx as u64;
        }
        4 => {
            let ecx = (vcpu.regs.rcx as u32).wrapping_sub(count as u32);
            vcpu.regs.rcx = ecx as u64;
        }
        _ => vcpu.regs.rcx = vcpu.regs.rcx.wrapping_sub(count),
    }
}

fn addr_size_bytes(vcpu: &X86_64Vcpu, ctx: &InsnContext) -> u8 {
    if vcpu.sregs.cs.l {
        if ctx.address_size_override { 4 } else { 8 }
    } else {
        let default_16bit = !vcpu.sregs.cs.db;
        let is_16bit = default_16bit ^ ctx.address_size_override;
        if is_16bit { 2 } else { 4 }
    }
}

fn di_addr(vcpu: &X86_64Vcpu, addr_size: u8) -> u64 {
    match addr_size {
        2 => vcpu.regs.rdi & 0xFFFF,
        4 => vcpu.regs.rdi & 0xFFFF_FFFF,
        _ => vcpu.regs.rdi,
    }
}

fn update_di(vcpu: &mut X86_64Vcpu, addr_size: u8, size: u8, df: bool) {
    match addr_size {
        2 => {
            let di = vcpu.regs.rdi as u16;
            let delta = size as u16;
            let di = if df {
                di.wrapping_sub(delta)
            } else {
                di.wrapping_add(delta)
            };
            vcpu.regs.rdi = (vcpu.regs.rdi & !0xFFFF) | di as u64;
        }
        4 => {
            let edi = vcpu.regs.rdi as u32;
            let delta = size as u32;
            let edi = if df {
                edi.wrapping_sub(delta)
            } else {
                edi.wrapping_add(delta)
            };
            vcpu.regs.rdi = edi as u64;
        }
        _ => {
            let delta = size as u64;
            if df {
                vcpu.regs.rdi = vcpu.regs.rdi.wrapping_sub(delta);
            } else {
                vcpu.regs.rdi = vcpu.regs.rdi.wrapping_add(delta);
            }
        }
    }
}

fn rep_count(vcpu: &X86_64Vcpu, addr_size: u8) -> u64 {
    match addr_size {
        2 => vcpu.regs.rcx & 0xFFFF,
        4 => vcpu.regs.rcx & 0xFFFF_FFFF,
        _ => vcpu.regs.rcx,
    }
}

fn dec_rep_count(vcpu: &mut X86_64Vcpu, addr_size: u8) -> u64 {
    match addr_size {
        2 => {
            let cx = (vcpu.regs.rcx as u16).wrapping_sub(1);
            vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF) | cx as u64;
            cx as u64
        }
        4 => {
            let ecx = (vcpu.regs.rcx as u32).wrapping_sub(1);
            vcpu.regs.rcx = ecx as u64;
            ecx as u64
        }
        _ => {
            vcpu.regs.rcx = vcpu.regs.rcx.wrapping_sub(1);
            vcpu.regs.rcx
        }
    }
}

/// OUTSB (0x6E) - Output byte from DS:[RSI] to port DX
pub fn outsb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    outs_common(vcpu, ctx, 1)
}

fn outs_common(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext, size: u8) -> Result<Option<VcpuExit>> {
    let port = vcpu.regs.rdx as u16;
    let df = (vcpu.regs.rflags & flags::bits::DF) != 0;
    let rep = matches!(ctx.rep_prefix, Some(0xF3) | Some(0xF2));
    let addr_size = addr_size_bytes(vcpu, ctx);

    // Check REP count - if zero, skip the operation
    if rep && rep_count(vcpu, addr_size) == 0 {
        vcpu.regs.rip += ctx.cursor as u64;
        return Ok(None);
    }

    // Read data from memory at DS:RSI
    let addr = si_addr(vcpu, addr_size);
    let val = vcpu.read_mem(addr, size)?;
    let mut data = Vec::with_capacity(size as usize);
    for i in 0..size {
        data.push((val >> (i * 8)) as u8);
    }

    // Update RSI
    update_si(vcpu, addr_size, size, df);

    // Handle REP prefix
    if rep {
        let remaining = dec_rep_count(vcpu, addr_size);
        if remaining == 0 {
            vcpu.regs.rip += ctx.cursor as u64;
        }
        // RIP stays the same if remaining > 0 (will re-execute)
    } else {
        vcpu.regs.rip += ctx.cursor as u64;
    }

    Ok(Some(VcpuExit::IoOut { port, data }))
}

fn si_addr(vcpu: &X86_64Vcpu, addr_size: u8) -> u64 {
    match addr_size {
        2 => vcpu.regs.rsi & 0xFFFF,
        4 => vcpu.regs.rsi & 0xFFFF_FFFF,
        _ => vcpu.regs.rsi,
    }
}

fn update_si(vcpu: &mut X86_64Vcpu, addr_size: u8, size: u8, df: bool) {
    match addr_size {
        2 => {
            let si = vcpu.regs.rsi as u16;
            let delta = size as u16;
            let si = if df {
                si.wrapping_sub(delta)
            } else {
                si.wrapping_add(delta)
            };
            vcpu.regs.rsi = (vcpu.regs.rsi & !0xFFFF) | si as u64;
        }
        4 => {
            let esi = vcpu.regs.rsi as u32;
            let delta = size as u32;
            let esi = if df {
                esi.wrapping_sub(delta)
            } else {
                esi.wrapping_add(delta)
            };
            vcpu.regs.rsi = esi as u64;
        }
        _ => {
            let delta = size as u64;
            if df {
                vcpu.regs.rsi = vcpu.regs.rsi.wrapping_sub(delta);
            } else {
                vcpu.regs.rsi = vcpu.regs.rsi.wrapping_add(delta);
            }
        }
    }
}

/// OUTSW/OUTSD (0x6F) - Output word/dword from DS:[RSI] to port DX
pub fn outsw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Determine operand size: 0x66 prefix toggles between word (2) and dword (4)
    // In 64-bit mode default is 32-bit, in 32-bit mode default is also 32-bit
    // 0x66 prefix makes it 16-bit
    let size: u8 = if ctx.operand_size_override { 2 } else { 4 };
    outs_common(vcpu, ctx, size)
}
