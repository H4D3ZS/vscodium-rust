//! FPU helper functions for x87 instruction implementations.

use crate::error::Result;

use super::super::super::cpu::X86_64Vcpu;

/// Set FPU condition codes for comparison (C3, C2, C0)
pub fn set_fpu_compare_flags(vcpu: &mut X86_64Vcpu, a: f64, b: f64) {
    // C3 C2 C0 meaning:
    // 0  0  0  ST(0) > source
    // 0  0  1  ST(0) < source
    // 1  0  0  ST(0) = source
    // 1  1  1  unordered (NaN)
    let (c3, c2, c0) = if a.is_nan() || b.is_nan() {
        (true, true, true)
    } else if a > b {
        (false, false, false)
    } else if a < b {
        (false, false, true)
    } else {
        (true, false, false)
    };

    vcpu.fpu.status_word &= !0x4500; // Clear C3, C2, C0
    if c3 {
        vcpu.fpu.status_word |= 0x4000;
    }
    if c2 {
        vcpu.fpu.status_word |= 0x0400;
    }
    if c0 {
        vcpu.fpu.status_word |= 0x0100;
    }
}

/// Set RFLAGS for FCOMI/FUCOMI instructions
pub fn set_fcomi_flags(vcpu: &mut X86_64Vcpu, a: f64, b: f64) {
    // ZF PF CF meaning:
    // 0  0  0  ST(0) > ST(i)
    // 0  0  1  ST(0) < ST(i)
    // 1  0  0  ST(0) = ST(i)
    // 1  1  1  unordered
    let (zf, pf, cf) = if a.is_nan() || b.is_nan() {
        (true, true, true)
    } else if a > b {
        (false, false, false)
    } else if a < b {
        (false, false, true)
    } else {
        (true, false, false)
    };

    vcpu.regs.rflags &= !0x8D5; // Clear OF, SF, ZF, AF, PF, CF
    if zf {
        vcpu.regs.rflags |= 0x40;
    }
    if pf {
        vcpu.regs.rflags |= 0x04;
    }
    if cf {
        vcpu.regs.rflags |= 0x01;
    }
}

/// FXAM - examine ST(0)
pub fn fxam(vcpu: &mut X86_64Vcpu) {
    let st0 = vcpu.fpu.get_st(0);
    // C1 = sign bit
    let c1 = st0.is_sign_negative();

    // C3 C2 C0 = class
    let (c3, c2, c0) = if st0.is_nan() {
        (false, false, true) // NaN
    } else if st0.is_infinite() {
        (false, true, true) // Infinity
    } else if st0 == 0.0 {
        (true, false, false) // Zero
    } else if st0.is_subnormal() {
        (true, true, false) // Denormal
    } else {
        (false, true, false) // Normal
    };

    vcpu.fpu.status_word &= !0x4700;
    if c3 {
        vcpu.fpu.status_word |= 0x4000;
    }
    if c2 {
        vcpu.fpu.status_word |= 0x0400;
    }
    if c1 {
        vcpu.fpu.status_word |= 0x0200;
    }
    if c0 {
        vcpu.fpu.status_word |= 0x0100;
    }
}

/// FPU rounding based on control word
pub fn fpu_round(cw: u16, val: f64) -> f64 {
    let rc = (cw >> 10) & 3;
    match rc {
        0 => round_nearest_even(val), // Round to nearest, ties to even
        1 => val.floor(),             // Round down (toward -infinity)
        2 => val.ceil(),              // Round up (toward +infinity)
        3 => val.trunc(),             // Truncate (toward zero)
        _ => unreachable!(),
    }
}

pub fn round_nearest_even(val: f64) -> f64 {
    if !val.is_finite() {
        return val;
    }

    let int_part = val.trunc();
    let frac = val - int_part;

    if frac.abs() == 0.5 {
        let int_even = (int_part as i64) & 1 == 0;
        if int_even {
            return int_part;
        }
        return if val.is_sign_negative() {
            int_part - 1.0
        } else {
            int_part + 1.0
        };
    }

    val.round()
}

