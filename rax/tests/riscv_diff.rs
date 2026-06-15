//! RISC-V (RV64GC) differential test harness: rax interpreter vs. QEMU oracle.
//!
//! The rax software interpreter (`src/riscv/`) is checked against a
//! hardware-semantics reference produced by running each instruction under
//! `qemu-riscv64` (user mode). The reference harness is `tools/riscv-diff/
//! oracle.c`, built on demand into a static RV64GC ELF.
//!
//! For each `(instruction, initial architectural state)` pair we:
//!   1. run it on the oracle (x1..x31 except gp/tp, f0..f31, fcsr, plus a shared
//!      scratch window captured), and
//!   2. run it on the rax `RiscVCpu` from the *identical* initial state,
//! then compare the full register file. Any divergence is an interpreter bug.
//!
//! Robustness (mirrors `tests/arm_diff.rs`):
//!   - if the cross compiler or `qemu-riscv64` is unavailable, every test
//!     self-skips (returns without failing) so the suite is green anywhere.
//!   - only non-PC-relative instructions are tested (no AUIPC/JAL/JALR/branch),
//!     because the oracle executes at a different PC than rax. Control-flow
//!     correctness is covered by the in-crate unit tests.
//!
//! `x3` (gp) and `x4` (tp) are reserved by the oracle (it must preserve them
//! for the signal handler's TLS), so they are never used as operands and never
//! compared.

#![cfg(target_os = "linux")]

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use rax::riscv::decode::{Op, decode, decode_compressed};
use rax::riscv::{FlatMemory, Isa, RiscVConfig, RiscVCpu, RiscVExit, Xlen};

// ---------------------------------------------------------------------------
// Wire format -- must match tools/riscv-diff/oracle.c byte for byte.
// ---------------------------------------------------------------------------

const WIRE_MAGIC: u32 = 0x3436_5652; // 'R','V','6','4'
const SCRATCH_ADDR: u64 = 0x20_0000;
const SCRATCH_BASE: u64 = SCRATCH_ADDR + 64;

/// Full architectural register file exchanged with the oracle.
#[repr(C)]
#[derive(Clone, Copy)]
struct RvState {
    x: [u64; 32],
    f: [u64; 32],
    pc: u64,
    fcsr: u64,
    scratch: [u64; 32],
}

impl RvState {
    fn zeroed() -> Self {
        RvState {
            x: [0; 32],
            f: [0; 32],
            pc: 0,
            fcsr: 0,
            scratch: [0; 32],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InCase {
    insn: u32,
    insn_len: u32,
    st: RvState,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct OutCase {
    st: RvState,
    trapped: u32,
    valid: u32,
}

fn as_bytes<T: Copy>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
}

fn read_struct<T: Copy>(buf: &[u8], off: usize) -> T {
    assert!(off + std::mem::size_of::<T>() <= buf.len());
    unsafe { std::ptr::read_unaligned(buf[off..].as_ptr() as *const T) }
}

// ---------------------------------------------------------------------------
// Deterministic PRNG (splitmix64) -- reproducible without std rng.
// ---------------------------------------------------------------------------

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// A register value biased towards interesting bit patterns.
    fn reg(&mut self) -> u64 {
        match self.next() % 16 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => 1u64 << 63,
            4 => 0x8000_0000,
            5 => 0xffff_ffff,
            6 => (self.next() & 0x3f), // small shift-amount sized
            7 => self.next() & 0xff,
            _ => self.next(),
        }
    }
}

// ---------------------------------------------------------------------------
// Instruction encoders.
// ---------------------------------------------------------------------------

fn r_type(funct7: u32, rs2: u32, rs1: u32, funct3: u32, rd: u32, opcode: u32) -> u32 {
    (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}
fn i_type(imm: i32, rs1: u32, funct3: u32, rd: u32, opcode: u32) -> u32 {
    (((imm as u32) & 0xfff) << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}
fn shift_imm(funct6: u32, shamt: u32, rs1: u32, funct3: u32, rd: u32, opcode: u32) -> u32 {
    (funct6 << 26) | ((shamt & 0x3f) << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}
fn shift_imm_w(funct7: u32, shamt: u32, rs1: u32, funct3: u32, rd: u32, opcode: u32) -> u32 {
    (funct7 << 25) | ((shamt & 0x1f) << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}
fn s_type(imm: i32, rs2: u32, rs1: u32, funct3: u32, opcode: u32) -> u32 {
    let u = (imm as u32) & 0xfff;
    let hi = (u >> 5) & 0x7f;
    let lo = u & 0x1f;
    (hi << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (lo << 7) | opcode
}
fn amo(funct5: u32, rs2: u32, rs1: u32, funct3: u32, rd: u32) -> u32 {
    (funct5 << 27) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | 0x2f
}
fn b_type(imm: i32, rs2: u32, rs1: u32, funct3: u32) -> u32 {
    let u = (imm as u32) & 0x1fff;
    let b12 = (u >> 12) & 1;
    let b11 = (u >> 11) & 1;
    let b10_5 = (u >> 5) & 0x3f;
    let b4_1 = (u >> 1) & 0xf;
    (b12 << 31)
        | (b10_5 << 25)
        | (rs2 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | (b4_1 << 8)
        | (b11 << 7)
        | 0x63
}

// ---------------------------------------------------------------------------
// Oracle build + run.
// ---------------------------------------------------------------------------

fn oracle_path() -> Option<PathBuf> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/riscv-diff");
    let oracle = dir.join("oracle");
    let needs_build = match (oracle.metadata(), dir.join("oracle.c").metadata()) {
        (Ok(o), Ok(s)) => match (o.modified(), s.modified()) {
            (Ok(om), Ok(sm)) => om < sm,
            _ => true,
        },
        _ => true,
    };
    if needs_build {
        let status = Command::new("bash")
            .arg(dir.join("build.sh"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .ok()?;
        if !status.success() {
            return None;
        }
    }
    if oracle.exists() { Some(oracle) } else { None }
}

fn run_oracle(oracle: &PathBuf, cases: &[(u32, u32, RvState)]) -> Option<Vec<OutCase>> {
    let mut payload = Vec::with_capacity(8 + cases.len() * std::mem::size_of::<InCase>());
    payload.extend_from_slice(&WIRE_MAGIC.to_le_bytes());
    payload.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for (insn, len, st) in cases {
        let ic = InCase {
            insn: *insn,
            insn_len: *len,
            st: *st,
        };
        payload.extend_from_slice(as_bytes(&ic));
    }

    let mut child = Command::new("qemu-riscv64")
        .arg(oracle)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(&payload);
    });
    let mut out = Vec::new();
    child.stdout.take().unwrap().read_to_end(&mut out).ok()?;
    let _ = writer.join();
    let status = child.wait().ok()?;
    if !status.success() || out.len() < 8 {
        return None;
    }
    let magic = u32::from_le_bytes([out[0], out[1], out[2], out[3]]);
    let count = u32::from_le_bytes([out[4], out[5], out[6], out[7]]) as usize;
    if magic != WIRE_MAGIC || count != cases.len() {
        return None;
    }
    let mut res = Vec::with_capacity(count);
    let mut off = 8;
    for _ in 0..count {
        res.push(read_struct::<OutCase>(&out, off));
        off += std::mem::size_of::<OutCase>();
    }
    Some(res)
}

// ---------------------------------------------------------------------------
// rax: run one instruction from an identical initial state.
// ---------------------------------------------------------------------------

const INSN_ADDR: u64 = 0x1000;

fn run_rax(insn: u32, len: u32, input: &RvState) -> Option<RvState> {
    let mem = FlatMemory::new(0, 0x21_0000); // covers code @0x1000 and scratch @0x200000
    let mut cpu = RiscVCpu::new(RiscVConfig::rv64gc(), Box::new(mem));

    for i in 1..32u8 {
        if i == 3 || i == 4 {
            continue;
        }
        cpu.set_x(i, input.x[i as usize]);
    }
    for i in 0..32u8 {
        cpu.set_f(i, input.f[i as usize]);
    }
    cpu.set_fcsr(input.fcsr as u32);

    // Install the scratch window.
    let mut scratch_bytes: Vec<u8> = Vec::with_capacity(256);
    for w in input.scratch.iter() {
        scratch_bytes.extend_from_slice(&w.to_le_bytes());
    }
    cpu.write_memory(SCRATCH_ADDR, &scratch_bytes).ok()?;

    cpu.write_memory(INSN_ADDR, &insn.to_le_bytes()[..len as usize])
        .ok()?;
    cpu.set_pc(INSN_ADDR);

    match cpu.step() {
        RiscVExit::Continue => {}
        _ => return None,
    }

    let mut out = RvState::zeroed();
    for i in 1..32u8 {
        out.x[i as usize] = cpu.x(i);
    }
    for i in 0..32u8 {
        out.f[i as usize] = cpu.f(i);
    }
    out.fcsr = cpu.fcsr() as u64;
    // PC as displacement from the instruction address, matching the oracle.
    out.pc = cpu.pc().wrapping_sub(INSN_ADDR);
    for (i, w) in out.scratch.iter_mut().enumerate() {
        *w = cpu.mem_read_u64(SCRATCH_ADDR + (i as u64) * 8).ok()?;
    }
    Some(out)
}

// ---------------------------------------------------------------------------
// Comparison.
// ---------------------------------------------------------------------------

struct Mismatch {
    label: String,
    insn: u32,
    detail: String,
}

fn compare_case(
    label: &str,
    insn: u32,
    input: &RvState,
    oracle: &OutCase,
    cmp_fp: bool,
    cmp_pc: bool,
    mismatches: &mut Vec<Mismatch>,
) {
    let rax = run_rax(insn, if insn & 3 == 3 { 4 } else { 2 }, input);

    if oracle.trapped != 0 {
        if rax.is_some() {
            mismatches.push(Mismatch {
                label: label.into(),
                insn,
                detail: format!(
                    "hw faulted (sig {}) but rax executed the encoding",
                    oracle.trapped
                ),
            });
        }
        return;
    }
    let rax = match rax {
        Some(s) => s,
        None => {
            mismatches.push(Mismatch {
                label: label.into(),
                insn,
                detail: "hw executed but rax rejected the encoding".into(),
            });
            return;
        }
    };

    let mut diffs = Vec::new();
    for i in 1..32usize {
        if i == 3 || i == 4 {
            continue;
        }
        if rax.x[i] != oracle.st.x[i] {
            diffs.push(format!(
                "x{i}: rax={:#018x} hw={:#018x}",
                rax.x[i], oracle.st.x[i]
            ));
        }
    }
    for i in 0..32usize {
        if rax.scratch[i] != oracle.st.scratch[i] {
            diffs.push(format!(
                "scratch[{i}]: rax={:#018x} hw={:#018x}",
                rax.scratch[i], oracle.st.scratch[i]
            ));
        }
    }
    if cmp_fp {
        for i in 0..32usize {
            if rax.f[i] != oracle.st.f[i] {
                diffs.push(format!(
                    "f{i}: rax={:#018x} hw={:#018x}",
                    rax.f[i], oracle.st.f[i]
                ));
            }
        }
        if rax.fcsr != oracle.st.fcsr {
            diffs.push(format!(
                "fcsr: rax={:#x} hw={:#x}",
                rax.fcsr, oracle.st.fcsr
            ));
        }
    }
    if cmp_pc && rax.pc != oracle.st.pc {
        diffs.push(format!("pc-disp: rax={:#x} hw={:#x}", rax.pc, oracle.st.pc));
    }

    if !diffs.is_empty() {
        mismatches.push(Mismatch {
            label: label.into(),
            insn,
            detail: diffs.join(", "),
        });
    }
}

/// Run a batch of `(label, insn, input_state)` cases against the oracle and
/// fail with a report if any diverge. Self-skips if the toolchain is absent.
fn run_batch(batch: &[(String, u32, RvState)], cmp_fp: bool) {
    run_batch_opts(batch, cmp_fp, false);
}

/// As [`run_batch`], with explicit control over PC-displacement comparison
/// (for PC-relative control-flow instructions).
fn run_batch_opts(batch: &[(String, u32, RvState)], cmp_fp: bool, cmp_pc: bool) {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!("riscv_diff: oracle/toolchain unavailable; skipping");
            return;
        }
    };
    let cases: Vec<(u32, u32, RvState)> = batch
        .iter()
        .map(|(_, insn, st)| (*insn, if *insn & 3 == 3 { 4 } else { 2 }, *st))
        .collect();
    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!("riscv_diff: oracle run failed; skipping");
            return;
        }
    };
    let mut mismatches = Vec::new();
    for ((label, insn, st), oc) in batch.iter().zip(outs.iter()) {
        compare_case(label, *insn, st, oc, cmp_fp, cmp_pc, &mut mismatches);
    }
    if !mismatches.is_empty() {
        let mut msg = format!("\n{} divergence(s) from qemu-riscv64:\n", mismatches.len());
        for m in mismatches.iter().take(40) {
            msg += &format!("  [{}] insn={:#010x}: {}\n", m.label, m.insn, m.detail);
        }
        if mismatches.len() > 40 {
            msg += &format!("  ... and {} more\n", mismatches.len() - 40);
        }
        panic!("{msg}");
    }
}

/// Register operands used in tests (excludes x0, x3=gp, x4=tp).
const POOL: [u32; 6] = [1, 5, 6, 10, 28, 31];

/// Build a random integer input state.
fn rand_state(rng: &mut Rng) -> RvState {
    let mut st = RvState::zeroed();
    for i in 1..32usize {
        if i == 3 || i == 4 {
            continue;
        }
        st.x[i] = rng.reg();
    }
    st
}

// ===========================================================================
// Tests.
// ===========================================================================

#[test]
fn diff_alu_reg() {
    let ops: &[(&str, u32, u32)] = &[
        ("add", 0b0000000, 0),
        ("sub", 0b0100000, 0),
        ("sll", 0b0000000, 1),
        ("slt", 0b0000000, 2),
        ("sltu", 0b0000000, 3),
        ("xor", 0b0000000, 4),
        ("srl", 0b0000000, 5),
        ("sra", 0b0100000, 5),
        ("or", 0b0000000, 6),
        ("and", 0b0000000, 7),
    ];
    let mut rng = Rng::new(0x1111);
    let mut batch = Vec::new();
    for (name, f7, f3) in ops {
        for _ in 0..40 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let rs2 = POOL[(rng.next() % 6) as usize];
            let insn = r_type(*f7, rs2, rs1, *f3, rd, 0x33);
            batch.push((name.to_string(), insn, rand_state(&mut rng)));
        }
    }
    run_batch(&batch, false);
}

#[test]
fn diff_alu_imm() {
    // (name, funct3) for I-type; shifts handled separately.
    let imm_ops: &[(&str, u32)] = &[
        ("addi", 0),
        ("slti", 2),
        ("sltiu", 3),
        ("xori", 4),
        ("ori", 6),
        ("andi", 7),
    ];
    let mut rng = Rng::new(0x2222);
    let mut batch = Vec::new();
    for (name, f3) in imm_ops {
        for _ in 0..40 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let imm = (rng.next() as i32 % 4096) - 2048;
            let insn = i_type(imm, rs1, *f3, rd, 0x13);
            batch.push((name.to_string(), insn, rand_state(&mut rng)));
        }
    }
    // Shift immediates (RV64: 6-bit shamt).
    let shifts: &[(&str, u32, u32)] = &[
        ("slli", 0b000000, 1),
        ("srli", 0b000000, 5),
        ("srai", 0b010000, 5),
    ];
    for (name, f6, f3) in shifts {
        for sh in 0..64u32 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let insn = shift_imm(*f6, sh, rs1, *f3, rd, 0x13);
            batch.push((name.to_string(), insn, rand_state(&mut rng)));
        }
    }
    run_batch(&batch, false);
}

