//! State-backed AArch64-to-x86_64 SMIR lowerer.
//!
//! This lowers AArch64-lifted scalar SMIR into normal x86-64 SysV leaf
//! functions. RDI is the persistent `*mut Aarch64GuestRegs` state pointer; guest
//! X/SP/PC/NZCV are read and written through that struct, while SMIR virtual
//! temporaries are stack slots in the native frame.

use std::collections::{HashMap, HashSet};

use crate::smir::flags::FlagUpdate;
use crate::smir::ir::{CallTarget, SmirBlock, SmirFunction, Terminator};
use crate::smir::ops::{OpKind, SmirOp};
use crate::smir::types::{
    Address, ArchReg, ArmReg, AtomicOp, BlockId, Condition, ExtendOp, MemWidth, OpWidth, ShiftOp,
    SignExtend, SrcOperand, VReg, VirtualId,
};

use super::regalloc::PhysReg;
use super::x86_64::{X86Cond, X86Emitter};
use super::{CodeBuffer, LowerError, LowerResult, RelocKind, Relocation, SmirLowerer};

const STATE: PhysReg = PhysReg::Rdi;
const ACC: PhysReg = PhysReg::Rax;
const HI: PhysReg = PhysReg::Rdx;
const RHS: PhysReg = PhysReg::Rcx;
const B0: PhysReg = PhysReg::R8;
const B1: PhysReg = PhysReg::R9;
const B2: PhysReg = PhysReg::R10;
const B3: PhysReg = PhysReg::R11;
const ADDR: PhysReg = PhysReg::Rsi;

const NZCV_N: i64 = 1_i64 << 31;
const NZCV_Z: i64 = 1_i64 << 30;
const NZCV_C: i64 = 1_i64 << 29;
const NZCV_V: i64 = 1_i64 << 28;

const A64_X0_OFFSET: i32 = 0;
const A64_SP_OFFSET: i32 = 31 * 8;
const A64_PC_OFFSET: i32 = 32 * 8;
const A64_NZCV_OFFSET: i32 = 33 * 8;
const A64_FPCR_OFFSET: i32 = 34 * 8;
const A64_FPSR_OFFSET: i32 = 35 * 8;
const A64_V_OFFSET: i32 = 36 * 8;
const A64_CTX_OFFSET: i32 = A64_V_OFFSET + 64 * 8;
const A64_LOAD_FN_OFFSET: i32 = A64_CTX_OFFSET + 8;
const A64_STORE_FN_OFFSET: i32 = A64_LOAD_FN_OFFSET + 8;
const A64_EXCLUSIVE_ADDR_OFFSET: i32 = A64_STORE_FN_OFFSET + 8;
const A64_EXCLUSIVE_SIZE_OFFSET: i32 = A64_EXCLUSIVE_ADDR_OFFSET + 8;
const A64_EXCLUSIVE_VALID_OFFSET: i32 = A64_EXCLUSIVE_SIZE_OFFSET + 8;

#[derive(Clone, Copy, Debug)]
enum FlagForm {
    Add,
    Sub,
    Logic,
}

#[derive(Clone, Copy, Debug)]
enum LogicForm {
    And,
    Or,
    Xor,
}

/// Lower AArch64 SMIR to x86-64 code using an explicit AArch64 state pointer.
pub struct Aarch64X86_64Lowerer {
    code: CodeBuffer,
    block_offsets: HashMap<BlockId, usize>,
    pending_jumps: Vec<(usize, BlockId, RelocKind)>,
    virtual_slots: HashMap<VirtualId, i32>,
    frame_size: usize,
    relocations: Vec<Relocation>,
}

impl Aarch64X86_64Lowerer {
    pub fn new() -> Self {
        Self {
            code: CodeBuffer::with_capacity(4096),
            block_offsets: HashMap::new(),
            pending_jumps: Vec::new(),
            virtual_slots: HashMap::new(),
            frame_size: 0,
            relocations: Vec::new(),
        }
    }

    fn collect_virtuals(&mut self, func: &SmirFunction) {
        let mut ids = HashSet::new();
        for block in &func.blocks {
            for phi in &block.phis {
                if let VReg::Virtual(id) = phi.dst {
                    ids.insert(id);
                }
                for (_, src) in &phi.sources {
                    if let VReg::Virtual(id) = *src {
                        ids.insert(id);
                    }
                }
            }
            for op in &block.ops {
                for v in op.kind.dests().into_iter().chain(op.kind.source_vregs()) {
                    if let VReg::Virtual(id) = v {
                        ids.insert(id);
                    }
                }
            }
            match &block.terminator {
                Terminator::CondBranch { cond, .. }
                | Terminator::IndirectBranch { target: cond, .. } => {
                    if let VReg::Virtual(id) = *cond {
                        ids.insert(id);
                    }
                }
                Terminator::Call { target, .. } | Terminator::TailCall { target, .. } => {
                    if let CallTarget::Indirect(reg) = target {
                        if let VReg::Virtual(id) = *reg {
                            ids.insert(id);
                        }
                    }
                    if let CallTarget::IndirectMem(addr) = target {
                        for reg in addr.regs() {
                            if let VReg::Virtual(id) = reg {
                                ids.insert(id);
                            }
                        }
                    }
                }
                Terminator::Return { values } => {
                    for v in values {
                        if let VReg::Virtual(id) = *v {
                            ids.insert(id);
                        }
                    }
                }
                _ => {}
            }
        }

        let mut ids: Vec<_> = ids.into_iter().collect();
        ids.sort_by_key(|id| id.0);
        self.virtual_slots.clear();
        for (idx, id) in ids.into_iter().enumerate() {
            self.virtual_slots.insert(id, -8 * (idx as i32 + 1));
        }
        self.frame_size = align16(self.virtual_slots.len() * 8);
    }

    fn emit_prologue(&mut self) {
        let mut e = X86Emitter::new(&mut self.code);
        e.emit_push(PhysReg::Rbp);
        e.emit_mov_rr(PhysReg::Rbp, PhysReg::Rsp, OpWidth::W64);
        if self.frame_size != 0 {
            e.emit_sub_ri(PhysReg::Rsp, self.frame_size as i64, OpWidth::W64);
        }
    }

    fn emit_epilogue(&mut self) {
        let mut e = X86Emitter::new(&mut self.code);
        e.emit_mov_rr(PhysReg::Rsp, PhysReg::Rbp, OpWidth::W64);
        e.emit_pop(PhysReg::Rbp);
        e.emit_ret();
    }

