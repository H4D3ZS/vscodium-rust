//! Native execution runtime for SMIR-lowered blocks (the JIT back end's
//! "executor"). This is the bridge that takes the x86-64 machine code produced
//! by [`crate::smir::lower::x86_64::X86_64Lowerer`] and actually runs it on the
//! host CPU, marshalling guest register state in and out.
//!
//! Gated behind the `smir-jit` feature and `target_arch = "x86_64"` — the entry
//! trampoline is hand-written x86-64 assembly, and execution relies on the
//! lowerer's 1:1 identity register map (guest GPR `N` ⇒ the same-named host
//! GPR), so a lowered block reads and writes guest state directly with no
//! per-instruction marshalling. The only marshalling is once on entry and once
//! on exit, in [`enter_native`].
//!
//! Validated bit-exact against KVM by the differential harness in
//! `tests/diff_fuzz.rs` (`smir_native_*` tests) across ALU, shifts, MUL,
//! conditional branches, and whole loops (the "dragon" path).

#![cfg(all(feature = "smir-jit", target_arch = "x86_64"))]

use super::{
    X86_GUEST_CALL_FN_OFFSET, X86_GUEST_CTX_OFFSET, X86_GUEST_EXIT_PC_OFFSET,
    X86_GUEST_FS_BASE_OFFSET, X86_GUEST_GPR_COUNT, X86_GUEST_GS_BASE_OFFSET,
    X86_GUEST_LOAD_FN_OFFSET, X86_GUEST_RFLAGS_OFFSET, X86_GUEST_STORE_FN_OFFSET,
    X86_STATE_PTR_AT_RBP,
};

/// Guest register file marshalled in/out of a lowered native block.
///
/// `gpr[i]` is indexed by x86 register *encoding*
/// (0=RAX, 1=RCX, 2=RDX, 3=RBX, 4=RSP, 5=RBP, 6=RSI, 7=RDI, 8..=15=R8..=R15,
/// 16..=31=R16..=R31). `rflags` holds the materialized flags. `repr(C)` with a
/// fixed layout — the trampoline reads/writes by byte offset (`gpr[i]` at
/// `i*8`, `rflags` at [`X86_GUEST_RFLAGS_OFFSET`]).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GuestRegs {
    /// General-purpose registers, indexed by x86 encoding.
    pub gpr: [u64; X86_GUEST_GPR_COUNT],
    /// Materialized RFLAGS.
    pub rflags: u64,
    /// Resume guest PC, written by an exit stub when a block lowered with the
    /// general-exit ABI hands control back to the interpreter. Only meaningful
    /// for blocks run via [`ExecMem::run_with_exit`]. See
    /// [`enter_native`] (the R15-reserved trampoline) and the lowerer's
    /// `native_exit` mode.
    pub exit_pc: u64,
    /// Opaque context pointer passed as arg0 to the memory helpers (the
    /// `*mut X86_64Vcpu`). Set by the JIT before each run.
    pub ctx: u64,
    /// Address of the load helper `fn(ctx, addr, size, signed) -> (value, ok)`
    /// (SysV: value in RAX, ok in RDX).
    pub load_fn: u64,
    /// Address of the store helper `fn(ctx, addr, value, size) -> ok`.
    pub store_fn: u64,
    /// IA32_FS_BASE. The lowered code adds
    /// this to the effective address of an `fs:`-overridden memory operand
    /// ([`Address::SegmentRel`]). Set from `sregs.fs.base` before each run.
    pub fs_base: u64,
    /// IA32_GS_BASE. As `fs_base` but for
    /// `gs:`-overridden operands (per-CPU data in the Linux kernel).
    pub gs_base: u64,
    /// Address of the call helper `fn(gr, target_pc, return_pc) -> ok`. Used by
    /// the lift-through-calls path (RAX_JIT_CALL): a guest CALL in a JIT region
    /// lowers to a call-out into this helper, which runs the interpreter for the
    /// callee until it returns to `return_pc`, then resumes native execution.
    /// `ok == 0` means the callee bailed to the interpreter (an exit/exception)
    /// and the region must return; the helper has set `exit_pc`. NOTE: arg0 is
    /// the `*mut GuestRegs` itself (not `ctx`), because the helper needs the
    /// full marshalled guest state, and `gr.ctx` carries the vcpu pointer.
    pub call_fn: u64,
}

