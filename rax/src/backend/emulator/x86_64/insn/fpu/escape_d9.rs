//! D9 escape - FLD, FST, FSTP, FLDENV, FLDCW, FNSTENV, FNSTCW, etc.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::helpers::{fldenv, fnstenv, fpu_round, fxam, round_nearest_even, set_fpu_compare_flags};

/// D9 escape - FLD, FST, FSTP, FLDENV, FLDCW, FNSTENV, FNSTCW, etc.
pub fn escape_d9(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm = ctx.consume_u8()?;
    let reg = (modrm >> 3) & 7;
    let rm = modrm & 7;
    let is_memory = (modrm >> 6) != 3;

    if is_memory {
        let addr = vcpu.decode_fpu_modrm_addr(ctx, modrm)?;
        match reg {
            0 => {
                // FLD m32
                let val = vcpu.read_f32(addr)?;
                vcpu.fpu.push(val as f64);
            }
            2 => {
                // FST m32
                let val = vcpu.fpu.get_st(0) as f32;
                vcpu.write_f32(addr, val)?;
            }
            3 => {
                // FSTP m32
                let val = vcpu.fpu.pop() as f32;
                vcpu.write_f32(addr, val)?;
            }
            4 => {
                // FLDENV m14/28byte
                fldenv(vcpu, addr)?;
            }
            5 => {
                // FLDCW m16
                let cw = vcpu.read_mem16(addr)?;
                vcpu.fpu.control_word = cw;
            }
            6 => {
                // FNSTENV m14/28byte
                fnstenv(vcpu, addr)?;
            }
            7 => {
                // FNSTCW m16
                vcpu.write_mem16(addr, vcpu.fpu.control_word)?;
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented D9 memory opcode reg={} at RIP={:#x}",
                    reg, vcpu.regs.rip
                )));
            }
        }
    } else {
        // Register or special opcodes
        match modrm {
            0xC0..=0xC7 => {
                // FLD ST(i)
                let val = vcpu.fpu.get_st(rm);
                vcpu.fpu.push(val);
            }
            0xC8..=0xCF => {
                // FXCH ST(i)
                let st0 = vcpu.fpu.get_st(0);
                let sti = vcpu.fpu.get_st(rm);
                vcpu.fpu.set_st(0, sti);
                vcpu.fpu.set_st(rm, st0);
            }
            0xD0 => {} // FNOP
            0xE0 => {
                // FCHS
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, -st0);
            }
            0xE1 => {
                // FABS
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0.abs());
            }
            0xE4 => {
                // FTST
                let st0 = vcpu.fpu.get_st(0);
                set_fpu_compare_flags(vcpu, st0, 0.0);
            }
            0xE5 => {
                // FXAM
                fxam(vcpu);
            }
            0xE8 => vcpu.fpu.push(1.0),                       // FLD1
            0xE9 => vcpu.fpu.push(std::f64::consts::LOG2_10), // FLDL2T
            0xEA => vcpu.fpu.push(std::f64::consts::LOG2_E),  // FLDL2E
            0xEB => vcpu.fpu.push(std::f64::consts::PI),      // FLDPI
            0xEC => vcpu
                .fpu
                .push(std::f64::consts::LN_2 / std::f64::consts::LN_10), // FLDLG2
            0xED => vcpu.fpu.push(std::f64::consts::LN_2),    // FLDLN2
            0xEE => vcpu.fpu.push(0.0),                       // FLDZ
            0xF0 => {
                // F2XM1
                let st0 = vcpu.fpu.get_st(0);
                let mut result = (2.0_f64).powf(st0) - 1.0;
                if st0 == 0.0 {
                    result = if st0.is_sign_negative() { -0.0 } else { 0.0 };
                }
                vcpu.fpu.set_st(0, result);
            }
            0xF1 => {
                // FYL2X
                let st0 = vcpu.fpu.pop();
                let st1 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st1 * st0.log2());
            }
            0xF2 => {
                // FPTAN
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0.tan());
                vcpu.fpu.push(1.0);
            }
            0xF3 => {
                // FPATAN
                let st0 = vcpu.fpu.pop();
                let st1 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st1.atan2(st0));
            }
            0xF4 => {
                // FXTRACT
                let st0 = vcpu.fpu.get_st(0);
                let (significand, exponent) = if st0 == 0.0 {
                    (st0, f64::NEG_INFINITY)
                } else if st0.is_nan() {
                    (f64::NAN, f64::NAN)
                } else if st0.is_infinite() {
                    (st0, f64::INFINITY)
                } else {
                    let bits = st0.to_bits();
                    let sign = bits >> 63 != 0;
                    let exp_bits = ((bits >> 52) & 0x7FF) as i32;
                    let mant_bits = bits & 0x000F_FFFF_FFFF_FFFF;

                    if exp_bits == 0 {
                        if mant_bits == 0 {
                            (st0, f64::NEG_INFINITY)
                        } else {
                            let msb_index = 63 - mant_bits.leading_zeros() as i32;
                            let shift = 52 - msb_index;
                            let norm_mant = mant_bits << shift;
                            let significand = (norm_mant as f64) / (1u64 << 52) as f64;
                            let exponent = (1 - 1023 - shift) as f64;
                            (if sign { -significand } else { significand }, exponent)
                        }
                    } else {
                        let significand = 1.0 + (mant_bits as f64) / (1u64 << 52) as f64;
                        let exponent = (exp_bits - 1023) as f64;
                        (if sign { -significand } else { significand }, exponent)
                    }
                };

                vcpu.fpu.set_st(0, exponent);
                vcpu.fpu.push(significand);
            }
            0xF5 => {
                // FPREM1
                let st0 = vcpu.fpu.get_st(0);
                let st1 = vcpu.fpu.get_st(1);
                let (remainder, q_bits) = if st0 == 0.0 {
                    (st0, 0)
                } else if st1.is_infinite() {
                    (st0, 0)
                } else {
                    let quotient = round_nearest_even(st0 / st1);
                    let remainder = st0 - quotient * st1;
                    ((remainder), (quotient as i64).abs() & 0x7)
                };

                vcpu.fpu.set_st(0, remainder);
                vcpu.fpu.status_word &= !0x4700;
                if q_bits & 0x1 != 0 {
                    vcpu.fpu.status_word |= 0x0200; // C1 (Q0)
                }
                if q_bits & 0x2 != 0 {
                    vcpu.fpu.status_word |= 0x4000; // C3 (Q1)
                }
                if q_bits & 0x4 != 0 {
                    vcpu.fpu.status_word |= 0x0100; // C0 (Q2)
                }
            }
            0xF6 => {
                // FDECSTP
                vcpu.fpu.top = vcpu.fpu.top.wrapping_sub(1) & 7;
                vcpu.fpu.status_word =
                    (vcpu.fpu.status_word & !0x3800) | ((vcpu.fpu.top as u16) << 11);
            }
            0xF7 => {
                // FINCSTP
                vcpu.fpu.top = vcpu.fpu.top.wrapping_add(1) & 7;
                vcpu.fpu.status_word =
                    (vcpu.fpu.status_word & !0x3800) | ((vcpu.fpu.top as u16) << 11);
            }
            0xF8 => {
                // FPREM
                let st0 = vcpu.fpu.get_st(0);
                let st1 = vcpu.fpu.get_st(1);
                let (remainder, q_bits) = if st0 == 0.0 {
                    (st0, 0)
                } else if st1.is_infinite() {
                    (st0, 0)
                } else {
                    let quotient = (st0 / st1).trunc();
                    let remainder = st0 - quotient * st1;
                    ((remainder), (quotient as i64).abs() & 0x7)
                };

                vcpu.fpu.set_st(0, remainder);
                vcpu.fpu.status_word &= !0x4700;
                if q_bits & 0x1 != 0 {
                    vcpu.fpu.status_word |= 0x0200; // C1 (Q0)
                }
                if q_bits & 0x2 != 0 {
                    vcpu.fpu.status_word |= 0x4000; // C3 (Q1)
                }
                if q_bits & 0x4 != 0 {
                    vcpu.fpu.status_word |= 0x0100; // C0 (Q2)
                }
            }
            0xF9 => {
                // FYL2XP1
                let st0 = vcpu.fpu.pop();
                let st1 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st1 * (1.0 + st0).log2());
            }
            0xFA => {
                // FSQRT
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0.sqrt());
            }
            0xFB => {
                // FSINCOS
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0.sin());
                vcpu.fpu.push(st0.cos());
            }
            0xFC => {
                // FRNDINT
                let st0 = vcpu.fpu.get_st(0);
                let rounded = fpu_round(vcpu.fpu.control_word, st0);
                vcpu.fpu.set_st(0, rounded);
            }
            0xFD => {
                // FSCALE
                let st0 = vcpu.fpu.get_st(0);
                let st1 = vcpu.fpu.get_st(1);
                vcpu.fpu.set_st(0, st0 * (2.0_f64).powf(st1.trunc()));
            }
            0xFE => {
                // FSIN
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0.sin());
            }
            0xFF => {
                // FCOS
                let st0 = vcpu.fpu.get_st(0);
                vcpu.fpu.set_st(0, st0.cos());
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented D9 opcode modrm={:#x} at RIP={:#x}",
                    modrm, vcpu.regs.rip
                )));
            }
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
