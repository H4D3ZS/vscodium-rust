//! DD escape - FLD, FST, FSTP, FRSTOR, FNSAVE, FUCOM, FUCOMP, FFREE

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::helpers::{fnsave, frstor, set_fpu_compare_flags};

/// DD escape - FLD, FST, FSTP, FRSTOR, FNSAVE, FUCOM, FUCOMP, FFREE
pub fn escape_dd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm = ctx.consume_u8()?;
    let reg = (modrm >> 3) & 7;
    let rm = modrm & 7;
    let is_memory = (modrm >> 6) != 3;

    if is_memory {
        let addr = vcpu.decode_fpu_modrm_addr(ctx, modrm)?;
        match reg {
            0 => {
                // FLD m64
                let val = vcpu.read_f64(addr)?;
                vcpu.fpu.push(val);
            }
            1 => {
                // FISTTP m64int
                let val = vcpu.fpu.pop().trunc() as i64;
                vcpu.write_mem64(addr, val as u64)?;
            }
            2 => {
                // FST m64
                let val = vcpu.fpu.get_st(0);
                vcpu.write_f64(addr, val)?;
            }
            3 => {
                // FSTP m64
                let val = vcpu.fpu.pop();
                vcpu.write_f64(addr, val)?;
            }
            4 => {
                // FRSTOR m94/108byte
                frstor(vcpu, addr)?;
            }
            6 => {
                // FNSAVE m94/108byte
                fnsave(vcpu, addr)?;
            }
            7 => {
                // FNSTSW m16
                vcpu.write_mem16(addr, vcpu.fpu.status_word)?;
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented DD memory opcode reg={} at RIP={:#x}",
                    reg, vcpu.regs.rip
                )));
            }
        }
    } else {
        match modrm {
            0xC0..=0xC7 => {
                // FFREE ST(i)
                let tag_shift = (vcpu.fpu.st_index(rm) as u16) * 2;
                vcpu.fpu.tag_word |= 3 << tag_shift; // Mark as empty
            }
            0xD0..=0xD7 => {
                // FST ST(i)
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(rm, st0);
            }
            0xD8..=0xDF => {
                // FSTP ST(i)
                let st0 = vcpu.fpu.pop();
                vcpu.fpu.set_st(rm.wrapping_sub(1) & 7, st0);
            }
            0xE0..=0xE7 => {
                // FUCOM ST(i)
                let st0 = vcpu.fpu.get_st(0);
                let sti = vcpu.fpu.get_st(rm);
                set_fpu_compare_flags(vcpu, st0, sti);
            }
            0xE8..=0xEF => {
                // FUCOMP ST(i)
                let st0 = vcpu.fpu.get_st(0);
                let sti = vcpu.fpu.get_st(rm);
                set_fpu_compare_flags(vcpu, st0, sti);
                vcpu.fpu.pop();
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented DD opcode modrm={:#x} at RIP={:#x}",
                    modrm, vcpu.regs.rip
                )));
            }
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
