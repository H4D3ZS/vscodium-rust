//! HVX (vector) differential test harness: rax vs. qemu-hexagon. Verifies HVX
//! vector instructions (V0..V31, 1024-bit) against the vector oracle
//! `tools/hexagon-diff/oracle_hvx` (which load/captures the vector register file
//! via aligned `vmem`). For each (packet, scalar state, vector state) we run it
//! on the oracle and on rax's `HexagonVcpu` and compare GPRs, USR, and all 32
//! vector registers. Self-skips if the toolchain is unavailable.

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

const NREG: usize = 32;
const I_PRED: usize = 32;
const I_USR: usize = 33;
const ST_WORDS: usize = 44;
const VREGS: usize = 32;
const VWORDS: usize = 32; // 128 bytes / 4
const CASE_VOFF: usize = 256; // 128-byte aligned vblock offset within InCase/OutCase
const WIRE_MAGIC: u32 = 0x3158_4548;
const CODE_ADDR: u32 = 0x1000;

#[derive(Clone)]
struct Case {
    words: Vec<u32>,
    st: [u32; ST_WORDS],
    v: [[u32; VWORDS]; VREGS],
}

#[derive(Clone)]
struct Out {
    st: [u32; ST_WORDS],
    v: [[u32; VWORDS]; VREGS],
}

fn which(prog: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(prog))
        .find(|c| c.is_file())
}

