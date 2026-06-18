//! SMIR lowering - code generation from SMIR to native machine code.
//!
//! This module provides the infrastructure for lowering SMIR IR to native
//! machine code for various target architectures.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐
//! │   SmirFunction  │
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Register Alloc  │  (VReg → PhysReg mapping)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │    Lowering     │  (SMIR Op → Machine Instructions)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │   CodeBuffer    │  (Raw machine code bytes)
//! └─────────────────┘
//! ```

pub mod aarch64;
pub mod aarch64_x86;
pub mod avx10;
pub mod regalloc;
/// Native execution runtime for lowered blocks (the JIT executor). Only present
/// with the `smir-jit` feature on an x86-64 host.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
pub mod runtime;
#[cfg(test)]
mod validation;
pub mod x86_64;

use std::collections::HashMap;

use crate::smir::ir::SmirFunction;
use crate::smir::types::{BlockId, GuestAddr};

/// Number of x86-64 guest GPR slots in the native JIT register file. APX adds
/// R16-R31, which are state-backed because the host has no physical EGPRs.
pub const X86_GUEST_GPR_COUNT: usize = 32;
/// Byte offset of `GuestRegs.rflags`.
pub const X86_GUEST_RFLAGS_OFFSET: i32 = (X86_GUEST_GPR_COUNT as i32) * 8;
/// Byte offset of `GuestRegs.exit_pc`.
pub const X86_GUEST_EXIT_PC_OFFSET: i32 = X86_GUEST_RFLAGS_OFFSET + 8;
/// Byte offset of `GuestRegs.ctx`.
pub const X86_GUEST_CTX_OFFSET: i32 = X86_GUEST_EXIT_PC_OFFSET + 8;
/// Byte offset of `GuestRegs.load_fn`.
pub const X86_GUEST_LOAD_FN_OFFSET: i32 = X86_GUEST_CTX_OFFSET + 8;
/// Byte offset of `GuestRegs.store_fn`.
pub const X86_GUEST_STORE_FN_OFFSET: i32 = X86_GUEST_LOAD_FN_OFFSET + 8;
/// Byte offset of `GuestRegs.fs_base`.
pub const X86_GUEST_FS_BASE_OFFSET: i32 = X86_GUEST_STORE_FN_OFFSET + 8;
/// Byte offset of `GuestRegs.gs_base`.
pub const X86_GUEST_GS_BASE_OFFSET: i32 = X86_GUEST_FS_BASE_OFFSET + 8;
/// Byte offset of `GuestRegs.call_fn`.
pub const X86_GUEST_CALL_FN_OFFSET: i32 = X86_GUEST_GS_BASE_OFFSET + 8;
/// Offset of the `*mut GuestRegs` state pointer in the native block frame.
pub const X86_STATE_PTR_AT_RBP: i32 = 24;

// ============================================================================
// Lowerer Trait
// ============================================================================

/// Trait for lowering SMIR to native machine code
pub trait SmirLowerer: Send {
    /// Target architecture name
    fn target_arch(&self) -> &'static str;

    /// Lower an entire function to machine code
    fn lower_function(&mut self, func: &SmirFunction) -> Result<LowerResult, LowerError>;

    /// Get the generated code buffer
    fn code_buffer(&self) -> &CodeBuffer;

    /// Finalize and get the executable code
    fn finalize(&mut self) -> Result<Vec<u8>, LowerError>;
}

// ============================================================================
// Lower Result
// ============================================================================

/// Result of lowering a function
#[derive(Clone, Debug)]
pub struct LowerResult {
    /// Size of generated code in bytes
    pub code_size: usize,

    /// Entry point offset within the code buffer
    pub entry_offset: usize,

    /// Block offsets (BlockId -> offset in code)
    pub block_offsets: HashMap<BlockId, usize>,

    /// Relocations that need to be applied
    pub relocations: Vec<Relocation>,

    /// Stack frame size required
    pub stack_size: usize,
}

// ============================================================================
// Relocation
// ============================================================================

/// A relocation that needs to be applied after code generation
#[derive(Clone, Debug)]
pub struct Relocation {
    /// Offset in the code buffer where the relocation applies
    pub offset: usize,

    /// Kind of relocation
    pub kind: RelocKind,

    /// Target of the relocation
    pub target: RelocTarget,
}

/// Relocation kind
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelocKind {
    /// PC-relative 8-bit displacement (for short jumps)
    PcRel8,

    /// PC-relative 32-bit displacement (for jumps and calls)
    PcRel32,

    /// Absolute 64-bit address
    Abs64,

    /// Absolute 32-bit address
    Abs32,
}

/// Relocation target
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelocTarget {
    /// Target is a block within the same function
    Block(BlockId),

    /// Target is a guest address (for indirect branches)
    GuestAddr(GuestAddr),

    /// Target is an external symbol
    External(String),

    /// Target is a runtime helper function
    Runtime(RuntimeHelper),
}