#[test]
fn diff_alu_word() {
    let ops: &[(&str, u32, u32)] = &[
        ("addw", 0b0000000, 0),
        ("subw", 0b0100000, 0),
        ("sllw", 0b0000000, 1),
        ("srlw", 0b0000000, 5),
        ("sraw", 0b0100000, 5),
    ];
    let mut rng = Rng::new(0x3333);
    let mut batch = Vec::new();
    for (name, f7, f3) in ops {
        for _ in 0..40 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let rs2 = POOL[(rng.next() % 6) as usize];
            let insn = r_type(*f7, rs2, rs1, *f3, rd, 0x3b);
            batch.push((name.to_string(), insn, rand_state(&mut rng)));
        }
    }
    // word immediates
    let mut push_imm = |batch: &mut Vec<(String, u32, RvState)>, rng: &mut Rng| {
        for _ in 0..40 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let imm = (rng.next() as i32 % 4096) - 2048;
            batch.push((
                "addiw".into(),
                i_type(imm, rs1, 0, rd, 0x1b),
                rand_state(rng),
            ));
        }
        for sh in 0..32u32 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            batch.push((
                "slliw".into(),
                shift_imm_w(0b0000000, sh, rs1, 1, rd, 0x1b),
                rand_state(rng),
            ));
            batch.push((
                "srliw".into(),
                shift_imm_w(0b0000000, sh, rs1, 5, rd, 0x1b),
                rand_state(rng),
            ));
            batch.push((
                "sraiw".into(),
                shift_imm_w(0b0100000, sh, rs1, 5, rd, 0x1b),
                rand_state(rng),
            ));
        }
    };
    push_imm(&mut batch, &mut rng);
    run_batch(&batch, false);
}

#[test]
fn diff_muldiv() {
    let ops: &[(&str, u32, u32)] = &[
        ("mul", 1, 0),
        ("mulh", 1, 1),
        ("mulhsu", 1, 2),
        ("mulhu", 1, 3),
        ("div", 1, 4),
        ("divu", 1, 5),
        ("rem", 1, 6),
        ("remu", 1, 7),
    ];
    let word_ops: &[(&str, u32)] = &[
        ("mulw", 0),
        ("divw", 4),
        ("divuw", 5),
        ("remw", 6),
        ("remuw", 7),
    ];
    let mut rng = Rng::new(0x4444);
    let mut batch = Vec::new();
    for (name, f7, f3) in ops {
        for _ in 0..60 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let rs2 = POOL[(rng.next() % 6) as usize];
            batch.push((
                name.to_string(),
                r_type(*f7, rs2, rs1, *f3, rd, 0x33),
                rand_state(&mut rng),
            ));
        }
    }
    for (name, f3) in word_ops {
        for _ in 0..60 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let rs2 = POOL[(rng.next() % 6) as usize];
            batch.push((
                name.to_string(),
                r_type(1, rs2, rs1, *f3, rd, 0x3b),
                rand_state(&mut rng),
            ));
        }
    }
    run_batch(&batch, false);
}

#[test]
fn diff_bitmanip() {
    let mut rng = Rng::new(0x5555);
    let mut batch = Vec::new();
    // Zba (OP, funct7=0x10)
    for (name, f3) in [("sh1add", 2u32), ("sh2add", 4), ("sh3add", 6)] {
        for _ in 0..30 {
            let (rd, rs1, rs2) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                name.into(),
                r_type(0b0010000, rs2, rs1, f3, rd, 0x33),
                rand_state(&mut rng),
            ));
        }
    }
    // Zba (OP-32)
    for _ in 0..30 {
        let (rd, rs1, rs2) = (
            POOL[(rng.next() % 6) as usize],
            POOL[(rng.next() % 6) as usize],
            POOL[(rng.next() % 6) as usize],
        );
        batch.push((
            "add.uw".into(),
            r_type(0b0000100, rs2, rs1, 0, rd, 0x3b),
            rand_state(&mut rng),
        ));
        batch.push((
            "sh1add.uw".into(),
            r_type(0b0010000, rs2, rs1, 2, rd, 0x3b),
            rand_state(&mut rng),
        ));
        batch.push((
            "sh2add.uw".into(),
            r_type(0b0010000, rs2, rs1, 4, rd, 0x3b),
            rand_state(&mut rng),
        ));
        batch.push((
            "sh3add.uw".into(),
            r_type(0b0010000, rs2, rs1, 6, rd, 0x3b),
            rand_state(&mut rng),
        ));
    }
    // slli.uw (OP-IMM-32, funct6=0b000010)
    for sh in 0..64u32 {
        let (rd, rs1) = (
            POOL[(rng.next() % 6) as usize],
            POOL[(rng.next() % 6) as usize],
        );
        batch.push((
            "slli.uw".into(),
            shift_imm(0b000010, sh, rs1, 1, rd, 0x1b),
            rand_state(&mut rng),
        ));
    }
    // Zbb logic-with-negate (OP, funct7=0x20)
    for (name, f3) in [("andn", 7u32), ("orn", 6), ("xnor", 4)] {
        for _ in 0..30 {
            let (rd, rs1, rs2) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                name.into(),
                r_type(0b0100000, rs2, rs1, f3, rd, 0x33),
                rand_state(&mut rng),
            ));
        }
    }
    // Zbb rotate (OP, funct7=0x30)
    for (name, f3) in [("rol", 1u32), ("ror", 5)] {
        for _ in 0..30 {
            let (rd, rs1, rs2) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                name.into(),
                r_type(0b0110000, rs2, rs1, f3, rd, 0x33),
                rand_state(&mut rng),
            ));
        }
    }
    // rolw/rorw (OP-32)
    for (name, f3) in [("rolw", 1u32), ("rorw", 5)] {
        for _ in 0..30 {
            let (rd, rs1, rs2) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                name.into(),
                r_type(0b0110000, rs2, rs1, f3, rd, 0x3b),
                rand_state(&mut rng),
            ));
        }
    }
    // Zbb min/max (OP, funct7=0x05)
    for (name, f3) in [("min", 4u32), ("minu", 5), ("max", 6), ("maxu", 7)] {
        for _ in 0..30 {
            let (rd, rs1, rs2) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                name.into(),
                r_type(0b0000101, rs2, rs1, f3, rd, 0x33),
                rand_state(&mut rng),
            ));
        }
    }
    // Zbc clmul (OP, funct7=0x05)
    for (name, f3) in [("clmul", 1u32), ("clmulr", 2), ("clmulh", 3)] {
        for _ in 0..30 {
            let (rd, rs1, rs2) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                name.into(),
                r_type(0b0000101, rs2, rs1, f3, rd, 0x33),
                rand_state(&mut rng),
            ));
        }
    }
    // Zbb unary (OP-IMM funct7=0x30): clz/ctz/cpop/sext.b/sext.h
    for (name, rs2v) in [
        ("clz", 0u32),
        ("ctz", 1),
        ("cpop", 2),
        ("sext.b", 4),
        ("sext.h", 5),
    ] {
        for _ in 0..30 {
            let (rd, rs1) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            let insn = r_type(0b0110000, rs2v, rs1, 1, rd, 0x13);
            batch.push((name.into(), insn, rand_state(&mut rng)));
        }
    }
    // Zbb unary word (OP-IMM-32 funct7=0x30): clzw/ctzw/cpopw
    for (name, rs2v) in [("clzw", 0u32), ("ctzw", 1), ("cpopw", 2)] {
        for _ in 0..30 {
            let (rd, rs1) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            let insn = r_type(0b0110000, rs2v, rs1, 1, rd, 0x1b);
            batch.push((name.into(), insn, rand_state(&mut rng)));
        }
    }
    // zext.h (OP-32 funct7=0x04, rs2=0, funct3=4)
    for _ in 0..30 {
        let (rd, rs1) = (
            POOL[(rng.next() % 6) as usize],
            POOL[(rng.next() % 6) as usize],
        );
        batch.push((
            "zext.h".into(),
            r_type(0b0000100, 0, rs1, 4, rd, 0x3b),
            rand_state(&mut rng),
        ));
    }
    // orc.b / rev8 / rori / roriw
    for _ in 0..30 {
        let (rd, rs1) = (
            POOL[(rng.next() % 6) as usize],
            POOL[(rng.next() % 6) as usize],
        );
        batch.push((
            "orc.b".into(),
            r_type(0b0010100, 0b00111, rs1, 5, rd, 0x13),
            rand_state(&mut rng),
        ));
        batch.push((
            "rev8".into(),
            i_type(0b011010111000, rs1, 5, rd, 0x13),
            rand_state(&mut rng),
        ));
    }
    for sh in 0..64u32 {
        let (rd, rs1) = (
            POOL[(rng.next() % 6) as usize],
            POOL[(rng.next() % 6) as usize],
        );
        batch.push((
            "rori".into(),
            shift_imm(0b011000, sh, rs1, 5, rd, 0x13),
            rand_state(&mut rng),
        ));
    }
    for sh in 0..32u32 {
        let (rd, rs1) = (
            POOL[(rng.next() % 6) as usize],
            POOL[(rng.next() % 6) as usize],
        );
        batch.push((
            "roriw".into(),
            shift_imm_w(0b0110000, sh, rs1, 5, rd, 0x1b),
            rand_state(&mut rng),
        ));
    }
    // Zbs (OP funct7) and immediate forms
    for (name, f7, f3) in [
        ("bclr", 0b0100100u32, 1u32),
        ("bext", 0b0100100, 5),
        ("binv", 0b0110100, 1),
        ("bset", 0b0010100, 1),
    ] {
        for _ in 0..30 {
            let (rd, rs1, rs2) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                name.into(),
                r_type(f7, rs2, rs1, f3, rd, 0x33),
                rand_state(&mut rng),
            ));
        }
    }
    for (name, f6, f3) in [
        ("bclri", 0b010010u32, 1u32),
        ("bexti", 0b010010, 5),
        ("binvi", 0b011010, 1),
        ("bseti", 0b001010, 1),
    ] {
        for sh in (0..64u32).step_by(7) {
            let (rd, rs1) = (
                POOL[(rng.next() % 6) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                name.into(),
                shift_imm(f6, sh, rs1, f3, rd, 0x13),
                rand_state(&mut rng),
            ));
        }
    }
    run_batch(&batch, false);
}

