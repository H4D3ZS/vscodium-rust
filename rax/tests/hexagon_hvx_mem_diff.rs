//! HVX load/store differential test harness: rax vs. qemu-hexagon. Verifies HVX
//! `vmem` loads/stores against `tools/hexagon-diff/oracle_hvx_mem`, which carries
//! both the vector register file and a 128-byte-aligned memory arena
//! (`g_varena`). The test points a base GPR at the arena, runs a vmem packet on
//! the oracle and on rax's `HexagonVcpu`, and compares all V registers, GPRs and
//! the arena. Self-skips if the toolchain is unavailable.

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
// State-word indices for the control registers the oracle loads (byte offset / 4
// within the 176-byte HexState; mirrors `CREGS` in gen_oracle_hvx_mem.py).
const I_M0: usize = 34; // M0 / C6 (scatter/gather modifier = region length-1)
const I_M1: usize = 35; // M1 / C7
const ST_WORDS: usize = 44;
const VREGS: usize = 32;
const VWORDS: usize = 32;
const ARENA: usize = 1024;
const VOFF: usize = 256;
const AOFF: usize = VOFF + VREGS * VWORDS * 4; // 4352
const QSRC_OFF: usize = AOFF + ARENA; // 5376 (Q0..Q3 mask-source vectors)
const QSRC_VECS: usize = 4;
const IN_SIZE: usize = QSRC_OFF + QSRC_VECS * 128; // InCase carries the qsrc block
const OUT_SIZE: usize = AOFF + ARENA; // OutCase does not (Q is not captured)
const WIRE_MAGIC: u32 = 0x3158_4548;
const CODE_ADDR: u32 = 0x1000;
const BASE_REG: usize = 4; // r4 points at the arena
const GATHER_REG: usize = 6; // r6 points at the gather store target (arena+512)
const GATHER_OFF: u32 = 512; // gather writes its 128B result here (upper half)

#[derive(Clone)]
struct Case {
    words: Vec<u32>,
    st: [u32; ST_WORDS],
    v: [[u32; VWORDS]; VREGS],
    arena: [u8; ARENA],
    /// Per-byte LSB seeds Q0..Q3 (the OLD vector predicates). Each [128] vector
    /// maps to one Q register: byte i's bit 0 = Q.bit[i].
    qsrc: [[u8; 128]; QSRC_VECS],
}

#[derive(Clone)]
struct Out {
    st: [u32; ST_WORDS],
    v: [[u32; VWORDS]; VREGS],
    arena: [u8; ARENA],
}

fn which(prog: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(prog))
        .find(|c| c.is_file())
}

/// Build oracle_hvx_mem; return (path, g_varena address).
fn oracle() -> Option<(PathBuf, u32)> {
    which("qemu-hexagon")?;
    which("llvm-mc")?;
    which("ld.lld")?;
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/hexagon-diff");
    let bin = dir.join("oracle_hvx_mem");
    let src = dir.join("gen_oracle_hvx_mem.py");
    let need = match (bin.metadata(), src.metadata()) {
        (Ok(b), Ok(s)) => match (b.modified(), s.modified()) {
            (Ok(bm), Ok(sm)) => bm < sm,
            _ => true,
        },
        _ => true,
    };
    if need {
        let ok = Command::new("bash")
            .arg(dir.join("build_hvx_mem.sh"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            return None;
        }
    }
    let nm = which("llvm-nm")
        .map(|_| "llvm-nm")
        .or(which("nm").map(|_| "nm"))?;
    let out = Command::new(nm).arg(&bin).output().ok()?;
    let addr = String::from_utf8_lossy(&out.stdout).lines().find_map(|l| {
        let mut it = l.split_whitespace();
        let a = it.next()?;
        let _t = it.next()?;
        (it.next() == Some("g_varena")).then(|| u32::from_str_radix(a, 16).ok())?
    })?;
    Some((bin, addr))
}

fn run_oracle(bin: &PathBuf, cases: &[Case]) -> Option<Vec<Out>> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&WIRE_MAGIC.to_le_bytes());
    payload.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for c in cases {
        let mut buf = vec![0u8; IN_SIZE];
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
                let o = VOFF + (r * VWORDS + w) * 4;
                buf[o..o + 4].copy_from_slice(&c.v[r][w].to_le_bytes());
            }
        }
        buf[AOFF..AOFF + ARENA].copy_from_slice(&c.arena);
        for k in 0..QSRC_VECS {
            let o = QSRC_OFF + k * 128;
            buf[o..o + 128].copy_from_slice(&c.qsrc[k]);
        }
        payload.extend_from_slice(&buf);
    }
    let mut child = Command::new("qemu-hexagon")
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
                let o = off + VOFF + (r * VWORDS + w) * 4;
                v[r][w] = u32::from_le_bytes([out[o], out[o + 1], out[o + 2], out[o + 3]]);
            }
        }
        let mut arena = [0u8; ARENA];
        arena.copy_from_slice(&out[off + AOFF..off + AOFF + ARENA]);
        off += OUT_SIZE;
        res.push(Out { st, v, arena });
    }
    Some(res)
}

