//! D8 escape - FADD, FMUL, FCOM, FCOMP, FSUB, FSUBR, FDIV, FDIVR (float32)

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::helpers::set_fpu_compare_flags;

/// D8 escape - FADD, FMUL, FCOM, FCOMP, FSUB, FSUBR, FDIV, FDIVR
pub fn escape_d8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm = ctx.consume_u8()?;
    let reg = (modrm >> 3) & 7;
    let rm = modrm & 7;
    let is_memory = (modrm >> 6) != 3;

    if is_memory {
        // Memory operand - float32
        let addr = vcpu.decode_fpu_modrm_addr(ctx, modrm)?;
        let val = vcpu.read_f32(addr)?;
        match reg {
            0 => {
                // FADD m32
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0 + val as f64);
            }
            1 => {
                // FMUL m32
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0 * val as f64);
            }
            2 => {
                // FCOM m32
                let st0 = vcpu.fpu.get_st(0);
                set_fpu_compare_flags(vcpu, st0, val as f64);
            }
            3 => {
                // FCOMP m32
                let st0 = vcpu.fpu.get_st(0);
                set_fpu_compare_flags(vcpu, st0, val as f64);
                vcpu.fpu.pop();
            }
            4 => {
                // FSUB m32
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0 - val as f64);
            }
            5 => {
                // FSUBR m32
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, val as f64 - st0);
            }
            6 => {
                // FDIV m32
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0 / val as f64);
            }
            7 => {
                // FDIVR m32
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, val as f64 / st0);
            }
            _ => unreachable!(),
        }
    } else {
        // Register operand - ST(i)
        let sti = vcpu.fpu.get_st(rm);
        let st0 = vcpu.fpu.get_st(0);
        match reg {
            0 => vcpu.fpu.set_st(0, st0 + sti),         // FADD ST(0), ST(i)
            1 => vcpu.fpu.set_st(0, st0 * sti),         // FMUL ST(0), ST(i)
            2 => set_fpu_compare_flags(vcpu, st0, sti), // FCOM ST(i)
            3 => {
                // FCOMP ST(i)
                set_fpu_compare_flags(vcpu, st0, sti);
                vcpu.fpu.pop();
            }
            4 => vcpu.fpu.set_st(0, st0 - sti), // FSUB ST(0), ST(i)
            5 => vcpu.fpu.set_st(0, sti - st0), // FSUBR ST(0), ST(i)
            6 => vcpu.fpu.set_st(0, st0 / sti), // FDIV ST(0), ST(i)
            7 => vcpu.fpu.set_st(0, sti / st0), // FDIVR ST(0), ST(i)
            _ => unreachable!(),
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