/// AArch64 guest register file for state-backed x86-64 lowering.
///
/// Unlike [`GuestRegs`], this ABI does not rely on identity-mapping guest
/// registers into host registers. Lowered code is entered as a normal SysV
/// function with `RDI = *mut Aarch64GuestRegs` and reads/writes architectural
/// state through this struct. NZCV is stored in architectural PSTATE position
/// (bits 31:28); the remaining bits are preserved as zero for now.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aarch64GuestRegs {
    /// X0-X30.
    pub x: [u64; 31],
    /// Stack pointer.
    pub sp: u64,
    /// Program counter.
    pub pc: u64,
    /// PSTATE.NZCV in bits 31:28.
    pub nzcv: u64,
    /// Floating-point control register.
    pub fpcr: u64,
    /// Floating-point status register.
    pub fpsr: u64,
    /// V0-V31 as low/high u64 pairs.
    pub v: [u64; 64],
    /// Opaque context pointer passed as arg0 to AArch64 memory helpers.
    pub ctx: u64,
    /// Address of `fn(ctx, addr, size, signed) -> value`.
    pub load_fn: u64,
    /// Address of `fn(ctx, addr, value, size) -> ok`.
    pub store_fn: u64,
    /// Address armed by the last load-exclusive.
    pub exclusive_addr: u64,
    /// Byte size armed by the last load-exclusive.
    pub exclusive_size: u64,
    /// Non-zero when the exclusive monitor is armed.
    pub exclusive_valid: u64,
}

impl Default for Aarch64GuestRegs {
    fn default() -> Self {
        Self {
            x: [0; 31],
            sp: 0,
            pc: 0,
            nzcv: 0,
            fpcr: 0,
            fpsr: 0,
            v: [0; 64],
            ctx: 0,
            load_fn: 0,
            store_fn: 0,
            exclusive_addr: 0,
            exclusive_size: 0,
            exclusive_valid: 0,
        }
    }
}

impl Aarch64GuestRegs {
    pub const X0_OFFSET: i32 = 0;
    pub const SP_OFFSET: i32 = 31 * 8;
    pub const PC_OFFSET: i32 = 32 * 8;
    pub const NZCV_OFFSET: i32 = 33 * 8;
    pub const FPCR_OFFSET: i32 = 34 * 8;
    pub const FPSR_OFFSET: i32 = 35 * 8;
    pub const V_OFFSET: i32 = 36 * 8;
    pub const CTX_OFFSET: i32 = Self::V_OFFSET + 64 * 8;
    pub const LOAD_FN_OFFSET: i32 = Self::CTX_OFFSET + 8;
    pub const STORE_FN_OFFSET: i32 = Self::LOAD_FN_OFFSET + 8;
    pub const EXCLUSIVE_ADDR_OFFSET: i32 = Self::STORE_FN_OFFSET + 8;
    pub const EXCLUSIVE_SIZE_OFFSET: i32 = Self::EXCLUSIVE_ADDR_OFFSET + 8;
    pub const EXCLUSIVE_VALID_OFFSET: i32 = Self::EXCLUSIVE_SIZE_OFFSET + 8;
}

