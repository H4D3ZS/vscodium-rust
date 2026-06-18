//! DE escape - FADDP, FMULP, FCOMP, FSUBP, FSUBRP, FDIVP, FDIVRP

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::helpers::set_fpu_compare_flags;

/// DE escape - FADDP, FMULP, FCOMP, FSUBP, FSUBRP, FDIVP, FDIVRP
pub fn escape_de(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm = ctx.consume_u8()?;
    let reg = (modrm >> 3) & 7;
    let rm = modrm & 7;
    let is_memory = (modrm >> 6) != 3;

    if is_memory {
        // Integer operations with m16int
        let addr = vcpu.decode_fpu_modrm_addr(ctx, modrm)?;
        let val = vcpu.read_mem16(addr)? as i16 as f64;
        let st0 = vcpu.fpu.get_st(0);
        match reg {
            0 => vcpu.fpu.set_st(0, st0 + val),         // FIADD m16int
            1 => vcpu.fpu.set_st(0, st0 * val),         // FIMUL m16int
            2 => set_fpu_compare_flags(vcpu, st0, val), // FICOM m16int
            3 => {
                // FICOMP m16int
                set_fpu_compare_flags(vcpu, st0, val);
                vcpu.fpu.pop();
            }
            4 => vcpu.fpu.set_st(0, st0 - val), // FISUB m16int
            5 => vcpu.fpu.set_st(0, val - st0), // FISUBR m16int
            6 => vcpu.fpu.set_st(0, st0 / val), // FIDIV m16int
            7 => vcpu.fpu.set_st(0, val / st0), // FIDIVR m16int
            _ => unreachable!(),
        }
    } else {
        // Register forms with pop
        let st0 = vcpu.fpu.get_st(0);
        let sti = vcpu.fpu.get_st(rm);
        match modrm {
            0xC0..=0xC7 => {
                // FADDP ST(i), ST(0)
                vcpu.fpu.set_st(rm, sti + st0);
                vcpu.fpu.pop();
            }
            0xC8..=0xCF => {
                // FMULP ST(i), ST(0)
                vcpu.fpu.set_st(rm, sti * st0);
                vcpu.fpu.pop();
            }
            0xD9 => {
                // FCOMPP
                set_fpu_compare_flags(vcpu, st0, sti);
                vcpu.fpu.pop();
                vcpu.fpu.pop();
            }
            0xE0..=0xE7 => {
                // FSUBRP ST(i), ST(0)
                vcpu.fpu.set_st(rm, st0 - sti);
                vcpu.fpu.pop();
            }
            0xE8..=0xEF => {
                // FSUBP ST(i), ST(0)
                vcpu.fpu.set_st(rm, sti - st0);
                vcpu.fpu.pop();
            }
            0xF0..=0xF7 => {
                // FDIVRP ST(i), ST(0)
                vcpu.fpu.set_st(rm, st0 / sti);
                vcpu.fpu.pop();
            }
            0xF8..=0xFF => {
                // FDIVP ST(i), ST(0)
                vcpu.fpu.set_st(rm, sti / st0);
                vcpu.fpu.pop();
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented DE opcode modrm={:#x} at RIP={:#x}",
                    modrm, vcpu.regs.rip
                )));
            }
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