/// Decode f64 to mantissa, exponent, sign
pub fn decode_f64(val: f64) -> (f64, i32, bool) {
    let bits = val.to_bits();
    let sign = bits >> 63 != 0;
    let exponent = ((bits >> 52) & 0x7FF) as i32;
    let mantissa_bits = bits & 0x000F_FFFF_FFFF_FFFF;
    let mantissa = if exponent == 0 {
        (mantissa_bits as f64) / (1u64 << 52) as f64
    } else {
        1.0 + (mantissa_bits as f64) / (1u64 << 52) as f64
    };
    (if sign { -mantissa } else { mantissa }, exponent, sign)
}

/// Convert 80-bit extended precision to f64
pub fn f80_to_f64(bytes: &[u8]) -> f64 {
    // Simple conversion - just extract the value
    let mantissa = u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]);
    let exp_sign = u16::from_le_bytes([bytes[8], bytes[9]]);
    let sign = (exp_sign >> 15) != 0;
    let exponent = (exp_sign & 0x7FFF) as i32;

    if exponent == 0 && mantissa == 0 {
        return if sign { -0.0 } else { 0.0 };
    }
    if exponent == 0x7FFF {
        if mantissa & 0x7FFF_FFFF_FFFF_FFFF == 0 {
            return if sign {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            };
        } else {
            return f64::NAN;
        }
    }

    // Convert to f64 - bias difference is 16383 - 1023 = 15360
    let f64_exp = exponent - 15360;
    let f64_mantissa = mantissa >> 11; // Drop precision

    if f64_exp <= 0 || f64_exp >= 2047 {
        // Out of range - just approximate
        let val = (mantissa as f64) * (2.0_f64).powi(exponent - 16383 - 63);
        return if sign { -val } else { val };
    }

    let bits =
        ((sign as u64) << 63) | ((f64_exp as u64) << 52) | (f64_mantissa & 0x000F_FFFF_FFFF_FFFF);
    f64::from_bits(bits)
}

/// Convert f64 to 80-bit extended precision
pub fn f64_to_f80(val: f64) -> [u8; 10] {
    let mut bytes = [0u8; 10];

    if val == 0.0 {
        if val.is_sign_negative() {
            bytes[9] = 0x80;
        }
        return bytes;
    }
    if val.is_nan() {
        bytes[7] = 0xC0;
        bytes[8] = 0xFF;
        bytes[9] = 0x7F;
        return bytes;
    }
    if val.is_infinite() {
        bytes[7] = 0x80;
        bytes[8] = 0xFF;
        bytes[9] = if val.is_sign_negative() { 0xFF } else { 0x7F };
        return bytes;
    }

    let bits = val.to_bits();
    let sign = bits >> 63;
    let exp = ((bits >> 52) & 0x7FF) as i32;
    let mantissa = bits & 0x000F_FFFF_FFFF_FFFF;

    // Convert exponent (bias 1023 -> 16383)
    let f80_exp = (exp + 15360) as u16;
    // Add explicit integer bit
    let f80_mantissa = (1u64 << 63) | (mantissa << 11);

    bytes[0..8].copy_from_slice(&f80_mantissa.to_le_bytes());
    let exp_sign = f80_exp | ((sign as u16) << 15);
    bytes[8..10].copy_from_slice(&exp_sign.to_le_bytes());

    bytes
}

/// Convert BCD to f64
pub fn bcd_to_f64(bytes: &[u8]) -> f64 {
    let sign = (bytes[9] & 0x80) != 0;
    let mut val = 0i64;
    let mut multiplier = 1i64;
    for i in 0..9 {
        let lo = (bytes[i] & 0x0F) as i64;
        let hi = ((bytes[i] >> 4) & 0x0F) as i64;
        val += lo * multiplier;
        multiplier *= 10;
        val += hi * multiplier;
        multiplier *= 10;
    }
    if sign { -(val as f64) } else { val as f64 }
}

