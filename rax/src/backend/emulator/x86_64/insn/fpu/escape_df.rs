//! DF escape - FILD, FIST, FISTP (word/qword), FBLD, FBSTP, FNSTSW AX

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::helpers::{bcd_to_f64, f64_to_bcd, fpu_round, set_fcomi_flags};

/// DF escape - FILD, FIST, FISTP (word/qword), FBLD, FBSTP, FNSTSW AX
pub fn escape_df(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm = ctx.consume_u8()?;
    let reg = (modrm >> 3) & 7;
    let rm = modrm & 7;
    let is_memory = (modrm >> 6) != 3;

    if is_memory {
        let addr = vcpu.decode_fpu_modrm_addr(ctx, modrm)?;
        match reg {
            0 => {
                // FILD m16int
                let val = vcpu.read_mem16(addr)? as i16 as f64;
                vcpu.fpu.push(val);
            }
            1 => {
                // FISTTP m16int
                let val = vcpu.fpu.pop().trunc() as i16;
                vcpu.write_mem16(addr, val as u16)?;
            }
            2 => {
                // FIST m16int
                let val = fpu_round(vcpu.fpu.control_word, vcpu.fpu.get_st(0)) as i16;
                vcpu.write_mem16(addr, val as u16)?;
            }
            3 => {
                // FISTP m16int
                let val = fpu_round(vcpu.fpu.control_word, vcpu.fpu.pop()) as i16;
                vcpu.write_mem16(addr, val as u16)?;
            }
            4 => {
                // FBLD m80bcd
                let bytes = vcpu.read_bytes(addr, 10)?;
                let val = bcd_to_f64(&bytes);
                vcpu.fpu.push(val);
            }
            5 => {
                // FILD m64int
                let val = vcpu.read_mem64(addr)? as i64 as f64;
                vcpu.fpu.push(val);
            }
            6 => {
                // FBSTP m80bcd
                let val = fpu_round(vcpu.fpu.control_word, vcpu.fpu.pop());
                let bytes = f64_to_bcd(val);
                vcpu.write_bytes(addr, &bytes)?;
            }
            7 => {
                // FISTP m64int
                let val = fpu_round(vcpu.fpu.control_word, vcpu.fpu.pop()) as i64;
                vcpu.write_mem64(addr, val as u64)?;
            }
            _ => unreachable!(),
        }
    } else {
        match modrm {
            0xE0 => {
                // FNSTSW AX
                vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | vcpu.fpu.status_word as u64;
            }
            0xE8..=0xEF => {
                // FUCOMIP ST(0), ST(i)
                let st0 = vcpu.fpu.get_st(0);
                let sti = vcpu.fpu.get_st(rm);
                set_fcomi_flags(vcpu, st0, sti);
                vcpu.fpu.pop();
            }
            0xF0..=0xF7 => {
                // FCOMIP ST(0), ST(i)
                let st0 = vcpu.fpu.get_st(0);
                let sti = vcpu.fpu.get_st(rm);
                set_fcomi_flags(vcpu, st0, sti);
                vcpu.fpu.pop();
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented DF opcode modrm={:#x} at RIP={:#x}",
                    modrm, vcpu.regs.rip
                )));
            }
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
