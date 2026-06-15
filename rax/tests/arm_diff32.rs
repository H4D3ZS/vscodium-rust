//! AArch32 (A32 + T16/T32 Thumb) differential test harness: the rax AArch32
//! interpreter vs. QEMU (hardware oracle).
//!
//! The rax software interpreter (`src/arm/execution.rs` +
//! `src/arm/decoder/{aarch32,thumb}.rs`) is checked against a hardware-semantics
//! reference produced by running each instruction under `qemu-arm` (user mode).
//! The reference harness is `tools/arm-diff/oracle-a32.c`, built on demand into a
//! static 32-bit ARM ELF. This mirrors `tests/arm_diff.rs` (the AArch64 harness).
//!
//! For each `(instruction, mode, initial state)` triple we:
//!   1. run it on the oracle (R0..R14, CPSR flags, D0..D31, scratch captured), and
//!   2. run it on the rax `Armv7Cpu` from the *identical* initial state,
//! then compare the register file. Any divergence is an interpreter bug.
//!
//! Robustness: if the cross compiler or `qemu-arm` is unavailable, every test
//! self-skips (returns without failing) so the suite is green anywhere.

#![cfg(target_os = "linux")]

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use rax::arm::decoder::{Aarch32Decoder, ThumbDecoder};
use rax::arm::execution::{ArmMemory, FlatMemory};
use rax::arm::{Armv7Cpu, ExecResult, Executor};

// ---------------------------------------------------------------------------
// Wire format -- must match tools/arm-diff/oracle-a32.c byte for byte.
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
struct ArmState32 {
    r: [u32; 15],       // R0..R14
    pc: u32,            // input: unused; output: post-instruction PC
    cpsr: u32,          // NZCVQ (31:27) + GE (19:16) + T (5)
    fpscr: u32,         // VFP status/control
    d: [u64; 32],       // D0..D31 (NEON Q0..Q15)
    scratch: [u32; 64], // shared scratch window (256 bytes)
}

/// Address of the shared scratch window (matches oracle-a32.c SCRATCH_ADDR).
const SCRATCH_ADDR: u32 = 0x20_0000;
/// Base pointer tests aim a register at (matches oracle-a32.c SCRATCH_BASE).
const SCRATCH_BASE: u32 = SCRATCH_ADDR + 128;

