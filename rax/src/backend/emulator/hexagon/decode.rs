use crate::config::HexagonIsa;

use super::opcode::{self, DecodedOp, EncClass, FieldVal, Opcode};

#[derive(Clone, Copy, Debug)]
pub struct PredCond {
    pub pred: u8,
    pub sense: bool,
    pub pred_new: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemWidth {
    Byte,
    Half,
    Word,
    Double,
}

#[derive(Clone, Copy, Debug)]
pub enum MemSign {
    Signed,
    Unsigned,
}

/// Splice byte count `N` for `Vsplice` (`vspliceb`): an immediate `#u3` or the
/// low 3 bits of a predicate `Pu`.
#[derive(Clone, Copy, Debug)]
pub enum SpliceAmount {
    Imm(u8),
    Pred(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum AddrMode {
    Offset {
        base: u8,
        offset: i32,
    },
    PostIncImm {
        base: u8,
        offset: i32,
    },
    GpOffset {
        offset: i32,
    },
    Abs {
        addr: u32,
    },
    /// `memX(Rx++Mu)` — post-increment the base by the M register (M0 or M1
    /// selected by `modsel`; the raw M value is added, not the I-field).
    PostIncReg {
        base: u8,
        modsel: u8,
    },
    /// `memX(Rx++Mu:brev)` — the effective address is the bit-reversed base;
    /// the base is then post-incremented by the M register.
    PostIncBrev {
        base: u8,
        modsel: u8,
    },
    /// `memX(Rx++#s4:N:circ(Mu))` — circular post-increment by an immediate.
    /// `incr` is the (already scaled) byte increment; the buffer length/K come
    /// from M[modsel] and the base wraps within [CS[modsel], CS+length).
    PostIncCircImm {
        base: u8,
        modsel: u8,
        incr: i32,
    },
    /// `memX(Rx++I:circ(Mu))` — circular post-increment by register. The
    /// increment is the I field of M[modsel] (`fREAD_IREG`), shifted by the
    /// access size, applied with the same circular wrap.
    PostIncCircReg {
        base: u8,
        modsel: u8,
        shift: u8,
    },
    /// `memX(Rs+Ru<<#u2)` — register-offset addressing (`S4_*_rr`). The
    /// effective address is `Rs + (Ru << shift)`.
    RegScaled {
        base: u8,
        index: u8,
        shift: u8,
    },
    /// `memX(Re=##U6)` — absolute-set addressing (`L4_*_ap`). The effective
    /// address is the constant-extended `addr`; in addition the address
    /// register `areg` (Re) is written with that same absolute address.
    AbsSet {
        areg: u8,
        addr: u32,
    },
    /// `memX(Ru<<#u2+##U6)` — scaled-index absolute addressing (`L4_*_ur`).
    /// The effective address is `addr + (Ru << shift)` with `addr` the
    /// constant-extended literal.
    IndexAbs {
        index: u8,
        shift: u8,
        addr: u32,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum ShiftKind {
    Lsl,
    Lsr,
    Asr,
}

#[derive(Clone, Copy, Debug)]
pub enum CmpKind {
    Eq,
    Gt,
    Gtu,
    Ne,
    Lte,
    Lteu,
    /// Signed `>=` (used by the `jumprgtez` compare-to-zero jump).
    Gte,
}

/// Compare performed by a J4 compound compare-and-jump
/// (`J4_<cmp>_<cond>_jump[nv]_<hint>`).
#[derive(Clone, Copy, Debug)]
pub enum CmpJumpKind {
    /// `cmp.eq(Rs, Rt)` / for jumpnv `cmp.eq(Ns.new, Rt)`.
    Eq,
    /// `cmp.gt(Rs, Rt)` (signed).
    Gt,
    /// `cmp.gtu(Rs, Rt)` (unsigned).
    Gtu,
    /// `cmp.eq(Rs, #u5)` (the `_cmpeqi_` forms).
    EqImm(i32),
    /// `cmp.gt(Rs, #u5)` (signed, `_cmpgti_` forms).
    GtImm(i32),
    /// `cmp.gtu(Rs, #u5)` (unsigned, `_cmpgtui_` forms).
    GtuImm(u32),
    /// `cmp.eq(Rs, #-1)` (the `_cmpeqn1_` forms).
    EqN1,
    /// `cmp.gt(Rs, #-1)` (signed, `_cmpgtn1_` forms).
    GtN1,
    /// `cmp.lt(Rs, Rt)` == `cmp.gt(Rt, Rs)` (signed; only exists as jumpnv).
    Lt,
    /// `cmp.ltu(Rs, Rt)` == `cmp.gtu(Rt, Rs)` (unsigned; only exists as jumpnv).
    Ltu,
    /// `tstbit(Rs, #0)` — test bit 0 of Rs.
    TstBit0,
}

#[derive(Clone, Copy, Debug)]
pub enum ExtendKind {
    Sxt8,
    Sxt16,
    Zxt8,
    Zxt16,
}

#[derive(Clone, Copy, Debug)]
pub enum CombineOperand {
    Reg(u8),
    Imm(u32),
}

/// Source operand of a `J4_jumpset{i,r}` (`Rd = <#u6 | Rs> ; jump #r`).
#[derive(Clone, Copy, Debug)]
pub enum JumpSetSrc {
    Reg(u8),
    Imm(u32),
}

/// Read-modify-write memory operation (`memX(Rs+#u) OP= ...`).
#[derive(Clone, Copy, Debug)]
pub enum MemOpKind {
    Add,
    Sub,
    And,
    Or,
    /// `mem &= ~(1 << src)` — clear bit `src`.
    ClrBit,
    /// `mem |= (1 << src)` — set bit `src`.
    SetBit,
}

#[derive(Clone, Copy, Debug)]
pub enum MemOpSrc {
    Reg(u8),
    Imm(u32),
}

#[derive(Clone, Copy, Debug)]
pub enum DecodedInsn {
    ImmExt {
        value: u32,
    },
    Add {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    Sub {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    And {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    AndImm {
        dst: u8,
        src: u8,
        imm: u32,
    },
    OrImm {
        dst: u8,
        src: u8,
        imm: u32,
    },
    Or {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    Xor {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    AddImm {
        dst: u8,
        src: u8,
        imm: i32,
    },
    SubImmRev {
        dst: u8,
        src: u8,
        imm: i32,
    },
    Mov {
        dst: u8,
        src: u8,
    },
    MovImm {
        dst: u8,
        imm: i32,
    },
    Abs {
        dst: u8,
        src: u8,
        sat: bool,
    },
    NegSat {
        dst: u8,
        src: u8,
    },
    Max {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    Maxu {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    Min {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    Minu {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    ClearCond {
        dst: u8,
        pred: PredCond,
    },
    Extend {
        dst: u8,
        src: u8,
        kind: ExtendKind,
    },
    Combine {
        dst: u8,
        high: CombineOperand,
        low: CombineOperand,
    },
    Load {
        dst: u8,
        addr: AddrMode,
        width: MemWidth,
        sign: MemSign,
        pred: Option<PredCond>,
    },
    /// Shift-and-insert FIFO load (`Ryy = memX_fifo(...)`, `L2_loadalign{b,h}` /
    /// `L4_loadalign{b,h}_{ap,ur}`). `dst_pair` is the EVEN register of the
    /// read-modify destination pair Ryy. The freshly loaded byte/halfword is
    /// shifted into the TOP of Ryy while the rest shifts right:
    /// `Ryy = (Ryy u>> w) | (zxt(load) << (64-w))`, where `w` is 8 (Byte) or
    /// 16 (Half). Reads the OLD Ryy value (or the in-packet new value).
    LoadAlign {
        dst_pair: u8,
        addr: AddrMode,
        width: MemWidth,
        pred: Option<PredCond>,
    },
    /// Byte-unpack load (`Rd = memubh/membh(...)` / `Rdd = memubh/membh(...)`,
    /// `L2_loadbsw{2,4}` / `L2_loadbzw{2,4}` + `L4_*_{ap,ur}`). Loads `count`
    /// contiguous bytes (2 or 4) and unpacks each byte into a halfword lane,
    /// sign-extended (`bsw`, membh) or zero-extended (`bzw`, memubh). `count`==2
    /// writes the single 32-bit register `dst`; `count`==4 writes the register
    /// PAIR whose even half is `dst`.
    LoadUnpack {
        dst: u8,
        addr: AddrMode,
        count: u8,
        sign: MemSign,
        pred: Option<PredCond>,
    },
    Store {
        src: u8,
        addr: AddrMode,
        width: MemWidth,
        pred: Option<PredCond>,
        src_new: bool,
        /// `storerf` high-half store: the stored halfword is `Rt[31:16]`
        /// (always `MemWidth::Half`). `false` for all other stores.
        high_half: bool,
    },
    StoreImm {
        value: u32,
        addr: AddrMode,
        width: MemWidth,
        pred: Option<PredCond>,
    },
    AllocFrame {
        base: u8,
        size: u32,
    },
    DeallocFrame {
        base: u8,
        dst: Option<u8>,
        update_lr_fp: bool,
    },
    DeallocReturn {
        base: u8,
        dst: Option<u8>,
        pred: Option<PredCond>,
        update_lr_fp: bool,
    },
    Jump {
        offset: i32,
    },
    JumpCond {
        offset: i32,
        pred: u8,
        sense: bool,
        pred_new: bool,
    },
    JumpReg {
        src: u8,
    },
    JumpRegCond {
        src: u8,
        pred: u8,
        sense: bool,
        pred_new: bool,
    },
    /// `if (cmp.<kind>(Rs,#0)) jump #r13:2` — the J2 jumpr-compare-zero family
    /// (`jumprz`/`jumprnz`/`jumprgtez`/`jumprltez` and their `pt` hints). Despite
    /// the `jumpr` mnemonic these are DIRECT PC-relative jumps (target =
    /// `PC + offset`); only the *condition* is a register compare against zero.
    JumpRegZero {
        src: u8,
        kind: CmpKind,
        offset: i32,
    },
    /// `Rd = <value> ; jump #r` — the J4 jumpset{i,r} compound. Writes `Rd`
    /// (always, unconditionally) and then takes the PC-relative branch.
    JumpSet {
        dst: u8,
        value: JumpSetSrc,
        offset: i32,
    },
    /// Architectural NOP (e.g. `J2_pause`): advances PC only.
    Nop,
    Call {
        offset: i32,
        /// `Some` for the predicated `J2_callt`/`J2_callf` forms: the call (and
        /// its `r31` link) only happens when the predicate condition holds.
        pred: Option<PredCond>,
    },
    CallReg {
        src: u8,
        /// `Some` for the predicated `J2_callrt`/`J2_callrf` forms.
        pred: Option<PredCond>,
    },
    Cmp {
        pred: u8,
        src1: u8,
        src2: u8,
        kind: CmpKind,
    },
    CmpImm {
        pred: u8,
        src: u8,
        imm: i32,
        kind: CmpKind,
        unsigned: bool,
    },
    Mul {
        dst: u8,
        src1: u8,
        src2: u8,
    },
    ShiftImm {
        dst: u8,
        src: u8,
        kind: ShiftKind,
        amount: u8,
    },
    ShiftReg {
        dst: u8,
        src: u8,
        amt: u8,
        kind: ShiftKind,
    },
    TfrCrR {
        dst: u8,
        src: u8,
    },
    TfrRrCr {
        dst: u8,
        src: u8,
    },
    /// `Rdd = Css`: read a control-register PAIR into a GPR pair (A4_tfrcpp).
    TfrCrRPair {
        dst: u8,
        src: u8,
    },
    /// `Cdd = Rss`: write a GPR pair into a control-register PAIR (A4_tfrpcp).
    TfrRrCrPair {
        dst: u8,
        src: u8,
    },
    /// `dczeroa(Rs)`: zero the 32-byte cache line at `Rs & !31` (Y2_dczeroa).
    DcZero {
        base: u8,
    },
    LoopStartReg {
        loop_id: u8,
        start_offset: i32,
        count_reg: u8,
        /// `Some(n)` for the `spNloop0` (`J2_ploopNsr`) software-pipelined
        /// variants: also sets USR.LPCFG = n and presets P3 = 0. `None` for the
        /// plain `loop0`/`loop1`.
        lpcfg: Option<u8>,
    },
    LoopStartImm {
        loop_id: u8,
        start_offset: i32,
        count: u32,
        /// See `LoopStartReg::lpcfg`.
        lpcfg: Option<u8>,
    },
    Trap0,
    /// HVX vector load: `Vd = vmem(base + offset)`; `post_inc` updates `base` by
    /// that many bytes; `aligned` masks the effective address to a 128-byte
    /// boundary (false for the `vmemu` unaligned form).
    VLoad {
        dst: u8,
        base: u8,
        offset: i32,
        post_inc: Option<i32>,
        /// `ppu` post-increment by the modifier register Mu (`Some(modsel)`,
        /// 0 => M0/C6, 1 => M1/C7). Mutually exclusive with `post_inc` (the
        /// `pi` immediate form). The increment is the full Mu byte value.
        post_inc_mod: Option<u8>,
        aligned: bool,
        /// `false` for a `.tmp` load: the value is only forwardable within the
        /// packet and is NOT committed to the architectural vector register.
        commit: bool,
        /// Scalar predicate for the `if (Pv[!]) Vd = vmem(...)` forms. On a false
        /// predicate the load is cancelled: no register write, no post-increment.
        pred: Option<PredCond>,
    },
    /// HVX vector store: `vmem(base + offset) = Vs`, optionally scalar-predicated
    /// (`if (Pv[!]) vmem(...) = Vs`).
    VStore {
        src: u8,
        base: u8,
        offset: i32,
        post_inc: Option<i32>,
        /// `ppu` post-increment by the modifier register Mu (`Some(modsel)`).
        post_inc_mod: Option<u8>,
        aligned: bool,
        pred: Option<PredCond>,
        /// Byte-mask store: `(Qv register, sense)`. When set, byte `i` is stored
        /// iff `Q.bit[i] == sense` (`qpred` => true, `nqpred` => false).
        qmask: Option<(u8, bool)>,
        /// New-value vector store (`vmem(...) = V.new`). The store data is the
        /// vector produced earlier in this packet. When a same-packet `vgather`
        /// produced no architectural vector (the `vmem=vtmp.new` idiom), the data
        /// is the per-packet gather scratch (`gather_tmp`) instead.
        from_gather: bool,
        /// `srls` (`vmem(...):scatter_release`): a scatter-release barrier with no
        /// vector source. Architecturally a no-op apart from the post-increment;
        /// no memory is written.
        srls: bool,
    },
    /// HVX V65 scatter: for each element, store (or accumulate) the data element
    /// to `Rt + offset[i]` when the (aligned) effective address lies within the
    /// region `[Rt, Rt+Mu]`. See `cpu.rs` for the element/range semantics.
    VScatter {
        /// Region base register `Rt`.
        base: u8,
        /// Modifier select for `Mu` (0 => M0/C6, 1 => M1/C7); holds `length-1`.
        modsel: u8,
        /// Offset vector register `Vv` (for `hw` forms, the even base of the
        /// `Vvv` register pair).
        offsets: u8,
        /// Data vector register `Vw`.
        data: u8,
        /// Element size in bytes (2 = halfword, 4 = word).
        esz: u8,
        /// `true` for the half-word-offset (`hw`) double-resource forms: a 64
        /// word-sized offset pair feeding 64 halfword data elements.
        off_pair: bool,
        /// `true` for the accumulate (`_add`) forms (`*EA += data`).
        add: bool,
        /// Vector predicate `Qs` for the `q` (predicated) forms.
        pred: Option<u8>,
    },
    /// HVX V65 gather: read in-range (and, for `q` forms, predicate-on) elements
    /// from `Rt + offset[i]` into the per-packet gather scratch. A paired
    /// `vmem(Rs+#k)=vtmp.new` store commits only the gathered bytes (the rest of
    /// the destination is preserved). See `cpu.rs`.
    VGather {
        base: u8,
        modsel: u8,
        offsets: u8,
        esz: u8,
        off_pair: bool,
        pred: Option<u8>,
    },
    /// New-value store: stores the register produced earlier in this packet,
    /// selected by the `Nt8` field. The packet driver resolves `nt` to the
    /// producer register (using `Nt8 >> 1` as the back-distance among the
    /// packet's GPR producers) before execution.
    StoreNew {
        nt: u8,
        addr: AddrMode,
        width: MemWidth,
        pred: Option<PredCond>,
    },
    /// Load-locked (`Rd=memw_locked(Rs)` / `Rdd=memd_locked(Rs)`): a plain word /
    /// doubleword load at `Rs+0` that ALSO sets the LL reservation on that
    /// address. A later matching store-conditional succeeds; a store-conditional
    /// with no (or a stale) reservation fails.
    LoadLocked {
        dst: u8,
        base: u8,
        width: MemWidth,
    },
    /// Store-conditional / store-release at `Rs+0` (`memw_locked(Rs,Pd)=Rt`,
    /// `memd_locked(Rs,Pd)=Rtt`, and the `memX_rl(Rs):at/:st` release stores).
    /// `base` is Rs, `src` is the data register (the EVEN reg of the pair for
    /// `MemWidth::Double`). `success_pred` is `Some(Pd)` for the `_locked`
    /// store-conditional forms: the store happens and `Pd` is set TRUE (0xff)
    /// ONLY if the LL reservation set by a preceding load-locked matches `Rs`;
    /// otherwise the store is skipped and `Pd` is cleared. `None` for the plain
    /// release stores (always store; no reservation, no predicate side effect).
    StoreCond {
        src: u8,
        base: u8,
        width: MemWidth,
        success_pred: Option<u8>,
    },
    /// Vector byte splice (`Rdd=vspliceb(Rss,Rtt,#u3|Pu)`). The low `N` bytes
    /// come from `src_low` (Rss) and the high `8-N` bytes from `src_high` (Rtt);
    /// `amount` gives `N` (an immediate `#u3` or `Pu & 7`). A register-pair write
    /// to `dst` (the even reg of Rdd); touches no memory.
    Vsplice {
        dst: u8,
        src_low: u8,
        src_high: u8,
        amount: SpliceAmount,
    },
    MemOp {
        base: u8,
        offset: i32,
        width: MemWidth,
        op: MemOpKind,
        src: MemOpSrc,
    },
    /// J4 compound compare-and-jump (`J4_<cmp>_<cond>_jump[nv]_<hint>`): a single
    /// instruction that evaluates a compare and branches on the result.
    ///
    /// * `kind` is the comparison (reg/reg, reg/imm, or vs the constants -1 / bit
    ///   0). `src1` is `Rs` (or, when `new_value` is set, the `Ns8` producer
    ///   index resolved against the packet's GPR producers exactly like a
    ///   new-value store). `src2` is `Rt` for the register/`Lt`/`Ltu` forms.
    /// * `write_pred` is `Some(p)` for the predicate-writing `_jump` forms
    ///   (`tp0`/`fp0` write P0, `tp1`/`fp1` write P1); it is `None` for the
    ///   `_jumpnv` (new-value compare-and-branch) forms which write no predicate.
    /// * `sense` is `true` for the `t*` (jump-if-true) forms and `false` for the
    ///   `f*` (jump-if-false) forms.
    /// * `offset` is the (already `<<2`-scaled) PC-relative branch displacement.
    CompoundCmpJump {
        kind: CmpJumpKind,
        src1: u8,
        src2: u8,
        write_pred: Option<u8>,
        sense: bool,
        new_value: bool,
        offset: i32,
    },
    Unknown(u32),
}

pub struct DecodedWord {
    pub insn: DecodedInsn,
    pub used_ext: bool,
    pub opcode: Option<Opcode>,
}

pub struct DecodedSub {
    pub insn: DecodedInsn,
    pub opcode: Option<Opcode>,
}

fn sign_extend(value: u32, bits: u8) -> i32 {
    let shift = 32u8.saturating_sub(bits);
    ((value << shift) as i32) >> shift
}

fn isa_version(isa: HexagonIsa) -> u16 {
    match isa {
        HexagonIsa::V4 => 4,
        HexagonIsa::V5 => 5,
        HexagonIsa::V55 => 55,
        HexagonIsa::V60 => 60,
        HexagonIsa::V62 => 62,
        HexagonIsa::V65 => 65,
        HexagonIsa::V66 => 66,
        HexagonIsa::V67 => 67,
        HexagonIsa::V68 => 68,
        HexagonIsa::V69 => 69,
    }
}

fn isa_at_least(isa: HexagonIsa, min: HexagonIsa) -> bool {
    isa_version(isa) >= isa_version(min)
}

fn isa_supports_duplex(isa: HexagonIsa) -> bool {
    isa_at_least(isa, HexagonIsa::V4)
}

fn isa_supports_dotnew(isa: HexagonIsa) -> bool {
    isa_at_least(isa, HexagonIsa::V4)
}

fn pred_uses_dotnew(pred: Option<PredCond>) -> bool {
    pred.map_or(false, |cond| cond.pred_new)
}

fn insn_uses_dotnew(insn: &DecodedInsn) -> bool {
    match insn {
        DecodedInsn::Load { pred, .. } => pred_uses_dotnew(*pred),
        DecodedInsn::LoadAlign { pred, .. } => pred_uses_dotnew(*pred),
        DecodedInsn::LoadUnpack { pred, .. } => pred_uses_dotnew(*pred),
        DecodedInsn::Store { pred, src_new, .. } => pred_uses_dotnew(*pred) || *src_new,
        DecodedInsn::StoreImm { pred, .. } => pred_uses_dotnew(*pred),
        DecodedInsn::JumpCond { pred_new, .. } => *pred_new,
        DecodedInsn::JumpRegCond { pred_new, .. } => *pred_new,
        DecodedInsn::DeallocReturn { pred, .. } => pred_uses_dotnew(*pred),
        DecodedInsn::ClearCond { pred, .. } => pred.pred_new,
        // The `_jumpnv` compound forms compare a register produced earlier in the
        // same packet (a new-value operand); the predicate-writing `_jump` forms
        // produce and consume their `.new` predicate within the one instruction.
        DecodedInsn::CompoundCmpJump { .. } => true,
        _ => false,
    }
}

pub(crate) fn isa_supports_insn(
    isa: HexagonIsa,
    insn: &DecodedInsn,
    opcode: Option<Opcode>,
) -> bool {
    if insn_uses_dotnew(insn) && !isa_supports_dotnew(isa) {
        return false;
    }
    if let Some(opcode) = opcode {
        return isa_version(isa) >= opcode::opcode_min_version(opcode);
    }
    true
}

fn apply_immext(imm: u32, immext: u32) -> u32 {
    let ext = immext & 0x03ff_ffff;
    (ext << 6) | (imm & 0x3f)
}

fn decode_simm_val(imm: u32, bits: u8, immext: Option<u32>) -> (i32, bool) {
    if let Some(ext) = immext {
        return (apply_immext(imm, ext) as i32, true);
    }
    (sign_extend(imm, bits), false)
}

fn decode_uimm_val(imm: u32, immext: Option<u32>) -> (u32, bool) {
    if let Some(ext) = immext {
        return (apply_immext(imm, ext), true);
    }
    (imm, false)
}

fn field_val(decoded: &DecodedOp, letter: u8) -> Option<FieldVal> {
    decoded.field(letter)
}

fn field_u8(decoded: &DecodedOp, letter: u8) -> Option<u8> {
    decoded.field(letter).map(|val| val.value as u8)
}

fn decode_field_simm(decoded: &DecodedOp, letter: u8, immext: Option<u32>) -> Option<(i32, bool)> {
    let field = field_val(decoded, letter)?;
    Some(decode_simm_val(field.value, field.bits, immext))
}

fn decode_field_scaled_simm(
    decoded: &DecodedOp,
    letter: u8,
    immext: Option<u32>,
    shift: u8,
) -> Option<(i32, bool)> {
    let field = field_val(decoded, letter)?;
    if let Some(ext) = immext {
        return Some((apply_immext(field.value, ext) as i32, true));
    }
    Some((sign_extend(field.value, field.bits) << shift, false))
}

fn decode_field_uimm(decoded: &DecodedOp, letter: u8, immext: Option<u32>) -> Option<(u32, bool)> {
    let field = field_val(decoded, letter)?;
    Some(decode_uimm_val(field.value, immext))
}

fn width_shift(width: MemWidth) -> u8 {
    match width {
        MemWidth::Byte => 0,
        MemWidth::Half => 1,
        MemWidth::Word => 2,
        MemWidth::Double => 3,
    }
}

fn subreg(code: u8) -> u8 {
    if code < 8 { code } else { code + 8 }
}

fn subreg_pair(code: u8) -> u8 {
    let code = code & 0x7;
    if code < 4 {
        code * 2
    } else {
        16 + (code - 4) * 2
    }
}

fn pred_cond(pred: u8, sense: bool, pred_new: bool) -> PredCond {
    PredCond {
        pred,
        sense,
        pred_new,
    }
}

/// Decode the predicate condition `(sense, pred_new)` from a predicated-store
/// opcode by parsing the trailing condition segment of its mnemonic. The
/// mnemonic ends in `<cond>_<mode>` where `cond` is one of `t`/`f`/`tnew`/`fnew`
/// (`t`/`f` = test `Pv`, `tnew`/`fnew` = test `Pv.new`). For new-value stores
/// (`...new<cond>_<mode>`) the `new` segment precedes the condition, so we
/// only inspect the condition immediately before the final `_<mode>`.
fn pstore_cond(opcode: Opcode) -> (bool, bool) {
    let name = opcode::opcode_name(opcode);
    // Strip the addressing-mode suffix (`_io` / `_pi` / `_rr` / `_abs`).
    let body = name.rsplit_once('_').map(|(head, _)| head).unwrap_or(name);
    // `tnew`/`fnew` -> dot-new predicate; `t`/`f` -> plain predicate. The
    // data-side `new` of a new-value store (e.g. `pstorerbnewt`) is always
    // followed by the `t`/`f`/`tnew`/`fnew` condition, so suffix tests are safe.
    if body.ends_with("tnew") {
        (true, true)
    } else if body.ends_with("fnew") {
        (false, true)
    } else if body.ends_with('t') {
        (true, false)
    } else {
        (false, false)
    }
}

/// Decode the predicate condition `(sense, pred_new)` from a predicated-LOAD
/// opcode mnemonic. The mnemonic is `...<cond>_<mode>` where `cond` is one of
/// `t`/`f`/`tnew`/`fnew` (`t`/`f` = test `Pv`, `tnew`/`fnew` = test `Pv.new`).
fn pload_cond(opcode: Opcode) -> (bool, bool) {
    let name = opcode::opcode_name(opcode);
    let body = name.rsplit_once('_').map(|(head, _)| head).unwrap_or(name);
    if body.ends_with("tnew") {
        (true, true)
    } else if body.ends_with("fnew") {
        (false, true)
    } else if body.ends_with('t') {
        (true, false)
    } else {
        (false, false)
    }
}

fn load_io(
    decoded: &DecodedOp,
    width: MemWidth,
    sign: MemSign,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let dst = field_u8(decoded, b'd')?;
    let (imm, used) = decode_field_simm(decoded, b'i', immext)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::Offset { base, offset },
            width,
            sign,
            pred: None,
        },
        used,
    ))
}

fn load_gp(
    decoded: &DecodedOp,
    width: MemWidth,
    sign: MemSign,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let dst = field_u8(decoded, b'd')?;
    let (imm, used) = decode_field_uimm(decoded, b'i', immext)?;
    // A constant extender turns the GP-relative form into an *absolute* access
    // (the extended immediate is the full byte address; GP is not added).
    let addr = if used {
        AddrMode::Abs { addr: imm }
    } else {
        AddrMode::GpOffset {
            offset: (imm << width_shift(width)) as i32,
        }
    };
    Some((
        DecodedInsn::Load {
            dst,
            addr,
            width,
            sign,
            pred: None,
        },
        used,
    ))
}

fn load_pi(decoded: &DecodedOp, width: MemWidth, sign: MemSign) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let dst = field_u8(decoded, b'd')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::PostIncImm { base, offset },
            width,
            sign,
            pred: None,
        },
        false,
    ))
}

fn store_io(
    decoded: &DecodedOp,
    width: MemWidth,
    immext: Option<u32>,
    src_new: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let src = field_u8(decoded, b't')?;
    let (imm, used) = decode_field_simm(decoded, b'i', immext)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::Offset { base, offset },
            width,
            pred: None,
            src_new,
            high_half: false,
        },
        used,
    ))
}

/// `storerf` high-half store with register+immediate offset (`memh(Rs+#s11:1)
/// =Rt.h`, `S2_storerf_io`). Always `MemWidth::Half`; stores `Rt[31:16]`.
fn store_io_high(decoded: &DecodedOp, immext: Option<u32>) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let src = field_u8(decoded, b't')?;
    let (imm, used) = decode_field_simm(decoded, b'i', immext)?;
    let offset = imm.wrapping_shl(width_shift(MemWidth::Half) as u32);
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::Offset { base, offset },
            width: MemWidth::Half,
            pred: None,
            src_new: false,
            high_half: true,
        },
        used,
    ))
}

