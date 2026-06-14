//! Register allocation for SMIR lowering.
//!
//! This module implements a simple linear-scan register allocator for
//! mapping SMIR virtual registers to physical machine registers.

use std::collections::{HashMap, HashSet};

use crate::smir::ir::{SmirBlock, SmirFunction};
use crate::smir::ops::OpKind;
use crate::smir::types::{ArchReg, OpId, VReg, VirtualId, X86Reg};

use super::LowerError;

// ============================================================================
// Physical Register
// ============================================================================

/// Physical x86_64 register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhysReg {
    // General purpose registers
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    // Vector registers
    Xmm(u8),
    Ymm(u8),
    Zmm(u8),
}

impl PhysReg {
    /// Get register encoding (for ModR/M, etc.)
    pub fn encoding(self) -> u8 {
        match self {
            PhysReg::Rax => 0,
            PhysReg::Rcx => 1,
            PhysReg::Rdx => 2,
            PhysReg::Rbx => 3,
            PhysReg::Rsp => 4,
            PhysReg::Rbp => 5,
            PhysReg::Rsi => 6,
            PhysReg::Rdi => 7,
            PhysReg::R8 => 8,
            PhysReg::R9 => 9,
            PhysReg::R10 => 10,
            PhysReg::R11 => 11,
            PhysReg::R12 => 12,
            PhysReg::R13 => 13,
            PhysReg::R14 => 14,
            PhysReg::R15 => 15,
            PhysReg::Xmm(idx) | PhysReg::Ymm(idx) | PhysReg::Zmm(idx) => idx,
        }
    }

    /// Check if this is an extended register (R8-R15)
    pub fn is_extended(self) -> bool {
        self.encoding() >= 8
    }

    /// Check if this register requires REX.B for reg field
    pub fn needs_rex_b(self) -> bool {
        self.is_extended()
    }

    /// Check if this register requires REX.R for reg field
    pub fn needs_rex_r(self) -> bool {
        self.is_extended()
    }

    /// Check if this register requires REX.X for index field
    pub fn needs_rex_x(self) -> bool {
        self.is_extended()
    }

    /// Get the low 3 bits of encoding (for instruction encoding)
    pub fn low3(self) -> u8 {
        self.encoding() & 0x7
    }

    pub fn vec_ext(self) -> u8 {
        (self.encoding() >> 3) & 0x1
    }

    pub fn vec_ext2(self) -> u8 {
        (self.encoding() >> 4) & 0x1
    }

    pub fn is_vec(self) -> bool {
        matches!(self, PhysReg::Xmm(_) | PhysReg::Ymm(_) | PhysReg::Zmm(_))
    }

    pub fn is_xmm(self) -> bool {
        matches!(self, PhysReg::Xmm(_))
    }

    pub fn is_ymm(self) -> bool {
        matches!(self, PhysReg::Ymm(_))
    }

    pub fn is_zmm(self) -> bool {
        matches!(self, PhysReg::Zmm(_))
    }

    /// All general purpose registers
    pub const ALL_GPR: [PhysReg; 16] = [
        PhysReg::Rax,
        PhysReg::Rcx,
        PhysReg::Rdx,
        PhysReg::Rbx,
        PhysReg::Rsp,
        PhysReg::Rbp,
        PhysReg::Rsi,
        PhysReg::Rdi,
        PhysReg::R8,
        PhysReg::R9,
        PhysReg::R10,
        PhysReg::R11,
        PhysReg::R12,
        PhysReg::R13,
        PhysReg::R14,
        PhysReg::R15,
    ];

    /// Caller-saved registers (scratch registers)
    pub const CALLER_SAVED: [PhysReg; 9] = [
        PhysReg::Rax,
        PhysReg::Rcx,
        PhysReg::Rdx,
        PhysReg::Rsi,
        PhysReg::Rdi,
        PhysReg::R8,
        PhysReg::R9,
        PhysReg::R10,
        PhysReg::R11,
    ];

    /// Callee-saved registers
    pub const CALLEE_SAVED: [PhysReg; 5] = [
        PhysReg::Rbx,
        PhysReg::R12,
        PhysReg::R13,
        PhysReg::R14,
        PhysReg::R15,
    ];

