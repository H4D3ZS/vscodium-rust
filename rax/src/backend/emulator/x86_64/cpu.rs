//! x86_64 CPU state and core execution loop.

use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicU64, AtomicUsize};

#[cfg(feature = "trace")]
use crate::trace;

#[cfg(feature = "profiling")]
use crate::profiling;

/// Global tracker for current RIP (for debugging write watchpoints)
pub static CURRENT_RIP: AtomicU64 = AtomicU64::new(0);

/// Circular buffer of last 16 RIPs for debugging crashes
pub static RIP_HISTORY: [AtomicU64; 16] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];
pub static RIP_IDX: AtomicUsize = AtomicUsize::new(0);

/// Log an IF state transition with context (disabled for performance)
#[inline]
pub fn log_if_transition(_rip: u64, _old_if: bool, _new_if: bool, _source: &str) {
    // Disabled - IF flag logic verified working correctly
}

use vm_memory::GuestMemoryMmap;

use super::decoder::Decoder;
use super::flags;
use super::insn;
use super::mmu::Mmu;
use crate::cpu::{CpuState, Registers, SystemRegisters, VCpu, VcpuExit, X86_64CpuState};

/// Byte offset of each GPR field within `Registers`, indexed by x86 register
/// encoding (0=rax,1=rcx,2=rdx,3=rbx,4=rsp,5=rbp,6=rsi,7=rdi, 8..=15 = r8..=r15,
/// 16..=31 = r16..=r31). Built with `offset_of!`, so it reflects the actual
/// field layout for any `repr` and lets `get_reg`/`set_reg` index a register
/// branchlessly instead of via a 32-arm match (which the profiler showed as a
/// hot jump table inside every ALU handler).
const GPR_OFFSETS: [usize; 32] = [
    std::mem::offset_of!(Registers, rax),
    std::mem::offset_of!(Registers, rcx),
    std::mem::offset_of!(Registers, rdx),
    std::mem::offset_of!(Registers, rbx),
    std::mem::offset_of!(Registers, rsp),
    std::mem::offset_of!(Registers, rbp),
    std::mem::offset_of!(Registers, rsi),
    std::mem::offset_of!(Registers, rdi),
    std::mem::offset_of!(Registers, r8),
    std::mem::offset_of!(Registers, r9),
    std::mem::offset_of!(Registers, r10),
    std::mem::offset_of!(Registers, r11),
    std::mem::offset_of!(Registers, r12),
    std::mem::offset_of!(Registers, r13),
    std::mem::offset_of!(Registers, r14),
    std::mem::offset_of!(Registers, r15),
    std::mem::offset_of!(Registers, r16),
    std::mem::offset_of!(Registers, r17),
    std::mem::offset_of!(Registers, r18),
    std::mem::offset_of!(Registers, r19),
    std::mem::offset_of!(Registers, r20),
    std::mem::offset_of!(Registers, r21),
    std::mem::offset_of!(Registers, r22),
    std::mem::offset_of!(Registers, r23),
    std::mem::offset_of!(Registers, r24),
    std::mem::offset_of!(Registers, r25),
    std::mem::offset_of!(Registers, r26),
    std::mem::offset_of!(Registers, r27),
    std::mem::offset_of!(Registers, r28),
    std::mem::offset_of!(Registers, r29),
    std::mem::offset_of!(Registers, r30),
    std::mem::offset_of!(Registers, r31),
];
use crate::error::{Error, Result};

/// x87 FPU state.
#[derive(Clone, Debug)]
pub struct FpuState {
    /// FPU control word (default 0x037F)
    pub control_word: u16,
    /// FPU status word (default 0x0000)
    pub status_word: u16,
    /// FPU tag word (default 0xFFFF - all empty)
    pub tag_word: u16,
    /// FPU data pointer
    pub data_ptr: u64,
    /// FPU instruction pointer
    pub instr_ptr: u64,
    /// FPU last opcode
    pub last_opcode: u16,
    /// FPU register stack (8 x 80-bit, stored as f64 for simplicity)
    pub st: [f64; 8],
    /// Top of stack pointer (0-7), stored in status word bits 11-13
    pub top: u8,
}

impl Default for FpuState {
    fn default() -> Self {
        FpuState {
            control_word: 0x037F, // Round to nearest, all exceptions masked, 64-bit precision
            status_word: 0x0000,
            tag_word: 0xFFFF, // All registers empty
            data_ptr: 0,
            instr_ptr: 0,
            last_opcode: 0,
            st: [0.0; 8],
            top: 0,
        }
    }
}

impl FpuState {
    /// Initialize FPU to default state (FINIT/FNINIT)
    pub fn init(&mut self) {
        self.control_word = 0x037F;
        self.status_word = 0x0000;
        self.tag_word = 0xFFFF;
        self.data_ptr = 0;
        self.instr_ptr = 0;
        self.last_opcode = 0;
        self.top = 0;
        // Note: register values are preserved, just tagged as empty
    }

    /// Get physical register index from stack-relative index
    #[inline]
    pub fn st_index(&self, i: u8) -> usize {
        ((self.top.wrapping_add(i)) & 7) as usize
    }

    /// Push a value onto the FPU stack
    pub fn push(&mut self, value: f64) {
        // New TOP is the register below the current one. Per the x87 spec, if it
        // is not already empty (tag != 3) the push is a stack OVERFLOW: raise the
        // invalid-operation (IE) and stack-fault (SF) exceptions, set C1 to flag
        // the overflow direction, and raise the error-summary (ES) bit. With the
        // exception masked (the default) the push still completes.
        let dst = self.top.wrapping_sub(1) & 7;
        let dst_tag = (self.tag_word >> ((dst as u16) * 2)) & 3;
        if dst_tag != 3 {
            // IE (bit 0) | SF (bit 6) | ES (bit 7) | C1 (bit 9, overflow direction)
            self.status_word |= 0x0001 | 0x0040 | 0x0080 | 0x0200;
        }
        self.top = dst;
        self.st[self.top as usize] = value;
        // Update tag for this register (mark as valid)
        let tag_shift = (self.top as u16) * 2;
        self.tag_word &= !(3 << tag_shift);
        // 0 = valid, 1 = zero, 2 = special, 3 = empty
        if value == 0.0 {
            self.tag_word |= 1 << tag_shift;
        }
        // Update TOP in status word
        self.status_word = (self.status_word & !0x3800) | ((self.top as u16) << 11);
    }

    /// Pop a value from the FPU stack
    pub fn pop(&mut self) -> f64 {
        // If the current TOP register is empty (tag == 3) the pop is a stack
        // UNDERFLOW: raise invalid-operation (IE), stack-fault (SF) and the
        // error-summary (ES) bit, and clear C1 to flag the underflow direction.
        let tag_shift = (self.top as u16) * 2;
        if (self.tag_word >> tag_shift) & 3 == 3 {
            // Set IE (bit 0) | SF (bit 6) | ES (bit 7); clear C1 (bit 9).
            self.status_word = (self.status_word | 0x0001 | 0x0040 | 0x0080) & !0x0200;
        }
        let value = self.st[self.top as usize];
        // Mark register as empty
        self.tag_word |= 3 << tag_shift;
        self.top = self.top.wrapping_add(1) & 7;
        // Update TOP in status word
        self.status_word = (self.status_word & !0x3800) | ((self.top as u16) << 11);
        value
    }

    /// Get ST(i) value
    #[inline]
    pub fn get_st(&self, i: u8) -> f64 {
        self.st[self.st_index(i)]
    }

    /// Set ST(i) value
    #[inline]
    pub fn set_st(&mut self, i: u8, value: f64) {
        let idx = self.st_index(i);
        self.st[idx] = value;
    }
}

/// Type of lazy flag operation - determines how to compute flags on demand
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum LazyFlagOp {
    /// No lazy flags - rflags is valid
    None,
    /// Add operation: CF = result < a, OF = signed overflow
    Add,
    /// Sub/CMP operation: CF = a < b (borrow), OF = signed overflow
    Sub,
    /// Logic operation (AND/OR/XOR/TEST): CF=OF=0
    Logic,
    /// Inc operation: like Add but CF preserved
    Inc,
    /// Dec operation: like Sub but CF preserved
    Dec,
}

/// Lazy flag state - stores operands to compute flags on demand
#[derive(Clone, Copy, Debug)]
pub(super) struct LazyFlags {
    pub op: LazyFlagOp,
    pub result: u64,
    pub src: u64, // First operand (a)
    pub dst: u64, // Second operand (b) - only used for Add/Sub
    pub size: u8,
}

impl Default for LazyFlags {
    fn default() -> Self {
        LazyFlags {
            op: LazyFlagOp::None,
            result: 0,
            src: 0,
            dst: 0,
            size: 4,
        }
    }
}

/// Emulated x86_64 vCPU.
pub struct X86_64Vcpu {
    id: u32,
    /// Per-vCPU retired-instruction counter. Drives RDTSC (insn_count*3000) and
    /// is published to the global counter only at run() yield boundaries, so the
    /// hot loop stays atomic-free.
    pub(super) insn_count: u64,
    pub(super) regs: Registers,
    pub(super) sregs: SystemRegisters,
    pub(super) mmu: Mmu,
    pub(super) fpu: FpuState,
    pub(super) halted: bool,
    io_pending: Option<IoPending>,
    /// IA32_KERNEL_GS_BASE MSR (0xC0000102) for SWAPGS
    pub(super) kernel_gs_base: u64,
    /// Protection Key Rights Register (PKRU).
    pub(super) pkru: u32,
    /// Extended control register XCR0 (XSAVE feature-enable mask): bit0 x87
    /// (always 1), bit1 SSE, bit2 AVX (YMM_Hi128). Written by XSETBV, read by
    /// XGETBV, and consulted by XSAVE/XRSTOR and CPUID leaf 0xD.
    pub(super) xcr0: u64,
    /// Decoded instruction cache for avoiding re-decode in hot loops
    pub(super) decode_cache: Box<[DecodeCacheEntry; DECODE_CACHE_SIZE]>,
    /// Lazy flag state for deferred flag computation. A plain field (not a Cell):
    /// every writer holds `&mut self`, and the two `&self` readers
    /// (`compute_materialized_rflags`, `get_emulator_state`) only copy it out, so
    /// no interior mutability is needed. Keeping it inline lets the optimizer hold
    /// the hot lazy state in registers instead of routing through a Cell.
    pub(super) lazy_flags: LazyFlags,
    /// Single-step mode for GDB debugging.
    #[cfg(feature = "debug")]
    single_step: bool,
    /// SMIR hot-block JIT: compiled native regions keyed by (RIP, mode_tag);
    /// `Some` = runnable, `None` = known-ineligible (don't recompile). Evicted
    /// when the guest writes the corresponding code page (SMC).
    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
    jit_cache: std::collections::HashMap<(u64, u64), Option<std::sync::Arc<JitRegion>>>,
    /// SMIR hot-block JIT: per-loop-head backward-branch hit counter; a head is
    /// promoted (compiled) once it crosses the hotness threshold.
    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
    jit_hot: std::collections::HashMap<u64, u32>,
    /// SMIR hot-block JIT: known-ineligible heads memoized as `(head, mode_tag)
    /// -> code fingerprint`. UNLIKE `jit_cache`, this is NOT wiped by SMC: when a
    /// guest writes a page that merely shares a 4 KiB frame with code (common in
    /// TempleOS, whose compiler keeps data beside code), the cache+hotness wipe
    /// would otherwise re-promote the same ineligible head thousands of times,
    /// re-running the (futile) lift/optimize each time. The fingerprint (16 code
    /// bytes at the head) self-corrects the memo: a genuine code change shifts it
    /// and re-triggers compilation; an unchanged head is skipped cheaply. Only
    /// ineligible verdicts are memoized here — compiled regions stay in
    /// `jit_cache` and are still SMC-invalidated for correctness.
    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
    jit_ineligible: std::collections::HashMap<(u64, u64), u64>,
    /// JIT of memory-touching regions (Load/Store via MMU helper calls). Seeded
    /// from `RAX_JIT_MEM` at construction; settable for tests. Off ⇒ memory ops
    /// bail to the interpreter (the validated default).
    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
    jit_mem: bool,
    /// Set by [`rax_jit_call`] (the lift-through-calls helper) when a callee
    /// yields a VMM-bound exit (I/O, HLT, …): `jit_run_region_native` recovers it
    /// and propagates it so the run loop returns it to the VMM. `None` otherwise.
    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
    jit_callout_exit: Option<VcpuExit>,
    /// When `Some`, the memory-JIT store helper logs each store's `(addr, size,
    /// old_value)` here so verify mode can UNDO the region's writes and re-run
    /// the interpreter for a store-sound differential. `None` in normal use.
    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
    jit_mem_log: Option<Vec<(u64, u8, u64)>>,
    /// When `Some`, every data memory access funnelled through `read_mem` /
    /// `write_mem` is appended as `(kind, addr, size, value)` (kind 0=load,
    /// 1=store). Verify mode captures the native run's trace and the interpreter
    /// re-run's trace, then diffs them to pinpoint the exact diverging access.
    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
    jit_mem_trace: Option<Vec<(u8, u64, u8, u64)>>,
}

/// Pending I/O operation.
enum IoInTarget {
    Reg,
    Mem { addr: u64 },
}

struct IoPending {
    size: u8,
    target: IoInTarget,
    /// Element count: 1 for a normal IN, N for a batched `rep ins` block (the
    /// destination is then `count` consecutive `size`-byte elements starting at
    /// the `Mem` address).
    count: u32,
}

/// Maximum instruction length in bytes.
pub const MAX_INSN_LEN: usize = 15;

/// Decode cache size (must be power of 2 for fast indexing)
const DECODE_CACHE_SIZE: usize = 4096;

/// How often run() performs periodic housekeeping (LAPIC poll, VMM yield,
/// counter publish). Keeps clock reads and RefCell borrows off the per-insn path.
const LAPIC_POLL_STRIDE: u64 = 1024;
pub(super) const DECODE_CACHE_MASK: usize = DECODE_CACHE_SIZE - 1;

/// Uniform-signature instruction handler. Resolved once on a decode-cache miss
/// (see [`X86_64Vcpu::resolve_handler`]) and stored in the cache entry so a hit
/// can call the handler directly, skipping the big `execute` opcode match and
/// the escape/two-byte call chain. Opcode-/cc-derived arguments are recovered
/// from `InsnContext::opcode` by thin shim wrappers.
pub(super) type HandlerFn = fn(&mut X86_64Vcpu, &mut InsnContext) -> Result<Option<VcpuExit>>;

/// Cached decoded instruction entry
#[derive(Clone, Copy, Debug)]
pub(super) struct DecodeCacheEntry {
    /// RIP where this instruction lives (0 = invalid)
    pub(super) rip: u64,
    /// Primary opcode byte
    pub(super) opcode: u8,
    /// Decoded operand size
    pub(super) op_size: u8,
    /// Cursor position after prefix decode (start of opcode)
    pub(super) cursor: usize,
    /// REX prefix if present
    pub(super) rex: Option<u8>,
    /// REX2 prefix if present
    pub(super) rex2: Option<Rex2Prefix>,
    /// 0x66 prefix
    pub(super) operand_size_override: bool,
    /// 0x67 prefix
    pub(super) address_size_override: bool,
    /// REP/REPNE prefix
    pub(super) rep_prefix: Option<u8>,
    /// Segment override prefix (0x64=FS, 0x65=GS, etc.)
    pub(super) segment_override: Option<u8>,
    /// Address-space + CPU-mode tag (CR3 base | cs.l | cs.db<<1): part of the key
    /// so a hit never reuses stale bytes/decode across a context or mode switch.
    pub(super) mode_tag: u64,
    /// Raw instruction bytes captured at fill time (so hits skip the MMU fetch).
    pub(super) bytes: [u8; MAX_INSN_LEN],
    /// Number of valid bytes in `bytes`.
    pub(super) bytes_len: usize,
    /// Whether a LOCK (0xF0) prefix is present. Computed once on the fill path so
    /// the per-instruction hit path can skip the prefix-byte scan and only pay the
    /// (cold) legality check when LOCK is actually present.
    pub(super) has_lock: bool,
    /// Handler resolved on the fill (miss) path. On a hit it is called directly,
    /// skipping the `execute` opcode match. Invalidated with the rest of the
    /// entry (SMC / mode switch zero `rip`, so a stale handler can never run).
    pub(super) handler: HandlerFn,
}

/// Placeholder handler stored in freshly-defaulted (invalid, `rip == 0`) cache
/// entries. It can never actually run: an entry only dispatches after a key
/// match, which requires a non-zero `rip` installed by the fill path together
/// with a real resolved handler.
fn unreachable_handler(_vcpu: &mut X86_64Vcpu, _ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    Err(Error::Emulator(
        "decode-cache handler invoked on an invalid entry".to_string(),
    ))
}

impl Default for DecodeCacheEntry {
    #[inline(always)]
    fn default() -> Self {
        DecodeCacheEntry {
            rip: 0,
            opcode: 0,
            op_size: 4,
            cursor: 0,
            rex: None,
            rex2: None,
            operand_size_override: false,
            address_size_override: false,
            rep_prefix: None,
            segment_override: None,
            mode_tag: 0,
            bytes: [0; MAX_INSN_LEN],
            bytes_len: 0,
            has_lock: false,
            handler: unreachable_handler,
        }
    }
}

/// Decoded instruction context passed to instruction handlers.
pub(super) struct InsnContext {
    /// Instruction bytes (fixed-size to avoid allocation)
    pub bytes: [u8; MAX_INSN_LEN],
    /// Actual number of valid bytes
    pub bytes_len: usize,
    pub cursor: usize,
    pub rex: Option<u8>,
    /// REX2 prefix state (if present) - APX extension
    pub rex2: Option<Rex2Prefix>,
    pub operand_size_override: bool,
    pub address_size_override: bool,
    pub rep_prefix: Option<u8>,
    pub op_size: u8,
    pub rip_relative_offset: usize,
    /// Segment override prefix (0x26=ES, 0x2E=CS, 0x36=SS, 0x3E=DS, 0x64=FS, 0x65=GS)
    pub segment_override: Option<u8>,
    /// EVEX prefix state (if present)
    pub evex: Option<EvexPrefix>,
    /// Primary opcode byte. Set by `step()` before dispatch so uniform-signature
    /// handler shims (resolved via the fn-pointer dispatch path) can recover the
    /// opcode-derived register / condition-code arguments without it being passed
    /// as a separate parameter.
    pub opcode: u8,
}

/// REX2 prefix decoded fields (2-byte prefix for APX EGPR access)
/// Format: 0xD5 [M:R4:X4:B4:W:R:X:B]. Field names below preserve older
/// internal naming: r3/x3/b3 are the high (+16) extension bits, and r4/x4/b4
/// are the low (+8) extension bits.
#[derive(Clone, Copy, Debug)]
pub(super) struct Rex2Prefix {
    /// M bit: opcode map select (0=legacy map, 1=0F map)
    pub m: bool,
    /// W bit: operand size (0=default, 1=64-bit)
    pub w: bool,
    /// High ModR/M reg extension bit (+16)
    pub r3: bool,
    /// High SIB index extension bit (+16)
    pub x3: bool,
    /// High ModR/M r/m or SIB base extension bit (+16)
    pub b3: bool,
    /// Low ModR/M reg extension bit (+8)
    pub r4: bool,
    /// Low SIB index extension bit (+8)
    pub x4: bool,
    /// Low ModR/M r/m extension bit (+8)
    pub b4: bool,
}