// enter_native(rdi = entry ptr, rsi = *mut GuestRegs):
//   preserve host callee-saved -> load guest GPRs+RFLAGS into the identical host
//   regs -> `call` the block -> store the host regs back into GuestRegs.
// RSP (gpr[4]) is NOT loaded — the block runs on the host stack (it owns no
// guest stack). Alignment: 6 callee pushes (48) + `sub rsp,24` (72 total) leaves
// rsp 16-aligned at the `call`.
core::arch::global_asm!(
    ".text",
    ".p2align 4",
    ".globl rax_smir_enter_native",
    "rax_smir_enter_native:",
    "push rbp",
    "push rbx",
    "push r12",
    "push r13",
    "push r14",
    "push r15",
    "sub rsp, 24", // [rsp]=entry [rsp+8]=state [rsp+16]=pad ; rsp 16-aligned
    "mov [rsp], rdi",
    "mov [rsp+8], rsi",
    "mov rax, [rsi+256]", // RFLAGS
    "push rax",
    "popfq",
    "mov rax, [rsi+0]",
    "mov rcx, [rsi+8]",
    "mov rdx, [rsi+16]",
    "mov rbx, [rsi+24]",
    "mov rbp, [rsi+40]",
    "mov rdi, [rsi+56]",
    "mov r8,  [rsi+64]",
    "mov r9,  [rsi+72]",
    "mov r10, [rsi+80]",
    "mov r11, [rsi+88]",
    "mov r12, [rsi+96]",
    "mov r13, [rsi+104]",
    "mov r14, [rsi+112]",
    "mov r15, [rsi+120]",
    "mov rsi, [rsi+48]", // rsi last (was the base pointer)
    "call [rsp]",
    "push rax",          // save guest RAX ; state now at [rsp+16]
    "mov rax, [rsp+16]", // rax = *mut GuestRegs
    "mov [rax+8],   rcx",
    "mov [rax+16],  rdx",
    "mov [rax+24],  rbx",
    "mov [rax+40],  rbp",
    "mov [rax+48],  rsi",
    "mov [rax+56],  rdi",
    "mov [rax+64],  r8",
    "mov [rax+72],  r9",
    "mov [rax+80],  r10",
    "mov [rax+88],  r11",
    "mov [rax+96],  r12",
    "mov [rax+104], r13",
    "mov [rax+112], r14",
    "mov [rax+120], r15",
    "pushfq",
    "pop rcx",
    "mov [rax+256], rcx",
    // Sanitize the HOST EFLAGS before returning to Rust. The `popfq` above loaded
    // the GUEST RFLAGS into the host, and the region runs with them — but the
    // sticky control flags then LEAK into the host: AC (alignment check, set by
    // the kernel's SMAP `stac` for user copies) faults the next unaligned host
    // access with #AC/SIGBUS; DF (direction) reverses host `rep` string ops
    // (memcpy/memset) → corruption; TF would single-step → SIGTRAP; NT corrupts
    // a host `iret`. Clear bits 8(TF)/10(DF)/14(NT)/18(AC); the arithmetic flags
    // are caller-saved scratch the host re-derives, so they need no restore.
    "pushfq",
    "and qword ptr [rsp], -0x44501", // ~0x44500: clear TF(0x100)+DF(0x400)+NT(0x4000)+AC(0x40000)
    "popfq",
    "mov rcx, [rsp]", // saved guest RAX
    "mov [rax+0], rcx",
    "add rsp, 8",  // pop saved RAX
    "add rsp, 24", // pop locals
    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop rbx",
    "pop rbp",
    "ret",
);

// ELF marks the trampoline as a function in `.symtab`; Mach-O and COFF have no
// equivalent of `@function` and reject the directive, so it stays ELF-only. The
// trampoline itself is identical on every host — same Intel-syntax body, same
// underscore-free win64/SysV symbol name matching the `extern "C"` import.
#[cfg(target_os = "linux")]
core::arch::global_asm!(".type rax_smir_enter_native,@function");

unsafe extern "C" {
    fn rax_smir_enter_native(entry: *const u8, state: *mut GuestRegs);
}

/// Byte offset of `GuestRegs.exit_pc` (after `gpr[32]` + `rflags`). An exit stub
/// writes the resume guest PC here via the state pointer.
pub const EXIT_PC_OFFSET: i32 = X86_GUEST_EXIT_PC_OFFSET;

