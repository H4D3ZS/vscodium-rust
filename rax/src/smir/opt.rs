//! SMIR optimization passes.
//!
//! This module implements optimization passes for SMIR to improve execution performance.
//! The most impactful optimization for x86 is dead flag elimination, which removes
//! flag updates that are never read.

use std::collections::{HashMap, HashSet};

use crate::smir::flags::{FlagSet, FlagState, FlagUpdate};
use crate::smir::ir::{CallTarget, SmirBlock, SmirFunction, Terminator};
use crate::smir::ops::{OpKind, SmirOp, X86OpHint, X86VecAlign};
use crate::smir::types::{
    Address, ArchReg, BlockId, HexagonReg, MemWidth, OpWidth, SrcOperand, VReg, VecWidth, X86Reg,
};

// ============================================================================
// Optimization Level
// ============================================================================

/// Optimization level
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OptLevel {
    /// No optimization (for debugging)
    #[default]
    O0,

    /// Basic optimizations (fast compile, some speedup)
    O1,

    /// Full optimization (slower compile, best runtime)
    O2,
}

// ============================================================================
// Optimization Statistics
// ============================================================================

/// Statistics from optimization passes
#[derive(Clone, Debug, Default)]
pub struct OptStats {
    /// Dead flag updates eliminated
    pub dead_flags_eliminated: usize,

    /// Constants propagated
    pub constants_propagated: usize,

    /// Expressions folded
    pub expressions_folded: usize,

    /// Dead ops eliminated
    pub dead_ops_eliminated: usize,

    /// Strength reductions applied
    pub strength_reductions: usize,

    /// Blocks merged
    pub blocks_merged: usize,

    /// Redundant loads eliminated
    pub redundant_loads_eliminated: usize,

    /// Vector alignment hints inferred
    pub vector_alignments_inferred: usize,

    /// Copy-propagation operand rewrites
    pub copies_propagated: usize,

    /// Branch foldings / unreachable blocks removed
    pub branches_folded: usize,
}

impl OptStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge stats from another run
    pub fn merge(&mut self, other: &OptStats) {
        self.dead_flags_eliminated += other.dead_flags_eliminated;
        self.constants_propagated += other.constants_propagated;
        self.expressions_folded += other.expressions_folded;
        self.dead_ops_eliminated += other.dead_ops_eliminated;
        self.strength_reductions += other.strength_reductions;
        self.blocks_merged += other.blocks_merged;
        self.redundant_loads_eliminated += other.redundant_loads_eliminated;
        self.vector_alignments_inferred += other.vector_alignments_inferred;
        self.copies_propagated += other.copies_propagated;
        self.branches_folded += other.branches_folded;
    }

    /// Total optimizations applied
    pub fn total(&self) -> usize {
        self.dead_flags_eliminated
            + self.constants_propagated
            + self.expressions_folded
            + self.dead_ops_eliminated
            + self.strength_reductions
            + self.blocks_merged
            + self.redundant_loads_eliminated
            + self.vector_alignments_inferred
            + self.copies_propagated
            + self.branches_folded
    }
}

// ============================================================================
// Liveness analysis (registers + flags), frontier-aware
// ============================================================================
//
// The SMIR optimizer runs on *regions* that the JIT (or the differential
// harness) hands back to the interpreter at their exits. At such an exit EVERY
// architectural register and flag is live-out: the interpreter resumes and may
// read any of them. Only an *internal* `Branch`/`CondBranch`/`Switch` edge
// (whose targets are blocks present in this function) propagates a precise
// live set from the successor. Treating block boundaries as "nothing live"
// (the classic compiler default) would let DCE / dead-flag-elim delete the
// final architectural writes — which is why this analysis exists and why every
// flag-effect-dropping transform is gated on the op's flags already being dead.
//
// x86 partial-register semantics: a write of width >= 32 bits zero-extends and
// thus *fully* overwrites the 64-bit GPR; an 8/16-bit write preserves the upper
// bits and is therefore read-modify-write (it keeps the prior definition live).

/// Destination width of an integer op, when it has architecturally-meaningful
/// width (used for x86 partial-register liveness). `None` for ops without a
/// single integer result width (vectors, memory, etc.).
fn op_out_width(kind: &OpKind) -> Option<OpWidth> {
    match kind {
        OpKind::Add { width, .. }
        | OpKind::Sub { width, .. }
        | OpKind::Adc { width, .. }
        | OpKind::Sbb { width, .. }
        | OpKind::Neg { width, .. }
        | OpKind::Inc { width, .. }
        | OpKind::Dec { width, .. }
        | OpKind::And { width, .. }
        | OpKind::Or { width, .. }
        | OpKind::Xor { width, .. }
        | OpKind::AndNot { width, .. }
        | OpKind::Not { width, .. }
        | OpKind::Shl { width, .. }
        | OpKind::Shr { width, .. }
        | OpKind::Sar { width, .. }
        | OpKind::Shld { width, .. }
        | OpKind::Shrd { width, .. }
        | OpKind::Rol { width, .. }
        | OpKind::Ror { width, .. }
        | OpKind::Rcl { width, .. }
        | OpKind::Rcr { width, .. }
        | OpKind::MulU { width, .. }
        | OpKind::MulS { width, .. }
        | OpKind::DivU { width, .. }
        | OpKind::DivS { width, .. }
        | OpKind::Mov { width, .. }
        | OpKind::CMove { width, .. }
        | OpKind::Cwd { width, .. }
        | OpKind::Bsf { width, .. }
        | OpKind::Bsr { width, .. }
        | OpKind::Clz { width, .. }
        | OpKind::Ctz { width, .. }
        | OpKind::Popcnt { width, .. }
        | OpKind::Bswap { width, .. }
        | OpKind::Bt { width, .. }
        | OpKind::Bts { width, .. }
        | OpKind::Btr { width, .. }
        | OpKind::Btc { width, .. } => Some(*width),
        // ZeroExtend / SignExtend write the *destination* (to) width.
        OpKind::ZeroExtend { to_width, .. } | OpKind::SignExtend { to_width, .. } => {
            Some(*to_width)
        }
        // LEA computes a full pointer; SETcc writes a single byte.
        OpKind::Lea { .. } => Some(OpWidth::W64),
        OpKind::SetCC { .. } => Some(OpWidth::W8),
        _ => None,
    }
}

/// True if executing `op` fully overwrites every architectural register it
/// defines, so an earlier definition of the same register becomes dead.
/// Conservative: returns false when unsure (the register stays live — this is
/// the safe direction; it can only cost a missed optimization, never delete a
/// live definition).
fn op_fully_defines(kind: &OpKind) -> bool {
    let dests = kind.dests();
    if dests.is_empty() {
        return true;
    }
    // SSA virtual temporaries are defined in full by their (single) writer.
    if dests.iter().all(|d| matches!(d, VReg::Virtual(_))) {
        return true;
    }
    matches!(
        op_out_width(kind),
        Some(OpWidth::W32) | Some(OpWidth::W64) | Some(OpWidth::W128)
    )
}

/// Registers read by a terminator (used at the block's exit point).
fn terminator_reg_uses(term: &Terminator) -> Vec<VReg> {
    let mut v = Vec::new();
    match term {
        Terminator::CondBranch { cond, .. } => v.push(*cond),
        Terminator::Switch { index, .. } => v.push(*index),
        Terminator::IndirectBranch { target, .. } => v.push(*target),
        Terminator::IndirectBranchMem { addr, .. } => v.extend(addr.regs()),
        Terminator::Return { values } => v.extend(values.iter().copied()),
        Terminator::Call { target, args, .. } | Terminator::TailCall { target, args } => {
            if let CallTarget::Indirect(reg) = target {
                v.push(*reg);
            }
            if let CallTarget::IndirectMem(addr) = target {
                v.extend(addr.regs());
            }
            v.extend(args.iter().copied());
        }
        _ => {}
    }
    v
}

/// Does this terminator hand control out of the region (back to the
/// interpreter, a callee, or an unknown target)? Anything that is not an
/// internal branch whose every target is a block present in `func`.
fn terminator_is_exit(func: &SmirFunction, term: &Terminator) -> bool {
    let in_func = |id: BlockId| func.blocks.iter().any(|b| b.id == id);
    match term {
        Terminator::Branch { target } => !in_func(*target),
        Terminator::CondBranch {
            true_target,
            false_target,
            ..
        } => !in_func(*true_target) || !in_func(*false_target),
        Terminator::Switch {
            targets, default, ..
        } => !in_func(*default) || targets.iter().any(|t| !in_func(*t)),
        // Indirect branches (incomplete target lists), calls (escape to a
        // callee), tail calls, traps, returns, unreachable: all exits.
        _ => true,
    }
}

/// Per-block live-out sets after a frontier-aware backward dataflow fixpoint.
struct FuncLiveness {
    reg_out: HashMap<BlockId, HashSet<VReg>>,
    flag_out: HashMap<BlockId, FlagSet>,
}

/// Backward transfer through one block: given the live-out reg/flag sets,
/// returns the live-in sets. Handles x86 partial-register RMW.
fn block_transfer(
    block: &SmirBlock,
    mut rlive: HashSet<VReg>,
    mut flive: FlagSet,
) -> (HashSet<VReg>, FlagSet) {
    for op in block.ops.iter().rev() {
        let full = op_fully_defines(&op.kind);
        let dests = op.kind.dests();
        if full {
            for d in &dests {
                rlive.remove(d);
            }
        }
        for s in op.kind.source_vregs() {
            rlive.insert(s);
        }
        if !full {
            // Partial-width write reads the destination it merges into.
            for d in &dests {
                rlive.insert(*d);
            }
        }
        flive = flive
            .difference(op.kind.flags_must_write())
            .union(op.kind.flags_read());
    }
    (rlive, flive)
}