/// New-value store with register+immediate offset (`memX(Rs+#s11:N)=Nt8.new`).
fn store_new_io(
    decoded: &DecodedOp,
    width: MemWidth,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let nt = field_u8(decoded, b't')?;
    let (imm, used) = decode_field_simm(decoded, b'i', immext)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr: AddrMode::Offset { base, offset },
            width,
            pred: None,
        },
        used,
    ))
}

/// Predicated new-value store (`if ([!]Pv[.new]) memX(Rs+#u6:N)=Nt8.new`). The
/// store data is the `Nt8` producer (field `t`), resolved against the packet's
/// producers, not a direct register read — hence `StoreNew` with a predicate.
/// `pred_new` selects whether the guarding predicate is `.new` (tnew/fnew).
fn pred_store_new_io(
    decoded: &DecodedOp,
    width: MemWidth,
    sense: bool,
    pred_new: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let nt = field_u8(decoded, b't')?;
    let pred = field_u8(decoded, b'v')?;
    let (imm, _) = decode_field_uimm(decoded, b'i', None)?;
    let offset = (imm << width_shift(width)) as i32;
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr: AddrMode::Offset { base, offset },
            width,
            pred: Some(pred_cond(pred, sense, pred_new)),
        },
        false,
    ))
}

fn store_gp(
    decoded: &DecodedOp,
    width: MemWidth,
    src_new: bool,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let src = field_u8(decoded, b't')?;
    let (imm, used) = decode_field_uimm(decoded, b'i', immext)?;
    let addr = if used {
        AddrMode::Abs { addr: imm }
    } else {
        AddrMode::GpOffset {
            offset: (imm << width_shift(width)) as i32,
        }
    };
    Some((
        DecodedInsn::Store {
            src,
            addr,
            width,
            pred: None,
            src_new,
            high_half: false,
        },
        used,
    ))
}

fn store_pi(decoded: &DecodedOp, width: MemWidth, src_new: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let src = field_u8(decoded, b't')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncImm { base, offset },
            width,
            pred: None,
            src_new,
            high_half: false,
        },
        false,
    ))
}

/// Base register-offset store (`memX(Rs+Ru<<#u2)=Rt[.h]`, `S4_storer*_rr`).
/// Fields: `s` = Rs base, `u` = Ru index, `i` = #u2 shift, `t` = data Rt. (Note
/// the load form `L4_loadr*_rr` instead carries the index in field `t`.)
fn store_rr(decoded: &DecodedOp, width: MemWidth, high_half: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let index = field_u8(decoded, b'u')?;
    let src = field_u8(decoded, b't')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::RegScaled { base, index, shift },
            width,
            pred: None,
            src_new: false,
            high_half,
        },
        false,
    ))
}

/// NEW-VALUE register-offset store (`S4_storer*new_rr`). The data source is the
/// `Nt8` producer (field `t`); index is `u`, base is `s`, shift is `i`.
fn store_new_rr(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let index = field_u8(decoded, b'u')?;
    let nt = field_u8(decoded, b't')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr: AddrMode::RegScaled { base, index, shift },
            width,
            pred: None,
        },
        false,
    ))
}

/// Absolute-set store (`memX(Re=##U6)=Rt[.h]`, `S4_storer*_ap`). The EA is the
/// constant-extended `I` field; the address register `Re` (field `e`) is also
/// written with that absolute. Data is `Rt` (field `t`). Requires an extender.
fn store_ap(
    decoded: &DecodedOp,
    width: MemWidth,
    high_half: bool,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let src = field_u8(decoded, b't')?;
    let areg = field_u8(decoded, b'e')?;
    let (addr, used) = decode_field_uimm(decoded, b'I', immext)?;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::AbsSet { areg, addr },
            width,
            pred: None,
            src_new: false,
            high_half,
        },
        used,
    ))
}

/// NEW-VALUE absolute-set store (`S4_storer*new_ap`). Data source is the `Nt8`
/// producer (field `t`); the address register `Re` is also written.
fn store_new_ap(
    decoded: &DecodedOp,
    width: MemWidth,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let nt = field_u8(decoded, b't')?;
    let areg = field_u8(decoded, b'e')?;
    let (addr, used) = decode_field_uimm(decoded, b'I', immext)?;
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr: AddrMode::AbsSet { areg, addr },
            width,
            pred: None,
        },
        used,
    ))
}

/// Scaled-index absolute store (`memX(Ru<<#u2+##U6)=Rt[.h]`, `S4_storer*_ur`).
/// The EA is `addr + (Ru << shift)` with `addr` the constant-extended `I` field.
/// Fields: `u` = Ru index, `i` = #u2 shift, `t` = data Rt. (The load form
/// `L4_loadr*_ur` instead carries the index in field `t`.)
fn store_ur(
    decoded: &DecodedOp,
    width: MemWidth,
    high_half: bool,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let src = field_u8(decoded, b't')?;
    let index = field_u8(decoded, b'u')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    let (addr, used) = decode_field_uimm(decoded, b'I', immext)?;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::IndexAbs { index, shift, addr },
            width,
            pred: None,
            src_new: false,
            high_half,
        },
        used,
    ))
}

/// NEW-VALUE scaled-index absolute store (`S4_storer*new_ur`). Data source is
/// the `Nt8` producer (field `t`); index is `u`, shift is `i`.
fn store_new_ur(
    decoded: &DecodedOp,
    width: MemWidth,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let nt = field_u8(decoded, b't')?;
    let index = field_u8(decoded, b'u')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    let (addr, used) = decode_field_uimm(decoded, b'I', immext)?;
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr: AddrMode::IndexAbs { index, shift, addr },
            width,
            pred: None,
        },
        used,
    ))
}

/// `storerf` post-increment-by-immediate store (`memh(Rx++#s4:1)=Rt.h`,
/// `S2_storerf_pi`). Like `store_pi` but stores the high halfword `Rt[31:16]`.
fn store_high_pi(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let src = field_u8(decoded, b't')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncImm { base, offset },
            width,
            pred: None,
            src_new: false,
            high_half: true,
        },
        false,
    ))
}

/// `storerf` register / bit-reverse post-increment store (`S2_storerf_pr` /
/// `_pbr`). `x` = base (post-incremented), `u` = Mu select, `t` = data Rt.
fn store_high_pr(decoded: &DecodedOp, width: MemWidth, brev: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    let src = field_u8(decoded, b't')?;
    let addr = if brev {
        AddrMode::PostIncBrev { base, modsel }
    } else {
        AddrMode::PostIncReg { base, modsel }
    };
    Some((
        DecodedInsn::Store {
            src,
            addr,
            width,
            pred: None,
            src_new: false,
            high_half: true,
        },
        false,
    ))
}

/// `storerf` circular post-increment store by immediate (`S2_storerf_pci`).
/// `x` = base, `u` = Mu select, `i` = signed #s4 increment (scaled), `t` = data.
fn store_high_pci(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    let src = field_u8(decoded, b't')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let incr = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncCircImm { base, modsel, incr },
            width,
            pred: None,
            src_new: false,
            high_half: true,
        },
        false,
    ))
}

/// `storerf` circular post-increment store by the M I-field (`S2_storerf_pcr`).
/// `x` = base, `u` = Mu select, `t` = data Rt.
fn store_high_pcr(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    let src = field_u8(decoded, b't')?;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncCircReg {
                base,
                modsel,
                shift: width_shift(width),
            },
            width,
            pred: None,
            src_new: false,
            high_half: true,
        },
        false,
    ))
}

/// NEW-VALUE GP-relative store (`memX(gp+#u16:N)=Nt8.new`, `S2_storer*newgp`).
/// Data is the `Nt8` producer (field `t`); offset is `i` (scaled by access
/// size, or the full byte address if a constant extender is present).
fn store_new_gp(
    decoded: &DecodedOp,
    width: MemWidth,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let nt = field_u8(decoded, b't')?;
    let (imm, used) = decode_field_uimm(decoded, b'i', immext)?;
    let addr = if used {
        AddrMode::Abs { addr: imm }
    } else {
        AddrMode::GpOffset {
            offset: (imm << width_shift(width)) as i32,
        }
    };
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr,
            width,
            pred: None,
        },
        used,
    ))
}

/// `storerfgp` GP-relative high-half store (`memh(gp+#u16:1)=Rt.h`). Like
/// `store_gp` but stores `Rt[31:16]`.
fn store_high_gp(
    decoded: &DecodedOp,
    width: MemWidth,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let src = field_u8(decoded, b't')?;
    let (imm, used) = decode_field_uimm(decoded, b'i', immext)?;
    let addr = if used {
        AddrMode::Abs { addr: imm }
    } else {
        AddrMode::GpOffset {
            offset: (imm << width_shift(width)) as i32,
        }
    };
    Some((
        DecodedInsn::Store {
            src,
            addr,
            width,
            pred: None,
            src_new: false,
            high_half: true,
        },
        used,
    ))
}

/// Store-conditional / store-release (`memw_locked(Rs,Pd)=Rt`,
/// `memd_locked(Rs,Pd)=Rtt`, and the `memX_rl(Rs):at/:st` release stores). The
/// EA is always `Rs+0` (field `s`); data is `Rt`/`Rtt` (field `t`). For the
/// `_locked` forms field `d` is the success predicate Pd, which in single-thread
/// user mode is always set TRUE (the reservation never fails). Release stores
/// have no success predicate.
fn store_cond(decoded: &DecodedOp, width: MemWidth, set_pred: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let src = field_u8(decoded, b't')?;
    let success_pred = if set_pred {
        Some(field_u8(decoded, b'd')?)
    } else {
        None
    };
    Some((
        DecodedInsn::StoreCond {
            src,
            base,
            width,
            success_pred,
        },
        false,
    ))
}

/// Vector byte splice (`Rdd=vspliceb(Rss,Rtt,#u3|Pu)`, `S2_vsplice{ib,rb}`). The
/// low `N` bytes come from Rss and the high `8-N` bytes from Rtt, where `N` is
/// the `#u3` immediate (`ib`) or `Pu & 7` (`rb`). A pure register-pair write.
fn vsplice(decoded: &DecodedOp, imm_n: bool) -> Option<(DecodedInsn, bool)> {
    let dst = field_u8(decoded, b'd')?;
    let src_low = field_u8(decoded, b's')?; // Rss
    let src_high = field_u8(decoded, b't')?; // Rtt
    let amount = if imm_n {
        SpliceAmount::Imm(decode_field_uimm(decoded, b'i', None)?.0 as u8)
    } else {
        SpliceAmount::Pred(field_u8(decoded, b'u')?)
    };
    Some((
        DecodedInsn::Vsplice {
            dst,
            src_low,
            src_high,
            amount,
        },
        false,
    ))
}

/// `memX(Rx++Mu)` post-increment by the M register (`L2_load*_pr`).
fn load_pr(decoded: &DecodedOp, width: MemWidth, sign: MemSign) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let dst = field_u8(decoded, b'd')?;
    let modsel = field_u8(decoded, b'u')?;
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::PostIncReg { base, modsel },
            width,
            sign,
            pred: None,
        },
        false,
    ))
}

/// `memX(Rx++Mu:brev)` bit-reverse post-increment (`L2_load*_pbr`).
fn load_pbr(decoded: &DecodedOp, width: MemWidth, sign: MemSign) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let dst = field_u8(decoded, b'd')?;
    let modsel = field_u8(decoded, b'u')?;
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::PostIncBrev { base, modsel },
            width,
            sign,
            pred: None,
        },
        false,
    ))
}

/// `memX(Rx++#s4:N:circ(Mu))` circular post-increment by immediate
/// (`L2_load*_pci`). The s4 field is scaled by the access size.
fn load_pci(decoded: &DecodedOp, width: MemWidth, sign: MemSign) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let dst = field_u8(decoded, b'd')?;
    let modsel = field_u8(decoded, b'u')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let incr = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::PostIncCircImm { base, modsel, incr },
            width,
            sign,
            pred: None,
        },
        false,
    ))
}

/// `memX(Rx++I:circ(Mu))` circular post-increment by the M register's I field
/// (`L2_load*_pcr`).
fn load_pcr(decoded: &DecodedOp, width: MemWidth, sign: MemSign) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let dst = field_u8(decoded, b'd')?;
    let modsel = field_u8(decoded, b'u')?;
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::PostIncCircReg {
                base,
                modsel,
                shift: width_shift(width),
            },
            width,
            sign,
            pred: None,
        },
        false,
    ))
}

// ====================================================================
// Shared addressing-mode decoders for the L2/L4 LOADALIGN / BSW / BZW and
// predicated-load families. Each returns the resolved `AddrMode` plus whether
// a constant extender was consumed. `scale_shift` is the immediate / I-field
// scale (the `:N` in `Rx++#s4:N`): 0 for byte FIFO, 1 for half FIFO and the
// 2-byte unpack, 2 for the 4-byte unpack.
// ====================================================================

/// `mem...(Rs+#s11:N)` — register + scaled signed immediate. Field `s` = base,
/// `i` = signed immediate (scaled by `scale_shift`).
fn addr_io_n(
    decoded: &DecodedOp,
    scale_shift: u8,
    immext: Option<u32>,
) -> Option<(AddrMode, bool)> {
    let base = field_u8(decoded, b's')?;
    let (imm, used) = decode_field_simm(decoded, b'i', immext)?;
    let offset = imm.wrapping_shl(scale_shift as u32);
    Some((AddrMode::Offset { base, offset }, used))
}

/// `mem...(Rx++#s4:N)` — post-increment by scaled signed immediate. Field `x`
/// = base (post-incremented), `i` = signed immediate (scaled by `scale_shift`).
fn addr_pi_n(decoded: &DecodedOp, scale_shift: u8) -> Option<AddrMode> {
    let base = field_u8(decoded, b'x')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let offset = imm.wrapping_shl(scale_shift as u32);
    Some(AddrMode::PostIncImm { base, offset })
}

/// `mem...(Rx++Mu)` — post-increment by the modifier register. Fields `x`, `u`.
fn addr_pr(decoded: &DecodedOp) -> Option<AddrMode> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    Some(AddrMode::PostIncReg { base, modsel })
}

/// `mem...(Rx++Mu:brev)` — bit-reverse post-increment. Fields `x`, `u`.
fn addr_pbr(decoded: &DecodedOp) -> Option<AddrMode> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    Some(AddrMode::PostIncBrev { base, modsel })
}

