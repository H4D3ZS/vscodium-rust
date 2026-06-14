//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vinsertf128(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        // Read 128-bit source from xmm or memory
        let (insert_lo, insert_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };

        // Copy src1 to dst first
        self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src1][0];
        self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
        self.regs.ymm_high[xmm_dst][0] = self.regs.ymm_high[xmm_src1][0];
        self.regs.ymm_high[xmm_dst][1] = self.regs.ymm_high[xmm_src1][1];

        // Insert into selected lane based on imm8[0]
        if (imm8 & 1) == 0 {
            // Insert into low 128 bits
            self.regs.xmm[xmm_dst][0] = insert_lo;
            self.regs.xmm[xmm_dst][1] = insert_hi;
        } else {
            // Insert into high 128 bits
            self.regs.ymm_high[xmm_dst][0] = insert_lo;
            self.regs.ymm_high[xmm_dst][1] = insert_hi;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vextractf128(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_src = reg as usize;

        // Select lane based on imm8[0]
        let (extract_lo, extract_hi) = if (imm8 & 1) == 0 {
            // Extract low 128 bits
            (self.regs.xmm[xmm_src][0], self.regs.xmm[xmm_src][1])
        } else {
            // Extract high 128 bits
            (
                self.regs.ymm_high[xmm_src][0],
                self.regs.ymm_high[xmm_src][1],
            )
        };

        if is_memory {
            self.write_mem(addr, extract_lo, 8)?;
            self.write_mem(addr + 8, extract_hi, 8)?;
        } else {
            let xmm_dst = rm as usize;
            self.regs.xmm[xmm_dst][0] = extract_lo;
            self.regs.xmm[xmm_dst][1] = extract_hi;
            // VEX clears upper bits
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vperm2f128(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        // Get all 4 source lanes (2 from src1, 2 from src2)
        let src1_lo = (self.regs.xmm[xmm_src1][0], self.regs.xmm[xmm_src1][1]);
        let src1_hi = (
            self.regs.ymm_high[xmm_src1][0],
            self.regs.ymm_high[xmm_src1][1],
        );
        let (src2_lo, src2_hi) = if is_memory {
            (
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?),
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?),
            )
        } else {
            (
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1]),
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                ),
            )
        };

        // Select result low 128 bits based on imm8[1:0]
        let result_lo = if (imm8 & 0x08) != 0 {
            // Zero this lane
            (0u64, 0u64)
        } else {
            match imm8 & 0x03 {
                0 => src1_lo,
                1 => src1_hi,
                2 => src2_lo,
                3 => src2_hi,
                _ => unreachable!(),
            }
        };

        // Select result high 128 bits based on imm8[5:4]
        let result_hi = if (imm8 & 0x80) != 0 {
            // Zero this lane
            (0u64, 0u64)
        } else {
            match (imm8 >> 4) & 0x03 {
                0 => src1_lo,
                1 => src1_hi,
                2 => src2_lo,
                3 => src2_hi,
                _ => unreachable!(),
            }
        };

        self.regs.xmm[xmm_dst][0] = result_lo.0;
        self.regs.xmm[xmm_dst][1] = result_lo.1;
        self.regs.ymm_high[xmm_dst][0] = result_hi.0;
        self.regs.ymm_high[xmm_dst][1] = result_hi.1;

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}