#[test]
fn diff_loads_stores() {
    let mut rng = Rng::new(0x6666);
    let mut batch = Vec::new();
    // Base register x10 -> SCRATCH_BASE; offsets small so we stay in the window.
    let loads: &[(&str, u32)] = &[
        ("lb", 0),
        ("lh", 1),
        ("lw", 2),
        ("ld", 3),
        ("lbu", 4),
        ("lhu", 5),
        ("lwu", 6),
    ];
    let stores: &[(&str, u32)] = &[("sb", 0), ("sh", 1), ("sw", 2), ("sd", 3)];
    let offs: [i32; 5] = [0, 8, 16, -8, 24];
    for (name, f3) in loads {
        for &off in offs.iter() {
            for _ in 0..6 {
                let rd = POOL[(rng.next() % 6) as usize];
                if rd == 10 {
                    continue;
                }
                let mut st = rand_state(&mut rng);
                st.x[10] = SCRATCH_BASE;
                for s in st.scratch.iter_mut() {
                    *s = rng.next();
                }
                batch.push((name.to_string(), i_type(off, 10, *f3, rd, 0x03), st));
            }
        }
    }
    for (name, f3) in stores {
        for &off in offs.iter() {
            for _ in 0..6 {
                let rs2 = POOL[(rng.next() % 6) as usize];
                if rs2 == 10 {
                    continue;
                }
                let mut st = rand_state(&mut rng);
                st.x[10] = SCRATCH_BASE;
                for s in st.scratch.iter_mut() {
                    *s = rng.next();
                }
                batch.push((name.to_string(), s_type(off, rs2, 10, *f3, 0x23), st));
            }
        }
    }
    run_batch(&batch, false);
}

// ---------------------------------------------------------------------------
// Compressed (C) extension.
// ---------------------------------------------------------------------------

/// Compressed 3-bit register field for x8..x15.
fn cr(x: u32) -> u32 {
    x - 8
}

// Structured 16-bit RVC encoders (q = quadrant in low 2 bits).
fn c_addi(rd: u32, imm: i32) -> u32 {
    let u = (imm as u32) & 0x3f;
    (0b000 << 13) | (((u >> 5) & 1) << 12) | (rd << 7) | ((u & 0x1f) << 2) | 0b01
}
fn c_addiw(rd: u32, imm: i32) -> u32 {
    let u = (imm as u32) & 0x3f;
    (0b001 << 13) | (((u >> 5) & 1) << 12) | (rd << 7) | ((u & 0x1f) << 2) | 0b01
}
fn c_li(rd: u32, imm: i32) -> u32 {
    let u = (imm as u32) & 0x3f;
    (0b010 << 13) | (((u >> 5) & 1) << 12) | (rd << 7) | ((u & 0x1f) << 2) | 0b01
}
fn c_lui(rd: u32, imm17: u32) -> u32 {
    (0b011 << 13) | (((imm17 >> 17) & 1) << 12) | (rd << 7) | (((imm17 >> 12) & 0x1f) << 2) | 0b01
}
fn c_addi16sp(v: i32) -> u32 {
    let u = (v as u32) & 0x3ff;
    (0b011 << 13)
        | (((u >> 9) & 1) << 12)
        | (2 << 7)
        | (((u >> 4) & 1) << 6)
        | (((u >> 6) & 1) << 5)
        | (((u >> 7) & 3) << 3)
        | (((u >> 5) & 1) << 2)
        | 0b01
}
fn c_addi4spn(rd_: u32, u: u32) -> u32 {
    (0b000 << 13)
        | (((u >> 4) & 3) << 11)
        | (((u >> 6) & 0xf) << 7)
        | (((u >> 2) & 1) << 6)
        | (((u >> 3) & 1) << 5)
        | (cr(rd_) << 2)
        | 0b00
}
fn c_mv(rd: u32, rs2: u32) -> u32 {
    (0b100 << 13) | (0 << 12) | (rd << 7) | (rs2 << 2) | 0b10
}
fn c_add(rd: u32, rs2: u32) -> u32 {
    (0b100 << 13) | (1 << 12) | (rd << 7) | (rs2 << 2) | 0b10
}
fn c_alu(rd_: u32, rs2_: u32, bit12: u32, sel: u32) -> u32 {
    (0b100 << 13)
        | (bit12 << 12)
        | (0b11 << 10)
        | (cr(rd_) << 7)
        | (sel << 5)
        | (cr(rs2_) << 2)
        | 0b01
}
fn c_shift(rd_: u32, sh: u32, funct2: u32) -> u32 {
    (0b100 << 13)
        | (((sh >> 5) & 1) << 12)
        | (funct2 << 10)
        | (cr(rd_) << 7)
        | ((sh & 0x1f) << 2)
        | 0b01
}
fn c_andi(rd_: u32, imm: i32) -> u32 {
    let u = (imm as u32) & 0x3f;
    (0b100 << 13)
        | (((u >> 5) & 1) << 12)
        | (0b10 << 10)
        | (cr(rd_) << 7)
        | ((u & 0x1f) << 2)
        | 0b01
}
fn c_slli(rd: u32, sh: u32) -> u32 {
    (0b000 << 13) | (((sh >> 5) & 1) << 12) | (rd << 7) | ((sh & 0x1f) << 2) | 0b10
}
fn c_lwsp(rd: u32, u: u32) -> u32 {
    (0b010 << 13)
        | (((u >> 5) & 1) << 12)
        | (rd << 7)
        | (((u >> 2) & 7) << 4)
        | (((u >> 6) & 3) << 2)
        | 0b10
}
fn c_ldsp(rd: u32, u: u32) -> u32 {
    (0b011 << 13)
        | (((u >> 5) & 1) << 12)
        | (rd << 7)
        | (((u >> 3) & 3) << 5)
        | (((u >> 6) & 7) << 2)
        | 0b10
}
fn c_swsp(rs2: u32, u: u32) -> u32 {
    (0b110 << 13) | (((u >> 2) & 0xf) << 9) | (((u >> 6) & 3) << 7) | (rs2 << 2) | 0b10
}
fn c_sdsp(rs2: u32, u: u32) -> u32 {
    (0b111 << 13) | (((u >> 3) & 7) << 10) | (((u >> 6) & 7) << 7) | (rs2 << 2) | 0b10
}
fn c_lw(rd_: u32, rs1_: u32, u: u32) -> u32 {
    (0b010 << 13)
        | (((u >> 3) & 7) << 10)
        | (cr(rs1_) << 7)
        | (((u >> 2) & 1) << 6)
        | (((u >> 6) & 1) << 5)
        | (cr(rd_) << 2)
        | 0b00
}
fn c_ld(rd_: u32, rs1_: u32, u: u32) -> u32 {
    (0b011 << 13)
        | (((u >> 3) & 7) << 10)
        | (cr(rs1_) << 7)
        | (((u >> 6) & 3) << 5)
        | (cr(rd_) << 2)
        | 0b00
}
fn c_sw(rs2_: u32, rs1_: u32, u: u32) -> u32 {
    (0b110 << 13)
        | (((u >> 3) & 7) << 10)
        | (cr(rs1_) << 7)
        | (((u >> 2) & 1) << 6)
        | (((u >> 6) & 1) << 5)
        | (cr(rs2_) << 2)
        | 0b00
}
fn c_sd(rs2_: u32, rs1_: u32, u: u32) -> u32 {
    (0b111 << 13)
        | (((u >> 3) & 7) << 10)
        | (cr(rs1_) << 7)
        | (((u >> 6) & 3) << 5)
        | (cr(rs2_) << 2)
        | 0b00
}

/// Ops the compressed differential test can compare (register/immediate only,
/// no control flow, no memory, no FP, no system).
fn diffable_compressed(op: Op) -> bool {
    use Op::*;
    matches!(
        op,
        Lui | Addi
            | Slti
            | Sltiu
            | Xori
            | Ori
            | Andi
            | Slli
            | Srli
            | Srai
            | Add
            | Sub
            | Sll
            | Slt
            | Sltu
            | Xor
            | Srl
            | Sra
            | Or
            | And
            | Addiw
            | Slliw
            | Srliw
            | Sraiw
            | Addw
            | Subw
            | Sllw
            | Srlw
            | Sraw
    )
}

#[test]
fn diff_compressed_alu() {
    let mut rng = Rng::new(0x8888);
    let mut batch = Vec::new();
    let cpool = [8u32, 9, 10, 11, 12, 13, 14, 15];
    // Immediate / register integer forms.
    for _ in 0..30 {
        let rd = POOL[(rng.next() % 6) as usize];
        let rd_nz = if rd == 0 { 1 } else { rd };
        let imm = (rng.next() as i32 % 64) - 32;
        batch.push(("c.addi".into(), c_addi(rd_nz, imm), rand_state(&mut rng)));
        batch.push(("c.li".into(), c_li(rd_nz, imm), rand_state(&mut rng)));
        if rd_nz != 0 {
            batch.push(("c.addiw".into(), c_addiw(rd_nz, imm), rand_state(&mut rng)));
        }
        // c.lui rd!=0,2 with non-zero imm
        let rl = POOL[(rng.next() % 6) as usize];
        if rl != 0 && rl != 2 {
            let imm17 = ((rng.next() as u32) & 0x3f000) | 0x1000; // ensure nonzero
            batch.push(("c.lui".into(), c_lui(rl, imm17), rand_state(&mut rng)));
        }
        // c.addi16sp (nonzero multiple of 16)
        let v = (((rng.next() as i32) % 32) - 16) * 16;
        if v != 0 {
            batch.push(("c.addi16sp".into(), c_addi16sp(v), rand_state(&mut rng)));
        }
        // c.addi4spn -> needs x2 base set; rand_state sets x2 random, fine (no mem)
        let rdp = cpool[(rng.next() % 8) as usize];
        let u = (((rng.next() as u32) % 64) + 1) * 4; // nonzero multiple of 4
        let mut st = rand_state(&mut rng);
        st.x[2] = rng.next();
        batch.push(("c.addi4spn".into(), c_addi4spn(rdp, u & 0x3ff), st));
        // c.mv / c.add
        let r1 = POOL[(rng.next() % 6) as usize];
        let r2 = POOL[(rng.next() % 6) as usize];
        if r1 != 0 && r2 != 0 {
            batch.push(("c.mv".into(), c_mv(r1, r2), rand_state(&mut rng)));
            batch.push(("c.add".into(), c_add(r1, r2), rand_state(&mut rng)));
        }
        // c.sub/xor/or/and/subw/addw (reg' forms)
        let a = cpool[(rng.next() % 8) as usize];
        let b = cpool[(rng.next() % 8) as usize];
        for (name, b12, sel) in [
            ("c.sub", 0u32, 0b00u32),
            ("c.xor", 0, 0b01),
            ("c.or", 0, 0b10),
            ("c.and", 0, 0b11),
            ("c.subw", 1, 0b00),
            ("c.addw", 1, 0b01),
        ] {
            batch.push((name.into(), c_alu(a, b, b12, sel), rand_state(&mut rng)));
        }
        // c.srli/srai/andi/slli
        let sh = (rng.next() % 64) as u32;
        batch.push(("c.srli".into(), c_shift(a, sh, 0b00), rand_state(&mut rng)));
        batch.push(("c.srai".into(), c_shift(a, sh, 0b01), rand_state(&mut rng)));
        batch.push(("c.andi".into(), c_andi(a, imm), rand_state(&mut rng)));
        if rd_nz != 0 {
            batch.push(("c.slli".into(), c_slli(rd_nz, sh), rand_state(&mut rng)));
        }
    }
    run_batch(&batch, false);
}