fn assemble(packets: &[String]) -> Option<Vec<Vec<u32>>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Vec<Vec<u32>>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let key = packets.join("\n@@@\n");
    if let Some(v) = cache.lock().unwrap().get(&key) {
        return Some(v.clone());
    }
    let mut child = Command::new("llvm-mc")
        .args([
            "-triple=hexagon",
            "-mcpu=hexagonv68",
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
        .write_all(packets.join("\n").as_bytes())
        .ok()?;
    let mut out = String::new();
    child.stdout.take().unwrap().read_to_string(&mut out).ok()?;
    if !child.wait().ok()?.success() {
        return None;
    }
    let mut bytes = Vec::new();
    for line in out.lines() {
        if let Some(i) = line.find("encoding: [") {
            let rest = &line[i + 11..];
            let end = rest.find(']')?;
            for t in rest[..end].split(',') {
                let t = t.trim().strip_prefix("0x").unwrap_or(t.trim());
                if let Ok(b) = u8::from_str_radix(t, 16) {
                    bytes.push(b);
                }
            }
        }
    }
    let mut grouped = Vec::new();
    let mut cur = Vec::new();
    let mut acc = Vec::new();
    for b in bytes {
        acc.push(b);
        if acc.len() == 4 {
            let w = acc[0] as u32
                | (acc[1] as u32) << 8
                | (acc[2] as u32) << 16
                | (acc[3] as u32) << 24;
            acc.clear();
            cur.push(w);
            let pb = (w >> 14) & 3;
            if pb == 0b11 || pb == 0b00 {
                grouped.push(std::mem::take(&mut cur));
            }
        }
    }
    if !cur.is_empty() {
        grouped.push(cur);
    }
    if grouped.len() != packets.len() {
        return None;
    }
    cache.lock().unwrap().insert(key, grouped.clone());
    Some(grouped)
}

fn trap0_word() -> u32 {
    static W: OnceLock<u32> = OnceLock::new();
    *W.get_or_init(|| assemble(&["{ trap0(#0) }".to_string()]).expect("trap0")[0][0])
}

fn run_rax(words: &[u32], c: &Case, varena: u32) -> Option<Out> {
    let regions = vec![(GuestAddress(0), 0x20_0000usize)];
    let mem = Arc::new(GuestMemoryMmap::<()>::from_ranges(&regions).ok()?);
    let mut off = CODE_ADDR;
    for &w in words {
        mem.write_slice(&w.to_le_bytes(), GuestAddress(off as u64))
            .ok()?;
        off += 4;
    }
    mem.write_slice(&trap0_word().to_le_bytes(), GuestAddress(off as u64))
        .ok()?;
    mem.write_slice(&c.arena, GuestAddress(varena as u64))
        .ok()?;

    let mut regs = HexagonRegisters::default();
    for i in 0..NREG {
        regs.r[i] = c.st[i];
    }
    for i in 0..4 {
        regs.p[i] = ((c.st[I_PRED] >> (8 * i)) & 0xff) as u8;
    }
    // Scatter/gather modifier registers M0/M1 (C6/C7): the region length-1. The
    // oracle loads these from the same state words via its CREGS table.
    regs.c[6] = c.st[I_M0];
    regs.c[7] = c.st[I_M1];
    regs.v = c.v;
    // OLD vector predicates Q0..Q3 from the per-byte LSB of the source vectors
    // (matching the oracle's vandvrt seeding).
    for k in 0..QSRC_VECS {
        let mut q = [0u32; 4];
        for i in 0..128 {
            if c.qsrc[k][i] & 1 == 1 {
                q[i / 32] |= 1u32 << (i % 32);
            }
        }
        regs.q[k] = q;
    }
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
    let mut arena = [0u8; ARENA];
    mem.read_slice(&mut arena, GuestAddress(varena as u64))
        .ok()?;
    Some(Out {
        st,
        v: regs.v,
        arena,
    })
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

fn run_family(name: &str, cases: &[(&str, &str)], n: usize, seed: u64) {
    run_family_q(name, cases, n, seed, false);
}

/// Offset-vector layout for a scatter/gather case. Decides how the harness seeds
/// the offset register(s) with valid, in-range, element-aligned byte offsets
/// (random 32-bit offsets are virtually all out-of-range, so nothing would
/// scatter/gather). `gather` selects the constrained source window so the
/// gather's 128-byte store target (upper arena half) never overlaps the source.
#[derive(Clone, Copy)]
enum SgKind {
    /// 32 word offsets in `v1` (esz = 4).
    Word,
    /// 64 halfword offsets in `v1` (esz = 2).
    Half,
    /// 64 word-sized offsets in the `v1:0` register pair (v0 = low, v1 = high),
    /// feeding 64 halfword data elements (esz = 2).
    Hw,
}

/// Seed the offset register(s) for `kind` into `v` with distinct, element-aligned
/// byte offsets (mostly in `[0, region)`, ~1/4 deliberately out-of-range).
/// Distinct offsets avoid same-address scatter-collision ambiguity. `region` is
/// the exclusive upper bound for a full element to fit.
fn seed_offsets(v: &mut [[u32; VWORDS]; VREGS], kind: SgKind, region: u32, rng: &mut Rng) {
    // Pick `count` distinct aligned offsets, mostly in [0, region - align] but
    // ~1/4 deliberately out-of-range (aligned, but >= region) so the drop /
    // destination-preserve paths are exercised against the oracle too. For the
    // halfword layout offsets must fit in 16 bits, so out-of-range stays < 0x10000.
    let pick = |count: usize, align: u32, rng: &mut Rng| -> Vec<u32> {
        let slots = (region / align).max(1);
        let oor_cap = 0x1_0000 / align; // keep halfword offsets within 16 bits
        let mut used = std::collections::HashSet::new();
        let mut out = Vec::with_capacity(count);
        while out.len() < count {
            let oor = rng.next() & 3 == 0;
            let slot = if oor {
                slots + (rng.next() as u32) % (oor_cap.saturating_sub(slots)).max(1)
            } else {
                (rng.next() as u32) % slots
            };
            if !oor && slot * align + align > region {
                continue;
            }
            if used.insert(slot) {
                out.push(slot.wrapping_mul(align));
            }
        }
        out
    };
    match kind {
        SgKind::Word => {
            let offs = pick(32, 4, rng);
            for (w, o) in offs.iter().enumerate() {
                v[1][w] = *o;
            }
        }
        SgKind::Half => {
            // 64 halfword offsets packed two-per-word into v1.
            let offs = pick(64, 2, rng);
            for w in 0..32 {
                v[1][w] = (offs[2 * w] & 0xffff) | (offs[2 * w + 1] << 16);
            }
        }
        SgKind::Hw => {
            // 64 word-sized offsets: low halves in v0, high halves in v1.
            let offs = pick(64, 2, rng);
            for i in 0..32 {
                v[0][i] = offs[2 * i]; // even element k=2i  (low vector of pair)
                v[1][i] = offs[2 * i + 1]; // odd element  k=2i+1 (high vector)
            }
        }
    }
}

/// Scatter/gather differential harness. Each case carries its offset layout
/// (`SgKind`). The harness seeds M0 = ARENA-1 (the whole arena is the region),
/// r6 = arena + GATHER_OFF (gather store target), and constrains the offset
/// register(s); Q0..Q3 are seeded from random per-byte LSBs for the predicated
/// (`q`) forms. The diff over the arena + register file then verifies the op.
fn run_family_sg(name: &str, cases: &[(&str, &str, SgKind)], n: usize, seed: u64, gather: bool) {
    let (bin, varena) = match oracle() {
        Some(x) => x,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: toolchain unavailable -> skipping");
            return;
        }
    };
    let asms: Vec<String> = cases.iter().map(|(_, a, _)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: assembly failed -> skipping");
            return;
        }
    };
    // Region for offsets: scatter spans the whole arena; gather is constrained to
    // the lower half so its source never overlaps the upper-half store target.
    let region = if gather { GATHER_OFF } else { ARENA as u32 };
    let mut rng = Rng::new(seed);
    let mut batch = Vec::new();
    let mut labels = Vec::new();
    for ((label, _, kind), words) in cases.iter().zip(words_per.iter()) {
        for _ in 0..n {
            let mut st = [0u32; ST_WORDS];
            for r in 0..NREG {
                st[r] = rng.next() as u32;
            }
            st[BASE_REG] = varena; // r4 = region base (128-aligned arena)
            st[I_M0] = ARENA as u32 - 1; // M0 = length-1: whole arena is the region
            st[I_M1] = ARENA as u32 - 1;
            if gather {
                st[GATHER_REG] = varena + GATHER_OFF; // r6 = gather store target
            }
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
            seed_offsets(&mut v, *kind, region, &mut rng);
            let mut arena = [0u8; ARENA];
            for b in arena.iter_mut() {
                *b = rng.next() as u8;
            }
            // Q0..Q3 from random per-byte LSBs (predicated forms read OLD Q).
            let mut qsrc = [[0u8; 128]; QSRC_VECS];
            for q in qsrc.iter_mut() {
                for b in q.iter_mut() {
                    *b = rng.next() as u8;
                }
            }
            labels.push(*label);
            batch.push(Case {
                words: words.clone(),
                st,
                v,
                arena,
                qsrc,
            });
        }
    }
    let outs = match run_oracle(&bin, &batch) {
        Some(o) => o,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: oracle failed -> skipping");
            return;
        }
    };
    let mut mismatches = Vec::new();
    for (i, c) in batch.iter().enumerate() {
        let rax = match run_rax(&c.words, c, varena) {
            Some(r) => r,
            None => {
                mismatches.push(format!("[{}] rax rejected", labels[i]));
                continue;
            }
        };
        let mut diffs = Vec::new();
        if rax.arena != outs[i].arena {
            let j = (0..ARENA)
                .find(|&j| rax.arena[j] != outs[i].arena[j])
                .unwrap();
            diffs.push(format!(
                "arena[{j}]:rax={:#x},hw={:#x}",
                rax.arena[j], outs[i].arena[j]
            ));
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
        panic!("{name}: {} HVX-mem divergences vs oracle", mismatches.len());
    }
}