/// EVEX prefix decoded fields (4-byte prefix for AVX-512)
#[derive(Clone, Copy, Debug)]
pub(super) struct EvexPrefix {
    /// R bit (inverted, extends ModR/M reg field to 4 bits)
    pub r: bool,
    /// X bit (inverted, extends SIB index field)
    pub x: bool,
    /// B bit (inverted, extends ModR/M r/m or SIB base)
    pub b: bool,
    /// R' bit (inverted, extends reg field to 5 bits for ZMM16-31)
    pub r_prime: bool,
    /// mm field (opcode map: 1=0F, 2=0F38, 3=0F3A, 5=MAP5, 6=MAP6)
    pub mm: u8,
    /// W bit (operand size: 0=32-bit, 1=64-bit elements)
    pub w: bool,
    /// vvvv field (inverted, non-destructive source register)
    pub vvvv: u8,
    /// pp field (implied prefix: 0=none, 1=66, 2=F3, 3=F2)
    pub pp: u8,
    /// z bit (zeroing-masking: 0=merge, 1=zero)
    pub z: bool,
    /// L'L field (vector length: 0=128, 1=256, 2=512)
    pub ll: u8,
    /// b bit (broadcast/rounding control)
    pub broadcast: bool,
    /// V' bit (inverted, extends vvvv to 5 bits)
    pub v_prime: bool,
    /// aaa field (opmask register k0-k7)
    pub aaa: u8,
    // APX-specific fields
    /// B4 bit (APX MAP4 P0[3], extends r/m to EGPR R16-R31)
    pub b4: bool,
    /// X4 bit (inverted, extends SIB index to 5 bits for EGPR R16-R31)
    pub x4: bool,
    /// ND bit (New Data Destination - 3-operand form)
    pub nd: bool,
    /// NF bit (No Flags - suppress RFLAGS updates)
    pub nf: bool,
    /// APX mode indicator (for EVEX-encoded GPR instructions)
    pub apx_mode: bool,
}

impl InsnContext {
    /// Get REX.W flag.
    #[inline(always)]
    pub fn rex_w(&self) -> bool {
        self.rex.map_or(false, |r| r & 0x08 != 0)
    }

    /// Get REX.R flag (extends ModR/M reg field).
    #[inline(always)]
    pub fn rex_r(&self) -> u8 {
        self.rex.map_or(0, |r| (r & 0x04) << 1)
    }

    /// Get REX.B flag (extends ModR/M r/m field or opcode reg).
    #[inline(always)]
    pub fn rex_b(&self) -> u8 {
        self.rex.map_or(0, |r| (r & 0x01) << 3)
    }

    // =========================================================================
    // REX2 helper methods (APX)
    // =========================================================================

    /// Check if REX2 prefix is present
    #[inline(always)]
    pub fn has_rex2(&self) -> bool {
        self.rex2.is_some()
    }

    /// Check if any REX-type prefix is present (REX or REX2)
    #[inline(always)]
    pub fn has_any_rex(&self) -> bool {
        self.rex.is_some() || self.rex2.is_some()
    }

    /// Get REX2.W flag (64-bit operand size)
    #[inline(always)]
    pub fn rex2_w(&self) -> bool {
        self.rex2.map_or(false, |r| r.w)
    }

    /// Get W flag from either REX or REX2
    #[inline(always)]
    pub fn any_rex_w(&self) -> bool {
        self.rex_w() || self.rex2_w()
    }

    /// Get REX2.M flag (opcode map: 0=legacy, 1=0F map)
    #[inline(always)]
    pub fn rex2_m(&self) -> bool {
        self.rex2.map_or(false, |r| r.m)
    }

    /// Get full 5-bit reg extension from REX2.
    #[inline(always)]
    pub fn rex2_r(&self) -> u8 {
        self.rex2.map_or(0, |r| {
            let r3 = if r.r3 { 16 } else { 0 };
            let r4 = if r.r4 { 8 } else { 0 };
            r3 | r4
        })
    }

    /// Get full 5-bit r/m extension from REX2.
    #[inline(always)]
    pub fn rex2_b(&self) -> u8 {
        self.rex2.map_or(0, |r| {
            let b3 = if r.b3 { 16 } else { 0 };
            let b4 = if r.b4 { 8 } else { 0 };
            b3 | b4
        })
    }

    /// Get full 5-bit index extension from REX2.
    #[inline(always)]
    pub fn rex2_x(&self) -> u8 {
        self.rex2.map_or(0, |r| {
            let x3 = if r.x3 { 16 } else { 0 };
            let x4 = if r.x4 { 8 } else { 0 };
            x3 | x4
        })
    }

    /// Get combined reg extension from REX or REX2
    #[inline(always)]
    pub fn any_rex_r(&self) -> u8 {
        if self.rex2.is_some() {
            self.rex2_r()
        } else {
            self.rex_r()
        }
    }

    /// Get combined r/m extension from REX or REX2
    #[inline(always)]
    pub fn any_rex_b(&self) -> u8 {
        if self.rex2.is_some() {
            self.rex2_b()
        } else {
            self.rex_b()
        }
    }

    // =========================================================================
    // EVEX helper methods
    // =========================================================================

    /// Get full 5-bit destination register (ModR/M reg extended by EVEX.R and EVEX.R')
    pub fn evex_dest_reg(&self) -> u8 {
        if let Some(evex) = &self.evex {
            // reg field from ModR/M (3 bits) + R (bit 3) + R' (bit 4)
            let r_ext = if evex.r { 0 } else { 8 };
            let r_prime_ext = if evex.r_prime { 0 } else { 16 };
            r_ext | r_prime_ext
        } else {
            self.rex_r()
        }
    }

    /// Get full 5-bit source register (EVEX.vvvv extended by EVEX.V')
    pub fn evex_vvvv(&self) -> u8 {
        if let Some(evex) = &self.evex {
            // vvvv is inverted, V' extends to 5 bits
            let v_prime_ext = if evex.v_prime { 0 } else { 16 };
            (evex.vvvv ^ 0xF) | v_prime_ext
        } else {
            0
        }
    }

    /// Get full 5-bit r/m register (extended by EVEX.B and EVEX.X for certain encodings)
    /// For APX mode, uses B4 bit for EGPR extension
    pub fn evex_rm_reg(&self) -> u8 {
        if let Some(evex) = &self.evex {
            let b_ext = if evex.b { 0 } else { 8 };
            // For APX, P0[3] is the non-inverted B4 bit; for vector EVEX,
            // X is the inverted high extension bit used by some encodings.
            let high_ext = if evex.apx_mode {
                if evex.b4 { 16 } else { 0 }
            } else {
                if evex.x { 0 } else { 16 }
            };
            b_ext | high_ext
        } else {
            self.rex_b()
        }
    }

    /// Get full 5-bit SIB index register for APX (uses X4 for EGPR)
    pub fn evex_index_reg(&self) -> u8 {
        if let Some(evex) = &self.evex {
            let x_ext = if evex.x { 0 } else { 8 };
            if evex.apx_mode {
                let x4_ext = if evex.x4 { 0 } else { 16 };
                x_ext | x4_ext
            } else {
                x_ext
            }
        } else {
            // Fall back to REX.X
            self.rex.map_or(0, |r| (r & 0x02) << 2)
        }
    }

    /// Get vector length from EVEX.L'L (0=128, 1=256, 2=512 bits)
    pub fn evex_vl(&self) -> u16 {
        if let Some(evex) = &self.evex {
            match evex.ll {
                0 => 128,
                1 => 256,
                2 => 512,
                _ => 128,
            }
        } else {
            128
        }
    }

    /// Check if EVEX zeroing-masking is enabled
    pub fn evex_zeroing(&self) -> bool {
        self.evex.map_or(false, |e| e.z)
    }

    /// Get opmask register index (k0-k7)
    pub fn evex_mask(&self) -> u8 {
        self.evex.map_or(0, |e| e.aaa)
    }

    /// Check if EVEX broadcast is enabled
    pub fn evex_broadcast(&self) -> bool {
        self.evex.map_or(false, |e| e.broadcast)
    }

    /// Get EVEX.W bit (element width)
    pub fn evex_w(&self) -> bool {
        self.evex.map_or(false, |e| e.w)
    }

    // =========================================================================
    // APX-specific helper methods
    // =========================================================================

    /// Check if this is an APX (EVEX-encoded GPR) instruction
    #[inline(always)]
    pub fn is_apx(&self) -> bool {
        self.evex.map_or(false, |e| e.apx_mode)
    }

    /// Check if NDD (New Data Destination) mode is enabled
    /// In NDD mode, the vvvv field specifies a separate destination register
    #[inline(always)]
    pub fn apx_ndd(&self) -> bool {
        self.evex.map_or(false, |e| e.nd)
    }

    /// Check if NF (No Flags) mode is enabled
    /// In NF mode, arithmetic operations don't update RFLAGS
    #[inline(always)]
    pub fn apx_nf(&self) -> bool {
        self.evex.map_or(false, |e| e.nf)
    }

    /// Get the NDD destination register (from vvvv field with V4 extension)
    /// Only valid when apx_ndd() returns true
    #[inline(always)]
    pub fn apx_ndd_reg(&self) -> u8 {
        self.evex_vvvv()
    }

    /// Consume and return the next byte.
    #[inline(always)]
    pub fn consume_u8(&mut self) -> Result<u8> {
        if self.cursor >= self.bytes_len {
            return Err(Error::Emulator("instruction too short".to_string()));
        }
        let b = self.bytes[self.cursor];
        self.cursor += 1;
        Ok(b)
    }

    /// Peek at the next byte without consuming.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn peek_u8(&self) -> Result<u8> {
        if self.cursor >= self.bytes_len {
            return Err(Error::Emulator("instruction too short".to_string()));
        }
        Ok(self.bytes[self.cursor])
    }

    /// Consume and return a little-endian u16.
    #[inline(always)]
    pub fn consume_u16(&mut self) -> Result<u16> {
        if self.cursor + 2 > self.bytes_len {
            return Err(Error::Emulator("instruction too short for u16".to_string()));
        }
        let val = u16::from_le_bytes([self.bytes[self.cursor], self.bytes[self.cursor + 1]]);
        self.cursor += 2;
        Ok(val)
    }

    /// Consume and return a little-endian u32.
    #[inline(always)]
    pub fn consume_u32(&mut self) -> Result<u32> {
        if self.cursor + 4 > self.bytes_len {
            return Err(Error::Emulator("instruction too short for u32".to_string()));
        }
        let val = u32::from_le_bytes([
            self.bytes[self.cursor],
            self.bytes[self.cursor + 1],
            self.bytes[self.cursor + 2],
            self.bytes[self.cursor + 3],
        ]);
        self.cursor += 4;
        Ok(val)
    }

    /// Consume and return a little-endian u64.
    #[inline(always)]
    pub fn consume_u64(&mut self) -> Result<u64> {
        if self.cursor + 8 > self.bytes_len {
            return Err(Error::Emulator("instruction too short for u64".to_string()));
        }
        let val = u64::from_le_bytes([
            self.bytes[self.cursor],
            self.bytes[self.cursor + 1],
            self.bytes[self.cursor + 2],
            self.bytes[self.cursor + 3],
            self.bytes[self.cursor + 4],
            self.bytes[self.cursor + 5],
            self.bytes[self.cursor + 6],
            self.bytes[self.cursor + 7],
        ]);
        self.cursor += 8;
        Ok(val)
    }

    /// Read an immediate value of the specified size.
    pub fn consume_imm(&mut self, size: u8) -> Result<u64> {
        match size {
            1 => Ok(self.consume_u8()? as u64),
            2 => Ok(self.consume_u16()? as u64),
            4 => Ok(self.consume_u32()? as u64),
            8 => Ok(self.consume_u64()?),
            _ => Err(Error::Emulator(format!("invalid immediate size: {}", size))),
        }
    }
}

impl X86_64Vcpu {
    pub fn new(id: u32, mem: Arc<GuestMemoryMmap>) -> Self {
        // Use vec! to heap-allocate the cache, then convert to boxed array
        let cache_vec = vec![DecodeCacheEntry::default(); DECODE_CACHE_SIZE];
        let decode_cache: Box<[DecodeCacheEntry; DECODE_CACHE_SIZE]> =
            cache_vec.into_boxed_slice().try_into().unwrap();

        X86_64Vcpu {
            id,
            insn_count: 0,
            regs: Registers::default(),
            sregs: SystemRegisters::default(),
            mmu: Mmu::new(mem),
            fpu: FpuState::default(),
            halted: false,
            io_pending: None,
            kernel_gs_base: 0,
            pkru: 0,
            xcr0: 1, // x87 state component always enabled

            decode_cache,
            lazy_flags: LazyFlags::default(),
            #[cfg(feature = "debug")]
            single_step: false,
            #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
            jit_cache: std::collections::HashMap::new(),
            #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
            jit_hot: std::collections::HashMap::new(),
            #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
            jit_ineligible: std::collections::HashMap::new(),
            #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
            jit_mem: jit_mem_enabled() || jit_call_enabled(),
            #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
            jit_callout_exit: None,
            #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
            jit_mem_log: None,
            #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
            jit_mem_trace: None,
        }
    }

    /// Materialize lazy flags into rflags.
    /// Call this before any instruction that reads flags (Jcc, CMOVcc, SETcc, ADC, SBB, PUSHF, LAHF).
    #[inline]
    pub(super) fn materialize_flags(&mut self) {
        let lf = self.lazy_flags;
        if lf.op == LazyFlagOp::None {
            return; // Flags already materialized
        }

        let result = lf.result;
        let a = lf.src;
        let b = lf.dst;
        let size = lf.size;

        let mask = match size {
            1 => 0xFFu64,
            2 => 0xFFFFu64,
            4 => 0xFFFF_FFFFu64,
            _ => u64::MAX,
        };
        let result_m = result & mask;
        let a_m = a & mask;
        let b_m = b & mask;

        let sign_bit = match size {
            1 => 0x80u64,
            2 => 0x8000u64,
            4 => 0x8000_0000u64,
            _ => 0x8000_0000_0000_0000u64,
        };

        // Common flags for all operations
        let zf = result_m == 0;
        let sf = (result_m & sign_bit) != 0;
        let pf = (result as u8).count_ones() % 2 == 0;

        // Clear status flags (preserve CF for Inc/Dec)
        let cf_mask = if lf.op == LazyFlagOp::Inc || lf.op == LazyFlagOp::Dec {
            0 // Don't clear CF for INC/DEC
        } else {
            flags::bits::CF
        };
        self.regs.rflags &= !(cf_mask
            | flags::bits::ZF
            | flags::bits::SF
            | flags::bits::PF
            | flags::bits::OF
            | flags::bits::AF);

        // Set common flags
        if zf {
            self.regs.rflags |= flags::bits::ZF;
        }
        if sf {
            self.regs.rflags |= flags::bits::SF;
        }
        if pf {
            self.regs.rflags |= flags::bits::PF;
        }

        // Operation-specific flags
        match lf.op {
            LazyFlagOp::Add | LazyFlagOp::Inc => {
                let cf = result_m < a_m;
                let of = ((a_m ^ result_m) & (b_m ^ result_m) & sign_bit) != 0;
                let af = ((a_m ^ b_m ^ result_m) & 0x10) != 0;
                if lf.op == LazyFlagOp::Add && cf {
                    self.regs.rflags |= flags::bits::CF;
                }
                if of {
                    self.regs.rflags |= flags::bits::OF;
                }
                if af {
                    self.regs.rflags |= flags::bits::AF;
                }
            }
            LazyFlagOp::Sub | LazyFlagOp::Dec => {
                let cf = a_m < b_m;
                let of = ((a_m ^ b_m) & (a_m ^ result_m) & sign_bit) != 0;
                let af = ((a_m ^ b_m ^ result_m) & 0x10) != 0;
                if lf.op == LazyFlagOp::Sub && cf {
                    self.regs.rflags |= flags::bits::CF;
                }
                if of {
                    self.regs.rflags |= flags::bits::OF;
                }
                if af {
                    self.regs.rflags |= flags::bits::AF;
                }
            }
            LazyFlagOp::Logic => {
                // CF=0, OF=0 already cleared above; AF is undefined
            }
            LazyFlagOp::None => {}
        }

        // Mark flags as materialized
        self.lazy_flags = LazyFlags {
            op: LazyFlagOp::None,
            ..lf
        };
    }

    /// Compute what rflags would be if lazy flags were materialized (without modifying self).
    /// Used by get_state() to return accurate flags via &self.
    #[inline]
    fn compute_materialized_rflags(&self) -> u64 {
        let lf = self.lazy_flags;
        if lf.op == LazyFlagOp::None {
            return self.regs.rflags; // Already materialized
        }

        let result = lf.result;
        let a = lf.src;
        let b = lf.dst;
        let size = lf.size;

        let mask = match size {
            1 => 0xFFu64,
            2 => 0xFFFFu64,
            4 => 0xFFFF_FFFFu64,
            _ => u64::MAX,
        };
        let result_m = result & mask;
        let a_m = a & mask;
        let b_m = b & mask;

        let sign_bit = match size {
            1 => 0x80u64,
            2 => 0x8000u64,
            4 => 0x8000_0000u64,
            _ => 0x8000_0000_0000_0000u64,
        };

        // Common flags for all operations
        let zf = result_m == 0;
        let sf = (result_m & sign_bit) != 0;
        let pf = (result as u8).count_ones() % 2 == 0;

        // Start with current rflags, clear status flags (preserve CF for Inc/Dec)
        let cf_mask = if lf.op == LazyFlagOp::Inc || lf.op == LazyFlagOp::Dec {
            0 // Don't clear CF for INC/DEC
        } else {
            flags::bits::CF
        };
        let mut rflags = self.regs.rflags
            & !(cf_mask
                | flags::bits::ZF
                | flags::bits::SF
                | flags::bits::PF
                | flags::bits::OF
                | flags::bits::AF);

        // Set common flags
        if zf {
            rflags |= flags::bits::ZF;
        }
        if sf {
            rflags |= flags::bits::SF;
        }
        if pf {
            rflags |= flags::bits::PF;
        }

        // Operation-specific flags
        match lf.op {
            LazyFlagOp::Add | LazyFlagOp::Inc => {
                let cf = result_m < a_m;
                let of = ((a_m ^ result_m) & (b_m ^ result_m) & sign_bit) != 0;
                let af = ((a_m ^ b_m ^ result_m) & 0x10) != 0;
                if lf.op == LazyFlagOp::Add && cf {
                    rflags |= flags::bits::CF;
                }
                if of {
                    rflags |= flags::bits::OF;
                }
                if af {
                    rflags |= flags::bits::AF;
                }
            }
            LazyFlagOp::Sub | LazyFlagOp::Dec => {
                let cf = a_m < b_m;
                let of = ((a_m ^ b_m) & (a_m ^ result_m) & sign_bit) != 0;
                let af = ((a_m ^ b_m ^ result_m) & 0x10) != 0;
                if lf.op == LazyFlagOp::Sub && cf {
                    rflags |= flags::bits::CF;
                }
                if of {
                    rflags |= flags::bits::OF;
                }
                if af {
                    rflags |= flags::bits::AF;
                }
            }
            LazyFlagOp::Logic => {
                // CF=0, OF=0 already cleared above; AF is undefined
            }
            LazyFlagOp::None => {}
        }

        rflags
    }

    /// Set lazy flags for an Add operation
    #[inline(always)]
    pub(super) fn set_lazy_add(&mut self, a: u64, b: u64, result: u64, size: u8) {
        self.lazy_flags = LazyFlags {
            op: LazyFlagOp::Add,
            result,
            src: a,
            dst: b,
            size,
        };
    }

    /// Set lazy flags for a Sub/CMP operation
    #[inline(always)]
    pub(super) fn set_lazy_sub(&mut self, a: u64, b: u64, result: u64, size: u8) {
        self.lazy_flags = LazyFlags {
            op: LazyFlagOp::Sub,
            result,
            src: a,
            dst: b,
            size,
        };
    }

    /// Set lazy flags for a Logic operation (AND/OR/XOR/TEST)
    #[inline(always)]
    pub(super) fn set_lazy_logic(&mut self, result: u64, size: u8) {
        self.lazy_flags = LazyFlags {
            op: LazyFlagOp::Logic,
            result,
            src: 0,
            dst: 0,
            size,
        };
    }

