//! RISC-V instruction decoder.
//!
//! [`decode`] resolves a 32-bit instruction word into a fully-populated
//! [`Insn`]; [`decode_compressed`] handles the 16-bit RVC encodings.
//! [`decode_at`] fetches from memory, selecting the width from the low two bits
//! of the first parcel, and is the entry point used by the execution loop.
//!
//! Decoding is gated by the active [`Isa`]: encodings belonging to a disabled
//! extension resolve to [`Op::Illegal`] so the CPU raises an illegal-instruction
//! exception, exactly as hardware would when the extension is absent.

use super::memory::{MemError, Memory};
use super::{Isa, Xlen};

/// A decoded RISC-V operation. One variant per architectural operation across
/// the I/M/A/F/D/C and Zb* extensions; operand fields live in [`Insn`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Op {
    // ---- RV32I / RV64I base ----
    Lui,
    Auipc,
    Jal,
    Jalr,
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Lwu,
    Ld,
    Sb,
    Sh,
    Sw,
    Sd,
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    Addiw,
    Slliw,
    Srliw,
    Sraiw,
    Addw,
    Subw,
    Sllw,
    Srlw,
    Sraw,
    Fence,
    FenceI,
    Ecall,
    Ebreak,
    // ---- privileged / system (subset) ----
    Mret,
    Sret,
    Wfi,
    // ---- Zicsr ----
    Csrrw,
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
    // ---- M ----
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
    Mulw,
    Divw,
    Divuw,
    Remw,
    Remuw,
    // ---- A (word) ----
    LrW,
    ScW,
    AmoswapW,
    AmoaddW,
    AmoxorW,
    AmoandW,
    AmoorW,
    AmominW,
    AmomaxW,
    AmominuW,
    AmomaxuW,
    // ---- A (double) ----
    LrD,
    ScD,
    AmoswapD,
    AmoaddD,
    AmoxorD,
    AmoandD,
    AmoorD,
    AmominD,
    AmomaxD,
    AmominuD,
    AmomaxuD,
    // ---- F (single precision) ----
    Flw,
    Fsw,
    FmaddS,
    FmsubS,
    FnmsubS,
    FnmaddS,
    FaddS,
    FsubS,
    FmulS,
    FdivS,
    FsqrtS,
    FsgnjS,
    FsgnjnS,
    FsgnjxS,
    FminS,
    FmaxS,
    FcvtWS,
    FcvtWuS,
    FcvtLS,
    FcvtLuS,
    FmvXW,
    FeqS,
    FltS,
    FleS,
    FclassS,
    FcvtSW,
    FcvtSWu,
    FcvtSL,
    FcvtSLu,
    FmvWX,
    // ---- D (double precision) ----
    Fld,
    Fsd,
    FmaddD,
    FmsubD,
    FnmsubD,
    FnmaddD,
    FaddD,
    FsubD,
    FmulD,
    FdivD,
    FsqrtD,
    FsgnjD,
    FsgnjnD,
    FsgnjxD,
    FminD,
    FmaxD,
    FcvtSD,
    FcvtDS,
    FeqD,
    FltD,
    FleD,
    FclassD,
    FcvtWD,
    FcvtWuD,
    FcvtLD,
    FcvtLuD,
    FcvtDW,
    FcvtDWu,
    FcvtDL,
    FcvtDLu,
    FmvXD,
    FmvDX,
    // ---- Zba ----
    Sh1add,
    Sh2add,
    Sh3add,
    AddUw,
    Sh1addUw,
    Sh2addUw,
    Sh3addUw,
    SlliUw,
    // ---- Zbb ----
    Andn,
    Orn,
    Xnor,
    Clz,
    Ctz,
    Cpop,
    Max,
    Maxu,
    Min,
    Minu,
    SextB,
    SextH,
    ZextH,
    Rol,
    Ror,
    Rori,
    Orcb,
    Rev8,
    Clzw,
    Ctzw,
    Cpopw,
    Rolw,
    Rorw,
    Roriw,
    // ---- Zbc ----
    Clmul,
    Clmulh,
    Clmulr,
    // ---- Zbs ----
    Bclr,
    Bclri,
    Bext,
    Bexti,
    Binv,
    Binvi,
    Bset,
    Bseti,
    // ---- Zicond ----
    CzeroEqz,
    CzeroNez,
    // ---- Zfa ----
    FliS,
    FliD,
    FminmS,
    FmaxmS,
    FminmD,
    FmaxmD,
    FroundS,
    FroundnxS,
    FroundD,
    FroundnxD,
    FleqS,
    FltqS,
    FleqD,
    FltqD,
    FcvtmodWD,
    // ---- Zbkb ----
    Pack,
    Packh,
    Packw,
    Brev8,
    // ---- Zfh (half precision) ----
    Flh,
    Fsh,
    FaddH,
    FsubH,
    FmulH,
    FdivH,
    FsqrtH,
    FmaddH,
    FmsubH,
    FnmsubH,
    FnmaddH,
    FsgnjH,
    FsgnjnH,
    FsgnjxH,
    FminH,
    FmaxH,
    FeqH,
    FltH,
    FleH,
    FclassH,
    FcvtSH,
    FcvtHS,
    FcvtDH,
    FcvtHD,
    FcvtWH,
    FcvtWuH,
    FcvtLH,
    FcvtLuH,
    FcvtHW,
    FcvtHWu,
    FcvtHL,
    FcvtHLu,
    FmvXH,
    FmvHX,
    // ---- Zfh + Zfa (half) ----
    FliH,
    FminmH,
    FmaxmH,
    FroundH,
    FroundnxH,
    FleqH,
    FltqH,
    // ---- Zbkx ----
    Xperm4,
    Xperm8,
    // ---- Zknh (SHA) ----
    Sha256Sig0,
    Sha256Sig1,
    Sha256Sum0,
    Sha256Sum1,
    Sha512Sig0,
    Sha512Sig1,
    Sha512Sum0,
    Sha512Sum1,
    // ---- Zksh (SM3) ----
    Sm3p0,
    Sm3p1,
    // ---- Zksed (SM4) ----
    Sm4ed,
    Sm4ks,
    // ---- Zkne / Zknd (AES) ----
    Aes64es,
    Aes64esm,
    Aes64ds,
    Aes64dsm,
    Aes64ks1i,
    Aes64ks2,
    Aes64im,
    // ---- V (vector configuration) ----
    Vsetvli,
    Vsetivli,
    Vsetvl,
    // ---- V (vector load/store, unit-stride; width in funct3) ----
    Vle,
    Vse,
    Vlse,
    Vsse,
    Vlxei,
    Vsxei,
    Vlm,
    Vsm,
    Vlre,
    Vsre,
    Vlseg,
    Vsseg,
    Vleff,
    // ---- V (vector integer arithmetic; form vv/vx/vi in funct3) ----
    Vadd,
    Vsub,
    Vrsub,
    Vand,
    Vor,
    Vxor,
    Vminu,
    Vmin,
    Vmaxu,
    Vmax,
    Vsll,
    Vsrl,
    Vsra,
    Vmerge, // also vmv.v.* when vm=1
    Vmseq,
    Vmsne,
    Vmsltu,
    Vmslt,
    Vmsleu,
    Vmsle,
    Vmsgtu,
    Vmsgt,
    // ---- V (OPMVV/OPMVX integer multiply/divide) ----
    Vmul,
    Vmulh,
    Vmulhu,
    Vmulhsu,
    Vdivu,
    Vdiv,
    Vremu,
    Vrem,
    // ---- V (OPFVV/OPFVF floating point) ----
    Vfadd,
    Vfsub,
    Vfrsub,
    Vfmul,
    Vfdiv,
    Vfrdiv,
    Vfsqrt,
    Vfmin,
    Vfmax,
    Vfsgnj,
    Vfsgnjn,
    Vfsgnjx,
    Vmfeq,
    Vmfne,
    Vmflt,
    Vmfle,
    Vmfgt,
    Vmfge,
    Vfmacc,
    Vfnmacc,
    Vfmsac,
    Vfnmsac,
    Vfmadd,
    Vfnmadd,
    Vfmsub,
    Vfnmsub,
    // ---- V (OPMVV integer reductions) ----
    Vredsum,
    Vredand,
    Vredor,
    Vredxor,
    Vredminu,
    Vredmin,
    Vredmaxu,
    Vredmax,
    // ---- V (OPFVV floating-point reductions) ----
    Vfredusum,
    Vfredosum,
    Vfredmin,
    Vfredmax,
    // ---- V (scalar element moves: lane 0 <-> x/f register) ----
    VmvXS,
    VmvSX,
    VfmvFS,
    VfmvSF,
    // ---- V (mask-register logical) ----
    Vmand,
    Vmnand,
    Vmandn,
    Vmxor,
    Vmor,
    Vmnor,
    Vmorn,
    Vmxnor,
    // ---- V (integer zero/sign extension, VXUNARY0) ----
    VzextVf2,
    VsextVf2,
    VzextVf4,
    VsextVf4,
    VzextVf8,
    VsextVf8,
    // ---- V (mask population / set / index) ----
    Vcpop,
    Vfirst,
    Vmsbf,
    Vmsof,
    Vmsif,
    Viota,
    Vid,
    // ---- V (slides) ----
    Vslideup,
    Vslidedown,
    Vslide1up,
    Vslide1down,
    Vfslide1up,
    Vfslide1down,
    Vrgather,
    Vrgatherei16,
    Vcompress,
    // ---- V (add/subtract with carry/borrow) ----
    Vadc,
    Vmadc,
    Vsbc,
    Vmsbc,
    // ---- V (saturating fixed-point add/subtract) ----
    Vsaddu,
    Vsadd,
    Vssubu,
    Vssub,
    // ---- V (averaging add/subtract) ----
    Vaaddu,
    Vaadd,
    Vasubu,
    Vasub,
    // ---- V (scaling shift / fractional multiply) ----
    Vssrl,
    Vssra,
    Vsmul,
    // ---- V (widening integer add/subtract) ----
    Vwaddu,
    Vwadd,
    Vwsubu,
    Vwsub,
    VwadduW,
    VwaddW,
    VwsubuW,
    VwsubW,
    // ---- V (widening integer multiply / multiply-accumulate) ----
    Vwmulu,
    Vwmulsu,
    Vwmul,
    Vwmaccu,
    Vwmacc,
    Vwmaccsu,
    Vwmaccus,
    // ---- V (narrowing shift / clip) ----
    Vnsrl,
    Vnsra,
    Vnclipu,
    Vnclip,
    // ---- V (single-width FP/integer conversions, VFUNARY0) ----
    VfcvtXuF,
    VfcvtXF,
    VfcvtFXu,
    VfcvtFX,
    VfcvtRtzXuF,
    VfcvtRtzXF,
    // ---- V (widening FP/integer conversions) ----
    VfwcvtXuF,
    VfwcvtXF,
    VfwcvtFXu,
    VfwcvtFX,
    VfwcvtFF,
    VfwcvtRtzXuF,
    VfwcvtRtzXF,
    // ---- V (narrowing FP/integer conversions) ----
    VfncvtXuF,
    VfncvtXF,
    VfncvtFXu,
    VfncvtFX,
    VfncvtFF,
    VfncvtRodFF,
    VfncvtRtzXuF,
    VfncvtRtzXF,
    // ---- V (widening FP arithmetic) ----
    Vfwadd,
    Vfwsub,
    Vfwmul,
    VfwaddW,
    VfwsubW,
    Vfwmacc,
    Vfwnmacc,
    Vfwmsac,
    Vfwnmsac,
    // ---- V (widening reductions) ----
    Vwredsumu,
    Vwredsum,
    Vfwredusum,
    Vfwredosum,
    // ---- V (classify / whole-register move) ----
    Vfclass,
    Vmvr,
    // ---- V (reciprocal / rsqrt estimates) ----
    Vfrsqrt7,
    Vfrec7,
    // ---- sentinel ----
    Illegal,
}

