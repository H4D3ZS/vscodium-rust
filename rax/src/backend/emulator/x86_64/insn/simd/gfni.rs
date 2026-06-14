//! GFNI (Galois Field New Instructions): GF2P8MULB, GF2P8AFFINEQB,
//! GF2P8AFFINEINVQB.
//!
//! These operate on packed bytes in GF(2^8) using the AES reduction polynomial
//! x^8 + x^4 + x^3 + x + 1 (0x11B). This module implements the legacy
//! (66-prefixed) SSE 128-bit forms; the destination register is also the first
//! source, and xmm2/m128 supplies the multiplier (MULB) or the 8x8 bit-matrix
//! per qword (AFFINE). Semantics follow the Intel SDM `affine_byte` definition.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// GF(2^8) multiply with the AES reduction polynomial (0x11B).
#[inline]
pub(crate) fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let carry = a & 0x80;
        a <<= 1;
        if carry != 0 {
            a ^= 0x1B; // reduce modulo 0x11B (mod 2^8)
        }
        b >>= 1;
    }
    p
}

/// GF(2^8) multiplicative inverse; inverse(0) := 0. b^254 == b^-1 in GF(2^8).
#[inline]
pub(crate) fn gf_inv(b: u8) -> u8 {
    if b == 0 {
        return 0;
    }
    let mut result: u8 = 1;
    for _ in 0..254 {
        result = gf_mul(result, b);
    }
    result
}

/// GFNI affine transform of one byte: each output bit i is the GF(2) parity of
/// (matrix.byte[7-i] AND input) XOR imm8.bit[i]. `matrix` is one qword (8 rows).
#[inline]
fn affine_byte(matrix: &[u8], input: u8, imm8: u8) -> u8 {
    let mut out: u8 = 0;
    for i in 0..8 {
        let parity = (matrix[7 - i] & input).count_ones() as u8 & 1;
        let bit = parity ^ ((imm8 >> i) & 1);
        out |= bit << i;
    }
    out
}

#[inline]
fn read_src128(vcpu: &mut X86_64Vcpu, rm: u8, is_memory: bool, addr: u64) -> Result<[u8; 16]> {
    let (lo, hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let mut b = [0u8; 16];
    b[0..8].copy_from_slice(&lo.to_le_bytes());
    b[8..16].copy_from_slice(&hi.to_le_bytes());
    Ok(b)
}

#[inline]
fn xmm_bytes(vcpu: &X86_64Vcpu, idx: usize) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[0..8].copy_from_slice(&vcpu.regs.xmm[idx][0].to_le_bytes());
    b[8..16].copy_from_slice(&vcpu.regs.xmm[idx][1].to_le_bytes());
    b
}

#[inline]
fn write_xmm(vcpu: &mut X86_64Vcpu, idx: usize, b: [u8; 16]) {
    vcpu.regs.xmm[idx][0] = u64::from_le_bytes(b[0..8].try_into().unwrap());
    vcpu.regs.xmm[idx][1] = u64::from_le_bytes(b[8..16].try_into().unwrap());
}

/// GF2P8MULB xmm1, xmm2/m128 (66 0F 38 CF /r): per-byte GF(2^8) multiply.
pub fn gf2p8mulb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("GF2P8MULB requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst_idx = reg as usize;
    let src = read_src128(vcpu, rm, is_memory, addr)?;
    let mut dst = xmm_bytes(vcpu, dst_idx);
    for i in 0..16 {
        dst[i] = gf_mul(dst[i], src[i]);
    }
    write_xmm(vcpu, dst_idx, dst);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// GF2P8AFFINEQB xmm1, xmm2/m128, imm8 (66 0F 3A CE /r ib): per-qword affine.
pub fn gf2p8affineqb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator(
            "GF2P8AFFINEQB requires 66 prefix".to_string(),
        ));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let imm8 = ctx.consume_u8()?;
    let dst_idx = reg as usize;
    let matrix = read_src128(vcpu, rm, is_memory, addr)?;
    let mut dst = xmm_bytes(vcpu, dst_idx);
    for q in 0..2 {
        let m = &matrix[q * 8..q * 8 + 8];
        for k in 0..8 {
            let bi = q * 8 + k;
            dst[bi] = affine_byte(m, dst[bi], imm8);
        }
    }
    write_xmm(vcpu, dst_idx, dst);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// GF2P8AFFINEINVQB xmm1, xmm2/m128, imm8 (66 0F 3A CF /r ib): GF inverse then affine.