/// `mem...(Rx++#s4:N:circ(Mu))` — circular post-increment by scaled immediate.
fn addr_pci_n(decoded: &DecodedOp, scale_shift: u8) -> Option<AddrMode> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let incr = imm.wrapping_shl(scale_shift as u32);
    Some(AddrMode::PostIncCircImm { base, modsel, incr })
}

/// `mem...(Rx++I:circ(Mu))` — circular post-increment by the I field, scaled.
fn addr_pcr_n(decoded: &DecodedOp, scale_shift: u8) -> Option<AddrMode> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    Some(AddrMode::PostIncCircReg {
        base,
        modsel,
        shift: scale_shift,
    })
}

/// `mem...(Re=##U6)` — absolute-set (`L4_*_ap`). Field `e` = address register
/// Re (also written), `I` = the 6 low bits of the extended absolute address.
/// Requires a constant extender; without one the address is just `I`.
fn addr_ap(decoded: &DecodedOp, immext: Option<u32>) -> Option<(AddrMode, bool)> {
    let areg = field_u8(decoded, b'e')?;
    let (addr, used) = decode_field_uimm(decoded, b'I', immext)?;
    Some((AddrMode::AbsSet { areg, addr }, used))
}

/// `mem...(Ru<<#u2+##U6)` — scaled-index absolute (`L4_*_ur`). Field `t` = Ru
/// index, `i` = #u2 shift, `I` = low 6 bits of the extended absolute address.
fn addr_ur(decoded: &DecodedOp, immext: Option<u32>) -> Option<(AddrMode, bool)> {
    let index = field_u8(decoded, b't')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    let (addr, used) = decode_field_uimm(decoded, b'I', immext)?;
    Some((AddrMode::IndexAbs { index, shift, addr }, used))
}

/// Build a `LoadAlign` (FIFO) insn from a resolved address. `width` is Byte
/// (shift 8) or Half (shift 16); the read-modify dest pair Ryy is field `y`.
fn loadalign(
    decoded: &DecodedOp,
    addr: AddrMode,
    width: MemWidth,
    used: bool,
) -> Option<(DecodedInsn, bool)> {
    let dst_pair = field_u8(decoded, b'y')?;
    Some((
        DecodedInsn::LoadAlign {
            dst_pair,
            addr,
            width,
            pred: None,
        },
        used,
    ))
}

/// Build a `LoadUnpack` (bsw/bzw) insn from a resolved address. `count` is 2 or
/// 4 bytes; `sign` selects membh (Signed) vs memubh (Unsigned). Dest is `d`.
fn loadunpack(
    decoded: &DecodedOp,
    addr: AddrMode,
    count: u8,
    sign: MemSign,
    used: bool,
) -> Option<(DecodedInsn, bool)> {
    let dst = field_u8(decoded, b'd')?;
    Some((
        DecodedInsn::LoadUnpack {
            dst,
            addr,
            count,
            sign,
            pred: None,
        },
        used,
    ))
}

/// Base register-offset load (`Rd=memX(Rs+Ru<<#u2)`, `L4_loadr*_rr`). Fields:
/// `s` = Rs base, `t` = Ru index, `i` = #u2 shift, `d` = dest.
fn load_rr(decoded: &DecodedOp, width: MemWidth, sign: MemSign) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let index = field_u8(decoded, b't')?;
    let dst = field_u8(decoded, b'd')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::RegScaled { base, index, shift },
            width,
            sign,
            pred: None,
        },
        false,
    ))
}

/// Build a plain (non-predicated) `Load` from an already-resolved address
/// `(AddrMode, extender_used)`. Dest is field `d` (the `_ap`/`_ur` L4 forms).
fn load_simple(
    decoded: &DecodedOp,
    width: MemWidth,
    sign: MemSign,
    addr_used: (AddrMode, bool),
) -> Option<(DecodedInsn, bool)> {
    let dst = field_u8(decoded, b'd')?;
    Some((
        DecodedInsn::Load {
            dst,
            addr: addr_used.0,
            width,
            sign,
            pred: None,
        },
        addr_used.1,
    ))
}

/// Atomic load (`L2_loadw_locked`/`_aq`, `L4_loadd_locked`/`_aq`). In
/// single-thread user mode these are plain word / doubleword loads
/// (`Rd=memw(Rs)` / `Rdd=memd(Rs)`); the acquire-barrier has no observable
/// effect on register / memory state. The `_locked` forms additionally set a
/// reservation on the accessed address (consumed by a later store-conditional),
/// so they decode to `LoadLocked`; the `_aq` forms are plain `Load`. Fields:
/// `s` = base, `d` = dest. Always reads at `Rs+0`.
fn load_atomic(decoded: &DecodedOp, width: MemWidth, locked: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let dst = field_u8(decoded, b'd')?;
    if locked {
        Some((DecodedInsn::LoadLocked { dst, base, width }, false))
    } else {
        Some((
            DecodedInsn::Load {
                dst,
                addr: AddrMode::Offset { base, offset: 0 },
                width,
                sign: MemSign::Unsigned,
                pred: None,
            },
            false,
        ))
    }
}

/// Predicated post-increment load (`if ([!]Pv[.new]) Rd=memX(Rx++#s4:N)`,
/// `L2_ploadr*_pi`). Fields: `x` = base, `d` = dest, `t` = Pv, `i` = signed
/// increment (scaled by the access width). A not-taken load CANCELS: no Rd
/// write and no post-increment.
fn pred_load_pi(
    decoded: &DecodedOp,
    width: MemWidth,
    sign: MemSign,
    sense: bool,
    pred_new: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let dst = field_u8(decoded, b'd')?;
    let pred = field_u8(decoded, b't')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::PostIncImm { base, offset },
            width,
            sign,
            pred: Some(pred_cond(pred, sense, pred_new)),
        },
        false,
    ))
}

/// Predicated register-offset load (`if ([!]Pv[.new]) Rd=memX(Rs+Ru<<#u2)`,
/// `L4_ploadr*_rr`). Fields: `s` = Rs base, `t` = Ru index, `i` = #u2 shift,
/// `d` = dest, `v` = Pv.
fn pred_load_rr(
    decoded: &DecodedOp,
    width: MemWidth,
    sign: MemSign,
    sense: bool,
    pred_new: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let index = field_u8(decoded, b't')?;
    let dst = field_u8(decoded, b'd')?;
    let pred = field_u8(decoded, b'v')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::RegScaled { base, index, shift },
            width,
            sign,
            pred: Some(pred_cond(pred, sense, pred_new)),
        },
        false,
    ))
}

/// Predicated absolute load (`if ([!]Pv[.new]) Rd=memX(##addr)`,
/// `L4_ploadr*_abs`). Fields: `d` = dest, `t` = Pv, `i` = absolute address
/// (low 6 bits, completed by the constant extender).
fn pred_load_abs(
    decoded: &DecodedOp,
    width: MemWidth,
    sign: MemSign,
    sense: bool,
    pred_new: bool,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let dst = field_u8(decoded, b'd')?;
    let pred = field_u8(decoded, b't')?;
    let (addr, used) = decode_field_uimm(decoded, b'i', immext)?;
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::Abs { addr },
            width,
            sign,
            pred: Some(pred_cond(pred, sense, pred_new)),
        },
        used,
    ))
}

/// `memX(Rx++Mu)=Rt` post-increment store by the M register (`S2_store*_pr`).
fn store_pr(decoded: &DecodedOp, width: MemWidth, src_new: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let src = field_u8(decoded, b't')?;
    let modsel = field_u8(decoded, b'u')?;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncReg { base, modsel },
            width,
            pred: None,
            src_new,
            high_half: false,
        },
        false,
    ))
}

/// `memX(Rx++Mu:brev)=Rt` bit-reverse post-increment store (`S2_store*_pbr`).
fn store_pbr(decoded: &DecodedOp, width: MemWidth, src_new: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let src = field_u8(decoded, b't')?;
    let modsel = field_u8(decoded, b'u')?;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncBrev { base, modsel },
            width,
            pred: None,
            src_new,
            high_half: false,
        },
        false,
    ))
}

/// `memX(Rx++#s4:N:circ(Mu))=Rt` circular post-increment store by immediate
/// (`S2_store*_pci`). The store source for *_pci uses the `t`/`SRC` field.
fn store_pci(decoded: &DecodedOp, width: MemWidth, src_new: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let src = field_u8(decoded, b't')?;
    let modsel = field_u8(decoded, b'u')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let incr = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncCircImm { base, modsel, incr },
            width,
            pred: None,
            src_new,
            high_half: false,
        },
        false,
    ))
}

/// `memX(Rx++I:circ(Mu))=Rt` circular post-increment store by register
/// (`S2_store*_pcr`).
fn store_pcr(decoded: &DecodedOp, width: MemWidth, src_new: bool) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let src = field_u8(decoded, b't')?;
    let modsel = field_u8(decoded, b'u')?;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncCircReg {
                base,
                modsel,
                shift: width_shift(width),
            },
            width,
            pred: None,
            src_new,
            high_half: false,
        },
        false,
    ))
}

/// New-value store with circular/post-increment register addressing. The
/// new-value source register comes from `t` (the `Nt8` field).
fn store_new_addr(
    decoded: &DecodedOp,
    width: MemWidth,
    addr: AddrMode,
) -> Option<(DecodedInsn, bool)> {
    let nt = field_u8(decoded, b't')?;
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr,
            width,
            pred: None,
        },
        false,
    ))
}

fn store_new_pi(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    store_new_addr(decoded, width, AddrMode::PostIncImm { base, offset })
}

fn store_new_pr(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    store_new_addr(decoded, width, AddrMode::PostIncReg { base, modsel })
}

fn store_new_pbr(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    store_new_addr(decoded, width, AddrMode::PostIncBrev { base, modsel })
}

fn store_new_pci(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let incr = imm.wrapping_shl(width_shift(width) as u32);
    store_new_addr(
        decoded,
        width,
        AddrMode::PostIncCircImm { base, modsel, incr },
    )
}

fn store_new_pcr(decoded: &DecodedOp, width: MemWidth) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let modsel = field_u8(decoded, b'u')?;
    store_new_addr(
        decoded,
        width,
        AddrMode::PostIncCircReg {
            base,
            modsel,
            shift: width_shift(width),
        },
    )
}

fn pred_load_io(
    decoded: &DecodedOp,
    width: MemWidth,
    sign: MemSign,
    sense: bool,
    pred_new: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let dst = field_u8(decoded, b'd')?;
    let pred = field_u8(decoded, b't')?;
    let (imm, _) = decode_field_uimm(decoded, b'i', None)?;
    let offset = (imm << width_shift(width)) as i32;
    Some((
        DecodedInsn::Load {
            dst,
            addr: AddrMode::Offset { base, offset },
            width,
            sign,
            pred: Some(pred_cond(pred, sense, pred_new)),
        },
        false,
    ))
}

fn pred_store_io(
    decoded: &DecodedOp,
    width: MemWidth,
    sense: bool,
    pred_new: bool,
    src_new: bool,
    high_half: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let src = field_u8(decoded, b't')?;
    let pred = field_u8(decoded, b'v')?;
    let (imm, _) = decode_field_uimm(decoded, b'i', None)?;
    let offset = (imm << width_shift(width)) as i32;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::Offset { base, offset },
            width,
            pred: Some(pred_cond(pred, sense, pred_new)),
            src_new,
            high_half,
        },
        false,
    ))
}

/// Predicated post-increment store (`if ([!]Pv[.new]) memX(Rx++#s4:N)=Rt[.h]`).
/// Fields: `x` = base (post-incremented), `t` = data Rt, `v` = Pv, `i` = signed
/// increment (scaled by the access size). A not-taken store performs NO
/// post-increment (full cancel).
fn pred_store_pi(
    decoded: &DecodedOp,
    width: MemWidth,
    sense: bool,
    pred_new: bool,
    high_half: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let src = field_u8(decoded, b't')?;
    let pred = field_u8(decoded, b'v')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::PostIncImm { base, offset },
            width,
            pred: Some(pred_cond(pred, sense, pred_new)),
            src_new: false,
            high_half,
        },
        false,
    ))
}

/// Predicated NEW-VALUE post-increment store (`if ([!]Pv[.new]) memX(Rx++#s4:N)
/// =Nt8.new`). Like `pred_store_pi` but the data source is the `Nt8` producer
/// (resolved against the packet's producers), so we emit `StoreNew`.
fn pred_store_new_pi(
    decoded: &DecodedOp,
    width: MemWidth,
    sense: bool,
    pred_new: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b'x')?;
    let nt = field_u8(decoded, b't')?;
    let pred = field_u8(decoded, b'v')?;
    let (imm, _) = decode_field_simm(decoded, b'i', None)?;
    let offset = imm.wrapping_shl(width_shift(width) as u32);
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr: AddrMode::PostIncImm { base, offset },
            width,
            pred: Some(pred_cond(pred, sense, pred_new)),
        },
        false,
    ))
}

/// Predicated register-offset store (`if ([!]Pv[.new]) memX(Rs+Ru<<#u2)=Rt[.h]`,
/// `S4_pstorer*_rr`). Fields: `s` = Rs base, `u` = Ru index, `i` = #u2 shift,
/// `t` = data Rt, `v` = Pv.
fn pred_store_rr(
    decoded: &DecodedOp,
    width: MemWidth,
    sense: bool,
    pred_new: bool,
    high_half: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let index = field_u8(decoded, b'u')?;
    let src = field_u8(decoded, b't')?;
    let pred = field_u8(decoded, b'v')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::RegScaled { base, index, shift },
            width,
            pred: Some(pred_cond(pred, sense, pred_new)),
            src_new: false,
            high_half,
        },
        false,
    ))
}

/// Predicated NEW-VALUE register-offset store (`S4_pstorer*new_rr`). The data
/// source is the `Nt8` producer (`t`), so we emit `StoreNew`.
fn pred_store_new_rr(
    decoded: &DecodedOp,
    width: MemWidth,
    sense: bool,
    pred_new: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let index = field_u8(decoded, b'u')?;
    let nt = field_u8(decoded, b't')?;
    let pred = field_u8(decoded, b'v')?;
    let shift = decode_field_uimm(decoded, b'i', None)?.0 as u8;
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr: AddrMode::RegScaled { base, index, shift },
            width,
            pred: Some(pred_cond(pred, sense, pred_new)),
        },
        false,
    ))
}

/// Predicated absolute store (`if ([!]Pv[.new]) memX(##addr)=Rt[.h]`,
/// `S4_pstorer*_abs`). The address is the constant-extended `i` field. Fields:
/// `i` = absolute address (extended), `t` = data Rt, `v` = Pv.
fn pred_store_abs(
    decoded: &DecodedOp,
    width: MemWidth,
    sense: bool,
    pred_new: bool,
    high_half: bool,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let src = field_u8(decoded, b't')?;
    let pred = field_u8(decoded, b'v')?;
    let (addr, used) = decode_field_uimm(decoded, b'i', immext)?;
    Some((
        DecodedInsn::Store {
            src,
            addr: AddrMode::Abs { addr },
            width,
            pred: Some(pred_cond(pred, sense, pred_new)),
            src_new: false,
            high_half,
        },
        used,
    ))
}

/// Predicated NEW-VALUE absolute store (`S4_pstorer*new_abs`). Data source is
/// the `Nt8` producer (`t`), so we emit `StoreNew`.
fn pred_store_new_abs(
    decoded: &DecodedOp,
    width: MemWidth,
    sense: bool,
    pred_new: bool,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    let nt = field_u8(decoded, b't')?;
    let pred = field_u8(decoded, b'v')?;
    let (addr, used) = decode_field_uimm(decoded, b'i', immext)?;
    Some((
        DecodedInsn::StoreNew {
            nt,
            addr: AddrMode::Abs { addr },
            width,
            pred: Some(pred_cond(pred, sense, pred_new)),
        },
        used,
    ))
}

/// Store-immediate `memX(Rs+#u6:N)=#s6` (`S4_storeir<W>[cond]_io`). Fields:
/// `s` = base, `i` = unsigned offset (scaled by access size), `I` = signed #s6
/// value, `v` = Pv (only for the predicated forms). The value is sign-extended
/// from 6 bits. `pred` is `None` for the unconditional `S4_storeir<W>_io`.
fn store_imm_io(
    decoded: &DecodedOp,
    width: MemWidth,
    pred: Option<(bool, bool)>,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let (imm, _) = decode_field_uimm(decoded, b'i', None)?;
    let offset = (imm << width_shift(width)) as i32;
    let raw = decode_field_uimm(decoded, b'I', None)?.0;
    let value = sign_extend(raw, 6) as u32;
    let pred = match pred {
        Some((sense, pred_new)) => Some(pred_cond(field_u8(decoded, b'v')?, sense, pred_new)),
        None => None,
    };
    Some((
        DecodedInsn::StoreImm {
            value,
            addr: AddrMode::Offset { base, offset },
            width,
            pred,
        },
        false,
    ))
}

/// Decode a read-modify-write memop (`memX(Rs+#u6:N) OP= Rt|#U5`).
/// `op` selects the operation; `imm_src` chooses the `#U5` immediate operand
/// (field `I`) over the register operand (field `t`).
fn memop(
    decoded: &DecodedOp,
    width: MemWidth,
    op: MemOpKind,
    imm_src: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b's')?;
    let imm = decode_field_uimm(decoded, b'i', None)?.0;
    let offset = (imm << width_shift(width)) as i32;
    let src = if imm_src {
        MemOpSrc::Imm(decode_field_uimm(decoded, b'I', None)?.0)
    } else {
        MemOpSrc::Reg(field_u8(decoded, b't')?)
    };
    Some((
        DecodedInsn::MemOp {
            base,
            offset,
            width,
            op,
            src,
        },
        false,
    ))
}

const HVX_VEC_BYTES: i32 = 128;

/// HVX vmem addressing mode.
#[derive(Clone, Copy, PartialEq, Eq)]
enum VAddr {
    /// `vmem(Rt+#s4)` — offset form (base field `t`, immediate is the offset).
    Ai,
    /// `vmem(Rt++#s3)` — post-increment by an immediate (base field `x`).
    Pi,
    /// `vmem(Rt++Mu)` — post-increment by the modifier register (base field `x`,
    /// modifier-select field `u`: 0 => M0/C6, 1 => M1/C7).
    Ppu,
}

impl VAddr {
    /// Base register field letter: `t` for the offset form, `x` (read-modify) for
    /// either post-increment form.
    fn base_letter(self) -> u8 {
        match self {
            VAddr::Ai => b't',
            VAddr::Pi | VAddr::Ppu => b'x',
        }
    }

    /// Resolve to `(offset, post_inc, post_inc_mod)` from the decoded fields.
    fn addr_fields(self, decoded: &DecodedOp) -> Option<(i32, Option<i32>, Option<u8>)> {
        match self {
            VAddr::Ai => {
                let imm = decode_field_simm(decoded, b'i', None)?.0 * HVX_VEC_BYTES;
                Some((imm, None, None))
            }
            VAddr::Pi => {
                let imm = decode_field_simm(decoded, b'i', None)?.0 * HVX_VEC_BYTES;
                Some((0, Some(imm), None))
            }
            VAddr::Ppu => {
                let modsel = field_u8(decoded, b'u')?;
                Some((0, None, Some(modsel)))
            }
        }
    }
}

/// Decode an HVX vector load `Vd = vmem(...)`. `am` selects the addressing mode.
fn vmem_load(decoded: &DecodedOp, am: VAddr, aligned: bool) -> Option<(DecodedInsn, bool)> {
    vmem_load_full(decoded, am, aligned, true, None)
}

fn vmem_load_c(
    decoded: &DecodedOp,
    am: VAddr,
    aligned: bool,
    commit: bool,
) -> Option<(DecodedInsn, bool)> {
    vmem_load_full(decoded, am, aligned, commit, None)
}

/// Scalar-predicated load `if (Pv[!]) Vd[.cur|.tmp] = vmem(...)`. Predicate field
/// is `v`; `commit` distinguishes the committing forms (plain / `.cur`) from the
/// non-committing `.tmp` form.
fn vmem_load_pred(
    decoded: &DecodedOp,
    am: VAddr,
    aligned: bool,
    sense: bool,
    commit: bool,
) -> Option<(DecodedInsn, bool)> {
    let pred = pred_cond(field_u8(decoded, b'v')?, sense, false);
    vmem_load_full(decoded, am, aligned, commit, Some(pred))
}

fn vmem_load_full(
    decoded: &DecodedOp,
    am: VAddr,
    aligned: bool,
    commit: bool,
    pred: Option<PredCond>,
) -> Option<(DecodedInsn, bool)> {
    let dst = field_u8(decoded, b'd')?;
    let base = field_u8(decoded, am.base_letter())?;
    let (offset, post_inc, post_inc_mod) = am.addr_fields(decoded)?;
    Some((
        DecodedInsn::VLoad {
            dst,
            base,
            offset,
            post_inc,
            post_inc_mod,
            aligned,
            commit,
            pred,
        },
        false,
    ))
}

fn vmem_store(decoded: &DecodedOp, am: VAddr, aligned: bool) -> Option<(DecodedInsn, bool)> {
    vmem_store_pred(decoded, am, aligned, None)
}