impl Op {
    /// `true` if this is a floating-point (F/D) operation handled by the FP
    /// execution path.
    pub fn is_fp(self) -> bool {
        use Op::*;
        matches!(
            self,
            Flw | Fsw
                | FmaddS
                | FmsubS
                | FnmsubS
                | FnmaddS
                | FaddS
                | FsubS
                | FmulS
                | FdivS
                | FsqrtS
                | FsgnjS
                | FsgnjnS
                | FsgnjxS
                | FminS
                | FmaxS
                | FcvtWS
                | FcvtWuS
                | FcvtLS
                | FcvtLuS
                | FmvXW
                | FeqS
                | FltS
                | FleS
                | FclassS
                | FcvtSW
                | FcvtSWu
                | FcvtSL
                | FcvtSLu
                | FmvWX
                | Fld
                | Fsd
                | FmaddD
                | FmsubD
                | FnmsubD
                | FnmaddD
                | FaddD
                | FsubD
                | FmulD
                | FdivD
                | FsqrtD
                | FsgnjD
                | FsgnjnD
                | FsgnjxD
                | FminD
                | FmaxD
                | FcvtSD
                | FcvtDS
                | FeqD
                | FltD
                | FleD
                | FclassD
                | FcvtWD
                | FcvtWuD
                | FcvtLD
                | FcvtLuD
                | FcvtDW
                | FcvtDWu
                | FcvtDL
                | FcvtDLu
                | FmvXD
                | FmvDX
                | FliS
                | FliD
                | FminmS
                | FmaxmS
                | FminmD
                | FmaxmD
                | FroundS
                | FroundnxS
                | FroundD
                | FroundnxD
                | FleqS
                | FltqS
                | FleqD
                | FltqD
                | FcvtmodWD
                | Flh
                | Fsh
                | FaddH
                | FsubH
                | FmulH
                | FdivH
                | FsqrtH
                | FmaddH
                | FmsubH
                | FnmsubH
                | FnmaddH
                | FsgnjH
                | FsgnjnH
                | FsgnjxH
                | FminH
                | FmaxH
                | FeqH
                | FltH
                | FleH
                | FclassH
                | FcvtSH
                | FcvtHS
                | FcvtDH
                | FcvtHD
                | FcvtWH
                | FcvtWuH
                | FcvtLH
                | FcvtLuH
                | FcvtHW
                | FcvtHWu
                | FcvtHL
                | FcvtHLu
                | FmvXH
                | FmvHX
                | FliH
                | FminmH
                | FmaxmH
                | FroundH
                | FroundnxH
                | FleqH
                | FltqH
        )
    }
}

/// A fully decoded instruction with all operand fields resolved.
#[derive(Clone, Copy, Debug)]
pub struct Insn {
    /// The operation.
    pub op: Op,
    /// Destination register (or `rd` for FP).
    pub rd: u8,
    /// First source register.
    pub rs1: u8,
    /// Second source register.
    pub rs2: u8,
    /// Third source register (FMA only).
    pub rs3: u8,
    /// Sign-extended immediate (semantics depend on `op`).
    pub imm: i64,
    /// `funct3` field, reused as the FP rounding-mode field.
    pub funct3: u8,
    /// CSR address (Zicsr) — also carries the 5-bit zimm in `rs1` for the
    /// immediate CSR forms.
    pub csr: u16,
    /// Atomic ordering bit `aq`.
    pub aq: bool,
    /// Atomic ordering bit `rl`.
    pub rl: bool,
    /// Encoded length in bytes (2 for compressed, 4 otherwise).
    pub len: u8,
    /// The raw little-endian instruction bits.
    pub raw: u32,
}

impl Insn {
    /// An illegal instruction of the given length carrying its raw bits.
    fn illegal(raw: u32, len: u8) -> Self {
        Insn {
            op: Op::Illegal,
            rd: 0,
            rs1: 0,
            rs2: 0,
            rs3: 0,
            imm: 0,
            funct3: 0,
            csr: 0,
            aq: false,
            rl: false,
            len,
            raw,
        }
    }

    /// An illegal 16-bit (compressed) parcel.
    pub(crate) fn illegal_compressed(half: u16) -> Self {
        Insn::illegal(half as u32, 2)
    }

    /// `true` for [`Op::Illegal`].
    #[inline]
    pub fn is_illegal(&self) -> bool {
        matches!(self.op, Op::Illegal)
    }

    /// The rounding-mode field of a floating-point instruction.
    #[inline]
    pub fn rm(&self) -> u8 {
        self.funct3
    }
}

/// Errors that can occur while fetching an instruction for decode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The instruction parcel could not be read from memory.
    Fetch(MemError),
}

// ---------------------------------------------------------------------------
// Field extraction helpers (operate on the 32-bit instruction word).
// ---------------------------------------------------------------------------

