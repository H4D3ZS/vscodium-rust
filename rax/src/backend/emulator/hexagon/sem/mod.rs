//! Direct opcode-dispatch semantic execution layer.
//!
//! This complements the legacy `DecodedInsn` interpreter in `cpu.rs`: when that
//! path yields `Unknown`, the packet driver calls [`dispatch`], which routes the
//! decoded opcode to a per-class handler that reads operand fields straight from
//! the [`DecodedOp`] and records its effects into the in-flight packet state.
//!
//! Every handler is verified against the `qemu-hexagon` reference oracle
//! (`tests/hexagon_diff.rs`); semantics are taken verbatim from the Hexagon
//! V68 spec (`tools/hexagon/qemu/imported/*.idef`, expanded via gen_semantics.c).
//!
//! Register reads observe the *old* architectural state (Hexagon reads
//! previous-packet values); writes are buffered in `new_r`/`new_p` and committed
//! when the packet completes, matching VLIW semantics.

use super::opcode::DecodedOp;
use crate::cpu::HexagonRegisters;

mod alu;
mod alu_ext;
mod alu_pred;
mod bitmanip;
mod compare;
mod extra;
mod extra2;
mod float;
mod float_ext;
mod hvx;
mod hvx_addsub;
mod hvx_carry;
mod hvx_cmp;
mod hvx_cmpacc;
mod hvx_cmpy;
mod hvx_hist;
mod hvx_lut;
mod hvx_minmax;
mod hvx_misc;
mod hvx_mpy;
mod hvx_mpys;
mod hvx_mpyv;
mod hvx_perm;
mod hvx_permx;
mod hvx_predop;
mod hvx_rmpy;
mod hvx_round;
mod hvx_shift;
mod hvx_v6mpy;
mod mpy;
mod mpy_ext;
mod shift;
mod shift_ext;
mod vecalu;

/// USR sticky overflow / saturation bit (`USR:0`).
pub const USR_OVF: u32 = 1 << 0;

/// Mutable execution context handed to each per-class handler.
pub struct SemCtx<'a> {
    /// Old architectural register file (read side).
    pub regs: &'a HexagonRegisters,
    /// In-flight GPR writes for this packet.
    pub new_r: &'a mut [Option<u32>; 32],
    /// In-flight predicate writes for this packet.
    pub new_p: &'a mut [Option<u8>; 4],
    /// Pending constant extender for this instruction's immediate, if any.
    pub immext: Option<u32>,
    /// Bits to OR into USR on commit (sticky saturation flag).
    pub usr_or: u32,
    /// In-flight HVX vector writes from earlier in this packet (for `.new`
    /// vector reads). Read-only; the driver owns the buffer.
    pub vnew: &'a [Option<[u32; 32]>; 32],
    /// `.tmp` HVX vector loads from earlier in this packet (scratch, never
    /// committed). Consulted by `vread_new` after `vnew`. Read-only.
    pub vtmp: &'a [Option<[u32; 32]>; 32],
    /// In-flight HVX vector-predicate writes.
    pub qnew: &'a [Option<[u32; 4]>; 4],
    /// HVX vector writes produced by this instruction (applied after dispatch).
    pub v_writes: Vec<(u8, [u32; 32])>,
    /// HVX vector-predicate writes produced by this instruction.
    pub q_writes: Vec<(u8, [u32; 4])>,
}