/// `pred_sense`: `Some(true)` for `if (Pv) ...`, `Some(false)` for `if (!Pv) ...`,
/// `None` for an unconditional store. The predicate operand is field `v`.
fn vmem_store_pred(
    decoded: &DecodedOp,
    am: VAddr,
    aligned: bool,
    pred_sense: Option<bool>,
) -> Option<(DecodedInsn, bool)> {
    let src = field_u8(decoded, b's')?;
    let base = field_u8(decoded, am.base_letter())?;
    let (offset, post_inc, post_inc_mod) = am.addr_fields(decoded)?;
    let pred = match pred_sense {
        Some(sense) => Some(pred_cond(field_u8(decoded, b'v')?, sense, false)),
        None => None,
    };
    Some((
        DecodedInsn::VStore {
            src,
            base,
            offset,
            post_inc,
            post_inc_mod,
            aligned,
            pred,
            qmask: None,
            from_gather: false,
            srls: false,
        },
        false,
    ))
}

/// Byte-masked vector store `if (Qv[!]) vmem(...) = Vs` (`qpred`/`nqpred`).
fn vmem_store_q(decoded: &DecodedOp, am: VAddr, sense: bool) -> Option<(DecodedInsn, bool)> {
    let src = field_u8(decoded, b's')?;
    let base = field_u8(decoded, am.base_letter())?;
    let qv = field_u8(decoded, b'v')?;
    let (offset, post_inc, post_inc_mod) = am.addr_fields(decoded)?;
    Some((
        DecodedInsn::VStore {
            src,
            base,
            offset,
            post_inc,
            post_inc_mod,
            aligned: true,
            pred: None,
            qmask: Some((qv, sense)),
            from_gather: false,
            srls: false,
        },
        false,
    ))
}

/// New-value vector store `vmem(...) = V.new` (`vS32b_new[_pred|_npred]`). The
/// `.new` source is the vector produced earlier in this packet (or the per-packet
/// gather scratch for the `vmem=vtmp.new` idiom); the executor resolves it. Base
/// register field is `t` (offset) or `x` (post-inc). `pred_sense` is `Some` for
/// the scalar-predicated `if (Pv[!])` forms (predicate field `v`).
fn vmem_store_new(
    decoded: &DecodedOp,
    am: VAddr,
    pred_sense: Option<bool>,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, am.base_letter())?;
    let (offset, post_inc, post_inc_mod) = am.addr_fields(decoded)?;
    let pred = match pred_sense {
        Some(sense) => Some(pred_cond(field_u8(decoded, b'v')?, sense, false)),
        None => None,
    };
    Some((
        DecodedInsn::VStore {
            src: 0,
            base,
            offset,
            post_inc,
            post_inc_mod,
            aligned: true,
            pred,
            qmask: None,
            from_gather: true,
            srls: false,
        },
        false,
    ))
}

/// Store-release barrier `vmem(...):scatter_release` (`vS32b_srls`). No vector
/// source and no memory write; only the post-increment (if any) takes effect.
fn vmem_store_srls(decoded: &DecodedOp, am: VAddr) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, am.base_letter())?;
    let (offset, post_inc, post_inc_mod) = am.addr_fields(decoded)?;
    Some((
        DecodedInsn::VStore {
            src: 0,
            base,
            offset,
            post_inc,
            post_inc_mod,
            aligned: true,
            pred: None,
            qmask: None,
            from_gather: false,
            srls: true,
        },
        false,
    ))
}

/// HVX V65 scatter (`vscatter(Rt,Mu,Vv.{w,h})[.{w,h}][+]=Vw`). `esz` is the data
/// element size (2 or 4); `off_pair` selects the `Vvv.w` double-resource forms;
/// `add` selects accumulate; `pred_sense` (`Some`) marks the `q` predicated forms.
fn vscatter(
    decoded: &DecodedOp,
    esz: u8,
    off_pair: bool,
    add: bool,
    predicated: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b't')?;
    let modsel = field_u8(decoded, b'u')?;
    let offsets = field_u8(decoded, b'v')?;
    let data = field_u8(decoded, b'w')?;
    let pred = if predicated {
        Some(field_u8(decoded, b's')?)
    } else {
        None
    };
    Some((
        DecodedInsn::VScatter {
            base,
            modsel,
            offsets,
            data,
            esz,
            off_pair,
            add,
            pred,
        },
        false,
    ))
}

/// HVX V65 gather (`vtmp.{w,h}=vgather(Rt,Mu,Vv.{w,h}).{w,h}`). The gather has no
/// architectural destination; it writes the per-packet scratch consumed by a
/// paired `vmem(Rs+#k)=vtmp.new`.
fn vgather(
    decoded: &DecodedOp,
    esz: u8,
    off_pair: bool,
    predicated: bool,
) -> Option<(DecodedInsn, bool)> {
    let base = field_u8(decoded, b't')?;
    let modsel = field_u8(decoded, b'u')?;
    let offsets = field_u8(decoded, b'v')?;
    let pred = if predicated {
        Some(field_u8(decoded, b's')?)
    } else {
        None
    };
    Some((
        DecodedInsn::VGather {
            base,
            modsel,
            offsets,
            esz,
            off_pair,
            pred,
        },
        false,
    ))
}

/// Decode a J4 compound compare-and-jump (`J4_<cmp>_<cond>_jump[nv]_<hint>`) by
/// parsing the mnemonic. Returns `(insn, used_ext)`. The branch displacement is
/// the `i` field (signed, `<<2`); the immediate compares read `#u5`/`#u6` from
/// the `I` field with the imm-extension applied (so a preceding `immext` widens
/// the compare constant). Falls through to `None` for any unrecognised name.
fn decode_compound_cmpjump(
    decoded: &DecodedOp,
    name: &str,
    immext: Option<u32>,
) -> Option<(DecodedInsn, bool)> {
    // name == "J4_<cmp>_<cond>_jump[nv]_<hint>"
    let body = name.strip_prefix("J4_")?;
    // Split off the hint (last segment, "_t" or "_nt"): no architectural effect.
    let body = body
        .strip_suffix("_t")
        .or_else(|| body.strip_suffix("_nt"))?;
    let new_value = body.ends_with("_jumpnv");
    let core = body
        .strip_suffix("_jumpnv")
        .or_else(|| body.strip_suffix("_jump"))?;
    // core == "<cmp>_<cond>"; cond is the final "_<tp0|fp0|tp1|fp1|t|f>".
    let (cmp, cond) = core.rsplit_once('_')?;

    // Taken sense + predicate-to-write from the condition segment.
    let (sense, write_pred) = match cond {
        "tp0" => (true, Some(0u8)),
        "fp0" => (false, Some(0u8)),
        "tp1" => (true, Some(1u8)),
        "fp1" => (false, Some(1u8)),
        "t" => (true, None),
        "f" => (false, None),
        _ => return None,
    };

    // Only the branch offset (`ri`) is imm-extensible (`fIMMEXT(riV)`); the
    // compare immediate `#u5` is a fixed 5-bit field and is never extended.
    let (offset, used) = decode_field_scaled_simm(decoded, b'i', immext, 2)?;
    let src1 = field_u8(decoded, b's')?;
    // `Rt` (or `#u5`) operand depending on the compare form.
    let src2 = field_u8(decoded, b't').unwrap_or(0);

    let kind = match cmp {
        "cmpeq" => CmpJumpKind::Eq,
        "cmpgt" => CmpJumpKind::Gt,
        "cmpgtu" => CmpJumpKind::Gtu,
        "cmplt" => CmpJumpKind::Lt,
        "cmpltu" => CmpJumpKind::Ltu,
        "cmpeqn1" => CmpJumpKind::EqN1,
        "cmpgtn1" => CmpJumpKind::GtN1,
        "tstbit0" => CmpJumpKind::TstBit0,
        "cmpeqi" => CmpJumpKind::EqImm(decode_field_uimm(decoded, b'I', None)?.0 as i32),
        "cmpgti" => CmpJumpKind::GtImm(decode_field_uimm(decoded, b'I', None)?.0 as i32),
        "cmpgtui" => CmpJumpKind::GtuImm(decode_field_uimm(decoded, b'I', None)?.0),
        _ => return None,
    };

    Some((
        DecodedInsn::CompoundCmpJump {
            kind,
            src1,
            src2,
            write_pred,
            sense,
            new_value,
            offset,
        },
        used,
    ))
}