/// As `run_family`, but when `qsrc_random` is set the OLD vector predicates
/// Q0..Q3 are seeded from random per-byte LSBs (wired into the oracle via
/// `vandvrt` and into rax via `regs.q`). Used to verify Q-masked vmem stores
/// that read the architectural (OLD) Q rather than an in-packet `.new` Q.
fn run_family_q(name: &str, cases: &[(&str, &str)], n: usize, seed: u64, qsrc_random: bool) {
    let (bin, varena) = match oracle() {
        Some(x) => x,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: toolchain unavailable -> skipping");
            return;
        }
    };
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: assembly failed -> skipping");
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
            st[BASE_REG] = varena; // base points at the (128-aligned) arena
            // Each scalar predicate P0..P3 independently 0x00 / 0xff.
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
            let mut arena = [0u8; ARENA];
            for b in arena.iter_mut() {
                *b = rng.next() as u8;
            }
            let mut qsrc = [[0u8; 128]; QSRC_VECS];
            if qsrc_random {
                for q in qsrc.iter_mut() {
                    for b in q.iter_mut() {
                        *b = rng.next() as u8;
                    }
                }
            }
            labels.push(*label);
            batch.push(Case {
                words: words.clone(),
                st,
                v,
                arena,
                qsrc,
            });
        }
    }
    let outs = match run_oracle(&bin, &batch) {
        Some(o) => o,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: oracle failed -> skipping");
            return;
        }
    };
    let mut mismatches = Vec::new();
    for (i, c) in batch.iter().enumerate() {
        let rax = match run_rax(&c.words, c, varena) {
            Some(r) => r,
            None => {
                mismatches.push(format!("[{}] rax rejected", labels[i]));
                continue;
            }
        };
        let mut diffs = Vec::new();
        for vr in 0..VREGS {
            if rax.v[vr] != outs[i].v[vr] {
                let w = (0..VWORDS)
                    .find(|&w| rax.v[vr][w] != outs[i].v[vr][w])
                    .unwrap();
                diffs.push(format!(
                    "v{vr}.w[{w}]:rax={:#x},hw={:#x}",
                    rax.v[vr][w], outs[i].v[vr][w]
                ));
                break;
            }
        }
        if rax.arena != outs[i].arena {
            let j = (0..ARENA)
                .find(|&j| rax.arena[j] != outs[i].arena[j])
                .unwrap();
            diffs.push(format!(
                "arena[{j}]:rax={:#x},hw={:#x}",
                rax.arena[j], outs[i].arena[j]
            ));
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
        panic!("{name}: {} HVX-mem divergences vs oracle", mismatches.len());
    }
}