/// Compute per-block register + flag live-out for a function, with all
/// architectural state live at frontier exits.
fn compute_liveness(func: &SmirFunction) -> FuncLiveness {
    // Universe of architectural registers touched anywhere in the function —
    // the set that is live-out at any region exit.
    let mut universe: HashSet<VReg> = HashSet::new();
    let mut note = |v: VReg, set: &mut HashSet<VReg>| {
        if matches!(v, VReg::Arch(_)) {
            set.insert(v);
        }
    };
    for block in &func.blocks {
        for op in &block.ops {
            for d in op.kind.dests() {
                note(d, &mut universe);
            }
            for s in op.kind.source_vregs() {
                note(s, &mut universe);
            }
        }
        for u in terminator_reg_uses(&block.terminator) {
            note(u, &mut universe);
        }
    }

    let mut reg_in: HashMap<BlockId, HashSet<VReg>> =
        func.blocks.iter().map(|b| (b.id, HashSet::new())).collect();
    let mut flag_in: HashMap<BlockId, FlagSet> =
        func.blocks.iter().map(|b| (b.id, FlagSet::EMPTY)).collect();

    // Iterate to fixpoint (live sets grow monotonically).
    let max_iters = func.blocks.len() + 2;
    for _ in 0..max_iters {
        let mut changed = false;
        for block in &func.blocks {
            let mut rout: HashSet<VReg> = HashSet::new();
            let mut fout = FlagSet::EMPTY;
            if terminator_is_exit(func, &block.terminator) {
                rout.extend(universe.iter().copied());
                fout = FlagSet::ALL_X86;
            }
            for s in block.terminator.successors() {
                if let Some(ri) = reg_in.get(&s) {
                    rout.extend(ri.iter().copied());
                }
                if let Some(fi) = flag_in.get(&s) {
                    fout = fout.union(*fi);
                }
            }
            for u in terminator_reg_uses(&block.terminator) {
                rout.insert(u);
            }
            let (rin, fin) = block_transfer(block, rout, fout);
            if reg_in[&block.id] != rin {
                reg_in.insert(block.id, rin);
                changed = true;
            }
            if flag_in[&block.id] != fin {
                flag_in.insert(block.id, fin);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // Materialize live-out per block from the converged live-in sets.
    let mut reg_out = HashMap::new();
    let mut flag_out = HashMap::new();
    for block in &func.blocks {
        let mut rout: HashSet<VReg> = HashSet::new();
        let mut fout = FlagSet::EMPTY;
        if terminator_is_exit(func, &block.terminator) {
            rout.extend(universe.iter().copied());
            fout = FlagSet::ALL_X86;
        }
        for s in block.terminator.successors() {
            if let Some(ri) = reg_in.get(&s) {
                rout.extend(ri.iter().copied());
            }
            if let Some(fi) = flag_in.get(&s) {
                fout = fout.union(*fi);
            }
        }
        for u in terminator_reg_uses(&block.terminator) {
            rout.insert(u);
        }
        reg_out.insert(block.id, rout);
        flag_out.insert(block.id, fout);
    }

    FuncLiveness { reg_out, flag_out }
}

// ============================================================================
// Main Optimization Entry Point
// ============================================================================

/// Run optimization pipeline on a function
pub fn optimize_function(func: &mut SmirFunction, level: OptLevel) -> OptStats {
    optimize_function_with_stats(func, level)
}

/// Run optimization pipeline on a function, returning statistics.
///
/// Block-level passes are run to a fixpoint (they enable one another and change
/// liveness); liveness is recomputed each round so dead-flag and dead-code
/// elimination always see correct, frontier-aware live-out sets. This is the
/// only entry point that is safe to use on JIT regions / against KVM — the bare
/// per-block passes assume a caller-supplied live-out and must not be used
/// directly on architectural regions.
pub fn optimize_function_with_stats(func: &mut SmirFunction, level: OptLevel) -> OptStats {
    let mut stats = OptStats::new();
    if level == OptLevel::O0 {
        return stats;
    }
    let o2 = level == OptLevel::O2;

    let max_rounds = 8;
    for _ in 0..max_rounds {
        let live = compute_liveness(func);
        let mut round_changes = 0usize;
        for block in &mut func.blocks {
            let flag_out = live
                .flag_out
                .get(&block.id)
                .copied()
                .unwrap_or(FlagSet::ALL_X86);
            let empty_regs;
            let reg_out = match live.reg_out.get(&block.id) {
                Some(r) => r,
                None => {
                    empty_regs = HashSet::new();
                    &empty_regs
                }
            };

            let n = dead_flag_elimination_with(block, flag_out);
            stats.dead_flags_eliminated += n;
            round_changes += n;

            let n = constant_propagation(block);
            stats.constants_propagated += n;
            round_changes += n;

            let n = copy_propagation(block);
            stats.copies_propagated += n;
            round_changes += n;

            if o2 {
                let n = constant_folding(block);
                stats.expressions_folded += n;
                round_changes += n;

                let n = strength_reduction(block);
                stats.strength_reductions += n;
                round_changes += n;
            }

            let n = dead_code_elimination_with(block, reg_out);
            stats.dead_ops_eliminated += n;
            round_changes += n;
        }

        if o2 {
            let n = branch_folding(func);
            stats.branches_folded += n;
            round_changes += n;

            let n = block_merging(func);
            stats.blocks_merged += n;
            round_changes += n;

            let n = redundant_load_elimination(func);
            stats.redundant_loads_eliminated += n;
            round_changes += n;
        }

        if round_changes == 0 {
            break;
        }
    }

    if o2 {
        // Hint-only pass (no IR mutation) — run once at the end.
        stats.vector_alignments_inferred += vector_alignment_inference(func);
    }

    stats
}

// ============================================================================
// Dead Flag Elimination
// ============================================================================

/// Eliminate dead flag updates.
///
/// This is the most impactful optimization for x86 - removes flag updates that
/// are never read. Uses backward analysis to find live flags.
///
/// Returns the number of flag updates eliminated.
pub fn dead_flag_elimination(block: &mut SmirBlock) -> usize {
    // Bare per-block use: approximate live-out from the terminator only (a
    // CondBranch needs the status flags; any other terminator is assumed to
    // leave no flag live). This is the legacy block-local contract; JIT regions
    // must go through `optimize_function`, which supplies a frontier-aware
    // live-out via `dead_flag_elimination_with`.
    let live_out = if matches!(block.terminator, Terminator::CondBranch { .. }) {
        FlagSet::NZCV
    } else {
        FlagSet::EMPTY
    };
    dead_flag_elimination_with(block, live_out)
}

/// Eliminate dead flag updates given the flags live on block exit.
///
/// A flag-writing op has its `FlagUpdate` cleared to `None` when none of the
/// flags it writes are live after it — either because a later op in this block
/// overwrites them before any read, or because they are not in `live_out`.
/// Returns the number of flag updates eliminated.
pub fn dead_flag_elimination_with(block: &mut SmirBlock, live_out: FlagSet) -> usize {
    if block.ops.is_empty() {
        return 0;
    }

    // Backward pass: liveness[i] = flags live immediately AFTER op i.
    let mut liveness = vec![FlagSet::EMPTY; block.ops.len()];
    let mut current_live = live_out;
    for i in (0..block.ops.len()).rev() {
        liveness[i] = current_live;
        let op = &block.ops[i];
        let reads = op.kind.flags_read();
        // Only flags DEFINITELY written kill upstream liveness.
        let kills = op.kind.flags_must_write();
        // live_in = (live_out - must_write) | reads
        current_live = current_live.difference(kills).union(reads);
    }

    // Forward pass: eliminate dead flag updates.
    let mut eliminated = 0;
    for i in 0..block.ops.len() {
        let live = liveness[i];
        if let Some(flags) = block.ops[i].kind.flags_written_mut() {
            let written = flags.as_set();
            if !written.is_empty() && live.intersection(written).is_empty() {
                *flags = FlagUpdate::None;
                eliminated += 1;
            }
        }
    }

    eliminated
}

// ============================================================================
// Constant Propagation
// ============================================================================

/// Constant propagation within a block.
///
/// Tracks known constant values through the block and replaces register
/// operands with immediate values when possible.
///
/// Returns the number of constants propagated.
pub fn constant_propagation(block: &mut SmirBlock) -> usize {
    // Tracked values are the FULL architectural register value (for x86, a W32
    // definition zero-extends, so we mask to 32 bits and store the
    // zero-extended 64-bit value). Partial-width (8/16-bit) definitions leave
    // the upper bits unknown, so they are NOT tracked.
    let mut constants: HashMap<VReg, i64> = HashMap::new();
    let mut propagated = 0;

    // Mask a value to a width (the register value the op produces / sees).
    fn m(v: i64, w: OpWidth) -> i64 {
        ((v as u64) & w.mask()) as i64
    }
    // Only W32/W64 definitions fully overwrite the destination register.
    fn trackable(w: OpWidth) -> bool {
        matches!(w, OpWidth::W32 | OpWidth::W64)
    }

    for op in &mut block.ops {
        // Discriminants read before the mutable borrow of `op.kind` below.
        let alu = alu_tag(&op.kind);
        let is_shl = matches!(op.kind, OpKind::Shl { .. });
        match &mut op.kind {
            OpKind::Mov { dst, src, width } => {
                if let SrcOperand::Imm(imm) = src {
                    if trackable(*width) {
                        constants.insert(*dst, m(*imm, *width));
                    } else {
                        constants.remove(dst);
                    }
                } else if let SrcOperand::Reg(r) = src {
                    if let Some(&val) = constants.get(r) {
                        *src = SrcOperand::Imm(m(val, *width));
                        propagated += 1;
                        if trackable(*width) {
                            constants.insert(*dst, m(val, *width));
                        } else {
                            constants.remove(dst);
                        }
                    } else {
                        constants.remove(dst);
                    }
                } else {
                    constants.remove(dst);
                }
            }

            OpKind::Add {
                dst,
                src1,
                src2,
                width,
                ..
            }
            | OpKind::Sub {
                dst,
                src1,
                src2,
                width,
                ..
            }
            | OpKind::And {
                dst,
                src1,
                src2,
                width,
                ..
            }
            | OpKind::Or {
                dst,
                src1,
                src2,
                width,
                ..
            }
            | OpKind::Xor {
                dst,
                src1,
                src2,
                width,
                ..
            } => {
                // Substitute a known constant for the register second operand.
                if let SrcOperand::Reg(r) = src2 {
                    if let Some(&val) = constants.get(r) {
                        *src2 = SrcOperand::Imm(m(val, *width));
                        propagated += 1;
                    }
                }
                // Fold the result if both operands are now known constants.
                let folded = if let (Some(&v1), SrcOperand::Imm(v2)) = (constants.get(src1), &*src2)
                {
                    let a = (v1 as u64) & width.mask();
                    let b = (*v2 as u64) & width.mask();
                    let r = match alu {
                        AluTag::Add => a.wrapping_add(b),
                        AluTag::Sub => a.wrapping_sub(b),
                        AluTag::And => a & b,
                        AluTag::Or => a | b,
                        AluTag::Xor => a ^ b,
                    } & width.mask();
                    Some(r as i64)
                } else {
                    None
                };
                match (folded, trackable(*width)) {
                    (Some(r), true) => {
                        constants.insert(*dst, r);
                    }
                    _ => {
                        constants.remove(dst);
                    }
                }
            }

            OpKind::Shl {
                dst,
                src,
                amount,
                width,
                ..
            }
            | OpKind::Shr {
                dst,
                src,
                amount,
                width,
                ..
            } => {
                if let SrcOperand::Reg(r) = amount {
                    if let Some(&val) = constants.get(r) {
                        *amount = SrcOperand::Imm(val);
                        propagated += 1;
                    }
                }
                let folded = if let (Some(&v), SrcOperand::Imm(a)) = (constants.get(src), &*amount)
                {
                    let count_mask = (width.bits() - 1) as u64;
                    let cnt = (*a as u64) & count_mask;
                    let base = (v as u64) & width.mask();
                    let r = if is_shl { base << cnt } else { base >> cnt } & width.mask();
                    Some(r as i64)
                } else {
                    None
                };
                match (folded, trackable(*width)) {
                    (Some(r), true) => {
                        constants.insert(*dst, r);
                    }
                    _ => {
                        constants.remove(dst);
                    }
                }
            }

            OpKind::Load { dst, .. } | OpKind::AtomicLoad { dst, .. } => {
                // Loads produce unknown values.
                constants.remove(dst);
            }

            _ => {
                // For other ops, invalidate destinations.
                for dst in op.kind.dests() {
                    constants.remove(&dst);
                }
            }
        }
    }

    propagated
}

/// Small discriminant for the ALU constant-fold in `constant_propagation`.
#[derive(Clone, Copy)]
enum AluTag {
    Add,
    Sub,
    And,
    Or,
    Xor,
}

fn alu_tag(kind: &OpKind) -> AluTag {
    match kind {
        OpKind::Sub { .. } => AluTag::Sub,
        OpKind::And { .. } => AluTag::And,
        OpKind::Or { .. } => AluTag::Or,
        OpKind::Xor { .. } => AluTag::Xor,
        // Add and anything else (the tag is only consulted in the ALU arm).
        _ => AluTag::Add,
    }
}

// ============================================================================
// Constant Folding
// ============================================================================

/// Fold constant expressions at compile time.
///
/// Evaluates operations where all operands are constants and replaces them
/// with simple moves.
///
/// Returns the number of expressions folded.
pub fn constant_folding(block: &mut SmirBlock) -> usize {
    let mut folded = 0;

    for i in 0..block.ops.len() {
        // Rewrites that turn a flag-setting op into a flag-less `Mov` are only
        // legal when the op's flags are dead (`FlagUpdate::None`, established by
        // `dead_flag_elimination`). Shift-by-0 is exempt: x86 leaves flags
        // untouched on a zero count, so the `Mov` is flag-equivalent regardless.
        let new_kind = match &block.ops[i].kind {
            // Add with two immediates
            OpKind::Add {
                dst,
                src1,
                src2: SrcOperand::Imm(v2),
                width,
                flags,
            } if matches!(src1, VReg::Imm(..)) && matches!(flags, FlagUpdate::None) => {
                if let VReg::Imm(v1) = src1 {
                    let result = ((*v1 as u64).wrapping_add(*v2 as u64)) & width.mask();
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Imm(result as i64),
                        width: *width,
                    })
                } else {
                    None
                }
            }

            // Sub with two immediates
            OpKind::Sub {
                dst,
                src1,
                src2: SrcOperand::Imm(v2),
                width,
                flags,
            } if matches!(src1, VReg::Imm(..)) && matches!(flags, FlagUpdate::None) => {
                if let VReg::Imm(v1) = src1 {
                    let result = ((*v1 as u64).wrapping_sub(*v2 as u64)) & width.mask();
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Imm(result as i64),
                        width: *width,
                    })
                } else {
                    None
                }
            }

            // And with zero -> 0
            OpKind::And {
                dst,
                src2: SrcOperand::Imm(0),
                width,
                flags,
                ..
            } if matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Imm(0),
                width: *width,
            }),

            // And with -1 (all bits) -> mov src1
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Imm(-1),
                width,
                flags,
            } if matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Reg(*src1),
                width: *width,
            }),

            // Or with zero -> mov src1
            OpKind::Or {
                dst,
                src1,
                src2: SrcOperand::Imm(0),
                width,
                flags,
            } if matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Reg(*src1),
                width: *width,
            }),

            // Xor with zero -> mov src1
            OpKind::Xor {
                dst,
                src1,
                src2: SrcOperand::Imm(0),
                width,
                flags,
            } if matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Reg(*src1),
                width: *width,
            }),

            // Xor of same register -> 0
            OpKind::Xor {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width,
                flags,
            } if src1 == src2 && matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Imm(0),
                width: *width,
            }),

            // Sub of same register -> 0
            OpKind::Sub {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width,
                flags,
            } if src1 == src2 && matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Imm(0),
                width: *width,
            }),

            // And/Or of a register with itself -> mov src1 (idempotent).
            OpKind::And {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width,
                flags,
            }
            | OpKind::Or {
                dst,
                src1,
                src2: SrcOperand::Reg(src2),
                width,
                flags,
            } if src1 == src2 && matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Reg(*src1),
                width: *width,
            }),

            // Multiply by 1 -> mov src1 (no high half, flags dead).
            OpKind::MulU {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Imm(1),
                width,
                flags: FlagUpdate::None,
            }
            | OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Imm(1),
                width,
                flags: FlagUpdate::None,
            } => Some(OpKind::Mov {
                dst: *dst_lo,
                src: SrcOperand::Reg(*src1),
                width: *width,
            }),

            // Multiply by 0 -> mov 0 (no high half, flags dead).
            OpKind::MulU {
                dst_lo,
                dst_hi: None,
                src2: SrcOperand::Imm(0),
                width,
                flags: FlagUpdate::None,
                ..
            }
            | OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src2: SrcOperand::Imm(0),
                width,
                flags: FlagUpdate::None,
                ..
            } => Some(OpKind::Mov {
                dst: *dst_lo,
                src: SrcOperand::Imm(0),
                width: *width,
            }),

            // Shift by zero -> mov src (flags untouched on x86 zero count).
            OpKind::Shl {
                dst,
                src,
                amount: SrcOperand::Imm(0),
                width,
                ..
            }
            | OpKind::Shr {
                dst,
                src,
                amount: SrcOperand::Imm(0),
                width,
                ..
            }
            | OpKind::Sar {
                dst,
                src,
                amount: SrcOperand::Imm(0),
                width,
                ..
            } => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Reg(*src),
                width: *width,
            }),

            // Add zero -> mov src1
            OpKind::Add {
                dst,
                src1,
                src2: SrcOperand::Imm(0),
                width,
                flags,
            } if matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Reg(*src1),
                width: *width,
            }),

            // Sub zero -> mov src1
            OpKind::Sub {
                dst,
                src1,
                src2: SrcOperand::Imm(0),
                width,
                flags,
            } if matches!(flags, FlagUpdate::None) => Some(OpKind::Mov {
                dst: *dst,
                src: SrcOperand::Reg(*src1),
                width: *width,
            }),

            _ => None,
        };

        if let Some(new_kind) = new_kind {
            block.ops[i].kind = new_kind;
            folded += 1;
        }
    }

    folded
}