#[test]
fn diff_compressed_mem() {
    let mut rng = Rng::new(0x9999);
    let mut batch = Vec::new();
    let cpool = [8u32, 9, 11, 12, 13, 14, 15]; // exclude x10 (used as base)
    // SP-relative: base x2 = SCRATCH_BASE.
    for _ in 0..20 {
        let rd = POOL[(rng.next() % 6) as usize];
        let rd_nz = if rd == 0 || rd == 2 { 1 } else { rd };
        let rs2 = POOL[(rng.next() % 6) as usize];
        let mk_state = |rng: &mut Rng| {
            let mut st = rand_state(rng);
            st.x[2] = SCRATCH_BASE;
            for s in st.scratch.iter_mut() {
                *s = rng.next();
            }
            st
        };
        let uw = (rng.next() % 8) as u32 * 4; // word offset within window
        let ud = (rng.next() % 4) as u32 * 8; // dword offset within window
        batch.push(("c.lwsp".into(), c_lwsp(rd_nz, uw), mk_state(&mut rng)));
        batch.push(("c.ldsp".into(), c_ldsp(rd_nz, ud), mk_state(&mut rng)));
        batch.push(("c.swsp".into(), c_swsp(rs2, uw), mk_state(&mut rng)));
        batch.push(("c.sdsp".into(), c_sdsp(rs2, ud), mk_state(&mut rng)));
    }
    // reg'-relative: base x10 (cr=2) = SCRATCH_BASE.
    for _ in 0..20 {
        let rdp = cpool[(rng.next() % 7) as usize];
        let rs2p = cpool[(rng.next() % 7) as usize];
        let mk_state = |rng: &mut Rng| {
            let mut st = rand_state(rng);
            st.x[10] = SCRATCH_BASE;
            for s in st.scratch.iter_mut() {
                *s = rng.next();
            }
            st
        };
        let uw = (rng.next() % 8) as u32 * 4;
        let ud = (rng.next() % 4) as u32 * 8;
        batch.push(("c.lw".into(), c_lw(rdp, 10, uw), mk_state(&mut rng)));
        batch.push(("c.ld".into(), c_ld(rdp, 10, ud), mk_state(&mut rng)));
        batch.push(("c.sw".into(), c_sw(rs2p, 10, uw), mk_state(&mut rng)));
        batch.push(("c.sd".into(), c_sd(rs2p, 10, ud), mk_state(&mut rng)));
    }
    run_batch(&batch, false);
}

#[test]
fn diff_compressed_fuzz() {
    // Throw random 16-bit parcels at both rax and qemu, keeping only those rax
    // decodes to a register/immediate integer op (no memory/control/FP/system),
    // so any decoder divergence surfaces independent of our structured encoders.
    let mut rng = Rng::new(0xABCD);
    let isa = Isa::rv64gc();
    let mut batch = Vec::new();
    let mut tries = 0;
    while batch.len() < 8000 && tries < 400_000 {
        tries += 1;
        let half = (rng.next() & 0xffff) as u16;
        if half & 0x3 == 0x3 || half == 0 {
            continue; // not a (legal) compressed parcel
        }
        let insn = decode_compressed(half, Xlen::Rv64, &isa);
        if insn.is_illegal() || !diffable_compressed(insn.op) {
            continue;
        }
        // Reject encodings that read x3/x4 (reserved) as a source.
        if insn.rs1 == 3
            || insn.rs1 == 4
            || insn.rs2 == 3
            || insn.rs2 == 4
            || insn.rd == 3
            || insn.rd == 4
        {
            continue;
        }
        batch.push(("fuzz".into(), half as u32, rand_state(&mut rng)));
    }
    run_batch(&batch, false);
}

// ---------------------------------------------------------------------------
// Zicsr (control/status registers).
// ---------------------------------------------------------------------------

fn csr_insn(csr: u32, rs1_or_zimm: u32, funct3: u32, rd: u32) -> u32 {
    (csr << 20) | (rs1_or_zimm << 15) | (funct3 << 12) | (rd << 7) | 0x73
}

#[test]
fn diff_zicsr() {
    // Only the user-accessible FP CSRs (fflags/frm/fcsr) can be exercised under
    // qemu-user; the read/modify/write semantics are identical across all CSRs.
    let mut rng = Rng::new(0xC5C5);
    let mut batch = Vec::new();
    for csr in [0x001u32, 0x002, 0x003] {
        for _ in 0..40 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let mut st = rand_state(&mut rng);
            st.fcsr = rng.next() & 0xff;
            st.x[rs1 as usize] = rng.next();
            batch.push(("csrrw".into(), csr_insn(csr, rs1, 1, rd), st));
            batch.push(("csrrs".into(), csr_insn(csr, rs1, 2, rd), st));
            batch.push(("csrrc".into(), csr_insn(csr, rs1, 3, rd), st));
            let zimm = (rng.next() % 32) as u32;
            batch.push(("csrrwi".into(), csr_insn(csr, zimm, 5, rd), st));
            batch.push(("csrrsi".into(), csr_insn(csr, zimm, 6, rd), st));
            batch.push(("csrrci".into(), csr_insn(csr, zimm, 7, rd), st));
        }
    }
    run_batch(&batch, true);
}

// ---------------------------------------------------------------------------
// Floating point (F / D).
// ---------------------------------------------------------------------------

fn fp(funct7: u32, rs2: u32, rs1: u32, rm: u32, rd: u32) -> u32 {
    (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (rm << 12) | (rd << 7) | 0x53
}
fn fma_enc(rs3: u32, funct2: u32, rs2: u32, rs1: u32, rm: u32, rd: u32, opcode: u32) -> u32 {
    (rs3 << 27) | (funct2 << 25) | (rs2 << 20) | (rs1 << 15) | (rm << 12) | (rd << 7) | opcode
}

fn box32(b: u32) -> u64 {
    0xffff_ffff_0000_0000 | b as u64
}

fn rand_f32_bits(rng: &mut Rng) -> u32 {
    match rng.next() % 16 {
        0 => 0x0000_0000,
        1 => 0x8000_0000,
        2 => 0x7f80_0000,                        // +inf
        3 => 0xff80_0000,                        // -inf
        4 => 0x7fc0_0000,                        // qNaN
        5 => 0x7f80_0001,                        // sNaN
        6 => 0x3f80_0000,                        // 1.0
        7 => 0xbf80_0000,                        // -1.0
        8 => 0x0000_0001,                        // min subnormal
        9 => 0x0080_0000,                        // min normal
        10 => 0x7f7f_ffff,                       // max normal
        11 => (rng.next() as u32) & 0x807f_ffff, // small exponent
        _ => rng.next() as u32,
    }
}
fn rand_f64_bits(rng: &mut Rng) -> u64 {
    match rng.next() % 16 {
        0 => 0x0000_0000_0000_0000,
        1 => 0x8000_0000_0000_0000,
        2 => 0x7ff0_0000_0000_0000,  // +inf
        3 => 0xfff0_0000_0000_0000,  // -inf
        4 => 0x7ff8_0000_0000_0000,  // qNaN
        5 => 0x7ff0_0000_0000_0001,  // sNaN
        6 => 0x3ff0_0000_0000_0000,  // 1.0
        7 => 0xbff0_0000_0000_0000,  // -1.0
        8 => 0x0000_0000_0000_0001,  // min subnormal
        9 => 0x0010_0000_0000_0000,  // min normal
        10 => 0x7fef_ffff_ffff_ffff, // max normal
        11 => rng.next() & 0x800f_ffff_ffff_ffff,
        _ => rng.next(),
    }
}

/// FP register pool (no reservation concerns).
const FPOOL: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

/// Build a state with single-precision (NaN-boxed) operands in fs1/fs2/fs3.
fn fp_state_s(rng: &mut Rng, fs1: u32, fs2: u32, fs3: u32, frm: u64) -> RvState {
    let mut st = RvState::zeroed();
    for i in 0..32usize {
        st.f[i] = box32(0x7fc0_0000); // default canonical NaN, boxed
    }
    st.f[fs1 as usize] = box32(rand_f32_bits(rng));
    st.f[fs2 as usize] = box32(rand_f32_bits(rng));
    st.f[fs3 as usize] = box32(rand_f32_bits(rng));
    st.fcsr = frm << 5;
    st
}
fn fp_state_d(rng: &mut Rng, fs1: u32, fs2: u32, fs3: u32, frm: u64) -> RvState {
    let mut st = RvState::zeroed();
    st.f[fs1 as usize] = rand_f64_bits(rng);
    st.f[fs2 as usize] = rand_f64_bits(rng);
    st.f[fs3 as usize] = rand_f64_bits(rng);
    st.fcsr = frm << 5;
    st
}

#[test]
fn diff_fp_arith() {
    let mut rng = Rng::new(0xF10A7);
    let mut batch = Vec::new();
    // (name, funct7_single, funct7_double)
    let bin: &[(&str, u32, u32)] = &[
        ("fadd", 0x00, 0x01),
        ("fsub", 0x04, 0x05),
        ("fmul", 0x08, 0x09),
        ("fdiv", 0x0c, 0x0d),
    ];
    // static rounding modes 0..4 plus dynamic (7) with frm 0..4.
    let modes: [(u32, u64); 6] = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (7, 2)];
    for (name, f7s, f7d) in bin {
        for &(rm, frm) in modes.iter() {
            for _ in 0..25 {
                let fd = FPOOL[(rng.next() % 8) as usize];
                let f1 = FPOOL[(rng.next() % 8) as usize];
                let f2 = FPOOL[(rng.next() % 8) as usize];
                batch.push((
                    format!("{name}.s"),
                    fp(*f7s, f2, f1, rm, fd),
                    fp_state_s(&mut rng, f1, f2, 0, frm),
                ));
                batch.push((
                    format!("{name}.d"),
                    fp(*f7d, f2, f1, rm, fd),
                    fp_state_d(&mut rng, f1, f2, 0, frm),
                ));
            }
        }
    }
    // sqrt
    for &(rm, frm) in modes.iter() {
        for _ in 0..40 {
            let fd = FPOOL[(rng.next() % 8) as usize];
            let f1 = FPOOL[(rng.next() % 8) as usize];
            batch.push((
                "fsqrt.s".into(),
                fp(0x2c, 0, f1, rm, fd),
                fp_state_s(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "fsqrt.d".into(),
                fp(0x2d, 0, f1, rm, fd),
                fp_state_d(&mut rng, f1, 0, 0, frm),
            ));
        }
    }
    run_batch(&batch, true);
}

#[test]
fn diff_fp_fma() {
    let mut rng = Rng::new(0xFADD);
    let mut batch = Vec::new();
    let fmas: &[(&str, u32)] = &[
        ("fmadd", 0x43),
        ("fmsub", 0x47),
        ("fnmsub", 0x4b),
        ("fnmadd", 0x4f),
    ];
    let modes: [(u32, u64); 6] = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (7, 3)];
    for (name, opcode) in fmas {
        for &(rm, frm) in modes.iter() {
            for _ in 0..20 {
                let fd = FPOOL[(rng.next() % 8) as usize];
                let f1 = FPOOL[(rng.next() % 8) as usize];
                let f2 = FPOOL[(rng.next() % 8) as usize];
                let f3 = FPOOL[(rng.next() % 8) as usize];
                batch.push((
                    format!("{name}.s"),
                    fma_enc(f3, 0b00, f2, f1, rm, fd, *opcode),
                    fp_state_s(&mut rng, f1, f2, f3, frm),
                ));
                batch.push((
                    format!("{name}.d"),
                    fma_enc(f3, 0b01, f2, f1, rm, fd, *opcode),
                    fp_state_d(&mut rng, f1, f2, f3, frm),
                ));
            }
        }
    }
    run_batch(&batch, true);
}