// Second base register: producer loads for new-value vector stores read from
// here (`v3 = vmem(r5+#0)`), pointed one vector into the arena so the produced
// vector differs from the store target's old contents.
const PROD_REG: usize = 5;
const PROD_OFF: u32 = 128; // r5 = arena + 128 (one vector in)

/// Extended HVX-mem harness for the `ppu` (post-inc-by-Mu) and new-value forms.
/// Like `run_family_q` but additionally seeds:
///   * M0/M1 (C6/C7) = one / two vectors (128 / 256 bytes) — the `ppu` increment;
///   * r5 (`PROD_REG`) = arena + PROD_OFF — the new-value store's producer base.
/// All V registers, GPRs (including the post-incremented base and M0/M1) and the
/// arena are diffed against the oracle. `qsrc_random` seeds the OLD Q predicates
/// (for the q-masked `ppu` stores).
fn run_family_ext(name: &str, cases: &[(&str, &str)], n: usize, seed: u64, qsrc_random: bool) {
    let (bin, varena) = match oracle() {
        Some(x) => x,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: toolchain unavailable -> skipping");
            return;
        }
    };
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: assembly failed -> skipping");
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
            st[BASE_REG] = varena; // r4 = arena base (128-aligned)
            st[PROD_REG] = varena + PROD_OFF; // r5 = producer source (arena + 1 vec)
            st[I_M0] = 128; // M0 = one vector: ppu post-inc by 128
            st[I_M1] = 256; // M1 = two vectors: ppu post-inc by 256
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
            let mut arena = [0u8; ARENA];
            for b in arena.iter_mut() {
                *b = rng.next() as u8;
            }
            let mut qsrc = [[0u8; 128]; QSRC_VECS];
            if qsrc_random {
                for q in qsrc.iter_mut() {
                    for b in q.iter_mut() {
                        *b = rng.next() as u8;
                    }
                }
            }
            labels.push(*label);
            batch.push(Case {
                words: words.clone(),
                st,
                v,
                arena,
                qsrc,
            });
        }
    }
    let outs = match run_oracle(&bin, &batch) {
        Some(o) => o,
        None => {
            eprintln!("[hexagon_hvx_mem_diff] {name}: oracle failed -> skipping");
            return;
        }
    };
    let mut mismatches = Vec::new();
    for (i, c) in batch.iter().enumerate() {
        let rax = match run_rax(&c.words, c, varena) {
            Some(r) => r,
            None => {
                mismatches.push(format!("[{}] rax rejected", labels[i]));
                continue;
            }
        };
        let mut diffs = Vec::new();
        for vr in 0..VREGS {
            if rax.v[vr] != outs[i].v[vr] {
                let w = (0..VWORDS)
                    .find(|&w| rax.v[vr][w] != outs[i].v[vr][w])
                    .unwrap();
                diffs.push(format!(
                    "v{vr}.w[{w}]:rax={:#x},hw={:#x}",
                    rax.v[vr][w], outs[i].v[vr][w]
                ));
                break;
            }
        }
        if rax.arena != outs[i].arena {
            let j = (0..ARENA)
                .find(|&j| rax.arena[j] != outs[i].arena[j])
                .unwrap();
            diffs.push(format!(
                "arena[{j}]:rax={:#x},hw={:#x}",
                rax.arena[j], outs[i].arena[j]
            ));
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
        panic!("{name}: {} HVX-mem divergences vs oracle", mismatches.len());
    }
}

#[test]
fn diff_hvx_load() {
    // base r4 = g_varena (128-aligned); offsets in vector units stay in arena.
    run_family(
        "hvx_load",
        &[
            ("vL32b_ai_0", "{ v0 = vmem(r4+#0) }"),
            ("vL32b_ai_1", "{ v0 = vmem(r4+#1) }"),
            ("vL32b_nt", "{ v0 = vmem(r4+#2):nt }"),
            ("vL32Ub", "{ v0 = vmemu(r4+#0) }"),
            ("vL32b_cur", "{ v0.cur = vmem(r4+#1) }"),
            ("vL32b_tmp", "{ v0.tmp = vmem(r4+#2) }"),
        ],
        8,
        0xa10,
    );
}

#[test]
fn diff_hvx_store() {
    run_family(
        "hvx_store",
        &[
            ("vS32b_ai_0", "{ vmem(r4+#0) = v1 }"),
            ("vS32b_ai_1", "{ vmem(r4+#1) = v2 }"),
            ("vS32b_nt", "{ vmem(r4+#2):nt = v3 }"),
            ("vS32Ub", "{ vmemu(r4+#3) = v4 }"),
        ],
        8,
        0xa20,
    );
}