#[inline]
fn opcode(w: u32) -> u32 {
    w & 0x7f
}
#[inline]
fn rd(w: u32) -> u8 {
    ((w >> 7) & 0x1f) as u8
}
#[inline]
fn funct3(w: u32) -> u8 {
    ((w >> 12) & 0x7) as u8
}
#[inline]
fn rs1(w: u32) -> u8 {
    ((w >> 15) & 0x1f) as u8
}
#[inline]
fn rs2(w: u32) -> u8 {
    ((w >> 20) & 0x1f) as u8
}
#[inline]
fn rs3(w: u32) -> u8 {
    ((w >> 27) & 0x1f) as u8
}
#[inline]
fn funct7(w: u32) -> u32 {
    (w >> 25) & 0x7f
}
#[inline]
fn funct2(w: u32) -> u32 {
    (w >> 25) & 0x3
}

// Sign-extended immediates.
#[inline]
fn imm_i(w: u32) -> i64 {
    (w as i32 as i64) >> 20
}
#[inline]
fn imm_s(w: u32) -> i64 {
    let hi = (w >> 25) & 0x7f;
    let lo = (w >> 7) & 0x1f;
    let v = (hi << 5) | lo;
    // sign extend 12 bits
    ((v as i32) << 20 >> 20) as i64
}
#[inline]
fn imm_b(w: u32) -> i64 {
    let b12 = (w >> 31) & 1;
    let b11 = (w >> 7) & 1;
    let b10_5 = (w >> 25) & 0x3f;
    let b4_1 = (w >> 8) & 0xf;
    let v = (b12 << 12) | (b11 << 11) | (b10_5 << 5) | (b4_1 << 1);
    ((v as i32) << 19 >> 19) as i64
}
#[inline]
fn imm_u(w: u32) -> i64 {
    (w & 0xffff_f000) as i32 as i64
}
#[inline]
fn imm_j(w: u32) -> i64 {
    let b20 = (w >> 31) & 1;
    let b19_12 = (w >> 12) & 0xff;
    let b11 = (w >> 20) & 1;
    let b10_1 = (w >> 21) & 0x3ff;
    let v = (b20 << 20) | (b19_12 << 12) | (b11 << 11) | (b10_1 << 1);
    ((v as i32) << 11 >> 11) as i64
}

/// Build a base [`Insn`] with the common register/length fields populated.
fn base(op: Op, w: u32) -> Insn {
    Insn {
        op,
        rd: rd(w),
        rs1: rs1(w),
        rs2: rs2(w),
        rs3: rs3(w),
        imm: 0,
        funct3: funct3(w),
        csr: ((w >> 20) & 0xfff) as u16,
        aq: (w >> 26) & 1 != 0,
        rl: (w >> 25) & 1 != 0,
        len: 4,
        raw: w,
    }
}

/// Decode a 32-bit instruction word under the given XLEN and ISA.
pub fn decode(w: u32, xlen: Xlen, isa: &Isa) -> Insn {
    let rv64 = xlen == Xlen::Rv64;
    match opcode(w) {
        0x37 => with_imm(Op::Lui, w, imm_u(w)),
        0x17 => with_imm(Op::Auipc, w, imm_u(w)),
        0x6f => with_imm(Op::Jal, w, imm_j(w)),
        0x67 if funct3(w) == 0 => with_imm(Op::Jalr, w, imm_i(w)),
        0x63 => decode_branch(w),
        0x03 => decode_load(w, rv64),
        0x23 => decode_store(w, rv64),
        0x13 => decode_op_imm(w, rv64, isa),
        0x1b if rv64 => decode_op_imm32(w, isa),
        0x33 => decode_op(w, isa),
        0x3b if rv64 => decode_op32(w, isa),
        0x0f => decode_fence(w, isa),
        0x73 => decode_system(w, isa),
        0x2f if isa.a => decode_amo(w, rv64),
        0x07 if isa.f => decode_load_fp(w, isa),
        0x27 if isa.f => decode_store_fp(w, isa),
        0x53 if isa.f => decode_op_fp(w, rv64, isa),
        0x43 if isa.f => decode_fma(Op::FmaddS, Op::FmaddD, Op::FmaddH, w, isa),
        0x47 if isa.f => decode_fma(Op::FmsubS, Op::FmsubD, Op::FmsubH, w, isa),
        0x4b if isa.f => decode_fma(Op::FnmsubS, Op::FnmsubD, Op::FnmsubH, w, isa),
        0x4f if isa.f => decode_fma(Op::FnmaddS, Op::FnmaddD, Op::FnmaddH, w, isa),
        0x57 if isa.v => decode_vector(w),
        _ => Insn::illegal(w, 4),
    }
}