fn decode_main(decoded: &DecodedOp, word: u32, immext: Option<u32>) -> (DecodedInsn, bool) {
    macro_rules! req {
        ($expr:expr) => {
            match $expr {
                Some(value) => value,
                None => return (DecodedInsn::Unknown(word), false),
            }
        };
    }

    match decoded.opcode {
        Opcode::A4_ext => {
            let imm = req!(field_val(decoded, b'i')).value;
            (DecodedInsn::ImmExt { value: imm }, false)
        }
        Opcode::A2_add => {
            let dst = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b's'));
            let src2 = req!(field_u8(decoded, b't'));
            (DecodedInsn::Add { dst, src1, src2 }, false)
        }
        Opcode::A2_sub => {
            let dst = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b't'));
            let src2 = req!(field_u8(decoded, b's'));
            (DecodedInsn::Sub { dst, src1, src2 }, false)
        }
        Opcode::A2_and => {
            let dst = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b's'));
            let src2 = req!(field_u8(decoded, b't'));
            (DecodedInsn::And { dst, src1, src2 }, false)
        }
        Opcode::A2_or => {
            let dst = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b's'));
            let src2 = req!(field_u8(decoded, b't'));
            (DecodedInsn::Or { dst, src1, src2 }, false)
        }
        Opcode::A2_xor => {
            let dst = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b's'));
            let src2 = req!(field_u8(decoded, b't'));
            (DecodedInsn::Xor { dst, src1, src2 }, false)
        }
        Opcode::A2_addi => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let (imm, used) = req!(decode_field_simm(decoded, b'i', immext));
            (DecodedInsn::AddImm { dst, src, imm }, used)
        }
        Opcode::A2_tfr => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            (DecodedInsn::Mov { dst, src }, false)
        }
        Opcode::A2_tfrsi => {
            let dst = req!(field_u8(decoded, b'd'));
            let (imm, used) = req!(decode_field_simm(decoded, b'i', immext));
            (DecodedInsn::MovImm { dst, imm }, used)
        }
        Opcode::A2_andir => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let (imm, used) = req!(decode_field_simm(decoded, b'i', immext));
            (
                DecodedInsn::AndImm {
                    dst,
                    src,
                    imm: imm as u32,
                },
                used,
            )
        }
        Opcode::A2_sxtb => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            (
                DecodedInsn::Extend {
                    dst,
                    src,
                    kind: ExtendKind::Sxt8,
                },
                false,
            )
        }
        Opcode::A2_sxth => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            (
                DecodedInsn::Extend {
                    dst,
                    src,
                    kind: ExtendKind::Sxt16,
                },
                false,
            )
        }
        Opcode::A2_zxth => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            (
                DecodedInsn::Extend {
                    dst,
                    src,
                    kind: ExtendKind::Zxt16,
                },
                false,
            )
        }
        Opcode::A2_tfrcrr => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            (DecodedInsn::TfrCrR { dst, src }, false)
        }
        Opcode::A2_tfrrcr => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            (DecodedInsn::TfrRrCr { dst, src }, false)
        }
        Opcode::A4_tfrcpp => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            (DecodedInsn::TfrCrRPair { dst, src }, false)
        }
        Opcode::A4_tfrpcp => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            (DecodedInsn::TfrRrCrPair { dst, src }, false)
        }
        Opcode::Y2_dczeroa => {
            let base = req!(field_u8(decoded, b's'));
            (DecodedInsn::DcZero { base }, false)
        }
        Opcode::M2_mpyi => {
            let dst = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b's'));
            let src2 = req!(field_u8(decoded, b't'));
            (DecodedInsn::Mul { dst, src1, src2 }, false)
        }
        Opcode::S2_asr_i_r => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let amount = req!(decode_field_uimm(decoded, b'i', None)).0 as u8;
            (
                DecodedInsn::ShiftImm {
                    dst,
                    src,
                    kind: ShiftKind::Asr,
                    amount,
                },
                false,
            )
        }
        Opcode::S2_lsr_i_r => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let amount = req!(decode_field_uimm(decoded, b'i', None)).0 as u8;
            (
                DecodedInsn::ShiftImm {
                    dst,
                    src,
                    kind: ShiftKind::Lsr,
                    amount,
                },
                false,
            )
        }
        Opcode::S2_asl_i_r => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let amount = req!(decode_field_uimm(decoded, b'i', None)).0 as u8;
            (
                DecodedInsn::ShiftImm {
                    dst,
                    src,
                    kind: ShiftKind::Lsl,
                    amount,
                },
                false,
            )
        }
        Opcode::S2_asr_r_r => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let amt = req!(field_u8(decoded, b't'));
            (
                DecodedInsn::ShiftReg {
                    dst,
                    src,
                    amt,
                    kind: ShiftKind::Asr,
                },
                false,
            )
        }
        Opcode::S2_lsr_r_r => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let amt = req!(field_u8(decoded, b't'));
            (
                DecodedInsn::ShiftReg {
                    dst,
                    src,
                    amt,
                    kind: ShiftKind::Lsr,
                },
                false,
            )
        }
        Opcode::S2_asl_r_r => {
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let amt = req!(field_u8(decoded, b't'));
            (
                DecodedInsn::ShiftReg {
                    dst,
                    src,
                    amt,
                    kind: ShiftKind::Lsl,
                },
                false,
            )
        }
        Opcode::C2_cmpeq => {
            let pred = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b's'));
            let src2 = req!(field_u8(decoded, b't'));
            (
                DecodedInsn::Cmp {
                    pred,
                    src1,
                    src2,
                    kind: CmpKind::Eq,
                },
                false,
            )
        }
        Opcode::C2_cmpgt => {
            let pred = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b's'));
            let src2 = req!(field_u8(decoded, b't'));
            (
                DecodedInsn::Cmp {
                    pred,
                    src1,
                    src2,
                    kind: CmpKind::Gt,
                },
                false,
            )
        }
        Opcode::C2_cmpgtu => {
            let pred = req!(field_u8(decoded, b'd'));
            let src1 = req!(field_u8(decoded, b's'));
            let src2 = req!(field_u8(decoded, b't'));
            (
                DecodedInsn::Cmp {
                    pred,
                    src1,
                    src2,
                    kind: CmpKind::Gtu,
                },
                false,
            )
        }
        Opcode::C2_cmpeqi => {
            let pred = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let (imm, used) = req!(decode_field_simm(decoded, b'i', immext));
            (
                DecodedInsn::CmpImm {
                    pred,
                    src,
                    imm,
                    kind: CmpKind::Eq,
                    unsigned: false,
                },
                used,
            )
        }
        Opcode::C2_cmpgti => {
            let pred = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let (imm, used) = req!(decode_field_simm(decoded, b'i', immext));
            (
                DecodedInsn::CmpImm {
                    pred,
                    src,
                    imm,
                    kind: CmpKind::Gt,
                    unsigned: false,
                },
                used,
            )
        }
        Opcode::C2_cmpgtui => {
            let pred = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let (imm, used) = req!(decode_field_uimm(decoded, b'i', immext));
            (
                DecodedInsn::CmpImm {
                    pred,
                    src,
                    imm: imm as i32,
                    kind: CmpKind::Gtu,
                    unsigned: true,
                },
                used,
            )
        }
        Opcode::J2_jump => {
            let (offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            (DecodedInsn::Jump { offset }, used)
        }
        Opcode::J2_call => {
            let (offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            (DecodedInsn::Call { offset, pred: None }, used)
        }
        Opcode::J2_callt | Opcode::J2_callf => {
            let pred = req!(field_u8(decoded, b'u'));
            let sense = matches!(decoded.opcode, Opcode::J2_callt);
            let (offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            (
                DecodedInsn::Call {
                    offset,
                    pred: Some(pred_cond(pred, sense, false)),
                },
                used,
            )
        }
        Opcode::J2_jumpr | Opcode::J2_jumprh => {
            // `jumprh` is a jump-register with a prefetch hint; same effect.
            let src = req!(field_u8(decoded, b's'));
            (DecodedInsn::JumpReg { src }, false)
        }
        Opcode::J4_hintjumpr => {
            // Jump-register hint: behaves exactly like `jumpr Rs`.
            let src = req!(field_u8(decoded, b's'));
            (DecodedInsn::JumpReg { src }, false)
        }
        Opcode::J2_callr | Opcode::J2_callrh => {
            // `callrh` is a call-register with a prefetch hint; same effect.
            let src = req!(field_u8(decoded, b's'));
            (DecodedInsn::CallReg { src, pred: None }, false)
        }
        Opcode::J2_callrt | Opcode::J2_callrf => {
            let src = req!(field_u8(decoded, b's'));
            let pred = req!(field_u8(decoded, b'u'));
            let sense = matches!(decoded.opcode, Opcode::J2_callrt);
            (
                DecodedInsn::CallReg {
                    src,
                    pred: Some(pred_cond(pred, sense, false)),
                },
                false,
            )
        }
        Opcode::J2_jumprz
        | Opcode::J2_jumprzpt
        | Opcode::J2_jumprnz
        | Opcode::J2_jumprnzpt
        | Opcode::J2_jumprgtez
        | Opcode::J2_jumprgtezpt
        | Opcode::J2_jumprltez
        | Opcode::J2_jumprltezpt => {
            // `if (cmp.<..>(Rs,#0)) jump #r13:2`: a DIRECT PC-relative jump whose
            // condition is a register compare-to-zero (the `pt` variants are a
            // taken hint with no architectural effect). Per the PRM/idef:
            //   jumprz   -> if (Rs != 0)   jumprnz  -> if (Rs == 0)
            //   jumprgtez-> if (Rs >= 0)   jumprltez-> if (Rs <= 0)
            let src = req!(field_u8(decoded, b's'));
            let (offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            let kind = match decoded.opcode {
                Opcode::J2_jumprz | Opcode::J2_jumprzpt => CmpKind::Ne,
                Opcode::J2_jumprnz | Opcode::J2_jumprnzpt => CmpKind::Eq,
                Opcode::J2_jumprgtez | Opcode::J2_jumprgtezpt => CmpKind::Gte,
                _ => CmpKind::Lte,
            };
            (DecodedInsn::JumpRegZero { src, kind, offset }, used)
        }
        Opcode::J4_jumpseti => {
            // `Rd = #u6 ; jump #r` — set a register then jump.
            let dst = req!(field_u8(decoded, b'd'));
            let imm = req!(decode_field_uimm(decoded, b'I', None)).0;
            let (offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            (
                DecodedInsn::JumpSet {
                    dst,
                    value: JumpSetSrc::Imm(imm),
                    offset,
                },
                used,
            )
        }
        Opcode::J4_jumpsetr => {
            // `Rd = Rs ; jump #r` — copy a register then jump.
            let dst = req!(field_u8(decoded, b'd'));
            let src = req!(field_u8(decoded, b's'));
            let (offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            (
                DecodedInsn::JumpSet {
                    dst,
                    value: JumpSetSrc::Reg(src),
                    offset,
                },
                used,
            )
        }
        Opcode::J2_pause => {
            // Architecturally a NOP for our purposes (no observable state change
            // beyond advancing PC). The `#u8` operand is the pause cycle count.
            (DecodedInsn::Nop, false)
        }
        Opcode::J2_jumpt
        | Opcode::J2_jumptpt
        | Opcode::J2_jumptnew
        | Opcode::J2_jumptnewpt
        | Opcode::J2_jumpf
        | Opcode::J2_jumpfpt
        | Opcode::J2_jumpfnew
        | Opcode::J2_jumpfnewpt => {
            let pred = req!(field_u8(decoded, b'u'));
            let (offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            let (sense, pred_new) = match decoded.opcode {
                Opcode::J2_jumpt | Opcode::J2_jumptpt => (true, false),
                Opcode::J2_jumpf | Opcode::J2_jumpfpt => (false, false),
                Opcode::J2_jumptnew | Opcode::J2_jumptnewpt => (true, true),
                _ => (false, true),
            };
            (
                DecodedInsn::JumpCond {
                    offset,
                    pred,
                    sense,
                    pred_new,
                },
                used,
            )
        }
        Opcode::J2_jumprt
        | Opcode::J2_jumprtpt
        | Opcode::J2_jumprtnew
        | Opcode::J2_jumprtnewpt
        | Opcode::J2_jumprf
        | Opcode::J2_jumprfpt
        | Opcode::J2_jumprfnew
        | Opcode::J2_jumprfnewpt => {
            let src = req!(field_u8(decoded, b's'));
            let pred = req!(field_u8(decoded, b'u'));
            let (sense, pred_new) = match decoded.opcode {
                Opcode::J2_jumprt | Opcode::J2_jumprtpt => (true, false),
                Opcode::J2_jumprf | Opcode::J2_jumprfpt => (false, false),
                Opcode::J2_jumprtnew | Opcode::J2_jumprtnewpt => (true, true),
                _ => (false, true),
            };
            (
                DecodedInsn::JumpRegCond {
                    src,
                    pred,
                    sense,
                    pred_new,
                },
                false,
            )
        }
        Opcode::J2_loop0r
        | Opcode::J2_loop1r
        | Opcode::J2_ploop1sr
        | Opcode::J2_ploop2sr
        | Opcode::J2_ploop3sr => {
            let (loop_id, lpcfg) = match decoded.opcode {
                Opcode::J2_loop0r => (0, None),
                Opcode::J2_loop1r => (1, None),
                Opcode::J2_ploop1sr => (0, Some(1)),
                Opcode::J2_ploop2sr => (0, Some(2)),
                _ => (0, Some(3)),
            };
            let count_reg = req!(field_u8(decoded, b's'));
            let (start_offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            (
                DecodedInsn::LoopStartReg {
                    loop_id,
                    start_offset,
                    count_reg,
                    lpcfg,
                },
                used,
            )
        }
        Opcode::J2_loop0i
        | Opcode::J2_loop1i
        | Opcode::J2_ploop1si
        | Opcode::J2_ploop2si
        | Opcode::J2_ploop3si => {
            let (loop_id, lpcfg) = match decoded.opcode {
                Opcode::J2_loop0i => (0, None),
                Opcode::J2_loop1i => (1, None),
                Opcode::J2_ploop1si => (0, Some(1)),
                Opcode::J2_ploop2si => (0, Some(2)),
                _ => (0, Some(3)),
            };
            let (start_offset, used) = req!(decode_field_scaled_simm(decoded, b'i', immext, 2));
            let count = req!(decode_field_uimm(decoded, b'I', None)).0;
            (
                DecodedInsn::LoopStartImm {
                    loop_id,
                    start_offset,
                    count,
                    lpcfg,
                },
                used,
            )
        }
        Opcode::J2_trap0 => (DecodedInsn::Trap0, false),
        Opcode::S2_allocframe => {
            let base = req!(field_u8(decoded, b'x'));
            let imm = req!(decode_field_uimm(decoded, b'i', None)).0;
            (
                DecodedInsn::AllocFrame {
                    base,
                    size: imm << 3,
                },
                false,
            )
        }
        Opcode::L2_deallocframe => {
            let base = req!(field_u8(decoded, b's'));
            let dst = req!(field_u8(decoded, b'd'));
            (
                DecodedInsn::DeallocFrame {
                    base,
                    dst: Some(dst),
                    update_lr_fp: false,
                },
                false,
            )
        }
        Opcode::L4_return
        | Opcode::L4_return_t
        | Opcode::L4_return_f
        | Opcode::L4_return_tnew_pt
        | Opcode::L4_return_tnew_pnt
        | Opcode::L4_return_fnew_pt
        | Opcode::L4_return_fnew_pnt => {
            let base = req!(field_u8(decoded, b's'));
            let dst = req!(field_u8(decoded, b'd'));
            let pred = match decoded.opcode {
                Opcode::L4_return => None,
                Opcode::L4_return_t => Some(pred_cond(req!(field_u8(decoded, b'v')), true, false)),
                Opcode::L4_return_f => Some(pred_cond(req!(field_u8(decoded, b'v')), false, false)),
                Opcode::L4_return_tnew_pt | Opcode::L4_return_tnew_pnt => {
                    Some(pred_cond(req!(field_u8(decoded, b'v')), true, true))
                }
                _ => Some(pred_cond(req!(field_u8(decoded, b'v')), false, true)),
            };
            (
                DecodedInsn::DeallocReturn {
                    base,
                    dst: Some(dst),
                    pred,
                    update_lr_fp: false,
                },
                false,
            )
        }
        Opcode::L2_loadrb_io => req!(load_io(decoded, MemWidth::Byte, MemSign::Signed, immext)),
        Opcode::L2_loadrub_io => req!(load_io(decoded, MemWidth::Byte, MemSign::Unsigned, immext)),
        Opcode::L2_loadrh_io => req!(load_io(decoded, MemWidth::Half, MemSign::Signed, immext)),
        Opcode::L2_loadruh_io => req!(load_io(decoded, MemWidth::Half, MemSign::Unsigned, immext)),
        Opcode::L2_loadri_io => req!(load_io(decoded, MemWidth::Word, MemSign::Unsigned, immext)),
        Opcode::L2_loadrd_io => req!(load_io(
            decoded,
            MemWidth::Double,
            MemSign::Unsigned,
            immext
        )),
        Opcode::L2_loadrbgp => req!(load_gp(decoded, MemWidth::Byte, MemSign::Signed, immext)),
        Opcode::L2_loadrubgp => req!(load_gp(decoded, MemWidth::Byte, MemSign::Unsigned, immext)),
        Opcode::L2_loadrhgp => req!(load_gp(decoded, MemWidth::Half, MemSign::Signed, immext)),
        Opcode::L2_loadruhgp => req!(load_gp(decoded, MemWidth::Half, MemSign::Unsigned, immext)),
        Opcode::L2_loadrigp => req!(load_gp(decoded, MemWidth::Word, MemSign::Unsigned, immext)),
        Opcode::L2_loadrdgp => req!(load_gp(
            decoded,
            MemWidth::Double,
            MemSign::Unsigned,
            immext
        )),
        Opcode::L2_loadrb_pi => req!(load_pi(decoded, MemWidth::Byte, MemSign::Signed)),
        Opcode::L2_loadrub_pi => req!(load_pi(decoded, MemWidth::Byte, MemSign::Unsigned)),
        Opcode::L2_loadrh_pi => req!(load_pi(decoded, MemWidth::Half, MemSign::Signed)),
        Opcode::L2_loadruh_pi => req!(load_pi(decoded, MemWidth::Half, MemSign::Unsigned)),
        Opcode::L2_loadri_pi => req!(load_pi(decoded, MemWidth::Word, MemSign::Unsigned)),
        Opcode::L2_loadrd_pi => req!(load_pi(decoded, MemWidth::Double, MemSign::Unsigned)),
        Opcode::S2_storerb_io => req!(store_io(decoded, MemWidth::Byte, immext, false)),
        Opcode::S2_storerh_io => req!(store_io(decoded, MemWidth::Half, immext, false)),
        Opcode::S2_storeri_io => req!(store_io(decoded, MemWidth::Word, immext, false)),
        Opcode::S2_storerd_io => req!(store_io(decoded, MemWidth::Double, immext, false)),
        Opcode::S2_storerbgp => req!(store_gp(decoded, MemWidth::Byte, false, immext)),
        Opcode::S2_storerhgp => req!(store_gp(decoded, MemWidth::Half, false, immext)),
        Opcode::S2_storerigp => req!(store_gp(decoded, MemWidth::Word, false, immext)),
        Opcode::S2_storerdgp => req!(store_gp(decoded, MemWidth::Double, false, immext)),
        // ---- S4 base register-offset stores `memX(Rs+Ru<<#u2)=Rt` (_rr) ----
        Opcode::S4_storerb_rr => req!(store_rr(decoded, MemWidth::Byte, false)),
        Opcode::S4_storerh_rr => req!(store_rr(decoded, MemWidth::Half, false)),
        Opcode::S4_storeri_rr => req!(store_rr(decoded, MemWidth::Word, false)),
        Opcode::S4_storerd_rr => req!(store_rr(decoded, MemWidth::Double, false)),
        Opcode::S4_storerf_rr => req!(store_rr(decoded, MemWidth::Half, true)),
        Opcode::S4_storerbnew_rr => req!(store_new_rr(decoded, MemWidth::Byte)),
        Opcode::S4_storerhnew_rr => req!(store_new_rr(decoded, MemWidth::Half)),
        Opcode::S4_storerinew_rr => req!(store_new_rr(decoded, MemWidth::Word)),
        // ---- S4 absolute-set stores `memX(Re=##U6)=Rt` (_ap) ----
        Opcode::S4_storerb_ap => req!(store_ap(decoded, MemWidth::Byte, false, immext)),
        Opcode::S4_storerh_ap => req!(store_ap(decoded, MemWidth::Half, false, immext)),
        Opcode::S4_storeri_ap => req!(store_ap(decoded, MemWidth::Word, false, immext)),
        Opcode::S4_storerd_ap => req!(store_ap(decoded, MemWidth::Double, false, immext)),
        Opcode::S4_storerf_ap => req!(store_ap(decoded, MemWidth::Half, true, immext)),
        Opcode::S4_storerbnew_ap => req!(store_new_ap(decoded, MemWidth::Byte, immext)),
        Opcode::S4_storerhnew_ap => req!(store_new_ap(decoded, MemWidth::Half, immext)),
        Opcode::S4_storerinew_ap => req!(store_new_ap(decoded, MemWidth::Word, immext)),
        // ---- S4 scaled-index absolute stores `memX(Ru<<#u2+##U6)=Rt` (_ur) ----
        Opcode::S4_storerb_ur => req!(store_ur(decoded, MemWidth::Byte, false, immext)),
        Opcode::S4_storerh_ur => req!(store_ur(decoded, MemWidth::Half, false, immext)),
        Opcode::S4_storeri_ur => req!(store_ur(decoded, MemWidth::Word, false, immext)),
        Opcode::S4_storerd_ur => req!(store_ur(decoded, MemWidth::Double, false, immext)),
        Opcode::S4_storerf_ur => req!(store_ur(decoded, MemWidth::Half, true, immext)),
        Opcode::S4_storerbnew_ur => req!(store_new_ur(decoded, MemWidth::Byte, immext)),
        Opcode::S4_storerhnew_ur => req!(store_new_ur(decoded, MemWidth::Half, immext)),
        Opcode::S4_storerinew_ur => req!(store_new_ur(decoded, MemWidth::Word, immext)),
        // ---- storerf high-half stores `memh(...)=Rt.h` (all address modes) ----
        Opcode::S2_storerf_io => req!(store_io_high(decoded, immext)),
        Opcode::S2_storerf_pi => req!(store_high_pi(decoded, MemWidth::Half)),
        Opcode::S2_storerf_pr => req!(store_high_pr(decoded, MemWidth::Half, false)),
        Opcode::S2_storerf_pbr => req!(store_high_pr(decoded, MemWidth::Half, true)),
        Opcode::S2_storerf_pci => req!(store_high_pci(decoded, MemWidth::Half)),
        Opcode::S2_storerf_pcr => req!(store_high_pcr(decoded, MemWidth::Half)),
        // ---- GP-relative high-half + new-value stores ----
        Opcode::S2_storerfgp => req!(store_high_gp(decoded, MemWidth::Half, immext)),
        Opcode::S2_storerbnewgp => req!(store_new_gp(decoded, MemWidth::Byte, immext)),
        Opcode::S2_storerhnewgp => req!(store_new_gp(decoded, MemWidth::Half, immext)),
        Opcode::S2_storerinewgp => req!(store_new_gp(decoded, MemWidth::Word, immext)),
        // ---- store-conditional (locked) and store-release (rl) ----
        Opcode::S2_storew_locked => req!(store_cond(decoded, MemWidth::Word, true)),
        Opcode::S4_stored_locked => req!(store_cond(decoded, MemWidth::Double, true)),
        Opcode::S2_storew_rl_at_vi | Opcode::S2_storew_rl_st_vi => {
            req!(store_cond(decoded, MemWidth::Word, false))
        }
        Opcode::S4_stored_rl_at_vi | Opcode::S4_stored_rl_st_vi => {
            req!(store_cond(decoded, MemWidth::Double, false))
        }
        // ---- vector byte splice (register-only) ----
        Opcode::S2_vspliceib => req!(vsplice(decoded, true)),
        Opcode::S2_vsplicerb => req!(vsplice(decoded, false)),
        Opcode::S2_storerbnew_io => req!(store_new_io(decoded, MemWidth::Byte, immext)),
        Opcode::S2_storerhnew_io => req!(store_new_io(decoded, MemWidth::Half, immext)),
        Opcode::S2_storerinew_io => req!(store_new_io(decoded, MemWidth::Word, immext)),
        Opcode::S2_storerb_pi => req!(store_pi(decoded, MemWidth::Byte, false)),
        Opcode::S2_storerh_pi => req!(store_pi(decoded, MemWidth::Half, false)),
        Opcode::S2_storeri_pi => req!(store_pi(decoded, MemWidth::Word, false)),
        Opcode::S2_storerd_pi => req!(store_pi(decoded, MemWidth::Double, false)),
        // ---- register / bit-reverse / circular post-increment loads ----
        Opcode::L2_loadrb_pr => req!(load_pr(decoded, MemWidth::Byte, MemSign::Signed)),
        Opcode::L2_loadrub_pr => req!(load_pr(decoded, MemWidth::Byte, MemSign::Unsigned)),
        Opcode::L2_loadrh_pr => req!(load_pr(decoded, MemWidth::Half, MemSign::Signed)),
        Opcode::L2_loadruh_pr => req!(load_pr(decoded, MemWidth::Half, MemSign::Unsigned)),
        Opcode::L2_loadri_pr => req!(load_pr(decoded, MemWidth::Word, MemSign::Unsigned)),
        Opcode::L2_loadrd_pr => req!(load_pr(decoded, MemWidth::Double, MemSign::Unsigned)),
        Opcode::L2_loadrb_pbr => req!(load_pbr(decoded, MemWidth::Byte, MemSign::Signed)),
        Opcode::L2_loadrub_pbr => req!(load_pbr(decoded, MemWidth::Byte, MemSign::Unsigned)),
        Opcode::L2_loadrh_pbr => req!(load_pbr(decoded, MemWidth::Half, MemSign::Signed)),
        Opcode::L2_loadruh_pbr => req!(load_pbr(decoded, MemWidth::Half, MemSign::Unsigned)),
        Opcode::L2_loadri_pbr => req!(load_pbr(decoded, MemWidth::Word, MemSign::Unsigned)),
        Opcode::L2_loadrd_pbr => req!(load_pbr(decoded, MemWidth::Double, MemSign::Unsigned)),
        Opcode::L2_loadrb_pci => req!(load_pci(decoded, MemWidth::Byte, MemSign::Signed)),
        Opcode::L2_loadrub_pci => req!(load_pci(decoded, MemWidth::Byte, MemSign::Unsigned)),
        Opcode::L2_loadrh_pci => req!(load_pci(decoded, MemWidth::Half, MemSign::Signed)),
        Opcode::L2_loadruh_pci => req!(load_pci(decoded, MemWidth::Half, MemSign::Unsigned)),
        Opcode::L2_loadri_pci => req!(load_pci(decoded, MemWidth::Word, MemSign::Unsigned)),
        Opcode::L2_loadrd_pci => req!(load_pci(decoded, MemWidth::Double, MemSign::Unsigned)),
        Opcode::L2_loadrb_pcr => req!(load_pcr(decoded, MemWidth::Byte, MemSign::Signed)),
        Opcode::L2_loadrub_pcr => req!(load_pcr(decoded, MemWidth::Byte, MemSign::Unsigned)),
        Opcode::L2_loadrh_pcr => req!(load_pcr(decoded, MemWidth::Half, MemSign::Signed)),
        Opcode::L2_loadruh_pcr => req!(load_pcr(decoded, MemWidth::Half, MemSign::Unsigned)),
        Opcode::L2_loadri_pcr => req!(load_pcr(decoded, MemWidth::Word, MemSign::Unsigned)),
        Opcode::L2_loadrd_pcr => req!(load_pcr(decoded, MemWidth::Double, MemSign::Unsigned)),
        // ---- register / bit-reverse / circular post-increment stores ----
        Opcode::S2_storerb_pr => req!(store_pr(decoded, MemWidth::Byte, false)),
        Opcode::S2_storerh_pr => req!(store_pr(decoded, MemWidth::Half, false)),
        Opcode::S2_storeri_pr => req!(store_pr(decoded, MemWidth::Word, false)),
        Opcode::S2_storerd_pr => req!(store_pr(decoded, MemWidth::Double, false)),
        Opcode::S2_storerb_pbr => req!(store_pbr(decoded, MemWidth::Byte, false)),
        Opcode::S2_storerh_pbr => req!(store_pbr(decoded, MemWidth::Half, false)),
        Opcode::S2_storeri_pbr => req!(store_pbr(decoded, MemWidth::Word, false)),
        Opcode::S2_storerd_pbr => req!(store_pbr(decoded, MemWidth::Double, false)),
        Opcode::S2_storerb_pci => req!(store_pci(decoded, MemWidth::Byte, false)),
        Opcode::S2_storerh_pci => req!(store_pci(decoded, MemWidth::Half, false)),
        Opcode::S2_storeri_pci => req!(store_pci(decoded, MemWidth::Word, false)),
        Opcode::S2_storerd_pci => req!(store_pci(decoded, MemWidth::Double, false)),
        Opcode::S2_storerb_pcr => req!(store_pcr(decoded, MemWidth::Byte, false)),
        Opcode::S2_storerh_pcr => req!(store_pcr(decoded, MemWidth::Half, false)),
        Opcode::S2_storeri_pcr => req!(store_pcr(decoded, MemWidth::Word, false)),
        Opcode::S2_storerd_pcr => req!(store_pcr(decoded, MemWidth::Double, false)),
        // ---- new-value post-increment stores (register/brev/circular) ----
        Opcode::S2_storerbnew_pi => req!(store_new_pi(decoded, MemWidth::Byte)),
        Opcode::S2_storerhnew_pi => req!(store_new_pi(decoded, MemWidth::Half)),
        Opcode::S2_storerinew_pi => req!(store_new_pi(decoded, MemWidth::Word)),
        Opcode::S2_storerbnew_pr => req!(store_new_pr(decoded, MemWidth::Byte)),
        Opcode::S2_storerhnew_pr => req!(store_new_pr(decoded, MemWidth::Half)),
        Opcode::S2_storerinew_pr => req!(store_new_pr(decoded, MemWidth::Word)),
        Opcode::S2_storerbnew_pbr => req!(store_new_pbr(decoded, MemWidth::Byte)),
        Opcode::S2_storerhnew_pbr => req!(store_new_pbr(decoded, MemWidth::Half)),
        Opcode::S2_storerinew_pbr => req!(store_new_pbr(decoded, MemWidth::Word)),
        Opcode::S2_storerbnew_pci => req!(store_new_pci(decoded, MemWidth::Byte)),
        Opcode::S2_storerhnew_pci => req!(store_new_pci(decoded, MemWidth::Half)),
        Opcode::S2_storerinew_pci => req!(store_new_pci(decoded, MemWidth::Word)),
        Opcode::S2_storerbnew_pcr => req!(store_new_pcr(decoded, MemWidth::Byte)),
        Opcode::S2_storerhnew_pcr => req!(store_new_pcr(decoded, MemWidth::Half)),
        Opcode::S2_storerinew_pcr => req!(store_new_pcr(decoded, MemWidth::Word)),
        Opcode::L2_ploadrbt_io
        | Opcode::L2_ploadrbf_io
        | Opcode::L2_ploadrbtnew_io
        | Opcode::L2_ploadrbfnew_io => {
            let (sense, pred_new) = match decoded.opcode {
                Opcode::L2_ploadrbt_io => (true, false),
                Opcode::L2_ploadrbf_io => (false, false),
                Opcode::L2_ploadrbtnew_io => (true, true),
                _ => (false, true),
            };
            req!(pred_load_io(
                decoded,
                MemWidth::Byte,
                MemSign::Signed,
                sense,
                pred_new
            ))
        }
        Opcode::L2_ploadrubt_io
        | Opcode::L2_ploadrubf_io
        | Opcode::L2_ploadrubtnew_io
        | Opcode::L2_ploadrubfnew_io => {
            let (sense, pred_new) = match decoded.opcode {
                Opcode::L2_ploadrubt_io => (true, false),
                Opcode::L2_ploadrubf_io => (false, false),
                Opcode::L2_ploadrubtnew_io => (true, true),
                _ => (false, true),
            };
            req!(pred_load_io(
                decoded,
                MemWidth::Byte,
                MemSign::Unsigned,
                sense,
                pred_new
            ))
        }
        Opcode::L2_ploadrht_io
        | Opcode::L2_ploadrhf_io
        | Opcode::L2_ploadrhtnew_io
        | Opcode::L2_ploadrhfnew_io => {
            let (sense, pred_new) = match decoded.opcode {
                Opcode::L2_ploadrht_io => (true, false),
                Opcode::L2_ploadrhf_io => (false, false),
                Opcode::L2_ploadrhtnew_io => (true, true),
                _ => (false, true),
            };
            req!(pred_load_io(
                decoded,
                MemWidth::Half,
                MemSign::Signed,
                sense,
                pred_new
            ))
        }
        Opcode::L2_ploadruht_io
        | Opcode::L2_ploadruhf_io
        | Opcode::L2_ploadruhtnew_io
        | Opcode::L2_ploadruhfnew_io => {
            let (sense, pred_new) = match decoded.opcode {
                Opcode::L2_ploadruht_io => (true, false),
                Opcode::L2_ploadruhf_io => (false, false),
                Opcode::L2_ploadruhtnew_io => (true, true),
                _ => (false, true),
            };
            req!(pred_load_io(
                decoded,
                MemWidth::Half,
                MemSign::Unsigned,
                sense,
                pred_new
            ))
        }
        Opcode::L2_ploadrit_io
        | Opcode::L2_ploadrif_io
        | Opcode::L2_ploadritnew_io
        | Opcode::L2_ploadrifnew_io => {
            let (sense, pred_new) = match decoded.opcode {
                Opcode::L2_ploadrit_io => (true, false),
                Opcode::L2_ploadrif_io => (false, false),
                Opcode::L2_ploadritnew_io => (true, true),
                _ => (false, true),
            };
            req!(pred_load_io(
                decoded,
                MemWidth::Word,
                MemSign::Unsigned,
                sense,
                pred_new
            ))
        }
        Opcode::L2_ploadrdt_io
        | Opcode::L2_ploadrdf_io
        | Opcode::L2_ploadrdtnew_io
        | Opcode::L2_ploadrdfnew_io => {
            let (sense, pred_new) = match decoded.opcode {
                Opcode::L2_ploadrdt_io => (true, false),
                Opcode::L2_ploadrdf_io => (false, false),
                Opcode::L2_ploadrdtnew_io => (true, true),
                _ => (false, true),
            };
            req!(pred_load_io(
                decoded,
                MemWidth::Double,
                MemSign::Unsigned,
                sense,
                pred_new
            ))
        }
        // ---- predicated post-increment loads (`L2_ploadr*_pi`) ----
        Opcode::L2_ploadrbt_pi
        | Opcode::L2_ploadrbf_pi
        | Opcode::L2_ploadrbtnew_pi
        | Opcode::L2_ploadrbfnew_pi => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_pi(decoded, MemWidth::Byte, MemSign::Signed, s, n))
        }
        Opcode::L2_ploadrubt_pi
        | Opcode::L2_ploadrubf_pi
        | Opcode::L2_ploadrubtnew_pi
        | Opcode::L2_ploadrubfnew_pi => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_pi(
                decoded,
                MemWidth::Byte,
                MemSign::Unsigned,
                s,
                n
            ))
        }
        Opcode::L2_ploadrht_pi
        | Opcode::L2_ploadrhf_pi
        | Opcode::L2_ploadrhtnew_pi
        | Opcode::L2_ploadrhfnew_pi => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_pi(decoded, MemWidth::Half, MemSign::Signed, s, n))
        }
        Opcode::L2_ploadruht_pi
        | Opcode::L2_ploadruhf_pi
        | Opcode::L2_ploadruhtnew_pi
        | Opcode::L2_ploadruhfnew_pi => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_pi(
                decoded,
                MemWidth::Half,
                MemSign::Unsigned,
                s,
                n
            ))
        }
        Opcode::L2_ploadrit_pi
        | Opcode::L2_ploadrif_pi
        | Opcode::L2_ploadritnew_pi
        | Opcode::L2_ploadrifnew_pi => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_pi(
                decoded,
                MemWidth::Word,
                MemSign::Unsigned,
                s,
                n
            ))
        }
        Opcode::L2_ploadrdt_pi
        | Opcode::L2_ploadrdf_pi
        | Opcode::L2_ploadrdtnew_pi
        | Opcode::L2_ploadrdfnew_pi => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_pi(
                decoded,
                MemWidth::Double,
                MemSign::Unsigned,
                s,
                n
            ))
        }
        // ---- predicated register-offset loads (`L4_ploadr*_rr`) ----
        Opcode::L4_ploadrbt_rr
        | Opcode::L4_ploadrbf_rr
        | Opcode::L4_ploadrbtnew_rr
        | Opcode::L4_ploadrbfnew_rr => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_rr(decoded, MemWidth::Byte, MemSign::Signed, s, n))
        }
        Opcode::L4_ploadrubt_rr
        | Opcode::L4_ploadrubf_rr
        | Opcode::L4_ploadrubtnew_rr
        | Opcode::L4_ploadrubfnew_rr => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_rr(
                decoded,
                MemWidth::Byte,
                MemSign::Unsigned,
                s,
                n
            ))
        }
        Opcode::L4_ploadrht_rr
        | Opcode::L4_ploadrhf_rr
        | Opcode::L4_ploadrhtnew_rr
        | Opcode::L4_ploadrhfnew_rr => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_rr(decoded, MemWidth::Half, MemSign::Signed, s, n))
        }
        Opcode::L4_ploadruht_rr
        | Opcode::L4_ploadruhf_rr
        | Opcode::L4_ploadruhtnew_rr
        | Opcode::L4_ploadruhfnew_rr => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_rr(
                decoded,
                MemWidth::Half,
                MemSign::Unsigned,
                s,
                n
            ))
        }
        Opcode::L4_ploadrit_rr
        | Opcode::L4_ploadrif_rr
        | Opcode::L4_ploadritnew_rr
        | Opcode::L4_ploadrifnew_rr => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_rr(
                decoded,
                MemWidth::Word,
                MemSign::Unsigned,
                s,
                n
            ))
        }
        Opcode::L4_ploadrdt_rr
        | Opcode::L4_ploadrdf_rr
        | Opcode::L4_ploadrdtnew_rr
        | Opcode::L4_ploadrdfnew_rr => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_rr(
                decoded,
                MemWidth::Double,
                MemSign::Unsigned,
                s,
                n
            ))
        }
        // ---- predicated absolute loads (`L4_ploadr*_abs`) ----
        Opcode::L4_ploadrbt_abs
        | Opcode::L4_ploadrbf_abs
        | Opcode::L4_ploadrbtnew_abs
        | Opcode::L4_ploadrbfnew_abs => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_abs(
                decoded,
                MemWidth::Byte,
                MemSign::Signed,
                s,
                n,
                immext
            ))
        }
        Opcode::L4_ploadrubt_abs
        | Opcode::L4_ploadrubf_abs
        | Opcode::L4_ploadrubtnew_abs
        | Opcode::L4_ploadrubfnew_abs => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_abs(
                decoded,
                MemWidth::Byte,
                MemSign::Unsigned,
                s,
                n,
                immext
            ))
        }
        Opcode::L4_ploadrht_abs
        | Opcode::L4_ploadrhf_abs
        | Opcode::L4_ploadrhtnew_abs
        | Opcode::L4_ploadrhfnew_abs => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_abs(
                decoded,
                MemWidth::Half,
                MemSign::Signed,
                s,
                n,
                immext
            ))
        }
        Opcode::L4_ploadruht_abs
        | Opcode::L4_ploadruhf_abs
        | Opcode::L4_ploadruhtnew_abs
        | Opcode::L4_ploadruhfnew_abs => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_abs(
                decoded,
                MemWidth::Half,
                MemSign::Unsigned,
                s,
                n,
                immext
            ))
        }
        Opcode::L4_ploadrit_abs
        | Opcode::L4_ploadrif_abs
        | Opcode::L4_ploadritnew_abs
        | Opcode::L4_ploadrifnew_abs => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_abs(
                decoded,
                MemWidth::Word,
                MemSign::Unsigned,
                s,
                n,
                immext
            ))
        }
        Opcode::L4_ploadrdt_abs
        | Opcode::L4_ploadrdf_abs
        | Opcode::L4_ploadrdtnew_abs
        | Opcode::L4_ploadrdfnew_abs => {
            let (s, n) = pload_cond(decoded.opcode);
            req!(pred_load_abs(
                decoded,
                MemWidth::Double,
                MemSign::Unsigned,
                s,
                n,
                immext
            ))
        }
        // ---- base L4 loads: register-offset / scaled-abs / abs-set ----
        Opcode::L4_loadrb_rr => req!(load_rr(decoded, MemWidth::Byte, MemSign::Signed)),
        Opcode::L4_loadrub_rr => req!(load_rr(decoded, MemWidth::Byte, MemSign::Unsigned)),
        Opcode::L4_loadrh_rr => req!(load_rr(decoded, MemWidth::Half, MemSign::Signed)),
        Opcode::L4_loadruh_rr => req!(load_rr(decoded, MemWidth::Half, MemSign::Unsigned)),
        Opcode::L4_loadri_rr => req!(load_rr(decoded, MemWidth::Word, MemSign::Unsigned)),
        Opcode::L4_loadrd_rr => req!(load_rr(decoded, MemWidth::Double, MemSign::Unsigned)),
        Opcode::L4_loadrb_ur => req!(load_simple(
            decoded,
            MemWidth::Byte,
            MemSign::Signed,
            req!(addr_ur(decoded, immext))
        )),
        Opcode::L4_loadrub_ur => req!(load_simple(
            decoded,
            MemWidth::Byte,
            MemSign::Unsigned,
            req!(addr_ur(decoded, immext))
        )),
        Opcode::L4_loadrh_ur => req!(load_simple(
            decoded,
            MemWidth::Half,
            MemSign::Signed,
            req!(addr_ur(decoded, immext))
        )),
        Opcode::L4_loadruh_ur => req!(load_simple(
            decoded,
            MemWidth::Half,
            MemSign::Unsigned,
            req!(addr_ur(decoded, immext))
        )),
        Opcode::L4_loadri_ur => req!(load_simple(
            decoded,
            MemWidth::Word,
            MemSign::Unsigned,
            req!(addr_ur(decoded, immext))
        )),
        Opcode::L4_loadrd_ur => req!(load_simple(
            decoded,
            MemWidth::Double,
            MemSign::Unsigned,
            req!(addr_ur(decoded, immext))
        )),
        Opcode::L4_loadrb_ap => req!(load_simple(
            decoded,
            MemWidth::Byte,
            MemSign::Signed,
            req!(addr_ap(decoded, immext))
        )),
        Opcode::L4_loadrub_ap => req!(load_simple(
            decoded,
            MemWidth::Byte,
            MemSign::Unsigned,
            req!(addr_ap(decoded, immext))
        )),
        Opcode::L4_loadrh_ap => req!(load_simple(
            decoded,
            MemWidth::Half,
            MemSign::Signed,
            req!(addr_ap(decoded, immext))
        )),
        Opcode::L4_loadruh_ap => req!(load_simple(
            decoded,
            MemWidth::Half,
            MemSign::Unsigned,
            req!(addr_ap(decoded, immext))
        )),
        Opcode::L4_loadri_ap => req!(load_simple(
            decoded,
            MemWidth::Word,
            MemSign::Unsigned,
            req!(addr_ap(decoded, immext))
        )),
        Opcode::L4_loadrd_ap => req!(load_simple(
            decoded,
            MemWidth::Double,
            MemSign::Unsigned,
            req!(addr_ap(decoded, immext))
        )),
        // ---- atomic loads (acquire == plain load; locked sets a reservation) ----
        Opcode::L2_loadw_locked => req!(load_atomic(decoded, MemWidth::Word, true)),
        Opcode::L2_loadw_aq => req!(load_atomic(decoded, MemWidth::Word, false)),
        Opcode::L4_loadd_locked => req!(load_atomic(decoded, MemWidth::Double, true)),
        Opcode::L4_loadd_aq => req!(load_atomic(decoded, MemWidth::Double, false)),
        // ---- LOADALIGN (FIFO) — byte (shift 8) ----
        Opcode::L2_loadalignb_io => {
            let a = req!(addr_io_n(decoded, 0, immext));
            req!(loadalign(decoded, a.0, MemWidth::Byte, a.1))
        }
        Opcode::L2_loadalignb_pi => {
            let a = req!(addr_pi_n(decoded, 0));
            req!(loadalign(decoded, a, MemWidth::Byte, false))
        }
        Opcode::L2_loadalignb_pr => req!(loadalign(
            decoded,
            req!(addr_pr(decoded)),
            MemWidth::Byte,
            false
        )),
        Opcode::L2_loadalignb_pbr => req!(loadalign(
            decoded,
            req!(addr_pbr(decoded)),
            MemWidth::Byte,
            false
        )),
        Opcode::L2_loadalignb_pci => req!(loadalign(
            decoded,
            req!(addr_pci_n(decoded, 0)),
            MemWidth::Byte,
            false
        )),
        Opcode::L2_loadalignb_pcr => req!(loadalign(
            decoded,
            req!(addr_pcr_n(decoded, 0)),
            MemWidth::Byte,
            false
        )),
        Opcode::L4_loadalignb_ap => {
            let a = req!(addr_ap(decoded, immext));
            req!(loadalign(decoded, a.0, MemWidth::Byte, a.1))
        }
        Opcode::L4_loadalignb_ur => {
            let a = req!(addr_ur(decoded, immext));
            req!(loadalign(decoded, a.0, MemWidth::Byte, a.1))
        }
        // ---- LOADALIGN (FIFO) — half (shift 16, :1 immediate scale) ----
        Opcode::L2_loadalignh_io => {
            let a = req!(addr_io_n(decoded, 1, immext));
            req!(loadalign(decoded, a.0, MemWidth::Half, a.1))
        }
        Opcode::L2_loadalignh_pi => {
            let a = req!(addr_pi_n(decoded, 1));
            req!(loadalign(decoded, a, MemWidth::Half, false))
        }
        Opcode::L2_loadalignh_pr => req!(loadalign(
            decoded,
            req!(addr_pr(decoded)),
            MemWidth::Half,
            false
        )),
        Opcode::L2_loadalignh_pbr => req!(loadalign(
            decoded,
            req!(addr_pbr(decoded)),
            MemWidth::Half,
            false
        )),
        Opcode::L2_loadalignh_pci => req!(loadalign(
            decoded,
            req!(addr_pci_n(decoded, 1)),
            MemWidth::Half,
            false
        )),
        Opcode::L2_loadalignh_pcr => req!(loadalign(
            decoded,
            req!(addr_pcr_n(decoded, 1)),
            MemWidth::Half,
            false
        )),
        Opcode::L4_loadalignh_ap => {
            let a = req!(addr_ap(decoded, immext));
            req!(loadalign(decoded, a.0, MemWidth::Half, a.1))
        }
        Opcode::L4_loadalignh_ur => {
            let a = req!(addr_ur(decoded, immext));
            req!(loadalign(decoded, a.0, MemWidth::Half, a.1))
        }
        // ---- BSW (membh, sign-extend unpack) — 2 bytes (:1 scale) ----
        Opcode::L2_loadbsw2_io => {
            let a = req!(addr_io_n(decoded, 1, immext));
            req!(loadunpack(decoded, a.0, 2, MemSign::Signed, a.1))
        }
        Opcode::L2_loadbsw2_pi => req!(loadunpack(
            decoded,
            req!(addr_pi_n(decoded, 1)),
            2,
            MemSign::Signed,
            false
        )),
        Opcode::L2_loadbsw2_pr => req!(loadunpack(
            decoded,
            req!(addr_pr(decoded)),
            2,
            MemSign::Signed,
            false
        )),
        Opcode::L2_loadbsw2_pbr => req!(loadunpack(
            decoded,
            req!(addr_pbr(decoded)),
            2,
            MemSign::Signed,
            false
        )),
        Opcode::L2_loadbsw2_pci => req!(loadunpack(
            decoded,
            req!(addr_pci_n(decoded, 1)),
            2,
            MemSign::Signed,
            false
        )),
        Opcode::L2_loadbsw2_pcr => req!(loadunpack(
            decoded,
            req!(addr_pcr_n(decoded, 1)),
            2,
            MemSign::Signed,
            false
        )),
        Opcode::L4_loadbsw2_ap => {
            let a = req!(addr_ap(decoded, immext));
            req!(loadunpack(decoded, a.0, 2, MemSign::Signed, a.1))
        }
        Opcode::L4_loadbsw2_ur => {
            let a = req!(addr_ur(decoded, immext));
            req!(loadunpack(decoded, a.0, 2, MemSign::Signed, a.1))
        }
        // ---- BSW (membh) — 4 bytes (:2 scale, Rdd pair) ----
        Opcode::L2_loadbsw4_io => {
            let a = req!(addr_io_n(decoded, 2, immext));
            req!(loadunpack(decoded, a.0, 4, MemSign::Signed, a.1))
        }
        Opcode::L2_loadbsw4_pi => req!(loadunpack(
            decoded,
            req!(addr_pi_n(decoded, 2)),
            4,
            MemSign::Signed,
            false
        )),
        Opcode::L2_loadbsw4_pr => req!(loadunpack(
            decoded,
            req!(addr_pr(decoded)),
            4,
            MemSign::Signed,
            false
        )),
        Opcode::L2_loadbsw4_pbr => req!(loadunpack(
            decoded,
            req!(addr_pbr(decoded)),
            4,
            MemSign::Signed,
            false
        )),
        Opcode::L2_loadbsw4_pci => req!(loadunpack(
            decoded,
            req!(addr_pci_n(decoded, 2)),
            4,
            MemSign::Signed,
            false
        )),
        Opcode::L2_loadbsw4_pcr => req!(loadunpack(
            decoded,
            req!(addr_pcr_n(decoded, 2)),
            4,
            MemSign::Signed,
            false
        )),
        Opcode::L4_loadbsw4_ap => {
            let a = req!(addr_ap(decoded, immext));
            req!(loadunpack(decoded, a.0, 4, MemSign::Signed, a.1))
        }
        Opcode::L4_loadbsw4_ur => {
            let a = req!(addr_ur(decoded, immext));
            req!(loadunpack(decoded, a.0, 4, MemSign::Signed, a.1))
        }
        // ---- BZW (memubh, zero-extend unpack) — 2 bytes (:1 scale) ----
        Opcode::L2_loadbzw2_io => {
            let a = req!(addr_io_n(decoded, 1, immext));
            req!(loadunpack(decoded, a.0, 2, MemSign::Unsigned, a.1))
        }
        Opcode::L2_loadbzw2_pi => req!(loadunpack(
            decoded,
            req!(addr_pi_n(decoded, 1)),
            2,
            MemSign::Unsigned,
            false
        )),
        Opcode::L2_loadbzw2_pr => req!(loadunpack(
            decoded,
            req!(addr_pr(decoded)),
            2,
            MemSign::Unsigned,
            false
        )),
        Opcode::L2_loadbzw2_pbr => req!(loadunpack(
            decoded,
            req!(addr_pbr(decoded)),
            2,
            MemSign::Unsigned,
            false
        )),
        Opcode::L2_loadbzw2_pci => req!(loadunpack(
            decoded,
            req!(addr_pci_n(decoded, 1)),
            2,
            MemSign::Unsigned,
            false
        )),
        Opcode::L2_loadbzw2_pcr => req!(loadunpack(
            decoded,
            req!(addr_pcr_n(decoded, 1)),
            2,
            MemSign::Unsigned,
            false
        )),
        Opcode::L4_loadbzw2_ap => {
            let a = req!(addr_ap(decoded, immext));
            req!(loadunpack(decoded, a.0, 2, MemSign::Unsigned, a.1))
        }
        Opcode::L4_loadbzw2_ur => {
            let a = req!(addr_ur(decoded, immext));
            req!(loadunpack(decoded, a.0, 2, MemSign::Unsigned, a.1))
        }
        // ---- BZW (memubh) — 4 bytes (:2 scale, Rdd pair) ----
        Opcode::L2_loadbzw4_io => {
            let a = req!(addr_io_n(decoded, 2, immext));
            req!(loadunpack(decoded, a.0, 4, MemSign::Unsigned, a.1))
        }
        Opcode::L2_loadbzw4_pi => req!(loadunpack(
            decoded,
            req!(addr_pi_n(decoded, 2)),
            4,
            MemSign::Unsigned,
            false
        )),
        Opcode::L2_loadbzw4_pr => req!(loadunpack(
            decoded,
            req!(addr_pr(decoded)),
            4,
            MemSign::Unsigned,
            false
        )),
        Opcode::L2_loadbzw4_pbr => req!(loadunpack(
            decoded,
            req!(addr_pbr(decoded)),
            4,
            MemSign::Unsigned,
            false
        )),
        Opcode::L2_loadbzw4_pci => req!(loadunpack(
            decoded,
            req!(addr_pci_n(decoded, 2)),
            4,
            MemSign::Unsigned,
            false
        )),
        Opcode::L2_loadbzw4_pcr => req!(loadunpack(
            decoded,
            req!(addr_pcr_n(decoded, 2)),
            4,
            MemSign::Unsigned,
            false
        )),
        Opcode::L4_loadbzw4_ap => {
            let a = req!(addr_ap(decoded, immext));
            req!(loadunpack(decoded, a.0, 4, MemSign::Unsigned, a.1))
        }
        Opcode::L4_loadbzw4_ur => {
            let a = req!(addr_ur(decoded, immext));
            req!(loadunpack(decoded, a.0, 4, MemSign::Unsigned, a.1))
        }
        Opcode::S2_pstorerbt_io | Opcode::S2_pstorerbf_io => {
            let sense = matches!(decoded.opcode, Opcode::S2_pstorerbt_io);
            req!(pred_store_io(
                decoded,
                MemWidth::Byte,
                sense,
                false,
                false,
                false
            ))
        }
        Opcode::S2_pstorerbnewt_io | Opcode::S2_pstorerbnewf_io => {
            let sense = matches!(decoded.opcode, Opcode::S2_pstorerbnewt_io);
            req!(pred_store_new_io(decoded, MemWidth::Byte, sense, false))
        }
        Opcode::S2_pstorerht_io | Opcode::S2_pstorerhf_io => {
            let sense = matches!(decoded.opcode, Opcode::S2_pstorerht_io);
            req!(pred_store_io(
                decoded,
                MemWidth::Half,
                sense,
                false,
                false,
                false
            ))
        }
        Opcode::S2_pstorerhnewt_io | Opcode::S2_pstorerhnewf_io => {
            let sense = matches!(decoded.opcode, Opcode::S2_pstorerhnewt_io);
            req!(pred_store_new_io(decoded, MemWidth::Half, sense, false))
        }
        Opcode::S2_pstorerit_io | Opcode::S2_pstorerif_io => {
            let sense = matches!(decoded.opcode, Opcode::S2_pstorerit_io);
            req!(pred_store_io(
                decoded,
                MemWidth::Word,
                sense,
                false,
                false,
                false
            ))
        }
        Opcode::S2_pstorerinewt_io | Opcode::S2_pstorerinewf_io => {
            let sense = matches!(decoded.opcode, Opcode::S2_pstorerinewt_io);
            req!(pred_store_new_io(decoded, MemWidth::Word, sense, false))
        }
        Opcode::S2_pstorerdt_io | Opcode::S2_pstorerdf_io => {
            let sense = matches!(decoded.opcode, Opcode::S2_pstorerdt_io);
            req!(pred_store_io(
                decoded,
                MemWidth::Double,
                sense,
                false,
                false,
                false
            ))
        }
        // ---- storerf (high-half) predicated _io: stores Rt[31:16] as a half ----
        Opcode::S2_pstorerft_io | Opcode::S2_pstorerff_io => {
            let sense = matches!(decoded.opcode, Opcode::S2_pstorerft_io);
            req!(pred_store_io(
                decoded,
                MemWidth::Half,
                sense,
                false,
                false,
                true
            ))
        }
        // ---- S4 _io predicated stores with a .new predicate (tnew/fnew) ----
        Opcode::S4_pstorerbtnew_io | Opcode::S4_pstorerbfnew_io => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_io(
                decoded,
                MemWidth::Byte,
                sense,
                pred_new,
                false,
                false
            ))
        }
        Opcode::S4_pstorerhtnew_io | Opcode::S4_pstorerhfnew_io => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_io(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                false,
                false
            ))
        }
        Opcode::S4_pstoreritnew_io | Opcode::S4_pstorerifnew_io => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_io(
                decoded,
                MemWidth::Word,
                sense,
                pred_new,
                false,
                false
            ))
        }
        Opcode::S4_pstorerdtnew_io | Opcode::S4_pstorerdfnew_io => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_io(
                decoded,
                MemWidth::Double,
                sense,
                pred_new,
                false,
                false
            ))
        }
        Opcode::S4_pstorerftnew_io | Opcode::S4_pstorerffnew_io => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_io(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                false,
                true
            ))
        }
        // S4 new-value _io stores with a .new predicate.
        Opcode::S4_pstorerbnewtnew_io | Opcode::S4_pstorerbnewfnew_io => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_io(decoded, MemWidth::Byte, sense, pred_new))
        }
        Opcode::S4_pstorerhnewtnew_io | Opcode::S4_pstorerhnewfnew_io => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_io(decoded, MemWidth::Half, sense, pred_new))
        }
        Opcode::S4_pstorerinewtnew_io | Opcode::S4_pstorerinewfnew_io => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_io(decoded, MemWidth::Word, sense, pred_new))
        }
        // ================================================================
        // Predicated post-increment stores (S2_pstorer*_pi / *new_pi)
        // ================================================================
        Opcode::S2_pstorerbt_pi
        | Opcode::S2_pstorerbf_pi
        | Opcode::S2_pstorerbtnew_pi
        | Opcode::S2_pstorerbfnew_pi => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_pi(
                decoded,
                MemWidth::Byte,
                sense,
                pred_new,
                false
            ))
        }
        Opcode::S2_pstorerht_pi
        | Opcode::S2_pstorerhf_pi
        | Opcode::S2_pstorerhtnew_pi
        | Opcode::S2_pstorerhfnew_pi => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_pi(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                false
            ))
        }
        Opcode::S2_pstorerit_pi
        | Opcode::S2_pstorerif_pi
        | Opcode::S2_pstoreritnew_pi
        | Opcode::S2_pstorerifnew_pi => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_pi(
                decoded,
                MemWidth::Word,
                sense,
                pred_new,
                false
            ))
        }
        Opcode::S2_pstorerdt_pi
        | Opcode::S2_pstorerdf_pi
        | Opcode::S2_pstorerdtnew_pi
        | Opcode::S2_pstorerdfnew_pi => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_pi(
                decoded,
                MemWidth::Double,
                sense,
                pred_new,
                false
            ))
        }
        // storerf post-increment (high half): Rt[31:16] as a halfword.
        Opcode::S2_pstorerft_pi
        | Opcode::S2_pstorerff_pi
        | Opcode::S2_pstorerftnew_pi
        | Opcode::S2_pstorerffnew_pi => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_pi(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                true
            ))
        }
        // new-value post-increment (storerXnew_pi): source is the Nt8 producer.
        Opcode::S2_pstorerbnewt_pi
        | Opcode::S2_pstorerbnewf_pi
        | Opcode::S2_pstorerbnewtnew_pi
        | Opcode::S2_pstorerbnewfnew_pi => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_pi(decoded, MemWidth::Byte, sense, pred_new))
        }
        Opcode::S2_pstorerhnewt_pi
        | Opcode::S2_pstorerhnewf_pi
        | Opcode::S2_pstorerhnewtnew_pi
        | Opcode::S2_pstorerhnewfnew_pi => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_pi(decoded, MemWidth::Half, sense, pred_new))
        }
        Opcode::S2_pstorerinewt_pi
        | Opcode::S2_pstorerinewf_pi
        | Opcode::S2_pstorerinewtnew_pi
        | Opcode::S2_pstorerinewfnew_pi => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_pi(decoded, MemWidth::Word, sense, pred_new))
        }
        // ================================================================
        // Predicated register-offset stores (S4_pstorer*_rr / *new_rr)
        // ================================================================
        Opcode::S4_pstorerbt_rr
        | Opcode::S4_pstorerbf_rr
        | Opcode::S4_pstorerbtnew_rr
        | Opcode::S4_pstorerbfnew_rr => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_rr(
                decoded,
                MemWidth::Byte,
                sense,
                pred_new,
                false
            ))
        }
        Opcode::S4_pstorerht_rr
        | Opcode::S4_pstorerhf_rr
        | Opcode::S4_pstorerhtnew_rr
        | Opcode::S4_pstorerhfnew_rr => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_rr(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                false
            ))
        }
        Opcode::S4_pstorerit_rr
        | Opcode::S4_pstorerif_rr
        | Opcode::S4_pstoreritnew_rr
        | Opcode::S4_pstorerifnew_rr => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_rr(
                decoded,
                MemWidth::Word,
                sense,
                pred_new,
                false
            ))
        }
        Opcode::S4_pstorerdt_rr
        | Opcode::S4_pstorerdf_rr
        | Opcode::S4_pstorerdtnew_rr
        | Opcode::S4_pstorerdfnew_rr => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_rr(
                decoded,
                MemWidth::Double,
                sense,
                pred_new,
                false
            ))
        }
        Opcode::S4_pstorerft_rr
        | Opcode::S4_pstorerff_rr
        | Opcode::S4_pstorerftnew_rr
        | Opcode::S4_pstorerffnew_rr => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_rr(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                true
            ))
        }
        Opcode::S4_pstorerbnewt_rr
        | Opcode::S4_pstorerbnewf_rr
        | Opcode::S4_pstorerbnewtnew_rr
        | Opcode::S4_pstorerbnewfnew_rr => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_rr(decoded, MemWidth::Byte, sense, pred_new))
        }
        Opcode::S4_pstorerhnewt_rr
        | Opcode::S4_pstorerhnewf_rr
        | Opcode::S4_pstorerhnewtnew_rr
        | Opcode::S4_pstorerhnewfnew_rr => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_rr(decoded, MemWidth::Half, sense, pred_new))
        }
        Opcode::S4_pstorerinewt_rr
        | Opcode::S4_pstorerinewf_rr
        | Opcode::S4_pstorerinewtnew_rr
        | Opcode::S4_pstorerinewfnew_rr => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_rr(decoded, MemWidth::Word, sense, pred_new))
        }
        // ================================================================
        // Predicated absolute stores (S4_pstorer*_abs / *new_abs)
        // ================================================================
        Opcode::S4_pstorerbt_abs
        | Opcode::S4_pstorerbf_abs
        | Opcode::S4_pstorerbtnew_abs
        | Opcode::S4_pstorerbfnew_abs => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_abs(
                decoded,
                MemWidth::Byte,
                sense,
                pred_new,
                false,
                immext
            ))
        }
        Opcode::S4_pstorerht_abs
        | Opcode::S4_pstorerhf_abs
        | Opcode::S4_pstorerhtnew_abs
        | Opcode::S4_pstorerhfnew_abs => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_abs(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                false,
                immext
            ))
        }
        Opcode::S4_pstorerit_abs
        | Opcode::S4_pstorerif_abs
        | Opcode::S4_pstoreritnew_abs
        | Opcode::S4_pstorerifnew_abs => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_abs(
                decoded,
                MemWidth::Word,
                sense,
                pred_new,
                false,
                immext
            ))
        }
        Opcode::S4_pstorerdt_abs
        | Opcode::S4_pstorerdf_abs
        | Opcode::S4_pstorerdtnew_abs
        | Opcode::S4_pstorerdfnew_abs => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_abs(
                decoded,
                MemWidth::Double,
                sense,
                pred_new,
                false,
                immext
            ))
        }
        Opcode::S4_pstorerft_abs
        | Opcode::S4_pstorerff_abs
        | Opcode::S4_pstorerftnew_abs
        | Opcode::S4_pstorerffnew_abs => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_abs(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                true,
                immext
            ))
        }
        Opcode::S4_pstorerbnewt_abs
        | Opcode::S4_pstorerbnewf_abs
        | Opcode::S4_pstorerbnewtnew_abs
        | Opcode::S4_pstorerbnewfnew_abs => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_abs(
                decoded,
                MemWidth::Byte,
                sense,
                pred_new,
                immext
            ))
        }
        Opcode::S4_pstorerhnewt_abs
        | Opcode::S4_pstorerhnewf_abs
        | Opcode::S4_pstorerhnewtnew_abs
        | Opcode::S4_pstorerhnewfnew_abs => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_abs(
                decoded,
                MemWidth::Half,
                sense,
                pred_new,
                immext
            ))
        }
        Opcode::S4_pstorerinewt_abs
        | Opcode::S4_pstorerinewf_abs
        | Opcode::S4_pstorerinewtnew_abs
        | Opcode::S4_pstorerinewfnew_abs => {
            let (sense, pred_new) = pstore_cond(decoded.opcode);
            req!(pred_store_new_abs(
                decoded,
                MemWidth::Word,
                sense,
                pred_new,
                immext
            ))
        }
        // ================================================================
        // Store-immediate (S4_storeir<W>[cond]_io): mem = sign-extended #s6
        // ================================================================
        Opcode::S4_storeirb_io => req!(store_imm_io(decoded, MemWidth::Byte, None)),
        Opcode::S4_storeirh_io => req!(store_imm_io(decoded, MemWidth::Half, None)),
        Opcode::S4_storeiri_io => req!(store_imm_io(decoded, MemWidth::Word, None)),
        Opcode::S4_storeirbt_io
        | Opcode::S4_storeirbf_io
        | Opcode::S4_storeirbtnew_io
        | Opcode::S4_storeirbfnew_io => {
            req!(store_imm_io(
                decoded,
                MemWidth::Byte,
                Some(pstore_cond(decoded.opcode))
            ))
        }
        Opcode::S4_storeirht_io
        | Opcode::S4_storeirhf_io
        | Opcode::S4_storeirhtnew_io
        | Opcode::S4_storeirhfnew_io => {
            req!(store_imm_io(
                decoded,
                MemWidth::Half,
                Some(pstore_cond(decoded.opcode))
            ))
        }
        Opcode::S4_storeirit_io
        | Opcode::S4_storeirif_io
        | Opcode::S4_storeiritnew_io
        | Opcode::S4_storeirifnew_io => {
            req!(store_imm_io(
                decoded,
                MemWidth::Word,
                Some(pstore_cond(decoded.opcode))
            ))
        }
        // ---- read-modify-write memops (memX(Rs+#u6:N) OP= Rt | #U5) ----
        Opcode::L4_add_memopb_io => req!(memop(decoded, MemWidth::Byte, MemOpKind::Add, false)),
        Opcode::L4_sub_memopb_io => req!(memop(decoded, MemWidth::Byte, MemOpKind::Sub, false)),
        Opcode::L4_and_memopb_io => req!(memop(decoded, MemWidth::Byte, MemOpKind::And, false)),
        Opcode::L4_or_memopb_io => req!(memop(decoded, MemWidth::Byte, MemOpKind::Or, false)),
        Opcode::L4_iadd_memopb_io => req!(memop(decoded, MemWidth::Byte, MemOpKind::Add, true)),
        Opcode::L4_isub_memopb_io => req!(memop(decoded, MemWidth::Byte, MemOpKind::Sub, true)),
        Opcode::L4_iand_memopb_io => req!(memop(decoded, MemWidth::Byte, MemOpKind::ClrBit, true)),
        Opcode::L4_ior_memopb_io => req!(memop(decoded, MemWidth::Byte, MemOpKind::SetBit, true)),
        Opcode::L4_add_memoph_io => req!(memop(decoded, MemWidth::Half, MemOpKind::Add, false)),
        Opcode::L4_sub_memoph_io => req!(memop(decoded, MemWidth::Half, MemOpKind::Sub, false)),
        Opcode::L4_and_memoph_io => req!(memop(decoded, MemWidth::Half, MemOpKind::And, false)),
        Opcode::L4_or_memoph_io => req!(memop(decoded, MemWidth::Half, MemOpKind::Or, false)),
        Opcode::L4_iadd_memoph_io => req!(memop(decoded, MemWidth::Half, MemOpKind::Add, true)),
        Opcode::L4_isub_memoph_io => req!(memop(decoded, MemWidth::Half, MemOpKind::Sub, true)),
        Opcode::L4_iand_memoph_io => req!(memop(decoded, MemWidth::Half, MemOpKind::ClrBit, true)),
        Opcode::L4_ior_memoph_io => req!(memop(decoded, MemWidth::Half, MemOpKind::SetBit, true)),
        Opcode::L4_add_memopw_io => req!(memop(decoded, MemWidth::Word, MemOpKind::Add, false)),
        Opcode::L4_sub_memopw_io => req!(memop(decoded, MemWidth::Word, MemOpKind::Sub, false)),
        Opcode::L4_and_memopw_io => req!(memop(decoded, MemWidth::Word, MemOpKind::And, false)),
        Opcode::L4_or_memopw_io => req!(memop(decoded, MemWidth::Word, MemOpKind::Or, false)),
        Opcode::L4_iadd_memopw_io => req!(memop(decoded, MemWidth::Word, MemOpKind::Add, true)),
        Opcode::L4_isub_memopw_io => req!(memop(decoded, MemWidth::Word, MemOpKind::Sub, true)),
        Opcode::L4_iand_memopw_io => req!(memop(decoded, MemWidth::Word, MemOpKind::ClrBit, true)),
        Opcode::L4_ior_memopw_io => req!(memop(decoded, MemWidth::Word, MemOpKind::SetBit, true)),
        // ---- HVX vector loads/stores (vmem / vmemu) ----
        // Aligned loads (`vL32b`); `:nt` is a cache hint with no architectural
        // effect. `ai` = vmem(Rt+#s4), `pi` = vmem(Rt++#s4), `ppu` = vmem(Rt++Mu).
        Opcode::V6_vL32b_ai | Opcode::V6_vL32b_nt_ai => req!(vmem_load(decoded, VAddr::Ai, true)),
        Opcode::V6_vL32b_pi | Opcode::V6_vL32b_nt_pi => req!(vmem_load(decoded, VAddr::Pi, true)),
        Opcode::V6_vL32b_ppu | Opcode::V6_vL32b_nt_ppu => {
            req!(vmem_load(decoded, VAddr::Ppu, true))
        }
        // Unaligned loads (`vL32Ub` / `vmemu`).
        Opcode::V6_vL32Ub_ai => req!(vmem_load(decoded, VAddr::Ai, false)),
        Opcode::V6_vL32Ub_pi => req!(vmem_load(decoded, VAddr::Pi, false)),
        Opcode::V6_vL32Ub_ppu => req!(vmem_load(decoded, VAddr::Ppu, false)),
        // `cur` loads commit the value to Vd (and forward it within the packet);
        // `tmp` loads do NOT commit (scratch, forward-only) — same loaded data.
        // `:nt` is just a hint, so the `nt_cur`/`nt_tmp` forms behave identically.
        Opcode::V6_vL32b_cur_ai | Opcode::V6_vL32b_nt_cur_ai => {
            req!(vmem_load_c(decoded, VAddr::Ai, true, true))
        }
        Opcode::V6_vL32b_cur_pi | Opcode::V6_vL32b_nt_cur_pi => {
            req!(vmem_load_c(decoded, VAddr::Pi, true, true))
        }
        Opcode::V6_vL32b_cur_ppu | Opcode::V6_vL32b_nt_cur_ppu => {
            req!(vmem_load_c(decoded, VAddr::Ppu, true, true))
        }
        Opcode::V6_vL32b_tmp_ai | Opcode::V6_vL32b_nt_tmp_ai => {
            req!(vmem_load_c(decoded, VAddr::Ai, true, false))
        }
        Opcode::V6_vL32b_tmp_pi | Opcode::V6_vL32b_nt_tmp_pi => {
            req!(vmem_load_c(decoded, VAddr::Pi, true, false))
        }
        Opcode::V6_vL32b_tmp_ppu | Opcode::V6_vL32b_nt_tmp_ppu => {
            req!(vmem_load_c(decoded, VAddr::Ppu, true, false))
        }
        // Scalar-predicated loads (`if (Pv[!]) Vd[.cur|.tmp] = vmem(...)`). The
        // predicate operand is field `v`; on a false predicate the load is
        // cancelled (no register write, no post-increment) — handled by the
        // executor via the load's `pred` field. Plain (non-cur/tmp) predicated
        // loads commit; `.cur` commits; `.tmp` does not.
        Opcode::V6_vL32b_pred_ai | Opcode::V6_vL32b_nt_pred_ai => {
            req!(vmem_load_pred(decoded, VAddr::Ai, true, true, true))
        }
        Opcode::V6_vL32b_pred_pi | Opcode::V6_vL32b_nt_pred_pi => {
            req!(vmem_load_pred(decoded, VAddr::Pi, true, true, true))
        }
        Opcode::V6_vL32b_pred_ppu | Opcode::V6_vL32b_nt_pred_ppu => {
            req!(vmem_load_pred(decoded, VAddr::Ppu, true, true, true))
        }
        Opcode::V6_vL32b_npred_ai | Opcode::V6_vL32b_nt_npred_ai => {
            req!(vmem_load_pred(decoded, VAddr::Ai, true, false, true))
        }
        Opcode::V6_vL32b_npred_pi | Opcode::V6_vL32b_nt_npred_pi => {
            req!(vmem_load_pred(decoded, VAddr::Pi, true, false, true))
        }
        Opcode::V6_vL32b_npred_ppu | Opcode::V6_vL32b_nt_npred_ppu => {
            req!(vmem_load_pred(decoded, VAddr::Ppu, true, false, true))
        }
        Opcode::V6_vL32b_cur_pred_ai | Opcode::V6_vL32b_nt_cur_pred_ai => {
            req!(vmem_load_pred(decoded, VAddr::Ai, true, true, true))
        }
        Opcode::V6_vL32b_cur_pred_pi | Opcode::V6_vL32b_nt_cur_pred_pi => {
            req!(vmem_load_pred(decoded, VAddr::Pi, true, true, true))
        }
        Opcode::V6_vL32b_cur_pred_ppu | Opcode::V6_vL32b_nt_cur_pred_ppu => {
            req!(vmem_load_pred(decoded, VAddr::Ppu, true, true, true))
        }
        Opcode::V6_vL32b_cur_npred_ai | Opcode::V6_vL32b_nt_cur_npred_ai => {
            req!(vmem_load_pred(decoded, VAddr::Ai, true, false, true))
        }
        Opcode::V6_vL32b_cur_npred_pi | Opcode::V6_vL32b_nt_cur_npred_pi => {
            req!(vmem_load_pred(decoded, VAddr::Pi, true, false, true))
        }
        Opcode::V6_vL32b_cur_npred_ppu | Opcode::V6_vL32b_nt_cur_npred_ppu => {
            req!(vmem_load_pred(decoded, VAddr::Ppu, true, false, true))
        }
        Opcode::V6_vL32b_tmp_pred_ai | Opcode::V6_vL32b_nt_tmp_pred_ai => {
            req!(vmem_load_pred(decoded, VAddr::Ai, true, true, false))
        }
        Opcode::V6_vL32b_tmp_pred_pi | Opcode::V6_vL32b_nt_tmp_pred_pi => {
            req!(vmem_load_pred(decoded, VAddr::Pi, true, true, false))
        }
        Opcode::V6_vL32b_tmp_pred_ppu | Opcode::V6_vL32b_nt_tmp_pred_ppu => {
            req!(vmem_load_pred(decoded, VAddr::Ppu, true, true, false))
        }
        Opcode::V6_vL32b_tmp_npred_ai | Opcode::V6_vL32b_nt_tmp_npred_ai => {
            req!(vmem_load_pred(decoded, VAddr::Ai, true, false, false))
        }
        Opcode::V6_vL32b_tmp_npred_pi | Opcode::V6_vL32b_nt_tmp_npred_pi => {
            req!(vmem_load_pred(decoded, VAddr::Pi, true, false, false))
        }
        Opcode::V6_vL32b_tmp_npred_ppu | Opcode::V6_vL32b_nt_tmp_npred_ppu => {
            req!(vmem_load_pred(decoded, VAddr::Ppu, true, false, false))
        }
        // Unaligned post-inc-by-Mu store (`vS32Ub_ppu` + ai/pi already below).
        Opcode::V6_vS32Ub_ppu => req!(vmem_store(decoded, VAddr::Ppu, false)),
        // Aligned stores (`vS32b`); `:nt` hint only.
        Opcode::V6_vS32b_ai | Opcode::V6_vS32b_nt_ai => req!(vmem_store(decoded, VAddr::Ai, true)),
        Opcode::V6_vS32b_pi | Opcode::V6_vS32b_nt_pi => req!(vmem_store(decoded, VAddr::Pi, true)),
        Opcode::V6_vS32b_ppu | Opcode::V6_vS32b_nt_ppu => {
            req!(vmem_store(decoded, VAddr::Ppu, true))
        }
        Opcode::V6_vS32Ub_ai => req!(vmem_store(decoded, VAddr::Ai, false)),
        Opcode::V6_vS32Ub_pi => req!(vmem_store(decoded, VAddr::Pi, false)),
        // Scalar-predicated vector stores: if (Pv[!]) vmem(...) = Vs.
        Opcode::V6_vS32b_pred_ai => req!(vmem_store_pred(decoded, VAddr::Ai, true, Some(true))),
        Opcode::V6_vS32b_npred_ai => req!(vmem_store_pred(decoded, VAddr::Ai, true, Some(false))),
        Opcode::V6_vS32b_pred_pi => req!(vmem_store_pred(decoded, VAddr::Pi, true, Some(true))),
        Opcode::V6_vS32b_npred_pi => req!(vmem_store_pred(decoded, VAddr::Pi, true, Some(false))),
        Opcode::V6_vS32b_pred_ppu | Opcode::V6_vS32b_nt_pred_ppu => {
            req!(vmem_store_pred(decoded, VAddr::Ppu, true, Some(true)))
        }
        Opcode::V6_vS32b_npred_ppu | Opcode::V6_vS32b_nt_npred_ppu => {
            req!(vmem_store_pred(decoded, VAddr::Ppu, true, Some(false)))
        }
        Opcode::V6_vS32b_nt_pred_ai => req!(vmem_store_pred(decoded, VAddr::Ai, true, Some(true))),
        Opcode::V6_vS32b_nt_npred_ai => {
            req!(vmem_store_pred(decoded, VAddr::Ai, true, Some(false)))
        }
        Opcode::V6_vS32b_nt_pred_pi => req!(vmem_store_pred(decoded, VAddr::Pi, true, Some(true))),
        Opcode::V6_vS32b_nt_npred_pi => {
            req!(vmem_store_pred(decoded, VAddr::Pi, true, Some(false)))
        }
        // Unaligned predicated stores (`vS32Ub_[n]pred_*`).
        Opcode::V6_vS32Ub_pred_ai => req!(vmem_store_pred(decoded, VAddr::Ai, false, Some(true))),
        Opcode::V6_vS32Ub_npred_ai => {
            req!(vmem_store_pred(decoded, VAddr::Ai, false, Some(false)))
        }
        Opcode::V6_vS32Ub_pred_pi => req!(vmem_store_pred(decoded, VAddr::Pi, false, Some(true))),
        Opcode::V6_vS32Ub_npred_pi => {
            req!(vmem_store_pred(decoded, VAddr::Pi, false, Some(false)))
        }
        Opcode::V6_vS32Ub_pred_ppu => {
            req!(vmem_store_pred(decoded, VAddr::Ppu, false, Some(true)))
        }
        Opcode::V6_vS32Ub_npred_ppu => {
            req!(vmem_store_pred(decoded, VAddr::Ppu, false, Some(false)))
        }
        // Byte-masked vector stores: if (Qv[!]) vmem(...) = Vs.
        Opcode::V6_vS32b_qpred_ai | Opcode::V6_vS32b_nt_qpred_ai => {
            req!(vmem_store_q(decoded, VAddr::Ai, true))
        }
        Opcode::V6_vS32b_nqpred_ai | Opcode::V6_vS32b_nt_nqpred_ai => {
            req!(vmem_store_q(decoded, VAddr::Ai, false))
        }
        Opcode::V6_vS32b_qpred_pi | Opcode::V6_vS32b_nt_qpred_pi => {
            req!(vmem_store_q(decoded, VAddr::Pi, true))
        }
        Opcode::V6_vS32b_nqpred_pi | Opcode::V6_vS32b_nt_nqpred_pi => {
            req!(vmem_store_q(decoded, VAddr::Pi, false))
        }
        Opcode::V6_vS32b_qpred_ppu | Opcode::V6_vS32b_nt_qpred_ppu => {
            req!(vmem_store_q(decoded, VAddr::Ppu, true))
        }
        Opcode::V6_vS32b_nqpred_ppu | Opcode::V6_vS32b_nt_nqpred_ppu => {
            req!(vmem_store_q(decoded, VAddr::Ppu, false))
        }
        // New-value vector store `vmem(...) = V.new`: the source is the vector
        // produced earlier in the packet (or the gather scratch). `:nt` hint only.
        Opcode::V6_vS32b_new_ai | Opcode::V6_vS32b_nt_new_ai => {
            req!(vmem_store_new(decoded, VAddr::Ai, None))
        }
        Opcode::V6_vS32b_new_pi | Opcode::V6_vS32b_nt_new_pi => {
            req!(vmem_store_new(decoded, VAddr::Pi, None))
        }
        Opcode::V6_vS32b_new_ppu | Opcode::V6_vS32b_nt_new_ppu => {
            req!(vmem_store_new(decoded, VAddr::Ppu, None))
        }
        Opcode::V6_vS32b_new_pred_ai | Opcode::V6_vS32b_nt_new_pred_ai => {
            req!(vmem_store_new(decoded, VAddr::Ai, Some(true)))
        }
        Opcode::V6_vS32b_new_npred_ai | Opcode::V6_vS32b_nt_new_npred_ai => {
            req!(vmem_store_new(decoded, VAddr::Ai, Some(false)))
        }
        Opcode::V6_vS32b_new_pred_pi | Opcode::V6_vS32b_nt_new_pred_pi => {
            req!(vmem_store_new(decoded, VAddr::Pi, Some(true)))
        }
        Opcode::V6_vS32b_new_npred_pi | Opcode::V6_vS32b_nt_new_npred_pi => {
            req!(vmem_store_new(decoded, VAddr::Pi, Some(false)))
        }
        Opcode::V6_vS32b_new_pred_ppu | Opcode::V6_vS32b_nt_new_pred_ppu => {
            req!(vmem_store_new(decoded, VAddr::Ppu, Some(true)))
        }
        Opcode::V6_vS32b_new_npred_ppu | Opcode::V6_vS32b_nt_new_npred_ppu => {
            req!(vmem_store_new(decoded, VAddr::Ppu, Some(false)))
        }
        // Store-release barrier `vmem(...):scatter_release` (no vector source).
        Opcode::V6_vS32b_srls_ai => req!(vmem_store_srls(decoded, VAddr::Ai)),
        Opcode::V6_vS32b_srls_pi => req!(vmem_store_srls(decoded, VAddr::Pi)),
        Opcode::V6_vS32b_srls_ppu => req!(vmem_store_srls(decoded, VAddr::Ppu)),
        // ---- HVX V65 scatter (vscatter) ----
        // esz, off_pair, add, predicated.
        Opcode::V6_vscattermw => req!(vscatter(decoded, 4, false, false, false)),
        Opcode::V6_vscattermh => req!(vscatter(decoded, 2, false, false, false)),
        Opcode::V6_vscattermhw => req!(vscatter(decoded, 2, true, false, false)),
        Opcode::V6_vscattermw_add => req!(vscatter(decoded, 4, false, true, false)),
        Opcode::V6_vscattermh_add => req!(vscatter(decoded, 2, false, true, false)),
        Opcode::V6_vscattermhw_add => req!(vscatter(decoded, 2, true, true, false)),
        Opcode::V6_vscattermwq => req!(vscatter(decoded, 4, false, false, true)),
        Opcode::V6_vscattermhq => req!(vscatter(decoded, 2, false, false, true)),
        Opcode::V6_vscattermhwq => req!(vscatter(decoded, 2, true, false, true)),
        // ---- HVX V65 gather (vgather) ----
        Opcode::V6_vgathermw => req!(vgather(decoded, 4, false, false)),
        Opcode::V6_vgathermh => req!(vgather(decoded, 2, false, false)),
        Opcode::V6_vgathermhw => req!(vgather(decoded, 2, true, false)),
        Opcode::V6_vgathermwq => req!(vgather(decoded, 4, false, true)),
        Opcode::V6_vgathermhq => req!(vgather(decoded, 2, false, true)),
        Opcode::V6_vgathermhwq => req!(vgather(decoded, 2, true, true)),
        // J4 compound compare-and-jump (`J4_<cmp>_<cond>_jump[nv]_<hint>`): there
        // are 119 of these, decoded uniformly by parsing the mnemonic.
        _ => match decode_compound_cmpjump(decoded, opcode::opcode_name(decoded.opcode), immext) {
            Some(result) => result,
            None => (DecodedInsn::Unknown(word), false),
        },
    }
}

