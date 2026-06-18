//! Core shift and rotate execution helpers.

use crate::error::{Error, Result};

use super::super::super::cpu::X86_64Vcpu;
use super::super::super::flags;

/// Execute 8-bit shift/rotate operation.
pub fn execute_shift8(vcpu: &mut X86_64Vcpu, op: u8, val: u8, count: u8) -> Result<u8> {
    if count == 0 {
        return Ok(val);
    }
    let count = count & 0x1F;
    let cf_bit = flags::bits::CF;
    let of_bit = flags::bits::OF;
    vcpu.materialize_flags(); // Need CF for RCL/RCR
    let old_cf = (vcpu.regs.rflags & cf_bit) != 0;

    let is_rotate = op <= 3;
    let (result, cf, of) = match op {
        0 => {
            // ROL
            let result = val.rotate_left(count as u32);
            let cf = (result & 1) != 0;
            let of = if count == 1 {
                Some(((result >> 7) ^ (result & 1)) != 0)
            } else {
                None
            };
            (result, cf, of)
        }
        1 => {
            // ROR
            let result = val.rotate_right(count as u32);
            let cf = (result >> 7) != 0;
            let of = if count == 1 {
                Some(((result >> 7) ^ ((result >> 6) & 1)) != 0)
            } else {
                None
            };
            (result, cf, of)
        }
        2 => {
            // RCL - Rotate through carry left
            // 9-bit value: [CF:val], rotate left by count
            let count = count % 9; // 9-bit rotation period
            if count == 0 {
                return Ok(val);
            }
            // Build 9-bit value: bit 8 = old CF, bits 0-7 = val
            let mut wide = ((old_cf as u16) << 8) | (val as u16);
            // Rotate left by count within 9 bits
            for _ in 0..count {
                let msb = (wide >> 8) & 1;
                wide = ((wide << 1) & 0x1FF) | msb;
            }
            let result = (wide & 0xFF) as u8;
            let new_cf = (wide >> 8) & 1 != 0;
            let of = if count == 1 {
                Some(((result >> 7) != 0) ^ new_cf)
            } else {
                None
            };
            (result, new_cf, of)
        }
        3 => {
            // RCR - Rotate through carry right
            // 9-bit value: [CF:val], rotate right by count
            let count = count % 9; // 9-bit rotation period
            if count == 0 {
                return Ok(val);
            }
            // Build 9-bit value: bit 8 = old CF, bits 0-7 = val
            let mut wide = ((old_cf as u16) << 8) | (val as u16);
            // Rotate right by count within 9 bits
            for _ in 0..count {
                let lsb = wide & 1;
                wide = (wide >> 1) | (lsb << 8);
            }
            let result = (wide & 0xFF) as u8;
            let new_cf = (wide >> 8) & 1 != 0;
            let of = if count == 1 {
                Some(((result >> 7) ^ ((result >> 6) & 1)) != 0)
            } else {
                None
            };
            (result, new_cf, of)
        }
        4 => {
            // SHL/SAL
            let result = if count >= 8 { 0 } else { val << count };
            let cf = if count > 0 && count <= 8 {
                (val >> (8 - count)) & 1 != 0
            } else {
                false
            };
            let of = if count == 1 {
                Some(((result >> 7) ^ (cf as u8)) != 0)
            } else {
                Some(false)
            };
            (result, cf, of)
        }
        5 => {
            // SHR
            let result = if count >= 8 { 0 } else { val >> count };
            let cf = if count > 0 && count <= 8 {
                (val >> (count - 1)) & 1 != 0
            } else {
                false
            };
            let of = if count == 1 {
                Some((val >> 7) != 0)
            } else {
                Some(false)
            };
            (result, cf, of)
        }
        7 => {
            // SAR
            let result = if count >= 8 {
                if (val as i8) < 0 { 0xFF } else { 0 }
            } else {
                ((val as i8) >> count) as u8
            };
            let cf = if count == 0 {
                false
            } else if count <= 8 {
                (val >> (count - 1)) & 1 != 0
            } else {
                // SAR with count > width: operand is fully sign-extended,
                // so the last bit shifted out is the sign bit.
                (val >> 7) & 1 != 0
            };
            let of = Some(false);
            (result, cf, of)
        }
        _ => return Err(Error::Emulator(format!("unimplemented shift8 op: {}", op))),
    };

    if !is_rotate {
        // Update ZF, SF, PF first (this clears CF and OF)
        flags::update_flags_logic(&mut vcpu.regs.rflags, result as u64, 1);
    }

    // Now set CF and OF based on the shift/rotate operation
    if cf {
        vcpu.regs.rflags |= cf_bit;
    } else {
        vcpu.regs.rflags &= !cf_bit;
    }
    if let Some(of) = of {
        if of {
            vcpu.regs.rflags |= of_bit;
        } else {
            vcpu.regs.rflags &= !of_bit;
        }
    }
    vcpu.clear_lazy_flags();

    Ok(result)
}