/// OP-V (0x57): vector configuration (funct3 == 0b111) and integer arithmetic
/// (OPIVV/OPIVX/OPIVI).
fn decode_vector(w: u32) -> Insn {
    let f3 = funct3(w);
    // Integer arithmetic: OPIVV(000) / OPIVI(011) / OPIVX(100), op in funct6.
    if f3 == 0b000 || f3 == 0b011 || f3 == 0b100 {
        let funct6 = w >> 26;
        let op = match funct6 {
            0b000000 => Op::Vadd,
            0b000010 if f3 != 0b011 => Op::Vsub, // no OPIVI form
            0b000011 if f3 != 0b000 => Op::Vrsub, // no OPIVV form
            0b001001 => Op::Vand,
            0b001010 => Op::Vor,
            0b001011 => Op::Vxor,
            0b000100 if f3 != 0b011 => Op::Vminu, // vv/vx
            0b000101 if f3 != 0b011 => Op::Vmin,
            0b000110 if f3 != 0b011 => Op::Vmaxu,
            0b000111 if f3 != 0b011 => Op::Vmax,
            0b100101 => Op::Vsll,
            0b101000 => Op::Vsrl,
            0b101001 => Op::Vsra,
            0b010111 => Op::Vmerge, // vmerge (vm=0) / vmv.v.* (vm=1)
            0b011000 => Op::Vmseq,  // vv/vx/vi
            0b011001 => Op::Vmsne,
            0b011010 if f3 != 0b011 => Op::Vmsltu, // vv/vx
            0b011011 if f3 != 0b011 => Op::Vmslt,
            0b011100 => Op::Vmsleu, // vv/vx/vi
            0b011101 => Op::Vmsle,
            0b011110 if f3 != 0b000 => Op::Vmsgtu, // vx/vi
            0b011111 if f3 != 0b000 => Op::Vmsgt,
            // Slides (vx/vi forms; OPIVV 0b001110 is vrgatherei16).
            0b001110 if f3 != 0b000 => Op::Vslideup,
            0b001111 if f3 != 0b000 => Op::Vslidedown,
            // Gathers: vrgather (vv/vx/vi), vrgatherei16 (vv only).
            0b001100 => Op::Vrgather,
            0b001110 if f3 == 0b000 => Op::Vrgatherei16,
            // Widening integer sum reductions (.vs form, OPIVV only).
            0b110000 if f3 == 0b000 => Op::Vwredsumu,
            0b110001 if f3 == 0b000 => Op::Vwredsum,
            // Add/subtract with carry/borrow (vsbc/vmsbc have no immediate form).
            0b010000 => Op::Vadc,
            0b010001 => Op::Vmadc,
            0b010010 if f3 != 0b011 => Op::Vsbc,
            0b010011 if f3 != 0b011 => Op::Vmsbc,
            // Saturating add/subtract (ssub variants have no immediate form).
            0b100000 => Op::Vsaddu,
            0b100001 => Op::Vsadd,
            0b100010 if f3 != 0b011 => Op::Vssubu,
            0b100011 if f3 != 0b011 => Op::Vssub,
            // Scaling shift right (vv/vx/vi) and fractional multiply (vv/vx).
            0b101010 => Op::Vssrl,
            0b101011 => Op::Vssra,
            0b100111 if f3 != 0b011 => Op::Vsmul,
            0b100111 if f3 == 0b011 => Op::Vmvr, // vmv<nr>r.v whole-register move
            // Narrowing shift / clip (wide 2*SEW vs2 source).
            0b101100 => Op::Vnsrl,
            0b101101 => Op::Vnsra,
            0b101110 => Op::Vnclipu,
            0b101111 => Op::Vnclip,
            _ => return Insn::illegal(w, 4),
        };
        return base(op, w);
    }
    // OPMVV(010) / OPMVX(110): integer multiply/divide.
    if f3 == 0b010 || f3 == 0b110 {
        let funct6 = w >> 26;
        let op = match funct6 {
            0b100101 => Op::Vmul,
            0b100111 => Op::Vmulh,
            0b100100 => Op::Vmulhu,
            0b100110 => Op::Vmulhsu,
            0b100000 => Op::Vdivu,
            0b100001 => Op::Vdiv,
            0b100010 => Op::Vremu,
            0b100011 => Op::Vrem,
            // Integer reductions are OPMVV-only (funct3 == 0b010).
            0b000000 if f3 == 0b010 => Op::Vredsum,
            0b000001 if f3 == 0b010 => Op::Vredand,
            0b000010 if f3 == 0b010 => Op::Vredor,
            0b000011 if f3 == 0b010 => Op::Vredxor,
            0b000100 if f3 == 0b010 => Op::Vredminu,
            0b000101 if f3 == 0b010 => Op::Vredmin,
            0b000110 if f3 == 0b010 => Op::Vredmaxu,
            0b000111 if f3 == 0b010 => Op::Vredmax,
            // Scalar element moves (VWXUNARY0 / VRXUNARY0), funct6 = 010000.
            0b010000 if f3 == 0b010 && (w >> 15) & 0x1f == 0 => Op::VmvXS,
            0b010000 if f3 == 0b010 && (w >> 15) & 0x1f == 0b10000 => Op::Vcpop,
            0b010000 if f3 == 0b010 && (w >> 15) & 0x1f == 0b10001 => Op::Vfirst,
            0b010000 if f3 == 0b110 && (w >> 20) & 0x1f == 0 => Op::VmvSX,
            // Mask set / iota / id (VMUNARY0); vs1 field selects the variant.
            0b010100 if f3 == 0b010 => match (w >> 15) & 0x1f {
                0b00001 => Op::Vmsbf,
                0b00010 => Op::Vmsof,
                0b00011 => Op::Vmsif,
                0b10000 => Op::Viota,
                0b10001 => Op::Vid,
                _ => return Insn::illegal(w, 4),
            },
            // Averaging add/subtract (OPMVV/OPMVX).
            0b001000 => Op::Vaaddu,
            0b001001 => Op::Vaadd,
            0b001010 => Op::Vasubu,
            0b001011 => Op::Vasub,
            // Widening integer add/subtract (.w forms take a wide vs2).
            0b110000 => Op::Vwaddu,
            0b110001 => Op::Vwadd,
            0b110010 => Op::Vwsubu,
            0b110011 => Op::Vwsub,
            0b110100 => Op::VwadduW,
            0b110101 => Op::VwaddW,
            0b110110 => Op::VwsubuW,
            0b110111 => Op::VwsubW,
            // Widening multiply / multiply-accumulate (vwmaccus is vx-only).
            0b111000 => Op::Vwmulu,
            0b111010 => Op::Vwmulsu,
            0b111011 => Op::Vwmul,
            0b111100 => Op::Vwmaccu,
            0b111101 => Op::Vwmacc,
            0b111110 if f3 == 0b110 => Op::Vwmaccus,
            0b111111 => Op::Vwmaccsu,
            // Slide-by-one (OPMVX form, funct3 == 0b110).
            0b001110 if f3 == 0b110 => Op::Vslide1up,
            0b001111 if f3 == 0b110 => Op::Vslide1down,
            // Compress active vs2 elements into vd (OPMVV).
            0b010111 if f3 == 0b010 => Op::Vcompress,
            // Mask-register logical ops are OPMVV-only (funct3 == 0b010).
            0b011000 if f3 == 0b010 => Op::Vmandn,
            0b011001 if f3 == 0b010 => Op::Vmand,
            0b011010 if f3 == 0b010 => Op::Vmor,
            0b011011 if f3 == 0b010 => Op::Vmxor,
            0b011100 if f3 == 0b010 => Op::Vmorn,
            0b011101 if f3 == 0b010 => Op::Vmnand,
            0b011110 if f3 == 0b010 => Op::Vmnor,
            0b011111 if f3 == 0b010 => Op::Vmxnor,
            // Integer extension (VXUNARY0); the vs1 field selects the variant.
            0b010010 if f3 == 0b010 => match (w >> 15) & 0x1f {
                0b00010 => Op::VzextVf8,
                0b00011 => Op::VsextVf8,
                0b00100 => Op::VzextVf4,
                0b00101 => Op::VsextVf4,
                0b00110 => Op::VzextVf2,
                0b00111 => Op::VsextVf2,
                _ => return Insn::illegal(w, 4),
            },
            _ => return Insn::illegal(w, 4),
        };
        return base(op, w);
    }
    // OPFVV(001) / OPFVF(101): floating-point arithmetic.
    if f3 == 0b001 || f3 == 0b101 {
        let vf = f3 == 0b101;
        let funct6 = w >> 26;
        let vs1 = (w >> 15) & 0x1f;
        let op = match funct6 {
            0b000000 => Op::Vfadd,
            0b000010 => Op::Vfsub,
            0b100111 if vf => Op::Vfrsub,
            0b001110 if vf => Op::Vfslide1up,
            0b001111 if vf => Op::Vfslide1down,
            // Widening FP arithmetic.
            0b110000 => Op::Vfwadd,
            0b110010 => Op::Vfwsub,
            0b110100 => Op::VfwaddW,
            0b110110 => Op::VfwsubW,
            0b111000 => Op::Vfwmul,
            0b111100 => Op::Vfwmacc,
            0b111101 => Op::Vfwnmacc,
            0b111110 => Op::Vfwmsac,
            0b111111 => Op::Vfwnmsac,
            0b100100 => Op::Vfmul,
            0b100000 => Op::Vfdiv,
            0b100001 if vf => Op::Vfrdiv,
            0b000100 => Op::Vfmin,
            0b000110 => Op::Vfmax,
            0b001000 => Op::Vfsgnj,
            0b001001 => Op::Vfsgnjn,
            0b001010 => Op::Vfsgnjx,
            0b011000 => Op::Vmfeq,
            0b011001 => Op::Vmfle,
            0b011011 => Op::Vmflt,
            0b011100 => Op::Vmfne,
            0b011101 if vf => Op::Vmfgt,
            0b011111 if vf => Op::Vmfge,
            0b101100 => Op::Vfmacc,
            0b101101 => Op::Vfnmacc,
            0b101110 => Op::Vfmsac,
            0b101111 => Op::Vfnmsac,
            0b101000 => Op::Vfmadd,
            0b101001 => Op::Vfnmadd,
            0b101010 => Op::Vfmsub,
            0b101011 => Op::Vfnmsub,
            // FP reductions are OPFVV-only (.vs form).
            0b000001 if !vf => Op::Vfredusum,
            0b000011 if !vf => Op::Vfredosum,
            0b000101 if !vf => Op::Vfredmin,
            0b000111 if !vf => Op::Vfredmax,
            0b110001 if !vf => Op::Vfwredusum,
            0b110011 if !vf => Op::Vfwredosum,
            // FP scalar element moves (VWFUNARY0 / VRFUNARY0), funct6 = 010000.
            0b010000 if !vf && vs1 == 0 => Op::VfmvFS,
            0b010000 if vf && (w >> 20) & 0x1f == 0 => Op::VfmvSF,
            // Single-width conversions (VFUNARY0); vs1 field selects the variant.
            0b010010 if !vf => match vs1 {
                0b00000 => Op::VfcvtXuF,
                0b00001 => Op::VfcvtXF,
                0b00010 => Op::VfcvtFXu,
                0b00011 => Op::VfcvtFX,
                0b00110 => Op::VfcvtRtzXuF,
                0b00111 => Op::VfcvtRtzXF,
                0b01000 => Op::VfwcvtXuF,
                0b01001 => Op::VfwcvtXF,
                0b01010 => Op::VfwcvtFXu,
                0b01011 => Op::VfwcvtFX,
                0b01100 => Op::VfwcvtFF,
                0b01110 => Op::VfwcvtRtzXuF,
                0b01111 => Op::VfwcvtRtzXF,
                0b10000 => Op::VfncvtXuF,
                0b10001 => Op::VfncvtXF,
                0b10010 => Op::VfncvtFXu,
                0b10011 => Op::VfncvtFX,
                0b10100 => Op::VfncvtFF,
                0b10101 => Op::VfncvtRodFF,
                0b10110 => Op::VfncvtRtzXuF,
                0b10111 => Op::VfncvtRtzXF,
                _ => return Insn::illegal(w, 4),
            },
            0b010011 if !vf && vs1 == 0 => Op::Vfsqrt,
            0b010011 if !vf && vs1 == 0b00100 => Op::Vfrsqrt7,
            0b010011 if !vf && vs1 == 0b00101 => Op::Vfrec7,
            0b010011 if !vf && vs1 == 0b10000 => Op::Vfclass,
            _ => return Insn::illegal(w, 4),
        };
        return base(op, w);
    }
    if f3 != 0b111 {
        return Insn::illegal(w, 4);
    }
    if (w >> 31) & 1 == 0 {
        // vsetvli: 11-bit vtypei in bits[30:20].
        let mut i = base(Op::Vsetvli, w);
        i.imm = ((w >> 20) & 0x7ff) as i64;
        i
    } else if (w >> 30) & 1 == 1 {
        // vsetivli: 10-bit vtypei in bits[29:20]; AVL is the 5-bit rs1 field.
        let mut i = base(Op::Vsetivli, w);
        i.imm = ((w >> 20) & 0x3ff) as i64;
        i
    } else if (w >> 25) & 0x3f == 0 {
        // vsetvl: vtype comes from rs2.
        base(Op::Vsetvl, w)
    } else {
        Insn::illegal(w, 4)
    }
}