/// Offset of the `*mut GuestRegs` state pointer relative to a lowered block's
/// frame pointer (RBP), under the `rax_smir_enter_native` trampoline's stack
/// layout: the trampoline does `sub rsp,24; [rsp+8]=state` before `call`, and
/// the block's prologue `push rbp; mov rbp,rsp` lands RBP 24 bytes below that
/// slot — so `[rbp+24]` holds the state pointer throughout the block. An exit
/// stub loads it from here to record `exit_pc` (no reserved guest register).
pub const STATE_PTR_AT_RBP: i32 = X86_STATE_PTR_AT_RBP;

/// Byte offset of `GuestRegs.ctx` (the memory-helper context pointer).
pub const CTX_OFFSET: i32 = X86_GUEST_CTX_OFFSET;
/// Byte offset of `GuestRegs.load_fn` (the memory-load helper address).
pub const LOAD_FN_OFFSET: i32 = X86_GUEST_LOAD_FN_OFFSET;
/// Byte offset of `GuestRegs.store_fn` (the memory-store helper address).
pub const STORE_FN_OFFSET: i32 = X86_GUEST_STORE_FN_OFFSET;
/// Byte offset of `GuestRegs.fs_base` (the FS segment base for `fs:` operands).
pub const FS_BASE_OFFSET: i32 = X86_GUEST_FS_BASE_OFFSET;
/// Byte offset of `GuestRegs.gs_base` (the GS segment base for `gs:` operands).
pub const GS_BASE_OFFSET: i32 = X86_GUEST_GS_BASE_OFFSET;
/// Byte offset of `GuestRegs.call_fn` (the lift-through-calls helper address).
pub const CALL_FN_OFFSET: i32 = X86_GUEST_CALL_FN_OFFSET;

/// W^X executable memory holding a finalized lowered block. Maps RW, copies the
/// code in, then flips to RX; unmaps on drop.
pub struct ExecMem {
    ptr: *mut u8,
    len: usize,
}

impl ExecMem {
    /// Map `code` into a fresh W^X region and make it executable.
    pub fn new(code: &[u8]) -> Result<Self, ExecMemError> {
        if code.is_empty() {
            return Err(ExecMemError::Empty);
        }
        let len = (code.len() + 0xFFF) & !0xFFF;
        let ptr = os_alloc_rw(len).ok_or(ExecMemError::Mmap)?;
        unsafe { core::ptr::copy_nonoverlapping(code.as_ptr(), ptr, code.len()) };
        if !os_make_rx(ptr, len) {
            os_free(ptr, len);
            return Err(ExecMemError::Mprotect);
        }
        Ok(ExecMem { ptr, len })
    }

    /// Execute the block at `entry_offset` (the lowerer's `LowerResult.entry_offset`),
    /// marshalling `regs` in and reading the result back out.
    ///
    /// # Safety
    /// The caller must guarantee that the code was produced by a trusted lowerer
    /// for an identity-register-mapped block that does not require a guest stack
    /// (RSP is not loaded — the block runs on the host stack).
    pub fn run(&self, entry_offset: usize, regs: &mut GuestRegs) {
        let entry = unsafe { self.ptr.add(entry_offset) } as *const u8;
        unsafe { rax_smir_enter_native(entry, regs as *mut GuestRegs) };
    }

    /// Execute a state-backed AArch64-on-x86 lowered block.
    ///
    /// # Safety
    /// The mapped code must use the `extern "C" fn(*mut Aarch64GuestRegs)` ABI
    /// and preserve the host ABI. The AArch64 state-backed lowerer emits leaf
    /// functions using only caller-saved registers plus an RBP frame.
    pub fn run_aarch64(&self, entry_offset: usize, regs: &mut Aarch64GuestRegs) {
        type Entry = unsafe extern "C" fn(*mut Aarch64GuestRegs);
        let entry = unsafe { self.ptr.add(entry_offset) } as *const u8;
        let entry: Entry = unsafe { core::mem::transmute(entry) };
        unsafe { entry(regs as *mut Aarch64GuestRegs) };
    }
}

impl Drop for ExecMem {
    fn drop(&mut self) {
        os_free(self.ptr, self.len);
    }
}