#[test]
fn diff_hvx_store_pred() {
    // Scalar-predicated vector stores: if (p0[!]) vmem(...) = Vs (or CANCEL).
    run_family(
        "hvx_store_pred",
        &[
            ("vS32b_pred", "{ if (p0) vmem(r4+#0) = v1 }"),
            ("vS32b_npred", "{ if (!p0) vmem(r4+#1) = v2 }"),
            ("vS32b_pred_pi", "{ if (p0) vmem(r4++#1) = v1 }"),
            ("vS32b_npred_pi", "{ if (!p0) vmem(r4++#1) = v2 }"),
        ],
        12,
        0xa40,
    );
}

#[test]
fn diff_hvx_store_qmask() {
    // Byte-masked vector stores read the ARCHITECTURAL (OLD) vector predicate Q,
    // not an in-packet `.new` Q. We seed Q0..Q3 from random per-byte LSBs (the
    // `qsrc` block) on both the oracle (via vandvrt) and rax, then byte-mask the
    // store of Vs: lanes where Q==sense are written, the rest keep memory.
    run_family_q(
        "hvx_store_qmask",
        &[
            ("vS32b_qpred", "{ if (q0) vmem(r4+#0) = v1 }"),
            ("vS32b_nqpred", "{ if (!q0) vmem(r4+#1) = v2 }"),
            ("vS32b_qpred_q1", "{ if (q1) vmem(r4+#2) = v3 }"),
            ("vS32b_qpred_pi", "{ if (q2) vmem(r4++#1) = v4 }"),
            ("vS32b_nqpred_pi", "{ if (!q3) vmem(r4++#1) = v5 }"),
        ],
        12,
        0xa50,
        true,
    );
}

#[test]
fn diff_hvx_load_pi() {
    // post-increment: r4 advances by one vector (128 bytes).
    run_family(
        "hvx_load_pi",
        &[
            ("vL32b_pi", "{ v0 = vmem(r4++#1) }"),
            ("vS32b_pi", "{ vmem(r4++#1) = v1 }"),
        ],
        8,
        0xa30,
    );
}

#[test]
fn diff_hvx_hist() {
    // HVX histogram family. The input is a `.tmp` vector load that MUST live in
    // the same packet as the histogram op (qemu asserts exactly one `.tmp` load;
    // a bare `{ vhist }` faults). We load V0.tmp from the 128-aligned arena (r4),
    // which rax forwards into its per-packet scratch buffer (matching qemu's
    // tmp_VRegs[0]); the op then tallies those bytes into bins spread across the
    // register file. The Qv-gated forms read the OLD architectural Q, seeded via
    // the `qsrc` block on both the oracle (vandvrt) and rax — so `run_family_q`.
    run_family_q(
        "hvx_hist",
        &[
            ("vhist", "{ v0.tmp = vmem(r4+#0); vhist }"),
            ("vhistq", "{ v0.tmp = vmem(r4+#0); vhist(q0) }"),
            ("vwhist128", "{ v0.tmp = vmem(r4+#0); vwhist128 }"),
            ("vwhist128m", "{ v0.tmp = vmem(r4+#0); vwhist128(#1) }"),
            ("vwhist128q", "{ v0.tmp = vmem(r4+#0); vwhist128(q0) }"),
            ("vwhist128qm", "{ v0.tmp = vmem(r4+#0); vwhist128(q0,#1) }"),
            ("vwhist256", "{ v0.tmp = vmem(r4+#0); vwhist256 }"),
            ("vwhist256_sat", "{ v0.tmp = vmem(r4+#0); vwhist256:sat }"),
            ("vwhist256q", "{ v0.tmp = vmem(r4+#0); vwhist256(q0) }"),
            (
                "vwhist256q_sat",
                "{ v0.tmp = vmem(r4+#0); vwhist256(q0):sat }",
            ),
        ],
        8,
        0x4157,
        true,
    );
}

#[test]
fn diff_hvx_scatter() {
    // HVX V65 scatter family. r4 = arena base, M0 = ARENA-1 (whole-arena region).
    // The offset vector v1 (and v0 for the `hw` pair forms) holds valid in-range,
    // element-aligned byte offsets; v2 is the data. Plain forms store, `_add`
    // forms accumulate, and `q` forms gate per-element on the OLD vector predicate
    // Q0. The arena diff verifies the scattered result.
    run_family_sg(
        "hvx_scatter",
        &[
            ("vscattermw", "{ vscatter(r4,m0,v1.w).w=v2 }", SgKind::Word),
            ("vscattermh", "{ vscatter(r4,m0,v1.h).h=v2 }", SgKind::Half),
            ("vscattermhw", "{ vscatter(r4,m0,v1:0.w).h=v2 }", SgKind::Hw),
            (
                "vscattermw_add",
                "{ vscatter(r4,m0,v1.w).w+=v2 }",
                SgKind::Word,
            ),
            (
                "vscattermh_add",
                "{ vscatter(r4,m0,v1.h).h+=v2 }",
                SgKind::Half,
            ),
            (
                "vscattermhw_add",
                "{ vscatter(r4,m0,v1:0.w).h+=v2 }",
                SgKind::Hw,
            ),
            (
                "vscattermwq",
                "{ if (q0) vscatter(r4,m0,v1.w).w=v2 }",
                SgKind::Word,
            ),
            (
                "vscattermhq",
                "{ if (q0) vscatter(r4,m0,v1.h).h=v2 }",
                SgKind::Half,
            ),
            (
                "vscattermhwq",
                "{ if (q0) vscatter(r4,m0,v1:0.w).h=v2 }",
                SgKind::Hw,
            ),
        ],
        24,
        0x5ca7,
        false,
    );
}