    /// Registers available for allocation (excluding RSP, RBP)
    pub const ALLOCATABLE: [PhysReg; 14] = [
        PhysReg::Rax,
        PhysReg::Rcx,
        PhysReg::Rdx,
        PhysReg::Rbx,
        PhysReg::Rsi,
        PhysReg::Rdi,
        PhysReg::R8,
        PhysReg::R9,
        PhysReg::R10,
        PhysReg::R11,
        PhysReg::R12,
        PhysReg::R13,
        PhysReg::R14,
        PhysReg::R15,
    ];

    /// Convert from X86Reg
    pub fn from_x86_reg(reg: X86Reg) -> Option<PhysReg> {
        match reg {
            X86Reg::Rax => Some(PhysReg::Rax),
            X86Reg::Rcx => Some(PhysReg::Rcx),
            X86Reg::Rdx => Some(PhysReg::Rdx),
            X86Reg::Rbx => Some(PhysReg::Rbx),
            X86Reg::Rsp => Some(PhysReg::Rsp),
            X86Reg::Rbp => Some(PhysReg::Rbp),
            X86Reg::Rsi => Some(PhysReg::Rsi),
            X86Reg::Rdi => Some(PhysReg::Rdi),
            X86Reg::R8 => Some(PhysReg::R8),
            X86Reg::R9 => Some(PhysReg::R9),
            X86Reg::R10 => Some(PhysReg::R10),
            X86Reg::R11 => Some(PhysReg::R11),
            X86Reg::R12 => Some(PhysReg::R12),
            X86Reg::R13 => Some(PhysReg::R13),
            X86Reg::R14 => Some(PhysReg::R14),
            X86Reg::R15 => Some(PhysReg::R15),
            X86Reg::Xmm(n) => Some(PhysReg::Xmm(n)),
            X86Reg::Ymm(n) => Some(PhysReg::Ymm(n)),
            X86Reg::Zmm(n) => Some(PhysReg::Zmm(n)),
            _ => None,
        }
    }

    /// Convert to X86Reg
    pub fn to_x86_reg(self) -> X86Reg {
        match self {
            PhysReg::Rax => X86Reg::Rax,
            PhysReg::Rcx => X86Reg::Rcx,
            PhysReg::Rdx => X86Reg::Rdx,
            PhysReg::Rbx => X86Reg::Rbx,
            PhysReg::Rsp => X86Reg::Rsp,
            PhysReg::Rbp => X86Reg::Rbp,
            PhysReg::Rsi => X86Reg::Rsi,
            PhysReg::Rdi => X86Reg::Rdi,
            PhysReg::R8 => X86Reg::R8,
            PhysReg::R9 => X86Reg::R9,
            PhysReg::R10 => X86Reg::R10,
            PhysReg::R11 => X86Reg::R11,
            PhysReg::R12 => X86Reg::R12,
            PhysReg::R13 => X86Reg::R13,
            PhysReg::R14 => X86Reg::R14,
            PhysReg::R15 => X86Reg::R15,
            PhysReg::Xmm(n) => X86Reg::Xmm(n),
            PhysReg::Ymm(n) => X86Reg::Ymm(n),
            PhysReg::Zmm(n) => X86Reg::Zmm(n),
        }
    }
}

// ============================================================================
// Register Location
// ============================================================================

/// Location of a value (register or stack slot)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegLocation {
    /// Value is in a physical register
    Register(PhysReg),

    /// Value is on the stack at [RBP - offset]
    Stack(i32),

    /// Value is a constant (no allocation needed)
    Constant(i64),

    /// Value is not yet allocated
    Unallocated,
}

impl RegLocation {
    /// Check if this is a register
    pub fn is_register(&self) -> bool {
        matches!(self, RegLocation::Register(_))
    }

    /// Check if this is a stack slot
    pub fn is_stack(&self) -> bool {
        matches!(self, RegLocation::Stack(_))
    }

    /// Get the physical register if this is a register location
    pub fn as_register(&self) -> Option<PhysReg> {
        if let RegLocation::Register(r) = self {
            Some(*r)
        } else {
            None
        }
    }

    /// Get the stack offset if this is a stack location
    pub fn as_stack_offset(&self) -> Option<i32> {
        if let RegLocation::Stack(off) = self {
            Some(*off)
        } else {
            None
        }
    }
}

// ============================================================================
// Liveness Analysis
// ============================================================================

/// Liveness information for a virtual register
#[derive(Clone, Debug)]
pub struct LiveRange {
    /// First use (op index)
    pub start: usize,

    /// Last use (op index)
    pub end: usize,