pub fn gf2p8affineinvqb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator(
            "GF2P8AFFINEINVQB requires 66 prefix".to_string(),
        ));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let imm8 = ctx.consume_u8()?;
    let dst_idx = reg as usize;
    let matrix = read_src128(vcpu, rm, is_memory, addr)?;
    let mut dst = xmm_bytes(vcpu, dst_idx);
    for q in 0..2 {
        let m = &matrix[q * 8..q * 8 + 8];
        for k in 0..8 {
            let bi = q * 8 + k;
            dst[bi] = affine_byte(m, gf_inv(dst[bi]), imm8);
        }
    }
    write_xmm(vcpu, dst_idx, dst);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gf_mul_known_values() {
        // FIPS-197 / AES field examples.
        assert_eq!(gf_mul(0x57, 0x83), 0xC1);
        assert_eq!(gf_mul(0x57, 0x13), 0xFE);
        assert_eq!(gf_mul(0x02, 0x87), 0x15); // xtime(0x87)
        assert_eq!(gf_mul(0, 0xAB), 0);
        assert_eq!(gf_mul(1, 0xAB), 0xAB);
    }

    #[test]
    fn gf_inv_is_inverse() {
        assert_eq!(gf_inv(0), 0);
        for b in 1u16..=255 {
            let b = b as u8;
            assert_eq!(gf_mul(b, gf_inv(b)), 1, "inv of {:#x}", b);
        }
    }

    #[test]
    fn affine_matches_aes_sbox() {
        // Independent reference: the AES S-box affine in its classic rotation form,
        //   s = b ^ rotl(b,1) ^ rotl(b,2) ^ rotl(b,3) ^ rotl(b,4) ^ 0x63,
        // applied to b = inverse(x). GFNI computes the SAME S-box via the matrix
        // form GF2P8AFFINEINVQB(x, 0xF1E3C78F1F3E7CF8, 0x63). They must agree for
        // all 256 inputs — this pins down the affine bit-order (matrix.byte[7-i]).
        fn aes_sbox_ref(x: u8) -> u8 {
            let b = gf_inv(x);
            b ^ b.rotate_left(1) ^ b.rotate_left(2) ^ b.rotate_left(3) ^ b.rotate_left(4) ^ 0x63
        }
        let m = 0xF1E3C78F1F3E7CF8u64.to_le_bytes();
        for x in 0u16..=255 {
            let x = x as u8;
            assert_eq!(
                affine_byte(&m, gf_inv(x), 0x63),
                aes_sbox_ref(x),
                "S-box mismatch at {:#x}",
                x
            );
        }
        // Spot-check canonical entries: S(0)=0x63, S(1)=0x7c, S(0x53)=0xed.
        assert_eq!(affine_byte(&m, gf_inv(0x00), 0x63), 0x63);
        assert_eq!(affine_byte(&m, gf_inv(0x01), 0x63), 0x7c);
        assert_eq!(affine_byte(&m, gf_inv(0x53), 0x63), 0xed);
    }

    #[test]
    fn affine_identity_matrix() {
        // Per GFNI, matrix.byte[7-i] = 1<<i gives identity (with imm8 = 0).
        // byte[k] = 1 << (7 - k).
        let id = [0x80u8, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];
        for x in 0u16..=255 {
            assert_eq!(affine_byte(&id, x as u8, 0), x as u8);
        }
        // imm8 = 0xFF complements the result.
        for x in 0u16..=255 {
            assert_eq!(affine_byte(&id, x as u8, 0xFF), !(x as u8));
        }
    }
}