#[test]
fn diff_hvx_gather() {
    // HVX V65 gather family. r4 = arena base, M0 = ARENA-1; offsets in v1 (and v0
    // for `hw`) point into the lower arena half. The gather collects the in-range
    // (and, for `q` forms, predicate-on) bytes into an internal vtmp; the paired
    // `vmem(r6+#0)=vtmp.new` store (r6 = arena+512) commits only those bytes,
    // leaving the rest of the destination intact (matching hardware, which seeds
    // the gather temp from the destination memory). Diffing the upper arena half
    // against the oracle verifies the gather.
    run_family_sg(
        "hvx_gather",
        &[
            (
                "vgathermw",
                "{ vmem(r6+#0)=vtmp.new; vtmp.w=vgather(r4,m0,v1.w).w }",
                SgKind::Word,
            ),
            (
                "vgathermh",
                "{ vmem(r6+#0)=vtmp.new; vtmp.h=vgather(r4,m0,v1.h).h }",
                SgKind::Half,
            ),
            (
                "vgathermhw",
                "{ vmem(r6+#0)=vtmp.new; vtmp.h=vgather(r4,m0,v1:0.w).h }",
                SgKind::Hw,
            ),
            (
                "vgathermwq",
                "{ vmem(r6+#0)=vtmp.new; if (q0) vtmp.w=vgather(r4,m0,v1.w).w }",
                SgKind::Word,
            ),
            (
                "vgathermhq",
                "{ vmem(r6+#0)=vtmp.new; if (q0) vtmp.h=vgather(r4,m0,v1.h).h }",
                SgKind::Half,
            ),
            (
                "vgathermhwq",
                "{ vmem(r6+#0)=vtmp.new; if (q0) vtmp.h=vgather(r4,m0,v1:0.w).h }",
                SgKind::Hw,
            ),
        ],
        24,
        0x6a73,
        true,
    );
}

#[test]
fn diff_hvx_ppu() {
    // `ppu` post-increment by the modifier register Mu (vmem(Rt++Mu)). M0 = 128
    // (one vector), M1 = 256 (two vectors); r4 = arena. After the access the base
    // must advance by Mu (verified via the r4 diff). The loaded/stored vector and
    // the arena are diffed too. Covers aligned/unaligned loads & stores.
    run_family_ext(
        "hvx_ppu",
        &[
            ("vL32b_ppu_m0", "{ v0 = vmem(r4++m0) }"),
            ("vL32b_ppu_m1", "{ v0 = vmem(r4++m1) }"),
            ("vL32b_nt_ppu", "{ v0 = vmem(r4++m0):nt }"),
            ("vL32Ub_ppu", "{ v0 = vmemu(r4++m0) }"),
            ("vS32b_ppu_m0", "{ vmem(r4++m0) = v1 }"),
            ("vS32b_ppu_m1", "{ vmem(r4++m1) = v2 }"),
            ("vS32b_nt_ppu", "{ vmem(r4++m0):nt = v3 }"),
            ("vS32Ub_ppu", "{ vmemu(r4++m0) = v4 }"),
        ],
        8,
        0xb10,
        false,
    );
}

#[test]
fn diff_hvx_ppu_cur_tmp() {
    // `.cur` (commit) and `.tmp` (forward-only, no commit) loads with the `ppu`
    // post-increment. For `.tmp` the value is NOT committed to Vd, so the V file
    // is unchanged (apart from the r4 post-inc); for `.cur` it IS committed.
    run_family_ext(
        "hvx_ppu_cur_tmp",
        &[
            ("vL32b_cur_ppu", "{ v0.cur = vmem(r4++m0) }"),
            ("vL32b_tmp_ppu", "{ v0.tmp = vmem(r4++m0) }"),
            ("vL32b_nt_cur_ppu", "{ v0.cur = vmem(r4++m0):nt }"),
            ("vL32b_nt_tmp_ppu", "{ v0.tmp = vmem(r4++m0):nt }"),
            ("vL32b_nt_cur_ai", "{ v0.cur = vmem(r4+#0):nt }"),
            ("vL32b_nt_tmp_ai", "{ v0.tmp = vmem(r4+#0):nt }"),
            ("vL32b_nt_cur_pi", "{ v0.cur = vmem(r4++#1):nt }"),
            ("vL32b_nt_tmp_pi", "{ v0.tmp = vmem(r4++#1):nt }"),
        ],
        8,
        0xb20,
        false,
    );
}

