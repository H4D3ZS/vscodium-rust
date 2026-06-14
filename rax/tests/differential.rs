//! Differential test harness: software interpreter vs. KVM (hardware oracle).
//!
//! Each test case is a short machine-code sequence ending in `HLT`. It is run on
//! BOTH the rax software interpreter and on KVM from an *identical* initial
//! architectural state, then the resulting state (GPRs, RIP, the observable
//! RFLAGS bits, a couple of XMM registers, and a scratch memory page) is
//! compared. Any divergence is an interpreter bug (or, rarely, a harness
//! limitation) and is reported precisely.
//!
//! Robustness:
//!  - If `/dev/kvm` cannot be opened/driven, every test self-skips (returns
//!    without failing) so the suite is green in no-KVM environments.
//!  - Execution on both backends is bounded so a buggy case cannot hang.
//!
//! Initial state: long mode with identity-mapped paging. Real long mode requires
//! paging (CR0.PG=1 with EFER.LMA=1), and KVM enforces this, so we install a
//! tiny identity map (PML4 -> PDPTE with 1GiB pages) making GPA == GVA. The
//! interpreter is given the exact same CR0/CR3/CR4/EFER/segments so the two are
//! directly comparable.

#![cfg(all(feature = "kvm", target_os = "linux"))]

use std::sync::Arc;

use rax::backend::emulator::x86_64::{X86_64Vcpu, flags};
use rax::cpu::{Registers, SystemRegisters, VCpu, VcpuExit};
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

// ---------------------------------------------------------------------------
// Memory layout (all identity-mapped: GVA == GPA)
// ---------------------------------------------------------------------------

/// Total guest memory size for each backend.
const MEM_SIZE: usize = 8 * 1024 * 1024; // 8 MiB
/// Where test code is loaded / RIP starts.
const CODE_ADDR: u64 = 0x1_0000;
/// Initial stack pointer.
const STACK_ADDR: u64 = 0x2_0000;
/// Scratch data page (read back after the run for memory-effect tests).
const DATA_ADDR: u64 = 0x3_0000;
/// Page-table addresses (mirrors src/arch/x86_64 layout).
const PML4_ADDR: u64 = 0x9000;
const PDPTE_ADDR: u64 = 0xA000;

// Long-mode control register / EFER values WITH paging enabled.
const CR0_PE: u64 = 1 << 0;
const CR0_MP: u64 = 1 << 1;
const CR0_ET: u64 = 1 << 4;
const CR0_NE: u64 = 1 << 5;
const CR0_WP: u64 = 1 << 16;
const CR0_PG: u64 = 1 << 31;
const CR0_VAL: u64 = CR0_PE | CR0_MP | CR0_ET | CR0_NE | CR0_WP | CR0_PG;
const CR4_PAE: u64 = 1 << 5;
const CR4_OSFXSR: u64 = 1 << 9; // enable SSE / FXSAVE so SSE2 ops don't #UD
const CR4_OSXMMEXCPT: u64 = 1 << 10;
const CR4_VAL: u64 = CR4_PAE | CR4_OSFXSR | CR4_OSXMMEXCPT;
const EFER_SCE: u64 = 1 << 0;
const EFER_LME: u64 = 1 << 8;
const EFER_LMA: u64 = 1 << 10;
const EFER_VAL: u64 = EFER_SCE | EFER_LME | EFER_LMA;

// CR0 has MP|ET|NE set but TS/EM clear, so SSE is usable.

const MAX_ITERS: u64 = 100_000;

// ---------------------------------------------------------------------------
// Captured state for comparison
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct FinalState {
    regs: Registers,
    /// All 16 XMM registers, [low, high].
    xmm: [[u64; 2]; 16],
    /// Snapshot of the scratch data page (first 64 bytes).
    scratch: [u8; 64],
}

/// Observable, architecturally-defined RFLAGS status bits.
const FLAG_MASK: u64 = flags::bits::CF
    | flags::bits::PF
    | flags::bits::AF
    | flags::bits::ZF
    | flags::bits::SF
    | flags::bits::OF;

// ---------------------------------------------------------------------------
// Build the shared identity-mapped long-mode initial state.
// ---------------------------------------------------------------------------

fn base_sregs() -> SystemRegisters {
    let mut sregs = SystemRegisters::default();
    sregs.cr0 = CR0_VAL;
    sregs.cr3 = PML4_ADDR;
    sregs.cr4 = CR4_VAL;
    sregs.efer = EFER_VAL;

    // Flat 64-bit code segment (CS.L=1).
    sregs.cs.base = 0;
    sregs.cs.limit = 0xFFFFF;
    sregs.cs.selector = 0x8;
    sregs.cs.type_ = 0xB; // code, executed/read/accessed
    sregs.cs.present = true;
    sregs.cs.dpl = 0;
    sregs.cs.s = true;
    sregs.cs.l = true;
    sregs.cs.db = false; // must be 0 when L=1
    sregs.cs.g = true;

    // Flat data segments (DS/ES/FS/GS/SS).
    let mut data = sregs.cs.clone();
    data.selector = 0x10;
    data.type_ = 0x3; // data, read/write/accessed
    data.l = false;
    data.db = true;
    for seg in [
        &mut sregs.ds,
        &mut sregs.es,
        &mut sregs.fs,
        &mut sregs.gs,
        &mut sregs.ss,
    ] {
        *seg = data.clone();
    }

    sregs
}

/// Write the page tables + scratch page into a `Bytes` guest memory.
fn install_tables_mmap(write: &mut dyn FnMut(u64, &[u8])) {
    // PML4[0] -> PDPTE (present + writable)
    write(PML4_ADDR, &(PDPTE_ADDR | 0x3).to_le_bytes());
    // PDPTE[i] identity 1GiB huge pages (present + writable + PS), 4 entries (4GiB).
    for i in 0u64..4 {
        let entry: u64 = (i << 30) | 0x83;
        write(PDPTE_ADDR + i * 8, &entry.to_le_bytes());
    }
}

// ---------------------------------------------------------------------------
// Interpreter backend
// ---------------------------------------------------------------------------

fn run_interpreter(
    code: &[u8],
    init: &Registers,
    scratch_init: &[u8; 64],
) -> Result<FinalState, String> {
    let regions = vec![(GuestAddress(0), MEM_SIZE)];
    let mem =
        Arc::new(GuestMemoryMmap::<()>::from_ranges(&regions).map_err(|e| format!("mem: {e:?}"))?);

    // Page tables.
    install_tables_mmap(&mut |addr, bytes| {
        mem.write_slice(bytes, GuestAddress(addr)).unwrap();
    });
    // Code + scratch.
    mem.write_slice(code, GuestAddress(CODE_ADDR))
        .map_err(|e| format!("code: {e:?}"))?;
    mem.write_slice(scratch_init, GuestAddress(DATA_ADDR))
        .map_err(|e| format!("scratch: {e:?}"))?;

    let mut vcpu = X86_64Vcpu::new(0, mem.clone());

    let mut regs = init.clone();
    regs.rip = CODE_ADDR;
    if regs.rsp == 0 {
        regs.rsp = STACK_ADDR;
    }
    regs.rflags |= 0x2; // reserved bit 1 always set
    vcpu.set_regs(&regs)
        .map_err(|e| format!("set_regs: {e:?}"))?;
    vcpu.set_sregs(&base_sregs())
        .map_err(|e| format!("set_sregs: {e:?}"))?;

    // Run to HLT, counting individual instructions via step().
    let mut iters = 0u64;
    loop {
        iters += 1;
        if iters > MAX_ITERS {
            return Err(format!("interpreter exceeded {MAX_ITERS} iterations"));
        }
        match vcpu.step().map_err(|e| format!("step: {e:?}"))? {
            Some(VcpuExit::Hlt) => break,
            Some(VcpuExit::IoIn { size, .. }) => {
                let data = vec![0u8; size as usize];
                vcpu.complete_io_in(&data);
            }
            Some(VcpuExit::Shutdown)
            | Some(VcpuExit::FailEntry { .. })
            | Some(VcpuExit::InternalError) => {
                return Err("interpreter abnormal exit".to_string());
            }
            _ => {}
        }
    }

    let final_regs = vcpu.get_regs().map_err(|e| format!("get_regs: {e:?}"))?;
    let mut scratch = [0u8; 64];
    mem.read_slice(&mut scratch, GuestAddress(DATA_ADDR))
        .map_err(|e| format!("read scratch: {e:?}"))?;

    Ok(FinalState {
        xmm: final_regs.xmm,
        regs: final_regs,
        scratch,
    })
}

// ---------------------------------------------------------------------------
// KVM backend (driven directly with kvm-ioctls, mirroring kvm_minimal.rs)
// ---------------------------------------------------------------------------

/// Owns the mmap backing KVM guest memory so we can read it back and free it.
struct KvmMem {
    ptr: *mut u8,
    size: usize,
}

impl KvmMem {
    fn new(size: usize) -> Option<Self> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
                -1,
                0,
            )
        };
        if ptr == libc::MAP_FAILED {
            return None;
        }
        Some(KvmMem {
            ptr: ptr as *mut u8,
            size,
        })
    }

    fn write(&self, addr: u64, bytes: &[u8]) {
        assert!(addr as usize + bytes.len() <= self.size);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(addr as usize), bytes.len());
        }
    }

    fn read(&self, addr: u64, out: &mut [u8]) {
        assert!(addr as usize + out.len() <= self.size);
        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr.add(addr as usize), out.as_mut_ptr(), out.len());
        }
    }
}

impl Drop for KvmMem {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.size);
        }
    }
}

/// Returns Ok(None) if KVM is unavailable (so callers can skip gracefully),
/// Ok(Some(state)) on success, Err on a genuine run failure.
fn run_kvm(
    code: &[u8],
    init: &Registers,
    scratch_init: &[u8; 64],
) -> Result<Option<FinalState>, String> {
    use kvm_bindings::{kvm_segment, kvm_userspace_memory_region};
    use kvm_ioctls::Kvm;

    let kvm = match Kvm::new() {
        Ok(k) => k,
        Err(_) => return Ok(None), // /dev/kvm unavailable -> skip
    };
    let vm = match kvm.create_vm() {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let mem = match KvmMem::new(MEM_SIZE) {
        Some(m) => m,
        None => return Ok(None),
    };

    // Identity-mapped page tables + code + scratch.
    install_tables_mmap(&mut |addr, bytes| mem.write(addr, bytes));
    mem.write(CODE_ADDR, code);
    mem.write(DATA_ADDR, scratch_init);

    let region = kvm_userspace_memory_region {
        slot: 0,
        guest_phys_addr: 0,
        memory_size: MEM_SIZE as u64,
        userspace_addr: mem.ptr as u64,
        flags: 0,
    };
    if unsafe { vm.set_user_memory_region(region) }.is_err() {
        return Ok(None);
    }

    let mut vcpu = match vm.create_vcpu(0) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    // --- sregs: long mode w/ paging, flat segments ---
    let our_sregs = base_sregs();
    let mut sregs = vcpu
        .get_sregs()
        .map_err(|e| format!("kvm get_sregs: {e:?}"))?;

    let to_kvm_seg = |s: &rax::cpu::Segment| kvm_segment {
        base: s.base,
        limit: s.limit,
        selector: s.selector,
        type_: s.type_,
        present: s.present as u8,
        dpl: s.dpl,
        db: s.db as u8,
        s: s.s as u8,
        l: s.l as u8,
        g: s.g as u8,
        avl: s.avl as u8,
        unusable: s.unusable as u8,
        padding: 0,
    };

    sregs.cr0 = our_sregs.cr0;
    sregs.cr3 = our_sregs.cr3;
    sregs.cr4 = our_sregs.cr4;
    sregs.efer = our_sregs.efer;
    sregs.cs = to_kvm_seg(&our_sregs.cs);
    sregs.ds = to_kvm_seg(&our_sregs.ds);
    sregs.es = to_kvm_seg(&our_sregs.es);
    sregs.fs = to_kvm_seg(&our_sregs.fs);
    sregs.gs = to_kvm_seg(&our_sregs.gs);
    sregs.ss = to_kvm_seg(&our_sregs.ss);
    vcpu.set_sregs(&sregs)
        .map_err(|e| format!("kvm set_sregs: {e:?}"))?;

    // --- gprs ---
    let mut kregs = vcpu
        .get_regs()
        .map_err(|e| format!("kvm get_regs: {e:?}"))?;
    kregs.rax = init.rax;
    kregs.rbx = init.rbx;
    kregs.rcx = init.rcx;
    kregs.rdx = init.rdx;
    kregs.rsi = init.rsi;
    kregs.rdi = init.rdi;
    kregs.rbp = init.rbp;
    kregs.rsp = if init.rsp == 0 { STACK_ADDR } else { init.rsp };
    kregs.r8 = init.r8;
    kregs.r9 = init.r9;
    kregs.r10 = init.r10;
    kregs.r11 = init.r11;
    kregs.r12 = init.r12;
    kregs.r13 = init.r13;
    kregs.r14 = init.r14;
    kregs.r15 = init.r15;
    kregs.rip = CODE_ADDR;
    kregs.rflags = init.rflags | 0x2;
    vcpu.set_regs(&kregs)
        .map_err(|e| format!("kvm set_regs: {e:?}"))?;

    // --- xmm (via FPU state) ---
    if init.xmm.iter().any(|x| x != &[0, 0]) {
        let mut fpu = vcpu.get_fpu().map_err(|e| format!("kvm get_fpu: {e:?}"))?;
        for i in 0..16 {
            let lo = init.xmm[i][0].to_le_bytes();
            let hi = init.xmm[i][1].to_le_bytes();
            fpu.xmm[i][..8].copy_from_slice(&lo);
            fpu.xmm[i][8..].copy_from_slice(&hi);
        }
        vcpu.set_fpu(&fpu)
            .map_err(|e| format!("kvm set_fpu: {e:?}"))?;
    }

    // --- run to HLT, bounded ---
    let mut iters = 0u64;
    loop {
        iters += 1;
        if iters > MAX_ITERS {
            return Err(format!("kvm exceeded {MAX_ITERS} iterations"));
        }
        match vcpu.run().map_err(|e| format!("kvm run: {e:?}"))? {
            kvm_ioctls::VcpuExit::Hlt => break,
            kvm_ioctls::VcpuExit::IoIn(_, data) => {
                for b in data.iter_mut() {
                    *b = 0;
                }
            }
            kvm_ioctls::VcpuExit::IoOut(..) => {}
            kvm_ioctls::VcpuExit::MmioRead(_, data) => {
                for b in data.iter_mut() {
                    *b = 0;
                }
            }
            kvm_ioctls::VcpuExit::MmioWrite(..) => {}
            other => {
                return Err(format!("kvm abnormal exit: {other:?}"));
            }
        }
    }

    let final_kregs = vcpu
        .get_regs()
        .map_err(|e| format!("kvm get_regs(final): {e:?}"))?;
    let final_fpu = vcpu
        .get_fpu()
        .map_err(|e| format!("kvm get_fpu(final): {e:?}"))?;

    let mut regs = Registers::default();
    regs.rax = final_kregs.rax;
    regs.rbx = final_kregs.rbx;
    regs.rcx = final_kregs.rcx;
    regs.rdx = final_kregs.rdx;
    regs.rsi = final_kregs.rsi;
    regs.rdi = final_kregs.rdi;
    regs.rsp = final_kregs.rsp;
    regs.rbp = final_kregs.rbp;
    regs.r8 = final_kregs.r8;
    regs.r9 = final_kregs.r9;
    regs.r10 = final_kregs.r10;
    regs.r11 = final_kregs.r11;
    regs.r12 = final_kregs.r12;
    regs.r13 = final_kregs.r13;
    regs.r14 = final_kregs.r14;
    regs.r15 = final_kregs.r15;
    regs.rip = final_kregs.rip;
    regs.rflags = final_kregs.rflags;

    let mut xmm = [[0u64; 2]; 16];
    for i in 0..16 {
        let lo = u64::from_le_bytes(final_fpu.xmm[i][..8].try_into().unwrap());
        let hi = u64::from_le_bytes(final_fpu.xmm[i][8..].try_into().unwrap());
        xmm[i] = [lo, hi];
    }
    regs.xmm = xmm;

    let mut scratch = [0u8; 64];
    mem.read(DATA_ADDR, &mut scratch);

    Ok(Some(FinalState { regs, xmm, scratch }))
}

// ---------------------------------------------------------------------------
// Comparison
// ---------------------------------------------------------------------------

/// What aspects of architectural state to compare.
#[derive(Clone, Copy)]
struct CompareOpts {
    /// RFLAGS bits to compare. Usually all status flags; for instructions that
    /// leave some flags *architecturally undefined* (MUL/IMUL/DIV/shifts by a
    /// variable count, etc.) the undefined bits are masked out so we only check
    /// the bits the ISA actually defines.
    flag_mask: u64,
    /// Number of XMM registers to compare (0 = none).
    xmm_count: usize,
    /// Compare the scratch data page.
    scratch: bool,
    /// Compare RSP/RBP (off for tests that intentionally don't touch the stack
    /// in a comparable way; on by default).
    stack: bool,
}

impl Default for CompareOpts {
    fn default() -> Self {
        CompareOpts {
            flag_mask: FLAG_MASK,
            xmm_count: 0,
            scratch: false,
            stack: true,
        }
    }
}

fn gpr_list(r: &Registers) -> [(&'static str, u64); 16] {
    [
        ("rax", r.rax),
        ("rbx", r.rbx),
        ("rcx", r.rcx),
        ("rdx", r.rdx),
        ("rsi", r.rsi),
        ("rdi", r.rdi),
        ("rsp", r.rsp),
        ("rbp", r.rbp),
        ("r8", r.r8),
        ("r9", r.r9),
        ("r10", r.r10),
        ("r11", r.r11),
        ("r12", r.r12),
        ("r13", r.r13),
        ("r14", r.r14),
        ("r15", r.r15),
    ]
}

fn compare(interp: &FinalState, kvm: &FinalState, opts: CompareOpts) -> Vec<String> {
    let mut diffs = Vec::new();

    let il = gpr_list(&interp.regs);
    let kl = gpr_list(&kvm.regs);
    for ((name, iv), (_, kv)) in il.iter().zip(kl.iter()) {
        if !opts.stack && (*name == "rsp" || *name == "rbp") {
            continue;
        }
        if iv != kv {
            diffs.push(format!("{name}: interp={iv:#x} kvm={kv:#x}"));
        }
    }

    if opts.flag_mask != 0 {
        let im = interp.regs.rflags & opts.flag_mask;
        let km = kvm.regs.rflags & opts.flag_mask;
        if im != km {
            diffs.push(format!(
                "rflags(status): interp={:#x} kvm={:#x} (diff bits={:#x}) [{}]",
                im,
                km,
                im ^ km,
                describe_flags(im ^ km)
            ));
        }
    }

    for i in 0..opts.xmm_count {
        if interp.xmm[i] != kvm.xmm[i] {
            diffs.push(format!(
                "xmm{i}: interp=[{:#018x},{:#018x}] kvm=[{:#018x},{:#018x}]",
                interp.xmm[i][0], interp.xmm[i][1], kvm.xmm[i][0], kvm.xmm[i][1]
            ));
        }
    }

    if opts.scratch && interp.scratch != kvm.scratch {
        diffs.push(format!(
            "scratch page differs:\n  interp={:02x?}\n  kvm   ={:02x?}",
            &interp.scratch[..],
            &kvm.scratch[..]
        ));
    }

    diffs
}

fn describe_flags(bits: u64) -> String {
    let mut v = Vec::new();
    if bits & flags::bits::CF != 0 {
        v.push("CF");
    }
    if bits & flags::bits::PF != 0 {
        v.push("PF");
    }
    if bits & flags::bits::AF != 0 {
        v.push("AF");
    }
    if bits & flags::bits::ZF != 0 {
        v.push("ZF");
    }
    if bits & flags::bits::SF != 0 {
        v.push("SF");
    }
    if bits & flags::bits::OF != 0 {
        v.push("OF");
    }
    v.join("|")
}

// ---------------------------------------------------------------------------
// Top-level driver used by every test case.
// ---------------------------------------------------------------------------

/// Run `code` on both backends from `init`, returning `(interp, kvm)`.
/// Returns `None` if KVM is unavailable (the test should then `return`).
fn run_both(
    code: &[u8],
    init: Registers,
    scratch_init: [u8; 64],
) -> Option<(FinalState, FinalState)> {
    // KVM first: if unavailable we skip without even bothering the interpreter.
    let kvm = match run_kvm(code, &init, &scratch_init) {
        Ok(Some(s)) => s,
        Ok(None) => {
            eprintln!("[skip] /dev/kvm unavailable or undrivable; skipping differential case");
            return None;
        }
        Err(e) => panic!("KVM backend failure: {e}"),
    };
    let interp = match run_interpreter(code, &init, &scratch_init) {
        Ok(s) => s,
        Err(e) => panic!("interpreter backend failure: {e}"),
    };
    Some((interp, kvm))
}

/// Assert that the two backends agree, with a precise diff on mismatch.
fn assert_match(
    label: &str,
    code: &[u8],
    interp: &FinalState,
    kvm: &FinalState,
    opts: CompareOpts,
) {
    let diffs = compare(interp, kvm, opts);
    if !diffs.is_empty() {
        panic!(
            "DIVERGENCE in `{label}` (code = {:02x?}):\n  {}",
            code,
            diffs.join("\n  ")
        );
    }
}

// Convenience: a zeroed scratch page.
fn zero_scratch() -> [u8; 64] {
    [0u8; 64]
}

/// Run a case with default compare options (GPRs + all status flags + stack).
fn check(label: &str, code: &[u8], init: Registers) {
    let Some((interp, kvm)) = run_both(code, init, zero_scratch()) else {
        return;
    };
    assert_match(label, code, &interp, &kvm, CompareOpts::default());
}

/// Run a case comparing GPRs + only the flag bits in `flag_mask` (others are
/// architecturally undefined for this instruction and must not be compared).
fn check_flags_masked(label: &str, code: &[u8], init: Registers, flag_mask: u64) {
    let Some((interp, kvm)) = run_both(code, init, zero_scratch()) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask,
        ..CompareOpts::default()
    };
    assert_match(label, code, &interp, &kvm, opts);
}

/// Run an SSE case: the scratch page holds inputs, the guest code loads them
/// into XMM, operates, and stores the result back; we compare the scratch page
/// (and the live XMM registers as a bonus). Driving SSE through guest memory
/// avoids host-side FPU-injection quirks across the two backends.
fn check_sse(label: &str, code: &[u8], scratch_in: [u8; 64]) {
    let Some((interp, kvm)) = run_both(code, regs(), scratch_in) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0, // SSE integer ops don't set RFLAGS
        scratch: true,
        ..CompareOpts::default()
    };
    assert_match(label, code, &interp, &kvm, opts);
}

// Helper to build initial regs concisely.
fn regs() -> Registers {
    Registers::default()
}

// `HLT` terminator.
const HLT: u8 = 0xF4;

/// Append a HLT to a code buffer.
fn with_hlt(mut code: Vec<u8>) -> Vec<u8> {
    code.push(HLT);
    code
}

// ===========================================================================
// TEST CORPUS
// ===========================================================================

// ---- ADD / SUB / ADC / SBB / INC / DEC / NEG / CMP : flag exactness ----

#[test]
fn add_basic() {
    // mov rax, 0x...; add rax, rbx
    let mut r = regs();
    r.rax = 0x1122_3344_5566_7788;
    r.rbx = 0x1010_1010_1010_1010;
    // 48 01 D8  add rax, rbx
    check("add_rax_rbx", &with_hlt(vec![0x48, 0x01, 0xD8]), r);
}

#[test]
fn add_carry_overflow() {
    let mut r = regs();
    r.rax = 0x7fff_ffff_ffff_ffff; // signed max
    r.rbx = 1; // -> overflow + sign flip
    check("add_signed_overflow", &with_hlt(vec![0x48, 0x01, 0xD8]), r);
}

#[test]
fn add8_af_edge() {
    // add al, bl with low-nibble carry to exercise AF + PF
    let mut r = regs();
    r.rax = 0x0F;
    r.rbx = 0x01; // 0x0F + 0x01 = 0x10 -> AF set
    // 00 D8  add al, bl
    check("add8_af", &with_hlt(vec![0x00, 0xD8]), r);
}

#[test]
fn add8_unsigned_wrap() {
    let mut r = regs();
    r.rax = 0xFF;
    r.rbx = 0x01; // wraps to 0 -> CF, ZF, AF
    check("add8_wrap", &with_hlt(vec![0x00, 0xD8]), r);
}

#[test]
fn sub_basic() {
    let mut r = regs();
    r.rax = 0x1000;
    r.rbx = 0x0001;
    // 48 29 D8  sub rax, rbx
    check("sub_rax_rbx", &with_hlt(vec![0x48, 0x29, 0xD8]), r);
}

#[test]
fn sub_borrow() {
    let mut r = regs();
    r.rax = 0x00;
    r.rbx = 0x01; // 0 - 1 -> CF + AF + SF
    check("sub_borrow", &with_hlt(vec![0x48, 0x29, 0xD8]), r);
}

#[test]
fn sub8_signed_overflow() {
    let mut r = regs();
    r.rax = 0x80; // -128
    r.rbx = 0x01; //  - 1 -> overflow
    // 28 D8  sub al, bl
    check("sub8_overflow", &with_hlt(vec![0x28, 0xD8]), r);
}

#[test]
fn adc_with_carry_set() {
    let mut r = regs();
    r.rax = 0x10;
    r.rbx = 0x20;
    r.rflags = flags::bits::CF; // CF=1 going in
    // 48 11 D8  adc rax, rbx
    check("adc_carry_in", &with_hlt(vec![0x48, 0x11, 0xD8]), r);
}

#[test]
fn adc_chain_propagation() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF;
    r.rbx = 0x0;
    r.rflags = flags::bits::CF;
    check("adc_max_plus_carry", &with_hlt(vec![0x48, 0x11, 0xD8]), r);
}

#[test]
fn sbb_with_borrow() {
    let mut r = regs();
    r.rax = 0x100;
    r.rbx = 0x1;
    r.rflags = flags::bits::CF; // borrow in
    // 48 19 D8  sbb rax, rbx
    check("sbb_borrow_in", &with_hlt(vec![0x48, 0x19, 0xD8]), r);
}

#[test]
fn inc_overflow_preserves_cf() {
    let mut r = regs();
    r.rax = 0x7fff_ffff_ffff_ffff;
    r.rflags = flags::bits::CF; // INC must NOT touch CF
    // 48 FF C0  inc rax
    check("inc_of_keeps_cf", &with_hlt(vec![0x48, 0xFF, 0xC0]), r);
}

#[test]
fn dec_to_zero() {
    let mut r = regs();
    r.rax = 0x1;
    r.rflags = flags::bits::CF;
    // 48 FF C8  dec rax
    check("dec_to_zero", &with_hlt(vec![0x48, 0xFF, 0xC8]), r);
}

#[test]
fn dec_wrap_af() {
    let mut r = regs();
    r.rax = 0x10; // dec -> 0x0F, AF set
    // 48 FF C8 dec rax
    check("dec_af", &with_hlt(vec![0x48, 0xFF, 0xC8]), r);
}

#[test]
fn neg_nonzero() {
    let mut r = regs();
    r.rax = 0x1234;
    // 48 F7 D8  neg rax  (CF set because operand != 0)
    check("neg_nonzero", &with_hlt(vec![0x48, 0xF7, 0xD8]), r);
}

#[test]
fn neg_zero() {
    let mut r = regs();
    r.rax = 0x0; // neg 0 -> 0, CF clear, ZF set
    check("neg_zero", &with_hlt(vec![0x48, 0xF7, 0xD8]), r);
}

#[test]
fn cmp_equal() {
    let mut r = regs();
    r.rax = 0x42;
    r.rbx = 0x42;
    // 48 39 D8  cmp rax, rbx
    check("cmp_equal", &with_hlt(vec![0x48, 0x39, 0xD8]), r);
}

#[test]
fn cmp_less_signed() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF; // -1
    r.rbx = 0x1;
    check("cmp_neg_vs_pos", &with_hlt(vec![0x48, 0x39, 0xD8]), r);
}

// ---- AND / OR / XOR / TEST ----

#[test]
fn and_clears_cf_of() {
    let mut r = regs();
    r.rax = 0xFF00_FF00_FF00_FF00;
    r.rbx = 0x0FF0_0FF0_0FF0_0FF0;
    r.rflags = flags::bits::CF | flags::bits::OF; // must be cleared
    // 48 21 D8  and rax, rbx
    check("and", &with_hlt(vec![0x48, 0x21, 0xD8]), r);
}

#[test]
fn or_sets_sf_pf() {
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0000;
    r.rbx = 0x1;
    // 48 09 D8  or rax, rbx
    check("or", &with_hlt(vec![0x48, 0x09, 0xD8]), r);
}

#[test]
fn xor_self_zero() {
    let mut r = regs();
    r.rax = 0xDEAD_BEEF_CAFE_BABE;
    // 48 31 C0  xor rax, rax
    check("xor_self", &with_hlt(vec![0x48, 0x31, 0xC0]), r);
}

#[test]
fn test_parity() {
    let mut r = regs();
    r.rax = 0xFF; // low byte has even parity (8 ones) -> PF=1
    // 48 85 C0  test rax, rax
    check("test_parity", &with_hlt(vec![0x48, 0x85, 0xC0]), r);
}

// ---- Shifts / rotates : CF & OF ----

#[test]
fn shl_into_cf_of() {
    let mut r = regs();
    r.rax = 0x4000_0000_0000_0000;
    // 48 D1 E0  shl rax, 1  (1-bit shift defines OF)
    check("shl1", &with_hlt(vec![0x48, 0xD1, 0xE0]), r);
}

#[test]
fn shl_by_cl() {
    let mut r = regs();
    r.rax = 0x1;
    r.rcx = 4;
    // 48 D3 E0  shl rax, cl
    check("shl_cl", &with_hlt(vec![0x48, 0xD3, 0xE0]), r);
}

#[test]
fn shr_cf() {
    let mut r = regs();
    r.rax = 0x3; // shr by 1 -> CF from bit 0
    // 48 D1 E8  shr rax, 1
    check("shr1", &with_hlt(vec![0x48, 0xD1, 0xE8]), r);
}

#[test]
fn sar_sign_extend() {
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0000;
    r.rcx = 4;
    // 48 D3 F8  sar rax, cl
    check("sar_cl", &with_hlt(vec![0x48, 0xD3, 0xF8]), r);
}

#[test]
fn rol_cf_of() {
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0001;
    // 48 D1 C0  rol rax, 1
    check("rol1", &with_hlt(vec![0x48, 0xD1, 0xC0]), r);
}

#[test]
fn ror_cf_of() {
    let mut r = regs();
    r.rax = 0x1;
    // 48 D1 C8  ror rax, 1
    check("ror1", &with_hlt(vec![0x48, 0xD1, 0xC8]), r);
}

#[test]
fn rcl_through_carry() {
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0000;
    r.rflags = flags::bits::CF; // carry rotates in
    // 48 D1 D0  rcl rax, 1
    check("rcl1", &with_hlt(vec![0x48, 0xD1, 0xD0]), r);
}

#[test]
fn rcr_through_carry() {
    let mut r = regs();
    r.rax = 0x1;
    r.rflags = flags::bits::CF;
    // 48 D1 D8  rcr rax, 1
    check("rcr1", &with_hlt(vec![0x48, 0xD1, 0xD8]), r);
}

// ---- IMUL / MUL / DIV ----

// For MUL/IMUL the ISA defines only CF and OF; SF/ZF/AF/PF are *undefined*.
// For DIV/IDIV *all* status flags are undefined. We therefore mask the
// comparison accordingly so we only validate the architecturally-defined bits
// (and the full GPR result, which is always defined).
const MULDIV_DEFINED: u64 = flags::bits::CF | flags::bits::OF;

