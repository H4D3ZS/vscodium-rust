//! Group 3 instructions (0xF6, 0xF7).
//!
//! These opcodes handle multiple operations (TEST, NOT, NEG, MUL, IMUL, DIV, IDIV)
//! based on the ModR/M reg field.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// Group 3: TEST/NOT/NEG r/m8 (0xF6)
pub fn group3_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let rm = (modrm & 0x07) | ctx.rex_b();

    let has_rex = ctx.rex.is_some();
    match op {
        0 | 1 => {
            // TEST r/m8, imm8
            let dst = if modrm >> 6 == 3 {
                vcpu.get_reg8(rm, has_rex)
            } else {
                ctx.rip_relative_offset = 1;
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64
            };
            let imm = ctx.consume_u8()? as u64;
            let result = dst & imm;

            vcpu.set_lazy_logic(result, 1);
        }
        2 => {
            // NOT r/m8
            if modrm >> 6 == 3 {
                let val = vcpu.get_reg8(rm, has_rex);
                vcpu.set_reg8(rm, !val, has_rex);
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                let val = vcpu.mmu.read_u8(addr, &vcpu.sregs)?;
                vcpu.mmu.write_u8(addr, !val, &vcpu.sregs)?;
            }
        }
        3 => {
            // NEG r/m8
            if modrm >> 6 == 3 {
                let val = vcpu.get_reg8(rm, has_rex) as u8;
                let result = (val as i8).wrapping_neg() as u8;
                vcpu.set_reg8(rm, result as u64, has_rex);
                flags::update_flags_sub(&mut vcpu.regs.rflags, 0, val as u64, result as u64, 1);
                vcpu.clear_lazy_flags();
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                let val = vcpu.mmu.read_u8(addr, &vcpu.sregs)?;
                let result = (val as i8).wrapping_neg() as u8;
                vcpu.mmu.write_u8(addr, result, &vcpu.sregs)?;
                flags::update_flags_sub(&mut vcpu.regs.rflags, 0, val as u64, result as u64, 1);
                vcpu.clear_lazy_flags();
            }
        }
        4 => {
            // MUL r/m8 (unsigned)
            let src = if modrm >> 6 == 3 {
                vcpu.get_reg8(rm, has_rex) as u8
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u8(addr, &vcpu.sregs)?
            };
            let al = vcpu.regs.rax as u8;
            let result = (al as u16) * (src as u16);
            vcpu.set_reg(0, result as u64, 2);
            let overflow = (result >> 8) != 0;
            flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
            vcpu.clear_lazy_flags();
        }
        5 => {
            // IMUL r/m8 (signed, one-operand)
            let src = if modrm >> 6 == 3 {
                vcpu.get_reg8(rm, has_rex) as u8
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u8(addr, &vcpu.sregs)?
            };
            let al = vcpu.regs.rax as u8;
            let result = (al as i8 as i16) * (src as i8 as i16);
            vcpu.set_reg(0, result as i16 as u16 as u64, 2);
            let overflow = result != (result as i8 as i16);
            flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
            vcpu.clear_lazy_flags();
        }
        6 => {
            // DIV r/m8 (unsigned)
            let divisor = if modrm >> 6 == 3 {
                vcpu.get_reg8(rm, has_rex) as u8
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u8(addr, &vcpu.sregs)?
            };
            if divisor == 0 {
                // #DE exception - don't advance RIP
                vcpu.inject_exception(0, None)?;
                return Ok(None);
            }
            let dividend = vcpu.regs.rax as u16;
            let quotient = dividend / divisor as u16;
            let remainder = dividend % divisor as u16;
            if quotient > 0xFF {
                // #DE exception for overflow - don't advance RIP
                vcpu.inject_exception(0, None)?;
                return Ok(None);
            }
            let ax = ((remainder as u16) << 8) | (quotient as u16);
            vcpu.set_reg(0, ax as u64, 2);
        }
        7 => {
            // IDIV r/m8 (signed)
            let divisor = if modrm >> 6 == 3 {
                vcpu.get_reg8(rm, has_rex) as u8
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.mmu.read_u8(addr, &vcpu.sregs)?
            };
            if divisor == 0 {
                // #DE exception - don't advance RIP
                vcpu.inject_exception(0, None)?;
                return Ok(None);
            }
            let dividend = vcpu.regs.rax as i16;
            let divisor = divisor as i8 as i16;
            // Guard against INT_MIN / -1: the division itself would overflow
            // (and panic the host). This must raise #DE without advancing RIP.
            let (quotient, remainder) =
                match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
                    (Some(q), Some(r)) => (q, r),
                    _ => {
                        // #DE exception for overflow - don't advance RIP
                        vcpu.inject_exception(0, None)?;
                        return Ok(None);
                    }
                };
            if quotient < i8::MIN as i16 || quotient > i8::MAX as i16 {
                // #DE exception for overflow - don't advance RIP
                vcpu.inject_exception(0, None)?;
                return Ok(None);
            }
            let ax = ((remainder as i8 as u8 as u16) << 8) | (quotient as i8 as u8 as u16);
            vcpu.set_reg(0, ax as u64, 2);
        }
        _ => return Err(Error::Emulator(format!("unimplemented 0xF6 /op: {}", op))),
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 3: TEST/NOT/NEG r/m (0xF7)
pub fn group3_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let rm = (modrm & 0x07) | ctx.rex_b();

    match op {
        0 | 1 => {
            // TEST r/m, imm
            let imm_size = if op_size == 8 { 4 } else { op_size };
            let dst = if modrm >> 6 == 3 {
                vcpu.get_reg(rm, op_size)
            } else {
                ctx.rip_relative_offset = imm_size as usize;
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.read_mem(addr, op_size)?
            };
            let imm = ctx.consume_imm(imm_size)?;
            let imm = if op_size == 8 {
                imm as i32 as i64 as u64
            } else {
                imm
            };
            let result = dst & imm;
            vcpu.set_lazy_logic(result, op_size);
        }
        2 => {
            // NOT r/m
            if modrm >> 6 == 3 {
                let val = vcpu.get_reg(rm, op_size);
                vcpu.set_reg(rm, !val, op_size);
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                let val = vcpu.read_mem(addr, op_size)?;
                vcpu.write_mem(addr, !val, op_size)?;
            }
        }
        3 => {
            // NEG r/m
            if modrm >> 6 == 3 {
                let val = vcpu.get_reg(rm, op_size);
                let result = match op_size {
                    1 => (val as i8).wrapping_neg() as u8 as u64,
                    2 => (val as i16).wrapping_neg() as u16 as u64,
                    4 => (val as i32).wrapping_neg() as u32 as u64,
                    8 => (val as i64).wrapping_neg() as u64,
                    _ => val,
                };
                vcpu.set_reg(rm, result, op_size);
                flags::update_flags_sub(&mut vcpu.regs.rflags, 0, val, result, op_size);
                vcpu.clear_lazy_flags();
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                let val = vcpu.read_mem(addr, op_size)?;
                let result = match op_size {
                    1 => (val as i8).wrapping_neg() as u8 as u64,
                    2 => (val as i16).wrapping_neg() as u16 as u64,
                    4 => (val as i32).wrapping_neg() as u32 as u64,
                    8 => (val as i64).wrapping_neg() as u64,
                    _ => val,
                };
                vcpu.write_mem(addr, result, op_size)?;
                flags::update_flags_sub(&mut vcpu.regs.rflags, 0, val, result, op_size);
                vcpu.clear_lazy_flags();
            }
        }
        4 => {
            // MUL r/m (unsigned multiply)
            // DX:AX = AX * r/m (or 64-bit equivalent)
            let val = if modrm >> 6 == 3 {
                vcpu.get_reg(rm, op_size)
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.read_mem(addr, op_size)?
            };

            match op_size {
                2 => {
                    let result = (vcpu.regs.rax as u16 as u32) * (val as u16 as u32);
                    vcpu.set_reg(0, (result & 0xFFFF) as u64, 2);
                    vcpu.set_reg(2, ((result >> 16) & 0xFFFF) as u64, 2);
                    let overflow = (result >> 16) != 0;
                    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
                    vcpu.clear_lazy_flags();
                }
                4 => {
                    let result = (vcpu.regs.rax as u32 as u64) * (val as u32 as u64);
                    vcpu.set_reg(0, (result & 0xFFFFFFFF) as u64, 4);
                    vcpu.set_reg(2, ((result >> 32) & 0xFFFFFFFF) as u64, 4);
                    let overflow = (result >> 32) != 0;
                    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
                    vcpu.clear_lazy_flags();
                }
                8 => {
                    let result = (vcpu.regs.rax as u128) * (val as u128);
                    vcpu.set_reg(0, result as u64, 8);
                    vcpu.set_reg(2, (result >> 64) as u64, 8);
                    let overflow = (result >> 64) != 0;
                    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
                    vcpu.clear_lazy_flags();
                }
                _ => {}
            }
        }
        5 => {
            // IMUL r/m (signed multiply, one-operand form)
            let val = if modrm >> 6 == 3 {
                vcpu.get_reg(rm, op_size)
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.read_mem(addr, op_size)?
            };

            match op_size {
                2 => {
                    let result = (vcpu.regs.rax as i16 as i32) * (val as i16 as i32);
                    vcpu.set_reg(0, result as i16 as u16 as u64, 2);
                    vcpu.set_reg(2, (result >> 16) as i16 as u16 as u64, 2);
                    let overflow = result as i16 as i32 != result;
                    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
                    vcpu.clear_lazy_flags();
                }
                4 => {
                    let result = (vcpu.regs.rax as i32 as i64) * (val as i32 as i64);
                    vcpu.set_reg(0, result as u32 as u64, 4);
                    vcpu.set_reg(2, (result >> 32) as u32 as u64, 4);
                    let overflow = result as i32 as i64 != result;
                    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
                    vcpu.clear_lazy_flags();
                }
                8 => {
                    let result = (vcpu.regs.rax as i64 as i128) * (val as i64 as i128);
                    vcpu.set_reg(0, result as u64, 8);
                    vcpu.set_reg(2, (result >> 64) as u64, 8);
                    let overflow = result as i64 as i128 != result;
                    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
                    vcpu.clear_lazy_flags();
                }
                _ => {}
            }
        }
        6 => {
            // DIV r/m (unsigned divide)
            let divisor = if modrm >> 6 == 3 {
                vcpu.get_reg(rm, op_size)
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.read_mem(addr, op_size)?
            };

            if divisor == 0 {
                // #DE exception - don't advance RIP
                vcpu.inject_exception(0, None)?;
                return Ok(None);
            }

            match op_size {
                2 => {
                    let dividend =
                        ((vcpu.regs.rdx as u16 as u32) << 16) | (vcpu.regs.rax as u16 as u32);
                    let divisor = divisor as u16 as u32;
                    let quotient = dividend / divisor;
                    let remainder = dividend % divisor;
                    if quotient > 0xFFFF {
                        // #DE exception for overflow - don't advance RIP
                        vcpu.inject_exception(0, None)?;
                        return Ok(None);
                    }
                    vcpu.set_reg(0, quotient as u64, 2);
                    vcpu.set_reg(2, remainder as u64, 2);
                }
                4 => {
                    let dividend =
                        ((vcpu.regs.rdx as u32 as u64) << 32) | (vcpu.regs.rax as u32 as u64);
                    let divisor = divisor as u32 as u64;
                    let quotient = dividend / divisor;
                    let remainder = dividend % divisor;
                    if quotient > 0xFFFFFFFF {
                        // #DE exception for overflow - don't advance RIP
                        vcpu.inject_exception(0, None)?;
                        return Ok(None);
                    }
                    vcpu.set_reg(0, quotient as u32 as u64, 4);
                    vcpu.set_reg(2, remainder as u32 as u64, 4);
                }
                8 => {
                    let dividend = ((vcpu.regs.rdx as u128) << 64) | (vcpu.regs.rax as u128);
                    let divisor = divisor as u128;
                    let quotient = dividend / divisor;
                    let remainder = dividend % divisor;
                    if quotient > u64::MAX as u128 {
                        // #DE exception for overflow - don't advance RIP
                        vcpu.inject_exception(0, None)?;
                        return Ok(None);
                    }
                    vcpu.set_reg(0, quotient as u64, 8);
                    vcpu.set_reg(2, remainder as u64, 8);
                }
                _ => {}
            }
        }
        7 => {
            // IDIV r/m (signed divide)
            let divisor = if modrm >> 6 == 3 {
                vcpu.get_reg(rm, op_size)
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.read_mem(addr, op_size)?
            };

            if divisor == 0 {
                // #DE exception - don't advance RIP
                vcpu.inject_exception(0, None)?;
                return Ok(None);
            }

            match op_size {
                2 => {
                    let dividend = (((vcpu.regs.rdx as u16 as u32) << 16)
                        | (vcpu.regs.rax as u16 as u32)) as i32;
                    let divisor = divisor as i16 as i32;
                    // Guard against INT_MIN / -1 (division would panic the host).
                    let (quotient, remainder) =
                        match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
                            (Some(q), Some(r)) => (q, r),
                            _ => {
                                // #DE exception for overflow - don't advance RIP
                                vcpu.inject_exception(0, None)?;
                                return Ok(None);
                            }
                        };
                    if quotient < i16::MIN as i32 || quotient > i16::MAX as i32 {
                        // #DE exception for overflow - don't advance RIP
                        vcpu.inject_exception(0, None)?;
                        return Ok(None);
                    }
                    vcpu.set_reg(0, quotient as u16 as u64, 2);
                    vcpu.set_reg(2, remainder as u16 as u64, 2);
                }
                4 => {
                    let dividend = (((vcpu.regs.rdx as u32 as u64) << 32)
                        | (vcpu.regs.rax as u32 as u64)) as i64;
                    let divisor = divisor as i32 as i64;
                    // Guard against INT_MIN / -1 (division would panic the host).
                    let (quotient, remainder) =
                        match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
                            (Some(q), Some(r)) => (q, r),
                            _ => {
                                // #DE exception for overflow - don't advance RIP
                                vcpu.inject_exception(0, None)?;
                                return Ok(None);
                            }
                        };
                    if quotient < i32::MIN as i64 || quotient > i32::MAX as i64 {
                        // #DE exception for overflow - don't advance RIP
                        vcpu.inject_exception(0, None)?;
                        return Ok(None);
                    }
                    vcpu.set_reg(0, quotient as u32 as u64, 4);
                    vcpu.set_reg(2, remainder as u32 as u64, 4);
                }
                8 => {
                    let dividend =
                        (((vcpu.regs.rdx as u128) << 64) | (vcpu.regs.rax as u128)) as i128;
                    let divisor = divisor as i64 as i128;
                    // Guard against INT_MIN / -1 (division would panic the host).
                    let (quotient, remainder) =
                        match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
                            (Some(q), Some(r)) => (q, r),
                            _ => {
                                // #DE exception for overflow - don't advance RIP
                                vcpu.inject_exception(0, None)?;
                                return Ok(None);
                            }
                        };
                    if quotient < i64::MIN as i128 || quotient > i64::MAX as i128 {
                        // #DE exception for overflow - don't advance RIP
                        vcpu.inject_exception(0, None)?;
                        return Ok(None);
                    }
                    vcpu.set_reg(0, quotient as u64, 8);
                    vcpu.set_reg(2, remainder as u64, 8);
                }
                _ => {}
            }
        }
        _ => return Err(Error::Emulator(format!("unimplemented 0xF7 /op: {}", op))),
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