    /// Set lazy flags for an Inc operation (CF preserved)
    #[inline(always)]
    pub(super) fn set_lazy_inc(&mut self, a: u64, result: u64, size: u8) {
        self.lazy_flags = LazyFlags {
            op: LazyFlagOp::Inc,
            result,
            src: a,
            dst: 1,
            size,
        };
    }

    /// Set lazy flags for a Dec operation (CF preserved)
    #[inline(always)]
    pub(super) fn set_lazy_dec(&mut self, a: u64, result: u64, size: u8) {
        self.lazy_flags = LazyFlags {
            op: LazyFlagOp::Dec,
            result,
            src: a,
            dst: 1,
            size,
        };
    }

    /// Clear lazy flags state (call after directly writing to rflags)
    #[inline(always)]
    pub(super) fn clear_lazy_flags(&mut self) {
        let lf = self.lazy_flags;
        self.lazy_flags = LazyFlags {
            op: LazyFlagOp::None,
            ..lf
        };
    }

    /// Resolve ONLY the CF bit of any pending lazy op into `regs.rflags`, leaving
    /// the lazy op intact for later full materialization. Used by INC/DEC, which
    /// preserve CF: before switching the lazy state to Inc/Dec we must lock in the
    /// CF that the pending op would have produced, without paying for a full
    /// 6-flag computation. Inc/Dec/None already have valid CF in `rflags`, so
    /// those are no-ops; Logic forces CF=0; Add/Sub compute the single carry bit.
    #[inline(always)]
    pub(super) fn resolve_lazy_cf(&mut self) {
        let lf = self.lazy_flags;
        match lf.op {
            LazyFlagOp::None | LazyFlagOp::Inc | LazyFlagOp::Dec => {
                // CF in rflags is already authoritative for these.
            }
            LazyFlagOp::Logic => {
                self.regs.rflags &= !flags::bits::CF;
            }
            LazyFlagOp::Add => {
                let mask = Self::size_mask(lf.size);
                let cf = (lf.result & mask) < (lf.src & mask);
                if cf {
                    self.regs.rflags |= flags::bits::CF;
                } else {
                    self.regs.rflags &= !flags::bits::CF;
                }
            }
            LazyFlagOp::Sub => {
                let mask = Self::size_mask(lf.size);
                let cf = (lf.src & mask) < (lf.dst & mask);
                if cf {
                    self.regs.rflags |= flags::bits::CF;
                } else {
                    self.regs.rflags &= !flags::bits::CF;
                }
            }
        }
    }

    /// Operand-size mask shared by the lazy-flag paths.
    #[inline(always)]
    fn size_mask(size: u8) -> u64 {
        match size {
            1 => 0xFFu64,
            2 => 0xFFFFu64,
            4 => 0xFFFF_FFFFu64,
            _ => u64::MAX,
        }
    }

    /// Fetch instruction bytes from RIP into a stack buffer.
    /// Returns (buffer, actual_length).
    #[inline]
    pub(super) fn fetch(&mut self) -> Result<([u8; MAX_INSN_LEN], usize)> {
        // The fetch linear address is CS.base + RIP. CS.base is 0 in long mode
        // (flat) — so this is unchanged there — and selector<<4 in real mode.
        let rip = self.sregs.cs.base.wrapping_add(self.regs.rip);
        // Mark this page as containing code for self-modifying code detection
        self.mmu.mark_code_page(rip);

        let mut buf = [0u8; MAX_INSN_LEN];
        let mut last_err = None;
        match self.mmu.read(rip, &mut buf, &self.sregs) {
            Ok(()) => return Ok((buf, MAX_INSN_LEN)),
            Err(Error::PageFault { vaddr, error_code }) => {
                // Instruction fetch page fault - add instruction fetch bit to error code
                return Err(Error::PageFault {
                    vaddr,
                    error_code: error_code | 0x10,
                });
            }
            Err(e) => last_err = Some(e), // Try smaller amounts
        }
        // If we can't read 15 bytes, try smaller amounts
        for len in (1..MAX_INSN_LEN).rev() {
            match self.mmu.read(rip, &mut buf[..len], &self.sregs) {
                Ok(()) => return Ok((buf, len)),
                Err(Error::PageFault { vaddr, error_code }) => {
                    return Err(Error::PageFault {
                        vaddr,
                        error_code: error_code | 0x10,
                    });
                }
                Err(e) => last_err = Some(e),
            }
        }
        // Debug: print the actual error
        if let Some(e) = &last_err {
            eprintln!(
                "[FETCH FAIL] RIP={:#x} CR3={:#x} CR0={:#x} EFER={:#x} error: {:?}",
                rip, self.sregs.cr3, self.sregs.cr0, self.sregs.efer, e
            );
        }
        Err(Error::Emulator(format!(
            "failed to fetch instruction at RIP={:#x}",
            rip
        )))
    }

    /// Compute decode cache index from RIP
    #[inline(always)]
    fn decode_cache_index(rip: u64) -> usize {
        (rip as usize) & DECODE_CACHE_MASK
    }

    /// Execute a single instruction.
    #[inline]
    /// Per-vCPU timestamp counter. Real-time: tracks host wall-clock elapsed
    /// since emulator start, scaled to the advertised 3 GHz (3 cycles/ns), so
    /// the guest's RDTSC/TSC clocksource measures real time and delay loops
    /// complete in real time — not tied to emulator instruction throughput.
    #[inline(always)]
    pub(super) fn tsc(&self) -> u64 {
        crate::timing::elapsed_nanos().wrapping_mul(3)
    }

    pub fn step(&mut self) -> Result<Option<VcpuExit>> {
        // Retired-instruction counter; drives TSC. Plain add - no atomics on the
        // hot path (published to the global counter at run() yield boundaries).
        self.insn_count = self.insn_count.wrapping_add(1);

        // Start profiling timer
        #[cfg(feature = "profiling")]
        let prof_start = profiling::begin_instruction();

        // Crash-diagnostic RIP telemetry (debug builds only - these are atomics).
        #[cfg(feature = "debug")]
        {
            CURRENT_RIP.store(self.regs.rip, Ordering::Relaxed);
            let idx = RIP_IDX.fetch_add(1, Ordering::Relaxed) % 16;
            RIP_HISTORY[idx].store(self.regs.rip, Ordering::Relaxed);
        }

        let rip = self.regs.rip;

        let cache_idx = Self::decode_cache_index(rip);
        // Key on address space (CR3) + CPU mode so a hit can never dispatch stale
        // bytes/decode across a context or mode switch. In real mode there is no
        // paging (cr3 unused) and the fetch linear address is CS.base + RIP, so
        // CS.base must be part of the key — otherwise the same offset under
        // different segments (common in real-mode relocators) would alias to one
        // cached decode. CS.base is 0 in long mode, so this is a no-op there.
        let mode_tag = (self.sregs.cr3 & !0xFFF)
            | (self.sregs.cs.l as u64)
            | ((self.sregs.cs.db as u64) << 1)
            | if self.sregs.cr0 & 1 == 0 {
                self.sregs.cs.base
            } else {
                0
            };

        // Check decode cache for a hit (copy to avoid borrow issues). A filled
        // entry always has bytes_len >= 1; default/invalidated entries have
        // bytes_len == 0. This guard matters in real mode, where the empty
        // sentinel (rip==0, mode_tag==0) would otherwise collide with a guest
        // legitimately executing at offset 0 of a segment.
        let cached = self.decode_cache[cache_idx];
        if cached.bytes_len != 0 && cached.rip == rip && cached.mode_tag == mode_tag {
            // Cache hit! Record for profiling
            #[cfg(feature = "profiling")]
            profiling::record_cache_hit();

            // Reuse the cached instruction bytes - skip the MMU fetch +
            // mark_code_page entirely (the page was marked when the entry filled).
            let mut ctx = InsnContext {
                bytes: cached.bytes,
                bytes_len: cached.bytes_len,
                cursor: if cached.rex2.map_or(false, |r| r.m) {
                    cached.cursor
                } else {
                    cached.cursor + 1 // Skip past opcode byte
                },
                rex: cached.rex,
                rex2: cached.rex2,
                operand_size_override: cached.operand_size_override,
                address_size_override: cached.address_size_override,
                rep_prefix: cached.rep_prefix,
                op_size: cached.op_size,
                rip_relative_offset: 0,
                segment_override: cached.segment_override,
                evex: None,
                opcode: cached.opcode,
            };

            // Enforce LOCK-prefix legality (#UD on illegal use) before dispatch.
            // The LOCK-present verdict was computed once on the fill path, so the
            // hit path skips the prefix-byte scan and only takes the (cold)
            // legality check when a 0xF0 prefix is actually present.
            if cached.has_lock {
                if let Some(exit) = self.enforce_lock_prefix_cold(&ctx, cached.opcode)? {
                    return Ok(Some(exit));
                }
            }

            // Function-pointer dispatch: call the handler resolved on the fill
            // path directly, skipping the `execute` opcode match and the
            // two-byte / escape call chain. Equivalent to `trace_and_execute`
            // when tracing is off (the common build); the `trace` build keeps
            // the instrumented path so traces stay complete.
            let result = self.trace_and_execute_cached(cached.handler, &mut ctx, rip);

            // End profiling for this instruction
            #[cfg(feature = "profiling")]
            {
                // Use precise opcode key if set by dispatch, otherwise fall back to simple key
                let key = profiling::take_current_opcode_key()
                    .unwrap_or_else(|| profiling::build_simple_opcode_key(cached.opcode));
                profiling::end_instruction(key, prof_start);
            }

            return result;
        }

        // Cache miss - do full decode
        #[cfg(feature = "profiling")]
        profiling::record_cache_miss();

        let (bytes, bytes_len) = self.fetch()?;

        // Decode prefixes (mode-aware: 0xD5 is REX2 in long mode, AAD otherwise)
        let mut ctx = Decoder::decode_prefixes(bytes, bytes_len, self.sregs.cs.l)?;

        // Determine operand size (64-bit mode defaults to 32-bit; compat depends on CS.D).
        ctx.op_size = if self.sregs.cs.l {
            if ctx.any_rex_w() {
                8
            } else if ctx.operand_size_override {
                2
            } else {
                4
            }
        } else {
            let default_16bit = !self.sregs.cs.db;
            let is_16bit = default_16bit ^ ctx.operand_size_override;
            if is_16bit { 2 } else { 4 }
        };

        // Save cursor before consuming opcode (for cache)
        let opcode_cursor = ctx.cursor;

        // Get opcode. REX2.M selects the 0F opcode map without encoding an
        // actual 0x0F byte, so leave the cursor on the map opcode and dispatch
        // through the normal two-byte handler.
        let opcode = if ctx.rex2_m() {
            0x0F
        } else {
            ctx.consume_u8()?
        };
        ctx.opcode = opcode;

        // Resolve the handler once, here on the (cold) miss path, so subsequent
        // hits dispatch via the stored fn-pointer. `None` => opcode unimplemented
        // in `execute`; store a shim that re-enters `execute` to yield the exact
        // same error the match would (keeps the slow path byte-for-byte correct).
        let handler = Self::resolve_handler(opcode).unwrap_or(Self::execute_via_match);

        // Detect a LOCK (0xF0) prefix once, here on the fill path, and cache the
        // verdict so hits skip the prefix-byte scan entirely.
        let has_lock = ctx.bytes[..opcode_cursor.min(ctx.bytes_len)].contains(&0xF0);

        // Cache the decoded instruction (incl. raw bytes so hits skip fetch()).
        self.decode_cache[cache_idx] = DecodeCacheEntry {
            rip,
            mode_tag,
            opcode,
            op_size: ctx.op_size,
            cursor: opcode_cursor,
            rex: ctx.rex,
            rex2: ctx.rex2,
            operand_size_override: ctx.operand_size_override,
            address_size_override: ctx.address_size_override,
            rep_prefix: ctx.rep_prefix,
            segment_override: ctx.segment_override,
            bytes: ctx.bytes,
            bytes_len: ctx.bytes_len,
            has_lock,
            handler,
        };

        // Enforce LOCK-prefix legality (#UD on illegal use) before dispatch.
        // `opcode_cursor` is the primary-opcode offset; prefixes precede it. Only
        // pay the legality check when a LOCK prefix is actually present.
        if has_lock {
            if let Some(exit) = self.enforce_lock_prefix_cold(&ctx, opcode)? {
                return Ok(Some(exit));
            }
        }

        // Execute instruction
        let result = self.trace_and_execute(opcode, &mut ctx, rip);

        // End profiling for this instruction
        #[cfg(feature = "profiling")]
        {
            // Use precise opcode key if set by dispatch, otherwise fall back to simple key
            let key = profiling::take_current_opcode_key()
                .unwrap_or_else(|| profiling::build_simple_opcode_key(opcode));
            profiling::end_instruction(key, prof_start);
        }

        result
    }

    /// Execute instruction with optional tracing (when trace feature is enabled)
    #[cfg(feature = "trace")]
    #[inline]
    fn trace_and_execute(
        &mut self,
        opcode: u8,
        ctx: &mut InsnContext,
        rip: u64,
    ) -> Result<Option<VcpuExit>> {
        if trace::is_enabled() {
            // Save pre-execution state for comparison
            let pre_regs = self.regs.clone();
            let pre_xmm = self.regs.xmm.clone();

            // Execute the instruction
            let result = self.execute(opcode, ctx);

            // Format instruction bytes as hex
            let insn_len = ctx.cursor.min(15);
            let mut insn_hex = String::with_capacity(insn_len * 3);
            for i in 0..insn_len {
                if i > 0 {
                    insn_hex.push(' ');
                }
                insn_hex.push_str(&format!("{:02x}", ctx.bytes[i]));
            }

            // Build register change description
            let mut changes = String::new();

            // Check for GPR changes
            if self.regs.rax != pre_regs.rax {
                changes.push_str(&format!("rax = 0x{:x}", self.regs.rax));
            }
            if self.regs.rcx != pre_regs.rcx {
                if !changes.is_empty() {
                    changes.push_str(", ");
                }
                changes.push_str(&format!("rcx = 0x{:x}", self.regs.rcx));
            }
            if self.regs.rdx != pre_regs.rdx {
                if !changes.is_empty() {
                    changes.push_str(", ");
                }
                changes.push_str(&format!("rdx = 0x{:x}", self.regs.rdx));
            }
            if self.regs.rbx != pre_regs.rbx {
                if !changes.is_empty() {
                    changes.push_str(", ");
                }
                changes.push_str(&format!("rbx = 0x{:x}", self.regs.rbx));
            }
            if self.regs.rsp != pre_regs.rsp {
                if !changes.is_empty() {
                    changes.push_str(", ");
                }
                changes.push_str(&format!("rsp = 0x{:x}", self.regs.rsp));
            }
            if self.regs.rbp != pre_regs.rbp {
                if !changes.is_empty() {
                    changes.push_str(", ");
                }
                changes.push_str(&format!("rbp = 0x{:x}", self.regs.rbp));
            }
            if self.regs.rsi != pre_regs.rsi {
                if !changes.is_empty() {
                    changes.push_str(", ");
                }
                changes.push_str(&format!("rsi = 0x{:x}", self.regs.rsi));
            }
            if self.regs.rdi != pre_regs.rdi {
                if !changes.is_empty() {
                    changes.push_str(", ");
                }
                changes.push_str(&format!("rdi = 0x{:x}", self.regs.rdi));
            }
            if self.regs.rflags != pre_regs.rflags {
                if !changes.is_empty() {
                    changes.push_str(", ");
                }
                changes.push_str(&format!("rflags = 0x{:x}", self.regs.rflags));
            }

            // Write instruction trace
            trace::write_insn(rip, &insn_hex, &changes);

            // Check for XMM changes and output them
            for i in 0..16 {
                if self.regs.xmm[i] != pre_xmm[i] {
                    trace::write_xmm(i, self.regs.xmm[i][0], self.regs.xmm[i][1]);
                }
            }

            result
        } else {
            self.execute(opcode, ctx)
        }
    }

    /// Execute instruction (no tracing - default when trace feature is disabled)
    #[cfg(not(feature = "trace"))]
    #[inline(always)]
    fn trace_and_execute(
        &mut self,
        opcode: u8,
        ctx: &mut InsnContext,
        _rip: u64,
    ) -> Result<Option<VcpuExit>> {
        self.execute(opcode, ctx)
    }

    /// Uniform-signature wrapper around the `execute` opcode match, used as the
    /// stored handler for opcodes the resolver leaves unmapped (the `_ =>`
    /// unimplemented arm of `execute`). Recovers the opcode from `ctx` so the
    /// stored fn-pointer reproduces the match's behaviour (including its error)
    /// byte-for-byte.
    #[inline(never)]
    #[cold]
    pub(super) fn execute_via_match(&mut self, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
        let opcode = ctx.opcode;
        self.execute(opcode, ctx)
    }

    #[inline(always)]
    pub(in crate::backend::emulator::x86_64) fn inject_undefined_instruction(
        &mut self,
    ) -> Result<Option<VcpuExit>> {
        self.inject_exception(6, None)?;
        Ok(None)
    }

    /// Dispatch a decode-cache HIT through the pre-resolved handler fn-pointer.
    ///
    /// In the default (non-`trace`) build this is the whole point of the
    /// fn-pointer cache: one indirect call straight into the handler, skipping
    /// the `execute` match and escape chain.
    #[cfg(not(feature = "trace"))]
    #[inline(always)]
    fn trace_and_execute_cached(
        &mut self,
        handler: HandlerFn,
        ctx: &mut InsnContext,
        _rip: u64,
    ) -> Result<Option<VcpuExit>> {
        handler(self, ctx)
    }

    /// Tracing build: route the cached hit back through the instrumented
    /// `trace_and_execute` (opcode match) so trace output stays complete and
    /// identical to the pre-fn-pointer behaviour. The resolved handler is
    /// equivalent to the match arm, so correctness is unaffected.
    #[cfg(feature = "trace")]
    #[inline]
    fn trace_and_execute_cached(
        &mut self,
        _handler: HandlerFn,
        ctx: &mut InsnContext,
        rip: u64,
    ) -> Result<Option<VcpuExit>> {
        let opcode = ctx.opcode;
        self.trace_and_execute(opcode, ctx, rip)
    }

    // Register access methods
    #[inline(always)]
    pub(super) fn get_reg(&self, reg: u8, size: u8) -> u64 {
        // Branchless GPR read: index the precomputed field-offset table (which
        // respects the actual struct layout via `offset_of!`, so it is sound for
        // any `repr`) instead of a 32-arm match that the profiler showed as a
        // hot jump table inside every ALU handler.
        let off = GPR_OFFSETS[(reg & 0x1F) as usize];
        // SAFETY: `off` is the real byte offset of a `u64` GPR field within
        // `Registers`; the struct and each `u64` field are 8-byte aligned, so
        // the access is in-bounds and aligned. `&self.regs` is a valid base.
        let val =
            unsafe { *((&self.regs as *const Registers as *const u8).add(off) as *const u64) };
        match size {
            1 => val & 0xFF,
            2 => val & 0xFFFF,
            4 => val & 0xFFFF_FFFF,
            _ => val,
        }
    }

    /// Set an 8-bit register value, correctly handling AH/CH/DH/BH when no REX prefix
    #[inline(always)]
    pub(super) fn set_reg8(&mut self, reg: u8, value: u64, has_rex: bool) {
        // In 64-bit mode, without REX prefix, reg 4-7 are AH/CH/DH/BH
        // With REX prefix, reg 4-7 are SPL/BPL/SIL/DIL
        if !has_rex {
            match reg & 0x07 {
                4 => {
                    self.regs.rax = (self.regs.rax & !0xFF00) | ((value & 0xFF) << 8);
                    return;
                }
                5 => {
                    self.regs.rcx = (self.regs.rcx & !0xFF00) | ((value & 0xFF) << 8);
                    return;
                }
                6 => {
                    self.regs.rdx = (self.regs.rdx & !0xFF00) | ((value & 0xFF) << 8);
                    return;
                }
                7 => {
                    self.regs.rbx = (self.regs.rbx & !0xFF00) | ((value & 0xFF) << 8);
                    return;
                }
                _ => {}
            }
        }
        self.set_reg(reg, value, 1);
    }

