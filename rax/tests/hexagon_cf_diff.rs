//! Hexagon control-flow / hardware-loop differential harness: rax vs.
//! qemu-hexagon (hardware oracle).
//!
//! The reg-only harness (`tests/hexagon_diff.rs`) cannot test control flow: it
//! patches one always-fall-through packet into a 4-word slot whose successor is
//! the capture epilogue, so a taken branch or a hardware-loop back-edge escapes
//! the harness and the post-state is never captured. Hardware loops in
//! particular only show their effect once the body has actually re-executed N
//! times (final LC0/LC1/SA0/SA1 plus whatever the body accumulated).
//!
//! This harness instead runs a *whole assembled fragment* (multiple packets,
//! containing loop setup / branches / a body / endloop) inside a large,
//! NOP-padded executable region whose fall-through is funnelled to the capture
//! epilogue. The fragment is assembled with `llvm-mc -filetype=obj` and its
//! `.text` extracted with `llvm-objcopy`, so every PC-relative branch and loop
//! offset is already *relocated*; the resulting position-independent word blob
//! is fed byte-for-byte to BOTH the oracle (`tools/hexagon-diff/oracle_cf`) and
//! rax's `HexagonVcpu`. After execution we compare the full architectural state
//! including the loop control registers (LC0/LC1/SA0/SA1) and USR.
//!
//! Branch paths are made observable by writing distinct sentinel registers on
//! the taken vs. not-taken path before the two paths reconverge: the captured
//! sentinel reveals which path executed. Self-skips if the toolchain
//! (qemu-hexagon / llvm-mc / ld.lld / llvm-objcopy) is unavailable.

#![cfg(target_os = "linux")]

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use rax::backend::emulator::hexagon::HexagonVcpu;
use rax::config::{Endianness, HexagonIsa};
use rax::cpu::{CpuState, HexagonRegisters, VCpu, VcpuExit};

// ---------------------------------------------------------------------------
// Wire format -- must match tools/hexagon-diff/gen_oracle_cf.py byte for byte.
// HexState is 44 little-endian u32 (176 bytes). The per-case word array is
// NMAX_WORDS long (a control-flow fragment spans several packets).
// ---------------------------------------------------------------------------

const NREG: usize = 32;
const I_PRED: usize = 32;
const I_USR: usize = 33;
const I_M0: usize = 34;
const I_M1: usize = 35;
const I_GP: usize = 36;
const I_CS0: usize = 37;
const I_CS1: usize = 38;
const I_SA0: usize = 39;
const I_LC0: usize = 40;
const I_SA1: usize = 41;
const I_LC1: usize = 42;
const ST_WORDS: usize = 44;

const NMAX_WORDS: usize = 32;
const WIRE_MAGIC: u32 = 0x3158_4548; // 'H','E','X','1'
const CODE_ADDR: u32 = 0x1000;

#[derive(Clone, Copy)]
struct HexState {
    w: [u32; ST_WORDS],
}

impl HexState {
    fn zeroed() -> Self {
        HexState { w: [0; ST_WORDS] }
    }
}

fn which(prog: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(prog))
        .find(|c| c.is_file())
}

fn tools_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/hexagon-diff")
}

/// Build oracle_cf if needed; return (binary, `testslot` guest address). The
/// testslot address is where the fragment runs in the oracle, used to normalise
/// the (absolute, load-address-dependent) SA0/SA1 loop-start registers so they
/// can be compared against rax (which loads the fragment at CODE_ADDR). `None`
/// if the toolchain is unavailable.
fn oracle_path() -> Option<(PathBuf, u32)> {
    which("qemu-hexagon")?;
    which("llvm-mc")?;
    which("ld.lld")?;
    let dir = tools_dir();
    let bin = dir.join("oracle_cf");
    let src = dir.join("gen_oracle_cf.py");
    let need = match (bin.metadata(), src.metadata()) {
        (Ok(b), Ok(s)) => match (b.modified(), s.modified()) {
            (Ok(bm), Ok(sm)) => bm < sm,
            _ => true,
        },
        _ => true,
    };
    if need {
        let ok = Command::new("bash")
            .arg(dir.join("build_cf.sh"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            return None;
        }
    }
    if !bin.exists() {
        return None;
    }
    // Read the `testslot` address from the ELF symbol table.
    let nm = which("llvm-nm")
        .map(|_| "llvm-nm")
        .or(which("nm").map(|_| "nm"))?;
    let out = Command::new(nm).arg(&bin).output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let addr = text.lines().find_map(|l| {
        let mut it = l.split_whitespace();
        let a = it.next()?;
        let _ty = it.next()?;
        if it.next() == Some("testslot") {
            u32::from_str_radix(a, 16).ok()
        } else {
            None
        }
    })?;
    Some((bin, addr))
}

fn run_oracle(oracle: &PathBuf, cases: &[(Vec<u32>, HexState)]) -> Option<Vec<HexState>> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&WIRE_MAGIC.to_le_bytes());
    payload.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for (words, st) in cases {
        let nwords = words.len().min(NMAX_WORDS) as u32;
        payload.extend_from_slice(&nwords.to_le_bytes());
        for i in 0..NMAX_WORDS {
            payload.extend_from_slice(&words.get(i).copied().unwrap_or(0).to_le_bytes());
        }
        for v in &st.w {
            payload.extend_from_slice(&v.to_le_bytes());
        }
    }

    let mut child = Command::new("qemu-hexagon")
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
        let mut st = HexState::zeroed();
        for k in 0..ST_WORDS {
            st.w[k] = u32::from_le_bytes([
                out[off + 4 * k],
                out[off + 4 * k + 1],
                out[off + 4 * k + 2],
                out[off + 4 * k + 3],
            ]);
        }
        off += 176 + 8; // HexState + trapped + valid
        res.push(st);
    }
    Some(res)
}