/// Convert f64 to BCD
pub fn f64_to_bcd(val: f64) -> [u8; 10] {
    let mut bytes = [0u8; 10];
    let sign = val.is_sign_negative();
    let mut n = val.abs() as u64;

    for i in 0..9 {
        let lo = (n % 10) as u8;
        n /= 10;
        let hi = (n % 10) as u8;
        n /= 10;
        bytes[i] = lo | (hi << 4);
    }
    if sign {
        bytes[9] = 0x80;
    }

    bytes
}

/// FLDENV - load FPU environment
pub fn fldenv(vcpu: &mut X86_64Vcpu, addr: u64) -> Result<()> {
    // Format (28 bytes):
    // 0-1: FCW, 2-3: FSW, 4-5: FTW, 6-7: FIP, 8-9: FCS, 10-11: FDP, 12-13: FDS
    // 14-27: reserved
    vcpu.fpu.control_word = vcpu.read_mem16(addr)?;
    vcpu.fpu.status_word = vcpu.read_mem16(addr + 2)?;
    vcpu.fpu.tag_word = vcpu.read_mem16(addr + 4)?;
    vcpu.fpu.instr_ptr = vcpu.read_mem16(addr + 6)? as u64;
    // FCS at offset 8 (code segment, ignored in 64-bit mode)
    vcpu.fpu.data_ptr = vcpu.read_mem16(addr + 10)? as u64;
    // FDS at offset 12 (data segment, ignored in 64-bit mode)
    vcpu.fpu.top = ((vcpu.fpu.status_word >> 11) & 7) as u8;
    Ok(())
}

/// FNSTENV - store FPU environment
pub fn fnstenv(vcpu: &mut X86_64Vcpu, addr: u64) -> Result<()> {
    // Format (28 bytes):
    // 0-1: FCW, 2-3: FSW, 4-5: FTW, 6-7: FIP, 8-9: FCS, 10-11: FDP, 12-13: FDS
    // 14-27: reserved
    vcpu.write_mem16(addr, vcpu.fpu.control_word)?;
    vcpu.write_mem16(addr + 2, vcpu.fpu.status_word)?;
    vcpu.write_mem16(addr + 4, vcpu.fpu.tag_word)?;
    vcpu.write_mem16(addr + 6, vcpu.fpu.instr_ptr as u16)?;
    vcpu.write_mem16(addr + 8, 0)?; // FCS (code segment)
    vcpu.write_mem16(addr + 10, vcpu.fpu.data_ptr as u16)?;
    vcpu.write_mem16(addr + 12, 0)?; // FDS (data segment)
    // Remaining 14 bytes are reserved/unused
    for i in 0..7 {
        vcpu.write_mem16(addr + 14 + i * 2, 0)?;
    }
    Ok(())
}

/// FRSTOR - restore FPU state
pub fn frstor(vcpu: &mut X86_64Vcpu, addr: u64) -> Result<()> {
    // Load environment first
    fldenv(vcpu, addr)?;
    // Load registers (28 bytes env + 8 * 10 bytes regs = 108 bytes)
    for i in 0usize..8 {
        let bytes = vcpu.read_bytes(addr + 28 + (i as u64) * 10, 10)?;
        vcpu.fpu.st[i] = f80_to_f64(&bytes);
    }
    Ok(())
}

/// FNSAVE - save FPU state and reinitialize
pub fn fnsave(vcpu: &mut X86_64Vcpu, addr: u64) -> Result<()> {
    // Store environment first
    fnstenv(vcpu, addr)?;
    // Store registers (28 bytes env + 8 * 10 bytes regs = 108 bytes)
    for i in 0usize..8 {
        let bytes = f64_to_f80(vcpu.fpu.st[i]);
        vcpu.write_bytes(addr + 28 + i as u64 * 10, &bytes)?;
    }
    // Reinitialize FPU
    vcpu.fpu.init();
    Ok(())
}

// Public wrappers for FXSAVE/FXRSTOR in cpu.rs
pub fn f80_to_f64_pub(bytes: &[u8]) -> f64 {
    f80_to_f64(bytes)
}

pub fn f64_to_f80_pub(val: f64) -> [u8; 10] {
    f64_to_f80(val)
}
