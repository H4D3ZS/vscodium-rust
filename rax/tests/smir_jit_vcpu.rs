//! End-to-end M4 integration test: the SMIR native hot-block JIT tier executing
//! through the real `X86_64Vcpu` state, validated against the interpreter.
//!
//! Run with: `cargo test --features smir-jit --test smir_jit_vcpu -- --nocapture`
#![cfg(all(feature = "smir-jit", target_arch = "x86_64"))]

use std::sync::Arc;
use std::time::Instant;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestRegionMmap, MmapRegion};

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::{Registers, SystemRegisters, VCpu, VcpuExit};

const LOAD_ADDR: u64 = 0x10_0000;
const MEM_SIZE: u64 = 16 * 1024 * 1024;

/// Build a vcpu loaded with the `bench_loop` hot loop for `iters` iterations.
//   xor eax,eax ; mov ecx,iters ; loop: add eax,3 ; xor edx,edx ; sub eax,1 ;
//   dec ecx ; jnz loop ; hlt
fn make_vcpu(iters: u32) -> X86_64Vcpu {
    let mut code: Vec<u8> = vec![0x31, 0xC0]; // xor eax,eax
    code.push(0xB9); // mov ecx, imm32
    code.extend_from_slice(&iters.to_le_bytes());
    code.extend_from_slice(&[0x83, 0xC0, 0x03]); // add eax,3
    code.extend_from_slice(&[0x31, 0xD2]); // xor edx,edx
    code.extend_from_slice(&[0x83, 0xE8, 0x01]); // sub eax,1
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xF4]); // jnz loop
    code.push(0xF4); // hlt

    let region = MmapRegion::new(MEM_SIZE as usize).unwrap();
    let guest_region = GuestRegionMmap::new(region, GuestAddress(0)).unwrap();
    let memory = Arc::new(GuestMemoryMmap::from_regions(vec![guest_region]).unwrap());
    memory.write_slice(&code, GuestAddress(LOAD_ADDR)).unwrap();

    let mut regs = Registers::default();
    regs.rip = LOAD_ADDR;
    regs.rsp = 0x11_0000;
    regs.rflags = 0x2;

    let mut sregs = SystemRegisters::default();
    sregs.cr0 = 0x21;
    sregs.cr4 = 0x20;
    sregs.efer = 0x500;
    sregs.cs.base = 0;
    sregs.cs.limit = 0xFFFFFFFF;
    sregs.cs.selector = 0x8;
    sregs.cs.type_ = 0xB;
    sregs.cs.present = true;
    sregs.cs.s = true;
    sregs.cs.l = true;
    sregs.cs.g = true;
    sregs.ds.base = 0;
    sregs.ds.limit = 0xFFFFFFFF;
    sregs.ds.selector = 0x10;
    sregs.ds.type_ = 0x3;
    sregs.ds.present = true;
    sregs.ds.db = true;
    sregs.ds.s = true;
    sregs.ds.g = true;
    sregs.es = sregs.ds.clone();
    sregs.fs = sregs.ds.clone();
    sregs.gs = sregs.ds.clone();
    sregs.ss = sregs.ds.clone();

    let mut vcpu = X86_64Vcpu::new(0, memory);
    vcpu.set_regs(&regs).unwrap();
    vcpu.set_sregs(&sregs).unwrap();
    vcpu
}

/// Build a vcpu loaded with arbitrary guest `code` at LOAD_ADDR.
fn make_vcpu_code(code: &[u8]) -> X86_64Vcpu {
    let region = MmapRegion::new(MEM_SIZE as usize).unwrap();
    let guest_region = GuestRegionMmap::new(region, GuestAddress(0)).unwrap();
    let memory = Arc::new(GuestMemoryMmap::from_regions(vec![guest_region]).unwrap());
    memory.write_slice(code, GuestAddress(LOAD_ADDR)).unwrap();

    let mut regs = Registers::default();
    regs.rip = LOAD_ADDR;
    regs.rsp = 0x11_0000;
    regs.rflags = 0x2;

    let mut sregs = SystemRegisters::default();
    sregs.cr0 = 0x21;
    sregs.cr4 = 0x20;
    sregs.efer = 0x500;
    sregs.cs.limit = 0xFFFFFFFF;
    sregs.cs.selector = 0x8;
    sregs.cs.type_ = 0xB;
    sregs.cs.present = true;
    sregs.cs.s = true;
    sregs.cs.l = true;
    sregs.cs.g = true;
    sregs.ds.limit = 0xFFFFFFFF;
    sregs.ds.selector = 0x10;
    sregs.ds.type_ = 0x3;
    sregs.ds.present = true;
    sregs.ds.db = true;
    sregs.ds.s = true;
    sregs.ds.g = true;
    sregs.es = sregs.ds.clone();
    sregs.fs = sregs.ds.clone();
    sregs.gs = sregs.ds.clone();
    sregs.ss = sregs.ds.clone();

    let mut vcpu = X86_64Vcpu::new(0, memory);
    vcpu.set_regs(&regs).unwrap();
    vcpu.set_sregs(&sregs).unwrap();
    vcpu
}

/// The eligibility/clobber checks must DECLINE ineligible regions (returning
/// None so a caller falls back to the interpreter) rather than mis-compile —
/// the safety net for a future automatic `run()` integration.
#[test]
fn jit_bails_on_ineligible() {
    // (a) A region with a CALL terminator — not a self-contained HALT region.
    //     call $+5 ; hlt
    let mut v = make_vcpu_code(&[0xE8, 0x00, 0x00, 0x00, 0x00, 0xF4]);
    assert!(
        !v.jit_try_block().expect("jit_try_block"),
        "a CALL region (entry block is a frontier) must bail to the interpreter"
    );

    // (b) A straight-line block ending in RET — entry block is a frontier exit,
    //     so there is no internal native work; must bail.
    let mut v = make_vcpu_code(&[0xB8, 0x05, 0x00, 0x00, 0x00, 0xC3]);
    assert!(
        !v.jit_try_block().expect("jit_try_block"),
        "an entry-is-frontier region must bail"
    );

    // (c) A frontier-less spin loop `jmp $` (EB FE) — running it natively would
    //     loop forever with no way back; must bail (and must NOT hang).
    let mut v = make_vcpu_code(&[0xEB, 0xFE]);
    assert!(
        !v.jit_try_block().expect("jit_try_block"),
        "a frontier-less infinite loop must bail (no native exit)"
    );

    // (d) A loop whose body READS MEMORY (mov eax,[rbx]; dec ecx; jnz) — memory
    //     ops are not on the JIT whitelist (they would access wrong host
    //     addresses under native exec); the region MUST bail to the interpreter
    //     and never execute natively. This is the kernel-safety gate.
    let mut v = make_vcpu_code(&[0x8B, 0x03, 0xFF, 0xC9, 0x75, 0xFA, 0xF4]);
    let mut r = v.get_regs().unwrap();
    r.rcx = 5;
    r.rbx = LOAD_ADDR;
    v.set_regs(&r).unwrap();
    assert!(
        !v.jit_try_block().expect("jit_try_block"),
        "a memory-touching loop must bail (not JIT-safe)"
    );
}

fn run_interp(vcpu: &mut X86_64Vcpu) {
    loop {
        match vcpu.step() {
            Ok(Some(VcpuExit::Hlt)) => break,
            Ok(Some(_)) => break,
            Ok(None) => {}
            Err(e) => panic!("interp error: {e:?}"),
        }
    }
}

/// The JIT-tier final state must equal the interpreter's, register for register.
#[test]
fn jit_matches_interpreter() {
    let iters = 1000u32;

    let mut jit = make_vcpu(iters);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "the loop region should JIT and advance to its exit"
    );
    // The JIT ran the loop natively and parked RIP at the HLT (the frontier
    // exit, not yet executed). Step it so both vcpus end at the same point.
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    let mut interp = make_vcpu(iters);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    // Whole-loop native execution must reproduce the interpreter's GPR state.
    assert_eq!(jr.rax, ir.rax, "rax");
    assert_eq!(jr.rcx, ir.rcx, "rcx");
    assert_eq!(jr.rdx, ir.rdx, "rdx");
    assert_eq!(jr.rbx, ir.rbx, "rbx");
    assert_eq!(jr.rsi, ir.rsi, "rsi");
    assert_eq!(jr.rdi, ir.rdi, "rdi");
    assert_eq!(jr.r8, ir.r8, "r8");
    assert_eq!(jr.r15, ir.r15, "r15");
    // Sanity vs the closed-form result: eax = 2*iters, ecx = 0.
    assert_eq!(jr.rax & 0xffff_ffff, (2 * iters as u64) & 0xffff_ffff);
    assert_eq!(jr.rcx & 0xffff_ffff, 0);
}

/// Register-only LEA (address arithmetic, no dereference) + BSF (bit-scan) in a
/// hot loop must JIT bit-exactly vs the interpreter — verifies the whitelist
/// additions (Lea/Bsf/Bsr) lower correctly under the native runtime.
#[test]
fn jit_lea_sib_matches_interpreter() {
    // xor eax,eax; mov ecx,300
    // loop: lea edx,[rax+rax*2+5]  (SIB base+index*scale+disp); add eax,1;
    //       dec ecx; jnz loop; hlt
    let code: &[u8] = &[
        0x31, 0xC0, // xor eax,eax
        0xB9, 0x2C, 0x01, 0x00, 0x00, // mov ecx,300
        0x8D, 0x54, 0x40, 0x05, // loop: lea edx,[rax+rax*2+5]
        0x83, 0xC0, 0x01, // add eax,1
        0xFF, 0xC9, // dec ecx
        0x75, 0xF5, // jnz loop
        0xF4, // hlt
    ];

    let mut jit = make_vcpu_code(code);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "LEA-SIB loop should JIT and advance to its exit"
    );
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    let mut interp = make_vcpu_code(code);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    assert_eq!(jr.rax, ir.rax, "rax");
    assert_eq!(jr.rcx, ir.rcx, "rcx");
    assert_eq!(jr.rdx, ir.rdx, "rdx (lea base+index*scale+disp)");
    assert_eq!(jr.rax & 0xffff_ffff, 300, "closed form: eax == iters");
    // Last iter rax==299: edx = 299*3 + 5 = 902.
    assert_eq!(jr.rdx & 0xffff_ffff, 902, "lea result of last iteration");
}

/// Variable shift by CL (`shl edx,cl`) in a hot loop — the pattern the kernel's
/// __free_pages_memory bootmem loop uses. Must JIT bit-exactly vs the interpreter.
#[test]
fn jit_shl_cl_matches_interpreter() {
    // mov ecx,5; xor eax,eax
    // loop: mov edx,1; shl edx,cl; add eax,edx; dec ecx; jns loop; hlt
    let code: &[u8] = &[
        0xB9, 0x05, 0x00, 0x00, 0x00, // mov ecx,5
        0x31, 0xC0, // xor eax,eax
        0xBA, 0x01, 0x00, 0x00, 0x00, // loop: mov edx,1
        0xD3, 0xE2, // shl edx,cl
        0x01, 0xD0, // add eax,edx
        0xFF, 0xC9, // dec ecx
        0x79, 0xF3, // jns loop
        0xF4, // hlt
    ];

    let mut jit = make_vcpu_code(code);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "SHL-by-CL loop should JIT and advance to its exit"
    );
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    let mut interp = make_vcpu_code(code);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    assert_eq!(jr.rax, ir.rax, "rax (sum of 1<<cl for cl=5..0)");
    assert_eq!(jr.rdx, ir.rdx, "rdx (last shl result)");
    // Sum 32+16+8+4+2+1 = 63.
    assert_eq!(jr.rax & 0xffff_ffff, 63, "closed form sum");
}