// ---------------------------------------------------------------------------
// Assembler: assemble a whole multi-packet fragment (with local labels) to an
// object file, then extract its fully-relocated `.text` as a word stream. This
// is mandatory for control flow: `-show-encoding` leaves branch/loop offsets as
// unresolved fixups, whereas the object's `.text` has them applied.
// ---------------------------------------------------------------------------

/// Assemble one fragment (a multi-line packet sequence with local labels) and
/// return its relocated `.text` machine words. Cached by source text.
fn assemble_fragment(src: &str) -> Option<Vec<u32>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Vec<u32>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(v) = cache.lock().unwrap().get(src) {
        return Some(v.clone());
    }

    let dir = std::env::temp_dir();
    let pid = std::process::id();
    let nonce = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let stem = dir.join(format!("rax_hexcf_{pid}_{nonce}"));
    let asm = stem.with_extension("s");
    let obj = stem.with_extension("o");
    let bin = stem.with_extension("bin");

    std::fs::write(&asm, src.as_bytes()).ok()?;

    let mc_ok = Command::new("llvm-mc")
        .args(["-triple=hexagon", "-mcpu=hexagonv68", "-filetype=obj"])
        .arg(&asm)
        .arg("-o")
        .arg(&obj)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !mc_ok {
        let _ = std::fs::remove_file(&asm);
        return None;
    }

    // Extract the relocated .text as raw bytes.
    let objcopy = which("llvm-objcopy")
        .map(|_| "llvm-objcopy")
        .or_else(|| which("objcopy").map(|_| "objcopy"))?;
    let oc_ok = Command::new(objcopy)
        .args(["-O", "binary", "-j", ".text"])
        .arg(&obj)
        .arg(&bin)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    let result = if oc_ok {
        std::fs::read(&bin).ok().map(|bytes| {
            bytes
                .chunks(4)
                .filter(|c| c.len() == 4)
                .map(|c| {
                    c[0] as u32 | (c[1] as u32) << 8 | (c[2] as u32) << 16 | (c[3] as u32) << 24
                })
                .collect::<Vec<u32>>()
        })
    } else {
        None
    };

    let _ = std::fs::remove_file(&asm);
    let _ = std::fs::remove_file(&obj);
    let _ = std::fs::remove_file(&bin);

    let words = result?;
    if words.is_empty() || words.len() > NMAX_WORDS {
        return None;
    }
    cache.lock().unwrap().insert(src.to_string(), words.clone());
    Some(words)
}

static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn trap0_word() -> u32 {
    static W: OnceLock<u32> = OnceLock::new();
    *W.get_or_init(|| assemble_fragment("{ trap0(#0) }\n").expect("assemble trap0")[0])
}

// ---------------------------------------------------------------------------
// rax: run one fragment from an identical initial state.
// ---------------------------------------------------------------------------

fn rax_regs_from_state(st: &HexState) -> HexagonRegisters {
    let mut regs = HexagonRegisters::default();
    for i in 0..NREG {
        regs.r[i] = st.w[i];
    }
    for i in 0..4 {
        regs.p[i] = ((st.w[I_PRED] >> (8 * i)) & 0xff) as u8;
    }
    regs.c[8] = st.w[I_USR];
    regs.c[6] = st.w[I_M0];
    regs.c[7] = st.w[I_M1];
    regs.c[11] = st.w[I_GP];
    regs.c[12] = st.w[I_CS0];
    regs.c[13] = st.w[I_CS1];
    regs.c[0] = st.w[I_SA0];
    regs.c[1] = st.w[I_LC0];
    regs.c[2] = st.w[I_SA1];
    regs.c[3] = st.w[I_LC1];
    regs.set_pc(CODE_ADDR);
    regs
}

fn rax_pred_byte(regs: &HexagonRegisters) -> u32 {
    let mut v = 0u32;
    for i in 0..4 {
        v |= (regs.p[i] as u32) << (8 * i);
    }
    v
}

/// Run the fragment in rax: lay it out at CODE_ADDR, append a trap0 terminator
/// (every fragment reconverges and falls through into it), step until Shutdown.
/// Returns `None` if rax rejected an encoding or otherwise errored.
fn run_rax(words: &[u32], st: &HexState) -> Option<HexState> {
    let regions = vec![(GuestAddress(0), 0x10000usize)];
    let mem = Arc::new(GuestMemoryMmap::<()>::from_ranges(&regions).ok()?);

    let mut off = CODE_ADDR;
    for &w in words {
        mem.write_slice(&w.to_le_bytes(), GuestAddress(off as u64))
            .ok()?;
        off += 4;
    }
    mem.write_slice(&trap0_word().to_le_bytes(), GuestAddress(off as u64))
        .ok()?;

    let mut vcpu = HexagonVcpu::new(0, mem.clone(), HexagonIsa::V68, Endianness::Little);
    vcpu.set_state(&CpuState::hexagon(rax_regs_from_state(st)))
        .ok()?;

    // Hardware loops run the body many times; allow generous packet budget.
    let mut iters = 0;
    loop {
        iters += 1;
        if iters > 100_000 {
            return None;
        }
        match vcpu.run() {
            Ok(VcpuExit::Shutdown) => break,
            Ok(_) => return None,
            Err(_) => return None,
        }
    }

    let regs = match vcpu.get_state().ok()? {
        CpuState::Hexagon(s) => s.regs,
        _ => return None,
    };
    let mut out = HexState::zeroed();
    for i in 0..NREG {
        out.w[i] = regs.r[i];
    }
    out.w[I_PRED] = rax_pred_byte(&regs);
    out.w[I_USR] = regs.c[8];
    out.w[I_M0] = regs.c[6];
    out.w[I_M1] = regs.c[7];
    out.w[I_GP] = regs.c[11];
    out.w[I_CS0] = regs.c[12];
    out.w[I_CS1] = regs.c[13];
    out.w[I_SA0] = regs.c[0];
    out.w[I_LC0] = regs.c[1];
    out.w[I_SA1] = regs.c[2];
    out.w[I_LC1] = regs.c[3];
    Some(out)
}