fn decode_subinsn(sub: u16, class: EncClass, isa: HexagonIsa) -> Option<DecodedSub> {
    let decoded = opcode::decode_sub(sub, class)?;
    let opcode = decoded.opcode;
    let insn = match opcode {
        Opcode::SL1_loadri_io => {
            let base = subreg(field_u8(&decoded, b's')?);
            let dst = subreg(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            let offset = imm << 2;
            DecodedInsn::Load {
                dst,
                addr: AddrMode::Offset { base, offset },
                width: MemWidth::Word,
                sign: MemSign::Unsigned,
                pred: None,
            }
        }
        Opcode::SL1_loadrub_io => {
            let base = subreg(field_u8(&decoded, b's')?);
            let dst = subreg(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Load {
                dst,
                addr: AddrMode::Offset { base, offset: imm },
                width: MemWidth::Byte,
                sign: MemSign::Unsigned,
                pred: None,
            }
        }
        Opcode::SS1_storew_io => {
            let base = subreg(field_u8(&decoded, b's')?);
            let src = subreg(field_u8(&decoded, b't')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Store {
                src,
                addr: AddrMode::Offset {
                    base,
                    offset: imm << 2,
                },
                width: MemWidth::Word,
                pred: None,
                src_new: false,
                high_half: false,
            }
        }
        Opcode::SS1_storeb_io => {
            let base = subreg(field_u8(&decoded, b's')?);
            let src = subreg(field_u8(&decoded, b't')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Store {
                src,
                addr: AddrMode::Offset { base, offset: imm },
                width: MemWidth::Byte,
                pred: None,
                src_new: false,
                high_half: false,
            }
        }
        Opcode::SL2_loadrh_io => {
            let base = subreg(field_u8(&decoded, b's')?);
            let dst = subreg(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Load {
                dst,
                addr: AddrMode::Offset {
                    base,
                    offset: imm << 1,
                },
                width: MemWidth::Half,
                sign: MemSign::Signed,
                pred: None,
            }
        }
        Opcode::SL2_loadruh_io => {
            let base = subreg(field_u8(&decoded, b's')?);
            let dst = subreg(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Load {
                dst,
                addr: AddrMode::Offset {
                    base,
                    offset: imm << 1,
                },
                width: MemWidth::Half,
                sign: MemSign::Unsigned,
                pred: None,
            }
        }
        Opcode::SL2_loadrb_io => {
            let base = subreg(field_u8(&decoded, b's')?);
            let dst = subreg(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Load {
                dst,
                addr: AddrMode::Offset { base, offset: imm },
                width: MemWidth::Byte,
                sign: MemSign::Signed,
                pred: None,
            }
        }
        Opcode::SL2_loadri_sp => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Load {
                dst,
                addr: AddrMode::Offset {
                    base: 29,
                    offset: imm << 2,
                },
                width: MemWidth::Word,
                sign: MemSign::Unsigned,
                pred: None,
            }
        }
        Opcode::SL2_loadrd_sp => {
            let dst = subreg_pair(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Load {
                dst,
                addr: AddrMode::Offset {
                    base: 29,
                    offset: imm << 3,
                },
                width: MemWidth::Double,
                sign: MemSign::Unsigned,
                pred: None,
            }
        }
        Opcode::SL2_deallocframe => DecodedInsn::DeallocFrame {
            base: 30,
            dst: None,
            update_lr_fp: true,
        },
        Opcode::SL2_return => DecodedInsn::DeallocReturn {
            base: 30,
            dst: None,
            pred: None,
            update_lr_fp: true,
        },
        Opcode::SL2_return_t => DecodedInsn::DeallocReturn {
            base: 30,
            dst: None,
            pred: Some(pred_cond(0, true, false)),
            update_lr_fp: true,
        },
        Opcode::SL2_return_f => DecodedInsn::DeallocReturn {
            base: 30,
            dst: None,
            pred: Some(pred_cond(0, false, false)),
            update_lr_fp: true,
        },
        Opcode::SL2_return_tnew => DecodedInsn::DeallocReturn {
            base: 30,
            dst: None,
            pred: Some(pred_cond(0, true, true)),
            update_lr_fp: true,
        },
        Opcode::SL2_return_fnew => DecodedInsn::DeallocReturn {
            base: 30,
            dst: None,
            pred: Some(pred_cond(0, false, true)),
            update_lr_fp: true,
        },
        Opcode::SL2_jumpr31 => DecodedInsn::JumpReg { src: 31 },
        Opcode::SL2_jumpr31_t => DecodedInsn::JumpRegCond {
            src: 31,
            pred: 0,
            sense: true,
            pred_new: false,
        },
        Opcode::SL2_jumpr31_f => DecodedInsn::JumpRegCond {
            src: 31,
            pred: 0,
            sense: false,
            pred_new: false,
        },
        Opcode::SL2_jumpr31_tnew => DecodedInsn::JumpRegCond {
            src: 31,
            pred: 0,
            sense: true,
            pred_new: true,
        },
        Opcode::SL2_jumpr31_fnew => DecodedInsn::JumpRegCond {
            src: 31,
            pred: 0,
            sense: false,
            pred_new: true,
        },
        Opcode::SS2_allocframe => {
            let imm = decode_field_uimm(&decoded, b'i', None)?.0;
            DecodedInsn::AllocFrame {
                base: 29,
                size: imm << 3,
            }
        }
        Opcode::SS2_storeh_io => {
            let base = subreg(field_u8(&decoded, b's')?);
            let src = subreg(field_u8(&decoded, b't')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Store {
                src,
                addr: AddrMode::Offset {
                    base,
                    offset: imm << 1,
                },
                width: MemWidth::Half,
                pred: None,
                src_new: false,
                high_half: false,
            }
        }
        Opcode::SS2_storew_sp => {
            let src = subreg(field_u8(&decoded, b't')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::Store {
                src,
                addr: AddrMode::Offset {
                    base: 29,
                    offset: imm << 2,
                },
                width: MemWidth::Word,
                pred: None,
                src_new: false,
                high_half: false,
            }
        }
        Opcode::SS2_stored_sp => {
            let src = subreg_pair(field_u8(&decoded, b't')?);
            let imm = decode_simm_val(decode_field_uimm(&decoded, b'i', None)?.0, 6, None).0;
            DecodedInsn::Store {
                src,
                addr: AddrMode::Offset {
                    base: 29,
                    offset: imm << 3,
                },
                width: MemWidth::Double,
                pred: None,
                src_new: false,
                high_half: false,
            }
        }
        Opcode::SS2_storewi0 => {
            let base = subreg(field_u8(&decoded, b's')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::StoreImm {
                value: 0,
                addr: AddrMode::Offset {
                    base,
                    offset: imm << 2,
                },
                width: MemWidth::Word,
                pred: None,
            }
        }
        Opcode::SS2_storewi1 => {
            let base = subreg(field_u8(&decoded, b's')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::StoreImm {
                value: 1,
                addr: AddrMode::Offset {
                    base,
                    offset: imm << 2,
                },
                width: MemWidth::Word,
                pred: None,
            }
        }
        Opcode::SS2_storebi0 => {
            let base = subreg(field_u8(&decoded, b's')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::StoreImm {
                value: 0,
                addr: AddrMode::Offset { base, offset: imm },
                width: MemWidth::Byte,
                pred: None,
            }
        }
        Opcode::SS2_storebi1 => {
            let base = subreg(field_u8(&decoded, b's')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::StoreImm {
                value: 1,
                addr: AddrMode::Offset { base, offset: imm },
                width: MemWidth::Byte,
                pred: None,
            }
        }
        Opcode::SA1_addi => {
            let rx = subreg(field_u8(&decoded, b'x')?);
            let imm = decode_simm_val(decode_field_uimm(&decoded, b'i', None)?.0, 7, None).0;
            DecodedInsn::AddImm {
                dst: rx,
                src: rx,
                imm,
            }
        }
        Opcode::SA1_addsp => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::AddImm {
                dst,
                src: 29,
                imm: imm << 2,
            }
        }
        Opcode::SA1_seti => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::MovImm { dst, imm }
        }
        Opcode::SA1_setin1 => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            DecodedInsn::MovImm { dst, imm: -1 }
        }
        Opcode::SA1_tfr => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::Mov { dst, src }
        }
        Opcode::SA1_inc => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::AddImm { dst, src, imm: 1 }
        }
        Opcode::SA1_dec => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::AddImm { dst, src, imm: -1 }
        }
        Opcode::SA1_and1 => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::AndImm { dst, src, imm: 1 }
        }
        Opcode::SA1_addrx => {
            // Rx = add(Rx, Rs)  (read-modify GPR; fields x=Rx, s=Rs).
            let rx = subreg(field_u8(&decoded, b'x')?);
            let rs = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::Add {
                dst: rx,
                src1: rx,
                src2: rs,
            }
        }
        Opcode::SA1_sxtb => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::Extend {
                dst,
                src,
                kind: ExtendKind::Sxt8,
            }
        }
        Opcode::SA1_sxth => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::Extend {
                dst,
                src,
                kind: ExtendKind::Sxt16,
            }
        }
        Opcode::SA1_zxtb => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::Extend {
                dst,
                src,
                kind: ExtendKind::Zxt8,
            }
        }
        Opcode::SA1_zxth => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::Extend {
                dst,
                src,
                kind: ExtendKind::Zxt16,
            }
        }
        Opcode::SA1_cmpeqi => {
            let src = subreg(field_u8(&decoded, b's')?);
            let imm = decode_field_uimm(&decoded, b'i', None)?.0 as i32;
            DecodedInsn::CmpImm {
                pred: 0,
                src,
                imm,
                kind: CmpKind::Eq,
                unsigned: false,
            }
        }
        Opcode::SA1_clrt => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            DecodedInsn::ClearCond {
                dst,
                pred: pred_cond(0, true, false),
            }
        }
        Opcode::SA1_clrf => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            DecodedInsn::ClearCond {
                dst,
                pred: pred_cond(0, false, false),
            }
        }
        Opcode::SA1_clrtnew => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            DecodedInsn::ClearCond {
                dst,
                pred: pred_cond(0, true, true),
            }
        }
        Opcode::SA1_clrfnew => {
            let dst = subreg(field_u8(&decoded, b'd')?);
            DecodedInsn::ClearCond {
                dst,
                pred: pred_cond(0, false, true),
            }
        }
        Opcode::SA1_combinezr => {
            let dst = subreg_pair(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::Combine {
                dst,
                high: CombineOperand::Imm(0),
                low: CombineOperand::Reg(src),
            }
        }
        Opcode::SA1_combinerz => {
            let dst = subreg_pair(field_u8(&decoded, b'd')?);
            let src = subreg(field_u8(&decoded, b's')?);
            DecodedInsn::Combine {
                dst,
                high: CombineOperand::Reg(src),
                low: CombineOperand::Imm(0),
            }
        }
        Opcode::SA1_combine0i
        | Opcode::SA1_combine1i
        | Opcode::SA1_combine2i
        | Opcode::SA1_combine3i => {
            let dst = subreg_pair(field_u8(&decoded, b'd')?);
            let low = decode_field_uimm(&decoded, b'i', None)?.0;
            let high = match decoded.opcode {
                Opcode::SA1_combine0i => 0,
                Opcode::SA1_combine1i => 1,
                Opcode::SA1_combine2i => 2,
                _ => 3,
            };
            DecodedInsn::Combine {
                dst,
                high: CombineOperand::Imm(high),
                low: CombineOperand::Imm(low),
            }
        }
        _ => DecodedInsn::Unknown(sub as u32),
    };

    if insn_uses_dotnew(&insn) && !isa_supports_dotnew(isa) {
        return None;
    }

    Some(DecodedSub {
        insn,
        opcode: Some(opcode),
    })
}