/// CMOVcc in a hot loop (the conditional-move pattern the kernel bootmem loop
/// uses, `cmovge`). Must JIT bit-exactly vs the interpreter.
#[test]
fn jit_cmovge_matches_interpreter() {
    // mov ecx,100; xor eax,eax; mov ebx,0xFF
    // loop: add eax,1; cmp eax,50; cmovge ebx,eax; dec ecx; jnz loop; hlt
    let code: &[u8] = &[
        0xB9, 0x64, 0x00, 0x00, 0x00, // mov ecx,100
        0x31, 0xC0, // xor eax,eax
        0xBB, 0xFF, 0x00, 0x00, 0x00, // mov ebx,0xFF
        0x83, 0xC0, 0x01, // loop: add eax,1
        0x83, 0xF8, 0x32, // cmp eax,50
        0x0F, 0x4D, 0xD8, // cmovge ebx,eax
        0xFF, 0xC9, // dec ecx
        0x75, 0xF3, // jnz loop
        0xF4, // hlt
    ];

    let mut jit = make_vcpu_code(code);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "CMOVGE loop should JIT and advance to its exit"
    );
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    let mut interp = make_vcpu_code(code);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    assert_eq!(jr.rax, ir.rax, "rax");
    assert_eq!(jr.rbx, ir.rbx, "rbx (cmovge target)");
    assert_eq!(jr.rcx, ir.rcx, "rcx");
    // eax 1..100; cmovge ebx,eax fires while eax>=50 → ebx ends at 100.
    assert_eq!(jr.rbx & 0xffff_ffff, 100, "closed-form cmovge result");
}

/// Loop with an EARLY forward exit to a separate continuation (two distinct
/// frontier exits + a back-edge) — the multi-exit CFG shape the kernel hot
/// regions use. Stresses the JIT's CFG / exit-PC lowering. Must match interp.
#[test]
fn jit_loop_early_exit_matches_interpreter() {
    // xor eax,eax; xor ebx,ebx; mov ecx,1000
    // loop: add eax,1; cmp eax,500; jge early; dec ecx; jnz loop; jmp late
    // early: mov ebx,0x1111; hlt
    // late:  mov ebx,0x2222; hlt
    let code: &[u8] = &[
        0x31, 0xC0, // xor eax,eax
        0x31, 0xDB, // xor ebx,ebx
        0xB9, 0xE8, 0x03, 0x00, 0x00, // mov ecx,1000
        0x83, 0xC0, 0x01, // loop: add eax,1
        0x3D, 0xF4, 0x01, 0x00, 0x00, // cmp eax,500
        0x7D, 0x06, // jge early
        0xFF, 0xC9, // dec ecx
        0x75, 0xF2, // jnz loop
        0xEB, 0x06, // jmp late
        0xBB, 0x11, 0x11, 0x00, 0x00, // early: mov ebx,0x1111
        0xF4, // hlt
        0xBB, 0x22, 0x22, 0x00, 0x00, // late: mov ebx,0x2222
        0xF4, // hlt
    ];

    let mut jit = make_vcpu_code(code);
    // May or may not promote in one shot; if it JITs, it must match the interp.
    let _ = jit.jit_try_block().expect("jit_try_block");
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    let mut interp = make_vcpu_code(code);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    assert_eq!(jr.rax, ir.rax, "rax");
    assert_eq!(jr.rbx, ir.rbx, "rbx (which continuation ran)");
    assert_eq!(jr.rcx, ir.rcx, "rcx");
    // eax reaches 500 (iter 500) before ecx hits 0 → early taken → ebx=0x1111.
    assert_eq!(jr.rbx & 0xffff_ffff, 0x1111, "early continuation taken");
    assert_eq!(jr.rax & 0xffff_ffff, 500, "exited at eax==500");
}

/// Loop containing a CALL (a Call-terminator frontier the JIT exits through, as
/// the kernel hrtimer/text_poke hot regions do). The JIT runs up to the call,
/// hands back to the interpreter to execute call+ret, then re-enters. Final
/// state must match the pure interpreter.
#[test]
fn jit_loop_with_call_matches_interpreter() {
    // xor eax,eax; mov ecx,5
    // loop: add eax,1; call func; dec ecx; jnz loop; hlt
    // func: ret
    let code: &[u8] = &[
        0x31, 0xC0, // xor eax,eax
        0xB9, 0x05, 0x00, 0x00, 0x00, // mov ecx,5
        0x83, 0xC0, 0x01, // loop: add eax,1
        0xE8, 0x05, 0x00, 0x00, 0x00, // call func (rel32 +5)
        0xFF, 0xC9, // dec ecx
        0x75, 0xF4, // jnz loop
        0xF4, // hlt
        0xC3, // func: ret
    ];

    let mut jit = make_vcpu_code(code);
    let _ = jit.jit_try_block().expect("jit_try_block");
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    let mut interp = make_vcpu_code(code);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    assert_eq!(jr.rax, ir.rax, "rax");
    assert_eq!(jr.rcx, ir.rcx, "rcx");
    assert_eq!(jr.rsp, ir.rsp, "rsp (call/ret balance)");
    assert_eq!(jr.rax & 0xffff_ffff, 5, "5 iterations");
}

/// Conditional CALL where the FALL-THROUGH (not the taken branch) is the
/// frontier — the exact polarity of the kernel hrtimer region (`test;jcc cont;
/// call`). Exercises the JIT exiting on a fall-through frontier with the correct
/// resume PC. Must match interp.
#[test]
fn jit_loop_cond_call_matches_interpreter() {
    // xor eax,eax; mov ecx,5
    // loop: add eax,1; test al,1; jnz cont; call func; cont: dec ecx; jnz loop; hlt
    // func: ret
    let code: &[u8] = &[
        0x31, 0xC0, // xor eax,eax
        0xB9, 0x05, 0x00, 0x00, 0x00, // mov ecx,5
        0x83, 0xC0, 0x01, // loop: add eax,1
        0xA8, 0x01, // test al,1
        0x75, 0x05, // jnz cont (skip call)
        0xE8, 0x05, 0x00, 0x00, 0x00, // call func (fall-through frontier)
        0xFF, 0xC9, // cont: dec ecx
        0x75, 0xF0, // jnz loop
        0xF4, // hlt
        0xC3, // func: ret
    ];

    let mut jit = make_vcpu_code(code);
    let _ = jit.jit_try_block().expect("jit_try_block");
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    let mut interp = make_vcpu_code(code);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    assert_eq!(jr.rax, ir.rax, "rax");
    assert_eq!(jr.rcx, ir.rcx, "rcx");
    assert_eq!(jr.rsp, ir.rsp, "rsp (call/ret balance)");
    assert_eq!(jr.rax & 0xffff_ffff, 5, "5 iterations");
}

/// A realistic hot loop with an INTERNAL conditional (if-inside-loop): multiple
/// internal blocks, a forward branch + a join, two back-edges to the head, and a
/// HLT frontier — all run natively by `jit_try_block`. JIT final state must equal
/// the interpreter's (self-validating regardless of the exact arithmetic).
//   loop: add eax,1 ; cmp eax,10 ; jl skip ; add ebx,10 ; skip: dec ecx ; jnz loop ; hlt
#[test]
fn jit_loop_with_internal_if_matches_interp() {
    let code: Vec<u8> = vec![
        0x83, 0xC0, 0x01, // add eax,1
        0x83, 0xF8, 0x0A, // cmp eax,10
        0x7C, 0x03, // jl skip (+3 -> skip)   (eax<10 -> skip the add)
        0x83, 0xC3, 0x0A, // add ebx,10  (only when eax>=10)
        0xFF, 0xC9, // skip: dec ecx
        0x75, 0xF1, // jnz loop (-15 -> head)
        0xF4, // hlt
    ];
    let setup = |v: &mut X86_64Vcpu| {
        let mut r = v.get_regs().unwrap();
        r.rax = 0;
        r.rcx = 20;
        r.rbx = 0;
        v.set_regs(&r).unwrap();
    };

    let mut jit = make_vcpu_code(&code);
    setup(&mut jit);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "a loop with an internal if should JIT natively"
    );
    run_interp(&mut jit); // execute the parked HLT
    let jr = jit.get_regs().unwrap();

    let mut interp = make_vcpu_code(&code);
    setup(&mut interp);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    assert_eq!(jr.rax & 0xffff_ffff, ir.rax & 0xffff_ffff, "eax");
    assert_eq!(
        jr.rbx & 0xffff_ffff,
        ir.rbx & 0xffff_ffff,
        "ebx (conditional accumulation)"
    );
    assert_eq!(jr.rcx & 0xffff_ffff, ir.rcx & 0xffff_ffff, "ecx");
    assert_eq!(jr.rax & 0xffff_ffff, 20, "ran all 20 iterations");
    assert_eq!(
        jr.rbx & 0xffff_ffff,
        110,
        "ebx += 10 for each eax>=10 (iterations 10..=20)"
    );
}

/// General-exit lowering: a hot loop that exits to a NON-HLT continuation runs
/// natively (back-edge internal) and hands control back to the interpreter at
/// the loop-exit address via an exit stub recording `exit_pc`. The native
/// result + resume PC must match the interpreter stepped to the same point.
#[test]
fn jit_general_exit_matches_interp_at_handoff() {
    use rax::smir::ir::Terminator;
    use rax::smir::lift::x86_64::X86_64Lifter;
    use rax::smir::lift::{LiftContext, MemoryReader, SmirLifter};
    use rax::smir::lower::SmirLowerer;
    use rax::smir::lower::runtime::{ExecMem, GuestRegs, is_native_clobber_safe};
    use rax::smir::lower::x86_64::X86_64Lowerer;
    use rax::smir::memory::MemoryError;
    use rax::smir::types::SourceArch;
    use std::collections::HashMap;

    // loop: add eax,2 ; dec ecx ; jnz loop      (exits to a continuation)
    // cont: mov ebx,0x7777 ; hlt
    let code: Vec<u8> = vec![
        0x83, 0xC0, 0x02, // add eax,2
        0xFF, 0xC9, // dec ecx
        0x75, 0xF9, // jnz loop (rel8 -7)
        0xBB, 0x77, 0x77, 0x00, 0x00, // mov ebx,0x7777  (continuation)
        0xF4, // hlt
    ];
    let cont_addr = LOAD_ADDR + 7;

    // --- JIT general-exit path ---
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
    let reader = Win {
        base: LOAD_ADDR,
        bytes: code.clone(),
    };
    let mut lifter = X86_64Lifter::strict();
    let mut lctx = LiftContext::new(SourceArch::X86_64);
    let func = lifter
        .lift_function(LOAD_ADDR, &reader, &mut lctx)
        .expect("lift_function");

    // Mark every "frontier" terminal (the JIT can't continue through it) as a
    // native-exit recording the block's guest_pc — the JIT runs up to but NOT
    // through it, so the interpreter resumes there and re-executes the block.
    let mut exits: HashMap<_, u64> = HashMap::new();
    for b in &func.blocks {
        let frontier = matches!(
            b.terminator,
            Terminator::Trap { .. }
                | Terminator::Return { .. }
                | Terminator::Call { .. }
                | Terminator::TailCall { .. }
                | Terminator::IndirectBranch { .. }
                | Terminator::IndirectBranchMem { .. }
                | Terminator::Switch { .. }
        );
        if frontier {
            exits.insert(b.id, b.guest_pc);
        }
    }
    assert!(!exits.is_empty(), "expected a frontier exit block");
    assert!(is_native_clobber_safe(&func), "loop must be clobber-safe");

    let mut lowerer = X86_64Lowerer::new();
    lowerer.set_native_exits(exits);
    let res = lowerer.lower_function(&func).expect("lower (native_exit)");
    assert!(res.relocations.is_empty());
    let bytes = lowerer.finalize().expect("finalize");
    let mem = ExecMem::new(&bytes).expect("ExecMem");

    let mut gr = GuestRegs::default();
    gr.gpr[0] = 0; // eax
    gr.gpr[1] = 10; // ecx (trip)
    gr.gpr[3] = 0x1111; // ebx (must NOT become 0x7777 — cont not executed)
    gr.rflags = 0x2;
    mem.run(res.entry_offset, &mut gr);

    // --- Interpreter, stepped to the same hand-off point ---
    let mut interp = make_vcpu_code(&code);
    let mut r = interp.get_regs().unwrap();
    r.rax = 0;
    r.rcx = 10;
    r.rbx = 0x1111;
    interp.set_regs(&r).unwrap();
    loop {
        if interp.get_regs().unwrap().rip == cont_addr {
            break;
        }
        match interp.step() {
            Ok(Some(VcpuExit::Hlt)) => panic!("interp hit HLT before the cont hand-off"),
            Ok(_) => {}
            Err(e) => panic!("interp error: {e:?}"),
        }
    }
    let ir = interp.get_regs().unwrap();

    assert_eq!(gr.exit_pc, cont_addr, "exit stub recorded the loop-exit PC");
    assert_eq!(gr.gpr[0] & 0xffff_ffff, ir.rax & 0xffff_ffff, "eax");
    assert_eq!(gr.gpr[1] & 0xffff_ffff, ir.rcx & 0xffff_ffff, "ecx");
    assert_eq!(
        gr.gpr[3] & 0xffff_ffff,
        ir.rbx & 0xffff_ffff,
        "ebx (continuation NOT run)"
    );
    assert_eq!(gr.gpr[0] & 0xffff_ffff, 20, "eax = 2*10");
    assert_eq!(
        gr.gpr[3] & 0xffff_ffff,
        0x1111,
        "ebx unchanged — exit block skipped"
    );
}