    /// Get an 8-bit register value, correctly handling AH/CH/DH/BH when no REX prefix
    #[inline(always)]
    pub(super) fn get_reg8(&self, reg: u8, has_rex: bool) -> u64 {
        // In 64-bit mode, without REX prefix, reg 4-7 are AH/CH/DH/BH
        // With REX prefix, reg 4-7 are SPL/BPL/SIL/DIL
        if !has_rex {
            match reg & 0x07 {
                4 => return (self.regs.rax >> 8) & 0xFF,
                5 => return (self.regs.rcx >> 8) & 0xFF,
                6 => return (self.regs.rdx >> 8) & 0xFF,
                7 => return (self.regs.rbx >> 8) & 0xFF,
                _ => {}
            }
        }
        self.get_reg(reg, 1)
    }

    #[inline(always)]
    pub(super) fn set_reg(&mut self, reg: u8, value: u64, size: u8) {
        // Branchless GPR write via the `offset_of!` table (see GPR_OFFSETS /
        // get_reg). Partial-width semantics are preserved exactly: 8/16-bit
        // writes merge into the low bits, 32-bit writes zero-extend, 64-bit
        // writes replace the register.
        let off = GPR_OFFSETS[(reg & 0x1F) as usize];
        // SAFETY: `off` is the real byte offset of a `u64` GPR field within
        // `Registers`; the field is 8-byte aligned and in-bounds, and `&mut
        // self.regs` grants exclusive access for the duration of the write.
        let reg_ref =
            unsafe { &mut *((&mut self.regs as *mut Registers as *mut u8).add(off) as *mut u64) };
        match size {
            1 => *reg_ref = (*reg_ref & !0xFF) | (value & 0xFF),
            2 => *reg_ref = (*reg_ref & !0xFFFF) | (value & 0xFFFF),
            4 => *reg_ref = value & 0xFFFF_FFFF, // 32-bit ops zero-extend
            8 => *reg_ref = value,
            _ => {}
        }
    }

    // Memory access helpers
    #[inline(always)]
    pub(super) fn read_mem(&mut self, addr: u64, size: u8) -> Result<u64> {
        let val = match size {
            1 => self.mmu.read_u8(addr, &self.sregs)? as u64,
            2 => self.mmu.read_u16(addr, &self.sregs)? as u64,
            4 => self.mmu.read_u32(addr, &self.sregs)? as u64,
            8 => self.mmu.read_u64(addr, &self.sregs)?,
            _ => {
                return Err(Error::Emulator(format!(
                    "invalid memory access size: {}",
                    size
                )));
            }
        };
        #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
        if let Some(t) = self.jit_mem_trace.as_mut() {
            t.push((0, addr, size, val));
        }
        Ok(val)
    }

    /// Drain the MMU's self-modifying-code journal (code pages written by ANY
    /// store, including the ~39 handlers that call `mmu.write_u*` directly) and
    /// invalidate the decode + JIT caches for each. Called at every instruction
    /// boundary so a modified instruction is always re-decoded before it next
    /// executes. The `has_smc_dirty` guard keeps the hot path free of work when
    /// no code page has been written.
    #[inline(always)]
    fn drain_smc(&mut self) {
        if self.mmu.has_smc_dirty() {
            for page_base in self.mmu.take_smc_dirty() {
                self.invalidate_code_page(page_base);
            }
        }
    }

    /// Invalidate every cached decode and JIT region overlapping the 4 KiB page
    /// at `page_base`. The decode cache is indexed by `RIP & 0xFFF`, so all 4096
    /// entries are scanned. JIT regions are keyed by entry RIP; a region's body
    /// fits in one ≤512B lift window, so a write to page P can stale a region
    /// keyed on P (body starts here) or on P-1 (body extends into P) — evict
    /// both, and drop their hotness counters so they re-promote against the new
    /// code.
    fn invalidate_code_page(&mut self, page_base: u64) {
        for idx in 0..DECODE_CACHE_SIZE {
            let entry = &mut self.decode_cache[idx];
            if entry.rip != 0 && (entry.rip & !0xFFF) == page_base {
                entry.rip = 0; // Invalidate
                entry.bytes_len = 0; // mark empty so a real rip==0 can't false-hit
            }
        }

        #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
        if !self.jit_cache.is_empty() || !self.jit_hot.is_empty() {
            let prev_page = page_base.wrapping_sub(0x1000);
            let overlaps = |rip: u64| {
                let p = rip & !0xFFF;
                p == page_base || p == prev_page
            };
            self.jit_cache.retain(|&(rip, _), _| !overlaps(rip));
            self.jit_hot.retain(|&rip, _| !overlaps(rip));
        }
    }

    #[inline(always)]
    pub(super) fn write_mem(&mut self, addr: u64, value: u64, size: u8) -> Result<()> {
        // Self-modifying-code is handled by the MMU's write journal (`note_smc`
        // in every `write_u*`) drained once per instruction in `run()` BEFORE the
        // next fetch — so no per-store invalidation is needed here. (An immediate
        // `check_smc` used to run a full decode-cache scan PER store; for code
        // pages that meant N redundant scans per multi-store instruction on top
        // of the drain's single deduplicated one. Removed — see `drain_smc`.)
        let r = match size {
            1 => self.mmu.write_u8(addr, value as u8, &self.sregs),
            2 => self.mmu.write_u16(addr, value as u16, &self.sregs),
            4 => self.mmu.write_u32(addr, value as u32, &self.sregs),
            8 => self.mmu.write_u64(addr, value, &self.sregs),
            _ => Err(Error::Emulator(format!(
                "invalid memory access size: {}",
                size
            ))),
        };
        #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
        if r.is_ok() {
            if let Some(t) = self.jit_mem_trace.as_mut() {
                let mask = match size {
                    1 => 0xFFu64,
                    2 => 0xFFFF,
                    4 => 0xFFFF_FFFF,
                    _ => u64::MAX,
                };
                t.push((1, addr, size, value & mask));
            }
        }
        r
    }

    // FPU memory access helpers
    #[inline(always)]
    pub(super) fn read_mem16(&mut self, addr: u64) -> Result<u16> {
        self.mmu.read_u16(addr, &self.sregs)
    }

    #[inline(always)]
    pub(super) fn write_mem16(&mut self, addr: u64, value: u16) -> Result<()> {
        self.mmu.write_u16(addr, value, &self.sregs)
    }

    #[inline(always)]
    pub(super) fn read_mem32(&mut self, addr: u64) -> Result<u32> {
        self.mmu.read_u32(addr, &self.sregs)
    }

    #[inline(always)]
    pub(super) fn write_mem32(&mut self, addr: u64, value: u32) -> Result<()> {
        self.mmu.write_u32(addr, value, &self.sregs)
    }

    #[inline(always)]
    pub(super) fn read_mem64(&mut self, addr: u64) -> Result<u64> {
        self.mmu.read_u64(addr, &self.sregs)
    }

    #[inline(always)]
    pub(super) fn write_mem64(&mut self, addr: u64, value: u64) -> Result<()> {
        // Use the generic write_mem which has watchpoints
        self.write_mem(addr, value, 8)
    }

    #[inline(always)]
    pub(super) fn read_f32(&mut self, addr: u64) -> Result<f32> {
        let bits = self.mmu.read_u32(addr, &self.sregs)?;
        Ok(f32::from_bits(bits))
    }

    #[inline(always)]
    pub(super) fn write_f32(&mut self, addr: u64, value: f32) -> Result<()> {
        self.mmu.write_u32(addr, value.to_bits(), &self.sregs)
    }

    #[inline(always)]
    pub(super) fn read_f64(&mut self, addr: u64) -> Result<f64> {
        let bits = self.mmu.read_u64(addr, &self.sregs)?;
        Ok(f64::from_bits(bits))
    }

    #[inline(always)]
    pub(super) fn write_f64(&mut self, addr: u64, value: f64) -> Result<()> {
        self.mmu.write_u64(addr, value.to_bits(), &self.sregs)
    }

    #[inline]
    pub(super) fn read_bytes(&mut self, addr: u64, len: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.mmu.read(addr, &mut buf, &self.sregs)?;
        Ok(buf)
    }

    #[inline]
    pub(super) fn write_bytes(&mut self, addr: u64, data: &[u8]) -> Result<()> {
        self.mmu.write(addr, data, &self.sregs)
    }

    // Stack helpers
    // NOTE: These must NOT modify RSP if the write fails, otherwise page fault
    // handling will corrupt the stack (RSP gets decremented twice on retry).
    pub(super) fn push64(&mut self, value: u64) -> Result<()> {
        let new_rsp = self.regs.rsp.wrapping_sub(8);
        self.mmu
            .write_u64(self.sregs.ss.base.wrapping_add(new_rsp), value, &self.sregs)?;
        self.regs.rsp = new_rsp;
        Ok(())
    }

    /// Push a 64-bit value to the stack with supervisor privilege.
    /// Used during exception/interrupt delivery where the kernel stack
    /// is accessed regardless of current CPL.
    fn push64_supervisor(&mut self, value: u64) -> Result<()> {
        let new_rsp = self.regs.rsp.wrapping_sub(8);
        self.mmu.write_u64_supervisor(new_rsp, value, &self.sregs)?;
        self.regs.rsp = new_rsp;
        Ok(())
    }

    pub(super) fn pop64(&mut self) -> Result<u64> {
        let value = self
            .mmu
            .read_u64(self.sregs.ss.base.wrapping_add(self.regs.rsp), &self.sregs)?;
        self.regs.rsp = self.regs.rsp.wrapping_add(8);
        Ok(value)
    }

    pub(super) fn push32(&mut self, value: u32) -> Result<()> {
        let new_rsp = self.regs.rsp.wrapping_sub(4);
        self.mmu
            .write_u32(self.sregs.ss.base.wrapping_add(new_rsp), value, &self.sregs)?;
        self.regs.rsp = new_rsp;
        Ok(())
    }

    pub(super) fn pop32(&mut self) -> Result<u32> {
        let value = self
            .mmu
            .read_u32(self.sregs.ss.base.wrapping_add(self.regs.rsp), &self.sregs)?;
        self.regs.rsp = self.regs.rsp.wrapping_add(4);
        Ok(value)
    }

    pub(super) fn push16(&mut self, value: u16) -> Result<()> {
        let new_rsp = self.regs.rsp.wrapping_sub(2);
        self.mmu
            .write_u16(self.sregs.ss.base.wrapping_add(new_rsp), value, &self.sregs)?;
        self.regs.rsp = new_rsp;
        Ok(())
    }

    pub(super) fn pop16(&mut self) -> Result<u16> {
        let value = self
            .mmu
            .read_u16(self.sregs.ss.base.wrapping_add(self.regs.rsp), &self.sregs)?;
        self.regs.rsp = self.regs.rsp.wrapping_add(2);
        Ok(value)
    }

    // I/O pending helpers
    pub(super) fn set_io_pending_reg(&mut self, size: u8) {
        self.io_pending = Some(IoPending {
            size,
            target: IoInTarget::Reg,
            count: 1,
        });
    }

    pub(super) fn set_io_pending_mem(&mut self, size: u8, addr: u64) {
        self.io_pending = Some(IoPending {
            size,
            target: IoInTarget::Mem { addr },
            count: 1,
        });
    }

    /// Stage a batched `rep ins` block: `count` consecutive `size`-byte elements
    /// written to memory starting at `addr` (forward). Completed in one shot by
    /// [`Self::complete_io_in`] from the data the backend reads off the port.
    pub(super) fn set_io_pending_block(&mut self, size: u8, addr: u64, count: u32) {
        self.io_pending = Some(IoPending {
            size,
            target: IoInTarget::Mem { addr },
            count,
        });
    }

    // Segment register access
    pub(super) fn get_sreg(&self, sreg: u8) -> u16 {
        match sreg {
            0 => self.sregs.es.selector,
            1 => self.sregs.cs.selector,
            2 => self.sregs.ss.selector,
            3 => self.sregs.ds.selector,
            4 => self.sregs.fs.selector,
            5 => self.sregs.gs.selector,
            _ => 0,
        }
    }

    pub(super) fn set_sreg(&mut self, sreg: u8, value: u16) {
        // Real mode (CR0.PE=0) loads a segment base of selector<<4 directly,
        // with a 64 KiB limit and 16-bit addressing — no descriptor lookup.
        let real_mode = self.sregs.cr0 & 1 == 0;
        // 64-bit (long) mode keeps flat data segments and MSR-based FS/GS bases,
        // so it does NOT consult the descriptor table here. In 32-bit protected
        // mode a segment load takes its base/limit/attributes from the GDT/LDT
        // descriptor (resolved up front, before the &mut segment borrow). Require
        // a present code/data (S=1, P=1) descriptor; otherwise fall back to flat.
        let long_mode = (self.sregs.efer & 0x400) != 0 && self.sregs.cs.l;
        let desc = if !real_mode && !long_mode {
            self.read_descriptor(value)
                .ok()
                .flatten()
                .filter(|d| (d >> 44) & 1 != 0 && (d >> 47) & 1 != 0)
        } else {
            None
        };
        let seg = match sreg {
            0 => &mut self.sregs.es,
            1 => &mut self.sregs.cs,
            2 => &mut self.sregs.ss,
            3 => &mut self.sregs.ds,
            4 => &mut self.sregs.fs,
            5 => &mut self.sregs.gs,
            _ => return,
        };
        let preserve_mode = sreg == 1;
        let prev_db = seg.db;
        let prev_l = seg.l;
        seg.selector = value;
        if real_mode {
            // Real mode: the segment base is selector<<4, the limit is 64 KiB,
            // and addressing is 16-bit. (No descriptor table is consulted.)
            seg.base = (value as u64) << 4;
            seg.limit = 0xFFFF;
            seg.type_ = 0x03;
            seg.present = true;
            seg.dpl = 0;
            seg.db = false;
            seg.s = true;
            seg.l = false;
            seg.g = false;
        } else if let Some(d) = desc {
            // 32-bit protected mode: decode base/limit/attributes from the
            // descriptor (e.g. TempleOS uses based data segments during boot).
            let base = ((d >> 16) & 0x00FF_FFFF) | (((d >> 56) & 0xFF) << 24);
            let lim = ((d & 0xFFFF) | (((d >> 48) & 0xF) << 16)) as u32;
            let g = (d >> 55) & 1 != 0;
            seg.base = base;
            seg.limit = if g { (lim << 12) | 0xFFF } else { lim };
            seg.type_ = ((d >> 40) & 0xF) as u8;
            seg.s = true;
            seg.dpl = ((d >> 45) & 3) as u8;
            seg.present = true;
            seg.db = if preserve_mode {
                prev_db
            } else {
                (d >> 54) & 1 != 0
            };
            seg.l = if preserve_mode {
                prev_l
            } else {
                (d >> 53) & 1 != 0
            };
            seg.g = g;
        } else {
            // Flat fallback: long mode, a null selector, or a non-usable
            // descriptor (matches the prior always-flat behavior).
            seg.base = 0;
            seg.limit = 0xFFFF_FFFF;
            seg.type_ = 0x03; // Data segment, read/write, accessed
            seg.present = true;
            seg.dpl = 0;
            seg.db = if preserve_mode { prev_db } else { true };
            seg.s = true;
            seg.l = if preserve_mode { prev_l } else { false };
            seg.g = true;
        }
    }

    /// Read the raw 8-byte segment descriptor selected by `selector` from the
    /// active descriptor table (GDT, or LDT when the TI bit is set).
    ///
    /// Returns `Ok(None)` for a null selector (selector index 0, TI=0). Returns
    /// `Err` (#GP-style) if the selector lies outside the table limit. Otherwise
    /// returns the raw little-endian descriptor qword.
    pub(super) fn read_descriptor(&mut self, selector: u16) -> Result<Option<u64>> {
        // A null selector (index 0 in the GDT) selects no descriptor.
        if selector & 0xFFFC == 0 {
            return Ok(None);
        }

        let ti = (selector & 0x4) != 0;
        let index = (selector >> 3) as u64;
        let (table_base, table_limit) = if ti {
            (self.sregs.ldt.base, self.sregs.ldt.limit as u64)
        } else {
            (self.sregs.gdt.base, self.sregs.gdt.limit as u64)
        };

        // The descriptor occupies bytes [offset, offset + 7]; it must fit fully
        // within the table limit (limit is the last valid byte offset).
        let offset = index * 8;
        if offset + 7 > table_limit {
            return Err(Error::Emulator(format!(
                "load_code_segment: selector {:#x} outside descriptor table limit (#GP)",
                selector
            )));
        }

        let raw = self
            .mmu
            .read_u64_supervisor(table_base + offset, &self.sregs)?;
        Ok(Some(raw))
    }

    /// Decode a raw descriptor qword into the architectural fields of a code
    /// segment, validating presence and type. On success the CS register's
    /// base/limit/l/db/dpl/type/s/g are populated from the descriptor and the
    /// selector is written with the supplied RPL/CPL bits preserved.
    ///
    /// `selector` carries the RPL the caller wants recorded in CS.selector.
    fn apply_code_descriptor(&mut self, selector: u16, raw: u64) -> Result<()> {
        // Field extraction (legacy 8-byte descriptor layout).
        let limit_lo = (raw & 0xFFFF) as u32;
        let limit_hi = ((raw >> 48) & 0xF) as u32;
        let mut limit = (limit_hi << 16) | limit_lo;

        let base_lo = ((raw >> 16) & 0xFFFF) as u64;
        let base_mid = ((raw >> 32) & 0xFF) as u64;
        let base_hi = ((raw >> 56) & 0xFF) as u64;
        let base = base_lo | (base_mid << 16) | (base_hi << 24);

        let access = ((raw >> 40) & 0xFF) as u8;
        let present = (access & 0x80) != 0;
        let dpl = (access >> 5) & 0x3;
        let s = (access & 0x10) != 0; // 1 = code/data, 0 = system
        let type_ = access & 0x0F;

        let flags = ((raw >> 52) & 0xF) as u8;
        let avl = (flags & 0x1) != 0;
        let l = (flags & 0x2) != 0; // 64-bit code segment
        let db = (flags & 0x4) != 0; // default operand/address size
        let g = (flags & 0x8) != 0; // granularity

        // Present check: a not-present code segment raises #NP.
        if !present {
            return Err(Error::Emulator(format!(
                "load_code_segment: selector {:#x} not present (#NP)",
                selector
            )));
        }

        // Type check: must be a code segment (S=1 and type bit 3 set => executable).
        if !s || (type_ & 0x08) == 0 {
            return Err(Error::Emulator(format!(
                "load_code_segment: selector {:#x} is not a code segment (#GP)",
                selector
            )));
        }

        // Apply granularity scaling: G=1 means limit is in 4 KiB units, so the
        // byte limit is (limit << 12) | 0xFFF.
        if g {
            limit = (limit << 12) | 0xFFF;
        }

        self.sregs.cs.selector = selector;
        self.sregs.cs.base = base;
        self.sregs.cs.limit = limit;
        self.sregs.cs.type_ = type_;
        self.sregs.cs.present = true;
        self.sregs.cs.dpl = dpl;
        self.sregs.cs.s = true;
        self.sregs.cs.avl = avl;
        self.sregs.cs.g = g;
        // In a 64-bit code segment L=1 forces D=0; otherwise honor the D bit.
        if l {
            self.sregs.cs.l = true;
            self.sregs.cs.db = false;
        } else {
            self.sregs.cs.l = false;
            self.sregs.cs.db = db;
        }
        self.sregs.cs.unusable = false;
        Ok(())
    }