fn duplex_iclass(word: u32) -> u8 {
    let low = ((word >> 13) & 0x1) as u8;
    let high = ((word >> 29) & 0x7) as u8;
    (high << 1) | low
}

pub fn decode_duplex(word: u32, isa: HexagonIsa) -> Option<(DecodedSub, DecodedSub)> {
    if !isa_supports_duplex(isa) {
        return None;
    }
    let iclass = duplex_iclass(word);
    let slot0 = (word & 0x1fff) as u16;
    let slot1 = ((word >> 16) & 0x1fff) as u16;

    let (class1, class0) = match iclass {
        0x0 => (EncClass::SubinsnL1, EncClass::SubinsnL1),
        0x1 => (EncClass::SubinsnL2, EncClass::SubinsnL1),
        0x2 => (EncClass::SubinsnL2, EncClass::SubinsnL2),
        0x3 => (EncClass::SubinsnA, EncClass::SubinsnA),
        0x4 => (EncClass::SubinsnL1, EncClass::SubinsnA),
        0x5 => (EncClass::SubinsnL2, EncClass::SubinsnA),
        0x6 => (EncClass::SubinsnS1, EncClass::SubinsnA),
        0x7 => (EncClass::SubinsnS2, EncClass::SubinsnA),
        0x8 => (EncClass::SubinsnS1, EncClass::SubinsnL1),
        0x9 => (EncClass::SubinsnS1, EncClass::SubinsnL2),
        0xa => (EncClass::SubinsnS1, EncClass::SubinsnS1),
        0xb => (EncClass::SubinsnS2, EncClass::SubinsnS1),
        0xc => (EncClass::SubinsnS2, EncClass::SubinsnL1),
        0xd => (EncClass::SubinsnS2, EncClass::SubinsnL2),
        0xe => (EncClass::SubinsnS2, EncClass::SubinsnS2),
        _ => return None,
    };

    let insn1 = decode_subinsn(slot1, class1, isa)?;
    let insn0 = decode_subinsn(slot0, class0, isa)?;
    Some((insn1, insn0))
}

pub fn decode(word: u32, immext: Option<u32>, _isa: HexagonIsa) -> DecodedWord {
    let decoded = match opcode::decode_word(word) {
        Some(decoded) => decoded,
        None => {
            return DecodedWord {
                insn: DecodedInsn::Unknown(word),
                used_ext: false,
                opcode: None,
            };
        }
    };

    let (insn, used_ext) = decode_main(&decoded, word, immext);
    DecodedWord {
        insn,
        used_ext,
        opcode: Some(decoded.opcode),
    }
}