/// THE M5c GOAL: `run()` itself auto-detects the hot loop, compiles it, and
/// executes it natively — with no manual `jit_try_block` call — and produces the
/// correct architectural result. Proves the auto-trigger + cache + back-edge
/// hotness path end-to-end through the real run loop.
#[test]
fn run_auto_jits_hot_loop() {
    let n = 5000u32;
    let mut vcpu = make_vcpu(n);
    // run() services ~1ms slices and returns Hlt on the timer as well as on a
    // real guest HALT, so loop until the guest loop has actually drained
    // (ecx==0), bounded so a bug can't hang the test.
    let mut slices = 0;
    loop {
        let _ = vcpu.run().expect("run");
        slices += 1;
        if vcpu.get_regs().unwrap().rcx & 0xffff_ffff == 0 || slices > 10_000 {
            break;
        }
    }
    let r = vcpu.get_regs().unwrap();
    assert_eq!(r.rcx & 0xffff_ffff, 0, "the hot loop drained under run()");
    assert_eq!(
        r.rax & 0xffff_ffff,
        (2 * n as u64) & 0xffff_ffff,
        "eax = 2*n (correct result)"
    );
    // The auto-trigger must have fired: at least one region was compiled.
    assert!(
        vcpu.jit_region_count() >= 1,
        "run() should have auto-compiled the hot loop, got {} regions",
        vcpu.jit_region_count()
    );
}

/// SMC safety: a guest store to a code page that has a cached JIT region must
/// EVICT that region (so stale native code never runs) — essential for a kernel
/// that patches/loads code. Control case confirms the region would otherwise
/// stay cached.
#[test]
fn run_smc_evicts_jit_region() {
    // add eax,1 ; dec ecx ; jnz loop   (back-edge to LOAD_ADDR)
    let loop_bytes = [0x83, 0xC0, 0x01, 0xFF, 0xC9, 0x75, 0xF9];
    let mk = |store: bool| -> X86_64Vcpu {
        let mut code = loop_bytes.to_vec();
        if store {
            // mov rbx, LOAD_ADDR ; mov byte [rbx], 0x90   (self-modify code page)
            code.extend_from_slice(&[0x48, 0xBB]);
            code.extend_from_slice(&LOAD_ADDR.to_le_bytes());
            code.extend_from_slice(&[0xC6, 0x03, 0x90]);
        }
        code.push(0xF4); // hlt
        let mut v = make_vcpu_code(&code);
        let mut r = v.get_regs().unwrap();
        r.rcx = 500; // well past the 64-hit JIT threshold
        v.set_regs(&r).unwrap();
        v
    };
    let drive = |v: &mut X86_64Vcpu| {
        for _ in 0..10_000 {
            let _ = v.run().expect("run");
            if v.get_regs().unwrap().rcx & 0xffff_ffff == 0 {
                let _ = v.run().expect("run"); // let the continuation (store+hlt) finish
                break;
            }
        }
    };

    // Control: no self-modifying write — the compiled region remains cached.
    let mut a = mk(false);
    drive(&mut a);
    assert!(
        a.jit_region_count() >= 1,
        "control: the hot loop should compile and stay cached (got {})",
        a.jit_region_count()
    );

    // SMC: the guest store to the loop's code page must evict the region.
    let mut b = mk(true);
    drive(&mut b);
    assert_eq!(
        b.jit_region_count(),
        0,
        "a self-modifying store must evict the cached JIT region"
    );
}

/// Report JIT vs interpreter throughput on the same loop (informational).
#[test]
fn jit_throughput() {
    // Large count: the whole loop runs in ONE native call (internal back-edge).
    let big = 200_000_000u32;
    let mut jit = make_vcpu(big);
    let t = Instant::now();
    let ran = jit.jit_try_block().expect("jit_try_block");
    let dt = t.elapsed();
    assert!(ran, "the loop region should JIT");
    let executed = (big as u64) * 5 + 3; // matches bench_loop accounting
    let mips = executed as f64 / dt.as_secs_f64() / 1e6;
    let r = jit.get_regs().unwrap();
    println!(
        "[jit-vcpu] {} insns in {:.4}s => {:.1} MIPS  (final eax={:#x} ecx={:#x})",
        executed,
        dt.as_secs_f64(),
        mips,
        r.rax & 0xffff_ffff,
        r.rcx & 0xffff_ffff
    );
    assert_eq!(r.rax & 0xffff_ffff, (2 * big as u64) & 0xffff_ffff);
}

/// Build a vcpu loaded with `code`, returning the guest memory so a test can
/// seed/inspect scratch data. Same long-mode flat config as `make_vcpu_code`.
fn make_vcpu_mem(code: &[u8]) -> (X86_64Vcpu, Arc<GuestMemoryMmap>) {
    let region = MmapRegion::new(MEM_SIZE as usize).unwrap();
    let guest_region = GuestRegionMmap::new(region, GuestAddress(0)).unwrap();
    let memory = Arc::new(GuestMemoryMmap::from_regions(vec![guest_region]).unwrap());
    memory.write_slice(code, GuestAddress(LOAD_ADDR)).unwrap();

    let mut regs = Registers::default();
    regs.rip = LOAD_ADDR;
    regs.rsp = 0x11_0000;
    regs.rflags = 0x2;
    let mut sregs = SystemRegisters::default();
    sregs.cr0 = 0x21;
    sregs.cr4 = 0x20;
    sregs.efer = 0x500;
    sregs.cs.base = 0;
    sregs.cs.limit = 0xFFFFFFFF;
    sregs.cs.selector = 0x8;
    sregs.cs.type_ = 0xB;
    sregs.cs.present = true;
    sregs.cs.s = true;
    sregs.cs.l = true;
    sregs.cs.g = true;
    sregs.ds.base = 0;
    sregs.ds.limit = 0xFFFFFFFF;
    sregs.ds.selector = 0x10;
    sregs.ds.type_ = 0x3;
    sregs.ds.present = true;
    sregs.ds.db = true;
    sregs.ds.s = true;
    sregs.ds.g = true;
    sregs.es = sregs.ds.clone();
    sregs.fs = sregs.ds.clone();
    sregs.gs = sregs.ds.clone();
    sregs.ss = sregs.ds.clone();

    let mut vcpu = X86_64Vcpu::new(0, memory.clone());
    vcpu.set_regs(&regs).unwrap();
    vcpu.set_sregs(&sregs).unwrap();
    (vcpu, memory)
}

/// Memory-operand JIT (RAX_JIT_MEM path): a loop that LOADS from a scratch array
/// and STORES each element into a second array runs natively via the MMU helper
/// calls and reproduces the interpreter's GPRs AND memory bit-exact.
#[test]
fn jit_mem_load_store_loop_matches_interpreter() {
    const SCRATCH: u64 = 0x20_0000;
    const DST: u64 = 0x20_0040;
    const COUNT: u32 = 4;

    // mov ecx,COUNT ; mov rbx,SCRATCH ;
    // loop: mov rax,[rbx] ; mov [rbx+0x40],rax ; add rbx,8 ; dec ecx ; jnz loop ; hlt
    let mut code: Vec<u8> = Vec::new();
    code.push(0xB9);
    code.extend_from_slice(&COUNT.to_le_bytes()); // mov ecx, COUNT
    code.extend_from_slice(&[0x48, 0xBB]);
    code.extend_from_slice(&SCRATCH.to_le_bytes()); // mov rbx, SCRATCH (movabs)
    code.extend_from_slice(&[0x48, 0x8B, 0x03]); // mov rax, [rbx]
    code.extend_from_slice(&[0x48, 0x89, 0x43, 0x40]); // mov [rbx+0x40], rax
    code.extend_from_slice(&[0x48, 0x83, 0xC3, 0x08]); // add rbx, 8
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xF1]); // jnz loop (rel8 = -15)
    code.push(0xF4); // hlt

    let seed: [u64; 4] = [0x1111_2222_3333_4444, 0xAAAA_BBBB_CCCC_DDDD, 7, 0xDEAD_BEEF];
    let setup = |mem: &Arc<GuestMemoryMmap>| {
        for (i, &val) in seed.iter().enumerate() {
            mem.write_obj(val, GuestAddress(SCRATCH + (i as u64) * 8))
                .unwrap();
            mem.write_obj(0u64, GuestAddress(DST + (i as u64) * 8))
                .unwrap();
        }
    };

    // Interpreter reference.
    let (mut interp, imem) = make_vcpu_mem(&code);
    setup(&imem);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();
    let mut idst = [0u64; 4];
    for (i, slot) in idst.iter_mut().enumerate() {
        *slot = imem.read_obj(GuestAddress(DST + (i as u64) * 8)).unwrap();
    }

    // JIT with memory operands enabled.
    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    setup(&jmem);
    let ran = jit.jit_try_block().expect("jit_try_block");
    assert!(ran, "the memory loop region should JIT (RAX_JIT_MEM path)");
    run_interp(&mut jit); // step the parked HLT
    let jr = jit.get_regs().unwrap();
    let mut jdst = [0u64; 4];
    for (i, slot) in jdst.iter_mut().enumerate() {
        *slot = jmem.read_obj(GuestAddress(DST + (i as u64) * 8)).unwrap();
    }

    assert_eq!(jr.rax, ir.rax, "rax");
    assert_eq!(jr.rbx, ir.rbx, "rbx");
    assert_eq!(jr.rcx, ir.rcx, "rcx");
    assert_eq!(jdst, idst, "stored array (jit) must match interpreter");
    assert_eq!(jdst, seed, "DST should equal the seed after the copy loop");
    assert_eq!(jr.rbx, SCRATCH + 4 * 8, "rbx walked the array");
}

