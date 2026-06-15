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
    pub(in crate::backend::emulator::x86_64) fn execute_0f3a(
        &mut self,
        ctx: &mut InsnContext,
    ) -> Result<Option<VcpuExit>> {
        let opcode3 = ctx.consume_u8()?;

        // Record precise opcode key for profiling
        #[cfg(feature = "profiling")]
        crate::profiling::set_current_opcode_key(crate::profiling::OpcodeKey::ThreeByte3A(opcode3));

        match opcode3 {
            // ROUNDPS - Round Packed Single Precision Floating-Point Values (0x08)
            0x08 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("ROUNDPS requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };

                // Round mode from imm8[1:0]: 0=nearest-even, 1=floor, 2=ceil, 3=truncate.
                // Mode 0 is round-to-nearest with ties to EVEN (banker's rounding),
                // NOT Rust's `f32::round` which rounds half AWAY from zero.
                let round_mode = imm8 & 0x03;
                let round_fn = |v: f32| -> f32 {
                    match round_mode {
                        0 => v.round_ties_even(),
                        1 => v.floor(),
                        2 => v.ceil(),
                        3 => v.trunc(),
                        _ => v,
                    }
                };

                let s0 = f32::from_bits(src_lo as u32);
                let s1 = f32::from_bits((src_lo >> 32) as u32);
                let s2 = f32::from_bits(src_hi as u32);
                let s3 = f32::from_bits((src_hi >> 32) as u32);

                let r0 = round_fn(s0).to_bits();
                let r1 = round_fn(s1).to_bits();
                let r2 = round_fn(s2).to_bits();
                let r3 = round_fn(s3).to_bits();

                self.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // ROUNDPD - Round Packed Double Precision Floating-Point Values (0x09)
            0x09 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("ROUNDPD requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };

                let round_mode = imm8 & 0x03;
                let round_fn = |v: f64| -> f64 {
                    match round_mode {
                        0 => v.round_ties_even(),
                        1 => v.floor(),
                        2 => v.ceil(),
                        3 => v.trunc(),
                        _ => v,
                    }
                };

                let s0 = f64::from_bits(src_lo);
                let s1 = f64::from_bits(src_hi);

                self.regs.xmm[xmm_dst][0] = round_fn(s0).to_bits();
                self.regs.xmm[xmm_dst][1] = round_fn(s1).to_bits();
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // ROUNDSS - Round Scalar Single Precision Floating-Point (0x0A)
            0x0A => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("ROUNDSS requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let src = if is_memory {
                    self.read_mem(addr, 4)? as u32
                } else {
                    self.regs.xmm[rm as usize][0] as u32
                };

                let round_mode = imm8 & 0x03;
                let v = f32::from_bits(src);
                let result = match round_mode {
                    0 => v.round_ties_even(),
                    1 => v.floor(),
                    2 => v.ceil(),
                    3 => v.trunc(),
                    _ => v,
                };

                // Only update the low 32 bits, keep the rest
                self.regs.xmm[xmm_dst][0] =
                    (self.regs.xmm[xmm_dst][0] & 0xFFFF_FFFF_0000_0000) | (result.to_bits() as u64);
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // ROUNDSD - Round Scalar Double Precision Floating-Point (0x0B)
            0x0B => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("ROUNDSD requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let src = if is_memory {
                    self.read_mem(addr, 8)?
                } else {
                    self.regs.xmm[rm as usize][0]
                };

                let round_mode = imm8 & 0x03;
                let v = f64::from_bits(src);
                let result = match round_mode {
                    0 => v.round_ties_even(),
                    1 => v.floor(),
                    2 => v.ceil(),
                    3 => v.trunc(),
                    _ => v,
                };

                // Only update the low 64 bits, keep xmm[1] unchanged
                self.regs.xmm[xmm_dst][0] = result.to_bits();
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // BLENDPS - Blend Packed Single Precision Floating-Point Values (0x0C)
            0x0C => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("BLENDPS requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm_dst][0];
                let dst_hi = self.regs.xmm[xmm_dst][1];

                // Each bit in imm8[3:0] selects src (1) or dst (0) for each 32-bit element
                let r0 = if imm8 & 0x01 != 0 {
                    src_lo & 0xFFFF_FFFF
                } else {
                    dst_lo & 0xFFFF_FFFF
                };
                let r1 = if imm8 & 0x02 != 0 {
                    src_lo & 0xFFFF_FFFF_0000_0000
                } else {
                    dst_lo & 0xFFFF_FFFF_0000_0000
                };
                let r2 = if imm8 & 0x04 != 0 {
                    src_hi & 0xFFFF_FFFF
                } else {
                    dst_hi & 0xFFFF_FFFF
                };
                let r3 = if imm8 & 0x08 != 0 {
                    src_hi & 0xFFFF_FFFF_0000_0000
                } else {
                    dst_hi & 0xFFFF_FFFF_0000_0000
                };

                self.regs.xmm[xmm_dst][0] = r0 | r1;
                self.regs.xmm[xmm_dst][1] = r2 | r3;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // BLENDPD - Blend Packed Double Precision Floating-Point Values (0x0D)
            0x0D => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("BLENDPD requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm_dst][0];
                let dst_hi = self.regs.xmm[xmm_dst][1];

                // imm8[0] selects low qword, imm8[1] selects high qword
                self.regs.xmm[xmm_dst][0] = if imm8 & 0x01 != 0 { src_lo } else { dst_lo };
                self.regs.xmm[xmm_dst][1] = if imm8 & 0x02 != 0 { src_hi } else { dst_hi };
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PBLENDW - Blend Packed Words (0x0E)
            0x0E => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PBLENDW requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm_dst][0];
                let dst_hi = self.regs.xmm[xmm_dst][1];

                // Each bit in imm8 selects one of 8 words
                let mut result_lo = 0u64;
                let mut result_hi = 0u64;
                for i in 0..4 {
                    let mask = 0xFFFFu64 << (i * 16);
                    if imm8 & (1 << i) != 0 {
                        result_lo |= src_lo & mask;
                    } else {
                        result_lo |= dst_lo & mask;
                    }
                }
                for i in 0..4 {
                    let mask = 0xFFFFu64 << (i * 16);
                    if imm8 & (1 << (i + 4)) != 0 {
                        result_hi |= src_hi & mask;
                    } else {
                        result_hi |= dst_hi & mask;
                    }
                }
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PALIGNR - Packed Align Right (0x0F)
            0x0F => {
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;

                if ctx.operand_size_override {
                    // XMM version: concatenate dst:src (256 bits), shift right by imm8 bytes
                    let xmm_dst = reg as usize;
                    let (src_lo, src_hi) = if is_memory {
                        (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                    } else {
                        (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                    };
                    let dst_lo = self.regs.xmm[xmm_dst][0];
                    let dst_hi = self.regs.xmm[xmm_dst][1];

                    let shift = (imm8 as usize) * 8;
                    if shift >= 256 {
                        self.regs.xmm[xmm_dst][0] = 0;
                        self.regs.xmm[xmm_dst][1] = 0;
                    } else if shift >= 128 {
                        // Shift only uses dst bits
                        let s = shift - 128;
                        if s >= 64 {
                            self.regs.xmm[xmm_dst][0] = dst_hi >> (s - 64);
                            self.regs.xmm[xmm_dst][1] = 0;
                        } else if s > 0 {
                            self.regs.xmm[xmm_dst][0] = (dst_lo >> s) | (dst_hi << (64 - s));
                            self.regs.xmm[xmm_dst][1] = dst_hi >> s;
                        } else {
                            self.regs.xmm[xmm_dst][0] = dst_lo;
                            self.regs.xmm[xmm_dst][1] = dst_hi;
                        }
                    } else if shift >= 64 {
                        // Crosses from src to dst
                        let s = shift - 64;
                        if s > 0 {
                            self.regs.xmm[xmm_dst][0] = (src_hi >> s) | (dst_lo << (64 - s));
                            self.regs.xmm[xmm_dst][1] = (dst_lo >> s) | (dst_hi << (64 - s));
                        } else {
                            self.regs.xmm[xmm_dst][0] = src_hi;
                            self.regs.xmm[xmm_dst][1] = dst_lo;
                        }
                    } else if shift > 0 {
                        // Shift within src part
                        self.regs.xmm[xmm_dst][0] = (src_lo >> shift) | (src_hi << (64 - shift));
                        self.regs.xmm[xmm_dst][1] = (src_hi >> shift) | (dst_lo << (64 - shift));
                    }
                    // shift == 0: no change needed
                } else {
                    // MMX version: concatenate dst:src (128 bits), shift right by imm8 bytes
                    let mm_dst = reg as usize & 0x7;
                    let src = if is_memory {
                        self.read_mem(addr, 8)?
                    } else {
                        self.regs.mm[rm as usize & 0x7]
                    };
                    let dst = self.regs.mm[mm_dst];

                    let shift = (imm8 as usize) * 8;
                    if shift >= 128 {
                        self.regs.mm[mm_dst] = 0;
                    } else if shift >= 64 {
                        self.regs.mm[mm_dst] = dst >> (shift - 64);
                    } else if shift > 0 {
                        self.regs.mm[mm_dst] = (src >> shift) | (dst << (64 - shift));
                    }
                    // shift == 0: no change
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PEXTRB - Extract Byte (0x14)
            0x14 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PEXTRB requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_src = reg as usize;
                let idx = (imm8 & 0x0F) as usize;

                // Get byte from XMM register
                let byte = if idx < 8 {
                    ((self.regs.xmm[xmm_src][0] >> (idx * 8)) & 0xFF) as u8
                } else {
                    ((self.regs.xmm[xmm_src][1] >> ((idx - 8) * 8)) & 0xFF) as u8
                };

                if is_memory {
                    self.write_mem(addr, byte as u64, 1)?;
                } else {
                    // Zero-extend to 32/64 bit register
                    let dest_size = if ctx.rex_w() { 8 } else { 4 };
                    self.set_reg(rm, byte as u64, dest_size);
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PEXTRW - Extract Word (0x15) - note: this is actually in 0F C5, but 0F 3A 15 also exists
            0x15 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PEXTRW requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_src = reg as usize;
                let idx = (imm8 & 0x07) as usize;

                let word = if idx < 4 {
                    ((self.regs.xmm[xmm_src][0] >> (idx * 16)) & 0xFFFF) as u16
                } else {
                    ((self.regs.xmm[xmm_src][1] >> ((idx - 4) * 16)) & 0xFFFF) as u16
                };

                if is_memory {
                    self.write_mem(addr, word as u64, 2)?;
                } else {
                    let dest_size = if ctx.rex_w() { 8 } else { 4 };
                    self.set_reg(rm, word as u64, dest_size);
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PEXTRD/PEXTRQ - Extract Dword/Qword (0x16)
            0x16 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PEXTRD/Q requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_src = reg as usize;

                if ctx.rex_w() {
                    // PEXTRQ - extract qword
                    let idx = (imm8 & 0x01) as usize;
                    let value = self.regs.xmm[xmm_src][idx];
                    if is_memory {
                        self.write_mem(addr, value, 8)?;
                    } else {
                        self.set_reg(rm, value, 8);
                    }
                } else {
                    // PEXTRD - extract dword
                    let idx = (imm8 & 0x03) as usize;
                    let value = if idx < 2 {
                        ((self.regs.xmm[xmm_src][0] >> (idx * 32)) & 0xFFFF_FFFF) as u32
                    } else {
                        ((self.regs.xmm[xmm_src][1] >> ((idx - 2) * 32)) & 0xFFFF_FFFF) as u32
                    };
                    if is_memory {
                        self.write_mem(addr, value as u64, 4)?;
                    } else {
                        self.set_reg(rm, value as u64, 4);
                    }
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // EXTRACTPS - Extract Packed Single Precision Floating-Point (0x17)
            0x17 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("EXTRACTPS requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_src = reg as usize;
                let idx = (imm8 & 0x03) as usize;

                let value = if idx < 2 {
                    ((self.regs.xmm[xmm_src][0] >> (idx * 32)) & 0xFFFF_FFFF) as u32
                } else {
                    ((self.regs.xmm[xmm_src][1] >> ((idx - 2) * 32)) & 0xFFFF_FFFF) as u32
                };

                if is_memory {
                    self.write_mem(addr, value as u64, 4)?;
                } else {
                    self.set_reg(rm, value as u64, 4);
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PINSRB - Insert Byte (0x20)
            0x20 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PINSRB requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let idx = (imm8 & 0x0F) as usize;

                let byte = if is_memory {
                    self.read_mem(addr, 1)? as u8
                } else {
                    self.get_reg(rm, 1) as u8
                };

                // Insert byte at position idx
                if idx < 8 {
                    let mask = !(0xFFu64 << (idx * 8));
                    self.regs.xmm[xmm_dst][0] =
                        (self.regs.xmm[xmm_dst][0] & mask) | ((byte as u64) << (idx * 8));
                } else {
                    let pos = idx - 8;
                    let mask = !(0xFFu64 << (pos * 8));
                    self.regs.xmm[xmm_dst][1] =
                        (self.regs.xmm[xmm_dst][1] & mask) | ((byte as u64) << (pos * 8));
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // INSERTPS - Insert Packed Single Precision Floating-Point (0x21)
            0x21 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("INSERTPS requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;

                // imm8[7:6] = count_s (source element select for register source)
                // imm8[5:4] = count_d (destination element)
                // imm8[3:0] = zmask (zero mask)
                let count_s = ((imm8 >> 6) & 0x03) as usize;
                let count_d = ((imm8 >> 4) & 0x03) as usize;
                let zmask = imm8 & 0x0F;

                // Get source value
                let src_val = if is_memory {
                    self.read_mem(addr, 4)? as u32
                } else {
                    let xmm_src = rm as usize;
                    if count_s < 2 {
                        ((self.regs.xmm[xmm_src][0] >> (count_s * 32)) & 0xFFFF_FFFF) as u32
                    } else {
                        ((self.regs.xmm[xmm_src][1] >> ((count_s - 2) * 32)) & 0xFFFF_FFFF) as u32
                    }
                };

                // Insert into destination position
                if count_d < 2 {
                    let mask = !(0xFFFF_FFFFu64 << (count_d * 32));
                    self.regs.xmm[xmm_dst][0] =
                        (self.regs.xmm[xmm_dst][0] & mask) | ((src_val as u64) << (count_d * 32));
                } else {
                    let pos = count_d - 2;
                    let mask = !(0xFFFF_FFFFu64 << (pos * 32));
                    self.regs.xmm[xmm_dst][1] =
                        (self.regs.xmm[xmm_dst][1] & mask) | ((src_val as u64) << (pos * 32));
                }

                // Apply zero mask
                for i in 0..4 {
                    if zmask & (1 << i) != 0 {
                        if i < 2 {
                            self.regs.xmm[xmm_dst][0] &= !(0xFFFF_FFFFu64 << (i * 32));
                        } else {
                            self.regs.xmm[xmm_dst][1] &= !(0xFFFF_FFFFu64 << ((i - 2) * 32));
                        }
                    }
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PINSRD/PINSRQ - Insert Dword/Qword (0x22)
            0x22 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PINSRD/Q requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;

                if ctx.rex_w() {
                    // PINSRQ - insert qword
                    let idx = (imm8 & 0x01) as usize;
                    let value = if is_memory {
                        self.read_mem(addr, 8)?
                    } else {
                        self.get_reg(rm, 8)
                    };
                    self.regs.xmm[xmm_dst][idx] = value;
                } else {
                    // PINSRD - insert dword
                    let idx = (imm8 & 0x03) as usize;
                    let value = if is_memory {
                        self.read_mem(addr, 4)? as u32
                    } else {
                        self.get_reg(rm, 4) as u32
                    };

                    if idx < 2 {
                        let mask = !(0xFFFF_FFFFu64 << (idx * 32));
                        self.regs.xmm[xmm_dst][0] =
                            (self.regs.xmm[xmm_dst][0] & mask) | ((value as u64) << (idx * 32));
                    } else {
                        let pos = idx - 2;
                        let mask = !(0xFFFF_FFFFu64 << (pos * 32));
                        self.regs.xmm[xmm_dst][1] =
                            (self.regs.xmm[xmm_dst][1] & mask) | ((value as u64) << (pos * 32));
                    }
                }
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // DPPS - Dot Product of Packed Single Precision Floating-Point (0x40)
            0x40 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("DPPS requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm_dst][0];
                let dst_hi = self.regs.xmm[xmm_dst][1];

                // imm8[7:4] = input mask, imm8[3:0] = output mask
                let in_mask = (imm8 >> 4) & 0x0F;
                let out_mask = imm8 & 0x0F;

                // Get all 4 floats from both operands
                let d0 = f32::from_bits(dst_lo as u32);
                let d1 = f32::from_bits((dst_lo >> 32) as u32);
                let d2 = f32::from_bits(dst_hi as u32);
                let d3 = f32::from_bits((dst_hi >> 32) as u32);
                let s0 = f32::from_bits(src_lo as u32);
                let s1 = f32::from_bits((src_lo >> 32) as u32);
                let s2 = f32::from_bits(src_hi as u32);
                let s3 = f32::from_bits((src_hi >> 32) as u32);

                // Compute dot product with input mask
                let mut dp = 0.0f32;
                if in_mask & 0x01 != 0 {
                    dp += d0 * s0;
                }
                if in_mask & 0x02 != 0 {
                    dp += d1 * s1;
                }
                if in_mask & 0x04 != 0 {
                    dp += d2 * s2;
                }
                if in_mask & 0x08 != 0 {
                    dp += d3 * s3;
                }

                // Store result with output mask
                let dp_bits = dp.to_bits();
                let r0 = if out_mask & 0x01 != 0 { dp_bits } else { 0 };
                let r1 = if out_mask & 0x02 != 0 { dp_bits } else { 0 };
                let r2 = if out_mask & 0x04 != 0 { dp_bits } else { 0 };
                let r3 = if out_mask & 0x08 != 0 { dp_bits } else { 0 };

                self.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
                self.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // DPPD - Dot Product of Packed Double Precision Floating-Point (0x41)
            0x41 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("DPPD requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm_dst][0];
                let dst_hi = self.regs.xmm[xmm_dst][1];

                // imm8[5:4] = input mask, imm8[1:0] = output mask
                let in_mask = (imm8 >> 4) & 0x03;
                let out_mask = imm8 & 0x03;

                let d0 = f64::from_bits(dst_lo);
                let d1 = f64::from_bits(dst_hi);
                let s0 = f64::from_bits(src_lo);
                let s1 = f64::from_bits(src_hi);

                let mut dp = 0.0f64;
                if in_mask & 0x01 != 0 {
                    dp += d0 * s0;
                }
                if in_mask & 0x02 != 0 {
                    dp += d1 * s1;
                }

                let dp_bits = dp.to_bits();
                self.regs.xmm[xmm_dst][0] = if out_mask & 0x01 != 0 { dp_bits } else { 0 };
                self.regs.xmm[xmm_dst][1] = if out_mask & 0x02 != 0 { dp_bits } else { 0 };
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // MPSADBW - Multiple Sum of Absolute Differences (0x42)
            0x42 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("MPSADBW requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm_dst][0];
                let dst_hi = self.regs.xmm[xmm_dst][1];

                // imm8[1:0] selects source block offset (0, 4, 8, or 12 bytes)
                // imm8[2] selects destination block offset (0 or 4 bytes)
                let src_offset = ((imm8 & 0x03) * 4) as usize;
                let dst_offset = if imm8 & 0x04 != 0 { 4 } else { 0 };

                // Get source bytes (need 4 consecutive bytes starting at src_offset)
                let get_src_byte = |idx: usize| -> u8 {
                    let actual_idx = src_offset + idx;
                    if actual_idx < 8 {
                        ((src_lo >> (actual_idx * 8)) & 0xFF) as u8
                    } else {
                        ((src_hi >> ((actual_idx - 8) * 8)) & 0xFF) as u8
                    }
                };

                // Get destination bytes
                let get_dst_byte = |idx: usize| -> u8 {
                    if idx < 8 {
                        ((dst_lo >> (idx * 8)) & 0xFF) as u8
                    } else {
                        ((dst_hi >> ((idx - 8) * 8)) & 0xFF) as u8
                    }
                };

                // Compute 8 SADs
                let mut results = [0u16; 8];
                for i in 0..8 {
                    let mut sad = 0u16;
                    for j in 0..4 {
                        let s = get_src_byte(j) as i16;
                        let d = get_dst_byte(dst_offset + i + j) as i16;
                        sad += (s - d).unsigned_abs();
                    }
                    results[i] = sad;
                }

                self.regs.xmm[xmm_dst][0] = (results[0] as u64)
                    | ((results[1] as u64) << 16)
                    | ((results[2] as u64) << 32)
                    | ((results[3] as u64) << 48);
                self.regs.xmm[xmm_dst][1] = (results[4] as u64)
                    | ((results[5] as u64) << 16)
                    | ((results[6] as u64) << 32)
                    | ((results[7] as u64) << 48);
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PCLMULQDQ - Carry-Less Multiplication Quadword (0x44)
            0x44 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PCLMULQDQ requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm_dst][0];
                let dst_hi = self.regs.xmm[xmm_dst][1];

                // imm8[0] selects which qword from dst, imm8[4] selects which qword from src
                let a = if imm8 & 0x01 != 0 { dst_hi } else { dst_lo };
                let b = if imm8 & 0x10 != 0 { src_hi } else { src_lo };

                // Carry-less multiplication (XOR instead of add in polynomial multiplication)
                let mut result_lo = 0u64;
                let mut result_hi = 0u64;

                for i in 0..64 {
                    if (b >> i) & 1 != 0 {
                        // XOR a shifted left by i
                        if i == 0 {
                            result_lo ^= a;
                        } else {
                            result_lo ^= a << i;
                            result_hi ^= a >> (64 - i);
                        }
                    }
                }

                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PCMPESTRM - Packed Compare Explicit Length Strings, Return Mask (0x60)
            0x60 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PCMPESTRM requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm1 = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm1][0];
                let dst_hi = self.regs.xmm[xmm1][1];

                // Explicit length from EAX (dst) and EDX (src)
                let len1 = (self.regs.rax & 0xFFFF_FFFF) as i32;
                let len2 = (self.regs.rdx & 0xFFFF_FFFF) as i32;

                // Perform string comparison and return mask in XMM0
                let result =
                    self.pcmpxstrx(dst_lo, dst_hi, src_lo, src_hi, len1, len2, imm8, true)?;
                self.regs.xmm[0][0] = result as u64;
                self.regs.xmm[0][1] = (result >> 64) as u64;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PCMPESTRI - Packed Compare Explicit Length Strings, Return Index (0x61)
            0x61 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PCMPESTRI requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm1 = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm1][0];
                let dst_hi = self.regs.xmm[xmm1][1];

                let len1 = (self.regs.rax & 0xFFFF_FFFF) as i32;
                let len2 = (self.regs.rdx & 0xFFFF_FFFF) as i32;

                let result =
                    self.pcmpxstrx(dst_lo, dst_hi, src_lo, src_hi, len1, len2, imm8, false)?;
                self.regs.rcx = result as u64;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PCMPISTRM - Packed Compare Implicit Length Strings, Return Mask (0x62)
            0x62 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PCMPISTRM requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm1 = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm1][0];
                let dst_hi = self.regs.xmm[xmm1][1];

                // Implicit length - find null terminator
                let len1 = self.find_null_terminator(dst_lo, dst_hi, imm8);
                let len2 = self.find_null_terminator(src_lo, src_hi, imm8);

                let result =
                    self.pcmpxstrx(dst_lo, dst_hi, src_lo, src_hi, len1, len2, imm8, true)?;
                self.regs.xmm[0][0] = result as u64;
                self.regs.xmm[0][1] = (result >> 64) as u64;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // PCMPISTRI - Packed Compare Implicit Length Strings, Return Index (0x63)
            0x63 => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator("PCMPISTRI requires 66 prefix".to_string()));
                }
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm1 = reg as usize;
                let (src_lo, src_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let dst_lo = self.regs.xmm[xmm1][0];
                let dst_hi = self.regs.xmm[xmm1][1];

                let len1 = self.find_null_terminator(dst_lo, dst_hi, imm8);
                let len2 = self.find_null_terminator(src_lo, src_hi, imm8);

                let result =
                    self.pcmpxstrx(dst_lo, dst_hi, src_lo, src_hi, len1, len2, imm8, false)?;

                self.regs.rcx = result as u64;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // SHA1RNDS4 - Perform Four Rounds of SHA1 Operation (0xCC)
            0xCC => {
                let (reg, rm, is_memory, addr, _) = self.decode_modrm(ctx)?;
                let imm8 = ctx.consume_u8()?;
                let xmm_dst = reg as usize;
                let (src2_lo, src2_hi) = if is_memory {
                    (self.read_mem(addr, 8)?, self.read_mem(addr + 8, 8)?)
                } else {
                    (self.regs.xmm[rm as usize][0], self.regs.xmm[rm as usize][1])
                };
                let src1_lo = self.regs.xmm[xmm_dst][0];
                let src1_hi = self.regs.xmm[xmm_dst][1];
                let (result_lo, result_hi) =
                    sha::sha1rnds4(src1_lo, src1_hi, src2_lo, src2_hi, imm8);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // AESKEYGENASSIST - AES Key Generation Assist (0xDF)
            // DEST[31:0] := SubWord(X1)
            // DEST[63:32] := RotWord(SubWord(X1)) XOR RCON
            // DEST[95:64] := SubWord(X3)
            // DEST[127:96] := RotWord(SubWord(X3)) XOR RCON
            0xDF => {
                if !ctx.operand_size_override {
                    return Err(Error::Emulator(
                        "AESKEYGENASSIST requires 66 prefix".to_string(),
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

                let (result_lo, result_hi) = aes::aeskeygenassist(src_lo, src_hi, imm8);
                self.regs.xmm[xmm_dst][0] = result_lo;
                self.regs.xmm[xmm_dst][1] = result_hi;
                self.regs.rip += ctx.cursor as u64;
                Ok(None)
            }

            // GFNI affine transforms (66 0F 3A CE/CF /r ib)
            0xCE => insn::simd::gf2p8affineqb(self, ctx),
            0xCF => insn::simd::gf2p8affineinvqb(self, ctx),

            _ => Err(Error::Emulator(format!(
                "unimplemented 0x0F 0x3A opcode: {:#04x} at RIP={:#x}",
                opcode3, self.regs.rip
            ))),
        }
    }

    /// Helper for PCMPxSTRx instructions - find null terminator position
    fn find_null_terminator(&self, lo: u64, hi: u64, imm8: u8) -> i32 {
        let is_word = (imm8 & 0x01) != 0;
        if is_word {
            // Word elements (8 elements)
            for i in 0..4 {
                if ((lo >> (i * 16)) & 0xFFFF) == 0 {
                    return i as i32;
                }
            }
            for i in 0..4 {
                if ((hi >> (i * 16)) & 0xFFFF) == 0 {
                    return (i + 4) as i32;
                }
            }
            8
        } else {
            // Byte elements (16 elements)
            for i in 0..8 {
                if ((lo >> (i * 8)) & 0xFF) == 0 {
                    return i as i32;
                }
            }
            for i in 0..8 {
                if ((hi >> (i * 8)) & 0xFF) == 0 {
                    return (i + 8) as i32;
                }
            }
            16
        }
    }

    /// Helper for PCMPxSTRx instructions - perform comparison
    fn pcmpxstrx(
        &mut self,
        dst_lo: u64,
        dst_hi: u64,
        src_lo: u64,
        src_hi: u64,
        len1: i32,
        len2: i32,
        imm8: u8,
        return_mask: bool,
    ) -> Result<u128> {
        let is_word = (imm8 & 0x01) != 0;
        let is_signed = (imm8 & 0x02) != 0;
        let agg_op = (imm8 >> 2) & 0x03;
        let polarity = (imm8 >> 4) & 0x03;
        let output_sel = (imm8 >> 6) & 0x01;

        let num_elements = if is_word { 8 } else { 16 };
        // PCMPESTRx takes the operand lengths from EAX/EDX as a SIGNED value
        // whose ABSOLUTE value is the length, saturated to the element count
        // (Intel SDM): e.g. EAX = -3 means length 3, not 0. PCMPISTRx passes a
        // find_null_terminator() result already in [0, num_elements], for which
        // unsigned_abs() is a no-op. The previous `.clamp(0, num_elements)`
        // wrongly mapped a negative explicit length to 0.
        let valid1 = len1.unsigned_abs().min(num_elements as u32) as i32;
        let valid2 = len2.unsigned_abs().min(num_elements as u32) as i32;

        // Get elements from operands
        let get_elem = |lo: u64, hi: u64, idx: usize| -> i32 {
            if is_word {
                let val = if idx < 4 {
                    ((lo >> (idx * 16)) & 0xFFFF) as u16
                } else {
                    ((hi >> ((idx - 4) * 16)) & 0xFFFF) as u16
                };
                if is_signed {
                    val as i16 as i32
                } else {
                    val as i32
                }
            } else {
                let val = if idx < 8 {
                    ((lo >> (idx * 8)) & 0xFF) as u8
                } else {
                    ((hi >> ((idx - 8) * 8)) & 0xFF) as u8
                };
                if is_signed {
                    val as i8 as i32
                } else {
                    val as i32
                }
            }
        };

        // Build intermediate result based on aggregation operation
        let mut int_res1 = 0u16;

        match agg_op {
            0 => {
                // Equal any - check if each char in src2 is in src1
                for j in 0..num_elements as usize {
                    if j >= valid2 as usize {
                        break;
                    }
                    let s2 = get_elem(src_lo, src_hi, j);
                    for i in 0..valid1 as usize {
                        let s1 = get_elem(dst_lo, dst_hi, i);
                        if s1 == s2 {
                            int_res1 |= 1 << j;
                            break;
                        }
                    }
                }
            }
            1 => {
                // Ranges - check if each char in src2 is in range pairs from src1
                for j in 0..num_elements as usize {
                    if j >= valid2 as usize {
                        break;
                    }
                    let s2 = get_elem(src_lo, src_hi, j);
                    let mut i = 0;
                    while i + 1 < valid1 as usize {
                        let lo_range = get_elem(dst_lo, dst_hi, i);
                        let hi_range = get_elem(dst_lo, dst_hi, i + 1);
                        if s2 >= lo_range && s2 <= hi_range {
                            int_res1 |= 1 << j;
                            break;
                        }
                        i += 2;
                    }
                }
            }
            2 => {
                // Equal each - compare corresponding elements, applying the
                // Intel SDM "valid/invalid override" (Vol. 2B, PCMPxSTRx):
                //   both elements valid    -> a[i] == b[i]
                //   exactly one invalid    -> 0 (force false)
                //   both invalid           -> 1 (force true)
                // The both-invalid -> 1 rule is essential: it is exactly what
                // makes the strlen/terminator idiom `pcmpistri $0x3a,xmm,xmm`
                // (self-compare, EQUAL_EACH + masked-negative polarity) report
                // the NUL position as the result index. The previous code broke
                // out of the loop at min(valid1,valid2) and left the post-NUL
                // bits 0, so that idiom returned `num_elements` (no terminator)
                // and callers like glibc __strcspn_sse42 walked off the string.
                for i in 0..num_elements as usize {
                    let v1 = i < valid1 as usize;
                    let v2 = i < valid2 as usize;
                    let bit = if v1 && v2 {
                        let s1 = get_elem(dst_lo, dst_hi, i);
                        let s2 = get_elem(src_lo, src_hi, i);
                        (s1 == s2) as u16
                    } else if !v1 && !v2 {
                        1
                    } else {
                        0
                    };
                    int_res1 |= bit << i;
                }
            }
            3 => {
                // Equal ordered - substring search: is operand1 (the needle)
                // found in operand2 (the haystack) starting at position j? Per
                // the Intel SDM, IntRes1[j] is computed for ALL j in 0..n (so an
                // empty needle matches at every position) as the AND over needle
                // indices i of an overridden per-element boolean:
                //   i+j beyond the vector (>= n)   -> 1 (no constraint: a partial
                //                                    match running off the end of
                //                                    the window still counts — the
                //                                    caller, e.g. strstr, rechecks)
                //   needle exhausted (i >= valid1) -> 1
                //   haystack past its NUL          -> 0  (i+j >= valid2, in-vector)
                //   both valid                     -> needle[i] == haystack[i+j]
                // The previous code broke at j>=valid2 (dropping the empty-needle
                // case and the high bits that matter once polarity negates them)
                // and conflated "beyond the vector" with "past the NUL", so it
                // returned no-match where hardware reports a tail-partial match.
                let nn = num_elements as usize;
                let v1 = valid1 as usize;
                let v2 = valid2 as usize;
                for j in 0..nn {
                    let mut matched = true;
                    for i in 0..nn {
                        if i + j >= nn || i >= v1 {
                            break; // remaining terms are forced to 1 (match)
                        }
                        if i + j >= v2
                            || get_elem(dst_lo, dst_hi, i) != get_elem(src_lo, src_hi, i + j)
                        {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        int_res1 |= 1 << j;
                    }
                }
            }
            _ => {}
        }

        // Apply polarity
        // Use u32 for the mask to avoid overflow when num_elements = 16
        let mask = if num_elements == 16 {
            0xFFFFu16
        } else {
            ((1u16 << num_elements) - 1)
        };
        let int_res2 = match polarity {
            0 => int_res1,         // Positive
            1 => !int_res1 & mask, // Negative
            2 => int_res1,         // Masked positive
            3 => {
                // Masked negative: XOR with valid mask
                let valid_mask = if valid2 == 16 {
                    0xFFFFu16
                } else if valid2 == 0 {
                    0
                } else {
                    (1u16 << valid2) - 1
                };
                (int_res1 ^ valid_mask) & mask
            }
            _ => int_res1,
        };

        // Set flags
        // CF = int_res2 != 0
        // ZF = any byte/word of src2 is null (len2 < num_elements)
        // SF = any byte/word of src1 is null (len1 < num_elements)
        // OF = int_res2[0]
        // AF = 0, PF = 0
        // CRITICAL: Clear lazy flags before setting flags directly to prevent
        // materialize_flags() from overwriting our flag settings
        self.clear_lazy_flags();
        use super::super::super::super::flags::bits;
        self.regs.rflags &= !(bits::CF | bits::ZF | bits::SF | bits::OF | bits::AF | bits::PF);
        if int_res2 != 0 {
            self.regs.rflags |= bits::CF;
        }
        if valid2 < num_elements {
            self.regs.rflags |= bits::ZF;
        }
        if valid1 < num_elements {
            self.regs.rflags |= bits::SF;
        }
        if int_res2 & 1 != 0 {
            self.regs.rflags |= bits::OF;
        }

        if return_mask {
            // Return mask (for PCMPESTRM/PCMPISTRM)
            if output_sel != 0 {
                // Expanded byte/word mask: each matching element becomes all-ones
                // across the full 128-bit destination.
                if is_word {
                    let mut result = 0u128;
                    for i in 0..8 {
                        if int_res2 & (1 << i) != 0 {
                            result |= 0xFFFFu128 << (i * 16);
                        }
                    }
                    Ok(result)
                } else {
                    let mut result = 0u128;
                    for i in 0..16 {
                        if int_res2 & (1 << i) != 0 {
                            result |= 0xFFu128 << (i * 8);
                        }
                    }
                    Ok(result)
                }
            } else {
                // Bit mask: result is in the low bits, high bits zero.
                Ok(int_res2 as u128)
            }
        } else {
            // Return index (for PCMPESTRI/PCMPISTRI)
            if output_sel != 0 {
                // MSB - find most significant set bit
                for i in (0..num_elements as usize).rev() {
                    if int_res2 & (1 << i) != 0 {
                        return Ok(i as u128);
                    }
                }
                Ok(num_elements as u128)
            } else {
                // LSB - find least significant set bit
                for i in 0..num_elements as usize {
                    if int_res2 & (1 << i) != 0 {
                        return Ok(i as u128);
                    }
                }
                Ok(num_elements as u128)
            }
        }
    }
}
