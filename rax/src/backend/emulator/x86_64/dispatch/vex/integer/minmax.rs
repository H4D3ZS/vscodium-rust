//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_pminmax_sb(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        is_max: bool,
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

        self.regs.xmm[xmm_dst][0] = self.minmax_sb_64(src1_lo, src2_lo, is_max);
        self.regs.xmm[xmm_dst][1] = self.minmax_sb_64(src1_hi, src2_hi, is_max);

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
            self.regs.ymm_high[xmm_dst][0] = self.minmax_sb_64(src1_hi2, src2_hi2, is_max);
            self.regs.ymm_high[xmm_dst][1] = self.minmax_sb_64(src1_hi3, src2_hi3, is_max);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn minmax_sb_64(&self, a: u64, b: u64, is_max: bool) -> u64 {
        let mut result = 0u64;
        for i in 0..8 {
            let va = ((a >> (i * 8)) & 0xFF) as i8;
            let vb = ((b >> (i * 8)) & 0xFF) as i8;
            let v = if is_max { va.max(vb) } else { va.min(vb) } as u8;
            result |= (v as u64) << (i * 8);
        }
        result
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_pminmax_sd(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        is_max: bool,
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

        self.regs.xmm[xmm_dst][0] = self.minmax_sd_64(src1_lo, src2_lo, is_max);
        self.regs.xmm[xmm_dst][1] = self.minmax_sd_64(src1_hi, src2_hi, is_max);

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
            self.regs.ymm_high[xmm_dst][0] = self.minmax_sd_64(src1_hi2, src2_hi2, is_max);
            self.regs.ymm_high[xmm_dst][1] = self.minmax_sd_64(src1_hi3, src2_hi3, is_max);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn minmax_sd_64(&self, a: u64, b: u64, is_max: bool) -> u64 {
        let a0 = a as i32;
        let a1 = (a >> 32) as i32;
        let b0 = b as i32;
        let b1 = (b >> 32) as i32;
        let r0 = if is_max { a0.max(b0) } else { a0.min(b0) } as u32;
        let r1 = if is_max { a1.max(b1) } else { a1.min(b1) } as u32;
        (r0 as u64) | ((r1 as u64) << 32)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_pminmax_uw(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        is_max: bool,
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

        self.regs.xmm[xmm_dst][0] = self.minmax_uw_64(src1_lo, src2_lo, is_max);
        self.regs.xmm[xmm_dst][1] = self.minmax_uw_64(src1_hi, src2_hi, is_max);

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
            self.regs.ymm_high[xmm_dst][0] = self.minmax_uw_64(src1_hi2, src2_hi2, is_max);
            self.regs.ymm_high[xmm_dst][1] = self.minmax_uw_64(src1_hi3, src2_hi3, is_max);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn minmax_uw_64(&self, a: u64, b: u64, is_max: bool) -> u64 {
        let mut result = 0u64;
        for i in 0..4 {
            let va = ((a >> (i * 16)) & 0xFFFF) as u16;
            let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
            let v = if is_max { va.max(vb) } else { va.min(vb) };
            result |= (v as u64) << (i * 16);
        }
        result
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_pminmax_ud(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        is_max: bool,
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

        self.regs.xmm[xmm_dst][0] = self.minmax_ud_64(src1_lo, src2_lo, is_max);
        self.regs.xmm[xmm_dst][1] = self.minmax_ud_64(src1_hi, src2_hi, is_max);

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
            self.regs.ymm_high[xmm_dst][0] = self.minmax_ud_64(src1_hi2, src2_hi2, is_max);
            self.regs.ymm_high[xmm_dst][1] = self.minmax_ud_64(src1_hi3, src2_hi3, is_max);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn minmax_ud_64(&self, a: u64, b: u64, is_max: bool) -> u64 {
        let a0 = a as u32;
        let a1 = (a >> 32) as u32;
        let b0 = b as u32;
        let b1 = (b >> 32) as u32;
        let r0 = if is_max { a0.max(b0) } else { a0.min(b0) };
        let r1 = if is_max { a1.max(b1) } else { a1.min(b1) };
        (r0 as u64) | ((r1 as u64) << 32)
    }
}