/// memset-shaped loop: the loop-count flag is set by `dec` BEFORE the stores,
/// and `jnz` reads it AFTER them — so a store that clobbers the flags makes the
/// branch always-taken and the loop overruns. Reproduces the kernel `__memset`
/// region the JIT crashed on. Must terminate and match the interpreter exactly.
#[test]
fn jit_mem_memset_loop_matches_interpreter() {
    const SCRATCH: u64 = 0x20_0000;
    const N: u32 = 5;

    // mov rcx,N ; mov rdi,SCRATCH ; mov rax,0x4242...
    // loop: dec rcx ; mov [rdi],rax ; mov [rdi+8],rax ; lea rdi,[rdi+16] ; jnz loop ; hlt
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xC7, 0xC1]);
    code.extend_from_slice(&N.to_le_bytes()); // mov rcx, N
    code.extend_from_slice(&[0x48, 0xBF]);
    code.extend_from_slice(&SCRATCH.to_le_bytes()); // mov rdi, SCRATCH
    code.extend_from_slice(&[0x48, 0xB8]);
    code.extend_from_slice(&0x4242_4242_4242_4242u64.to_le_bytes()); // mov rax, imm
    // loop body (14 bytes): dec rcx(3) ; mov[rdi]rax(3) ; mov[rdi+8]rax(4) ; lea rdi,[rdi+16](4)
    code.extend_from_slice(&[0x48, 0xFF, 0xC9]); // dec rcx
    code.extend_from_slice(&[0x48, 0x89, 0x07]); // mov [rdi], rax
    code.extend_from_slice(&[0x48, 0x89, 0x47, 0x08]); // mov [rdi+8], rax
    code.extend_from_slice(&[0x48, 0x8D, 0x7F, 0x10]); // lea rdi, [rdi+16]
    code.extend_from_slice(&[0x75, 0xF0]); // jnz loop (rel8 = -16)
    code.push(0xF4); // hlt

    let val = 0x4242_4242_4242_4242u64;

    let (mut interp, _imem) = make_vcpu_mem(&code);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    let ran = jit.jit_try_block().expect("jit_try_block");
    assert!(ran, "memset loop should JIT");
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    assert_eq!(
        jr.rcx, ir.rcx,
        "rcx (loop count) — overrun if flags clobbered"
    );
    assert_eq!(jr.rcx, 0, "loop ran exactly N times");
    assert_eq!(jr.rdi, ir.rdi, "rdi");
    assert_eq!(jr.rdi, SCRATCH + (N as u64) * 16, "rdi walked N*16 bytes");
    // The N*2 stored slots are `val`; the slot just past must be untouched (0).
    for i in 0..(N as u64) * 2 {
        let got: u64 = jmem.read_obj(GuestAddress(SCRATCH + i * 8)).unwrap();
        assert_eq!(got, val, "slot {i} stored");
    }
    let past: u64 = jmem
        .read_obj(GuestAddress(SCRATCH + (N as u64) * 16))
        .unwrap();
    assert_eq!(past, 0, "no overrun past the memset region");
}

/// SIB addressing: a copy loop that loads `[rbx + rsi*8]` and stores
/// `[rdi + rsi*8]` exercises the JIT memory path's BaseIndexScale address
/// computation (the most complex addressing mode). Must match the interpreter.
#[test]
fn jit_mem_indexed_copy_loop_matches_interpreter() {
    const SRC: u64 = 0x20_0000;
    const DST: u64 = 0x21_0000;
    const N: u32 = 6;

    // mov rbx,SRC ; mov rdi,DST ; mov ecx,N ; xor esi,esi
    // loop: mov rax,[rbx+rsi*8] ; mov [rdi+rsi*8],rax ; inc rsi ; dec ecx ; jnz loop ; hlt
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xBB]);
    code.extend_from_slice(&SRC.to_le_bytes()); // mov rbx, SRC
    code.extend_from_slice(&[0x48, 0xBF]);
    code.extend_from_slice(&DST.to_le_bytes()); // mov rdi, DST
    code.push(0xB9);
    code.extend_from_slice(&N.to_le_bytes()); // mov ecx, N
    code.extend_from_slice(&[0x31, 0xF6]); // xor esi, esi
    // loop body (13 bytes):
    code.extend_from_slice(&[0x48, 0x8B, 0x04, 0xF3]); // mov rax, [rbx+rsi*8]
    code.extend_from_slice(&[0x48, 0x89, 0x04, 0xF7]); // mov [rdi+rsi*8], rax
    code.extend_from_slice(&[0x48, 0xFF, 0xC6]); // inc rsi
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xF1]); // jnz loop (rel8 = -15)
    code.push(0xF4); // hlt

    let seed: [u64; 6] = [10, 20, 30, 40, 50, 60];
    let setup = |mem: &Arc<GuestMemoryMmap>| {
        for (i, &v) in seed.iter().enumerate() {
            mem.write_obj(v, GuestAddress(SRC + (i as u64) * 8))
                .unwrap();
            mem.write_obj(0u64, GuestAddress(DST + (i as u64) * 8))
                .unwrap();
        }
    };

    let (mut interp, imem) = make_vcpu_mem(&code);
    setup(&imem);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    setup(&jmem);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "indexed copy should JIT"
    );
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    assert_eq!(jr.rcx, ir.rcx, "rcx");
    assert_eq!(jr.rsi, ir.rsi, "rsi (index)");
    for i in 0..6u64 {
        let got: u64 = jmem.read_obj(GuestAddress(DST + i * 8)).unwrap();
        assert_eq!(got, seed[i as usize], "DST[{i}] via SIB copy");
    }
}

/// Partial-width register stores (B1/B2/B4/B8) and store-immediate forms — the
/// shapes kernel struct/array writers (e.g. text_poke batch entries) use. Each
/// must write exactly `size` bytes via the MMU helper; must match the
/// interpreter's memory image byte-for-byte.
#[test]
fn jit_mem_partial_and_imm_stores_match_interpreter() {
    const DST: u64 = 0x21_0000;
    const STRIDE: u64 = 0x20;
    const N: u32 = 2;
    let rax_val = 0xAABB_CCDD_EEFF_1122u64;

    // mov rdi,DST ; mov rax,rax_val ; mov ecx,N
    // loop:
    //   mov [rdi],al ; mov [rdi+1],ax ; mov [rdi+4],eax ; mov [rdi+8],rax ;
    //   mov byte [rdi+16],0x55 ; mov dword [rdi+20],0x12345678 ;
    //   add rdi,0x20 ; dec ecx ; jnz loop ; hlt
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xBF]);
    code.extend_from_slice(&DST.to_le_bytes());
    code.extend_from_slice(&[0x48, 0xB8]);
    code.extend_from_slice(&rax_val.to_le_bytes());
    code.push(0xB9);
    code.extend_from_slice(&N.to_le_bytes());
    // loop body (30 bytes):
    code.extend_from_slice(&[0x88, 0x07]); // mov [rdi], al
    code.extend_from_slice(&[0x66, 0x89, 0x47, 0x01]); // mov [rdi+1], ax
    code.extend_from_slice(&[0x89, 0x47, 0x04]); // mov [rdi+4], eax
    code.extend_from_slice(&[0x48, 0x89, 0x47, 0x08]); // mov [rdi+8], rax
    code.extend_from_slice(&[0xC6, 0x47, 0x10, 0x55]); // mov byte [rdi+16], 0x55
    code.extend_from_slice(&[0xC7, 0x47, 0x14, 0x78, 0x56, 0x34, 0x12]); // mov dword [rdi+20], 0x12345678
    code.extend_from_slice(&[0x48, 0x83, 0xC7, 0x20]); // add rdi, 0x20
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xE0]); // jnz loop (rel8 = -32)
    code.push(0xF4); // hlt

    let (mut interp, imem) = make_vcpu_mem(&code);
    for i in 0..(N as u64) * STRIDE {
        imem.write_obj(0u8, GuestAddress(DST + i)).unwrap();
    }
    run_interp(&mut interp);

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    for i in 0..(N as u64) * STRIDE {
        jmem.write_obj(0u8, GuestAddress(DST + i)).unwrap();
    }
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "partial-store loop should JIT"
    );
    run_interp(&mut jit);

    // Compare the whole written region byte-for-byte against the interpreter.
    for i in 0..(N as u64) * STRIDE {
        let ib: u8 = imem.read_obj(GuestAddress(DST + i)).unwrap();
        let jb: u8 = jmem.read_obj(GuestAddress(DST + i)).unwrap();
        assert_eq!(jb, ib, "byte at DST+{i:#x}: jit={jb:#x} interp={ib:#x}");
    }
    // Spot-check the expected pattern in the first record.
    assert_eq!(jmem.read_obj::<u8>(GuestAddress(DST)).unwrap(), 0x22, "al");
    assert_eq!(
        jmem.read_obj::<u16>(GuestAddress(DST + 1)).unwrap(),
        0x1122,
        "ax"
    );
    assert_eq!(
        jmem.read_obj::<u32>(GuestAddress(DST + 4)).unwrap(),
        0xEEFF_1122,
        "eax"
    );
    assert_eq!(
        jmem.read_obj::<u64>(GuestAddress(DST + 8)).unwrap(),
        rax_val,
        "rax"
    );
    assert_eq!(
        jmem.read_obj::<u8>(GuestAddress(DST + 16)).unwrap(),
        0x55,
        "imm8"
    );
    assert_eq!(
        jmem.read_obj::<u32>(GuestAddress(DST + 20)).unwrap(),
        0x1234_5678,
        "imm32"
    );
}

/// RIP-relative memory access — the addressing mode kernel code uses for static
/// globals (e.g. the text_poke batch array). The JIT must resolve the absolute
/// guest target at lift time. Must match the interpreter.
#[test]
fn jit_mem_riprel_store_loop_matches_interpreter() {
    const SCRATCH: u64 = 0x20_0000;
    let val = 0x1000u64;

    // mov rax,val ; mov ecx,3
    // loop: mov [rip+disp], rax ; add rax,1 ; dec ecx ; jnz loop ; hlt
    // The mov [rip+disp],rax is `48 89 05 <disp32>`; disp = SCRATCH - next_insn.
    // Layout: mov rax(10) + mov ecx(5) = 15 = loop start; the RIP-rel mov is 7
    // bytes (15..22), so next_insn (guest) = LOAD_ADDR + 22.
    let next_insn = LOAD_ADDR + 22;
    let disp = (SCRATCH as i64 - next_insn as i64) as i32;

    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xB8]);
    code.extend_from_slice(&val.to_le_bytes()); // mov rax, val
    code.push(0xB9);
    code.extend_from_slice(&3u32.to_le_bytes()); // mov ecx, 3
    code.extend_from_slice(&[0x48, 0x89, 0x05]); // mov [rip+disp], rax
    code.extend_from_slice(&disp.to_le_bytes());
    code.extend_from_slice(&[0x48, 0x83, 0xC0, 0x01]); // add rax, 1
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xF1]); // jnz loop (rel8 = -15)
    code.push(0xF4); // hlt

    let (mut interp, imem) = make_vcpu_mem(&code);
    imem.write_obj(0u64, GuestAddress(SCRATCH)).unwrap();
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();
    let iv: u64 = imem.read_obj(GuestAddress(SCRATCH)).unwrap();

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    jmem.write_obj(0u64, GuestAddress(SCRATCH)).unwrap();
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "rip-rel loop should JIT"
    );
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();
    let jv: u64 = jmem.read_obj(GuestAddress(SCRATCH)).unwrap();

    assert_eq!(jr.rax, ir.rax, "rax");
    assert_eq!(jv, iv, "RIP-relative store target (jit vs interp)");
    assert_eq!(jv, val + 2, "last stored value (val, val+1, val+2)");
}