/// Execute shift/rotate operation for 16/32/64-bit operands.
pub fn execute_shift(vcpu: &mut X86_64Vcpu, op: u8, val: u64, count: u8, size: u8) -> Result<u64> {
    if count == 0 {
        return Ok(val);
    }
    let bits = size as u32 * 8;
    let mask = if bits == 64 {
        !0u64
    } else {
        (1u64 << bits) - 1
    };
    let cf_bit = flags::bits::CF;
    let of_bit = flags::bits::OF;
    vcpu.materialize_flags(); // Need CF for RCL/RCR
    let old_cf = (vcpu.regs.rflags & cf_bit) != 0;

    let is_rotate = op <= 3;
    let (result, cf, of) = match op {
        0 => {
            // ROL
            let count = count as u32 % bits;
            let result = if count == 0 {
                val & mask
            } else {
                ((val << count) | (val >> (bits - count))) & mask
            };
            let cf = (result & 1) != 0;
            let of = if count == 1 {
                Some((((result >> (bits - 1)) ^ result) & 1) != 0)
            } else {
                None
            };
            (result, cf, of)
        }
        1 => {
            // ROR
            let count = count as u32 % bits;
            let result = if count == 0 {
                val & mask
            } else {
                ((val >> count) | (val << (bits - count))) & mask
            };
            let cf = (result >> (bits - 1)) != 0;
            let of = if count == 1 {
                Some(((result >> (bits - 1)) ^ ((result >> (bits - 2)) & 1)) != 0)
            } else {
                None
            };
            (result, cf, of)
        }
        2 => {
            // RCL - Rotate through carry left
            // (bits+1)-bit rotation through carry
            let rotate_size = bits + 1;
            let count = (count as u32) % rotate_size;
            if count == 0 {
                return Ok(val & mask);
            }

            // We need to perform a (bits+1)-bit rotation through carry
            // For simplicity, use a loop-based approach
            let mut result = val & mask;
            let mut carry = old_cf;

            for _ in 0..count {
                let msb = (result >> (bits - 1)) & 1 != 0;
                result = ((result << 1) | (carry as u64)) & mask;
                carry = msb;
            }

            let new_cf = carry;
            let of = if count == 1 {
                Some(((result >> (bits - 1)) & 1 != 0) ^ new_cf)
            } else {
                None
            };
            (result, new_cf, of)
        }
        3 => {
            // RCR - Rotate through carry right
            // (bits+1)-bit rotation through carry
            let rotate_size = bits + 1;
            let count = (count as u32) % rotate_size;
            if count == 0 {
                return Ok(val & mask);
            }

            let mut result = val & mask;
            let mut carry = old_cf;

            for _ in 0..count {
                let lsb = result & 1 != 0;
                result = (result >> 1) | ((carry as u64) << (bits - 1));
                carry = lsb;
            }

            let new_cf = carry;
            let of = if count == 1 {
                // OF = MSB XOR (MSB-1)
                Some(((result >> (bits - 1)) ^ (result >> (bits - 2))) & 1 != 0)
            } else {
                None
            };
            (result & mask, new_cf, of)
        }
        4 => {
            // SHL/SAL
            let result = if count as u32 >= bits {
                0
            } else {
                (val << count) & mask
            };
            let cf = if count > 0 && (count as u32) <= bits {
                (val >> (bits - count as u32)) & 1 != 0
            } else {
                false
            };
            let of = if count == 1 {
                Some(((result >> (bits - 1)) ^ (cf as u64)) != 0)
            } else {
                Some(false)
            };
            (result, cf, of)
        }
        5 => {
            // SHR
            let result = if count as u32 >= bits {
                0
            } else {
                (val >> count) & mask
            };
            let cf = if count > 0 && (count as u32) <= bits {
                (val >> (count - 1)) & 1 != 0
            } else {
                false
            };
            let of = if count == 1 {
                Some((val >> (bits - 1)) != 0)
            } else {
                Some(false)
            };
            (result, cf, of)
        }
        7 => {
            // SAR
            let result = if count as u32 >= bits {
                let sign = (val >> (bits - 1)) & 1;
                if sign != 0 { mask } else { 0 }
            } else {
                match size {
                    2 => (((val & 0xFFFF) as i16 >> count) as u16) as u64,
                    4 => (((val & 0xFFFF_FFFF) as i32 >> count) as u32) as u64,
                    8 => ((val as i64) >> count) as u64,
                    _ => val >> count,
                }
            };
            let cf = if count == 0 {
                false
            } else if (count as u32) <= bits {
                (val >> (count - 1)) & 1 != 0
            } else {
                // SAR with count > width: operand is fully sign-extended,
                // so the last bit shifted out is the sign bit.
                (val >> (bits - 1)) & 1 != 0
            };
            let of = Some(false);
            (result, cf, of)
        }
        _ => return Err(Error::Emulator(format!("unimplemented shift op: {}", op))),
    };

    if !is_rotate {
        // Update ZF, SF, PF first (this clears CF and OF)
        flags::update_flags_logic(&mut vcpu.regs.rflags, result, size);
    }

    // Now set CF and OF based on the shift/rotate operation
    if cf {
        vcpu.regs.rflags |= cf_bit;
    } else {
        vcpu.regs.rflags &= !cf_bit;
    }
    if let Some(of) = of {
        if of {
            vcpu.regs.rflags |= of_bit;
        } else {
            vcpu.regs.rflags &= !of_bit;
        }
    }
    vcpu.clear_lazy_flags();

    Ok(result)
}