// ============================================================================
// Dead Code Elimination
// ============================================================================

/// Eliminate dead code.
///
/// Removes operations whose results are never used and have no side effects.
///
/// Returns the number of operations eliminated.
pub fn dead_code_elimination(block: &mut SmirBlock) -> usize {
    // Bare per-block use: seed only from the terminator (legacy contract). JIT
    // regions must go through `optimize_function`, which supplies a
    // frontier-aware register live-out via `dead_code_elimination_with`.
    dead_code_elimination_with(block, &HashSet::new())
}

/// Eliminate dead operations given the registers live on block exit.
///
/// An op is kept when any of its destinations is still used downstream (live),
/// or it has memory/side-effects, or it still writes a live flag (after
/// `dead_flag_elimination` has cleared the dead ones). x86 partial-register
/// writes are treated as read-modify-write so they keep the prior definition
/// live. Returns the number of operations removed.
pub fn dead_code_elimination_with(block: &mut SmirBlock, live_out: &HashSet<VReg>) -> usize {
    // Values used by something we must keep, seeded with the live-out set and
    // the terminator's own register uses.
    let mut used: HashSet<VReg> = live_out.clone();
    for u in terminator_reg_uses(&block.terminator) {
        used.insert(u);
    }

    // Backward pass to find all used values.
    for op in block.ops.iter().rev() {
        let dests = op.kind.dests();
        let dest_live = dests.is_empty() || dests.iter().any(|d| used.contains(d));
        let keep = dest_live || op.kind.has_side_effects() || !op.kind.flags_written().is_empty();

        if keep {
            for src in op.kind.source_vregs() {
                used.insert(src);
            }
            // A partial-width write merges into (reads) its destination.
            if !op_fully_defines(&op.kind) {
                for d in &dests {
                    used.insert(*d);
                }
            }
        }
    }

    // Remove ops that are neither live, side-effecting, nor flag-producing.
    let before = block.ops.len();
    block.ops.retain(|op| {
        let dests = op.kind.dests();
        dests.is_empty()
            || dests.iter().any(|d| used.contains(d))
            || op.kind.has_side_effects()
            || !op.kind.flags_written().is_empty()
    });

    before - block.ops.len()
}

// ============================================================================
// Strength Reduction
// ============================================================================

/// Strength reduction transformations.
///
/// Replaces expensive operations with cheaper equivalents:
/// - Multiply by power of 2 -> shift left
/// - Unsigned divide by power of 2 -> shift right
///
/// Returns the number of reductions applied.
pub fn strength_reduction(block: &mut SmirBlock) -> usize {
    let mut reductions = 0;

    for op in &mut block.ops {
        let new_kind = match &op.kind {
            // Multiply by power of 2 -> shift. Only legal when there is no
            // high-half result to produce (`dst_hi == None`) and the multiply's
            // flags are dead (a shift's CF/OF differ from MUL/IMUL's), which
            // `dead_flag_elimination` establishes as `FlagUpdate::None`.
            OpKind::MulU {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Imm(imm),
                width,
                flags: FlagUpdate::None,
            }
            | OpKind::MulS {
                dst_lo,
                dst_hi: None,
                src1,
                src2: SrcOperand::Imm(imm),
                width,
                flags: FlagUpdate::None,
            } if *imm > 0 && (*imm as u64).is_power_of_two() => {
                let shift = (*imm as u64).trailing_zeros() as i64;
                Some(OpKind::Shl {
                    dst: *dst_lo,
                    src: *src1,
                    amount: SrcOperand::Imm(shift),
                    width: *width,
                    flags: FlagUpdate::None,
                })
            }

            // Unsigned divide by power of 2 -> shift right. Only legal when the
            // remainder is not needed (`rem == None`); the quotient of an
            // unsigned divide by 2^k is exactly `src >> k`.
            OpKind::DivU {
                quot,
                rem: None,
                src1,
                src2: SrcOperand::Imm(imm),
                width,
                flags: FlagUpdate::None,
            } if *imm > 0 && (*imm as u64).is_power_of_two() => {
                let shift = (*imm as u64).trailing_zeros() as i64;
                Some(OpKind::Shr {
                    dst: *quot,
                    src: *src1,
                    amount: SrcOperand::Imm(shift),
                    width: *width,
                    flags: FlagUpdate::None,
                })
            }

            _ => None,
        };

        if let Some(new_kind) = new_kind {
            op.kind = new_kind;
            reductions += 1;
        }
    }

    reductions
}

// ============================================================================
// Copy Propagation
// ============================================================================

/// Apply `f` to every PURE-SOURCE register operand of `kind` (operands that are
/// read but never also written — so never a destination or read-modify-write
/// field). Returns how many operands changed. Address operands and RMW fields
/// (Shld/Shrd dst, Xchg, CMove dst, accumulators) are intentionally left
/// untouched: a missed rewrite only forgoes an optimization, never changes
/// semantics.
fn rewrite_pure_src_vregs(kind: &mut OpKind, f: &dyn Fn(VReg) -> VReg) -> usize {
    let mut n = 0usize;
    let mut do_v = |v: &mut VReg, n: &mut usize| {
        let nv = f(*v);
        if nv != *v {
            *v = nv;
            *n += 1;
        }
    };
    let mut do_s = |s: &mut SrcOperand, n: &mut usize| {
        if let SrcOperand::Reg(r) = s {
            let nv = f(*r);
            if nv != *r {
                *s = SrcOperand::Reg(nv);
                *n += 1;
            }
        }
    };
    match kind {
        OpKind::Add { src1, src2, .. }
        | OpKind::Sub { src1, src2, .. }
        | OpKind::Adc { src1, src2, .. }
        | OpKind::Sbb { src1, src2, .. }
        | OpKind::And { src1, src2, .. }
        | OpKind::Or { src1, src2, .. }
        | OpKind::Xor { src1, src2, .. }
        | OpKind::AndNot { src1, src2, .. }
        | OpKind::Cmp { src1, src2, .. }
        | OpKind::Test { src1, src2, .. }
        | OpKind::MulU { src1, src2, .. }
        | OpKind::MulS { src1, src2, .. }
        | OpKind::DivU { src1, src2, .. }
        | OpKind::DivS { src1, src2, .. } => {
            do_v(src1, &mut n);
            do_s(src2, &mut n);
        }
        OpKind::Mov { src, .. } => do_s(src, &mut n),
        OpKind::Neg { src, .. }
        | OpKind::Inc { src, .. }
        | OpKind::Dec { src, .. }
        | OpKind::Not { src, .. }
        | OpKind::Cwd { src, .. }
        | OpKind::Bsf { src, .. }
        | OpKind::Bsr { src, .. }
        | OpKind::Clz { src, .. }
        | OpKind::Ctz { src, .. }
        | OpKind::Popcnt { src, .. }
        | OpKind::Bswap { src, .. }
        | OpKind::Rbit { src, .. }
        | OpKind::ZeroExtend { src, .. }
        | OpKind::SignExtend { src, .. }
        | OpKind::Truncate { src, .. } => do_v(src, &mut n),
        OpKind::Shl { src, amount, .. }
        | OpKind::Shr { src, amount, .. }
        | OpKind::Sar { src, amount, .. }
        | OpKind::Rol { src, amount, .. }
        | OpKind::Ror { src, amount, .. }
        | OpKind::Rcl { src, amount, .. }
        | OpKind::Rcr { src, amount, .. } => {
            do_v(src, &mut n);
            do_s(amount, &mut n);
        }
        // Shld/Shrd: `dst` is read-modify-write (skip); `src` and `amount` are
        // pure sources.
        OpKind::Shld { src, amount, .. } | OpKind::Shrd { src, amount, .. } => {
            do_v(src, &mut n);
            do_s(amount, &mut n);
        }
        OpKind::CMove { src, .. } => do_v(src, &mut n),
        OpKind::Bt { src, index, .. }
        | OpKind::Bts { src, index, .. }
        | OpKind::Btr { src, index, .. }
        | OpKind::Btc { src, index, .. } => {
            do_v(src, &mut n);
            do_s(index, &mut n);
        }
        OpKind::Select {
            cond,
            src_true,
            src_false,
            ..
        } => {
            do_v(cond, &mut n);
            do_v(src_true, &mut n);
            do_v(src_false, &mut n);
        }
        _ => {}
    }
    n
}