// --- Host W^X primitives ---------------------------------------------------
//
// The JIT needs a private region that is RW while the lowered code is copied in,
// then flipped to RX before execution. POSIX does this with mmap+mprotect;
// Windows with VirtualAlloc+VirtualProtect. Both produce an identical W^X
// guarantee — the region is never simultaneously writable and executable.

/// Allocate a fresh page-aligned, zero-filled, read-write region of `len` bytes.
#[cfg(unix)]
fn os_alloc_rw(len: usize) -> Option<*mut u8> {
    let ptr = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        None
    } else {
        Some(ptr as *mut u8)
    }
}

/// Flip a region allocated by [`os_alloc_rw`] from RW to RX. Returns `false` on
/// failure (the caller frees the region).
#[cfg(unix)]
fn os_make_rx(ptr: *mut u8, len: usize) -> bool {
    unsafe {
        libc::mprotect(
            ptr as *mut libc::c_void,
            len,
            libc::PROT_READ | libc::PROT_EXEC,
        ) == 0
    }
}

/// Release a region allocated by [`os_alloc_rw`].
#[cfg(unix)]
fn os_free(ptr: *mut u8, len: usize) {
    unsafe {
        libc::munmap(ptr as *mut libc::c_void, len);
    }
}

#[cfg(windows)]
fn os_alloc_rw(len: usize) -> Option<*mut u8> {
    use windows_sys::Win32::System::Memory::{
        MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc,
    };
    let ptr = unsafe {
        VirtualAlloc(
            core::ptr::null(),
            len,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    };
    if ptr.is_null() {
        None
    } else {
        Some(ptr as *mut u8)
    }
}

#[cfg(windows)]
fn os_make_rx(ptr: *mut u8, len: usize) -> bool {
    use windows_sys::Win32::System::Memory::{PAGE_EXECUTE_READ, VirtualProtect};
    let mut old: u32 = 0;
    let ok = unsafe { VirtualProtect(ptr as *mut _, len, PAGE_EXECUTE_READ, &mut old) != 0 };
    if ok {
        // x86 keeps the icache coherent with stores, but flush explicitly so the
        // same abstraction is correct if this ever lowers for another host ISA.
        use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
        use windows_sys::Win32::System::Threading::GetCurrentProcess;
        unsafe { FlushInstructionCache(GetCurrentProcess(), ptr as *const _, len) };
    }
    ok
}

#[cfg(windows)]
fn os_free(ptr: *mut u8, _len: usize) {
    use windows_sys::Win32::System::Memory::{MEM_RELEASE, VirtualFree};
    unsafe {
        VirtualFree(ptr as *mut _, 0, MEM_RELEASE);
    }
}

// SAFETY: an ExecMem owns a private W^X mapping of immutable native code. After
// construction the bytes never change, and execution only reads them; the
// owning vcpu is the sole accessor. Sending the mapping to another thread (when
// a vcpu migrates) or sharing &ExecMem for read-only execution is therefore
// sound. The raw pointer alone makes ExecMem !Send/!Sync by default.
unsafe impl Send for ExecMem {}
unsafe impl Sync for ExecMem {}

/// Errors mapping/executing a lowered block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecMemError {
    /// Empty code buffer.
    Empty,
    /// `mmap` failed.
    Mmap,
    /// `mprotect` to RX failed.
    Mprotect,
}

impl core::fmt::Display for ExecMemError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExecMemError::Empty => write!(f, "empty code buffer"),
            ExecMemError::Mmap => write!(f, "mmap failed"),
            ExecMemError::Mprotect => write!(f, "mprotect to RX failed"),
        }
    }
}

impl std::error::Error for ExecMemError {}