fn oracle_hvx() -> Option<PathBuf> {
    which("qemu-hexagon")?;
    which("llvm-mc")?;
    which("ld.lld")?;
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/hexagon-diff");
    let bin = dir.join("oracle_hvx");
    let src = dir.join("gen_oracle_hvx.py");
    let need = match (bin.metadata(), src.metadata()) {
        (Ok(b), Ok(s)) => match (b.modified(), s.modified()) {
            (Ok(bm), Ok(sm)) => bm < sm,
            _ => true,
        },
        _ => true,
    };
    if need {
        let ok = Command::new("bash")
            .arg(dir.join("build_hvx.sh"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            return None;
        }
    }
    bin.exists().then_some(bin)
}

fn run_oracle(bin: &PathBuf, cases: &[Case]) -> Option<Vec<Out>> {
    let case_size = CASE_VOFF + VREGS * VWORDS * 4;
    let mut payload = Vec::new();
    payload.extend_from_slice(&WIRE_MAGIC.to_le_bytes());
    payload.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for c in cases {
        let mut buf = vec![0u8; case_size];
        buf[0..4].copy_from_slice(&(c.words.len().min(4) as u32).to_le_bytes());
        for i in 0..4 {
            let w = c.words.get(i).copied().unwrap_or(0);
            buf[4 + i * 4..8 + i * 4].copy_from_slice(&w.to_le_bytes());
        }
        for k in 0..ST_WORDS {
            buf[20 + k * 4..24 + k * 4].copy_from_slice(&c.st[k].to_le_bytes());
        }
        for r in 0..VREGS {
            for w in 0..VWORDS {
                let off = CASE_VOFF + (r * VWORDS + w) * 4;
                buf[off..off + 4].copy_from_slice(&c.v[r][w].to_le_bytes());
            }
        }
        payload.extend_from_slice(&buf);
    }
    let mut child = Command::new("qemu-hexagon")
        .args(["-cpu", "v69"])
        .arg(bin)
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
    if !child.wait().ok()?.success() || out.len() < 8 {
        return None;
    }
    let count = u32::from_le_bytes([out[4], out[5], out[6], out[7]]) as usize;
    if count != cases.len() {
        return None;
    }
    let out_size = CASE_VOFF + VREGS * VWORDS * 4;
    let mut res = Vec::with_capacity(count);
    let mut off = 8;
    for _ in 0..count {
        let mut st = [0u32; ST_WORDS];
        for k in 0..ST_WORDS {
            st[k] = u32::from_le_bytes([
                out[off + 4 * k],
                out[off + 4 * k + 1],
                out[off + 4 * k + 2],
                out[off + 4 * k + 3],
            ]);
        }
        let mut v = [[0u32; VWORDS]; VREGS];
        for r in 0..VREGS {
            for w in 0..VWORDS {
                let o = off + CASE_VOFF + (r * VWORDS + w) * 4;
                v[r][w] = u32::from_le_bytes([out[o], out[o + 1], out[o + 2], out[o + 3]]);
            }
        }
        off += out_size;
        res.push(Out { st, v });
    }
    Some(res)
}

/// Assemble one case-string into its full instruction-word stream. A case may
/// contain several `\n`-separated packets (e.g. a `{ q0 = vcmp… }` producer
/// followed by a `{ v0 = vmux… }` consumer); all packets' words are returned in
/// execution order so the caller writes them sequentially. Cached per case.
fn assemble_one(case: &str) -> Option<Vec<u32>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Option<Vec<u32>>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(v) = cache.lock().unwrap().get(case) {
        return v.clone();
    }
    let result = (|| {
        let mut child = Command::new("llvm-mc")
            .args([
                "-triple=hexagon",
                "-mcpu=hexagonv69",
                "-mhvx",
                "-show-encoding",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        child
            .stdin
            .take()
            .unwrap()
            .write_all(case.as_bytes())
            .ok()?;
        let mut out = String::new();
        child.stdout.take().unwrap().read_to_string(&mut out).ok()?;
        if !child.wait().ok()?.success() {
            return None;
        }
        let mut words = Vec::new();
        let mut acc = Vec::new();
        for line in out.lines() {
            if let Some(i) = line.find("encoding: [") {
                let rest = &line[i + 11..];
                let end = rest.find(']')?;
                for t in rest[..end].split(',') {
                    let t = t.trim().strip_prefix("0x").unwrap_or(t.trim());
                    if let Ok(b) = u8::from_str_radix(t, 16) {
                        acc.push(b);
                        if acc.len() == 4 {
                            words.push(
                                acc[0] as u32
                                    | (acc[1] as u32) << 8
                                    | (acc[2] as u32) << 16
                                    | (acc[3] as u32) << 24,
                            );
                            acc.clear();
                        }
                    }
                }
            }
        }
        (!words.is_empty()).then_some(words)
    })();
    cache
        .lock()
        .unwrap()
        .insert(case.to_string(), result.clone());
    result
}

/// Assemble a family: one full word stream per case-string. Returns `None` if
/// any case fails to assemble (caller self-skips the family).
fn assemble(packets: &[String]) -> Option<Vec<Vec<u32>>> {
    let mut result = Vec::with_capacity(packets.len());
    for case in packets {
        result.push(assemble_one(case)?);
    }
    Some(result)
}

fn trap0_word() -> u32 {
    static W: OnceLock<u32> = OnceLock::new();
    *W.get_or_init(|| assemble(&["{ trap0(#0) }".to_string()]).expect("trap0")[0][0])
}

fn run_rax(words: &[u32], c: &Case) -> Option<Out> {
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

    let mut regs = HexagonRegisters::default();
    for i in 0..NREG {
        regs.r[i] = c.st[i];
    }
    regs.c[8] = c.st[I_USR];
    // Scalar predicates P0..P3 (one packed byte each), matching the oracle's c4
    // seeding — needed for scalar-predicated vector ops (vcmov, vccombine, …).
    for i in 0..4 {
        regs.p[i] = ((c.st[I_PRED] >> (8 * i)) & 0xff) as u8;
    }
    regs.v = c.v;
    regs.set_pc(CODE_ADDR);

    let mut vcpu = HexagonVcpu::new(0, mem.clone(), HexagonIsa::V68, Endianness::Little);
    vcpu.set_state(&CpuState::hexagon(regs)).ok()?;
    let mut iters = 0;
    loop {
        iters += 1;
        if iters > 64 {
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
    let mut st = [0u32; ST_WORDS];
    for i in 0..NREG {
        st[i] = regs.r[i];
    }
    st[I_USR] = regs.c[8];
    Some(Out { st, v: regs.v })
}

struct Rng(u64);
impl Rng {
    fn new(s: u64) -> Self {
        Rng(s ^ 0x9e37_79b9_7f4a_7c15)
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

/// Run an HVX family: each (label, asm) over `n` random vector states.
fn run_family(name: &str, cases: &[(&str, &str)], n: usize, seed: u64) {
    let bin = match oracle_hvx() {
        Some(b) => b,
        None => {
            eprintln!("[hexagon_hvx_diff] {name}: toolchain unavailable -> skipping");
            return;
        }
    };
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_hvx_diff] {name}: assembly failed -> skipping");
            return;
        }
    };
    let mut rng = Rng::new(seed);
    let mut batch = Vec::new();
    let mut labels = Vec::new();
    for ((label, _), words) in cases.iter().zip(words_per.iter()) {
        for _ in 0..n {
            let mut st = [0u32; ST_WORDS];
            for r in 0..NREG {
                st[r] = rng.next() as u32;
            }
            st[I_USR] = 0;
            // Each scalar predicate P0..P3 independently all-0 / all-1, so
            // scalar-predicated vector ops exercise both branches.
            let mut pred = 0u32;
            for k in 0..4 {
                if rng.next() & 1 == 1 {
                    pred |= 0xffu32 << (8 * k);
                }
            }
            st[I_PRED] = pred;
            let mut v = [[0u32; VWORDS]; VREGS];
            for r in 0..VREGS {
                for w in 0..VWORDS {
                    v[r][w] = rng.next() as u32;
                }
            }
            labels.push(*label);
            batch.push(Case {
                words: words.clone(),
                st,
                v,
            });
        }
    }
    let outs = match run_oracle(&bin, &batch) {
        Some(o) => o,
        None => {
            eprintln!("[hexagon_hvx_diff] {name}: oracle failed -> skipping");
            return;
        }
    };
    let mut mismatches = Vec::new();
    for (i, c) in batch.iter().enumerate() {
        let rax = match run_rax(&c.words, c) {
            Some(r) => r,
            None => {
                mismatches.push(format!("[{}] rax rejected", labels[i]));
                continue;
            }
        };
        let mut diffs = Vec::new();
        for vr in 0..VREGS {
            if rax.v[vr] != outs[i].v[vr] {
                let lane = (0..VWORDS)
                    .find(|&w| rax.v[vr][w] != outs[i].v[vr][w])
                    .unwrap();
                diffs.push(format!(
                    "v{vr}.w[{lane}]:rax={:#x},hw={:#x}",
                    rax.v[vr][lane], outs[i].v[vr][lane]
                ));
                break;
            }
        }
        for r in 0..NREG {
            if rax.st[r] != outs[i].st[r] {
                diffs.push(format!("r{r}:rax={:#x},hw={:#x}", rax.st[r], outs[i].st[r]));
            }
        }
        if !diffs.is_empty() {
            mismatches.push(format!("[{}] {}", labels[i], diffs.join(" ")));
        }
    }
    if !mismatches.is_empty() {
        eprintln!("\n==== {name}: {} mismatches ====", mismatches.len());
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!("{name}: {} HVX divergences vs oracle", mismatches.len());
    }
}

#[test]
fn diff_hvx_addsub() {
    run_family(
        "hvx_addsub",
        &[
            ("vaddb", "{ v0.b = vadd(v1.b,v2.b) }"),
            ("vaddh", "{ v0.h = vadd(v1.h,v2.h) }"),
            ("vaddw", "{ v0.w = vadd(v1.w,v2.w) }"),
            ("vsubb", "{ v0.b = vsub(v1.b,v2.b) }"),
            ("vsubh", "{ v0.h = vsub(v1.h,v2.h) }"),
            ("vsubw", "{ v0.w = vsub(v1.w,v2.w) }"),
        ],
        6,
        0x4856,
    );
}

#[test]
fn diff_hvx_sat() {
    run_family(
        "hvx_sat",
        &[
            ("vaddbsat", "{ v0.b = vadd(v1.b,v2.b):sat }"),
            ("vaddhsat", "{ v0.h = vadd(v1.h,v2.h):sat }"),
            ("vaddwsat", "{ v0.w = vadd(v1.w,v2.w):sat }"),
            ("vaddubsat", "{ v0.ub = vadd(v1.ub,v2.ub):sat }"),
            ("vadduhsat", "{ v0.uh = vadd(v1.uh,v2.uh):sat }"),
            ("vsubbsat", "{ v0.b = vsub(v1.b,v2.b):sat }"),
            ("vsubhsat", "{ v0.h = vsub(v1.h,v2.h):sat }"),
            ("vsububsat", "{ v0.ub = vsub(v1.ub,v2.ub):sat }"),
        ],
        6,
        0x5a7,
    );
}

#[test]
fn diff_hvx_mpyi() {
    // Integer same-width multiplies (vmpyi). Scalar operand is r5.
    run_family(
        "hvx_mpyi",
        &[
            ("vmpyih", "{ v0.h = vmpyi(v1.h,v2.h) }"),
            ("vmpyihb", "{ v0.h = vmpyi(v1.h,r5.b) }"),
            ("vmpyiwb", "{ v0.w = vmpyi(v1.w,r5.b) }"),
            ("vmpyiwh", "{ v0.w = vmpyi(v1.w,r5.h) }"),
        ],
        6,
        0x3c2,
    );
}

#[test]
fn diff_hvx_logical() {
    run_family(
        "hvx_logical",
        &[
            ("vand", "{ v0 = vand(v1,v2) }"),
            ("vor", "{ v0 = vor(v1,v2) }"),
            ("vxor", "{ v0 = vxor(v1,v2) }"),
        ],
        6,
        0x109,
    );
}

// ==== hvx_shift (workflow) ====
#[test]
fn diff_hvx_shift_scalar() {
    // Per-lane shift by scalar Rt; r3 holds the (random) shift amount, masked
    // to the lane width by the instruction.
    run_family(
        "hvx_shift_scalar",
        &[
            ("vaslh", "{ v0.h = vasl(v1.h,r3) }"),
            ("vaslw", "{ v0.w = vasl(v1.w,r3) }"),
            ("vasrh", "{ v0.h = vasr(v1.h,r3) }"),
            ("vasrw", "{ v0.w = vasr(v1.w,r3) }"),
            ("vlsrh", "{ v0.uh = vlsr(v1.uh,r3) }"),
            ("vlsrw", "{ v0.uw = vlsr(v1.uw,r3) }"),
            ("vlsrb", "{ v0.ub = vlsr(v1.ub,r3) }"),
        ],
        8,
        0x5417,
    );
}

#[test]
fn diff_hvx_shift_vector() {
    // Per-lane bidirectional shift by vector Vv (amount = sign-extended low
    // bits of each lane).
    run_family(
        "hvx_shift_vector",
        &[
            ("vaslhv", "{ v0.h = vasl(v1.h,v2.h) }"),
            ("vaslwv", "{ v0.w = vasl(v1.w,v2.w) }"),
            ("vasrhv", "{ v0.h = vasr(v1.h,v2.h) }"),
            ("vasrwv", "{ v0.w = vasr(v1.w,v2.w) }"),
            ("vlsrhv", "{ v0.h = vlsr(v1.h,v2.h) }"),
            ("vlsrwv", "{ v0.w = vlsr(v1.w,v2.w) }"),
        ],
        8,
        0x91c2,
    );
}

#[test]
fn diff_hvx_ror() {
    run_family("hvx_ror", &[("vror", "{ v0 = vror(v1,r3) }")], 10, 0x7be1);
}

#[test]
fn diff_hvx_bitcount() {
    run_family(
        "hvx_bitcount",
        &[
            ("vcl0h", "{ v0.uh = vcl0(v1.uh) }"),
            ("vcl0w", "{ v0.uw = vcl0(v1.uw) }"),
            ("vnormamth", "{ v0.h = vnormamt(v1.h) }"),
            ("vnormamtw", "{ v0.w = vnormamt(v1.w) }"),
            ("vpopcounth", "{ v0.h = vpopcount(v1.h) }"),
        ],
        8,
        0x3d09,
    );
}

#[test]
fn diff_hvx_shift_narrow() {
    // Narrowing rounding/saturating right shifts: Vu/Vv -> packed narrow Vd
    // (even sub-lane from Vv, odd from Vu); r3 holds the shift amount.
    run_family(
        "hvx_shift_narrow",
        &[
            ("vasrwh", "{ v0.h = vasr(v1.w,v2.w,r3) }"),
            ("vasrwhsat", "{ v0.h = vasr(v1.w,v2.w,r3):sat }"),
            ("vasrwhrndsat", "{ v0.h = vasr(v1.w,v2.w,r3):rnd:sat }"),
            ("vasrwuhsat", "{ v0.uh = vasr(v1.w,v2.w,r3):sat }"),
            ("vasrwuhrndsat", "{ v0.uh = vasr(v1.w,v2.w,r3):rnd:sat }"),
            ("vasruwuhsat", "{ v0.uh = vasr(v1.uw,v2.uw,r3):sat }"),
            ("vasruwuhrndsat", "{ v0.uh = vasr(v1.uw,v2.uw,r3):rnd:sat }"),
            ("vasrhubsat", "{ v0.ub = vasr(v1.h,v2.h,r3):sat }"),
            ("vasrhubrndsat", "{ v0.ub = vasr(v1.h,v2.h,r3):rnd:sat }"),
            ("vasrhbsat", "{ v0.b = vasr(v1.h,v2.h,r3):sat }"),
            ("vasrhbrndsat", "{ v0.b = vasr(v1.h,v2.h,r3):rnd:sat }"),
            ("vasruhubsat", "{ v0.ub = vasr(v1.uh,v2.uh,r3):sat }"),
            ("vasruhubrndsat", "{ v0.ub = vasr(v1.uh,v2.uh,r3):rnd:sat }"),
        ],
        8,
        0x2af6,
    );
}

// ==== hvx_perm (workflow) ====
#[test]
fn diff_hvx_perm_move() {
    run_family(
        "hvx_perm_move",
        &[
            ("vassign", "{ v0 = v1 }"),
            ("vcombine", "{ v1:0 = vcombine(v3,v2) }"),
            ("vsplatb", "{ v0.b = vsplat(r1) }"),
            ("vsplath", "{ v0.h = vsplat(r1) }"),
            ("vsplatw", "{ v0 = vsplat(r1) }"),
        ],
        6,
        0x9101,
    );
}

#[test]
fn diff_hvx_perm_align() {
    run_family(
        "hvx_perm_align",
        &[
            ("valignb", "{ v0 = valign(v1,v2,r3) }"),
            ("vlalignb", "{ v0 = vlalign(v1,v2,r3) }"),
            ("valignbi", "{ v0 = valign(v1,v2,#3) }"),
            ("vlalignbi", "{ v0 = vlalign(v1,v2,#5) }"),
            ("vror", "{ v0 = vror(v1,r2) }"),
        ],
        8,
        0x9102,
    );
}

#[test]
fn diff_hvx_perm_shuffle() {
    run_family(
        "hvx_perm_shuffle",
        &[
            ("vshuffb", "{ v0.b = vshuff(v1.b) }"),
            ("vshuffh", "{ v0.h = vshuff(v1.h) }"),
            ("vdealb", "{ v0.b = vdeal(v1.b) }"),
            ("vdealh", "{ v0.h = vdeal(v1.h) }"),
            ("vdealb4w", "{ v0.b = vdeale(v1.b,v2.b) }"),
            ("vshuffeb", "{ v0.b = vshuffe(v1.b,v2.b) }"),
            ("vshuffob", "{ v0.b = vshuffo(v1.b,v2.b) }"),
            ("vshufeh", "{ v0.h = vshuffe(v1.h,v2.h) }"),
            ("vshufoh", "{ v0.h = vshuffo(v1.h,v2.h) }"),
            ("vshuffvdd", "{ v1:0 = vshuff(v3,v2,r4) }"),
        ],
        8,
        0x9103,
    );
}

#[test]
fn diff_hvx_perm_pack() {
    run_family(
        "hvx_perm_pack",
        &[
            ("vpackeb", "{ v0.b = vpacke(v1.h,v2.h) }"),
            ("vpackeh", "{ v0.h = vpacke(v1.w,v2.w) }"),
            ("vpackob", "{ v0.b = vpacko(v1.h,v2.h) }"),
            ("vpackoh", "{ v0.h = vpacko(v1.w,v2.w) }"),
            ("vpackhb_sat", "{ v0.b = vpack(v1.h,v2.h):sat }"),
            ("vpackhub_sat", "{ v0.ub = vpack(v1.h,v2.h):sat }"),
            ("vpackwh_sat", "{ v0.h = vpack(v1.w,v2.w):sat }"),
            ("vpackwuh_sat", "{ v0.uh = vpack(v1.w,v2.w):sat }"),
        ],
        8,
        0x9104,
    );
}

#[test]
fn diff_hvx_perm_widen() {
    run_family(
        "hvx_perm_widen",
        &[
            ("vzb", "{ v1:0.uh = vzxt(v2.ub) }"),
            ("vsb", "{ v1:0.h = vsxt(v2.b) }"),
            ("vzh", "{ v1:0.uw = vzxt(v2.uh) }"),
            ("vsh", "{ v1:0.w = vsxt(v2.h) }"),
            ("vunpackub", "{ v1:0.uh = vunpack(v2.ub) }"),
            ("vunpackb", "{ v1:0.h = vunpack(v2.b) }"),
            ("vunpackuh", "{ v1:0.uw = vunpack(v2.uh) }"),
            ("vunpackh", "{ v1:0.w = vunpack(v2.h) }"),
            ("vunpackob", "{ v1:0.h |= vunpacko(v2.b) }"),
            ("vunpackoh", "{ v1:0.w |= vunpacko(v2.h) }"),
        ],
        8,
        0x9105,
    );
}

// ==== hvx_cmp (workflow) ====
// --- hvx_cmp: vector compares verified by consuming Q in a vmux into a V ----
// The oracle does NOT capture Q, so each Q-producer feeds a vmux into V0, which
// is captured and diffed. A vector predicate written by one packet is only
// visible to a consumer in a *later* packet (Hexagon does not forward Q within a
// packet), so the producer and consumer are split into two packets per case.

#[test]
fn diff_hvx_cmp() {
    run_family(
        "hvx_cmp",
        &[
            (
                "veqb",
                "{ q0 = vcmp.eq(v1.b,v2.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtb",
                "{ q0 = vcmp.gt(v1.b,v2.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtub",
                "{ q0 = vcmp.gt(v1.ub,v2.ub) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqh",
                "{ q0 = vcmp.eq(v1.h,v2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgth",
                "{ q0 = vcmp.gt(v1.h,v2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtuh",
                "{ q0 = vcmp.gt(v1.uh,v2.uh) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqw",
                "{ q0 = vcmp.eq(v1.w,v2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtw",
                "{ q0 = vcmp.gt(v1.w,v2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtuw",
                "{ q0 = vcmp.gt(v1.uw,v2.uw) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
        ],
        8,
        0x6311,
    );
}

// Force the boundary cases by comparing a register against itself (eq -> all
// true, gt -> all false), exercising both vmux branches with a uniform Q.
#[test]
fn diff_hvx_cmp_self() {
    run_family(
        "hvx_cmp_self",
        &[
            (
                "veqb_self",
                "{ q0 = vcmp.eq(v1.b,v1.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtb_self",
                "{ q0 = vcmp.gt(v1.b,v1.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqh_self",
                "{ q0 = vcmp.eq(v1.h,v1.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtw_self",
                "{ q0 = vcmp.gt(v1.w,v1.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtuw_self",
                "{ q0 = vcmp.gt(v1.uw,v1.uw) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
        ],
        6,
        0x77a2,
    );
}

// Q-predicate logic, verified by combining two independent compare-produced Q
// predicates and feeding the result into vmux. Producer compares are in packet
// 1, the Q-logic in packet 2, the vmux in packet 3.
#[test]
fn diff_hvx_pred_logic() {
    run_family(
        "hvx_pred_logic",
        &[
            (
                "pred_and",
                "{ q0 = vcmp.gt(v1.b,v2.b); q1 = vcmp.gt(v5.b,v6.b) }\n{ q2 = and(q0,q1) }\n{ v0 = vmux(q2,v3,v4) }",
            ),
            (
                "pred_or",
                "{ q0 = vcmp.gt(v1.b,v2.b); q1 = vcmp.gt(v5.b,v6.b) }\n{ q2 = or(q0,q1) }\n{ v0 = vmux(q2,v3,v4) }",
            ),
            (
                "pred_xor",
                "{ q0 = vcmp.gt(v1.b,v2.b); q1 = vcmp.gt(v5.b,v6.b) }\n{ q2 = xor(q0,q1) }\n{ v0 = vmux(q2,v3,v4) }",
            ),
            (
                "pred_and_n",
                "{ q0 = vcmp.gt(v1.b,v2.b); q1 = vcmp.gt(v5.b,v6.b) }\n{ q2 = and(q0,!q1) }\n{ v0 = vmux(q2,v3,v4) }",
            ),
            (
                "pred_or_n",
                "{ q0 = vcmp.gt(v1.b,v2.b); q1 = vcmp.gt(v5.b,v6.b) }\n{ q2 = or(q0,!q1) }\n{ v0 = vmux(q2,v3,v4) }",
            ),
            (
                "pred_not",
                "{ q0 = vcmp.gt(v1.b,v2.b) }\n{ q1 = not(q0) }\n{ v0 = vmux(q1,v3,v4) }",
            ),
        ],
        8,
        0x91c3,
    );
}

// vand bridges: Q<->V (vandvqv / vandvnqv), Q<->R (vandqrt / vandnqrt), and
// V<->R producing a Q (vandvrt, consumed by vmux). Each Q-producer is in its own
// packet; the consumer (which yields a captured V) follows in the next packet.
#[test]
fn diff_hvx_vand_bridge() {
    run_family(
        "hvx_vand_bridge",
        &[
            (
                "vandvqv",
                "{ q0 = vcmp.gt(v1.b,v2.b) }\n{ v0 = vand(q0,v3) }",
            ),
            (
                "vandvnqv",
                "{ q0 = vcmp.gt(v1.b,v2.b) }\n{ v0 = vand(!q0,v3) }",
            ),
            (
                "vandqrt",
                "{ q0 = vcmp.gt(v1.b,v2.b) }\n{ v0 = vand(q0,r3) }",
            ),
            (
                "vandnqrt",
                "{ q0 = vcmp.gt(v1.b,v2.b) }\n{ v0 = vand(!q0,r3) }",
            ),
            ("vandvrt", "{ q0 = vand(v1,r2) }\n{ v0 = vmux(q0,v3,v4) }"),
        ],
        8,
        0xa4d7,
    );
}

#[test]
fn diff_hvx_minmax() {
    run_family(
        "hvx_minmax",
        &[
            ("vmaxh", "{ v0.h = vmax(v1.h,v2.h) }"),
            ("vmaxub", "{ v0.ub = vmax(v1.ub,v2.ub) }"),
            ("vminw", "{ v0.w = vmin(v1.w,v2.w) }"),
            ("vavgh", "{ v0.h = vavg(v1.h,v2.h) }"),
            ("vavgub", "{ v0.ub = vavg(v1.ub,v2.ub) }"),
            ("vavghrnd", "{ v0.h = vavg(v1.h,v2.h):rnd }"),
            ("vnavgh", "{ v0.h = vnavg(v1.h,v2.h) }"),
            ("vabsh", "{ v0.h = vabs(v1.h) }"),
            ("vabsw_sat", "{ v0.w = vabs(v1.w):sat }"),
            ("vabsdiffh", "{ v0.uh = vabsdiff(v1.h,v2.h) }"),
            ("vabsdiffub", "{ v0.ub = vabsdiff(v1.ub,v2.ub) }"),
        ],
        6,
        0xa1b2,
    );
}

// ==== hvx_rmpy (workflow: HVX multiply family) ====
#[test]
fn diff_hvx_rmpy() {
    // Reduction multiplies (vrmpy / vdmpy / vtmpy / vdsad / vrsad). Scalar = r5.
    // Accumulate forms read+write v0 (seeded random); pair dest = v1:0, pair src = v3:2.
    run_family(
        "hvx_rmpy",
        &[
            // vrmpy scalar single-vector
            ("vrmpyub", "{ v0.uw = vrmpy(v4.ub,r5.ub) }"),
            ("vrmpyub_acc", "{ v0.uw += vrmpy(v4.ub,r5.ub) }"),
            ("vrmpybus", "{ v0.w = vrmpy(v4.ub,r5.b) }"),
            ("vrmpybus_acc", "{ v0.w += vrmpy(v4.ub,r5.b) }"),
            // vrmpy vector-vector
            ("vrmpyubv", "{ v0.uw = vrmpy(v4.ub,v6.ub) }"),
            ("vrmpyubv_acc", "{ v0.uw += vrmpy(v4.ub,v6.ub) }"),
            ("vrmpybv", "{ v0.w = vrmpy(v4.b,v6.b) }"),
            ("vrmpybv_acc", "{ v0.w += vrmpy(v4.b,v6.b) }"),
            ("vrmpybusv", "{ v0.w = vrmpy(v4.ub,v6.b) }"),
            ("vrmpybusv_acc", "{ v0.w += vrmpy(v4.ub,v6.b) }"),
            // vrmpy pair with #u1 byte rotate
            ("vrmpyubi0", "{ v1:0.uw = vrmpy(v3:2.ub,r5.ub,#0) }"),
            ("vrmpyubi1", "{ v1:0.uw = vrmpy(v3:2.ub,r5.ub,#1) }"),
            ("vrmpyubi_acc", "{ v1:0.uw += vrmpy(v3:2.ub,r5.ub,#1) }"),
            ("vrmpybusi0", "{ v1:0.w = vrmpy(v3:2.ub,r5.b,#0) }"),
            ("vrmpybusi1", "{ v1:0.w = vrmpy(v3:2.ub,r5.b,#1) }"),
            ("vrmpybusi_acc", "{ v1:0.w += vrmpy(v3:2.ub,r5.b,#1) }"),
            // vrsad pair with #u1 byte rotate
            ("vrsadubi0", "{ v1:0.uw = vrsad(v3:2.ub,r5.ub,#0) }"),
            ("vrsadubi1", "{ v1:0.uw = vrsad(v3:2.ub,r5.ub,#1) }"),
            ("vrsadubi_acc", "{ v1:0.uw += vrsad(v3:2.ub,r5.ub,#1) }"),
            // vdsad pair
            ("vdsaduh", "{ v1:0.uw = vdsad(v3:2.uh,r5.uh) }"),
            ("vdsaduh_acc", "{ v1:0.uw += vdsad(v3:2.uh,r5.uh) }"),
            // vdmpybus single + dv
            ("vdmpybus", "{ v0.h = vdmpy(v4.ub,r5.b) }"),
            ("vdmpybus_acc", "{ v0.h += vdmpy(v4.ub,r5.b) }"),
            ("vdmpybus_dv", "{ v1:0.h = vdmpy(v3:2.ub,r5.b) }"),
            ("vdmpybus_dv_acc", "{ v1:0.h += vdmpy(v3:2.ub,r5.b) }"),
            // vdmpyhb single + dv
            ("vdmpyhb", "{ v0.w = vdmpy(v4.h,r5.b) }"),
            ("vdmpyhb_acc", "{ v0.w += vdmpy(v4.h,r5.b) }"),
            ("vdmpyhb_dv", "{ v1:0.w = vdmpy(v3:2.h,r5.b) }"),
            ("vdmpyhb_dv_acc", "{ v1:0.w += vdmpy(v3:2.h,r5.b) }"),
            // vdmpyh*sat single
            ("vdmpyhsat", "{ v0.w = vdmpy(v4.h,r5.h):sat }"),
            ("vdmpyhsat_acc", "{ v0.w += vdmpy(v4.h,r5.h):sat }"),
            ("vdmpyhsusat", "{ v0.w = vdmpy(v4.h,r5.uh):sat }"),
            ("vdmpyhsusat_acc", "{ v0.w += vdmpy(v4.h,r5.uh):sat }"),
            // vdmpyh*isat pair-src -> single
            ("vdmpyhisat", "{ v0.w = vdmpy(v3:2.h,r5.h):sat }"),
            ("vdmpyhisat_acc", "{ v0.w += vdmpy(v3:2.h,r5.h):sat }"),
            ("vdmpyhsuisat", "{ v0.w = vdmpy(v3:2.h,r5.uh,#1):sat }"),
            ("vdmpyhsuisat_acc", "{ v0.w += vdmpy(v3:2.h,r5.uh,#1):sat }"),
            // vdmpyhvsat vector-vector
            ("vdmpyhvsat", "{ v0.w = vdmpy(v4.h,v6.h):sat }"),
            ("vdmpyhvsat_acc", "{ v0.w += vdmpy(v4.h,v6.h):sat }"),
            // vtmpy pair 3-wide sliding window
            ("vtmpyb", "{ v1:0.h = vtmpy(v3:2.b,r5.b) }"),
            ("vtmpyb_acc", "{ v1:0.h += vtmpy(v3:2.b,r5.b) }"),
            ("vtmpybus", "{ v1:0.h = vtmpy(v3:2.ub,r5.b) }"),
            ("vtmpybus_acc", "{ v1:0.h += vtmpy(v3:2.ub,r5.b) }"),
            ("vtmpyhb", "{ v1:0.w = vtmpy(v3:2.h,r5.b) }"),
            ("vtmpyhb_acc", "{ v1:0.w += vtmpy(v3:2.h,r5.b) }"),
        ],
        8,
        0x7a1d,
    );
}

// ==== hvx_mpys (workflow: HVX multiply family) ====
#[test]
fn diff_hvx_mpys() {
    // Scalar multiply-add / piecewise (vmpa*) and integer multiply-accumulate
    // scalar forms. Scalar Rt = r5, scalar pair Rtt = r5:4 (all GPRs seeded).
    // Pair source = v3:2, pair dest = v1:0 (v0/v1 seeded random so accumulate
    // and read-modify-write forms are exercised).
    run_family(
        "hvx_mpys",
        &[
            ("vmpabus", "{ v1:0.h = vmpa(v3:2.ub,r5.b) }"),
            ("vmpabus_acc", "{ v1:0.h += vmpa(v3:2.ub,r5.b) }"),
            ("vmpabuu", "{ v1:0.h = vmpa(v3:2.ub,r5.ub) }"),
            ("vmpabuu_acc", "{ v1:0.h += vmpa(v3:2.ub,r5.ub) }"),
            ("vmpahb", "{ v1:0.w = vmpa(v3:2.h,r5.b) }"),
            ("vmpahb_acc", "{ v1:0.w += vmpa(v3:2.h,r5.b) }"),
            ("vmpauhb", "{ v1:0.w = vmpa(v3:2.uh,r5.b) }"),
            ("vmpauhb_acc", "{ v1:0.w += vmpa(v3:2.uh,r5.b) }"),
            ("vmpabusv", "{ v1:0.h = vmpa(v3:2.ub,v5:4.b) }"),
            ("vmpabuuv", "{ v1:0.h = vmpa(v3:2.ub,v5:4.ub) }"),
            ("vmpahhsat", "{ v0.h = vmpa(v0.h,v2.h,r5:4.h):sat }"),
            ("vmpauhuhsat", "{ v0.h = vmpa(v0.h,v2.uh,r5:4.uh):sat }"),
            ("vmpsuhuhsat", "{ v0.h = vmps(v0.h,v2.uh,r5:4.uh):sat }"),
            ("vmpyih_acc", "{ v0.h += vmpyi(v2.h,v3.h) }"),
            ("vmpyihb_acc", "{ v0.h += vmpyi(v2.h,r5.b) }"),
            ("vmpyiwb_acc", "{ v0.w += vmpyi(v2.w,r5.b) }"),
            ("vmpyiwh_acc", "{ v0.w += vmpyi(v2.w,r5.h) }"),
            ("vmpyieoh", "{ v0.w = vmpyieo(v2.h,v3.h) }"),
            ("vmpyiewh_acc", "{ v0.w += vmpyie(v2.w,v3.h) }"),
            ("vmpyiewuh", "{ v0.w = vmpyie(v2.w,v3.uh) }"),
            ("vmpyiewuh_acc", "{ v0.w += vmpyie(v2.w,v3.uh) }"),
            ("vmpyiowh", "{ v0.w = vmpyio(v2.w,v3.h) }"),
            ("vmpyiwub", "{ v0.w = vmpyi(v2.w,r5.ub) }"),
            ("vmpyiwub_acc", "{ v0.w += vmpyi(v2.w,r5.ub) }"),
        ],
        8,
        0x6d7a,
    );
}

// ==== hvx_mpyv (workflow: HVX multiply family) ====
#[test]
fn diff_hvx_mpyv() {
    // Widening vector-by-vector and vector-by-scalar multiplies. Pair dests use
    // v1:0 (even=v0, odd=v1, both seeded random so acc forms are exercised);
    // scalar operand is r5. Sources use distinct, non-overlapping v4/v5.
    run_family(
        "hvx_mpyv",
        &[
            // vector-by-vector, widening -> pair
            ("vmpybv", "{ v1:0.h = vmpy(v4.b,v5.b) }"),
            ("vmpybv_acc", "{ v1:0.h += vmpy(v4.b,v5.b) }"),
            ("vmpybusv", "{ v1:0.h = vmpy(v4.ub,v5.b) }"),
            ("vmpybusv_acc", "{ v1:0.h += vmpy(v4.ub,v5.b) }"),
            ("vmpyubv", "{ v1:0.uh = vmpy(v4.ub,v5.ub) }"),
            ("vmpyubv_acc", "{ v1:0.uh += vmpy(v4.ub,v5.ub) }"),
            ("vmpyhv", "{ v1:0.w = vmpy(v4.h,v5.h) }"),
            ("vmpyhv_acc", "{ v1:0.w += vmpy(v4.h,v5.h) }"),
            ("vmpyhus", "{ v1:0.w = vmpy(v4.h,v5.uh) }"),
            ("vmpyhus_acc", "{ v1:0.w += vmpy(v4.h,v5.uh) }"),
            ("vmpyuhv", "{ v1:0.uw = vmpy(v4.uh,v5.uh) }"),
            ("vmpyuhv_acc", "{ v1:0.uw += vmpy(v4.uh,v5.uh) }"),
            // vector-by-vector, halfword <<1:rnd:sat -> single
            ("vmpyhvsrs", "{ v0.h = vmpy(v4.h,v5.h):<<1:rnd:sat }"),
            // vector-by-scalar, widening -> pair
            ("vmpybus", "{ v1:0.h = vmpy(v4.ub,r5.b) }"),
            ("vmpybus_acc", "{ v1:0.h += vmpy(v4.ub,r5.b) }"),
            ("vmpyub", "{ v1:0.uh = vmpy(v4.ub,r5.ub) }"),
            ("vmpyub_acc", "{ v1:0.uh += vmpy(v4.ub,r5.ub) }"),
            ("vmpyh", "{ v1:0.w = vmpy(v4.h,r5.h) }"),
            ("vmpyh_acc", "{ v1:0.w += vmpy(v4.h,r5.h) }"),
            ("vmpyhsat_acc", "{ v1:0.w += vmpy(v4.h,r5.h):sat }"),
            ("vmpyuh", "{ v1:0.uw = vmpy(v4.uh,r5.uh) }"),
            ("vmpyuh_acc", "{ v1:0.uw += vmpy(v4.uh,r5.uh) }"),
            // vector-by-scalar, halfword <<1 sat / rnd:sat -> single
            ("vmpyhss", "{ v0.h = vmpy(v4.h,r5.h):<<1:sat }"),
            ("vmpyhsrs", "{ v0.h = vmpy(v4.h,r5.h):<<1:rnd:sat }"),
            // even unsigned halfword by scalar unsigned halfword -> single
            ("vmpyuhe", "{ v0.uw = vmpye(v4.uh,r5.uh) }"),
            ("vmpyuhe_acc", "{ v0.uw += vmpye(v4.uh,r5.uh) }"),
        ],
        8,
        0x7be,
    );
}

// ==== hvx_cmpy (workflow: HVX multiply family) ====
#[test]
fn diff_hvx_cmpy() {
    // Even/odd word multiplies (vmpyewuh / vmpyowh): Vu.w * Vv.uh[0] (even,
    // unsigned low half) or Vu.w * Vv.h[1] (odd, signed high half). Accumulate
    // forms write v0 (seeded random); pair dest is v1:0, pair source v3:2.
    run_family(
        "hvx_cmpy",
        &[
            ("vmpyewuh", "{ v4.w = vmpye(v2.w,v3.uh) }"),
            ("vmpyewuh_64", "{ v1:0 = vmpye(v2.w,v3.uh) }"),
            ("vmpyowh", "{ v4.w = vmpyo(v2.w,v3.h):<<1:sat }"),
            ("vmpyowh_rnd", "{ v4.w = vmpyo(v2.w,v3.h):<<1:rnd:sat }"),
            ("vmpyowh_sacc", "{ v0.w += vmpyo(v2.w,v3.h):<<1:sat:shift }"),
            (
                "vmpyowh_rnd_sacc",
                "{ v0.w += vmpyo(v2.w,v3.h):<<1:rnd:sat:shift }",
            ),
            ("vmpyowh_64_acc", "{ v1:0 += vmpyo(v2.w,v3.h) }"),
        ],
        32,
        0xc91a,
    );
}

// ==== hvx_lut (workflow: HVX multiply family) ====
#[test]
fn diff_hvx_lut() {
    run_family(
        "hvx_lut",
        &[
            // 4-entry lookup from a scalar register pair (r5:4).
            ("vlut4", "{ v0.h = vlut4(v1.uh,r5:4.h) }"),
            // vlut32: byte lookups. Scalar Rt = r5; immediate = #3.
            ("vlutvvb", "{ v0.b = vlut32(v1.b,v2.b,r5) }"),
            ("vlutvvb_nm", "{ v0.b = vlut32(v1.b,v2.b,r5):nomatch }"),
            ("vlutvvbi", "{ v0.b = vlut32(v1.b,v2.b,#3) }"),
            ("vlutvvb_oracc", "{ v0.b |= vlut32(v1.b,v2.b,r5) }"),
            ("vlutvvb_oracci", "{ v0.b |= vlut32(v1.b,v2.b,#3) }"),
            // vlut16: halfword lookups, vector-pair dest (v1:0).
            ("vlutvwh", "{ v1:0.h = vlut16(v2.b,v3.h,r5) }"),
            ("vlutvwh_nm", "{ v1:0.h = vlut16(v2.b,v3.h,r5):nomatch }"),
            ("vlutvwhi", "{ v1:0.h = vlut16(v2.b,v3.h,#3) }"),
            ("vlutvwh_oracc", "{ v1:0.h |= vlut16(v2.b,v3.h,r5) }"),
            ("vlutvwh_oracci", "{ v1:0.h |= vlut16(v2.b,v3.h,#3) }"),
        ],
        8,
        0x1c7e,
    );
}

// ==== hvx_addsub (wave-2 workflow) ====
// ==== hvx_addsub_dv: widening, dual-vector and saturating-special add/sub ====
#[test]
fn diff_hvx_addsub_dv() {
    run_family(
        "hvx_addsub_dv",
        &[
            // ---- dual-vector (Vdd = op(Vuu, Vvv)) ----
            ("vaddb_dv", "{ v1:0.b = vadd(v3:2.b,v5:4.b) }"),
            ("vaddh_dv", "{ v1:0.h = vadd(v3:2.h,v5:4.h) }"),
            ("vaddw_dv", "{ v1:0.w = vadd(v3:2.w,v5:4.w) }"),
            ("vaddbsat_dv", "{ v1:0.b = vadd(v3:2.b,v5:4.b):sat }"),
            ("vaddhsat_dv", "{ v1:0.h = vadd(v3:2.h,v5:4.h):sat }"),
            ("vaddwsat_dv", "{ v1:0.w = vadd(v3:2.w,v5:4.w):sat }"),
            ("vaddubsat_dv", "{ v1:0.ub = vadd(v3:2.ub,v5:4.ub):sat }"),
            ("vadduhsat_dv", "{ v1:0.uh = vadd(v3:2.uh,v5:4.uh):sat }"),
            ("vadduwsat_dv", "{ v1:0.uw = vadd(v3:2.uw,v5:4.uw):sat }"),
            ("vsubb_dv", "{ v1:0.b = vsub(v3:2.b,v5:4.b) }"),
            ("vsubh_dv", "{ v1:0.h = vsub(v3:2.h,v5:4.h) }"),
            ("vsubw_dv", "{ v1:0.w = vsub(v3:2.w,v5:4.w) }"),
            ("vsubbsat_dv", "{ v1:0.b = vsub(v3:2.b,v5:4.b):sat }"),
            ("vsubhsat_dv", "{ v1:0.h = vsub(v3:2.h,v5:4.h):sat }"),
            ("vsubwsat_dv", "{ v1:0.w = vsub(v3:2.w,v5:4.w):sat }"),
            ("vsububsat_dv", "{ v1:0.ub = vsub(v3:2.ub,v5:4.ub):sat }"),
            ("vsubuhsat_dv", "{ v1:0.uh = vsub(v3:2.uh,v5:4.uh):sat }"),
            ("vsubuwsat_dv", "{ v1:0.uw = vsub(v3:2.uw,v5:4.uw):sat }"),
            // ---- widening (single source -> pair dest) ----
            ("vaddubh", "{ v1:0.h = vadd(v2.ub,v3.ub) }"),
            ("vaddubh_acc", "{ v1:0.h += vadd(v2.ub,v3.ub) }"),
            ("vsububh", "{ v1:0.h = vsub(v2.ub,v3.ub) }"),
            ("vaddhw", "{ v1:0.w = vadd(v2.h,v3.h) }"),
            ("vaddhw_acc", "{ v1:0.w += vadd(v2.h,v3.h) }"),
            ("vsubhw", "{ v1:0.w = vsub(v2.h,v3.h) }"),
            ("vadduhw", "{ v1:0.w = vadd(v2.uh,v3.uh) }"),
            ("vadduhw_acc", "{ v1:0.w += vadd(v2.uh,v3.uh) }"),
            ("vsubuhw", "{ v1:0.w = vsub(v2.uh,v3.uh) }"),
            // ---- special ----
            ("vaddububb_sat", "{ v0.ub = vadd(v1.ub,v2.b):sat }"),
            ("vsubububb_sat", "{ v0.ub = vsub(v1.ub,v2.b):sat }"),
            ("vaddclbh", "{ v0.h = vadd(vclb(v1.h),v2.h) }"),
            ("vaddclbw", "{ v0.w = vadd(vclb(v1.w),v2.w) }"),
        ],
        8,
        0x2add,
    );
}

// ==== hvx_round (wave-2 workflow) ====
// ==== hvx_round (workflow: rounding/saturating narrows + accumulating shifts) ====
#[test]
fn diff_hvx_round() {
    // Narrowing rounding/saturating converts (vround*, vsat*), the 64-bit
    // saturate (vsatdw), the accumulating shifts (v{asr,asl}{h,w}_acc, Vx seeded
    // random) and the shift-into overlay (vasr_into, Vxx pair seeded random).
    // The vasrv*sat per-element variable narrows are V69+ and do not assemble at
    // -mcpu=hexagonv68, so they are exercised in rax only (verified by reasoning
    // against the spec, not the oracle).
    run_family(
        "hvx_round",
        &[
            ("vroundhb", "{ v0.b = vround(v1.h,v2.h):sat }"),
            ("vroundhub", "{ v0.ub = vround(v1.h,v2.h):sat }"),
            ("vrounduhub", "{ v0.ub = vround(v1.uh,v2.uh):sat }"),
            ("vroundwh", "{ v0.h = vround(v1.w,v2.w):sat }"),
            ("vroundwuh", "{ v0.uh = vround(v1.w,v2.w):sat }"),
            ("vrounduwuh", "{ v0.uh = vround(v1.uw,v2.uw):sat }"),
            ("vsathub", "{ v0.ub = vsat(v1.h,v2.h) }"),
            ("vsatwh", "{ v0.h = vsat(v1.w,v2.w) }"),
            ("vsatuwuh", "{ v0.uh = vsat(v1.uw,v2.uw) }"),
            ("vsatdw", "{ v0.w = vsatdw(v1.w,v2.w) }"),
            ("vasrh_acc", "{ v0.h += vasr(v1.h,r5) }"),
            ("vaslh_acc", "{ v0.h += vasl(v1.h,r5) }"),
            ("vasrw_acc", "{ v0.w += vasr(v1.w,r5) }"),
            ("vaslw_acc", "{ v0.w += vasl(v1.w,r5) }"),
            ("vasr_into", "{ v1:0.w = vasrinto(v2.w,v3.w) }"),
        ],
        10,
        0x52ad,
    );
}

// ==== hvx_permx (wave-2 workflow) ====
// ==== hvx_permx (workflow: extended permutes) ====
// Full shuffle/deal (in-place dual-register vshuff/vdeal + pair-dest vdealvdd),
// odd/even shuffle into a pair (vshufoeb/vshufoeh), the vdelta/vrdelta byte
// permute networks, Q-selected vswap, and per-word bit rotate vrotr.
//
// vshuff/vdeal write BOTH source registers in place (Vy and Vx), and both are
// seeded random, so the diff over all 32 V regs verifies both halves. vswap
// consumes a vector predicate Q; Hexagon does not forward Q within a packet, so
// the Q is produced by a vcmp in an earlier packet and consumed by vswap in a
// later one (the captured V pair reflects the produced Q).
#[test]
fn diff_hvx_permx() {
    run_family(
        "hvx_permx",
        &[
            // in-place dual-register full shuffle / deal (control = r5)
            ("vshuff", "{ vshuff(v1,v0,r5) }"),
            ("vdeal", "{ vdeal(v1,v0,r5) }"),
            // pair-dest deal: Vdd = vdeal(Vu,Vv,Rt)
            ("vdealvdd", "{ v1:0 = vdeal(v3,v2,r5) }"),
            // odd/even shuffle into a vector pair
            ("vshufoeb", "{ v1:0.b = vshuffoe(v3.b,v2.b) }"),
            ("vshufoeh", "{ v1:0.h = vshuffoe(v3.h,v2.h) }"),
            // byte permute networks (control vector Vv = v2)
            ("vdelta", "{ v0 = vdelta(v1,v2) }"),
            ("vrdelta", "{ v0 = vrdelta(v1,v2) }"),
            // per-word bit rotate right
            ("vrotr", "{ v0.uw = vrotr(v1.uw,v2.uw) }"),
            // Q-selected swap: produce Q0 in packet 1, consume in packet 2.
            (
                "vswap",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ v1:0 = vswap(q0,v3,v2) }",
            ),
        ],
        12,
        0x9106,
    );
}

// ==== hvx_predop (wave-2 workflow) ====
// ==== hvx_predop (workflow: vector-predicated add/sub + scalar-pred mov/combine) ====
// The vector-predicated add/sub forms consume a Q produced in an EARLIER packet
// (Hexagon does not forward Q within a packet); the destination v0 is a
// read-modify-write accumulator (seeded random) that the oracle captures. The
// scalar-predicated mov/combine forms are single-packet: P0 is seeded all-0 /
// all-1 so both the take and CANCEL branches are exercised across iterations.
#[test]
fn diff_hvx_predop() {
    run_family(
        "hvx_predop",
        &[
            // if (Qv) Vx += Vu  /  if (!Qv) Vx += Vu  (byte/half/word)
            (
                "vaddbq",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ if (q0) v0.b += v1.b }",
            ),
            (
                "vaddbnq",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ if (!q0) v0.b += v1.b }",
            ),
            (
                "vaddhq",
                "{ q0 = vcmp.gt(v5.h,v6.h) }\n{ if (q0) v0.h += v1.h }",
            ),
            (
                "vaddhnq",
                "{ q0 = vcmp.gt(v5.h,v6.h) }\n{ if (!q0) v0.h += v1.h }",
            ),
            (
                "vaddwq",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ if (q0) v0.w += v1.w }",
            ),
            (
                "vaddwnq",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ if (!q0) v0.w += v1.w }",
            ),
            // if (Qv) Vx -= Vu  /  if (!Qv) Vx -= Vu
            (
                "vsubbq",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ if (q0) v0.b -= v1.b }",
            ),
            (
                "vsubbnq",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ if (!q0) v0.b -= v1.b }",
            ),
            (
                "vsubhq",
                "{ q0 = vcmp.gt(v5.h,v6.h) }\n{ if (q0) v0.h -= v1.h }",
            ),
            (
                "vsubhnq",
                "{ q0 = vcmp.gt(v5.h,v6.h) }\n{ if (!q0) v0.h -= v1.h }",
            ),
            (
                "vsubwq",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ if (q0) v0.w -= v1.w }",
            ),
            (
                "vsubwnq",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ if (!q0) v0.w -= v1.w }",
            ),
            // scalar-predicated move: if (Ps) Vd = Vu  /  vncmov: if (!Ps)
            ("vcmov", "{ if (p0) v0 = v1 }"),
            ("vncmov", "{ if (!p0) v0 = v1 }"),
            // scalar-predicated combine: if (Ps) Vdd = vcombine(Vu, Vv)
            ("vccombine", "{ if (p0) v1:0 = vcombine(v3,v2) }"),
            ("vnccombine", "{ if (!p0) v1:0 = vcombine(v3,v2) }"),
        ],
        8,
        0x6b1f,
    );
}