impl SemCtx<'_> {
    /// Read a 32-bit GPR.
    #[inline]
    pub fn r(&self, reg: u8) -> u32 {
        self.regs.r[reg as usize]
    }

    /// Read a 64-bit register pair (even-aligned).
    #[inline]
    pub fn rp(&self, reg: u8) -> u64 {
        let e = (reg & !1) as usize;
        (self.regs.r[e] as u64) | ((self.regs.r[e + 1] as u64) << 32)
    }

    /// Read an 8-bit predicate register (old architectural value).
    #[inline]
    pub fn p(&self, pred: u8) -> u8 {
        self.regs.p[pred as usize]
    }

    /// Read a predicate's `.new` value: the value produced earlier in this
    /// packet if any, else the old architectural value. Used by `.new`
    /// predicated forms (`if (Pu.new) ...`).
    #[inline]
    pub fn p_new(&self, pred: u8) -> u8 {
        self.new_p[pred as usize].unwrap_or(self.regs.p[pred as usize])
    }

    /// Write a 32-bit GPR.
    #[inline]
    pub fn set_r(&mut self, reg: u8, value: u32) {
        self.new_r[reg as usize] = Some(value);
    }

    /// Write a 64-bit register pair (even-aligned).
    #[inline]
    pub fn set_rp(&mut self, reg: u8, value: u64) {
        let e = (reg & !1) as usize;
        self.new_r[e] = Some(value as u32);
        self.new_r[e + 1] = Some((value >> 32) as u32);
    }

    /// Write an 8-bit predicate register.
    #[inline]
    pub fn set_p(&mut self, pred: u8, value: u8) {
        self.new_p[pred as usize] = Some(value);
    }

    /// Raise the sticky saturation/overflow flag.
    #[inline]
    pub fn set_ovf(&mut self) {
        self.usr_or |= USR_OVF;
    }

    /// Read an HVX vector register V0..V31 (old architectural value), as 32 LE
    /// u32 words (128 bytes).
    #[inline]
    pub fn vread(&self, reg: u8) -> [u32; 32] {
        self.regs.v[reg as usize]
    }

    /// Read a vector's `.new` value: an in-flight write produced earlier in the
    /// packet if any, else a `.tmp` load forwarded earlier in the packet, else
    /// the old architectural value.
    #[inline]
    pub fn vread_new(&self, reg: u8) -> [u32; 32] {
        self.vnew[reg as usize]
            .or(self.vtmp[reg as usize])
            .unwrap_or(self.regs.v[reg as usize])
    }

    /// Write an HVX vector register.
    #[inline]
    pub fn set_v(&mut self, reg: u8, value: [u32; 32]) {
        self.v_writes.push((reg, value));
    }

    /// Read a vector-predicate register Q0..Q3 (old), as 4 LE u32 (128 bits).
    #[inline]
    pub fn qread(&self, reg: u8) -> [u32; 4] {
        self.regs.q[reg as usize]
    }

    /// Read a vector-predicate's `.new` value (in-flight if produced earlier in
    /// the packet, else the old architectural value).
    #[inline]
    pub fn qread_new(&self, reg: u8) -> [u32; 4] {
        self.qnew[reg as usize].unwrap_or(self.regs.q[reg as usize])
    }

    /// Write a vector-predicate register.
    #[inline]
    pub fn set_q(&mut self, reg: u8, value: [u32; 4]) {
        self.q_writes.push((reg, value));
    }

    /// Saturate a value to a signed `n`-bit range, flagging overflow (`fSATN`).
    #[inline]
    pub fn sat_n(&mut self, val: i64, n: u32) -> i64 {
        let lo = -(1i64 << (n - 1));
        let hi = (1i64 << (n - 1)) - 1;
        if val < lo {
            self.set_ovf();
            lo
        } else if val > hi {
            self.set_ovf();
            hi
        } else {
            val
        }
    }

    /// Saturate a value to an unsigned `n`-bit range, flagging overflow (`fSATUN`).
    #[inline]
    pub fn satu_n(&mut self, val: i64, n: u32) -> i64 {
        let hi = (1i64 << n) - 1;
        if val < 0 {
            self.set_ovf();
            0
        } else if val > hi {
            self.set_ovf();
            hi
        } else {
            val
        }
    }
}

// --- field-extraction helpers shared by the class handlers -----------------

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32u8.saturating_sub(bits);
    ((value << shift) as i32) >> shift
}

fn apply_immext(imm: u32, ext: u32) -> u32 {
    ((ext & 0x03ff_ffff) << 6) | (imm & 0x3f)
}

/// Read an operand register/predicate field (the raw small field value).
#[inline]
pub(crate) fn fld(d: &DecodedOp, letter: u8) -> u8 {
    d.field(letter).map(|f| f.value as u8).unwrap_or(0)
}

/// Read a signed immediate field, applying a constant extender if present.
#[inline]
pub(crate) fn fimm_s(d: &DecodedOp, letter: u8, immext: Option<u32>) -> i32 {
    match d.field(letter) {
        Some(f) => match immext {
            Some(ext) => apply_immext(f.value, ext) as i32,
            None => sign_extend(f.value, f.bits),
        },
        None => 0,
    }
}

/// Read an unsigned immediate field, applying a constant extender if present.
#[inline]
pub(crate) fn fimm_u(d: &DecodedOp, letter: u8, immext: Option<u32>) -> u32 {
    match d.field(letter) {
        Some(f) => match immext {
            Some(ext) => apply_immext(f.value, ext),
            None => f.value,
        },
        None => 0,
    }
}

/// Dispatch a decoded opcode to its class handler.
/// Returns `true` if a handler executed the instruction.
pub fn dispatch(d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    // Each class returns `false` for opcodes it does not own; try them in turn.
    let op = d.opcode;
    alu::exec(op, d, ctx)
        || alu_pred::exec(op, d, ctx)
        || bitmanip::exec(op, d, ctx)
        || compare::exec(op, d, ctx)
        || float::exec(op, d, ctx)
        || mpy::exec(op, d, ctx)
        || shift::exec(op, d, ctx)
        || vecalu::exec(op, d, ctx)
        || extra::exec(op, d, ctx)
        || extra2::exec(op, d, ctx)
        || hvx::exec(op, d, ctx)
        || hvx_mpy::exec(op, d, ctx)
        || hvx_perm::exec(op, d, ctx)
        || hvx_cmp::exec(op, d, ctx)
        || hvx_shift::exec(op, d, ctx)
        || hvx_minmax::exec(op, d, ctx)
        || hvx_mpyv::exec(op, d, ctx)
        || hvx_mpys::exec(op, d, ctx)
        || hvx_rmpy::exec(op, d, ctx)
        || hvx_cmpy::exec(op, d, ctx)
        || hvx_lut::exec(op, d, ctx)
        || hvx_addsub::exec(op, d, ctx)
        || hvx_round::exec(op, d, ctx)
        || hvx_permx::exec(op, d, ctx)
        || hvx_predop::exec(op, d, ctx)
        || hvx_cmpacc::exec(op, d, ctx)
        || hvx_misc::exec(op, d, ctx)
        || hvx_carry::exec(op, d, ctx)
        || hvx_hist::exec(op, d, ctx)
        || hvx_v6mpy::exec(op, d, ctx)
        || mpy_ext::exec(op, d, ctx)
        || shift_ext::exec(op, d, ctx)
        || alu_ext::exec(op, d, ctx)
        || float_ext::exec(op, d, ctx)
}