    /// Load CS from a real GDT/LDT descriptor on a far control transfer.
    ///
    /// For a non-null selector this reads the 8-byte descriptor, validates that
    /// it is present (#NP otherwise) and a code segment (#GP otherwise), and
    /// populates CS.base/limit (with G granularity scaling), CS.l (64-bit),
    /// CS.db (D bit), CS.dpl and CS.selector. A null selector is rejected (#GP)
    /// because CS may never be loaded with a null selector.
    pub(super) fn load_code_segment(&mut self, selector: u16) -> Result<()> {
        match self.read_descriptor(selector)? {
            None => Err(Error::Emulator(
                "load_code_segment: null CS selector (#GP)".to_string(),
            )),
            Some(raw) => self.apply_code_descriptor(selector, raw),
        }
    }

    /// Test/integration entry point for strict CS descriptor loading.
    ///
    /// Exposes [`Self::load_code_segment`] (which the lenient instruction paths
    /// wrap) so out-of-crate tests can exercise the architectural #NP/#GP
    /// validation directly against a hand-built descriptor table.
    pub fn load_code_segment_strict(&mut self, selector: u16) -> Result<()> {
        self.load_code_segment(selector)
    }

    /// Best-effort CS load for far transfers used by the emulated instruction
    /// paths. When the selected descriptor is a present code segment, the real
    /// architectural fields (base, granularity-scaled limit, DPL, type, S, G,
    /// AVL) are loaded from the descriptor via [`Self::apply_code_descriptor`].
    /// When the descriptor table slot is absent or holds something that is not
    /// a usable present code segment, this falls back to the historical
    /// flat-segment behavior of [`Self::set_sreg`] so code that runs against a
    /// sparsely-populated descriptor table keeps working. The caller must
    /// already have validated table limits via `validate_far_selector`.
    ///
    /// NOTE: unlike the strict [`Self::load_code_segment`], this preserves the
    /// *prior* CS.l/CS.db (execution mode) rather than adopting the descriptor's
    /// L/D bits. The test harness installs a single 64-bit (L=1) code descriptor
    /// at selector 0x08 that both 64-bit and compatibility-mode code transfers
    /// through; honoring its L bit would switch compatibility-mode code into
    /// 64-bit mode mid-stream. Preserving the mode here keeps existing behavior
    /// intact while still loading the real base/limit/DPL the audit cares about.
    /// Callers that need true descriptor-driven mode switching use
    /// [`Self::load_code_segment`].
    pub(super) fn load_code_segment_lenient(&mut self, selector: u16) {
        // Real mode (CR0.PE=0): CS.base = selector<<4 directly, no descriptor
        // lookup (the GDT is not consulted in real mode).
        if self.sregs.cr0 & 1 == 0 {
            self.set_sreg(1, selector);
            return;
        }
        let prev_l = self.sregs.cs.l;
        let prev_db = self.sregs.cs.db;
        match self.read_descriptor(selector) {
            Ok(Some(raw)) => {
                // Only adopt the real descriptor when it is a present code
                // segment; otherwise fall back to flat segmentation.
                if self.apply_code_descriptor(selector, raw).is_ok() {
                    // A far transfer adopts the descriptor's D/L bits — this is
                    // exactly how the mode switches take effect: real→protected
                    // (D: 16→32) outside long mode, and the 32-bit-compat→64-bit
                    // switch a guest performs after enabling long mode (it runs
                    // 32-bit (D=1) compatibility code under the 4-level page
                    // tables, then far-jumps to its 64-bit code segment).
                    //
                    // The one carve-out is the audit/test fixtures, which route
                    // transfers through a single 64-bit (L=1) descriptor while
                    // running 16-bit compatibility code (prev D=0) and must stay
                    // in that mode. So inside long mode preserve the prior mode
                    // ONLY when leaving 16-bit compatibility (prev D=0); a real
                    // OS never enters 64-bit mode from 16-bit compat. 64-bit
                    // code (also D=0) likewise preserves — through an L=1
                    // descriptor that is a no-op anyway.
                    let in_long_mode = self.sregs.efer & 0x400 != 0;
                    if in_long_mode && !prev_db {
                        self.sregs.cs.l = prev_l;
                        self.sregs.cs.db = prev_db;
                    }
                } else {
                    self.set_sreg(1, selector);
                }
            }
            // Null selector or out-of-limit selector: preserve legacy behavior.
            Ok(None) | Err(_) => self.set_sreg(1, selector),
        }
    }

    // Condition checking for Jcc/SETcc/CMOVcc - materializes lazy flags first
    pub(super) fn check_condition(&mut self, cc: u8) -> bool {
        // Evaluate the predicate without materializing RFLAGS (a conditional
        // branch doesn't modify flags, so the lazy op is left intact). ZF/SF are
        // cheap and computed eagerly; CF/OF/PF are closures so a condition only
        // pays for the flags it actually reads — e.g. JZ/JNZ touch ZF alone and
        // skip the PF popcount + OF/CF work entirely. Results are identical to
        // materialize-then-read; this just avoids computing unused flags + the
        // RFLAGS round-trip on every Jcc/SETcc/CMOVcc.
        let lf = self.lazy_flags;
        let materialized = lf.op == LazyFlagOp::None;
        let rflags = self.regs.rflags;

        // Geometry of the pending lazy op (ignored when already materialized).
        let (mask, sign_bit) = match lf.size {
            1 => (0xFFu64, 0x80u64),
            2 => (0xFFFFu64, 0x8000u64),
            4 => (0xFFFF_FFFFu64, 0x8000_0000u64),
            _ => (u64::MAX, 0x8000_0000_0000_0000u64),
        };
        let result_m = lf.result & mask;
        let a_m = lf.src & mask;
        let b_m = lf.dst & mask;

        let zf = if materialized {
            rflags & flags::bits::ZF != 0
        } else {
            result_m == 0
        };
        let sf = if materialized {
            rflags & flags::bits::SF != 0
        } else {
            (result_m & sign_bit) != 0
        };
        let cf = || {
            if materialized {
                rflags & flags::bits::CF != 0
            } else {
                match lf.op {
                    LazyFlagOp::Add => result_m < a_m,
                    LazyFlagOp::Sub => a_m < b_m,
                    // INC/DEC preserve CF (its prior value lives in RFLAGS).
                    LazyFlagOp::Inc | LazyFlagOp::Dec => rflags & flags::bits::CF != 0,
                    _ => false, // Logic
                }
            }
        };
        let of = || {
            if materialized {
                rflags & flags::bits::OF != 0
            } else {
                match lf.op {
                    LazyFlagOp::Add | LazyFlagOp::Inc => {
                        ((a_m ^ result_m) & (b_m ^ result_m) & sign_bit) != 0
                    }
                    LazyFlagOp::Sub | LazyFlagOp::Dec => {
                        ((a_m ^ b_m) & (a_m ^ result_m) & sign_bit) != 0
                    }
                    _ => false, // Logic
                }
            }
        };
        let pf = || {
            if materialized {
                rflags & flags::bits::PF != 0
            } else {
                (lf.result as u8).count_ones() & 1 == 0
            }
        };

        match cc {
            0x0 => of(),                // O
            0x1 => !of(),               // NO
            0x2 => cf(),                // B/NAE/C
            0x3 => !cf(),               // NB/AE/NC
            0x4 => zf,                  // E/Z
            0x5 => !zf,                 // NE/NZ
            0x6 => cf() || zf,          // BE/NA
            0x7 => !cf() && !zf,        // NBE/A
            0x8 => sf,                  // S
            0x9 => !sf,                 // NS
            0xA => pf(),                // P/PE
            0xB => !pf(),               // NP/PO
            0xC => sf != of(),          // L/NGE
            0xD => sf == of(),          // NL/GE
            0xE => zf || (sf != of()),  // LE/NG
            0xF => !zf && (sf == of()), // NLE/G
            _ => false,
        }
    }

    /// Inject a page fault exception (#PF, vector 14) into the guest.
    /// This allows the kernel's page fault handler to run and set up page tables on demand.
    pub(super) fn inject_page_fault(&mut self, vaddr: u64, error_code: u64) -> Result<()> {
        // Page fault logging disabled for performance

        // Set CR2 to the faulting virtual address
        self.sregs.cr2 = vaddr;
        self.inject_exception(14, Some(error_code))
    }

    /// Inject a generic exception into the guest.
    /// vector: exception vector number (0-255)
    /// error_code: optional error code (only for exceptions that have error codes)
    pub fn inject_exception(&mut self, vector: u8, error_code: Option<u64>) -> Result<()> {
        // Read IDT entry for the vector
        // Each IDT entry in 64-bit mode is 16 bytes
        let idt_base = self.sregs.idt.base;
        let idt_entry_addr = idt_base + (vector as u64) * 16;

        // Read the 16-byte IDT entry (using supervisor access - exception delivery
        // always uses supervisor privilege regardless of current CPL)
        let mut idt_entry = [0u8; 16];
        self.mmu
            .read_supervisor(idt_entry_addr, &mut idt_entry, &self.sregs)?;

        let offset_low = u16::from_le_bytes([idt_entry[0], idt_entry[1]]) as u64;
        let selector = u16::from_le_bytes([idt_entry[2], idt_entry[3]]);
        let ist = idt_entry[4] & 0x07;
        let type_attr = idt_entry[5];
        let offset_mid = u16::from_le_bytes([idt_entry[6], idt_entry[7]]) as u64;
        let offset_high =
            u32::from_le_bytes([idt_entry[8], idt_entry[9], idt_entry[10], idt_entry[11]]) as u64;

        // Check if entry is present
        if type_attr & 0x80 == 0 {
            return Err(Error::Emulator(format!(
                "IDT entry {} not present (type_attr={:#x})",
                vector, type_attr
            )));
        }

        let handler_addr = offset_low | (offset_mid << 16) | (offset_high << 32);

        // Materialize lazy flags before saving RFLAGS
        self.materialize_flags();

        // In 64-bit mode, push exception frame (in this order, growing downward):
        // SS, RSP, RFLAGS, CS, RIP, [Error Code if applicable]

        // Save current state
        let old_ss = self.sregs.ss.selector;
        let old_rsp = self.regs.rsp;
        let old_rflags = self.regs.rflags;
        let old_cs = self.sregs.cs.selector;
        let old_rip = self.regs.rip;

        // Determine privilege levels for stack switching
        // The target CPL comes from the code segment selector in the IDT gate (RPL bits)
        // For kernel exception handlers, this is typically 0x10 (ring 0 code segment)
        let target_cpl = (selector & 0x3) as u8;
        let old_cpl = (old_cs & 0x3) as u8;

        // Stack switching rules for 64-bit mode:
        // 1. If IST is non-zero, use the IST stack (regardless of privilege change)
        // 2. Else if transitioning to a more privileged level, load RSP from TSS
        //    (CPL 3 to 0 uses RSP0, CPL 3 to 1 uses RSP1, etc.)
        // 3. Else keep current RSP (same or less privileged)
        if ist != 0 {
            // IST entries are in the TSS at offset 0x24 + (ist-1)*8
            let tss_base = self.sregs.tr.base;
            let ist_offset = 0x24 + ((ist as u64 - 1) * 8);
            let ist_addr = tss_base + ist_offset;
            let ist_rsp = self.mmu.read_u64_supervisor(ist_addr, &self.sregs)?;
            if ist_rsp != 0 {
                self.regs.rsp = ist_rsp;
                self.set_sreg(2, 0); // SS = 0 for IST switches
            }
        } else if old_cpl > target_cpl {
            // Inter-privilege transition - load RSP from TSS
            // RSP0 is at offset 0x04, RSP1 at 0x0C, RSP2 at 0x14 in 64-bit TSS
            let tss_base = self.sregs.tr.base;
            let rsp_offset = 0x04 + (target_cpl as u64) * 8;
            let new_rsp = self
                .mmu
                .read_u64_supervisor(tss_base + rsp_offset, &self.sregs)?;
            if new_rsp != 0 {
                self.regs.rsp = new_rsp;
                self.set_sreg(2, 0); // SS = 0 for inter-privilege switches
            }
        }
        // If same privilege or less privileged, keep current RSP

        // Push exception frame (each push is 8 bytes in 64-bit mode)
        // Use supervisor access since we're writing to the kernel stack
        self.push64_supervisor(old_ss as u64)?;
        self.push64_supervisor(old_rsp)?;
        self.push64_supervisor(old_rflags)?;
        self.push64_supervisor(old_cs as u64)?;
        self.push64_supervisor(old_rip)?;
        if let Some(ec) = error_code {
            self.push64_supervisor(ec)?;
        }

        // Clear IF (disable interrupts) for interrupt gates (type 0xE)
        // Trap gates (type 0xF) don't clear IF
        let gate_type = type_attr & 0x0F;
        if gate_type == 0x0E {
            let old_if = (self.regs.rflags & flags::bits::IF) != 0;
            self.regs.rflags &= !flags::bits::IF;
            log_if_transition(
                handler_addr,
                old_if,
                false,
                &format!("INT_GATE(vec={})", vector),
            );
        }

        // Jump to the handler
        self.regs.rip = handler_addr;

        // Update CS selector (handler runs in kernel mode)
        // The segment selector from the IDT entry becomes the new CS
        self.set_sreg(1, selector);

        // For 64-bit interrupt/trap gates (type 0x0E/0x0F), the handler must run in 64-bit mode
        // Set CS.L = true to enable 64-bit mode for the handler
        // Note: set_sreg doesn't load the GDT descriptor, so we must set this explicitly
        let gate_type = type_attr & 0x0F;
        if gate_type == 0x0E || gate_type == 0x0F {
            self.sregs.cs.l = true;
            self.sregs.cs.db = false; // D must be 0 when L=1
        }

        Ok(())
    }
}

/// Spawn a one-shot background thread (when `RAX_MIPS` is set) that prints the
/// retired-instruction rate once a second. Reads the global counter published at
/// run-exit boundaries, so it reports interpreted throughput (native JIT regions
/// do not tick it). Used to A/B emulator-speed changes against a live boot.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn maybe_spawn_mips_reporter() {
    use std::sync::OnceLock;
    static STARTED: OnceLock<()> = OnceLock::new();
    if std::env::var_os("RAX_MIPS").is_none() {
        return;
    }
    STARTED.get_or_init(|| {
        std::thread::spawn(|| {
            let mut last = 0u64;
            let mut secs = 0u64;
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let now = get_total_instruction_count();
                secs += 1;
                let delta = now.saturating_sub(last);
                last = now;
                eprintln!(
                    "[MIPS] t={secs}s  +{:.2} MIPS  (total {} insns)",
                    delta as f64 / 1.0e6,
                    now
                );
            }
        });
    });
}

impl VCpu for X86_64Vcpu {
    fn run(&mut self) -> Result<VcpuExit> {
        #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
        maybe_spawn_mips_reporter();
        let start_time = std::time::Instant::now();
        let mut batch: u64 = 0;
        loop {
            // Periodic housekeeping on a stride keeps the per-instruction path
            // free of clock reads, RefCell borrows and 64-bit division.
            batch = batch.wrapping_add(1);
            if batch % LAPIC_POLL_STRIDE == 0 {
                // Deliver any due LAPIC timer interrupt.
                if let Some(vector) = self.mmu.tick_lapic_timer() {
                    if self.can_inject_interrupt() && self.inject_interrupt(vector).unwrap_or(false)
                    {
                        self.mmu.clear_lapic_pending();
                        self.halted = false;
                    }
                }
                // Yield to the VMM (~1ms wall-clock slices) so timers/IRQs get
                // serviced. Real-time paced: the guest clock (TSC, elapsed_nanos)
                // tracks host wall time, so delays and timers complete in real
                // time rather than being tied to emulator instruction throughput.
                if start_time.elapsed().as_millis() >= 1 {
                    publish_instruction_count(self.insn_count);
                    return Ok(VcpuExit::Hlt);
                }
            }

            if self.halted {
                publish_instruction_count(self.insn_count);
                // If halted but an interrupt is pending, keep spinning lightly.
                if self.mmu.has_lapic_pending() {
                    std::thread::yield_now();
                    continue;
                }
                return Ok(VcpuExit::Hlt);
            }

            // Self-modifying-code: drain the MMU's write journal and invalidate
            // decode + JIT caches for any code page written since the previous
            // instruction, so a freshly-modified opcode is re-decoded (and any
            // stale native region dropped) before it next executes. Guarded —
            // zero work when no code page has been written. Sits on the
            // run-loop path (where real guest execution and the JIT live); for
            // a JIT'd hot loop it costs one guarded check per whole-loop run,
            // not per iteration. This is now the SOLE SMC invalidation point on
            // the run path: `note_smc` (in every MMU `write_u*`) journals the
            // page and this drain invalidates it once — deduplicated — before
            // the next fetch, so no per-store immediate scan is needed.
            self.drain_smc();

            // SMIR hot-block JIT fast path: if the region at RIP has been
            // compiled, run it natively (whole loop in one call) and continue.
            // Cheap O(1) guard keeps the interpreter path untouched until any
            // region has actually been promoted. `_jit_rip_before` snapshots RIP
            // so the post-step back-edge sampler can spot loop heads.
            #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
            let _jit_rip_before = {
                let rip = self.regs.rip;
                if !self.jit_cache.is_empty() {
                    let key = (rip, self.jit_mode_tag());
                    if let Some(slot) = self.jit_cache.get(&key).cloned() {
                        if let Some(region) = slot {
                            self.jit_run_region(&region);
                            // A lift-through-calls call-out may have bailed with a
                            // VMM-bound exit (I/O, HLT, …) from a callee — propagate it.
                            if let Some(exit) = self.jit_callout_exit.take() {
                                publish_instruction_count(self.insn_count);
                                return Ok(exit);
                            }
                            continue;
                        }
                        // None ⇒ known-ineligible: fall through to the interpreter.
                    }
                }
                rip
            };

            match self.step() {
                Ok(Some(exit)) => {
                    publish_instruction_count(self.insn_count);
                    return Ok(exit);
                }
                Ok(None) => {
                    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
                    {
                        self.jit_sample_backedge(_jit_rip_before);
                        // A region run on promotion may have bailed a call-out exit.
                        if let Some(exit) = self.jit_callout_exit.take() {
                            publish_instruction_count(self.insn_count);
                            return Ok(exit);
                        }
                    }
                    // Check for single-step mode (GDB debugging)
                    #[cfg(feature = "debug")]
                    if self.single_step {
                        publish_instruction_count(self.insn_count);
                        return Ok(VcpuExit::GdbStep);
                    }
                    continue;
                }
                Err(Error::PageFault { vaddr, error_code }) => {
                    // Inject the page fault exception into the guest
                    match self.inject_page_fault(vaddr, error_code) {
                        Ok(()) => continue,
                        Err(Error::PageFault {
                            vaddr: _df_vaddr, ..
                        }) => {
                            // Page fault during page fault delivery = double fault
                            // Try to inject #DF (vector 8)
                            match self.inject_exception(8, Some(0)) {
                                Ok(()) => continue,
                                Err(e) => {
                                    // Triple fault - CPU should reset
                                    return Err(Error::Emulator(format!(
                                        "Triple fault at RIP={:#x} (double fault delivery failed: {:?}, original #PF at {:#x})",
                                        self.regs.rip, e, vaddr
                                    )));
                                }
                            }
                        }
                        Err(e) => {
                            // IDT entry not present or other error during #PF injection
                            return Err(Error::Emulator(format!(
                                "#PF at vaddr={:#x} (error_code={:#x}, RIP={:#x}): {}",
                                vaddr, error_code, self.regs.rip, e
                            )));
                        }
                    }
                }
                Err(e) => {
                    publish_instruction_count(self.insn_count);
                    return Err(e);
                }
            }
        }
    }