    /// Is this a definition (vs just a use)
    pub is_def: bool,
}

/// Compute live ranges for all virtual registers in a block
pub fn compute_live_ranges(block: &SmirBlock) -> HashMap<VirtualId, LiveRange> {
    let mut ranges: HashMap<VirtualId, LiveRange> = HashMap::new();

    for (idx, op) in block.ops.iter().enumerate() {
        // Get destinations (definitions)
        for dst in op.kind.dests() {
            if let VReg::Virtual(id) = dst {
                ranges
                    .entry(id)
                    .and_modify(|r| {
                        r.start = r.start.min(idx);
                        r.end = r.end.max(idx);
                    })
                    .or_insert(LiveRange {
                        start: idx,
                        end: idx,
                        is_def: true,
                    });
            }
        }

        // Get sources (uses)
        for src in op.kind.source_vregs() {
            if let VReg::Virtual(id) = src {
                ranges
                    .entry(id)
                    .and_modify(|r| {
                        r.start = r.start.min(idx);
                        r.end = r.end.max(idx);
                    })
                    .or_insert(LiveRange {
                        start: idx,
                        end: idx,
                        is_def: false,
                    });
            }
        }
    }

    ranges
}

// ============================================================================
// Register Allocator
// ============================================================================

/// Register allocator state
pub struct RegAlloc {
    /// Mapping from virtual register to location
    vreg_locations: HashMap<VirtualId, RegLocation>,

    /// Mapping from arch register to location
    arch_locations: HashMap<ArchReg, RegLocation>,

    /// Which physical registers are currently free
    free_regs: Vec<PhysReg>,

    /// Which physical registers are currently in use
    used_regs: HashSet<PhysReg>,

    /// Callee-saved registers that we've used (need to save/restore)
    callee_saved_used: Vec<PhysReg>,

    /// Next stack slot offset (grows negative from RBP)
    next_stack_slot: i32,

    /// Stack slots used for spills
    spill_slots: Vec<i32>,

    /// Live ranges (computed once per block)
    live_ranges: HashMap<VirtualId, LiveRange>,

    /// Current instruction index (for liveness checking)
    current_idx: usize,
}

impl RegAlloc {
    /// Create a new register allocator
    pub fn new() -> Self {
        let free_regs: Vec<PhysReg> = PhysReg::ALLOCATABLE.into_iter().collect();

        RegAlloc {
            vreg_locations: HashMap::new(),
            arch_locations: HashMap::new(),
            free_regs,
            used_regs: HashSet::new(),
            callee_saved_used: Vec::new(),
            next_stack_slot: -8, // First slot at [RBP-8]
            spill_slots: Vec::new(),
            live_ranges: HashMap::new(),
            current_idx: 0,
        }
    }

    /// Prepare for a new block
    pub fn begin_block(&mut self, block: &SmirBlock) {
        self.live_ranges = compute_live_ranges(block);
        self.current_idx = 0;
    }

    /// Set current instruction index (for liveness tracking)
    pub fn set_current_idx(&mut self, idx: usize) {
        self.current_idx = idx;

        // Free registers for values that are no longer live
        let mut to_free = Vec::new();
        for (&vid, &loc) in &self.vreg_locations {
            if let Some(range) = self.live_ranges.get(&vid) {
                if range.end < idx {
                    if let RegLocation::Register(r) = loc {
                        to_free.push((vid, r));
                    }
                }
            }
        }

        for (vid, r) in to_free {
            self.vreg_locations.remove(&vid);
            self.free_reg(r);
        }
    }

