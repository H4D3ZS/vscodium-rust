//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_packed_int_arith(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        let (src2_lo, src2_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };

        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];

        // Process low 128 bits
        let (dst_lo, dst_hi) = self.packed_int_op(src1_lo, src1_hi, src2_lo, src2_hi, opcode);
        self.regs.xmm[xmm_dst][0] = dst_lo;
        self.regs.xmm[xmm_dst][1] = dst_hi;

        if vex_l == 1 {
            let (src2_hi2, src2_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let src1_hi2 = self.regs.ymm_high[xmm_src1][0];
            let src1_hi3 = self.regs.ymm_high[xmm_src1][1];

            let (dst_hi2, dst_hi3) =
                self.packed_int_op(src1_hi2, src1_hi3, src2_hi2, src2_hi3, opcode);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // Helper: perform packed integer operation
    fn packed_int_op(&self, a_lo: u64, a_hi: u64, b_lo: u64, b_hi: u64, opcode: u8) -> (u64, u64) {
        match opcode {
            // PADDQ: add packed qwords
            0xD4 => (a_lo.wrapping_add(b_lo), a_hi.wrapping_add(b_hi)),
            // PMULLW: multiply packed words, low result
            0xD5 => (
                self.mul_words_low(a_lo, b_lo),
                self.mul_words_low(a_hi, b_hi),
            ),
            // PSUBUSB: subtract packed unsigned bytes with saturation
            0xD8 => (self.sub_usb(a_lo, b_lo), self.sub_usb(a_hi, b_hi)),
            // PSUBUSW: subtract packed unsigned words with saturation
            0xD9 => (self.sub_usw(a_lo, b_lo), self.sub_usw(a_hi, b_hi)),
            // PMINUB: minimum of packed unsigned bytes
            0xDA => (self.min_ub(a_lo, b_lo), self.min_ub(a_hi, b_hi)),
            // PAND: bitwise AND
            0xDB => (a_lo & b_lo, a_hi & b_hi),
            // PADDUSB: add packed unsigned bytes with saturation
            0xDC => (self.add_usb(a_lo, b_lo), self.add_usb(a_hi, b_hi)),
            // PADDUSW: add packed unsigned words with saturation
            0xDD => (self.add_usw(a_lo, b_lo), self.add_usw(a_hi, b_hi)),
            // PMAXUB: maximum of packed unsigned bytes
            0xDE => (self.max_ub(a_lo, b_lo), self.max_ub(a_hi, b_hi)),
            // PANDN: bitwise AND NOT
            0xDF => (!a_lo & b_lo, !a_hi & b_hi),
            // PAVGB: average packed unsigned bytes
            0xE0 => (self.avg_ub(a_lo, b_lo), self.avg_ub(a_hi, b_hi)),
            // PAVGW: average packed unsigned words
            0xE3 => (self.avg_uw(a_lo, b_lo), self.avg_uw(a_hi, b_hi)),
            // PMULHUW: multiply packed unsigned words, high result
            0xE4 => (
                self.mul_words_high_unsigned(a_lo, b_lo),
                self.mul_words_high_unsigned(a_hi, b_hi),
            ),
            // PMULHW: multiply packed signed words, high result
            0xE5 => (
                self.mul_words_high_signed(a_lo, b_lo),
                self.mul_words_high_signed(a_hi, b_hi),
            ),
            // PSUBSB: subtract packed signed bytes with saturation
            0xE8 => (self.sub_sb(a_lo, b_lo), self.sub_sb(a_hi, b_hi)),
            // PSUBSW: subtract packed signed words with saturation
            0xE9 => (self.sub_sw(a_lo, b_lo), self.sub_sw(a_hi, b_hi)),
            // PMINSW: minimum of packed signed words
            0xEA => (self.min_sw(a_lo, b_lo), self.min_sw(a_hi, b_hi)),
            // POR: bitwise OR
            0xEB => (a_lo | b_lo, a_hi | b_hi),
            // PADDSB: add packed signed bytes with saturation
            0xEC => (self.add_sb(a_lo, b_lo), self.add_sb(a_hi, b_hi)),
            // PADDSW: add packed signed words with saturation
            0xED => (self.add_sw(a_lo, b_lo), self.add_sw(a_hi, b_hi)),
            // PMAXSW: maximum of packed signed words
            0xEE => (self.max_sw(a_lo, b_lo), self.max_sw(a_hi, b_hi)),
            // PXOR: bitwise XOR
            0xEF => (a_lo ^ b_lo, a_hi ^ b_hi),
            // PMULUDQ: multiply unsigned dwords, produce qword results
            0xF4 => (self.mul_udq(a_lo, b_lo), self.mul_udq(a_hi, b_hi)),
            // PMADDWD: multiply and add packed words
            0xF5 => (self.madd_wd(a_lo, b_lo), self.madd_wd(a_hi, b_hi)),
            // PSADBW: sum of absolute differences
            0xF6 => (self.sad_bw(a_lo, b_lo), self.sad_bw(a_hi, b_hi)),
            // PSUBB: subtract packed bytes
            0xF8 => (self.sub_bytes(a_lo, b_lo), self.sub_bytes(a_hi, b_hi)),
            // PSUBW: subtract packed words
            0xF9 => (self.sub_words(a_lo, b_lo), self.sub_words(a_hi, b_hi)),
            // PSUBD: subtract packed dwords
            0xFA => (self.sub_dwords(a_lo, b_lo), self.sub_dwords(a_hi, b_hi)),
            // PSUBQ: subtract packed qwords
            0xFB => (a_lo.wrapping_sub(b_lo), a_hi.wrapping_sub(b_hi)),
            // PADDB: add packed bytes
            0xFC => (self.add_bytes(a_lo, b_lo), self.add_bytes(a_hi, b_hi)),
            // PADDW: add packed words
            0xFD => (self.add_words(a_lo, b_lo), self.add_words(a_hi, b_hi)),
            // PADDD: add packed dwords
            0xFE => (self.add_dwords(a_lo, b_lo), self.add_dwords(a_hi, b_hi)),
            _ => (0, 0), // Should not happen
        }
    }

    // Helper: multiply packed words, return low 16 bits of each product
    fn mul_words_low(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as i16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
            let prod = (va as i32) * (vb as i32);
            result |= ((prod as u16) as u64) << (i * 16);
        }
        result
    }

    // Helper: multiply packed unsigned words, return high 16 bits
    fn mul_words_high_unsigned(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as u32;
            let vb = ((b >> (i * 16)) & 0xFFFF) as u32;
            let prod = va * vb;
            result |= ((prod >> 16) as u64) << (i * 16);
        }
        result
    }

    // Helper: multiply packed signed words, return high 16 bits
    fn mul_words_high_signed(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as i16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
            let prod = (va as i32) * (vb as i32);
            result |= (((prod >> 16) as u16) as u64) << (i * 16);
        }
        result
    }

    // Helper: subtract unsigned bytes with saturation
    fn sub_usb(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as u8;
            let vb = ((b >> (i * 8)) & 0xFF) as u8;
            let diff = va.saturating_sub(vb);
            result |= (diff as u64) << (i * 8);
        }
        result
    }

    // Helper: subtract unsigned words with saturation
    fn sub_usw(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as u16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
            let diff = va.saturating_sub(vb);
            result |= (diff as u64) << (i * 16);
        }
        result
    }

    // Helper: add unsigned bytes with saturation
    fn add_usb(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as u8;
            let vb = ((b >> (i * 8)) & 0xFF) as u8;
            let sum = va.saturating_add(vb);
            result |= (sum as u64) << (i * 8);
        }
        result
    }

    // Helper: add unsigned words with saturation
    fn add_usw(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as u16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
            let sum = va.saturating_add(vb);
            result |= (sum as u64) << (i * 16);
        }
        result
    }

    // Helper: subtract signed bytes with saturation
    fn sub_sb(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as i8;
            let vb = ((b >> (i * 8)) & 0xFF) as i8;
            let diff = va.saturating_sub(vb) as u8;
            result |= (diff as u64) << (i * 8);
        }
        result
    }

    // Helper: subtract signed words with saturation
    fn sub_sw(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as i16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
            let diff = va.saturating_sub(vb) as u16;
            result |= (diff as u64) << (i * 16);
        }
        result
    }

    // Helper: add signed bytes with saturation
    fn add_sb(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as i8;
            let vb = ((b >> (i * 8)) & 0xFF) as i8;
            let sum = va.saturating_add(vb) as u8;
            result |= (sum as u64) << (i * 8);
        }
        result
    }

    // Helper: add signed words with saturation
    fn add_sw(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as i16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
            let sum = va.saturating_add(vb) as u16;
            result |= (sum as u64) << (i * 16);
        }
        result
    }

    // Helper: minimum unsigned bytes
    fn min_ub(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as u8;
            let vb = ((b >> (i * 8)) & 0xFF) as u8;
            result |= (va.min(vb) as u64) << (i * 8);
        }
        result
    }

    // Helper: maximum unsigned bytes
    fn max_ub(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as u8;
            let vb = ((b >> (i * 8)) & 0xFF) as u8;
            result |= (va.max(vb) as u64) << (i * 8);
        }
        result
    }

    // Helper: minimum signed words
    fn min_sw(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as i16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
            result |= (va.min(vb) as u16 as u64) << (i * 16);
        }
        result
    }

    // Helper: maximum signed words
    fn max_sw(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as i16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
            result |= (va.max(vb) as u16 as u64) << (i * 16);
        }
        result
    }

    // Helper: average unsigned bytes
    fn avg_ub(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as u16;
            let vb = ((b >> (i * 8)) & 0xFF) as u16;
            let avg = ((va + vb + 1) >> 1) as u8;
            result |= (avg as u64) << (i * 8);
        }
        result
    }

    // Helper: average unsigned words
    fn avg_uw(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as u32;
            let vb = ((b >> (i * 16)) & 0xFFFF) as u32;
            let avg = ((va + vb + 1) >> 1) as u16;
            result |= (avg as u64) << (i * 16);
        }
        result
    }

    // Helper: multiply unsigned dwords to produce qwords
    fn mul_udq(&self, a: u64, b: u64) -> u64 {
        // Only uses the low dword of each qword
        let va = a as u32;
        let vb = b as u32;
        (va as u64) * (vb as u64)
    }

    // Helper: multiply and add packed words
    fn madd_wd(&self, a: u64, b: u64) -> u64 {
        let a0 = (a & 0xFFFF) as i16;
        let a1 = ((a >> 16) & 0xFFFF) as i16;
        let a2 = ((a >> 32) & 0xFFFF) as i16;
        let a3 = ((a >> 48) & 0xFFFF) as i16;
        let b0 = (b & 0xFFFF) as i16;
        let b1 = ((b >> 16) & 0xFFFF) as i16;
        let b2 = ((b >> 32) & 0xFFFF) as i16;
        let b3 = ((b >> 48) & 0xFFFF) as i16;
        // Each i16*i16 product fits in i32, but the sum of two products can
        // overflow i32 (e.g. 0x8000*0x8000 in both lanes); PMADDWD wraps it.
        let d0 = ((a0 as i32) * (b0 as i32)).wrapping_add((a1 as i32) * (b1 as i32)) as u32;
        let d1 = ((a2 as i32) * (b2 as i32)).wrapping_add((a3 as i32) * (b3 as i32)) as u32;
        (d0 as u64) | ((d1 as u64) << 32)
    }

    // Helper: sum of absolute differences
    fn sad_bw(&self, a: u64, b: u64) -> u64 {
        let mut sum = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as i16;
            let vb = ((b >> (i * 8)) & 0xFF) as i16;
            sum += (va - vb).unsigned_abs() as u64;
        }
        sum
    }

    // Helper: add packed bytes
    fn add_bytes(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as u8;
            let vb = ((b >> (i * 8)) & 0xFF) as u8;
            result |= (va.wrapping_add(vb) as u64) << (i * 8);
        }
        result
    }

    // Helper: subtract packed bytes
    fn sub_bytes(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as u8;
            let vb = ((b >> (i * 8)) & 0xFF) as u8;
            result |= (va.wrapping_sub(vb) as u64) << (i * 8);
        }
        result
    }

    // Helper: add packed words
    fn add_words(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as u16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
            result |= (va.wrapping_add(vb) as u64) << (i * 16);
        }
        result
    }

    // Helper: subtract packed words
    fn sub_words(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as u16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
            result |= (va.wrapping_sub(vb) as u64) << (i * 16);
        }
        result
    }

    // Helper: add packed dwords
    fn add_dwords(&self, a: u64, b: u64) -> u64 {
        let lo = (a as u32).wrapping_add(b as u32);
        let hi = ((a >> 32) as u32).wrapping_add((b >> 32) as u32);
        (lo as u64) | ((hi as u64) << 32)
    }

    // Helper: subtract packed dwords
    fn sub_dwords(&self, a: u64, b: u64) -> u64 {
        let lo = (a as u32).wrapping_sub(b as u32);
        let hi = ((a >> 32) as u32).wrapping_sub((b >> 32) as u32);
        (lo as u64) | ((hi as u64) << 32)
    }
}