/// Runtime helper functions that lowered code may call
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeHelper {
    /// Memory read helper
    MemRead8,
    MemRead16,
    MemRead32,
    MemRead64,

    /// Memory write helper
    MemWrite8,
    MemWrite16,
    MemWrite32,
    MemWrite64,

    /// Syscall handler
    Syscall,

    /// Exception handler
    Exception,

    /// Division by zero handler
    DivByZero,

    /// Debug breakpoint handler
    Breakpoint,

    /// Lookup jump target for indirect branches
    LookupTarget,
}

// ============================================================================
// Code Buffer
// ============================================================================

/// Buffer for emitting machine code
#[derive(Clone, Debug, Default)]
pub struct CodeBuffer {
    /// The raw code bytes
    data: Vec<u8>,

    /// Current write position
    pos: usize,

    /// Labels (name -> offset)
    labels: HashMap<String, usize>,

    /// Pending fixups (offset -> label name)
    fixups: Vec<(usize, String, RelocKind)>,
}

impl CodeBuffer {
    /// Create a new code buffer
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a code buffer with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        CodeBuffer {
            data: Vec::with_capacity(capacity),
            pos: 0,
            labels: HashMap::new(),
            fixups: Vec::new(),
        }
    }

    /// Current position in the buffer
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Total length of emitted code
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the raw code bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get the raw code bytes as a slice (alias for data())
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Consume and return the code bytes
    pub fn into_data(self) -> Vec<u8> {
        self.data
    }

    /// Emit a single byte
    pub fn emit_u8(&mut self, byte: u8) {
        if self.pos >= self.data.len() {
            self.data.push(byte);
        } else {
            self.data[self.pos] = byte;
        }
        self.pos += 1;
    }

    /// Emit a 16-bit value (little-endian)
    pub fn emit_u16(&mut self, value: u16) {
        self.emit_u8(value as u8);
        self.emit_u8((value >> 8) as u8);
    }

    /// Emit a 32-bit value (little-endian)
    pub fn emit_u32(&mut self, value: u32) {
        self.emit_u8(value as u8);
        self.emit_u8((value >> 8) as u8);
        self.emit_u8((value >> 16) as u8);
        self.emit_u8((value >> 24) as u8);
    }

    /// Emit a 64-bit value (little-endian)
    pub fn emit_u64(&mut self, value: u64) {
        self.emit_u32(value as u32);
        self.emit_u32((value >> 32) as u32);
    }

    /// Emit a signed 8-bit value
    pub fn emit_i8(&mut self, value: i8) {
        self.emit_u8(value as u8);
    }

    /// Emit a signed 32-bit value
    pub fn emit_i32(&mut self, value: i32) {
        self.emit_u32(value as u32);
    }

    /// Emit raw bytes
    pub fn emit_bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.emit_u8(b);
        }
    }

    /// Define a label at the current position
    pub fn define_label(&mut self, name: impl Into<String>) {
        self.labels.insert(name.into(), self.pos);
    }

    /// Get label offset
    pub fn label_offset(&self, name: &str) -> Option<usize> {
        self.labels.get(name).copied()
    }

    /// Record a fixup to be applied later
    pub fn record_fixup(&mut self, label: impl Into<String>, kind: RelocKind) {
        self.fixups.push((self.pos, label.into(), kind));
    }

    /// Apply all recorded fixups
    pub fn apply_fixups(&mut self) -> Result<(), LowerError> {
        for (offset, label, kind) in self.fixups.clone() {
            let target = self
                .labels
                .get(&label)
                .ok_or_else(|| LowerError::UndefinedLabel {
                    label: label.clone(),
                })?;

            match kind {
                RelocKind::PcRel8 => {
                    let rel = (*target as i64) - (offset as i64) - 1;
                    if rel < -128 || rel > 127 {
                        return Err(LowerError::RelocationOutOfRange {
                            offset,
                            target: *target,
                        });
                    }
                    self.data[offset] = rel as i8 as u8;
                }
                RelocKind::PcRel32 => {
                    let rel = (*target as i64) - (offset as i64) - 4;
                    if rel < i32::MIN as i64 || rel > i32::MAX as i64 {
                        return Err(LowerError::RelocationOutOfRange {
                            offset,
                            target: *target,
                        });
                    }
                    let bytes = (rel as i32).to_le_bytes();
                    self.data[offset..offset + 4].copy_from_slice(&bytes);
                }
                RelocKind::Abs32 => {
                    let bytes = (*target as u32).to_le_bytes();
                    self.data[offset..offset + 4].copy_from_slice(&bytes);
                }
                RelocKind::Abs64 => {
                    let bytes = (*target as u64).to_le_bytes();
                    self.data[offset..offset + 8].copy_from_slice(&bytes);
                }
            }
        }
        Ok(())
    }

    /// Patch a 32-bit value at a specific offset
    pub fn patch_i32(&mut self, offset: usize, value: i32) {
        let bytes = value.to_le_bytes();
        self.data[offset..offset + 4].copy_from_slice(&bytes);
    }

    /// Patch a 64-bit value at a specific offset
    pub fn patch_u64(&mut self, offset: usize, value: u64) {
        let bytes = value.to_le_bytes();
        self.data[offset..offset + 8].copy_from_slice(&bytes);
    }

    /// Align to boundary (pad with NOPs or zeros)
    pub fn align(&mut self, alignment: usize, pad_byte: u8) {
        while self.pos % alignment != 0 {
            self.emit_u8(pad_byte);
        }
    }

    /// Reset the buffer
    pub fn clear(&mut self) {
        self.data.clear();
        self.pos = 0;
        self.labels.clear();
        self.fixups.clear();
    }
}