/// Decide whether a lifted function is safe to execute through the native tier
/// under the 1:1 identity register map.
///
/// The identity map (guest GPR `N` ⇒ host GPR `N`) is what makes native
/// execution marshal-free, but it leaves *every* host GPR holding live guest
/// state — there is no free scratch register. So any value the block writes to a
/// `VReg::Virtual` (a non-architectural temporary the lifter introduced) would
/// be allocated onto a guest-occupied host register and silently corrupt guest
/// state on write-back. Such a block must NOT be promoted; the interpreter runs
/// it instead.
///
/// Exempt: a trailing `TestCondition` whose `dst` feeds the block's
/// `CondBranch` — the lowerer folds it into a direct `Jcc` off the live flags
/// and never materializes the temporary (see `X86_64Lowerer::lower_block`).
///
/// Pure architectural-register blocks (counter/pointer loops, ALU chains,
/// guest-conditional branches) pass — which is the bulk of hot code.
pub fn is_native_clobber_safe(func: &crate::smir::ir::SmirFunction) -> bool {
    func.blocks.iter().all(|b| block_is_clobber_safe(b, false))
}

/// Like [`is_native_clobber_safe`] but skips blocks in `excluded` (block-id ⇒
/// resume PC, i.e. the native-exit stubs). Those blocks are lowered to exit
/// stubs and never execute natively, so their ops can't clobber guest state —
/// excluding them lets the JIT accept regions whose loop is clobber-safe even
/// when an exit/continuation block uses a virtual temporary.
pub fn is_native_clobber_safe_excluding(
    func: &crate::smir::ir::SmirFunction,
    excluded: &std::collections::HashMap<crate::smir::types::BlockId, u64>,
    allow_mem: bool,
) -> bool {
    func.blocks
        .iter()
        .filter(|b| !excluded.contains_key(&b.id))
        .all(|b| block_is_clobber_safe(b, allow_mem))
}

