//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_permilps_reg(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src = vvvv as usize;

        let (ctrl_lo, ctrl_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let src_lo = self.regs.xmm[xmm_src][0];
        let src_hi = self.regs.xmm[xmm_src][1];

        let (dst_lo, dst_hi) = self.permilps_128(src_lo, src_hi, ctrl_lo, ctrl_hi);
        self.regs.xmm[xmm_dst][0] = dst_lo;
        self.regs.xmm[xmm_dst][1] = dst_hi;

        if vex_l == 1 {
            let (ctrl_hi2, ctrl_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let src_hi2 = self.regs.ymm_high[xmm_src][0];
            let src_hi3 = self.regs.ymm_high[xmm_src][1];
            let (dst_hi2, dst_hi3) = self.permilps_128(src_hi2, src_hi3, ctrl_hi2, ctrl_hi3);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn permilps_128(&self, src_lo: u64, src_hi: u64, ctrl_lo: u64, ctrl_hi: u64) -> (u64, u64) {
        let floats = [
            src_lo as u32,
            (src_lo >> 32) as u32,
            src_hi as u32,
            (src_hi >> 32) as u32,
        ];
        let r0 = floats[(ctrl_lo & 3) as usize];
        let r1 = floats[((ctrl_lo >> 32) & 3) as usize];
        let r2 = floats[(ctrl_hi & 3) as usize];
        let r3 = floats[((ctrl_hi >> 32) & 3) as usize];
        (
            (r0 as u64) | ((r1 as u64) << 32),
            (r2 as u64) | ((r3 as u64) << 32),
        )
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_permilps_imm(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VPERMILPS imm requires VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;

        let (src_lo, src_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let (dst_lo, dst_hi) = self.permilps_imm_128(src_lo, src_hi, imm8);
        self.regs.xmm[xmm_dst][0] = dst_lo;
        self.regs.xmm[xmm_dst][1] = dst_hi;

        if vex_l == 1 {
            let (src_hi2, src_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let (dst_hi2, dst_hi3) = self.permilps_imm_128(src_hi2, src_hi3, imm8);
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn permilps_imm_128(&self, src_lo: u64, src_hi: u64, imm8: u8) -> (u64, u64) {
        let floats = [
            src_lo as u32,
            (src_lo >> 32) as u32,
            src_hi as u32,
            (src_hi >> 32) as u32,
        ];
        let r0 = floats[(imm8 & 0x3) as usize];
        let r1 = floats[((imm8 >> 2) & 0x3) as usize];
        let r2 = floats[((imm8 >> 4) & 0x3) as usize];
        let r3 = floats[((imm8 >> 6) & 0x3) as usize];
        (
            (r0 as u64) | ((r1 as u64) << 32),
            (r2 as u64) | ((r3 as u64) << 32),
        )
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_permilpd_reg(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src = vvvv as usize;

        let (ctrl_lo, ctrl_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let src_lo = self.regs.xmm[xmm_src][0];
        let src_hi = self.regs.xmm[xmm_src][1];

        // Select based on bit 1 of each control qword
        let dst_lo = if (ctrl_lo >> 1) & 1 == 0 {
            src_lo
        } else {
            src_hi
        };
        let dst_hi = if (ctrl_hi >> 1) & 1 == 0 {
            src_lo
        } else {
            src_hi
        };
        self.regs.xmm[xmm_dst][0] = dst_lo;
        self.regs.xmm[xmm_dst][1] = dst_hi;

        if vex_l == 1 {
            let (ctrl_hi2, ctrl_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let src_hi2 = self.regs.ymm_high[xmm_src][0];
            let src_hi3 = self.regs.ymm_high[xmm_src][1];
            let dst_hi2 = if (ctrl_hi2 >> 1) & 1 == 0 {
                src_hi2
            } else {
                src_hi3
            };
            let dst_hi3 = if (ctrl_hi3 >> 1) & 1 == 0 {
                src_hi2
            } else {
                src_hi3
            };
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_permilpd_imm(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VPERMILPD imm requires VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;

        let (src_lo, src_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };
        let bit0 = (imm8 & 0x1) != 0;
        let bit1 = (imm8 & 0x2) != 0;
        let dst_lo = if bit0 { src_hi } else { src_lo };
        let dst_hi = if bit1 { src_hi } else { src_lo };
        self.regs.xmm[xmm_dst][0] = dst_lo;
        self.regs.xmm[xmm_dst][1] = dst_hi;

        if vex_l == 1 {
            let (src_hi2, src_hi3) = if is_memory {
                (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
            } else {
                (
                    self.regs.ymm_high[rm as usize][0],
                    self.regs.ymm_high[rm as usize][1],
                )
            };
            let bit2 = (imm8 & 0x4) != 0;
            let bit3 = (imm8 & 0x8) != 0;
            let dst_hi2 = if bit2 { src_hi3 } else { src_hi2 };
            let dst_hi3 = if bit3 { src_hi3 } else { src_hi2 };
            self.regs.ymm_high[xmm_dst][0] = dst_hi2;
            self.regs.ymm_high[xmm_dst][1] = dst_hi3;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_permd(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vex_l == 0 {
            return Err(Error::Emulator("VPERMD requires VEX.L=1".to_string()));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_idx = vvvv as usize;

        let src2_dwords = if is_memory {
            self.read_dwords_256(addr)?
        } else {
            self.read_dwords_ymm(rm as usize)
        };
        let idx_dwords = self.read_dwords_ymm(xmm_idx);

        let mut result = [0u32; 8];
        for i in 0..8 {
            let sel = (idx_dwords[i] & 0x7) as usize;
            result[i] = src2_dwords[sel];
        }
        self.write_dwords_ymm(xmm_dst, &result);

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_permps(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vex_l == 0 {
            return Err(Error::Emulator("VPERMPS requires VEX.L=1".to_string()));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_idx = vvvv as usize;

        let src2_dwords = if is_memory {
            self.read_dwords_256(addr)?
        } else {
            self.read_dwords_ymm(rm as usize)
        };
        let idx_dwords = self.read_dwords_ymm(xmm_idx);

        let mut result = [0u32; 8];
        for i in 0..8 {
            let sel = (idx_dwords[i] & 0x7) as usize;
            result[i] = src2_dwords[sel];
        }
        self.write_dwords_ymm(xmm_dst, &result);

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_permqd_imm(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VPERMQ/VPERMPD requires VEX.vvvv=1111b".to_string(),
            ));
        }
        if vex_l == 0 {
            return Err(Error::Emulator(
                "VPERMQ/VPERMPD requires VEX.L=1".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;

        let src_qwords = if is_memory {
            self.read_qwords_256(addr)?
        } else {
            self.read_qwords_ymm(rm as usize)
        };

        let mut result = [0u64; 4];
        for i in 0..4 {
            let sel = ((imm8 >> (i * 2)) & 0x3) as usize;
            result[i] = src_qwords[sel];
        }
        self.write_qwords_ymm(xmm_dst, &result);

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    fn read_dwords_ymm(&self, reg: usize) -> [u32; 8] {
        let lo0 = self.regs.xmm[reg][0];
        let lo1 = self.regs.xmm[reg][1];
        let hi0 = self.regs.ymm_high[reg][0];
        let hi1 = self.regs.ymm_high[reg][1];
        [
            lo0 as u32,
            (lo0 >> 32) as u32,
            lo1 as u32,
            (lo1 >> 32) as u32,
            hi0 as u32,
            (hi0 >> 32) as u32,
            hi1 as u32,
            (hi1 >> 32) as u32,
        ]
    }

    fn write_dwords_ymm(&mut self, reg: usize, values: &[u32; 8]) {
        self.regs.xmm[reg][0] = (values[0] as u64) | ((values[1] as u64) << 32);
        self.regs.xmm[reg][1] = (values[2] as u64) | ((values[3] as u64) << 32);
        self.regs.ymm_high[reg][0] = (values[4] as u64) | ((values[5] as u64) << 32);
        self.regs.ymm_high[reg][1] = (values[6] as u64) | ((values[7] as u64) << 32);
    }

    fn read_dwords_256(&mut self, addr: u64) -> Result<[u32; 8]> {
        let mut out = [0u32; 8];
        for i in 0..8 {
            out[i] = self.read_mem(addr + (i * 4) as u64, 4)? as u32;
        }
        Ok(out)
    }

    fn read_qwords_ymm(&self, reg: usize) -> [u64; 4] {
        [
            self.regs.xmm[reg][0],
            self.regs.xmm[reg][1],
            self.regs.ymm_high[reg][0],
            self.regs.ymm_high[reg][1],
        ]
    }

    fn write_qwords_ymm(&mut self, reg: usize, values: &[u64; 4]) {
        self.regs.xmm[reg][0] = values[0];
        self.regs.xmm[reg][1] = values[1];
        self.regs.ymm_high[reg][0] = values[2];
        self.regs.ymm_high[reg][1] = values[3];
    }

    fn read_qwords_256(&mut self, addr: u64) -> Result<[u64; 4]> {
        let mut out = [0u64; 4];
        for i in 0..4 {
            out[i] = self.read_mem(addr + (i * 8) as u64, 8)?;
        }
        Ok(out)
    }
}