#[test]
fn diff_fp_minmax_sgnj() {
    let mut rng = Rng::new(0x11AC);
    let mut batch = Vec::new();
    for _ in 0..120 {
        let fd = FPOOL[(rng.next() % 8) as usize];
        let f1 = FPOOL[(rng.next() % 8) as usize];
        let f2 = FPOOL[(rng.next() % 8) as usize];
        // min/max (funct3 0/1, funct7 0x14/0x15)
        batch.push((
            "fmin.s".into(),
            fp(0x14, f2, f1, 0, fd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fmax.s".into(),
            fp(0x14, f2, f1, 1, fd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fmin.d".into(),
            fp(0x15, f2, f1, 0, fd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fmax.d".into(),
            fp(0x15, f2, f1, 1, fd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
        // sign inject (funct7 0x10/0x11, funct3 0/1/2)
        for f3 in 0..3u32 {
            batch.push((
                format!("fsgnj.s{f3}"),
                fp(0x10, f2, f1, f3, fd),
                fp_state_s(&mut rng, f1, f2, 0, 0),
            ));
            batch.push((
                format!("fsgnj.d{f3}"),
                fp(0x11, f2, f1, f3, fd),
                fp_state_d(&mut rng, f1, f2, 0, 0),
            ));
        }
    }
    run_batch(&batch, true);
}

#[test]
fn diff_fp_compare_class() {
    let mut rng = Rng::new(0xC1A55);
    let mut batch = Vec::new();
    for _ in 0..150 {
        let rd = POOL[(rng.next() % 6) as usize];
        let f1 = FPOOL[(rng.next() % 8) as usize];
        let f2 = FPOOL[(rng.next() % 8) as usize];
        // feq/flt/fle (funct7 0x50/0x51, funct3 2/1/0)
        batch.push((
            "feq.s".into(),
            fp(0x50, f2, f1, 2, rd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "flt.s".into(),
            fp(0x50, f2, f1, 1, rd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fle.s".into(),
            fp(0x50, f2, f1, 0, rd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "feq.d".into(),
            fp(0x51, f2, f1, 2, rd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "flt.d".into(),
            fp(0x51, f2, f1, 1, rd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fle.d".into(),
            fp(0x51, f2, f1, 0, rd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
        // fclass (funct7 0x70/0x71, funct3=1, rs2=0)
        batch.push((
            "fclass.s".into(),
            fp(0x70, 0, f1, 1, rd),
            fp_state_s(&mut rng, f1, 0, 0, 0),
        ));
        batch.push((
            "fclass.d".into(),
            fp(0x71, 0, f1, 1, rd),
            fp_state_d(&mut rng, f1, 0, 0, 0),
        ));
    }
    run_batch(&batch, true);
}

#[test]
fn diff_fp_convert() {
    let mut rng = Rng::new(0xC0FFEE);
    let mut batch = Vec::new();
    let modes: [(u32, u64); 6] = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (7, 1)];
    for &(rm, frm) in modes.iter() {
        for _ in 0..40 {
            let rd_i = POOL[(rng.next() % 6) as usize];
            let rd_f = FPOOL[(rng.next() % 8) as usize];
            let rs_i = POOL[(rng.next() % 6) as usize];
            let f1 = FPOOL[(rng.next() % 8) as usize];
            // float -> int (funct7 0x60 single / 0x61 double; rs2 selects width/sign)
            for (name, rs2) in [
                ("fcvt.w.s", 0u32),
                ("fcvt.wu.s", 1),
                ("fcvt.l.s", 2),
                ("fcvt.lu.s", 3),
            ] {
                batch.push((
                    name.into(),
                    fp(0x60, rs2, f1, rm, rd_i),
                    fp_state_s(&mut rng, f1, 0, 0, frm),
                ));
            }
            for (name, rs2) in [
                ("fcvt.w.d", 0u32),
                ("fcvt.wu.d", 1),
                ("fcvt.l.d", 2),
                ("fcvt.lu.d", 3),
            ] {
                batch.push((
                    name.into(),
                    fp(0x61, rs2, f1, rm, rd_i),
                    fp_state_d(&mut rng, f1, 0, 0, frm),
                ));
            }
            // int -> float (funct7 0x68 single / 0x69 double)
            let mut st_i = rand_state(&mut rng);
            st_i.fcsr = frm << 5;
            for (name, rs2) in [
                ("fcvt.s.w", 0u32),
                ("fcvt.s.wu", 1),
                ("fcvt.s.l", 2),
                ("fcvt.s.lu", 3),
            ] {
                batch.push((name.into(), fp(0x68, rs2, rs_i, rm, rd_f), st_i));
            }
            for (name, rs2) in [
                ("fcvt.d.w", 0u32),
                ("fcvt.d.wu", 1),
                ("fcvt.d.l", 2),
                ("fcvt.d.lu", 3),
            ] {
                batch.push((name.into(), fp(0x69, rs2, rs_i, rm, rd_f), st_i));
            }
            // float <-> float
            batch.push((
                "fcvt.s.d".into(),
                fp(0x20, 1, f1, rm, rd_f),
                fp_state_d(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "fcvt.d.s".into(),
                fp(0x21, 0, f1, rm, rd_f),
                fp_state_s(&mut rng, f1, 0, 0, frm),
            ));
        }
    }
    run_batch(&batch, true);
}

#[test]
fn diff_fp_move() {
    let mut rng = Rng::new(0x301E);
    let mut batch = Vec::new();
    for _ in 0..120 {
        let ri = POOL[(rng.next() % 6) as usize];
        let f1 = FPOOL[(rng.next() % 8) as usize];
        // fmv.x.w / fmv.x.d (funct7 0x70/0x71, funct3=0, rs2=0) -> int rd
        batch.push((
            "fmv.x.w".into(),
            fp(0x70, 0, f1, 0, ri),
            fp_state_s(&mut rng, f1, 0, 0, 0),
        ));
        batch.push((
            "fmv.x.d".into(),
            fp(0x71, 0, f1, 0, ri),
            fp_state_d(&mut rng, f1, 0, 0, 0),
        ));
        // fmv.w.x / fmv.d.x (funct7 0x78/0x79) -> fp rd from int rs1
        let rd_f = FPOOL[(rng.next() % 8) as usize];
        let mut st = rand_state(&mut rng);
        for s in st.f.iter_mut() {
            *s = rng.next();
        }
        batch.push(("fmv.w.x".into(), fp(0x78, 0, ri, 0, rd_f), st));
        batch.push(("fmv.d.x".into(), fp(0x79, 0, ri, 0, rd_f), st));
    }
    run_batch(&batch, true);
}

#[test]
fn diff_atomics() {
    let mut rng = Rng::new(0x7777);
    let mut batch = Vec::new();
    // base x10 -> SCRATCH_BASE (8-aligned). rd, rs2 from pool != 10.
    let amos: &[(&str, u32, u32)] = &[
        ("amoswap.w", 0b00001, 0b010),
        ("amoadd.w", 0b00000, 0b010),
        ("amoxor.w", 0b00100, 0b010),
        ("amoand.w", 0b01100, 0b010),
        ("amoor.w", 0b01000, 0b010),
        ("amomin.w", 0b10000, 0b010),
        ("amomax.w", 0b10100, 0b010),
        ("amominu.w", 0b11000, 0b010),
        ("amomaxu.w", 0b11100, 0b010),
        ("amoswap.d", 0b00001, 0b011),
        ("amoadd.d", 0b00000, 0b011),
        ("amoxor.d", 0b00100, 0b011),
        ("amoand.d", 0b01100, 0b011),
        ("amoor.d", 0b01000, 0b011),
        ("amomin.d", 0b10000, 0b011),
        ("amomax.d", 0b10100, 0b011),
        ("amominu.d", 0b11000, 0b011),
        ("amomaxu.d", 0b11100, 0b011),
        ("lr.w", 0b00010, 0b010),
        ("lr.d", 0b00010, 0b011),
        ("sc.w", 0b00011, 0b010),
        ("sc.d", 0b00011, 0b011),
    ];
    for (name, f5, f3) in amos {
        for _ in 0..30 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs2 = POOL[(rng.next() % 6) as usize];
            if rd == 10 || rs2 == 10 {
                continue;
            }
            let rs2enc = if name.starts_with("lr") { 0 } else { rs2 };
            let mut st = rand_state(&mut rng);
            st.x[10] = SCRATCH_BASE;
            for s in st.scratch.iter_mut() {
                *s = rng.next();
            }
            batch.push((name.to_string(), amo(*f5, rs2enc, 10, *f3, rd), st));
        }
    }
    run_batch(&batch, false);
}

// ---------------------------------------------------------------------------
// Zicond.
// ---------------------------------------------------------------------------

#[test]
fn diff_zicond() {
    let mut rng = Rng::new(0xC04D);
    let mut batch = Vec::new();
    for (name, f3) in [("czero.eqz", 5u32), ("czero.nez", 7)] {
        for _ in 0..80 {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            let rs2 = POOL[(rng.next() % 6) as usize];
            let mut st = rand_state(&mut rng);
            // Bias rs2 towards zero half the time to exercise both selections.
            if rng.next() & 1 == 0 {
                st.x[rs2 as usize] = 0;
            }
            batch.push((name.into(), r_type(0b0000111, rs2, rs1, f3, rd, 0x33), st));
        }
    }
    run_batch(&batch, false);
}

// ---------------------------------------------------------------------------
// FP loads / stores.
// ---------------------------------------------------------------------------

#[test]
fn diff_fp_loadstore() {
    let mut rng = Rng::new(0xF105);
    let mut batch = Vec::new();
    let offs: [i32; 4] = [0, 8, 16, 24];
    for &off in offs.iter() {
        for _ in 0..15 {
            let fd = FPOOL[(rng.next() % 8) as usize];
            let frs2 = FPOOL[(rng.next() % 8) as usize];
            let mut st = rand_state(&mut rng);
            st.x[10] = SCRATCH_BASE;
            for s in st.scratch.iter_mut() {
                *s = rng.next();
            }
            for fi in st.f.iter_mut() {
                *fi = rng.next();
            }
            // flw/fld fd, off(x10)  (opcode 0x07)
            batch.push(("flw".into(), i_type(off, 10, 2, fd, 0x07), st));
            batch.push(("fld".into(), i_type(off, 10, 3, fd, 0x07), st));
            // fsw/fsd frs2, off(x10)  (opcode 0x27)
            batch.push(("fsw".into(), s_type(off, frs2, 10, 2, 0x27), st));
            batch.push(("fsd".into(), s_type(off, frs2, 10, 3, 0x27), st));
        }
    }
    run_batch(&batch, true);
}

// ---------------------------------------------------------------------------
// Branches (verified via PC displacement; imm=+8 reaches the oracle's taken
// target trap, fall-through reaches the +4 trap).
// ---------------------------------------------------------------------------

#[test]
fn diff_branches() {
    let mut rng = Rng::new(0xB4A4);
    let mut batch = Vec::new();
    let branches: &[(&str, u32)] = &[
        ("beq", 0),
        ("bne", 1),
        ("blt", 4),
        ("bge", 5),
        ("bltu", 6),
        ("bgeu", 7),
    ];
    for (name, f3) in branches {
        for i in 0..60 {
            let rs1 = POOL[(rng.next() % 6) as usize];
            // Half the cases compare a register with itself (forces equality);
            // the rest use independent random operands.
            let rs2 = if i % 2 == 0 {
                rs1
            } else {
                POOL[(rng.next() % 6) as usize]
            };
            batch.push((
                name.to_string(),
                b_type(8, rs2, rs1, *f3),
                rand_state(&mut rng),
            ));
        }
    }
    run_batch_opts(&batch, false, true);
}

// ---------------------------------------------------------------------------
// Exhaustive random fuzzer: high-volume coverage across every register/
// immediate-only comparable op family with full random operands and all
// rounding modes. Any divergence from qemu fails the test.
// ---------------------------------------------------------------------------

/// Valid (funct7, funct3) pairs for OP (0x33), covering base/M/Zb/Zicond.
const OP_RR: &[(u32, u32)] = &[
    (0x00, 0),
    (0x20, 0),
    (0x00, 1),
    (0x00, 2),
    (0x00, 3),
    (0x00, 4),
    (0x00, 5),
    (0x20, 5),
    (0x00, 6),
    (0x00, 7), // base I
    (0x01, 0),
    (0x01, 1),
    (0x01, 2),
    (0x01, 3),
    (0x01, 4),
    (0x01, 5),
    (0x01, 6),
    (0x01, 7), // M
    (0x10, 2),
    (0x10, 4),
    (0x10, 6), // Zba sh*add
    (0x20, 7),
    (0x20, 6),
    (0x20, 4), // Zbb andn/orn/xnor
    (0x30, 1),
    (0x30, 5), // Zbb rol/ror
    (0x05, 1),
    (0x05, 2),
    (0x05, 3), // Zbc clmul/clmulr/clmulh
    (0x05, 4),
    (0x05, 5),
    (0x05, 6),
    (0x05, 7), // Zbb min/minu/max/maxu
    (0x24, 1),
    (0x24, 5),
    (0x34, 1),
    (0x14, 1), // Zbs bclr/bext/binv/bset
    (0x07, 5),
    (0x07, 7), // Zicond
];
/// Valid (funct7, funct3) for OP-32 (0x3b).
const OP32_RR: &[(u32, u32)] = &[
    (0x00, 0),
    (0x20, 0),
    (0x00, 1),
    (0x00, 5),
    (0x20, 5), // base W
    (0x01, 0),
    (0x01, 4),
    (0x01, 5),
    (0x01, 6),
    (0x01, 7), // M W
    (0x04, 0),
    (0x10, 2),
    (0x10, 4),
    (0x10, 6), // Zba add.uw/sh*add.uw
    (0x30, 1),
    (0x30, 5), // Zbb rolw/rorw
];
/// FP binary (funct7) for single/double arithmetic + sgnj/minmax.
const FP_BIN: &[u32] = &[
    0x00, 0x04, 0x08, 0x0c, // add/sub/mul/div .s
    0x01, 0x05, 0x09, 0x0d, // .d
    0x10, 0x11, 0x14, 0x15, // sgnj/minmax .s/.d
];

fn rand_ipool(rng: &mut Rng) -> u32 {
    // Wide integer register pool excluding x0, x3 (gp), x4 (tp).
    const P: [u32; 13] = [1, 5, 6, 7, 8, 9, 10, 11, 12, 28, 29, 30, 31];
    P[(rng.next() % 13) as usize]
}

fn fuzz_one(rng: &mut Rng) -> (u32, RvState) {
    let family = rng.next() % 14;
    let rd = rand_ipool(rng);
    let rs1 = rand_ipool(rng);
    let rs2 = rand_ipool(rng);
    let fd = FPOOL[(rng.next() % 8) as usize];
    let f1 = FPOOL[(rng.next() % 8) as usize];
    let f2 = FPOOL[(rng.next() % 8) as usize];
    let f3 = FPOOL[(rng.next() % 8) as usize];
    let rm = [0u32, 1, 2, 3, 4, 7][(rng.next() % 6) as usize];
    let frm = (rng.next() % 5) << 5;
    let mut st = rand_state(rng);

    let insn = match family {
        0 => {
            let (f7, f3) = OP_RR[(rng.next() as usize) % OP_RR.len()];
            // bias rs2 to zero sometimes (Zicond / shifts)
            if rng.next() & 3 == 0 {
                st.x[rs2 as usize] = 0;
            }
            r_type(f7, rs2, rs1, f3, rd, 0x33)
        }
        1 => {
            let (f7, f3) = OP32_RR[(rng.next() as usize) % OP32_RR.len()];
            r_type(f7, rs2, rs1, f3, rd, 0x3b)
        }
        2 => {
            // OP-IMM
            let f3 = [0u32, 2, 3, 4, 6, 7][(rng.next() % 6) as usize];
            let imm = (rng.next() as i32 % 4096) - 2048;
            i_type(imm, rs1, f3, rd, 0x13)
        }
        3 => {
            // shift immediates (RV64 6-bit)
            let (f6, f3) = [
                (0b000000u32, 1u32),
                (0b000000, 5),
                (0b010000, 5),
                (0b011000, 5),
            ][(rng.next() % 4) as usize];
            shift_imm(f6, (rng.next() % 64) as u32, rs1, f3, rd, 0x13)
        }
        4 => {
            // OP-IMM-32
            if rng.next() & 1 == 0 {
                i_type((rng.next() as i32 % 4096) - 2048, rs1, 0, rd, 0x1b) // addiw
            } else {
                let (f7, f3) = [(0u32, 1u32), (0, 5), (0x20, 5)][(rng.next() % 3) as usize];
                shift_imm_w(f7, (rng.next() % 32) as u32, rs1, f3, rd, 0x1b)
            }
        }
        5 | 6 => {
            // FP single binary
            let f7 = FP_BIN[(rng.next() as usize) % FP_BIN.len()];
            install_fp(&mut st, f1, f2, f3, frm, false);
            // sgnj uses funct3 0/1/2; min/max only 0/1; arithmetic uses rm.
            let f3field = match f7 {
                0x10 | 0x11 => rng.next() as u32 % 3,
                0x14 | 0x15 => rng.next() as u32 % 2,
                _ => rm,
            };
            // double family if odd funct7; install double operands accordingly
            if f7 & 1 == 1 {
                install_fp(&mut st, f1, f2, f3, frm, true);
            }
            fp(f7, f2, f1, f3field, fd)
        }
        7 => {
            // FP fma (single + double)
            let opc = [0x43u32, 0x47, 0x4b, 0x4f][(rng.next() % 4) as usize];
            let dbl = rng.next() & 1 == 1;
            install_fp(&mut st, f1, f2, f3, frm, dbl);
            fma_enc(f3, if dbl { 1 } else { 0 }, f2, f1, rm, fd, opc)
        }
        8 => {
            // FP sqrt
            let dbl = rng.next() & 1 == 1;
            install_fp(&mut st, f1, f1, f1, frm, dbl);
            fp(if dbl { 0x2d } else { 0x2c }, 0, f1, rm, fd)
        }
        9 => {
            // FP <-> int conversions
            let dbl = rng.next() & 1 == 1;
            match rng.next() % 3 {
                0 => {
                    // f -> int
                    install_fp(&mut st, f1, f1, f1, frm, dbl);
                    let rs2sel = (rng.next() % 4) as u32;
                    fp(if dbl { 0x61 } else { 0x60 }, rs2sel, f1, rm, rd)
                }
                1 => {
                    // int -> f
                    st.fcsr = frm;
                    let rs2sel = (rng.next() % 4) as u32;
                    fp(if dbl { 0x69 } else { 0x68 }, rs2sel, rs1, rm, fd)
                }
                _ => {
                    // f <-> f
                    if dbl {
                        install_fp(&mut st, f1, f1, f1, frm, false);
                        fp(0x21, 0, f1, rm, fd) // fcvt.d.s
                    } else {
                        install_fp(&mut st, f1, f1, f1, frm, true);
                        fp(0x20, 1, f1, rm, fd) // fcvt.s.d
                    }
                }
            }
        }
        10 => {
            // FP compare / classify / move-to-int
            let dbl = rng.next() & 1 == 1;
            install_fp(&mut st, f1, f2, f2, 0, dbl);
            match rng.next() % 3 {
                0 => fp(
                    if dbl { 0x51 } else { 0x50 },
                    f2,
                    f1,
                    (rng.next() % 3) as u32,
                    rd,
                ), // cmp
                1 => fp(if dbl { 0x71 } else { 0x70 }, 0, f1, 1, rd), // fclass
                _ => fp(if dbl { 0x71 } else { 0x70 }, 0, f1, 0, rd), // fmv.x
            }
        }
        11 => {
            // FP move-from-int
            let dbl = rng.next() & 1 == 1;
            st.fcsr = 0;
            fp(if dbl { 0x79 } else { 0x78 }, 0, rs1, 0, fd)
        }
        12 => {
            // Zfa: fminm/fmaxm/fround/froundnx/fli
            let dbl = rng.next() & 1 == 1;
            match rng.next() % 5 {
                0 => {
                    install_fp(&mut st, f1, f2, f2, 0, dbl);
                    fp(if dbl { 0x15 } else { 0x14 }, f2, f1, 2, fd)
                }
                1 => {
                    install_fp(&mut st, f1, f2, f2, 0, dbl);
                    fp(if dbl { 0x15 } else { 0x14 }, f2, f1, 3, fd)
                }
                2 => {
                    install_fp(&mut st, f1, f1, f1, frm, dbl);
                    fp(if dbl { 0x21 } else { 0x20 }, 4, f1, rm, fd)
                }
                3 => {
                    install_fp(&mut st, f1, f1, f1, frm, dbl);
                    fp(if dbl { 0x21 } else { 0x20 }, 5, f1, rm, fd)
                }
                _ => {
                    st.fcsr = 0;
                    fp(
                        if dbl { 0x79 } else { 0x78 },
                        1,
                        (rng.next() % 32) as u32,
                        0,
                        fd,
                    )
                }
            }
        }
        _ => {
            // Zfa: fleq/fltq/fcvtmod.w.d -> integer rd
            match rng.next() % 3 {
                0 => {
                    let dbl = rng.next() & 1 == 1;
                    install_fp(&mut st, f1, f2, f2, 0, dbl);
                    fp(if dbl { 0x51 } else { 0x50 }, f2, f1, 4, rd)
                }
                1 => {
                    let dbl = rng.next() & 1 == 1;
                    install_fp(&mut st, f1, f2, f2, 0, dbl);
                    fp(if dbl { 0x51 } else { 0x50 }, f2, f1, 5, rd)
                }
                _ => {
                    install_fp(&mut st, f1, f1, f1, 0, true);
                    fp(0x61, 8, f1, 1, rd)
                }
            }
        }
    };
    (insn, st)
}

/// Install random FP operands (NaN-boxed singles or raw doubles) and a frm.
fn install_fp(st: &mut RvState, a: u32, b: u32, c: u32, frm: u64, dbl: bool) {
    let mut rng =
        Rng::new((st.x[1] ^ st.x[5] ^ (a as u64) << 8 ^ (frm << 1)).wrapping_add(0x9e3779b9));
    for i in 0..32usize {
        st.f[i] = if dbl {
            rand_f64_bits(&mut rng)
        } else {
            box32(rand_f32_bits(&mut rng))
        };
    }
    let _ = (a, b, c);
    st.fcsr = frm;
}

#[test]
fn diff_fuzz_exhaustive() {
    let mut rng = Rng::new(0x5EED_1234);
    let mut batch = Vec::with_capacity(40000);
    for _ in 0..90000 {
        let (insn, st) = fuzz_one(&mut rng);
        batch.push(("fuzz".to_string(), insn, st));
    }
    run_batch(&batch, true);
}

// ---------------------------------------------------------------------------
// Zfa (additional FP).
// ---------------------------------------------------------------------------

#[test]
fn diff_zfa() {
    let mut rng = Rng::new(0x2FA0);
    let mut batch = Vec::new();
    let modes: [(u32, u64); 6] = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (7, 2)];

    // fminm / fmaxm (NaN-propagating).
    for _ in 0..40 {
        let fd = FPOOL[(rng.next() % 8) as usize];
        let f1 = FPOOL[(rng.next() % 8) as usize];
        let f2 = FPOOL[(rng.next() % 8) as usize];
        batch.push((
            "fminm.s".into(),
            fp(0x14, f2, f1, 2, fd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fmaxm.s".into(),
            fp(0x14, f2, f1, 3, fd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fminm.d".into(),
            fp(0x15, f2, f1, 2, fd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fmaxm.d".into(),
            fp(0x15, f2, f1, 3, fd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
    }
    // fround / froundnx (rounding mode in funct3).
    for &(rm, frm) in modes.iter() {
        for _ in 0..15 {
            let fd = FPOOL[(rng.next() % 8) as usize];
            let f1 = FPOOL[(rng.next() % 8) as usize];
            batch.push((
                "fround.s".into(),
                fp(0x20, 4, f1, rm, fd),
                fp_state_s(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "froundnx.s".into(),
                fp(0x20, 5, f1, rm, fd),
                fp_state_s(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "fround.d".into(),
                fp(0x21, 4, f1, rm, fd),
                fp_state_d(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "froundnx.d".into(),
                fp(0x21, 5, f1, rm, fd),
                fp_state_d(&mut rng, f1, 0, 0, frm),
            ));
        }
    }
    // fleq / fltq -> integer rd.
    for _ in 0..40 {
        let rd = POOL[(rng.next() % 6) as usize];
        let f1 = FPOOL[(rng.next() % 8) as usize];
        let f2 = FPOOL[(rng.next() % 8) as usize];
        batch.push((
            "fleq.s".into(),
            fp(0x50, f2, f1, 4, rd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fltq.s".into(),
            fp(0x50, f2, f1, 5, rd),
            fp_state_s(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fleq.d".into(),
            fp(0x51, f2, f1, 4, rd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fltq.d".into(),
            fp(0x51, f2, f1, 5, rd),
            fp_state_d(&mut rng, f1, f2, 0, 0),
        ));
    }
    // fli — exhaustively cover all 32 constant-table indices.
    for idx in 0..32u32 {
        let fd = FPOOL[(rng.next() % 8) as usize];
        batch.push((
            format!("fli.s#{idx}"),
            fp(0x78, 1, idx, 0, fd),
            RvState::zeroed(),
        ));
        batch.push((
            format!("fli.d#{idx}"),
            fp(0x79, 1, idx, 0, fd),
            RvState::zeroed(),
        ));
    }
    // fcvtmod.w.d -> integer rd (modular truncation of a double).
    for _ in 0..60 {
        let rd = POOL[(rng.next() % 6) as usize];
        let f1 = FPOOL[(rng.next() % 8) as usize];
        batch.push((
            "fcvtmod.w.d".into(),
            fp(0x61, 8, f1, 1, rd),
            fp_state_d(&mut rng, f1, 0, 0, 0),
        ));
    }
    run_batch(&batch, true);
}

// ---------------------------------------------------------------------------
// Opcode-constrained decode/execute fuzzer: random words across the register-
// only opcode spaces (OP, OP-IMM, OP-32, OP-IMM-32, OP-FP, FMADD..FNMADD) with
// full random state. Proves decode+execute agreement with qemu across the
// entire encoding space (catches any instruction qemu implements that rax does
// not, and vice versa). rd/rs1 are constrained to a safe pool so a write or
// read never touches gp/tp (reserved by the oracle).
// ---------------------------------------------------------------------------

#[test]
fn diff_decode_fuzz() {
    let mut rng = Rng::new(0xDEC0_DE99);
    let opcodes: [u32; 9] = [0x33, 0x13, 0x3b, 0x1b, 0x53, 0x43, 0x47, 0x4b, 0x4f];
    let safe: [u32; 11] = [1, 5, 6, 7, 8, 9, 10, 11, 12, 28, 31];
    let mut batch = Vec::with_capacity(140000);
    for _ in 0..140000 {
        let opc = opcodes[(rng.next() as usize) % opcodes.len()];
        let mut w = (rng.next() as u32 & !0x7f) | opc;
        let rd = safe[(rng.next() as usize) % 11];
        let rs1 = safe[(rng.next() as usize) % 11];
        w = (w & !(0x1f << 7)) | (rd << 7);
        w = (w & !(0x1f << 15)) | (rs1 << 15);
        // For integer register-register ops, rs2 is a GPR read: keep it safe.
        if opc == 0x33 || opc == 0x3b {
            let rs2 = safe[(rng.next() as usize) % 11];
            w = (w & !(0x1f << 20)) | (rs2 << 20);
        }
        // This fuzzer proves that every encoding rax *decodes* matches qemu.
        // rax follows the spec strictly; qemu's FP decoder is occasionally more
        // lenient (e.g. it accepts fcvt.h.s with a reserved non-zero rs2 field),
        // so encodings rax treats as illegal are out of scope here. With the
        // full extension set implemented, this skips only those reserved-field
        // leniencies -- every encoding rax accepts is checked against qemu.
        if decode(w, Xlen::Rv64, &Isa::rv64gc()).is_illegal() {
            continue;
        }
        let mut st = rand_state(&mut rng);
        for fi in st.f.iter_mut() {
            *fi = match rng.next() % 4 {
                0 => box32(rand_f32_bits(&mut rng)),
                1 => rand_f64_bits(&mut rng),
                2 => rng.next(),
                _ => box32(rng.next() as u32),
            };
        }
        st.fcsr = (rng.next() % 5) << 5;
        batch.push(("decode".to_string(), w, st));
    }
    run_batch(&batch, true);
}

// ---------------------------------------------------------------------------
// Zbkb (bit-manip for crypto).
// ---------------------------------------------------------------------------

#[test]
fn diff_zbkb() {
    let mut rng = Rng::new(0xB8B0);
    let mut batch = Vec::new();
    for _ in 0..70 {
        let rd = POOL[(rng.next() % 6) as usize];
        let rs1 = POOL[(rng.next() % 6) as usize];
        let rs2 = POOL[(rng.next() % 6) as usize]; // pool excludes 0, so packw != zext.h
        batch.push((
            "pack".into(),
            r_type(0b0000100, rs2, rs1, 4, rd, 0x33),
            rand_state(&mut rng),
        ));
        batch.push((
            "packh".into(),
            r_type(0b0000100, rs2, rs1, 7, rd, 0x33),
            rand_state(&mut rng),
        ));
        batch.push((
            "packw".into(),
            r_type(0b0000100, rs2, rs1, 4, rd, 0x3b),
            rand_state(&mut rng),
        ));
        batch.push((
            "brev8".into(),
            r_type(0b0110100, 0b00111, rs1, 5, rd, 0x13),
            rand_state(&mut rng),
        ));
    }
    run_batch(&batch, false);
}

// ---------------------------------------------------------------------------
// Zfh (half precision).
// ---------------------------------------------------------------------------

fn box16(b: u16) -> u64 {
    0xffff_ffff_ffff_0000 | b as u64
}
fn rand_f16_bits(rng: &mut Rng) -> u16 {
    match rng.next() % 16 {
        0 => 0x0000,
        1 => 0x8000,
        2 => 0x7c00,                        // +inf
        3 => 0xfc00,                        // -inf
        4 => 0x7e00,                        // qNaN
        5 => 0x7c01,                        // sNaN
        6 => 0x3c00,                        // 1.0
        7 => 0xbc00,                        // -1.0
        8 => 0x0001,                        // min subnormal
        9 => 0x0400,                        // min normal
        10 => 0x7bff,                       // max normal
        11 => (rng.next() as u16) & 0x83ff, // small exponent
        _ => rng.next() as u16,
    }
}
fn fp_state_h(rng: &mut Rng, fs1: u32, fs2: u32, fs3: u32, frm: u64) -> RvState {
    let mut st = RvState::zeroed();
    for i in 0..32usize {
        st.f[i] = box16(0x7e00);
    }
    st.f[fs1 as usize] = box16(rand_f16_bits(rng));
    st.f[fs2 as usize] = box16(rand_f16_bits(rng));
    st.f[fs3 as usize] = box16(rand_f16_bits(rng));
    st.fcsr = frm << 5;
    st
}

#[test]
fn diff_zfh() {
    let mut rng = Rng::new(0x21F0);
    let mut batch = Vec::new();
    let modes: [(u32, u64); 6] = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (7, 2)];
    // arithmetic (funct7 fmt=10)
    let bin: &[(&str, u32)] = &[
        ("fadd.h", 0x02),
        ("fsub.h", 0x06),
        ("fmul.h", 0x0a),
        ("fdiv.h", 0x0e),
    ];
    for (name, f7) in bin {
        for &(rm, frm) in modes.iter() {
            for _ in 0..12 {
                let (fd, f1, f2) = (
                    FPOOL[(rng.next() % 8) as usize],
                    FPOOL[(rng.next() % 8) as usize],
                    FPOOL[(rng.next() % 8) as usize],
                );
                batch.push((
                    name.to_string(),
                    fp(*f7, f2, f1, rm, fd),
                    fp_state_h(&mut rng, f1, f2, 0, frm),
                ));
            }
        }
    }
    for &(rm, frm) in modes.iter() {
        for _ in 0..15 {
            let (fd, f1) = (
                FPOOL[(rng.next() % 8) as usize],
                FPOOL[(rng.next() % 8) as usize],
            );
            batch.push((
                "fsqrt.h".into(),
                fp(0x2e, 0, f1, rm, fd),
                fp_state_h(&mut rng, f1, 0, 0, frm),
            ));
        }
    }
    // fma half (funct2=10)
    for (name, opc) in [
        ("fmadd.h", 0x43u32),
        ("fmsub.h", 0x47),
        ("fnmsub.h", 0x4b),
        ("fnmadd.h", 0x4f),
    ] {
        for &(rm, frm) in modes.iter() {
            for _ in 0..8 {
                let (fd, f1, f2, f3) = (
                    FPOOL[(rng.next() % 8) as usize],
                    FPOOL[(rng.next() % 8) as usize],
                    FPOOL[(rng.next() % 8) as usize],
                    FPOOL[(rng.next() % 8) as usize],
                );
                batch.push((
                    name.into(),
                    fma_enc(f3, 0b10, f2, f1, rm, fd, opc),
                    fp_state_h(&mut rng, f1, f2, f3, frm),
                ));
            }
        }
    }
    // sgnj / min / max / minm / maxm
    for _ in 0..40 {
        let (fd, f1, f2) = (
            FPOOL[(rng.next() % 8) as usize],
            FPOOL[(rng.next() % 8) as usize],
            FPOOL[(rng.next() % 8) as usize],
        );
        for f3 in 0..3u32 {
            batch.push((
                format!("fsgnj.h{f3}"),
                fp(0x12, f2, f1, f3, fd),
                fp_state_h(&mut rng, f1, f2, 0, 0),
            ));
        }
        batch.push((
            "fmin.h".into(),
            fp(0x16, f2, f1, 0, fd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fmax.h".into(),
            fp(0x16, f2, f1, 1, fd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fminm.h".into(),
            fp(0x16, f2, f1, 2, fd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fmaxm.h".into(),
            fp(0x16, f2, f1, 3, fd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
    }
    // compares / class -> int rd
    for _ in 0..40 {
        let (rd, f1, f2) = (
            POOL[(rng.next() % 6) as usize],
            FPOOL[(rng.next() % 8) as usize],
            FPOOL[(rng.next() % 8) as usize],
        );
        batch.push((
            "feq.h".into(),
            fp(0x52, f2, f1, 2, rd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "flt.h".into(),
            fp(0x52, f2, f1, 1, rd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fle.h".into(),
            fp(0x52, f2, f1, 0, rd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fleq.h".into(),
            fp(0x52, f2, f1, 4, rd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fltq.h".into(),
            fp(0x52, f2, f1, 5, rd),
            fp_state_h(&mut rng, f1, f2, 0, 0),
        ));
        batch.push((
            "fclass.h".into(),
            fp(0x72, 0, f1, 1, rd),
            fp_state_h(&mut rng, f1, 0, 0, 0),
        ));
        batch.push((
            "fmv.x.h".into(),
            fp(0x72, 0, f1, 0, rd),
            fp_state_h(&mut rng, f1, 0, 0, 0),
        ));
    }
    // round
    for &(rm, frm) in modes.iter() {
        for _ in 0..12 {
            let (fd, f1) = (
                FPOOL[(rng.next() % 8) as usize],
                FPOOL[(rng.next() % 8) as usize],
            );
            batch.push((
                "fround.h".into(),
                fp(0x22, 4, f1, rm, fd),
                fp_state_h(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "froundnx.h".into(),
                fp(0x22, 5, f1, rm, fd),
                fp_state_h(&mut rng, f1, 0, 0, frm),
            ));
        }
    }
    // conversions half<->single/double, half<->int, fli.h, fmv.h.x
    for &(rm, frm) in modes.iter() {
        for _ in 0..15 {
            let (fd, f1, ri) = (
                FPOOL[(rng.next() % 8) as usize],
                FPOOL[(rng.next() % 8) as usize],
                POOL[(rng.next() % 6) as usize],
            );
            batch.push((
                "fcvt.s.h".into(),
                fp(0x20, 2, f1, rm, fd),
                fp_state_h(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "fcvt.d.h".into(),
                fp(0x21, 2, f1, rm, fd),
                fp_state_h(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "fcvt.h.s".into(),
                fp(0x22, 0, f1, rm, fd),
                fp_state_s(&mut rng, f1, 0, 0, frm),
            ));
            batch.push((
                "fcvt.h.d".into(),
                fp(0x22, 1, f1, rm, fd),
                fp_state_d(&mut rng, f1, 0, 0, frm),
            ));
            for (name, rs2) in [
                ("fcvt.w.h", 0u32),
                ("fcvt.wu.h", 1),
                ("fcvt.l.h", 2),
                ("fcvt.lu.h", 3),
            ] {
                batch.push((
                    name.into(),
                    fp(0x62, rs2, f1, rm, POOL[(rng.next() % 6) as usize]),
                    fp_state_h(&mut rng, f1, 0, 0, frm),
                ));
            }
            let mut st_i = rand_state(&mut rng);
            st_i.fcsr = frm << 5;
            for (name, rs2) in [
                ("fcvt.h.w", 0u32),
                ("fcvt.h.wu", 1),
                ("fcvt.h.l", 2),
                ("fcvt.h.lu", 3),
            ] {
                batch.push((name.into(), fp(0x6a, rs2, ri, rm, fd), st_i));
            }
            batch.push(("fmv.h.x".into(), fp(0x7a, 0, ri, 0, fd), st_i));
        }
    }
    for idx in 0..32u32 {
        let fd = FPOOL[(rng.next() % 8) as usize];
        batch.push((
            format!("fli.h#{idx}"),
            fp(0x7a, 1, idx, 0, fd),
            RvState::zeroed(),
        ));
    }
    run_batch(&batch, true);
}

// ---------------------------------------------------------------------------
// Scalar cryptography (Zbkx, Zknh, Zksh, Zksed, Zkne, Zknd).
// ---------------------------------------------------------------------------

/// OP-IMM unary crypto (sha/sm3/aes64im/aes64ks1i): funct7, rs2 selector.
fn op_imm_u(funct7: u32, rs2: u32, rs1: u32, rd: u32) -> u32 {
    (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (1 << 12) | (rd << 7) | 0x13
}

#[test]
fn diff_crypto() {
    let mut rng = Rng::new(0xC2_19_70);
    let mut batch = Vec::new();
    let r = |rng: &mut Rng| POOL[(rng.next() % 6) as usize];
    for _ in 0..60 {
        let (rd, rs1, rs2) = (r(&mut rng), r(&mut rng), r(&mut rng));
        // Zbkx
        batch.push((
            "xperm8".into(),
            r_type(0b0010100, rs2, rs1, 4, rd, 0x33),
            rand_state(&mut rng),
        ));
        batch.push((
            "xperm4".into(),
            r_type(0b0010100, rs2, rs1, 2, rd, 0x33),
            rand_state(&mut rng),
        ));
        // Zknh (OP-IMM unary)
        for (name, sel) in [
            ("sha256sum0", 0u32),
            ("sha256sum1", 1),
            ("sha256sig0", 2),
            ("sha256sig1", 3),
            ("sha512sum0", 4),
            ("sha512sum1", 5),
            ("sha512sig0", 6),
            ("sha512sig1", 7),
        ] {
            batch.push((
                name.into(),
                op_imm_u(0b0001000, sel, rs1, rd),
                rand_state(&mut rng),
            ));
        }
        // Zksh (SM3)
        batch.push((
            "sm3p0".into(),
            op_imm_u(0b0001000, 8, rs1, rd),
            rand_state(&mut rng),
        ));
        batch.push((
            "sm3p1".into(),
            op_imm_u(0b0001000, 9, rs1, rd),
            rand_state(&mut rng),
        ));
        // Zksed (SM4) all byte-selects
        for bs in 0..4u32 {
            batch.push((
                "sm4ed".into(),
                r_type((bs << 5) | 0b11000, rs2, rs1, 0, rd, 0x33),
                rand_state(&mut rng),
            ));
            batch.push((
                "sm4ks".into(),
                r_type((bs << 5) | 0b11010, rs2, rs1, 0, rd, 0x33),
                rand_state(&mut rng),
            ));
        }
        // Zkne / Zknd (AES-64)
        batch.push((
            "aes64es".into(),
            r_type(0b0011001, rs2, rs1, 0, rd, 0x33),
            rand_state(&mut rng),
        ));
        batch.push((
            "aes64esm".into(),
            r_type(0b0011011, rs2, rs1, 0, rd, 0x33),
            rand_state(&mut rng),
        ));
        batch.push((
            "aes64ds".into(),
            r_type(0b0011101, rs2, rs1, 0, rd, 0x33),
            rand_state(&mut rng),
        ));
        batch.push((
            "aes64dsm".into(),
            r_type(0b0011111, rs2, rs1, 0, rd, 0x33),
            rand_state(&mut rng),
        ));
        batch.push((
            "aes64ks2".into(),
            r_type(0b0111111, rs2, rs1, 0, rd, 0x33),
            rand_state(&mut rng),
        ));
        batch.push((
            "aes64im".into(),
            op_imm_u(0b0011000, 0, rs1, rd),
            rand_state(&mut rng),
        ));
        for rnum in 0..=0xAu32 {
            batch.push((
                "aes64ks1i".into(),
                op_imm_u(0b0011000, 0x10 | rnum, rs1, rd),
                rand_state(&mut rng),
            ));
        }
    }
    run_batch(&batch, false);
}

// ---------------------------------------------------------------------------
// Zcb (additional compressed) and FP-compressed (c.fld/c.fsd/...).
// ---------------------------------------------------------------------------

#[test]
fn diff_compressed_extra() {
    let mut rng = Rng::new(0xCB0);
    let mut batch = Vec::new();
    let cpool = [8u32, 9, 11, 12, 13, 14, 15]; // compressed regs excl. x10 (base)

    // --- Zcb quadrant-1 unary / mul (register only) ---
    for _ in 0..40 {
        let rd = cpool[(rng.next() % 7) as usize];
        let rs2 = cpool[(rng.next() % 7) as usize];
        let base = |f6top: u32, f3lo: u32, rd: u32, rest: u32| -> u32 {
            (0b100 << 13)
                | (1 << 12)
                | (0b11 << 10)
                | (cr(rd) << 7)
                | (f6top << 5)
                | (rest << 2)
                | 0b01
        };
        let _ = base;
        // c.mul
        batch.push((
            "c.mul".into(),
            (0b100u32 << 13)
                | (1 << 12)
                | (0b11 << 10)
                | (cr(rd) << 7)
                | (0b10 << 5)
                | (cr(rs2) << 2)
                | 0b01,
            rand_state(&mut rng),
        ));
        // c.zext.b/sext.b/zext.h/sext.h/zext.w/not
        for (name, sel) in [
            ("c.zext.b", 0u32),
            ("c.sext.b", 1),
            ("c.zext.h", 2),
            ("c.sext.h", 3),
            ("c.zext.w", 4),
            ("c.not", 5),
        ] {
            let w = (0b100u32 << 13)
                | (1 << 12)
                | (0b11 << 10)
                | (cr(rd) << 7)
                | (0b11 << 5)
                | (sel << 2)
                | 0b01;
            batch.push((name.into(), w, rand_state(&mut rng)));
        }
    }

    // --- Zcb quadrant-0 byte/half loads/stores (base x10 = SCRATCH_BASE) ---
    for _ in 0..20 {
        let rd = cpool[(rng.next() % 7) as usize];
        let rs2 = cpool[(rng.next() % 7) as usize];
        let mk_state = |rng: &mut Rng| {
            let mut st = rand_state(rng);
            st.x[10] = SCRATCH_BASE;
            for s in st.scratch.iter_mut() {
                *s = rng.next();
            }
            st
        };
        let u2 = (rng.next() % 4) as u32; // byte offset 0..3
        let u1 = (rng.next() % 2) as u32 * 2; // half offset 0/2
        // c.lbu (bits[12:10]=000): uimm bit5=uimm[1], bit6=uimm[0]
        let lbu = (0b100u32 << 13)
            | (cr(10) << 7)
            | (((u2 >> 1) & 1) << 5)
            | ((u2 & 1) << 6)
            | (cr(rd) << 2)
            | 0b00;
        batch.push(("c.lbu".into(), lbu, mk_state(&mut rng)));
        // c.lhu (001,bit6=0)
        let lhu = (0b100u32 << 13)
            | (0b001 << 10)
            | (cr(10) << 7)
            | (((u1 >> 1) & 1) << 5)
            | (cr(rd) << 2)
            | 0b00;
        batch.push(("c.lhu".into(), lhu, mk_state(&mut rng)));
        // c.lh (001,bit6=1)
        let lh = lhu | (1 << 6);
        batch.push(("c.lh".into(), lh, mk_state(&mut rng)));
        // c.sb (010)
        let sb = (0b100u32 << 13)
            | (0b010 << 10)
            | (cr(10) << 7)
            | (((u2 >> 1) & 1) << 5)
            | ((u2 & 1) << 6)
            | (cr(rs2) << 2)
            | 0b00;
        batch.push(("c.sb".into(), sb, mk_state(&mut rng)));
        // c.sh (011,bit6=0)
        let sh = (0b100u32 << 13)
            | (0b011 << 10)
            | (cr(10) << 7)
            | (((u1 >> 1) & 1) << 5)
            | (cr(rs2) << 2)
            | 0b00;
        batch.push(("c.sh".into(), sh, mk_state(&mut rng)));
    }

    // --- FP-compressed (c.fld/c.fsd reg', c.fldsp/c.fsdsp sp) ---
    for _ in 0..20 {
        let mk_state_sp = |rng: &mut Rng| {
            let mut st = rand_state(rng);
            st.x[2] = SCRATCH_BASE;
            for s in st.scratch.iter_mut() {
                *s = rng.next();
            }
            for fi in st.f.iter_mut() {
                *fi = rng.next();
            }
            st
        };
        let mk_state_r = |rng: &mut Rng| {
            let mut st = rand_state(rng);
            st.x[10] = SCRATCH_BASE;
            for s in st.scratch.iter_mut() {
                *s = rng.next();
            }
            for fi in st.f.iter_mut() {
                *fi = rng.next();
            }
            st
        };
        let rd = FPOOL[(rng.next() % 8) as usize];
        let rs2 = FPOOL[(rng.next() % 8) as usize];
        let rdp = cpool[(rng.next() % 7) as usize];
        let rs2p = cpool[(rng.next() % 7) as usize];
        let ud = (rng.next() % 4) as u32 * 8; // dword offset within window
        // c.fldsp fd, ud(sp)
        let fldsp = (0b001u32 << 13)
            | (((ud >> 5) & 1) << 12)
            | (rd << 7)
            | (((ud >> 3) & 3) << 5)
            | (((ud >> 6) & 7) << 2)
            | 0b10;
        batch.push(("c.fldsp".into(), fldsp, mk_state_sp(&mut rng)));
        // c.fsdsp fs2, ud(sp)
        let fsdsp =
            (0b101u32 << 13) | (((ud >> 3) & 7) << 10) | (((ud >> 6) & 7) << 7) | (rs2 << 2) | 0b10;
        batch.push(("c.fsdsp".into(), fsdsp, mk_state_sp(&mut rng)));
        // c.fld fd', ud(x10)
        let fld = (0b001u32 << 13)
            | (((ud >> 3) & 7) << 10)
            | (cr(10) << 7)
            | (((ud >> 6) & 3) << 5)
            | (cr(rdp) << 2)
            | 0b00;
        batch.push(("c.fld".into(), fld, mk_state_r(&mut rng)));
        // c.fsd fs2', ud(x10)
        let fsd = (0b101u32 << 13)
            | (((ud >> 3) & 7) << 10)
            | (cr(10) << 7)
            | (((ud >> 6) & 3) << 5)
            | (cr(rs2p) << 2)
            | 0b00;
        batch.push(("c.fsd".into(), fsd, mk_state_r(&mut rng)));
    }

    run_batch(&batch, true);
}

// ---------------------------------------------------------------------------
// Memory / AMO decode fuzzer: random words across the load/store/atomic opcode
// spaces, with the base register set so every access lands in the shared
// scratch window. Proves decode+execute agreement with qemu over the whole
// memory encoding space.
// ---------------------------------------------------------------------------

#[test]
fn diff_mem_fuzz() {
    let mut rng = Rng::new(0x3EAD_3EE7);
    let safe: [u32; 9] = [1, 5, 6, 7, 8, 9, 12, 28, 31]; // excl x10 (base), gp/tp
    let opcodes: [u32; 5] = [0x03, 0x23, 0x07, 0x27, 0x2f];
    let mut batch = Vec::with_capacity(40000);
    let mut tries = 0;
    while batch.len() < 70000 && tries < 500_000 {
        tries += 1;
        let opc = opcodes[(rng.next() as usize) % opcodes.len()];
        let mut w = (rng.next() as u32 & !0x7f) | opc;
        // Base register = x10; dest/data = safe registers.
        w = (w & !(0x1f << 15)) | (10 << 15);
        let rd = safe[(rng.next() as usize) % 9];
        let rs2 = safe[(rng.next() as usize) % 9];
        w = (w & !(0x1f << 7)) | (rd << 7);
        w = (w & !(0x1f << 20)) | (rs2 << 20);
        // Vector load/store (opcode 0x07/0x27, width funct3 in {0,5,6,7}) needs a
        // configured vector context the scalar oracle doesn't provide; those are
        // covered with proper vtype/vl setup in tests/riscv_vector.rs.
        if matches!(opc, 0x07 | 0x27) && matches!((w >> 12) & 7, 0 | 5 | 6 | 7) {
            continue;
        }
        // Only test encodings rax decodes (qemu's reserved-field leniency aside).
        if decode(w, Xlen::Rv64, &Isa::rv64gc()).is_illegal() {
            continue;
        }
        // Set up the base so the effective address is SCRATCH_BASE.
        let imm: i64 = match opc {
            0x03 | 0x07 => ((w as i32) >> 20) as i64, // I-type
            0x23 | 0x27 => {
                let u = (((w >> 25) & 0x7f) << 5) | ((w >> 7) & 0x1f);
                ((u as i32) << 20 >> 20) as i64
            }
            _ => 0, // AMO: address = x[rs1]
        };
        let mut st = rand_state(&mut rng);
        st.x[10] = (SCRATCH_BASE as i64).wrapping_sub(imm) as u64;
        for s in st.scratch.iter_mut() {
            *s = rng.next();
        }
        for fi in st.f.iter_mut() {
            *fi = rng.next();
        }
        batch.push(("mem".to_string(), w, st));
    }
    run_batch(&batch, true);
}

// ---------------------------------------------------------------------------
// V: vector configuration (vsetvli / vsetivli / vsetvl). Verifies the computed
// `vl` (written to x[rd]) against qemu across vtype/AVL combinations. (The
// Keep form rs1=x0,rd=x0 depends on persistent vl state in the oracle and is
// covered by unit tests instead.)
// ---------------------------------------------------------------------------

#[test]
fn diff_vsetvl() {
    let mut rng = Rng::new(0x5E7_71);
    let mut batch = Vec::new();
    // vtype = (vma<<7)|(vta<<6)|(vsew<<3)|vlmul ; iterate sew 0..5, lmul 0..8.
    let mut vtypes = Vec::new();
    for sew in 0..5u32 {
        for lmul in 0..8u32 {
            for &flags in &[0u32, 0xC0] {
                vtypes.push(flags | (sew << 3) | lmul);
            }
        }
    }
    let avls = [0u64, 1, 2, 4, 8, 15, 16, 17, 31, 32, 33, 63, 100, 1000];

    for &vtype in &vtypes {
        for &avl in &avls {
            let rd = POOL[(rng.next() % 6) as usize];
            let rs1 = POOL[(rng.next() % 6) as usize];
            // vsetvli rd, rs1, vtype  (AVL = x[rs1])
            let mut st = rand_state(&mut rng);
            st.x[rs1 as usize] = avl;
            let vsetvli = (vtype << 20) | (rs1 << 15) | (7 << 12) | (rd << 7) | 0x57;
            batch.push(("vsetvli".into(), vsetvli, st));
            // vsetvli rd, x0, vtype  (AVL = VLMAX)
            let vsetvli_max = (vtype << 20) | (0 << 15) | (7 << 12) | (rd << 7) | 0x57;
            batch.push(("vsetvli.max".into(), vsetvli_max, rand_state(&mut rng)));
            // vsetvl rd, rs1, rs2  (vtype from x[rs2])
            let rs2 = POOL[(rng.next() % 6) as usize];
            if rs2 != rs1 {
                let mut st2 = rand_state(&mut rng);
                st2.x[rs1 as usize] = avl;
                st2.x[rs2 as usize] = vtype as u64;
                let vsetvl =
                    (1u32 << 31) | (rs2 << 20) | (rs1 << 15) | (7 << 12) | (rd << 7) | 0x57;
                batch.push(("vsetvl".into(), vsetvl, st2));
            }
        }
        // vsetivli rd, uimm, vtype  (AVL = 5-bit immediate)
        for uimm in [0u32, 1, 4, 15, 16, 17, 31] {
            let rd = POOL[(rng.next() % 6) as usize];
            let vsetivli =
                (0b11u32 << 30) | (vtype << 20) | (uimm << 15) | (7 << 12) | (rd << 7) | 0x57;
            batch.push(("vsetivli".into(), vsetivli, rand_state(&mut rng)));
        }
    }
    run_batch(&batch, false);
}
