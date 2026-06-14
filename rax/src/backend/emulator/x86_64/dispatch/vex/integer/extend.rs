//! VEX integer instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};

impl X86_64Vcpu {
    pub(in crate::backend::emulator::x86_64) fn execute_vex_pmovsx(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        // Read source bytes needed
        let src = if is_memory {
            // Read appropriate bytes based on operation
            let bytes_needed = match opcode {
                0x20 => {
                    if vex_l == 0 {
                        8
                    } else {
                        16
                    }
                } // BW
                0x21 => {
                    if vex_l == 0 {
                        4
                    } else {
                        8
                    }
                } // BD
                0x22 => {
                    if vex_l == 0 {
                        2
                    } else {
                        4
                    }
                } // BQ
                0x23 => {
                    if vex_l == 0 {
                        8
                    } else {
                        16
                    }
                } // WD
                0x24 => {
                    if vex_l == 0 {
                        4
                    } else {
                        8
                    }
                } // WQ
                0x25 => {
                    if vex_l == 0 {
                        8
                    } else {
                        16
                    }
                } // DQ
                _ => 16,
            };
            let mut data = [0u8; 16];
            for i in 0..bytes_needed.min(16) {
                data[i] = self.read_mem(addr + i as u64, 1)? as u8;
            }
            data
        } else {
            let lo = self.regs.xmm[rm as usize][0];
            let hi = self.regs.xmm[rm as usize][1];
            let mut data = [0u8; 16];
            for i in 0..8 {
                data[i] = ((lo >> (i * 8)) & 0xFF) as u8;
            }
            for i in 0..8 {
                data[i + 8] = ((hi >> (i * 8)) & 0xFF) as u8;
            }
            data
        };

        match opcode {
            0x20 => {
                // PMOVSXBW: bytes to words
                let mut dst = [0u16; 16];
                for i in 0..8 {
                    dst[i] = src[i] as i8 as i16 as u16;
                }
                self.regs.xmm[xmm_dst][0] = (dst[0] as u64)
                    | ((dst[1] as u64) << 16)
                    | ((dst[2] as u64) << 32)
                    | ((dst[3] as u64) << 48);
                self.regs.xmm[xmm_dst][1] = (dst[4] as u64)
                    | ((dst[5] as u64) << 16)
                    | ((dst[6] as u64) << 32)
                    | ((dst[7] as u64) << 48);
                if vex_l == 1 {
                    for i in 0..8 {
                        dst[i + 8] = src[i + 8] as i8 as i16 as u16;
                    }
                    self.regs.ymm_high[xmm_dst][0] = (dst[8] as u64)
                        | ((dst[9] as u64) << 16)
                        | ((dst[10] as u64) << 32)
                        | ((dst[11] as u64) << 48);
                    self.regs.ymm_high[xmm_dst][1] = (dst[12] as u64)
                        | ((dst[13] as u64) << 16)
                        | ((dst[14] as u64) << 32)
                        | ((dst[15] as u64) << 48);
                }
            }
            0x21 => {
                // PMOVSXBD: bytes to dwords
                let mut dst = [0u32; 8];
                for i in 0..4 {
                    dst[i] = src[i] as i8 as i32 as u32;
                }
                self.regs.xmm[xmm_dst][0] = (dst[0] as u64) | ((dst[1] as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (dst[2] as u64) | ((dst[3] as u64) << 32);
                if vex_l == 1 {
                    for i in 0..4 {
                        dst[i + 4] = src[i + 4] as i8 as i32 as u32;
                    }
                    self.regs.ymm_high[xmm_dst][0] = (dst[4] as u64) | ((dst[5] as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = (dst[6] as u64) | ((dst[7] as u64) << 32);
                }
            }
            0x22 => {
                // PMOVSXBQ: bytes to qwords
                let d0 = src[0] as i8 as i64 as u64;
                let d1 = src[1] as i8 as i64 as u64;
                self.regs.xmm[xmm_dst][0] = d0;
                self.regs.xmm[xmm_dst][1] = d1;
                if vex_l == 1 {
                    let d2 = src[2] as i8 as i64 as u64;
                    let d3 = src[3] as i8 as i64 as u64;
                    self.regs.ymm_high[xmm_dst][0] = d2;
                    self.regs.ymm_high[xmm_dst][1] = d3;
                }
            }
            0x23 => {
                // PMOVSXWD: words to dwords
                let mut dst = [0u32; 8];
                for i in 0..4 {
                    let w = (src[i * 2] as u16) | ((src[i * 2 + 1] as u16) << 8);
                    dst[i] = w as i16 as i32 as u32;
                }
                self.regs.xmm[xmm_dst][0] = (dst[0] as u64) | ((dst[1] as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (dst[2] as u64) | ((dst[3] as u64) << 32);
                if vex_l == 1 {
                    for i in 0..4 {
                        let w = (src[8 + i * 2] as u16) | ((src[8 + i * 2 + 1] as u16) << 8);
                        dst[i + 4] = w as i16 as i32 as u32;
                    }
                    self.regs.ymm_high[xmm_dst][0] = (dst[4] as u64) | ((dst[5] as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = (dst[6] as u64) | ((dst[7] as u64) << 32);
                }
            }
            0x24 => {
                // PMOVSXWQ: words to qwords
                let w0 = (src[0] as u16) | ((src[1] as u16) << 8);
                let w1 = (src[2] as u16) | ((src[3] as u16) << 8);
                self.regs.xmm[xmm_dst][0] = w0 as i16 as i64 as u64;
                self.regs.xmm[xmm_dst][1] = w1 as i16 as i64 as u64;
                if vex_l == 1 {
                    let w2 = (src[4] as u16) | ((src[5] as u16) << 8);
                    let w3 = (src[6] as u16) | ((src[7] as u16) << 8);
                    self.regs.ymm_high[xmm_dst][0] = w2 as i16 as i64 as u64;
                    self.regs.ymm_high[xmm_dst][1] = w3 as i16 as i64 as u64;
                }
            }
            0x25 => {
                // PMOVSXDQ: dwords to qwords
                let d0 = (src[0] as u32)
                    | ((src[1] as u32) << 8)
                    | ((src[2] as u32) << 16)
                    | ((src[3] as u32) << 24);
                let d1 = (src[4] as u32)
                    | ((src[5] as u32) << 8)
                    | ((src[6] as u32) << 16)
                    | ((src[7] as u32) << 24);
                self.regs.xmm[xmm_dst][0] = d0 as i32 as i64 as u64;
                self.regs.xmm[xmm_dst][1] = d1 as i32 as i64 as u64;
                if vex_l == 1 {
                    let d2 = (src[8] as u32)
                        | ((src[9] as u32) << 8)
                        | ((src[10] as u32) << 16)
                        | ((src[11] as u32) << 24);
                    let d3 = (src[12] as u32)
                        | ((src[13] as u32) << 8)
                        | ((src[14] as u32) << 16)
                        | ((src[15] as u32) << 24);
                    self.regs.ymm_high[xmm_dst][0] = d2 as i32 as i64 as u64;
                    self.regs.ymm_high[xmm_dst][1] = d3 as i32 as i64 as u64;
                }
            }
            _ => unreachable!(),
        }

        if vex_l == 0 {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }

    pub(in crate::backend::emulator::x86_64) fn execute_vex_pmovzx(
        &mut self,
        ctx: &mut InsnContext,
        vex_l: u8,
        opcode: u8,
    ) -> Result<Option<VcpuExit>> {
        let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
        let xmm_dst = reg as usize;

        let src = if is_memory {
            let bytes_needed = match opcode {
                0x30 => {
                    if vex_l == 0 {
                        8
                    } else {
                        16
                    }
                }
                0x31 => {
                    if vex_l == 0 {
                        4
                    } else {
                        8
                    }
                }
                0x32 => {
                    if vex_l == 0 {
                        2
                    } else {
                        4
                    }
                }
                0x33 => {
                    if vex_l == 0 {
                        8
                    } else {
                        16
                    }
                }
                0x34 => {
                    if vex_l == 0 {
                        4
                    } else {
                        8
                    }
                }
                0x35 => {
                    if vex_l == 0 {
                        8
                    } else {
                        16
                    }
                }
                _ => 16,
            };
            let mut data = [0u8; 16];
            for i in 0..bytes_needed.min(16) {
                data[i] = self.read_mem(addr + i as u64, 1)? as u8;
            }
            data
        } else {
            let lo = self.regs.xmm[rm as usize][0];
            let hi = self.regs.xmm[rm as usize][1];
            let mut data = [0u8; 16];
            for i in 0..8 {
                data[i] = ((lo >> (i * 8)) & 0xFF) as u8;
            }
            for i in 0..8 {
                data[i + 8] = ((hi >> (i * 8)) & 0xFF) as u8;
            }
            data
        };

        match opcode {
            0x30 => {
                // PMOVZXBW
                let mut dst = [0u16; 16];
                for i in 0..8 {
                    dst[i] = src[i] as u16;
                }
                self.regs.xmm[xmm_dst][0] = (dst[0] as u64)
                    | ((dst[1] as u64) << 16)
                    | ((dst[2] as u64) << 32)
                    | ((dst[3] as u64) << 48);
                self.regs.xmm[xmm_dst][1] = (dst[4] as u64)
                    | ((dst[5] as u64) << 16)
                    | ((dst[6] as u64) << 32)
                    | ((dst[7] as u64) << 48);
                if vex_l == 1 {
                    for i in 0..8 {
                        dst[i + 8] = src[i + 8] as u16;
                    }
                    self.regs.ymm_high[xmm_dst][0] = (dst[8] as u64)
                        | ((dst[9] as u64) << 16)
                        | ((dst[10] as u64) << 32)
                        | ((dst[11] as u64) << 48);
                    self.regs.ymm_high[xmm_dst][1] = (dst[12] as u64)
                        | ((dst[13] as u64) << 16)
                        | ((dst[14] as u64) << 32)
                        | ((dst[15] as u64) << 48);
                }
            }
            0x31 => {
                // PMOVZXBD
                let mut dst = [0u32; 8];
                for i in 0..4 {
                    dst[i] = src[i] as u32;
                }
                self.regs.xmm[xmm_dst][0] = (dst[0] as u64) | ((dst[1] as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (dst[2] as u64) | ((dst[3] as u64) << 32);
                if vex_l == 1 {
                    for i in 0..4 {
                        dst[i + 4] = src[i + 4] as u32;
                    }
                    self.regs.ymm_high[xmm_dst][0] = (dst[4] as u64) | ((dst[5] as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = (dst[6] as u64) | ((dst[7] as u64) << 32);
                }
            }
            0x32 => {
                // PMOVZXBQ
                self.regs.xmm[xmm_dst][0] = src[0] as u64;
                self.regs.xmm[xmm_dst][1] = src[1] as u64;
                if vex_l == 1 {
                    self.regs.ymm_high[xmm_dst][0] = src[2] as u64;
                    self.regs.ymm_high[xmm_dst][1] = src[3] as u64;
                }
            }
            0x33 => {
                // PMOVZXWD
                let mut dst = [0u32; 8];
                for i in 0..4 {
                    let w = (src[i * 2] as u16) | ((src[i * 2 + 1] as u16) << 8);
                    dst[i] = w as u32;
                }
                self.regs.xmm[xmm_dst][0] = (dst[0] as u64) | ((dst[1] as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (dst[2] as u64) | ((dst[3] as u64) << 32);
                if vex_l == 1 {
                    for i in 0..4 {
                        let w = (src[8 + i * 2] as u16) | ((src[8 + i * 2 + 1] as u16) << 8);
                        dst[i + 4] = w as u32;
                    }
                    self.regs.ymm_high[xmm_dst][0] = (dst[4] as u64) | ((dst[5] as u64) << 32);
                    self.regs.ymm_high[xmm_dst][1] = (dst[6] as u64) | ((dst[7] as u64) << 32);
                }
            }
            0x34 => {
                // PMOVZXWQ
                let w0 = (src[0] as u16) | ((src[1] as u16) << 8);
                let w1 = (src[2] as u16) | ((src[3] as u16) << 8);
                self.regs.xmm[xmm_dst][0] = w0 as u64;
                self.regs.xmm[xmm_dst][1] = w1 as u64;
                if vex_l == 1 {
                    let w2 = (src[4] as u16) | ((src[5] as u16) << 8);
                    let w3 = (src[6] as u16) | ((src[7] as u16) << 8);
                    self.regs.ymm_high[xmm_dst][0] = w2 as u64;
                    self.regs.ymm_high[xmm_dst][1] = w3 as u64;
                }
            }
            0x35 => {
                // PMOVZXDQ
                let d0 = (src[0] as u32)
                    | ((src[1] as u32) << 8)
                    | ((src[2] as u32) << 16)
                    | ((src[3] as u32) << 24);
                let d1 = (src[4] as u32)
                    | ((src[5] as u32) << 8)
                    | ((src[6] as u32) << 16)
                    | ((src[7] as u32) << 24);
                self.regs.xmm[xmm_dst][0] = d0 as u64;
                self.regs.xmm[xmm_dst][1] = d1 as u64;
                if vex_l == 1 {
                    let d2 = (src[8] as u32)
                        | ((src[9] as u32) << 8)
                        | ((src[10] as u32) << 16)
                        | ((src[11] as u32) << 24);
                    let d3 = (src[12] as u32)
                        | ((src[13] as u32) << 8)
                        | ((src[14] as u32) << 16)
                        | ((src[15] as u32) << 24);
                    self.regs.ymm_high[xmm_dst][0] = d2 as u64;
                    self.regs.ymm_high[xmm_dst][1] = d3 as u64;
                }
            }
            _ => unreachable!(),
        }

        if vex_l == 0 {
            self.regs.ymm_high[xmm_dst][0] = 0;
            self.regs.ymm_high[xmm_dst][1] = 0;
        }

        self.regs.rip += ctx.cursor as u64;
        Ok(None)
    }
}
