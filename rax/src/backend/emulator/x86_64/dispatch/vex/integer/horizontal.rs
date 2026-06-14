//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_pshufb(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        let (mask_lo, mask_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };

        let src_lo = self.regs.xmm[xmm_src1][0];
        let src_hi = self.regs.xmm[xmm_src1][1];

        // Create 16-byte array from source
        let mut src = [0u8; 16];
        for i in 0..8 {
            src[i] = ((src_lo >> (i * 8)) & 0xFF) as u8;
            src[i + 8] = ((src_hi >> (i * 8)) & 0xFF) as u8;
        }

        // Shuffle based on mask
        let mut dst_lo = 0u64;
        let mut dst_hi = 0u64;
        for i in 0..8 {
            let idx = ((mask_lo >> (i * 8)) & 0xFF) as u8;
            let val = if idx & 0x80 != 0 {
                0
            } else {
                src[(idx & 0x0F) as usize]
            };
            dst_lo |= (val as u64) << (i * 8);
        }
        for i in 0..8 {
            let idx = ((mask_hi >> (i * 8)) & 0xFF) as u8;
            let val = if idx & 0x80 != 0 {
                0
            } else {
                src[(idx & 0x0F) as usize]
            };
            dst_hi |= (val as u64) << (i * 8);
        }

        self.regs.xmm[xmm_dst][0] = dst_lo;
        self.regs.xmm[xmm_dst][1] = dst_hi;

        if vex_l == 1 {
            let (mask_hi2, mask_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let src_hi2 = self.regs.ymm_high[xmm_src1][0];
            let src_hi3 = self.regs.ymm_high[xmm_src1][1];

            let mut src2 = [0u8; 16];
            for i in 0..8 {
                src2[i] = ((src_hi2 >> (i * 8)) & 0xFF) as u8;
                src2[i + 8] = ((src_hi3 >> (i * 8)) & 0xFF) as u8;
            }

            let mut dst_hi2 = 0u64;
            let mut dst_hi3 = 0u64;
            for i in 0..8 {
                let idx = ((mask_hi2 >> (i * 8)) & 0xFF) as u8;
                let val = if idx & 0x80 != 0 {
                    0
                } else {
                    src2[(idx & 0x0F) as usize]
                };
                dst_hi2 |= (val as u64) << (i * 8);
            }
            for i in 0..8 {
                let idx = ((mask_hi3 >> (i * 8)) & 0xFF) as u8;
                let val = if idx & 0x80 != 0 {
                    0
                } else {
                    src2[(idx & 0x0F) as usize]
                };
                dst_hi3 |= (val as u64) << (i * 8);
            }

            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_phadd(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        elem_bits: u32,
        saturate: bool,
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

        let (dst_lo, dst_hi) =
            self.hadd_128(src1_lo, src1_hi, src2_lo, src2_hi, elem_bits, saturate);
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
                self.hadd_128(src1_hi2, src1_hi3, src2_hi2, src2_hi3, elem_bits, saturate);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // Helper: horizontal add for 128-bit lane
    fn hadd_128(
        &self,
        a_lo: u64,
        a_hi: u64,
        b_lo: u64,
        b_hi: u64,
        elem_bits: u32,
        saturate: bool,
    ) -> (u64, u64) {
        if elem_bits == 16 {
            // Words
            let a = [
                a_lo as u16,
                (a_lo >> 16) as u16,
                (a_lo >> 32) as u16,
                (a_lo >> 48) as u16,
                a_hi as u16,
                (a_hi >> 16) as u16,
                (a_hi >> 32) as u16,
                (a_hi >> 48) as u16,
            ];
            let b = [
                b_lo as u16,
                (b_lo >> 16) as u16,
                (b_lo >> 32) as u16,
                (b_lo >> 48) as u16,
                b_hi as u16,
                (b_hi >> 16) as u16,
                (b_hi >> 32) as u16,
                (b_hi >> 48) as u16,
            ];
            let mut r = [0u16; 8];
            for i in 0..4 {
                if saturate {
                    r[i] = (a[i * 2] as i16).saturating_add(a[i * 2 + 1] as i16) as u16;
                    r[i + 4] = (b[i * 2] as i16).saturating_add(b[i * 2 + 1] as i16) as u16;
                } else {
                    r[i] = (a[i * 2] as i16).wrapping_add(a[i * 2 + 1] as i16) as u16;
                    r[i + 4] = (b[i * 2] as i16).wrapping_add(b[i * 2 + 1] as i16) as u16;
                }
            }
            let lo = (r[0] as u64)
                | ((r[1] as u64) << 16)
                | ((r[2] as u64) << 32)
                | ((r[3] as u64) << 48);
            let hi = (r[4] as u64)
                | ((r[5] as u64) << 16)
                | ((r[6] as u64) << 32)
                | ((r[7] as u64) << 48);
            (lo, hi)
        } else {
            // Dwords
            let a = [
                a_lo as u32,
                (a_lo >> 32) as u32,
                a_hi as u32,
                (a_hi >> 32) as u32,
            ];
            let b = [
                b_lo as u32,
                (b_lo >> 32) as u32,
                b_hi as u32,
                (b_hi >> 32) as u32,
            ];
            let r0 = (a[0] as i32).wrapping_add(a[1] as i32) as u32;
            let r1 = (a[2] as i32).wrapping_add(a[3] as i32) as u32;
            let r2 = (b[0] as i32).wrapping_add(b[1] as i32) as u32;
            let r3 = (b[2] as i32).wrapping_add(b[3] as i32) as u32;
            let lo = (r0 as u64) | ((r1 as u64) << 32);
            let hi = (r2 as u64) | ((r3 as u64) << 32);
            (lo, hi)
        }
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_phsub(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        elem_bits: u32,
        saturate: bool,
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

        let (dst_lo, dst_hi) =
            self.hsub_128(src1_lo, src1_hi, src2_lo, src2_hi, elem_bits, saturate);
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
                self.hsub_128(src1_hi2, src1_hi3, src2_hi2, src2_hi3, elem_bits, saturate);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // Helper: horizontal subtract for 128-bit lane
    fn hsub_128(
        &self,
        a_lo: u64,
        a_hi: u64,
        b_lo: u64,
        b_hi: u64,
        elem_bits: u32,
        saturate: bool,
    ) -> (u64, u64) {
        if elem_bits == 16 {
            let a = [
                a_lo as u16,
                (a_lo >> 16) as u16,
                (a_lo >> 32) as u16,
                (a_lo >> 48) as u16,
                a_hi as u16,
                (a_hi >> 16) as u16,
                (a_hi >> 32) as u16,
                (a_hi >> 48) as u16,
            ];
            let b = [
                b_lo as u16,
                (b_lo >> 16) as u16,
                (b_lo >> 32) as u16,
                (b_lo >> 48) as u16,
                b_hi as u16,
                (b_hi >> 16) as u16,
                (b_hi >> 32) as u16,
                (b_hi >> 48) as u16,
            ];
            let mut r = [0u16; 8];
            for i in 0..4 {
                if saturate {
                    r[i] = (a[i * 2] as i16).saturating_sub(a[i * 2 + 1] as i16) as u16;
                    r[i + 4] = (b[i * 2] as i16).saturating_sub(b[i * 2 + 1] as i16) as u16;
                } else {
                    r[i] = (a[i * 2] as i16).wrapping_sub(a[i * 2 + 1] as i16) as u16;
                    r[i + 4] = (b[i * 2] as i16).wrapping_sub(b[i * 2 + 1] as i16) as u16;
                }
            }
            let lo = (r[0] as u64)
                | ((r[1] as u64) << 16)
                | ((r[2] as u64) << 32)
                | ((r[3] as u64) << 48);
            let hi = (r[4] as u64)
                | ((r[5] as u64) << 16)
                | ((r[6] as u64) << 32)
                | ((r[7] as u64) << 48);
            (lo, hi)
        } else {
            let a = [
                a_lo as u32,
                (a_lo >> 32) as u32,
                a_hi as u32,
                (a_hi >> 32) as u32,
            ];
            let b = [
                b_lo as u32,
                (b_lo >> 32) as u32,
                b_hi as u32,
                (b_hi >> 32) as u32,
            ];
            let r0 = (a[0] as i32).wrapping_sub(a[1] as i32) as u32;
            let r1 = (a[2] as i32).wrapping_sub(a[3] as i32) as u32;
            let r2 = (b[0] as i32).wrapping_sub(b[1] as i32) as u32;
            let r3 = (b[2] as i32).wrapping_sub(b[3] as i32) as u32;
            let lo = (r0 as u64) | ((r1 as u64) << 32);
            let hi = (r2 as u64) | ((r3 as u64) << 32);
            (lo, hi)
        }
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_pmaddubsw(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
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

        self.regs.xmm[xmm_dst][0] = self.pmaddubsw_64(src1_lo, src2_lo);
        self.regs.xmm[xmm_dst][1] = self.pmaddubsw_64(src1_hi, src2_hi);

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
            self.regs.ymm_high[xmm_dst][0] = self.pmaddubsw_64(src1_hi2, src2_hi2);
            self.regs.ymm_high[xmm_dst][1] = self.pmaddubsw_64(src1_hi3, src2_hi3);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // Helper: pmaddubsw for 64 bits (8 bytes -> 4 words)
    fn pmaddubsw_64(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            // First operand is treated as unsigned bytes, second as signed bytes.
            // The two products and their sum must be accumulated at i32 width: each
            // product can reach 255*127 = 32385 and the sum 255*-128*2 = -65280,
            // both of which overflow i16 before the signed-saturate to a word.
            let a0 = ((a >> (i * 16)) & 0xFF) as u8 as i32;
            let a1 = ((a >> (i * 16 + 8)) & 0xFF) as u8 as i32;
            let b0 = ((b >> (i * 16)) & 0xFF) as i8 as i32;
            let b1 = ((b >> (i * 16 + 8)) & 0xFF) as i8 as i32;
            let prod = (a0 * b0 + a1 * b1).clamp(-32768, 32767) as u16;
            result |= (prod as u64) << (i * 16);
        }
        result
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_psign(
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

        let elem_bits = match opcode {
            0x08 => 8,  // PSIGNB
            0x09 => 16, // PSIGNW
            0x0A => 32, // PSIGND
            _ => unreachable!(),
        };

        self.regs.xmm[xmm_dst][0] = self.psign_64(src1_lo, src2_lo, elem_bits);
        self.regs.xmm[xmm_dst][1] = self.psign_64(src1_hi, src2_hi, elem_bits);

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
            self.regs.ymm_high[xmm_dst][0] = self.psign_64(src1_hi2, src2_hi2, elem_bits);
            self.regs.ymm_high[xmm_dst][1] = self.psign_64(src1_hi3, src2_hi3, elem_bits);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn psign_64(&self, a: u64, b: u64, elem_bits: u32) -> u64 {
        let elem_count = 64 / elem_bits;
        let mask = (1u64 << elem_bits) - 1;
        let sign_bit = 1u64 << (elem_bits - 1);
        let mut result = 0u64;
        for i in 0..elem_count {
            let shift = i * elem_bits;
            let av = (a >> shift) & mask;
            let bv = (b >> shift) & mask;
            let rv = if bv == 0 {
                0
            } else if bv & sign_bit != 0 {
                // b is negative, negate a
                ((!av).wrapping_add(1)) & mask
            } else {
                av
            };
            result |= rv << shift;
        }
        result
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_pmulhrsw(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
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

        self.regs.xmm[xmm_dst][0] = self.pmulhrsw_64(src1_lo, src2_lo);
        self.regs.xmm[xmm_dst][1] = self.pmulhrsw_64(src1_hi, src2_hi);

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
            self.regs.ymm_high[xmm_dst][0] = self.pmulhrsw_64(src1_hi2, src2_hi2);
            self.regs.ymm_high[xmm_dst][1] = self.pmulhrsw_64(src1_hi3, src2_hi3);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn pmulhrsw_64(&self, a: u64, b: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let av = ((a >> (i * 16)) & 0xFFFF) as i16 as i32;
            let bv = ((b >> (i * 16)) & 0xFFFF) as i16 as i32;
            // temp = (a * b + 0x4000) >> 15
            let temp = ((av * bv + 0x4000) >> 15) as u16;
            result |= (temp as u64) << (i * 16);
        }
        result
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_phminposuw(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VPHMINPOSUW requires VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };

        let words = [
            (src_lo & 0xFFFF) as u16,
            ((src_lo >> 16) & 0xFFFF) as u16,
            ((src_lo >> 32) & 0xFFFF) as u16,
            ((src_lo >> 48) & 0xFFFF) as u16,
            (src_hi & 0xFFFF) as u16,
            ((src_hi >> 16) & 0xFFFF) as u16,
            ((src_hi >> 32) & 0xFFFF) as u16,
            ((src_hi >> 48) & 0xFFFF) as u16,
        ];

        let mut min_val = words[0];
        let mut min_idx = 0u16;
        for i in 1..8 {
            if words[i] < min_val {
                min_val = words[i];
                min_idx = i as u16;
            }
        }

        self.regs.xmm[xmm_dst][0] = (min_val as u64) | ((min_idx as u64) << 16);
        self.regs.xmm[xmm_dst][1] = 0;
        self.regs.ymm_high[xmm_dst][0] = 0;
        self.regs.ymm_high[xmm_dst][1] = 0;

        if vex_l == 1 {
            return Err(Error::Emulator(
                "VPHMINPOSUW does not support VEX.256".to_string(),
            ));
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_mpsadbw(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        let (src2_lo, src2_hi, src2_hi2, src2_hi3) = if is_memory {
            (
                self.read_mem(addr, 8)?,
                self.read_mem(addr + 8, 8)?,
                if vex_l == 1 {
                    self.read_mem(addr + 16, 8)?
                } else {
                    0
                },
                if vex_l == 1 {
                    self.read_mem(addr + 24, 8)?
                } else {
                    0
                },
            )
        } else {
            (
                self.regs.xmm[rm as usize][0],
                self.regs.xmm[rm as usize][1],
                if vex_l == 1 {
                    self.regs.ymm_high[rm as usize][0]
                } else {
                    0
                },
                if vex_l == 1 {
                    self.regs.ymm_high[rm as usize][1]
                } else {
                    0
                },
            )
        };

        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];

        let (dst_lo, dst_hi) =
            self.mpsadbw_lane(src1_lo, src1_hi, src2_lo, src2_hi, imm8 & 0x07, imm8 & 0x03);
        self.regs.xmm[xmm_dst][0] = dst_lo;
        self.regs.xmm[xmm_dst][1] = dst_hi;

        if vex_l == 1 {
            let src1_hi2 = self.regs.ymm_high[xmm_src1][0];
            let src1_hi3 = self.regs.ymm_high[xmm_src1][1];
            let (dst_hi2, dst_hi3) = self.mpsadbw_lane(
                src1_hi2,
                src1_hi3,
                src2_hi2,
                src2_hi3,
                imm8 & 0x07,
                (imm8 >> 3) & 0x03,
            );
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn mpsadbw_lane(
        &self,
        s1_lo: u64,
        s1_hi: u64,
        s2_lo: u64,
        s2_hi: u64,
        blk1_sel: u8,
        blk2_sel: u8,
    ) -> (u64, u64) {
        let mut src1 = [0u8; 16];
        let mut src2 = [0u8; 16];
        src1[0..8].copy_from_slice(&s1_lo.to_le_bytes());
        src1[8..16].copy_from_slice(&s1_hi.to_le_bytes());
        src2[0..8].copy_from_slice(&s2_lo.to_le_bytes());
        src2[8..16].copy_from_slice(&s2_hi.to_le_bytes());

        let blk1_offset = ((blk1_sel >> 2) & 1) as usize * 4;
        let blk2_offset = (blk2_sel & 0x3) as usize * 4;

        let mut results = [0u16; 8];
        for i in 0..8 {
            let mut sum = 0u16;
            for k in 0..4 {
                let a = src1[blk1_offset + i + k] as i16;
                let b = src2[blk2_offset + k] as i16;
                sum = sum.wrapping_add((a - b).abs() as u16);
            }
            results[i] = sum;
        }

        let mut lo = 0u64;
        let mut hi = 0u64;
        for i in 0..4 {
            lo |= (results[i] as u64) << (i * 16);
        }
        for i in 0..4 {
            hi |= (results[i + 4] as u64) << (i * 16);
        }
        (lo, hi)
    }
}