#[inline]
fn with_imm(op: Op, w: u32, imm: i64) -> Insn {
    let mut i = base(op, w);
    i.imm = imm;
    i
}

fn decode_branch(w: u32) -> Insn {
    let op = match funct3(w) {
        0 => Op::Beq,
        1 => Op::Bne,
        4 => Op::Blt,
        5 => Op::Bge,
        6 => Op::Bltu,
        7 => Op::Bgeu,
        _ => return Insn::illegal(w, 4),
    };
    with_imm(op, w, imm_b(w))
}

fn decode_load(w: u32, rv64: bool) -> Insn {
    let op = match funct3(w) {
        0 => Op::Lb,
        1 => Op::Lh,
        2 => Op::Lw,
        3 if rv64 => Op::Ld,
        4 => Op::Lbu,
        5 => Op::Lhu,
        6 if rv64 => Op::Lwu,
        _ => return Insn::illegal(w, 4),
    };
    with_imm(op, w, imm_i(w))
}

fn decode_store(w: u32, rv64: bool) -> Insn {
    let op = match funct3(w) {
        0 => Op::Sb,
        1 => Op::Sh,
        2 => Op::Sw,
        3 if rv64 => Op::Sd,
        _ => return Insn::illegal(w, 4),
    };
    with_imm(op, w, imm_s(w))
}

fn decode_op_imm(w: u32, rv64: bool, isa: &Isa) -> Insn {
    match funct3(w) {
        0 => with_imm(Op::Addi, w, imm_i(w)),
        2 => with_imm(Op::Slti, w, imm_i(w)),
        3 => with_imm(Op::Sltiu, w, imm_i(w)),
        4 => with_imm(Op::Xori, w, imm_i(w)),
        6 => with_imm(Op::Ori, w, imm_i(w)),
        7 => with_imm(Op::Andi, w, imm_i(w)),
        1 => decode_shift_left_imm(w, rv64, isa),
        5 => decode_shift_right_imm(w, rv64, isa),
        _ => Insn::illegal(w, 4),
    }
}

// OP-IMM funct3==1 (SLLI and Zbb/Zbs left-shift-immediate overlays).
fn decode_shift_left_imm(w: u32, rv64: bool, isa: &Isa) -> Insn {
    let funct6 = (w >> 26) & 0x3f;
    let funct7 = funct7(w);
    let shamt = ((w >> 20) & if rv64 { 0x3f } else { 0x1f }) as i64;
    let rs2f = rs2(w);
    // SHA / SM3 message-schedule transforms (funct7 = 0b0001000).
    if funct7 == 0b0001000 {
        match rs2f {
            0b00000 if isa.zknh => return base(Op::Sha256Sum0, w),
            0b00001 if isa.zknh => return base(Op::Sha256Sum1, w),
            0b00010 if isa.zknh => return base(Op::Sha256Sig0, w),
            0b00011 if isa.zknh => return base(Op::Sha256Sig1, w),
            0b00100 if isa.zknh => return base(Op::Sha512Sum0, w),
            0b00101 if isa.zknh => return base(Op::Sha512Sum1, w),
            0b00110 if isa.zknh => return base(Op::Sha512Sig0, w),
            0b00111 if isa.zknh => return base(Op::Sha512Sig1, w),
            0b01000 if isa.zksh => return base(Op::Sm3p0, w),
            0b01001 if isa.zksh => return base(Op::Sm3p1, w),
            _ => {}
        }
    }
    // AES-64 decrypt InvMixColumns / key-schedule step 1 (funct7 = 0b0011000).
    if funct7 == 0b0011000 {
        if rs2f == 0 && isa.zknd {
            return base(Op::Aes64im, w);
        }
        if rs2f & 0b10000 != 0 && (isa.zkne || isa.zknd) {
            // aes64ks1i: rnum in rs2[3:0], must be <= 0xA.
            if (rs2f & 0xf) <= 0xA {
                return base(Op::Aes64ks1i, w);
            }
        }
    }
    // CLZ/CTZ/CPOP/SEXT.B/SEXT.H share funct7=0b0110000.
    if isa.zbb && funct7 == 0b0110000 {
        let op = match rs2f {
            0b00000 => Op::Clz,
            0b00001 => Op::Ctz,
            0b00010 => Op::Cpop,
            0b00100 => Op::SextB,
            0b00101 => Op::SextH,
            _ => return Insn::illegal(w, 4),
        };
        return base(op, w);
    }
    if isa.zbs {
        match funct6 {
            0b010010 => return with_imm(Op::Bclri, w, shamt),
            0b011010 => return with_imm(Op::Binvi, w, shamt),
            0b001010 => return with_imm(Op::Bseti, w, shamt),
            _ => {}
        }
    }
    // SLLI: funct6 must be zero (RV64) / funct7 zero (RV32).
    if (rv64 && funct6 == 0) || (!rv64 && funct7 == 0) {
        return with_imm(Op::Slli, w, shamt);
    }
    Insn::illegal(w, 4)
}

// OP-IMM funct3==5 (SRLI/SRAI and Zbb/Zbs right-shift-immediate overlays).
fn decode_shift_right_imm(w: u32, rv64: bool, isa: &Isa) -> Insn {
    let funct6 = (w >> 26) & 0x3f;
    let funct7 = funct7(w);
    let rs2f = rs2(w);
    let shamt = ((w >> 20) & if rv64 { 0x3f } else { 0x1f }) as i64;
    if isa.zbb {
        // ORC.B: funct7=0b0010100, rs2=0b00111.
        if funct7 == 0b0010100 && rs2f == 0b00111 {
            return base(Op::Orcb, w);
        }
        // REV8: RV64 funct12=0b011010111000, RV32 funct12=0b011010011000.
        let funct12 = (w >> 20) & 0xfff;
        if (rv64 && funct12 == 0b0110_1011_1000) || (!rv64 && funct12 == 0b0110_1001_1000) {
            return base(Op::Rev8, w);
        }
    }
    // Zbkb brev8: funct7=0b0110100, rs2=0b00111, funct3=5.
    if isa.zbkb && funct7 == 0b0110100 && rs2f == 0b00111 {
        return base(Op::Brev8, w);
    }
    if isa.zbb {
        if funct6 == 0b011000 {
            return with_imm(Op::Rori, w, shamt);
        }
    }
    if isa.zbs && funct6 == 0b010010 {
        return with_imm(Op::Bexti, w, shamt);
    }
    match funct6 {
        0b000000 => with_imm(Op::Srli, w, shamt),
        0b010000 => with_imm(Op::Srai, w, shamt),
        _ if !rv64 && funct7 == 0b0000000 => with_imm(Op::Srli, w, shamt),
        _ if !rv64 && funct7 == 0b0100000 => with_imm(Op::Srai, w, shamt),
        _ => Insn::illegal(w, 4),
    }
}