/// True if every op in `block` is safe to execute natively under the JIT:
///   (1) it is on the fail-safe register-only whitelist ([`OpKind::is_jit_safe`])
///       — so it touches no memory and is validated bit-exact vs KVM; and
///   (2) it writes only architectural registers (no virtual temp, which would
///       alias a guest GPR under the identity register map).
/// A trailing `TestCondition` feeding the block's `CondBranch` is exempt (the
/// lowerer folds it into a direct `Jcc` and never materializes its dst).
fn block_is_clobber_safe(block: &crate::smir::ir::SmirBlock, allow_mem: bool) -> bool {
    use crate::smir::ir::Terminator;
    use crate::smir::ops::OpKind;
    use crate::smir::types::{ArchReg, VReg, X86Reg};

    // The native trampoline runs the region on the HOST stack: guest RSP is
    // never loaded into the host RSP, and the lowerer's prologue repurposes RBP
    // as the frame pointer (clobbering the guest RBP loaded in). So any op that
    // reads OR writes guest RSP/RBP would compute against the host RSP/RBP
    // instead of the guest value — silently wrong, and a write to RSP corrupts
    // the host stack. Such stack-frame code must stay in the interpreter. (The
    // differential fuzzers never generate RSP/RBP operands, so this gap was
    // invisible until real kernel code — which uses them constantly — was JIT'd.)
    let touches_sp_bp = |v: &VReg| {
        matches!(
            v,
            VReg::Arch(ArchReg::X86(X86Reg::Rsp)) | VReg::Arch(ArchReg::X86(X86Reg::Rbp))
        )
    };

    let n = block.ops.len();
    for (i, op) in block.ops.iter().enumerate() {
        if i + 1 == n {
            if let (Terminator::CondBranch { cond, .. }, OpKind::TestCondition { dst, .. }) =
                (&block.terminator, &op.kind)
            {
                if dst == cond {
                    continue;
                }
            }
        }
        // (1) fail-safe whitelist: any non-whitelisted op (div, FP/SIMD,
        // syscall, unvalidated) makes the whole region ineligible. When memory
        // JIT is enabled, register-destination Load/Store are additionally
        // allowed (they lower to MMU helper calls with fault-bail); RMW forms
        // still bail via the virtual-temp check below, and RSP/RBP-based
        // addresses via check (3).
        let mem_ok = allow_mem && matches!(op.kind, OpKind::Load { .. } | OpKind::Store { .. });
        if !op.kind.is_jit_safe() && !mem_ok {
            return false;
        }
        // (2) no virtual-temp writes (would clobber a guest GPR).
        if op
            .kind
            .dests()
            .iter()
            .any(|d| matches!(d, VReg::Virtual(_)))
        {
            return false;
        }
        // (3) guest RSP/RBP. A WRITE is never modeled (the trampoline freezes
        // both — see note above) → bail. A READ is fine ONLY as an operand of a
        // mem-JIT Load/Store (an address base/index, or a stored value): the MMU
        // helper reads the value from the GuestRegs struct — the correct frozen
        // guest RSP/RBP — not the host RSP/RBP. Any OTHER op reading RSP/RBP
        // would use the host frame pointer / host stack (wrong) → bail. (When
        // `allow_mem` is off, `mem_ok` is always false, so this is identical to
        // the prior "no RSP/RBP reads or writes" rule — the validated default.)
        if op.kind.dests().iter().any(touches_sp_bp) {
            return false;
        }
        if !mem_ok && op.kind.source_vregs().iter().any(touches_sp_bp) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hand-assembled `mov eax, 0x2a ; ret` — proves ExecMem (W^X map) and the
    // enter_native trampoline marshal a result back out, independent of the
    // lowerer. The lowerer-driven end-to-end paths live in tests/diff_fuzz.rs.
    #[test]
    fn exec_mem_runs_raw_block() {
        let code = [0xB8, 0x2A, 0x00, 0x00, 0x00, 0xC3];
        let mem = ExecMem::new(&code).expect("ExecMem map");
        let mut regs = GuestRegs::default();
        regs.rflags = 0x2;
        mem.run(0, &mut regs);
        assert_eq!(regs.gpr[0], 0x2a, "RAX should be 0x2a");
    }

    // RAX = RBX + RCX, exercising guest-GPR marshal IN as well as OUT.
    //   lea eax,[rbx+rcx] won't preserve 64-bit; use: mov rax,rbx; add rax,rcx; ret
    #[test]
    fn exec_mem_marshals_inputs() {
        // 48 89 D8        mov rax, rbx
        // 48 01 C8        add rax, rcx
        // C3              ret
        let code = [0x48, 0x89, 0xD8, 0x48, 0x01, 0xC8, 0xC3];
        let mem = ExecMem::new(&code).expect("ExecMem map");
        let mut regs = GuestRegs::default();
        regs.gpr[3] = 40; // RBX
        regs.gpr[1] = 2; // RCX
        regs.rflags = 0x2;
        mem.run(0, &mut regs);
        assert_eq!(regs.gpr[0], 42, "RAX should be RBX+RCX");
    }

    // General-exit stub: a block (with the lowerer's `push rbp; mov rbp,rsp`
    // prologue) records its resume PC into exit_pc by loading the state pointer
    // from the trampoline frame into a push/pop-saved
    // scratch — no reserved guest register, runs under the existing trampoline.
    #[test]
    fn exec_mem_exit_pc_via_stub() {
        let mut code = vec![
            0x55, // push rbp
            0x48,
            0x89,
            0xE5, // mov rbp, rsp
            0x50, // push rax (scratch)
            0x48,
            0x8B,
            0x45,
            X86_STATE_PTR_AT_RBP as u8, // mov rax, [rbp+state_ptr]
            0xC7,
            0x80,
        ];
        code.extend_from_slice(&(X86_GUEST_EXIT_PC_OFFSET as u32).to_le_bytes());
        code.extend_from_slice(&0x1234_abcdu32.to_le_bytes());
        code.extend_from_slice(&[0xC7, 0x80]);
        code.extend_from_slice(&((X86_GUEST_EXIT_PC_OFFSET + 4) as u32).to_le_bytes());
        code.extend_from_slice(&0u32.to_le_bytes());
        code.extend_from_slice(&[
            0x58, // pop rax
            0x48, 0x89, 0xEC, // mov rsp, rbp
            0x5D, // pop rbp
            0xC3, // ret
        ]);
        let mem = ExecMem::new(&code).expect("ExecMem map");
        let mut regs = GuestRegs::default();
        regs.gpr[0] = 0xCAFE; // guest RAX must pass through (scratch restored)
        regs.rflags = 0x2;
        mem.run(0, &mut regs);
        assert_eq!(
            regs.exit_pc, 0x1234_abcd,
            "exit_pc recorded via frame state ptr"
        );
        assert_eq!(regs.gpr[0], 0xCAFE, "guest RAX restored after scratch use");
    }

    use crate::smir::flags::FlagUpdate;
    use crate::smir::ir::{FunctionBuilder, Terminator, TrapKind};
    use crate::smir::ops::OpKind;
    use crate::smir::types::{ArchReg, Condition, FunctionId, OpWidth, SrcOperand, VReg, X86Reg};

    fn rax() -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::Rax))
    }
    fn rcx() -> VReg {
        VReg::Arch(ArchReg::X86(X86Reg::Rcx))
    }

    #[test]
    fn clobber_gate_passes_pure_arch_block() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x1000);
        b.push_op(
            0x1000,
            OpKind::Add {
                dst: rax(),
                src1: rax(),
                src2: SrcOperand::Reg(rcx()),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });
        assert!(is_native_clobber_safe(&b.finish()));
    }

    #[test]
    fn clobber_gate_rejects_virtual_temp() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x1000);
        let tmp = b.alloc_vreg(); // VReg::Virtual
        b.push_op(
            0x1000,
            OpKind::Add {
                dst: tmp, // writes a virtual temporary -> would clobber a guest GPR
                src1: rax(),
                src2: SrcOperand::Reg(rcx()),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );
        b.set_terminator(Terminator::Return { values: vec![] });
        assert!(!is_native_clobber_safe(&b.finish()));
    }

    #[test]
    fn clobber_gate_excludes_exit_blocks() {
        // entry: add rax,rcx (arch) → Branch to exit_blk
        // exit_blk: writes a VIRTUAL temp, then Trap (a frontier the JIT skips).
        let mut b = FunctionBuilder::new(FunctionId(0), 0x1000);
        let exit_blk = b.create_block(0x2000);
        b.push_op(
            0x1000,
            OpKind::Add {
                dst: rax(),
                src1: rax(),
                src2: SrcOperand::Reg(rcx()),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );
        b.set_terminator(Terminator::Branch { target: exit_blk });
        b.switch_to_block(exit_blk);
        let tmp = b.alloc_vreg();
        b.push_op(
            0x2000,
            OpKind::Add {
                dst: tmp, // virtual temp — only safe because this block is skipped
                src1: rax(),
                src2: SrcOperand::Reg(rcx()),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );
        b.set_terminator(Terminator::Trap {
            kind: TrapKind::Halt,
        });
        let func = b.finish();

        assert!(
            !is_native_clobber_safe(&func),
            "exit block's virtual write trips the strict gate"
        );
        let mut exits = std::collections::HashMap::new();
        exits.insert(exit_blk, 0x2000u64);
        assert!(
            is_native_clobber_safe_excluding(&func, &exits, false),
            "excluding the (skipped) exit block, the executed region is safe"
        );
    }

    #[test]
    fn clobber_gate_exempts_folded_testcondition() {
        let mut b = FunctionBuilder::new(FunctionId(0), 0x1000);
        let t_blk = b.create_block(0x2000);
        let f_blk = b.create_block(0x3000);
        let cond = b.alloc_vreg();
        b.push_op(
            0x1000,
            OpKind::Sub {
                dst: rcx(),
                src1: rcx(),
                src2: SrcOperand::imm(1),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
        );
        // Trailing TestCondition feeding the CondBranch: lowerer folds it, never
        // materializing `cond`, so the gate must treat the block as safe.
        b.push_op(
            0x1003,
            OpKind::TestCondition {
                dst: cond,
                cond: Condition::Ne,
            },
        );
        b.set_terminator(Terminator::CondBranch {
            cond,
            true_target: t_blk,
            false_target: f_blk,
        });
        b.switch_to_block(t_blk);
        b.set_terminator(Terminator::Return { values: vec![] });
        b.switch_to_block(f_blk);
        b.set_terminator(Terminator::Return { values: vec![] });
        assert!(is_native_clobber_safe(&b.finish()));
    }
}