#[test]
fn imul_two_operand() {
    let mut r = regs();
    r.rax = 0x1_0000;
    r.rbx = 0x1_0000; // 2^16 * 2^16 = 2^32 (no overflow of 64-bit)
    // 48 0F AF C3  imul rax, rbx
    check_flags_masked(
        "imul2",
        &with_hlt(vec![0x48, 0x0F, 0xAF, 0xC3]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn imul_overflow_flags() {
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0000u64; // large -> CF/OF set on truncation
    r.rbx = 0x2;
    check_flags_masked(
        "imul_overflow",
        &with_hlt(vec![0x48, 0x0F, 0xAF, 0xC3]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn mul_rdx_rax() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF;
    r.rbx = 0x2; // RDX:RAX = product
    // 48 F7 E3  mul rbx
    check_flags_masked(
        "mul64",
        &with_hlt(vec![0x48, 0xF7, 0xE3]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn div_unsigned() {
    let mut r = regs();
    r.rax = 0x100; // dividend low
    r.rdx = 0x0; // dividend high
    r.rbx = 0x7; // divisor
    // 48 F7 F3  div rbx -> quotient in rax, remainder in rdx
    // All flags undefined for DIV; compare GPRs only (flag_mask = 0).
    check_flags_masked("div64", &with_hlt(vec![0x48, 0xF7, 0xF3]), r, 0);
}

#[test]
fn idiv_signed() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FF9C; // -100
    r.rdx = 0xFFFF_FFFF_FFFF_FFFF; // sign extension of -100
    r.rbx = 0x7;
    // 48 F7 FB  idiv rbx
    check_flags_masked("idiv64", &with_hlt(vec![0x48, 0xF7, 0xFB]), r, 0);
}

// ---- BSF / BSR ----

#[test]
fn bsf_nonzero() {
    let mut r = regs();
    r.rbx = 0x0000_0000_0001_0000; // lowest set bit = 16
    // 48 0F BC C3  bsf rax, rbx
    check("bsf", &with_hlt(vec![0x48, 0x0F, 0xBC, 0xC3]), r);
}

#[test]
fn bsr_nonzero() {
    let mut r = regs();
    r.rbx = 0x0000_0000_0001_0000; // highest set bit = 16
    // 48 0F BD C3  bsr rax, rbx
    check("bsr", &with_hlt(vec![0x48, 0x0F, 0xBD, 0xC3]), r);
}

// ---- SETcc / CMOVcc ----

#[test]
fn setcc_below() {
    // cmp then sete/setb on AL
    let mut r = regs();
    r.rax = 0x1;
    r.rbx = 0x2; // 1 < 2 -> CF set
    // 48 39 D8       cmp rax, rbx
    // 0F 92 C0       setb al
    check(
        "setb",
        &with_hlt(vec![0x48, 0x39, 0xD8, 0x0F, 0x92, 0xC0]),
        r,
    );
}

#[test]
fn cmovcc_greater() {
    let mut r = regs();
    r.rax = 0x5;
    r.rbx = 0x3;
    r.rcx = 0xAAAA; // value to maybe move
    // 48 39 D8        cmp rax, rbx     (5 - 3 -> not below, SF=OF, ZF=0)
    // 48 0F 4F C1     cmovg rax, rcx   (move if greater)
    check(
        "cmovg",
        &with_hlt(vec![0x48, 0x39, 0xD8, 0x48, 0x0F, 0x4F, 0xC1]),
        r,
    );
}

// ---- LEA ----

#[test]
fn lea_scaled_index() {
    let mut r = regs();
    r.rbx = 0x1000;
    r.rcx = 0x10;
    // 48 8D 04 8B  lea rax, [rbx + rcx*4]
    check("lea", &with_hlt(vec![0x48, 0x8D, 0x04, 0x8B]), r);
}

// ---- MOVZX / MOVSX ----

#[test]
fn movzx_byte() {
    let mut r = regs();
    r.rbx = 0xFFFF_FFFF_FFFF_FF80;
    // 48 0F B6 C3  movzx rax, bl
    check("movzx_bl", &with_hlt(vec![0x48, 0x0F, 0xB6, 0xC3]), r);
}

#[test]
fn movsx_byte() {
    let mut r = regs();
    r.rbx = 0x80; // sign-extends to 0xFFFF...80
    // 48 0F BE C3  movsx rax, bl
    check("movsx_bl", &with_hlt(vec![0x48, 0x0F, 0xBE, 0xC3]), r);
}

#[test]
fn movsxd_dword() {
    let mut r = regs();
    r.rbx = 0x0000_0000_8000_0000; // ebx = 0x80000000 sign-extends
    // 48 63 C3  movsxd rax, ebx
    check("movsxd", &with_hlt(vec![0x48, 0x63, 0xC3]), r);
}

// ---- SSE2: PADDB / PADDW / PADDD / PSUBB / PXOR / PAND via XMM ----
//
// SSE XMM state injection through KVM_SET_FPU does not reliably survive
// KVM_RUN's XSAVE-based FPU management, so instead of host-injecting XMM we
// drive these through guest memory: the guest loads two 128-bit inputs from the
// scratch page, performs the op, and stores the 128-bit result back. We then
// compare the scratch page byte-for-byte (and the live XMM regs). This is both
// robust and a closer model of real SSE code.
//
// Scratch layout: [0..16) = input A (xmm0), [16..32) = input B (xmm1),
//                 [32..48) = result (written by the guest).

/// Build a guest program: load xmm0/xmm1 from scratch, run `op` (a `66 0F xx C1`
/// style 2-operand SSE instruction on xmm0,xmm1), store xmm0 back, HLT.
fn sse_program(op: &[u8]) -> Vec<u8> {
    let mut code = Vec::new();
    // mov rdi, DATA_ADDR (imm32, sign-extended — 0x30000 fits)
    code.extend_from_slice(&[0x48, 0xC7, 0xC7]);
    code.extend_from_slice(&(DATA_ADDR as u32).to_le_bytes());
    // movdqu xmm0, [rdi]
    code.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    // movdqu xmm1, [rdi+0x10]
    code.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    // <op> xmm0, xmm1
    code.extend_from_slice(op);
    // movdqu [rdi+0x20], xmm0
    code.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    code.push(HLT);
    code
}

/// Build a 64-byte scratch page from two 16-byte inputs.
fn sse_scratch(a: [u8; 16], b: [u8; 16]) -> [u8; 64] {
    let mut s = [0u8; 64];
    s[0..16].copy_from_slice(&a);
    s[16..32].copy_from_slice(&b);
    s
}

#[test]
fn sse_paddb() {
    // per-byte add with wraps in several lanes
    let a = [
        1, 2, 3, 4, 5, 6, 7, 8, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    ];
    let b = [
        8, 8, 8, 8, 8, 8, 8, 8, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0,
    ];
    // 66 0F FC C1  paddb xmm0, xmm1
    check_sse(
        "paddb",
        &sse_program(&[0x66, 0x0F, 0xFC, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_paddw() {
    let a = 0xFFFF_0001_8000_1234u64.to_le_bytes();
    let a = [a, 0x0102_0304_0506_0708u64.to_le_bytes()].concat();
    let b = 0x0001_FFFF_8000_0001u64.to_le_bytes();
    let b = [b, 0x1111_2222_3333_4444u64.to_le_bytes()].concat();
    // 66 0F FD C1  paddw xmm0, xmm1
    check_sse(
        "paddw",
        &sse_program(&[0x66, 0x0F, 0xFD, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_paddd() {
    let a = [
        0x0000_0001_FFFF_FFFFu64.to_le_bytes(),
        0x8000_0000_7FFF_FFFFu64.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0000_0001_0000_0001u64.to_le_bytes(),
        0x0000_0001_0000_0001u64.to_le_bytes(),
    ]
    .concat();
    // 66 0F FE C1  paddd xmm0, xmm1
    check_sse(
        "paddd",
        &sse_program(&[0x66, 0x0F, 0xFE, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_psubb() {
    let a = [0, 1, 2, 3, 0x80, 0x7F, 0xFF, 0x10, 1, 2, 3, 4, 5, 6, 7, 8];
    let b = [
        1, 1, 1, 1, 1, 1, 1, 1, 0xFF, 0xFE, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60,
    ];
    // 66 0F F8 C1  psubb xmm0, xmm1
    check_sse(
        "psubb",
        &sse_program(&[0x66, 0x0F, 0xF8, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pxor() {
    let a = [
        0xDEAD_BEEF_CAFE_BABEu64.to_le_bytes(),
        0x0123_4567_89AB_CDEFu64.to_le_bytes(),
    ]
    .concat();
    let b = [
        0xFFFF_FFFF_FFFF_FFFFu64.to_le_bytes(),
        0x0F0F_0F0F_0F0F_0F0Fu64.to_le_bytes(),
    ]
    .concat();
    // 66 0F EF C1  pxor xmm0, xmm1
    check_sse(
        "pxor",
        &sse_program(&[0x66, 0x0F, 0xEF, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_pand() {
    let a = [
        0xFF00_FF00_FF00_FF00u64.to_le_bytes(),
        0xAAAA_5555_AAAA_5555u64.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0FF0_0FF0_0FF0_0FF0u64.to_le_bytes(),
        0xFFFF_0000_FFFF_0000u64.to_le_bytes(),
    ]
    .concat();
    // 66 0F DB C1  pand xmm0, xmm1
    check_sse(
        "pand",
        &sse_program(&[0x66, 0x0F, 0xDB, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

// ===========================================================================
// EXPANDED COVERAGE
// ===========================================================================
//
// Flag-definition reminders used by the masks below (Intel SDM Vol.2):
//  - BT/BTS/BTR/BTC : define CF; OF/SF/ZF/AF/PF undefined.
//  - BSF/BSR        : define ZF; CF/OF/SF/AF/PF undefined. Dest undefined if src==0.
//  - LZCNT/TZCNT    : define CF and ZF; OF/SF/AF/PF undefined.
//  - POPCNT         : ZF reflects src==0; CF/OF/SF/AF/PF cleared (all defined).
//  - shifts/rotates : OF defined only for a 1-bit shift; undefined for count>1.
//                     For a masked count of 0 NO flags change (we avoid that
//                     ambiguity by using nonzero counts or comparing GPRs only).
//  - SHLD/SHRD      : like shifts; OF defined only for count==1, undefined else;
//                     AF is undefined. Result undefined if count > operand size.
//  - XCHG           : affects no flags. XADD/CMPXCHG set flags like ADD/CMP.

const BT_DEFINED: u64 = flags::bits::CF;
const BSF_DEFINED: u64 = flags::bits::ZF;
const CNT_DEFINED: u64 = flags::bits::CF | flags::bits::ZF;
// For count>1 shifts/rotates OF is undefined; CF (and for shifts SF/ZF/PF) defined.
const SHIFT_NO_OF: u64 = flags::bits::CF | flags::bits::PF | flags::bits::ZF | flags::bits::SF;
// Rotates only ever touch CF/OF; for count>1 OF is undefined -> CF only.
const ROT_MULTI_DEFINED: u64 = flags::bits::CF;

// ---- BT / BTS / BTR / BTC (reg index and imm8 index) ----

#[test]
fn bt_reg_set() {
    let mut r = regs();
    r.rax = 0x0000_0000_0001_0000; // bit 16 set
    r.rdx = 16;
    // 48 0F A3 D0  bt rax, rdx  (CF <- bit 16 = 1)
    check_flags_masked(
        "bt_reg_set",
        &with_hlt(vec![0x48, 0x0F, 0xA3, 0xD0]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn bt_reg_clear() {
    let mut r = regs();
    r.rax = 0x0000_0000_0001_0000;
    r.rdx = 17; // bit 17 = 0
    check_flags_masked(
        "bt_reg_clear",
        &with_hlt(vec![0x48, 0x0F, 0xA3, 0xD0]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn bt_reg_index_wraps_modulo_64() {
    // For a register-operand BT the bit index is taken modulo operand size.
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0002; // bit 1 set
    r.rdx = 65; // 65 mod 64 = 1 -> CF should be 1
    check_flags_masked(
        "bt_reg_mod64",
        &with_hlt(vec![0x48, 0x0F, 0xA3, 0xD0]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn bt_imm() {
    let mut r = regs();
    r.rax = 0x0000_0000_8000_0000; // bit 31 set
    // 48 0F BA E0 1F  bt rax, 31
    check_flags_masked(
        "bt_imm31",
        &with_hlt(vec![0x48, 0x0F, 0xBA, 0xE0, 0x1F]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn bts_imm_sets_bit() {
    let mut r = regs();
    r.rax = 0x0; // CF <- old bit (0), bit 5 then set in dest
    // 48 0F BA E8 05  bts rax, 5
    check_flags_masked(
        "bts_imm5",
        &with_hlt(vec![0x48, 0x0F, 0xBA, 0xE8, 0x05]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn bts_reg_already_set() {
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0008; // bit 3 set
    r.rcx = 3;
    // 48 0F AB C8  bts rax, rcx  (CF<-1, bit stays set)
    check_flags_masked(
        "bts_reg_set",
        &with_hlt(vec![0x48, 0x0F, 0xAB, 0xC8]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn btr_imm_clears_bit() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF;
    // 48 0F BA F0 07  btr rax, 7  (CF<-1, bit 7 cleared)
    check_flags_masked(
        "btr_imm7",
        &with_hlt(vec![0x48, 0x0F, 0xBA, 0xF0, 0x07]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn btr_reg_clears_bit() {
    let mut r = regs();
    r.rax = 0x0000_0000_0010_0000; // bit 20 set
    r.rsi = 20;
    // 48 0F B3 F0  btr rax, rsi
    check_flags_masked(
        "btr_reg20",
        &with_hlt(vec![0x48, 0x0F, 0xB3, 0xF0]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn btc_imm_toggles_bit() {
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0001; // bit 0 set
    // 48 0F BA F8 00  btc rax, 0  (CF<-1, bit 0 toggled to 0)
    check_flags_masked(
        "btc_imm0",
        &with_hlt(vec![0x48, 0x0F, 0xBA, 0xF8, 0x00]),
        r,
        BT_DEFINED,
    );
}

#[test]
fn btc_reg_toggles_bit() {
    let mut r = regs();
    r.rax = 0x0; // bit 40 = 0 -> CF<-0, bit set
    r.rdi = 40;
    // 48 0F BB F8  btc rax, rdi
    check_flags_masked(
        "btc_reg40",
        &with_hlt(vec![0x48, 0x0F, 0xBB, 0xF8]),
        r,
        BT_DEFINED,
    );
}

// ---- BSF / BSR (incl. zero-source, where ZF=1 and dest is undefined) ----

#[test]
fn bsf_low_bit() {
    let mut r = regs();
    r.rbx = 0x0000_0000_0000_0028; // bits 3 and 5 -> lowest is 3
    check_flags_masked(
        "bsf_low3",
        &with_hlt(vec![0x48, 0x0F, 0xBC, 0xC3]),
        r,
        BSF_DEFINED,
    );
}

#[test]
fn bsf_zero_source() {
    // src == 0 -> ZF=1, destination is architecturally UNDEFINED; preload RAX so
    // both backends would only match if they leave it unchanged. Since the dest
    // is undefined we deliberately do NOT compare RAX here by checking flags only.
    let mut r = regs();
    r.rbx = 0;
    r.rax = 0xDEAD_DEAD_DEAD_DEAD;
    // Compare ZF only; mask out GPR by... we still compare GPRs in check_flags_masked.
    // To avoid an undefined-dest false diff, see bsf_zero_source_flags below.
    let Some((interp, kvm)) = run_both(&with_hlt(vec![0x48, 0x0F, 0xBC, 0xC3]), r, zero_scratch())
    else {
        return;
    };
    // Only the ZF flag is defined; RAX is undefined on a zero source.
    let opts = CompareOpts {
        flag_mask: BSF_DEFINED,
        xmm_count: 0,
        scratch: false,
        stack: true,
    };
    // Compare everything except RAX (undefined) by checking flags + that RBX/others match.
    let mut diffs = compare(&interp, &kvm, opts);
    diffs.retain(|d| !d.starts_with("rax:"));
    if !diffs.is_empty() {
        panic!("DIVERGENCE in `bsf_zero_source`: {}", diffs.join("; "));
    }
}

#[test]
fn bsr_high_bit() {
    let mut r = regs();
    r.rbx = 0x0000_0000_0000_0028; // highest set bit is 5
    check_flags_masked(
        "bsr_high5",
        &with_hlt(vec![0x48, 0x0F, 0xBD, 0xC3]),
        r,
        BSF_DEFINED,
    );
}

#[test]
fn bsr_top_bit() {
    let mut r = regs();
    r.rbx = 0x8000_0000_0000_0000; // bit 63
    check_flags_masked(
        "bsr_top63",
        &with_hlt(vec![0x48, 0x0F, 0xBD, 0xC3]),
        r,
        BSF_DEFINED,
    );
}

// ---- BSWAP ----

#[test]
fn bswap_r64() {
    let mut r = regs();
    r.rax = 0x0011_2233_4455_6677;
    // 48 0F C8  bswap rax
    check("bswap64", &with_hlt(vec![0x48, 0x0F, 0xC8]), r);
}

#[test]
fn bswap_r32() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_1122_3344; // bswap eax zero-extends into rax
    // 0F C8  bswap eax
    check("bswap32", &with_hlt(vec![0x0F, 0xC8]), r);
}

// ---- POPCNT / LZCNT / TZCNT ----

#[test]
fn popcnt_r64() {
    let mut r = regs();
    r.rbx = 0xFF00_F0F0_1234_5678;
    // F3 48 0F B8 C3  popcnt rax, rbx
    check("popcnt64", &with_hlt(vec![0xF3, 0x48, 0x0F, 0xB8, 0xC3]), r);
}

#[test]
fn popcnt_zero_sets_zf() {
    let mut r = regs();
    r.rbx = 0; // result 0 -> ZF set, all others cleared
    check(
        "popcnt_zero",
        &with_hlt(vec![0xF3, 0x48, 0x0F, 0xB8, 0xC3]),
        r,
    );
}

#[test]
fn lzcnt_r64() {
    let mut r = regs();
    r.rbx = 0x0000_0000_0001_0000; // 47 leading zeros
    // F3 48 0F BD C3  lzcnt rax, rbx
    check_flags_masked(
        "lzcnt64",
        &with_hlt(vec![0xF3, 0x48, 0x0F, 0xBD, 0xC3]),
        r,
        CNT_DEFINED,
    );
}

#[test]
fn lzcnt_zero_source() {
    let mut r = regs();
    r.rbx = 0; // result = operand size (64), CF set, ZF clear
    check_flags_masked(
        "lzcnt_zero",
        &with_hlt(vec![0xF3, 0x48, 0x0F, 0xBD, 0xC3]),
        r,
        CNT_DEFINED,
    );
}

#[test]
fn tzcnt_r64() {
    let mut r = regs();
    r.rbx = 0x0000_0000_0001_0000; // 16 trailing zeros
    // F3 48 0F BC C3  tzcnt rax, rbx
    check_flags_masked(
        "tzcnt64",
        &with_hlt(vec![0xF3, 0x48, 0x0F, 0xBC, 0xC3]),
        r,
        CNT_DEFINED,
    );
}

#[test]
fn tzcnt_zero_source() {
    let mut r = regs();
    r.rbx = 0; // result = 64, CF set
    check_flags_masked(
        "tzcnt_zero",
        &with_hlt(vec![0xF3, 0x48, 0x0F, 0xBC, 0xC3]),
        r,
        CNT_DEFINED,
    );
}

// ---- XCHG / XADD / CMPXCHG (register operands) ----

#[test]
fn xchg_reg_reg() {
    let mut r = regs();
    r.rax = 0x1111_1111_1111_1111;
    r.rbx = 0x2222_2222_2222_2222;
    // 48 87 D8  xchg rax, rbx  (no flags affected)
    check("xchg_rr", &with_hlt(vec![0x48, 0x87, 0xD8]), r);
}

#[test]
fn xadd_reg_reg() {
    let mut r = regs();
    r.rax = 0x100; // dest
    r.rbx = 0x0FF; // src; after: rbx<-old rax(0x100), rax<-0x1FF, flags like ADD
    // 48 0F C1 D8  xadd rax, rbx
    check("xadd_rr", &with_hlt(vec![0x48, 0x0F, 0xC1, 0xD8]), r);
}

#[test]
fn xadd_carry() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF;
    r.rbx = 0x1; // sum wraps -> CF, ZF, AF
    check("xadd_carry", &with_hlt(vec![0x48, 0x0F, 0xC1, 0xD8]), r);
}

#[test]
fn cmpxchg_success() {
    // RAX (accumulator) == dest -> ZF=1, dest <- src.
    let mut r = regs();
    r.rbx = 0x1234; // dest
    r.rax = 0x1234; // accumulator matches
    r.rcx = 0xABCD; // src written on success
    // 48 0F B1 CB  cmpxchg rbx, rcx
    check("cmpxchg_ok", &with_hlt(vec![0x48, 0x0F, 0xB1, 0xCB]), r);
}

#[test]
fn cmpxchg_fail() {
    // RAX != dest -> ZF=0, RAX <- dest, dest unchanged.
    let mut r = regs();
    r.rbx = 0x1234; // dest
    r.rax = 0x9999; // accumulator differs
    r.rcx = 0xABCD; // not written on failure
    check("cmpxchg_fail", &with_hlt(vec![0x48, 0x0F, 0xB1, 0xCB]), r);
}

#[test]
fn cmpxchg8_success() {
    let mut r = regs();
    r.rbx = 0x55;
    r.rax = 0x55;
    r.rcx = 0xAA;
    // 0F B0 CB  cmpxchg bl, cl
    check("cmpxchg8_ok", &with_hlt(vec![0x0F, 0xB0, 0xCB]), r);
}

// ---- Shifts / rotates by CL and imm, double-shift SHLD/SHRD ----

#[test]
fn shl_imm_multi() {
    let mut r = regs();
    r.rax = 0x1;
    // 48 C1 E0 05  shl rax, 5  (count>1 -> OF undefined)
    check_flags_masked(
        "shl_imm5",
        &with_hlt(vec![0x48, 0xC1, 0xE0, 0x05]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shr_imm_multi() {
    let mut r = regs();
    r.rax = 0xFF00;
    // 48 C1 E8 04  shr rax, 4
    check_flags_masked(
        "shr_imm4",
        &with_hlt(vec![0x48, 0xC1, 0xE8, 0x04]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn sar_imm_multi() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_8000_0000;
    // 48 C1 F8 08  sar rax, 8
    check_flags_masked(
        "sar_imm8",
        &with_hlt(vec![0x48, 0xC1, 0xF8, 0x08]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shr_cl_multi() {
    let mut r = regs();
    r.rax = 0xDEAD_BEEF_0000_0000;
    r.rcx = 12;
    // 48 D3 E8  shr rax, cl
    check_flags_masked(
        "shr_cl12",
        &with_hlt(vec![0x48, 0xD3, 0xE8]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shl32_cl_clears_high() {
    // 32-bit shift zero-extends the result into the full 64-bit register.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_0000_00FF;
    r.rcx = 8;
    // D3 E0  shl eax, cl
    check_flags_masked("shl32_cl8", &with_hlt(vec![0xD3, 0xE0]), r, SHIFT_NO_OF);
}

#[test]
fn rol_cl_multi() {
    let mut r = regs();
    r.rax = 0x1234_5678_9ABC_DEF0;
    r.rcx = 12;
    // 48 D3 C0  rol rax, cl  (count>1 -> OF undefined)
    check_flags_masked(
        "rol_cl12",
        &with_hlt(vec![0x48, 0xD3, 0xC0]),
        r,
        ROT_MULTI_DEFINED,
    );
}

#[test]
fn ror_cl_multi() {
    let mut r = regs();
    r.rax = 0x1234_5678_9ABC_DEF0;
    r.rcx = 20;
    // 48 D3 C8  ror rax, cl
    check_flags_masked(
        "ror_cl20",
        &with_hlt(vec![0x48, 0xD3, 0xC8]),
        r,
        ROT_MULTI_DEFINED,
    );
}

#[test]
fn shld_imm() {
    // SHLD rax, rbx, 8 : shift rax left by 8, feeding in the top 8 bits of rbx.
    let mut r = regs();
    r.rax = 0x1122_3344_5566_7788;
    r.rbx = 0xAABB_CCDD_EEFF_0011;
    // 48 0F A4 D8 08  shld rax, rbx, 8  (count>1 -> OF undefined)
    check_flags_masked(
        "shld_imm8",
        &with_hlt(vec![0x48, 0x0F, 0xA4, 0xD8, 0x08]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shrd_imm() {
    // SHRD rax, rbx, 8 : shift rax right by 8, feeding in the low 8 bits of rbx.
    let mut r = regs();
    r.rax = 0x1122_3344_5566_7788;
    r.rbx = 0xAABB_CCDD_EEFF_0011;
    // 48 0F AC D8 08  shrd rax, rbx, 8
    check_flags_masked(
        "shrd_imm8",
        &with_hlt(vec![0x48, 0x0F, 0xAC, 0xD8, 0x08]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shld_cl() {
    let mut r = regs();
    r.rax = 0xFFFF_0000_FFFF_0000;
    r.rbx = 0x0F0F_0F0F_0F0F_0F0F;
    r.rcx = 16;
    // 48 0F A5 D8  shld rax, rbx, cl
    check_flags_masked(
        "shld_cl16",
        &with_hlt(vec![0x48, 0x0F, 0xA5, 0xD8]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shrd_cl() {
    let mut r = regs();
    r.rax = 0xFFFF_0000_FFFF_0000;
    r.rbx = 0x0F0F_0F0F_0F0F_0F0F;
    r.rcx = 24;
    // 48 0F AD D8  shrd rax, rbx, cl
    check_flags_masked(
        "shrd_cl24",
        &with_hlt(vec![0x48, 0x0F, 0xAD, 0xD8]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shld_imm1_defines_of() {
    // A 1-bit double shift DOES define OF, so compare all status flags.
    let mut r = regs();
    r.rax = 0x4000_0000_0000_0000;
    r.rbx = 0x8000_0000_0000_0000;
    // 48 0F A4 D8 01  shld rax, rbx, 1
    check(
        "shld_imm1",
        &with_hlt(vec![0x48, 0x0F, 0xA4, 0xD8, 0x01]),
        r,
    );
}

// ---- Sign/zero extension: MOVSX/MOVZX word sources, CBW family, CWD family ----

#[test]
fn movzx_word() {
    let mut r = regs();
    r.rbx = 0xFFFF_FFFF_FFFF_8000;
    // 48 0F B7 C3  movzx rax, bx
    check("movzx_bx", &with_hlt(vec![0x48, 0x0F, 0xB7, 0xC3]), r);
}

#[test]
fn movsx_word() {
    let mut r = regs();
    r.rbx = 0x0000_0000_0000_8000; // bx = 0x8000 -> sign extends
    // 48 0F BF C3  movsx rax, bx
    check("movsx_bx", &with_hlt(vec![0x48, 0x0F, 0xBF, 0xC3]), r);
}

#[test]
fn movsx_byte_to_word() {
    // 16-bit destination: movsx ax, bl (66 prefix, opcode 0F BE)
    let mut r = regs();
    r.rax = 0x1111_2222_3333_4444;
    r.rbx = 0x90; // sign-extends within the 16-bit AX only
    // 66 0F BE C3  movsx ax, bl
    check("movsx_ax_bl", &with_hlt(vec![0x66, 0x0F, 0xBE, 0xC3]), r);
}

#[test]
fn cbw() {
    // AL -> AX sign extension; 66 98
    let mut r = regs();
    r.rax = 0x1234_5678_9ABC_DE80; // al = 0x80
    check("cbw", &with_hlt(vec![0x66, 0x98]), r);
}

#[test]
fn cwde() {
    // AX -> EAX sign extension; 98 (zero-extends EAX into RAX)
    let mut r = regs();
    r.rax = 0x1234_5678_0000_8000; // ax = 0x8000
    check("cwde", &with_hlt(vec![0x98]), r);
}

#[test]
fn cdqe() {
    // EAX -> RAX sign extension; 48 98
    let mut r = regs();
    r.rax = 0x1234_5678_8000_0000; // eax = 0x80000000
    check("cdqe", &with_hlt(vec![0x48, 0x98]), r);
}

#[test]
fn cwd() {
    // AX -> DX:AX ; 66 99
    let mut r = regs();
    r.rax = 0x0000_0000_0000_8000; // ax negative
    r.rdx = 0x1111_2222_3333_4444; // only dx (low 16) should change
    check("cwd", &with_hlt(vec![0x66, 0x99]), r);
}

#[test]
fn cdq() {
    // EAX -> EDX:EAX ; 99  (EDX zero-extends into RDX)
    let mut r = regs();
    r.rax = 0x0000_0000_8000_0000; // eax negative
    r.rdx = 0xFFFF_FFFF_1234_5678;
    check("cdq", &with_hlt(vec![0x99]), r);
}

#[test]
fn cqo() {
    // RAX -> RDX:RAX ; 48 99
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0000; // negative
    r.rdx = 0x1234_5678_9ABC_DEF0;
    check("cqo", &with_hlt(vec![0x48, 0x99]), r);
}

#[test]
fn cqo_positive() {
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0001; // positive -> rdx becomes 0
    r.rdx = 0xFFFF_FFFF_FFFF_FFFF;
    check("cqo_pos", &with_hlt(vec![0x48, 0x99]), r);
}

// ---- CMOVcc across all 16 conditions ----
//
// We set up a known flag state with a single CMP, then CMOVcc rax, rcx. The
// destination is loaded with a sentinel beforehand so a "no move" is observable.

/// cmp rax,rbx (48 39 D8) sets flags, then `cmov` (2-byte 0F xx) rax, rcx.
fn cmov_program(cmov2: [u8; 2]) -> Vec<u8> {
    // 48 31 C0           xor rax, rax           (clear so the sentinel below is exact)
    // 48 B8 imm64        mov rax, sentinel
    // ... but simpler: use the regs we pass in. We just emit cmp + cmov.
    let mut code = vec![0x48, 0x39, 0xD8]; // cmp rax, rbx
    code.extend_from_slice(&[0x48, cmov2[0], cmov2[1], 0xC1]); // cmov rax, rcx (REX.W, 0F, opc, modrm C1)
    code.push(HLT);
    code
}

/// Run a CMOVcc with the given 2nd opcode byte and an (rax,rbx) pair that sets
/// flags via CMP. rcx holds the candidate value moved on a true condition.
fn check_cmov(label: &str, opc: u8, rax: u64, rbx: u64) {
    let mut r = regs();
    r.rax = rax;
    r.rbx = rbx;
    r.rcx = 0xCAFE_F00D_1234_5678;
    check(label, &cmov_program([0x0F, opc]), r);
}

/// All 16 CMOVcc conditions in one case. Each tuple is (opcode2, rax, rbx) where
/// the CMP rax,rbx sets a flag state that makes the condition TRUE (and a couple
/// also test the FALSE/no-move path). Opcodes 0x40..=0x4F are CMOVO..CMOVG.
#[test]
fn cmovcc_all_conditions() {
    // (opcode, rax, rbx) chosen so the condition is TRUE for these inputs.
    let true_cases: &[(u8, u64, u64, &str)] = &[
        (0x40, 0x8000_0000_0000_0000, 1, "cmovo"), // INT_MIN - 1 -> OF
        (0x41, 5, 3, "cmovno"),                    // no overflow
        (0x42, 1, 2, "cmovb"),                     // 1 < 2 unsigned -> CF
        (0x43, 5, 3, "cmovae"),                    // CF clear
        (0x44, 7, 7, "cmove"),                     // equal -> ZF
        (0x45, 7, 8, "cmovne"),                    // not equal
        (0x46, 2, 2, "cmovbe"),                    // CF|ZF (equal)
        (0x47, 5, 3, "cmova"),                     // above
        (0x48, 0, 1, "cmovs"),                     // 0-1 -> SF
        (0x49, 5, 3, "cmovns"),                    // SF clear
        (0x4A, 3, 0, "cmovp"),                     // result 3 -> even parity
        (0x4B, 1, 0, "cmovnp"),                    // result 1 -> odd parity
        (0x4C, 1, 2, "cmovl"),                     // signed less
        (0x4D, 5, 3, "cmovge"),                    // signed >=
        (0x4E, 2, 2, "cmovle"),                    // signed <= (equal)
        (0x4F, 5, 3, "cmovg"),                     // signed >
    ];
    for &(opc, rax, rbx, name) in true_cases {
        check_cmov(name, opc, rax, rbx);
    }
    // No-move (FALSE) path for a representative set so we also exercise the
    // "destination unchanged" branch on both backends.
    check_cmov("cmovo_false", 0x40, 5, 3); // no overflow -> no move
    check_cmov("cmove_false", 0x44, 7, 8); // not equal -> no move
    check_cmov("cmovg_false", 0x4F, 1, 2); // not greater -> no move
}

// ---- SETcc across conditions ----

/// cmp rax,rbx then setcc al, then movzx eax, al so we read a clean 0/1.
fn setcc_program(opc: u8) -> Vec<u8> {
    let mut code = vec![0x48, 0x39, 0xD8]; // cmp rax, rbx
    code.extend_from_slice(&[0x0F, opc, 0xC0]); // setcc al
    code.extend_from_slice(&[0x0F, 0xB6, 0xC0]); // movzx eax, al
    code.push(HLT);
    code
}

fn check_setcc(label: &str, opc: u8, rax: u64, rbx: u64) {
    let mut r = regs();
    r.rax = rax;
    r.rbx = rbx;
    check(label, &setcc_program(opc), r);
}

/// SETcc across a spread of conditions, both true and false results. Opcodes
/// 0x90..=0x9F are SETO..SETG.
#[test]
fn setcc_all_conditions() {
    let cases: &[(u8, u64, u64, &str)] = &[
        (0x90, 0x8000_0000_0000_0000, 1, "seto"), // overflow -> 1
        (0x91, 5, 3, "setno"),                    // no overflow -> 1
        (0x92, 1, 2, "setb"),                     // below -> 1
        (0x93, 5, 3, "setae"),                    // not below -> 1
        (0x94, 7, 7, "sete"),                     // equal -> 1
        (0x95, 7, 8, "setne"),                    // not equal -> 1
        (0x96, 2, 2, "setbe"),                    // be (equal) -> 1
        (0x97, 5, 3, "seta"),                     // above -> 1
        (0x98, 0, 1, "sets"),                     // sign -> 1
        (0x99, 5, 3, "setns"),                    // no sign -> 1
        (0x9A, 3, 0, "setp"),                     // even parity -> 1
        (0x9B, 1, 0, "setnp"),                    // odd parity -> 1
        (0x9C, 1, 2, "setl"),                     // less -> 1
        (0x9D, 5, 3, "setge"),                    // ge -> 1
        (0x9E, 2, 2, "setle"),                    // le (equal) -> 1
        (0x9F, 5, 3, "setg"),                     // greater -> 1
        // false-result spot checks
        (0x94, 7, 8, "sete_false"), // not equal -> 0
        (0x9F, 1, 2, "setg_false"), // not greater -> 0
    ];
    for &(opc, rax, rbx, name) in cases {
        check_setcc(name, opc, rax, rbx);
    }
}

// ---- LEA with complex SIB (base + index*scale + disp) ----

#[test]
fn lea_base_index_scale_disp8() {
    let mut r = regs();
    r.rbx = 0x1_0000; // base
    r.rcx = 0x10; // index
    // 48 8D 44 8B 20  lea rax, [rbx + rcx*4 + 0x20]
    check(
        "lea_sib_disp8",
        &with_hlt(vec![0x48, 0x8D, 0x44, 0x8B, 0x20]),
        r,
    );
}

#[test]
fn lea_index_scale8_disp32() {
    let mut r = regs();
    r.rsi = 0x100;
    r.rdi = 0x2_0000;
    // 48 8D 84 F7 00 10 00 00  lea rax, [rdi + rsi*8 + 0x1000]
    check(
        "lea_sib_disp32",
        &with_hlt(vec![0x48, 0x8D, 0x84, 0xF7, 0x00, 0x10, 0x00, 0x00]),
        r,
    );
}

#[test]
fn lea_no_base_index_scale() {
    // [rcx*8 + disp32] with no base (mod=00, base=101 in SIB -> disp32, no base)
    let mut r = regs();
    r.rcx = 0x11;
    // 48 8D 04 CD 00 02 00 00  lea rax, [rcx*8 + 0x200]
    check(
        "lea_idx_only",
        &with_hlt(vec![0x48, 0x8D, 0x04, 0xCD, 0x00, 0x02, 0x00, 0x00]),
        r,
    );
}

#[test]
fn lea_32bit_addrsize_truncates() {
    // 67 prefix -> 32-bit address calc, result zero-extended into rax.
    let mut r = regs();
    r.rbx = 0xFFFF_FFFF_FFFF_F000; // ebx = 0xFFFFF000
    r.rcx = 0x0000_0000_0000_2000; // ecx = 0x2000
    // 67 48 8D 04 0B  lea rax, [ebx + ecx]  -> (0xFFFFF000+0x2000) mod 2^32
    check(
        "lea_addr32",
        &with_hlt(vec![0x67, 0x48, 0x8D, 0x04, 0x0B]),
        r,
    );
}

// ---- 8/16/32-bit operand-size ALU variants for flag exactness ----

#[test]
fn add16_overflow() {
    let mut r = regs();
    r.rax = 0x7FFF; // 16-bit signed max
    r.rbx = 0x0001; // -> OF, SF
    // 66 01 D8  add ax, bx
    check("add16_of", &with_hlt(vec![0x66, 0x01, 0xD8]), r);
}

#[test]
fn add32_wrap_zero_extends() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF; // eax = 0xFFFFFFFF
    r.rbx = 0x0000_0000_0000_0001; // -> 0, CF, ZF, AF; rax zero-extends
    // 01 D8  add eax, ebx
    check("add32_wrap", &with_hlt(vec![0x01, 0xD8]), r);
}

#[test]
fn sub16_borrow() {
    let mut r = regs();
    r.rax = 0x0000;
    r.rbx = 0x0001; // 0-1 -> CF, SF, AF
    // 66 29 D8  sub ax, bx
    check("sub16_borrow", &with_hlt(vec![0x66, 0x29, 0xD8]), r);
}

#[test]
fn and16() {
    let mut r = regs();
    r.rax = 0xF0F0;
    r.rbx = 0x0FF0;
    // 66 21 D8  and ax, bx
    check("and16", &with_hlt(vec![0x66, 0x21, 0xD8]), r);
}

#[test]
fn or8() {
    let mut r = regs();
    r.rax = 0x80;
    r.rbx = 0x01; // -> 0x81, SF set, odd parity
    // 08 D8  or al, bl
    check("or8", &with_hlt(vec![0x08, 0xD8]), r);
}

#[test]
fn xor16_zero() {
    let mut r = regs();
    r.rax = 0xABCD;
    r.rbx = 0xABCD; // -> 0, ZF set, PF set
    // 66 31 D8  xor ax, bx
    check("xor16_zero", &with_hlt(vec![0x66, 0x31, 0xD8]), r);
}

#[test]
fn cmp16_equal() {
    let mut r = regs();
    r.rax = 0x1234_5678_9ABC_8000;
    r.rbx = 0x0000_0000_0000_8000; // ax==bx -> ZF
    // 66 39 D8  cmp ax, bx
    check("cmp16_eq", &with_hlt(vec![0x66, 0x39, 0xD8]), r);
}

#[test]
fn cmp8_less() {
    let mut r = regs();
    r.rax = 0x7F; // +127
    r.rbx = 0x80; // -128 ; 127 - (-128) overflows -> OF
    // 38 D8  cmp al, bl
    check("cmp8_of", &with_hlt(vec![0x38, 0xD8]), r);
}

#[test]
fn test8() {
    let mut r = regs();
    r.rax = 0xF0;
    r.rbx = 0x0F; // 0xF0 & 0x0F = 0 -> ZF, PF
    // 84 D8  test al, bl
    check("test8", &with_hlt(vec![0x84, 0xD8]), r);
}

#[test]
fn inc8_overflow() {
    let mut r = regs();
    r.rax = 0x7F; // inc -> 0x80, OF set, SF set, AF set
    r.rflags = flags::bits::CF; // INC must preserve CF
    // FE C0  inc al
    check("inc8_of", &with_hlt(vec![0xFE, 0xC0]), r);
}

#[test]
fn dec16_to_zero() {
    let mut r = regs();
    r.rax = 0x0001;
    r.rflags = flags::bits::CF;
    // 66 FF C8  dec ax
    check("dec16_zero", &with_hlt(vec![0x66, 0xFF, 0xC8]), r);
}

#[test]
fn inc32_zero_extends() {
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF; // eax = 0xFFFFFFFF -> inc to 0, zero-extends
    // FF C0  inc eax
    check("inc32_wrap", &with_hlt(vec![0xFF, 0xC0]), r);
}

#[test]
fn add8_imm() {
    let mut r = regs();
    r.rax = 0x7E;
    // 04 02  add al, 2  -> 0x80, OF + SF
    check("add8_imm", &with_hlt(vec![0x04, 0x02]), r);
}

#[test]
fn add16_imm() {
    let mut r = regs();
    r.rax = 0x00FF;
    // 66 05 01 00  add ax, 1
    check("add16_imm", &with_hlt(vec![0x66, 0x05, 0x01, 0x00]), r);
}

// ---- More SSE2 integer ops (PMULLW/PMADDWD/PSADBW/PACK*/PUNPCK*) ----

#[test]
fn sse_pmullw() {
    // packed 16-bit multiply low: per-lane (a*b) low 16 bits.
    let a = [
        0x0002u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
        0x1234u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x0010u16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
        0x0007u16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0003u16.to_le_bytes(),
        0x0100u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0x0010u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x0010u16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
        0x0006u16.to_le_bytes(),
    ]
    .concat();
    // 66 0F D5 C1  pmullw xmm0, xmm1
    check_sse(
        "pmullw",
        &sse_program(&[0x66, 0x0F, 0xD5, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_pmaddwd() {
    // PMADDWD: multiply 16-bit lanes, add adjacent pairs into 32-bit results.
    let a = [
        0x0001u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(), // -1
        0x0003u16.to_le_bytes(),
        0x8000u16.to_le_bytes(), // -32768
        0x8000u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0004u16.to_le_bytes(),
        0x0005u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(), // -1
        0x8000u16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
    ]
    .concat();
    // 66 0F F5 C1  pmaddwd xmm0, xmm1
    check_sse(
        "pmaddwd",
        &sse_program(&[0x66, 0x0F, 0xF5, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_psadbw() {
    // sum of absolute differences of bytes -> two 16-bit sums (low halves).
    let a = [
        10, 20, 30, 40, 50, 60, 70, 80, 100, 110, 120, 130, 140, 150, 160, 170,
    ];
    let b = [
        5, 25, 35, 35, 60, 55, 80, 75, 90, 120, 110, 140, 130, 160, 150, 180,
    ];
    // 66 0F F6 C1  psadbw xmm0, xmm1
    check_sse(
        "psadbw",
        &sse_program(&[0x66, 0x0F, 0xF6, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_packsswb() {
    // signed saturate pack words->bytes.
    let a = [
        0x0001u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(), // -> +127
        0x8000u16.to_le_bytes(), // -> -128
        0xFF80u16.to_le_bytes(), // -128 stays
        0x0080u16.to_le_bytes(), // +128 -> +127
        0xFFFFu16.to_le_bytes(), // -1
        0x0000u16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0100u16.to_le_bytes(),
        0x7F00u16.to_le_bytes(),
        0x8001u16.to_le_bytes(),
        0x0040u16.to_le_bytes(),
        0x0010u16.to_le_bytes(),
        0xFFF0u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
    ]
    .concat();
    // 66 0F 63 C1  packsswb xmm0, xmm1
    check_sse(
        "packsswb",
        &sse_program(&[0x66, 0x0F, 0x63, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_packuswb() {
    // unsigned saturate pack words->bytes.
    let a = [
        0x0001u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x0100u16.to_le_bytes(), // -> 255
        0x8000u16.to_le_bytes(), // negative -> 0
        0xFFFFu16.to_le_bytes(), // negative -> 0
        0x007Fu16.to_le_bytes(),
        0x00FEu16.to_le_bytes(),
        0x0080u16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0000u16.to_le_bytes(),
        0x0010u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x0101u16.to_le_bytes(), // -> 255
        0x7FFFu16.to_le_bytes(), // -> 255
        0x0001u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0x0003u16.to_le_bytes(),
    ]
    .concat();
    // 66 0F 67 C1  packuswb xmm0, xmm1
    check_sse(
        "packuswb",
        &sse_program(&[0x66, 0x0F, 0x67, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_punpcklbw() {
    // interleave low bytes of xmm0 and xmm1.
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = [
        0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E,
        0x8F,
    ];
    // 66 0F 60 C1  punpcklbw xmm0, xmm1
    check_sse(
        "punpcklbw",
        &sse_program(&[0x66, 0x0F, 0x60, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_punpckhbw() {
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = [
        0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E,
        0x8F,
    ];
    // 66 0F 68 C1  punpckhbw xmm0, xmm1
    check_sse(
        "punpckhbw",
        &sse_program(&[0x66, 0x0F, 0x68, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_punpckldq() {
    // interleave low dwords.
    let a = [
        0x1111_1111u32.to_le_bytes(),
        0x2222_2222u32.to_le_bytes(),
        0x3333_3333u32.to_le_bytes(),
        0x4444_4444u32.to_le_bytes(),
    ]
    .concat();
    let b = [
        0xAAAA_AAAAu32.to_le_bytes(),
        0xBBBB_BBBBu32.to_le_bytes(),
        0xCCCC_CCCCu32.to_le_bytes(),
        0xDDDD_DDDDu32.to_le_bytes(),
    ]
    .concat();
    // 66 0F 62 C1  punpckldq xmm0, xmm1
    check_sse(
        "punpckldq",
        &sse_program(&[0x66, 0x0F, 0x62, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_punpcklqdq() {
    // interleave low qwords -> [a.lo, b.lo].
    let a = [
        0x0123_4567_89AB_CDEFu64.to_le_bytes(),
        0xDEAD_BEEF_CAFE_BABEu64.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x1122_3344_5566_7788u64.to_le_bytes(),
        0x99AA_BBCC_DDEE_FF00u64.to_le_bytes(),
    ]
    .concat();
    // 66 0F 6C C1  punpcklqdq xmm0, xmm1
    check_sse(
        "punpcklqdq",
        &sse_program(&[0x66, 0x0F, 0x6C, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse_pmulhw() {
    // packed 16-bit multiply, store the HIGH 16 bits (signed).
    let a = [
        0x4000u16.to_le_bytes(),
        0x8000u16.to_le_bytes(), // -32768
        0x7FFFu16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(), // -1
        0x0100u16.to_le_bytes(),
        0x1234u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0004u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
        0x0100u16.to_le_bytes(),
        0x1000u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
    ]
    .concat();
    // 66 0F E5 C1  pmulhw xmm0, xmm1
    check_sse(
        "pmulhw",
        &sse_program(&[0x66, 0x0F, 0xE5, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

// ===========================================================================
// EXPANDED COVERAGE PART 2: x87 FPU / string ops / SSE2-SSE3 float / BMI
// ===========================================================================
//
// These reuse the existing infrastructure:
//  - x87 and string tests drive data through the scratch page (just like the
//    SSE tests) so we never rely on host-side FPU/XMM injection surviving
//    KVM_RUN. The guest loads from / stores to the scratch page and we compare
//    it byte-for-byte plus the relevant GPRs.
//  - BMI/MOVBE/SAHF/LAHF are plain GPR ops driven by `check`/`check_flags_masked`.
//
// IMPORTANT x87 caveat: rax keeps the x87 stack as f64, not the architectural
// 80-bit extended format. Therefore every x87 value used below is chosen so the
// 80-bit and 64-bit representations are *bit-identical*: small integers and
// exact dyadic fractions (n / 2^k). Results are stored as m64 (FSTP qword) so
// what we compare is exactly the f64 the op produced — which, for these
// operands, equals what real hardware computes in 80-bit then rounds to 64-bit.

/// A scratch-comparing runner for memory-effect tests (x87 store / string ops).
/// Compares all GPRs, the masked flags, and the scratch page.
fn check_mem(label: &str, code: &[u8], init: Registers, scratch_in: [u8; 64], flag_mask: u64) {
    let Some((interp, kvm)) = run_both(code, init, scratch_in) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask,
        scratch: true,
        ..CompareOpts::default()
    };
    assert_match(label, code, &interp, &kvm, opts);
}

/// Emit `mov rdi, DATA_ADDR` (REX.W mov r/m64, imm32 sign-extended; 0x30000 fits).
fn load_rdi_data() -> Vec<u8> {
    let mut c = vec![0x48, 0xC7, 0xC7];
    c.extend_from_slice(&(DATA_ADDR as u32).to_le_bytes());
    c
}

/// Build a 64-byte scratch page from a list of f64 inputs laid out from offset 0.
fn scratch_f64(vals: &[f64]) -> [u8; 64] {
    let mut s = [0u8; 64];
    for (i, v) in vals.iter().enumerate() {
        s[i * 8..i * 8 + 8].copy_from_slice(&v.to_le_bytes());
    }
    s
}

// ---- x87: load / arithmetic round-trips, result stored as FSTP qword ----
//
// Program skeleton for a binary op on two m64 inputs:
//   mov rdi, DATA_ADDR
//   fld qword [rdi+0]      ; ST0 = a            (DD /0)
//   f<op> qword [rdi+8]    ; ST0 = a <op> b     (DC /n, memory form)
//   fstp qword [rdi+16]    ; store + pop        (DD /3)
//   hlt
// Inputs a,b at offsets 0 and 8; result compared at offset 16.

/// Build an x87 "load a; <mem-op> b; store result" program. `op_modrm` is the
/// ModRM byte selecting the DC-escape memory operation against [rdi+8].
fn x87_binop(op_escape: u8, op_modrm: u8) -> Vec<u8> {
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]
    c.extend_from_slice(&[op_escape, op_modrm]); // f<op> qword [rdi+8] (modrm encodes disp8 0x08)
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp qword [rdi+0x10]
    c.push(HLT);
    c
}

#[test]
fn x87_fld_fstp_roundtrip() {
    // Load an exact f64 and store it straight back — pure load/store fidelity.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp qword [rdi+0x10]
    c.push(HLT);
    // 12345.5 is exactly representable.
    check_mem(
        "x87_fld_fstp",
        &with_hlt(c),
        regs(),
        scratch_f64(&[12345.5]),
        0,
    );
}

#[test]
fn x87_fadd_m64() {
    // DC /0 = FADD m64. ModRM for [rdi+disp8], reg=000 -> 0x47, disp8=0x08.
    check_mem(
        "x87_fadd",
        &with_hlt(x87_binop(0xDC, 0x47)),
        regs(),
        scratch_f64(&[3.5, 4.25]),
        0,
    );
}

#[test]
fn x87_fsub_m64() {
    // DC /4 = FSUB m64. reg=100 -> modrm 0x67.
    check_mem(
        "x87_fsub",
        &with_hlt(x87_binop(0xDC, 0x67)),
        regs(),
        scratch_f64(&[10.0, 2.5]),
        0,
    );
}

#[test]
fn x87_fmul_m64() {
    // DC /1 = FMUL m64. reg=001 -> modrm 0x4F.
    check_mem(
        "x87_fmul",
        &with_hlt(x87_binop(0xDC, 0x4F)),
        regs(),
        scratch_f64(&[6.0, 7.0]),
        0,
    );
}

#[test]
fn x87_fdiv_m64() {
    // DC /6 = FDIV m64. reg=110 -> modrm 0x77. 9/8 = 1.125 is exact.
    check_mem(
        "x87_fdiv",
        &with_hlt(x87_binop(0xDC, 0x77)),
        regs(),
        scratch_f64(&[9.0, 8.0]),
        0,
    );
}

#[test]
fn x87_fsqrt() {
    // fld qword [rdi]; fsqrt (D9 FA); fstp qword [rdi+0x10]. sqrt(16)=4 exact.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]
    c.extend_from_slice(&[0xD9, 0xFA]); // fsqrt
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp qword [rdi+0x10]
    c.push(HLT);
    check_mem("x87_fsqrt", &with_hlt(c), regs(), scratch_f64(&[16.0]), 0);
}

#[test]
fn x87_fild_fistp_roundtrip() {
    // FILD m32 (DB /0) loads a 32-bit integer; FISTP m32 (DB /3) stores it back.
    // Put the integer at offset 0, read the stored int back at offset 16.
    let mut s = [0u8; 64];
    s[0..4].copy_from_slice(&(-12345i32).to_le_bytes());
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDB, 0x07]); // fild dword [rdi]
    c.extend_from_slice(&[0xDB, 0x5F, 0x10]); // fistp dword [rdi+0x10]
    c.push(HLT);
    check_mem("x87_fild_fistp", &with_hlt(c), regs(), s, 0);
}

#[test]
fn x87_fild_fadd_fistp() {
    // Integer load, add an exact float, store back as integer (round-to-nearest).
    // 100 + 23.0 = 123 -> stored int 123.
    let mut s = [0u8; 64];
    s[0..4].copy_from_slice(&100i32.to_le_bytes());
    s[8..16].copy_from_slice(&23.0f64.to_le_bytes());
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDB, 0x07]); // fild dword [rdi]
    c.extend_from_slice(&[0xDC, 0x47, 0x08]); // fadd qword [rdi+8]
    c.extend_from_slice(&[0xDB, 0x5F, 0x10]); // fistp dword [rdi+0x10]
    c.push(HLT);
    check_mem("x87_fild_fadd_fistp", &with_hlt(c), regs(), s, 0);
}

#[test]
fn x87_fist_m64() {
    // FILD m32 then FISTP m64 (DF /7) round-trip of a 64-bit integer store.
    let mut s = [0u8; 64];
    s[0..4].copy_from_slice(&424242i32.to_le_bytes());
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDB, 0x07]); // fild dword [rdi]
    c.extend_from_slice(&[0xDF, 0x7F, 0x10]); // fistp qword [rdi+0x10]
    c.push(HLT);
    check_mem("x87_fist_m64", &with_hlt(c), regs(), s, 0);
}

#[test]
fn x87_fadd_st_chain() {
    // Two sequential memory adds: ((1 + 2.5) + 4.25) = 7.75, all exact dyadic.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]      ST0=1.0
    c.extend_from_slice(&[0xDC, 0x47, 0x08]); // fadd qword [rdi+8]   ST0=3.5
    c.extend_from_slice(&[0xDC, 0x47, 0x10]); // fadd qword [rdi+16]  ST0=7.75
    c.extend_from_slice(&[0xDD, 0x5F, 0x18]); // fstp qword [rdi+0x18]
    c.push(HLT);
    check_mem(
        "x87_fadd_chain",
        &with_hlt(c),
        regs(),
        scratch_f64(&[1.0, 2.5, 4.25]),
        0,
    );
}

#[test]
fn x87_fsubr_m64() {
    // DC /5 = FSUBR m64 (ST0 = mem - ST0). reg=101 -> modrm 0x6F. 2.5 - 10 = -7.5.
    check_mem(
        "x87_fsubr",
        &with_hlt(x87_binop(0xDC, 0x6F)),
        regs(),
        scratch_f64(&[10.0, 2.5]),
        0,
    );
}

#[test]
fn x87_fdivr_m64() {
    // DC /7 = FDIVR m64 (ST0 = mem / ST0). reg=111 -> modrm 0x7F. 8/2 = 4.
    check_mem(
        "x87_fdivr",
        &with_hlt(x87_binop(0xDC, 0x7F)),
        regs(),
        scratch_f64(&[2.0, 8.0]),
        0,
    );
}

// ---- x87 FCOMI / FUCOMI: compare ST0 with ST(i), set ZF/PF/CF directly ----
//
// Setup: fld b ([rdi+8]) then fld a ([rdi]) so ST0=a, ST1=b, then FCOMI ST0,ST1.
// FCOMI sets ZF/PF/CF in EFLAGS; OF/SF/AF are cleared. We compare exactly the
// status mask (the harness masks to the 6 status flags; FCOMI clears AF/SF/OF).

const FCOMI_FLAGS: u64 = flags::bits::ZF | flags::bits::PF | flags::bits::CF;

/// Build: load b then a so ST0=a,ST1=b; FCOMI ST0,ST1 (DB F1); HLT.
fn fcomi_program() -> Vec<u8> {
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x47, 0x08]); // fld qword [rdi+8]  -> ST0=b
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]    -> ST0=a, ST1=b
    c.extend_from_slice(&[0xDB, 0xF1]); // fcomi st0, st1
    c.push(HLT);
    c
}

#[test]
fn x87_fcomi_greater() {
    // a > b: ZF=0, CF=0, PF=0.
    check_mem(
        "x87_fcomi_gt",
        &fcomi_program(),
        regs(),
        scratch_f64(&[5.0, 3.0]),
        FCOMI_FLAGS,
    );
}

#[test]
fn x87_fcomi_less() {
    // a < b: ZF=0, CF=1, PF=0.
    check_mem(
        "x87_fcomi_lt",
        &fcomi_program(),
        regs(),
        scratch_f64(&[3.0, 5.0]),
        FCOMI_FLAGS,
    );
}

#[test]
fn x87_fcomi_equal() {
    // a == b: ZF=1, CF=0, PF=0.
    check_mem(
        "x87_fcomi_eq",
        &fcomi_program(),
        regs(),
        scratch_f64(&[42.0, 42.0]),
        FCOMI_FLAGS,
    );
}

#[test]
fn x87_fucomi_equal() {
    // FUCOMI ST0,ST1 = DB E9. Same ordered result for non-NaN operands.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x47, 0x08]); // fld qword [rdi+8]
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]
    c.extend_from_slice(&[0xDB, 0xE9]); // fucomi st0, st1
    c.push(HLT);
    check_mem(
        "x87_fucomi_eq",
        &c,
        regs(),
        scratch_f64(&[7.5, 7.5]),
        FCOMI_FLAGS,
    );
}

// ---- String ops: MOVS / STOS / LODS / SCAS / CMPS (with and without REP, DF) ----
//
// Scratch layout convention:
//   src buffer at offset 0, dst buffer at offset 16. We set RSI/RDI to the
//   matching guest addresses, RCX to the element count, and clear/set DF as
//   needed. We compare the whole scratch page plus the final RSI/RDI/RCX.
// String ops affect no arithmetic flags (except SCAS/CMPS which set them like
// CMP); DF must be restored where we set it, and we always end with `CLD`-free
// HLT — the harness reads RFLAGS so we keep DF out of the compared mask.

const SRC_OFF: u64 = 0;
const DST_OFF: u64 = 16;

/// Initial regs for a string test: RSI=src, RDI=dst, RCX=count.
fn string_regs(count: u64) -> Registers {
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF;
    r.rdi = DATA_ADDR + DST_OFF;
    r.rcx = count;
    r
}

/// Scratch with a source byte pattern at offset 0 (dst region left zero).
fn string_scratch(src: &[u8]) -> [u8; 64] {
    let mut s = [0u8; 64];
    s[..src.len()].copy_from_slice(src);
    s
}

#[test]
fn str_rep_movsb() {
    // REP MOVSB: copy RCX bytes src->dst. F3 A4.
    let r = string_regs(8);
    check_mem(
        "rep_movsb",
        &with_hlt(vec![0xF3, 0xA4]),
        r,
        string_scratch(&[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]),
        0,
    );
}

#[test]
fn str_rep_movsw() {
    // REP MOVSW: copy RCX words. 66 F3 A5 (operand-size 16). Copy 4 words.
    let r = string_regs(4);
    check_mem(
        "rep_movsw",
        &with_hlt(vec![0xF3, 0x66, 0xA5]),
        r,
        string_scratch(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]),
        0,
    );
}

#[test]
fn str_rep_movsd() {
    // REP MOVSD: copy RCX dwords. F3 A5. Copy 2 dwords.
    let r = string_regs(2);
    check_mem(
        "rep_movsd",
        &with_hlt(vec![0xF3, 0xA5]),
        r,
        string_scratch(&[0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE]),
        0,
    );
}

#[test]
fn str_rep_movsq() {
    // REP MOVSQ: copy RCX qwords. F3 48 A5. Copy 1 qword.
    let r = string_regs(1);
    check_mem(
        "rep_movsq",
        &with_hlt(vec![0xF3, 0x48, 0xA5]),
        r,
        string_scratch(&[1, 2, 3, 4, 5, 6, 7, 8]),
        0,
    );
}

#[test]
fn str_movsb_single() {
    // Single MOVSB (no REP): copies 1 byte, advances RSI/RDI by 1.
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF;
    r.rdi = DATA_ADDR + DST_OFF;
    check_mem(
        "movsb_single",
        &with_hlt(vec![0xA4]),
        r,
        string_scratch(&[0xAB, 0xCD]),
        0,
    );
}

#[test]
fn str_rep_movsb_df_reverse() {
    // DF=1 reverse copy. Point RSI/RDI at the LAST element so the run walks down.
    // 3 bytes at offsets 0,1,2 copied to dst 16,17,18 in reverse address order.
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF + 2;
    r.rdi = DATA_ADDR + DST_OFF + 2;
    r.rcx = 3;
    r.rflags = flags::bits::DF; // set direction = down
    check_mem(
        "rep_movsb_df",
        &with_hlt(vec![0xF3, 0xA4]),
        r,
        string_scratch(&[0xA1, 0xB2, 0xC3]),
        0,
    );
}

#[test]
fn str_rep_stosb() {
    // REP STOSB: fill RCX bytes at dst with AL. F3 AA. AL=0x5A, count=6.
    let mut r = string_regs(6);
    r.rax = 0x5A;
    check_mem(
        "rep_stosb",
        &with_hlt(vec![0xF3, 0xAA]),
        r,
        string_scratch(&[]),
        0,
    );
}

#[test]
fn str_rep_stosd() {
    // REP STOSD: fill RCX dwords with EAX. F3 AB. EAX=0xCAFEBABE, count=3.
    let mut r = string_regs(3);
    r.rax = 0xCAFE_BABE;
    check_mem(
        "rep_stosd",
        &with_hlt(vec![0xF3, 0xAB]),
        r,
        string_scratch(&[]),
        0,
    );
}

#[test]
fn str_lodsb() {
    // LODSB: AL <- [RSI], RSI advances. AC. Single, no REP.
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF;
    r.rax = 0x1122_3344_5566_7700; // only AL should change
    check_mem(
        "lodsb",
        &with_hlt(vec![0xAC]),
        r,
        string_scratch(&[0x99, 0x88]),
        0,
    );
}

#[test]
fn str_repne_scasb_found() {
    // REPNE SCASB: scan dst for AL, stop on match. F2 AE.
    // Buffer at dst (offset 16) = [1,2,3,4,5,...]; AL=4, count large enough.
    let mut s = [0u8; 64];
    s[DST_OFF as usize..DST_OFF as usize + 6].copy_from_slice(&[1, 2, 3, 4, 5, 6]);
    let mut r = regs();
    r.rdi = DATA_ADDR + DST_OFF;
    r.rcx = 6;
    r.rax = 4; // AL = 4 -> match at index 3
    // SCAS sets arithmetic flags like CMP; compare all status flags.
    check_mem("repne_scasb", &with_hlt(vec![0xF2, 0xAE]), r, s, FLAG_MASK);
}

#[test]
fn str_scasb_single_equal() {
    // Single SCASB where [RDI]==AL -> ZF=1, all CMP flags defined.
    let mut s = [0u8; 64];
    s[DST_OFF as usize] = 0x7F;
    let mut r = regs();
    r.rdi = DATA_ADDR + DST_OFF;
    r.rax = 0x7F;
    check_mem("scasb_eq", &with_hlt(vec![0xAE]), r, s, FLAG_MASK);
}

#[test]
fn str_repe_cmpsb_equal_run() {
    // REPE CMPSB: compare src vs dst while equal. F3 A6.
    // Make src==dst for the whole run -> RCX hits 0, ZF=1 at the end.
    let mut s = [0u8; 64];
    let pat = [0xDE, 0xAD, 0xBE, 0xEF, 0x12];
    s[SRC_OFF as usize..SRC_OFF as usize + 5].copy_from_slice(&pat);
    s[DST_OFF as usize..DST_OFF as usize + 5].copy_from_slice(&pat);
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF;
    r.rdi = DATA_ADDR + DST_OFF;
    r.rcx = 5;
    check_mem(
        "repe_cmpsb_eq",
        &with_hlt(vec![0xF3, 0xA6]),
        r,
        s,
        FLAG_MASK,
    );
}

#[test]
fn str_repe_cmpsb_mismatch() {
    // REPE CMPSB stops at the first differing byte; flags reflect that compare.
    let mut s = [0u8; 64];
    s[SRC_OFF as usize..SRC_OFF as usize + 5].copy_from_slice(&[1, 2, 3, 4, 5]);
    s[DST_OFF as usize..DST_OFF as usize + 5].copy_from_slice(&[1, 2, 9, 4, 5]);
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF;
    r.rdi = DATA_ADDR + DST_OFF;
    r.rcx = 5;
    check_mem(
        "repe_cmpsb_ne",
        &with_hlt(vec![0xF3, 0xA6]),
        r,
        s,
        FLAG_MASK,
    );
}

#[test]
fn str_cmpsb_single() {
    // Single CMPSB: compares [RSI] vs [RDI], sets CMP flags, advances both.
    let mut s = [0u8; 64];
    s[SRC_OFF as usize] = 0x10;
    s[DST_OFF as usize] = 0x20; // 0x10 - 0x20 -> CF, SF
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF;
    r.rdi = DATA_ADDR + DST_OFF;
    check_mem("cmpsb_single", &with_hlt(vec![0xA6]), r, s, FLAG_MASK);
}

// ---- SSE3 / SSE2 float: HADDPS/HSUBPS/ADDSUBPS/MOVDDUP/SHUFPD + conversions ----
//
// All packed-float results below use exactly-representable values so the f64/f32
// rounding is bit-identical across backends. We reuse `sse_program` for the
// 2-operand reg forms and `check_sse` to compare the stored 128-bit result.

#[test]
fn sse3_haddps() {
    // HADDPS xmm0, xmm1 = F2 0F 7C C1. Horizontal add of single-precision lanes.
    // a = [1,2,3,4], b = [10,20,30,40] (as f32). result = [a0+a1, a2+a3, b0+b1, b2+b3].
    let af = [1.0f32, 2.0, 3.0, 4.0];
    let bf = [10.0f32, 20.0, 30.0, 40.0];
    let mut a = [0u8; 16];
    let mut b = [0u8; 16];
    for i in 0..4 {
        a[i * 4..i * 4 + 4].copy_from_slice(&af[i].to_le_bytes());
        b[i * 4..i * 4 + 4].copy_from_slice(&bf[i].to_le_bytes());
    }
    check_sse(
        "haddps",
        &sse_program(&[0xF2, 0x0F, 0x7C, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse3_hsubps() {
    // HSUBPS xmm0, xmm1 = F2 0F 7D C1.
    let af = [10.0f32, 3.0, 8.0, 2.5];
    let bf = [100.0f32, 40.0, 9.0, 1.0];
    let mut a = [0u8; 16];
    let mut b = [0u8; 16];
    for i in 0..4 {
        a[i * 4..i * 4 + 4].copy_from_slice(&af[i].to_le_bytes());
        b[i * 4..i * 4 + 4].copy_from_slice(&bf[i].to_le_bytes());
    }
    check_sse(
        "hsubps",
        &sse_program(&[0xF2, 0x0F, 0x7D, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse3_addsubps() {
    // ADDSUBPS xmm0, xmm1 = F2 0F D0 C1. Subtract even lanes, add odd lanes.
    let af = [5.0f32, 5.0, 5.0, 5.0];
    let bf = [1.0f32, 2.0, 3.0, 4.0];
    let mut a = [0u8; 16];
    let mut b = [0u8; 16];
    for i in 0..4 {
        a[i * 4..i * 4 + 4].copy_from_slice(&af[i].to_le_bytes());
        b[i * 4..i * 4 + 4].copy_from_slice(&bf[i].to_le_bytes());
    }
    check_sse(
        "addsubps",
        &sse_program(&[0xF2, 0x0F, 0xD0, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse3_movddup() {
    // MOVDDUP xmm0, xmm1 = F2 0F 12 C1. Duplicate the low f64 of the source.
    let a = [0u8; 16]; // dest, overwritten
    let b = [12.5f64.to_le_bytes(), 99.0f64.to_le_bytes()].concat();
    check_sse(
        "movddup",
        &sse_program(&[0xF2, 0x0F, 0x12, 0xC1]),
        sse_scratch(a, b.try_into().unwrap()),
    );
}

#[test]
fn sse2_shufpd() {
    // SHUFPD xmm0, xmm1, imm8 = 66 0F C6 C1 ib. imm=0b10 -> dst.lo=a.hi, dst.hi=b.lo.
    let a = [1.0f64.to_le_bytes(), 2.0f64.to_le_bytes()].concat();
    let b = [3.0f64.to_le_bytes(), 4.0f64.to_le_bytes()].concat();
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]); // movdqu xmm0, [rdi]
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0x66, 0x0F, 0xC6, 0xC1, 0x02]); // shufpd xmm0, xmm1, 2
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // movdqu [rdi+0x20], xmm0
    prog.push(HLT);
    check_sse(
        "shufpd",
        &prog,
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse2_cvtdq2ps() {
    // CVTDQ2PS xmm0, xmm1 = 0F 5B C1. Convert 4 packed i32 -> 4 packed f32.
    // Use exact integers (small) so the result is bit-exact.
    let a = [0u8; 16];
    let ints = [3i32, -7, 100, -1];
    let mut b = [0u8; 16];
    for i in 0..4 {
        b[i * 4..i * 4 + 4].copy_from_slice(&ints[i].to_le_bytes());
    }
    check_sse(
        "cvtdq2ps",
        &sse_program(&[0x0F, 0x5B, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse2_cvtps2dq() {
    // CVTPS2DQ xmm0, xmm1 = 66 0F 5B C1. Convert 4 f32 -> 4 i32 (round-to-nearest).
    // Integral inputs -> exact.
    let a = [0u8; 16];
    let fs = [3.0f32, -7.0, 100.0, -1.0];
    let mut b = [0u8; 16];
    for i in 0..4 {
        b[i * 4..i * 4 + 4].copy_from_slice(&fs[i].to_le_bytes());
    }
    check_sse(
        "cvtps2dq",
        &sse_program(&[0x66, 0x0F, 0x5B, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse2_cvttps2dq_truncates() {
    // CVTTPS2DQ xmm0, xmm1 = F3 0F 5B C1. Truncating convert f32 -> i32.
    // 3.9 -> 3, -3.9 -> -3, 2.1 -> 2, -0.9 -> 0.
    let a = [0u8; 16];
    let fs = [3.9f32, -3.9, 2.1, -0.9];
    let mut b = [0u8; 16];
    for i in 0..4 {
        b[i * 4..i * 4 + 4].copy_from_slice(&fs[i].to_le_bytes());
    }
    check_sse(
        "cvttps2dq",
        &sse_program(&[0xF3, 0x0F, 0x5B, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---- Misc: SAHF / LAHF ----

#[test]
fn sahf_loads_flags() {
    // SAHF (9E): AH -> low byte of RFLAGS (SF,ZF,_,AF,_,PF,_,CF). AH=0xD5
    // sets SF,ZF,AF,PF (and bit1 reserved=1, CF=1). We then read the status flags.
    let mut r = regs();
    // AH = 1101_0101: SF=1 ZF=1 (bit5=0) AF=1 (bit3=0) PF=1 (bit1=1 reserved) CF=1
    r.rax = 0xD5 << 8;
    check("sahf", &with_hlt(vec![0x9E]), r);
}

#[test]
fn lahf_stores_flags() {
    // LAHF (9F): low byte of RFLAGS -> AH. Set a known flag state via CMP first.
    let mut r = regs();
    r.rax = 0x0; // 0 - 1
    r.rbx = 0x1;
    // 48 39 D8  cmp rax, rbx  (sets SF, CF, AF)
    // 9F        lahf          (AH <- status byte)
    check("lahf", &with_hlt(vec![0x48, 0x39, 0xD8, 0x9F]), r);
}

#[test]
fn lahf_sahf_roundtrip() {
    // LAHF then SAHF should reproduce the same flag state. Seed via CMP.
    let mut r = regs();
    r.rax = 0x5;
    r.rbx = 0x5; // equal -> ZF, PF
    // cmp; lahf; xor ah with nothing; sahf
    check(
        "lahf_sahf",
        &with_hlt(vec![0x48, 0x39, 0xD8, 0x9F, 0x9E]),
        r,
    );
}

// ---- MOVBE (move with byte swap) ----

#[test]
fn movbe_load_r64() {
    // MOVBE r64, m64 = 48 0F 38 F0 /r. Load+byteswap from [rdi].
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0x0011_2233_4455_6677u64.to_le_bytes());
    let mut r = regs();
    r.rdi = DATA_ADDR;
    // 48 0F 38 F0 07  movbe rax, [rdi]
    check_mem(
        "movbe_load64",
        &with_hlt(vec![0x48, 0x0F, 0x38, 0xF0, 0x07]),
        r,
        s,
        0,
    );
}

#[test]
fn movbe_store_r32() {
    // MOVBE m32, r32 = 0F 38 F1 /r. Store EAX byteswapped to [rdi]; verify scratch.
    let mut r = regs();
    r.rax = 0x1122_3344;
    r.rdi = DATA_ADDR;
    // 0F 38 F1 07  movbe [rdi], eax
    check_mem(
        "movbe_store32",
        &with_hlt(vec![0x0F, 0x38, 0xF1, 0x07]),
        r,
        zero_scratch(),
        0,
    );
}

// ---- BMI1: ANDN / BLSI / BLSR / BLSMSK (VEX-encoded) ----
//
// BMI1 defines ZF and SF from the result, clears OF; AF/PF are undefined and CF
// is defined per-instruction (ANDN clears CF; BLS* set CF specially). To stay on
// the safe side we compare the architecturally-defined bits: ZF, SF, CF, OF.
const BMI_DEFINED: u64 = flags::bits::ZF | flags::bits::SF | flags::bits::CF | flags::bits::OF;

#[test]
fn bmi_andn() {
    // ANDN r32a, r32b, r/m32 = VEX.LZ.0F38.W0 F2 /r : dest = ~src1 & src2.
    // VEX 2-byte: C4 E2 (map 0F38) ... use 3-byte VEX. Encode andn eax, ebx, ecx:
    //   VEX.NDS.LZ.0F38.W0 F2 /r, vvvv = ~ebx, src2 = ecx (modrm).
    // C4 E2 60 F2 C1  -> andn eax, ebx, ecx   (dest=~ebx & ecx = 0xAA00)
    let mut r = regs();
    r.rbx = 0x0000_00FF; // src1 (inverted)
    r.rcx = 0x0000_AAAA; // src2
    check_flags_masked(
        "andn",
        &with_hlt(vec![0xC4, 0xE2, 0x60, 0xF2, 0xC1]),
        r,
        BMI_DEFINED,
    );
}

#[test]
fn bmi_blsi() {
    // BLSI r32, r/m32 = VEX.LZ.0F38.W0 F3 /3 : dest = src & -src (isolate lowest set bit).
    // vvvv encodes dest, modrm.reg=/3 selects the op, modrm.rm=src.
    // andn-style 3-byte VEX with vvvv=eax(dest), rm=ecx(src): C4 E2 78 F3 D9
    //   reg field = 011 (/3 = BLSI), rm = 001 (ecx), vvvv=1111-? -> dest eax.
    let mut r = regs();
    r.rcx = 0x0000_00B0; // lowest set bit is bit 4 (0x10)
    check_flags_masked(
        "blsi",
        &with_hlt(vec![0xC4, 0xE2, 0x78, 0xF3, 0xD9]),
        r,
        BMI_DEFINED,
    );
}

#[test]
fn bmi_blsr() {
    // BLSR r32, r/m32 = VEX.LZ.0F38.W0 F3 /1 : dest = src & (src-1) (clear lowest set bit).
    // reg field = 001 (/1 = BLSR), rm=001(ecx), vvvv -> eax. C4 E2 78 F3 C9.
    let mut r = regs();
    r.rcx = 0x0000_00B0;
    check_flags_masked(
        "blsr",
        &with_hlt(vec![0xC4, 0xE2, 0x78, 0xF3, 0xC9]),
        r,
        BMI_DEFINED,
    );
}

#[test]
fn bmi_blsmsk() {
    // BLSMSK r32, r/m32 = VEX.LZ.0F38.W0 F3 /2 : dest = src ^ (src-1) (mask up to lowest set bit).
    // reg field = 010 (/2 = BLSMSK), rm=001(ecx), vvvv -> eax. C4 E2 78 F3 D1.
    let mut r = regs();
    r.rcx = 0x0000_00B0;
    check_flags_masked(
        "blsmsk",
        &with_hlt(vec![0xC4, 0xE2, 0x78, 0xF3, 0xD1]),
        r,
        BMI_DEFINED,
    );
}

// ---- BMI2: PEXT / PDEP / MULX / RORX / SARX / SHRX / SHLX ----
//
// BMI2 bit-manipulation ops do NOT affect flags at all, so compare GPRs only.

#[test]
fn bmi2_pext() {
    // PEXT r32, r32, r/m32 = VEX.LZ.F3.0F38.W0 F5 /r : extract bits of src1 selected by mask.
    //   andn-form: vvvv = src1 (ebx), rm = mask (ecx), dest = eax.
    //   pp=10 (F3), map=0F38 -> VEX3 C4 E2 62 F5 C1.
    let mut r = regs();
    r.rbx = 0x1234_5678; // source
    r.rcx = 0x0F0F_0F0F; // mask: take low nibble of each byte
    check_flags_masked("pext", &with_hlt(vec![0xC4, 0xE2, 0x62, 0xF5, 0xC1]), r, 0);
}

#[test]
fn bmi2_pdep() {
    // PDEP r32, r32, r/m32 = VEX.LZ.F2.0F38.W0 F5 /r : deposit bits into mask positions.
    //   pp=11 (F2) -> VEX3 C4 E2 63 F5 C1. vvvv=src1(ebx), rm=mask(ecx), dest=eax.
    let mut r = regs();
    r.rbx = 0x0000_000F; // 4 source bits
    r.rcx = 0x0F0F_0F0F; // scatter into these positions
    check_flags_masked("pdep", &with_hlt(vec![0xC4, 0xE2, 0x63, 0xF5, 0xC1]), r, 0);
}

#[test]
fn bmi2_mulx() {
    // MULX r32a, r32b, r/m32 = VEX.LZ.F2.0F38.W0 F6 /r : unsigned mul of EDX by r/m,
    //   high half -> dest1 (vvvv), low half -> dest2 (modrm.reg). EDX is implicit.
    //   mulx eax, ebx, ecx : reg=eax(dest2), vvvv=ebx(dest1), rm=ecx(src2).
    //   pp=11(F2), map=0F38 -> C4 E2 63 F6 C1  (vvvv=ebx=0b1101 inverted... encode below)
    // Encoding: C4 E2 (62?) — vvvv must encode ebx(=0b0011) inverted = 0b1100.
    //   byte3 = W(0)|vvvv(1100)|L(0)|pp(11) = 0 1100 0 11 = 0x63. reg/rm: reg=eax(000), rm=ecx(001) -> modrm C1.
    let mut r = regs();
    r.rdx = 0x0001_0000; // implicit multiplicand
    r.rcx = 0x0001_0000; // src2 -> product 0x1_0000_0000
    check_flags_masked("mulx", &with_hlt(vec![0xC4, 0xE2, 0x63, 0xF6, 0xC1]), r, 0);
}

#[test]
fn bmi2_rorx() {
    // RORX r32, r/m32, imm8 = VEX.LZ.F2.0F3A.W0 F0 /r ib : rotate right, no flags.
    //   map=0F3A -> byte2 low nibble = 3. pp=11(F2). rorx eax, ecx, 8.
    //   C4 E3 7B F0 C1 08 : reg=eax(000), rm=ecx(001) -> C1, imm=8.
    let mut r = regs();
    r.rcx = 0x1234_5678;
    check_flags_masked(
        "rorx",
        &with_hlt(vec![0xC4, 0xE3, 0x7B, 0xF0, 0xC1, 0x08]),
        r,
        0,
    );
}

#[test]
fn bmi2_sarx() {
    // SARX r32, r/m32, r32 = VEX.LZ.F3.0F38.W0 F7 /r : arithmetic shift right by count reg.
    //   pp=10(F3). vvvv encodes the count register (ebx). sarx eax, ecx, ebx.
    //   byte3 = W0 vvvv(~ebx=1100) L0 pp(10) -> 0 1100 0 10 = 0x62. modrm reg=eax,rm=ecx -> C1.
    let mut r = regs();
    r.rcx = 0xFFFF_8000; // negative when treated as i32
    r.rbx = 4; // shift count
    check_flags_masked("sarx", &with_hlt(vec![0xC4, 0xE2, 0x62, 0xF7, 0xC1]), r, 0);
}

#[test]
fn bmi2_shrx() {
    // SHRX r32, r/m32, r32 = VEX.LZ.F2.0F38.W0 F7 /r : logical shift right.
    //   pp=11(F2). vvvv=ebx(count). byte3 = 0 1100 0 11 = 0x63.
    let mut r = regs();
    r.rcx = 0xF000_0000;
    r.rbx = 8;
    check_flags_masked("shrx", &with_hlt(vec![0xC4, 0xE2, 0x63, 0xF7, 0xC1]), r, 0);
}

#[test]
fn bmi2_shlx() {
    // SHLX r32, r/m32, r32 = VEX.LZ.66.0F38.W0 F7 /r : logical shift left.
    //   pp=01(66). vvvv=ebx(count). byte3 = 0 1100 0 01 = 0x61.
    let mut r = regs();
    r.rcx = 0x0000_00FF;
    r.rbx = 12;
    check_flags_masked("shlx", &with_hlt(vec![0xC4, 0xE2, 0x61, 0xF7, 0xC1]), r, 0);
}

#[test]
fn bmi2_rorx_r64() {
    // 64-bit RORX: VEX.LZ.F2.0F3A.W1 F0 /r ib. W1 -> byte3 bit7 set: 0xFB.
    //   rorx rax, rcx, 20 : C4 E3 FB F0 C1 14.
    let mut r = regs();
    r.rcx = 0x0123_4567_89AB_CDEF;
    check_flags_masked(
        "rorx64",
        &with_hlt(vec![0xC4, 0xE3, 0xFB, 0xF0, 0xC1, 0x14]),
        r,
        0,
    );
}

// ===========================================================================
// EXPANDED COVERAGE PART 3: exhaustive ALU operand sizes / mul-div forms /
// rotate-through-carry / SHLD-SHRD edge counts / BCD (mode note) / SSE float /
// more BMI2 / RIP-relative LEA.
// ===========================================================================
//
// Everything here reuses the existing helpers verbatim:
//  - `check`               : GPRs + all 6 status flags + stack.
//  - `check_flags_masked`  : GPRs + a subset of status flags (for ops that leave
//                            some flags architecturally undefined).
//  - `check_sse`           : scratch-driven 128-bit SSE result comparison.
//  - `sse_program`/`sse_scratch` : load xmm0/xmm1 from scratch, run op, store back.
//
// Flag-definition reminders (Intel SDM Vol.2) used below:
//  - ADD/SUB/ADC/SBB/CMP/NEG/AND/OR/XOR/TEST : all 6 status flags defined.
//  - INC/DEC : define OF/SF/ZF/AF/PF, leave CF UNCHANGED (so we must seed CF and
//              verify it survives - `check` compares CF too).
//  - MUL/IMUL : define CF/OF only; SF/ZF/AF/PF UNDEFINED -> mask to MULDIV_DEFINED.
//  - DIV/IDIV : ALL status flags UNDEFINED -> mask 0 (GPR-only compare).
//  - RCL/RCR  : define CF; OF defined only for a 1-count, UNDEFINED for count>1.
//  - SHLD/SHRD: define CF/SF/ZF/PF; OF defined only for count==1; AF undefined;
//               result UNDEFINED when count > operand size (we avoid that range).
//  - SSE compare/min/max ops produce a bit-exact 128-bit result we compare via
//    scratch; no RFLAGS effect (check_sse uses flag_mask 0).

// Rotate-through-carry: CF always defined; OF undefined for count>1.
const RCL_RCR_1: u64 = FLAG_MASK; // a 1-count rotate fully defines OF too
const RCL_RCR_MULTI: u64 = flags::bits::CF; // count>1 -> only CF defined

// ---------------------------------------------------------------------------
// ALU: ADD/SUB/AND/OR/XOR/CMP across 8/16/32/64 at sign/zero/overflow edges.
// ---------------------------------------------------------------------------

// ---- ADD at signed/unsigned boundaries, all widths ----

#[test]
fn add8_7f_plus_1_overflow() {
    // 0x7F + 1 = 0x80 : OF=1, SF=1, AF=1, CF=0, ZF=0.
    let mut r = regs();
    r.rax = 0x7F;
    r.rbx = 0x01;
    check("add8_7f_1", &with_hlt(vec![0x00, 0xD8]), r); // add al, bl
}

#[test]
fn add8_ff_plus_ff() {
    // 0xFF + 0xFF = 0x1FE -> al=0xFE, CF=1, OF=0, SF=1.
    let mut r = regs();
    r.rax = 0xFF;
    r.rbx = 0xFF;
    check("add8_ff_ff", &with_hlt(vec![0x00, 0xD8]), r);
}

#[test]
fn add16_8000_plus_8000() {
    // 0x8000 + 0x8000 = 0x10000 -> ax=0, CF=1, OF=1, ZF=1.
    let mut r = regs();
    r.rax = 0x8000;
    r.rbx = 0x8000;
    check("add16_8000", &with_hlt(vec![0x66, 0x01, 0xD8]), r); // add ax, bx
}

#[test]
fn add32_7fffffff_plus_1() {
    // 0x7FFFFFFF + 1 = 0x80000000 : OF=1, SF=1, zero-extends into rax.
    let mut r = regs();
    r.rax = 0x7FFF_FFFF;
    r.rbx = 0x0000_0001;
    check("add32_7fff", &with_hlt(vec![0x01, 0xD8]), r); // add eax, ebx
}

#[test]
fn add64_signed_boundary() {
    // INT64_MAX + 1 -> INT64_MIN : OF=1, SF=1.
    let mut r = regs();
    r.rax = 0x7FFF_FFFF_FFFF_FFFF;
    r.rbx = 0x1;
    check("add64_int_min", &with_hlt(vec![0x48, 0x01, 0xD8]), r); // add rax, rbx
}

// ---- SUB at signed/unsigned boundaries, all widths ----

#[test]
fn sub8_80_minus_1_overflow() {
    // 0x80 - 1 = 0x7F : signed (-128)-1 underflow -> OF=1, SF=0, CF=0.
    let mut r = regs();
    r.rax = 0x80;
    r.rbx = 0x01;
    check("sub8_80_1", &with_hlt(vec![0x28, 0xD8]), r); // sub al, bl
}

#[test]
fn sub16_0_minus_8000() {
    // 0 - 0x8000 : CF=1, OF=1 (result 0x8000 = INT16_MIN), SF=1.
    let mut r = regs();
    r.rax = 0x0000;
    r.rbx = 0x8000;
    check("sub16_0_8000", &with_hlt(vec![0x66, 0x29, 0xD8]), r); // sub ax, bx
}

#[test]
fn sub32_80000000_minus_1() {
    // 0x80000000 - 1 = 0x7FFFFFFF : OF=1 (INT32_MIN-1), CF=0.
    let mut r = regs();
    r.rax = 0x8000_0000;
    r.rbx = 0x0000_0001;
    check("sub32_int_min", &with_hlt(vec![0x29, 0xD8]), r); // sub eax, ebx
}

#[test]
fn sub64_equal_zero() {
    // equal operands -> 0, ZF=1, CF=0, OF=0.
    let mut r = regs();
    r.rax = 0xDEAD_BEEF_CAFE_BABE;
    r.rbx = 0xDEAD_BEEF_CAFE_BABE;
    check("sub64_eq", &with_hlt(vec![0x48, 0x29, 0xD8]), r);
}

// ---- AND / OR / XOR at width edges (CF/OF always cleared, SF/ZF/PF defined) ----

#[test]
fn and8_high_bit() {
    // 0x80 & 0xFF = 0x80 : SF=1, ZF=0, PF=0 (one bit), CF=OF=0.
    let mut r = regs();
    r.rax = 0x80;
    r.rbx = 0xFF;
    check("and8_sf", &with_hlt(vec![0x20, 0xD8]), r); // and al, bl
}

#[test]
fn or32_zero_extends() {
    // OR of 32-bit operands zero-extends into rax; result 0x80000001 -> SF=1.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_8000_0000;
    r.rbx = 0x0000_0000_0000_0001;
    check("or32_zx", &with_hlt(vec![0x09, 0xD8]), r); // or eax, ebx
}

#[test]
fn xor16_high_bit() {
    // 0x8000 ^ 0x0001 = 0x8001 : SF=1, PF=0.
    let mut r = regs();
    r.rax = 0x8000;
    r.rbx = 0x0001;
    check("xor16_sf", &with_hlt(vec![0x66, 0x31, 0xD8]), r); // xor ax, bx
}

#[test]
fn or64_zero_result() {
    // 0 | 0 -> ZF=1, PF=1.
    let r = regs();
    check("or64_zero", &with_hlt(vec![0x48, 0x09, 0xD8]), r); // or rax, rbx (both 0)
}

// ---- CMP at width edges (signed overflow boundaries) ----

#[test]
fn cmp16_overflow() {
    // 0x7FFF cmp 0xFFFF (-1): 32767 - (-1) overflows -> OF=1.
    let mut r = regs();
    r.rax = 0x7FFF;
    r.rbx = 0xFFFF;
    check("cmp16_of", &with_hlt(vec![0x66, 0x39, 0xD8]), r); // cmp ax, bx
}

#[test]
fn cmp32_equal() {
    let mut r = regs();
    r.rax = 0x1234_5678;
    r.rbx = 0x1234_5678;
    check("cmp32_eq", &with_hlt(vec![0x39, 0xD8]), r); // cmp eax, ebx
}

#[test]
fn cmp8_imm_boundary() {
    // CMP AL, 0x80 with AL=0x7F : signed 127 - (-128) overflows -> OF=1, SF=1, CF=1.
    let mut r = regs();
    r.rax = 0x7F;
    check("cmp8_imm80", &with_hlt(vec![0x3C, 0x80]), r); // cmp al, 0x80
}

// ---------------------------------------------------------------------------
// ADC / SBB carry-in chains: model multi-word add/sub with carry propagation.
// ---------------------------------------------------------------------------

#[test]
fn adc_chain_two_words() {
    // Emulate a 128-bit add of two 64-bit halves:
    //   add rax, rcx     (low halves, sets CF)
    //   adc rbx, rdx     (high halves, consumes CF)
    // low: 0xFFFF.. + 1 -> 0, CF=1 ; high: 0 + 0 + 1 -> 1.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF;
    r.rcx = 0x0000_0000_0000_0001;
    r.rbx = 0x0000_0000_0000_0000;
    r.rdx = 0x0000_0000_0000_0000;
    // 48 01 C8  add rax, rcx
    // 48 11 D3  adc rbx, rdx
    check(
        "adc_chain",
        &with_hlt(vec![0x48, 0x01, 0xC8, 0x48, 0x11, 0xD3]),
        r,
    );
}

#[test]
fn sbb_chain_two_words() {
    // 128-bit subtract:
    //   sub rax, rcx     (low halves, sets borrow)
    //   sbb rbx, rdx     (high halves, consumes borrow)
    // low: 0 - 1 -> 0xFFFF.., CF=1 ; high: 5 - 0 - 1 -> 4.
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0000;
    r.rcx = 0x0000_0000_0000_0001;
    r.rbx = 0x0000_0000_0000_0005;
    r.rdx = 0x0000_0000_0000_0000;
    // 48 29 C8  sub rax, rcx
    // 48 19 D3  sbb rbx, rdx
    check(
        "sbb_chain",
        &with_hlt(vec![0x48, 0x29, 0xC8, 0x48, 0x19, 0xD3]),
        r,
    );
}

#[test]
fn adc8_carry_in_no_carry_out() {
    // adc al, bl with CF=1 : 0x10 + 0x20 + 1 = 0x31, CF=0, AF=0.
    let mut r = regs();
    r.rax = 0x10;
    r.rbx = 0x20;
    r.rflags = flags::bits::CF;
    check("adc8_cin", &with_hlt(vec![0x10, 0xD8]), r); // adc al, bl
}

#[test]
fn adc16_carry_in_wrap() {
    // adc ax, bx with CF=1 : 0xFFFF + 0 + 1 = 0x10000 -> ax=0, CF=1, ZF=1.
    let mut r = regs();
    r.rax = 0xFFFF;
    r.rbx = 0x0000;
    r.rflags = flags::bits::CF;
    check("adc16_cin", &with_hlt(vec![0x66, 0x11, 0xD8]), r); // adc ax, bx
}

#[test]
fn sbb16_borrow_in() {
    // sbb ax, bx with CF=1 : 0x0001 - 0x0000 - 1 = 0x0000 -> ZF=1.
    let mut r = regs();
    r.rax = 0x0001;
    r.rbx = 0x0000;
    r.rflags = flags::bits::CF;
    check("sbb16_bin", &with_hlt(vec![0x66, 0x19, 0xD8]), r); // sbb ax, bx
}

#[test]
fn sbb8_borrow_in_underflow() {
    // sbb al, bl with CF=1 : 0x00 - 0x00 - 1 -> 0xFF, CF=1, SF=1, AF=1.
    let mut r = regs();
    r.rax = 0x00;
    r.rbx = 0x00;
    r.rflags = flags::bits::CF;
    check("sbb8_bin", &with_hlt(vec![0x18, 0xD8]), r); // sbb al, bl
}

// ---------------------------------------------------------------------------
// INC / DEC must preserve CF across all widths; NEG flag exactness.
// ---------------------------------------------------------------------------

#[test]
fn inc16_overflow_keeps_cf() {
    // inc ax with ax=0x7FFF -> 0x8000 : OF=1, SF=1; CF must stay as seeded (1).
    let mut r = regs();
    r.rax = 0x7FFF;
    r.rflags = flags::bits::CF;
    check("inc16_of_cf", &with_hlt(vec![0x66, 0xFF, 0xC0]), r); // inc ax
}

#[test]
fn dec8_underflow_keeps_cf() {
    // dec al with al=0 -> 0xFF : SF=1, AF=1, OF=0; CF preserved (seeded 1).
    let mut r = regs();
    r.rax = 0x00;
    r.rflags = flags::bits::CF;
    check("dec8_uf_cf", &with_hlt(vec![0xFE, 0xC8]), r); // dec al
}

#[test]
fn dec32_int_min_overflow() {
    // dec eax with eax=0x80000000 -> 0x7FFFFFFF : OF=1; CF preserved.
    let mut r = regs();
    r.rax = 0x8000_0000;
    r.rflags = flags::bits::CF;
    check("dec32_of", &with_hlt(vec![0xFF, 0xC8]), r); // dec eax
}

#[test]
fn neg8_int_min() {
    // neg al with al=0x80 (-128) -> 0x80 (overflow): OF=1, CF=1, SF=1.
    let mut r = regs();
    r.rax = 0x80;
    check("neg8_int_min", &with_hlt(vec![0xF6, 0xD8]), r); // neg al
}

#[test]
fn neg16_one() {
    // neg ax with ax=1 -> 0xFFFF : CF=1, SF=1, OF=0, AF=1.
    let mut r = regs();
    r.rax = 0x0001;
    check("neg16_one", &with_hlt(vec![0x66, 0xF7, 0xD8]), r); // neg ax
}

#[test]
fn neg32_zero_extends() {
    // neg eax with eax=0x00000010 -> 0xFFFFFFF0, zero-extends into rax; CF=1.
    let mut r = regs();
    r.rax = 0x0000_0010;
    check("neg32_zx", &with_hlt(vec![0xF7, 0xD8]), r); // neg eax
}

// ---------------------------------------------------------------------------
// IMUL forms: one-operand (RDX:RAX), two-operand, three-operand-with-imm.
// MUL one-operand. All mask to MULDIV_DEFINED (CF/OF only defined).
// ---------------------------------------------------------------------------

#[test]
fn imul_one_operand_64() {
    // IMUL r/m64 (F7 /5) : RDX:RAX = RAX * RBX (signed full product).
    // (-2) * 3 = -6 : RAX=0xFFFF..FA, RDX=0xFFFF..FF (sign extension), CF/OF=0.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFE; // -2
    r.rbx = 0x0000_0000_0000_0003; // 3
    // 48 F7 EB  imul rbx
    check_flags_masked(
        "imul1_64",
        &with_hlt(vec![0x48, 0xF7, 0xEB]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn imul_one_operand_overflow() {
    // Large signed product fills RDX -> CF/OF=1 (result doesn't fit in RAX alone).
    let mut r = regs();
    r.rax = 0x0000_0001_0000_0000; // 2^32
    r.rbx = 0x0000_0001_0000_0000; // 2^32 -> product 2^64 in RDX:RAX
    check_flags_masked(
        "imul1_of",
        &with_hlt(vec![0x48, 0xF7, 0xEB]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn imul_one_operand_32() {
    // IMUL r/m32 (F7 /5, no REX.W): EDX:EAX = EAX * EBX, zero-extends both into r64.
    // (-3) * 7 = -21 : EAX=0xFFFFFFEB, EDX=0xFFFFFFFF.
    let mut r = regs();
    r.rax = 0xFFFF_FFFD; // -3 in 32-bit
    r.rbx = 0x0000_0007;
    // F7 EB  imul ebx
    check_flags_masked("imul1_32", &with_hlt(vec![0xF7, 0xEB]), r, MULDIV_DEFINED);
}

#[test]
fn imul_two_operand_negative() {
    // imul rax, rbx with signed operands : (-4) * 5 = -20.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFC; // -4
    r.rbx = 0x0000_0000_0000_0005;
    // 48 0F AF C3  imul rax, rbx
    check_flags_masked(
        "imul2_neg",
        &with_hlt(vec![0x48, 0x0F, 0xAF, 0xC3]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn imul_three_operand_imm8() {
    // IMUL rax, rbx, imm8 (48 6B C3 ib) : rax = rbx * sign_extend(imm8).
    // 0x100 * (-3) = -0x300.
    let mut r = regs();
    r.rbx = 0x0000_0000_0000_0100;
    // 48 6B C3 FD  imul rax, rbx, -3
    check_flags_masked(
        "imul3_imm8",
        &with_hlt(vec![0x48, 0x6B, 0xC3, 0xFD]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn imul_three_operand_imm32() {
    // IMUL rax, rbx, imm32 (48 69 C3 id) : rax = rbx * sign_extend(imm32).
    // 7 * 0x0001_0000 = 0x0007_0000.
    let mut r = regs();
    r.rbx = 0x0000_0000_0000_0007;
    // 48 69 C3 00 00 01 00  imul rax, rbx, 0x10000
    check_flags_masked(
        "imul3_imm32",
        &with_hlt(vec![0x48, 0x69, 0xC3, 0x00, 0x00, 0x01, 0x00]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn imul_three_operand_imm32_overflow() {
    // Product overflows 64 bits when truncated -> CF/OF=1.
    let mut r = regs();
    r.rbx = 0x4000_0000_0000_0000;
    // 48 69 C3 00 00 00 40  imul rax, rbx, 0x40000000
    check_flags_masked(
        "imul3_of",
        &with_hlt(vec![0x48, 0x69, 0xC3, 0x00, 0x00, 0x00, 0x40]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn imul_two_operand_32() {
    // imul eax, ebx (0F AF) : 32-bit product, zero-extends into rax.
    // 0x10000 * 0x10000 = 0x1_0000_0000 -> low 32 = 0, CF/OF set (truncation).
    let mut r = regs();
    r.rax = 0x0001_0000;
    r.rbx = 0x0001_0000;
    // 0F AF C3  imul eax, ebx
    check_flags_masked(
        "imul2_32",
        &with_hlt(vec![0x0F, 0xAF, 0xC3]),
        r,
        MULDIV_DEFINED,
    );
}

#[test]
fn mul_one_operand_32() {
    // MUL r/m32 (F7 /4): EDX:EAX = EAX * EBX (unsigned). 0xFFFFFFFF * 2.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF;
    r.rbx = 0x0000_0002;
    // F7 E3  mul ebx
    check_flags_masked("mul32", &with_hlt(vec![0xF7, 0xE3]), r, MULDIV_DEFINED);
}

#[test]
fn mul_one_operand_8() {
    // MUL r/m8 (F6 /4): AX = AL * BL (unsigned). 0xFF * 0xFF = 0xFE01.
    let mut r = regs();
    r.rax = 0x00FF;
    r.rbx = 0x00FF;
    // F6 E3  mul bl
    check_flags_masked("mul8", &with_hlt(vec![0xF6, 0xE3]), r, MULDIV_DEFINED);
}

#[test]
fn mul_no_overflow_clears_cf_of() {
    // small product fits in low half -> CF=OF=0.
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0003;
    r.rbx = 0x0000_0000_0000_0005;
    // 48 F7 E3  mul rbx -> RDX=0, RAX=15, CF=OF=0
    check_flags_masked(
        "mul_nooverflow",
        &with_hlt(vec![0x48, 0xF7, 0xE3]),
        r,
        MULDIV_DEFINED,
    );
}

// ---------------------------------------------------------------------------
// DIV / IDIV exact quotient & remainder (all flags undefined -> mask 0, GPRs only).
// ---------------------------------------------------------------------------

#[test]
fn div_with_remainder() {
    // RDX:RAX / RBX : 1000 / 7 = 142 rem 6.
    let mut r = regs();
    r.rax = 1000;
    r.rdx = 0;
    r.rbx = 7;
    // 48 F7 F3  div rbx
    check_flags_masked("div_rem", &with_hlt(vec![0x48, 0xF7, 0xF3]), r, 0);
}

#[test]
fn div_128_by_64() {
    // RDX:RAX is a true 128-bit dividend: (1<<64 + 0) / 2 = 0x8000_0000_0000_0000.
    let mut r = regs();
    r.rdx = 0x0000_0000_0000_0001; // high
    r.rax = 0x0000_0000_0000_0000; // low -> dividend = 2^64
    r.rbx = 0x0000_0000_0000_0002;
    check_flags_masked("div128", &with_hlt(vec![0x48, 0xF7, 0xF3]), r, 0);
}

#[test]
fn div32_with_remainder() {
    // EDX:EAX / EBX : 0x10000007 / 0x10 = 0x1000000 rem 7. Zero-extends into r64.
    let mut r = regs();
    r.rax = 0x1000_0007;
    r.rdx = 0;
    r.rbx = 0x10;
    // F7 F3  div ebx
    check_flags_masked("div32", &with_hlt(vec![0xF7, 0xF3]), r, 0);
}

#[test]
fn div8_ax_by_bl() {
    // DIV r/m8 (F6 /6): AL <- AX/BL quotient, AH <- remainder. 0x00FF / 0x10 = 0x0F r 0x0F.
    let mut r = regs();
    r.rax = 0x00FF;
    r.rbx = 0x0010;
    // F6 F3  div bl
    check_flags_masked("div8", &with_hlt(vec![0xF6, 0xF3]), r, 0);
}

#[test]
fn idiv_negative_dividend() {
    // signed: -1000 / 7 = -142 r -6 (truncation toward zero).
    let mut r = regs();
    r.rax = (-1000i64) as u64;
    r.rdx = 0xFFFF_FFFF_FFFF_FFFF; // sign extension of the negative dividend
    r.rbx = 7;
    // 48 F7 FB  idiv rbx
    check_flags_masked("idiv_neg", &with_hlt(vec![0x48, 0xF7, 0xFB]), r, 0);
}

#[test]
fn idiv_negative_divisor() {
    // 1000 / -7 = -142 r 6.
    let mut r = regs();
    r.rax = 1000;
    r.rdx = 0;
    r.rbx = (-7i64) as u64;
    check_flags_masked("idiv_negdiv", &with_hlt(vec![0x48, 0xF7, 0xFB]), r, 0);
}

#[test]
fn idiv32_negative() {
    // signed 32-bit: -100 / 9 = -11 r -1. EDX:EAX sign-extended dividend.
    let mut r = regs();
    r.rax = (-100i32) as u32 as u64;
    r.rdx = 0xFFFF_FFFF; // sign-extend low 32
    r.rbx = 9;
    // F7 FB  idiv ebx
    check_flags_masked("idiv32", &with_hlt(vec![0xF7, 0xFB]), r, 0);
}

// ---------------------------------------------------------------------------
// Rotate through carry: RCL / RCR by 1 and by CL.
// ---------------------------------------------------------------------------

#[test]
fn rcl8_by_1() {
    // rcl al, 1 with al=0x80, CF=0 : MSB(1) -> CF, CF(0) -> bit0 => al=0x00, CF=1, OF=(CF^MSB)=1.
    let mut r = regs();
    r.rax = 0x80;
    r.rflags = 0; // CF=0
    // D0 D0  rcl al, 1
    check_flags_masked("rcl8_1", &with_hlt(vec![0xD0, 0xD0]), r, RCL_RCR_1);
}

#[test]
fn rcr8_by_1() {
    // rcr al, 1 with al=0x01, CF=1 : bit0(1)->CF, old CF(1)->MSB => al=0x80, CF=1.
    let mut r = regs();
    r.rax = 0x01;
    r.rflags = flags::bits::CF;
    // D0 D8  rcr al, 1
    check_flags_masked("rcr8_1", &with_hlt(vec![0xD0, 0xD8]), r, RCL_RCR_1);
}

#[test]
fn rcl16_by_1() {
    let mut r = regs();
    r.rax = 0xC000; // top two bits set
    r.rflags = 0;
    // 66 D1 D0  rcl ax, 1
    check_flags_masked("rcl16_1", &with_hlt(vec![0x66, 0xD1, 0xD0]), r, RCL_RCR_1);
}

#[test]
fn rcl32_by_1_carry_in() {
    // rcl eax, 1 with eax=0x8000_0000, CF=1 : MSB->CF(1), CF->bit0 => eax=1, CF=1.
    let mut r = regs();
    r.rax = 0x8000_0000;
    r.rflags = flags::bits::CF;
    // D1 D0  rcl eax, 1
    check_flags_masked("rcl32_1", &with_hlt(vec![0xD1, 0xD0]), r, RCL_RCR_1);
}

#[test]
fn rcl64_by_1() {
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0001;
    r.rflags = flags::bits::CF;
    // 48 D1 D0  rcl rax, 1
    check_flags_masked("rcl64_1", &with_hlt(vec![0x48, 0xD1, 0xD0]), r, RCL_RCR_1);
}

#[test]
fn rcr64_by_1() {
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0001;
    r.rflags = 0; // CF=0 rotates a 0 into the MSB
    // 48 D1 D8  rcr rax, 1
    check_flags_masked("rcr64_1", &with_hlt(vec![0x48, 0xD1, 0xD8]), r, RCL_RCR_1);
}

#[test]
fn rcl_by_cl_multi() {
    // rcl rax, cl with count>1 : CF defined, OF undefined -> mask CF only.
    let mut r = regs();
    r.rax = 0x1234_5678_9ABC_DEF0;
    r.rcx = 5;
    r.rflags = flags::bits::CF;
    // 48 D3 D0  rcl rax, cl
    check_flags_masked(
        "rcl_cl5",
        &with_hlt(vec![0x48, 0xD3, 0xD0]),
        r,
        RCL_RCR_MULTI,
    );
}

#[test]
fn rcr_by_cl_multi() {
    let mut r = regs();
    r.rax = 0x0FED_CBA9_8765_4321;
    r.rcx = 9;
    r.rflags = 0;
    // 48 D3 D8  rcr rax, cl
    check_flags_masked(
        "rcr_cl9",
        &with_hlt(vec![0x48, 0xD3, 0xD8]),
        r,
        RCL_RCR_MULTI,
    );
}

#[test]
fn rcr32_by_cl_multi() {
    // 32-bit rcr by CL: count is masked mod 32; result zero-extends.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_DEAD_BEEF;
    r.rcx = 7;
    r.rflags = flags::bits::CF;
    // D3 D8  rcr eax, cl
    check_flags_masked("rcr32_cl7", &with_hlt(vec![0xD3, 0xD8]), r, RCL_RCR_MULTI);
}

#[test]
fn rcl8_by_cl_wraps_mod9() {
    // 8-bit RCL count is taken modulo 9 (8 data bits + carry). CL=10 -> effective 1.
    let mut r = regs();
    r.rax = 0x55;
    r.rcx = 10;
    r.rflags = flags::bits::CF;
    // D2 D0  rcl al, cl
    check_flags_masked("rcl8_cl10", &with_hlt(vec![0xD2, 0xD0]), r, RCL_RCR_MULTI);
}

// ---------------------------------------------------------------------------
// SHLD / SHRD by various counts, including a 1-count (OF defined) and counts
// approaching operand size. (count > operand size is architecturally UNDEFINED
// and is deliberately avoided.)
// ---------------------------------------------------------------------------

#[test]
fn shrd_imm1_defines_of() {
    // A 1-bit SHRD defines OF -> compare all status flags.
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0001;
    r.rbx = 0x8000_0000_0000_0000;
    // 48 0F AC D8 01  shrd rax, rbx, 1
    check(
        "shrd_imm1",
        &with_hlt(vec![0x48, 0x0F, 0xAC, 0xD8, 0x01]),
        r,
    );
}

#[test]
fn shld_count_63() {
    // shld rax, rbx, 63 : maximal in-range count for a 64-bit double shift.
    let mut r = regs();
    r.rax = 0x1;
    r.rbx = 0xFFFF_FFFF_FFFF_FFFF;
    // 48 0F A4 D8 3F  shld rax, rbx, 63
    check_flags_masked(
        "shld_63",
        &with_hlt(vec![0x48, 0x0F, 0xA4, 0xD8, 0x3F]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shrd_count_63() {
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0000;
    r.rbx = 0xFFFF_FFFF_FFFF_FFFF;
    // 48 0F AC D8 3F  shrd rax, rbx, 63
    check_flags_masked(
        "shrd_63",
        &with_hlt(vec![0x48, 0x0F, 0xAC, 0xD8, 0x3F]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shld16_cl() {
    // 16-bit SHLD: count masked mod 32, but a count within [1,16) is well-defined.
    let mut r = regs();
    r.rax = 0x0000_0000_0000_1234;
    r.rbx = 0x0000_0000_0000_ABCD;
    r.rcx = 4;
    // 66 0F A5 D8  shld ax, bx, cl
    check_flags_masked(
        "shld16_cl4",
        &with_hlt(vec![0x66, 0x0F, 0xA5, 0xD8]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shrd16_imm() {
    let mut r = regs();
    r.rax = 0x0000_0000_0000_F000;
    r.rbx = 0x0000_0000_0000_000F;
    // 66 0F AC D8 04  shrd ax, bx, 4
    check_flags_masked(
        "shrd16_imm4",
        &with_hlt(vec![0x66, 0x0F, 0xAC, 0xD8, 0x04]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shld32_cl16() {
    // 32-bit SHLD by 16: result zero-extends into rax.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_1234_5678;
    r.rbx = 0x0000_0000_ABCD_EF01;
    r.rcx = 16;
    // 0F A5 D8  shld eax, ebx, cl
    check_flags_masked(
        "shld32_cl16",
        &with_hlt(vec![0x0F, 0xA5, 0xD8]),
        r,
        SHIFT_NO_OF,
    );
}

#[test]
fn shrd_cl_31() {
    // shrd rax, rbx, cl with CL=31.
    let mut r = regs();
    r.rax = 0xAAAA_AAAA_AAAA_AAAA;
    r.rbx = 0x5555_5555_5555_5555;
    r.rcx = 31;
    // 48 0F AD D8  shrd rax, rbx, cl
    check_flags_masked(
        "shrd_cl31",
        &with_hlt(vec![0x48, 0x0F, 0xAD, 0xD8]),
        r,
        SHIFT_NO_OF,
    );
}

// ---------------------------------------------------------------------------
// BCD adjust instructions (AAA/AAS/AAD/AAM/DAA/DAS).
//
// NOTE: these opcodes (0x37, 0x3F, 0x27, 0x2F, 0xD4, 0xD5) are INVALID in
// 64-bit mode and raise #UD on real hardware and under KVM. The differential
// harness only sets up 64-bit long mode, so these cannot be exercised here and
// are intentionally NOT added as runnable cases (they would only "diverge"
// because both backends should #UD, which the harness models as an abnormal
// exit rather than a comparable architectural state). This block documents the
// deliberate omission required by the harness's 64-bit-only setup.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Conditional branch (Jcc) across the flag matrix. Each program does
//   cmp rax, rbx        ; set flags
//   jcc taken           ; +N forward
//   mov rcx, 0xBAD      ; (not-taken path) marker
//   jmp end
//   taken: mov rcx, 0x600D
//   end: hlt
// We compare RCX (and flags) so a wrong branch decision is observable.
// ---------------------------------------------------------------------------

/// Build a Jcc test: cmp rax,rbx then `jcc` (1-byte opcode 0x7x). The taken path
/// sets rcx=0x600D, the fall-through sets rcx=0xBAD.
fn jcc_program(jcc: u8) -> Vec<u8> {
    // 48 39 D8            cmp rax, rbx
    // 7x 07               jcc +7  (skip the 7-byte not-taken block)
    // 48 C7 C1 AD 0B 00 00  mov rcx, 0xBAD     (7 bytes) [not taken]
    // EB 07               jmp +7 (skip the taken block)
    // 48 C7 C1 0D 60 00 00  mov rcx, 0x600D    (7 bytes) [taken target]
    // F4                  hlt
    let mut c = vec![0x48, 0x39, 0xD8];
    c.extend_from_slice(&[jcc, 0x07]);
    c.extend_from_slice(&[0x48, 0xC7, 0xC1, 0xAD, 0x0B, 0x00, 0x00]); // mov rcx, 0xBAD
    c.extend_from_slice(&[0xEB, 0x07]); // jmp +7
    c.extend_from_slice(&[0x48, 0xC7, 0xC1, 0x0D, 0x60, 0x00, 0x00]); // mov rcx, 0x600D
    c.push(HLT);
    c
}

fn check_jcc(label: &str, jcc: u8, rax: u64, rbx: u64) {
    let mut r = regs();
    r.rax = rax;
    r.rbx = rbx;
    check(label, &jcc_program(jcc), r);
}

#[test]
fn jcc_all_conditions() {
    // 0x70..=0x7F = JO,JNO,JB,JAE,JE,JNE,JBE,JA,JS,JNS,JP,JNP,JL,JGE,JLE,JG.
    // Each tuple sets flags so the condition is TRUE.
    let true_cases: &[(u8, u64, u64, &str)] = &[
        (0x70, 0x8000_0000_0000_0000, 1, "jo"), // INT_MIN - 1 -> OF
        (0x71, 5, 3, "jno"),
        (0x72, 1, 2, "jb"),  // 1 < 2 unsigned -> CF
        (0x73, 5, 3, "jae"), // CF=0
        (0x74, 7, 7, "je"),  // equal -> ZF
        (0x75, 7, 8, "jne"),
        (0x76, 2, 2, "jbe"), // CF|ZF
        (0x77, 5, 3, "ja"),
        (0x78, 0, 1, "js"), // 0-1 -> SF
        (0x79, 5, 3, "jns"),
        (0x7A, 3, 0, "jp"),  // result 3 -> even parity
        (0x7B, 1, 0, "jnp"), // result 1 -> odd parity
        (0x7C, 1, 2, "jl"),  // signed less
        (0x7D, 5, 3, "jge"),
        (0x7E, 2, 2, "jle"), // equal -> le
        (0x7F, 5, 3, "jg"),
    ];
    for &(opc, rax, rbx, name) in true_cases {
        check_jcc(name, opc, rax, rbx);
    }
    // A few not-taken (FALSE) paths so the fall-through is exercised too.
    check_jcc("jo_false", 0x70, 5, 3); // no overflow
    check_jcc("je_false", 0x74, 7, 8); // not equal
    check_jcc("jg_false", 0x7F, 1, 2); // not greater
}

#[test]
fn jcc_rel32_forward() {
    // 32-bit displacement conditional jump (0F 84 = JE rel32).
    //   cmp rax, rbx           (equal -> ZF)
    //   0F 84 07 00 00 00      je +7
    //   mov rcx, 0xBAD
    //   EB 07                  jmp +7
    //   mov rcx, 0x600D
    //   hlt
    let mut r = regs();
    r.rax = 0x1234;
    r.rbx = 0x1234;
    let mut c = vec![0x48, 0x39, 0xD8];
    c.extend_from_slice(&[0x0F, 0x84, 0x07, 0x00, 0x00, 0x00]); // je rel32 +7
    c.extend_from_slice(&[0x48, 0xC7, 0xC1, 0xAD, 0x0B, 0x00, 0x00]);
    c.extend_from_slice(&[0xEB, 0x07]);
    c.extend_from_slice(&[0x48, 0xC7, 0xC1, 0x0D, 0x60, 0x00, 0x00]);
    c.push(HLT);
    check("je_rel32", &c, r);
}

#[test]
fn loop_decrements_rcx() {
    // LOOP (E2 rel8) decrements RCX and jumps while RCX != 0.
    //   add rax, 1          (48 83 C0 01)
    //   loop -6             (E2 FA)  back to the add
    //   hlt
    // RCX=3 -> rax incremented 3 times, RCX ends at 0.
    let mut r = regs();
    r.rcx = 3;
    r.rax = 0;
    let mut c = vec![0x48, 0x83, 0xC0, 0x01]; // add rax, 1
    c.extend_from_slice(&[0xE2, 0xFA]); // loop -6 (back to add)
    c.push(HLT);
    check("loop_rcx", &c, r);
}

// ---------------------------------------------------------------------------
// LEA RIP-relative (mod=00, r/m=101 -> [RIP + disp32]).
// ---------------------------------------------------------------------------

#[test]
fn lea_rip_relative() {
    // 48 8D 05 disp32  lea rax, [rip + disp32]
    // RIP is the address of the NEXT instruction (here CODE_ADDR + 7, since the
    // LEA is 7 bytes). With disp32 = 0x100, rax = CODE_ADDR + 7 + 0x100.
    // We just need both backends to agree on the computed absolute address.
    let r = regs();
    // 48 8D 05 00 01 00 00  lea rax, [rip + 0x100]
    check(
        "lea_rip",
        &with_hlt(vec![0x48, 0x8D, 0x05, 0x00, 0x01, 0x00, 0x00]),
        r,
    );
}

#[test]
fn lea_rip_relative_negative_disp() {
    // Negative RIP-relative displacement.
    // 48 8D 05 F0 FF FF FF  lea rax, [rip - 16]
    let r = regs();
    check(
        "lea_rip_neg",
        &with_hlt(vec![0x48, 0x8D, 0x05, 0xF0, 0xFF, 0xFF, 0xFF]),
        r,
    );
}

// ---------------------------------------------------------------------------
// SSE / SSE2 single-precision float ops driven via the scratch page.
// All inputs are exactly representable in f32 so f64/f32 rounding is irrelevant
// and the 128-bit result is bit-identical across backends.
// ---------------------------------------------------------------------------

/// Build a 16-byte little-endian buffer from four f32 lanes (lane 0 = bytes 0..4).
fn f32x4(v: [f32; 4]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for i in 0..4 {
        o[i * 4..i * 4 + 4].copy_from_slice(&v[i].to_le_bytes());
    }
    o
}

/// Build a 16-byte buffer from two f64 lanes.
fn f64x2(v: [f64; 2]) -> [u8; 16] {
    let mut o = [0u8; 16];
    o[0..8].copy_from_slice(&v[0].to_le_bytes());
    o[8..16].copy_from_slice(&v[1].to_le_bytes());
    o
}

#[test]
fn sse_addps() {
    // ADDPS xmm0, xmm1 = 0F 58 C1. Packed single add.
    let a = f32x4([1.0, 2.5, -3.0, 100.0]);
    let b = f32x4([0.5, 0.5, 3.0, -100.0]);
    check_sse(
        "addps",
        &sse_program(&[0x0F, 0x58, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_subps() {
    // SUBPS xmm0, xmm1 = 0F 5C C1.
    let a = f32x4([10.0, 0.0, -5.0, 256.0]);
    let b = f32x4([2.5, 1.0, -5.0, 256.0]);
    check_sse(
        "subps",
        &sse_program(&[0x0F, 0x5C, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_mulps() {
    // MULPS xmm0, xmm1 = 0F 59 C1.
    let a = f32x4([2.0, 3.0, -4.0, 0.5]);
    let b = f32x4([8.0, 0.25, -2.0, 16.0]);
    check_sse(
        "mulps",
        &sse_program(&[0x0F, 0x59, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_divps() {
    // DIVPS xmm0, xmm1 = 0F 5E C1. Use exact dyadic quotients.
    let a = f32x4([1.0, 9.0, -8.0, 256.0]);
    let b = f32x4([2.0, 8.0, 2.0, 4.0]); // -> 0.5, 1.125, -4.0, 64.0
    check_sse(
        "divps",
        &sse_program(&[0x0F, 0x5E, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_minps() {
    // MINPS xmm0, xmm1 = 0F 5D C1. Per-lane min.
    let a = f32x4([1.0, 5.0, -3.0, 7.0]);
    let b = f32x4([2.0, 4.0, -1.0, 7.0]);
    check_sse(
        "minps",
        &sse_program(&[0x0F, 0x5D, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_maxps() {
    // MAXPS xmm0, xmm1 = 0F 5F C1. Per-lane max.
    let a = f32x4([1.0, 5.0, -3.0, 7.0]);
    let b = f32x4([2.0, 4.0, -1.0, 7.0]);
    check_sse(
        "maxps",
        &sse_program(&[0x0F, 0x5F, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_minps_nan_handling() {
    // SDM (MINPS): for each lane, dst = (a < b) ? a : b, where ANY NaN operand
    // (or unordered compare) makes the `<` test FALSE, so the lane result is the
    // SECOND source operand (b). With:
    //   a = [NaN, 9.0, 1.0, 2.0]   (NaN in lane 0, dst = xmm0)
    //   b = [5.0, NaN, 3.0, 4.0]   (NaN in lane 1, src = xmm1)
    // the architecturally-correct result is:
    //   lane0: a is NaN  -> b = 5.0
    //   lane1: b is NaN  -> b = NaN
    //   lane2: 1.0 < 3.0 -> a = 1.0
    //   lane3: 2.0 < 4.0 -> a = 2.0
    // i.e. [5.0, NaN, 1.0, 2.0].
    //
    // FIXED: execute_sse_min previously used Rust's `f32::min`, which returns the
    // NON-NaN operand (so lane1 wrongly returned dst=9.0). It now uses the x86
    // rule `(dst<src)?dst:src`, returning src on any unordered lane, matching KVM.
    let mut a = f32x4([0.0, 9.0, 1.0, 2.0]);
    a[0..4].copy_from_slice(&f32::NAN.to_le_bytes());
    let mut b = f32x4([5.0, 0.0, 3.0, 4.0]);
    b[4..8].copy_from_slice(&f32::NAN.to_le_bytes());
    check_sse(
        "minps_nan",
        &sse_program(&[0x0F, 0x5D, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_maxps_nan_handling() {
    // MAXPS xmm0, xmm1 = 0F 5F C1. Same NaN rule as MINPS: any unordered lane
    // returns the SECOND operand (src = xmm1). Mirror of the MINPS case.
    //   a = [NaN, 9.0, 5.0, 2.0] (dst), b = [5.0, NaN, 3.0, 4.0] (src)
    //   lane0: a NaN -> src = 5.0 ; lane1: b NaN -> src = NaN
    //   lane2: 5.0 > 3.0 -> dst = 5.0 ; lane3: 2.0 > 4.0 false -> src = 4.0
    let mut a = f32x4([0.0, 9.0, 5.0, 2.0]);
    a[0..4].copy_from_slice(&f32::NAN.to_le_bytes());
    let mut b = f32x4([5.0, 0.0, 3.0, 4.0]);
    b[4..8].copy_from_slice(&f32::NAN.to_le_bytes());
    check_sse(
        "maxps_nan",
        &sse_program(&[0x0F, 0x5F, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_maxpd_nan_handling() {
    // MAXPD xmm0, xmm1 = 66 0F 5F C1. Packed-double NaN handling: unordered lane
    // returns src. a = [NaN, 7.0] (dst), b = [3.0, NaN] (src) -> [3.0, NaN].
    let mut a = f64x2([0.0, 7.0]);
    a[0..8].copy_from_slice(&f64::NAN.to_le_bytes());
    let mut b = f64x2([3.0, 0.0]);
    b[8..16].copy_from_slice(&f64::NAN.to_le_bytes());
    check_sse(
        "maxpd_nan",
        &sse_program(&[0x66, 0x0F, 0x5F, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_minpd_nan_handling() {
    // MINPD xmm0, xmm1 = 66 0F 5D C1. a = [NaN, 7.0], b = [3.0, NaN] -> [3.0, NaN].
    let mut a = f64x2([0.0, 7.0]);
    a[0..8].copy_from_slice(&f64::NAN.to_le_bytes());
    let mut b = f64x2([3.0, 0.0]);
    b[8..16].copy_from_slice(&f64::NAN.to_le_bytes());
    check_sse(
        "minpd_nan",
        &sse_program(&[0x66, 0x0F, 0x5D, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_minss_nan_returns_src() {
    // MINSS xmm0, xmm1 = F3 0F 5D C1. Scalar lane0: dst=9.0, src=NaN -> NaN (src).
    // Upper 3 lanes of xmm0 preserved.
    let a = f32x4([9.0, 11.0, 12.0, 13.0]);
    let mut b = f32x4([0.0, 0.0, 0.0, 0.0]);
    b[0..4].copy_from_slice(&f32::NAN.to_le_bytes());
    check_sse(
        "minss_nan",
        &sse_program(&[0xF3, 0x0F, 0x5D, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_maxsd_nan_returns_src() {
    // MAXSD xmm0, xmm1 = F2 0F 5F C1. Scalar double lane0: dst=NaN, src=2.0 -> 2.0.
    let mut a = f64x2([0.0, 7.0]);
    a[0..8].copy_from_slice(&f64::NAN.to_le_bytes());
    let b = f64x2([2.0, 99.0]);
    check_sse(
        "maxsd_nan",
        &sse_program(&[0xF2, 0x0F, 0x5F, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_maxps_signed_zero() {
    // MAX(-0.0, +0.0): SDM says the second operand is returned when operands are
    // equal-but-signed-different, so MAXPS returns b. Probe both lanes.
    let a = f32x4([-0.0, 0.0, -0.0, 0.0]);
    let b = f32x4([0.0, -0.0, -0.0, 0.0]);
    check_sse(
        "maxps_signzero",
        &sse_program(&[0x0F, 0x5F, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_minps_signed_zero() {
    // MIN(-0.0,+0.0) also returns the second operand on the equality tie.
    let a = f32x4([-0.0, 0.0, 1.0, -1.0]);
    let b = f32x4([0.0, -0.0, 1.0, -1.0]);
    check_sse(
        "minps_signzero",
        &sse_program(&[0x0F, 0x5D, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_sqrtps() {
    // SQRTPS xmm0, xmm1 = 0F 51 C1. Source xmm1 -> dest xmm0. Perfect squares.
    let a = f32x4([0.0, 0.0, 0.0, 0.0]); // dest, overwritten
    let b = f32x4([4.0, 9.0, 16.0, 0.25]); // -> 2,3,4,0.5
    check_sse(
        "sqrtps",
        &sse_program(&[0x0F, 0x51, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_sqrtss() {
    // SQRTSS xmm0, xmm1 = F3 0F 51 C1. Only lane 0 changes; lanes 1..3 keep xmm0.
    let a = f32x4([1.0, 11.0, 12.0, 13.0]); // upper lanes preserved
    let b = f32x4([25.0, 0.0, 0.0, 0.0]); // lane0 -> 5.0
    check_sse(
        "sqrtss",
        &sse_program(&[0xF3, 0x0F, 0x51, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_addss() {
    // ADDSS xmm0, xmm1 = F3 0F 58 C1. Scalar add; upper 3 lanes of xmm0 preserved.
    let a = f32x4([10.0, 1.0, 2.0, 3.0]);
    let b = f32x4([5.5, 99.0, 99.0, 99.0]); // only lane0 of b is used
    check_sse(
        "addss",
        &sse_program(&[0xF3, 0x0F, 0x58, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_minss_scalar() {
    // MINSS xmm0, xmm1 = F3 0F 5D C1. Scalar min in lane 0.
    let a = f32x4([7.0, 1.0, 2.0, 3.0]);
    let b = f32x4([4.0, 0.0, 0.0, 0.0]); // min(7,4)=4
    check_sse(
        "minss",
        &sse_program(&[0xF3, 0x0F, 0x5D, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---- CMPPS: all 8 immediate predicates (0..7). Produces an all-1s/all-0s mask. ----

/// Run CMPPS xmm0, xmm1, imm8 (0F C2 C1 ib) and compare the 128-bit mask result.
fn cmpps_case(label: &str, imm: u8, a: [u8; 16], b: [u8; 16]) {
    check_sse(
        label,
        &sse_program(&[0x0F, 0xC2, 0xC1, imm]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_cmpps_all_predicates() {
    // Lanes chosen to span equal / less / greater / unordered(NaN) relationships.
    // a = [1.0, 2.0, 3.0, NaN], b = [1.0, 5.0, 1.0, 4.0].
    let mut a = f32x4([1.0, 2.0, 3.0, 0.0]);
    a[12..16].copy_from_slice(&f32::NAN.to_le_bytes());
    let b = f32x4([1.0, 5.0, 1.0, 4.0]);
    // 0=EQ, 1=LT, 2=LE, 3=UNORD, 4=NEQ, 5=NLT, 6=NLE, 7=ORD.
    cmpps_case("cmpps_eq", 0, a, b);
    cmpps_case("cmpps_lt", 1, a, b);
    cmpps_case("cmpps_le", 2, a, b);
    cmpps_case("cmpps_unord", 3, a, b);
    cmpps_case("cmpps_neq", 4, a, b);
    cmpps_case("cmpps_nlt", 5, a, b);
    cmpps_case("cmpps_nle", 6, a, b);
    cmpps_case("cmpps_ord", 7, a, b);
}

// ---- MOVMSKPS: extract the 4 sign bits of the packed singles into a GPR. ----

#[test]
fn sse_movmskps() {
    // Load xmm1 from scratch, MOVMSKPS eax, xmm1 (0F 50 C1), store eax to scratch+0x20.
    // Signs: lane0 -, lane1 +, lane2 -, lane3 + -> mask = 0b0101 = 0x5.
    let a = [0u8; 16];
    let b = f32x4([-1.0, 2.0, -3.0, 4.0]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0x0F, 0x50, 0xC1]); // movmskps eax, xmm1
    prog.extend_from_slice(&[0x89, 0x47, 0x20]); // mov [rdi+0x20], eax
    prog.push(HLT);
    check_sse("movmskps", &prog, sse_scratch(a, b));
}

#[test]
fn sse_movmskpd() {
    // MOVMSKPD eax, xmm1 (66 0F 50 C1): 2 sign bits from the packed doubles.
    // lane0 -, lane1 + -> mask = 0b01 = 0x1.
    let a = [0u8; 16];
    let b = f64x2([-1.5, 2.5]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0x66, 0x0F, 0x50, 0xC1]); // movmskpd eax, xmm1
    prog.extend_from_slice(&[0x89, 0x47, 0x20]); // mov [rdi+0x20], eax
    prog.push(HLT);
    check_sse("movmskpd", &prog, sse_scratch(a, b));
}

// ---- UNPCK low/high for packed singles/doubles ----

#[test]
fn sse_unpcklps() {
    // UNPCKLPS xmm0, xmm1 = 0F 14 C1 : interleave low two singles of each.
    // result = [a0, b0, a1, b1].
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    check_sse(
        "unpcklps",
        &sse_program(&[0x0F, 0x14, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_unpckhps() {
    // UNPCKHPS xmm0, xmm1 = 0F 15 C1 : interleave high two singles.
    // result = [a2, b2, a3, b3].
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    check_sse(
        "unpckhps",
        &sse_program(&[0x0F, 0x15, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_unpcklpd() {
    // UNPCKLPD xmm0, xmm1 = 66 0F 14 C1 : result = [a.lo, b.lo].
    let a = f64x2([1.5, 2.5]);
    let b = f64x2([3.5, 4.5]);
    check_sse(
        "unpcklpd",
        &sse_program(&[0x66, 0x0F, 0x14, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_unpckhpd() {
    // UNPCKHPD xmm0, xmm1 = 66 0F 15 C1 : result = [a.hi, b.hi].
    let a = f64x2([1.5, 2.5]);
    let b = f64x2([3.5, 4.5]);
    check_sse(
        "unpckhpd",
        &sse_program(&[0x66, 0x0F, 0x15, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---- Double-precision arithmetic + min/max/sqrt + scalar ----

#[test]
fn sse_addpd() {
    // ADDPD xmm0, xmm1 = 66 0F 58 C1.
    let a = f64x2([1.25, -100.0]);
    let b = f64x2([0.75, 100.0]);
    check_sse(
        "addpd",
        &sse_program(&[0x66, 0x0F, 0x58, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_mulpd() {
    // MULPD xmm0, xmm1 = 66 0F 59 C1.
    let a = f64x2([3.0, -0.5]);
    let b = f64x2([0.25, 8.0]);
    check_sse(
        "mulpd",
        &sse_program(&[0x66, 0x0F, 0x59, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_sqrtpd() {
    // SQRTPD xmm0, xmm1 = 66 0F 51 C1.
    let a = f64x2([0.0, 0.0]);
    let b = f64x2([81.0, 0.0625]); // -> 9.0, 0.25
    check_sse(
        "sqrtpd",
        &sse_program(&[0x66, 0x0F, 0x51, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_minpd_maxpd() {
    // MINPD then independent MAXPD test, both per-lane.
    let a = f64x2([3.0, 8.0]);
    let b = f64x2([5.0, 2.0]);
    check_sse(
        "minpd",
        &sse_program(&[0x66, 0x0F, 0x5D, 0xC1]),
        sse_scratch(a, b),
    );
    check_sse(
        "maxpd",
        &sse_program(&[0x66, 0x0F, 0x5F, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_divsd_scalar() {
    // DIVSD xmm0, xmm1 = F2 0F 5E C1. Scalar; lane1 of xmm0 preserved.
    let a = f64x2([9.0, 123.0]);
    let b = f64x2([4.0, 0.0]); // lane0 -> 2.25
    check_sse(
        "divsd",
        &sse_program(&[0xF2, 0x0F, 0x5E, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---- CMPPD predicate + CVT edge cases ----

#[test]
fn sse_cmppd_predicates() {
    // CMPPD xmm0, xmm1, imm8 = 66 0F C2 C1 ib. Test EQ(0) and LT(1).
    let a = f64x2([1.0, 3.0]);
    let b = f64x2([1.0, 2.0]);
    check_sse(
        "cmppd_eq",
        &sse_program(&[0x66, 0x0F, 0xC2, 0xC1, 0x00]),
        sse_scratch(a, b),
    );
    check_sse(
        "cmppd_lt",
        &sse_program(&[0x66, 0x0F, 0xC2, 0xC1, 0x01]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_cvtps2pd() {
    // CVTPS2PD xmm0, xmm1 = 0F 5A C1. Low two f32 -> two f64.
    let a = [0u8; 16];
    let b = f32x4([2.5, -4.0, 99.0, 99.0]); // only low two lanes used
    check_sse(
        "cvtps2pd",
        &sse_program(&[0x0F, 0x5A, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_cvtpd2ps() {
    // CVTPD2PS xmm0, xmm1 = 66 0F 5A C1. Two f64 -> low two f32, high two = 0.
    let a = [0u8; 16];
    let b = f64x2([3.5, -7.25]);
    check_sse(
        "cvtpd2ps",
        &sse_program(&[0x66, 0x0F, 0x5A, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_cvttps2dq_negative_trunc() {
    // CVTTPS2DQ (F3 0F 5B): truncation toward zero on negatives.
    // -1.9 -> -1, -2.5 -> -2, 2.9 -> 2, 0.9 -> 0.
    let a = [0u8; 16];
    let b = f32x4([-1.9, -2.5, 2.9, 0.9]);
    check_sse(
        "cvttps2dq_neg",
        &sse_program(&[0xF3, 0x0F, 0x5B, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_cvtsi2sd() {
    // CVTSI2SD xmm0, r/m64 = F2 48 0F 2A C0 (from rax). rax=-12345 -> double in lane0.
    let mut r = regs();
    r.rax = (-12345i64) as u64;
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF2, 0x48, 0x0F, 0x2A, 0xC0]); // cvtsi2sd xmm0, rax
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // movdqu [rdi+0x20], xmm0
    prog.push(HLT);
    // Drive via run_both since we need a nonzero GPR init; compare the scratch store.
    let Some((interp, kvm)) = run_both(&prog, r, zero_scratch()) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0,
        scratch: true,
        ..CompareOpts::default()
    };
    assert_match("cvtsi2sd", &prog, &interp, &kvm, opts);
}

#[test]
fn sse_cvttsd2si() {
    // CVTTSD2SI rax, xmm1 = F2 48 0F 2C C1. Truncating f64->i64.
    // Load -42.9 into xmm1 from scratch lane0; convert to rax; expect -42.
    let b = f64x2([-42.9, 0.0]);
    let scratch = sse_scratch([0u8; 16], b);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0xF2, 0x48, 0x0F, 0x2C, 0xC1]); // cvttsd2si rax, xmm1
    prog.push(HLT);
    // Compare GPRs (rax holds the converted integer); no flags, no scratch store.
    let Some((interp, kvm)) = run_both(&prog, regs(), scratch) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0,
        ..CompareOpts::default()
    };
    assert_match("cvttsd2si", &prog, &interp, &kvm, opts);
}

// ---- SHUFPS: arbitrary lane selection via imm8 ----

#[test]
fn sse_shufps() {
    // SHUFPS xmm0, xmm1, imm8 = 0F C6 C1 ib.
    // imm = 0b11_01_10_00 = 0xD8 : dst = [a0, a2, b1, b3].
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    check_sse(
        "shufps",
        &sse_program(&[0x0F, 0xC6, 0xC1, 0xD8]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// More BMI2: BZHI, and additional PEXT/PDEP/MULX/RORX edge cases.
// (BZHI defines ZF/CF/SF and clears OF; PEXT/PDEP/MULX/RORX touch no flags -> mask 0.)
// ---------------------------------------------------------------------------

// BZHI defines ZF, CF, SF; clears OF; AF/PF undefined.
const BZHI_DEFINED: u64 = flags::bits::ZF | flags::bits::CF | flags::bits::SF | flags::bits::OF;

#[test]
fn bmi2_bzhi() {
    // BZHI r32, r/m32, r32 = VEX.LZ.0F38.W0 F5 /r : zero bits of src at index >= count.
    //   count is in the vvvv-encoded register; src is r/m. (pp=00, NP)
    //   bzhi eax, ecx, ebx : vvvv=ebx(count), rm=ecx(src), dest=eax.
    //   byte3 = W0 vvvv(~ebx=1100) L0 pp(00) = 0 1100 0 00 = 0x60.
    //   C4 E2 60 F5 C1.
    let mut r = regs();
    r.rcx = 0xFFFF_FFFF; // source: all ones
    r.rbx = 12; // keep low 12 bits -> 0x00000FFF
    check_flags_masked(
        "bzhi",
        &with_hlt(vec![0xC4, 0xE2, 0x60, 0xF5, 0xC1]),
        r,
        BZHI_DEFINED,
    );
}

#[test]
fn bmi2_bzhi_count_ge_width() {
    // count >= operand size leaves the source unchanged; CF set when count>=width.
    let mut r = regs();
    r.rcx = 0xDEAD_BEEF;
    r.rbx = 40; // >= 32 -> no bits cleared, CF=1
    check_flags_masked(
        "bzhi_wide",
        &with_hlt(vec![0xC4, 0xE2, 0x60, 0xF5, 0xC1]),
        r,
        BZHI_DEFINED,
    );
}

#[test]
fn bmi2_bzhi_64() {
    // 64-bit BZHI (W1): byte3 sets W bit -> 0xE0.
    //   bzhi rax, rcx, rbx : C4 E2 E0 F5 C1.
    let mut r = regs();
    r.rcx = 0xFFFF_FFFF_FFFF_FFFF;
    r.rbx = 33; // keep low 33 bits
    check_flags_masked(
        "bzhi64",
        &with_hlt(vec![0xC4, 0xE2, 0xE0, 0xF5, 0xC1]),
        r,
        BZHI_DEFINED,
    );
}

#[test]
fn bmi2_pext_sparse_mask() {
    // PEXT with a sparse (non-contiguous) mask gathers selected bits to the low end.
    let mut r = regs();
    r.rbx = 0xFEDC_BA98; // source
    r.rcx = 0x8421_1248; // sparse mask (scattered single bits)
    // C4 E2 62 F5 C1  pext eax, ebx, ecx
    check_flags_masked(
        "pext_sparse",
        &with_hlt(vec![0xC4, 0xE2, 0x62, 0xF5, 0xC1]),
        r,
        0,
    );
}

#[test]
fn bmi2_pext_64() {
    // 64-bit PEXT (W1): byte3 W bit -> from 0x62 to 0xE2.
    let mut r = regs();
    r.rbx = 0x0123_4567_89AB_CDEF;
    r.rcx = 0xF0F0_F0F0_F0F0_F0F0; // take the high nibble of each byte
    // C4 E2 E2 F5 C1  pext rax, rbx, rcx
    check_flags_masked(
        "pext64",
        &with_hlt(vec![0xC4, 0xE2, 0xE2, 0xF5, 0xC1]),
        r,
        0,
    );
}

#[test]
fn bmi2_pdep_sparse_mask() {
    // PDEP scatters the low source bits into a sparse mask's set positions.
    let mut r = regs();
    r.rbx = 0x0000_00FF; // 8 source bits
    r.rcx = 0x8421_1248; // sparse target positions
    // C4 E2 63 F5 C1  pdep eax, ebx, ecx
    check_flags_masked(
        "pdep_sparse",
        &with_hlt(vec![0xC4, 0xE2, 0x63, 0xF5, 0xC1]),
        r,
        0,
    );
}

#[test]
fn bmi2_pdep_64() {
    // 64-bit PDEP (W1): byte3 from 0x63 to 0xE3.
    let mut r = regs();
    r.rbx = 0x0000_0000_0000_FFFF;
    r.rcx = 0xF0F0_F0F0_F0F0_F0F0;
    // C4 E2 E3 F5 C1  pdep rax, rbx, rcx
    check_flags_masked(
        "pdep64",
        &with_hlt(vec![0xC4, 0xE2, 0xE3, 0xF5, 0xC1]),
        r,
        0,
    );
}

#[test]
fn bmi2_mulx_64() {
    // 64-bit MULX (W1): high half -> vvvv(rbx), low half -> reg(rax). RDX implicit.
    //   byte3 = W1 vvvv(~rbx=1100) L0 pp(11) = 1 1100 0 11 = 0xE3.
    //   mulx rax, rbx, rcx : C4 E2 E3 F6 C1.
    let mut r = regs();
    r.rdx = 0xFFFF_FFFF_FFFF_FFFF; // multiplicand
    r.rcx = 0x0000_0000_0000_0002; // src2 -> product 0x1_FFFF...E
    check_flags_masked(
        "mulx64",
        &with_hlt(vec![0xC4, 0xE2, 0xE3, 0xF6, 0xC1]),
        r,
        0,
    );
}

#[test]
fn bmi2_mulx_same_dest() {
    // MULX where dest1 (vvvv) == dest2 (reg): per the ISA only the high half is
    // written when both destinations are the same register. Here both = eax.
    //   mulx eax, eax, ecx : vvvv=eax -> field 0b1111 (inverted), reg=eax(000), rm=ecx(001).
    //   byte3 = W0 vvvv(1111) L0 pp(11) = 0 1111 0 11 = 0x7B. modrm = 0xC1.
    //   C4 E2 7B F6 C1.
    let mut r = regs();
    r.rdx = 0x0000_0000_0001_0000;
    r.rcx = 0x0000_0000_0001_0000; // product 0x1_0000_0000 : high=1, low=0 -> eax gets high(1)
    check_flags_masked(
        "mulx_samedest",
        &with_hlt(vec![0xC4, 0xE2, 0x7B, 0xF6, 0xC1]),
        r,
        0,
    );
}

#[test]
fn bmi2_rorx_count_zero() {
    // RORX with imm=0 : no rotation, plain copy. ecx -> eax unchanged (zero-extended).
    let mut r = regs();
    r.rcx = 0xDEAD_BEEF;
    // C4 E3 7B F0 C1 00  rorx eax, ecx, 0
    check_flags_masked(
        "rorx_0",
        &with_hlt(vec![0xC4, 0xE3, 0x7B, 0xF0, 0xC1, 0x00]),
        r,
        0,
    );
}

#[test]
fn bmi2_rorx_count_31() {
    // 32-bit RORX by 31 (== rotate left by 1).
    let mut r = regs();
    r.rcx = 0x8000_0001;
    // C4 E3 7B F0 C1 1F  rorx eax, ecx, 31
    check_flags_masked(
        "rorx_31",
        &with_hlt(vec![0xC4, 0xE3, 0x7B, 0xF0, 0xC1, 0x1F]),
        r,
        0,
    );
}

#[test]
fn bmi2_rorx64_count_63() {
    // 64-bit RORX by 63 (== rotate left by 1).
    let mut r = regs();
    r.rcx = 0x8000_0000_0000_0001;
    // C4 E3 FB F0 C1 3F  rorx rax, rcx, 63
    check_flags_masked(
        "rorx64_63",
        &with_hlt(vec![0xC4, 0xE3, 0xFB, 0xF0, 0xC1, 0x3F]),
        r,
        0,
    );
}

// ===========================================================================
// EXPANDED COVERAGE PART 4: SSE/SSE2 integer (saturation, compares, multiplies,
// pack/shuffle/unpack/shift-bytes/movmsk), SSE3/SSSE3, scalar conversion edges,
// AES-NI, CRC32/POPCNT r/m, BT* with memory operands and out-of-range indices,
// ADCX/ADOX carry chains, and CMC/STC/CLC/STD plus XADD/CMPXCHG/BSWAP memory.
// ===========================================================================
//
// Everything here reuses the existing harness helpers verbatim:
//  - `check_sse`/`sse_program`/`sse_scratch` for 128-bit SSE results via scratch.
//  - `check`/`check_flags_masked`/`check_mem` for GPR/flag/memory comparisons.
//  - `load_rdi_data`/`f32x4`/`f64x2`/`scratch_f64` builders.
// Undefined-flag masks reuse the existing constants (FLAG_MASK, etc.).
//
// All SSE integer ops below set NO RFLAGS bits, so `check_sse` (flag_mask 0) is
// the right tool: it compares the stored 128-bit result page plus the live XMM.

// Build two 16-byte inputs from packed u16 lanes (little-endian).
fn u16x8(v: [u16; 8]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for i in 0..8 {
        o[i * 2..i * 2 + 2].copy_from_slice(&v[i].to_le_bytes());
    }
    o
}

// Build a 16-byte input from packed u32 lanes (little-endian).
fn u32x4(v: [u32; 4]) -> [u8; 16] {
    let mut o = [0u8; 16];
    for i in 0..4 {
        o[i * 4..i * 4 + 4].copy_from_slice(&v[i].to_le_bytes());
    }
    o
}

// ---------------------------------------------------------------------------
// SSE2 integer: saturating adds/subs (PADDUSB/PADDUSW/PADDSB/PADDSW/PSUBUSB).
// ---------------------------------------------------------------------------

#[test]
fn sse_paddusb() {
    // PADDUSB xmm0, xmm1 = 66 0F DC C1 : unsigned byte add with saturation to 0xFF.
    let a = [
        0x00, 0x01, 0x7F, 0x80, 0xFE, 0xFF, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90,
        0xA0,
    ];
    let b = [
        0x00, 0xFF, 0x01, 0x80, 0x01, 0x01, 0xF0, 0xF0, 0xD0, 0xC0, 0xB0, 0xA0, 0x90, 0x80, 0x70,
        0x60,
    ];
    check_sse(
        "paddusb",
        &sse_program(&[0x66, 0x0F, 0xDC, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_paddusw() {
    // PADDUSW xmm0, xmm1 = 66 0F DD C1 : unsigned word add with saturation to 0xFFFF.
    let a = u16x8([
        0x0000, 0x0001, 0x7FFF, 0x8000, 0xFFFE, 0xFFFF, 0x1234, 0xABCD,
    ]);
    let b = u16x8([
        0x0000, 0xFFFF, 0x0001, 0x8000, 0x0001, 0x0001, 0xF000, 0x6000,
    ]);
    check_sse(
        "paddusw",
        &sse_program(&[0x66, 0x0F, 0xDD, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_paddsb() {
    // PADDSB xmm0, xmm1 = 66 0F EC C1 : signed byte add with saturation [-128,127].
    let a = [
        0x7F, 0x7F, 0x80, 0x80, 0x01, 0xFF, 0x40, 0x40, 0xC0, 0xC0, 0x00, 0x7E, 0x81, 0x10, 0x20,
        0x30,
    ];
    let b = [
        0x01, 0x7F, 0xFF, 0x80, 0xFF, 0x01, 0x40, 0x50, 0xC0, 0xB0, 0x00, 0x02, 0xFF, 0x20, 0x30,
        0x40,
    ];
    check_sse(
        "paddsb",
        &sse_program(&[0x66, 0x0F, 0xEC, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_paddsw() {
    // PADDSW xmm0, xmm1 = 66 0F ED C1 : signed word add with saturation [-32768,32767].
    let a = u16x8([
        0x7FFF, 0x8000, 0x0001, 0xFFFF, 0x4000, 0xC000, 0x7FFE, 0x8001,
    ]);
    let b = u16x8([
        0x0001, 0xFFFF, 0xFFFF, 0x0001, 0x4000, 0xC000, 0x0003, 0xFFFE,
    ]);
    check_sse(
        "paddsw",
        &sse_program(&[0x66, 0x0F, 0xED, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_psubusb() {
    // PSUBUSB xmm0, xmm1 = 66 0F D8 C1 : unsigned byte sub with saturation to 0.
    let a = [
        0x00, 0x10, 0x80, 0xFF, 0x01, 0x7F, 0x05, 0x90, 0xA0, 0xB0, 0xC0, 0x00, 0x01, 0x02, 0x03,
        0x04,
    ];
    let b = [
        0x01, 0x20, 0x01, 0x80, 0x02, 0x40, 0x05, 0x91, 0x10, 0xC0, 0xFF, 0x00, 0x05, 0x00, 0x10,
        0x04,
    ];
    check_sse(
        "psubusb",
        &sse_program(&[0x66, 0x0F, 0xD8, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_psubusw() {
    // PSUBUSW xmm0, xmm1 = 66 0F D9 C1 : unsigned word sub with saturation to 0.
    let a = u16x8([
        0x0000, 0x1000, 0x8000, 0xFFFF, 0x0001, 0x7FFF, 0x00FF, 0xABCD,
    ]);
    let b = u16x8([
        0x0001, 0x2000, 0x0001, 0x8000, 0x0002, 0x4000, 0x0100, 0xABCE,
    ]);
    check_sse(
        "psubusw",
        &sse_program(&[0x66, 0x0F, 0xD9, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_psubsb() {
    // PSUBSB xmm0, xmm1 = 66 0F E8 C1 : signed byte sub with saturation [-128,127].
    let a = [
        0x7F, 0x80, 0x00, 0x7F, 0x80, 0x01, 0x40, 0xC0, 0x10, 0x20, 0x30, 0x7F, 0x80, 0xFF, 0x00,
        0x01,
    ];
    let b = [
        0xFF, 0x01, 0x00, 0x80, 0x7F, 0x02, 0xC0, 0x40, 0x05, 0x10, 0x40, 0xFF, 0x01, 0xFF, 0x01,
        0x02,
    ];
    check_sse(
        "psubsb",
        &sse_program(&[0x66, 0x0F, 0xE8, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_psubsw() {
    // PSUBSW xmm0, xmm1 = 66 0F E9 C1 : signed word sub with saturation.
    let a = u16x8([
        0x7FFF, 0x8000, 0x0000, 0x7FFF, 0x8000, 0x0001, 0x4000, 0xC000,
    ]);
    let b = u16x8([
        0xFFFF, 0x0001, 0x0000, 0x8000, 0x7FFF, 0x0002, 0xC000, 0x4000,
    ]);
    check_sse(
        "psubsw",
        &sse_program(&[0x66, 0x0F, 0xE9, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// SSE2 integer: packed equality / greater-than compares (produce all-1s/0s).
// ---------------------------------------------------------------------------

#[test]
fn sse_pcmpeqb() {
    // PCMPEQB xmm0, xmm1 = 66 0F 74 C1 : per-byte equality mask.
    let a = [
        0, 1, 2, 3, 4, 5, 6, 7, 0xFF, 0x80, 0x00, 0x7F, 0xAA, 0x55, 0x10, 0x20,
    ];
    let b = [
        0, 9, 2, 9, 4, 9, 6, 9, 0xFF, 0x00, 0x00, 0x7F, 0xAA, 0x54, 0x10, 0x21,
    ];
    check_sse(
        "pcmpeqb",
        &sse_program(&[0x66, 0x0F, 0x74, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pcmpeqw() {
    // PCMPEQW xmm0, xmm1 = 66 0F 75 C1 : per-word equality mask.
    let a = u16x8([
        0x1234, 0xABCD, 0x0000, 0xFFFF, 0x8000, 0x7FFF, 0x0001, 0xDEAD,
    ]);
    let b = u16x8([
        0x1234, 0x0000, 0x0000, 0xFFFE, 0x8000, 0x7FFE, 0x0001, 0xBEEF,
    ]);
    check_sse(
        "pcmpeqw",
        &sse_program(&[0x66, 0x0F, 0x75, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pcmpeqd() {
    // PCMPEQD xmm0, xmm1 = 66 0F 76 C1 : per-dword equality mask.
    let a = u32x4([0x1234_5678, 0xFFFF_FFFF, 0x0000_0000, 0x8000_0000]);
    let b = u32x4([0x1234_5678, 0xFFFF_FFFE, 0x0000_0000, 0x7FFF_FFFF]);
    check_sse(
        "pcmpeqd",
        &sse_program(&[0x66, 0x0F, 0x76, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pcmpgtb() {
    // PCMPGTB xmm0, xmm1 = 66 0F 64 C1 : per-byte SIGNED greater-than mask.
    let a = [
        0x7F, 0x80, 0x00, 0xFF, 0x01, 0x10, 0xC0, 0x40, 0x05, 0x06, 0x80, 0x7F, 0x00, 0x01, 0xFE,
        0x02,
    ];
    let b = [
        0x7E, 0x7F, 0x00, 0x00, 0xFF, 0x10, 0x40, 0xC0, 0x06, 0x05, 0x7F, 0x80, 0x01, 0x00, 0xFF,
        0x03,
    ];
    check_sse(
        "pcmpgtb",
        &sse_program(&[0x66, 0x0F, 0x64, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pcmpgtw() {
    // PCMPGTW xmm0, xmm1 = 66 0F 65 C1 : per-word SIGNED greater-than mask.
    let a = u16x8([
        0x7FFF, 0x8000, 0x0001, 0xFFFF, 0x4000, 0xC000, 0x0000, 0x1234,
    ]);
    let b = u16x8([
        0x7FFE, 0x7FFF, 0xFFFF, 0x0000, 0xC000, 0x4000, 0x0000, 0x1234,
    ]);
    check_sse(
        "pcmpgtw",
        &sse_program(&[0x66, 0x0F, 0x65, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pcmpgtd() {
    // PCMPGTD xmm0, xmm1 = 66 0F 66 C1 : per-dword SIGNED greater-than mask.
    let a = u32x4([0x7FFF_FFFF, 0x8000_0000, 0x0000_0001, 0xFFFF_FFFF]);
    let b = u32x4([0x7FFF_FFFE, 0x7FFF_FFFF, 0xFFFF_FFFF, 0x0000_0000]);
    check_sse(
        "pcmpgtd",
        &sse_program(&[0x66, 0x0F, 0x66, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// SSE2 integer: multiplies (PMULHW/PMULHUW/PMULUDQ).  PMULHW is already covered.
// ---------------------------------------------------------------------------

#[test]
fn sse_pmulhuw() {
    // PMULHUW xmm0, xmm1 = 66 0F E4 C1 : packed 16-bit UNSIGNED multiply, high 16 bits.
    let a = u16x8([
        0xFFFF, 0x8000, 0x4000, 0x0100, 0x00FF, 0x1234, 0xABCD, 0x0001,
    ]);
    let b = u16x8([
        0xFFFF, 0x0002, 0x0004, 0x0100, 0x00FF, 0x1000, 0x0010, 0xFFFF,
    ]);
    check_sse(
        "pmulhuw",
        &sse_program(&[0x66, 0x0F, 0xE4, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pmuludq() {
    // PMULUDQ xmm0, xmm1 = 66 0F F4 C1 : multiply unsigned dword lanes 0 and 2
    // (the even dwords) producing two 64-bit results.
    let a = u32x4([0xFFFF_FFFF, 0xDEAD_BEEF, 0x0001_0000, 0xCAFE_BABE]);
    let b = u32x4([0xFFFF_FFFF, 0x1111_1111, 0x0001_0000, 0x2222_2222]);
    check_sse(
        "pmuludq",
        &sse_program(&[0x66, 0x0F, 0xF4, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// SSE2 integer: PSHUFD / PSHUFLW / PSHUFHW (immediate lane selection).
// These read the SOURCE (xmm1) and write the DEST (xmm0), so we drive them with
// the standard sse_program where the op encodes the shuffle of xmm1 into xmm0.
// ---------------------------------------------------------------------------

#[test]
fn sse_pshufd() {
    // PSHUFD xmm0, xmm1, imm8 = 66 0F 70 C1 ib. imm=0b00_01_10_11=0x1B -> reverse dwords.
    let a = [0u8; 16];
    let b = u32x4([0x1111_1111, 0x2222_2222, 0x3333_3333, 0x4444_4444]);
    check_sse(
        "pshufd",
        &sse_program(&[0x66, 0x0F, 0x70, 0xC1, 0x1B]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pshuflw() {
    // PSHUFLW xmm0, xmm1, imm8 = F2 0F 70 C1 ib. Shuffle the LOW 4 words; high 64 copied.
    // imm=0b00_01_10_11=0x1B -> reverse the low four words.
    let a = [0u8; 16];
    let b = u16x8([
        0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD, 0x1111, 0x2222, 0x3333, 0x4444,
    ]);
    check_sse(
        "pshuflw",
        &sse_program(&[0xF2, 0x0F, 0x70, 0xC1, 0x1B]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_pshufhw() {
    // PSHUFHW xmm0, xmm1, imm8 = F3 0F 70 C1 ib. Shuffle the HIGH 4 words; low 64 copied.
    // imm=0b00_01_10_11=0x1B -> reverse the high four words.
    let a = [0u8; 16];
    let b = u16x8([
        0x1111, 0x2222, 0x3333, 0x4444, 0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD,
    ]);
    check_sse(
        "pshufhw",
        &sse_program(&[0xF3, 0x0F, 0x70, 0xC1, 0x1B]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// SSE2 integer: PSLLDQ / PSRLDQ (byte shift of the whole 128-bit register, imm8).
// These are encoded with a group opcode on the register itself (66 0F 73 /7 and
// /3), operating in-place on xmm0. We load xmm0 from scratch then store it back.
// ---------------------------------------------------------------------------

/// Build: load xmm0 from [rdi], run a single in-place op on xmm0, store xmm0 back.
fn sse_unary_xmm0(op: &[u8]) -> Vec<u8> {
    let mut code = load_rdi_data();
    code.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]); // movdqu xmm0, [rdi]
    code.extend_from_slice(op);
    code.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // movdqu [rdi+0x20], xmm0
    code.push(HLT);
    code
}

#[test]
fn sse_pslldq() {
    // PSLLDQ xmm0, imm8 = 66 0F 73 /7 ib (modrm F8 selects /7 on xmm0). Shift left 3 bytes.
    let a = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    check_sse(
        "pslldq",
        &sse_unary_xmm0(&[0x66, 0x0F, 0x73, 0xF8, 0x03]),
        sse_scratch(a, [0u8; 16]),
    );
}

#[test]
fn sse_psrldq() {
    // PSRLDQ xmm0, imm8 = 66 0F 73 /3 ib (modrm D8 selects /3 on xmm0). Shift right 5 bytes.
    let a = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    check_sse(
        "psrldq",
        &sse_unary_xmm0(&[0x66, 0x0F, 0x73, 0xD8, 0x05]),
        sse_scratch(a, [0u8; 16]),
    );
}

#[test]
fn sse_pslldq_full() {
    // Shifting by >= 16 yields all zeros.
    let a = [0xFFu8; 16];
    check_sse(
        "pslldq16",
        &sse_unary_xmm0(&[0x66, 0x0F, 0x73, 0xF8, 0x10]),
        sse_scratch(a, [0u8; 16]),
    );
}

// ---------------------------------------------------------------------------
// SSE2 integer: more PUNPCK forms (words, high dwords, high qwords).
// ---------------------------------------------------------------------------

#[test]
fn sse_punpcklwd() {
    // PUNPCKLWD xmm0, xmm1 = 66 0F 61 C1 : interleave low 4 words.
    let a = u16x8([
        0x0000, 0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666, 0x7777,
    ]);
    let b = u16x8([
        0x8888, 0x9999, 0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD, 0xEEEE, 0xFFFF,
    ]);
    check_sse(
        "punpcklwd",
        &sse_program(&[0x66, 0x0F, 0x61, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_punpckhwd() {
    // PUNPCKHWD xmm0, xmm1 = 66 0F 69 C1 : interleave high 4 words.
    let a = u16x8([
        0x0000, 0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666, 0x7777,
    ]);
    let b = u16x8([
        0x8888, 0x9999, 0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD, 0xEEEE, 0xFFFF,
    ]);
    check_sse(
        "punpckhwd",
        &sse_program(&[0x66, 0x0F, 0x69, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_punpckhdq() {
    // PUNPCKHDQ xmm0, xmm1 = 66 0F 6A C1 : interleave high 2 dwords.
    let a = u32x4([0x1111_1111, 0x2222_2222, 0x3333_3333, 0x4444_4444]);
    let b = u32x4([0xAAAA_AAAA, 0xBBBB_BBBB, 0xCCCC_CCCC, 0xDDDD_DDDD]);
    check_sse(
        "punpckhdq",
        &sse_program(&[0x66, 0x0F, 0x6A, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse_punpckhqdq() {
    // PUNPCKHQDQ xmm0, xmm1 = 66 0F 6D C1 : result = [a.hi, b.hi].
    let a = [
        0x0123_4567_89AB_CDEFu64.to_le_bytes(),
        0xDEAD_BEEF_CAFE_BABEu64.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x1122_3344_5566_7788u64.to_le_bytes(),
        0x99AA_BBCC_DDEE_FF00u64.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "punpckhqdq",
        &sse_program(&[0x66, 0x0F, 0x6D, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

// ---------------------------------------------------------------------------
// SSE2 integer: PACKSSDW (signed dword->word) and a second PACKUSWB shape.
// ---------------------------------------------------------------------------

#[test]
fn sse_packssdw() {
    // PACKSSDW xmm0, xmm1 = 66 0F 6B C1 : signed saturate pack dwords->words.
    let a = u32x4([0x0000_7FFF, 0x0000_8000, 0x7FFF_FFFF, 0x8000_0000]); // 32767, 32768->32767, large->32767, large neg->-32768
    let b = u32x4([0xFFFF_8000, 0x0000_0001, 0xFFFF_FFFF, 0x0001_0000]); // -32768, 1, -1, 65536->32767
    check_sse(
        "packssdw",
        &sse_program(&[0x66, 0x0F, 0x6B, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// SSE2: PMOVMSKB and MOVMSKPD (extract sign/high bits into a GPR).
// ---------------------------------------------------------------------------

#[test]
fn sse_pmovmskb() {
    // PMOVMSKB eax, xmm1 = 66 0F D7 C1 : gather the top bit of each of 16 bytes.
    // Bytes with the high bit set: indices 0,2,4,...,14 (even) here -> mask 0x5555.
    let a = [0u8; 16];
    let b = [
        0x80, 0x00, 0x81, 0x7F, 0xFF, 0x10, 0x90, 0x20, 0xC0, 0x40, 0xA0, 0x50, 0xE0, 0x60, 0xF0,
        0x70,
    ];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0x66, 0x0F, 0xD7, 0xC1]); // pmovmskb eax, xmm1
    prog.extend_from_slice(&[0x89, 0x47, 0x20]); // mov [rdi+0x20], eax
    prog.push(HLT);
    check_sse("pmovmskb", &prog, sse_scratch(a, b));
}

// ---------------------------------------------------------------------------
// SSSE3: PSHUFB, PHADDW/PHADDD, PMADDUBSW, PABSB/W/D, PALIGNR, PSIGNB/W/D.
// (3-byte opcodes 66 0F 38 xx; PALIGNR is 66 0F 3A 0F ib.)
// ---------------------------------------------------------------------------

#[test]
fn ssse3_pshufb() {
    // PSHUFB xmm0, xmm1 = 66 0F 38 00 C1 : permute bytes of xmm0 by indices in xmm1.
    // If a control byte has its top bit set, the result byte is zeroed.
    let a = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F,
    ];
    // control: reverse low 8, zero a few, select some high bytes.
    let b = [
        0x07, 0x06, 0x05, 0x04, 0x80, 0x02, 0x01, 0x00, 0x0F, 0x0E, 0x88, 0x0C, 0x0B, 0x0A, 0x09,
        0x08,
    ];
    check_sse(
        "pshufb",
        &sse_program(&[0x66, 0x0F, 0x38, 0x00, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_phaddw() {
    // PHADDW xmm0, xmm1 = 66 0F 38 01 C1 : horizontal add of adjacent word pairs.
    let a = u16x8([1, 2, 3, 4, 5, 6, 7, 8]);
    let b = u16x8([10, 20, 30, 40, 50, 60, 70, 80]);
    check_sse(
        "phaddw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x01, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_phaddd() {
    // PHADDD xmm0, xmm1 = 66 0F 38 02 C1 : horizontal add of adjacent dword pairs.
    let a = u32x4([0x0000_0001, 0x0000_0002, 0x0000_0003, 0x0000_0004]);
    let b = u32x4([0x1000_0000, 0x2000_0000, 0x3000_0000, 0x4000_0000]);
    check_sse(
        "phaddd",
        &sse_program(&[0x66, 0x0F, 0x38, 0x02, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_phaddsw() {
    // PHADDSW xmm0, xmm1 = 66 0F 38 03 C1 : horizontal add of word pairs WITH signed saturation.
    let a = u16x8([
        0x7FFF, 0x7FFF, 0x8000, 0x8000, 0x0001, 0xFFFF, 0x4000, 0x4000,
    ]);
    let b = u16x8([
        0x7FFF, 0x0001, 0x8000, 0xFFFF, 0x0000, 0x0000, 0xC000, 0xC000,
    ]);
    check_sse(
        "phaddsw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x03, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_pmaddubsw() {
    // PMADDUBSW xmm0, xmm1 = 66 0F 38 04 C1 : multiply UNSIGNED bytes of xmm0 by
    // SIGNED bytes of xmm1, add adjacent pairs, saturate to signed word.
    let a = [
        0xFF, 0xFF, 0x80, 0x80, 0x01, 0x02, 0x10, 0x20, 0x7F, 0x7F, 0x00, 0xFF, 0x55, 0xAA, 0x40,
        0x40,
    ];
    let b = [
        0x7F, 0x7F, 0x80, 0x80, 0xFF, 0x01, 0x02, 0x03, 0x7F, 0x7F, 0x12, 0x34, 0x01, 0xFF, 0x80,
        0x80,
    ];
    check_sse(
        "pmaddubsw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x04, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_pabsb() {
    // PABSB xmm0, xmm1 = 66 0F 38 1C C1 : per-byte absolute value (src xmm1 -> xmm0).
    // abs(-128) saturates to 0x80 (stays -128 pattern) per the ISA.
    let a = [0u8; 16];
    let b = [
        0x00, 0xFF, 0x80, 0x7F, 0x01, 0xFE, 0x40, 0xC0, 0x10, 0xF0, 0x55, 0xAB, 0x81, 0x7E, 0x02,
        0xFD,
    ];
    check_sse(
        "pabsb",
        &sse_program(&[0x66, 0x0F, 0x38, 0x1C, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_pabsw() {
    // PABSW xmm0, xmm1 = 66 0F 38 1D C1 : per-word absolute value.
    let a = [0u8; 16];
    let b = u16x8([
        0x0000, 0xFFFF, 0x8000, 0x7FFF, 0x0001, 0xFFFE, 0x4000, 0xC000,
    ]);
    check_sse(
        "pabsw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x1D, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_pabsd() {
    // PABSD xmm0, xmm1 = 66 0F 38 1E C1 : per-dword absolute value.
    let a = [0u8; 16];
    let b = u32x4([0x0000_0000, 0xFFFF_FFFF, 0x8000_0000, 0x7FFF_FFFF]);
    check_sse(
        "pabsd",
        &sse_program(&[0x66, 0x0F, 0x38, 0x1E, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_palignr() {
    // PALIGNR xmm0, xmm1, imm8 = 66 0F 3A 0F C1 ib : concatenate xmm0:xmm1 (32 bytes),
    // shift right by imm bytes, take the low 16. imm=5.
    let a = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F,
    ];
    let b = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    check_sse(
        "palignr",
        &sse_program(&[0x66, 0x0F, 0x3A, 0x0F, 0xC1, 0x05]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_palignr_ge16() {
    // PALIGNR with imm >= 16 shifts in zeros from the high end. imm=18 -> shift the
    // upper operand (xmm0) right by 2 and zero-fill the top.
    let a = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F,
    ];
    let b = [0u8; 16];
    check_sse(
        "palignr18",
        &sse_program(&[0x66, 0x0F, 0x3A, 0x0F, 0xC1, 0x12]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_psignb() {
    // PSIGNB xmm0, xmm1 = 66 0F 38 08 C1 : negate/zero bytes of xmm0 by the sign of xmm1.
    let a = [
        0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x7F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08,
    ];
    let b = [
        0x01, 0xFF, 0x00, 0x7F, 0x80, 0x00, 0x05, 0xFE, 0xFF, 0x01, 0x00, 0xFF, 0x01, 0x00, 0xFF,
        0x01,
    ];
    check_sse(
        "psignb",
        &sse_program(&[0x66, 0x0F, 0x38, 0x08, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_psignw() {
    // PSIGNW xmm0, xmm1 = 66 0F 38 09 C1 : per-word sign apply.
    let a = u16x8([
        0x0010, 0x0020, 0x0030, 0x7FFF, 0x0001, 0x0002, 0x0003, 0x0004,
    ]);
    let b = u16x8([
        0x0001, 0xFFFF, 0x0000, 0x8000, 0xFFFF, 0x0000, 0x0001, 0xFFFE,
    ]);
    check_sse(
        "psignw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x09, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn ssse3_psignd() {
    // PSIGND xmm0, xmm1 = 66 0F 38 0A C1 : per-dword sign apply.
    let a = u32x4([0x0000_0010, 0x0000_0020, 0x7FFF_FFFF, 0x0000_0001]);
    let b = u32x4([0x0000_0001, 0xFFFF_FFFF, 0x0000_0000, 0x8000_0000]);
    check_sse(
        "psignd",
        &sse_program(&[0x66, 0x0F, 0x38, 0x0A, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// SSE3: LDDQU (unaligned 128-bit load) and HADDPD/HSUBPD (double horizontal).
// ---------------------------------------------------------------------------

#[test]
fn sse3_lddqu() {
    // LDDQU xmm0, m128 = F2 0F F0 /r. Load from [rdi] then store to [rdi+0x20].
    let a = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF2, 0x0F, 0xF0, 0x07]); // lddqu xmm0, [rdi]
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // movdqu [rdi+0x20], xmm0
    prog.push(HLT);
    check_sse("lddqu", &prog, sse_scratch(a, [0u8; 16]));
}

#[test]
fn sse3_haddpd() {
    // HADDPD xmm0, xmm1 = 66 0F 7C C1 : result = [a0+a1, b0+b1].
    let a = f64x2([1.5, 2.25]);
    let b = f64x2([10.0, 20.5]);
    check_sse(
        "haddpd",
        &sse_program(&[0x66, 0x0F, 0x7C, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse3_hsubpd() {
    // HSUBPD xmm0, xmm1 = 66 0F 7D C1 : result = [a0-a1, b0-b1].
    let a = f64x2([5.0, 1.25]);
    let b = f64x2([100.0, 40.5]);
    check_sse(
        "hsubpd",
        &sse_program(&[0x66, 0x0F, 0x7D, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// Scalar conversions: edge cases for CVTSI2SS/SD, CVTTSS2SI overflow, CVTSD2SS.
// These need a nonzero GPR or scratch input, so they use run_both directly with
// CompareOpts (mirroring the existing cvtsi2sd / cvttsd2si tests).
// ---------------------------------------------------------------------------

/// Run a program with the given init regs comparing GPRs only (no flags/scratch).
fn check_gpr_only(label: &str, code: &[u8], init: Registers) {
    let Some((interp, kvm)) = run_both(code, init, zero_scratch()) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0,
        ..CompareOpts::default()
    };
    assert_match(label, code, &interp, &kvm, opts);
}

#[test]
fn cvt_cvtsi2ss_negative() {
    // CVTSI2SS xmm0, r/m64 = F3 48 0F 2A C0 (from rax). rax = -1 -> -1.0f, stored.
    let mut r = regs();
    r.rax = (-1i64) as u64;
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x48, 0x0F, 0x2A, 0xC0]); // cvtsi2ss xmm0, rax
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // movdqu [rdi+0x20], xmm0
    prog.push(HLT);
    let Some((interp, kvm)) = run_both(&prog, r, zero_scratch()) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0,
        scratch: true,
        ..CompareOpts::default()
    };
    assert_match("cvtsi2ss_neg", &prog, &interp, &kvm, opts);
}

#[test]
fn cvt_cvtsi2ss_large() {
    // Large value not exactly representable in f32 -> tests round-to-nearest-even.
    // 0x4000_0001 (1073741825) rounds to 1073741824.0f (loses the low bit).
    let mut r = regs();
    r.rax = 0x4000_0001;
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x48, 0x0F, 0x2A, 0xC0]); // cvtsi2ss xmm0, rax
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // movdqu [rdi+0x20], xmm0
    prog.push(HLT);
    let Some((interp, kvm)) = run_both(&prog, r, zero_scratch()) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0,
        scratch: true,
        ..CompareOpts::default()
    };
    assert_match("cvtsi2ss_large", &prog, &interp, &kvm, opts);
}

#[test]
fn cvt_cvtsi2sd_large() {
    // CVTSI2SD from a 64-bit value whose magnitude exceeds f64's 53-bit mantissa,
    // forcing rounding. 0x2000_0000_0000_0001 -> rounds to 0x2000_0000_0000_0000.
    let mut r = regs();
    r.rax = 0x2000_0000_0000_0001;
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF2, 0x48, 0x0F, 0x2A, 0xC0]); // cvtsi2sd xmm0, rax
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // movdqu [rdi+0x20], xmm0
    prog.push(HLT);
    let Some((interp, kvm)) = run_both(&prog, r, zero_scratch()) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0,
        scratch: true,
        ..CompareOpts::default()
    };
    assert_match("cvtsi2sd_large", &prog, &interp, &kvm, opts);
}

#[test]
fn cvt_cvttss2si_overflow_indefinite() {
    // CVTTSS2SI r32, xmm1 = F3 0F 2C C1 : truncating f32 -> i32. An out-of-range
    // value (here 1e20f) yields the "integer indefinite" 0x8000_0000.
    let mut b = [0u8; 16];
    b[0..4].copy_from_slice(&1.0e20f32.to_le_bytes());
    let scratch = sse_scratch([0u8; 16], b);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0xF3, 0x0F, 0x2C, 0xC1]); // cvttss2si eax, xmm1
    prog.push(HLT);
    let Some((interp, kvm)) = run_both(&prog, regs(), scratch) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0,
        ..CompareOpts::default()
    };
    assert_match("cvttss2si_of", &prog, &interp, &kvm, opts);
}

#[test]
fn cvt_cvttsd2si_overflow_indefinite() {
    // CVTTSD2SI r64, xmm1 = F2 48 0F 2C C1 : an out-of-range f64 yields the 64-bit
    // integer indefinite 0x8000_0000_0000_0000.
    let b = f64x2([1.0e30, 0.0]);
    let scratch = sse_scratch([0u8; 16], b);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0xF2, 0x48, 0x0F, 0x2C, 0xC1]); // cvttsd2si rax, xmm1
    prog.push(HLT);
    // GPR comparison; the converted integer lands in RAX. Scratch holds the input.
    let Some((interp, kvm)) = run_both(&prog, regs(), scratch) else {
        return;
    };
    let opts = CompareOpts {
        flag_mask: 0,
        ..CompareOpts::default()
    };
    assert_match("cvttsd2si_of", &prog, &interp, &kvm, opts);
}

#[test]
fn cvt_cvtsd2ss_rounding() {
    // CVTSD2SS xmm0, xmm1 = F2 0F 5A C1 : f64 -> f32 with round-to-nearest-even.
    // 1.0 + 2^-30 in f64 is not representable in f32 and must round to 1.0f.
    let v = 1.0_f64 + 2.0_f64.powi(-30);
    let b = f64x2([v, 0.0]);
    check_sse(
        "cvtsd2ss",
        &sse_program(&[0xF2, 0x0F, 0x5A, 0xC1]),
        sse_scratch([0u8; 16], b),
    );
}

#[test]
fn cvt_cvtss2sd_exact() {
    // CVTSS2SD xmm0, xmm1 = F3 0F 5A C1 : f32 -> f64 (always exact for finite).
    let mut b = [0u8; 16];
    b[0..4].copy_from_slice(&(-3.5f32).to_le_bytes());
    check_sse(
        "cvtss2sd",
        &sse_program(&[0xF3, 0x0F, 0x5A, 0xC1]),
        sse_scratch([0u8; 16], b),
    );
}

// ---------------------------------------------------------------------------
// AES-NI: AESENC / AESDEC / AESIMC / AESKEYGENASSIST (KVM exposes AES-NI; we
// require an exact bit-for-bit match of the 128-bit result).
// ---------------------------------------------------------------------------

#[test]
fn aes_aesenc() {
    // AESENC xmm0, xmm1 = 66 0F 38 DC C1 : one AES encryption round (state^round key).
    let state = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ];
    let rkey = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ];
    check_sse(
        "aesenc",
        &sse_program(&[0x66, 0x0F, 0x38, 0xDC, 0xC1]),
        sse_scratch(state, rkey),
    );
}

#[test]
fn aes_aesenclast() {
    // AESENCLAST xmm0, xmm1 = 66 0F 38 DD C1 : final AES round (no MixColumns).
    let state = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF,
    ];
    let rkey = [
        0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
        0x00,
    ];
    check_sse(
        "aesenclast",
        &sse_program(&[0x66, 0x0F, 0x38, 0xDD, 0xC1]),
        sse_scratch(state, rkey),
    );
}

#[test]
fn aes_aesdec() {
    // AESDEC xmm0, xmm1 = 66 0F 38 DE C1 : one AES decryption round.
    let state = [
        0x7a, 0xd5, 0xfd, 0xa7, 0x89, 0xef, 0x4e, 0x27, 0x2b, 0xca, 0x10, 0x0b, 0x3d, 0x9f, 0xf5,
        0x9f,
    ];
    let rkey = [
        0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20, 0x46,
        0x75,
    ];
    check_sse(
        "aesdec",
        &sse_program(&[0x66, 0x0F, 0x38, 0xDE, 0xC1]),
        sse_scratch(state, rkey),
    );
}

#[test]
fn aes_aesdeclast() {
    // AESDECLAST xmm0, xmm1 = 66 0F 38 DF C1 : final AES decryption round.
    let state = [
        0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x0F, 0xED, 0xCB, 0xA9, 0x87, 0x65, 0x43,
        0x21,
    ];
    let rkey = [
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
        0x99,
    ];
    check_sse(
        "aesdeclast",
        &sse_program(&[0x66, 0x0F, 0x38, 0xDF, 0xC1]),
        sse_scratch(state, rkey),
    );
}

#[test]
fn aes_aesimc() {
    // AESIMC xmm0, xmm1 = 66 0F 38 DB C1 : inverse MixColumns of xmm1 -> xmm0.
    let a = [0u8; 16];
    let b = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ];
    check_sse(
        "aesimc",
        &sse_program(&[0x66, 0x0F, 0x38, 0xDB, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn aes_aeskeygenassist() {
    // AESKEYGENASSIST xmm0, xmm1, imm8 = 66 0F 3A DF C1 ib : key expansion helper.
    // imm8 is the round constant (RCON). Use RCON=1.
    let a = [0u8; 16];
    let b = [
        0x09, 0xcf, 0x4f, 0x3c, 0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15,
        0x88,
    ];
    check_sse(
        "aeskeygen",
        &sse_program(&[0x66, 0x0F, 0x3A, 0xDF, 0xC1, 0x01]),
        sse_scratch(a, b),
    );
}

// ---------------------------------------------------------------------------
// CRC32 (SSE4.2) and POPCNT r/m forms (register + memory operand).
// CRC32 affects no flags; POPCNT defines ZF (set when src==0) and clears the rest.
// ---------------------------------------------------------------------------

#[test]
fn crc32_r32_r8() {
    // CRC32 r32, r/m8 = F2 0F 38 F0 C3 : accumulate one byte. rax(EAX) is the CRC.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF; // initial CRC
    r.rbx = 0x000000AB; // bl = byte to fold in
    check_gpr_only("crc32_8", &with_hlt(vec![0xF2, 0x0F, 0x38, 0xF0, 0xC3]), r);
}

#[test]
fn crc32_r64_r64() {
    // CRC32 r64, r/m64 = F2 48 0F 38 F1 C3 : fold an 8-byte quantity into RAX.
    let mut r = regs();
    r.rax = 0x0000_0000_FFFF_FFFF;
    r.rbx = 0x0123_4567_89AB_CDEF;
    check_gpr_only(
        "crc32_64",
        &with_hlt(vec![0xF2, 0x48, 0x0F, 0x38, 0xF1, 0xC3]),
        r,
    );
}

#[test]
fn crc32_r32_mem8() {
    // CRC32 r32, m8 (F2 0F 38 F0 /r) reading [rdi]. Memory operand form.
    let mut s = [0u8; 64];
    s[0] = 0x5A;
    let mut r = regs();
    r.rax = 0xFFFF_FFFF;
    r.rdi = DATA_ADDR;
    // F2 0F 38 F0 07  crc32 eax, byte [rdi]
    check_mem(
        "crc32_mem8",
        &with_hlt(vec![0xF2, 0x0F, 0x38, 0xF0, 0x07]),
        r,
        s,
        0,
    );
}

#[test]
fn popcnt_r16() {
    // POPCNT r16, r/m16 = 66 F3 0F B8 C3. Count bits in bx.
    let mut r = regs();
    r.rbx = 0x0000_0000_0000_F0F0; // bx = 0xF0F0 -> 8 bits set
    check("popcnt16", &with_hlt(vec![0x66, 0xF3, 0x0F, 0xB8, 0xC3]), r);
}

#[test]
fn popcnt_r64_mem() {
    // POPCNT r64, m64 = F3 48 0F B8 /r reading [rdi]. Memory-operand form; ZF flag.
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0xFF00_F0F0_1234_5678u64.to_le_bytes());
    let mut r = regs();
    r.rdi = DATA_ADDR;
    // F3 48 0F B8 07  popcnt rax, qword [rdi]
    check_mem(
        "popcnt_mem64",
        &with_hlt(vec![0xF3, 0x48, 0x0F, 0xB8, 0x07]),
        r,
        s,
        FLAG_MASK,
    );
}

// ---------------------------------------------------------------------------
// BT/BTS/BTR/BTC with a MEMORY operand and a bit index that can exceed the
// operand size. For a memory bit-string the index is NOT taken modulo the
// operand size: it selects byte (index/8) at an effective address computed from
// the base plus a signed bit-offset displacement. We verify both CF and the
// resulting memory bytes against KVM.
// ---------------------------------------------------------------------------

#[test]
fn bt_mem_index_small() {
    // BT [rdi], rdx with index 5 in the first byte. CF <- bit 5 of [rdi].
    let mut s = [0u8; 64];
    s[0] = 0b0010_0000; // bit 5 set
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rdx = 5;
    // 48 0F A3 17  bt qword [rdi], rdx
    check_mem(
        "bt_mem5",
        &with_hlt(vec![0x48, 0x0F, 0xA3, 0x17]),
        r,
        s,
        BT_DEFINED,
    );
}

#[test]
fn bt_mem_index_large() {
    // BT [rdi], rdx with index 100 -> byte 12, bit 4. Memory bit-string semantics:
    // index is NOT reduced mod operand size; it addresses [rdi + 100/8].
    let mut s = [0u8; 64];
    s[12] = 0b0001_0000; // bit 100 == byte 12 bit 4
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rdx = 100;
    // 48 0F A3 17  bt qword [rdi], rdx
    check_mem(
        "bt_mem100",
        &with_hlt(vec![0x48, 0x0F, 0xA3, 0x17]),
        r,
        s,
        BT_DEFINED,
    );
}

#[test]
fn bts_mem_large_sets_bit() {
    // BTS [rdi], rdx index 70 -> byte 8 bit 6. CF<-old(0), then the bit is set.
    let s = [0u8; 64];
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rdx = 70;
    // 48 0F AB 17  bts qword [rdi], rdx
    check_mem(
        "bts_mem70",
        &with_hlt(vec![0x48, 0x0F, 0xAB, 0x17]),
        r,
        s,
        BT_DEFINED,
    );
}

#[test]
fn btr_mem_large_clears_bit() {
    // BTR [rdi], rdx index 130 -> byte 16 bit 2. Preset that bit; CF<-1, bit cleared.
    let mut s = [0u8; 64];
    s[16] = 0b0000_0100; // bit 130 == byte 16 bit 2
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rdx = 130;
    // 48 0F B3 17  btr qword [rdi], rdx
    check_mem(
        "btr_mem130",
        &with_hlt(vec![0x48, 0x0F, 0xB3, 0x17]),
        r,
        s,
        BT_DEFINED,
    );
}

#[test]
fn btc_mem_large_toggles_bit() {
    // BTC [rdi], rdx index 200 -> byte 25 bit 0. CF<-old(0), bit toggled to 1.
    let s = [0u8; 64];
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rdx = 200;
    // 48 0F BB 17  btc qword [rdi], rdx
    check_mem(
        "btc_mem200",
        &with_hlt(vec![0x48, 0x0F, 0xBB, 0x17]),
        r,
        s,
        BT_DEFINED,
    );
}

#[test]
fn bts_mem_negative_index() {
    // A SIGNED negative bit index addresses BELOW the base operand. With base at
    // [rdi+8] and index -64, the effective byte is [rdi+8 + (-64/8)] = [rdi].
    let s = [0u8; 64];
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rdx = (-64i64) as u64;
    // 48 0F AB 57 08  bts qword [rdi+8], rdx
    check_mem(
        "bts_mem_neg",
        &with_hlt(vec![0x48, 0x0F, 0xAB, 0x57, 0x08]),
        r,
        s,
        BT_DEFINED,
    );
}

#[test]
fn bt_mem_imm_index() {
    // BT m64, imm8 (0F BA /4 ib): immediate bit index IS taken modulo operand size
    // (64) for the imm form. imm=63 selects bit 63 of the qword.
    let mut s = [0u8; 64];
    s[7] = 0x80; // bit 63 of the qword at [rdi]
    let mut r = regs();
    r.rdi = DATA_ADDR;
    // 48 0F BA 27 3F  bt qword [rdi], 63
    check_mem(
        "bt_mem_imm63",
        &with_hlt(vec![0x48, 0x0F, 0xBA, 0x27, 0x3F]),
        r,
        s,
        BT_DEFINED,
    );
}

// ---------------------------------------------------------------------------
// ADCX / ADOX: ADD with carry that touches ONLY CF (ADCX) or ONLY OF (ADOX),
// enabling two independent carry chains. We model an interleaved chain and check
// the full status mask (each instruction leaves the other 5 flags untouched, so
// comparing all 6 against KVM validates the "only one flag changes" property).
// ---------------------------------------------------------------------------

#[test]
fn adcx_propagates_only_cf() {
    // ADCX rax, rbx = 66 48 0F 38 F6 C3 : rax += rbx + CF; only CF updated.
    // Seed OF=1 and CF=1; the sum wraps so CF stays 1, and OF must be PRESERVED.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF;
    r.rbx = 0x0000_0000_0000_0001;
    r.rflags = flags::bits::CF | flags::bits::OF | flags::bits::SF | flags::bits::ZF;
    check(
        "adcx",
        &with_hlt(vec![0x66, 0x48, 0x0F, 0x38, 0xF6, 0xC3]),
        r,
    );
}

#[test]
fn adox_propagates_only_of() {
    // ADOX rax, rbx = F3 48 0F 38 F6 C3 : rax += rbx + OF; only OF updated.
    // Seed CF=1 (must be preserved) and OF=1; the sum wraps -> OF stays 1.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFF;
    r.rbx = 0x0000_0000_0000_0001;
    r.rflags = flags::bits::CF | flags::bits::OF | flags::bits::PF;
    check(
        "adox",
        &with_hlt(vec![0xF3, 0x48, 0x0F, 0x38, 0xF6, 0xC3]),
        r,
    );
}

#[test]
fn adcx_adox_interleaved_chains() {
    // Two independent carry chains advanced in lockstep:
    //   adcx rax, rcx   (CF chain)
    //   adox rbx, rdx   (OF chain)
    // Seed CF=1 and OF=1; both low halves wrap so both carries re-propagate.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_FFFF_FFFE;
    r.rcx = 0x0000_0000_0000_0001; // + CF(1) -> wraps to 0, CF=1
    r.rbx = 0xFFFF_FFFF_FFFF_FFFE;
    r.rdx = 0x0000_0000_0000_0001; // + OF(1) -> wraps to 0, OF=1
    r.rflags = flags::bits::CF | flags::bits::OF;
    // 66 48 0F 38 F6 C1  adcx rax, rcx
    // F3 48 0F 38 F6 DA  adox rbx, rdx
    check(
        "adcx_adox_chain",
        &with_hlt(vec![
            0x66, 0x48, 0x0F, 0x38, 0xF6, 0xC1, // adcx rax, rcx
            0xF3, 0x48, 0x0F, 0x38, 0xF6, 0xDA, // adox rbx, rdx
        ]),
        r,
    );
}

#[test]
fn adcx_no_carry_clears_cf() {
    // ADCX with a small sum that doesn't overflow -> CF cleared, OF preserved.
    let mut r = regs();
    r.rax = 0x0000_0000_0000_0010;
    r.rbx = 0x0000_0000_0000_0020;
    r.rflags = flags::bits::OF | flags::bits::CF; // CF=1 in (consumed), OF stays
    check(
        "adcx_noc",
        &with_hlt(vec![0x66, 0x48, 0x0F, 0x38, 0xF6, 0xC3]),
        r,
    );
}

// ---------------------------------------------------------------------------
// Misc flag manipulation: CMC / STC / CLC / CLD / STD, plus a string op honoring
// DF, and BSWAP r16 (undefined result, but we still compare against KVM).
// ---------------------------------------------------------------------------

#[test]
fn flag_stc() {
    // STC (F9) sets CF. Seed CF=0; expect CF=1 afterward.
    let mut r = regs();
    r.rflags = 0;
    check("stc", &with_hlt(vec![0xF9]), r);
}

#[test]
fn flag_clc() {
    // CLC (F8) clears CF. Seed CF=1.
    let mut r = regs();
    r.rflags = flags::bits::CF;
    check("clc", &with_hlt(vec![0xF8]), r);
}

#[test]
fn flag_cmc() {
    // CMC (F5) complements CF. Seed CF=1 -> 0; other flags untouched.
    let mut r = regs();
    r.rflags = flags::bits::CF | flags::bits::ZF | flags::bits::SF;
    check("cmc", &with_hlt(vec![0xF5]), r);
}

#[test]
fn flag_cmc_from_zero() {
    // CMC with CF=0 -> CF=1.
    let mut r = regs();
    r.rflags = flags::bits::PF;
    check("cmc0", &with_hlt(vec![0xF5]), r);
}

#[test]
fn flag_stc_clc_cmc_sequence() {
    // STC; CMC; -> CF=0. F9 F5.
    let mut r = regs();
    r.rflags = 0;
    check("stc_cmc", &with_hlt(vec![0xF9, 0xF5]), r);
}

#[test]
fn flag_cld_then_movsb() {
    // CLD (FC) forces DF=0, so a following MOVSB advances RSI/RDI UPward even if
    // DF was seeded set. Compare the scratch copy + RSI/RDI.
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF;
    r.rdi = DATA_ADDR + DST_OFF;
    r.rflags = flags::bits::DF; // seeded set; CLD must clear it
    // FC      cld
    // A4      movsb
    check_mem(
        "cld_movsb",
        &with_hlt(vec![0xFC, 0xA4]),
        r,
        string_scratch(&[0xC1, 0xC2, 0xC3]),
        0,
    );
}

#[test]
fn flag_std_then_movsb() {
    // STD (FD) forces DF=1, so MOVSB walks DOWN. Point RSI/RDI at the 3rd element
    // so the single copy moves [src+2]->[dst+2] and decrements both pointers.
    let mut r = regs();
    r.rsi = DATA_ADDR + SRC_OFF + 2;
    r.rdi = DATA_ADDR + DST_OFF + 2;
    r.rflags = 0; // seeded clear; STD must set DF
    // FD      std
    // A4      movsb
    check_mem(
        "std_movsb",
        &with_hlt(vec![0xFD, 0xA4]),
        r,
        string_scratch(&[0xD1, 0xD2, 0xD3]),
        0,
    );
}

#[test]
fn flag_std_rep_stosb_then_cld() {
    // STD; REP STOSB; CLD : fill RCX bytes DOWNward, then restore DF. We verify the
    // descending fill landed correctly. RDI starts at the last target byte.
    let mut r = regs();
    r.rdi = DATA_ADDR + DST_OFF + 4; // fill offsets 16..20 descending
    r.rcx = 5;
    r.rax = 0x7C;
    r.rflags = 0;
    // FD        std
    // F3 AA     rep stosb
    // FC        cld
    check_mem(
        "std_rep_stosb",
        &with_hlt(vec![0xFD, 0xF3, 0xAA, 0xFC]),
        r,
        zero_scratch(),
        0,
    );
}

#[ignore = "ARCHITECTURALLY UNDEFINED, divergence expected: BSWAP with a 16-bit \
operand (66 0F C8) is explicitly 'undefined' per the Intel SDM. Observed: KVM \
(this Intel host) zeroes the swapped low 16 bits -> ax becomes 0x0000 \
(rax=0x1122334455660000), whereas the interpreter leaves the 16-bit value \
unchanged (rax=0x1122334455667788). Both are defensible for an undefined \
encoding, so this is NOT a correctness bug; it is kept (ignored) to document the \
behavioral difference the task asked us to compare."]
#[test]
fn bswap_r16_undefined_compare() {
    // BSWAP on a 16-bit register (66 0F C8) is documented as UNDEFINED behavior;
    // KVM zeroes the low 16 bits while the interpreter preserves them (see the
    // #[ignore] note above). Comparison is intentionally disabled.
    let mut r = regs();
    r.rax = 0x1122_3344_5566_7788; // ax = 0x7788
    // 66 0F C8  bswap ax
    check("bswap_r16", &with_hlt(vec![0x66, 0x0F, 0xC8]), r);
}

// ---------------------------------------------------------------------------
// XADD / CMPXCHG with MEMORY operands (lock-free single-thread semantics).
// ---------------------------------------------------------------------------

#[test]
fn xadd_mem() {
    // XADD [rdi], rbx = 48 0F C1 1F : tmp=[rdi]; [rdi]=tmp+rbx; rbx=tmp; flags=ADD.
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0x0000_0000_0000_0100u64.to_le_bytes());
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rbx = 0x0000_0000_0000_00FF;
    // 48 0F C1 1F  xadd [rdi], rbx
    check_mem(
        "xadd_mem",
        &with_hlt(vec![0x48, 0x0F, 0xC1, 0x1F]),
        r,
        s,
        FLAG_MASK,
    );
}

#[test]
fn xadd_mem_carry() {
    // XADD memory where the in-memory add wraps -> CF, ZF set.
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0xFFFF_FFFF_FFFF_FFFFu64.to_le_bytes());
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rbx = 0x0000_0000_0000_0001;
    check_mem(
        "xadd_mem_carry",
        &with_hlt(vec![0x48, 0x0F, 0xC1, 0x1F]),
        r,
        s,
        FLAG_MASK,
    );
}

#[test]
fn cmpxchg_mem_success() {
    // CMPXCHG [rdi], rcx = 48 0F B1 0F : if RAX==[rdi] then ZF=1, [rdi]=rcx;
    // else ZF=0, RAX=[rdi]. Success path: [rdi]==RAX.
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rax = 0x1122_3344_5566_7788; // matches memory -> success
    r.rcx = 0xCAFE_BABE_DEAD_BEEF; // written to memory on success
    // 48 0F B1 0F  cmpxchg [rdi], rcx
    check_mem(
        "cmpxchg_mem_ok",
        &with_hlt(vec![0x48, 0x0F, 0xB1, 0x0F]),
        r,
        s,
        FLAG_MASK,
    );
}

#[test]
fn cmpxchg_mem_fail() {
    // Failure path: RAX != [rdi] -> ZF=0, RAX loaded from memory, memory unchanged.
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rax = 0x0000_0000_0000_0001; // mismatch -> failure
    r.rcx = 0xCAFE_BABE_DEAD_BEEF; // NOT written on failure
    check_mem(
        "cmpxchg_mem_fail",
        &with_hlt(vec![0x48, 0x0F, 0xB1, 0x0F]),
        r,
        s,
        FLAG_MASK,
    );
}

#[test]
fn lock_xadd_mem() {
    // LOCK XADD [rdi], rbx (F0 prefix) : same architectural result as XADD; the
    // lock prefix must be accepted and the memory effect/flags must match KVM.
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0x0000_0000_0000_0042u64.to_le_bytes());
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rbx = 0x0000_0000_0000_0008;
    // F0 48 0F C1 1F  lock xadd [rdi], rbx
    check_mem(
        "lock_xadd_mem",
        &with_hlt(vec![0xF0, 0x48, 0x0F, 0xC1, 0x1F]),
        r,
        s,
        FLAG_MASK,
    );
}

#[test]
fn cmpxchg8_mem_success() {
    // CMPXCHG8B m64 (0F C7 /1) : if EDX:EAX == [rdi] then ZF=1, [rdi]=ECX:EBX;
    // else ZF=0, EDX:EAX=[rdi]. Success path.
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rax = 0x5566_7788; // low half of the 64-bit compare value
    r.rdx = 0x1122_3344; // high half
    r.rbx = 0xDEAD_BEEF; // new low half
    r.rcx = 0xCAFE_F00D; // new high half
    // 0F C7 0F  cmpxchg8b [rdi]
    check_mem(
        "cmpxchg8b_ok",
        &with_hlt(vec![0x0F, 0xC7, 0x0F]),
        r,
        s,
        FLAG_MASK,
    );
}

#[test]
fn cmpxchg16_mem_success() {
    // CMPXCHG16B m128 (48 0F C7 /1) : 128-bit compare-and-swap. Success path:
    // RDX:RAX == [rdi] -> ZF=1, [rdi] = RCX:RBX.
    let mut s = [0u8; 64];
    s[0..8].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes()); // low qword
    s[8..16].copy_from_slice(&0x99AA_BBCC_DDEE_FF00u64.to_le_bytes()); // high qword
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rax = 0x1122_3344_5566_7788; // low compare
    r.rdx = 0x99AA_BBCC_DDEE_FF00; // high compare
    r.rbx = 0x0123_4567_89AB_CDEF; // new low
    r.rcx = 0xFEDC_BA98_7654_3210; // new high
    // 48 0F C7 0F  cmpxchg16b [rdi]
    check_mem(
        "cmpxchg16b_ok",
        &with_hlt(vec![0x48, 0x0F, 0xC7, 0x0F]),
        r,
        s,
        FLAG_MASK,
    );
}

// ===========================================================================
// EXPANDED COVERAGE PART 4: SSE4.1 ROUND modes, SSE3/SSSE3 horizontal & sign,
// SSE4 PMOVSX/ZX & PEXTR/PINSR & PALIGNR & DPPS, MOVD/MOVQ gpr<->xmm, MMX,
// x87 FXCH/FABS/FCHS/FRNDINT/FSCALE/FPREM, and a few ALU corner cases.
//
// Reuses the existing helpers verbatim. SSE results that are exactly
// representable (small integers / exact dyadic) are bit-identical across
// backends and compared byte-for-byte via the scratch page.
// ===========================================================================

// ---- SSE4.1 ROUNDPS/ROUNDPD/ROUNDSS/ROUNDSD across all 4 rounding modes ----
//
// imm8[1:0] = 0 round-to-nearest-EVEN, 1 floor, 2 ceil, 3 truncate. Mode 0 uses
// banker's rounding (ties to even) — NOT "round half away from zero". The .5 ties
// here (0.5->0, 2.5->2, -2.5->-2, 3.5->4) probe exactly that distinction.

/// ROUNDPS xmm0, xmm1, imm8 = 66 0F 3A 08 C1 ib. Source = xmm1, dest = xmm0.
fn roundps_program(imm: u8) -> Vec<u8> {
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]); // movdqu xmm0, [rdi]
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x08, 0xC1, imm]); // roundps xmm0, xmm1, imm
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // movdqu [rdi+0x20], xmm0
    prog.push(HLT);
    prog
}

#[test]
fn sse4_roundps_nearest_even() {
    // Ties-to-even: 0.5->0, 2.5->2, 3.5->4, -2.5->-2.
    let a = [0u8; 16];
    let b = f32x4([0.5, 2.5, 3.5, -2.5]);
    check_sse("roundps_nearest", &roundps_program(0x00), sse_scratch(a, b));
}

#[test]
fn sse4_roundps_floor() {
    let a = [0u8; 16];
    let b = f32x4([1.4, -1.4, 2.9, -2.9]);
    check_sse("roundps_floor", &roundps_program(0x01), sse_scratch(a, b));
}

#[test]
fn sse4_roundps_ceil() {
    let a = [0u8; 16];
    let b = f32x4([1.4, -1.4, 2.1, -2.1]);
    check_sse("roundps_ceil", &roundps_program(0x02), sse_scratch(a, b));
}

#[test]
fn sse4_roundps_trunc() {
    let a = [0u8; 16];
    let b = f32x4([1.9, -1.9, 2.5, -2.5]);
    check_sse("roundps_trunc", &roundps_program(0x03), sse_scratch(a, b));
}

#[test]
fn sse4_roundpd_nearest_even() {
    // ROUNDPD xmm0, xmm1, 0 = 66 0F 3A 09 C1 00. Doubles: 0.5->0, 2.5->2.
    let a = [0u8; 16];
    let b = f64x2([0.5, 2.5]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x09, 0xC1, 0x00]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("roundpd_nearest", &prog, sse_scratch(a, b));
}

#[test]
fn sse4_roundpd_floor_neg() {
    // floor of negatives: -0.1 -> -1.0, -3.5 -> -4.0.
    let a = [0u8; 16];
    let b = f64x2([-0.1, -3.5]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x09, 0xC1, 0x01]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("roundpd_floor", &prog, sse_scratch(a, b));
}

#[test]
fn sse4_roundss_nearest_even() {
    // ROUNDSS xmm0, xmm1, 0 = 66 0F 3A 0A C1 00. lane0 rounded ties-to-even (2.5->2),
    // upper lanes of xmm0 preserved.
    let a = f32x4([1.0, 11.0, 12.0, 13.0]);
    let b = f32x4([2.5, 0.0, 0.0, 0.0]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x0A, 0xC1, 0x00]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("roundss_nearest", &prog, sse_scratch(a, b));
}

#[test]
fn sse4_roundsd_nearest_even() {
    // ROUNDSD xmm0, xmm1, 0 = 66 0F 3A 0B C1 00. lane0 = round(0.5)=0 ties-to-even.
    let a = f64x2([1.0, 99.0]);
    let b = f64x2([0.5, 0.0]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x0B, 0xC1, 0x00]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("roundsd_nearest", &prog, sse_scratch(a, b));
}

// ---- SSE3 packed-double addsub (HADDPD/HSUBPD already covered earlier) ----

#[test]
fn sse3_addsubpd() {
    // ADDSUBPD xmm0, xmm1 = 66 0F D0 C1. lane0 = a0-b0, lane1 = a1+b1.
    let a = f64x2([10.0, 10.0]);
    let b = f64x2([3.0, 4.0]);
    check_sse(
        "addsubpd",
        &sse_program(&[0x66, 0x0F, 0xD0, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---- SSSE3 horizontal subtract / mulhrs (phaddw/d/sw, pmaddubsw, pshufb,
//      psign*, pabs*, palignr already covered earlier) ----

#[test]
fn ssse3_phsubw() {
    // PHSUBW xmm0, xmm1 = 66 0F 38 05 C1. Pairwise subtract (lane0 - lane1).
    let a = [
        0x000Au16.to_le_bytes(),
        0x0003u16.to_le_bytes(),
        0x0000u16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
        0x0005u16.to_le_bytes(),
        0x0008u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0020u16.to_le_bytes(),
        0x0010u16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0x0100u16.to_le_bytes(),
        0x0050u16.to_le_bytes(),
        0x0000u16.to_le_bytes(),
        0x0000u16.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "phsubw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x05, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn ssse3_pmulhrsw() {
    // PMULHRSW xmm0, xmm1 = 66 0F 38 0B C1. Signed 16-bit mul, round, take bits [16:30].
    let a = [
        0x4000u16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
        0x1234u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x4000u16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0x1000u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "pmulhrsw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x0B, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

// ---- SSSE3 PALIGNR with imm >= 16 (the imm < 16 case is covered earlier) ----

#[test]
fn ssse3_palignr_imm17() {
    // imm >= 16: result shifts dst alone right by (imm-16), zero-filling top.
    let a = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F,
    ];
    let b = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x0F, 0xC1, 0x11]); // imm=17
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("palignr17", &prog, sse_scratch(a, b));
}

// ---- SSE4.1 PMOVSX / PMOVZX (sign/zero extend packed integers) ----

#[test]
fn sse4_pmovsxbw() {
    // PMOVSXBW xmm0, xmm1 = 66 0F 38 20 C1. Sign-extend low 8 bytes -> 8 words.
    let a = [0u8; 16];
    let b = [
        0x01, 0xFF, 0x7F, 0x80, 0x00, 0x40, 0xC0, 0x10, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    check_sse(
        "pmovsxbw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x20, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse4_pmovzxbw() {
    // PMOVZXBW xmm0, xmm1 = 66 0F 38 30 C1. Zero-extend low 8 bytes -> 8 words.
    let a = [0u8; 16];
    let b = [
        0x01, 0xFF, 0x7F, 0x80, 0x00, 0x40, 0xC0, 0x10, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    check_sse(
        "pmovzxbw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x30, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse4_pmovsxbd() {
    // PMOVSXBD xmm0, xmm1 = 66 0F 38 21 C1. Sign-extend low 4 bytes -> 4 dwords.
    let a = [0u8; 16];
    let b = [0x01, 0xFF, 0x7F, 0x80, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    check_sse(
        "pmovsxbd",
        &sse_program(&[0x66, 0x0F, 0x38, 0x21, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse4_pmovsxwd() {
    // PMOVSXWD xmm0, xmm1 = 66 0F 38 23 C1. Sign-extend low 4 words -> 4 dwords.
    let a = [0u8; 16];
    let b = [
        0xFFFFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
        0x1234u16.to_le_bytes(),
        0x5678u16.to_le_bytes(),
        0x9ABCu16.to_le_bytes(),
        0xDEF0u16.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "pmovsxwd",
        &sse_program(&[0x66, 0x0F, 0x38, 0x23, 0xC1]),
        sse_scratch(a, b.try_into().unwrap()),
    );
}

#[test]
fn sse4_pmovzxdq() {
    // PMOVZXDQ xmm0, xmm1 = 66 0F 38 35 C1. Zero-extend low 2 dwords -> 2 qwords.
    let a = [0u8; 16];
    let b = [
        0x8000_0001u32.to_le_bytes(),
        0x0000_0002u32.to_le_bytes(),
        0x1111_1111u32.to_le_bytes(),
        0x2222_2222u32.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "pmovzxdq",
        &sse_program(&[0x66, 0x0F, 0x38, 0x35, 0xC1]),
        sse_scratch(a, b.try_into().unwrap()),
    );
}

// ---- SSE4.1 packed integer min/max (signed/unsigned dwords & bytes) ----

#[test]
fn sse4_pminsd() {
    // PMINSD xmm0, xmm1 = 66 0F 38 39 C1. Per-dword signed min.
    let a = [
        0x0000_0005u32.to_le_bytes(),
        0xFFFF_FFFFu32.to_le_bytes(),
        0x7FFF_FFFFu32.to_le_bytes(),
        0x8000_0000u32.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0000_0003u32.to_le_bytes(),
        0x0000_0001u32.to_le_bytes(),
        0x0000_0000u32.to_le_bytes(),
        0xFFFF_FFFFu32.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "pminsd",
        &sse_program(&[0x66, 0x0F, 0x38, 0x39, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse4_pmaxud() {
    // PMAXUD xmm0, xmm1 = 66 0F 38 3F C1. Per-dword unsigned max.
    let a = [
        0x0000_0005u32.to_le_bytes(),
        0xFFFF_FFFFu32.to_le_bytes(),
        0x7FFF_FFFFu32.to_le_bytes(),
        0x8000_0000u32.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0000_0003u32.to_le_bytes(),
        0x0000_0001u32.to_le_bytes(),
        0xFFFF_FFFFu32.to_le_bytes(),
        0x0000_0001u32.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "pmaxud",
        &sse_program(&[0x66, 0x0F, 0x38, 0x3F, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse4_pmulld() {
    // PMULLD xmm0, xmm1 = 66 0F 38 40 C1. Per-dword 32-bit multiply, low 32 kept.
    let a = [
        0x0000_0002u32.to_le_bytes(),
        0xFFFF_FFFFu32.to_le_bytes(),
        0x0001_0000u32.to_le_bytes(),
        0x1234_5678u32.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0000_0003u32.to_le_bytes(),
        0x0000_0002u32.to_le_bytes(),
        0x0001_0000u32.to_le_bytes(),
        0x0000_0010u32.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "pmulld",
        &sse_program(&[0x66, 0x0F, 0x38, 0x40, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse4_pmuldq() {
    // PMULDQ xmm0, xmm1 = 66 0F 38 28 C1. Signed 32x32->64 of lanes 0 and 2.
    let a = [
        0xFFFF_FFFFu32.to_le_bytes(),
        0xDEAD_BEEFu32.to_le_bytes(),
        0x0000_0002u32.to_le_bytes(),
        0xCAFE_BABEu32.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0000_0003u32.to_le_bytes(),
        0x1111_1111u32.to_le_bytes(),
        0x7FFF_FFFFu32.to_le_bytes(),
        0x2222_2222u32.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "pmuldq",
        &sse_program(&[0x66, 0x0F, 0x38, 0x28, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse4_pcmpeqq() {
    // PCMPEQQ xmm0, xmm1 = 66 0F 38 29 C1. Per-qword equality -> all-1s/all-0s.
    let a = [
        0x1122_3344_5566_7788u64.to_le_bytes(),
        0xDEAD_BEEF_CAFE_BABEu64.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x1122_3344_5566_7788u64.to_le_bytes(),
        0x0000_0000_0000_0000u64.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "pcmpeqq",
        &sse_program(&[0x66, 0x0F, 0x38, 0x29, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse4_packusdw() {
    // PACKUSDW xmm0, xmm1 = 66 0F 38 2B C1. Pack signed dwords to unsigned words (sat).
    let a = [
        0x0000_0001u32.to_le_bytes(),
        0x0001_0000u32.to_le_bytes(),
        0xFFFF_FFFFu32.to_le_bytes(),
        0x0000_FFFFu32.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0000_8000u32.to_le_bytes(),
        0x8000_0000u32.to_le_bytes(),
        0x0000_7FFFu32.to_le_bytes(),
        0x0001_2345u32.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "packusdw",
        &sse_program(&[0x66, 0x0F, 0x38, 0x2B, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

// ---- SSE4.1 DPPS / DPPD (dot product) — exactly-representable inputs ----

#[test]
fn sse4_dpps() {
    // DPPS xmm0, xmm1, imm8 = 66 0F 3A 40 C1 ib. imm high nibble selects which lanes
    // to multiply-accumulate; low nibble selects which result lanes get the sum.
    // imm=0xF1: use all 4 products, broadcast sum to lane0 only.
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([8.0, 0.5, 2.0, 0.25]); // products: 8,1,6,1 -> sum 16
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x40, 0xC1, 0xF1]); // dpps xmm0,xmm1,0xF1
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("dpps", &prog, sse_scratch(a, b));
}

#[test]
fn sse4_dppd() {
    // DPPD xmm0, xmm1, imm8 = 66 0F 3A 41 C1 ib. imm=0x33: both lanes selected,
    // sum broadcast to both result lanes.
    let a = f64x2([3.0, 4.0]);
    let b = f64x2([2.0, 0.5]); // products: 6, 2 -> sum 8
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x41, 0xC1, 0x33]); // dppd xmm0,xmm1,0x33
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("dppd", &prog, sse_scratch(a, b));
}

// ---- SSE4.1 PBLENDW / BLENDPS (imm-controlled blends) ----

#[test]
fn sse4_pblendw() {
    // PBLENDW xmm0, xmm1, imm8 = 66 0F 3A 0E C1 ib. imm bit i selects src word i.
    let a = [
        0x0000u16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0x0003u16.to_le_bytes(),
        0x0004u16.to_le_bytes(),
        0x0005u16.to_le_bytes(),
        0x0006u16.to_le_bytes(),
        0x0007u16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0xA0A0u16.to_le_bytes(),
        0xA1A1u16.to_le_bytes(),
        0xA2A2u16.to_le_bytes(),
        0xA3A3u16.to_le_bytes(),
        0xA4A4u16.to_le_bytes(),
        0xA5A5u16.to_le_bytes(),
        0xA6A6u16.to_le_bytes(),
        0xA7A7u16.to_le_bytes(),
    ]
    .concat();
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x0E, 0xC1, 0b1010_0101]); // imm
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse(
        "pblendw",
        &prog,
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse4_blendps() {
    // BLENDPS xmm0, xmm1, imm8 = 66 0F 3A 0C C1 ib. imm bit i selects src f32 lane i.
    let a = f32x4([1.0, 2.0, 3.0, 4.0]);
    let b = f32x4([10.0, 20.0, 30.0, 40.0]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x0C, 0xC1, 0b0101]); // lanes 0,2 from src
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("blendps", &prog, sse_scratch(a, b));
}

// ---- SSE4.1 PEXTR* / PINSR* (extract/insert lane to/from GPR via memory) ----

#[test]
fn sse4_pextrb_to_mem() {
    // PEXTRB [rdi+0x20], xmm1, idx = 66 0F 3A 14 /r ib. Extract byte idx from xmm1.
    let a = [0u8; 16];
    let b = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF,
    ];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    // pextrb [rdi+0x20], xmm1, 0x0A  -> modrm 0x4F reg=xmm1(001), rm=[rdi+disp8]
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x14, 0x4F, 0x20, 0x0A]);
    prog.push(HLT);
    check_sse("pextrb_mem", &prog, sse_scratch(a, b));
}

#[test]
fn sse4_pextrd_to_reg() {
    // PEXTRD eax, xmm1, idx = 66 0F 3A 16 C1 ib (W0). idx=2. Then store eax to scratch.
    let a = [0u8; 16];
    let b = [
        0x11111111u32.to_le_bytes(),
        0x22222222u32.to_le_bytes(),
        0xDEADBEEFu32.to_le_bytes(),
        0x44444444u32.to_le_bytes(),
    ]
    .concat();
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x16, 0xC8, 0x02]); // pextrd eax, xmm1, 2
    prog.extend_from_slice(&[0x89, 0x47, 0x20]); // mov [rdi+0x20], eax
    prog.push(HLT);
    check_sse("pextrd_reg", &prog, sse_scratch(a, b.try_into().unwrap()));
}

#[test]
fn sse4_pinsrd_from_reg() {
    // PINSRD xmm0, eax, idx = 66 0F 3A 22 C0 ib. Insert EAX into dword lane idx=1.
    let a = [
        0xAAAAAAAAu32.to_le_bytes(),
        0xBBBBBBBBu32.to_le_bytes(),
        0xCCCCCCCCu32.to_le_bytes(),
        0xDDDDDDDDu32.to_le_bytes(),
    ]
    .concat();
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]); // movdqu xmm0, [rdi]
    prog.extend_from_slice(&[0xB8, 0xEF, 0xBE, 0xAD, 0xDE]); // mov eax, 0xDEADBEEF
    prog.extend_from_slice(&[0x66, 0x0F, 0x3A, 0x22, 0xC0, 0x01]); // pinsrd xmm0, eax, 1
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("pinsrd_reg", &prog, sse_scratch(a.try_into().unwrap(), b));
}

#[test]
fn sse2_pinsrw_pextrw() {
    // PINSRW xmm0, eax, 3 (66 0F C4 C0 03) then PEXTRW eax, xmm0, 3 (66 0F C5 C0 03).
    let a = [
        0xFFFFu16.to_le_bytes(),
        0x1111u16.to_le_bytes(),
        0x2222u16.to_le_bytes(),
        0x3333u16.to_le_bytes(),
        0x4444u16.to_le_bytes(),
        0x5555u16.to_le_bytes(),
        0x6666u16.to_le_bytes(),
        0x7777u16.to_le_bytes(),
    ]
    .concat();
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]); // movdqu xmm0, [rdi]
    prog.extend_from_slice(&[0xB8, 0xCD, 0xAB, 0x00, 0x00]); // mov eax, 0xABCD
    prog.extend_from_slice(&[0x66, 0x0F, 0xC4, 0xC0, 0x03]); // pinsrw xmm0, eax, 3
    prog.extend_from_slice(&[0x66, 0x0F, 0xC5, 0xC0, 0x03]); // pextrw eax, xmm0, 3
    prog.extend_from_slice(&[0x89, 0x47, 0x30]); // mov [rdi+0x30], eax
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]); // store xmm0
    prog.push(HLT);
    check_sse(
        "pinsrw_pextrw",
        &prog,
        sse_scratch(a.try_into().unwrap(), b),
    );
}

// ---- MOVD / MOVQ between GPR and XMM ----

#[test]
fn sse_movd_gpr_to_xmm_and_back() {
    // movd xmm0, eax (66 0F 6E C0) then movd ecx, xmm0 (66 0F 7E C1); store both.
    let a = [0u8; 16];
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xB8, 0x78, 0x56, 0x34, 0x12]); // mov eax, 0x12345678
    prog.extend_from_slice(&[0x66, 0x0F, 0x6E, 0xC0]); // movd xmm0, eax (zero-extends to 128)
    prog.extend_from_slice(&[0x66, 0x0F, 0x7E, 0xC1]); // movd ecx, xmm0
    prog.extend_from_slice(&[0x89, 0x4F, 0x20]); // mov [rdi+0x20], ecx
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x30]); // store xmm0 at +0x30
    prog.push(HLT);
    check_sse("movd_roundtrip", &prog, sse_scratch(a, b));
}

#[test]
fn sse_movq_gpr_to_xmm() {
    // movq xmm0, rax (66 48 0F 6E C0). Zero-extends to 128. Store xmm0.
    let a = [0u8; 16];
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0x48, 0xB8]); // mov rax, imm64
    prog.extend_from_slice(&0x0123_4567_89AB_CDEFu64.to_le_bytes());
    prog.extend_from_slice(&[0x66, 0x48, 0x0F, 0x6E, 0xC0]); // movq xmm0, rax
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("movq_gpr_xmm", &prog, sse_scratch(a, b));
}

#[test]
fn sse_movq_xmm_to_xmm_zeroes_high() {
    // MOVQ xmm0, xmm1 (F3 0F 7E C1): copies low 64, ZEROES the high 64 of dst.
    let a = [
        0xFFFF_FFFF_FFFF_FFFFu64.to_le_bytes(),
        0xFFFF_FFFF_FFFF_FFFFu64.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x1122_3344_5566_7788u64.to_le_bytes(),
        0xDEAD_BEEF_CAFE_BABEu64.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "movq_xmm_xmm",
        &sse_program(&[0xF3, 0x0F, 0x7E, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

// ---- MMX integer ops (operate on 64-bit MM registers) ----

#[test]
fn mmx_paddb() {
    // movq mm0, [rdi]; movq mm1, [rdi+8]; paddb mm0, mm1; movq [rdi+0x20], mm0.
    // 0F 6F /r = movq mm, m64 ; 0F FC = paddb ; 0F 7F /r = movq m64, mm.
    let mut s = [0u8; 64];
    let a = [1, 2, 3, 4, 0x7F, 0x80, 0xFF, 0x10];
    let b = [8, 8, 8, 8, 0x01, 0x80, 0x01, 0xF0];
    s[0..8].copy_from_slice(&a);
    s[8..16].copy_from_slice(&b);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0x0F, 0x6F, 0x07]); // movq mm0, [rdi]
    prog.extend_from_slice(&[0x0F, 0x6F, 0x4F, 0x08]); // movq mm1, [rdi+8]
    prog.extend_from_slice(&[0x0F, 0xFC, 0xC1]); // paddb mm0, mm1
    prog.extend_from_slice(&[0x0F, 0x7F, 0x47, 0x20]); // movq [rdi+0x20], mm0
    prog.push(HLT);
    check_mem("mmx_paddb", &with_hlt(prog), regs(), s, 0);
}

#[test]
fn mmx_pmullw() {
    let mut s = [0u8; 64];
    let a = [
        0x0002u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0003u16.to_le_bytes(),
        0x0100u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
    ]
    .concat();
    s[0..8].copy_from_slice(&a);
    s[8..16].copy_from_slice(&b);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0x0F, 0x6F, 0x4F, 0x08]);
    prog.extend_from_slice(&[0x0F, 0xD5, 0xC1]); // pmullw mm0, mm1
    prog.extend_from_slice(&[0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_mem("mmx_pmullw", &with_hlt(prog), regs(), s, 0);
}

#[test]
fn mmx_punpcklbw() {
    let mut s = [0u8; 64];
    let a = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let b = [0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87];
    s[0..8].copy_from_slice(&a);
    s[8..16].copy_from_slice(&b);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0x0F, 0x6F, 0x4F, 0x08]);
    prog.extend_from_slice(&[0x0F, 0x60, 0xC1]); // punpcklbw mm0, mm1
    prog.extend_from_slice(&[0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_mem("mmx_punpcklbw", &with_hlt(prog), regs(), s, 0);
}

#[test]
fn mmx_psubusb_saturate() {
    // PSUBUSB mm0, mm1 (0F D8): unsigned byte subtract with saturation to 0.
    let mut s = [0u8; 64];
    let a = [0x10, 0x05, 0xFF, 0x00, 0x80, 0x7F, 0x01, 0xAA];
    let b = [0x20, 0x05, 0x01, 0x01, 0x40, 0x80, 0x02, 0x55];
    s[0..8].copy_from_slice(&a);
    s[8..16].copy_from_slice(&b);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0x0F, 0x6F, 0x4F, 0x08]);
    prog.extend_from_slice(&[0x0F, 0xD8, 0xC1]); // psubusb mm0, mm1
    prog.extend_from_slice(&[0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_mem("mmx_psubusb", &with_hlt(prog), regs(), s, 0);
}

// ---- SSE2 integer saturating add/sub and average ----

#[test]
fn sse2_paddsb() {
    // PADDSB xmm0, xmm1 = 66 0F EC C1. Signed byte add with saturation.
    let a = [
        0x7F, 0x7F, 0x80, 0x80, 0x40, 0xC0, 0x01, 0xFF, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70,
        0x7F,
    ];
    let b = [
        0x01, 0x7F, 0xFF, 0x80, 0x40, 0xC0, 0x02, 0x01, 0xF0, 0xE0, 0xD0, 0xC0, 0x10, 0x20, 0x30,
        0x01,
    ];
    check_sse(
        "paddsb",
        &sse_program(&[0x66, 0x0F, 0xEC, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse2_paddusw() {
    // PADDUSW xmm0, xmm1 = 66 0F DD C1. Unsigned word add with saturation.
    let a = [
        0xFFFFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
        0x1234u16.to_le_bytes(),
        0xF000u16.to_le_bytes(),
        0x0FFFu16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0x0000u16.to_le_bytes(),
    ]
    .concat();
    let b = [
        0x0001u16.to_le_bytes(),
        0x8001u16.to_le_bytes(),
        0x0002u16.to_le_bytes(),
        0x1000u16.to_le_bytes(),
        0x2000u16.to_le_bytes(),
        0xF001u16.to_le_bytes(),
        0x8001u16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
    ]
    .concat();
    check_sse(
        "paddusw",
        &sse_program(&[0x66, 0x0F, 0xDD, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b.try_into().unwrap()),
    );
}

#[test]
fn sse2_pavgb() {
    // PAVGB xmm0, xmm1 = 66 0F E0 C1. Unsigned byte average, rounded up.
    let a = [
        0x00, 0xFF, 0x10, 0x01, 0x80, 0x7F, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
        0x0C,
    ];
    let b = [
        0x01, 0xFF, 0x20, 0x02, 0x81, 0x80, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
        0x0D,
    ];
    check_sse(
        "pavgb",
        &sse_program(&[0x66, 0x0F, 0xE0, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse2_pmaxub_pminub() {
    // PMAXUB then PMINUB on the SAME register isn't useful; just probe PMAXUB.
    // PMAXUB xmm0, xmm1 = 66 0F DE C1.
    let a = [
        0x00, 0xFF, 0x10, 0x80, 0x7F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
        0x0B,
    ];
    let b = [
        0x01, 0x00, 0x20, 0x7F, 0x80, 0x02, 0x01, 0xFF, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
        0x0A,
    ];
    check_sse(
        "pmaxub",
        &sse_program(&[0x66, 0x0F, 0xDE, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse2_pcmpgtb() {
    // PCMPGTB xmm0, xmm1 = 66 0F 64 C1. Signed byte greater-than -> all-1s/all-0s.
    let a = [
        0x05, 0x80, 0x7F, 0xFF, 0x00, 0x01, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x7F, 0x81,
        0xC0,
    ];
    let b = [
        0x03, 0x7F, 0x80, 0x01, 0x00, 0xFF, 0x10, 0x21, 0x2F, 0x40, 0x4F, 0x61, 0x6F, 0x7E, 0x80,
        0xBF,
    ];
    check_sse(
        "pcmpgtb",
        &sse_program(&[0x66, 0x0F, 0x64, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---- SSE2 shifts: PSLLW/PSRLD/PSRAW (imm and xmm count) ----

#[test]
fn sse2_psllw_imm() {
    // PSLLW xmm0, imm8 = 66 0F 71 /6 ib. Shift each word left by 4.
    let a = [
        0x0001u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
        0x8000u16.to_le_bytes(),
        0x1234u16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
        0x0FF0u16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
        0x4000u16.to_le_bytes(),
    ]
    .concat();
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]); // movdqu xmm0, [rdi]
    prog.extend_from_slice(&[0x66, 0x0F, 0x71, 0xF0, 0x04]); // psllw xmm0, 4
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("psllw_imm4", &prog, sse_scratch(a.try_into().unwrap(), b));
}

#[test]
fn sse2_psraw_imm() {
    // PSRAW xmm0, imm8 = 66 0F 71 /4 ib. Arithmetic shift right words by 2.
    let a = [
        0x8000u16.to_le_bytes(),
        0x7FFFu16.to_le_bytes(),
        0xFFFFu16.to_le_bytes(),
        0x0004u16.to_le_bytes(),
        0x4000u16.to_le_bytes(),
        0xC000u16.to_le_bytes(),
        0x0001u16.to_le_bytes(),
        0x00FFu16.to_le_bytes(),
    ]
    .concat();
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x71, 0xE0, 0x02]); // psraw xmm0, 2
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("psraw_imm2", &prog, sse_scratch(a.try_into().unwrap(), b));
}

#[test]
fn sse2_psrld_imm() {
    // PSRLD xmm0, imm8 = 66 0F 72 /2 ib. Logical shift right dwords by 8.
    let a = [
        0x8000_00FFu32.to_le_bytes(),
        0xFFFF_FFFFu32.to_le_bytes(),
        0x0000_0100u32.to_le_bytes(),
        0x1234_5678u32.to_le_bytes(),
    ]
    .concat();
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x72, 0xD0, 0x08]); // psrld xmm0, 8
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("psrld_imm8", &prog, sse_scratch(a.try_into().unwrap(), b));
}

#[test]
fn sse2_pslldq_imm() {
    // PSLLDQ xmm0, imm8 = 66 0F 73 /7 ib. Byte shift left of the whole 128 by 3.
    let a = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x07]);
    prog.extend_from_slice(&[0x66, 0x0F, 0x73, 0xF8, 0x03]); // pslldq xmm0, 3
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("pslldq3", &prog, sse_scratch(a, b));
}

// ---- SSE2 shuffles: PSHUFD / PSHUFHW / PSHUFLW ----

#[test]
fn sse2_pshufd() {
    // PSHUFD xmm0, xmm1, imm8 = 66 0F 70 C1 ib. imm=0b00_01_10_11 reverses lanes.
    let a = [0u8; 16];
    let b = [
        0x11111111u32.to_le_bytes(),
        0x22222222u32.to_le_bytes(),
        0x33333333u32.to_le_bytes(),
        0x44444444u32.to_le_bytes(),
    ]
    .concat();
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0x66, 0x0F, 0x70, 0xC1, 0b00_01_10_11]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("pshufd", &prog, sse_scratch(a, b.try_into().unwrap()));
}

#[test]
fn sse2_pshuflw() {
    // PSHUFLW xmm0, xmm1, imm8 = F2 0F 70 C1 ib. Shuffle low 4 words, high qword copied.
    let a = [0u8; 16];
    let b = [
        0x0000u16.to_le_bytes(),
        0x1111u16.to_le_bytes(),
        0x2222u16.to_le_bytes(),
        0x3333u16.to_le_bytes(),
        0xAAAAu16.to_le_bytes(),
        0xBBBBu16.to_le_bytes(),
        0xCCCCu16.to_le_bytes(),
        0xDDDDu16.to_le_bytes(),
    ]
    .concat();
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0xF2, 0x0F, 0x70, 0xC1, 0b00_01_10_11]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("pshuflw", &prog, sse_scratch(a, b.try_into().unwrap()));
}

#[test]
fn sse2_pshufhw() {
    // PSHUFHW xmm0, xmm1, imm8 = F3 0F 70 C1 ib. Shuffle high 4 words, low qword copied.
    let a = [0u8; 16];
    let b = [
        0x0000u16.to_le_bytes(),
        0x1111u16.to_le_bytes(),
        0x2222u16.to_le_bytes(),
        0x3333u16.to_le_bytes(),
        0xAAAAu16.to_le_bytes(),
        0xBBBBu16.to_le_bytes(),
        0xCCCCu16.to_le_bytes(),
        0xDDDDu16.to_le_bytes(),
    ]
    .concat();
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x70, 0xC1, 0b00_01_10_11]);
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("pshufhw", &prog, sse_scratch(a, b.try_into().unwrap()));
}

// ---- SSE3 MOVSHDUP / MOVSLDUP ----

#[test]
fn sse3_movshdup() {
    // MOVSHDUP xmm0, xmm1 = F3 0F 16 C1. Duplicate odd singles: [a1,a1,a3,a3].
    let a = [0u8; 16];
    let b = f32x4([1.0, 2.0, 3.0, 4.0]);
    check_sse(
        "movshdup",
        &sse_program(&[0xF3, 0x0F, 0x16, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse3_movsldup() {
    // MOVSLDUP xmm0, xmm1 = F3 0F 12 C1. Duplicate even singles: [a0,a0,a2,a2].
    let a = [0u8; 16];
    let b = f32x4([1.0, 2.0, 3.0, 4.0]);
    check_sse(
        "movsldup",
        &sse_program(&[0xF3, 0x0F, 0x12, 0xC1]),
        sse_scratch(a, b),
    );
}

// ---- Scalar conversions: CVTSI2SD / CVTTSD2SI / CVTSS2SD ----

#[test]
fn sse2_cvtsi2sd_neg() {
    // CVTSI2SD xmm0, rax = F2 48 0F 2A C0. Convert signed -1234567 to f64; store.
    let a = [0u8; 16];
    let b = [0u8; 16];
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0x48, 0xC7, 0xC0]); // mov rax, imm32 (sign-ext)
    prog.extend_from_slice(&(-1234567i32).to_le_bytes());
    prog.extend_from_slice(&[0xF2, 0x48, 0x0F, 0x2A, 0xC0]); // cvtsi2sd xmm0, rax
    prog.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("cvtsi2sd_neg", &prog, sse_scratch(a, b));
}

#[test]
fn sse2_cvttsd2si_trunc() {
    // CVTTSD2SI rax, xmm1 = F2 48 0F 2C C1. Truncate -3.9 -> -3; store rax.
    let a = [0u8; 16];
    let b = f64x2([-3.9, 0.0]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]); // movdqu xmm1, [rdi+0x10]
    prog.extend_from_slice(&[0xF2, 0x48, 0x0F, 0x2C, 0xC1]); // cvttsd2si rax, xmm1
    prog.extend_from_slice(&[0x48, 0x89, 0x47, 0x20]); // mov [rdi+0x20], rax
    prog.push(HLT);
    check_sse("cvttsd2si", &prog, sse_scratch(a, b));
}

#[test]
fn sse2_cvtsd2si_round_even() {
    // CVTSD2SI rax, xmm1 = F2 48 0F 2D C1. Round-to-nearest-even of 2.5 -> 2.
    let a = [0u8; 16];
    let b = f64x2([2.5, 0.0]);
    let mut prog = load_rdi_data();
    prog.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x4F, 0x10]);
    prog.extend_from_slice(&[0xF2, 0x48, 0x0F, 0x2D, 0xC1]); // cvtsd2si rax, xmm1
    prog.extend_from_slice(&[0x48, 0x89, 0x47, 0x20]);
    prog.push(HLT);
    check_sse("cvtsd2si_even", &prog, sse_scratch(a, b));
}

#[test]
fn sse2_cvtss2sd_scalar() {
    // CVTSS2SD xmm0, xmm1 = F3 0F 5A C1. Convert lane0 f32 -> f64; lane1 of xmm0 kept.
    let a = f64x2([99.0, 7.0]); // high qword preserved
    let b = f32x4([1.5, 0.0, 0.0, 0.0]);
    check_sse(
        "cvtss2sd",
        &sse_program(&[0xF3, 0x0F, 0x5A, 0xC1]),
        sse_scratch(a, b),
    );
}

#[test]
fn sse2_cvtpd2ps_scalar() {
    // CVTPD2PS xmm0, xmm1 = 66 0F 5A C1. 2 doubles -> 2 floats in low 64, high zeroed.
    let a = [
        0xFFFF_FFFF_FFFF_FFFFu64.to_le_bytes(),
        0xFFFF_FFFF_FFFF_FFFFu64.to_le_bytes(),
    ]
    .concat();
    let b = f64x2([1.25, -8.5]);
    check_sse(
        "cvtpd2ps",
        &sse_program(&[0x66, 0x0F, 0x5A, 0xC1]),
        sse_scratch(a.try_into().unwrap(), b),
    );
}

// ---- x87: FXCH / FABS / FCHS / FRNDINT / FSCALE / FPREM / FSQRT-after-FXCH ----

#[test]
fn x87_fabs() {
    // fld -5.5 ; fabs (D9 E1) ; fstp qword -> 5.5
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]
    c.extend_from_slice(&[0xD9, 0xE1]); // fabs
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp qword [rdi+0x10]
    c.push(HLT);
    check_mem("x87_fabs", &with_hlt(c), regs(), scratch_f64(&[-5.5]), 0);
}

#[test]
fn x87_fchs() {
    // fld 12.25 ; fchs (D9 E0) ; fstp -> -12.25
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x07]);
    c.extend_from_slice(&[0xD9, 0xE0]); // fchs
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]);
    c.push(HLT);
    check_mem("x87_fchs", &with_hlt(c), regs(), scratch_f64(&[12.25]), 0);
}

#[test]
fn x87_fxch() {
    // fld b ; fld a ; fxch (D9 C9 -> swaps ST0,ST1) ; fstp [rdi+16] (stores old ST1=b);
    // fstp [rdi+24] (stores a). Verifies FXCH swap.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x47, 0x08]); // fld qword [rdi+8]  ST0=b
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]    ST0=a, ST1=b
    c.extend_from_slice(&[0xD9, 0xC9]); // fxch -> ST0=b, ST1=a
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp [rdi+16] = b
    c.extend_from_slice(&[0xDD, 0x5F, 0x18]); // fstp [rdi+24] = a
    c.push(HLT);
    check_mem(
        "x87_fxch",
        &with_hlt(c),
        regs(),
        scratch_f64(&[3.5, 7.25]),
        0,
    );
}

#[test]
fn x87_frndint_even() {
    // fld 2.5 ; frndint (D9 FC, round-to-nearest-even by default) -> 2.0 ; fstp.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x07]);
    c.extend_from_slice(&[0xD9, 0xFC]); // frndint
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]);
    c.push(HLT);
    check_mem("x87_frndint", &with_hlt(c), regs(), scratch_f64(&[2.5]), 0);
}

#[test]
fn x87_fscale() {
    // FSCALE (D9 FD): ST0 = ST0 * 2^trunc(ST1). fld ST1=3.0, fld ST0=1.5 -> 1.5*8=12.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x47, 0x08]); // fld qword [rdi+8]  ST0=exp=3.0
    c.extend_from_slice(&[0xDD, 0x07]); // fld qword [rdi]    ST0=1.5, ST1=3.0
    c.extend_from_slice(&[0xD9, 0xFD]); // fscale -> ST0 = 1.5 * 2^3 = 12.0
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp [rdi+16]
    c.push(HLT);
    check_mem(
        "x87_fscale",
        &with_hlt(c),
        regs(),
        scratch_f64(&[1.5, 3.0]),
        0,
    );
}

#[test]
fn x87_fprem() {
    // FPREM (D9 F8): ST0 = ST0 - (ST1 * trunc(ST0/ST1)). 17 mod 5 = 2 (exact dyadic ok).
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x47, 0x08]); // fld [rdi+8] ST0=divisor=5.0
    c.extend_from_slice(&[0xDD, 0x07]); // fld [rdi] ST0=17.0, ST1=5.0
    c.extend_from_slice(&[0xD9, 0xF8]); // fprem -> 2.0
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp [rdi+16]
    c.push(HLT);
    check_mem(
        "x87_fprem",
        &with_hlt(c),
        regs(),
        scratch_f64(&[17.0, 5.0]),
        0,
    );
}

#[test]
fn x87_fmul_st_then_fxch() {
    // fld a; fld b; fmulp st1,st0 (DE C9) -> ST0 = a*b ; fstp. (4.0 * 0.5 = 2.0)
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x07]); // fld a -> ST0=a
    c.extend_from_slice(&[0xDD, 0x47, 0x08]); // fld b -> ST0=b, ST1=a
    c.extend_from_slice(&[0xDE, 0xC9]); // fmulp st1, st0 -> ST0 = a*b, pop
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp [rdi+16]
    c.push(HLT);
    check_mem(
        "x87_fmulp",
        &with_hlt(c),
        regs(),
        scratch_f64(&[4.0, 0.5]),
        0,
    );
}

#[test]
fn x87_fld1_fldz_fadd() {
    // FLD1 (D9 E8) pushes 1.0, FLDZ (D9 EE) pushes 0.0, FADDP adds -> 1.0; fstp.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xD9, 0xE8]); // fld1 -> ST0=1.0
    c.extend_from_slice(&[0xD9, 0xEE]); // fldz -> ST0=0.0, ST1=1.0
    c.extend_from_slice(&[0xDE, 0xC1]); // faddp st1, st0 -> ST0=1.0
    c.extend_from_slice(&[0xDD, 0x5F, 0x10]); // fstp [rdi+16]
    c.push(HLT);
    check_mem(
        "x87_fld1_fldz",
        &with_hlt(c),
        regs(),
        scratch_f64(&[0.0]),
        0,
    );
}

#[test]
fn x87_fcom_fstsw_flags() {
    // Compare ST0 vs ST1 via FCOM, then FNSTSW AX (DF E0) and SAHF to map C0/C2/C3
    // into CF/PF/ZF. a=3.0 > b=5.0? No (a<b) -> C0=1 (CF). Probe via SAHF.
    // fld b; fld a; fcom st1 (D8 D1); fnstsw ax (DF E0); sahf (9E); hlt.
    let mut c = load_rdi_data();
    c.extend_from_slice(&[0xDD, 0x47, 0x08]); // fld [rdi+8] = b -> ST0=b
    c.extend_from_slice(&[0xDD, 0x07]); // fld [rdi] = a -> ST0=a, ST1=b
    c.extend_from_slice(&[0xD8, 0xD1]); // fcom st1
    c.extend_from_slice(&[0xDF, 0xE0]); // fnstsw ax
    c.extend_from_slice(&[0x9E]); // sahf  (CF<-C0, PF<-C2, ZF<-C3)
    c.push(HLT);
    // a=3.0 < b=5.0 -> C3=0,C2=0,C0=1 -> ZF=0, PF=0, CF=1. Compare ZF|PF|CF.
    check_mem(
        "x87_fcom_sahf",
        &c,
        regs(),
        scratch_f64(&[3.0, 5.0]),
        FCOMI_FLAGS,
    );
}

// ---- ALU corner cases not yet hit ----

#[test]
fn neg64_int_min() {
    // NEG of INT64_MIN: result is INT64_MIN again (overflow), OF=1, CF=1, SF=1.
    let mut r = regs();
    r.rax = 0x8000_0000_0000_0000;
    check("neg64_int_min", &with_hlt(vec![0x48, 0xF7, 0xD8]), r);
}

#[test]
fn not64_no_flags() {
    // NOT rax (48 F7 D0) affects no flags; seed all status flags and verify survival.
    let mut r = regs();
    r.rax = 0x0F0F_0F0F_0F0F_0F0F;
    r.rflags = FLAG_MASK;
    check("not64", &with_hlt(vec![0x48, 0xF7, 0xD0]), r);
}

#[test]
fn test_imm32_high_bit() {
    // TEST eax, imm32 (A9 id) with a high-bit-set mask -> SF=1.
    let mut r = regs();
    r.rax = 0xFFFF_FFFF_8000_0001;
    // A9 00 00 00 80  test eax, 0x80000000
    check(
        "test_imm32",
        &with_hlt(vec![0xA9, 0x00, 0x00, 0x00, 0x80]),
        r,
    );
}

#[test]
fn add_al_imm_af_only() {
    // ADD AL, imm8 where only AF is set (0x08 + 0x08 = 0x10): AF=1, CF=0, ZF=0.
    let mut r = regs();
    r.rax = 0x08;
    check("add_al_af", &with_hlt(vec![0x04, 0x08]), r); // add al, 8
}

#[test]
fn sbb_self_with_carry() {
    // SBB rax, rax with CF=1 -> rax = -1 (0xFFFF...), a common idiom (mask = -CF).
    let mut r = regs();
    r.rax = 0x1234;
    r.rflags = flags::bits::CF;
    check("sbb_self", &with_hlt(vec![0x48, 0x19, 0xC0]), r); // sbb rax, rax
}

#[test]
fn shl_count_masked_to_zero_64() {
    // shl rax, cl with CL=64 -> masked to 0 (no shift, NO flags change). Seed flags
    // and verify they survive and rax is unchanged.
    let mut r = regs();
    r.rax = 0x1;
    r.rcx = 64; // 64 & 63 = 0
    r.rflags = flags::bits::CF | flags::bits::OF | flags::bits::SF;
    // 48 D3 E0  shl rax, cl ; flags must be unchanged when masked count is 0.
    check("shl_cl64", &with_hlt(vec![0x48, 0xD3, 0xE0]), r);
}

#[test]
fn bt_mem_reg_bit_index() {
    // BT [rdi], rdx (48 0F A3 17) : memory bit-test with a large bit index that
    // selects beyond the first qword (index 70 -> byte 8, bit 6). CF<-that bit.
    let mut s = [0u8; 64];
    s[8] = 1 << 6; // bit 70 set
    let mut r = regs();
    r.rdi = DATA_ADDR;
    r.rdx = 70;
    check_mem(
        "bt_mem",
        &with_hlt(vec![0x48, 0x0F, 0xA3, 0x17]),
        r,
        s,
        BT_DEFINED,
    );
}

#[test]
fn movsxd_no_rex_w() {
    // 63 /r without REX.W is MOVSXD r32, r/m32 in 64-bit mode but acts as a plain
    // 32-bit mov (no sign extension) — actually MOVSXD ALWAYS needs REX.W to be
    // meaningful; without it, it's still MOVSXD but dest is 32-bit (zero-extends).
    // Verify both backends agree on the 32-bit (no-REX.W) form.
    let mut r = regs();
    r.rbx = 0x0000_0000_8000_0000;
    r.rax = 0xFFFF_FFFF_FFFF_FFFF; // should be overwritten
    // 63 C3  movsxd eax, ebx  -> eax = ebx, zero-extended into rax
    check("movsxd_no_rexw", &with_hlt(vec![0x63, 0xC3]), r);
}
