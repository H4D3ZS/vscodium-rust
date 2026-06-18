//! x86_64 instruction lifter.
//!
//! This module lifts x86_64 machine code to SMIR. Unlike AArch64 which has a clean
//! decoder, x86 decoding is interleaved with lifting due to variable-length encoding.

use std::collections::HashSet;

use crate::smir::flags::{FlagSet, FlagUpdate};
use crate::smir::ir::{
    CallTarget, CallingConv, FunctionAttrs, SmirBlock, SmirFunction, Terminator, TrapKind,
};
use crate::smir::lift::{
    ControlFlow, LiftContext, LiftError, LiftResult, MemoryReader, SmirLifter,
};
use crate::smir::memory::MemoryError;
use crate::smir::ops::{
    OpKind, SmirOp, X86AluEncoding, X86OpHint, X86SsePrefix, X86VecAlign, X86VecMap,
};
use crate::smir::types::*;

fn x86_rotate_flags() -> FlagUpdate {
    FlagUpdate::Specific(FlagSet::CF.union(FlagSet::OF))
}

fn x86_bextr_flags() -> FlagUpdate {
    FlagUpdate::Specific(FlagSet::CF.union(FlagSet::ZF).union(FlagSet::OF))
}

fn x86_bzhi_flags() -> FlagUpdate {
    FlagUpdate::Specific(
        FlagSet::CF
            .union(FlagSet::ZF)
            .union(FlagSet::SF)
            .union(FlagSet::OF),
    )
}

const APX_CCMP_FLAGS_MASK: i64 = 0x8D5; // CF, PF, AF, ZF, SF, OF

// ============================================================================
// x86_64 Lifter
// ============================================================================

/// x86_64 instruction lifter
pub struct X86_64Lifter {
    /// Whether to use strict mode (fail on unsupported instructions)
    strict: bool,
    /// Lift-through-calls: when set, `lift_function` follows a `CALL`'s
    /// continuation (return address) and keeps lifting the caller's CFG past the
    /// call, instead of ending the function at the call. Used by the JIT's
    /// lift-through-calls path (the call itself lowers to a runtime call-out).
    /// `max_blocks` bounds the lifted CFG so a large/looping function can't lift
    /// unboundedly.
    lift_through_calls: bool,
    /// Cap on lifted blocks (only enforced under `lift_through_calls`).
    max_blocks: usize,
}

impl Default for X86_64Lifter {
    fn default() -> Self {
        Self::new()
    }
}

impl X86_64Lifter {
    /// Create a new x86_64 lifter
    pub fn new() -> Self {
        X86_64Lifter {
            strict: false,
            lift_through_calls: false,
            max_blocks: 0,
        }
    }

    /// Create a lifter in strict mode
    pub fn strict() -> Self {
        X86_64Lifter {
            strict: true,
            lift_through_calls: false,
            max_blocks: 0,
        }
    }

    /// Enable lift-through-calls with a block cap (see the field docs).
    pub fn set_lift_through_calls(&mut self, max_blocks: usize) {
        self.lift_through_calls = true;
        self.max_blocks = max_blocks;
    }
}

// ============================================================================
// Prefix Decoding
// ============================================================================

/// Lookup table for prefix detection
static PREFIX_LUT: [u8; 256] = {
    let mut lut = [0u8; 256];
    // Segment overrides
    lut[0x26] = 1; // ES
    lut[0x2E] = 1; // CS
    lut[0x36] = 1; // SS
    lut[0x3E] = 1; // DS
    lut[0x64] = 1; // FS
    lut[0x65] = 1; // GS
    // Operand/address size
    lut[0x66] = 1;
    lut[0x67] = 1;
    // LOCK, REP
    lut[0xF0] = 1;
    lut[0xF2] = 1;
    lut[0xF3] = 1;
    // REX (0x40-0x4F)
    let mut i = 0x40u8;
    while i <= 0x4F {
        lut[i as usize] = 1;
        i += 1;
    }
    // REX2 (APX)
    lut[0xD5] = 1;
    lut
};

/// Decoded APX REX2 prefix state.
///
/// Payload layout follows LLVM/Intel APX encoding: `M R4 X4 B4 W R3 X3 B3`.
/// The `*_hi` bits add 16 and the `*_lo` bits add 8 to the corresponding
/// ModR/M, SIB, or opcode-register field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rex2Prefix {
    pub m: bool,
    pub w: bool,
    pub r_hi: bool,
    pub x_hi: bool,
    pub b_hi: bool,
    pub r_lo: bool,
    pub x_lo: bool,
    pub b_lo: bool,
}

impl Rex2Prefix {
    #[inline]
    fn r_ext(self) -> u8 {
        (if self.r_hi { 16 } else { 0 }) | (if self.r_lo { 8 } else { 0 })
    }

    #[inline]
    fn x_ext(self) -> u8 {
        (if self.x_hi { 16 } else { 0 }) | (if self.x_lo { 8 } else { 0 })
    }

    #[inline]
    fn b_ext(self) -> u8 {
        (if self.b_hi { 16 } else { 0 }) | (if self.b_lo { 8 } else { 0 })
    }
}

/// Decoded x86 instruction prefix state
#[derive(Clone, Debug, Default)]
pub struct X86Prefix {
    /// REX prefix if present
    pub rex: Option<u8>,
    /// REX2 prefix if present (APX)
    pub rex2: Option<Rex2Prefix>,
    /// Operand size override (0x66)
    pub operand_size_override: bool,
    /// Address size override (0x67)
    pub address_size_override: bool,
    /// REP/REPNE prefix
    pub rep_prefix: Option<u8>,
    /// Segment override
    pub segment_override: Option<u8>,
    /// LOCK prefix
    pub lock: bool,
    /// Cursor position after prefixes
    pub cursor: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VecEncodingKind {
    Vex,
    Evex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AdxKind {
    Adcx,
    Adox,
}

impl AdxKind {
    fn carry_cond(self) -> Condition {
        match self {
            // x86 condition B/C maps to the carry flag.
            AdxKind::Adcx => Condition::Ult,
            AdxKind::Adox => Condition::Overflow,
        }
    }

    fn flag_bit(self) -> u8 {
        match self {
            AdxKind::Adcx => 0,
            AdxKind::Adox => 11,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct VecPrefix {
    encoding: VecEncodingKind,
    map: X86VecMap,
    pp: X86SsePrefix,
    width: VecWidth,
    w: bool,
    vvvv: u8,
    rex: Option<u8>,
    bytes: usize,
}

#[derive(Clone, Copy, Debug)]
struct ApxEvexPrefix {
    bytes: usize,
    r: bool,
    x: bool,
    vvvv: u8,
    r_prime: bool,
    v_prime: bool,
    w: bool,
    pp: u8,
    operand_size_override: bool,
    nd: bool,
    nf: bool,
    aaa: u8,
    b: bool,
    b4: bool,
    x4: bool,
}

impl ApxEvexPrefix {
    fn rm_ext(self) -> u8 {
        let b_ext = if self.b { 0 } else { 8 };
        let b4_ext = if self.b4 { 16 } else { 0 };
        b_ext | b4_ext
    }

    fn reg_ext(self) -> u8 {
        let r_ext = if self.r { 0 } else { 8 };
        let r_prime_ext = if self.r_prime { 0 } else { 16 };
        r_ext | r_prime_ext
    }

    fn index_ext(self) -> u8 {
        let x_ext = if self.x { 0 } else { 8 };
        let x4_ext = if self.x4 { 0 } else { 16 };
        x_ext | x4_ext
    }

    fn vvvv_reg(self) -> u8 {
        let v_prime_ext = if self.v_prime { 0 } else { 16 };
        (self.vvvv ^ 0x0F) | v_prime_ext
    }

    fn op_size(self, is_byte: bool) -> u8 {
        if is_byte {
            1
        } else if self.w {
            8
        } else if self.operand_size_override {
            2
        } else {
            4
        }
    }

    fn flags(self) -> FlagUpdate {
        if self.nf {
            FlagUpdate::None
        } else {
            FlagUpdate::All
        }
    }

    fn ccmp_cond(self) -> u8 {
        ((self.v_prime as u8) << 3) | self.aaa
    }

    fn ccmp_default_flags(self) -> u8 {
        self.vvvv
    }

    fn as_modrm_prefix(self, cursor: usize) -> X86Prefix {
        X86Prefix {
            rex2: Some(Rex2Prefix {
                m: false,
                w: self.w,
                r_hi: (self.reg_ext() & 16) != 0,
                x_hi: (self.index_ext() & 16) != 0,
                b_hi: (self.rm_ext() & 16) != 0,
                r_lo: (self.reg_ext() & 8) != 0,
                x_lo: (self.index_ext() & 8) != 0,
                b_lo: (self.rm_ext() & 8) != 0,
            }),
            cursor,
            operand_size_override: self.operand_size_override,
            ..X86Prefix::default()
        }
    }
}

impl X86Prefix {
    /// Get REX.W flag
    #[inline]
    pub fn rex_w(&self) -> bool {
        self.rex2
            .map_or_else(|| self.rex.map_or(false, |r| r & 0x08 != 0), |r| r.w)
    }

    /// Get REX.R flag (extends ModR/M reg field)
    #[inline]
    pub fn rex_r(&self) -> u8 {
        self.rex2
            .map_or_else(|| self.rex.map_or(0, |r| (r & 0x04) << 1), |r| r.r_ext())
    }

    /// Get REX.X flag (extends SIB index field)
    #[inline]
    pub fn rex_x(&self) -> u8 {
        self.rex2
            .map_or_else(|| self.rex.map_or(0, |r| (r & 0x02) << 2), |r| r.x_ext())
    }

    /// Get REX.B flag (extends ModR/M r/m or opcode reg)
    #[inline]
    pub fn rex_b(&self) -> u8 {
        self.rex2
            .map_or_else(|| self.rex.map_or(0, |r| (r & 0x01) << 3), |r| r.b_ext())
    }

    /// Check if any REX prefix is present
    #[inline]
    pub fn has_rex(&self) -> bool {
        self.rex.is_some() || self.rex2.is_some()
    }

    /// Check if REX2 selects the compressed 0F opcode map.
    #[inline]
    pub fn rex2_m(&self) -> bool {
        self.rex2.map_or(false, |r| r.m)
    }

    /// Compute operand size for 64-bit mode
    #[inline]
    pub fn op_size(&self) -> u8 {
        if self.rex_w() {
            8
        } else if self.operand_size_override {
            2
        } else {
            4
        }
    }

    /// Compute operand width for SMIR
    #[inline]
    pub fn op_width(&self) -> OpWidth {
        match self.op_size() {
            1 => OpWidth::W8,
            2 => OpWidth::W16,
            4 => OpWidth::W32,
            8 => OpWidth::W64,
            _ => OpWidth::W32,
        }
    }
}

/// Decode instruction prefixes
fn decode_prefixes(bytes: &[u8]) -> Result<X86Prefix, LiftError> {
    if bytes.is_empty() {
        return Err(LiftError::Incomplete {
            addr: 0,
            have: 0,
            need: 1,
        });
    }

    let mut prefix = X86Prefix::default();
    let mut cursor = 0;

    while cursor < bytes.len() {
        let b = bytes[cursor];
        if PREFIX_LUT[b as usize] == 0 {
            break;
        }

        match b {
            0x66 => prefix.operand_size_override = true,
            0x67 => prefix.address_size_override = true,
            0x40..=0x4F => prefix.rex = Some(b),
            0xD5 => {
                cursor += 1;
                if cursor >= bytes.len() {
                    return Err(LiftError::Incomplete {
                        addr: 0,
                        have: bytes.len(),
                        need: cursor + 1,
                    });
                }
                let payload = bytes[cursor];
                prefix.rex2 = Some(Rex2Prefix {
                    m: (payload & 0x80) != 0,
                    r_hi: (payload & 0x40) != 0,
                    x_hi: (payload & 0x20) != 0,
                    b_hi: (payload & 0x10) != 0,
                    w: (payload & 0x08) != 0,
                    r_lo: (payload & 0x04) != 0,
                    x_lo: (payload & 0x02) != 0,
                    b_lo: (payload & 0x01) != 0,
                });
                cursor += 1;
                break;
            }
            0xF0 => prefix.lock = true,
            0xF2 | 0xF3 => prefix.rep_prefix = Some(b),
            0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 => {
                prefix.segment_override = Some(b);
            }
            _ => break,
        }
        cursor += 1;
    }

    prefix.cursor = cursor;
    Ok(prefix)
}

fn vex_pp_to_prefix(pp: u8) -> X86SsePrefix {
    match pp & 0x3 {
        0 => X86SsePrefix::None,
        1 => X86SsePrefix::OpSize,
        2 => X86SsePrefix::Rep,
        _ => X86SsePrefix::Repne,
    }
}

fn vec_map_from_bits(map: u8) -> Option<X86VecMap> {
    match map {
        0x01 => Some(X86VecMap::Map0F),
        0x02 => Some(X86VecMap::Map0F38),
        0x03 => Some(X86VecMap::Map0F3A),
        _ => None,
    }
}

fn build_rex(r: u8, x: u8, b: u8, w: bool) -> Option<u8> {
    let mut rex = 0x40;
    if w {
        rex |= 0x08;
    }
    if r != 0 {
        rex |= 0x04;
    }
    if x != 0 {
        rex |= 0x02;
    }
    if b != 0 {
        rex |= 0x01;
    }
    if rex == 0x40 { None } else { Some(rex) }
}

fn decode_vex_prefix(bytes: &[u8], addr: u64) -> Result<VecPrefix, LiftError> {
    if bytes.is_empty() {
        return Err(LiftError::Incomplete {
            addr,
            have: 0,
            need: 1,
        });
    }

    match bytes[0] {
        0xC5 => {
            if bytes.len() < 2 {
                return Err(LiftError::Incomplete {
                    addr,
                    have: bytes.len(),
                    need: 2,
                });
            }
            let b1 = bytes[1];
            let r = ((b1 >> 7) & 1) ^ 1;
            let vvvv = (!b1 >> 3) & 0x0F;
            let l = (b1 >> 2) & 1;
            let pp = vex_pp_to_prefix(b1 & 0x3);

            Ok(VecPrefix {
                encoding: VecEncodingKind::Vex,
                map: X86VecMap::Map0F,
                pp,
                width: if l == 1 {
                    VecWidth::V256
                } else {
                    VecWidth::V128
                },
                w: false,
                vvvv,
                rex: build_rex(r, 0, 0, false),
                bytes: 2,
            })
        }
        0xC4 => {
            if bytes.len() < 3 {
                return Err(LiftError::Incomplete {
                    addr,
                    have: bytes.len(),
                    need: 3,
                });
            }
            let b1 = bytes[1];
            let b2 = bytes[2];
            let r = ((b1 >> 7) & 1) ^ 1;
            let x = ((b1 >> 6) & 1) ^ 1;
            let b = ((b1 >> 5) & 1) ^ 1;
            let map = vec_map_from_bits(b1 & 0x1F).ok_or_else(|| LiftError::Unsupported {
                addr,
                mnemonic: format!("VEX map 0x{:02X}", b1 & 0x1F),
            })?;
            let w = (b2 >> 7) & 1 != 0;
            let vvvv = (!b2 >> 3) & 0x0F;
            let l = (b2 >> 2) & 1;
            let pp = vex_pp_to_prefix(b2 & 0x3);

            Ok(VecPrefix {
                encoding: VecEncodingKind::Vex,
                map,
                pp,
                width: if l == 1 {
                    VecWidth::V256
                } else {
                    VecWidth::V128
                },
                w,
                vvvv,
                rex: build_rex(r, x, b, w),
                bytes: 3,
            })
        }
        _ => Err(LiftError::Unsupported {
            addr,
            mnemonic: "VEX prefix".to_string(),
        }),
    }
}

fn decode_evex_prefix(bytes: &[u8], addr: u64) -> Result<VecPrefix, LiftError> {
    if bytes.len() < 4 {
        return Err(LiftError::Incomplete {
            addr,
            have: bytes.len(),
            need: 4,
        });
    }

    let b1 = bytes[1];
    let b2 = bytes[2];
    let b3 = bytes[3];

    let r = ((b1 >> 4) & 1) ^ 1;
    let x = ((b1 >> 6) & 1) ^ 1;
    let b = ((b1 >> 5) & 1) ^ 1;
    let map_bits = b1 & 0x07;
    let map = vec_map_from_bits(map_bits).ok_or_else(|| LiftError::Unsupported {
        addr,
        mnemonic: format!("EVEX map 0x{map_bits:02X}"),
    })?;

    let w = (b2 >> 7) & 1 != 0;
    let vvvv = (!b2 >> 3) & 0x0F;
    let pp = vex_pp_to_prefix(b2 & 0x3);

    let l_bits = (b3 >> 5) & 0x3;
    let width = match l_bits {
        0 => VecWidth::V128,
        1 => VecWidth::V256,
        2 => VecWidth::V512,
        _ => VecWidth::V512,
    };

    Ok(VecPrefix {
        encoding: VecEncodingKind::Evex,
        map,
        pp,
        width,
        w,
        vvvv,
        rex: build_rex(r, x, b, w),
        bytes: 4,
    })
}

fn decode_apx_evex_prefix(bytes: &[u8], addr: u64) -> Result<ApxEvexPrefix, LiftError> {
    decode_apx_evex_prefix_for_map(bytes, addr, 4)
}

fn decode_apx_evex_prefix_for_map(
    bytes: &[u8],
    addr: u64,
    expected_mm: u8,
) -> Result<ApxEvexPrefix, LiftError> {
    if bytes.len() < 4 {
        return Err(LiftError::Incomplete {
            addr,
            have: bytes.len(),
            need: 4,
        });
    }

    let p0 = bytes[1];
    let p1 = bytes[2];
    let p2 = bytes[3];
    let mm = p0 & 0x07;
    if mm != expected_mm {
        return Err(LiftError::Unsupported {
            addr,
            mnemonic: format!("EVEX map 0x{mm:02X}"),
        });
    }

    Ok(ApxEvexPrefix {
        bytes: 4,
        r: (p0 & 0x80) != 0,
        x: (p0 & 0x40) != 0,
        vvvv: (p1 >> 3) & 0x0F,
        r_prime: (p0 & 0x10) != 0,
        v_prime: (p2 & 0x08) != 0,
        w: (p1 & 0x80) != 0,
        pp: p1 & 0x03,
        operand_size_override: (p1 & 0x03) == 0x01,
        nd: (p2 & 0x10) != 0,
        nf: (p2 & 0x04) != 0,
        aaa: p2 & 0x07,
        b: (p0 & 0x20) != 0,
        b4: (p0 & 0x08) != 0,
        x4: (p1 & 0x04) != 0,
    })
}

// ============================================================================
// ModR/M and SIB Decoding
// ============================================================================

/// Decoded ModR/M result
#[derive(Clone, Debug)]
pub struct ModRm {
    /// ModR/M byte value
    pub byte: u8,
    /// mod field (0-3)
    pub mod_bits: u8,
    /// reg field with REX.R (0-15)
    pub reg: u8,
    /// r/m field with REX.B (0-15)
    pub rm: u8,
    /// Is this a memory operand (mod != 3)?
    pub is_memory: bool,
    /// Decoded memory address (if is_memory)
    pub addr: Option<X86Address>,
    /// Total bytes consumed (including SIB and displacement)
    pub bytes_consumed: usize,
}

/// x86 memory address representation for lifting
#[derive(Clone, Debug)]
pub struct X86Address {
    /// Base register (None for absolute addresses)
    pub base: Option<u8>,
    /// Index register (None if no index)
    pub index: Option<u8>,
    /// Scale (1, 2, 4, or 8)
    pub scale: u8,
    /// Displacement
    pub disp: i64,
    /// RIP-relative addressing?
    pub rip_relative: bool,
    /// Displacement size hint
    pub disp_size: DispSize,
    /// FS/GS segment override, if any (`X86Reg::FsBase` / `X86Reg::GsBase`). In
    /// 64-bit mode CS/DS/ES/SS are flat (base 0) and recorded as `None`.
    pub segment: Option<X86Reg>,
}

/// Decode ModR/M byte and any following SIB/displacement
fn decode_modrm(bytes: &[u8], prefix: &X86Prefix, addr: u64) -> Result<ModRm, LiftError> {
    if bytes.is_empty() {
        return Err(LiftError::Incomplete {
            addr,
            have: 0,
            need: 1,
        });
    }

    let modrm = bytes[0];
    let mod_bits = modrm >> 6;
    let reg_field = (modrm >> 3) & 0x07;
    let rm_field = modrm & 0x07;

    let reg = reg_field | prefix.rex_r();
    let rm = rm_field | prefix.rex_b();

    if mod_bits == 3 {
        // Register operand
        return Ok(ModRm {
            byte: modrm,
            mod_bits,
            reg,
            rm,
            is_memory: false,
            addr: None,
            bytes_consumed: 1,
        });
    }

    // 32-bit address-size override (0x67) in 64-bit mode uses the 32-bit halves
    // of base/index and ZERO-extends the effective address — semantics the
    // lifter does not model (it would treat `[eax]` as `[rax]`, wrong whenever
    // the upper 32 bits are non-zero). Refuse to lift such operands so the
    // region falls back to the interpreter. (Rare in 64-bit kernel code.)
    if prefix.address_size_override {
        return Err(LiftError::Unsupported {
            addr,
            mnemonic: "32-bit address-size (0x67) memory operand".to_string(),
        });
    }

    // FS (0x64) / GS (0x65) overrides carry a non-zero segment base in long mode
    // (TLS / per-CPU data); the lifted memory operand becomes an
    // `Address::SegmentRel` that adds the FsBase/GsBase register. CS/DS/ES/SS
    // overrides are flat/zero-based in long mode and carry no base, so they are
    // left as ordinary addresses (segment = None).
    let segment = match prefix.segment_override {
        Some(0x64) => Some(X86Reg::FsBase),
        Some(0x65) => Some(X86Reg::GsBase),
        _ => None,
    };

    // Memory operand - decode SIB and displacement
    let mut consumed = 1;
    let mut x86_addr = X86Address {
        base: None,
        index: None,
        scale: 1,
        disp: 0,
        rip_relative: false,
        disp_size: DispSize::Auto,
        segment,
    };

    if rm_field == 4 {
        // SIB byte follows
        if bytes.len() < 2 {
            return Err(LiftError::Incomplete {
                addr,
                have: bytes.len(),
                need: 2,
            });
        }
        let sib = bytes[1];
        consumed += 1;

        let scale = 1u8 << (sib >> 6);
        let index_field = (sib >> 3) & 0x07;
        let base_field = sib & 0x07;

        let index = index_field | prefix.rex_x();
        let base = base_field | prefix.rex_b();

        x86_addr.scale = scale;

        // Index = 4 means no index
        if index != 4 {
            x86_addr.index = Some(index);
        }

        // Handle base
        if base_field == 5 && mod_bits == 0 {
            // No base, disp32 follows
            if bytes.len() < consumed + 4 {
                return Err(LiftError::Incomplete {
                    addr,
                    have: bytes.len(),
                    need: consumed + 4,
                });
            }
            let disp = i32::from_le_bytes([
                bytes[consumed],
                bytes[consumed + 1],
                bytes[consumed + 2],
                bytes[consumed + 3],
            ]) as i64;
            consumed += 4;
            x86_addr.disp = disp;
            x86_addr.disp_size = DispSize::Disp32;
        } else {
            x86_addr.base = Some(base);
        }
    } else if rm_field == 5 && mod_bits == 0 {
        // RIP-relative addressing in 64-bit mode
        if bytes.len() < 5 {
            return Err(LiftError::Incomplete {
                addr,
                have: bytes.len(),
                need: 5,
            });
        }
        let disp = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as i64;
        consumed += 4;
        x86_addr.disp = disp;
        x86_addr.rip_relative = true;
        x86_addr.disp_size = DispSize::Disp32;
    } else {
        // Regular register indirect
        x86_addr.base = Some(rm);
    }

    // Handle displacement for mod=1 (disp8) and mod=2 (disp32)
    match mod_bits {
        1 => {
            if bytes.len() < consumed + 1 {
                return Err(LiftError::Incomplete {
                    addr,
                    have: bytes.len(),
                    need: consumed + 1,
                });
            }
            x86_addr.disp = bytes[consumed] as i8 as i64;
            consumed += 1;
            x86_addr.disp_size = DispSize::Disp8;
        }
        2 => {
            if bytes.len() < consumed + 4 {
                return Err(LiftError::Incomplete {
                    addr,
                    have: bytes.len(),
                    need: consumed + 4,
                });
            }
            x86_addr.disp = i32::from_le_bytes([
                bytes[consumed],
                bytes[consumed + 1],
                bytes[consumed + 2],
                bytes[consumed + 3],
            ]) as i64;
            consumed += 4;
            x86_addr.disp_size = DispSize::Disp32;
        }
        _ => {}
    }

    Ok(ModRm {
        byte: modrm,
        mod_bits,
        reg,
        rm,
        is_memory: true,
        addr: Some(x86_addr),
        bytes_consumed: consumed,
    })
}

// ============================================================================
// Register Helpers
// ============================================================================

impl X86_64Lifter {
    /// Convert x86 register number to VReg
    fn x86_gpr(&self, reg: u8) -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::gpr(reg)))
    }

    /// Get x86 register by number
    fn gpr(&self, reg: u8) -> VReg {
        self.x86_gpr(reg & 0x1F)
    }

    fn xmm(&self, reg: u8) -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::Xmm(reg)))
    }

    fn ymm(&self, reg: u8) -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::Ymm(reg)))
    }

    fn zmm(&self, reg: u8) -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::Zmm(reg)))
    }

    fn vec_reg(&self, reg: u8, width: VecWidth) -> VReg {
        match width {
            VecWidth::V128 => self.xmm(reg),
            VecWidth::V256 => self.ymm(reg),
            VecWidth::V512 => self.zmm(reg),
            VecWidth::V64 => self.xmm(reg),
        }
    }

    /// Get RSP register
    fn rsp(&self) -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::Rsp))
    }

    /// Convert op_size to OpWidth
    fn size_to_width(&self, size: u8) -> OpWidth {
        match size {
            1 => OpWidth::W8,
            2 => OpWidth::W16,
            4 => OpWidth::W32,
            8 => OpWidth::W64,
            _ => OpWidth::W32,
        }
    }

    /// Convert op_size to MemWidth
    fn size_to_memwidth(&self, size: u8) -> MemWidth {
        match size {
            1 => MemWidth::B1,
            2 => MemWidth::B2,
            4 => MemWidth::B4,
            8 => MemWidth::B8,
            _ => MemWidth::B4,
        }
    }

    /// Convert x86 address to SMIR Address, optionally generating pre-ops
    fn x86_addr_to_smir(
        &self,
        x86_addr: &X86Address,
        next_rip: u64,
        ctx: &mut LiftContext,
    ) -> (Address, Vec<SmirOp>) {
        let mut pre_ops = Vec::new();
        let pc = ctx.guest_pc;
        let disp_i32 = |disp: i64| -> Option<i32> {
            if disp >= i32::MIN as i64 && disp <= i32::MAX as i64 {
                Some(disp as i32)
            } else {
                None
            }
        };

        // FS/GS segment override → segment-relative address. The effective
        // address is segment_base + base + index*scale + disp. A RIP-relative
        // segment operand folds the (constant) next-RIP into the displacement so
        // `base`/`index` stay true GPRs.
        if let Some(seg) = x86_addr.segment {
            let segment = VReg::Arch(ArchReg::X86(seg));
            let base = x86_addr.base.map(|b| self.gpr(b));
            let index = x86_addr.index.map(|i| self.gpr(i));
            let disp = if x86_addr.rip_relative {
                next_rip as i64 + x86_addr.disp
            } else {
                x86_addr.disp
            };
            return (
                Address::SegmentRel {
                    segment,
                    base,
                    index,
                    scale: x86_addr.scale,
                    disp,
                },
                pre_ops,
            );
        }

        if x86_addr.rip_relative {
            return (
                Address::PcRel {
                    offset: x86_addr.disp,
                    disp_size: x86_addr.disp_size,
                    base: Some(next_rip),
                },
                pre_ops,
            );
        }

        match (x86_addr.base, x86_addr.index) {
            (None, None) => {
                // Absolute address
                (Address::Absolute(x86_addr.disp as u64), pre_ops)
            }
            (Some(base), None) => {
                if x86_addr.disp == 0 && x86_addr.disp_size == DispSize::Auto {
                    (Address::Direct(self.gpr(base)), pre_ops)
                } else {
                    (
                        Address::BaseOffset {
                            base: self.gpr(base),
                            offset: x86_addr.disp,
                            disp_size: x86_addr.disp_size,
                        },
                        pre_ops,
                    )
                }
            }
            (None, Some(index)) => {
                if let Some(disp) = disp_i32(x86_addr.disp) {
                    (
                        Address::BaseIndexScale {
                            base: None,
                            index: self.gpr(index),
                            scale: x86_addr.scale,
                            disp,
                            disp_size: x86_addr.disp_size,
                        },
                        pre_ops,
                    )
                } else {
                    // Fallback to computed address
                    let tmp = ctx.alloc_vreg();
                    if x86_addr.scale > 1 {
                        pre_ops.push(SmirOp::new(
                            OpId(0),
                            pc,
                            OpKind::Shl {
                                dst: tmp,
                                src: self.gpr(index),
                                amount: SrcOperand::Imm(x86_addr.scale.trailing_zeros() as i64),
                                width: OpWidth::W64,
                                flags: FlagUpdate::None,
                            },
                        ));
                        if x86_addr.disp != 0 {
                            let tmp2 = ctx.alloc_vreg();
                            pre_ops.push(SmirOp::new(
                                OpId(1),
                                pc,
                                OpKind::Add {
                                    dst: tmp2,
                                    src1: tmp,
                                    src2: SrcOperand::Imm(x86_addr.disp),
                                    width: OpWidth::W64,
                                    flags: FlagUpdate::None,
                                },
                            ));
                            (Address::Direct(tmp2), pre_ops)
                        } else {
                            (Address::Direct(tmp), pre_ops)
                        }
                    } else if x86_addr.disp != 0 {
                        pre_ops.push(SmirOp::new(
                            OpId(0),
                            pc,
                            OpKind::Add {
                                dst: tmp,
                                src1: self.gpr(index),
                                src2: SrcOperand::Imm(x86_addr.disp),
                                width: OpWidth::W64,
                                flags: FlagUpdate::None,
                            },
                        ));
                        (Address::Direct(tmp), pre_ops)
                    } else {
                        (Address::Direct(self.gpr(index)), pre_ops)
                    }
                }
            }
            (Some(base), Some(index)) => {
                if let Some(disp) = disp_i32(x86_addr.disp) {
                    (
                        Address::BaseIndexScale {
                            base: Some(self.gpr(base)),
                            index: self.gpr(index),
                            scale: x86_addr.scale,
                            disp,
                            disp_size: x86_addr.disp_size,
                        },
                        pre_ops,
                    )
                } else {
                    // Fallback to computed address
                    let tmp_idx = ctx.alloc_vreg();
                    let tmp_sum = ctx.alloc_vreg();

                    // Scale the index
                    if x86_addr.scale > 1 {
                        pre_ops.push(SmirOp::new(
                            OpId(0),
                            pc,
                            OpKind::Shl {
                                dst: tmp_idx,
                                src: self.gpr(index),
                                amount: SrcOperand::Imm(x86_addr.scale.trailing_zeros() as i64),
                                width: OpWidth::W64,
                                flags: FlagUpdate::None,
                            },
                        ));
                        pre_ops.push(SmirOp::new(
                            OpId(1),
                            pc,
                            OpKind::Add {
                                dst: tmp_sum,
                                src1: self.gpr(base),
                                src2: SrcOperand::Reg(tmp_idx),
                                width: OpWidth::W64,
                                flags: FlagUpdate::None,
                            },
                        ));
                    } else {
                        pre_ops.push(SmirOp::new(
                            OpId(0),
                            pc,
                            OpKind::Add {
                                dst: tmp_sum,
                                src1: self.gpr(base),
                                src2: SrcOperand::Reg(self.gpr(index)),
                                width: OpWidth::W64,
                                flags: FlagUpdate::None,
                            },
                        ));
                    }

                    if x86_addr.disp != 0 {
                        (
                            Address::BaseOffset {
                                base: tmp_sum,
                                offset: x86_addr.disp,
                                disp_size: x86_addr.disp_size,
                            },
                            pre_ops,
                        )
                    } else {
                        (Address::Direct(tmp_sum), pre_ops)
                    }
                }
            }
        }
    }

    /// Map x86 condition code (0-15) to SMIR Condition
    fn x86_cond(&self, cc: u8) -> Condition {
        match cc & 0x0F {
            0x0 => Condition::Overflow,   // O
            0x1 => Condition::NoOverflow, // NO
            0x2 => Condition::Ult,        // B/C/NAE
            0x3 => Condition::Uge,        // AE/NB/NC
            0x4 => Condition::Eq,         // E/Z
            0x5 => Condition::Ne,         // NE/NZ
            0x6 => Condition::Ule,        // BE/NA
            0x7 => Condition::Ugt,        // A/NBE
            0x8 => Condition::Negative,   // S
            0x9 => Condition::Positive,   // NS
            0xA => Condition::Parity,     // P/PE
            0xB => Condition::NoParity,   // NP/PO
            0xC => Condition::Slt,        // L/NGE
            0xD => Condition::Sge,        // GE/NL
            0xE => Condition::Sle,        // LE/NG
            0xF => Condition::Sgt,        // G/NLE
            _ => Condition::Always,
        }
    }
}

// ============================================================================
// Instruction Lifting
// ============================================================================

impl X86_64Lifter {
    /// Lift arithmetic instruction (ADD, SUB, ADC, SBC, CMP)
    fn lift_arith(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        // Determine operation type from opcode
        let (is_8bit, dir_rm_reg) = match opcode & 0x07 {
            0 => (true, true),   // rm8, r8
            1 => (false, true),  // rm, r
            2 => (true, false),  // r8, rm8
            3 => (false, false), // r, rm
            4 => (true, true),   // AL, imm8 (handled separately)
            5 => (false, true),  // rAX, imm (handled separately)
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr: pc,
                    bytes: bytes.to_vec(),
                });
            }
        };

        let op_size = if is_8bit { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);

        if (opcode & 0x07) == 4 || (opcode & 0x07) == 5 {
            let imm_size = if is_8bit {
                1
            } else if op_size == 2 {
                2
            } else {
                4
            };
            if bytes.len() < imm_size {
                return Err(LiftError::Incomplete {
                    addr: pc,
                    have: bytes.len(),
                    need: imm_size,
                });
            }

            let imm = match imm_size {
                1 => bytes[0] as i8 as i64,
                2 => i16::from_le_bytes([bytes[0], bytes[1]]) as i64,
                _ => i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64,
            };

            let dst = self.gpr(0);
            let hint = X86OpHint::AluEncoding(X86AluEncoding::AccImm);
            let op_kind = match (opcode >> 3) & 0x07 {
                0 => OpKind::Add {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
                1 => OpKind::Or {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
                2 => OpKind::Adc {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
                3 => OpKind::Sbb {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
                4 => OpKind::And {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
                5 => OpKind::Sub {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
                6 => OpKind::Xor {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
                7 => {
                    let op = SmirOp::with_hint(
                        OpId(0),
                        pc,
                        OpKind::Cmp {
                            src1: dst,
                            src2: SrcOperand::Imm(imm),
                            width,
                        },
                        hint,
                    );
                    return Ok(LiftResult::fallthrough(vec![op], prefix.cursor + imm_size));
                }
                _ => unreachable!(),
            };

            let op = SmirOp::with_hint(OpId(0), pc, op_kind, hint);
            return Ok(LiftResult::fallthrough(vec![op], prefix.cursor + imm_size));
        }

        // Decode ModR/M
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        // Get source and destination
        let (dst, src) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            if dir_rm_reg {
                // rm is destination, reg is source
                let tmp = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Load {
                        dst: tmp,
                        addr: addr.clone(),
                        width: self.size_to_memwidth(op_size),
                        sign: SignExtend::Zero,
                    },
                ));
                (tmp, self.gpr(modrm.reg))
            } else {
                // reg is destination, rm is source
                let tmp = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Load {
                        dst: tmp,
                        addr,
                        width: self.size_to_memwidth(op_size),
                        sign: SignExtend::Zero,
                    },
                ));
                (self.gpr(modrm.reg), tmp)
            }
        } else if dir_rm_reg {
            (self.gpr(modrm.rm), self.gpr(modrm.reg))
        } else {
            (self.gpr(modrm.reg), self.gpr(modrm.rm))
        };

        // Determine operation from opcode high bits
        let result = dst;
        let hint = X86OpHint::AluEncoding(if dir_rm_reg {
            X86AluEncoding::RmReg
        } else {
            X86AluEncoding::RegRm
        });
        let op_kind = match (opcode >> 3) & 0x07 {
            0 => OpKind::Add {
                dst: result,
                src1: dst,
                src2: SrcOperand::Reg(src),
                width,
                flags: FlagUpdate::All,
            },
            1 => OpKind::Or {
                dst: result,
                src1: dst,
                src2: SrcOperand::Reg(src),
                width,
                flags: FlagUpdate::All,
            },
            2 => OpKind::Adc {
                dst: result,
                src1: dst,
                src2: SrcOperand::Reg(src),
                width,
                flags: FlagUpdate::All,
            },
            3 => OpKind::Sbb {
                dst: result,
                src1: dst,
                src2: SrcOperand::Reg(src),
                width,
                flags: FlagUpdate::All,
            },
            4 => OpKind::And {
                dst: result,
                src1: dst,
                src2: SrcOperand::Reg(src),
                width,
                flags: FlagUpdate::All,
            },
            5 => OpKind::Sub {
                dst: result,
                src1: dst,
                src2: SrcOperand::Reg(src),
                width,
                flags: FlagUpdate::All,
            },
            6 => OpKind::Xor {
                dst: result,
                src1: dst,
                src2: SrcOperand::Reg(src),
                width,
                flags: FlagUpdate::All,
            },
            7 => {
                // CMP - subtract but don't store
                ops.push(SmirOp::with_hint(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Cmp {
                        src1: dst,
                        src2: SrcOperand::Reg(src),
                        width,
                    },
                    hint,
                ));
                return Ok(LiftResult::fallthrough(
                    ops,
                    prefix.cursor + modrm.bytes_consumed,
                ));
            }
            _ => unreachable!(),
        };

        ops.push(SmirOp::with_hint(OpId(ops.len() as u16), pc, op_kind, hint));

        // Write back if destination was memory
        if modrm.is_memory && dir_rm_reg {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, _) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Store {
                    src: result,
                    addr,
                    width: self.size_to_memwidth(op_size),
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift Group 1 immediate instructions (80/81/83)
    fn lift_group1_imm(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_8bit = opcode == 0x80;
        let op_size = if is_8bit { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let imm_offset = modrm.bytes_consumed;

        let (imm, imm_size) = match opcode {
            0x80 => {
                if bytes.len() < imm_offset + 1 {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: bytes.len(),
                        need: imm_offset + 1,
                    });
                }
                (bytes[imm_offset] as i8 as i64, 1)
            }
            0x81 => {
                let imm_size = if op_size == 2 { 2 } else { 4 };
                if bytes.len() < imm_offset + imm_size {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: bytes.len(),
                        need: imm_offset + imm_size,
                    });
                }
                let imm = if imm_size == 2 {
                    i16::from_le_bytes([bytes[imm_offset], bytes[imm_offset + 1]]) as i64
                } else {
                    i32::from_le_bytes([
                        bytes[imm_offset],
                        bytes[imm_offset + 1],
                        bytes[imm_offset + 2],
                        bytes[imm_offset + 3],
                    ]) as i64
                };
                (imm, imm_size)
            }
            0x83 => {
                if bytes.len() < imm_offset + 1 {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: bytes.len(),
                        need: imm_offset + 1,
                    });
                }
                (bytes[imm_offset] as i8 as i64, 1)
            }
            _ => unreachable!(),
        };

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();

        let (dst, addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let group = (modrm.byte >> 3) & 0x07;
        match group {
            0 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Add {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
            )),
            1 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Or {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
            )),
            2 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Adc {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
            )),
            3 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Sbb {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
            )),
            4 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::And {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
            )),
            5 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Sub {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
            )),
            6 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Xor {
                    dst,
                    src1: dst,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::All,
                },
            )),
            7 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Cmp {
                        src1: dst,
                        src2: SrcOperand::Imm(imm),
                        width,
                    },
                ));
            }
            _ => {}
        }

        if group != 7 {
            if let Some(addr) = addr {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: dst,
                        addr,
                        width: mem_width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed + imm_size,
        ))
    }

    /// Lift shift instructions with immediate (C0/C1)
    fn lift_shift_imm(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op_size = if opcode == 0xC0 { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        if bytes.len() < modrm.bytes_consumed + 1 {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: modrm.bytes_consumed + 1,
            });
        }

        let imm = bytes[modrm.bytes_consumed] as i64;
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64 + 1;
        let mut ops = Vec::new();

        let group = (modrm.byte >> 3) & 0x07;

        let (src, addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: self.size_to_memwidth(op_size),
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let result = src;
        let op_kind = match group {
            0 => OpKind::Rol {
                dst: result,
                src,
                amount: SrcOperand::Imm(imm),
                width,
                flags: x86_rotate_flags(),
            },
            1 => OpKind::Ror {
                dst: result,
                src,
                amount: SrcOperand::Imm(imm),
                width,
                flags: x86_rotate_flags(),
            },
            2 => OpKind::Rcl {
                dst: result,
                src,
                amount: SrcOperand::Imm(imm),
                width,
                flags: x86_rotate_flags(),
            },
            3 => OpKind::Rcr {
                dst: result,
                src,
                amount: SrcOperand::Imm(imm),
                width,
                flags: x86_rotate_flags(),
            },
            4 | 6 => OpKind::Shl {
                dst: result,
                src,
                amount: SrcOperand::Imm(imm),
                width,
                flags: FlagUpdate::All,
            },
            5 => OpKind::Shr {
                dst: result,
                src,
                amount: SrcOperand::Imm(imm),
                width,
                flags: FlagUpdate::All,
            },
            7 => OpKind::Sar {
                dst: result,
                src,
                amount: SrcOperand::Imm(imm),
                width,
                flags: FlagUpdate::All,
            },
            _ => {
                if self.strict {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("shift group {}", group),
                    });
                }
                return Ok(LiftResult::fallthrough(
                    vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                    prefix.cursor + modrm.bytes_consumed + 1,
                ));
            }
        };

        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));

        if let Some(addr) = addr {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Store {
                    src: result,
                    addr,
                    width: self.size_to_memwidth(op_size),
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed + 1,
        ))
    }

    /// Lift shift instructions with implicit count = 1 (D0/D1)
    fn lift_shift_one(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op_size = if opcode == 0xD0 { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let group = (modrm.byte >> 3) & 0x07;

        let (src, addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: self.size_to_memwidth(op_size),
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let result = src;
        let op_kind = match group {
            0 => OpKind::Rol {
                dst: result,
                src,
                amount: SrcOperand::Imm(1),
                width,
                flags: x86_rotate_flags(),
            },
            1 => OpKind::Ror {
                dst: result,
                src,
                amount: SrcOperand::Imm(1),
                width,
                flags: x86_rotate_flags(),
            },
            2 => OpKind::Rcl {
                dst: result,
                src,
                amount: SrcOperand::Imm(1),
                width,
                flags: x86_rotate_flags(),
            },
            3 => OpKind::Rcr {
                dst: result,
                src,
                amount: SrcOperand::Imm(1),
                width,
                flags: x86_rotate_flags(),
            },
            4 | 6 => OpKind::Shl {
                dst: result,
                src,
                amount: SrcOperand::Imm(1),
                width,
                flags: FlagUpdate::All,
            },
            5 => OpKind::Shr {
                dst: result,
                src,
                amount: SrcOperand::Imm(1),
                width,
                flags: FlagUpdate::All,
            },
            7 => OpKind::Sar {
                dst: result,
                src,
                amount: SrcOperand::Imm(1),
                width,
                flags: FlagUpdate::All,
            },
            _ => {
                if self.strict {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("shift group {}", group),
                    });
                }
                return Ok(LiftResult::fallthrough(
                    vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                    prefix.cursor + modrm.bytes_consumed,
                ));
            }
        };

        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));

        if let Some(addr) = addr {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Store {
                    src: result,
                    addr,
                    width: self.size_to_memwidth(op_size),
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift shift instructions with count in CL (D2/D3)
    fn lift_shift_cl(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op_size = if opcode == 0xD2 { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let group = (modrm.byte >> 3) & 0x07;

        let (src, addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: self.size_to_memwidth(op_size),
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let result = src;
        let amount = SrcOperand::Reg(self.gpr(1));
        let op_kind = match group {
            0 => OpKind::Rol {
                dst: result,
                src,
                amount,
                width,
                flags: x86_rotate_flags(),
            },
            1 => OpKind::Ror {
                dst: result,
                src,
                amount,
                width,
                flags: x86_rotate_flags(),
            },
            2 => OpKind::Rcl {
                dst: result,
                src,
                amount,
                width,
                flags: x86_rotate_flags(),
            },
            3 => OpKind::Rcr {
                dst: result,
                src,
                amount,
                width,
                flags: x86_rotate_flags(),
            },
            4 | 6 => OpKind::Shl {
                dst: result,
                src,
                amount,
                width,
                flags: FlagUpdate::All,
            },
            5 => OpKind::Shr {
                dst: result,
                src,
                amount,
                width,
                flags: FlagUpdate::All,
            },
            7 => OpKind::Sar {
                dst: result,
                src,
                amount,
                width,
                flags: FlagUpdate::All,
            },
            _ => {
                if self.strict {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("shift group {}", group),
                    });
                }
                return Ok(LiftResult::fallthrough(
                    vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                    prefix.cursor + modrm.bytes_consumed,
                ));
            }
        };

        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));

        if let Some(addr) = addr {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Store {
                    src: result,
                    addr,
                    width: self.size_to_memwidth(op_size),
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift BSF/BSR (0F BC/0F BD)
    fn lift_bsf_bsr(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op_size = prefix.op_size();
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        let src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        let op_kind = if opcode == 0xBC {
            OpKind::Bsf {
                dst: self.gpr(modrm.reg),
                src,
                width,
                flags: FlagUpdate::All,
            }
        } else {
            OpKind::Bsr {
                dst: self.gpr(modrm.reg),
                src,
                width,
                flags: FlagUpdate::All,
            }
        };

        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift CMPXCHG r/m, r (0F B0/0F B1).
    fn lift_cmpxchg(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = opcode == 0xB0;
        let op_size = if is_byte { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let acc = self.gpr(0);
        let src = self.gpr(modrm.reg);

        let saved_src = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Mov {
                dst: saved_src,
                src: SrcOperand::Reg(src),
                width,
            },
        ));

        let saved_acc = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Mov {
                dst: saved_acc,
                src: SrcOperand::Reg(acc),
                width,
            },
        ));

        let (old_dst, dst_reg, dst_addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, None, Some(addr))
        } else {
            let dst = self.gpr(modrm.rm);
            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: tmp,
                    src: SrcOperand::Reg(dst),
                    width,
                },
            ));
            (tmp, Some(dst), None)
        };

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Cmp {
                src1: saved_acc,
                src2: SrcOperand::Reg(old_dst),
                width,
            },
        ));

        let matched = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::SetCC {
                dst: matched,
                cond: Condition::Eq,
                width: OpWidth::W8,
            },
        ));

        if let Some(addr) = dst_addr {
            let new_dst = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Select {
                    dst: new_dst,
                    cond: matched,
                    src_true: saved_src,
                    src_false: old_dst,
                    width,
                },
            ));
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::PredStore {
                    src: SrcOperand::Reg(new_dst),
                    cond: matched,
                    addr,
                    width: mem_width,
                },
            ));
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Select {
                    dst: acc,
                    cond: matched,
                    src_true: saved_acc,
                    src_false: old_dst,
                    width,
                },
            ));
        } else if let Some(dst) = dst_reg {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Select {
                    dst,
                    cond: matched,
                    src_true: saved_src,
                    src_false: old_dst,
                    width,
                },
            ));

            if dst != acc {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Select {
                        dst: acc,
                        cond: matched,
                        src_true: saved_acc,
                        src_false: old_dst,
                        width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift XADD r/m, r (0F C0/0F C1).
    fn lift_xadd(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = opcode == 0xC0;
        let op_size = if is_byte { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let src_reg = self.gpr(modrm.reg);
        let mut ops = Vec::new();

        let saved_src = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Mov {
                dst: saved_src,
                src: SrcOperand::Reg(src_reg),
                width,
            },
        ));

        if modrm.is_memory {
            let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let old_dst = ctx.alloc_vreg();
            if prefix.lock {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::AtomicRmw {
                        dst: old_dst,
                        addr: addr.clone(),
                        src: saved_src,
                        op: AtomicOp::Add,
                        width: mem_width,
                        order: MemoryOrder::SeqCst,
                    },
                ));
            } else {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Load {
                        dst: old_dst,
                        addr: addr.clone(),
                        width: mem_width,
                        sign: SignExtend::Zero,
                    },
                ));
            }

            let sum = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Add {
                    dst: sum,
                    src1: old_dst,
                    src2: SrcOperand::Reg(saved_src),
                    width,
                    flags: FlagUpdate::All,
                },
            ));

            if !prefix.lock {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: sum,
                        addr,
                        width: mem_width,
                    },
                ));
            }

            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: src_reg,
                    src: SrcOperand::Reg(old_dst),
                    width,
                },
            ));

            return Ok(LiftResult::fallthrough(
                ops,
                prefix.cursor + modrm.bytes_consumed,
            ));
        }

        if prefix.lock {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes[..modrm.bytes_consumed].to_vec(),
            });
        }

        let dst_reg = self.gpr(modrm.rm);
        let old_dst = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Mov {
                dst: old_dst,
                src: SrcOperand::Reg(dst_reg),
                width,
            },
        ));

        let sum = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Add {
                dst: sum,
                src1: old_dst,
                src2: SrcOperand::Reg(saved_src),
                width,
                flags: FlagUpdate::All,
            },
        ));

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Mov {
                dst: src_reg,
                src: SrcOperand::Reg(old_dst),
                width,
            },
        ));
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Mov {
                dst: dst_reg,
                src: SrcOperand::Reg(sum),
                width,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift XCHG r/m, r (86/87).
    fn lift_xchg_rm_r(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = opcode == 0x86;
        let op_size = if is_byte { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm = decode_modrm(bytes, prefix, pc)?;
        if modrm.is_memory {
            let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, mut ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            let old = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::AtomicRmw {
                    dst: old,
                    addr,
                    src: self.gpr(modrm.reg),
                    op: AtomicOp::Swap,
                    width: mem_width,
                    order: MemoryOrder::SeqCst,
                },
            ));
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: self.gpr(modrm.reg),
                    src: SrcOperand::Reg(old),
                    width,
                },
            ));

            return Ok(LiftResult::fallthrough(
                ops,
                prefix.cursor + modrm.bytes_consumed,
            ));
        }

        Ok(LiftResult::fallthrough(
            vec![SmirOp::new(
                OpId(0),
                pc,
                OpKind::Xchg {
                    reg1: self.gpr(modrm.rm),
                    reg2: self.gpr(modrm.reg),
                    width,
                },
            )],
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift SSE MOVDQA/MOVDQU (0F 6F/7F with prefixes)
    fn lift_sse_mov(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        let prefix_kind = if prefix.rep_prefix == Some(0xF3) {
            X86SsePrefix::Rep
        } else if prefix.rep_prefix == Some(0xF2) {
            X86SsePrefix::Repne
        } else if prefix.operand_size_override {
            X86SsePrefix::OpSize
        } else {
            X86SsePrefix::None
        };

        let hint = X86OpHint::SseMov {
            prefix: prefix_kind,
            opcode,
        };

        match opcode {
            0x6F => {
                if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);
                    ops.push(SmirOp::with_hint(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::VLoad {
                            dst: self.xmm(modrm.reg),
                            addr,
                            width: VecWidth::V128,
                        },
                        hint,
                    ));
                } else {
                    ops.push(SmirOp::with_hint(
                        OpId(0),
                        pc,
                        OpKind::VMov {
                            dst: self.xmm(modrm.reg),
                            src: self.xmm(modrm.rm),
                            width: VecWidth::V128,
                        },
                        hint,
                    ));
                }
            }
            0x7F => {
                if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);
                    ops.push(SmirOp::with_hint(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::VStore {
                            src: self.xmm(modrm.reg),
                            addr,
                            width: VecWidth::V128,
                        },
                        hint,
                    ));
                } else {
                    ops.push(SmirOp::with_hint(
                        OpId(0),
                        pc,
                        OpKind::VMov {
                            dst: self.xmm(modrm.rm),
                            src: self.xmm(modrm.reg),
                            width: VecWidth::V128,
                        },
                        hint,
                    ));
                }
            }
            _ => {}
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift SSE PADDD (66 0F FE)
    fn lift_sse_padd(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        let prefix_kind = if prefix.rep_prefix == Some(0xF3) {
            X86SsePrefix::Rep
        } else if prefix.rep_prefix == Some(0xF2) {
            X86SsePrefix::Repne
        } else if prefix.operand_size_override {
            X86SsePrefix::OpSize
        } else {
            X86SsePrefix::None
        };

        if prefix_kind != X86SsePrefix::OpSize {
            if self.strict {
                return Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: format!("sse opcode 0x{:02X}", opcode),
                });
            }
            return Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                prefix.cursor + modrm.bytes_consumed,
            ));
        }

        let hint = X86OpHint::SseOp {
            prefix: prefix_kind,
            opcode,
        };

        let dst = self.xmm(modrm.reg);
        if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::with_hint(
                OpId(ops.len() as u16),
                pc,
                OpKind::VLoad {
                    dst: tmp,
                    addr,
                    width: VecWidth::V128,
                },
                X86OpHint::VecAlign(X86VecAlign::Unaligned),
            ));
            ops.push(SmirOp::with_hint(
                OpId(ops.len() as u16),
                pc,
                OpKind::VAdd {
                    dst,
                    src1: dst,
                    src2: tmp,
                    elem: VecElementType::I32,
                    lanes: 4,
                },
                hint,
            ));
        } else {
            ops.push(SmirOp::with_hint(
                OpId(0),
                pc,
                OpKind::VAdd {
                    dst,
                    src1: dst,
                    src2: self.xmm(modrm.rm),
                    elem: VecElementType::I32,
                    lanes: 4,
                },
                hint,
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift SSE4.1 PMULLD (66 0F 38 40)
    fn lift_sse_pmulld(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        let prefix_kind = if prefix.rep_prefix == Some(0xF3) {
            X86SsePrefix::Rep
        } else if prefix.rep_prefix == Some(0xF2) {
            X86SsePrefix::Repne
        } else if prefix.operand_size_override {
            X86SsePrefix::OpSize
        } else {
            X86SsePrefix::None
        };

        if prefix_kind != X86SsePrefix::OpSize {
            if self.strict {
                return Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: format!("sse opcode 0x{:02X}", opcode),
                });
            }
            return Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                prefix.cursor + modrm.bytes_consumed,
            ));
        }

        let dst = self.xmm(modrm.reg);
        if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::with_hint(
                OpId(ops.len() as u16),
                pc,
                OpKind::VLoad {
                    dst: tmp,
                    addr,
                    width: VecWidth::V128,
                },
                X86OpHint::VecAlign(X86VecAlign::Unaligned),
            ));
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::VMul {
                    dst,
                    src1: dst,
                    src2: tmp,
                    elem: VecElementType::I32,
                    lanes: 4,
                },
            ));
        } else {
            ops.push(SmirOp::new(
                OpId(0),
                pc,
                OpKind::VMul {
                    dst,
                    src1: dst,
                    src2: self.xmm(modrm.rm),
                    elem: VecElementType::I32,
                    lanes: 4,
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift 0F 38-prefixed (three-byte) opcodes
    fn lift_0f38_opcode(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: prefix.cursor + 1,
                need: prefix.cursor + 2,
            });
        }

        let opcode3 = bytes[0];
        let after_opcode = &bytes[1..];
        let prefix3 = X86Prefix {
            cursor: prefix.cursor + 1,
            ..prefix.clone()
        };

        match opcode3 {
            0x40 => self.lift_sse_pmulld(opcode3, after_opcode, &prefix3, pc, ctx),
            0x8A | 0x8B => self.lift_movrs_0f38(opcode3, after_opcode, &prefix3, pc, ctx),
            0xF6 => self.lift_adx_0f38(after_opcode, &prefix3, pc, ctx),
            0xF0 | 0xF1 => self.lift_movbe_0f38(opcode3, after_opcode, &prefix3, pc, ctx),
            0xFC => self.lift_rao_int_0f38(after_opcode, &prefix3, pc, ctx),
            _ => {
                if self.strict {
                    Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("0x0F 0x38 0x{:02X}", opcode3),
                    })
                } else {
                    Ok(LiftResult::fallthrough(
                        vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                        prefix3.cursor,
                    ))
                }
            }
        }
    }

    fn lift_movbe_0f38(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.rep_prefix == Some(0xF2) {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "CRC32".to_string(),
            });
        }
        if prefix.lock || prefix.rep_prefix == Some(0xF3) {
            let mut err_bytes = vec![opcode];
            err_bytes.extend_from_slice(bytes);
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: err_bytes,
            });
        }

        let op_size = prefix.op_size();
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm = decode_modrm(bytes, prefix, pc)?;
        if !modrm.is_memory {
            let mut err_bytes = vec![opcode];
            err_bytes.extend_from_slice(bytes);
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: err_bytes,
            });
        }

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let x86_addr = modrm.addr.as_ref().unwrap();
        let (addr, mut ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);

        match opcode {
            0xF0 => {
                let tmp = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Load {
                        dst: tmp,
                        addr,
                        width: mem_width,
                        sign: SignExtend::Zero,
                    },
                ));
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Bswap {
                        dst: self.gpr(modrm.reg),
                        src: tmp,
                        width,
                    },
                ));
            }
            0xF1 => {
                let tmp = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Bswap {
                        dst: tmp,
                        src: self.gpr(modrm.reg),
                        width,
                    },
                ));
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: tmp,
                        addr,
                        width: mem_width,
                    },
                ));
            }
            _ => unreachable!("MOVBE is only dispatched for opcodes F0/F1"),
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    fn lift_adx_0f38(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let kind = match prefix.rep_prefix {
            Some(0xF3) => AdxKind::Adox,
            Some(_) => {
                return Err(LiftError::InvalidEncoding {
                    addr: pc,
                    bytes: bytes.to_vec(),
                });
            }
            None if prefix.operand_size_override => AdxKind::Adcx,
            None => {
                return Err(LiftError::InvalidEncoding {
                    addr: pc,
                    bytes: bytes.to_vec(),
                });
            }
        };

        let width = if prefix.rex_w() {
            OpWidth::W64
        } else {
            OpWidth::W32
        };
        self.lift_adx_modrm(bytes, prefix, pc, ctx, kind, width, None)
    }

    fn lift_adx_modrm(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
        kind: AdxKind,
        width: OpWidth,
        dst_override: Option<VReg>,
    ) -> Result<LiftResult, LiftError> {
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let mem_width = match width {
            OpWidth::W32 => MemWidth::B4,
            OpWidth::W64 => MemWidth::B8,
            _ => unreachable!("ADX is only defined for 32- and 64-bit operands"),
        };
        let mut ops = Vec::new();

        let src1 = self.gpr(modrm.reg);
        let src2 = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };
        let dst = dst_override.unwrap_or(src1);
        self.push_adx_ops(&mut ops, pc, ctx, dst, src1, src2, width, kind);

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    fn push_adx_ops(
        &self,
        ops: &mut Vec<SmirOp>,
        pc: u64,
        ctx: &mut LiftContext,
        dst: VReg,
        src1: VReg,
        src2: VReg,
        width: OpWidth,
        kind: AdxKind,
    ) {
        let old_flags = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::ReadFlags { dst: old_flags },
        ));

        let carry_in = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::SetCC {
                dst: carry_in,
                cond: kind.carry_cond(),
                width: OpWidth::W64,
            },
        ));

        let lhs = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Mov {
                dst: lhs,
                src: SrcOperand::Reg(src1),
                width,
            },
        ));

        let sum = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Add {
                dst: sum,
                src1: lhs,
                src2: SrcOperand::Reg(src2),
                width,
                flags: FlagUpdate::None,
            },
        ));
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Add {
                dst,
                src1: sum,
                src2: SrcOperand::Reg(carry_in),
                width,
                flags: FlagUpdate::None,
            },
        ));

        let carry_from_add = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Cmp {
                src1: sum,
                src2: SrcOperand::Reg(lhs),
                width,
            },
        ));
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::SetCC {
                dst: carry_from_add,
                cond: Condition::Ult,
                width: OpWidth::W64,
            },
        ));

        let carry_from_cin = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Cmp {
                src1: dst,
                src2: SrcOperand::Reg(sum),
                width,
            },
        ));
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::SetCC {
                dst: carry_from_cin,
                cond: Condition::Ult,
                width: OpWidth::W64,
            },
        ));

        let carry_out = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Or {
                dst: carry_out,
                src1: carry_from_add,
                src2: SrcOperand::Reg(carry_from_cin),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        let flag_value = if kind.flag_bit() == 0 {
            carry_out
        } else {
            let shifted = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Shl {
                    dst: shifted,
                    src: carry_out,
                    amount: SrcOperand::Imm(kind.flag_bit() as i64),
                    width: OpWidth::W64,
                    flags: FlagUpdate::None,
                },
            ));
            shifted
        };

        let flag_mask = 1i64 << kind.flag_bit();
        let cleared_flags = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::And {
                dst: cleared_flags,
                src1: old_flags,
                src2: SrcOperand::Imm(!flag_mask),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        let new_flags = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Or {
                dst: new_flags,
                src1: cleared_flags,
                src2: SrcOperand::Reg(flag_value),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::WriteFlags { src: new_flags },
        ));
    }

    fn lift_rao_int_0f38(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op = match prefix.rep_prefix {
            Some(0xF2) => AtomicOp::Or,
            Some(0xF3) => AtomicOp::Xor,
            Some(_) => {
                return Err(LiftError::InvalidEncoding {
                    addr: pc,
                    bytes: bytes.to_vec(),
                });
            }
            None if prefix.operand_size_override => AtomicOp::And,
            None => AtomicOp::Add,
        };

        let width = if prefix.rex_w() {
            MemWidth::B8
        } else {
            MemWidth::B4
        };
        self.lift_rao_int_modrm(bytes, prefix, pc, ctx, op, width)
    }

    fn lift_rao_int_modrm(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
        op: AtomicOp,
        width: MemWidth,
    ) -> Result<LiftResult, LiftError> {
        let modrm = decode_modrm(bytes, prefix, pc)?;
        if !modrm.is_memory {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let x86_addr = modrm.addr.as_ref().unwrap();
        let (addr, mut ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
        let old = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::AtomicRmw {
                dst: old,
                addr,
                src: self.gpr(modrm.reg),
                op,
                width,
                order: MemoryOrder::SeqCst,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    fn lift_movrs_0f38(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = opcode == 0x8A;
        let op_size = if is_byte { 1 } else { prefix.op_size() };
        let mem_width = self.size_to_memwidth(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        if !modrm.is_memory {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let x86_addr = modrm.addr.as_ref().unwrap();
        let (addr, mut ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Load {
                dst: self.gpr(modrm.reg),
                addr,
                width: mem_width,
                sign: SignExtend::Zero,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    fn vec_hint(&self, prefix: VecPrefix, opcode: u8) -> X86OpHint {
        match prefix.encoding {
            VecEncodingKind::Vex => X86OpHint::VexOp {
                map: prefix.map,
                pp: prefix.pp,
                opcode,
                width: prefix.width,
            },
            VecEncodingKind::Evex => X86OpHint::EvexOp {
                map: prefix.map,
                pp: prefix.pp,
                opcode,
                width: prefix.width,
            },
        }
    }

    fn lift_apx_nf_bmi_0f38(
        &self,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let prefix = decode_apx_evex_prefix_for_map(bytes, pc, 2)?;
        if prefix.pp != 0 || !prefix.nf || prefix.nd || prefix.aaa != 4 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX NF BMI 0F38 form".to_string(),
            });
        }

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let rm_src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        let copy_if_dst_aliases =
            |ops: &mut Vec<SmirOp>, ctx: &mut LiftContext, dst: VReg, src: VReg| -> VReg {
                if dst != src {
                    return src;
                }
                let tmp = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Mov {
                        dst: tmp,
                        src: SrcOperand::Reg(src),
                        width,
                    },
                ));
                tmp
            };

        match opcode {
            0xF2 => {
                let dst = self.gpr(modrm.reg);
                let src1 = self.gpr(prefix.vvvv_reg());
                let src2 = copy_if_dst_aliases(&mut ops, ctx, dst, rm_src);
                let inverted = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Not {
                        dst: inverted,
                        src: src1,
                        width,
                    },
                ));
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::And {
                        dst,
                        src1: inverted,
                        src2: SrcOperand::Reg(src2),
                        width,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            0xF3 => {
                let group = (modrm.byte >> 3) & 0x07;
                if !matches!(group, 1..=3) {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("APX NF BMI F3 /{group}"),
                    });
                }

                let dst = self.gpr(prefix.vvvv_reg());
                let src = copy_if_dst_aliases(&mut ops, ctx, dst, rm_src);
                match group {
                    1 => {
                        let minus_one = ctx.alloc_vreg();
                        ops.push(SmirOp::new(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::Sub {
                                dst: minus_one,
                                src1: src,
                                src2: SrcOperand::Imm(1),
                                width,
                                flags: FlagUpdate::None,
                            },
                        ));
                        ops.push(SmirOp::new(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::And {
                                dst,
                                src1: src,
                                src2: SrcOperand::Reg(minus_one),
                                width,
                                flags: FlagUpdate::None,
                            },
                        ));
                    }
                    2 => {
                        let minus_one = ctx.alloc_vreg();
                        ops.push(SmirOp::new(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::Sub {
                                dst: minus_one,
                                src1: src,
                                src2: SrcOperand::Imm(1),
                                width,
                                flags: FlagUpdate::None,
                            },
                        ));
                        ops.push(SmirOp::new(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::Xor {
                                dst,
                                src1: src,
                                src2: SrcOperand::Reg(minus_one),
                                width,
                                flags: FlagUpdate::None,
                            },
                        ));
                    }
                    3 => {
                        let negated = ctx.alloc_vreg();
                        ops.push(SmirOp::new(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::Neg {
                                dst: negated,
                                src,
                                width,
                                flags: FlagUpdate::None,
                            },
                        ));
                        ops.push(SmirOp::new(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::And {
                                dst,
                                src1: negated,
                                src2: SrcOperand::Reg(src),
                                width,
                                flags: FlagUpdate::None,
                            },
                        ));
                    }
                    _ => unreachable!(),
                }
            }
            0xF5 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Bzhi {
                        dst: self.gpr(modrm.reg),
                        src: rm_src,
                        index: self.gpr(prefix.vvvv_reg()),
                        width,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            0xF7 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Bextr {
                        dst: self.gpr(modrm.reg),
                        src: rm_src,
                        control: self.gpr(prefix.vvvv_reg()),
                        width,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            _ => {
                return Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: format!("APX NF BMI opcode 0x{opcode:02X}"),
                });
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_vex_andn_0f38(
        &self,
        prefix: VecPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.encoding != VecEncodingKind::Vex
            || prefix.width != VecWidth::V128
            || prefix.pp != X86SsePrefix::None
        {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = X86Prefix {
            rex: prefix.rex,
            cursor: prefix.bytes + 1,
            ..X86Prefix::default()
        };
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();
        let src2 = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::AndNot {
                dst: self.gpr(modrm.reg),
                src1: src2,
                src2: SrcOperand::Reg(self.gpr(prefix.vvvv)),
                width,
                flags: FlagUpdate::All,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_vex_bzhi_bextr_0f38(
        &self,
        prefix: VecPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.encoding != VecEncodingKind::Vex
            || prefix.width != VecWidth::V128
            || prefix.pp != X86SsePrefix::None
        {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = X86Prefix {
            rex: prefix.rex,
            cursor: prefix.bytes + 1,
            ..X86Prefix::default()
        };
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();
        let src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        let dst = self.gpr(modrm.reg);
        let control = self.gpr(prefix.vvvv);
        let kind = match opcode {
            0xF5 => OpKind::Bzhi {
                dst,
                src,
                index: control,
                width,
                flags: x86_bzhi_flags(),
            },
            0xF7 => OpKind::Bextr {
                dst,
                src,
                control,
                width,
                flags: x86_bextr_flags(),
            },
            _ => unreachable!("VEX BZHI/BEXTR only dispatches F5/F7"),
        };
        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, kind));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn validate_apx_bmi_payload(
        &self,
        prefix: ApxEvexPrefix,
        bytes: &[u8],
        pc: u64,
        allow_nf: bool,
    ) -> Result<(), LiftError> {
        let allowed_p2 = 0x08 | if allow_nf { 0x04 } else { 0 };
        if bytes[3] & !allowed_p2 != 0 || (!allow_nf && prefix.nf) {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX BMI reserved EVEX field".to_string(),
            });
        }
        Ok(())
    }

    fn lift_apx_bmi2_0f38(
        &self,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let prefix = decode_apx_evex_prefix_for_map(bytes, pc, 2)?;
        self.validate_apx_bmi_payload(prefix, bytes, pc, false)?;

        let kind = match (opcode, prefix.pp) {
            (0xF5, 0x03) => "pdep",
            (0xF5, 0x02) => "pext",
            (0xF6, 0x03) => "mulx",
            (0xF7, 0x02) => "sarx",
            (0xF7, 0x03) => "shrx",
            (0xF7, 0x01) => "shlx",
            _ => {
                return Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: format!("APX BMI2 opcode 0x{opcode:02X} pp {}", prefix.pp),
                });
            }
        };

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let rm_src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        match kind {
            "pdep" => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Pdep {
                    dst: self.gpr(modrm.reg),
                    src: self.gpr(prefix.vvvv_reg()),
                    mask: rm_src,
                    width,
                },
            )),
            "pext" => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Pext {
                    dst: self.gpr(modrm.reg),
                    src: self.gpr(prefix.vvvv_reg()),
                    mask: rm_src,
                    width,
                },
            )),
            "mulx" => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::MulU {
                    dst_lo: self.gpr(prefix.vvvv_reg()),
                    dst_hi: Some(self.gpr(modrm.reg)),
                    src1: self.gpr(2),
                    src2: SrcOperand::Reg(rm_src),
                    width,
                    flags: FlagUpdate::None,
                },
            )),
            "sarx" | "shrx" | "shlx" => {
                let count = self.gpr(prefix.vvvv_reg());
                let masked_count = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::And {
                        dst: masked_count,
                        src1: count,
                        src2: SrcOperand::Imm((width.bits() - 1) as i64),
                        width,
                        flags: FlagUpdate::None,
                    },
                ));
                let amount = SrcOperand::Reg(masked_count);
                let dst = self.gpr(modrm.reg);
                let op_kind = match kind {
                    "sarx" => OpKind::Sar {
                        dst,
                        src: rm_src,
                        amount,
                        width,
                        flags: FlagUpdate::None,
                    },
                    "shrx" => OpKind::Shr {
                        dst,
                        src: rm_src,
                        amount,
                        width,
                        flags: FlagUpdate::None,
                    },
                    "shlx" => OpKind::Shl {
                        dst,
                        src: rm_src,
                        amount,
                        width,
                        flags: FlagUpdate::None,
                    },
                    _ => unreachable!(),
                };
                ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));
            }
            _ => unreachable!(),
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_vex_pdep_pext_0f38(
        &self,
        prefix: VecPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.encoding != VecEncodingKind::Vex
            || prefix.width != VecWidth::V128
            || !matches!(prefix.pp, X86SsePrefix::Rep | X86SsePrefix::Repne)
        {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = X86Prefix {
            rex: prefix.rex,
            rep_prefix: match prefix.pp {
                X86SsePrefix::Rep => Some(0xF3),
                X86SsePrefix::Repne => Some(0xF2),
                _ => None,
            },
            cursor: prefix.bytes + 1,
            ..X86Prefix::default()
        };
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();
        let mask = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        let dst = self.gpr(modrm.reg);
        let src = self.gpr(prefix.vvvv);
        let op = match prefix.pp {
            X86SsePrefix::Rep => OpKind::Pext {
                dst,
                src,
                mask,
                width,
            },
            X86SsePrefix::Repne => OpKind::Pdep {
                dst,
                src,
                mask,
                width,
            },
            _ => unreachable!("PDEP/PEXT are only dispatched for F2/F3 VEX prefixes"),
        };
        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_vex_mulx_0f38(
        &self,
        prefix: VecPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.encoding != VecEncodingKind::Vex
            || prefix.width != VecWidth::V128
            || prefix.pp != X86SsePrefix::Repne
        {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = X86Prefix {
            rex: prefix.rex,
            rep_prefix: Some(0xF2),
            cursor: prefix.bytes + 1,
            ..X86Prefix::default()
        };
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();
        let src2 = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::MulU {
                dst_lo: self.gpr(prefix.vvvv),
                dst_hi: Some(self.gpr(modrm.reg)),
                src1: self.gpr(2),
                src2: SrcOperand::Reg(src2),
                width,
                flags: FlagUpdate::None,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_vex_bmi2_shift_0f38(
        &self,
        prefix: VecPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.encoding != VecEncodingKind::Vex
            || prefix.width != VecWidth::V128
            || !matches!(
                prefix.pp,
                X86SsePrefix::OpSize | X86SsePrefix::Rep | X86SsePrefix::Repne
            )
        {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = X86Prefix {
            rex: prefix.rex,
            rep_prefix: match prefix.pp {
                X86SsePrefix::OpSize => Some(0x66),
                X86SsePrefix::Rep => Some(0xF3),
                X86SsePrefix::Repne => Some(0xF2),
                _ => None,
            },
            cursor: prefix.bytes + 1,
            ..X86Prefix::default()
        };
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();
        let src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        let dst = self.gpr(modrm.reg);
        let count = self.gpr(prefix.vvvv);
        let masked_count = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::And {
                dst: masked_count,
                src1: count,
                src2: SrcOperand::Imm((width.bits() - 1) as i64),
                width,
                flags: FlagUpdate::None,
            },
        ));

        let amount = SrcOperand::Reg(masked_count);
        let op = match prefix.pp {
            X86SsePrefix::Rep => OpKind::Sar {
                dst,
                src,
                amount,
                width,
                flags: FlagUpdate::None,
            },
            X86SsePrefix::Repne => OpKind::Shr {
                dst,
                src,
                amount,
                width,
                flags: FlagUpdate::None,
            },
            X86SsePrefix::OpSize => OpKind::Shl {
                dst,
                src,
                amount,
                width,
                flags: FlagUpdate::None,
            },
            _ => unreachable!("BMI2 VEX shifts require 66/F2/F3 prefix encodings"),
        };
        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_vex_bmi2_rorx_0f3a(
        &self,
        prefix: VecPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.encoding != VecEncodingKind::Vex
            || prefix.width != VecWidth::V128
            || prefix.pp != X86SsePrefix::Repne
            || prefix.vvvv != 0
        {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = X86Prefix {
            rex: prefix.rex,
            rep_prefix: Some(0xF2),
            cursor: prefix.bytes + 1,
            ..X86Prefix::default()
        };
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let imm_offset = prefix.bytes + 1 + modrm.bytes_consumed;
        if bytes.len() <= imm_offset {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: imm_offset + 1,
            });
        }

        let next_pc = pc + imm_offset as u64 + 1;
        let mut ops = Vec::new();
        let src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Ror {
                dst: self.gpr(modrm.reg),
                src,
                amount: SrcOperand::Imm(bytes[imm_offset] as i64),
                width,
                flags: FlagUpdate::None,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed + 1,
        ))
    }

    fn lift_apx_bmi2_rorx(
        &self,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let prefix = decode_apx_evex_prefix_for_map(bytes, pc, 3)?;
        self.validate_apx_bmi_payload(prefix, bytes, pc, false)?;
        if prefix.pp != 0x03 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX RORX pp {}", prefix.pp),
            });
        }

        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let mem_width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(&bytes[prefix.bytes + 1..], &modrm_prefix, pc)?;
        let imm_offset = prefix.bytes + 1 + modrm.bytes_consumed;
        if bytes.len() <= imm_offset {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: imm_offset + 1,
            });
        }

        let next_pc = pc + imm_offset as u64 + 1;
        let mut ops = Vec::new();
        let src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Ror {
                dst: self.gpr(modrm.reg),
                src,
                amount: SrcOperand::Imm(bytes[imm_offset] as i64),
                width,
                flags: FlagUpdate::None,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed + 1,
        ))
    }

    fn lift_cmpccxadd(
        &self,
        prefix: VecPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.pp != X86SsePrefix::OpSize || prefix.width != VecWidth::V128 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "CMPccXADD reserved vector prefix".to_string(),
            });
        }

        let (modrm_prefix, add, width, prefix_bytes) = match prefix.encoding {
            VecEncodingKind::Vex => {
                let width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
                (
                    X86Prefix {
                        rex: prefix.rex,
                        operand_size_override: true,
                        cursor: prefix.bytes + 1,
                        ..X86Prefix::default()
                    },
                    self.gpr(prefix.vvvv),
                    width,
                    prefix.bytes,
                )
            }
            VecEncodingKind::Evex => {
                let apx = decode_apx_evex_prefix_for_map(bytes, pc, 2)?;
                if apx.pp != 1 || apx.aaa != 0 || apx.nf || apx.nd || (bytes[3] & 0xE0) != 0 {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: "EVEX CMPccXADD reserved field".to_string(),
                    });
                }
                let width = if apx.w { MemWidth::B8 } else { MemWidth::B4 };
                (
                    apx.as_modrm_prefix(apx.bytes + 1),
                    self.gpr(apx.vvvv_reg()),
                    width,
                    apx.bytes,
                )
            }
        };

        let modrm = decode_modrm(&bytes[prefix_bytes + 1..], &modrm_prefix, pc)?;
        if !modrm.is_memory {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let next_pc = pc + prefix_bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let x86_addr = modrm.addr.as_ref().unwrap();
        let (addr, mut ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
        let cmp = self.gpr(modrm.reg);
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::AtomicCmpXadd {
                dst_old: cmp,
                addr,
                cmp,
                add,
                cond: self.x86_cond(opcode & 0x0F),
                width,
                order: MemoryOrder::SeqCst,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix_bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_vec_opcode(
        &self,
        prefix: VecPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.len() < prefix.bytes + 1 {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: prefix.bytes + 1,
            });
        }

        let opcode = bytes[prefix.bytes];
        let after_opcode = &bytes[prefix.bytes + 1..];
        let cursor = prefix.bytes + 1;
        let prefix_modrm = X86Prefix {
            rex: prefix.rex,
            operand_size_override: matches!(prefix.pp, X86SsePrefix::OpSize),
            rep_prefix: match prefix.pp {
                X86SsePrefix::Rep => Some(0xF3),
                X86SsePrefix::Repne => Some(0xF2),
                _ => None,
            },
            cursor,
            ..X86Prefix::default()
        };

        let mut ops = Vec::new();
        let hint = self.vec_hint(prefix, opcode);

        match prefix.map {
            X86VecMap::Map0F => match opcode {
                // VMOVAPS (0F 28/29) and VMOVDQA (0F 6F/7F with 66)
                0x28 | 0x29 | 0x6F | 0x7F => {
                    let modrm = decode_modrm(after_opcode, &prefix_modrm, pc)?;
                    let next_pc = pc + cursor as u64 + modrm.bytes_consumed as u64;
                    let dst_reg = self.vec_reg(modrm.reg, prefix.width);
                    let rm_reg = self.vec_reg(modrm.rm, prefix.width);

                    match opcode {
                        0x28 | 0x6F => {
                            if modrm.is_memory {
                                let x86_addr = modrm.addr.as_ref().unwrap();
                                let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                                ops.extend(pre_ops);
                                ops.push(SmirOp::with_hint(
                                    OpId(ops.len() as u16),
                                    pc,
                                    OpKind::VLoad {
                                        dst: dst_reg,
                                        addr,
                                        width: prefix.width,
                                    },
                                    hint,
                                ));
                            } else {
                                ops.push(SmirOp::with_hint(
                                    OpId(0),
                                    pc,
                                    OpKind::VMov {
                                        dst: dst_reg,
                                        src: rm_reg,
                                        width: prefix.width,
                                    },
                                    hint,
                                ));
                            }
                        }
                        0x29 | 0x7F => {
                            if modrm.is_memory {
                                let x86_addr = modrm.addr.as_ref().unwrap();
                                let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                                ops.extend(pre_ops);
                                ops.push(SmirOp::with_hint(
                                    OpId(ops.len() as u16),
                                    pc,
                                    OpKind::VStore {
                                        src: dst_reg,
                                        addr,
                                        width: prefix.width,
                                    },
                                    hint,
                                ));
                            } else {
                                ops.push(SmirOp::with_hint(
                                    OpId(0),
                                    pc,
                                    OpKind::VMov {
                                        dst: rm_reg,
                                        src: dst_reg,
                                        width: prefix.width,
                                    },
                                    hint,
                                ));
                            }
                        }
                        _ => {}
                    }

                    Ok(LiftResult::fallthrough(ops, cursor + modrm.bytes_consumed))
                }

                // VADDPS/VADDPS/VMULPS/VMAXPS
                0x58 | 0x59 | 0x5C | 0x5F | 0xFE => {
                    let modrm = decode_modrm(after_opcode, &prefix_modrm, pc)?;
                    let next_pc = pc + cursor as u64 + modrm.bytes_consumed as u64;
                    let dst = self.vec_reg(modrm.reg, prefix.width);
                    let src1 = self.vec_reg(prefix.vvvv, prefix.width);

                    let (elem, lanes) = if opcode == 0xFE {
                        (
                            VecElementType::I32,
                            prefix.width.lanes(VecElementType::I32) as u8,
                        )
                    } else {
                        (
                            VecElementType::F32,
                            prefix.width.lanes(VecElementType::F32) as u8,
                        )
                    };

                    let src2 = if modrm.is_memory {
                        let x86_addr = modrm.addr.as_ref().unwrap();
                        let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                        ops.extend(pre_ops);
                        let tmp = ctx.alloc_vreg();
                        ops.push(SmirOp::with_hint(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::VLoad {
                                dst: tmp,
                                addr,
                                width: prefix.width,
                            },
                            X86OpHint::VecAlign(X86VecAlign::Unaligned),
                        ));
                        tmp
                    } else {
                        self.vec_reg(modrm.rm, prefix.width)
                    };

                    let op_kind = match opcode {
                        0x58 | 0xFE => OpKind::VAdd {
                            dst,
                            src1,
                            src2,
                            elem,
                            lanes,
                        },
                        0x5C => OpKind::VSub {
                            dst,
                            src1,
                            src2,
                            elem,
                            lanes,
                        },
                        0x59 => OpKind::VMul {
                            dst,
                            src1,
                            src2,
                            elem,
                            lanes,
                        },
                        0x5F => OpKind::VMax {
                            dst,
                            src1,
                            src2,
                            elem,
                            lanes,
                        },
                        _ => {
                            return Err(LiftError::Unsupported {
                                addr: pc,
                                mnemonic: format!("VEX opcode 0x{:02X}", opcode),
                            });
                        }
                    };

                    ops.push(SmirOp::with_hint(OpId(ops.len() as u16), pc, op_kind, hint));

                    Ok(LiftResult::fallthrough(ops, cursor + modrm.bytes_consumed))
                }

                // VPSLLD imm8 (0F 72 /6)
                0x72 => {
                    let modrm = decode_modrm(after_opcode, &prefix_modrm, pc)?;
                    let imm_offset = modrm.bytes_consumed;
                    if after_opcode.len() <= imm_offset {
                        return Err(LiftError::Incomplete {
                            addr: pc,
                            have: bytes.len(),
                            need: cursor + imm_offset + 1,
                        });
                    }
                    let imm = after_opcode[imm_offset];
                    let next_pc = pc + cursor as u64 + modrm.bytes_consumed as u64 + 1;

                    let group = (modrm.byte >> 3) & 0x07;
                    if group != 6 {
                        return Err(LiftError::Unsupported {
                            addr: pc,
                            mnemonic: format!("VEX shift group {}", group),
                        });
                    }

                    let dst = self.vec_reg(prefix.vvvv, prefix.width);
                    let src = if modrm.is_memory {
                        let x86_addr = modrm.addr.as_ref().unwrap();
                        let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                        ops.extend(pre_ops);
                        let tmp = ctx.alloc_vreg();
                        ops.push(SmirOp::with_hint(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::VLoad {
                                dst: tmp,
                                addr,
                                width: prefix.width,
                            },
                            X86OpHint::VecAlign(X86VecAlign::Unaligned),
                        ));
                        tmp
                    } else {
                        self.vec_reg(modrm.rm, prefix.width)
                    };

                    ops.push(SmirOp::with_hint(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::VShift {
                            dst,
                            src,
                            amount: SrcOperand::Imm(imm as i64),
                            shift: ShiftOp::Lsl,
                            elem: VecElementType::I32,
                            lanes: prefix.width.lanes(VecElementType::I32) as u8,
                        },
                        hint,
                    ));

                    Ok(LiftResult::fallthrough(
                        ops,
                        cursor + modrm.bytes_consumed + 1,
                    ))
                }

                _ => Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: format!("VEX opcode 0x{:02X}", opcode),
                }),
            },
            X86VecMap::Map0F38 => match opcode {
                0xE0..=0xEF => self.lift_cmpccxadd(prefix, opcode, bytes, pc, ctx),
                0xF2 if prefix.encoding == VecEncodingKind::Vex
                    && prefix.pp == X86SsePrefix::None =>
                {
                    self.lift_vex_andn_0f38(prefix, bytes, pc, ctx)
                }
                0xF5 | 0xF7
                    if prefix.encoding == VecEncodingKind::Vex
                        && prefix.pp == X86SsePrefix::None =>
                {
                    self.lift_vex_bzhi_bextr_0f38(prefix, opcode, bytes, pc, ctx)
                }
                0xF5 if prefix.encoding == VecEncodingKind::Vex
                    && matches!(prefix.pp, X86SsePrefix::Rep | X86SsePrefix::Repne) =>
                {
                    self.lift_vex_pdep_pext_0f38(prefix, bytes, pc, ctx)
                }
                0xF6 if prefix.encoding == VecEncodingKind::Vex
                    && prefix.pp == X86SsePrefix::Repne =>
                {
                    self.lift_vex_mulx_0f38(prefix, bytes, pc, ctx)
                }
                0xF7 if prefix.encoding == VecEncodingKind::Vex
                    && matches!(
                        prefix.pp,
                        X86SsePrefix::OpSize | X86SsePrefix::Rep | X86SsePrefix::Repne
                    ) =>
                {
                    self.lift_vex_bmi2_shift_0f38(prefix, bytes, pc, ctx)
                }
                0xF5 | 0xF6 | 0xF7
                    if prefix.encoding == VecEncodingKind::Evex
                        && !matches!(prefix.pp, X86SsePrefix::None) =>
                {
                    self.lift_apx_bmi2_0f38(opcode, bytes, pc, ctx)
                }
                0xF2 | 0xF3 | 0xF5 | 0xF7
                    if prefix.encoding == VecEncodingKind::Evex
                        && prefix.pp == X86SsePrefix::None =>
                {
                    self.lift_apx_nf_bmi_0f38(opcode, bytes, pc, ctx)
                }
                0x40 => {
                    let modrm = decode_modrm(after_opcode, &prefix_modrm, pc)?;
                    let next_pc = pc + cursor as u64 + modrm.bytes_consumed as u64;
                    let dst = self.vec_reg(modrm.reg, prefix.width);
                    let src1 = self.vec_reg(prefix.vvvv, prefix.width);

                    let src2 = if modrm.is_memory {
                        let x86_addr = modrm.addr.as_ref().unwrap();
                        let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                        ops.extend(pre_ops);
                        let tmp = ctx.alloc_vreg();
                        ops.push(SmirOp::with_hint(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::VLoad {
                                dst: tmp,
                                addr,
                                width: prefix.width,
                            },
                            X86OpHint::VecAlign(X86VecAlign::Unaligned),
                        ));
                        tmp
                    } else {
                        self.vec_reg(modrm.rm, prefix.width)
                    };

                    ops.push(SmirOp::with_hint(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::VMul {
                            dst,
                            src1,
                            src2,
                            elem: VecElementType::I32,
                            lanes: prefix.width.lanes(VecElementType::I32) as u8,
                        },
                        hint,
                    ));

                    Ok(LiftResult::fallthrough(ops, cursor + modrm.bytes_consumed))
                }
                _ => Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: format!("VEX 0F38 opcode 0x{:02X}", opcode),
                }),
            },
            X86VecMap::Map0F3A => match opcode {
                0xF0 if prefix.encoding == VecEncodingKind::Vex
                    && prefix.pp == X86SsePrefix::Repne =>
                {
                    self.lift_vex_bmi2_rorx_0f3a(prefix, bytes, pc, ctx)
                }
                0xF0 if prefix.encoding == VecEncodingKind::Evex => {
                    self.lift_apx_bmi2_rorx(bytes, pc, ctx)
                }
                _ => Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: "VEX 0F3A".to_string(),
                }),
            },
        }
    }

    fn lift_vex_evex(
        &self,
        pc: u64,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let prefix = match bytes.first().copied() {
            Some(0x62) => decode_evex_prefix(bytes, pc)?,
            _ => decode_vex_prefix(bytes, pc)?,
        };

        self.lift_vec_opcode(prefix, bytes, pc, ctx)
    }

    fn lift_apx_push2(
        &self,
        prefix: ApxEvexPrefix,
        modrm: u8,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if (modrm >> 6) != 3 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX PUSH2 memory form".to_string(),
            });
        }
        let group = (modrm >> 3) & 0x07;
        if group != 6 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX FF /{group}"),
            });
        }

        let reg1 = (modrm & 0x07) | prefix.rm_ext();
        let reg2 = prefix.vvvv_reg();
        let tmp1 = ctx.alloc_vreg();
        let tmp2 = ctx.alloc_vreg();
        let rsp = self.rsp();
        let ops = vec![
            SmirOp::new(
                OpId(0),
                pc,
                OpKind::Mov {
                    dst: tmp1,
                    src: SrcOperand::Reg(self.gpr(reg1)),
                    width: OpWidth::W64,
                },
            ),
            SmirOp::new(
                OpId(1),
                pc,
                OpKind::Mov {
                    dst: tmp2,
                    src: SrcOperand::Reg(self.gpr(reg2)),
                    width: OpWidth::W64,
                },
            ),
            SmirOp::new(
                OpId(2),
                pc,
                OpKind::Sub {
                    dst: rsp,
                    src1: rsp,
                    src2: SrcOperand::Imm(16),
                    width: OpWidth::W64,
                    flags: FlagUpdate::None,
                },
            ),
            SmirOp::new(
                OpId(3),
                pc,
                OpKind::Store {
                    src: tmp1,
                    addr: Address::Direct(rsp),
                    width: MemWidth::B8,
                },
            ),
            SmirOp::new(
                OpId(4),
                pc,
                OpKind::Store {
                    src: tmp2,
                    addr: Address::base_off(rsp, 8),
                    width: MemWidth::B8,
                },
            ),
        ];

        Ok(LiftResult::fallthrough(ops, prefix.bytes + 2))
    }

    fn lift_apx_pop2(
        &self,
        prefix: ApxEvexPrefix,
        modrm: u8,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if (modrm >> 6) != 3 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX POP2 memory form".to_string(),
            });
        }
        let group = (modrm >> 3) & 0x07;
        if group != 0 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX 8F /{group}"),
            });
        }

        let reg1 = (modrm & 0x07) | prefix.rm_ext();
        let reg2 = prefix.vvvv_reg();
        let tmp1 = ctx.alloc_vreg();
        let tmp2 = ctx.alloc_vreg();
        let rsp = self.rsp();
        let ops = vec![
            SmirOp::new(
                OpId(0),
                pc,
                OpKind::Load {
                    dst: tmp1,
                    addr: Address::Direct(rsp),
                    width: MemWidth::B8,
                    sign: SignExtend::Zero,
                },
            ),
            SmirOp::new(
                OpId(1),
                pc,
                OpKind::Load {
                    dst: tmp2,
                    addr: Address::base_off(rsp, 8),
                    width: MemWidth::B8,
                    sign: SignExtend::Zero,
                },
            ),
            SmirOp::new(
                OpId(2),
                pc,
                OpKind::Add {
                    dst: rsp,
                    src1: rsp,
                    src2: SrcOperand::Imm(16),
                    width: OpWidth::W64,
                    flags: FlagUpdate::None,
                },
            ),
            SmirOp::new(
                OpId(3),
                pc,
                OpKind::Mov {
                    dst: self.gpr(reg1),
                    src: SrcOperand::Reg(tmp1),
                    width: OpWidth::W64,
                },
            ),
            SmirOp::new(
                OpId(4),
                pc,
                OpKind::Mov {
                    dst: self.gpr(reg2),
                    src: SrcOperand::Reg(tmp2),
                    width: OpWidth::W64,
                },
            ),
        ];

        Ok(LiftResult::fallthrough(ops, prefix.bytes + 2))
    }

    fn apx_alu_op(
        &self,
        group: u8,
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
        pc: u64,
    ) -> Result<OpKind, LiftError> {
        match group {
            0 => Ok(OpKind::Add {
                dst,
                src1,
                src2,
                width,
                flags,
            }),
            1 => Ok(OpKind::Or {
                dst,
                src1,
                src2,
                width,
                flags,
            }),
            2 => {
                if !flags.updates_any() {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: "APX NF ADC".to_string(),
                    });
                }
                Ok(OpKind::Adc {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                })
            }
            3 => {
                if !flags.updates_any() {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: "APX NF SBB".to_string(),
                    });
                }
                Ok(OpKind::Sbb {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                })
            }
            4 => Ok(OpKind::And {
                dst,
                src1,
                src2,
                width,
                flags,
            }),
            5 => Ok(OpKind::Sub {
                dst,
                src1,
                src2,
                width,
                flags,
            }),
            6 => Ok(OpKind::Xor {
                dst,
                src1,
                src2,
                width,
                flags,
            }),
            _ => Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX ALU group {group}"),
            }),
        }
    }

    fn lift_apx_alu(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let low = opcode & 0x07;
        let (is_byte, rm_is_legacy_dst) = match low {
            0 => (true, true),
            1 => (false, true),
            2 => (true, false),
            3 => (false, false),
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr: pc,
                    bytes: bytes.to_vec(),
                });
            }
        };
        let group = (opcode >> 3) & 0x07;
        let op_size = prefix.op_size(is_byte);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let reg = self.gpr(modrm.reg);
        let (rm, rm_addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let (legacy_dst, src2, legacy_dst_addr) = if rm_is_legacy_dst {
            (rm, reg, rm_addr)
        } else {
            (reg, rm, None)
        };
        let dst = if prefix.nd {
            self.gpr(prefix.vvvv_reg())
        } else {
            legacy_dst
        };
        let src2_operand = if prefix.nd && dst == src2 {
            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: tmp,
                    src: SrcOperand::Reg(src2),
                    width,
                },
            ));
            SrcOperand::Reg(tmp)
        } else {
            SrcOperand::Reg(src2)
        };
        let op_kind = self.apx_alu_op(
            group,
            dst,
            legacy_dst,
            src2_operand,
            width,
            prefix.flags(),
            pc,
        )?;
        let hint = X86OpHint::AluEncoding(if rm_is_legacy_dst {
            X86AluEncoding::RmReg
        } else {
            X86AluEncoding::RegRm
        });
        ops.push(SmirOp::with_hint(OpId(ops.len() as u16), pc, op_kind, hint));

        if !prefix.nd {
            if let Some(addr) = legacy_dst_addr {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: dst,
                        addr,
                        width: mem_width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_group1_imm(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = opcode == 0x80;
        let op_size = prefix.op_size(is_byte);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let imm_offset = modrm.bytes_consumed;

        let (imm, imm_size) = match opcode {
            0x80 => {
                if bytes.len() < imm_offset + 1 {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: bytes.len(),
                        need: imm_offset + 1,
                    });
                }
                (bytes[imm_offset] as i8 as i64, 1)
            }
            0x81 => {
                if bytes.len() < imm_offset + 4 {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: bytes.len(),
                        need: imm_offset + 4,
                    });
                }
                (
                    i32::from_le_bytes([
                        bytes[imm_offset],
                        bytes[imm_offset + 1],
                        bytes[imm_offset + 2],
                        bytes[imm_offset + 3],
                    ]) as i64,
                    4,
                )
            }
            0x83 => {
                if bytes.len() < imm_offset + 1 {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: bytes.len(),
                        need: imm_offset + 1,
                    });
                }
                (bytes[imm_offset] as i8 as i64, 1)
            }
            _ => unreachable!(),
        };

        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();
        let group = (modrm.byte >> 3) & 0x07;
        if group == 7 {
            if prefix.nd {
                return Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: "APX CCMP immediate with NDD".to_string(),
                });
            }

            let memory_load = if modrm.is_memory {
                let x86_addr = modrm.addr.as_ref().unwrap();
                let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                ops.extend(pre_ops);

                let tmp = ctx.alloc_vreg();
                Some((tmp, addr))
            } else {
                None
            };
            let src1 = memory_load
                .as_ref()
                .map(|(tmp, _)| *tmp)
                .unwrap_or_else(|| self.gpr(modrm.rm));

            self.push_apx_conditional_flags_with(
                &mut ops,
                pc,
                ctx,
                self.x86_cond(prefix.ccmp_cond()),
                prefix.ccmp_default_flags(),
                |ops, cond_reg| {
                    if let Some((dst, addr)) = memory_load {
                        ops.push(SmirOp::new(
                            OpId(ops.len() as u16),
                            pc,
                            OpKind::PredLoad {
                                dst,
                                cond: cond_reg,
                                addr,
                                width: mem_width,
                                signed: SignExtend::Zero,
                            },
                        ));
                    }
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Cmp {
                            src1,
                            src2: SrcOperand::Imm(imm),
                            width,
                        },
                    ));
                },
            );

            return Ok(LiftResult::fallthrough(
                ops,
                prefix.bytes + 1 + modrm.bytes_consumed + imm_size,
            ));
        }

        let (legacy_dst, legacy_dst_addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let dst = if prefix.nd {
            self.gpr(prefix.vvvv_reg())
        } else {
            legacy_dst
        };
        let op_kind = self.apx_alu_op(
            group,
            dst,
            legacy_dst,
            SrcOperand::Imm(imm),
            width,
            prefix.flags(),
            pc,
        )?;
        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));

        if !prefix.nd {
            if let Some(addr) = legacy_dst_addr {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: dst,
                        addr,
                        width: mem_width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed + imm_size,
        ))
    }

    fn lift_apx_movbe(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.nd || prefix.nf || prefix.pp > 1 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX MOVBE with NDD/NF".to_string(),
            });
        }

        let op_size = prefix.op_size(false);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        match opcode {
            0x60 => {
                let dst = self.gpr(modrm.reg);
                let src = if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Load {
                            dst: tmp,
                            addr,
                            width: mem_width,
                            sign: SignExtend::Zero,
                        },
                    ));
                    tmp
                } else {
                    self.gpr(modrm.rm)
                };

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Bswap { dst, src, width },
                ));
            }
            0x61 => {
                let src = self.gpr(modrm.reg);
                if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Bswap {
                            dst: tmp,
                            src,
                            width,
                        },
                    ));
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Store {
                            src: tmp,
                            addr,
                            width: mem_width,
                        },
                    ));
                } else {
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Bswap {
                            dst: self.gpr(modrm.rm),
                            src,
                            width,
                        },
                    ));
                }
            }
            _ => unreachable!("APX MOVBE is only dispatched for opcodes 0x60 and 0x61"),
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_movrs(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = opcode == 0x8A;
        if prefix.nd
            || prefix.nf
            || prefix.pp > 1
            || prefix.vvvv != 0x0F
            || !prefix.v_prime
            || (is_byte && (prefix.w || prefix.pp != 0))
        {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX MOVRS reserved EVEX field".to_string(),
            });
        }

        let op_size = prefix.op_size(is_byte);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        if !modrm.is_memory {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let x86_addr = modrm.addr.as_ref().unwrap();
        let (addr, mut ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Load {
                dst: self.gpr(modrm.reg),
                addr,
                width: mem_width,
                sign: SignExtend::Zero,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn apx_ccmp_default_rflags(dfv: u8) -> i64 {
        let mut flags = 0x02;
        if dfv & 0x1 != 0 {
            flags |= 0x001;
        }
        if dfv & 0x2 != 0 {
            flags |= 0x040;
        }
        if dfv & 0x4 != 0 {
            flags |= 0x080;
        }
        if dfv & 0x8 != 0 {
            flags |= 0x800;
        }
        flags
    }

    fn push_apx_conditional_flags_with(
        &self,
        ops: &mut Vec<SmirOp>,
        pc: u64,
        ctx: &mut LiftContext,
        cond: Condition,
        dfv: u8,
        push_true_ops: impl FnOnce(&mut Vec<SmirOp>, VReg),
    ) {
        let old_flags = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::ReadFlags { dst: old_flags },
        ));

        let cond_reg = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::SetCC {
                dst: cond_reg,
                cond,
                width: OpWidth::W64,
            },
        ));

        let false_flags = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::And {
                dst: false_flags,
                src1: old_flags,
                src2: SrcOperand::Imm(!APX_CCMP_FLAGS_MASK),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Or {
                dst: false_flags,
                src1: false_flags,
                src2: SrcOperand::Imm(Self::apx_ccmp_default_rflags(dfv)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        push_true_ops(ops, cond_reg);

        let true_flags = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::ReadFlags { dst: true_flags },
        ));

        let selected_flags = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Select {
                dst: selected_flags,
                cond: cond_reg,
                src_true: true_flags,
                src_false: false_flags,
                width: OpWidth::W64,
            },
        ));
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::WriteFlags {
                src: selected_flags,
            },
        ));
    }

    fn lift_apx_ccmp(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.nd {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX CCMP with NDD".to_string(),
            });
        }

        let is_byte = (opcode & 0x01) == 0;
        let op_size = prefix.op_size(is_byte);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;

        let reg_is_src = (opcode & 0x02) == 0;
        let mut ops = Vec::new();
        let memory_load = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            Some((tmp, addr))
        } else {
            None
        };
        let rm_src = memory_load
            .as_ref()
            .map(|(tmp, _)| *tmp)
            .unwrap_or_else(|| self.gpr(modrm.rm));
        let (src1, src2, hint) = if reg_is_src {
            (rm_src, self.gpr(modrm.reg), X86AluEncoding::RmReg)
        } else {
            (self.gpr(modrm.reg), rm_src, X86AluEncoding::RegRm)
        };

        self.push_apx_conditional_flags_with(
            &mut ops,
            pc,
            ctx,
            self.x86_cond(prefix.ccmp_cond()),
            prefix.ccmp_default_flags(),
            |ops, cond_reg| {
                if let Some((dst, addr)) = memory_load {
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::PredLoad {
                            dst,
                            cond: cond_reg,
                            addr,
                            width: mem_width,
                            signed: SignExtend::Zero,
                        },
                    ));
                }
                ops.push(SmirOp::with_hint(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Cmp {
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width,
                    },
                    X86OpHint::AluEncoding(hint),
                ));
            },
        );

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_ctest_reg(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.nd {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX CTEST with NDD".to_string(),
            });
        }

        let is_byte = opcode == 0x84;
        let op_size = prefix.op_size(is_byte);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;

        let mut ops = Vec::new();
        let memory_load = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            Some((tmp, addr))
        } else {
            None
        };
        let src1 = memory_load
            .as_ref()
            .map(|(tmp, _)| *tmp)
            .unwrap_or_else(|| self.gpr(modrm.rm));

        self.push_apx_conditional_flags_with(
            &mut ops,
            pc,
            ctx,
            self.x86_cond(prefix.ccmp_cond()),
            prefix.ccmp_default_flags(),
            |ops, cond_reg| {
                if let Some((dst, addr)) = memory_load {
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::PredLoad {
                            dst,
                            cond: cond_reg,
                            addr,
                            width: mem_width,
                            signed: SignExtend::Zero,
                        },
                    ));
                }
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Test {
                        src1,
                        src2: SrcOperand::Reg(self.gpr(modrm.reg)),
                        width,
                    },
                ));
            },
        );

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_ctest_imm(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.nd {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX CTEST immediate with NDD".to_string(),
            });
        }

        let is_byte = opcode == 0xF6;
        let op_size = prefix.op_size(is_byte);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let group = (modrm.byte >> 3) & 0x07;
        if group != 0 {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX F6/F7 /{group}"),
            });
        }

        let imm_offset = modrm.bytes_consumed;
        let imm_size = if is_byte {
            1
        } else if op_size == 2 {
            2
        } else {
            4
        };
        if bytes.len() < imm_offset + imm_size {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: imm_offset + imm_size,
            });
        }

        let imm = match imm_size {
            1 => bytes[imm_offset] as i8 as i64,
            2 => i16::from_le_bytes([bytes[imm_offset], bytes[imm_offset + 1]]) as i64,
            _ => i32::from_le_bytes([
                bytes[imm_offset],
                bytes[imm_offset + 1],
                bytes[imm_offset + 2],
                bytes[imm_offset + 3],
            ]) as i64,
        };

        let mut ops = Vec::new();
        let memory_load = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            Some((tmp, addr))
        } else {
            None
        };
        let src1 = memory_load
            .as_ref()
            .map(|(tmp, _)| *tmp)
            .unwrap_or_else(|| self.gpr(modrm.rm));

        self.push_apx_conditional_flags_with(
            &mut ops,
            pc,
            ctx,
            self.x86_cond(prefix.ccmp_cond()),
            prefix.ccmp_default_flags(),
            |ops, cond_reg| {
                if let Some((dst, addr)) = memory_load {
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::PredLoad {
                            dst,
                            cond: cond_reg,
                            addr,
                            width: mem_width,
                            signed: SignExtend::Zero,
                        },
                    ));
                }
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Test {
                        src1,
                        src2: SrcOperand::Imm(imm),
                        width,
                    },
                ));
            },
        );

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed + imm_size,
        ))
    }

    fn lift_apx_group3(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = opcode == 0xF6;
        let op_size = prefix.op_size(is_byte);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let group = (modrm.byte >> 3) & 0x07;

        if group == 0 {
            return self.lift_apx_ctest_imm(prefix, opcode, bytes, pc, ctx);
        }

        if !matches!(group, 2..=7) {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX F6/F7 /{group}"),
            });
        }

        if matches!(group, 4..=7) && (!prefix.nf || prefix.nd) {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX F6/F7 /{group} without implicit NF form"),
            });
        }

        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();
        let (src, store_addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let dst = if prefix.nd {
            self.gpr(prefix.vvvv_reg())
        } else {
            src
        };

        match group {
            2 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Not { dst, src, width },
            )),
            3 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Neg {
                    dst,
                    src,
                    width,
                    flags: prefix.flags(),
                },
            )),
            4 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::MulU {
                    dst_lo: self.gpr(0),
                    dst_hi: Some(self.gpr(2)),
                    src1: self.gpr(0),
                    src2: SrcOperand::Reg(src),
                    width,
                    flags: prefix.flags(),
                },
            )),
            5 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::MulS {
                    dst_lo: self.gpr(0),
                    dst_hi: Some(self.gpr(2)),
                    src1: self.gpr(0),
                    src2: SrcOperand::Reg(src),
                    width,
                    flags: prefix.flags(),
                },
            )),
            6 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::DivU {
                    quot: self.gpr(0),
                    rem: Some(self.gpr(2)),
                    src1: self.gpr(0),
                    src2: SrcOperand::Reg(src),
                    width,
                    flags: prefix.flags(),
                },
            )),
            7 => ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::DivS {
                    quot: self.gpr(0),
                    rem: Some(self.gpr(2)),
                    src1: self.gpr(0),
                    src2: SrcOperand::Reg(src),
                    width,
                    flags: prefix.flags(),
                },
            )),
            _ => unreachable!(),
        }

        if !prefix.nd && matches!(group, 2 | 3) {
            if let Some(addr) = store_addr {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: dst,
                        addr,
                        width: mem_width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_inc_dec(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = opcode == 0xFE;
        let op_size = prefix.op_size(is_byte);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let group = (modrm.byte >> 3) & 0x07;
        if !matches!(group, 0 | 1) {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX FE/FF /{group}"),
            });
        }

        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();
        let (src, store_addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let dst = if prefix.nd {
            self.gpr(prefix.vvvv_reg())
        } else {
            src
        };

        if group == 0 {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Inc {
                    dst,
                    src,
                    width,
                    flags: prefix.flags(),
                },
            ));
        } else {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Dec {
                    dst,
                    src,
                    width,
                    flags: prefix.flags(),
                },
            ));
        }

        if !prefix.nd {
            if let Some(addr) = store_addr {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: dst,
                        addr,
                        width: mem_width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_count(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.nd || !prefix.nf {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX count without required NF-only form".to_string(),
            });
        }

        let op_size = prefix.op_size(false);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };
        let dst = self.gpr(modrm.reg);
        let kind = match opcode {
            0x88 => OpKind::Popcnt { dst, src, width },
            0xF4 => OpKind::Ctz { dst, src, width },
            0xF5 => OpKind::Clz { dst, src, width },
            _ => unreachable!(),
        };
        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, kind));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_rao_int(
        &self,
        prefix: ApxEvexPrefix,
        bytes: &[u8],
        full_bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.nd
            || prefix.nf
            || prefix.aaa != 0
            || prefix.vvvv != 0x0F
            || !prefix.v_prime
            || (full_bytes[3] & 0xE0) != 0
        {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: full_bytes.to_vec(),
            });
        }

        let op = match prefix.pp {
            0 => AtomicOp::Add,
            1 => AtomicOp::And,
            2 => AtomicOp::Or,
            3 => AtomicOp::Xor,
            _ => unreachable!(),
        };
        let width = if prefix.w { MemWidth::B8 } else { MemWidth::B4 };
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        self.lift_rao_int_modrm(bytes, &modrm_prefix, pc, ctx, op, width)
    }

    fn lift_apx_adx(
        &self,
        prefix: ApxEvexPrefix,
        bytes: &[u8],
        full_bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if !prefix.nd || prefix.nf || (full_bytes[3] & 0x80) != 0 {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: full_bytes.to_vec(),
            });
        }

        let kind = match prefix.pp {
            1 => AdxKind::Adcx,
            2 => AdxKind::Adox,
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr: pc,
                    bytes: full_bytes.to_vec(),
                });
            }
        };
        let width = if prefix.w { OpWidth::W64 } else { OpWidth::W32 };
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let dst = self.gpr(prefix.vvvv_reg());
        self.lift_adx_modrm(bytes, &modrm_prefix, pc, ctx, kind, width, Some(dst))
    }

    fn lift_apx_setzucc(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let cond = self.x86_cond(opcode & 0x0F);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::SetCC {
                    dst: tmp,
                    cond,
                    width: OpWidth::W8,
                },
            ));
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Store {
                    src: tmp,
                    addr,
                    width: MemWidth::B1,
                },
            ));
        } else {
            ops.push(SmirOp::new(
                OpId(0),
                pc,
                OpKind::SetCC {
                    dst: self.gpr(modrm.rm),
                    cond,
                    width: OpWidth::W64,
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn push_apx_condition(
        &self,
        ops: &mut Vec<SmirOp>,
        pc: u64,
        ctx: &mut LiftContext,
        cond: Condition,
    ) -> VReg {
        let cond_reg = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::SetCC {
                dst: cond_reg,
                cond,
                width: OpWidth::W8,
            },
        ));
        cond_reg
    }

    fn lift_apx_evex_setcc(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let cond = self.x86_cond(opcode & 0x0F);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::SetCC {
                    dst: tmp,
                    cond,
                    width: OpWidth::W8,
                },
            ));
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Store {
                    src: tmp,
                    addr,
                    width: MemWidth::B1,
                },
            ));
        } else {
            ops.push(SmirOp::new(
                OpId(0),
                pc,
                OpKind::SetCC {
                    dst: self.gpr(modrm.rm),
                    cond,
                    width: OpWidth::W8,
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_cmovcc(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let cond = self.x86_cond(opcode & 0x0F);
        let op_size = prefix.op_size(false);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        if prefix.nd {
            let dst = self.gpr(prefix.vvvv_reg());
            let src1 = self.gpr(modrm.reg);

            if prefix.nf {
                let cond_reg = self.push_apx_condition(&mut ops, pc, ctx, cond);
                if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let loaded = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::PredLoad {
                            dst: loaded,
                            cond: cond_reg,
                            addr,
                            width: mem_width,
                            signed: SignExtend::Zero,
                        },
                    ));
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Select {
                            dst,
                            cond: cond_reg,
                            src_true: loaded,
                            src_false: src1,
                            width,
                        },
                    ));
                } else {
                    let src2 = self.gpr(modrm.rm);
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Select {
                            dst,
                            cond: cond_reg,
                            src_true: src2,
                            src_false: src1,
                            width,
                        },
                    ));
                }
            } else {
                let src2 = if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Load {
                            dst: tmp,
                            addr,
                            width: mem_width,
                            sign: SignExtend::Zero,
                        },
                    ));
                    tmp
                } else {
                    self.gpr(modrm.rm)
                };

                let cond_reg = self.push_apx_condition(&mut ops, pc, ctx, cond);
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Select {
                        dst,
                        cond: cond_reg,
                        src_true: src2,
                        src_false: src1,
                        width,
                    },
                ));
            }
        } else if prefix.nf {
            let src = self.gpr(modrm.reg);
            let cond_reg = self.push_apx_condition(&mut ops, pc, ctx, cond);

            if modrm.is_memory {
                let x86_addr = modrm.addr.as_ref().unwrap();
                let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                ops.extend(pre_ops);

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::PredStore {
                        src: SrcOperand::Reg(src),
                        cond: cond_reg,
                        addr,
                        width: mem_width,
                    },
                ));
            } else {
                let dst = self.gpr(modrm.rm);
                let zero = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Mov {
                        dst: zero,
                        src: SrcOperand::Imm(0),
                        width,
                    },
                ));
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Select {
                        dst,
                        cond: cond_reg,
                        src_true: src,
                        src_false: zero,
                        width,
                    },
                ));
            }
        } else {
            let dst = self.gpr(modrm.reg);
            let zero = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: zero,
                    src: SrcOperand::Imm(0),
                    width,
                },
            ));
            let cond_reg = self.push_apx_condition(&mut ops, pc, ctx, cond);

            let src = if modrm.is_memory {
                let x86_addr = modrm.addr.as_ref().unwrap();
                let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                ops.extend(pre_ops);

                let loaded = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::PredLoad {
                        dst: loaded,
                        cond: cond_reg,
                        addr,
                        width: mem_width,
                        signed: SignExtend::Zero,
                    },
                ));
                loaded
            } else {
                self.gpr(modrm.rm)
            };

            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Select {
                    dst,
                    cond: cond_reg,
                    src_true: src,
                    src_false: zero,
                    width,
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_conditional_map4(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if prefix.pp == 0x02 {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes[..bytes.len().min(2)].to_vec(),
            });
        }

        if prefix.pp == 0x03 && !prefix.nf {
            if prefix.nd {
                self.lift_apx_setzucc(prefix, opcode, bytes, pc, ctx)
            } else {
                self.lift_apx_evex_setcc(prefix, opcode, bytes, pc, ctx)
            }
        } else {
            self.lift_apx_cmovcc(prefix, opcode, bytes, pc, ctx)
        }
    }

    fn lift_apx_imul_reg(
        &self,
        prefix: ApxEvexPrefix,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op_size = prefix.op_size(false);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let src1 = self.gpr(modrm.reg);
        let src2 = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        let dst = if prefix.nd {
            self.gpr(prefix.vvvv_reg())
        } else {
            src1
        };
        let src2_operand = if prefix.nd && dst == src2 && dst != src1 {
            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: tmp,
                    src: SrcOperand::Reg(src2),
                    width,
                },
            ));
            SrcOperand::Reg(tmp)
        } else {
            SrcOperand::Reg(src2)
        };

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::MulS {
                dst_lo: dst,
                dst_hi: None,
                src1,
                src2: src2_operand,
                width,
                flags: prefix.flags(),
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed,
        ))
    }

    fn lift_apx_imul_imm(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op_size = prefix.op_size(false);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let imm_offset = modrm.bytes_consumed;
        let imm_size = if opcode == 0x6B { 1 } else { 4 };

        if bytes.len() < imm_offset + imm_size {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: imm_offset + imm_size,
            });
        }

        let imm = if opcode == 0x6B {
            bytes[imm_offset] as i8 as i64
        } else {
            i32::from_le_bytes([
                bytes[imm_offset],
                bytes[imm_offset + 1],
                bytes[imm_offset + 2],
                bytes[imm_offset + 3],
            ]) as i64
        };

        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();
        let src1 = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };
        let dst = if prefix.nd {
            self.gpr(prefix.vvvv_reg())
        } else {
            self.gpr(modrm.reg)
        };
        let hint = if opcode == 0x6B {
            X86OpHint::ImulImm8
        } else {
            X86OpHint::ImulImm32
        };

        ops.push(SmirOp::with_hint(
            OpId(ops.len() as u16),
            pc,
            OpKind::MulS {
                dst_lo: dst,
                dst_hi: None,
                src1,
                src2: SrcOperand::Imm(imm),
                width,
                flags: prefix.flags(),
            },
            hint,
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed + imm_size,
        ))
    }

    fn apx_shift_op(
        &self,
        group: u8,
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
        pc: u64,
    ) -> Result<OpKind, LiftError> {
        if matches!(group, 2 | 3) && !flags.updates_any() {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "APX NF carry rotate".to_string(),
            });
        }
        let rotate_flags = if flags.updates_any() {
            x86_rotate_flags()
        } else {
            FlagUpdate::None
        };
        match group {
            0 => Ok(OpKind::Rol {
                dst,
                src,
                amount,
                width,
                flags: rotate_flags,
            }),
            1 => Ok(OpKind::Ror {
                dst,
                src,
                amount,
                width,
                flags: rotate_flags,
            }),
            2 => Ok(OpKind::Rcl {
                dst,
                src,
                amount,
                width,
                flags: rotate_flags,
            }),
            3 => Ok(OpKind::Rcr {
                dst,
                src,
                amount,
                width,
                flags: rotate_flags,
            }),
            4 | 6 => Ok(OpKind::Shl {
                dst,
                src,
                amount,
                width,
                flags,
            }),
            5 => Ok(OpKind::Shr {
                dst,
                src,
                amount,
                width,
                flags,
            }),
            7 => Ok(OpKind::Sar {
                dst,
                src,
                amount,
                width,
                flags,
            }),
            _ => Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX shift group {group}"),
            }),
        }
    }

    fn lift_apx_shift(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_byte = matches!(opcode, 0xC0 | 0xD0 | 0xD2);
        let op_size = prefix.op_size(is_byte);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let group = (modrm.byte >> 3) & 0x07;

        let (amount, imm_size) = match opcode {
            0xC0 | 0xC1 => {
                if bytes.len() < modrm.bytes_consumed + 1 {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: bytes.len(),
                        need: modrm.bytes_consumed + 1,
                    });
                }
                (SrcOperand::Imm(bytes[modrm.bytes_consumed] as i64), 1)
            }
            0xD0 | 0xD1 => (SrcOperand::Imm(1), 0),
            0xD2 | 0xD3 => (SrcOperand::Reg(self.gpr(1)), 0),
            _ => unreachable!(),
        };

        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();
        let (legacy_dst, legacy_dst_addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let dst = if prefix.nd {
            self.gpr(prefix.vvvv_reg())
        } else {
            legacy_dst
        };
        let amount = if prefix.nd && dst == self.gpr(1) {
            match amount {
                SrcOperand::Reg(reg) if reg == self.gpr(1) => {
                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Mov {
                            dst: tmp,
                            src: SrcOperand::Reg(reg),
                            width: OpWidth::W8,
                        },
                    ));
                    SrcOperand::Reg(tmp)
                }
                other => other,
            }
        } else {
            amount
        };
        let op_kind =
            self.apx_shift_op(group, dst, legacy_dst, amount, width, prefix.flags(), pc)?;
        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));

        if !prefix.nd {
            if let Some(addr) = legacy_dst_addr {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: dst,
                        addr,
                        width: mem_width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed + imm_size,
        ))
    }

    fn lift_apx_double_shift(
        &self,
        prefix: ApxEvexPrefix,
        opcode: u8,
        bytes: &[u8],
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let uses_cl = matches!(opcode, 0xA5 | 0xAD);
        let is_shld = matches!(opcode, 0x24 | 0xA5);
        let op_size = prefix.op_size(false);
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);
        let modrm_prefix = prefix.as_modrm_prefix(prefix.bytes + 1);
        let modrm = decode_modrm(bytes, &modrm_prefix, pc)?;
        let imm_size = if uses_cl { 0 } else { 1 };
        if bytes.len() < modrm.bytes_consumed + imm_size {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: modrm.bytes_consumed + imm_size,
            });
        }

        let amount = if uses_cl {
            SrcOperand::Reg(self.gpr(1))
        } else {
            SrcOperand::Imm(bytes[modrm.bytes_consumed] as i8 as i64)
        };
        let next_pc = pc + prefix.bytes as u64 + 1 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();
        let (legacy_dst, legacy_dst_addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let architectural_dst = if prefix.nd {
            self.gpr(prefix.vvvv_reg())
        } else {
            legacy_dst
        };
        let amount_uses_cl = matches!(amount, SrcOperand::Reg(reg) if reg == self.gpr(1));
        let op_dst = if prefix.nd
            && amount_uses_cl
            && architectural_dst == self.gpr(1)
            && architectural_dst != legacy_dst
        {
            ctx.alloc_vreg()
        } else {
            architectural_dst
        };
        let mut src = self.gpr(modrm.reg);

        if prefix.nd && op_dst == src && op_dst != legacy_dst {
            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: tmp,
                    src: SrcOperand::Reg(src),
                    width,
                },
            ));
            src = tmp;
        }

        if op_dst != legacy_dst {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: op_dst,
                    src: SrcOperand::Reg(legacy_dst),
                    width,
                },
            ));
        }

        let op_kind = if is_shld {
            OpKind::Shld {
                dst: op_dst,
                src,
                amount,
                width,
                flags: prefix.flags(),
            }
        } else {
            OpKind::Shrd {
                dst: op_dst,
                src,
                amount,
                width,
                flags: prefix.flags(),
            }
        };
        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));

        if op_dst != architectural_dst {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: architectural_dst,
                    src: SrcOperand::Reg(op_dst),
                    width,
                },
            ));
        }

        if !prefix.nd {
            if let Some(addr) = legacy_dst_addr {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: op_dst,
                        addr,
                        width: mem_width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.bytes + 1 + modrm.bytes_consumed + imm_size,
        ))
    }

    fn lift_apx_evex_map4(
        &self,
        pc: u64,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let prefix = decode_apx_evex_prefix(bytes, pc)?;
        if bytes.len() < prefix.bytes + 1 {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: prefix.bytes + 1,
            });
        }

        let opcode = bytes[prefix.bytes];
        if bytes.len() < prefix.bytes + 2 {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: prefix.bytes + 2,
            });
        }
        let modrm = bytes[prefix.bytes + 1];
        match opcode {
            0x00..=0x03
            | 0x08..=0x0B
            | 0x10..=0x13
            | 0x18..=0x1B
            | 0x20..=0x23
            | 0x28..=0x2B
            | 0x30..=0x33 => self.lift_apx_alu(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx),
            0x38..=0x3B => self.lift_apx_ccmp(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx),
            0x80 | 0x81 | 0x83 => {
                self.lift_apx_group1_imm(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx)
            }
            0x84 | 0x85 => {
                self.lift_apx_ctest_reg(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx)
            }
            0xC0 | 0xC1 | 0xD0 | 0xD1 | 0xD2 | 0xD3 => {
                self.lift_apx_shift(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx)
            }
            0x24 | 0x2C | 0xA5 | 0xAD => {
                self.lift_apx_double_shift(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx)
            }
            0x40..=0x4F => {
                self.lift_apx_conditional_map4(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx)
            }
            0x60 | 0x61 => self.lift_apx_movbe(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx),
            0x66 => self.lift_apx_adx(prefix, &bytes[prefix.bytes + 1..], bytes, pc, ctx),
            0x8A | 0x8B => self.lift_apx_movrs(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx),
            0x69 | 0x6B => {
                self.lift_apx_imul_imm(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx)
            }
            0xAF => self.lift_apx_imul_reg(prefix, &bytes[prefix.bytes + 1..], pc, ctx),
            0x88 if prefix.nf => {
                self.lift_apx_count(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx)
            }
            0xF4 | 0xF5 => self.lift_apx_count(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx),
            0xFC => self.lift_apx_rao_int(prefix, &bytes[prefix.bytes + 1..], bytes, pc, ctx),
            0xF6 | 0xF7 => {
                self.lift_apx_group3(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx)
            }
            0xFE => self.lift_apx_inc_dec(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx),
            0x8F => self.lift_apx_pop2(prefix, modrm, pc, ctx),
            0xFF if ((modrm >> 3) & 0x07) == 6 => self.lift_apx_push2(prefix, modrm, pc, ctx),
            0xFF => self.lift_apx_inc_dec(prefix, opcode, &bytes[prefix.bytes + 1..], pc, ctx),
            _ => Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("APX MAP4 opcode 0x{opcode:02X}"),
            }),
        }
    }

    /// Lift group 5 instructions (FF)
    fn lift_group5(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op_size = prefix.op_size();
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let mut ops = Vec::new();

        let group = (modrm.byte >> 3) & 0x07;

        if modrm.is_memory && (group == 2 || group == 4) {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);
            let control_flow = if group == 2 {
                ControlFlow::Call {
                    target: CallTarget::IndirectMem(addr),
                }
            } else {
                ControlFlow::IndirectBranchMem { addr }
            };

            return Ok(LiftResult {
                ops,
                bytes_consumed: prefix.cursor + modrm.bytes_consumed,
                control_flow,
                branch_targets: vec![],
            });
        }

        let (operand, addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        match group {
            0 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Inc {
                        dst: operand,
                        src: operand,
                        width,
                        flags: FlagUpdate::All,
                    },
                ));
                if let Some(addr) = addr {
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Store {
                            src: operand,
                            addr,
                            width: mem_width,
                        },
                    ));
                }
                Ok(LiftResult::fallthrough(
                    ops,
                    prefix.cursor + modrm.bytes_consumed,
                ))
            }
            1 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Dec {
                        dst: operand,
                        src: operand,
                        width,
                        flags: FlagUpdate::All,
                    },
                ));
                if let Some(addr) = addr {
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Store {
                            src: operand,
                            addr,
                            width: mem_width,
                        },
                    ));
                }
                Ok(LiftResult::fallthrough(
                    ops,
                    prefix.cursor + modrm.bytes_consumed,
                ))
            }
            2 => Ok(LiftResult {
                ops,
                bytes_consumed: prefix.cursor + modrm.bytes_consumed,
                control_flow: ControlFlow::Call {
                    target: CallTarget::Indirect(operand),
                },
                branch_targets: vec![],
            }),
            4 => Ok(LiftResult {
                ops,
                bytes_consumed: prefix.cursor + modrm.bytes_consumed,
                control_flow: ControlFlow::IndirectBranch { target: operand },
                branch_targets: vec![],
            }),
            6 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Sub {
                        dst: self.rsp(),
                        src1: self.rsp(),
                        src2: SrcOperand::Imm(8),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                ));
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: operand,
                        addr: Address::Direct(self.rsp()),
                        width: MemWidth::B8,
                    },
                ));
                Ok(LiftResult::fallthrough(
                    ops,
                    prefix.cursor + modrm.bytes_consumed,
                ))
            }
            _ => {
                if self.strict {
                    Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("group5 {}", group),
                    })
                } else {
                    Ok(LiftResult::fallthrough(
                        vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                        prefix.cursor + modrm.bytes_consumed,
                    ))
                }
            }
        }
    }

    /// Lift group 3 instructions (F6/F7)
    fn lift_group3(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_8bit = opcode == 0xF6;
        let op_size = if is_8bit { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let group = (modrm.byte >> 3) & 0x07;
        let imm_size = if group == 0 {
            if is_8bit {
                1
            } else if op_size == 2 {
                2
            } else {
                4
            }
        } else {
            0
        };

        if bytes.len() < modrm.bytes_consumed + imm_size {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: modrm.bytes_consumed + imm_size,
            });
        }

        let imm = if imm_size == 0 {
            0
        } else if imm_size == 1 {
            bytes[modrm.bytes_consumed] as i8 as i64
        } else if imm_size == 2 {
            i16::from_le_bytes([bytes[modrm.bytes_consumed], bytes[modrm.bytes_consumed + 1]])
                as i64
        } else {
            i32::from_le_bytes([
                bytes[modrm.bytes_consumed],
                bytes[modrm.bytes_consumed + 1],
                bytes[modrm.bytes_consumed + 2],
                bytes[modrm.bytes_consumed + 3],
            ]) as i64
        };

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();

        let (operand, addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        match group {
            0 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Test {
                        src1: operand,
                        src2: SrcOperand::Imm(imm),
                        width,
                    },
                ));
            }
            2 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Not {
                        dst: operand,
                        src: operand,
                        width,
                    },
                ));
            }
            3 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Neg {
                        dst: operand,
                        src: operand,
                        width,
                        flags: FlagUpdate::All,
                    },
                ));
            }
            4 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::MulU {
                        dst_lo: self.gpr(0),
                        dst_hi: Some(self.gpr(2)),
                        src1: self.gpr(0),
                        src2: SrcOperand::Reg(operand),
                        width,
                        flags: FlagUpdate::All,
                    },
                ));
            }
            5 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::MulS {
                        dst_lo: self.gpr(0),
                        dst_hi: Some(self.gpr(2)),
                        src1: self.gpr(0),
                        src2: SrcOperand::Reg(operand),
                        width,
                        flags: FlagUpdate::All,
                    },
                ));
            }
            6 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::DivU {
                        quot: self.gpr(0),
                        rem: Some(self.gpr(2)),
                        src1: self.gpr(0),
                        src2: SrcOperand::Reg(operand),
                        width,
                        flags: FlagUpdate::All,
                    },
                ));
            }
            7 => {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::DivS {
                        quot: self.gpr(0),
                        rem: Some(self.gpr(2)),
                        src1: self.gpr(0),
                        src2: SrcOperand::Reg(operand),
                        width,
                        flags: FlagUpdate::All,
                    },
                ));
            }
            _ => {
                if self.strict {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("group3 {}", group),
                    });
                }
                ops.push(SmirOp::new(OpId(ops.len() as u16), pc, OpKind::Nop));
            }
        }

        if matches!(group, 2 | 3) {
            if let Some(addr) = addr {
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: operand,
                        addr,
                        width: mem_width,
                    },
                ));
            }
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed + imm_size,
        ))
    }

    /// Lift IMUL r, r/m, imm (69/6B)
    fn lift_imul_rmi(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let op_size = prefix.op_size();
        let width = self.size_to_width(op_size);
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let imm_offset = modrm.bytes_consumed;
        let imm_size = if opcode == 0x6B {
            1
        } else if op_size == 2 {
            2
        } else {
            4
        };

        if bytes.len() < imm_offset + imm_size {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: imm_offset + imm_size,
            });
        }

        let imm = match imm_size {
            1 => bytes[imm_offset] as i8 as i64,
            2 => i16::from_le_bytes([bytes[imm_offset], bytes[imm_offset + 1]]) as i64,
            _ => i32::from_le_bytes([
                bytes[imm_offset],
                bytes[imm_offset + 1],
                bytes[imm_offset + 2],
                bytes[imm_offset + 3],
            ]) as i64,
        };

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();

        let src = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: self.size_to_memwidth(op_size),
                    sign: SignExtend::Zero,
                },
            ));
            tmp
        } else {
            self.gpr(modrm.rm)
        };

        let hint = if opcode == 0x6B {
            X86OpHint::ImulImm8
        } else {
            X86OpHint::ImulImm32
        };

        ops.push(SmirOp::with_hint(
            OpId(ops.len() as u16),
            pc,
            OpKind::MulS {
                dst_lo: self.gpr(modrm.reg),
                dst_hi: None,
                src1: src,
                src2: SrcOperand::Imm(imm),
                width,
                flags: FlagUpdate::All,
            },
            hint,
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed + imm_size,
        ))
    }

    /// Lift SHLD/SHRD (0F A4/A5/AC/AD)
    fn lift_shld_shrd(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let uses_cl = opcode == 0xA5 || opcode == 0xAD;
        let is_shld = opcode == 0xA4 || opcode == 0xA5;

        let op_size = prefix.op_size();
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let imm_size = if uses_cl { 0 } else { 1 };
        if bytes.len() < modrm.bytes_consumed + imm_size {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: modrm.bytes_consumed + imm_size,
            });
        }

        let imm = if uses_cl {
            0
        } else {
            bytes[modrm.bytes_consumed] as i8 as i64
        };

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();

        let (dst, addr) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr: addr.clone(),
                    width: mem_width,
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, Some(addr))
        } else {
            (self.gpr(modrm.rm), None)
        };

        let amount = if uses_cl {
            SrcOperand::Reg(self.gpr(1))
        } else {
            SrcOperand::Imm(imm)
        };

        let op_kind = if is_shld {
            OpKind::Shld {
                dst,
                src: self.gpr(modrm.reg),
                amount,
                width,
                flags: FlagUpdate::All,
            }
        } else {
            OpKind::Shrd {
                dst,
                src: self.gpr(modrm.reg),
                amount,
                width,
                flags: FlagUpdate::All,
            }
        };

        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, op_kind));

        if let Some(addr) = addr {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Store {
                    src: dst,
                    addr,
                    width: mem_width,
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed + imm_size,
        ))
    }

    /// Lift MOV r, imm (B8-BF)
    fn lift_mov_r_imm(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let reg = (opcode & 0x07) | prefix.rex_b();
        let op_size = prefix.op_size();
        let width = self.size_to_width(op_size);

        // In 64-bit mode with REX.W, we can have 64-bit immediate
        let (imm, imm_size): (i64, usize) = if prefix.rex_w() {
            if bytes.len() < 8 {
                return Err(LiftError::Incomplete {
                    addr: pc,
                    have: bytes.len(),
                    need: 8,
                });
            }
            (
                i64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]),
                8,
            )
        } else {
            match op_size {
                2 => {
                    if bytes.len() < 2 {
                        return Err(LiftError::Incomplete {
                            addr: pc,
                            have: bytes.len(),
                            need: 2,
                        });
                    }
                    (i16::from_le_bytes([bytes[0], bytes[1]]) as i64, 2)
                }
                _ => {
                    if bytes.len() < 4 {
                        return Err(LiftError::Incomplete {
                            addr: pc,
                            have: bytes.len(),
                            need: 4,
                        });
                    }
                    (
                        i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64,
                        4,
                    )
                }
            }
        };

        let src = if prefix.rex_w() {
            SrcOperand::Imm64(imm)
        } else {
            SrcOperand::Imm(imm)
        };

        let ops = vec![SmirOp::new(
            OpId(0),
            pc,
            OpKind::Mov {
                dst: self.gpr(reg),
                src,
                width,
            },
        )];

        Ok(LiftResult::fallthrough(ops, prefix.cursor + imm_size))
    }

    /// Lift MOV r8, imm8 (B0-B7)
    fn lift_mov_r8_imm8(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let reg = (opcode & 0x07) | prefix.rex_b();

        if bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: 0,
                need: 1,
            });
        }

        let imm = bytes[0] as i8 as i64;

        let ops = vec![SmirOp::new(
            OpId(0),
            pc,
            OpKind::Mov {
                dst: self.gpr(reg),
                src: SrcOperand::Imm(imm),
                width: OpWidth::W8,
            },
        )];

        Ok(LiftResult::fallthrough(ops, prefix.cursor + 1))
    }

    /// Lift PUSH r64 (50-57)
    fn lift_push_r64(
        &self,
        opcode: u8,
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let reg = (opcode & 0x07) | prefix.rex_b();
        let mut ops = Vec::new();

        // RSP -= 8
        ops.push(SmirOp::new(
            OpId(0),
            pc,
            OpKind::Sub {
                dst: self.rsp(),
                src1: self.rsp(),
                src2: SrcOperand::Imm(8),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        // [RSP] = reg
        ops.push(SmirOp::new(
            OpId(1),
            pc,
            OpKind::Store {
                src: self.gpr(reg),
                addr: Address::Direct(self.rsp()),
                width: MemWidth::B8,
            },
        ));

        Ok(LiftResult::fallthrough(ops, prefix.cursor))
    }

    /// Lift POP r64 (58-5F)
    fn lift_pop_r64(
        &self,
        opcode: u8,
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let reg = (opcode & 0x07) | prefix.rex_b();
        let mut ops = Vec::new();

        // reg = [RSP]
        ops.push(SmirOp::new(
            OpId(0),
            pc,
            OpKind::Load {
                dst: self.gpr(reg),
                addr: Address::Direct(self.rsp()),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            },
        ));

        // RSP += 8
        ops.push(SmirOp::new(
            OpId(1),
            pc,
            OpKind::Add {
                dst: self.rsp(),
                src1: self.rsp(),
                src2: SrcOperand::Imm(8),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        Ok(LiftResult::fallthrough(ops, prefix.cursor))
    }

    /// Lift CWD/CDQ/CQO (99)
    fn lift_cwd_cdq_cqo(&self, prefix: &X86Prefix, pc: u64) -> Result<LiftResult, LiftError> {
        let width = self.size_to_width(prefix.op_size());
        let ops = vec![SmirOp::new(
            OpId(0),
            pc,
            OpKind::Cwd {
                dst: self.gpr(2),
                src: self.gpr(0),
                width,
            },
        )];

        Ok(LiftResult::fallthrough(ops, prefix.cursor))
    }

    /// Lift XCHG rax, r64 (90-97)
    fn lift_xchg_rax(
        &self,
        opcode: u8,
        prefix: &X86Prefix,
        pc: u64,
    ) -> Result<LiftResult, LiftError> {
        let reg = (opcode & 0x07) | prefix.rex_b();
        let width = self.size_to_width(prefix.op_size());
        let ops = vec![SmirOp::new(
            OpId(0),
            pc,
            OpKind::Xchg {
                reg1: self.gpr(0),
                reg2: self.gpr(reg),
                width,
            },
        )];

        Ok(LiftResult::fallthrough(ops, prefix.cursor))
    }

    /// Lift BSWAP r32/r64 (0F C8+rd)
    fn lift_bswap_opcode(
        &self,
        opcode: u8,
        prefix: &X86Prefix,
        pc: u64,
    ) -> Result<LiftResult, LiftError> {
        if prefix.operand_size_override && !prefix.rex_w() {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: vec![opcode],
            });
        }

        let reg = (opcode & 0x07) | prefix.rex_b();
        let width = if prefix.rex_w() {
            OpWidth::W64
        } else {
            OpWidth::W32
        };
        let ops = vec![SmirOp::new(
            OpId(0),
            pc,
            OpKind::Bswap {
                dst: self.gpr(reg),
                src: self.gpr(reg),
                width,
            },
        )];

        Ok(LiftResult::fallthrough(ops, prefix.cursor))
    }

    /// Lift PUSH imm8/imm32 (6A/68)
    fn lift_push_imm(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
    ) -> Result<LiftResult, LiftError> {
        let imm_size = if opcode == 0x6A { 1 } else { 4 };
        if bytes.len() < imm_size {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: imm_size,
            });
        }

        let imm = if imm_size == 1 {
            bytes[0] as i8 as i64
        } else {
            i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64
        };

        let hint = if imm_size == 1 {
            X86OpHint::PushImm8
        } else {
            X86OpHint::PushImm32
        };

        let mut ops = Vec::new();
        ops.push(SmirOp::new(
            OpId(0),
            pc,
            OpKind::Sub {
                dst: self.rsp(),
                src1: self.rsp(),
                src2: SrcOperand::Imm(8),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        ops.push(SmirOp::with_hint(
            OpId(1),
            pc,
            OpKind::Store {
                src: VReg::Imm(imm),
                addr: Address::Direct(self.rsp()),
                width: MemWidth::B8,
            },
            hint,
        ));

        Ok(LiftResult::fallthrough(ops, prefix.cursor + imm_size))
    }

    /// Lift CALL rel32 (E8)
    fn lift_call_rel32(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.len() < 4 {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: 4,
            });
        }

        let rel = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64;
        let insn_len = prefix.cursor + 4;
        let next_rip = pc + insn_len as u64;
        let target = (next_rip as i64 + rel) as u64;

        Ok(LiftResult {
            ops: vec![],
            bytes_consumed: insn_len,
            control_flow: ControlFlow::Call {
                target: CallTarget::GuestAddr(target),
            },
            branch_targets: vec![target],
        })
    }

    /// Lift RET (C3)
    fn lift_ret(
        &self,
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let mut ops = Vec::new();
        let ret_addr = ctx.alloc_vreg();

        // Load return address
        ops.push(SmirOp::new(
            OpId(0),
            pc,
            OpKind::Load {
                dst: ret_addr,
                addr: Address::Direct(self.rsp()),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            },
        ));

        // RSP += 8
        ops.push(SmirOp::new(
            OpId(1),
            pc,
            OpKind::Add {
                dst: self.rsp(),
                src1: self.rsp(),
                src2: SrcOperand::Imm(8),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        Ok(LiftResult::ret(ops, prefix.cursor))
    }

    /// Lift RET imm16 (C2)
    fn lift_ret_imm16(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.len() < 2 {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: 2,
            });
        }

        let imm = u16::from_le_bytes([bytes[0], bytes[1]]) as i64;
        let mut ops = Vec::new();
        let ret_addr = ctx.alloc_vreg();

        ops.push(SmirOp::new(
            OpId(0),
            pc,
            OpKind::Load {
                dst: ret_addr,
                addr: Address::Direct(self.rsp()),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            },
        ));

        ops.push(SmirOp::new(
            OpId(1),
            pc,
            OpKind::Add {
                dst: self.rsp(),
                src1: self.rsp(),
                src2: SrcOperand::Imm(8 + imm),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        Ok(LiftResult::ret(ops, prefix.cursor + 2))
    }

    /// Lift LEAVE (C9)
    fn lift_leave(&self, prefix: &X86Prefix, pc: u64) -> Result<LiftResult, LiftError> {
        let ops = vec![SmirOp::new(OpId(0), pc, OpKind::Leave)];
        Ok(LiftResult::fallthrough(ops, prefix.cursor))
    }

    /// Lift JMP rel8 (EB)
    fn lift_jmp_rel8(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: 0,
                need: 1,
            });
        }

        let rel = bytes[0] as i8 as i64;
        let insn_len = prefix.cursor + 1;
        let target = (pc as i64 + insn_len as i64 + rel) as u64;

        Ok(LiftResult::branch(vec![], insn_len, target))
    }

    /// Lift JMP rel32 (E9)
    fn lift_jmp_rel32(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.len() < 4 {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: 4,
            });
        }

        let rel = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64;
        let insn_len = prefix.cursor + 4;
        let target = (pc as i64 + insn_len as i64 + rel) as u64;

        Ok(LiftResult::branch(vec![], insn_len, target))
    }

    /// Lift APX JMPABS imm64 (REX2 + A1).
    fn lift_jmp_abs(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
    ) -> Result<LiftResult, LiftError> {
        if bytes.len() < 8 {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: 8,
            });
        }

        let target = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        Ok(LiftResult::branch(vec![], prefix.cursor + 8, target))
    }

    /// Lift Jcc rel8 (70-7F)
    fn lift_jcc_rel8(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: 0,
                need: 1,
            });
        }

        let cc = opcode & 0x0F;
        let cond = self.x86_cond(cc);
        let rel = bytes[0] as i8 as i64;
        let insn_len = prefix.cursor + 1;
        let next_pc = pc + insn_len as u64;
        let target = (next_pc as i64 + rel) as u64;

        Ok(LiftResult::cond_branch(
            vec![],
            insn_len,
            cond,
            target,
            next_pc,
        ))
    }

    /// Lift LEA (8D)
    fn lift_lea(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let modrm = decode_modrm(bytes, prefix, pc)?;

        if !modrm.is_memory {
            return Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            });
        }

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        // LEA computes the effective ADDRESS — the segment OFFSET — and IGNORES
        // any segment override: `lea rax, fs:[rbx]` yields rbx, NOT fs_base+rbx
        // (LEA performs no memory access, so the segment base never applies).
        // Strip the override before lowering, else x86_addr_to_smir would emit a
        // SegmentRel that wrongly adds the FS/GS base.
        let mut lea_addr = modrm.addr.as_ref().unwrap().clone();
        lea_addr.segment = None;
        let (addr, mut ops) = self.x86_addr_to_smir(&lea_addr, next_pc, ctx);

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Lea {
                dst: self.gpr(modrm.reg),
                addr,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift STOS/REP STOS (AA/AB with optional REP prefix)
    fn lift_stos(
        &self,
        opcode: u8,
        prefix: &X86Prefix,
        pc: u64,
        _ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let width = match opcode {
            0xAA => MemWidth::B1,
            0xAB => self.size_to_memwidth(prefix.op_size()),
            _ => MemWidth::B1,
        };

        if prefix.rep_prefix == Some(0xF3) {
            let ops = vec![SmirOp::new(
                OpId(0),
                pc,
                OpKind::RepStos {
                    dst: self.gpr(7),
                    src: self.gpr(0),
                    count: self.gpr(1),
                    width,
                },
            )];

            Ok(LiftResult::fallthrough(ops, prefix.cursor))
        } else if self.strict {
            Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "stos".to_string(),
            })
        } else {
            Ok(LiftResult::fallthrough(vec![], prefix.cursor))
        }
    }

    /// Lift MOVS/REP MOVS (A4/A5 with optional REP prefix)
    fn lift_movs(&self, opcode: u8, prefix: &X86Prefix, pc: u64) -> Result<LiftResult, LiftError> {
        let width = match opcode {
            0xA4 => MemWidth::B1,
            0xA5 => self.size_to_memwidth(prefix.op_size()),
            _ => MemWidth::B1,
        };

        if prefix.rep_prefix.is_some() {
            let ops = vec![SmirOp::new(
                OpId(0),
                pc,
                OpKind::RepMovs {
                    dst: self.gpr(7),
                    src: self.gpr(6),
                    count: self.gpr(1),
                    width,
                },
            )];

            Ok(LiftResult::fallthrough(ops, prefix.cursor))
        } else if self.strict {
            Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: "movs".to_string(),
            })
        } else {
            Ok(LiftResult::fallthrough(vec![], prefix.cursor))
        }
    }

    /// Lift IN imm8 or DX (E4/E5/EC/ED)
    fn lift_in(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
    ) -> Result<LiftResult, LiftError> {
        let (port, width, imm_len) = match opcode {
            0xE4 => {
                if bytes.is_empty() {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: 0,
                        need: 1,
                    });
                }
                (VReg::Imm(bytes[0] as i8 as i64), MemWidth::B1, 1)
            }
            0xE5 => {
                if bytes.is_empty() {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: 0,
                        need: 1,
                    });
                }
                let width = if prefix.operand_size_override {
                    MemWidth::B2
                } else {
                    MemWidth::B4
                };
                (VReg::Imm(bytes[0] as i8 as i64), width, 1)
            }
            0xEC => (self.gpr(2), MemWidth::B1, 0),
            0xED => {
                let width = if prefix.operand_size_override {
                    MemWidth::B2
                } else {
                    MemWidth::B4
                };
                (self.gpr(2), width, 0)
            }
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr: pc,
                    bytes: bytes.to_vec(),
                });
            }
        };

        let ops = vec![SmirOp::new(
            OpId(0),
            pc,
            OpKind::IoIn {
                dst: self.gpr(0),
                port,
                width,
            },
        )];

        Ok(LiftResult::fallthrough(ops, prefix.cursor + imm_len))
    }

    /// Lift OUT imm8 or DX (E6/E7/EE/EF)
    fn lift_out(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
    ) -> Result<LiftResult, LiftError> {
        let (port, width, imm_len) = match opcode {
            0xE6 => {
                if bytes.is_empty() {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: 0,
                        need: 1,
                    });
                }
                (VReg::Imm(bytes[0] as i8 as i64), MemWidth::B1, 1)
            }
            0xE7 => {
                if bytes.is_empty() {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: 0,
                        need: 1,
                    });
                }
                let width = if prefix.operand_size_override {
                    MemWidth::B2
                } else {
                    MemWidth::B4
                };
                (VReg::Imm(bytes[0] as i8 as i64), width, 1)
            }
            0xEE => (self.gpr(2), MemWidth::B1, 0),
            0xEF => {
                let width = if prefix.operand_size_override {
                    MemWidth::B2
                } else {
                    MemWidth::B4
                };
                (self.gpr(2), width, 0)
            }
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr: pc,
                    bytes: bytes.to_vec(),
                });
            }
        };

        let ops = vec![SmirOp::new(
            OpId(0),
            pc,
            OpKind::IoOut {
                port,
                value: self.gpr(0),
                width,
            },
        )];

        Ok(LiftResult::fallthrough(ops, prefix.cursor + imm_len))
    }

    /// Lift NOP (90)
    fn lift_nop(&self, prefix: &X86Prefix, _pc: u64) -> Result<LiftResult, LiftError> {
        Ok(LiftResult::fallthrough(vec![], prefix.cursor))
    }

    /// Lift MOV r/m, r and MOV r, r/m (88-8B)
    fn lift_mov_rm_r(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_8bit = (opcode & 0x01) == 0;
        let dir_reg_rm = (opcode & 0x02) != 0; // true = reg is src, rm is dst

        let op_size = if is_8bit { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            if dir_reg_rm {
                // MOV r, rm - load from memory
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Load {
                        dst: self.gpr(modrm.reg),
                        addr,
                        width: mem_width,
                        sign: SignExtend::Zero,
                    },
                ));
            } else {
                // MOV rm, r - store to memory
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: self.gpr(modrm.reg),
                        addr,
                        width: mem_width,
                    },
                ));
            }
        } else {
            // Register to register
            let (dst, src) = if dir_reg_rm {
                (self.gpr(modrm.reg), self.gpr(modrm.rm))
            } else {
                (self.gpr(modrm.rm), self.gpr(modrm.reg))
            };

            ops.push(SmirOp::new(
                OpId(0),
                pc,
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Reg(src),
                    width,
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift MOVSXD r64, r/m32 (63)
    fn lift_movsxd(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: MemWidth::B4,
                    sign: SignExtend::Sign,
                },
            ));
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::SignExtend {
                    dst: self.gpr(modrm.reg),
                    src: tmp,
                    from_width: OpWidth::W32,
                    to_width: OpWidth::W64,
                },
            ));
        } else {
            ops.push(SmirOp::new(
                OpId(0),
                pc,
                OpKind::SignExtend {
                    dst: self.gpr(modrm.reg),
                    src: self.gpr(modrm.rm),
                    from_width: OpWidth::W32,
                    to_width: OpWidth::W64,
                },
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift MOV r/m, imm (C6/C7)
    fn lift_mov_rm_imm(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_8bit = opcode == 0xC6;
        let op_size = if is_8bit { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);
        let mem_width = self.size_to_memwidth(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let group = (modrm.byte >> 3) & 0x07;
        if group != 0 {
            if self.strict {
                return Err(LiftError::Unsupported {
                    addr: pc,
                    mnemonic: format!("mov group {}", group),
                });
            }
            return Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                prefix.cursor + modrm.bytes_consumed,
            ));
        }

        let imm_offset = modrm.bytes_consumed;
        let imm_size = if is_8bit {
            1
        } else if op_size == 2 {
            2
        } else {
            4
        };
        if bytes.len() < imm_offset + imm_size {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: imm_offset + imm_size,
            });
        }

        let imm = match imm_size {
            1 => bytes[imm_offset] as i8 as i64,
            2 => i16::from_le_bytes([bytes[imm_offset], bytes[imm_offset + 1]]) as i64,
            _ => i32::from_le_bytes([
                bytes[imm_offset],
                bytes[imm_offset + 1],
                bytes[imm_offset + 2],
                bytes[imm_offset + 3],
            ]) as i64,
        };

        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64 + imm_size as u64;
        let mut ops = Vec::new();
        let hint = X86OpHint::MovImmModRm;

        if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            ops.push(SmirOp::with_hint(
                OpId(ops.len() as u16),
                pc,
                OpKind::Store {
                    src: VReg::Imm(imm),
                    addr,
                    width: mem_width,
                },
                hint,
            ));
        } else {
            ops.push(SmirOp::with_hint(
                OpId(ops.len() as u16),
                pc,
                OpKind::Mov {
                    dst: self.gpr(modrm.rm),
                    src: SrcOperand::Imm(imm),
                    width,
                },
                hint,
            ));
        }

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed + imm_size,
        ))
    }

    /// Lift TEST r/m, r (84/85)
    fn lift_test_rm_r(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_8bit = (opcode & 0x01) == 0;
        let op_size = if is_8bit { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;

        let (src1, src2) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: tmp,
                    addr,
                    width: self.size_to_memwidth(op_size),
                    sign: SignExtend::Zero,
                },
            ));
            (tmp, self.gpr(modrm.reg))
        } else {
            (self.gpr(modrm.rm), self.gpr(modrm.reg))
        };

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Test {
                src1,
                src2: SrcOperand::Reg(src2),
                width,
            },
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift XOR r/m, r and XOR r, r/m (30-33)
    fn lift_xor_rm_r(
        &self,
        opcode: u8,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        let is_8bit = (opcode & 0x01) == 0;
        let dir_reg_rm = (opcode & 0x02) != 0;

        let op_size = if is_8bit { 1 } else { prefix.op_size() };
        let width = self.size_to_width(op_size);

        let modrm = decode_modrm(bytes, prefix, pc)?;
        let mut ops = Vec::new();
        let next_pc = pc + prefix.cursor as u64 + modrm.bytes_consumed as u64;
        let hint = X86OpHint::AluEncoding(if dir_reg_rm {
            X86AluEncoding::RegRm
        } else {
            X86AluEncoding::RmReg
        });

        let (dst, src1, src2) = if modrm.is_memory {
            let x86_addr = modrm.addr.as_ref().unwrap();
            let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
            ops.extend(pre_ops);

            if dir_reg_rm {
                // XOR r, rm
                let tmp = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Load {
                        dst: tmp,
                        addr,
                        width: self.size_to_memwidth(op_size),
                        sign: SignExtend::Zero,
                    },
                ));
                (self.gpr(modrm.reg), self.gpr(modrm.reg), tmp)
            } else {
                // XOR rm, r - load-modify-store
                let tmp = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Load {
                        dst: tmp,
                        addr: addr.clone(),
                        width: self.size_to_memwidth(op_size),
                        sign: SignExtend::Zero,
                    },
                ));
                ops.push(SmirOp::with_hint(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Xor {
                        dst: tmp,
                        src1: tmp,
                        src2: SrcOperand::Reg(self.gpr(modrm.reg)),
                        width,
                        flags: FlagUpdate::All,
                    },
                    hint,
                ));
                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Store {
                        src: tmp,
                        addr,
                        width: self.size_to_memwidth(op_size),
                    },
                ));
                return Ok(LiftResult::fallthrough(
                    ops,
                    prefix.cursor + modrm.bytes_consumed,
                ));
            }
        } else if dir_reg_rm {
            (self.gpr(modrm.reg), self.gpr(modrm.reg), self.gpr(modrm.rm))
        } else {
            (self.gpr(modrm.rm), self.gpr(modrm.rm), self.gpr(modrm.reg))
        };

        let result = dst;
        ops.push(SmirOp::with_hint(
            OpId(ops.len() as u16),
            pc,
            OpKind::Xor {
                dst: result,
                src1,
                src2: SrcOperand::Reg(src2),
                width,
                flags: FlagUpdate::All,
            },
            hint,
        ));

        Ok(LiftResult::fallthrough(
            ops,
            prefix.cursor + modrm.bytes_consumed,
        ))
    }

    /// Lift the main instruction
    fn lift_insn_inner(
        &self,
        pc: u64,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: 0,
                need: 1,
            });
        }

        if bytes[0] == 0x62 {
            let is_apx_map4 = bytes.get(1).map_or(false, |p0| (p0 & 0x07) == 4);
            if is_apx_map4 || bytes.len() < 2 {
                return self.lift_apx_evex_map4(pc, bytes, ctx);
            }
            return self.lift_vex_evex(pc, bytes, ctx);
        }

        if matches!(bytes[0], 0xC4 | 0xC5) {
            return self.lift_vex_evex(pc, bytes, ctx);
        }

        // Decode prefixes
        let prefix = decode_prefixes(bytes)?;
        let opcode_bytes = &bytes[prefix.cursor..];

        if opcode_bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: bytes.len(),
                need: prefix.cursor + 1,
            });
        }

        if prefix.rex2_m() {
            return self.lift_0f_opcode(opcode_bytes, &prefix, pc, ctx, 1);
        }

        let opcode = opcode_bytes[0];
        let after_opcode = &opcode_bytes[1..];

        match opcode {
            // XCHG rax, r64 / NOP / PAUSE (with REP prefix)
            0x90..=0x97 => {
                if opcode == 0x90 && prefix.rep_prefix == Some(0xF3) {
                    // PAUSE - treat as NOP for lifting
                    Ok(LiftResult::fallthrough(vec![], prefix.cursor + 1))
                } else {
                    self.lift_xchg_rax(
                        opcode,
                        &X86Prefix {
                            cursor: prefix.cursor + 1,
                            ..prefix
                        },
                        pc,
                    )
                }
            }

            // CMC/CLC/STC
            0xF5 => Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::CmcCF)],
                prefix.cursor + 1,
            )),
            0xF8 => Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::SetCF { value: false })],
                prefix.cursor + 1,
            )),
            0xF9 => Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::SetCF { value: true })],
                prefix.cursor + 1,
            )),
            0xFC => Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::SetDF { value: false })],
                prefix.cursor + 1,
            )),
            0xFD => Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::SetDF { value: true })],
                prefix.cursor + 1,
            )),
            0xCC => Ok(LiftResult::fallthrough(
                vec![SmirOp::new(OpId(0), pc, OpKind::Breakpoint)],
                prefix.cursor + 1,
            )),

            // HLT
            0xF4 => Ok(LiftResult {
                ops: vec![],
                bytes_consumed: prefix.cursor + 1,
                control_flow: ControlFlow::Trap {
                    kind: TrapKind::Halt,
                },
                branch_targets: vec![],
            }),

            // Two-byte opcode prefix
            0x0F => self.lift_0f_opcode(after_opcode, &prefix, pc, ctx, 2),

            // Control flow
            0xEB => self.lift_jmp_rel8(
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xE9 => self.lift_jmp_rel32(
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xE8 => self.lift_call_rel32(
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xC2 => self.lift_ret_imm16(
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xC3 => self.lift_ret(
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xC9 => self.lift_leave(
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
            ),
            0x99 => self.lift_cwd_cdq_cqo(
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
            ),
            0x70..=0x7F => self.lift_jcc_rel8(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xA1 if prefix.rex2.is_some() => self.lift_jmp_abs(
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
            ),

            // Data movement
            0xB0..=0xB7 => self.lift_mov_r8_imm8(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xB8..=0xBF => self.lift_mov_r_imm(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x88..=0x8B => self.lift_mov_rm_r(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x86 | 0x87 => self.lift_xchg_rm_r(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xC6 | 0xC7 => self.lift_mov_rm_imm(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x8D => self.lift_lea(
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x63 => self.lift_movsxd(
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x50..=0x57 => self.lift_push_r64(
                opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x58..=0x5F => self.lift_pop_r64(
                opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x6A | 0x68 => self.lift_push_imm(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
            ),
            0xF6 | 0xF7 => self.lift_group3(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x69 | 0x6B => self.lift_imul_rmi(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),

            // Arithmetic
            0x00..=0x05 => self.lift_arith(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ), // ADD
            0x08..=0x0D => self.lift_arith(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ), // OR
            0x10..=0x15 => self.lift_arith(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ), // ADC
            0x18..=0x1D => self.lift_arith(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ), // SBB
            0x20..=0x25 => self.lift_arith(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ), // AND
            0x28..=0x2D => self.lift_arith(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ), // SUB
            0x30..=0x33 => self.lift_xor_rm_r(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0x38..=0x3D => self.lift_arith(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ), // CMP

            // Group 1 immediate (80/81/83)
            0x80 | 0x81 | 0x83 => self.lift_group1_imm(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),

            // Logic
            0x84 | 0x85 => self.lift_test_rm_r(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),

            // String ops
            0xAA | 0xAB => self.lift_stos(
                opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),
            0xA4 | 0xA5 => self.lift_movs(
                opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
            ),

            // Shift/rotate group (C0/C1) - immediate
            0xC0 | 0xC1 => self.lift_shift_imm(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),

            // Shift/rotate group (D0/D1) - count = 1
            0xD0 | 0xD1 => self.lift_shift_one(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),

            // Shift/rotate group (D2/D3) - count in CL
            0xD2 | 0xD3 => self.lift_shift_cl(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),

            // Group 5 (FF)
            0xFF => self.lift_group5(
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
                ctx,
            ),

            // I/O port instructions
            0xE4 | 0xE5 | 0xEC | 0xED => self.lift_in(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
            ),
            0xE6 | 0xE7 | 0xEE | 0xEF => self.lift_out(
                opcode,
                after_opcode,
                &X86Prefix {
                    cursor: prefix.cursor + 1,
                    ..prefix
                },
                pc,
            ),

            // Unsupported - return error with mnemonic
            _ => {
                if self.strict {
                    Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("0x{:02X}", opcode),
                    })
                } else {
                    // In non-strict mode, emit a Nop and continue
                    Ok(LiftResult::fallthrough(
                        vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                        prefix.cursor + 1,
                    ))
                }
            }
        }
    }

    /// Lift 0F-prefixed (two-byte) opcodes
    fn lift_0f_opcode(
        &self,
        bytes: &[u8],
        prefix: &X86Prefix,
        pc: u64,
        ctx: &mut LiftContext,
        map_len: usize,
    ) -> Result<LiftResult, LiftError> {
        if bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr: pc,
                have: prefix.cursor + map_len.saturating_sub(1),
                need: prefix.cursor + map_len,
            });
        }

        let opcode2 = bytes[0];
        let after_opcode = &bytes[1..];
        let prefix2 = X86Prefix {
            cursor: prefix.cursor + map_len,
            ..prefix.clone()
        };

        match opcode2 {
            // Jcc rel32 (0F 80 - 0F 8F)
            0x80..=0x8F => {
                if after_opcode.len() < 4 {
                    return Err(LiftError::Incomplete {
                        addr: pc,
                        have: prefix2.cursor + after_opcode.len(),
                        need: prefix2.cursor + 4,
                    });
                }

                let cc = opcode2 & 0x0F;
                let cond = self.x86_cond(cc);
                let rel = i32::from_le_bytes([
                    after_opcode[0],
                    after_opcode[1],
                    after_opcode[2],
                    after_opcode[3],
                ]) as i64;

                let insn_len = prefix2.cursor + 4;
                let next_pc = pc + insn_len as u64;
                let target = (next_pc as i64 + rel) as u64;

                Ok(LiftResult::cond_branch(
                    vec![],
                    insn_len,
                    cond,
                    target,
                    next_pc,
                ))
            }

            // SETcc (0F 90 - 0F 9F)
            0x90..=0x9F => {
                let cc = opcode2 & 0x0F;
                let cond = self.x86_cond(cc);

                let modrm = decode_modrm(after_opcode, &prefix2, pc)?;
                let mut ops = Vec::new();
                let next_pc = pc + prefix2.cursor as u64 + modrm.bytes_consumed as u64;

                if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::SetCC {
                            dst: tmp,
                            cond,
                            width: OpWidth::W8,
                        },
                    ));
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Store {
                            src: tmp,
                            addr,
                            width: MemWidth::B1,
                        },
                    ));
                } else {
                    ops.push(SmirOp::new(
                        OpId(0),
                        pc,
                        OpKind::SetCC {
                            dst: self.gpr(modrm.rm),
                            cond,
                            width: OpWidth::W8,
                        },
                    ));
                }

                Ok(LiftResult::fallthrough(
                    ops,
                    prefix2.cursor + modrm.bytes_consumed,
                ))
            }

            // CMOVcc (0F 40 - 0F 4F)
            0x40..=0x4F => {
                let cc = opcode2 & 0x0F;
                let cond = self.x86_cond(cc);
                let op_size = prefix.op_size();
                let width = self.size_to_width(op_size);

                let modrm = decode_modrm(after_opcode, &prefix2, pc)?;
                let mut ops = Vec::new();
                let next_pc = pc + prefix2.cursor as u64 + modrm.bytes_consumed as u64;

                let src = if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Load {
                            dst: tmp,
                            addr,
                            width: self.size_to_memwidth(op_size),
                            sign: SignExtend::Zero,
                        },
                    ));
                    tmp
                } else {
                    self.gpr(modrm.rm)
                };

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::CMove {
                        dst: self.gpr(modrm.reg),
                        src,
                        cond,
                        width,
                    },
                ));

                Ok(LiftResult::fallthrough(
                    ops,
                    prefix2.cursor + modrm.bytes_consumed,
                ))
            }

            // SSE MOVDQA/MOVDQU (0F 6F/7F)
            0x6F | 0x7F => self.lift_sse_mov(opcode2, after_opcode, &prefix2, pc, ctx),

            // SSE PADDD (66 0F FE)
            0xFE => self.lift_sse_padd(opcode2, after_opcode, &prefix2, pc, ctx),

            // SSE4.1 opcodes (0F 38)
            0x38 if prefix2.rex2_m() => Err(LiftError::InvalidEncoding {
                addr: pc,
                bytes: bytes.to_vec(),
            }),
            0x38 => self.lift_0f38_opcode(after_opcode, &prefix2, pc, ctx),

            // SHLD/SHRD (0F A4/A5/AC/AD)
            0xA4 | 0xA5 | 0xAC | 0xAD => {
                self.lift_shld_shrd(opcode2, after_opcode, &prefix2, pc, ctx)
            }

            // CMPXCHG r/m, r (0F B0/0F B1)
            0xB0 | 0xB1 => self.lift_cmpxchg(opcode2, after_opcode, &prefix2, pc, ctx),

            // XADD r/m, r (0F C0/0F C1)
            0xC0 | 0xC1 => self.lift_xadd(opcode2, after_opcode, &prefix2, pc, ctx),

            // BSWAP r32/r64 (0F C8+rd)
            0xC8..=0xCF => self.lift_bswap_opcode(opcode2, &prefix2, pc),

            // MOVZX r, r/m8 (0F B6)
            0xB6 => {
                let op_size = prefix.op_size();
                let modrm = decode_modrm(after_opcode, &prefix2, pc)?;
                let mut ops = Vec::new();
                let next_pc = pc + prefix2.cursor as u64 + modrm.bytes_consumed as u64;

                let src = if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Load {
                            dst: tmp,
                            addr,
                            width: MemWidth::B1,
                            sign: SignExtend::Zero,
                        },
                    ));
                    tmp
                } else {
                    self.gpr(modrm.rm)
                };

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::ZeroExtend {
                        dst: self.gpr(modrm.reg),
                        src,
                        from_width: OpWidth::W8,
                        to_width: self.size_to_width(op_size),
                    },
                ));

                Ok(LiftResult::fallthrough(
                    ops,
                    prefix2.cursor + modrm.bytes_consumed,
                ))
            }

            // MOVZX r, r/m16 (0F B7)
            0xB7 => {
                let op_size = prefix.op_size();
                let modrm = decode_modrm(after_opcode, &prefix2, pc)?;
                let mut ops = Vec::new();
                let next_pc = pc + prefix2.cursor as u64 + modrm.bytes_consumed as u64;

                let src = if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Load {
                            dst: tmp,
                            addr,
                            width: MemWidth::B2,
                            sign: SignExtend::Zero,
                        },
                    ));
                    tmp
                } else {
                    self.gpr(modrm.rm)
                };

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::ZeroExtend {
                        dst: self.gpr(modrm.reg),
                        src,
                        from_width: OpWidth::W16,
                        to_width: self.size_to_width(op_size),
                    },
                ));

                Ok(LiftResult::fallthrough(
                    ops,
                    prefix2.cursor + modrm.bytes_consumed,
                ))
            }

            // BSF/BSR (0F BC/0F BD)
            0xBC | 0xBD => self.lift_bsf_bsr(opcode2, after_opcode, &prefix2, pc, ctx),

            // MOVSX r, r/m8 (0F BE)
            0xBE => {
                let op_size = prefix.op_size();
                let modrm = decode_modrm(after_opcode, &prefix2, pc)?;
                let mut ops = Vec::new();
                let next_pc = pc + prefix2.cursor as u64 + modrm.bytes_consumed as u64;

                let src = if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Load {
                            dst: tmp,
                            addr,
                            width: MemWidth::B1,
                            sign: SignExtend::Sign,
                        },
                    ));
                    tmp
                } else {
                    self.gpr(modrm.rm)
                };

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::SignExtend {
                        dst: self.gpr(modrm.reg),
                        src,
                        from_width: OpWidth::W8,
                        to_width: self.size_to_width(op_size),
                    },
                ));

                Ok(LiftResult::fallthrough(
                    ops,
                    prefix2.cursor + modrm.bytes_consumed,
                ))
            }

            // MOVSX r, r/m16 (0F BF)
            0xBF => {
                let op_size = prefix.op_size();
                let modrm = decode_modrm(after_opcode, &prefix2, pc)?;
                let mut ops = Vec::new();
                let next_pc = pc + prefix2.cursor as u64 + modrm.bytes_consumed as u64;

                let src = if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Load {
                            dst: tmp,
                            addr,
                            width: MemWidth::B2,
                            sign: SignExtend::Sign,
                        },
                    ));
                    tmp
                } else {
                    self.gpr(modrm.rm)
                };

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::SignExtend {
                        dst: self.gpr(modrm.reg),
                        src,
                        from_width: OpWidth::W16,
                        to_width: self.size_to_width(op_size),
                    },
                ));

                Ok(LiftResult::fallthrough(
                    ops,
                    prefix2.cursor + modrm.bytes_consumed,
                ))
            }

            // IMUL r, r/m (0F AF)
            0xAF => {
                let op_size = prefix.op_size();
                let width = self.size_to_width(op_size);
                let modrm = decode_modrm(after_opcode, &prefix2, pc)?;
                let mut ops = Vec::new();
                let next_pc = pc + prefix2.cursor as u64 + modrm.bytes_consumed as u64;

                let src = if modrm.is_memory {
                    let x86_addr = modrm.addr.as_ref().unwrap();
                    let (addr, pre_ops) = self.x86_addr_to_smir(x86_addr, next_pc, ctx);
                    ops.extend(pre_ops);

                    let tmp = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::Load {
                            dst: tmp,
                            addr,
                            width: self.size_to_memwidth(op_size),
                            sign: SignExtend::Zero,
                        },
                    ));
                    tmp
                } else {
                    self.gpr(modrm.rm)
                };

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::MulS {
                        dst_lo: self.gpr(modrm.reg),
                        dst_hi: None,
                        src1: self.gpr(modrm.reg),
                        src2: SrcOperand::Reg(src),
                        width,
                        flags: FlagUpdate::All,
                    },
                ));

                Ok(LiftResult::fallthrough(
                    ops,
                    prefix2.cursor + modrm.bytes_consumed,
                ))
            }

            // SYSCALL (0F 05)
            0x05 => Ok(LiftResult {
                ops: vec![],
                bytes_consumed: prefix2.cursor,
                control_flow: ControlFlow::Syscall,
                branch_targets: vec![],
            }),

            // SYSRET (0F 07)
            0x07 => {
                // Treat as return for lifting purposes
                Ok(LiftResult::ret(vec![], prefix2.cursor))
            }

            // NOP (0F 1F /0) - multi-byte NOP
            0x1F => {
                let modrm = decode_modrm(after_opcode, &prefix2, pc)?;
                Ok(LiftResult::fallthrough(
                    vec![],
                    prefix2.cursor + modrm.bytes_consumed,
                ))
            }

            _ => {
                if self.strict {
                    Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("0x0F 0x{:02X}", opcode2),
                    })
                } else {
                    Ok(LiftResult::fallthrough(
                        vec![SmirOp::new(OpId(0), pc, OpKind::Nop)],
                        prefix2.cursor,
                    ))
                }
            }
        }
    }
}

// ============================================================================
// SmirLifter Implementation
// ============================================================================

impl SmirLifter for X86_64Lifter {
    fn source_arch(&self) -> SourceArch {
        SourceArch::X86_64
    }

    fn lift_insn(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        ctx.guest_pc = addr;
        self.lift_insn_inner(addr, bytes, ctx)
    }

    fn lift_block(
        &mut self,
        addr: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirBlock, LiftError> {
        let block_id = ctx.get_or_create_block(addr);
        let mut block = SmirBlock::new(block_id, addr);

        let mut pc = addr;
        let mut buf = [0u8; 15];

        loop {
            // Read instruction bytes
            let bytes = mem
                .read(pc, 15)
                .map_err(|e| LiftError::MemoryError { addr: pc, error: e })?;

            buf[..bytes.len()].copy_from_slice(&bytes);

            ctx.guest_pc = pc;
            let result = self.lift_insn_inner(pc, &buf[..bytes.len()], ctx)?;

            // Add ops to block
            block.ops.extend(result.ops);
            pc += result.bytes_consumed as u64;

            // Check for block-ending control flow
            match result.control_flow {
                ControlFlow::Fallthrough | ControlFlow::NextInsn => continue,
                ControlFlow::Branch { target } | ControlFlow::DirectBranch(target) => {
                    block.terminator = Terminator::Branch {
                        target: ctx.get_or_create_block(target),
                    };
                    break;
                }
                ControlFlow::CondBranch {
                    cond,
                    target,
                    fallthrough,
                } => {
                    // We need a VReg holding the condition result
                    let cond_vreg = ctx.alloc_vreg();
                    block.ops.push(SmirOp::new(
                        OpId(block.ops.len() as u16),
                        pc,
                        OpKind::TestCondition {
                            dst: cond_vreg,
                            cond,
                        },
                    ));
                    block.terminator = Terminator::CondBranch {
                        cond: cond_vreg,
                        true_target: ctx.get_or_create_block(target),
                        false_target: ctx.get_or_create_block(fallthrough),
                    };
                    break;
                }
                ControlFlow::CondBranchReg {
                    cond,
                    taken,
                    not_taken,
                } => {
                    block.terminator = Terminator::CondBranch {
                        cond,
                        true_target: ctx.get_or_create_block(taken),
                        false_target: ctx.get_or_create_block(not_taken),
                    };
                    break;
                }
                ControlFlow::IndirectBranch { target } => {
                    block.terminator = Terminator::IndirectBranch {
                        target,
                        possible_targets: vec![],
                    };
                    break;
                }
                ControlFlow::IndirectBranchMem { addr } => {
                    block.terminator = Terminator::IndirectBranchMem {
                        addr,
                        possible_targets: vec![],
                    };
                    break;
                }
                ControlFlow::Call { target } => {
                    let continuation = ctx.get_or_create_block(pc);
                    block.terminator = Terminator::Call {
                        target,
                        args: vec![],
                        continuation,
                    };
                    break;
                }
                ControlFlow::Return => {
                    block.terminator = Terminator::Return { values: vec![] };
                    break;
                }
                ControlFlow::Trap { kind } => {
                    block.terminator = Terminator::Trap { kind };
                    break;
                }
                ControlFlow::Syscall => {
                    // For syscall, we'll use a TailCall to the syscall runtime
                    block.terminator = Terminator::TailCall {
                        target: CallTarget::Runtime(crate::smir::ir::RuntimeFunc::Syscall),
                        args: vec![],
                    };
                    break;
                }
            }
        }

        Ok(block)
    }

    fn lift_function(
        &mut self,
        entry: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirFunction, LiftError> {
        let entry_block = ctx.get_or_create_block(entry);
        let mut func = SmirFunction::new(FunctionId(entry as u32), entry_block, entry);

        // Work queue of blocks to lift
        let mut worklist = vec![entry];
        let mut visited = HashSet::new();

        while let Some(block_addr) = worklist.pop() {
            if visited.contains(&block_addr) {
                continue;
            }
            // Lift-through-calls: bound the lifted CFG so a large or call-chained
            // function can't lift unboundedly (the cap counts lifted blocks).
            if self.lift_through_calls && self.max_blocks != 0 && visited.len() >= self.max_blocks {
                break;
            }
            visited.insert(block_addr);

            let block = self.lift_block(block_addr, mem, ctx)?;

            // Add branch targets to worklist
            match &block.terminator {
                Terminator::Branch { target } => {
                    if let Some(&addr) = ctx
                        .block_cache
                        .iter()
                        .find_map(|(a, id)| if id == target { Some(a) } else { None })
                    {
                        worklist.push(addr);
                    }
                }
                Terminator::CondBranch {
                    true_target,
                    false_target,
                    ..
                } => {
                    for target in [true_target, false_target] {
                        if let Some(&addr) = ctx
                            .block_cache
                            .iter()
                            .find_map(|(a, id)| if id == target { Some(a) } else { None })
                        {
                            worklist.push(addr);
                        }
                    }
                }
                // Lift-through-calls: follow the CALL's continuation (the return
                // address) so the caller's CFG past the call is lifted. The call
                // target itself is NOT lifted — it runs in the interpreter via the
                // runtime call-out. TailCall has no continuation (it doesn't return).
                Terminator::Call { continuation, .. } if self.lift_through_calls => {
                    if let Some(&addr) = ctx
                        .block_cache
                        .iter()
                        .find_map(|(a, id)| if id == continuation { Some(a) } else { None })
                    {
                        worklist.push(addr);
                    }
                }
                _ => {}
            }

            func.add_block(block);
        }

        Ok(func)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test memory reader for unit tests
    struct TestMemory {
        data: Vec<u8>,
        base: u64,
    }

    impl TestMemory {
        fn new(base: u64, data: Vec<u8>) -> Self {
            TestMemory { data, base }
        }
    }

    impl MemoryReader for TestMemory {
        fn read(&self, addr: u64, size: usize) -> Result<Vec<u8>, MemoryError> {
            if addr < self.base {
                return Err(MemoryError::OutOfBounds { addr });
            }
            let offset = (addr - self.base) as usize;
            if offset >= self.data.len() {
                return Err(MemoryError::OutOfBounds { addr });
            }
            // Return as many bytes as possible up to size
            let available = (self.data.len() - offset).min(size);
            Ok(self.data[offset..offset + available].to_vec())
        }
    }

    #[test]
    fn test_prefix_decode() {
        // No prefix
        let prefix = decode_prefixes(&[0x90]).unwrap();
        assert_eq!(prefix.cursor, 0);
        assert!(!prefix.has_rex());

        // REX.W prefix
        let prefix = decode_prefixes(&[0x48, 0xB8]).unwrap();
        assert_eq!(prefix.cursor, 1);
        assert!(prefix.rex_w());
        assert_eq!(prefix.op_size(), 8);

        // Operand size override
        let prefix = decode_prefixes(&[0x66, 0xB8]).unwrap();
        assert_eq!(prefix.cursor, 1);
        assert!(prefix.operand_size_override);
        assert_eq!(prefix.op_size(), 2);

        // REX2.W with B high bit: LLVM encodes `mov r16, imm64` as d5 18 b8...
        let prefix = decode_prefixes(&[0xD5, 0x18, 0xB8]).unwrap();
        assert_eq!(prefix.cursor, 2);
        assert!(prefix.has_rex());
        assert!(prefix.rex_w());
        assert_eq!(prefix.rex_b(), 16);
        assert!(!prefix.rex2_m());

        // REX2.M compressed 0F map: LLVM encodes `imul r16, rax` as d5 c8 af c0.
        let prefix = decode_prefixes(&[0xD5, 0xC8, 0xAF]).unwrap();
        assert_eq!(prefix.cursor, 2);
        assert!(prefix.rex2_m());
        assert!(prefix.rex_w());
        assert_eq!(prefix.rex_r(), 16);
    }

    /// Lift one instruction (a trailing HLT terminates the block) and return its ops.
    fn lift_one(code: &[u8]) -> Result<Vec<SmirOp>, LiftError> {
        use crate::smir::lift::SmirLifter;
        let mut bytes = code.to_vec();
        bytes.push(0xF4); // hlt → block terminator
        let mem = TestMemory::new(0x1000, bytes);
        let mut lifter = X86_64Lifter::strict();
        let mut lctx = LiftContext::new(SourceArch::X86_64);
        lifter.lift_block(0x1000, &mem, &mut lctx).map(|b| b.ops)
    }

    /// LEA computes the segment OFFSET and must IGNORE a segment override —
    /// `lea rax, fs:[rbx]` yields rbx, so it must NOT lift to a SegmentRel that
    /// would add fs_base. (Regression for the segment-base-in-LEA bug.)
    #[test]
    fn lea_ignores_segment_override() {
        let ops = lift_one(&[0x64, 0x48, 0x8d, 0x03]).expect("lift lea fs:[rbx]"); // lea rax, fs:[rbx]
        let addr = ops
            .iter()
            .find_map(|o| match &o.kind {
                OpKind::Lea { addr, .. } => Some(addr),
                _ => None,
            })
            .expect("a Lea op");
        assert!(
            !matches!(addr, Address::SegmentRel { .. }),
            "LEA must NOT add the segment base (got {addr:?})"
        );
    }

    /// A genuine FS/GS LOAD, by contrast, DOES carry the segment base.
    #[test]
    fn mov_gs_load_produces_segmentrel() {
        let ops = lift_one(&[0x65, 0x48, 0x8b, 0x03]).expect("lift mov rax, gs:[rbx]"); // mov rax, gs:[rbx]
        let addr = ops
            .iter()
            .find_map(|o| match &o.kind {
                OpKind::Load { addr, .. } => Some(addr),
                _ => None,
            })
            .expect("a Load op");
        assert!(
            matches!(
                addr,
                Address::SegmentRel {
                    segment: VReg::Arch(ArchReg::X86(X86Reg::GsBase)),
                    ..
                }
            ),
            "mov gs:[rbx] must lift to SegmentRel{{GsBase}} (got {addr:?})"
        );
    }

    /// A 0x67 (32-bit address-size) memory operand is not modeled and must bail
    /// rather than silently mis-lift `[ebx]` as `[rbx]`.
    #[test]
    fn addr_size_override_memory_bails() {
        let r = lift_one(&[0x67, 0x48, 0x8b, 0x03]); // mov rax, [ebx]  (32-bit addr)
        assert!(r.is_err(), "0x67 address-size memory operand must bail");
    }

    fn x86_gpr(idx: u8) -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::gpr(idx)))
    }

    fn lift_single(bytes: &[u8]) -> Result<LiftResult, LiftError> {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);
        lifter.lift_insn(0x1000, bytes, &mut ctx)
    }

    fn assert_adx_sequence(
        result: &LiftResult,
        start: usize,
        kind: AdxKind,
        dst: VReg,
        src1: VReg,
        src2: VReg,
        width: OpWidth,
    ) {
        let ops = &result.ops[start..];
        assert_eq!(ops.len(), if kind == AdxKind::Adcx { 13 } else { 14 });

        let old_flags = match &ops[0].kind {
            OpKind::ReadFlags { dst } => *dst,
            other => panic!("expected ADX ReadFlags, got {other:?}"),
        };
        let carry_in = match &ops[1].kind {
            OpKind::SetCC {
                dst,
                cond,
                width: OpWidth::W64,
            } => {
                assert_eq!(*cond, kind.carry_cond());
                *dst
            }
            other => panic!("expected ADX carry-in SetCC, got {other:?}"),
        };
        let lhs = match &ops[2].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: got_width,
            } => {
                assert_eq!(*src, src1);
                assert_eq!(*got_width, width);
                *dst
            }
            other => panic!("expected ADX lhs copy, got {other:?}"),
        };
        let sum = match &ops[3].kind {
            OpKind::Add {
                dst,
                src1: got_src1,
                src2: SrcOperand::Reg(got_src2),
                width: got_width,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*got_src1, lhs);
                assert_eq!(*got_src2, src2);
                assert_eq!(*got_width, width);
                *dst
            }
            other => panic!("expected ADX source add, got {other:?}"),
        };
        match &ops[4].kind {
            OpKind::Add {
                dst: got_dst,
                src1: got_src1,
                src2: SrcOperand::Reg(got_src2),
                width: got_width,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*got_dst, dst);
                assert_eq!(*got_src1, sum);
                assert_eq!(*got_src2, carry_in);
                assert_eq!(*got_width, width);
            }
            other => panic!("expected ADX carry add, got {other:?}"),
        }
        match &ops[5].kind {
            OpKind::Cmp {
                src1: got_src1,
                src2: SrcOperand::Reg(got_src2),
                width: got_width,
            } => {
                assert_eq!(*got_src1, sum);
                assert_eq!(*got_src2, lhs);
                assert_eq!(*got_width, width);
            }
            other => panic!("expected ADX first carry Cmp, got {other:?}"),
        }
        let carry_from_add = match &ops[6].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W64,
            } => *dst,
            other => panic!("expected ADX first carry SetCC, got {other:?}"),
        };
        match &ops[7].kind {
            OpKind::Cmp {
                src1: got_src1,
                src2: SrcOperand::Reg(got_src2),
                width: got_width,
            } => {
                assert_eq!(*got_src1, dst);
                assert_eq!(*got_src2, sum);
                assert_eq!(*got_width, width);
            }
            other => panic!("expected ADX carry-in Cmp, got {other:?}"),
        }
        let carry_from_cin = match &ops[8].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W64,
            } => *dst,
            other => panic!("expected ADX carry-in SetCC, got {other:?}"),
        };
        let carry_out = match &ops[9].kind {
            OpKind::Or {
                dst,
                src1: got_src1,
                src2: SrcOperand::Reg(got_src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*got_src1, carry_from_add);
                assert_eq!(*got_src2, carry_from_cin);
                *dst
            }
            other => panic!("expected ADX carry merge, got {other:?}"),
        };

        let (flag_value, flag_patch_idx) = if kind == AdxKind::Adcx {
            (carry_out, 10)
        } else {
            let shifted = match &ops[10].kind {
                OpKind::Shl {
                    dst,
                    src,
                    amount: SrcOperand::Imm(11),
                    width: OpWidth::W64,
                    flags: FlagUpdate::None,
                } => {
                    assert_eq!(*src, carry_out);
                    *dst
                }
                other => panic!("expected ADOX OF shift, got {other:?}"),
            };
            (shifted, 11)
        };

        let flag_mask = 1i64 << kind.flag_bit();
        let cleared_flags = match &ops[flag_patch_idx].kind {
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Imm(mask),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*src1, old_flags);
                assert_eq!(*mask, !flag_mask);
                *dst
            }
            other => panic!("expected ADX old flag clear, got {other:?}"),
        };
        let new_flags = match &ops[flag_patch_idx + 1].kind {
            OpKind::Or {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*src1, cleared_flags);
                assert_eq!(*src2, flag_value);
                *dst
            }
            other => panic!("expected ADX new flag merge, got {other:?}"),
        };
        match &ops[flag_patch_idx + 2].kind {
            OpKind::WriteFlags { src } => assert_eq!(*src, new_flags),
            other => panic!("expected ADX WriteFlags, got {other:?}"),
        }
    }

    #[test]
    fn lift_adx_legacy_prefixes_like_llvm() {
        for (bytes, name, kind, width) in [
            (
                &[0x66, 0x0F, 0x38, 0xF6, 0xC3][..],
                "adcxl",
                AdxKind::Adcx,
                OpWidth::W32,
            ),
            (
                &[0x66, 0x48, 0x0F, 0x38, 0xF6, 0xC3][..],
                "adcxq",
                AdxKind::Adcx,
                OpWidth::W64,
            ),
            (
                &[0xF3, 0x0F, 0x38, 0xF6, 0xC3][..],
                "adoxl",
                AdxKind::Adox,
                OpWidth::W32,
            ),
            (
                &[0xF3, 0x48, 0x0F, 0x38, 0xF6, 0xC3][..],
                "adoxq",
                AdxKind::Adox,
                OpWidth::W64,
            ),
            (
                &[0x66, 0xF3, 0x0F, 0x38, 0xF6, 0xC3][..],
                "66+f3 adoxl",
                AdxKind::Adox,
                OpWidth::W32,
            ),
        ] {
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len(), "{name}");
            assert_adx_sequence(&result, 0, kind, x86_gpr(0), x86_gpr(0), x86_gpr(3), width);
        }
    }

    #[test]
    fn lift_adx_apx_nd_uses_vvvv_destination_like_llvm() {
        // LLVM 20: `adcxq %rbx, %rax, %r8` => 62 f4 bd 18 66 c3.
        let result = lift_single(&[0x62, 0xF4, 0xBD, 0x18, 0x66, 0xC3]).unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_adx_sequence(
            &result,
            0,
            AdxKind::Adcx,
            x86_gpr(8),
            x86_gpr(0),
            x86_gpr(3),
            OpWidth::W64,
        );

        // LLVM accepts nonzero EVEX aaa bits for this opcode and ignores them.
        let result = lift_single(&[0x62, 0xF4, 0xBD, 0x19, 0x66, 0xC3]).unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_adx_sequence(
            &result,
            0,
            AdxKind::Adcx,
            x86_gpr(8),
            x86_gpr(0),
            x86_gpr(3),
            OpWidth::W64,
        );

        // LLVM 20: `adoxq 32(%r17,%r18,4), %r19, %r20`
        // => 62 ec da 10 66 5c 91 20.
        let result = lift_single(&[0x62, 0xEC, 0xDA, 0x10, 0x66, 0x5C, 0x91, 0x20]).unwrap();
        assert_eq!(result.bytes_consumed, 8);
        let mem_src = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        ..
                    },
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(17));
                assert_eq!(*index, x86_gpr(18));
                *dst
            }
            other => panic!("expected APX ADOX memory load, got {other:?}"),
        };
        assert_adx_sequence(
            &result,
            1,
            AdxKind::Adox,
            x86_gpr(20),
            x86_gpr(19),
            mem_src,
            OpWidth::W64,
        );
    }

    #[test]
    fn lift_adx_rejects_invalid_forms_like_llvm() {
        for (bytes, name) in [
            (&[0x0F, 0x38, 0xF6, 0xC3][..], "legacy missing prefix"),
            (&[0xF2, 0x0F, 0x38, 0xF6, 0xC3][..], "legacy f2 prefix"),
            (&[0x62, 0xF4, 0xBD, 0x08, 0x66, 0xC3][..], "APX missing ND"),
            (&[0x62, 0xF4, 0xBD, 0x1C, 0x66, 0xC3][..], "APX NF reserved"),
            (&[0x62, 0xF4, 0xBC, 0x18, 0x66, 0xC3][..], "APX pp none"),
            (&[0x62, 0xF4, 0xBF, 0x18, 0x66, 0xC3][..], "APX pp 3"),
            (
                &[0x62, 0xF4, 0xBD, 0x98, 0x66, 0xC3][..],
                "APX z bit reserved",
            ),
        ] {
            let err = lift_single(bytes).expect_err(name);
            assert!(
                matches!(err, LiftError::InvalidEncoding { .. }),
                "{name}: {err:?}"
            );
        }
    }

    fn assert_vex_andn_op(
        ops: &[SmirOp],
        index: usize,
        dst: VReg,
        src: VReg,
        inverted: VReg,
        width: OpWidth,
    ) {
        match &ops[index].kind {
            OpKind::AndNot {
                dst: got_dst,
                src1,
                src2: SrcOperand::Reg(got_inverted),
                width: got_width,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*got_dst, dst);
                assert_eq!(*src1, src);
                assert_eq!(*got_inverted, inverted);
                assert_eq!(*got_width, width);
            }
            other => panic!("expected VEX ANDN, got {other:?}"),
        }
    }

    #[test]
    fn lift_vex_andn_registers_like_llvm() {
        for (bytes, width) in [
            (&[0xC4, 0xE2, 0x70, 0xF2, 0xC2][..], OpWidth::W32),
            (&[0xC4, 0xE2, 0xF0, 0xF2, 0xC2][..], OpWidth::W64),
        ] {
            // LLVM 23 examples:
            //   `andn eax, ecx, edx` => c4 e2 70 f2 c2
            //   `andn rax, rcx, rdx` => c4 e2 f0 f2 c2
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len());
            assert_eq!(result.ops.len(), 1);
            assert_vex_andn_op(&result.ops, 0, x86_gpr(0), x86_gpr(2), x86_gpr(1), width);
        }
    }

    #[test]
    fn lift_vex_andn_memory_source_like_llvm() {
        // LLVM 23:
        //   `andn eax, r9d, dword ptr [r10 + 4*r11 + 32]`
        //       => c4 82 30 f2 44 9a 20
        let result = lift_single(&[0xC4, 0x82, 0x30, 0xF2, 0x44, 0x9A, 0x20]).unwrap();
        assert_eq!(result.bytes_consumed, 7);
        assert_eq!(result.ops.len(), 2);
        let src = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        disp_size: DispSize::Disp8,
                    },
                width: MemWidth::B4,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(10));
                assert_eq!(*index, x86_gpr(11));
                *dst
            }
            other => panic!("expected VEX ANDN memory source load, got {other:?}"),
        };
        assert_vex_andn_op(&result.ops, 1, x86_gpr(0), src, x86_gpr(9), OpWidth::W32);
    }

    #[test]
    fn lift_vex_andn_allows_destination_source_alias_like_llvm() {
        // LLVM 23: `andn ecx, ecx, edx` => c4 e2 70 f2 ca.
        let result = lift_single(&[0xC4, 0xE2, 0x70, 0xF2, 0xCA]).unwrap();
        assert_eq!(result.bytes_consumed, 5);
        assert_eq!(result.ops.len(), 1);
        assert_vex_andn_op(
            &result.ops,
            0,
            x86_gpr(1),
            x86_gpr(2),
            x86_gpr(1),
            OpWidth::W32,
        );
    }

    #[test]
    fn lift_vex_andn_rejects_invalid_forms_like_spec() {
        let err = lift_single(&[0xC4, 0xE2, 0x74, 0xF2, 0xC2]).expect_err("ANDN VEX.L=1");
        assert!(matches!(err, LiftError::InvalidEncoding { .. }), "{err:?}");

        let err = lift_single(&[0xC4, 0xE2, 0x73, 0xF2, 0xC2]).unwrap_err();
        assert!(matches!(err, LiftError::Unsupported { .. }), "{err:?}");
    }

    fn assert_vex_bzhi_bextr_op(
        ops: &[SmirOp],
        index: usize,
        name: &str,
        dst: VReg,
        src: VReg,
        control: VReg,
        width: OpWidth,
    ) {
        match (&ops[index].kind, name) {
            (
                OpKind::Bzhi {
                    dst: got_dst,
                    src: got_src,
                    index: got_control,
                    width: got_width,
                    flags: got_flags,
                },
                "bzhi",
            )
            | (
                OpKind::Bextr {
                    dst: got_dst,
                    src: got_src,
                    control: got_control,
                    width: got_width,
                    flags: got_flags,
                },
                "bextr",
            ) => {
                let expected_flags = match name {
                    "bzhi" => FlagSet::CF
                        .union(FlagSet::ZF)
                        .union(FlagSet::SF)
                        .union(FlagSet::OF),
                    "bextr" => FlagSet::CF.union(FlagSet::ZF).union(FlagSet::OF),
                    _ => unreachable!(),
                };
                assert_eq!(*got_dst, dst, "{name}");
                assert_eq!(*got_src, src, "{name}");
                assert_eq!(*got_control, control, "{name}");
                assert_eq!(*got_width, width, "{name}");
                assert_eq!(got_flags.as_set(), expected_flags, "{name}");
            }
            (other, _) => panic!("expected VEX {name}, got {other:?}"),
        }
    }

    #[test]
    fn lift_vex_bzhi_bextr_registers_like_llvm() {
        for (bytes, name, width) in [
            (&[0xC4, 0xE2, 0x70, 0xF5, 0xC2][..], "bzhi", OpWidth::W32),
            (&[0xC4, 0xE2, 0x70, 0xF7, 0xC2][..], "bextr", OpWidth::W32),
            (&[0xC4, 0xE2, 0xF0, 0xF5, 0xC2][..], "bzhi", OpWidth::W64),
            (&[0xC4, 0xE2, 0xF0, 0xF7, 0xC2][..], "bextr", OpWidth::W64),
        ] {
            // LLVM 23 examples:
            //   `bzhi eax, edx, ecx`  => c4 e2 70 f5 c2
            //   `bextr eax, edx, ecx` => c4 e2 70 f7 c2
            //   `bzhi rax, rdx, rcx`  => c4 e2 f0 f5 c2
            //   `bextr rax, rdx, rcx` => c4 e2 f0 f7 c2
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len(), "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            assert_vex_bzhi_bextr_op(
                &result.ops,
                0,
                name,
                x86_gpr(0),
                x86_gpr(2),
                x86_gpr(1),
                width,
            );
        }
    }

    #[test]
    fn lift_vex_bzhi_bextr_memory_source_like_llvm() {
        for (bytes, name) in [
            (&[0xC4, 0x82, 0x30, 0xF5, 0x44, 0x9A, 0x20][..], "bzhi"),
            (&[0xC4, 0x82, 0x30, 0xF7, 0x44, 0x9A, 0x20][..], "bextr"),
        ] {
            // LLVM 23:
            //   `bzhi eax, dword ptr [r10 + 4*r11 + 32], r9d`
            //       => c4 82 30 f5 44 9a 20
            //   `bextr eax, dword ptr [r10 + 4*r11 + 32], r9d`
            //       => c4 82 30 f7 44 9a 20
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len(), "{name}");
            assert_eq!(result.ops.len(), 2, "{name}");
            let src = match &result.ops[0].kind {
                OpKind::Load {
                    dst,
                    addr:
                        Address::BaseIndexScale {
                            base: Some(base),
                            index,
                            scale: 4,
                            disp: 0x20,
                            disp_size: DispSize::Disp8,
                        },
                    width: MemWidth::B4,
                    sign: SignExtend::Zero,
                } => {
                    assert_eq!(*base, x86_gpr(10), "{name}");
                    assert_eq!(*index, x86_gpr(11), "{name}");
                    *dst
                }
                other => panic!("expected VEX {name} memory source load, got {other:?}"),
            };
            assert_vex_bzhi_bextr_op(
                &result.ops,
                1,
                name,
                x86_gpr(0),
                src,
                x86_gpr(9),
                OpWidth::W32,
            );
        }
    }

    #[test]
    fn lift_vex_bzhi_bextr_rejects_invalid_forms_like_spec() {
        for (bytes, name) in [
            (&[0xC4, 0xE2, 0x74, 0xF5, 0xC2][..], "bzhi VEX.L=1"),
            (&[0xC4, 0xE2, 0x74, 0xF7, 0xC2][..], "bextr VEX.L=1"),
        ] {
            let err = lift_single(bytes).expect_err(name);
            assert!(
                matches!(err, LiftError::InvalidEncoding { .. }),
                "{name}: {err:?}"
            );
        }
    }

    fn assert_vex_pdep_pext_op(
        ops: &[SmirOp],
        index: usize,
        name: &str,
        dst: VReg,
        src: VReg,
        mask: VReg,
        width: OpWidth,
    ) {
        match (&ops[index].kind, name) {
            (
                OpKind::Pdep {
                    dst: got_dst,
                    src: got_src,
                    mask: got_mask,
                    width: got_width,
                },
                "pdep",
            )
            | (
                OpKind::Pext {
                    dst: got_dst,
                    src: got_src,
                    mask: got_mask,
                    width: got_width,
                },
                "pext",
            ) => {
                assert_eq!(*got_dst, dst, "{name}");
                assert_eq!(*got_src, src, "{name}");
                assert_eq!(*got_mask, mask, "{name}");
                assert_eq!(*got_width, width, "{name}");
            }
            (other, _) => panic!("expected VEX {name}, got {other:?}"),
        }
    }

    #[test]
    fn lift_vex_pdep_pext_registers_like_llvm() {
        for (bytes, name, width) in [
            (&[0xC4, 0xE2, 0x73, 0xF5, 0xC2][..], "pdep", OpWidth::W32),
            (&[0xC4, 0xE2, 0x72, 0xF5, 0xC2][..], "pext", OpWidth::W32),
            (&[0xC4, 0xE2, 0xF3, 0xF5, 0xC2][..], "pdep", OpWidth::W64),
            (&[0xC4, 0xE2, 0xF2, 0xF5, 0xC2][..], "pext", OpWidth::W64),
        ] {
            // LLVM 23 examples:
            //   `pdep eax, ecx, edx` => c4 e2 73 f5 c2
            //   `pext eax, ecx, edx` => c4 e2 72 f5 c2
            //   `pdep rax, rcx, rdx` => c4 e2 f3 f5 c2
            //   `pext rax, rcx, rdx` => c4 e2 f2 f5 c2
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len(), "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            assert_vex_pdep_pext_op(
                &result.ops,
                0,
                name,
                x86_gpr(0),
                x86_gpr(1),
                x86_gpr(2),
                width,
            );
        }
    }

    #[test]
    fn lift_vex_pdep_pext_memory_mask_like_llvm() {
        for (bytes, name) in [
            (&[0xC4, 0x82, 0x33, 0xF5, 0x44, 0x9A, 0x20][..], "pdep"),
            (&[0xC4, 0x82, 0x32, 0xF5, 0x44, 0x9A, 0x20][..], "pext"),
        ] {
            // LLVM 23:
            //   `pdep eax, r9d, dword ptr [r10 + 4*r11 + 32]`
            //       => c4 82 33 f5 44 9a 20
            //   `pext eax, r9d, dword ptr [r10 + 4*r11 + 32]`
            //       => c4 82 32 f5 44 9a 20
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len(), "{name}");
            assert_eq!(result.ops.len(), 2, "{name}");
            let mask = match &result.ops[0].kind {
                OpKind::Load {
                    dst,
                    addr:
                        Address::BaseIndexScale {
                            base: Some(base),
                            index,
                            scale: 4,
                            disp: 0x20,
                            disp_size: DispSize::Disp8,
                        },
                    width: MemWidth::B4,
                    sign: SignExtend::Zero,
                } => {
                    assert_eq!(*base, x86_gpr(10), "{name}");
                    assert_eq!(*index, x86_gpr(11), "{name}");
                    *dst
                }
                other => panic!("expected VEX {name} memory mask load, got {other:?}"),
            };
            assert_vex_pdep_pext_op(
                &result.ops,
                1,
                name,
                x86_gpr(0),
                x86_gpr(9),
                mask,
                OpWidth::W32,
            );
        }
    }

    #[test]
    fn lift_vex_pdep_pext_rejects_invalid_l_like_spec() {
        for (bytes, name) in [
            (&[0xC4, 0xE2, 0x77, 0xF5, 0xC2][..], "pdep VEX.L=1"),
            (&[0xC4, 0xE2, 0x76, 0xF5, 0xC2][..], "pext VEX.L=1"),
        ] {
            let err = lift_single(bytes).expect_err(name);
            assert!(
                matches!(err, LiftError::InvalidEncoding { .. }),
                "{name}: {err:?}"
            );
        }

        let err = lift_single(&[0xC4, 0xE2, 0x71, 0xF5, 0xC2]).unwrap_err();
        assert!(matches!(err, LiftError::Unsupported { .. }), "{err:?}");
    }

    fn assert_vex_mulx_op(
        ops: &[SmirOp],
        index: usize,
        dst_hi: VReg,
        dst_lo: VReg,
        src2: VReg,
        width: OpWidth,
    ) {
        match &ops[index].kind {
            OpKind::MulU {
                dst_lo: got_dst_lo,
                dst_hi: Some(got_dst_hi),
                src1,
                src2: SrcOperand::Reg(got_src2),
                width: got_width,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*got_dst_hi, dst_hi);
                assert_eq!(*got_dst_lo, dst_lo);
                assert_eq!(*src1, x86_gpr(2));
                assert_eq!(*got_src2, src2);
                assert_eq!(*got_width, width);
            }
            other => panic!("expected VEX MULX, got {other:?}"),
        }
    }

    #[test]
    fn lift_vex_mulx_registers_like_llvm() {
        for (bytes, width) in [
            (&[0xC4, 0xE2, 0x73, 0xF6, 0xC3][..], OpWidth::W32),
            (&[0xC4, 0xE2, 0xF3, 0xF6, 0xC3][..], OpWidth::W64),
        ] {
            // LLVM 23 examples:
            //   `mulx eax, ecx, ebx` => c4 e2 73 f6 c3
            //   `mulx rax, rcx, rbx` => c4 e2 f3 f6 c3
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len());
            assert_eq!(result.ops.len(), 1);
            assert_vex_mulx_op(&result.ops, 0, x86_gpr(0), x86_gpr(1), x86_gpr(3), width);
        }
    }

    #[test]
    fn lift_vex_mulx_memory_source_like_llvm() {
        // LLVM 23:
        //   `mulx eax, r9d, dword ptr [r10 + 4*r11 + 32]`
        //       => c4 82 33 f6 44 9a 20
        let result = lift_single(&[0xC4, 0x82, 0x33, 0xF6, 0x44, 0x9A, 0x20]).unwrap();
        assert_eq!(result.bytes_consumed, 7);
        assert_eq!(result.ops.len(), 2);
        let src2 = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        disp_size: DispSize::Disp8,
                    },
                width: MemWidth::B4,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(10));
                assert_eq!(*index, x86_gpr(11));
                *dst
            }
            other => panic!("expected VEX MULX memory source load, got {other:?}"),
        };
        assert_vex_mulx_op(&result.ops, 1, x86_gpr(0), x86_gpr(9), src2, OpWidth::W32);
    }

    #[test]
    fn lift_vex_mulx_alias_destination_keeps_high_half_like_spec() {
        // LLVM 23: `mulx rcx, rcx, rdx` => c4 e2 f3 f6 ca.
        let result = lift_single(&[0xC4, 0xE2, 0xF3, 0xF6, 0xCA]).unwrap();
        assert_eq!(result.bytes_consumed, 5);
        assert_eq!(result.ops.len(), 1);
        assert_vex_mulx_op(
            &result.ops,
            0,
            x86_gpr(1),
            x86_gpr(1),
            x86_gpr(2),
            OpWidth::W64,
        );
    }

    #[test]
    fn lift_vex_mulx_rejects_invalid_forms_like_spec() {
        let err = lift_single(&[0xC4, 0xE2, 0x77, 0xF6, 0xC3]).expect_err("MULX VEX.L=1");
        assert!(matches!(err, LiftError::InvalidEncoding { .. }), "{err:?}");

        let err = lift_single(&[0xC4, 0xE2, 0x72, 0xF6, 0xC3]).unwrap_err();
        assert!(matches!(err, LiftError::Unsupported { .. }), "{err:?}");
    }

    fn assert_vex_bmi2_shift(
        bytes: &[u8],
        expected_op: &str,
        dst: VReg,
        src: VReg,
        count: VReg,
        width: OpWidth,
    ) {
        let result = lift_single(bytes).unwrap();
        assert_eq!(result.bytes_consumed, bytes.len(), "{expected_op}");
        assert_eq!(result.ops.len(), 2, "{expected_op}");
        assert_vex_bmi2_shift_ops(&result.ops, 0, expected_op, dst, src, count, width);
    }

    fn assert_vex_bmi2_shift_ops(
        ops: &[SmirOp],
        start: usize,
        expected_op: &str,
        dst: VReg,
        src: VReg,
        count: VReg,
        width: OpWidth,
    ) {
        let masked_count = match &ops[start].kind {
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Imm(mask),
                width: got_width,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*src1, count, "{expected_op}");
                assert_eq!(*mask, (width.bits() - 1) as i64, "{expected_op}");
                assert_eq!(*got_width, width, "{expected_op}");
                *dst
            }
            other => panic!("expected VEX BMI2 {expected_op} count mask, got {other:?}"),
        };
        match (&ops[start + 1].kind, expected_op) {
            (
                OpKind::Sar {
                    dst: got_dst,
                    src: got_src,
                    amount: SrcOperand::Reg(amount),
                    width: got_width,
                    flags: FlagUpdate::None,
                },
                "sarx",
            )
            | (
                OpKind::Shr {
                    dst: got_dst,
                    src: got_src,
                    amount: SrcOperand::Reg(amount),
                    width: got_width,
                    flags: FlagUpdate::None,
                },
                "shrx",
            )
            | (
                OpKind::Shl {
                    dst: got_dst,
                    src: got_src,
                    amount: SrcOperand::Reg(amount),
                    width: got_width,
                    flags: FlagUpdate::None,
                },
                "shlx",
            ) => {
                assert_eq!(*got_dst, dst, "{expected_op}");
                assert_eq!(*got_src, src, "{expected_op}");
                assert_eq!(*amount, masked_count, "{expected_op}");
                assert_eq!(*got_width, width, "{expected_op}");
            }
            (other, _) => panic!("expected VEX BMI2 {expected_op}, got {other:?}"),
        }
    }

    #[test]
    fn lift_vex_bmi2_shift_registers_like_llvm() {
        for (bytes, expected_op, width) in [
            (&[0xC4, 0xE2, 0x72, 0xF7, 0xC3][..], "sarx", OpWidth::W32),
            (&[0xC4, 0xE2, 0x73, 0xF7, 0xC3][..], "shrx", OpWidth::W32),
            (&[0xC4, 0xE2, 0x71, 0xF7, 0xC3][..], "shlx", OpWidth::W32),
            (&[0xC4, 0xE2, 0xF2, 0xF7, 0xC3][..], "sarx", OpWidth::W64),
            (&[0xC4, 0xE2, 0xF3, 0xF7, 0xC3][..], "shrx", OpWidth::W64),
            (&[0xC4, 0xE2, 0xF1, 0xF7, 0xC3][..], "shlx", OpWidth::W64),
        ] {
            // LLVM 23 examples:
            //   `sarx eax, ebx, ecx` => c4 e2 72 f7 c3
            //   `shrx eax, ebx, ecx` => c4 e2 73 f7 c3
            //   `shlx eax, ebx, ecx` => c4 e2 71 f7 c3
            //   `sarx rax, rbx, rcx` => c4 e2 f2 f7 c3
            //   `shrx rax, rbx, rcx` => c4 e2 f3 f7 c3
            //   `shlx rax, rbx, rcx` => c4 e2 f1 f7 c3
            assert_vex_bmi2_shift(
                bytes,
                expected_op,
                x86_gpr(0),
                x86_gpr(3),
                x86_gpr(1),
                width,
            );
        }
    }

    #[test]
    fn lift_vex_bmi2_shift_memory_source_like_llvm() {
        for (bytes, expected_op) in [
            (&[0xC4, 0x82, 0x32, 0xF7, 0x44, 0x9A, 0x20][..], "sarx"),
            (&[0xC4, 0x82, 0x33, 0xF7, 0x44, 0x9A, 0x20][..], "shrx"),
            (&[0xC4, 0x82, 0x31, 0xF7, 0x44, 0x9A, 0x20][..], "shlx"),
        ] {
            // LLVM 23:
            //   `sarx eax, dword ptr [r10 + 4*r11 + 32], r9d`
            //       => c4 82 32 f7 44 9a 20
            //   `shrx eax, dword ptr [r10 + 4*r11 + 32], r9d`
            //       => c4 82 33 f7 44 9a 20
            //   `shlx eax, dword ptr [r10 + 4*r11 + 32], r9d`
            //       => c4 82 31 f7 44 9a 20
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len(), "{expected_op}");
            assert_eq!(result.ops.len(), 3, "{expected_op}");
            let src = match &result.ops[0].kind {
                OpKind::Load {
                    dst,
                    addr:
                        Address::BaseIndexScale {
                            base: Some(base),
                            index,
                            scale: 4,
                            disp: 0x20,
                            disp_size: DispSize::Disp8,
                        },
                    width: MemWidth::B4,
                    sign: SignExtend::Zero,
                } => {
                    assert_eq!(*base, x86_gpr(10), "{expected_op}");
                    assert_eq!(*index, x86_gpr(11), "{expected_op}");
                    *dst
                }
                other => panic!("expected VEX BMI2 {expected_op} memory load, got {other:?}"),
            };
            assert_vex_bmi2_shift_ops(
                &result.ops,
                1,
                expected_op,
                x86_gpr(0),
                src,
                x86_gpr(9),
                OpWidth::W32,
            );
        }
    }

    #[test]
    fn lift_vex_bmi2_shift_rejects_invalid_forms_like_spec() {
        for (bytes, name) in [
            (&[0xC4, 0xE2, 0x76, 0xF7, 0xC3][..], "sarx VEX.L=1"),
            (&[0xC4, 0xE2, 0x77, 0xF7, 0xC3][..], "shrx VEX.L=1"),
            (&[0xC4, 0xE2, 0x75, 0xF7, 0xC3][..], "shlx VEX.L=1"),
        ] {
            let err = lift_single(bytes).expect_err(name);
            assert!(
                matches!(err, LiftError::InvalidEncoding { .. }),
                "{name}: {err:?}"
            );
        }
    }

    fn assert_vex_rorx_op(
        ops: &[SmirOp],
        index: usize,
        dst: VReg,
        src: VReg,
        amount: i64,
        width: OpWidth,
    ) {
        match &ops[index].kind {
            OpKind::Ror {
                dst: got_dst,
                src: got_src,
                amount: SrcOperand::Imm(got_amount),
                width: got_width,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*got_dst, dst);
                assert_eq!(*got_src, src);
                assert_eq!(*got_amount, amount);
                assert_eq!(*got_width, width);
            }
            other => panic!("expected VEX RORX, got {other:?}"),
        }
    }

    #[test]
    fn lift_vex_rorx_registers_like_llvm() {
        for (bytes, width) in [
            (&[0xC4, 0xE3, 0x7B, 0xF0, 0xC3, 0x0D][..], OpWidth::W32),
            (&[0xC4, 0xE3, 0xFB, 0xF0, 0xC3, 0x0D][..], OpWidth::W64),
        ] {
            // LLVM 23 examples:
            //   `rorx eax, ebx, 13` => c4 e3 7b f0 c3 0d
            //   `rorx rax, rbx, 13` => c4 e3 fb f0 c3 0d
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len());
            assert_eq!(result.ops.len(), 1);
            assert_vex_rorx_op(&result.ops, 0, x86_gpr(0), x86_gpr(3), 13, width);
        }
    }

    #[test]
    fn lift_vex_rorx_memory_source_like_llvm() {
        // LLVM 23:
        //   `rorx eax, dword ptr [r10 + 4*r11 + 32], 13`
        //       => c4 83 7b f0 44 9a 20 0d
        let result = lift_single(&[0xC4, 0x83, 0x7B, 0xF0, 0x44, 0x9A, 0x20, 0x0D]).unwrap();
        assert_eq!(result.bytes_consumed, 8);
        assert_eq!(result.ops.len(), 2);
        let src = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        disp_size: DispSize::Disp8,
                    },
                width: MemWidth::B4,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(10));
                assert_eq!(*index, x86_gpr(11));
                *dst
            }
            other => panic!("expected VEX RORX memory source load, got {other:?}"),
        };
        assert_vex_rorx_op(&result.ops, 1, x86_gpr(0), src, 13, OpWidth::W32);
    }

    #[test]
    fn lift_vex_rorx_rejects_invalid_forms_like_llvm() {
        for (bytes, name) in [
            (&[0xC4, 0xE3, 0x7F, 0xF0, 0xC3, 0x0D][..], "rorx VEX.L=1"),
            (
                &[0xC4, 0xE3, 0x73, 0xF0, 0xC3, 0x0D][..],
                "rorx reserved vvvv",
            ),
        ] {
            let err = lift_single(bytes).expect_err(name);
            assert!(
                matches!(err, LiftError::InvalidEncoding { .. }),
                "{name}: {err:?}"
            );
        }

        let err = lift_single(&[0xC4, 0xE3, 0x78, 0xF0, 0xC3, 0x0D]).unwrap_err();
        assert!(matches!(err, LiftError::Unsupported { .. }), "{err:?}");

        let err = lift_single(&[0xC4, 0xE3, 0x7B, 0xF0, 0xC3]).unwrap_err();
        assert!(matches!(err, LiftError::Incomplete { .. }), "{err:?}");
    }

    fn assert_apx_bmi2_shift(
        bytes: &[u8],
        expected_op: &str,
        dst: VReg,
        src: VReg,
        count: VReg,
        width: OpWidth,
    ) {
        let result = lift_single(bytes).unwrap();
        assert_eq!(result.bytes_consumed, bytes.len(), "{expected_op}");
        assert_eq!(result.ops.len(), 2, "{expected_op}");
        assert_apx_bmi2_shift_ops(&result.ops, 0, expected_op, dst, src, count, width);
    }

    fn assert_apx_bmi2_shift_ops(
        ops: &[SmirOp],
        start: usize,
        expected_op: &str,
        dst: VReg,
        src: VReg,
        count: VReg,
        width: OpWidth,
    ) {
        let masked_count = match &ops[start].kind {
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Imm(mask),
                width: got_width,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*src1, count, "{expected_op}");
                assert_eq!(*mask, (width.bits() - 1) as i64, "{expected_op}");
                assert_eq!(*got_width, width, "{expected_op}");
                *dst
            }
            other => panic!("expected APX BMI2 {expected_op} count mask, got {other:?}"),
        };
        match (&ops[start + 1].kind, expected_op) {
            (
                OpKind::Sar {
                    dst: got_dst,
                    src: got_src,
                    amount: SrcOperand::Reg(amount),
                    width: got_width,
                    flags: FlagUpdate::None,
                },
                "sarx",
            )
            | (
                OpKind::Shr {
                    dst: got_dst,
                    src: got_src,
                    amount: SrcOperand::Reg(amount),
                    width: got_width,
                    flags: FlagUpdate::None,
                },
                "shrx",
            )
            | (
                OpKind::Shl {
                    dst: got_dst,
                    src: got_src,
                    amount: SrcOperand::Reg(amount),
                    width: got_width,
                    flags: FlagUpdate::None,
                },
                "shlx",
            ) => {
                assert_eq!(*got_dst, dst, "{expected_op}");
                assert_eq!(*got_src, src, "{expected_op}");
                assert_eq!(*amount, masked_count, "{expected_op}");
                assert_eq!(*got_width, width, "{expected_op}");
            }
            (other, _) => panic!("expected APX BMI2 {expected_op}, got {other:?}"),
        }
    }

    fn assert_apx_bmi2_memory_load(op: &SmirOp, expected: &str) -> VReg {
        match &op.kind {
            OpKind::Load {
                dst,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        ..
                    },
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(17), "{expected}");
                assert_eq!(*index, x86_gpr(18), "{expected}");
                *dst
            }
            other => panic!("expected APX BMI2 {expected} memory load, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_bmi2_register_forms_like_llvm() {
        // LLVM 20 APX EVEX encodings:
        //   pdepq %rbx, %r19, %r20 => 62 e2 e7 00 f5 e3
        //   pextq %rbx, %r19, %r20 => 62 e2 e6 00 f5 e3
        //   mulxq %rbx, %r19, %r20 => 62 e2 e7 00 f6 e3
        let pdep = lift_single(&[0x62, 0xE2, 0xE7, 0x00, 0xF5, 0xE3]).unwrap();
        assert_eq!(pdep.bytes_consumed, 6);
        match &pdep.ops[..] {
            [
                SmirOp {
                    kind:
                        OpKind::Pdep {
                            dst,
                            src,
                            mask,
                            width,
                        },
                    ..
                },
            ] => {
                assert_eq!(*dst, x86_gpr(20));
                assert_eq!(*src, x86_gpr(19));
                assert_eq!(*mask, x86_gpr(3));
                assert_eq!(*width, OpWidth::W64);
            }
            other => panic!("expected APX PDEP, got {other:?}"),
        }

        let pext = lift_single(&[0x62, 0xE2, 0xE6, 0x00, 0xF5, 0xE3]).unwrap();
        assert_eq!(pext.bytes_consumed, 6);
        match &pext.ops[..] {
            [
                SmirOp {
                    kind:
                        OpKind::Pext {
                            dst,
                            src,
                            mask,
                            width,
                        },
                    ..
                },
            ] => {
                assert_eq!(*dst, x86_gpr(20));
                assert_eq!(*src, x86_gpr(19));
                assert_eq!(*mask, x86_gpr(3));
                assert_eq!(*width, OpWidth::W64);
            }
            other => panic!("expected APX PEXT, got {other:?}"),
        }

        let mulx = lift_single(&[0x62, 0xE2, 0xE7, 0x00, 0xF6, 0xE3]).unwrap();
        assert_eq!(mulx.bytes_consumed, 6);
        match &mulx.ops[..] {
            [
                SmirOp {
                    kind:
                        OpKind::MulU {
                            dst_lo,
                            dst_hi: Some(dst_hi),
                            src1,
                            src2: SrcOperand::Reg(src2),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ..
                },
            ] => {
                assert_eq!(*dst_lo, x86_gpr(19));
                assert_eq!(*dst_hi, x86_gpr(20));
                assert_eq!(*src1, x86_gpr(2));
                assert_eq!(*src2, x86_gpr(3));
                assert_eq!(*width, OpWidth::W64);
            }
            other => panic!("expected APX MULX, got {other:?}"),
        }

        assert_apx_bmi2_shift(
            &[0x62, 0xEA, 0xE6, 0x08, 0xF7, 0xE3],
            "sarx",
            x86_gpr(20),
            x86_gpr(19),
            x86_gpr(3),
            OpWidth::W64,
        );
        assert_apx_bmi2_shift(
            &[0x62, 0xEA, 0xE7, 0x08, 0xF7, 0xE3],
            "shrx",
            x86_gpr(20),
            x86_gpr(19),
            x86_gpr(3),
            OpWidth::W64,
        );
        assert_apx_bmi2_shift(
            &[0x62, 0xEA, 0x65, 0x08, 0xF7, 0xE3],
            "shlx",
            x86_gpr(20),
            x86_gpr(19),
            x86_gpr(3),
            OpWidth::W32,
        );

        // LLVM 20: `rorxq $13, %rbx, %r20` => 62 e3 ff 08 f0 e3 0d.
        let rorx = lift_single(&[0x62, 0xE3, 0xFF, 0x08, 0xF0, 0xE3, 0x0D]).unwrap();
        assert_eq!(rorx.bytes_consumed, 7);
        match &rorx.ops[..] {
            [
                SmirOp {
                    kind:
                        OpKind::Ror {
                            dst,
                            src,
                            amount: SrcOperand::Imm(13),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ..
                },
            ] => {
                assert_eq!(*dst, x86_gpr(20));
                assert_eq!(*src, x86_gpr(3));
                assert_eq!(*width, OpWidth::W64);
            }
            other => panic!("expected APX RORX, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_bmi2_memory_forms_like_llvm() {
        // LLVM 20: `pdepq 32(%r17,%r18,4), %r19, %r20`
        // => 62 ea e3 00 f5 64 91 20.
        let pdep = lift_single(&[0x62, 0xEA, 0xE3, 0x00, 0xF5, 0x64, 0x91, 0x20]).unwrap();
        assert_eq!(pdep.bytes_consumed, 8);
        assert_eq!(pdep.ops.len(), 2);
        let mask = assert_apx_bmi2_memory_load(&pdep.ops[0], "PDEP");
        match &pdep.ops[1].kind {
            OpKind::Pdep {
                dst,
                src,
                mask: got_mask,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(20));
                assert_eq!(*src, x86_gpr(19));
                assert_eq!(*got_mask, mask);
            }
            other => panic!("expected APX PDEP memory op, got {other:?}"),
        }

        // LLVM 20: `pextq 32(%r17,%r18,4), %r19, %r20`
        // => 62 ea e2 00 f5 64 91 20.
        let pext = lift_single(&[0x62, 0xEA, 0xE2, 0x00, 0xF5, 0x64, 0x91, 0x20]).unwrap();
        assert_eq!(pext.bytes_consumed, 8);
        assert_eq!(pext.ops.len(), 2);
        let mask = assert_apx_bmi2_memory_load(&pext.ops[0], "PEXT");
        match &pext.ops[1].kind {
            OpKind::Pext {
                dst,
                src,
                mask: got_mask,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(20));
                assert_eq!(*src, x86_gpr(19));
                assert_eq!(*got_mask, mask);
            }
            other => panic!("expected APX PEXT memory op, got {other:?}"),
        }

        // LLVM 20: `mulxq 32(%r17,%r18,4), %r19, %r20`
        // => 62 ea e3 00 f6 64 91 20.
        let mulx = lift_single(&[0x62, 0xEA, 0xE3, 0x00, 0xF6, 0x64, 0x91, 0x20]).unwrap();
        assert_eq!(mulx.bytes_consumed, 8);
        assert_eq!(mulx.ops.len(), 2);
        let src2 = assert_apx_bmi2_memory_load(&mulx.ops[0], "MULX");
        match &mulx.ops[1].kind {
            OpKind::MulU {
                dst_lo,
                dst_hi: Some(dst_hi),
                src1,
                src2: SrcOperand::Reg(got_src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst_lo, x86_gpr(19));
                assert_eq!(*dst_hi, x86_gpr(20));
                assert_eq!(*src1, x86_gpr(2));
                assert_eq!(*got_src2, src2);
            }
            other => panic!("expected APX MULX memory op, got {other:?}"),
        }

        for (bytes, expected_op) in [
            (
                &[0x62, 0xEA, 0xE2, 0x00, 0xF7, 0x64, 0x91, 0x20][..],
                "sarx",
            ),
            (
                &[0x62, 0xEA, 0xE3, 0x00, 0xF7, 0x64, 0x91, 0x20][..],
                "shrx",
            ),
            (
                &[0x62, 0xEA, 0xE1, 0x00, 0xF7, 0x64, 0x91, 0x20][..],
                "shlx",
            ),
        ] {
            let shifted = lift_single(bytes).unwrap();
            assert_eq!(shifted.bytes_consumed, bytes.len(), "{expected_op}");
            assert_eq!(shifted.ops.len(), 3, "{expected_op}");
            let src = assert_apx_bmi2_memory_load(&shifted.ops[0], expected_op);
            assert_apx_bmi2_shift_ops(
                &shifted.ops,
                1,
                expected_op,
                x86_gpr(20),
                src,
                x86_gpr(19),
                OpWidth::W64,
            );
        }

        // LLVM 20: `rorxq $13, 32(%r17,%r18,4), %r20`
        // => 62 eb fb 08 f0 64 91 20 0d.
        let rorx = lift_single(&[0x62, 0xEB, 0xFB, 0x08, 0xF0, 0x64, 0x91, 0x20, 0x0D]).unwrap();
        assert_eq!(rorx.bytes_consumed, 9);
        assert_eq!(rorx.ops.len(), 2);
        let src = assert_apx_bmi2_memory_load(&rorx.ops[0], "RORX");
        match &rorx.ops[1].kind {
            OpKind::Ror {
                dst,
                src: got_src,
                amount: SrcOperand::Imm(13),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(20));
                assert_eq!(*got_src, src);
            }
            other => panic!("expected APX RORX memory op, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_bmi2_rejects_invalid_forms_like_llvm() {
        for (bytes, name) in [
            (
                &[0x62, 0xE3, 0xFF, 0x0C, 0xF0, 0xE3, 0x0D][..],
                "rorx NF reserved",
            ),
            (
                &[0x62, 0xE3, 0xFF, 0x09, 0xF0, 0xE3, 0x0D][..],
                "rorx aaa reserved",
            ),
            (
                &[0x62, 0xE3, 0xFF, 0x18, 0xF0, 0xE3, 0x0D][..],
                "rorx bit4 reserved",
            ),
            (
                &[0x62, 0xE3, 0xFF, 0x28, 0xF0, 0xE3, 0x0D][..],
                "rorx L reserved",
            ),
            (
                &[0x62, 0xE3, 0xFF, 0x48, 0xF0, 0xE3, 0x0D][..],
                "rorx L-prime reserved",
            ),
            (
                &[0x62, 0xE3, 0xFF, 0x88, 0xF0, 0xE3, 0x0D][..],
                "rorx z reserved",
            ),
            (
                &[0x62, 0xE2, 0xE7, 0x04, 0xF5, 0xE3][..],
                "pdep NF reserved",
            ),
            (&[0x62, 0xE2, 0xE6, 0x00, 0xF6, 0xE3][..], "mulx wrong pp"),
        ] {
            assert!(lift_single(bytes).is_err(), "{name}");
        }
    }

    #[test]
    fn lift_rao_int_legacy_prefixes_like_llvm() {
        for (bytes, name, op, width) in [
            (
                &[0x0F, 0x38, 0xFC, 0x18][..],
                "aaddl",
                AtomicOp::Add,
                MemWidth::B4,
            ),
            (
                &[0x48, 0x0F, 0x38, 0xFC, 0x18][..],
                "aaddq",
                AtomicOp::Add,
                MemWidth::B8,
            ),
            (
                &[0x66, 0x0F, 0x38, 0xFC, 0x18][..],
                "aandl",
                AtomicOp::And,
                MemWidth::B4,
            ),
            (
                &[0xF2, 0x48, 0x0F, 0x38, 0xFC, 0x18][..],
                "aorq",
                AtomicOp::Or,
                MemWidth::B8,
            ),
            (
                &[0xF3, 0x0F, 0x38, 0xFC, 0x18][..],
                "axorl",
                AtomicOp::Xor,
                MemWidth::B4,
            ),
            (
                &[0xF3, 0x48, 0x0F, 0x38, 0xFC, 0x18][..],
                "axorq",
                AtomicOp::Xor,
                MemWidth::B8,
            ),
        ] {
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, bytes.len(), "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            match &result.ops[0].kind {
                OpKind::AtomicRmw {
                    addr: Address::Direct(base),
                    src,
                    op: got_op,
                    width: got_width,
                    order: MemoryOrder::SeqCst,
                    ..
                } => {
                    assert_eq!(*base, x86_gpr(0), "{name}");
                    assert_eq!(*src, x86_gpr(3), "{name}");
                    assert_eq!(*got_op, op, "{name}");
                    assert_eq!(*got_width, width, "{name}");
                }
                other => panic!("expected {name} AtomicRmw, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_rao_int_apx_egpr_memory_like_llvm() {
        // LLVM 20: `aandq %r17, (%r16)` => 62 ec fd 08 fc 08.
        let result = lift_single(&[0x62, 0xEC, 0xFD, 0x08, 0xFC, 0x08]).unwrap();
        assert_eq!(result.bytes_consumed, 6);
        match &result.ops[0].kind {
            OpKind::AtomicRmw {
                addr: Address::Direct(base),
                src,
                op: AtomicOp::And,
                width: MemWidth::B8,
                order: MemoryOrder::SeqCst,
                ..
            } => {
                assert_eq!(*base, x86_gpr(16));
                assert_eq!(*src, x86_gpr(17));
            }
            other => panic!("expected APX AAND AtomicRmw, got {other:?}"),
        }

        // LLVM 20: `aaddl %r19d, 32(%r17,%r18,4)`
        // => 62 ec 78 08 fc 5c 91 20.
        let result = lift_single(&[0x62, 0xEC, 0x78, 0x08, 0xFC, 0x5C, 0x91, 0x20]).unwrap();
        assert_eq!(result.bytes_consumed, 8);
        match &result.ops[0].kind {
            OpKind::AtomicRmw {
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        ..
                    },
                src,
                op: AtomicOp::Add,
                width: MemWidth::B4,
                order: MemoryOrder::SeqCst,
                ..
            } => {
                assert_eq!(*base, x86_gpr(17));
                assert_eq!(*index, x86_gpr(18));
                assert_eq!(*src, x86_gpr(19));
            }
            other => panic!("expected APX AADD complex AtomicRmw, got {other:?}"),
        }
    }

    #[test]
    fn lift_rao_int_rejects_invalid_forms_like_llvm() {
        for (bytes, name) in [
            (&[0x0F, 0x38, 0xFC, 0xD8][..], "legacy register operand"),
            (
                &[0x62, 0xF4, 0xFC, 0x0C, 0xFC, 0x18][..],
                "EVEX NF reserved",
            ),
            (
                &[0x62, 0xF4, 0xFC, 0x18, 0xFC, 0x18][..],
                "EVEX ND reserved",
            ),
            (
                &[0x62, 0xF4, 0xBC, 0x08, 0xFC, 0x18][..],
                "EVEX vvvv reserved",
            ),
            (
                &[0x62, 0xF4, 0xFC, 0x09, 0xFC, 0x18][..],
                "EVEX aaa reserved",
            ),
            (
                &[0x62, 0xF4, 0xFC, 0x08, 0xFC, 0xD8][..],
                "EVEX register operand",
            ),
        ] {
            let err = lift_single(bytes).expect_err(name);
            assert!(
                matches!(err, LiftError::InvalidEncoding { .. }),
                "{name}: {err:?}"
            );
        }
    }

    #[test]
    fn lift_cmpccxadd_vex_conditions_like_llvm() {
        for (opcode, cond) in [
            (0xE0, Condition::Overflow),
            (0xE1, Condition::NoOverflow),
            (0xE2, Condition::Ult),
            (0xE3, Condition::Uge),
            (0xE4, Condition::Eq),
            (0xE5, Condition::Ne),
            (0xE6, Condition::Ule),
            (0xE7, Condition::Ugt),
            (0xE8, Condition::Negative),
            (0xE9, Condition::Positive),
            (0xEA, Condition::Parity),
            (0xEB, Condition::NoParity),
            (0xEC, Condition::Slt),
            (0xED, Condition::Sge),
            (0xEE, Condition::Sle),
            (0xEF, Condition::Sgt),
        ] {
            let bytes = [0xC4, 0xE2, 0x71, opcode, 0x18];
            let result = lift_single(&bytes).unwrap();
            assert_eq!(result.bytes_consumed, 5, "opcode {opcode:02x}");
            assert_eq!(result.ops.len(), 1, "opcode {opcode:02x}");
            match &result.ops[0].kind {
                OpKind::AtomicCmpXadd {
                    dst_old,
                    addr: Address::Direct(base),
                    cmp,
                    add,
                    cond: got_cond,
                    width,
                    order: MemoryOrder::SeqCst,
                } => {
                    assert_eq!(*dst_old, x86_gpr(3), "opcode {opcode:02x}");
                    assert_eq!(*cmp, x86_gpr(3), "opcode {opcode:02x}");
                    assert_eq!(*add, x86_gpr(1), "opcode {opcode:02x}");
                    assert_eq!(*base, x86_gpr(0), "opcode {opcode:02x}");
                    assert_eq!(*got_cond, cond, "opcode {opcode:02x}");
                    assert_eq!(*width, MemWidth::B4, "opcode {opcode:02x}");
                }
                other => panic!("expected CMPccXADD op for {opcode:02x}, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_cmpccxadd_vex_width_and_high_regs_like_llvm() {
        for (bytes, name, width, dst, base, add) in [
            (
                &[0xC4, 0xE2, 0xF1, 0xE2, 0x18][..],
                "cmpbxadd64",
                MemWidth::B8,
                x86_gpr(3),
                x86_gpr(0),
                x86_gpr(1),
            ),
            (
                &[0xC4, 0x42, 0x29, 0xE2, 0x08][..],
                "cmpbxadd32_r8_r9_r10",
                MemWidth::B4,
                x86_gpr(9),
                x86_gpr(8),
                x86_gpr(10),
            ),
        ] {
            let result = lift_single(bytes).unwrap();
            assert_eq!(result.bytes_consumed, 5, "{name}");
            match &result.ops[0].kind {
                OpKind::AtomicCmpXadd {
                    dst_old,
                    addr: Address::Direct(got_base),
                    cmp,
                    add: got_add,
                    cond: Condition::Ult,
                    width: got_width,
                    order: MemoryOrder::SeqCst,
                } => {
                    assert_eq!(*dst_old, dst, "{name}");
                    assert_eq!(*cmp, dst, "{name}");
                    assert_eq!(*got_base, base, "{name}");
                    assert_eq!(*got_add, add, "{name}");
                    assert_eq!(*got_width, width, "{name}");
                }
                other => panic!("expected VEX {name} AtomicCmpXadd, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_cmpccxadd_evex_egpr_memory_like_llvm() {
        let result = lift_single(&[0x62, 0xEA, 0x61, 0x00, 0xE2, 0x44, 0x91, 0x20]).unwrap();
        assert_eq!(result.bytes_consumed, 8);
        assert_eq!(result.ops.len(), 1);
        match &result.ops[0].kind {
            OpKind::AtomicCmpXadd {
                dst_old,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        ..
                    },
                cmp,
                add,
                cond: Condition::Ult,
                width: MemWidth::B4,
                order: MemoryOrder::SeqCst,
            } => {
                assert_eq!(*dst_old, x86_gpr(16));
                assert_eq!(*cmp, x86_gpr(16));
                assert_eq!(*add, x86_gpr(19));
                assert_eq!(*base, x86_gpr(17));
                assert_eq!(*index, x86_gpr(18));
            }
            other => panic!("expected EVEX CMPccXADD AtomicCmpXadd, got {other:?}"),
        }

        let result = lift_single(&[0x62, 0xEA, 0x65, 0x08, 0xE2, 0x08]).unwrap();
        match &result.ops[0].kind {
            OpKind::AtomicCmpXadd {
                dst_old,
                addr: Address::Direct(base),
                cmp,
                add,
                width: MemWidth::B4,
                ..
            } => {
                assert_eq!(*dst_old, x86_gpr(17));
                assert_eq!(*cmp, x86_gpr(17));
                assert_eq!(*base, x86_gpr(16));
                assert_eq!(*add, x86_gpr(3));
            }
            other => panic!("expected EVEX CMPccXADD with legacy addend, got {other:?}"),
        }
    }

    #[test]
    fn lift_cmpccxadd_rejects_invalid_forms_like_llvm() {
        for (bytes, name) in [
            (&[0xC4, 0xE2, 0x75, 0xE2, 0x18][..], "vex_l"),
            (&[0xC4, 0xE2, 0x72, 0xE2, 0x18][..], "vex_pp2"),
            (&[0xC4, 0xE2, 0x71, 0xE2, 0xD8][..], "vex_register_source"),
            (&[0x62, 0xEA, 0xE5, 0x01, 0xE2, 0x08][..], "evex_mask"),
            (&[0x62, 0xEA, 0xE5, 0x04, 0xE2, 0x08][..], "evex_nf"),
            (&[0x62, 0xEA, 0xE5, 0x20, 0xE2, 0x08][..], "evex_l"),
            (
                &[0x62, 0xEA, 0xE5, 0x00, 0xE2, 0xC0][..],
                "evex_register_source",
            ),
        ] {
            assert!(
                lift_single(bytes).is_err(),
                "{name} should be rejected like LLVM"
            );
        }
    }

    fn assert_apx_conditional_flag_shape(
        result: &LiftResult,
        cond: Condition,
        default_rflags: i64,
    ) {
        assert_apx_conditional_flag_shape_with_true_ops(result, cond, default_rflags, 1);
    }

    fn assert_apx_conditional_flag_shape_with_true_ops(
        result: &LiftResult,
        cond: Condition,
        default_rflags: i64,
        true_op_count: usize,
    ) -> VReg {
        let true_flags_idx = 4 + true_op_count;
        let select_idx = true_flags_idx + 1;
        let write_flags_idx = select_idx + 1;
        assert_eq!(result.ops.len(), write_flags_idx + 1);

        let old_flags = match &result.ops[0].kind {
            OpKind::ReadFlags { dst } => *dst,
            other => panic!("expected APX conditional old ReadFlags, got {other:?}"),
        };
        let cond_reg = match &result.ops[1].kind {
            OpKind::SetCC {
                dst,
                cond: got_cond,
                width: OpWidth::W64,
            } => {
                assert_eq!(*got_cond, cond);
                *dst
            }
            other => panic!("expected APX conditional SetCC, got {other:?}"),
        };
        let false_flags = match &result.ops[2].kind {
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Imm(mask),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*src1, old_flags);
                assert_eq!(*mask, !APX_CCMP_FLAGS_MASK);
                *dst
            }
            other => panic!("expected APX conditional false-flag mask, got {other:?}"),
        };
        match &result.ops[3].kind {
            OpKind::Or {
                dst,
                src1,
                src2: SrcOperand::Imm(flags),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, false_flags);
                assert_eq!(*src1, false_flags);
                assert_eq!(*flags, default_rflags);
            }
            other => panic!("expected APX conditional false-flag defaults, got {other:?}"),
        }
        let true_flags = match &result.ops[true_flags_idx].kind {
            OpKind::ReadFlags { dst } => *dst,
            other => panic!("expected APX conditional true ReadFlags, got {other:?}"),
        };
        let selected_flags = match &result.ops[select_idx].kind {
            OpKind::Select {
                dst,
                cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*cond, cond_reg);
                assert_eq!(*src_true, true_flags);
                assert_eq!(*src_false, false_flags);
                *dst
            }
            other => panic!("expected APX conditional flag Select, got {other:?}"),
        };
        match &result.ops[write_flags_idx].kind {
            OpKind::WriteFlags { src } => assert_eq!(*src, selected_flags),
            other => panic!("expected APX conditional WriteFlags, got {other:?}"),
        }
        cond_reg
    }

    fn assert_apx_conditional_predload(
        result: &LiftResult,
        cond_reg: VReg,
        index: usize,
        width: MemWidth,
    ) -> VReg {
        match &result.ops[index].kind {
            OpKind::PredLoad {
                dst,
                cond,
                addr: Address::Direct(base),
                width: got_width,
                signed: SignExtend::Zero,
            } => {
                assert_eq!(*cond, cond_reg);
                assert_eq!(*base, x86_gpr(3));
                assert_eq!(*got_width, width);
                *dst
            }
            other => panic!("expected APX conditional PredLoad, got {other:?}"),
        }
    }

    #[test]
    fn rex2_modrm_decode_extends_to_apx_gprs() {
        let prefix = decode_prefixes(&[0xD5, 0x5D, 0x89, 0xF8]).unwrap();
        let modrm = decode_modrm(&[0xF8], &prefix, 0).unwrap();
        assert!(!modrm.is_memory);
        assert_eq!(modrm.reg, 31);
        assert_eq!(modrm.rm, 24);
    }

    #[test]
    fn lift_rex2_mov_egpr_imm64_uses_llvm_encoding() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `mov r16, 0x1122334455667788`
        let result = lifter
            .lift_insn(
                0x1000,
                &[
                    0xD5, 0x18, 0xB8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11,
                ],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(result.bytes_consumed, 11);
        assert_eq!(result.ops.len(), 1);
        match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Imm64(0x1122_3344_5566_7788),
                width: OpWidth::W64,
            } => assert_eq!(*dst, x86_gpr(16)),
            other => panic!("expected R16 imm64 mov, got {other:?}"),
        }

        // LLVM 20: `mov r24, 0x1122334455667788`
        let result = lifter
            .lift_insn(
                0x1000,
                &[
                    0xD5, 0x19, 0xB8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11,
                ],
                &mut ctx,
            )
            .unwrap();
        match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Imm64(0x1122_3344_5566_7788),
                width: OpWidth::W64,
            } => assert_eq!(*dst, x86_gpr(24)),
            other => panic!("expected R24 imm64 mov, got {other:?}"),
        }
    }

    #[test]
    fn lift_rex2_mov_egpr_reg_uses_llvm_encoding() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `mov r16, rax`
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0x18, 0x89, 0xC0], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 4);
        match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(16));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected mov r16, rax, got {other:?}"),
        }

        // LLVM 20: `mov r16, rax` has the APX register in r/m; `mov rax, r16`
        // uses ModR/M.reg extension instead.
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0x48, 0x89, 0xC0], &mut ctx)
            .unwrap();
        match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src, x86_gpr(16));
            }
            other => panic!("expected mov rax, r16, got {other:?}"),
        }

        // LLVM 20: `mov r24, r31`
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0x5D, 0x89, 0xF8], &mut ctx)
            .unwrap();
        match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(24));
                assert_eq!(*src, x86_gpr(31));
            }
            other => panic!("expected mov r24, r31, got {other:?}"),
        }
    }

    #[test]
    fn lift_rex2_push_pop_egpr_uses_llvm_encoding() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20 accepts this non-canonical payload and disassembles it as
        // `pushp %r16`; the canonical encoding below keeps the APX oracle exact.
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0x10, 0x50], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 3);
        assert_eq!(result.ops.len(), 2);
        match &result.ops[1].kind {
            OpKind::Store {
                src,
                addr: Address::Direct(_),
                width: MemWidth::B8,
            } => assert_eq!(*src, x86_gpr(16)),
            other => panic!("expected push r16 store, got {other:?}"),
        }

        // LLVM 20: `pushp %r16` => d5 18 50.
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0x18, 0x50], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 3);
        match &result.ops[1].kind {
            OpKind::Store {
                src,
                addr: Address::Direct(_),
                width: MemWidth::B8,
            } => assert_eq!(*src, x86_gpr(16)),
            other => panic!("expected pushp r16 store, got {other:?}"),
        }

        // LLVM 20 accepts this non-canonical payload and disassembles it as
        // `popp %r31`; the canonical encoding for a concrete register follows.
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0x11, 0x5F], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 3);
        match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr: Address::Direct(_),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => assert_eq!(*dst, x86_gpr(31)),
            other => panic!("expected pop r31 load, got {other:?}"),
        }

        // LLVM 20: `popp %r16` => d5 18 58.
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0x18, 0x58], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 3);
        match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr: Address::Direct(_),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => assert_eq!(*dst, x86_gpr(16)),
            other => panic!("expected popp r16 load, got {other:?}"),
        }
    }

    #[test]
    fn lift_rex2_m_compressed_0f_map_uses_llvm_encoding() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `imul r16, rax` as REX2.M compressed 0F AF.
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0xC8, 0xAF, 0xC0], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 4);
        match &result.ops[0].kind {
            OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst_lo, x86_gpr(16));
                assert_eq!(*src1, x86_gpr(16));
                assert_eq!(*src2, x86_gpr(0));
            }
            other => panic!("expected imul r16, rax, got {other:?}"),
        }
    }

    #[test]
    fn lift_rex2_cmpxchg_registers_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `cmpxchgq %r17, %r16` => d5 d8 b1 c8.
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0xD8, 0xB1, 0xC8], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 4);
        assert_eq!(result.ops.len(), 7);

        let saved_src = match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(17));
                *dst
            }
            other => panic!("expected CMPXCHG source snapshot, got {other:?}"),
        };
        let saved_acc = match &result.ops[1].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(0));
                *dst
            }
            other => panic!("expected CMPXCHG accumulator snapshot, got {other:?}"),
        };
        let old_dst = match &result.ops[2].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(16));
                *dst
            }
            other => panic!("expected CMPXCHG destination snapshot, got {other:?}"),
        };
        match &result.ops[3].kind {
            OpKind::Cmp {
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src1, saved_acc);
                assert_eq!(*src2, old_dst);
            }
            other => panic!("expected CMPXCHG compare, got {other:?}"),
        }
        let matched = match &result.ops[4].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Eq,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected CMPXCHG equality condition, got {other:?}"),
        };
        match &result.ops[5].kind {
            OpKind::Select {
                dst,
                cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(16));
                assert_eq!(*cond, matched);
                assert_eq!(*src_true, saved_src);
                assert_eq!(*src_false, old_dst);
            }
            other => panic!("expected CMPXCHG destination select, got {other:?}"),
        }
        match &result.ops[6].kind {
            OpKind::Select {
                dst,
                cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*cond, matched);
                assert_eq!(*src_true, saved_acc);
                assert_eq!(*src_false, old_dst);
            }
            other => panic!("expected CMPXCHG accumulator select, got {other:?}"),
        }
    }

    #[test]
    fn lift_rex2_cmpxchg_memory_egpr_sib_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `cmpxchgq %r18, 32(%r16,%r17,4)` => d5 f8 b1 54 88 20.
        let result = lifter
            .lift_insn(0x1000, &[0xD5, 0xF8, 0xB1, 0x54, 0x88, 0x20], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 8);

        let saved_src = match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(18));
                *dst
            }
            other => panic!("expected CMPXCHG memory source snapshot, got {other:?}"),
        };
        let saved_acc = match &result.ops[1].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(0));
                *dst
            }
            other => panic!("expected CMPXCHG memory accumulator snapshot, got {other:?}"),
        };
        let old_dst = match &result.ops[2].kind {
            OpKind::Load {
                dst,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        ..
                    },
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(16));
                assert_eq!(*index, x86_gpr(17));
                *dst
            }
            other => panic!("expected CMPXCHG memory destination load, got {other:?}"),
        };
        match &result.ops[3].kind {
            OpKind::Cmp {
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src1, saved_acc);
                assert_eq!(*src2, old_dst);
            }
            other => panic!("expected CMPXCHG memory compare, got {other:?}"),
        }
        let matched = match &result.ops[4].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Eq,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected CMPXCHG memory equality condition, got {other:?}"),
        };
        let new_dst = match &result.ops[5].kind {
            OpKind::Select {
                dst,
                cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*cond, matched);
                assert_eq!(*src_true, saved_src);
                assert_eq!(*src_false, old_dst);
                *dst
            }
            other => panic!("expected CMPXCHG memory destination select, got {other:?}"),
        };
        match &result.ops[6].kind {
            OpKind::PredStore {
                src: SrcOperand::Reg(src),
                cond,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 4,
                        disp: 0x20,
                        ..
                    },
                width: MemWidth::B8,
            } => {
                assert_eq!(*src, new_dst);
                assert_eq!(*cond, matched);
                assert_eq!(*base, x86_gpr(16));
                assert_eq!(*index, x86_gpr(17));
            }
            other => panic!("expected CMPXCHG predicated memory store, got {other:?}"),
        }
        match &result.ops[7].kind {
            OpKind::Select {
                dst,
                cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*cond, matched);
                assert_eq!(*src_true, saved_acc);
                assert_eq!(*src_false, old_dst);
            }
            other => panic!("expected CMPXCHG memory accumulator select, got {other:?}"),
        }
    }

    #[test]
    fn lift_cmpxchg_accumulator_destination_alias_keeps_single_final_write() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `cmpxchgq %rcx, %rax` => 48 0f b1 c8.
        let result = lifter
            .lift_insn(0x1000, &[0x48, 0x0F, 0xB1, 0xC8], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 4);
        assert_eq!(result.ops.len(), 6);

        let saved_src = match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(1));
                *dst
            }
            other => panic!("expected alias CMPXCHG source snapshot, got {other:?}"),
        };
        let old_dst = match &result.ops[2].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(0));
                *dst
            }
            other => panic!("expected alias CMPXCHG destination snapshot, got {other:?}"),
        };
        let matched = match &result.ops[4].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Eq,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected alias CMPXCHG equality condition, got {other:?}"),
        };
        match &result.ops[5].kind {
            OpKind::Select {
                dst,
                cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*cond, matched);
                assert_eq!(*src_true, saved_src);
                assert_eq!(*src_false, old_dst);
            }
            other => panic!("expected one final alias CMPXCHG write, got {other:?}"),
        }
    }

    fn assert_rex2_xadd_sib_addr(addr: &Address, name: &str) {
        match addr {
            Address::BaseIndexScale {
                base: Some(base),
                index,
                scale: 4,
                disp: 0x20,
                disp_size: DispSize::Disp8,
            } => {
                assert_eq!(*base, x86_gpr(16), "{name}");
                assert_eq!(*index, x86_gpr(17), "{name}");
            }
            other => panic!("expected REX2 {name} SIB address, got {other:?}"),
        }
    }

    fn assert_xadd_register_ops(
        result: &LiftResult,
        name: &str,
        dst_reg: VReg,
        src_reg: VReg,
        width: OpWidth,
    ) {
        assert_eq!(result.ops.len(), 5, "{name}");
        let saved_src = match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: got_width,
            } => {
                assert_eq!(*src, src_reg, "{name}");
                assert_eq!(*got_width, width, "{name}");
                *dst
            }
            other => panic!("expected {name} source snapshot, got {other:?}"),
        };
        let old_dst = match &result.ops[1].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: got_width,
            } => {
                assert_eq!(*src, dst_reg, "{name}");
                assert_eq!(*got_width, width, "{name}");
                *dst
            }
            other => panic!("expected {name} destination snapshot, got {other:?}"),
        };
        let sum = match &result.ops[2].kind {
            OpKind::Add {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: got_width,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*src1, old_dst, "{name}");
                assert_eq!(*src2, saved_src, "{name}");
                assert_eq!(*got_width, width, "{name}");
                *dst
            }
            other => panic!("expected {name} flagged add, got {other:?}"),
        };
        match &result.ops[3].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: got_width,
            } => {
                assert_eq!(*dst, src_reg, "{name}");
                assert_eq!(*src, old_dst, "{name}");
                assert_eq!(*got_width, width, "{name}");
            }
            other => panic!("expected {name} source writeback, got {other:?}"),
        }
        match &result.ops[4].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: got_width,
            } => {
                assert_eq!(*dst, dst_reg, "{name}");
                assert_eq!(*src, sum, "{name}");
                assert_eq!(*got_width, width, "{name}");
            }
            other => panic!("expected {name} destination writeback, got {other:?}"),
        }
    }

    #[test]
    fn lift_rex2_xadd_registers_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let cases: &[(&[u8], &str, usize, VReg, VReg, OpWidth)] = &[
            (
                &[0xD5, 0xD8, 0xC1, 0xC8],
                "xadd_r16_r17",
                4,
                x86_gpr(16),
                x86_gpr(17),
                OpWidth::W64,
            ),
            (
                &[0xD5, 0xD0, 0xC1, 0xC8],
                "xadd_r16d_r17d",
                4,
                x86_gpr(16),
                x86_gpr(17),
                OpWidth::W32,
            ),
            (
                &[0x66, 0xD5, 0xD0, 0xC1, 0xC8],
                "xadd_r16w_r17w",
                5,
                x86_gpr(16),
                x86_gpr(17),
                OpWidth::W16,
            ),
            (
                &[0xD5, 0xD0, 0xC0, 0xC8],
                "xadd_r16b_r17b",
                4,
                x86_gpr(16),
                x86_gpr(17),
                OpWidth::W8,
            ),
            (
                &[0xD5, 0xDD, 0xC1, 0xF8],
                "xadd_r24_r31",
                4,
                x86_gpr(24),
                x86_gpr(31),
                OpWidth::W64,
            ),
        ];

        for (bytes, name, bytes_consumed, dst_reg, src_reg, width) in cases {
            // LLVM 23 examples:
            //   `xadd r16, r17`   => d5 d8 c1 c8
            //   `xadd r16d, r17d` => d5 d0 c1 c8
            //   `xadd r16w, r17w` => 66 d5 d0 c1 c8
            //   `xadd r16b, r17b` => d5 d0 c0 c8
            //   `xadd r24, r31`   => d5 dd c1 f8
            let result = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, *bytes_consumed, "{name}");
            assert_xadd_register_ops(&result, name, *dst_reg, *src_reg, *width);
        }
    }

    #[test]
    fn lift_xadd_same_register_writes_sum_last_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `xadd rax, rax` => 48 0f c1 c0.
        let result = lifter
            .lift_insn(0x1000, &[0x48, 0x0F, 0xC1, 0xC0], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 4);
        assert_xadd_register_ops(
            &result,
            "xadd_rax_rax",
            x86_gpr(0),
            x86_gpr(0),
            OpWidth::W64,
        );
    }

    #[test]
    fn lift_rex2_xadd_memory_egpr_sib_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let cases: &[(&[u8], &str, usize, MemWidth, OpWidth)] = &[
            (
                &[0xD5, 0xF8, 0xC1, 0x54, 0x88, 0x20],
                "xaddq",
                6,
                MemWidth::B8,
                OpWidth::W64,
            ),
            (
                &[0xD5, 0xF0, 0xC1, 0x54, 0x88, 0x20],
                "xaddl",
                6,
                MemWidth::B4,
                OpWidth::W32,
            ),
            (
                &[0x66, 0xD5, 0xF0, 0xC1, 0x54, 0x88, 0x20],
                "xaddw",
                7,
                MemWidth::B2,
                OpWidth::W16,
            ),
            (
                &[0xD5, 0xF0, 0xC0, 0x54, 0x88, 0x20],
                "xaddb",
                6,
                MemWidth::B1,
                OpWidth::W8,
            ),
        ];

        for (bytes, name, bytes_consumed, mem_width, op_width) in cases {
            // LLVM 23 examples:
            //   `xaddq %r18, 32(%r16,%r17,4)` => d5 f8 c1 54 88 20
            //   `xaddl %r18d, 32(%r16,%r17,4)` => d5 f0 c1 54 88 20
            //   `xaddw %r18w, 32(%r16,%r17,4)` => 66 d5 f0 c1 54 88 20
            //   `xaddb %r18b, 32(%r16,%r17,4)` => d5 f0 c0 54 88 20
            let result = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, *bytes_consumed, "{name}");
            assert_eq!(result.ops.len(), 5, "{name}");

            let saved_src = match &result.ops[0].kind {
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Reg(src),
                    width,
                } => {
                    assert_eq!(*src, x86_gpr(18), "{name}");
                    assert_eq!(*width, *op_width, "{name}");
                    *dst
                }
                other => panic!("expected REX2 {name} source snapshot, got {other:?}"),
            };
            let old_dst = match &result.ops[1].kind {
                OpKind::Load {
                    dst,
                    addr,
                    width,
                    sign: SignExtend::Zero,
                } => {
                    assert_rex2_xadd_sib_addr(addr, name);
                    assert_eq!(*width, *mem_width, "{name}");
                    *dst
                }
                other => panic!("expected REX2 {name} memory load, got {other:?}"),
            };
            let sum = match &result.ops[2].kind {
                OpKind::Add {
                    dst,
                    src1,
                    src2: SrcOperand::Reg(src2),
                    width,
                    flags: FlagUpdate::All,
                } => {
                    assert_eq!(*src1, old_dst, "{name}");
                    assert_eq!(*src2, saved_src, "{name}");
                    assert_eq!(*width, *op_width, "{name}");
                    *dst
                }
                other => panic!("expected REX2 {name} flagged add, got {other:?}"),
            };
            match &result.ops[3].kind {
                OpKind::Store { src, addr, width } => {
                    assert_eq!(*src, sum, "{name}");
                    assert_rex2_xadd_sib_addr(addr, name);
                    assert_eq!(*width, *mem_width, "{name}");
                }
                other => panic!("expected REX2 {name} memory store, got {other:?}"),
            }
            match &result.ops[4].kind {
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Reg(src),
                    width,
                } => {
                    assert_eq!(*dst, x86_gpr(18), "{name}");
                    assert_eq!(*src, old_dst, "{name}");
                    assert_eq!(*width, *op_width, "{name}");
                }
                other => panic!("expected REX2 {name} source writeback, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_rex2_lock_xadd_memory_uses_atomic_add_like_spec() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let cases: &[(&[u8], &str, usize, MemWidth, OpWidth)] = &[
            (
                &[0xF0, 0xD5, 0xF8, 0xC1, 0x54, 0x88, 0x20],
                "lock_xaddq",
                7,
                MemWidth::B8,
                OpWidth::W64,
            ),
            (
                &[0xF0, 0xD5, 0xF0, 0xC1, 0x54, 0x88, 0x20],
                "lock_xaddl",
                7,
                MemWidth::B4,
                OpWidth::W32,
            ),
            (
                &[0x66, 0xF0, 0xD5, 0xF0, 0xC1, 0x54, 0x88, 0x20],
                "lock_xaddw",
                8,
                MemWidth::B2,
                OpWidth::W16,
            ),
            (
                &[0xF0, 0xD5, 0xF0, 0xC0, 0x54, 0x88, 0x20],
                "lock_xaddb",
                7,
                MemWidth::B1,
                OpWidth::W8,
            ),
        ];

        for (bytes, name, bytes_consumed, mem_width, op_width) in cases {
            // LLVM 23 examples:
            //   `lock xaddq %r18, 32(%r16,%r17,4)` => f0 d5 f8 c1 54 88 20
            //   `lock xaddl %r18d, 32(%r16,%r17,4)` => f0 d5 f0 c1 54 88 20
            //   `lock xaddw %r18w, 32(%r16,%r17,4)` => 66 f0 d5 f0 c1 54 88 20
            //   `lock xaddb %r18b, 32(%r16,%r17,4)` => f0 d5 f0 c0 54 88 20
            let result = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, *bytes_consumed, "{name}");
            assert_eq!(result.ops.len(), 4, "{name}");

            let saved_src = match &result.ops[0].kind {
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Reg(src),
                    width,
                } => {
                    assert_eq!(*src, x86_gpr(18), "{name}");
                    assert_eq!(*width, *op_width, "{name}");
                    *dst
                }
                other => panic!("expected REX2 {name} source snapshot, got {other:?}"),
            };
            let old_dst = match &result.ops[1].kind {
                OpKind::AtomicRmw {
                    dst,
                    addr,
                    src,
                    op: AtomicOp::Add,
                    width,
                    order: MemoryOrder::SeqCst,
                } => {
                    assert_rex2_xadd_sib_addr(addr, name);
                    assert_eq!(*src, saved_src, "{name}");
                    assert_eq!(*width, *mem_width, "{name}");
                    *dst
                }
                other => panic!("expected REX2 {name} AtomicRmw Add, got {other:?}"),
            };
            match &result.ops[2].kind {
                OpKind::Add {
                    dst: _,
                    src1,
                    src2: SrcOperand::Reg(src2),
                    width,
                    flags: FlagUpdate::All,
                } => {
                    assert_eq!(*src1, old_dst, "{name}");
                    assert_eq!(*src2, saved_src, "{name}");
                    assert_eq!(*width, *op_width, "{name}");
                }
                other => panic!("expected REX2 {name} flag-producing add, got {other:?}"),
            }
            match &result.ops[3].kind {
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Reg(src),
                    width,
                } => {
                    assert_eq!(*dst, x86_gpr(18), "{name}");
                    assert_eq!(*src, old_dst, "{name}");
                    assert_eq!(*width, *op_width, "{name}");
                }
                other => panic!("expected REX2 {name} source writeback, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_lock_xadd_register_rejected_like_spec() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // Intel XADD specifies #UD when LOCK is used without a memory destination.
        for bytes in [
            &[0xF0, 0x48, 0x0F, 0xC1, 0xC0][..],
            &[0xF0, 0xD5, 0xD8, 0xC1, 0xC8][..],
        ] {
            let err = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap_err();
            assert!(matches!(err, LiftError::InvalidEncoding { .. }), "{err:?}");
        }
    }

    fn assert_bswap_op(result: &LiftResult, name: &str, reg: VReg, width: OpWidth) {
        assert_eq!(result.ops.len(), 1, "{name}");
        match &result.ops[0].kind {
            OpKind::Bswap {
                dst,
                src,
                width: got_width,
            } => {
                assert_eq!(*dst, reg, "{name}");
                assert_eq!(*src, reg, "{name}");
                assert_eq!(*got_width, width, "{name}");
            }
            other => panic!("expected {name} Bswap, got {other:?}"),
        }
    }

    #[test]
    fn lift_bswap_registers_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let cases: &[(&[u8], &str, usize, VReg, OpWidth)] = &[
            (&[0x0F, 0xC8], "bswap_eax", 2, x86_gpr(0), OpWidth::W32),
            (
                &[0x48, 0x0F, 0xC8],
                "bswap_rax",
                3,
                x86_gpr(0),
                OpWidth::W64,
            ),
            (
                &[0xD5, 0x90, 0xC8],
                "bswap_r16d",
                3,
                x86_gpr(16),
                OpWidth::W32,
            ),
            (
                &[0xD5, 0x98, 0xC8],
                "bswap_r16",
                3,
                x86_gpr(16),
                OpWidth::W64,
            ),
            (
                &[0xD5, 0x99, 0xC8],
                "bswap_r24",
                3,
                x86_gpr(24),
                OpWidth::W64,
            ),
            (
                &[0xD5, 0x91, 0xCF],
                "bswap_r31d",
                3,
                x86_gpr(31),
                OpWidth::W32,
            ),
            (
                &[0xD5, 0x99, 0xCF],
                "bswap_r31",
                3,
                x86_gpr(31),
                OpWidth::W64,
            ),
        ];

        for (bytes, name, bytes_consumed, reg, width) in cases {
            // LLVM 23 examples:
            //   `bswap eax`   => 0f c8
            //   `bswap rax`   => 48 0f c8
            //   `bswap r16d`  => d5 90 c8
            //   `bswap r16`   => d5 98 c8
            //   `bswap r24`   => d5 99 c8
            //   `bswap r31d`  => d5 91 cf
            //   `bswap r31`   => d5 99 cf
            let result = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, *bytes_consumed, "{name}");
            assert_bswap_op(&result, name, *reg, *width);
        }
    }

    #[test]
    fn lift_bswap_16bit_rejected_like_spec() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // Intel documents BSWAP r16 as undefined; LLVM 23 rejects the assembly
        // form even though its disassembler can print raw data16-prefixed bytes.
        for bytes in [&[0x66, 0x0F, 0xC8][..], &[0x66, 0xD5, 0x90, 0xC8][..]] {
            let err = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap_err();
            assert!(matches!(err, LiftError::InvalidEncoding { .. }), "{err:?}");
        }
    }

    #[test]
    fn lift_rex2_xchg_registers_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, reg1, reg2) in [
            (
                [0xD5, 0x58, 0x87, 0xC1],
                "xchg_r16_r17",
                x86_gpr(17),
                x86_gpr(16),
            ),
            (
                [0xD5, 0x5D, 0x87, 0xC7],
                "xchg_r24_r31",
                x86_gpr(31),
                x86_gpr(24),
            ),
        ] {
            let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 4, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            match &result.ops[0].kind {
                OpKind::Xchg {
                    reg1: got_reg1,
                    reg2: got_reg2,
                    width: OpWidth::W64,
                } => {
                    assert_eq!(*got_reg1, reg1, "{name}");
                    assert_eq!(*got_reg2, reg2, "{name}");
                }
                other => panic!("expected REX2 {name} Xchg, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_rex2_xchg_memory_uses_atomic_swap_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, mem_width, op_width) in [
            (
                [0xD5, 0x78, 0x87, 0x54, 0x88, 0x20],
                "xchgq",
                MemWidth::B8,
                OpWidth::W64,
            ),
            (
                [0xD5, 0x70, 0x86, 0x54, 0x88, 0x20],
                "xchgb",
                MemWidth::B1,
                OpWidth::W8,
            ),
        ] {
            // LLVM 23:
            //   `xchgq %r18, 32(%r16,%r17,4)` => d5 78 87 54 88 20
            //   `xchgb %r18b, 32(%r16,%r17,4)` => d5 70 86 54 88 20
            let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 6, "{name}");
            assert_eq!(result.ops.len(), 2, "{name}");

            let old_mem = match &result.ops[0].kind {
                OpKind::AtomicRmw {
                    dst,
                    addr:
                        Address::BaseIndexScale {
                            base: Some(base),
                            index,
                            scale: 4,
                            disp: 0x20,
                            disp_size: DispSize::Disp8,
                        },
                    src,
                    op: AtomicOp::Swap,
                    width,
                    order: MemoryOrder::SeqCst,
                } => {
                    assert_eq!(*base, x86_gpr(16), "{name}");
                    assert_eq!(*index, x86_gpr(17), "{name}");
                    assert_eq!(*src, x86_gpr(18), "{name}");
                    assert_eq!(*width, mem_width, "{name}");
                    *dst
                }
                other => panic!("expected REX2 {name} AtomicRmw Swap, got {other:?}"),
            };

            match &result.ops[1].kind {
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Reg(src),
                    width,
                } => {
                    assert_eq!(*dst, x86_gpr(18), "{name}");
                    assert_eq!(*src, old_mem, "{name}");
                    assert_eq!(*width, op_width, "{name}");
                }
                other => panic!("expected REX2 {name} register writeback, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_rex2_jmpabs_uses_llvm_encoding() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `jmpabs 0x1122334455667788` as REX2 + A1 imm64.
        let result = lifter
            .lift_insn(
                0x1000,
                &[
                    0xD5, 0x00, 0xA1, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11,
                ],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(result.bytes_consumed, 11);
        assert!(result.ops.is_empty());
        match result.control_flow {
            ControlFlow::Branch {
                target: 0x1122_3344_5566_7788,
            } => {}
            other => panic!("expected JMPABS direct branch, got {other:?}"),
        }
    }

    #[test]
    fn lift_rex2_jmpabs_ignores_w_bit_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20 also decodes REX2.W + A1 as JMPABS, not MOV rAX,moffs.
        let result = lifter
            .lift_insn(
                0x1000,
                &[
                    0xD5, 0x08, 0xA1, 0x21, 0x43, 0x65, 0x87, 0x78, 0x56, 0x34, 0x12,
                ],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(result.bytes_consumed, 11);
        match result.control_flow {
            ControlFlow::Branch {
                target: 0x1234_5678_8765_4321,
            } => {}
            other => panic!("expected JMPABS direct branch, got {other:?}"),
        }
    }

    #[test]
    fn lift_rex2_jmpabs_requires_imm64() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let err = lifter
            .lift_insn(0x1000, &[0xD5, 0x00, 0xA1, 0x88, 0x77], &mut ctx)
            .unwrap_err();
        assert!(matches!(
            err,
            LiftError::Incomplete {
                addr: 0x1000,
                have: 2,
                need: 8
            }
        ));
    }

    #[test]
    fn lift_apx_ndd_group1_immediates_use_vvvv_destination() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (group, name) in [
            (0u8, "add"),
            (1u8, "or"),
            (2u8, "adc"),
            (3u8, "sbb"),
            (4u8, "and"),
            (5u8, "sub"),
            (6u8, "xor"),
        ] {
            // LLVM 23 APX NDD-style prefix: W64, ND, destination in vvvv = r8.
            // ModR/M r/m is rax, so the lifted shape is `r8 = rax <op> -16`.
            let result = lifter
                .lift_insn(
                    0x1000,
                    &[0x62, 0xF4, 0xBC, 0x18, 0x83, 0xC0 | (group << 3), 0xF0],
                    &mut ctx,
                )
                .unwrap();
            assert_eq!(result.bytes_consumed, 7, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");

            match (name, &result.ops[0].kind) {
                (
                    "add",
                    OpKind::Add {
                        dst,
                        src1,
                        src2: SrcOperand::Imm(-16),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "or",
                    OpKind::Or {
                        dst,
                        src1,
                        src2: SrcOperand::Imm(-16),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "adc",
                    OpKind::Adc {
                        dst,
                        src1,
                        src2: SrcOperand::Imm(-16),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "sbb",
                    OpKind::Sbb {
                        dst,
                        src1,
                        src2: SrcOperand::Imm(-16),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "and",
                    OpKind::And {
                        dst,
                        src1,
                        src2: SrcOperand::Imm(-16),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "sub",
                    OpKind::Sub {
                        dst,
                        src1,
                        src2: SrcOperand::Imm(-16),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "xor",
                    OpKind::Xor {
                        dst,
                        src1,
                        src2: SrcOperand::Imm(-16),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                ) => {
                    assert_eq!(*dst, x86_gpr(8), "{name}");
                    assert_eq!(*src1, x86_gpr(0), "{name}");
                }
                other => panic!("expected APX NDD {name} imm8, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_ndd_adc_sbb_use_carry_ops_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            ([0x62, 0xF4, 0xBC, 0x18, 0x11, 0xD8], "adc"),
            ([0x62, 0xF4, 0xBC, 0x18, 0x19, 0xD8], "sbb"),
        ] {
            // LLVM 20:
            //   adcq %rbx, %rax, %r8 => 62 f4 bc 18 11 d8
            //   sbbq %rbx, %rax, %r8 => 62 f4 bc 18 19 d8
            let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 6, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");

            match (name, &result.ops[0].kind) {
                (
                    "adc",
                    OpKind::Adc {
                        dst,
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "sbb",
                    OpKind::Sbb {
                        dst,
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                ) => {
                    assert_eq!(*dst, x86_gpr(8), "{name}");
                    assert_eq!(*src1, x86_gpr(0), "{name}");
                    assert_eq!(*src2, x86_gpr(3), "{name}");
                }
                other => panic!("expected APX NDD {name}, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_nf_adc_sbb_rejected_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            ([0x62, 0xF4, 0xBC, 0x1C, 0x11, 0xD8], "adc"),
            ([0x62, 0xF4, 0xBC, 0x1C, 0x19, 0xD8], "sbb"),
        ] {
            // LLVM 20 rejects `{nf} adc r8, rax, rbx` and
            // `{nf} sbb r8, rax, rbx`; do not silently lift them as no-flag
            // carry/borrow operations.
            let err = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::Unsupported { .. }),
                "{name}: {err:?}"
            );
        }

        for (bytes, name) in [
            ([0x62, 0xF4, 0xBC, 0x1C, 0x83, 0xD0, 0x01], "adc imm"),
            ([0x62, 0xF4, 0xBC, 0x1C, 0x83, 0xD8, 0x01], "sbb imm"),
        ] {
            let err = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::Unsupported { .. }),
                "{name}: {err:?}"
            );
        }
    }

    #[test]
    fn lift_apx_ndd_shift_rotate_use_group2_ops_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, amount) in [
            (
                [0x62, 0xF4, 0xBC, 0x18, 0xC1, 0xE0, 0x04],
                "shl",
                SrcOperand::Imm(4),
            ),
            (
                [0x62, 0xF4, 0xBC, 0x18, 0xD3, 0xE8, 0x00],
                "shr",
                SrcOperand::Reg(x86_gpr(1)),
            ),
            (
                [0x62, 0xF4, 0xBC, 0x18, 0xD1, 0xF8, 0x00],
                "sar",
                SrcOperand::Imm(1),
            ),
            (
                [0x62, 0xF4, 0xBC, 0x18, 0xC1, 0xC0, 0x07],
                "rol",
                SrcOperand::Imm(7),
            ),
            (
                [0x62, 0xF4, 0xBC, 0x18, 0xD3, 0xC8, 0x00],
                "ror",
                SrcOperand::Reg(x86_gpr(1)),
            ),
        ] {
            // LLVM 20 APX MAP4 NDD forms:
            //   shlq $4,  %rax, %r8 => 62 f4 bc 18 c1 e0 04
            //   shrq %cl, %rax, %r8 => 62 f4 bc 18 d3 e8
            //   sarq      %rax, %r8 => 62 f4 bc 18 d1 f8
            //   rolq $7,  %rax, %r8 => 62 f4 bc 18 c1 c0 07
            //   rorq %cl, %rax, %r8 => 62 f4 bc 18 d3 c8
            let len = if bytes[4] == 0xC1 { 7 } else { 6 };
            let result = lifter.lift_insn(0x1000, &bytes[..len], &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, len, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");

            match (name, &result.ops[0].kind) {
                (
                    "shl",
                    OpKind::Shl {
                        dst,
                        src,
                        amount: got_amount,
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "shr",
                    OpKind::Shr {
                        dst,
                        src,
                        amount: got_amount,
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                )
                | (
                    "sar",
                    OpKind::Sar {
                        dst,
                        src,
                        amount: got_amount,
                        width: OpWidth::W64,
                        flags: FlagUpdate::All,
                    },
                ) => {
                    assert_eq!(*dst, x86_gpr(8), "{name}");
                    assert_eq!(*src, x86_gpr(0), "{name}");
                    assert_eq!(*got_amount, amount, "{name}");
                }
                (
                    "rol",
                    OpKind::Rol {
                        dst,
                        src,
                        amount: got_amount,
                        width: OpWidth::W64,
                        flags,
                    },
                )
                | (
                    "ror",
                    OpKind::Ror {
                        dst,
                        src,
                        amount: got_amount,
                        width: OpWidth::W64,
                        flags,
                    },
                ) => {
                    assert_eq!(*dst, x86_gpr(8), "{name}");
                    assert_eq!(*src, x86_gpr(0), "{name}");
                    assert_eq!(*got_amount, amount, "{name}");
                    assert_eq!(*flags, x86_rotate_flags(), "{name}");
                }
                other => panic!("expected APX NDD {name}, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_shift_widths_nf_memory_and_cl_alias_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `shl r8d, eax, 4` => 62 f4 3c 18 c1 e0 04.
        let shl32 = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0x3C, 0x18, 0xC1, 0xE0, 0x04],
                &mut ctx,
            )
            .unwrap();
        match &shl32.ops[0].kind {
            OpKind::Shl {
                dst,
                src,
                amount: SrcOperand::Imm(4),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX NDD shl r32, got {other:?}"),
        }

        // LLVM 20: `shl r8b, al, 4` => 62 f4 3c 18 c0 e0 04.
        let shl8 = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0x3C, 0x18, 0xC0, 0xE0, 0x04],
                &mut ctx,
            )
            .unwrap();
        match &shl8.ops[0].kind {
            OpKind::Shl {
                dst,
                src,
                amount: SrcOperand::Imm(4),
                width: OpWidth::W8,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX NDD shl r8, got {other:?}"),
        }

        // LLVM 20: `{nf} shr r8, rax, cl` => 62 f4 bc 1c d3 e8.
        let nf = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x1C, 0xD3, 0xE8], &mut ctx)
            .unwrap();
        match &nf.ops[0].kind {
            OpKind::Shr {
                dst,
                src,
                amount: SrcOperand::Reg(amount),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
                assert_eq!(*amount, x86_gpr(1));
            }
            other => panic!("expected APX NF NDD shr, got {other:?}"),
        }

        // LLVM 20: `shl r8, qword ptr [rax], 4` => 62 f4 bc 18 c1 20 04.
        let mem = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0xBC, 0x18, 0xC1, 0x20, 0x04],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(mem.ops.len(), 2);
        let tmp = match &mem.ops[0].kind {
            OpKind::Load {
                dst,
                addr: Address::Direct(base),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(0));
                *dst
            }
            other => panic!("expected APX shift memory source load, got {other:?}"),
        };
        match &mem.ops[1].kind {
            OpKind::Shl {
                dst,
                src,
                amount: SrcOperand::Imm(4),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, tmp);
            }
            other => panic!("expected APX memory-source shift, got {other:?}"),
        }

        // LLVM 20: `shl rcx, rax, cl` => 62 f4 f4 18 d3 e0. Capture CL before
        // the NDD result can overwrite RCX.
        let alias = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xF4, 0x18, 0xD3, 0xE0], &mut ctx)
            .unwrap();
        assert_eq!(alias.ops.len(), 2);
        let tmp = match &alias.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W8,
            } => {
                assert_eq!(*src, x86_gpr(1));
                *dst
            }
            other => panic!("expected CL capture before NDD shift, got {other:?}"),
        };
        match &alias.ops[1].kind {
            OpKind::Shl {
                dst,
                src,
                amount: SrcOperand::Reg(amount),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(1));
                assert_eq!(*src, x86_gpr(0));
                assert_eq!(*amount, tmp);
            }
            other => panic!("expected APX NDD shift with captured CL, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_double_shifts_use_shld_shrd_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `shldq $4, %rbx, %rax, %r8` => 62 f4 bc 18 24 d8 04.
        let shld = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0xBC, 0x18, 0x24, 0xD8, 0x04],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(shld.bytes_consumed, 7);
        assert_eq!(shld.ops.len(), 2);
        match &shld.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX SHLD source1 seed, got {other:?}"),
        }
        match &shld.ops[1].kind {
            OpKind::Shld {
                dst,
                src,
                amount: SrcOperand::Imm(4),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(3));
            }
            other => panic!("expected APX NDD shld, got {other:?}"),
        }

        // LLVM 20: `shrdq %cl, %rbx, %rax, %r8` => 62 f4 bc 18 ad d8.
        let shrd = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x18, 0xAD, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(shrd.bytes_consumed, 6);
        assert_eq!(shrd.ops.len(), 2);
        match &shrd.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX SHRD source1 seed, got {other:?}"),
        }
        match &shrd.ops[1].kind {
            OpKind::Shrd {
                dst,
                src,
                amount: SrcOperand::Reg(amount),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(3));
                assert_eq!(*amount, x86_gpr(1));
            }
            other => panic!("expected APX NDD shrd, got {other:?}"),
        }

        // LLVM 20: `{nf} shldq $4, %rbx, %rax, %r8` => 62 f4 bc 1c 24 d8 04.
        let nf = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0xBC, 0x1C, 0x24, 0xD8, 0x04],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(nf.bytes_consumed, 7);
        assert_eq!(nf.ops.len(), 2);
        match &nf.ops[1].kind {
            OpKind::Shld {
                dst,
                src,
                amount: SrcOperand::Imm(4),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(3));
            }
            other => panic!("expected APX NF NDD shld, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_double_shift_aliases_preserve_inputs_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `shldq $4, %rbx, %rax, %rbx` => 62 f4 e4 18 24 d8 04.
        let src_alias = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0xE4, 0x18, 0x24, 0xD8, 0x04],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(src_alias.bytes_consumed, 7);
        assert_eq!(src_alias.ops.len(), 3);
        let captured_src = match &src_alias.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert!(matches!(dst, VReg::Virtual(_)));
                assert_eq!(*src, x86_gpr(3));
                *dst
            }
            other => panic!("expected APX SHLD source2 capture, got {other:?}"),
        };
        match &src_alias.ops[1].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(3));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX SHLD source1 seed, got {other:?}"),
        }
        match &src_alias.ops[2].kind {
            OpKind::Shld {
                dst,
                src,
                amount: SrcOperand::Imm(4),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(3));
                assert_eq!(*src, captured_src);
            }
            other => panic!("expected APX NDD shld with captured source2, got {other:?}"),
        }

        // LLVM 20: `shrdq %cl, %rbx, %rax, %rcx` => 62 f4 f4 18 ad d8.
        let cl_alias = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xF4, 0x18, 0xAD, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(cl_alias.bytes_consumed, 6);
        assert_eq!(cl_alias.ops.len(), 3);
        let tmp_dst = match &cl_alias.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert!(matches!(dst, VReg::Virtual(_)));
                assert_eq!(*src, x86_gpr(0));
                *dst
            }
            other => panic!("expected APX SHRD temp destination seed, got {other:?}"),
        };
        match &cl_alias.ops[1].kind {
            OpKind::Shrd {
                dst,
                src,
                amount: SrcOperand::Reg(amount),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, tmp_dst);
                assert_eq!(*src, x86_gpr(3));
                assert_eq!(*amount, x86_gpr(1));
            }
            other => panic!("expected APX NDD shrd on temp destination, got {other:?}"),
        }
        match &cl_alias.ops[2].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(1));
                assert_eq!(*src, tmp_dst);
            }
            other => panic!("expected APX SHRD result move into RCX, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_nf_imul_reg_uses_muls_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, flags) in [
            ([0x62, 0xF4, 0xBC, 0x18, 0xAF, 0xC3], "ndd", FlagUpdate::All),
            (
                [0x62, 0xF4, 0xBC, 0x1C, 0xAF, 0xC3],
                "ndd_nf",
                FlagUpdate::None,
            ),
        ] {
            let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 6, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            match &result.ops[0].kind {
                OpKind::MulS {
                    dst_lo,
                    dst_hi: None,
                    src1,
                    src2: SrcOperand::Reg(src2),
                    width: OpWidth::W64,
                    flags: got_flags,
                } => {
                    assert_eq!(*dst_lo, x86_gpr(8), "{name}");
                    assert_eq!(*src1, x86_gpr(0), "{name}");
                    assert_eq!(*src2, x86_gpr(3), "{name}");
                    assert_eq!(*got_flags, flags, "{name}");
                }
                other => panic!("expected APX {name} IMUL MulS, got {other:?}"),
            }
        }

        // LLVM 20: `{nf} imulq %rbx, %rax` => 62 f4 fc 0c af c3.
        let nf = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x0C, 0xAF, 0xC3], &mut ctx)
            .unwrap();
        assert_eq!(nf.bytes_consumed, 6);
        assert_eq!(nf.ops.len(), 1);
        match &nf.ops[0].kind {
            OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst_lo, x86_gpr(0));
                assert_eq!(*src1, x86_gpr(0));
                assert_eq!(*src2, x86_gpr(3));
            }
            other => panic!("expected APX NF IMUL MulS, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_imul_immediates_use_evex_destination_and_flags() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `{nf} imulq $7, %rax, %r8` => 62 74 fc 0c 6b c0 07.
        let nf_imm8 = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0x74, 0xFC, 0x0C, 0x6B, 0xC0, 0x07],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(nf_imm8.bytes_consumed, 7);
        assert_eq!(nf_imm8.ops.len(), 1);
        match &nf_imm8.ops[0].kind {
            OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Imm(7),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst_lo, x86_gpr(8));
                assert_eq!(*src1, x86_gpr(0));
            }
            other => panic!("expected APX NF IMUL imm8 MulS, got {other:?}"),
        }

        // LLVM 20: `{nf} imulq $0x12345678, %rax, %r8`.
        let nf_imm32 = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0x74, 0xFC, 0x0C, 0x69, 0xC0, 0x78, 0x56, 0x34, 0x12],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(nf_imm32.bytes_consumed, 10);
        match &nf_imm32.ops[0].kind {
            OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Imm(0x1234_5678),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst_lo, x86_gpr(8));
                assert_eq!(*src1, x86_gpr(0));
            }
            other => panic!("expected APX NF IMUL imm32 MulS, got {other:?}"),
        }

        // APX NDD immediate form uses vvvv as the destination. LLVM prefers the
        // non-NDD EVEX encoding for this syntax because legacy IMUL already has
        // an independent immediate destination.
        let ndd_imm8 = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0xBC, 0x18, 0x6B, 0xC0, 0xF9],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(ndd_imm8.bytes_consumed, 7);
        match &ndd_imm8.ops[0].kind {
            OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Imm(-7),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst_lo, x86_gpr(8));
                assert_eq!(*src1, x86_gpr(0));
            }
            other => panic!("expected APX NDD IMUL imm8 MulS, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_imul_alias_preserves_r_m_source_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `imulq %rbx, %rax, %rbx` => 62 f4 e4 18 af c3.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xE4, 0x18, 0xAF, 0xC3], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 2);
        let captured_src = match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert!(matches!(dst, VReg::Virtual(_)));
                assert_eq!(*src, x86_gpr(3));
                *dst
            }
            other => panic!("expected APX IMUL r/m source capture, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst_lo, x86_gpr(3));
                assert_eq!(*src1, x86_gpr(0));
                assert_eq!(*src2, captured_src);
            }
            other => panic!("expected APX NDD IMUL with captured source, got {other:?}"),
        }
    }

    #[test]
    fn lift_0f38_movbe_load_widths_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let cases: &[(&[u8], &str, usize, MemWidth, OpWidth, VReg, VReg)] = &[
            (
                &[0x66, 0x0F, 0x38, 0xF0, 0x00],
                "movbe_load16",
                5,
                MemWidth::B2,
                OpWidth::W16,
                x86_gpr(0),
                x86_gpr(0),
            ),
            (
                &[0x0F, 0x38, 0xF0, 0x00],
                "movbe_load32",
                4,
                MemWidth::B4,
                OpWidth::W32,
                x86_gpr(0),
                x86_gpr(0),
            ),
            (
                &[0x48, 0x0F, 0x38, 0xF0, 0x07],
                "movbe_load64",
                5,
                MemWidth::B8,
                OpWidth::W64,
                x86_gpr(0),
                x86_gpr(7),
            ),
        ];

        for (bytes, name, bytes_consumed, mem_width, op_width, dst_reg, base_reg) in cases {
            // LLVM 23 examples:
            //   `movbe ax, word ptr [rax]`    => 66 0f 38 f0 00
            //   `movbe eax, dword ptr [rax]` => 0f 38 f0 00
            //   `movbe rax, qword ptr [rdi]` => 48 0f 38 f0 07
            let result = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, *bytes_consumed, "{name}");
            assert_eq!(result.ops.len(), 2, "{name}");

            let loaded = match &result.ops[0].kind {
                OpKind::Load {
                    dst,
                    addr: Address::Direct(base),
                    width,
                    sign: SignExtend::Zero,
                } => {
                    assert_eq!(*base, *base_reg, "{name}");
                    assert_eq!(*width, *mem_width, "{name}");
                    *dst
                }
                other => panic!("expected {name} memory load, got {other:?}"),
            };
            match &result.ops[1].kind {
                OpKind::Bswap { dst, src, width } => {
                    assert_eq!(*dst, *dst_reg, "{name}");
                    assert_eq!(*src, loaded, "{name}");
                    assert_eq!(*width, *op_width, "{name}");
                }
                other => panic!("expected {name} loaded Bswap, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_0f38_movbe_store_widths_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let cases: &[(&[u8], &str, usize, MemWidth, OpWidth, VReg, VReg)] = &[
            (
                &[0x66, 0x0F, 0x38, 0xF1, 0x08],
                "movbe_store16",
                5,
                MemWidth::B2,
                OpWidth::W16,
                x86_gpr(1),
                x86_gpr(0),
            ),
            (
                &[0x0F, 0x38, 0xF1, 0x08],
                "movbe_store32",
                4,
                MemWidth::B4,
                OpWidth::W32,
                x86_gpr(1),
                x86_gpr(0),
            ),
            (
                &[0x48, 0x0F, 0x38, 0xF1, 0x17],
                "movbe_store64",
                5,
                MemWidth::B8,
                OpWidth::W64,
                x86_gpr(2),
                x86_gpr(7),
            ),
        ];

        for (bytes, name, bytes_consumed, mem_width, op_width, src_reg, base_reg) in cases {
            // LLVM 23 examples:
            //   `movbe word ptr [rax], cx`    => 66 0f 38 f1 08
            //   `movbe dword ptr [rax], ecx` => 0f 38 f1 08
            //   `movbe qword ptr [rdi], rdx` => 48 0f 38 f1 17
            let result = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, *bytes_consumed, "{name}");
            assert_eq!(result.ops.len(), 2, "{name}");

            let swapped = match &result.ops[0].kind {
                OpKind::Bswap { dst, src, width } => {
                    assert!(matches!(dst, VReg::Virtual(_)), "{name}");
                    assert_eq!(*src, *src_reg, "{name}");
                    assert_eq!(*width, *op_width, "{name}");
                    *dst
                }
                other => panic!("expected {name} store Bswap, got {other:?}"),
            };
            match &result.ops[1].kind {
                OpKind::Store { src, addr, width } => {
                    assert_eq!(*src, swapped, "{name}");
                    assert_eq!(*width, *mem_width, "{name}");
                    match addr {
                        Address::Direct(base) => assert_eq!(*base, *base_reg, "{name}"),
                        other => panic!("expected {name} direct address, got {other:?}"),
                    }
                }
                other => panic!("expected {name} memory store, got {other:?}"),
            }
        }
    }

    fn assert_0f38_movbe_rex_sib_addr(addr: &Address, name: &str) {
        match addr {
            Address::BaseIndexScale {
                base: Some(base),
                index,
                scale: 4,
                disp: 0x20,
                disp_size: DispSize::Disp8,
            } => {
                assert_eq!(*base, x86_gpr(8), "{name}");
                assert_eq!(*index, x86_gpr(9), "{name}");
            }
            other => panic!("expected {name} REX SIB address, got {other:?}"),
        }
    }

    #[test]
    fn lift_0f38_movbe_rex_extended_memory_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `movbe r10, qword ptr [r8 + 4*r9 + 32]`
        //          => 4f 0f 38 f0 54 88 20.
        let load = lifter
            .lift_insn(
                0x1000,
                &[0x4F, 0x0F, 0x38, 0xF0, 0x54, 0x88, 0x20],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(load.bytes_consumed, 7);
        assert_eq!(load.ops.len(), 2);
        let loaded = match &load.ops[0].kind {
            OpKind::Load {
                dst,
                addr,
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_0f38_movbe_rex_sib_addr(addr, "movbe_load_rex");
                *dst
            }
            other => panic!("expected MOVBE REX memory load, got {other:?}"),
        };
        match &load.ops[1].kind {
            OpKind::Bswap {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(10));
                assert_eq!(*src, loaded);
            }
            other => panic!("expected MOVBE REX loaded Bswap, got {other:?}"),
        }

        // LLVM 23: `movbe qword ptr [r8 + 4*r9 + 32], r10`
        //          => 4f 0f 38 f1 54 88 20.
        let store = lifter
            .lift_insn(
                0x2000,
                &[0x4F, 0x0F, 0x38, 0xF1, 0x54, 0x88, 0x20],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(store.bytes_consumed, 7);
        assert_eq!(store.ops.len(), 2);
        let swapped = match &store.ops[0].kind {
            OpKind::Bswap {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert!(matches!(dst, VReg::Virtual(_)));
                assert_eq!(*src, x86_gpr(10));
                *dst
            }
            other => panic!("expected MOVBE REX store Bswap, got {other:?}"),
        };
        match &store.ops[1].kind {
            OpKind::Store {
                src,
                addr,
                width: MemWidth::B8,
            } => {
                assert_eq!(*src, swapped);
                assert_0f38_movbe_rex_sib_addr(addr, "movbe_store_rex");
            }
            other => panic!("expected MOVBE REX memory store, got {other:?}"),
        }
    }

    #[test]
    fn lift_0f38_movbe_rejects_invalid_forms_like_spec() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            (&[0x0F, 0x38, 0xF0, 0xC0][..], "load register operand"),
            (&[0x0F, 0x38, 0xF1, 0xC0][..], "store register operand"),
            (&[0xF0, 0x0F, 0x38, 0xF0, 0x00][..], "lock prefix"),
            (&[0xF3, 0x0F, 0x38, 0xF0, 0x00][..], "rep prefix"),
            (
                &[0xD5, 0xF8, 0x38, 0xF0, 0x54, 0x88, 0x20][..],
                "rex2 compressed 0f38",
            ),
        ] {
            let err = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::InvalidEncoding { .. }),
                "{name}: {err:?}"
            );
        }

        let err = lifter
            .lift_insn(0x1000, &[0xF2, 0x0F, 0x38, 0xF0, 0xC3], &mut ctx)
            .unwrap_err();
        assert!(matches!(err, LiftError::Unsupported { .. }), "{err:?}");
    }

    #[test]
    fn lift_apx_movbe_reg_reg_uses_bswap_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, expected_dst, expected_src, width) in [
            (
                [0x62, 0xD4, 0xFC, 0x08, 0x61, 0xC0],
                "movbe64",
                x86_gpr(8),
                x86_gpr(0),
                OpWidth::W64,
            ),
            (
                [0x62, 0xD4, 0x7C, 0x08, 0x61, 0xC0],
                "movbe32",
                x86_gpr(8),
                x86_gpr(0),
                OpWidth::W32,
            ),
            (
                [0x62, 0xD4, 0x7D, 0x08, 0x61, 0xC0],
                "movbe16",
                x86_gpr(8),
                x86_gpr(0),
                OpWidth::W16,
            ),
            (
                [0x62, 0xF4, 0xFC, 0x08, 0x60, 0xD8],
                "movbe60_reg_reg",
                x86_gpr(3),
                x86_gpr(0),
                OpWidth::W64,
            ),
        ] {
            let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 6, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            match &result.ops[0].kind {
                OpKind::Bswap {
                    dst: got_dst,
                    src: got_src,
                    width: got_width,
                } => {
                    assert_eq!(*got_dst, expected_dst, "{name}");
                    assert_eq!(*got_src, expected_src, "{name}");
                    assert_eq!(*got_width, width, "{name}");
                }
                other => panic!("expected APX {name} as Bswap, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_movbe_memory_forms_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `movbeq (%r16,%r17,2), %r18` => 62 ec f8 08 60 14 48.
        let load = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xEC, 0xF8, 0x08, 0x60, 0x14, 0x48],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(load.bytes_consumed, 7);
        assert_eq!(load.ops.len(), 2);
        let loaded = match &load.ops[0].kind {
            OpKind::Load {
                dst,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 2,
                        disp: 0,
                        ..
                    },
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(16));
                assert_eq!(*index, x86_gpr(17));
                assert!(matches!(dst, VReg::Virtual(_)));
                *dst
            }
            other => panic!("expected APX MOVBE memory load, got {other:?}"),
        };
        match &load.ops[1].kind {
            OpKind::Bswap {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(18));
                assert_eq!(*src, loaded);
            }
            other => panic!("expected APX MOVBE loaded Bswap, got {other:?}"),
        }

        // LLVM 20: `movbeq %r18, (%r16,%r17,2)` => 62 ec f8 08 61 14 48.
        let store = lifter
            .lift_insn(
                0x2000,
                &[0x62, 0xEC, 0xF8, 0x08, 0x61, 0x14, 0x48],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(store.bytes_consumed, 7);
        assert_eq!(store.ops.len(), 2);
        let swapped = match &store.ops[0].kind {
            OpKind::Bswap {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert!(matches!(dst, VReg::Virtual(_)));
                assert_eq!(*src, x86_gpr(18));
                *dst
            }
            other => panic!("expected APX MOVBE store Bswap, got {other:?}"),
        };
        match &store.ops[1].kind {
            OpKind::Store {
                src,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 2,
                        disp: 0,
                        ..
                    },
                width: MemWidth::B8,
            } => {
                assert_eq!(*src, swapped);
                assert_eq!(*base, x86_gpr(16));
                assert_eq!(*index, x86_gpr(17));
            }
            other => panic!("expected APX MOVBE memory store, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_movbe_rejects_invalid_forms_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            ([0x62, 0xD4, 0xFC, 0x0C, 0x61, 0xC0], "nf"),
            ([0x62, 0xD4, 0xFC, 0x18, 0x61, 0xC0], "ndd"),
            ([0x62, 0xD4, 0xFE, 0x08, 0x61, 0xC0], "f3"),
            ([0x62, 0xD4, 0xFF, 0x08, 0x61, 0xC0], "f2"),
        ] {
            let err = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::Unsupported { .. }),
                "{name}: {err:?}"
            );
        }
    }

    #[test]
    fn lift_movrs_0f38_legacy_widths_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, width, consumed) in [
            (
                &[0x44, 0x0F, 0x38, 0x8A, 0x03][..],
                "movrs8",
                MemWidth::B1,
                5,
            ),
            (
                &[0x44, 0x0F, 0x38, 0x8B, 0x03][..],
                "movrs32",
                MemWidth::B4,
                5,
            ),
            (
                &[0x4C, 0x0F, 0x38, 0x8B, 0x03][..],
                "movrs64",
                MemWidth::B8,
                5,
            ),
            (
                &[0x66, 0x44, 0x0F, 0x38, 0x8B, 0x03][..],
                "movrs16",
                MemWidth::B2,
                6,
            ),
        ] {
            let result = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, consumed, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            match &result.ops[0].kind {
                OpKind::Load {
                    dst,
                    addr: Address::Direct(base),
                    width: got_width,
                    sign: SignExtend::Zero,
                } => {
                    assert_eq!(*dst, x86_gpr(8), "{name}");
                    assert_eq!(*base, x86_gpr(3), "{name}");
                    assert_eq!(*got_width, width, "{name}");
                }
                other => panic!("expected legacy {name} Load, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_movrs_evex_memory_egpr_widths_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, width) in [
            (
                &[0x62, 0xEC, 0xF8, 0x08, 0x8B, 0x44, 0x91, 0x20][..],
                "movrs64",
                MemWidth::B8,
            ),
            (
                &[0x62, 0xEC, 0x78, 0x08, 0x8B, 0x44, 0x91, 0x20][..],
                "movrs32",
                MemWidth::B4,
            ),
            (
                &[0x62, 0xEC, 0x79, 0x08, 0x8B, 0x44, 0x91, 0x20][..],
                "movrs16",
                MemWidth::B2,
            ),
            (
                &[0x62, 0xEC, 0x78, 0x08, 0x8A, 0x44, 0x91, 0x20][..],
                "movrs8",
                MemWidth::B1,
            ),
            (
                &[0x62, 0xEC, 0xF8, 0x09, 0x8B, 0x44, 0x91, 0x20][..],
                "movrs64_aaa1",
                MemWidth::B8,
            ),
        ] {
            let result = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 8, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            match &result.ops[0].kind {
                OpKind::Load {
                    dst,
                    addr:
                        Address::BaseIndexScale {
                            base: Some(base),
                            index,
                            scale: 4,
                            disp: 0x20,
                            ..
                        },
                    width: got_width,
                    sign: SignExtend::Zero,
                } => {
                    assert_eq!(*dst, x86_gpr(16), "{name}");
                    assert_eq!(*base, x86_gpr(17), "{name}");
                    assert_eq!(*index, x86_gpr(18), "{name}");
                    assert_eq!(*got_width, width, "{name}");
                }
                other => panic!("expected APX EVEX {name} Load, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_movrs_rejects_invalid_forms_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            (&[0x44, 0x0F, 0x38, 0x8B, 0xC0][..], "legacy_reg_source"),
            (&[0x62, 0xEC, 0xF8, 0x08, 0x8B, 0xC0][..], "evex_reg_source"),
        ] {
            let err = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::InvalidEncoding { .. }),
                "{name}: {err:?}"
            );
        }

        for (bytes, name) in [
            (&[0x62, 0xEC, 0xF8, 0x0C, 0x8B, 0x44, 0x91, 0x20][..], "nf"),
            (&[0x62, 0xEC, 0xF8, 0x18, 0x8B, 0x44, 0x91, 0x20][..], "ndd"),
            (
                &[0x62, 0xEC, 0xF8, 0x08, 0x8A, 0x44, 0x91, 0x20][..],
                "byte_w",
            ),
            (&[0x62, 0xEC, 0x7A, 0x08, 0x8B, 0x44, 0x91, 0x20][..], "pp2"),
            (
                &[0x62, 0xEC, 0x38, 0x08, 0x8B, 0x44, 0x91, 0x20][..],
                "vvvv",
            ),
            (
                &[0x62, 0xEC, 0xF8, 0x00, 0x8B, 0x44, 0x91, 0x20][..],
                "vprime",
            ),
        ] {
            let err = lifter.lift_insn(0x1000, bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::Unsupported { .. }),
                "{name}: {err:?}"
            );
        }
    }

    #[test]
    fn lift_apx_setzucc_registers_zero_full_gpr_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, dst, cond) in [
            (
                [0x62, 0xF4, 0x7F, 0x18, 0x40, 0xC0],
                "setzuo_al",
                x86_gpr(0),
                Condition::Overflow,
            ),
            (
                [0x62, 0xF4, 0x7F, 0x18, 0x45, 0xC3],
                "setzune_bl",
                x86_gpr(3),
                Condition::Ne,
            ),
            (
                [0x62, 0xD4, 0x7F, 0x18, 0x40, 0xC0],
                "setzuo_r8b",
                x86_gpr(8),
                Condition::Overflow,
            ),
        ] {
            let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 6, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            match &result.ops[0].kind {
                OpKind::SetCC {
                    dst: got_dst,
                    cond: got_cond,
                    width: OpWidth::W64,
                } => {
                    assert_eq!(*got_dst, dst, "{name}");
                    assert_eq!(*got_cond, cond, "{name}");
                }
                other => panic!("expected APX {name} as full-width SetCC, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_setzucc_memory_stores_one_byte_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `setzuo (%rax)` => 62 f4 7f 18 40 00.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7F, 0x18, 0x40, 0x00], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 2);
        let tmp = match &result.ops[0].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Overflow,
                width: OpWidth::W8,
            } => {
                assert!(matches!(dst, VReg::Virtual(_)));
                *dst
            }
            other => panic!("expected APX SETZUcc temp byte SetCC, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Store {
                src,
                addr: Address::Direct(base),
                width: MemWidth::B1,
            } => {
                assert_eq!(*src, tmp);
                assert_eq!(*base, x86_gpr(0));
            }
            other => panic!("expected APX SETZUcc byte store, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_evex_setcc_without_zu_keeps_byte_width_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `{evex} setb %al` => 62 f4 7f 08 42 c0.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7F, 0x08, 0x42, 0xC0], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 1);
        match &result.ops[0].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W8,
            } => assert_eq!(*dst, x86_gpr(0)),
            other => panic!("expected EVEX SETcc byte register write, got {other:?}"),
        }

        // LLVM 20: `{evex} setb (%rax)` => 62 f4 7f 08 42 00.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7F, 0x08, 0x42, 0x00], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 2);
        let tmp = match &result.ops[0].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected EVEX SETcc byte temp, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Store {
                src,
                addr: Address::Direct(base),
                width: MemWidth::B1,
            } => {
                assert_eq!(*src, tmp);
                assert_eq!(*base, x86_gpr(0));
            }
            other => panic!("expected EVEX SETcc byte store, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_cmov_nd_uses_vvvv_destination_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `cmovbq %rbx, %rax, %r8` => 62 f4 bc 18 42 c3.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x18, 0x42, 0xC3], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 2);
        let cond = match &result.ops[0].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected CMOV_ND condition, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Select {
                dst,
                cond: got_cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*got_cond, cond);
                assert_eq!(*src_true, x86_gpr(3));
                assert_eq!(*src_false, x86_gpr(0));
            }
            other => panic!("expected CMOV_ND Select, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_cfcmov_two_operand_directions_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `cfcmovbq %rbx, %rax` => 62 f4 fc 0c 42 d8.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x0C, 0x42, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 3);
        let cond = match &result.ops[0].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected CFCMOV condition, got {other:?}"),
        };
        let zero = match &result.ops[1].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Imm(0),
                width: OpWidth::W64,
            } => *dst,
            other => panic!("expected CFCMOV false zero temp, got {other:?}"),
        };
        match &result.ops[2].kind {
            OpKind::Select {
                dst,
                cond: got_cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*got_cond, cond);
                assert_eq!(*src_true, x86_gpr(3));
                assert_eq!(*src_false, zero);
            }
            other => panic!("expected CFCMOV reg-destination Select, got {other:?}"),
        }

        // LLVM also decodes clear NF with PP=0 as the opposite reg-reg direction:
        // `cfcmovbq %rax, %rbx` from 62 f4 fc 08 42 d8.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x08, 0x42, 0xD8], &mut ctx)
            .unwrap();
        match &result.ops[2].kind {
            OpKind::Select {
                dst,
                src_true,
                width: OpWidth::W64,
                ..
            } => {
                assert_eq!(*dst, x86_gpr(3));
                assert_eq!(*src_true, x86_gpr(0));
            }
            other => panic!("expected opposite CFCMOV direction, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_cfcmov_memory_uses_predicated_access_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `cfcmovbq (%rbx), %rax` => 62 f4 fc 08 42 03.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x08, 0x42, 0x03], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 4);
        let cond = match &result.ops[1].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected CFCMOVrm condition, got {other:?}"),
        };
        let loaded = match &result.ops[2].kind {
            OpKind::PredLoad {
                dst,
                cond: got_cond,
                addr: Address::Direct(base),
                width: MemWidth::B8,
                signed: SignExtend::Zero,
            } => {
                assert_eq!(*got_cond, cond);
                assert_eq!(*base, x86_gpr(3));
                *dst
            }
            other => panic!("expected CFCMOVrm PredLoad, got {other:?}"),
        };
        match &result.ops[3].kind {
            OpKind::Select {
                dst,
                src_true,
                width: OpWidth::W64,
                ..
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src_true, loaded);
            }
            other => panic!("expected CFCMOVrm final Select, got {other:?}"),
        }

        // LLVM 20: `cfcmovbq %rbx, (%rax)` => 62 f4 fc 0c 42 18.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x0C, 0x42, 0x18], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 2);
        let cond = match &result.ops[0].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected CFCMOVmr condition, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::PredStore {
                src: SrcOperand::Reg(src),
                cond: got_cond,
                addr: Address::Direct(base),
                width: MemWidth::B8,
            } => {
                assert_eq!(*src, x86_gpr(3));
                assert_eq!(*got_cond, cond);
                assert_eq!(*base, x86_gpr(0));
            }
            other => panic!("expected CFCMOVmr PredStore, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_cfcmov_nd_memory_source_suppresses_false_fault_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `cfcmovbq (%rbx), %rax, %r8` => 62 f4 bc 1c 42 03.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x1C, 0x42, 0x03], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 3);
        let cond = match &result.ops[0].kind {
            OpKind::SetCC {
                dst,
                cond: Condition::Ult,
                width: OpWidth::W8,
            } => *dst,
            other => panic!("expected CFCMOV_ND condition, got {other:?}"),
        };
        let loaded = match &result.ops[1].kind {
            OpKind::PredLoad {
                dst,
                cond: got_cond,
                addr: Address::Direct(base),
                width: MemWidth::B8,
                signed: SignExtend::Zero,
            } => {
                assert_eq!(*got_cond, cond);
                assert_eq!(*base, x86_gpr(3));
                *dst
            }
            other => panic!("expected CFCMOV_ND PredLoad, got {other:?}"),
        };
        match &result.ops[2].kind {
            OpKind::Select {
                dst,
                cond: got_cond,
                src_true,
                src_false,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*got_cond, cond);
                assert_eq!(*src_true, loaded);
                assert_eq!(*src_false, x86_gpr(0));
            }
            other => panic!("expected CFCMOV_ND final Select, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_conditional_map4_rejects_invalid_pp2_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let err = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7E, 0x18, 0x42, 0xC0], &mut ctx)
            .unwrap_err();
        assert!(matches!(err, LiftError::InvalidEncoding { .. }), "{err:?}");
    }

    #[test]
    fn lift_apx_nf_count_registers_use_count_ops_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            ([0x62, 0x74, 0xFC, 0x0C, 0x88, 0xC0], "popcnt"),
            ([0x62, 0x74, 0xFC, 0x0C, 0xF5, 0xC0], "lzcnt"),
            ([0x62, 0x74, 0xFC, 0x0C, 0xF4, 0xC0], "tzcnt"),
        ] {
            let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 6, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");
            match (name, &result.ops[0].kind) {
                (
                    "popcnt",
                    OpKind::Popcnt {
                        dst,
                        src,
                        width: OpWidth::W64,
                    },
                )
                | (
                    "lzcnt",
                    OpKind::Clz {
                        dst,
                        src,
                        width: OpWidth::W64,
                    },
                )
                | (
                    "tzcnt",
                    OpKind::Ctz {
                        dst,
                        src,
                        width: OpWidth::W64,
                    },
                ) => {
                    assert_eq!(*dst, x86_gpr(8), "{name}");
                    assert_eq!(*src, x86_gpr(0), "{name}");
                }
                other => panic!("expected APX {name} count op, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_nf_count_width_and_memory_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `{nf} popcnt r8w, ax` => 62 74 7d 0c 88 c0.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0x74, 0x7D, 0x0C, 0x88, 0xC0], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 1);
        match &result.ops[0].kind {
            OpKind::Popcnt {
                dst,
                src,
                width: OpWidth::W16,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX word POPCNT, got {other:?}"),
        }

        // LLVM 20: `{nf} lzcnt r8, [rbx]` => 62 74 fc 0c f5 03.
        let result = lifter
            .lift_insn(0x2000, &[0x62, 0x74, 0xFC, 0x0C, 0xF5, 0x03], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 2);
        let tmp = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr: Address::Direct(base),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(3));
                assert!(matches!(dst, VReg::Virtual(_)));
                *dst
            }
            other => panic!("expected APX LZCNT memory load, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Clz {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, tmp);
            }
            other => panic!("expected APX LZCNT memory count op, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_nf_counts_reject_invalid_forms_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            ([0x62, 0x74, 0xFC, 0x08, 0x88, 0xC0], "popcnt without nf"),
            ([0x62, 0x74, 0xFC, 0x08, 0xF4, 0xC0], "tzcnt without nf"),
            ([0x62, 0x74, 0xFC, 0x1C, 0xF5, 0xC0], "lzcnt with ndd"),
        ] {
            let err = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::Unsupported { .. }),
                "{name}: {err:?}"
            );
        }
    }

    #[test]
    fn lift_apx_nf_bmi_0f38_register_forms_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `{nf} andn r8, rax, rbx` => 62 72 fc 0c f2 c3.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0x72, 0xFC, 0x0C, 0xF2, 0xC3], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 2);
        let inverted = match &result.ops[0].kind {
            OpKind::Not {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(0));
                *dst
            }
            other => panic!("expected APX ANDN Not temp, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src1, inverted);
                assert_eq!(*src2, x86_gpr(3));
            }
            other => panic!("expected APX ANDN final And, got {other:?}"),
        }

        // LLVM 20: `{nf} bextr r16, r17, r18` => 62 ea ec 04 f7 c1.
        let result = lifter
            .lift_insn(0x2000, &[0x62, 0xEA, 0xEC, 0x04, 0xF7, 0xC1], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 1);
        match &result.ops[0].kind {
            OpKind::Bextr {
                dst,
                src,
                control,
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(16));
                assert_eq!(*src, x86_gpr(17));
                assert_eq!(*control, x86_gpr(18));
            }
            other => panic!("expected APX BEXTR op, got {other:?}"),
        }

        // LLVM 20: `{nf} bzhi r16, r17, r18` => 62 ea ec 04 f5 c1.
        let result = lifter
            .lift_insn(0x3000, &[0x62, 0xEA, 0xEC, 0x04, 0xF5, 0xC1], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 1);
        match &result.ops[0].kind {
            OpKind::Bzhi {
                dst,
                src,
                index,
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(16));
                assert_eq!(*src, x86_gpr(17));
                assert_eq!(*index, x86_gpr(18));
            }
            other => panic!("expected APX BZHI op, got {other:?}"),
        }

        for (bytes, name, expected_first) in [
            ([0x62, 0xFA, 0xFC, 0x04, 0xF3, 0xD9], "blsi", "neg"),
            ([0x62, 0xFA, 0xFC, 0x04, 0xF3, 0xD1], "blsmsk", "sub"),
            ([0x62, 0xFA, 0xFC, 0x04, 0xF3, 0xC9], "blsr", "sub"),
        ] {
            // LLVM 20 encodes these as APX NF EVEX.0F38 F3 /3,/2,/1 forms.
            let result = lifter.lift_insn(0x4000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 6, "{name}");
            assert_eq!(result.ops.len(), 2, "{name}");
            let tmp = match (&result.ops[0].kind, expected_first) {
                (
                    OpKind::Neg {
                        dst,
                        src,
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    "neg",
                ) => {
                    assert_eq!(*src, x86_gpr(17), "{name}");
                    *dst
                }
                (
                    OpKind::Sub {
                        dst,
                        src1,
                        src2: SrcOperand::Imm(1),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    "sub",
                ) => {
                    assert_eq!(*src1, x86_gpr(17), "{name}");
                    *dst
                }
                (other, _) => panic!("expected APX {name} temp op, got {other:?}"),
            };
            match (&result.ops[1].kind, name) {
                (
                    OpKind::And {
                        dst,
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    "blsi",
                ) => {
                    assert_eq!(*dst, x86_gpr(16));
                    assert_eq!(*src1, tmp);
                    assert_eq!(*src2, x86_gpr(17));
                }
                (
                    OpKind::Xor {
                        dst,
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    "blsmsk",
                ) => {
                    assert_eq!(*dst, x86_gpr(16));
                    assert_eq!(*src1, x86_gpr(17));
                    assert_eq!(*src2, tmp);
                }
                (
                    OpKind::And {
                        dst,
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    "blsr",
                ) => {
                    assert_eq!(*dst, x86_gpr(16));
                    assert_eq!(*src1, x86_gpr(17));
                    assert_eq!(*src2, tmp);
                }
                (other, _) => panic!("expected APX {name} final op, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_nf_bmi_0f38_memory_width_and_alias_forms_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `{nf} andn r8d, eax, ebx` => 62 72 7c 0c f2 c3.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0x72, 0x7C, 0x0C, 0xF2, 0xC3], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 2);
        assert!(matches!(
            &result.ops[0].kind,
            OpKind::Not {
                src,
                width: OpWidth::W32,
                ..
            } if *src == x86_gpr(0)
        ));
        assert!(matches!(
            &result.ops[1].kind,
            OpKind::And {
                dst,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
                ..
            } if *dst == x86_gpr(8) && *src2 == x86_gpr(3)
        ));

        // LLVM 20: `{nf} bextr r8, qword ptr [rbx], rcx` => 62 72 f4 0c f7 03.
        let result = lifter
            .lift_insn(0x2000, &[0x62, 0x72, 0xF4, 0x0C, 0xF7, 0x03], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 2);
        let loaded = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr: Address::Direct(base),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(3));
                *dst
            }
            other => panic!("expected APX BEXTR memory load, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Bextr {
                dst,
                src,
                control,
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, loaded);
                assert_eq!(*control, x86_gpr(1));
            }
            other => panic!("expected APX BEXTR memory op, got {other:?}"),
        }

        // LLVM 20: `{nf} bzhi r8, qword ptr [rbx], rcx` => 62 72 f4 0c f5 03.
        let result = lifter
            .lift_insn(0x3000, &[0x62, 0x72, 0xF4, 0x0C, 0xF5, 0x03], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 2);
        let loaded = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr: Address::Direct(base),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(3));
                *dst
            }
            other => panic!("expected APX BZHI memory load, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Bzhi {
                dst,
                src,
                index,
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, loaded);
                assert_eq!(*index, x86_gpr(1));
            }
            other => panic!("expected APX BZHI memory op, got {other:?}"),
        }

        // LLVM 20: `{nf} blsr r8, qword ptr [rbx]` => 62 f2 bc 0c f3 0b.
        let result = lifter
            .lift_insn(0x4000, &[0x62, 0xF2, 0xBC, 0x0C, 0xF3, 0x0B], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 3);
        let loaded = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr: Address::Direct(base),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(3));
                *dst
            }
            other => panic!("expected APX BLSR memory load, got {other:?}"),
        };
        let minus_one = match &result.ops[1].kind {
            OpKind::Sub {
                dst,
                src1,
                src2: SrcOperand::Imm(1),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*src1, loaded);
                *dst
            }
            other => panic!("expected APX BLSR subtract temp, got {other:?}"),
        };
        match &result.ops[2].kind {
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src1, loaded);
                assert_eq!(*src2, minus_one);
            }
            other => panic!("expected APX BLSR final And, got {other:?}"),
        }

        // LLVM 20: `{nf} andn rax, rbx, rax` => 62 f2 e4 0c f2 c0.
        let result = lifter
            .lift_insn(0x5000, &[0x62, 0xF2, 0xE4, 0x0C, 0xF2, 0xC0], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 3);
        let saved_src2 = match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(0));
                *dst
            }
            other => panic!("expected APX ANDN alias-preserving Mov, got {other:?}"),
        };
        let inverted = match &result.ops[1].kind {
            OpKind::Not {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(3));
                *dst
            }
            other => panic!("expected APX ANDN alias Not, got {other:?}"),
        };
        match &result.ops[2].kind {
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src1, inverted);
                assert_eq!(*src2, saved_src2);
            }
            other => panic!("expected APX ANDN alias final And, got {other:?}"),
        }

        // LLVM 20: `{nf} blsr rax, rax` => 62 f2 fc 0c f3 c8.
        let result = lifter
            .lift_insn(0x6000, &[0x62, 0xF2, 0xFC, 0x0C, 0xF3, 0xC8], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 3);
        let saved_src = match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src, x86_gpr(0));
                *dst
            }
            other => panic!("expected APX BLSR alias-preserving Mov, got {other:?}"),
        };
        let minus_one = match &result.ops[1].kind {
            OpKind::Sub {
                dst,
                src1,
                src2: SrcOperand::Imm(1),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*src1, saved_src);
                *dst
            }
            other => panic!("expected APX BLSR alias Sub, got {other:?}"),
        };
        match &result.ops[2].kind {
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src1, saved_src);
                assert_eq!(*src2, minus_one);
            }
            other => panic!("expected APX BLSR alias final And, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_nf_bmi_0f38_rejects_invalid_forms_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            ([0x62, 0x72, 0xFC, 0x08, 0xF2, 0xC3], "without nf"),
            ([0x62, 0x72, 0xFC, 0x1C, 0xF2, 0xC3], "with nd"),
            ([0x62, 0x72, 0xFC, 0x0D, 0xF2, 0xC3], "nonzero aaa low bits"),
            ([0x62, 0x72, 0xFC, 0x0C, 0xF3, 0xC0], "F3 group /0"),
        ] {
            let err = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::Unsupported { .. }),
                "{name}: {err:?}"
            );
        }
    }

    #[test]
    fn lift_apx_ccmp_registers_use_conditional_flag_sequence_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `ccmpo {dfv=cf,zf} rax, rbx` has no trailing DFV byte.
        let ccmpo = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x9C, 0x00, 0x39, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(ccmpo.bytes_consumed, 6);
        assert_apx_conditional_flag_shape(&ccmpo, Condition::Overflow, 0x43);
        match &ccmpo.ops[4].kind {
            OpKind::Cmp {
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src1, x86_gpr(0));
                assert_eq!(*src2, x86_gpr(3));
            }
            other => panic!("expected APX CCMP register compare, got {other:?}"),
        }

        // LLVM 23: `ccmpno {dfv=cf,zf} rax, rbx`.
        let ccmpno = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x9C, 0x01, 0x39, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(ccmpno.bytes_consumed, 6);
        assert_apx_conditional_flag_shape(&ccmpno, Condition::NoOverflow, 0x43);
    }

    #[test]
    fn lift_apx_ctest_register_and_immediate_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `ctesto {dfv=sf,of} rax, rbx`.
        let ctest = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xE4, 0x40, 0x85, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(ctest.bytes_consumed, 6);
        assert_apx_conditional_flag_shape(&ctest, Condition::Overflow, 0x882);
        match &ctest.ops[4].kind {
            OpKind::Test {
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src1, x86_gpr(0));
                assert_eq!(*src2, x86_gpr(3));
            }
            other => panic!("expected APX CTEST register test, got {other:?}"),
        }

        // CTESTNZ rax, 0x0f, with DFV embedded in EVEX.vvvv.
        let ctest_imm = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0xE4, 0x45, 0xF7, 0xC0, 0x0F, 0x00, 0x00, 0x00],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(ctest_imm.bytes_consumed, 10);
        assert_apx_conditional_flag_shape(&ctest_imm, Condition::Ne, 0x882);
        match &ctest_imm.ops[4].kind {
            OpKind::Test {
                src1,
                src2: SrcOperand::Imm(0x0F),
                width: OpWidth::W64,
            } => assert_eq!(*src1, x86_gpr(0)),
            other => panic!("expected APX CTEST immediate test, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ccmp_ctest_memory_forms_use_predload_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `ccmpnz {dfv=of,sf} rax, [rbx]`.
        let ccmp_mem = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xE4, 0x05, 0x3B, 0x03], &mut ctx)
            .unwrap();
        assert_eq!(ccmp_mem.bytes_consumed, 6);
        let cond =
            assert_apx_conditional_flag_shape_with_true_ops(&ccmp_mem, Condition::Ne, 0x882, 2);
        let loaded = assert_apx_conditional_predload(&ccmp_mem, cond, 4, MemWidth::B8);
        match &ccmp_mem.ops[5].kind {
            OpKind::Cmp {
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src1, x86_gpr(0));
                assert_eq!(*src2, loaded);
            }
            other => panic!("expected APX CCMP memory compare, got {other:?}"),
        }

        // LLVM 20: `ccmpae {dfv=of,sf} qword ptr [rbx], 100`.
        let ccmp_imm_mem = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0xE4, 0x03, 0x83, 0x3B, 0x64],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(ccmp_imm_mem.bytes_consumed, 7);
        let cond = assert_apx_conditional_flag_shape_with_true_ops(
            &ccmp_imm_mem,
            Condition::Uge,
            0x882,
            2,
        );
        let loaded = assert_apx_conditional_predload(&ccmp_imm_mem, cond, 4, MemWidth::B8);
        match &ccmp_imm_mem.ops[5].kind {
            OpKind::Cmp {
                src1,
                src2: SrcOperand::Imm(100),
                width: OpWidth::W64,
            } => assert_eq!(*src1, loaded),
            other => panic!("expected APX CCMP memory immediate compare, got {other:?}"),
        }

        // LLVM 20: `ctestb {dfv=of,sf} [rbx], rcx`.
        let ctest_mem = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xE4, 0x02, 0x85, 0x0B], &mut ctx)
            .unwrap();
        assert_eq!(ctest_mem.bytes_consumed, 6);
        let cond =
            assert_apx_conditional_flag_shape_with_true_ops(&ctest_mem, Condition::Ult, 0x882, 2);
        let loaded = assert_apx_conditional_predload(&ctest_mem, cond, 4, MemWidth::B8);
        match &ctest_mem.ops[5].kind {
            OpKind::Test {
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
            } => {
                assert_eq!(*src1, loaded);
                assert_eq!(*src2, x86_gpr(1));
            }
            other => panic!("expected APX CTEST memory test, got {other:?}"),
        }

        // LLVM 20: `ctests {dfv=of,sf} qword ptr [rbx], 0xf0`.
        let ctest_imm_mem = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0xE4, 0x08, 0xF7, 0x03, 0xF0, 0x00, 0x00, 0x00],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(ctest_imm_mem.bytes_consumed, 10);
        let cond = assert_apx_conditional_flag_shape_with_true_ops(
            &ctest_imm_mem,
            Condition::Negative,
            0x882,
            2,
        );
        let loaded = assert_apx_conditional_predload(&ctest_imm_mem, cond, 4, MemWidth::B8);
        match &ctest_imm_mem.ops[5].kind {
            OpKind::Test {
                src1,
                src2: SrcOperand::Imm(0xF0),
                width: OpWidth::W64,
            } => assert_eq!(*src1, loaded),
            other => panic!("expected APX CTEST memory immediate test, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_group3_not_neg_use_vvvv_destination_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `not r8, rax` => 62 f4 bc 18 f7 d0.
        let not = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x18, 0xF7, 0xD0], &mut ctx)
            .unwrap();
        assert_eq!(not.bytes_consumed, 6);
        assert_eq!(not.ops.len(), 1);
        match &not.ops[0].kind {
            OpKind::Not {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX NDD NOT, got {other:?}"),
        }

        // LLVM 23: `neg r8, rax` => 62 f4 bc 18 f7 d8.
        let neg = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x18, 0xF7, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(neg.bytes_consumed, 6);
        assert_eq!(neg.ops.len(), 1);
        match &neg.ops[0].kind {
            OpKind::Neg {
                dst,
                src,
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX NDD NEG, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_nf_group3_neg_suppresses_flags_and_ndd_memory_source() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let neg_nf = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(neg_nf.bytes_consumed, 6);
        match &neg_nf.ops[0].kind {
            OpKind::Neg {
                dst,
                src,
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX NF NEG, got {other:?}"),
        }

        let not_mem = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x18, 0xF7, 0x10], &mut ctx)
            .unwrap();
        assert_eq!(not_mem.bytes_consumed, 6);
        assert_eq!(not_mem.ops.len(), 2);
        let loaded = match &not_mem.ops[0].kind {
            OpKind::Load {
                dst,
                width: MemWidth::B8,
                ..
            } => *dst,
            other => panic!("expected APX NDD NOT memory load, got {other:?}"),
        };
        match &not_mem.ops[1].kind {
            OpKind::Not {
                dst,
                src,
                width: OpWidth::W64,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, loaded);
            }
            other => panic!("expected APX NDD NOT memory source, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_nf_group3_implicit_mul_div_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, group) in [
            ([0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xE3], "mul", 4),
            ([0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xEB], "imul", 5),
            ([0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xF3], "div", 6),
            ([0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xFB], "idiv", 7),
        ] {
            let lifted = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(lifted.bytes_consumed, 6, "{name}");
            assert_eq!(lifted.ops.len(), 1, "{name}");

            match (&lifted.ops[0].kind, group) {
                (
                    OpKind::MulU {
                        dst_lo,
                        dst_hi: Some(dst_hi),
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    4,
                )
                | (
                    OpKind::MulS {
                        dst_lo,
                        dst_hi: Some(dst_hi),
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    5,
                ) => {
                    assert_eq!(*dst_lo, x86_gpr(0), "{name} low destination");
                    assert_eq!(*dst_hi, x86_gpr(2), "{name} high destination");
                    assert_eq!(*src1, x86_gpr(0), "{name} accumulator source");
                    assert_eq!(*src2, x86_gpr(3), "{name} r/m source");
                }
                (
                    OpKind::DivU {
                        quot,
                        rem: Some(rem),
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    6,
                )
                | (
                    OpKind::DivS {
                        quot,
                        rem: Some(rem),
                        src1,
                        src2: SrcOperand::Reg(src2),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                    7,
                ) => {
                    assert_eq!(*quot, x86_gpr(0), "{name} quotient");
                    assert_eq!(*rem, x86_gpr(2), "{name} remainder");
                    assert_eq!(*src1, x86_gpr(0), "{name} accumulator source");
                    assert_eq!(*src2, x86_gpr(3), "{name} r/m source");
                }
                (other, _) => panic!("expected APX NF implicit {name}, got {other:?}"),
            }
        }
    }

    #[test]
    fn lift_apx_group3_implicit_rejects_ndd_and_non_nf_forms() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name) in [
            ([0x62, 0xF4, 0xFC, 0x08, 0xF7, 0xE3], "non-nf mul"),
            ([0x62, 0xF4, 0xFC, 0x1C, 0xF7, 0xE3], "ndd nf mul"),
        ] {
            let err = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap_err();
            assert!(
                matches!(err, LiftError::Unsupported { .. }),
                "{name}: {err:?}"
            );
        }
    }

    #[test]
    fn lift_apx_nf_group3_implicit_memory_source_does_not_store() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let mul_mem = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0x20], &mut ctx)
            .unwrap();
        assert_eq!(mul_mem.bytes_consumed, 6);
        assert_eq!(mul_mem.ops.len(), 2);
        let loaded = match &mul_mem.ops[0].kind {
            OpKind::Load {
                dst,
                width: MemWidth::B8,
                ..
            } => *dst,
            other => panic!("expected APX NF MUL memory load, got {other:?}"),
        };
        match &mul_mem.ops[1].kind {
            OpKind::MulU {
                dst_lo,
                dst_hi: Some(dst_hi),
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst_lo, x86_gpr(0));
                assert_eq!(*dst_hi, x86_gpr(2));
                assert_eq!(*src1, x86_gpr(0));
                assert_eq!(*src2, loaded);
            }
            other => panic!("expected APX NF MUL memory source, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_nf_inc_dec_use_vvvv_destination_and_flags_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let inc = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x18, 0xFF, 0xC0], &mut ctx)
            .unwrap();
        assert_eq!(inc.bytes_consumed, 6);
        match &inc.ops[0].kind {
            OpKind::Inc {
                dst,
                src,
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX NDD INC, got {other:?}"),
        }

        let dec = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x18, 0xFF, 0xC8], &mut ctx)
            .unwrap();
        match &dec.ops[0].kind {
            OpKind::Dec {
                dst,
                src,
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX NDD DEC, got {other:?}"),
        }

        let inc_nf = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x0C, 0xFF, 0xC0], &mut ctx)
            .unwrap();
        match &inc_nf.ops[0].kind {
            OpKind::Inc {
                dst,
                src,
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src, x86_gpr(0));
            }
            other => panic!("expected APX NF INC, got {other:?}"),
        }

        let inc_mem = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x18, 0xFF, 0x00], &mut ctx)
            .unwrap();
        assert_eq!(inc_mem.ops.len(), 2);
        let loaded = match &inc_mem.ops[0].kind {
            OpKind::Load {
                dst,
                width: MemWidth::B8,
                ..
            } => *dst,
            other => panic!("expected APX NDD INC memory load, got {other:?}"),
        };
        match &inc_mem.ops[1].kind {
            OpKind::Inc {
                dst,
                src,
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(8));
                assert_eq!(*src, loaded);
            }
            other => panic!("expected APX NDD INC memory source, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_carry_rotates_use_rcl_rcr_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        for (bytes, name, amount) in [
            (
                [0x62, 0xF4, 0xBC, 0x18, 0xD1, 0xD0],
                "rcl",
                SrcOperand::Imm(1),
            ),
            (
                [0x62, 0xF4, 0xBC, 0x18, 0xD3, 0xD8],
                "rcr",
                SrcOperand::Reg(x86_gpr(1)),
            ),
        ] {
            let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
            assert_eq!(result.bytes_consumed, 6, "{name}");
            assert_eq!(result.ops.len(), 1, "{name}");

            match (name, &result.ops[0].kind) {
                (
                    "rcl",
                    OpKind::Rcl {
                        dst,
                        src,
                        amount: got_amount,
                        width: OpWidth::W64,
                        flags,
                    },
                )
                | (
                    "rcr",
                    OpKind::Rcr {
                        dst,
                        src,
                        amount: got_amount,
                        width: OpWidth::W64,
                        flags,
                    },
                ) => {
                    assert_eq!(*dst, x86_gpr(8), "{name}");
                    assert_eq!(*src, x86_gpr(0), "{name}");
                    assert_eq!(*got_amount, amount, "{name}");
                    assert_eq!(*flags, x86_rotate_flags(), "{name}");
                }
                other => panic!("expected APX NDD {name}, got {other:?}"),
            }
        }

        // LLVM 20 rejects `{nf} rcl r8, rax, 1`; carry rotates read CF and
        // cannot be encoded as no-flag-update APX forms.
        let err = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xBC, 0x1C, 0xD1, 0xD0], &mut ctx)
            .unwrap_err();
        assert!(matches!(err, LiftError::Unsupported { .. }), "{err:?}");
    }

    #[test]
    fn lift_apx_ndd_nf_add_suppresses_flag_updates() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `{nf} add rax, rbx` as EVEX MAP4 01 /r.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0xFC, 0x0C, 0x01, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 1);
        match &result.ops[0].kind {
            OpKind::Add {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src1, x86_gpr(0));
                assert_eq!(*src2, x86_gpr(3));
            }
            other => panic!("expected APX NF add rax, rbx, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_memory_source_decodes_x4_sib_index_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 23: `add eax, ebx, dword ptr [rax + 2*r16]`.
        let result = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xF4, 0x78, 0x18, 0x03, 0x1C, 0x40],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(result.bytes_consumed, 7);
        assert_eq!(result.ops.len(), 2);

        let tmp = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 2,
                        disp: 0,
                        ..
                    },
                width: MemWidth::B4,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(0));
                assert_eq!(*index, x86_gpr(16));
                *dst
            }
            other => panic!("expected APX memory source load with r16 index, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Add {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src1, x86_gpr(3));
                assert_eq!(*src2, tmp);
            }
            other => panic!("expected APX NDD memory-source add, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_memory_source_decodes_b4_sib_base_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // Same operation shape, but B4 extends the SIB base to r16.
        let result = lifter
            .lift_insn(
                0x1000,
                &[0x62, 0xFC, 0x7C, 0x18, 0x03, 0x1C, 0x40],
                &mut ctx,
            )
            .unwrap();
        match &result.ops[0].kind {
            OpKind::Load {
                addr:
                    Address::BaseIndexScale {
                        base: Some(base),
                        index,
                        scale: 2,
                        disp: 0,
                        ..
                    },
                ..
            } => {
                assert_eq!(*base, x86_gpr(16));
                assert_eq!(*index, x86_gpr(0));
            }
            other => panic!("expected APX memory source load with r16 base, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_memory_destination_becomes_register_result() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // Legacy 01 /r would write memory. APX ND redirects the result to vvvv.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7C, 0x18, 0x01, 0x18], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 2);
        assert!(
            !result
                .ops
                .iter()
                .any(|op| matches!(&op.kind, OpKind::Store { .. })),
            "NDD memory-destination ALU must not write the legacy memory destination"
        );
        let tmp = match &result.ops[0].kind {
            OpKind::Load { dst, .. } => *dst,
            other => panic!("expected memory destination load, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Add {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src1, tmp);
                assert_eq!(*src2, x86_gpr(3));
            }
            other => panic!("expected APX NDD register result, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_ndd_captures_source_when_destination_aliases_src2_for_lowering() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // Legacy 03 /r has src1=ModR/M.reg and src2=r/m. Here vvvv selects rax,
        // which aliases src2. The lifter captures rax before the result write.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7C, 0x18, 0x03, 0xD8], &mut ctx)
            .unwrap();
        assert_eq!(result.ops.len(), 2);
        let tmp = match &result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W32,
            } => {
                assert!(dst.is_virtual());
                assert_eq!(*src, x86_gpr(0));
                *dst
            }
            other => panic!("expected APX source alias capture, got {other:?}"),
        };
        match &result.ops[1].kind {
            OpKind::Add {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            } => {
                assert_eq!(*dst, x86_gpr(0));
                assert_eq!(*src1, x86_gpr(3));
                assert_eq!(*src2, tmp);
            }
            other => panic!("expected APX alias-safe NDD add, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_push2_uses_llvm_encoding_and_preserves_source_order() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `push2 %rax, %rsp` as EVEX MAP4 FF /6.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x5C, 0x18, 0xFF, 0xF0], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert!(matches!(result.control_flow, ControlFlow::Fallthrough));
        assert_eq!(result.ops.len(), 5);

        let tmp1 = match result.ops[0].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert!(dst.is_virtual());
                assert_eq!(src, x86_gpr(0));
                dst
            }
            ref other => panic!("expected source capture for PUSH2 operand 1, got {other:?}"),
        };
        let tmp2 = match result.ops[1].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert!(dst.is_virtual());
                assert_eq!(src, x86_gpr(4));
                dst
            }
            ref other => panic!("expected source capture for PUSH2 operand 2, got {other:?}"),
        };
        match result.ops[2].kind {
            OpKind::Sub {
                dst,
                src1,
                src2: SrcOperand::Imm(16),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(dst, x86_gpr(4));
                assert_eq!(src1, x86_gpr(4));
            }
            ref other => panic!("expected PUSH2 stack decrement, got {other:?}"),
        }
        match &result.ops[3].kind {
            OpKind::Store {
                src,
                addr: Address::Direct(base),
                width: MemWidth::B8,
            } => {
                assert_eq!(*src, tmp1);
                assert_eq!(*base, x86_gpr(4));
            }
            other => panic!("expected first PUSH2 store, got {other:?}"),
        }
        match &result.ops[4].kind {
            OpKind::Store {
                src,
                addr,
                width: MemWidth::B8,
            } => {
                assert_eq!(*src, tmp2);
                assert_eq!(*addr, Address::base_off(x86_gpr(4), 8));
            }
            other => panic!("expected second PUSH2 store, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_pop2_uses_llvm_encoding_and_writes_after_rsp_increment() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `pop2 %rsp, %rax` as EVEX MAP4 8F.
        let result = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7C, 0x18, 0x8F, 0xC4], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 6);
        assert_eq!(result.ops.len(), 5);

        let tmp1 = match &result.ops[0].kind {
            OpKind::Load {
                dst,
                addr: Address::Direct(base),
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*base, x86_gpr(4));
                *dst
            }
            other => panic!("expected first POP2 load, got {other:?}"),
        };
        let tmp2 = match &result.ops[1].kind {
            OpKind::Load {
                dst,
                addr,
                width: MemWidth::B8,
                sign: SignExtend::Zero,
            } => {
                assert_eq!(*addr, Address::base_off(x86_gpr(4), 8));
                *dst
            }
            other => panic!("expected second POP2 load, got {other:?}"),
        };
        match result.ops[2].kind {
            OpKind::Add {
                dst,
                src1,
                src2: SrcOperand::Imm(16),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            } => {
                assert_eq!(dst, x86_gpr(4));
                assert_eq!(src1, x86_gpr(4));
            }
            ref other => panic!("expected POP2 stack increment, got {other:?}"),
        }
        match result.ops[3].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(dst, x86_gpr(4));
                assert_eq!(src, tmp1);
            }
            ref other => panic!("expected POP2 first destination write, got {other:?}"),
        }
        match result.ops[4].kind {
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(src),
                width: OpWidth::W64,
            } => {
                assert_eq!(dst, x86_gpr(0));
                assert_eq!(src, tmp2);
            }
            ref other => panic!("expected POP2 second destination write, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_push2_pop2_decode_egprs_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // LLVM 20: `push2 %r16, %rcx`.
        let push = lifter
            .lift_insn(0x1000, &[0x62, 0xEC, 0x74, 0x18, 0xFF, 0xF0], &mut ctx)
            .unwrap();
        match push.ops[0].kind {
            OpKind::Mov {
                src: SrcOperand::Reg(src),
                ..
            } => assert_eq!(src, x86_gpr(16)),
            ref other => panic!("expected PUSH2 first EGPR operand, got {other:?}"),
        }
        match push.ops[1].kind {
            OpKind::Mov {
                src: SrcOperand::Reg(src),
                ..
            } => assert_eq!(src, x86_gpr(1)),
            ref other => panic!("expected PUSH2 second operand, got {other:?}"),
        }

        // LLVM 20: `pop2 %r20, %rbp`.
        let pop = lifter
            .lift_insn(0x2000, &[0x62, 0xEC, 0x54, 0x18, 0x8F, 0xC4], &mut ctx)
            .unwrap();
        match pop.ops[3].kind {
            OpKind::Mov { dst, .. } => assert_eq!(dst, x86_gpr(20)),
            ref other => panic!("expected POP2 first EGPR destination, got {other:?}"),
        }
        match pop.ops[4].kind {
            OpKind::Mov { dst, .. } => assert_eq!(dst, x86_gpr(5)),
            ref other => panic!("expected POP2 second destination, got {other:?}"),
        }
    }

    #[test]
    fn lift_apx_push2_pop2_reject_invalid_forms_like_llvm() {
        let mut lifter = X86_64Lifter::strict();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let push_err = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x6C, 0x18, 0xFF, 0x30], &mut ctx)
            .unwrap_err();
        assert!(matches!(push_err, LiftError::Unsupported { .. }));

        let pop_err = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7C, 0x18, 0x8F, 0x00], &mut ctx)
            .unwrap_err();
        assert!(matches!(pop_err, LiftError::Unsupported { .. }));

        let pop_group_err = lifter
            .lift_insn(0x1000, &[0x62, 0xF4, 0x7C, 0x18, 0x8F, 0xC8], &mut ctx)
            .unwrap_err();
        assert!(matches!(pop_group_err, LiftError::Unsupported { .. }));
    }

    #[test]
    fn test_modrm_decode() {
        // MOD=3 (register)
        let prefix = X86Prefix::default();
        let modrm = decode_modrm(&[0xC0], &prefix, 0).unwrap();
        assert!(!modrm.is_memory);
        assert_eq!(modrm.reg, 0);
        assert_eq!(modrm.rm, 0);

        // MOD=0, RM=5 (RIP-relative)
        let modrm = decode_modrm(&[0x05, 0x10, 0x00, 0x00, 0x00], &prefix, 0).unwrap();
        assert!(modrm.is_memory);
        assert!(modrm.addr.as_ref().unwrap().rip_relative);
        assert_eq!(modrm.bytes_consumed, 5);
    }

    #[test]
    fn test_lift_nop() {
        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // NOP
        let result = lifter.lift_insn(0x1000, &[0x90], &mut ctx).unwrap();
        assert_eq!(result.bytes_consumed, 1);
        assert!(matches!(result.control_flow, ControlFlow::Fallthrough));
    }

    #[test]
    fn test_lift_mov_r_imm() {
        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // MOV EAX, 0x12345678
        let result = lifter
            .lift_insn(0x1000, &[0xB8, 0x78, 0x56, 0x34, 0x12], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 5);
        assert_eq!(result.ops.len(), 1);

        // MOV RAX, 0x123456789ABCDEF0 (REX.W prefix)
        let result = lifter
            .lift_insn(
                0x1000,
                &[0x48, 0xB8, 0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12],
                &mut ctx,
            )
            .unwrap();
        assert_eq!(result.bytes_consumed, 10);
    }

    #[test]
    fn test_lift_jmp() {
        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // JMP rel8
        let result = lifter.lift_insn(0x1000, &[0xEB, 0x10], &mut ctx).unwrap();
        assert_eq!(result.bytes_consumed, 2);
        assert!(matches!(
            result.control_flow,
            ControlFlow::Branch { target: 0x1012 }
        ));

        // JMP rel32
        let result = lifter
            .lift_insn(0x1000, &[0xE9, 0x00, 0x10, 0x00, 0x00], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 5);
        assert!(matches!(
            result.control_flow,
            ControlFlow::Branch { target: 0x2005 }
        ));
    }

    #[test]
    fn test_lift_jcc() {
        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // JE rel8
        let result = lifter.lift_insn(0x1000, &[0x74, 0x10], &mut ctx).unwrap();
        assert_eq!(result.bytes_consumed, 2);
        match result.control_flow {
            ControlFlow::CondBranch {
                cond,
                target,
                fallthrough,
            } => {
                assert_eq!(cond, Condition::Eq);
                assert_eq!(target, 0x1012);
                assert_eq!(fallthrough, 0x1002);
            }
            _ => panic!("Expected CondBranch"),
        }
    }

    #[test]
    fn test_lift_push_pop() {
        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // PUSH RAX
        let result = lifter.lift_insn(0x1000, &[0x50], &mut ctx).unwrap();
        assert_eq!(result.bytes_consumed, 1);
        assert_eq!(result.ops.len(), 2); // SUB RSP + STORE

        // POP RAX
        let result = lifter.lift_insn(0x1000, &[0x58], &mut ctx).unwrap();
        assert_eq!(result.bytes_consumed, 1);
        assert_eq!(result.ops.len(), 2); // LOAD + ADD RSP
    }

    #[test]
    fn test_lift_call_ret() {
        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // CALL rel32
        let result = lifter
            .lift_insn(0x1000, &[0xE8, 0x00, 0x10, 0x00, 0x00], &mut ctx)
            .unwrap();
        assert_eq!(result.bytes_consumed, 5);
        assert!(matches!(
            result.control_flow,
            ControlFlow::Call {
                target: CallTarget::GuestAddr(0x2005)
            }
        ));

        // RET
        let result = lifter.lift_insn(0x1000, &[0xC3], &mut ctx).unwrap();
        assert_eq!(result.bytes_consumed, 1);
        assert!(matches!(result.control_flow, ControlFlow::Return));
    }

    #[test]
    fn test_lift_block() {
        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        // Simple block: MOV EAX, 1; RET
        let mem = TestMemory::new(0x1000, vec![0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3]);
        let block = lifter.lift_block(0x1000, &mem, &mut ctx).unwrap();

        assert_eq!(block.guest_pc, 0x1000);
        assert!(matches!(block.terminator, Terminator::Return { .. }));
    }
}