// OP-IMM-32 (RV64 word immediate ops + Zba/Zbb word overlays).
fn decode_op_imm32(w: u32, isa: &Isa) -> Insn {
    let funct7 = funct7(w);
    let funct6 = (w >> 26) & 0x3f;
    let rs2f = rs2(w);
    let shamt5 = ((w >> 20) & 0x1f) as i64;
    let shamt6 = ((w >> 20) & 0x3f) as i64;
    match funct3(w) {
        0 => with_imm(Op::Addiw, w, imm_i(w)),
        1 => {
            if isa.zba && funct6 == 0b000010 {
                return with_imm(Op::SlliUw, w, shamt6);
            }
            if isa.zbb && funct7 == 0b0110000 {
                let op = match rs2f {
                    0b00000 => Op::Clzw,
                    0b00001 => Op::Ctzw,
                    0b00010 => Op::Cpopw,
                    _ => return Insn::illegal(w, 4),
                };
                return base(op, w);
            }
            if funct7 == 0 {
                return with_imm(Op::Slliw, w, shamt5);
            }
            Insn::illegal(w, 4)
        }
        5 => {
            if isa.zbb && funct7 == 0b0110000 {
                return with_imm(Op::Roriw, w, shamt5);
            }
            match funct7 {
                0b0000000 => with_imm(Op::Srliw, w, shamt5),
                0b0100000 => with_imm(Op::Sraiw, w, shamt5),
                _ => Insn::illegal(w, 4),
            }
        }
        _ => Insn::illegal(w, 4),
    }
}

// OP (R-type): base, M, Zba/Zbb/Zbc/Zbs overlays.
fn decode_op(w: u32, isa: &Isa) -> Insn {
    let f3 = funct3(w);
    let f7 = funct7(w);
    // M extension.
    if isa.m && f7 == 0b0000001 {
        let op = match f3 {
            0 => Op::Mul,
            1 => Op::Mulh,
            2 => Op::Mulhsu,
            3 => Op::Mulhu,
            4 => Op::Div,
            5 => Op::Divu,
            6 => Op::Rem,
            7 => Op::Remu,
            _ => unreachable!(),
        };
        return base(op, w);
    }
    // Zba.
    if isa.zba && f7 == 0b0010000 {
        let op = match f3 {
            2 => Op::Sh1add,
            4 => Op::Sh2add,
            6 => Op::Sh3add,
            _ => return Insn::illegal(w, 4),
        };
        return base(op, w);
    }
    // Zbb logical-with-negate.
    if isa.zbb && f7 == 0b0100000 {
        match f3 {
            7 => return base(Op::Andn, w),
            6 => return base(Op::Orn, w),
            4 => return base(Op::Xnor, w),
            _ => {}
        }
    }
    // Zbb rotate.
    if isa.zbb && f7 == 0b0110000 {
        match f3 {
            1 => return base(Op::Rol, w),
            5 => return base(Op::Ror, w),
            _ => {}
        }
    }
    // Zbb min/max and Zbc carry-less multiply share funct7=0b0000101.
    if f7 == 0b0000101 {
        if isa.zbc {
            match f3 {
                1 => return base(Op::Clmul, w),
                2 => return base(Op::Clmulr, w),
                3 => return base(Op::Clmulh, w),
                _ => {}
            }
        }
        if isa.zbb {
            match f3 {
                4 => return base(Op::Min, w),
                5 => return base(Op::Minu, w),
                6 => return base(Op::Max, w),
                7 => return base(Op::Maxu, w),
                _ => {}
            }
        }
    }
    // Zbs single-bit (register).
    if isa.zbs {
        match (f7, f3) {
            (0b0100100, 1) => return base(Op::Bclr, w),
            (0b0100100, 5) => return base(Op::Bext, w),
            (0b0110100, 1) => return base(Op::Binv, w),
            (0b0010100, 1) => return base(Op::Bset, w),
            _ => {}
        }
    }
    // Zicond integer conditional.
    if isa.zicond && f7 == 0b0000111 {
        match f3 {
            5 => return base(Op::CzeroEqz, w),
            7 => return base(Op::CzeroNez, w),
            _ => {}
        }
    }
    // Zbkb pack/packh.
    if isa.zbkb && f7 == 0b0000100 {
        match f3 {
            4 => return base(Op::Pack, w),
            7 => return base(Op::Packh, w),
            _ => {}
        }
    }
    // Zbkx crossbar permute.
    if isa.zbkx && f7 == 0b0010100 {
        match f3 {
            2 => return base(Op::Xperm4, w),
            4 => return base(Op::Xperm8, w),
            _ => {}
        }
    }
    if f3 == 0 {
        // AES-64 round / key-schedule (Zkne / Zknd).
        match f7 {
            0b0011001 if isa.zkne => return base(Op::Aes64es, w),
            0b0011011 if isa.zkne => return base(Op::Aes64esm, w),
            0b0011101 if isa.zknd => return base(Op::Aes64ds, w),
            0b0011111 if isa.zknd => return base(Op::Aes64dsm, w),
            0b0111111 if isa.zkne || isa.zknd => return base(Op::Aes64ks2, w),
            _ => {}
        }
        // SM4 (Zksed): funct7 low 5 bits select ed/ks, top 2 bits carry `bs`.
        if isa.zksed {
            match f7 & 0b0011111 {
                0b11000 => return base(Op::Sm4ed, w),
                0b11010 => return base(Op::Sm4ks, w),
                _ => {}
            }
        }
    }
    // Base RV32I/RV64I.
    let op = match (f7, f3) {
        (0b0000000, 0) => Op::Add,
        (0b0100000, 0) => Op::Sub,
        (0b0000000, 1) => Op::Sll,
        (0b0000000, 2) => Op::Slt,
        (0b0000000, 3) => Op::Sltu,
        (0b0000000, 4) => Op::Xor,
        (0b0000000, 5) => Op::Srl,
        (0b0100000, 5) => Op::Sra,
        (0b0000000, 6) => Op::Or,
        (0b0000000, 7) => Op::And,
        _ => return Insn::illegal(w, 4),
    };
    base(op, w)
}

// OP-32 (RV64 R-type word ops): base, M, Zba/Zbb overlays.
fn decode_op32(w: u32, isa: &Isa) -> Insn {
    let f3 = funct3(w);
    let f7 = funct7(w);
    if isa.m && f7 == 0b0000001 {
        let op = match f3 {
            0 => Op::Mulw,
            4 => Op::Divw,
            5 => Op::Divuw,
            6 => Op::Remw,
            7 => Op::Remuw,
            _ => return Insn::illegal(w, 4),
        };
        return base(op, w);
    }
    if isa.zba {
        match (f7, f3) {
            (0b0000100, 0) => return base(Op::AddUw, w),
            (0b0010000, 2) => return base(Op::Sh1addUw, w),
            (0b0010000, 4) => return base(Op::Sh2addUw, w),
            (0b0010000, 6) => return base(Op::Sh3addUw, w),
            _ => {}
        }
    }
    if isa.zbb {
        // ZEXT.H (RV64): funct7=0b0000100, funct3=4, rs2=0.
        if f7 == 0b0000100 && f3 == 4 && rs2(w) == 0 {
            return base(Op::ZextH, w);
        }
        if f7 == 0b0110000 {
            match f3 {
                1 => return base(Op::Rolw, w),
                5 => return base(Op::Rorw, w),
                _ => {}
            }
        }
    }
    // Zbkb packw (RV64): funct7=0b0000100, funct3=4, rs2 != 0.
    if isa.zbkb && f7 == 0b0000100 && f3 == 4 && rs2(w) != 0 {
        return base(Op::Packw, w);
    }
    let op = match (f7, f3) {
        (0b0000000, 0) => Op::Addw,
        (0b0100000, 0) => Op::Subw,
        (0b0000000, 1) => Op::Sllw,
        (0b0000000, 5) => Op::Srlw,
        (0b0100000, 5) => Op::Sraw,
        _ => return Insn::illegal(w, 4),
    };
    base(op, w)
}

fn decode_fence(w: u32, isa: &Isa) -> Insn {
    match funct3(w) {
        0 => base(Op::Fence, w),
        1 if isa.zifencei => base(Op::FenceI, w),
        _ => Insn::illegal(w, 4),
    }
}

fn decode_system(w: u32, isa: &Isa) -> Insn {
    let f3 = funct3(w);
    if f3 == 0 {
        // PRIV: distinguished by the full 12-bit funct12 and rs1/rd == 0.
        let funct12 = (w >> 20) & 0xfff;
        return match funct12 {
            0x000 => base(Op::Ecall, w),
            0x001 => base(Op::Ebreak, w),
            0x302 => base(Op::Mret, w),
            0x102 => base(Op::Sret, w),
            0x105 => base(Op::Wfi, w),
            _ => Insn::illegal(w, 4),
        };
    }
    if !isa.zicsr {
        return Insn::illegal(w, 4);
    }
    let op = match f3 {
        1 => Op::Csrrw,
        2 => Op::Csrrs,
        3 => Op::Csrrc,
        5 => Op::Csrrwi,
        6 => Op::Csrrsi,
        7 => Op::Csrrci,
        _ => return Insn::illegal(w, 4),
    };
    base(op, w)
}

