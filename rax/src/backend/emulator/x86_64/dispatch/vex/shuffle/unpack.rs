//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_unpack(
        &mut self,
        ctx: &mut InsnContext,
        vex_pp: u8,
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

        let is_high = opcode == 0x15;
        let is_pd = vex_pp == 1; // 66 prefix = PD

        if is_pd {
            // Double precision - interleave 64-bit elements
            if is_high {
                // VUNPCKHPD: dst[0]=src1[1], dst[1]=src2[1]
                self.regs.xmm[xmm_dst][0] = src1_hi;
                self.regs.xmm[xmm_dst][1] = src2_hi;
            } else {
                // VUNPCKLPD: dst[0]=src1[0], dst[1]=src2[0]
                self.regs.xmm[xmm_dst][0] = src1_lo;
                self.regs.xmm[xmm_dst][1] = src2_lo;
            }

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

                if is_high {
                    self.regs.ymm_high[xmm_dst][0] = src1_hi3;
                    self.regs.ymm_high[xmm_dst][1] = src2_hi3;
                } else {
                    self.regs.ymm_high[xmm_dst][0] = src1_hi2;
                    self.regs.ymm_high[xmm_dst][1] = src2_hi2;
                }
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        } else {
            // Single precision - interleave 32-bit elements
            if is_high {
                // VUNPCKHPS: interleave high singles
                let d2 = src1_hi as u32;
                let d3 = (src1_hi >> 32) as u32;
                let s2 = src2_hi as u32;
                let s3 = (src2_hi >> 32) as u32;
                self.regs.xmm[xmm_dst][0] = d2 as u64 | ((s2 as u64) << 32);
                self.regs.xmm[xmm_dst][1] = d3 as u64 | ((s3 as u64) << 32);
            } else {
                // VUNPCKLPS: interleave low singles
                let d0 = src1_lo as u32;
                let d1 = (src1_lo >> 32) as u32;
                let s0 = src2_lo as u32;
                let s1 = (src2_lo >> 32) as u32;
                self.regs.xmm[xmm_dst][0] = d0 as u64 | ((s0 as u64) << 32);
                self.regs.xmm[xmm_dst][1] = d1 as u64 | ((s1 as u64) << 32);
            }

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

                if is_high {
                    let d6 = src1_hi3 as u32;
                    let d7 = (src1_hi3 >> 32) as u32;
                    let s6 = src2_hi3 as u32;
                    let s7 = (src2_hi3 >> 32) as u32;
                    self.regs.ymm_high[xmm_dst][0] = d6 as u64 | ((s6 as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = d7 as u64 | ((s7 as u64) << 32);
                } else {
                    let d4 = src1_hi2 as u32;
                    let d5 = (src1_hi2 >> 32) as u32;
                    let s4 = src2_hi2 as u32;
                    let s5 = (src2_hi2 >> 32) as u32;
                    self.regs.ymm_high[xmm_dst][0] = d4 as u64 | ((s4 as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = d5 as u64 | ((s5 as u64) << 32);
                }
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_punpck(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        // Get source 2 from rm
        let (src2_lo, src2_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };

        // Get source 1 from vvvv
        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];

        match opcode {
            // PUNPCKLBW: interleave low bytes
            0x60 => {
                self.regs.xmm[xmm_dst][0] = self.unpack_low_bytes(src1_lo, src2_lo);
                self.regs.xmm[xmm_dst][1] = self.unpack_high_bytes(src1_lo, src2_lo);
            }
            // PUNPCKLWD: interleave low words
            0x61 => {
                self.regs.xmm[xmm_dst][0] = self.unpack_low_words(src1_lo, src2_lo);
                self.regs.xmm[xmm_dst][1] = self.unpack_high_words(src1_lo, src2_lo);
            }
            // PUNPCKLDQ: interleave low dwords
            0x62 => {
                // Low dwords from each qword interleaved
                let d0_src1 = src1_lo as u32;
                let d0_src2 = src2_lo as u32;
                let d1_src1 = (src1_lo >> 32) as u32;
                let d1_src2 = (src2_lo >> 32) as u32;
                self.regs.xmm[xmm_dst][0] = (d0_src1 as u64) | ((d0_src2 as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (d1_src1 as u64) | ((d1_src2 as u64) << 32);
            }
            // PACKSSWB: pack words to bytes with signed saturation
            0x63 => {
                self.regs.xmm[xmm_dst][0] = self.pack_sswb(src1_lo, src1_hi);
                self.regs.xmm[xmm_dst][1] = self.pack_sswb(src2_lo, src2_hi);
            }
            // PCMPGTB: compare bytes for greater than
            0x64 => {
                self.regs.xmm[xmm_dst][0] = self.cmp_gt_bytes(src1_lo, src2_lo);
                self.regs.xmm[xmm_dst][1] = self.cmp_gt_bytes(src1_hi, src2_hi);
            }
            // PCMPGTW: compare words for greater than
            0x65 => {
                self.regs.xmm[xmm_dst][0] = self.cmp_gt_words(src1_lo, src2_lo);
                self.regs.xmm[xmm_dst][1] = self.cmp_gt_words(src1_hi, src2_hi);
            }
            // PCMPGTD: compare dwords for greater than
            0x66 => {
                self.regs.xmm[xmm_dst][0] = self.cmp_gt_dwords(src1_lo, src2_lo);
                self.regs.xmm[xmm_dst][1] = self.cmp_gt_dwords(src1_hi, src2_hi);
            }
            // PACKUSWB: pack words to unsigned bytes with saturation
            0x67 => {
                self.regs.xmm[xmm_dst][0] = self.pack_uswb(src1_lo, src1_hi);
                self.regs.xmm[xmm_dst][1] = self.pack_uswb(src2_lo, src2_hi);
            }
            // PUNPCKHBW: interleave high bytes
            0x68 => {
                self.regs.xmm[xmm_dst][0] = self.unpack_low_bytes(src1_hi, src2_hi);
                self.regs.xmm[xmm_dst][1] = self.unpack_high_bytes(src1_hi, src2_hi);
            }
            // PUNPCKHWD: interleave high words
            0x69 => {
                self.regs.xmm[xmm_dst][0] = self.unpack_low_words(src1_hi, src2_hi);
                self.regs.xmm[xmm_dst][1] = self.unpack_high_words(src1_hi, src2_hi);
            }
            // PUNPCKHDQ: interleave high dwords
            0x6A => {
                let d0_src1 = src1_hi as u32;
                let d0_src2 = src2_hi as u32;
                let d1_src1 = (src1_hi >> 32) as u32;
                let d1_src2 = (src2_hi >> 32) as u32;
                self.regs.xmm[xmm_dst][0] = (d0_src1 as u64) | ((d0_src2 as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (d1_src1 as u64) | ((d1_src2 as u64) << 32);
            }
            // PACKSSDW: pack dwords to words with signed saturation
            0x6B => {
                self.regs.xmm[xmm_dst][0] = self.pack_ssdw(src1_lo, src1_hi);
                self.regs.xmm[xmm_dst][1] = self.pack_ssdw(src2_lo, src2_hi);
            }
            // PUNPCKLQDQ: interleave low qwords
            0x6C => {
                self.regs.xmm[xmm_dst][0] = src1_lo;
                self.regs.xmm[xmm_dst][1] = src2_lo;
            }
            // PUNPCKHQDQ: interleave high qwords
            0x6D => {
                self.regs.xmm[xmm_dst][0] = src1_hi;
                self.regs.xmm[xmm_dst][1] = src2_hi;
            }
            _ => unreachable!(),
        }

        // Handle YMM (256-bit)
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

            match opcode {
                0x60 => {
                    self.regs.ymm_high[xmm_dst][0] = self.unpack_low_bytes(src1_hi2, src2_hi2);
                    self.regs.ymm_high[xmm_dst][1] = self.unpack_high_bytes(src1_hi2, src2_hi2);
                }
                0x61 => {
                    self.regs.ymm_high[xmm_dst][0] = self.unpack_low_words(src1_hi2, src2_hi2);
                    self.regs.ymm_high[xmm_dst][1] = self.unpack_high_words(src1_hi2, src2_hi2);
                }
                0x62 => {
                    let d0_src1 = src1_hi2 as u32;
                    let d0_src2 = src2_hi2 as u32;
                    let d1_src1 = (src1_hi2 >> 32) as u32;
                    let d1_src2 = (src2_hi2 >> 32) as u32;
                    self.regs.ymm_high[xmm_dst][0] = (d0_src1 as u64) | ((d0_src2 as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = (d1_src1 as u64) | ((d1_src2 as u64) << 32);
                }
                0x63 => {
                    self.regs.ymm_high[xmm_dst][0] = self.pack_sswb(src1_hi2, src1_hi3);
                    self.regs.ymm_high[xmm_dst][1] = self.pack_sswb(src2_hi2, src2_hi3);
                }
                0x64 => {
                    self.regs.ymm_high[xmm_dst][0] = self.cmp_gt_bytes(src1_hi2, src2_hi2);
                    self.regs.ymm_high[xmm_dst][1] = self.cmp_gt_bytes(src1_hi3, src2_hi3);
                }
                0x65 => {
                    self.regs.ymm_high[xmm_dst][0] = self.cmp_gt_words(src1_hi2, src2_hi2);
                    self.regs.ymm_high[xmm_dst][1] = self.cmp_gt_words(src1_hi3, src2_hi3);
                }
                0x66 => {
                    self.regs.ymm_high[xmm_dst][0] = self.cmp_gt_dwords(src1_hi2, src2_hi2);
                    self.regs.ymm_high[xmm_dst][1] = self.cmp_gt_dwords(src1_hi3, src2_hi3);
                }
                0x67 => {
                    self.regs.ymm_high[xmm_dst][0] = self.pack_uswb(src1_hi2, src1_hi3);
                    self.regs.ymm_high[xmm_dst][1] = self.pack_uswb(src2_hi2, src2_hi3);
                }
                0x68 => {
                    self.regs.ymm_high[xmm_dst][0] = self.unpack_low_bytes(src1_hi3, src2_hi3);
                    self.regs.ymm_high[xmm_dst][1] = self.unpack_high_bytes(src1_hi3, src2_hi3);
                }
                0x69 => {
                    self.regs.ymm_high[xmm_dst][0] = self.unpack_low_words(src1_hi3, src2_hi3);
                    self.regs.ymm_high[xmm_dst][1] = self.unpack_high_words(src1_hi3, src2_hi3);
                }
                0x6A => {
                    let d0_src1 = src1_hi3 as u32;
                    let d0_src2 = src2_hi3 as u32;
                    let d1_src1 = (src1_hi3 >> 32) as u32;
                    let d1_src2 = (src2_hi3 >> 32) as u32;
                    self.regs.ymm_high[xmm_dst][0] = (d0_src1 as u64) | ((d0_src2 as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = (d1_src1 as u64) | ((d1_src2 as u64) << 32);
                }
                0x6B => {
                    self.regs.ymm_high[xmm_dst][0] = self.pack_ssdw(src1_hi2, src1_hi3);
                    self.regs.ymm_high[xmm_dst][1] = self.pack_ssdw(src2_hi2, src2_hi3);
                }
                0x6C => {
                    self.regs.ymm_high[xmm_dst][0] = src1_hi2;
                    self.regs.ymm_high[xmm_dst][1] = src2_hi2;
                }
                0x6D => {
                    self.regs.ymm_high[xmm_dst][0] = src1_hi3;
                    self.regs.ymm_high[xmm_dst][1] = src2_hi3;
                }
                _ => {}
            }
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    // Helper: unpack low 4 bytes from two qwords interleaved
    fn unpack_low_bytes(&self, a: u64, b: u64) -> u64 {
        let a0 = (a >> 0) & 0xFF;
        let b0 = (b >> 0) & 0xFF;
        let a1 = (a >> 8) & 0xFF;
        let b1 = (b >> 8) & 0xFF;
        let a2 = (a >> 16) & 0xFF;
        let b2 = (b >> 16) & 0xFF;
        let a3 = (a >> 24) & 0xFF;
        let b3 = (b >> 24) & 0xFF;
        a0 | (b0 << 8) | (a1 << 16) | (b1 << 24) | (a2 << 32) | (b2 << 40) | (a3 << 48) | (b3 << 56)
    }

    // Helper: unpack high 4 bytes from two qwords interleaved
    fn unpack_high_bytes(&self, a: u64, b: u64) -> u64 {
        let a4 = (a >> 32) & 0xFF;
        let b4 = (b >> 32) & 0xFF;
        let a5 = (a >> 40) & 0xFF;
        let b5 = (b >> 40) & 0xFF;
        let a6 = (a >> 48) & 0xFF;
        let b6 = (b >> 48) & 0xFF;
        let a7 = (a >> 56) & 0xFF;
        let b7 = (b >> 56) & 0xFF;
        a4 | (b4 << 8) | (a5 << 16) | (b5 << 24) | (a6 << 32) | (b6 << 40) | (a7 << 48) | (b7 << 56)
    }

    // Helper: unpack low 2 words from two qwords interleaved
    fn unpack_low_words(&self, a: u64, b: u64) -> u64 {
        let a0 = a & 0xFFFF;
        let b0 = b & 0xFFFF;
        let a1 = (a >> 16) & 0xFFFF;
        let b1 = (b >> 16) & 0xFFFF;
        a0 | (b0 << 16) | (a1 << 32) | (b1 << 48)
    }

    // Helper: unpack high 2 words from two qwords interleaved
    fn unpack_high_words(&self, a: u64, b: u64) -> u64 {
        let a2 = (a >> 32) & 0xFFFF;
        let b2 = (b >> 32) & 0xFFFF;
        let a3 = (a >> 48) & 0xFFFF;
        let b3 = (b >> 48) & 0xFFFF;
        a2 | (b2 << 16) | (a3 << 32) | (b3 << 48)
    }

    // Helper: pack words to bytes with signed saturation
    fn pack_sswb(&self, lo: u64, hi: u64) -> u64 {
        let saturate = |v: i16| -> u8 {
            if v < -128 {
                0x80u8
            } else if v > 127 {
                0x7Fu8
            } else {
                v as i8 as u8
            }
        };
        let b0 = saturate(lo as i16) as u64;
        let b1 = saturate((lo >> 16) as i16) as u64;
        let b2 = saturate((lo >> 32) as i16) as u64;
        let b3 = saturate((lo >> 48) as i16) as u64;
        let b4 = saturate(hi as i16) as u64;
        let b5 = saturate((hi >> 16) as i16) as u64;
        let b6 = saturate((hi >> 32) as i16) as u64;
        let b7 = saturate((hi >> 48) as i16) as u64;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24) | (b4 << 32) | (b5 << 40) | (b6 << 48) | (b7 << 56)
    }

    // Helper: pack words to unsigned bytes with saturation
    fn pack_uswb(&self, lo: u64, hi: u64) -> u64 {
        let saturate = |v: i16| -> u8 {
            if v < 0 {
                0u8
            } else if v > 255 {
                0xFFu8
            } else {
                v as u8
            }
        };
        let b0 = saturate(lo as i16) as u64;
        let b1 = saturate((lo >> 16) as i16) as u64;
        let b2 = saturate((lo >> 32) as i16) as u64;
        let b3 = saturate((lo >> 48) as i16) as u64;
        let b4 = saturate(hi as i16) as u64;
        let b5 = saturate((hi >> 16) as i16) as u64;
        let b6 = saturate((hi >> 32) as i16) as u64;
        let b7 = saturate((hi >> 48) as i16) as u64;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24) | (b4 << 32) | (b5 << 40) | (b6 << 48) | (b7 << 56)
    }

    // Helper: pack dwords to words with signed saturation
    fn pack_ssdw(&self, lo: u64, hi: u64) -> u64 {
        let saturate = |v: i32| -> u16 {
            if v < -32768 {
                0x8000u16
            } else if v > 32767 {
                0x7FFFu16
            } else {
                v as i16 as u16
            }
        };
        let w0 = saturate(lo as i32) as u64;
        let w1 = saturate((lo >> 32) as i32) as u64;
        let w2 = saturate(hi as i32) as u64;
        let w3 = saturate((hi >> 32) as i32) as u64;
        w0 | (w1 << 16) | (w2 << 32) | (w3 << 48)
    }
}
