//! Load string instructions: LODSB, LODSW, LODSD, LODSQ.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;
use super::{advance_index, dec_count, index, rep_count};

/// LODSB (0xAC)
pub fn lodsb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let is_rep = ctx.rep_prefix.is_some();
    // Source DS:[RSI] honors a segment-override prefix (FS/GS); 0x67 selects
    // 32-bit addressing (ESI/ECX) in 64-bit mode.
    let src_base = vcpu.get_segment_base(ctx.segment_override);
    let addr32 = ctx.address_size_override && vcpu.sregs.cs.l;
    let count = if is_rep {
        rep_count(vcpu.regs.rcx, addr32)
    } else {
        1
    };
    for _ in 0..count {
        if is_rep && rep_count(vcpu.regs.rcx, addr32) == 0 {
            break;
        }
        let src = src_base.wrapping_add(index(vcpu.regs.rsi, addr32));
        let val = vcpu.mmu.read_u8(src, &vcpu.sregs)?;
        vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (val as u64);
        let forward = vcpu.regs.rflags & flags::bits::DF == 0;
        vcpu.regs.rsi = advance_index(vcpu.regs.rsi, 1, forward, addr32);
        if is_rep {
            vcpu.regs.rcx = dec_count(vcpu.regs.rcx, addr32);
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// LODSW/LODSD/LODSQ (0xAD)
pub fn lods(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let delta = op_size as u64;
    let is_rep = ctx.rep_prefix.is_some();
    let src_base = vcpu.get_segment_base(ctx.segment_override);
    let addr32 = ctx.address_size_override && vcpu.sregs.cs.l;
    let count = if is_rep {
        rep_count(vcpu.regs.rcx, addr32)
    } else {
        1
    };
    for _ in 0..count {
        if is_rep && rep_count(vcpu.regs.rcx, addr32) == 0 {
            break;
        }
        let src = src_base.wrapping_add(index(vcpu.regs.rsi, addr32));
        let val = vcpu.read_mem(src, op_size)?;
        vcpu.set_reg(0, val, op_size);
        let forward = vcpu.regs.rflags & flags::bits::DF == 0;
        vcpu.regs.rsi = advance_index(vcpu.regs.rsi, delta, forward, addr32);
        if is_rep {
            vcpu.regs.rcx = dec_count(vcpu.regs.rcx, addr32);
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
