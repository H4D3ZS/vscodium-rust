//! Two-byte opcode instruction implementation for x86_64 emulator.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::super::aes;
use super::super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::super::flags;
use super::super::super::super::insn;
use super::super::super::super::sha;

impl X86_64Vcpu {
    #[inline(always)]
    pub(in crate::backend::emulator::x86_64) fn execute_0f38(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let opcode3 = ctx.consume_u8()?;

        // Record precise opcode key for profiling
        #[cfg(feature = "profiling")]
        crate::profiling::set_current_opcode_key(crate::profiling::OpcodeKey::ThreeByte38(opcode3));

        match opcode3 {
            // ===== SSSE3 Instructions (0x00-0x0B, 0x1C-0x1E) =====
            0x00 => insn::simd::pshufb(self, ctx),
            0x01 => insn::simd::phaddw(self, ctx),
            0x02 => insn::simd::phaddd(self, ctx),
            0x03 => insn::simd::phaddsw(self, ctx),
            0x04 => insn::simd::pmaddubsw(self, ctx),
            0x05 => insn::simd::phsubw(self, ctx),
            0x06 => insn::simd::phsubd(self, ctx),
            0x07 => insn::simd::phsubsw(self, ctx),
            0x08 => insn::simd::psignb(self, ctx),
            0x09 => insn::simd::psignw(self, ctx),
            0x0A => insn::simd::psignd(self, ctx),
            0x0B => insn::simd::pmulhrsw(self, ctx),
            0x1C => insn::simd::pabsb(self, ctx),
            0x1D => insn::simd::pabsw(self, ctx),
            0x1E => insn::simd::pabsd(self, ctx),

            // ===== SSE4.1 Instructions =====
            0x10 => insn::simd::pblendvb(self, ctx),
            0x14 => insn::simd::blendvps(self, ctx),
            0x15 => insn::simd::blendvpd(self, ctx),
            0x17 => insn::simd::ptest(self, ctx),
            0x20 => insn::simd::pmovsxbw(self, ctx),
            0x21 => insn::simd::pmovsxbd(self, ctx),
            0x22 => insn::simd::pmovsxbq(self, ctx),
            0x23 => insn::simd::pmovsxwd(self, ctx),
            0x24 => insn::simd::pmovsxwq(self, ctx),
            0x25 => insn::simd::pmovsxdq(self, ctx),
            0x28 => insn::simd::pmuldq(self, ctx),
            0x29 => insn::simd::pcmpeqq(self, ctx),
            0x2A => insn::simd::movntdqa(self, ctx),
            0x2B => insn::simd::packusdw(self, ctx),
            0x30 => insn::simd::pmovzxbw(self, ctx),
            0x31 => insn::simd::pmovzxbd(self, ctx),
            0x32 => insn::simd::pmovzxbq(self, ctx),
            0x33 => insn::simd::pmovzxwd(self, ctx),
            0x34 => insn::simd::pmovzxwq(self, ctx),
            0x35 => insn::simd::pmovzxdq(self, ctx),
            0x37 => insn::simd::pcmpgtq(self, ctx),
            0x38 => insn::simd::pminsb(self, ctx),
            0x39 => insn::simd::pminsd(self, ctx),
            0x3A => insn::simd::pminuw(self, ctx),
            0x3B => insn::simd::pminud(self, ctx),
            0x3C => insn::simd::pmaxsb(self, ctx),
            0x3D => insn::simd::pmaxsd(self, ctx),
            0x3E => insn::simd::pmaxuw(self, ctx),
            0x3F => insn::simd::pmaxud(self, ctx),
            0x40 => insn::simd::pmulld(self, ctx),
            0x41 => insn::simd::phminposuw(self, ctx),

            // GFNI: GF2P8MULB xmm1, xmm2/m128 (66 0F 38 CF)
            0xCF => insn::simd::gf2p8mulb(self, ctx),

            // ===== AES-NI Instructions (0xDB-0xDF) =====

            // AESIMC - AES Inverse Mix Columns (0xDB)
            // DEST := InvMixColumns(SRC)
            0xDB => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("AESIMC requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let (result_lo, result_hi) = aes::aesimc(src_lo, src_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // AESENC - AES Encrypt Round (0xDC)
            // STATE := ShiftRows(SubBytes(STATE)); STATE := MixColumns(STATE); DEST := STATE XOR RoundKey
            0xDC => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("AESENC requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (key_lo, key_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let state_lo = self.regs.xmm[xmm_dst][0];
                let state_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = aes::aesenc(state_lo, state_hi, key_lo, key_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // AESENCLAST - AES Encrypt Last Round (0xDD)
            // STATE := ShiftRows(SubBytes(STATE)); DEST := STATE XOR RoundKey (no MixColumns)
            0xDD => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("AESENCLAST requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (key_lo, key_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let state_lo = self.regs.xmm[xmm_dst][0];
                let state_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = aes::aesenclast(state_lo, state_hi, key_lo, key_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // AESDEC - AES Decrypt Round (0xDE)
            // STATE := InvShiftRows(InvSubBytes(STATE)); STATE := InvMixColumns(STATE); DEST := STATE XOR RoundKey
            0xDE => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("AESDEC requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (key_lo, key_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let state_lo = self.regs.xmm[xmm_dst][0];
                let state_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = aes::aesdec(state_lo, state_hi, key_lo, key_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // AESDECLAST - AES Decrypt Last Round (0xDF)
            // STATE := InvShiftRows(InvSubBytes(STATE)); DEST := STATE XOR RoundKey (no InvMixColumns)
            0xDF => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("AESDECLAST requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (key_lo, key_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let state_lo = self.regs.xmm[xmm_dst][0];
                let state_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = aes::aesdeclast(state_lo, state_hi, key_lo, key_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // ===== SHA-NI Instructions (0xC8-0xCD) =====

            // SHA1NEXTE - Calculate SHA1 state variable E after four rounds (0xC8)
            0xC8 => {
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_dst][0];
                let src1_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = sha::sha1nexte(src1_lo, src1_hi, src2_lo, src2_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // SHA1MSG1 - SHA1 message schedule update 1 (0xC9)
            0xC9 => {
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_dst][0];
                let src1_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = sha::sha1msg1(src1_lo, src1_hi, src2_lo, src2_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // SHA1MSG2 - SHA1 message schedule update 2 (0xCA)
            0xCA => {
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_dst][0];
                let src1_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = sha::sha1msg2(src1_lo, src1_hi, src2_lo, src2_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // SHA256RNDS2 - Perform two rounds of SHA256 (0xCB)
            // Uses XMM0 implicitly as the third operand
            0xCB => {
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_dst][0];
                let src1_hi = self.regs.xmm[xmm_dst][1];
                let xmm0_lo = self.regs.xmm[0][0]; // Implicit XMM0 operand
                let (result_lo, result_hi) =
                    sha::sha256rnds2(src1_lo, src1_hi, src2_lo, src2_hi, xmm0_lo);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // SHA256MSG1 - SHA256 message schedule update 1 (0xCC)
            0xCC => {
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_dst][0];
                let src1_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = sha::sha256msg1(src1_lo, src1_hi, src2_lo, src2_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // SHA256MSG2 - SHA256 message schedule update 2 (0xCD)
            0xCD => {
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let xmm_dst = reg as usize;
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_dst][0];
                let src1_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) = sha::sha256msg2(src1_lo, src1_hi, src2_lo, src2_hi);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // ===== CRC32 / MOVBE Instructions =====
            // CRC32 uses F2 prefix, MOVBE doesn't

            // CRC32 r32, r/m8 (F2 0F 38 F0) or MOVBE r, m16/32/64 (0F 38 F0)
            0xF0 => {
                if ctx.rep_prefix == Some(0xF2) {
                    // CRC32 r32/r64, r/m8
                    let has_rex = ctx.rex.is_some();
                    let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                    let src = if is_memory {
                        self.read_mem(addr, 1)? as u8
                    } else {
                        self.get_reg8(rm, has_rex) as u8
                    };
                    let crc_in = self.get_reg(reg, 4) as u32;
                    let crc_out = crc32c_u8(crc_in, src);
                    if ctx.rex_w() {
                        self.set_reg(reg, crc_out as u64, 8);
                    } else {
                        self.set_reg(reg, crc_out as u64, 4);
                    }
                    self.regs.rip += ctx.cursor as u64;
                    Ok(None)
                } else {
                    // MOVBE r, m16/32/64 (load with byte swap)
                    let (reg, _rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                    if !is_memory {
                        return Err(Error::Emulator("MOVBE requires memory operand".to_string()));
                    }
                    let size = ctx.op_size;
                    let value = self.read_mem(addr, size)?;
                    let swapped = match size {
                        2 => (value as u16).swap_bytes() as u64,
                        4 => (value as u32).swap_bytes() as u64,
                        8 => value.swap_bytes(),
                        _ => value,
                    };
                    self.set_reg(reg, swapped, size);
                    self.regs.rip += ctx.cursor as u64;
                    Ok(None)
                }
            }
            // CRC32 r32, r/m16/32/64 (F2 0F 38 F1) or MOVBE m16/32/64, r (0F 38 F1)
            0xF1 => {
                if ctx.rep_prefix == Some(0xF2) {
                    // CRC32 r32/r64, r/m16/32/64
                    let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                    let crc_in = self.get_reg(reg, 4) as u32;

                    let crc_out = if ctx.rex_w() {
                        // 64-bit source
                        let src = if is_memory {
                            self.read_mem(addr, 8)?
                        } else {
                            self.get_reg(rm, 8)
                        };
                        crc32c_u64(crc_in, src)
                    } else if ctx.operand_size_override {
                        // 16-bit source
                        let src = if is_memory {
                            self.read_mem(addr, 2)? as u16
                        } else {
                            self.get_reg(rm, 2) as u16
                        };
                        crc32c_u16(crc_in, src)
                    } else {
                        // 32-bit source
                        let src = if is_memory {
                            self.read_mem(addr, 4)? as u32
                        } else {
                            self.get_reg(rm, 4) as u32
                        };
                        crc32c_u32(crc_in, src)
                    };

                    if ctx.rex_w() {
                        self.set_reg(reg, crc_out as u64, 8);
                    } else {
                        self.set_reg(reg, crc_out as u64, 4);
                    }
                    self.regs.rip += ctx.cursor as u64;
                    Ok(None)
                } else {
                    // MOVBE m16/32/64, r (store with byte swap)
                    let (reg, _rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                    if !is_memory {
                        return Err(Error::Emulator("MOVBE requires memory operand".to_string()));
                    }
                    let size = ctx.op_size;
                    let value = self.get_reg(reg, size);
                    let swapped = match size {
                        2 => (value as u16).swap_bytes() as u64,
                        4 => (value as u32).swap_bytes() as u64,
                        8 => value.swap_bytes(),
                        _ => value,
                    };
                    self.write_mem(addr, swapped, size)?;
                    self.regs.rip += ctx.cursor as u64;
                    Ok(None)
                }
            }
            // ADCX/ADOX (0xF6) - ADX instructions with mandatory prefixes
            0xF6 => {
                if ctx.rep_prefix == Some(0xF3) {
                    insn::arith::adox_r_rm(self, ctx)
                } else if ctx.operand_size_override {
                    insn::arith::adcx_r_rm(self, ctx)
                } else {
                    Err(Error::Emulator(
                        "ADCX/ADOX requires 66 or F3 prefix".to_string(),
                    ))
                }
            }
            // MOVDIR64B (0xF8)
            0xF8 => insn::data::movdir64b(self, ctx),
            // MOVDIRI (0xF9)
            0xF9 => insn::data::movdiri(self, ctx),

            _ => Err(Error::Emulator(format!(
                "unimplemented 0x0F 0x38 opcode: {:#04x} at RIP={:#x}",
                opcode3, self.regs.rip
            ))),
        }
    }
}

// CRC-32C polynomial (Castagnoli) in reflected form
const CRC32C_POLY: u32 = 0x82F63B78;

/// Compute CRC32C for a single byte
fn crc32c_u8(crc: u32, data: u8) -> u32 {
    let mut crc = crc ^ (data as u32);
    for _ in 0..8 {
        if crc & 1 != 0 {
            crc = (crc >> 1) ^ CRC32C_POLY;
        } else {
            crc >>= 1;
        }
    }
    crc
}

/// Compute CRC32C for a 16-bit word
fn crc32c_u16(crc: u32, data: u16) -> u32 {
    let mut crc = crc;
    crc = crc32c_u8(crc, data as u8);
    crc = crc32c_u8(crc, (data >> 8) as u8);
    crc
}

/// Compute CRC32C for a 32-bit dword
fn crc32c_u32(crc: u32, data: u32) -> u32 {
    let mut crc = crc;
    crc = crc32c_u8(crc, data as u8);
    crc = crc32c_u8(crc, (data >> 8) as u8);
    crc = crc32c_u8(crc, (data >> 16) as u8);
    crc = crc32c_u8(crc, (data >> 24) as u8);
    crc
}

/// Compute CRC32C for a 64-bit qword
fn crc32c_u64(crc: u32, data: u64) -> u32 {
    let mut crc = crc;
    crc = crc32c_u8(crc, data as u8);
    crc = crc32c_u8(crc, (data >> 8) as u8);
    crc = crc32c_u8(crc, (data >> 16) as u8);
    crc = crc32c_u8(crc, (data >> 24) as u8);
    crc = crc32c_u8(crc, (data >> 32) as u8);
    crc = crc32c_u8(crc, (data >> 40) as u8);
    crc = crc32c_u8(crc, (data >> 48) as u8);
    crc = crc32c_u8(crc, (data >> 56) as u8);
    crc
}