// ---------------------------------------------------------------------------
// Comparison.
// ---------------------------------------------------------------------------

/// State fields that hold an *absolute code address* and so legitimately differ
/// between rax (fragment loaded at CODE_ADDR) and the oracle (loaded at
/// `testslot`). Such fields are base-normalised (offset within the fragment)
/// before comparison, which still verifies the address arithmetic is bit-exact
/// (PC-alignment masking, `PC + imm<<2` for loop starts, the link value for
/// calls). Every other field is compared raw.
#[derive(Clone, Copy, Default)]
struct AddrFields {
    /// SA0/SA1 are set by hardware-loop setup (`loop0`/`loop1`).
    sa: bool,
    /// r31 (LR) is an absolute return address after a `call`.
    lr: bool,
}

/// Compare the post-fragment architectural state. All GPRs, predicates, USR and
/// the loop counts (LC0/LC1) are compared raw; address-bearing fields named in
/// `af` are base-normalised first. PC is excluded (both runs reconverge to a
/// terminator, so the final PC is not meaningful).
fn diff_state(
    rax: &HexState,
    hw: &HexState,
    rax_base: u32,
    hw_base: u32,
    af: AddrFields,
) -> Vec<String> {
    let mut diffs = Vec::new();
    let norm = |v: u32, base: u32| v.wrapping_sub(base);
    for i in 0..NREG {
        if i == 31 && af.lr {
            let (r, h) = (norm(rax.w[i], rax_base), norm(hw.w[i], hw_base));
            if r != h {
                diffs.push(format!(
                    "r31/LR(rel): rax={r:#010x} hw={h:#010x}  (abs rax={:#010x} hw={:#010x})",
                    rax.w[i], hw.w[i]
                ));
            }
        } else if rax.w[i] != hw.w[i] {
            diffs.push(format!("r{i}: rax={:#010x} hw={:#010x}", rax.w[i], hw.w[i]));
        }
    }
    if rax.w[I_PRED] != hw.w[I_PRED] {
        diffs.push(format!(
            "P3:0: rax={:#010x} hw={:#010x}",
            rax.w[I_PRED], hw.w[I_PRED]
        ));
    }
    if rax.w[I_USR] != hw.w[I_USR] {
        diffs.push(format!(
            "USR: rax={:#010x} hw={:#010x}",
            rax.w[I_USR], hw.w[I_USR]
        ));
    }
    if rax.w[I_LC0] != hw.w[I_LC0] {
        diffs.push(format!(
            "LC0: rax={:#010x} hw={:#010x}",
            rax.w[I_LC0], hw.w[I_LC0]
        ));
    }
    if rax.w[I_LC1] != hw.w[I_LC1] {
        diffs.push(format!(
            "LC1: rax={:#010x} hw={:#010x}",
            rax.w[I_LC1], hw.w[I_LC1]
        ));
    }
    for (idx, name) in [(I_SA0, "SA0"), (I_SA1, "SA1")] {
        // An SA register the fragment never set retains its (zero) input value
        // in both runs; only a non-zero (code-address) SA is base-normalised, so
        // an unset SA still compares equal (0 == 0) under `af.sa`.
        let nz = |v: u32, base: u32| if v == 0 { 0 } else { norm(v, base) };
        let (r, h) = if af.sa {
            (nz(rax.w[idx], rax_base), nz(hw.w[idx], hw_base))
        } else {
            (rax.w[idx], hw.w[idx])
        };
        if r != h {
            let tag = if af.sa { "(rel)" } else { "" };
            diffs.push(format!(
                "{name}{tag}: rax={r:#010x} hw={h:#010x}  (abs rax={:#010x} hw={:#010x})",
                rax.w[idx], hw.w[idx]
            ));
        }
    }
    diffs
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
}

/// Generate an input state. GPRs get small values so that any accumulating loop
/// body stays well-defined; predicates are independently 0x00/0xff. SA/LC are
/// zeroed (the fragment installs them via its own loop-setup packet).
fn gen_input(rng: &mut Rng, small: bool) -> HexState {
    let mut st = HexState::zeroed();
    for i in 0..NREG {
        st.w[i] = if small {
            (rng.next() % 64) as u32
        } else {
            rng.next() as u32
        };
    }
    let mut pred = 0u32;
    for i in 0..4 {
        if rng.next() & 1 == 1 {
            pred |= 0xffu32 << (8 * i);
        }
    }
    st.w[I_PRED] = pred;
    st.w[I_USR] = 0;
    st
}

// ---------------------------------------------------------------------------
// Test driver.
// ---------------------------------------------------------------------------