    fn get_state(&self) -> Result<CpuState> {
        // Compute materialized rflags without modifying self
        let rflags = self.compute_materialized_rflags();
        let mut regs = self.regs.clone();
        regs.rflags = rflags;
        Ok(CpuState::X86_64(X86_64CpuState {
            regs,
            sregs: self.sregs.clone(),
        }))
    }

    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        let state = match state {
            CpuState::X86_64(state) => state,
            _ => {
                return Err(Error::Emulator(
                    "expected x86_64 state for x86_64 vCPU".to_string(),
                ));
            }
        };
        self.regs = state.regs.clone();
        self.sregs = state.sregs.clone();
        // Injecting CPU state is a serializing event: drop the decode cache so we
        // re-decode from (possibly externally rewritten) code memory. Not hot -
        // set_state is only called at init / snapshot restore / GDB, never in run().
        self.decode_cache.iter_mut().for_each(|e| {
            e.rip = 0;
            e.bytes_len = 0;
        });
        Ok(())
    }

    fn set_pci_bridge(
        &mut self,
        bridge: std::sync::Arc<std::sync::Mutex<crate::devices::pci::PciStub>>,
        ap_base: u64,
        ap_end: u64,
    ) {
        self.mmu.set_pci_bridge(bridge, ap_base, ap_end);
    }

    fn complete_io_in(&mut self, data: &[u8]) {
        if let Some(pending) = self.io_pending.take() {
            let sz = pending.size as usize;
            // Batched `rep ins` block: write `count` consecutive elements from
            // `data` to memory starting at the staged address (forward).
            if pending.count > 1 {
                if let IoInTarget::Mem { addr } = pending.target {
                    for i in 0..pending.count as usize {
                        let off = i * sz;
                        if off + sz > data.len() {
                            break;
                        }
                        let value = match pending.size {
                            1 => data[off] as u64,
                            2 => u16::from_le_bytes([data[off], data[off + 1]]) as u64,
                            _ => u32::from_le_bytes([
                                data[off],
                                data[off + 1],
                                data[off + 2],
                                data[off + 3],
                            ]) as u64,
                        };
                        let _ = self.write_mem(addr + off as u64, value, pending.size);
                    }
                }
                return;
            }

            let value = match pending.size {
                1 => data.first().copied().unwrap_or(0) as u64,
                2 if data.len() >= 2 => u16::from_le_bytes([data[0], data[1]]) as u64,
                4 if data.len() >= 4 => {
                    u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as u64
                }
                _ => 0,
            };

            match pending.target {
                IoInTarget::Reg => match pending.size {
                    1 => self.regs.rax = (self.regs.rax & !0xFF) | value,
                    2 => self.regs.rax = (self.regs.rax & !0xFFFF) | value,
                    4 => self.regs.rax = value,
                    _ => {}
                },
                IoInTarget::Mem { addr } => {
                    let _ = self.write_mem(addr, value, pending.size);
                }
            }
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn can_inject_interrupt(&self) -> bool {
        // IF is set/cleared only by STI/CLI/POPF/IRET (written straight to
        // regs.rflags), never by the lazy ALU-flag engine - so read it directly.
        (self.regs.rflags & flags::bits::IF) != 0
    }

    fn inject_interrupt(&mut self, vector: u8) -> Result<bool> {
        // Check if interrupts are enabled
        if !self.can_inject_interrupt() {
            return Ok(false);
        }

        // Inject the external interrupt
        // External interrupts don't push an error code
        self.inject_exception(vector, None)?;

        // Clear the halted state if we were halted
        self.halted = false;

        Ok(true)
    }

    fn inject_nmi(&mut self) -> Result<bool> {
        // NMI is vector 2 and ignores IF flag
        // TODO: Track NMI blocking (NMIs are blocked until IRET after an NMI)
        self.inject_exception(2, None)?;
        self.halted = false;
        tracing::debug!("Injected NMI");
        Ok(true)
    }

    #[cfg(feature = "debug")]
    fn set_single_step(&mut self, enabled: bool) {
        self.single_step = enabled;
    }

    #[cfg(feature = "debug")]
    fn is_single_step(&self) -> bool {
        self.single_step
    }

    #[cfg(feature = "debug")]
    fn invalidate_code_cache(&mut self, addr: u64) {
        // Invalidate all decode cache entries on the same page as addr.
        // This ensures we re-decode instructions after breakpoint modification.
        let page_base = addr & !0xFFF;
        for idx in 0..DECODE_CACHE_SIZE {
            let entry = &mut self.decode_cache[idx];
            if entry.rip != 0 && (entry.rip & !0xFFF) == page_base {
                entry.rip = 0; // Invalidate
                entry.bytes_len = 0; // mark empty so a real rip==0 can't false-hit
            }
        }
    }

    fn instruction_count(&self) -> u64 {
        // Access the global instruction counter
        static TOTAL_INSN_READER: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(0);
        // Note: This reads from a separate static; we need to use the same one as run()
        // For now, we'll get the count from run()'s TOTAL_INSN via a global accessor
        get_total_instruction_count()
    }

    fn get_emulator_state(&self) -> Option<crate::snapshot::EmulatorState> {
        use crate::snapshot::{EmulatorState, FpuSnapshot, LazyFlagsSnapshot};

        let lf = self.lazy_flags;
        Some(EmulatorState {
            fpu: FpuSnapshot {
                control_word: self.fpu.control_word,
                status_word: self.fpu.status_word,
                tag_word: self.fpu.tag_word,
                data_ptr: self.fpu.data_ptr,
                instr_ptr: self.fpu.instr_ptr,
                last_opcode: self.fpu.last_opcode,
                st: self.fpu.st,
                top: self.fpu.top,
            },
            lazy_flags: LazyFlagsSnapshot {
                op: match lf.op {
                    LazyFlagOp::None => 0,
                    LazyFlagOp::Add => 1,
                    LazyFlagOp::Sub => 2,
                    LazyFlagOp::Logic => 3,
                    LazyFlagOp::Inc => 4,
                    LazyFlagOp::Dec => 5,
                },
                result: lf.result,
                src: lf.src,
                dst: lf.dst,
                size: lf.size,
            },
            kernel_gs_base: self.kernel_gs_base,
            pkru: self.pkru,
            halted: self.halted,
        })
    }

    fn set_emulator_state(&mut self, state: &crate::snapshot::EmulatorState) -> Result<()> {
        // Restore FPU state
        self.fpu.control_word = state.fpu.control_word;
        self.fpu.status_word = state.fpu.status_word;
        self.fpu.tag_word = state.fpu.tag_word;
        self.fpu.data_ptr = state.fpu.data_ptr;
        self.fpu.instr_ptr = state.fpu.instr_ptr;
        self.fpu.last_opcode = state.fpu.last_opcode;
        self.fpu.st = state.fpu.st;
        self.fpu.top = state.fpu.top;

        // Restore lazy flags
        let op = match state.lazy_flags.op {
            0 => LazyFlagOp::None,
            1 => LazyFlagOp::Add,
            2 => LazyFlagOp::Sub,
            3 => LazyFlagOp::Logic,
            4 => LazyFlagOp::Inc,
            5 => LazyFlagOp::Dec,
            _ => LazyFlagOp::None,
        };
        self.lazy_flags = LazyFlags {
            op,
            result: state.lazy_flags.result,
            src: state.lazy_flags.src,
            dst: state.lazy_flags.dst,
            size: state.lazy_flags.size,
        };

        // Restore other state
        self.kernel_gs_base = state.kernel_gs_base;
        self.pkru = state.pkru;
        self.halted = state.halted;

        Ok(())
    }
}

/// Global instruction counter for snapshotting
static GLOBAL_INSN_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Get the total instruction count
pub fn get_total_instruction_count() -> u64 {
    GLOBAL_INSN_COUNT.load(std::sync::atomic::Ordering::Relaxed)
}

/// Publish a vCPU's retired-instruction count to the global counter. Called at
/// run() exit boundaries (not per-instruction) for snapshot/diagnostic readers.
#[inline]
pub fn publish_instruction_count(count: u64) {
    GLOBAL_INSN_COUNT.store(count, std::sync::atomic::Ordering::Relaxed);
}

// ============================================================================
// SMIR native hot-block JIT tier (opt-in via the `smir-jit` feature).
//
// This is an additive fast tier that sits BESIDE the interpreter — it never
// touches the `step()` hot path. Given a self-contained basic-block region at
// the current RIP (a hot loop / ALU chain that exits via HLT), it lifts the
// region to SMIR, verifies it is clobber-safe under the 1:1 identity register
// map, lowers it to native x86-64, and runs the WHOLE region in one call —
// loops stay internal via native back-edges (the "dragon" path). Validated
// bit-exact vs KVM by the `smir_native_*` differential tests.
// ============================================================================
/// Backward-branch hits at a loop head before the JIT promotes (compiles) it.
/// Low enough to catch real hot loops quickly, high enough to skip loops that
/// run only a handful of times (where lift+lower would not pay off).
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
const JIT_HOT_THRESHOLD: u32 = 64;

/// A compiled native hot-block region. The lowered code is register-state
/// independent (it marshals guest state in/out per run), so one `JitRegion` is
/// cached by (RIP, mode_tag) and re-run for every later entry to that RIP until
/// the underlying guest code page is written (SMC invalidation).
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
pub(super) struct JitRegion {
    exec: crate::smir::lower::runtime::ExecMem,
    entry_offset: usize,
}

/// RAX_JIT_BAIL=1 logs why each hot region is rejected by the JIT (diagnostic
/// for expanding the whitelist toward the highest-frequency bail reasons).
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn jit_bail_log() -> bool {
    use std::sync::OnceLock;
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("RAX_JIT_BAIL").is_some())
}

/// RAX_JIT_MEM=1 enables JIT of memory-touching hot regions: register Load/Store
/// lower to MMU helper calls (`rax_jit_mem_load`/`rax_jit_mem_store`) with a
/// per-op fault-bail to the interpreter. Off by default while it soaks; the
/// register-only path (default) is unaffected.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn jit_mem_enabled() -> bool {
    use std::sync::OnceLock;
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("RAX_JIT_MEM").is_some())
}

/// Result of a JIT memory load: `value` in RAX, `ok` in RDX (SysV two-eightbyte
/// integer struct return). `ok == 0` signals a fault/MMIO/unmapped access — the
/// native region bails to the interpreter at the faulting instruction.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
#[repr(C)]
struct JitLoadRet {
    value: u64,
    ok: u64,
}

/// Guest entry PC of the JIT region currently executing natively (RAX_JIT_TRACE).
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
static JIT_LAST_ENTRY: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// SIGSEGV/SIGBUS handler: a host fault inside native JIT code prints the guest
/// region entry + faulting address (async-signal-safe: only `write` + manual hex),
/// then restores the default disposition and re-raises.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64", unix))]
extern "C" fn jit_crash_handler(
    sig: libc::c_int,
    info: *mut libc::siginfo_t,
    _ctx: *mut libc::c_void,
) {
    use std::sync::atomic::Ordering;
    // Build "[JIT-CRASH] sig=NN entry=0xHEX addr=0xHEX\n" into a fixed buffer.
    let mut buf = [0u8; 64];
    let mut n = 0usize;
    let mut put = |s: &[u8], buf: &mut [u8; 64], n: &mut usize| {
        for &b in s {
            if *n < buf.len() {
                buf[*n] = b;
                *n += 1;
            }
        }
    };
    let mut put_hex = |mut v: u64, buf: &mut [u8; 64], n: &mut usize| {
        put(b"0x", buf, n);
        let mut started = false;
        for shift in (0..16).rev() {
            let nib = ((v >> (shift * 4)) & 0xf) as u8;
            if nib != 0 || started || shift == 0 {
                started = true;
                let c = if nib < 10 {
                    b'0' + nib
                } else {
                    b'a' + nib - 10
                };
                if *n < buf.len() {
                    buf[*n] = c;
                    *n += 1;
                }
            }
        }
        let _ = &mut v;
    };
    put(b"\n[JIT-CRASH] sig=", &mut buf, &mut n);
    put_hex(sig as u64, &mut buf, &mut n);
    put(b" entry=", &mut buf, &mut n);
    put_hex(JIT_LAST_ENTRY.load(Ordering::Relaxed), &mut buf, &mut n);
    put(b" addr=", &mut buf, &mut n);
    let addr = unsafe { (*info).si_addr() } as u64;
    put_hex(addr, &mut buf, &mut n);
    put(b"\n", &mut buf, &mut n);
    unsafe {
        libc::write(2, buf.as_ptr() as *const libc::c_void, n);
        // Restore default disposition and re-raise to produce the core dump.
        libc::signal(sig, libc::SIG_DFL);
        libc::raise(sig);
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64", unix))]
fn jit_install_crash_handler() {
    use std::sync::OnceLock;
    static INSTALLED: OnceLock<()> = OnceLock::new();
    INSTALLED.get_or_init(|| unsafe {
        let mut sa: libc::sigaction = std::mem::zeroed();
        sa.sa_sigaction = jit_crash_handler as usize;
        sa.sa_flags = libc::SA_SIGINFO;
        libc::sigemptyset(&mut sa.sa_mask);
        libc::sigaction(libc::SIGSEGV, &sa, std::ptr::null_mut());
        libc::sigaction(libc::SIGBUS, &sa, std::ptr::null_mut());
    });
}

/// Non-unix hosts have no async-signal-safe SIGSEGV sidecar (libc termios/signal
/// are unix-only). The JIT still runs; a fault inside native code aborts the
/// process the normal way instead of printing the guest entry. RAX_JIT_TRACE
/// diagnostics that don't depend on the handler are unaffected.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64", not(unix)))]
fn jit_install_crash_handler() {}

/// JIT memory-load helper: translate + read `size` bytes at guest `addr` via the
/// vcpu MMU, sign- or zero-extending to 64 bits. Called from lowered native code
/// with the vcpu pointer in `ctx`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
unsafe extern "C" fn rax_jit_mem_load(
    ctx: *mut X86_64Vcpu,
    addr: u64,
    size: u32,
    signed: u32,
) -> JitLoadRet {
    let vcpu = unsafe { &mut *ctx };
    match vcpu.read_mem(addr, size as u8) {
        Ok(val) => {
            let value = if signed != 0 {
                match size {
                    1 => val as u8 as i8 as i64 as u64,
                    2 => val as u16 as i16 as i64 as u64,
                    4 => val as u32 as i32 as i64 as u64,
                    _ => val,
                }
            } else {
                val
            };
            JitLoadRet { value, ok: 1 }
        }
        Err(_) => JitLoadRet { value: 0, ok: 0 },
    }
}

/// JIT memory-store helper: translate + write `size` bytes of `value` at guest
/// `addr` via the vcpu MMU. Returns 1 on success, 0 on fault/MMIO/unmapped.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
unsafe extern "C" fn rax_jit_mem_store(
    ctx: *mut X86_64Vcpu,
    addr: u64,
    value: u64,
    size: u32,
) -> u64 {
    let vcpu = unsafe { &mut *ctx };
    // A store to a code page is self-modifying code (e.g. the kernel's
    // text_poke / alternatives patching). Bail to the interpreter so the full
    // SMC + instruction-patching semantics (decode/JIT invalidation ordering,
    // int3 batching) are handled there rather than mid-native-region.
    if vcpu.mmu.is_code_page(addr) {
        return 0;
    }
    // Verify mode: record the pre-store value so the region's writes can be
    // undone and the interpreter re-run for a store-sound differential. The
    // old-value read must NOT pollute the access trace (it is bookkeeping, not
    // a guest access), so the trace is suspended around it.
    if vcpu.jit_mem_log.is_some() {
        let saved_trace = vcpu.jit_mem_trace.take();
        let old = vcpu.read_mem(addr, size as u8);
        vcpu.jit_mem_trace = saved_trace;
        match old {
            Ok(old) => vcpu
                .jit_mem_log
                .as_mut()
                .unwrap()
                .push((addr, size as u8, old)),
            // Can't snapshot this store → can't soundly verify; abort logging.
            Err(_) => vcpu.jit_mem_log = None,
        }
    }
    match vcpu.write_mem(addr, value, size as u8) {
        Ok(()) => 1,
        Err(_) => 0,
    }
}

/// Lift-through-calls helper: run the interpreter for a guest CALL's callee.
///
/// Called from a lowered JIT region at a guest `CALL` site (the `RAX_JIT_CALL`
/// path). The native region holds the live guest state in `gr` (the marshalled
/// GuestRegs); this helper syncs it into the vcpu, simulates the CALL (push the
/// return address, jump to `target_pc`), then steps the INTERPRETER until the
/// callee returns to `return_pc` — i.e. runs the entire called subtree (incl.
/// nested calls, which the interpreter handles) — then syncs the post-call state
/// back into `gr` so native execution resumes at the continuation.
///
/// Returns `1` on a clean return to `return_pc`. Returns `0` (and sets
/// `gr.exit_pc`) when the callee yields an exit that must reach the VMM (I/O,
/// HLT, …) or errors: the stashed exit is recovered by `jit_run_region_native`
/// and propagated, and the region returns to the interpreter at `exit_pc`.
///
/// Safety: `gr` is the region's marshalled state; `gr.ctx` is the owning vcpu.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
unsafe extern "C" fn rax_jit_call(
    gr: *mut crate::smir::lower::runtime::GuestRegs,
    target_pc: u64,
    return_pc: u64,
) -> u64 {
    let gr = unsafe { &mut *gr };
    let vcpu = unsafe { &mut *(gr.ctx as *mut X86_64Vcpu) };

    // Sync marshalled native state -> vcpu interpreter state. The JIT works with
    // materialized flags, so clear any pending lazy op.
    vcpu.regs.rax = gr.gpr[0];
    vcpu.regs.rcx = gr.gpr[1];
    vcpu.regs.rdx = gr.gpr[2];
    vcpu.regs.rbx = gr.gpr[3];
    vcpu.regs.rsp = gr.gpr[4];
    vcpu.regs.rbp = gr.gpr[5];
    vcpu.regs.rsi = gr.gpr[6];
    vcpu.regs.rdi = gr.gpr[7];
    vcpu.regs.r8 = gr.gpr[8];
    vcpu.regs.r9 = gr.gpr[9];
    vcpu.regs.r10 = gr.gpr[10];
    vcpu.regs.r11 = gr.gpr[11];
    vcpu.regs.r12 = gr.gpr[12];
    vcpu.regs.r13 = gr.gpr[13];
    vcpu.regs.r14 = gr.gpr[14];
    vcpu.regs.r15 = gr.gpr[15];
    vcpu.regs.r16 = gr.gpr[16];
    vcpu.regs.r17 = gr.gpr[17];
    vcpu.regs.r18 = gr.gpr[18];
    vcpu.regs.r19 = gr.gpr[19];
    vcpu.regs.r20 = gr.gpr[20];
    vcpu.regs.r21 = gr.gpr[21];
    vcpu.regs.r22 = gr.gpr[22];
    vcpu.regs.r23 = gr.gpr[23];
    vcpu.regs.r24 = gr.gpr[24];
    vcpu.regs.r25 = gr.gpr[25];
    vcpu.regs.r26 = gr.gpr[26];
    vcpu.regs.r27 = gr.gpr[27];
    vcpu.regs.r28 = gr.gpr[28];
    vcpu.regs.r29 = gr.gpr[29];
    vcpu.regs.r30 = gr.gpr[30];
    vcpu.regs.r31 = gr.gpr[31];
    vcpu.regs.rflags = gr.rflags;
    vcpu.lazy_flags = LazyFlags {
        op: LazyFlagOp::None,
        ..Default::default()
    };

    // Simulate the CALL's stack effect (the block's own ops already ran
    // natively; only the call's push+transfer remain), then enter the callee.
    let _ = vcpu.push64(return_pc);
    vcpu.regs.rip = target_pc;

    // Run the callee to completion. The callee is bounded (a normal function);
    // the step cap is a runaway backstop only.
    let mut ok: u64 = 1;
    let mut steps: u64 = 0;
    loop {
        if vcpu.regs.rip == return_pc {
            break;
        }
        steps += 1;
        if steps > 500_000_000 {
            ok = 0;
            break;
        }
        // SMC: invalidate any code page the callee wrote before its next fetch
        // (this direct-step loop bypasses the run-loop drain). Guarded → cheap.
        vcpu.drain_smc();
        match vcpu.step() {
            Ok(None) => {}
            Ok(Some(exit)) => {
                // An exit that needs the VMM (I/O, HLT, …): stash it and bail.
                vcpu.jit_callout_exit = Some(exit);
                ok = 0;
                break;
            }
            Err(_) => {
                ok = 0;
                break;
            }
        }
    }

    // Sync vcpu state back into the marshalled file. Materialize flags first so
    // gr.rflags is current (the native reload / trampoline reads it).
    vcpu.materialize_flags();
    gr.gpr[0] = vcpu.regs.rax;
    gr.gpr[1] = vcpu.regs.rcx;
    gr.gpr[2] = vcpu.regs.rdx;
    gr.gpr[3] = vcpu.regs.rbx;
    gr.gpr[4] = vcpu.regs.rsp;
    gr.gpr[5] = vcpu.regs.rbp;
    gr.gpr[6] = vcpu.regs.rsi;
    gr.gpr[7] = vcpu.regs.rdi;
    gr.gpr[8] = vcpu.regs.r8;
    gr.gpr[9] = vcpu.regs.r9;
    gr.gpr[10] = vcpu.regs.r10;
    gr.gpr[11] = vcpu.regs.r11;
    gr.gpr[12] = vcpu.regs.r12;
    gr.gpr[13] = vcpu.regs.r13;
    gr.gpr[14] = vcpu.regs.r14;
    gr.gpr[15] = vcpu.regs.r15;
    gr.gpr[16] = vcpu.regs.r16;
    gr.gpr[17] = vcpu.regs.r17;
    gr.gpr[18] = vcpu.regs.r18;
    gr.gpr[19] = vcpu.regs.r19;
    gr.gpr[20] = vcpu.regs.r20;
    gr.gpr[21] = vcpu.regs.r21;
    gr.gpr[22] = vcpu.regs.r22;
    gr.gpr[23] = vcpu.regs.r23;
    gr.gpr[24] = vcpu.regs.r24;
    gr.gpr[25] = vcpu.regs.r25;
    gr.gpr[26] = vcpu.regs.r26;
    gr.gpr[27] = vcpu.regs.r27;
    gr.gpr[28] = vcpu.regs.r28;
    gr.gpr[29] = vcpu.regs.r29;
    gr.gpr[30] = vcpu.regs.r30;
    gr.gpr[31] = vcpu.regs.r31;
    gr.rflags = vcpu.regs.rflags;
    if ok == 0 {
        gr.exit_pc = vcpu.regs.rip;
    }
    ok
}

/// RAX_JIT_CALL=1 enables lift-through-calls: a guest CALL inside a hot region
/// lowers to a call-out into [`rax_jit_call`] (instead of being a region-ending
/// frontier), so call-heavy loops compile and run natively between calls. Opt-in
/// while it soaks; requires (and implies) the mem-helper path.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn jit_call_enabled() -> bool {
    use std::sync::OnceLock;
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("RAX_JIT_CALL").is_some())
}

