//! VEX floating-point move instruction implementations.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_movss_sd(
        &mut self,
        ctx: &mut InsnContext,
        vex_pp: u8,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        if vex_l != 0 {
            return Err(Error::Emulator("VMOVSS/VMOVSD require VEX.L=0".to_string()));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        let is_ss = vex_pp == 2;
        let is_sd = vex_pp == 3;
        if !is_ss && !is_sd {
            return Err(Error::Emulator(
                "VMOVSS/VMOVSD require F3/F2 prefix".to_string(),
            ));
        }

        match opcode {
            0x10 => {
                // Load or reg/reg merge
                if is_memory {
                    if vvvv != 0 {
                        return Err(Error::Emulator(
                            "VMOVSS/VMOVSD load requires VEX.vvvv=1111b".to_string(),
                        ));
                    }
                    if is_ss {
                        let val = self.read_mem(addr, 4)? as u32;
                        self.regs.xmm[xmm_dst][0] = val as u64;
                        self.regs.xmm[xmm_dst][1] = 0;
                    } else {
                        let val = self.read_mem(addr, 8)?;
                        self.regs.xmm[xmm_dst][0] = val;
                        self.regs.xmm[xmm_dst][1] = 0;
                    }
                } else {
                    let xmm_src1 = vvvv as usize;
                    let xmm_src2 = rm as usize;
                    if is_ss {
                        let src1_lo = self.regs.xmm[xmm_src1][0];
                        let src1_hi = self.regs.xmm[xmm_src1][1];
                        let src2_lo = self.regs.xmm[xmm_src2][0];
                        self.regs.xmm[xmm_dst][0] =
                            (src1_lo & !0xFFFF_FFFF) | (src2_lo & 0xFFFF_FFFF);
                        self.regs.xmm[xmm_dst][1] = src1_hi;
                    } else {
                        self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src2][0];
                        self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
                    }
                }
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
            0x11 => {
                if is_memory {
                    if vvvv != 0 {
                        return Err(Error::Emulator(
                            "VMOVSS/VMOVSD store requires VEX.vvvv=1111b".to_string(),
                        ));
                    }
                    let xmm_src = reg as usize;
                    if is_ss {
                        let val = self.regs.xmm[xmm_src][0] as u32;
                        self.write_mem(addr, val as u64, 4)?;
                    } else {
                        let val = self.regs.xmm[xmm_src][0];
                        self.write_mem(addr, val, 8)?;
                    }
                } else {
                    // Reg/reg encoding has the same merge semantics as opcode 0x10.
                    let xmm_src1 = vvvv as usize;
                    let xmm_src2 = rm as usize;
                    if is_ss {
                        let src1_lo = self.regs.xmm[xmm_src1][0];
                        let src1_hi = self.regs.xmm[xmm_src1][1];
                        let src2_lo = self.regs.xmm[xmm_src2][0];
                        self.regs.xmm[xmm_dst][0] =
                            (src1_lo & !0xFFFF_FFFF) | (src2_lo & 0xFFFF_FFFF);
                        self.regs.xmm[xmm_dst][1] = src1_hi;
                    } else {
                        self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src2][0];
                        self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
                    }
                    self.regs.ymm_high[xmm_dst][0] = 0;
                    self.regs.ymm_high[xmm_dst][1] = 0;
                }
            }
            _ => unreachable!(),
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_movlps_hps(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        if vex_l != 0 {
            return Err(Error::Emulator(
                "VMOVLPS/VMOVHPS require VEX.L=0".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        if is_memory {
            let mem_val = self.read_mem(addr, 8)?;
            match opcode {
                0x12 => {
                    // VMOVLPS: low qword from mem, high qword from src1.
                    self.regs.xmm[xmm_dst][0] = mem_val;
                    self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
                }
                0x16 => {
                    // VMOVHPS: high qword from mem, low qword from src1.
                    self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src1][0];
                    self.regs.xmm[xmm_dst][1] = mem_val;
                }
                _ => unreachable!(),
            }
        } else {
            let xmm_src2 = rm as usize;
            match opcode {
                0x12 => {
                    // VMOVHLPS: low qword from src2 high, high qword from src1 high.
                    self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src2][1];
                    self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
                }
                0x16 => {
                    // VMOVLHPS: low qword from src1 low, high qword from src2 low.
                    self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src1][0];
                    self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src2][0];
                }
                _ => unreachable!(),
            }
        }

        self.regs.ymm_high[xmm_dst][0] = 0;
        self.regs.ymm_high[xmm_dst][1] = 0;
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_movlpd_hpd(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        if vex_l != 0 {
            return Err(Error::Emulator(
                "VMOVLPD/VMOVHPD require VEX.L=0".to_string(),
            ));
        }
        let (reg, _rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        if !is_memory {
            return Err(Error::Emulator(
                "VMOVLPD/VMOVHPD require memory operand".to_string(),
            ));
        }
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;
        let mem_val = self.read_mem(addr, 8)?;

        match opcode {
            0x12 => {
                // VMOVLPD: low qword from mem, high qword from src1.
                self.regs.xmm[xmm_dst][0] = mem_val;
                self.regs.xmm[xmm_dst][1] = self.regs.xmm[xmm_src1][1];
            }
            0x16 => {
                // VMOVHPD: high qword from mem, low qword from src1.
                self.regs.xmm[xmm_dst][0] = self.regs.xmm[xmm_src1][0];
                self.regs.xmm[xmm_dst][1] = mem_val;
            }
            _ => unreachable!(),
        }

        self.regs.ymm_high[xmm_dst][0] = 0;
        self.regs.ymm_high[xmm_dst][1] = 0;
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_movsldup(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VMOVSLDUP requires VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let count = if vex_l == 1 { 8 } else { 4 };
        let mut src = [0u32; 8];

        if is_memory {
            for i in 0..count {
                src[i] = self.read_mem(addr + (i * 4) as u64, 4)? as u32;
            }
        } else {
            let xmm_src = rm as usize;
            let lo = self.regs.xmm[xmm_src][0];
            let hi = self.regs.xmm[xmm_src][1];
            src[0] = lo as u32;
            src[1] = (lo >> 32) as u32;
            src[2] = hi as u32;
            src[3] = (hi >> 32) as u32;
            if vex_l == 1 {
                let hi2 = self.regs.ymm_high[xmm_src][0];
                let hi3 = self.regs.ymm_high[xmm_src][1];
                src[4] = hi2 as u32;
                src[5] = (hi2 >> 32) as u32;
                src[6] = hi3 as u32;
                src[7] = (hi3 >> 32) as u32;
            }
        }

        let mut dst = [0u32; 8];
        for i in 0..(count / 2) {
            let even = 2 * i;
            let odd = even + 1;
            dst[even] = src[even];
            dst[odd] = src[even];
        }

        self.regs.xmm[xmm_dst][0] = (dst[0] as u64) | ((dst[1] as u64) << 32);
        self.regs.xmm[xmm_dst][1] = (dst[2] as u64) | ((dst[3] as u64) << 32);
        if vex_l == 1 {
            self.regs.ymm_high[xmm_dst][0] = (dst[4] as u64) | ((dst[5] as u64) << 32);
            self.regs.ymm_high[xmm_dst][1] = (dst[6] as u64) | ((dst[7] as u64) << 32);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_movshdup(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VMOVSHDUP requires VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let count = if vex_l == 1 { 8 } else { 4 };
        let mut src = [0u32; 8];

        if is_memory {
            for i in 0..count {
                src[i] = self.read_mem(addr + (i * 4) as u64, 4)? as u32;
            }
        } else {
            let xmm_src = rm as usize;
            let lo = self.regs.xmm[xmm_src][0];
            let hi = self.regs.xmm[xmm_src][1];
            src[0] = lo as u32;
            src[1] = (lo >> 32) as u32;
            src[2] = hi as u32;
            src[3] = (hi >> 32) as u32;
            if vex_l == 1 {
                let hi2 = self.regs.ymm_high[xmm_src][0];
                let hi3 = self.regs.ymm_high[xmm_src][1];
                src[4] = hi2 as u32;
                src[5] = (hi2 >> 32) as u32;
                src[6] = hi3 as u32;
                src[7] = (hi3 >> 32) as u32;
            }
        }

        let mut dst = [0u32; 8];
        for i in 0..(count / 2) {
            let even = 2 * i;
            let odd = even + 1;
            dst[even] = src[odd];
            dst[odd] = src[odd];
        }

        self.regs.xmm[xmm_dst][0] = (dst[0] as u64) | ((dst[1] as u64) << 32);
        self.regs.xmm[xmm_dst][1] = (dst[2] as u64) | ((dst[3] as u64) << 32);
        if vex_l == 1 {
            self.regs.ymm_high[xmm_dst][0] = (dst[4] as u64) | ((dst[5] as u64) << 32);
            self.regs.ymm_high[xmm_dst][1] = (dst[6] as u64) | ((dst[7] as u64) << 32);
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_movddup(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VMOVDDUP requires VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;
        let count = if vex_l == 1 { 4 } else { 2 };
        let mut src = [0u64; 4];

        if is_memory {
            for i in 0..count {
                src[i] = self.read_mem(addr + (i * 8) as u64, 8)?;
            }
        } else {
            let xmm_src = rm as usize;
            src[0] = self.regs.xmm[xmm_src][0];
            src[1] = self.regs.xmm[xmm_src][1];
            if vex_l == 1 {
                src[2] = self.regs.ymm_high[xmm_src][0];
                src[3] = self.regs.ymm_high[xmm_src][1];
            }
        }

        let mut dst = [0u64; 4];
        for i in 0..(count / 2) {
            let even = 2 * i;
            dst[even] = src[even];
            dst[even + 1] = src[even];
        }

        self.regs.xmm[xmm_dst][0] = dst[0];
        self.regs.xmm[xmm_dst][1] = dst[1];
        if vex_l == 1 {
            self.regs.ymm_high[xmm_dst][0] = dst[2];
            self.regs.ymm_high[xmm_dst][1] = dst[3];
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_movnt_store(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VMOVNT* store requires VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, _rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        if !is_memory {
            return Err(Error::Emulator(
                "VMOVNT* requires memory destination".to_string(),
            ));
        }
        let xmm_src = reg as usize;
        let qwords = if vex_l == 1 { 4 } else { 2 };
        for i in 0..qwords {
            let val = if i < 2 {
                self.regs.xmm[xmm_src][i]
            } else {
                self.regs.ymm_high[xmm_src][i - 2]
            };
            self.write_mem(addr + (i * 8) as u64, val, 8)?;
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_movntdqa(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VMOVNTDQA requires VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, _rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        if !is_memory {
            return Err(Error::Emulator(
                "VMOVNTDQA requires memory operand".to_string(),
            ));
        }
        let xmm_dst = reg as usize;
        self.regs.xmm[xmm_dst][0] = self.read_mem(addr, 8)?;
        self.regs.xmm[xmm_dst][1] = self.read_mem(addr + 8, 8)?;
        if vex_l == 1 {
            self.regs.ymm_high[xmm_dst][0] = self.read_mem(addr + 16, 8)?;
            self.regs.ymm_high[xmm_dst][1] = self.read_mem(addr + 24, 8)?;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}
