//! SSSE3 instruction implementations (0x0F 0x38 0x00-0x0B).
//!
//! Instructions: PSHUFB, PHADDW, PHADDD, PHADDSW, PMADDUBSW, PHSUBW, PHSUBD,
//!               PHSUBSW, PSIGNB, PSIGNW, PSIGND, PMULHRSW, PABSB, PABSW, PABSD

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

// =============================================================================
// Helper Functions
// =============================================================================

fn sat_add_i16(a: i16, b: i16) -> i16 {
    (a as i32 + b as i32).clamp(-32768, 32767) as i16
}

fn sat_sub_i16(a: i16, b: i16) -> i16 {
    (a as i32 - b as i32).clamp(-32768, 32767) as i16
}

fn maddubs(a: u8, b: i8, c: u8, d: i8) -> i16 {
    // Each unsigned*signed product fits in i16, but their sum can reach +-65280,
    // so accumulate in i32 before signed-saturating to a word (PMADDUBSW).
    let prod1 = (a as i32) * (b as i32);
    let prod2 = (c as i32) * (d as i32);
    (prod1 + prod2).clamp(-32768, 32767) as i16
}

fn sign_byte(dst: u8, src: i8) -> u8 {
    if src < 0 {
        (-(dst as i8)) as u8
    } else if src == 0 {
        0
    } else {
        dst
    }
}

fn sign_word(dst: u16, src: i16) -> u16 {
    if src < 0 {
        (-(dst as i16)) as u16
    } else if src == 0 {
        0
    } else {
        dst
    }
}

fn sign_dword(dst: u32, src: i32) -> u32 {
    if src < 0 {
        (-(dst as i32)) as u32
    } else if src == 0 {
        0
    } else {
        dst
    }
}

fn mulhrsw(a: i16, b: i16) -> i16 {
    let prod = (a as i32) * (b as i32);
    (((prod >> 14) + 1) >> 1) as i16
}

// =============================================================================
// PSHUFB - Packed Shuffle Bytes (0x00)
// =============================================================================