    /// Allocate a register for a virtual register
    pub fn alloc_vreg(&mut self, vreg: VReg) -> Result<RegLocation, LowerError> {
        match vreg {
            VReg::Virtual(id) => {
                // Check if already allocated
                if let Some(&loc) = self.vreg_locations.get(&id) {
                    return Ok(loc);
                }

                // Try to allocate a physical register
                if let Some(reg) = self.alloc_phys_reg() {
                    let loc = RegLocation::Register(reg);
                    self.vreg_locations.insert(id, loc);
                    Ok(loc)
                } else {
                    // Need to spill - allocate stack slot
                    let slot = self.alloc_stack_slot();
                    let loc = RegLocation::Stack(slot);
                    self.vreg_locations.insert(id, loc);
                    Ok(loc)
                }
            }

            VReg::Arch(arch_reg) => {
                // Arch registers have fixed locations
                if let Some(&loc) = self.arch_locations.get(&arch_reg) {
                    return Ok(loc);
                }

                // Map to physical register if possible
                if let ArchReg::X86(x86_reg) = arch_reg {
                    if let Some(phys) = PhysReg::from_x86_reg(x86_reg) {
                        // Mark as used
                        self.use_phys_reg(phys);
                        let loc = RegLocation::Register(phys);
                        self.arch_locations.insert(arch_reg, loc);
                        return Ok(loc);
                    }
                    return Err(LowerError::UnsupportedOp {
                        op: format!("state-backed x86 register {x86_reg:?}"),
                    });
                }

                // Otherwise allocate normally
                if let Some(reg) = self.alloc_phys_reg() {
                    let loc = RegLocation::Register(reg);
                    self.arch_locations.insert(arch_reg, loc);
                    Ok(loc)
                } else {
                    let slot = self.alloc_stack_slot();
                    let loc = RegLocation::Stack(slot);
                    self.arch_locations.insert(arch_reg, loc);
                    Ok(loc)
                }
            }

            VReg::Imm(val) => Ok(RegLocation::Constant(val)),
        }
    }

    /// Get the location of a virtual register (must already be allocated)
    pub fn get_vreg_location(&self, vreg: VReg) -> Option<RegLocation> {
        match vreg {
            VReg::Virtual(id) => self.vreg_locations.get(&id).copied(),
            VReg::Arch(arch_reg) => self.arch_locations.get(&arch_reg).copied(),
            VReg::Imm(val) => Some(RegLocation::Constant(val)),
        }
    }

    /// Allocate a temporary register (caller must free it)
    pub fn alloc_temp(&mut self) -> Result<PhysReg, LowerError> {
        self.alloc_phys_reg()
            .ok_or_else(|| LowerError::RegisterAllocationFailed {
                reason: "no free registers for temporary".to_string(),
            })
    }

    /// Free a temporary register
    pub fn free_temp(&mut self, reg: PhysReg) {
        self.free_reg(reg);
    }

    /// Get a scratch register (spill if needed)
    pub fn get_scratch(&mut self) -> Result<PhysReg, LowerError> {
        if let Some(reg) = self.alloc_phys_reg() {
            Ok(reg)
        } else {
            // Need to spill something
            // For simplicity, spill RAX
            // A better implementation would pick the register with the furthest next use
            self.spill_reg(PhysReg::Rax)?;
            Ok(PhysReg::Rax)
        }
    }

    /// Spill a register to stack
    pub fn spill_reg(&mut self, reg: PhysReg) -> Result<i32, LowerError> {
        // Find which vreg is in this register
        let mut vreg_to_spill = None;
        for (&vid, &loc) in &self.vreg_locations {
            if loc == RegLocation::Register(reg) {
                vreg_to_spill = Some(vid);
                break;
            }
        }

        if let Some(vid) = vreg_to_spill {
            let slot = self.alloc_stack_slot();
            self.vreg_locations.insert(vid, RegLocation::Stack(slot));
            self.free_reg(reg);
            Ok(slot)
        } else {
            // Register wasn't holding a vreg, just mark it free
            self.free_reg(reg);
            Ok(0) // No actual spill needed
        }
    }

    /// Allocate a physical register from the free list
    fn alloc_phys_reg(&mut self) -> Option<PhysReg> {
        // Prefer caller-saved registers first
        for &reg in &PhysReg::CALLER_SAVED {
            if let Some(pos) = self.free_regs.iter().position(|&r| r == reg) {
                self.free_regs.remove(pos);
                self.used_regs.insert(reg);
                return Some(reg);
            }
        }

        // Then try callee-saved
        if let Some(reg) = self.free_regs.pop() {
            self.used_regs.insert(reg);

            // Track callee-saved usage
            if PhysReg::CALLEE_SAVED.contains(&reg) && !self.callee_saved_used.contains(&reg) {
                self.callee_saved_used.push(reg);
            }

            return Some(reg);
        }

        None
    }

    /// Mark a physical register as used (for fixed register constraints)
    fn use_phys_reg(&mut self, reg: PhysReg) {
        if let Some(pos) = self.free_regs.iter().position(|&r| r == reg) {
            self.free_regs.remove(pos);
        }
        self.used_regs.insert(reg);

        if PhysReg::CALLEE_SAVED.contains(&reg) && !self.callee_saved_used.contains(&reg) {
            self.callee_saved_used.push(reg);
        }
    }