fn decode_amo(w: u32, rv64: bool) -> Insn {
    let f3 = funct3(w);
    let funct5 = (w >> 27) & 0x1f;
    let is_d = match f3 {
        0b010 => false, // .W
        0b011 if rv64 => true,
        _ => return Insn::illegal(w, 4),
    };
    let op = match (funct5, is_d) {
        (0b00010, false) => Op::LrW,
        (0b00011, false) => Op::ScW,
        (0b00001, false) => Op::AmoswapW,
        (0b00000, false) => Op::AmoaddW,
        (0b00100, false) => Op::AmoxorW,
        (0b01100, false) => Op::AmoandW,
        (0b01000, false) => Op::AmoorW,
        (0b10000, false) => Op::AmominW,
        (0b10100, false) => Op::AmomaxW,
        (0b11000, false) => Op::AmominuW,
        (0b11100, false) => Op::AmomaxuW,
        (0b00010, true) => Op::LrD,
        (0b00011, true) => Op::ScD,
        (0b00001, true) => Op::AmoswapD,
        (0b00000, true) => Op::AmoaddD,
        (0b00100, true) => Op::AmoxorD,
        (0b01100, true) => Op::AmoandD,
        (0b01000, true) => Op::AmoorD,
        (0b10000, true) => Op::AmominD,
        (0b10100, true) => Op::AmomaxD,
        (0b11000, true) => Op::AmominuD,
        (0b11100, true) => Op::AmomaxuD,
        _ => return Insn::illegal(w, 4),
    };
    // LR requires rs2 == 0.
    if matches!(op, Op::LrW | Op::LrD) && rs2(w) != 0 {
        return Insn::illegal(w, 4);
    }
    base(op, w)
}

fn decode_load_fp(w: u32, isa: &Isa) -> Insn {
    let f3 = funct3(w);
    // Vector unit-stride load (width 8/16/32/64 in funct3).
    if isa.v && matches!(f3, 0 | 5 | 6 | 7) {
        let nf = (w >> 29) & 7;
        let mop = (w >> 26) & 3;
        let lumop = (w >> 20) & 0x1f;
        return match mop {
            0b00 => match lumop {
                0b00000 if nf == 0 => base(Op::Vle, w),
                0b00000 => base(Op::Vlseg, w), // unit-stride segment (nf+1 fields)
                0b01000 => base(Op::Vlre, w),  // whole register (nf+1 regs)
                0b01011 if nf == 0 => base(Op::Vlm, w),
                0b10000 if nf == 0 => base(Op::Vleff, w), // fault-only-first
                _ => Insn::illegal(w, 4),
            },
            0b10 if nf == 0 => base(Op::Vlse, w), // strided
            0b10 => base(Op::Vlseg, w),           // strided segment
            0b01 | 0b11 if nf == 0 => base(Op::Vlxei, w), // indexed
            0b01 | 0b11 => base(Op::Vlseg, w),    // indexed segment
            _ => Insn::illegal(w, 4),
        };
    }
    let op = match f3 {
        1 if isa.zfh => Op::Flh,
        2 => Op::Flw,
        3 if isa.d => Op::Fld,
        _ => return Insn::illegal(w, 4),
    };
    with_imm(op, w, imm_i(w))
}

fn decode_store_fp(w: u32, isa: &Isa) -> Insn {
    let f3 = funct3(w);
    // Vector unit-stride store (width 8/16/32/64 in funct3).
    if isa.v && matches!(f3, 0 | 5 | 6 | 7) {
        let nf = (w >> 29) & 7;
        let mop = (w >> 26) & 3;
        let sumop = (w >> 20) & 0x1f;
        return match mop {
            0b00 => match sumop {
                0b00000 if nf == 0 => base(Op::Vse, w),
                0b00000 => base(Op::Vsseg, w), // unit-stride segment
                0b01000 => base(Op::Vsre, w),  // whole register
                0b01011 if nf == 0 => base(Op::Vsm, w),
                _ => Insn::illegal(w, 4),
            },
            0b10 if nf == 0 => base(Op::Vsse, w), // strided
            0b10 => base(Op::Vsseg, w),           // strided segment
            0b01 | 0b11 if nf == 0 => base(Op::Vsxei, w), // indexed
            0b01 | 0b11 => base(Op::Vsseg, w),    // indexed segment
            _ => Insn::illegal(w, 4),
        };
    }
    let op = match f3 {
        1 if isa.zfh => Op::Fsh,
        2 => Op::Fsw,
        3 if isa.d => Op::Fsd,
        _ => return Insn::illegal(w, 4),
    };
    with_imm(op, w, imm_s(w))
}

fn decode_fma(single: Op, double: Op, half: Op, w: u32, isa: &Isa) -> Insn {
    match funct2(w) {
        0b00 => base(single, w),
        0b01 if isa.d => base(double, w),
        0b10 if isa.zfh => base(half, w),
        _ => Insn::illegal(w, 4),
    }
}

// OP-FP (0x53): selected by funct7, with funct3 reused as rm or sub-op.
/// Zfa additional FP ops layered over the OP-FP encoding space.
fn decode_zfa(f7: u32, f3: u8, rs2f: u8, isa: &Isa) -> Option<Op> {
    Some(match (f7, f3, rs2f) {
        (0b0010100, 2, _) => Op::FminmS,
        (0b0010100, 3, _) => Op::FmaxmS,
        (0b0010101, 2, _) if isa.d => Op::FminmD,
        (0b0010101, 3, _) if isa.d => Op::FmaxmD,
        (0b0100000, _, 4) => Op::FroundS,
        (0b0100000, _, 5) => Op::FroundnxS,
        (0b0100001, _, 4) if isa.d => Op::FroundD,
        (0b0100001, _, 5) if isa.d => Op::FroundnxD,
        (0b1010000, 4, _) => Op::FleqS,
        (0b1010000, 5, _) => Op::FltqS,
        (0b1010001, 4, _) if isa.d => Op::FleqD,
        (0b1010001, 5, _) if isa.d => Op::FltqD,
        (0b1111000, 0, 1) => Op::FliS,
        (0b1111001, 0, 1) if isa.d => Op::FliD,
        (0b1100001, 1, 8) if isa.d => Op::FcvtmodWD,
        _ => return None,
    })
}

/// Zfh half-precision ops layered over the OP-FP encoding space (`fmt = 10`).
fn decode_zfh(f7: u32, f3: u8, rs2f: u8, rv64: bool, isa: &Isa) -> Option<Op> {
    let zfa = isa.zfa;
    Some(match (f7, f3, rs2f) {
        (0b0000010, _, _) => Op::FaddH,
        (0b0000110, _, _) => Op::FsubH,
        (0b0001010, _, _) => Op::FmulH,
        (0b0001110, _, _) => Op::FdivH,
        (0b0101110, _, 0) => Op::FsqrtH,
        (0b0010010, 0, _) => Op::FsgnjH,
        (0b0010010, 1, _) => Op::FsgnjnH,
        (0b0010010, 2, _) => Op::FsgnjxH,
        (0b0010110, 0, _) => Op::FminH,
        (0b0010110, 1, _) => Op::FmaxH,
        (0b0010110, 2, _) if zfa => Op::FminmH,
        (0b0010110, 3, _) if zfa => Op::FmaxmH,
        (0b0100000, _, 2) => Op::FcvtSH,
        (0b0100010, _, 0) => Op::FcvtHS,
        (0b0100001, _, 2) if isa.d => Op::FcvtDH,
        (0b0100010, _, 1) if isa.d => Op::FcvtHD,
        (0b0100010, _, 4) if zfa => Op::FroundH,
        (0b0100010, _, 5) if zfa => Op::FroundnxH,
        (0b1100010, _, 0) => Op::FcvtWH,
        (0b1100010, _, 1) => Op::FcvtWuH,
        (0b1100010, _, 2) if rv64 => Op::FcvtLH,
        (0b1100010, _, 3) if rv64 => Op::FcvtLuH,
        (0b1101010, _, 0) => Op::FcvtHW,
        (0b1101010, _, 1) => Op::FcvtHWu,
        (0b1101010, _, 2) if rv64 => Op::FcvtHL,
        (0b1101010, _, 3) if rv64 => Op::FcvtHLu,
        (0b1110010, 0, 0) => Op::FmvXH,
        (0b1110010, 1, 0) => Op::FclassH,
        (0b1111010, 0, 0) => Op::FmvHX,
        (0b1111010, 0, 1) if zfa => Op::FliH,
        (0b1010010, 0, _) => Op::FleH,
        (0b1010010, 1, _) => Op::FltH,
        (0b1010010, 2, _) => Op::FeqH,
        (0b1010010, 4, _) if zfa => Op::FleqH,
        (0b1010010, 5, _) if zfa => Op::FltqH,
        _ => return None,
    })
}