/// SIB with scale=1 AND a (negative) displacement into an extended register —
/// `mov r9, [rsi + rdx*1 - 16]` — the exact shape the kernel's memmove uses that
/// the JIT verifier flagged. Must match the interpreter.
#[test]
fn jit_mem_sib_scale1_disp_matches_interpreter() {
    const SRC: u64 = 0x20_0000;
    const DST: u64 = 0x21_0000;
    let v = 0x0000_0000_0000_2000u64; // the value at SRC + (rdx - 16)

    // mov rsi,SRC ; mov rdx,0x40 ; mov rdi,DST ; mov ecx,1
    // loop: mov r9,[rsi+rdx*1-16] ; mov [rdi],r9 ; dec ecx ; jnz loop ; hlt
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xBE]);
    code.extend_from_slice(&SRC.to_le_bytes());
    code.extend_from_slice(&[0x48, 0xBA]);
    code.extend_from_slice(&0x40u64.to_le_bytes());
    code.extend_from_slice(&[0x48, 0xBF]);
    code.extend_from_slice(&DST.to_le_bytes());
    code.push(0xB9);
    code.extend_from_slice(&1u32.to_le_bytes());
    code.extend_from_slice(&[0x4C, 0x8B, 0x4C, 0x16, 0xF0]); // mov r9, [rsi+rdx*1-16]
    code.extend_from_slice(&[0x4C, 0x89, 0x0F]); // mov [rdi], r9
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xF4]); // jnz loop (rel8 = -12)
    code.push(0xF4); // hlt

    // SRC + (rdx=0x40) - 16 = SRC + 0x30.
    let setup = |mem: &Arc<GuestMemoryMmap>| {
        mem.write_obj(v, GuestAddress(SRC + 0x30)).unwrap();
        mem.write_obj(0u64, GuestAddress(DST)).unwrap();
    };

    let (mut interp, imem) = make_vcpu_mem(&code);
    setup(&imem);
    run_interp(&mut interp);
    let iv: u64 = imem.read_obj(GuestAddress(DST)).unwrap();

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    setup(&jmem);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "sib-disp loop should JIT"
    );
    run_interp(&mut jit);
    let jv: u64 = jmem.read_obj(GuestAddress(DST)).unwrap();

    assert_eq!(jv, iv, "SIB scale=1 + disp load (jit vs interp)");
    assert_eq!(jv, v, "loaded [rsi+rdx-16] = SRC[0x30]");
}

/// Extended registers (r8-r15) as memory-JIT load destinations AND store
/// sources — the registers the kernel's memmove uses (r8-r11), which none of
/// the other tests exercise. A REX miscoding in spill/reload/deliver would
/// corrupt them. Must match the interpreter byte-for-byte.
#[test]
fn jit_mem_extended_regs_copy_matches_interpreter() {
    const SRC: u64 = 0x20_0000;
    const DST: u64 = 0x21_0000;
    const N: u32 = 4;

    // mov rbx,SRC ; mov rdi,DST ; mov ecx,N
    // loop: mov r9,[rbx] ; mov r11,[rbx+8] ; mov [rdi],r9 ; mov [rdi+8],r11 ;
    //       add rbx,16 ; add rdi,16 ; dec ecx ; jnz loop ; hlt
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xBB]);
    code.extend_from_slice(&SRC.to_le_bytes());
    code.extend_from_slice(&[0x48, 0xBF]);
    code.extend_from_slice(&DST.to_le_bytes());
    code.push(0xB9);
    code.extend_from_slice(&N.to_le_bytes());
    // loop body (18 bytes):
    code.extend_from_slice(&[0x4C, 0x8B, 0x0B]); // mov r9, [rbx]
    code.extend_from_slice(&[0x4C, 0x8B, 0x5B, 0x08]); // mov r11, [rbx+8]
    code.extend_from_slice(&[0x4C, 0x89, 0x0F]); // mov [rdi], r9
    code.extend_from_slice(&[0x4C, 0x89, 0x5F, 0x08]); // mov [rdi+8], r11
    code.extend_from_slice(&[0x48, 0x83, 0xC3, 0x10]); // add rbx, 16
    code.extend_from_slice(&[0x48, 0x83, 0xC7, 0x10]); // add rdi, 16
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xE6]); // jnz loop (rel8 = -26, body is 24 bytes)
    code.push(0xF4); // hlt

    let seed: [u64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let setup = |mem: &Arc<GuestMemoryMmap>| {
        for (i, &x) in seed.iter().enumerate() {
            mem.write_obj(x, GuestAddress(SRC + (i as u64) * 8))
                .unwrap();
            mem.write_obj(0u64, GuestAddress(DST + (i as u64) * 8))
                .unwrap();
        }
    };

    let (mut interp, imem) = make_vcpu_mem(&code);
    setup(&imem);
    run_interp(&mut interp);

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    setup(&jmem);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "r8-15 copy should JIT"
    );
    run_interp(&mut jit);

    for i in 0..8u64 {
        let ib: u64 = imem.read_obj(GuestAddress(DST + i * 8)).unwrap();
        let jb: u64 = jmem.read_obj(GuestAddress(DST + i * 8)).unwrap();
        assert_eq!(jb, ib, "DST[{i}] jit vs interp");
        assert_eq!(jb, seed[i as usize], "DST[{i}] == seed (r8-15 copy)");
    }
}

/// OVERLAPPING backwards memmove: dst = src + 8, copied high-to-low so each
/// read precedes the overwrite of that location. This is the read-after-write
/// memmove shape (a load reading a just-stored, overlapping address) that the
/// kernel boot region used and that simple copies don't exercise. The JIT must
/// match the interpreter.
#[test]
fn jit_mem_overlapping_memmove_matches_interpreter() {
    const SRC: u64 = 0x20_0000;
    const N: u32 = 4; // elements

    // rsi = SRC+0x20 ; rdi = SRC+0x28 ; ecx = N
    // loop: mov rax,[rsi-8]; mov [rdi-8],rax; sub rsi,8; sub rdi,8; dec ecx; jnz loop; hlt
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xBE]);
    code.extend_from_slice(&(SRC + 0x20).to_le_bytes()); // mov rsi, SRC+0x20
    code.extend_from_slice(&[0x48, 0xBF]);
    code.extend_from_slice(&(SRC + 0x28).to_le_bytes()); // mov rdi, SRC+0x28
    code.push(0xB9);
    code.extend_from_slice(&N.to_le_bytes()); // mov ecx, N
    // loop body (18 bytes):
    code.extend_from_slice(&[0x48, 0x8B, 0x46, 0xF8]); // mov rax, [rsi-8]
    code.extend_from_slice(&[0x48, 0x89, 0x47, 0xF8]); // mov [rdi-8], rax
    code.extend_from_slice(&[0x48, 0x83, 0xEE, 0x08]); // sub rsi, 8
    code.extend_from_slice(&[0x48, 0x83, 0xEF, 0x08]); // sub rdi, 8
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xEC]); // jnz loop (rel8 = -20)
    code.push(0xF4); // hlt

    let seed: [u64; 5] = [10, 20, 30, 40, 50];
    let setup = |mem: &Arc<GuestMemoryMmap>| {
        for (i, &x) in seed.iter().enumerate() {
            mem.write_obj(x, GuestAddress(SRC + (i as u64) * 8))
                .unwrap();
        }
    };

    let (mut interp, imem) = make_vcpu_mem(&code);
    setup(&imem);
    run_interp(&mut interp);

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    setup(&jmem);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "overlapping memmove should JIT"
    );
    run_interp(&mut jit);

    for i in 0..5u64 {
        let ib: u64 = imem.read_obj(GuestAddress(SRC + i * 8)).unwrap();
        let jb: u64 = jmem.read_obj(GuestAddress(SRC + i * 8)).unwrap();
        assert_eq!(jb, ib, "SRC[{i}] jit vs interp (overlapping memmove)");
    }
    // Expected memmove(SRC+8, SRC, 32): [10, 10, 20, 30, 40].
    assert_eq!(
        jmem.read_obj::<u64>(GuestAddress(SRC + 8)).unwrap(),
        10,
        "moved[1]"
    );
    assert_eq!(
        jmem.read_obj::<u64>(GuestAddress(SRC + 32)).unwrap(),
        40,
        "moved[4]"
    );
}

/// Exact reconstruction of the kernel-boot memmove block-0: a 32-byte backwards
/// copy loop whose counter is `sub rdx,32` (CF/`jae`), with EIGHT memory ops
/// (4 loads + 4 stores via r8-r11) between the flag-set and the branch. This is
/// the region the JIT verifier flagged; the flags must survive all 8 mem ops so
/// the CF-based loop count matches the interpreter.
#[test]
fn jit_mem_boot_memcpy_block0_matches_interpreter() {
    const SRC: u64 = 0x20_0000;
    const DST: u64 = 0x21_0000;
    const BYTES: u64 = 0x80; // rdx start; loop copies 32 at a time

    // mov rsi,SRC+BYTES ; mov rdi,DST+BYTES ; mov rdx,BYTES
    // loop: sub rdx,32 ; mov r11,[rsi-8]; r10,[rsi-16]; r9,[rsi-24]; r8,[rsi-32];
    //       lea rsi,[rsi-32]; mov [rdi-8],r11; [rdi-16],r10; [rdi-24],r9; [rdi-32],r8;
    //       lea rdi,[rdi-32]; jae loop ; hlt
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xBE]);
    code.extend_from_slice(&(SRC + BYTES).to_le_bytes());
    code.extend_from_slice(&[0x48, 0xBF]);
    code.extend_from_slice(&(DST + BYTES).to_le_bytes());
    code.extend_from_slice(&[0x48, 0xBA]);
    code.extend_from_slice(&BYTES.to_le_bytes());
    // block 0 (44 bytes), bytes copied verbatim from the boot region dump:
    code.extend_from_slice(&[0x48, 0x83, 0xEA, 0x20]); // sub rdx, 32
    code.extend_from_slice(&[0x4C, 0x8B, 0x5E, 0xF8]); // mov r11, [rsi-8]
    code.extend_from_slice(&[0x4C, 0x8B, 0x56, 0xF0]); // mov r10, [rsi-16]
    code.extend_from_slice(&[0x4C, 0x8B, 0x4E, 0xE8]); // mov r9, [rsi-24]
    code.extend_from_slice(&[0x4C, 0x8B, 0x46, 0xE0]); // mov r8, [rsi-32]
    code.extend_from_slice(&[0x48, 0x8D, 0x76, 0xE0]); // lea rsi, [rsi-32]
    code.extend_from_slice(&[0x4C, 0x89, 0x5F, 0xF8]); // mov [rdi-8], r11
    code.extend_from_slice(&[0x4C, 0x89, 0x57, 0xF0]); // mov [rdi-16], r10
    code.extend_from_slice(&[0x4C, 0x89, 0x4F, 0xE8]); // mov [rdi-24], r9
    code.extend_from_slice(&[0x4C, 0x89, 0x47, 0xE0]); // mov [rdi-32], r8
    code.extend_from_slice(&[0x48, 0x8D, 0x7F, 0xE0]); // lea rdi, [rdi-32]
    code.extend_from_slice(&[0x73, 0xD2]); // jae loop (rel8 = -46)
    code.push(0xF4); // hlt

    let setup = |mem: &Arc<GuestMemoryMmap>| {
        for i in 0..(BYTES / 8) {
            mem.write_obj(0x1000 + i, GuestAddress(SRC + i * 8))
                .unwrap();
            mem.write_obj(0u64, GuestAddress(DST + i * 8)).unwrap();
        }
    };

    let (mut interp, imem) = make_vcpu_mem(&code);
    setup(&imem);
    run_interp(&mut interp);
    let ir = interp.get_regs().unwrap();

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    setup(&jmem);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "block0 memcpy should JIT"
    );
    run_interp(&mut jit);
    let jr = jit.get_regs().unwrap();

    assert_eq!(jr.rdx, ir.rdx, "rdx (CF-based loop counter)");
    assert_eq!(jr.rsi, ir.rsi, "rsi");
    assert_eq!(jr.rdi, ir.rdi, "rdi");
    for i in 0..(BYTES / 8) {
        let ib: u64 = imem.read_obj(GuestAddress(DST + i * 8)).unwrap();
        let jb: u64 = jmem.read_obj(GuestAddress(DST + i * 8)).unwrap();
        assert_eq!(jb, ib, "DST[{i}] jit vs interp");
    }
}