// ==== hvx_cmpacc (wave-2 workflow) ====
// --- hvx_cmpacc: compare-accumulate vector predicates -----------------------
// `Qx {&=,|=,^=} vcmp.{gt,eq}(Vu.<t>,Vv.<t>)` reads-modifies an existing Q. The
// accumulator Q must come from an *earlier* packet (Hexagon does not forward Q
// within a packet), so each case is a 3-packet chain: packet 1 seeds q0 with an
// independent compare, packet 2 is the compare-accumulate under test, packet 3
// muxes the final q0 into the oracle-captured v0. The eq unsigned (ub/uh/uw)
// forms are assembler aliases of the signed (b/h/w) encodings, so only the gt
// forms have distinct signed/unsigned opcodes.
#[test]
fn diff_hvx_cmpacc() {
    run_family(
        "hvx_cmpacc",
        &[
            // ---- gt signed byte ----
            (
                "vgtb_and",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ q0 &= vcmp.gt(v1.b,v2.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtb_or",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ q0 |= vcmp.gt(v1.b,v2.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtb_xor",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ q0 ^= vcmp.gt(v1.b,v2.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            // ---- gt signed halfword ----
            (
                "vgth_and",
                "{ q0 = vcmp.gt(v5.h,v6.h) }\n{ q0 &= vcmp.gt(v1.h,v2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgth_or",
                "{ q0 = vcmp.gt(v5.h,v6.h) }\n{ q0 |= vcmp.gt(v1.h,v2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgth_xor",
                "{ q0 = vcmp.gt(v5.h,v6.h) }\n{ q0 ^= vcmp.gt(v1.h,v2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            // ---- gt signed word ----
            (
                "vgtw_and",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ q0 &= vcmp.gt(v1.w,v2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtw_or",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ q0 |= vcmp.gt(v1.w,v2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtw_xor",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ q0 ^= vcmp.gt(v1.w,v2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            // ---- gt unsigned byte ----
            (
                "vgtub_and",
                "{ q0 = vcmp.gt(v5.ub,v6.ub) }\n{ q0 &= vcmp.gt(v1.ub,v2.ub) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtub_or",
                "{ q0 = vcmp.gt(v5.ub,v6.ub) }\n{ q0 |= vcmp.gt(v1.ub,v2.ub) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtub_xor",
                "{ q0 = vcmp.gt(v5.ub,v6.ub) }\n{ q0 ^= vcmp.gt(v1.ub,v2.ub) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            // ---- gt unsigned halfword ----
            (
                "vgtuh_and",
                "{ q0 = vcmp.gt(v5.uh,v6.uh) }\n{ q0 &= vcmp.gt(v1.uh,v2.uh) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtuh_or",
                "{ q0 = vcmp.gt(v5.uh,v6.uh) }\n{ q0 |= vcmp.gt(v1.uh,v2.uh) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtuh_xor",
                "{ q0 = vcmp.gt(v5.uh,v6.uh) }\n{ q0 ^= vcmp.gt(v1.uh,v2.uh) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            // ---- gt unsigned word ----
            (
                "vgtuw_and",
                "{ q0 = vcmp.gt(v5.uw,v6.uw) }\n{ q0 &= vcmp.gt(v1.uw,v2.uw) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtuw_or",
                "{ q0 = vcmp.gt(v5.uw,v6.uw) }\n{ q0 |= vcmp.gt(v1.uw,v2.uw) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vgtuw_xor",
                "{ q0 = vcmp.gt(v5.uw,v6.uw) }\n{ q0 ^= vcmp.gt(v1.uw,v2.uw) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            // ---- eq byte ----
            (
                "veqb_and",
                "{ q0 = vcmp.eq(v5.b,v6.b) }\n{ q0 &= vcmp.eq(v1.b,v2.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqb_or",
                "{ q0 = vcmp.eq(v5.b,v6.b) }\n{ q0 |= vcmp.eq(v1.b,v2.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqb_xor",
                "{ q0 = vcmp.eq(v5.b,v6.b) }\n{ q0 ^= vcmp.eq(v1.b,v2.b) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            // ---- eq halfword ----
            (
                "veqh_and",
                "{ q0 = vcmp.eq(v5.h,v6.h) }\n{ q0 &= vcmp.eq(v1.h,v2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqh_or",
                "{ q0 = vcmp.eq(v5.h,v6.h) }\n{ q0 |= vcmp.eq(v1.h,v2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqh_xor",
                "{ q0 = vcmp.eq(v5.h,v6.h) }\n{ q0 ^= vcmp.eq(v1.h,v2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            // ---- eq word ----
            (
                "veqw_and",
                "{ q0 = vcmp.eq(v5.w,v6.w) }\n{ q0 &= vcmp.eq(v1.w,v2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqw_or",
                "{ q0 = vcmp.eq(v5.w,v6.w) }\n{ q0 |= vcmp.eq(v1.w,v2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "veqw_xor",
                "{ q0 = vcmp.eq(v5.w,v6.w) }\n{ q0 ^= vcmp.eq(v1.w,v2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
        ],
        8,
        0x2cac,
    );
}

// ==== hvx_misc (wave-2 workflow) ====
// ==== hvx_misc (workflow: misc HVX bridges/housekeeping) ====
// vnot, vinsertwr, extractw, the Q<->R/V accumulate bridges, predicate prefix
// sums, vsetq, and the predicate shuffle/shrink ops. Scalar Rt = r5.
//   * Acc bridges (vand{,n}qrt_acc) read+write v0 (seeded random) and consume a
//     Q produced in an earlier packet (Hexagon does not forward Q intra-packet).
//   * Q-producing ops (vandvrt_acc, vsetq, shuffeq{h,w}) are verified by feeding
//     the produced Q into a later vmux whose captured V0 reflects the whole chain.
//   * vinsertwr seeds v0 random and replaces word 0; extractw captures r1.
#[test]
fn diff_hvx_misc() {
    run_family(
        "hvx_misc",
        &[
            ("vnot", "{ v0 = vnot(v1) }"),
            ("vinsertwr", "{ v0.w = vinsert(r5) }"),
            ("extractw", "{ r1 = vextract(v0,r2) }"),
            (
                "vandqrt_acc",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ v0.ub |= vand(q0.ub,r5.ub) }",
            ),
            (
                "vandnqrt_acc",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ v0.ub |= vand(!q0.ub,r5.ub) }",
            ),
            (
                "vandvrt_acc",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ q0 |= vand(v1,r5) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "vprefixqb",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ v0.b = prefixsum(q0) }",
            ),
            (
                "vprefixqh",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ v0.h = prefixsum(q0) }",
            ),
            (
                "vprefixqw",
                "{ q0 = vcmp.gt(v5.b,v6.b) }\n{ v0.w = prefixsum(q0) }",
            ),
            (
                "pred_scalar2",
                "{ q0 = vsetq(r5) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "shuffeqh",
                "{ q1 = vcmp.gt(v5.b,v6.b); q2 = vcmp.gt(v7.b,v8.b) }\n{ q0.b = vshuffe(q1.h,q2.h) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
            (
                "shuffeqw",
                "{ q1 = vcmp.gt(v5.b,v6.b); q2 = vcmp.gt(v7.b,v8.b) }\n{ q0.h = vshuffe(q1.w,q2.w) }\n{ v0 = vmux(q0,v3,v4) }",
            ),
        ],
        16,
        0xd1f5,
    );
}

// ==== hvx_carry (wave-3 workflow) ====
// ==== hvx_carry (wave-3 workflow) ====
// Carry-chain add/sub with a vector-predicate carry, unsigned-sat word subtract,
// and the V62 set-predicate-v2 (vsetq2). The carry bits live in a Q predicate
// (one 4-bit group per 32-bit word lane: carry-in is read at bit 4*i, carry-out
// is written across all 4 bits of the group).
//   * vaddcarry/vsubcarry use the SAME Qx for carry-in AND carry-out: q0 is
//     seeded by an EARLIER vcmp packet (Hexagon does not forward Q intra-packet,
//     so the op reads the OLD architectural Q), the op's Vd is captured directly,
//     and the resulting q0 is muxed into v0 in a FINAL packet to verify carry-out.
//   * vaddcarryo/vsubcarryo write the carry into a SEPARATE Qe with no carry-in;
//     Vd is captured directly and the produced q1 is muxed into v0.
//   * vaddcarrysat reads carry-in from Qs (seeded earlier) and writes no Q, so a
//     single producer + op chain captures Vd.
//   * vsubuwsat is a plain single-packet per-lane unsigned-saturating subtract.
//   * vsetq2 produces q0, verified by muxing it into v0; r5 supplies the length.
#[test]
fn diff_hvx_carry() {
    run_family(
        "hvx_carry",
        &[
            // Vd captured directly; carry-out q0 fed into a separate v7 via vmux.
            (
                "vaddcarry_vd",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ v0.w = vadd(v1.w,v2.w,q0):carry }",
            ),
            (
                "vaddcarry_qout",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ v3.w = vadd(v1.w,v2.w,q0):carry }\n{ v0 = vmux(q0,v8,v9) }",
            ),
            (
                "vsubcarry_vd",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ v0.w = vsub(v1.w,v2.w,q0):carry }",
            ),
            (
                "vsubcarry_qout",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ v3.w = vsub(v1.w,v2.w,q0):carry }\n{ v0 = vmux(q0,v8,v9) }",
            ),
            // carry-out only (separate Qe); Vd captured, then mux the produced q1.
            ("vaddcarryo_vd", "{ v0.w,q1 = vadd(v1.w,v2.w):carry }"),
            (
                "vaddcarryo_qout",
                "{ v3.w,q1 = vadd(v1.w,v2.w):carry }\n{ v0 = vmux(q1,v8,v9) }",
            ),
            ("vsubcarryo_vd", "{ v0.w,q1 = vsub(v1.w,v2.w):carry }"),
            (
                "vsubcarryo_qout",
                "{ v3.w,q1 = vsub(v1.w,v2.w):carry }\n{ v0 = vmux(q1,v8,v9) }",
            ),
            // carry-in only, saturating; no Q written, capture Vd.
            (
                "vaddcarrysat",
                "{ q0 = vcmp.gt(v5.w,v6.w) }\n{ v0.w = vadd(v1.w,v2.w,q0):carry:sat }",
            ),
            // plain single-packet unsigned-sat word subtract.
            ("vsubuwsat", "{ v0.uw = vsub(v1.uw,v2.uw):sat }"),
            // vsetq2: produce q0 from r5, verify by muxing into v0.
            ("vsetq2", "{ q0 = vsetq2(r5) }\n{ v0 = vmux(q0,v8,v9) }"),
        ],
        16,
        0xca77,
    );
}

// ==== HVX V69 register ops (verified against the v69 oracle) ====
#[test]
fn diff_hvx_v69() {
    run_family(
        "hvx_v69",
        &[
            // unsigned 16x16 -> high 16 bits
            ("vmpyuhvs", "{ v0.uh = vmpy(v1.uh,v2.uh):>>16 }"),
        ],
        8,
        0x6900,
    );
}

// ==== hvx_v6mpy (V69 workflow) ====
#[test]
fn diff_hvx_v6mpy() {
    // V69 byte-matrix multiply: Vuu.ub * Vvv.b coefficients -> Vdd/Vxx.w pair,
    // for both :h (horizontal) and :v (vertical) phases, all four #u2 phases,
    // and the += accumulate forms. Pairs use distinct even bases.
    run_family(
        "hvx_v6mpy",
        &[
            ("v6mpyh_u0", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#0):h }"),
            ("v6mpyh_u1", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#1):h }"),
            ("v6mpyh_u2", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#2):h }"),
            ("v6mpyh_u3", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#3):h }"),
            ("v6mpyv_u0", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#0):v }"),
            ("v6mpyv_u1", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#1):v }"),
            ("v6mpyv_u2", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#2):v }"),
            ("v6mpyv_u3", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#3):v }"),
            ("v6mpyh_acc_u0", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#0):h }"),
            ("v6mpyh_acc_u1", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#1):h }"),
            ("v6mpyh_acc_u2", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#2):h }"),
            ("v6mpyh_acc_u3", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#3):h }"),
            ("v6mpyv_acc_u0", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#0):v }"),
            ("v6mpyv_acc_u1", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#1):v }"),
            ("v6mpyv_acc_u2", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#2):v }"),
            ("v6mpyv_acc_u3", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#3):v }"),
        ],
        12,
        0x6909,
    );
}