/// Copy propagation within a block.
///
/// For `mov dst, reg(src)` (a full-width register copy), later pure-source uses
/// of `dst` are rewritten to `src` until `dst` or `src` is redefined. This
/// turns the very common lifted pattern `mov vtmp, r; OP _, vtmp` into
/// `OP _, r`, letting dead-code elimination drop the now-unused copy.
///
/// Returns the number of operand rewrites performed.
pub fn copy_propagation(block: &mut SmirBlock) -> usize {
    // `copies[d] = s` means "register d currently holds the same value as s".
    let mut copies: HashMap<VReg, VReg> = HashMap::new();
    let mut count = 0;

    for op in &mut block.ops {
        // 1) Rewrite pure-source uses through the copy map.
        if !copies.is_empty() {
            let map = &copies;
            count += rewrite_pure_src_vregs(&mut op.kind, &|v| map.get(&v).copied().unwrap_or(v));
        }

        // 2) Invalidate copies killed by this op's destinations.
        let dests = op.kind.dests();
        if !dests.is_empty() {
            for d in &dests {
                copies.remove(d);
            }
            copies.retain(|_, val| !dests.contains(val));
        }

        // 3) Record a new register copy. ONLY W64 moves give full-register
        //    equality (`rcx == rax`); a W32 `mov ecx, eax` yields
        //    `ecx == zero_extend(low32(eax))`, which is not equal to `eax` when
        //    its upper bits are set, so substituting it into a 64-bit use would
        //    be wrong. Restrict to W64 to stay correct regardless of use width.
        if let OpKind::Mov {
            dst,
            src: SrcOperand::Reg(s),
            width: OpWidth::W64,
        } = &op.kind
        {
            if dst != s {
                copies.insert(*dst, *s);
            }
        }
    }

    count
}

// ============================================================================
// Branch Folding
// ============================================================================

/// Fold degenerate conditional branches and drop unreachable blocks.
///
/// - A `CondBranch` whose two targets are identical becomes an unconditional
///   `Branch` (the condition no longer matters).
/// - Blocks not reachable from the entry are removed (they can arise after
///   constant folding / block merging).
///
/// Returns the number of transformations applied.
pub fn branch_folding(func: &mut SmirFunction) -> usize {
    let mut changes = 0;

    // Same-target conditional branches -> unconditional.
    for block in &mut func.blocks {
        if let Terminator::CondBranch {
            true_target,
            false_target,
            ..
        } = &block.terminator
        {
            if true_target == false_target {
                let target = *true_target;
                block.set_terminator(Terminator::Branch { target });
                changes += 1;
            }
        }
    }

    // Reachability from the entry.
    let mut reachable: HashSet<BlockId> = HashSet::new();
    let mut stack = vec![func.entry];
    while let Some(id) = stack.pop() {
        if !reachable.insert(id) {
            continue;
        }
        if let Some(b) = func.blocks.iter().find(|b| b.id == id) {
            for s in b.terminator.successors() {
                if !reachable.contains(&s) {
                    stack.push(s);
                }
            }
        }
    }
    let before = func.blocks.len();
    func.blocks.retain(|b| reachable.contains(&b.id));
    changes += before - func.blocks.len();

    changes
}

// ============================================================================
// Block Merging
// ============================================================================

/// Merge adjacent blocks with unconditional jumps.
///
/// When a block ends with an unconditional branch to a block with only one
/// predecessor, merge them together.
///
/// Returns the number of blocks merged.
pub fn block_merging(func: &mut SmirFunction) -> usize {
    if func.blocks.len() < 2 {
        return 0;
    }

    // Build predecessor count
    let mut pred_count: HashMap<BlockId, usize> = HashMap::new();

    for block in &func.blocks {
        match &block.terminator {
            Terminator::Branch { target } => {
                *pred_count.entry(*target).or_default() += 1;
            }
            Terminator::CondBranch {
                true_target,
                false_target,
                ..
            } => {
                *pred_count.entry(*true_target).or_default() += 1;
                *pred_count.entry(*false_target).or_default() += 1;
            }
            Terminator::Switch {
                targets, default, ..
            } => {
                for target in targets {
                    *pred_count.entry(*target).or_default() += 1;
                }
                *pred_count.entry(*default).or_default() += 1;
            }
            _ => {}
        }
    }

    // Find blocks to merge
    let mut merge_pairs: Vec<(BlockId, BlockId)> = Vec::new();

    for block in &func.blocks {
        if let Terminator::Branch { target } = &block.terminator {
            // Only merge if target has single predecessor
            if pred_count.get(target) == Some(&1) && *target != block.id {
                merge_pairs.push((block.id, *target));
            }
        }
    }

    let merged_count = merge_pairs.len();

    // Perform merges
    for (from, to) in merge_pairs {
        let from_idx = func.blocks.iter().position(|b| b.id == from);
        let to_idx = func.blocks.iter().position(|b| b.id == to);

        if let (Some(from_idx), Some(to_idx)) = (from_idx, to_idx) {
            // Get ops and terminator from target block
            let to_ops = func.blocks[to_idx].ops.clone();
            let to_term = func.blocks[to_idx].terminator.clone();

            // Append to source block
            func.blocks[from_idx].ops.extend(to_ops);
            func.blocks[from_idx].terminator = to_term;

            // Mark target block for removal
            func.blocks[to_idx].ops.clear();
            func.blocks[to_idx].terminator = Terminator::Unreachable;
        }
    }

    // Remove empty blocks (but keep entry block)
    func.blocks.retain(|b| {
        b.id == func.entry || !b.ops.is_empty() || !matches!(b.terminator, Terminator::Unreachable)
    });

    merged_count
}

// ============================================================================
// Redundant Load Elimination
// ============================================================================

/// Eliminate redundant loads.
///
/// When a value is loaded from memory and the same address is loaded again
/// (without an intervening store), replace the second load with a move.
///
/// Returns the number of redundant loads eliminated.
pub fn redundant_load_elimination(func: &mut SmirFunction) -> usize {
    let mut eliminated = 0;

    for block in &mut func.blocks {
        eliminated += redundant_load_elimination_block(block);
    }

    eliminated
}

// ============================================================================
// Vector Alignment Inference
// ============================================================================

/// Infer vector alignment hints for VLoad/VStore ops.
pub fn vector_alignment_inference(func: &mut SmirFunction) -> usize {
    let mut inferred = 0;

    for block in &mut func.blocks {
        inferred += vector_alignment_inference_block(block);
    }

    inferred
}

fn vector_alignment_inference_block(block: &mut SmirBlock) -> usize {
    let mut inferred = 0;
    let mut alignments = seed_x86_alignments();

    for op in &mut block.ops {
        inferred += apply_vec_align_hint(op, &alignments);
        update_pointer_alignment(op, &mut alignments);
    }

    inferred
}

fn seed_x86_alignments() -> HashMap<VReg, usize> {
    let mut alignments = HashMap::new();
    alignments.insert(VReg::Arch(ArchReg::X86(X86Reg::Rsp)), 16);
    alignments.insert(VReg::Arch(ArchReg::X86(X86Reg::Rbp)), 16);
    alignments
}

fn apply_vec_align_hint(op: &mut SmirOp, alignments: &HashMap<VReg, usize>) -> usize {
    let (addr, width) = match &op.kind {
        OpKind::VLoad { addr, width, .. } | OpKind::VStore { addr, width, .. } => (addr, width),
        _ => return 0,
    };

    match op.x86_hint {
        None | Some(X86OpHint::VecAlign(X86VecAlign::Unaligned)) => {}
        _ => return 0,
    }

    let required = vec_width_bytes(*width);
    if let Some(alignment) = address_alignment(addr, alignments) {
        if alignment >= required {
            op.x86_hint = Some(X86OpHint::VecAlign(X86VecAlign::Aligned));
            return 1;
        }
    }

    0
}

fn update_pointer_alignment(op: &SmirOp, alignments: &mut HashMap<VReg, usize>) {
    let mut computed = HashMap::new();

    match &op.kind {
        OpKind::Mov { dst, src, width } if *width == OpWidth::W64 => {
            if let Some(src_reg) = src.as_reg() {
                if let Some(&alignment) = alignments.get(&src_reg) {
                    computed.insert(*dst, alignment);
                }
            } else if let Some(imm) = src.as_imm() {
                if imm >= 0 {
                    computed.insert(*dst, alignment_from_addr(imm as u64));
                }
            }
        }
        OpKind::Add {
            dst,
            src1,
            src2,
            width,
            ..
        }
        | OpKind::Sub {
            dst,
            src1,
            src2,
            width,
            ..
        } if *width == OpWidth::W64 => {
            if let Some(&src_align) = alignments.get(src1) {
                if let Some(imm) = src2.as_imm() {
                    computed.insert(*dst, gcd(src_align, imm.unsigned_abs() as usize));
                } else if let Some(src2_reg) = src2.as_reg() {
                    if let Some(&src2_align) = alignments.get(&src2_reg) {
                        computed.insert(*dst, gcd(src_align, src2_align));
                    }
                }
            }
        }
        OpKind::Shl {
            dst,
            src,
            amount,
            width,
            ..
        } if *width == OpWidth::W64 => {
            if let (Some(&src_align), Some(shift)) = (alignments.get(src), amount.as_imm()) {
                if let Ok(shift) = u32::try_from(shift) {
                    if let Some(alignment) = src_align.checked_shl(shift) {
                        computed.insert(*dst, alignment);
                    }
                }
            }
        }
        OpKind::And {
            dst,
            src1,
            src2,
            width,
            ..
        } if *width == OpWidth::W64 => {
            if let Some(imm) = src2.as_imm() {
                let mask = imm as u64;
                let mut alignment = if mask == 0 {
                    1
                } else {
                    1usize << mask.trailing_zeros()
                };
                if let Some(&src_align) = alignments.get(src1) {
                    alignment = alignment.max(src_align);
                }
                computed.insert(*dst, alignment);
            }
        }
        OpKind::CMove {
            dst, src, width, ..
        } if *width == OpWidth::W64 => {
            if let (Some(&dst_align), Some(&src_align)) = (alignments.get(dst), alignments.get(src))
            {
                computed.insert(*dst, gcd(dst_align, src_align));
            }
        }
        OpKind::Select {
            dst,
            src_true,
            src_false,
            width,
            ..
        } if *width == OpWidth::W64 => {
            if let (Some(&a), Some(&b)) = (alignments.get(src_true), alignments.get(src_false)) {
                computed.insert(*dst, gcd(a, b));
            }
        }
        OpKind::Lea { dst, addr } => {
            if let Some(alignment) = address_alignment(addr, alignments) {
                computed.insert(*dst, alignment);
            }
        }
        _ => {}
    }

    for dst in op.kind.dests() {
        if let Some(&alignment) = computed.get(&dst) {
            alignments.insert(dst, alignment);
        } else {
            alignments.remove(&dst);
        }
    }
}