/// Boot block-0 with the EXACT overlap from the failing kernel region:
/// dst = src + 24, length rdx = 0x130 (non-32-multiple). A backwards 32-byte
/// copy within one overlapping buffer — the read-after-write geometry the
/// non-overlapping block-0 test doesn't reach. JIT must match the interpreter.
#[test]
fn jit_mem_boot_memmove_overlap24_matches_interpreter() {
    const BASE: u64 = 0x20_0000;
    const LEN: u64 = 0x130;
    let rsi0 = BASE + 0x200; // src end (high), copy proceeds downward
    let rdi0 = rsi0 + 0x18; // dst end = src + 24 (overlap)

    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0xBE]);
    code.extend_from_slice(&rsi0.to_le_bytes()); // mov rsi, rsi0
    code.extend_from_slice(&[0x48, 0xBF]);
    code.extend_from_slice(&rdi0.to_le_bytes()); // mov rdi, rdi0
    code.extend_from_slice(&[0x48, 0xBA]);
    code.extend_from_slice(&LEN.to_le_bytes()); // mov rdx, LEN
    code.extend_from_slice(&[0x48, 0x83, 0xEA, 0x20]); // sub rdx, 32
    code.extend_from_slice(&[0x4C, 0x8B, 0x5E, 0xF8]); // mov r11, [rsi-8]
    code.extend_from_slice(&[0x4C, 0x8B, 0x56, 0xF0]); // mov r10, [rsi-16]
    code.extend_from_slice(&[0x4C, 0x8B, 0x4E, 0xE8]); // mov r9, [rsi-24]
    code.extend_from_slice(&[0x4C, 0x8B, 0x46, 0xE0]); // mov r8, [rsi-32]
    code.extend_from_slice(&[0x48, 0x8D, 0x76, 0xE0]); // lea rsi, [rsi-32]
    code.extend_from_slice(&[0x4C, 0x89, 0x5F, 0xF8]); // mov [rdi-8], r11
    code.extend_from_slice(&[0x4C, 0x89, 0x57, 0xF0]); // mov [rdi-16], r10
    code.extend_from_slice(&[0x4C, 0x89, 0x4F, 0xE8]); // mov [rdi-24], r9
    code.extend_from_slice(&[0x4C, 0x89, 0x47, 0xE0]); // mov [rdi-32], r8
    code.extend_from_slice(&[0x48, 0x8D, 0x7F, 0xE0]); // lea rdi, [rdi-32]
    code.extend_from_slice(&[0x73, 0xD2]); // jae loop
    code.push(0xF4); // hlt

    let setup = |mem: &Arc<GuestMemoryMmap>| {
        for i in 0..0x60u64 {
            mem.write_obj(0x100000 + i, GuestAddress(BASE + i * 8))
                .unwrap();
        }
    };

    let (mut interp, imem) = make_vcpu_mem(&code);
    setup(&imem);
    run_interp(&mut interp);

    let (mut jit, jmem) = make_vcpu_mem(&code);
    jit.set_jit_mem(true);
    setup(&jmem);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "overlap-24 memmove should JIT"
    );
    run_interp(&mut jit);

    for i in 0..0x60u64 {
        let ib: u64 = imem.read_obj(GuestAddress(BASE + i * 8)).unwrap();
        let jb: u64 = jmem.read_obj(GuestAddress(BASE + i * 8)).unwrap();
        assert_eq!(jb, ib, "buf[{i}] jit vs interp (overlap-24 memmove)");
    }
}

/// A long byte-copy loop (`mov al,[rsi]; mov [rdi],al; inc rdi; inc rsi; dec rcx;
/// jne`) — the kernel `memcpy` byte tail at region 0x8214929f. Each iteration is
/// TWO mem-helper calls; checks the host stack stays balanced over many
/// iterations (RSP drift would corrupt the host stack → crash) and that RSP +
/// the copied bytes match the interpreter.
#[test]
fn jit_mem_byte_copy_loop_long() {
    const SRC: u64 = 0x20_0000;
    const DST: u64 = 0x40_0000;
    const N: u64 = 0x1000;
    let code: &[u8] = &[
        0x8a, 0x06, // mov al, [rsi]
        0x88, 0x07, // mov [rdi], al
        0x48, 0xff, 0xc7, // inc rdi
        0x48, 0xff, 0xc6, // inc rsi
        0x48, 0xff, 0xc9, // dec rcx
        0x75, 0xf3, // jne loop (-13)
        0xf4, // hlt
    ];
    let setup = |v: &mut X86_64Vcpu, mem: &Arc<GuestMemoryMmap>| {
        for i in 0..N {
            mem.write_obj(
                (i as u8).wrapping_mul(7).wrapping_add(3),
                GuestAddress(SRC + i),
            )
            .unwrap();
        }
        let mut r = v.get_regs().unwrap();
        r.rsi = SRC;
        r.rdi = DST;
        r.rcx = N;
        v.set_regs(&r).unwrap();
    };

    let (mut interp, im) = make_vcpu_mem(code);
    setup(&mut interp, &im);
    run_interp(&mut interp);

    let (mut jit, jm) = make_vcpu_mem(code);
    setup(&mut jit, &jm);
    jit.set_jit_mem(true);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "region should JIT"
    );
    run_interp(&mut jit);

    assert_eq!(
        jit.get_regs().unwrap().rsp,
        interp.get_regs().unwrap().rsp,
        "RSP drift"
    );
    assert_eq!(jit.get_regs().unwrap().rcx, 0, "rcx");
    for i in 0..N {
        let ib: u8 = im.read_obj(GuestAddress(DST + i)).unwrap();
        let jb: u8 = jm.read_obj(GuestAddress(DST + i)).unwrap();
        assert_eq!(jb, ib, "DST[{i}]");
    }
}

// FS/GS segment-relative memory operands (the `64`/`65` prefixes) must JIT
// CORRECTLY: the effective address is `segment_base + base + index*scale + disp`,
// where the base comes from the FS/GS descriptor / IA32_FS_BASE/GS_BASE MSR
// (`sregs.fs.base`/`sregs.gs.base`), lifted as `Address::SegmentRel`. These are
// the kernel's per-CPU (`gs:`) and TLS (`fs:`) accesses. Each test uses a
// NON-ZERO segment base, places the correct value at `seg_base+addr` AND a
// distinct SENTINEL at the un-segmented `addr`, so a JIT that dropped the
// segment base would read the sentinel and diverge from the interpreter.
fn seg_jit_vs_interp(
    code: &[u8],
    fs_base: u64,
    gs_base: u64,
    setup: impl Fn(&mut X86_64Vcpu, &Arc<GuestMemoryMmap>),
) -> (
    X86_64Vcpu,
    Arc<GuestMemoryMmap>,
    X86_64Vcpu,
    Arc<GuestMemoryMmap>,
) {
    let prep = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        let mut s = v.get_sregs().unwrap();
        s.fs.base = fs_base;
        s.gs.base = gs_base;
        v.set_sregs(&s).unwrap();
        setup(v, m);
    };
    let (mut interp, im) = make_vcpu_mem(code);
    prep(&mut interp, &im);
    run_interp(&mut interp);

    let (mut jit, jm) = make_vcpu_mem(code);
    prep(&mut jit, &jm);
    jit.set_jit_mem(true);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "FS/GS-relative region MUST now JIT (Address::SegmentRel)"
    );
    run_interp(&mut jit);
    (interp, im, jit, jm)
}

const GSB: u64 = 0x50_0000;
const FSB: u64 = 0x60_0000;

// Each test wraps the segment op in a 1-iteration loop (`dec <ctr>; jne head`)
// so the region's entry block ends in a back-edge (not a frontier) and the JIT
// actually compiles + runs the op. The counter is a register the op doesn't use.

/// `mov rax, gs:[rbx+8]` — base + disp, GS base added.
#[test]
fn jit_mem_gs_relative_base_disp() {
    // loop: mov rax, gs:[rbx+8]; dec rcx; jne loop; hlt
    let code: &[u8] = &[
        0x65, 0x48, 0x8b, 0x43, 0x08, 0x48, 0xff, 0xc9, 0x75, 0xf6, 0xf4,
    ];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0xCAFEu64, GuestAddress(GSB + 0x1008)).unwrap(); // gs.base+rbx+8
        m.write_obj(0xBADu64, GuestAddress(0x1008)).unwrap(); // sentinel (no gs base)
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x1000;
        r.rcx = 1;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0xCAFE,
        "must read [gs.base+rbx+8], not sentinel"
    );
}

/// `mov rax, gs:[0x1234]` — disp-only (the kernel `this_cpu` per-CPU pattern).
#[test]
fn jit_mem_gs_relative_disp_only() {
    // loop: mov rax, gs:[0x1234]; dec rcx; jne loop; hlt
    let code: &[u8] = &[
        0x65, 0x48, 0x8b, 0x04, 0x25, 0x34, 0x12, 0x00, 0x00, 0x48, 0xff, 0xc9, 0x75, 0xf2, 0xf4,
    ];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0xDEADu64, GuestAddress(GSB + 0x1234)).unwrap();
        m.write_obj(0xBADu64, GuestAddress(0x1234)).unwrap();
        let mut r = v.get_regs().unwrap();
        r.rcx = 1;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0xDEAD,
        "must read [gs.base+0x1234]"
    );
}

/// `mov rax, fs:[rbx]` — FS base added (TLS).
#[test]
fn jit_mem_fs_relative() {
    // loop: mov rax, fs:[rbx]; dec rcx; jne loop; hlt
    let code: &[u8] = &[0x64, 0x48, 0x8b, 0x03, 0x48, 0xff, 0xc9, 0x75, 0xf7, 0xf4];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0xF00Du64, GuestAddress(FSB + 0x800)).unwrap();
        m.write_obj(0xBADu64, GuestAddress(0x800)).unwrap();
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x800;
        r.rcx = 1;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, FSB, 0, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0xF00D,
        "must read [fs.base+rbx]"
    );
}

/// `mov gs:[rbx], rax` — STORE to a GS-relative address.
#[test]
fn jit_mem_gs_relative_store() {
    // loop: mov gs:[rbx], rax; dec rcx; jne loop; hlt
    let code: &[u8] = &[0x65, 0x48, 0x89, 0x03, 0x48, 0xff, 0xc9, 0x75, 0xf7, 0xf4];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0u64, GuestAddress(GSB + 0x900)).unwrap();
        m.write_obj(0u64, GuestAddress(0x900)).unwrap();
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x900;
        r.rcx = 1;
        r.rax = 0x1234_5678_9ABC_DEF0;
        v.set_regs(&r).unwrap();
    };
    let (_interp, _im, _jit, jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    let stored: u64 = jm.read_obj(GuestAddress(GSB + 0x900)).unwrap();
    assert_eq!(
        stored, 0x1234_5678_9ABC_DEF0,
        "store must hit [gs.base+rbx]"
    );
    let sentinel: u64 = jm.read_obj(GuestAddress(0x900)).unwrap();
    assert_eq!(sentinel, 0, "store must NOT hit the un-segmented address");
}