    fn emit_exit_to_acc(&mut self) {
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_mr(STATE, A64_PC_OFFSET, ACC, OpWidth::W64);
        }
        self.emit_epilogue();
    }

    fn emit_exit_to_guest_addr(&mut self, addr: u64) {
        self.emit_mov_imm(ACC, addr as i64, OpWidth::W64);
        self.emit_exit_to_acc();
    }

    fn lower_exit_to_vreg(&mut self, target: VReg) -> Result<(), LowerError> {
        self.load_vreg_to(target, ACC, OpWidth::W64)?;
        self.emit_exit_to_acc();
        Ok(())
    }

    fn lower_exit_to_call_target(&mut self, target: &CallTarget) -> Result<(), LowerError> {
        match target {
            CallTarget::GuestAddr(addr) => {
                self.emit_exit_to_guest_addr(*addr);
                Ok(())
            }
            CallTarget::Indirect(reg) => self.lower_exit_to_vreg(*reg),
            other => Err(LowerError::UnsupportedOp {
                op: format!("AArch64 state-backed call target {other:?}"),
            }),
        }
    }

    fn arm_offset(reg: ArmReg) -> Result<i32, LowerError> {
        match reg {
            ArmReg::X(n) if n < 31 => Ok(A64_X0_OFFSET + i32::from(n) * 8),
            ArmReg::X(_) => Err(LowerError::InvalidRegister(
                "x31 must be encoded as XZR".into(),
            )),
            ArmReg::Sp => Ok(A64_SP_OFFSET),
            ArmReg::Pc => Ok(A64_PC_OFFSET),
            ArmReg::Nzcv => Ok(A64_NZCV_OFFSET),
            ArmReg::Fpcr => Ok(A64_FPCR_OFFSET),
            ArmReg::Fpsr => Ok(A64_FPSR_OFFSET),
            ArmReg::V(n) if n < 32 => Ok(A64_V_OFFSET + i32::from(n) * 16),
            ArmReg::V(_) | ArmReg::SysReg(_) => {
                Err(LowerError::InvalidRegister(format!("{reg:?}")))
            }
        }
    }

    fn virtual_slot(&self, id: VirtualId) -> Result<i32, LowerError> {
        self.virtual_slots
            .get(&id)
            .copied()
            .ok_or_else(|| LowerError::RegisterAllocationFailed {
                reason: format!("missing stack slot for virtual {:?}", id),
            })
    }

    fn emit_mov_imm(&mut self, dst: PhysReg, imm: i64, width: OpWidth) {
        let mut e = X86Emitter::new(&mut self.code);
        if width == OpWidth::W64 {
            e.emit_mov_ri_imm64(dst, imm);
        } else {
            e.emit_mov_ri(dst, imm, width);
        }
    }

    fn load_vreg_to(&mut self, vreg: VReg, dst: PhysReg, width: OpWidth) -> Result<(), LowerError> {
        match vreg {
            VReg::Imm(value) => {
                self.emit_mov_imm(dst, value, width);
            }
            VReg::Virtual(id) => {
                let off = self.virtual_slot(id)?;
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_mov_rm(dst, PhysReg::Rbp, off, OpWidth::W64);
            }
            VReg::Arch(ArchReg::Arm(reg)) => {
                let off = Self::arm_offset(reg)?;
                let load_width = if reg == ArmReg::Nzcv {
                    OpWidth::W32
                } else {
                    OpWidth::W64
                };
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_mov_rm(dst, STATE, off, load_width);
            }
            VReg::Arch(other) => {
                return Err(LowerError::InvalidRegister(format!(
                    "non-AArch64 register in AArch64 lowerer: {other:?}"
                )));
            }
        }
        Ok(())
    }

    fn load_src_to(
        &mut self,
        src: &SrcOperand,
        dst: PhysReg,
        width: OpWidth,
    ) -> Result<(), LowerError> {
        match src {
            SrcOperand::Reg(v) => self.load_vreg_to(*v, dst, width),
            SrcOperand::Imm(value) | SrcOperand::Imm64(value) => {
                self.emit_mov_imm(dst, *value, width);
                Ok(())
            }
            SrcOperand::Shifted { reg, shift, amount } => {
                self.load_vreg_to(*reg, dst, width)?;
                self.emit_shift_imm(dst, *shift, *amount, width)
            }
            SrcOperand::Extended { reg, extend, shift } => {
                self.load_vreg_to(*reg, dst, OpWidth::W64)?;
                self.emit_extend_in_place(dst, *extend, width)?;
                if *shift != 0 {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_shl_ri(dst, *shift, width);
                }
                Ok(())
            }
        }
    }

    fn store_reg_to(&mut self, dst: VReg, src: PhysReg, width: OpWidth) -> Result<(), LowerError> {
        self.emit_normalize_gpr_width(src, width)?;
        match dst {
            VReg::Imm(_) => {}
            VReg::Virtual(id) => {
                let off = self.virtual_slot(id)?;
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_mov_mr(PhysReg::Rbp, off, src, OpWidth::W64);
            }
            VReg::Arch(ArchReg::Arm(reg)) => {
                let off = Self::arm_offset(reg)?;
                let store_width = if reg == ArmReg::Nzcv {
                    OpWidth::W32
                } else {
                    match width {
                        OpWidth::W32 | OpWidth::W64 => OpWidth::W64,
                        other => other,
                    }
                };
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_mov_mr(STATE, off, src, store_width);
            }
            VReg::Arch(other) => {
                return Err(LowerError::InvalidRegister(format!(
                    "non-AArch64 destination in AArch64 lowerer: {other:?}"
                )));
            }
        }
        Ok(())
    }

    fn emit_normalize_gpr_width(&mut self, reg: PhysReg, width: OpWidth) -> Result<(), LowerError> {
        let mut e = X86Emitter::new(&mut self.code);
        match width {
            OpWidth::W8 | OpWidth::W16 => e.emit_movzx(reg, reg, width, OpWidth::W64),
            OpWidth::W32 => e.emit_mov_rr(reg, reg, OpWidth::W32),
            OpWidth::W64 => {}
            OpWidth::W128 => {
                return Err(LowerError::UnsupportedOp {
                    op: "AArch64 scalar store width W128".into(),
                });
            }
        }
        Ok(())
    }

    fn emit_extend_in_place(
        &mut self,
        reg: PhysReg,
        extend: ExtendOp,
        width: OpWidth,
    ) -> Result<(), LowerError> {
        let mut e = X86Emitter::new(&mut self.code);
        match extend {
            ExtendOp::Uxtb => e.emit_movzx(reg, reg, OpWidth::W8, width),
            ExtendOp::Uxth => e.emit_movzx(reg, reg, OpWidth::W16, width),
            ExtendOp::Uxtw => e.emit_mov_rr(reg, reg, OpWidth::W32),
            ExtendOp::Uxtx => {}
            ExtendOp::Sxtb => e.emit_movsx(reg, reg, OpWidth::W8, width),
            ExtendOp::Sxth => e.emit_movsx(reg, reg, OpWidth::W16, width),
            ExtendOp::Sxtw => e.emit_movsx(reg, reg, OpWidth::W32, width),
            ExtendOp::Sxtx => {}
        }
        Ok(())
    }

    fn emit_shift_imm(
        &mut self,
        reg: PhysReg,
        shift: ShiftOp,
        amount: u8,
        width: OpWidth,
    ) -> Result<(), LowerError> {
        if amount == 0 {
            return Ok(());
        }
        let mut e = X86Emitter::new(&mut self.code);
        match shift {
            ShiftOp::Lsl => e.emit_shl_ri(reg, amount, width),
            ShiftOp::Lsr => e.emit_shr_ri(reg, amount, width),
            ShiftOp::Asr => e.emit_sar_ri(reg, amount, width),
            ShiftOp::Ror | ShiftOp::Rrx => e.emit_ror_ri(reg, amount, width),
        }
        Ok(())
    }

    fn emit_shift_src(
        &mut self,
        reg: PhysReg,
        amount: &SrcOperand,
        kind: ShiftOp,
        width: OpWidth,
    ) -> Result<(), LowerError> {
        match amount {
            SrcOperand::Imm(value) | SrcOperand::Imm64(value) => {
                self.emit_shift_imm(reg, kind, *value as u8, width)
            }
            _ => {
                self.load_src_to(amount, RHS, width)?;
                let mut e = X86Emitter::new(&mut self.code);
                match kind {
                    ShiftOp::Lsl => e.emit_shl_cl(reg, width),
                    ShiftOp::Lsr => e.emit_shr_cl(reg, width),
                    ShiftOp::Asr => e.emit_sar_cl(reg, width),
                    ShiftOp::Ror | ShiftOp::Rrx => e.emit_ror_cl(reg, width),
                }
                Ok(())
            }
        }
    }

    fn scalar_mem_width(width: MemWidth) -> Result<(OpWidth, i64), LowerError> {
        match width {
            MemWidth::B1 => Ok((OpWidth::W8, 1)),
            MemWidth::B2 => Ok((OpWidth::W16, 2)),
            MemWidth::B4 => Ok((OpWidth::W32, 4)),
            MemWidth::B8 => Ok((OpWidth::W64, 8)),
            _ => Err(LowerError::UnsupportedOp {
                op: format!("AArch64 scalar memory width {width:?}"),
            }),
        }
    }

    fn emit_add_i64_to_reg(&mut self, reg: PhysReg, imm: i64) {
        if imm == 0 {
            return;
        }
        let mut e = X86Emitter::new(&mut self.code);
        if i32::try_from(imm).is_ok() {
            e.emit_add_ri(reg, imm, OpWidth::W64);
        } else {
            drop(e);
            self.emit_mov_imm(B2, imm, OpWidth::W64);
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_add_rr(reg, B2, OpWidth::W64);
        }
    }

    fn load_addr_to(&mut self, addr: &Address, dst: PhysReg) -> Result<(), LowerError> {
        match addr {
            Address::Direct(base) => self.load_vreg_to(*base, dst, OpWidth::W64)?,
            Address::BaseOffset { base, offset, .. } => {
                self.load_vreg_to(*base, dst, OpWidth::W64)?;
                self.emit_add_i64_to_reg(dst, *offset);
            }
            Address::BaseIndexScale {
                base,
                index,
                scale,
                disp,
                ..
            } => {
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    if let Some(base) = base {
                        drop(e);
                        self.load_vreg_to(*base, dst, OpWidth::W64)?;
                    } else {
                        e.emit_xor_rr(dst, dst, OpWidth::W64);
                    }
                }
                self.load_vreg_to(*index, B2, OpWidth::W64)?;
                match scale {
                    1 => {}
                    2 | 4 | 8 => {
                        let mut e = X86Emitter::new(&mut self.code);
                        e.emit_shl_ri(B2, scale.trailing_zeros() as u8, OpWidth::W64);
                    }
                    _ => {
                        return Err(LowerError::UnsupportedOp {
                            op: format!("AArch64 memory scale {scale}"),
                        });
                    }
                }
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_add_rr(dst, B2, OpWidth::W64);
                }
                self.emit_add_i64_to_reg(dst, i64::from(*disp));
            }
            Address::Absolute(addr) => self.emit_mov_imm(dst, *addr as i64, OpWidth::W64),
            Address::PcRel { offset, base, .. } => {
                let addr = base.unwrap_or(0).wrapping_add(*offset as u64);
                self.emit_mov_imm(dst, addr as i64, OpWidth::W64);
            }
            Address::GpRel { .. } | Address::SegmentRel { .. } => {
                return Err(LowerError::UnsupportedOp {
                    op: format!("AArch64 memory address {addr:?}"),
                });
            }
        }
        Ok(())
    }

    fn emit_mem_helper_call(&mut self, target: PhysReg) {
        let mut e = X86Emitter::new(&mut self.code);
        e.emit_push(STATE);
        e.emit_sub_ri(PhysReg::Rsp, 8, OpWidth::W64);
        e.emit_mov_rm(STATE, STATE, A64_CTX_OFFSET, OpWidth::W64);
        e.emit_call_reg(target);
        e.emit_add_ri(PhysReg::Rsp, 8, OpWidth::W64);
        e.emit_pop(STATE);
    }

    fn lower_load(
        &mut self,
        dst: VReg,
        addr: &Address,
        width: MemWidth,
        sign: SignExtend,
    ) -> Result<(), LowerError> {
        let (_op_width, size) = Self::scalar_mem_width(width)?;
        self.load_addr_to(addr, ADDR)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B3, STATE, A64_LOAD_FN_OFFSET, OpWidth::W64);
            e.emit_mov_ri(HI, size, OpWidth::W64);
            e.emit_mov_ri(
                RHS,
                i64::from(matches!(sign, SignExtend::Sign)),
                OpWidth::W64,
            );
        }
        self.emit_mem_helper_call(B3);
        self.store_reg_to(dst, ACC, OpWidth::W64)
    }

    fn lower_load_exclusive(
        &mut self,
        dst: VReg,
        addr: &Address,
        width: MemWidth,
    ) -> Result<(), LowerError> {
        let (_op_width, size) = Self::scalar_mem_width(width)?;
        self.load_addr_to(addr, ADDR)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_mr(STATE, A64_EXCLUSIVE_ADDR_OFFSET, ADDR, OpWidth::W64);
            e.emit_mov_ri(B0, size, OpWidth::W64);
            e.emit_mov_mr(STATE, A64_EXCLUSIVE_SIZE_OFFSET, B0, OpWidth::W64);
            e.emit_mov_ri(B0, 1, OpWidth::W64);
            e.emit_mov_mr(STATE, A64_EXCLUSIVE_VALID_OFFSET, B0, OpWidth::W64);
            e.emit_mov_rm(B3, STATE, A64_LOAD_FN_OFFSET, OpWidth::W64);
            e.emit_mov_ri(HI, size, OpWidth::W64);
            e.emit_mov_ri(RHS, 0, OpWidth::W64);
        }
        self.emit_mem_helper_call(B3);
        self.store_reg_to(dst, ACC, OpWidth::W64)
    }

    fn lower_store(
        &mut self,
        src: VReg,
        addr: &Address,
        width: MemWidth,
    ) -> Result<(), LowerError> {
        let (op_width, size) = Self::scalar_mem_width(width)?;
        self.load_addr_to(addr, ADDR)?;
        self.load_vreg_to(src, HI, op_width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B3, STATE, A64_STORE_FN_OFFSET, OpWidth::W64);
            e.emit_mov_ri(RHS, size, OpWidth::W64);
        }
        self.emit_mem_helper_call(B3);
        Ok(())
    }

    fn setup_mem_load(&mut self, size: i64, signed: bool) {
        let mut e = X86Emitter::new(&mut self.code);
        e.emit_mov_rm(B3, STATE, A64_LOAD_FN_OFFSET, OpWidth::W64);
        e.emit_mov_ri(HI, size, OpWidth::W64);
        e.emit_mov_ri(RHS, i64::from(signed), OpWidth::W64);
    }

    fn setup_mem_store(&mut self, size: i64) {
        let mut e = X86Emitter::new(&mut self.code);
        e.emit_mov_rm(B3, STATE, A64_STORE_FN_OFFSET, OpWidth::W64);
        e.emit_mov_ri(RHS, size, OpWidth::W64);
    }

    fn lower_load_pair(
        &mut self,
        dst1: VReg,
        dst2: VReg,
        addr: &Address,
        width: MemWidth,
    ) -> Result<(), LowerError> {
        let (_op_width, size) = Self::scalar_mem_width(width)?;
        self.load_addr_to(addr, ADDR)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_sub_ri(PhysReg::Rsp, 16, OpWidth::W64);
            e.emit_mov_mr(PhysReg::Rsp, 0, ADDR, OpWidth::W64);
        }

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(ADDR, PhysReg::Rsp, 0, OpWidth::W64);
        }
        self.setup_mem_load(size, false);
        self.emit_mem_helper_call(B3);
        self.store_reg_to(dst1, ACC, OpWidth::W64)?;

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(ADDR, PhysReg::Rsp, 0, OpWidth::W64);
            e.emit_add_ri(ADDR, size, OpWidth::W64);
        }
        self.setup_mem_load(size, false);
        self.emit_mem_helper_call(B3);
        self.store_reg_to(dst2, ACC, OpWidth::W64)?;

        let mut e = X86Emitter::new(&mut self.code);
        e.emit_add_ri(PhysReg::Rsp, 16, OpWidth::W64);
        Ok(())
    }

    fn lower_store_pair(
        &mut self,
        src1: VReg,
        src2: VReg,
        addr: &Address,
        width: MemWidth,
    ) -> Result<(), LowerError> {
        let (op_width, size) = Self::scalar_mem_width(width)?;
        self.load_addr_to(addr, ADDR)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_sub_ri(PhysReg::Rsp, 16, OpWidth::W64);
            e.emit_mov_mr(PhysReg::Rsp, 0, ADDR, OpWidth::W64);
        }

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(ADDR, PhysReg::Rsp, 0, OpWidth::W64);
        }
        self.load_vreg_to(src1, HI, op_width)?;
        self.setup_mem_store(size);
        self.emit_mem_helper_call(B3);

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(ADDR, PhysReg::Rsp, 0, OpWidth::W64);
            e.emit_add_ri(ADDR, size, OpWidth::W64);
        }
        self.load_vreg_to(src2, HI, op_width)?;
        self.setup_mem_store(size);
        self.emit_mem_helper_call(B3);

        let mut e = X86Emitter::new(&mut self.code);
        e.emit_add_ri(PhysReg::Rsp, 16, OpWidth::W64);
        Ok(())
    }

    fn lower_store_exclusive(
        &mut self,
        status: VReg,
        src: VReg,
        addr: &Address,
        width: MemWidth,
    ) -> Result<(), LowerError> {
        let (op_width, size) = Self::scalar_mem_width(width)?;
        self.load_addr_to(addr, ADDR)?;
        self.load_vreg_to(src, HI, op_width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B0, STATE, A64_EXCLUSIVE_VALID_OFFSET, OpWidth::W64);
            e.emit_cmp_ri(B0, 1, OpWidth::W64);
        }
        let fail_valid = self.emit_jcc_placeholder(X86Cond::Ne);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B0, STATE, A64_EXCLUSIVE_ADDR_OFFSET, OpWidth::W64);
            e.emit_cmp_rr(B0, ADDR, OpWidth::W64);
        }
        let fail_addr = self.emit_jcc_placeholder(X86Cond::Ne);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B0, STATE, A64_EXCLUSIVE_SIZE_OFFSET, OpWidth::W64);
            e.emit_cmp_ri(B0, size, OpWidth::W64);
        }
        let fail_size = self.emit_jcc_placeholder(X86Cond::Ne);

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B3, STATE, A64_STORE_FN_OFFSET, OpWidth::W64);
            e.emit_mov_ri(RHS, size, OpWidth::W64);
        }
        self.emit_mem_helper_call(B3);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_xor_rr(ACC, ACC, OpWidth::W64);
        }
        let done = self.emit_jmp_placeholder();

        self.patch_rel32_to_current(fail_valid)?;
        self.patch_rel32_to_current(fail_addr)?;
        self.patch_rel32_to_current(fail_size)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_ri(ACC, 1, OpWidth::W64);
        }

        self.patch_rel32_to_current(done)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_xor_rr(B0, B0, OpWidth::W64);
            e.emit_mov_mr(STATE, A64_EXCLUSIVE_VALID_OFFSET, B0, OpWidth::W64);
        }
        self.store_reg_to(status, ACC, OpWidth::W32)
    }

    fn lower_atomic_rmw(
        &mut self,
        dst: VReg,
        addr: &Address,
        src: VReg,
        op: AtomicOp,
        width: MemWidth,
    ) -> Result<(), LowerError> {
        if op == AtomicOp::Nand {
            return Err(LowerError::UnsupportedOp {
                op: format!("AArch64 AtomicRmw op {op:?}"),
            });
        }

        let (op_width, size) = Self::scalar_mem_width(width)?;
        self.load_addr_to(addr, ADDR)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B3, STATE, A64_LOAD_FN_OFFSET, OpWidth::W64);
            e.emit_mov_ri(HI, size, OpWidth::W64);
            e.emit_mov_ri(RHS, 0, OpWidth::W64);
        }
        self.emit_mem_helper_call(B3);

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_sub_ri(PhysReg::Rsp, 16, OpWidth::W64);
            e.emit_mov_mr(PhysReg::Rsp, 0, ACC, OpWidth::W64);
        }

        self.load_vreg_to(src, HI, op_width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(ACC, PhysReg::Rsp, 0, OpWidth::W64);
            match op {
                AtomicOp::Add => e.emit_add_rr(ACC, HI, op_width),
                AtomicOp::Sub => e.emit_sub_rr(ACC, HI, op_width),
                AtomicOp::And => e.emit_and_rr(ACC, HI, op_width),
                AtomicOp::Or => e.emit_or_rr(ACC, HI, op_width),
                AtomicOp::Xor => e.emit_xor_rr(ACC, HI, op_width),
                AtomicOp::Swap => e.emit_mov_rr(ACC, HI, op_width),
                AtomicOp::Max | AtomicOp::Min | AtomicOp::Umax | AtomicOp::Umin => {
                    let signed = matches!(op, AtomicOp::Max | AtomicOp::Min);
                    match (op_width, signed) {
                        (OpWidth::W8 | OpWidth::W16 | OpWidth::W32, true) => {
                            e.emit_movsx(ACC, ACC, op_width, OpWidth::W64);
                            e.emit_movsx(HI, HI, op_width, OpWidth::W64);
                        }
                        (OpWidth::W8 | OpWidth::W16, false) => {
                            e.emit_movzx(ACC, ACC, op_width, OpWidth::W64);
                            e.emit_movzx(HI, HI, op_width, OpWidth::W64);
                        }
                        (OpWidth::W32, false) => {
                            e.emit_mov_rr(ACC, ACC, OpWidth::W32);
                            e.emit_mov_rr(HI, HI, OpWidth::W32);
                        }
                        (OpWidth::W64, _) => {}
                        (OpWidth::W128, _) => unreachable!(),
                    }
                    e.emit_cmp_rr(ACC, HI, OpWidth::W64);
                    let take_operand = match op {
                        AtomicOp::Max => X86Cond::L,
                        AtomicOp::Min => X86Cond::G,
                        AtomicOp::Umax => X86Cond::B,
                        AtomicOp::Umin => X86Cond::A,
                        _ => unreachable!(),
                    };
                    e.emit_cmovcc(take_operand, ACC, HI, OpWidth::W64);
                }
                AtomicOp::Nand => unreachable!(),
            }
            e.emit_mov_rr(HI, ACC, OpWidth::W64);
        }
        self.load_addr_to(addr, ADDR)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B3, STATE, A64_STORE_FN_OFFSET, OpWidth::W64);
            e.emit_mov_ri(RHS, size, OpWidth::W64);
        }
        self.emit_mem_helper_call(B3);

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(ACC, PhysReg::Rsp, 0, OpWidth::W64);
            e.emit_add_ri(PhysReg::Rsp, 16, OpWidth::W64);
        }
        self.store_reg_to(dst, ACC, OpWidth::W64)
    }

    fn lower_cas(
        &mut self,
        dst: VReg,
        success: VReg,
        addr: &Address,
        expected: VReg,
        new_val: VReg,
        width: MemWidth,
    ) -> Result<(), LowerError> {
        let (op_width, size) = Self::scalar_mem_width(width)?;
        self.load_addr_to(addr, ADDR)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B3, STATE, A64_LOAD_FN_OFFSET, OpWidth::W64);
            e.emit_mov_ri(HI, size, OpWidth::W64);
            e.emit_mov_ri(RHS, 0, OpWidth::W64);
        }
        self.emit_mem_helper_call(B3);

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_sub_ri(PhysReg::Rsp, 16, OpWidth::W64);
            e.emit_mov_mr(PhysReg::Rsp, 0, ACC, OpWidth::W64);
        }

        self.load_vreg_to(expected, RHS, op_width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_cmp_rr(ACC, RHS, op_width);
            e.emit_setcc(X86Cond::E, B0);
            e.emit_movzx(B0, B0, OpWidth::W8, OpWidth::W64);
            e.emit_mov_mr(PhysReg::Rsp, 8, B0, OpWidth::W64);
        }
        let skip_store = self.emit_jcc_placeholder(X86Cond::Ne);
        self.load_vreg_to(new_val, HI, op_width)?;
        self.load_addr_to(addr, ADDR)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B3, STATE, A64_STORE_FN_OFFSET, OpWidth::W64);
            e.emit_mov_ri(RHS, size, OpWidth::W64);
        }
        self.emit_mem_helper_call(B3);
        self.patch_rel32_to_current(skip_store)?;

        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B0, PhysReg::Rsp, 8, OpWidth::W64);
            e.emit_mov_rm(ACC, PhysReg::Rsp, 0, OpWidth::W64);
            e.emit_add_ri(PhysReg::Rsp, 16, OpWidth::W64);
        }
        self.store_reg_to(success, B0, OpWidth::W64)?;
        self.store_reg_to(dst, ACC, OpWidth::W64)
    }

    fn emit_jcc_placeholder(&mut self, cond: X86Cond) -> usize {
        let off = self.code.position();
        let mut e = X86Emitter::new(&mut self.code);
        e.emit_jcc_rel32(cond, 0);
        off + 2
    }

    fn emit_jmp_placeholder(&mut self) -> usize {
        let off = self.code.position();
        let mut e = X86Emitter::new(&mut self.code);
        e.emit_jmp_rel32(0);
        off + 1
    }

    fn patch_rel32_to_current(&mut self, offset: usize) -> Result<(), LowerError> {
        let target = self.code.position();
        let rel = target as i64 - offset as i64 - 4;
        if rel < i32::MIN as i64 || rel > i32::MAX as i64 {
            return Err(LowerError::RelocationOutOfRange { offset, target });
        }
        self.code.patch_i32(offset, rel as i32);
        Ok(())
    }

    fn ensure_div_width(width: OpWidth, op: &'static str) -> Result<(), LowerError> {
        match width {
            OpWidth::W32 | OpWidth::W64 => Ok(()),
            _ => Err(LowerError::UnsupportedOp {
                op: format!("AArch64 {op} width {width:?}"),
            }),
        }
    }

    fn lower_divu(
        &mut self,
        quot: VReg,
        rem: Option<VReg>,
        src1: VReg,
        src2: &SrcOperand,
        width: OpWidth,
    ) -> Result<(), LowerError> {
        Self::ensure_div_width(width, "DivU")?;
        self.load_src_to(src2, RHS, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_test_rr(RHS, RHS, width);
        }
        let div_path = self.emit_jcc_placeholder(X86Cond::Ne);
        self.emit_mov_imm(ACC, 0, width);
        self.store_reg_to(quot, ACC, width)?;
        if let Some(rem) = rem {
            self.store_reg_to(rem, ACC, width)?;
        }
        let done = self.emit_jmp_placeholder();

        self.patch_rel32_to_current(div_path)?;
        self.load_vreg_to(src1, ACC, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_zero_rdx();
            e.emit_div(RHS, width);
        }
        self.store_reg_to(quot, ACC, width)?;
        if let Some(rem) = rem {
            self.store_reg_to(rem, HI, width)?;
        }

        self.patch_rel32_to_current(done)
    }

    fn lower_divs(
        &mut self,
        quot: VReg,
        rem: Option<VReg>,
        src1: VReg,
        src2: &SrcOperand,
        width: OpWidth,
    ) -> Result<(), LowerError> {
        Self::ensure_div_width(width, "DivS")?;
        self.load_src_to(src2, RHS, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_test_rr(RHS, RHS, width);
        }
        let nonzero_path = self.emit_jcc_placeholder(X86Cond::Ne);
        self.emit_mov_imm(ACC, 0, width);
        self.store_reg_to(quot, ACC, width)?;
        if let Some(rem) = rem {
            self.store_reg_to(rem, ACC, width)?;
        }
        let zero_done = self.emit_jmp_placeholder();

        self.patch_rel32_to_current(nonzero_path)?;
        self.load_vreg_to(src1, ACC, width)?;
        let signed_min = match width {
            OpWidth::W32 => i32::MIN as i64,
            OpWidth::W64 => i64::MIN,
            _ => unreachable!(),
        };
        self.emit_mov_imm(B1, signed_min, width);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_cmp_rr(ACC, B1, width);
        }
        let not_min = self.emit_jcc_placeholder(X86Cond::Ne);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_cmp_ri(RHS, -1, width);
        }
        let not_overflow = self.emit_jcc_placeholder(X86Cond::Ne);
        self.store_reg_to(quot, ACC, width)?;
        if let Some(rem) = rem {
            self.emit_mov_imm(B1, 0, width);
            self.store_reg_to(rem, B1, width)?;
        }
        let overflow_done = self.emit_jmp_placeholder();

        self.patch_rel32_to_current(not_min)?;
        self.patch_rel32_to_current(not_overflow)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            match width {
                OpWidth::W32 => e.emit_cdq(),
                OpWidth::W64 => e.emit_cqo(),
                _ => unreachable!(),
            }
            e.emit_idiv(RHS, width);
        }
        self.store_reg_to(quot, ACC, width)?;
        if let Some(rem) = rem {
            self.store_reg_to(rem, HI, width)?;
        }

        self.patch_rel32_to_current(zero_done)?;
        self.patch_rel32_to_current(overflow_done)
    }

    fn emit_nzcv_from_flags(&mut self, form: FlagForm) {
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_setcc(X86Cond::S, B0);
            e.emit_setcc(X86Cond::E, B1);
            match form {
                FlagForm::Add => e.emit_setcc(X86Cond::B, B2),
                FlagForm::Sub => e.emit_setcc(X86Cond::Ae, B2),
                FlagForm::Logic => e.emit_mov_ri(B2, 0, OpWidth::W32),
            }
            match form {
                FlagForm::Add | FlagForm::Sub => e.emit_setcc(X86Cond::O, B3),
                FlagForm::Logic => e.emit_mov_ri(B3, 0, OpWidth::W32),
            }
            e.emit_movzx(B0, B0, OpWidth::W8, OpWidth::W64);
            e.emit_movzx(B1, B1, OpWidth::W8, OpWidth::W64);
            e.emit_movzx(B2, B2, OpWidth::W8, OpWidth::W64);
            e.emit_movzx(B3, B3, OpWidth::W8, OpWidth::W64);
            e.emit_shl_ri(B0, 31, OpWidth::W64);
            e.emit_shl_ri(B1, 30, OpWidth::W64);
            e.emit_shl_ri(B2, 29, OpWidth::W64);
            e.emit_shl_ri(B3, 28, OpWidth::W64);
            e.emit_or_rr(B0, B1, OpWidth::W64);
            e.emit_or_rr(B0, B2, OpWidth::W64);
            e.emit_or_rr(B0, B3, OpWidth::W64);
            e.emit_mov_mr(STATE, A64_NZCV_OFFSET, B0, OpWidth::W32);
        }
    }

    fn emit_nzcv_test_to_bool(&mut self, mask: i64, set: bool, dst: PhysReg) {
        let mut e = X86Emitter::new(&mut self.code);
        e.emit_mov_rm(ACC, STATE, A64_NZCV_OFFSET, OpWidth::W32);
        e.emit_test_ri(ACC, mask, OpWidth::W32);
        e.emit_setcc(if set { X86Cond::Ne } else { X86Cond::E }, dst);
        e.emit_movzx(dst, dst, OpWidth::W8, OpWidth::W64);
    }

    fn emit_cf_from_nzcv_c(&mut self, invert: bool) -> Result<(), LowerError> {
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rm(B0, STATE, A64_NZCV_OFFSET, OpWidth::W32);
            e.emit_test_ri(B0, NZCV_C, OpWidth::W32);
        }
        let c_set = self.emit_jcc_placeholder(X86Cond::Ne);
        {
            let mut e = X86Emitter::new(&mut self.code);
            if invert {
                e.emit_stc();
            } else {
                e.emit_clc();
            }
        }
        let done = self.emit_jmp_placeholder();
        self.patch_rel32_to_current(c_set)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            if invert {
                e.emit_clc();
            } else {
                e.emit_stc();
            }
        }
        self.patch_rel32_to_current(done)
    }

    fn emit_condition_to_reg(&mut self, cond: Condition, dst: PhysReg) -> Result<(), LowerError> {
        match cond {
            Condition::Always => {
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_mov_ri(dst, 1, OpWidth::W64);
            }
            Condition::Eq => self.emit_nzcv_test_to_bool(NZCV_Z, true, dst),
            Condition::Ne => self.emit_nzcv_test_to_bool(NZCV_Z, false, dst),
            Condition::Uge => self.emit_nzcv_test_to_bool(NZCV_C, true, dst),
            Condition::Ult => self.emit_nzcv_test_to_bool(NZCV_C, false, dst),
            Condition::Negative => self.emit_nzcv_test_to_bool(NZCV_N, true, dst),
            Condition::Positive => self.emit_nzcv_test_to_bool(NZCV_N, false, dst),
            Condition::Overflow => self.emit_nzcv_test_to_bool(NZCV_V, true, dst),
            Condition::NoOverflow => self.emit_nzcv_test_to_bool(NZCV_V, false, dst),
            Condition::Ule => {
                self.emit_nzcv_test_to_bool(NZCV_C, false, B0);
                self.emit_nzcv_test_to_bool(NZCV_Z, true, B1);
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_or_rr(B0, B1, OpWidth::W8);
                e.emit_movzx(dst, B0, OpWidth::W8, OpWidth::W64);
            }
            Condition::Ugt => {
                self.emit_nzcv_test_to_bool(NZCV_C, true, B0);
                self.emit_nzcv_test_to_bool(NZCV_Z, false, B1);
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_and_rr(B0, B1, OpWidth::W8);
                e.emit_movzx(dst, B0, OpWidth::W8, OpWidth::W64);
            }
            Condition::Slt | Condition::Sge | Condition::Sle | Condition::Sgt => {
                self.emit_nzcv_test_to_bool(NZCV_N, true, B0);
                self.emit_nzcv_test_to_bool(NZCV_V, true, B1);
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_xor_rr(B0, B1, OpWidth::W8);
                    if matches!(cond, Condition::Sge | Condition::Sgt) {
                        e.emit_xor_ri(B0, 1, OpWidth::W8);
                    }
                }
                if matches!(cond, Condition::Sle) {
                    self.emit_nzcv_test_to_bool(NZCV_Z, true, B1);
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_or_rr(B0, B1, OpWidth::W8);
                } else if matches!(cond, Condition::Sgt) {
                    self.emit_nzcv_test_to_bool(NZCV_Z, false, B1);
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_and_rr(B0, B1, OpWidth::W8);
                }
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_movzx(dst, B0, OpWidth::W8, OpWidth::W64);
            }
            Condition::Parity | Condition::NoParity => {
                return Err(LowerError::UnsupportedOp {
                    op: format!("AArch64 condition {:?}", cond),
                });
            }
        }
        Ok(())
    }

    fn lower_binop(
        &mut self,
        dst: VReg,
        src1: VReg,
        src2: &SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
        form: FlagForm,
    ) -> Result<(), LowerError> {
        self.load_vreg_to(src1, ACC, width)?;
        self.load_src_to(src2, RHS, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            match form {
                FlagForm::Add => e.emit_add_rr(ACC, RHS, width),
                FlagForm::Sub => e.emit_sub_rr(ACC, RHS, width),
                FlagForm::Logic => unreachable!(),
            }
        }
        self.store_reg_to(dst, ACC, width)?;
        if flags.updates_any() {
            self.emit_nzcv_from_flags(form);
        }
        Ok(())
    }

    fn lower_carry_binop(
        &mut self,
        dst: VReg,
        src1: VReg,
        src2: &SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
        subtract: bool,
    ) -> Result<(), LowerError> {
        match width {
            OpWidth::W32 | OpWidth::W64 => {}
            _ => {
                return Err(LowerError::UnsupportedOp {
                    op: format!("AArch64 carry binop width {width:?}"),
                });
            }
        }

        self.load_vreg_to(src1, ACC, width)?;
        self.load_src_to(src2, RHS, width)?;
        self.emit_cf_from_nzcv_c(subtract)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            if subtract {
                e.emit_sbb_rr(ACC, RHS, width);
            } else {
                e.emit_adc_rr(ACC, RHS, width);
            }
        }
        self.store_reg_to(dst, ACC, width)?;
        if flags.updates_any() {
            self.emit_nzcv_from_flags(if subtract {
                FlagForm::Sub
            } else {
                FlagForm::Add
            });
        }
        Ok(())
    }

    fn ensure_bitmanip_width(width: OpWidth, op: &'static str) -> Result<(), LowerError> {
        match width {
            OpWidth::W32 | OpWidth::W64 => Ok(()),
            _ => Err(LowerError::UnsupportedOp {
                op: format!("AArch64 {op} width {width:?}"),
            }),
        }
    }

    fn validate_bitfield_args(
        op: &'static str,
        lsb: u8,
        width_bits: u8,
        op_width: OpWidth,
    ) -> Result<u8, LowerError> {
        Self::ensure_bitmanip_width(op_width, op)?;
        let op_bits = op_width.bits() as u8;
        if width_bits == 0 || u16::from(lsb) + u16::from(width_bits) > u16::from(op_bits) {
            return Err(LowerError::UnsupportedOp {
                op: format!(
                    "AArch64 {op} field lsb {lsb} width {width_bits} op_width {op_width:?}"
                ),
            });
        }
        Ok(op_bits)
    }

    fn low_bit_mask(width_bits: u8, op_width: OpWidth) -> u64 {
        let op_bits = op_width.bits() as u8;
        if width_bits >= op_bits {
            op_width.mask()
        } else {
            (1u64 << width_bits) - 1
        }
    }

    fn lower_bswap(&mut self, dst: VReg, src: VReg, width: OpWidth) -> Result<(), LowerError> {
        Self::ensure_bitmanip_width(width, "Bswap")?;
        self.load_vreg_to(src, ACC, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_bswap(ACC, width);
        }
        self.store_reg_to(dst, ACC, width)
    }

    fn lower_bfx(
        &mut self,
        dst: VReg,
        src: VReg,
        lsb: u8,
        width_bits: u8,
        sign_extend: bool,
        op_width: OpWidth,
    ) -> Result<(), LowerError> {
        let op_bits = Self::validate_bitfield_args("Bfx", lsb, width_bits, op_width)?;
        self.load_vreg_to(src, ACC, op_width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            if lsb != 0 {
                e.emit_shr_ri(ACC, lsb, op_width);
            }
        }

        if width_bits < op_bits {
            if sign_extend {
                let shift = op_bits - width_bits;
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_shl_ri(ACC, shift, op_width);
                e.emit_sar_ri(ACC, shift, op_width);
            } else {
                self.emit_mov_imm(
                    B1,
                    Self::low_bit_mask(width_bits, op_width) as i64,
                    op_width,
                );
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_and_rr(ACC, B1, op_width);
            }
        }

        self.store_reg_to(dst, ACC, op_width)
    }

    fn lower_bfi(
        &mut self,
        dst: VReg,
        dst_in: VReg,
        src: VReg,
        lsb: u8,
        width_bits: u8,
        op_width: OpWidth,
    ) -> Result<(), LowerError> {
        let op_bits = Self::validate_bitfield_args("Bfi", lsb, width_bits, op_width)?;
        if width_bits == op_bits {
            self.load_vreg_to(src, ACC, op_width)?;
            return self.store_reg_to(dst, ACC, op_width);
        }

        let low_mask = Self::low_bit_mask(width_bits, op_width);
        let field_mask = low_mask << lsb;
        let clear_mask = (!field_mask) & op_width.mask();

        self.load_vreg_to(dst_in, ACC, op_width)?;
        self.load_vreg_to(src, RHS, op_width)?;
        self.emit_mov_imm(B1, low_mask as i64, op_width);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_and_rr(RHS, B1, op_width);
            if lsb != 0 {
                e.emit_shl_ri(RHS, lsb, op_width);
            }
        }
        self.emit_mov_imm(B1, clear_mask as i64, op_width);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_and_rr(ACC, B1, op_width);
            e.emit_or_rr(ACC, RHS, op_width);
        }
        self.store_reg_to(dst, ACC, op_width)
    }

    fn lower_clz(&mut self, dst: VReg, src: VReg, width: OpWidth) -> Result<(), LowerError> {
        Self::ensure_bitmanip_width(width, "Clz")?;
        self.load_vreg_to(src, ACC, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_test_rr(ACC, ACC, width);
        }
        let nonzero_path = self.emit_jcc_placeholder(X86Cond::Ne);
        self.emit_mov_imm(ACC, i64::from(width.bits()), width);
        let done = self.emit_jmp_placeholder();

        self.patch_rel32_to_current(nonzero_path)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_bsr(ACC, ACC, width);
        }
        self.emit_mov_imm(B0, i64::from(width.bits() - 1), width);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_sub_rr(B0, ACC, width);
            e.emit_mov_rr(ACC, B0, width);
        }

        self.patch_rel32_to_current(done)?;
        self.store_reg_to(dst, ACC, width)
    }

    fn emit_rbit_stage(&mut self, width: OpWidth, shift: u8, mask: u64) {
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_mov_rr(B0, ACC, width);
            e.emit_shr_ri(B0, shift, width);
        }
        self.emit_mov_imm(B1, mask as i64, width);
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_and_rr(B0, B1, width);
            e.emit_and_rr(ACC, B1, width);
            e.emit_shl_ri(ACC, shift, width);
            e.emit_or_rr(ACC, B0, width);
        }
    }

    fn lower_rbit(&mut self, dst: VReg, src: VReg, width: OpWidth) -> Result<(), LowerError> {
        Self::ensure_bitmanip_width(width, "Rbit")?;
        self.load_vreg_to(src, ACC, width)?;
        let masks = match width {
            OpWidth::W32 => [
                (1u8, 0x5555_5555_u64),
                (2u8, 0x3333_3333_u64),
                (4u8, 0x0f0f_0f0f_u64),
            ],
            OpWidth::W64 => [
                (1u8, 0x5555_5555_5555_5555_u64),
                (2u8, 0x3333_3333_3333_3333_u64),
                (4u8, 0x0f0f_0f0f_0f0f_0f0f_u64),
            ],
            _ => unreachable!(),
        };
        for (shift, mask) in masks {
            self.emit_rbit_stage(width, shift, mask);
        }
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_bswap(ACC, width);
        }
        self.store_reg_to(dst, ACC, width)
    }

    fn lower_mul(
        &mut self,
        dst_lo: VReg,
        dst_hi: Option<VReg>,
        src1: VReg,
        src2: &SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
        signed: bool,
    ) -> Result<(), LowerError> {
        let op_name = if signed { "MulS" } else { "MulU" };
        if flags.updates_any() {
            return Err(LowerError::UnsupportedOp {
                op: format!("AArch64 {op_name} with flags"),
            });
        }
        match width {
            OpWidth::W32 | OpWidth::W64 => {}
            _ => {
                return Err(LowerError::UnsupportedOp {
                    op: format!("AArch64 {op_name} width {width:?}"),
                });
            }
        }
        self.load_vreg_to(src1, ACC, width)?;
        self.load_src_to(src2, RHS, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            if signed {
                e.emit_imul(RHS, width);
            } else {
                e.emit_mul(RHS, width);
            }
        }
        self.store_reg_to(dst_lo, ACC, width)?;
        if let Some(dst_hi) = dst_hi {
            self.store_reg_to(dst_hi, HI, width)?;
        }
        Ok(())
    }

    fn lower_mul_acc(
        &mut self,
        dst: VReg,
        acc: VReg,
        src1: VReg,
        src2: VReg,
        width: OpWidth,
        subtract: bool,
    ) -> Result<(), LowerError> {
        match width {
            OpWidth::W32 | OpWidth::W64 => {}
            _ => {
                return Err(LowerError::UnsupportedOp {
                    op: format!("AArch64 multiply-accumulate width {width:?}"),
                });
            }
        }

        self.load_vreg_to(src1, ACC, width)?;
        self.load_vreg_to(src2, RHS, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            e.emit_imul_rr(ACC, RHS, width);
        }
        self.load_vreg_to(acc, RHS, width)?;
        if subtract {
            {
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_sub_rr(RHS, ACC, width);
            }
            self.store_reg_to(dst, RHS, width)
        } else {
            {
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_add_rr(ACC, RHS, width);
            }
            self.store_reg_to(dst, ACC, width)
        }
    }

    fn lower_logic(
        &mut self,
        dst: VReg,
        src1: VReg,
        src2: &SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
        op: LogicForm,
    ) -> Result<(), LowerError> {
        self.load_vreg_to(src1, ACC, width)?;
        self.load_src_to(src2, RHS, width)?;
        {
            let mut e = X86Emitter::new(&mut self.code);
            match op {
                LogicForm::And => e.emit_and_rr(ACC, RHS, width),
                LogicForm::Or => e.emit_or_rr(ACC, RHS, width),
                LogicForm::Xor => e.emit_xor_rr(ACC, RHS, width),
            }
        }
        self.store_reg_to(dst, ACC, width)?;
        if flags.updates_any() {
            self.emit_nzcv_from_flags(FlagForm::Logic);
        }
        Ok(())
    }

    fn lower_op(&mut self, op: &SmirOp) -> Result<(), LowerError> {
        match &op.kind {
            OpKind::Nop => {
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_nop();
            }
            OpKind::Mov { dst, src, width } => {
                self.load_src_to(src, ACC, *width)?;
                self.store_reg_to(*dst, ACC, *width)?;
            }
            OpKind::Add {
                dst,
                src1,
                src2,
                width,
                flags,
            } => self.lower_binop(*dst, *src1, src2, *width, *flags, FlagForm::Add)?,
            OpKind::Sub {
                dst,
                src1,
                src2,
                width,
                flags,
            } => self.lower_binop(*dst, *src1, src2, *width, *flags, FlagForm::Sub)?,
            OpKind::Adc {
                dst,
                src1,
                src2,
                width,
                flags,
            } => self.lower_carry_binop(*dst, *src1, src2, *width, *flags, false)?,
            OpKind::Sbb {
                dst,
                src1,
                src2,
                width,
                flags,
            } => self.lower_carry_binop(*dst, *src1, src2, *width, *flags, true)?,
            OpKind::Neg {
                dst,
                src,
                width,
                flags,
            } => {
                self.load_vreg_to(*src, ACC, *width)?;
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_neg(ACC, *width);
                }
                self.store_reg_to(*dst, ACC, *width)?;
                if flags.updates_any() {
                    self.emit_nzcv_from_flags(FlagForm::Sub);
                }
            }
            OpKind::MulU {
                dst_lo,
                dst_hi,
                src1,
                src2,
                width,
                flags,
            } => self.lower_mul(*dst_lo, *dst_hi, *src1, src2, *width, *flags, false)?,
            OpKind::MulS {
                dst_lo,
                dst_hi,
                src1,
                src2,
                width,
                flags,
            } => self.lower_mul(*dst_lo, *dst_hi, *src1, src2, *width, *flags, true)?,
            OpKind::MulAdd {
                dst,
                acc,
                src1,
                src2,
                width,
            } => self.lower_mul_acc(*dst, *acc, *src1, *src2, *width, false)?,
            OpKind::MulSub {
                dst,
                acc,
                src1,
                src2,
                width,
            } => self.lower_mul_acc(*dst, *acc, *src1, *src2, *width, true)?,
            OpKind::DivU {
                quot,
                rem,
                src1,
                src2,
                width,
                ..
            } => self.lower_divu(*quot, *rem, *src1, src2, *width)?,
            OpKind::DivS {
                quot,
                rem,
                src1,
                src2,
                width,
                ..
            } => self.lower_divs(*quot, *rem, *src1, src2, *width)?,
            OpKind::And {
                dst,
                src1,
                src2,
                width,
                flags,
            } => self.lower_logic(*dst, *src1, src2, *width, *flags, LogicForm::And)?,
            OpKind::Or {
                dst,
                src1,
                src2,
                width,
                flags,
            } => self.lower_logic(*dst, *src1, src2, *width, *flags, LogicForm::Or)?,
            OpKind::Xor {
                dst,
                src1,
                src2,
                width,
                flags,
            } => self.lower_logic(*dst, *src1, src2, *width, *flags, LogicForm::Xor)?,
            OpKind::AndNot {
                dst,
                src1,
                src2,
                width,
                flags,
            } => {
                self.load_vreg_to(*src1, ACC, *width)?;
                self.load_src_to(src2, RHS, *width)?;
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_not(RHS, *width);
                    e.emit_and_rr(ACC, RHS, *width);
                }
                self.store_reg_to(*dst, ACC, *width)?;
                if flags.updates_any() {
                    self.emit_nzcv_from_flags(FlagForm::Logic);
                }
            }
            OpKind::Not { dst, src, width } => {
                self.load_vreg_to(*src, ACC, *width)?;
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_not(ACC, *width);
                }
                self.store_reg_to(*dst, ACC, *width)?;
            }
            OpKind::Cmp { src1, src2, width } => {
                self.load_vreg_to(*src1, ACC, *width)?;
                self.load_src_to(src2, RHS, *width)?;
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_cmp_rr(ACC, RHS, *width);
                }
                self.emit_nzcv_from_flags(FlagForm::Sub);
            }
            OpKind::Test { src1, src2, width } => {
                self.load_vreg_to(*src1, ACC, *width)?;
                self.load_src_to(src2, RHS, *width)?;
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_test_rr(ACC, RHS, *width);
                }
                self.emit_nzcv_from_flags(FlagForm::Logic);
            }
            OpKind::Clz { dst, src, width } => self.lower_clz(*dst, *src, *width)?,
            OpKind::Bswap { dst, src, width } => self.lower_bswap(*dst, *src, *width)?,
            OpKind::Rbit { dst, src, width } => self.lower_rbit(*dst, *src, *width)?,
            OpKind::Bfx {
                dst,
                src,
                lsb,
                width_bits,
                sign_extend,
                op_width,
            } => self.lower_bfx(*dst, *src, *lsb, *width_bits, *sign_extend, *op_width)?,
            OpKind::Bfi {
                dst,
                dst_in,
                src,
                lsb,
                width_bits,
                op_width,
            } => self.lower_bfi(*dst, *dst_in, *src, *lsb, *width_bits, *op_width)?,
            OpKind::Shl {
                dst,
                src,
                amount,
                width,
                flags,
            } => {
                self.load_vreg_to(*src, ACC, *width)?;
                self.emit_shift_src(ACC, amount, ShiftOp::Lsl, *width)?;
                self.store_reg_to(*dst, ACC, *width)?;
                if flags.updates_any() {
                    self.emit_nzcv_from_flags(FlagForm::Logic);
                }
            }
            OpKind::Shr {
                dst,
                src,
                amount,
                width,
                flags,
            } => {
                self.load_vreg_to(*src, ACC, *width)?;
                self.emit_shift_src(ACC, amount, ShiftOp::Lsr, *width)?;
                self.store_reg_to(*dst, ACC, *width)?;
                if flags.updates_any() {
                    self.emit_nzcv_from_flags(FlagForm::Logic);
                }
            }
            OpKind::Sar {
                dst,
                src,
                amount,
                width,
                flags,
            } => {
                self.load_vreg_to(*src, ACC, *width)?;
                self.emit_shift_src(ACC, amount, ShiftOp::Asr, *width)?;
                self.store_reg_to(*dst, ACC, *width)?;
                if flags.updates_any() {
                    self.emit_nzcv_from_flags(FlagForm::Logic);
                }
            }
            OpKind::Ror {
                dst,
                src,
                amount,
                width,
                flags,
            } => {
                self.load_vreg_to(*src, ACC, *width)?;
                self.emit_shift_src(ACC, amount, ShiftOp::Ror, *width)?;
                self.store_reg_to(*dst, ACC, *width)?;
                if flags.updates_any() {
                    self.emit_nzcv_from_flags(FlagForm::Logic);
                }
            }
            OpKind::Rcl { .. } | OpKind::Rcr { .. } => {
                return Err(LowerError::UnsupportedOp {
                    op: "AArch64-to-x86 RCL/RCR".to_string(),
                });
            }
            OpKind::ZeroExtend {
                dst,
                src,
                from_width,
                to_width,
            } => {
                self.load_vreg_to(*src, ACC, *to_width)?;
                let mut e = X86Emitter::new(&mut self.code);
                match from_width {
                    OpWidth::W8 | OpWidth::W16 => e.emit_movzx(ACC, ACC, *from_width, *to_width),
                    OpWidth::W32 => e.emit_mov_rr(ACC, ACC, OpWidth::W32),
                    OpWidth::W64 | OpWidth::W128 => {}
                }
                self.store_reg_to(*dst, ACC, *to_width)?;
            }
            OpKind::SignExtend {
                dst,
                src,
                from_width,
                to_width,
            } => {
                self.load_vreg_to(*src, ACC, *to_width)?;
                let mut e = X86Emitter::new(&mut self.code);
                match from_width {
                    OpWidth::W8 | OpWidth::W16 | OpWidth::W32 => {
                        e.emit_movsx(ACC, ACC, *from_width, *to_width)
                    }
                    OpWidth::W64 | OpWidth::W128 => {}
                }
                self.store_reg_to(*dst, ACC, *to_width)?;
            }
            OpKind::Truncate {
                dst, src, to_width, ..
            } => {
                self.load_vreg_to(*src, ACC, *to_width)?;
                self.store_reg_to(*dst, ACC, *to_width)?;
            }
            OpKind::Select {
                dst,
                cond,
                src_true,
                src_false,
                width,
            } => {
                let cmov_width = match width {
                    OpWidth::W8 | OpWidth::W16 => OpWidth::W16,
                    OpWidth::W32 | OpWidth::W64 => *width,
                    OpWidth::W128 => {
                        return Err(LowerError::UnsupportedOp {
                            op: "AArch64 scalar Select width W128".into(),
                        });
                    }
                };
                self.load_vreg_to(*src_false, ACC, *width)?;
                self.load_vreg_to(*src_true, RHS, *width)?;
                self.load_vreg_to(*cond, B0, OpWidth::W64)?;
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_test_rr(B0, B0, OpWidth::W64);
                    e.emit_cmovcc(X86Cond::Ne, ACC, RHS, cmov_width);
                }
                self.store_reg_to(*dst, ACC, *width)?;
            }
            OpKind::Load {
                dst,
                addr,
                width,
                sign,
            } => self.lower_load(*dst, addr, *width, *sign)?,
            OpKind::LoadPair {
                dst1,
                dst2,
                addr,
                width,
            } => self.lower_load_pair(*dst1, *dst2, addr, *width)?,
            OpKind::LoadExclusive { dst, addr, width } => {
                self.lower_load_exclusive(*dst, addr, *width)?
            }
            OpKind::Store { src, addr, width } => self.lower_store(*src, addr, *width)?,
            OpKind::StorePair {
                src1,
                src2,
                addr,
                width,
            } => self.lower_store_pair(*src1, *src2, addr, *width)?,
            OpKind::StoreExclusive {
                status,
                src,
                addr,
                width,
            } => self.lower_store_exclusive(*status, *src, addr, *width)?,
            OpKind::ClearExclusive => {
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_xor_rr(ACC, ACC, OpWidth::W64);
                e.emit_mov_mr(STATE, A64_EXCLUSIVE_VALID_OFFSET, ACC, OpWidth::W64);
            }
            OpKind::AtomicRmw {
                dst,
                addr,
                src,
                op,
                width,
                ..
            } => self.lower_atomic_rmw(*dst, addr, *src, *op, *width)?,
            OpKind::Cas {
                dst,
                success,
                addr,
                expected,
                new_val,
                width,
                ..
            } => self.lower_cas(*dst, *success, addr, *expected, *new_val, *width)?,
            OpKind::Fence { .. } => {
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_mfence();
            }
            OpKind::TestCondition { dst, cond } | OpKind::SetCC { dst, cond, .. } => {
                self.emit_condition_to_reg(*cond, ACC)?;
                self.store_reg_to(*dst, ACC, OpWidth::W64)?;
            }
            _ => {
                return Err(LowerError::UnsupportedOp {
                    op: format!("AArch64 state-backed lowering for {:?}", op.kind),
                });
            }
        }
        Ok(())
    }

    fn lower_terminator(&mut self, block: &SmirBlock) -> Result<(), LowerError> {
        match &block.terminator {
            Terminator::Branch { target } => {
                let off = self.code.position();
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_jmp_rel32(0);
                self.pending_jumps
                    .push((off + 1, *target, RelocKind::PcRel32));
            }
            Terminator::CondBranch {
                cond,
                true_target,
                false_target,
            } => {
                self.load_vreg_to(*cond, ACC, OpWidth::W64)?;
                let jcc_off = self.code.position();
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_test_rr(ACC, ACC, OpWidth::W64);
                    e.emit_jcc_rel32(X86Cond::Ne, 0);
                }
                self.pending_jumps
                    .push((jcc_off + 5, *true_target, RelocKind::PcRel32));

                let jmp_off = self.code.position();
                {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_jmp_rel32(0);
                }
                self.pending_jumps
                    .push((jmp_off + 1, *false_target, RelocKind::PcRel32));
            }
            Terminator::IndirectBranch { target, .. } => {
                self.lower_exit_to_vreg(*target)?;
            }
            Terminator::Return { .. } => {
                if let Some(pc) = fallthrough_pc(block) {
                    let mut e = X86Emitter::new(&mut self.code);
                    e.emit_mov_ri_imm64(ACC, pc as i64);
                    e.emit_mov_mr(STATE, A64_PC_OFFSET, ACC, OpWidth::W64);
                }
                self.emit_epilogue();
            }
            Terminator::Call { target, .. } | Terminator::TailCall { target, .. } => {
                self.lower_exit_to_call_target(target)?;
            }
            Terminator::Trap { .. } | Terminator::Unreachable => {
                let mut e = X86Emitter::new(&mut self.code);
                e.emit_ud2();
            }
            other => {
                return Err(LowerError::UnsupportedOp {
                    op: format!("AArch64 state-backed terminator {other:?}"),
                });
            }
        }
        Ok(())
    }

    fn lower_block(&mut self, block: &SmirBlock) -> Result<(), LowerError> {
        self.block_offsets.insert(block.id, self.code.position());
        for op in &block.ops {
            self.lower_op(op)?;
        }
        self.lower_terminator(block)
    }

    fn fixup_jumps(&mut self) -> Result<(), LowerError> {
        for (offset, target, kind) in self.pending_jumps.drain(..).collect::<Vec<_>>() {
            let Some(&target_offset) = self.block_offsets.get(&target) else {
                return Err(LowerError::UndefinedLabel {
                    label: format!("block_{}", target.0),
                });
            };
            match kind {
                RelocKind::PcRel32 => {
                    let rel = target_offset as i64 - offset as i64 - 4;
                    if rel < i32::MIN as i64 || rel > i32::MAX as i64 {
                        return Err(LowerError::RelocationOutOfRange {
                            offset,
                            target: target_offset,
                        });
                    }
                    self.code.patch_i32(offset, rel as i32);
                }
                RelocKind::PcRel8 => {
                    let rel = target_offset as i64 - offset as i64 - 1;
                    if rel < -128 || rel > 127 {
                        return Err(LowerError::RelocationOutOfRange {
                            offset,
                            target: target_offset,
                        });
                    }
                    self.code.data[offset] = rel as i8 as u8;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Default for Aarch64X86_64Lowerer {
    fn default() -> Self {
        Self::new()
    }
}

impl SmirLowerer for Aarch64X86_64Lowerer {
    fn target_arch(&self) -> &'static str {
        "x86_64"
    }

    fn lower_function(&mut self, func: &SmirFunction) -> Result<LowerResult, LowerError> {
        self.code.clear();
        self.block_offsets.clear();
        self.pending_jumps.clear();
        self.relocations.clear();
        self.collect_virtuals(func);

        let entry_offset = self.code.position();
        self.emit_prologue();

        if let Some(entry) = func.get_block(func.entry) {
            self.lower_block(entry)?;
        }
        for block in &func.blocks {
            if block.id != func.entry {
                self.lower_block(block)?;
            }
        }
        self.fixup_jumps()?;

        Ok(LowerResult {
            code_size: self.code.len(),
            entry_offset,
            block_offsets: self.block_offsets.clone(),
            relocations: self.relocations.clone(),
            stack_size: self.frame_size,
        })
    }

    fn code_buffer(&self) -> &CodeBuffer {
        &self.code
    }

    fn finalize(&mut self) -> Result<Vec<u8>, LowerError> {
        Ok(self.code.data().to_vec())
    }
}

fn align16(n: usize) -> usize {
    (n + 15) & !15
}

fn fallthrough_pc(block: &SmirBlock) -> Option<u64> {
    block
        .ops
        .last()
        .map(|op| op.guest_pc + 4)
        .or(Some(block.guest_pc))
}

#[cfg(all(test, feature = "smir-jit", target_arch = "x86_64"))]
mod tests {
    use super::*;
    use crate::smir::flags::FlagUpdate;
    use crate::smir::ir::{FunctionBuilder, Terminator};
    use crate::smir::lower::runtime::{Aarch64GuestRegs, ExecMem};
    use crate::smir::types::{FenceKind, FunctionId};

    fn x(n: u8) -> VReg {
        VReg::Arch(ArchReg::Arm(ArmReg::X(n)))
    }

    fn run_func(func: &SmirFunction, regs: &mut Aarch64GuestRegs) {
        let mut lowerer = Aarch64X86_64Lowerer::new();
        let result = lowerer.lower_function(func).expect("lower AArch64 SMIR");
        let code = lowerer.finalize().expect("finalize");
        let mem = ExecMem::new(&code).expect("executable memory");
        mem.run_aarch64(result.entry_offset, regs);
    }

    #[repr(C)]
    struct TestMem {
        bytes: [u8; 64],
    }

    unsafe extern "C" fn test_load(ctx: u64, addr: u64, size: u64, signed: u64) -> u64 {
        let mem = unsafe { &*(ctx as *const TestMem) };
        let off = addr as usize;
        let size = size as usize;
        let mut value = 0u64;
        for i in 0..size {
            value |= u64::from(mem.bytes[off + i]) << (i * 8);
        }
        if signed != 0 && size < 8 {
            let bits = size * 8;
            let sign = 1u64 << (bits - 1);
            if (value & sign) != 0 {
                value |= u64::MAX << bits;
            }
        }
        value
    }

    unsafe extern "C" fn test_store(ctx: u64, addr: u64, value: u64, size: u64) -> u64 {
        let mem = unsafe { &mut *(ctx as *mut TestMem) };
        let off = addr as usize;
        for i in 0..size as usize {
            mem.bytes[off + i] = (value >> (i * 8)) as u8;
        }
        1
    }

    fn install_test_mem(regs: &mut Aarch64GuestRegs, mem: &mut TestMem) {
        regs.ctx = mem as *mut TestMem as usize as u64;
        regs.load_fn = test_load as *const () as usize as u64;
        regs.store_fn = test_store as *const () as usize as u64;
    }

    #[test]
    fn add_sets_aarch64_nzcv_and_writes_x() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x1000);
        b.push_op(
            0x1000,
            OpKind::Add {
                dst: x(0),
                src1: x(1),
                src2: SrcOperand::Reg(x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[1] = u64::MAX;
        regs.x[2] = 1;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0);
        assert_eq!(regs.nzcv & 0xF000_0000, 0x6000_0000, "Z and C set");
        assert_eq!(regs.pc, 0x1004);
    }

    #[test]
    fn w32_write_zero_extends_x_register_storage() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2000);
        b.push_op(
            0x2000,
            OpKind::Mov {
                dst: x(0),
                src: SrcOperand::Imm(-1),
                width: OpWidth::W32,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0xffff_ffff);
        assert_eq!(regs.pc, 0x2004);
    }

    #[test]
    fn select_uses_condition_temp_and_zero_extends_w32_result() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2400);
        let cond = b.alloc_vreg();
        b.push_op(
            0x2400,
            OpKind::TestCondition {
                dst: cond,
                cond: Condition::Eq,
            },
        );
        b.push_op(
            0x2400,
            OpKind::Select {
                dst: x(0),
                cond,
                src_true: x(1),
                src_false: x(2),
                width: OpWidth::W32,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });
        let func = b.finish();

        let mut regs = Aarch64GuestRegs::default();
        regs.x[1] = 0xffff_ffff_1234_5678;
        regs.x[2] = 0xffff_ffff_8765_4321;
        regs.nzcv = 0;
        run_func(&func, &mut regs);
        assert_eq!(regs.x[0], 0x8765_4321);

        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        regs.nzcv = 0x4000_0000;
        run_func(&func, &mut regs);
        assert_eq!(regs.x[0], 0x1234_5678);
    }

    #[test]
    fn mulu_w32_writes_low_product_and_zero_extends() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2800);
        b.push_op(
            0x2800,
            OpKind::MulU {
                dst_lo: x(0),
                dst_hi: None,
                src1: x(1),
                src2: SrcOperand::Reg(x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        regs.x[1] = 0xffff_ffff_ffff_fffe;
        regs.x[2] = 0xffff_ffff_0000_0003;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0xffff_fffa);
    }

    #[test]
    fn divu_zero_divisor_writes_zero_without_trap() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2c00);
        b.push_op(
            0x2c00,
            OpKind::DivU {
                quot: x(0),
                rem: None,
                src1: x(1),
                src2: SrcOperand::Reg(x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        regs.x[1] = 0x1234_5678_9abc_def0;
        regs.x[2] = 0;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0);
        assert_eq!(regs.pc, 0x2c04);
    }

    #[test]
    fn divs_w32_min_overflow_wraps_and_zero_extends() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2e00);
        b.push_op(
            0x2e00,
            OpKind::DivS {
                quot: x(0),
                rem: None,
                src1: x(1),
                src2: SrcOperand::Reg(x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        regs.x[1] = 0xffff_ffff_8000_0000;
        regs.x[2] = 0xffff_ffff_ffff_ffff;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0x8000_0000);
        assert_eq!(regs.pc, 0x2e04);
    }

    #[test]
    fn signed_w_load_zero_extends_after_helper_result() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2f00);
        let tmp = b.alloc_vreg();
        b.push_op(
            0x2f00,
            OpKind::Load {
                dst: tmp,
                addr: Address::BaseOffset {
                    base: x(1),
                    offset: 4,
                    disp_size: crate::smir::types::DispSize::Auto,
                },
                width: MemWidth::B1,
                sign: SignExtend::Sign,
            },
        );
        b.push_op(
            0x2f00,
            OpKind::ZeroExtend {
                dst: x(0),
                src: tmp,
                from_width: OpWidth::W32,
                to_width: OpWidth::W64,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut mem = TestMem { bytes: [0; 64] };
        mem.bytes[12] = 0x80;
        let mut regs = Aarch64GuestRegs::default();
        install_test_mem(&mut regs, &mut mem);
        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        regs.x[1] = 8;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0xffff_ff80);
        assert_eq!(regs.pc, 0x2f04);
    }

    #[test]
    fn store_w32_truncates_value_through_helper() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2f80);
        b.push_op(
            0x2f80,
            OpKind::Store {
                src: x(0),
                addr: Address::Direct(x(1)),
                width: MemWidth::B4,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut mem = TestMem { bytes: [0; 64] };
        let mut regs = Aarch64GuestRegs::default();
        install_test_mem(&mut regs, &mut mem);
        regs.x[0] = 0x1122_3344_5566_7788;
        regs.x[1] = 16;
        run_func(&b.finish(), &mut regs);

        assert_eq!(&mem.bytes[16..20], &[0x88, 0x77, 0x66, 0x55]);
        assert_eq!(regs.x[0], 0x1122_3344_5566_7788);
        assert_eq!(regs.pc, 0x2f84);
    }

    #[test]
    fn fence_full_lowers_without_clobbering_state() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2fc0);
        b.push_op(
            0x2fc0,
            OpKind::Fence {
                kind: FenceKind::Full,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[0] = 0x1122_3344_5566_7788;
        regs.x[1] = 0x99aa_bbcc_ddee_ff00;
        regs.nzcv = 0x9000_0000;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0x1122_3344_5566_7788);
        assert_eq!(regs.x[1], 0x99aa_bbcc_ddee_ff00);
        assert_eq!(regs.nzcv & 0xF000_0000, 0x9000_0000);
        assert_eq!(regs.pc, 0x2fc4);
    }

    #[test]
    fn clz_w32_counts_zero_and_ignores_high_bits() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2fd0);
        b.push_op(
            0x2fd0,
            OpKind::Clz {
                dst: x(0),
                src: x(1),
                width: OpWidth::W32,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });
        let func = b.finish();

        let mut regs = Aarch64GuestRegs::default();
        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        regs.x[1] = 0xffff_ffff_0000_0000;
        run_func(&func, &mut regs);
        assert_eq!(regs.x[0], 32);

        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        regs.x[1] = 0xffff_ffff_0000_0001;
        run_func(&func, &mut regs);
        assert_eq!(regs.x[0], 31);
    }

    #[test]
    fn bswap_x64_reverses_bytes() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2fe0);
        b.push_op(
            0x2fe0,
            OpKind::Bswap {
                dst: x(0),
                src: x(1),
                width: OpWidth::W64,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[1] = 0x1122_3344_5566_7788;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0x8877_6655_4433_2211);
    }

    #[test]
    fn muls_x64_writes_signed_high_half() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2fe8);
        b.push_op(
            0x2fe8,
            OpKind::MulS {
                dst_lo: x(3),
                dst_hi: Some(x(0)),
                src1: x(1),
                src2: SrcOperand::Reg(x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[1] = i64::MIN as u64;
        regs.x[2] = 2;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], u64::MAX);
        assert_eq!(regs.x[3], 0);
    }

    #[test]
    fn rbit_w32_reverses_bits_and_zero_extends() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x2ff0);
        b.push_op(
            0x2ff0,
            OpKind::Rbit {
                dst: x(0),
                src: x(1),
                width: OpWidth::W32,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[0] = 0xaaaa_bbbb_cccc_dddd;
        regs.x[1] = 0xffff_ffff_0000_0001;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0x8000_0000);
    }

    #[test]
    fn sub_sets_no_borrow_carry_and_signed_overflow() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x3000);
        b.push_op(
            0x3000,
            OpKind::Sub {
                dst: x(0),
                src1: x(1),
                src2: SrcOperand::Reg(x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[1] = 0x8000_0000_0000_0000;
        regs.x[2] = 1;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0x7fff_ffff_ffff_ffff);
        assert_eq!(regs.nzcv & 0xF000_0000, 0x3000_0000, "C and V set");
    }

    #[test]
    fn adc_reads_aarch64_carry_and_sets_flags() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x3400);
        b.push_op(
            0x3400,
            OpKind::Adc {
                dst: x(0),
                src1: x(1),
                src2: SrcOperand::Reg(x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[1] = u64::MAX;
        regs.x[2] = 0;
        regs.nzcv = 0x2000_0000;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], 0);
        assert_eq!(regs.nzcv & 0xF000_0000, 0x6000_0000, "Z and C set");
    }

    #[test]
    fn sbc_inverts_aarch64_carry_for_borrow_and_sets_flags() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x3800);
        b.push_op(
            0x3800,
            OpKind::Sbb {
                dst: x(0),
                src1: x(1),
                src2: SrcOperand::Reg(x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let mut regs = Aarch64GuestRegs::default();
        regs.x[1] = 0;
        regs.x[2] = 1;
        regs.nzcv = 0;
        run_func(&b.finish(), &mut regs);

        assert_eq!(regs.x[0], u64::MAX - 1);
        assert_eq!(regs.nzcv & 0xF000_0000, 0x8000_0000, "N set and C clear");
    }

    #[test]
    fn test_condition_reads_stored_nzcv() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x4000);
        b.push_op(
            0x4000,
            OpKind::TestCondition {
                dst: x(0),
                cond: Condition::Uge,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });
        let func = b.finish();

        let mut regs = Aarch64GuestRegs::default();
        regs.nzcv = 0x2000_0000;
        run_func(&func, &mut regs);
        assert_eq!(regs.x[0], 1);

        regs.x[0] = 99;
        regs.nzcv = 0;
        run_func(&func, &mut regs);
        assert_eq!(regs.x[0], 0);
    }

    #[test]
    fn cond_branch_through_virtual_temp_selects_target() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x5000);
        let taken = b.create_block(0x5010);
        let not_taken = b.create_block(0x5020);
        let cond = b.alloc_vreg();

        b.push_op(
            0x5000,
            OpKind::TestCondition {
                dst: cond,
                cond: Condition::Eq,
            },
        );
        b.set_terminator(Terminator::CondBranch {
            cond,
            true_target: taken,
            false_target: not_taken,
        });

        b.switch_to_block(taken);
        b.push_op(
            0x5010,
            OpKind::Mov {
                dst: x(0),
                src: SrcOperand::Imm(1),
                width: OpWidth::W64,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        b.switch_to_block(not_taken);
        b.push_op(
            0x5020,
            OpKind::Mov {
                dst: x(0),
                src: SrcOperand::Imm(2),
                width: OpWidth::W64,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });

        let func = b.finish();
        let mut regs = Aarch64GuestRegs::default();
        regs.nzcv = 0x4000_0000;
        run_func(&func, &mut regs);
        assert_eq!(regs.x[0], 1);
        assert_eq!(regs.pc, 0x5014);

        regs.x[0] = 0;
        regs.nzcv = 0;
        run_func(&func, &mut regs);
        assert_eq!(regs.x[0], 2);
        assert_eq!(regs.pc, 0x5024);
    }
}