#[test]
fn diff_hvx_load_pred() {
    // Scalar-predicated loads: if (Pv[!]) Vd[.cur|.tmp] = vmem(...). On a false
    // predicate the load is cancelled (no register write, no post-increment).
    // P0..P3 are independently 0x00/0xff, so both senses are exercised; the V
    // file, arena and r4 (post-inc only when taken) are diffed.
    run_family_ext(
        "hvx_load_pred",
        &[
            ("vL32b_pred_ai", "{ if (p0) v0 = vmem(r4+#0) }"),
            ("vL32b_npred_ai", "{ if (!p1) v0 = vmem(r4+#0) }"),
            ("vL32b_pred_pi", "{ if (p2) v0 = vmem(r4++#1) }"),
            ("vL32b_npred_pi", "{ if (!p3) v0 = vmem(r4++#1) }"),
            ("vL32b_pred_ppu", "{ if (p0) v0 = vmem(r4++m0) }"),
            ("vL32b_npred_ppu", "{ if (!p1) v0 = vmem(r4++m0) }"),
            ("vL32b_cur_pred_ppu", "{ if (p2) v0.cur = vmem(r4++m0) }"),
            ("vL32b_cur_npred_ai", "{ if (!p3) v0.cur = vmem(r4+#0) }"),
            ("vL32b_cur_pred_ai", "{ if (p0) v0.cur = vmem(r4+#0) }"),
            ("vL32b_cur_pred_pi", "{ if (p1) v0.cur = vmem(r4++#1) }"),
            ("vL32b_cur_npred_pi", "{ if (!p2) v0.cur = vmem(r4++#1) }"),
            ("vL32b_cur_npred_ppu", "{ if (!p3) v0.cur = vmem(r4++m0) }"),
            ("vL32b_tmp_pred_ppu", "{ if (p0) v0.tmp = vmem(r4++m0) }"),
            ("vL32b_tmp_npred_pi", "{ if (!p1) v0.tmp = vmem(r4++#1) }"),
            ("vL32b_tmp_pred_ai", "{ if (p2) v0.tmp = vmem(r4+#0) }"),
            ("vL32b_tmp_npred_ai", "{ if (!p3) v0.tmp = vmem(r4+#0) }"),
            ("vL32b_tmp_pred_pi", "{ if (p0) v0.tmp = vmem(r4++#1) }"),
            ("vL32b_tmp_npred_ppu", "{ if (!p1) v0.tmp = vmem(r4++m0) }"),
            ("vL32b_nt_pred_ppu", "{ if (p2) v0 = vmem(r4++m0):nt }"),
            ("vL32b_nt_pred_ai", "{ if (p3) v0 = vmem(r4+#0):nt }"),
            ("vL32b_nt_pred_pi", "{ if (p0) v0 = vmem(r4++#1):nt }"),
            ("vL32b_nt_npred_ai", "{ if (!p1) v0 = vmem(r4+#0):nt }"),
            ("vL32b_nt_npred_pi", "{ if (!p2) v0 = vmem(r4++#1):nt }"),
            ("vL32b_nt_npred_ppu", "{ if (!p3) v0 = vmem(r4++m0):nt }"),
            (
                "vL32b_nt_cur_npred_ppu",
                "{ if (!p3) v0.cur = vmem(r4++m0):nt }",
            ),
            (
                "vL32b_nt_cur_pred_ai",
                "{ if (p0) v0.cur = vmem(r4+#0):nt }",
            ),
            (
                "vL32b_nt_cur_pred_pi",
                "{ if (p1) v0.cur = vmem(r4++#1):nt }",
            ),
            (
                "vL32b_nt_cur_pred_ppu",
                "{ if (p2) v0.cur = vmem(r4++m0):nt }",
            ),
            (
                "vL32b_nt_cur_npred_ai",
                "{ if (!p3) v0.cur = vmem(r4+#0):nt }",
            ),
            (
                "vL32b_nt_cur_npred_pi",
                "{ if (!p0) v0.cur = vmem(r4++#1):nt }",
            ),
            (
                "vL32b_nt_tmp_pred_ai",
                "{ if (p1) v0.tmp = vmem(r4+#0):nt }",
            ),
            (
                "vL32b_nt_tmp_pred_pi",
                "{ if (p2) v0.tmp = vmem(r4++#1):nt }",
            ),
            (
                "vL32b_nt_tmp_pred_ppu",
                "{ if (p3) v0.tmp = vmem(r4++m0):nt }",
            ),
            (
                "vL32b_nt_tmp_npred_ai",
                "{ if (!p0) v0.tmp = vmem(r4+#0):nt }",
            ),
            (
                "vL32b_nt_tmp_npred_pi",
                "{ if (!p1) v0.tmp = vmem(r4++#1):nt }",
            ),
            (
                "vL32b_nt_tmp_npred_ppu",
                "{ if (!p2) v0.tmp = vmem(r4++m0):nt }",
            ),
        ],
        12,
        0xb30,
        false,
    );
}

#[test]
fn diff_hvx_store_pred_ppu() {
    // Scalar-predicated stores with the `ppu` increment (and the unaligned
    // vS32Ub predicated forms). A false predicate CANCELS the whole op (no store,
    // no post-increment).
    run_family_ext(
        "hvx_store_pred_ppu",
        &[
            ("vS32b_pred_ppu", "{ if (p0) vmem(r4++m0) = v1 }"),
            ("vS32b_npred_ppu", "{ if (!p1) vmem(r4++m0) = v2 }"),
            ("vS32b_nt_pred_ppu", "{ if (p2) vmem(r4++m0):nt = v3 }"),
            ("vS32b_nt_npred_ppu", "{ if (!p3) vmem(r4++m0):nt = v4 }"),
            ("vS32b_nt_pred_ai", "{ if (p0) vmem(r4+#0):nt = v1 }"),
            ("vS32b_nt_npred_ai", "{ if (!p1) vmem(r4+#0):nt = v2 }"),
            ("vS32b_nt_pred_pi", "{ if (p2) vmem(r4++#1):nt = v3 }"),
            ("vS32b_nt_npred_pi", "{ if (!p3) vmem(r4++#1):nt = v4 }"),
            ("vS32Ub_pred_ai", "{ if (p0) vmemu(r4+#0) = v1 }"),
            ("vS32Ub_npred_ai", "{ if (!p1) vmemu(r4+#0) = v2 }"),
            ("vS32Ub_pred_pi", "{ if (p2) vmemu(r4++#1) = v3 }"),
            ("vS32Ub_npred_pi", "{ if (!p3) vmemu(r4++#1) = v4 }"),
            ("vS32Ub_pred_ppu", "{ if (p2) vmemu(r4++m0) = v3 }"),
            ("vS32Ub_npred_ppu", "{ if (!p3) vmemu(r4++m0) = v4 }"),
        ],
        12,
        0xb40,
        false,
    );
}