    /// Free a physical register
    fn free_reg(&mut self, reg: PhysReg) {
        if self.used_regs.remove(&reg) && PhysReg::ALLOCATABLE.contains(&reg) {
            self.free_regs.push(reg);
        }
    }

    /// Allocate a stack slot
    fn alloc_stack_slot(&mut self) -> i32 {
        let slot = self.next_stack_slot;
        self.next_stack_slot -= 8; // 8-byte slots
        self.spill_slots.push(slot);
        slot
    }

    /// Get total stack space needed for spills
    pub fn spill_stack_size(&self) -> usize {
        self.spill_slots.len() * 8
    }

    /// Get callee-saved registers that were used
    pub fn callee_saved_used(&self) -> &[PhysReg] {
        &self.callee_saved_used
    }

    /// Total stack frame size (spills + callee-saved)
    pub fn frame_size(&self) -> usize {
        let spill_size = self.spill_stack_size();
        let save_size = self.callee_saved_used.len() * 8;
        // Round up to 16-byte alignment
        let total = spill_size + save_size;
        (total + 15) & !15
    }

    /// Reset allocator state for a new function
    pub fn reset(&mut self) {
        self.vreg_locations.clear();
        self.arch_locations.clear();
        self.free_regs = PhysReg::ALLOCATABLE.into_iter().collect();
        self.used_regs.clear();
        self.callee_saved_used.clear();
        self.next_stack_slot = -8;
        self.spill_slots.clear();
        self.live_ranges.clear();
        self.current_idx = 0;
    }
}

impl Default for RegAlloc {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phys_reg_encoding() {
        assert_eq!(PhysReg::Rax.encoding(), 0);
        assert_eq!(PhysReg::Rcx.encoding(), 1);
        assert_eq!(PhysReg::R8.encoding(), 8);
        assert_eq!(PhysReg::R15.encoding(), 15);

        assert!(!PhysReg::Rax.is_extended());
        assert!(PhysReg::R8.is_extended());
    }

    #[test]
    fn test_reg_alloc_basic() {
        let mut alloc = RegAlloc::new();

        let v0 = VReg::Virtual(VirtualId(0));
        let v1 = VReg::Virtual(VirtualId(1));

        let loc0 = alloc.alloc_vreg(v0).unwrap();
        let loc1 = alloc.alloc_vreg(v1).unwrap();

        assert!(loc0.is_register());
        assert!(loc1.is_register());
        assert_ne!(loc0, loc1);

        // Same vreg should return same location
        let loc0_again = alloc.alloc_vreg(v0).unwrap();
        assert_eq!(loc0, loc0_again);
    }

    #[test]
    fn test_reg_alloc_immediate() {
        let alloc = RegAlloc::new();

        let imm = VReg::Imm(42);
        let loc = alloc.get_vreg_location(imm).unwrap();

        assert_eq!(loc, RegLocation::Constant(42));
    }

    #[test]
    fn test_reg_alloc_spill() {
        let mut alloc = RegAlloc::new();

        // Allocate all available registers
        for i in 0..PhysReg::ALLOCATABLE.len() {
            let vreg = VReg::Virtual(VirtualId(i as u32));
            let loc = alloc.alloc_vreg(vreg).unwrap();
            assert!(loc.is_register());
        }

        // Next allocation should spill to stack
        let vreg = VReg::Virtual(VirtualId(100));
        let loc = alloc.alloc_vreg(vreg).unwrap();
        assert!(loc.is_stack());
    }

    #[test]
    fn test_reg_alloc_temp() {
        let mut alloc = RegAlloc::new();

        let temp = alloc.alloc_temp().unwrap();
        assert!(PhysReg::ALLOCATABLE.contains(&temp));

        alloc.free_temp(temp);

        // Should be able to allocate again
        let temp2 = alloc.alloc_temp().unwrap();
        assert_eq!(temp, temp2);
    }

    #[test]
    fn test_frame_size() {
        let mut alloc = RegAlloc::new();

        // Force some spills
        for i in 0..20 {
            let vreg = VReg::Virtual(VirtualId(i));
            alloc.alloc_vreg(vreg).unwrap();
        }

        let frame_size = alloc.frame_size();
        assert!(frame_size > 0);
        assert_eq!(frame_size % 16, 0); // Must be 16-byte aligned
    }
}
