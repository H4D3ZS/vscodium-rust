//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_shufp(
        &mut self,
        ctx: &mut InsnContext,
        vex_pp: u8,
        vex_l: u8,
        vvvv: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;
        let xmm_src1 = vvvv as usize;

        if vex_pp == 0 {
            // VSHUFPS: shuffle packed singles
            let (src2_lo, src2_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let src1_lo = self.regs.xmm[xmm_src1][0];
            let src1_hi = self.regs.xmm[xmm_src1][1];

            // Extract 4 singles from src1 and src2
            let src1 = [
                src1_lo as u32,
                (src1_lo >> 32) as u32,
                src1_hi as u32,
                (src1_hi >> 32) as u32,
            ];
            let src2 = [
                src2_lo as u32,
                (src2_lo >> 32) as u32,
                src2_hi as u32,
                (src2_hi >> 32) as u32,
            ];

            // dst[0] = src1[imm[1:0]], dst[1] = src1[imm[3:2]]
            // dst[2] = src2[imm[5:4]], dst[3] = src2[imm[7:6]]
            let d0 = src1[(imm8 & 0x03) as usize] as u64;
            let d1 = src1[((imm8 >> 2) & 0x03) as usize] as u64;
            let d2 = src2[((imm8 >> 4) & 0x03) as usize] as u64;
            let d3 = src2[((imm8 >> 6) & 0x03) as usize] as u64;

            self.regs.xmm[xmm_dst][0] = d0 | (d1 << 32);
            self.regs.xmm[xmm_dst][1] = d2 | (d3 << 32);

            if vex_l == 1 {
                // YMM: repeat for upper 128 bits
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

                let src1_u = [
                    src1_hi2 as u32,
                    (src1_hi2 >> 32) as u32,
                    src1_hi3 as u32,
                    (src1_hi3 >> 32) as u32,
                ];
                let src2_u = [
                    src2_hi2 as u32,
                    (src2_hi2 >> 32) as u32,
                    src2_hi3 as u32,
                    (src2_hi3 >> 32) as u32,
                ];

                let d4 = src1_u[(imm8 & 0x03) as usize] as u64;
                let d5 = src1_u[((imm8 >> 2) & 0x03) as usize] as u64;
                let d6 = src2_u[((imm8 >> 4) & 0x03) as usize] as u64;
                let d7 = src2_u[((imm8 >> 6) & 0x03) as usize] as u64;

                self.regs.ymm_high[xmm_dst][0] = d4 | (d5 << 32);
                self.regs.ymm_high[xmm_dst][1] = d6 | (d7 << 32);
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        } else {
            // VSHUFPD: shuffle packed doubles
            let (src2_lo, src2_hi) = if is_memory {
                (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
            } else {
                (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
            };
            let src1_lo = self.regs.xmm[xmm_src1][0];
            let src1_hi = self.regs.xmm[xmm_src1][1];

            // dst[0] = src1[imm[0]], dst[1] = src2[imm[1]]
            let d0 = if (imm8 & 0x01) == 0 { src1_lo } else { src1_hi };
            let d1 = if (imm8 & 0x02) == 0 { src2_lo } else { src2_hi };

            self.regs.xmm[xmm_dst][0] = d0;
            self.regs.xmm[xmm_dst][1] = d1;

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

                let d2 = if (imm8 & 0x04) == 0 {
                    src1_hi2
                } else {
                    src1_hi3
                };
                let d3 = if (imm8 & 0x08) == 0 {
                    src2_hi2
                } else {
                    src2_hi3
                };

                self.regs.ymm_high[xmm_dst][0] = d2;
                self.regs.ymm_high[xmm_dst][1] = d3;
            } else {
                self.regs.ymm_high[xmm_dst][0] = 0;
                self.regs.ymm_high[xmm_dst][1] = 0;
            }
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_shuffle(
        &mut self,
        ctx: &mut InsnContext,
        vex_pp: u8,
        vex_l: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let imm8 = ctx.consume_u8()?;
        let xmm_dst = reg as usize;

        match vex_pp {
            1 => {
                // VPSHUFD: 66 prefix - shuffle dwords
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dwords: [u32; 4] = [
                    src_lo as u32,
                    (src_lo >> 32) as u32,
                    src_hi as u32,
                    (src_hi >> 32) as u32,
                ];
                let d0 = dwords[((imm8 >> 0) & 3) as usize];
                let d1 = dwords[((imm8 >> 2) & 3) as usize];
                let d2 = dwords[((imm8 >> 4) & 3) as usize];
                let d3 = dwords[((imm8 >> 6) & 3) as usize];
                self.regs.xmm[xmm_dst][0] = (d0 as u64) | ((d1 as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (d2 as u64) | ((d3 as u64) << 32);

                if vex_l == 1 {
                    // YMM: process high 128-bit lane independently
                    let (src2_lo, src2_hi) = if is_memory {
                        (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
                    } else {
                        (
                            self.regs.ymm_high[rm as usize][0],
                            self.regs.ymm_high[rm as usize][1],
                        )
                    };
                    let dwords2: [u32; 4] = [
                        src2_lo as u32,
                        (src2_lo >> 32) as u32,
                        src2_hi as u32,
                        (src2_hi >> 32) as u32,
                    ];
                    let d4 = dwords2[((imm8 >> 0) & 3) as usize];
                    let d5 = dwords2[((imm8 >> 2) & 3) as usize];
                    let d6 = dwords2[((imm8 >> 4) & 3) as usize];
                    let d7 = dwords2[((imm8 >> 6) & 3) as usize];
                    self.regs.ymm_high[xmm_dst][0] = (d4 as u64) | ((d5 as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = (d6 as u64) | ((d7 as u64) << 32);
                } else {
                    self.regs.ymm_high[xmm_dst][0] = 0;
                    self.regs.ymm_high[xmm_dst][1] = 0;
                }
            }
            2 => {
                // VPSHUFHW: F3 prefix - shuffle high words
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                self.regs.xmm[xmm_dst][0] = src_lo;
                let w0 = (src_hi >> (((imm8 >> 0) & 3) * 16)) as u16;
                let w1 = (src_hi >> (((imm8 >> 2) & 3) * 16)) as u16;
                let w2 = (src_hi >> (((imm8 >> 4) & 3) * 16)) as u16;
                let w3 = (src_hi >> (((imm8 >> 6) & 3) * 16)) as u16;
                self.regs.xmm[xmm_dst][1] =
                    (w0 as u64) | ((w1 as u64) << 16) | ((w2 as u64) << 32) | ((w3 as u64) << 48);

                if vex_l == 1 {
                    let (src2_lo, src2_hi) = if is_memory {
                        (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
                    } else {
                        (
                            self.regs.ymm_high[rm as usize][0],
                            self.regs.ymm_high[rm as usize][1],
                        )
                    };
                    self.regs.ymm_high[xmm_dst][0] = src2_lo;
                    let w4 = (src2_hi >> (((imm8 >> 0) & 3) * 16)) as u16;
                    let w5 = (src2_hi >> (((imm8 >> 2) & 3) * 16)) as u16;
                    let w6 = (src2_hi >> (((imm8 >> 4) & 3) * 16)) as u16;
                    let w7 = (src2_hi >> (((imm8 >> 6) & 3) * 16)) as u16;
                    self.regs.ymm_high[xmm_dst][1] = (w4 as u64)
                        | ((w5 as u64) << 16)
                        | ((w6 as u64) << 32)
                        | ((w7 as u64) << 48);
                } else {
                    self.regs.ymm_high[xmm_dst][0] = 0;
                    self.regs.ymm_high[xmm_dst][1] = 0;
                }
            }
            3 => {
                // VPSHUFLW: F2 prefix - shuffle low words
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let w0 = (src_lo >> (((imm8 >> 0) & 3) * 16)) as u16;
                let w1 = (src_lo >> (((imm8 >> 2) & 3) * 16)) as u16;
                let w2 = (src_lo >> (((imm8 >> 4) & 3) * 16)) as u16;
                let w3 = (src_lo >> (((imm8 >> 6) & 3) * 16)) as u16;
                self.regs.xmm[xmm_dst][0] =
                    (w0 as u64) | ((w1 as u64) << 16) | ((w2 as u64) << 32) | ((w3 as u64) << 48);
                self.regs.xmm[xmm_dst][1] = src_hi;

                if vex_l == 1 {
                    let (src2_lo, src2_hi) = if is_memory {
                        (self.read_mem(addr + 16, 8)?, self.read_mem(addr + 24, 8)?)
                    } else {
                        (
                            self.regs.ymm_high[rm as usize][0],
                            self.regs.ymm_high[rm as usize][1],
                        )
                    };
                    let w4 = (src2_lo >> (((imm8 >> 0) & 3) * 16)) as u16;
                    let w5 = (src2_lo >> (((imm8 >> 2) & 3) * 16)) as u16;
                    let w6 = (src2_lo >> (((imm8 >> 4) & 3) * 16)) as u16;
                    let w7 = (src2_lo >> (((imm8 >> 6) & 3) * 16)) as u16;
                    self.regs.ymm_high[xmm_dst][0] = (w4 as u64)
                        | ((w5 as u64) << 16)
                        | ((w6 as u64) << 32)
                        | ((w7 as u64) << 48);
                    self.regs.ymm_high[xmm_dst][1] = src2_hi;
                } else {
                    self.regs.ymm_high[xmm_dst][0] = 0;
                    self.regs.ymm_high[xmm_dst][1] = 0;
                }
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "unimplemented VEX shuffle pp={}",
                    vex_pp
                )));
            }
        }
        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}