/// `mov rax, gs:[rbx+rcx*8]` — base + index*scale + GS base (full SIB form).
/// Uses RDX as the loop counter (RCX is the index).
#[test]
fn jit_mem_gs_relative_index_scale() {
    // loop: mov rax, gs:[rbx+rcx*8]; dec rdx; jne loop; hlt
    let code: &[u8] = &[
        0x65, 0x48, 0x8b, 0x04, 0xcb, 0x48, 0xff, 0xca, 0x75, 0xf6, 0xf4,
    ];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        // rbx=0x1000, rcx=3 → gs.base + 0x1000 + 3*8 = gs.base + 0x1018
        m.write_obj(0xBEEFu64, GuestAddress(GSB + 0x1018)).unwrap();
        m.write_obj(0xBADu64, GuestAddress(0x1018)).unwrap();
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x1000;
        r.rcx = 3;
        r.rdx = 1;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0xBEEF,
        "must read [gs.base+rbx+rcx*8]"
    );
}

/// `mov al, gs:[rbx]` (B1) — a partial-register write: x86 `mov r8, r/m8`
/// preserves the upper 56 bits of RAX, it does NOT zero-extend. The kernel's
/// per-CPU byte reads rely on this; a JIT that zero-extended would corrupt the
/// upper bits (the exact boot divergence `rax: interp=0x80010000 jit=0x0`).
#[test]
fn jit_mem_gs_relative_byte_partial_write() {
    // loop: mov al, gs:[rbx]; dec rcx; jne loop; hlt
    let code: &[u8] = &[0x65, 0x8a, 0x03, 0x48, 0xff, 0xc9, 0x75, 0xf8, 0xf4];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0x42u8, GuestAddress(GSB + 0x700)).unwrap();
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x700;
        r.rcx = 1;
        r.rax = 0xDEAD_BEEF_0000_0000;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0xDEAD_BEEF_0000_0042,
        "mov al must preserve the upper 56 bits of RAX (partial write)"
    );
}

/// `mov ax, gs:[rbx]` (B2) — partial-register write preserving the upper 48 bits.
#[test]
fn jit_mem_gs_relative_word_partial_write() {
    // loop: mov ax, gs:[rbx]; dec rcx; jne loop; hlt  (65=GS, 66=opsize)
    let code: &[u8] = &[0x65, 0x66, 0x8b, 0x03, 0x48, 0xff, 0xc9, 0x75, 0xf7, 0xf4];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0x1234u16, GuestAddress(GSB + 0x780)).unwrap();
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x780;
        r.rcx = 1;
        r.rax = 0xFFFF_FFFF_FFFF_FFFF;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0xFFFF_FFFF_FFFF_1234,
        "mov ax must preserve the upper 48 bits of RAX (partial write)"
    );
}

/// `mov eax, gs:[rbx]` (B4) — a 32-bit write ZERO-EXTENDS to 64 bits (clears the
/// upper 32 of RAX), unlike B1/B2. Confirms the deliver path's width semantics.
#[test]
fn jit_mem_gs_relative_dword_zero_extend() {
    // loop: mov eax, gs:[rbx]; dec rcx; jne loop; hlt
    let code: &[u8] = &[0x65, 0x8b, 0x03, 0x48, 0xff, 0xc9, 0x75, 0xf8, 0xf4];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0x1234_5678u32, GuestAddress(GSB + 0x680))
            .unwrap();
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x680;
        r.rcx = 1;
        r.rax = 0xFFFF_FFFF_FFFF_FFFF;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0x0000_0000_1234_5678,
        "mov eax must zero-extend (clear upper 32 of RAX)"
    );
}

/// `mov rax, gs:[rbx-8]` — negative displacement, GS base added.
#[test]
fn jit_mem_gs_relative_negative_disp() {
    // loop: mov rax, gs:[rbx-8]; dec rcx; jne loop; hlt
    let code: &[u8] = &[
        0x65, 0x48, 0x8b, 0x43, 0xf8, 0x48, 0xff, 0xc9, 0x75, 0xf6, 0xf4,
    ];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0xC0DEu64, GuestAddress(GSB + 0x1000 - 8))
            .unwrap(); // gs.base + rbx - 8
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x1000;
        r.rcx = 1;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0xC0DE,
        "must read [gs.base+rbx-8]"
    );
}

/// `mov fs:[rbx+rcx*4+0x10], rax` — FS store with base + index*scale + disp.
#[test]
fn jit_mem_fs_relative_store_index_disp() {
    // loop: mov fs:[rbx+rcx*4+0x10], rax; dec rdx; jne loop; hlt
    let code: &[u8] = &[
        0x64, 0x48, 0x89, 0x44, 0x8b, 0x10, 0x48, 0xff, 0xca, 0x75, 0xf5, 0xf4,
    ];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        // rbx=0x100, rcx=2 → fs.base + 0x100 + 2*4 + 0x10 = fs.base + 0x118
        m.write_obj(0u64, GuestAddress(FSB + 0x118)).unwrap();
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x100;
        r.rcx = 2;
        r.rdx = 1;
        r.rax = 0xABCD_1234_5678_9ABC;
        v.set_regs(&r).unwrap();
    };
    let (_interp, _im, _jit, jm) = seg_jit_vs_interp(code, FSB, 0, setup);
    let stored: u64 = jm.read_obj(GuestAddress(FSB + 0x118)).unwrap();
    assert_eq!(
        stored, 0xABCD_1234_5678_9ABC,
        "store must hit [fs.base+rbx+rcx*4+0x10]"
    );
}

/// `lea rax, fs:[rbx]` — LEA computes the OFFSET and must IGNORE the segment
/// override: it yields rbx, NOT fs.base+rbx, even with a non-zero fs.base.
/// (End-to-end guard for the LEA-adds-segment-base bug the audit found.)
#[test]
fn jit_mem_lea_ignores_segment() {
    // loop: lea rax, fs:[rbx]; dec rcx; jne loop; hlt
    let code: &[u8] = &[0x64, 0x48, 0x8d, 0x03, 0x48, 0xff, 0xc9, 0x75, 0xf7, 0xf4];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        let _ = m;
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x1234;
        r.rcx = 1;
        v.set_regs(&r).unwrap();
    };
    let (interp, _im, jit, _jm) = seg_jit_vs_interp(code, FSB, 0, setup);
    assert_eq!(
        jit.get_regs().unwrap().rax,
        interp.get_regs().unwrap().rax,
        "rax jit vs interp"
    );
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0x1234,
        "LEA must ignore fs.base (yield the offset rbx)"
    );
}

/// `mov gs:[rbx+rcx*4], rax` — GS STORE with base + index*scale.
#[test]
fn jit_mem_gs_relative_store_index_scale() {
    // loop: mov gs:[rbx+rcx*4], rax; dec rdx; jne loop; hlt  (rdx counter; rcx is index)
    let code: &[u8] = &[
        0x65, 0x48, 0x89, 0x04, 0x8b, 0x48, 0xff, 0xca, 0x75, 0xf6, 0xf4,
    ];
    let setup = |v: &mut X86_64Vcpu, m: &Arc<GuestMemoryMmap>| {
        m.write_obj(0u64, GuestAddress(GSB + 0x1008)).unwrap(); // gs.base + 0x1000 + 2*4
        let mut r = v.get_regs().unwrap();
        r.rbx = 0x1000;
        r.rcx = 2;
        r.rdx = 1;
        r.rax = 0xCAFE_BABE_DEAD_BEEF;
        v.set_regs(&r).unwrap();
    };
    let (_interp, _im, _jit, jm) = seg_jit_vs_interp(code, 0, GSB, setup);
    let stored: u64 = jm.read_obj(GuestAddress(GSB + 0x1008)).unwrap();
    assert_eq!(
        stored, 0xCAFE_BABE_DEAD_BEEF,
        "store must hit [gs.base+rbx+rcx*4]"
    );
}

/// `movzx ecx, dil` (REX-prefixed `40 0f b6 cf`) wedged BETWEEN two loads, as in
/// kernel region 0x82149bd0. The lifter must not drop the movzx — if it does,
/// rcx keeps a stale value and the dependent indexed load reads a wrong address.
#[test]
fn jit_mem_movzx_dil_between_loads() {
    const PTR: u64 = 0x20_0000;
    const DATA: u64 = 0x21_0000;
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0x8b, 0x02]); // mov rax, [rdx]
    code.extend_from_slice(&[0x40, 0x0f, 0xb6, 0xcf]); // movzx ecx, dil
    code.extend_from_slice(&[0x8b, 0x04, 0x88]); // mov eax, [rax+rcx*4]
    code.extend_from_slice(&[0x48, 0xff, 0xce]); // dec rsi
    code.extend_from_slice(&[0x75, 0xf1]); // jne loop (-15)
    code.push(0xf4); // hlt

    let setup = |v: &mut X86_64Vcpu, mem: &Arc<GuestMemoryMmap>| {
        mem.write_obj(DATA, GuestAddress(PTR)).unwrap();
        mem.write_obj(0x8580u32, GuestAddress(DATA + 4)).unwrap(); // [DATA + dil*4], dil=1
        let mut r = v.get_regs().unwrap();
        r.rdx = PTR;
        r.rdi = 1; // dil = 1 → rcx should become 1
        r.rcx = 0x3c; // stale value that must be overwritten by the movzx
        r.rsi = 1;
        v.set_regs(&r).unwrap();
    };

    let (mut interp, im) = make_vcpu_mem(&code);
    setup(&mut interp, &im);
    run_interp(&mut interp);

    let (mut jit, jm) = make_vcpu_mem(&code);
    setup(&mut jit, &jm);
    jit.set_jit_mem(true);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "region should JIT"
    );
    run_interp(&mut jit);

    assert_eq!(interp.get_regs().unwrap().rcx, 1, "interp rcx (movzx dil)");
    assert_eq!(
        jit.get_regs().unwrap().rcx,
        1,
        "jit rcx (movzx must not be dropped)"
    );
    assert_eq!(interp.get_regs().unwrap().rax, 0x8580, "interp rax");
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0x8580,
        "jit rax (depends on rcx=1)"
    );
}