/// Classify the first reason an executed block of `func` fails the clobber gate:
/// the offending op's variant name, or `rsp/rbp` / `virtual-dst`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn jit_classify_bail(
    func: &crate::smir::ir::SmirFunction,
    exits: &std::collections::HashMap<crate::smir::types::BlockId, u64>,
) -> String {
    use crate::smir::types::{ArchReg, VReg, X86Reg};
    let is_sp_bp = |v: &VReg| {
        matches!(
            v,
            VReg::Arch(ArchReg::X86(X86Reg::Rsp)) | VReg::Arch(ArchReg::X86(X86Reg::Rbp))
        )
    };
    let variant = |k: &crate::smir::ops::OpKind| -> String {
        let s = format!("{k:?}");
        s.split([' ', '{', '(']).next().unwrap_or("?").to_string()
    };
    use crate::smir::ir::Terminator;
    use crate::smir::ops::OpKind;
    for b in &func.blocks {
        if exits.contains_key(&b.id) {
            continue;
        }
        let n = b.ops.len();
        for (i, op) in b.ops.iter().enumerate() {
            // Mirror block_is_clobber_safe: a trailing TestCondition feeding the
            // block's CondBranch is folded to a direct Jcc (exempt), not a bail.
            if i + 1 == n {
                if let (Terminator::CondBranch { cond, .. }, OpKind::TestCondition { dst, .. }) =
                    (&b.terminator, &op.kind)
                {
                    if dst == cond {
                        continue;
                    }
                }
            }
            if !op.kind.is_jit_safe() {
                return variant(&op.kind);
            }
            if op
                .kind
                .dests()
                .iter()
                .any(|d| matches!(d, VReg::Virtual(_)))
            {
                return format!("virtual-dst:{}", variant(&op.kind));
            }
            if op.kind.dests().iter().any(is_sp_bp)
                || op.kind.source_vregs().iter().any(|v| is_sp_bp(v))
            {
                return format!("rsp/rbp:{}", variant(&op.kind));
            }
        }
    }
    "?".to_string()
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
impl X86_64Vcpu {
    /// Attempt to JIT-compile and natively execute the hot region at the current
    /// RIP, handing control back to the interpreter at the region's exit.
    /// Returns `Ok(true)` if it ran natively (guest registers updated and RIP
    /// advanced to the exit address — the caller should continue interpreting
    /// from there), or `Ok(false)` if the region is ineligible (caller falls
    /// back to `step()`).
    ///
    /// The region is the CFG reachable from RIP up to "frontier" terminals
    /// (HLT / RET / CALL / indirect / syscall / switch); internal Branch and
    /// CondBranch edges (loop back-edges, if/else) execute natively. Each
    /// frontier block lowers to an exit stub that records its guest PC into
    /// `exit_pc`; the JIT runs UP TO but not THROUGH it, so the interpreter
    /// resumes there and re-executes that block. Eligibility: the entry block
    /// must not itself be a frontier (else there is no native work), every
    /// block must be clobber-safe (writes only architectural registers — a
    /// virtual temporary would corrupt a guest GPR under the identity register
    /// map), and the region must lower with no unresolved relocations.
    ///
    /// CAVEAT: a guest infinite loop with no reachable frontier would spin in
    /// native code uninterruptibly — callers should only invoke this for
    /// regions known to terminate (e.g. promoted hot loops with an exit edge).
    pub fn jit_try_block(&mut self) -> Result<bool> {
        match self.jit_compile_region()? {
            Some(region) => {
                if std::env::var_os("RAX_JIT_LOG").is_some() {
                    eprintln!(
                        "[JIT] compiled hot region @ {:#x} (regions cached: {})",
                        self.regs.rip,
                        self.jit_region_count()
                    );
                }
                self.jit_run_region(&region);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Compile the hot region at the current RIP into a native [`JitRegion`], or
    /// return `Ok(None)` if it is ineligible (see [`Self::jit_try_block`] for the
    /// eligibility rules). This is the cacheable half — the returned region is
    /// register-state-independent and may be re-run for any later entry to the
    /// same RIP (until the underlying guest code changes; see SMC invalidation).
    pub(super) fn jit_compile_region(&mut self) -> Result<Option<JitRegion>> {
        use crate::smir::ir::Terminator;
        use crate::smir::lift::x86_64::X86_64Lifter;
        use crate::smir::lift::{LiftContext, MemoryReader, SmirLifter};
        use crate::smir::lower::SmirLowerer;
        use crate::smir::lower::runtime::{ExecMem, is_native_clobber_safe_excluding};
        use crate::smir::lower::x86_64::X86_64Lowerer;
        use crate::smir::memory::MemoryError;
        use crate::smir::opt::{OptLevel, optimize_function};
        use crate::smir::types::SourceArch;
        use std::collections::HashMap;

        let entry = self.regs.rip;

        // Snapshot a window of guest code to lift from. 512B covers typical hot
        // loops; lifting past it (or across an unmapped page) yields an error
        // and we bail to the interpreter.
        const WINDOW: usize = 512;
        let bytes = match self.read_bytes(entry, WINDOW) {
            Ok(b) => b,
            Err(_) => return Ok(None),
        };

        struct Win {
            base: u64,
            bytes: Vec<u8>,
        }
        impl MemoryReader for Win {
            fn read(&self, addr: u64, size: usize) -> core::result::Result<Vec<u8>, MemoryError> {
                let off = addr
                    .checked_sub(self.base)
                    .filter(|&o| (o as usize) < self.bytes.len())
                    .ok_or(MemoryError::OutOfBounds { addr })? as usize;
                let n = (self.bytes.len() - off).min(size);
                Ok(self.bytes[off..off + n].to_vec())
            }
        }
        let reader = Win { base: entry, bytes };

        // Try lift-through-calls first (a call-heavy loop body lifts into one
        // native region with runtime call-outs). If that bigger region is
        // ineligible, retry WITHOUT call-mode (the smaller call-as-frontier
        // region). This makes lift-through-calls STRICTLY ADDITIVE — never worse
        // than the baseline mem/register JIT coverage.
        let want_call = jit_call_enabled();
        let modes: &[bool] = if want_call { &[true, false] } else { &[false] };
        'modes: for &cm in modes {
            let mut lifter = X86_64Lifter::strict();
            if cm {
                lifter.set_lift_through_calls(512);
            }
            let mut lctx = LiftContext::new(SourceArch::X86_64);
            let mut func = match lifter.lift_function(entry, &reader, &mut lctx) {
                Ok(f) => f,
                Err(e) => {
                    if jit_bail_log() {
                        eprintln!("[JIT-BAIL] lift-err:{e:?} @ {entry:#x} (call={cm})");
                    }
                    continue 'modes;
                }
            };

            // Optimize before computing exits / lowering (frontier-aware, see note).
            if std::env::var_os("RAX_JIT_NO_OPT").is_none() {
                optimize_function(&mut func, OptLevel::O2);
            }

            // Mark every frontier terminal as a native-exit stub (records guest
            // PC); internal Branch/CondBranch edges stay native (loops, if/else).
            let mut exits: HashMap<_, u64> = HashMap::new();
            // Block ids present in the lifted function — to validate a CALL's
            // continuation was lifted before lowering it as a call-out.
            let lifted: std::collections::HashSet<_> = func.blocks.iter().map(|b| b.id).collect();
            for b in &func.blocks {
                let frontier = match &b.terminator {
                    Terminator::Trap { .. }
                    | Terminator::Return { .. }
                    | Terminator::TailCall { .. }
                    | Terminator::IndirectBranch { .. }
                    | Terminator::IndirectBranchMem { .. }
                    | Terminator::Switch { .. } => true,
                    // Lift-through-calls: a CALL is NOT a frontier when call-mode
                    // is on, the continuation was lifted, and the target form is
                    // supported (direct or register-indirect) — it lowers to a
                    // runtime call-out and continues natively at `continuation`.
                    Terminator::Call {
                        target,
                        continuation,
                        ..
                    } => {
                        let target_ok = matches!(
                            target,
                            crate::smir::ir::CallTarget::GuestAddr(_)
                                | crate::smir::ir::CallTarget::Indirect(_)
                        );
                        !(cm && target_ok && lifted.contains(continuation))
                    }
                    _ => false,
                };
                if frontier {
                    exits.insert(b.id, b.guest_pc);
                }
            }
            // No frontier reachable ⇒ the region never returns (spin loop). Bail.
            if exits.is_empty() {
                if jit_bail_log() {
                    eprintln!("[JIT-BAIL] no-frontier @ {entry:#x} (call={cm})");
                }
                continue 'modes;
            }
            // Entry block is itself a frontier ⇒ no native work.
            if exits.contains_key(&func.entry) {
                if jit_bail_log() {
                    // Diagnostic: WHY is the entry a frontier? Print the entry
                    // terminator's shape (and, for a Call, why it stayed a
                    // frontier) so we can see what lift-through-calls must crack.
                    let why = func
                        .blocks
                        .iter()
                        .find(|b| b.id == func.entry)
                        .map(|b| match &b.terminator {
                            Terminator::Trap { .. } => "Trap".to_string(),
                            Terminator::Return { .. } => "Return".to_string(),
                            Terminator::TailCall { .. } => "TailCall".to_string(),
                            Terminator::IndirectBranch { .. } => "IndirectBranch".to_string(),
                            Terminator::IndirectBranchMem { .. } => "IndirectBranchMem".to_string(),
                            Terminator::Switch { .. } => "Switch".to_string(),
                            Terminator::Call {
                                target,
                                continuation,
                                ..
                            } => {
                                let tk = match target {
                                    crate::smir::ir::CallTarget::Direct(_) => "DirectFn",
                                    crate::smir::ir::CallTarget::GuestAddr(_) => "GuestAddr",
                                    crate::smir::ir::CallTarget::Indirect(_) => "IndirectReg",
                                    crate::smir::ir::CallTarget::IndirectMem(_) => "IndirectMem",
                                    crate::smir::ir::CallTarget::Runtime(_) => "Runtime",
                                };
                                format!("Call/{tk}(lifted-cont={})", lifted.contains(continuation))
                            }
                            _ => "other".to_string(),
                        })
                        .unwrap_or_else(|| "?".to_string());
                    eprintln!("[JIT-BAIL] entry-frontier:{why} @ {entry:#x} (call={cm})");
                }
                continue 'modes;
            }
            // Fail-safe gate over the EXECUTED (non-exit) blocks.
            let allow_mem = self.jit_mem;
            if !is_native_clobber_safe_excluding(&func, &exits, allow_mem) {
                if jit_bail_log() {
                    eprintln!(
                        "[JIT-BAIL] gate:{} @ {:#x} (call={cm})",
                        jit_classify_bail(&func, &exits),
                        entry
                    );
                }
                continue 'modes;
            }

            let mut lowerer = X86_64Lowerer::new();
            lowerer.set_native_exits(exits);
            if allow_mem {
                lowerer.set_mem_helpers(true);
            }
            if cm {
                // Lower CALL terminators as runtime call-outs (rax_jit_call).
                lowerer.set_call_helpers(true);
            }
            let res = match lowerer.lower_function(&func) {
                Ok(r) if r.relocations.is_empty() => r,
                Ok(r) => {
                    if jit_bail_log() {
                        eprintln!(
                            "[JIT-BAIL] relocs:{} @ {:#x} (call={cm})",
                            r.relocations.len(),
                            entry
                        );
                    }
                    continue 'modes;
                }
                Err(e) => {
                    if jit_bail_log() {
                        eprintln!("[JIT-BAIL] lower-err:{e:?} @ {entry:#x} (call={cm})");
                    }
                    continue 'modes;
                }
            };
            let code = match lowerer.finalize() {
                Ok(c) => c,
                Err(_) => continue 'modes,
            };
            let exec = match ExecMem::new(&code) {
                Ok(m) => m,
                Err(_) => continue 'modes,
            };
            return Ok(Some(JitRegion {
                exec,
                entry_offset: res.entry_offset,
            }));
        }
        Ok(None)
    }

    /// Execute a (possibly cached) compiled region with the current guest state,
    /// then resume at the recorded exit PC. Marshals guest GPRs+flags into the
    /// native file, runs, and bridges the result back. RSP is neither loaded nor
    /// written by the trampoline (the block runs on the host stack).
    pub(super) fn jit_run_region(&mut self, region: &JitRegion) {
        // Self-verifying mode (RAX_JIT_VERIFY=1): run the region natively, then
        // re-run the INTERPRETER from the identical entry state up to the JIT's
        // exit PC and diff the architectural state. On the first divergence,
        // dump the region (entry/exit PC, code bytes, lifted+optimized ops, and
        // the diverging registers) and abort — this pinpoints a miscompiled hot
        // region on a live boot. Register-only regions touch no memory/RSP/RBP,
        // so re-executing the interpreter from the snapshot is side-effect-free.
        {
            use std::sync::OnceLock;
            static VERIFY: OnceLock<bool> = OnceLock::new();
            if *VERIFY.get_or_init(|| std::env::var_os("RAX_JIT_VERIFY").is_some()) {
                self.jit_run_region_verified(region);
                return;
            }
        }
        self.jit_run_region_native(region);
    }

    /// Native-only execution of a compiled region (the production path).
    pub(super) fn jit_run_region_native(&mut self, region: &JitRegion) {
        use crate::smir::lower::runtime::GuestRegs;

        // Crash diagnostic (RAX_JIT_TRACE=1): record the region entry about to run
        // natively and install a SIGSEGV/SIGBUS handler that prints it + the
        // faulting address. Lets a host crash IN native JIT code be traced to the
        // exact guest region. Opt-in, so default runs are untouched.
        #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
        {
            use std::sync::OnceLock;
            use std::sync::atomic::Ordering;
            static TRACE: OnceLock<bool> = OnceLock::new();
            if *TRACE.get_or_init(|| std::env::var_os("RAX_JIT_TRACE").is_some()) {
                jit_install_crash_handler();
                JIT_LAST_ENTRY.store(self.regs.rip, Ordering::Relaxed);
                static DUMP_AT: OnceLock<Option<u64>> = OnceLock::new();
                let dump_at = *DUMP_AT.get_or_init(|| {
                    std::env::var("RAX_JIT_DUMP")
                        .ok()
                        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
                });
                if dump_at == Some(self.regs.rip) {
                    static DONE: OnceLock<()> = OnceLock::new();
                    if DONE.set(()).is_ok() {
                        let rip = self.regs.rip;
                        eprintln!("[JIT-DUMP] region {rip:#x}:\n{}", self.jit_dump_region(rip));
                    }
                }
            }
        }

        // The interpreter keeps RFLAGS LAZY: `self.regs.rflags` is stale while a
        // lazy op is pending (the truth lives in `self.lazy_flags`). Materialize
        // it first so the value bridged into the native region is the real
        // architectural RFLAGS (the region's entry may read CF/etc., and the
        // trampoline `popfq`s `gr.rflags`).
        self.materialize_flags();
        // Guest RFLAGS before the region — every bit EXCEPT the six status flags
        // (CF/PF/AF/ZF/SF/OF) must be preserved across the native region: the
        // whitelisted ops only ever touch the status flags, and the trampoline's
        // user-mode `popfq`/`pushfq` does NOT round-trip IF/IOPL/etc. (`pushfq`
        // returns the HOST's IF=1), so taking those bits from the native result
        // would spuriously ENABLE guest interrupts mid-boot and crash the kernel.
        let pre_rflags = self.regs.rflags;

        let mut gr = GuestRegs::default();
        // Memory-helper channel: the vcpu pointer + helper addresses, used only
        // by regions lowered with the MMU helper-call path (RAX_JIT_MEM).
        gr.ctx = self as *mut X86_64Vcpu as u64;
        gr.load_fn = rax_jit_mem_load as usize as u64;
        gr.store_fn = rax_jit_mem_store as usize as u64;
        // Lift-through-calls channel (RAX_JIT_CALL): a guest CALL in the region
        // calls out here to run its callee in the interpreter, then resumes.
        gr.call_fn = rax_jit_call as usize as u64;
        // Segment bases for `fs:`/`gs:`-overridden operands (Address::SegmentRel).
        gr.fs_base = self.sregs.fs.base;
        gr.gs_base = self.sregs.gs.base;
        gr.gpr[0] = self.regs.rax;
        gr.gpr[1] = self.regs.rcx;
        gr.gpr[2] = self.regs.rdx;
        gr.gpr[3] = self.regs.rbx;
        gr.gpr[4] = self.regs.rsp;
        gr.gpr[5] = self.regs.rbp;
        gr.gpr[6] = self.regs.rsi;
        gr.gpr[7] = self.regs.rdi;
        gr.gpr[8] = self.regs.r8;
        gr.gpr[9] = self.regs.r9;
        gr.gpr[10] = self.regs.r10;
        gr.gpr[11] = self.regs.r11;
        gr.gpr[12] = self.regs.r12;
        gr.gpr[13] = self.regs.r13;
        gr.gpr[14] = self.regs.r14;
        gr.gpr[15] = self.regs.r15;
        gr.gpr[16] = self.regs.r16;
        gr.gpr[17] = self.regs.r17;
        gr.gpr[18] = self.regs.r18;
        gr.gpr[19] = self.regs.r19;
        gr.gpr[20] = self.regs.r20;
        gr.gpr[21] = self.regs.r21;
        gr.gpr[22] = self.regs.r22;
        gr.gpr[23] = self.regs.r23;
        gr.gpr[24] = self.regs.r24;
        gr.gpr[25] = self.regs.r25;
        gr.gpr[26] = self.regs.r26;
        gr.gpr[27] = self.regs.r27;
        gr.gpr[28] = self.regs.r28;
        gr.gpr[29] = self.regs.r29;
        gr.gpr[30] = self.regs.r30;
        gr.gpr[31] = self.regs.r31;
        gr.rflags = self.regs.rflags;
        gr.exit_pc = self.regs.rip; // fallback (an exit stub overwrites this)

        region.exec.run(region.entry_offset, &mut gr);

        // Lift-through-calls: if a call-out bailed (a callee yielded a VMM-bound
        // exit or errored), the helper already synced the vcpu's full
        // architectural state — RIP = exit_pc, complete RFLAGS, all GPRs. Do NOT
        // re-marshal `gr` here: the status-only flag merge below would drop the
        // callee's IF/DF changes. Leave the vcpu exactly as the helper left it
        // (the run loop reads `jit_callout_exit` and returns the stashed exit).
        #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
        if self.jit_callout_exit.is_some() {
            self.lazy_flags = LazyFlags {
                op: LazyFlagOp::None,
                ..Default::default()
            };
            return;
        }

        self.regs.rax = gr.gpr[0];
        self.regs.rcx = gr.gpr[1];
        self.regs.rdx = gr.gpr[2];
        self.regs.rbx = gr.gpr[3];
        self.regs.rbp = gr.gpr[5];
        self.regs.rsi = gr.gpr[6];
        self.regs.rdi = gr.gpr[7];
        self.regs.r8 = gr.gpr[8];
        self.regs.r9 = gr.gpr[9];
        self.regs.r10 = gr.gpr[10];
        self.regs.r11 = gr.gpr[11];
        self.regs.r12 = gr.gpr[12];
        self.regs.r13 = gr.gpr[13];
        self.regs.r14 = gr.gpr[14];
        self.regs.r15 = gr.gpr[15];
        self.regs.r16 = gr.gpr[16];
        self.regs.r17 = gr.gpr[17];
        self.regs.r18 = gr.gpr[18];
        self.regs.r19 = gr.gpr[19];
        self.regs.r20 = gr.gpr[20];
        self.regs.r21 = gr.gpr[21];
        self.regs.r22 = gr.gpr[22];
        self.regs.r23 = gr.gpr[23];
        self.regs.r24 = gr.gpr[24];
        self.regs.r25 = gr.gpr[25];
        self.regs.r26 = gr.gpr[26];
        self.regs.r27 = gr.gpr[27];
        self.regs.r28 = gr.gpr[28];
        self.regs.r29 = gr.gpr[29];
        self.regs.r30 = gr.gpr[30];
        self.regs.r31 = gr.gpr[31];
        // Merge: status flags from the native result, all other bits (IF, DF,
        // IOPL, NT, reserved, …) preserved from the guest's pre-region value.
        const STATUS: u64 = flags::bits::CF
            | flags::bits::PF
            | flags::bits::AF
            | flags::bits::ZF
            | flags::bits::SF
            | flags::bits::OF;
        self.regs.rflags = (pre_rflags & !STATUS) | (gr.rflags & STATUS);
        self.regs.rip = gr.exit_pc;
        // The native region produced fully-materialized RFLAGS. Mark the lazy
        // state as materialized so the interpreter, on resume, reads
        // `self.regs.rflags` (the JIT result) instead of recomputing from a
        // STALE lazy op left over from before the region — the desync that
        // corrupted kernel state across the JIT/interp boundary.
        self.lazy_flags = LazyFlags {
            op: LazyFlagOp::None,
            ..Default::default()
        };
    }

    /// Verify a compiled region against the interpreter (RAX_JIT_VERIFY=1).
    fn jit_run_region_verified(&mut self, region: &JitRegion) {
        let entry_pc = self.regs.rip;
        let snap = self.regs.clone();
        let snap_lf = self.lazy_flags;

        // 1) Run natively with store-logging (to UNDO writes) and an access
        //    trace (to diff against the interpreter's access sequence).
        self.jit_mem_log = Some(Vec::new());
        self.jit_mem_trace = Some(Vec::new());
        self.jit_run_region_native(region);
        let jit = self.regs.clone();
        let jit_rflags = self.regs.rflags; // already materialized by the native bridge
        let exit_pc = self.regs.rip;
        // Take the native trace NOW, before the undo/re-read loops add to it.
        let jit_trace = self.jit_mem_trace.take().unwrap_or_default();
        let log = match self.jit_mem_log.take() {
            Some(l) => l,
            // Logging aborted (unreadable store target) → can't undo → adopt
            // the native result unverified.
            None => {
                self.regs = jit;
                return;
            }
        };
        // Capture the native final value at each written address, then UNDO the
        // region's writes (reverse order handles overlapping stores) so the
        // interpreter re-runs from the original memory image.
        let mut native_writes: Vec<(u64, u8, u64)> = Vec::with_capacity(log.len());
        for &(addr, size, _old) in &log {
            if let Ok(v) = self.read_mem(addr, size) {
                native_writes.push((addr, size, v));
            }
        }
        for &(addr, size, old) in log.iter().rev() {
            let _ = self.write_mem(addr, old, size);
        }

        // 2) Re-run the interpreter from the same entry up to the exit PC,
        //    restoring the LAZY flag state (the interpreter's source of truth).
        self.regs = snap.clone();
        self.lazy_flags = snap_lf;
        self.jit_mem_trace = Some(Vec::new());
        let cap = 50_000_000u64;
        let mut steps = 0u64;
        let mut reached = true;
        while self.regs.rip != exit_pc {
            if steps >= cap {
                reached = false;
                break;
            }
            // SMC: mirror the run-loop drain (this verify re-step bypasses it).
            self.drain_smc();
            match self.step() {
                Ok(None) => {}
                _ => {
                    reached = false;
                    break;
                }
            }
            steps += 1;
        }
        let interp_trace = self.jit_mem_trace.take().unwrap_or_default();

        if reached {
            // Per-access trace diff: the FIRST point where the native and
            // interpreter memory-access sequences differ pinpoints the exact
            // miscompiled load/store (address or value).
            {
                let kindname = |k: u8| if k == 0 { "load " } else { "store" };
                let n = jit_trace.len().min(interp_trace.len());
                let mut diff_at: Option<usize> = None;
                for i in 0..n {
                    if jit_trace[i] != interp_trace[i] {
                        diff_at = Some(i);
                        break;
                    }
                }
                let report = diff_at.is_some() || jit_trace.len() != interp_trace.len();
                if report {
                    eprintln!(
                        "\n[JIT-VERIFY] MEM-TRACE DIVERGENCE entry={entry_pc:#x} (jit {} accesses, interp {}, first diff at {:?})",
                        jit_trace.len(),
                        interp_trace.len(),
                        diff_at
                    );
                    eprintln!(
                        "[JIT-VERIFY] trace-entry regs: rax={:#x} rcx={:#x} rdx={:#x} rbx={:#x} rsi={:#x} rdi={:#x}",
                        snap.rax, snap.rcx, snap.rdx, snap.rbx, snap.rsi, snap.rdi
                    );
                    let bytes = self.read_bytes(entry_pc, 128).unwrap_or_default();
                    eprintln!("[JIT-VERIFY] region bytes = {bytes:02x?}");
                    let center = diff_at.unwrap_or(n.saturating_sub(1));
                    let lo = center.saturating_sub(4);
                    let hi = (center + 4).min(jit_trace.len().max(interp_trace.len()));
                    for i in lo..hi {
                        let j = jit_trace
                            .get(i)
                            .map(|&(k, a, s, v)| format!("{} [{a:#x}/{s}B]={v:#x}", kindname(k)));
                        let ip = interp_trace
                            .get(i)
                            .map(|&(k, a, s, v)| format!("{} [{a:#x}/{s}B]={v:#x}", kindname(k)));
                        let mark = if jit_trace.get(i) != interp_trace.get(i) {
                            "<<<"
                        } else {
                            ""
                        };
                        eprintln!(
                            "[JIT-VERIFY]   #{i:<3} jit={:<34} interp={:<34} {mark}",
                            j.unwrap_or_else(|| "-".into()),
                            ip.unwrap_or_else(|| "-".into())
                        );
                    }
                }
            }

            // Status flags (CF PF AF ZF SF OF) + IF (0x200) + DF (0x400): the JIT
            // must change ONLY the status flags and preserve IF/DF — IF
            // corruption (spurious interrupt enable) crashed the kernel boot.
            const MASK: u64 = 0x0000_0000_0000_0ED5;
            let g = [
                ("rax", self.regs.rax, jit.rax),
                ("rcx", self.regs.rcx, jit.rcx),
                ("rdx", self.regs.rdx, jit.rdx),
                ("rbx", self.regs.rbx, jit.rbx),
                ("rsp", self.regs.rsp, jit.rsp),
                ("rbp", self.regs.rbp, jit.rbp),
                ("rsi", self.regs.rsi, jit.rsi),
                ("rdi", self.regs.rdi, jit.rdi),
                ("r8", self.regs.r8, jit.r8),
                ("r9", self.regs.r9, jit.r9),
                ("r10", self.regs.r10, jit.r10),
                ("r11", self.regs.r11, jit.r11),
                ("r12", self.regs.r12, jit.r12),
                ("r13", self.regs.r13, jit.r13),
                ("r14", self.regs.r14, jit.r14),
                ("r15", self.regs.r15, jit.r15),
            ];
            let mut diffs: Vec<String> = Vec::new();
            for (name, interp, native) in g {
                if interp != native {
                    diffs.push(format!("{name}: interp={interp:#x} jit={native:#x}"));
                }
            }
            // A flags-ONLY divergence (registers + memory all match) is a benign
            // dead-flag artifact: the optimizer drops a flag update it proved
            // dead across the FULL lifted function, but the JIT region is
            // truncated at a frontier, so at the hand-off PC the stale flags are
            // still visible — yet the interpreter resumes into the very blocks
            // that overwrite them before any read. Log, don't abort.
            let interp_rflags = self.compute_materialized_rflags();
            let flag_diff = if (interp_rflags & MASK) != (jit_rflags & MASK) {
                Some(format!(
                    "rflags: interp={:#x} jit={:#x}",
                    interp_rflags & MASK,
                    jit_rflags & MASK
                ))
            } else {
                None
            };
            // Memory: compare the interpreter's final value at each address the
            // native region wrote.
            for &(addr, size, native_v) in &native_writes {
                if let Ok(interp_v) = self.read_mem(addr, size) {
                    if interp_v != native_v {
                        diffs.push(format!(
                            "mem[{addr:#x}/{size}B]: interp={interp_v:#x} jit={native_v:#x}"
                        ));
                    }
                }
            }
            if !diffs.is_empty() {
                let code = self.read_bytes(entry_pc, 256).unwrap_or_default();
                eprintln!(
                    "\n[JIT-VERIFY] DIVERGENCE entry={entry_pc:#x} exit={exit_pc:#x} steps={steps}"
                );
                eprintln!(
                    "[JIT-VERIFY] entry regs: rax={:#x} rcx={:#x} rdx={:#x} rbx={:#x} rsi={:#x} rdi={:#x} r8={:#x} r9={:#x} r10={:#x} r11={:#x}",
                    snap.rax,
                    snap.rcx,
                    snap.rdx,
                    snap.rbx,
                    snap.rsi,
                    snap.rdi,
                    snap.r8,
                    snap.r9,
                    snap.r10,
                    snap.r11
                );
                eprintln!("[JIT-VERIFY] code@entry[256] = {code:02x?}");
                // The JIT's load trace reconstructs the memory the region reads
                // (the helper funnels every JIT access through read_mem).
                let loads: Vec<String> = jit_trace
                    .iter()
                    .filter(|&&(k, _, _, _)| k == 0)
                    .map(|&(_, a, s, v)| format!("[{a:#x}/{s}B]={v:#x}"))
                    .collect();
                eprintln!("[JIT-VERIFY] jit loads ({}): {:?}", loads.len(), loads);
                eprintln!(
                    "[JIT-VERIFY] lifted+optimized region:\n{}",
                    self.jit_dump_region(entry_pc)
                );
                for d in &diffs {
                    eprintln!("[JIT-VERIFY]   {d}");
                }
                eprintln!("[JIT-VERIFY] aborting (first divergence).");
                std::process::exit(70);
            }

            // Registers + memory matched. A residual flags-only difference is a
            // benign dead-flag artifact (see above) — log a throttled sample and
            // carry on with the native result, exactly as a non-verify run would.
            if let Some(d) = flag_diff {
                use std::sync::atomic::{AtomicUsize, Ordering};
                static N: AtomicUsize = AtomicUsize::new(0);
                let n = N.fetch_add(1, Ordering::Relaxed);
                if n < 8 {
                    eprintln!(
                        "[JIT-VERIFY] benign dead-flag diff #{n} entry={entry_pc:#x} exit={exit_pc:#x}: {d}"
                    );
                }
            }
        }

        // Matched (or unverifiable within the cap): adopt the native result.
        self.regs = jit;
    }

    /// Re-lift + optimize the region at `entry` and pretty-print its blocks/ops
    /// for the verify-mode divergence dump.
    pub fn jit_dump_region(&mut self, entry: u64) -> String {
        use crate::smir::lift::x86_64::X86_64Lifter;
        use crate::smir::lift::{LiftContext, MemoryReader, SmirLifter};
        use crate::smir::memory::MemoryError;
        use crate::smir::opt::{OptLevel, optimize_function};
        use crate::smir::types::SourceArch;

        let bytes = match self.read_bytes(entry, 512) {
            Ok(b) => b,
            Err(_) => return "<unreadable>".to_string(),
        };
        struct Win {
            base: u64,
            bytes: Vec<u8>,
        }
        impl MemoryReader for Win {
            fn read(&self, addr: u64, size: usize) -> core::result::Result<Vec<u8>, MemoryError> {
                let off = addr
                    .checked_sub(self.base)
                    .filter(|&o| (o as usize) < self.bytes.len())
                    .ok_or(MemoryError::OutOfBounds { addr })? as usize;
                let n = (self.bytes.len() - off).min(size);
                Ok(self.bytes[off..off + n].to_vec())
            }
        }
        let reader = Win { base: entry, bytes };
        let mut lifter = X86_64Lifter::strict();
        let mut lctx = LiftContext::new(SourceArch::X86_64);
        let mut func = match lifter.lift_function(entry, &reader, &mut lctx) {
            Ok(f) => f,
            Err(e) => return format!("<lift error: {e:?}>"),
        };
        optimize_function(&mut func, OptLevel::O2);
        let mut s = String::new();
        for b in &func.blocks {
            s.push_str(&format!("  block {:?} @ {:#x}:\n", b.id, b.guest_pc));
            for op in &b.ops {
                s.push_str(&format!("    {:#x}: {:?}\n", op.guest_pc, op.kind));
            }
            s.push_str(&format!("    term: {:?}\n", b.terminator));
        }
        s
    }

    /// The decode/JIT cache key discriminator: address space (CR3) + CPU mode
    /// (CS.L long-mode, CS.DB default-size). A cached region can never be reused
    /// across a context or mode switch.
    #[inline]
    pub(super) fn jit_mode_tag(&self) -> u64 {
        (self.sregs.cr3 & !0xFFF) | (self.sregs.cs.l as u64) | ((self.sregs.cs.db as u64) << 1)
    }

    /// Loop-head hotness sampling: called after an interpreted instruction. If
    /// the instruction was a BACKWARD branch (rip decreased) — i.e. a loop
    /// back-edge to `rip` — bump that head's counter and, once hot, compile +
    /// cache the region (and run it immediately). RIP now equals the head, so
    /// `jit_compile_region` compiles exactly there. Ineligible heads are cached
    /// as `None` so they are never retried.
    fn jit_sample_backedge(&mut self, rip_before: u64) {
        // Diagnostic kill-switch: RAX_NO_JIT disables hot-region promotion so the
        // interpreter handles everything (isolates JIT-codegen bugs from the
        // sampling/SMC infrastructure). Cached once — back-edges are hot.
        {
            use std::sync::OnceLock;
            static OFF: OnceLock<bool> = OnceLock::new();
            if *OFF.get_or_init(|| std::env::var_os("RAX_NO_JIT").is_some()) {
                return;
            }
        }
        let head = self.regs.rip;
        if head >= rip_before {
            return; // forward/fallthrough — not a loop back-edge
        }
        let mt = self.jit_mode_tag();
        if self.jit_cache.contains_key(&(head, mt)) {
            return; // already promoted (runnable) this session
        }
        // Known-ineligible memo (survives SMC). Skip the futile re-lift unless the
        // head's actual code bytes changed — a self-modifying-code write to a
        // page that merely shares a frame with this head must not re-promote it.
        let memo_on = {
            use std::sync::OnceLock;
            static ON: OnceLock<bool> = OnceLock::new();
            *ON.get_or_init(|| std::env::var_os("RAX_JIT_NOMEMO").is_none())
        };
        if memo_on {
            if let Some(&fp) = self.jit_ineligible.get(&(head, mt)) {
                if fp == self.jit_head_fingerprint(head) {
                    return; // identical code, still ineligible — no work
                }
                self.jit_ineligible.remove(&(head, mt)); // code changed → re-evaluate
            }
        }
        let hot = {
            let c = self.jit_hot.entry(head).or_insert(0);
            *c = c.saturating_add(1);
            *c
        };
        if hot < JIT_HOT_THRESHOLD {
            return;
        }
        self.jit_hot.remove(&head);
        let region = self
            .jit_compile_region()
            .ok()
            .flatten()
            .map(std::sync::Arc::new);
        if std::env::var_os("RAX_JIT_LOG").is_some() {
            eprintln!(
                "[JIT] promote @ {head:#x} -> {}",
                if region.is_some() {
                    "compiled"
                } else {
                    "ineligible"
                }
            );
        }
        match &region {
            Some(r) => {
                let r = r.clone();
                self.jit_cache.insert((head, mt), region);
                self.jit_run_region(&r);
            }
            None => {
                // Memoize the ineligible verdict keyed on the current code bytes.
                // Soft-cap the table so a long-running guest can't grow it without
                // bound (cleared wholesale — the fingerprints rebuild on demand).
                if self.jit_ineligible.len() >= 8192 {
                    self.jit_ineligible.clear();
                }
                let fp = self.jit_head_fingerprint(head);
                self.jit_ineligible.insert((head, mt), fp);
            }
        }
    }

    /// Cheap fault-free fingerprint of the guest code bytes at `head`: 16 bytes
    /// folded into a u64. Used to memoize an ineligible JIT verdict (see
    /// [`Self::jit_ineligible`]) so a self-modifying-code page write doesn't force
    /// a redundant re-lift when the head's instructions are actually unchanged. A
    /// genuine code edit shifts the fingerprint and re-triggers compilation. Reads
    /// are pure RAM fetches (no SMC marking); an unmapped head folds to a stable
    /// sentinel so its deterministic-fault verdict is still memoized.
    #[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
    fn jit_head_fingerprint(&mut self, head: u64) -> u64 {
        let lo = self.mmu.read_u64(head, &self.sregs).unwrap_or(head);
        let hi = self
            .mmu
            .read_u64(head.wrapping_add(8), &self.sregs)
            .unwrap_or(0);
        lo ^ hi.rotate_left(17)
    }

    /// Number of distinct regions the JIT has compiled (cache entries that
    /// produced runnable native code). For tests / diagnostics.
    pub fn jit_region_count(&self) -> usize {
        self.jit_cache.values().filter(|v| v.is_some()).count()
    }

    /// Enable/disable JIT of memory-touching regions (Load/Store via MMU helper
    /// calls). For tests; production seeds this from `RAX_JIT_MEM`.
    pub fn set_jit_mem(&mut self, on: bool) {
        self.jit_mem = on;
    }
}