impl ArmState32 {
    fn zeroed() -> Self {
        ArmState32 {
            r: [0; 15],
            pc: 0,
            cpsr: 0,
            fpscr: 0,
            d: [0; 32],
            scratch: [0; 64],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InCase32 {
    insn: u32,
    mode: u32, // 0 = ARM, 1 = Thumb16, 2 = Thumb32
    st: ArmState32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct OutCase32 {
    st: ArmState32,
    trapped: u32,
    valid: u32,
}

const WIRE_MAGIC: u32 = 0x3132_3341; // 'A','3','2','1'

// Compile-time guarantee the layout matches the C side.
const _: () = assert!(core::mem::size_of::<ArmState32>() == 584);
const _: () = assert!(core::mem::size_of::<InCase32>() == 592);
const _: () = assert!(core::mem::size_of::<OutCase32>() == 592);

// CPSR bits we compare: N Z C V Q (31:27) and GE (19:16).
const CPSR_CMP_MASK: u32 = 0xF800_0000 | 0x000F_0000;

// ---------------------------------------------------------------------------
// Byte (de)serialisation helpers.
// ---------------------------------------------------------------------------

fn as_bytes<T: Copy>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
}

fn read_struct<T: Copy>(buf: &[u8], off: usize) -> T {
    assert!(off + std::mem::size_of::<T>() <= buf.len());
    unsafe { std::ptr::read_unaligned(buf[off..].as_ptr() as *const T) }
}

// ---------------------------------------------------------------------------
// Oracle: build on demand, run a whole batch through one qemu invocation.
// ---------------------------------------------------------------------------

fn oracle_path() -> Option<PathBuf> {
    if which("qemu-arm").is_none() {
        return None;
    }
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/arm-diff");
    let bin = dir.join("oracle-a32");
    let need_build = match (bin.metadata(), dir.join("oracle-a32.c").metadata()) {
        (Ok(b), Ok(c)) => match (b.modified(), c.modified()) {
            (Ok(bm), Ok(cm)) => bm < cm,
            _ => true,
        },
        _ => true,
    };
    if need_build {
        let status = Command::new("bash")
            .arg(dir.join("build-a32.sh"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        match status {
            Ok(s) if s.success() => {}
            _ => return None,
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

fn run_oracle(oracle: &PathBuf, cases: &[(u32, u32, ArmState32)]) -> Option<Vec<OutCase32>> {
    let mut payload = Vec::with_capacity(8 + cases.len() * std::mem::size_of::<InCase32>());
    payload.extend_from_slice(&WIRE_MAGIC.to_le_bytes());
    payload.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for (insn, mode, st) in cases {
        let ic = InCase32 {
            insn: *insn,
            mode: *mode,
            st: *st,
        };
        payload.extend_from_slice(as_bytes(&ic));
    }

    let mut child = Command::new("qemu-arm")
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
        res.push(read_struct::<OutCase32>(&out, off));
        off += std::mem::size_of::<OutCase32>();
    }
    Some(res)
}

// ---------------------------------------------------------------------------
// rax: run one instruction from an identical initial state.
// ---------------------------------------------------------------------------

/// Returns `Some(out_state)` if rax executed the instruction, or `None` if rax
/// treated it as undefined / errored.
fn run_rax(insn: u32, mode: u32, input: &ArmState32) -> Option<ArmState32> {
    let mut mem = FlatMemory::new(0x0040_0000, 0);
    // Install the scratch window.
    for (i, w) in input.scratch.iter().enumerate() {
        mem.write_word(SCRATCH_ADDR + (i as u32) * 4, *w).ok()?;
    }

    let mut cpu = Armv7Cpu::new();
    cpu.cpsr.t = mode != 0;
    for i in 0..15usize {
        cpu.set_reg(i, input.r[i]);
    }
    let ps = input.cpsr;
    cpu.cpsr.n = ps & (1 << 31) != 0;
    cpu.cpsr.z = ps & (1 << 30) != 0;
    cpu.cpsr.c = ps & (1 << 29) != 0;
    cpu.cpsr.v = ps & (1 << 28) != 0;
    cpu.cpsr.q = ps & (1 << 27) != 0;
    cpu.cpsr.ge = ((ps >> 16) & 0xF) as u8;

    let decoded = match mode {
        0 => Aarch32Decoder::decode(insn).ok()?,
        1 => ThumbDecoder::decode_16bit(insn as u16).ok()?,
        _ => ThumbDecoder::decode_32bit(insn).ok()?,
    };

    {
        let mut exec = Executor::new(&mut cpu, &mut mem);
        match exec.execute(&decoded) {
            ExecResult::Continue | ExecResult::Branch(_) => {}
            _ => return None,
        }
    }

    // Capture: pass d-regs / fpscr / pc through unchanged (rax has no AArch32
    // VFP model yet), which is correct for integer instructions.
    let mut out = *input;
    for i in 0..15usize {
        out.r[i] = cpu.reg(i);
    }
    let mut cpsr = 0u32;
    if cpu.cpsr.n {
        cpsr |= 1 << 31;
    }
    if cpu.cpsr.z {
        cpsr |= 1 << 30;
    }
    if cpu.cpsr.c {
        cpsr |= 1 << 29;
    }
    if cpu.cpsr.v {
        cpsr |= 1 << 28;
    }
    if cpu.cpsr.q {
        cpsr |= 1 << 27;
    }
    cpsr |= ((cpu.cpsr.ge & 0xF) as u32) << 16;
    if cpu.cpsr.t {
        cpsr |= 1 << 5;
    }
    out.cpsr = cpsr;
    for (i, w) in out.scratch.iter_mut().enumerate() {
        *w = mem.read_word(SCRATCH_ADDR + (i as u32) * 4).ok()?;
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
    mode: u32,
    input: &ArmState32,
    oracle: &OutCase32,
    cmp_vfp: bool,
    mismatches: &mut Vec<Mismatch>,
) {
    let rax = run_rax(insn, mode, input);

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
                detail: "hw executed but rax rejected the encoding (undefined/err)".into(),
            });
            return;
        }
    };

    let mut diffs = Vec::new();
    for i in 0..15 {
        if rax.r[i] != oracle.st.r[i] {
            diffs.push(format!(
                "r{i}: rax={:#010x} hw={:#010x}",
                rax.r[i], oracle.st.r[i]
            ));
        }
    }
    let rax_cpsr = rax.cpsr & CPSR_CMP_MASK;
    let hw_cpsr = oracle.st.cpsr & CPSR_CMP_MASK;
    if rax_cpsr != hw_cpsr {
        diffs.push(format!(
            "cpsr(NZCVQ+GE): rax={:#010x} hw={:#010x}",
            rax_cpsr, hw_cpsr
        ));
    }
    if cmp_vfp {
        for r in 0..32 {
            if rax.d[r] != oracle.st.d[r] {
                diffs.push(format!(
                    "d{r}: rax={:#018x} hw={:#018x}",
                    rax.d[r], oracle.st.d[r]
                ));
            }
        }
    }
    for i in 0..64 {
        if rax.scratch[i] != oracle.st.scratch[i] {
            diffs.push(format!(
                "scratch[{i}]: rax={:#010x} hw={:#010x}",
                rax.scratch[i], oracle.st.scratch[i]
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
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn interesting32(&mut self) -> u32 {
        match self.next() % 8 {
            0 => 0,
            1 => u32::MAX,
            2 => 1,
            3 => 0x8000_0000,
            4 => 0x7FFF_FFFF,
            5 => (self.next() & 0xFF) as u32,
            6 => (self.next() & 0xFFFF) as u32,
            _ => self.next() as u32,
        }
    }
}

/// Build a randomised input state. `scratch_reg`, if set, is pointed at the
/// scratch window so load/store families have a valid base.
fn gen_input(rng: &mut Rng, scratch_reg: Option<usize>) -> ArmState32 {
    let mut st = ArmState32::zeroed();
    for i in 0..15 {
        st.r[i] = rng.interesting32();
    }
    // Random NZCVQ (31:27) and GE (19:16).
    st.cpsr = (rng.next() as u32) & (0xF800_0000 | 0x000F_0000);
    st.fpscr = 0;
    for r in 0..32 {
        st.d[r] = rng.next();
    }
    for w in 0..64 {
        st.scratch[w] = rng.interesting32();
    }
    if let Some(reg) = scratch_reg {
        st.r[reg] = SCRATCH_BASE;
    }
    st
}

// ---------------------------------------------------------------------------
// Test driver.
// ---------------------------------------------------------------------------

fn run_batch(name: &str, batch: Vec<(String, u32, u32, ArmState32)>, cmp_vfp: bool) {
    let oracle = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!("[arm_diff32] {name}: qemu/cross-toolchain unavailable -> skipping");
            return;
        }
    };

    let labels: Vec<String> = batch.iter().map(|(l, _, _, _)| l.clone()).collect();
    let modes: Vec<u32> = batch.iter().map(|(_, _, m, _)| *m).collect();
    let cases: Vec<(u32, u32, ArmState32)> =
        batch.iter().map(|(_, i, m, s)| (*i, *m, *s)).collect();

    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!("[arm_diff32] {name}: oracle run failed -> skipping");
            return;
        }
    };
    assert_eq!(outs.len(), cases.len());

    let mut mismatches = Vec::new();
    for (i, ((insn, _mode, st), out)) in cases.iter().zip(outs.iter()).enumerate() {
        compare_case(
            &labels[i],
            *insn,
            modes[i],
            st,
            out,
            cmp_vfp,
            &mut mismatches,
        );
    }

    if !mismatches.is_empty() {
        use std::collections::BTreeMap;
        // Classify each mismatch.
        let kind = |d: &str| -> &'static str {
            if d.starts_with("hw executed but rax rejected") {
                "decode-gap"
            } else if d.starts_with("hw faulted") {
                "over-permissive"
            } else {
                "value"
            }
        };
        // Distinct (label) sets per kind, with a representative.
        let mut by_kind: BTreeMap<&str, usize> = BTreeMap::new();
        let mut distinct: BTreeMap<(&str, String), (usize, u32, String)> = BTreeMap::new();
        for m in &mismatches {
            let k = kind(&m.detail);
            *by_kind.entry(k).or_default() += 1;
            let e = distinct
                .entry((k, m.label.clone()))
                .or_insert((0, m.insn, m.detail.clone()));
            e.0 += 1;
        }
        eprintln!(
            "\n==== {name}: {} mismatches across {} cases ====",
            mismatches.len(),
            cases.len()
        );
        eprintln!("-- by kind --");
        for (k, c) in &by_kind {
            eprintln!("  {c:6}x  {k}");
        }
        eprintln!("-- distinct (kind, encoding): {} --", distinct.len());
        // Print value-mismatches first (most interesting), then decode-gaps.
        for want in ["value", "over-permissive", "decode-gap"] {
            let rows: Vec<_> = distinct.iter().filter(|((k, _), _)| *k == want).collect();
            eprintln!("  == {want} ({} distinct) ==", rows.len());
            for ((_, label), (count, insn, detail)) in rows.iter().take(120) {
                let d = if detail.len() > 200 {
                    &detail[..200]
                } else {
                    detail
                };
                eprintln!("    {count:4}x [{:#010x}] {label}: {d}", insn);
            }
        }
        panic!("{name}: {} mismatches (see above)", mismatches.len());
    }
}

// The generated comprehensive integer sweep table.
include!("arm32_gen.rs");

/// Run the integer sweep filtered to the given modes. An `A32DIFF_FILTER`
/// environment variable (substring match on the label) narrows it further, for
/// fast iteration on one category.
fn integer_sweep(name: &str, modes: &[u8], n_inputs: usize, seed: u64) {
    let filter = std::env::var("A32DIFF_FILTER").ok();
    let mut rng = Rng::new(seed);
    let mut batch = Vec::new();
    for (label, insn, mode) in A32_SWEEP.iter() {
        if !modes.contains(mode) {
            continue;
        }
        if let Some(f) = &filter {
            if !label.contains(f.as_str()) {
                continue;
            }
        }
        for _ in 0..n_inputs {
            let st = gen_input(&mut rng, None);
            batch.push((label.to_string(), *insn, *mode as u32, st));
        }
    }
    run_batch(name, batch, false);
}

/// Run the memory sweep: r1 points at the scratch window, r2 holds a small
/// offset, so every access stays inside the exchanged scratch region.
fn memory_sweep(name: &str, modes: &[u8], n_inputs: usize, seed: u64) {
    let filter = std::env::var("A32DIFF_FILTER").ok();
    let mut rng = Rng::new(seed);
    let mut batch = Vec::new();
    for (label, insn, mode) in A32_MEM_SWEEP.iter() {
        if !modes.contains(mode) {
            continue;
        }
        if let Some(f) = &filter {
            if !label.contains(f.as_str()) {
                continue;
            }
        }
        for _ in 0..n_inputs {
            let mut st = gen_input(&mut rng, None);
            st.r[1] = SCRATCH_BASE; // base register
            st.r[2] = 8; // small offset register
            batch.push((label.to_string(), *insn, *mode as u32, st));
        }
    }
    run_batch(name, batch, false);
}

/// A32 (ARM) load/store sweep (single/dual/multiple, all addressing modes).
#[test]
fn diff_a32_memory_sweep() {
    memory_sweep("a32_memory_sweep", &[0], 16, 0xA32_1001);
}

/// T16 (Thumb 16-bit) load/store sweep.
#[test]
fn diff_t16_memory_sweep() {
    memory_sweep("t16_memory_sweep", &[1], 16, 0xA32_1002);
}

/// T32 (Thumb-2 32-bit) load/store sweep.
#[test]
fn diff_t32_memory_sweep() {
    memory_sweep("t32_memory_sweep", &[2], 16, 0xA32_1003);
}

/// A32 (ARM) integer register-data-processing sweep.
#[test]
fn diff_a32_integer_sweep() {
    integer_sweep("a32_integer_sweep", &[0], 24, 0xA32_0001);
}

/// T32 (Thumb-2 32-bit) integer register-data-processing sweep.
#[test]
fn diff_t32_integer_sweep() {
    integer_sweep("t32_integer_sweep", &[2], 24, 0xA32_0002);
}

/// T16 (Thumb 16-bit) integer register-data-processing sweep.
#[test]
fn diff_t16_integer_sweep() {
    integer_sweep("t16_integer_sweep", &[1], 24, 0xA32_0003);
}
