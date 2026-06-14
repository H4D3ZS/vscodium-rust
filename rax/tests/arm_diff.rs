//! AArch64 differential test harness: rax interpreter vs. QEMU (hardware oracle).
//!
//! The rax software interpreter (`src/arm/aarch64/cpu.rs`) is checked against a
//! hardware-semantics reference produced by running each instruction under
//! `qemu-aarch64` (user mode). The reference harness is `tools/arm-diff/oracle.c`,
//! built on demand into a static AArch64 ELF.
//!
//! For each `(instruction, initial architectural state)` pair we:
//!   1. run it on the oracle (X0..X30, SP, NZCV, V0..V31 captured), and
//!   2. run it on the rax `AArch64Cpu` from the *identical* initial state,
//! then compare the full register file. Any divergence is an interpreter bug.
//!
//! Robustness (mirrors `tests/differential.rs` for x86-vs-KVM):
//!   - if the cross compiler or `qemu-aarch64` is unavailable, every test
//!     self-skips (returns without failing) so the suite is green anywhere.
//!   - only register-only instructions (no memory / branch / system) are tested;
//!     these are exactly where the SIMD/FP semantic bugs live.
//!
//! Scope is intentionally exhaustive *within* a family: encodings are enumerated
//! over their opcode/size/Q/U fields with fixed register fields, and many
//! pseudo-random input states are driven through each.

#![cfg(target_os = "linux")]

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use rax::arm::{AArch64Config, AArch64Cpu, ArmCpu, CpuExit, FlatMemory};
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::flags::{FlagSet, FlagUpdate};
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::ir::{FunctionBuilder, SmirFunction, Terminator};
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::lift::aarch64::Aarch64Lifter;
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::lift::{ControlFlow, LiftContext, SmirLifter};
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::lower::SmirLowerer;
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::lower::aarch64::Aarch64Lowerer;
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::lower::aarch64_x86::Aarch64X86_64Lowerer;
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::lower::runtime::{Aarch64GuestRegs, ExecMem};
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::ops::OpKind;
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
use rax::smir::types::{
    Address, ArchReg, ArmReg, AtomicOp, Condition, DispSize, ExtendOp, FenceKind, FunctionId,
    MemWidth, MemoryOrder, OpWidth, ShiftOp, SignExtend, SourceArch, SrcOperand, VReg,
};

// ---------------------------------------------------------------------------
// Wire format -- must match tools/arm-diff/oracle.c byte for byte.
// ---------------------------------------------------------------------------

/// Full architectural register file exchanged with the oracle.
#[repr(C)]
#[derive(Clone, Copy)]
struct ArmState {
    x: [u64; 31], // X0..X30
    sp: u64,      // SP
    pc: u64,      // input: unused; output: post-instruction PC
    pstate: u64,  // NZCV in bits [31:28]
    fpsr: u64,
    fpcr: u64,
    v: [u64; 64],       // V0..V31 as lo/hi u64 pairs
    scratch: [u64; 32], // shared scratch window (256 bytes) for load/store tests
    preds: [u64; 4],    // SVE P0..P15 packed as 16 x 16-bit (VL=128), byte-granular
}

/// AArch64 NOP (used to fill the oracle's unused second instruction slot).
const NOP: u32 = 0xd503201f;
/// Address of the shared scratch window (matches oracle.c SCRATCH_ADDR).
const SCRATCH_ADDR: u64 = 0x20_0000;
/// Base pointer tests aim a register at (matches oracle.c SCRATCH_BASE).
const SCRATCH_BASE: u64 = SCRATCH_ADDR + 64;
/// Enables oracle PC-relative register relocation for control-flow tests.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
const PCREL_MAGIC: u64 = 0x5241_5850_4352_454c;
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
const PCREL_TOKEN: u64 = 0x5241_5800_0000_0000;

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn pcrel_marker(offset: i32) -> u64 {
    PCREL_TOKEN | u64::from(offset as u32)
}

impl ArmState {
    fn zeroed() -> Self {
        ArmState {
            x: [0; 31],
            sp: 0,
            pc: 0,
            pstate: 0,
            fpsr: 0,
            fpcr: 0,
            v: [0; 64],
            scratch: [0; 32],
            preds: [0; 4],
        }
    }
    fn vreg(&self, r: usize) -> (u64, u64) {
        (self.v[2 * r], self.v[2 * r + 1])
    }
    fn set_vreg(&mut self, r: usize, lo: u64, hi: u64) {
        self.v[2 * r] = lo;
        self.v[2 * r + 1] = hi;
    }
    /// Read SVE predicate `r` (16 bits at VL=128) from the packed `preds`.
    fn preg(&self, r: usize) -> u16 {
        (self.preds[r / 4] >> (16 * (r % 4))) as u16
    }
    /// Write SVE predicate `r` (16 bits at VL=128) into the packed `preds`.
    fn set_preg(&mut self, r: usize, v: u16) {
        let shift = 16 * (r % 4);
        self.preds[r / 4] = (self.preds[r / 4] & !(0xFFFFu64 << shift)) | ((v as u64) << shift);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InCase {
    insn: u32,
    flags: u32,
    insn3: u32,
    reserved: u32,
    st: ArmState,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct OutCase {
    st: ArmState,
    trapped: u32,
    valid: u32,
}

const WIRE_MAGIC: u32 = 0x314d_5241; // 'A','R','M','1'

// Compile-time guarantee the layout matches the C side (preds[4] adds 32 bytes).
const _: () = assert!(core::mem::size_of::<ArmState>() == 1088);
const _: () = assert!(core::mem::size_of::<InCase>() == 1104);
const _: () = assert!(core::mem::size_of::<OutCase>() == 1096);

// ---------------------------------------------------------------------------
// Byte (de)serialisation helpers -- plain little-endian copies of the structs.
// ---------------------------------------------------------------------------

fn as_bytes<T: Copy>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
}

fn read_struct<T: Copy>(buf: &[u8], off: usize) -> T {
    assert!(off + std::mem::size_of::<T>() <= buf.len());
    unsafe { std::ptr::read_unaligned(buf[off..].as_ptr() as *const T) }
}

// ---------------------------------------------------------------------------
// Oracle: build on demand, then run a whole batch through one qemu invocation.
// ---------------------------------------------------------------------------

/// Build the oracle if needed; return its path, or `None` if the toolchain is
/// unavailable (test self-skips).
fn oracle_path() -> Option<PathBuf> {
    if which("qemu-aarch64").is_none() {
        return None;
    }
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/arm-diff");
    let bin = dir.join("oracle");
    let need_build = match (bin.metadata(), dir.join("oracle.c").metadata()) {
        (Ok(b), Ok(c)) => match (b.modified(), c.modified()) {
            (Ok(bm), Ok(cm)) => bm < cm,
            _ => true,
        },
        _ => true,
    };
    if need_build {
        let status = Command::new("bash")
            .arg(dir.join("build.sh"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        match status {
            Ok(s) if s.success() => {}
            _ => return None, // cross compiler absent -> skip
        }
    }
    if bin.exists() { Some(bin) } else { None }
}

fn which(prog: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let cand = dir.join(prog);
        if cand.is_file() {
            return Some(cand);
        }
    }
    None
}

/// Run `cases` through the oracle under qemu; returns one `OutCase` per input.
fn run_oracle(oracle: &PathBuf, cases: &[(u32, u32, ArmState)]) -> Option<Vec<OutCase>> {
    let cases3: Vec<(u32, u32, u32, ArmState)> = cases
        .iter()
        .map(|(insn, insn2, st)| (*insn, *insn2, NOP, *st))
        .collect();
    run_oracle3(oracle, &cases3)
}

/// Run three-instruction cases through the oracle under qemu.
fn run_oracle3(oracle: &PathBuf, cases: &[(u32, u32, u32, ArmState)]) -> Option<Vec<OutCase>> {
    let mut payload = Vec::with_capacity(8 + cases.len() * std::mem::size_of::<InCase>());
    payload.extend_from_slice(&WIRE_MAGIC.to_le_bytes());
    payload.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for (insn, insn2, insn3, st) in cases {
        let ic = InCase {
            insn: *insn,
            flags: *insn2,
            insn3: *insn3,
            reserved: 0,
            st: *st,
        };
        payload.extend_from_slice(as_bytes(&ic));
    }

    let mut child = Command::new("qemu-aarch64")
        .arg(oracle)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    // The oracle interleaves reads and writes, so feeding the whole batch before
    // draining stdout would deadlock once both pipe buffers fill. Write from a
    // dedicated thread while the main thread drains stdout.
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(&payload);
        // drop(stdin) closes the pipe, signalling EOF to the oracle
    });
    let mut out = Vec::new();
    child.stdout.take().unwrap().read_to_end(&mut out).ok()?;
    let _ = writer.join();
    let status = child.wait().ok()?;
    if !status.success() {
        return None;
    }
    if out.len() < 8 {
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

/// Returns `Some(out_state)` if rax executed the instruction (CpuExit::Continue),
/// or `None` if rax treated it as undefined / errored.
fn run_rax(insn: u32, input: &ArmState) -> Option<ArmState> {
    // Memory must cover both the instruction (at 0) and the scratch window.
    let mem = FlatMemory::new(0, 0x30_0000);
    let mut cpu = AArch64Cpu::new(AArch64Config::default(), Box::new(mem));

    for i in 0..31u8 {
        cpu.set_gpr(i, input.x[i as usize]);
    }
    cpu.set_current_sp(input.sp);
    let ps = input.pstate;
    cpu.set_nzcv(
        ps & (1 << 31) != 0,
        ps & (1 << 30) != 0,
        ps & (1 << 29) != 0,
        ps & (1 << 28) != 0,
    );
    for r in 0..32u8 {
        let (lo, hi) = input.vreg(r as usize);
        cpu.set_simd_reg(r, lo, hi).ok()?;
    }
    for r in 0..16usize {
        cpu.set_sve_pred(r, input.preg(r) as u32);
    }

    // Install the scratch window at SCRATCH_ADDR.
    let scratch_bytes: Vec<u8> = input.scratch.iter().flat_map(|w| w.to_le_bytes()).collect();
    cpu.write_memory(SCRATCH_ADDR, &scratch_bytes).ok()?;

    cpu.write_memory(0, &insn.to_le_bytes()).ok()?;
    cpu.set_pc(0);

    match cpu.step() {
        Ok(CpuExit::Continue) => {}
        _ => return None,
    }

    let mut out = ArmState::zeroed();
    for i in 0..31u8 {
        out.x[i as usize] = cpu.get_gpr(i);
    }
    out.sp = cpu.current_sp();
    out.pc = cpu.get_pc();
    let mut pstate = 0u64;
    if cpu.get_n() {
        pstate |= 1 << 31;
    }
    if cpu.get_z() {
        pstate |= 1 << 30;
    }
    if cpu.get_c() {
        pstate |= 1 << 29;
    }
    if cpu.get_v() {
        pstate |= 1 << 28;
    }
    out.pstate = pstate;
    for r in 0..32u8 {
        if let Some((lo, hi)) = cpu.get_simd_reg(r) {
            out.set_vreg(r as usize, lo, hi);
        }
    }
    // Read the scratch window back.
    for (i, w) in out.scratch.iter_mut().enumerate() {
        *w = cpu.mem_read_u64(SCRATCH_ADDR + (i as u64) * 8).ok()?;
    }
    for r in 0..16usize {
        out.set_preg(r, cpu.sve_pred(r) as u16);
    }
    Some(out)
}

// ---------------------------------------------------------------------------
// Comparison.
// ---------------------------------------------------------------------------

/// Description of a single divergence between rax and the oracle.
struct Mismatch {
    label: String,
    insn: u32,
    detail: String,
}

/// Compare one case. `cmp_flags` selects whether NZCV is compared (only for
/// flag-setting instructions; otherwise both should be unchanged anyway).
fn compare_case(
    label: &str,
    insn: u32,
    input: &ArmState,
    oracle: &OutCase,
    mismatches: &mut Vec<Mismatch>,
) {
    let rax = run_rax(insn, input);

    // Agreement on legality first.
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
        return; // both reject -> fine; nothing to value-compare
    }
    let rax = match rax {
        Some(s) => s,
        None => {
            mismatches.push(Mismatch {
                label: label.into(),
                insn,
                detail: "hw executed but rax rejected the encoding (undefined/err)".into(),
            });
            return;
        }
    };

    let mut diffs = Vec::new();
    for i in 0..31 {
        if rax.x[i] != oracle.st.x[i] {
            diffs.push(format!(
                "x{i}: rax={:#018x} hw={:#018x}",
                rax.x[i], oracle.st.x[i]
            ));
        }
    }
    if rax.sp != oracle.st.sp {
        diffs.push(format!(
            "sp: rax={:#018x} hw={:#018x}",
            rax.sp, oracle.st.sp
        ));
    }
    let rax_nzcv = (rax.pstate >> 28) & 0xF;
    let hw_nzcv = (oracle.st.pstate >> 28) & 0xF;
    if rax_nzcv != hw_nzcv {
        diffs.push(format!("nzcv: rax={:#x} hw={:#x}", rax_nzcv, hw_nzcv));
    }
    for r in 0..32 {
        let (rlo, rhi) = rax.vreg(r);
        let (hlo, hhi) = oracle.st.vreg(r);
        if (rlo, rhi) != (hlo, hhi) {
            diffs.push(format!(
                "v{r}: rax={:#018x}{:016x} hw={:#018x}{:016x}",
                rhi, rlo, hhi, hlo
            ));
        }
    }
    for i in 0..32 {
        if rax.scratch[i] != oracle.st.scratch[i] {
            diffs.push(format!(
                "scratch[{i}]: rax={:#018x} hw={:#018x}",
                rax.scratch[i], oracle.st.scratch[i]
            ));
        }
    }
    for r in 0..16 {
        if rax.preg(r) != oracle.st.preg(r) {
            diffs.push(format!(
                "p{r}: rax={:#06x} hw={:#06x}",
                rax.preg(r),
                oracle.st.preg(r)
            ));
        }
    }

    if !diffs.is_empty() {
        mismatches.push(Mismatch {
            label: label.into(),
            insn,
            detail: diffs.join("  |  "),
        });
    }
}

// ---------------------------------------------------------------------------
// Deterministic pseudo-random input generation.
// ---------------------------------------------------------------------------

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed ^ 0x9e37_79b9_7f4a_7c15)
    }
    fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    /// A value biased toward "interesting" bit patterns.
    fn interesting(&mut self) -> u64 {
        match self.next() % 8 {
            0 => 0,
            1 => u64::MAX,
            2 => 1,
            3 => 0x8000_0000_0000_0000,
            4 => 0x0000_0000_8000_0000,
            5 => self.next() & 0xFF,
            6 => self.next() & 0xFFFF_FFFF,
            _ => self.next(),
        }
    }
}

/// Build a randomised input state. GPRs x1.. and V1.. carry operands; the
/// destination registers are also randomised so unintended clobbers show up.
fn gen_input(rng: &mut Rng) -> ArmState {
    let mut st = ArmState::zeroed();
    for i in 0..31 {
        st.x[i] = rng.interesting();
    }
    st.sp = rng.interesting() & !0xF; // keep 16-aligned-ish; sp not dereferenced
    st.pstate = (rng.next() & 0xF) << 28; // random NZCV
    st.fpcr = 0; // round-to-nearest, no flush -- matches rax default
    st.fpsr = 0;
    for r in 0..32 {
        st.set_vreg(r, rng.interesting(), rng.interesting());
    }
    st
}

// ---------------------------------------------------------------------------
// Instruction family encoders.  Register fields are fixed: Rd=0, Rn=1, Rm=2,
// Ra=3.  Inputs vary the register *values*, exercising the data path.
// ---------------------------------------------------------------------------

const RD: u32 = 0;
const RN: u32 = 1;
const RM: u32 = 2;
const RA: u32 = 3;

/// Advanced SIMD three-same (integer + FP), main encoding:
/// `0 Q U 01110 size 1 Rm opcode 1 Rn Rd`
fn enc_three_same(q: u32, u: u32, size: u32, opcode: u32) -> u32 {
    (q << 30)
        | (u << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (1 << 21)
        | (RM << 16)
        | (opcode << 11)
        | (1 << 10)
        | (RN << 5)
        | RD
}

/// Build the full list of three-same encodings to test (over-generates; illegal
/// combos are filtered by oracle agreement).
fn three_same_cases() -> Vec<(String, u32)> {
    let mut v = Vec::new();
    for q in 0..2 {
        for u in 0..2 {
            for size in 0..4 {
                // Integer opcodes 0x00..=0x17. FP opcodes (0x18..=0x1F) are
                // exercised separately with NaN-aware comparison.
                for opcode in 0..0b11000u32 {
                    let insn = enc_three_same(q, u, size, opcode);
                    v.push((format!("3same q{q} u{u} sz{size} op{:05b}", opcode), insn));
                }
            }
        }
    }
    v
}

/// Data-processing (2 source): `sf 0 0 11010110 Rm opcode2 Rn Rd`
fn enc_dp2_regs(sf: u32, opcode2: u32, rn: u32, rm: u32, rd: u32) -> u32 {
    (sf << 31)
        | (0b0011010110 << 21)
        | ((rm & 0x1f) << 16)
        | (opcode2 << 10)
        | ((rn & 0x1f) << 5)
        | (rd & 0x1f)
}

fn enc_dp2(sf: u32, opcode2: u32) -> u32 {
    enc_dp2_regs(sf, opcode2, RN, RM, RD)
}

fn enc_crc32_x(castagnoli: bool) -> u32 {
    enc_dp2(1, if castagnoli { 0b010111 } else { 0b010011 })
}

fn dp2_cases() -> Vec<(String, u32)> {
    let mut v = Vec::new();
    // opcode2: 0010=UDIV 0011=SDIV 1000=LSLV 1001=LSRV 1010=ASRV 1011=RORV
    for &op in &[0b0010u32, 0b0011, 0b1000, 0b1001, 0b1010, 0b1011] {
        for sf in 0..2 {
            v.push((format!("dp2 sf{sf} op{:04b}", op), enc_dp2(sf, op)));
        }
    }
    v
}

/// Data-processing (1 source): `sf 1 0 11010110 00000 opcode Rn Rd`
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_dp1_regs(sf: u32, opcode: u32, rn: u32, rd: u32) -> u32 {
    (sf << 31) | (0b1011010110 << 21) | (opcode << 10) | (rn << 5) | rd
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_dp1(sf: u32, opcode: u32) -> u32 {
    enc_dp1_regs(sf, opcode, RN, RD)
}

/// Bitfield: `sf opc 100110 N immr imms Rn Rd`
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_bitfield(sf: u32, opc: u32, immr: u32, imms: u32) -> u32 {
    enc_bitfield_rn(sf, opc, immr, imms, RN)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_bitfield_rn(sf: u32, opc: u32, immr: u32, imms: u32, rn: u32) -> u32 {
    enc_bitfield_regs(sf, opc, immr, imms, rn, RD)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_bitfield_regs(sf: u32, opc: u32, immr: u32, imms: u32, rn: u32, rd: u32) -> u32 {
    (sf << 31)
        | (opc << 29)
        | (0b100110 << 23)
        | (sf << 22)
        | (immr << 16)
        | (imms << 10)
        | (rn << 5)
        | rd
}

/// Logical immediate: `sf opc 100100 N immr imms Rn Rd`
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_logical_imm_regs(sf: u32, opc: u32, n: u32, immr: u32, imms: u32, rn: u32, rd: u32) -> u32 {
    (sf << 31)
        | (opc << 29)
        | (0b100100 << 23)
        | (n << 22)
        | (immr << 16)
        | (imms << 10)
        | (rn << 5)
        | rd
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_logical_imm(sf: u32, opc: u32, n: u32, immr: u32, imms: u32, rn: u32) -> u32 {
    enc_logical_imm_regs(sf, opc, n, immr, imms, rn, RD)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_logical_reg_n(sf: u32, opc: u32, n: u32, rd: u32, rn: u32, rm: u32) -> u32 {
    (sf << 31)
        | (opc << 29)
        | (0b01010 << 24)
        | (n << 21)
        | ((rm & 0x1f) << 16)
        | ((rn & 0x1f) << 5)
        | (rd & 0x1f)
}

/// Extract: `sf 00 100111 N 0 Rm imms Rn Rd`
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_extract(sf: u32, rn: u32, rm: u32, imms: u32) -> u32 {
    (sf << 31) | (0b100111 << 23) | (sf << 22) | (rm << 16) | (imms << 10) | (rn << 5) | RD
}

/// Data-processing (3 source): `sf 00 11011 op31 Rm o0 Ra Rn Rd`
fn enc_dp3(sf: u32, op31: u32, o0: u32) -> u32 {
    enc_dp3_ra(sf, op31, o0, RA)
}

/// Same, but with an explicit Ra field (SMULH/UMULH require Ra = 31/xzr).
fn enc_dp3_ra(sf: u32, op31: u32, o0: u32, ra: u32) -> u32 {
    enc_dp3_ra_regs(sf, op31, o0, RD, RN, RM, ra)
}

fn enc_dp3_ra_regs(sf: u32, op31: u32, o0: u32, rd: u32, rn: u32, rm: u32, ra: u32) -> u32 {
    (sf << 31)
        | (0b11011 << 24)
        | (op31 << 21)
        | ((rm & 0x1f) << 16)
        | (o0 << 15)
        | ((ra & 0x1f) << 10)
        | ((rn & 0x1f) << 5)
        | (rd & 0x1f)
}

fn dp3_cases() -> Vec<(String, u32)> {
    let mut v = Vec::new();
    // MADD: op31=000 o0=0 ; MSUB: op31=000 o0=1
    v.push(("madd".into(), enc_dp3(1, 0b000, 0)));
    v.push(("msub".into(), enc_dp3(1, 0b000, 1)));
    v.push(("madd_w".into(), enc_dp3(0, 0b000, 0)));
    v.push(("msub_w".into(), enc_dp3(0, 0b000, 1)));
    // SMADDL: op31=001 o0=0 ; SMSUBL: 001 o0=1 ; UMADDL: 101 o0=0 ; UMSUBL: 101 o0=1
    v.push(("smaddl".into(), enc_dp3(1, 0b001, 0)));
    v.push(("smsubl".into(), enc_dp3(1, 0b001, 1)));
    v.push(("umaddl".into(), enc_dp3(1, 0b101, 0)));
    v.push(("umsubl".into(), enc_dp3(1, 0b101, 1)));
    // SMULH: op31=010 o0=0 ; UMULH: 110 o0=0 (Ra must be 31/xzr)
    v.push(("smulh".into(), enc_dp3_ra(1, 0b010, 0, 31)));
    v.push(("umulh".into(), enc_dp3_ra(1, 0b110, 0, 31)));
    v
}

/// Add/subtract (shifted register): `sf op S 01011 shift 0 Rm imm6 Rn Rd`
fn enc_addsub_shift_regs(
    sf: u32,
    op: u32,
    s: u32,
    shift: u32,
    imm6: u32,
    rd: u32,
    rn: u32,
    rm: u32,
) -> u32 {
    (sf << 31)
        | (op << 30)
        | (s << 29)
        | (0b01011 << 24)
        | (shift << 22)
        | ((rm & 0x1F) << 16)
        | (imm6 << 10)
        | ((rn & 0x1F) << 5)
        | (rd & 0x1F)
}

fn enc_addsub_shift(sf: u32, op: u32, s: u32, shift: u32, imm6: u32) -> u32 {
    enc_addsub_shift_regs(sf, op, s, shift, imm6, RD, RN, RM)
}

/// Add/subtract (immediate): `sf op S 10001 sh imm12 Rn Rd`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_addsub_imm_regs(sf: u32, op: u32, s: u32, shift: u32, imm12: u32, rd: u32, rn: u32) -> u32 {
    (sf << 31)
        | (op << 30)
        | (s << 29)
        | (0b10001 << 24)
        | ((shift & 1) << 22)
        | ((imm12 & 0xfff) << 10)
        | ((rn & 0x1f) << 5)
        | (rd & 0x1f)
}

/// Add/subtract (extended register): `sf op S 01011 00 1 Rm option imm3 Rn Rd`
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_addsub_ext_regs(
    sf: u32,
    op: u32,
    s: u32,
    option: u32,
    imm3: u32,
    rd: u32,
    rn: u32,
    rm: u32,
) -> u32 {
    (sf << 31)
        | (op << 30)
        | (s << 29)
        | (0b01011 << 24)
        | (1 << 21)
        | ((rm & 0x1f) << 16)
        | (option << 13)
        | (imm3 << 10)
        | ((rn & 0x1f) << 5)
        | (rd & 0x1f)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_addsub_ext(sf: u32, op: u32, s: u32, option: u32, imm3: u32) -> u32 {
    enc_addsub_ext_regs(sf, op, s, option, imm3, RD, RN, RM)
}

fn addsub_shift_cases() -> Vec<(String, u32)> {
    let mut v = Vec::new();
    for sf in 0..2 {
        for op in 0..2 {
            for s in 0..2 {
                for shift in 0..3 {
                    // LSL/LSR/ASR
                    for &imm6 in &[0u32, 1, 7, 31, if sf == 1 { 63 } else { 31 }] {
                        v.push((
                            format!("addsub sf{sf} op{op} s{s} sh{shift} #{imm6}"),
                            enc_addsub_shift(sf, op, s, shift, imm6),
                        ));
                    }
                }
            }
        }
    }
    v
}

/// Add/subtract with carry: `sf op S 11010000 Rm 000000 Rn Rd`
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_addsub_carry(sf: u32, op: u32, s: u32) -> u32 {
    enc_addsub_carry_rn(sf, op, s, RN)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_addsub_carry_rn(sf: u32, op: u32, s: u32, rn: u32) -> u32 {
    enc_addsub_carry_regs(sf, op, s, RD, rn, RM)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_addsub_carry_regs(sf: u32, op: u32, s: u32, rd: u32, rn: u32, rm: u32) -> u32 {
    (sf << 31) | (op << 30) | (s << 29) | (0b11010000 << 21) | (rm << 16) | (rn << 5) | rd
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_addsub_tags(op: u32, uimm6: u32, uimm4: u32) -> u32 {
    enc_addsub_tags_regs(op, uimm6, uimm4, RN, RD)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_addsub_tags_regs(op: u32, uimm6: u32, uimm4: u32, rn: u32, rd: u32) -> u32 {
    (1 << 31)
        | (op << 30)
        | (0b100011 << 23)
        | ((uimm6 & 0x3f) << 16)
        | ((uimm4 & 0xf) << 10)
        | ((rn & 0x1f) << 5)
        | (rd & 0x1f)
}

/// Conditional compare: `sf op 111010010 Rm/imm5 cond imm 0 Rn 0 nzcv`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_condcmp(sf: u32, op: u32, imm: bool, rm_imm5: u32, cond: u32, nzcv: u32) -> u32 {
    (sf << 31)
        | (op << 30)
        | (0b111010010 << 21)
        | (rm_imm5 << 16)
        | (cond << 12)
        | ((imm as u32) << 11)
        | (RN << 5)
        | nzcv
}

/// Test and branch: `b5 011011 op b40 imm14 Rt`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_test_branch_rt(op: u32, bit: u32, offset: i32, rt: u32) -> u32 {
    let b5 = (bit >> 5) & 1;
    let b40 = bit & 0x1F;
    let imm14 = ((offset >> 2) as u32) & 0x3FFF;
    (b5 << 31) | (0b011011 << 25) | (op << 24) | (b40 << 19) | (imm14 << 5) | (rt & 0x1f)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_test_branch(op: u32, bit: u32, offset: i32) -> u32 {
    enc_test_branch_rt(op, bit, offset, RN)
}

/// Compare and branch: `sf 011010 op imm19 Rt`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_compare_branch(sf: u32, op: u32, rt: u32, offset: i32) -> u32 {
    let imm19 = ((offset >> 2) as u32) & 0x7FFFF;
    (sf << 31) | (0b011010 << 25) | (op << 24) | (imm19 << 5) | (rt & 0x1F)
}

/// Conditional branch: `01010100 imm19 0 cond`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_cond_branch(cond: u32, offset: i32) -> u32 {
    let imm19 = ((offset >> 2) as u32) & 0x7FFFF;
    0x5400_0000 | (imm19 << 5) | (cond & 0xF)
}

/// Branch with link: `100101 imm26`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_bl(offset: i32) -> u32 {
    0x9400_0000 | (((offset >> 2) as u32) & 0x03ff_ffff)
}

/// Branch to register: `1101011 0000 11111 000000 Rn 00000`.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_br(rn: u32) -> u32 {
    0xd61f_0000 | ((rn & 0x1f) << 5)
}

/// Branch with link to register.
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_blr(rn: u32) -> u32 {
    0xd63f_0000 | ((rn & 0x1f) << 5)
}

/// Return from subroutine through `rn` (`rn == 30` is plain RET).
#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_ret(rn: u32) -> u32 {
    0xd65f_0000 | ((rn & 0x1f) << 5)
}

/// Logical (shifted register): `sf opc 01010 shift N Rm imm6 Rn Rd`
fn enc_logical_shift_regs(
    sf: u32,
    opc: u32,
    shift: u32,
    n: u32,
    imm6: u32,
    rd: u32,
    rn: u32,
    rm: u32,
) -> u32 {
    (sf << 31)
        | (opc << 29)
        | (0b01010 << 24)
        | (shift << 22)
        | (n << 21)
        | ((rm & 0x1F) << 16)
        | (imm6 << 10)
        | ((rn & 0x1F) << 5)
        | (rd & 0x1F)
}

fn enc_logical_shift(sf: u32, opc: u32, shift: u32, n: u32, imm6: u32) -> u32 {
    enc_logical_shift_regs(sf, opc, shift, n, imm6, RD, RN, RM)
}

fn enc_mov_reg(sf: u32, rd: u32, rm: u32) -> u32 {
    enc_logical_shift_regs(sf, 0b01, 0, 0, 0, rd, 31, rm)
}

fn logical_shift_cases() -> Vec<(String, u32)> {
    let mut v = Vec::new();
    for sf in 0..2 {
        for opc in 0..4 {
            // AND/ORR/EOR/ANDS
            for n in 0..2 {
                for shift in 0..4 {
                    for &imm6 in &[0u32, 1, 31, if sf == 1 { 63 } else { 31 }] {
                        v.push((
                            format!("logical sf{sf} opc{opc} n{n} sh{shift} #{imm6}"),
                            enc_logical_shift(sf, opc, shift, n, imm6),
                        ));
                    }
                }
            }
        }
    }
    v
}

// ---------------------------------------------------------------------------
// Test driver.
// ---------------------------------------------------------------------------

/// Run a family of encodings, each with `n_inputs` random states, against the
/// oracle and assert no divergences.
fn run_family(name: &str, cases: Vec<(String, u32)>, n_inputs: usize, seed: u64) {
    // Build the full batch of (label, insn, state) triples.
    let mut rng = Rng::new(seed);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..n_inputs {
            let st = gen_input(&mut rng);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch(name, batch);
}

/// Run an explicit batch of (label, insn, input-state) triples against the
/// oracle and assert no divergences. Self-skips if the toolchain is absent.
fn run_batch(name: &str, batch: Vec<(String, u32, ArmState)>) {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!("[arm_diff] {name}: qemu/cross-toolchain unavailable -> skipping");
            return;
        }
    };

    let labels: Vec<String> = batch.iter().map(|(l, _, _)| l.clone()).collect();
    let cases: Vec<(u32, u32, ArmState)> = batch.iter().map(|(_, i, s)| (*i, NOP, *s)).collect();

    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!("[arm_diff] {name}: oracle run failed -> skipping");
            return;
        }
    };
    assert_eq!(outs.len(), cases.len());

    let mut mismatches = Vec::new();
    for (i, ((insn, _insn2, st), out)) in cases.iter().zip(outs.iter()).enumerate() {
        compare_case(&labels[i], *insn, st, out, &mut mismatches);
    }

    let batch = &cases; // for the count in the summary below
    if !mismatches.is_empty() {
        // Summarise: group by label, show a few representative cases.
        use std::collections::BTreeMap;
        let mut by_label: BTreeMap<String, usize> = BTreeMap::new();
        for m in &mismatches {
            *by_label.entry(m.label.clone()).or_default() += 1;
        }
        eprintln!(
            "\n==== {name}: {} mismatches across {} cases ====",
            mismatches.len(),
            batch.len()
        );
        eprintln!("-- by encoding --");
        for (label, count) in &by_label {
            eprintln!("  {count:5}x  {label}");
        }
        eprintln!("-- first 25 examples --");
        for m in mismatches.iter().take(25) {
            eprintln!("  [{}] {:#010x}: {}", m.label, m.insn, m.detail);
        }
        panic!(
            "{name}: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn arm_to_smir_regs(st: &ArmState) -> Aarch64GuestRegs {
    let mut regs = Aarch64GuestRegs::default();
    regs.x = st.x;
    regs.sp = st.sp;
    regs.pc = st.pc;
    regs.nzcv = st.pstate & 0xF000_0000;
    regs.fpcr = st.fpcr & 0xffff_ffff;
    regs.fpsr = st.fpsr & 0xffff_ffff;
    regs.v = st.v;
    regs
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
struct SmirA64Mem {
    bytes: [u8; 256],
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
impl SmirA64Mem {
    fn from_state(st: &ArmState) -> Self {
        let mut bytes = [0u8; 256];
        for (idx, word) in st.scratch.iter().enumerate() {
            bytes[idx * 8..idx * 8 + 8].copy_from_slice(&word.to_le_bytes());
        }
        Self { bytes }
    }

    fn scratch_words(&self) -> [u64; 32] {
        let mut words = [0u64; 32];
        for (idx, word) in words.iter_mut().enumerate() {
            let off = idx * 8;
            *word = u64::from_le_bytes([
                self.bytes[off],
                self.bytes[off + 1],
                self.bytes[off + 2],
                self.bytes[off + 3],
                self.bytes[off + 4],
                self.bytes[off + 5],
                self.bytes[off + 6],
                self.bytes[off + 7],
            ]);
        }
        words
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn smir_scratch_offset(addr: u64, size: usize) -> Option<usize> {
    let off = addr.checked_sub(SCRATCH_ADDR)? as usize;
    (off + size <= 256).then_some(off)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
unsafe extern "C" fn smir_a64_mem_load(ctx: u64, addr: u64, size: u64, signed: u64) -> u64 {
    let mem = unsafe { &*(ctx as *const SmirA64Mem) };
    let size = size as usize;
    let Some(off) = smir_scratch_offset(addr, size) else {
        return 0;
    };
    let mut value = 0u64;
    for idx in 0..size {
        value |= u64::from(mem.bytes[off + idx]) << (idx * 8);
    }
    if signed != 0 && size < 8 {
        let bits = size * 8;
        let sign_bit = 1u64 << (bits - 1);
        if (value & sign_bit) != 0 {
            value |= u64::MAX << bits;
        }
    }
    value
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
unsafe extern "C" fn smir_a64_mem_store(ctx: u64, addr: u64, value: u64, size: u64) -> u64 {
    let mem = unsafe { &mut *(ctx as *mut SmirA64Mem) };
    let size = size as usize;
    let Some(off) = smir_scratch_offset(addr, size) else {
        return 0;
    };
    for idx in 0..size {
        mem.bytes[off + idx] = (value >> (idx * 8)) as u8;
    }
    1
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn run_smir_aarch64_x86_one(insn: u32, input: &ArmState) -> Result<Aarch64GuestRegs, String> {
    let mut lifter = Aarch64Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::Aarch64);
    let bytes = insn.to_le_bytes();
    let lifted = lifter
        .lift_insn(0, &bytes, &mut ctx)
        .map_err(|e| format!("lift failed: {e:?}"))?;
    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for op in lifted.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {
            builder.set_terminator(Terminator::Return { values: vec![] });
        }
        other => return Err(format!("unexpected control flow: {other:?}")),
    }
    let func = builder.finish();

    let mut lowerer = Aarch64X86_64Lowerer::new();
    let result = lowerer
        .lower_function(&func)
        .map_err(|e| format!("lower failed: {e:?}"))?;
    let code = lowerer
        .finalize()
        .map_err(|e| format!("finalize failed: {e:?}"))?;
    let mem = ExecMem::new(&code).map_err(|e| format!("exec memory failed: {e:?}"))?;

    let mut regs = arm_to_smir_regs(input);
    mem.run_aarch64(result.entry_offset, &mut regs);
    Ok(regs)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn run_smir_aarch64_x86_mem_one(
    insn: u32,
    input: &ArmState,
) -> Result<(Aarch64GuestRegs, [u64; 32]), String> {
    let mut lifter = Aarch64Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::Aarch64);
    let bytes = insn.to_le_bytes();
    let lifted = lifter
        .lift_insn(0, &bytes, &mut ctx)
        .map_err(|e| format!("lift failed: {e:?}"))?;
    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for op in lifted.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {
            builder.set_terminator(Terminator::Return { values: vec![] });
        }
        other => return Err(format!("unexpected control flow: {other:?}")),
    }
    let func = builder.finish();

    let mut lowerer = Aarch64X86_64Lowerer::new();
    let result = lowerer
        .lower_function(&func)
        .map_err(|e| format!("lower failed: {e:?}"))?;
    let code = lowerer
        .finalize()
        .map_err(|e| format!("finalize failed: {e:?}"))?;
    let mem = ExecMem::new(&code).map_err(|e| format!("exec memory failed: {e:?}"))?;

    let mut scratch = SmirA64Mem::from_state(input);
    let mut regs = arm_to_smir_regs(input);
    regs.ctx = &mut scratch as *mut SmirA64Mem as usize as u64;
    regs.load_fn = smir_a64_mem_load as *const () as usize as u64;
    regs.store_fn = smir_a64_mem_store as *const () as usize as u64;
    mem.run_aarch64(result.entry_offset, &mut regs);
    Ok((regs, scratch.scratch_words()))
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn run_smir_aarch64_x86_mem_pair(
    insn: u32,
    insn2: u32,
    input: &ArmState,
) -> Result<(Aarch64GuestRegs, [u64; 32]), String> {
    let mut lifter = Aarch64Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::Aarch64);
    let lifted = lifter
        .lift_insn(0, &insn.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("first lift failed: {e:?}"))?;
    let lifted2 = lifter
        .lift_insn(4, &insn2.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("second lift failed: {e:?}"))?;

    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for op in lifted.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {}
        other => return Err(format!("unexpected first control flow: {other:?}")),
    }
    for op in lifted2.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted2.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {
            builder.set_terminator(Terminator::Return { values: vec![] });
        }
        other => return Err(format!("unexpected second control flow: {other:?}")),
    }
    let func = builder.finish();

    let mut lowerer = Aarch64X86_64Lowerer::new();
    let result = lowerer
        .lower_function(&func)
        .map_err(|e| format!("lower failed: {e:?}"))?;
    let code = lowerer
        .finalize()
        .map_err(|e| format!("finalize failed: {e:?}"))?;
    let mem = ExecMem::new(&code).map_err(|e| format!("exec mem failed: {e}"))?;

    let mut scratch = SmirA64Mem::from_state(input);
    let mut regs = arm_to_smir_regs(input);
    regs.ctx = &mut scratch as *mut SmirA64Mem as usize as u64;
    regs.load_fn = smir_a64_mem_load as *const () as usize as u64;
    regs.store_fn = smir_a64_mem_store as *const () as usize as u64;
    mem.run_aarch64(result.entry_offset, &mut regs);
    Ok((regs, scratch.scratch_words()))
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn run_smir_aarch64_x86_mem_three(
    insn: u32,
    insn2: u32,
    insn3: u32,
    input: &ArmState,
) -> Result<(Aarch64GuestRegs, [u64; 32]), String> {
    let mut lifter = Aarch64Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::Aarch64);
    let lifted = lifter
        .lift_insn(0, &insn.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("first lift failed: {e:?}"))?;
    let lifted2 = lifter
        .lift_insn(4, &insn2.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("second lift failed: {e:?}"))?;
    let lifted3 = lifter
        .lift_insn(8, &insn3.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("third lift failed: {e:?}"))?;

    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for op in lifted.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {}
        other => return Err(format!("unexpected first control flow: {other:?}")),
    }
    for op in lifted2.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted2.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {}
        other => return Err(format!("unexpected second control flow: {other:?}")),
    }
    for op in lifted3.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted3.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {
            builder.set_terminator(Terminator::Return { values: vec![] });
        }
        other => return Err(format!("unexpected third control flow: {other:?}")),
    }
    let func = builder.finish();

    let mut lowerer = Aarch64X86_64Lowerer::new();
    let result = lowerer
        .lower_function(&func)
        .map_err(|e| format!("lower failed: {e:?}"))?;
    let code = lowerer
        .finalize()
        .map_err(|e| format!("finalize failed: {e:?}"))?;
    let mem = ExecMem::new(&code).map_err(|e| format!("exec mem failed: {e}"))?;

    let mut scratch = SmirA64Mem::from_state(input);
    let mut regs = arm_to_smir_regs(input);
    regs.ctx = &mut scratch as *mut SmirA64Mem as usize as u64;
    regs.load_fn = smir_a64_mem_load as *const () as usize as u64;
    regs.store_fn = smir_a64_mem_store as *const () as usize as u64;
    mem.run_aarch64(result.entry_offset, &mut regs);
    Ok((regs, scratch.scratch_words()))
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn run_smir_aarch64_x86_pair(
    insn: u32,
    insn2: u32,
    input: &ArmState,
) -> Result<Aarch64GuestRegs, String> {
    let mut lifter = Aarch64Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::Aarch64);
    let lifted = lifter
        .lift_insn(0, &insn.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("first lift failed: {e:?}"))?;
    let lifted2 = lifter
        .lift_insn(4, &insn2.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("second lift failed: {e:?}"))?;

    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for op in lifted.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {}
        other => return Err(format!("unexpected first control flow: {other:?}")),
    }
    for op in lifted2.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted2.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {
            builder.set_terminator(Terminator::Return { values: vec![] });
        }
        other => return Err(format!("unexpected second control flow: {other:?}")),
    }
    let func = builder.finish();

    let mut lowerer = Aarch64X86_64Lowerer::new();
    let result = lowerer
        .lower_function(&func)
        .map_err(|e| format!("lower failed: {e:?}"))?;
    let code = lowerer
        .finalize()
        .map_err(|e| format!("finalize failed: {e:?}"))?;
    let mem = ExecMem::new(&code).map_err(|e| format!("exec memory failed: {e:?}"))?;

    let mut regs = arm_to_smir_regs(input);
    mem.run_aarch64(result.entry_offset, &mut regs);
    Ok(regs)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn run_smir_aarch64_x86_branch_pair(
    insn: u32,
    insn2: u32,
    input: &ArmState,
) -> Result<Aarch64GuestRegs, String> {
    let mut lifter = Aarch64Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::Aarch64);
    let lifted = lifter
        .lift_insn(0, &insn.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("first lift failed: {e:?}"))?;
    if !lifted.branch_targets.contains(&4) || !lifted.branch_targets.contains(&8) {
        return Err(format!(
            "unexpected branch targets: {:?}",
            lifted.branch_targets
        ));
    }

    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    let fallthrough_block = builder.create_block(4);
    let exit_block = builder.create_block(8);
    for op in lifted.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted.control_flow {
        ControlFlow::CondBranch {
            cond,
            target,
            fallthrough,
        } => {
            let cond_vreg = ctx.alloc_vreg();
            builder.push_op(
                0,
                rax::smir::ops::OpKind::TestCondition {
                    dst: cond_vreg,
                    cond,
                },
            );
            let to_block = |addr| match addr {
                4 => Ok(fallthrough_block),
                8 => Ok(exit_block),
                other => Err(format!("unexpected branch target {other:#x}")),
            };
            builder.set_terminator(Terminator::CondBranch {
                cond: cond_vreg,
                true_target: to_block(target)?,
                false_target: to_block(fallthrough)?,
            });
        }
        ControlFlow::CondBranchReg {
            cond,
            taken,
            not_taken,
        } => {
            let to_block = |addr| match addr {
                4 => Ok(fallthrough_block),
                8 => Ok(exit_block),
                other => Err(format!("unexpected branch target {other:#x}")),
            };
            builder.set_terminator(Terminator::CondBranch {
                cond,
                true_target: to_block(taken)?,
                false_target: to_block(not_taken)?,
            });
        }
        other => return Err(format!("unexpected first control flow: {other:?}")),
    }

    builder.switch_to_block(fallthrough_block);
    let lifted2 = lifter
        .lift_insn(4, &insn2.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("second lift failed: {e:?}"))?;
    for op in lifted2.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted2.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {
            builder.set_terminator(Terminator::Return { values: vec![] });
        }
        other => return Err(format!("unexpected second control flow: {other:?}")),
    }

    builder.switch_to_block(exit_block);
    builder.set_terminator(Terminator::Return { values: vec![] });
    let func = builder.finish();

    let mut lowerer = Aarch64X86_64Lowerer::new();
    let result = lowerer
        .lower_function(&func)
        .map_err(|e| format!("lower failed: {e:?}"))?;
    let code = lowerer
        .finalize()
        .map_err(|e| format!("finalize failed: {e:?}"))?;
    let mem = ExecMem::new(&code).map_err(|e| format!("exec memory failed: {e:?}"))?;

    let mut regs = arm_to_smir_regs(input);
    mem.run_aarch64(result.entry_offset, &mut regs);
    Ok(regs)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn run_smir_aarch64_x86_control_one(
    insn: u32,
    input: &ArmState,
) -> Result<Aarch64GuestRegs, String> {
    let mut lifter = Aarch64Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::Aarch64);
    let lifted = lifter
        .lift_insn(0, &insn.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("lift failed: {e:?}"))?;

    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for op in lifted.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted.control_flow {
        ControlFlow::Call { target } => {
            let continuation = builder.create_block(4);
            builder.set_terminator(Terminator::Call {
                target,
                args: vec![],
                continuation,
            });
            builder.switch_to_block(continuation);
            builder.set_terminator(Terminator::Return { values: vec![] });
        }
        ControlFlow::IndirectBranch { target } => {
            builder.set_terminator(Terminator::IndirectBranch {
                target,
                possible_targets: vec![],
            });
        }
        other => return Err(format!("unexpected control flow: {other:?}")),
    }
    let func = builder.finish();

    let mut lowerer = Aarch64X86_64Lowerer::new();
    let result = lowerer
        .lower_function(&func)
        .map_err(|e| format!("lower failed: {e:?}"))?;
    let code = lowerer
        .finalize()
        .map_err(|e| format!("finalize failed: {e:?}"))?;
    let mem = ExecMem::new(&code).map_err(|e| format!("exec memory failed: {e:?}"))?;

    let mut regs = arm_to_smir_regs(input);
    mem.run_aarch64(result.entry_offset, &mut regs);
    Ok(regs)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn arm_x(n: u8) -> VReg {
    VReg::Arch(ArchReg::Arm(ArmReg::X(n)))
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn lower_aarch64_native_function(func: &SmirFunction) -> Result<[u32; 3], String> {
    let mut lowerer = Aarch64Lowerer::new();
    lowerer
        .lower_function(func)
        .map_err(|e| format!("lower failed: {e:?}"))?;
    let code = lowerer
        .finalize()
        .map_err(|e| format!("finalize failed: {e:?}"))?;
    if code.len() % 4 != 0 {
        return Err(format!(
            "unexpected native AArch64 code size {}",
            code.len()
        ));
    }
    let words = code.len() / 4;
    if words > 3 {
        if words != 4 {
            return Err(format!(
                "unexpected native AArch64 code size {}",
                code.len()
            ));
        }
        let ret = u32::from_le_bytes([code[12], code[13], code[14], code[15]]);
        if ret != 0xd65f_03c0 {
            return Err(format!(
                "unexpected fourth native AArch64 instruction {ret:#010x}"
            ));
        }
    }

    let mut insns = [NOP; 3];
    for (idx, chunk) in code.chunks_exact(4).take(3).enumerate() {
        insns[idx] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    }
    Ok(insns)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn lower_aarch64_native_ops(ops: Vec<OpKind>) -> Result<[u32; 3], String> {
    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for (idx, kind) in ops.into_iter().enumerate() {
        builder.push_op((idx as u64) * 4, kind);
    }
    builder.set_terminator(Terminator::Return { values: vec![] });
    let func = builder.finish();

    lower_aarch64_native_function(&func)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn lower_aarch64_native_ops_same_pc(ops: Vec<OpKind>) -> Result<[u32; 3], String> {
    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for kind in ops {
        builder.push_op(0, kind);
    }
    builder.set_terminator(Terminator::Return { values: vec![] });
    let func = builder.finish();

    lower_aarch64_native_function(&func)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn lower_aarch64_native_insn(insn: u32) -> Result<[u32; 3], String> {
    let mut lifter = Aarch64Lifter::new();
    let mut ctx = LiftContext::new(SourceArch::Aarch64);
    let lifted = lifter
        .lift_insn(0, &insn.to_le_bytes(), &mut ctx)
        .map_err(|e| format!("lift failed: {e:?}"))?;

    let mut builder = FunctionBuilder::new(FunctionId(0), 0);
    for op in lifted.ops {
        builder.push_op(op.guest_pc, op.kind);
    }
    match lifted.control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => {
            builder.set_terminator(Terminator::Return { values: vec![] });
        }
        other => return Err(format!("unexpected control flow: {other:?}")),
    }
    let func = builder.finish();

    lower_aarch64_native_function(&func)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_mov_wide(sf: u32, opc: u32, hw: u32, imm16: u32) -> u32 {
    enc_mov_wide_regs(sf, opc, hw, imm16, RD)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_mov_wide_regs(sf: u32, opc: u32, hw: u32, imm16: u32, rd: u32) -> u32 {
    (sf << 31) | (opc << 29) | (0b100101 << 23) | (hw << 21) | (imm16 << 5) | rd
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_b_cond(cond: u32, imm19: i32) -> u32 {
    0x5400_0000 | (((imm19 as u32) & 0x7ffff) << 5) | (cond & 0xf)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_cbz(rt: u32, imm19: i32) -> u32 {
    0xb400_0000 | (((imm19 as u32) & 0x7ffff) << 5) | (rt & 0x1f)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_cbnz(rt: u32, imm19: i32) -> u32 {
    0xb500_0000 | (((imm19 as u32) & 0x7ffff) << 5) | (rt & 0x1f)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_not_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x5000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Not {
        dst: arm_x(0),
        src: VReg::Imm(0x1234),
        width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| panic!("not_w_imm_as_movn_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "not_w_imm_as_movn_preserves_flags".into(),
        [enc_mov_wide(0, 0b00, 0, 0x1234), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0x9000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Not {
        dst: arm_x(0),
        src: VReg::Imm(0x1234),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| panic!("not_x_imm_as_movn_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "not_x_imm_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0x1234), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_sar_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_fff0;
    st.pstate = 0x9000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sar {
        dst: arm_x(0),
        src: VReg::Imm(-16),
        amount: SrcOperand::Imm(4),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sar_w_imm_src_negative_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sar_w_imm_src_negative_as_movn_preserves_flags".into(),
        [enc_bitfield(0, 0b00, 4, 31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_ffff_fff0;
    st.pstate = 0x5000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sar {
        dst: arm_x(0),
        src: VReg::Imm(-16),
        amount: SrcOperand::Imm(4),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sar_x_imm_src_negative_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sar_x_imm_src_negative_as_movn_preserves_flags".into(),
        [enc_bitfield(1, 0b00, 4, 63), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_ror_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Ror {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        amount: SrcOperand::Imm(4),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("ror_w_imm_src_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "ror_w_imm_src_all_ones_as_movn_preserves_flags".into(),
        [enc_extract(0, RN, RN, 4), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Ror {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        amount: SrcOperand::Imm(4),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("ror_x_imm_src_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "ror_x_imm_src_all_ones_as_movn_preserves_flags".into(),
        [enc_extract(1, RN, RN, 4), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_rol_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Rol {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        amount: SrcOperand::Imm(4),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("rol_w_imm_src_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rol_w_imm_src_all_ones_as_movn_preserves_flags".into(),
        [enc_extract(0, RN, RN, 28), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Rol {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        amount: SrcOperand::Imm(4),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("rol_x_imm_src_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rol_x_imm_src_all_ones_as_movn_preserves_flags".into(),
        [enc_extract(1, RN, RN, 60), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_shl_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Shl {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        amount: SrcOperand::Imm(4),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("shl_w_imm_src_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "shl_w_imm_src_all_ones_as_movn_preserves_flags".into(),
        [enc_bitfield(0, 0b10, 28, 27), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Shl {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        amount: SrcOperand::Imm(4),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("shl_x_imm_src_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "shl_x_imm_src_all_ones_as_movn_preserves_flags".into(),
        [enc_bitfield(1, 0b10, 60, 59), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_shr_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xd000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Shr {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        amount: SrcOperand::Imm(0),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("shr_w_imm_src_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "shr_w_imm_src_all_ones_as_movn_preserves_flags".into(),
        [enc_bitfield(0, 0b10, 0, 31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Shr {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        amount: SrcOperand::Imm(0),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("shr_x_imm_src_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "shr_x_imm_src_all_ones_as_movn_preserves_flags".into(),
        [enc_bitfield(1, 0b10, 0, 63), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_rbit_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Rbit {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!("rbit_w_imm_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rbit_w_imm_all_ones_as_movn_preserves_flags".into(),
        [enc_dp1(0, 0b000000), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Rbit {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("rbit_x_imm_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rbit_x_imm_all_ones_as_movn_preserves_flags".into(),
        [enc_dp1(1, 0b000000), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bswap_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bswap {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!("bswap_w_imm_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "bswap_w_imm_all_ones_as_movn_preserves_flags".into(),
        [enc_dp1(0, 0b000010), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bswap {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("bswap_x_imm_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "bswap_x_imm_all_ones_as_movn_preserves_flags".into(),
        [enc_dp1(1, 0b000011), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_extract_masked_shift_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1122_3344_5566_7788;
    st.x[2] = 0x99aa_bbcc_ddee_ff00;
    st.pstate = 0x6000_0000;
    let lo = VReg::virt(0);
    let hi = VReg::virt(1);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Shr {
            dst: lo,
            src: arm_x(2),
            amount: SrcOperand::Imm64(77),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Shl {
            dst: hi,
            src: arm_x(1),
            amount: SrcOperand::Imm(115),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: arm_x(0),
            src1: lo,
            src2: SrcOperand::Reg(hi),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("extr_x_lifted_masked_shift_counts_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "extr_x_lifted_masked_shift_counts_preserves_flags".into(),
        [enc_extract(1, RN, RM, 13), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bitfield_insert_zero_masked_shift_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_de77;
    st.pstate = 0x6000_0000;
    let extracted = VReg::virt(0);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Bfx {
            dst: extracted,
            src: arm_x(1),
            lsb: 0,
            width_bits: 8,
            sign_extend: false,
            op_width: OpWidth::W64,
        },
        OpKind::Shl {
            dst: arm_x(0),
            src: extracted,
            amount: SrcOperand::Imm64(68),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("ubfiz_x_lifted_masked_shift_count_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "ubfiz_x_lifted_masked_shift_count_preserves_flags".into(),
        [enc_bitfield(1, 0b10, 60, 7), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_rev16_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x7000_0000;
    let lo = VReg::virt(0);
    let hi = VReg::virt(1);
    let lo_shifted = VReg::virt(2);
    let hi_shifted = VReg::virt(3);
    let src = VReg::Imm(-1);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::And {
            dst: lo,
            src1: src,
            src2: SrcOperand::Imm64(0x00ff_00ff),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: hi,
            src1: src,
            src2: SrcOperand::Imm64(0xff00_ff00),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Shl {
            dst: lo_shifted,
            src: lo,
            amount: SrcOperand::Imm(8),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Shr {
            dst: hi_shifted,
            src: hi,
            amount: SrcOperand::Imm(8),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: arm_x(0),
            src1: lo_shifted,
            src2: SrcOperand::Reg(hi_shifted),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("rev16_w_lifted_imm_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rev16_w_lifted_imm_all_ones_as_movn_preserves_flags".into(),
        [enc_dp1(0, 0b000001), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x2000_0000;
    let lo = VReg::virt(0);
    let hi = VReg::virt(1);
    let lo_shifted = VReg::virt(2);
    let hi_shifted = VReg::virt(3);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::And {
            dst: lo,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_00ff_00ff),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: hi,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_ff00_ff00),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Shl {
            dst: lo_shifted,
            src: lo,
            amount: SrcOperand::Imm(72),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Shr {
            dst: hi_shifted,
            src: hi,
            amount: SrcOperand::Imm64(72),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: arm_x(0),
            src1: lo_shifted,
            src2: SrcOperand::Reg(hi_shifted),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("rev16_w_lifted_masked_immediates_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rev16_w_lifted_masked_immediates_preserves_flags".into(),
        [enc_dp1(0, 0b000001), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xb000_0000;
    let lo = VReg::virt(0);
    let hi = VReg::virt(1);
    let lo_shifted = VReg::virt(2);
    let hi_shifted = VReg::virt(3);
    let src = VReg::Imm(-1);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::And {
            dst: lo,
            src1: src,
            src2: SrcOperand::Imm64(0x00ff_00ff_00ff_00ff),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: hi,
            src1: src,
            src2: SrcOperand::Imm64(0xff00_ff00_ff00_ff00_u64 as i64),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Shl {
            dst: lo_shifted,
            src: lo,
            amount: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Shr {
            dst: hi_shifted,
            src: hi,
            amount: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: arm_x(0),
            src1: lo_shifted,
            src2: SrcOperand::Reg(hi_shifted),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("rev16_x_lifted_imm_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rev16_x_lifted_imm_all_ones_as_movn_preserves_flags".into(),
        [enc_dp1(1, 0b000001), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_rev32_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xb000_0000;
    let lo_rev = VReg::virt(0);
    let hi = VReg::virt(1);
    let hi_rev = VReg::virt(2);
    let hi_shifted = VReg::virt(3);
    let src = VReg::Imm(-1);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Bswap {
            dst: lo_rev,
            src,
            width: OpWidth::W32,
        },
        OpKind::Shr {
            dst: hi,
            src,
            amount: SrcOperand::Imm(32),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Bswap {
            dst: hi_rev,
            src: hi,
            width: OpWidth::W32,
        },
        OpKind::Shl {
            dst: hi_shifted,
            src: hi_rev,
            amount: SrcOperand::Imm(32),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: arm_x(0),
            src1: hi_shifted,
            src2: SrcOperand::Reg(lo_rev),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("rev32_x_lifted_imm_all_ones_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rev32_x_lifted_imm_all_ones_as_movn_preserves_flags".into(),
        [enc_dp1(1, 0b000010), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x4000_0000;
    let lo_rev = VReg::virt(0);
    let hi = VReg::virt(1);
    let hi_rev = VReg::virt(2);
    let hi_shifted = VReg::virt(3);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Bswap {
            dst: lo_rev,
            src: arm_x(1),
            width: OpWidth::W32,
        },
        OpKind::Shr {
            dst: hi,
            src: arm_x(1),
            amount: SrcOperand::Imm64(96),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Bswap {
            dst: hi_rev,
            src: hi,
            width: OpWidth::W32,
        },
        OpKind::Shl {
            dst: hi_shifted,
            src: hi_rev,
            amount: SrcOperand::Imm(96),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: arm_x(0),
            src1: hi_shifted,
            src2: SrcOperand::Reg(lo_rev),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("rev32_x_lifted_masked_shift_counts_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "rev32_x_lifted_masked_shift_counts_preserves_flags".into(),
        [enc_dp1(1, 0b000010), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_truncate_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_0000_ffff_0000;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Truncate {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        from_width: OpWidth::W64,
        to_width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("truncate_imm_to_x_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "truncate_imm_to_x_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_divu_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0xc000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::DivU {
        quot: arm_x(0),
        rem: None,
        src1: VReg::Imm(-1),
        src2: SrcOperand::Imm(1),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("divu_x_two_imms_all_ones_quot_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "divu_x_two_imms_all_ones_quot_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_divs_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0xd000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::DivS {
        quot: arm_x(0),
        rem: None,
        src1: VReg::Imm(-15),
        src2: SrcOperand::Imm(1),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("divs_x_two_imms_negative_quot_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "divs_x_two_imms_negative_quot_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0xe), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_div_one_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let div_cases = [
        (
            "divu_x_imm_one_same_dst_as_noop_preserves_flags",
            OpKind::DivU {
                quot: arm_x(0),
                rem: None,
                src1: arm_x(0),
                src2: SrcOperand::Imm(1),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [0xd65f_03c0u32, NOP, NOP],
            0x1111_2222_3333_4444,
            0x3000_0000,
        ),
        (
            "divs_w_imm_one_same_dst_as_self_mov_zero_ext_preserves_flags",
            OpKind::DivS {
                quot: arm_x(0),
                rem: None,
                src1: arm_x(0),
                src2: SrcOperand::Imm64(1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_mov_reg(0, RD, RD), NOP, NOP],
            0xffff_ffff_8765_4321,
            0x6000_0000,
        ),
        (
            "divu_w8_imm_one_same_dst_as_uxtb_preserves_flags",
            OpKind::DivU {
                quot: arm_x(0),
                rem: None,
                src1: arm_x(0),
                src2: SrcOperand::Imm(1),
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [enc_bitfield_regs(0, 0b10, 0, 7, RD, RD), NOP, NOP],
            0xaaaa_bbbb_cccc_dd81,
            0x9000_0000,
        ),
    ];

    for (name, op, source, x0, pstate) in div_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_divs_neg_one_subword_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_8001;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::DivS {
        quot: arm_x(0),
        rem: None,
        src1: arm_x(1),
        src2: SrcOperand::Imm64(0x1_ffff),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "divs_w16_imm_masked_neg_one_as_neg_uxth_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "divs_w16_imm_masked_neg_one_as_neg_uxth_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_divs_neg_one_remainder_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0xffff_ffff_ffff_ff85;
    st.x[3] = 0x5555_6666_7777_8888;
    st.pstate = 0x3000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::DivS {
        quot: arm_x(0),
        rem: Some(arm_x(3)),
        src1: arm_x(1),
        src2: SrcOperand::Imm(-1),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("divs_x_imm_neg_one_with_remainder_as_neg_zero_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "divs_x_imm_neg_one_with_remainder_as_neg_zero_preserves_flags".into(),
        [
            enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RN),
            enc_mov_wide_regs(1, 0b10, 0, 0, 3),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_8001;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::DivS {
        quot: arm_x(0),
        rem: Some(arm_x(1)),
        src1: arm_x(1),
        src2: SrcOperand::Imm64(0x1_ffff),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("divs_w16_imm_neg_one_remainder_aliases_source_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "divs_w16_imm_neg_one_remainder_aliases_source_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            enc_mov_wide_regs(0, 0b10, 0, 0, RN),
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_divs_power_of_two_alias_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_ffff_ff81;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::DivS {
        quot: arm_x(0),
        rem: None,
        src1: arm_x(0),
        src2: SrcOperand::Imm(8),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "divs_x_imm_power_of_two_in_place_as_bias_asr_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "divs_x_imm_power_of_two_in_place_as_bias_asr_preserves_flags".into(),
        [
            enc_addsub_shift_regs(1, 0, 0, 1, 61, RD, RD, RD),
            enc_bitfield_regs(1, 0b00, 3, 63, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_8765_4321;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::DivS {
        quot: arm_x(0),
        rem: None,
        src1: arm_x(0),
        src2: SrcOperand::Imm64(0x1_0000_0010),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "divs_w_imm_power_of_two_in_place_as_bias_asr_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "divs_w_imm_power_of_two_in_place_as_bias_asr_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 0, 0, 1, 28, RD, RD, RD),
            enc_bitfield_regs(0, 0b00, 4, 31, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_mov_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x3333_4444_5555_6666;
    st.pstate = 0x9000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Mov {
        dst: arm_x(0),
        src: SrcOperand::Imm(-15),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("mov_x_negative_imm_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mov_x_negative_imm_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0xe), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_mov_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Mov {
        dst: arm_x(0),
        src: SrcOperand::Reg(arm_x(0)),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("mov_x_same_reg_as_noop_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mov_x_same_reg_as_noop_preserves_flags".into(),
        [0xd65f_03c0, NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_9999_aaaa;
    st.pstate = 0x3000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Mov {
        dst: arm_x(0),
        src: SrcOperand::Reg(arm_x(0)),
        width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!("mov_w_same_reg_as_self_mov_zero_ext_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mov_w_same_reg_as_self_mov_zero_ext_preserves_flags".into(),
        [enc_mov_reg(0, RD, RD), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_extend_same_width_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let extend_cases = [
        (
            "zero_extend_x_same_width_same_reg_as_noop_preserves_flags",
            OpKind::ZeroExtend {
                dst: arm_x(0),
                src: arm_x(0),
                from_width: OpWidth::W64,
                to_width: OpWidth::W64,
            },
            [0xd65f_03c0u32, NOP, NOP],
            0x1111_2222_3333_4444,
            0,
            0xb000_0000,
        ),
        (
            "sign_extend_x_same_width_same_reg_as_noop_preserves_flags",
            OpKind::SignExtend {
                dst: arm_x(0),
                src: arm_x(0),
                from_width: OpWidth::W64,
                to_width: OpWidth::W64,
            },
            [0xd65f_03c0u32, NOP, NOP],
            0x8000_0000_0000_0001,
            0,
            0x5000_0000,
        ),
        (
            "zero_extend_w_same_width_same_reg_as_self_mov_zero_ext_preserves_flags",
            OpKind::ZeroExtend {
                dst: arm_x(0),
                src: arm_x(0),
                from_width: OpWidth::W32,
                to_width: OpWidth::W32,
            },
            [enc_mov_reg(0, RD, RD), NOP, NOP],
            0xffff_ffff_8765_4321,
            0,
            0x6000_0000,
        ),
        (
            "sign_extend_x_same_width_as_mov_preserves_flags",
            OpKind::SignExtend {
                dst: arm_x(0),
                src: arm_x(1),
                from_width: OpWidth::W64,
                to_width: OpWidth::W64,
            },
            [enc_mov_reg(1, RD, RN), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x7777_8888_9999_aaaa,
            0xc000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in extend_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_truncate_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let truncate_cases = [
        (
            "truncate_x_same_reg_as_noop_preserves_flags",
            OpKind::Truncate {
                dst: arm_x(0),
                src: arm_x(0),
                from_width: OpWidth::W64,
                to_width: OpWidth::W64,
            },
            [0xd65f_03c0u32, NOP, NOP],
            0x1111_2222_3333_4444,
            0,
            0x9000_0000,
        ),
        (
            "truncate_x_to_w_same_reg_as_self_mov_zero_ext_preserves_flags",
            OpKind::Truncate {
                dst: arm_x(0),
                src: arm_x(0),
                from_width: OpWidth::W64,
                to_width: OpWidth::W32,
            },
            [enc_mov_reg(0, RD, RD), NOP, NOP],
            0xffff_ffff_8765_4321,
            0,
            0x3000_0000,
        ),
        (
            "truncate_x_as_mov_preserves_flags",
            OpKind::Truncate {
                dst: arm_x(0),
                src: arm_x(1),
                from_width: OpWidth::W64,
                to_width: OpWidth::W64,
            },
            [enc_mov_reg(1, RD, RN), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x8000_0000_0000_0001,
            0x6000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in truncate_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_xchg_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Xchg {
        reg1: arm_x(0),
        reg2: arm_x(0),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| panic!("xchg_same_x_as_noop_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "xchg_same_x_as_noop_preserves_flags".into(),
        [0xd65f_03c0, NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_logical_identity_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let logical_cases = [
        (
            "and_x_all_ones_same_reg_as_noop_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Imm64(-1),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x1111_2222_3333_4444,
            0x3000_0000,
        ),
        (
            "orr_x_zero_same_reg_as_noop_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Imm(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x5555_6666_7777_8888,
            0x9000_0000,
        ),
        (
            "eor_x_zero_same_reg_as_noop_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Imm(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x9999_aaaa_bbbb_cccc,
            0xc000_0000,
        ),
        (
            "and_x_same_reg_as_noop_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x1357_9bdf_2468_ace0,
            0x1000_0000,
        ),
        (
            "orr_x_same_reg_as_noop_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0xfedc_ba98_7654_3210,
            0x6000_0000,
        ),
    ];

    for (name, op, x0, pstate) in logical_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), [0xd65f_03c0, NOP, NOP], lowered, st));
    }

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::And {
        dst: arm_x(0),
        src1: arm_x(0),
        src2: SrcOperand::Reg(arm_x(0)),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("and_w_same_reg_as_self_mov_zero_ext_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "and_w_same_reg_as_self_mov_zero_ext_preserves_flags".into(),
        [enc_mov_reg(0, 0, 0), NOP, NOP],
        lowered,
        st,
    ));

    let logical_move_cases = [
        (
            "and_x_same_sources_as_mov_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Reg(arm_x(1)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b00, 0, 0, 0, 0, 1, 1), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x0123_4567_89ab_cdef,
            0x5000_0000,
        ),
        (
            "orr_x_same_sources_as_mov_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Reg(arm_x(1)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b01, 0, 0, 0, 0, 1, 1), NOP, NOP],
            0xbbbb_cccc_dddd_eeee,
            0xfedc_ba98_7654_3210,
            0x8000_0000,
        ),
        (
            "orr_w_same_sources_as_mov_zero_ext_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Reg(arm_x(1)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(0, 0b01, 0, 0, 0, 0, 1, 1), NOP, NOP],
            0xcccc_dddd_eeee_ffff,
            0xffff_ffff_89ab_cdef,
            0x2000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in logical_move_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_logical_zero_base_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let logical_cases = [
        (
            "and_x_zero_base_as_movz_zero_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b00, 0, 0, 0, 0, 31, 0), NOP, NOP],
            0x1234_5678_9abc_def0,
            0xe000_0000,
        ),
        (
            "and_w_zero_source_as_movz_zero_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(0, 0b00, 0, 0, 0, 0, 0, 31), NOP, NOP],
            0xffff_ffff_89ab_cdef,
            0x5000_0000,
        ),
        (
            "bic_x_zero_base_as_movz_zero_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b00, 0, 1, 0, 0, 31, 0), NOP, NOP],
            0x0fed_cba9_8765_4321,
            0xa000_0000,
        ),
        (
            "bic_w_zero_base_as_movz_zero_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(0, 0b00, 0, 1, 0, 0, 31, 0), NOP, NOP],
            0xffff_ffff_7654_3210,
            0x3000_0000,
        ),
        (
            "bic_x_zero_source_same_dst_as_noop_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b00, 0, 1, 0, 0, 0, 31), NOP, NOP],
            0x1357_9bdf_2468_ace0,
            0xc000_0000,
        ),
        (
            "bic_w_zero_source_same_dst_as_self_mov_zero_ext_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(0, 0b00, 0, 1, 0, 0, 0, 31), NOP, NOP],
            0xffff_ffff_89ab_cdef,
            0x8000_0000,
        ),
        (
            "orr_x_zero_base_same_dst_as_noop_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_mov_reg(1, 0, 0), NOP, NOP],
            0x1357_2468_9abc_def0,
            0x1000_0000,
        ),
        (
            "eor_x_zero_base_same_dst_as_noop_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b10, 0, 0, 0, 0, 31, 0), NOP, NOP],
            0xfedc_ba98_7654_3210,
            0x6000_0000,
        ),
        (
            "orr_w_zero_base_same_dst_as_self_mov_zero_ext_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_mov_reg(0, 0, 0), NOP, NOP],
            0xffff_ffff_89ab_cdef,
            0x2000_0000,
        ),
        (
            "eor_w_zero_base_same_dst_as_self_mov_zero_ext_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(0, 0b10, 0, 0, 0, 0, 31, 0), NOP, NOP],
            0xffff_ffff_0123_4567,
            0x9000_0000,
        ),
        (
            "orr_x_zero_source_same_dst_as_noop_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b01, 0, 0, 0, 0, 0, 31), NOP, NOP],
            0x2468_ace0_1357_9bdf,
            0x7000_0000,
        ),
        (
            "eor_x_zero_source_same_dst_as_noop_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b10, 0, 0, 0, 0, 0, 31), NOP, NOP],
            0xfedc_0123_7654_89ab,
            0x4000_0000,
        ),
        (
            "orr_w_zero_source_same_dst_as_self_mov_zero_ext_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(0, 0b01, 0, 0, 0, 0, 0, 31), NOP, NOP],
            0xffff_ffff_7654_3210,
            0xd000_0000,
        ),
    ];

    for (name, op, source, x0, pstate) in logical_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_logical_zero_base_extended_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let logical_cases = [
        (
            "and_x_zero_base_uxtw_extended_as_movz_zero_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtw,
                    shift: 0,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(1, 0b10, 0, 0), NOP, NOP],
            [enc_mov_wide(1, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0x5151_6161_7171_8181,
            0xffff_ffff_0000_0100,
            0xe000_0000,
        ),
        (
            "bic_w_zero_base_sxtb_lsl1_extended_as_movz_zero_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(0, 0b10, 0, 0), NOP, NOP],
            [enc_mov_wide(0, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0xffff_ffff_5252_6262,
            0xffff_ffff_ffff_ff80,
            0x3000_0000,
        ),
        (
            "ands_x_zero_base_sxtw_lsl2_extended_sets_zero_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, RD, 31, 31),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, RD, 31, 31),
                0xd65f_03c0,
                NOP,
            ],
            0x5353_6363_7373_8383,
            0x0000_0000_ffff_ffff,
            0xf000_0000,
        ),
        (
            "orr_x_zero_base_uxtw_extended_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtw,
                    shift: 0,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_addsub_ext_regs(1, 0, 0, 0b010, 0, RD, 31, RM), NOP, NOP],
            [
                enc_addsub_ext_regs(1, 0, 0, 0b010, 0, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0x5454_6464_7474_8484,
            0xffff_ffff_0000_0100,
            0x5000_0000,
        ),
        (
            "eor_w_zero_base_uxtb_lsl1_extended_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_addsub_ext_regs(0, 0, 0, 0b000, 1, RD, 31, RM), NOP, NOP],
            [
                enc_addsub_ext_regs(0, 0, 0, 0b000, 1, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0xffff_ffff_5555_6565,
            0xffff_ffff_ffff_ff80,
            0x9000_0000,
        ),
        (
            "orrs_x_zero_base_sxtw_lsl2_extended_sets_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [enc_addsub_ext_regs(1, 0, 1, 0b110, 2, RD, 31, RM), NOP, NOP],
            [
                enc_addsub_ext_regs(1, 0, 0, 0b110, 2, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0x5656_6666_7676_8686,
            0x0000_0000_ffff_ffff,
            0x1000_0000,
        ),
        (
            "eors_w_zero_base_uxtb_lsl1_extended_sets_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [enc_addsub_ext_regs(0, 0, 1, 0b000, 1, RD, 31, RM), NOP, NOP],
            [
                enc_addsub_ext_regs(0, 0, 0, 0b000, 1, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_5757_6767,
            0,
            0xa000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in logical_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_test_zero_imm_operand_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "test_x_zero_left_imm_reg_opkind_sets_z",
            OpKind::Test {
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
            },
            [
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, 31, 31),
                NOP,
                NOP,
            ],
            0xffff_ffff_ffff_ffff,
            0x8000_0000_0000_0000,
            0xf000_0000,
        ),
        (
            "test_w_masked_zero_left_imm_reg_opkind_sets_z",
            OpKind::Test {
                src1: VReg::Imm(0x1_0000_0000),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
            },
            [
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, 31, 31),
                NOP,
                NOP,
            ],
            0xffff_ffff_ffff_ffff,
            0xffff_ffff,
            0x8000_0000,
        ),
        (
            "test_w8_masked_zero_imm_opkind_sets_z",
            OpKind::Test {
                src1: arm_x(1),
                src2: SrcOperand::Imm(0x100),
                width: OpWidth::W8,
            },
            [
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, 31, 31),
                NOP,
                NOP,
            ],
            0xff,
            0,
            0x1000_0000,
        ),
        (
            "test_w8_two_imms_zero_result_opkind_sets_z",
            OpKind::Test {
                src1: VReg::Imm(0x10),
                src2: SrcOperand::Imm(0x20),
                width: OpWidth::W8,
            },
            [
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, 31, 31),
                NOP,
                NOP,
            ],
            0x1111_2222_3333_4444,
            0x5555_6666_7777_8888,
            0xa000_0000,
        ),
        (
            "test_x_two_imms_positive_result_opkind_clears_flags",
            OpKind::Test {
                src1: VReg::Imm(0x03),
                src2: SrcOperand::Imm(0x05),
                width: OpWidth::W64,
            },
            [enc_msr_sysreg(31, 3, 4, 2, 0), NOP, NOP],
            0x9999_aaaa_bbbb_cccc,
            0xdddd_eeee_ffff_0000,
            0xf000_0000,
        ),
        (
            "test_w16_two_imms_negative_result_opkind_sets_n",
            OpKind::Test {
                src1: VReg::Imm(0x8001),
                src2: SrcOperand::Imm(0x8000),
                width: OpWidth::W16,
            },
            [enc_addsub_imm_regs(0, 1, 1, 0, 1, 31, 31), NOP, NOP],
            0x1111_2222_3333_4444,
            0x5555_6666_7777_8888,
            0x7000_0000,
        ),
    ];

    for (name, op, source, x1, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[1] = x1;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(
            lowered,
            [source[0], 0xd65f_03c0, NOP],
            "{name}: unexpected lowering"
        );
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_materialized_logical_imm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "orr_x_non_contiguous_imm_destination_scratch_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Imm(0x55),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_mov_wide(1, 0b10, 0, 0x55),
                enc_logical_shift_regs(1, 0b01, 0, 0, 0, RD, RN, RD),
                NOP,
            ],
            [
                enc_mov_wide(1, 0b10, 0, 0x55),
                enc_logical_shift_regs(1, 0b01, 0, 0, 0, RD, RN, RD),
                0xd65f_03c0,
            ],
            0x9090_a0a0_b0b0_c0c0,
            0x2000_0000,
        ),
        (
            "bic_x_unencodable_inverse_imm_destination_scratch_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Imm64(!0x55_i64),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_mov_wide(1, 0b10, 0, 0x55),
                enc_logical_shift_regs(1, 0b00, 0, 0, 0, RD, RN, RD),
                NOP,
            ],
            [
                enc_mov_wide(1, 0b10, 0, 0x55),
                enc_logical_shift_regs(1, 0b00, 0, 0, 0, RD, RN, RD),
                0xd65f_03c0,
            ],
            0xa0a0_b0b0_c0c0_d0d0,
            0x4000_0000,
        ),
        (
            "ands_x_non_contiguous_imm_destination_scratch_sets_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Imm(0x55),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_mov_wide(1, 0b10, 0, 0x55),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, RD, RN, RD),
                NOP,
            ],
            [
                enc_mov_wide(1, 0b10, 0, 0x55),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, RD, RN, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_ffff_ff80,
            0x6000_0000,
        ),
        (
            "eor_w_non_contiguous_imm_destination_scratch_zero_ext_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Imm(0x55),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_mov_wide(0, 0b10, 0, 0x55),
                enc_logical_shift_regs(0, 0b10, 0, 0, 0, RD, RN, RD),
                NOP,
            ],
            [
                enc_mov_wide(0, 0b10, 0, 0x55),
                enc_logical_shift_regs(0, 0b10, 0, 0, 0, RD, RN, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_1234_5680,
            0x8000_0000,
        ),
        (
            "orr_w8_non_contiguous_imm_destination_scratch_zero_ext_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Imm(0x5a),
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [
                enc_mov_wide(0, 0b10, 0, 0x5a),
                enc_logical_shift_regs(0, 0b01, 0, 0, 0, RD, RN, RD),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            ],
            [
                enc_mov_wide(0, 0b10, 0, 0x5a),
                enc_logical_shift_regs(0, 0b01, 0, 0, 0, RD, RN, RD),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            ],
            0xffff_ffff_ffff_ff20,
            0x1000_0000,
        ),
        (
            "bic_w16_unencodable_inverse_imm_destination_scratch_zero_ext_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Imm64(!0x5a_i64),
                width: OpWidth::W16,
                flags: FlagUpdate::None,
            },
            [
                enc_mov_wide(0, 0b10, 0, 0x5a),
                enc_logical_shift_regs(0, 0b00, 0, 0, 0, RD, RN, RD),
                enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            ],
            [
                enc_mov_wide(0, 0b10, 0, 0x5a),
                enc_logical_shift_regs(0, 0b00, 0, 0, 0, RD, RN, RD),
                enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            ],
            0xffff_ffff_0000_f5ff,
            0xe000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x1, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
#[inline(never)]
fn push_sparse_logical_imm_in_place_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let name = "orr_x_sparse_imm_in_place_as_logical_imms_preserves_flags";
    let source = [
        enc_logical_imm_regs(1, 0b01, 1, 0, 0, RN, RN),
        enc_logical_imm_regs(1, 0b01, 1, 62, 0, RN, RN),
        NOP,
    ];
    let expected_lowered = [
        enc_logical_imm_regs(1, 0b01, 1, 0, 0, RN, RN),
        enc_logical_imm_regs(1, 0b01, 1, 62, 0, RN, RN),
        0xd65f_03c0,
    ];
    let lowered = lower_aarch64_native_ops(vec![OpKind::Or {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0x5),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
    assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0xf0f0_0000_0000_0000;
    st.pstate = 0x7000_0000;
    cases.push((name.into(), source, lowered, st));

    let name = "eors_w_sparse_imm_in_place_as_logical_imms_sets_flags";
    let source = [
        enc_logical_imm_regs(0, 0b10, 0, 0, 0, RN, RN),
        enc_logical_imm_regs(0, 0b10, 0, 27, 0, RN, RN),
        enc_logical_reg_n(0, 0b11, 0, 31, RN, RN),
    ];
    let expected_lowered = source;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Xor {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0x21),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
    assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0xffff_ffff_8000_0021;
    st.pstate = 0x3000_0000;
    cases.push((name.into(), source, lowered, st));

    let name = "and_x_sparse_clear_imm_in_place_as_logical_imms_preserves_flags";
    let source = [
        enc_logical_imm_regs(1, 0b00, 1, 63, 62, RN, RN),
        enc_logical_imm_regs(1, 0b00, 1, 61, 62, RN, RN),
        NOP,
    ];
    let expected_lowered = [
        enc_logical_imm_regs(1, 0b00, 1, 63, 62, RN, RN),
        enc_logical_imm_regs(1, 0b00, 1, 61, 62, RN, RN),
        0xd65f_03c0,
    ];
    let lowered = lower_aarch64_native_ops(vec![OpKind::And {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm64(!0x5),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
    assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xd000_0000;
    cases.push((name.into(), source, lowered, st));

    let name = "ands_w_sparse_clear_imm_in_place_as_logical_imms_sets_flags";
    let source = [
        enc_logical_imm_regs(0, 0b11, 0, 31, 30, RN, RN),
        enc_logical_imm_regs(0, 0b11, 0, 26, 30, RN, RN),
        NOP,
    ];
    let expected_lowered = [
        enc_logical_imm_regs(0, 0b11, 0, 31, 30, RN, RN),
        enc_logical_imm_regs(0, 0b11, 0, 26, 30, RN, RN),
        0xd65f_03c0,
    ];
    let lowered = lower_aarch64_native_ops(vec![OpKind::And {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm(!0x21),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
    assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0xffff_ffff_8000_0021;
    st.pstate = 0x7000_0000;
    cases.push((name.into(), source, lowered, st));

    let name = "orr_w8_sparse_imm_in_place_as_logical_imms_zero_ext_preserves_flags";
    let source = [
        enc_logical_imm_regs(0, 0b01, 0, 0, 0, RN, RN),
        enc_logical_imm_regs(0, 0b01, 0, 30, 0, RN, RN),
        enc_bitfield_regs(0, 0b10, 0, 7, RN, RN),
    ];
    let expected_lowered = source;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Or {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0x5),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
    assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0xffff_ffff_ffff_ff00;
    st.pstate = 0x6000_0000;
    cases.push((name.into(), source, lowered, st));

    let name = "and_w16_sparse_clear_imm_in_place_as_logical_imms_zero_ext_preserves_flags";
    let source = [
        enc_logical_imm_regs(0, 0b00, 0, 31, 30, RN, RN),
        enc_logical_imm_regs(0, 0b00, 0, 29, 30, RN, RN),
        enc_bitfield_regs(0, 0b10, 0, 15, RN, RN),
    ];
    let expected_lowered = source;
    let lowered = lower_aarch64_native_ops(vec![OpKind::And {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm(!0x5),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
    assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x9000_0000;
    cases.push((name.into(), source, lowered, st));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_test_all_ones_left_imm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "test_x_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::Test {
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
            },
            [
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RM, RM),
                NOP,
                NOP,
            ],
            0x8000_0000_0000_1234,
            0x4000_0000,
        ),
        (
            "test_w_masked_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::Test {
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
            },
            [
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RM, RM),
                NOP,
                NOP,
            ],
            0xffff_ffff_8000_1234,
            0,
        ),
    ];

    for (name, op, source, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(
            lowered,
            [source[0], 0xd65f_03c0, NOP],
            "{name}: unexpected lowering"
        );
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_and_all_ones_left_imm_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "and_x_all_ones_left_imm_reg_opkind_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(1, 0b01, 0, 0, 0, RD, 31, RM),
                NOP,
                NOP,
            ],
            0x2121_3131_4141_5151,
            0xfedc_ba98_7654_3210,
            0x7000_0000,
        ),
        (
            "and_w_masked_all_ones_left_imm_reg_opkind_zero_ext_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(0, 0b01, 0, 0, 0, RD, 31, RM),
                NOP,
                NOP,
            ],
            0x2222_3232_4242_5252,
            0xffff_ffff_8000_1234,
            0x3000_0000,
        ),
        (
            "ands_x_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, RD, RM, RM),
                NOP,
                NOP,
            ],
            0x4141_5151_6161_7171,
            0x8000_0000_0000_5678,
            0xf000_0000,
        ),
        (
            "ands_w_masked_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, RD, RM, RM),
                NOP,
                NOP,
            ],
            0x4242_5252_6262_7272,
            0x0000_0000_8000_1234,
            0x7000_0000,
        ),
    ];

    for (name, op, source, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(
            lowered,
            [source[0], 0xd65f_03c0, NOP],
            "{name}: unexpected lowering"
        );
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_andnot_all_ones_left_imm_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "bic_x_all_ones_left_imm_reg_opkind_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RM),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0x2727_3737_4747_5757,
            0xffff_ffff_0000_0100,
            0xb000_0000,
        ),
        (
            "bic_w_masked_all_ones_left_imm_reg_opkind_zero_ext_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(0, 0b01, 0, 1, 0, RD, 31, RM),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(0, 0b01, 0, 1, 0, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0xffff_ffff_2828_3838,
            0xffff_ffff_8000_1234,
            0x5000_0000,
        ),
        (
            "bics_x_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0x4747_5757_6767_7777,
            0,
            0x9000_0000,
        ),
        (
            "bics_w_masked_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(0, 0b01, 0, 1, 0, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_logical_shift_regs(0, 0b01, 0, 1, 0, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_4848_5858,
            0x0000_0000_ffff_ffff,
            0x7000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_andnot_all_ones_left_shifted_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "bic_x_all_ones_left_imm_lsl_opkind_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsl,
                    amount: 4,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(1, 0b01, 0, 1, 4, RD, 31, RM),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b01, 0, 1, 4, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0x2929_3939_4949_5959,
            0x0000_0000_0000_1001,
            0xd000_0000,
        ),
        (
            "bic_w_masked_all_ones_left_imm_ror_opkind_zero_ext_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Ror,
                    amount: 13,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(0, 0b01, 3, 1, 13, RD, 31, RM),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(0, 0b01, 3, 1, 13, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0xffff_ffff_3030_4040,
            0xffff_ffff_8000_1234,
            0x7000_0000,
        ),
        (
            "bics_x_all_ones_left_imm_lsr_opkind_sets_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsr,
                    amount: 8,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(1, 0b01, 1, 1, 8, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b01, 1, 1, 8, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0x4949_5959_6969_7979,
            0x8000_0000_0000_0000,
            0x3000_0000,
        ),
        (
            "bics_w_masked_all_ones_left_imm_asr_opkind_sets_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Asr,
                    amount: 31,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(0, 0b01, 2, 1, 31, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_logical_shift_regs(0, 0b01, 2, 1, 31, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_5050_6060,
            0x0000_0000_8000_1234,
            0xb000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_andnot_all_ones_left_extended_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "bic_x_all_ones_left_imm_uxtw_opkind_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtw,
                    shift: 0,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_ext_regs(1, 0, 0, 0b010, 0, RD, 31, RM),
                enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RD),
                NOP,
            ],
            [
                enc_addsub_ext_regs(1, 0, 0, 0b010, 0, RD, 31, RM),
                enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RD),
                0xd65f_03c0,
            ],
            0x2a2a_3a3a_4a4a_5a5a,
            0xffff_ffff_0000_0100,
            0x9000_0000,
        ),
        (
            "bic_x_all_ones_left_imm_sxtw_lsl2_opkind_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_ext_regs(1, 0, 0, 0b110, 2, RD, 31, RM),
                enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RD),
                NOP,
            ],
            [
                enc_addsub_ext_regs(1, 0, 0, 0b110, 2, RD, 31, RM),
                enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RD),
                0xd65f_03c0,
            ],
            0x2b2b_3b3b_4b4b_5b5b,
            0x0000_0000_ffff_ffff,
            0x5000_0000,
        ),
        (
            "bic_w_masked_all_ones_left_imm_uxtb_lsl1_opkind_zero_ext_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_ext_regs(0, 0, 0, 0b000, 1, RD, 31, RM),
                enc_logical_shift_regs(0, 0b01, 0, 1, 0, RD, 31, RD),
                NOP,
            ],
            [
                enc_addsub_ext_regs(0, 0, 0, 0b000, 1, RD, 31, RM),
                enc_logical_shift_regs(0, 0b01, 0, 1, 0, RD, 31, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_2c2c_3c3c,
            0xffff_ffff_ffff_ff0f,
            0xd000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_or_all_ones_left_imm_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "orr_x_all_ones_left_imm_reg_opkind_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(1, 0b00, 0, 0), NOP, NOP],
            [enc_mov_wide(1, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0x2d2d_3d3d_4d4d_5d5d,
            0x1111_2222_3333_4444,
            0x3000_0000,
        ),
        (
            "orr_w_masked_all_ones_left_imm_reg_opkind_zero_ext_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(0, 0b00, 0, 0), NOP, NOP],
            [enc_mov_wide(0, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0xffff_ffff_2e2e_3e3e,
            0x5555_6666_7777_8888,
            0x5000_0000,
        ),
        (
            "orrs_x_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_mov_wide(1, 0b00, 0, 0),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_mov_wide(1, 0b00, 0, 0),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0x4d4d_5d5d_6d6d_7d7d,
            0x9999_aaaa_bbbb_cccc,
            0x7000_0000,
        ),
        (
            "orrs_w_masked_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [
                enc_mov_wide(0, 0b00, 0, 0),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_mov_wide(0, 0b00, 0, 0),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_4e4e_5e5e,
            0xdddd_eeee_ffff_0000,
            0x9000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_or_all_ones_left_shifted_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "orr_x_all_ones_left_imm_lsl_opkind_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsl,
                    amount: 4,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(1, 0b00, 0, 0), NOP, NOP],
            [enc_mov_wide(1, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0x2f2f_3f3f_4f4f_5f5f,
            0x1111_2222_3333_4444,
            0x3000_0000,
        ),
        (
            "orr_w_masked_all_ones_left_imm_ror_opkind_zero_ext_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Ror,
                    amount: 13,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(0, 0b00, 0, 0), NOP, NOP],
            [enc_mov_wide(0, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0xffff_ffff_3030_4040,
            0x5555_6666_7777_8888,
            0x5000_0000,
        ),
        (
            "orrs_x_all_ones_left_imm_lsr_opkind_sets_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsr,
                    amount: 8,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_mov_wide(1, 0b00, 0, 0),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_mov_wide(1, 0b00, 0, 0),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0x4f4f_5f5f_6f6f_7f7f,
            0x9999_aaaa_bbbb_cccc,
            0x7000_0000,
        ),
        (
            "orrs_w_masked_all_ones_left_imm_asr_opkind_sets_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Asr,
                    amount: 31,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [
                enc_mov_wide(0, 0b00, 0, 0),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_mov_wide(0, 0b00, 0, 0),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_5050_6060,
            0xdddd_eeee_ffff_0000,
            0x9000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_or_all_ones_left_extended_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "orr_x_all_ones_left_imm_uxtw_opkind_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtw,
                    shift: 0,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(1, 0b00, 0, 0), NOP, NOP],
            [enc_mov_wide(1, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0x3131_4141_5151_6161,
            0x0000_0000_8000_1234,
            0x3000_0000,
        ),
        (
            "orr_w_masked_all_ones_left_imm_uxtb_lsl1_opkind_zero_ext_preserves_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(0, 0b00, 0, 0), NOP, NOP],
            [enc_mov_wide(0, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0xffff_ffff_3232_4242,
            0xffff_ffff_ffff_ff0f,
            0x5000_0000,
        ),
        (
            "orrs_x_all_ones_left_imm_sxtw_lsl2_opkind_sets_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_mov_wide(1, 0b00, 0, 0),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_mov_wide(1, 0b00, 0, 0),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0x5151_6161_7171_8181,
            0x0000_0000_ffff_ffff,
            0x7000_0000,
        ),
        (
            "orrs_w_masked_all_ones_left_imm_sxtb_lsl1_opkind_sets_flags",
            OpKind::Or {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [
                enc_mov_wide(0, 0b00, 0, 0),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_mov_wide(0, 0b00, 0, 0),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_5252_6262,
            0xffff_ffff_ffff_ff80,
            0x9000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_xor_all_ones_left_imm_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "eor_x_all_ones_left_imm_reg_opkind_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(1, 0b10, 0, 1, 0, RD, 31, RM),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b10, 0, 1, 0, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0x3333_4343_5353_6363,
            0x1111_2222_3333_4444,
            0x3000_0000,
        ),
        (
            "eor_w_masked_all_ones_left_imm_reg_opkind_zero_ext_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(0, 0b10, 0, 1, 0, RD, 31, RM),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(0, 0b10, 0, 1, 0, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0xffff_ffff_3434_4444,
            0x5555_6666_7777_8888,
            0x5000_0000,
        ),
        (
            "eors_x_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(1, 0b10, 0, 1, 0, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b10, 0, 1, 0, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0x5353_6363_7373_8383,
            0x9999_aaaa_bbbb_cccc,
            0x7000_0000,
        ),
        (
            "eors_w_masked_all_ones_left_imm_reg_opkind_sets_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(0, 0b10, 0, 1, 0, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_logical_shift_regs(0, 0b10, 0, 1, 0, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_5454_6464,
            0xdddd_eeee_ffff_0000,
            0x9000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_xor_all_ones_left_shifted_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "eor_x_all_ones_left_imm_lsl_opkind_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsl,
                    amount: 4,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(1, 0b10, 0, 1, 4, RD, 31, RM),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b10, 0, 1, 4, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0x3535_4545_5555_6565,
            0x1111_2222_3333_4444,
            0x3000_0000,
        ),
        (
            "eor_w_masked_all_ones_left_imm_ror_opkind_zero_ext_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Ror,
                    amount: 13,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(0, 0b10, 3, 1, 13, RD, 31, RM),
                NOP,
                NOP,
            ],
            [
                enc_logical_shift_regs(0, 0b10, 3, 1, 13, RD, 31, RM),
                0xd65f_03c0,
                NOP,
            ],
            0xffff_ffff_3636_4646,
            0x5555_6666_7777_8888,
            0x5000_0000,
        ),
        (
            "eors_x_all_ones_left_imm_lsr_opkind_sets_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsr,
                    amount: 8,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(1, 0b10, 1, 1, 8, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_logical_shift_regs(1, 0b10, 1, 1, 8, RD, 31, RM),
                enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0x5555_6565_7575_8585,
            0x9999_aaaa_bbbb_cccc,
            0x7000_0000,
        ),
        (
            "eors_w_masked_all_ones_left_imm_asr_opkind_sets_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Asr,
                    amount: 31,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [
                enc_logical_shift_regs(0, 0b10, 2, 1, 31, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_logical_shift_regs(0, 0b10, 2, 1, 31, RD, 31, RM),
                enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_5656_6666,
            0xdddd_eeee_ffff_0000,
            0x9000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_xor_all_ones_left_extended_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "eor_x_all_ones_left_imm_uxtw_opkind_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtw,
                    shift: 0,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_ext_regs(1, 0, 0, 0b010, 0, RD, 31, RM),
                enc_logical_reg_n(1, 0b10, 1, RD, 31, RD),
                NOP,
            ],
            [
                enc_addsub_ext_regs(1, 0, 0, 0b010, 0, RD, 31, RM),
                enc_logical_reg_n(1, 0b10, 1, RD, 31, RD),
                0xd65f_03c0,
            ],
            0x3737_4747_5757_6767,
            0xffff_ffff_0000_0100,
            0x3000_0000,
        ),
        (
            "eor_x_all_ones_left_imm_sxtw_lsl2_opkind_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_ext_regs(1, 0, 0, 0b110, 2, RD, 31, RM),
                enc_logical_reg_n(1, 0b10, 1, RD, 31, RD),
                NOP,
            ],
            [
                enc_addsub_ext_regs(1, 0, 0, 0b110, 2, RD, 31, RM),
                enc_logical_reg_n(1, 0b10, 1, RD, 31, RD),
                0xd65f_03c0,
            ],
            0x3838_4848_5858_6868,
            0x0000_0000_ffff_ffff,
            0x5000_0000,
        ),
        (
            "eor_w_masked_all_ones_left_imm_uxtb_lsl1_opkind_zero_ext_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_ext_regs(0, 0, 0, 0b000, 1, RD, 31, RM),
                enc_logical_reg_n(0, 0b10, 1, RD, 31, RD),
                NOP,
            ],
            [
                enc_addsub_ext_regs(0, 0, 0, 0b000, 1, RD, 31, RM),
                enc_logical_reg_n(0, 0b10, 1, RD, 31, RD),
                0xd65f_03c0,
            ],
            0xffff_ffff_3939_4949,
            0xffff_ffff_ffff_ff0f,
            0xd000_0000,
        ),
    ];

    for (name, op, source, expected_lowered, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected_lowered, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_and_all_ones_left_shifted_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "and_x_all_ones_left_imm_lsl_opkind_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsl,
                    amount: 4,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(1, 0b01, 0, 0, 4, RD, 31, RM),
                NOP,
                NOP,
            ],
            0x2323_3333_4343_5353,
            0x0000_0000_0000_1001,
            0xb000_0000,
        ),
        (
            "and_w_masked_all_ones_left_imm_ror_opkind_zero_ext_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Ror,
                    amount: 13,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [
                enc_logical_shift_regs(0, 0b01, 3, 0, 13, RD, 31, RM),
                NOP,
                NOP,
            ],
            0x2424_3434_4444_5454,
            0xffff_ffff_8000_1234,
            0x5000_0000,
        ),
        (
            "ands_x_all_ones_left_imm_lsr_opkind_sets_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsr,
                    amount: 8,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [enc_addsub_shift_regs(1, 0, 1, 1, 8, RD, 31, RM), NOP, NOP],
            0x4343_5353_6363_7373,
            0x8000_0000_0000_0000,
            0xf000_0000,
        ),
        (
            "ands_w_masked_all_ones_left_imm_asr_opkind_sets_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Asr,
                    amount: 31,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [enc_addsub_shift_regs(0, 0, 1, 2, 31, RD, 31, RM), NOP, NOP],
            0x4444_5454_6464_7474,
            0x0000_0000_8000_1234,
            0x7000_0000,
        ),
    ];

    for (name, op, source, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(
            lowered,
            [source[0], 0xd65f_03c0, NOP],
            "{name}: unexpected lowering"
        );
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_and_all_ones_left_extended_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "and_x_all_ones_left_imm_uxtw_opkind_preserves_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtw,
                    shift: 0,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_addsub_ext_regs(1, 0, 0, 0b010, 0, RD, 31, RM), NOP, NOP],
            0x2525_3535_4545_5555,
            0xffff_ffff_0000_0100,
            0xd000_0000,
        ),
        (
            "ands_x_all_ones_left_imm_sxtw_lsl2_opkind_sets_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [enc_addsub_ext_regs(1, 0, 1, 0b110, 2, RD, 31, RM), NOP, NOP],
            0x4545_5555_6565_7575,
            0x0000_0000_ffff_ffff,
            0x1000_0000,
        ),
        (
            "ands_w_masked_all_ones_left_imm_uxtb_lsl1_opkind_sets_flags",
            OpKind::And {
                dst: arm_x(0),
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [enc_addsub_ext_regs(0, 0, 1, 0b000, 1, RD, 31, RM), NOP, NOP],
            0x4646_5656_6666_7676,
            0xffff_ffff_ffff_ff0f,
            0x8000_0000,
        ),
    ];

    for (name, op, source, x0, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(
            lowered,
            [source[0], 0xd65f_03c0, NOP],
            "{name}: unexpected lowering"
        );
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_test_all_ones_left_shifted_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "test_x_all_ones_left_imm_lsl_opkind_sets_flags",
            OpKind::Test {
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsl,
                    amount: 4,
                },
                width: OpWidth::W64,
            },
            [enc_addsub_shift_regs(1, 0, 1, 0, 4, 31, 31, RM), NOP, NOP],
            0x0000_0000_0000_1000,
            0xf000_0000,
        ),
        (
            "test_x_all_ones_left_imm_lsr_opkind_sets_flags",
            OpKind::Test {
                src1: VReg::Imm(-1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsr,
                    amount: 8,
                },
                width: OpWidth::W64,
            },
            [enc_addsub_shift_regs(1, 0, 1, 1, 8, 31, 31, RM), NOP, NOP],
            0x8000_0000_0000_0000,
            0xf000_0000,
        ),
        (
            "test_w_all_ones_left_imm_asr_opkind_sets_flags",
            OpKind::Test {
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Asr,
                    amount: 31,
                },
                width: OpWidth::W32,
            },
            [enc_addsub_shift_regs(0, 0, 1, 2, 31, 31, 31, RM), NOP, NOP],
            0x0000_0000_8000_0000,
            0x6000_0000,
        ),
    ];

    for (name, op, source, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(
            lowered,
            [source[0], 0xd65f_03c0, NOP],
            "{name}: unexpected lowering"
        );
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_test_all_ones_left_extended_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let test_cases = [
        (
            "test_x_all_ones_left_imm_uxtw_opkind_sets_flags",
            OpKind::Test {
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtw,
                    shift: 0,
                },
                width: OpWidth::W64,
            },
            [enc_addsub_ext_regs(1, 0, 1, 0b010, 0, 31, 31, RM), NOP, NOP],
            0xffff_ffff_0000_0100,
            0xf000_0000,
        ),
        (
            "test_x_all_ones_left_imm_sxtw_lsl2_opkind_sets_flags",
            OpKind::Test {
                src1: VReg::Imm(-1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
            },
            [enc_addsub_ext_regs(1, 0, 1, 0b110, 2, 31, 31, RM), NOP, NOP],
            0x0000_0000_ffff_ffff,
            0,
        ),
        (
            "test_w_all_ones_left_imm_uxtb_lsl1_opkind_sets_flags",
            OpKind::Test {
                src1: VReg::Imm(0x1_ffff_ffff),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Uxtb,
                    shift: 1,
                },
                width: OpWidth::W32,
            },
            [enc_addsub_ext_regs(0, 0, 1, 0b000, 1, 31, 31, RM), NOP, NOP],
            0xffff_ffff_ffff_ff0f,
            0x8000_0000,
        ),
    ];

    for (name, op, source, x2, pstate) in test_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(
            lowered,
            [source[0], 0xd65f_03c0, NOP],
            "{name}: unexpected lowering"
        );
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_logical_same_source_zero_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let logical_cases = [
        (
            "eor_x_same_sources_as_movz_zero_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b10, 0, 0, 0, 0, 0, 0), NOP, NOP],
            0x1357_9bdf_2468_ace0,
            0x3000_0000,
        ),
        (
            "eor_w_same_sources_as_movz_zero_preserves_flags",
            OpKind::Xor {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(0, 0b10, 0, 0, 0, 0, 0, 0), NOP, NOP],
            0xffff_ffff_89ab_cdef,
            0xc000_0000,
        ),
        (
            "bic_x_same_sources_as_movz_zero_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(1, 0b00, 0, 1, 0, 0, 0, 0), NOP, NOP],
            0xfedc_ba98_7654_3210,
            0x9000_0000,
        ),
        (
            "bic_w_same_sources_as_movz_zero_preserves_flags",
            OpKind::AndNot {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_logical_shift_regs(0, 0b00, 0, 1, 0, 0, 0, 0), NOP, NOP],
            0xffff_ffff_0123_4567,
            0x6000_0000,
        ),
    ];

    for (name, op, source, x0, pstate) in logical_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_subword_logical_flag_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_00ff;
    st.pstate = 0xf000_0000;

    let lowered = lower_aarch64_native_ops(vec![OpKind::And {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0x7f),
        width: OpWidth::W8,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!("ands_w8_sign_clear_imm_sets_subword_flags: native lowering failed: {e}")
    });
    cases.push((
        "ands_w8_sign_clear_imm_sets_subword_flags".into(),
        [
            enc_logical_imm(0, 0b00, 0, 0, 6, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            enc_logical_reg_n(0, 0b11, 0, 31, RD, RD),
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_or_xor_flag_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 0x0000_0000_0000_0001;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Or {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Reg(arm_x(2)),
        width: OpWidth::W64,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| panic!("orrs_x_reg_as_orr_ands: native lowering failed: {e}"));
    cases.push((
        "orrs_x_reg_as_orr_ands".into(),
        [
            enc_logical_shift_regs(1, 0b01, 0, 0, 0, RD, RN, RM),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.x[1] = 0xaaaa_bbbb_1234_5678;
    st.x[2] = 0xcccc_dddd_1234_5678;
    st.pstate = 0xf000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Xor {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Reg(arm_x(2)),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| panic!("eors_w_reg_as_eor_ands: native lowering failed: {e}"));
    cases.push((
        "eors_w_reg_as_eor_ands".into(),
        [
            enc_logical_shift_regs(0, 0b10, 0, 0, 0, RD, RN, RM),
            enc_logical_reg_n(0, 0b11, 0, 31, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_addsub_zero_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let addsub_cases = [
        (
            "add_x_zero_reg_same_reg_as_noop_preserves_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x1111_2222_3333_4444,
            0x2000_0000,
        ),
        (
            "add_x_zero_imm_same_reg_as_noop_preserves_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Imm(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x5555_6666_7777_8888,
            0xa000_0000,
        ),
        (
            "sub_x_zero_reg_same_reg_as_noop_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x9999_aaaa_bbbb_cccc,
            0x5000_0000,
        ),
        (
            "sub_x_zero_imm_same_reg_as_noop_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Imm64(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0xdddd_eeee_ffff_0001,
            0xc000_0000,
        ),
    ];

    for (name, op, x0, pstate) in addsub_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), [0xd65f_03c0, NOP, NOP], lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_addsub_masked_zero_imm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let addsub_cases = [
        (
            "add_w_masked_zero_imm_same_reg_as_self_mov_zero_ext_preserves_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: arm_x(0),
                src2: SrcOperand::Imm64(0x1_0000_0000),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            enc_mov_reg(0, RD, RD),
            0xffff_ffff_8000_0001,
            0xb000_0000,
        ),
        (
            "sub_w_masked_zero_imm_as_mov_zero_ext_preserves_flags",
            OpKind::Sub {
                dst: arm_x(3),
                src1: arm_x(1),
                src2: SrcOperand::Imm64(0x1_0000_0000),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            enc_mov_reg(0, 3, RN),
            0xffff_ffff_8765_4321,
            0x4000_0000,
        ),
    ];

    for (name, op, source, x0_or_x1, pstate) in addsub_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        if matches!(&op, OpKind::Add { .. }) {
            st.x[0] = x0_or_x1;
        } else {
            st.x[1] = x0_or_x1;
        }
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), [source, NOP, NOP], lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_addsub_zero_base_nonzero_imm_flag_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xf000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm(1),
        width: OpWidth::W64,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!("adds_x_zero_base_nonzero_imm_as_movz_adds_reg: native lowering failed: {e}")
    });
    cases.push((
        "adds_x_zero_base_nonzero_imm_as_movz_adds_reg".into(),
        [
            enc_mov_wide(1, 0b10, 0, 1),
            enc_addsub_shift_regs(1, 0, 1, 0, 0, RD, 31, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sub {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm(1),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!("subs_w_zero_base_nonzero_imm_as_movz_subs_reg: native lowering failed: {e}")
    });
    cases.push((
        "subs_w_zero_base_nonzero_imm_as_movz_subs_reg".into(),
        [
            enc_mov_wide(0, 0b10, 0, 1),
            enc_addsub_shift_regs(0, 1, 1, 0, 0, RD, 31, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0xf000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: VReg::virt(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm(1),
        width: OpWidth::W64,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!("adds_x_zero_base_positive_imm_without_destination_scratch_as_msr_nzcv_xzr_clears_flags: native lowering failed: {e}")
    });
    cases.push((
        "adds_x_zero_base_positive_imm_without_destination_scratch_as_msr_nzcv_xzr_clears_flags"
            .into(),
        [enc_msr_nzcv(31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sub {
        dst: VReg::virt(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm64(0xffff_ffff),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!("subs_w_zero_base_masked_neg_one_imm_without_destination_scratch_as_msr_nzcv_xzr_clears_flags: native lowering failed: {e}")
    });
    cases.push((
        "subs_w_zero_base_masked_neg_one_imm_without_destination_scratch_as_msr_nzcv_xzr_clears_flags"
            .into(),
        [enc_msr_nzcv(31), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cmp_zero_base_neg_imm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0xf000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Cmp {
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm(-1),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "cmp_x_zero_base_neg_one_imm_as_msr_nzcv_xzr_clears_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "cmp_x_zero_base_neg_one_imm_as_msr_nzcv_xzr_clears_flags".into(),
        [enc_msr_nzcv(31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Cmp {
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm64(0xffff_ffff),
        width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!("cmp_w_zero_base_masked_neg_one_imm_as_msr_nzcv_xzr_clears_flags: native lowering failed: {e}")
    });
    cases.push((
        "cmp_w_zero_base_masked_neg_one_imm_as_msr_nzcv_xzr_clears_flags".into(),
        [enc_msr_nzcv(31), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cmp_two_imm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0xf000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Cmp {
        src1: VReg::Imm(5),
        src2: SrcOperand::Imm(3),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("cmp_x_two_imms_positive_carry_as_constant_nzcv: native lowering failed: {e}")
    });
    cases.push((
        "cmp_x_two_imms_positive_carry_as_constant_nzcv".into(),
        [
            enc_addsub_shift_regs(1, 1, 1, 0, 0, 31, 31, 31),
            enc_condcmp(1, 1, false, 31, 1, 0b0010),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Cmp {
        src1: VReg::Imm(0x7f),
        src2: SrcOperand::Imm(0xff),
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| {
        panic!("cmp_w8_two_imms_overflow_as_constant_nzcv: native lowering failed: {e}")
    });
    cases.push((
        "cmp_w8_two_imms_overflow_as_constant_nzcv".into(),
        [
            enc_addsub_shift_regs(0, 1, 1, 0, 0, 31, 31, 31),
            enc_condcmp(0, 1, false, 31, 1, 0b1001),
            NOP,
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cmp_zero_base_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let cmp_cases = [
        (
            "cmp_x_zero_base_reg_as_subs_zero_reg_sets_flags",
            OpKind::Cmp {
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(1)),
                width: OpWidth::W64,
            },
            [enc_addsub_shift_regs(1, 1, 1, 0, 0, 31, 31, RN), NOP, NOP],
            0x1111_2222_3333_4444,
            0x0123_4567_89ab_cdef,
            0xf000_0000,
        ),
        (
            "cmp_w8_zero_base_zero_source_reg_as_subs_zero_regs_sets_flags",
            OpKind::Cmp {
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W8,
            },
            [enc_addsub_shift_regs(0, 1, 1, 0, 0, 31, 31, 31), NOP, NOP],
            0x4444_5555_6666_7777,
            0x89ab_cdef_0123_4567,
            0x9000_0000,
        ),
        (
            "cmp_w_zero_base_lsl3_as_subs_zero_shifted_sets_flags",
            OpKind::Cmp {
                src1: VReg::Imm(0),
                src2: SrcOperand::Shifted {
                    reg: arm_x(1),
                    shift: ShiftOp::Lsl,
                    amount: 3,
                },
                width: OpWidth::W32,
            },
            [enc_addsub_shift_regs(0, 1, 1, 0, 3, 31, 31, RN), NOP, NOP],
            0x2222_3333_4444_5555,
            0x0000_0000_1000_0001,
            0,
        ),
        (
            "cmp_x_zero_base_uxtw_lsl2_as_subs_zero_extended_sets_flags",
            OpKind::Cmp {
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(1),
                    extend: ExtendOp::Uxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
            },
            [enc_addsub_ext_regs(1, 1, 1, 0b010, 2, 31, 31, RN), NOP, NOP],
            0x3333_4444_5555_6666,
            0xffff_ffff_8000_0001,
            0x6000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in cmp_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_addsub_carry_nonzero_imm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 5;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sbb {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(1),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sbb_x_nonzero_imm_with_destination_scratch_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sbb_x_nonzero_imm_with_destination_scratch_preserves_flags".into(),
        [
            enc_mov_wide(1, 0b10, 0, 1),
            enc_addsub_carry_regs(1, 1, 0, RD, RN, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 1;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sbb {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(1),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "sbcs_w_nonzero_imm_with_destination_scratch_sets_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "sbcs_w_nonzero_imm_with_destination_scratch_sets_flags".into(),
        [
            enc_mov_wide(0, 0b10, 0, 1),
            enc_addsub_carry_regs(0, 1, 1, RD, RN, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sbb {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm(-1),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "sbb_x_neg_one_imm_alias_as_adc_zero_reg_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "sbb_x_neg_one_imm_alias_as_adc_zero_reg_preserves_flags".into(),
        [enc_addsub_carry_regs(1, 0, 0, RN, RN, 31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_1111;
    st.x[1] = 0x0000_0000_8000_0000;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sbb {
        dst: VReg::virt(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm64(0xffff_ffff),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!("sbcs_w_masked_neg_one_imm_without_destination_scratch_as_adcs_zero_reg_sets_flags: native lowering failed: {e}")
    });
    cases.push((
        "sbcs_w_masked_neg_one_imm_without_destination_scratch_as_adcs_zero_reg_sets_flags".into(),
        [enc_addsub_carry_regs(0, 0, 1, 31, RN, 31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xeeee_ffff_1111_2222;
    st.x[1] = 5;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Adc {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm(-1),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "adc_x_neg_one_imm_alias_as_sbc_zero_reg_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "adc_x_neg_one_imm_alias_as_sbc_zero_reg_preserves_flags".into(),
        [enc_addsub_carry_regs(1, 1, 0, RN, RN, 31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_1111_2222_3333;
    st.x[1] = 0;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Adc {
        dst: VReg::virt(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm64(0xffff_ffff),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!("adcs_w_masked_neg_one_imm_without_destination_scratch_as_sbcs_zero_reg_sets_flags: native lowering failed: {e}")
    });
    cases.push((
        "adcs_w_masked_neg_one_imm_without_destination_scratch_as_sbcs_zero_reg_sets_flags".into(),
        [enc_addsub_carry_regs(0, 1, 1, 31, RN, 31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sbb {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm(-1),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sbb_w8_neg_one_imm_alias_as_adc_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sbb_w8_neg_one_imm_alias_as_adc_uxtb_preserves_flags".into(),
        [
            enc_addsub_carry_regs(0, 0, 0, RN, RN, 31),
            enc_bitfield_regs(0, 0b10, 0, 7, RN, RN),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xffff_ffff_0001_0000;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Adc {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Imm64(0xffff),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("adc_w16_masked_neg_one_imm_alias_as_sbc_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "adc_w16_masked_neg_one_imm_alias_as_sbc_uxth_preserves_flags".into(),
        [
            enc_addsub_carry_regs(0, 1, 0, RN, RN, 31),
            enc_bitfield_regs(0, 0b10, 0, 15, RN, RN),
            NOP,
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_addsub_carry_zero_base_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0x0102_0304_0506_0708;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Adc {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Reg(arm_x(1)),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "adc_x_zero_base_reg_as_adc_zero_reg_base_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "adc_x_zero_base_reg_as_adc_zero_reg_base_preserves_flags".into(),
        [enc_addsub_carry_regs(1, 0, 0, RD, 31, RN), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x4444_5555_6666_7777;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sbb {
        dst: VReg::virt(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm(0),
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    }])
    .unwrap_or_else(|e| {
        panic!("sbcs_w_zero_base_zero_imm_discards_result_sets_flags: native lowering failed: {e}")
    });
    cases.push((
        "sbcs_w_zero_base_zero_imm_discards_result_sets_flags".into(),
        [enc_addsub_carry_regs(0, 1, 1, 31, 31, 31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x5555_6666_7777_8888;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Adc {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm(0xff),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("adc_w8_zero_base_neg_one_imm_alias_as_sbc_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "adc_w8_zero_base_neg_one_imm_alias_as_sbc_uxtb_preserves_flags".into(),
        [
            enc_addsub_carry_regs(0, 1, 0, RD, 31, 31),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_add_zero_base_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Reg(arm_x(1)),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("add_x_zero_base_reg_as_mov_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "add_x_zero_base_reg_as_mov_preserves_flags".into(),
        [enc_mov_reg(1, RD, RN), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x5555_6666_7777_8888;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Reg(arm_x(0)),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("add_x_zero_base_same_reg_as_noop_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "add_x_zero_base_same_reg_as_noop_preserves_flags".into(),
        [0xd65f_03c0, NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_9999_aaaa;
    st.pstate = 0x5000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Reg(arm_x(0)),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("add_w_zero_base_same_reg_as_self_mov_zero_ext_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "add_w_zero_base_same_reg_as_self_mov_zero_ext_preserves_flags".into(),
        [enc_mov_reg(0, RD, RD), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_addsub_zero_base_reg_operand_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let addsub_cases = [
        (
            "sub_x_zero_base_reg_as_sub_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(1)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RN), NOP, NOP],
            0x1111_2222_3333_4444,
            0x0123_4567_89ab_cdef,
            0x6000_0000,
        ),
        (
            "adds_x_zero_base_reg_sets_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(1)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [enc_addsub_shift_regs(1, 0, 1, 0, 0, RD, 31, RN), NOP, NOP],
            0x2222_3333_4444_5555,
            0x8000_0000_0000_0001,
            0,
        ),
        (
            "subs_w_zero_base_lsl2_discards_result_sets_flags",
            OpKind::Sub {
                dst: VReg::virt(240),
                src1: VReg::Imm(0),
                src2: SrcOperand::Shifted {
                    reg: arm_x(1),
                    shift: ShiftOp::Lsl,
                    amount: 2,
                },
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [enc_addsub_shift_regs(0, 1, 1, 0, 2, 31, 31, RN), NOP, NOP],
            0x3333_4444_5555_6666,
            0x0000_0000_2000_0001,
            0xf000_0000,
        ),
        (
            "sub_x_zero_base_ror9_as_sub_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Shifted {
                    reg: arm_x(1),
                    shift: ShiftOp::Ror,
                    amount: 9,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [
                enc_extract(1, RN, RN, 9),
                enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RD),
                NOP,
            ],
            0x5555_6666_7777_8888,
            0x0123_4567_89ab_cdef,
            0x5000_0000,
        ),
        (
            "add_x_zero_base_uxtw_lsl2_as_add_extended_preserves_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(1),
                    extend: ExtendOp::Uxtw,
                    shift: 2,
                },
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_addsub_ext_regs(1, 0, 0, 0b010, 2, RD, 31, RN), NOP, NOP],
            0x4444_5555_6666_7777,
            0xffff_ffff_8000_0001,
            0x9000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in addsub_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_subword_addsub_zero_base_reg_operand_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let addsub_cases = [
        (
            "sub_w8_zero_base_reg_as_sub_uxtb_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Reg(arm_x(1)),
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, RN),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
                NOP,
            ],
            0x6666_7777_8888_9999,
            0xaaaa_bbbb_cccc_dd81,
            0x3000_0000,
        ),
        (
            "add_w16_zero_base_lsl3_as_add_uxth_preserves_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Shifted {
                    reg: arm_x(1),
                    shift: ShiftOp::Lsl,
                    amount: 3,
                },
                width: OpWidth::W16,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_shift_regs(1, 0, 0, 0, 3, RD, 31, RN),
                enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
                NOP,
            ],
            0x7777_8888_9999_aaaa,
            0x0000_0000_0000_1003,
            0xc000_0000,
        ),
        (
            "sub_w8_zero_base_asr40_as_sub_uxtb_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Shifted {
                    reg: arm_x(1),
                    shift: ShiftOp::Asr,
                    amount: 40,
                },
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_shift_regs(1, 1, 0, 2, 40, RD, 31, RN),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
                NOP,
            ],
            0x7777_8888_9999_aaaa,
            0x8000_0000_0000_0081,
            0xc000_0000,
        ),
        (
            "sub_w8_zero_base_ror13_as_sub_uxtb_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Shifted {
                    reg: arm_x(1),
                    shift: ShiftOp::Ror,
                    amount: 13,
                },
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [
                enc_extract(1, RN, RN, 13),
                enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RD),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            ],
            0x8888_9999_aaaa_bbbb,
            0x0123_4567_89ab_cdef,
            0x5000_0000,
        ),
        (
            "sub_w8_zero_base_uxtb_lsl1_as_sub_uxtb_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::Imm(0),
                src2: SrcOperand::Extended {
                    reg: arm_x(1),
                    extend: ExtendOp::Uxtb,
                    shift: 1,
                },
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_ext_regs(1, 1, 0, 0b000, 1, RD, 31, RN),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
                NOP,
            ],
            0x8888_9999_aaaa_bbbb,
            0xffff_ffff_ffff_ff81,
            0xa000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in addsub_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_subword_addsub_reg_operand_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let addsub_cases = [
        (
            "add_w8_lsl2_as_add_uxtb_preserves_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsl,
                    amount: 2,
                },
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_shift_regs(1, 0, 0, 0, 2, RD, RN, RM),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
                NOP,
            ],
            0x9999_aaaa_bbbb_cccc,
            0xffff_ffff_0000_00f1,
            0xaaaa_bbbb_cccc_dd03,
            0x6000_0000,
        ),
        (
            "add_w8_lsr40_as_add_uxtb_preserves_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Lsr,
                    amount: 40,
                },
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_shift_regs(1, 0, 0, 1, 40, RD, RN, RM),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
                NOP,
            ],
            0x9999_aaaa_bbbb_cccc,
            0xffff_ffff_0000_00f1,
            0xaaaa_bbbb_ccdd_0300,
            0x6000_0000,
        ),
        (
            "add_w16_ror52_as_add_uxth_preserves_flags",
            OpKind::Add {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Ror,
                    amount: 52,
                },
                width: OpWidth::W16,
                flags: FlagUpdate::None,
            },
            [
                enc_extract(1, RM, RM, 52),
                enc_addsub_shift_regs(1, 0, 0, 0, 0, RD, RN, RD),
                enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            ],
            0xaaaa_bbbb_cccc_dddd,
            0xffff_ffff_0000_0100,
            0x0123_4567_89ab_cdef,
            0x9000_0000,
        ),
        (
            "sub_w16_asr36_as_sub_uxth_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Shifted {
                    reg: arm_x(2),
                    shift: ShiftOp::Asr,
                    amount: 36,
                },
                width: OpWidth::W16,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_shift_regs(1, 1, 0, 2, 36, RD, RN, RM),
                enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
                NOP,
            ],
            0xaaaa_bbbb_cccc_dddd,
            0xffff_ffff_0000_0100,
            0xf000_0000_0000_1000,
            0xb000_0000,
        ),
        (
            "sub_w16_sxth_lsl1_as_sub_uxth_preserves_flags",
            OpKind::Sub {
                dst: arm_x(0),
                src1: arm_x(1),
                src2: SrcOperand::Extended {
                    reg: arm_x(2),
                    extend: ExtendOp::Sxth,
                    shift: 1,
                },
                width: OpWidth::W16,
                flags: FlagUpdate::None,
            },
            [
                enc_addsub_ext_regs(1, 1, 0, 0b101, 1, RD, RN, RM),
                enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
                NOP,
            ],
            0xaaaa_bbbb_cccc_dddd,
            0xffff_ffff_0000_0100,
            0x1111_2222_3333_8003,
            0xb000_0000,
        ),
    ];

    for (name, op, source, x0, x1, x2, pstate) in addsub_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_addsub_effective_zero_ror_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0x0102_0304_0506_0708;
    st.x[2] = 0x1111_2222_3333_4444;
    st.pstate = 0x9000_0000;
    let op = OpKind::Add {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Shifted {
            reg: arm_x(2),
            shift: ShiftOp::Ror,
            amount: 64,
        },
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    };
    let lowered = lower_aarch64_native_ops(vec![op]).unwrap_or_else(|e| {
        panic!("add_x_ror64_same_dst_base_as_add_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "add_x_ror64_same_dst_base_as_add_preserves_flags".into(),
        [enc_addsub_shift_regs(1, 0, 0, 0, 0, RN, RN, RM), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0;
    st.x[2] = 0;
    st.pstate = 0x2000_0000;
    let op = OpKind::Sub {
        dst: VReg::virt(240),
        src1: arm_x(1),
        src2: SrcOperand::Shifted {
            reg: arm_x(2),
            shift: ShiftOp::Ror,
            amount: 32,
        },
        width: OpWidth::W32,
        flags: FlagUpdate::All,
    };
    let lowered = lower_aarch64_native_ops(vec![op]).unwrap_or_else(|e| {
        panic!("subs_w_ror32_discards_result_sets_flags: native lowering failed: {e}")
    });
    cases.push((
        "subs_w_ror32_discards_result_sets_flags".into(),
        [enc_addsub_shift_regs(0, 1, 1, 0, 0, 31, RN, RM), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1111_2222_3333_00f1;
    st.x[2] = 0xaaaa_bbbb_cccc_0010;
    st.pstate = 0x5000_0000;
    let op = OpKind::Add {
        dst: arm_x(1),
        src1: arm_x(1),
        src2: SrcOperand::Shifted {
            reg: arm_x(2),
            shift: ShiftOp::Ror,
            amount: 64,
        },
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    };
    let lowered = lower_aarch64_native_ops(vec![op]).unwrap_or_else(|e| {
        panic!(
            "add_w8_ror64_same_dst_base_as_add_uxtb_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "add_w8_ror64_same_dst_base_as_add_uxtb_preserves_flags".into(),
        [
            enc_addsub_shift_regs(1, 0, 0, 0, 0, RN, RN, RM),
            enc_bitfield_regs(0, 0b10, 0, 7, RN, RN),
            NOP,
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_shift_zero_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let shift_cases = [
        (
            "shl_x_zero_same_reg_as_noop_preserves_flags",
            OpKind::Shl {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Imm(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x0123_4567_89ab_cdef,
            0x3000_0000,
        ),
        (
            "shr_x_effective_zero_same_reg_as_noop_preserves_flags",
            OpKind::Shr {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Imm64(64),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0xfedc_ba98_7654_3210,
            0x6000_0000,
        ),
        (
            "sar_x_zero_same_reg_as_noop_preserves_flags",
            OpKind::Sar {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Imm(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x8000_0000_0000_0001,
            0x9000_0000,
        ),
        (
            "ror_x_effective_zero_same_reg_as_noop_preserves_flags",
            OpKind::Ror {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Imm64(64),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x1357_9bdf_2468_ace0,
            0xc000_0000,
        ),
        (
            "rol_x_effective_zero_same_reg_as_noop_preserves_flags",
            OpKind::Rol {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Imm(64),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x0f0f_f0f0_aaaa_5555,
            0xf000_0000,
        ),
    ];

    for (name, op, x0, pstate) in shift_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), [0xd65f_03c0, NOP, NOP], lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_carry_rotate_zero_count_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let carry_rotate_cases = [
        (
            "rcl_x_effective_zero_same_reg_as_noop_preserves_flags",
            OpKind::Rcl {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Imm64(64),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            [0xd65f_03c0, NOP, NOP],
            0x0123_4567_89ab_cdef,
            0,
            0x3000_0000,
        ),
        (
            "rcr_w_effective_zero_same_reg_as_self_mov_zero_ext_preserves_flags",
            OpKind::Rcr {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Imm(32),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            [enc_mov_reg(0, RD, RD), NOP, NOP],
            0xfedc_ba98_7654_3210,
            0,
            0x6000_0000,
        ),
        (
            "rcl_w8_effective_zero_extracts_low_bits_preserves_flags",
            OpKind::Rcl {
                dst: arm_x(0),
                src: arm_x(1),
                amount: SrcOperand::Imm(9),
                width: OpWidth::W8,
                flags: FlagUpdate::All,
            },
            [enc_bitfield(0, 0b10, 0, 7), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x1234_5678_9abc_de77,
            0x9000_0000,
        ),
    ];

    for (name, op, expected, x0, x1, pstate) in carry_rotate_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), expected, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_shift_zero_count_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let shift_cases = [
        (
            "lslv_x_zero_count_reg_same_dst_as_noop_preserves_flags",
            OpKind::Shl {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(1, 0b1000, 0, 31, 0), NOP, NOP],
            0x0123_4567_89ab_cdef,
            0,
            0x3000_0000,
        ),
        (
            "lsrv_w_zero_count_reg_same_dst_as_self_mov_zero_ext_preserves_flags",
            OpKind::Shr {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(0, 0b1001, 0, 31, 0), NOP, NOP],
            0xffff_ffff_89ab_cdef,
            0,
            0x6000_0000,
        ),
        (
            "asrv_x_zero_count_reg_as_mov_preserves_flags",
            OpKind::Sar {
                dst: arm_x(0),
                src: arm_x(1),
                amount: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(1, 0b1010, 1, 31, 0), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x8000_0000_0000_0001,
            0x9000_0000,
        ),
        (
            "rorv_w_zero_count_reg_as_mov_zero_ext_preserves_flags",
            OpKind::Ror {
                dst: arm_x(0),
                src: arm_x(1),
                amount: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(0, 0b1011, 1, 31, 0), NOP, NOP],
            0xbbbb_cccc_dddd_eeee,
            0xffff_ffff_7654_3210,
            0xc000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in shift_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_shift_zero_imm_source_reg_count_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let shift_cases = [
        (
            "lslv_w_zero_imm_source_reg_count_as_movz_preserves_flags",
            OpKind::Shl {
                dst: arm_x(0),
                src: VReg::Imm(0),
                amount: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(0, 0b1000, 31, RM, RD), NOP, NOP],
            [enc_mov_wide(0, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0xaaaa_bbbb_cccc_dddd,
            37,
            0x9000_0000,
        ),
        (
            "lsrv_w8_zero_imm_source_reg_count_as_movz_preserves_flags",
            OpKind::Shr {
                dst: arm_x(0),
                src: VReg::Imm(0),
                amount: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(0, 0b1001, 31, RM, RD), NOP, NOP],
            [enc_mov_wide(0, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0xbbbb_cccc_dddd_eeee,
            9,
            0x6000_0000,
        ),
        (
            "asrv_w16_zero_imm_source_reg_count_as_movz_preserves_flags",
            OpKind::Sar {
                dst: arm_x(0),
                src: VReg::Imm(0),
                amount: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W16,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(0, 0b1010, 31, RM, RD), NOP, NOP],
            [enc_mov_wide(0, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0xcccc_dddd_eeee_ffff,
            17,
            0x3000_0000,
        ),
        (
            "rorv_x_zero_imm_source_reg_count_as_movz_preserves_flags",
            OpKind::Ror {
                dst: arm_x(0),
                src: VReg::Imm(0),
                amount: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(1, 0b1011, 31, RM, RD), NOP, NOP],
            [enc_mov_wide(1, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0xdddd_eeee_ffff_0000,
            65,
            0xc000_0000,
        ),
    ];

    for (name, op, source, expected, x0, x2, pstate) in shift_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_shift_all_ones_imm_source_reg_count_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let shift_cases = [
        (
            "asrv_w_all_ones_imm_source_reg_count_as_movn_preserves_flags",
            OpKind::Sar {
                dst: arm_x(0),
                src: VReg::Imm(-1),
                amount: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(0, 0b1010, RN, RM, RD), NOP, NOP],
            [enc_mov_wide(0, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0xffff_ffff,
            13,
            0x8000_0000,
        ),
        (
            "rorv_x_all_ones_imm_source_reg_count_as_movn_preserves_flags",
            OpKind::Ror {
                dst: arm_x(0),
                src: VReg::Imm(-1),
                amount: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(1, 0b1011, RN, RM, RD), NOP, NOP],
            [enc_mov_wide(1, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0xbbbb_cccc_dddd_eeee,
            u64::MAX,
            65,
            0x5000_0000,
        ),
    ];

    for (name, op, source, expected, x0, x1, x2, pstate) in shift_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_rol_constant_imm_source_reg_count_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let rol_cases = [
        (
            "rol_x_zero_imm_source_reg_count_as_movz_preserves_flags",
            OpKind::Rol {
                dst: arm_x(0),
                src: VReg::Imm(0),
                amount: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(1, 0b1011, 31, RM, RD), NOP, NOP],
            [enc_mov_wide(1, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0,
            19,
            0x8000_0000,
        ),
        (
            "rol_w_all_ones_imm_source_reg_count_as_movn_preserves_flags",
            OpKind::Rol {
                dst: arm_x(0),
                src: VReg::Imm(-1),
                amount: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(0, 0b1011, RN, RM, RD), NOP, NOP],
            [enc_mov_wide(0, 0b00, 0, 0), 0xd65f_03c0, NOP],
            0xbbbb_cccc_dddd_eeee,
            0xffff_ffff,
            35,
            0x2000_0000,
        ),
    ];

    for (name, op, source, expected, x0, x1, x2, pstate) in rol_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_rol_zero_count_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let rol_cases = [
        (
            "rol_x_zero_count_reg_same_dst_as_noop_preserves_flags",
            OpKind::Rol {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(1, 0b1011, 0, 31, 0), NOP, NOP],
            0x0123_4567_89ab_cdef,
            0,
            0x3000_0000,
        ),
        (
            "rol_w_zero_count_reg_same_dst_as_self_mov_zero_ext_preserves_flags",
            OpKind::Rol {
                dst: arm_x(0),
                src: arm_x(0),
                amount: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(0, 0b1011, 0, 31, 0), NOP, NOP],
            0xffff_ffff_89ab_cdef,
            0,
            0x6000_0000,
        ),
        (
            "rol_x_zero_count_reg_as_mov_preserves_flags",
            OpKind::Rol {
                dst: arm_x(0),
                src: arm_x(1),
                amount: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_dp2_regs(1, 0b1011, 1, 31, 0), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x8000_0000_0000_0001,
            0x9000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in rol_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_double_shift_zero_count_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let double_shift_cases = [
        (
            "shrd_x_zero_count_as_noop_preserves_flags",
            OpKind::Shrd {
                dst: arm_x(0),
                src: arm_x(1),
                amount: SrcOperand::Imm(0),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x0123_4567_89ab_cdef,
            0xfedc_ba98_7654_3210,
            0x4000_0000,
        ),
        (
            "shld_x_effective_zero_count_as_noop_preserves_flags",
            OpKind::Shld {
                dst: arm_x(0),
                src: arm_x(1),
                amount: SrcOperand::Imm64(64),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x1357_9bdf_2468_ace0,
            0x0f0f_f0f0_aaaa_5555,
            0xb000_0000,
        ),
    ];

    for (name, op, x0, x1, pstate) in double_shift_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), [0xd65f_03c0, NOP, NOP], lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_mul_one_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mul_noop_cases = [
        (
            "mulu_x_imm_one_same_reg_as_noop_preserves_flags",
            OpKind::MulU {
                dst_lo: arm_x(0),
                dst_hi: None,
                src1: arm_x(0),
                src2: SrcOperand::Imm(1),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0x0123_4567_89ab_cdef,
            0x2000_0000,
        ),
        (
            "muls_x_left_imm_one_same_reg_as_noop_preserves_flags",
            OpKind::MulS {
                dst_lo: arm_x(0),
                dst_hi: None,
                src1: VReg::Imm(1),
                src2: SrcOperand::Reg(arm_x(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            0xfedc_ba98_7654_3210,
            0x9000_0000,
        ),
    ];

    for (name, op, x0, pstate) in mul_noop_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), [0xd65f_03c0, NOP, NOP], lowered, st));
    }

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulU {
        dst_lo: arm_x(0),
        dst_hi: None,
        src1: arm_x(0),
        src2: SrcOperand::Imm(1),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("mulu_w_imm_one_same_reg_as_self_mov_zero_ext_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mulu_w_imm_one_same_reg_as_self_mov_zero_ext_preserves_flags".into(),
        [enc_mov_reg(0, RD, RD), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_mul_zero_source_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mul_zero_cases = [
        (
            "mulu_x_zero_source_reg_as_movz_preserves_flags",
            OpKind::MulU {
                dst_lo: arm_x(0),
                dst_hi: None,
                src1: arm_x(1),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_dp3_ra_regs(1, 0b000, 0, 0, 1, 31, 31), NOP, NOP],
            0x0123_4567_89ab_cdef,
            0xfedc_ba98_7654_3210,
            0x2000_0000,
        ),
        (
            "muls_w_zero_source_reg_as_movz_zero_ext_preserves_flags",
            OpKind::MulS {
                dst_lo: arm_x(0),
                dst_hi: None,
                src1: arm_x(1),
                src2: SrcOperand::Reg(VReg::Imm(0)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            [enc_dp3_ra_regs(0, 0b000, 0, 0, 1, 31, 31), NOP, NOP],
            0xffff_ffff_89ab_cdef,
            0x0f0f_f0f0_aaaa_5555,
            0x9000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in mul_zero_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x8000_0000_0000_0001;
    st.pstate = 0x5000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulS {
        dst_lo: VReg::virt(0),
        dst_hi: Some(arm_x(0)),
        src1: arm_x(1),
        src2: SrcOperand::Reg(VReg::Imm(0)),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("smulh_x_zero_source_reg_as_movz_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "smulh_x_zero_source_reg_as_movz_preserves_flags".into(),
        [enc_dp3_ra_regs(1, 0b010, 0, 0, 1, 31, 31), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.x[2] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulU {
        dst_lo: arm_x(1),
        dst_hi: Some(arm_x(2)),
        src1: arm_x(1),
        src2: SrcOperand::Reg(VReg::Imm(0)),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "mulu_full_width_zero_source_reg_outputs_alias_sources_as_movz_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "mulu_full_width_zero_source_reg_outputs_alias_sources_as_movz_preserves_flags".into(),
        [
            enc_dp3_ra_regs(1, 0b000, 0, 1, 1, 31, 31),
            enc_dp3_ra_regs(1, 0b110, 0, 2, 1, 31, 31),
            NOP,
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_mul_full_width_const_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[3] = 0x1111_2222_3333_4444;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulS {
        dst_lo: arm_x(0),
        dst_hi: Some(arm_x(3)),
        src1: VReg::Imm(-2),
        src2: SrcOperand::Imm64(3),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("muls_full_width_two_imms_as_movn_pair_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "muls_full_width_two_imms_as_movn_pair_preserves_flags".into(),
        [
            enc_mov_wide(1, 0b00, 0, 5),
            enc_mov_wide_regs(1, 0b00, 0, 0, 3),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulU {
        dst_lo: VReg::virt(0),
        dst_hi: Some(arm_x(0)),
        src1: VReg::Imm(-1),
        src2: SrcOperand::Imm64(2),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("umulh_x_two_imms_as_movz_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "umulh_x_two_imms_as_movz_preserves_flags".into(),
        [enc_mov_wide(1, 0b10, 0, 1), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_mulacc_masked_identity_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mulacc_cases = [
        (
            "muladd_w16_imm_masked_one_as_add_uxth_preserves_flags",
            OpKind::MulAdd {
                dst: arm_x(0),
                acc: arm_x(3),
                src1: VReg::Imm(0x1_0001),
                src2: arm_x(1),
                width: OpWidth::W16,
            },
            [
                enc_addsub_shift_regs(0, 0, 0, 0, 0, RD, 3, RN),
                enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
                NOP,
            ],
            0xaaaa_bbbb_cccc_dddd,
            0xffff_ffff_0000_5678,
            0x1111_2222_3333_4444,
            0x3000_0000,
        ),
        (
            "mulsub_w8_imm_masked_neg_one_as_add_uxtb_preserves_flags",
            OpKind::MulSub {
                dst: arm_x(0),
                acc: arm_x(3),
                src1: arm_x(1),
                src2: VReg::Imm(0x1ff),
                width: OpWidth::W8,
            },
            [
                enc_addsub_shift_regs(0, 0, 0, 0, 0, RD, 3, RN),
                enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
                NOP,
            ],
            0xbbbb_cccc_dddd_eeee,
            0xffff_ffff_ffff_00f0,
            0x2222_3333_4444_5555,
            0xa000_0000,
        ),
    ];

    for (name, op, source, x0, x1, x3, pstate) in mulacc_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.x[3] = x3;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_lea_absolute_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x4444_5555_6666_7777;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Lea {
        dst: arm_x(0),
        addr: Address::Absolute(0xffff_ffff_ffff_fff1),
    }])
    .unwrap_or_else(|e| {
        panic!("lea_absolute_negative_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "lea_absolute_negative_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0xe), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_lea_pcrel_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x5555_6666_7777_8888;
    st.pstate = 0xc000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Lea {
        dst: arm_x(0),
        addr: Address::PcRel {
            offset: -15,
            disp_size: DispSize::Auto,
            base: Some(0),
        },
    }])
    .unwrap_or_else(|e| {
        panic!("lea_pcrel_negative_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "lea_pcrel_negative_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0xe), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bfi_full_width_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bfi {
        dst: arm_x(0),
        dst_in: arm_x(1),
        src: VReg::Imm(-15),
        lsb: 0,
        width_bits: 32,
        op_width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "bfi_w_full_width_negative_imm_src_as_movn_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "bfi_w_full_width_negative_imm_src_as_movn_preserves_flags".into(),
        [enc_mov_wide(0, 0b00, 0, 0xe), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x5000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bfi {
        dst: arm_x(0),
        dst_in: arm_x(1),
        src: VReg::Imm(-15),
        lsb: 0,
        width_bits: 64,
        op_width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "bfi_x_full_width_negative_imm_src_as_movn_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "bfi_x_full_width_negative_imm_src_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0xe), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bfi_full_width_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bfi_cases = [
        (
            "bfi_x_full_width_same_src_as_noop_preserves_flags",
            OpKind::Bfi {
                dst: arm_x(0),
                dst_in: arm_x(1),
                src: arm_x(0),
                lsb: 0,
                width_bits: 64,
                op_width: OpWidth::W64,
            },
            [0xd65f_03c0u32, NOP, NOP],
            0x1111_2222_3333_4444,
            0xaaaa_bbbb_cccc_dddd,
            0x2000_0000,
        ),
        (
            "bfi_w_full_width_same_src_as_self_mov_zero_ext_preserves_flags",
            OpKind::Bfi {
                dst: arm_x(0),
                dst_in: arm_x(1),
                src: arm_x(0),
                lsb: 0,
                width_bits: 32,
                op_width: OpWidth::W32,
            },
            [enc_mov_reg(0, RD, RD), NOP, NOP],
            0xffff_ffff_8765_4321,
            0xbbbb_cccc_dddd_eeee,
            0x7000_0000,
        ),
        (
            "bfi_x_full_width_reg_as_mov_preserves_flags",
            OpKind::Bfi {
                dst: arm_x(0),
                dst_in: arm_x(0),
                src: arm_x(1),
                lsb: 0,
                width_bits: 64,
                op_width: OpWidth::W64,
            },
            [enc_mov_reg(1, RD, RN), NOP, NOP],
            0x3333_4444_5555_6666,
            0x8000_0000_0000_0001,
            0xa000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in bfi_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bfx_full_width_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bfx_cases = [
        (
            "ubfx_x_full_width_same_dst_as_noop_preserves_flags",
            OpKind::Bfx {
                dst: arm_x(0),
                src: arm_x(0),
                lsb: 0,
                width_bits: 64,
                sign_extend: false,
                op_width: OpWidth::W64,
            },
            [0xd65f_03c0u32, NOP, NOP],
            0x1111_2222_3333_4444,
            0x3000_0000,
            0,
        ),
        (
            "sbfx_w_full_width_same_dst_as_self_mov_zero_ext_preserves_flags",
            OpKind::Bfx {
                dst: arm_x(0),
                src: arm_x(0),
                lsb: 0,
                width_bits: 32,
                sign_extend: true,
                op_width: OpWidth::W32,
            },
            [enc_mov_reg(0, RD, RD), NOP, NOP],
            0xffff_ffff_8765_4321,
            0x6000_0000,
            0,
        ),
        (
            "sbfx_x_full_width_as_mov_preserves_flags",
            OpKind::Bfx {
                dst: arm_x(0),
                src: arm_x(1),
                lsb: 0,
                width_bits: 64,
                sign_extend: true,
                op_width: OpWidth::W64,
            },
            [enc_mov_reg(1, RD, RN), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x9000_0000,
            0x8000_0000_0000_0001,
        ),
    ];

    for (name, op, source, x0, pstate, x1) in bfx_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bfxil_full_width_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bfxil_cases = [
        (
            "bfxil_x_full_width_same_src_as_noop_preserves_flags",
            arm_x(0),
            arm_x(1),
            arm_x(0),
            64,
            OpWidth::W64,
            [enc_bitfield_regs(1, 0b01, 0, 63, RD, RD), NOP, NOP],
            0x1111_2222_3333_4444,
            0xaaaa_bbbb_cccc_dddd,
            0x4000_0000,
        ),
        (
            "bfxil_w_full_width_same_src_as_self_mov_zero_ext_preserves_flags",
            arm_x(0),
            arm_x(1),
            arm_x(0),
            32,
            OpWidth::W32,
            [enc_bitfield_regs(0, 0b01, 0, 31, RD, RD), NOP, NOP],
            0xffff_ffff_8765_4321,
            0xbbbb_cccc_dddd_eeee,
            0x8000_0000,
        ),
        (
            "bfxil_x_full_width_as_mov_preserves_flags",
            arm_x(0),
            arm_x(0),
            arm_x(1),
            64,
            OpWidth::W64,
            [enc_bitfield_regs(1, 0b01, 0, 63, RN, RD), NOP, NOP],
            0x3333_4444_5555_6666,
            0x8000_0000_0000_0001,
            0xc000_0000,
        ),
    ];

    for (name, dst, dst_in, src, width_bits, op_width, source, x0, x1, pstate) in bfxil_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let extracted = VReg::virt(0);
        let lowered = lower_aarch64_native_ops_same_pc(vec![
            OpKind::Bfx {
                dst: extracted,
                src,
                lsb: 0,
                width_bits,
                sign_extend: false,
                op_width,
            },
            OpKind::Bfi {
                dst,
                dst_in,
                src: extracted,
                lsb: 0,
                width_bits,
                op_width,
            },
        ])
        .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bfxil_imm_source_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0x3c0;
    st.pstate = 0x8000_0000;

    let extracted = VReg::virt(0);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Bfx {
            dst: extracted,
            src: VReg::Imm(0x3c0),
            lsb: 4,
            width_bits: 8,
            sign_extend: false,
            op_width: OpWidth::W64,
        },
        OpKind::Bfi {
            dst: arm_x(0),
            dst_in: arm_x(0),
            src: extracted,
            lsb: 0,
            width_bits: 8,
            op_width: OpWidth::W64,
        },
    ])
    .unwrap_or_else(|e| panic!("bfxil_x_imm_source_as_low_insert_preserves_flags: {e}"));

    cases.push((
        "bfxil_x_imm_source_as_low_insert_preserves_flags".into(),
        [enc_bitfield_regs(1, 0b01, 4, 11, RN, RD), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bit_permute_identity_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bit_permute_cases = [
        (
            "pdep_x_zero_mask_as_zero_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0),
                width: OpWidth::W64,
            },
            [enc_mov_wide(1, 0b10, 0, 0), NOP, NOP],
            0x1111_2222_3333_4444,
            0xaaaa_bbbb_cccc_dddd,
            0x2000_0000,
        ),
        (
            "pext_x_full_mask_same_reg_as_noop_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: arm_x(0),
                mask: VReg::Imm(-1),
                width: OpWidth::W64,
            },
            [0xd65f_03c0, NOP, NOP],
            0x0123_4567_89ab_cdef,
            0,
            0x6000_0000,
        ),
        (
            "pdep_w_full_mask_same_reg_zero_ext_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: arm_x(0),
                mask: VReg::Imm(0xffff_ffff),
                width: OpWidth::W32,
            },
            [enc_mov_reg(0, RD, RD), NOP, NOP],
            0xffff_ffff_8765_4321,
            0,
            0x9000_0000,
        ),
        (
            "pext_w8_full_mask_extracts_low_bits_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0xff),
                width: OpWidth::W8,
            },
            [enc_bitfield(0, 0b10, 0, 7), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x1234_5678_9abc_de77,
            0xc000_0000,
        ),
        (
            "pdep_w16_full_mask_imm_as_movz_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: VReg::Imm(-1),
                mask: VReg::Imm(0xffff),
                width: OpWidth::W16,
            },
            [enc_mov_wide(0, 0b10, 0, 0xffff), NOP, NOP],
            0x2222_3333_4444_5555,
            0,
            0xf000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in bit_permute_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bit_permute_low_mask_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bit_permute_cases = [
        (
            "pdep_x_low_mask_as_and_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0x1fff),
                width: OpWidth::W64,
            },
            [enc_logical_imm(1, 0b00, 1, 0, 12, RN), NOP, NOP],
            0x1111_2222_3333_4444,
            0xaaaa_bbbb_cccc_1ddd,
            0x2000_0000,
        ),
        (
            "pext_w_low_mask_as_and_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0x1f),
                width: OpWidth::W32,
            },
            [enc_logical_imm(0, 0b00, 0, 0, 4, RN), NOP, NOP],
            0xffff_ffff_8765_4321,
            0x1234_5678_9abc_def7,
            0x6000_0000,
        ),
        (
            "pdep_w8_low_mask_as_and_uxtb_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0xf),
                width: OpWidth::W8,
            },
            [enc_logical_imm(0, 0b00, 0, 0, 3, RN), NOP, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x1234_5678_9abc_de7f,
            0x9000_0000,
        ),
        (
            "pext_w16_low_mask_imm_as_movz_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: VReg::Imm(0x12345),
                mask: VReg::Imm(0xff),
                width: OpWidth::W16,
            },
            [enc_mov_wide(0, 0b10, 0, 0x45), NOP, NOP],
            0x2222_3333_4444_5555,
            0,
            0xc000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in bit_permute_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bit_permute_single_bit_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bit_permute_cases = [
        (
            "pext_x_single_bit_as_ubfx_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(1 << 12),
                width: OpWidth::W64,
            },
            [enc_bitfield(1, 0b10, 12, 12), NOP, NOP],
            0x1111_2222_3333_4444,
            0xaaaa_bbbb_cccc_1ddd,
            0x2000_0000,
        ),
        (
            "pdep_x_single_bit_as_ubfiz_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(1 << 12),
                width: OpWidth::W64,
            },
            [enc_bitfield(1, 0b10, 52, 0), NOP, NOP],
            0x2222_3333_4444_5555,
            0x1234_5678_9abc_def1,
            0x6000_0000,
        ),
        (
            "pext_w8_single_bit_as_ubfx_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0x20),
                width: OpWidth::W8,
            },
            [enc_bitfield(0, 0b10, 5, 5), NOP, NOP],
            0xffff_ffff_8765_4321,
            0x1234_5678_9abc_de20,
            0x9000_0000,
        ),
        (
            "pdep_w16_single_bit_imm_as_movz_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: VReg::Imm(3),
                mask: VReg::Imm(0x80),
                width: OpWidth::W16,
            },
            [enc_mov_wide(0, 0b10, 0, 0x80), NOP, NOP],
            0x3333_4444_5555_6666,
            0,
            0xc000_0000,
        ),
        (
            "pext_w16_single_bit_imm_zero_as_movz_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: VReg::Imm(0),
                mask: VReg::Imm(0x4000),
                width: OpWidth::W16,
            },
            [enc_mov_wide(0, 0b10, 0, 0), NOP, NOP],
            0x4444_5555_6666_7777,
            0,
            0xf000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in bit_permute_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bit_permute_contiguous_mask_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bit_permute_cases = [
        (
            "pext_x_contiguous_mask_as_ubfx_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0x1f00),
                width: OpWidth::W64,
            },
            [enc_bitfield(1, 0b10, 8, 12), NOP, NOP],
            0x1111_2222_3333_4444,
            0xaaaa_bbbb_cccc_1ddd,
            0x2000_0000,
        ),
        (
            "pdep_x_contiguous_mask_as_ubfiz_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0x3f_0000),
                width: OpWidth::W64,
            },
            [enc_bitfield(1, 0b10, 48, 5), NOP, NOP],
            0x2222_3333_4444_5555,
            0x1234_5678_9abc_de3f,
            0x6000_0000,
        ),
        (
            "pext_w16_contiguous_mask_as_ubfx_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0x03f0),
                width: OpWidth::W16,
            },
            [enc_bitfield(0, 0b10, 4, 9), NOP, NOP],
            0xffff_ffff_8765_4321,
            0x1234_5678_9abc_d3f0,
            0x9000_0000,
        ),
        (
            "pdep_w8_contiguous_mask_as_ubfiz_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: arm_x(1),
                mask: VReg::Imm(0x70),
                width: OpWidth::W8,
            },
            [enc_bitfield(0, 0b10, 28, 2), NOP, NOP],
            0x3333_4444_5555_6666,
            0x1234_5678_9abc_de07,
            0xc000_0000,
        ),
        (
            "pdep_w16_contiguous_mask_imm_as_movz_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: VReg::Imm(0x3f),
                mask: VReg::Imm(0x3f00),
                width: OpWidth::W16,
            },
            [enc_mov_wide(0, 0b10, 0, 0x3f00), NOP, NOP],
            0x4444_5555_6666_7777,
            0,
            0xf000_0000,
        ),
        (
            "pext_x_contiguous_mask_imm_as_movz_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: VReg::Imm(0xabc0),
                mask: VReg::Imm(0x0ff0),
                width: OpWidth::W64,
            },
            [enc_mov_wide(1, 0b10, 0, 0xbc), NOP, NOP],
            0x5555_6666_7777_8888,
            0,
            0x3000_0000,
        ),
    ];

    for (name, op, source, x0, x1, pstate) in bit_permute_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[1] = x1;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bit_permute_imm_source_arbitrary_mask_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bit_permute_cases = [
        (
            "pext_x_arbitrary_mask_imm_as_movz_preserves_flags",
            OpKind::Pext {
                dst: arm_x(0),
                src: VReg::Imm(0x421),
                mask: VReg::Imm(0x421),
                width: OpWidth::W64,
            },
            [enc_mov_wide(1, 0b10, 0, 7), NOP, NOP],
            0x1111_2222_3333_4444,
            0x2000_0000,
        ),
        (
            "pdep_w16_arbitrary_mask_imm_as_movz_preserves_flags",
            OpKind::Pdep {
                dst: arm_x(0),
                src: VReg::Imm(0b1010),
                mask: VReg::Imm(0x8421),
                width: OpWidth::W16,
            },
            [enc_mov_wide(0, 0b10, 0, 0x8020), NOP, NOP],
            0xffff_ffff_8765_4321,
            0x6000_0000,
        ),
    ];

    for (name, op, source, x0, pstate) in bit_permute_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bit_scan_flag_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut push_case3 = |label: &str, source: [u32; 3], ops: Vec<OpKind>, st: ArmState| {
        let lowered = lower_aarch64_native_ops(ops)
            .unwrap_or_else(|e| panic!("{label}: native lowering failed: {e}"));
        cases.push((label.into(), source, lowered, st));
    };

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0xf000_0000;
    push_case3(
        "bsf_x_imm_one_sets_zf_from_source_before_result_mov",
        [
            enc_mov_wide(1, 0b10, 0, 1),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
            enc_mov_wide(1, 0b10, 0, 0),
        ],
        vec![OpKind::Bsf {
            dst: arm_x(0),
            src: VReg::Imm(1),
            width: OpWidth::W64,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_ffff_aaaa_bbbb;
    st.pstate = 0x2000_0000;
    push_case3(
        "bsr_w_imm_zero_sets_zf_from_source_before_result_mov",
        [
            enc_logical_reg_n(0, 0b11, 0, 31, 31, 31),
            enc_mov_wide(0, 0b10, 0, 0),
            NOP,
        ],
        vec![OpKind::Bsr {
            dst: arm_x(0),
            src: VReg::Imm(0),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bextr_zero_imm_source_reg_control_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bextr_cases = [
        (
            "bextr_x_zero_imm_source_reg_control_as_movz_preserves_flags",
            OpKind::Bextr {
                dst: arm_x(0),
                src: VReg::Imm(0),
                control: arm_x(2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(1, 0b10, 0, 0), NOP, NOP],
            [enc_mov_wide(1, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0xaaaa_bbbb_cccc_dddd,
            0x1234,
            0x8000_0000,
        ),
        (
            "bextr_w_masked_zero_imm_source_self_control_sets_zf",
            OpKind::Bextr {
                dst: arm_x(0),
                src: VReg::Imm(0x1_0000_0000),
                control: arm_x(0),
                width: OpWidth::W32,
                flags: bextr_flags(),
            },
            [
                enc_mov_wide(0, 0b10, 0, 0),
                enc_logical_reg_n(0, 0b11, 0, 31, RD, RD),
                NOP,
            ],
            [
                enc_mov_wide(0, 0b10, 0, 0),
                enc_logical_reg_n(0, 0b11, 0, 31, RD, RD),
                0xd65f_03c0,
            ],
            0xbbbb_cccc_dddd_eeee,
            0,
            0x2000_0000,
        ),
        (
            "bextr_w8_masked_zero_imm_source_reg_control_as_movz_preserves_flags",
            OpKind::Bextr {
                dst: arm_x(0),
                src: VReg::Imm(0x100),
                control: arm_x(2),
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            [enc_mov_wide(0, 0b10, 0, 0), NOP, NOP],
            [enc_mov_wide(0, 0b10, 0, 0), 0xd65f_03c0, NOP],
            0xcccc_dddd_eeee_ffff,
            0x87,
            0x4000_0000,
        ),
    ];

    for (name, op, source, expected, x0, x2, pstate) in bextr_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(lowered, expected, "{name}: unexpected lowering");
        cases.push((name.into(), source, lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bzhi_zero_imm_source_reg_index_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let bzhi_cases = [
        (
            "bzhi_x_zero_imm_source_reg_index_as_movz_preserves_flags",
            OpKind::Bzhi {
                dst: arm_x(0),
                src: VReg::Imm(0),
                index: arm_x(2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            enc_mov_wide(1, 0b10, 0, 0),
            0xaaaa_bbbb_cccc_dddd,
            17,
            0x9000_0000,
        ),
        (
            "bzhi_w_masked_zero_imm_source_self_index_as_movz_preserves_flags",
            OpKind::Bzhi {
                dst: arm_x(0),
                src: VReg::Imm(0x1_0000_0000),
                index: arm_x(0),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            enc_mov_wide(0, 0b10, 0, 0),
            0xbbbb_cccc_dddd_eeee,
            0,
            0x4000_0000,
        ),
        (
            "bzhi_w8_masked_zero_imm_source_reg_index_as_movz_preserves_flags",
            OpKind::Bzhi {
                dst: arm_x(0),
                src: VReg::Imm(0x100),
                index: arm_x(2),
                width: OpWidth::W8,
                flags: FlagUpdate::None,
            },
            enc_mov_wide(0, 0b10, 0, 0),
            0xcccc_dddd_eeee_ffff,
            0x87,
            0x2000_0000,
        ),
    ];

    for (name, op, source, x0, x2, pstate) in bzhi_cases {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st.x[0] = x0;
        st.x[2] = x2;
        st.pstate = pstate;
        let lowered = lower_aarch64_native_ops(vec![op])
            .unwrap_or_else(|e| panic!("{name}: native lowering failed: {e}"));
        assert_eq!(
            lowered,
            [source, 0xd65f_03c0, NOP],
            "{name}: unexpected lowering"
        );
        cases.push((name.into(), [source, NOP, NOP], lowered, st));
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_bzhi_subword_flag_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_00f3;
    st.pstate = 0xf000_0000;

    let lowered = lower_aarch64_native_ops(vec![OpKind::Bzhi {
        dst: arm_x(0),
        src: arm_x(1),
        index: VReg::Imm(5),
        width: OpWidth::W8,
        flags: bzhi_flags(),
    }])
    .unwrap_or_else(|e| {
        panic!("bzhi_w8_imm_index_with_flags_clears_carry: native lowering failed: {e}")
    });
    cases.push((
        "bzhi_w8_imm_index_with_flags_clears_carry".into(),
        [
            enc_logical_imm(0, 0b00, 0, 0, 4, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            enc_logical_reg_n(0, 0b11, 0, 31, RD, RD),
        ],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cmove_imm_movn_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: VReg::Imm(-15),
        cond: Condition::Always,
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_x_always_negative_imm_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "cmove_x_always_negative_imm_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0xe), NOP, NOP],
        lowered,
        st,
    ));

    let source = [
        enc_b_cond(1, 2),
        enc_mov_wide(0, 0b00, 0, 0xe),
        enc_mov_reg(0, RD, RD),
    ];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: VReg::Imm(-15),
        cond: Condition::Eq,
        width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_w_eq_true_negative_imm_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "cmove_w_eq_true_negative_imm_as_movn_preserves_flags".into(),
        source,
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: VReg::Imm(-15),
        cond: Condition::Eq,
        width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "cmove_w_eq_false_negative_imm_zero_ext_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "cmove_w_eq_false_negative_imm_zero_ext_preserves_flags".into(),
        source,
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cmove_imm_csel_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let cmove_zero = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: VReg::Imm(0),
        cond: Condition::Eq,
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_x_eq_zero_imm_as_csel_preserves_flags: native lowering failed: {e}")
    });
    let zero_source = [enc_csel_form(1, 0, 0, 31, RD, 0), NOP, NOP];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x4000_0000;
    cases.push((
        "cmove_x_eq_true_zero_imm_as_csel_preserves_flags".into(),
        zero_source,
        cmove_zero,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0;
    cases.push((
        "cmove_x_eq_false_zero_imm_as_csel_preserves_flags".into(),
        zero_source,
        cmove_zero,
        st,
    ));

    let cmove_one = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: VReg::Imm(1),
        cond: Condition::Eq,
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_x_eq_one_imm_as_csinc_preserves_flags: native lowering failed: {e}")
    });
    let one_source = [enc_csel_form(1, 0, 1, RD, 31, 1), NOP, NOP];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x4000_0000;
    cases.push((
        "cmove_x_eq_true_one_imm_as_csinc_preserves_flags".into(),
        one_source,
        cmove_one,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0;
    cases.push((
        "cmove_x_eq_false_one_imm_as_csinc_preserves_flags".into(),
        one_source,
        cmove_one,
        st,
    ));

    let cmove_all_ones = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        cond: Condition::Ne,
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_w8_ne_all_ones_imm_as_csinv_preserves_flags: native lowering failed: {e}")
    });
    let all_ones_source = [
        enc_csel_form(0, 1, 0, RD, 31, 0),
        enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
        NOP,
    ];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0;
    cases.push((
        "cmove_w8_ne_true_all_ones_imm_as_csinv_preserves_flags".into(),
        all_ones_source,
        cmove_all_ones,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_0000_1111_2222;
    st.pstate = 0x4000_0000;
    cases.push((
        "cmove_w8_ne_false_all_ones_imm_as_csinv_preserves_flags".into(),
        all_ones_source,
        cmove_all_ones,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cmove_always_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let cmove_x = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: arm_x(1),
        cond: Condition::Always,
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_x_always_reg_as_mov_preserves_flags: native lowering failed: {e}")
    });

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0;
    cases.push((
        "cmove_x_always_reg_as_mov_preserves_flags".into(),
        [enc_mov_reg(1, RD, RN), NOP, NOP],
        cmove_x,
        st,
    ));

    let cmove_w8 = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: arm_x(1),
        cond: Condition::Always,
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_w8_always_reg_as_mov_uxtb_preserves_flags: native lowering failed: {e}")
    });
    let cmove_w8_source = [
        enc_mov_reg(0, RD, RN),
        enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
        NOP,
    ];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x2222_3333_4444_55f0;
    st.pstate = 0x9000_0000;
    cases.push((
        "cmove_w8_always_reg_as_mov_uxtb_preserves_flags".into(),
        cmove_w8_source,
        cmove_w8,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cmove_same_reg_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let cmove_x = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: arm_x(0),
        cond: Condition::Eq,
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_x_same_reg_as_noop_preserves_flags: native lowering failed: {e}")
    });

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0;
    cases.push((
        "cmove_x_same_reg_as_noop_preserves_flags".into(),
        [0xd65f_03c0, NOP, NOP],
        cmove_x,
        st,
    ));

    let cmove_w = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: arm_x(0),
        cond: Condition::Eq,
        width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_w_same_reg_as_self_mov_zero_ext_preserves_flags: native lowering failed: {e}")
    });

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0;
    cases.push((
        "cmove_w_same_reg_as_self_mov_zero_ext_preserves_flags".into(),
        [enc_mov_reg(0, RD, RD), NOP, NOP],
        cmove_w,
        st,
    ));

    let cmove_w8 = lower_aarch64_native_ops(vec![OpKind::CMove {
        dst: arm_x(0),
        src: arm_x(0),
        cond: Condition::Eq,
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| {
        panic!("cmove_w8_same_reg_as_uxtb_preserves_flags: native lowering failed: {e}")
    });

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ff80;
    st.pstate = 0x9000_0000;
    cases.push((
        "cmove_w8_same_reg_as_uxtb_preserves_flags".into(),
        [enc_bitfield_regs(0, 0b10, 0, 7, RD, RD), NOP, NOP],
        cmove_w8,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_select_identical_arm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let select_x = lower_aarch64_native_ops(vec![OpKind::Select {
        dst: arm_x(0),
        cond: arm_x(3),
        src_true: arm_x(1),
        src_false: arm_x(1),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("select_x_identical_arms_as_mov_preserves_flags: native lowering failed: {e}")
    });
    let select_x_source = [enc_mov_reg(1, RD, RN), NOP, NOP];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[3] = 1;
    st.pstate = 0x9000_0000;
    cases.push((
        "select_x_true_identical_arms_as_mov_preserves_flags".into(),
        select_x_source,
        select_x,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x2222_3333_4444_5555;
    st.pstate = 0x2000_0000;
    cases.push((
        "select_x_false_identical_arms_as_mov_preserves_flags".into(),
        select_x_source,
        select_x,
        st,
    ));

    let select_w8 = lower_aarch64_native_ops(vec![OpKind::Select {
        dst: arm_x(0),
        cond: arm_x(3),
        src_true: arm_x(1),
        src_false: arm_x(1),
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| {
        panic!("select_w8_identical_arms_as_mov_uxtb_preserves_flags: native lowering failed: {e}")
    });
    let select_w8_source = [
        enc_mov_reg(0, RD, RN),
        enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
        NOP,
    ];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x3333_4444_5555_56a5;
    st.x[3] = 7;
    st.pstate = 0x6000_0000;
    cases.push((
        "select_w8_true_identical_arms_as_mov_uxtb_preserves_flags".into(),
        select_w8_source,
        select_w8,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x4444_5555_6666_67c3;
    st.pstate = 0x5000_0000;
    cases.push((
        "select_w8_false_identical_arms_as_mov_uxtb_preserves_flags".into(),
        select_w8_source,
        select_w8,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_select_dst_arm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let true_arm_dst = lower_aarch64_native_ops(vec![OpKind::Select {
        dst: arm_x(0),
        cond: arm_x(3),
        src_true: arm_x(0),
        src_false: arm_x(1),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("select_x_true_arm_dst_as_cbnz_preserves_flags: native lowering failed: {e}")
    });
    let true_arm_dst_source = [enc_cbnz(3, 2), enc_mov_reg(1, RD, RN), NOP];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[3] = 1;
    st.pstate = 0x8000_0000;
    cases.push((
        "select_x_true_arm_dst_true_preserves_dst_and_flags".into(),
        true_arm_dst_source,
        true_arm_dst,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x2222_3333_4444_5555;
    st.pstate = 0x2000_0000;
    cases.push((
        "select_x_true_arm_dst_false_selects_false_and_flags".into(),
        true_arm_dst_source,
        true_arm_dst,
        st,
    ));

    let false_arm_dst = lower_aarch64_native_ops(vec![OpKind::Select {
        dst: arm_x(0),
        cond: arm_x(0),
        src_true: arm_x(1),
        src_false: arm_x(0),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| {
        panic!("select_x_false_arm_dst_as_cbz_preserves_flags: native lowering failed: {e}")
    });
    let false_arm_dst_source = [enc_cbz(RD, 2), enc_mov_reg(1, RD, RN), NOP];

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x3333_4444_5555_6666;
    st.pstate = 0x4000_0000;
    cases.push((
        "select_x_false_arm_dst_true_alias_cond_selects_true_and_flags".into(),
        false_arm_dst_source,
        false_arm_dst,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[1] = 0x4444_5555_6666_7777;
    st.pstate = 0xa000_0000;
    cases.push((
        "select_x_false_arm_dst_false_alias_cond_preserves_dst_and_flags".into(),
        false_arm_dst_source,
        false_arm_dst,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cond_select_true_transform_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xabcd_eeee_ffff_0000;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 0x10;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![
        OpKind::TestCondition {
            dst: VReg::virt(215),
            cond: Condition::Eq,
        },
        OpKind::Add {
            dst: VReg::virt(216),
            src1: arm_x(2),
            src2: SrcOperand::Imm(1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Select {
            dst: arm_x(0),
            cond: VReg::virt(215),
            src_true: VReg::virt(216),
            src_false: arm_x(1),
            width: OpWidth::W64,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("csinc_x_ne_opkind_true_transform_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "csinc_x_ne_opkind_true_transform_preserves_flags".into(),
        [enc_csel_form(1, 0, 1, RN, RM, 1), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 0xaaaa_bbbb_ffff_ffff;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![
        OpKind::TestCondition {
            dst: VReg::virt(216),
            cond: Condition::Eq,
        },
        OpKind::Add {
            dst: VReg::virt(217),
            src1: arm_x(2),
            src2: SrcOperand::Imm64(0x1_0000_0001),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Select {
            dst: arm_x(0),
            cond: VReg::virt(216),
            src_true: VReg::virt(217),
            src_false: arm_x(1),
            width: OpWidth::W32,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("csinc_w_masked_one_true_transform_zero_ext_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "csinc_w_masked_one_true_transform_zero_ext_preserves_flags".into(),
        [enc_csel_form(0, 0, 1, RN, RM, 1), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbcde_ffff_0000_1111;
    st.x[1] = 0x2222_3333_4444_5555;
    st.x[2] = 0xffff_ffff_1234_5678;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![
        OpKind::TestCondition {
            dst: VReg::virt(217),
            cond: Condition::Ne,
        },
        OpKind::Not {
            dst: VReg::virt(218),
            src: arm_x(2),
            width: OpWidth::W32,
        },
        OpKind::Select {
            dst: arm_x(0),
            cond: VReg::virt(217),
            src_true: VReg::virt(218),
            src_false: arm_x(1),
            width: OpWidth::W32,
        },
    ])
    .unwrap_or_else(|e| {
        panic!(
            "csinv_w_eq_opkind_true_transform_zero_ext_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "csinv_w_eq_opkind_true_transform_zero_ext_preserves_flags".into(),
        [enc_csel_form(0, 1, 0, RN, RM, 0), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xcdef_0000_1111_2222;
    st.x[1] = 0x3333_4444_5555_6666;
    st.x[2] = 5;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![
        OpKind::TestCondition {
            dst: VReg::virt(219),
            cond: Condition::Ugt,
        },
        OpKind::Neg {
            dst: VReg::virt(220),
            src: arm_x(2),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Select {
            dst: arm_x(0),
            cond: VReg::virt(219),
            src_true: VReg::virt(220),
            src_false: arm_x(1),
            width: OpWidth::W64,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("csneg_x_ls_opkind_true_transform_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "csneg_x_ls_opkind_true_transform_preserves_flags".into(),
        [enc_csel_form(1, 1, 1, RN, RM, 9), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xffff_0000_1111_2222;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![
        OpKind::TestCondition {
            dst: VReg::virt(221),
            cond: Condition::Eq,
        },
        OpKind::Not {
            dst: VReg::virt(222),
            src: VReg::Imm(0),
            width: OpWidth::W32,
        },
        OpKind::Select {
            dst: arm_x(0),
            cond: VReg::virt(221),
            src_true: VReg::virt(222),
            src_false: VReg::Imm(0),
            width: OpWidth::W32,
        },
    ])
    .unwrap_or_else(|e| {
        panic!(
            "csetm_w_eq_opkind_true_transform_zero_ext_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "csetm_w_eq_opkind_true_transform_zero_ext_preserves_flags".into(),
        [enc_csel_form(0, 1, 0, 31, 31, 1), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_flagm_masked_imm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1234_5678_9abc_def0;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Xor {
        dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
        src1: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
        src2: SrcOperand::Imm64(0x1_2000_0000),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("cfinv_masked_carry_opkind_clears_c: native lowering failed: {e}"));
    cases.push((
        "cfinv_masked_carry_opkind_clears_c".into(),
        [enc_cfinv(), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_sysreg_masked_mask_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let nzcv = VReg::Arch(ArchReg::Arm(ArmReg::Nzcv));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0xa000_0000;
    let masked = VReg::virt(240);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::And {
            dst: masked,
            src1: nzcv,
            src2: SrcOperand::Imm64(0x1_f000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Mov {
            dst: arm_x(0),
            src: SrcOperand::Reg(masked),
            width: OpWidth::W64,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("mrs_nzcv_lifted_masked_mask_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mrs_nzcv_lifted_masked_mask_preserves_flags".into(),
        [enc_mrs_nzcv(RD), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_1234_5678;
    st.pstate = 0xf000_0000;
    let masked = VReg::virt(241);
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::And {
            dst: masked,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_f000_0000),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Mov {
            dst: nzcv,
            src: SrcOperand::Reg(masked),
            width: OpWidth::W32,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("msr_nzcv_lifted_masked_mask_preserves_regs: native lowering failed: {e}")
    });
    cases.push((
        "msr_nzcv_lifted_masked_mask_preserves_regs".into(),
        [enc_msr_nzcv(RN), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_flagm_masked_shift_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let nzcv = VReg::Arch(ArchReg::Arm(ArmReg::Nzcv));

    let v_to_z = VReg::virt(240);
    let z_or_v = VReg::virt(241);
    let z_bit = VReg::virt(242);
    let v_to_c = VReg::virt(243);
    let c_raw = VReg::virt(244);
    let c_bit = VReg::virt(245);
    let ax_result = VReg::virt(246);
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x9000_0000;
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Shl {
            dst: v_to_z,
            src: nzcv,
            amount: SrcOperand::Imm64(66),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: z_or_v,
            src1: nzcv,
            src2: SrcOperand::Reg(v_to_z),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: z_bit,
            src1: z_or_v,
            src2: SrcOperand::Imm64(0x1_4000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Shl {
            dst: v_to_c,
            src: nzcv,
            amount: SrcOperand::Imm64(65),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: c_raw,
            src1: nzcv,
            src2: SrcOperand::Imm64(0x1_2000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::AndNot {
            dst: c_bit,
            src1: c_raw,
            src2: SrcOperand::Reg(v_to_c),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: ax_result,
            src1: z_bit,
            src2: SrcOperand::Reg(c_bit),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Mov {
            dst: nzcv,
            src: SrcOperand::Reg(ax_result),
            width: OpWidth::W32,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("axflag_lifted_masked_shift_counts_preserves_regs: native lowering failed: {e}")
    });
    cases.push((
        "axflag_lifted_masked_shift_counts_preserves_regs".into(),
        [enc_axflag(), NOP, NOP],
        lowered,
        st,
    ));

    let shl1 = VReg::virt(250);
    let shl2 = VReg::virt(251);
    let has_c_or_z_as_n = VReg::virt(252);
    let n_bit = VReg::virt(253);
    let z_raw = VReg::virt(254);
    let z_bit = VReg::virt(255);
    let shr1 = VReg::virt(256);
    let c_or_z = VReg::virt(257);
    let c_bit = VReg::virt(258);
    let shr2 = VReg::virt(259);
    let v_unmasked = VReg::virt(260);
    let v_bit = VReg::virt(261);
    let nz = VReg::virt(262);
    let cv = VReg::virt(263);
    let xa_result = VReg::virt(264);
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Shl {
            dst: shl1,
            src: nzcv,
            amount: SrcOperand::Imm64(65),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Shl {
            dst: shl2,
            src: nzcv,
            amount: SrcOperand::Imm64(66),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: has_c_or_z_as_n,
            src1: shl1,
            src2: SrcOperand::Reg(shl2),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::AndNot {
            dst: n_bit,
            src1: VReg::Imm(0x8000_0000),
            src2: SrcOperand::Reg(has_c_or_z_as_n),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: z_raw,
            src1: nzcv,
            src2: SrcOperand::Imm64(0x1_4000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: z_bit,
            src1: z_raw,
            src2: SrcOperand::Reg(shl1),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Shr {
            dst: shr1,
            src: nzcv,
            amount: SrcOperand::Imm64(65),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: c_or_z,
            src1: nzcv,
            src2: SrcOperand::Reg(shr1),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: c_bit,
            src1: c_or_z,
            src2: SrcOperand::Imm64(0x1_2000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Shr {
            dst: shr2,
            src: nzcv,
            amount: SrcOperand::Imm64(66),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::AndNot {
            dst: v_unmasked,
            src1: shr2,
            src2: SrcOperand::Reg(shr1),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: v_bit,
            src1: v_unmasked,
            src2: SrcOperand::Imm64(0x1_1000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: nz,
            src1: n_bit,
            src2: SrcOperand::Reg(z_bit),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: cv,
            src1: c_bit,
            src2: SrcOperand::Reg(v_bit),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: xa_result,
            src1: nz,
            src2: SrcOperand::Reg(cv),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Mov {
            dst: nzcv,
            src: SrcOperand::Reg(xa_result),
            width: OpWidth::W32,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("xaflag_lifted_masked_shift_counts_preserves_regs: native lowering failed: {e}")
    });
    cases.push((
        "xaflag_lifted_masked_shift_counts_preserves_regs".into(),
        [enc_xaflag(), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cls_masked_imm_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let sign_mask = VReg::virt(230);
    let normalized = VReg::virt(231);
    let leading = VReg::virt(232);
    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Sar {
            dst: sign_mask,
            src: arm_x(1),
            amount: SrcOperand::Imm64(95),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Xor {
            dst: normalized,
            src1: arm_x(1),
            src2: SrcOperand::Reg(sign_mask),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
        OpKind::Clz {
            dst: leading,
            src: normalized,
            width: OpWidth::W32,
        },
        OpKind::Sub {
            dst: arm_x(0),
            src1: leading,
            src2: SrcOperand::Imm64(0x1_0000_0001),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("cls_w_lifted_masked_imms_as_cls_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "cls_w_lifted_masked_imms_as_cls_preserves_flags".into(),
        [enc_dp1(0, 0b000101), NOP, NOP],
        lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn push_cond_compare_inverted_native_cases(
    cases: &mut Vec<(String, [u32; 3], [u32; 3], ArmState)>,
    control_target: i32,
) {
    let ccmp_source = [enc_condcmp(1, 1, false, RM, 1, 0b0100), NOP, NOP];
    let ccmp_lowered = lower_aarch64_native_ops(vec![
        OpKind::TestCondition {
            dst: VReg::virt(223),
            cond: Condition::Eq,
        },
        OpKind::Sub {
            dst: VReg::virt(224),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::All,
        },
        OpKind::Mov {
            dst: VReg::virt(225),
            src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Nzcv))),
            width: OpWidth::W32,
        },
        OpKind::Select {
            dst: VReg::virt(226),
            cond: VReg::virt(223),
            src_true: VReg::Imm(0x4000_0000),
            src_false: VReg::virt(225),
            width: OpWidth::W32,
        },
        OpKind::Mov {
            dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
            src: SrcOperand::Reg(VReg::virt(226)),
            width: OpWidth::W32,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("ccmp_x_ne_inverted_select_fallback_z: native lowering failed: {e}")
    });

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 5;
    st.x[2] = 7;
    st.pstate = 0x4000_0000;
    cases.push((
        "ccmp_x_ne_inverted_select_fallback_z".into(),
        ccmp_source,
        ccmp_lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 9;
    st.x[2] = 3;
    st.pstate = 0;
    cases.push((
        "ccmp_x_ne_inverted_select_compares".into(),
        ccmp_source,
        ccmp_lowered,
        st,
    ));

    let ccmn_source = [enc_condcmp(0, 0, true, 5, 9, 0b1001), NOP, NOP];
    let ccmn_lowered = lower_aarch64_native_ops(vec![
        OpKind::TestCondition {
            dst: VReg::virt(227),
            cond: Condition::Ugt,
        },
        OpKind::Add {
            dst: VReg::virt(228),
            src1: arm_x(1),
            src2: SrcOperand::Imm(5),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        },
        OpKind::Mov {
            dst: VReg::virt(229),
            src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Nzcv))),
            width: OpWidth::W32,
        },
        OpKind::Select {
            dst: VReg::virt(230),
            cond: VReg::virt(227),
            src_true: VReg::Imm(0x9000_0000),
            src_false: VReg::virt(229),
            width: OpWidth::W32,
        },
        OpKind::Mov {
            dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
            src: SrcOperand::Reg(VReg::virt(230)),
            width: OpWidth::W32,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("ccmn_w_ls_inverted_select_fallback_nv: native lowering failed: {e}")
    });

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff;
    st.pstate = 0x2000_0000;
    cases.push((
        "ccmn_w_ls_inverted_select_fallback_nv".into(),
        ccmn_source,
        ccmn_lowered,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.pc = PCREL_MAGIC;
    st.x[30] = pcrel_marker(control_target);
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0xffff_ffff;
    st.pstate = 0;
    cases.push((
        "ccmn_w_ls_inverted_select_compares".into(),
        ccmn_source,
        ccmn_lowered,
        st,
    ));
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_csel(sf: u32, cond: u32) -> u32 {
    enc_csel_form(sf, 0, 0, RN, RM, cond)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_csel_form(sf: u32, op: u32, op2: u32, rn: u32, rm: u32, cond: u32) -> u32 {
    (sf << 31)
        | (op << 30)
        | (0b11010100 << 21)
        | (rm << 16)
        | (cond << 12)
        | (op2 << 10)
        | (rn << 5)
        | RD
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_barrier(op2: u32) -> u32 {
    0xd500_0000 | (3 << 16) | (3 << 12) | (0xf << 8) | (op2 << 5) | 31
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_clrex() -> u32 {
    enc_barrier(0b010)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_hint(crm: u32, op2: u32) -> u32 {
    0xd500_0000 | (3 << 16) | (2 << 12) | (crm << 8) | (op2 << 5) | 31
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
const SYSREG_NZCV_RAW: u32 = (3 << 14) | (3 << 11) | (4 << 7) | (2 << 3);

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_mrs_nzcv(rt: u32) -> u32 {
    enc_mrs_sysreg(rt, 3, 4, 2, 0)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_mrs_fpcr(rt: u32) -> u32 {
    enc_mrs_sysreg(rt, 3, 4, 4, 0)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_mrs_fpsr(rt: u32) -> u32 {
    enc_mrs_sysreg(rt, 3, 4, 4, 1)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_mrs_sysreg(rt: u32, op1: u32, crn: u32, crm: u32, op2: u32) -> u32 {
    0xd500_0000
        | (1 << 21)
        | (3 << 19)
        | (op1 << 16)
        | (crn << 12)
        | (crm << 8)
        | (op2 << 5)
        | (rt & 0x1f)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_msr_nzcv(rt: u32) -> u32 {
    enc_msr_sysreg(rt, 3, 4, 2, 0)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_msr_fpcr(rt: u32) -> u32 {
    enc_msr_sysreg(rt, 3, 4, 4, 0)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_msr_fpsr(rt: u32) -> u32 {
    enc_msr_sysreg(rt, 3, 4, 4, 1)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_msr_sysreg(rt: u32, op1: u32, crn: u32, crm: u32, op2: u32) -> u32 {
    0xd500_0000 | (3 << 19) | (op1 << 16) | (crn << 12) | (crm << 8) | (op2 << 5) | (rt & 0x1f)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_prfm_lit(rt: u32, imm19: i32) -> u32 {
    (0b11 << 30) | (0b011 << 27) | (((imm19 as u32) & 0x7ffff) << 5) | (rt & 0x1f)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_cfinv() -> u32 {
    enc_flagm(0b000)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_axflag() -> u32 {
    enc_flagm(0b010)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_xaflag() -> u32 {
    enc_flagm(0b001)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_flagm(op2: u32) -> u32 {
    0xd500_401f | (op2 << 5)
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn bextr_flags() -> FlagUpdate {
    FlagUpdate::Specific(FlagSet::CF.union(FlagSet::ZF).union(FlagSet::OF))
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn bzhi_flags() -> FlagUpdate {
    FlagUpdate::Specific(
        FlagSet::CF
            .union(FlagSet::ZF)
            .union(FlagSet::SF)
            .union(FlagSet::OF),
    )
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn compare_smir_scalar_case(
    label: &str,
    insn: u32,
    got: &Aarch64GuestRegs,
    hw: &ArmState,
    mismatches: &mut Vec<Mismatch>,
) {
    let mut diffs = Vec::new();
    for r in 0..31 {
        if got.x[r] != hw.x[r] {
            diffs.push(format!(
                "x{r}: smir={:#018x} hw={:#018x}",
                got.x[r], hw.x[r]
            ));
        }
    }
    if got.sp != hw.sp {
        diffs.push(format!("sp: smir={:#018x} hw={:#018x}", got.sp, hw.sp));
    }
    // The oracle captures PC from its signal/trap harness, so it is not a
    // stable architectural post-instruction PC for these scalar comparisons.
    let got_nzcv = (got.nzcv >> 28) & 0xF;
    let hw_nzcv = (hw.pstate >> 28) & 0xF;
    if got_nzcv != hw_nzcv {
        diffs.push(format!("nzcv: smir={:#x} hw={:#x}", got_nzcv, hw_nzcv));
    }
    if got.fpcr != hw.fpcr {
        diffs.push(format!(
            "fpcr: smir={:#018x} hw={:#018x}",
            got.fpcr, hw.fpcr
        ));
    }
    if got.fpsr != hw.fpsr {
        diffs.push(format!(
            "fpsr: smir={:#018x} hw={:#018x}",
            got.fpsr, hw.fpsr
        ));
    }

    if !diffs.is_empty() {
        mismatches.push(Mismatch {
            label: label.into(),
            insn,
            detail: diffs.join("  |  "),
        });
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn compare_native_aarch64_case(
    label: &str,
    insn: u32,
    got: &ArmState,
    hw: &ArmState,
    mismatches: &mut Vec<Mismatch>,
) {
    let mut diffs = Vec::new();
    for r in 0..31 {
        if got.x[r] != hw.x[r] {
            diffs.push(format!(
                "x{r}: generated={:#018x} source={:#018x}",
                got.x[r], hw.x[r]
            ));
        }
    }
    if got.sp != hw.sp {
        diffs.push(format!(
            "sp: generated={:#018x} source={:#018x}",
            got.sp, hw.sp
        ));
    }
    if got.pc != hw.pc {
        diffs.push(format!(
            "pc: generated={:#018x} source={:#018x}",
            got.pc, hw.pc
        ));
    }
    let got_nzcv = (got.pstate >> 28) & 0xF;
    let hw_nzcv = (hw.pstate >> 28) & 0xF;
    if got_nzcv != hw_nzcv {
        diffs.push(format!(
            "nzcv: generated={:#x} source={:#x}",
            got_nzcv, hw_nzcv
        ));
    }
    if got.fpcr != hw.fpcr {
        diffs.push(format!(
            "fpcr: generated={:#018x} source={:#018x}",
            got.fpcr, hw.fpcr
        ));
    }
    if got.fpsr != hw.fpsr {
        diffs.push(format!(
            "fpsr: generated={:#018x} source={:#018x}",
            got.fpsr, hw.fpsr
        ));
    }
    for i in 0..32 {
        if got.scratch[i] != hw.scratch[i] {
            diffs.push(format!(
                "scratch[{i}]: generated={:#018x} source={:#018x}",
                got.scratch[i], hw.scratch[i]
            ));
        }
    }

    if !diffs.is_empty() {
        mismatches.push(Mismatch {
            label: label.into(),
            insn,
            detail: diffs.join("  |  "),
        });
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn compare_smir_memory_case(
    label: &str,
    insn: u32,
    got: &Aarch64GuestRegs,
    got_scratch: &[u64; 32],
    hw: &ArmState,
    mismatches: &mut Vec<Mismatch>,
) {
    let mut diffs = Vec::new();
    for r in 0..31 {
        if got.x[r] != hw.x[r] {
            diffs.push(format!(
                "x{r}: smir={:#018x} hw={:#018x}",
                got.x[r], hw.x[r]
            ));
        }
    }
    if got.sp != hw.sp {
        diffs.push(format!("sp: smir={:#018x} hw={:#018x}", got.sp, hw.sp));
    }
    let got_nzcv = (got.nzcv >> 28) & 0xF;
    let hw_nzcv = (hw.pstate >> 28) & 0xF;
    if got_nzcv != hw_nzcv {
        diffs.push(format!("nzcv: smir={:#x} hw={:#x}", got_nzcv, hw_nzcv));
    }
    for idx in 0..32 {
        if got_scratch[idx] != hw.scratch[idx] {
            diffs.push(format!(
                "scratch[{idx}]: smir={:#018x} hw={:#018x}",
                got_scratch[idx], hw.scratch[idx]
            ));
        }
    }

    if !diffs.is_empty() {
        mismatches.push(Mismatch {
            label: label.into(),
            insn,
            detail: diffs.join("  |  "),
        });
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
#[test]
fn smir_aarch64_x86_scalar_lowering_matches_qemu_oracle() {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_x86_scalar: qemu/cross-toolchain unavailable -> skipping"
            );
            return;
        }
    };

    let encodings: Vec<(&str, u32)> = vec![
        ("adds_x", enc_addsub_shift(1, 0, 1, 0, 0)),
        ("subs_x", enc_addsub_shift(1, 1, 1, 0, 0)),
        ("add_w_zero_ext", enc_addsub_shift(0, 0, 0, 0, 0)),
        ("sub_w_zero_ext", enc_addsub_shift(0, 1, 0, 0, 0)),
        ("add_x_lsl7", enc_addsub_shift(1, 0, 0, 0, 7)),
        ("subs_w_asr31_zero_ext", enc_addsub_shift(0, 1, 1, 2, 31)),
        ("add_x_uxtw", enc_addsub_ext(1, 0, 0, 0b010, 0)),
        ("add_x_sxtw_lsl2", enc_addsub_ext(1, 0, 0, 0b110, 2)),
        (
            "subs_w_uxtb_lsl1_zero_ext",
            enc_addsub_ext(0, 1, 1, 0b000, 1),
        ),
        ("adc_x", enc_addsub_carry(1, 0, 0)),
        ("adcs_x", enc_addsub_carry(1, 0, 1)),
        ("sbc_x", enc_addsub_carry(1, 1, 0)),
        ("sbcs_x", enc_addsub_carry(1, 1, 1)),
        ("ngc_x", enc_addsub_carry_rn(1, 1, 0, 31)),
        ("ngcs_x", enc_addsub_carry_rn(1, 1, 1, 31)),
        ("addg_x", enc_addsub_tags(0, 1, 0)),
        ("subg_x", enc_addsub_tags(1, 1, 0)),
        ("adc_w_zero_ext", enc_addsub_carry(0, 0, 0)),
        ("adcs_w_zero_ext", enc_addsub_carry(0, 0, 1)),
        ("sbc_w_zero_ext", enc_addsub_carry(0, 1, 0)),
        ("sbcs_w_zero_ext", enc_addsub_carry(0, 1, 1)),
        ("ngc_w_zero_ext", enc_addsub_carry_rn(0, 1, 0, 31)),
        ("ngcs_w_zero_ext", enc_addsub_carry_rn(0, 1, 1, 31)),
        ("ccmp_x_eq", enc_condcmp(1, 1, false, RM, 0, 0)),
        ("ccmn_x_ne", enc_condcmp(1, 0, false, RM, 1, 0b1001)),
        ("ccmp_w_imm_hi", enc_condcmp(0, 1, true, 5, 8, 0b0010)),
        ("and_x", enc_logical_shift(1, 0, 0, 0, 0)),
        ("ands_x", enc_logical_shift(1, 3, 0, 0, 0)),
        ("orr_x", enc_logical_shift(1, 1, 0, 0, 0)),
        ("eor_x", enc_logical_shift(1, 2, 0, 0, 0)),
        ("bic_x", enc_logical_shift(1, 0, 0, 1, 0)),
        ("orn_x", enc_logical_shift(1, 1, 0, 1, 0)),
        ("eon_x", enc_logical_shift(1, 2, 0, 1, 0)),
        ("and_w_asr31_zero_ext", enc_logical_shift(0, 0, 2, 0, 31)),
        ("bic_x_lsl4", enc_logical_shift(1, 0, 0, 1, 4)),
        ("orn_x_lsr8", enc_logical_shift(1, 1, 1, 1, 8)),
        ("eon_w_ror4_zero_ext", enc_logical_shift(0, 2, 3, 1, 4)),
        ("ands_x_lsr1", enc_logical_shift(1, 3, 1, 0, 1)),
        ("orr_w_zero_ext", enc_logical_shift(0, 1, 0, 0, 0)),
        ("movz_x_lsl16", enc_mov_wide(1, 0b10, 1, 0x1234)),
        ("movn_w", enc_mov_wide(0, 0b00, 0, 0)),
        ("movk_x_lsl16", enc_mov_wide(1, 0b11, 1, 0xabcd)),
        ("csel_x_eq", enc_csel(1, 0)),
        ("csel_x_ne", enc_csel(1, 1)),
        ("csel_x_hi", enc_csel(1, 8)),
        ("csel_x_ge", enc_csel(1, 10)),
        ("csel_w_lt_zero_ext", enc_csel(0, 11)),
        ("csel_w_le_zero_ext", enc_csel(0, 13)),
        ("csinc_x_eq", enc_csel_form(1, 0, 1, RN, RM, 0)),
        ("csinv_x_ne", enc_csel_form(1, 1, 0, RN, RM, 1)),
        ("csneg_x_hi", enc_csel_form(1, 1, 1, RN, RM, 8)),
        ("csinv_w_zero_ext", enc_csel_form(0, 1, 0, RN, RM, 1)),
        ("cset_x_raw_ne_alias", enc_csel_form(1, 0, 1, 31, 31, 1)),
        ("csetm_w_raw_ne_alias", enc_csel_form(0, 1, 0, 31, 31, 1)),
        ("cinc_x_raw_ne_alias", enc_csel_form(1, 0, 1, RN, RN, 1)),
        ("cinv_x_raw_ne_alias", enc_csel_form(1, 1, 0, RN, RN, 1)),
        ("cneg_w_raw_ne_alias", enc_csel_form(0, 1, 1, RN, RN, 1)),
        ("mul_x", enc_dp3_ra(1, 0b000, 0, 31)),
        ("mul_w_zero_ext", enc_dp3_ra(0, 0b000, 0, 31)),
        ("madd_x", enc_dp3(1, 0b000, 0)),
        ("madd_w_zero_ext", enc_dp3(0, 0b000, 0)),
        ("msub_x", enc_dp3(1, 0b000, 1)),
        ("msub_w_zero_ext", enc_dp3(0, 0b000, 1)),
        ("mneg_x", enc_dp3_ra(1, 0b000, 1, 31)),
        ("mneg_w_zero_ext", enc_dp3_ra(0, 0b000, 1, 31)),
        ("smaddl", enc_dp3(1, 0b001, 0)),
        ("smsubl", enc_dp3(1, 0b001, 1)),
        ("umaddl", enc_dp3(1, 0b101, 0)),
        ("umsubl", enc_dp3(1, 0b101, 1)),
        ("smull", enc_dp3_ra(1, 0b001, 0, 31)),
        ("umull", enc_dp3_ra(1, 0b101, 0, 31)),
        ("smulh", enc_dp3_ra(1, 0b010, 0, 31)),
        ("umulh", enc_dp3_ra(1, 0b110, 0, 31)),
        ("lsl_imm_x", enc_bitfield(1, 0b10, 51, 50)),
        ("lsl_imm_w_zero_ext", enc_bitfield(0, 0b10, 25, 24)),
        ("lsr_imm_x", enc_bitfield(1, 0b10, 9, 63)),
        ("asr_imm_x", enc_bitfield(1, 0b00, 9, 63)),
        ("sxtb_x", enc_bitfield(1, 0b00, 0, 7)),
        ("ubfx_x", enc_bitfield(1, 0b10, 8, 23)),
        ("sbfx_w_zero_ext", enc_bitfield(0, 0b00, 4, 11)),
        ("ubfiz_x", enc_bitfield(1, 0b10, 60, 7)),
        ("sbfiz_w_zero_ext", enc_bitfield(0, 0b00, 24, 7)),
        ("bfi_x", enc_bitfield(1, 0b01, 56, 7)),
        ("bfxil_x", enc_bitfield(1, 0b01, 8, 15)),
        ("bfc_x", enc_bitfield_rn(1, 0b01, 56, 7, 31)),
        ("extr_x", enc_extract(1, RN, RM, 13)),
        ("extr_w_zero_ext", enc_extract(0, RN, RM, 7)),
        ("ror_imm_x", enc_extract(1, RN, RN, 17)),
        ("udiv_x", enc_dp2(1, 0b0010)),
        ("udiv_w_zero_ext", enc_dp2(0, 0b0010)),
        ("sdiv_x", enc_dp2(1, 0b0011)),
        ("sdiv_w_zero_ext", enc_dp2(0, 0b0011)),
        ("crc32b", enc_dp2(0, 0b010000)),
        ("crc32h", enc_dp2(0, 0b010001)),
        ("crc32w", enc_dp2(0, 0b010010)),
        ("crc32x", enc_crc32_x(false)),
        ("crc32cb", enc_dp2(0, 0b010100)),
        ("crc32ch", enc_dp2(0, 0b010101)),
        ("crc32cw", enc_dp2(0, 0b010110)),
        ("crc32cx", enc_crc32_x(true)),
        ("lslv_x", enc_dp2(1, 0b1000)),
        ("lslv_w_zero_ext", enc_dp2(0, 0b1000)),
        ("lsrv_x", enc_dp2(1, 0b1001)),
        ("lsrv_w_zero_ext", enc_dp2(0, 0b1001)),
        ("asrv_x", enc_dp2(1, 0b1010)),
        ("asrv_w_zero_ext", enc_dp2(0, 0b1010)),
        ("rorv_x", enc_dp2(1, 0b1011)),
        ("rorv_w_zero_ext", enc_dp2(0, 0b1011)),
        ("rbit_x", enc_dp1(1, 0b000000)),
        ("rbit_w_zero_ext", enc_dp1(0, 0b000000)),
        ("rev16_x", enc_dp1(1, 0b000001)),
        ("rev16_w_zero_ext", enc_dp1(0, 0b000001)),
        ("rev32_x", enc_dp1(1, 0b000010)),
        ("rev_x", enc_dp1(1, 0b000011)),
        ("rev_w_zero_ext", enc_dp1(0, 0b000010)),
        ("clz_x", enc_dp1(1, 0b000100)),
        ("clz_w_zero_ext", enc_dp1(0, 0b000100)),
        ("cls_x", enc_dp1(1, 0b000101)),
        ("cls_w_zero_ext", enc_dp1(0, 0b000101)),
        ("dsb_sy", enc_barrier(0b100)),
        ("dmb_sy", enc_barrier(0b101)),
        ("isb", enc_barrier(0b110)),
        ("clrex", enc_clrex()),
        ("yield", enc_hint(0b0000, 0b001)),
        ("wfe", enc_hint(0b0000, 0b010)),
        ("wfi", enc_hint(0b0000, 0b011)),
        ("sev", enc_hint(0b0000, 0b100)),
        ("sevl", enc_hint(0b0000, 0b101)),
        ("hint_dgh", enc_hint(0b0000, 0b110)),
        ("csdb", enc_hint(0b0010, 0b100)),
        ("bti", enc_hint(0b0100, 0b000)),
        ("bti_c", enc_hint(0b0100, 0b010)),
        ("bti_j", enc_hint(0b0100, 0b100)),
        ("bti_jc", enc_hint(0b0100, 0b110)),
        ("prfm_lit_pldl1keep", enc_prfm_lit(0, 0)),
        ("prfm_lit_pstl3strm", enc_prfm_lit(0b10101, 3)),
        ("prfm_lit_raw31_neg", enc_prfm_lit(31, -1)),
        ("cfinv", enc_cfinv()),
        ("axflag", enc_axflag()),
        ("xaflag", enc_xaflag()),
        ("mrs_nzcv", enc_mrs_nzcv(RD)),
        ("msr_nzcv_x1", enc_msr_nzcv(RN)),
        ("msr_nzcv_xzr", enc_msr_nzcv(31)),
    ];

    let mut rng = Rng::new(0x5a11_64c0_de);
    let mut batch = Vec::new();
    for (label, insn) in encodings {
        for _ in 0..8 {
            batch.push((label.to_string(), insn, gen_input(&mut rng)));
        }
    }

    let mut st = ArmState::zeroed();
    st.x[1] = 1;
    st.x[2] = 1;
    batch.push((
        "add_x_lsl4_crafted".into(),
        enc_addsub_shift(1, 0, 0, 0, 4),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 1;
    st.x[2] = 0x8000_0000;
    batch.push((
        "subs_w_asr31_crafted".into(),
        enc_addsub_shift(0, 1, 1, 2, 31),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = 0x1000_0000_0000_0000;
    st.x[2] = 0xffff_ffff_0000_0100;
    batch.push((
        "add_x_uxtw_crafted".into(),
        enc_addsub_ext(1, 0, 0, 0b010, 0),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = 8;
    st.x[2] = 0xffff_ffff;
    batch.push((
        "add_x_sxtw_lsl2_crafted".into(),
        enc_addsub_ext(1, 0, 0, 0b110, 2),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x10;
    st.x[2] = 0xffff_ff0f;
    batch.push((
        "subs_w_uxtb_lsl1_crafted".into(),
        enc_addsub_ext(0, 1, 1, 0b000, 1),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 5;
    st.x[2] = 7;
    st.pstate = 0x6000_0000;
    batch.push((
        "cmp_x_preserves_x0_sets_borrow_flags".into(),
        enc_addsub_shift_regs(1, 1, 1, 0, 0, 31, RN, RM),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0xffff_ffff_0000_0001;
    st.x[2] = 0xffff_ffff_0000_0002;
    st.pstate = 0xf000_0000;
    batch.push((
        "cmp_w_ignores_high32_preserves_x0".into(),
        enc_addsub_shift_regs(0, 1, 1, 0, 0, 31, RN, RM),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = u64::MAX;
    st.x[2] = 1;
    st.pstate = 0;
    batch.push((
        "cmn_x_preserves_x0_sets_zero_carry".into(),
        enc_addsub_shift_regs(1, 0, 1, 0, 0, 31, RN, RM),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[2] = 0xffff_ffff_0000_0003;
    st.pstate = 0x9000_0000;
    batch.push((
        "neg_x_preserves_flags".into(),
        enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RM),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[2] = 0xffff_ffff_8000_0000;
    st.pstate = 0;
    batch.push((
        "negs_w_min_sets_overflow_zero_ext".into(),
        enc_addsub_shift_regs(0, 1, 1, 0, 0, RD, 31, RM),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = u64::MAX;
    st.x[2] = 1;
    batch.push((
        "bic_x_lsl4_crafted".into(),
        enc_logical_shift(1, 0, 0, 1, 4),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = 0x0000_00ff_0000_0000;
    st.x[2] = 0xffff_0000_0000_0000;
    batch.push((
        "orn_x_lsr8_crafted".into(),
        enc_logical_shift(1, 1, 1, 1, 8),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x5555_aaaa_0000_ffff;
    st.x[2] = 0x8000_0001;
    batch.push((
        "eon_w_ror4_zero_ext_crafted".into(),
        enc_logical_shift(0, 2, 3, 1, 4),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0x00ff_0000_0000_0000;
    st.x[2] = 0x0000_00ff_0000_0000;
    st.pstate = 0xf000_0000;
    batch.push((
        "tst_x_lsr8_preserves_x0_sets_logic_flags".into(),
        enc_logical_shift_regs(1, 0b11, 1, 0, 8, 31, RN, RM),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.x[2] = 0xffff_ffff_0000_0001;
    st.pstate = 0x3000_0000;
    batch.push((
        "tst_w_preserves_x0_ignores_high32".into(),
        enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RN, RM),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = u64::MAX;
    st.x[2] = 0;
    st.pstate = 0x2000_0000;
    batch.push(("adc_x_carry_in_set".into(), enc_addsub_carry(1, 0, 0), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff;
    st.x[2] = 0;
    st.pstate = 0x2000_0000;
    batch.push((
        "adc_w_carry_in_set_zero_ext".into(),
        enc_addsub_carry(0, 0, 0),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = 5;
    st.x[2] = 3;
    st.pstate = 0;
    batch.push((
        "sbc_x_carry_clear_borrow_in".into(),
        enc_addsub_carry(1, 1, 0),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 5;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    batch.push((
        "sbc_w_carry_set_no_borrow_zero_ext".into(),
        enc_addsub_carry(0, 1, 0),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = u64::MAX;
    st.x[2] = 0;
    st.pstate = 0x2000_0000;
    batch.push(("adcs_x_sets_carry".into(), enc_addsub_carry(1, 0, 1), st));

    let mut st = ArmState::zeroed();
    st.x[1] = 0;
    st.x[2] = 1;
    st.pstate = 0;
    batch.push(("sbcs_x_borrow_flags".into(), enc_addsub_carry(1, 1, 1), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    batch.push((
        "ngc_x_carry_set".into(),
        enc_addsub_carry_rn(1, 1, 0, 31),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[2] = 0;
    st.pstate = 0;
    batch.push((
        "ngc_x_carry_clear_borrow_in".into(),
        enc_addsub_carry_rn(1, 1, 0, 31),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[2] = 1;
    st.pstate = 0;
    batch.push((
        "ngcs_w_carry_clear_zero_ext_flags".into(),
        enc_addsub_carry_rn(0, 1, 1, 31),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x0500_0000_0000_1000;
    batch.push(("addg_clears_tag".into(), enc_addsub_tags(0, 2, 7), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x0a00_0000_0000_1040;
    batch.push(("subg_clears_tag".into(), enc_addsub_tags(1, 3, 4), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.sp = 0x0700_0000_0000_2000;
    batch.push((
        "addg_sp_source_clears_tag".into(),
        enc_addsub_tags_regs(0, 1, 9, 31, RD),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.sp = 0x0900_0000_0000_2000;
    batch.push((
        "subg_sp_dest_clears_tag".into(),
        enc_addsub_tags_regs(1, 1, 9, 31, 31),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = 5;
    st.x[2] = 5;
    st.pstate = 0x4000_0000;
    batch.push((
        "ccmp_x_eq_true".into(),
        enc_condcmp(1, 1, false, RM, 0, 0),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = 1;
    st.x[2] = 2;
    st.pstate = 0;
    batch.push((
        "ccmp_x_eq_false_fallback_z".into(),
        enc_condcmp(1, 1, false, RM, 0, 0b0100),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = u64::MAX;
    st.x[2] = 1;
    st.pstate = 0;
    batch.push((
        "ccmn_x_ne_true".into(),
        enc_condcmp(1, 0, false, RM, 1, 0),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = 0x100;
    st.x[2] = 0x20;
    st.pstate = 0x4000_0000;
    batch.push((
        "ccmn_w_ne_false_fallback_nv".into(),
        enc_condcmp(0, 0, false, RM, 1, 0b1001),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[1] = 7;
    st.pstate = 0x2000_0000;
    batch.push((
        "ccmp_w_imm_hi_true".into(),
        enc_condcmp(0, 1, true, 5, 8, 0),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 0xffff_ffff_1234_5678;
    st.pstate = 0x4000_0000;
    batch.push((
        "csinv_w_raw_ne_false_zero_ext".into(),
        enc_csel_form(0, 1, 0, RN, RM, 1),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 5;
    st.pstate = 0;
    batch.push((
        "csneg_x_raw_hi_false".into(),
        enc_csel_form(1, 1, 1, RN, RM, 8),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0;
    batch.push((
        "cset_x_raw_ne_true_alias".into(),
        enc_csel_form(1, 0, 1, 31, 31, 1),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x4000_0000;
    batch.push((
        "cset_x_raw_ne_false_alias".into(),
        enc_csel_form(1, 0, 1, 31, 31, 1),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x4000_0000;
    batch.push((
        "csetm_w_raw_ne_false_alias".into(),
        enc_csel_form(0, 1, 0, 31, 31, 1),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = u64::MAX;
    st.pstate = 0x4000_0000;
    batch.push((
        "cinc_x_raw_ne_false_alias".into(),
        enc_csel_form(1, 0, 1, RN, RN, 1),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1122_3344_5566_7788;
    st.pstate = 0x4000_0000;
    batch.push((
        "cinv_x_raw_ne_false_alias".into(),
        enc_csel_form(1, 1, 0, RN, RN, 1),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0x4000_0000;
    batch.push((
        "cneg_w_raw_ne_false_alias".into(),
        enc_csel_form(0, 1, 1, RN, RN, 1),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0;
    batch.push(("clz_x_zero".into(), enc_dp1(1, 0b000100), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_0000_0000;
    batch.push(("clz_w_zero_ext".into(), enc_dp1(0, 0b000100), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0;
    batch.push(("cls_x_zero".into(), enc_dp1(1, 0b000101), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = u64::MAX;
    batch.push(("cls_x_all_ones".into(), enc_dp1(1, 0b000101), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_8000_0000;
    batch.push(("cls_w_sign_edge_zero_ext".into(), enc_dp1(0, 0b000101), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 1;
    batch.push(("rbit_x_low_bit".into(), enc_dp1(1, 0b000000), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_1122_3344;
    batch.push(("rev_w_zero_ext_crafted".into(), enc_dp1(0, 0b000010), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x0011_2233_4455_6677;
    batch.push(("rev16_x_crafted".into(), enc_dp1(1, 0b000001), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_1122_3344;
    batch.push(("rev16_w_zero_ext_crafted".into(), enc_dp1(0, 0b000001), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1122_3344_aabb_ccdd;
    batch.push(("rev32_x_crafted".into(), enc_dp1(1, 0b000010), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 3;
    st.x[2] = 5;
    st.x[3] = 2;
    batch.push(("msub_w_zero_ext_crafted".into(), enc_dp3(0, 0b000, 1), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff;
    st.x[2] = 2;
    st.x[3] = 5;
    batch.push(("smaddl_negative".into(), enc_dp3(1, 0b001, 0), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff;
    st.x[2] = 0xffff_ffff;
    st.x[3] = 1;
    batch.push(("umaddl_high".into(), enc_dp3(1, 0b101, 0), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = i64::MIN as u64;
    st.x[2] = 2;
    batch.push(("smulh_negative".into(), enc_dp3_ra(1, 0b010, 0, 31), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = u64::MAX;
    st.x[2] = 2;
    batch.push(("umulh_high".into(), enc_dp3_ra(1, 0b110, 0, 31), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_8000_0001;
    batch.push((
        "lsl_w_zero_ext_crafted".into(),
        enc_bitfield(0, 0b10, 25, 24),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x8000_0000_0000_0000;
    batch.push((
        "asr_x_sign_crafted".into(),
        enc_bitfield(1, 0b00, 9, 63),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_def0;
    batch.push(("ubfx_x_crafted".into(), enc_bitfield(1, 0b10, 8, 23), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_f8f0;
    batch.push((
        "sbfx_w_sign_crafted".into(),
        enc_bitfield(0, 0b00, 4, 11),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_de8f;
    batch.push(("ubfiz_x_crafted".into(), enc_bitfield(1, 0b10, 60, 7), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_ff80;
    batch.push((
        "sbfiz_w_sign_crafted".into(),
        enc_bitfield(0, 0b00, 24, 7),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_de5a;
    batch.push(("bfi_x_crafted".into(), enc_bitfield(1, 0b01, 56, 7), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_5a00;
    batch.push(("bfxil_x_crafted".into(), enc_bitfield(1, 0b01, 8, 15), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    batch.push((
        "bfc_x_crafted".into(),
        enc_bitfield_rn(1, 0b01, 56, 7, 31),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1122_3344_5566_7788;
    st.x[2] = 0x99aa_bbcc_ddee_ff00;
    batch.push(("extr_x_crafted".into(), enc_extract(1, RN, RM, 13), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_1122_3344;
    st.x[2] = 0xffff_ffff_aabb_ccdd;
    batch.push((
        "extr_w_zero_ext_crafted".into(),
        enc_extract(0, RN, RM, 7),
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.x[2] = 0;
    batch.push(("udiv_x_zero".into(), enc_dp2(1, 0b0010), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_8765_4321;
    st.x[2] = 0;
    batch.push(("udiv_w_zero_ext".into(), enc_dp2(0, 0b0010), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = u64::MAX;
    batch.push(("sdiv_x_min_overflow".into(), enc_dp2(1, 0b0011), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.x[2] = u64::MAX;
    batch.push(("sdiv_w_min_overflow".into(), enc_dp2(0, 0b0011), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_89ab_cdef;
    st.x[2] = 0xffff_ffff_1234_5678;
    batch.push(("crc32w_zero_ext_crafted".into(), enc_dp2(0, 0b010010), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_89ab_cdef;
    st.x[2] = 0x0123_4567_89ab_cdef;
    batch.push(("crc32x_uses_xm_crafted".into(), enc_crc32_x(false), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_89ab_cdef;
    st.x[2] = 0x0123_4567_89ab_cdef;
    batch.push(("crc32cx_uses_xm_crafted".into(), enc_crc32_x(true), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 1;
    st.x[2] = 36;
    batch.push(("lslv_w_masked_count_crafted".into(), enc_dp2(0, 0b1000), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 130;
    batch.push(("lsrv_x_masked_count_crafted".into(), enc_dp2(1, 0b1001), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.x[2] = 31;
    batch.push(("asrv_w_sign_crafted".into(), enc_dp2(0, 0b1010), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.x[2] = 68;
    batch.push(("rorv_x_masked_count_crafted".into(), enc_dp2(1, 0b1011), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x0000_0000;
    batch.push(("cfinv_carry_set_crafted".into(), enc_cfinv(), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x2000_0000;
    batch.push(("cfinv_carry_clear_crafted".into(), enc_cfinv(), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x9000_0000;
    batch.push(("mrs_nzcv_reads_flags".into(), enc_mrs_nzcv(RD), st));

    let mut st = ArmState::zeroed();
    st.x[1] = 0xffff_ffff_1234_5678;
    st.pstate = 0xf000_0000;
    batch.push(("msr_nzcv_masks_x1".into(), enc_msr_nzcv(RN), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0xf000_0000;
    batch.push(("msr_nzcv_xzr_clears".into(), enc_msr_nzcv(31), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.fpcr = 0x00c0_0000;
    batch.push(("mrs_fpcr_reads_control".into(), enc_mrs_fpcr(RD), st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.fpsr = 0x0800_009f;
    batch.push(("mrs_fpsr_reads_status".into(), enc_mrs_fpsr(RD), st));

    let mut st = ArmState::zeroed();
    st.x[1] = 0xffff_ffff_00c0_0000;
    st.fpcr = 0;
    batch.push(("msr_fpcr_masks_x1".into(), enc_msr_fpcr(RN), st));

    let mut st = ArmState::zeroed();
    st.x[1] = 0xffff_ffff_0800_009f;
    st.fpsr = 0;
    batch.push(("msr_fpsr_masks_x1".into(), enc_msr_fpsr(RN), st));

    let mut st = ArmState::zeroed();
    st.fpcr = 0x00c0_0000;
    batch.push(("msr_fpcr_xzr_clears".into(), enc_msr_fpcr(31), st));

    let mut st = ArmState::zeroed();
    st.fpsr = 0x0800_009f;
    batch.push(("msr_fpsr_xzr_clears".into(), enc_msr_fpsr(31), st));

    for nzcv in 0..16 {
        let mut st = ArmState::zeroed();
        st.x[0] = 0xaaaa_bbbb_cccc_dddd;
        st.pstate = (nzcv as u64) << 28;
        batch.push((format!("axflag_nzcv_{nzcv:x}"), enc_axflag(), st));

        let mut st = ArmState::zeroed();
        st.x[0] = 0xaaaa_bbbb_cccc_dddd;
        st.pstate = (nzcv as u64) << 28;
        batch.push((format!("xaflag_nzcv_{nzcv:x}"), enc_xaflag(), st));
    }

    let cases: Vec<(u32, u32, ArmState)> = batch
        .iter()
        .map(|(_, insn, st)| (*insn, NOP, *st))
        .collect();
    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!("[arm_diff] smir_aarch64_x86_scalar: oracle run failed -> skipping");
            return;
        }
    };
    assert_eq!(outs.len(), cases.len());

    let mut mismatches = Vec::new();
    for (i, (label, insn, st)) in batch.iter().enumerate() {
        let out = &outs[i];
        if out.trapped != 0 {
            mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail: format!("hardware faulted with signal {}", out.trapped),
            });
            continue;
        }

        match run_smir_aarch64_x86_one(*insn, st) {
            Ok(got) => compare_smir_scalar_case(label, *insn, &got, &out.st, &mut mismatches),
            Err(detail) => mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail,
            }),
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n==== smir_aarch64_x86_scalar: {} mismatches across {} cases ====",
            mismatches.len(),
            batch.len()
        );
        for m in mismatches.iter().take(25) {
            eprintln!("  [{}] {:#010x}: {}", m.label, m.insn, m.detail);
        }
        panic!(
            "smir_aarch64_x86_scalar: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
#[test]
fn smir_aarch64_x86_nzcv_sysreg_roundtrip_matches_qemu_oracle() {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_x86_nzcv_sysreg_roundtrip: qemu/cross-toolchain unavailable -> skipping"
            );
            return;
        }
    };

    let mut batch: Vec<(String, u32, u32, ArmState)> = Vec::new();
    for &(label, value, pstate) in &[
        ("zero", 0, 0xf000_0000),
        ("n_only", 0x8000_0000, 0),
        ("v_with_low_bits", 0xffff_ffff_1234_5678, 0xe000_0000),
        ("all_flags", 0xffff_ffff_f000_0000, 0),
        ("lower_ones_no_flags", 0x0000_0000_0fff_ffff, 0xa000_0000),
    ] {
        let mut st = ArmState::zeroed();
        st.x[0] = 0xaaaa_bbbb_cccc_dddd;
        st.x[1] = value;
        st.pstate = pstate;
        batch.push((
            format!("msr_mrs_nzcv_{label}"),
            enc_msr_nzcv(RN),
            enc_mrs_nzcv(RD),
            st,
        ));
    }

    let cases: Vec<(u32, u32, ArmState)> = batch
        .iter()
        .map(|(_, insn, insn2, st)| (*insn, *insn2, *st))
        .collect();
    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_x86_nzcv_sysreg_roundtrip: oracle run failed -> skipping"
            );
            return;
        }
    };
    assert_eq!(outs.len(), cases.len());

    let mut mismatches = Vec::new();
    for (i, (label, insn, insn2, st)) in batch.iter().enumerate() {
        let out = &outs[i];
        if out.trapped != 0 {
            mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail: format!("hardware faulted with signal {}", out.trapped),
            });
            continue;
        }

        match run_smir_aarch64_x86_pair(*insn, *insn2, st) {
            Ok(got) => compare_smir_scalar_case(label, *insn, &got, &out.st, &mut mismatches),
            Err(detail) => mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail,
            }),
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n==== smir_aarch64_x86_nzcv_sysreg_roundtrip: {} mismatches across {} cases ====",
            mismatches.len(),
            batch.len()
        );
        for m in mismatches.iter().take(25) {
            eprintln!("  [{}] {:#010x}: {}", m.label, m.insn, m.detail);
        }
        panic!(
            "smir_aarch64_x86_nzcv_sysreg_roundtrip: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
#[test]
fn smir_aarch64_x86_fp_sysreg_roundtrip_matches_qemu_oracle() {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_x86_fp_sysreg_roundtrip: qemu/cross-toolchain unavailable -> skipping"
            );
            return;
        }
    };

    let mut batch: Vec<(String, u32, u32, ArmState)> = Vec::new();
    for &(label, value, initial_fpcr) in &[
        ("fpcr_zero", 0, 0x00c0_0000),
        ("fpcr_rmode_high_bits", 0xffff_ffff_00c0_0000, 0),
        ("fpcr_dn_fz_high_bits", 0xffff_ffff_0300_0000, 0x0040_0000),
    ] {
        let mut st = ArmState::zeroed();
        st.x[0] = 0xaaaa_bbbb_cccc_dddd;
        st.x[1] = value;
        st.fpcr = initial_fpcr;
        batch.push((
            format!("msr_mrs_{label}"),
            enc_msr_fpcr(RN),
            enc_mrs_fpcr(RD),
            st,
        ));
    }
    for &(label, value, initial_fpsr) in &[
        ("fpsr_zero", 0, 0x0800_009f),
        ("fpsr_status_high_bits", 0xffff_ffff_0800_009f, 0),
        ("fpsr_low_exc_high_bits", 0xffff_ffff_0000_009f, 0x0800_0000),
    ] {
        let mut st = ArmState::zeroed();
        st.x[0] = 0xaaaa_bbbb_cccc_dddd;
        st.x[1] = value;
        st.fpsr = initial_fpsr;
        batch.push((
            format!("msr_mrs_{label}"),
            enc_msr_fpsr(RN),
            enc_mrs_fpsr(RD),
            st,
        ));
    }

    let cases: Vec<(u32, u32, ArmState)> = batch
        .iter()
        .map(|(_, insn, insn2, st)| (*insn, *insn2, *st))
        .collect();
    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_x86_fp_sysreg_roundtrip: oracle run failed -> skipping"
            );
            return;
        }
    };
    assert_eq!(outs.len(), cases.len());

    let mut mismatches = Vec::new();
    for (i, (label, insn, insn2, st)) in batch.iter().enumerate() {
        let out = &outs[i];
        if out.trapped != 0 {
            mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail: format!("hardware faulted with signal {}", out.trapped),
            });
            continue;
        }

        match run_smir_aarch64_x86_pair(*insn, *insn2, st) {
            Ok(got) => compare_smir_scalar_case(label, *insn, &got, &out.st, &mut mismatches),
            Err(detail) => mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail,
            }),
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n==== smir_aarch64_x86_fp_sysreg_roundtrip: {} mismatches across {} cases ====",
            mismatches.len(),
            batch.len()
        );
        for m in mismatches.iter().take(25) {
            eprintln!("  [{}] {:#010x}: {}", m.label, m.insn, m.detail);
        }
        panic!(
            "smir_aarch64_x86_fp_sysreg_roundtrip: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
#[test]
fn smir_aarch64_x86_test_branch_lowering_matches_qemu_oracle() {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_x86_test_branch: qemu/cross-toolchain unavailable -> skipping"
            );
            return;
        }
    };

    let marker = enc_mov_wide(1, 0b10, 0, 0xcafe);
    let mut batch: Vec<(String, u32, u32, ArmState)> = Vec::new();

    let mut st = ArmState::zeroed();
    st.x[0] = 0x1010_2020_3030_4040;
    st.pstate = 0x4000_0000;
    batch.push(("b_eq_taken".into(), enc_cond_branch(0, 8), marker, st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x1010_2020_3030_4040;
    st.pstate = 0;
    batch.push(("b_eq_not_taken".into(), enc_cond_branch(0, 8), marker, st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x2020_3030_4040_5050;
    st.pstate = 0;
    batch.push(("b_ne_taken".into(), enc_cond_branch(1, 8), marker, st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x3030_4040_5050_6060;
    st.pstate = 0x8000_0000;
    batch.push(("b_lt_taken".into(), enc_cond_branch(11, 8), marker, st));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0;
    st.pstate = 0x9000_0000;
    batch.push((
        "cbz_x_taken_preserves_flags".into(),
        enc_compare_branch(1, 0, RN, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0x1_0000_0000;
    st.pstate = 0x2000_0000;
    batch.push((
        "cbz_x_not_taken_preserves_flags".into(),
        enc_compare_branch(1, 0, RN, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0xffff_ffff_0000_0001;
    st.pstate = 0x4000_0000;
    batch.push((
        "cbnz_w_taken_low32_preserves_flags".into(),
        enc_compare_branch(0, 1, RN, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0xffff_ffff_0000_0000;
    st.pstate = 0x8000_0000;
    batch.push((
        "cbnz_w_not_taken_ignores_high32".into(),
        enc_compare_branch(0, 1, RN, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0;
    st.pstate = 0x9000_0000;
    batch.push((
        "tbz_bit0_taken".into(),
        enc_test_branch(0, 0, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 1;
    st.pstate = 0x6000_0000;
    batch.push((
        "tbz_bit0_not_taken".into(),
        enc_test_branch(0, 0, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 1 << 5;
    st.pstate = 0x2000_0000;
    batch.push((
        "tbnz_bit5_taken".into(),
        enc_test_branch(1, 5, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0;
    st.pstate = 0x4000_0000;
    batch.push((
        "tbnz_bit5_not_taken".into(),
        enc_test_branch(1, 5, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0x7fff_ffff_ffff_ffff;
    st.pstate = 0x8000_0000;
    batch.push((
        "tbz_bit63_taken".into(),
        enc_test_branch(0, 63, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0x8000_0000_0000_0000;
    st.pstate = 0x1000_0000;
    batch.push((
        "tbz_bit63_not_taken".into(),
        enc_test_branch(0, 63, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0x8000_0000;
    st.pstate = 0x3000_0000;
    batch.push((
        "tbnz_w_bit31_taken".into(),
        enc_test_branch(1, 31, 8),
        marker,
        st,
    ));

    let mut st = ArmState::zeroed();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0xffff_ffff_7fff_ffff;
    st.pstate = 0x5000_0000;
    batch.push((
        "tbnz_w_bit31_not_taken".into(),
        enc_test_branch(1, 31, 8),
        marker,
        st,
    ));

    let cases: Vec<(u32, u32, ArmState)> = batch
        .iter()
        .map(|(_, insn, insn2, st)| (*insn, *insn2, *st))
        .collect();
    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!("[arm_diff] smir_aarch64_x86_test_branch: oracle run failed -> skipping");
            return;
        }
    };
    assert_eq!(outs.len(), cases.len());

    let mut mismatches = Vec::new();
    for (i, (label, insn, insn2, st)) in batch.iter().enumerate() {
        let out = &outs[i];
        if out.trapped != 0 {
            mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail: format!("hardware faulted with signal {}", out.trapped),
            });
            continue;
        }

        match run_smir_aarch64_x86_branch_pair(*insn, *insn2, st) {
            Ok(got) => compare_smir_scalar_case(label, *insn, &got, &out.st, &mut mismatches),
            Err(detail) => mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail,
            }),
        }
    }

    let control_target = 12;
    let mut control_batch: Vec<(String, u32, ArmState, ArmState)> = Vec::new();

    let mut smir_st = ArmState::zeroed();
    smir_st.x[0] = 0x1111_2222_3333_4444;
    smir_st.x[30] = 0xaaaa_bbbb_cccc_dddd;
    smir_st.pstate = 0xa000_0000;
    let mut hw_st = smir_st;
    hw_st.pc = PCREL_MAGIC;
    control_batch.push((
        "bl_direct_sets_lr_and_pc".into(),
        enc_bl(control_target),
        hw_st,
        smir_st,
    ));

    let mut smir_st = ArmState::zeroed();
    smir_st.x[1] = control_target as u64;
    smir_st.x[30] = 0x1234_5678_9abc_def0;
    smir_st.pstate = 0x5000_0000;
    let mut hw_st = smir_st;
    hw_st.pc = PCREL_MAGIC;
    hw_st.x[1] = pcrel_marker(control_target);
    control_batch.push(("br_x1_sets_pc".into(), enc_br(RN), hw_st, smir_st));

    let mut smir_st = ArmState::zeroed();
    smir_st.x[1] = control_target as u64;
    smir_st.x[30] = 0xfedc_ba98_7654_3210;
    smir_st.pstate = 0x6000_0000;
    let mut hw_st = smir_st;
    hw_st.pc = PCREL_MAGIC;
    hw_st.x[1] = pcrel_marker(control_target);
    control_batch.push(("blr_x1_sets_lr_and_pc".into(), enc_blr(RN), hw_st, smir_st));

    let mut smir_st = ArmState::zeroed();
    smir_st.x[0] = 0x2222_3333_4444_5555;
    smir_st.x[30] = control_target as u64;
    smir_st.pstate = 0x9000_0000;
    let mut hw_st = smir_st;
    hw_st.pc = PCREL_MAGIC;
    hw_st.x[30] = pcrel_marker(control_target);
    control_batch.push(("ret_x30_sets_pc".into(), enc_ret(30), hw_st, smir_st));

    let mut smir_st = ArmState::zeroed();
    smir_st.x[2] = control_target as u64;
    smir_st.x[30] = 0x3333_4444_5555_6666;
    smir_st.pstate = 0xc000_0000;
    let mut hw_st = smir_st;
    hw_st.pc = PCREL_MAGIC;
    hw_st.x[2] = pcrel_marker(control_target);
    control_batch.push(("ret_x2_sets_pc".into(), enc_ret(RM), hw_st, smir_st));

    let control_cases: Vec<(u32, u32, ArmState)> = control_batch
        .iter()
        .map(|(_, insn, hw_st, _)| (*insn, NOP, *hw_st))
        .collect();
    let control_outs = match run_oracle(&oracle, &control_cases) {
        Some(o) => o,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_x86_test_branch: control-flow oracle run failed -> skipping"
            );
            return;
        }
    };
    assert_eq!(control_outs.len(), control_cases.len());

    for (i, (label, insn, _hw_st, smir_st)) in control_batch.iter().enumerate() {
        let out = &control_outs[i];
        if out.trapped != 0 {
            mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail: format!("hardware faulted with signal {}", out.trapped),
            });
            continue;
        }

        match run_smir_aarch64_x86_control_one(*insn, smir_st) {
            Ok(got) => {
                compare_smir_scalar_case(label, *insn, &got, &out.st, &mut mismatches);
                if got.pc != out.st.pc {
                    mismatches.push(Mismatch {
                        label: label.clone(),
                        insn: *insn,
                        detail: format!("pc: smir={:#018x} hw={:#018x}", got.pc, out.st.pc),
                    });
                }
            }
            Err(detail) => mismatches.push(Mismatch {
                label: label.clone(),
                insn: *insn,
                detail,
            }),
        }
    }

    if !mismatches.is_empty() {
        let total_cases = batch.len() + control_batch.len();
        eprintln!(
            "\n==== smir_aarch64_x86_test_branch: {} mismatches across {} cases ====",
            mismatches.len(),
            total_cases
        );
        for m in mismatches.iter().take(25) {
            eprintln!("  [{}] {:#010x}: {}", m.label, m.insn, m.detail);
        }
        panic!(
            "smir_aarch64_x86_test_branch: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
#[test]
fn smir_aarch64_native_lowering_matches_qemu_oracle() {
    let handle = std::thread::Builder::new()
        .name("smir_aarch64_native_lowering".into())
        .stack_size(16 * 1024 * 1024)
        .spawn(smir_aarch64_native_lowering_matches_qemu_oracle_inner)
        .expect("failed to spawn native lowering oracle test thread");
    if let Err(payload) = handle.join() {
        std::panic::resume_unwind(payload);
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn smir_aarch64_native_lowering_matches_qemu_oracle_inner() {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_native_lowering: qemu/cross-toolchain unavailable -> skipping"
            );
            return;
        }
    };

    let control_target = 12;
    let native_state = || {
        let mut st = ArmState::zeroed();
        st.pc = PCREL_MAGIC;
        st.x[30] = pcrel_marker(control_target);
        st
    };
    let native_direct_addr = Address::Direct(arm_x(1));
    let native_base_offset_addr = Address::BaseOffset {
        base: arm_x(1),
        offset: 8,
        disp_size: DispSize::Auto,
    };
    let native_pair_x_addr = Address::BaseOffset {
        base: arm_x(1),
        offset: 16,
        disp_size: DispSize::Auto,
    };

    let mut cases: Vec<(String, [u32; 3], [u32; 3], ArmState)> = Vec::new();
    let mut push_case = |label: &str, source: u32, ops: Vec<OpKind>, st: ArmState| {
        let lowered = lower_aarch64_native_ops(ops)
            .unwrap_or_else(|e| panic!("{label}: native lowering failed: {e}"));
        cases.push((label.into(), [source, NOP, NOP], lowered, st));
    };

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 0x0101_0202_0303_0404;
    st.pstate = 0xa000_0000;
    push_case(
        "add_x_reg_preserves_flags",
        enc_addsub_shift_regs(1, 0, 0, 0, 0, RD, RN, RM),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0x5000_0000;
    push_case(
        "add_x_zero_source_reg_as_mov_preserves_flags",
        enc_addsub_shift_regs(1, 0, 0, 0, 0, RD, RN, 31),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(VReg::Imm(0)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.x[1] = 0x2222_3333_4444_5555;
    st.pstate = 0x9000_0000;
    push_case(
        "add_x_zero_imm_as_mov_preserves_flags",
        enc_addsub_imm_regs(1, 0, 0, 0, 0, RD, RN),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x7000_0000;
    push_case(
        "add_x_zero_base_imm_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x1234),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm(0x1234),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x6000_0000;
    push_case(
        "add_w_zero_base_neg_imm_as_movn_preserves_flags",
        enc_mov_wide(0, 0b00, 0, 0x33),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm(-0x34),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x4000_0000;
    push_case(
        "add_x_zero_base_neg_imm_as_movn_preserves_flags",
        enc_mov_wide(1, 0b00, 0, 0x33),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm(-0x34),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x2000_0000;
    push_case(
        "adds_x_zero_base_zero_imm_as_adds_zero_regs_sets_flags",
        enc_addsub_shift_regs(1, 0, 1, 0, 0, RD, 31, 31),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff;
    st.x[2] = 1;
    push_case(
        "adds_w_reg_sets_flags_zero_ext",
        enc_addsub_shift_regs(0, 0, 1, 0, 0, RD, RN, RM),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0;
    st.pstate = 0x6000_0000;
    push_case(
        "adds_w_imm_masked_neg_one_as_subs_one_sets_flags",
        enc_addsub_imm_regs(0, 1, 1, 0, 1, RD, RN),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0xffff_ffff),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x0102_0304_0506_0708;
    st.x[2] = 0x11;
    st.pstate = 0xd000_0000;
    push_case(
        "add_x_lsl7_opkind_preserves_flags",
        enc_addsub_shift(1, 0, 0, 0, 7),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Lsl,
                amount: 7,
            },
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_0000_0010;
    st.x[2] = 0xffff_ffff_8000_0000;
    st.pstate = 0x1000_0000;
    push_case(
        "subs_w_asr31_opkind_sets_flags_zero_ext",
        enc_addsub_shift(0, 1, 1, 2, 31),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Asr,
                amount: 31,
            },
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0x1000_0000_0000_0000;
    st.x[2] = 0xffff_ffff_0000_0100;
    st.pstate = 0x8000_0000;
    push_case(
        "add_x_uxtw_opkind_preserves_flags",
        enc_addsub_ext(1, 0, 0, 0b010, 0),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Extended {
                reg: arm_x(2),
                extend: ExtendOp::Uxtw,
                shift: 0,
            },
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 8;
    st.x[2] = 0xffff_ffff;
    st.pstate = 0x4000_0000;
    push_case(
        "add_x_sxtw_lsl2_opkind_preserves_flags",
        enc_addsub_ext(1, 0, 0, 0b110, 2),
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Extended {
                reg: arm_x(2),
                extend: ExtendOp::Sxtw,
                shift: 2,
            },
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x10;
    st.x[2] = 0xffff_ff0f;
    st.pstate = 0x6000_0000;
    push_case(
        "subs_w_uxtb_lsl1_opkind_sets_flags_zero_ext",
        enc_addsub_ext(0, 1, 1, 0b000, 1),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Extended {
                reg: arm_x(2),
                extend: ExtendOp::Uxtb,
                shift: 1,
            },
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = u64::MAX;
    st.x[2] = 0;
    st.pstate = 0x2000_0000;
    push_case(
        "adc_x_carry_in_set_opkind_preserves_flags",
        enc_addsub_carry(1, 0, 0),
        vec![OpKind::Adc {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x2000_0000;
    push_case(
        "adc_x_zero_imm_as_adc_zero_reg_preserves_flags",
        enc_addsub_carry_regs(1, 0, 0, RD, RN, 31),
        vec![OpKind::Adc {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff;
    st.x[2] = 0;
    st.pstate = 0x2000_0000;
    push_case(
        "adcs_w_carry_in_set_opkind_sets_flags_zero_ext",
        enc_addsub_carry(0, 0, 1),
        vec![OpKind::Adc {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 5;
    st.x[2] = 3;
    st.pstate = 0;
    push_case(
        "sbc_x_carry_clear_borrow_in_opkind_preserves_flags",
        enc_addsub_carry(1, 1, 0),
        vec![OpKind::Sbb {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 5;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    push_case(
        "sbcs_w_carry_set_opkind_sets_flags_zero_ext",
        enc_addsub_carry(0, 1, 1),
        vec![OpKind::Sbb {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0;
    st.pstate = 0;
    push_case(
        "sbcs_w_masked_zero_imm_as_sbcs_zero_reg_sets_flags",
        enc_addsub_carry_regs(0, 1, 1, RD, RN, 31),
        vec![OpKind::Sbb {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_0000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    push_case(
        "ngc_x_carry_set_opkind_preserves_flags",
        enc_addsub_carry_rn(1, 1, 0, 31),
        vec![OpKind::Sbb {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[2] = 1;
    st.pstate = 0;
    push_case(
        "ngcs_w_carry_clear_opkind_sets_flags_zero_ext",
        enc_addsub_carry_rn(0, 1, 1, 31),
        vec![OpKind::Sbb {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0xf000_0000;
    push_case(
        "sub_x_imm_preserves_flags",
        enc_addsub_imm_regs(1, 1, 0, 0, 0x123, RD, RN),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0x123),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0x3000_0000;
    push_case(
        "sub_w_zero_source_reg_as_mov_preserves_flags",
        enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, RN, 31),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(VReg::Imm(0)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = 0xaaaa_bbbb_8000_0001;
    st.pstate = 0x6000_0000;
    push_case(
        "sub_w_zero_imm_as_mov_preserves_flags",
        enc_addsub_imm_regs(0, 1, 0, 0, 0, RD, RN),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x3000_0000;
    push_case(
        "sub_w_zero_base_imm_as_movn_preserves_flags",
        enc_mov_wide(0, 0b00, 0, 0x33),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm(0x34),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x8000_0000;
    push_case(
        "sub_x_zero_base_imm_as_movn_preserves_flags",
        enc_mov_wide(1, 0b00, 0, 0x33),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm(0x34),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x1000_0000;
    push_case(
        "subs_w_zero_base_masked_zero_imm_as_subs_zero_regs_sets_flags",
        enc_addsub_shift_regs(0, 1, 1, 0, 0, RD, 31, 31),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm64(0x1_0000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xa000_0000;
    push_case(
        "inc_x_as_add_imm_preserves_flags",
        enc_addsub_imm_regs(1, 0, 0, 0, 1, RD, RN),
        vec![OpKind::Inc {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xfedc_ba98_7654_3210;
    st.x[1] = 0xffff_ffff_0000_0000;
    st.pstate = 0x7000_0000;
    push_case(
        "dec_w_as_sub_imm_zero_ext_preserves_flags",
        enc_addsub_imm_regs(0, 1, 0, 0, 1, RD, RN),
        vec![OpKind::Dec {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0;
    st.pstate = 0x6000_0000;
    push_case(
        "subs_x_imm_sets_flags",
        enc_addsub_imm_regs(1, 1, 1, 0, 1, RD, RN),
        vec![OpKind::Sub {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(1),
            width: OpWidth::W64,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xb000_0000;
    push_case(
        "movz_x_imm_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x7bcd),
        vec![OpKind::Mov {
            dst: arm_x(0),
            src: SrcOperand::Imm(0x7bcd),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0x6000_0000;
    push_case(
        "select_const_true_imm_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x2468),
        vec![OpKind::Select {
            dst: arm_x(0),
            cond: VReg::Imm(1),
            src_true: VReg::Imm(0x2468),
            src_false: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0xc000_0000;
    push_case(
        "select_const_true_w8_imm_as_uxtb_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0x34),
        vec![OpKind::Select {
            dst: arm_x(0),
            cond: VReg::Imm(1),
            src_true: VReg::Imm(0x1234),
            src_false: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0x0f0f_0000_ffff_0000;
    st.x[2] = 0xf0f0_1111_0000_2222;
    st.pstate = 0x2000_0000;
    push_case(
        "orr_x_reg_preserves_flags",
        enc_logical_shift_regs(1, 0b01, 0, 0, 0, RD, RN, RM),
        vec![OpKind::Or {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_de00;
    st.pstate = 0x6000_0000;
    push_case(
        "orr_x_low_mask_imm_opkind_preserves_flags",
        enc_logical_imm(1, 0b01, 1, 0, 5, RN),
        vec![OpKind::Or {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0x3f),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0x8000_0000;
    push_case(
        "orr_x_wrapping_mask_imm_opkind_preserves_flags",
        enc_logical_imm(1, 0b01, 1, 1, 1, RN),
        vec![OpKind::Or {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x8000_0000_0000_0001_u64 as i64),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.x[1] = 0xffff_ffff_aaaa_5555;
    st.x[2] = 0x1234_5678_0f0f_f0f0;
    st.pstate = 0xc000_0000;
    push_case(
        "eor_w_reg_zero_ext_preserves_flags",
        enc_logical_shift_regs(0, 0b10, 0, 0, 0, RD, RN, RM),
        vec![OpKind::Xor {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0xe000_0000;
    push_case(
        "eor_x_high_bit_imm_opkind_preserves_flags",
        enc_logical_imm(1, 0b10, 1, 1, 0, RN),
        vec![OpKind::Xor {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x8000_0000_0000_0000_u64 as i64),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x4000_0000;
    push_case(
        "eor_x_repeated_alternating_bit_imm_opkind_preserves_flags",
        enc_logical_imm(1, 0b10, 0, 0, 60, RN),
        vec![OpKind::Xor {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x5555_5555_5555_5555),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1010_2020_3030_4040;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xa000_0000;
    push_case(
        "and_x_zero_imm_opkind_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2020_3030_4040_5050;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0xb000_0000;
    push_case(
        "and_x_all_ones_imm_opkind_preserves_flags",
        enc_logical_shift_regs(1, 0b01, 0, 0, 0, RD, 31, RN),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(-1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2525_3535_4545_5555;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xe000_0000;
    push_case(
        "orr_x_zero_imm_opkind_preserves_flags",
        enc_logical_shift_regs(1, 0b01, 0, 0, 0, RD, 31, RN),
        vec![OpKind::Or {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2828_3838_4848_5858;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0xf000_0000;
    push_case(
        "orr_x_all_ones_imm_opkind_preserves_flags",
        enc_mov_wide(1, 0b00, 0, 0),
        vec![OpKind::Or {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(-1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_0000_1111;
    st.x[1] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x5000_0000;
    push_case(
        "orr_w_all_ones_imm_opkind_zero_ext_preserves_flags",
        enc_mov_wide(0, 0b00, 0, 0),
        vec![OpKind::Or {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(-1),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3030_4040_5050_6060;
    st.x[1] = 0x00ff_0000_ffff_00ff;
    st.pstate = 0x6000_0000;
    push_case(
        "eor_x_all_ones_imm_opkind_preserves_flags",
        enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RN),
        vec![OpKind::Xor {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(-1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3535_4545_5555_6565;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x3000_0000;
    push_case(
        "ands_x_zero_imm_opkind_sets_z",
        enc_logical_shift_regs(1, 0b11, 0, 0, 0, RD, 31, 31),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4040_5050_6060_7070;
    st.x[1] = 0x8000_0000_0000_1234;
    st.pstate = 0x1000_0000;
    push_case(
        "ands_x_all_ones_imm_opkind_sets_flags",
        enc_logical_shift_regs(1, 0b11, 0, 0, 0, RD, RN, RN),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(-1),
            width: OpWidth::W64,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5050_6060_7070_8080;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x9000_0000;
    push_case(
        "test_x_zero_imm_opkind_sets_z",
        enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, 31, 31),
        vec![OpKind::Test {
            src1: arm_x(1),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6060_7070_8080_9090;
    st.x[1] = 0x0f0f_f0f0_aaaa_5555;
    st.pstate = 0xc000_0000;
    push_case(
        "bic_x_zero_imm_opkind_preserves_flags",
        enc_logical_shift_regs(1, 0b01, 0, 0, 0, RD, 31, RN),
        vec![OpKind::AndNot {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7070_8080_9090_a0a0;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0xd000_0000;
    push_case(
        "bic_x_all_ones_imm_opkind_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0),
        vec![OpKind::AndNot {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(-1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x7000_0000;
    push_case(
        "ands_x_reg_sets_flags",
        enc_logical_shift_regs(1, 0b11, 0, 0, 0, RD, RN, RM),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5656_7878_9a9a_bcbc;
    st.x[1] = 0xffff_ffff_1234_abcd;
    st.pstate = 0x3000_0000;
    push_case(
        "and_w_shifted_byte_mask_imm_opkind_zero_ext_preserves_flags",
        enc_logical_imm(0, 0b00, 0, 24, 7, RN),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0xff00),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6767_8989_abab_cdcd;
    st.x[1] = 0xffff_ffff_1234_5678;
    st.pstate = 0x2000_0000;
    push_case(
        "and_w_wrapping_mask_imm_opkind_zero_ext_preserves_flags",
        enc_logical_imm(0, 0b00, 0, 4, 7, RN),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0xf000_000f),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7878_9a9a_bcbc_dede;
    st.x[1] = 0xffff_ffff_1234_abcd;
    st.pstate = 0x5000_0000;
    push_case(
        "and_w_repeated_byte_mask_imm_opkind_zero_ext_preserves_flags",
        enc_logical_imm(0, 0b00, 0, 0, 39, RN),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0x00ff_00ff),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3434_5656_7878_9a9a;
    st.x[1] = 0xffff_ffff_ffff_ffe0;
    st.pstate = 0xf000_0000;
    push_case(
        "ands_w_low_mask_imm_opkind_sets_flags_zero_ext",
        enc_logical_imm(0, 0b11, 0, 0, 4, RN),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0x1f),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1212_3434_5656_7878;
    st.x[1] = 0xffff_0000_ffff_0000;
    st.x[2] = 0x00ff_00ff_00ff_00ff;
    st.pstate = 0x2000_0000;
    push_case(
        "bic_x_opkind_preserves_flags",
        enc_logical_shift_regs(1, 0b00, 0, 1, 0, RD, RN, RM),
        vec![OpKind::AndNot {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3434_5656_7878_9a9a;
    st.x[1] = 0xffff_ffff_8000_00f0;
    st.x[2] = 0xffff_ffff_0000_00f0;
    st.pstate = 0x9000_0000;
    push_case(
        "bics_w_opkind_sets_flags_zero_ext",
        enc_logical_shift_regs(0, 0b11, 0, 1, 0, RD, RN, RM),
        vec![OpKind::AndNot {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xfedc_ba98_7654_3210;
    st.x[1] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x6000_0000;
    push_case(
        "bic_x_high_bit_imm_opkind_preserves_flags",
        enc_logical_imm(1, 0b00, 1, 0, 62, RN),
        vec![OpKind::AndNot {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x8000_0000_0000_0000_u64 as i64),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0xffff_ffff_0000_ff00;
    st.pstate = 0x2000_0000;
    push_case(
        "bics_w_high_bits_imm_opkind_sets_flags_zero_ext",
        enc_logical_imm(0, 0b11, 0, 0, 7, RN),
        vec![OpKind::AndNot {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm(0xffff_ff00),
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4545_6767_8989_abab;
    st.x[1] = 0xffff_0000_00ff_00ff;
    st.x[2] = 0x00ff_00ff_ffff_0000;
    st.pstate = 0xa000_0000;
    push_case(
        "and_x_lsr8_opkind_preserves_flags",
        enc_logical_shift(1, 0b00, 1, 0, 8),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Lsr,
                amount: 8,
            },
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5656_7878_9a9a_bcbc;
    st.x[1] = 0xffff_ffff_0000_00f0;
    st.x[2] = 0xffff_ffff_8000_0000;
    st.pstate = 0x5000_0000;
    push_case(
        "orr_w_asr31_opkind_zero_ext_preserves_flags",
        enc_logical_shift(0, 0b01, 2, 0, 31),
        vec![OpKind::Or {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Asr,
                amount: 31,
            },
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6767_8989_abab_cdcd;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.x[2] = 0x0fed_cba9_8765_4321;
    st.pstate = 0x3000_0000;
    push_case(
        "eor_x_ror13_opkind_preserves_flags",
        enc_logical_shift(1, 0b10, 3, 0, 13),
        vec![OpKind::Xor {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Ror,
                amount: 13,
            },
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7878_9a9a_bcbc_dede;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0;
    push_case(
        "ands_x_lsr1_opkind_sets_flags",
        enc_logical_shift(1, 0b11, 1, 0, 1),
        vec![OpKind::And {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Lsr,
                amount: 1,
            },
            width: OpWidth::W64,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8989_abab_cdcd_efef;
    st.x[1] = 0x00ff_0000_0000_0000;
    st.x[2] = 0x0000_00ff_0000_0000;
    st.pstate = 0xf000_0000;
    push_case(
        "tst_x_lsr8_opkind_discards_result_sets_flags",
        enc_logical_shift_regs(1, 0b11, 1, 0, 8, 31, RN, RM),
        vec![OpKind::Test {
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Lsr,
                amount: 8,
            },
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9a9a_bcbc_dede_f0f0;
    st.x[1] = u64::MAX;
    st.x[2] = 1;
    st.pstate = 0x7000_0000;
    push_case(
        "bic_x_lsl4_opkind_preserves_flags",
        enc_logical_shift(1, 0b00, 0, 1, 4),
        vec![OpKind::AndNot {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Lsl,
                amount: 4,
            },
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0x10;
    st.x[2] = 0x20;
    st.pstate = 0x4000_0000;
    push_case(
        "cmp_x_opkind_sets_flags",
        enc_addsub_shift_regs(1, 1, 1, 0, 0, 31, RN, RM),
        vec![OpKind::Cmp {
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0x123;
    st.pstate = 0x9000_0000;
    push_case(
        "cmp_x_imm_opkind_sets_flags",
        enc_addsub_imm_regs(1, 1, 1, 0, 0x123, 31, RN),
        vec![OpKind::Cmp {
            src1: arm_x(1),
            src2: SrcOperand::Imm(0x123),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = 0xffff_ffff;
    st.pstate = 0x1000_0000;
    push_case(
        "cmp_w_imm_masked_neg_one_as_cmn_one_sets_flags",
        enc_addsub_imm_regs(0, 0, 1, 0, 1, 31, RN),
        vec![OpKind::Cmp {
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0xffff_ffff),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.pstate = 0x8000_0000;
    push_case(
        "cmp_x_zero_base_zero_imm_as_cmp_zero_regs_sets_flags",
        enc_addsub_shift_regs(1, 1, 1, 0, 0, 31, 31, 31),
        vec![OpKind::Cmp {
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.pstate = 0;
    push_case(
        "cmp_w_zero_base_masked_zero_imm_as_cmp_zero_regs_sets_flags",
        enc_addsub_shift_regs(0, 1, 1, 0, 0, 31, 31, 31),
        vec![OpKind::Cmp {
            src1: VReg::Imm(0),
            src2: SrcOperand::Imm64(0x1_0000_0000),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0x0100;
    st.x[2] = 0x0200;
    st.pstate = 0xe000_0000;
    push_case(
        "cmp_x_lsr4_opkind_sets_flags",
        enc_addsub_shift_regs(1, 1, 1, 1, 4, 31, RN, RM),
        vec![OpKind::Cmp {
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Lsr,
                amount: 4,
            },
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x7fff_ffff;
    st.x[2] = 1;
    st.pstate = 0x2000_0000;
    push_case(
        "cmn_w_lsl1_opkind_discards_result_sets_flags",
        enc_addsub_shift_regs(0, 0, 1, 0, 1, 31, RN, RM),
        vec![OpKind::Add {
            dst: VReg::virt(100),
            src1: arm_x(1),
            src2: SrcOperand::Shifted {
                reg: arm_x(2),
                shift: ShiftOp::Lsl,
                amount: 1,
            },
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0;
    st.x[2] = 0xffff;
    st.pstate = 0x5000_0000;
    push_case(
        "cmp_x_sxth_lsl3_opkind_sets_flags",
        enc_addsub_ext_regs(1, 1, 1, 0b101, 3, 31, RN, RM),
        vec![OpKind::Cmp {
            src1: arm_x(1),
            src2: SrcOperand::Extended {
                reg: arm_x(2),
                extend: ExtendOp::Sxth,
                shift: 3,
            },
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_fff0;
    st.x[2] = 1;
    st.pstate = 0x3000_0000;
    push_case(
        "cmn_w_uxtb_lsl4_opkind_discards_result_sets_flags",
        enc_addsub_ext_regs(0, 0, 1, 0b000, 4, 31, RN, RM),
        vec![OpKind::Add {
            dst: VReg::virt(101),
            src1: arm_x(1),
            src2: SrcOperand::Extended {
                reg: arm_x(2),
                extend: ExtendOp::Uxtb,
                shift: 4,
            },
            width: OpWidth::W32,
            flags: FlagUpdate::All,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0x3000_0000;
    push_case(
        "test_x_opkind_sets_flags",
        enc_logical_shift_regs(1, 0b11, 0, 0, 0, 31, RN, RM),
        vec![OpKind::Test {
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0x0000_0000_0000_ff00;
    st.pstate = 0x9000_0000;
    push_case(
        "test_x_shifted_byte_mask_imm_opkind_sets_flags",
        enc_logical_imm_regs(1, 0b11, 1, 56, 7, RN, 31),
        vec![OpKind::Test {
            src1: arm_x(1),
            src2: SrcOperand::Imm(0xff00),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0x0000_0000_0000_0003;
    st.pstate = 0x9000_0000;
    push_case(
        "shl_x_imm_preserves_flags",
        enc_bitfield(1, 0b10, 51, 50),
        vec![OpKind::Shl {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(13),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.pstate = 0x5000_0000;
    push_case(
        "shl_x_imm_src_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x1230),
        vec![OpKind::Shl {
            dst: arm_x(0),
            src: VReg::Imm(0x123),
            amount: SrcOperand::Imm(4),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.pstate = 0x2000_0000;
    push_case(
        "shr_w_imm_zero_ext_preserves_flags",
        enc_bitfield(0, 0b10, 9, 31),
        vec![OpKind::Shr {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(9),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.pstate = 0x6000_0000;
    push_case(
        "shr_x_imm_src_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x123),
        vec![OpKind::Shr {
            dst: arm_x(0),
            src: VReg::Imm(0x1230),
            amount: SrcOperand::Imm(4),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x8000_0000_0000_0000;
    st.pstate = 0x4000_0000;
    push_case(
        "sar_x_imm_preserves_flags",
        enc_bitfield(1, 0b00, 9, 63),
        vec![OpKind::Sar {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(9),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x9000_0000;
    push_case(
        "sar_x_imm_src_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x123),
        vec![OpKind::Sar {
            dst: arm_x(0),
            src: VReg::Imm(0x1230),
            amount: SrcOperand::Imm(4),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x8000_0000_0000_0000;
    st.pstate = 0x5000_0000;
    push_case(
        "cwd_x_as_asr63_preserves_flags",
        enc_bitfield(1, 0b00, 63, 63),
        vec![OpKind::Cwd {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x7000_0000;
    push_case(
        "cwd_x_imm_sign_set_as_movn_zero_preserves_flags",
        enc_mov_wide(1, 0b00, 0, 0),
        vec![OpKind::Cwd {
            dst: arm_x(0),
            src: VReg::Imm(-1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.pstate = 0xc000_0000;
    push_case(
        "cwd_w_as_asr31_zero_ext_preserves_flags",
        enc_bitfield(0, 0b00, 31, 31),
        vec![OpKind::Cwd {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x6000_0000;
    push_case(
        "ror_x_imm_preserves_flags",
        enc_extract(1, RN, RN, 17),
        vec![OpKind::Ror {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(17),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x9000_0000;
    push_case(
        "ror_x_imm_src_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x123),
        vec![OpKind::Ror {
            dst: arm_x(0),
            src: VReg::Imm(0x1230),
            amount: SrcOperand::Imm(4),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0xa000_0000;
    push_case(
        "rol_x_imm_as_ror_preserves_flags",
        enc_extract(1, RN, RN, 51),
        vec![OpKind::Rol {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(13),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x5000_0000;
    push_case(
        "rol_x_imm_src_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x1230),
        vec![OpKind::Rol {
            dst: arm_x(0),
            src: VReg::Imm(0x123),
            amount: SrcOperand::Imm(4),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0x3000_0000;
    push_case(
        "rol_w_imm_as_ror_zero_ext_preserves_flags",
        enc_extract(0, RN, RN, 25),
        vec![OpKind::Rol {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(7),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 1;
    st.x[2] = 65;
    st.pstate = 0x1000_0000;
    push_case(
        "shl_x_reg_masked_count_preserves_flags",
        enc_dp2(1, 0b1000),
        vec![OpKind::Shl {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 4;
    st.pstate = 0x8000_0000;
    push_case(
        "shr_x_reg_preserves_flags",
        enc_dp2(1, 0b1001),
        vec![OpKind::Shr {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 4;
    st.pstate = 0x3000_0000;
    push_case(
        "sar_x_reg_preserves_flags",
        enc_dp2(1, 0b1010),
        vec![OpKind::Sar {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0000_1111_2222_3333;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.x[2] = 36;
    st.pstate = 0x5000_0000;
    push_case(
        "ror_w_reg_zero_ext_masked_count_preserves_flags",
        enc_dp2(0, 0b1011),
        vec![OpKind::Ror {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0x0000_0000_0000_00f0;
    st.pstate = 0x7000_0000;
    push_case(
        "clz_x_opkind_preserves_flags",
        enc_dp1(1, 0b000100),
        vec![OpKind::Clz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0x9000_0000;
    push_case(
        "clz_x_imm_zero_as_width_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 64),
        vec![OpKind::Clz {
            dst: arm_x(0),
            src: VReg::Imm(0),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xffff_ffff_ffff_f000;
    st.pstate = 0x6000_0000;
    push_case(
        "cls_x_lifted_pattern_preserves_flags",
        enc_dp1(1, 0b000101),
        vec![
            OpKind::Sar {
                dst: VReg::virt(300),
                src: arm_x(1),
                amount: SrcOperand::Imm(63),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            OpKind::Xor {
                dst: VReg::virt(301),
                src1: arm_x(1),
                src2: SrcOperand::Reg(VReg::virt(300)),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            OpKind::Clz {
                dst: VReg::virt(302),
                src: VReg::virt(301),
                width: OpWidth::W64,
            },
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::virt(302),
                src2: SrcOperand::Imm(1),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0x0000_0000_0000_00f0;
    st.pstate = 0xa000_0000;
    push_case(
        "cls_w_lifted_imm_src_as_movz_preserves_flags",
        enc_dp1(0, 0b000101),
        vec![
            OpKind::Sar {
                dst: VReg::virt(306),
                src: VReg::Imm(0xf0),
                amount: SrcOperand::Imm(31),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            OpKind::Xor {
                dst: VReg::virt(307),
                src1: VReg::Imm(0xf0),
                src2: SrcOperand::Reg(VReg::virt(306)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            OpKind::Clz {
                dst: VReg::virt(308),
                src: VReg::virt(307),
                width: OpWidth::W32,
            },
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::virt(308),
                src2: SrcOperand::Imm(1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0xa000_0000;
    push_case(
        "cls_w_lifted_pattern_zero_ext_preserves_flags",
        enc_dp1(0, 0b000101),
        vec![
            OpKind::Sar {
                dst: VReg::virt(303),
                src: arm_x(1),
                amount: SrcOperand::Imm(31),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            OpKind::Xor {
                dst: VReg::virt(304),
                src1: arm_x(1),
                src2: SrcOperand::Reg(VReg::virt(303)),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
            OpKind::Clz {
                dst: VReg::virt(305),
                src: VReg::virt(304),
                width: OpWidth::W32,
            },
            OpKind::Sub {
                dst: arm_x(0),
                src1: VReg::virt(305),
                src2: SrcOperand::Imm(1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xffff_ffff_0000_0001;
    st.pstate = 0x5000_0000;
    push_case(
        "rbit_w_opkind_zero_ext_preserves_flags",
        enc_dp1(0, 0b000000),
        vec![OpKind::Rbit {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0x5000_0000;
    push_case(
        "rbit_w_imm_as_movz_zero_ext_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 1),
        vec![OpKind::Rbit {
            dst: arm_x(0),
            src: VReg::Imm(0x8000_0000),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1122_3344_5566_7788;
    st.pstate = 0x3000_0000;
    push_case(
        "rbit_w16_opkind_as_mov_preserves_flags",
        enc_mov_reg(1, RD, RN),
        vec![OpKind::Rbit {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1357_9bdf_2468_ace0;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xa000_0000;
    push_case(
        "rbit_w8_opkind_as_mov_preserves_flags",
        enc_mov_reg(1, RD, RN),
        vec![OpKind::Rbit {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1357_9bdf_2468_ace0;
    st.pstate = 0xa000_0000;
    push_case(
        "rbit_w8_imm_as_movz_full_imm_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x1234),
        vec![OpKind::Rbit {
            dst: arm_x(0),
            src: VReg::Imm(0x1234),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0x1122_3344_5566_7788;
    st.pstate = 0x9000_0000;
    push_case(
        "bswap_x_opkind_preserves_flags",
        enc_dp1(1, 0b000011),
        vec![OpKind::Bswap {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0xd000_0000;
    push_case(
        "bswap_w8_opkind_as_mov_preserves_flags",
        enc_mov_reg(1, RD, RN),
        vec![OpKind::Bswap {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.pstate = 0xd000_0000;
    push_case(
        "bswap_w8_imm_as_movz_full_imm_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x1234),
        vec![OpKind::Bswap {
            dst: arm_x(0),
            src: VReg::Imm(0x1234),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0x2000_0000;
    push_case(
        "ubfx_x_opkind_preserves_flags",
        enc_bitfield(1, 0b10, 8, 23),
        vec![OpKind::Bfx {
            dst: arm_x(0),
            src: arm_x(1),
            lsb: 8,
            width_bits: 16,
            sign_extend: false,
            op_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x5000_0000;
    push_case(
        "ubfx_x_imm_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x9abc),
        vec![OpKind::Bfx {
            dst: arm_x(0),
            src: VReg::Imm(0x1234_5678_9abc_def0),
            lsb: 16,
            width_bits: 16,
            sign_extend: false,
            op_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xa000_0000;
    push_case(
        "bextr_x_imm_control_as_ubfx_preserves_flags",
        enc_bitfield(1, 0b10, 8, 23),
        vec![OpKind::Bextr {
            dst: arm_x(0),
            src: arm_x(1),
            control: VReg::Imm((16 << 8) | 8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x2000_0000;
    push_case(
        "bextr_w_two_imms_all_ones_as_movn_preserves_flags",
        enc_mov_wide(0, 0b00, 0, 0),
        vec![OpKind::Bextr {
            dst: arm_x(0),
            src: VReg::Imm(-1),
            control: VReg::Imm(32 << 8),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x6000_0000;
    push_case(
        "bextr_x_two_imms_all_ones_as_movn_preserves_flags",
        enc_mov_wide(1, 0b00, 0, 0),
        vec![OpKind::Bextr {
            dst: arm_x(0),
            src: VReg::Imm(-1),
            control: VReg::Imm(64 << 8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xe000_0000;
    push_case(
        "pdep_x_contiguous_imm_mask_as_ubfiz_preserves_flags",
        enc_bitfield(1, 0b10, 60, 4),
        vec![OpKind::Pdep {
            dst: arm_x(0),
            src: arm_x(1),
            mask: VReg::Imm(0x1f0),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x7000_0000;
    push_case(
        "pext_x_contiguous_imm_mask_as_ubfx_preserves_flags",
        enc_bitfield(1, 0b10, 8, 15),
        vec![OpKind::Pext {
            dst: arm_x(0),
            src: arm_x(1),
            mask: VReg::Imm(0xff00),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_0000_000b;
    st.pstate = 0xb000_0000;
    push_case(
        "pdep_w16_contiguous_imm_mask_as_ubfiz_preserves_flags",
        enc_bitfield(0, 0b10, 24, 3),
        vec![OpKind::Pdep {
            dst: arm_x(0),
            src: arm_x(1),
            mask: VReg::Imm(0x0f00),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_0000_00b4;
    st.pstate = 0x5000_0000;
    push_case(
        "pext_w8_contiguous_imm_mask_as_ubfx_preserves_flags",
        enc_bitfield(0, 0b10, 2, 4),
        vec![OpKind::Pext {
            dst: arm_x(0),
            src: arm_x(1),
            mask: VReg::Imm(0x1c),
            width: OpWidth::W8,
        }],
        st,
    );

    drop(push_case);
    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bfx {
        dst: arm_x(0),
        src: VReg::Imm(0xf0),
        lsb: 4,
        width_bits: 4,
        sign_extend: true,
        op_width: OpWidth::W32,
    }])
    .unwrap_or_else(|e| {
        panic!("sbfx_w_imm_sign_extend_as_movn_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sbfx_w_imm_sign_extend_as_movn_preserves_flags".into(),
        [enc_mov_wide(0, 0b00, 0, 0), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bextr {
        dst: arm_x(0),
        src: arm_x(1),
        control: VReg::Imm((16 << 8) | 8),
        width: OpWidth::W64,
        flags: bextr_flags(),
    }])
    .unwrap_or_else(|e| {
        panic!("bextr_x_imm_control_with_flags_sets_nzcv: native lowering failed: {e}")
    });
    cases.push((
        "bextr_x_imm_control_with_flags_sets_nzcv".into(),
        [
            enc_bitfield(1, 0b10, 8, 23),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bextr {
        dst: arm_x(0),
        src: VReg::Imm(0x1234_5678_9abc_def0),
        control: VReg::Imm((32 << 8) | 16),
        width: OpWidth::W64,
        flags: bextr_flags(),
    }])
    .unwrap_or_else(|e| {
        panic!("bextr_x_two_imms_with_flags_sets_nzcv: native lowering failed: {e}")
    });
    cases.push((
        "bextr_x_two_imms_with_flags_sets_nzcv".into(),
        [
            enc_mov_wide(1, 0b10, 0, 0x9abc),
            enc_mov_wide(1, 0b11, 1, 0x5678),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bextr {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        control: VReg::Imm(64 << 8),
        width: OpWidth::W64,
        flags: bextr_flags(),
    }])
    .unwrap_or_else(|e| {
        panic!("bextr_x_two_imms_all_ones_with_flags_sets_nzcv: native lowering failed: {e}")
    });
    cases.push((
        "bextr_x_two_imms_all_ones_with_flags_sets_nzcv".into(),
        [
            enc_mov_wide(1, 0b00, 0, 0),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut push_case = |label: &str, source: u32, ops: Vec<OpKind>, st: ArmState| {
        let lowered = lower_aarch64_native_ops(ops)
            .unwrap_or_else(|e| panic!("{label}: native lowering failed: {e}"));
        cases.push((label.into(), [source, NOP, NOP], lowered, st));
    };

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_00d4;
    st.pstate = 0x6000_0000;
    push_case(
        "bextr_w8_imm_control_as_ubfx_preserves_flags",
        enc_bitfield(0, 0b10, 2, 4),
        vec![OpKind::Bextr {
            dst: arm_x(0),
            src: arm_x(1),
            control: VReg::Imm((3 << 8) | 2),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_0000_f234;
    st.pstate = 0x9000_0000;
    push_case(
        "bextr_w16_imm_control_clips_at_subword_width_preserves_flags",
        enc_bitfield(0, 0b10, 12, 15),
        vec![OpKind::Bextr {
            dst: arm_x(0),
            src: arm_x(1),
            control: VReg::Imm((8 << 8) | 12),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_00ff;
    st.pstate = 0x3000_0000;
    push_case(
        "bextr_w8_empty_extract_as_zero_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0),
        vec![OpKind::Bextr {
            dst: arm_x(0),
            src: arm_x(1),
            control: VReg::Imm((1 << 8) | 8),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0x1000_0000;
    push_case(
        "bzhi_x_imm_index_as_and_mask_preserves_flags",
        enc_logical_imm(1, 0b00, 1, 0, 12, RN),
        vec![OpKind::Bzhi {
            dst: arm_x(0),
            src: arm_x(1),
            index: VReg::Imm(13),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.pstate = 0x1000_0000;
    push_case(
        "bzhi_x_two_imms_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x34),
        vec![OpKind::Bzhi {
            dst: arm_x(0),
            src: VReg::Imm(0x1234),
            index: VReg::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x2000_0000;
    push_case(
        "bzhi_w_two_imms_all_ones_as_movn_preserves_flags",
        enc_mov_wide(0, 0b00, 0, 0),
        vec![OpKind::Bzhi {
            dst: arm_x(0),
            src: VReg::Imm(-1),
            index: VReg::Imm(32),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x6000_0000;
    push_case(
        "bzhi_x_two_imms_all_ones_as_movn_preserves_flags",
        enc_mov_wide(1, 0b00, 0, 0),
        vec![OpKind::Bzhi {
            dst: arm_x(0),
            src: VReg::Imm(-1),
            index: VReg::Imm(64),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    drop(push_case);
    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0x1000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bzhi {
        dst: arm_x(0),
        src: arm_x(1),
        index: VReg::Imm(13),
        width: OpWidth::W64,
        flags: bzhi_flags(),
    }])
    .unwrap_or_else(|e| {
        panic!("bzhi_x_imm_index_with_flags_clears_carry: native lowering failed: {e}")
    });
    cases.push((
        "bzhi_x_imm_index_with_flags_clears_carry".into(),
        [
            enc_logical_imm(1, 0b00, 1, 0, 12, RN),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0x8000_0000_0000_0001;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bzhi {
        dst: arm_x(0),
        src: arm_x(1),
        index: VReg::Imm(64),
        width: OpWidth::W64,
        flags: bzhi_flags(),
    }])
    .unwrap_or_else(|e| {
        panic!("bzhi_x_imm_index_with_flags_sets_carry: native lowering failed: {e}")
    });
    cases.push((
        "bzhi_x_imm_index_with_flags_sets_carry".into(),
        [
            enc_mov_reg(1, RD, RN),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
            enc_cfinv(),
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bzhi {
        dst: arm_x(0),
        src: VReg::Imm(1),
        index: VReg::Imm(64),
        width: OpWidth::W64,
        flags: bzhi_flags(),
    }])
    .unwrap_or_else(|e| {
        panic!("bzhi_x_two_imms_with_flags_sets_carry: native lowering failed: {e}")
    });
    cases.push((
        "bzhi_x_two_imms_with_flags_sets_carry".into(),
        [
            enc_mov_wide(1, 0b10, 0, 1),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
            enc_cfinv(),
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bzhi {
        dst: arm_x(0),
        src: VReg::Imm(-1),
        index: VReg::Imm(64),
        width: OpWidth::W64,
        flags: bzhi_flags(),
    }])
    .unwrap_or_else(|e| {
        panic!("bzhi_x_two_imms_all_ones_with_flags_sets_carry: native lowering failed: {e}")
    });
    cases.push((
        "bzhi_x_two_imms_all_ones_with_flags_sets_carry".into(),
        [
            enc_mov_wide(1, 0b00, 0, 0),
            enc_logical_reg_n(1, 0b11, 0, 31, RD, RD),
            enc_cfinv(),
        ],
        lowered,
        st,
    ));

    let mut push_case = |label: &str, source: u32, ops: Vec<OpKind>, st: ArmState| {
        let lowered = lower_aarch64_native_ops(ops)
            .unwrap_or_else(|e| panic!("{label}: native lowering failed: {e}"));
        cases.push((label.into(), [source, NOP, NOP], lowered, st));
    };

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0xffff_ffff_0000_f234;
    st.pstate = 0xb000_0000;
    push_case(
        "bzhi_w16_imm_index_at_width_as_uxth_preserves_flags",
        enc_bitfield(0, 0b10, 0, 15),
        vec![OpKind::Bzhi {
            dst: arm_x(0),
            src: arm_x(1),
            index: VReg::Imm(16),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_00ff;
    st.pstate = 0x7000_0000;
    push_case(
        "bzhi_w8_imm_index_zero_as_zero_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0),
        vec![OpKind::Bzhi {
            dst: arm_x(0),
            src: arm_x(1),
            index: VReg::Imm(0),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0xffff_ffff_0000_08f0;
    st.pstate = 0x4000_0000;
    push_case(
        "sbfx_w_opkind_sign_ext_zero_ext_preserves_flags",
        enc_bitfield(0, 0b00, 4, 11),
        vec![OpKind::Bfx {
            dst: arm_x(0),
            src: arm_x(1),
            lsb: 4,
            width_bits: 8,
            sign_extend: true,
            op_width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_de77;
    st.pstate = 0x6000_0000;
    push_case(
        "bfi_x_opkind_preserves_flags",
        enc_bitfield(1, 0b01, 56, 7),
        vec![OpKind::Bfi {
            dst: arm_x(0),
            dst_in: arm_x(0),
            src: arm_x(1),
            lsb: 8,
            width_bits: 8,
            op_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_00ff;
    st.pstate = 0xa000_0000;
    push_case(
        "bfi_x_all_ones_imm_src_as_orr_preserves_flags",
        enc_bitfield(1, 0b01, 56, 7),
        vec![OpKind::Bfi {
            dst: arm_x(0),
            dst_in: arm_x(0),
            src: VReg::Imm(0xff),
            lsb: 8,
            width_bits: 8,
            op_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0x1000_0000;
    push_case(
        "bfi_x_all_ones_imm_src_from_input_as_orr_preserves_flags",
        enc_logical_imm(1, 0b01, 1, 56, 7, 1),
        vec![OpKind::Bfi {
            dst: arm_x(0),
            dst_in: arm_x(1),
            src: VReg::Imm(0xff),
            lsb: 8,
            width_bits: 8,
            op_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x2000_0000;
    push_case(
        "bfi_x_zero_imm_src_as_and_preserves_flags",
        enc_bitfield_rn(1, 0b01, 56, 7, 31),
        vec![OpKind::Bfi {
            dst: arm_x(0),
            dst_in: arm_x(0),
            src: VReg::Imm(0),
            lsb: 8,
            width_bits: 8,
            op_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_de3c;
    st.pstate = 0x5000_0000;
    push_case(
        "bfi_x_encodable_imm_src_as_and_orr_preserves_flags",
        enc_bitfield(1, 0b01, 56, 7),
        vec![OpKind::Bfi {
            dst: arm_x(0),
            dst_in: arm_x(0),
            src: VReg::Imm(0x3c),
            lsb: 8,
            width_bits: 8,
            op_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0x9000_0000;
    push_case(
        "bfi_w_full_width_imm_src_as_movz_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0x1234),
        vec![OpKind::Bfi {
            dst: arm_x(0),
            dst_in: arm_x(1),
            src: VReg::Imm(0x1234),
            lsb: 0,
            width_bits: 32,
            op_width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0x6000_0000;
    push_case(
        "bfi_x_full_width_all_ones_imm_src_as_movn_preserves_flags",
        enc_mov_wide(1, 0b00, 0, 0),
        vec![OpKind::Bfi {
            dst: arm_x(0),
            dst_in: arm_x(1),
            src: VReg::Imm(-1),
            lsb: 0,
            width_bits: 64,
            op_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0xffff_ffff_1234_56ab;
    st.pstate = 0x3000_0000;
    push_case(
        "zero_extend_uxtb_w_opkind_preserves_flags",
        enc_bitfield(0, 0b10, 0, 7),
        vec![OpKind::ZeroExtend {
            dst: arm_x(0),
            src: arm_x(1),
            from_width: OpWidth::W8,
            to_width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_12ab;
    st.pstate = 0xc000_0000;
    push_case(
        "zero_extend_w8_to_w16_as_uxtb_preserves_flags",
        enc_bitfield(0, 0b10, 0, 7),
        vec![OpKind::ZeroExtend {
            dst: arm_x(0),
            src: arm_x(1),
            from_width: OpWidth::W8,
            to_width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0x5000_0000;
    push_case(
        "zero_extend_w8_imm_src_to_x_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0xab),
        vec![OpKind::ZeroExtend {
            dst: arm_x(0),
            src: VReg::Imm(0x12ab),
            from_width: OpWidth::W8,
            to_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x6000_0000;
    push_case(
        "zero_extend_w8_imm_src_to_w16_as_movz_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0xab),
        vec![OpKind::ZeroExtend {
            dst: arm_x(0),
            src: VReg::Imm(0x12ab),
            from_width: OpWidth::W8,
            to_width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0x1000_0000;
    push_case(
        "sign_extend_sxtw_x_opkind_preserves_flags",
        enc_bitfield(1, 0b00, 0, 31),
        vec![OpKind::SignExtend {
            dst: arm_x(0),
            src: arm_x(1),
            from_width: OpWidth::W32,
            to_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.pstate = 0xa000_0000;
    push_case(
        "sign_extend_w8_imm_src_positive_to_x_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x7f),
        vec![OpKind::SignExtend {
            dst: arm_x(0),
            src: VReg::Imm(0x7f),
            from_width: OpWidth::W8,
            to_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_00ff;
    st.pstate = 0xd000_0000;
    push_case(
        "sign_extend_w8_imm_src_negative_to_x_as_movn_preserves_flags",
        enc_bitfield(1, 0b00, 0, 7),
        vec![OpKind::SignExtend {
            dst: arm_x(0),
            src: VReg::Imm(0xff),
            from_width: OpWidth::W8,
            to_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0xb000_0000;
    push_case(
        "sign_extend_w8_imm_src_negative_to_w16_as_movz_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0xff80),
        vec![OpKind::SignExtend {
            dst: arm_x(0),
            src: VReg::Imm(0x80),
            from_width: OpWidth::W8,
            to_width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = 0x0000_0000_1234_5678;
    st.x[2] = 0x10;
    st.pstate = 0x9000_0000;
    push_case(
        "mul_x_opkind_preserves_flags",
        enc_dp3_ra(1, 0b000, 0, 31),
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    push_case(
        "mul_w_opkind_zero_ext_preserves_flags",
        enc_dp3_ra(0, 0b000, 0, 31),
        vec![OpKind::MulS {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0xa000_0000;
    push_case(
        "mulu_x_imm_zero_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0),
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_ffff_8001;
    st.pstate = 0x1000_0000;
    push_case(
        "muls_w16_imm_zero_as_movz_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0),
        vec![OpKind::MulS {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_8765_4321;
    st.pstate = 0x5000_0000;
    push_case(
        "mulu_w_imm_masked_zero_as_movz_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0),
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_0000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0x6000_0000;
    push_case(
        "mulu_x_imm_one_as_mov_preserves_flags",
        enc_mov_reg(1, RD, RN),
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0x7000_0000;
    push_case(
        "mulu_x_imm_power_of_two_as_lsl_preserves_flags",
        enc_bitfield(1, 0b10, 61, 60),
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0x0fed_cba9_8765_4321;
    st.pstate = 0x2000_0000;
    push_case(
        "mulu_x_left_imm_power_of_two_as_lsl_preserves_flags",
        enc_bitfield(1, 0b10, 61, 60),
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: VReg::Imm(8),
            src2: SrcOperand::Reg(arm_x(1)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x3000_0000;
    push_case(
        "mulu_x_two_imms_as_mov_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 63),
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: VReg::Imm(7),
            src2: SrcOperand::Imm(9),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x5000_0000;
    push_case(
        "muls_w_two_imms_negative_product_as_movn_preserves_flags",
        enc_mov_wide(0, 0b00, 0, 0xe),
        vec![OpKind::MulS {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: VReg::Imm(-3),
            src2: SrcOperand::Imm(5),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_ffff_8003;
    st.pstate = 0x8000_0000;
    push_case(
        "muls_w16_imm_masked_power_of_two_as_lsl_uxth_preserves_flags",
        enc_bitfield(0, 0b10, 29, 12),
        vec![OpKind::MulS {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_0008),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x8000_0000_0000_0001;
    st.pstate = 0x9000_0000;
    push_case(
        "mulu_x_imm_neg_one_as_neg_preserves_flags",
        enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RN),
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(-1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234;
    st.x[2] = 0x1000;
    st.x[3] = 0x55;
    st.pstate = 0x4000_0000;
    push_case(
        "madd_x_opkind_preserves_flags",
        enc_dp3(1, 0b000, 0),
        vec![OpKind::MulAdd {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: arm_x(1),
            src2: arm_x(2),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1122_3344_5566_7788;
    st.x[3] = 0x0102_0304_0506_0708;
    st.pstate = 0xa000_0000;
    push_case(
        "muladd_x_imm_one_as_add_preserves_flags",
        enc_addsub_shift_regs(1, 0, 0, 0, 0, RD, 3, RN),
        vec![OpKind::MulAdd {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: VReg::Imm(1),
            src2: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x1122_3344_5566_7788;
    st.x[3] = 0x0102_0304_0506_0708;
    st.pstate = 0xb000_0000;
    push_case(
        "muladd_x_imm_zero_as_acc_mov_preserves_flags",
        enc_mov_reg(1, RD, 3),
        vec![OpKind::MulAdd {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: VReg::Imm(0),
            src2: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x0102_0304_0506_0708;
    st.x[3] = 0x1111_2222_3333_4444;
    st.pstate = 0x9000_0000;
    push_case(
        "muladd_x_imm_power_of_two_as_add_shifted_preserves_flags",
        enc_addsub_shift_regs(1, 0, 0, 0, 3, RD, 3, RN),
        vec![OpKind::MulAdd {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: VReg::Imm(8),
            src2: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[3] = 0x3333_4444_5555_6666;
    st.pstate = 0x8000_0000;
    push_case(
        "muladd_x_two_imms_negative_product_as_sub_imm_preserves_flags",
        enc_addsub_imm_regs(1, 1, 0, 0, 15, RD, 3),
        vec![OpKind::MulAdd {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: VReg::Imm(-3),
            src2: VReg::Imm(5),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x1122_3344_5566_7788;
    st.x[3] = 0x0102_0304_0506_0708;
    st.pstate = 0x3000_0000;
    push_case(
        "muladd_x_imm_neg_one_as_sub_preserves_flags",
        enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 3, RN),
        vec![OpKind::MulAdd {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: VReg::Imm(-1),
            src2: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_ffff_fffd;
    st.x[2] = 7;
    st.x[3] = 0x20;
    st.pstate = 0x6000_0000;
    push_case(
        "msub_w_opkind_zero_ext_preserves_flags",
        enc_dp3(0, 0b000, 1),
        vec![OpKind::MulSub {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: arm_x(1),
            src2: arm_x(2),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x7777_8888_1234_fffe;
    st.x[3] = 0x3333_4444_5555_beef;
    st.pstate = 0x5000_0000;
    push_case(
        "mulsub_w16_imm_masked_zero_as_acc_uxth_preserves_flags",
        enc_bitfield_regs(0, 0b10, 0, 15, 3, RD),
        vec![OpKind::MulSub {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: arm_x(1),
            src2: VReg::Imm(0x1_0000),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_eeee_dddd_cccc;
    st.x[3] = 0x3333_4444_5555_12ab;
    st.pstate = 0x9000_0000;
    push_case(
        "mulsub_w8_two_imms_wrapping_zero_product_as_acc_uxtb_preserves_flags",
        enc_bitfield_regs(0, 0b10, 0, 7, 3, RD),
        vec![OpKind::MulSub {
            dst: arm_x(0),
            acc: arm_x(3),
            src1: VReg::Imm(16),
            src2: VReg::Imm(16),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0x7777_8888_1234_0007;
    st.x[3] = 0x3333_4444_5555_0100;
    st.pstate = 0xa000_0000;
    drop(push_case);
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulSub {
        dst: arm_x(0),
        acc: arm_x(3),
        src1: arm_x(1),
        src2: VReg::Imm(0x1_0004),
        width: OpWidth::W16,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "mulsub_w16_imm_masked_power_of_two_as_sub_shifted_uxth_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "mulsub_w16_imm_masked_power_of_two_as_sub_shifted_uxth_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 2, RD, 3, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut push_case = |label: &str, source: u32, ops: Vec<OpKind>, st: ArmState| {
        let lowered = lower_aarch64_native_ops(ops)
            .unwrap_or_else(|e| panic!("{label}: native lowering failed: {e}"));
        cases.push((label.into(), [source, NOP, NOP], lowered, st));
    };

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 3;
    st.pstate = 0x1000_0000;
    push_case(
        "smulh_x_opkind_preserves_flags",
        enc_dp3_ra(1, 0b010, 0, 31),
        vec![OpKind::MulS {
            dst_lo: VReg::virt(0),
            dst_hi: Some(arm_x(0)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ff00;
    st.x[2] = 0x100;
    st.pstate = 0x3000_0000;
    push_case(
        "umulh_x_opkind_preserves_flags",
        enc_dp3_ra(1, 0b110, 0, 31),
        vec![OpKind::MulU {
            dst_lo: VReg::virt(1),
            dst_hi: Some(arm_x(0)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.x[2] = 0x12345;
    st.pstate = 0x5000_0000;
    push_case(
        "udiv_x_opkind_preserves_flags",
        enc_dp2(1, 0b0010),
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0xffff_ffff_8000_0100;
    st.x[2] = 0xffff_ffff_ffff_fffb;
    st.pstate = 0x7000_0000;
    push_case(
        "sdiv_w_opkind_zero_ext_preserves_flags",
        enc_dp2(0, 0b0011),
        vec![OpKind::DivS {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0x3000_0000;
    push_case(
        "divu_x_imm_one_as_mov_preserves_flags",
        enc_mov_reg(1, RD, RN),
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0x8000_0000_0000_0000;
    st.pstate = 0x9000_0000;
    push_case(
        "divs_x_imm_neg_one_as_neg_preserves_flags",
        enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RN),
        vec![OpKind::DivS {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(-1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0x6000_0000;
    push_case(
        "divs_w_imm_masked_neg_one_as_neg_zero_ext_preserves_flags",
        enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, RN),
        vec![OpKind::DivS {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_ffff_ffff),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0x5000_0000;
    push_case(
        "divu_x_imm_power_of_two_as_lsr_preserves_flags",
        enc_bitfield(1, 0b10, 3, 63),
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.pstate = 0xb000_0000;
    push_case(
        "divu_w_imm_masked_power_of_two_as_lsr_preserves_flags",
        enc_bitfield(0, 0b10, 31, 31),
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_8000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0xffff_ffff_ffff_00fc;
    st.pstate = 0x2000_0000;
    push_case(
        "divu_w8_imm_power_of_two_as_lsr_uxtb_preserves_flags",
        enc_bitfield(0, 0b10, 2, 7),
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(4),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.pstate = 0xa000_0000;
    push_case(
        "divu_w8_two_masked_imms_as_mov_quot_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 5),
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: VReg::Imm(0x123),
            src2: SrcOperand::Imm(6),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0xffff_ffff_ffff_8001;
    st.pstate = 0xc000_0000;
    push_case(
        "divu_w16_imm_masked_power_of_two_as_lsr_uxth_preserves_flags",
        enc_bitfield(0, 0b10, 15, 15),
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_0000_8000),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0x8877_6655_4433_2211;
    st.pstate = 0x6000_0000;
    push_case(
        "ldr_x_base_offset_opkind_preserves_flags",
        enc_ldst_uimm(3, 0, 1, 1),
        vec![OpKind::Load {
            dst: arm_x(0),
            addr: native_base_offset_addr.clone(),
            width: MemWidth::B8,
            sign: SignExtend::Zero,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0xffff_ffff_ffff_ff80;
    st.pstate = 0x3000_0000;
    push_case(
        "ldrb_direct_opkind_zero_ext_preserves_flags",
        enc_ldst_uimm(0, 0, 1, 0),
        vec![OpKind::Load {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            width: MemWidth::B1,
            sign: SignExtend::Zero,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0x0000_0000_8000_0001;
    st.pstate = 0x9000_0000;
    push_case(
        "ldrsw_direct_opkind_sign_ext_preserves_flags",
        enc_ldst_uimm(2, 0, 2, 0),
        vec![OpKind::Load {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            width: MemWidth::B4,
            sign: SignExtend::Sign,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0;
    st.pstate = 0x5000_0000;
    push_case(
        "str_x_base_offset_opkind_preserves_flags_and_memory",
        enc_ldst_uimm(3, 0, 0, 1),
        vec![OpKind::Store {
            src: arm_x(0),
            addr: native_base_offset_addr.clone(),
            width: MemWidth::B8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_44aa;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0xffff_ffff_ffff_5500;
    st.pstate = 0xa000_0000;
    push_case(
        "strb_direct_opkind_preserves_flags_and_memory",
        enc_ldst_uimm(0, 0, 0, 0),
        vec![OpKind::Store {
            src: arm_x(0),
            addr: native_direct_addr.clone(),
            width: MemWidth::B1,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 1;
    st.scratch[9] = 0x0123_4567_89ab_cdef;
    st.pstate = 0xe000_0000;
    push_case(
        "ldr_x_base_index_scale_opkind_preserves_flags",
        enc_ldst_reg(3, 1, RM, 0b011, 1),
        vec![OpKind::Load {
            dst: arm_x(0),
            addr: Address::BaseIndexScale {
                base: Some(arm_x(1)),
                index: arm_x(2),
                scale: 8,
                disp: 0,
                disp_size: DispSize::Auto,
            },
            width: MemWidth::B8,
            sign: SignExtend::Zero,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_aabb_ccdd;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 8;
    st.scratch[9] = 0x1122_3344_5566_7788;
    st.pstate = 0x1000_0000;
    push_case(
        "str_w_base_index_scale_opkind_preserves_flags_and_memory",
        enc_ldst_reg(2, 0, RM, 0b011, 0),
        vec![OpKind::Store {
            src: arm_x(0),
            addr: Address::BaseIndexScale {
                base: Some(arm_x(1)),
                index: arm_x(2),
                scale: 1,
                disp: 0,
                disp_size: DispSize::Auto,
            },
            width: MemWidth::B4,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0xc000_0000;
    push_case(
        "atomic_load_acquire_x_direct_opkind_preserves_flags",
        enc_ldar(3),
        vec![OpKind::AtomicLoad {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            width: MemWidth::B8,
            order: MemoryOrder::Acquire,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0xffff_ffff_8000_0001;
    st.pstate = 0x1000_0000;
    push_case(
        "atomic_load_relaxed_w_base_offset_opkind_zero_ext_preserves_flags",
        enc_ldst_uimm(2, 0, 1, 2),
        vec![OpKind::AtomicLoad {
            dst: arm_x(0),
            addr: native_base_offset_addr.clone(),
            width: MemWidth::B4,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = SCRATCH_BASE;
    st.x[3] = 0xffff_ffff_aabb_ccdd;
    st.scratch[8] = 0x1122_3344_5566_7788;
    st.pstate = 0x5000_0000;
    push_case(
        "atomic_store_release_w_direct_opkind_preserves_flags_and_memory",
        enc_stlr(2),
        vec![OpKind::AtomicStore {
            src: arm_x(3),
            addr: native_direct_addr.clone(),
            width: MemWidth::B4,
            order: MemoryOrder::Release,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0102_0304_0506_0708;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0x9999_aaaa_bbbb_cccc;
    st.pstate = 0x6000_0000;
    push_case(
        "atomic_store_relaxed_x_base_offset_opkind_preserves_flags_and_memory",
        enc_ldst_uimm(3, 0, 0, 1),
        vec![OpKind::AtomicStore {
            src: arm_x(0),
            addr: native_base_offset_addr.clone(),
            width: MemWidth::B8,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x2000_0000;
    push_case(
        "ldxr_x_direct_opkind_preserves_flags",
        enc_ldxr(3, 0),
        vec![OpKind::LoadExclusive {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            width: MemWidth::B8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xffff_ffff_ffff_ffff;
    st.x[3] = 0x0102_0304_0506_0708;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0x8000_0000;
    push_case(
        "stxr_x_direct_opkind_fails_without_monitor_preserves_flags_and_memory",
        enc_stxr(3, 0),
        vec![OpKind::StoreExclusive {
            status: arm_x(2),
            src: arm_x(3),
            addr: native_direct_addr.clone(),
            width: MemWidth::B8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0102_0304_0506_0708;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0x4000_0000;
    push_case(
        "swp_x_direct_opkind_preserves_flags_and_memory",
        enc_atomic_smir(3, 0, 0, 1, 0b000, 2, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: arm_x(2),
            op: AtomicOp::Swap,
            width: MemWidth::B8,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x7777_8888_9999_aaaa;
    st.scratch[8] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xe000_0000;
    push_case(
        "atomic_and_zero_x_direct_opkind_as_swp_zero_preserves_flags_and_memory",
        enc_atomic_smir(3, 0, 0, 1, 0b000, 31, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: VReg::Imm(0),
            op: AtomicOp::And,
            width: MemWidth::B8,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1020_3040_5060_7080;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xffff_ffff_ffff_fffb;
    st.scratch[8] = 0x1357_9bdf_2468_ace0;
    st.pstate = 0x6000_0000;
    push_case(
        "atomic_sub_zero_x_direct_opkind_as_ldadd_zero_preserves_flags_and_memory",
        enc_atomic_smir(3, 0, 0, 0, 0b000, 31, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: VReg::Imm(0),
            op: AtomicOp::Sub,
            width: MemWidth::B8,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8899_aabb_ccdd_eeff;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x1234_5678_9abc_def0;
    st.scratch[8] = 0x0246_8ace_1357_9bdf;
    st.pstate = 0xb000_0000;
    push_case(
        "atomic_and_all_ones_x_direct_opkind_as_ldclr_zero_preserves_flags_and_memory",
        enc_atomic_smir(3, 0, 0, 0, 0b001, 31, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: VReg::Imm(-1),
            op: AtomicOp::And,
            width: MemWidth::B8,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_aaaa_bbbb;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_0007;
    st.scratch[8] = 0x1122_3344_ffff_fffe;
    st.pstate = 0x1000_0000;
    push_case(
        "ldadd_w_direct_opkind_acqrel_zero_ext_preserves_flags_and_memory",
        enc_atomic_smir(2, 1, 1, 0, 0b000, 2, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: arm_x(2),
            op: AtomicOp::Add,
            width: MemWidth::B4,
            order: MemoryOrder::AcqRel,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0101_0101_0101_0101;
    st.scratch[8] = 0xf0f0_0f0f_aaaa_5555;
    st.pstate = 0x5000_0000;
    push_case(
        "ldeor_x_direct_opkind_preserves_flags_and_memory",
        enc_atomic_smir(3, 0, 0, 0, 0b010, 2, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: arm_x(2),
            op: AtomicOp::Xor,
            width: MemWidth::B8,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_00f0;
    st.scratch[8] = 0xaaaa_bbbb_ccdd_000f;
    st.pstate = 0x6000_0000;
    push_case(
        "ldset_h_direct_opkind_preserves_flags_and_memory",
        enc_atomic_smir(1, 0, 1, 0, 0b011, 2, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: arm_x(2),
            op: AtomicOp::Or,
            width: MemWidth::B2,
            order: MemoryOrder::Release,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_0007;
    st.scratch[8] = 0xaaaa_bbbb_ffff_fff0;
    st.pstate = 0x3000_0000;
    push_case(
        "ldsmax_w_direct_opkind_uses_signed_width_preserves_flags_and_memory",
        enc_atomic_smir(2, 1, 0, 0, 0b100, 2, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: arm_x(2),
            op: AtomicOp::Max,
            width: MemWidth::B4,
            order: MemoryOrder::Acquire,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xffff_ffff_ffff_fff0;
    st.scratch[8] = 0x1111_2222_0000_0007;
    st.pstate = 0xa000_0000;
    push_case(
        "ldsmin_w_direct_opkind_uses_signed_width_preserves_flags_and_memory",
        enc_atomic_smir(2, 0, 0, 0, 0b101, 2, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: arm_x(2),
            op: AtomicOp::Min,
            width: MemWidth::B4,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_ffff_ffff;
    st.scratch[8] = 0xaaaa_bbbb_0000_0001;
    st.pstate = 0x2000_0000;
    push_case(
        "ldumax_w_direct_opkind_uses_unsigned_width_preserves_flags_and_memory",
        enc_atomic_smir(2, 1, 1, 0, 0b110, 2, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: arm_x(2),
            op: AtomicOp::Umax,
            width: MemWidth::B4,
            order: MemoryOrder::SeqCst,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_0012;
    st.scratch[8] = 0x3333_4444_5555_6680;
    st.pstate = 0x8000_0000;
    push_case(
        "ldumin_b_direct_opkind_uses_unsigned_width_preserves_flags_and_memory",
        enc_atomic_smir(0, 0, 0, 0, 0b111, 2, RN, RD),
        vec![OpKind::AtomicRmw {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            src: arm_x(2),
            op: AtomicOp::Umin,
            width: MemWidth::B1,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0102_0304_0506_0708;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x8877_6655_4433_2211;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0x7000_0000;
    push_case(
        "cas_x_direct_opkind_success_preserves_flags_and_memory",
        enc_cas(3, 0, 0),
        vec![OpKind::Cas {
            dst: arm_x(2),
            success: VReg::virt(0),
            addr: native_direct_addr.clone(),
            expected: arm_x(2),
            new_val: arm_x(0),
            width: MemWidth::B8,
            order: MemoryOrder::Relaxed,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0000_0000_aaaa_bbbb;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_1111_2222;
    st.scratch[8] = 0xcccc_dddd_3333_4444;
    st.pstate = 0xd000_0000;
    push_case(
        "cas_w_direct_opkind_failure_zero_ext_preserves_flags_and_memory",
        enc_cas(2, 1, 1),
        vec![OpKind::Cas {
            dst: arm_x(2),
            success: VReg::virt(0),
            addr: native_direct_addr.clone(),
            expected: arm_x(2),
            new_val: arm_x(0),
            width: MemWidth::B4,
            order: MemoryOrder::AcqRel,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x3333_4444_5555_6666;
    st.scratch[10] = 0x0123_4567_89ab_cdef;
    st.scratch[11] = 0xfedc_ba98_7654_3210;
    st.pstate = 0x7000_0000;
    push_case(
        "ldp_x_base_offset_opkind_preserves_flags",
        enc_ldp(0b10, 0, 0b10, 1, 2),
        vec![OpKind::LoadPair {
            dst1: arm_x(0),
            dst2: arm_x(2),
            addr: native_pair_x_addr.clone(),
            width: MemWidth::B8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x5555_6666_7777_8888;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0x3000_0000;
    push_case(
        "ldp_w_direct_opkind_zero_ext_preserves_flags",
        enc_ldp(0b00, 0, 0b10, 1, 0),
        vec![OpKind::LoadPair {
            dst1: arm_x(0),
            dst2: arm_x(2),
            addr: native_direct_addr.clone(),
            width: MemWidth::B4,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xfedc_ba98_7654_3210;
    st.scratch[10] = 0;
    st.scratch[11] = 0;
    st.pstate = 0x5000_0000;
    push_case(
        "stp_x_base_offset_opkind_preserves_flags_and_memory",
        enc_ldp(0b10, 0, 0b10, 0, 2),
        vec![OpKind::StorePair {
            src1: arm_x(0),
            src2: arm_x(2),
            addr: native_pair_x_addr.clone(),
            width: MemWidth::B8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0123_4567_7654_3210;
    st.scratch[8] = 0;
    st.pstate = 0xa000_0000;
    push_case(
        "stp_w_direct_opkind_preserves_flags_and_memory",
        enc_ldp(0b00, 0, 0b10, 0, 0),
        vec![OpKind::StorePair {
            src1: arm_x(0),
            src2: arm_x(2),
            addr: native_direct_addr.clone(),
            width: MemWidth::B4,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0xa000_0000;
    push_case(
        "clrex_opkind_preserves_arch_state",
        enc_clrex(),
        vec![OpKind::ClearExclusive],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0x5000_0000;
    push_case(
        "fence_full_opkind_preserves_arch_state",
        enc_barrier(0b100),
        vec![OpKind::Fence {
            kind: FenceKind::Full,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.pstate = 0x9000_0000;
    push_case(
        "fence_isync_opkind_preserves_arch_state",
        enc_barrier(0b110),
        vec![OpKind::Fence {
            kind: FenceKind::ISync,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0x8888_9999_aaaa_bbbb;
    st.pstate = 0xf000_0000;
    push_case(
        "materialize_flags_opkind_preserves_arch_state",
        NOP,
        vec![OpKind::MaterializeFlags],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = SCRATCH_BASE;
    st.pstate = 0x6000_0000;
    push_case(
        "prefetch_read_opkind_preserves_arch_state",
        enc_ldst_uimm(3, 0, 2, 0),
        vec![OpKind::Prefetch {
            addr: Address::Direct(arm_x(1)),
            write: false,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0;
    st.pstate = 0x7000_0000;
    push_case(
        "prefetch_write_opkind_preserves_arch_state",
        enc_prfm_reg(0b10101, RM, 0b111, 0),
        vec![OpKind::Prefetch {
            addr: Address::Direct(arm_x(1)),
            write: true,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = SCRATCH_BASE;
    st.pstate = 0x8000_0000;
    push_case(
        "prefetch_read_base_offset_opkind_preserves_arch_state",
        enc_ldst_uimm(3, 0, 2, 3),
        vec![OpKind::Prefetch {
            addr: Address::BaseOffset {
                base: arm_x(1),
                offset: 24,
                disp_size: DispSize::Auto,
            },
            write: false,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0xb000_0000;
    push_case(
        "prefetch_read_pcrel_opkind_preserves_arch_state",
        enc_prfm_lit(0, 3),
        vec![OpKind::Prefetch {
            addr: Address::PcRel {
                offset: 12,
                disp_size: DispSize::Auto,
                base: None,
            },
            write: false,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 1;
    st.pstate = 0x2000_0000;
    push_case(
        "prefetch_write_base_index_scale_opkind_preserves_arch_state",
        enc_prfm_reg(0b10000, RM, 0b011, 1),
        vec![OpKind::Prefetch {
            addr: Address::BaseIndexScale {
                base: Some(arm_x(1)),
                index: arm_x(2),
                scale: 8,
                disp: 0,
                disp_size: DispSize::Auto,
            },
            write: true,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x9000_0000;
    push_case(
        "mrs_nzcv_opkind_reads_flags",
        enc_mrs_nzcv(RD),
        vec![OpKind::Mov {
            dst: arm_x(0),
            src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Nzcv))),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0xa000_0000;
    push_case(
        "read_sysreg_nzcv_direct_opkind_matches_mrs",
        enc_mrs_nzcv(RD),
        vec![OpKind::ReadSysReg {
            dst: arm_x(0),
            reg: SYSREG_NZCV_RAW,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_1234_5678;
    st.pstate = 0xf000_0000;
    push_case(
        "msr_nzcv_opkind_masks_x1",
        enc_msr_nzcv(RN),
        vec![OpKind::Mov {
            dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
            src: SrcOperand::Reg(arm_x(1)),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xb000_0000;
    st.pstate = 0x4000_0000;
    push_case(
        "write_sysreg_nzcv_direct_opkind_matches_msr",
        enc_msr_nzcv(RN),
        vec![OpKind::WriteSysReg {
            reg: SYSREG_NZCV_RAW,
            src: arm_x(1),
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0xf000_0000;
    push_case(
        "msr_nzcv_xzr_opkind_clears",
        enc_msr_nzcv(31),
        vec![OpKind::Mov {
            dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
            src: SrcOperand::Imm(0),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.fpcr = 0x00c0_0000;
    push_case(
        "mrs_fpcr_opkind_reads_control",
        enc_mrs_fpcr(RD),
        vec![OpKind::Mov {
            dst: arm_x(0),
            src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Fpcr))),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_00c0_0000;
    st.fpcr = 0;
    push_case(
        "msr_fpcr_opkind_masks_x1",
        enc_msr_fpcr(RN),
        vec![OpKind::Mov {
            dst: VReg::Arch(ArchReg::Arm(ArmReg::Fpcr)),
            src: SrcOperand::Reg(arm_x(1)),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.fpsr = 0x0800_009f;
    push_case(
        "mrs_fpsr_opkind_reads_status",
        enc_mrs_fpsr(RD),
        vec![OpKind::Mov {
            dst: arm_x(0),
            src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Fpsr))),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_0800_009f;
    st.fpsr = 0;
    push_case(
        "msr_fpsr_opkind_masks_x1",
        enc_msr_fpsr(RN),
        vec![OpKind::Mov {
            dst: VReg::Arch(ArchReg::Arm(ArmReg::Fpsr)),
            src: SrcOperand::Reg(arm_x(1)),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.pstate = 0;
    push_case(
        "cfinv_carry_clear_opkind_sets_c",
        enc_cfinv(),
        vec![OpKind::Xor {
            dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
            src1: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
            src2: SrcOperand::Imm(0x2000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.pstate = 0x2000_0000;
    push_case(
        "cfinv_carry_set_opkind_clears_c",
        enc_cfinv(),
        vec![OpKind::Xor {
            dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
            src1: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
            src2: SrcOperand::Imm(0x2000_0000),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.pstate = 0;
    push_case(
        "cmc_cf_carry_clear_opkind_sets_c",
        enc_cfinv(),
        vec![OpKind::CmcCF],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.pstate = 0x2000_0000;
    push_case(
        "cmc_cf_carry_set_opkind_clears_c",
        enc_cfinv(),
        vec![OpKind::CmcCF],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 5;
    st.x[2] = 7;
    st.pstate = 0x4000_0000;
    push_case(
        "ccmp_x_eq_true_opkind_sets_flags",
        enc_condcmp(1, 1, false, RM, 0, 0),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(190),
                cond: Condition::Eq,
            },
            OpKind::Sub {
                dst: VReg::virt(191),
                src1: arm_x(1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            OpKind::Mov {
                dst: VReg::virt(192),
                src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Nzcv))),
                width: OpWidth::W32,
            },
            OpKind::Select {
                dst: VReg::virt(193),
                cond: VReg::virt(190),
                src_true: VReg::virt(192),
                src_false: VReg::Imm(0),
                width: OpWidth::W32,
            },
            OpKind::Mov {
                dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
                src: SrcOperand::Reg(VReg::virt(193)),
                width: OpWidth::W32,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 5;
    st.x[2] = 7;
    st.pstate = 0;
    push_case(
        "ccmp_x_eq_false_opkind_fallback_z",
        enc_condcmp(1, 1, false, RM, 0, 0b0100),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(194),
                cond: Condition::Eq,
            },
            OpKind::Sub {
                dst: VReg::virt(195),
                src1: arm_x(1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W64,
                flags: FlagUpdate::All,
            },
            OpKind::Mov {
                dst: VReg::virt(196),
                src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Nzcv))),
                width: OpWidth::W32,
            },
            OpKind::Select {
                dst: VReg::virt(197),
                cond: VReg::virt(194),
                src_true: VReg::virt(196),
                src_false: VReg::Imm(0x4000_0000),
                width: OpWidth::W32,
            },
            OpKind::Mov {
                dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
                src: SrcOperand::Reg(VReg::virt(197)),
                width: OpWidth::W32,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff;
    st.x[2] = 1;
    st.pstate = 0x4000_0000;
    push_case(
        "ccmn_w_ne_false_opkind_fallback_nv",
        enc_condcmp(0, 0, false, RM, 1, 0b1001),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(198),
                cond: Condition::Ne,
            },
            OpKind::Add {
                dst: VReg::virt(199),
                src1: arm_x(1),
                src2: SrcOperand::Reg(arm_x(2)),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            OpKind::Mov {
                dst: VReg::virt(209),
                src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Nzcv))),
                width: OpWidth::W32,
            },
            OpKind::Select {
                dst: VReg::virt(210),
                cond: VReg::virt(198),
                src_true: VReg::virt(209),
                src_false: VReg::Imm(0x9000_0000),
                width: OpWidth::W32,
            },
            OpKind::Mov {
                dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
                src: SrcOperand::Reg(VReg::virt(210)),
                width: OpWidth::W32,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0x10;
    st.pstate = 0x2000_0000;
    push_case(
        "ccmp_w_imm_hi_true_opkind_sets_flags",
        enc_condcmp(0, 1, true, 5, 8, 0),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(211),
                cond: Condition::Ugt,
            },
            OpKind::Sub {
                dst: VReg::virt(212),
                src1: arm_x(1),
                src2: SrcOperand::Imm(5),
                width: OpWidth::W32,
                flags: FlagUpdate::All,
            },
            OpKind::Mov {
                dst: VReg::virt(213),
                src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Nzcv))),
                width: OpWidth::W32,
            },
            OpKind::Select {
                dst: VReg::virt(214),
                cond: VReg::virt(211),
                src_true: VReg::virt(213),
                src_false: VReg::Imm(0),
                width: OpWidth::W32,
            },
            OpKind::Mov {
                dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
                src: SrcOperand::Reg(VReg::virt(214)),
                width: OpWidth::W32,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 0x5555_6666_7777_8888;
    st.pstate = 0x4000_0000;
    push_case(
        "csel_x_eq_opkind_true_preserves_flags",
        enc_csel_form(1, 0, 0, RN, RM, 0),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(200),
                cond: Condition::Eq,
            },
            OpKind::Select {
                dst: arm_x(0),
                cond: VReg::virt(200),
                src_true: arm_x(1),
                src_false: arm_x(2),
                width: OpWidth::W64,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = u64::MAX;
    st.pstate = 0;
    push_case(
        "csinc_x_eq_opkind_false_preserves_flags",
        enc_csel_form(1, 0, 1, RN, RM, 0),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(201),
                cond: Condition::Eq,
            },
            OpKind::Add {
                dst: VReg::virt(202),
                src1: arm_x(2),
                src2: SrcOperand::Imm(1),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            OpKind::Select {
                dst: arm_x(0),
                cond: VReg::virt(201),
                src_true: arm_x(1),
                src_false: VReg::virt(202),
                width: OpWidth::W64,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 0xffff_ffff_1234_5678;
    st.pstate = 0x4000_0000;
    push_case(
        "csinv_w_ne_opkind_false_zero_ext_preserves_flags",
        enc_csel_form(0, 1, 0, RN, RM, 1),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(203),
                cond: Condition::Ne,
            },
            OpKind::Not {
                dst: VReg::virt(204),
                src: arm_x(2),
                width: OpWidth::W32,
            },
            OpKind::Select {
                dst: arm_x(0),
                cond: VReg::virt(203),
                src_true: arm_x(1),
                src_false: VReg::virt(204),
                width: OpWidth::W32,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 5;
    st.pstate = 0;
    push_case(
        "csneg_x_hi_opkind_false_preserves_flags",
        enc_csel_form(1, 1, 1, RN, RM, 8),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(205),
                cond: Condition::Ugt,
            },
            OpKind::Neg {
                dst: VReg::virt(206),
                src: arm_x(2),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
            OpKind::Select {
                dst: arm_x(0),
                cond: VReg::virt(205),
                src_true: arm_x(1),
                src_false: VReg::virt(206),
                width: OpWidth::W64,
            },
        ],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x4000_0000;
    push_case(
        "test_condition_eq_opkind_materializes_bool",
        enc_csel_form(1, 0, 1, 31, 31, 1),
        vec![OpKind::TestCondition {
            dst: arm_x(0),
            cond: Condition::Eq,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0x4000_0000;
    push_case(
        "cmove_x_eq_true_opkind_selects_src_preserves_flags",
        enc_csel_form(1, 0, 0, RN, RD, 0),
        vec![OpKind::CMove {
            dst: arm_x(0),
            src: arm_x(1),
            cond: Condition::Eq,
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x2222_3333_4444_5555;
    st.pstate = 0;
    push_case(
        "cmove_x_eq_false_opkind_preserves_dst_and_flags",
        enc_csel_form(1, 0, 0, RN, RD, 0),
        vec![OpKind::CMove {
            dst: arm_x(0),
            src: arm_x(1),
            cond: Condition::Eq,
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x9999_aaaa_1234_5678;
    st.pstate = 0x4000_0000;
    push_case(
        "cmove_w_eq_true_opkind_selects_src_zero_ext_preserves_flags",
        enc_csel_form(0, 0, 0, RN, RD, 0),
        vec![OpKind::CMove {
            dst: arm_x(0),
            src: arm_x(1),
            cond: Condition::Eq,
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_abcd_1234;
    st.x[1] = 0x8888_9999_5678_9abc;
    st.pstate = 0;
    push_case(
        "cmove_w_eq_false_opkind_preserves_dst_zero_ext_and_flags",
        enc_csel_form(0, 0, 0, RN, RD, 0),
        vec![OpKind::CMove {
            dst: arm_x(0),
            src: arm_x(1),
            cond: Condition::Eq,
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_44f0;
    st.pstate = 0xf000_0000;
    push_case(
        "truncate_x_to_w8_as_ubfx_preserves_flags",
        enc_bitfield_rn(1, 0b10, 0, 7, RN),
        vec![OpKind::Truncate {
            dst: arm_x(0),
            src: arm_x(1),
            from_width: OpWidth::W64,
            to_width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x2222_3333_4444_8ace;
    st.pstate = 0x6000_0000;
    push_case(
        "truncate_x_to_w16_as_ubfx_preserves_flags",
        enc_bitfield_rn(1, 0b10, 0, 15, RN),
        vec![OpKind::Truncate {
            dst: arm_x(0),
            src: arm_x(1),
            from_width: OpWidth::W64,
            to_width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x3333_4444_8765_4321;
    st.pstate = 0x9000_0000;
    push_case(
        "truncate_x_to_w32_as_w_mov_zero_ext_preserves_flags",
        enc_logical_shift_regs(0, 0b01, 0, 0, 0, RD, 31, RN),
        vec![OpKind::Truncate {
            dst: arm_x(0),
            src: arm_x(1),
            from_width: OpWidth::W64,
            to_width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0xc000_0000;
    push_case(
        "truncate_imm_to_w8_as_movz_masked_preserves_flags",
        enc_mov_wide(0, 0b10, 0, 0xef),
        vec![OpKind::Truncate {
            dst: arm_x(0),
            src: VReg::Imm(0x1234_56ef),
            from_width: OpWidth::W64,
            to_width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x3000_0000;
    push_case(
        "truncate_imm_to_x_as_movz_preserves_flags",
        enc_mov_wide(1, 0b10, 0, 0x2468),
        vec![OpKind::Truncate {
            dst: arm_x(0),
            src: VReg::Imm(0x2468),
            from_width: OpWidth::W64,
            to_width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.pstate = 0;
    push_case(
        "setcc_ne_w8_true_opkind_materializes_bool_preserves_flags",
        enc_csel_form(1, 0, 1, 31, 31, 0),
        vec![OpKind::SetCC {
            dst: arm_x(0),
            cond: Condition::Ne,
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.pstate = 0x4000_0000;
    push_case(
        "setcc_ne_w8_false_opkind_materializes_zero_preserves_flags",
        enc_csel_form(1, 0, 1, 31, 31, 0),
        vec![OpKind::SetCC {
            dst: arm_x(0),
            cond: Condition::Ne,
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.pstate = 0x4000_0000;
    push_case(
        "csetm_w_raw_ne_opkind_false_zero_ext_preserves_flags",
        enc_csel_form(0, 1, 0, 31, 31, 1),
        vec![
            OpKind::TestCondition {
                dst: VReg::virt(207),
                cond: Condition::Ne,
            },
            OpKind::Not {
                dst: VReg::virt(208),
                src: VReg::Imm(0),
                width: OpWidth::W32,
            },
            OpKind::Select {
                dst: arm_x(0),
                cond: VReg::virt(207),
                src_true: VReg::Imm(0),
                src_false: VReg::virt(208),
                width: OpWidth::W32,
            },
        ],
        st,
    );

    drop(push_case);

    push_select_identical_arm_native_cases(&mut cases, control_target);
    push_select_dst_arm_native_cases(&mut cases, control_target);
    push_cond_select_true_transform_native_cases(&mut cases, control_target);
    push_flagm_masked_imm_native_cases(&mut cases, control_target);
    push_sysreg_masked_mask_native_cases(&mut cases, control_target);
    push_flagm_masked_shift_native_cases(&mut cases, control_target);
    push_cls_masked_imm_native_cases(&mut cases, control_target);
    push_cond_compare_inverted_native_cases(&mut cases, control_target);
    push_sar_imm_movn_native_cases(&mut cases, control_target);
    push_ror_imm_movn_native_cases(&mut cases, control_target);
    push_rol_imm_movn_native_cases(&mut cases, control_target);
    push_shl_imm_movn_native_cases(&mut cases, control_target);
    push_shr_imm_movn_native_cases(&mut cases, control_target);
    push_rbit_imm_movn_native_cases(&mut cases, control_target);
    push_bswap_imm_movn_native_cases(&mut cases, control_target);
    push_extract_masked_shift_native_cases(&mut cases, control_target);
    push_bitfield_insert_zero_masked_shift_native_cases(&mut cases, control_target);
    push_rev16_imm_movn_native_cases(&mut cases, control_target);
    push_rev32_imm_movn_native_cases(&mut cases, control_target);
    push_truncate_imm_movn_native_cases(&mut cases, control_target);
    push_divu_imm_movn_native_cases(&mut cases, control_target);
    push_divs_imm_movn_native_cases(&mut cases, control_target);
    push_div_one_same_reg_native_cases(&mut cases, control_target);
    push_divs_neg_one_subword_native_cases(&mut cases, control_target);
    push_divs_neg_one_remainder_native_cases(&mut cases, control_target);
    push_divs_power_of_two_alias_native_cases(&mut cases, control_target);
    push_mov_imm_movn_native_cases(&mut cases, control_target);
    push_mov_same_reg_native_cases(&mut cases, control_target);
    push_extend_same_width_native_cases(&mut cases, control_target);
    push_truncate_same_reg_native_cases(&mut cases, control_target);
    push_xchg_same_reg_native_cases(&mut cases, control_target);
    push_logical_identity_same_reg_native_cases(&mut cases, control_target);
    push_logical_zero_base_reg_native_cases(&mut cases, control_target);
    push_logical_zero_base_extended_native_cases(&mut cases, control_target);
    push_test_zero_imm_operand_native_cases(&mut cases, control_target);
    push_materialized_logical_imm_native_cases(&mut cases, control_target);
    push_sparse_logical_imm_in_place_native_cases(&mut cases, control_target);
    push_and_all_ones_left_imm_reg_native_cases(&mut cases, control_target);
    push_andnot_all_ones_left_imm_reg_native_cases(&mut cases, control_target);
    push_andnot_all_ones_left_shifted_native_cases(&mut cases, control_target);
    push_andnot_all_ones_left_extended_native_cases(&mut cases, control_target);
    push_or_all_ones_left_imm_reg_native_cases(&mut cases, control_target);
    push_or_all_ones_left_shifted_native_cases(&mut cases, control_target);
    push_or_all_ones_left_extended_native_cases(&mut cases, control_target);
    push_xor_all_ones_left_imm_reg_native_cases(&mut cases, control_target);
    push_xor_all_ones_left_shifted_native_cases(&mut cases, control_target);
    push_xor_all_ones_left_extended_native_cases(&mut cases, control_target);
    push_and_all_ones_left_shifted_native_cases(&mut cases, control_target);
    push_and_all_ones_left_extended_native_cases(&mut cases, control_target);
    push_test_all_ones_left_imm_native_cases(&mut cases, control_target);
    push_test_all_ones_left_shifted_native_cases(&mut cases, control_target);
    push_test_all_ones_left_extended_native_cases(&mut cases, control_target);
    push_logical_same_source_zero_native_cases(&mut cases, control_target);
    push_subword_logical_flag_native_cases(&mut cases, control_target);
    push_or_xor_flag_native_cases(&mut cases, control_target);
    push_addsub_zero_same_reg_native_cases(&mut cases, control_target);
    push_addsub_masked_zero_imm_native_cases(&mut cases, control_target);
    push_addsub_zero_base_nonzero_imm_flag_native_cases(&mut cases, control_target);
    push_cmp_zero_base_neg_imm_native_cases(&mut cases, control_target);
    push_cmp_two_imm_native_cases(&mut cases, control_target);
    push_cmp_zero_base_reg_native_cases(&mut cases, control_target);
    push_addsub_carry_nonzero_imm_native_cases(&mut cases, control_target);
    push_addsub_carry_zero_base_native_cases(&mut cases, control_target);
    push_add_zero_base_reg_native_cases(&mut cases, control_target);
    push_addsub_zero_base_reg_operand_native_cases(&mut cases, control_target);
    push_subword_addsub_zero_base_reg_operand_native_cases(&mut cases, control_target);
    push_subword_addsub_reg_operand_native_cases(&mut cases, control_target);
    push_addsub_effective_zero_ror_native_cases(&mut cases, control_target);
    push_shift_zero_same_reg_native_cases(&mut cases, control_target);
    push_carry_rotate_zero_count_native_cases(&mut cases, control_target);
    push_shift_zero_count_reg_native_cases(&mut cases, control_target);
    push_shift_zero_imm_source_reg_count_native_cases(&mut cases, control_target);
    push_shift_all_ones_imm_source_reg_count_native_cases(&mut cases, control_target);
    push_rol_constant_imm_source_reg_count_native_cases(&mut cases, control_target);
    push_rol_zero_count_reg_native_cases(&mut cases, control_target);
    push_double_shift_zero_count_native_cases(&mut cases, control_target);
    push_mul_one_same_reg_native_cases(&mut cases, control_target);
    push_mul_zero_source_reg_native_cases(&mut cases, control_target);
    push_mul_full_width_const_native_cases(&mut cases, control_target);
    push_mulacc_masked_identity_native_cases(&mut cases, control_target);
    push_lea_absolute_movn_native_cases(&mut cases, control_target);
    push_lea_pcrel_movn_native_cases(&mut cases, control_target);
    push_bfi_full_width_movn_native_cases(&mut cases, control_target);
    push_bfi_full_width_reg_native_cases(&mut cases, control_target);
    push_bfx_full_width_native_cases(&mut cases, control_target);
    push_bfxil_full_width_native_cases(&mut cases, control_target);
    push_bfxil_imm_source_native_cases(&mut cases, control_target);
    push_bit_permute_identity_native_cases(&mut cases, control_target);
    push_bit_permute_low_mask_native_cases(&mut cases, control_target);
    push_bit_permute_single_bit_native_cases(&mut cases, control_target);
    push_bit_permute_contiguous_mask_native_cases(&mut cases, control_target);
    push_bit_permute_imm_source_arbitrary_mask_native_cases(&mut cases, control_target);
    push_bit_scan_flag_native_cases(&mut cases, control_target);
    push_bextr_zero_imm_source_reg_control_native_cases(&mut cases, control_target);
    push_bzhi_zero_imm_source_reg_index_native_cases(&mut cases, control_target);
    push_bzhi_subword_flag_native_cases(&mut cases, control_target);
    push_cmove_imm_movn_native_cases(&mut cases, control_target);
    push_cmove_imm_csel_native_cases(&mut cases, control_target);
    push_cmove_always_reg_native_cases(&mut cases, control_target);
    push_cmove_same_reg_native_cases(&mut cases, control_target);

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_de70;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Not {
        dst: arm_x(0),
        src: arm_x(1),
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| panic!("not_w8_as_mvn_uxtb_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "not_w8_as_mvn_uxtb_preserves_flags".into(),
        [
            enc_logical_shift_regs(0, 0b01, 0, 1, 0, RD, 31, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Not {
        dst: arm_x(0),
        src: VReg::Imm(0),
        width: OpWidth::W16,
    }])
    .unwrap_or_else(|e| {
        panic!("not_w16_zero_as_movz_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "not_w16_zero_as_movz_preserves_flags".into(),
        [
            enc_logical_shift_regs(0, 0b01, 0, 1, 0, RD, 31, 31),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x3000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Not {
        dst: arm_x(0),
        src: VReg::Imm(0x3c),
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| panic!("not_w8_imm_as_movz_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "not_w8_imm_as_movz_preserves_flags".into(),
        [enc_mov_wide(0, 0b10, 0, 0xc3), NOP, NOP],
        lowered,
        st,
    ));

    push_not_imm_movn_native_cases(&mut cases, control_target);

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0xc000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Not {
        dst: arm_x(0),
        src: VReg::Imm(-16),
        width: OpWidth::W64,
    }])
    .unwrap_or_else(|e| panic!("not_x_imm_as_movz_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "not_x_imm_as_movz_preserves_flags".into(),
        [enc_mov_wide(1, 0b10, 0, 15), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1234_5678_9abc_de81;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Neg {
        dst: arm_x(0),
        src: arm_x(1),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("neg_w8_as_sub_uxtb_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "neg_w8_as_sub_uxtb_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x5000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Neg {
        dst: arm_x(0),
        src: VReg::Imm(0),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("neg_w8_zero_as_movz_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "neg_w8_zero_as_movz_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, 31),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Neg {
        dst: arm_x(0),
        src: VReg::Imm(3),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("neg_w8_imm_as_movz_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "neg_w8_imm_as_movz_preserves_flags".into(),
        [enc_mov_wide(0, 0b10, 0, 0xfd), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Neg {
        dst: arm_x(0),
        src: VReg::Imm(0x1234),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("neg_w_imm_as_movn_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "neg_w_imm_as_movn_preserves_flags".into(),
        [enc_mov_wide(0, 0b00, 0, 0x1233), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Neg {
        dst: arm_x(0),
        src: VReg::Imm(0x1234),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("neg_x_imm_as_movn_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "neg_x_imm_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0x1233), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x9000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Neg {
        dst: arm_x(0),
        src: VReg::Imm(-16),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("neg_x_imm_as_movz_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "neg_x_imm_as_movz_preserves_flags".into(),
        [enc_mov_wide(1, 0b10, 0, 16), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x1234_5678_9abc_deff;
    st.pstate = 0x3000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Inc {
        dst: arm_x(0),
        src: arm_x(1),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("inc_w8_as_add_uxtb_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "inc_w8_as_add_uxtb_preserves_flags".into(),
        [
            enc_addsub_imm_regs(0, 0, 0, 0, 1, RD, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xabcd_eeee_ffff_0000;
    st.pstate = 0xc000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Inc {
        dst: arm_x(0),
        src: VReg::Imm(0),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("inc_w8_zero_as_movz_one_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "inc_w8_zero_as_movz_one_preserves_flags".into(),
        [
            enc_addsub_imm_regs(0, 0, 0, 0, 1, RD, 31),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Inc {
        dst: arm_x(0),
        src: VReg::Imm(0xff),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("inc_w8_imm_wrap_as_movz_zero_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "inc_w8_imm_wrap_as_movz_zero_preserves_flags".into(),
        [enc_mov_wide(0, 0b10, 0, 0), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xbcde_ffff_0000_1111;
    st.pstate = 0x1000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Dec {
        dst: arm_x(0),
        src: VReg::Imm(0),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("dec_w16_zero_as_movz_all_ones_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "dec_w16_zero_as_movz_all_ones_preserves_flags".into(),
        [
            enc_addsub_imm_regs(0, 1, 0, 0, 1, RD, 31),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Dec {
        dst: arm_x(0),
        src: VReg::Imm(0),
        width: OpWidth::W32,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("dec_w_zero_as_movn_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "dec_w_zero_as_movn_preserves_flags".into(),
        [enc_mov_wide(0, 0b00, 0, 0), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Dec {
        dst: arm_x(0),
        src: VReg::Imm(0),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("dec_x_zero_as_movn_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "dec_x_zero_as_movn_preserves_flags".into(),
        [enc_mov_wide(1, 0b00, 0, 0), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xffff_eeee_dddd_cccc;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Dec {
        dst: arm_x(0),
        src: VReg::Imm(17),
        width: OpWidth::W64,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("dec_x_imm_as_movz_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "dec_x_imm_as_movz_preserves_flags".into(),
        [enc_mov_wide(1, 0b10, 0, 16), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0xaaaa_bbbb_cccc_ddf0;
    st.x[2] = 0x1234_5678_9abc_de31;
    st.pstate = 0x9000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Reg(arm_x(2)),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("add_w8_reg_as_add_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "add_w8_reg_as_add_uxtb_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 0, 0, 0, 0, RD, RN, RM),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xaaaa_bbbb_cccc_ddf2;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Reg(arm_x(1)),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("add_w8_zero_base_reg_as_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "add_w8_zero_base_reg_as_uxtb_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 0, 0, 0, 0, RD, 31, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xaaaa_bbbb_cccc_fedc;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Reg(VReg::Imm(0)),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("add_w16_zero_source_reg_as_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "add_w16_zero_source_reg_as_uxth_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 0, 0, 0, 0, RD, RN, 31),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.pstate = 0xf000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Add {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm(0x1234),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("add_w16_zero_base_imm_as_movz_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "add_w16_zero_base_imm_as_movz_preserves_flags".into(),
        [
            enc_addsub_imm_regs(0, 0, 0, 0, 0x234, RD, 31),
            enc_addsub_imm_regs(0, 0, 0, 1, 0x1, RD, RD),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sub {
        dst: arm_x(0),
        src1: VReg::Imm(0),
        src2: SrcOperand::Imm(0x34),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sub_w8_zero_base_imm_as_movz_negated_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sub_w8_zero_base_imm_as_movz_negated_preserves_flags".into(),
        [
            enc_addsub_imm_regs(0, 1, 0, 0, 0x34, RD, 31),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0x9999_aaaa_bbbb_cc80;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sub {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Reg(VReg::Imm(0)),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sub_w8_zero_source_reg_as_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sub_w8_zero_source_reg_as_uxtb_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, RN, 31),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xaaaa_bbbb_cccc_0100;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sub {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0x1234),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sub_w16_large_imm_as_split_sub_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sub_w16_large_imm_as_split_sub_uxth_preserves_flags".into(),
        [
            enc_addsub_imm_regs(0, 1, 0, 0, 0x234, RD, RN),
            enc_addsub_imm_regs(0, 1, 0, 1, 0x1, RD, RD),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0xffff_ffff_ffff_12fe;
    st.x[2] = 0xaaaa_bbbb_cccc_dd01;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Adc {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Reg(arm_x(2)),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("adc_w8_carry_set_as_adc_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "adc_w8_carry_set_as_adc_uxtb_preserves_flags".into(),
        [
            enc_addsub_carry(0, 0, 0),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0xffff_ffff_ffff_12ff;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Adc {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0x100),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("adc_w8_masked_zero_imm_as_adc_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "adc_w8_masked_zero_imm_as_adc_uxtb_preserves_flags".into(),
        [
            enc_addsub_carry_regs(0, 0, 0, RD, RN, 31),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0xffff_ffff_ffff_0100;
    st.x[2] = 0x1111_2222_3333_0001;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sbb {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Reg(arm_x(2)),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sbb_w16_carry_clear_as_sbc_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sbb_w16_carry_clear_as_sbc_uxth_preserves_flags".into(),
        [
            enc_addsub_carry(0, 1, 0),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0xffff_ffff_ffff_0100;
    st.pstate = 0;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Sbb {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0x1_0000),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("sbb_w16_masked_zero_imm_as_sbc_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "sbb_w16_masked_zero_imm_as_sbc_uxth_preserves_flags".into(),
        [
            enc_addsub_carry_regs(0, 1, 0, RD, RN, 31),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = 0xffff_ffff_1234_56fe;
    st.x[2] = 0x1111_2222_3333_4405;
    st.pstate = 0xa000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulU {
        dst_lo: arm_x(0),
        dst_hi: None,
        src1: arm_x(1),
        src2: SrcOperand::Reg(arm_x(2)),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| panic!("mulu_w8_as_mul_uxtb_preserves_flags: native lowering failed: {e}"));
    cases.push((
        "mulu_w8_as_mul_uxtb_preserves_flags".into(),
        [
            enc_dp3_ra(0, 0b000, 0, 31),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0x7777_8888_1234_fffe;
    st.x[2] = 0x1111_2222_abcd_0003;
    st.pstate = 0x4000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulS {
        dst_lo: arm_x(0),
        dst_hi: None,
        src1: arm_x(1),
        src2: SrcOperand::Reg(arm_x(2)),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("muls_w16_as_mul_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "muls_w16_as_mul_uxth_preserves_flags".into(),
        [
            enc_dp3_ra(0, 0b000, 0, 31),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_1234_56fe;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulS {
        dst_lo: arm_x(0),
        dst_hi: None,
        src1: arm_x(1),
        src2: SrcOperand::Imm64(1),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("muls_w8_imm_one_as_mov_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "muls_w8_imm_one_as_mov_uxtb_preserves_flags".into(),
        [
            enc_mov_reg(0, RD, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_8765_43ab;
    st.pstate = 0x3000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulU {
        dst_lo: arm_x(0),
        dst_hi: None,
        src1: arm_x(1),
        src2: SrcOperand::Imm64(0x101),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("mulu_w8_imm_masked_one_as_mov_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mulu_w8_imm_masked_one_as_mov_uxtb_preserves_flags".into(),
        [
            enc_mov_reg(0, RD, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_1234_8001;
    st.pstate = 0x5000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulS {
        dst_lo: arm_x(0),
        dst_hi: None,
        src1: arm_x(1),
        src2: SrcOperand::Imm64(-1),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("muls_w16_imm_neg_one_as_neg_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "muls_w16_imm_neg_one_as_neg_uxth_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_8765_8001;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulU {
        dst_lo: arm_x(0),
        dst_hi: None,
        src1: arm_x(1),
        src2: SrcOperand::Imm64(0x1_ffff),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!(
            "mulu_w16_imm_masked_neg_one_as_neg_uxth_preserves_flags: native lowering failed: {e}"
        )
    });
    cases.push((
        "mulu_w16_imm_masked_neg_one_as_neg_uxth_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_1234_56fe;
    st.x[2] = 0x1111_2222_3333_4405;
    st.x[3] = 0x2222_3333_4444_5511;
    st.pstate = 0xc000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulAdd {
        dst: arm_x(0),
        acc: arm_x(3),
        src1: arm_x(1),
        src2: arm_x(2),
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| {
        panic!("muladd_w8_as_madd_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "muladd_w8_as_madd_uxtb_preserves_flags".into(),
        [
            enc_dp3(0, 0b000, 0),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x7777_8888_1234_fffe;
    st.x[2] = 0x1111_2222_abcd_0003;
    st.x[3] = 0x3333_4444_5555_0100;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulSub {
        dst: arm_x(0),
        acc: arm_x(3),
        src1: arm_x(1),
        src2: arm_x(2),
        width: OpWidth::W16,
    }])
    .unwrap_or_else(|e| {
        panic!("mulsub_w16_as_msub_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mulsub_w16_as_msub_uxth_preserves_flags".into(),
        [
            enc_dp3(0, 0b000, 1),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_8002;
    st.x[3] = 0x4444_5555_6666_0010;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulSub {
        dst: arm_x(0),
        acc: arm_x(3),
        src1: arm_x(1),
        src2: VReg::Imm(1),
        width: OpWidth::W16,
    }])
    .unwrap_or_else(|e| {
        panic!("mulsub_w16_imm_one_as_sub_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mulsub_w16_imm_one_as_sub_uxth_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 3, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_00f2;
    st.x[3] = 0x4444_5555_6666_0010;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::MulSub {
        dst: arm_x(0),
        acc: arm_x(3),
        src1: arm_x(1),
        src2: VReg::Imm(-1),
        width: OpWidth::W8,
    }])
    .unwrap_or_else(|e| {
        panic!("mulsub_w8_imm_neg_one_as_add_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "mulsub_w8_imm_neg_one_as_add_uxtb_preserves_flags".into(),
        [
            enc_addsub_shift_regs(0, 0, 0, 0, 0, RD, 3, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff_ffff_12f0;
    st.x[2] = 0xaaaa_bbbb_cccc_003f;
    st.pstate = 0xb000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::And {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Reg(arm_x(2)),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("and_w8_reg_as_and_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "and_w8_reg_as_and_uxtb_preserves_flags".into(),
        [
            enc_logical_shift_regs(0, 0b00, 0, 0, 0, RD, RN, RM),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x3f3f_4f4f_5f5f_6f6f;
    st.x[1] = 0xffff_ffff_1234_abcd;
    st.pstate = 0x9000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::And {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm64(-1),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("and_w16_imm_masked_all_ones_as_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "and_w16_imm_masked_all_ones_as_uxth_preserves_flags".into(),
        [enc_bitfield_regs(0, 0b10, 0, 15, RN, RD), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0x7000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::And {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("and_w8_zero_imm_as_movz_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "and_w8_zero_imm_as_movz_preserves_flags".into(),
        [enc_mov_wide(0, 0b10, 0, 0), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x4f4f_5f5f_6f6f_7f7f;
    st.x[1] = 0xffff_ffff_ffff_12a5;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Xor {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("xor_w8_zero_imm_as_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "xor_w8_zero_imm_as_uxtb_preserves_flags".into(),
        [enc_bitfield_regs(0, 0b10, 0, 7, RN, RD), NOP, NOP],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0xffff_ffff_1234_56f0;
    st.pstate = 0x8000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Xor {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0x00ff),
        width: OpWidth::W16,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("xor_w16_imm_as_eor_uxth_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "xor_w16_imm_as_eor_uxth_preserves_flags".into(),
        [
            enc_logical_imm(0, 0b10, 0, 0, 7, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0x9999_aaaa_bbbb_ccf5;
    st.pstate = 0x2000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::AndNot {
        dst: arm_x(0),
        src1: arm_x(1),
        src2: SrcOperand::Imm(0xf0),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("andnot_w8_imm_as_and_inverse_uxtb_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "andnot_w8_imm_as_and_inverse_uxtb_preserves_flags".into(),
        [
            enc_logical_imm(0, 0b00, 0, 0, 3, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0xffff_ffff_ffff_00f7;
    st.pstate = 0xd000_0000;
    let lowered = lower_aarch64_native_ops(vec![OpKind::Bzhi {
        dst: arm_x(0),
        src: arm_x(1),
        index: VReg::Imm(5),
        width: OpWidth::W8,
        flags: FlagUpdate::None,
    }])
    .unwrap_or_else(|e| {
        panic!("bzhi_w8_imm_index_as_and_mask_preserves_flags: native lowering failed: {e}")
    });
    cases.push((
        "bzhi_w8_imm_index_as_and_mask_preserves_flags".into(),
        [
            enc_logical_imm(0, 0b00, 0, 0, 4, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        lowered,
        st,
    ));

    let mut st = native_state();
    st.x[0] = 0xdead_beef_dead_beef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xffff_ffff_ffff_ffff;
    st.x[3] = 0xaabb_ccdd_eeff_0011;
    st.scratch[8] = 0x1122_3344_5566_7788;
    st.pstate = 0x6000_0000;
    let lowered = lower_aarch64_native_ops(vec![
        OpKind::LoadExclusive {
            dst: arm_x(0),
            addr: native_direct_addr.clone(),
            width: MemWidth::B8,
        },
        OpKind::StoreExclusive {
            status: arm_x(2),
            src: arm_x(3),
            addr: native_direct_addr.clone(),
            width: MemWidth::B8,
        },
    ])
    .unwrap_or_else(|e| {
        panic!("ldxr_stxr_x_direct_opkind_stores_with_monitor: native lowering failed: {e}")
    });
    cases.push((
        "ldxr_stxr_x_direct_opkind_stores_with_monitor".into(),
        [enc_ldxr(3, 0), enc_stxr(3, 0), NOP],
        lowered,
        st,
    ));

    let mut push_lifted_case = |label: &str, source: u32, st: ArmState| {
        let lowered = lower_aarch64_native_insn(source)
            .unwrap_or_else(|e| panic!("{label}: native lifted lowering failed: {e}"));
        cases.push((label.into(), [source, NOP, NOP], lowered, st));
    };

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0x0102_0304_0506_0708;
    st.x[2] = 0x11;
    st.pstate = 0xd000_0000;
    push_lifted_case(
        "add_x_lsl7_lifted_preserves_flags",
        enc_addsub_shift(1, 0, 0, 0, 7),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_0000_0010;
    st.x[2] = 0xffff_ffff_8000_0000;
    st.pstate = 0x1000_0000;
    push_lifted_case(
        "subs_w_asr31_lifted_sets_flags_zero_ext",
        enc_addsub_shift(0, 1, 1, 2, 31),
        st,
    );

    let mut st = native_state();
    st.x[1] = 0x1000_0000_0000_0000;
    st.x[2] = 0xffff_ffff_0000_0100;
    st.pstate = 0x8000_0000;
    push_lifted_case(
        "add_x_uxtw_lifted_preserves_flags",
        enc_addsub_ext(1, 0, 0, 0b010, 0),
        st,
    );

    let mut st = native_state();
    st.x[1] = 8;
    st.x[2] = 0xffff_ffff;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "add_x_sxtw_lsl2_lifted_preserves_flags",
        enc_addsub_ext(1, 0, 0, 0b110, 2),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x10;
    st.x[2] = 0xffff_ff0f;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "subs_w_uxtb_lsl1_lifted_sets_flags_zero_ext",
        enc_addsub_ext(0, 1, 1, 0b000, 1),
        st,
    );

    let mut st = native_state();
    st.x[1] = u64::MAX;
    st.x[2] = 0;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "adc_x_carry_in_set_lifted_preserves_flags",
        enc_addsub_carry(1, 0, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff;
    st.x[2] = 0;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "adcs_w_carry_in_set_lifted_sets_flags_zero_ext",
        enc_addsub_carry(0, 0, 1),
        st,
    );

    let mut st = native_state();
    st.x[1] = 5;
    st.x[2] = 3;
    st.pstate = 0;
    push_lifted_case(
        "sbc_x_carry_clear_borrow_in_lifted_preserves_flags",
        enc_addsub_carry(1, 1, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 5;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "sbcs_w_carry_set_lifted_sets_flags_zero_ext",
        enc_addsub_carry(0, 1, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "ngc_x_carry_set_lifted_preserves_flags",
        enc_addsub_carry_rn(1, 1, 0, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[2] = 1;
    st.pstate = 0;
    push_lifted_case(
        "ngcs_w_carry_clear_lifted_sets_flags_zero_ext",
        enc_addsub_carry_rn(0, 1, 1, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 5;
    st.x[2] = 7;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "ccmp_x_eq_true_lifted_sets_flags",
        enc_condcmp(1, 1, false, RM, 0, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 5;
    st.x[2] = 7;
    st.pstate = 0;
    push_lifted_case(
        "ccmp_x_eq_false_lifted_fallback_z",
        enc_condcmp(1, 1, false, RM, 0, 0b0100),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = u64::MAX;
    st.x[2] = 1;
    st.pstate = 0;
    push_lifted_case(
        "ccmn_x_ne_true_lifted_sets_flags",
        enc_condcmp(1, 0, false, RM, 1, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0xffff_ffff;
    st.x[2] = 1;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "ccmn_w_ne_false_lifted_fallback_nv",
        enc_condcmp(0, 0, false, RM, 1, 0b1001),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0x10;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "ccmp_w_imm_hi_true_lifted_sets_flags",
        enc_condcmp(0, 1, true, 5, 8, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 0x5555_6666_7777_8888;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "csel_x_eq_lifted_true_preserves_flags",
        enc_csel_form(1, 0, 0, RN, RM, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = u64::MAX;
    st.pstate = 0;
    push_lifted_case(
        "csinc_x_eq_lifted_false_preserves_flags",
        enc_csel_form(1, 0, 1, RN, RM, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 0xffff_ffff_1234_5678;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "csinv_w_ne_lifted_false_zero_ext_preserves_flags",
        enc_csel_form(0, 1, 0, RN, RM, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x1111_2222_3333_4444;
    st.x[2] = 5;
    st.pstate = 0;
    push_lifted_case(
        "csneg_x_hi_lifted_false_preserves_flags",
        enc_csel_form(1, 1, 1, RN, RM, 8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0;
    push_lifted_case(
        "cset_x_raw_ne_lifted_true_alias_preserves_flags",
        enc_csel_form(1, 0, 1, 31, 31, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "csetm_w_raw_ne_lifted_false_alias_zero_ext_preserves_flags",
        enc_csel_form(0, 1, 0, 31, 31, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 5;
    st.x[2] = 7;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "cmp_x_lifted_discards_result_sets_flags",
        enc_addsub_shift_regs(1, 1, 1, 0, 0, 31, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = u64::MAX;
    st.x[2] = 1;
    st.pstate = 0;
    push_lifted_case(
        "cmn_x_lifted_discards_result_sets_flags",
        enc_addsub_shift_regs(1, 0, 1, 0, 0, 31, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x0100;
    st.x[2] = 0x0200;
    st.pstate = 0xe000_0000;
    push_lifted_case(
        "cmp_x_lsr4_lifted_discards_result_sets_flags",
        enc_addsub_shift_regs(1, 1, 1, 1, 4, 31, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x7fff_ffff;
    st.x[2] = 1;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "cmn_w_lsl1_lifted_discards_result_sets_flags",
        enc_addsub_shift_regs(0, 0, 1, 0, 1, 31, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0;
    st.x[2] = 0xffff;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "cmp_x_sxth_lsl3_lifted_discards_result_sets_flags",
        enc_addsub_ext_regs(1, 1, 1, 0b101, 3, 31, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_fff0;
    st.x[2] = 1;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "cmn_w_uxtb_lsl4_lifted_discards_result_sets_flags",
        enc_addsub_ext_regs(0, 0, 1, 0b000, 4, 31, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.x[2] = 0xffff_ffff_0000_0001;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "tst_w_lifted_discards_result_sets_flags",
        enc_logical_shift_regs(0, 0b11, 0, 0, 0, 31, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5656_7878_9a9a_bcbc;
    st.x[1] = 0xffff_0000_ffff_0000;
    st.x[2] = 0x00ff_00ff_00ff_00ff;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "bic_x_lifted_preserves_flags",
        enc_logical_shift_regs(1, 0b00, 0, 1, 0, RD, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7878_9a9a_bcbc_dede;
    st.x[1] = 0xffff_ffff_8000_00f0;
    st.x[2] = 0xffff_ffff_0000_00f0;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "bics_w_lifted_sets_flags_zero_ext",
        enc_logical_shift_regs(0, 0b11, 0, 1, 0, RD, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9a9a_bcbc_dede_f0f0;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.x[2] = 0xffff_ffff_0000_0001;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "and_w_asr31_lifted_zero_ext_preserves_flags",
        enc_logical_shift(0, 0b00, 2, 0, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xabab_cdcd_efef_0101;
    st.x[1] = 0xffff_0000_00ff_00ff;
    st.x[2] = 0x00ff_00ff_ffff_0000;
    st.pstate = 0xa000_0000;
    push_lifted_case(
        "orr_x_lsr8_lifted_preserves_flags",
        enc_logical_shift(1, 0b01, 1, 0, 8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbcbc_dede_f0f0_1212;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.x[2] = 0x0fed_cba9_8765_4321;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "eor_x_ror13_lifted_preserves_flags",
        enc_logical_shift(1, 0b10, 3, 0, 13),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcdcd_efef_0101_2323;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 0xffff_ffff_ffff_ffff;
    st.pstate = 0;
    push_lifted_case(
        "ands_x_lsr1_lifted_sets_flags",
        enc_logical_shift(1, 0b11, 1, 0, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdede_f0f0_1212_3434;
    st.x[1] = 0x00ff_0000_0000_0000;
    st.x[2] = 0x0000_00ff_0000_0000;
    st.pstate = 0xf000_0000;
    push_lifted_case(
        "tst_x_lsr8_lifted_discards_result_sets_flags",
        enc_logical_shift_regs(1, 0b11, 1, 0, 8, 31, RN, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xefef_0101_2323_4545;
    st.x[1] = u64::MAX;
    st.x[2] = 1;
    st.pstate = 0x7000_0000;
    push_lifted_case(
        "bic_x_lsl4_lifted_preserves_flags",
        enc_logical_shift(1, 0b00, 0, 1, 4),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[2] = 0xffff_ffff_0000_0003;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "neg_x_lifted_preserves_flags",
        enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[2] = 0xffff_ffff_8000_0000;
    st.pstate = 0;
    push_lifted_case(
        "negs_w_lifted_sets_flags_zero_ext",
        enc_addsub_shift_regs(0, 1, 1, 0, 0, RD, 31, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[2] = 0x00ff_0000_ffff_0000;
    st.pstate = 0xf000_0000;
    push_lifted_case(
        "mvn_x_lifted_preserves_flags",
        enc_logical_shift_regs(1, 0b01, 0, 1, 0, RD, 31, RM),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0x0000_0000_0000_0003;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "lsl_imm_x_lifted_preserves_flags",
        enc_bitfield(1, 0b10, 51, 50),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "lsr_imm_w_lifted_zero_ext_preserves_flags",
        enc_bitfield(0, 0b10, 9, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = 0x8000_0000_0000_0000;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "asr_imm_x_lifted_preserves_flags",
        enc_bitfield(1, 0b00, 9, 63),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0x1122_3344_5566_7788;
    st.x[2] = 0x99aa_bbcc_ddee_ff00;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "extr_x_lifted_preserves_flags",
        enc_extract(1, RN, RM, 13),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_1122_3344;
    st.x[2] = 0xffff_ffff_aabb_ccdd;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "extr_w_lifted_zero_ext_preserves_flags",
        enc_extract(0, RN, RM, 7),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "ror_imm_x_lifted_preserves_flags",
        enc_extract(1, RN, RN, 17),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 1;
    st.x[2] = 65;
    st.pstate = 0x1000_0000;
    push_lifted_case(
        "lslv_x_lifted_masked_count_preserves_flags",
        enc_dp2(1, 0b1000),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 4;
    st.pstate = 0x8000_0000;
    push_lifted_case("lsrv_x_lifted_preserves_flags", enc_dp2(1, 0b1001), st);

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 4;
    st.pstate = 0x3000_0000;
    push_lifted_case("asrv_x_lifted_preserves_flags", enc_dp2(1, 0b1010), st);

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.x[2] = 36;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "rorv_w_lifted_zero_ext_masked_count_preserves_flags",
        enc_dp2(0, 0b1011),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x0000_0000_0000_00f0;
    st.pstate = 0x7000_0000;
    push_lifted_case("clz_x_lifted_preserves_flags", enc_dp1(1, 0b000100), st);

    let mut st = native_state();
    st.x[0] = 0xeded_0101_2323_4545;
    st.x[1] = 0xffff_ffff_ffff_f000;
    st.pstate = 0xa000_0000;
    push_lifted_case("cls_x_lifted_preserves_flags", enc_dp1(1, 0b000101), st);

    let mut st = native_state();
    st.x[0] = 0xfefe_1010_3232_5454;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "cls_w_lifted_zero_ext_preserves_flags",
        enc_dp1(0, 0b000101),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_0000_0001;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "rbit_w_lifted_zero_ext_preserves_flags",
        enc_dp1(0, 0b000000),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0x1122_3344_5566_7788;
    st.pstate = 0x9000_0000;
    push_lifted_case("rev_x_lifted_preserves_flags", enc_dp1(1, 0b000011), st);

    let mut st = native_state();
    st.x[0] = 0x1234_5678_9abc_def0;
    st.x[1] = 0x0011_2233_4455_6677;
    st.pstate = 0x3000_0000;
    push_lifted_case("rev16_x_lifted_preserves_flags", enc_dp1(1, 0b000001), st);

    let mut st = native_state();
    st.x[0] = 0x2345_6789_abcd_ef01;
    st.x[1] = 0xffff_ffff_1122_3344;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "rev16_w_lifted_zero_ext_preserves_flags",
        enc_dp1(0, 0b000001),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3456_789a_bcde_f012;
    st.x[1] = 0x1122_3344_aabb_ccdd;
    st.pstate = 0x5000_0000;
    push_lifted_case("rev32_x_lifted_preserves_flags", enc_dp1(1, 0b000010), st);

    let mut st = native_state();
    st.x[0] = 0x0000_1111_2222_3333;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "ubfx_x_lifted_preserves_flags",
        enc_bitfield(1, 0b10, 8, 23),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0xffff_ffff_0000_08f0;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "sbfx_w_lifted_sign_ext_zero_ext_preserves_flags",
        enc_bitfield(0, 0b00, 4, 11),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0x1234_5678_9abc_de77;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "ubfiz_x_lifted_preserves_flags",
        enc_bitfield(1, 0b10, 60, 7),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff_0000_0081;
    st.pstate = 0x1000_0000;
    push_lifted_case(
        "sbfiz_w_lifted_sign_ext_zero_ext_preserves_flags",
        enc_bitfield(0, 0b00, 24, 7),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1234_5678_9abc_de77;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "bfi_x_lifted_preserves_flags",
        enc_bitfield(1, 0b01, 56, 7),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0x1234_5678_9abc_de77;
    st.pstate = 0x7000_0000;
    push_lifted_case(
        "bfxil_x_lifted_preserves_flags",
        enc_bitfield(1, 0b01, 8, 15),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0xffff_ffff_1234_56ab;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "uxtb_w_lifted_zero_ext_preserves_flags",
        enc_bitfield(0, 0b10, 0, 7),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = 0xffff_ffff_8000_0001;
    st.pstate = 0x1000_0000;
    push_lifted_case(
        "sxtw_x_lifted_preserves_flags",
        enc_bitfield(1, 0b00, 0, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0x0000_0000_1234_5678;
    st.x[2] = 0x10;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "mul_x_lifted_preserves_flags",
        enc_dp3_ra(1, 0b000, 0, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "mul_w_lifted_zero_ext_preserves_flags",
        enc_dp3_ra(0, 0b000, 0, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = 0x1234;
    st.x[2] = 0x1000;
    st.x[3] = 0x55;
    st.pstate = 0x4000_0000;
    push_lifted_case("madd_x_lifted_preserves_flags", enc_dp3(1, 0b000, 0), st);

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0xffff_ffff_ffff_fffd;
    st.x[2] = 7;
    st.x[3] = 0x20;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "msub_w_lifted_zero_ext_preserves_flags",
        enc_dp3(0, 0b000, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.x[2] = 5;
    st.pstate = 0x8000_0000;
    push_lifted_case(
        "mneg_x_lifted_preserves_flags",
        enc_dp3_ra(1, 0b000, 1, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0x8000_0000_0000_0000;
    st.x[2] = 3;
    st.pstate = 0x1000_0000;
    push_lifted_case(
        "smulh_x_lifted_preserves_flags",
        enc_dp3_ra(1, 0b010, 0, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_ff00;
    st.x[2] = 0x100;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "umulh_x_lifted_preserves_flags",
        enc_dp3_ra(1, 0b110, 0, 31),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.x[2] = 0x12345;
    st.pstate = 0x5000_0000;
    push_lifted_case("udiv_x_lifted_preserves_flags", enc_dp2(1, 0b0010), st);

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_8000_0100;
    st.x[2] = 0xffff_ffff_ffff_fffb;
    st.pstate = 0x7000_0000;
    push_lifted_case(
        "sdiv_w_lifted_zero_ext_preserves_flags",
        enc_dp2(0, 0b0011),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0x8877_6655_4433_2211;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "ldr_x_base_offset_lifted_preserves_flags",
        enc_ldst_uimm(3, 0, 1, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0xffff_ffff_ffff_ff80;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "ldrb_direct_lifted_zero_ext_preserves_flags",
        enc_ldst_uimm(0, 0, 1, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0x1111_2222_3333_4480;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "ldrsb_w_direct_lifted_zero_ext_preserves_flags",
        enc_ldst_uimm(0, 0, 3, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0x0000_0000_8000_0001;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "ldrsw_direct_lifted_sign_ext_preserves_flags",
        enc_ldst_uimm(2, 0, 2, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0xc000_0000;
    push_lifted_case("ldar_x_lifted_preserves_flags", enc_ldar(3), st);

    let mut st = native_state();
    st.x[1] = SCRATCH_BASE;
    st.x[3] = 0xffff_ffff_aabb_ccdd;
    st.scratch[8] = 0x1122_3344_5566_7788;
    st.pstate = 0x5000_0000;
    push_lifted_case("stlr_w_lifted_preserves_flags_and_memory", enc_stlr(2), st);

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x2000_0000;
    push_lifted_case("ldxr_x_direct_lifted_preserves_flags", enc_ldxr(3, 0), st);

    let mut st = native_state();
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xffff_ffff_ffff_ffff;
    st.x[3] = 0x0102_0304_0506_0708;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0x8000_0000;
    push_lifted_case(
        "stxr_x_direct_lifted_fails_without_monitor_preserves_flags_and_memory",
        enc_stxr(3, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0102_0304_0506_0708;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "swp_x_lifted_preserves_flags_and_memory",
        enc_atomic_smir(3, 0, 0, 1, 0b000, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_aaaa_bbbb;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_0007;
    st.scratch[8] = 0x1122_3344_ffff_fffe;
    st.pstate = 0x1000_0000;
    push_lifted_case(
        "ldadd_w_lifted_acqrel_zero_ext_preserves_flags_and_memory",
        enc_atomic_smir(2, 1, 1, 0, 0b000, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1234_5678_9abc_def0;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_00ff;
    st.scratch[8] = 0xff00_ff00_0f0f_f0f0;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "ldclr_x_lifted_preserves_flags_and_memory",
        enc_atomic_smir(3, 0, 0, 0, 0b001, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0101_0101_0101_0101;
    st.scratch[8] = 0xf0f0_0f0f_aaaa_5555;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "ldeor_x_lifted_preserves_flags_and_memory",
        enc_atomic_smir(3, 0, 0, 0, 0b010, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_00f0;
    st.scratch[8] = 0xaaaa_bbbb_ccdd_000f;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "ldset_h_lifted_preserves_flags_and_memory",
        enc_atomic_smir(1, 0, 1, 0, 0b011, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_0007;
    st.scratch[8] = 0xaaaa_bbbb_ffff_fff0;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "ldsmax_w_lifted_uses_signed_width_preserves_flags_and_memory",
        enc_atomic_smir(2, 1, 0, 0, 0b100, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xffff_ffff_ffff_fff0;
    st.scratch[8] = 0x1111_2222_0000_0007;
    st.pstate = 0xa000_0000;
    push_lifted_case(
        "ldsmin_w_lifted_uses_signed_width_preserves_flags_and_memory",
        enc_atomic_smir(2, 0, 0, 0, 0b101, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_ffff_ffff;
    st.scratch[8] = 0xaaaa_bbbb_0000_0001;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "ldumax_w_lifted_uses_unsigned_width_preserves_flags_and_memory",
        enc_atomic_smir(2, 1, 1, 0, 0b110, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_0000_0012;
    st.scratch[8] = 0x3333_4444_5555_6680;
    st.pstate = 0x8000_0000;
    push_lifted_case(
        "ldumin_b_lifted_uses_unsigned_width_preserves_flags_and_memory",
        enc_atomic_smir(0, 0, 0, 0, 0b111, 2, RN, RD),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0102_0304_0506_0708;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x8877_6655_4433_2211;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0x7000_0000;
    push_lifted_case(
        "cas_x_lifted_success_preserves_flags_and_memory",
        enc_cas(3, 0, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0000_0000_aaaa_bbbb;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0000_0000_1111_2222;
    st.scratch[8] = 0xcccc_dddd_3333_4444;
    st.pstate = 0xd000_0000;
    push_lifted_case(
        "cas_w_lifted_failure_zero_ext_preserves_flags_and_memory",
        enc_cas(2, 1, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0x0000_0000_0000_8001;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "ldrsh_w_base_offset_lifted_zero_ext_preserves_flags",
        enc_ldst_uimm(1, 0, 3, 4),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = SCRATCH_BASE;
    st.scratch[7] = 0x2222_3333_4444_5580;
    st.pstate = 0x8000_0000;
    push_lifted_case(
        "ldursb_w_neg8_lifted_zero_ext_preserves_flags",
        enc_ldst_simm(0, 0, 3, 0b00, -8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "str_x_base_offset_lifted_preserves_flags_and_memory",
        enc_ldst_uimm(3, 0, 0, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_44aa;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0xffff_ffff_ffff_5500;
    st.pstate = 0xa000_0000;
    push_lifted_case(
        "strb_direct_lifted_preserves_flags_and_memory",
        enc_ldst_uimm(0, 0, 0, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "ldr_x_pre8_lifted_preserves_flags_and_writeback",
        enc_ldst_simm(3, 0, 1, 0b11, 8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1234_5678_9abc_def0;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "str_x_post_neg8_lifted_preserves_flags_memory_and_writeback",
        enc_ldst_simm(3, 0, 0, 0b01, -8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0xffff_ffff_8000_1234;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "ldr_w_post8_lifted_zero_ext_preserves_flags_and_writeback",
        enc_ldst_simm(2, 0, 1, 0b01, 8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = SCRATCH_BASE;
    st.scratch[8] = 0x0000_0000_0000_8001;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "ldrsh_w_post8_lifted_zero_ext_preserves_flags_and_writeback",
        enc_ldst_simm(1, 0, 3, 0b01, 8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_44aa;
    st.x[1] = SCRATCH_BASE;
    st.scratch[7] = 0x1122_3344_5566_7700;
    st.pstate = 0xa000_0000;
    push_lifted_case(
        "strb_pre_neg8_lifted_preserves_flags_memory_and_writeback",
        enc_ldst_simm(0, 0, 0, 0b11, -8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = SCRATCH_BASE;
    st.scratch[7] = 0x3333_4444_5555_6680;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "ldrsb_w_pre_neg8_lifted_zero_ext_preserves_flags_and_writeback",
        enc_ldst_simm(0, 0, 3, 0b11, -8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = SCRATCH_BASE;
    st.scratch[9] = 0x0000_0000_8000_0001;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "ldrsw_pre8_lifted_sign_ext_preserves_flags_and_writeback",
        enc_ldst_simm(2, 0, 2, 0b11, 8),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 8;
    st.scratch[9] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x7000_0000;
    push_lifted_case(
        "ldr_x_reg_lsl0_lifted_preserves_flags",
        enc_ldst_reg(3, 1, RM, 0b011, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 8;
    st.scratch[16] = 0x8877_6655_4433_2211;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "ldr_x_reg_uxtw_lsl3_lifted_preserves_flags",
        enc_ldst_reg(3, 1, RM, 0b010, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = (-8i64) as u64;
    st.scratch[7] = 0;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "str_x_reg_sxtx_neg8_lifted_preserves_flags_and_memory",
        enc_ldst_reg(3, 0, RM, 0b111, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 8;
    st.scratch[9] = 0xffff_ffff_8000_1234;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "ldr_w_reg_uxtw_lifted_zero_ext_preserves_flags",
        enc_ldst_reg(2, 1, RM, 0b010, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = (-8i64) as u64;
    st.scratch[7] = 0x0000_0000_0000_0080;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "ldrsb_w_reg_sxtw_lifted_zero_ext_preserves_flags",
        enc_ldst_reg(0, 3, RM, 0b110, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 4;
    st.scratch[9] = 0x0000_0000_0000_8001;
    st.pstate = 0x9000_0000;
    push_lifted_case(
        "ldrsh_x_reg_uxtw_lsl1_lifted_sign_ext_preserves_flags",
        enc_ldst_reg(1, 2, RM, 0b010, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 4;
    st.scratch[9] = 0x0000_0000_0000_8001;
    st.pstate = 0x1000_0000;
    push_lifted_case(
        "ldrsh_w_reg_uxtw_lsl1_lifted_zero_ext_preserves_flags",
        enc_ldst_reg(1, 3, RM, 0b010, 1),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_99aa;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 15;
    st.scratch[9] = 0x1122_3344_5566_7788;
    st.pstate = 0xa000_0000;
    push_lifted_case(
        "strb_reg_uxtw_lifted_preserves_flags_and_memory",
        enc_ldst_reg(0, 0, RM, 0b010, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x3333_4444_5555_6666;
    st.scratch[10] = 0x0123_4567_89ab_cdef;
    st.scratch[11] = 0xfedc_ba98_7654_3210;
    st.pstate = 0x7000_0000;
    push_lifted_case(
        "ldp_x_base_offset_lifted_preserves_flags",
        enc_ldp(0b10, 0, 0b10, 1, 2),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x5555_6666_7777_8888;
    st.scratch[8] = 0x8877_6655_4433_2211;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "ldp_w_direct_lifted_zero_ext_preserves_flags",
        enc_ldp(0b00, 0, 0b10, 1, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xbbbb_cccc_dddd_eeee;
    st.scratch[9] = 0x8000_0001_7fff_fffe;
    st.pstate = 0x4000_0000;
    push_lifted_case(
        "ldpsw_base_offset_lifted_sign_ext_preserves_flags",
        enc_ldp(0b01, 0, 0b10, 1, 2),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xfedc_ba98_7654_3210;
    st.scratch[10] = 0;
    st.scratch[11] = 0;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "stp_x_base_offset_lifted_preserves_flags_and_memory",
        enc_ldp(0b10, 0, 0b10, 0, 2),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0123_4567_7654_3210;
    st.scratch[8] = 0;
    st.pstate = 0xa000_0000;
    push_lifted_case(
        "stp_w_direct_lifted_preserves_flags_and_memory",
        enc_ldp(0b00, 0, 0b10, 0, 0),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x3333_4444_5555_6666;
    st.scratch[10] = 0x0123_4567_89ab_cdef;
    st.scratch[11] = 0xfedc_ba98_7654_3210;
    st.pstate = 0x7000_0000;
    push_lifted_case(
        "ldp_x_pre16_lifted_preserves_flags_and_writeback",
        enc_ldp(0b10, 0, 0b11, 1, 2),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xdddd_eeee_ffff_0000;
    st.scratch[9] = 0x8000_0001_7fff_fffe;
    st.pstate = 0x8000_0000;
    push_lifted_case(
        "ldpsw_pre8_lifted_sign_ext_preserves_flags_and_writeback",
        enc_ldp(0b01, 0, 0b11, 1, 2),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xfedc_ba98_7654_3210;
    st.scratch[8] = 0;
    st.scratch[9] = 0;
    st.pstate = 0x5000_0000;
    push_lifted_case(
        "stp_x_post_neg16_lifted_preserves_flags_memory_and_writeback",
        enc_ldp(0b10, 0, 0b01, 0, ((-2i32) as u32) & 0x7f),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_ffff;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x5555_6666_7777_8888;
    st.scratch[8] = 0xffff_ffff_8000_1234;
    st.pstate = 0x3000_0000;
    push_lifted_case(
        "ldp_w_post8_lifted_zero_ext_preserves_flags_and_writeback",
        enc_ldp(0b00, 0, 0b01, 1, 2),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xeeee_ffff_0000_1111;
    st.scratch[8] = 0x8000_0002_ffff_ff80;
    st.pstate = 0x1000_0000;
    push_lifted_case(
        "ldpsw_post_neg8_lifted_sign_ext_preserves_flags_and_writeback",
        enc_ldp(0b01, 0, 0b01, 1, ((-2i32) as u32) & 0x7f),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x0123_4567_7654_3210;
    st.scratch[7] = 0;
    st.pstate = 0xa000_0000;
    push_lifted_case(
        "stp_w_pre_neg8_lifted_preserves_flags_memory_and_writeback",
        enc_ldp(0b00, 0, 0b11, 0, ((-2i32) as u32) & 0x7f),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0x7777_8888_9999_aaaa;
    st.scratch[7] = 0x1020_3040_5060_7080;
    st.scratch[8] = 0x9080_7060_5040_3020;
    st.pstate = 0x6000_0000;
    push_lifted_case(
        "ldnp_x_neg8_lifted_preserves_flags",
        enc_ldp(0b10, 0, 0b00, 1, ((-1i32) as u32) & 0x7f),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_0bad_cafe;
    st.x[1] = SCRATCH_BASE;
    st.x[2] = 0xffff_ffff_dead_beef;
    st.scratch[9] = 0;
    st.pstate = 0x2000_0000;
    push_lifted_case(
        "stnp_w_base_offset_lifted_preserves_flags_and_memory",
        enc_ldp(0b00, 0, 0b00, 0, 2),
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0xa000_0000;
    push_lifted_case("clrex_lifted_preserves_arch_state", enc_clrex(), st);

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0x5000_0000;
    push_lifted_case("dsb_sy_lifted_preserves_arch_state", enc_barrier(0b100), st);

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.pstate = 0x9000_0000;
    push_lifted_case("dmb_sy_lifted_preserves_arch_state", enc_barrier(0b101), st);

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.pstate = 0x3000_0000;
    push_lifted_case("isb_lifted_preserves_arch_state", enc_barrier(0b110), st);

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.pstate = 0x9000_0000;
    push_lifted_case("mrs_nzcv_lifted_reads_flags", enc_mrs_nzcv(RD), st);

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_1234_5678;
    st.pstate = 0xf000_0000;
    push_lifted_case("msr_nzcv_lifted_masks_x1", enc_msr_nzcv(RN), st);

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0xf000_0000;
    push_lifted_case("msr_nzcv_xzr_lifted_clears", enc_msr_nzcv(31), st);

    let mut st = native_state();
    st.x[0] = 0x3333_4444_5555_6666;
    st.fpcr = 0x00c0_0000;
    push_lifted_case("mrs_fpcr_lifted_reads_control", enc_mrs_fpcr(RD), st);

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_00c0_0000;
    st.fpcr = 0;
    push_lifted_case("msr_fpcr_lifted_masks_x1", enc_msr_fpcr(RN), st);

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.fpsr = 0x0800_009f;
    push_lifted_case("mrs_fpsr_lifted_reads_status", enc_mrs_fpsr(RD), st);

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_0800_009f;
    st.fpsr = 0;
    push_lifted_case("msr_fpsr_lifted_masks_x1", enc_msr_fpsr(RN), st);

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.pstate = 0;
    push_lifted_case("cfinv_carry_clear_lifted_sets_c", enc_cfinv(), st);

    let mut st = native_state();
    st.x[0] = 0x6666_7777_8888_9999;
    st.pstate = 0x2000_0000;
    push_lifted_case("cfinv_carry_set_lifted_clears_c", enc_cfinv(), st);

    for nzcv in 0..16 {
        let mut st = native_state();
        st.x[0] = 0x7777_8888_9999_aaaa;
        st.pstate = (nzcv as u64) << 28;
        let label = format!("axflag_nzcv_{nzcv:x}_lifted");
        push_lifted_case(&label, enc_axflag(), st);

        let mut st = native_state();
        st.x[0] = 0x8888_9999_aaaa_bbbb;
        st.pstate = (nzcv as u64) << 28;
        let label = format!("xaflag_nzcv_{nzcv:x}_lifted");
        push_lifted_case(&label, enc_xaflag(), st);
    }

    let mut push_case3 = |label: &str, source: [u32; 3], ops: Vec<OpKind>, st: ArmState| {
        let lowered = lower_aarch64_native_ops(ops)
            .unwrap_or_else(|e| panic!("{label}: native lowering failed: {e}"));
        cases.push((label.into(), source, lowered, st));
    };

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0xffff_ffff_1234_0000;
    st.pstate = 0x5000_0000;
    push_case3(
        "add_w16_imm_masked_neg_one_as_sub_uxth_preserves_flags",
        [
            enc_addsub_imm_regs(0, 1, 0, 0, 1, RD, RN),
            enc_bitfield_rn(0, 0b10, 0, 15, RD),
            NOP,
        ],
        vec![OpKind::Add {
            dst: arm_x(0),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0xffff),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_0080;
    st.pstate = 0x7000_0000;
    push_case3(
        "sign_extend_w8_to_w16_as_sxtb_uxth_preserves_flags",
        [
            enc_bitfield_rn(0, 0b00, 0, 7, RN),
            enc_bitfield_rn(0, 0b10, 0, 15, RD),
            NOP,
        ],
        vec![OpKind::SignExtend {
            dst: arm_x(0),
            src: arm_x(1),
            from_width: OpWidth::W8,
            to_width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_ff00;
    st.x[2] = 0x100;
    st.pstate = 0x9000_0000;
    push_case3(
        "mulu_full_width_low_aliases_src1_high_then_low_preserves_flags",
        [
            enc_dp3_ra_regs(1, 0b110, 0, RD, RN, RM, 31),
            enc_dp3_ra_regs(1, 0b000, 0, RN, RN, RM, 31),
            NOP,
        ],
        vec![OpKind::MulU {
            dst_lo: arm_x(1),
            dst_hi: Some(arm_x(0)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x8000_0000_0000_0001;
    st.x[2] = 3;
    st.pstate = 0x2000_0000;
    push_case3(
        "muls_full_width_low_aliases_src2_high_then_low_preserves_flags",
        [
            enc_dp3_ra_regs(1, 0b010, 0, RD, RN, RM, 31),
            enc_dp3_ra_regs(1, 0b000, 0, RM, RN, RM, 31),
            NOP,
        ],
        vec![OpKind::MulS {
            dst_lo: arm_x(2),
            dst_hi: Some(arm_x(0)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_ffff_ff00;
    st.x[2] = 0x100;
    st.pstate = 0x6000_0000;
    push_case3(
        "mulu_full_width_high_aliases_src1_low_then_high_preserves_flags",
        [
            enc_dp3_ra_regs(1, 0b000, 0, RD, RN, RM, 31),
            enc_dp3_ra_regs(1, 0b110, 0, RN, RN, RM, 31),
            NOP,
        ],
        vec![OpKind::MulU {
            dst_lo: arm_x(0),
            dst_hi: Some(arm_x(1)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0x1234_5678_9abc_def0;
    st.x[2] = 0x12345;
    st.x[3] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x5000_0000;
    push_case3(
        "divu_x_remainder_as_udiv_msub_preserves_flags",
        [
            enc_dp2_regs(1, 0b0010, RN, RM, RD),
            enc_dp3_ra_regs(1, 0b000, 1, 3, RD, RM, RN),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: Some(arm_x(3)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[3] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x5000_0000;
    push_case3(
        "divu_x_two_imms_as_mov_quot_rem_preserves_flags",
        [
            enc_mov_wide(1, 0b10, 0, 14),
            enc_mov_wide_regs(1, 0b10, 0, 2, 3),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: Some(arm_x(3)),
            src1: VReg::Imm(100),
            src2: SrcOperand::Imm(7),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xfedc_ba98_7654_321f;
    st.x[3] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x4000_0000;
    push_case3(
        "divu_x_imm_power_of_two_remainder_as_lsr_and_preserves_flags",
        [
            enc_bitfield(1, 0b10, 3, 63),
            enc_logical_imm_regs(1, 0b00, 1, 0, 2, RN, 3),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: Some(arm_x(3)),
            src1: arm_x(1),
            src2: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_8765_433f;
    st.x[3] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0xd000_0000;
    push_case3(
        "divu_w_imm_power_of_two_remainder_before_quot_alias_preserves_flags",
        [
            enc_logical_imm_regs(0, 0b00, 0, 0, 4, RN, 3),
            enc_bitfield_regs(0, 0b10, 5, 31, RN, RN),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(1),
            rem: Some(arm_x(3)),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(32),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xfedc_ba98_7654_321f;
    st.pstate = 0x1000_0000;
    push_case3(
        "divu_x_imm_power_of_two_remainder_after_rem_alias_preserves_flags",
        [
            enc_bitfield(1, 0b10, 4, 63),
            enc_logical_imm_regs(1, 0b00, 1, 0, 3, RN, RN),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: Some(arm_x(1)),
            src1: arm_x(1),
            src2: SrcOperand::Imm(16),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_00ff;
    st.x[3] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x7000_0000;
    push_case3(
        "divu_w8_imm_power_of_two_remainder_as_lsr_and_preserves_flags",
        [
            enc_bitfield(0, 0b10, 3, 7),
            enc_logical_imm_regs(0, 0b00, 0, 0, 2, RN, 3),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: Some(arm_x(3)),
            src1: arm_x(1),
            src2: SrcOperand::Imm(8),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_ffff_81ff;
    st.x[3] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x3000_0000;
    push_case3(
        "divu_w16_imm_power_of_two_remainder_before_quot_alias_preserves_flags",
        [
            enc_logical_imm_regs(0, 0b00, 0, 0, 6, RN, 3),
            enc_bitfield_regs(0, 0b10, 7, 15, RN, RN),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(1),
            rem: Some(arm_x(3)),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_0000_0080),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_1234_8001;
    st.pstate = 0xb000_0000;
    push_case3(
        "divu_w16_imm_masked_one_as_mov_uxth_preserves_flags",
        [
            enc_mov_reg(0, RD, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, RD, RD),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_0001),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_8765_4321;
    st.x[3] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x1000_0000;
    push_case3(
        "divs_w_imm_masked_one_with_remainder_as_mov_zero_preserves_flags",
        [enc_mov_reg(0, 3, RN), enc_mov_wide(0, 0b10, 0, 0), NOP],
        vec![OpKind::DivS {
            quot: arm_x(3),
            rem: Some(arm_x(0)),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_0000_0001),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_ffff_ff81;
    st.pstate = 0x9000_0000;
    push_case3(
        "divs_x_imm_power_of_two_as_bias_asr_preserves_flags",
        [
            enc_bitfield(1, 0b00, 63, 63),
            enc_addsub_shift_regs(1, 0, 0, 1, 61, RD, RN, RD),
            enc_bitfield_regs(1, 0b00, 3, 63, RD, RD),
        ],
        vec![OpKind::DivS {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_8765_4321;
    st.x[3] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x5000_0000;
    push_case3(
        "divs_w_imm_masked_power_of_two_as_bias_asr_preserves_flags",
        [
            enc_bitfield_regs(0, 0b00, 31, 31, RN, 3),
            enc_addsub_shift_regs(0, 0, 0, 1, 28, 3, RN, 3),
            enc_bitfield_regs(0, 0b00, 4, 31, 3, 3),
        ],
        vec![OpKind::DivS {
            quot: arm_x(3),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm64(0x1_0000_0010),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0xffff_ffff_8000_0100;
    st.x[2] = 0xffff_ffff_ffff_fffb;
    st.x[3] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x7000_0000;
    push_case3(
        "divs_w_remainder_as_sdiv_msub_zero_ext_preserves_flags",
        [
            enc_dp2_regs(0, 0b0011, RN, RM, RD),
            enc_dp3_ra_regs(0, 0b000, 1, 3, RD, RM, RN),
            NOP,
        ],
        vec![OpKind::DivS {
            quot: arm_x(0),
            rem: Some(arm_x(3)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[3] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0xa000_0000;
    push_case3(
        "divs_w8_two_imms_as_mov_quot_rem_preserves_flags",
        [
            enc_mov_wide(0, 0b10, 0, 0xfd),
            enc_mov_wide_regs(0, 0b10, 0, 0xff, 3),
            NOP,
        ],
        vec![OpKind::DivS {
            quot: arm_x(0),
            rem: Some(arm_x(3)),
            src1: VReg::Imm(0xf6),
            src2: SrcOperand::Imm(3),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_8000_0100;
    st.x[3] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x6000_0000;
    push_case3(
        "divs_w_imm_one_with_remainder_as_mov_zero_preserves_flags",
        [enc_mov_reg(0, 3, RN), enc_mov_wide(0, 0b10, 0, 0), NOP],
        vec![OpKind::DivS {
            quot: arm_x(3),
            rem: Some(arm_x(0)),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(1),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_fe80;
    st.pstate = 0x2000_0000;
    push_case3(
        "divu_w8_imm_one_as_mov_uxtb_preserves_flags",
        [
            enc_mov_reg(0, RD, RN),
            enc_bitfield_regs(0, 0b10, 0, 7, RD, RD),
            NOP,
        ],
        vec![OpKind::DivU {
            quot: arm_x(0),
            rem: None,
            src1: arm_x(1),
            src2: SrcOperand::Imm(1),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_ffff_8001;
    st.x[3] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0xa000_0000;
    push_case3(
        "divs_w16_imm_one_with_remainder_as_mov_uxth_zero_preserves_flags",
        [
            enc_mov_reg(0, 3, RN),
            enc_bitfield_regs(0, 0b10, 0, 15, 3, 3),
            enc_mov_wide(0, 0b10, 0, 0),
        ],
        vec![OpKind::DivS {
            quot: arm_x(3),
            rem: Some(arm_x(0)),
            src1: arm_x(1),
            src2: SrcOperand::Imm64(1),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0x0fed_cba9_8765_4321;
    st.x[2] = 0x1f3d;
    st.x[3] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x9000_0000;
    push_case3(
        "divu_x_remainder_quot_aliases_dividend_uses_rem_temp_preserves_flags",
        [
            enc_dp2_regs(1, 0b0010, RN, RM, 3),
            enc_dp3_ra_regs(1, 0b000, 1, 3, 3, RM, RN),
            enc_dp2_regs(1, 0b0010, RN, RM, RN),
        ],
        vec![OpKind::DivU {
            quot: arm_x(1),
            rem: Some(arm_x(3)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xffff_ffff_ffff_ff00;
    st.x[2] = 0xffff_ffff_ffff_fffb;
    st.x[3] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0x2000_0000;
    push_case3(
        "divs_w_remainder_quot_aliases_divisor_uses_rem_temp_zero_ext_preserves_flags",
        [
            enc_dp2_regs(0, 0b0011, RN, RM, 3),
            enc_dp3_ra_regs(0, 0b000, 1, 3, 3, RM, RN),
            enc_dp2_regs(0, 0b0011, RN, RM, RM),
        ],
        vec![OpKind::DivS {
            quot: arm_x(2),
            rem: Some(arm_x(3)),
            src1: arm_x(1),
            src2: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let cmove_w16_source = [
        enc_csel_form(0, 0, 0, RN, RD, 0),
        enc_bitfield_rn(0, 0b10, 0, 15, RD),
        NOP,
    ];
    let cmove_w8_source = [
        enc_csel_form(0, 0, 0, RN, RD, 0),
        enc_bitfield_rn(0, 0b10, 0, 7, RD),
        NOP,
    ];

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0x4000_0000;
    push_case3(
        "cmove_w16_eq_true_opkind_selects_src_uxth_preserves_flags",
        cmove_w16_source,
        vec![OpKind::CMove {
            dst: arm_x(0),
            src: arm_x(1),
            cond: Condition::Eq,
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x2222_3333_4444_5555;
    st.pstate = 0;
    push_case3(
        "cmove_w16_eq_false_opkind_preserves_dst_uxth_and_flags",
        cmove_w16_source,
        vec![OpKind::CMove {
            dst: arm_x(0),
            src: arm_x(1),
            cond: Condition::Eq,
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_44f0;
    st.pstate = 0x4000_0000;
    push_case3(
        "cmove_w8_eq_true_opkind_selects_src_uxtb_preserves_flags",
        cmove_w8_source,
        vec![OpKind::CMove {
            dst: arm_x(0),
            src: arm_x(1),
            cond: Condition::Eq,
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_ee7f;
    st.x[1] = 0x2222_3333_4444_5555;
    st.pstate = 0;
    push_case3(
        "cmove_w8_eq_false_opkind_preserves_dst_uxtb_and_flags",
        cmove_w8_source,
        vec![OpKind::CMove {
            dst: arm_x(0),
            src: arm_x(1),
            cond: Condition::Eq,
            width: OpWidth::W8,
        }],
        st,
    );

    let w32_count_guard = |opcode2: u32| {
        [
            enc_dp2(0, opcode2),
            enc_test_branch_rt(0, 5, 8, RM),
            enc_logical_shift_regs(0, 0b01, 0, 0, 0, RD, 31, 31),
        ]
    };
    let w32_sar_guard = [
        enc_dp2(0, 0b1010),
        enc_test_branch_rt(0, 5, 8, RM),
        enc_bitfield_rn(0, 0b00, 31, 31, RD),
    ];

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_1234_5678;
    st.x[2] = 4;
    st.pstate = 0xa000_0000;
    push_case3(
        "shl_w_reg_count_below32_zero_ext_preserves_flags",
        [enc_dp2(0, 0b1000), NOP, NOP],
        vec![OpKind::Shl {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_1234_5678;
    st.x[2] = 32;
    st.pstate = 0x5000_0000;
    push_case3(
        "shl_w_reg_count_32_zero_ext_preserves_flags",
        w32_count_guard(0b1000),
        vec![OpKind::Shl {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_8765_4321;
    st.x[2] = 7;
    st.pstate = 0x6000_0000;
    push_case3(
        "shr_w_reg_count_below32_zero_ext_preserves_flags",
        [enc_dp2(0, 0b1001), NOP, NOP],
        vec![OpKind::Shr {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_8765_4321;
    st.x[2] = 33;
    st.pstate = 0x9000_0000;
    push_case3(
        "shr_w_reg_count_33_zero_ext_preserves_flags",
        w32_count_guard(0b1001),
        vec![OpKind::Shr {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_8765_4321;
    st.x[2] = 7;
    st.pstate = 0x2000_0000;
    push_case3(
        "sar_w_reg_count_below32_zero_ext_preserves_flags",
        [enc_dp2(0, 0b1010), NOP, NOP],
        vec![OpKind::Sar {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0xffff_ffff_8000_0000;
    st.x[2] = 33;
    st.pstate = 0x4000_0000;
    push_case3(
        "sar_w_reg_count_33_sign_fill_preserves_flags",
        w32_sar_guard,
        vec![OpKind::Sar {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x0123_4567_89ab_cdef;
    st.x[2] = 13;
    st.pstate = 0x3000_0000;
    push_case3(
        "rol_x_reg_as_neg_count_rorv_preserves_flags",
        [
            enc_addsub_shift_regs(1, 1, 0, 0, 0, RD, 31, RM),
            enc_dp2_regs(1, 0b1011, RN, RD, RD),
            NOP,
        ],
        vec![OpKind::Rol {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xaaaa_bbbb_89ab_cdef;
    st.x[2] = 9;
    st.pstate = 0xc000_0000;
    push_case3(
        "rol_w_reg_as_neg_count_rorv_zero_ext_preserves_flags",
        [
            enc_addsub_shift_regs(0, 1, 0, 0, 0, RD, 31, RM),
            enc_dp2_regs(0, 0b1011, RN, RD, RD),
            NOP,
        ],
        vec![OpKind::Rol {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.x[2] = 41;
    st.pstate = 0x5000_0000;
    push_case3(
        "rol_x_reg_dst_is_count_preserves_flags",
        [
            enc_addsub_shift_regs(1, 1, 0, 0, 0, RM, 31, RM),
            enc_dp2_regs(1, 0b1011, RN, RM, RM),
            NOP,
        ],
        vec![OpKind::Rol {
            dst: arm_x(2),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let cwd_w8 = [
        enc_bitfield_rn(0, 0b00, 7, 7, RN),
        enc_bitfield_rn(0, 0b10, 0, 7, RD),
        NOP,
    ];

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_0000_007f;
    st.pstate = 0x3000_0000;
    push_case3(
        "cwd_w8_sign_clear_as_sbfm_uxtb_preserves_flags",
        cwd_w8,
        vec![OpKind::Cwd {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1111_2222_0000_0080;
    st.pstate = 0xc000_0000;
    push_case3(
        "cwd_w8_sign_set_as_sbfm_uxtb_preserves_flags",
        cwd_w8,
        vec![OpKind::Cwd {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0x5000_0000;
    push_case3(
        "cwd_w8_imm_sign_set_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0xff), NOP, NOP],
        vec![OpKind::Cwd {
            dst: arm_x(0),
            src: VReg::Imm(0x80),
            width: OpWidth::W8,
        }],
        st,
    );

    let cwd_w16 = [
        enc_bitfield_rn(0, 0b00, 15, 15, RN),
        enc_bitfield_rn(0, 0b10, 0, 15, RD),
        NOP,
    ];

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_0000_7fff;
    st.pstate = 0x6000_0000;
    push_case3(
        "cwd_w16_sign_clear_as_sbfm_uxth_preserves_flags",
        cwd_w16,
        vec![OpKind::Cwd {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1111_2222_0000_8000;
    st.pstate = 0x9000_0000;
    push_case3(
        "cwd_w16_sign_set_as_sbfm_uxth_preserves_flags",
        cwd_w16,
        vec![OpKind::Cwd {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_ff19;
    st.pstate = 0x7000_0000;
    push_case3(
        "shl_w8_imm_as_ubfiz_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 29, 4, RN), NOP, NOP],
        vec![OpKind::Shl {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(3),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x4000_0000;
    push_case3(
        "shl_w8_imm_src_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0xf0), NOP, NOP],
        vec![OpKind::Shl {
            dst: arm_x(0),
            src: VReg::Imm(0x1f),
            amount: SrcOperand::Imm(4),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x1111_2222_3333_ace0;
    st.pstate = 0x2000_0000;
    push_case3(
        "shr_w16_imm_as_ubfx_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 5, 15, RN), NOP, NOP],
        vec![OpKind::Shr {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(5),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0x8000_0000;
    push_case3(
        "shr_w8_imm_src_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0xf), NOP, NOP],
        vec![OpKind::Shr {
            dst: arm_x(0),
            src: VReg::Imm(0x1f0),
            amount: SrcOperand::Imm(4),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x1111_2222_3333_44f0;
    st.pstate = 0x5000_0000;
    push_case3(
        "sar_w8_imm_as_sbfm_uxtb_preserves_flags",
        [
            enc_bitfield_rn(0, 0b00, 3, 7, RN),
            enc_bitfield_rn(0, 0b10, 0, 7, RD),
            NOP,
        ],
        vec![OpKind::Sar {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(3),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0x2000_0000;
    push_case3(
        "sar_w8_imm_src_negative_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0xff), NOP, NOP],
        vec![OpKind::Sar {
            dst: arm_x(0),
            src: VReg::Imm(0xf0),
            amount: SrcOperand::Imm(4),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x1111_2222_3333_ffff;
    st.pstate = 0xa000_0000;
    push_case3(
        "shl_w16_imm_count_above_width_as_zero_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0), NOP, NOP],
        vec![OpKind::Shl {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(17),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let ror_w8 = [
        enc_bitfield_rn(0, 0b10, 0, 7, RN),
        enc_bitfield_rn(0, 0b01, 24, 7, RD),
        enc_bitfield_rn(0, 0b10, 3, 10, RD),
    ];

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0x1111_2222_3333_4481;
    st.pstate = 0x3000_0000;
    push_case3(
        "ror_w8_imm_as_duplicate_extract_preserves_flags",
        ror_w8,
        vec![OpKind::Ror {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(3),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0xb000_0000;
    push_case3(
        "ror_w8_imm_src_wrap_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0xc0), NOP, NOP],
        vec![OpKind::Ror {
            dst: arm_x(0),
            src: VReg::Imm(0x81),
            amount: SrcOperand::Imm(1),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let rol_w16 = [
        enc_bitfield_rn(0, 0b10, 0, 15, RN),
        enc_bitfield_rn(0, 0b01, 16, 15, RD),
        enc_bitfield_rn(0, 0b10, 11, 26, RD),
    ];

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0x2222_3333_4444_a531;
    st.pstate = 0xc000_0000;
    push_case3(
        "rol_w16_imm_as_subword_ror_preserves_flags",
        rol_w16,
        vec![OpKind::Rol {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(5),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x7000_0000;
    push_case3(
        "rol_w8_imm_src_wrap_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0x3), NOP, NOP],
        vec![OpKind::Rol {
            dst: arm_x(0),
            src: VReg::Imm(0x81),
            amount: SrcOperand::Imm(1),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_ff81;
    st.pstate = 0x6000_0000;
    push_case3(
        "ror_w8_imm_masked_zero_as_uxtb_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 7, RN), NOP, NOP],
        vec![OpKind::Ror {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(8),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let ror_w16_reg_in_place = [
        enc_bitfield_regs(0, 0b01, 16, 15, RN, RN),
        enc_dp2_regs(0, 0b1011, RN, RM, RN),
        enc_bitfield_regs(0, 0b10, 0, 15, RN, RN),
    ];

    let mut st = native_state();
    st.x[1] = 0x2222_3333_4444_a531;
    st.x[2] = 21;
    st.pstate = 0x9000_0000;
    push_case3(
        "ror_w16_reg_in_place_as_duplicate_rorv_uxth_preserves_flags",
        ror_w16_reg_in_place,
        vec![OpKind::Ror {
            dst: arm_x(1),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(2)),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let ror_w16_reg_self_count = [
        enc_bitfield_regs(0, 0b01, 16, 15, RN, RN),
        enc_dp2_regs(0, 0b1011, RN, RN, RN),
        enc_bitfield_regs(0, 0b10, 0, 15, RN, RN),
    ];

    let mut st = native_state();
    st.x[1] = 0x3333_4444_5555_a531;
    st.pstate = 0x4000_0000;
    push_case3(
        "ror_w16_reg_self_count_in_place_preserves_flags",
        ror_w16_reg_self_count,
        vec![OpKind::Ror {
            dst: arm_x(1),
            src: arm_x(1),
            amount: SrcOperand::Reg(arm_x(1)),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1122_3344_5566_7788;
    st.pstate = 0xb000_0000;
    push_case3(
        "bswap_w16_opkind_as_rev16_uxth_preserves_flags",
        [
            enc_dp1(0, 0b000001),
            enc_bitfield_rn(0, 0b10, 0, 15, RD),
            NOP,
        ],
        vec![OpKind::Bswap {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.pstate = 0xb000_0000;
    push_case3(
        "bswap_w16_imm_as_movz_swapped_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0x3412), NOP, NOP],
        vec![OpKind::Bswap {
            dst: arm_x(0),
            src: VReg::Imm(0x1_0000_1234),
            width: OpWidth::W16,
        }],
        st,
    );

    let rev16_lo = VReg::virt(0);
    let rev16_hi = VReg::virt(1);
    let rev16_lo_shifted = VReg::virt(2);
    let rev16_hi_shifted = VReg::virt(3);
    let rev16_src = VReg::Imm(0x12);
    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.pstate = 0x6000_0000;
    drop(push_case3);

    let rev16_label = "rev16_x_lifted_imm_src_as_movz_preserves_flags";
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::And {
            dst: rev16_lo,
            src1: rev16_src,
            src2: SrcOperand::Imm64(0x00ff_00ff_00ff_00ff),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::And {
            dst: rev16_hi,
            src1: rev16_src,
            src2: SrcOperand::Imm64(0xff00_ff00_ff00_ff00_u64 as i64),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Shl {
            dst: rev16_lo_shifted,
            src: rev16_lo,
            amount: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Shr {
            dst: rev16_hi_shifted,
            src: rev16_hi,
            amount: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: arm_x(0),
            src1: rev16_lo_shifted,
            src2: SrcOperand::Reg(rev16_hi_shifted),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| panic!("{rev16_label}: native lowering failed: {e}"));
    cases.push((
        rev16_label.into(),
        [enc_mov_wide(1, 0b10, 0, 0x1200), NOP, NOP],
        lowered,
        st,
    ));

    let rev32_lo_rev = VReg::virt(0);
    let rev32_hi = VReg::virt(1);
    let rev32_hi_rev = VReg::virt(2);
    let rev32_hi_shifted = VReg::virt(3);
    let rev32_src = VReg::Imm(0x1200_0000);
    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.pstate = 0xa000_0000;
    let rev32_label = "rev32_x_lifted_imm_src_as_movz_preserves_flags";
    let lowered = lower_aarch64_native_ops_same_pc(vec![
        OpKind::Bswap {
            dst: rev32_lo_rev,
            src: rev32_src,
            width: OpWidth::W32,
        },
        OpKind::Shr {
            dst: rev32_hi,
            src: rev32_src,
            amount: SrcOperand::Imm(32),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Bswap {
            dst: rev32_hi_rev,
            src: rev32_hi,
            width: OpWidth::W32,
        },
        OpKind::Shl {
            dst: rev32_hi_shifted,
            src: rev32_hi_rev,
            amount: SrcOperand::Imm(32),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
        OpKind::Or {
            dst: arm_x(0),
            src1: rev32_hi_shifted,
            src2: SrcOperand::Reg(rev32_lo_rev),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        },
    ])
    .unwrap_or_else(|e| panic!("{rev32_label}: native lowering failed: {e}"));
    cases.push((
        rev32_label.into(),
        [enc_mov_wide(1, 0b10, 0, 0x12), NOP, NOP],
        lowered,
        st,
    ));

    let mut push_case3 = |label: &str, source: [u32; 3], ops: Vec<OpKind>, st: ArmState| {
        let lowered = lower_aarch64_native_ops(ops)
            .unwrap_or_else(|e| panic!("{label}: native lowering failed: {e}"));
        cases.push((label.into(), source, lowered, st));
    };

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0x1111_2222_3333_4444;
    st.pstate = 0xf000_0000;
    push_case3(
        "lea_direct_as_add_zero_preserves_flags",
        [enc_addsub_imm_regs(1, 0, 0, 0, 0, 0, RN), NOP, NOP],
        vec![OpKind::Lea {
            dst: arm_x(0),
            addr: Address::Direct(arm_x(1)),
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0x2222_3333_4444_5000;
    st.pstate = 0xa000_0000;
    push_case3(
        "lea_base_positive_offset_as_add_imm_preserves_flags",
        [enc_addsub_imm_regs(1, 0, 0, 0, 0x123, 0, RN), NOP, NOP],
        vec![OpKind::Lea {
            dst: arm_x(0),
            addr: Address::BaseOffset {
                base: arm_x(1),
                offset: 0x123,
                disp_size: DispSize::Auto,
            },
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0x3333_4444_5555_6000;
    st.pstate = 0x5000_0000;
    push_case3(
        "lea_base_negative_offset_as_sub_imm_preserves_flags",
        [enc_addsub_imm_regs(1, 1, 0, 1, 2, 0, RN), NOP, NOP],
        vec![OpKind::Lea {
            dst: arm_x(0),
            addr: Address::BaseOffset {
                base: arm_x(1),
                offset: -0x2000,
                disp_size: DispSize::Auto,
            },
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0x4444_5555_6666_7000;
    st.x[2] = 0x1234;
    st.pstate = 0xc000_0000;
    push_case3(
        "lea_base_index_scale_disp_preserves_flags",
        [
            enc_addsub_shift_regs(1, 0, 0, 0, 2, 0, RN, RM),
            enc_addsub_imm_regs(1, 1, 0, 0, 0x20, 0, 0),
            NOP,
        ],
        vec![OpKind::Lea {
            dst: arm_x(0),
            addr: Address::BaseIndexScale {
                base: Some(arm_x(1)),
                index: arm_x(2),
                scale: 4,
                disp: -0x20,
                disp_size: DispSize::Auto,
            },
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[2] = 0x0123_4567_89ab_cdef;
    st.pstate = 0x3000_0000;
    push_case3(
        "lea_index_scale_without_base_preserves_flags",
        [enc_addsub_shift_regs(1, 0, 0, 0, 3, 0, 31, RM), NOP, NOP],
        vec![OpKind::Lea {
            dst: arm_x(0),
            addr: Address::BaseIndexScale {
                base: None,
                index: arm_x(2),
                scale: 8,
                disp: 0,
                disp_size: DispSize::Auto,
            },
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.pstate = 0x6000_0000;
    push_case3(
        "lea_absolute_small_as_movz_preserves_flags",
        [enc_mov_wide(1, 0b10, 0, 0x1234), NOP, NOP],
        vec![OpKind::Lea {
            dst: arm_x(0),
            addr: Address::Absolute(0x1234),
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0000_1111_2222_3333;
    st.pstate = 0x9000_0000;
    push_case3(
        "lea_pcrel_small_as_movz_preserves_flags",
        [enc_mov_wide(1, 0b10, 0, 0x1234), NOP, NOP],
        vec![OpKind::Lea {
            dst: arm_x(0),
            addr: Address::PcRel {
                offset: 0x234,
                disp_size: DispSize::Auto,
                base: Some(0x1000),
            },
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x1111_2222_3333_4444;
    st.x[1] = 0x8000_0000_0000_1000;
    st.pstate = 0x7000_0000;
    push_case3(
        "ctz_x_as_rbit_clz_preserves_flags",
        [
            enc_dp1_regs(1, 0b000000, 1, 0),
            enc_dp1_regs(1, 0b000100, 0, 0),
            NOP,
        ],
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.pstate = 0x9000_0000;
    push_case3(
        "ctz_x_imm_zero_as_width_preserves_flags",
        [enc_mov_wide(1, 0b10, 0, 64), NOP, NOP],
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: VReg::Imm(0),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_2222_3333;
    st.x[1] = 0xaaaa_bbbb_0000_0080;
    st.pstate = 0x6000_0000;
    push_case3(
        "ctz_w_as_rbit_clz_zero_ext_preserves_flags",
        [
            enc_dp1_regs(0, 0b000000, 1, 0),
            enc_dp1_regs(0, 0b000100, 0, 0),
            NOP,
        ],
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W32,
        }],
        st,
    );

    let clz_w8 = [
        enc_bitfield_regs(0, 0b10, 8, 7, RN, RD),
        enc_logical_imm(0, 0b01, 0, 9, 0, RD),
        enc_dp1_regs(0, 0b000100, RD, RD),
    ];

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.x[1] = 0xffff_ffff_ffff_ff08;
    st.pstate = 0x6000_0000;
    push_case3(
        "clz_w8_as_aligned_sentinel_clz_preserves_flags",
        clz_w8,
        vec![OpKind::Clz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xbbbb_cccc_dddd_eeee;
    st.x[1] = 0xffff_ffff_ffff_ff00;
    st.pstate = 0x1000_0000;
    push_case3(
        "clz_w8_zero_low8_as_width_preserves_flags",
        clz_w8,
        vec![OpKind::Clz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let clz_w16 = [
        enc_bitfield_regs(0, 0b10, 16, 15, RN, RD),
        enc_logical_imm(0, 0b01, 0, 17, 0, RD),
        enc_dp1_regs(0, 0b000100, RD, RD),
    ];

    let mut st = native_state();
    st.x[0] = 0xcccc_dddd_eeee_ffff;
    st.x[1] = 0xffff_ffff_0000_0080;
    st.pstate = 0x9000_0000;
    push_case3(
        "clz_w16_as_aligned_sentinel_clz_preserves_flags",
        clz_w16,
        vec![OpKind::Clz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.x[1] = 0xffff_ffff_0000_0000;
    st.pstate = 0xe000_0000;
    push_case3(
        "clz_w16_zero_low16_as_width_preserves_flags",
        clz_w16,
        vec![OpKind::Clz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0xb000_0000;
    push_case3(
        "clz_w16_imm_masked_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 8), NOP, NOP],
        vec![OpKind::Clz {
            dst: arm_x(0),
            src: VReg::Imm(0x1_0000_0080),
            width: OpWidth::W16,
        }],
        st,
    );

    let ctz_w8 = [
        enc_logical_imm(0, 0b01, 0, 24, 0, RN),
        enc_dp1_regs(0, 0b000000, RD, RD),
        enc_dp1_regs(0, 0b000100, RD, RD),
    ];

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_ffff_ff80;
    st.pstate = 0x3000_0000;
    push_case3(
        "ctz_w8_as_sentinel_rbit_clz_preserves_flags",
        ctz_w8,
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_eeee_ffff_0000;
    st.pstate = 0x7000_0000;
    push_case3(
        "ctz_w8_imm_masked_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 7), NOP, NOP],
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: VReg::Imm(0x1_0000_0080),
            width: OpWidth::W8,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0xffff_ffff_ffff_ff00;
    st.pstate = 0xc000_0000;
    push_case3(
        "ctz_w8_zero_low8_as_width_preserves_flags",
        ctz_w8,
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W8,
        }],
        st,
    );

    let ctz_w16 = [
        enc_logical_imm(0, 0b01, 0, 16, 0, RN),
        enc_dp1_regs(0, 0b000000, RD, RD),
        enc_dp1_regs(0, 0b000100, RD, RD),
    ];

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.x[1] = 0xffff_ffff_0000_0080;
    st.pstate = 0xb000_0000;
    push_case3(
        "ctz_w16_as_sentinel_rbit_clz_preserves_flags",
        ctz_w16,
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_0000_1111_2222;
    st.x[1] = 0xffff_ffff_0000_0000;
    st.pstate = 0x4000_0000;
    push_case3(
        "ctz_w16_zero_low16_as_width_preserves_flags",
        ctz_w16,
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W16,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x9999_aaaa_bbbb_cccc;
    st.x[1] = 0;
    st.pstate = 0xa000_0000;
    push_case3(
        "ctz_x_zero_as_width_preserves_flags",
        [
            enc_dp1_regs(1, 0b000000, 1, 0),
            enc_dp1_regs(1, 0b000100, 0, 0),
            NOP,
        ],
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0000_0000_0000_0040;
    st.pstate = 0xd000_0000;
    push_case3(
        "ctz_in_place_as_rbit_clz_preserves_flags",
        [
            enc_dp1_regs(1, 0b000000, 0, 0),
            enc_dp1_regs(1, 0b000100, 0, 0),
            NOP,
        ],
        vec![OpKind::Ctz {
            dst: arm_x(0),
            src: arm_x(0),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x2222_3333_4444_5555;
    st.x[1] = 0x8000_0000_0000_1000;
    st.pstate = 0x3000_0000;
    push_case3(
        "bsf_x_as_rbit_clz_ubfx_preserves_flags",
        [
            enc_dp1_regs(1, 0b000000, 1, 0),
            enc_dp1_regs(1, 0b000100, 0, 0),
            enc_bitfield_rn(1, 0b10, 0, 5, 0),
        ],
        vec![OpKind::Bsf {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_3333_4444;
    st.x[1] = 0xaaaa_bbbb_0000_0080;
    st.pstate = 0x8000_0000;
    push_case3(
        "bsf_w_as_rbit_clz_ubfx_zero_ext_preserves_flags",
        [
            enc_dp1_regs(0, 0b000000, 1, 0),
            enc_dp1_regs(0, 0b000100, 0, 0),
            enc_bitfield_rn(0, 0b10, 0, 4, 0),
        ],
        vec![OpKind::Bsf {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_ffff_0000_1111;
    st.pstate = 0xb000_0000;
    push_case3(
        "bsf_w16_imm_masked_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 7), NOP, NOP],
        vec![OpKind::Bsf {
            dst: arm_x(0),
            src: VReg::Imm(0x1_0000_0080),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x4444_5555_6666_7777;
    st.x[1] = 0;
    st.pstate = 0x9000_0000;
    push_case3(
        "bsf_x_zero_as_zero_preserves_flags",
        [
            enc_dp1_regs(1, 0b000000, 1, 0),
            enc_dp1_regs(1, 0b000100, 0, 0),
            enc_bitfield_rn(1, 0b10, 0, 5, 0),
        ],
        vec![OpKind::Bsf {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.pstate = 0x6000_0000;
    push_case3(
        "bsf_x_imm_zero_as_movz_preserves_flags",
        [enc_mov_wide(1, 0b10, 0, 0), NOP, NOP],
        vec![OpKind::Bsf {
            dst: arm_x(0),
            src: VReg::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x5555_6666_7777_8888;
    st.x[1] = 0x8000_0000_0000_1000;
    st.pstate = 0x6000_0000;
    push_case3(
        "bsr_x_as_orr_clz_eor_mask_preserves_flags",
        [
            enc_logical_imm(1, 0b01, 1, 0, 0, RN),
            enc_dp1_regs(1, 0b000100, 0, 0),
            enc_logical_imm(1, 0b10, 1, 0, 5, RD),
        ],
        vec![OpKind::Bsr {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_6666_7777;
    st.x[1] = 0xaaaa_bbbb_0000_0080;
    st.pstate = 0x5000_0000;
    push_case3(
        "bsr_w_as_orr_clz_eor_mask_zero_ext_preserves_flags",
        [
            enc_logical_imm(0, 0b01, 0, 0, 0, RN),
            enc_dp1_regs(0, 0b000100, 0, 0),
            enc_logical_imm(0, 0b10, 0, 0, 4, RD),
        ],
        vec![OpKind::Bsr {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_9999_aaaa;
    st.pstate = 0x3000_0000;
    push_case3(
        "bsr_w16_imm_masked_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 15), NOP, NOP],
        vec![OpKind::Bsr {
            dst: arm_x(0),
            src: VReg::Imm(0x1_0000_8000),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x7777_8888_9999_aaaa;
    st.x[1] = 0;
    st.pstate = 0xc000_0000;
    push_case3(
        "bsr_x_zero_as_zero_preserves_flags",
        [
            enc_logical_imm(1, 0b01, 1, 0, 0, RN),
            enc_dp1_regs(1, 0b000100, 0, 0),
            enc_logical_imm(1, 0b10, 1, 0, 5, RD),
        ],
        vec![OpKind::Bsr {
            dst: arm_x(0),
            src: arm_x(1),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x8888_9999_aaaa_bbbb;
    st.pstate = 0x9000_0000;
    push_case3(
        "bsr_x_imm_zero_as_movz_preserves_flags",
        [enc_mov_wide(1, 0b10, 0, 0), NOP, NOP],
        vec![OpKind::Bsr {
            dst: arm_x(0),
            src: VReg::Imm(0),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xa000_0000;
    push_case3(
        "shrd_x_imm_as_extract_preserves_flags",
        [enc_extract(1, 1, 0, 13), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(13),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = u64::MAX;
    st.pstate = 0xe000_0000;
    push_case3(
        "shrd_x_all_ones_imm_src_as_shift_orr_preserves_flags",
        [enc_extract(1, RN, RD, 13), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: VReg::Imm(-1),
            amount: SrcOperand::Imm(13),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = 0x1200;
    st.pstate = 0x2000_0000;
    push_case3(
        "shrd_x_imm_src_zero_insert_as_lsr_preserves_flags",
        [enc_extract(1, RN, RD, 8), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: VReg::Imm(0x1200),
            amount: SrcOperand::Imm(8),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = 0x1f;
    st.pstate = 0x6000_0000;
    push_case3(
        "shrd_x_encodable_imm_src_as_shift_orr_preserves_flags",
        [enc_extract(1, RN, RD, 13), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: VReg::Imm(0x1f),
            amount: SrcOperand::Imm(13),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0x5000_0000;
    push_case3(
        "shld_x_imm_as_extract_preserves_flags",
        [enc_extract(1, 0, 1, 51), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(13),
            width: OpWidth::W64,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_0123_4567;
    st.x[1] = 0xffff_ffff;
    st.pstate = 0x7000_0000;
    push_case3(
        "shld_w_all_ones_imm_src_as_shift_orr_preserves_flags",
        [enc_extract(0, RD, RN, 25), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: VReg::Imm(-1),
            amount: SrcOperand::Imm(7),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_0123_4567;
    st.x[1] = 0x00ff;
    st.pstate = 0x8000_0000;
    push_case3(
        "shld_w_imm_src_zero_insert_as_lsl_preserves_flags",
        [enc_extract(0, RD, RN, 25), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: VReg::Imm(0x00ff),
            amount: SrcOperand::Imm(7),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_0123_4567;
    st.x[1] = 0xe000_0000;
    st.pstate = 0xd000_0000;
    push_case3(
        "shld_w_encodable_imm_src_as_shift_orr_preserves_flags",
        [enc_extract(0, RD, RN, 25), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: VReg::Imm(0xe000_0000),
            amount: SrcOperand::Imm(7),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_0123_4567;
    st.x[1] = 0xcccc_dddd_89ab_cdef;
    st.pstate = 0xc000_0000;
    push_case3(
        "shrd_w_imm_as_extract_zero_ext_preserves_flags",
        [enc_extract(0, 1, 0, 7), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(7),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_0123_4567;
    st.x[1] = 0xcccc_dddd_89ab_cdef;
    st.pstate = 0x3000_0000;
    push_case3(
        "shld_w_imm_as_extract_zero_ext_preserves_flags",
        [enc_extract(0, 0, 1, 25), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(7),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_0123_4567;
    st.x[1] = 0xcccc_dddd_0000_abcd;
    st.pstate = 0x6000_0000;
    push_case3(
        "shld_w16_imm_as_shift_bfxil_uxth_preserves_flags",
        [
            enc_bitfield_rn(0, 0b10, 27, 26, RD),
            enc_bitfield_rn(0, 0b01, 11, 15, RN),
            enc_bitfield_rn(0, 0b10, 0, 15, RD),
        ],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(5),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_0123_4567;
    st.x[1] = 0x07ff;
    st.pstate = 0xf000_0000;
    push_case3(
        "shld_w16_imm_src_zero_insert_as_lsl_uxth_preserves_flags",
        [
            enc_bitfield_rn(0, 0b10, 27, 26, RD),
            enc_bitfield_rn(0, 0b10, 0, 15, RD),
            NOP,
        ],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: VReg::Imm(0x07ff),
            amount: SrcOperand::Imm(5),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_0123_4567;
    st.x[1] = 0xf800;
    st.pstate = 0x3000_0000;
    push_case3(
        "shld_w16_encodable_imm_src_as_shift_orr_uxth_preserves_flags",
        [
            enc_bitfield_rn(0, 0b10, 27, 26, RD),
            enc_logical_imm(0, 0b01, 0, 0, 4, RD),
            enc_bitfield_rn(0, 0b10, 0, 15, RD),
        ],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: VReg::Imm(0xf800),
            amount: SrcOperand::Imm(5),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.x[1] = 0xcccc_dddd_0123_4567;
    st.pstate = 0x1000_0000;
    push_case3(
        "shld_w16_masked_zero_as_uxth_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 15, RD), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(32),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_12a5;
    st.x[1] = 0xcccc_dddd_eeee_00e7;
    st.pstate = 0x7000_0000;
    push_case3(
        "shld_w8_imm_as_shift_bfxil_uxtb_preserves_flags",
        [
            enc_bitfield_rn(0, 0b10, 29, 28, RD),
            enc_bitfield_rn(0, 0b01, 5, 7, RN),
            enc_bitfield_rn(0, 0b10, 0, 7, RD),
        ],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(3),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_34c9;
    st.x[1] = 0xcccc_dddd_eeee_0015;
    st.pstate = 0x4000_0000;
    push_case3(
        "shld_w8_masked_zero_as_uxtb_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 7, RD), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(32),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_12a5;
    st.x[1] = 0xcccc_dddd_eeee_00e7;
    st.pstate = 0x8000_0000;
    push_case3(
        "shrd_w8_imm_as_shift_bfi_uxtb_preserves_flags",
        [
            enc_bitfield_rn(0, 0b10, 3, 31, RD),
            enc_bitfield_rn(0, 0b01, 27, 2, RN),
            enc_bitfield_rn(0, 0b10, 0, 7, RD),
        ],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(3),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_12a5;
    st.x[1] = 0x18;
    st.pstate = 0x9000_0000;
    push_case3(
        "shrd_w8_imm_src_zero_insert_as_lsr_uxtb_preserves_flags",
        [
            enc_bitfield_rn(0, 0b10, 3, 31, RD),
            enc_bitfield_rn(0, 0b10, 0, 7, RD),
            NOP,
        ],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: VReg::Imm(0x18),
            amount: SrcOperand::Imm(3),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_12a5;
    st.x[1] = 0x07;
    st.pstate = 0xb000_0000;
    push_case3(
        "shrd_w8_encodable_imm_src_as_shift_orr_uxtb_preserves_flags",
        [
            enc_bitfield_rn(0, 0b10, 3, 31, RD),
            enc_logical_imm(0, 0b01, 0, 27, 2, RD),
            enc_bitfield_rn(0, 0b10, 0, 7, RD),
        ],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: VReg::Imm(0x07),
            amount: SrcOperand::Imm(3),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_ffff_34c9;
    st.x[1] = 0xcccc_dddd_eeee_0015;
    st.pstate = 0xe000_0000;
    push_case3(
        "shrd_w8_masked_zero_as_uxtb_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 7, RD), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(32),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_0123_4567;
    st.x[1] = 0xcccc_dddd_0000_abcd;
    st.pstate = 0x2000_0000;
    push_case3(
        "shrd_w16_imm_as_shift_bfi_uxth_preserves_flags",
        [
            enc_bitfield_rn(0, 0b10, 5, 31, RD),
            enc_bitfield_rn(0, 0b01, 21, 4, RN),
            enc_bitfield_rn(0, 0b10, 0, 15, RD),
        ],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(5),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.x[1] = 0xcccc_dddd_0123_4567;
    st.pstate = 0xd000_0000;
    push_case3(
        "shrd_w16_masked_zero_as_uxth_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 15, RD), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(32),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_7654_abcd;
    st.pstate = 0xb000_0000;
    push_case3(
        "shld_w16_full_count_alias_as_uxth_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 15, RD), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: arm_x(0),
            amount: SrcOperand::Imm(16),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xeeee_dddd_cccc_12a5;
    st.pstate = 0xa000_0000;
    push_case3(
        "shld_w8_full_count_alias_as_uxtb_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 7, RD), NOP, NOP],
        vec![OpKind::Shld {
            dst: arm_x(0),
            src: arm_x(0),
            amount: SrcOperand::Imm(8),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xdddd_cccc_bbbb_dcba;
    st.pstate = 0x5000_0000;
    push_case3(
        "shrd_w16_full_count_alias_as_uxth_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 15, RD), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(0),
            amount: SrcOperand::Imm(16),
            width: OpWidth::W16,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_bbbb_aaaa_34c9;
    st.pstate = 0xc000_0000;
    push_case3(
        "shrd_w8_full_count_alias_as_uxtb_preserves_flags",
        [enc_bitfield_rn(0, 0b10, 0, 7, RD), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(0),
            amount: SrcOperand::Imm(8),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xcccc_bbbb_aaaa_34c9;
    st.pstate = 0x2000_0000;
    push_case3(
        "shrd_w8_full_count_imm_src_as_movz_preserves_flags",
        [enc_mov_wide(0, 0b10, 0, 0x00e7), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: VReg::Imm(0x1e7),
            amount: SrcOperand::Imm(8),
            width: OpWidth::W8,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.x[1] = 0xcccc_dddd_0123_4567;
    st.pstate = 0x9000_0000;
    push_case3(
        "shrd_w_masked_zero_as_self_mov_zero_ext_preserves_flags",
        [enc_logical_shift_regs(0, 0b01, 0, 0, 0, 0, 31, 0), NOP, NOP],
        vec![OpKind::Shrd {
            dst: arm_x(0),
            src: arm_x(1),
            amount: SrcOperand::Imm(32),
            width: OpWidth::W32,
            flags: FlagUpdate::None,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0x0123_4567_89ab_cdef;
    st.x[1] = 0xfedc_ba98_7654_3210;
    st.pstate = 0xb000_0000;
    push_case3(
        "xchg_x_as_eor_swap_preserves_flags",
        [
            enc_logical_shift_regs(1, 0b10, 0, 0, 0, 0, 0, 1),
            enc_logical_shift_regs(1, 0b10, 0, 0, 0, 1, 0, 1),
            enc_logical_shift_regs(1, 0b10, 0, 0, 0, 0, 0, 1),
        ],
        vec![OpKind::Xchg {
            reg1: arm_x(0),
            reg2: arm_x(1),
            width: OpWidth::W64,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xaaaa_bbbb_0123_4567;
    st.x[1] = 0xcccc_dddd_89ab_cdef;
    st.pstate = 0x4000_0000;
    push_case3(
        "xchg_w_as_eor_swap_zero_ext_preserves_flags",
        [
            enc_logical_shift_regs(0, 0b10, 0, 0, 0, 0, 0, 1),
            enc_logical_shift_regs(0, 0b10, 0, 0, 0, 1, 0, 1),
            enc_logical_shift_regs(0, 0b10, 0, 0, 0, 0, 0, 1),
        ],
        vec![OpKind::Xchg {
            reg1: arm_x(0),
            reg2: arm_x(1),
            width: OpWidth::W32,
        }],
        st,
    );

    let mut st = native_state();
    st.x[0] = 0xffff_ffff_89ab_cdef;
    st.pstate = 0x9000_0000;
    push_case3(
        "xchg_same_w_as_self_mov_zero_ext_preserves_flags",
        [enc_logical_shift_regs(0, 0b01, 0, 0, 0, 0, 31, 0), NOP, NOP],
        vec![OpKind::Xchg {
            reg1: arm_x(0),
            reg2: arm_x(0),
            width: OpWidth::W32,
        }],
        st,
    );

    let source_cases: Vec<(u32, u32, u32, ArmState)> = cases
        .iter()
        .map(|(_, source, _, st)| (source[0], source[1], source[2], *st))
        .collect();
    let lowered_cases: Vec<(u32, u32, u32, ArmState)> = cases
        .iter()
        .map(|(_, _, lowered, st)| (lowered[0], lowered[1], lowered[2], *st))
        .collect();

    let source_outs = match run_oracle3(&oracle, &source_cases) {
        Some(o) => o,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_native_lowering: source oracle run failed -> skipping"
            );
            return;
        }
    };
    let lowered_outs = match run_oracle3(&oracle, &lowered_cases) {
        Some(o) => o,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_native_lowering: generated oracle run failed -> skipping"
            );
            return;
        }
    };
    assert_eq!(source_outs.len(), cases.len());
    assert_eq!(lowered_outs.len(), cases.len());

    let mut mismatches = Vec::new();
    for (i, (label, source, lowered, _st)) in cases.iter().enumerate() {
        let source_out = &source_outs[i];
        let lowered_out = &lowered_outs[i];
        if source_out.trapped != 0 {
            mismatches.push(Mismatch {
                label: label.clone(),
                insn: source[0],
                detail: format!(
                    "source instruction faulted with signal {}",
                    source_out.trapped
                ),
            });
            continue;
        }
        if lowered_out.trapped != 0 {
            mismatches.push(Mismatch {
                label: label.clone(),
                insn: lowered[0],
                detail: format!("generated code faulted with signal {}", lowered_out.trapped),
            });
            continue;
        }
        compare_native_aarch64_case(
            label,
            lowered[0],
            &lowered_out.st,
            &source_out.st,
            &mut mismatches,
        );
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n==== smir_aarch64_native_lowering: {} mismatches across {} cases ====",
            mismatches.len(),
            cases.len()
        );
        for m in mismatches.iter().take(25) {
            eprintln!("  [{}] {:#010x}: {}", m.label, m.insn, m.detail);
        }
        panic!(
            "smir_aarch64_native_lowering: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
#[test]
fn smir_aarch64_x86_memory_lowering_matches_qemu_oracle() {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!(
                "[arm_diff] smir_aarch64_x86_memory: qemu/cross-toolchain unavailable -> skipping"
            );
            return;
        }
    };

    let ops: &[(u32, u32, u32, &str)] = &[
        (3, 0, 0, "str_x"),
        (3, 0, 1, "ldr_x"),
        (2, 0, 0, "str_w"),
        (2, 0, 1, "ldr_w"),
        (0, 0, 0, "str_b"),
        (0, 0, 1, "ldr_b"),
        (1, 0, 0, "str_h"),
        (1, 0, 1, "ldr_h"),
        (0, 0, 2, "ldrsb_x"),
        (0, 0, 3, "ldrsb_w"),
        (1, 0, 2, "ldrsh_x"),
        (1, 0, 3, "ldrsh_w"),
        (2, 0, 2, "ldrsw_x"),
    ];

    let mut rng = Rng::new(0x5a11_64c0_0c0d_e123);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(size, v, opc, name) in ops {
        for imm12 in [0u32, 1, 3] {
            for _ in 0..4 {
                batch.push((
                    format!("{name} #{imm12}"),
                    enc_ldst_uimm(size, v, opc, imm12),
                    mem_input(&mut rng),
                ));
            }
        }
    }
    for &(label, insn, rm_value) in &[
        (
            "ldr_x_reg_uxtw_lsl3",
            enc_ldst_reg(3, 1, RM, 0b010, 1),
            8u64,
        ),
        ("str_w_reg_sxtw", enc_ldst_reg(2, 0, RM, 0b110, 0), 8u64),
        (
            "ldrsb_x_reg_sxtx_neg8",
            enc_ldst_reg(0, 2, RM, 0b111, 0),
            (-8i64) as u64,
        ),
        (
            "ldrsh_w_reg_uxtw_lsl1",
            enc_ldst_reg(1, 3, RM, 0b010, 1),
            4u64,
        ),
    ] {
        for _ in 0..4 {
            let mut st = mem_input(&mut rng);
            st.x[0] = 0x1122_3344_5566_7788;
            st.x[2] = rm_value;
            st.scratch[7] = 0x0000_0000_0000_0080;
            st.scratch[8] = 0x1111_2222_3333_4444;
            st.scratch[9] = 0x5555_6666_7777_8888;
            st.scratch[16] = 0x9999_aaaa_bbbb_cccc;
            batch.push((label.to_string(), insn, st));
        }
    }
    for &(label, insn) in &[
        ("ldtr_x_pos8", enc_ldst_simm(3, 0, 1, 0b10, 8)),
        ("sttr_w_neg8", enc_ldst_simm(2, 0, 0, 0b10, -8)),
        ("ldtrsb_w_neg8", enc_ldst_simm(0, 0, 3, 0b10, -8)),
        ("ldtrsh_x_pos8", enc_ldst_simm(1, 0, 2, 0b10, 8)),
    ] {
        for _ in 0..4 {
            let mut st = mem_input(&mut rng);
            st.x[0] = 0x1122_3344_5566_7788;
            st.scratch[7] = 0x0000_0000_0000_0080;
            st.scratch[8] = 0x1111_2222_3333_4444;
            st.scratch[9] = 0x5555_6666_7777_8888;
            batch.push((label.to_string(), insn, st));
        }
    }
    for &(label, insn) in &[
        ("prfm_uimm_0", enc_ldst_uimm(3, 0, 2, 0)),
        ("prfm_uimm_3", enc_ldst_uimm(3, 0, 2, 3)),
        ("prfum_simm_neg8", enc_ldst_simm(3, 0, 2, 0b00, -8)),
        ("prfm_reg_uxtw_lsl3", enc_prfm_reg(4, RM, 0b010, 1)),
        ("prfm_reg_sxtx", enc_prfm_reg(0b10010, RM, 0b111, 0)),
    ] {
        for _ in 0..4 {
            let mut st = mem_input(&mut rng);
            st.x[2] = 8;
            batch.push((label.to_string(), insn, st));
        }
    }

    let acquire_release_ops: &[(u32, &str, bool)] = &[
        (0, "ldarb", true),
        (1, "ldarh", true),
        (2, "ldar_w", true),
        (3, "ldar_x", true),
        (0, "stlrb", false),
        (1, "stlrh", false),
        (2, "stlr_w", false),
        (3, "stlr_x", false),
    ];
    for &(size, name, is_load) in acquire_release_ops {
        for _ in 0..6 {
            let insn = if is_load {
                enc_ldar(size)
            } else {
                enc_stlr(size)
            };
            batch.push((name.to_string(), insn, mem_input(&mut rng)));
        }
    }

    let exclusive_load_ops: &[(u32, u32, &str)] = &[
        (0, 0, "ldxrb"),
        (1, 0, "ldxrh"),
        (2, 0, "ldxr_w"),
        (3, 0, "ldxr_x"),
        (0, 1, "ldaxrb"),
        (1, 1, "ldaxrh"),
        (2, 1, "ldaxr_w"),
        (3, 1, "ldaxr_x"),
    ];
    for &(size, acquire, name) in exclusive_load_ops {
        for _ in 0..6 {
            batch.push((
                name.to_string(),
                enc_ldxr_smir(size, acquire),
                mem_input(&mut rng),
            ));
        }
    }

    for size in 0..4 {
        for &(release, name) in &[(0, "stxr_unarmed"), (1, "stlxr_unarmed")] {
            for _ in 0..6 {
                let mut st = mem_input(&mut rng);
                st.x[3] = rng.interesting();
                batch.push((format!("{name} sz{size}"), enc_stxr(size, release), st));
            }
        }
    }

    let atomic_swap_ops: &[(u32, u32, &str)] = &[
        (0, 0, "swp"),
        (1, 0, "swpa"),
        (0, 1, "swpl"),
        (1, 1, "swpal"),
    ];
    for size in 0..4 {
        for &(acquire, release, name) in atomic_swap_ops {
            for _ in 0..4 {
                batch.push((
                    format!("{name} sz{size}"),
                    enc_atomic_smir(size, acquire, release, 1, 0, 2, RN, RD),
                    mem_input(&mut rng),
                ));
            }
        }
    }

    let mut st = mem_input(&mut rng);
    st.x[0] = 0xfeed_face_cafe_beef;
    st.scratch[8] = 0x1122_3344_5566_7788;
    batch.push((
        "swp_x_same_src_dst".into(),
        enc_atomic_smir(3, 0, 0, 1, 0, RD, RN, RD),
        st,
    ));

    let atomic_rmw_ops: &[(u32, &str)] = &[
        (0b000, "ldadd"),
        (0b001, "ldclr"),
        (0b010, "ldeor"),
        (0b011, "ldset"),
        (0b100, "ldsmax"),
        (0b101, "ldsmin"),
        (0b110, "ldumax"),
        (0b111, "ldumin"),
    ];
    for size in 0..4 {
        for &(opc, name) in atomic_rmw_ops {
            for &(acquire, release, suffix) in &[(0, 0, ""), (1, 1, "al")] {
                for _ in 0..3 {
                    batch.push((
                        format!("{name}{suffix} sz{size}"),
                        enc_atomic_smir(size, acquire, release, 0, opc, 2, RN, RD),
                        mem_input(&mut rng),
                    ));
                }
            }
        }
    }

    for size in 0..4 {
        for &(acquire, release, suffix) in &[(0, 0, ""), (1, 0, "a"), (0, 1, "l"), (1, 1, "al")] {
            for case_idx in 0..6 {
                let mut st = mem_input(&mut rng);
                if case_idx % 2 == 0 {
                    let mask = match size {
                        0 => 0xff,
                        1 => 0xffff,
                        2 => 0xffff_ffff,
                        _ => u64::MAX,
                    };
                    st.x[2] = st.scratch[8] & mask;
                }
                batch.push((
                    format!("cas{suffix} sz{size}"),
                    enc_cas(size, acquire, release),
                    st,
                ));
            }
        }
    }

    let indexed_ops: &[(u32, u32, u32, &str)] = &[
        (3, 0, 0, "str_x"),
        (3, 0, 1, "ldr_x"),
        (2, 0, 0, "str_w"),
        (2, 0, 1, "ldr_w"),
        (0, 0, 0, "str_b"),
        (0, 0, 1, "ldr_b"),
        (1, 0, 0, "str_h"),
        (1, 0, 1, "ldr_h"),
        (0, 0, 3, "ldrsb_w"),
        (1, 0, 3, "ldrsh_w"),
        (2, 0, 2, "ldrsw_x"),
    ];
    for &(size, v, opc, name) in indexed_ops {
        for &(mode, mode_name) in &[(0b01u32, "post"), (0b11u32, "pre")] {
            for imm9 in [8, -8] {
                for _ in 0..4 {
                    batch.push((
                        format!("{name} {mode_name} #{imm9}"),
                        enc_ldst_simm(size, v, opc, mode, imm9),
                        mem_input(&mut rng),
                    ));
                }
            }
        }
    }

    let pair_ops: &[(u32, u32, u32, &str)] = &[
        (0b10, 0, 0, "stp_x"),
        (0b10, 0, 1, "ldp_x"),
        (0b00, 0, 0, "stp_w"),
        (0b00, 0, 1, "ldp_w"),
    ];
    for &(opc, v, l, name) in pair_ops {
        let imm7s: &[i32] = if opc == 0b00 { &[2, -2] } else { &[1, -1] };
        for &(mode, mode_name) in &[(0b01u32, "post"), (0b11u32, "pre")] {
            for &imm7 in imm7s {
                for _ in 0..4 {
                    batch.push((
                        format!("{name} {mode_name} #{imm7}"),
                        enc_ldp(opc, v, mode, l, (imm7 as u32) & 0x7F),
                        mem_input(&mut rng),
                    ));
                }
            }
        }
    }

    let non_temporal_pair_ops: &[(u32, u32, &str)] = &[
        (0b10, 0, "stnp_x"),
        (0b10, 1, "ldnp_x"),
        (0b00, 0, "stnp_w"),
        (0b00, 1, "ldnp_w"),
    ];
    for &(opc, l, name) in non_temporal_pair_ops {
        let imm7s: &[i32] = if opc == 0b00 {
            &[0, 2, -2]
        } else {
            &[0, 1, -1]
        };
        for &imm7 in imm7s {
            for _ in 0..4 {
                batch.push((
                    format!("{name} #{imm7}"),
                    enc_ldp(opc, 0, 0b00, l, (imm7 as u32) & 0x7F),
                    mem_input(&mut rng),
                ));
            }
        }
    }

    let mut st = mem_input(&mut rng);
    st.scratch[8] = 0x1122_3344_5566_7788;
    st.scratch[9] = 0x99aa_bbcc_ddee_ff00;
    batch.push((
        "ldp_x_rt_base_overlap".into(),
        enc_ldp_regs(0b10, 0, 0b10, 1, 0, RN, RM, RN),
        st,
    ));

    let mut st = mem_input(&mut rng);
    st.scratch[8] = 0x1122_3344_8000_0001;
    batch.push((
        "ldp_w_rt_base_overlap".into(),
        enc_ldp_regs(0b00, 0, 0b10, 1, 0, RN, RM, RN),
        st,
    ));

    let mut st = mem_input(&mut rng);
    st.x[2] = 0x1122_3344_5566_7788;
    batch.push((
        "stp_x_src_base_overlap".into(),
        enc_ldp_regs(0b10, 0, 0b10, 0, 0, RN, RM, RN),
        st,
    ));

    let mut st = mem_input(&mut rng);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.scratch[8] = 0x0000_0000_0000_0080;
    batch.push(("ldrsb_w_negative".into(), enc_ldst_uimm(0, 0, 3, 0), st));

    let mut st = mem_input(&mut rng);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.scratch[8] = 0x0000_0000_0000_8000;
    batch.push(("ldrsh_w_negative".into(), enc_ldst_uimm(1, 0, 3, 0), st));

    let mut st = mem_input(&mut rng);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.scratch[9] = 0x0000_0000_0000_0080;
    batch.push((
        "ldrsb_w_pre_negative".into(),
        enc_ldst_simm(0, 0, 3, 0b11, 8),
        st,
    ));

    let mut st = mem_input(&mut rng);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.scratch[7] = 0x0000_0000_0000_8000;
    batch.push((
        "ldrsh_w_pre_negative_offset".into(),
        enc_ldst_simm(1, 0, 3, 0b11, -8),
        st,
    ));

    let mut st = mem_input(&mut rng);
    st.x[0] = 0xaaaa_bbbb_cccc_dddd;
    st.scratch[9] = 0x0000_0000_8000_0000;
    batch.push((
        "ldrsw_x_pre_negative".into(),
        enc_ldst_simm(2, 0, 2, 0b11, 8),
        st,
    ));

    let mut pair_batch: Vec<(String, u32, u32, ArmState)> = Vec::new();
    for size in 0..4 {
        for &(ordered, name) in &[(0, "ldxr_stxr"), (1, "ldaxr_stlxr")] {
            for _ in 0..6 {
                let mut st = mem_input(&mut rng);
                st.x[3] = rng.interesting();
                pair_batch.push((
                    format!("{name} sz{size}"),
                    enc_ldxr_smir(size, ordered),
                    enc_stxr(size, ordered),
                    st,
                ));
            }
        }
    }

    let mut triple_batch: Vec<(String, u32, u32, u32, ArmState)> = Vec::new();
    for size in 0..4 {
        for &(ordered, name) in &[(0, "ldxr_clrex_stxr"), (1, "ldaxr_clrex_stlxr")] {
            for _ in 0..6 {
                let mut st = mem_input(&mut rng);
                st.x[3] = rng.interesting();
                triple_batch.push((
                    format!("{name} sz{size}"),
                    enc_ldxr_smir(size, ordered),
                    enc_clrex(),
                    enc_stxr(size, ordered),
                    st,
                ));
            }
        }
    }

    let labels: Vec<String> = batch.iter().map(|(label, _, _)| label.clone()).collect();
    let cases: Vec<(u32, u32, ArmState)> = batch
        .iter()
        .map(|(_, insn, st)| (*insn, NOP, *st))
        .collect();
    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!("[arm_diff] smir_aarch64_x86_memory: oracle run failed -> skipping");
            return;
        }
    };
    assert_eq!(outs.len(), cases.len());

    let mut mismatches = Vec::new();
    for (idx, ((insn, _insn2, st), out)) in cases.iter().zip(outs.iter()).enumerate() {
        if out.trapped != 0 {
            mismatches.push(Mismatch {
                label: labels[idx].clone(),
                insn: *insn,
                detail: format!("hardware faulted with signal {}", out.trapped),
            });
            continue;
        }
        match run_smir_aarch64_x86_mem_one(*insn, st) {
            Ok((got, got_scratch)) => {
                compare_smir_memory_case(
                    &labels[idx],
                    *insn,
                    &got,
                    &got_scratch,
                    &out.st,
                    &mut mismatches,
                );
            }
            Err(e) => mismatches.push(Mismatch {
                label: labels[idx].clone(),
                insn: *insn,
                detail: e,
            }),
        }
    }

    let pair_labels: Vec<String> = pair_batch
        .iter()
        .map(|(label, _, _, _)| label.clone())
        .collect();
    let pair_cases: Vec<(u32, u32, ArmState)> = pair_batch
        .iter()
        .map(|(_, insn, insn2, st)| (*insn, *insn2, *st))
        .collect();
    let pair_outs = match run_oracle(&oracle, &pair_cases) {
        Some(o) => o,
        None => {
            eprintln!("[arm_diff] smir_aarch64_x86_memory: pair oracle run failed -> skipping");
            return;
        }
    };
    assert_eq!(pair_outs.len(), pair_cases.len());
    for (idx, ((insn, insn2, st), out)) in pair_cases.iter().zip(pair_outs.iter()).enumerate() {
        if out.trapped != 0 {
            mismatches.push(Mismatch {
                label: pair_labels[idx].clone(),
                insn: *insn,
                detail: format!("hardware faulted with signal {}", out.trapped),
            });
            continue;
        }
        match run_smir_aarch64_x86_mem_pair(*insn, *insn2, st) {
            Ok((got, got_scratch)) => {
                compare_smir_memory_case(
                    &pair_labels[idx],
                    *insn,
                    &got,
                    &got_scratch,
                    &out.st,
                    &mut mismatches,
                );
            }
            Err(e) => mismatches.push(Mismatch {
                label: pair_labels[idx].clone(),
                insn: *insn,
                detail: format!("{e}; second insn {insn2:#010x}"),
            }),
        }
    }

    let triple_labels: Vec<String> = triple_batch
        .iter()
        .map(|(label, _, _, _, _)| label.clone())
        .collect();
    let triple_cases: Vec<(u32, u32, u32, ArmState)> = triple_batch
        .iter()
        .map(|(_, insn, insn2, insn3, st)| (*insn, *insn2, *insn3, *st))
        .collect();
    let triple_outs = match run_oracle3(&oracle, &triple_cases) {
        Some(o) => o,
        None => {
            eprintln!("[arm_diff] smir_aarch64_x86_memory: triple oracle run failed -> skipping");
            return;
        }
    };
    assert_eq!(triple_outs.len(), triple_cases.len());
    for (idx, ((insn, insn2, insn3, st), out)) in
        triple_cases.iter().zip(triple_outs.iter()).enumerate()
    {
        if out.trapped != 0 {
            mismatches.push(Mismatch {
                label: triple_labels[idx].clone(),
                insn: *insn,
                detail: format!("hardware faulted with signal {}", out.trapped),
            });
            continue;
        }
        match run_smir_aarch64_x86_mem_three(*insn, *insn2, *insn3, st) {
            Ok((got, got_scratch)) => {
                compare_smir_memory_case(
                    &triple_labels[idx],
                    *insn,
                    &got,
                    &got_scratch,
                    &out.st,
                    &mut mismatches,
                );
            }
            Err(e) => mismatches.push(Mismatch {
                label: triple_labels[idx].clone(),
                insn: *insn,
                detail: format!("{e}; second insn {insn2:#010x}; third insn {insn3:#010x}"),
            }),
        }
    }

    if !mismatches.is_empty() {
        let total_cases = cases.len() + pair_cases.len() + triple_cases.len();
        eprintln!(
            "\n==== smir_aarch64_x86_memory: {} mismatches across {} cases ====",
            mismatches.len(),
            total_cases
        );
        for m in mismatches.iter().take(25) {
            eprintln!("  [{}] {:#010x}: {}", m.label, m.insn, m.detail);
        }
        panic!(
            "smir_aarch64_x86_memory: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

/// Run rax over a two-instruction sequence and return the final state.
fn run_rax_pair(insn: u32, insn2: u32, input: &ArmState) -> Option<ArmState> {
    let mem = FlatMemory::new(0, 0x30_0000);
    let mut cpu = AArch64Cpu::new(AArch64Config::default(), Box::new(mem));
    for i in 0..31u8 {
        cpu.set_gpr(i, input.x[i as usize]);
    }
    cpu.set_current_sp(input.sp);
    let ps = input.pstate;
    cpu.set_nzcv(
        ps & (1 << 31) != 0,
        ps & (1 << 30) != 0,
        ps & (1 << 29) != 0,
        ps & (1 << 28) != 0,
    );
    for r in 0..32u8 {
        let (lo, hi) = input.vreg(r as usize);
        cpu.set_simd_reg(r, lo, hi).ok()?;
    }
    for r in 0..16usize {
        cpu.set_sve_pred(r, input.preg(r) as u32);
    }
    let scratch_bytes: Vec<u8> = input.scratch.iter().flat_map(|w| w.to_le_bytes()).collect();
    cpu.write_memory(SCRATCH_ADDR, &scratch_bytes).ok()?;
    cpu.write_memory(0, &insn.to_le_bytes()).ok()?;
    cpu.write_memory(4, &insn2.to_le_bytes()).ok()?;
    cpu.set_pc(0);
    match cpu.step() {
        Ok(CpuExit::Continue) => {}
        _ => return None,
    }
    match cpu.step() {
        Ok(CpuExit::Continue) => {}
        _ => return None,
    }
    let mut out = ArmState::zeroed();
    for i in 0..31u8 {
        out.x[i as usize] = cpu.get_gpr(i);
    }
    out.sp = cpu.current_sp();
    out.pc = cpu.get_pc();
    let mut pstate = 0u64;
    if cpu.get_n() {
        pstate |= 1 << 31;
    }
    if cpu.get_z() {
        pstate |= 1 << 30;
    }
    if cpu.get_c() {
        pstate |= 1 << 29;
    }
    if cpu.get_v() {
        pstate |= 1 << 28;
    }
    out.pstate = pstate;
    for r in 0..32u8 {
        if let Some((lo, hi)) = cpu.get_simd_reg(r) {
            out.set_vreg(r as usize, lo, hi);
        }
    }
    for (i, w) in out.scratch.iter_mut().enumerate() {
        *w = cpu.mem_read_u64(SCRATCH_ADDR + (i as u64) * 8).ok()?;
    }
    for r in 0..16usize {
        out.set_preg(r, cpu.sve_pred(r) as u16);
    }
    Some(out)
}

/// Run a batch of two-instruction sequences differentially.
fn run_batch_pair(name: &str, batch: Vec<(String, u32, u32, ArmState)>) {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!("[arm_diff] {name}: qemu/cross-toolchain unavailable -> skipping");
            return;
        }
    };
    let cases: Vec<(u32, u32, ArmState)> = batch.iter().map(|(_, a, b, s)| (*a, *b, *s)).collect();
    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!("[arm_diff] {name}: oracle run failed -> skipping");
            return;
        }
    };
    let mut mismatches = Vec::new();
    for (i, ((insn, insn2, st), out)) in cases.iter().zip(outs.iter()).enumerate() {
        let rax = run_rax_pair(*insn, *insn2, st);
        if out.trapped != 0 {
            if rax.is_some() {
                mismatches.push(Mismatch {
                    label: batch[i].0.clone(),
                    insn: *insn,
                    detail: format!("hw faulted (sig {}) but rax executed", out.trapped),
                });
            }
            continue;
        }
        let rax = match rax {
            Some(r) => r,
            None => {
                mismatches.push(Mismatch {
                    label: batch[i].0.clone(),
                    insn: *insn,
                    detail: "hw executed but rax rejected".into(),
                });
                continue;
            }
        };
        let mut diffs = Vec::new();
        for r in 0..31 {
            if rax.x[r] != out.st.x[r] {
                diffs.push(format!("x{r}: rax={:#x} hw={:#x}", rax.x[r], out.st.x[r]));
            }
        }
        if (rax.pstate >> 28) & 0xF != (out.st.pstate >> 28) & 0xF {
            diffs.push(format!(
                "nzcv: rax={:#x} hw={:#x}",
                (rax.pstate >> 28) & 0xF,
                (out.st.pstate >> 28) & 0xF
            ));
        }
        for r in 0..32 {
            if rax.vreg(r) != out.st.vreg(r) {
                diffs.push(format!("v{r} differs"));
            }
        }
        for r in 0..16 {
            if rax.preg(r) != out.st.preg(r) {
                diffs.push(format!(
                    "p{r}: rax={:#06x} hw={:#06x}",
                    rax.preg(r),
                    out.st.preg(r)
                ));
            }
        }
        for k in 0..32 {
            if rax.scratch[k] != out.st.scratch[k] {
                diffs.push(format!(
                    "scratch[{k}]: rax={:#x} hw={:#x}",
                    rax.scratch[k], out.st.scratch[k]
                ));
            }
        }
        if !diffs.is_empty() {
            mismatches.push(Mismatch {
                label: batch[i].0.clone(),
                insn: *insn,
                detail: diffs.join("  |  "),
            });
        }
    }
    if !mismatches.is_empty() {
        use std::collections::BTreeMap;
        let mut by_label: BTreeMap<String, usize> = BTreeMap::new();
        for m in &mismatches {
            *by_label.entry(m.label.clone()).or_default() += 1;
        }
        eprintln!(
            "\n==== {name}: {} mismatches across {} cases ====",
            mismatches.len(),
            cases.len()
        );
        for (label, count) in &by_label {
            eprintln!("  {count:5}x  {label}");
        }
        for m in mismatches.iter().take(25) {
            eprintln!("  [{}] {:#010x}: {}", m.label, m.insn, m.detail);
        }
        panic!(
            "{name}: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

#[test]
fn diff_dp_addsub_shifted() {
    run_family("dp_addsub_shifted", addsub_shift_cases(), 12, 0x1001);
}

#[test]
fn diff_dp_logical_shifted() {
    run_family("dp_logical_shifted", logical_shift_cases(), 12, 0x1002);
}

#[test]
fn diff_dp2_source() {
    run_family("dp2_source", dp2_cases(), 40, 0x1003);
}

#[test]
fn diff_dp3_source() {
    run_family("dp3_source", dp3_cases(), 40, 0x1004);
}

#[test]
fn diff_simd_three_same() {
    run_family("simd_three_same", three_same_cases(), 8, 0x2001);
}

// A few finite FP16 bit patterns; products stay exactly representable in FP32,
// so the result can be compared bit-exactly (no NaN/inf payload ambiguity).
const F16_VALUES: [u16; 10] = [
    0x0000, // 0.0
    0x3C00, // 1.0
    0x4000, // 2.0
    0x4200, // 3.0
    0x4400, // 4.0
    0x3800, // 0.5
    0xBC00, // -1.0
    0xC000, // -2.0
    0xB800, // -0.5
    0x4500, // 5.0
];

/// FMLAL/FMLSL/FMLAL2/FMLSL2 widening FP16 multiply-accumulate. Inputs use
/// clean finite FP16 lanes and small finite FP32 accumulators.
#[test]
fn diff_fmlal() {
    // size<1:0> = {a, 0}: a=0 -> FMLAL/FMLAL2, a=1 -> FMLSL/FMLSL2.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for q in 0..2 {
        // FMLAL (u=0, opcode 11101, size=00) / FMLSL (size=10)
        cases.push((format!("fmlal q{q}"), enc_three_same(q, 0, 0b00, 0b11101)));
        cases.push((format!("fmlsl q{q}"), enc_three_same(q, 0, 0b10, 0b11101)));
        // FMLAL2 (u=1, opcode 11001, size=00) / FMLSL2 (size=10)
        cases.push((format!("fmlal2 q{q}"), enc_three_same(q, 1, 0b00, 0b11001)));
        cases.push((format!("fmlsl2 q{q}"), enc_three_same(q, 1, 0b10, 0b11001)));
    }

    let mut rng = Rng::new(0x2002);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..32 {
            let mut st = ArmState::zeroed();
            // v1 (Rn) and v2 (Rm): 8 finite FP16 lanes each (mix of the fixed
            // clean set with random finite normals/subnormals/zeros so the
            // subnormal fp16->fp32 widening path is exercised). No inf/NaN to
            // avoid 0*inf -> NaN payload ambiguity.
            for r in [1usize, 2] {
                let mut packed: u128 = 0;
                for lane in 0..8 {
                    let h = if rng.next() & 1 == 0 {
                        F16_VALUES[(rng.next() as usize) % F16_VALUES.len()] as u128
                    } else {
                        let sign = (rng.next() & 1) as u128;
                        match rng.next() % 4 {
                            0 => sign << 15,                                  // signed zero
                            1 => (sign << 15) | (rng.next() as u128 & 0x3FF), // subnormal
                            _ => {
                                let exp = (rng.next() % 30 + 1) as u128; // normal 1..30
                                (sign << 15) | (exp << 10) | (rng.next() as u128 & 0x3FF)
                            }
                        }
                    };
                    packed |= h << (16 * lane);
                }
                st.set_vreg(r, packed as u64, (packed >> 64) as u64);
            }
            // v0 (Rd) accumulator: 4 small finite FP32 lanes.
            let mut acc: u128 = 0;
            for lane in 0..4 {
                let val = ((rng.next() % 21) as i64 - 10) as f32; // -10..10
                acc |= (val.to_bits() as u128) << (32 * lane);
            }
            st.set_vreg(0, acc as u64, (acc >> 64) as u64);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("fmlal", batch);
}

/// AdvSIMD load/store single structure:
/// `0 Q 001101 post L R Rm opcode S size Rn Rt`. Rn=x1, Rt=v0.
fn enc_single_fields(
    q: u32,
    post: u32,
    l: u32,
    r: u32,
    rm: u32,
    opcode: u32,
    s: u32,
    size: u32,
) -> u32 {
    (q << 30)
        | (0b001101 << 24)
        | (post << 23)
        | (l << 22)
        | (r << 21)
        | (rm << 16)
        | (opcode << 13)
        | (s << 12)
        | (size << 10)
        | (RN << 5)
        | RD
}

/// Single-element form for element-log2-size `esz` (0=B,1=H,2=S,3=D), structure
/// size `selem` (1-4), lane `index`, load `l`, and `post` index.
fn enc_single(esz: u32, selem: u32, index: u32, l: u32, post: u32) -> u32 {
    let scale = if esz == 3 { 0b10 } else { esz };
    let lsb = ((selem - 1) >> 1) & 1;
    let r = (selem - 1) & 1;
    let opcode = (scale << 1) | lsb;
    let (q, s, size) = match esz {
        0 => ((index >> 3) & 1, (index >> 2) & 1, index & 3),
        1 => ((index >> 2) & 1, (index >> 1) & 1, (index & 1) << 1),
        2 => ((index >> 1) & 1, index & 1, 0),
        _ => (index & 1, 0, 1),
    };
    let rm = if post == 1 { 31 } else { 0 };
    enc_single_fields(q, post, l, r, rm, opcode, s, size)
}

/// Replicating load LD1R-LD4R for element-log2-size `esz`, structure `selem`, Q.
fn enc_single_rep(esz: u32, selem: u32, q: u32, post: u32) -> u32 {
    let lsb = ((selem - 1) >> 1) & 1;
    let r = (selem - 1) & 1;
    let opcode = (0b11 << 1) | lsb;
    let rm = if post == 1 { 31 } else { 0 };
    enc_single_fields(q, post, 1, r, rm, opcode, 0, esz)
}

#[test]
fn diff_mem_ldst_single() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for esz in 0..4u32 {
        let max_index = 16u32 >> esz; // B:16 H:8 S:4 D:2
        for selem in 1..=4u32 {
            for &index in &[0u32, max_index - 1] {
                for l in 0..2 {
                    let op = if l == 1 { "ld" } else { "st" };
                    cases.push((
                        format!("{op}{selem}_single e{esz} i{index}"),
                        enc_single(esz, selem, index, l, 0),
                    ));
                    cases.push((
                        format!("{op}{selem}_single e{esz} i{index} post"),
                        enc_single(esz, selem, index, l, 1),
                    ));
                }
            }
            // Replicating loads (LD1R-LD4R)
            for q in 0..2 {
                cases.push((
                    format!("ld{selem}r e{esz} q{q}"),
                    enc_single_rep(esz, selem, q, 0),
                ));
            }
        }
    }
    let mut rng = Rng::new(0x1_0005);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..6 {
            batch.push((label.clone(), *insn, mem_input(&mut rng)));
        }
    }
    run_batch("mem_ldst_single", batch);
}

/// LDXR/LDAXR <Rt>, [Rn]: `size 001000 0 1 0 11111 o0 11111 Rn Rt`. Rt=x0, Rn=x1.
fn enc_ldxr(size: u32, o0: u32) -> u32 {
    (size << 30)
        | (0b001000 << 24)
        | (1 << 22)
        | (0b11111 << 16)
        | (o0 << 15)
        | (0b11111 << 10)
        | (RN << 5)
        | RD
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_ldxr_smir(size: u32, acquire: u32) -> u32 {
    (size << 30)
        | (0b001000 << 24)
        | (1 << 22)
        | (0b11111 << 16)
        | ((acquire & 1) << 15)
        | (0b11111 << 10)
        | (RN << 5)
        | RD
}

#[cfg(all(feature = "smir-jit", target_arch = "x86_64"))]
fn enc_atomic_smir(
    size: u32,
    acquire: u32,
    release: u32,
    o3: u32,
    opc: u32,
    rs: u32,
    rn: u32,
    rt: u32,
) -> u32 {
    (size << 30)
        | (0b111 << 27)
        | ((acquire & 1) << 23)
        | ((release & 1) << 22)
        | (1 << 21)
        | (rs << 16)
        | ((o3 & 1) << 15)
        | (opc << 12)
        | (rn << 5)
        | rt
}

/// STXR/STLXR <Ws>, <Rt>, [Rn]: `size 001000 0 0 0 Rs o0 11111 Rn Rt`. Ws=x2, Rt=x3, Rn=x1.
fn enc_stxr(size: u32, o0: u32) -> u32 {
    (size << 30) | (0b001000 << 24) | (2 << 16) | (o0 << 15) | (0b11111 << 10) | (RN << 5) | 3
}

/// LDAR/LDARB/LDARH <Rt>, [Rn]: `size 001000 1 1 0 11111 1 11111 Rn Rt`.
fn enc_ldar(size: u32) -> u32 {
    (size << 30)
        | (0b001000 << 24)
        | (1 << 23)
        | (1 << 22)
        | (0b11111 << 16)
        | (1 << 15)
        | (0b11111 << 10)
        | (RN << 5)
        | RD
}

/// STLR/STLRB/STLRH <Rt>, [Rn]: `size 001000 1 0 0 11111 1 11111 Rn Rt`. Rt=x3.
fn enc_stlr(size: u32) -> u32 {
    (size << 30)
        | (0b001000 << 24)
        | (1 << 23)
        | (0b11111 << 16)
        | (1 << 15)
        | (0b11111 << 10)
        | (RN << 5)
        | 3
}

/// AES single-block op: `0100111000 10100 opcode 10 Rn Rd`. Rn=v1, Rd=v0.
fn enc_aes(opcode: u32) -> u32 {
    0x4e28_0800 | (opcode << 12) | (RN << 5) | RD
}

/// Advanced SIMD three-same (FP16): `0 Q U 01110 a 10 Rm 00 opcode 1 Rn Rd`.
/// Rd=v0, Rn=v1, Rm=v2.
fn enc_fp16_3s(q: u32, u: u32, a: u32, opcode: u32) -> u32 {
    (q << 30)
        | (u << 29)
        | (0b01110 << 24)
        | (a << 23)
        | (1 << 22)
        | (RM << 16)
        | (opcode << 11)
        | (1 << 10)
        | (RN << 5)
        | RD
}

/// Generate a finite/inf/zero/subnormal binary16 value (never a NaN, to keep
/// NaN-payload propagation out of the differential comparison).
fn rand_fp16(rng: &mut Rng) -> u16 {
    let sign = (rng.next() & 1) as u16;
    match rng.next() % 16 {
        0 => sign << 15,                                 // zero
        1 => (sign << 15) | 0x7C00,                      // infinity
        2 => (sign << 15) | (rng.next() as u16 & 0x3FF), // subnormal
        _ => {
            let exp = (rng.next() % 28 + 1) as u16; // normal exponent 1..28
            let mant = rng.next() as u16 & 0x3FF;
            (sign << 15) | (exp << 10) | mant
        }
    }
}

fn fp16_vec(rng: &mut Rng) -> (u64, u64) {
    let mut lo = 0u64;
    let mut hi = 0u64;
    for e in 0..4 {
        lo |= (rand_fp16(rng) as u64) << (e * 16);
        hi |= (rand_fp16(rng) as u64) << (e * 16);
    }
    (lo, hi)
}

/// Advanced SIMD vector x indexed element (FP16):
/// `0 Q U 01111 00 L M Rm opcode H 0 Rn Rd`. Rd=v0, Rn=v1, Rm=v2. The broadcast
/// lane index is H:L:M.
fn enc_fp16_idx(q: u32, u: u32, opcode: u32, index: u32) -> u32 {
    let h = (index >> 2) & 1;
    let l = (index >> 1) & 1;
    let m = index & 1;
    (q << 30)
        | (u << 29)
        | (0b01111 << 24)
        | (l << 21)
        | (m << 20)
        | (RM << 16)
        | (opcode << 12)
        | (h << 11)
        | (RN << 5)
        | RD
}

/// Scalar three-same FP16: `01 U 11110 a 10 Rm 00 opcode 1 Rn Rd`.
fn enc_fp16_3s_scalar(u: u32, a: u32, opcode: u32) -> u32 {
    (1 << 30)
        | (u << 29)
        | (0b11110 << 24)
        | (a << 23)
        | (1 << 22)
        | (RM << 16)
        | (opcode << 11)
        | (1 << 10)
        | (RN << 5)
        | RD
}

/// Scalar two-reg-misc FP16: `01 U 11110 a 1 11100 opcode 10 Rn Rd`.
fn enc_fp16_2r_scalar(u: u32, a: u32, opcode: u32) -> u32 {
    (1 << 30)
        | (u << 29)
        | (0b11110 << 24)
        | (a << 23)
        | (1 << 22)
        | (0b11100 << 17)
        | (opcode << 12)
        | (0b10 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_fp16_scalar() {
    let mut rng = Rng::new(0x1_0015);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    // Scalar three-same FP16 (only lane 0 is used; upper bits must zero).
    // Genuine scalar three-same forms plus several with no scalar encoding
    // (fadd/fmul/fmax/fmaxnm/fmla/faddp) — both rax and the oracle must reject
    // those.
    let three: &[(u32, u32, u32, &str)] = &[
        (0, 0, 0b011, "fmulx"),
        (0, 0, 0b111, "frecps"),
        (0, 1, 0b111, "frsqrts"),
        (1, 1, 0b010, "fabd"),
        (0, 0, 0b100, "fcmeq"),
        (1, 0, 0b100, "fcmge"),
        (1, 1, 0b100, "fcmgt"),
        (1, 0, 0b101, "facge"),
        (1, 1, 0b101, "facgt"),
        (0, 0, 0b010, "fadd"),
        (1, 0, 0b011, "fmul"),
        (0, 0, 0b110, "fmax"),
        (0, 0, 0b000, "fmaxnm"),
        (0, 0, 0b001, "fmla"),
        (1, 0, 0b010, "faddp"),
    ];
    for &(u, a, opcode, name) in three {
        let insn = enc_fp16_3s_scalar(u, a, opcode);
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            let (a1, b1) = fp16_vec(&mut rng);
            let (a2, b2) = fp16_vec(&mut rng);
            st.set_vreg(1, a1, b1);
            st.set_vreg(2, a2, b2);
            batch.push((format!("3s {name}"), insn, st));
        }
    }
    // Scalar two-reg-misc FP16.
    // Includes ops with no SIMD-scalar form (fsqrt/fabs/fneg/frint*) — both rax
    // and the oracle must reject those — alongside the genuine scalar forms.
    let two: &[(u32, u32, u32, &str)] = &[
        (1, 1, 0b11111, "fsqrt"),
        (0, 1, 0b11111, "frecpx"),
        (0, 1, 0b11101, "frecpe"),
        (1, 1, 0b11101, "frsqrte"),
        (0, 1, 0b01100, "fcmgt0"),
        (1, 1, 0b01101, "fcmle0"),
        (0, 1, 0b11011, "fcvtzs"),
        (1, 1, 0b11011, "fcvtzu"),
        (0, 0, 0b11101, "scvtf"),
        (0, 1, 0b01111, "fabs"),
        (1, 1, 0b01111, "fneg"),
        (1, 1, 0b11001, "frinti"),
        (0, 0, 0b11000, "frintn"),
    ];
    for &(u, a, opcode, name) in two {
        let insn = enc_fp16_2r_scalar(u, a, opcode);
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            let (a1, b1) = fp16_vec(&mut rng);
            st.set_vreg(1, a1, b1);
            batch.push((format!("2r {name}"), insn, st));
        }
    }
    run_batch("simd_fp16_scalar", batch);
}

/// A binary16 NaN (signaling or quiet), random sign/payload (payload != 0).
fn rand_fp16_nan(rng: &mut Rng) -> u16 {
    let sign = (rng.next() & 1) as u16;
    let quiet = (rng.next() & 1) as u16; // bit9 (0x0200) set => quiet
    let payload = ((rng.next() as u16) & 0x1FF).max(1); // bits[8:0], nonzero
    (sign << 15) | 0x7C00 | (quiet << 9) | payload
}

#[test]
fn diff_simd_fp16_nan() {
    // Verify NaN propagation/quieting with exactly one NaN operand (so the
    // result is unambiguous — no two-NaN ordering dependence).
    let three: &[(u32, u32, u32, &str)] = &[
        (0, 0, 0b010, "fadd"),
        (0, 1, 0b010, "fsub"),
        (1, 0, 0b011, "fmul"),
        (0, 0, 0b000, "fmaxnm"),
        (0, 0, 0b110, "fmax"),
        (0, 0, 0b001, "fmla"),
        (0, 0, 0b100, "fcmeq"),
    ];
    let two: &[(u32, u32, u32, &str)] = &[
        (0, 1, 0b01111, "fabs"),
        (1, 1, 0b01111, "fneg"),
        (1, 1, 0b11111, "fsqrt"),
        (0, 1, 0b11101, "frecpe"),
        (1, 1, 0b11101, "frsqrte"),
        (1, 1, 0b11001, "frinti"),
        (0, 1, 0b11011, "fcvtzs"),
    ];
    let mut rng = Rng::new(0x1_0014);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    // Three-same: NaN in exactly one of v1/v2 per lane, finite elsewhere. v0
    // (the FMLA accumulator) stays finite.
    for &(u, a, opcode, name) in three {
        let insn = enc_fp16_3s(1, u, a, opcode);
        for _ in 0..32 {
            let mut st = ArmState::zeroed();
            let (mut a1, mut b1) = fp16_vec(&mut rng);
            let (mut a2, mut b2) = fp16_vec(&mut rng);
            let (a0, b0) = fp16_vec(&mut rng);
            // Force one operand of each 16-bit lane to be a NaN, the other finite.
            for lane in 0..8 {
                let nan = rand_fp16_nan(&mut rng) as u64;
                let pick = rng.next() & 1;
                let sh = (lane % 4) * 16;
                let (v1, v2) = if lane < 4 {
                    (&mut a1, &mut a2)
                } else {
                    (&mut b1, &mut b2)
                };
                if pick == 0 {
                    *v1 = (*v1 & !(0xFFFFu64 << sh)) | (nan << sh);
                } else {
                    *v2 = (*v2 & !(0xFFFFu64 << sh)) | (nan << sh);
                }
            }
            st.set_vreg(0, a0, b0);
            st.set_vreg(1, a1, b1);
            st.set_vreg(2, a2, b2);
            batch.push((format!("3s {name}"), insn, st));
        }
    }
    // Two-reg-misc: NaN inputs in v1.
    for &(u, a, opcode, name) in two {
        let insn = enc_fp16_2r(1, u, a, opcode);
        for _ in 0..32 {
            let mut st = ArmState::zeroed();
            let lo = ((rand_fp16_nan(&mut rng) as u64) << 48)
                | ((rand_fp16_nan(&mut rng) as u64) << 32)
                | ((rand_fp16_nan(&mut rng) as u64) << 16)
                | rand_fp16_nan(&mut rng) as u64;
            let hi = ((rand_fp16_nan(&mut rng) as u64) << 48)
                | ((rand_fp16_nan(&mut rng) as u64) << 32)
                | ((rand_fp16_nan(&mut rng) as u64) << 16)
                | rand_fp16_nan(&mut rng) as u64;
            st.set_vreg(1, lo, hi);
            batch.push((format!("2r {name}"), insn, st));
        }
    }
    run_batch("simd_fp16_nan", batch);
}

#[test]
fn diff_simd_fp16_indexed() {
    let ops: &[(u32, u32, &str)] = &[
        (0, 0b0001, "fmla"),
        (0, 0b0101, "fmls"),
        (0, 0b1001, "fmul"),
        (1, 0b1001, "fmulx"),
    ];
    let mut rng = Rng::new(0x1_0013);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(u, opcode, name) in ops {
        for q in 0..2u32 {
            for index in 0..8u32 {
                let insn = enc_fp16_idx(q, u, opcode, index);
                for _ in 0..6 {
                    let mut st = ArmState::zeroed();
                    let (a0, b0) = fp16_vec(&mut rng);
                    let (a1, b1) = fp16_vec(&mut rng);
                    let (a2, b2) = fp16_vec(&mut rng);
                    st.set_vreg(0, a0, b0);
                    st.set_vreg(1, a1, b1);
                    st.set_vreg(2, a2, b2);
                    batch.push((format!("{name} q{q} i{index}"), insn, st));
                }
            }
        }
    }
    run_batch("simd_fp16_indexed", batch);
}

/// Advanced SIMD two-register miscellaneous (FP16):
/// `0 Q U 01110 a 1 11100 opcode 10 Rn Rd`. Rd=v0, Rn=v1.
fn enc_fp16_2r(q: u32, u: u32, a: u32, opcode: u32) -> u32 {
    (q << 30)
        | (u << 29)
        | (0b01110 << 24)
        | (a << 23)
        | (1 << 22)
        | (0b11100 << 17)
        | (opcode << 12)
        | (0b10 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_fp16_2reg() {
    // (U, a=bit23, opcode) table for the FP16 two-register-misc group.
    let ops: &[(u32, u32, u32, &str)] = &[
        (0, 1, 0b01111, "fabs"),
        (1, 1, 0b01111, "fneg"),
        (1, 1, 0b11111, "fsqrt"),
        (0, 1, 0b11101, "frecpe"),
        (1, 1, 0b11101, "frsqrte"),
        (0, 1, 0b01100, "fcmgt0"),
        (0, 1, 0b01101, "fcmeq0"),
        (0, 1, 0b01110, "fcmlt0"),
        (1, 1, 0b01100, "fcmge0"),
        (1, 1, 0b01101, "fcmle0"),
        (0, 0, 0b11000, "frintn"),
        (0, 0, 0b11001, "frintm"),
        (0, 1, 0b11000, "frintp"),
        (0, 1, 0b11001, "frintz"),
        (1, 0, 0b11000, "frinta"),
        (1, 0, 0b11001, "frintx"),
        (1, 1, 0b11001, "frinti"),
        (0, 0, 0b11010, "fcvtns"),
        (0, 0, 0b11011, "fcvtms"),
        (0, 0, 0b11100, "fcvtas"),
        (0, 1, 0b11010, "fcvtps"),
        (0, 1, 0b11011, "fcvtzs"),
        (1, 0, 0b11010, "fcvtnu"),
        (1, 0, 0b11011, "fcvtmu"),
        (1, 0, 0b11100, "fcvtau"),
        (1, 1, 0b11010, "fcvtpu"),
        (1, 1, 0b11011, "fcvtzu"),
        (0, 0, 0b11101, "scvtf"),
        (1, 0, 0b11101, "ucvtf"),
    ];
    let mut rng = Rng::new(0x1_0012);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(u, a, opcode, name) in ops {
        for q in 0..2u32 {
            let insn = enc_fp16_2r(q, u, a, opcode);
            for _ in 0..24 {
                let mut st = ArmState::zeroed();
                let (lo, hi) = fp16_vec(&mut rng);
                st.set_vreg(1, lo, hi);
                batch.push((format!("{name} q{q}"), insn, st));
            }
        }
    }
    run_batch("simd_fp16_2reg", batch);
}

#[test]
fn diff_simd_fp16_3same() {
    // Full (U, a, opcode) table of the FP16 three-same group.
    let ops: &[(u32, u32, u32, &str)] = &[
        (0, 0, 0b000, "fmaxnm"),
        (0, 1, 0b000, "fminnm"),
        (0, 0, 0b001, "fmla"),
        (0, 1, 0b001, "fmls"),
        (0, 0, 0b010, "fadd"),
        (0, 1, 0b010, "fsub"),
        (0, 0, 0b011, "fmulx"),
        (0, 0, 0b100, "fcmeq"),
        (0, 0, 0b110, "fmax"),
        (0, 1, 0b110, "fmin"),
        (0, 0, 0b111, "frecps"),
        (0, 1, 0b111, "frsqrts"),
        (1, 0, 0b000, "fmaxnmp"),
        (1, 1, 0b000, "fminnmp"),
        (1, 0, 0b010, "faddp"),
        (1, 1, 0b010, "fabd"),
        (1, 0, 0b011, "fmul"),
        (1, 0, 0b100, "fcmge"),
        (1, 1, 0b100, "fcmgt"),
        (1, 0, 0b101, "facge"),
        (1, 1, 0b101, "facgt"),
        (1, 0, 0b110, "fmaxp"),
        (1, 1, 0b110, "fminp"),
        (1, 0, 0b111, "fdiv"),
    ];
    let mut rng = Rng::new(0x1_0011);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(u, a, opcode, name) in ops {
        for q in 0..2u32 {
            let insn = enc_fp16_3s(q, u, a, opcode);
            for _ in 0..24 {
                let mut st = ArmState::zeroed();
                // v0 (Rd) seeds the FMLA/FMLS accumulator; v1/v2 are the sources.
                let (a0, b0) = fp16_vec(&mut rng);
                let (a1, b1) = fp16_vec(&mut rng);
                let (a2, b2) = fp16_vec(&mut rng);
                st.set_vreg(0, a0, b0);
                st.set_vreg(1, a1, b1);
                st.set_vreg(2, a2, b2);
                batch.push((format!("{name} q{q}"), insn, st));
            }
        }
    }
    run_batch("simd_fp16_3same", batch);
}

#[test]
fn diff_simd_urecpe() {
    // URECPE (U=0) / URSQRTE (U=1): 32-bit unsigned integer estimates.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for q in 0..2 {
        cases.push((format!("urecpe q{q}"), enc_two_reg(q, 0, 0b10, 0b11100)));
        cases.push((format!("ursqrte q{q}"), enc_two_reg(q, 1, 0b10, 0b11100)));
    }
    run_family("simd_urecpe", cases, 40, 0x1_000C);
}

#[test]
fn diff_simd_frecpe() {
    // FRECPE: U=0, opcode 11101, sz_hi=1 -> size = 0b10 (f32) / 0b11 (f64).
    let mut cases: Vec<(String, u32, bool)> = Vec::new();
    for q in 0..2 {
        cases.push((
            format!("frecpe f32 q{q}"),
            enc_two_reg(q, 0, 0b10, 0b11101),
            false,
        ));
    }
    cases.push(("frecpe f64".into(), enc_two_reg(1, 0, 0b11, 0b11101), true));
    // Normal positive finite inputs so no special-case / over-underflow paths.
    let mut rng = Rng::new(0x1_000A);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn, f64op) in &cases {
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            let mut packed: u128 = 0;
            if *f64op {
                for lane in 0..2 {
                    let v = ((rng.next() % 200 + 1) as f64) * 0.125;
                    packed |= (v.to_bits() as u128) << (64 * lane);
                }
            } else {
                for lane in 0..4 {
                    let v = ((rng.next() % 200 + 1) as f32) * 0.125;
                    packed |= (v.to_bits() as u128) << (32 * lane);
                }
            }
            st.set_vreg(1, packed as u64, (packed >> 64) as u64);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("simd_frecpe", batch);
}

#[test]
fn diff_simd_frsqrte() {
    let mut cases: Vec<(String, u32, bool)> = Vec::new();
    for q in 0..2 {
        cases.push((
            format!("frsqrte f32 q{q}"),
            enc_two_reg(q, 1, 0b10, 0b11101),
            false,
        ));
    }
    cases.push(("frsqrte f64".into(), enc_two_reg(1, 1, 0b11, 0b11101), true));
    let mut rng = Rng::new(0x1_000B);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn, f64op) in &cases {
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            let mut packed: u128 = 0;
            if *f64op {
                for lane in 0..2 {
                    let v = ((rng.next() % 500 + 1) as f64) * 0.0625;
                    packed |= (v.to_bits() as u128) << (64 * lane);
                }
            } else {
                for lane in 0..4 {
                    let v = ((rng.next() % 500 + 1) as f32) * 0.0625;
                    packed |= (v.to_bits() as u128) << (32 * lane);
                }
            }
            st.set_vreg(1, packed as u64, (packed >> 64) as u64);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("simd_frsqrte", batch);
}

#[test]
fn diff_crypto_aes() {
    let ops: &[(u32, &str)] = &[
        (0b00100, "aese"),
        (0b00101, "aesd"),
        (0b00110, "aesmc"),
        (0b00111, "aesimc"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(opcode, name) in ops {
        cases.push((name.to_string(), enc_aes(opcode)));
    }
    run_family("crypto_aes", cases, 40, 0x1_0009);
}

/// Three-register SHA: `0101 1110 000 Rm 0 opcode 00 Rn Rd`.
fn enc_sha3(opcode: u32) -> u32 {
    0x5E00_0000 | (RM << 16) | (opcode << 12) | (RN << 5) | RD
}

/// Two-register SHA: `0101 1110 0010 1000 opcode 10 Rn Rd`.
fn enc_sha2(opcode: u32) -> u32 {
    0x5E28_0800 | (opcode << 12) | (RN << 5) | RD
}

#[test]
fn diff_crypto_sha() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    // Three-register (opcode at bits[14:12]).
    for &(opcode, name) in &[
        (0b000u32, "sha1c"),
        (0b001, "sha1p"),
        (0b010, "sha1m"),
        (0b011, "sha1su0"),
        (0b100, "sha256h"),
        (0b101, "sha256h2"),
        (0b110, "sha256su1"),
    ] {
        cases.push((name.to_string(), enc_sha3(opcode)));
    }
    // Two-register (opcode at bits[16:12]).
    for &(opcode, name) in &[
        (0b00000u32, "sha1h"),
        (0b00001, "sha1su1"),
        (0b00010, "sha256su0"),
    ] {
        cases.push((name.to_string(), enc_sha2(opcode)));
    }
    run_family("crypto_sha", cases, 40, 0x1_000D);
}

/// SM4E Vd.4S, Vn.4S: `11001110 11000000 100001 Rn Rd`. Rd=v0, Rn=v1.
fn enc_sm4e() -> u32 {
    0xCE00_0000 | (0xC0 << 16) | (0b100001 << 10) | (RN << 5) | RD
}

/// SM4EKEY Vd.4S, Vn.4S, Vm.4S: `11001110 011 Rm 110010 Rn Rd`. Rd=v0, Rn=v1, Rm=v2.
fn enc_sm4ekey() -> u32 {
    0xCE00_0000 | (0b011 << 21) | (RM << 16) | (0b110010 << 10) | (RN << 5) | RD
}

/// SDOT/UDOT: `0 Q U 01110 10 0 Rm 100101 Rn Rd`. Rd=v0, Rn=v1, Rm=v2.
fn enc_dot(q: u32, u: u32) -> u32 {
    (q << 30)
        | (u << 29)
        | (0b01110 << 24)
        | (0b10 << 22)
        | (RM << 16)
        | (0b100101 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_dot() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for q in 0..2u32 {
        cases.push((format!("sdot q{q}"), enc_dot(q, 0)));
        cases.push((format!("udot q{q}"), enc_dot(q, 1)));
    }
    run_family("simd_dot", cases, 40, 0x1_0018);
}

/// Fill a 128-bit vector with `lanes` finite FP values of width `esize` bits.
fn fill_finite_fp(rng: &mut Rng, esize: u32, lanes: usize) -> (u64, u64) {
    let mut v: u128 = 0;
    for i in 0..lanes {
        let bits: u64 = match esize {
            16 => {
                let sign = (rng.next() & 1) as u64;
                let exp = (rng.next() % 18 + 6) as u64; // finite normal exponents
                let mant = rng.next() & 0x3FF;
                (sign << 15) | (exp << 10) | mant
            }
            32 => {
                let val = (((rng.next() % 4000) as f32) - 2000.0) / 200.0; // +/-10
                val.to_bits() as u64
            }
            _ => {
                let val = (((rng.next() % 4000) as f64) - 2000.0) / 200.0;
                val.to_bits()
            }
        };
        v |= (bits as u128) << (i * esize as usize);
    }
    (v as u64, (v >> 64) as u64)
}

/// FCADD: `0 Q 1 01110 size 0 Rm 111 rot 01 Rn Rd`. Rd=v0, Rn=v1, Rm=v2.
fn enc_fcadd(q: u32, size: u32, rot: u32) -> u32 {
    (q << 30)
        | (1 << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b111 << 13)
        | (rot << 12)
        | (0b01 << 10)
        | (RN << 5)
        | RD
}

/// FCMLA (vector): `0 Q 1 01110 size 0 Rm 110 rot 1 Rn Rd` (rot is 2 bits).
fn enc_fcmla(q: u32, size: u32, rot: u32) -> u32 {
    (q << 30)
        | (1 << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b110 << 13)
        | (rot << 11)
        | (1 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_complex() {
    let mut rng = Rng::new(0x1_001A);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    // (size, esize): 01=f16, 10=f32, 11=f64.
    for &(size, esize) in &[(0b01u32, 16u32), (0b10, 32), (0b11, 64)] {
        for q in 0..2u32 {
            if esize == 64 && q == 0 {
                continue; // a 64-bit complex pair needs the full 128 bits
            }
            let datasize = if q == 1 { 128 } else { 64 };
            let lanes = datasize / esize as usize;
            // FCADD (1-bit rotation).
            for rot in 0..2u32 {
                let insn = enc_fcadd(q, size, rot);
                for _ in 0..16 {
                    let mut st = ArmState::zeroed();
                    let (l1, h1) = fill_finite_fp(&mut rng, esize, lanes);
                    let (l2, h2) = fill_finite_fp(&mut rng, esize, lanes);
                    st.set_vreg(1, l1, h1);
                    st.set_vreg(2, l2, h2);
                    batch.push((format!("fcadd e{esize} q{q} r{rot}"), insn, st));
                }
            }
            // FCMLA (2-bit rotation; reads Vd accumulator).
            for rot in 0..4u32 {
                let insn = enc_fcmla(q, size, rot);
                for _ in 0..16 {
                    let mut st = ArmState::zeroed();
                    let (l0, h0) = fill_finite_fp(&mut rng, esize, lanes);
                    let (l1, h1) = fill_finite_fp(&mut rng, esize, lanes);
                    let (l2, h2) = fill_finite_fp(&mut rng, esize, lanes);
                    st.set_vreg(0, l0, h0);
                    st.set_vreg(1, l1, h1);
                    st.set_vreg(2, l2, h2);
                    batch.push((format!("fcmla e{esize} q{q} r{rot}"), insn, st));
                }
            }
        }
    }
    run_batch("simd_complex", batch);
}

/// FCMLA by element: `0 Q 1 01111 size L M Rm 0 rot 1 H 0 Rn Rd`. Vm=M:Rm (=v2),
/// Rd=v0, Rn=v1. rot=bits[14:13], index=H:L (f16) / H (f32).
fn enc_fcmla_idx(q: u32, size: u32, rot: u32, index: u32) -> u32 {
    // For f16 index=H:L (2 bits); for f32 index=H (1 bit, L must be 0).
    let (h, l) = if size == 0b01 {
        ((index >> 1) & 1, index & 1)
    } else {
        (index & 1, 0)
    };
    (q << 30)
        | (1 << 29)
        | (0b01111 << 24)
        | (size << 22)
        | (l << 21)
        | (RM << 16)
        | (rot << 13)
        | (1 << 12)
        | (h << 11)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_complex_indexed() {
    let mut rng = Rng::new(0x1_001B);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(size, esize) in &[(0b01u32, 16u32), (0b10, 32)] {
        for q in 0..2u32 {
            // f32 needs Q=1; f16 index range depends on Q.
            let max_index = if size == 0b10 {
                if q == 0 {
                    continue;
                } else {
                    2
                }
            } else if q == 1 {
                4
            } else {
                2
            };
            let datasize = if q == 1 { 128 } else { 64 };
            let lanes = datasize / esize as usize;
            for rot in 0..4u32 {
                for index in 0..max_index {
                    let insn = enc_fcmla_idx(q, size, rot, index);
                    for _ in 0..8 {
                        let mut st = ArmState::zeroed();
                        let (l0, h0) = fill_finite_fp(&mut rng, esize, lanes);
                        let (l1, h1) = fill_finite_fp(&mut rng, esize, lanes);
                        let (l2, h2) = fill_finite_fp(&mut rng, esize, lanes);
                        st.set_vreg(0, l0, h0);
                        st.set_vreg(1, l1, h1);
                        st.set_vreg(2, l2, h2);
                        batch.push((format!("fcmla_idx e{esize} q{q} r{rot} i{index}"), insn, st));
                    }
                }
            }
        }
    }
    run_batch("simd_complex_indexed", batch);
}

/// SDOT/UDOT by element: `0 Q U 01111 10 L M Rm 1110 H 0 Rn Rd`. Rm=v2, the
/// H:L index selects a 32-bit group of Vm. Rd=v0, Rn=v1.
fn enc_dot_idx(q: u32, u: u32, index: u32) -> u32 {
    let h = (index >> 1) & 1;
    let l = index & 1;
    (q << 30)
        | (u << 29)
        | (0b01111 << 24)
        | (0b10 << 22)
        | (l << 21)
        | (RM << 16)
        | (0b1110 << 12)
        | (h << 11)
        | (RN << 5)
        | RD
}

/// SVE unpredicated integer arithmetic: `00000100 sz 1 Zm opc6 Zn Zd`.
/// Rd=z0, Rn=z1, Rm=z2. At VL=128 the Z registers alias V0..V31.
fn enc_sve_arith(sz: u32, opc6: u32) -> u32 {
    (0b00000100 << 24) | (sz << 22) | (1 << 21) | (RM << 16) | (opc6 << 10) | (RN << 5) | RD
}

/// SVE bitwise logical (unpredicated): `00000100 opc 1 Zm 001100 Zn Zd`.
fn enc_sve_logical(opc: u32) -> u32 {
    (0b00000100 << 24) | (opc << 22) | (1 << 21) | (RM << 16) | (0b001100 << 10) | (RN << 5) | RD
}

/// SVE2 unpredicated multiply: `00000100 size 1 Zm 0110 opc Zn Zd`.
/// opc (bits[11:10]): 00=MUL, 10=SMULH, 11=UMULH.
fn enc_sve_mul(sz: u32, opc: u32) -> u32 {
    (0b00000100 << 24)
        | (sz << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b0110 << 12)
        | (opc << 10)
        | (RN << 5)
        | RD
}

/// SVE2 bitwise ternary: `00000100 opc 1 Zm 00111 o2 Zk Zdn`. Zdn=z0, Zk=z1(RN),
/// Zm=z2(RM).
fn enc_sve2_tern(opc: u32, o2: u32) -> u32 {
    (0b00000100 << 24)
        | (opc << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b00111 << 11)
        | (o2 << 10)
        | (RN << 5)
        | RD
}

/// SVE2 integer add/subtract long: `01000101 size 0 Zm 000 S U T Zn Zd`.
/// Zn=z1(RN), Zm=z2(RM), Zd=z0(RD).
fn enc_sve2_addl(size: u32, s: u32, u: u32, t: u32) -> u32 {
    (0b01000101 << 24)
        | (size << 22)
        | (RM << 16)
        | (s << 12)
        | (u << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// SVE2 abs-diff long (S=1): `01000101 size 0 Zm 001 1 U T Zn Zd`.
fn enc_sve2_abdl(size: u32, u: u32, t: u32) -> u32 {
    (0b01000101 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b001 << 13)
        | (1 << 12)
        | (u << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// SVE2 add/subtract wide: `01000101 size 0 Zm 010 S U T Zn Zd`.
fn enc_sve2_addw(size: u32, s: u32, u: u32, t: u32) -> u32 {
    (0b01000101 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b010 << 13)
        | (s << 12)
        | (u << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// SVE2 multiply long: `01000101 size 0 Zm 011 op U T Zn Zd`.
fn enc_sve2_mull(size: u32, op: u32, u: u32, t: u32) -> u32 {
    (0b01000101 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b011 << 13)
        | (op << 12)
        | (u << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// SVE2 multiply-add long: `01000100 size 0 Zm 010 S U T Zn Zda`.
fn enc_sve2_mlal(size: u32, s: u32, u: u32, t: u32) -> u32 {
    (0b01000100 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b010 << 13)
        | (s << 12)
        | (u << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// SVE2 saturating doubling multiply-add long: `01000100 size 0 Zm 0110 S T Zn Zda`.
fn enc_sve2_sqdmlal(size: u32, s: u32, t: u32) -> u32 {
    (0b01000100 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b0110 << 12)
        | (s << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// SVE2 add/subtract high narrow: `01000101 size 1 Zm 011 S R T Zn Zd`.
fn enc_sve2_addhn(size: u32, s: u32, r: u32, t: u32) -> u32 {
    (0b01000101 << 24)
        | (size << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b011 << 13)
        | (s << 12)
        | (r << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// SVE2 saturating extract narrow: `010001010 tszh 1 tszl 000010 vv T Zn Zd`.
/// vv (bits[12:11]): 00=SQXTN, 01=UQXTN, 10=SQXTUN.
fn enc_sve2_xtn(tsz: u32, variant: u32, t: u32) -> u32 {
    let tszh = (tsz >> 2) & 1;
    let tszl = tsz & 0x3;
    (0b010001010 << 23)
        | (tszh << 22)
        | (1 << 21)
        | (tszl << 19)
        | (0b000010 << 13)
        | (variant << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// SVE2 complex integer add: `01000101 size 00000 op 11011 rot Zm Zdn`.
/// Zm=z1(RN), Zdn=z0(RD).
fn enc_sve2_cadd(size: u32, op: u32, rot: u32) -> u32 {
    (0b01000101 << 24) | (size << 22) | (op << 16) | (0b11011 << 11) | (rot << 10) | (RN << 5) | RD
}
/// SVE2 complex integer multiply-add: `01000100 size 0 Zm 001 op rot Zn Zda`.
/// Zm=z2(RM), Zn=z1(RN), Zda=z0(RD).
fn enc_sve2_cmla(size: u32, op: u32, rot: u32) -> u32 {
    (0b01000100 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b001 << 13)
        | (op << 12)
        | (rot << 10)
        | (RN << 5)
        | RD
}
/// SVE2 shift left long: `010001010 tszh 0 tszl imm3 1010 U T Zn Zd`. The
/// (tsz,imm3) encodes the source size and shift: tsz:imm3 = src_bits + amount.
fn enc_sve2_shll(tsz: u32, imm3: u32, u: u32, t: u32) -> u32 {
    let tszh = (tsz >> 2) & 1;
    let tszl = tsz & 0x3;
    (0b010001010 << 23)
        | (tszh << 22)
        | (tszl << 19)
        | (imm3 << 16)
        | (0b1010 << 12)
        | (u << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}

/// SVE2 shift right and accumulate: `01000101 tszh 0 tszl imm3 1110 R U Zn Zda`.
fn enc_sve2_ssra(tsz: u32, imm3: u32, r: u32, u: u32) -> u32 {
    let tszh = (tsz >> 2) & 0x3;
    let tszl = tsz & 0x3;
    (0b01000101 << 24)
        | (tszh << 22)
        | (tszl << 19)
        | (imm3 << 16)
        | (0b1110 << 12)
        | (r << 11)
        | (u << 10)
        | (RN << 5)
        | RD
}
/// SVE2 shift and insert: `01000101 tszh 0 tszl imm3 11110 op Zn Zd`. op: 0=SRI,
/// 1=SLI.
fn enc_sve2_sri(tsz: u32, imm3: u32, op: u32) -> u32 {
    let tszh = (tsz >> 2) & 0x3;
    let tszl = tsz & 0x3;
    (0b01000101 << 24)
        | (tszh << 22)
        | (tszl << 19)
        | (imm3 << 16)
        | (0b11110 << 11)
        | (op << 10)
        | (RN << 5)
        | RD
}
/// (tsz, imm3) for a same-size right shift of `amount` (1..=esize): 2*esize-amt.
fn ssra_tsz_imm(esize_bits: u32, amount: u32) -> (u32, u32) {
    let tszimm = 2 * esize_bits - amount;
    ((tszimm >> 3) & 0xF, tszimm & 0x7)
}
/// (tsz, imm3) for a same-size left shift of `amount` (0..esize): esize+amount.
fn sli_tsz_imm(esize_bits: u32, amount: u32) -> (u32, u32) {
    let tszimm = esize_bits + amount;
    ((tszimm >> 3) & 0xF, tszimm & 0x7)
}

/// SVE2 FMLALB/FMLALT/FMLSLB/FMLSLT: `0110 0100 10 1 Zm 10 sub 00 T Zn Zd`.
/// sub=bit13 (0=FMLAL,1=FMLSL), T=bit10. Zn=z1(RN), Zm=z2(RM), Zda=z0(RD).
fn enc_sve2_fmlal(sub: u32, top: u32) -> u32 {
    (0x64 << 24)
        | (0b10 << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b10 << 14)
        | (sub << 13)
        | (top << 10)
        | (RN << 5)
        | RD
}

/// SVE BFDOT (zzzz): `01100100 01 1 Zm 100000 Zn Zda`. Zn=z1(RN), Zda=z0(RD).
fn enc_sve_bfdot(zm: u32) -> u32 {
    (0x64 << 24) | (0b01 << 22) | (1 << 21) | (zm << 16) | (0b100000 << 10) | (RN << 5) | RD
}

/// SVE BFDOT (indexed zzxw): `01100100 01 1 idx Zm[2:0] 010000 Zn Zda`.
/// index=bits[20:19], Zm z0-z7. Zn=z1(RN), Zda=z0(RD).
fn enc_sve_bfdot_idx(index: u32, zm: u32) -> u32 {
    (0x64 << 24)
        | (0b01 << 22)
        | (1 << 21)
        | (index << 19)
        | ((zm & 0x7) << 16)
        | (0b010000 << 10)
        | (RN << 5)
        | RD
}

/// SVE BFMLALB/T (zzzw): `01100100 11 1 Zm 10000 T Zn Zda`. T=bit10. Zn=z1(RN),
/// Zda=z0(RD).
fn enc_sve_bfmlal(top: u32, zm: u32) -> u32 {
    (0x64 << 24)
        | (0b11 << 22)
        | (1 << 21)
        | (zm << 16)
        | (0b10000 << 11)
        | (top << 10)
        | (RN << 5)
        | RD
}

/// SVE BFMLALB/T by indexed element (zzxw): bf16 variant of enc_sve2_fmlal_idx
/// (bits[23:22]=11, add only). T=bit10, index 0-7, Zm z0-z7.
fn enc_sve_bfmlal_idx(top: u32, index: u32, zm: u32) -> u32 {
    (0x64 << 24)
        | (0b11 << 22)
        | (1 << 21)
        | (((index >> 1) & 0x3) << 19)
        | ((zm & 0x7) << 16)
        | (0b01 << 14)
        | ((index & 1) << 11)
        | (top << 10)
        | (RN << 5)
        | RD
}

/// SVE2 FMLAL/FMLSL by indexed element: `01100100 10 1 i[2:1] Zm[2:0] 01 sub 0
/// i[0] T Zn Zda`. sub=bit13, T=bit10. index 0-7, Zm z0-z7. Zn=z1(RN), Zda=z0.
fn enc_sve2_fmlal_idx(sub: u32, top: u32, index: u32, zm: u32) -> u32 {
    (0x64 << 24)
        | (0b10 << 22)
        | (1 << 21)
        | (((index >> 1) & 0x3) << 19)
        | ((zm & 0x7) << 16)
        | (0b01 << 14)
        | (sub << 13)
        | ((index & 1) << 11)
        | (top << 10)
        | (RN << 5)
        | RD
}

/// SVE FSCALE: `01100101 size 00 1001 100 Pg Zm Zdn`. Zdn=z0(RD), Zm=z1(RN),
/// Pg=p0.
fn enc_sve_fscale(size: u32) -> u32 {
    (0x65 << 24) | (size << 22) | (0b01001 << 16) | (0b100 << 13) | (RN << 5) | RD
}

/// SVE FEXPA: `00000100 size 1 00000 101110 Zn Zd`. Zn=z1(RN), Zd=z0(RD).
fn enc_sve_fexpa(size: u32) -> u32 {
    (0x04 << 24) | (size << 22) | (1 << 21) | (0b101110 << 10) | (RN << 5) | RD
}

/// SVE FP multiply/multiply-add by indexed element: 0x64, bit21==1, op=bits
/// [15:10] (000000 FMLA, 000001 FMLS, 001000 FMUL). size 1=.h,2=.s,3=.d with
/// per-size index/Zm packing. Zn=z1(RN), Zd=z0(RD).
fn enc_sve_fp_idx(op: u32, size: u32, index: u32, zm: u32) -> u32 {
    let base = (0x64u32 << 24) | (1 << 21) | (op << 10) | (RN << 5) | RD;
    match size {
        1 => base | (((index >> 2) & 1) << 22) | ((index & 0x3) << 19) | ((zm & 0x7) << 16),
        2 => base | (0b10 << 22) | ((index & 0x3) << 19) | ((zm & 0x7) << 16),
        _ => base | (0b11 << 22) | ((index & 1) << 20) | ((zm & 0xF) << 16),
    }
}

/// SVE FRECPS/FRSQRTS: `01100101 size 0 Zm 00011 r Zn Zd`. r=bit10 (0=FRECPS,
/// 1=FRSQRTS). Zn=z1(RN), Zm=z2(RM), Zd=z0(RD).
fn enc_sve_recps(size: u32, rsqrt: u32) -> u32 {
    (0x65 << 24) | (size << 22) | (RM << 16) | (0b00011 << 11) | (rsqrt << 10) | (RN << 5) | RD
}

/// SVE integer dot product (vector SDOT/UDOT): `01000100 1 sz 0 Zm 00000 u Zn
/// Zda`. sz: 0=.s,1=.d; u=bit10. Zn=z1(RN), Zm=z2(RM), Zda=z0(RD).
fn enc_sve_dot_vec(sz: u32, u: u32) -> u32 {
    (0x44 << 24) | (1 << 23) | (sz << 22) | (RM << 16) | (u << 10) | (RN << 5) | RD
}

/// SVE USDOT (vector): `01000100 10 0 Zm 011110 Zn Zda`. Zn=z1, Zm=z2, Zda=z0.
fn enc_sve_usdot_vec() -> u32 {
    (0x44 << 24) | (0b10 << 22) | (RM << 16) | (0b011110 << 10) | (RN << 5) | RD
}

/// SVE integer dot product (indexed): `01000100 1 sz 1 <idx:Zm> op Zn Zda`.
/// sz 0=.s (index bits[20:19], Zm[18:16]), 1=.d (index bit20, Zm[19:16]). op=
/// bits[15:10]. Zn=z1(RN), Zda=z0(RD).
fn enc_sve_dot_idx(sz: u32, op: u32, index: u32, zm: u32) -> u32 {
    let field = if sz == 0 {
        ((index & 0x3) << 3) | (zm & 0x7)
    } else {
        ((index & 1) << 4) | (zm & 0xF)
    };
    (0x44 << 24) | (1 << 23) | (sz << 22) | (1 << 21) | (field << 16) | (op << 10) | (RN << 5) | RD
}

/// SVE predicated FP FMA: `01100101 size 1 Rm op3 Pg Rn Rd`. op3=bits[15:13]
/// (0-3 FMLA/FMLS/FNMLA/FNMLS, 4-7 FMAD/FMSB/FNMAD/FNMSB). Rd=z0, Rn=z1, Rm=z2,
/// Pg=p0.
fn enc_sve_fp_fma(size: u32, op3: u32) -> u32 {
    (0x65 << 24) | (size << 22) | (1 << 21) | (RM << 16) | (op3 << 13) | (RN << 5) | RD
}

/// SVE predicated integer multiply-add: `00000100 size 0 Rm op3 Pg Rn Rd`.
/// op3=bits[15:13] (010 MLA, 011 MLS, 110 MAD, 111 MSB). Rd=z0, Rn=z1, Rm=z2.
fn enc_sve_int_mla(size: u32, op3: u32) -> u32 {
    (0x04 << 24) | (size << 22) | (RM << 16) | (op3 << 13) | (RN << 5) | RD
}

/// SVE predicate ZIP/UZP/TRN: `00000101 esz 10 Pm 010 opc 0 Pn 0 Pd`. opc=bits
/// [12:10]. Pd=p0, Pn=p1, Pm=p2.
fn enc_sve_pred_permute(esz: u32, opc: u32) -> u32 {
    (0x05 << 24) | (esz << 22) | (0b10 << 20) | (2 << 16) | (0b010 << 13) | (opc << 10) | (1 << 5)
}

/// SVE predicate REV (REV_p): `00000101 esz 110100 010000 Pn Pd`. Pd=p0, Pn=p1.
fn enc_sve_rev_p(esz: u32) -> u32 {
    (0x05 << 24) | (esz << 22) | (0b110100 << 16) | (0b010000 << 10) | (1 << 5)
}

/// SVE FP compare (register): `01100101 size 0 Zm cc13 Pg Zn bit4 Pd`. Pg=p1,
/// Zn=z1(RN), Zm=z2(RM), Pd=p0.
fn enc_sve_fp_cmp(size: u32, cc13: u32, bit4: u32) -> u32 {
    (0x65 << 24) | (size << 22) | (RM << 16) | (cc13 << 13) | (1 << 10) | (RN << 5) | (bit4 << 4)
}

/// SQINCP/UQINCP/SQDECP/UQDECP, GPR form: `00100101 esz 1010 d u 10001 sf 0 Pg
/// Rdn`. d=0 inc/1 dec, u=0 signed/1 unsigned, sf=0 32-bit/1 64-bit. Rdn=rd.
fn enc_sve_sincdecp_r(esz: u32, d: u32, u: u32, sf64: u32, pg: u32, rd: u32) -> u32 {
    (0x25 << 24)
        | (esz << 22)
        | (0b1010 << 18)
        | (d << 17)
        | (u << 16)
        | (0b10001 << 11)
        | (sf64 << 10)
        | (pg << 5)
        | rd
}

/// SQINCP/UQINCP/SQDECP/UQDECP, vector form: `00100101 esz 1010 d u 10000 00 Pg
/// Zdn`. Per-element saturating add/sub (esz>=1). Zdn=rd.
fn enc_sve_sincdecp_z(esz: u32, d: u32, u: u32, pg: u32, rd: u32) -> u32 {
    (0x25 << 24)
        | (esz << 22)
        | (0b1010 << 18)
        | (d << 17)
        | (u << 16)
        | (0b10000 << 11)
        | (pg << 5)
        | rd
}

/// SVE FP compare with zero: `01100101 size 0100 sub 001 Pg Zn bit4 Pd`. Pg=p1,
/// Zn=z1(RN), Pd=p0.
fn enc_sve_fp_cmp0(size: u32, sub: u32, bit4: u32) -> u32 {
    (0x65 << 24)
        | (size << 22)
        | (0b0100 << 18)
        | (sub << 16)
        | (0b001 << 13)
        | (1 << 10)
        | (RN << 5)
        | (bit4 << 4)
}

/// SVE integer compare with signed immediate: `00100101 size 0 imm5 cc13 Pg Zn
/// bit4 Pd`. Pg=p1, Zn=z1(RN), Pd=p0.
fn enc_sve_cmp_imm_s(size: u32, cc13: u32, bit4: u32, imm5: u32) -> u32 {
    (0x25 << 24)
        | (size << 22)
        | ((imm5 & 0x1F) << 16)
        | (cc13 << 13)
        | (1 << 10)
        | (RN << 5)
        | (bit4 << 4)
}

/// SVE integer compare with unsigned immediate: `00100100 size 1 imm7 lo Pg Zn
/// hi Pd`. Pg=p1, Zn=z1(RN), Pd=p0.
fn enc_sve_cmp_imm_u(size: u32, lo: u32, hi: u32, imm7: u32) -> u32 {
    (0x24 << 24)
        | (size << 22)
        | (1 << 21)
        | ((imm7 & 0x7F) << 14)
        | (lo << 13)
        | (1 << 10)
        | (RN << 5)
        | (hi << 4)
}

/// SVE BFCVT (f32->bf16, predicated): `01100101 10 0010 10 101 Pg Zn Zd`. Pg=p0,
/// Zn=z1(RN), Zd=z0(RD).
fn enc_sve_bfcvt() -> u32 {
    (0x65 << 24) | (0b10 << 22) | (0b0010 << 18) | (0b10 << 16) | (0b101 << 13) | (RN << 5) | RD
}

/// SVE predicated integer/FP unary: `00000100 size opc6 101 Pg Zn Zd`.
/// opc6=bits[21:16]. Pg=p0, Zn=z1(RN), Zd=z0(RD).
fn enc_sve_pred_unary(size: u32, opc6: u32) -> u32 {
    (0x04 << 24) | (size << 22) | (opc6 << 16) | (0b101 << 13) | (RN << 5) | RD
}

/// SVE UNPK (SUNPK/UUNPK HI/LO): `00000101 size 1100 u h 001110 Zn Zd`.
/// Zn=z1(RN), Zd=z0(RD).
fn enc_sve_unpk(size: u32, u: u32, h: u32) -> u32 {
    (0x05 << 24)
        | (size << 22)
        | (0b1100 << 18)
        | (u << 17)
        | (h << 16)
        | (0b001110 << 10)
        | (RN << 5)
        | RD
}

/// SVE DUPM: `00000101 110000 N immr imms Zd`. Zd=z0(RD).
fn enc_sve_dupm(n: u32, immr: u32, imms: u32) -> u32 {
    (0x05 << 24) | (0b110000 << 18) | (n << 17) | (immr << 11) | (imms << 5) | RD
}

/// SVE FRECPE/FRSQRTE: `01100101 size 00111 r 001100 Zn Zd`. r=bit16. Zn=z1(RN),
/// Zd=z0(RD).
fn enc_sve_frecpe(size: u32, rsqrt: u32) -> u32 {
    (0x65 << 24) | (size << 22) | ((0b001110 | rsqrt) << 16) | (0b001100 << 10) | (RN << 5) | RD
}

/// SVE predicated shift by vector (ASR/LSR/LSL and reversed ASRR/LSRR/LSLR):
/// `00000100 size 010 opc 100 Pg Zm Zdn`. opc=bits[18:16]. Pg=p0, Zm=z1(RN),
/// Zdn=z0(RD).
fn enc_sve_shift_pred_v(size: u32, opc: u32) -> u32 {
    (0x04 << 24) | (size << 22) | (0b010 << 19) | (opc << 16) | (0b100 << 13) | (RN << 5) | RD
}

/// SVE REVB/REVH/REVW/RBIT: `00000101 size 1 001 op 100 Pg Zn Zd`. op=bits[17:16]
/// (00 REVB,01 REVH,10 REVW,11 RBIT). Pg=p0, Zn=z1(RN), Zd=z0(RD).
fn enc_sve_rev_rbit(size: u32, op: u32) -> u32 {
    (0x05 << 24)
        | (size << 22)
        | (1 << 21)
        | (0b001 << 18)
        | (op << 16)
        | (0b100 << 13)
        | (RN << 5)
        | RD
}

/// SVE INSR: `00000101 size 1 form 001110 Rm Zdn`. form=bits[20:16] (00100 GPR,
/// 10100 SIMD scalar). Rm/Vm=z1(RN), Zdn=z0(RD).
fn enc_sve_insr(size: u32, f: u32) -> u32 {
    let form = if f == 1 { 0b10100u32 } else { 0b00100 };
    (0x05 << 24) | (size << 22) | (1 << 21) | (form << 16) | (0b001110 << 10) | (RN << 5) | RD
}

/// SVE CLASTA/CLASTB to vector/scalar: `00000101 size 1010 sc b 100 Pg Zm Zdn`.
/// sc=bit17 (0=vector,1=scalar), b=bit16 (CLASTB). Pg=p0, Zm/Zn=z1(RN), Zd=z0(RD).
fn enc_sve_clast(size: u32, scalar: u32, before: u32) -> u32 {
    (0x05 << 24)
        | (size << 22)
        | (0b10100 << 17)
        | (scalar << 17)
        | (before << 16)
        | (0b100 << 13)
        | (RN << 5)
        | RD
}

/// SVE FCPY: `00000101 size 01 Pg4 110 imm8 Zd`. Pg4=p0, Zd=z0(RD).
fn enc_sve_fcpy(size: u32, imm8: u32) -> u32 {
    (0x05 << 24) | (size << 22) | (0b01 << 20) | (0b110 << 13) | ((imm8 & 0xFF) << 5) | RD
}

/// SVE FDUP: `00100101 size 111001110 imm8 Zd`. Zd=z0(RD).
fn enc_sve_fdup(size: u32, imm8: u32) -> u32 {
    (0x25 << 24) | (size << 22) | (0b111001110 << 13) | ((imm8 & 0xFF) << 5) | RD
}

/// SVE CTERMEQ/CTERMNE: `00100101 1 sf 1 Rm 001000 Rn 0 ne`. sf=bit22 (0=32,
/// 1=64), ne=bit4. Rn=x1(RN), Rm=x2(RM).
fn enc_sve_cterm(sf: u32, ne: u32) -> u32 {
    (0x25 << 24)
        | (1 << 23)
        | (sf << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b001000 << 10)
        | (RN << 5)
        | (ne << 4)
}

/// SVE2 predicated integer ALU: `01000100 size opc6 op3 Pg Zm Zdn`. opc6=bits
/// [21:16], op3=bits[15:13] (100 binary, 101 unary). Pg=p0, Zm/Zn=z1(RN),
/// Zdn/Zd=z0(RD).
fn enc_sve2_pred_alu(size: u32, opc6: u32, op3: u32) -> u32 {
    (0x44 << 24) | (size << 22) | (opc6 << 16) | (op3 << 13) | (RN << 5) | RD
}

/// SVE2 CMLA by indexed element: `0100 0100 size 1 <idx:Zm> 0110 rot Zn Zda`.
/// size 2=.h,3=.s; .h: index=bits[20:19] Zm=bits[18:16]; .s: index=bit20
/// Zm=bits[19:16]. rot=bits[11:10]. Zn=z1(RN), Zda=z0(RD).
fn enc_sve2_cmla_idx(size: u32, index: u32, zm: u32, rot: u32) -> u32 {
    let field = if size == 2 {
        ((index & 0x3) << 3) | (zm & 0x7)
    } else {
        ((index & 1) << 4) | (zm & 0xF)
    };
    (0x44 << 24)
        | (size << 22)
        | (1 << 21)
        | (field << 16)
        | (0b0110 << 12)
        | (rot << 10)
        | (RN << 5)
        | RD
}

/// SVE FCMLA by indexed element: like enc_sve2_cmla_idx but 0x64 / opcode 0001.
fn enc_sve_fcmla_idx(size: u32, index: u32, zm: u32, rot: u32) -> u32 {
    let field = if size == 2 {
        ((index & 0x3) << 3) | (zm & 0x7)
    } else {
        ((index & 1) << 4) | (zm & 0xF)
    };
    (0x64 << 24)
        | (size << 22)
        | (1 << 21)
        | (field << 16)
        | (0b0001 << 12)
        | (rot << 10)
        | (RN << 5)
        | RD
}

/// SVE2 CDOT: `0100 0100 size 0 Zm 0001 rot Zn Zda`. size 2=.s,3=.d; rot=bits
/// [11:10]. Zn=z1(RN), Zm=z2(RM), Zda=z0(RD).
fn enc_sve2_cdot(size: u32, rot: u32) -> u32 {
    (0x44 << 24) | (size << 22) | (RM << 16) | (0b0001 << 12) | (rot << 10) | (RN << 5) | RD
}

/// SVE FCADD (predicated): `01100100 esz 00000 rot 100 Pg Zm Zdn`. rot=bit16.
/// Zdn=z0(RD), Zm=z1(RN at bits[9:5]), Pg=p0.
fn enc_sve_fcadd(size: u32, rot: u32) -> u32 {
    (0x64 << 24) | (size << 22) | (rot << 16) | (0b100 << 13) | (RN << 5) | RD
}

/// SVE FCMLA (predicated): `01100100 esz 0 Zm 0 rot 1? Pg Zn Zda`. rot=bits[14:13].
/// Zda=z0(RD), Zn=z1(RN), Zm=z2(RM), Pg=p0.
fn enc_sve_fcmla(size: u32, rot: u32) -> u32 {
    (0x64 << 24) | (size << 22) | (RM << 16) | (rot << 13) | (RN << 5) | RD
}

/// SVE FTSMUL: `01100101 size 0 Zm 000011 Zn Zd`. Zn=z1(RN), Zm=z2(RM), Zd=z0.
fn enc_sve_ftsmul(size: u32) -> u32 {
    (0x65 << 24) | (size << 22) | (RM << 16) | (0b000011 << 10) | (RN << 5) | RD
}

/// SVE FTSSEL: `00000100 size 1 Zm 101100 Zn Zd`. Zn=z1(RN), Zm=z2(RM), Zd=z0.
fn enc_sve_ftssel(size: u32) -> u32 {
    (0x04 << 24) | (size << 22) | (1 << 21) | (RM << 16) | (0b101100 << 10) | (RN << 5) | RD
}

/// SVE FTMAD: `01100101 esz 010 imm3 100000 Zm Zdn`. Zm=z1(RN), Zdn=z0(RD).
fn enc_sve_ftmad(size: u32, imm: u32) -> u32 {
    (0x65 << 24) | (size << 22) | (0b010 << 19) | (imm << 16) | (0b100000 << 10) | (RN << 5) | RD
}

/// SVE FMMLA/BFMMLA (FP matrix multiply): `0110 0100 sz 1 Zm 111001 Zn Zda`.
/// sz: 01=BFMMLA, 10=FMMLA.s, 11=FMMLA.d. Zn=z1(RN), Zm=z2(RM), Zda=z0(RD).
fn enc_sve_fmmla(sz: u32) -> u32 {
    (0x64 << 24) | (sz << 22) | (1 << 21) | (RM << 16) | (0b111001 << 10) | (RN << 5) | RD
}

/// SVE I8MM matrix multiply (SMMLA/UMMLA/USMMLA): `0100 0101 uns 0 Zm 100110
/// Zn Zda`. uns=bits[23:22] (00=SMMLA,10=USMMLA,11=UMMLA). Zn=z1(RN), Zm=z2(RM),
/// Zda=z0(RD).
fn enc_sve_mmla(uns: u32) -> u32 {
    (0x45 << 24) | (uns << 22) | (RM << 16) | (0b100110 << 10) | (RN << 5) | RD
}

/// SVE2 XAR (exclusive-or and rotate right by imm): `00000100 tszh 1 tszl imm3
/// 001101 Zm Zdn`. size_log 0=.b..3=.d; amount in 1..=esize_bits. Zm=z1(RN),
/// Zdn=z0(RD).
fn enc_sve2_xar(size_log: u32, amount: u32) -> u32 {
    let bits = 8u32 << size_log;
    let tszimm = 2 * bits - amount;
    let tsz = tszimm >> 3;
    let imm3 = tszimm & 7;
    (0x04 << 24)
        | ((tsz >> 2) << 22)
        | (1 << 21)
        | ((tsz & 3) << 19)
        | (imm3 << 16)
        | (0b001101 << 10)
        | (RN << 5)
        | RD
}

/// SVE FCVTX (double->single, round-to-odd, predicated): `01100101 00 0010 10
/// 101 Pg Zn Zd`. Pg=p0, Zn=z1(RN), Zd=z0(RD).
fn enc_sve_fcvtx() -> u32 {
    (0x65 << 24) | (0b0010 << 18) | (0b10 << 16) | (0b101 << 13) | (RN << 5) | RD
}

/// SVE2 ADCLB/ADCLT/SBCLB/SBCLT: `0100 0101 inv size 0 Zm 11010 T Zn Zda`.
/// inv=bit23 (0=ADC,1=SBC); d_form=bit22 (0=.s,1=.d); T=bit10.
fn enc_sve2_adcl(inv: u32, d_form: u32, top: u32) -> u32 {
    (0x45 << 24)
        | (inv << 23)
        | (d_form << 22)
        | (RM << 16)
        | (0b11010 << 11)
        | (top << 10)
        | (RN << 5)
        | RD
}

/// SVE2 EORBT/EORTB: `0100 0101 size 0 Zm 10010 TB Zn Zd`. TB=bit10 (0=EORBT).
fn enc_sve2_eorbt(size: u32, tb: u32) -> u32 {
    (0x45 << 24) | (size << 22) | (RM << 16) | (0b10010 << 11) | (tb << 10) | (RN << 5) | RD
}

/// SVE2 PMULLB/PMULLT: `0100 0101 size 0 Zm 011 01 T Zn Zd`. size: 0=.q(64->128),
/// 1=.h(8->16), 3=.d(32->64); T=bit10 (0=B,1=T). Zn=z1(RN), Zm=z2(RM), Zd=z0(RD).
fn enc_sve2_pmull(size: u32, top: u32) -> u32 {
    (0x45 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b011 << 13)
        | (1 << 11)
        | (top << 10)
        | (RN << 5)
        | RD
}

/// SVE2 AES/SM4E family: `0100 0101 <third> 11100 op Zm Zd`. third=bits[23:16]
/// (0x22 AESE/AESD, 0x23 SM4E); op=bit10. Zm=z1(RN), Zd=z0(RD).
fn enc_sve2_aes(third: u32, op: u32) -> u32 {
    (0x45 << 24) | (third << 16) | (0b11100 << 11) | (op << 10) | (RN << 5) | RD
}

/// SVE2 AESMC/AESIMC: `0100 0101 00100000 11100 op 00000 Zd`. op=bit10
/// (0=AESMC,1=AESIMC). Zd=z0(RD); the bits[9:5] source field is fixed 0.
fn enc_sve2_aesmc(op: u32) -> u32 {
    (0x45 << 24) | (0x20 << 16) | (0b11100 << 11) | (op << 10) | RD
}

/// SVE2 SM4EKEY/RAX1: `0100 0101 001 Zm 11110 op Zn Zd`. op=bit10 (0=SM4EKEY,
/// 1=RAX1). Zn=z1(RN), Zm=z2(RM), Zd=z0(RD).
fn enc_sve2_sm4ekey(op: u32) -> u32 {
    (0x45 << 24) | (1 << 21) | (RM << 16) | (0b11110 << 11) | (op << 10) | (RN << 5) | RD
}

/// SVE2 HISTCNT: `0100 0101 size 1 Zm 110 Pg Zn Zd`. size: 2=s,3=d. Pg=p0,
/// Zn=z1(RN), Zm=z2(RM), Zd=z0(RD).
fn enc_sve2_histcnt(size: u32) -> u32 {
    (0x45 << 24) | (size << 22) | (1 << 21) | (RM << 16) | (0b110 << 13) | (RN << 5) | RD
}

/// SVE2 HISTSEG (byte only): `0100 0101 00 1 Zm 101000 Zn Zd`. Zn=z1(RN),
/// Zm=z2(RM), Zd=z0(RD).
fn enc_sve2_histseg() -> u32 {
    (0x45 << 24) | (1 << 21) | (RM << 16) | (0b101000 << 10) | (RN << 5) | RD
}

/// SVE2 MATCH/NMATCH (character match -> predicate): `0100 0101 size 1 Zm 100
/// Pg Zn op4 Pd`. size: 0=b,1=h; op4=bit4 (0=MATCH,1=NMATCH). Pg=p1, Zn=z1(RN),
/// Zm=z2(RM), Pd=p0.
fn enc_sve2_match(size: u32, nmatch: u32) -> u32 {
    (0x45 << 24)
        | (size << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b100 << 13)
        | (1 << 10)
        | (RN << 5)
        | (nmatch << 4)
}

/// SVE2 WHILERW/WHILEWR (memory-hazard predicate): `0010 0101 sz 1 Rm 001100
/// Rn rw Pd`. sz: 0=b..3=d; rw: 1=WHILERW, 0=WHILEWR. Rn=x1(RN), Rm=x2(RM),
/// Pd=p0.
fn enc_whilerw(sz: u32, rw: u32) -> u32 {
    (0x25 << 24) | (sz << 22) | (1 << 21) | (RM << 16) | (0b001100 << 10) | (RN << 5) | (rw << 4)
}

/// SVE2 integer multiply (indexed): `0100 0100 <size:idx:Zm> <op6> Zn Zd`.
/// `op6` is bits[15:10] (MUL/SQDMULH/SQRDMULH/MLA/MLS); `size` is 1=h,2=s,3=d;
/// the (index, Zm) packing depends on size. Zn=z1(RN), Zd=z0(RD).
fn enc_sve2_mul_idx(op6: u32, size: u32, index: u32, zm: u32) -> u32 {
    let field = match size {
        1 => {
            // H: bit23=0, bit22=i[2], bit21=1, bit20=i[1], bit19=i[0], Zm[18:16].
            (((index >> 2) & 1) << 6)
                | (1 << 5)
                | (((index >> 1) & 1) << 4)
                | ((index & 1) << 3)
                | (zm & 0x7)
        }
        2 => {
            // S: bits[23:22]=10, bit21=1, bit20=i[1], bit19=i[0], Zm[18:16].
            (0b10 << 6) | (1 << 5) | (((index >> 1) & 1) << 4) | ((index & 1) << 3) | (zm & 0x7)
        }
        _ => {
            // D: bits[23:22]=11, bit21=1, bit20=i[0], Zm[19:16].
            (0b11 << 6) | (1 << 5) | ((index & 1) << 4) | (zm & 0xF)
        }
    };
    (0b01000100 << 24) | (field << 16) | (op6 << 10) | (RN << 5) | RD
}

/// SVE2 widening multiply-add long by indexed element: `0100 0100 size 1
/// <idx/Zm> <op4> i0 T Zn Zd`. op=bits[15:12]; size 2=.s(h src), 3=.d(s src);
/// per-size index/Zm packing; T=bit10. Zn=z1(RN), Zd=z0(RD).
fn enc_sve2_mull_idx(op: u32, size: u32, index: u32, zm: u32, top: u32) -> u32 {
    let (field, i0) = if size == 2 {
        // .s: i2->bit20, i1->bit19, Zm in bits[18:16]; i0 -> bit11.
        (
            (((index >> 2) & 1) << 4) | (((index >> 1) & 1) << 3) | (zm & 0x7),
            index & 1,
        )
    } else {
        // .d: i1->bit20, Zm in bits[19:16]; i0 -> bit11.
        ((((index >> 1) & 1) << 4) | (zm & 0xF), index & 1)
    };
    (0b01000100 << 24)
        | (size << 22)
        | (1 << 21)
        | (field << 16)
        | (op << 12)
        | (i0 << 11)
        | (top << 10)
        | (RN << 5)
        | RD
}

/// SVE2 FLOGB (find exponent): `01100101 00011 size 0 101 Pg Zn Zd`. size is
/// 1=h, 2=s, 3=d. Pg=p0, Zn=z1(RN), Zd=z0(RD).
fn enc_sve_flogb(size: u32) -> u32 {
    (0b01100101 << 24) | (0b00011 << 19) | (size << 17) | (0b101 << 13) | (RN << 5) | RD
}

/// Edge-case IEEE bit patterns of `esize` bytes for FLOGB: +0, -0, +inf, -inf,
/// qNaN, sNaN, smallest/largest subnormal, and a negative subnormal.
fn flogb_specials(esize: usize) -> Vec<u64> {
    match esize {
        2 => vec![
            0x0000, 0x8000, 0x7C00, 0xFC00, 0x7E00, 0x7C01, 0x0001, 0x03FF, 0x8001,
        ],
        4 => vec![
            0x0000_0000,
            0x8000_0000,
            0x7F80_0000,
            0xFF80_0000,
            0x7FC0_0000,
            0x7F80_0001,
            0x0000_0001,
            0x007F_FFFF,
            0x8000_0001,
        ],
        _ => vec![
            0,
            1 << 63,
            0x7FF0_0000_0000_0000,
            0xFFF0_0000_0000_0000,
            0x7FF8_0000_0000_0000,
            0x7FF0_0000_0000_0001,
            1,
            0x000F_FFFF_FFFF_FFFF,
            (1 << 63) | 1,
        ],
    }
}

/// SVE2 FCVTNT/FCVTLT/FCVTXNT: `01100100 opc 0010 opc2 101 Pg Zn Zd`. Pg=p0,
/// Zn=z1(RN), Zd=z0(RD).
fn enc_sve2_fcvtx(opc: u32, opc2: u32) -> u32 {
    (0b01100100 << 24)
        | (opc << 22)
        | (0b0010 << 18)
        | (opc2 << 16)
        | (0b101 << 13)
        | (RN << 5)
        | RD
}

/// SVE2 FP pairwise: `01100100 size 010 opc 100 Pg Zm Zdn`. Pg=p0, Zm=z1(RN),
/// Zdn=z0(RD). opc: 000=FADDP, 100=FMAXNMP, 101=FMINNMP, 110=FMAXP, 111=FMINP.
fn enc_sve2_fpairwise(size: u32, opc: u32) -> u32 {
    (0b01100100 << 24) | (size << 22) | (0b010 << 19) | (opc << 16) | (0b100 << 13) | (RN << 5) | RD
}

/// SVE2 add long pairwise accumulate: `01000100 size 00010 U 101 Pg Zn Zda`.
/// Pg=p0, Zn=z1(RN), Zda=z0(RD).
fn enc_sve2_adalp(size: u32, u: u32) -> u32 {
    (0b01000100 << 24) | (size << 22) | (0b00010 << 17) | (u << 16) | (0b101 << 13) | (RN << 5) | RD
}

/// SVE2 abs-diff accumulate long: `01000101 size 0 Zm 1100 U T Zn Zda`.
fn enc_sve2_abal(size: u32, u: u32, t: u32) -> u32 {
    (0b01000101 << 24)
        | (size << 22)
        | (RM << 16)
        | (0b1100 << 12)
        | (u << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}

/// SVE2 predicated pairwise: `01000100 size 010 opc U 101 Pg Zm Zdn`. Pg=p0,
/// Zm=z1(RN), Zdn=z0(RD).
fn enc_sve2_pairwise(size: u32, opc: u32, u: u32) -> u32 {
    (0b01000100 << 24)
        | (size << 22)
        | (0b010 << 19)
        | (opc << 17)
        | (u << 16)
        | (0b101 << 13)
        | (RN << 5)
        | RD
}

/// SVE2 bit permute: `01000101 size 0 Zm 1011 opc Zn Zd`. opc: 00=BEXT, 01=BDEP,
/// 10=BGRP. Zn=z1(RN), Zm=z2(RM), Zd=z0(RD).
fn enc_sve2_bperm(size: u32, opc: u32) -> u32 {
    (0b01000101 << 24) | (size << 22) | (RM << 16) | (0b1011 << 12) | (opc << 10) | (RN << 5) | RD
}

/// SVE2 SQDMULH/SQRDMULH: `00000100 size 1 Zm 01110 R Zn Zd`.
fn enc_sve2_sqdmulh(size: u32, r: u32) -> u32 {
    (0b00000100 << 24)
        | (size << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b01110 << 11)
        | (r << 10)
        | (RN << 5)
        | RD
}
/// SVE2 SQRDMLAH/SQRDMLSH: `01000100 size 0 Zm 01110 S Zn Zda`.
fn enc_sve2_sqrdmlah(size: u32, s: u32) -> u32 {
    (0b01000100 << 24) | (size << 22) | (RM << 16) | (0b01110 << 11) | (s << 10) | (RN << 5) | RD
}

/// SVE2 shift right narrow: `010001010 tszh 1 tszl imm3 00 op U R T Zn Zd`.
fn enc_sve2_shrn(tsz: u32, imm3: u32, op: u32, u: u32, r: u32, t: u32) -> u32 {
    let tszh = (tsz >> 2) & 1;
    let tszl = tsz & 0x3;
    (0b010001010 << 23)
        | (tszh << 22)
        | (1 << 21)
        | (tszl << 19)
        | (imm3 << 16)
        | (op << 13)
        | (u << 12)
        | (r << 11)
        | (t << 10)
        | (RN << 5)
        | RD
}
/// (tsz, imm3) for a shift-right-narrow with destination width `dst_bits` and
/// shift `amount` (1..=dst_bits): tsz:imm3 = 2*dst_bits - amount.
fn shrn_tsz_imm(dst_bits: u32, amount: u32) -> (u32, u32) {
    let tszimm = 2 * dst_bits - amount;
    ((tszimm >> 3) & 0x7, tszimm & 0x7)
}

/// SVE INDEX variants. base=imm5[9:5] or Xn; step=imm5[20:16] or Xm. Rn=x1, Rm=x2.
fn enc_index_ii(sz: u32, imm_step: u32, imm_base: u32) -> u32 {
    (0b00000100 << 24)
        | (sz << 22)
        | (1 << 21)
        | ((imm_step & 0x1F) << 16)
        | (0b010000 << 10)
        | ((imm_base & 0x1F) << 5)
        | RD
}
fn enc_index_ri(sz: u32, imm_step: u32) -> u32 {
    (0b00000100 << 24)
        | (sz << 22)
        | (1 << 21)
        | ((imm_step & 0x1F) << 16)
        | (0b010001 << 10)
        | (RN << 5)
        | RD // base = Xn (x1)
}
fn enc_index_ir(sz: u32, imm_base: u32) -> u32 {
    (0b00000100 << 24)
        | (sz << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b010010 << 10)
        | ((imm_base & 0x1F) << 5)
        | RD // step = Xm (x2)
}
fn enc_index_rr(sz: u32) -> u32 {
    (0b00000100 << 24) | (sz << 22) | (1 << 21) | (RM << 16) | (0b010011 << 10) | (RN << 5) | RD // base = x1, step = x2
}

/// SVE ZIP/UZP/TRN (unpredicated): `00000101 sz 1 Zm 011 opc Zn Zd`.
fn enc_sve_perm(sz: u32, opc: u32) -> u32 {
    (0b00000101 << 24)
        | (sz << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b011 << 13)
        | (opc << 10)
        | (RN << 5)
        | RD
}

/// SVE EXT (destructive): `00000101 001 imm8h 000 imm8l Zm Zdn`. imm8=imm8h:imm8l
/// is the byte offset into the concatenation Zm:Zdn. Zdn=Z0(RD), Zm=Z1(bits[9:5]).
fn enc_sve_ext(imm8: u32) -> u32 {
    let imm8h = (imm8 >> 3) & 0x1F;
    let imm8l = imm8 & 0x7;
    (0b00000101 << 24) | (0b001 << 21) | (imm8h << 16) | (imm8l << 10) | (RN << 5) | RD
}

/// SVE TBL (single source table): `00000101 size 1 Zm 001100 Zn Zd`.
/// Table=Zn(RN), indices=Zm(RM), dest=Zd(RD).
fn enc_sve_tbl(sz: u32) -> u32 {
    (0b00000101 << 24) | (sz << 22) | (1 << 21) | (RM << 16) | (0b001100 << 10) | (RN << 5) | RD
}

/// SVE FCVT (precision conversion): `01100101 opc 0010 opc2 101 Pg Zn Zd`.
/// Pg=p0, Zn=z1, Zd=z0.
fn enc_sve_fcvt(opc: u32, opc2: u32) -> u32 {
    (0b01100101 << 24)
        | (opc << 22)
        | (0b0010 << 18)
        | (opc2 << 16)
        | (0b101 << 13)
        | (RN << 5)
        | RD
}

/// SVE LD1 gather (64-bit scalar+vector, D-form): `1100010 msz ig1 Zm 1 U 0 Pg
/// Rn Zt`. ig1=10 unscaled / 11 scaled. Rn=x1 (base), Zm=z2 (offsets), Zt=z0.
fn enc_gather_d(msz: u32, scaled: bool, u: u32) -> u32 {
    let ig1: u32 = if scaled { 0b11 } else { 0b10 };
    (0b1100010 << 25)
        | (msz << 23)
        | (ig1 << 21)
        | (RM << 16)
        | (1 << 15)
        | (u << 14)
        | (RN << 5)
        | RD
}

/// SVE ST1 scatter (64-bit scalar+vector, D-form): `1110010 msz ig1 Zm 101 Pg
/// Rn Zt`. ig1=00 unscaled / 01 scaled. Rn=x1 (base), Zm=z2 (offsets), Zt=z0.
fn enc_scatter_d(msz: u32, scaled: bool) -> u32 {
    let ig1: u32 = if scaled { 0b01 } else { 0b00 };
    (0b1110010 << 25) | (msz << 23) | (ig1 << 21) | (RM << 16) | (0b101 << 13) | (RN << 5) | RD
}

/// SVE LDFF1 (first-fault, scalar+scalar): `1010010 dtype Rm 011 Pg Rn Zt`.
fn enc_ldff1(dtype: u32) -> u32 {
    (0b1010010 << 25) | (dtype << 21) | (RM << 16) | (0b011 << 13) | (RN << 5) | RD
}
/// SVE LDNF1 (non-fault, scalar+imm): `1010010 dtype 1 imm4 101 Pg Rn Zt`.
fn enc_ldnf1(dtype: u32, imm4: i32) -> u32 {
    (0b1010010 << 25)
        | (dtype << 21)
        | (1 << 20)
        | (((imm4 as u32) & 0xF) << 16)
        | (0b101 << 13)
        | (RN << 5)
        | RD
}

/// SVE FFR-manipulation encodings (fully fixed apart from the predicate fields).
fn enc_setffr() -> u32 {
    0x252C_9000
}
fn enc_wrffr(pn: u32) -> u32 {
    0x2528_9000 | ((pn & 0xF) << 5)
}
fn enc_rdffr(pd: u32) -> u32 {
    0x2519_F000 | (pd & 0xF)
}
fn enc_rdffr_pred(pd: u32, pg: u32) -> u32 {
    0x2518_F000 | ((pg & 0xF) << 5) | (pd & 0xF)
}

/// SVE LD1RQ (load-replicate quadword): `1010010 msz 00 0 imm4 001`(imm) /
/// `1010010 msz 00 Rm 000`(scalar). Rn=x1, Rm=x2, Zt=z0.
fn enc_ld1rq_i(msz: u32, imm4: i32) -> u32 {
    (0b1010010 << 25) | (msz << 23) | (((imm4 as u32) & 0xF) << 16) | (0b001 << 13) | (RN << 5) | RD
}
fn enc_ld1rq_r(msz: u32) -> u32 {
    (0b1010010 << 25) | (msz << 23) | (RM << 16) | (RN << 5) | RD
}

/// SVE LDNT1/STNT1 (non-temporal contiguous): `1010010 msz 000 imm4 111`(LD) /
/// `1110010 msz 001 imm4 111`(ST). Rn=x1, Zt=z0.
fn enc_ldnt1(msz: u32, imm4: i32) -> u32 {
    (0b1010010 << 25) | (msz << 23) | (((imm4 as u32) & 0xF) << 16) | (0b111 << 13) | (RN << 5) | RD
}
fn enc_stnt1(msz: u32, imm4: i32) -> u32 {
    (0b1110010 << 25)
        | (msz << 23)
        | (1 << 20)
        | (((imm4 as u32) & 0xF) << 16)
        | (0b111 << 13)
        | (RN << 5)
        | RD
}

/// SVE LD2/3/4 (de-interleaving): `1010010 msz opc 0 imm4 111 Pg Rn Zt`.
/// opc=nreg-1. Rn=x1, Zt=z0.
fn enc_ldn(msz: u32, nreg: u32, imm4: i32) -> u32 {
    (0b1010010 << 25)
        | (msz << 23)
        | ((nreg - 1) << 21)
        | (((imm4 as u32) & 0xF) << 16)
        | (0b111 << 13)
        | (RN << 5)
        | RD
}
/// SVE ST2/3/4 (interleaving): `1110010 msz opc 1 imm4 111 Pg Rn Zt`.
fn enc_stn(msz: u32, nreg: u32, imm4: i32) -> u32 {
    (0b1110010 << 25)
        | (msz << 23)
        | ((nreg - 1) << 21)
        | (1 << 20)
        | (((imm4 as u32) & 0xF) << 16)
        | (0b111 << 13)
        | (RN << 5)
        | RD
}

/// SVE LD1 gather (S-form vector base + immediate): `1000010 msz 01 imm5 1 U 0
/// Pg Zn Zt`. Zn=z1 (32-bit per-element bases), Zt=z0.
fn enc_gather_ai_s(msz: u32, imm5: u32, u: u32) -> u32 {
    (0b1000010 << 25)
        | (msz << 23)
        | (0b01 << 21)
        | ((imm5 & 0x1F) << 16)
        | (1 << 15)
        | (u << 14)
        | (RN << 5)
        | RD
}
/// SVE ST1 scatter (S-form vector base + immediate): `1110010 msz 11 imm5 101 Pg
/// Zn Zt`. Zn=z1, Zt=z0.
fn enc_scatter_ai_s(msz: u32, imm5: u32) -> u32 {
    (0b1110010 << 25)
        | (msz << 23)
        | (0b11 << 21)
        | ((imm5 & 0x1F) << 16)
        | (0b101 << 13)
        | (RN << 5)
        | RD
}

/// SVE LD1 gather (vector base + immediate, D-form): `1100010 msz 01 imm5 1 U 0
/// Pg Zn Zt`. Zn=z1 (per-element bases), Zt=z0.
fn enc_gather_ai(msz: u32, imm5: u32, u: u32) -> u32 {
    (0b1100010 << 25)
        | (msz << 23)
        | (0b01 << 21)
        | ((imm5 & 0x1F) << 16)
        | (1 << 15)
        | (u << 14)
        | (RN << 5)
        | RD
}

/// SVE ST1 scatter (vector base + immediate, D-form): `1110010 msz 10 imm5 101
/// Pg Zn Zt`. Zn=z1, Zt=z0.
fn enc_scatter_ai(msz: u32, imm5: u32) -> u32 {
    (0b1110010 << 25)
        | (msz << 23)
        | (0b10 << 21)
        | ((imm5 & 0x1F) << 16)
        | (0b101 << 13)
        | (RN << 5)
        | RD
}

/// SVE LD1 gather (unpacked x32: D-elem, 32-bit offset): `1100010 msz xs scaled
/// Zm 0 U 0 Pg Rn Zt`. Rn=x1, Zm=z2, Zt=z0.
fn enc_gather_x32(msz: u32, xs: u32, scaled: bool, u: u32) -> u32 {
    let sc: u32 = if scaled { 1 } else { 0 };
    (0b1100010 << 25)
        | (msz << 23)
        | (xs << 22)
        | (sc << 21)
        | (RM << 16)
        | (u << 14)
        | (RN << 5)
        | RD
}

/// SVE ST1 scatter (unpacked x32: D-elem, 32-bit offset): `1110010 msz 0 scaled
/// Zm 1 xs 0 Pg Rn Zt` (ig1 high bit clear). Rn=x1, Zm=z2, Zt=z0.
fn enc_scatter_x32(msz: u32, xs: u32, scaled: bool) -> u32 {
    let sc: u32 = if scaled { 1 } else { 0 };
    (0b1110010 << 25)
        | (msz << 23)
        | (sc << 21)
        | (RM << 16)
        | (1 << 15)
        | (xs << 14)
        | (RN << 5)
        | RD
}

/// SVE LD1 gather (32-bit scalar+vector, S-form): `1000010 msz xs scaled Zm 0 U
/// 0 Pg Rn Zt`. Rn=x1 (base), Zm=z2 (offsets), Zt=z0.
fn enc_gather_s(msz: u32, xs: u32, scaled: bool, u: u32) -> u32 {
    let sc: u32 = if scaled { 1 } else { 0 };
    (0b1000010 << 25)
        | (msz << 23)
        | (xs << 22)
        | (sc << 21)
        | (RM << 16)
        | (u << 14)
        | (RN << 5)
        | RD
}

/// SVE ST1 scatter (32-bit scalar+vector, S-form): `1110010 msz ig1 Zm 1 xs 0 Pg
/// Rn Zt`. ig1=10 unscaled / 11 scaled. Rn=x1, Zm=z2, Zt=z0.
fn enc_scatter_s(msz: u32, xs: u32, scaled: bool) -> u32 {
    let ig1: u32 = if scaled { 0b11 } else { 0b10 };
    (0b1110010 << 25)
        | (msz << 23)
        | (ig1 << 21)
        | (RM << 16)
        | (1 << 15)
        | (xs << 14)
        | (RN << 5)
        | RD
}

/// SVE LD1 (scalar+scalar): `1010010 dtype Rm 010 Pg Rn Zt`. Rn=x1, Rm=x2.
fn enc_sve_ld1_ss(dtype: u32) -> u32 {
    (0b1010010 << 25) | (dtype << 21) | (RM << 16) | (0b010 << 13) | (RN << 5) | RD
}
/// SVE ST1 (scalar+scalar): `1110010 msz size Rm 010 Pg Rn Zt`. Rn=x1, Rm=x2.
fn enc_sve_st1_ss(msz: u32, size: u32) -> u32 {
    (0b1110010 << 25) | (msz << 23) | (size << 21) | (RM << 16) | (0b010 << 13) | (RN << 5) | RD
}

/// SVE contiguous LD1 (scalar+imm): `1010010 dtype 0 imm4 101 Pg Rn Zt`.
/// Pg=p0, Rn=x1 (base), Zt=z0.
fn enc_sve_ld1(dtype: u32, imm4: i32) -> u32 {
    (0b1010010 << 25)
        | (dtype << 21)
        | (((imm4 as u32) & 0xF) << 16)
        | (0b101 << 13)
        | (RN << 5)
        | RD
}

/// SVE contiguous ST1 (scalar+imm): `1110010 msz size 0 imm4 111 Pg Rn Zt`.
/// msz=memory width, size=element width (>= msz). Pg=p0, Rn=x1, Zt=z0.
fn enc_sve_st1(msz: u32, size: u32, imm4: i32) -> u32 {
    (0b1110010 << 25)
        | (msz << 23)
        | (size << 21)
        | (((imm4 as u32) & 0xF) << 16)
        | (0b111 << 13)
        | (RN << 5)
        | RD
}

/// SVE ADR: `00000100 mode 1 Zm 1010 msz Zn Zd`. Zd=z0, Zn=z1, Zm=z2.
fn enc_sve_adr(mode: u32, msz: u32) -> u32 {
    (0b00000100 << 24)
        | (mode << 22)
        | (1 << 21)
        | (RM << 16)
        | (0b1010 << 12)
        | (msz << 10)
        | (RN << 5)
        | RD
}

/// SVE MOVPRFX Zd, Zn (unpredicated): `00000100 001 00000 101111 Zn Zd`.
fn enc_movprfx_z() -> u32 {
    (0b00000100 << 24) | (0b001 << 21) | (0b101111 << 10) | (RN << 5) | RD
}

/// SVE MOVPRFX Zd.T, Pg, Zn.T (predicated): `00000100 size 0100 0 M 001 Pg Zn Zd`.
/// M: 1=merging, 0=zeroing. Pg=p0, Zn=z1, Zd=z0.
fn enc_movprfx_p(size: u32, m: u32) -> u32 {
    (0b00000100 << 24) | (size << 22) | (0b0100 << 18) | (m << 16) | (0b001 << 13) | (RN << 5) | RD
}

/// SVE LD1R (load and replicate): `1000010 dtypeh 1 imm6 1 dtypel Pg Rn Zt`.
/// Pg=p0, Rn=x1, Zt=z0.
fn enc_ld1r(dtypeh: u32, dtypel: u32, imm6: u32) -> u32 {
    (0b1000010 << 25)
        | (dtypeh << 23)
        | (1 << 22)
        | ((imm6 & 0x3F) << 16)
        | (1 << 15)
        | (dtypel << 13)
        | (RN << 5)
        | RD
}

/// SVE LDR/STR whole-register fill/spill. `1000010110`(LDR)/`1110010110`(STR)
/// imm9h `010`(Z)/`000`(P) imm9l Rn Zt/Pt. Rn=x1, Zt=z0, Pt=p0.
fn enc_ldstr_z(store: bool, imm9: i32) -> u32 {
    let top: u32 = if store { 0b1110010110 } else { 0b1000010110 };
    let h = ((imm9 >> 3) & 0x3F) as u32;
    let l = (imm9 & 0x7) as u32;
    (top << 22) | (h << 16) | (0b010 << 13) | (l << 10) | (RN << 5) | RD
}
fn enc_ldstr_p(store: bool, imm9: i32) -> u32 {
    let top: u32 = if store { 0b1110010110 } else { 0b1000010110 };
    let h = ((imm9 >> 3) & 0x3F) as u32;
    let l = (imm9 & 0x7) as u32;
    (top << 22) | (h << 16) | (0b000 << 13) | (l << 10) | (RN << 5)
}

/// SVE FP<->int convert: `01100101 opc ig1 opc2 int_U 101 Pg Zn Zd`. ig1: 011=
/// FCVTZS/U (FP->int), 010=SCVTF/UCVTF (int->FP). int_U: 0=signed, 1=unsigned.
fn enc_sve_cvt(opc: u32, ig1: u32, opc2: u32, u: u32) -> u32 {
    (0b01100101 << 24)
        | (opc << 22)
        | (ig1 << 19)
        | (opc2 << 17)
        | (u << 16)
        | (0b101 << 13)
        | (RN << 5)
        | RD
}

/// A finite FP bit pattern of width `sz` bytes (no Inf/NaN). For 4/8-byte
/// sources the exponent is kept inside the fp16 normal range so the random
/// mantissa exercises narrowing rounding without overflow/subnormal noise.
fn finite_fp_bits(rng: &mut Rng, sz: usize) -> u64 {
    match sz {
        2 => {
            let sign = (rng.next() & 1) as u64;
            let exp = (rng.next() % 18 + 6) as u64;
            let mant = rng.next() & 0x3FF;
            (sign << 15) | (exp << 10) | mant
        }
        4 => {
            let sign = (rng.next() & 1) as u32;
            let exp = (rng.next() % 28 + 113) as u32; // 2^-14 .. 2^13
            let mant = (rng.next() as u32) & 0x7F_FFFF;
            ((sign << 31) | (exp << 23) | mant) as u64
        }
        _ => {
            let sign = rng.next() & 1;
            let exp = rng.next() % 28 + 1009; // 2^-14 .. 2^13
            let mant = rng.next() & 0xF_FFFF_FFFF_FFFF;
            (sign << 63) | (exp << 52) | mant
        }
    }
}

/// SVE TBX (table lookup, keep destination): `00000101 size 1 Zm 001011 Zn Zd`.
/// Table=Zn(RN), indices=Zm(RM), dest=Zd(RD, also the out-of-range source).
fn enc_sve_tbx(sz: u32) -> u32 {
    (0b00000101 << 24) | (sz << 22) | (1 << 21) | (RM << 16) | (0b001011 << 10) | (RN << 5) | RD
}

/// SVE DUP (indexed): `00000101 imm2 1 tsz 001000 Zn Zd`. esize_log selects the
/// element size (0=B..4=Q); the index is packed above the tsz size-marker bit.
fn enc_dup_idx(esize_log: u32, index: u32) -> u32 {
    let imm = (index << (esize_log + 1)) | (1 << esize_log); // 7-bit imm2:tsz
    let imm2 = (imm >> 5) & 0x3;
    let tsz = imm & 0x1F;
    (0b00000101 << 24) | (imm2 << 22) | (1 << 21) | (tsz << 16) | (0b001000 << 10) | (RN << 5) | RD
}

/// SVE COMPACT: `00000101 1 sz 100001 100 Pg Zn Zd`. sz: 0=S, 1=D. Zn=z1, Pg=p0.
fn enc_sve_compact(sz: u32) -> u32 {
    (0b00000101 << 24) | (1 << 23) | (sz << 22) | (0b100001 << 16) | (0b100 << 13) | (RN << 5) | RD
}

/// SVE SPLICE (destructive): `00000101 size 101100 100 Pg Zm Zdn`. Zm=z1, Pg=p0,
/// Zdn=z0 (both source-1 and destination).
fn enc_sve_splice(sz: u32) -> u32 {
    (0b00000101 << 24) | (sz << 22) | (0b101100 << 16) | (0b100 << 13) | (RN << 5) | RD
}

/// SVE PFIRST Pdn.B, Pg, Pdn.B: `00100101 01011000 1100000 Pg Pdn`.
fn enc_pfirst(pg: u32, pdn: u32) -> u32 {
    (0x25 << 24) | (0b01011000 << 16) | (0b1100000 << 9) | ((pg & 0xF) << 5) | (pdn & 0xF)
}

/// SVE PNEXT Pdn.T, Pg, Pdn.T: `00100101 size 011001 1100010 Pg Pdn`.
fn enc_pnext(sz: u32, pg: u32, pdn: u32) -> u32 {
    (0x25 << 24)
        | (sz << 22)
        | (0b011001 << 16)
        | (0b1100010 << 9)
        | ((pg & 0xF) << 5)
        | (pdn & 0xF)
}

/// SVE BRKA/BRKB: `00100101 B S 010000 01 Pg 0 Pn M Pd`. B: 0=BRKA, 1=BRKB.
fn enc_brka(b: u32, s: u32, m: u32, pg: u32, pn: u32, pd: u32) -> u32 {
    (0x25 << 24)
        | (b << 23)
        | (s << 22)
        | (0b010000 << 16)
        | (0b01 << 14)
        | ((pg & 0xF) << 10)
        | ((pn & 0xF) << 5)
        | ((m & 1) << 4)
        | (pd & 0xF)
}

/// SVE BRKN: `00100101 0 S 011000 01 Pg 0 Pn 0 Pdm`.
fn enc_brkn(s: u32, pg: u32, pn: u32, pdm: u32) -> u32 {
    (0x25 << 24)
        | (s << 22)
        | (0b011000 << 16)
        | (0b01 << 14)
        | ((pg & 0xF) << 10)
        | ((pn & 0xF) << 5)
        | (pdm & 0xF)
}

/// SVE BRKPA/BRKPB: `00100101 0 S 00 Pm 11 Pg 0 Pn B Pd`. B: 0=BRKPA, 1=BRKPB.
fn enc_brkp(b: u32, s: u32, pm: u32, pg: u32, pn: u32, pd: u32) -> u32 {
    (0x25 << 24)
        | (s << 22)
        | ((pm & 0xF) << 16)
        | (0b11 << 14)
        | ((pg & 0xF) << 10)
        | ((pn & 0xF) << 5)
        | ((b & 1) << 4)
        | (pd & 0xF)
}

#[test]
fn diff_sve_index() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for sz in 0..4u32 {
        cases.push((format!("index_ii sz{sz}"), enc_index_ii(sz, 1, 0)));
        cases.push((format!("index_iin sz{sz}"), enc_index_ii(sz, 0x1F, 5))); // negative step
        cases.push((format!("index_ri sz{sz}"), enc_index_ri(sz, 3)));
        cases.push((format!("index_ir sz{sz}"), enc_index_ir(sz, 2)));
        cases.push((format!("index_rr sz{sz}"), enc_index_rr(sz)));
    }
    run_family("sve_index", cases, 12, 0x1_0020);
}

/// SVE REV Zd.T, Zn.T: `00000101 sz 1 11000 001110 Zn Zd`.
fn enc_sve_rev(sz: u32) -> u32 {
    (0b00000101 << 24)
        | (sz << 22)
        | (1 << 21)
        | (0b11000 << 16)
        | (0b001110 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_sve_rev() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for sz in 0..4u32 {
        cases.push((format!("rev sz{sz}"), enc_sve_rev(sz)));
    }
    run_family("sve_rev", cases, 16, 0x1_0022);
}

/// PTRUE/PTRUES Pd.T, pattern: `00100101 sz 01100 S 111000 pattern 0 Pd`. Pd=p0.
fn enc_ptrue(sz: u32, pat: u32, s: u32) -> u32 {
    (0x25 << 24) | (sz << 22) | ((0b011000 | s) << 16) | (0b111000 << 10) | ((pat & 0x1F) << 5)
}
/// PFALSE p0.
const PFALSE: u32 = 0x2518_E400;
/// WHILE{LT,LE,LO,LS} Pd.T, {Wn,Xn}, {Wm,Xm}. Rn=x1, Rm=x2, Pd=p0.
fn enc_while(sz: u32, sf: u32, unsigned: bool, le: bool) -> u32 {
    let b1110 = if unsigned { 0b11 } else { 0b01 };
    (0x25 << 24)
        | (sz << 22)
        | (1 << 21)
        | (RM << 16)
        | (sf << 12)
        | (b1110 << 10)
        | (RN << 5)
        | ((le as u32) << 4)
}

#[test]
fn diff_sve_pred_gen() {
    let mut rng = Rng::new(0x1_0023);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    // PTRUE/PTRUES + PFALSE: deterministic, no register inputs.
    for sz in 0..4u32 {
        for pat in [0u32, 1, 2, 3, 4, 5, 7, 8, 0b11101, 0b11110, 0b11111] {
            for s in 0..2u32 {
                batch.push((
                    format!("ptrue sz{sz} p{pat} s{s}"),
                    enc_ptrue(sz, pat, s),
                    ArmState::zeroed(),
                ));
            }
        }
    }
    batch.push(("pfalse".into(), PFALSE, ArmState::zeroed()));
    // WHILE: small random index/limit in x1/x2 to exercise partial predicates.
    for sz in 0..4u32 {
        for sf in 0..2u32 {
            for u in 0..2u32 {
                for le in 0..2u32 {
                    let insn = enc_while(sz, sf, u == 1, le == 1);
                    for _ in 0..12 {
                        let mut st = gen_input(&mut rng);
                        st.x[1] = rng.next() % 24;
                        st.x[2] = rng.next() % 24;
                        batch.push((format!("while sz{sz} sf{sf} u{u} le{le}"), insn, st));
                    }
                }
            }
        }
    }
    run_batch("sve_pred_gen", batch);
}

/// SVE predicated integer ALU (destructive): `00000100 sz group opc Pg Zm Zdn`.
/// Zdn=z0, Zm=z1, Pg=p0.
fn enc_sve_palu(sz: u32, group: u32, opc: u32) -> u32 {
    (0x04 << 24) | (sz << 22) | (group << 19) | (opc << 16) | (RN << 5) | RD
}

/// SVE SEL Zd.T, Pg, Zn, Zm: `00000101 sz 1 Zm 11 Pg Zn Zd`. Zd=z0, Zn=z1,
/// Zm=z2, Pg=p0.
fn enc_sve_sel(sz: u32) -> u32 {
    (0x05 << 24) | (sz << 22) | (1 << 21) | (RM << 16) | (0b11 << 14) | (RN << 5) | RD
}

#[test]
fn diff_sve_sel() {
    let mut rng = Rng::new(0x1_0025);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..4u32 {
        let insn = enc_sve_sel(sz);
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            st.set_vreg(1, rng.next(), rng.next());
            st.set_vreg(2, rng.next(), rng.next());
            st.set_preg(0, rng.next() as u16);
            batch.push((format!("sel sz{sz}"), insn, st));
        }
    }
    run_batch("sve_sel", batch);
}

/// SVE CMP<cc>_P.P.ZZ: `00100100 sz 0 Zm cmp_hi Pg Zn cmp_lo Pd`. Zn=z1, Zm=z2,
/// Pg=p1, Pd=p0.
fn enc_sve_cmp(sz: u32, cmp_hi: u32, cmp_lo: u32) -> u32 {
    (0x24 << 24) | (sz << 22) | (RM << 16) | (cmp_hi << 13) | (1 << 10) | (RN << 5) | (cmp_lo << 4)
}

#[test]
fn diff_sve_cmp() {
    let ops: &[(u32, u32, &str)] = &[
        (0b000, 0, "hs"),
        (0b000, 1, "hi"),
        (0b100, 0, "ge"),
        (0b100, 1, "gt"),
        (0b101, 0, "eq"),
        (0b101, 1, "ne"),
    ];
    let mut rng = Rng::new(0x1_0026);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..4u32 {
        for &(hi, lo, name) in ops {
            let insn = enc_sve_cmp(sz, hi, lo);
            for _ in 0..14 {
                let mut st = ArmState::zeroed();
                // Narrow values so equalities actually occur for EQ/NE/GE/GT.
                let narrow = |r: &mut Rng| -> u64 {
                    let mut v = 0u64;
                    for b in 0..8 {
                        v |= ((r.next() % 6) as u64) << (b * 8);
                    }
                    v
                };
                st.set_vreg(1, narrow(&mut rng), narrow(&mut rng));
                st.set_vreg(2, narrow(&mut rng), narrow(&mut rng));
                st.set_preg(1, rng.next() as u16); // governing predicate Pg=p1
                batch.push((format!("cmp{name} sz{sz}"), insn, st));
            }
        }
    }
    run_batch("sve_cmp", batch);
}

/// SVE predicated shift by vector: `00000100 sz 010 opc 100 Pg Zm Zdn`. Zdn=z0,
/// Zm=z1, Pg=p0.
fn enc_sve_shift(sz: u32, opc: u32) -> u32 {
    (0x04 << 24) | (sz << 22) | (0b010 << 19) | (opc << 16) | (0b100 << 13) | (RN << 5) | RD
}

#[test]
fn diff_sve_shift_pred() {
    let ops = [(0b000u32, "asr"), (0b001, "lsr"), (0b011, "lsl")];
    let mut rng = Rng::new(0x1_002A);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..4u32 {
        for (opc, name) in ops {
            let insn = enc_sve_shift(sz, opc);
            for _ in 0..14 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next()); // value
                st.set_vreg(1, rng.next(), rng.next()); // shift amount (incl out-of-range)
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} sz{sz}"), insn, st));
            }
        }
    }
    run_batch("sve_shift_pred", batch);
}

#[test]
fn diff_sve_pcount() {
    let mut rng = Rng::new(0x1_002F);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..4u32 {
        // CNTP Rd(x0), Pg=p1, Pn=p2.
        let cntp =
            (0x25 << 24) | (sz << 22) | (0b100000 << 16) | (0b10 << 14) | (1 << 10) | (2 << 5);
        // INCP/DECP scalar (Rdn=x0, Pg=p1) and vector (Zdn=z0, Pg=p1).
        let incp_r =
            (0x25 << 24) | (sz << 22) | (0b101100 << 16) | (0b1000 << 12) | (1 << 11) | (1 << 5);
        let decp_r =
            (0x25 << 24) | (sz << 22) | (0b101101 << 16) | (0b1000 << 12) | (1 << 11) | (1 << 5);
        let incp_z = (0x25 << 24) | (sz << 22) | (0b101100 << 16) | (0b1000 << 12) | (1 << 5);
        let decp_z = (0x25 << 24) | (sz << 22) | (0b101101 << 16) | (0b1000 << 12) | (1 << 5);
        // LASTA/LASTB/CLASTA/CLASTB -> x0, Pg=p0, Zn=z1.
        let lasta = (0x05 << 24) | (sz << 22) | (0b100000 << 16) | (0b101 << 13) | (RN << 5);
        let lastb = (0x05 << 24) | (sz << 22) | (0b100001 << 16) | (0b101 << 13) | (RN << 5);
        let clasta = (0x05 << 24) | (sz << 22) | (0b110000 << 16) | (0b101 << 13) | (RN << 5);
        let clastb = (0x05 << 24) | (sz << 22) | (0b110001 << 16) | (0b101 << 13) | (RN << 5);
        for _ in 0..10 {
            let mut st = ArmState::zeroed();
            st.x[0] = rng.next();
            st.set_vreg(0, rng.next(), rng.next());
            st.set_vreg(1, rng.next(), rng.next());
            st.set_preg(0, rng.next() as u16);
            st.set_preg(1, rng.next() as u16);
            st.set_preg(2, rng.next() as u16);
            for (name, insn) in [
                ("cntp", cntp),
                ("incpr", incp_r),
                ("decpr", decp_r),
                ("incpz", incp_z),
                ("decpz", decp_z),
                ("lasta", lasta),
                ("lastb", lastb),
                ("clasta", clasta),
                ("clastb", clastb),
            ] {
                batch.push((format!("{name} sz{sz}"), insn, st));
            }
        }
    }
    run_batch("sve_pcount", batch);
}

#[test]
fn diff_sve_sincdecp() {
    // SQINCP/UQINCP/SQDECP/UQDECP: saturating INCP/DECP. GPR forms (32/64-bit,
    // signed/unsigned) and vector forms (per-element saturating). Seeds Xdn/Zdn
    // near the signed/unsigned limits to exercise the saturation boundaries.
    let mut rng = Rng::new(0x1_0030);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let gpr_vals: [u64; 12] = [
        0,
        1,
        u64::MAX,
        0x7FFF_FFFF,
        0x8000_0000,
        0xFFFF_FFFF,
        0x7FFF_FFFF_FFFF_FFFF,
        0x8000_0000_0000_0000,
        0xFFFF_FFFF_FFFF_FFF0,
        0x0000_0000_0000_0005,
        0xFFFF_FFFF_FFFF_FFFB,
        0x7FFF_FFFF_FFFF_FFFA,
    ];
    let vec_pats: [u64; 8] = [
        0x0000_0000_0000_0000,
        0x7FFF_7FFF_7FFF_7FFF,
        0x8000_8000_8000_8000,
        0xFFFF_FFFF_FFFF_FFFF,
        0x7FFF_FFFF_7FFF_FFFF,
        0x8000_0000_8000_0000,
        0x7FFF_FFFF_FFFF_FFFF,
        0x8000_0000_0000_0000,
    ];
    let preds = [0xFFFFu16, 0x5555, 0x0101, 0x0001, 0x0000];
    for esz in 0..4u32 {
        for d in 0..2u32 {
            for u in 0..2u32 {
                for sf64 in 0..2u32 {
                    let insn = enc_sve_sincdecp_r(esz, d, u, sf64, 1, 0);
                    let nm = format!("sincdecp_r e{esz} d{d} u{u} s{sf64}");
                    for &xv in gpr_vals.iter() {
                        for &pp in preds.iter() {
                            let mut st = ArmState::zeroed();
                            st.x[0] = xv;
                            st.set_preg(1, pp);
                            batch.push((nm.clone(), insn, st));
                        }
                    }
                    for _ in 0..4 {
                        let mut st = ArmState::zeroed();
                        st.x[0] = rng.next();
                        st.set_preg(1, rng.next() as u16);
                        batch.push((format!("{nm} rnd"), insn, st));
                    }
                }
                if esz >= 1 {
                    let insn = enc_sve_sincdecp_z(esz, d, u, 1, 0);
                    let nm = format!("sincdecp_z e{esz} d{d} u{u}");
                    for &vp in vec_pats.iter() {
                        for &pp in preds.iter() {
                            let mut st = ArmState::zeroed();
                            st.set_vreg(0, vp, vp);
                            st.set_preg(1, pp);
                            batch.push((nm.clone(), insn, st));
                        }
                    }
                    for _ in 0..4 {
                        let mut st = ArmState::zeroed();
                        st.set_vreg(0, rng.next(), rng.next());
                        st.set_preg(1, rng.next() as u16);
                        batch.push((format!("{nm} rnd"), insn, st));
                    }
                }
            }
        }
    }
    run_batch("sve_sincdecp", batch);
}

#[test]
fn diff_scalar_fp_cmp() {
    // Scalar FP compares (FCMP/FCMPE incl. compare-with-zero, with the NaN ->
    // unordered NZCV), FCCMP/FCCMPE, FCSEL, half-precision arithmetic/compare/
    // FSQRT/FMADD, and more FP<->int conversions (round modes, x/w widths).
    let enc: &[u32] = &[
        0x1e212000, 0x1e612000, 0x1e212010, 0x1e202008, 0x1e602018, 0x1e210405, 0x1e21141a,
        0x1e22cc20, 0x1e62bc20, 0x1ee22820, 0x1ee20820, 0x1ee21820, 0x1ee24820, 0x1ee1c020,
        0x1ee12000, 0x1fc20c20, 0x1e780020, 0x9e390020, 0x1e280020, 0x9e710020, 0x1e240020,
        0x9e630020, 0x1e63c020, 0x1ee2c020,
    ];
    let sp: [u64; 12] = [
        0x7F800000_7F800000,
        0xFF800000_FF800000,
        0x7FC00000_7FC00000,
        0x7F800001_FF800001,
        0x00000001_80000001,
        0x80000000_00000000,
        0x3F800000_BF800000,
        0x7F7FFFFF_FF7FFFFF,
        0x7FF0000000000000,
        0x7C007E007C007E00,
        0x7FF8000000000000,
        0x4000_4000_4000_4000,
    ];
    let mut rng = Rng::new(0x1_003D);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (k, &insn) in enc.iter().enumerate() {
        for (i, &v) in sp.iter().enumerate() {
            let mut st = ArmState::zeroed();
            st.set_vreg(0, v, 0);
            st.set_vreg(1, sp[(i + 3) % 12], 0);
            st.set_vreg(2, sp[(i + 5) % 12], 0);
            st.set_vreg(3, sp[(i + 7) % 12], 0);
            st.x[1] = v;
            st.pstate = rng.next() & 0xF000_0000;
            batch.push((format!("c{k}"), insn, st));
        }
        for _ in 0..8 {
            let mut st = ArmState::zeroed();
            for z in 0..4usize {
                st.set_vreg(z, rng.next(), rng.next());
            }
            st.x[1] = rng.next();
            st.pstate = rng.next() & 0xF000_0000;
            batch.push((format!("c{k} rnd"), insn, st));
        }
    }
    run_batch("scalar_fp_cmp", batch);
}

#[test]
fn diff_scalar_fp() {
    // Scalar FP data-processing (2-source FMUL/FDIV/FADD/FSUB/FMAX/FMIN/FMAXNM/
    // FMINNM/FNMUL, 3-source FMADD/FMSUB/FNMADD/FNMSUB, 1-source FSQRT/FABS/
    // FNEG/FRINT, FCVT precision change, and FP<->integer FCVTxS/xU/SCVTF/UCVTF)
    // with NaN/inf/denormal/-0 operands: the ARM default-NaN/quiet-propagation,
    // FNMUL/FMSUB/FNMADD sign flips, FCVT NaN payload conversion, and the
    // round-mode FP-to-int conversions.
    let enc: &[u32] = &[
        0x1e222820, 0x1e220820, 0x1e621820, 0x1e224820, 0x1e225820, 0x1e226820, 0x1e227820,
        0x1e228820, 0x1f020c20, 0x1f028c20, 0x1f220c20, 0x1f628c20, 0x1e21c020, 0x1e60c020,
        0x1e214020, 0x1e244020, 0x1e24c020, 0x1e65c020, 0x1e22c020, 0x1e624020, 0x1e23c020,
        0x1ee24020, 0x1e380020, 0x1e220020, 0x1e210400, 0x1e221c20, 0x1e628820,
    ];
    let sp: [u64; 12] = [
        0x7F800000_7F800000,
        0xFF800000_FF800000,
        0x7FC00000_7FC00000,
        0x7F800001_FF800001,
        0x00000001_80000001,
        0x80000000_00000000,
        0x3F800000_BF800000,
        0x7F7FFFFF_FF7FFFFF,
        0x7FF0000000000000,
        0xFFF0000000000000,
        0x7FF8000000000000,
        0x7E007C007DF80200,
    ];
    let mut rng = Rng::new(0x1_003C);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (k, &insn) in enc.iter().enumerate() {
        for (i, &v) in sp.iter().enumerate() {
            let mut st = ArmState::zeroed();
            st.set_vreg(1, v, 0);
            st.set_vreg(2, sp[(i + 5) % 12], 0);
            st.set_vreg(3, sp[(i + 7) % 12], 0);
            st.x[1] = v;
            st.pstate = rng.next() & 0xF000_0000;
            batch.push((format!("s{k}"), insn, st));
        }
        for _ in 0..8 {
            let mut st = ArmState::zeroed();
            for z in 1..4usize {
                st.set_vreg(z, rng.next(), rng.next());
            }
            st.x[1] = rng.next();
            st.pstate = rng.next() & 0xF000_0000;
            batch.push((format!("s{k} rnd"), insn, st));
        }
    }
    run_batch("scalar_fp", batch);
}

#[test]
fn diff_neon_fp_specials() {
    // NEON FP three-same / estimate / step ops with NaN/inf/denormal/-0
    // operands (single, double and half precision): the ARM default-NaN and
    // FPProcessNaNs rules, the fused FRECPS/FRSQRTS with FPNeg-first NaN sign,
    // FMLS op1-negation, FRECPE's denormal-output / tiny-input-overflow, and
    // FRECPX's scalar-only form.
    let enc: &[u32] = &[
        0x4e22d420, 0x6e22dc20, 0x6e62fc20, 0x4e22cc20, 0x4ee2cc20, 0x4e22dc20, 0x6ea2d420,
        0x4e22fc20, 0x4ee2fc20, 0x4e22c420, 0x4ea2c420, 0x4e22f420, 0x4e421420, 0x4e423c20,
        0x4e421c20, 0x4e420c20, 0x4ea1d820, 0x6ea1d820, 0x6ea1f820, 0x5ea1f820,
    ];
    let sp: [u64; 12] = [
        0x7F800000_7F800000,
        0xFF800000_FF800000,
        0x7FC00000_7FC00000,
        0x7F800001_FF800001,
        0x00000001_80000001,
        0x80000000_00000000,
        0x3F800000_BF800000,
        0x7F7FFFFF_FF7FFFFF,
        0x7FF0000000000000,
        0xFFF0000000000000,
        0x7FF8000000000000,
        0x7E007C007DF80200,
    ];
    let mut rng = Rng::new(0x1_003B);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (k, &insn) in enc.iter().enumerate() {
        for (i, &v) in sp.iter().enumerate() {
            let mut st = ArmState::zeroed();
            st.set_vreg(0, v, sp[(i + 3) % 12]);
            st.set_vreg(1, v, sp[(i + 5) % 12]);
            st.set_vreg(2, sp[(i + 7) % 12], v);
            batch.push((format!("n{k}"), insn, st));
        }
        for _ in 0..10 {
            let mut st = ArmState::zeroed();
            for z in 0..3usize {
                st.set_vreg(z, rng.next(), rng.next());
            }
            batch.push((format!("n{k} rnd"), insn, st));
        }
    }
    run_batch("neon_fp_specials", batch);
}

#[test]
fn diff_sve_fp16_fma_specials() {
    // f16 predicated FMLA/FMLS and indexed FMLA with NaN/inf/denormal operands:
    // FPProcessNaNs over the fused form must use (addend, op1, op2) order.
    let enc: [u32; 3] = [0x65620420, 0x65622420, 0x646a0020];
    let sp: [u64; 7] = [
        0x7E007E00_7E007E00,
        0x7C00FC00_7C00FC00,
        0x7DF87DF8_7DF87DF8,
        0x00010001_00010001,
        0x3C00BC00_3C00BC00,
        0x7E01FE01_7E01FE01,
        0x0200820002008200,
    ];
    let mut rng = Rng::new(0x1_003A);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (k, &insn) in enc.iter().enumerate() {
        for (i, &v) in sp.iter().enumerate() {
            let mut st = ArmState::zeroed();
            st.set_vreg(0, v, sp[(i + 2) % 7]);
            st.set_vreg(1, v, sp[(i + 4) % 7]);
            st.set_vreg(2, sp[(i + 1) % 7], v);
            st.set_preg(1, rng.next() as u16);
            batch.push((format!("h{k}"), insn, st));
        }
        for _ in 0..12 {
            let mut st = ArmState::zeroed();
            for z in 0..3usize {
                st.set_vreg(z, rng.next(), rng.next());
            }
            st.set_preg(1, rng.next() as u16);
            batch.push((format!("h{k} rnd"), insn, st));
        }
    }
    run_batch("sve_fp16_fma_specials", batch);
}

#[test]
fn diff_sve2_cdot_indexed() {
    // SVE2 CDOT by indexed element (complex integer dot product): the Zm
    // complex element is taken from a fixed index in the segment.
    let enc: [u32; 6] = [
        0x44a24020, 0x44ba4420, 0x44aa4820, 0x44b24c20, 0x44e24420, 0x44f24c20,
    ];
    let mut rng = Rng::new(0x1_0039);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (k, &insn) in enc.iter().enumerate() {
        for _ in 0..20 {
            let mut st = ArmState::zeroed();
            for z in 0..3usize {
                st.set_vreg(z, rng.next(), rng.next());
            }
            batch.push((format!("cdoti{k}"), insn, st));
        }
    }
    run_batch("sve2_cdot_indexed", batch);
}

#[test]
fn diff_sve_fp_specials() {
    // SVE predicated FP arithmetic and FP step ops with NaN/inf/denormal/-0
    // operands: exercises ARM's default-NaN-for-invalid-op rule and quiet-NaN
    // propagation (FPCR.DN=0), which native x86 f32/f64 arithmetic gets wrong
    // (x86 indefinite NaN 0xFFC00000 vs ARM default 0x7FC00000), plus the
    // FRECPS/FRSQRTS FPNeg-first NaN sign and the FMULX inf*0 -> +/-2.0 sign.
    let enc: [u32; 23] = [
        0x65808420, 0x65818420, 0x65828420, 0x658d8420, 0x65cc8420, 0x65838420, 0x65848420,
        0x65858420, 0x65c68420, 0x65878420, 0x65888420, 0x658a8420, 0x65ca8420, 0x65a20420,
        0x65e22420, 0x65a24420, 0x64b22020, 0x64aa0020, 0x65821820, 0x65c21c20, 0x65421820,
        0x65c08420, 0x65408420,
    ];
    let specials: [u64; 12] = [
        0x7F800000_7F800000,
        0xFF800000_FF800000,
        0x7FC00000_7FC00000,
        0x7F800001_FF800001,
        0x00000001_80000001,
        0x80000000_00000000,
        0x3F800000_BF800000,
        0x7F7FFFFF_FF7FFFFF,
        0x7FF0000000000000,
        0xFFF0000000000000,
        0x7FF8000000000000,
        0x0008000000000000,
    ];
    let mut rng = Rng::new(0x1_0038);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (k, &insn) in enc.iter().enumerate() {
        for (i, &sp) in specials.iter().enumerate() {
            let mut st = ArmState::zeroed();
            st.set_vreg(0, sp, specials[(i + 3) % specials.len()]);
            st.set_vreg(1, sp, specials[(i + 5) % specials.len()]);
            st.set_vreg(2, specials[(i + 7) % specials.len()], sp);
            st.set_preg(1, rng.next() as u16);
            batch.push((format!("fp{k}"), insn, st));
        }
        for _ in 0..14 {
            let mut st = ArmState::zeroed();
            for z in 0..3usize {
                st.set_vreg(z, rng.next(), rng.next());
            }
            st.set_preg(1, rng.next() as u16);
            batch.push((format!("fp{k} rnd"), insn, st));
        }
    }
    run_batch("sve_fp_specials", batch);
}

/// SVE predicate-on-predicate logical: `00100101 bit23 S 00 Pm 01 Pg o2 Pn o3
/// Pd`. The S-bit forms set NZCV; SEL is (bit23,o2,o3)=(0,1,1). Pg=p1, Pn=p2,
/// Pm=p3, Pd=p0.
fn enc_sve_pred_logical(bit23: u32, o2: u32, o3: u32, s: u32) -> u32 {
    (0x25 << 24)
        | (bit23 << 23)
        | (s << 22)
        | (3 << 16)
        | (0b01 << 14)
        | (1 << 10)
        | (o2 << 9)
        | (2 << 5)
        | (o3 << 4)
}

#[test]
fn diff_sve_pred_logical() {
    // Predicate logical AND/BIC/EOR/ORR/ORN/NOR/NAND (+ flag-setting S forms,
    // i.e. ANDS..MOVS) and SEL, plus RDFFRS. Random predicate inputs exercise
    // the PredTest first/last/none NZCV cases.
    let ops: [(u32, u32, u32, &str); 8] = [
        (0, 0, 0, "and"),
        (0, 0, 1, "bic"),
        (0, 1, 0, "eor"),
        (0, 1, 1, "sel"),
        (1, 0, 0, "orr"),
        (1, 0, 1, "orn"),
        (1, 1, 0, "nor"),
        (1, 1, 1, "nand"),
    ];
    let mut rng = Rng::new(0x1_0037);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(b23, o2, o3, name) in ops.iter() {
        // SEL has no flag-setting form; the rest cover S=0 and S=1.
        let s_vals: &[u32] = if name == "sel" { &[0] } else { &[0, 1] };
        for &s in s_vals {
            let insn = enc_sve_pred_logical(b23, o2, o3, s);
            for _ in 0..10 {
                let mut st = ArmState::zeroed();
                for p in 0..4usize {
                    st.set_preg(p, rng.next() as u16);
                }
                batch.push((format!("p_{name} s{s}"), insn, st));
            }
        }
    }
    // RDFFRS p0, p1/z (reads FFR, which the harness initialises consistently).
    for _ in 0..8 {
        let mut st = ArmState::zeroed();
        for p in 0..2usize {
            st.set_preg(p, rng.next() as u16);
        }
        batch.push(("rdffrs".to_string(), 0x2558f020, st));
    }
    run_batch("sve_pred_logical", batch);
}

#[test]
fn diff_sve_elem_count() {
    // SVE element-count / inc-dec-by-count / stack-allocation family:
    // RDVL/ADDVL/ADDPL, CNTB/H/W/D, INCB/DECB.. (GPR + vector), and the
    // saturating SQINCB/UQINCB.. forms, plus PTEST. Encodings span varied
    // patterns (all/vl*/pow2/mul3/mul4), MUL multipliers and signed imm6.
    let enc: [u32; 57] = [
        0x04bf5400, 0x04bf57e0, 0x04bf5020, 0x04bf53e0, 0x04215420, 0x04215000, 0x042150a0,
        0x043f579f, 0x04615400, 0x046150e0, 0x047f503f, 0x0420e3e0, 0x0421e100, 0x0460e000,
        0x04a2e3c0, 0x04efe3e0, 0x0420e020, 0x0460e120, 0x04a0e3a0, 0x0430e3e0, 0x0434e3e0,
        0x0470e3e0, 0x04b0e000, 0x04f1e3e0, 0x0430e7e0, 0x0470e7c0, 0x04b0e7e0, 0x04f3e7e0,
        0x0420f3e0, 0x0422f3e0, 0x0420f7e0, 0x0460f100, 0x0461f7e0, 0x04b0f3e0, 0x04b3f7e0,
        0x04fff3e0, 0x0420fbe0, 0x0420ffe0, 0x04f0fbe0, 0x0470c3e0, 0x0472c3e0, 0x04b0c000,
        0x04f1c3e0, 0x0470c7e0, 0x04b0c7c0, 0x04f6c7e0, 0x0460c3e0, 0x0461c3e0, 0x0460c7e0,
        0x04a0c000, 0x04a0c7e0, 0x04efc3e0, 0x0460cbe0, 0x04a0cfe0, 0x04e3cbe0, 0x2550c020,
        0x2550cc40,
    ];
    let gpr_vals: [u64; 8] = [
        0,
        1,
        0x7FFF_FFFF,
        0x8000_0000,
        0xFFFF_FFFF,
        0x7FFF_FFFF_FFFF_FFFF,
        0x8000_0000_0000_0000,
        0xFFFF_FFFF_FFFF_FFFF,
    ];
    let vec_pats: [u64; 4] = [
        0x0000_0000_0000_0000,
        0x7FFF_7FFF_7FFF_7FFF,
        0x8000_8000_8000_8000,
        0xFFFF_FFFF_FFFF_FFFF,
    ];
    let mut rng = Rng::new(0x1_0036);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (k, &insn) in enc.iter().enumerate() {
        for i in 0..8 {
            let mut st = ArmState::zeroed();
            st.x[0] = gpr_vals[i];
            st.x[1] = rng.next();
            st.sp = 0x7FFF_0000 ^ (i as u64 * 0x40);
            let vp = vec_pats[i % vec_pats.len()];
            st.set_vreg(0, vp, vp.rotate_left(16));
            for p in 0..4usize {
                st.set_preg(p, rng.next() as u16);
            }
            batch.push((format!("ec{k}"), insn, st));
        }
    }
    run_batch("sve_elem_count", batch);
}

/// WHILE gt-family: `00100101 esz 1 rm 000 sf u 0 rn eq rd`. eq=0 => GE/HS
/// (inclusive), eq=1 => GT/HI (strict); u selects signed(0)/unsigned(1). Rn=x0,
/// Rm=x1, Pd=p0.
fn enc_sve_while_gt(esz: u32, sf: u32, u: u32, eqbit: u32) -> u32 {
    (0x25 << 24) | (esz << 22) | (1 << 21) | (1 << 16) | (sf << 12) | (u << 11) | (eqbit << 4)
}

/// SVE predicated shift-by-immediate (destructive): `00000100 tszh op6 100 pg
/// tszl imm3 rd`. op6 is bits[21:16]. Rdn=z0, Pg=p1.
fn enc_sve_shift_imm_pred(op6: u32, esize_bits: u32, amt: u32) -> u32 {
    let tsize = match esize_bits {
        8 => 0b0001u32,
        16 => 0b0010,
        32 => 0b0100,
        _ => 0b1000,
    };
    let is_shl = matches!(op6, 0b000_011 | 0b000_110 | 0b000_111 | 0b001_111);
    let tszimm = if is_shl {
        esize_bits + amt
    } else {
        2 * esize_bits - amt
    };
    let imm3 = tszimm & 7;
    let tszh = tsize >> 2;
    let tszl = tsize & 3;
    (0b00000100u32 << 24)
        | (tszh << 22)
        | (op6 << 16)
        | (0b100 << 13)
        | (1 << 10)
        | (tszl << 8)
        | (imm3 << 5)
}

#[test]
fn diff_sve2_shift_imm_sat() {
    // ASRD / SQSHL / UQSHL / SRSHR / URSHR / SQSHLU predicated shift-by-
    // immediate, seeded with the signed/unsigned extremes to exercise the
    // rounding and saturation paths. (ASR/LSR/LSL are covered by
    // diff_sve_shift_imm.) Shift amounts stay in [1, bits-1] to avoid the
    // architecturally-ambiguous shift==width corner.
    let ops: [(u32, &str); 6] = [
        (0b000_100, "asrd"),
        (0b000_110, "sqshl"),
        (0b000_111, "uqshl"),
        (0b001_100, "srshr"),
        (0b001_101, "urshr"),
        (0b001_111, "sqshlu"),
    ];
    let pats: [u64; 6] = [
        0x0000_0000_0000_0000,
        0x8080_8080_8080_8080,
        0x7F7F_7F7F_7F7F_7F7F,
        0xFFFF_FFFF_FFFF_FFFF,
        0x8000_0000_8000_0000,
        0x1234_5678_9ABC_DEF0,
    ];
    let mut rng = Rng::new(0x1_0034);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(op6, name) in ops.iter() {
        for &bits in &[8u32, 16, 32, 64] {
            let amts = [1u32, 2, (bits / 2).max(1), bits - 1];
            for &amt in amts.iter() {
                let insn = enc_sve_shift_imm_pred(op6, bits, amt);
                let nm = format!("{name} b{bits} a{amt}");
                for &p in pats.iter() {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(0, p, p);
                    st.set_preg(1, 0xFFFF);
                    batch.push((nm.clone(), insn, st));
                }
                for _ in 0..3 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(0, rng.next(), rng.next());
                    st.set_preg(1, rng.next() as u16);
                    batch.push((format!("{nm} rnd"), insn, st));
                }
            }
        }
    }
    run_batch("sve2_shift_imm_sat", batch);
}

/// SVE2 SABA/UABA: `01000101 esz 0 Zm 11111 u Zn Zda`. u=1 unsigned. Zda=z0,
/// Zn=z1, Zm=z2.
fn enc_sve2_saba(esz: u32, u: u32) -> u32 {
    (0x45 << 24) | (esz << 22) | (2 << 16) | (0b11111 << 11) | (u << 10) | (1 << 5)
}

#[test]
fn diff_sve2_saba() {
    // SABA/UABA: Zda += |Zn - Zm| per element. Seeds the abs-difference extremes
    // (INT_MIN vs INT_MAX, 0 vs UMAX) that exercise the widened subtraction.
    let pats: [u64; 6] = [
        0x0000_0000_0000_0000,
        0x8080_8080_8080_8080,
        0x7F7F_7F7F_7F7F_7F7F,
        0xFFFF_FFFF_FFFF_FFFF,
        0x8000_8000_8000_8000,
        0x7FFF_7FFF_7FFF_7FFF,
    ];
    let mut rng = Rng::new(0x1_0033);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for esz in 0..4u32 {
        for u in 0..2u32 {
            let insn = enc_sve2_saba(esz, u);
            let nm = format!("saba e{esz} u{u}");
            for &p1 in pats.iter() {
                for &p2 in pats.iter() {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(0, rng.next(), rng.next()); // accumulator
                    st.set_vreg(1, p1, p1);
                    st.set_vreg(2, p2, p2);
                    batch.push((nm.clone(), insn, st));
                }
            }
            for _ in 0..8 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next());
                st.set_vreg(1, rng.next(), rng.next());
                st.set_vreg(2, rng.next(), rng.next());
                batch.push((format!("{nm} rnd"), insn, st));
            }
        }
    }
    run_batch("sve2_saba", batch);
}

#[test]
fn diff_bf16_dot() {
    // BFDOT / BFMMLA (NEON + SVE), FPCR.EBF==0. Exercises the round-to-odd-inf
    // overflow (-> +/-inf), the inf*0 / inf-inf default-NaN, and the denormal
    // flush-to-zero paths that distinguish the bf16 dot rounding from naive
    // f32 arithmetic, plus fully random lanes.
    let enc: &[(&str, u32)] = &[
        ("neon_bfdot_4s", 0x6e42fc20),
        ("neon_bfdot_2s", 0x2e42fc20),
        ("neon_bfdot_i", 0x4f62f020),
        ("neon_bfmmla", 0x6e42ec20),
        ("sve_bfdot", 0x64628020),
        ("sve_bfdot_i", 0x64624020),
        ("sve_bfmmla", 0x6462e420),
    ];
    let bf_specials: [u16; 10] = [
        0x0000, // +0
        0x8000, // -0
        0x0001, // smallest +denormal
        0x007F, // largest +denormal
        0x3F80, // 1.0
        0x7F7F, // max normal
        0xFF7F, // -max normal
        0x7F80, // +inf
        0xFF80, // -inf
        0x7FC0, // qNaN
    ];
    // Tile each 32-bit f32 slot with a pair of special bf16 lanes.
    let mut pats: Vec<u64> = Vec::new();
    for &x in bf_specials.iter() {
        for &y in bf_specials.iter() {
            let slot = (x as u32) | ((y as u32) << 16);
            pats.push((slot as u64) | ((slot as u64) << 32));
        }
    }
    let mut rng = Rng::new(0x1_0035);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(name, insn) in enc {
        for (i, &w1) in pats.iter().enumerate() {
            let w2 = pats[(i * 13) % pats.len()];
            let mut st = ArmState::zeroed();
            st.set_vreg(0, rng.next(), rng.next()); // accumulator
            st.set_vreg(1, w1, w1.rotate_left(16));
            st.set_vreg(2, w2, w2.rotate_left(48));
            batch.push((name.to_string(), insn, st));
        }
        for _ in 0..60 {
            let mut st = ArmState::zeroed();
            st.set_vreg(0, rng.next(), rng.next());
            st.set_vreg(1, rng.next(), rng.next());
            st.set_vreg(2, rng.next(), rng.next());
            batch.push((format!("{name} rnd"), insn, st));
        }
    }
    run_batch("bf16_dot", batch);
}

#[test]
fn diff_sve_while_gt() {
    // WHILEGT/WHILEGE/WHILEHI/WHILEHS: the gt-family WHILE producing a top-
    // anchored contiguous predicate. Stresses the signed/unsigned boundaries
    // and the equality (one-more-iteration / all-true) edge cases.
    let vals: [u64; 13] = [
        0,
        1,
        5,
        16,
        17,
        40,
        0x7FFF_FFFF,
        0x8000_0000,
        0xFFFF_FFFF,
        0x7FFF_FFFF_FFFF_FFFF,
        0x8000_0000_0000_0000,
        0xFFFF_FFFF_FFFF_FFFF,
        0xFFFF_FFFF_FFFF_FFFB,
    ];
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for esz in 0..4u32 {
        for sf in 0..2u32 {
            for u in 0..2u32 {
                for eqbit in 0..2u32 {
                    let insn = enc_sve_while_gt(esz, sf, u, eqbit);
                    let nm = format!("while_gt e{esz} sf{sf} u{u} eq{eqbit}");
                    for &a in vals.iter() {
                        for &b in vals.iter() {
                            let mut st = ArmState::zeroed();
                            st.x[0] = a;
                            st.x[1] = b;
                            batch.push((nm.clone(), insn, st));
                        }
                    }
                }
            }
        }
    }
    run_batch("sve_while_gt", batch);
}

#[test]
fn diff_sve_shift_imm() {
    let ops = [(0b000u32, "asr"), (0b001, "lsr"), (0b011, "lsl")];
    let mut rng = Rng::new(0x1_002E);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &esize in &[1usize, 2, 4, 8] {
        let ebits = esize * 8;
        for &(opc, name) in &ops {
            let amounts: Vec<usize> = if opc == 0b011 {
                vec![0, 1, ebits / 2, ebits - 1]
            } else {
                vec![1, 2, ebits / 2, ebits]
            };
            for &amount in &amounts {
                let tszimm = if opc == 0b011 {
                    ebits + amount
                } else {
                    2 * ebits - amount
                };
                let tsize = (tszimm >> 3) as u32;
                let imm3 = (tszimm & 7) as u32;
                let (tszh, tszl) = (tsize >> 2, tsize & 3);
                let insn = (0x04 << 24)
                    | (tszh << 22)
                    | (opc << 16)
                    | (0b100 << 13)
                    | (tszl << 8)
                    | (imm3 << 5)
                    | RD;
                for _ in 0..8 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(0, rng.next(), rng.next());
                    st.set_preg(0, rng.next() as u16);
                    batch.push((format!("{name} e{ebits} a{amount}"), insn, st));
                }
            }
        }
    }
    run_batch("sve_shift_imm", batch);
}

/// SVE CPY immediate: `00000101 sz 01 Pg M sh imm8 Zd`. Pg=p0, Zd=z0.
fn enc_cpy_imm(sz: u32, m: u32, sh: u32, imm8: i32) -> u32 {
    (0x05 << 24)
        | (sz << 22)
        | (0b01 << 20)
        | (m << 14)
        | (sh << 13)
        | (((imm8 as u32) & 0xFF) << 5)
        | RD
}

#[test]
fn diff_sve_cpy() {
    let mut rng = Rng::new(0x1_002D);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..4u32 {
        // CPY immediate, merging and zeroing.
        for m in 0..2u32 {
            for &(sh, imm) in &[(0u32, 0x5i32), (0, -7), (1, 0x12), (1, -1)] {
                let insn = enc_cpy_imm(sz, m, sh, imm);
                for _ in 0..6 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(0, rng.next(), rng.next());
                    st.set_preg(0, rng.next() as u16);
                    batch.push((format!("cpyi sz{sz} m{m}"), insn, st));
                }
            }
        }
        // CPY scalar GPR (Rn=x1) and SIMD scalar (Vn=v1), both merging.
        let cpyr = (0x05 << 24) | (sz << 22) | (0b101000 << 16) | (0b101 << 13) | (RN << 5) | RD;
        let cpyv = (0x05 << 24) | (sz << 22) | (0b100000 << 16) | (0b100 << 13) | (RN << 5) | RD;
        // DUP immediate broadcast.
        let dup = (0x25 << 24) | (sz << 22) | (0b111000 << 16) | (0b11 << 14) | (0x33 << 5) | RD;
        let dup2 = (0x25 << 24)
            | (sz << 22)
            | (0b111000 << 16)
            | (0b11 << 14)
            | (1 << 13)
            | (0xA1 << 5)
            | RD;
        for _ in 0..10 {
            let mut st = ArmState::zeroed();
            st.set_vreg(0, rng.next(), rng.next());
            st.x[1] = rng.next();
            st.set_preg(0, rng.next() as u16);
            batch.push((format!("cpyr sz{sz}"), cpyr, st));
            let mut st2 = ArmState::zeroed();
            st2.set_vreg(0, rng.next(), rng.next());
            st2.set_vreg(1, rng.next(), rng.next());
            st2.set_preg(0, rng.next() as u16);
            batch.push((format!("cpyv sz{sz}"), cpyv, st2));
            batch.push((format!("dup sz{sz}"), dup, ArmState::zeroed()));
            batch.push((format!("dup2 sz{sz}"), dup2, ArmState::zeroed()));
        }
    }
    run_batch("sve_cpy", batch);
}

#[test]
fn diff_sve_fp_unary() {
    // (top_byte, opc6, name). FABS/FNEG use 0x04; FSQRT/FRINT*/FRECPX use 0x65.
    let ops: &[(u32, u32, &str)] = &[
        (0x04, 0b011100, "fabs"),
        (0x04, 0b011101, "fneg"),
        (0x65, 0b001101, "fsqrt"),
        (0x65, 0b000000, "frintn"),
        (0x65, 0b000001, "frintp"),
        (0x65, 0b000010, "frintm"),
        (0x65, 0b000011, "frintz"),
        (0x65, 0b000100, "frinta"),
        (0x65, 0b000110, "frintx"),
        (0x65, 0b000111, "frinti"),
        (0x65, 0b001100, "frecpx"),
    ];
    let mut rng = Rng::new(0x1_002B);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(sz, esize) in &[(1u32, 16u32), (2, 32), (3, 64)] {
        let lanes = 128 / esize as usize;
        for &(top, opc6, name) in ops {
            let insn = (top << 24) | (sz << 22) | (opc6 << 16) | (0b101 << 13) | (RN << 5) | RD;
            for _ in 0..12 {
                let mut st = ArmState::zeroed();
                let (l0, h0) = fill_finite_fp(&mut rng, esize, lanes); // prior Zd (merge)
                let (l1, h1) = fill_finite_fp(&mut rng, esize, lanes); // source
                st.set_vreg(0, l0, h0);
                st.set_vreg(1, l1, h1);
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} sz{sz}"), insn, st));
            }
        }
    }
    run_batch("sve_fp_unary", batch);
}

#[test]
fn diff_sve_fp_reduce() {
    let ops: &[(u32, &str)] = &[
        (0b000000, "faddv"),
        (0b000110, "fmaxv"),
        (0b000111, "fminv"),
        (0b000100, "fmaxnmv"),
        (0b000101, "fminnmv"),
        (0b011000, "fadda"),
    ];
    let mut rng = Rng::new(0x1_002C);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(sz, esize) in &[(1u32, 16u32), (2, 32), (3, 64)] {
        let lanes = 128 / esize as usize;
        for &(opc6, name) in ops {
            let insn = (0x65 << 24) | (sz << 22) | (opc6 << 16) | (0b001 << 13) | (RN << 5) | RD;
            for _ in 0..14 {
                let mut st = ArmState::zeroed();
                let (l1, h1) = fill_finite_fp(&mut rng, esize, lanes);
                st.set_vreg(1, l1, h1); // Zn (or Zm for FADDA)
                // FADDA seeds the accumulator from Vdn[0] = v0.
                let (l0, h0) = fill_finite_fp(&mut rng, esize, lanes);
                st.set_vreg(0, l0, h0);
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} sz{sz}"), insn, st));
            }
        }
    }
    run_batch("sve_fp_reduce", batch);
}

/// SVE integer reduction: `00000100 sz opc6 001 Pg Zn Vd`. Zn=z1, Vd=v0, Pg=p0.
fn enc_sve_reduce(sz: u32, opc6: u32) -> u32 {
    (0x04 << 24) | (sz << 22) | (opc6 << 16) | (0b001 << 13) | (RN << 5) | RD
}

#[test]
fn diff_sve_reduce() {
    // (opc6, name, max_sz)
    let ops: &[(u32, &str, u32)] = &[
        (0b000000, "saddv", 2),
        (0b000001, "uaddv", 3),
        (0b001000, "smaxv", 3),
        (0b001001, "umaxv", 3),
        (0b001010, "sminv", 3),
        (0b001011, "uminv", 3),
        (0b011000, "orv", 3),
        (0b011001, "eorv", 3),
        (0b011010, "andv", 3),
    ];
    let mut rng = Rng::new(0x1_0029);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(opc6, name, max_sz) in ops {
        for sz in 0..=max_sz {
            let insn = enc_sve_reduce(sz, opc6);
            for _ in 0..14 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next());
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} sz{sz}"), insn, st));
            }
        }
    }
    run_batch("sve_reduce", batch);
}

/// SVE predicate-logical: `00100101 S000 Pm 01 Pg b9 Pn b4 Pd`. Pg=p1, Pm=p2,
/// Pn=p3, Pd=p0.
fn enc_sve_plog(s: u32, b9: u32, b4: u32) -> u32 {
    (0x25 << 24)
        | (s << 23)
        | (2 << 16)
        | (0b01 << 14)
        | (1 << 10)
        | (b9 << 9)
        | (3 << 5)
        | (b4 << 4)
}

#[test]
fn diff_sve_plog() {
    let ops: &[(u32, u32, u32, &str)] = &[
        (0, 0, 0, "and"),
        (0, 0, 1, "bic"),
        (0, 1, 0, "eor"),
        (1, 0, 0, "orr"),
        (1, 0, 1, "orn"),
        (1, 1, 0, "nor"),
        (1, 1, 1, "nand"),
    ];
    let mut rng = Rng::new(0x1_0028);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(s, b9, b4, name) in ops {
        let insn = enc_sve_plog(s, b9, b4);
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            st.set_preg(1, rng.next() as u16);
            st.set_preg(2, rng.next() as u16);
            st.set_preg(3, rng.next() as u16);
            batch.push((format!("p{name}"), insn, st));
        }
    }
    run_batch("sve_plog", batch);
}

/// SVE predicated FP binary arith: `01100101 sz opc5 100 Pg Zm Zdn`. Zdn=z0,
/// Zm=z1, Pg=p0.
fn enc_sve_fpp(sz: u32, opc5: u32) -> u32 {
    (0x65 << 24) | (sz << 22) | (opc5 << 16) | (0b100 << 13) | (RN << 5) | RD
}

#[test]
fn diff_sve_fp_pred() {
    let ops: &[(u32, &str)] = &[
        (0b00000, "fadd"),
        (0b00001, "fsub"),
        (0b00010, "fmul"),
        (0b00011, "fsubr"),
        (0b00100, "fmaxnm"),
        (0b00101, "fminnm"),
        (0b00110, "fmax"),
        (0b00111, "fmin"),
        (0b01000, "fabd"),
        (0b01100, "fdivr"),
        (0b01101, "fdiv"),
    ];
    let mut rng = Rng::new(0x1_0027);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(sz, esize) in &[(1u32, 16u32), (2, 32), (3, 64)] {
        let lanes = 128 / esize as usize;
        for &(opc5, name) in ops {
            let insn = enc_sve_fpp(sz, opc5);
            for _ in 0..14 {
                let mut st = ArmState::zeroed();
                let (l0, h0) = fill_finite_fp(&mut rng, esize, lanes);
                let (l1, h1) = fill_finite_fp(&mut rng, esize, lanes);
                st.set_vreg(0, l0, h0);
                st.set_vreg(1, l1, h1);
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} sz{sz}"), insn, st));
            }
        }
    }
    run_batch("sve_fp_pred", batch);
}

#[test]
fn diff_sve_palu() {
    // (group, opc, name, min_sz)
    let ops: &[(u32, u32, &str, u32)] = &[
        (0, 0, "add", 0),
        (0, 1, "sub", 0),
        (0, 3, "subr", 0),
        (1, 0, "smax", 0),
        (1, 1, "umax", 0),
        (1, 2, "smin", 0),
        (1, 3, "umin", 0),
        (1, 4, "sabd", 0),
        (1, 5, "uabd", 0),
        (2, 0, "mul", 0),
        (2, 2, "smulh", 0),
        (2, 3, "umulh", 0),
        (2, 4, "sdiv", 2),
        (2, 5, "udiv", 2),
        (2, 6, "sdivr", 2),
        (2, 7, "udivr", 2),
        (3, 0, "orr", 0),
        (3, 1, "eor", 0),
        (3, 2, "and", 0),
        (3, 3, "bic", 0),
    ];
    let mut rng = Rng::new(0x1_0024);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..4u32 {
        for &(group, opc, name, min_sz) in ops {
            if sz < min_sz {
                continue;
            }
            let insn = enc_sve_palu(sz, group, opc);
            for _ in 0..14 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next()); // Zdn (dest + first source)
                st.set_vreg(1, rng.next(), rng.next()); // Zm
                st.set_preg(0, rng.next() as u16); // governing predicate
                batch.push((format!("p{name} sz{sz}"), insn, st));
            }
        }
    }
    run_batch("sve_palu", batch);
}

#[test]
fn diff_sve_perm() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for sz in 0..4u32 {
        for (opc, name) in [
            (0b000u32, "zip1"),
            (0b001, "zip2"),
            (0b010, "uzp1"),
            (0b011, "uzp2"),
            (0b100, "trn1"),
            (0b101, "trn2"),
        ] {
            cases.push((format!("{name} sz{sz}"), enc_sve_perm(sz, opc)));
        }
    }
    run_family("sve_perm", cases, 12, 0x1_0021);
}

#[test]
fn diff_sve_ext() {
    // EXT extracts a VL-wide window from the byte concatenation Zm:Zdn at a
    // byte offset. imm8>=16 (at VL=128) wraps the offset to 0 (Zdn unchanged).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for imm8 in [0u32, 1, 2, 3, 7, 8, 11, 15, 16, 20, 31] {
        cases.push((format!("sve_ext #{imm8}"), enc_sve_ext(imm8)));
    }
    run_family("sve_ext", cases, 16, 0x2_5001);
}

#[test]
fn diff_sve_tbl() {
    // TBL gathers Zn[Zm[e]] per element; out-of-range indices yield 0. Random
    // index lanes exercise both in-range gathers and the zero-fill path.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for sz in 0..4u32 {
        cases.push((format!("sve_tbl sz{sz}"), enc_sve_tbl(sz)));
    }
    run_family("sve_tbl", cases, 24, 0x2_6001);
}

#[test]
fn diff_sve_tbx() {
    // TBX is TBL with destination preservation for out-of-range indices.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for sz in 0..4u32 {
        cases.push((format!("sve_tbx sz{sz}"), enc_sve_tbx(sz)));
    }
    run_family("sve_tbx", cases, 24, 0x2_F001);
}

#[test]
fn diff_sve_fcvt() {
    // Predicated FP precision conversions. Source values are packed at the
    // (larger) container boundary; merging keeps the prior Zd in inactive lanes.
    let convs = [
        (0b10u32, 0b01u32, 2usize, 4usize, "h2s"),
        (0b11, 0b01, 2, 8, "h2d"),
        (0b10, 0b00, 4, 2, "s2h"),
        (0b11, 0b11, 4, 8, "s2d"),
        (0b11, 0b00, 8, 2, "d2h"),
        (0b11, 0b10, 8, 4, "d2s"),
    ];
    let mut rng = Rng::new(0x3_0001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (opc, opc2, src_sz, dst_sz, name) in convs {
        let insn = enc_sve_fcvt(opc, opc2);
        let cont = src_sz.max(dst_sz);
        let elements = 16 / cont;
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            let mut zn: u128 = 0;
            for e in 0..elements {
                let bits = finite_fp_bits(&mut rng, src_sz);
                zn |= (bits as u128) << (e * cont * 8);
            }
            st.set_vreg(1, zn as u64, (zn >> 64) as u64);
            st.set_vreg(0, rng.next(), rng.next()); // prior Zd (merge target)
            st.set_preg(0, rng.next() as u16);
            batch.push((format!("fcvt_{name}"), insn, st));
        }
    }
    run_batch("sve_fcvt", batch);
}

#[test]
fn diff_sve2_fcvtx() {
    // SVE2 FCVTNT/FCVTLT/FCVTXNT operate on the top/odd half of each container.
    // Narrow (NT/XNT) reads the wide source from the whole container and writes
    // the converted narrow result into the top half (bottom half preserved).
    // Long (LT) reads the narrow source from the top half and writes the wide
    // result into the whole container. Predication is at container granularity.
    let convs = [
        (0b10u32, 0b00u32, 4usize, 2usize, true, "fcvtnt_s2h"),
        (0b11, 0b10, 8, 4, true, "fcvtnt_d2s"),
        (0b00, 0b10, 8, 4, true, "fcvtxnt_d2s"),
        (0b10, 0b01, 2, 4, false, "fcvtlt_h2s"),
        (0b11, 0b11, 4, 8, false, "fcvtlt_s2d"),
    ];
    let mut rng = Rng::new(0x6_1001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (opc, opc2, src_sz, dst_sz, narrow, name) in convs {
        let insn = enc_sve2_fcvtx(opc, opc2);
        let cont = src_sz.max(dst_sz);
        let containers = 16 / cont;
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            let mut zn: u128 = 0;
            for c in 0..containers {
                let bits = finite_fp_bits(&mut rng, src_sz) as u128;
                // NT: source is the full container (low). LT: source is the top
                // half of the container (offset by src_sz bytes).
                let off_bytes = if narrow { c * cont } else { c * cont + src_sz };
                zn |= bits << (off_bytes * 8);
            }
            st.set_vreg(1, zn as u64, (zn >> 64) as u64);
            st.set_vreg(0, rng.next(), rng.next()); // prior Zd (preserved half)
            st.set_preg(0, rng.next() as u16);
            batch.push((name.to_string(), insn, st));
        }
    }
    run_batch("sve2_fcvtx", batch);
}

#[test]
fn diff_sve2_fmlal() {
    // FMLALB/T and FMLSLB/T widen f16 lanes to f32 and fused-multiply-accumulate
    // into the f32 destination. Finite f16 sources and a finite f32 accumulator.
    let mut rng = Rng::new(0x6_a001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let variants = [
        (0u32, 0u32, "fmlalb"),
        (0, 1, "fmlalt"),
        (1, 0, "fmlslb"),
        (1, 1, "fmlslt"),
    ];
    for (sub, top, name) in variants {
        let insn = enc_sve2_fmlal(sub, top);
        for _ in 0..24 {
            let mut zn = 0u128;
            let mut zm = 0u128;
            for h in 0..8 {
                zn |= (finite_fp_bits(&mut rng, 2) as u128) << (h * 16);
                zm |= (finite_fp_bits(&mut rng, 2) as u128) << (h * 16);
            }
            let mut zd = 0u128;
            for s in 0..4 {
                zd |= (finite_fp_bits(&mut rng, 4) as u128) << (s * 32);
            }
            let mut st = ArmState::zeroed();
            st.set_vreg(1, zn as u64, (zn >> 64) as u64);
            st.set_vreg(2, zm as u64, (zm >> 64) as u64);
            st.set_vreg(0, zd as u64, (zd >> 64) as u64);
            batch.push((name.to_string(), insn, st));
        }
    }
    run_batch("sve2_fmlal", batch);
}

#[test]
fn diff_sve_bfdot() {
    // BFDOT bf16 dot product, both the three-register and indexed forms.
    let mut rng = Rng::new(0x7_d001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let mk = |rng: &mut Rng| {
        let (mut zn, mut zm, mut za) = (0u128, 0u128, 0u128);
        for h in 0..8 {
            zn |= ((finite_fp_bits(rng, 4) >> 16) as u128) << (h * 16);
            zm |= ((finite_fp_bits(rng, 4) >> 16) as u128) << (h * 16);
        }
        for s in 0..4 {
            za |= (finite_fp_bits(rng, 4) as u128) << (s * 32);
        }
        let mut st = ArmState::zeroed();
        st.set_vreg(1, zn as u64, (zn >> 64) as u64);
        st.set_vreg(2, zm as u64, (zm >> 64) as u64);
        st.set_vreg(0, za as u64, (za >> 64) as u64);
        st
    };
    let insn = enc_sve_bfdot(RM);
    for _ in 0..30 {
        batch.push(("bfdot".to_string(), insn, mk(&mut rng)));
    }
    for index in 0..4u32 {
        let insn = enc_sve_bfdot_idx(index, RM);
        for _ in 0..8 {
            batch.push((format!("bfdot_idx i{index}"), insn, mk(&mut rng)));
        }
    }
    run_batch("sve_bfdot", batch);
}

#[test]
fn diff_sve_bfmlal() {
    // BFMLALB/T bf16 widening fused multiply-add, three-register and indexed.
    let mut rng = Rng::new(0x7_e001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let mk = |rng: &mut Rng| {
        let (mut zn, mut zm, mut za) = (0u128, 0u128, 0u128);
        for h in 0..8 {
            zn |= ((finite_fp_bits(rng, 4) >> 16) as u128) << (h * 16);
            zm |= ((finite_fp_bits(rng, 4) >> 16) as u128) << (h * 16);
        }
        for s in 0..4 {
            za |= (finite_fp_bits(rng, 4) as u128) << (s * 32);
        }
        let mut st = ArmState::zeroed();
        st.set_vreg(1, zn as u64, (zn >> 64) as u64);
        st.set_vreg(2, zm as u64, (zm >> 64) as u64);
        st.set_vreg(0, za as u64, (za >> 64) as u64);
        st
    };
    for top in 0..2u32 {
        let insn = enc_sve_bfmlal(top, RM);
        for _ in 0..16 {
            batch.push((format!("bfmlal t{top}"), insn, mk(&mut rng)));
        }
        for index in 0..8u32 {
            let insn = enc_sve_bfmlal_idx(top, index, RM);
            for _ in 0..3 {
                batch.push((format!("bfmlal_idx t{top} i{index}"), insn, mk(&mut rng)));
            }
        }
    }
    run_batch("sve_bfmlal", batch);
}

#[test]
fn diff_sve2_fmlal_indexed() {
    // FMLALB/T and FMLSLB/T by indexed Zm.h[index] (broadcast), all indices.
    let mut rng = Rng::new(0x7_c001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sub in 0..2u32 {
        for top in 0..2u32 {
            for index in 0..8u32 {
                let insn = enc_sve2_fmlal_idx(sub, top, index, RM);
                for _ in 0..3 {
                    let (mut zn, mut zm, mut za) = (0u128, 0u128, 0u128);
                    for h in 0..8 {
                        zn |= (finite_fp_bits(&mut rng, 2) as u128) << (h * 16);
                        zm |= (finite_fp_bits(&mut rng, 2) as u128) << (h * 16);
                    }
                    for s in 0..4 {
                        za |= (finite_fp_bits(&mut rng, 4) as u128) << (s * 32);
                    }
                    let mut st = ArmState::zeroed();
                    st.set_vreg(1, zn as u64, (zn >> 64) as u64);
                    st.set_vreg(2, zm as u64, (zm >> 64) as u64);
                    st.set_vreg(0, za as u64, (za >> 64) as u64);
                    batch.push((format!("fmlal_idx sub{sub} t{top} i{index}"), insn, st));
                }
            }
        }
    }
    run_batch("sve2_fmlal_indexed", batch);
}

#[test]
fn diff_sve_fscale() {
    // FSCALE multiplies each FP lane by 2^(signed Zm element). Use moderate
    // exponents (-40..40) so results stay in range and vary.
    let mut rng = Rng::new(0x8_5001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        let insn = enc_sve_fscale(size);
        let emask: u64 = if esz == 8 {
            u64::MAX
        } else {
            (1u64 << (esz * 8)) - 1
        };
        for _ in 0..20 {
            let (mut zdn, mut zm) = (0u128, 0u128);
            for l in 0..(16 / esz) {
                zdn |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                let n = (rng.next() % 81) as i64 - 40;
                zm |= (((n as u64) & emask) as u128) << (l * esz * 8);
            }
            let mut st = ArmState::zeroed();
            st.set_vreg(0, zdn as u64, (zdn >> 64) as u64);
            st.set_vreg(1, zm as u64, (zm >> 64) as u64);
            st.set_preg(0, rng.next() as u16);
            batch.push((format!("fscale s{size}"), insn, st));
        }
    }
    run_batch("sve_fscale", batch);
}

#[test]
fn diff_sve_fexpa() {
    // FEXPA table lookup; the source bits are arbitrary (low bits index, next
    // bits become the exponent).
    let mut rng = Rng::new(0x8_6001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 1..4u32 {
        let insn = enc_sve_fexpa(size);
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            st.set_vreg(1, rng.next(), rng.next()); // Zn
            st.set_vreg(0, rng.next(), rng.next()); // dest (overwritten)
            batch.push((format!("fexpa s{size}"), insn, st));
        }
    }
    run_batch("sve_fexpa", batch);
}

#[test]
fn diff_sve_fp_indexed() {
    // FMLA/FMLS/FMUL by indexed FP element, all sizes and indices, finite inputs.
    let mut rng = Rng::new(0x8_3001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (op, name) in [
        (0b000000u32, "fmla"),
        (0b000001, "fmls"),
        (0b001000, "fmul"),
    ] {
        for (size, esz, idxn) in [(1u32, 2usize, 8u32), (2, 4, 4), (3, 8, 2)] {
            for index in 0..idxn {
                let insn = enc_sve_fp_idx(op, size, index, RM);
                for _ in 0..3 {
                    let mut st = ArmState::zeroed();
                    for r in 0..3 {
                        let mut v = 0u128;
                        for l in 0..(16 / esz) {
                            v |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                        }
                        st.set_vreg(r, v as u64, (v >> 64) as u64);
                    }
                    batch.push((format!("{name}_idx s{size} i{index}"), insn, st));
                }
            }
        }
    }
    run_batch("sve_fp_indexed", batch);
}

#[test]
fn diff_sve_recps() {
    // FRECPS / FRSQRTS reciprocal steps, all sizes, finite inputs.
    let mut rng = Rng::new(0x8_4001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        for rsqrt in 0..2u32 {
            let insn = enc_sve_recps(size, rsqrt);
            for _ in 0..16 {
                let mut st = ArmState::zeroed();
                for r in 1..3 {
                    let mut v = 0u128;
                    for l in 0..(16 / esz) {
                        v |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                    }
                    st.set_vreg(r, v as u64, (v >> 64) as u64);
                }
                batch.push((format!("recps s{size} r{rsqrt}"), insn, st));
            }
        }
    }
    run_batch("sve_recps", batch);
}

#[test]
fn diff_sve_dot() {
    // SVE integer dot product: vector SDOT/UDOT (.s/.d), USDOT, and the indexed
    // SDOT/UDOT/USDOT/SUDOT forms.
    let mut rng = Rng::new(0x8_2001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let setup = |rng: &mut Rng| {
        let mut st = ArmState::zeroed();
        st.set_vreg(0, rng.next(), rng.next());
        st.set_vreg(1, rng.next(), rng.next());
        st.set_vreg(2, rng.next(), rng.next());
        st
    };
    for sz in 0..2u32 {
        for u in 0..2u32 {
            let insn = enc_sve_dot_vec(sz, u);
            for _ in 0..10 {
                batch.push((format!("dot_v sz{sz} u{u}"), insn, setup(&mut rng)));
            }
        }
    }
    let insn = enc_sve_usdot_vec();
    for _ in 0..10 {
        batch.push(("usdot_v".to_string(), insn, setup(&mut rng)));
    }
    let idxops = [
        (0u32, 0b000000u32, "sdot"),
        (0, 0b000001, "udot"),
        (0, 0b000110, "usdot"),
        (0, 0b000111, "sudot"),
        (1, 0b000000, "sdot_d"),
        (1, 0b000001, "udot_d"),
    ];
    for (sz, op, name) in idxops {
        let idxn = if sz == 0 { 4 } else { 2 };
        for index in 0..idxn {
            let insn = enc_sve_dot_idx(sz, op, index, RM);
            for _ in 0..4 {
                batch.push((format!("{name}_idx i{index}"), insn, setup(&mut rng)));
            }
        }
    }
    run_batch("sve_dot", batch);
}

#[test]
fn diff_sve_fp_fma() {
    // Predicated FP fused multiply-add, all 8 variants and sizes, finite inputs.
    let mut rng = Rng::new(0x8_0001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        for op3 in 0..8u32 {
            let insn = enc_sve_fp_fma(size, op3);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                for r in 0..3 {
                    let mut v = 0u128;
                    for l in 0..(16 / esz) {
                        v |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                    }
                    st.set_vreg(r, v as u64, (v >> 64) as u64);
                }
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("fma s{size} op{op3}"), insn, st));
            }
        }
    }
    run_batch("sve_fp_fma", batch);
}

#[test]
fn diff_sve_int_mla() {
    // Predicated integer MLA/MLS/MAD/MSB, all sizes, random operands.
    let mut rng = Rng::new(0x8_1001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 0..4u32 {
        for op3 in [0b010u32, 0b011, 0b110, 0b111] {
            let insn = enc_sve_int_mla(size, op3);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next());
                st.set_vreg(1, rng.next(), rng.next());
                st.set_vreg(2, rng.next(), rng.next());
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("mla s{size} op{op3:b}"), insn, st));
            }
        }
    }
    run_batch("sve_int_mla", batch);
}

#[test]
fn diff_sve2_pred_alu() {
    // SVE2 predicated shifts/halving/saturating add-sub and SQABS/SQNEG. Random
    // operands give full-range (large/negative) shift amounts and saturation
    // edges; the predicate is random (merging).
    let ops: [(u32, &str); 28] = [
        (0b000010, "srshl"),
        (0b000011, "urshl"),
        (0b000110, "srshlr"),
        (0b000111, "urshlr"),
        (0b001000, "sqshl"),
        (0b001001, "uqshl"),
        (0b001100, "sqshlr"),
        (0b001101, "uqshlr"),
        (0b001010, "sqrshl"),
        (0b001011, "uqrshl"),
        (0b001110, "sqrshlr"),
        (0b001111, "uqrshlr"),
        (0b010000, "shadd"),
        (0b010001, "uhadd"),
        (0b010010, "shsub"),
        (0b010011, "uhsub"),
        (0b010100, "srhadd"),
        (0b010101, "urhadd"),
        (0b010110, "shsubr"),
        (0b010111, "uhsubr"),
        (0b011000, "sqadd"),
        (0b011001, "uqadd"),
        (0b011010, "sqsub"),
        (0b011011, "uqsub"),
        (0b011100, "suqadd"),
        (0b011101, "usqadd"),
        (0b011110, "sqsubr"),
        (0b011111, "uqsubr"),
    ];
    let mut rng = Rng::new(0x7_f001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 0..4u32 {
        for &(opc6, name) in &ops {
            let insn = enc_sve2_pred_alu(size, opc6, 0b100);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next()); // Zdn
                st.set_vreg(1, rng.next(), rng.next()); // Zm
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} s{size}"), insn, st));
            }
        }
        for &(opc6, name) in &[(0b001000u32, "sqabs"), (0b001001, "sqneg")] {
            let insn = enc_sve2_pred_alu(size, opc6, 0b101);
            for _ in 0..8 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next()); // source
                st.set_vreg(0, rng.next(), rng.next()); // merge target
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} s{size}"), insn, st));
            }
        }
    }
    // URECPE/URSQRTE (S-only unsigned reciprocal estimates).
    for &(opc6, name) in &[(0b000000u32, "urecpe"), (0b000001, "ursqrte")] {
        let insn = enc_sve2_pred_alu(2, opc6, 0b101);
        for _ in 0..10 {
            let mut st = ArmState::zeroed();
            st.set_vreg(1, rng.next(), rng.next());
            st.set_vreg(0, rng.next(), rng.next());
            st.set_preg(0, rng.next() as u16);
            batch.push((format!("{name}"), insn, st));
        }
    }
    run_batch("sve2_pred_alu", batch);
}

#[test]
fn diff_sve_fp_cmp() {
    // FP compare (register) and compare-with-zero -> predicate, incl. NaN/inf
    // (random bit patterns), all sizes; checks predicate result AND NZCV.
    let mut rng = Rng::new(0x9_c001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let regconds = [
        (0b010u32, 0u32),
        (0b010, 1),
        (0b011, 0),
        (0b011, 1),
        (0b110, 0),
        (0b110, 1),
        (0b111, 1),
    ];
    let zeroconds = [
        (0b00u32, 0u32),
        (0b00, 1),
        (0b01, 0),
        (0b01, 1),
        (0b10, 0),
        (0b11, 0),
    ];
    for size in 1..4u32 {
        for (cc13, bit4) in regconds {
            let insn = enc_sve_fp_cmp(size, cc13, bit4);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next());
                st.set_vreg(2, rng.next(), rng.next());
                st.set_preg(1, rng.next() as u16);
                batch.push((format!("fcmp s{size} c{cc13}{bit4}"), insn, st));
            }
        }
        for (sub, bit4) in zeroconds {
            let insn = enc_sve_fp_cmp0(size, sub, bit4);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next());
                st.set_preg(1, rng.next() as u16);
                batch.push((format!("fcmp0 s{size} c{sub}{bit4}"), insn, st));
            }
        }
    }
    run_batch("sve_fp_cmp", batch);
}

#[test]
fn diff_sve_cmp_imm() {
    // Integer compare with signed/unsigned immediate -> predicate + NZCV.
    let mut rng = Rng::new(0x9_d001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let sconds = [
        (0b000u32, 0u32),
        (0b000, 1),
        (0b001, 0),
        (0b001, 1),
        (0b100, 0),
        (0b100, 1),
    ];
    for size in 0..4u32 {
        for (cc13, bit4) in sconds {
            for imm5 in [0u32, 5, 0x1F, 0x10, 1] {
                let insn = enc_sve_cmp_imm_s(size, cc13, bit4, imm5);
                for _ in 0..3 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(1, rng.next(), rng.next());
                    st.set_preg(1, rng.next() as u16);
                    batch.push((format!("cmps s{size} c{cc13}{bit4} i{imm5}"), insn, st));
                }
            }
        }
        for (lo, hi) in [(0u32, 0u32), (0, 1), (1, 0), (1, 1)] {
            for imm7 in [0u32, 9, 0x7F, 0x40] {
                let insn = enc_sve_cmp_imm_u(size, lo, hi, imm7);
                for _ in 0..3 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(1, rng.next(), rng.next());
                    st.set_preg(1, rng.next() as u16);
                    batch.push((format!("cmpu s{size} {lo}{hi} i{imm7}"), insn, st));
                }
            }
        }
    }
    run_batch("sve_cmp_imm", batch);
}

#[test]
fn diff_sve_bfcvt() {
    // BFCVT f32 -> bf16, predicated merging, finite inputs.
    let insn = enc_sve_bfcvt();
    let mut rng = Rng::new(0x9_e001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for _ in 0..40 {
        let mut zn = 0u128;
        for l in 0..4 {
            zn |= (finite_fp_bits(&mut rng, 4) as u128) << (l * 32);
        }
        let mut st = ArmState::zeroed();
        st.set_vreg(1, zn as u64, (zn >> 64) as u64);
        st.set_vreg(0, rng.next(), rng.next());
        st.set_preg(0, rng.next() as u16);
        batch.push(("bfcvt".to_string(), insn, st));
    }
    run_batch("sve_bfcvt", batch);
}

#[test]
fn diff_sve_pred_permute() {
    // Predicate ZIP1/ZIP2/UZP1/UZP2/TRN1/TRN2 and REV, all element sizes.
    let mut rng = Rng::new(0x9_b001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for esz in 0..4u32 {
        for opc in 0..6u32 {
            let insn = enc_sve_pred_permute(esz, opc);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                for p in 0..3 {
                    st.set_preg(p, rng.next() as u16);
                }
                batch.push((format!("permp esz{esz} op{opc}"), insn, st));
            }
        }
        let insn = enc_sve_rev_p(esz);
        for _ in 0..6 {
            let mut st = ArmState::zeroed();
            st.set_preg(1, rng.next() as u16);
            batch.push((format!("revp esz{esz}"), insn, st));
        }
    }
    run_batch("sve_pred_permute", batch);
}

#[test]
fn diff_sve_pred_unary() {
    // SVE predicated integer/FP unary across all ops and their valid sizes.
    let ops: &[(u32, &str, &[u32])] = &[
        (0b010000, "sxtb", &[1, 2, 3]),
        (0b010001, "uxtb", &[1, 2, 3]),
        (0b010010, "sxth", &[2, 3]),
        (0b010011, "uxth", &[2, 3]),
        (0b010100, "sxtw", &[3]),
        (0b010101, "uxtw", &[3]),
        (0b010110, "abs", &[0, 1, 2, 3]),
        (0b010111, "neg", &[0, 1, 2, 3]),
        (0b011000, "cls", &[0, 1, 2, 3]),
        (0b011001, "clz", &[0, 1, 2, 3]),
        (0b011010, "cnt", &[0, 1, 2, 3]),
        (0b011011, "cnot", &[0, 1, 2, 3]),
        (0b011100, "fabs", &[1, 2, 3]),
        (0b011101, "fneg", &[1, 2, 3]),
        (0b011110, "not", &[0, 1, 2, 3]),
    ];
    let mut rng = Rng::new(0x9_6001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &(opc6, name, sizes) in ops {
        for &size in sizes {
            let insn = enc_sve_pred_unary(size, opc6);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next());
                st.set_vreg(0, rng.next(), rng.next());
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} s{size}"), insn, st));
            }
        }
    }
    run_batch("sve_pred_unary", batch);
}

#[test]
fn diff_sve_unpk() {
    // SUNPKHI/LO, UUNPKHI/LO, all dest sizes.
    let mut rng = Rng::new(0x9_7001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 1..4u32 {
        for u in 0..2u32 {
            for h in 0..2u32 {
                let insn = enc_sve_unpk(size, u, h);
                for _ in 0..6 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(1, rng.next(), rng.next());
                    batch.push((format!("unpk s{size} u{u} h{h}"), insn, st));
                }
            }
        }
    }
    run_batch("sve_unpk", batch);
}

#[test]
fn diff_sve_dupm() {
    // DUPM logical-immediate broadcast across element sizes and patterns.
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let cases = [
        (0u32, 8u32, 39u32),
        (0, 0, 0b111100),
        (1, 0, 0),
        (1, 0, 62),
        (0, 4, 0b011111),
        (0, 1, 0b110000),
        (0, 0, 0b000000),
        (1, 16, 31),
    ];
    for (n, immr, imms) in cases {
        batch.push((
            format!("dupm n{n} r{immr} s{imms}"),
            enc_sve_dupm(n, immr, imms),
            ArmState::zeroed(),
        ));
    }
    run_batch("sve_dupm", batch);
}

#[test]
fn diff_sve_frecpe() {
    // FRECPE/FRSQRTE estimate, all sizes, finite inputs.
    let mut rng = Rng::new(0x9_8001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        for rsqrt in 0..2u32 {
            let insn = enc_sve_frecpe(size, rsqrt);
            for _ in 0..16 {
                let mut zn = 0u128;
                for l in 0..(16 / esz) {
                    zn |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                }
                let mut st = ArmState::zeroed();
                st.set_vreg(1, zn as u64, (zn >> 64) as u64);
                batch.push((format!("frecpe s{size} r{rsqrt}"), insn, st));
            }
        }
    }
    run_batch("sve_frecpe", batch);
}

#[test]
fn diff_sve_shift_pred_v() {
    // Predicated shift by vector incl. the reversed ASRR/LSRR/LSLR forms.
    let mut rng = Rng::new(0x9_9001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 0..4u32 {
        for opc in [0b000u32, 0b001, 0b011, 0b100, 0b101, 0b111] {
            let insn = enc_sve_shift_pred_v(size, opc);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next());
                st.set_vreg(1, rng.next(), rng.next());
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("shl s{size} op{opc:b}"), insn, st));
            }
        }
    }
    run_batch("sve_shift_pred_v", batch);
}

#[test]
fn diff_sve_rev_rbit() {
    // REVB/REVH/REVW/RBIT reverse byte/halfword/word/bit order within elements.
    let mut rng = Rng::new(0x9_1001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let cases: &[(u32, &[u32])] = &[
        (0b00, &[1, 2, 3]),
        (0b01, &[2, 3]),
        (0b10, &[3]),
        (0b11, &[0, 1, 2, 3]),
    ];
    for &(op, sizes) in cases {
        for &size in sizes {
            let insn = enc_sve_rev_rbit(size, op);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next());
                st.set_vreg(0, rng.next(), rng.next());
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("rev op{op:b} s{size}"), insn, st));
            }
        }
    }
    run_batch("sve_rev_rbit", batch);
}

#[test]
fn diff_sve_insr() {
    // INSR inserts a GPR or SIMD-scalar element at the bottom, shifting up.
    let mut rng = Rng::new(0x9_2001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 0..4u32 {
        for f in 0..2u32 {
            let insn = enc_sve_insr(size, f);
            for _ in 0..6 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next());
                st.set_vreg(1, rng.next(), rng.next());
                st.x[1] = rng.next();
                batch.push((format!("insr s{size} f{f}"), insn, st));
            }
        }
    }
    run_batch("sve_insr", batch);
}

#[test]
fn diff_sve_clast_dst() {
    // CLASTA/CLASTB to vector and SIMD-scalar destinations.
    let mut rng = Rng::new(0x9_3001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 0..4u32 {
        for scalar in 0..2u32 {
            for before in 0..2u32 {
                let insn = enc_sve_clast(size, scalar, before);
                for _ in 0..6 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(0, rng.next(), rng.next());
                    st.set_vreg(1, rng.next(), rng.next());
                    st.set_preg(0, rng.next() as u16);
                    batch.push((format!("clast s{size} sc{scalar} b{before}"), insn, st));
                }
            }
        }
    }
    run_batch("sve_clast_dst", batch);
}

#[test]
fn diff_sve_fcpy_fdup() {
    // FCPY (predicated FP immediate copy) and FDUP (FP immediate broadcast).
    let mut rng = Rng::new(0x9_4001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 1..4u32 {
        for imm8 in [0x78u32, 0x80, 0x00, 0xe0, 0x55, 0xaa, 0x3f] {
            let mut st = ArmState::zeroed();
            st.set_vreg(0, rng.next(), rng.next());
            st.set_preg(0, rng.next() as u16);
            batch.push((
                format!("fcpy s{size} i{imm8:x}"),
                enc_sve_fcpy(size, imm8),
                st,
            ));
            batch.push((
                format!("fdup s{size} i{imm8:x}"),
                enc_sve_fdup(size, imm8),
                ArmState::zeroed(),
            ));
        }
    }
    run_batch("sve_fcpy_fdup", batch);
}

#[test]
fn diff_sve_cterm() {
    // CTERMEQ/CTERMNE: compare two GP regs, set N/V leaving Z/C unchanged (so
    // the input NZCV matters). Mix equal and unequal operands, random flags.
    let mut rng = Rng::new(0x8_7001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sf in 0..2u32 {
        for ne in 0..2u32 {
            let insn = enc_sve_cterm(sf, ne);
            for _ in 0..12 {
                let mut st = ArmState::zeroed();
                let base = rng.next();
                st.x[1] = base;
                st.x[2] = if rng.next() & 1 == 0 {
                    base
                } else {
                    rng.next()
                };
                st.pstate = (rng.next() & 0xF) << 28; // random input NZCV
                batch.push((format!("cterm sf{sf} ne{ne}"), insn, st));
            }
        }
    }
    run_batch("sve_cterm", batch);
}

#[test]
fn diff_sve2_cmla_indexed() {
    // CMLA by indexed element, integer, .h and .s, all indices and rotations.
    let mut rng = Rng::new(0x7_a001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, idxn) in [(2u32, 4u32), (3, 2)] {
        for index in 0..idxn {
            for rot in 0..4u32 {
                let insn = enc_sve2_cmla_idx(size, index, RM, rot);
                for _ in 0..4 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(1, rng.next(), rng.next());
                    st.set_vreg(2, rng.next(), rng.next());
                    st.set_vreg(0, rng.next(), rng.next());
                    batch.push((format!("cmla_idx s{size} i{index} r{rot}"), insn, st));
                }
            }
        }
    }
    run_batch("sve2_cmla_indexed", batch);
}

#[test]
fn diff_sve_fcmla_indexed() {
    // FCMLA by indexed element, finite FP, .h and .s, all indices/rotations.
    let mut rng = Rng::new(0x7_b001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz, idxn) in [(2u32, 2usize, 4u32), (3, 4, 2)] {
        for index in 0..idxn {
            for rot in 0..4u32 {
                let insn = enc_sve_fcmla_idx(size, index, RM, rot);
                for _ in 0..4 {
                    let (mut za, mut zn, mut zm) = (0u128, 0u128, 0u128);
                    for l in 0..(16 / esz) {
                        za |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                        zn |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                        zm |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                    }
                    let mut st = ArmState::zeroed();
                    st.set_vreg(0, za as u64, (za >> 64) as u64);
                    st.set_vreg(1, zn as u64, (zn >> 64) as u64);
                    st.set_vreg(2, zm as u64, (zm >> 64) as u64);
                    batch.push((format!("fcmla_idx s{size} i{index} r{rot}"), insn, st));
                }
            }
        }
    }
    run_batch("sve_fcmla_indexed", batch);
}

#[test]
fn diff_sve2_cdot() {
    // CDOT complex integer dot product, .s and .d, all four rotations.
    let mut rng = Rng::new(0x7_9001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in [2u32, 3] {
        for rot in 0..4u32 {
            let insn = enc_sve2_cdot(size, rot);
            for _ in 0..16 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next());
                st.set_vreg(2, rng.next(), rng.next());
                st.set_vreg(0, rng.next(), rng.next());
                batch.push((format!("cdot s{size} r{rot}"), insn, st));
            }
        }
    }
    run_batch("sve2_cdot", batch);
}

#[test]
fn diff_sve_fcadd() {
    // FCADD predicated FP complex add, both rotations, all sizes.
    let mut rng = Rng::new(0x7_7001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        for rot in 0..2u32 {
            let insn = enc_sve_fcadd(size, rot);
            for _ in 0..16 {
                let (mut a, mut b) = (0u128, 0u128);
                for l in 0..(16 / esz) {
                    a |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                    b |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                }
                let mut st = ArmState::zeroed();
                st.set_vreg(0, a as u64, (a >> 64) as u64); // Zdn
                st.set_vreg(1, b as u64, (b >> 64) as u64); // Zm
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("fcadd s{size} r{rot}"), insn, st));
            }
        }
    }
    run_batch("sve_fcadd", batch);
}

#[test]
fn diff_sve_fcmla() {
    // FCMLA predicated FP complex multiply-add, all four rotations and sizes.
    let mut rng = Rng::new(0x7_8001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        for rot in 0..4u32 {
            let insn = enc_sve_fcmla(size, rot);
            for _ in 0..12 {
                let (mut za, mut zn, mut zm) = (0u128, 0u128, 0u128);
                for l in 0..(16 / esz) {
                    za |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                    zn |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                    zm |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                }
                let mut st = ArmState::zeroed();
                st.set_vreg(0, za as u64, (za >> 64) as u64); // Zda
                st.set_vreg(1, zn as u64, (zn >> 64) as u64); // Zn
                st.set_vreg(2, zm as u64, (zm >> 64) as u64); // Zm
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("fcmla s{size} r{rot}"), insn, st));
            }
        }
    }
    run_batch("sve_fcmla", batch);
}

#[test]
fn diff_sve_ftsmul() {
    // FTSMUL squares each lane and takes the sign from Zm bit0.
    let mut rng = Rng::new(0x7_4001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        let insn = enc_sve_ftsmul(size);
        for _ in 0..20 {
            let mut zn = 0u128;
            for l in 0..(16 / esz) {
                zn |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
            }
            let mut st = ArmState::zeroed();
            st.set_vreg(1, zn as u64, (zn >> 64) as u64);
            st.set_vreg(2, rng.next(), rng.next()); // Zm sign source
            batch.push((format!("ftsmul s{size}"), insn, st));
        }
    }
    run_batch("sve_ftsmul", batch);
}

#[test]
fn diff_sve_ftssel() {
    // FTSSEL selects 1.0 or Zn per Zm bit0, then conditionally negates per bit1.
    let mut rng = Rng::new(0x7_5001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        let insn = enc_sve_ftssel(size);
        for _ in 0..20 {
            let mut zn = 0u128;
            for l in 0..(16 / esz) {
                zn |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
            }
            let mut st = ArmState::zeroed();
            st.set_vreg(1, zn as u64, (zn >> 64) as u64);
            st.set_vreg(2, rng.next(), rng.next()); // Zm select bits
            batch.push((format!("ftssel s{size}"), insn, st));
        }
    }
    run_batch("sve_ftssel", batch);
}

#[test]
fn diff_sve_ftmad() {
    // FTMAD fused-multiply-adds Zdn by |Zm| plus a coefficient selected by the
    // immediate and the sign of Zm. Finite inputs; Zm sign varies.
    let mut rng = Rng::new(0x7_6001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz) in [(1u32, 2usize), (2, 4), (3, 8)] {
        for imm in 0..8u32 {
            let insn = enc_sve_ftmad(size, imm);
            for _ in 0..6 {
                let (mut zdn, mut zm) = (0u128, 0u128);
                for l in 0..(16 / esz) {
                    zdn |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                    zm |= (finite_fp_bits(&mut rng, esz) as u128) << (l * esz * 8);
                }
                let mut st = ArmState::zeroed();
                st.set_vreg(0, zdn as u64, (zdn >> 64) as u64); // Zdn
                st.set_vreg(1, zm as u64, (zm >> 64) as u64); // Zm
                batch.push((format!("ftmad s{size} i{imm}"), insn, st));
            }
        }
    }
    run_batch("sve_ftmad", batch);
}

#[test]
fn diff_sve_fmmla() {
    // FMMLA.s: 2x2 f32 matrix multiply-accumulate (plain mul/add). FMMLA.d acts
    // on 256-bit segments, so at VL=128 it must leave Zda unchanged.
    let mut rng = Rng::new(0x7_2001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let insn_s = enc_sve_fmmla(0b10);
    for _ in 0..40 {
        let (mut zn, mut zm, mut za) = (0u128, 0u128, 0u128);
        for l in 0..4 {
            zn |= (finite_fp_bits(&mut rng, 4) as u128) << (l * 32);
            zm |= (finite_fp_bits(&mut rng, 4) as u128) << (l * 32);
            za |= (finite_fp_bits(&mut rng, 4) as u128) << (l * 32);
        }
        let mut st = ArmState::zeroed();
        st.set_vreg(1, zn as u64, (zn >> 64) as u64);
        st.set_vreg(2, zm as u64, (zm >> 64) as u64);
        st.set_vreg(0, za as u64, (za >> 64) as u64);
        batch.push(("fmmla_s".to_string(), insn_s, st));
    }
    let insn_d = enc_sve_fmmla(0b11);
    for _ in 0..6 {
        let mut st = ArmState::zeroed();
        st.set_vreg(1, rng.next(), rng.next());
        st.set_vreg(2, rng.next(), rng.next());
        st.set_vreg(0, rng.next(), rng.next());
        batch.push(("fmmla_d".to_string(), insn_d, st));
    }
    run_batch("sve_fmmla", batch);
}

#[test]
fn diff_sve_bfmmla() {
    // BFMMLA: bf16 2x2 matrix multiply accumulating into f32, round-to-odd.
    let insn = enc_sve_fmmla(0b01);
    let mut rng = Rng::new(0x7_3001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for _ in 0..40 {
        let (mut zn, mut zm, mut za) = (0u128, 0u128, 0u128);
        for l in 0..8 {
            zn |= ((finite_fp_bits(&mut rng, 4) >> 16) as u128) << (l * 16); // finite bf16
            zm |= ((finite_fp_bits(&mut rng, 4) >> 16) as u128) << (l * 16);
        }
        for l in 0..4 {
            za |= (finite_fp_bits(&mut rng, 4) as u128) << (l * 32);
        }
        let mut st = ArmState::zeroed();
        st.set_vreg(1, zn as u64, (zn >> 64) as u64);
        st.set_vreg(2, zm as u64, (zm >> 64) as u64);
        st.set_vreg(0, za as u64, (za >> 64) as u64);
        batch.push(("bfmmla".to_string(), insn, st));
    }
    run_batch("sve_bfmmla", batch);
}

#[test]
fn diff_sve_mmla() {
    // I8MM integer matrix multiply-accumulate, all three sign variants.
    let mut rng = Rng::new(0x7_1001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (uns, name) in [(0b00u32, "smmla"), (0b10, "usmmla"), (0b11, "ummla")] {
        let insn = enc_sve_mmla(uns);
        for _ in 0..30 {
            let mut st = ArmState::zeroed();
            st.set_vreg(1, rng.next(), rng.next());
            st.set_vreg(2, rng.next(), rng.next());
            st.set_vreg(0, rng.next(), rng.next());
            batch.push((name.to_string(), insn, st));
        }
    }
    run_batch("sve_mmla", batch);
}

#[test]
fn diff_sve2_xar() {
    // XAR rotates (Zdn ^ Zm) right by an immediate, per element. Test every size
    // and a spread of rotate amounts including the full-width (identity) case.
    let mut rng = Rng::new(0x6_f001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size_log in 0..4u32 {
        let bits = 8u32 << size_log;
        for &amount in &[1, 2, bits / 2, bits - 1, bits] {
            let insn = enc_sve2_xar(size_log, amount);
            for _ in 0..8 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next()); // Zdn
                st.set_vreg(1, rng.next(), rng.next()); // Zm
                batch.push((format!("xar sl{size_log} a{amount}"), insn, st));
            }
        }
    }
    run_batch("sve2_xar", batch);
}

#[test]
fn diff_sve_fcvtx() {
    // FCVTX narrows f64 lanes to f32 with round-to-odd, merging under Pg.
    let insn = enc_sve_fcvtx();
    let mut rng = Rng::new(0x7_0001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for _ in 0..40 {
        let mut zn = 0u128;
        for l in 0..2 {
            zn |= (finite_fp_bits(&mut rng, 8) as u128) << (l * 64);
        }
        let mut st = ArmState::zeroed();
        st.set_vreg(1, zn as u64, (zn >> 64) as u64);
        st.set_vreg(0, rng.next(), rng.next()); // merge target
        st.set_preg(0, rng.next() as u16);
        batch.push(("fcvtx".to_string(), insn, st));
    }
    run_batch("sve_fcvtx", batch);
}

#[test]
fn diff_sve2_adcl() {
    // ADCLB/ADCLT/SBCLB/SBCLT long add/subtract with carry. The carry-in comes
    // from the high half of each Zm element, so random Zm exercises both carry
    // states; .s and .d, both halves, add and subtract.
    let mut rng = Rng::new(0x6_c001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for inv in 0..2u32 {
        for d_form in 0..2u32 {
            for top in 0..2u32 {
                let insn = enc_sve2_adcl(inv, d_form, top);
                let nm = if inv == 1 { "sbcl" } else { "adcl" };
                for _ in 0..20 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(1, rng.next(), rng.next());
                    st.set_vreg(2, rng.next(), rng.next());
                    st.set_vreg(0, rng.next(), rng.next());
                    batch.push((format!("{nm} d{d_form} t{top}"), insn, st));
                }
            }
        }
    }
    run_batch("sve2_adcl", batch);
}

#[test]
fn diff_sve2_eorbt() {
    // EORBT/EORTB interleaving exclusive OR; the unwritten half of Zd must be
    // preserved, so a random prior Zd is supplied.
    let mut rng = Rng::new(0x6_e001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 0..4u32 {
        for tb in 0..2u32 {
            let insn = enc_sve2_eorbt(size, tb);
            for _ in 0..16 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next());
                st.set_vreg(2, rng.next(), rng.next());
                st.set_vreg(0, rng.next(), rng.next()); // prior Zd
                batch.push((format!("eorbt s{size} tb{tb}"), insn, st));
            }
        }
    }
    run_batch("sve2_eorbt", batch);
}

#[test]
fn diff_sve2_pmull() {
    // PMULLB/PMULLT carryless multiply long: .q (64->128, needs PMULL128), .h
    // (8->16) and .d (32->64), both even (B) and odd (T) halves. Random inputs.
    let mut rng = Rng::new(0x6_9001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in [0u32, 1, 3] {
        for top in 0..2u32 {
            let insn = enc_sve2_pmull(size, top);
            for _ in 0..16 {
                let mut st = ArmState::zeroed();
                st.set_vreg(1, rng.next(), rng.next());
                st.set_vreg(2, rng.next(), rng.next());
                st.set_vreg(0, rng.next(), rng.next()); // dest (overwritten)
                batch.push((format!("pmull sz{size} t{top}"), insn, st));
            }
        }
    }
    run_batch("sve2_pmull", batch);
}

#[test]
fn diff_sve2_crypto() {
    // SVE2 AES/SM4/RAX1 at VL=128 operate on the single 128-bit segment.
    // Random operands suffice (no special-value handling). The slice lists the
    // registers each form reads (and, where destructive, also writes).
    let mut rng = Rng::new(0x6_8001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let cases: [(&str, u32, &[usize]); 7] = [
        ("aesmc", enc_sve2_aesmc(0), &[0]),
        ("aesimc", enc_sve2_aesmc(1), &[0]),
        ("aese", enc_sve2_aes(0x22, 0), &[0, 1]),
        ("aesd", enc_sve2_aes(0x22, 1), &[0, 1]),
        ("sm4e", enc_sve2_aes(0x23, 0), &[0, 1]),
        ("sm4ekey", enc_sve2_sm4ekey(0), &[1, 2]),
        ("rax1", enc_sve2_sm4ekey(1), &[1, 2]),
    ];
    for (name, insn, regs) in cases {
        for _ in 0..20 {
            let mut st = ArmState::zeroed();
            for &r in regs {
                st.set_vreg(r, rng.next(), rng.next());
            }
            batch.push((name.to_string(), insn, st));
        }
    }
    run_batch("sve2_crypto", batch);
}

#[test]
fn diff_sve2_histcnt() {
    // HISTCNT counts, for each active lane i, how many active lanes j<=i hold a
    // Zm value equal to Zn[i]. Draw lane values from a tiny pool so collisions
    // (and thus nonzero, varied counts) are common. Pg=p0 governs.
    let mut rng = Rng::new(0x6_6001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 2..4u32 {
        let esize = 1usize << size;
        let elements = 16 / esize;
        let shift = esize * 8;
        let emask: u128 = if shift >= 128 {
            u128::MAX
        } else {
            (1u128 << shift) - 1
        };
        let insn = enc_sve2_histcnt(size);
        for _ in 0..40 {
            let pool: Vec<u128> = (0..3).map(|_| (rng.next() as u128) & emask).collect();
            let mut zn = 0u128;
            let mut zm = 0u128;
            for e in 0..elements {
                zn |= pool[(rng.next() as usize) % pool.len()] << (e * shift);
                zm |= pool[(rng.next() as usize) % pool.len()] << (e * shift);
            }
            let mut st = ArmState::zeroed();
            st.set_vreg(1, zn as u64, (zn >> 64) as u64);
            st.set_vreg(2, zm as u64, (zm >> 64) as u64);
            st.set_preg(0, rng.next() as u16); // Pg = p0
            batch.push((format!("histcnt sz{size}"), insn, st));
        }
    }
    run_batch("sve2_histcnt", batch);
}

#[test]
fn diff_sve2_histseg() {
    // HISTSEG: each result byte counts equal Zm bytes for the matching Zn byte.
    let insn = enc_sve2_histseg();
    let mut rng = Rng::new(0x6_7001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for _ in 0..60 {
        let mut st = ArmState::zeroed();
        st.set_vreg(1, rng.next(), rng.next()); // Zn
        st.set_vreg(2, rng.next(), rng.next()); // Zm
        batch.push(("histseg".to_string(), insn, st));
    }
    // Planted: Zm all one byte value, so counts are 0 or 16.
    for b in [0x00u128, 0x42, 0xFF] {
        let mut same = 0u128;
        for k in 0..16 {
            same |= b << (k * 8);
        }
        let mut st = ArmState::zeroed();
        st.set_vreg(1, rng.next(), rng.next());
        st.set_vreg(2, same as u64, (same >> 64) as u64);
        batch.push(("histseg_same".to_string(), insn, st));
    }
    run_batch("sve2_histseg", batch);
}

#[test]
fn diff_sve2_match() {
    // MATCH/NMATCH set a predicate bit where a Zn element equals (or, for
    // NMATCH, differs from) every Zm element in the 128-bit segment. Zn lanes
    // are a mix of copied Zm values (guaranteed matches) and random values, so
    // both branches are exercised. Pg=p1 governs (zeroing).
    let mut rng = Rng::new(0x6_5001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 0..2u32 {
        let esize = 1usize << size;
        let elements = 16 / esize;
        let shift = esize * 8;
        let emask: u128 = (1u128 << shift) - 1;
        for nmatch in 0..2u32 {
            let insn = enc_sve2_match(size, nmatch);
            for _ in 0..40 {
                let mut st = ArmState::zeroed();
                let zm_lo = rng.next();
                let zm_hi = rng.next();
                let zm = (zm_lo as u128) | ((zm_hi as u128) << 64);
                let mut zn = 0u128;
                for e in 0..elements {
                    let v = if rng.next() & 1 == 0 {
                        let j = (rng.next() as usize) % elements;
                        (zm >> (j * shift)) & emask
                    } else {
                        (rng.next() as u128) & emask
                    };
                    zn |= v << (e * shift);
                }
                st.set_vreg(1, zn as u64, (zn >> 64) as u64);
                st.set_vreg(2, zm_lo, zm_hi);
                st.set_preg(1, rng.next() as u16); // Pg = p1
                batch.push((format!("match sz{size} n{nmatch}"), insn, st));
            }
        }
    }
    run_batch("sve2_match", batch);
}

#[test]
fn diff_sve2_whilerw() {
    // WHILERW/WHILEWR generate a hazard predicate from the byte distance between
    // two pointers. Probe controlled deltas spanning the per-element boundary
    // (including Xn>Xm and Xn==Xm), plus fully random address pairs.
    let mut rng = Rng::new(0x6_4001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let deltas: [i64; 16] = [-40, -8, -1, 0, 1, 2, 3, 4, 7, 8, 15, 16, 32, 40, 64, 200];
    for sz in 0..4u32 {
        for rw in 0..2u32 {
            let insn = enc_whilerw(sz, rw);
            for &base in &[0x1000u64, 0x200040, 0xFFFF_FFF0] {
                for &delta in &deltas {
                    let mut st = gen_input(&mut rng);
                    st.x[1] = base; // Xn
                    st.x[2] = base.wrapping_add(delta as u64); // Xm
                    batch.push((format!("whilerw sz{sz} rw{rw} d{delta}"), insn, st));
                }
            }
            for _ in 0..16 {
                let mut st = gen_input(&mut rng);
                st.x[1] = rng.next();
                st.x[2] = rng.next();
                batch.push((format!("whilerw sz{sz} rw{rw} rand"), insn, st));
            }
        }
    }
    run_batch("sve2_whilerw", batch);
}

#[test]
fn diff_sve2_mull_indexed() {
    // Indexed widening multiply-long: S/U MULL/MLAL/MLSL and the saturating
    // SQDMULL/SQDMLAL/SQDMLSL, across .s and .d, all indices and both halves.
    // Random operands plus a min*min saturation edge for the saturating forms.
    let ops: [(u32, &str, bool); 9] = [
        (0b1100, "smullb", false),
        (0b1101, "umullb", false),
        (0b1000, "smlalb", false),
        (0b1001, "umlalb", false),
        (0b1010, "smlslb", false),
        (0b1011, "umlslb", false),
        (0b1110, "sqdmullb", true),
        (0b0010, "sqdmlalb", true),
        (0b0011, "sqdmlslb", true),
    ];
    let mut rng = Rng::new(0x6_b001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (op, name, sat) in ops {
        for size in [2u32, 3] {
            let idxn = if size == 2 { 8 } else { 4 };
            let s_esize = 1usize << (size - 1);
            for index in 0..idxn {
                for top in 0..2u32 {
                    let insn = enc_sve2_mull_idx(op, size, index, RM, top);
                    for _ in 0..4 {
                        let mut st = ArmState::zeroed();
                        st.set_vreg(1, rng.next(), rng.next());
                        st.set_vreg(2, rng.next(), rng.next());
                        st.set_vreg(0, rng.next(), rng.next());
                        batch.push((format!("{name}_s{size}_i{index}_t{top}"), insn, st));
                    }
                    if sat {
                        let m_elem = 1u128 << (s_esize * 8 - 1);
                        let mut minv = 0u128;
                        for l in 0..(16 / s_esize) {
                            minv |= m_elem << (l * s_esize * 8);
                        }
                        let mut st = ArmState::zeroed();
                        st.set_vreg(1, minv as u64, (minv >> 64) as u64);
                        st.set_vreg(2, minv as u64, (minv >> 64) as u64);
                        st.set_vreg(0, rng.next(), rng.next());
                        batch.push((format!("{name}_s{size}_i{index}_t{top}_sat"), insn, st));
                    }
                }
            }
        }
    }
    run_batch("sve2_mull_indexed", batch);
}

#[test]
fn diff_sve2_mul_indexed() {
    // MUL/MLA/MLS/SQDMULH/SQRDMULH and the saturating-rounding accumulate
    // SQRDMLAH/SQRDMLSH by an indexed Zm element, across all sizes and index
    // positions. Random operands plus a min*min saturation edge for the
    // saturating forms.
    let ops: [(u32, &str); 7] = [
        (0b111110, "mul"),
        (0b000010, "mla"),
        (0b000011, "mls"),
        (0b111100, "sqdmulh"),
        (0b111101, "sqrdmulh"),
        (0b000100, "sqrdmlah"),
        (0b000101, "sqrdmlsh"),
    ];
    let sizes: [(u32, usize, u32); 3] = [(1, 2, 8), (2, 4, 4), (3, 8, 2)]; // size, esize, #idx
    let mut rng = Rng::new(0x6_3001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (op6, opname) in ops {
        let saturating = matches!(op6, 0b111100 | 0b111101 | 0b000100 | 0b000101);
        for (size, esz, idxn) in sizes {
            for index in 0..idxn {
                let insn = enc_sve2_mul_idx(op6, size, index, RM); // Zm = z2
                for _ in 0..6 {
                    let mut st = ArmState::zeroed();
                    st.set_vreg(1, rng.next(), rng.next()); // Zn
                    st.set_vreg(2, rng.next(), rng.next()); // Zm
                    st.set_vreg(0, rng.next(), rng.next()); // Zd (accumulator)
                    batch.push((format!("{opname}_sz{size}_i{index}"), insn, st));
                }
                if saturating {
                    // Force min*min in every lane to exercise the saturation clamp.
                    let m_elem = 1u128 << (esz * 8 - 1);
                    let mut minv = 0u128;
                    for l in 0..(16 / esz) {
                        minv |= m_elem << (l * esz * 8);
                    }
                    let mut st = ArmState::zeroed();
                    st.set_vreg(1, minv as u64, (minv >> 64) as u64);
                    st.set_vreg(2, minv as u64, (minv >> 64) as u64);
                    st.set_vreg(0, rng.next(), rng.next());
                    batch.push((format!("{opname}_sz{size}_i{index}_sat"), insn, st));
                }
            }
        }
    }
    // Rounding-tie edge for SQRDMLSH .h: with every Zn lane and Zm[0] = 0x0080
    // the product is exactly 2^14, so the rounded doubling-high is 0 only when
    // the product is negated BEFORE the rounding bias (qemu); negating the
    // rounded SQRDMLAH result instead would give -1. The result must equal Zda.
    for op6 in [0b000101u32, 0b000100] {
        let insn = enc_sve2_mul_idx(op6, 1, 0, RM);
        let lanes: u128 = {
            let mut v = 0u128;
            for l in 0..8 {
                v |= 0x0080u128 << (l * 16);
            }
            v
        };
        let mut st = ArmState::zeroed();
        st.set_vreg(1, lanes as u64, (lanes >> 64) as u64);
        st.set_vreg(2, lanes as u64, (lanes >> 64) as u64);
        st.set_vreg(0, rng.next(), rng.next());
        batch.push((format!("sqrdml_tie_{op6:b}"), insn, st));
    }
    run_batch("sve2_mul_indexed", batch);
}

#[test]
fn diff_sve2_flogb() {
    // FLOGB returns floor(log2|x|) of each FP lane as a signed integer. Each
    // lane is drawn from edge cases (zero/inf/NaN/subnormal), bounded finite
    // values, or fully random bit patterns to exercise the whole input space.
    let sizes = [(1u32, 2usize, "h"), (2, 4, "s"), (3, 8, "d")];
    let mut rng = Rng::new(0x6_2001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (size, esz, name) in sizes {
        let insn = enc_sve_flogb(size);
        let specials = flogb_specials(esz);
        let lanes = 16 / esz;
        let mask: u64 = if esz == 8 {
            u64::MAX
        } else {
            (1u64 << (esz * 8)) - 1
        };
        for _ in 0..48 {
            let mut st = ArmState::zeroed();
            let mut zn: u128 = 0;
            for l in 0..lanes {
                let v = match rng.next() % 3 {
                    0 => specials[(rng.next() as usize) % specials.len()],
                    1 => finite_fp_bits(&mut rng, esz),
                    _ => rng.next() & mask,
                };
                zn |= (v as u128) << (l * esz * 8);
            }
            st.set_vreg(1, zn as u64, (zn >> 64) as u64);
            st.set_vreg(0, rng.next(), rng.next()); // merge target (inactive lanes)
            st.set_preg(0, rng.next() as u16);
            batch.push((format!("flogb_{name}"), insn, st));
        }
    }
    run_batch("sve2_flogb", batch);
}

#[test]
fn diff_sve_gather_d() {
    // 64-bit gather load: each active D lane reads from Xn + (Zm[e] << scale).
    // Zm offsets are kept small so every gathered address stays in the window.
    let mut rng = Rng::new(0x4_0001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for &scaled in &[false, true] {
            if scaled && msz == 0 {
                continue; // no scaled byte form
            }
            for u in 0..2u32 {
                if u == 0 && msz == 3 {
                    continue; // no signed 64-bit load (LD1SD does not exist)
                }
                let insn = enc_gather_d(msz, scaled, u);
                let name = format!("gather m{msz} sc{} u{u}", scaled as u32);
                for _ in 0..8 {
                    let mut st = mem_input(&mut rng);
                    let e0 = (rng.next() % 4) as u128;
                    let e1 = (rng.next() % 4) as u128;
                    let zm = e0 | (e1 << 64);
                    st.set_vreg(2, zm as u64, (zm >> 64) as u64); // Zm offsets
                    st.set_preg(0, rng.next() as u16);
                    batch.push((name.clone(), insn, st));
                }
            }
        }
    }
    run_batch("sve_gather_d", batch);
}

#[test]
fn diff_sve_scatter_d() {
    // 64-bit scatter store: each active D lane writes to Xn + (Zm[e] << scale).
    let mut rng = Rng::new(0x4_1001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for &scaled in &[false, true] {
            if scaled && msz == 0 {
                continue;
            }
            let insn = enc_scatter_d(msz, scaled);
            let name = format!("scatter m{msz} sc{}", scaled as u32);
            for _ in 0..8 {
                let mut st = mem_input(&mut rng);
                let e0 = (rng.next() % 4) as u128;
                let e1 = (rng.next() % 4) as u128;
                let zm = e0 | (e1 << 64);
                st.set_vreg(2, zm as u64, (zm >> 64) as u64); // Zm offsets
                st.set_preg(0, rng.next() as u16);
                batch.push((name.clone(), insn, st));
            }
        }
    }
    run_batch("sve_scatter_d", batch);
}

#[test]
fn diff_sve_ldff1() {
    // First-fault (LDFF1) / non-fault (LDNF1) contiguous loads. Only the FIRST
    // active element is architecturally guaranteed to load — qemu legally
    // suppresses later elements (CONSTRAINED UNPREDICTABLE), even a lone
    // higher-index active element. So each case activates ONLY element 0 (always
    // the guaranteed element) with FFR pre-set all-true via a leading SETFFR;
    // the loaded element-0 Zt is then deterministic and equals LD1. Per-element
    // load correctness for the other lanes is covered by the LD1/LD1R families.
    let mut rng = Rng::new(0x4_E001);
    let mut batch: Vec<(String, u32, u32, ArmState)> = Vec::new();
    for dtype in 0..16u32 {
        let ldff = enc_ldff1(dtype);
        for &xm in &[0u64, 1, 2, 3] {
            let mut st = mem_input(&mut rng);
            st.x[2] = xm;
            st.set_preg(0, 1); // only element 0 active
            batch.push((format!("ldff1 dt{dtype} x{xm}"), enc_setffr(), ldff, st));
        }
        for &imm4 in &[0i32, 1, -1] {
            let ldnf = enc_ldnf1(dtype, imm4);
            let mut st = mem_input(&mut rng);
            st.set_preg(0, 1);
            let _ = &mut rng;
            batch.push((format!("ldnf1 dt{dtype} i{imm4}"), enc_setffr(), ldnf, st));
        }
    }
    run_batch_pair("sve_ldff1", batch);
}

#[test]
fn diff_sve_ldff1_gather() {
    // First-fault gather (D-form, ff=bit13 set). As with contiguous LDFF1, only
    // element 0 is architecturally guaranteed, so it alone is activated with FFR
    // pre-set; the loaded lane then equals the plain gather.
    let mut rng = Rng::new(0x4_F001);
    let mut batch: Vec<(String, u32, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for &scaled in &[false, true] {
            if scaled && msz == 0 {
                continue; // no scaled byte form
            }
            for u in 0..2u32 {
                if u == 0 && msz == 3 {
                    continue;
                }
                let insn = enc_gather_d(msz, scaled, u) | (1 << 13); // ff=1 -> LDFF1
                let name = format!("ldff1g m{msz} sc{} u{u}", scaled as u32);
                for _ in 0..4 {
                    let mut st = mem_input(&mut rng);
                    let e0 = (rng.next() % 4) as u64;
                    st.set_vreg(2, e0, 0); // element-0 offset
                    st.set_preg(0, 1); // only element 0 active
                    batch.push((name.clone(), enc_setffr(), insn, st));
                }
            }
        }
    }
    run_batch_pair("sve_ldff1_gather", batch);
}

#[test]
fn diff_sve_ffr() {
    // FFR manipulation, tested as two-instruction sequences that read the FFR
    // back into a captured predicate (p0): SETFFR/WRFFR then RDFFR.
    let mut rng = Rng::new(0x4_D001);
    let mut batch: Vec<(String, u32, u32, ArmState)> = Vec::new();
    for _ in 0..8 {
        // SETFFR ; RDFFR p0  -> p0 = all-true.
        batch.push((
            "setffr+rdffr".to_string(),
            enc_setffr(),
            enc_rdffr(0),
            ArmState::zeroed(),
        ));
    }
    for _ in 0..16 {
        // WRFFR p1 ; RDFFR p0  -> p0 = p1.
        let mut st = ArmState::zeroed();
        st.set_preg(1, rng.next() as u16);
        batch.push(("wrffr+rdffr".to_string(), enc_wrffr(1), enc_rdffr(0), st));
    }
    for _ in 0..16 {
        // WRFFR p1 ; RDFFR p0, p2/Z  -> p0 = p1 & p2.
        let mut st = ArmState::zeroed();
        st.set_preg(1, rng.next() as u16);
        st.set_preg(2, rng.next() as u16);
        batch.push((
            "wrffr+rdffr_pred".to_string(),
            enc_wrffr(1),
            enc_rdffr_pred(0, 2),
            st,
        ));
    }
    run_batch_pair("sve_ffr", batch);
}

#[test]
fn diff_sve_ld1rq() {
    // Load-replicate quadword (at VL=128 a packed contiguous quadword load).
    let mut rng = Rng::new(0x4_C001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for &imm4 in &[0i32, 1, -1] {
            let insn = enc_ld1rq_i(msz, imm4);
            for _ in 0..5 {
                let mut st = mem_input(&mut rng);
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("ld1rqi m{msz} i{imm4}"), insn, st));
            }
        }
        for &xm in &[0u64, 1, 2] {
            let insn = enc_ld1rq_r(msz);
            for _ in 0..5 {
                let mut st = mem_input(&mut rng);
                st.x[2] = xm;
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("ld1rqr m{msz} x{xm}"), insn, st));
            }
        }
    }
    run_batch("sve_ld1rq", batch);
}

#[test]
fn diff_sve_ldnt1() {
    // Non-temporal contiguous load/store (behaves like packed LD1/ST1).
    let mut rng = Rng::new(0x4_9001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for &imm4 in &[0i32, 1, -1] {
            for _ in 0..6 {
                let mut s1 = mem_input(&mut rng);
                s1.set_preg(0, rng.next() as u16);
                batch.push((format!("ldnt1 m{msz} i{imm4}"), enc_ldnt1(msz, imm4), s1));
                let mut s2 = mem_input(&mut rng);
                s2.set_preg(0, rng.next() as u16);
                batch.push((format!("stnt1 m{msz} i{imm4}"), enc_stnt1(msz, imm4), s2));
            }
        }
    }
    run_batch("sve_ldnt1", batch);
}

#[test]
fn diff_sve_ldn_stn() {
    // Multi-register de-interleaving loads (LD2/3/4) and interleaving stores
    // (ST2/3/4), all element sizes and structure counts.
    let mut rng = Rng::new(0x4_8001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for nreg in 2..=4u32 {
            for &imm4 in &[0i32, 1] {
                let ld = enc_ldn(msz, nreg, imm4);
                let st = enc_stn(msz, nreg, imm4);
                for _ in 0..4 {
                    let mut s1 = mem_input(&mut rng);
                    s1.set_preg(0, rng.next() as u16);
                    batch.push((format!("ld{nreg} m{msz} i{imm4}"), ld, s1));
                    let mut s2 = mem_input(&mut rng);
                    s2.set_preg(0, rng.next() as u16);
                    batch.push((format!("st{nreg} m{msz} i{imm4}"), st, s2));
                }
            }
        }
    }
    run_batch("sve_ldn_stn", batch);
}

#[test]
fn diff_sve_gather_ai_s() {
    // S-form vector-base gather: 4 S lanes from Zn[e]<31:0> + imm5*mbytes.
    let mut rng = Rng::new(0x4_A001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..3u32 {
        for &imm5 in &[0u32, 1, 2] {
            for u in 0..2u32 {
                if u == 0 && msz == 2 {
                    continue;
                }
                let insn = enc_gather_ai_s(msz, imm5, u);
                let name = format!("gais m{msz} i{imm5} u{u}");
                for _ in 0..6 {
                    let mut st = mem_input(&mut rng);
                    let mut zn: u128 = 0;
                    for e in 0..4 {
                        let b = (SCRATCH_BASE as u32).wrapping_add((rng.next() % 16) as u32);
                        zn |= (b as u128) << (e * 32);
                    }
                    st.set_vreg(1, zn as u64, (zn >> 64) as u64);
                    st.set_preg(0, rng.next() as u16);
                    batch.push((name.clone(), insn, st));
                }
            }
        }
    }
    run_batch("sve_gather_ai_s", batch);
}

#[test]
fn diff_sve_scatter_ai_s() {
    let mut rng = Rng::new(0x4_B001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..3u32 {
        for &imm5 in &[0u32, 1, 2] {
            let insn = enc_scatter_ai_s(msz, imm5);
            let name = format!("sais m{msz} i{imm5}");
            for _ in 0..6 {
                let mut st = mem_input(&mut rng);
                let mut zn: u128 = 0;
                for e in 0..4 {
                    let b = (SCRATCH_BASE as u32).wrapping_add((rng.next() % 16) as u32);
                    zn |= (b as u128) << (e * 32);
                }
                st.set_vreg(1, zn as u64, (zn >> 64) as u64);
                st.set_preg(0, rng.next() as u16);
                batch.push((name.clone(), insn, st));
            }
        }
    }
    run_batch("sve_scatter_ai_s", batch);
}

#[test]
fn diff_sve_gather_ai() {
    // Vector-base gather: each D lane reads from Zn[e] + imm5*mbytes. Zn holds
    // per-element base addresses pointed into the scratch window.
    let mut rng = Rng::new(0x4_6001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for &imm5 in &[0u32, 1, 2] {
            for u in 0..2u32 {
                if u == 0 && msz == 3 {
                    continue;
                }
                let insn = enc_gather_ai(msz, imm5, u);
                let name = format!("gai m{msz} i{imm5} u{u}");
                for _ in 0..6 {
                    let mut st = mem_input(&mut rng);
                    let b0 = SCRATCH_BASE + (rng.next() % 16);
                    let b1 = SCRATCH_BASE + (rng.next() % 16);
                    st.set_vreg(1, b0, b1); // Zn = z1
                    st.set_preg(0, rng.next() as u16);
                    batch.push((name.clone(), insn, st));
                }
            }
        }
    }
    run_batch("sve_gather_ai", batch);
}

#[test]
fn diff_sve_scatter_ai() {
    let mut rng = Rng::new(0x4_7001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for &imm5 in &[0u32, 1, 2] {
            let insn = enc_scatter_ai(msz, imm5);
            let name = format!("sai m{msz} i{imm5}");
            for _ in 0..6 {
                let mut st = mem_input(&mut rng);
                let b0 = SCRATCH_BASE + (rng.next() % 16);
                let b1 = SCRATCH_BASE + (rng.next() % 16);
                st.set_vreg(1, b0, b1);
                st.set_preg(0, rng.next() as u16);
                batch.push((name.clone(), insn, st));
            }
        }
    }
    run_batch("sve_scatter_ai", batch);
}

#[test]
fn diff_sve_gather_x32() {
    // Unpacked gather: D lanes, 32-bit offsets from Xn + (extend(Zm[e]<31:0>,xs)
    // << scale).
    let mut rng = Rng::new(0x4_4001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for xs in 0..2u32 {
            for &scaled in &[false, true] {
                if scaled && msz == 0 {
                    continue;
                }
                for u in 0..2u32 {
                    if u == 0 && msz == 3 {
                        continue;
                    }
                    let insn = enc_gather_x32(msz, xs, scaled, u);
                    let name = format!("gx m{msz} x{xs} sc{} u{u}", scaled as u32);
                    for _ in 0..6 {
                        let mut st = mem_input(&mut rng);
                        let e0 = (rng.next() % 4) as u128;
                        let e1 = (rng.next() % 4) as u128;
                        let zm = e0 | (e1 << 64);
                        st.set_vreg(2, zm as u64, (zm >> 64) as u64);
                        st.set_preg(0, rng.next() as u16);
                        batch.push((name.clone(), insn, st));
                    }
                }
            }
        }
    }
    run_batch("sve_gather_x32", batch);
}

#[test]
fn diff_sve_scatter_x32() {
    let mut rng = Rng::new(0x4_5001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for xs in 0..2u32 {
            for &scaled in &[false, true] {
                if scaled && msz == 0 {
                    continue;
                }
                let insn = enc_scatter_x32(msz, xs, scaled);
                let name = format!("sx m{msz} x{xs} sc{}", scaled as u32);
                for _ in 0..6 {
                    let mut st = mem_input(&mut rng);
                    let e0 = (rng.next() % 4) as u128;
                    let e1 = (rng.next() % 4) as u128;
                    let zm = e0 | (e1 << 64);
                    st.set_vreg(2, zm as u64, (zm >> 64) as u64);
                    st.set_preg(0, rng.next() as u16);
                    batch.push((name.clone(), insn, st));
                }
            }
        }
    }
    run_batch("sve_scatter_x32", batch);
}

#[test]
fn diff_sve_gather_s() {
    // 32-bit gather load: 4 S lanes from Xn + (extend(Zm[e],xs) << scale).
    let mut rng = Rng::new(0x4_2001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..3u32 {
        for xs in 0..2u32 {
            for &scaled in &[false, true] {
                if scaled && msz == 0 {
                    continue;
                }
                for u in 0..2u32 {
                    if u == 0 && msz == 2 {
                        continue; // no signed word->S load
                    }
                    let insn = enc_gather_s(msz, xs, scaled, u);
                    let name = format!("gs m{msz} x{xs} sc{} u{u}", scaled as u32);
                    for _ in 0..6 {
                        let mut st = mem_input(&mut rng);
                        let mut zm: u128 = 0;
                        for e in 0..4 {
                            zm |= ((rng.next() % 4) as u128) << (e * 32);
                        }
                        st.set_vreg(2, zm as u64, (zm >> 64) as u64);
                        st.set_preg(0, rng.next() as u16);
                        batch.push((name.clone(), insn, st));
                    }
                }
            }
        }
    }
    run_batch("sve_gather_s", batch);
}

#[test]
fn diff_sve_scatter_s() {
    let mut rng = Rng::new(0x4_3001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..3u32 {
        for xs in 0..2u32 {
            for &scaled in &[false, true] {
                if scaled && msz == 0 {
                    continue;
                }
                let insn = enc_scatter_s(msz, xs, scaled);
                let name = format!("ss m{msz} x{xs} sc{}", scaled as u32);
                for _ in 0..6 {
                    let mut st = mem_input(&mut rng);
                    let mut zm: u128 = 0;
                    for e in 0..4 {
                        zm |= ((rng.next() % 4) as u128) << (e * 32);
                    }
                    st.set_vreg(2, zm as u64, (zm >> 64) as u64);
                    st.set_preg(0, rng.next() as u16);
                    batch.push((name.clone(), insn, st));
                }
            }
        }
    }
    run_batch("sve_scatter_s", batch);
}

#[test]
fn diff_sve_ld1_ss() {
    // LD1 with a scalar index register (element addresses base + (Xm+e)*mbytes).
    let mut rng = Rng::new(0x3_8001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for dtype in 0..16u32 {
        for &xm in &[0u64, 1, 2] {
            let insn = enc_sve_ld1_ss(dtype);
            for _ in 0..4 {
                let mut st = mem_input(&mut rng);
                st.x[2] = xm; // index register Rm
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("ld1ss dt{dtype} x{xm}"), insn, st));
            }
        }
    }
    run_batch("sve_ld1_ss", batch);
}

#[test]
fn diff_sve_st1_ss() {
    let mut rng = Rng::new(0x3_9001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for size in msz..4u32 {
            for &xm in &[0u64, 1, 2] {
                let insn = enc_sve_st1_ss(msz, size);
                for _ in 0..4 {
                    let mut st = mem_input(&mut rng);
                    st.x[2] = xm;
                    st.set_preg(0, rng.next() as u16);
                    batch.push((format!("st1ss m{msz} e{size} x{xm}"), insn, st));
                }
            }
        }
    }
    run_batch("sve_st1_ss", batch);
}

#[test]
fn diff_sve_ld1() {
    // Contiguous LD1{B,H,W,D}/LD1S{B,H,W} from the scratch window. The random
    // predicate exercises the byte-granular governing and zeroing of inactive
    // lanes; the various dtypes cover every mem-size/element-size/sign pairing.
    let mut rng = Rng::new(0x3_2001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for dtype in 0..16u32 {
        for &imm in &[0i32, 1, -1] {
            let insn = enc_sve_ld1(dtype, imm);
            for _ in 0..6 {
                let mut st = mem_input(&mut rng);
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("ld1 dt{dtype} i{imm}"), insn, st));
            }
        }
    }
    run_batch("sve_ld1", batch);
}

#[test]
fn diff_sve_st1() {
    // Contiguous ST1{B,H,W,D} to the scratch window (element width >= memory
    // width). Inactive lanes leave memory untouched; the store truncates each
    // element to the memory width.
    let mut rng = Rng::new(0x3_3001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for msz in 0..4u32 {
        for size in msz..4u32 {
            for &imm in &[0i32, 1] {
                let insn = enc_sve_st1(msz, size, imm);
                for _ in 0..6 {
                    let mut st = mem_input(&mut rng);
                    st.set_preg(0, rng.next() as u16);
                    batch.push((format!("st1 m{msz} e{size} i{imm}"), insn, st));
                }
            }
        }
    }
    run_batch("sve_st1", batch);
}

#[test]
fn diff_sve_adr() {
    // Vector address generation across all four forms (D+SXTW, D+UXTW, S/D
    // packed) and shift amounts.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for mode in 0..4u32 {
        for msz in 0..4u32 {
            cases.push((format!("adr m{mode} s{msz}"), enc_sve_adr(mode, msz)));
        }
    }
    run_family("sve_adr", cases, 16, 0x3_7001);
}

#[test]
fn diff_sve_movprfx() {
    // MOVPRFX standalone is a move: unpredicated copies the whole register; the
    // predicated form copies active lanes and merges/zeros the inactive ones.
    let mut rng = Rng::new(0x3_6001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let zi = enc_movprfx_z();
    for _ in 0..12 {
        let mut st = ArmState::zeroed();
        st.set_vreg(0, rng.next(), rng.next());
        st.set_vreg(1, rng.next(), rng.next());
        batch.push(("movprfx_z".to_string(), zi, st));
    }
    for size in 0..4u32 {
        for m in 0..2u32 {
            let insn = enc_movprfx_p(size, m);
            for _ in 0..8 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next()); // prior Zd (merge target)
                st.set_vreg(1, rng.next(), rng.next()); // Zn
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("movprfx_p e{size} m{m}"), insn, st));
            }
        }
    }
    run_batch("sve_movprfx", batch);
}

#[test]
fn diff_sve_ld1r() {
    // Load-and-replicate: one extended memory element broadcast to active lanes.
    let mut rng = Rng::new(0x3_5001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for dtype in 0..16u32 {
        let (dh, dl) = (dtype >> 2, dtype & 3);
        for &imm6 in &[0u32, 1, 2, 3] {
            let insn = enc_ld1r(dh, dl, imm6);
            for _ in 0..6 {
                let mut st = mem_input(&mut rng);
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("ld1r dt{dtype} i{imm6}"), insn, st));
            }
        }
    }
    run_batch("sve_ld1r", batch);
}

#[test]
fn diff_sve_ldr_str() {
    // Whole-register fill/spill of Z and P registers (unpredicated).
    let mut rng = Rng::new(0x3_4001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for &imm in &[0i32, 1, -1, 2] {
        let variants = [
            ("ldr_z", enc_ldstr_z(false, imm)),
            ("str_z", enc_ldstr_z(true, imm)),
            ("ldr_p", enc_ldstr_p(false, imm)),
            ("str_p", enc_ldstr_p(true, imm)),
        ];
        for (label, insn) in variants {
            for _ in 0..6 {
                let mut st = mem_input(&mut rng);
                st.set_vreg(0, rng.next(), rng.next()); // Zt source (str_z)
                st.set_preg(0, rng.next() as u16); // Pt source (str_p)
                batch.push((format!("{label} i{imm}"), insn, st));
            }
        }
    }
    run_batch("sve_ldr_str", batch);
}

#[test]
fn diff_sve_cvt() {
    // Predicated FCVTZS/FCVTZU (FP->int, trunc+saturate) and SCVTF/UCVTF
    // (int->FP, round-to-nearest). Each (opc,opc2) fixes the FP/int width pair;
    // ig1 picks the direction and int_U the signedness.
    let table = [
        (0b01u32, 0b01u32, 2usize, 2usize),
        (0b01, 0b10, 2, 4),
        (0b01, 0b11, 2, 8),
        (0b10, 0b10, 4, 4),
        (0b11, 0b10, 4, 8),
        (0b11, 0b00, 8, 4),
        (0b11, 0b11, 8, 8),
    ];
    let mut rng = Rng::new(0x3_1001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (opc, opc2, fp_sz, int_sz) in table {
        let cont = fp_sz.max(int_sz);
        let elements = 16 / cont;
        for (ig1, to_int) in [(0b011u32, true), (0b010u32, false)] {
            for u in 0..2u32 {
                let insn = enc_sve_cvt(opc, ig1, opc2, u);
                let name = format!(
                    "{}{}_f{}i{}",
                    if to_int { "fcvtz" } else { "cvtf" },
                    if u == 0 { "s" } else { "u" },
                    fp_sz * 8,
                    int_sz * 8
                );
                let imask: u64 = if int_sz == 8 {
                    u64::MAX
                } else {
                    (1u64 << (int_sz * 8)) - 1
                };
                for _ in 0..20 {
                    let mut st = ArmState::zeroed();
                    let mut zn: u128 = 0;
                    for e in 0..elements {
                        let bits = if to_int {
                            finite_fp_bits(&mut rng, fp_sz)
                        } else {
                            rng.next() & imask
                        };
                        zn |= (bits as u128) << (e * cont * 8);
                    }
                    st.set_vreg(1, zn as u64, (zn >> 64) as u64);
                    st.set_vreg(0, rng.next(), rng.next()); // prior Zd (merge)
                    st.set_preg(0, rng.next() as u16);
                    batch.push((name.clone(), insn, st));
                }
            }
        }
    }
    run_batch("sve_cvt", batch);
}

#[test]
fn diff_sve_dup_idx() {
    // DUP indexed broadcasts one element of Zn to all lanes; an index past the
    // end broadcasts zero. Covers every element size and in/out-of-range index.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for (elog, name, nelem) in [
        (0u32, "b", 16u32),
        (1, "h", 8),
        (2, "s", 4),
        (3, "d", 2),
        (4, "q", 1),
    ] {
        for index in [0u32, 1, nelem / 2, nelem - 1, nelem, nelem + 2] {
            cases.push((format!("dup_idx {name}[{index}]"), enc_dup_idx(elog, index)));
        }
    }
    run_family("sve_dup_idx", cases, 8, 0x2_E001);
}

#[test]
fn diff_sve_compact() {
    // COMPACT packs Pg-active S/D elements of Zn to the bottom of Zd, zeroing
    // the rest. A random predicate exercises arbitrary active-element patterns.
    let mut rng = Rng::new(0x2_7001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..2u32 {
        let insn = enc_sve_compact(sz);
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            st.set_vreg(1, rng.next(), rng.next());
            st.set_preg(0, rng.next() as u16);
            batch.push((format!("compact sz{sz}"), insn, st));
        }
    }
    run_batch("sve_compact", batch);
}

#[test]
fn diff_sve_splice() {
    // SPLICE copies Zdn's active span (first..last active) to the low result,
    // then fills the remainder from Zm. Random Zdn/Zm/Pg cover the active-span,
    // wrap, and empty-predicate (result == Zm) paths.
    let mut rng = Rng::new(0x2_8001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..4u32 {
        let insn = enc_sve_splice(sz);
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            st.set_vreg(0, rng.next(), rng.next()); // Zdn (source-1 + dest)
            st.set_vreg(1, rng.next(), rng.next()); // Zm
            st.set_preg(0, rng.next() as u16);
            batch.push((format!("splice sz{sz}"), insn, st));
        }
    }
    run_batch("sve_splice", batch);
}

#[test]
fn diff_sve_pfirst() {
    // PFIRST sets the first Pg-active element active in Pdn and writes NZCV via
    // PredTest. Random Pg/Pdn inputs cover the empty-mask and already-set paths.
    let mut rng = Rng::new(0x2_9001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    let insn = enc_pfirst(1, 0); // Pg=p1, Pdn=p0
    for _ in 0..40 {
        let mut st = ArmState::zeroed();
        st.set_preg(0, rng.next() as u16); // Pdn (source + dest)
        st.set_preg(1, rng.next() as u16); // Pg
        batch.push(("pfirst".to_string(), insn, st));
    }
    run_batch("sve_pfirst", batch);
}

#[test]
fn diff_sve_pnext() {
    // PNEXT advances to the next Pg-active element after Pdn's last active one,
    // for each element size, and writes NZCV via PredTest.
    let mut rng = Rng::new(0x2_A001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for sz in 0..4u32 {
        let insn = enc_pnext(sz, 1, 0);
        for _ in 0..40 {
            let mut st = ArmState::zeroed();
            st.set_preg(0, rng.next() as u16); // Pdn (source + dest)
            st.set_preg(1, rng.next() as u16); // Pg
            batch.push((format!("pnext sz{sz}"), insn, st));
        }
    }
    run_batch("sve_pnext", batch);
}

#[test]
fn diff_sve_brk() {
    // BRKA (break after) / BRKB (break before) the first Pn-true element, in
    // both merging/zeroing and flag-setting forms. Random Pg/Pn/prior-Pd.
    let mut rng = Rng::new(0x2_B001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for b in 0..2u32 {
        for s in 0..2u32 {
            for m in 0..2u32 {
                let insn = enc_brka(b, s, m, 1, 2, 0); // Pg=p1, Pn=p2, Pd=p0
                let name = format!("brk{} s{s} m{m}", if b == 0 { "a" } else { "b" });
                for _ in 0..16 {
                    let mut st = ArmState::zeroed();
                    st.set_preg(0, rng.next() as u16); // Pd (merge source)
                    st.set_preg(1, rng.next() as u16); // Pg
                    st.set_preg(2, rng.next() as u16); // Pn
                    batch.push((name.clone(), insn, st));
                }
            }
        }
    }
    run_batch("sve_brk", batch);
}

#[test]
fn diff_sve_brkn() {
    // BRKN: result = Pdm if the last Pg-active element of Pn is set, else 0.
    let mut rng = Rng::new(0x2_C001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for s in 0..2u32 {
        let insn = enc_brkn(s, 1, 2, 0); // Pg=p1, Pn=p2, Pdm=p0
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            st.set_preg(0, rng.next() as u16); // Pdm (source + dest)
            st.set_preg(1, rng.next() as u16); // Pg
            st.set_preg(2, rng.next() as u16); // Pn
            batch.push((format!("brkn s{s}"), insn, st));
        }
    }
    run_batch("sve_brkn", batch);
}

#[test]
fn diff_sve_brkp() {
    // BRKPA / BRKPB propagating partition break (Pm break condition, Pn carry).
    let mut rng = Rng::new(0x2_D001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for b in 0..2u32 {
        for s in 0..2u32 {
            let insn = enc_brkp(b, s, 3, 1, 2, 0); // Pm=p3, Pg=p1, Pn=p2, Pd=p0
            let name = format!("brkp{} s{s}", if b == 0 { "a" } else { "b" });
            for _ in 0..16 {
                let mut st = ArmState::zeroed();
                st.set_preg(1, rng.next() as u16); // Pg
                st.set_preg(2, rng.next() as u16); // Pn
                st.set_preg(3, rng.next() as u16); // Pm
                batch.push((name.clone(), insn, st));
            }
        }
    }
    run_batch("sve_brkp", batch);
}

#[test]
fn diff_sve_unpred() {
    // Element-wise ops: low 128 bits match at any VL; the oracle also pins
    // VL=128 so the full Z register is captured via the aliased V register.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for sz in 0..4u32 {
        cases.push((format!("sve_add sz{sz}"), enc_sve_arith(sz, 0b000000)));
        cases.push((format!("sve_sub sz{sz}"), enc_sve_arith(sz, 0b000001)));
        cases.push((format!("sve_sqadd sz{sz}"), enc_sve_arith(sz, 0b000100)));
        cases.push((format!("sve_uqadd sz{sz}"), enc_sve_arith(sz, 0b000101)));
        cases.push((format!("sve_sqsub sz{sz}"), enc_sve_arith(sz, 0b000110)));
        cases.push((format!("sve_uqsub sz{sz}"), enc_sve_arith(sz, 0b000111)));
    }
    for (opc, name) in [
        (0b00u32, "and"),
        (0b01, "orr"),
        (0b10, "eor"),
        (0b11, "bic"),
    ] {
        cases.push((format!("sve_{name}"), enc_sve_logical(opc)));
    }
    run_family("sve_unpred", cases, 16, 0x1_001F);
}

#[test]
fn diff_sve2_ternary() {
    // SVE2 bitwise ternary: EOR3/BCAX/BSL/BSL1N/BSL2N/NBSL (whole-register).
    let ops: &[(u32, u32, &str)] = &[
        (0b00, 0, "eor3"),
        (0b01, 0, "bcax"),
        (0b00, 1, "bsl"),
        (0b01, 1, "bsl1n"),
        (0b10, 1, "bsl2n"),
        (0b11, 1, "nbsl"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(opc, o2, name) in ops {
        cases.push((name.to_string(), enc_sve2_tern(opc, o2)));
    }
    run_family("sve2_ternary", cases, 16, 0x5_0001);
}

#[test]
fn diff_sve2_addl() {
    // SVE2 integer add/subtract long (S?ADDL/S?SUBL bottom/top, signed/unsigned).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 1..4u32 {
        for s in 0..2u32 {
            for u in 0..2u32 {
                for t in 0..2u32 {
                    cases.push((
                        format!("addl sz{size} s{s} u{u} t{t}"),
                        enc_sve2_addl(size, s, u, t),
                    ));
                    cases.push((
                        format!("addw sz{size} s{s} u{u} t{t}"),
                        enc_sve2_addw(size, s, u, t),
                    ));
                }
                cases.push((format!("abdl sz{size} u{u}"), enc_sve2_abdl(size, u, 0)));
                cases.push((format!("abdl sz{size} u{u} t"), enc_sve2_abdl(size, u, 1)));
            }
        }
    }
    run_family("sve2_addl", cases, 16, 0x5_1001);
}

#[test]
fn diff_sve2_mull() {
    // SVE2 integer multiply long: SMULL/UMULL/SQDMULL (sizes H/S/D) and PMULL
    // (H form only).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 1..4u32 {
        for t in 0..2u32 {
            cases.push((format!("smull sz{size} t{t}"), enc_sve2_mull(size, 1, 0, t)));
            cases.push((format!("umull sz{size} t{t}"), enc_sve2_mull(size, 1, 1, t)));
            cases.push((
                format!("sqdmull sz{size} t{t}"),
                enc_sve2_mull(size, 0, 0, t),
            ));
        }
    }
    for t in 0..2u32 {
        cases.push((format!("pmull t{t}"), enc_sve2_mull(1, 0, 1, t)));
    }
    run_family("sve2_mull", cases, 16, 0x5_2001);
}

#[test]
fn diff_sve2_mlal() {
    // SVE2 multiply-add long (S?MLAL/S?MLSL) and saturating doubling MAC long
    // (SQDMLAL/SQDMLSL).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 1..4u32 {
        for t in 0..2u32 {
            for s in 0..2u32 {
                for u in 0..2u32 {
                    cases.push((
                        format!("mlal sz{size} s{s} u{u} t{t}"),
                        enc_sve2_mlal(size, s, u, t),
                    ));
                }
                cases.push((
                    format!("sqdmlal sz{size} s{s} t{t}"),
                    enc_sve2_sqdmlal(size, s, t),
                ));
            }
        }
    }
    run_family("sve2_mlal", cases, 16, 0x5_3001);
}

#[test]
fn diff_sve2_addhn() {
    // SVE2 add/subtract high narrow (ADDHN/SUBHN/RADDHN/RSUBHN, bottom/top).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 1..4u32 {
        for s in 0..2u32 {
            for r in 0..2u32 {
                for t in 0..2u32 {
                    cases.push((
                        format!("addhn sz{size} s{s} r{r} t{t}"),
                        enc_sve2_addhn(size, s, r, t),
                    ));
                }
            }
        }
    }
    run_family("sve2_addhn", cases, 16, 0x5_4001);
}

#[test]
fn diff_sve2_xtn() {
    // SVE2 saturating extract narrow: SQXTN/UQXTN/SQXTUN (bottom/top), dest B/H/S.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &tsz in &[0b001u32, 0b010, 0b100] {
        for variant in 0..3u32 {
            for t in 0..2u32 {
                cases.push((
                    format!("xtn tsz{tsz} v{variant} t{t}"),
                    enc_sve2_xtn(tsz, variant, t),
                ));
            }
        }
    }
    run_family("sve2_xtn", cases, 16, 0x5_5001);
}

#[test]
fn diff_sve2_cadd() {
    // SVE2 complex integer add (CADD / saturating SQCADD), 90/270 rotation.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 0..4u32 {
        for op in 0..2u32 {
            for rot in 0..2u32 {
                cases.push((
                    format!("cadd sz{size} op{op} rot{rot}"),
                    enc_sve2_cadd(size, op, rot),
                ));
            }
        }
    }
    run_family("sve2_cadd", cases, 16, 0x5_6001);
}

#[test]
fn diff_sve2_cmla() {
    // SVE2 complex integer multiply-add: CMLA and saturating SQRDCMLAH, all four
    // rotations.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 0..4u32 {
        for op in 0..2u32 {
            for rot in 0..4u32 {
                cases.push((
                    format!("cmla sz{size} op{op} rot{rot}"),
                    enc_sve2_cmla(size, op, rot),
                ));
            }
        }
    }
    run_family("sve2_cmla", cases, 16, 0x5_7001);
}

#[test]
fn diff_sve2_shrn() {
    // SVE2 shift right narrow: SHRN/RSHRN, SQSHRUN, SQSHRN, UQSHRN (bottom/top),
    // across destination sizes and shift amounts.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(db, amt) in &[
        (8u32, 1u32),
        (8, 4),
        (8, 8),
        (16, 1),
        (16, 8),
        (16, 16),
        (32, 1),
        (32, 16),
        (32, 32),
    ] {
        let (tsz, imm3) = shrn_tsz_imm(db, amt);
        for op in 0..2u32 {
            for u in 0..2u32 {
                for r in 0..2u32 {
                    for t in 0..2u32 {
                        cases.push((
                            format!("shrn d{db} a{amt} op{op} u{u} r{r} t{t}"),
                            enc_sve2_shrn(tsz, imm3, op, u, r, t),
                        ));
                    }
                }
            }
        }
    }
    run_family("sve2_shrn", cases, 12, 0x5_8001);
}

#[test]
fn diff_sve2_sqdmulh() {
    // SVE2 SQDMULH/SQRDMULH and SQRDMLAH/SQRDMLSH (saturating doubling).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 0..4u32 {
        cases.push((format!("sqdmulh sz{size}"), enc_sve2_sqdmulh(size, 0)));
        cases.push((format!("sqrdmulh sz{size}"), enc_sve2_sqdmulh(size, 1)));
        cases.push((format!("sqrdmlah sz{size}"), enc_sve2_sqrdmlah(size, 0)));
        cases.push((format!("sqrdmlsh sz{size}"), enc_sve2_sqrdmlah(size, 1)));
    }
    run_family("sve2_sqdmulh", cases, 16, 0x5_9001);
}

#[test]
fn diff_sve2_sqrdmlsh_tie() {
    // Rounding-tie edge: every Zn/Zm .h lane = 0x0080, so the product is exactly
    // 2^14. SQRDMLSH negates the product before adding the rounding bias, giving
    // a doubling-high of 0 (result == Zda); negating the rounded SQRDMLAH result
    // would instead give -1 per lane.
    let mut rng = Rng::new(0x5_9101);
    let mut lanes = 0u128;
    for l in 0..8 {
        lanes |= 0x0080u128 << (l * 16);
    }
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for s in 0..2u32 {
        let insn = enc_sve2_sqrdmlah(1, s); // .h
        let mut st = ArmState::zeroed();
        st.set_vreg(1, lanes as u64, (lanes >> 64) as u64);
        st.set_vreg(2, lanes as u64, (lanes >> 64) as u64);
        st.set_vreg(0, rng.next(), rng.next()); // Zda
        batch.push((format!("sqrdml_tie s{s}"), insn, st));
    }
    run_batch("sve2_sqrdmlsh_tie", batch);
}

/// (tsz, imm3) for a shift-left-long with source width `src_bits` and shift
/// `amount` (0..src_bits): tsz:imm3 = src_bits + amount.
fn shll_tsz_imm(src_bits: u32, amount: u32) -> (u32, u32) {
    let tszimm = src_bits + amount;
    ((tszimm >> 3) & 0x7, tszimm & 0x7)
}

#[test]
fn diff_sve2_shll() {
    // SVE2 shift left long (SSHLL/USHLL, bottom/top), across source sizes and
    // shift amounts.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(sb, amt) in &[
        (8u32, 0u32),
        (8, 3),
        (8, 7),
        (16, 0),
        (16, 8),
        (16, 15),
        (32, 0),
        (32, 16),
        (32, 31),
    ] {
        let (tsz, imm3) = shll_tsz_imm(sb, amt);
        for u in 0..2u32 {
            for t in 0..2u32 {
                cases.push((
                    format!("shll s{sb} a{amt} u{u} t{t}"),
                    enc_sve2_shll(tsz, imm3, u, t),
                ));
            }
        }
    }
    run_family("sve2_shll", cases, 16, 0x5_A001);
}

#[test]
fn diff_sve2_bperm() {
    // SVE2 bit permute: BEXT/BDEP/BGRP, all element sizes.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 0..4u32 {
        for opc in 0..3u32 {
            cases.push((
                format!("bperm sz{size} opc{opc}"),
                enc_sve2_bperm(size, opc),
            ));
        }
    }
    run_family("sve2_bperm", cases, 20, 0x5_B001);
}

#[test]
fn diff_sve2_pairwise() {
    // SVE2 predicated integer pairwise: ADDP/SMAXP/UMAXP/SMINP/UMINP.
    let ops: &[(u32, u32, &str)] = &[
        (0b00, 1, "addp"),
        (0b10, 0, "smaxp"),
        (0b10, 1, "umaxp"),
        (0b11, 0, "sminp"),
        (0b11, 1, "uminp"),
    ];
    let mut rng = Rng::new(0x5_C001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 0..4u32 {
        for &(opc, u, name) in ops {
            let insn = enc_sve2_pairwise(size, opc, u);
            for _ in 0..16 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next());
                st.set_vreg(1, rng.next(), rng.next());
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} sz{size}"), insn, st));
            }
        }
    }
    run_batch("sve2_pairwise", batch);
}

#[test]
fn diff_sve2_abal() {
    // SVE2 absolute-difference accumulate long (SABAL/UABAL, bottom/top).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 1..4u32 {
        for u in 0..2u32 {
            for t in 0..2u32 {
                cases.push((
                    format!("abal sz{size} u{u} t{t}"),
                    enc_sve2_abal(size, u, t),
                ));
            }
        }
    }
    run_family("sve2_abal", cases, 16, 0x5_D001);
}

#[test]
fn diff_sve2_adalp() {
    // SVE2 add long pairwise accumulate (SADALP/UADALP), predicated.
    let mut rng = Rng::new(0x5_E001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 1..4u32 {
        for u in 0..2u32 {
            let insn = enc_sve2_adalp(size, u);
            for _ in 0..16 {
                let mut st = ArmState::zeroed();
                st.set_vreg(0, rng.next(), rng.next());
                st.set_vreg(1, rng.next(), rng.next());
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("adalp sz{size} u{u}"), insn, st));
            }
        }
    }
    run_batch("sve2_adalp", batch);
}

#[test]
fn diff_sve2_ssra() {
    // SVE2 shift-right-accumulate (SSRA/USRA/SRSRA/URSRA) and shift-insert
    // (SRI/SLI), across element sizes and shift amounts.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &eb in &[8u32, 16, 32, 64] {
        for &amt in &[1u32, eb / 2, eb] {
            let (tsz, imm3) = ssra_tsz_imm(eb, amt);
            for r in 0..2u32 {
                for u in 0..2u32 {
                    cases.push((
                        format!("ssra e{eb} a{amt} r{r} u{u}"),
                        enc_sve2_ssra(tsz, imm3, r, u),
                    ));
                }
            }
            cases.push((format!("sri e{eb} a{amt}"), enc_sve2_sri(tsz, imm3, 0)));
        }
        for &amt in &[0u32, eb / 2, eb - 1] {
            let (tsz, imm3) = sli_tsz_imm(eb, amt);
            cases.push((format!("sli e{eb} a{amt}"), enc_sve2_sri(tsz, imm3, 1)));
        }
    }
    run_family("sve2_ssra", cases, 12, 0x5_F001);
}

#[test]
fn diff_sve2_fpairwise() {
    // SVE2 FP pairwise: FADDP/FMAXNMP/FMINNMP/FMAXP/FMINP (predicated, merging).
    let ops: &[(u32, &str)] = &[
        (0b000, "faddp"),
        (0b100, "fmaxnmp"),
        (0b101, "fminnmp"),
        (0b110, "fmaxp"),
        (0b111, "fminp"),
    ];
    let mut rng = Rng::new(0x6_0001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for size in 1..4u32 {
        let eb = 8 << size; // element bits: 16 / 32 / 64
        let lanes = (128 / eb) as usize;
        for &(opc, name) in ops {
            let insn = enc_sve2_fpairwise(size, opc);
            for _ in 0..16 {
                let mut st = ArmState::zeroed();
                let (l0, h0) = fill_finite_fp(&mut rng, eb, lanes);
                st.set_vreg(0, l0, h0);
                let (l1, h1) = fill_finite_fp(&mut rng, eb, lanes);
                st.set_vreg(1, l1, h1);
                st.set_preg(0, rng.next() as u16);
                batch.push((format!("{name} sz{size}"), insn, st));
            }
        }
    }
    run_batch("sve2_fpairwise", batch);
}

#[test]
fn diff_sve_mul() {
    // SVE2 unpredicated MUL/SMULH/UMULH. SMULH/UMULH return the high half of
    // the double-width product; MUL returns the low half. Element-wise, so the
    // low 128 bits match at any VL (oracle pins VL=128).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for sz in 0..4u32 {
        cases.push((format!("sve_mul sz{sz}"), enc_sve_mul(sz, 0b00)));
        cases.push((format!("sve_smulh sz{sz}"), enc_sve_mul(sz, 0b10)));
        cases.push((format!("sve_umulh sz{sz}"), enc_sve_mul(sz, 0b11)));
    }
    // PMUL (SVE2 carry-less polynomial multiply) is byte-elements only.
    cases.push(("sve_pmul".to_string(), enc_sve_mul(0, 0b01)));
    run_family("sve_mul", cases, 16, 0x2_4001);
}

/// A finite bf16 value with a moderate exponent (so f64 dot-product sums stay
/// exact and the round-to-odd model matches hardware).
fn rand_bf16(rng: &mut Rng) -> u16 {
    let sign = (rng.next() & 1) as u16;
    let exp = (rng.next() % 11 + 122) as u16; // unbiased -5..5
    let frac = (rng.next() as u16) & 0x7F;
    (sign << 15) | (exp << 7) | frac
}

/// Fill a 128-bit vector with 8 finite bf16 lanes.
fn bf16_vec(rng: &mut Rng) -> (u64, u64) {
    let mut v: u128 = 0;
    for i in 0..8 {
        v |= (rand_bf16(rng) as u128) << (i * 16);
    }
    (v as u64, (v >> 64) as u64)
}

/// Fill a 128-bit vector with 4 finite f32 accumulator lanes.
fn f32_acc_vec(rng: &mut Rng) -> (u64, u64) {
    let mut v: u128 = 0;
    for i in 0..4 {
        let val = (((rng.next() % 4000) as f32) - 2000.0) / 200.0; // +/-10
        v |= (val.to_bits() as u128) << (i * 32);
    }
    (v as u64, (v >> 64) as u64)
}

/// BFMLALB/T (vector): `0 Q 1 01110 11 0 Rm 111111 Rn Rd`. Q=B(0)/T(1).
fn enc_bfmlal(q: u32) -> u32 {
    (q << 30)
        | (1 << 29)
        | (0b01110 << 24)
        | (0b11 << 22)
        | (RM << 16)
        | (0b111111 << 10)
        | (RN << 5)
        | RD
}

/// BFMLALB/T (by element): `0 Q 0 01111 11 L M Rm 1111 H 0 Rn Rd`. index=H:L:M.
fn enc_bfmlal_idx(q: u32, index: u32) -> u32 {
    let h = (index >> 2) & 1;
    let l = (index >> 1) & 1;
    let m = index & 1;
    (q << 30)
        | (0b01111 << 24)
        | (0b11 << 22)
        | (l << 21)
        | (m << 20)
        | (RM << 16)
        | (0b1111 << 12)
        | (h << 11)
        | (RN << 5)
        | RD
}

/// BFDOT (vector): `0 Q 1 01110 01 0 Rm 111111 Rn Rd`. Q=datasize.
fn enc_bfdot(q: u32) -> u32 {
    (q << 30)
        | (1 << 29)
        | (0b01110 << 24)
        | (0b01 << 22)
        | (RM << 16)
        | (0b111111 << 10)
        | (RN << 5)
        | RD
}

/// BFDOT (by element): `0 Q 0 01111 01 L M Rm 1111 H 0 Rn Rd`. index=H:L.
fn enc_bfdot_idx(q: u32, index: u32) -> u32 {
    let h = (index >> 1) & 1;
    let l = index & 1;
    (q << 30)
        | (0b01111 << 24)
        | (0b01 << 22)
        | (l << 21)
        | (RM << 16)
        | (0b1111 << 12)
        | (h << 11)
        | (RN << 5)
        | RD
}

/// BFMMLA: `0 1 1 01110 01 0 Rm 111011 Rn Rd`.
fn enc_bfmmla() -> u32 {
    (1 << 30)
        | (1 << 29)
        | (0b01110 << 24)
        | (0b01 << 22)
        | (RM << 16)
        | (0b111011 << 10)
        | (RN << 5)
        | RD
}

/// A varied f32 bit pattern for BFCVT testing: finite normals, tie cases,
/// overflow, signed zero, and (occasionally) inf. NaN is excluded to keep
/// payload-propagation out of the comparison.
fn rand_f32_for_bfcvt(rng: &mut Rng) -> u32 {
    match rng.next() % 12 {
        0 => 0x3F80_8000,                     // tie, bf16 lsb 0 -> rounds down
        1 => 0x3F81_8000,                     // tie, bf16 lsb 1 -> rounds up
        2 => 0x7F7F_FFFF,                     // max f32 -> overflow to bf16 inf
        3 => 0x0000_0000,                     // +0
        4 => 0x8000_0000,                     // -0
        5 => 0x7F80_0000,                     // +inf
        6 => 0xFF80_0000,                     // -inf
        7 => rng.next() as u32 & 0x0000_FFFF, // tiny / subnormal-ish low bits
        _ => {
            let sign = (rng.next() & 1) as u32;
            let exp = (rng.next() % 60 + 100) as u32; // finite normal exponents
            let mant = rng.next() as u32 & 0x7F_FFFF;
            (sign << 31) | (exp << 23) | mant
        }
    }
}

#[test]
fn diff_simd_bfcvt() {
    let bfcvt = 0x1E63_4000 | (RN << 5) | RD; // BFCVT Hd, Sn
    let bfcvtn = 0x0EA1_6800 | (RN << 5) | RD; // BFCVTN Vd.4H, Vn.4S
    let bfcvtn2 = 0x4EA1_6800 | (RN << 5) | RD; // BFCVTN2 Vd.8H, Vn.4S
    let mut rng = Rng::new(0x1_001E);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for _ in 0..200 {
        // BFCVT scalar: Sn in v1 low 32 bits.
        let mut st = ArmState::zeroed();
        st.set_vreg(1, rand_f32_for_bfcvt(&mut rng) as u64, 0);
        batch.push(("bfcvt".to_string(), bfcvt, st));

        // BFCVTN / BFCVTN2: 4 f32 lanes in Vn; seed Vd to check half handling.
        for &insn in &[bfcvtn, bfcvtn2] {
            let mut st = ArmState::zeroed();
            let lo = (rand_f32_for_bfcvt(&mut rng) as u64)
                | ((rand_f32_for_bfcvt(&mut rng) as u64) << 32);
            let hi = (rand_f32_for_bfcvt(&mut rng) as u64)
                | ((rand_f32_for_bfcvt(&mut rng) as u64) << 32);
            st.set_vreg(1, lo, hi);
            st.set_vreg(0, rng.next(), rng.next()); // Vd preset (BFCVTN2 preserves low half)
            batch.push(("bfcvtn".to_string(), insn, st));
        }
    }
    run_batch("simd_bfcvt", batch);
}

#[test]
fn diff_simd_bf16() {
    let mut rng = Rng::new(0x1_001D);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    // (label, insn, vector?) — all read v0 (f32 acc), v1/v2 (bf16).
    let mut cases: Vec<(String, u32)> = Vec::new();
    for q in 0..2u32 {
        cases.push((format!("bfmlal q{q}"), enc_bfmlal(q)));
        cases.push((format!("bfdot q{q}"), enc_bfdot(q)));
        for index in 0..4u32 {
            cases.push((format!("bfdot_idx q{q} i{index}"), enc_bfdot_idx(q, index)));
        }
        for index in 0..8u32 {
            cases.push((
                format!("bfmlal_idx q{q} i{index}"),
                enc_bfmlal_idx(q, index),
            ));
        }
    }
    cases.push(("bfmmla".to_string(), enc_bfmmla()));
    for (label, insn) in &cases {
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            let (a0, b0) = f32_acc_vec(&mut rng);
            let (a1, b1) = bf16_vec(&mut rng);
            let (a2, b2) = bf16_vec(&mut rng);
            st.set_vreg(0, a0, b0);
            st.set_vreg(1, a1, b1);
            st.set_vreg(2, a2, b2);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("simd_bf16", batch);
}

/// USDOT (vector): `0 Q 0 01110 10 0 Rm 100111 Rn Rd`. Rd=v0, Rn=v1, Rm=v2.
fn enc_usdot(q: u32) -> u32 {
    (q << 30) | (0b01110 << 24) | (0b10 << 22) | (RM << 16) | (0b100111 << 10) | (RN << 5) | RD
}

/// USDOT/SUDOT (by element): `0 Q 0 01111 US 0 L M Rm 1111 H 0 Rn Rd`. Rm=v2.
fn enc_usdot_idx(q: u32, us: u32, index: u32) -> u32 {
    let h = (index >> 1) & 1;
    let l = index & 1;
    (q << 30)
        | (0b01111 << 24)
        | (us << 23)
        | (l << 21)
        | (RM << 16)
        | (0b1111 << 12)
        | (h << 11)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_usdot() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for q in 0..2u32 {
        cases.push((format!("usdot q{q}"), enc_usdot(q)));
        for index in 0..4u32 {
            cases.push((
                format!("usdot_idx q{q} i{index}"),
                enc_usdot_idx(q, 1, index),
            ));
            cases.push((
                format!("sudot_idx q{q} i{index}"),
                enc_usdot_idx(q, 0, index),
            ));
        }
    }
    run_family("simd_usdot", cases, 32, 0x1_001C);
}

#[test]
fn diff_simd_dot_indexed() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for q in 0..2u32 {
        for index in 0..4u32 {
            cases.push((format!("sdot q{q} i{index}"), enc_dot_idx(q, 0, index)));
            cases.push((format!("udot q{q} i{index}"), enc_dot_idx(q, 1, index)));
        }
    }
    run_family("simd_dot_indexed", cases, 24, 0x1_0019);
}

#[test]
fn diff_crypto_sm4() {
    let cases: Vec<(String, u32)> = vec![
        ("sm4e".to_string(), enc_sm4e()),
        ("sm4ekey".to_string(), enc_sm4ekey()),
    ];
    run_family("crypto_sm4", cases, 40, 0x1_0016);
}

#[test]
fn diff_crypto_sm3() {
    // SM3SS1 (4-reg): 11001110 010 Rm 0 Ra Rn Rd. Ra=v3.
    let sm3ss1 = 0xCE00_0000 | (0b010 << 21) | (RM << 16) | (3 << 10) | (RN << 5) | RD;
    // SM3PARTW1/2: 11001110 011 Rm 11000{0,1} Rn Rd.
    let partw1 = 0xCE00_0000 | (0b011 << 21) | (RM << 16) | (0b110000 << 10) | (RN << 5) | RD;
    let partw2 = 0xCE00_0000 | (0b011 << 21) | (RM << 16) | (0b110001 << 10) | (RN << 5) | RD;
    let mut cases: Vec<(String, u32)> = vec![
        ("sm3ss1".to_string(), sm3ss1),
        ("sm3partw1".to_string(), partw1),
        ("sm3partw2".to_string(), partw2),
    ];
    // SM3TT{1,2}{A,B}: 11001110 010 Rm 10 imm2 sel Rn Rd (sel=bits[11:10]).
    for (sel, nm) in [
        (0b00u32, "tt1a"),
        (0b01, "tt1b"),
        (0b10, "tt2a"),
        (0b11, "tt2b"),
    ] {
        for i in 0..4u32 {
            let insn = 0xCE00_0000
                | (0b010 << 21)
                | (RM << 16)
                | (0b10 << 14)
                | (i << 12)
                | (sel << 10)
                | (RN << 5)
                | RD;
            cases.push((format!("sm3{nm} i{i}"), insn));
        }
    }
    run_family("crypto_sm3", cases, 40, 0x1_0017);
}

#[test]
fn diff_excl_load() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 0..4u32 {
        for o0 in 0..2 {
            cases.push((format!("ldxr sz{size} o0{o0}"), enc_ldxr(size, o0)));
        }
    }
    let mut rng = Rng::new(0x1_0007);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..8 {
            batch.push((label.clone(), *insn, mem_input(&mut rng)));
        }
    }
    run_batch("excl_load", batch);
}

#[test]
fn diff_excl_pair() {
    let mut cases: Vec<(String, u32, u32)> = Vec::new();
    for size in 0..4u32 {
        for o0 in 0..2 {
            cases.push((
                format!("ldxr_stxr sz{size} o0{o0}"),
                enc_ldxr(size, o0),
                enc_stxr(size, o0),
            ));
        }
    }
    let mut rng = Rng::new(0x1_0008);
    let mut batch: Vec<(String, u32, u32, ArmState)> = Vec::new();
    for (label, ldxr, stxr) in &cases {
        for _ in 0..8 {
            batch.push((label.clone(), *ldxr, *stxr, mem_input(&mut rng)));
        }
    }
    run_batch_pair("excl_pair", batch);
}

/// CAS: `size 0010001 L 1 Rs o0 11111 Rn Rt`. Rs=x2 (compare/old), Rn=x1, Rt=x0 (new).
fn enc_cas(size: u32, l: u32, o0: u32) -> u32 {
    (size << 30)
        | (0b001000 << 24)
        | (1 << 23)
        | (l << 22)
        | (1 << 21)
        | (2 << 16)
        | (o0 << 15)
        | (0b11111 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_mem_cas() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for size in 0..4u32 {
        for l in 0..2 {
            for o0 in 0..2 {
                cases.push((format!("cas sz{size} l{l} o0{o0}"), enc_cas(size, l, o0)));
            }
        }
    }
    let mut rng = Rng::new(0x1_0006);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for k in 0..12 {
            let mut st = mem_input(&mut rng);
            // Half the cases: make x2 (the comparison value) equal the memory at
            // SCRATCH_BASE so the swap succeeds; otherwise it (usually) fails.
            if k % 2 == 0 {
                st.x[2] = st.scratch[8]; // SCRATCH_BASE = SCRATCH_ADDR + 64 -> scratch[8]
            }
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("mem_cas", batch);
}

/// LDXP: `1 sz 001000 0 1 1 11111 o0 Rt2 Rn Rt`. Rt=x4, Rt2=x5, Rn=x1.
/// sz64 selects 64-bit (size=11) vs 32-bit (size=10) element pair.
fn enc_ldxp(sz64: bool, o0: u32) -> u32 {
    let size = if sz64 { 3 } else { 2 };
    (size << 30)
        | (0b001000 << 24)
        | (1 << 22)
        | (1 << 21)
        | (0b11111 << 16)
        | (o0 << 15)
        | (5 << 10)
        | (RN << 5)
        | 4
}

/// STXP: `1 sz 001000 0 0 1 Rs o0 Rt2 Rn Rt`. Rs=x6 (status), Rt=x4, Rt2=x5, Rn=x1.
fn enc_stxp(sz64: bool, o0: u32) -> u32 {
    let size = if sz64 { 3 } else { 2 };
    (size << 30) | (0b001000 << 24) | (1 << 21) | (6 << 16) | (o0 << 15) | (5 << 10) | (RN << 5) | 4
}

#[test]
fn diff_excl_ldxp() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &sz64 in &[false, true] {
        for o0 in 0..2 {
            cases.push((format!("ldxp sz64{sz64} o0{o0}"), enc_ldxp(sz64, o0)));
        }
    }
    let mut rng = Rng::new(0x1_000F);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..10 {
            batch.push((label.clone(), *insn, mem_input(&mut rng)));
        }
    }
    run_batch("excl_ldxp", batch);
}

#[test]
fn diff_excl_stxp() {
    // LDXP then STXP (round-trip): the monitor set by LDXP lets STXP succeed.
    let mut cases: Vec<(String, u32, u32)> = Vec::new();
    for &sz64 in &[false, true] {
        for o0 in 0..2 {
            cases.push((
                format!("ldxp_stxp sz64{sz64} o0{o0}"),
                enc_ldxp(sz64, o0),
                enc_stxp(sz64, o0),
            ));
        }
    }
    let mut rng = Rng::new(0x1_0010);
    let mut batch: Vec<(String, u32, u32, ArmState)> = Vec::new();
    for (label, ldxp, stxp) in &cases {
        for _ in 0..10 {
            batch.push((label.clone(), *ldxp, *stxp, mem_input(&mut rng)));
        }
    }
    run_batch_pair("excl_stxp", batch);
}

/// CASP: `0 sz 001000 0 L 1 Rs o0 11111 Rn Rt`. Rs=x2:x3 (compare/old),
/// Rt=x4:x5 (new), Rn=x1. sz selects 32-bit (0) or 64-bit (1) element pair.
fn enc_casp(sz: u32, l: u32, o0: u32) -> u32 {
    (sz << 30)
        | (0b001000 << 24)
        | (l << 22)
        | (1 << 21)
        | (2 << 16)
        | (o0 << 15)
        | (0b11111 << 10)
        | (RN << 5)
        | 4
}

#[test]
fn diff_mem_casp() {
    let mut cases: Vec<(String, u32, u32)> = Vec::new();
    for sz in 0..2u32 {
        for l in 0..2 {
            for o0 in 0..2 {
                cases.push((format!("casp sz{sz} l{l} o0{o0}"), enc_casp(sz, l, o0), sz));
            }
        }
    }
    let mut rng = Rng::new(0x1_000E);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn, sz) in &cases {
        for k in 0..12 {
            let mut st = mem_input(&mut rng);
            // Half the cases: make the compare pair (x2:x3) equal the memory pair
            // at SCRATCH_BASE (scratch[8]/scratch[9]) so the swap succeeds.
            if k % 2 == 0 {
                if *sz == 0 {
                    st.x[2] = st.scratch[8] & 0xFFFF_FFFF;
                    st.x[3] = st.scratch[8] >> 32;
                } else {
                    st.x[2] = st.scratch[8];
                    st.x[3] = st.scratch[9];
                }
            }
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("mem_casp", batch);
}

/// Atomic memory op: `size 111 0 00 A R 1 Rs o3 opc 00 Rn Rt`. Rs=x2, Rn=x1, Rt=x0.
fn enc_atomic(size: u32, a: u32, r: u32, o3: u32, opc: u32) -> u32 {
    (size << 30)
        | (0b111 << 27)
        | (a << 23)
        | (r << 22)
        | (1 << 21)
        | (2 << 16)
        | (o3 << 15)
        | (opc << 12)
        | (RN << 5)
        | RD
}

#[test]
fn diff_mem_atomic() {
    let ops: &[(u32, u32, &str)] = &[
        (0, 0b000, "ldadd"),
        (0, 0b001, "ldclr"),
        (0, 0b010, "ldeor"),
        (0, 0b011, "ldset"),
        (0, 0b100, "ldsmax"),
        (0, 0b101, "ldsmin"),
        (0, 0b110, "ldumax"),
        (0, 0b111, "ldumin"),
        (1, 0b000, "swp"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(o3, opc, name) in ops {
        for size in 0..4u32 {
            for &(a, r) in &[(0u32, 0u32), (1, 1)] {
                cases.push((
                    format!("{name} sz{size} a{a}r{r}"),
                    enc_atomic(size, a, r, o3, opc),
                ));
            }
        }
    }
    let mut rng = Rng::new(0x1_0004);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..8 {
            batch.push((label.clone(), *insn, mem_input(&mut rng)));
        }
    }
    run_batch("mem_atomic", batch);
}

/// AdvSIMD load/store multiple structures: `0 Q 0011 0 0 post L rm opcode size Rn Rt`.
fn enc_ldst_struct(q: u32, post: u32, l: u32, rm: u32, opcode: u32, size: u32) -> u32 {
    (q << 30)
        | (0b001100 << 24)
        | (post << 23)
        | (l << 22)
        | (rm << 16)
        | (opcode << 12)
        | (size << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_mem_ldst_struct() {
    // (opcode, name) -- LD1 x1/x2/x3/x4, LD2, LD3, LD4.
    let ops: &[(u32, &str)] = &[
        (0b0111, "ld1x1"),
        (0b1010, "ld1x2"),
        (0b0110, "ld1x3"),
        (0b0010, "ld1x4"),
        (0b1000, "ld2"),
        (0b0100, "ld3"),
        (0b0000, "ld4"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(opcode, name) in ops {
        for size in 0..4u32 {
            for q in 0..2 {
                for l in 0..2 {
                    let op = if l == 1 {
                        name.to_string()
                    } else {
                        name.replace("ld", "st")
                    };
                    // no-offset
                    cases.push((
                        format!("{op} sz{size} q{q} noff"),
                        enc_ldst_struct(q, 0, l, 0, opcode, size),
                    ));
                    // post-index, immediate increment (Rm == 31)
                    cases.push((
                        format!("{op} sz{size} q{q} post"),
                        enc_ldst_struct(q, 1, l, 31, opcode, size),
                    ));
                }
            }
        }
    }
    let mut rng = Rng::new(0x1_0003);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..6 {
            batch.push((label.clone(), *insn, mem_input(&mut rng)));
        }
    }
    run_batch("mem_ldst_struct", batch);
}

/// Load/store pair: `opc 101 V 0 mode L imm7 Rt2 Rn Rt`.
fn enc_ldp_regs(opc: u32, v: u32, mode: u32, l: u32, imm7: u32, rt: u32, rt2: u32, rn: u32) -> u32 {
    (opc << 30)
        | (0b101 << 27)
        | (v << 26)
        | (mode << 23)
        | (l << 22)
        | ((imm7 & 0x7F) << 15)
        | ((rt2 & 0x1F) << 10)
        | ((rn & 0x1F) << 5)
        | (rt & 0x1F)
}

/// Load/store pair with the standard test registers: Rt=x0, Rt2=x2, Rn=x1.
fn enc_ldp(opc: u32, v: u32, mode: u32, l: u32, imm7: u32) -> u32 {
    enc_ldp_regs(opc, v, mode, l, imm7, RD, RM, RN)
}

#[test]
fn diff_mem_ldp_stp() {
    // (opc, V, name) for GPR (32/LDPSW/64) and SIMD (S/D/Q).
    let kinds: &[(u32, u32, bool, &str)] = &[
        (0b00, 0, false, "stp_w"),
        (0b10, 0, false, "stp_x"),
        (0b00, 1, false, "stp_s"),
        (0b01, 1, false, "stp_d"),
        (0b10, 1, false, "stp_q"),
        (0b01, 0, true, "ldpsw"), // load-only
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(opc, v, load_only, name) in kinds {
        // modes: 10=signed offset, 01=post-index, 11=pre-index
        for &mode in &[0b10u32, 0b01, 0b11] {
            for &l in if load_only {
                &[1u32][..]
            } else {
                &[0u32, 1][..]
            } {
                for imm7 in 0..3u32 {
                    let nm = if l == 1 && !load_only {
                        name.replace("stp", "ldp")
                    } else {
                        name.to_string()
                    };
                    cases.push((
                        format!("{nm} m{mode} #{imm7}"),
                        enc_ldp(opc, v, mode, l, imm7),
                    ));
                }
            }
        }
    }
    let no_allocate_kinds: &[(u32, u32, &str)] = &[
        (0b00, 0, "stnp_w"),
        (0b10, 0, "stnp_x"),
        (0b00, 1, "ldnp_w"),
        (0b10, 1, "ldnp_x"),
    ];
    for &(opc, l, name) in no_allocate_kinds {
        for imm7 in 0..3u32 {
            cases.push((format!("{name} #{imm7}"), enc_ldp(opc, 0, 0b00, l, imm7)));
        }
    }
    for imm7 in 0..3u32 {
        cases.push((
            format!("ldpsw_m0_undef #{imm7}"),
            enc_ldp(0b01, 0, 0b00, 1, imm7),
        ));
    }
    let mut rng = Rng::new(0x1_0002);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..8 {
            batch.push((label.clone(), *insn, mem_input(&mut rng)));
        }
    }
    run_batch("mem_ldp_stp", batch);
}

/// Load/store register, unsigned immediate offset:
/// `size 111 V 01 opc imm12 Rn Rt`. Rn=base (x1), Rt=Rd (x0/v0).
fn enc_ldst_uimm(size: u32, v: u32, opc: u32, imm12: u32) -> u32 {
    (size << 30)
        | (0b111 << 27)
        | (v << 26)
        | (0b01 << 24)
        | (opc << 22)
        | (imm12 << 10)
        | (RN << 5)
        | RD
}

/// Load/store register, signed immediate offset:
/// `size 111 V 00 opc 0 imm9 mode Rn Rt`. Rn=base (x1), Rt=Rd (x0/v0).
fn enc_ldst_simm(size: u32, v: u32, opc: u32, mode: u32, imm9: i32) -> u32 {
    (size << 30)
        | (0b111 << 27)
        | (v << 26)
        | (opc << 22)
        | (((imm9 as u32) & 0x1FF) << 12)
        | (mode << 10)
        | (RN << 5)
        | RD
}

/// Load/store register offset: `size 111000 opc 1 Rm option S 10 Rn Rt`.
fn enc_ldst_reg(size: u32, opc: u32, rm: u32, option: u32, s: u32) -> u32 {
    (size << 30)
        | (0b111 << 27)
        | (opc << 22)
        | (1 << 21)
        | (rm << 16)
        | (option << 13)
        | (s << 12)
        | (0b10 << 10)
        | (RN << 5)
        | RD
}

/// PRFM register offset: `11 111000 0 0 1 Rm opc=10 option S 10 Rn Rt`.
fn enc_prfm_reg(rt: u32, rm: u32, option: u32, s: u32) -> u32 {
    (0b11 << 30)
        | (0b111 << 27)
        | (0b10 << 22)
        | (1 << 21)
        | (rm << 16)
        | (option << 13)
        | (s << 12)
        | (0b10 << 10)
        | (RN << 5)
        | (rt & 0x1f)
}

/// Build a memory-test input: base register x1 -> SCRATCH_BASE, random scratch
/// and operand registers.
fn mem_input(rng: &mut Rng) -> ArmState {
    let mut st = gen_input(rng);
    st.x[1] = SCRATCH_BASE; // Rn base pointer
    for w in st.scratch.iter_mut() {
        *w = rng.interesting();
    }
    st
}

#[test]
fn diff_mem_ldst_imm() {
    // (size, V, opc, name)
    let ops: &[(u32, u32, u32, &str)] = &[
        (3, 0, 0, "str_x"),
        (3, 0, 1, "ldr_x"),
        (2, 0, 0, "str_w"),
        (2, 0, 1, "ldr_w"),
        (0, 1, 0, "str_b"),
        (0, 1, 1, "ldr_b"),
        (1, 1, 0, "str_h"),
        (1, 1, 1, "ldr_h"),
        (2, 1, 0, "str_s"),
        (2, 1, 1, "ldr_s"),
        (3, 1, 0, "str_d"),
        (3, 1, 1, "ldr_d"),
        (0, 1, 2, "str_q"),
        (0, 1, 3, "ldr_q"),
        // sign-extending loads (GPR): opc=10 -> LDRSx to X, opc=11 -> to W
        (0, 0, 2, "ldrsb_x"),
        (1, 0, 2, "ldrsh_x"),
        (2, 0, 2, "ldrsw_x"),
        (0, 0, 1, "ldrb_w"),
        (1, 0, 1, "ldrh_w"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(size, v, opc, name) in ops {
        for imm12 in 0..4u32 {
            cases.push((
                format!("{name} #{imm12}"),
                enc_ldst_uimm(size, v, opc, imm12),
            ));
        }
    }
    // Custom batch with memory inputs.
    let mut rng = Rng::new(0x1_0001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..8 {
            batch.push((label.clone(), *insn, mem_input(&mut rng)));
        }
    }
    run_batch("mem_ldst_imm", batch);
}

#[test]
fn diff_mem_ldst_reg_offset() {
    let cases: Vec<(String, u32, u64)> = vec![
        (
            "ldr_x_reg_uxtw_lsl3".into(),
            enc_ldst_reg(3, 1, RM, 0b010, 1),
            8,
        ),
        (
            "str_x_reg_sxtx_neg8".into(),
            enc_ldst_reg(3, 0, RM, 0b111, 0),
            (-8i64) as u64,
        ),
        (
            "ldr_w_reg_uxtw".into(),
            enc_ldst_reg(2, 1, RM, 0b010, 0),
            12,
        ),
        (
            "ldrsb_w_reg_sxtw_neg8".into(),
            enc_ldst_reg(0, 3, RM, 0b110, 0),
            (-8i64) as u64,
        ),
        (
            "ldrsh_x_reg_uxtw_lsl1".into(),
            enc_ldst_reg(1, 2, RM, 0b010, 1),
            4,
        ),
        ("strb_reg_uxtw".into(), enc_ldst_reg(0, 0, RM, 0b010, 0), 15),
        (
            "ldr_x_reg_bad_extend_undef".into(),
            enc_ldst_reg(3, 1, RM, 0b000, 0),
            8,
        ),
    ];
    let mut rng = Rng::new(0x1_0021);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn, rm_value) in &cases {
        for _ in 0..6 {
            let mut st = mem_input(&mut rng);
            st.x[0] = 0x1122_3344_5566_7788;
            st.x[2] = *rm_value;
            st.scratch[7] = 0x0000_0000_0000_0080;
            st.scratch[8] = 0x1111_2222_3333_4444;
            st.scratch[9] = 0x5555_6666_7777_8888;
            st.scratch[10] = 0x9999_aaaa_bbbb_cccc;
            st.scratch[16] = 0xdead_beef_cafe_f00d;
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("mem_ldst_reg_offset", batch);
}

#[test]
fn diff_mem_ldst_unprivileged() {
    let cases: Vec<(String, u32)> = vec![
        ("ldtr_x_pos8".into(), enc_ldst_simm(3, 0, 1, 0b10, 8)),
        ("sttr_x_neg8".into(), enc_ldst_simm(3, 0, 0, 0b10, -8)),
        ("ldtr_w_pos8".into(), enc_ldst_simm(2, 0, 1, 0b10, 8)),
        ("sttrb_pos7".into(), enc_ldst_simm(0, 0, 0, 0b10, 7)),
        ("ldtrsb_w_neg8".into(), enc_ldst_simm(0, 0, 3, 0b10, -8)),
        ("ldtrsh_x_pos8".into(), enc_ldst_simm(1, 0, 2, 0b10, 8)),
        ("ldtrsw_x_pos8".into(), enc_ldst_simm(2, 0, 2, 0b10, 8)),
        (
            "ldtrsw_w_unpriv_undef".into(),
            enc_ldst_simm(2, 0, 3, 0b10, 0),
        ),
        ("prfm_unpriv_undef".into(), enc_ldst_simm(3, 0, 2, 0b10, 0)),
        (
            "size11_opc11_unpriv_undef".into(),
            enc_ldst_simm(3, 0, 3, 0b10, 0),
        ),
    ];
    let mut rng = Rng::new(0x1_0022);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..6 {
            let mut st = mem_input(&mut rng);
            st.x[0] = 0x1122_3344_5566_7788;
            st.scratch[7] = 0x0000_0000_0000_0080;
            st.scratch[8] = 0x1111_2222_3333_4444;
            st.scratch[9] = 0x5555_6666_7777_8888;
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("mem_ldst_unprivileged", batch);
}

#[test]
fn diff_mem_prfm() {
    let cases: Vec<(String, u32)> = vec![
        ("prfm_uimm_0".into(), enc_ldst_uimm(3, 0, 2, 0)),
        ("prfm_uimm_3".into(), enc_ldst_uimm(3, 0, 2, 3)),
        ("prfum_simm_neg8".into(), enc_ldst_simm(3, 0, 2, 0b00, -8)),
        ("prfm_reg_uxtw_lsl3".into(), enc_prfm_reg(4, RM, 0b010, 1)),
        ("prfm_reg_sxtx".into(), enc_prfm_reg(0b10010, RM, 0b111, 0)),
        ("prfm_post_undef".into(), enc_ldst_simm(3, 0, 2, 0b01, 8)),
        ("prfm_pre_undef".into(), enc_ldst_simm(3, 0, 2, 0b11, 8)),
        ("prfm_opc11_undef".into(), enc_ldst_uimm(3, 0, 3, 0)),
        (
            "prfm_reg_bad_extend_undef".into(),
            enc_prfm_reg(0, RM, 0b000, 0),
        ),
    ];
    let mut rng = Rng::new(0x1_0020);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..6 {
            let mut st = mem_input(&mut rng);
            st.x[2] = 8;
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("mem_prfm", batch);
}

/// Scalar FP 3-source: `0001_1111 type o1 Rm o0 Ra Rn Rd`.
fn enc_fp3(fp_type: u32, o1: u32, o0: u32) -> u32 {
    (0b00011111 << 24)
        | (fp_type << 22)
        | (o1 << 21)
        | (RM << 16)
        | (o0 << 15)
        | (RA << 10)
        | (RN << 5)
        | RD
}
/// Scalar FP 2-source: `0001_1110 type 1 Rm opcode 10 Rn Rd`.
fn enc_fp2(fp_type: u32, opcode: u32) -> u32 {
    (0b00011110 << 24)
        | (fp_type << 22)
        | (1 << 21)
        | (RM << 16)
        | (opcode << 12)
        | (0b10 << 10)
        | (RN << 5)
        | RD
}
/// Scalar FP 1-source: `0001_1110 type 1 opcode 10000 Rn Rd`.
fn enc_fp1(fp_type: u32, opcode: u32) -> u32 {
    (0b00011110 << 24)
        | (fp_type << 22)
        | (1 << 21)
        | (opcode << 15)
        | (0b10000 << 10)
        | (RN << 5)
        | RD
}

/// Fill v0..v3 low elements with finite (non-zero) floats. `nonneg` keeps them
/// >= 0 (for FSQRT).
fn fill_scalar_fp(st: &mut ArmState, rng: &mut Rng, f64op: bool, nonneg: bool) {
    for r in 0..4usize {
        let n = (rng.next() % 40) as i64 - 20;
        let iv = if nonneg {
            (n.abs()) + 1
        } else if n == 0 {
            1
        } else {
            n
        };
        if f64op {
            let v = iv as f64 * 0.25;
            st.set_vreg(r, v.to_bits(), 0);
        } else {
            let v = iv as f32 * 0.25;
            st.set_vreg(r, v.to_bits() as u64, 0);
        }
    }
}

#[test]
fn diff_fp_scalar() {
    let mut cases: Vec<(String, u32, bool)> = Vec::new();
    for &ft in &[0u32, 1] {
        let f64op = ft == 1;
        // 3-source: FMADD/FMSUB/FNMADD/FNMSUB
        for o1 in 0..2 {
            for o0 in 0..2 {
                cases.push((
                    format!("fp3 t{ft} o1{o1} o0{o0}"),
                    enc_fp3(ft, o1, o0),
                    f64op,
                ));
            }
        }
        // 2-source: FMUL/FDIV/FADD/FSUB/FMAX/FMIN/FMAXNM/FMINNM/FNMUL
        for opcode in 0..9u32 {
            cases.push((format!("fp2 t{ft} op{opcode}"), enc_fp2(ft, opcode), f64op));
        }
        // 1-source: FMOV/FABS/FNEG + FRINT family (skip FSQRT here; tested below)
        for &opcode in &[
            0b000000u32,
            0b000001,
            0b000010,
            0b001000,
            0b001001,
            0b001010,
            0b001011,
            0b001100,
            0b001110,
            0b001111,
        ] {
            cases.push((
                format!("fp1 t{ft} op{opcode:06b}"),
                enc_fp1(ft, opcode),
                f64op,
            ));
        }
    }
    let mut rng = Rng::new(0xF101);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn, f64op) in &cases {
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            fill_scalar_fp(&mut st, &mut rng, *f64op, false);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("fp_scalar", batch);
}

#[test]
fn diff_fp_scalar_sqrt() {
    let mut cases: Vec<(String, u32, bool)> = Vec::new();
    for &ft in &[0u32, 1] {
        cases.push((format!("fsqrt t{ft}"), enc_fp1(ft, 0b000011), ft == 1));
    }
    let mut rng = Rng::new(0xF102);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn, f64op) in &cases {
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            fill_scalar_fp(&mut st, &mut rng, *f64op, true);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("fp_scalar_sqrt", batch);
}

/// FCCMP/FCCMPE: scalar FP conditional compare. Output is only NZCV (the flag
/// rules are deterministic even for NaN operands), so comparison is exact.
#[test]
fn diff_fp_ccmp() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &fp_type in &[0b00u32, 0b01, 0b11] {
        for cond in 0..16u32 {
            for op in 0..2u32 {
                for &nzcv in &[0u32, 0xF, 0x5] {
                    let insn = (0b00011110u32 << 24)
                        | (fp_type << 22)
                        | (1 << 21)
                        | (RM << 16)
                        | (cond << 12)
                        | (0b01 << 10)
                        | (RN << 5)
                        | (op << 4)
                        | nzcv;
                    cases.push((
                        format!("fccmp ty{fp_type} cond{cond} op{op} nzcv{nzcv}"),
                        insn,
                    ));
                }
            }
        }
    }
    run_family("fp_ccmp", cases, 6, 0x3001);
}

/// Advanced SIMD shift-by-immediate: `0 Q U 011110 immh immb opcode 1 Rn Rd`.
fn enc_shift_imm(q: u32, u: u32, opcode: u32, immhimmb: u32) -> u32 {
    let immh = (immhimmb >> 3) & 0xF;
    let immb = immhimmb & 0x7;
    (q << 30)
        | (u << 29)
        | (0b011110 << 23)
        | (immh << 19)
        | (immb << 16)
        | (opcode << 11)
        | (1 << 10)
        | (RN << 5)
        | RD
}

/// Integer shift-by-immediate cases: same-size, widening and narrowing forms
/// across all element sizes and the full range of valid shift amounts.
fn shift_imm_int_cases() -> Vec<(String, u32)> {
    let mut v = Vec::new();
    // Same-size ops: (opcode, U-options, is_left)
    let same: &[(u32, &[u32], bool)] = &[
        (0b00000, &[0, 1], false), // SSHR/USHR
        (0b00010, &[0, 1], false), // SSRA/USRA
        (0b00100, &[0, 1], false), // SRSHR/URSHR
        (0b00110, &[0, 1], false), // SRSRA/URSRA
        (0b01000, &[1], false),    // SRI
        (0b01010, &[0, 1], true),  // SHL / SLI
        (0b01100, &[1], true),     // SQSHLU
        (0b01110, &[0, 1], true),  // SQSHL / UQSHL
    ];
    for &(opcode, us, is_left) in same {
        for &bits in &[8u32, 16, 32, 64] {
            for &u in us {
                for q in 0..2 {
                    if bits == 64 && q == 0 {
                        continue; // 1D not valid
                    }
                    let shifts: Vec<u32> = if is_left {
                        (0..bits).collect()
                    } else {
                        (1..=bits).collect()
                    };
                    for sh in shifts {
                        let immhimmb = if is_left { bits + sh } else { 2 * bits - sh };
                        v.push((
                            format!("sh same op{opcode:05b} u{u} b{bits} q{q} #{sh}"),
                            enc_shift_imm(q, u, opcode, immhimmb),
                        ));
                    }
                }
            }
        }
    }
    // Widening SSHLL/USHLL (opcode 10100), source size 8/16/32.
    for &bits in &[8u32, 16, 32] {
        for u in 0..2 {
            for q in 0..2 {
                for sh in 0..bits {
                    let immhimmb = bits + sh;
                    v.push((
                        format!("sshll u{u} b{bits} q{q} #{sh}"),
                        enc_shift_imm(q, u, 0b10100, immhimmb),
                    ));
                }
            }
        }
    }
    // Narrowing (opcode 10000/10001/10010/10011), dest size 8/16/32.
    for &opcode in &[0b10000u32, 0b10001, 0b10010, 0b10011] {
        for &bits in &[8u32, 16, 32] {
            for u in 0..2 {
                for q in 0..2 {
                    for sh in 1..=bits {
                        let immhimmb = 2 * bits - sh;
                        v.push((
                            format!("narrow op{opcode:05b} u{u} b{bits} q{q} #{sh}"),
                            enc_shift_imm(q, u, opcode, immhimmb),
                        ));
                    }
                }
            }
        }
    }
    v
}

#[test]
fn diff_simd_shift_imm() {
    run_family("simd_shift_imm", shift_imm_int_cases(), 6, 0x4001);
}

/// Fixed-point convert (SCVTF/UCVTF/FCVTZS/FCVTZU), clean finite inputs.
#[test]
fn diff_simd_shift_fixedpoint() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &opcode in &[0b11100u32, 0b11111] {
        for &bits in &[32u32, 64] {
            for u in 0..2 {
                for q in 0..2 {
                    if bits == 64 && q == 0 {
                        continue;
                    }
                    for &fbits in &[1u32, bits / 2, bits - 1] {
                        let immhimmb = 2 * bits - fbits;
                        cases.push((
                            format!("fxp op{opcode:05b} u{u} b{bits} q{q} f{fbits}"),
                            enc_shift_imm(q, u, opcode, immhimmb),
                        ));
                    }
                }
            }
        }
    }
    // Inputs: for FCVTZS/U the source lanes are floats; for SCVTF/UCVTF they are
    // integers. Use small finite magnitudes so results are exact / unambiguous.
    let mut rng = Rng::new(0x4002);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        let is_fcvt = label.contains("11111");
        let bits64 = label.contains("b64");
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            let mut packed: u128 = 0;
            for lane in 0..4 {
                let word: u64 = if is_fcvt {
                    // small finite float values
                    let val = ((rng.next() % 41) as i64 - 20) as f64 * 0.25;
                    if bits64 {
                        val.to_bits()
                    } else {
                        (val as f32).to_bits() as u64
                    }
                } else {
                    // small signed integers
                    ((rng.next() % 4001) as i64 - 2000) as u64
                };
                if bits64 {
                    packed |= (word as u128) << (64 * lane.min(1));
                    if lane == 1 {
                        break;
                    }
                } else {
                    packed |= ((word as u32) as u128) << (32 * lane);
                }
            }
            st.set_vreg(1, packed as u64, (packed >> 64) as u64);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("simd_shift_fixedpoint", batch);
}

/// Advanced SIMD three-different (widening/narrowing): `0 Q U 01110 size 1 Rm opcode 00 Rn Rd`.
fn enc_3diff(q: u32, u: u32, size: u32, opcode: u32) -> u32 {
    (q << 30)
        | (u << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (1 << 21)
        | (RM << 16)
        | (opcode << 12)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_three_diff() {
    // (opcode, U-options, name)
    let ops: &[(u32, &[u32], &str)] = &[
        (0b0000, &[0, 1], "saddl_uaddl"),
        (0b0001, &[0, 1], "saddw_uaddw"),
        (0b0010, &[0, 1], "ssubl_usubl"),
        (0b0011, &[0, 1], "ssubw_usubw"),
        (0b0100, &[0, 1], "addhn_raddhn"),
        (0b0101, &[0, 1], "sabal_uabal"),
        (0b0110, &[0, 1], "subhn_rsubhn"),
        (0b0111, &[0, 1], "sabdl_uabdl"),
        (0b1000, &[0, 1], "smlal_umlal"),
        (0b1001, &[0], "sqdmlal"),
        (0b1010, &[0, 1], "smlsl_umlsl"),
        (0b1011, &[0], "sqdmlsl"),
        (0b1100, &[0, 1], "smull_umull"),
        (0b1101, &[0], "sqdmull"),
        (0b1110, &[0], "pmull"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(opcode, us, name) in ops {
        for &u in us {
            for size in 0..3 {
                for q in 0..2 {
                    cases.push((
                        format!("{name} sz{size} q{q}"),
                        enc_3diff(q, u, size, opcode),
                    ));
                }
            }
            // PMULL.1Q (size==3) for the polynomial op only.
            if opcode == 0b1110 {
                for q in 0..2 {
                    cases.push((format!("{name} sz3 q{q}"), enc_3diff(q, u, 3, opcode)));
                }
            }
        }
    }
    run_family("simd_three_diff", cases, 8, 0x8001);
}

/// Advanced SIMD across-lanes reduction: `0 Q U 01110 size 11000 opcode 10 Rn Rd`.
fn enc_across(q: u32, u: u32, size: u32, opcode: u32) -> u32 {
    (q << 30)
        | (u << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (0b11000 << 17)
        | (opcode << 12)
        | (0b10 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_across_int() {
    let ops: &[(u32, u32, &str)] = &[
        (0, 0b11011, "addv"),
        (0, 0b00011, "saddlv"),
        (1, 0b00011, "uaddlv"),
        (0, 0b01010, "smaxv"),
        (1, 0b01010, "umaxv"),
        (0, 0b11010, "sminv"),
        (1, 0b11010, "uminv"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(u, opcode, name) in ops {
        for size in 0..4 {
            for q in 0..2 {
                cases.push((
                    format!("{name} sz{size} q{q}"),
                    enc_across(q, u, size, opcode),
                ));
            }
        }
    }
    run_family("simd_across_int", cases, 10, 0x7001);
}

#[test]
fn diff_simd_across_fp() {
    // FMAXV/FMINV/FMAXNMV/FMINNMV over 4S (q==1), size bit23 picks min.
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(opcode, name) in &[(0b01111u32, "fmaxv_fminv"), (0b01100, "fmaxnmv_fminnmv")] {
        for &size in &[0b00u32, 0b10] {
            cases.push((format!("{name} sz{size}"), enc_across(1, 1, size, opcode)));
        }
    }
    let mut rng = Rng::new(0x7002);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        for _ in 0..24 {
            let mut st = ArmState::zeroed();
            let mut packed: u128 = 0;
            for lane in 0..4 {
                let val = ((rng.next() % 81) as i64 - 40) as f32 * 0.25;
                packed |= (val.to_bits() as u128) << (32 * lane);
            }
            st.set_vreg(1, packed as u64, (packed >> 64) as u64);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("simd_across_fp", batch);
}

#[test]
fn diff_simd_three_same_fp() {
    // (U, a, opcode, name) for the FP three-same ops. size = (a<<1)|sz.
    let ops: &[(u32, u32, u32, &str)] = &[
        (0, 0, 0b11000, "fmaxnm"),
        (0, 0, 0b11001, "fmla"),
        (0, 0, 0b11010, "fadd"),
        (0, 0, 0b11011, "fmulx"),
        (0, 0, 0b11100, "fcmeq"),
        (0, 0, 0b11110, "fmax"),
        (0, 0, 0b11111, "frecps"),
        (0, 1, 0b11000, "fminnm"),
        (0, 1, 0b11001, "fmls"),
        (0, 1, 0b11010, "fsub"),
        (0, 1, 0b11110, "fmin"),
        (0, 1, 0b11111, "frsqrts"),
        (1, 0, 0b11000, "fmaxnmp"),
        (1, 0, 0b11010, "faddp"),
        (1, 0, 0b11011, "fmul"),
        (1, 0, 0b11100, "fcmge"),
        (1, 0, 0b11101, "facge"),
        (1, 0, 0b11110, "fmaxp"),
        (1, 0, 0b11111, "fdiv"),
        (1, 1, 0b11000, "fminnmp"),
        (1, 1, 0b11010, "fabd"),
        (1, 1, 0b11100, "fcmgt"),
        (1, 1, 0b11101, "facgt"),
        (1, 1, 0b11110, "fminp"),
    ];
    let mut cases: Vec<(String, u32, bool)> = Vec::new();
    for &(u, a, opcode, name) in ops {
        for q in 0..2 {
            cases.push((
                format!("{name} f32 q{q}"),
                enc_three_same(q, u, a << 1, opcode),
                false,
            ));
        }
        cases.push((
            format!("{name} f64"),
            enc_three_same(1, u, (a << 1) | 1, opcode),
            true,
        ));
    }
    // Non-zero finite lanes (multiples of 0.25) so FDIV is well-defined and
    // products/sums stay exactly representable.
    let mut rng = Rng::new(0xE001);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn, f64op) in &cases {
        for _ in 0..12 {
            let mut st = ArmState::zeroed();
            for r in 0..3usize {
                let mut packed: u128 = 0;
                if *f64op {
                    for lane in 0..2 {
                        let n = (rng.next() % 40) as i64 - 20;
                        let v = if n == 0 { 1 } else { n } as f64 * 0.25;
                        packed |= (v.to_bits() as u128) << (64 * lane);
                    }
                } else {
                    for lane in 0..4 {
                        let n = (rng.next() % 40) as i64 - 20;
                        let v = if n == 0 { 1 } else { n } as f32 * 0.25;
                        packed |= (v.to_bits() as u128) << (32 * lane);
                    }
                }
                st.set_vreg(r, packed as u64, (packed >> 64) as u64);
            }
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("simd_three_same_fp", batch);
}

/// Scalar Advanced SIMD three-same: `01 U 11110 size 1 Rm opcode 1 Rn Rd`.
fn enc_scalar_3same(u: u32, size: u32, opcode: u32) -> u32 {
    (0b01 << 30)
        | (u << 29)
        | (0b11110 << 24)
        | (size << 22)
        | (1 << 21)
        | (RM << 16)
        | (opcode << 11)
        | (1 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_scalar_three_same() {
    // Integer scalar three-same opcodes (size validity is enforced by rax /
    // checked against the oracle).
    let opcodes: &[(u32, &str)] = &[
        (0b00001, "sqadd"),
        (0b00101, "sqsub"),
        (0b00110, "cmgt_cmhi"),
        (0b00111, "cmge_cmhs"),
        (0b01000, "sshl_ushl"),
        (0b01001, "sqshl_uqshl"),
        (0b01010, "srshl_urshl"),
        (0b01011, "sqrshl_uqrshl"),
        (0b10000, "add_sub"),
        (0b10001, "cmtst_cmeq"),
        (0b10110, "sqdmulh_sqrdmulh"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(opcode, name) in opcodes {
        for u in 0..2 {
            for size in 0..4 {
                cases.push((
                    format!("{name} u{u} sz{size}"),
                    enc_scalar_3same(u, size, opcode),
                ));
            }
        }
    }
    run_family("simd_scalar_three_same", cases, 8, 0xD001);
}

/// SIMD modified immediate: `0 Q op 0111100000 abc cmode o2 1 defgh Rd`.
fn enc_modimm(q: u32, op: u32, cmode: u32, imm8: u32) -> u32 {
    let abc = (imm8 >> 5) & 0x7;
    let defgh = imm8 & 0x1F;
    (q << 30)
        | (op << 29)
        | (0x0F << 24)
        | (abc << 16)
        | (cmode << 12)
        | (1 << 10)
        | (defgh << 5)
        | RD
}

#[test]
fn diff_simd_modimm() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for op in 0..2u32 {
        for cmode in 0..16u32 {
            for q in 0..2 {
                for &imm8 in &[0x00u32, 0xFF, 0x55, 0xA3, 0x80, 0x01] {
                    cases.push((
                        format!("modimm op{op} cm{cmode:04b} q{q} #{imm8:#04x}"),
                        enc_modimm(q, op, cmode, imm8),
                    ));
                }
            }
        }
    }
    run_family("simd_modimm", cases, 4, 0xC001);
}

/// Advanced SIMD permute (ZIP/UZP/TRN): `0 Q 0 01110 size 0 Rm 0 opcode 10 Rn Rd`.
fn enc_permute(q: u32, size: u32, opcode: u32) -> u32 {
    (q << 30)
        | (0b01110 << 24)
        | (size << 22)
        | (RM << 16)
        | (opcode << 12)
        | (0b10 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_permute() {
    let ops: &[(u32, &str)] = &[
        (0b001, "uzp1"),
        (0b101, "uzp2"),
        (0b010, "trn1"),
        (0b110, "trn2"),
        (0b011, "zip1"),
        (0b111, "zip2"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(opcode, name) in ops {
        for size in 0..4 {
            for q in 0..2 {
                cases.push((
                    format!("{name} sz{size} q{q}"),
                    enc_permute(q, size, opcode),
                ));
            }
        }
    }
    run_family("simd_permute", cases, 6, 0xB001);
}

/// Advanced SIMD EXT: `0 Q 10 1110 00 0 Rm 0 imm4 0 Rn Rd`.
fn enc_ext(q: u32, imm4: u32) -> u32 {
    (q << 30) | (1 << 29) | (0b01110 << 24) | (RM << 16) | (imm4 << 11) | (RN << 5) | RD
}

#[test]
fn diff_simd_ext() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for q in 0..2 {
        let maxidx = if q == 1 { 16 } else { 8 };
        for imm4 in 0..maxidx {
            cases.push((format!("ext q{q} #{imm4}"), enc_ext(q, imm4)));
        }
    }
    run_family("simd_ext", cases, 6, 0xB002);
}

/// Advanced SIMD TBL/TBX: `0 Q 00 1110 00 0 Rm 0 len op 00 Rn Rd`.
fn enc_tbl(q: u32, len: u32, op: u32) -> u32 {
    (q << 30) | (0b001110 << 23) | (RM << 16) | (len << 13) | (op << 12) | (RN << 5) | RD
}

#[test]
fn diff_simd_tbl() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for op in 0..2 {
        for len in 0..4 {
            for q in 0..2 {
                let name = if op == 0 { "tbl" } else { "tbx" };
                cases.push((format!("{name} len{len} q{q}"), enc_tbl(q, len, op)));
            }
        }
    }
    run_family("simd_tbl", cases, 8, 0xB003);
}

/// Advanced SIMD copy: `0 Q op 01110000 imm5 0 imm4 1 Rn Rd`.
fn enc_copy(q: u32, op: u32, imm5: u32, imm4: u32) -> u32 {
    (q << 30)
        | (op << 29)
        | (0b01110 << 24)
        | (imm5 << 16)
        | (imm4 << 11)
        | (1 << 10)
        | (RN << 5)
        | RD
}

/// imm5 for a given element size index (0=B,1=H,2=S,3=D) and lane index.
fn copy_imm5(size: u32, index: u32) -> u32 {
    (index << (size + 1)) | (1 << size)
}

#[test]
fn diff_simd_copy() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    // DUP element (op0 imm4 0000), DUP general (imm4 0001), INS general (0011),
    // SMOV (0101), UMOV (0111); INS element (op1).
    for size in 0..4u32 {
        let lanes: u32 = 16u32 >> size; // lanes in 128 bits (16/8/4/2)
        for &index in &[0u32, lanes - 1] {
            for q in 0..2 {
                cases.push((
                    format!("dupelem sz{size} i{index} q{q}"),
                    enc_copy(q, 0, copy_imm5(size, index), 0b0000),
                ));
                cases.push((
                    format!("dupgen sz{size} i{index} q{q}"),
                    enc_copy(q, 0, copy_imm5(size, index), 0b0001),
                ));
                cases.push((
                    format!("smov sz{size} i{index} q{q}"),
                    enc_copy(q, 0, copy_imm5(size, index), 0b0101),
                ));
                cases.push((
                    format!("umov sz{size} i{index} q{q}"),
                    enc_copy(q, 0, copy_imm5(size, index), 0b0111),
                ));
            }
            // INS general/element are always 128-bit (Q=1 in the encoding).
            cases.push((
                format!("insgen sz{size} i{index}"),
                enc_copy(1, 0, copy_imm5(size, index), 0b0011),
            ));
            // INS element: dest index from imm5, source index from imm4 (per size).
            let src = (index + 1) % lanes;
            cases.push((
                format!("inselem sz{size} d{index} s{src}"),
                enc_copy(1, 1, copy_imm5(size, index), src << size),
            ));
        }
    }
    run_family("simd_copy", cases, 8, 0x9001);
}

/// Advanced SIMD two-register miscellaneous: `0 Q U 01110 size 10000 opcode 10 Rn Rd`.
fn enc_two_reg(q: u32, u: u32, size: u32, opcode: u32) -> u32 {
    (q << 30)
        | (u << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (0b10000 << 17)
        | (opcode << 12)
        | (0b10 << 10)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_two_reg_int() {
    // (U, opcode, name) for the integer same-size / REV / NOT-RBIT ops.
    let ops: &[(u32, u32, &str)] = &[
        (0, 0b00000, "rev64"),
        (1, 0b00000, "rev32"),
        (0, 0b00001, "rev16"),
        (0, 0b00011, "suqadd"),
        (1, 0b00011, "usqadd"),
        (0, 0b00100, "cls"),
        (1, 0b00100, "clz"),
        (0, 0b00101, "cnt"),
        (1, 0b00101, "not_rbit"),
        (0, 0b00111, "sqabs"),
        (1, 0b00111, "sqneg"),
        (0, 0b01000, "cmgt0"),
        (1, 0b01000, "cmge0"),
        (0, 0b01001, "cmeq0"),
        (1, 0b01001, "cmle0"),
        (0, 0b01010, "cmlt0"),
        (0, 0b01011, "abs"),
        (1, 0b01011, "neg"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(u, opcode, name) in ops {
        for size in 0..4 {
            for q in 0..2 {
                cases.push((
                    format!("{name} sz{size} q{q}"),
                    enc_two_reg(q, u, size, opcode),
                ));
            }
        }
    }
    run_family("simd_two_reg_int", cases, 8, 0x6001);
}

#[test]
fn diff_simd_two_reg_widen() {
    // Pairwise-widening, narrowing and shift-long two-reg forms (size 0..2).
    let ops: &[(u32, u32, &str)] = &[
        (0, 0b00010, "saddlp"),
        (1, 0b00010, "uaddlp"),
        (0, 0b00110, "sadalp"),
        (1, 0b00110, "uadalp"),
        (0, 0b10010, "xtn"),
        (1, 0b10010, "sqxtun"),
        (0, 0b10100, "sqxtn"),
        (1, 0b10100, "uqxtn"),
        (1, 0b10011, "shll"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(u, opcode, name) in ops {
        for size in 0..3 {
            for q in 0..2 {
                cases.push((
                    format!("{name} sz{size} q{q}"),
                    enc_two_reg(q, u, size, opcode),
                ));
            }
        }
    }
    run_family("simd_two_reg_widen", cases, 8, 0x6003);
}

/// Fill registers v0..v2 with finite float lanes (multiples of 0.25, so rounding
/// ties land exactly on .5). If `nonneg`, all lanes are >= 0 (for FSQRT).
fn fill_fp_lanes(st: &mut ArmState, rng: &mut Rng, f64op: bool, nonneg: bool) {
    for r in 0..3usize {
        let mut packed: u128 = 0;
        if f64op {
            for lane in 0..2 {
                let n = (rng.next() % 256) as i64;
                let iv = if nonneg { n } else { n - 128 };
                let val = iv as f64 * 0.25;
                packed |= (val.to_bits() as u128) << (64 * lane);
            }
        } else {
            for lane in 0..4 {
                let n = (rng.next() % 256) as i64;
                let iv = if nonneg { n } else { n - 128 };
                let val = iv as f32 * 0.25;
                packed |= (val.to_bits() as u128) << (32 * lane);
            }
        }
        st.set_vreg(r, packed as u64, (packed >> 64) as u64);
    }
}

/// Build FP two-reg cases for the given (u, sz_hi, opcode, name) ops, covering
/// f32 (2S/4S) and f64 (2D), then drive `n` finite inputs through each.
fn fp_two_reg_batch(
    ops: &[(u32, u32, u32, &str)],
    seed: u64,
    n: usize,
    nonneg: bool,
) -> Vec<(String, u32, ArmState)> {
    let mut cases: Vec<(String, u32, bool)> = Vec::new();
    for &(u, sz_hi, opcode, name) in ops {
        // f32: 2S (q=0) and 4S (q=1)
        for q in 0..2 {
            let size = sz_hi << 1; // sz=0
            cases.push((
                format!("{name} f32 q{q}"),
                enc_two_reg(q, u, size, opcode),
                false,
            ));
        }
        // f64: 2D (q=1 only)
        let size = (sz_hi << 1) | 1;
        cases.push((format!("{name} f64"), enc_two_reg(1, u, size, opcode), true));
    }
    let mut rng = Rng::new(seed);
    let mut batch = Vec::new();
    for (label, insn, f64op) in &cases {
        for _ in 0..n {
            let mut st = ArmState::zeroed();
            fill_fp_lanes(&mut st, &mut rng, *f64op, nonneg);
            batch.push((label.clone(), *insn, st));
        }
    }
    batch
}

#[test]
fn diff_simd_two_reg_fp() {
    // (U, sz_hi, opcode, name) -- float-input ops except FSQRT/SCVTF/UCVTF.
    let ops: &[(u32, u32, u32, &str)] = &[
        (0, 1, 0b01111, "fabs"),
        (1, 1, 0b01111, "fneg"),
        (0, 0, 0b11000, "frintn"),
        (0, 1, 0b11000, "frintp"),
        (1, 0, 0b11000, "frinta"),
        (0, 0, 0b11001, "frintm"),
        (0, 1, 0b11001, "frintz"),
        (1, 0, 0b11001, "frintx"),
        (1, 1, 0b11001, "frinti"),
        (0, 0, 0b11010, "fcvtns"),
        (0, 1, 0b11010, "fcvtps"),
        (1, 0, 0b11010, "fcvtnu"),
        (1, 1, 0b11010, "fcvtpu"),
        (0, 0, 0b11011, "fcvtms"),
        (0, 1, 0b11011, "fcvtzs"),
        (1, 0, 0b11011, "fcvtmu"),
        (1, 1, 0b11011, "fcvtzu"),
        (0, 0, 0b11100, "fcvtas"),
        (1, 0, 0b11100, "fcvtau"),
        (0, 1, 0b01100, "fcmgt0"),
        (1, 1, 0b01100, "fcmge0"),
        (0, 1, 0b01101, "fcmeq0"),
        (1, 1, 0b01101, "fcmle0"),
        (0, 1, 0b01110, "fcmlt0"),
    ];
    run_batch("simd_two_reg_fp", fp_two_reg_batch(ops, 0xA001, 12, false));
}

#[test]
fn diff_simd_two_reg_fsqrt() {
    let ops: &[(u32, u32, u32, &str)] = &[(1, 1, 0b11111, "fsqrt")];
    run_batch(
        "simd_two_reg_fsqrt",
        fp_two_reg_batch(ops, 0xA002, 24, true),
    );
}

#[test]
fn diff_simd_two_reg_cvtf() {
    // SCVTF/UCVTF take integer source lanes.
    let ops: &[(u32, u32, u32, &str)] = &[(0, 0, 0b11101, "scvtf"), (1, 0, 0b11101, "ucvtf")];
    let mut cases: Vec<(String, u32, bool)> = Vec::new();
    for &(u, sz_hi, opcode, name) in ops {
        for q in 0..2 {
            cases.push((
                format!("{name} 32 q{q}"),
                enc_two_reg(q, u, sz_hi << 1, opcode),
                false,
            ));
        }
        cases.push((
            format!("{name} 64"),
            enc_two_reg(1, u, (sz_hi << 1) | 1, opcode),
            true,
        ));
    }
    let mut rng = Rng::new(0xA003);
    let mut batch = Vec::new();
    for (label, insn, is64) in &cases {
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            let mut packed: u128 = 0;
            if *is64 {
                for lane in 0..2 {
                    packed |= ((rng.next() as u64) as u128) << (64 * lane);
                }
            } else {
                for lane in 0..4 {
                    packed |= ((rng.next() as u32) as u128) << (32 * lane);
                }
            }
            st.set_vreg(1, packed as u64, (packed >> 64) as u64);
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("simd_two_reg_cvtf", batch);
}

/// Advanced SIMD vector x indexed element: `0 Q U 01111 size L M Rm opcode H 0 Rn Rd`.
fn enc_indexed(q: u32, u: u32, size: u32, opcode: u32, vm: u32, index: u32) -> u32 {
    let (rm, mbit, lbit, hbit) = match size {
        0b01 => (vm & 0xF, index & 1, (index >> 1) & 1, (index >> 2) & 1),
        0b10 => (vm & 0xF, (vm >> 4) & 1, index & 1, (index >> 1) & 1),
        0b11 => (vm & 0xF, (vm >> 4) & 1, 0, index & 1),
        _ => (0, 0, 0, 0),
    };
    (q << 30)
        | (u << 29)
        | (0b01111 << 24)
        | (size << 22)
        | (lbit << 21)
        | (mbit << 20)
        | (rm << 16)
        | (opcode << 12)
        | (hbit << 11)
        | (RN << 5)
        | RD
}

#[test]
fn diff_simd_indexed_int() {
    // (U, opcode) for the integer indexed ops; widening forms produce 2x results.
    let ops: &[(u32, u32, &str)] = &[
        (0, 0b1000, "mul"),
        (1, 0b0000, "mla"),
        (1, 0b0100, "mls"),
        (0, 0b1100, "sqdmulh"),
        (0, 0b1101, "sqrdmulh"),
        (1, 0b1101, "sqrdmlah"),
        (1, 0b1111, "sqrdmlsh"),
        (0, 0b0010, "smlal"),
        (1, 0b0010, "umlal"),
        (0, 0b0110, "smlsl"),
        (1, 0b0110, "umlsl"),
        (0, 0b1010, "smull"),
        (1, 0b1010, "umull"),
        (0, 0b0011, "sqdmlal"),
        (0, 0b0111, "sqdmlsl"),
        (0, 0b1011, "sqdmull"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(u, opcode, name) in ops {
        for &size in &[0b01u32, 0b10] {
            let max_index = if size == 0b01 { 8 } else { 4 };
            for q in 0..2 {
                for &index in &[0u32, max_index - 1] {
                    cases.push((
                        format!("{name} sz{size} q{q} idx{index}"),
                        enc_indexed(q, u, size, opcode, 2, index),
                    ));
                }
            }
        }
    }
    run_family("simd_indexed_int", cases, 8, 0x5001);
}

#[test]
fn diff_simd_indexed_fp() {
    let ops: &[(u32, u32, &str)] = &[
        (0, 0b1001, "fmul"),
        (0, 0b0001, "fmla"),
        (0, 0b0101, "fmls"),
        (1, 0b1001, "fmulx"),
    ];
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &(u, opcode, name) in ops {
        // size 10 = f32 (index 0..3), size 11 = f64 (index 0..1, Q must be 1)
        for &(size, qs, maxidx) in &[(0b10u32, &[0u32, 1][..], 4u32), (0b11u32, &[1u32][..], 2)] {
            for &q in qs {
                for index in 0..maxidx {
                    cases.push((
                        format!("{name} sz{size} q{q} idx{index}"),
                        enc_indexed(q, u, size, opcode, 2, index),
                    ));
                }
            }
        }
    }
    // Clean finite FP inputs in v1 (Rn), v2 (Rm), v0 (Rd accumulator).
    let mut rng = Rng::new(0x5002);
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in &cases {
        let f64op = label.contains("sz11");
        for _ in 0..16 {
            let mut st = ArmState::zeroed();
            for r in [0usize, 1, 2] {
                let mut packed: u128 = 0;
                if f64op {
                    for lane in 0..2 {
                        let val = ((rng.next() % 41) as i64 - 20) as f64 * 0.5;
                        packed |= (val.to_bits() as u128) << (64 * lane);
                    }
                } else {
                    for lane in 0..4 {
                        let val = ((rng.next() % 41) as i64 - 20) as f32 * 0.5;
                        packed |= (val.to_bits() as u128) << (32 * lane);
                    }
                }
                st.set_vreg(r, packed as u64, (packed >> 64) as u64);
            }
            batch.push((label.clone(), *insn, st));
        }
    }
    run_batch("simd_indexed_fp", batch);
}

/// FCSEL: scalar FP conditional select. Output is a bit-exact register copy.
#[test]
fn diff_fp_csel() {
    let mut cases: Vec<(String, u32)> = Vec::new();
    for &fp_type in &[0b00u32, 0b01, 0b11] {
        for cond in 0..16u32 {
            let insn = (0b00011110u32 << 24)
                | (fp_type << 22)
                | (1 << 21)
                | (RM << 16)
                | (cond << 12)
                | (0b11 << 10)
                | (RN << 5)
                | RD;
            cases.push((format!("fcsel ty{fp_type} cond{cond}"), insn));
        }
    }
    run_family("fp_csel", cases, 8, 0x3002);
}

// ===========================================================================
// Comprehensive SVE2 differential sweep. Runs an llvm-mc-generated encoding
// table (tests/sve2_gen.rs, covering every SVE2/SVE2.1 data-processing mnemonic
// across all element sizes and key variants) through the qemu-aarch64 oracle
// with random + interesting (special-FP-laden) inputs, classifying each
// mnemonic as decode-gap (hw runs, rax rejects), value-mismatch (rax computes a
// wrong answer), fault-disagree, or OK. Asserts zero divergences.
// ===========================================================================
include!("sve2_gen.rs");

#[test]
fn diff_sve2_comprehensive_sweep() {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!("[arm_diff] sve2_sweep_probe: toolchain unavailable -> skip");
            return;
        }
    };
    let mut rng = Rng::new(0x5e2_5eed_1234);
    let n_inputs = 6usize;
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in SVE2_SWEEP {
        for _ in 0..n_inputs {
            let mut st = gen_input(&mut rng);
            for r in 0..16 {
                st.set_preg(r, rng.next() as u16);
            }
            for i in 0..32 {
                st.scratch[i] = rng.next();
            }
            batch.push(((*label).to_string(), *insn, st));
        }
    }
    let cases: Vec<(u32, u32, ArmState)> = batch.iter().map(|(_, i, s)| (*i, NOP, *s)).collect();
    let outs = run_oracle(&oracle, &cases).expect("oracle run failed");
    assert_eq!(outs.len(), cases.len());

    use std::collections::BTreeMap;
    // per-label: (decode_gap, value_mismatch, fault_disagree, total)
    let mut stats: BTreeMap<String, [usize; 4]> = BTreeMap::new();
    let mut sample: BTreeMap<String, String> = BTreeMap::new();
    for (i, (label, insn, st)) in batch.iter().enumerate() {
        let out = &outs[i];
        let e = stats.entry(label.clone()).or_insert([0; 4]);
        e[3] += 1;
        let rax = run_rax(*insn, st);
        if out.trapped != 0 {
            if rax.is_some() {
                e[2] += 1;
                sample
                    .entry(label.clone())
                    .or_insert_with(|| format!("hw faulted sig{} but rax executed", out.trapped));
            }
            continue;
        }
        let rax = match rax {
            Some(s) => s,
            None => {
                e[0] += 1;
                sample
                    .entry(label.clone())
                    .or_insert_with(|| "rax rejected (undefined)".into());
                continue;
            }
        };
        // value compare (regs, nzcv, v, preds, scratch)
        let mut diffs = Vec::new();
        for r in 0..31 {
            if rax.x[r] != out.st.x[r] {
                diffs.push(format!("x{r}:rax={:#x} hw={:#x}", rax.x[r], out.st.x[r]));
            }
        }
        if (rax.pstate >> 28) & 0xF != (out.st.pstate >> 28) & 0xF {
            diffs.push(format!(
                "nzcv:rax={:#x} hw={:#x}",
                (rax.pstate >> 28) & 0xF,
                (out.st.pstate >> 28) & 0xF
            ));
        }
        for r in 0..32 {
            if rax.vreg(r) != out.st.vreg(r) {
                let (rl, rh) = rax.vreg(r);
                let (hl, hh) = out.st.vreg(r);
                diffs.push(format!(
                    "v{r}:rax={:#x}{:016x} hw={:#x}{:016x}",
                    rh, rl, hh, hl
                ));
            }
        }
        for r in 0..16 {
            if rax.preg(r) != out.st.preg(r) {
                diffs.push(format!(
                    "p{r}:rax={:#x} hw={:#x}",
                    rax.preg(r),
                    out.st.preg(r)
                ));
            }
        }
        if !diffs.is_empty() {
            e[1] += 1;
            sample
                .entry(label.clone())
                .or_insert_with(|| diffs.join(" | "));
        }
    }

    let mut gaps = Vec::new();
    let mut vals = Vec::new();
    let mut faults = Vec::new();
    for (label, e) in &stats {
        if e[0] > 0 {
            gaps.push((label.clone(), e[0], e[3]));
        }
        if e[1] > 0 {
            vals.push((label.clone(), e[1], e[3]));
        }
        if e[2] > 0 {
            faults.push((label.clone(), e[2], e[3]));
        }
    }
    eprintln!(
        "\n==== SVE2 SWEEP PROBE: {} mnemonics, {} cases ====",
        stats.len(),
        batch.len()
    );
    eprintln!("\n-- DECODE GAPS (hw runs, rax rejects): {} --", gaps.len());
    for (l, c, t) in &gaps {
        eprintln!(
            "  {c:3}/{t:<3} {l}    [{}]",
            sample.get(l).cloned().unwrap_or_default()
        );
    }
    eprintln!("\n-- VALUE MISMATCHES (wrong answer): {} --", vals.len());
    for (l, c, t) in &vals {
        eprintln!(
            "  {c:3}/{t:<3} {l}    [{}]",
            sample.get(l).cloned().unwrap_or_default()
        );
    }
    eprintln!(
        "\n-- FAULT DISAGREE (hw faults, rax runs): {} --",
        faults.len()
    );
    for (l, c, t) in &faults {
        eprintln!(
            "  {c:3}/{t:<3} {l}    [{}]",
            sample.get(l).cloned().unwrap_or_default()
        );
    }
    eprintln!(
        "\n==== END PROBE: {} gaps, {} value-mismatch, {} fault-disagree ====",
        gaps.len(),
        vals.len(),
        faults.len()
    );

    let total = gaps.len() + vals.len() + faults.len();
    assert_eq!(
        total,
        0,
        "sve2 comprehensive sweep: {} mnemonics diverged from the oracle \
         ({} decode-gaps, {} value-mismatches, {} fault-disagrees)",
        total,
        gaps.len(),
        vals.len(),
        faults.len()
    );
}

// ===========================================================================
// Comprehensive NEON / VFP / FP16 differential sweep. Same machinery as
// diff_sve2_comprehensive_sweep but over the AdvSIMD + scalar-FP + FP16
// encoding space (tests/neon_gen.rs). Asserts zero divergence vs the oracle.
// ===========================================================================
include!("neon_gen.rs");

#[test]
fn diff_neon_comprehensive_sweep() {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!("[arm_diff] neon_comprehensive_sweep: toolchain unavailable -> skip");
            return;
        }
    };
    let mut rng = Rng::new(0x4e_e0_1234);
    let n_inputs = 16usize;
    let mut batch: Vec<(String, u32, ArmState)> = Vec::new();
    for (label, insn) in NEON_SWEEP {
        for _ in 0..n_inputs {
            let mut st = gen_input(&mut rng);
            for i in 0..32 {
                st.scratch[i] = rng.next();
            }
            batch.push(((*label).to_string(), *insn, st));
        }
    }
    let cases: Vec<(u32, u32, ArmState)> = batch.iter().map(|(_, i, s)| (*i, NOP, *s)).collect();
    let outs = run_oracle(&oracle, &cases).expect("oracle run failed");
    assert_eq!(outs.len(), cases.len());

    use std::collections::BTreeMap;
    let mut stats: BTreeMap<String, [usize; 4]> = BTreeMap::new();
    let mut sample: BTreeMap<String, String> = BTreeMap::new();
    for (i, (label, insn, st)) in batch.iter().enumerate() {
        let out = &outs[i];
        let e = stats.entry(label.clone()).or_insert([0; 4]);
        e[3] += 1;
        let rax = run_rax(*insn, st);
        if out.trapped != 0 {
            if rax.is_some() {
                e[2] += 1;
                sample
                    .entry(label.clone())
                    .or_insert_with(|| format!("hw faulted sig{} but rax executed", out.trapped));
            }
            continue;
        }
        let rax = match rax {
            Some(s) => s,
            None => {
                e[0] += 1;
                sample
                    .entry(label.clone())
                    .or_insert_with(|| "rax rejected (undefined)".into());
                continue;
            }
        };
        let mut diffs = Vec::new();
        for r in 0..31 {
            if rax.x[r] != out.st.x[r] {
                diffs.push(format!("x{r}:rax={:#x} hw={:#x}", rax.x[r], out.st.x[r]));
            }
        }
        if (rax.pstate >> 28) & 0xF != (out.st.pstate >> 28) & 0xF {
            diffs.push(format!(
                "nzcv:rax={:#x} hw={:#x}",
                (rax.pstate >> 28) & 0xF,
                (out.st.pstate >> 28) & 0xF
            ));
        }
        for r in 0..32 {
            if rax.vreg(r) != out.st.vreg(r) {
                let (rl, rh) = rax.vreg(r);
                let (hl, hh) = out.st.vreg(r);
                diffs.push(format!(
                    "v{r}:rax={:#x}{:016x} hw={:#x}{:016x}",
                    rh, rl, hh, hl
                ));
            }
        }
        if !diffs.is_empty() {
            e[1] += 1;
            sample
                .entry(label.clone())
                .or_insert_with(|| diffs.join(" | "));
        }
    }

    let mut gaps = Vec::new();
    let mut vals = Vec::new();
    let mut faults = Vec::new();
    for (label, e) in &stats {
        if e[0] > 0 {
            gaps.push((label.clone(), e[0], e[3]));
        }
        if e[1] > 0 {
            vals.push((label.clone(), e[1], e[3]));
        }
        if e[2] > 0 {
            faults.push((label.clone(), e[2], e[3]));
        }
    }
    eprintln!(
        "\n==== NEON SWEEP PROBE: {} mnemonics, {} cases ====",
        stats.len(),
        batch.len()
    );
    eprintln!("\n-- DECODE GAPS (hw runs, rax rejects): {} --", gaps.len());
    for (l, c, t) in &gaps {
        eprintln!(
            "  {c:3}/{t:<3} {l}    [{}]",
            sample.get(l).cloned().unwrap_or_default()
        );
    }
    eprintln!("\n-- VALUE MISMATCHES (wrong answer): {} --", vals.len());
    for (l, c, t) in &vals {
        eprintln!(
            "  {c:3}/{t:<3} {l}    [{}]",
            sample.get(l).cloned().unwrap_or_default()
        );
    }
    eprintln!(
        "\n-- FAULT DISAGREE (hw faults, rax runs): {} --",
        faults.len()
    );
    for (l, c, t) in &faults {
        eprintln!(
            "  {c:3}/{t:<3} {l}    [{}]",
            sample.get(l).cloned().unwrap_or_default()
        );
    }
    eprintln!(
        "\n==== END NEON PROBE: {} gaps, {} value-mismatch, {} fault-disagree ====",
        gaps.len(),
        vals.len(),
        faults.len()
    );

    let total = gaps.len() + vals.len() + faults.len();
    assert_eq!(
        total,
        0,
        "neon comprehensive sweep: {} mnemonics diverged ({} gaps, {} value, {} fault)",
        total,
        gaps.len(),
        vals.len(),
        faults.len()
    );
}
