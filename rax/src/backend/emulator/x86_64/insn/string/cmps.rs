//! Compare string instructions: CMPSB, CMPSW, CMPSD, CMPSQ.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;
use super::{advance_index, dec_count, index, rep_count};

/// CMPSB (0xA6)
pub fn cmpsb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let is_rep = ctx.rep_prefix.is_some();
    // CMPS compares the segment-overridable source DS:[RSI] (val1) with the
    // fixed ES:[RDI] destination (val2). 0x67 selects 32-bit ESI/EDI/ECX.
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
        let dst = index(vcpu.regs.rdi, addr32);
        let val1 = vcpu.mmu.read_u8(src, &vcpu.sregs)? as u64;
        let val2 = vcpu.mmu.read_u8(dst, &vcpu.sregs)? as u64;
        let result = val1.wrapping_sub(val2);
        flags::update_flags_sub(&mut vcpu.regs.rflags, val1, val2, result, 1);
        vcpu.clear_lazy_flags();
        let forward = vcpu.regs.rflags & flags::bits::DF == 0;
        vcpu.regs.rsi = advance_index(vcpu.regs.rsi, 1, forward, addr32);
        vcpu.regs.rdi = advance_index(vcpu.regs.rdi, 1, forward, addr32);
        if is_rep {
            vcpu.regs.rcx = dec_count(vcpu.regs.rcx, addr32);
            let zf = (vcpu.regs.rflags & flags::bits::ZF) != 0;
            if ctx.rep_prefix == Some(0xF3) && !zf {
                break;
            }
            if ctx.rep_prefix == Some(0xF2) && zf {
                break;
            }
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMPSW/CMPSD/CMPSQ (0xA7)
pub fn cmps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
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
        let dst = index(vcpu.regs.rdi, addr32);
        let val1 = vcpu.read_mem(src, op_size)?;
        let val2 = vcpu.read_mem(dst, op_size)?;
        let result = val1.wrapping_sub(val2);
        flags::update_flags_sub(&mut vcpu.regs.rflags, val1, val2, result, op_size);
        vcpu.clear_lazy_flags();
        let forward = vcpu.regs.rflags & flags::bits::DF == 0;
        vcpu.regs.rsi = advance_index(vcpu.regs.rsi, delta, forward, addr32);
        vcpu.regs.rdi = advance_index(vcpu.regs.rdi, delta, forward, addr32);
        if is_rep {
            vcpu.regs.rcx = dec_count(vcpu.regs.rcx, addr32);
            let zf = (vcpu.regs.rflags & flags::bits::ZF) != 0;
            if ctx.rep_prefix == Some(0xF3) && !zf {
                break;
            }
            if ctx.rep_prefix == Some(0xF2) && zf {
                break;
            }
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