/// BaseIndexScale loads with scale=4 and 32-bit (B4) width — the shape in the
/// kernel region 0x82149bd0 that my scale=1/scale=8 tests didn't cover. Also a
/// dependent pair (load a pointer, then index off it), which 0x82149bd0 uses.
#[test]
fn jit_mem_baseindexscale_scale4_b4_and_dependent() {
    const PTR: u64 = 0x20_0000; // holds a pointer
    const DATA: u64 = 0x21_0000; // pointed-to table

    // mov rax,[rdx]; mov eax,[rax+rcx*4]; dec rsi; jne loop; hlt
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x48, 0x8b, 0x02]); // mov rax, [rdx]
    code.extend_from_slice(&[0x8b, 0x04, 0x88]); // mov eax, [rax+rcx*4]  (B4, scale 4)
    code.extend_from_slice(&[0x48, 0xff, 0xce]); // dec rsi
    code.extend_from_slice(&[0x75, 0xf6]); // jne loop (-10)
    code.push(0xf4); // hlt

    let setup = |v: &mut X86_64Vcpu, mem: &Arc<GuestMemoryMmap>| {
        mem.write_obj(DATA, GuestAddress(PTR)).unwrap(); // [rdx] = DATA pointer
        mem.write_obj(0x8580u32, GuestAddress(DATA + 8)).unwrap(); // [DATA + rcx*4], rcx=2
        let mut r = v.get_regs().unwrap();
        r.rdx = PTR;
        r.rcx = 2;
        r.rsi = 1; // one iteration
        r.rax = 0xDEAD;
        v.set_regs(&r).unwrap();
    };

    let (mut interp, im) = make_vcpu_mem(&code);
    setup(&mut interp, &im);
    run_interp(&mut interp);

    let (mut jit, jm) = make_vcpu_mem(&code);
    setup(&mut jit, &jm);
    jit.set_jit_mem(true);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "region should JIT"
    );
    run_interp(&mut jit);

    assert_eq!(interp.get_regs().unwrap().rax, 0x8580, "interp rax");
    assert_eq!(
        jit.get_regs().unwrap().rax,
        0x8580,
        "jit rax (scale4/B4 dependent load)"
    );
}

/// Reconstruction of kernel region 0x8173fd10 (a memchr/scan helper) which the
/// verifier flagged with a flags divergence: the path reaches a frontier exit
/// right after `xor ecx,ecx`, so the JIT must materialize the xor's flags
/// (ZF+PF = 0x44) at the exit, not the preceding cmp's flags. Checks the
/// flag state at the frontier exit, where the bug lives (later ops overwrite it).
#[test]
fn jit_mem_xor_flags_at_frontier_exit() {
    const BUF: u64 = 0x30_0000;
    // Verbatim region bytes from the boot at 0xffffffff8173fd10.
    let region: &[u8] = &[
        0x48, 0x85, 0xf6, 0x74, 0x0d, 0x48, 0x8b, 0x07, 0x48, 0x83, 0xf8, 0xff, 0x74, 0x0a, 0x31,
        0xc9, 0xeb, 0x22, 0x31, 0xf6, 0x48, 0x89, 0xf0, 0xc3, 0x48, 0x83, 0xc7, 0x08, 0x31, 0xc9,
        0x48, 0x83, 0xc1, 0x40, 0x48, 0x39, 0xf1, 0x73, 0xed, 0x48, 0x8b, 0x07, 0x48, 0x83, 0xc7,
        0x08, 0x48, 0x83, 0xf8, 0xff, 0x74, 0xea, 0x48, 0xf7, 0xd0, 0xf3, 0x48, 0x0f, 0xbc, 0xc0,
        0x48, 0x01, 0xc8, 0x48, 0x39, 0xf0, 0x48, 0x0f, 0x43, 0xc6, 0xc3,
    ];
    const MASK: u64 = 0x0ED5;
    const STACK: u64 = 0x18_0000;
    const RET_HLT: u64 = LOAD_ADDR + 0x200;

    let setup = |v: &mut X86_64Vcpu, mem: &Arc<GuestMemoryMmap>| {
        mem.write_obj(0x12345u64, GuestAddress(BUF)).unwrap(); // [rdi] != -1 → xor path
        mem.write_obj(0xF4u8, GuestAddress(RET_HLT)).unwrap(); // hlt at the ret target
        mem.write_obj(RET_HLT, GuestAddress(STACK + 8)).unwrap(); // [rsp+8] (after add rsp,8; ret)
        let mut r = v.get_regs().unwrap();
        r.rsi = 0x400;
        r.rdi = BUF;
        r.rbx = BUF;
        r.rax = 0x7e;
        r.rcx = 0x7e;
        r.rdx = 0x40;
        r.rsp = STACK;
        v.set_regs(&r).unwrap();
    };

    // End-to-end: the JIT runs block0→2→4 then hands off at the block-5 frontier;
    // the interpreter resumes into block 5, which overwrites every flag before
    // reading it. So although the JIT's flags differ from the interpreter's AT
    // the frontier (the benign dead-flag artifact: xor's ZF+PF vs the eliminated
    // result), the FINAL architectural state — registers AND flags — must match.
    let (mut interp, _im) = make_vcpu_mem(region);
    setup(&mut interp, &_im);
    run_interp(&mut interp);

    let (mut jit, jm) = make_vcpu_mem(region);
    setup(&mut jit, &jm);
    jit.set_jit_mem(true);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "region should JIT"
    );
    run_interp(&mut jit);

    let ir = interp.get_regs().unwrap();
    let jr = jit.get_regs().unwrap();
    assert_eq!(jr.rax, ir.rax, "final rax");
    assert_eq!(jr.rcx, ir.rcx, "final rcx");
    assert_eq!(jr.rdx, ir.rdx, "final rdx");
    assert_eq!(jr.rsi, ir.rsi, "final rsi");
    assert_eq!(
        jr.rflags & MASK,
        ir.rflags & MASK,
        "final flags must match once block 5 overwrites the dead frontier flags"
    );
}

/// Minimal reproduction of the kernel-memmove tail bug: two `[rsi+rdx*1+disp]`
/// loads differing ONLY in displacement (-16, -8) into r9/r8, then two matching
/// stores. The JIT must lift BOTH loads — dropping `mov r8,[rsi+rdx-8]` while
/// keeping `mov [rdi+rdx-8],r8` stores a stale r8 (the boot memmove corruption).
#[test]
fn jit_mem_two_baseindexscale_loads_distinct_disp() {
    const SRC: u64 = 0x20_0000;
    const DST: u64 = 0x21_0000;
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0x4c, 0x8b, 0x4c, 0x16, 0xf0]); // mov r9, [rsi+rdx*1-16]
    code.extend_from_slice(&[0x4c, 0x8b, 0x44, 0x16, 0xf8]); // mov r8, [rsi+rdx*1-8]
    code.extend_from_slice(&[0x4c, 0x89, 0x4c, 0x17, 0xf0]); // mov [rdi+rdx*1-16], r9
    code.extend_from_slice(&[0x4c, 0x89, 0x44, 0x17, 0xf8]); // mov [rdi+rdx*1-8], r8
    code.extend_from_slice(&[0x48, 0xff, 0xc9]); // dec rcx
    code.extend_from_slice(&[0x75, 0xe7]); // jne loop (-25 → LOAD_ADDR)
    code.push(0xf4); // hlt

    let setup = |v: &mut X86_64Vcpu, mem: &Arc<GuestMemoryMmap>| {
        mem.write_obj(0x1111u64, GuestAddress(SRC + 0x10)).unwrap();
        mem.write_obj(0x2222u64, GuestAddress(SRC + 0x18)).unwrap();
        mem.write_obj(0u64, GuestAddress(DST + 0x10)).unwrap();
        mem.write_obj(0u64, GuestAddress(DST + 0x18)).unwrap();
        let mut r = v.get_regs().unwrap();
        r.rsi = SRC + 0x10;
        r.rdi = DST + 0x10;
        r.rdx = 0x10;
        r.rcx = 1;
        r.r8 = 0xDEAD; // sentinel: surfaces if the r8 load is dropped
        r.r9 = 0xBEEF;
        v.set_regs(&r).unwrap();
    };

    let (mut interp, imem) = make_vcpu_mem(&code);
    setup(&mut interp, &imem);
    run_interp(&mut interp);

    let (mut jit, jmem) = make_vcpu_mem(&code);
    setup(&mut jit, &jmem);
    jit.set_jit_mem(true);
    assert!(
        jit.jit_try_block().expect("jit_try_block"),
        "region should JIT"
    );
    run_interp(&mut jit);

    for (off, want) in [(0x10u64, 0x1111u64), (0x18, 0x2222)] {
        let iv: u64 = imem.read_obj(GuestAddress(DST + off)).unwrap();
        let jv: u64 = jmem.read_obj(GuestAddress(DST + off)).unwrap();
        assert_eq!(iv, want, "interp DST[{off:#x}]");
        assert_eq!(jv, want, "jit DST[{off:#x}] (stale r8 => dropped load)");
    }
}

/// Verbatim reconstruction of the kernel `__memset` region (0x82151000 in the
/// boot) with its captured entry state: rcx=1 (64-byte chunks), rdx=0x59 (89
/// bytes total → tail of 1 byte), rax=0 (fill). The JIT verifier flagged this
/// region; this test runs a FRESH interpreter vs the JIT to determine which is
/// correct (89 bytes should be zeroed by both).
#[test]
fn jit_mem_kernel_memset_region_matches_interpreter() {
    const BUF: u64 = 0x20_0000;
    const RET_HLT: u64 = LOAD_ADDR + 0x180;

    // Exact region bytes captured from the boot at 0xffffffff82151000.
    let region: &[u8] = &[
        0x48, 0xff, 0xc9, 0x48, 0x89, 0x07, 0x48, 0x89, 0x47, 0x08, 0x48, 0x89, 0x47, 0x10, 0x48,
        0x89, 0x47, 0x18, 0x48, 0x89, 0x47, 0x20, 0x48, 0x89, 0x47, 0x28, 0x48, 0x89, 0x47, 0x30,
        0x48, 0x89, 0x47, 0x38, 0x48, 0x8d, 0x7f, 0x40, 0x75, 0xd8, 0x0f, 0x1f, 0x84, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x89, 0xd1, 0x83, 0xe1, 0x38, 0x74, 0x14, 0xc1, 0xe9, 0x03, 0x66, 0x0f,
        0x1f, 0x44, 0x00, 0x00, 0xff, 0xc9, 0x48, 0x89, 0x07, 0x48, 0x8d, 0x7f, 0x08, 0x75, 0xf5,
        0x83, 0xe2, 0x07, 0x74, 0x0a, 0xff, 0xca, 0x88, 0x07, 0x48, 0x8d, 0x7f, 0x01, 0x75, 0xf6,
        0x4c, 0x89, 0xd0, 0xc3,
    ];

    let build = |fill_dst: bool| -> (X86_64Vcpu, Arc<GuestMemoryMmap>) {
        let (mut v, mem) = make_vcpu_mem(region);
        // hlt at the return target; stack returns there after the memset's ret.
        mem.write_obj(0xF4u8, GuestAddress(RET_HLT)).unwrap();
        let rsp = 0x18_0000u64;
        mem.write_obj(RET_HLT, GuestAddress(rsp)).unwrap();
        if fill_dst {
            for i in 0..0x80u64 {
                mem.write_obj(0xFFu8, GuestAddress(BUF + i)).unwrap();
            }
        }
        let mut r = v.get_regs().unwrap();
        r.rax = 0; // fill value
        r.rcx = 1; // 64-byte chunk count
        r.rdx = 0x59; // 89 bytes total
        r.rdi = BUF; // dst
        r.rsp = rsp;
        v.set_regs(&r).unwrap();
        (v, mem)
    };

    let (mut interp, imem) = build(true);
    run_interp(&mut interp);

    let (mut jit, jmem) = build(true);
    jit.set_jit_mem(true);
    // The region's hot 64-byte loop may not auto-promote in one shot; drive it.
    let _ = jit.jit_try_block();
    run_interp(&mut jit);

    // memset(BUF, 0, 89): bytes 0..89 zeroed, byte 89 (0x59) untouched (0xFF).
    for i in 0..0x80u64 {
        let ib: u8 = imem.read_obj(GuestAddress(BUF + i)).unwrap();
        let jb: u8 = jmem.read_obj(GuestAddress(BUF + i)).unwrap();
        assert_eq!(jb, ib, "BUF[{i}] jit vs interp");
        let expect = if i < 0x59 { 0u8 } else { 0xFFu8 };
        assert_eq!(jb, expect, "BUF[{i}] memset(89) result");
    }
}