fn vec_width_bytes(width: VecWidth) -> usize {
    match width {
        VecWidth::V64 => 8,
        VecWidth::V128 => 16,
        VecWidth::V256 => 32,
        VecWidth::V512 => 64,
    }
}

fn address_alignment(addr: &Address, alignments: &HashMap<VReg, usize>) -> Option<usize> {
    match addr {
        Address::Direct(base) => alignments.get(base).copied(),
        Address::BaseOffset { base, offset, .. } => {
            let base_align = alignments.get(base).copied()?;
            Some(gcd(base_align, offset.unsigned_abs() as usize))
        }
        Address::BaseIndexScale {
            base,
            index,
            scale,
            disp,
            ..
        } => {
            let index_align = alignments.get(index).copied()?;
            let scaled = index_align.checked_mul(*scale as usize)?;
            let mut alignment = scaled;
            if let Some(base_reg) = base {
                let base_align = alignments.get(base_reg).copied()?;
                alignment = gcd(alignment, base_align);
            }
            alignment = gcd(alignment, (*disp as i64).unsigned_abs() as usize);
            Some(alignment)
        }
        Address::PcRel { offset, base, .. } => {
            let base_addr = match base {
                Some(base_addr) => *base_addr as i128,
                None => return None,
            };
            let target = base_addr + *offset as i128;
            if target < 0 {
                None
            } else {
                Some(alignment_from_addr(target as u64))
            }
        }
        Address::Absolute(addr) => Some(alignment_from_addr(*addr)),
        _ => None,
    }
}

fn alignment_from_addr(addr: u64) -> usize {
    if addr == 0 {
        return 1;
    }
    1usize << addr.trailing_zeros()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}

fn redundant_load_elimination_block(block: &mut SmirBlock) -> usize {
    // Track what's currently in registers from memory
    // Key: (base_vreg, offset, width), Value: VReg holding the loaded value
    let mut mem_to_reg: HashMap<(Option<VReg>, i64, MemWidth), VReg> = HashMap::new();
    let mut eliminated = 0;

    let mut new_ops = Vec::new();

    for op in &block.ops {
        match &op.kind {
            OpKind::Load {
                dst,
                addr,
                width,
                sign: _,
            } => {
                // Only loads from a key-able address (Direct/BaseOffset/Absolute)
                // are candidates. Complex addresses (BaseIndexScale, PcRel) are
                // NOT tracked — a single sentinel key would make distinct
                // addresses (e.g. [rsi+rdx-16] vs [rsi+rdx-8]) collide and
                // wrongly forward one load's value to the other.
                if let Some(key) = address_key(addr, *width) {
                    if let Some(&existing) = mem_to_reg.get(&key) {
                        new_ops.push(SmirOp {
                            id: op.id,
                            guest_pc: op.guest_pc,
                            kind: OpKind::Mov {
                                dst: *dst,
                                src: SrcOperand::Reg(existing),
                                width: width.to_op_width().unwrap_or(OpWidth::W64),
                            },
                            x86_hint: None,
                        });
                        eliminated += 1;
                    } else {
                        mem_to_reg.insert(key, *dst);
                        new_ops.push(op.clone());
                    }
                } else {
                    new_ops.push(op.clone());
                }
            }

            OpKind::Store { .. }
            | OpKind::AtomicStore { .. }
            | OpKind::AtomicRmw { .. }
            | OpKind::Cas { .. }
            | OpKind::AtomicCmpXadd { .. }
            | OpKind::StoreExclusive { .. }
            | OpKind::Fence { .. }
            | OpKind::IoIn { .. }
            | OpKind::IoOut { .. }
            | OpKind::Syscall { .. } => {
                // Any store / atomic / I/O / syscall may alias an arbitrary
                // address, so conservatively drop every cached load.
                mem_to_reg.clear();
                new_ops.push(op.clone());
            }

            _ => {
                new_ops.push(op.clone());
            }
        }

        // Invalidate any cached load whose BASE register this op redefines: once
        // the base changes, the cached `(base, offset)` no longer names the same
        // memory (e.g. `load [rsi-8]; lea rsi,[rsi-32]; load [rsi-8]` are two
        // different addresses). Without this, the second load would be wrongly
        // forwarded from the first.
        for d in op.kind.dests() {
            mem_to_reg.retain(|key, _| key.0 != Some(d));
        }
    }

    block.ops = new_ops;
    eliminated
}

/// Create a key for memory-address tracking, or `None` for addresses we do not
/// track (complex forms whose equality we cannot cheaply decide). Returning
/// `None` — never a shared sentinel — is what keeps distinct untracked
/// addresses from aliasing each other.
fn address_key(addr: &Address, width: MemWidth) -> Option<(Option<VReg>, i64, MemWidth)> {
    match addr {
        Address::Direct(r) => Some((Some(*r), 0, width)),
        Address::BaseOffset { base, offset, .. } => Some((Some(*base), *offset, width)),
        Address::Absolute(a) => Some((None, *a as i64, width)),
        _ => None,
    }
}

// ============================================================================
// OpKind Helper Methods for Optimization
// ============================================================================

impl OpKind {
    /// Get mutable reference to flag update field
    pub fn flags_written_mut(&mut self) -> Option<&mut FlagUpdate> {
        match self {
            OpKind::Add { flags, .. }
            | OpKind::Sub { flags, .. }
            | OpKind::Adc { flags, .. }
            | OpKind::Sbb { flags, .. }
            | OpKind::Neg { flags, .. }
            | OpKind::Inc { flags, .. }
            | OpKind::Dec { flags, .. }
            | OpKind::And { flags, .. }
            | OpKind::Or { flags, .. }
            | OpKind::Xor { flags, .. }
            | OpKind::AndNot { flags, .. }
            | OpKind::Shl { flags, .. }
            | OpKind::Shr { flags, .. }
            | OpKind::Sar { flags, .. }
            | OpKind::Shld { flags, .. }
            | OpKind::Shrd { flags, .. }
            | OpKind::Rol { flags, .. }
            | OpKind::Ror { flags, .. }
            | OpKind::Rcl { flags, .. }
            | OpKind::Rcr { flags, .. }
            | OpKind::Bsf { flags, .. }
            | OpKind::Bsr { flags, .. }
            | OpKind::Bextr { flags, .. }
            | OpKind::Bzhi { flags, .. }
            | OpKind::MulU { flags, .. }
            | OpKind::MulS { flags, .. } => Some(flags),
            _ => None,
        }
    }

    /// Get the flags written by this operation (the flags it may define).
    pub fn flags_written(&self) -> FlagSet {
        match self {
            OpKind::Add { flags, .. }
            | OpKind::Sub { flags, .. }
            | OpKind::Adc { flags, .. }
            | OpKind::Sbb { flags, .. }
            | OpKind::Neg { flags, .. }
            | OpKind::And { flags, .. }
            | OpKind::Or { flags, .. }
            | OpKind::Xor { flags, .. }
            | OpKind::AndNot { flags, .. }
            | OpKind::Shl { flags, .. }
            | OpKind::Shr { flags, .. }
            | OpKind::Sar { flags, .. }
            | OpKind::Shld { flags, .. }
            | OpKind::Shrd { flags, .. }
            | OpKind::Bsf { flags, .. }
            | OpKind::Bsr { flags, .. }
            | OpKind::Bextr { flags, .. }
            | OpKind::Bzhi { flags, .. }
            | OpKind::MulU { flags, .. }
            | OpKind::MulS { flags, .. } => flags.as_set(),

            OpKind::Rol { flags, .. }
            | OpKind::Ror { flags, .. }
            | OpKind::Rcl { flags, .. }
            | OpKind::Rcr { flags, .. } => {
                flags.as_set().intersection(FlagSet::CF.union(FlagSet::OF))
            }

            // INC/DEC update OF/SF/ZF/AF/PF but PRESERVE CF (their defining
            // difference from ADD/SUB by 1). Never report CF as written.
            OpKind::Inc { flags, .. } | OpKind::Dec { flags, .. } => {
                flags.as_set().difference(FlagSet::CF)
            }

            // Cmp, Test, and CMPccXADD update all x86 arithmetic flags.
            OpKind::Cmp { .. } | OpKind::Test { .. } | OpKind::AtomicCmpXadd { .. } => {
                FlagSet::ALL_X86
            }

            // Bit test updates CF
            OpKind::Bt { .. } => FlagSet::CF,

            OpKind::SetCF { .. } | OpKind::CmcCF => FlagSet::CF,

            _ => FlagSet::EMPTY,
        }
    }

    /// Flags this op DEFINITELY writes a defined value to, regardless of its
    /// operands — the set safe to treat as "killed" (overwritten) in backward
    /// flag-liveness. Conservatively smaller than `flags_written` for ops whose
    /// flag effect is operand-conditional or partly undefined: a shift/rotate
    /// by a variable count writes nothing when the count is 0, and MUL/IMUL and
    /// BSF/BSR leave most flags undefined. Using a smaller must-write set can
    /// only keep more upstream flags live (safe), never delete a needed one.
    pub fn flags_must_write(&self) -> FlagSet {
        match self {
            OpKind::Shl { .. }
            | OpKind::Shr { .. }
            | OpKind::Sar { .. }
            | OpKind::Shld { .. }
            | OpKind::Shrd { .. }
            | OpKind::Rol { .. }
            | OpKind::Ror { .. }
            | OpKind::Rcl { .. }
            | OpKind::Rcr { .. }
            | OpKind::MulU { .. }
            | OpKind::MulS { .. }
            | OpKind::Bsf { .. }
            | OpKind::Bsr { .. } => FlagSet::EMPTY,
            _ => self.flags_written(),
        }
    }

    /// Get the flags read by this operation
    pub fn flags_read(&self) -> FlagSet {
        match self {
            // Add/Sub with carry read CF
            OpKind::Adc { .. } | OpKind::Sbb { .. } | OpKind::Rcl { .. } | OpKind::Rcr { .. } => {
                FlagSet::CF
            }

            // Conditional move reads flags based on condition
            OpKind::CMove { cond, .. } | OpKind::SetCC { cond, .. } => {
                FlagState::required_flags(*cond)
            }

            // TestCondition reads flags
            OpKind::TestCondition { cond, .. } => FlagState::required_flags(*cond),

            // Complement carry reads CF
            OpKind::CmcCF => FlagSet::CF,

            // ReadFlags reads all flags
            OpKind::ReadFlags { .. } => FlagSet::ALL_X86,

            _ => FlagSet::EMPTY,
        }
    }

