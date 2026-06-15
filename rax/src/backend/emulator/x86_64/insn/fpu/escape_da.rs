//! DA escape - FIADD, FIMUL, FICOM, FICOMP, FISUB, FISUBR, FIDIV, FIDIVR (dword int)

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::helpers::set_fpu_compare_flags;

/// DA escape - FIADD, FIMUL, FICOM, FICOMP, FISUB, FISUBR, FIDIV, FIDIVR (dword int)
pub fn escape_da(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm = ctx.consume_u8()?;
    let reg = (modrm >> 3) & 7;
    let rm = modrm & 7;
    let is_memory = (modrm >> 6) != 3;

    if is_memory {
        let addr = vcpu.decode_fpu_modrm_addr(ctx, modrm)?;
        let val = vcpu.read_mem32(addr)? as i32 as f64;
        let st0 = vcpu.fpu.get_st(0);
        match reg {
            0 => vcpu.fpu.set_st(0, st0 + val),         // FIADD m32int
            1 => vcpu.fpu.set_st(0, st0 * val),         // FIMUL m32int
            2 => set_fpu_compare_flags(vcpu, st0, val), // FICOM m32int
            3 => {
                // FICOMP m32int
                set_fpu_compare_flags(vcpu, st0, val);
                vcpu.fpu.pop();
            }
            4 => vcpu.fpu.set_st(0, st0 - val), // FISUB m32int
            5 => vcpu.fpu.set_st(0, val - st0), // FISUBR m32int
            6 => vcpu.fpu.set_st(0, st0 / val), // FIDIV m32int
            7 => vcpu.fpu.set_st(0, val / st0), // FIDIVR m32int
            _ => unreachable!(),
        }
    } else {
        // Register forms - FCMOV
        vcpu.materialize_flags();
        let st0 = vcpu.fpu.get_st(0);
        let sti = vcpu.fpu.get_st(rm);
        let rflags = vcpu.regs.rflags;
        let cf = rflags & 1 != 0;
        let zf = rflags & 0x40 != 0;
        let pf = rflags & 4 != 0;

        match reg {
            0 if cf => vcpu.fpu.set_st(0, sti),       // FCMOVB
            1 if zf => vcpu.fpu.set_st(0, sti),       // FCMOVE
            2 if cf || zf => vcpu.fpu.set_st(0, sti), // FCMOVBE
            3 if pf => vcpu.fpu.set_st(0, sti),       // FCMOVU
            _ => {}                                   // condition not met
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
