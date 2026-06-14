//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_broadcast(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        let elem_size = match opcode {
            0x78 => 1, // VPBROADCASTB
            0x79 => 2, // VPBROADCASTW
            0x58 => 4, // VPBROADCASTD
            0x59 => 8, // VPBROADCASTQ
            _ => unreachable!(),
        };

        let val = if is_memory {
            self.read_mem(addr, elem_size)?
        } else {
            let src = self.regs.xmm[rm as usize][0];
            if elem_size == 8 {
                src
            } else {
                src & ((1u64 << (elem_size * 8)) - 1)
            }
        };

        let broadcast = match elem_size {
            1 => {
                let b = val as u8;
                let q = (b as u64)
                    | ((b as u64) << 8)
                    | ((b as u64) << 16)
                    | ((b as u64) << 24)
                    | ((b as u64) << 32)
                    | ((b as u64) << 40)
                    | ((b as u64) << 48)
                    | ((b as u64) << 56);
                (q, q)
            }
            2 => {
                let w = val as u16;
                let q = (w as u64) | ((w as u64) << 16) | ((w as u64) << 32) | ((w as u64) << 48);
                (q, q)
            }
            4 => {
                let d = val as u32;
                let q = (d as u64) | ((d as u64) << 32);
                (q, q)
            }
            8 => (val, val),
            _ => unreachable!(),
        };

        self.regs.xmm[xmm_dst][0] = broadcast.0;
        self.regs.xmm[xmm_dst][1] = broadcast.1;

        if vex_l == 1 {
            self.regs.ymm_high[xmm_dst][0] = broadcast.0;
            self.regs.ymm_high[xmm_dst][1] = broadcast.1;
        } else {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_broadcast_i128(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
    ) -> Result<Option<VcpuExit>> {
        if vex_l == 0 {
            return Err(Error::Emulator(
                "VBROADCASTI128 requires VEX.L=1".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        let (src_lo, src_hi) = if is_memory {
            (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
        } else {
            (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
        };

        self.regs.xmm[xmm_dst][0] = src_lo;
        self.regs.xmm[xmm_dst][1] = src_hi;
        self.regs.ymm_high[xmm_dst][0] = src_lo;
        self.regs.ymm_high[xmm_dst][1] = src_hi;

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_broadcast_fp(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        vvvv: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        if vvvv != 0 {
            return Err(Error::Emulator(
                "VBROADCASTSS/SD require VEX.vvvv=1111b".to_string(),
            ));
        }
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        if opcode == 0x18 {
            // VBROADCASTSS
            let val = if is_memory {
                self.read_mem(addr, 4)? as u32
            } else {
                self.regs.xmm[rm as usize][0] as u32
            };
            let pair = (val as u64) | ((val as u64) << 32);
            self.regs.xmm[xmm_dst][0] = pair;
            self.regs.xmm[xmm_dst][1] = pair;
            if vex_l == 1 {
                self.regs.ymm_high[xmm_dst][0] = pair;
                self.regs.ymm_high[xmm_dst][1] = pair;
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        } else {
            // VBROADCASTSD
            let val = if is_memory {
                self.read_mem(addr, 8)?
            } else {
                self.regs.xmm[rm as usize][0]
            };
            self.regs.xmm[xmm_dst][0] = val;
            self.regs.xmm[xmm_dst][1] = val;
            if vex_l == 1 {
                self.regs.ymm_high[xmm_dst][0] = val;
                self.regs.ymm_high[xmm_dst][1] = val;
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}