// ============================================================================
// Lower Error
// ============================================================================

/// Error during lowering
#[derive(Clone, Debug)]
pub enum LowerError {
    /// Unsupported operation
    UnsupportedOp { op: String },

    /// Unsupported operation (string-only variant)
    UnsupportedOperation(String),

    /// Register allocation failed
    RegisterAllocationFailed { reason: String },

    /// Undefined label
    UndefinedLabel { label: String },

    /// Relocation out of range
    RelocationOutOfRange { offset: usize, target: usize },

    /// Invalid operand
    InvalidOperand { op: String, operand: String },

    /// Invalid register for lowering
    InvalidRegister(String),

    /// Stack overflow (too many spills)
    StackOverflow { required: usize, limit: usize },

    /// Internal error
    Internal(String),
}

impl std::fmt::Display for LowerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LowerError::UnsupportedOp { op } => write!(f, "unsupported operation: {}", op),
            LowerError::UnsupportedOperation(op) => write!(f, "unsupported operation: {}", op),
            LowerError::RegisterAllocationFailed { reason } => {
                write!(f, "register allocation failed: {}", reason)
            }
            LowerError::UndefinedLabel { label } => write!(f, "undefined label: {}", label),
            LowerError::RelocationOutOfRange { offset, target } => {
                write!(f, "relocation out of range: {} -> {}", offset, target)
            }
            LowerError::InvalidOperand { op, operand } => {
                write!(f, "invalid operand for {}: {}", op, operand)
            }
            LowerError::InvalidRegister(reg) => {
                write!(f, "invalid register: {}", reg)
            }
            LowerError::StackOverflow { required, limit } => {
                write!(
                    f,
                    "stack overflow: need {} bytes, limit {}",
                    required, limit
                )
            }
            LowerError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for LowerError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_buffer_emit() {
        let mut buf = CodeBuffer::new();

        buf.emit_u8(0x90); // NOP
        buf.emit_u8(0xC3); // RET

        assert_eq!(buf.len(), 2);
        assert_eq!(buf.data(), &[0x90, 0xC3]);
    }

    #[test]
    fn test_code_buffer_emit_multi() {
        let mut buf = CodeBuffer::new();

        buf.emit_u32(0x12345678);
        assert_eq!(buf.data(), &[0x78, 0x56, 0x34, 0x12]); // Little-endian

        buf.emit_u64(0xDEADBEEFCAFEBABE);
        assert_eq!(buf.len(), 12);
    }

    #[test]
    fn test_code_buffer_labels() {
        let mut buf = CodeBuffer::new();

        buf.emit_u8(0x90); // Position 0
        buf.define_label("target");
        buf.emit_u8(0x90); // Position 1

        assert_eq!(buf.label_offset("target"), Some(1));
    }

    #[test]
    fn test_code_buffer_fixups() {
        let mut buf = CodeBuffer::new();

        // JMP rel32 (placeholder)
        buf.emit_u8(0xE9);
        buf.record_fixup("target", RelocKind::PcRel32);
        buf.emit_u32(0); // Placeholder

        // Some code
        buf.emit_u8(0x90);
        buf.emit_u8(0x90);

        // Target
        buf.define_label("target");
        buf.emit_u8(0xC3);

        buf.apply_fixups().unwrap();

        // Verify the jump offset was patched correctly
        // Target is at offset 7, fixup is at offset 1, so rel = 7 - 1 - 4 = 2
        let rel = i32::from_le_bytes([buf.data()[1], buf.data()[2], buf.data()[3], buf.data()[4]]);
        assert_eq!(rel, 2);
    }

    #[test]
    fn test_code_buffer_align() {
        let mut buf = CodeBuffer::new();

        buf.emit_u8(0x90);
        buf.align(4, 0x00);

        assert_eq!(buf.len(), 4);
    }
}