#[test]
fn diff_hvx_store_qmask_ppu() {
    // Byte-masked vector stores with the `ppu` increment. Reads the OLD
    // architectural Q (seeded via the qsrc block on both oracle and rax). Lanes
    // where Q == sense are stored; the rest of memory is preserved. The base
    // always post-increments by Mu (a q-masked store is not cancelled).
    run_family_ext(
        "hvx_store_qmask_ppu",
        &[
            ("vS32b_qpred_ppu", "{ if (q0) vmem(r4++m0) = v1 }"),
            ("vS32b_nqpred_ppu", "{ if (!q1) vmem(r4++m0) = v2 }"),
            ("vS32b_nt_qpred_ppu", "{ if (q2) vmem(r4++m0):nt = v3 }"),
            ("vS32b_nt_nqpred_ppu", "{ if (!q3) vmem(r4++m0):nt = v4 }"),
            ("vS32b_nt_qpred_ai", "{ if (q0) vmem(r4+#0):nt = v5 }"),
            ("vS32b_nt_qpred_pi", "{ if (q1) vmem(r4++#1):nt = v6 }"),
            ("vS32b_nt_nqpred_ai", "{ if (!q2) vmem(r4+#0):nt = v7 }"),
            ("vS32b_nt_nqpred_pi", "{ if (!q3) vmem(r4++#1):nt = v8 }"),
        ],
        12,
        0xb50,
        true,
    );
}

#[test]
fn diff_hvx_store_new() {
    // New-value vector store: the source is the vector produced earlier in the
    // packet. The producer `v3 = vmem(r5+#0)` (r5 = arena + 128) loads the second
    // arena vector; the `.new` store then writes it to the target (r4 = arena).
    // Covers ai/pi/ppu, the `:nt` hint, and the scalar-predicated new-value forms.
    run_family_ext(
        "hvx_store_new",
        &[
            ("vS32b_new_ai", "{ v3 = vmem(r5+#0); vmem(r4+#0) = v3.new }"),
            (
                "vS32b_new_pi",
                "{ v3 = vmem(r5+#0); vmem(r4++#1) = v3.new }",
            ),
            (
                "vS32b_new_ppu",
                "{ v3 = vmem(r5+#0); vmem(r4++m0) = v3.new }",
            ),
            (
                "vS32b_nt_new_ai",
                "{ v3 = vmem(r5+#0); vmem(r4+#0):nt = v3.new }",
            ),
            (
                "vS32b_nt_new_pi",
                "{ v3 = vmem(r5+#0); vmem(r4++#1):nt = v3.new }",
            ),
            (
                "vS32b_nt_new_ppu",
                "{ v3 = vmem(r5+#0); vmem(r4++m0):nt = v3.new }",
            ),
            (
                "vS32b_new_pred_ai",
                "{ v3 = vmem(r5+#0); if (p0) vmem(r4+#0) = v3.new }",
            ),
            (
                "vS32b_new_npred_ai",
                "{ v3 = vmem(r5+#0); if (!p1) vmem(r4+#0) = v3.new }",
            ),
            (
                "vS32b_new_pred_pi",
                "{ v3 = vmem(r5+#0); if (p2) vmem(r4++#1) = v3.new }",
            ),
            (
                "vS32b_new_npred_pi",
                "{ v3 = vmem(r5+#0); if (!p3) vmem(r4++#1) = v3.new }",
            ),
            (
                "vS32b_new_pred_ppu",
                "{ v3 = vmem(r5+#0); if (p0) vmem(r4++m0) = v3.new }",
            ),
            (
                "vS32b_new_npred_ppu",
                "{ v3 = vmem(r5+#0); if (!p1) vmem(r4++m0) = v3.new }",
            ),
            (
                "vS32b_nt_new_pred_ai",
                "{ v3 = vmem(r5+#0); if (p2) vmem(r4+#0):nt = v3.new }",
            ),
            (
                "vS32b_nt_new_npred_ppu",
                "{ v3 = vmem(r5+#0); if (!p3) vmem(r4++m0):nt = v3.new }",
            ),
            (
                "vS32b_nt_new_pred_pi",
                "{ v3 = vmem(r5+#0); if (p0) vmem(r4++#1):nt = v3.new }",
            ),
            (
                "vS32b_nt_new_pred_ppu",
                "{ v3 = vmem(r5+#0); if (p1) vmem(r4++m0):nt = v3.new }",
            ),
            (
                "vS32b_nt_new_npred_ai",
                "{ v3 = vmem(r5+#0); if (!p2) vmem(r4+#0):nt = v3.new }",
            ),
            (
                "vS32b_nt_new_npred_pi",
                "{ v3 = vmem(r5+#0); if (!p3) vmem(r4++#1):nt = v3.new }",
            ),
        ],
        14,
        0xb60,
        false,
    );
}

#[test]
fn diff_hvx_srls() {
    // Store-release barrier `vmem(...):scatter_release`: no vector source and no
    // memory write — only the post-increment (if any) takes effect. The arena
    // must be unchanged; r4 advances by one vector (pi) / by Mu (ppu).
    run_family_ext(
        "hvx_srls",
        &[
            ("vS32b_srls_ai", "{ vmem(r4+#0):scatter_release }"),
            ("vS32b_srls_pi", "{ vmem(r4++#1):scatter_release }"),
            ("vS32b_srls_ppu", "{ vmem(r4++m0):scatter_release }"),
        ],
        8,
        0xb70,
        false,
    );
}