/// PSHUFB xmm1, xmm2/m128 (66 0F 38 00) or mm1, mm2/m64 (NP 0F 38 00)
pub fn pshufb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        // 66 0F 38 00: PSHUFB xmm1, xmm2/m128
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        // Combine dst into byte array for indexing
        let dst_bytes: [u8; 16] = {
            let mut arr = [0u8; 16];
            arr[0..8].copy_from_slice(&dst_lo.to_le_bytes());
            arr[8..16].copy_from_slice(&dst_hi.to_le_bytes());
            arr
        };

        // Process each byte: if high bit set, result is 0; else index into dst
        let mut result = [0u8; 16];
        let mask_bytes: [u8; 16] = {
            let mut arr = [0u8; 16];
            arr[0..8].copy_from_slice(&src_lo.to_le_bytes());
            arr[8..16].copy_from_slice(&src_hi.to_le_bytes());
            arr
        };
        for i in 0..16 {
            let mask = mask_bytes[i];
            result[i] = if mask & 0x80 != 0 {
                0
            } else {
                dst_bytes[(mask & 0x0F) as usize]
            };
        }
        vcpu.regs.xmm[xmm_dst][0] = u64::from_le_bytes(result[0..8].try_into().unwrap());
        vcpu.regs.xmm[xmm_dst][1] = u64::from_le_bytes(result[8..16].try_into().unwrap());
    } else {
        // NP 0F 38 00: PSHUFB mm1, mm2/m64 (MMX)
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let dst_bytes = dst.to_le_bytes();
        let mask_bytes = src.to_le_bytes();
        let mut result = [0u8; 8];
        for i in 0..8 {
            let mask = mask_bytes[i];
            result[i] = if mask & 0x80 != 0 {
                0
            } else {
                dst_bytes[(mask & 0x07) as usize]
            };
        }
        vcpu.regs.mm[mm_dst] = u64::from_le_bytes(result);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PHADDW - Packed Horizontal Add Words (0x01)
// =============================================================================

/// PHADDW xmm1, xmm2/m128 (66 0F 38 01) or mm1, mm2/m64 (NP 0F 38 01)
pub fn phaddw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        // Extract words and add pairs horizontally
        let d = [
            (dst_lo as u16).wrapping_add((dst_lo >> 16) as u16),
            ((dst_lo >> 32) as u16).wrapping_add((dst_lo >> 48) as u16),
            (dst_hi as u16).wrapping_add((dst_hi >> 16) as u16),
            ((dst_hi >> 32) as u16).wrapping_add((dst_hi >> 48) as u16),
            (src_lo as u16).wrapping_add((src_lo >> 16) as u16),
            ((src_lo >> 32) as u16).wrapping_add((src_lo >> 48) as u16),
            (src_hi as u16).wrapping_add((src_hi >> 16) as u16),
            ((src_hi >> 32) as u16).wrapping_add((src_hi >> 48) as u16),
        ];
        vcpu.regs.xmm[xmm_dst][0] =
            (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
        vcpu.regs.xmm[xmm_dst][1] =
            (d[4] as u64) | ((d[5] as u64) << 16) | ((d[6] as u64) << 32) | ((d[7] as u64) << 48);
    } else {
        // MMX version
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let d = [
            (dst as u16).wrapping_add((dst >> 16) as u16),
            ((dst >> 32) as u16).wrapping_add((dst >> 48) as u16),
            (src as u16).wrapping_add((src >> 16) as u16),
            ((src >> 32) as u16).wrapping_add((src >> 48) as u16),
        ];
        vcpu.regs.mm[mm_dst] =
            (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PHADDD - Packed Horizontal Add Doublewords (0x02)
// =============================================================================

/// PHADDD xmm1, xmm2/m128 (66 0F 38 02) or mm1, mm2/m64 (NP 0F 38 02)
pub fn phaddd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let d = [
            (dst_lo as u32).wrapping_add((dst_lo >> 32) as u32),
            (dst_hi as u32).wrapping_add((dst_hi >> 32) as u32),
            (src_lo as u32).wrapping_add((src_lo >> 32) as u32),
            (src_hi as u32).wrapping_add((src_hi >> 32) as u32),
        ];
        vcpu.regs.xmm[xmm_dst][0] = (d[0] as u64) | ((d[1] as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = (d[2] as u64) | ((d[3] as u64) << 32);
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let d0 = (dst as u32).wrapping_add((dst >> 32) as u32);
        let d1 = (src as u32).wrapping_add((src >> 32) as u32);
        vcpu.regs.mm[mm_dst] = (d0 as u64) | ((d1 as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PHADDSW - Packed Horizontal Add with Saturation (0x03)
// =============================================================================

/// PHADDSW xmm1, xmm2/m128 (66 0F 38 03) or mm1, mm2/m64 (NP 0F 38 03)
pub fn phaddsw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let d = [
            sat_add_i16(dst_lo as i16, (dst_lo >> 16) as i16) as u16,
            sat_add_i16((dst_lo >> 32) as i16, (dst_lo >> 48) as i16) as u16,
            sat_add_i16(dst_hi as i16, (dst_hi >> 16) as i16) as u16,
            sat_add_i16((dst_hi >> 32) as i16, (dst_hi >> 48) as i16) as u16,
            sat_add_i16(src_lo as i16, (src_lo >> 16) as i16) as u16,
            sat_add_i16((src_lo >> 32) as i16, (src_lo >> 48) as i16) as u16,
            sat_add_i16(src_hi as i16, (src_hi >> 16) as i16) as u16,
            sat_add_i16((src_hi >> 32) as i16, (src_hi >> 48) as i16) as u16,
        ];
        vcpu.regs.xmm[xmm_dst][0] =
            (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
        vcpu.regs.xmm[xmm_dst][1] =
            (d[4] as u64) | ((d[5] as u64) << 16) | ((d[6] as u64) << 32) | ((d[7] as u64) << 48);
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let d = [
            sat_add_i16(dst as i16, (dst >> 16) as i16) as u16,
            sat_add_i16((dst >> 32) as i16, (dst >> 48) as i16) as u16,
            sat_add_i16(src as i16, (src >> 16) as i16) as u16,
            sat_add_i16((src >> 32) as i16, (src >> 48) as i16) as u16,
        ];
        vcpu.regs.mm[mm_dst] =
            (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PMADDUBSW - Multiply and Add Packed Signed and Unsigned Bytes (0x04)
// =============================================================================

/// PMADDUBSW xmm1, xmm2/m128 (66 0F 38 04) or mm1, mm2/m64 (NP 0F 38 04)
pub fn pmaddubsw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let dst_bytes = [dst_lo.to_le_bytes(), dst_hi.to_le_bytes()].concat();
        let src_bytes = [src_lo.to_le_bytes(), src_hi.to_le_bytes()].concat();

        let mut result = [0u16; 8];
        for i in 0..8 {
            result[i] = maddubs(
                dst_bytes[i * 2],
                src_bytes[i * 2] as i8,
                dst_bytes[i * 2 + 1],
                src_bytes[i * 2 + 1] as i8,
            ) as u16;
        }
        vcpu.regs.xmm[xmm_dst][0] = (result[0] as u64)
            | ((result[1] as u64) << 16)
            | ((result[2] as u64) << 32)
            | ((result[3] as u64) << 48);
        vcpu.regs.xmm[xmm_dst][1] = (result[4] as u64)
            | ((result[5] as u64) << 16)
            | ((result[6] as u64) << 32)
            | ((result[7] as u64) << 48);
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let dst_bytes = dst.to_le_bytes();
        let src_bytes = src.to_le_bytes();
        let mut result = [0u16; 4];
        for i in 0..4 {
            result[i] = maddubs(
                dst_bytes[i * 2],
                src_bytes[i * 2] as i8,
                dst_bytes[i * 2 + 1],
                src_bytes[i * 2 + 1] as i8,
            ) as u16;
        }
        vcpu.regs.mm[mm_dst] = (result[0] as u64)
            | ((result[1] as u64) << 16)
            | ((result[2] as u64) << 32)
            | ((result[3] as u64) << 48);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PHSUBW - Packed Horizontal Subtract Words (0x05)
// =============================================================================

/// PHSUBW xmm1, xmm2/m128 (66 0F 38 05) or mm1, mm2/m64 (NP 0F 38 05)
pub fn phsubw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let d = [
            (dst_lo as u16).wrapping_sub((dst_lo >> 16) as u16),
            ((dst_lo >> 32) as u16).wrapping_sub((dst_lo >> 48) as u16),
            (dst_hi as u16).wrapping_sub((dst_hi >> 16) as u16),
            ((dst_hi >> 32) as u16).wrapping_sub((dst_hi >> 48) as u16),
            (src_lo as u16).wrapping_sub((src_lo >> 16) as u16),
            ((src_lo >> 32) as u16).wrapping_sub((src_lo >> 48) as u16),
            (src_hi as u16).wrapping_sub((src_hi >> 16) as u16),
            ((src_hi >> 32) as u16).wrapping_sub((src_hi >> 48) as u16),
        ];
        vcpu.regs.xmm[xmm_dst][0] =
            (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
        vcpu.regs.xmm[xmm_dst][1] =
            (d[4] as u64) | ((d[5] as u64) << 16) | ((d[6] as u64) << 32) | ((d[7] as u64) << 48);
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let d = [
            (dst as u16).wrapping_sub((dst >> 16) as u16),
            ((dst >> 32) as u16).wrapping_sub((dst >> 48) as u16),
            (src as u16).wrapping_sub((src >> 16) as u16),
            ((src >> 32) as u16).wrapping_sub((src >> 48) as u16),
        ];
        vcpu.regs.mm[mm_dst] =
            (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PHSUBD - Packed Horizontal Subtract Doublewords (0x06)
// =============================================================================

/// PHSUBD xmm1, xmm2/m128 (66 0F 38 06) or mm1, mm2/m64 (NP 0F 38 06)
pub fn phsubd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let d = [
            (dst_lo as u32).wrapping_sub((dst_lo >> 32) as u32),
            (dst_hi as u32).wrapping_sub((dst_hi >> 32) as u32),
            (src_lo as u32).wrapping_sub((src_lo >> 32) as u32),
            (src_hi as u32).wrapping_sub((src_hi >> 32) as u32),
        ];
        vcpu.regs.xmm[xmm_dst][0] = (d[0] as u64) | ((d[1] as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = (d[2] as u64) | ((d[3] as u64) << 32);
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let d0 = (dst as u32).wrapping_sub((dst >> 32) as u32);
        let d1 = (src as u32).wrapping_sub((src >> 32) as u32);
        vcpu.regs.mm[mm_dst] = (d0 as u64) | ((d1 as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PHSUBSW - Packed Horizontal Subtract with Saturation (0x07)
// =============================================================================

/// PHSUBSW xmm1, xmm2/m128 (66 0F 38 07) or mm1, mm2/m64 (NP 0F 38 07)
pub fn phsubsw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let d = [
            sat_sub_i16(dst_lo as i16, (dst_lo >> 16) as i16) as u16,
            sat_sub_i16((dst_lo >> 32) as i16, (dst_lo >> 48) as i16) as u16,
            sat_sub_i16(dst_hi as i16, (dst_hi >> 16) as i16) as u16,
            sat_sub_i16((dst_hi >> 32) as i16, (dst_hi >> 48) as i16) as u16,
            sat_sub_i16(src_lo as i16, (src_lo >> 16) as i16) as u16,
            sat_sub_i16((src_lo >> 32) as i16, (src_lo >> 48) as i16) as u16,
            sat_sub_i16(src_hi as i16, (src_hi >> 16) as i16) as u16,
            sat_sub_i16((src_hi >> 32) as i16, (src_hi >> 48) as i16) as u16,
        ];
        vcpu.regs.xmm[xmm_dst][0] =
            (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
        vcpu.regs.xmm[xmm_dst][1] =
            (d[4] as u64) | ((d[5] as u64) << 16) | ((d[6] as u64) << 32) | ((d[7] as u64) << 48);
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let d = [
            sat_sub_i16(dst as i16, (dst >> 16) as i16) as u16,
            sat_sub_i16((dst >> 32) as i16, (dst >> 48) as i16) as u16,
            sat_sub_i16(src as i16, (src >> 16) as i16) as u16,
            sat_sub_i16((src >> 32) as i16, (src >> 48) as i16) as u16,
        ];
        vcpu.regs.mm[mm_dst] =
            (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PSIGNB - Packed Sign Bytes (0x08)
// =============================================================================

/// PSIGNB xmm1, xmm2/m128 (66 0F 38 08) or mm1, mm2/m64 (NP 0F 38 08)
pub fn psignb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..8 {
            let d = ((dst_lo >> (i * 8)) & 0xFF) as u8;
            let s = ((src_lo >> (i * 8)) & 0xFF) as i8;
            result_lo |= (sign_byte(d, s) as u64) << (i * 8);
        }
        for i in 0..8 {
            let d = ((dst_hi >> (i * 8)) & 0xFF) as u8;
            let s = ((src_hi >> (i * 8)) & 0xFF) as i8;
            result_hi |= (sign_byte(d, s) as u64) << (i * 8);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let mut result = 0u64;
        for i in 0..8 {
            let d = ((dst >> (i * 8)) & 0xFF) as u8;
            let s = ((src >> (i * 8)) & 0xFF) as i8;
            result |= (sign_byte(d, s) as u64) << (i * 8);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PSIGNW - Packed Sign Words (0x09)
// =============================================================================

/// PSIGNW xmm1, xmm2/m128 (66 0F 38 09) or mm1, mm2/m64 (NP 0F 38 09)
pub fn psignw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..4 {
            let d = ((dst_lo >> (i * 16)) & 0xFFFF) as u16;
            let s = ((src_lo >> (i * 16)) & 0xFFFF) as i16;
            result_lo |= (sign_word(d, s) as u64) << (i * 16);
        }
        for i in 0..4 {
            let d = ((dst_hi >> (i * 16)) & 0xFFFF) as u16;
            let s = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
            result_hi |= (sign_word(d, s) as u64) << (i * 16);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let mut result = 0u64;
        for i in 0..4 {
            let d = ((dst >> (i * 16)) & 0xFFFF) as u16;
            let s = ((src >> (i * 16)) & 0xFFFF) as i16;
            result |= (sign_word(d, s) as u64) << (i * 16);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PSIGND - Packed Sign Doublewords (0x0A)
// =============================================================================

/// PSIGND xmm1, xmm2/m128 (66 0F 38 0A) or mm1, mm2/m64 (NP 0F 38 0A)
pub fn psignd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..2 {
            let d = ((dst_lo >> (i * 32)) & 0xFFFFFFFF) as u32;
            let s = ((src_lo >> (i * 32)) & 0xFFFFFFFF) as i32;
            result_lo |= (sign_dword(d, s) as u64) << (i * 32);
        }
        for i in 0..2 {
            let d = ((dst_hi >> (i * 32)) & 0xFFFFFFFF) as u32;
            let s = ((src_hi >> (i * 32)) & 0xFFFFFFFF) as i32;
            result_hi |= (sign_dword(d, s) as u64) << (i * 32);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let mut result = 0u64;
        for i in 0..2 {
            let d = ((dst >> (i * 32)) & 0xFFFFFFFF) as u32;
            let s = ((src >> (i * 32)) & 0xFFFFFFFF) as i32;
            result |= (sign_dword(d, s) as u64) << (i * 32);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PMULHRSW - Packed Multiply High with Round and Scale (0x0B)
// =============================================================================

/// PMULHRSW xmm1, xmm2/m128 (66 0F 38 0B) or mm1, mm2/m64 (NP 0F 38 0B)
pub fn pmulhrsw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..4 {
            let d = ((dst_lo >> (i * 16)) & 0xFFFF) as i16;
            let s = ((src_lo >> (i * 16)) & 0xFFFF) as i16;
            result_lo |= ((mulhrsw(d, s) as u16) as u64) << (i * 16);
        }
        for i in 0..4 {
            let d = ((dst_hi >> (i * 16)) & 0xFFFF) as i16;
            let s = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
            result_hi |= ((mulhrsw(d, s) as u16) as u64) << (i * 16);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let mut result = 0u64;
        for i in 0..4 {
            let d = ((dst >> (i * 16)) & 0xFFFF) as i16;
            let s = ((src >> (i * 16)) & 0xFFFF) as i16;
            result |= ((mulhrsw(d, s) as u16) as u64) << (i * 16);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PABSB - Packed Absolute Value Bytes (0x1C)
// =============================================================================

/// PABSB xmm1, xmm2/m128 (66 0F 38 1C) or mm1, mm2/m64 (NP 0F 38 1C)
pub fn pabsb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..8 {
            let b = ((src_lo >> (i * 8)) & 0xFF) as i8;
            result_lo |= ((b.wrapping_abs() as u8) as u64) << (i * 8);
        }
        for i in 0..8 {
            let b = ((src_hi >> (i * 8)) & 0xFF) as i8;
            result_hi |= ((b.wrapping_abs() as u8) as u64) << (i * 8);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let mut result = 0u64;
        for i in 0..8 {
            let b = ((src >> (i * 8)) & 0xFF) as i8;
            result |= ((b.wrapping_abs() as u8) as u64) << (i * 8);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PABSW - Packed Absolute Value Words (0x1D)
// =============================================================================

/// PABSW xmm1, xmm2/m128 (66 0F 38 1D) or mm1, mm2/m64 (NP 0F 38 1D)
pub fn pabsw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..4 {
            let w = ((src_lo >> (i * 16)) & 0xFFFF) as i16;
            result_lo |= ((w.wrapping_abs() as u16) as u64) << (i * 16);
        }
        for i in 0..4 {
            let w = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
            result_hi |= ((w.wrapping_abs() as u16) as u64) << (i * 16);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let mut result = 0u64;
        for i in 0..4 {
            let w = ((src >> (i * 16)) & 0xFFFF) as i16;
            result |= ((w.wrapping_abs() as u16) as u64) << (i * 16);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PABSD - Packed Absolute Value Doublewords (0x1E)
// =============================================================================

/// PABSD xmm1, xmm2/m128 (66 0F 38 1E) or mm1, mm2/m64 (NP 0F 38 1E)
pub fn pabsd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..2 {
            let d = ((src_lo >> (i * 32)) & 0xFFFFFFFF) as i32;
            result_lo |= ((d.wrapping_abs() as u32) as u64) << (i * 32);
        }
        for i in 0..2 {
            let d = ((src_hi >> (i * 32)) & 0xFFFFFFFF) as i32;
            result_hi |= ((d.wrapping_abs() as u32) as u64) << (i * 32);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let mut result = 0u64;
        for i in 0..2 {
            let d = ((src >> (i * 32)) & 0xFFFFFFFF) as i32;
            result |= ((d.wrapping_abs() as u32) as u64) << (i * 32);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
