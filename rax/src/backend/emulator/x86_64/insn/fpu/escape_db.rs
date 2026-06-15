//! DB escape - FILD, FIST, FISTP, FCLEX, FINIT, etc.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::helpers::{f64_to_f80, f80_to_f64, fpu_round, set_fcomi_flags};

/// DB escape - FILD, FIST, FISTP, FCLEX, FINIT, etc.
pub fn escape_db(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm = ctx.consume_u8()?;
    let reg = (modrm >> 3) & 7;
    let rm = modrm & 7;
    let is_memory = (modrm >> 6) != 3;

    if is_memory {
        let addr = vcpu.decode_fpu_modrm_addr(ctx, modrm)?;
        match reg {
            0 => {
                // FILD m32int
                let val = vcpu.read_mem32(addr)? as i32 as f64;
                vcpu.fpu.push(val);
            }
            1 => {
                // FISTTP m32int
                let val = vcpu.fpu.pop().trunc() as i32;
                vcpu.write_mem32(addr, val as u32)?;
            }
            2 => {
                // FIST m32int
                let val = fpu_round(vcpu.fpu.control_word, vcpu.fpu.get_st(0)) as i32;
                vcpu.write_mem32(addr, val as u32)?;
            }
            3 => {
                // FISTP m32int
                let val = fpu_round(vcpu.fpu.control_word, vcpu.fpu.pop()) as i32;
                vcpu.write_mem32(addr, val as u32)?;
            }
            5 => {
                // FLD m80fp (extended precision)
                let bytes = vcpu.read_bytes(addr, 10)?;
                let val = f80_to_f64(&bytes);
                vcpu.fpu.push(val);
            }
            7 => {
                // FSTP m80fp (extended precision)
                let val = vcpu.fpu.pop();
                let bytes = f64_to_f80(val);
                vcpu.write_bytes(addr, &bytes)?;
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented DB memory opcode reg={} at RIP={:#x}",
                    reg, vcpu.regs.rip
                )));
            }
        }
    } else {
        match modrm {
            0xC0..=0xC7 => {
                // FCMOVNB ST(0), ST(i)
                vcpu.materialize_flags();
                if vcpu.regs.rflags & 1 == 0 {
                    let sti = vcpu.fpu.get_st(rm);
                    vcpu.fpu.set_st(0, sti);
                }
            }
            0xC8..=0xCF => {
                // FCMOVNE ST(0), ST(i)
                vcpu.materialize_flags();
                if vcpu.regs.rflags & 0x40 == 0 {
                    let sti = vcpu.fpu.get_st(rm);
                    vcpu.fpu.set_st(0, sti);
                }
            }
            0xD0..=0xD7 => {
                // FCMOVNBE ST(0), ST(i)
                vcpu.materialize_flags();
                if vcpu.regs.rflags & 0x41 == 0 {
                    let sti = vcpu.fpu.get_st(rm);
                    vcpu.fpu.set_st(0, sti);
                }
            }
            0xD8..=0xDF => {
                // FCMOVNU ST(0), ST(i)
                vcpu.materialize_flags();
                if vcpu.regs.rflags & 4 == 0 {
                    let sti = vcpu.fpu.get_st(rm);
                    vcpu.fpu.set_st(0, sti);
                }
            }
            0xE2 => {
                // FNCLEX - clear exceptions
                vcpu.fpu.status_word &= !0x80FF; // Clear exception flags and ES
            }
            0xE3 => {
                // FNINIT - initialize FPU
                vcpu.fpu.init();
            }
            0xE8..=0xEF => {
                // FUCOMI ST(0), ST(i)
                let st0 = vcpu.fpu.get_st(0);
                let sti = vcpu.fpu.get_st(rm);
                set_fcomi_flags(vcpu, st0, sti);
            }
            0xF0..=0xF7 => {
                // FCOMI ST(0), ST(i)
                let st0 = vcpu.fpu.get_st(0);
                let sti = vcpu.fpu.get_st(rm);
                set_fcomi_flags(vcpu, st0, sti);
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented DB opcode modrm={:#x} at RIP={:#x}",
                    modrm, vcpu.regs.rip
                )));
            }
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