fn decode_op_fp(w: u32, rv64: bool, isa: &Isa) -> Insn {
    let f7 = funct7(w);
    let f3 = funct3(w);
    let rs2f = rs2(w);
    let d = isa.d;
    if isa.zfa {
        if let Some(op) = decode_zfa(f7, f3, rs2f, isa) {
            return base(op, w);
        }
    }
    if isa.zfh {
        if let Some(op) = decode_zfh(f7, f3, rs2f, rv64, isa) {
            return base(op, w);
        }
    }
    let op = match f7 {
        0b0000000 => Op::FaddS,
        0b0000001 if d => Op::FaddD,
        0b0000100 => Op::FsubS,
        0b0000101 if d => Op::FsubD,
        0b0001000 => Op::FmulS,
        0b0001001 if d => Op::FmulD,
        0b0001100 => Op::FdivS,
        0b0001101 if d => Op::FdivD,
        0b0101100 if rs2f == 0 => Op::FsqrtS,
        0b0101101 if d && rs2f == 0 => Op::FsqrtD,
        0b0010000 => match f3 {
            0 => Op::FsgnjS,
            1 => Op::FsgnjnS,
            2 => Op::FsgnjxS,
            _ => return Insn::illegal(w, 4),
        },
        0b0010001 if d => match f3 {
            0 => Op::FsgnjD,
            1 => Op::FsgnjnD,
            2 => Op::FsgnjxD,
            _ => return Insn::illegal(w, 4),
        },
        0b0010100 => match f3 {
            0 => Op::FminS,
            1 => Op::FmaxS,
            _ => return Insn::illegal(w, 4),
        },
        0b0010101 if d => match f3 {
            0 => Op::FminD,
            1 => Op::FmaxD,
            _ => return Insn::illegal(w, 4),
        },
        0b0100000 if d && rs2f == 1 => Op::FcvtSD,
        0b0100001 if d && rs2f == 0 => Op::FcvtDS,
        0b1100000 => match rs2f {
            0 => Op::FcvtWS,
            1 => Op::FcvtWuS,
            2 if rv64 => Op::FcvtLS,
            3 if rv64 => Op::FcvtLuS,
            _ => return Insn::illegal(w, 4),
        },
        0b1100001 if d => match rs2f {
            0 => Op::FcvtWD,
            1 => Op::FcvtWuD,
            2 if rv64 => Op::FcvtLD,
            3 if rv64 => Op::FcvtLuD,
            _ => return Insn::illegal(w, 4),
        },
        0b1101000 => match rs2f {
            0 => Op::FcvtSW,
            1 => Op::FcvtSWu,
            2 if rv64 => Op::FcvtSL,
            3 if rv64 => Op::FcvtSLu,
            _ => return Insn::illegal(w, 4),
        },
        0b1101001 if d => match rs2f {
            0 => Op::FcvtDW,
            1 => Op::FcvtDWu,
            2 if rv64 => Op::FcvtDL,
            3 if rv64 => Op::FcvtDLu,
            _ => return Insn::illegal(w, 4),
        },
        0b1110000 if rs2f == 0 => match f3 {
            0 => Op::FmvXW,
            1 => Op::FclassS,
            _ => return Insn::illegal(w, 4),
        },
        0b1110001 if d && rs2f == 0 => match f3 {
            0 if rv64 => Op::FmvXD,
            1 => Op::FclassD,
            _ => return Insn::illegal(w, 4),
        },
        0b1010000 => match f3 {
            0 => Op::FleS,
            1 => Op::FltS,
            2 => Op::FeqS,
            _ => return Insn::illegal(w, 4),
        },
        0b1010001 if d => match f3 {
            0 => Op::FleD,
            1 => Op::FltD,
            2 => Op::FeqD,
            _ => return Insn::illegal(w, 4),
        },
        0b1111000 if rs2f == 0 && f3 == 0 => Op::FmvWX,
        0b1111001 if d && rv64 && rs2f == 0 && f3 == 0 => Op::FmvDX,
        _ => return Insn::illegal(w, 4),
    };
    base(op, w)
}

/// Fetch and decode the instruction at `pc`, selecting 16- or 32-bit width.
pub fn decode_at(mem: &dyn Memory, pc: u64, xlen: Xlen, isa: &Isa) -> Result<Insn, DecodeError> {
    let lo = mem.read_u16(pc).map_err(DecodeError::Fetch)?;
    if lo & 0b11 != 0b11 {
        // 16-bit compressed parcel.
        return Ok(decode_compressed(lo, xlen, isa));
    }
    let hi = mem.read_u16(pc + 2).map_err(DecodeError::Fetch)?;
    let w = (lo as u32) | ((hi as u32) << 16);
    Ok(decode(w, xlen, isa))
}

/// Decode a 16-bit compressed parcel. When `C` is disabled (or the parcel is
/// the reserved all-zero word) the result is an illegal instruction.
pub fn decode_compressed(half: u16, xlen: Xlen, isa: &Isa) -> Insn {
    if !isa.c || half == 0 {
        return Insn::illegal(half as u32, 2);
    }
    decode_rvc(half, xlen, isa)
}

/// Decode a non-zero 16-bit compressed parcel into the equivalent base
/// operation. Implemented in full by the C-extension phase.
fn decode_rvc(half: u16, xlen: Xlen, isa: &Isa) -> Insn {
    super::rvc::decode_rvc(half, xlen, isa)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(w: u32) -> Insn {
        decode(w, Xlen::Rv64, &Isa::rv64gc())
    }

    #[test]
    fn decode_addi() {
        // addi a0, a1, 5  => imm=5, rs1=11, rd=10, funct3=0, opcode=0x13
        let w = (5u32 << 20) | (11 << 15) | (0 << 12) | (10 << 7) | 0x13;
        let i = dec(w);
        assert_eq!(i.op, Op::Addi);
        assert_eq!(i.rd, 10);
        assert_eq!(i.rs1, 11);
        assert_eq!(i.imm, 5);
    }

    #[test]
    fn decode_add_sub() {
        let add = (1u32 << 20) | (2 << 15) | (0 << 12) | (3 << 7) | 0x33;
        assert_eq!(dec(add).op, Op::Add);
        let sub = (0b0100000u32 << 25) | (1 << 20) | (2 << 15) | (0 << 12) | (3 << 7) | 0x33;
        assert_eq!(dec(sub).op, Op::Sub);
    }

    #[test]
    fn decode_branch_imm() {
        // beq x1, x2, +8 : imm=8
        // imm[12|10:5]=funct7, imm[4:1|11]=rd-ish. Build via fields.
        // For +8: bit3 set. b4_1 = 0b0100 (=4 -> *2=8). b11=0,b12=0,b10_5=0.
        let b4_1 = 0b0100u32; // bits [4:1] => value 8
        let w = (b4_1 << 8) | (2 << 20) | (1 << 15) | (0 << 12) | 0x63;
        let i = dec(w);
        assert_eq!(i.op, Op::Beq);
        assert_eq!(i.imm, 8);
    }

    #[test]
    fn decode_m_mul() {
        let w = (0b0000001u32 << 25) | (2 << 20) | (1 << 15) | (0 << 12) | (3 << 7) | 0x33;
        assert_eq!(dec(w).op, Op::Mul);
    }

    #[test]
    fn decode_slli_rv64() {
        // slli a0, a1, 40 (6-bit shamt)
        let w = (40u32 << 20) | (11 << 15) | (1 << 12) | (10 << 7) | 0x13;
        let i = dec(w);
        assert_eq!(i.op, Op::Slli);
        assert_eq!(i.imm, 40);
    }
}
