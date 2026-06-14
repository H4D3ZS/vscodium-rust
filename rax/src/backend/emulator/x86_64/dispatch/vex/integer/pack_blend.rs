//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_pblendvb(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;
        let xmm_mask = ((imm >> 4) & 0xF) as usize;

        let (src2_lo, src2_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];
        let mask_lo = self.regs.xmm[xmm_mask][0];
        let mask_hi = self.regs.xmm[xmm_mask][1];

        self.regs.xmm[xmm_dst][0] = self.blend_bytes(src1_lo, src2_lo, mask_lo);
        self.regs.xmm[xmm_dst][1] = self.blend_bytes(src1_hi, src2_hi, mask_hi);

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
            let mask_hi2 = self.regs.ymm_high[xmm_mask][0];
            let mask_hi3 = self.regs.ymm_high[xmm_mask][1];
            self.regs.ymm_high[xmm_dst][0] = self.blend_bytes(src1_hi2, src2_hi2, mask_hi2);
            self.regs.ymm_high[xmm_dst][1] = self.blend_bytes(src1_hi3, src2_hi3, mask_hi3);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn blend_bytes(&self, a: u64, b: u64, mask: u64) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let m = (mask >> (i * 8 + 7)) & 1;
            let v = if m != 0 {
                (b >> (i * 8)) & 0xFF
            } else {
                (a >> (i * 8)) & 0xFF
            };
            result |= v << (i * 8);
        }
        result
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_packusdw(
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

        self.regs.xmm[xmm_dst][0] = self.pack_usdw(src1_lo, src1_hi);
        self.regs.xmm[xmm_dst][1] = self.pack_usdw(src2_lo, src2_hi);

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
            self.regs.ymm_high[xmm_dst][0] = self.pack_usdw(src1_hi2, src1_hi3);
            self.regs.ymm_high[xmm_dst][1] = self.pack_usdw(src2_hi2, src2_hi3);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_packsswb(
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

        let (dst_lo, dst_hi) = self.pack_sswb_128(src1_lo, src1_hi, src2_lo, src2_hi);
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
            let (dst_hi2, dst_hi3) = self.pack_sswb_128(src1_hi2, src1_hi3, src2_hi2, src2_hi3);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_packssdw(
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

        let (dst_lo, dst_hi) = self.pack_ssdw_128(src1_lo, src1_hi, src2_lo, src2_hi);
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
            let (dst_hi2, dst_hi3) = self.pack_ssdw_128(src1_hi2, src1_hi3, src2_hi2, src2_hi3);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_packuswb(
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

        let (dst_lo, dst_hi) = self.pack_uswb_128(src1_lo, src1_hi, src2_lo, src2_hi);
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
            let (dst_hi2, dst_hi3) = self.pack_uswb_128(src1_hi2, src1_hi3, src2_hi2, src2_hi3);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_blendw(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        let (src2_lo, src2_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];

        self.regs.xmm[xmm_dst][0] = self.blend_words(src1_lo, src2_lo, imm8);
        self.regs.xmm[xmm_dst][1] = self.blend_words(src1_hi, src2_hi, imm8 >> 4);

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
            self.regs.ymm_high[xmm_dst][0] = self.blend_words(src1_hi2, src2_hi2, imm8);
            self.regs.ymm_high[xmm_dst][1] = self.blend_words(src1_hi3, src2_hi3, imm8 >> 4);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_blendd(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        let (src2_lo, src2_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let src1_lo = self.regs.xmm[xmm_src1][0];
        let src1_hi = self.regs.xmm[xmm_src1][1];

        self.regs.xmm[xmm_dst][0] = self.blend_dwords(src1_lo, src2_lo, imm8);
        self.regs.xmm[xmm_dst][1] = self.blend_dwords(src1_hi, src2_hi, imm8 >> 2);

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
            self.regs.ymm_high[xmm_dst][0] = self.blend_dwords(src1_hi2, src2_hi2, imm8 >> 4);
            self.regs.ymm_high[xmm_dst][1] = self.blend_dwords(src1_hi3, src2_hi3, imm8 >> 6);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn pack_usdw(&self, lo: u64, hi: u64) -> u64 {
        let saturate = |v: i32| -> u16 {
            if v < 0 {
                0u16
            } else if v > 65535 {
                0xFFFFu16
            } else {
                v as u16
            }
        };
        let w0 = saturate(lo as i32) as u64;
        let w1 = saturate((lo >> 32) as i32) as u64;
        let w2 = saturate(hi as i32) as u64;
        let w3 = saturate((hi >> 32) as i32) as u64;
        w0 | (w1 << 16) | (w2 << 32) | (w3 << 48)
    }

    fn pack_sswb_128(&self, s1_lo: u64, s1_hi: u64, s2_lo: u64, s2_hi: u64) -> (u64, u64) {
        let mut bytes = [0u8; 16];
        for i in 0..8 {
            let word = if i < 4 {
                ((s1_lo >> (i * 16)) & 0xFFFF) as i16
            } else {
                ((s1_hi >> ((i - 4) * 16)) & 0xFFFF) as i16
            };
            let v = if word > 127 {
                127
            } else if word < -128 {
                -128
            } else {
                word
            } as i8;
            bytes[i] = v as u8;
        }
        for i in 0..8 {
            let word = if i < 4 {
                ((s2_lo >> (i * 16)) & 0xFFFF) as i16
            } else {
                ((s2_hi >> ((i - 4) * 16)) & 0xFFFF) as i16
            };
            let v = if word > 127 {
                127
            } else if word < -128 {
                -128
            } else {
                word
            } as i8;
            bytes[8 + i] = v as u8;
        }
        let lo = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let hi = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        (lo, hi)
    }

    fn pack_ssdw_128(&self, s1_lo: u64, s1_hi: u64, s2_lo: u64, s2_hi: u64) -> (u64, u64) {
        let mut words = [0u16; 8];
        let src1 = [
            s1_lo as i32,
            (s1_lo >> 32) as i32,
            s1_hi as i32,
            (s1_hi >> 32) as i32,
        ];
        let src2 = [
            s2_lo as i32,
            (s2_lo >> 32) as i32,
            s2_hi as i32,
            (s2_hi >> 32) as i32,
        ];
        for i in 0..4 {
            let v = src1[i];
            let s = if v > 32767 {
                32767
            } else if v < -32768 {
                -32768
            } else {
                v
            } as i16;
            words[i] = s as u16;
        }
        for i in 0..4 {
            let v = src2[i];
            let s = if v > 32767 {
                32767
            } else if v < -32768 {
                -32768
            } else {
                v
            } as i16;
            words[4 + i] = s as u16;
        }
        let mut lo = 0u64;
        let mut hi = 0u64;
        for i in 0..4 {
            lo |= (words[i] as u64) << (i * 16);
        }
        for i in 0..4 {
            hi |= (words[4 + i] as u64) << (i * 16);
        }
        (lo, hi)
    }

    fn pack_uswb_128(&self, s1_lo: u64, s1_hi: u64, s2_lo: u64, s2_hi: u64) -> (u64, u64) {
        let mut bytes = [0u8; 16];
        for i in 0..8 {
            let word = if i < 4 {
                ((s1_lo >> (i * 16)) & 0xFFFF) as i16
            } else {
                ((s1_hi >> ((i - 4) * 16)) & 0xFFFF) as i16
            };
            let v = if word < 0 {
                0
            } else if word > 255 {
                255
            } else {
                word
            } as u8;
            bytes[i] = v;
        }
        for i in 0..8 {
            let word = if i < 4 {
                ((s2_lo >> (i * 16)) & 0xFFFF) as i16
            } else {
                ((s2_hi >> ((i - 4) * 16)) & 0xFFFF) as i16
            };
            let v = if word < 0 {
                0
            } else if word > 255 {
                255
            } else {
                word
            } as u8;
            bytes[8 + i] = v;
        }
        let lo = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let hi = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        (lo, hi)
    }

    fn blend_words(&self, a: u64, b: u64, mask: u8) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let select = (mask >> i) & 1;
            let word = if select != 0 {
                (b >> (i * 16)) & 0xFFFF
            } else {
                (a >> (i * 16)) & 0xFFFF
            };
            result |= word << (i * 16);
        }
        result
    }

    fn blend_dwords(&self, a: u64, b: u64, mask: u8) -> u64 {
        let mut result = 0u64;
        for i in 0..2 {
            let select = (mask >> i) & 1;
            let dword = if select != 0 {
                (b >> (i * 32)) & 0xFFFF_FFFF
            } else {
                (a >> (i * 32)) & 0xFFFF_FFFF
            };
            result |= dword << (i * 32);
        }
        result
    }
}