struct Mismatch {
    label: String,
    src: String,
    detail: String,
}

/// Run a family: each (label, fragment-src) assembled into a relocated word
/// blob, driven with `n_inputs` random states, compared against the oracle.
/// `small` clamps the random GPRs (keeps accumulating loop bodies tame); `af`
/// names the address-bearing fields to base-normalise. Self-skips if the
/// toolchain is absent.
fn run_family(
    name: &str,
    cases: &[(&str, &str)],
    n_inputs: usize,
    seed: u64,
    small: bool,
    af: AddrFields,
) {
    let (oracle, slot_addr) = match oracle_path() {
        Some(p) => p,
        None => {
            eprintln!("[hexagon_cf_diff] {name}: toolchain unavailable -> skipping");
            return;
        }
    };

    // Assemble every fragment up front. We are past the toolchain-availability
    // check (oracle_path succeeded => llvm-mc/objcopy are present), so an
    // assembly failure here is a *test-authoring* bug (bad asm or a fragment
    // exceeding NMAX_WORDS), not an environment issue: fail loudly rather than
    // silently disabling the family.
    let mut blobs = Vec::new();
    for (label, src) in cases {
        match assemble_fragment(src) {
            Some(w) => blobs.push(w),
            None => panic!(
                "{name}: fragment '{label}' failed to assemble (bad asm or > {NMAX_WORDS} words):\n{src}"
            ),
        }
    }

    let mut rng = Rng::new(seed);
    let mut batch: Vec<(String, String, Vec<u32>, HexState)> = Vec::new();
    for ((label, src), words) in cases.iter().zip(blobs.iter()) {
        for _ in 0..n_inputs {
            batch.push((
                label.to_string(),
                src.to_string(),
                words.clone(),
                gen_input(&mut rng, small),
            ));
        }
    }

    let ocases: Vec<(Vec<u32>, HexState)> =
        batch.iter().map(|(_, _, w, s)| (w.clone(), *s)).collect();
    let outs = match run_oracle(&oracle, &ocases) {
        Some(o) => o,
        None => {
            eprintln!("[hexagon_cf_diff] {name}: oracle run failed -> skipping");
            return;
        }
    };
    assert_eq!(outs.len(), batch.len());

    let mut mismatches: Vec<Mismatch> = Vec::new();
    for ((label, src, words, st), hw) in batch.iter().zip(outs.iter()) {
        let rax = match run_rax(words, st) {
            Some(r) => r,
            None => {
                mismatches.push(Mismatch {
                    label: label.clone(),
                    src: src.clone(),
                    detail: "hw executed but rax rejected/errored (unimplemented?)".into(),
                });
                continue;
            }
        };
        let diffs = diff_state(&rax, hw, CODE_ADDR, slot_addr, af);
        if !diffs.is_empty() {
            mismatches.push(Mismatch {
                label: label.clone(),
                src: src.clone(),
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
            batch.len()
        );
        eprintln!("-- by fragment --");
        for (label, count) in &by_label {
            eprintln!("  {count:5}x  {label}");
        }
        eprintln!("-- first 25 examples --");
        for m in mismatches.iter().take(25) {
            eprintln!(
                "  [{}] {}\n        {}",
                m.label,
                m.src.replace('\n', " ; "),
                m.detail
            );
        }
        panic!(
            "{name}: {} divergences vs hardware oracle",
            mismatches.len()
        );
    }
}

// ---------------------------------------------------------------------------
// Families.
// ---------------------------------------------------------------------------

// Each fragment is a self-contained multi-packet sequence with local labels.
// Loops accumulate into r0 (and sometimes r1/r2); after the loop the final
// LC0/LC1/SA0/SA1 and accumulator are captured. GPRs are seeded small so an
// accumulating body never overflows in a way that obscures a mismatch (it would
// still be bit-exact, but small keeps any failure legible).
//
// The families cover the full rax control-flow surface: plain conditional
// jumps, unconditional call + `jumpr r31` return, and the J2_loop*/J2_endloop*
// hardware loops (the original set), plus the J4 compound compare-and-jump
// family (`J4_cmp*_*_jump[nv]_*`, both predicate-writing and new-value forms;
// see `diff_cf_cmpjump`), predicated direct/indirect calls
// (`J2_callt`/`callf`/`callrt`/`callrf`), the jumpr-compare-zero family, the
// software-pipelined `sp*loop0` loops, jumpset/hintjr and `pause`
// (see `diff_cf_calls_loops`).

const AF_NONE: AddrFields = AddrFields {
    sa: false,
    lr: false,
};
const AF_SA: AddrFields = AddrFields {
    sa: true,
    lr: false,
};
const AF_LR: AddrFields = AddrFields {
    sa: false,
    lr: true,
};

/// J2_loop0i + J2_endloop0: immediate trip count. Verifies the body ran exactly
/// `count` times (accumulator), SA0 = body address (base-normalised), and LC0
/// ends at 1 (hardware decrements while LC0>1, stopping at 1).
#[test]
fn cf_loop0i_basic() {
    let cases = &[
        (
            "loop0i_3",
            "{ r0 = #0 }\n\
             { loop0(.Lb,#3) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop0\n",
        ),
        (
            "loop0i_1",
            "{ r0 = #0 }\n\
             { loop0(.Lb,#1) }\n\
             .Lb:\n\
             { r0 = add(r0,#7) }:endloop0\n",
        ),
        (
            "loop0i_10",
            "{ r0 = #0 }\n\
             { loop0(.Lb,#10) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop0\n",
        ),
        (
            "loop0i_multipkt_body",
            // Multi-PACKET body (the body itself is two packets; the second is
            // the single-instruction endloop packet). Verifies the loop spans
            // more than one packet per iteration.
            "{ r0 = #0 }\n\
             { loop0(.Lb,#4) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }\n\
             { r0 = add(r0,#10) }:endloop0\n",
        ),
    ];
    run_family("loop0i_basic", cases, 6, 0xC0F0, true, AF_SA);
}

/// J2_loop0r + endloop0: trip count from a register (r5). r5 is seeded 0..63;
/// the loop runs a register-defined number of times.
#[test]
fn cf_loop0r_basic() {
    let cases = &[
        (
            "loop0r_fixed",
            "{ r5 = #5 }\n\
             { r0 = #0 }\n\
             { loop0(.Lb,r5) }\n\
             .Lb:\n\
             { r0 = add(r0,#3) }:endloop0\n",
        ),
        (
            "loop0r_reg",
            "{ r0 = #0 }\n\
             { loop0(.Lb,r5) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop0\n",
        ),
    ];
    run_family("loop0r_basic", cases, 10, 0xC0F1, true, AF_SA);
}

/// J2_loop1i + J2_endloop1: the outer loop register set (LC1/SA1) exercised on
/// its own. A single-level loop1 with `:endloop1` body.
#[test]
fn cf_loop1i_basic() {
    let cases = &[
        (
            "loop1i_4",
            "{ r0 = #0 }\n\
             { loop1(.Lb,#4) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop1\n",
        ),
        (
            "loop1i_1",
            "{ r0 = #0 }\n\
             { loop1(.Lb,#1) }\n\
             .Lb:\n\
             { r0 = add(r0,#9) }:endloop1\n",
        ),
    ];
    run_family("loop1i_basic", cases, 6, 0xC0F8, true, AF_SA);
}

/// J2_loop1r + endloop1: outer loop with a register trip count (r5).
#[test]
fn cf_loop1r_basic() {
    let cases = &[(
        "loop1r_reg",
        "{ r0 = #0 }\n\
         { loop1(.Lb,r5) }\n\
         .Lb:\n\
         { r0 = add(r0,#1) }:endloop1\n",
    )];
    run_family("loop1r_basic", cases, 10, 0xC0F9, true, AF_SA);
}

/// J2_loop1i + J2_loop0i + J2_endloop01: nested hardware loops. The combined
/// `:endloop01` in the innermost packet handles BOTH levels (decrement LC0
/// first; only when LC0 reaches 1 does it take the LC1 back-edge). Verifies the
/// combined endloop semantics and the final LC0/LC1/SA0/SA1.
#[test]
fn cf_nested_loop01() {
    let cases = &[
        (
            "nest_2x3",
            "{ r0 = #0 }\n\
             { loop1(.Louter,#2) }\n\
             .Louter:\n\
             { loop0(.Linner,#3) }\n\
             .Linner:\n\
             { r0 = add(r0,#1) }:endloop01\n",
        ),
        (
            "nest_3x2",
            "{ r0 = #0 }\n\
             { loop1(.Louter,#3) }\n\
             .Louter:\n\
             { loop0(.Linner,#2) }\n\
             .Linner:\n\
             { r0 = add(r0,#5) }:endloop01\n",
        ),
        (
            "nest_4x4_multibody",
            "{ r0 = #0 }\n\
             { loop1(.Louter,#4) }\n\
             .Louter:\n\
             { loop0(.Linner,#4) }\n\
             .Linner:\n\
             { r0 = add(r0,#1); r1 = add(r1,#1) }:endloop01\n",
        ),
    ];
    run_family("nested_loop01", cases, 5, 0xC0F2, true, AF_SA);
}

/// Separate inner `:endloop0` + outer `:endloop1` (not the combined endloop01):
/// the inner body ends with endloop0, an outer body ends with endloop1. Exercises
/// the two back-edge paths as distinct packets.
#[test]
fn cf_nested_loop0_loop1() {
    let cases = &[(
        "nest0_1",
        "{ r0 = #0 }\n\
         { loop1(.Louter,#2) }\n\
         .Louter:\n\
         { loop0(.Linner,#3) }\n\
         .Linner:\n\
         { r0 = add(r0,#1) }:endloop0\n\
         { r2 = add(r2,#1) }:endloop1\n",
    )];
    run_family("nested_loop0_loop1", cases, 5, 0xC0F3, true, AF_SA);
}

/// Conditional direct jumps made observable: each path writes a distinct
/// sentinel to r0 before reconverging at `.Ldone`. The captured r0 reveals
/// which path the hardware took; rax must agree. (Plain `if (pN) jump` forms
/// only -- compound compare-and-jump is not yet decoded by rax.)
#[test]
fn cf_cond_jump() {
    let cases = &[
        (
            "jump_eq_t",
            "{ p0 = cmp.eq(r1,r2) }\n\
             { if (p0) jump .Ltaken }\n\
             { r0 = #100 }\n\
             { jump .Ldone }\n\
             .Ltaken:\n\
             { r0 = #200 }\n\
             .Ldone:\n",
        ),
        (
            "jump_eq_f",
            "{ p0 = cmp.eq(r1,r2) }\n\
             { if (!p0) jump .Ltaken }\n\
             { r0 = #100 }\n\
             { jump .Ldone }\n\
             .Ltaken:\n\
             { r0 = #200 }\n\
             .Ldone:\n",
        ),
        (
            "jump_gt",
            "{ p0 = cmp.gt(r1,r2) }\n\
             { if (p0) jump .Ltaken }\n\
             { r0 = #100 }\n\
             { jump .Ldone }\n\
             .Ltaken:\n\
             { r0 = #200 }\n\
             .Ldone:\n",
        ),
        (
            "jump_gtu",
            "{ p0 = cmp.gtu(r1,r2) }\n\
             { if (p0) jump .Ltaken }\n\
             { r0 = #100 }\n\
             { jump .Ldone }\n\
             .Ltaken:\n\
             { r0 = #200 }\n\
             .Ldone:\n",
        ),
        (
            "jump_p1",
            // Use predicate P1 to exercise a non-P0 condition register.
            "{ p1 = cmp.eq(r1,r2) }\n\
             { if (p1) jump .Ltaken }\n\
             { r0 = #100 }\n\
             { jump .Ldone }\n\
             .Ltaken:\n\
             { r0 = #200 }\n\
             .Ldone:\n",
        ),
    ];
    run_family("cond_jump", cases, 12, 0xC0F4, false, AF_NONE);
}

/// Conditional jump driven by a predicate from the *input* state (no compare):
/// `if (p0) jump`/`if (!p0) jump` with p0 randomly 0x00/0xff. This isolates the
/// branch-decode + predicate-read path from the compare logic.
#[test]
fn cf_cond_jump_input_pred() {
    let cases = &[
        (
            "in_p0_t",
            "{ if (p0) jump .Ltaken }\n\
             { r0 = #7 }\n\
             { jump .Ldone }\n\
             .Ltaken:\n\
             { r0 = #9 }\n\
             .Ldone:\n",
        ),
        (
            "in_p0_f",
            "{ if (!p0) jump .Ltaken }\n\
             { r0 = #7 }\n\
             { jump .Ldone }\n\
             .Ltaken:\n\
             { r0 = #9 }\n\
             .Ldone:\n",
        ),
    ];
    run_family("cond_jump_input_pred", cases, 16, 0xC0FA, false, AF_NONE);
}

/// Unconditional call + `jumpr r31` return: the call links the return address
/// into LR (r31), the callee runs and returns indirectly. We capture r31 (LR,
/// base-normalised) and the accumulator, verifying both the link value and the
/// register-indirect jump. (Predicated calls `J2_callt`/`J2_callf` are not yet
/// decoded by rax, so they are omitted.)
#[test]
fn cf_call_return() {
    let cases = &[
        (
            "call_ret",
            "{ r0 = #1 }\n\
             { call .Lsub }\n\
             { r0 = add(r0,#10) }\n\
             { jump .Ldone }\n\
             .Lsub:\n\
             { r0 = add(r0,#100) }\n\
             { jumpr r31 }\n\
             .Ldone:\n",
        ),
        (
            "call_ret_multibody",
            // Callee mutates several (base-independent) GPRs and returns; the
            // post-state must reflect the callee body plus the linked LR.
            "{ r0 = #1 }\n\
             { call .Lsub }\n\
             { r0 = add(r0,r6) }\n\
             { jump .Ldone }\n\
             .Lsub:\n\
             { r6 = add(r6,#3) }\n\
             { r7 = add(r7,#-1) }\n\
             { jumpr r31 }\n\
             .Ldone:\n",
        ),
    ];
    run_family("call_return", cases, 12, 0xC0F5, false, AF_LR);
}

/// J4 compound compare-and-jump (`J4_<cmp>_<cond>_jump[nv]_<hint>`): a single
/// instruction that both compares and branches. Two families are exercised:
///
///  * Predicate-writing `_jump` forms (`tp0`/`fp0`/`tp1`/`fp1`): a fused
///    `pN = cmp...; if ([!]pN.new) jump` — verifies BOTH the branch direction
///    (via the r0 sentinel) AND that the named predicate (P0/P1) is written with
///    the raw compare result (captured in P3:0).
///  * New-value `_jumpnv` forms: a producing insn in the same packet feeds a
///    `r5.new` operand to the compare; the branch is taken on the compare result
///    and NO architectural predicate is written.
///
/// Inputs span both taken and not-taken (small random GPRs make the eq/gt
/// compares sometimes true and sometimes false across the seeds).
#[test]
fn diff_cf_cmpjump() {
    let cases = &[
        // ---- predicate-writing forms (write P0/P1, then branch) ----
        (
            "cmpeq_tp0",
            "{ p0 = cmp.eq(r1,r2); if (p0.new) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "cmpeq_fp0",
            "{ p0 = cmp.eq(r1,r2); if (!p0.new) jump:nt .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "cmpgt_tp1",
            "{ p1 = cmp.gt(r1,r2); if (p1.new) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "cmpgtu_fp1",
            "{ p1 = cmp.gtu(r1,r2); if (!p1.new) jump:nt .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "cmpeqi_tp0",
            "{ p0 = cmp.eq(r1,#3); if (p0.new) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "cmpgti_fp0",
            "{ p0 = cmp.gt(r1,#5); if (!p0.new) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "cmpgtui_tp1",
            "{ p1 = cmp.gtu(r1,#4); if (p1.new) jump:nt .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "cmpeqn1_tp0",
            "{ p0 = cmp.eq(r1,#-1); if (p0.new) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "cmpgtn1_fp1",
            "{ p1 = cmp.gt(r1,#-1); if (!p1.new) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "tstbit0_tp0",
            "{ p0 = tstbit(r1,#0); if (p0.new) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        // ---- new-value compare-and-branch (`_jumpnv`); producer in the packet ----
        (
            "nv_cmpeq_t",
            "{ r5 = add(r4,#1); if (cmp.eq(r5.new,r6)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpeq_f",
            "{ r5 = add(r4,#1); if (!cmp.eq(r5.new,r6)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpgt_t",
            "{ r5 = add(r4,#1); if (cmp.gt(r5.new,r6)) jump:nt .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpgtu_f",
            "{ r5 = add(r4,#1); if (!cmp.gtu(r5.new,r6)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            // cmplt is encoded as cmp.gt with operands swapped (Rt, Ns.new).
            "nv_cmplt_t",
            "{ r5 = add(r4,#1); if (cmp.gt(r6,r5.new)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpltu_t",
            "{ r5 = add(r4,#1); if (cmp.gtu(r6,r5.new)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpeqi_t",
            "{ r5 = add(r4,#1); if (cmp.eq(r5.new,#3)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpgti_f",
            "{ r5 = add(r4,#1); if (!cmp.gt(r5.new,#5)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpgtui_t",
            "{ r5 = add(r4,#1); if (cmp.gtu(r5.new,#4)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpeqn1_t",
            "{ r5 = add(r4,#-1); if (cmp.eq(r5.new,#-1)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_cmpgtn1_f",
            "{ r5 = add(r4,#1); if (!cmp.gt(r5.new,#-1)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_tstbit0_t",
            "{ r5 = add(r4,#1); if (tstbit(r5.new,#0)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
        (
            "nv_tstbit0_f",
            "{ r5 = add(r4,#1); if (!tstbit(r5.new,#0)) jump:t .Lt }\n\
             { r0 = #100 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #200 }\n\
             .Ld:\n",
        ),
    ];
    // Small inputs keep the eq/gt compares straddling true/false across seeds.
    run_family("cmpjump", cases, 16, 0xC0FB, true, AF_NONE);
}

/// J2 conditional calls, jumpr-compare-zero, software-pipelined loops, and the
/// J4 jumpset/hintjumpr specials.
///
///  * Predicated calls `J2_callt`/`J2_callf` (+ `callr` variants): the call (and
///    its r31 link) happens only when the predicate holds; r31 (base-normalised)
///    and the callee accumulator are captured.
///  * `jumprz`/`jumprnz`/`jumprgtez`/`jumprltez`: `if (cmp.<>(Rs,#0)) jumpr Rs`.
///  * `p3 = sp<N>loop0(...)`: sets SA0/LC0, USR.LPCFG=N and presets P3=0; the
///    `endloop0` back-edges set the pipeline predicate P3 when LPCFG drains.
///  * `J4_jumpseti`/`J4_jumpsetr`: `Rd = <#u6|Rs> ; jump`.
#[test]
fn diff_cf_calls_loops() {
    // Predicated calls: the r0 sentinel reveals whether the call (callee adds
    // #100) executed vs. the fall-through (adds #10). r31 is normalised to a
    // base-independent value at the reconverge point so the (path-dependent,
    // absolute) link address never pollutes the raw comparison; the link value
    // itself is verified by the unconditional `cf_call_return` family. AF_NONE.
    let calls = &[
        (
            "callt_taken_or_not",
            "{ p0 = cmp.eq(r1,r2) }\n\
             { if (p0) call .Lsub }\n\
             { r0 = add(r0,#10) }\n\
             { jump .Ld }\n\
             .Lsub:\n\
             { r0 = add(r0,#100) }\n\
             { jumpr r31 }\n\
             .Ld:\n\
             { r31 = #0 }\n",
        ),
        (
            "callf_taken_or_not",
            "{ p0 = cmp.eq(r1,r2) }\n\
             { if (!p0) call .Lsub }\n\
             { r0 = add(r0,#10) }\n\
             { jump .Ld }\n\
             .Lsub:\n\
             { r0 = add(r0,#100) }\n\
             { jumpr r31 }\n\
             .Ld:\n\
             { r31 = #0 }\n",
        ),
    ];
    run_family("cf_calls", calls, 12, 0xC0FC, false, AF_NONE);

    // Predicated indirect call `if (Pu) callr Rs`: callee address is computed
    // PC-relative (base-dependent) into r4, so r4 is normalised away by clearing
    // it after reconverge; the r0 sentinel proves the call path.
    let callr = &[
        (
            "callrt",
            "{ p0 = cmp.eq(r1,r2) }\n\
             { r4 = add(pc,#16) }\n\
             { if (p0) callr r4 }\n\
             { r0 = add(r0,#10) }\n\
             { jump .Ld }\n\
             { r0 = add(r0,#100) }\n\
             { jumpr r31 }\n\
             .Ld:\n\
             { r4 = #0 ; r31 = #0 }\n",
        ),
        (
            "callrf",
            "{ p0 = cmp.eq(r1,r2) }\n\
             { r4 = add(pc,#16) }\n\
             { if (!p0) callr r4 }\n\
             { r0 = add(r0,#10) }\n\
             { jump .Ld }\n\
             { r0 = add(r0,#100) }\n\
             { jumpr r31 }\n\
             .Ld:\n\
             { r4 = #0 ; r31 = #0 }\n",
        ),
    ];
    run_family("cf_callr", callr, 12, 0xC0FC, false, AF_NONE);

    // The J2 jumpr-compare-zero family are DIRECT conditional jumps whose
    // condition is a register compared to zero. The r0 sentinel reveals the
    // path; the random r5 straddles zero / both signs across seeds. The `pt`
    // (jump:t) hint variants are also exercised to confirm the hint has no
    // architectural effect. AF_NONE.
    let jumprz = &[
        (
            // `if (r5!=#0) jump:nt` == J2_jumprz (jump-if-register-true).
            "jumprz_ne",
            "{ r0 = #100 }\n\
             { if (r5!=#0) jump:nt .Lt }\n\
             { r0 = #1 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #2 }\n\
             .Ld:\n",
        ),
        (
            // `if (r5==#0) jump:nt` == J2_jumprnz (jump-if-register-false).
            "jumprnz_eq",
            "{ r0 = #100 }\n\
             { if (r5==#0) jump:nt .Lt }\n\
             { r0 = #1 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #2 }\n\
             .Ld:\n",
        ),
        (
            "jumprgtez",
            "{ r0 = #100 }\n\
             { if (r5>=#0) jump:nt .Lt }\n\
             { r0 = #1 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #2 }\n\
             .Ld:\n",
        ),
        (
            "jumprltez",
            "{ r0 = #100 }\n\
             { if (r5<=#0) jump:nt .Lt }\n\
             { r0 = #1 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #2 }\n\
             .Ld:\n",
        ),
        (
            // `pt` (jump:t) prediction-hint variant: must give identical results.
            "jumprz_ne_pt",
            "{ r0 = #100 }\n\
             { if (r5!=#0) jump:t .Lt }\n\
             { r0 = #1 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #2 }\n\
             .Ld:\n",
        ),
        (
            "jumprgtez_pt",
            "{ r0 = #100 }\n\
             { if (r5>=#0) jump:t .Lt }\n\
             { r0 = #1 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #2 }\n\
             .Ld:\n",
        ),
    ];
    // Signed (non-small) inputs give r5 both signs and an occasional zero.
    run_family("cf_jumprz", jumprz, 24, 0xC0FD, false, AF_NONE);

    let ploop = &[
        (
            "sp1loop0_3",
            "{ r0 = #0 }\n\
             { p3 = sp1loop0(.Lb,#3) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop0\n",
        ),
        (
            "sp2loop0_4",
            "{ r0 = #0 }\n\
             { p3 = sp2loop0(.Lb,#4) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop0\n",
        ),
        (
            "sp3loop0_5",
            "{ r0 = #0 }\n\
             { p3 = sp3loop0(.Lb,#5) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop0\n",
        ),
        (
            "sp1loop0_r",
            "{ r0 = #0 }\n\
             { p3 = sp1loop0(.Lb,r5) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop0\n",
        ),
        (
            "sp2loop0_r",
            "{ r0 = #0 }\n\
             { p3 = sp2loop0(.Lb,r5) }\n\
             .Lb:\n\
             { r0 = add(r0,#1) }:endloop0\n",
        ),
    ];
    // Small inputs: r5 trip counts in 0..63 (loop body stays tame).
    run_family("cf_ploop", ploop, 10, 0xC0FE, true, AF_SA);

    let jumpset = &[
        (
            "jumpseti",
            "{ r3 = #42 ; jump .Lt }\n\
             { r0 = #1 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #2 }\n\
             .Ld:\n",
        ),
        (
            "jumpsetr",
            "{ r3 = r4 ; jump .Lt }\n\
             { r0 = #1 }\n\
             { jump .Ld }\n\
             .Lt:\n\
             { r0 = #2 }\n\
             .Ld:\n",
        ),
    ];
    run_family("cf_jumpset", jumpset, 8, 0xC0FF, false, AF_NONE);

    // Miscellaneous specials: `pause` (architectural NOP) and `hintjr` (a
    // jump-register hint that behaves exactly like `jumpr Rs`). For hintjr the
    // target is computed PC-relative into r5 so the fragment reconverges; r5 is
    // cleared afterwards to keep the (base-dependent) target out of the raw
    // comparison.
    let misc = &[
        (
            // pause is a NOP: the surrounding adds must accumulate normally.
            "pause_nop",
            "{ r0 = add(r1,#5) }\n\
             { pause(#1) }\n\
             { r0 = add(r0,#1) }\n",
        ),
        (
            "hintjr",
            "{ r5 = add(pc,#8) }\n\
             { hintjr(r5) }\n\
             { r0 = #99 }\n\
             { r0 = #7 }\n\
             { r5 = #0 }\n",
        ),
    ];
    run_family("cf_misc", misc, 8, 0xC100, false, AF_NONE);
}

/// A plain conditional jump *inside* a hardware-loop body: each iteration picks
/// one of two accumulations, then both paths reconverge before `:endloop0`.
/// Stresses branch + loop-back-edge interaction. (The compare and the jump are
/// in separate packets -- a compound cmp-jump would not decode in rax.)
#[test]
fn cf_loop_with_branch() {
    let cases = &[(
        "loop_branch",
        "{ r0 = #0 }\n\
         { r3 = #0 }\n\
         { loop0(.Lb,#6) }\n\
         .Lb:\n\
         { p0 = cmp.gt(r3,#2) }\n\
         { if (p0) jump .Lhi }\n\
         { r0 = add(r0,#1) }\n\
         { jump .Lcont }\n\
         .Lhi:\n\
         { r0 = add(r0,#10) }\n\
         .Lcont:\n\
         { r3 = add(r3,#1) }:endloop0\n",
    )];
    run_family("loop_with_branch", cases, 6, 0xC0F6, true, AF_SA);
}