    /// Get source registers used by this operation
    pub fn source_vregs(&self) -> Vec<VReg> {
        let mut result = Vec::new();

        match self {
            OpKind::Add { src1, src2, .. }
            | OpKind::Sub { src1, src2, .. }
            | OpKind::Adc { src1, src2, .. }
            | OpKind::Sbb { src1, src2, .. }
            | OpKind::And { src1, src2, .. }
            | OpKind::Or { src1, src2, .. }
            | OpKind::Xor { src1, src2, .. }
            | OpKind::AndNot { src1, src2, .. }
            | OpKind::Cmp { src1, src2, .. }
            | OpKind::Test { src1, src2, .. } => {
                result.push(*src1);
                if let SrcOperand::Reg(r) = src2 {
                    result.push(*r);
                }
            }

            OpKind::Shld {
                dst: src1,
                src: src3,
                amount: src2,
                ..
            }
            | OpKind::Shrd {
                dst: src1,
                src: src3,
                amount: src2,
                ..
            } => {
                result.push(*src1);
                result.push(*src3);
                if let SrcOperand::Reg(r) = src2 {
                    result.push(*r);
                }
            }

            OpKind::MulU { src1, src2, .. }
            | OpKind::MulS { src1, src2, .. }
            | OpKind::DivU { src1, src2, .. }
            | OpKind::DivS { src1, src2, .. } => {
                result.push(*src1);
                if let SrcOperand::Reg(r) = src2 {
                    result.push(*r);
                }
            }

            OpKind::MulAdd {
                acc, src1, src2, ..
            }
            | OpKind::MulSub {
                acc, src1, src2, ..
            } => {
                result.push(*acc);
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::Bextr { src, control, .. } => {
                result.push(*src);
                result.push(*control);
            }

            OpKind::Bzhi { src, index, .. } => {
                result.push(*src);
                result.push(*index);
            }

            OpKind::Pdep { src, mask, .. } | OpKind::Pext { src, mask, .. } => {
                result.push(*src);
                result.push(*mask);
            }

            OpKind::Neg { src, .. }
            | OpKind::Inc { src, .. }
            | OpKind::Dec { src, .. }
            | OpKind::Not { src, .. }
            | OpKind::Cwd { src, .. }
            | OpKind::Bsf { src, .. }
            | OpKind::Bsr { src, .. }
            | OpKind::Clz { src, .. }
            | OpKind::Ctz { src, .. }
            | OpKind::Popcnt { src, .. }
            | OpKind::Bswap { src, .. }
            | OpKind::Rbit { src, .. } => {
                result.push(*src);
            }

            OpKind::Leave => {
                result.push(VReg::Arch(ArchReg::X86(X86Reg::Rbp)));
            }

            OpKind::Shl { src, amount, .. }
            | OpKind::Shr { src, amount, .. }
            | OpKind::Sar { src, amount, .. }
            | OpKind::Rol { src, amount, .. }
            | OpKind::Ror { src, amount, .. }
            | OpKind::Rcl { src, amount, .. }
            | OpKind::Rcr { src, amount, .. } => {
                result.push(*src);
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }

            // Bidirectional shift: both `src` and `amount` are SrcOperand.
            OpKind::BidirShift { src, amount, .. } => {
                if let SrcOperand::Reg(r) = src {
                    result.push(*r);
                }
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }

            // Saturating clamp: `src` is a SrcOperand (register or immediate).
            OpKind::SatN { src, .. } => {
                if let SrcOperand::Reg(r) = src {
                    result.push(*r);
                }
            }

            OpKind::Bt { src, index, .. }
            | OpKind::Bts { src, index, .. }
            | OpKind::Btr { src, index, .. }
            | OpKind::Btc { src, index, .. } => {
                result.push(*src);
                if let SrcOperand::Reg(r) = index {
                    result.push(*r);
                }
            }

            OpKind::Bfx { src, .. } => {
                result.push(*src);
            }

            OpKind::Bfi { src, dst_in, .. } => {
                result.push(*src);
                result.push(*dst_in);
            }

            OpKind::Mov { src, .. } => {
                if let SrcOperand::Reg(r) = src {
                    result.push(*r);
                }
            }

            OpKind::CMove { src, .. } => {
                result.push(*src);
            }

            OpKind::Select {
                cond,
                src_true,
                src_false,
                ..
            } => {
                result.push(*cond);
                result.push(*src_true);
                result.push(*src_false);
            }

            OpKind::ZeroExtend { src, .. }
            | OpKind::SignExtend { src, .. }
            | OpKind::Truncate { src, .. } => {
                result.push(*src);
            }

            OpKind::Lea { addr, .. } => {
                result.extend(addr.regs());
            }

            OpKind::Xchg { reg1, reg2, .. } => {
                result.push(*reg1);
                result.push(*reg2);
            }

            OpKind::Load { addr, .. }
            | OpKind::AtomicLoad { addr, .. }
            | OpKind::LoadExclusive { addr, .. } => {
                result.extend(addr.regs());
            }

            OpKind::Store { src, addr, .. } | OpKind::AtomicStore { src, addr, .. } => {
                result.push(*src);
                result.extend(addr.regs());
            }

            // Predicated load: reads the predicate `cond` and the address base
            // register(s). The `dst` is conditionally written (in dests()).
            OpKind::PredLoad { cond, addr, .. } => {
                result.push(*cond);
                result.extend(addr.regs());
            }

            // Predicated store: reads the predicate `cond`, the source operand
            // (when a register), and the address base register(s).
            OpKind::PredStore {
                src, cond, addr, ..
            } => {
                result.push(*cond);
                if let SrcOperand::Reg(r) = src {
                    result.push(*r);
                }
                result.extend(addr.regs());
            }

            OpKind::RepStos {
                dst, src, count, ..
            } => {
                result.push(*dst);
                result.push(*src);
                result.push(*count);
            }

            OpKind::RepMovs {
                dst, src, count, ..
            } => {
                result.push(*dst);
                result.push(*src);
                result.push(*count);
            }

            OpKind::LoadPair { addr, .. } => {
                result.extend(addr.regs());
            }

            OpKind::StorePair {
                src1, src2, addr, ..
            } => {
                result.push(*src1);
                result.push(*src2);
                result.extend(addr.regs());
            }

            OpKind::AtomicRmw { addr, src, .. } => {
                result.extend(addr.regs());
                result.push(*src);
            }

            OpKind::Cas {
                addr,
                expected,
                new_val,
                ..
            } => {
                result.extend(addr.regs());
                result.push(*expected);
                result.push(*new_val);
            }

            OpKind::AtomicCmpXadd { addr, cmp, add, .. } => {
                result.extend(addr.regs());
                result.push(*cmp);
                result.push(*add);
            }

            OpKind::StoreExclusive { src, addr, .. } => {
                result.push(*src);
                result.extend(addr.regs());
            }

            OpKind::IoIn { port, .. } => {
                result.push(*port);
            }

            OpKind::IoOut { port, value, .. } => {
                result.push(*port);
                result.push(*value);
            }

            OpKind::WriteFlags { src } | OpKind::WriteSysReg { src, .. } => {
                result.push(*src);
            }

            OpKind::Syscall { num, args } => {
                result.push(*num);
                result.extend(args.iter().copied());
            }

            // FP operations
            OpKind::FAdd { src1, src2, .. }
            | OpKind::FSub { src1, src2, .. }
            | OpKind::FMul { src1, src2, .. }
            | OpKind::FDiv { src1, src2, .. }
            | OpKind::FMin { src1, src2, .. }
            | OpKind::FMax { src1, src2, .. }
            | OpKind::FCmp { src1, src2, .. }
            | OpKind::HexFp { src1, src2, .. }
            | OpKind::HexFpRecip { src1, src2, .. }
            | OpKind::HexCabacDecBin { src1, src2, .. }
            | OpKind::HexTlbMatch { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::FFma {
                src1, src2, src3, ..
            }
            | OpKind::HexFp3 {
                src1, src2, src3, ..
            }
            | OpKind::HexFpDf {
                src1, src2, src3, ..
            } => {
                result.push(*src1);
                result.push(*src2);
                result.push(*src3);
            }

            OpKind::HexFpScFma {
                src1,
                src2,
                src3,
                scale,
                ..
            } => {
                result.push(*src1);
                result.push(*src2);
                result.push(*src3);
                result.push(*scale);
            }

            OpKind::RvFp {
                src1,
                src2,
                src3,
                fcsr_src,
                ..
            } => {
                result.push(*src1);
                result.push(*src2);
                result.push(*src3);
                result.push(*fcsr_src);
            }

            OpKind::RvIntCrypto { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::RvVector { rs1, rs2, .. } => {
                result.push(*rs1);
                result.push(*rs2);
            }

            OpKind::FAbs { src, .. }
            | OpKind::FNeg { src, .. }
            | OpKind::FSqrt { src, .. }
            | OpKind::FConvert { src, .. }
            | OpKind::IntToFp { src, .. }
            | OpKind::FpToInt { src, .. }
            | OpKind::FRound { src, .. } => {
                result.push(*src);
            }

            // Vector operations
            OpKind::VAdd { src1, src2, .. }
            | OpKind::VSub { src1, src2, .. }
            | OpKind::VMax { src1, src2, .. }
            | OpKind::VMul { src1, src2, .. }
            | OpKind::VLane { src1, src2, .. }
            | OpKind::VAnd { src1, src2, .. }
            | OpKind::VOr { src1, src2, .. }
            | OpKind::VXor { src1, src2, .. }
            | OpKind::VCmp { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VShift { src, amount, .. } => {
                result.push(*src);
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }

            OpKind::VWidenMul {
                src1,
                src2,
                dst_lo,
                dst_hi,
                acc,
                ..
            }
            | OpKind::VWidenAddSub {
                src1,
                src2,
                dst_lo,
                dst_hi,
                acc,
                ..
            } => {
                result.push(*src1);
                result.push(*src2);
                if *acc {
                    // accumulating form reads the existing destination pair
                    result.push(*dst_lo);
                    result.push(*dst_hi);
                }
            }

            OpKind::VLaneUnary { src, .. } => {
                result.push(*src);
            }

            OpKind::VNavg { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VShiftAcc {
                src, amount, dst, ..
            } => {
                result.push(*src);
                // shift-accumulate reads the existing destination lane
                result.push(*dst);
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }

            OpKind::VPairReduceMul {
                src_lo,
                src_hi,
                src2,
                dst_lo,
                dst_hi,
                acc,
                ..
            }
            | OpKind::VSlideReduceMul {
                src_lo,
                src_hi,
                src2,
                dst_lo,
                dst_hi,
                acc,
                ..
            }
            | OpKind::VRotReduceMulPair {
                src_lo,
                src_hi,
                src2,
                dst_lo,
                dst_hi,
                acc,
                ..
            } => {
                result.push(*src_lo);
                result.push(*src_hi);
                result.push(*src2);
                if *acc {
                    result.push(*dst_lo);
                    result.push(*dst_hi);
                }
            }

            OpKind::VPairPairReduceMul {
                src_lo,
                src_hi,
                src2_lo,
                src2_hi,
                ..
            } => {
                result.push(*src_lo);
                result.push(*src_hi);
                result.push(*src2_lo);
                result.push(*src2_hi);
            }

            OpKind::VReduceMul {
                src1,
                src2,
                dst,
                acc,
                ..
            }
            | OpKind::VMulEvenWiden {
                src1,
                src2,
                dst,
                acc,
                ..
            }
            | OpKind::VMulSubLane {
                src1,
                src2,
                dst,
                acc,
                ..
            } => {
                result.push(*src1);
                result.push(*src2);
                if *acc {
                    result.push(*dst);
                }
            }

            OpKind::VWidenExt { src, .. } => {
                result.push(*src);
            }

            OpKind::VLut {
                src_idx,
                table,
                sel,
                dst,
                oracc,
                ..
            } => {
                result.push(*src_idx);
                result.push(*table);
                if let SrcOperand::Reg(r) = sel {
                    result.push(*r);
                }
                if *oracc {
                    result.push(*dst);
                }
            }

            OpKind::VLut16 {
                src_idx,
                table,
                sel,
                dst_lo,
                dst_hi,
                oracc,
                ..
            } => {
                result.push(*src_idx);
                result.push(*table);
                if let SrcOperand::Reg(r) = sel {
                    result.push(*r);
                }
                if *oracc {
                    result.push(*dst_lo);
                    result.push(*dst_hi);
                }
            }

            OpKind::VShuffVdd {
                src_lo,
                src_hi,
                amount,
                ..
            }
            | OpKind::VDealVdd {
                src_lo,
                src_hi,
                amount,
                ..
            } => {
                result.push(*src_lo);
                result.push(*src_hi);
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }

            OpKind::VShuffleEOPair { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            // In-place dual-register shuffle/deal reads AND writes both Vy and Vx.
            OpKind::VShuffleDeal {
                dst_y,
                dst_x,
                amount,
                ..
            } => {
                result.push(*dst_y);
                result.push(*dst_x);
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }

            // vunpacko OR-accumulates the source into the existing dst pair.
            OpKind::VUnpackOAcc {
                src,
                dst_lo,
                dst_hi,
                ..
            } => {
                result.push(*src);
                result.push(*dst_lo);
                result.push(*dst_hi);
            }

            OpKind::VInsertWordR { dst, scalar } => {
                // read-modify-write: preserves the other words of dst.
                result.push(*dst);
                result.push(*scalar);
            }

            OpKind::VExtractWord { src, sel, .. } => {
                result.push(*src);
                result.push(*sel);
            }

            OpKind::VLut4 { src, table, .. } => {
                result.push(*src);
                result.push(*table);
            }

            OpKind::VRotr { src, amount, .. } => {
                result.push(*src);
                result.push(*amount);
            }

            OpKind::VAddSubMixedSat { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VSetPredQ { scalar, .. } => {
                result.push(*scalar);
            }

            OpKind::VShuffEqQ { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            // vmpa(Vx, Vu, Rtt):sat reads the dst (Vx) accumulator, Vu, and Rtt.
            OpKind::VMpaHhSat {
                dst, src, table, ..
            } => {
                result.push(*dst);
                result.push(*src);
                result.push(*table);
            }

            // vmpyhsat_acc accumulates into the existing dst pair.
            OpKind::VMpyHsatAcc {
                dst_lo,
                dst_hi,
                src,
                scalar,
            } => {
                result.push(*dst_lo);
                result.push(*dst_hi);
                result.push(*src);
                result.push(*scalar);
            }

            // vasr_into shifts Vu into the running accumulator pair (read+write).
            OpKind::VAsrInto {
                dst_lo,
                dst_hi,
                src,
                amount,
            } => {
                result.push(*dst_lo);
                result.push(*dst_hi);
                result.push(*src);
                result.push(*amount);
            }

            OpKind::V6Mpy {
                src_lo,
                src_hi,
                src2_lo,
                src2_hi,
                dst_lo,
                dst_hi,
                acc,
                ..
            } => {
                result.push(*src_lo);
                result.push(*src_hi);
                result.push(*src2_lo);
                result.push(*src2_hi);
                if *acc {
                    result.push(*dst_lo);
                    result.push(*dst_hi);
                }
            }

            OpKind::VDelta { src, control, .. } => {
                result.push(*src);
                result.push(*control);
            }

            OpKind::VPack { src1, src2, .. }
            | OpKind::VPackSat { src1, src2, .. }
            | OpKind::VShuffleEO { src1, src2, .. }
            | OpKind::VDealB4W { src1, src2, .. }
            | OpKind::VMulSubLaneFrac { src1, src2, .. }
            | OpKind::VMulSubLaneSh { src1, src2, .. }
            | OpKind::VMulShiftSat { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VMulWord64Pair {
                src1,
                src2,
                dst_lo,
                dst_hi,
                mode,
            } => {
                result.push(*src1);
                result.push(*src2);
                // mode 1 (vmpyowh_64_acc) reads the existing dst pair.
                if *mode == 1 {
                    result.push(*dst_lo);
                    result.push(*dst_hi);
                }
            }

            OpKind::VShuffle2 { src, .. } => {
                result.push(*src);
            }

            OpKind::VAlign {
                src1, src2, amount, ..
            } => {
                result.push(*src1);
                result.push(*src2);
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }

            OpKind::VShiftV { src, amount, .. } => {
                result.push(*src);
                result.push(*amount);
            }

            OpKind::VNarrowShiftSat {
                src_lo,
                src_hi,
                amount,
                ..
            } => {
                result.push(*src_lo);
                result.push(*src_hi);
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }

            OpKind::VSatDW { src_lo, src_hi, .. } => {
                result.push(*src_lo);
                result.push(*src_hi);
            }

            OpKind::VNarrowShiftV {
                src_lo,
                src_hi,
                amount,
                ..
            } => {
                result.push(*src_lo);
                result.push(*src_hi);
                result.push(*amount);
            }

            OpKind::VCmpToQ { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VBlend {
                mask_q,
                src_true,
                src_false,
                ..
            } => {
                result.push(*mask_q);
                result.push(*src_true);
                result.push(*src_false);
            }

            OpKind::VMaskZero {
                mask_q,
                src,
                dst,
                oracc,
                ..
            } => {
                result.push(*mask_q);
                result.push(*src);
                // oracc (vandqrt_acc) OR-accumulates into the existing dst.
                if *oracc {
                    result.push(*dst);
                }
            }

            OpKind::VQFromVAndR {
                src1,
                src2,
                dst,
                oracc,
            } => {
                result.push(*src1);
                result.push(*src2);
                // oracc (vandvrt_acc) OR-accumulates into the existing dst Q.
                if *oracc {
                    result.push(*dst);
                }
            }

            // Q-predicated conditional add/sub: dst is read-modify-written.
            OpKind::VLaneCond {
                dst, src, mask_q, ..
            } => {
                result.push(*dst);
                result.push(*src);
                result.push(*mask_q);
            }

            // Carry add/sub: reads both vectors; reads the carry Q when it has a
            // carry-in (carry / carrysat forms).
            OpKind::VCarry {
                src1,
                src2,
                q_inout,
                has_cin,
                ..
            } => {
                result.push(*src1);
                result.push(*src2);
                if *has_cin {
                    result.push(*q_inout);
                }
            }

            OpKind::VSwap {
                mask_q, src1, src2, ..
            } => {
                result.push(*mask_q);
                result.push(*src1);
                result.push(*src2);
            }

            // Scalar-predicate-gated move/combine: when the gate is false the
            // dest(s) keep their prior value, so they are read; also reads the
            // predicate and the candidate sources.
            OpKind::VCondMove {
                dst_lo,
                dst_hi,
                src_lo,
                src_hi,
                pred,
                ..
            } => {
                result.push(*pred);
                result.push(*src_lo);
                result.push(*dst_lo);
                if let Some(hi) = dst_hi {
                    result.push(*src_hi);
                    result.push(*hi);
                }
            }

            OpKind::VPrefixSumQ { mask_q, .. } => {
                result.push(*mask_q);
            }

            // The histogram family read-modify-writes the WHOLE V0..V31 file and
            // reads the input vector from memory (the `.tmp` load's address) plus
            // the q-mask for the q-forms.
            OpKind::VHist {
                input,
                mask_q,
                use_q,
                ..
            } => {
                result.extend(input.regs());
                if *use_q {
                    result.push(*mask_q);
                }
                for n in 0..32u8 {
                    result.push(VReg::Arch(ArchReg::Hexagon(HexagonReg::V(n))));
                }
            }

            OpKind::VMov { src, .. } | OpKind::VBroadcast { scalar: src, .. } => {
                result.push(*src);
            }

            OpKind::VInsertLane { vec, scalar, .. } => {
                result.push(*vec);
                result.push(*scalar);
            }

            OpKind::VExtractLane { vec, .. } => {
                result.push(*vec);
            }

            OpKind::VShuffle {
                src1,
                src2,
                indices,
                ..
            } => {
                result.push(*src1);
                if let Some(s2) = src2 {
                    result.push(*s2);
                }
                result.push(*indices);
            }

            OpKind::VLoad { addr, .. } => {
                result.extend(addr.regs());
            }

            OpKind::VStore { src, addr, .. } => {
                result.push(*src);
                result.extend(addr.regs());
            }

            // Operations with no source registers
            OpKind::ReadFlags { .. }
            | OpKind::SetCF { .. }
            | OpKind::SetDF { .. }
            | OpKind::CmcCF
            | OpKind::MaterializeFlags
            | OpKind::TestCondition { .. }
            | OpKind::SetCC { .. }
            | OpKind::ClearExclusive
            | OpKind::Prefetch { .. }
            | OpKind::Fence { .. }
            | OpKind::IoIn { .. }
            | OpKind::IoOut { .. }
            | OpKind::Swi { .. }
            | OpKind::ReadSysReg { .. }
            | OpKind::Nop
            | OpKind::Undefined { .. }
            | OpKind::Breakpoint => {}

            // AVX10 operations - extract source registers
            OpKind::VMin { src1, src2, .. } | OpKind::VFma { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VDotProduct {
                acc, src1, src2, ..
            }
            | OpKind::VDotProductBF16 {
                acc, src1, src2, ..
            } => {
                result.push(*acc);
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VMultiplyAdd52 {
                dst, src1, src2, ..
            } => {
                result.push(*dst); // dst is also input (accumulator)
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VPopcnt { src, .. } | OpKind::VCvtBF16ToFP32 { src, .. } => {
                result.push(*src);
            }

            OpKind::VPermute {
                src1,
                src2,
                indices,
                ..
            } => {
                result.push(*src1);
                if let Some(s2) = src2 {
                    result.push(*s2);
                }
                result.push(*indices);
            }

            OpKind::VShuffleBitQM { src, indices, .. } => {
                result.push(*src);
                result.push(*indices);
            }

            OpKind::VCvtFP32ToBF16 { src1, src2, .. } => {
                result.push(*src1);
                if let Some(s2) = src2 {
                    result.push(*s2);
                }
            }

            OpKind::VFP16Arith { src1, src2, .. }
            | OpKind::VMinMax { src1, src2, .. }
            | OpKind::VMpsadbw { src1, src2, .. } => {
                result.push(*src1);
                result.push(*src2);
            }

            OpKind::VCvtFpToIntSat { src, .. } => {
                result.push(*src);
            }

            OpKind::VDotProductExt {
                acc, src1, src2, ..
            } => {
                result.push(*acc);
                result.push(*src1);
                result.push(*src2);
            }

            // Carry-less multiply: src1/src2 are SrcOperands; the `_acc` forms
            // also read the existing dst/dst_hi (XOR target).
            OpKind::ClMul {
                src1,
                src2,
                dst,
                dst_hi,
                acc,
                ..
            } => {
                if let SrcOperand::Reg(r) = src1 {
                    result.push(*r);
                }
                if let SrcOperand::Reg(r) = src2 {
                    result.push(*r);
                }
                if *acc {
                    result.push(*dst);
                    if let Some(hi) = dst_hi {
                        result.push(*hi);
                    }
                }
            }

            // Wide complex multiply: reads both halves of the Rss and Rtt pairs.
            OpKind::CmpyW128Sat {
                rss_lo,
                rss_hi,
                rtt_lo,
                rtt_hi,
                ..
            } => {
                result.push(*rss_lo);
                result.push(*rss_hi);
                result.push(*rtt_lo);
                result.push(*rtt_hi);
            }

            // Register-amount saturating shift: src and amount are SrcOperands.
            OpKind::SatOrigShl { src, amount, .. } => {
                if let SrcOperand::Reg(r) = src {
                    result.push(*r);
                }
                if let SrcOperand::Reg(r) = amount {
                    result.push(*r);
                }
            }
        }

        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smir::ops::OpKind;
    use crate::smir::types::{GuestAddr, OpId};

    fn make_op(id: u16, kind: OpKind) -> SmirOp {
        SmirOp::new(OpId(id), 0x1000, kind)
    }

    #[test]
    fn test_dead_flag_elimination() {
        let mut block = SmirBlock::new(BlockId(0), 0x1000);

        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);
        let v2 = VReg::virt(2);

        // Add with flags that are never used
        block.push_op(make_op(
            0,
            OpKind::Add {
                dst: v0,
                src1: v1,
                src2: SrcOperand::Imm(1),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
        ));

        // Another add with flags
        block.push_op(make_op(
            1,
            OpKind::Add {
                dst: v2,
                src1: v0,
                src2: SrcOperand::Imm(2),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
        ));

        block.set_terminator(Terminator::Return { values: vec![v2] });

        let eliminated = dead_flag_elimination(&mut block);

        // Both flag updates should be eliminated since no flags are read
        assert_eq!(eliminated, 2);

        // Check flags are now None
        for op in &block.ops {
            if let OpKind::Add { flags, .. } = &op.kind {
                assert_eq!(*flags, FlagUpdate::None);
            }
        }
    }

    #[test]
    fn test_constant_propagation() {
        let mut block = SmirBlock::new(BlockId(0), 0x1000);

        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);
        let v2 = VReg::virt(2);

        // mov v0, 10
        block.push_op(make_op(
            0,
            OpKind::Mov {
                dst: v0,
                src: SrcOperand::Imm(10),
                width: OpWidth::W64,
            },
        ));

        // mov v1, v0 (should propagate to mov v1, 10)
        block.push_op(make_op(
            1,
            OpKind::Mov {
                dst: v1,
                src: SrcOperand::Reg(v0),
                width: OpWidth::W64,
            },
        ));

        // add v2, v1, v0 (v0 should be replaced with 10)
        block.push_op(make_op(
            2,
            OpKind::Add {
                dst: v2,
                src1: v1,
                src2: SrcOperand::Reg(v0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        block.set_terminator(Terminator::Return { values: vec![v2] });

        let propagated = constant_propagation(&mut block);

        assert!(propagated >= 2);

        // Check that v0 in add was replaced with immediate
        if let OpKind::Add { src2, .. } = &block.ops[2].kind {
            assert!(matches!(src2, SrcOperand::Imm(10)));
        }
    }

    #[test]
    fn test_constant_folding() {
        let mut block = SmirBlock::new(BlockId(0), 0x1000);

        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);

        // and v0, v1, 0 -> mov v0, 0
        block.push_op(make_op(
            0,
            OpKind::And {
                dst: v0,
                src1: v1,
                src2: SrcOperand::Imm(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        block.set_terminator(Terminator::Return { values: vec![v0] });

        let folded = constant_folding(&mut block);

        assert_eq!(folded, 1);

        // Check it was folded to a mov
        if let OpKind::Mov { src, .. } = &block.ops[0].kind {
            assert!(matches!(src, SrcOperand::Imm(0)));
        } else {
            panic!("Expected Mov operation");
        }
    }

    #[test]
    fn test_xor_same_register_fold() {
        let mut block = SmirBlock::new(BlockId(0), 0x1000);

        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);

        // xor v0, v1, v1 -> mov v0, 0
        block.push_op(make_op(
            0,
            OpKind::Xor {
                dst: v0,
                src1: v1,
                src2: SrcOperand::Reg(v1),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        block.set_terminator(Terminator::Return { values: vec![v0] });

        let folded = constant_folding(&mut block);

        assert_eq!(folded, 1);

        if let OpKind::Mov { src, .. } = &block.ops[0].kind {
            assert!(matches!(src, SrcOperand::Imm(0)));
        } else {
            panic!("Expected Mov operation");
        }
    }

    #[test]
    fn test_dead_code_elimination() {
        let mut block = SmirBlock::new(BlockId(0), 0x1000);

        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);
        let v2 = VReg::virt(2);

        // mov v0, 10 (unused)
        block.push_op(make_op(
            0,
            OpKind::Mov {
                dst: v0,
                src: SrcOperand::Imm(10),
                width: OpWidth::W64,
            },
        ));

        // mov v1, 20 (used)
        block.push_op(make_op(
            1,
            OpKind::Mov {
                dst: v1,
                src: SrcOperand::Imm(20),
                width: OpWidth::W64,
            },
        ));

        // mov v2, 30 (unused)
        block.push_op(make_op(
            2,
            OpKind::Mov {
                dst: v2,
                src: SrcOperand::Imm(30),
                width: OpWidth::W64,
            },
        ));

        block.set_terminator(Terminator::Return { values: vec![v1] });

        let eliminated = dead_code_elimination(&mut block);

        assert_eq!(eliminated, 2);
        assert_eq!(block.ops.len(), 1);

        // Only v1 should remain
        if let OpKind::Mov { dst, .. } = &block.ops[0].kind {
            assert_eq!(*dst, v1);
        }
    }

    #[test]
    fn test_strength_reduction_mul() {
        let mut block = SmirBlock::new(BlockId(0), 0x1000);

        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);

        // mul v0, v1, 8 -> shl v0, v1, 3
        block.push_op(make_op(
            0,
            OpKind::MulU {
                dst_lo: v0,
                dst_hi: None,
                src1: v1,
                src2: SrcOperand::Imm(8),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        block.set_terminator(Terminator::Return { values: vec![v0] });

        let reductions = strength_reduction(&mut block);

        assert_eq!(reductions, 1);

        if let OpKind::Shl { amount, .. } = &block.ops[0].kind {
            assert!(matches!(amount, SrcOperand::Imm(3)));
        } else {
            panic!("Expected Shl operation");
        }
    }

    #[test]
    fn test_strength_reduction_div() {
        let mut block = SmirBlock::new(BlockId(0), 0x1000);

        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);

        // div v0, v1, 16 -> shr v0, v1, 4
        block.push_op(make_op(
            0,
            OpKind::DivU {
                quot: v0,
                rem: None,
                src1: v1,
                src2: SrcOperand::Imm(16),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));

        block.set_terminator(Terminator::Return { values: vec![v0] });

        let reductions = strength_reduction(&mut block);

        assert_eq!(reductions, 1);

        if let OpKind::Shr { amount, .. } = &block.ops[0].kind {
            assert!(matches!(amount, SrcOperand::Imm(4)));
        } else {
            panic!("Expected Shr operation");
        }
    }

    #[test]
    fn test_optimize_function() {
        use crate::smir::ir::FunctionBuilder;
        use crate::smir::types::FunctionId;

        let mut builder = FunctionBuilder::new(FunctionId(0), 0x1000);

        let v0 = builder.alloc_vreg();
        let v1 = builder.alloc_vreg();
        let v2 = builder.alloc_vreg();
        let v3 = builder.alloc_vreg();

        // mov v0, 10
        builder.push_op(
            0x1000,
            OpKind::Mov {
                dst: v0,
                src: SrcOperand::Imm(10),
                width: OpWidth::W64,
            },
        );

        // add v1, v0, 5 (with flags)
        builder.push_op(
            0x1004,
            OpKind::Add {
                dst: v1,
                src1: v0,
                src2: SrcOperand::Imm(5),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
        );

        // mov v2, 100 (dead)
        builder.push_op(
            0x1008,
            OpKind::Mov {
                dst: v2,
                src: SrcOperand::Imm(100),
                width: OpWidth::W64,
            },
        );

        // and v3, v1, 0 -> should fold to mov v3, 0
        builder.push_op(
            0x100c,
            OpKind::And {
                dst: v3,
                src1: v1,
                src2: SrcOperand::Imm(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );

        builder.set_terminator(Terminator::Return { values: vec![v3] });

        let mut func = builder.finish();

        let stats = optimize_function(&mut func, OptLevel::O2);

        // Should have optimizations applied
        assert!(stats.total() > 0);
    }

    #[test]
    fn test_opt_stats() {
        let mut stats1 = OptStats::new();
        stats1.dead_flags_eliminated = 5;
        stats1.constants_propagated = 3;

        let mut stats2 = OptStats::new();
        stats2.dead_ops_eliminated = 2;
        stats2.expressions_folded = 1;

        stats1.merge(&stats2);

        assert_eq!(stats1.dead_flags_eliminated, 5);
        assert_eq!(stats1.constants_propagated, 3);
        assert_eq!(stats1.dead_ops_eliminated, 2);
        assert_eq!(stats1.expressions_folded, 1);
        assert_eq!(stats1.total(), 11);
    }

    #[test]
    fn test_copy_propagation() {
        let mut block = SmirBlock::new(BlockId(0), 0x1000);
        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);
        let rax = VReg::Arch(ArchReg::X86(X86Reg::Rax));
        let rbx = VReg::Arch(ArchReg::X86(X86Reg::Rbx));

        // mov v0, rbx     (W64 copy)
        block.push_op(make_op(
            0,
            OpKind::Mov {
                dst: v0,
                src: SrcOperand::Reg(rbx),
                width: OpWidth::W64,
            },
        ));
        // add v1, rax, v0  -> v0 rewritten to rbx
        block.push_op(make_op(
            1,
            OpKind::Add {
                dst: v1,
                src1: rax,
                src2: SrcOperand::Reg(v0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));
        block.set_terminator(Terminator::Return { values: vec![v1] });

        let n = copy_propagation(&mut block);
        assert_eq!(n, 1);
        if let OpKind::Add { src2, .. } = &block.ops[1].kind {
            assert!(matches!(src2, SrcOperand::Reg(r) if *r == rbx));
        } else {
            panic!("expected Add");
        }
    }

    #[test]
    fn test_copy_propagation_w32_not_recorded() {
        // A 32-bit copy must NOT be propagated into a 64-bit-equality use.
        let mut block = SmirBlock::new(BlockId(0), 0x1000);
        let v0 = VReg::virt(0);
        let v1 = VReg::virt(1);
        let rax = VReg::Arch(ArchReg::X86(X86Reg::Rax));
        let rbx = VReg::Arch(ArchReg::X86(X86Reg::Rbx));
        block.push_op(make_op(
            0,
            OpKind::Mov {
                dst: v0,
                src: SrcOperand::Reg(rbx),
                width: OpWidth::W32, // zero-extends; v0 != rbx in 64 bits
            },
        ));
        block.push_op(make_op(
            1,
            OpKind::Add {
                dst: v1,
                src1: rax,
                src2: SrcOperand::Reg(v0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ));
        block.set_terminator(Terminator::Return { values: vec![v1] });
        let n = copy_propagation(&mut block);
        assert_eq!(n, 0); // not propagated
    }

    #[test]
    fn test_branch_folding_same_target_and_unreachable() {
        use crate::smir::types::FunctionId;
        let b0 = BlockId(0);
        let b1 = BlockId(1);
        let b2 = BlockId(2);
        let mut func = SmirFunction::new(FunctionId(0), b0, 0x1000);

        // b0: cond-branch to b1 either way (same target) -> folds to Branch b1.
        let mut blk0 = SmirBlock::new(b0, 0x1000);
        blk0.set_terminator(Terminator::CondBranch {
            cond: VReg::virt(0),
            true_target: b1,
            false_target: b1,
        });
        func.add_block(blk0);

        // b1: reachable, returns.
        let mut blk1 = SmirBlock::new(b1, 0x1010);
        blk1.set_terminator(Terminator::Return { values: vec![] });
        func.add_block(blk1);

        // b2: unreachable -> removed.
        let mut blk2 = SmirBlock::new(b2, 0x1020);
        blk2.set_terminator(Terminator::Return { values: vec![] });
        func.add_block(blk2);

        let n = branch_folding(&mut func);
        assert!(n >= 2); // 1 fold + 1 unreachable removed
        assert!(matches!(func.blocks[0].terminator, Terminator::Branch { target } if target == b1));
        assert!(func.blocks.iter().all(|b| b.id != b2));
    }
}
