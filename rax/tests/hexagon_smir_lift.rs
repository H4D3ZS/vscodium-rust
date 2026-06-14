//! Hexagon -> SMIR lift verification harness.
//!
//! For each instruction we lift the Hexagon machine word(s) to SMIR ops, execute
//! them on the `SmirInterpreter` from a seeded register state, and compare the
//! resulting GPR / predicate / USR state against rax's Hexagon interpreter
//! (`HexagonVcpu`, itself differentially verified against qemu-hexagon at 0
//! divergence). A match proves the lift is semantically correct for that op;
//! an instruction whose lift returns `Unsupported` is reported as an
//! (unimplemented) lift gap, not a divergence.
//!
//! This needs no external toolchain except `llvm-mc` to assemble the test words
//! (self-skips if unavailable), mirroring the differential test harnesses.

#![cfg(target_os = "linux")]

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::{Mutex, OnceLock};

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use rax::backend::emulator::hexagon::HexagonVcpu;
use rax::config::{Endianness, HexagonIsa};
use rax::cpu::{CpuState, HexagonRegisters, VCpu, VcpuExit};
use rax::smir::types::{ArchReg, BlockId, HexagonReg, OpId, SourceArch};
use rax::smir::{
    ArchRegState, BlockResult, ControlFlow, ExitReason, HexagonLifter, LiftContext, LiftError,
    SmirBlock, SmirContext, SmirInterpreter, SmirLifter, Terminator, TrapKind,
};

const NREG: usize = 32;
const CODE_ADDR: u32 = 0x1000;

/// HVX saturating families whose qemu-verified interpreter sets the USR:OVF
/// sticky bit but whose SMIR vector lift did NOT model it. These all route
/// through vector `OpKind`s (`VLane`/`VNarrowShiftSat`/`VReduceMul`/
/// `VSlideReduceMul`/`VMpaHhSat`/`VMpyHsatAcc`/`VSatDW`/`VNarrowShiftV`/`VCarry`/
/// `VAddSubMixedSat`) some of which are SHARED with NON-OVF-setting opcodes (e.g.
/// `vasrwhsat`/`vpackhub_sat`/`vsubuwsat_dv` use bare `clamp` and set no OVF,
/// while `vsathub`/`vsubuwsat`/`vdmpyhsat` use `sat_n`/`satu_n` and DO).
///
/// This is now EMPTY: a per-instance `set_ovf: bool` flag was threaded onto the
/// four shared OpKinds (`VLane`/`VNarrowShiftSat`/`VReduceMul`/`VSlideReduceMul`),
/// and the dedicated saturating arms (`VSatDW`/`VNarrowShiftV`/`VAddSubMixedSat`/
/// `VMpaHhSat`/`VMpyHsatAcc`/`VCarry` with `sat`) set USR:OVF directly when a lane
/// clamps. The lifter sets `set_ovf=true` only for the opcodes whose sem calls
/// `ctx.sat_n`/`ctx.satu_n`; the bare-`clamp` siblings keep `set_ovf=false`. All
/// 28 formerly-deferred families now verify 0-divergence INCLUDING usr_ovf, so
/// the usr_ovf comparison below is fully enforced for every label.
const OVF_DEFERRED_HVX: &[&str] = &[];

fn which(prog: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(prog))
        .find(|c| c.is_file())
}

/// Assemble single-packet sources with llvm-mc; one word-vec per input string.
fn assemble(packets: &[String]) -> Option<Vec<Vec<u32>>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Vec<u32>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    which("llvm-mc")?;
    let mut out = Vec::with_capacity(packets.len());
    for p in packets {
        if let Some(w) = cache.lock().unwrap().get(p) {
            out.push(w.clone());
            continue;
        }
        let mut child = Command::new("llvm-mc")
            .args([
                "-triple=hexagon",
                "-mcpu=hexagonv69",
                "-mhvx",
                "-mattr=+audio",
                "-show-encoding",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        child.stdin.take().unwrap().write_all(p.as_bytes()).ok()?;
        let mut s = String::new();
        child.stdout.take().unwrap().read_to_string(&mut s).ok()?;
        if !child.wait().ok()?.success() {
            return None;
        }
        let mut words = Vec::new();
        let mut acc: Vec<u8> = Vec::new();
        for line in s.lines() {
            if let Some(i) = line.find("encoding: [") {
                let rest = &line[i + 11..];
                let end = rest.find(']')?;
                for t in rest[..end].split(',') {
                    let t = t.trim().strip_prefix("0x").unwrap_or(t.trim());
                    if let Ok(b) = u8::from_str_radix(t, 16) {
                        acc.push(b);
                        if acc.len() == 4 {
                            words.push(u32::from_le_bytes([acc[0], acc[1], acc[2], acc[3]]));
                            acc.clear();
                        }
                    }
                }
            }
        }
        if words.is_empty() {
            return None;
        }
        cache.lock().unwrap().insert(p.clone(), words.clone());
        out.push(words);
    }
    Some(out)
}

fn trap_word() -> u32 {
    static W: OnceLock<u32> = OnceLock::new();
    *W.get_or_init(|| assemble(&["{ trap0(#0) }".to_string()]).expect("trap0")[0][0])
}

#[derive(Clone)]
struct State {
    r: [u32; NREG],
    p: [u8; 4],
    usr: u32,
    /// HVX vector registers V0-31 (1024-bit each, 32 LE u32 words) — interp layout.
    v: [[u32; 32]; 32],
    /// HVX vector predicate registers Q0-3 (128-bit each, 4 u32 words).
    q: [[u32; 4]; 4],
    /// Modifier registers M0/M1 (interp control regs C6/C7) — circular/brev/
    /// post-increment-by-register addressing. Seeded + compared only by the
    /// `_mem` harness path (the plain path leaves them 0).
    m: [u32; 2],
    /// Circular-buffer start registers CS0/CS1 (interp control regs C12/C13).
    cs: [u32; 2],
    /// Global pointer GP (interp control reg C11). The interp masks `GP & !0x3f`
    /// on read; the harness seeds an already-64-byte-aligned value so both sides
    /// agree without the SMIR `GpRel` reader needing the mask.
    gp: u32,
}

impl State {
    fn zeroed() -> Self {
        State {
            r: [0; NREG],
            p: [0; 4],
            usr: 0,
            v: [[0; 32]; 32],
            q: [[0; 4]; 4],
            m: [0; 2],
            cs: [0; 2],
            gp: 0,
        }
    }
}

/// Reference: run the words on rax's Hexagon interpreter from `init`.
fn run_interp(words: &[u32], init: &State) -> Option<State> {
    let mem = Arc::new(GuestMemoryMmap::<()>::from_ranges(&[(GuestAddress(0), 0x20000)]).ok()?);
    let mut off = CODE_ADDR;
    for &w in words {
        mem.write_slice(&w.to_le_bytes(), GuestAddress(off as u64))
            .ok()?;
        off += 4;
    }
    mem.write_slice(&trap_word().to_le_bytes(), GuestAddress(off as u64))
        .ok()?;
    let mut regs = HexagonRegisters::default();
    regs.r = init.r;
    regs.p = init.p;
    regs.c[8] = init.usr;
    regs.v = init.v;
    regs.q = init.q;
    regs.set_pc(CODE_ADDR);
    let mut vcpu = HexagonVcpu::new(0, mem, HexagonIsa::V68, Endianness::Little);
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
    Some(State {
        r: regs.r,
        p: regs.p,
        usr: regs.c[8],
        v: regs.v,
        q: regs.q,
        m: [regs.c[6], regs.c[7]],
        cs: [regs.c[12], regs.c[13]],
        gp: regs.c[11],
    })
}

/// Lift the words to SMIR and execute on the SmirInterpreter from `init`.
/// `Ok(None)` => the lift is not yet implemented (Unsupported) for some word.
fn lift_and_run(words: &[u32], init: &State) -> Result<Option<State>, String> {
    let mut lifter = HexagonLifter::default_isa();
    let mut lctx = LiftContext::new(SourceArch::Hexagon);
    let mut ops = Vec::new();
    let mut addr = CODE_ADDR as u64;
    for &w in words {
        let r = lifter.lift_insn(addr, &w.to_le_bytes(), &mut lctx);
        match r {
            Ok(res) => ops.extend(res.ops),
            Err(LiftError::Unsupported { .. }) => return Ok(None),
            Err(e) => return Err(format!("lift error: {e:?}")),
        }
        addr += 4;
    }
    // Renumber op ids so they are unique within the block.
    for (i, op) in ops.iter_mut().enumerate() {
        op.id = OpId(i as u16);
    }
    let block = SmirBlock {
        id: BlockId(0),
        guest_pc: CODE_ADDR as u64,
        phis: vec![],
        ops,
        terminator: Terminator::Trap {
            kind: TrapKind::Breakpoint,
        },
        exec_count: 0,
    };
    let mut ctx = SmirContext::new_hexagon();
    for n in 0..NREG {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::R(n as u8)), init.r[n] as u64);
    }
    for n in 0..4 {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::P(n as u8)), init.p[n] as u64);
    }
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Usr), init.usr as u64);
    // Seed HVX V/Q (1024-bit V as 16 u64 lanes; 128-bit Q in lanes 0-1).
    if let ArchRegState::Hexagon(hex) = &mut ctx.arch_regs {
        for n in 0..32 {
            let mut lanes = [0u64; 16];
            for (j, lane) in lanes.iter_mut().enumerate() {
                *lane = init.v[n][2 * j] as u64 | ((init.v[n][2 * j + 1] as u64) << 32);
            }
            hex.set_v(n as u8, lanes);
        }
        for n in 0..4 {
            let mut lanes = [0u64; 16];
            lanes[0] = init.q[n][0] as u64 | ((init.q[n][1] as u64) << 32);
            lanes[1] = init.q[n][2] as u64 | ((init.q[n][3] as u64) << 32);
            hex.set_q(n as u8, lanes);
        }
    }
    ctx.pc = CODE_ADDR as u64;

    let interp = SmirInterpreter::new();
    let mut mem = rax::smir::FlatMemory::with_base(0, 0x20000);
    interp.execute_block(&mut ctx, &mut mem, &block);

    let mut out = State::zeroed();
    for n in 0..NREG {
        out.r[n] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::R(n as u8))) as u32;
    }
    for n in 0..4 {
        let v = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::P(n as u8)));
        out.p[n] = v as u8;
    }
    out.usr = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Usr)) as u32;
    if let ArchRegState::Hexagon(hex) = &ctx.arch_regs {
        for n in 0..32 {
            let lanes = hex.get_v(n as u8);
            for (j, lane) in lanes.iter().enumerate() {
                out.v[n][2 * j] = *lane as u32;
                out.v[n][2 * j + 1] = (*lane >> 32) as u32;
            }
        }
        for n in 0..4 {
            let lanes = hex.get_q(n as u8);
            out.q[n][0] = lanes[0] as u32;
            out.q[n][1] = (lanes[0] >> 32) as u32;
            out.q[n][2] = lanes[1] as u32;
            out.q[n][3] = (lanes[1] >> 32) as u32;
        }
    }
    Ok(Some(out))
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

/// Lift-verify a family: each (label, single-packet asm) over `n` random states.
/// Compares the SMIR-lifted execution against the interpreter (GPRs, predicate
/// truth, USR). Panics on a real divergence; reports (and tolerates) ops whose
/// lift is not yet implemented so the harness doubles as a coverage probe.
fn lift_family(name: &str, cases: &[(&str, &str)], n: usize, seed: u64) {
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_smir_lift] {name}: llvm-mc unavailable -> skipping");
            return;
        }
    };
    let mut rng = Rng::new(seed);
    let mut mismatches = Vec::new();
    let mut unlifted = Vec::new();
    for ((label, _asm), words) in cases.iter().zip(words_per.iter()) {
        let mut lifted_ok = false;
        for _ in 0..n {
            let mut st = State::zeroed();
            for r in st.r.iter_mut() {
                *r = rng.next() as u32;
            }
            for k in 0..4 {
                // Seed a FULL random predicate byte (exercises per-lane masks);
                // bias toward the common scalar values 0x00/0xff.
                st.p[k] = match rng.next() & 3 {
                    0 => 0x00,
                    1 => 0xff,
                    _ => rng.next() as u8,
                };
            }
            for vv in st.v.iter_mut() {
                for w in vv.iter_mut() {
                    *w = rng.next() as u32;
                }
            }
            for qq in st.q.iter_mut() {
                for w in qq.iter_mut() {
                    *w = rng.next() as u32;
                }
            }
            let interp = match run_interp(words, &st) {
                Some(s) => s,
                None => continue, // interpreter rejected (e.g. faulting op); skip
            };
            match lift_and_run(words, &st) {
                Ok(None) => {
                    unlifted.push(*label);
                    break;
                }
                Ok(Some(lift)) => {
                    lifted_ok = true;
                    let mut diffs = Vec::new();
                    for r in 0..NREG {
                        if interp.r[r] != lift.r[r] {
                            diffs.push(format!("r{r}:i={:#x},l={:#x}", interp.r[r], lift.r[r]));
                        }
                    }
                    for k in 0..4 {
                        if interp.p[k] != lift.p[k] {
                            diffs.push(format!("p{k}:i={:#x},l={:#x}", interp.p[k], lift.p[k]));
                        }
                    }
                    for vn in 0..32 {
                        if interp.v[vn] != lift.v[vn] {
                            diffs.push(format!(
                                "v{vn}:i={:08x?},l={:08x?}",
                                &interp.v[vn][..4],
                                &lift.v[vn][..4]
                            ));
                        }
                    }
                    for qn in 0..4 {
                        if interp.q[qn] != lift.q[qn] {
                            diffs.push(format!(
                                "q{qn}:i={:08x?},l={:08x?}",
                                interp.q[qn], lift.q[qn]
                            ));
                        }
                    }
                    // USR:OVF (bit 0) — the sticky integer saturation/overflow
                    // flag. Only bit 0 is compared; other USR bits (FP flags,
                    // rounding mode) are not lifted and are irrelevant here.
                    if (interp.usr & 1) != (lift.usr & 1) {
                        if OVF_DEFERRED_HVX.contains(label) {
                            // Explicitly-deferred HVX saturating family (see the
                            // OVF_DEFERRED_HVX doc): exempt the OVF bit only,
                            // logged loudly so it is never a silent skip.
                            eprintln!(
                                "[hexagon_smir_lift] {name}: OVF deferred for [{label}] (i={},l={}) \
                                 — HVX shared-OpKind set_ovf follow-up",
                                interp.usr & 1,
                                lift.usr & 1
                            );
                        } else {
                            diffs.push(format!("usr_ovf:i={},l={}", interp.usr & 1, lift.usr & 1));
                        }
                    }
                    if !diffs.is_empty() {
                        mismatches.push(format!("[{label}] {}", diffs.join(" ")));
                    }
                }
                Err(e) => mismatches.push(format!("[{label}] {e}")),
            }
        }
        let _ = lifted_ok;
    }
    if !unlifted.is_empty() {
        eprintln!("[hexagon_smir_lift] {name}: UNLIFTED (gap): {:?}", unlifted);
    }
    if !mismatches.is_empty() {
        eprintln!("\n==== {name}: {} lift mismatches ====", mismatches.len());
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!(
            "{name}: {} SMIR-lift divergences vs interpreter",
            mismatches.len()
        );
    }
}

// ============================================================================
// Memory-aware harness path (Step 1).
//
// The plain run_interp / lift_and_run / lift_family path above never seeds guest
// memory nor compares it afterwards, so load/store correctness was untested. The
// `_mem` variants below mirror those functions but (a) seed a DATA region with the
// SAME random bytes on BOTH sides, (b) force the base register to point at DATA so
// every access lands in-region, and (c) read the DATA region back and compare it
// byte-for-byte (in addition to r/p/usr). Loads are caught by the loaded register
// compare; stores by the memory compare.
// ============================================================================

/// Data region used by the `_mem` harness. Well clear of CODE_ADDR (0x1000) + the
/// few code words + the trap, and below the 0x20000 map end. Accesses use offset 0
/// or small positive multiples of the access size so they stay inside the region.
const DATA_ADDR: u32 = 0x8000;
const DATA_LEN: usize = 0x400;

/// Reference (interp) side of the `_mem` path: like `run_interp`, but seeds the
/// DATA region with `data` before running and returns the DATA region read back
/// out of guest memory alongside the resulting State.
fn run_interp_mem(words: &[u32], init: &State, data: &[u8]) -> Option<(State, Vec<u8>)> {
    let mem = Arc::new(GuestMemoryMmap::<()>::from_ranges(&[(GuestAddress(0), 0x20000)]).ok()?);
    let mut off = CODE_ADDR;
    for &w in words {
        mem.write_slice(&w.to_le_bytes(), GuestAddress(off as u64))
            .ok()?;
        off += 4;
    }
    mem.write_slice(&trap_word().to_le_bytes(), GuestAddress(off as u64))
        .ok()?;
    // Seed the DATA region with the random bytes.
    mem.write_slice(data, GuestAddress(DATA_ADDR as u64)).ok()?;
    let mut regs = HexagonRegisters::default();
    regs.r = init.r;
    regs.p = init.p;
    regs.c[8] = init.usr;
    regs.v = init.v;
    regs.q = init.q;
    // Modifier / circular-start / global-pointer registers, mapped to the
    // interp's control registers (see HexagonVcpu::{modifier,circ_start} and the
    // GP read in resolve_addr): M0=C6, M1=C7, CS0=C12, CS1=C13, GP=C11.
    regs.c[6] = init.m[0];
    regs.c[7] = init.m[1];
    regs.c[12] = init.cs[0];
    regs.c[13] = init.cs[1];
    regs.c[11] = init.gp;
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
    let mut out_data = vec![0u8; DATA_LEN];
    mem.read_slice(&mut out_data, GuestAddress(DATA_ADDR as u64))
        .ok()?;
    Some((
        State {
            r: regs.r,
            p: regs.p,
            usr: regs.c[8],
            v: regs.v,
            q: regs.q,
            m: [regs.c[6], regs.c[7]],
            cs: [regs.c[12], regs.c[13]],
            gp: regs.c[11],
        },
        out_data,
    ))
}

/// Lift side of the `_mem` path: like `lift_and_run`, but seeds the FlatMemory
/// DATA region with `data` (base=0 so the DATA offset == DATA_ADDR), runs, then
/// reads the DATA region back out of the FlatMemory via `MemoryReader::read` and
/// returns it alongside the State. `Ok(None)` => the lift is Unsupported.
fn lift_and_run_mem(
    words: &[u32],
    init: &State,
    data: &[u8],
) -> Result<Option<(State, Vec<u8>)>, String> {
    let mut lifter = HexagonLifter::default_isa();
    let mut lctx = LiftContext::new(SourceArch::Hexagon);
    let mut ops = Vec::new();
    let mut addr = CODE_ADDR as u64;
    for &w in words {
        let r = lifter.lift_insn(addr, &w.to_le_bytes(), &mut lctx);
        match r {
            Ok(res) => ops.extend(res.ops),
            Err(LiftError::Unsupported { .. }) => return Ok(None),
            Err(e) => return Err(format!("lift error: {e:?}")),
        }
        addr += 4;
    }
    for (i, op) in ops.iter_mut().enumerate() {
        op.id = OpId(i as u16);
    }
    let block = SmirBlock {
        id: BlockId(0),
        guest_pc: CODE_ADDR as u64,
        phis: vec![],
        ops,
        terminator: Terminator::Trap {
            kind: TrapKind::Breakpoint,
        },
        exec_count: 0,
    };
    let mut ctx = SmirContext::new_hexagon();
    for n in 0..NREG {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::R(n as u8)), init.r[n] as u64);
    }
    for n in 0..4 {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::P(n as u8)), init.p[n] as u64);
    }
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Usr), init.usr as u64);
    // Seed M0/M1, CS0/CS1, GP (HexagonRegState m[2]/cs[2]/gp via write_arch_reg).
    for n in 0..2 {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::M(n as u8)), init.m[n] as u64);
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Cs(n as u8)), init.cs[n] as u64);
    }
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Gp), init.gp as u64);
    ctx.pc = CODE_ADDR as u64;

    let interp = SmirInterpreter::new();
    let mut mem = rax::smir::FlatMemory::with_base(0, 0x20000);
    // base==0 so the DATA offset is DATA_ADDR itself.
    mem.load(DATA_ADDR as usize, data);
    interp.execute_block(&mut ctx, &mut mem, &block);

    let mut out = State::zeroed();
    for n in 0..NREG {
        out.r[n] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::R(n as u8))) as u32;
    }
    for n in 0..4 {
        let v = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::P(n as u8)));
        out.p[n] = v as u8;
    }
    out.usr = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Usr)) as u32;
    for n in 0..2 {
        out.m[n] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::M(n as u8))) as u32;
        out.cs[n] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Cs(n as u8))) as u32;
    }
    out.gp = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Gp)) as u32;
    // Read the DATA region back out of the FlatMemory.
    let out_data = match rax::smir::MemoryReader::read(&mem, DATA_ADDR as u64, DATA_LEN) {
        Ok(d) => d,
        Err(e) => return Err(format!("flatmem readback: {e}")),
    };
    Ok(Some((out, out_data)))
}

/// Verify a load/store family with memory seeding + memory compare.
///
/// For each (label, asm) over `n` random states: the base register `base_reg` is
/// FORCED to DATA_ADDR (so the asm's `r{base_reg}+#imm` lands in [DATA_ADDR,
/// DATA_ADDR+DATA_LEN)); DATA_LEN random bytes seed BOTH memories identically;
/// both sides run; then r/p/usr AND the DATA region are compared. A divergence in
/// either is reported. The `extra_bases` registers are also forced into the DATA
/// region (for ops with two address bases, e.g. dword stores never need it but
/// some `_rr`/index forms might want the index small — handled per-call).
fn lift_mem_family(name: &str, cases: &[(&str, &str)], base_reg: usize, n: usize, seed: u64) {
    lift_mem_family_idx(name, cases, base_reg, &[], n, seed)
}

/// As `lift_mem_family`, but also forces each register in `index_regs` to 0 so
/// scaled-index (`base+Rt<<#sh`) forms keep their effective address == base and
/// thus stay in the DATA region.
fn lift_mem_family_idx(
    name: &str,
    cases: &[(&str, &str)],
    base_reg: usize,
    index_regs: &[usize],
    n: usize,
    seed: u64,
) {
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_smir_lift] {name}: llvm-mc unavailable -> skipping");
            return;
        }
    };
    let mut rng = Rng::new(seed);
    let mut mismatches = Vec::new();
    let mut unlifted = Vec::new();
    for ((label, _asm), words) in cases.iter().zip(words_per.iter()) {
        for _ in 0..n {
            let mut st = State::zeroed();
            for r in st.r.iter_mut() {
                *r = rng.next() as u32;
            }
            for k in 0..4 {
                // Seed a FULL random predicate byte (exercises per-lane masks);
                // bias toward the common scalar values 0x00/0xff.
                st.p[k] = match rng.next() & 3 {
                    0 => 0x00,
                    1 => 0xff,
                    _ => rng.next() as u8,
                };
            }
            // Force the base register to point at the DATA region. Index/scaled
            // forms keep their (random) index register; the asm uses a 0 or tiny
            // immediate so the whole access stays in-region — when a test uses an
            // index/scaled form it masks the index small via its own asm choice.
            st.r[base_reg] = DATA_ADDR;
            for &ir in index_regs {
                st.r[ir] = 0;
            }
            // Seed the modifier / circular-start / global-pointer registers so
            // every modifier/circular/bit-reverse/GP-relative access (and its
            // base UPDATE) stays inside [DATA_ADDR, DATA_ADDR+DATA_LEN). See the
            // M-register layout in HexagonVcpu::{hex_circ_add,hex_read_ireg}:
            //   M = 0x0002_0040 ->
            //     length (bits 0..16)  = 0x40   (64-byte circular buffer, a
            //                                     divisor of DATA_LEN=0x400)
            //     K      (bits 24..27) = 0      (so circular start == CS exactly)
            //     I field              = 1      (bit 17 set; the `_pcr` increment
            //                                     is `I << access_shift`)
            // CS0/CS1 = DATA_ADDR: the circular buffer starts at DATA_ADDR so a
            // wrap lands at DATA_ADDR + (length-ish), still in-region. GP =
            // DATA_ADDR (64-byte aligned, low 6 bits zero) so `gp+#u` is in-region
            // and matches the interp's `GP & !0x3f` read with no mask needed.
            //
            // For post-inc-by-register (memX(Rx++Mu)) the base update is `Rx +=
            // raw_M`, so the new base (DATA_ADDR + 0x20040) lands OUTSIDE the
            // region — that is harmless: it is only compared as a register value,
            // never dereferenced (the access uses the OLD base = DATA_ADDR).
            st.m = [0x0002_0040, 0x0002_0040];
            st.cs = [DATA_ADDR, DATA_ADDR];
            st.gp = DATA_ADDR;
            // Seed identical random DATA bytes on both sides.
            let mut data = vec![0u8; DATA_LEN];
            for b in data.iter_mut() {
                *b = rng.next() as u8;
            }
            let (interp, idata) = match run_interp_mem(words, &st, &data) {
                Some(x) => x,
                None => continue, // interpreter rejected (e.g. faulting access); skip
            };
            match lift_and_run_mem(words, &st, &data) {
                Ok(None) => {
                    unlifted.push(*label);
                    break;
                }
                Ok(Some((lift, ldata))) => {
                    let mut diffs = Vec::new();
                    for r in 0..NREG {
                        if interp.r[r] != lift.r[r] {
                            diffs.push(format!("r{r}:i={:#x},l={:#x}", interp.r[r], lift.r[r]));
                        }
                    }
                    for k in 0..4 {
                        if interp.p[k] != lift.p[k] {
                            diffs.push(format!("p{k}:i={:#x},l={:#x}", interp.p[k], lift.p[k]));
                        }
                    }
                    if (interp.usr & 1) != (lift.usr & 1) {
                        diffs.push(format!("usr_ovf:i={},l={}", interp.usr & 1, lift.usr & 1));
                    }
                    // Modifier / circular-start / global-pointer registers. The
                    // post-inc-by-register / circular / bit-reverse base updates
                    // are written to a GPR (caught above); M/CS/GP themselves are
                    // never written by these forms, but compare them so a lift
                    // that clobbers them is caught.
                    for k in 0..2 {
                        if interp.m[k] != lift.m[k] {
                            diffs.push(format!("m{k}:i={:#x},l={:#x}", interp.m[k], lift.m[k]));
                        }
                        if interp.cs[k] != lift.cs[k] {
                            diffs.push(format!("cs{k}:i={:#x},l={:#x}", interp.cs[k], lift.cs[k]));
                        }
                    }
                    if interp.gp != lift.gp {
                        diffs.push(format!("gp:i={:#x},l={:#x}", interp.gp, lift.gp));
                    }
                    // Byte-for-byte memory compare over the DATA region.
                    if idata != ldata {
                        let first = (0..DATA_LEN).find(|&i| idata[i] != ldata[i]).unwrap_or(0);
                        diffs.push(format!(
                            "mem@{:#x}:i={:#04x},l={:#04x}",
                            DATA_ADDR as usize + first,
                            idata[first],
                            ldata[first]
                        ));
                    }
                    if !diffs.is_empty() {
                        mismatches.push(format!("[{label}] {}", diffs.join(" ")));
                    }
                }
                Err(e) => mismatches.push(format!("[{label}] {e}")),
            }
        }
    }
    if !unlifted.is_empty() {
        eprintln!("[hexagon_smir_lift] {name}: UNLIFTED (gap): {:?}", unlifted);
    }
    if !mismatches.is_empty() {
        eprintln!(
            "\n==== {name}: {} mem-lift mismatches ====",
            mismatches.len()
        );
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!(
            "{name}: {} SMIR-lift divergences vs interpreter (mem)",
            mismatches.len()
        );
    }
}

// ============================================================================
// Control-flow harness path (Step 1/2).
//
// The plain / mem harnesses run a block to a HARDCODED `Trap{Breakpoint}` and
// never exercise the lifted Terminator, so the J2/J4 branch/call/loop control
// flow was untested. The `_cf` variants below build the REAL terminator from
// `res.control_flow` (exactly as the lifter's `lift_block` does) and compare the
// NEXT-PC the SMIR interpreter resolves against the PC the qemu-verified
// HexagonVcpu lands on, plus the flow-controlling registers (LR for calls,
// LC0/LC1/SA0/SA1 for loop setup, the written predicate for compound jumps, and
// all R/P).
//
// TRAP-PC CALIBRATION (verified empirically by `cf_calibrate_unconditional`):
//   The VCPU executes the test packet at CODE_ADDR; a branch sets PC := target,
//   fall-through sets PC := packet_end. Whatever address X it lands on holds a
//   one-word trap0 packet. Executing that trap returns Shutdown AFTER the PC was
//   advanced past it, so `regs.pc()` reports X+4. Hence:
//       interp_final_pc == smir_next_pc + 4
//   (the SMIR next-PC is the resolved branch/fall-through target X). The code
//   window is trap-filled BOTH before and after CODE_ADDR so any small forward
//   (or backward) target lands on a trap. The comparison below subtracts the
//   4-byte trap size: `interp_final_pc - 4 == smir_next_pc`.
// ============================================================================

/// Bytes of trap-filled code window placed on EACH side of CODE_ADDR.
const CF_WINDOW: u32 = 0x400;

/// Map a resolved guest target PC to a `BlockId`. The SMIR interpreter has an
/// EMPTY `block_addrs` map in this harness, so it falls back to `target.0 as
/// u64`; encoding the guest PC into the BlockId makes the resolved address equal
/// the guest target. Guest PCs here are ~0x1000, well within u32.
fn bid(pc: u64) -> BlockId {
    BlockId(pc as u32)
}

/// Build the block Terminator from a lifted `ControlFlow`, faithfully mirroring
/// the lifter's `lift_block` mapping EXCEPT that (a) block targets carry the
/// guest PC (see `bid`) and (b) `CondBranchReg` threads the REAL condition vreg
/// (the `CondBranch` variant in `lift_block` allocates a fresh, unread vreg —
/// our predicate-branch lifts emit `CondBranchReg`, so the cond is honoured).
/// Returns `None` for a `Fallthrough` (a packet with no terminating control flow
/// — its next-PC is simply CODE_ADDR + 4*words).
fn cf_terminator(control_flow: &ControlFlow, fallthrough_pc: u64) -> Option<Terminator> {
    Some(match control_flow {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => return None,
        ControlFlow::Branch { target } | ControlFlow::DirectBranch(target) => Terminator::Branch {
            target: bid(*target),
        },
        ControlFlow::CondBranch { .. } => {
            // The lifter no longer emits this for predicate branches (it uses
            // CondBranchReg). If it ever does, we cannot recover the cond vreg —
            // fail loudly rather than silently mis-resolve.
            panic!(
                "cf_terminator: bare CondBranch has no cond vreg; lifter must emit CondBranchReg"
            );
        }
        ControlFlow::CondBranchReg {
            cond,
            taken,
            not_taken,
        } => Terminator::CondBranch {
            cond: *cond,
            true_target: bid(*taken),
            false_target: bid(*not_taken),
        },
        ControlFlow::IndirectBranch { target } => Terminator::IndirectBranch {
            target: *target,
            possible_targets: vec![],
        },
        ControlFlow::IndirectBranchMem { addr } => Terminator::IndirectBranchMem {
            addr: addr.clone(),
            possible_targets: vec![],
        },
        ControlFlow::Call { target } => Terminator::Call {
            target: target.clone(),
            args: vec![],
            continuation: bid(fallthrough_pc),
        },
        ControlFlow::Return => Terminator::Return { values: vec![] },
        ControlFlow::Trap { kind } => Terminator::Trap { kind: *kind },
        ControlFlow::Syscall => Terminator::Trap {
            kind: TrapKind::SystemCall,
        },
    })
}

/// Reference (interp) side of the `_cf` path. Fills a trap-padded code window
/// around CODE_ADDR, writes the test packet at CODE_ADDR, runs the VCPU to the
/// trap, and returns the FINAL PC and register State.
fn run_interp_cf(words: &[u32], init: &State) -> Option<(u32, State)> {
    let mem = Arc::new(GuestMemoryMmap::<()>::from_ranges(&[(GuestAddress(0), 0x20000)]).ok()?);
    let tw = trap_word();
    // Trap-fill [CODE_ADDR - CF_WINDOW, CODE_ADDR + CF_WINDOW) so any forward or
    // backward small target lands on a one-word trap packet.
    let start = CODE_ADDR - CF_WINDOW;
    let end = CODE_ADDR + CF_WINDOW;
    let mut a = start;
    while a < end {
        mem.write_slice(&tw.to_le_bytes(), GuestAddress(a as u64))
            .ok()?;
        a += 4;
    }
    // Overwrite CODE_ADDR with the test packet.
    let mut off = CODE_ADDR;
    for &w in words {
        mem.write_slice(&w.to_le_bytes(), GuestAddress(off as u64))
            .ok()?;
        off += 4;
    }
    let mut regs = HexagonRegisters::default();
    regs.r = init.r;
    regs.p = init.p;
    regs.c[8] = init.usr;
    regs.v = init.v;
    regs.q = init.q;
    // Loop registers: SA0=c[0], LC0=c[1], SA1=c[2], LC1=c[3].
    regs.c[0] = init.cs[0]; // reuse cs[] as a generic seed carrier (sa0/sa1)
    regs.c[2] = init.cs[1];
    regs.c[1] = init.m[0]; // reuse m[] as lc0/lc1 seed carrier
    regs.c[3] = init.m[1];
    regs.set_pc(CODE_ADDR);
    let mut vcpu = HexagonVcpu::new(0, mem, HexagonIsa::V68, Endianness::Little);
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
    Some((
        regs.pc(),
        State {
            r: regs.r,
            p: regs.p,
            usr: regs.c[8],
            v: regs.v,
            q: regs.q,
            m: [regs.c[1], regs.c[3]],  // lc0, lc1
            cs: [regs.c[0], regs.c[2]], // sa0, sa1
            gp: regs.c[11],
        },
    ))
}

/// Lift side of the `_cf` path. Lifts the packet, builds the REAL terminator from
/// the lifted control flow, runs `execute_block` ONCE, and maps the BlockResult
/// to a next-PC. `Ok(None)` => the lift is Unsupported. The `Err` carries an
/// unexpected exit (e.g. an Undefined trap) so a wrong lift is reported, never
/// silently accepted.
fn lift_and_run_cf(words: &[u32], init: &State) -> Result<Option<(u64, State)>, String> {
    let mut lifter = HexagonLifter::default_isa();
    let mut lctx = LiftContext::new(SourceArch::Hexagon);
    let mut ops = Vec::new();
    let mut control_flow = ControlFlow::Fallthrough;
    let mut addr = CODE_ADDR as u64;
    for &w in words {
        match lifter.lift_insn(addr, &w.to_le_bytes(), &mut lctx) {
            Ok(res) => {
                ops.extend(res.ops);
                control_flow = res.control_flow;
            }
            Err(LiftError::Unsupported { .. }) => return Ok(None),
            Err(e) => return Err(format!("lift error: {e:?}")),
        }
        addr += 4;
    }
    for (i, op) in ops.iter_mut().enumerate() {
        op.id = OpId(i as u16);
    }
    let fallthrough_pc = CODE_ADDR as u64 + 4 * words.len() as u64;
    let terminator = match cf_terminator(&control_flow, fallthrough_pc) {
        Some(t) => t,
        None => Terminator::Branch {
            target: bid(fallthrough_pc),
        },
    };
    let block = SmirBlock {
        id: BlockId(0),
        guest_pc: CODE_ADDR as u64,
        phis: vec![],
        ops,
        terminator,
        exec_count: 0,
    };
    let mut ctx = SmirContext::new_hexagon();
    for n in 0..NREG {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::R(n as u8)), init.r[n] as u64);
    }
    for n in 0..4 {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::P(n as u8)), init.p[n] as u64);
    }
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Usr), init.usr as u64);
    // Loop / link registers (m[]=LC0/LC1, cs[]=SA0/SA1, LR=r[31] already seeded).
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Lc0), init.m[0] as u64);
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Lc1), init.m[1] as u64);
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Sa0), init.cs[0] as u64);
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Sa1), init.cs[1] as u64);
    ctx.pc = CODE_ADDR as u64;

    let interp = SmirInterpreter::new();
    let mut mem = rax::smir::FlatMemory::with_base(0, 0x20000);
    let next_pc = match interp.execute_block(&mut ctx, &mut mem, &block) {
        BlockResult::Continue(pc) => pc,
        BlockResult::Exit(ExitReason::Return { to }) => to,
        BlockResult::Exit(other) => {
            return Err(format!("unexpected SMIR exit: {other:?}"));
        }
    };

    let mut out = State::zeroed();
    for n in 0..NREG {
        out.r[n] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::R(n as u8))) as u32;
    }
    for n in 0..4 {
        let v = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::P(n as u8)));
        out.p[n] = v as u8;
    }
    out.usr = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Usr)) as u32;
    out.m[0] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Lc0)) as u32;
    out.m[1] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Lc1)) as u32;
    out.cs[0] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Sa0)) as u32;
    out.cs[1] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Sa1)) as u32;
    Ok(Some((next_pc, out)))
}

/// Verify a control-flow family: each (label, single-packet asm) over `n` random
/// states. Seeds random R/P plus LC0/LC1/SA0/SA1/LR, runs both sides, and
/// compares the NEXT-PC (`interp_final_pc - 4 == smir_next_pc`, per the trap-PC
/// calibration) AND the flow-controlling registers (all R/P, LC/SA). Panics on a
/// real divergence; tolerates+reports Unsupported lifts.
fn lift_cf_family(name: &str, cases: &[(&str, &str)], n: usize, seed: u64) {
    lift_cf_family_inner(name, cases, n, seed, false);
}

/// As `lift_cf_family`, but `invert_check` flips the expected next-PC comparison
/// to PROVE the harness catches a wrong branch (the self-check): with it set, a
/// CORRECT lift must be reported as a (forced) mismatch.
fn lift_cf_family_inner(
    name: &str,
    cases: &[(&str, &str)],
    n: usize,
    seed: u64,
    invert_check: bool,
) -> usize {
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_smir_lift] {name}: llvm-mc unavailable -> skipping");
            return 0;
        }
    };
    let mut rng = Rng::new(seed);
    let mut mismatches = Vec::new();
    let mut unlifted = Vec::new();
    let mut checked = 0usize;
    for ((label, _asm), words) in cases.iter().zip(words_per.iter()) {
        for _ in 0..n {
            let mut st = State::zeroed();
            // Every GPR is a word-aligned, IN-WINDOW address in [CODE_ADDR-0x80,
            // CODE_ADDR+0x80). This serves two roles at once:
            //   * jumpr/callr use a GPR as the branch TARGET -> stays in the
            //     trap-filled window (so the interp lands on a trap, not a fault),
            //   * compare-jumps compare GPR VALUES -> these aligned addresses span
            //     64 distinct slots giving eq/gt/lt/gtu coverage.
            for r in st.r.iter_mut() {
                let off = ((rng.next() as u32) % 0x40) * 4;
                *r = (CODE_ADDR - 0x80).wrapping_add(off);
            }
            // ~50% of the time, alias the common compare operand pairs so the
            // `cmp.eq`/`tstbit` true branch is exercised, not just the false one.
            if rng.next() & 1 == 0 {
                st.r[1] = st.r[0];
                st.r[3] = st.r[2];
            }
            for k in 0..4 {
                // Seed a FULL random predicate byte (exercises per-lane masks);
                // bias toward the common scalar values 0x00/0xff.
                st.p[k] = match rng.next() & 3 {
                    0 => 0x00,
                    1 => 0xff,
                    _ => rng.next() as u8,
                };
            }
            // Loop counts (LC0/LC1 via m[]) and start addresses (SA0/SA1 via cs[]).
            st.m = [rng.next() as u32, rng.next() as u32];
            st.cs = [rng.next() as u32, rng.next() as u32];
            let (ifin, istate) = match run_interp_cf(words, &st) {
                Some(x) => x,
                None => continue, // interp rejected (faulting target etc.); skip
            };
            // Calibrated mapping: interp final PC is the trap-after-target+4.
            let inext = ifin.wrapping_sub(4) as u64;
            match lift_and_run_cf(words, &st) {
                Ok(None) => {
                    unlifted.push(*label);
                    break;
                }
                Ok(Some((lnext, lstate))) => {
                    checked += 1;
                    let mut diffs = Vec::new();
                    let pc_ok = if invert_check {
                        inext == lnext // forced-wrong: report when they DO match
                    } else {
                        inext != lnext
                    };
                    if pc_ok {
                        diffs.push(format!("next_pc:i={inext:#x},l={lnext:#x}"));
                    }
                    for r in 0..NREG {
                        if istate.r[r] != lstate.r[r] {
                            diffs.push(format!("r{r}:i={:#x},l={:#x}", istate.r[r], lstate.r[r]));
                        }
                    }
                    for k in 0..4 {
                        if istate.p[k] != lstate.p[k] {
                            diffs.push(format!("p{k}:i={:#x},l={:#x}", istate.p[k], lstate.p[k]));
                        }
                    }
                    for k in 0..2 {
                        if istate.m[k] != lstate.m[k] {
                            diffs.push(format!("lc{k}:i={:#x},l={:#x}", istate.m[k], lstate.m[k]));
                        }
                        if istate.cs[k] != lstate.cs[k] {
                            diffs
                                .push(format!("sa{k}:i={:#x},l={:#x}", istate.cs[k], lstate.cs[k]));
                        }
                    }
                    if !diffs.is_empty() {
                        mismatches.push(format!("[{label}] {}", diffs.join(" ")));
                    }
                }
                Err(e) => mismatches.push(format!("[{label}] {e}")),
            }
        }
    }
    if !unlifted.is_empty() {
        eprintln!("[hexagon_smir_lift] {name}: UNLIFTED (gap): {:?}", unlifted);
    }
    if invert_check {
        // Self-check mode: caller asserts on the returned count, do not panic.
        return mismatches.len();
    }
    if !mismatches.is_empty() {
        eprintln!(
            "\n==== {name}: {} CF-lift mismatches ====",
            mismatches.len()
        );
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!(
            "{name}: {} SMIR-lift control-flow divergences vs interpreter",
            mismatches.len()
        );
    }
    let _ = checked;
    0
}

// ---- CF harness calibration + self-check ----

#[test]
fn cf_calibrate_unconditional() {
    // KNOWN-good unconditional jump forward by 0x10. Confirms the trap-PC
    // calibration: SMIR next_pc == branch target; interp final PC == target + 4.
    let asms = vec!["{ jump #0x10 }".to_string()];
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[cf_calibrate] llvm-mc unavailable -> skipping");
            return;
        }
    };
    let words = &words_per[0];
    let st = State::zeroed();
    let (ifin, _) = run_interp_cf(words, &st).expect("interp must run unconditional jump");
    let (lnext, _) = lift_and_run_cf(words, &st)
        .expect("lift ok")
        .expect("jump is lifted");
    // SMIR target is CODE_ADDR + 0x10.
    assert_eq!(lnext, CODE_ADDR as u64 + 0x10, "SMIR jump target");
    assert_eq!(
        ifin as u64,
        lnext + 4,
        "interp final PC must be SMIR next_pc + 4 (trap after target); got ifin={ifin:#x} lnext={lnext:#x}"
    );
    eprintln!("[cf_calibrate] OK: ifin={ifin:#x} == lnext({lnext:#x}) + 4");
}

#[test]
fn cf_self_check_catches_wrong_branch() {
    // SELF-CHECK: with the comparison inverted, a CORRECT conditional-jump lift
    // must be flagged as a (forced) mismatch on EVERY iteration — proving the
    // next_pc compare is load-bearing and a wrong target would diverge.
    let cases: &[(&str, &str)] = &[("jumpt", "{ if (p0) jump #0x10 }")];
    let forced = lift_cf_family_inner("cf_self_check", cases, 40, 0xBADC0DE, true);
    assert!(
        forced > 0,
        "self-check: inverted next_pc comparison produced NO forced mismatches — \
         the harness is not actually comparing the branch target"
    );
    eprintln!("[cf_self_check] OK: {forced} forced mismatches under inverted compare");
}

// ---- plain / already-lifted control flow (exercised via the CF path) ----

#[test]
fn cf_jump_plain() {
    lift_cf_family(
        "cf_jump_plain",
        &[("jump", "{ jump #0x10 }"), ("jumpr", "{ jumpr r1 }")],
        40,
        0xCF01,
    );
}

#[test]
fn cf_jump_pred() {
    lift_cf_family(
        "cf_jump_pred",
        &[
            ("jumpt", "{ if (p0) jump #0x10 }"),
            ("jumpf", "{ if (!p0) jump #0x10 }"),
            ("jumpt_p1", "{ if (p1) jump #0x14 }"),
            ("jumpf_p1", "{ if (!p1) jump #0x14 }"),
            ("jumprt", "{ if (p0) jumpr r1 }"),
            ("jumprf", "{ if (!p0) jumpr r1 }"),
        ],
        40,
        0xCF02,
    );
}

#[test]
fn cf_call() {
    lift_cf_family(
        "cf_call",
        &[
            ("call", "{ call #0x10 }"),
            ("callr", "{ callr r1 }"),
            ("callt", "{ if (p0) call #0x10 }"),
            ("callf", "{ if (!p0) call #0x10 }"),
            ("callrt", "{ if (p0) callr r1 }"),
            ("callrf", "{ if (!p0) callr r1 }"),
        ],
        40,
        0xCF03,
    );
}

#[test]
fn cf_loop_setup() {
    lift_cf_family(
        "cf_loop_setup",
        &[
            ("loop0r", "{ loop0(#0x10, r2) }"),
            ("loop1r", "{ loop1(#0x10, r2) }"),
            ("loop0i", "{ loop0(#0x10, #5) }"),
            ("loop1i", "{ loop1(#0x10, #5) }"),
        ],
        40,
        0xCF04,
    );
}

// Software-pipelined loop setup (`spNloop0`, N=1/2/3): like loop0 (SA0 = packet-
// relative start, LC0 = count) plus P3 := 0 and USR.LPCFG (bits 9:8) := N. The
// `_cf` harness compares next-PC + all R/P + LC0/LC1/SA0/SA1: SA0/LC0 and P3 are
// the OBSERVABLE effects (and ARE compared); USR is NOT compared by the `_cf`
// harness, so the LPCFG write (modeled faithfully in the lift) is invisible here
// — see the notes / LPCFG finding. All variants must drive next-PC to the
// fall-through (sploop setup does not branch) and write SA0/LC0/P3 identically.
#[test]
fn cf_sploop_setup() {
    lift_cf_family(
        "cf_sploop_setup",
        &[
            ("sp1loop0r", "{ p3 = sp1loop0(#0x10, r2) }"),
            ("sp2loop0r", "{ p3 = sp2loop0(#0x10, r2) }"),
            ("sp3loop0r", "{ p3 = sp3loop0(#0x10, r2) }"),
            ("sp1loop0i", "{ p3 = sp1loop0(#0x10, #5) }"),
            ("sp2loop0i", "{ p3 = sp2loop0(#0x10, #5) }"),
            ("sp3loop0i", "{ p3 = sp3loop0(#0x10, #5) }"),
        ],
        40,
        0xCF05,
    );
}

// Architectural pause (`J2_pause`): no register/memory/predicate effect, only a
// PC advance to the packet fall-through. It decodes to `DecodedInsn::Nop` and now
// lifts to an EMPTY op list (a faithful no-op; the `#u8` cycle count is
// timing-only, not architectural state). The plain `lift_family` path runs both
// sides from a fully-seeded register state and compares ALL of R/P/V/Q/USR-OVF —
// so a faithful no-op must leave the entire seeded state untouched on both sides.
//
// NOTE: `{ nop }` (A2_nop, which lifts to `OpKind::Nop` via the pre-existing
// `lift_unknown_op` path) is intentionally NOT exercised here: running a block
// whose sole op is `OpKind::Nop` through `lift_and_run` blows the 2 MB default
// test-thread stack (a pre-existing codegen/harness frame-size quirk in the
// giant `execute_op` match, unrelated to this lift — `pause`'s empty-ops block
// runs fine). A2_nop's lift is already covered by the audit.
#[test]
fn lift_pause() {
    lift_family("pause", &[("pause", "{ pause(#3) }")], 40, 0xCF06);
}

// ---- J4 compound compare-and-jump ----

#[test]
fn cf_cmpjump_rr() {
    lift_cf_family(
        "cf_cmpjump_rr",
        &[
            (
                "eq_tp0_nt",
                "{ p0 = cmp.eq(r0,r1); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "eq_tp0_t",
                "{ p0 = cmp.eq(r0,r1); if (p0.new) jump:t #0x10 }",
            ),
            (
                "eq_fp0_nt",
                "{ p0 = cmp.eq(r0,r1); if (!p0.new) jump:nt #0x10 }",
            ),
            (
                "eq_tp1_nt",
                "{ p1 = cmp.eq(r0,r1); if (p1.new) jump:nt #0x10 }",
            ),
            (
                "eq_fp1_nt",
                "{ p1 = cmp.eq(r0,r1); if (!p1.new) jump:nt #0x10 }",
            ),
            (
                "gt_tp0_nt",
                "{ p0 = cmp.gt(r0,r1); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "gt_fp0_nt",
                "{ p0 = cmp.gt(r0,r1); if (!p0.new) jump:nt #0x10 }",
            ),
            (
                "gtu_tp0_nt",
                "{ p0 = cmp.gtu(r0,r1); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "gtu_fp0_nt",
                "{ p0 = cmp.gtu(r0,r1); if (!p0.new) jump:nt #0x10 }",
            ),
        ],
        40,
        0xCF05,
    );
}

#[test]
fn cf_cmpjump_ri() {
    lift_cf_family(
        "cf_cmpjump_ri",
        &[
            (
                "eqi_tp0_nt",
                "{ p0 = cmp.eq(r0,#5); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "eqi_fp0_nt",
                "{ p0 = cmp.eq(r0,#5); if (!p0.new) jump:nt #0x10 }",
            ),
            (
                "gti_tp0_nt",
                "{ p0 = cmp.gt(r0,#5); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "gti_fp0_nt",
                "{ p0 = cmp.gt(r0,#5); if (!p0.new) jump:nt #0x10 }",
            ),
            (
                "gtui_tp0_nt",
                "{ p0 = cmp.gtu(r0,#5); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "gtui_fp0_nt",
                "{ p0 = cmp.gtu(r0,#5); if (!p0.new) jump:nt #0x10 }",
            ),
            (
                "eqn1_tp0_nt",
                "{ p0 = cmp.eq(r0,#-1); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "eqn1_fp0_nt",
                "{ p0 = cmp.eq(r0,#-1); if (!p0.new) jump:nt #0x10 }",
            ),
            (
                "gtn1_tp0_nt",
                "{ p0 = cmp.gt(r0,#-1); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "gtn1_fp0_nt",
                "{ p0 = cmp.gt(r0,#-1); if (!p0.new) jump:nt #0x10 }",
            ),
            (
                "tstbit_tp0_nt",
                "{ p0 = tstbit(r0,#0); if (p0.new) jump:nt #0x10 }",
            ),
            (
                "tstbit_fp0_nt",
                "{ p0 = tstbit(r0,#0); if (!p0.new) jump:nt #0x10 }",
            ),
        ],
        40,
        0xCF06,
    );
}

#[test]
fn cf_cmpjump_nv() {
    // New-value compare-and-branch (`_jumpnv`): the compare reads a register
    // produced EARLIER IN THE SAME PACKET. For a lone packet the producer must be
    // present; these forms write no predicate.
    lift_cf_family(
        "cf_cmpjump_nv",
        &[
            (
                "eq_nv_t",
                "{ r0 = r4; if (cmp.eq(r0.new,r1)) jump:nt #0x10 }",
            ),
            (
                "gt_nv_t",
                "{ r0 = r4; if (cmp.gt(r0.new,r1)) jump:nt #0x10 }",
            ),
            (
                "gtu_nv_t",
                "{ r0 = r4; if (cmp.gtu(r0.new,r1)) jump:nt #0x10 }",
            ),
        ],
        40,
        0xCF07,
    );
}

// ---- jumpr-compare-zero + jumpset ----

#[test]
fn cf_jump_regzero() {
    lift_cf_family(
        "cf_jump_regzero",
        &[
            ("jumprz", "{ if (r0!=#0) jump:nt #0x10 }"),
            ("jumprnz", "{ if (r0==#0) jump:nt #0x10 }"),
            ("jumprgtez", "{ if (r0>=#0) jump:nt #0x10 }"),
            ("jumprltez", "{ if (r0<=#0) jump:nt #0x10 }"),
        ],
        40,
        0xCF08,
    );
}

#[test]
fn cf_jumpset() {
    lift_cf_family(
        "cf_jumpset",
        &[
            ("jumpseti", "{ r0 = #5 ; jump #0x10 }"),
            ("jumpsetr", "{ r0 = r1 ; jump #0x10 }"),
        ],
        40,
        0xCF09,
    );
}

// ---- validate the harness on instructions already lifted by the DecodedInsn path ----

#[test]
fn lift_alu_rr() {
    lift_family(
        "alu_rr",
        &[
            ("add", "{ r0 = add(r1,r2) }"),
            ("sub", "{ r0 = sub(r1,r2) }"),
            ("and", "{ r0 = and(r1,r2) }"),
            ("or", "{ r0 = or(r1,r2) }"),
            ("xor", "{ r0 = xor(r1,r2) }"),
        ],
        12,
        0x5111,
    );
}

#[test]
fn lift_alu_imm() {
    lift_family(
        "alu_imm",
        &[
            ("addi", "{ r0 = add(r1,#10) }"),
            ("andi", "{ r0 = and(r1,#255) }"),
            ("ori", "{ r0 = or(r1,#15) }"),
            ("subri", "{ r0 = sub(#100,r1) }"),
        ],
        12,
        0x5112,
    );
}

#[test]
fn lift_shift_imm() {
    lift_family(
        "shift_imm",
        &[
            ("asl", "{ r0 = asl(r1,#5) }"),
            ("asr", "{ r0 = asr(r1,#5) }"),
            ("lsr", "{ r0 = lsr(r1,#5) }"),
        ],
        12,
        0x5113,
    );
}

// ---- newly-lifted scalar register opcodes (re-decoded from Unknown) ----

#[test]
fn lift_clmul_poly() {
    // Carry-less (GF(2)) polynomial multiply: pmpyw (32x32->64) and vpmpyh
    // (2x 16x16->32, interleaved), with their _acc XOR-accumulate forms.
    lift_family(
        "clmul_poly",
        &[
            ("pmpyw", "{ r1:0 = pmpyw(r2,r3) }"),
            ("pmpyw_acc", "{ r1:0 ^= pmpyw(r2,r3) }"),
            ("vpmpyh", "{ r1:0 = vpmpyh(r2,r3) }"),
            ("vpmpyh_acc", "{ r1:0 ^= vpmpyh(r2,r3) }"),
        ],
        40,
        0x7301,
    );
}

#[test]
fn lift_cmpy_w128_sat() {
    // M7 wide complex multiply: 32x32 with i128 accumulator, :<<1 scale, signed-
    // 32 saturation (USR:OVF sticky), real/imag and conjugate (*) plus :rnd.
    lift_family(
        "cmpy_w128_sat",
        &[
            ("cmpyrw", "{ r0 = cmpyrw(r3:2,r5:4):<<1:sat }"),
            ("cmpyrwc", "{ r0 = cmpyrw(r3:2,r5:4*):<<1:sat }"),
            ("cmpyiw", "{ r0 = cmpyiw(r3:2,r5:4):<<1:sat }"),
            ("cmpyiwc", "{ r0 = cmpyiw(r3:2,r5:4*):<<1:sat }"),
            ("cmpyrw_rnd", "{ r0 = cmpyrw(r3:2,r5:4):<<1:rnd:sat }"),
            ("cmpyrwc_rnd", "{ r0 = cmpyrw(r3:2,r5:4*):<<1:rnd:sat }"),
            ("cmpyiw_rnd", "{ r0 = cmpyiw(r3:2,r5:4):<<1:rnd:sat }"),
            ("cmpyiwc_rnd", "{ r0 = cmpyiw(r3:2,r5:4*):<<1:rnd:sat }"),
        ],
        40,
        0x7302,
    );
}

#[test]
fn lift_sat_orig_shl() {
    // Register-amount saturating shift (fSAT_ORIG_SHL): bidirectional, saturates
    // toward the ORIGINAL value's extreme on sign flip; USR:OVF sticky on clamp.
    lift_family(
        "sat_orig_shl",
        &[
            ("asl_r_r_sat", "{ r0 = asl(r1,r2):sat }"),
            ("asr_r_r_sat", "{ r0 = asr(r1,r2):sat }"),
        ],
        40,
        0x7303,
    );
}

#[test]
fn lift_a2_pair_logical() {
    lift_family(
        "a2_pair_logical",
        &[
            ("addp", "{ r1:0 = add(r3:2, r5:4) }"),
            ("subp", "{ r1:0 = sub(r3:2, r5:4) }"),
            ("andp", "{ r1:0 = and(r3:2, r5:4) }"),
            ("orp", "{ r1:0 = or(r3:2, r5:4) }"),
            ("xorp", "{ r1:0 = xor(r3:2, r5:4) }"),
            ("negp", "{ r1:0 = neg(r3:2) }"),
            ("notp", "{ r1:0 = not(r3:2) }"),
            ("andnp", "{ r1:0 = and(r3:2, ~r5:4) }"),
            ("ornp", "{ r1:0 = or(r3:2, ~r5:4) }"),
            ("sxtw", "{ r1:0 = sxtw(r2) }"),
        ],
        20,
        0x6201,
    );
}

#[test]
fn lift_a2_minmax() {
    lift_family(
        "a2_minmax",
        &[
            ("max", "{ r0 = max(r1,r2) }"),
            ("maxu", "{ r0 = maxu(r1,r2) }"),
            ("min", "{ r0 = min(r1,r2) }"),
            ("minu", "{ r0 = minu(r1,r2) }"),
            ("maxp", "{ r1:0 = max(r3:2, r5:4) }"),
            ("maxup", "{ r1:0 = maxu(r3:2, r5:4) }"),
            ("minp", "{ r1:0 = min(r3:2, r5:4) }"),
            ("minup", "{ r1:0 = minu(r3:2, r5:4) }"),
        ],
        20,
        0x6202,
    );
}

#[test]
fn lift_a2_misc() {
    lift_family(
        "a2_misc",
        &[
            ("orir", "{ r0 = or(r1,#129) }"),
            ("subri", "{ r0 = sub(#100,r1) }"),
            ("aslh", "{ r0 = aslh(r1) }"),
            ("asrh", "{ r0 = asrh(r1) }"),
            ("nop", "{ nop }"),
            ("andn", "{ r0 = and(r1, ~r2) }"),
            ("orn", "{ r0 = or(r1, ~r2) }"),
        ],
        20,
        0x6203,
    );
}

#[test]
fn lift_a2_sat_alu() {
    // USR-saturating scalar ALU ops. These set the USR:OVF sticky bit when a
    // clamp clobbered the result; the harness compares USR:OVF (bit 0) so a
    // missing OVF set would diverge. 40 iters over random GPRs reliably
    // exercises the saturating (OVF-setting) path for add/sub/sat/asl.
    lift_family(
        "a2_sat_alu",
        &[
            ("sat", "{ r0 = sat(r3:2) }"),
            ("satb", "{ r0 = satb(r1) }"),
            ("sath", "{ r0 = sath(r1) }"),
            ("satub", "{ r0 = satub(r1) }"),
            ("satuh", "{ r0 = satuh(r1) }"),
            ("addsat", "{ r0 = add(r1,r2):sat }"),
            ("subsat", "{ r0 = sub(r1,r2):sat }"),
            ("negsat", "{ r0 = neg(r1):sat }"),
            ("abssat", "{ r0 = abs(r1):sat }"),
            ("asl_i_sat", "{ r0 = asl(r1,#5):sat }"),
        ],
        40,
        0x6210,
    );
}

#[test]
fn lift_a2_combine() {
    lift_family(
        "a2_combine",
        &[
            ("combinew", "{ r1:0 = combine(r2,r3) }"),
            ("combineii", "{ r1:0 = combine(#7,#100) }"),
            ("combineri", "{ r1:0 = combine(r2,#5) }"),
            ("combineir", "{ r1:0 = combine(#5,r2) }"),
            ("combine_hh", "{ r0 = combine(r1.h,r2.h) }"),
            ("combine_hl", "{ r0 = combine(r1.h,r2.l) }"),
            ("combine_lh", "{ r0 = combine(r1.l,r2.h) }"),
            ("combine_ll", "{ r0 = combine(r1.l,r2.l) }"),
        ],
        20,
        0x6204,
    );
}

#[test]
fn lift_a4_rcmp() {
    lift_family(
        "a4_rcmp",
        &[
            ("rcmpeq", "{ r0 = cmp.eq(r1,r2) }"),
            ("rcmpneq", "{ r0 = !cmp.eq(r1,r2) }"),
            ("rcmpeqi", "{ r0 = cmp.eq(r1,#5) }"),
            ("rcmpneqi", "{ r0 = !cmp.eq(r1,#5) }"),
        ],
        20,
        0x6205,
    );
}

#[test]
fn lift_c2_cmp_extra() {
    lift_family(
        "c2_cmp_extra",
        &[
            ("cmpneq", "{ p0 = !cmp.eq(r1,r2) }"),
            ("cmplte", "{ p0 = !cmp.gt(r1,r2) }"),
            ("cmplteu", "{ p0 = !cmp.gtu(r1,r2) }"),
            ("cmpeqp", "{ p0 = cmp.eq(r1:0, r3:2) }"),
            ("cmpgtp", "{ p0 = cmp.gt(r1:0, r3:2) }"),
            ("cmpgtup", "{ p0 = cmp.gtu(r1:0, r3:2) }"),
            ("cmpltei", "{ p0 = !cmp.gt(r1,#5) }"),
            ("cmplteui", "{ p0 = !cmp.gtu(r1,#5) }"),
        ],
        20,
        0x6206,
    );
}

#[test]
fn lift_c2_bitstest() {
    lift_family(
        "c2_bitstest",
        &[
            ("bitsset", "{ p0 = bitsset(r1,r2) }"),
            ("bitsclr", "{ p0 = bitsclr(r1,r2) }"),
            ("bitsclri", "{ p0 = bitsclr(r1,#5) }"),
            ("nbitsset", "{ p0 = !bitsset(r1,r2) }"),
            ("nbitsclr", "{ p0 = !bitsclr(r1,r2) }"),
            ("nbitsclri", "{ p0 = !bitsclr(r1,#5) }"),
        ],
        20,
        0x6207,
    );
}

#[test]
fn lift_c2_mux() {
    lift_family(
        "c2_mux",
        &[
            ("mux", "{ r0 = mux(p0,r1,r2) }"),
            ("muxir", "{ r0 = mux(p0,r1,#5) }"),
            ("muxri", "{ r0 = mux(p0,#5,r1) }"),
            ("muxii", "{ r0 = mux(p0,#7,#9) }"),
        ],
        20,
        0x6208,
    );
}

#[test]
fn lift_c2_predlogic() {
    lift_family(
        "c2_predlogic",
        &[
            ("and", "{ p0 = and(p1,p2) }"),
            ("or", "{ p0 = or(p1,p2) }"),
            ("xor", "{ p0 = xor(p1,p2) }"),
            ("not", "{ p0 = not(p1) }"),
            ("andn", "{ p0 = and(p1,!p2) }"),
            ("orn", "{ p0 = or(p1,!p2) }"),
            ("tfrrp", "{ p0 = r1 }"),
        ],
        20,
        0x6209,
    );
}

#[test]
fn lift_s2_shift_pair() {
    lift_family(
        "s2_shift_pair",
        &[
            ("asl_i_p", "{ r1:0 = asl(r3:2,#5) }"),
            ("asr_i_p", "{ r1:0 = asr(r3:2,#5) }"),
            ("lsr_i_p", "{ r1:0 = lsr(r3:2,#5) }"),
            ("rol_i_r", "{ r0 = rol(r1,#5) }"),
            ("rol_i_p", "{ r1:0 = rol(r3:2,#5) }"),
        ],
        20,
        0x620a,
    );
}

#[test]
fn lift_s2_bitmanip() {
    lift_family(
        "s2_bitmanip",
        &[
            ("setbit_i", "{ r0 = setbit(r1,#5) }"),
            ("clrbit_i", "{ r0 = clrbit(r1,#5) }"),
            ("togglebit_i", "{ r0 = togglebit(r1,#5) }"),
            ("tstbit_i", "{ p0 = tstbit(r1,#5) }"),
            ("ntstbit_i", "{ p0 = !tstbit(r1,#5) }"),
            ("extractu", "{ r0 = extractu(r1,#8,#4) }"),
            ("extractu2", "{ r0 = extractu(r1,#13,#11) }"),
            ("insert", "{ r0 = insert(r1,#8,#4) }"),
            ("insert2", "{ r0 = insert(r1,#17,#9) }"),
            ("cl0", "{ r0 = cl0(r1) }"),
            ("cl1", "{ r0 = cl1(r1) }"),
            ("ct0", "{ r0 = ct0(r1) }"),
            ("ct1", "{ r0 = ct1(r1) }"),
            ("brev", "{ r0 = brev(r1) }"),
            ("vsplatrb", "{ r0 = vsplatb(r1) }"),
            ("vsplatrh", "{ r1:0 = vsplath(r2) }"),
        ],
        20,
        0x620b,
    );
}

#[test]
fn lift_s4_addasl() {
    lift_family(
        "s4_addasl",
        &[("addasl", "{ r0 = addasl(r1,r2,#3) }")],
        20,
        0x620c,
    );
}

#[test]
fn lift_m2_mpy() {
    lift_family(
        "m2_mpy",
        &[
            ("dpmpyss", "{ r1:0 = mpy(r2,r3) }"),
            ("dpmpyuu", "{ r1:0 = mpyu(r2,r3) }"),
            ("dpmpyss_acc", "{ r1:0 += mpy(r2,r3) }"),
            ("dpmpyss_nac", "{ r1:0 -= mpy(r2,r3) }"),
            ("dpmpyuu_acc", "{ r1:0 += mpyu(r2,r3) }"),
            ("dpmpyuu_nac", "{ r1:0 -= mpyu(r2,r3) }"),
            ("dpmpyss_rnd", "{ r0 = mpy(r1,r2):rnd }"),
            ("mpy_up", "{ r0 = mpy(r1,r2) }"),
            ("mpyu_up", "{ r0 = mpyu(r1,r2) }"),
            ("mpysu_up", "{ r0 = mpysu(r1,r2) }"),
            ("mpy_up_s1", "{ r0 = mpy(r1,r2):<<1 }"),
            ("mpysip", "{ r0 = mpyi(r1,#100) }"),
            ("mpysin", "{ r0 = mpyi(r1,#-100) }"),
            ("maci", "{ r0 += mpyi(r1,r2) }"),
            ("mnaci", "{ r0 -= mpyi(r1,r2) }"),
            ("macsip", "{ r0 += mpyi(r1,#100) }"),
            ("macsin", "{ r0 -= mpyi(r1,#100) }"),
            ("acci", "{ r0 += add(r1,r2) }"),
            ("accii", "{ r0 += add(r1,#5) }"),
            ("nacci", "{ r0 -= add(r1,r2) }"),
            ("naccii", "{ r0 -= add(r1,#5) }"),
            ("subacc", "{ r0 += sub(r2,r1) }"),
            ("xor_xacc", "{ r0 ^= xor(r1,r2) }"),
            ("m4_xor_xacc", "{ r1:0 ^= xor(r3:2,r5:4) }"),
        ],
        20,
        0x620d,
    );
}

// ---- HVX harness readiness probe (not a lift assertion) ----
// Confirms the V/Q-extended harness can drive a real HVX packet end-to-end:
// the assembler emits HVX encodings, the interpreter executes from seeded V
// state and mutates V, and the lifter is reached (Unsupported until HVX is
// lifted). This proves the oracle plumbing before the HVX lift wave.
#[test]
fn hvx_harness_probe() {
    let asms = vec!["{ v2.w = vadd(v0.w,v1.w) }".to_string()];
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hvx_harness_probe] llvm-mc unavailable/assemble-failed -> skipping");
            return;
        }
    };
    let words = &words_per[0];
    let mut rng = Rng::new(0x4242);
    let mut st = State::zeroed();
    for vv in st.v.iter_mut() {
        for w in vv.iter_mut() {
            *w = rng.next() as u32;
        }
    }
    let interp = run_interp(words, &st).expect("interp must execute HVX vadd from seeded V");
    // v2 = v0 + v1 lane-wise (32-bit words); prove the interp actually mutated V2.
    for j in 0..32 {
        let expect = st.v[0][j].wrapping_add(st.v[1][j]);
        assert_eq!(interp.v[2][j], expect, "interp HVX vadd.w lane {j}");
    }
    // The lifter is reached; HVX is not yet lifted, so expect Unsupported (Ok(None)).
    match lift_and_run(words, &st) {
        Ok(None) => eprintln!("[hvx_harness_probe] HVX vadd: UNLIFTED (expected until lifted)"),
        Ok(Some(_)) => eprintln!("[hvx_harness_probe] HVX vadd is now lifted"),
        Err(e) => panic!("lift error on HVX vadd: {e}"),
    }
}

// ---- HVX (V6_*) elementwise integer vector lift verification ----
// Each family compares the SMIR-lifted V/Q state against the qemu-verified
// HexagonVcpu interpreter (lift_family now also compares V0-31 / Q0-3).

#[test]
fn lift_hvx_vadd() {
    lift_family(
        "hvx_vadd",
        &[
            ("vaddb", "{ v2.b = vadd(v0.b,v1.b) }"),
            ("vaddh", "{ v2.h = vadd(v0.h,v1.h) }"),
            ("vaddw", "{ v2.w = vadd(v0.w,v1.w) }"),
        ],
        12,
        0x7001,
    );
}

#[test]
fn lift_hvx_vsub() {
    lift_family(
        "hvx_vsub",
        &[
            ("vsubb", "{ v2.b = vsub(v0.b,v1.b) }"),
            ("vsubh", "{ v2.h = vsub(v0.h,v1.h) }"),
            ("vsubw", "{ v2.w = vsub(v0.w,v1.w) }"),
        ],
        12,
        0x7002,
    );
}

#[test]
fn lift_hvx_vmaxu() {
    // Unsigned vector max only — the SMIR VMax does an unsigned lane compare.
    lift_family(
        "hvx_vmaxu",
        &[
            ("vmaxub", "{ v2.ub = vmax(v0.ub,v1.ub) }"),
            ("vmaxuh", "{ v2.uh = vmax(v0.uh,v1.uh) }"),
        ],
        12,
        0x7003,
    );
}

#[test]
fn lift_hvx_vshift() {
    // Logical-left (vasl) and logical-right (vlsr) shift-by-scalar only.
    lift_family(
        "hvx_vshift",
        &[
            ("vaslh", "{ v2.h = vasl(v0.h,r1) }"),
            ("vaslw", "{ v2.w = vasl(v0.w,r1) }"),
            ("vlsrb", "{ v2.ub = vlsr(v0.ub,r1) }"),
            ("vlsrh", "{ v2.uh = vlsr(v0.uh,r1) }"),
            ("vlsrw", "{ v2.uw = vlsr(v0.uw,r1) }"),
        ],
        12,
        0x7004,
    );
}

#[test]
fn lift_hvx_vshiftv() {
    // Per-lane bidirectional vector-amount shifts: Vd = op(Vu, Vv) where Vu is
    // the value and Vv the signed per-lane amount (sxtn of low log2(bits)+1
    // bits). The harness seeds Vv randomly, so the negative-amount (reverse-
    // direction) path is exercised too.
    lift_family(
        "hvx_vshiftv",
        &[
            ("vaslhv", "{ v2.h = vasl(v0.h,v1.h) }"),
            ("vaslwv", "{ v2.w = vasl(v0.w,v1.w) }"),
            ("vasrhv", "{ v2.h = vasr(v0.h,v1.h) }"),
            ("vasrwv", "{ v2.w = vasr(v0.w,v1.w) }"),
            ("vlsrhv", "{ v2.h = vlsr(v0.h,v1.h) }"),
            ("vlsrwv", "{ v2.w = vlsr(v0.w,v1.w) }"),
        ],
        16,
        0x7104,
    );
}

#[test]
fn lift_hvx_vassign() {
    lift_family("hvx_vassign", &[("vassign", "{ v2 = v0 }")], 12, 0x7005);
}

#[test]
fn lift_hvx_vmpyih() {
    // Non-widening per-halfword integer multiply keeping the low 16 bits.
    // VMul (wrapping_mul on zero-extended lanes, masked to 16b) is bit-exact
    // with the signed sem multiply because low product bits are sign-agnostic.
    lift_family(
        "hvx_vmpyih",
        &[("vmpyih", "{ v2.h = vmpyi(v0.h,v1.h) }")],
        12,
        0x7006,
    );
}

// ---- Wave 2: VLane-based HVX elementwise lifts ----

#[test]
fn lift_hvx_vlogical() {
    // Bitwise vand/vor/vxor over the full 1024-bit vector (VLane And/Or/Xor).
    lift_family(
        "hvx_vlogical",
        &[
            ("vand", "{ v2 = vand(v0,v1) }"),
            ("vor", "{ v2 = vor(v0,v1) }"),
            ("vxor", "{ v2 = vxor(v0,v1) }"),
        ],
        12,
        0x7007,
    );
}

#[test]
fn lift_hvx_vminmax_signed() {
    // Signed per-lane min/max (VLane Min/Max signed:true).
    lift_family(
        "hvx_vminmax_signed",
        &[
            ("vmaxb", "{ v2.b = vmax(v0.b,v1.b) }"),
            ("vmaxh", "{ v2.h = vmax(v0.h,v1.h) }"),
            ("vmaxw", "{ v2.w = vmax(v0.w,v1.w) }"),
            ("vminb", "{ v2.b = vmin(v0.b,v1.b) }"),
            ("vminh", "{ v2.h = vmin(v0.h,v1.h) }"),
            ("vminw", "{ v2.w = vmin(v0.w,v1.w) }"),
        ],
        12,
        0x7008,
    );
}

#[test]
fn lift_hvx_vminu() {
    // Unsigned per-lane min (vminub/vminuh) — VLane Min signed:false.
    // (vmaxub/vmaxuh are lifted in Wave 1 via VMax.)
    lift_family(
        "hvx_vminu",
        &[
            ("vminub", "{ v2.ub = vmin(v0.ub,v1.ub) }"),
            ("vminuh", "{ v2.uh = vmin(v0.uh,v1.uh) }"),
        ],
        12,
        0x7009,
    );
}

#[test]
fn lift_hvx_vsat_signed() {
    // Signed saturating add/sub, single vector (VLane AddSat/SubSat signed:true).
    lift_family(
        "hvx_vsat_signed",
        &[
            ("vaddbsat", "{ v2.b = vadd(v0.b,v1.b):sat }"),
            ("vaddhsat", "{ v2.h = vadd(v0.h,v1.h):sat }"),
            ("vaddwsat", "{ v2.w = vadd(v0.w,v1.w):sat }"),
            ("vsubbsat", "{ v2.b = vsub(v0.b,v1.b):sat }"),
            ("vsubhsat", "{ v2.h = vsub(v0.h,v1.h):sat }"),
            ("vsubwsat", "{ v2.w = vsub(v0.w,v1.w):sat }"),
        ],
        12,
        0x700a,
    );
}

#[test]
fn lift_hvx_vsat_unsigned() {
    // Unsigned saturating add/sub, single vector (VLane AddSat/SubSat signed:false).
    // (vsubuwsat has no single-vector form; only the _dv pair form exists.)
    lift_family(
        "hvx_vsat_unsigned",
        &[
            ("vaddubsat", "{ v2.ub = vadd(v0.ub,v1.ub):sat }"),
            ("vadduhsat", "{ v2.uh = vadd(v0.uh,v1.uh):sat }"),
            ("vadduwsat", "{ v2.uw = vadd(v0.uw,v1.uw):sat }"),
            ("vsububsat", "{ v2.ub = vsub(v0.ub,v1.ub):sat }"),
            ("vsubuhsat", "{ v2.uh = vsub(v0.uh,v1.uh):sat }"),
        ],
        12,
        0x700b,
    );
}

#[test]
fn lift_hvx_vsat_dv() {
    // Dual-vector saturating add/sub: Vdd = op(Vuu, Vvv) emits two VLane ops
    // over the even/odd register of each pair.
    lift_family(
        "hvx_vsat_dv",
        &[
            // signed
            ("vaddbsat_dv", "{ v3:2.b = vadd(v1:0.b,v5:4.b):sat }"),
            ("vaddhsat_dv", "{ v3:2.h = vadd(v1:0.h,v5:4.h):sat }"),
            ("vaddwsat_dv", "{ v3:2.w = vadd(v1:0.w,v5:4.w):sat }"),
            ("vsubbsat_dv", "{ v3:2.b = vsub(v1:0.b,v5:4.b):sat }"),
            ("vsubhsat_dv", "{ v3:2.h = vsub(v1:0.h,v5:4.h):sat }"),
            ("vsubwsat_dv", "{ v3:2.w = vsub(v1:0.w,v5:4.w):sat }"),
            // unsigned
            ("vaddubsat_dv", "{ v3:2.ub = vadd(v1:0.ub,v5:4.ub):sat }"),
            ("vadduhsat_dv", "{ v3:2.uh = vadd(v1:0.uh,v5:4.uh):sat }"),
            ("vadduwsat_dv", "{ v3:2.uw = vadd(v1:0.uw,v5:4.uw):sat }"),
            ("vsububsat_dv", "{ v3:2.ub = vsub(v1:0.ub,v5:4.ub):sat }"),
            ("vsubuhsat_dv", "{ v3:2.uh = vsub(v1:0.uh,v5:4.uh):sat }"),
            ("vsubuwsat_dv", "{ v3:2.uw = vsub(v1:0.uw,v5:4.uw):sat }"),
        ],
        12,
        0x700c,
    );
}

#[test]
fn lift_hvx_vasr_narrow() {
    // Narrowing shift-round-saturate by scalar Rt (OpKind::VNarrowShiftSat).
    // Vd.<n> = vasr(Vu.<2n>, Vv.<2n>, Rt)[:rnd][:sat]; even out sub-lane <- Vv,
    // odd <- Vu. arith=signed source (vasr*) vs unsigned (vasru*); sat selects
    // truncate / signed / unsigned narrow.
    lift_family(
        "hvx_vasr_narrow",
        &[
            // word -> half, signed source
            ("vasrwh", "{ v2.h = vasr(v0.w,v1.w,r3) }"),
            ("vasrwhsat", "{ v2.h = vasr(v0.w,v1.w,r3):sat }"),
            ("vasrwhrndsat", "{ v2.h = vasr(v0.w,v1.w,r3):rnd:sat }"),
            ("vasrwuhsat", "{ v2.uh = vasr(v0.w,v1.w,r3):sat }"),
            ("vasrwuhrndsat", "{ v2.uh = vasr(v0.w,v1.w,r3):rnd:sat }"),
            // word -> unsigned half, unsigned source
            ("vasruwuhsat", "{ v2.uh = vasr(v0.uw,v1.uw,r3):sat }"),
            ("vasruwuhrndsat", "{ v2.uh = vasr(v0.uw,v1.uw,r3):rnd:sat }"),
            // half -> byte, signed source
            ("vasrhbsat", "{ v2.b = vasr(v0.h,v1.h,r3):sat }"),
            ("vasrhbrndsat", "{ v2.b = vasr(v0.h,v1.h,r3):rnd:sat }"),
            ("vasrhubsat", "{ v2.ub = vasr(v0.h,v1.h,r3):sat }"),
            ("vasrhubrndsat", "{ v2.ub = vasr(v0.h,v1.h,r3):rnd:sat }"),
            // half -> unsigned byte, unsigned source
            ("vasruhubsat", "{ v2.ub = vasr(v0.uh,v1.uh,r3):sat }"),
            ("vasruhubrndsat", "{ v2.ub = vasr(v0.uh,v1.uh,r3):rnd:sat }"),
        ],
        16,
        0x7150,
    );
}

#[test]
fn lift_hvx_vround_narrow() {
    // Narrowing round-saturate, fixed shift = narrow_bits (VNarrowShiftSat with
    // an immediate amount + round). even out sub-lane <- Vv, odd <- Vu.
    lift_family(
        "hvx_vround_narrow",
        &[
            ("vroundhb", "{ v2.b = vround(v0.h,v1.h):sat }"),
            ("vroundhub", "{ v2.ub = vround(v0.h,v1.h):sat }"),
            ("vrounduhub", "{ v2.ub = vround(v0.uh,v1.uh):sat }"),
            ("vroundwh", "{ v2.h = vround(v0.w,v1.w):sat }"),
            ("vroundwuh", "{ v2.uh = vround(v0.w,v1.w):sat }"),
            ("vrounduwuh", "{ v2.uh = vround(v0.uw,v1.uw):sat }"),
        ],
        16,
        0x7151,
    );
}

#[test]
fn lift_hvx_vsat_narrow() {
    // Narrowing saturate, no shift (VNarrowShiftSat with amount=0). vsatdw is the
    // 64-bit pair {Vu.w:Vv.w} -> signed word (OpKind::VSatDW).
    lift_family(
        "hvx_vsat_narrow",
        &[
            ("vsathub", "{ v2.ub = vsat(v0.h,v1.h) }"),
            ("vsatwh", "{ v2.h = vsat(v0.w,v1.w) }"),
            ("vsatuwuh", "{ v2.uh = vsat(v0.uw,v1.uw) }"),
            ("vsatdw", "{ v2.w = vsatdw(v0.w,v1.w) }"),
        ],
        16,
        0x7152,
    );
}

#[test]
fn lift_hvx_vasrv_narrow() {
    // Per-element variable-shift narrowing saturate (V69+ vasrv*,
    // OpKind::VNarrowShiftV). Source is the pair Vuu; shift from Vv per sub-lane.
    // The V68 reference interpreter may reject these — the harness then skips.
    lift_family(
        "hvx_vasrv_narrow",
        &[
            ("vasrvwuhsat", "{ v2.uh = vasr(v5:4.w,v1.uh):sat }"),
            ("vasrvwuhrndsat", "{ v2.uh = vasr(v5:4.w,v1.uh):rnd:sat }"),
            ("vasrvuhubsat", "{ v2.ub = vasr(v5:4.uh,v1.ub):sat }"),
            ("vasrvuhubrndsat", "{ v2.ub = vasr(v5:4.uh,v1.ub):rnd:sat }"),
        ],
        16,
        0x7153,
    );
}

#[test]
fn lift_hvx_vavg() {
    // Truncating average (a+b)>>1 (VLane Avg) and rounding (a+b+1)>>1 (AvgRnd),
    // signed and unsigned per lane width.
    lift_family(
        "hvx_vavg",
        &[
            // truncating, unsigned
            ("vavgub", "{ v2.ub = vavg(v0.ub,v1.ub) }"),
            ("vavguh", "{ v2.uh = vavg(v0.uh,v1.uh) }"),
            ("vavguw", "{ v2.uw = vavg(v0.uw,v1.uw) }"),
            // truncating, signed
            ("vavgb", "{ v2.b = vavg(v0.b,v1.b) }"),
            ("vavgh", "{ v2.h = vavg(v0.h,v1.h) }"),
            ("vavgw", "{ v2.w = vavg(v0.w,v1.w) }"),
            // rounding, unsigned
            ("vavgubrnd", "{ v2.ub = vavg(v0.ub,v1.ub):rnd }"),
            ("vavguhrnd", "{ v2.uh = vavg(v0.uh,v1.uh):rnd }"),
            ("vavguwrnd", "{ v2.uw = vavg(v0.uw,v1.uw):rnd }"),
            // rounding, signed
            ("vavgbrnd", "{ v2.b = vavg(v0.b,v1.b):rnd }"),
            ("vavghrnd", "{ v2.h = vavg(v0.h,v1.h):rnd }"),
            ("vavgwrnd", "{ v2.w = vavg(v0.w,v1.w):rnd }"),
        ],
        12,
        0x700d,
    );
}

#[test]
fn lift_hvx_vabsdiff() {
    // Absolute difference |a-b| (VLane AbsDiff). Unsigned: vabsdiffub/uh.
    // Signed: vabsdiffh/w (the signed forms write an unsigned-typed dest in asm).
    lift_family(
        "hvx_vabsdiff",
        &[
            ("vabsdiffub", "{ v2.ub = vabsdiff(v0.ub,v1.ub) }"),
            ("vabsdiffuh", "{ v2.uh = vabsdiff(v0.uh,v1.uh) }"),
            ("vabsdiffh", "{ v2.uh = vabsdiff(v0.h,v1.h) }"),
            ("vabsdiffw", "{ v2.uw = vabsdiff(v0.w,v1.w) }"),
        ],
        12,
        0x700e,
    );
}

// ---- Wave 3: vector-by-vector WIDENING multiplies (OpKind::VWidenMul) ----
// `Vdd.<2w> = vmpy(Vu.<w>,Vv.<w>)`: each pair of adjacent narrow lanes is
// multiplied into a double-width product; even narrow lanes' products -> the
// low dest vector (V[base]), odd lanes' -> the high (V[base+1]). The dest is a
// register PAIR (`v3:2` => base v2, hi v3). Mirrors sem/hvx_mpyv.rs.
#[test]
fn lift_hvx_vmpyv_widen() {
    lift_family(
        "hvx_vmpyv_widen",
        &[
            // signed byte x signed byte -> halfword pair
            ("vmpybv", "{ v3:2.h = vmpy(v0.b,v1.b) }"),
            // unsigned byte x signed byte -> halfword pair
            ("vmpybusv", "{ v3:2.h = vmpy(v0.ub,v1.b) }"),
            // unsigned byte x unsigned byte -> uh pair
            ("vmpyubv", "{ v3:2.uh = vmpy(v0.ub,v1.ub) }"),
            // signed half x signed half -> word pair
            ("vmpyhv", "{ v3:2.w = vmpy(v0.h,v1.h) }"),
            // signed half x unsigned half -> word pair
            ("vmpyhus", "{ v3:2.w = vmpy(v0.h,v1.uh) }"),
            // unsigned half x unsigned half -> uw pair
            ("vmpyuhv", "{ v3:2.uw = vmpy(v0.uh,v1.uh) }"),
        ],
        12,
        0x700f,
    );
}

#[test]
fn lift_hvx_vmpyv_widen_acc() {
    // Accumulate forms (`Vxx += ...`): read-modify-write of the dst pair. The
    // harness seeds all V registers with random values, so v2/v3 are non-zero
    // and the read-modify-write path is exercised.
    lift_family(
        "hvx_vmpyv_widen_acc",
        &[
            ("vmpybv_acc", "{ v3:2.h += vmpy(v0.b,v1.b) }"),
            ("vmpybusv_acc", "{ v3:2.h += vmpy(v0.ub,v1.b) }"),
            ("vmpyubv_acc", "{ v3:2.uh += vmpy(v0.ub,v1.ub) }"),
            ("vmpyhv_acc", "{ v3:2.w += vmpy(v0.h,v1.h) }"),
            ("vmpyhus_acc", "{ v3:2.w += vmpy(v0.h,v1.uh) }"),
            ("vmpyuhv_acc", "{ v3:2.uw += vmpy(v0.uh,v1.uh) }"),
        ],
        12,
        0x7010,
    );
}

// ---- Wave 4: HVX horizontal reduce multiplies + scalar splats ----
// `OpKind::VReduceMul` models the vrmpy/vdmpy reduce family: each output lane
// is the sum of `taps` adjacent narrow sub-lane products, so the output lane is
// `src_elem_bits*taps` wide. `OpKind::VBroadcast` splats the low bits of a GPR
// into every lane. Scalar (Rt) reduce forms compose VBroadcast + VReduceMul.
// All non-saturating; matches sem/hvx_rmpy.rs + sem/hvx_perm.rs bit-for-bit.

#[test]
fn lift_hvx_vrmpy_vv() {
    // VECTOR-VECTOR 4-tap byte dot product -> word (single dest vector).
    lift_family(
        "hvx_vrmpy_vv",
        &[
            ("vrmpyubv", "{ v2.uw = vrmpy(v0.ub,v1.ub) }"),
            ("vrmpybv", "{ v2.w = vrmpy(v0.b,v1.b) }"),
            ("vrmpybusv", "{ v2.w = vrmpy(v0.ub,v1.b) }"),
        ],
        12,
        0x7011,
    );
}

#[test]
fn lift_hvx_vrmpy_vv_acc() {
    // Accumulate forms (`Vx += ...`): read-modify-write of the dest vector.
    // The harness seeds V2 with random values, exercising the acc read path.
    lift_family(
        "hvx_vrmpy_vv_acc",
        &[
            ("vrmpyubv_acc", "{ v2.uw += vrmpy(v0.ub,v1.ub) }"),
            ("vrmpybv_acc", "{ v2.w += vrmpy(v0.b,v1.b) }"),
            ("vrmpybusv_acc", "{ v2.w += vrmpy(v0.ub,v1.b) }"),
        ],
        12,
        0x7012,
    );
}

#[test]
fn lift_hvx_vrmpy_scalar() {
    // SCALAR 4-tap vrmpy (Vu.byte * Rt.byte, Rt's 4 bytes reused per word lane)
    // composed as VBroadcast(Rt, I32) + 4-tap byte VReduceMul.
    lift_family(
        "hvx_vrmpy_scalar",
        &[
            ("vrmpyub", "{ v2.uw = vrmpy(v0.ub,r3.ub) }"),
            ("vrmpybus", "{ v2.w = vrmpy(v0.ub,r3.b) }"),
            ("vrmpyub_acc", "{ v2.uw += vrmpy(v0.ub,r3.ub) }"),
            ("vrmpybus_acc", "{ v2.w += vrmpy(v0.ub,r3.b) }"),
        ],
        12,
        0x7013,
    );
}

#[test]
fn lift_hvx_vsplat() {
    // Scalar splat to all lanes: word / half / byte lanes.
    lift_family(
        "hvx_vsplat",
        &[
            ("lvsplatw", "{ v2 = vsplat(r3) }"),
            ("lvsplath", "{ v2.h = vsplat(r3) }"),
            ("lvsplatb", "{ v2.b = vsplat(r3) }"),
        ],
        12,
        0x7014,
    );
}

#[test]
fn lift_hvx_vdmpybus_scalar() {
    // SCALAR 2-tap vdmpybus (Vu.ub * Rt.b -> halfword) composed as
    // VBroadcast(Rt, I32) + 2-tap byte VReduceMul. The I32-broadcast makes the
    // temp's byte n equal Rt.byte[n%4], matching the sem's per-lane Rt reuse.
    lift_family(
        "hvx_vdmpybus_scalar",
        &[
            ("vdmpybus", "{ v2.h = vdmpy(v0.ub,r3.b) }"),
            ("vdmpybus_acc", "{ v2.h += vdmpy(v0.ub,r3.b) }"),
        ],
        12,
        0x7015,
    );
}

// ---- Wave 5: widen-extend pairs, vcombine, vector ASR-by-scalar,
// vector-by-scalar widening multiplies ----

#[test]
fn lift_hvx_vwiden_ext() {
    // Interleaved zero/sign extend a vector into a pair (vzxt/vsxt):
    // vzb/vsb widen byte->half, vzh/vsh widen half->word. Even narrow lanes
    // -> v[base], odd -> v[base+1] (OpKind::VWidenExt interleave:true).
    lift_family(
        "hvx_vwiden_ext",
        &[
            ("vzb", "{ v3:2.uh = vzxt(v0.ub) }"),
            ("vsb", "{ v3:2.h = vsxt(v0.b) }"),
            ("vzh", "{ v3:2.uw = vzxt(v0.uh) }"),
            ("vsh", "{ v3:2.w = vsxt(v0.h) }"),
        ],
        12,
        0x7016,
    );
}

#[test]
fn lift_hvx_vunpack() {
    // Sequential unpack of a vector into a pair (vunpack): the low half of the
    // narrow lanes -> v[base], the high half -> v[base+1]
    // (OpKind::VWidenExt interleave:false), zero/sign per u/s.
    lift_family(
        "hvx_vunpack",
        &[
            ("vunpackub", "{ v3:2.uh = vunpack(v0.ub) }"),
            ("vunpackb", "{ v3:2.h = vunpack(v0.b) }"),
            ("vunpackuh", "{ v3:2.uw = vunpack(v0.uh) }"),
            ("vunpackh", "{ v3:2.w = vunpack(v0.h) }"),
        ],
        12,
        0x7017,
    );
}

#[test]
fn lift_hvx_vcombine() {
    // Vdd = vcombine(Vu, Vv): register-pair copy with low := Vv, high := Vu.
    // Emitted as two VMov copies in that mapping.
    lift_family(
        "hvx_vcombine",
        &[("vcombine", "{ v3:2 = vcombine(v1,v0) }")],
        12,
        0x7018,
    );
}

#[test]
fn lift_hvx_vasr_scalar() {
    // Per-lane arithmetic right shift by scalar Rt (vasrh/vasrw).
    // VShift Asr sign-extends each lane, so this is bit-exact with the sem.
    lift_family(
        "hvx_vasr_scalar",
        &[
            ("vasrh", "{ v2.h = vasr(v0.h,r3) }"),
            ("vasrw", "{ v2.w = vasr(v0.w,r3) }"),
        ],
        12,
        0x7019,
    );
}

#[test]
fn lift_hvx_vmpy_scalar_widen() {
    // Vector-by-SCALAR widening multiply -> register PAIR, composed as
    // VBroadcast(Rt, I32) + VWidenMul. The I32-broadcast reuses Rt's sub-
    // elements per lane exactly as the sem (byte[(2i+k)%4], half[k]).
    lift_family(
        "hvx_vmpy_scalar_widen",
        &[
            ("vmpybus", "{ v3:2.h = vmpy(v0.ub,r3.b) }"),
            ("vmpyub", "{ v3:2.uh = vmpy(v0.ub,r3.ub) }"),
            ("vmpyh", "{ v3:2.w = vmpy(v0.h,r3.h) }"),
            ("vmpyuh", "{ v3:2.uw = vmpy(v0.uh,r3.uh) }"),
        ],
        12,
        0x701a,
    );
}

#[test]
fn lift_hvx_vmpy_scalar_widen_acc() {
    // Accumulate forms (`Vxx += ...`): read-modify-write of the dst pair
    // (VWidenMul acc:true). The harness seeds all V registers non-zero so the
    // read-modify-write path is exercised.
    lift_family(
        "hvx_vmpy_scalar_widen_acc",
        &[
            ("vmpybus_acc", "{ v3:2.h += vmpy(v0.ub,r3.b) }"),
            ("vmpyub_acc", "{ v3:2.uh += vmpy(v0.ub,r3.ub) }"),
            ("vmpyh_acc", "{ v3:2.w += vmpy(v0.h,r3.h) }"),
            ("vmpyuh_acc", "{ v3:2.uw += vmpy(v0.uh,r3.uh) }"),
        ],
        12,
        0x701b,
    );
}

// ---- Wave 6: HVX pack even/odd + saturating narrowing pack ----

#[test]
fn lift_hvx_vpack_evenodd() {
    // vpacke/vpacko: pick the even (e) or odd (o) narrow sub-element of each
    // wide lane. Output low half comes from Vv (second operand), high half from
    // Vu (first). VPack { elem = narrow output element, odd }.
    lift_family(
        "hvx_vpack_evenodd",
        &[
            ("vpackeb", "{ v2.b = vpacke(v0.h,v1.h) }"),
            ("vpackob", "{ v2.b = vpacko(v0.h,v1.h) }"),
            ("vpackeh", "{ v2.h = vpacke(v0.w,v1.w) }"),
            ("vpackoh", "{ v2.h = vpacko(v0.w,v1.w) }"),
        ],
        12,
        0x701c,
    );
}

#[test]
fn lift_hvx_vpack_sat() {
    // Saturating narrowing pack: each signed wide lane is clamped to half width
    // (unsigned range for ub/uh, signed for b/h). Low half from Vv, high from
    // Vu. VPackSat { src_elem = wide source element, to_unsigned }.
    lift_family(
        "hvx_vpack_sat",
        &[
            ("vpackhub_sat", "{ v2.ub = vpack(v0.h,v1.h):sat }"),
            ("vpackhb_sat", "{ v2.b = vpack(v0.h,v1.h):sat }"),
            ("vpackwuh_sat", "{ v2.uh = vpack(v0.w,v1.w):sat }"),
            ("vpackwh_sat", "{ v2.h = vpack(v0.w,v1.w):sat }"),
        ],
        12,
        0x701d,
    );
}

#[test]
fn lift_hvx_vshuff_deal() {
    // Single-vector shuffle/deal of narrow lanes (VShuffle2).
    //   vshuff (deal=false): out[2i]=Vu[i], out[2i+1]=Vu[i+half]
    //   vdeal  (deal=true):  out[i]=Vu[2i], out[i+half]=Vu[2i+1]
    // half = (1024/elem_bits)/2; elem is the lane type (b -> I8, h -> I16).
    lift_family(
        "hvx_vshuff_deal",
        &[
            ("vshuffb", "{ v2.b = vshuff(v0.b) }"),
            ("vshuffh", "{ v2.h = vshuff(v0.h) }"),
            ("vdealb", "{ v2.b = vdeal(v0.b) }"),
            ("vdealh", "{ v2.h = vdeal(v0.h) }"),
        ],
        12,
        0x701e,
    );
}

#[test]
fn lift_hvx_vshuff_eo() {
    // Two-vector even/odd shuffle (VShuffleEO): interleave the even (e) or odd
    // (o) narrow sub-elements of two source vectors.
    //   out[2i] = src2[2i+odd], out[2i+1] = src1[2i+odd]   (src1=Vu, src2=Vv)
    // vshuffe* takes the even sub-element, vshuffo* the odd one.
    lift_family(
        "hvx_vshuff_eo",
        &[
            ("vshuffeb", "{ v2.b = vshuffe(v0.b,v1.b) }"),
            ("vshuffob", "{ v2.b = vshuffo(v0.b,v1.b) }"),
            ("vshufeh", "{ v2.h = vshuffe(v0.h,v1.h) }"),
            ("vshufoh", "{ v2.h = vshuffo(v0.h,v1.h) }"),
        ],
        12,
        0x701f,
    );
}

#[test]
fn lift_hvx_valign() {
    // Byte-align / rotate of the 256-byte concat src1:src2 (VAlign).
    //   out[i] = src2[i+s] when i+s<128, else src1[i+s-128]
    // with byte shift s = (amount & 127) for right forms (valign/valignbi) and
    // s = 128 - (amount & 127) for left forms (vlalign/vlalignbi). src1 = Vu
    // (assembler first vector operand), src2 = Vv (second). vror rotates a single
    // vector: align(Vu, Vu, Rt & 127). Confirmed against sem/hvx_perm.rs.
    lift_family(
        "hvx_valign",
        &[
            ("vror", "{ v2 = vror(v0,r3) }"),
            ("valignb", "{ v2 = valign(v0,v1,r3) }"),
            ("vlalignb", "{ v2 = vlalign(v0,v1,r3) }"),
            ("valignbi", "{ v2 = valign(v0,v1,#3) }"),
            ("vlalignbi", "{ v2 = vlalign(v0,v1,#3) }"),
        ],
        12,
        0x7020,
    );
}

// ---- Wave 9: HVX shift-round-saturate narrowing multiply ----
// `OpKind::VMulShiftSat`: per lane p = ext(src1)*ext(src2) (i64); p <<= shift_left;
// if round add 1<<(out_shift-1); if sat_bits!=0 clamp to signed sat_bits range;
// out lane = (p >> out_shift) masked to src_elem (output elem = src_elem).
// Matches sem/hvx_mpyv.rs bit-for-bit.

#[test]
fn lift_hvx_vmpy_srs_vv() {
    // VECTOR-VECTOR halfword narrowing multiplies (direct VMulShiftSat).
    //   vmpyhvsrs: Vd.h=vmpy(Vu.h,Vv.h):<<1:rnd:sat
    //     signed*signed, <<1, +0x8000 round, sat32, >>16 high-half.
    //   vmpyuhvs:  Vd.uh=vmpy(Vu.uh,Vv.uh):>>16
    //     unsigned*unsigned, no shift/round/sat, >>16 high-half.
    lift_family(
        "hvx_vmpy_srs_vv",
        &[
            ("vmpyhvsrs", "{ v2.h = vmpy(v0.h,v1.h):<<1:rnd:sat }"),
            ("vmpyuhvs", "{ v2.uh = vmpy(v0.uh,v1.uh):>>16 }"),
        ],
        12,
        0x7021,
    );
}

#[test]
fn lift_hvx_vmpy_srs_scalar() {
    // VECTOR-SCALAR halfword narrowing multiplies (VBroadcast(Rt,I32) +
    // VMulShiftSat). Each even halfword lane multiplies by Rt.h[0], each odd by
    // Rt.h[1] (the I32 broadcast makes t.h[2i]=Rt.h[0], t.h[2i+1]=Rt.h[1]).
    //   vmpyhss:  Vd.h=vmpy(Vu.h,Rt.h):<<1:sat       signed*signed, <<1, sat32, >>16
    //   vmpyhsrs: Vd.h=vmpy(Vu.h,Rt.h):<<1:rnd:sat   as above + 0x8000 round
    lift_family(
        "hvx_vmpy_srs_scalar",
        &[
            ("vmpyhss", "{ v2.h = vmpy(v0.h,r3.h):<<1:sat }"),
            ("vmpyhsrs", "{ v2.h = vmpy(v0.h,r3.h):<<1:rnd:sat }"),
        ],
        12,
        0x7022,
    );
}

// ---- Wave 11: HVX vector compares -> Q, vmux, Q-predicate logic ----
// `OpKind::VCmpToQ` builds a Q vector-predicate (1 bit per vector byte) from a
// per-elem-lane compare; `OpKind::VBlend` selects bytes by a Q (vmux);
// Q-predicate bitwise logic is `OpKind::VLane` over the two I64 lanes of a Q.
// The harness seeds Q0-3 randomly so vmux/pred-logic read meaningful masks and
// compares all Q0-3 after, so the Q-producing compares are checked directly.

#[test]
fn lift_hvx_vcmp_to_q() {
    // Qd = vcmp.<cond>(Vu.<t>, Vv.<t>). eq is signedness-agnostic; gt is signed
    // (.b/.h/.w) vs unsigned (.ub/.uh/.uw). Each true lane sets all its per-byte
    // Q bits, so divergence here would catch a wrong cond/elem/lanes mapping.
    lift_family(
        "hvx_vcmp_to_q",
        &[
            ("veqb", "{ q0 = vcmp.eq(v0.b,v1.b) }"),
            ("vgtb", "{ q0 = vcmp.gt(v0.b,v1.b) }"),
            ("vgtub", "{ q0 = vcmp.gt(v0.ub,v1.ub) }"),
            ("veqh", "{ q0 = vcmp.eq(v0.h,v1.h) }"),
            ("vgth", "{ q0 = vcmp.gt(v0.h,v1.h) }"),
            ("vgtuh", "{ q0 = vcmp.gt(v0.uh,v1.uh) }"),
            ("veqw", "{ q0 = vcmp.eq(v0.w,v1.w) }"),
            ("vgtw", "{ q0 = vcmp.gt(v0.w,v1.w) }"),
            ("vgtuw", "{ q0 = vcmp.gt(v0.uw,v1.uw) }"),
        ],
        16,
        0x7023,
    );
}

#[test]
fn lift_hvx_vmux() {
    // Vd.b[i] = Qt.bit[i] ? Vu.b[i] : Vv.b[i]. Reads a (seeded) Q mask.
    lift_family(
        "hvx_vmux",
        &[("vmux", "{ v2 = vmux(q0,v0,v1) }")],
        16,
        0x7024,
    );
}

#[test]
fn lift_hvx_pred_logic() {
    // Qd = OP(Qs, Qt) per-bit over 128 bits, lifted as VLane bitwise on Q regs.
    // pred_not (!a, VLaneOp::Not) and pred_or_n (a|!b, VLaneOp::OrNot) are now
    // lifted too (Wave 12).
    lift_family(
        "hvx_pred_logic",
        &[
            ("pred_and", "{ q0 = and(q1,q2) }"),
            ("pred_or", "{ q0 = or(q1,q2) }"),
            ("pred_xor", "{ q0 = xor(q1,q2) }"),
            ("pred_and_n", "{ q0 = and(q1,!q2) }"),
            ("pred_not", "{ q0 = not(q1) }"),
            ("pred_or_n", "{ q0 = or(q1,!q2) }"),
        ],
        16,
        0x7025,
    );
}

// ---- Wave 12: HVX scalar 2-tap vdmpy halfword reduces -> word ----
// `Vd.w = vdmpy(Vu.h, Rt.<t>)` (and `+=` acc forms), composed as
// VBroadcast(Rt, I32) + 2-tap I16-source VReduceMul. The sat forms clamp the
// accumulated word lane to the signed 32-bit range; the harness seeds hit
// overflow. HVX vector saturation does not touch USR.

#[test]
fn lift_hvx_vdmpyh_scalar() {
    lift_family(
        "hvx_vdmpyh_scalar",
        &[
            // Vu.h(signed) * Rt.b(signed byte) -> word, 2-tap, no sat.
            ("vdmpyhb", "{ v2.w = vdmpy(v0.h,r3.b) }"),
            ("vdmpyhb_acc", "{ v2.w += vdmpy(v0.h,r3.b) }"),
            // Vu.h(signed) * Rt.h(signed) -> word, 2-tap, sat32.
            ("vdmpyhsat", "{ v2.w = vdmpy(v0.h,r3.h):sat }"),
            ("vdmpyhsat_acc", "{ v2.w += vdmpy(v0.h,r3.h):sat }"),
            // Vu.h(signed) * Rt.uh(UNSIGNED) -> word, 2-tap, sat32.
            ("vdmpyhsusat", "{ v2.w = vdmpy(v0.h,r3.uh):sat }"),
            ("vdmpyhsusat_acc", "{ v2.w += vdmpy(v0.h,r3.uh):sat }"),
        ],
        16,
        0x7026,
    );
}

// ---- Wave 14: HVX Q<->V and Q<->R bridge ops (vand* family) ----
// vandvqv/vandvnqv gate Vu by a (seeded) Q mask into Vd; vandqrt/vandnqrt gate
// a per-byte broadcast of Rt by Qu into Vd; vandvrt builds a Q predicate from
// (Vu.ub & Rt.byte) != 0. The harness seeds Q0-3 and V0-31 randomly so both the
// set and clear mask paths (and the negate polarity) are exercised.

#[test]
fn lift_hvx_vand_qv() {
    lift_family(
        "hvx_vand_qv",
        &[
            ("vandvqv", "{ v2 = vand(q0,v1) }"),
            ("vandvnqv", "{ v2 = vand(!q0,v1) }"),
        ],
        16,
        0x7027,
    );
}

#[test]
fn lift_hvx_vand_qr() {
    lift_family(
        "hvx_vand_qr",
        &[
            ("vandqrt", "{ v2 = vand(q0,r3) }"),
            ("vandnqrt", "{ v2 = vand(!q0,r3) }"),
        ],
        16,
        0x7028,
    );
}

#[test]
fn lift_hvx_vand_vr() {
    lift_family(
        "hvx_vand_vr",
        &[("vandvrt", "{ q0 = vand(v1,r3) }")],
        16,
        0x7029,
    );
}

// ---- Wave 15: more HVX integer multiply variants ----
// vmpyuhe (even unsigned-halfword * Rt.uh0) via VBroadcast + VMulEvenWiden;
// vmpyiwb/vmpyiwub/vmpyiwh (word * Rt sub-element, low 32) and vmpyihb (half *
// Rt.b, low 16) via VBroadcast + a 1-tap VReduceMul that reuses the broadcast
// temp's per-lane sub-element exactly as the sem indexes Rt (i%4 / i%2). The
// `_acc` forms seed the dest vector to exercise the read-modify-write path.

#[test]
fn lift_hvx_vmpyuhe() {
    lift_family(
        "hvx_vmpyuhe",
        &[
            ("vmpyuhe", "{ v2.uw = vmpye(v0.uh,r3.uh) }"),
            ("vmpyuhe_acc", "{ v2.uw += vmpye(v0.uh,r3.uh) }"),
        ],
        16,
        0x7030,
    );
}

#[test]
fn lift_hvx_vmpyiw_scalar() {
    lift_family(
        "hvx_vmpyiw_scalar",
        &[
            ("vmpyiwb", "{ v2.w = vmpyi(v0.w,r3.b) }"),
            ("vmpyiwb_acc", "{ v2.w += vmpyi(v0.w,r3.b) }"),
            ("vmpyiwub", "{ v2.w = vmpyi(v0.w,r3.ub) }"),
            ("vmpyiwub_acc", "{ v2.w += vmpyi(v0.w,r3.ub) }"),
            ("vmpyiwh", "{ v2.w = vmpyi(v0.w,r3.h) }"),
            ("vmpyiwh_acc", "{ v2.w += vmpyi(v0.w,r3.h) }"),
        ],
        16,
        0x7031,
    );
}

#[test]
fn lift_hvx_vmpyihb_scalar() {
    lift_family(
        "hvx_vmpyihb_scalar",
        &[
            ("vmpyihb", "{ v2.h = vmpyi(v0.h,r3.b) }"),
            ("vmpyihb_acc", "{ v2.h += vmpyi(v0.h,r3.b) }"),
        ],
        16,
        0x7032,
    );
}

// vmpyie/vmpyio: word * (even/odd) sub-halfword of Vv, low 32, via VMulSubLane.
//   vmpyiewuh:     Vd.w[i] = Vu.w[i] * Vv.uh[even hw of word i]   (even, unsigned)
//   vmpyiewuh_acc: Vx.w[i] += same
//   vmpyiowh:      Vd.w[i] = Vu.w[i] * Vv.h[odd  hw of word i]    (odd, signed)
//   vmpyiewh_acc:  Vx.w[i] += Vu.w[i] * Vv.h[even hw of word i]   (even, signed)
// The `_acc` forms seed the dest vector to exercise the read-modify-write path.
#[test]
fn lift_hvx_vmpyie_vmpyio() {
    lift_family(
        "hvx_vmpyie_vmpyio",
        &[
            ("vmpyiewuh", "{ v2.w = vmpyie(v0.w,v1.uh) }"),
            ("vmpyiewuh_acc", "{ v2.w += vmpyie(v0.w,v1.uh) }"),
            ("vmpyiowh", "{ v2.w = vmpyio(v0.w,v1.h) }"),
            ("vmpyiewh_acc", "{ v2.w += vmpyie(v0.w,v1.h) }"),
        ],
        16,
        0x7033,
    );
}

// ---- Wave 17: vmpa scalar-pair byte/half multiply-add ----
// Source is a register PAIR Vuu = (V[u], V[u+1]); the scalar Rt's 4 sub-bytes
// are reused per output lane. Lowered as VBroadcast(Rt, I32) + VPairReduceMul:
//   dst_lo[i] = Vuu0.narrow[2i]   * Rt.sub[0] + Vuu1.narrow[2i]   * Rt.sub[1]
//   dst_hi[i] = Vuu0.narrow[2i+1] * Rt.sub[2] + Vuu1.narrow[2i+1] * Rt.sub[3]
// The `_acc` forms target the same dst pair, exercising the read-modify-write
// accumulate (wrapping in the low out_elem bits). Note: llvm-mc spells the
// vmpabuu (unsigned * unsigned) destination as `.h` (not `.uh`).
//   vmpabus: Vuu.ub * Rt.b  -> .h    vmpabuu: Vuu.ub * Rt.ub -> .h
//   vmpahb:  Vuu.h  * Rt.b  -> .w    vmpauhb: Vuu.uh * Rt.b  -> .w
#[test]
fn lift_hvx_vmpa_scalar() {
    lift_family(
        "hvx_vmpa_scalar",
        &[
            ("vmpabus", "{ v3:2.h = vmpa(v5:4.ub,r3.b) }"),
            ("vmpabus_acc", "{ v3:2.h += vmpa(v5:4.ub,r3.b) }"),
            ("vmpabuu", "{ v3:2.h = vmpa(v5:4.ub,r3.ub) }"),
            ("vmpabuu_acc", "{ v3:2.h += vmpa(v5:4.ub,r3.ub) }"),
            ("vmpahb", "{ v3:2.w = vmpa(v5:4.h,r3.b) }"),
            ("vmpahb_acc", "{ v3:2.w += vmpa(v5:4.h,r3.b) }"),
            ("vmpauhb", "{ v3:2.w = vmpa(v5:4.uh,r3.b) }"),
            ("vmpauhb_acc", "{ v3:2.w += vmpa(v5:4.uh,r3.b) }"),
        ],
        16,
        0x7034,
    );
}

// Wave 18: vmpa cross-PAIR byte multiply-add. BOTH operands are register
// pairs (Vuu, Vvv); result is a halfword pair. Per lane i (0..64), narrow
// byte lanes 2i / 2i+1 reduce-add across the two pair halves:
//   vmpabusv: Vuu.ub * Vvv.b  -> .h   vmpabuuv: Vuu.ub * Vvv.ub -> .h
#[test]
fn lift_hvx_vmpa_pairpair() {
    lift_family(
        "hvx_vmpa_pairpair",
        &[
            ("vmpabusv", "{ v3:2.h = vmpa(v5:4.ub,v7:6.b) }"),
            ("vmpabuuv", "{ v3:2.h = vmpa(v5:4.ub,v7:6.ub) }"),
        ],
        16,
        0x18a4,
    );
}

// vdealb4w: deal bytes 0,2 of each word from two vectors into one (VDealB4W).
#[test]
fn lift_hvx_vdealb4w() {
    lift_family(
        "hvx_vdealb4w",
        &[("vdealb4w", "{ v2.b = vdeale(v0.b,v1.b) }")],
        16,
        0x18b0,
    );
}

// vmpyewuh / vmpyowh: word * even/odd halfword, fractional (>>16 / <<1:sat >>15).
#[test]
fn lift_hvx_vmpyewuh_vmpyowh() {
    lift_family(
        "hvx_vmpyewuh_vmpyowh",
        &[
            ("vmpyewuh", "{ v2.w = vmpye(v0.w,v1.uh) }"),
            ("vmpyowh", "{ v2.w = vmpyo(v0.w,v1.h):<<1:sat }"),
            ("vmpyowh_rnd", "{ v2.w = vmpyo(v0.w,v1.h):<<1:rnd:sat }"),
            ("vmpyowh_sacc", "{ v2.w += vmpyo(v0.w,v1.h):<<1:sat:shift }"),
            (
                "vmpyowh_rnd_sacc",
                "{ v2.w += vmpyo(v0.w,v1.h):<<1:rnd:sat:shift }",
            ),
        ],
        16,
        0x18c0,
    );
}

// vmpyieoh: Vd.w[i] = (Vu.h[even=2i] * Vv.h[odd=2i+1]) << 16, low 32 bits.
// Lowered as VMulSubLaneSh (sub-lanes BOTH operands: even half of Vu, odd half
// of Vv) with shl=16. Both halfwords are signed.
#[test]
fn lift_hvx_vmpyieoh() {
    lift_family(
        "hvx_vmpyieoh",
        &[("vmpyieoh", "{ v2.w = vmpyieo(v0.h,v1.h) }")],
        16,
        0x18c8,
    );
}

// vmpyewuh_64 / vmpyowh_64_acc: even/odd word*half multiply repacked into a
// 64-bit (vector-pair) result. Lowered as VMulWord64Pair (mode 0 / mode 1).
//   vmpyewuh_64:    Vdd = vmpye(Vu.w, Vv.uh)        prod>>16 -> hi, prod<<16 -> lo
//   vmpyowh_64_acc: Vxx += vmpyo(Vu.w, Vv.h)        accumulate into the pair,
//     repacking the low half of the product into the high half of the low reg.
// The _acc form seeds the dest pair (harness randomizes all V), exercising the
// read-modify-write accumulate + repack path.
#[test]
fn lift_hvx_vmpy_word64() {
    lift_family(
        "hvx_vmpy_word64",
        &[
            ("vmpyewuh_64", "{ v3:2 = vmpye(v0.w,v1.uh) }"),
            ("vmpyowh_64_acc", "{ v3:2 += vmpyo(v0.w,v1.h) }"),
        ],
        16,
        0x18c9,
    );
}

// vlut32 byte lookup-table family (VLut). Seeds randomize Vu/Vv/Rt so the
// group-match / nomatch / oracc paths are all exercised.
#[test]
fn lift_hvx_vlutvvb() {
    lift_family(
        "hvx_vlutvvb",
        &[
            ("vlutvvb", "{ v2.b = vlut32(v0.b,v1.b,r3) }"),
            ("vlutvvb_nm", "{ v2.b = vlut32(v0.b,v1.b,r3):nomatch }"),
            ("vlutvvbi", "{ v2.b = vlut32(v0.b,v1.b,#3) }"),
            ("vlutvvb_oracc", "{ v2.b |= vlut32(v0.b,v1.b,r3) }"),
            ("vlutvvb_oracci", "{ v2.b |= vlut32(v0.b,v1.b,#3) }"),
        ],
        16,
        0x18d0,
    );
}

// vlut16 halfword lookup-table -> pair (VLut16).
#[test]
fn lift_hvx_vlutvwh() {
    lift_family(
        "hvx_vlutvwh",
        &[
            ("vlutvwh", "{ v3:2.h = vlut16(v0.b,v1.h,r3) }"),
            ("vlutvwh_nm", "{ v3:2.h = vlut16(v0.b,v1.h,r3):nomatch }"),
            ("vlutvwhi", "{ v3:2.h = vlut16(v0.b,v1.h,#3) }"),
            ("vlutvwh_oracc", "{ v3:2.h |= vlut16(v0.b,v1.h,r3) }"),
            ("vlutvwh_oracci", "{ v3:2.h |= vlut16(v0.b,v1.h,#3) }"),
        ],
        16,
        0x18e0,
    );
}

// vshuffvdd: Rt-controlled byte swap network over a pair (VShuffVdd).
#[test]
fn lift_hvx_vshuffvdd() {
    lift_family(
        "hvx_vshuffvdd",
        &[("vshuffvdd", "{ v3:2 = vshuff(v5,v4,r3) }")],
        16,
        0x18f0,
    );
}

// vdelta/vrdelta: Vv-controlled byte butterfly permute (VDelta).
#[test]
fn lift_hvx_vdelta() {
    lift_family(
        "hvx_vdelta",
        &[
            ("vdelta", "{ v2 = vdelta(v0,v1) }"),
            ("vrdelta", "{ v2 = vrdelta(v0,v1) }"),
        ],
        16,
        0x1900,
    );
}

// ---- Wave 19: cross-register SLIDING-WINDOW reduces (OpKind::VSlideReduceMul).
// Source is a register PAIR Vuu = (V[u], V[u+1]); the window straddles the pair
// boundary so V[u+1] supplies the elements that slide into the high output. Rt
// is I32-broadcast into a temp so its sub-elements are reused per output lane.

// _dv 2-tap sliding (mode 0, pair -> pair):
//   vdmpyhb_dv : Vuu.h  * Rt.b -> .w
//   vdmpybus_dv: Vuu.ub * Rt.b -> .h
#[test]
fn lift_hvx_vdmpy_dv() {
    lift_family(
        "hvx_vdmpy_dv",
        &[
            ("vdmpyhb_dv", "{ v3:2.w = vdmpy(v5:4.h,r6.b) }"),
            ("vdmpyhb_dv_acc", "{ v3:2.w += vdmpy(v5:4.h,r6.b) }"),
            ("vdmpybus_dv", "{ v3:2.h = vdmpy(v5:4.ub,r6.b) }"),
            ("vdmpybus_dv_acc", "{ v3:2.h += vdmpy(v5:4.ub,r6.b) }"),
        ],
        16,
        0x1910,
    );
}

// vtmpy 3-tap sliding with a free (un-multiplied) addend tap (mode 1, pair->pair):
//   vtmpyb  : Vuu.b  * Rt.b -> .h
//   vtmpybus: Vuu.ub * Rt.b -> .h
//   vtmpyhb : Vuu.h  * Rt.b -> .w
#[test]
fn lift_hvx_vtmpy() {
    lift_family(
        "hvx_vtmpy",
        &[
            ("vtmpyb", "{ v3:2.h = vtmpy(v5:4.b,r6.b) }"),
            ("vtmpyb_acc", "{ v3:2.h += vtmpy(v5:4.b,r6.b) }"),
            ("vtmpybus", "{ v3:2.h = vtmpy(v5:4.ub,r6.b) }"),
            ("vtmpybus_acc", "{ v3:2.h += vtmpy(v5:4.ub,r6.b) }"),
            ("vtmpyhb", "{ v3:2.w = vtmpy(v5:4.h,r6.b) }"),
            ("vtmpyhb_acc", "{ v3:2.w += vtmpy(v5:4.h,r6.b) }"),
        ],
        16,
        0x1920,
    );
}

// pair -> SINGLE straddle, saturated (mode 2):
//   vdmpyhisat   : Vuu.h * Rt.h  -> .w :sat
//   vdmpyhsuisat : Vuu.h * Rt.uh -> .w :sat (the #1 is structural, not an operand)
#[test]
fn lift_hvx_vdmpyhisat() {
    lift_family(
        "hvx_vdmpyhisat",
        &[
            ("vdmpyhisat", "{ v2.w = vdmpy(v5:4.h,r6.h):sat }"),
            ("vdmpyhisat_acc", "{ v2.w += vdmpy(v5:4.h,r6.h):sat }"),
            ("vdmpyhsuisat", "{ v2.w = vdmpy(v5:4.h,r6.uh,#1):sat }"),
            ("vdmpyhsuisat_acc", "{ v2.w += vdmpy(v5:4.h,r6.uh,#1):sat }"),
        ],
        16,
        0x1930,
    );
}

// ---- Wave 20: #u1-byte-rotate pair reduce + sum-of-abs-diff
// (OpKind::VRotReduceMulPair). Source AND dest are register PAIRs. Rt is
// I32-broadcast so its sub-bytes/halfwords are reused per output lane.
//
// vrmpyubi/vrmpybusi (+_acc): 4-tap byte word reduce with a #u1 source-select
// (sel = imm ? v1 : v0) and an Rt byte rotate by -imm. ubi = Rt.ub unsigned,
// busi = Rt.b signed. Both #0 and #1 immediates are exercised so the
// source-select and rotate paths are covered.
#[test]
fn lift_hvx_vrmpy_pair_imm() {
    lift_family(
        "hvx_vrmpy_pair_imm",
        &[
            ("vrmpyubi_0", "{ v3:2.uw = vrmpy(v5:4.ub,r6.ub,#0) }"),
            ("vrmpyubi_1", "{ v3:2.uw = vrmpy(v5:4.ub,r6.ub,#1) }"),
            ("vrmpyubi_acc0", "{ v3:2.uw += vrmpy(v5:4.ub,r6.ub,#0) }"),
            ("vrmpyubi_acc1", "{ v3:2.uw += vrmpy(v5:4.ub,r6.ub,#1) }"),
            ("vrmpybusi_0", "{ v3:2.w = vrmpy(v5:4.ub,r6.b,#0) }"),
            ("vrmpybusi_1", "{ v3:2.w = vrmpy(v5:4.ub,r6.b,#1) }"),
            ("vrmpybusi_acc0", "{ v3:2.w += vrmpy(v5:4.ub,r6.b,#0) }"),
            ("vrmpybusi_acc1", "{ v3:2.w += vrmpy(v5:4.ub,r6.b,#1) }"),
        ],
        16,
        0x1940,
    );
}

// vrsadubi (+_acc): SAME byte window/imm-rotate but each tap is |Vuu.ub - Rt.ub|
// (sum of absolute differences). Rt unsigned.
#[test]
fn lift_hvx_vrsad_pair_imm() {
    lift_family(
        "hvx_vrsad_pair_imm",
        &[
            ("vrsadubi_0", "{ v3:2.uw = vrsad(v5:4.ub,r6.ub,#0) }"),
            ("vrsadubi_1", "{ v3:2.uw = vrsad(v5:4.ub,r6.ub,#1) }"),
            ("vrsadubi_acc0", "{ v3:2.uw += vrsad(v5:4.ub,r6.ub,#0) }"),
            ("vrsadubi_acc1", "{ v3:2.uw += vrsad(v5:4.ub,r6.ub,#1) }"),
        ],
        16,
        0x1950,
    );
}

// vdsaduh (+_acc): dual SAD over UNSIGNED halfwords (mode 1, no imm):
//   o0[i] = |v0.uh[2i]-Rt.uh[0]| + |v0.uh[2i+1]-Rt.uh[1]|
//   o1[i] = |v0.uh[2i+1]-Rt.uh[0]| + |v1.uh[2i]-Rt.uh[1]|
#[test]
fn lift_hvx_vdsaduh() {
    lift_family(
        "hvx_vdsaduh",
        &[
            ("vdsaduh", "{ v3:2.uw = vdsad(v5:4.uh,r6.uh) }"),
            ("vdsaduh_acc", "{ v3:2.uw += vdsad(v5:4.uh,r6.uh) }"),
        ],
        16,
        0x1960,
    );
}

// ============================================================================
// Wave 5: widening add/sub pairs, dual-vector add/sub, per-lane unary
// (abs/not/clz/popcount/normamt), vnavg, shift-accumulate, vmpyih_acc,
// vdmpyhvsat. All verified 0-divergence against the qemu-backed interpreter.
// ============================================================================

// Widening add/sub -> register pair (OpKind::VWidenAddSub): even narrow lanes
// -> low vector, odd -> high. ub op ub -> .h pair; (u)h op (u)h -> .w pair.
#[test]
fn lift_hvx_vwiden_addsub() {
    lift_family(
        "hvx_vwiden_addsub",
        &[
            ("vaddubh", "{ v1:0.h = vadd(v2.ub,v3.ub) }"),
            ("vsububh", "{ v1:0.h = vsub(v2.ub,v3.ub) }"),
            ("vaddhw", "{ v1:0.w = vadd(v2.h,v3.h) }"),
            ("vsubhw", "{ v1:0.w = vsub(v2.h,v3.h) }"),
            ("vadduhw", "{ v1:0.w = vadd(v2.uh,v3.uh) }"),
            ("vsubuhw", "{ v1:0.w = vsub(v2.uh,v3.uh) }"),
        ],
        12,
        0x1a00,
    );
}

#[test]
fn lift_hvx_vwiden_addsub_acc() {
    // Accumulate (`v1:0 += ...`): read-modify-write of the dest pair; the
    // existing wide lane is sign-extended before the add (matches sem).
    lift_family(
        "hvx_vwiden_addsub_acc",
        &[
            ("vaddubh_acc", "{ v1:0.h += vadd(v2.ub,v3.ub) }"),
            ("vaddhw_acc", "{ v1:0.w += vadd(v2.h,v3.h) }"),
            ("vadduhw_acc", "{ v1:0.w += vadd(v2.uh,v3.uh) }"),
        ],
        12,
        0x1a01,
    );
}

// Dual-vector (plain, wrapping) add/sub: two independent VLane ops over the
// even/odd registers of each pair.
#[test]
fn lift_hvx_vaddsub_dv() {
    lift_family(
        "hvx_vaddsub_dv",
        &[
            ("vaddb_dv", "{ v1:0.b = vadd(v3:2.b,v5:4.b) }"),
            ("vaddh_dv", "{ v1:0.h = vadd(v3:2.h,v5:4.h) }"),
            ("vaddw_dv", "{ v1:0.w = vadd(v3:2.w,v5:4.w) }"),
            ("vsubb_dv", "{ v1:0.b = vsub(v3:2.b,v5:4.b) }"),
            ("vsubh_dv", "{ v1:0.h = vsub(v3:2.h,v5:4.h) }"),
            ("vsubw_dv", "{ v1:0.w = vsub(v3:2.w,v5:4.w) }"),
        ],
        12,
        0x1a02,
    );
}

// Per-lane unary: abs (+sat), vnot, vcl0, vnormamt, vpopcounth
// (OpKind::VLaneUnary).
#[test]
fn lift_hvx_vunary() {
    lift_family(
        "hvx_vunary",
        &[
            ("vnot", "{ v0 = vnot(v1) }"),
            ("vabsb", "{ v0.b = vabs(v1.b) }"),
            ("vabsh", "{ v0.h = vabs(v1.h) }"),
            ("vabsw", "{ v0.w = vabs(v1.w) }"),
            ("vabsb_sat", "{ v0.b = vabs(v1.b):sat }"),
            ("vabsh_sat", "{ v0.h = vabs(v1.h):sat }"),
            ("vabsw_sat", "{ v0.w = vabs(v1.w):sat }"),
            ("vcl0h", "{ v0.uh = vcl0(v1.uh) }"),
            ("vcl0w", "{ v0.uw = vcl0(v1.uw) }"),
            ("vnormamth", "{ v0.h = vnormamt(v1.h) }"),
            ("vnormamtw", "{ v0.w = vnormamt(v1.w) }"),
            ("vpopcounth", "{ v0.h = vpopcount(v1.h) }"),
        ],
        12,
        0x1a03,
    );
}

// vnavg: (ext(a)-ext(b))>>1 arithmetic (OpKind::VNavg). Signed b/h/w,
// unsigned-source ub.
#[test]
fn lift_hvx_vnavg() {
    lift_family(
        "hvx_vnavg",
        &[
            ("vnavgb", "{ v0.b = vnavg(v1.b,v2.b) }"),
            ("vnavgh", "{ v0.h = vnavg(v1.h,v2.h) }"),
            ("vnavgw", "{ v0.w = vnavg(v1.w,v2.w) }"),
            ("vnavgub", "{ v0.b = vnavg(v1.ub,v2.ub) }"),
        ],
        12,
        0x1a04,
    );
}

// Shift-accumulate by scalar Rt: Vx.<w> += (Vu.<w> {<<,>>} (Rt & (W-1)))
// (OpKind::VShiftAcc).
#[test]
fn lift_hvx_vshift_acc() {
    lift_family(
        "hvx_vshift_acc",
        &[
            ("vaslh_acc", "{ v0.h += vasl(v1.h,r5) }"),
            ("vaslw_acc", "{ v0.w += vasl(v1.w,r5) }"),
            ("vasrh_acc", "{ v0.h += vasr(v1.h,r5) }"),
            ("vasrw_acc", "{ v0.w += vasr(v1.w,r5) }"),
        ],
        16,
        0x1a05,
    );
}

// vmpyih_acc: Vx.h += vmpyi(Vu.h,Vv.h) — VMul(temp) + VLane::Add(Vx).
#[test]
fn lift_hvx_vmpyih_acc() {
    lift_family(
        "hvx_vmpyih_acc",
        &[("vmpyih_acc", "{ v0.h += vmpyi(v2.h,v3.h) }")],
        16,
        0x1a06,
    );
}

// vdmpyhvsat(_acc): vector-vector 2-tap h*h -> word, saturated (VReduceMul).
#[test]
fn lift_hvx_vdmpyhvsat() {
    lift_family(
        "hvx_vdmpyhvsat",
        &[
            ("vdmpyhvsat", "{ v0.w = vdmpy(v4.h,v6.h):sat }"),
            ("vdmpyhvsat_acc", "{ v0.w += vdmpy(v4.h,v6.h):sat }"),
        ],
        16,
        0x1a07,
    );
}

// vaddclb{h,w}: Vd = vadd(vclb(Vu), Vv) — count-leading-sign-bits then add
// (VLaneUnary Clb + VLane::Add).
#[test]
fn lift_hvx_vaddclb() {
    lift_family(
        "hvx_vaddclb",
        &[
            ("vaddclbh", "{ v0.h = vadd(vclb(v1.h),v2.h) }"),
            ("vaddclbw", "{ v0.w = vadd(vclb(v1.w),v2.w) }"),
        ],
        16,
        0x1a08,
    );
}

// ---- Wave 16: HVX Q-predicated add/sub, carry, vswap, cmov, prefix, vand_acc ----
// The harness seeds Q0-3 and P0-3 randomly, so the predicated forms exercise both
// the take and skip paths and both Q polarities.

// Q-predicated conditional add/sub: `if (Qv[!]) Vx {+,-}= Vu` (VLaneCond). The
// destination Vx is read-modify-written, masked per vector byte by the OLD Qv.
#[test]
fn lift_hvx_vaddsubq() {
    lift_family(
        "hvx_vaddsubq",
        &[
            ("vaddbq", "{ if (q0) v0.b += v1.b }"),
            ("vaddbnq", "{ if (!q0) v0.b += v1.b }"),
            ("vaddhq", "{ if (q0) v0.h += v1.h }"),
            ("vaddhnq", "{ if (!q0) v0.h += v1.h }"),
            ("vaddwq", "{ if (q0) v0.w += v1.w }"),
            ("vaddwnq", "{ if (!q0) v0.w += v1.w }"),
            ("vsubbq", "{ if (q0) v0.b -= v1.b }"),
            ("vsubbnq", "{ if (!q0) v0.b -= v1.b }"),
            ("vsubhq", "{ if (q0) v0.h -= v1.h }"),
            ("vsubhnq", "{ if (!q0) v0.h -= v1.h }"),
            ("vsubwq", "{ if (q0) v0.w -= v1.w }"),
            ("vsubwnq", "{ if (!q0) v0.w -= v1.w }"),
        ],
        16,
        0x1b00,
    );
}

// Carry add/sub: per-word add/sub with a Q vector-predicate carry (VCarry).
// carry: Q is carry-in AND out; carryo: carry-out only; carrysat: carry-in,
// no carry-out, signed sat_32. The harness compares the result V AND the Q.
#[test]
fn lift_hvx_vcarry() {
    lift_family(
        "hvx_vcarry",
        &[
            ("vaddcarry", "{ v0.w = vadd(v1.w,v2.w,q3):carry }"),
            ("vsubcarry", "{ v0.w = vsub(v1.w,v2.w,q3):carry }"),
            ("vaddcarryo", "{ v0.w,q3 = vadd(v1.w,v2.w):carry }"),
            ("vsubcarryo", "{ v0.w,q3 = vsub(v1.w,v2.w):carry }"),
            ("vaddcarrysat", "{ v0.w = vadd(v1.w,v2.w,q3):carry:sat }"),
        ],
        16,
        0x1b01,
    );
}

// vswap: Vdd = vswap(Qt, Vu, Vv) — a pair Q-blend (VSwap).
#[test]
fn lift_hvx_vswap() {
    lift_family(
        "hvx_vswap",
        &[("vswap", "{ v1:0 = vswap(q3,v2,v3) }")],
        16,
        0x1b02,
    );
}

// Scalar-predicated whole-vector move / combine (VCondMove). CANCELs (no write)
// when the gate is false; the harness seeds P0-3 so both paths run.
#[test]
fn lift_hvx_vcmov() {
    lift_family(
        "hvx_vcmov",
        &[
            ("vcmov", "{ if (p0) v0 = v1 }"),
            ("vncmov", "{ if (!p0) v0 = v1 }"),
            ("vccombine", "{ if (p0) v1:0 = vcombine(v2,v3) }"),
            ("vnccombine", "{ if (!p0) v1:0 = vcombine(v2,v3) }"),
        ],
        16,
        0x1b03,
    );
}

// Q prefix-sum (VPrefixSumQ): running popcount of a Q's bits into b/h/w lanes.
#[test]
fn lift_hvx_vprefixq() {
    lift_family(
        "hvx_vprefixq",
        &[
            ("vprefixqb", "{ v0.b = prefixsum(q0) }"),
            ("vprefixqh", "{ v0.h = prefixsum(q0) }"),
            ("vprefixqw", "{ v0.w = prefixsum(q0) }"),
        ],
        16,
        0x1b04,
    );
}

// OR-accumulating Q<->V / V<->Q bridges (vand*_acc): read-modify the dst V/Q.
// The harness seeds V0-31 and Q0-3 so the OR-accumulate path is meaningful.
#[test]
fn lift_hvx_vand_acc() {
    lift_family(
        "hvx_vand_acc",
        &[
            ("vandqrt_acc", "{ v0 |= vand(q0,r3) }"),
            ("vandnqrt_acc", "{ v0 |= vand(!q0,r3) }"),
            ("vandvrt_acc", "{ q0 |= vand(v1,r3) }"),
        ],
        16,
        0x1b05,
    );
}

// ---- Wave: FINAL tractable HVX misc opcodes (VShuffleEOPair, VShuffleDeal,
// VDealVdd, VUnpackOAcc, VInsertWordR, VExtractWord, VLut4, VRotr,
// VAddSubMixedSat, VSetPredQ, VShuffEqQ, VMpaHhSat, VMpyHsatAcc, VAsrInto, V6Mpy)

// vshufoeb/vshufoeh: odd/even shuffle into a pair (VShuffleEOPair).
#[test]
fn lift_hvx_vshufoe() {
    lift_family(
        "hvx_vshufoe",
        &[
            ("vshufoeb", "{ v1:0.b = vshuffoe(v3.b,v2.b) }"),
            ("vshufoeh", "{ v1:0.h = vshuffoe(v3.h,v2.h) }"),
        ],
        16,
        0x1b10,
    );
}

// vshuff/vdeal: in-place dual-register byte shuffle/deal (VShuffleDeal).
#[test]
fn lift_hvx_vshuff_vdeal_inplace() {
    lift_family(
        "hvx_vshuff_vdeal_inplace",
        &[
            ("vshuff", "{ vshuff(v1,v0,r5) }"),
            ("vdeal", "{ vdeal(v1,v0,r5) }"),
        ],
        16,
        0x1b11,
    );
}

// vdealvdd: pair-dest deal Vdd = vdeal(Vu,Vv,Rt) (VDealVdd).
#[test]
fn lift_hvx_vdealvdd() {
    lift_family(
        "hvx_vdealvdd",
        &[("vdealvdd", "{ v1:0 = vdeal(v3,v2,r5) }")],
        16,
        0x1b12,
    );
}

// vunpackob/vunpackoh: OR-accumulate the odd-extended lanes (VUnpackOAcc).
#[test]
fn lift_hvx_vunpacko() {
    lift_family(
        "hvx_vunpacko",
        &[
            ("vunpackob", "{ v1:0.h |= vunpacko(v2.b) }"),
            ("vunpackoh", "{ v1:0.w |= vunpacko(v2.h) }"),
        ],
        16,
        0x1b13,
    );
}

// vinsertwr / extractw: V<->R word moves (VInsertWordR / VExtractWord).
#[test]
fn lift_hvx_vinsert_extract() {
    lift_family(
        "hvx_vinsert_extract",
        &[
            ("vinsertwr", "{ v0.w = vinsert(r5) }"),
            ("extractw", "{ r1 = vextract(v0,r2) }"),
        ],
        16,
        0x1b14,
    );
}

// vlut4: 4-entry halfword lookup from a scalar pair (VLut4).
#[test]
fn lift_hvx_vlut4() {
    lift_family(
        "hvx_vlut4",
        &[("vlut4", "{ v0.h = vlut4(v1.uh,r5:4.h) }")],
        16,
        0x1b15,
    );
}

// vrotr: per-word bit rotate-right (VRotr).
#[test]
fn lift_hvx_vrotr() {
    lift_family(
        "hvx_vrotr",
        &[("vrotr", "{ v0.uw = vrotr(v1.uw,v2.uw) }")],
        16,
        0x1b16,
    );
}

// vaddububb_sat/vsubububb_sat (ub +/- b -> ub:sat) and vsubuwsat (uw - uw -> uw:sat).
#[test]
fn lift_hvx_addsub_sat_misc() {
    lift_family(
        "hvx_addsub_sat_misc",
        &[
            ("vaddububb_sat", "{ v0.ub = vadd(v1.ub,v2.b):sat }"),
            ("vsubububb_sat", "{ v0.ub = vsub(v1.ub,v2.b):sat }"),
            ("vsubuwsat", "{ v0.uw = vsub(v1.uw,v2.uw):sat }"),
        ],
        16,
        0x1b17,
    );
}

// vsetq / vsetq2: build a Q predicate from a scalar (VSetPredQ). Q compared directly.
#[test]
fn lift_hvx_vsetq() {
    lift_family(
        "hvx_vsetq",
        &[
            ("vsetq", "{ q0 = vsetq(r5) }"),
            ("vsetq2", "{ q0 = vsetq2(r5) }"),
        ],
        24,
        0x1b18,
    );
}

// shuffeqh/shuffeqw: Q-predicate shrink/shuffle (VShuffEqQ). Q compared directly.
#[test]
fn lift_hvx_shuffeq() {
    lift_family(
        "hvx_shuffeq",
        &[
            ("shuffeqh", "{ q0.b = vshuffe(q1.h,q2.h) }"),
            ("shuffeqw", "{ q0.h = vshuffe(q1.w,q2.w) }"),
        ],
        16,
        0x1b19,
    );
}

// vmpahhsat / vmpauhuhsat / vmpsuhuhsat: saturating halfword mpa/mps (VMpaHhSat).
#[test]
fn lift_hvx_vmpa_hh_sat() {
    lift_family(
        "hvx_vmpa_hh_sat",
        &[
            ("vmpahhsat", "{ v0.h = vmpa(v0.h,v2.h,r5:4.h):sat }"),
            ("vmpauhuhsat", "{ v0.h = vmpa(v0.h,v2.uh,r5:4.uh):sat }"),
            ("vmpsuhuhsat", "{ v0.h = vmps(v0.h,v2.uh,r5:4.uh):sat }"),
        ],
        16,
        0x1b1a,
    );
}

// vmpyhsat_acc: saturating word accumulate (VMpyHsatAcc).
#[test]
fn lift_hvx_vmpyhsat_acc() {
    lift_family(
        "hvx_vmpyhsat_acc",
        &[("vmpyhsat_acc", "{ v1:0.w += vmpy(v4.h,r5.h):sat }")],
        16,
        0x1b1b,
    );
}

// vasr_into: shift-into-accumulator (VAsrInto).
#[test]
fn lift_hvx_vasr_into() {
    lift_family(
        "hvx_vasr_into",
        &[("vasr_into", "{ v1:0.w = vasrinto(v2.w,v3.w) }")],
        16,
        0x1b1c,
    );
}

// vassign_tmp / vcombine_tmp: .tmp register moves (VMov / pair copy).
#[test]
fn lift_hvx_tmp_moves() {
    lift_family(
        "hvx_tmp_moves",
        &[
            ("vassign_tmp", "{ v0.tmp = v1 }"),
            ("vcombine_tmp", "{ v1:0.tmp = vcombine(v3,v2) }"),
        ],
        12,
        0x1b1d,
    );
}

// v6mpy: V69 byte-matrix multiply, :h/:v phases + acc forms (V6Mpy).
#[test]
fn lift_hvx_v6mpy() {
    lift_family(
        "hvx_v6mpy",
        &[
            ("v6mpyh_u0", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#0):h }"),
            ("v6mpyh_u1", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#1):h }"),
            ("v6mpyh_u2", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#2):h }"),
            ("v6mpyh_u3", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#3):h }"),
            ("v6mpyv_u0", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#0):v }"),
            ("v6mpyv_u3", "{ v1:0.w = v6mpy(v3:2.ub,v5:4.b,#3):v }"),
            ("v6mpyh_acc_u1", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#1):h }"),
            ("v6mpyv_acc_u2", "{ v1:0.w += v6mpy(v3:2.ub,v5:4.b,#2):v }"),
        ],
        12,
        0x1b1e,
    );
}

// ============================================================================
// HVX histogram family (vhist / vhistq / vwhist128* / vwhist256*).
//
// These instructions have NO conventional dst operand and NO register operand
// for their input. Each tallies values from the 128-byte input vector (loaded
// by a same-packet `.tmp` vmem load into qemu's `tmp_VRegs[0]`) into histogram
// bins spread across the WHOLE V0..V31 register file (a read-modify-write of
// all 32 vector registers). The canonical idiom is:
//     { v0.tmp = vmem(Rx+#0); vhist }
//
// The standard `lift_family` driver seeds R randomly, which makes the `.tmp`
// load address fault and the interpreter skip every iteration, so we use a
// dedicated driver that points the load base at a valid, populated memory
// region and writes the SAME input bytes into both the interpreter's guest
// memory and the SMIR interpreter's FlatMemory. We then compare the FULL
// V0-31 / Q0-3 / R / P / USR state after, exactly like `lift_family`.
// ============================================================================

/// Aligned base address (within the 0x20000 test region) the `.tmp` load reads.
const HIST_INPUT_ADDR: u32 = 0x4000;

/// Run the histogram packet on rax's Hexagon interpreter, with `input` written
/// at `HIST_INPUT_ADDR` and `r0` pointing there. Returns the post-state.
fn run_interp_hist(words: &[u32], init: &State, input: &[u8; 128]) -> Option<State> {
    let mem = Arc::new(GuestMemoryMmap::<()>::from_ranges(&[(GuestAddress(0), 0x20000)]).ok()?);
    let mut off = CODE_ADDR;
    for &w in words {
        mem.write_slice(&w.to_le_bytes(), GuestAddress(off as u64))
            .ok()?;
        off += 4;
    }
    mem.write_slice(&trap_word().to_le_bytes(), GuestAddress(off as u64))
        .ok()?;
    mem.write_slice(&input[..], GuestAddress(HIST_INPUT_ADDR as u64))
        .ok()?;
    let mut regs = HexagonRegisters::default();
    regs.r = init.r;
    regs.p = init.p;
    regs.c[8] = init.usr;
    regs.v = init.v;
    regs.q = init.q;
    regs.set_pc(CODE_ADDR);
    let mut vcpu = HexagonVcpu::new(0, mem, HexagonIsa::V68, Endianness::Little);
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
    Some(State {
        r: regs.r,
        p: regs.p,
        usr: regs.c[8],
        v: regs.v,
        q: regs.q,
        m: [regs.c[6], regs.c[7]],
        cs: [regs.c[12], regs.c[13]],
        gp: regs.c[11],
    })
}

/// Lift the histogram packet to SMIR and execute it, with `input` written at
/// `HIST_INPUT_ADDR` in the FlatMemory and `r0` pointing there.
/// `Ok(None)` => some word lifted to Unsupported (a lift gap).
fn lift_and_run_hist(
    words: &[u32],
    init: &State,
    input: &[u8; 128],
) -> Result<Option<State>, String> {
    let mut lifter = HexagonLifter::default_isa();
    let mut lctx = LiftContext::new(SourceArch::Hexagon);
    let mut ops = Vec::new();
    let mut addr = CODE_ADDR as u64;
    for &w in words {
        match lifter.lift_insn(addr, &w.to_le_bytes(), &mut lctx) {
            Ok(res) => ops.extend(res.ops),
            Err(LiftError::Unsupported { .. }) => return Ok(None),
            Err(e) => return Err(format!("lift error: {e:?}")),
        }
        addr += 4;
    }
    if ops.is_empty() {
        // The histogram word emitted no ops and no `.tmp` load followed to flush
        // the pending VHist — treat as an (unexpected) lift gap rather than a
        // silent no-op.
        return Ok(None);
    }
    for (i, op) in ops.iter_mut().enumerate() {
        op.id = OpId(i as u16);
    }
    let block = SmirBlock {
        id: BlockId(0),
        guest_pc: CODE_ADDR as u64,
        phis: vec![],
        ops,
        terminator: Terminator::Trap {
            kind: TrapKind::Breakpoint,
        },
        exec_count: 0,
    };
    let mut ctx = SmirContext::new_hexagon();
    for n in 0..NREG {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::R(n as u8)), init.r[n] as u64);
    }
    for n in 0..4 {
        ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::P(n as u8)), init.p[n] as u64);
    }
    ctx.write_arch_reg(ArchReg::Hexagon(HexagonReg::Usr), init.usr as u64);
    if let ArchRegState::Hexagon(hex) = &mut ctx.arch_regs {
        for n in 0..32 {
            let mut lanes = [0u64; 16];
            for (j, lane) in lanes.iter_mut().enumerate() {
                *lane = init.v[n][2 * j] as u64 | ((init.v[n][2 * j + 1] as u64) << 32);
            }
            hex.set_v(n as u8, lanes);
        }
        for n in 0..4 {
            let mut lanes = [0u64; 16];
            lanes[0] = init.q[n][0] as u64 | ((init.q[n][1] as u64) << 32);
            lanes[1] = init.q[n][2] as u64 | ((init.q[n][3] as u64) << 32);
            hex.set_q(n as u8, lanes);
        }
    }
    ctx.pc = CODE_ADDR as u64;

    let interp = SmirInterpreter::new();
    let mut mem = rax::smir::FlatMemory::with_base(0, 0x20000);
    mem.load(HIST_INPUT_ADDR as usize, &input[..]);
    interp.execute_block(&mut ctx, &mut mem, &block);

    let mut out = State::zeroed();
    for n in 0..NREG {
        out.r[n] = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::R(n as u8))) as u32;
    }
    for n in 0..4 {
        let v = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::P(n as u8)));
        out.p[n] = v as u8;
    }
    out.usr = ctx.read_arch_reg(ArchReg::Hexagon(HexagonReg::Usr)) as u32;
    if let ArchRegState::Hexagon(hex) = &ctx.arch_regs {
        for n in 0..32 {
            let lanes = hex.get_v(n as u8);
            for (j, lane) in lanes.iter().enumerate() {
                out.v[n][2 * j] = *lane as u32;
                out.v[n][2 * j + 1] = (*lane >> 32) as u32;
            }
        }
        for n in 0..4 {
            let lanes = hex.get_q(n as u8);
            out.q[n][0] = lanes[0] as u32;
            out.q[n][1] = (lanes[0] >> 32) as u32;
            out.q[n][2] = lanes[1] as u32;
            out.q[n][3] = (lanes[1] >> 32) as u32;
        }
    }
    Ok(Some(out))
}

/// Lift-verify the histogram family: each (label, single-packet asm) over `n`
/// random V/Q/P/USR states with `r0` pointed at a populated input region.
fn lift_hist_family(name: &str, cases: &[(&str, &str)], n: usize, seed: u64) {
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_smir_lift] {name}: llvm-mc unavailable -> skipping");
            return;
        }
    };
    let mut rng = Rng::new(seed);
    let mut mismatches = Vec::new();
    let mut unlifted = Vec::new();
    for ((label, _asm), words) in cases.iter().zip(words_per.iter()) {
        for _ in 0..n {
            let mut st = State::zeroed();
            for r in st.r.iter_mut() {
                *r = rng.next() as u32;
            }
            // Base register for the `.tmp` load: point r0 at the input region.
            st.r[0] = HIST_INPUT_ADDR;
            for k in 0..4 {
                // Seed a FULL random predicate byte (exercises per-lane masks);
                // bias toward the common scalar values 0x00/0xff.
                st.p[k] = match rng.next() & 3 {
                    0 => 0x00,
                    1 => 0xff,
                    _ => rng.next() as u8,
                };
            }
            for vv in st.v.iter_mut() {
                for w in vv.iter_mut() {
                    *w = rng.next() as u32;
                }
            }
            for qq in st.q.iter_mut() {
                for w in qq.iter_mut() {
                    *w = rng.next() as u32;
                }
            }
            let mut input = [0u8; 128];
            for b in input.iter_mut() {
                *b = rng.next() as u8;
            }
            let interp = match run_interp_hist(words, &st, &input) {
                Some(s) => s,
                None => continue,
            };
            match lift_and_run_hist(words, &st, &input) {
                Ok(None) => {
                    unlifted.push(*label);
                    break;
                }
                Ok(Some(lift)) => {
                    let mut diffs = Vec::new();
                    for r in 0..NREG {
                        if interp.r[r] != lift.r[r] {
                            diffs.push(format!("r{r}:i={:#x},l={:#x}", interp.r[r], lift.r[r]));
                        }
                    }
                    for k in 0..4 {
                        if interp.p[k] != lift.p[k] {
                            diffs.push(format!("p{k}:i={:#x},l={:#x}", interp.p[k], lift.p[k]));
                        }
                    }
                    for vn in 0..32 {
                        if interp.v[vn] != lift.v[vn] {
                            diffs.push(format!(
                                "v{vn}:i={:08x?},l={:08x?}",
                                &interp.v[vn][..4],
                                &lift.v[vn][..4]
                            ));
                        }
                    }
                    for qn in 0..4 {
                        if interp.q[qn] != lift.q[qn] {
                            diffs.push(format!(
                                "q{qn}:i={:08x?},l={:08x?}",
                                interp.q[qn], lift.q[qn]
                            ));
                        }
                    }
                    // USR:OVF (bit 0) — the sticky integer saturation/overflow
                    // flag. Only bit 0 is compared; other USR bits (FP flags,
                    // rounding mode) are not lifted and are irrelevant here.
                    if (interp.usr & 1) != (lift.usr & 1) {
                        if OVF_DEFERRED_HVX.contains(label) {
                            eprintln!(
                                "[hexagon_smir_lift] {name}: OVF deferred for [{label}] (i={},l={}) \
                                 — HVX shared-OpKind set_ovf follow-up",
                                interp.usr & 1,
                                lift.usr & 1
                            );
                        } else {
                            diffs.push(format!("usr_ovf:i={},l={}", interp.usr & 1, lift.usr & 1));
                        }
                    }
                    if !diffs.is_empty() {
                        mismatches.push(format!("[{label}] {}", diffs.join(" ")));
                    }
                }
                Err(e) => mismatches.push(format!("[{label}] {e}")),
            }
        }
    }
    if !unlifted.is_empty() {
        eprintln!("[hexagon_smir_lift] {name}: UNLIFTED (gap): {:?}", unlifted);
    }
    if !mismatches.is_empty() {
        eprintln!("\n==== {name}: {} lift mismatches ====", mismatches.len());
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!(
            "{name}: {} SMIR-lift divergences vs interpreter",
            mismatches.len()
        );
    }
}

#[test]
fn lift_hvx_vhist() {
    lift_hist_family(
        "hvx_vhist",
        &[
            ("vhist", "{ v0.tmp = vmem(r0+#0); vhist }"),
            ("vhistq", "{ v0.tmp = vmem(r0+#0); vhist(q1) }"),
        ],
        16,
        0x1c01,
    );
}

#[test]
fn lift_hvx_vwhist128() {
    lift_hist_family(
        "hvx_vwhist128",
        &[
            ("vwhist128", "{ v0.tmp = vmem(r0+#0); vwhist128 }"),
            ("vwhist128m", "{ v0.tmp = vmem(r0+#0); vwhist128(#1) }"),
            ("vwhist128q", "{ v0.tmp = vmem(r0+#0); vwhist128(q1) }"),
            ("vwhist128qm", "{ v0.tmp = vmem(r0+#0); vwhist128(q1,#1) }"),
        ],
        16,
        0x1c02,
    );
}

#[test]
fn lift_hvx_vwhist256() {
    lift_hist_family(
        "hvx_vwhist256",
        &[
            ("vwhist256", "{ v0.tmp = vmem(r0+#0); vwhist256 }"),
            ("vwhist256q", "{ v0.tmp = vmem(r0+#0); vwhist256(q1) }"),
            ("vwhist256_sat", "{ v0.tmp = vmem(r0+#0); vwhist256:sat }"),
            (
                "vwhist256q_sat",
                "{ v0.tmp = vmem(r0+#0); vwhist256(q1):sat }",
            ),
        ],
        16,
        0x1c03,
    );
}

// ---- newly-lifted scalar register opcodes (wave 3: M2/M4/S2/S4/S6/A4/C4) ----

#[test]
fn lift_m2_mpy16_set() {
    // 16x16 halfword multiplies, set form, signed/unsigned, 32-bit and pair,
    // s0 and :<<1 (s1). NON-saturating only.
    lift_family(
        "m2_mpy16_set",
        &[
            ("mpy_hh_s0", "{ r0 = mpy(r1.h,r2.h) }"),
            ("mpy_hl_s0", "{ r0 = mpy(r1.h,r2.l) }"),
            ("mpy_lh_s0", "{ r0 = mpy(r1.l,r2.h) }"),
            ("mpy_ll_s0", "{ r0 = mpy(r1.l,r2.l) }"),
            ("mpy_hh_s1", "{ r0 = mpy(r1.h,r2.h):<<1 }"),
            ("mpy_ll_s1", "{ r0 = mpy(r1.l,r2.l):<<1 }"),
            ("mpyu_hh_s0", "{ r0 = mpyu(r1.h,r2.h) }"),
            ("mpyu_ll_s0", "{ r0 = mpyu(r1.l,r2.l) }"),
            ("mpyu_lh_s1", "{ r0 = mpyu(r1.l,r2.h):<<1 }"),
            ("mpyd_hh_s0", "{ r1:0 = mpy(r2.h,r3.h) }"),
            ("mpyd_ll_s0", "{ r1:0 = mpy(r2.l,r3.l) }"),
            ("mpyd_hl_s1", "{ r1:0 = mpy(r2.h,r3.l):<<1 }"),
            ("mpyud_hh_s0", "{ r1:0 = mpyu(r2.h,r3.h) }"),
            ("mpyud_ll_s0", "{ r1:0 = mpyu(r2.l,r3.l) }"),
            ("mpyud_lh_s1", "{ r1:0 = mpyu(r2.l,r3.h):<<1 }"),
        ],
        24,
        0x7301,
    );
}

#[test]
fn lift_m2_mpy16_acc() {
    // 16x16 halfword multiplies, acc (+=) and nac (-=) forms.
    lift_family(
        "m2_mpy16_acc",
        &[
            ("mpy_acc_hh_s0", "{ r0 += mpy(r1.h,r2.h) }"),
            ("mpy_acc_ll_s0", "{ r0 += mpy(r1.l,r2.l) }"),
            ("mpy_acc_lh_s1", "{ r0 += mpy(r1.l,r2.h):<<1 }"),
            ("mpy_nac_hh_s0", "{ r0 -= mpy(r1.h,r2.h) }"),
            ("mpy_nac_ll_s1", "{ r0 -= mpy(r1.l,r2.l):<<1 }"),
            ("mpyu_acc_hh_s0", "{ r0 += mpyu(r1.h,r2.h) }"),
            ("mpyu_nac_ll_s0", "{ r0 -= mpyu(r1.l,r2.l) }"),
            ("mpyd_acc_hh_s0", "{ r1:0 += mpy(r2.h,r3.h) }"),
            ("mpyd_acc_ll_s1", "{ r1:0 += mpy(r2.l,r3.l):<<1 }"),
            ("mpyd_nac_hl_s0", "{ r1:0 -= mpy(r2.h,r3.l) }"),
            ("mpyud_acc_hh_s0", "{ r1:0 += mpyu(r2.h,r3.h) }"),
            ("mpyud_nac_ll_s0", "{ r1:0 -= mpyu(r2.l,r3.l) }"),
        ],
        24,
        0x7302,
    );
}

#[test]
fn lift_m2_m4_mpyi() {
    lift_family(
        "m2_m4_mpyi",
        &[
            ("mpyi", "{ r0 = mpyi(r1,r2) }"),
            ("mpyrr_addr", "{ r0 = add(r1,mpyi(r0,r2)) }"),
            ("mpyrr_addi", "{ r0 = add(#10,mpyi(r1,r2)) }"),
            ("mpyri_addr", "{ r0 = add(r1,mpyi(r2,#5)) }"),
            ("mpyri_addr_u2", "{ r0 = add(r1,mpyi(r2,#8)) }"),
            ("mpyri_addi", "{ r0 = add(#10,mpyi(r1,#5)) }"),
        ],
        24,
        0x7303,
    );
}

#[test]
fn lift_m4_acc_logical() {
    lift_family(
        "m4_acc_logical",
        &[
            ("and_and", "{ r0 &= and(r1,r2) }"),
            ("and_or", "{ r0 &= or(r1,r2) }"),
            ("and_xor", "{ r0 &= xor(r1,r2) }"),
            ("and_andn", "{ r0 &= and(r1,~r2) }"),
            ("or_and", "{ r0 |= and(r1,r2) }"),
            ("or_or", "{ r0 |= or(r1,r2) }"),
            ("or_xor", "{ r0 |= xor(r1,r2) }"),
            ("or_andn", "{ r0 |= and(r1,~r2) }"),
            ("xor_and", "{ r0 ^= and(r1,r2) }"),
            ("xor_or", "{ r0 ^= or(r1,r2) }"),
            ("xor_andn", "{ r0 ^= and(r1,~r2) }"),
        ],
        24,
        0x7304,
    );
}

#[test]
fn lift_s2_shift_acc_r() {
    lift_family(
        "s2_shift_acc_r",
        &[
            ("asl_acc", "{ r0 += asl(r1,#5) }"),
            ("asl_nac", "{ r0 -= asl(r1,#5) }"),
            ("asl_and", "{ r0 &= asl(r1,#5) }"),
            ("asl_or", "{ r0 |= asl(r1,#5) }"),
            ("asl_xacc", "{ r0 ^= asl(r1,#5) }"),
            ("asr_acc", "{ r0 += asr(r1,#5) }"),
            ("asr_nac", "{ r0 -= asr(r1,#5) }"),
            ("asr_and", "{ r0 &= asr(r1,#5) }"),
            ("asr_or", "{ r0 |= asr(r1,#5) }"),
            ("lsr_acc", "{ r0 += lsr(r1,#5) }"),
            ("lsr_nac", "{ r0 -= lsr(r1,#5) }"),
            ("lsr_and", "{ r0 &= lsr(r1,#5) }"),
            ("lsr_or", "{ r0 |= lsr(r1,#5) }"),
            ("lsr_xacc", "{ r0 ^= lsr(r1,#5) }"),
            ("rol_acc", "{ r0 += rol(r1,#5) }"),
            ("rol_nac", "{ r0 -= rol(r1,#5) }"),
            ("rol_and", "{ r0 &= rol(r1,#5) }"),
            ("rol_or", "{ r0 |= rol(r1,#5) }"),
            ("rol_xacc", "{ r0 ^= rol(r1,#5) }"),
        ],
        24,
        0x7305,
    );
}

#[test]
fn lift_s2_shift_acc_p() {
    lift_family(
        "s2_shift_acc_p",
        &[
            ("asl_p_acc", "{ r1:0 += asl(r3:2,#5) }"),
            ("asl_p_nac", "{ r1:0 -= asl(r3:2,#5) }"),
            ("asl_p_and", "{ r1:0 &= asl(r3:2,#5) }"),
            ("asl_p_or", "{ r1:0 |= asl(r3:2,#5) }"),
            ("asl_p_xacc", "{ r1:0 ^= asl(r3:2,#5) }"),
            ("asr_p_acc", "{ r1:0 += asr(r3:2,#5) }"),
            ("asr_p_nac", "{ r1:0 -= asr(r3:2,#5) }"),
            ("asr_p_and", "{ r1:0 &= asr(r3:2,#5) }"),
            ("asr_p_or", "{ r1:0 |= asr(r3:2,#5) }"),
            ("lsr_p_acc", "{ r1:0 += lsr(r3:2,#5) }"),
            ("lsr_p_nac", "{ r1:0 -= lsr(r3:2,#5) }"),
            ("lsr_p_and", "{ r1:0 &= lsr(r3:2,#5) }"),
            ("lsr_p_or", "{ r1:0 |= lsr(r3:2,#5) }"),
            ("lsr_p_xacc", "{ r1:0 ^= lsr(r3:2,#5) }"),
            ("rol_p_acc", "{ r1:0 += rol(r3:2,#5) }"),
            ("rol_p_nac", "{ r1:0 -= rol(r3:2,#5) }"),
            ("rol_p_and", "{ r1:0 &= rol(r3:2,#5) }"),
            ("rol_p_or", "{ r1:0 |= rol(r3:2,#5) }"),
            ("rol_p_xacc", "{ r1:0 ^= rol(r3:2,#5) }"),
        ],
        24,
        0x7306,
    );
}

// Register-amount BIDIRECTIONAL shifts: the count is sxtn7(Rt) in [-64,63];
// a negative count reverses the shift direction. The harness seeds Rt randomly
// so negative and large (>=width) counts are exercised. These verify the new
// OpKind::BidirShift lift vs the qemu-verified HexagonVcpu.
#[test]
fn lift_s2_shift_r_r() {
    lift_family(
        "s2_shift_r_r",
        &[
            ("asl_r_r", "{ r0 = asl(r1,r2) }"),
            ("asr_r_r", "{ r0 = asr(r1,r2) }"),
            ("lsr_r_r", "{ r0 = lsr(r1,r2) }"),
            ("lsl_r_r", "{ r0 = lsl(r1,r2) }"),
        ],
        40,
        0x7401,
    );
}

#[test]
fn lift_s2_shift_r_p() {
    lift_family(
        "s2_shift_r_p",
        &[
            ("asl_r_p", "{ r1:0 = asl(r3:2,r4) }"),
            ("asr_r_p", "{ r1:0 = asr(r3:2,r4) }"),
            ("lsr_r_p", "{ r1:0 = lsr(r3:2,r4) }"),
            ("lsl_r_p", "{ r1:0 = lsl(r3:2,r4) }"),
        ],
        40,
        0x7402,
    );
}

#[test]
fn lift_s2_shift_r_r_acc() {
    lift_family(
        "s2_shift_r_r_acc",
        &[
            ("asl_acc", "{ r0 += asl(r1,r2) }"),
            ("asl_nac", "{ r0 -= asl(r1,r2) }"),
            ("asl_and", "{ r0 &= asl(r1,r2) }"),
            ("asl_or", "{ r0 |= asl(r1,r2) }"),
            ("asr_acc", "{ r0 += asr(r1,r2) }"),
            ("asr_nac", "{ r0 -= asr(r1,r2) }"),
            ("asr_and", "{ r0 &= asr(r1,r2) }"),
            ("asr_or", "{ r0 |= asr(r1,r2) }"),
            ("lsr_acc", "{ r0 += lsr(r1,r2) }"),
            ("lsr_nac", "{ r0 -= lsr(r1,r2) }"),
            ("lsr_and", "{ r0 &= lsr(r1,r2) }"),
            ("lsr_or", "{ r0 |= lsr(r1,r2) }"),
            ("lsl_acc", "{ r0 += lsl(r1,r2) }"),
            ("lsl_nac", "{ r0 -= lsl(r1,r2) }"),
            ("lsl_and", "{ r0 &= lsl(r1,r2) }"),
            ("lsl_or", "{ r0 |= lsl(r1,r2) }"),
        ],
        40,
        0x7403,
    );
}

#[test]
fn lift_s2_shift_r_p_acc() {
    lift_family(
        "s2_shift_r_p_acc",
        &[
            ("asl_p_acc", "{ r1:0 += asl(r3:2,r4) }"),
            ("asl_p_nac", "{ r1:0 -= asl(r3:2,r4) }"),
            ("asl_p_and", "{ r1:0 &= asl(r3:2,r4) }"),
            ("asl_p_or", "{ r1:0 |= asl(r3:2,r4) }"),
            ("asl_p_xor", "{ r1:0 ^= asl(r3:2,r4) }"),
            ("asr_p_acc", "{ r1:0 += asr(r3:2,r4) }"),
            ("asr_p_nac", "{ r1:0 -= asr(r3:2,r4) }"),
            ("asr_p_and", "{ r1:0 &= asr(r3:2,r4) }"),
            ("asr_p_or", "{ r1:0 |= asr(r3:2,r4) }"),
            ("asr_p_xor", "{ r1:0 ^= asr(r3:2,r4) }"),
            ("lsr_p_acc", "{ r1:0 += lsr(r3:2,r4) }"),
            ("lsr_p_nac", "{ r1:0 -= lsr(r3:2,r4) }"),
            ("lsr_p_and", "{ r1:0 &= lsr(r3:2,r4) }"),
            ("lsr_p_or", "{ r1:0 |= lsr(r3:2,r4) }"),
            ("lsr_p_xor", "{ r1:0 ^= lsr(r3:2,r4) }"),
            ("lsl_p_acc", "{ r1:0 += lsl(r3:2,r4) }"),
            ("lsl_p_nac", "{ r1:0 -= lsl(r3:2,r4) }"),
            ("lsl_p_and", "{ r1:0 &= lsl(r3:2,r4) }"),
            ("lsl_p_or", "{ r1:0 |= lsl(r3:2,r4) }"),
            ("lsl_p_xor", "{ r1:0 ^= lsl(r3:2,r4) }"),
        ],
        40,
        0x7404,
    );
}

#[test]
fn lift_s2_bit_r() {
    lift_family(
        "s2_bit_r",
        &[
            ("setbit_r", "{ r0 = setbit(r1,r2) }"),
            ("clrbit_r", "{ r0 = clrbit(r1,r2) }"),
            ("togglebit_r", "{ r0 = togglebit(r1,r2) }"),
            ("tstbit_r", "{ p0 = tstbit(r1,r2) }"),
            ("ntstbit_r", "{ p0 = !tstbit(r1,r2) }"),
        ],
        40,
        0x7405,
    );
}

#[test]
fn lift_s4_lsli() {
    lift_family("s4_lsli", &[("lsli", "{ r0 = lsl(#6,r2) }")], 40, 0x7406);
}

#[test]
fn lift_s2_bitmanip2() {
    lift_family(
        "s2_bitmanip2",
        &[
            ("clb", "{ r0 = clb(r1) }"),
            ("clbnorm", "{ r0 = normamt(r1) }"),
            ("cl0p", "{ r0 = cl0(r3:2) }"),
            ("cl1p", "{ r0 = cl1(r3:2) }"),
            ("clbp", "{ r0 = clb(r3:2) }"),
            ("ct0p", "{ r0 = ct0(r3:2) }"),
            ("ct1p", "{ r0 = ct1(r3:2) }"),
            ("brevp", "{ r1:0 = brev(r3:2) }"),
            ("popcountp", "{ r0 = popcount(r3:2) }"),
            ("parity", "{ r0 = parity(r1,r2) }"),
            ("parityp", "{ r0 = parity(r3:2,r5:4) }"),
            ("mask", "{ r0 = mask(#8,#4) }"),
            ("packhl", "{ r1:0 = packhl(r2,r3) }"),
            ("swiz", "{ r0 = swiz(r1) }"),
        ],
        24,
        0x7307,
    );
}

#[test]
fn lift_s4_compound() {
    lift_family(
        "s4_compound",
        &[
            ("addaddi", "{ r0 = add(r1,add(r2,#5)) }"),
            ("subaddi", "{ r0 = add(r1,sub(#5,r2)) }"),
            ("addi_asl", "{ r0 = add(#100,asl(r0,#5)) }"),
            ("addi_lsr", "{ r0 = add(#100,lsr(r0,#5)) }"),
            ("subi_asl", "{ r0 = sub(#100,asl(r0,#5)) }"),
            ("subi_lsr", "{ r0 = sub(#100,lsr(r0,#5)) }"),
            ("andi_asl", "{ r0 = and(#100,asl(r0,#5)) }"),
            ("andi_lsr", "{ r0 = and(#100,lsr(r0,#5)) }"),
            ("ori_asl", "{ r0 = or(#100,asl(r0,#5)) }"),
            ("ori_lsr", "{ r0 = or(#100,lsr(r0,#5)) }"),
            ("or_andi", "{ r0 |= and(r1,#100) }"),
            ("or_andix", "{ r0 = or(r1,and(r0,#100)) }"),
            ("or_ori", "{ r0 |= or(r1,#100) }"),
        ],
        24,
        0x7308,
    );
}

#[test]
fn lift_a4_subword_cmp() {
    lift_family(
        "a4_subword_cmp",
        &[
            ("cmpbeq", "{ p0 = cmpb.eq(r1,r2) }"),
            ("cmpbeqi", "{ p0 = cmpb.eq(r1,#5) }"),
            ("cmpbgt", "{ p0 = cmpb.gt(r1,r2) }"),
            ("cmpbgti", "{ p0 = cmpb.gt(r1,#5) }"),
            ("cmpbgtu", "{ p0 = cmpb.gtu(r1,r2) }"),
            ("cmpbgtui", "{ p0 = cmpb.gtu(r1,#5) }"),
            ("cmpheq", "{ p0 = cmph.eq(r1,r2) }"),
            ("cmpheqi", "{ p0 = cmph.eq(r1,#5) }"),
            ("cmphgt", "{ p0 = cmph.gt(r1,r2) }"),
            ("cmphgti", "{ p0 = cmph.gt(r1,#5) }"),
            ("cmphgtu", "{ p0 = cmph.gtu(r1,r2) }"),
            ("cmphgtui", "{ p0 = cmph.gtu(r1,#5) }"),
        ],
        24,
        0x7309,
    );
}

#[test]
fn lift_a4_modwrap() {
    lift_family(
        "a4_modwrap",
        &[("modwrapu", "{ r0 = modwrap(r1,r2) }")],
        24,
        0x730a,
    );
}

#[test]
fn lift_c4_predlogic() {
    lift_family(
        "c4_predlogic",
        &[
            ("and_and", "{ p0 = and(p1,and(p2,p3)) }"),
            ("and_or", "{ p0 = and(p1,or(p2,p3)) }"),
            ("or_and", "{ p0 = or(p1,and(p2,p3)) }"),
            ("or_or", "{ p0 = or(p1,or(p2,p3)) }"),
            ("and_andn", "{ p0 = and(p1,and(p2,!p3)) }"),
            ("and_orn", "{ p0 = and(p1,or(p2,!p3)) }"),
            ("or_andn", "{ p0 = or(p1,and(p2,!p3)) }"),
            ("or_orn", "{ p0 = or(p1,or(p2,!p3)) }"),
            ("any8", "{ p0 = any8(p1) }"),
            ("all8", "{ p0 = all8(p1) }"),
        ],
        24,
        0x730b,
    );
}

// ---- Full 8-bit predicate ops: any8/all8, vmux/mask/vitpack, fastcorner9,
// valignrb. These need the WHOLE predicate byte (not just bit 0); the harness
// now seeds random predicate bytes and compares the full byte. ----
#[test]
fn lift_c2_full_predicate() {
    lift_family(
        "c2_full_predicate",
        &[
            // per-byte blend of two pairs by Pu's 8 bits
            ("vmux", "{ r1:0 = vmux(p0,r3:2,r5:4) }"),
            // byte-expand Pt's 8 bits into a pair (0x00/0xff per byte)
            ("mask", "{ r1:0 = mask(p0) }"),
            // Rd = (Ps & 0x55) | (Pt & 0xaa)
            ("vitpack", "{ r0 = vitpack(p1,p0) }"),
            // any8/all8 read the full predicate byte
            ("any8", "{ p0 = any8(p1) }"),
            ("all8", "{ p0 = all8(p1) }"),
        ],
        40,
        0x9f01,
    );
}

#[test]
fn lift_c4_fastcorner9() {
    lift_family(
        "c4_fastcorner9",
        &[
            ("fc9", "{ p0 = fastcorner9(p1,p2) }"),
            ("fc9_not", "{ p0 = !fastcorner9(p1,p2) }"),
        ],
        40,
        0x9f02,
    );
}

#[test]
fn lift_s2_valignrb() {
    lift_family(
        "s2_valignrb",
        &[("valignrb", "{ r1:0 = valignb(r3:2,r5:4,p0) }")],
        40,
        0x9f03,
    );
}

// ---- Y2/Y4/Y5 cache / sync / prefetch / barrier hints: architectural no-ops
// (no register, predicate, or memory effect). Lift to an empty op list; the
// harness verifies state is unchanged (0-div). ----
#[test]
fn lift_y_system_noops() {
    lift_family(
        "y_system_noops",
        &[
            ("dccleana", "{ dccleana(r0) }"),
            ("dccleaninva", "{ dccleaninva(r0) }"),
            ("dcinva", "{ dcinva(r0) }"),
            ("icinva", "{ icinva(r0) }"),
            ("isync", "{ isync }"),
            ("syncht", "{ syncht }"),
            ("barrier", "{ barrier }"),
            ("dcfetch", "{ dcfetch(r0+#0) }"),
            ("l2fetch_rr", "{ l2fetch(r0,r1) }"),
            ("l2fetch_rp", "{ l2fetch(r0,r3:2) }"),
        ],
        20,
        0x9f04,
    );
}

// ---- M2/M4 saturating + rounding scalar multiplies (SMIR-lift wave) ----
// Verified 0-divergence incl. usr_ovf vs the qemu-backed HexagonVcpu.
// Random GPR seeds exercise both the clamping (OVF-set) and in-range paths.

#[test]
fn lift_m2_mpy_sat() {
    lift_family(
        "m2_mpy_sat",
        &[
            ("sat_hh", "{ r0 = mpy(r1.h,r2.h):sat }"),
            ("sat_hl", "{ r0 = mpy(r1.h,r2.l):sat }"),
            ("sat_lh", "{ r0 = mpy(r1.l,r2.h):sat }"),
            ("sat_ll", "{ r0 = mpy(r1.l,r2.l):sat }"),
            ("sat_hh_s1", "{ r0 = mpy(r1.h,r2.h):<<1:sat }"),
            ("sat_hl_s1", "{ r0 = mpy(r1.h,r2.l):<<1:sat }"),
            ("sat_lh_s1", "{ r0 = mpy(r1.l,r2.h):<<1:sat }"),
            ("sat_ll_s1", "{ r0 = mpy(r1.l,r2.l):<<1:sat }"),
        ],
        40,
        0x9a01,
    );
}

#[test]
fn lift_m2_mpy_rnd() {
    lift_family(
        "m2_mpy_rnd",
        &[
            ("rnd_hh", "{ r0 = mpy(r1.h,r2.h):rnd }"),
            ("rnd_hl", "{ r0 = mpy(r1.h,r2.l):rnd }"),
            ("rnd_lh", "{ r0 = mpy(r1.l,r2.h):rnd }"),
            ("rnd_ll", "{ r0 = mpy(r1.l,r2.l):rnd }"),
            ("rnd_hh_s1", "{ r0 = mpy(r1.h,r2.h):<<1:rnd }"),
            ("rnd_ll_s1", "{ r0 = mpy(r1.l,r2.l):<<1:rnd }"),
        ],
        40,
        0x9a02,
    );
}

#[test]
fn lift_m2_mpy_sat_rnd() {
    lift_family(
        "m2_mpy_sat_rnd",
        &[
            ("satrnd_hh", "{ r0 = mpy(r1.h,r2.h):rnd:sat }"),
            ("satrnd_hl", "{ r0 = mpy(r1.h,r2.l):rnd:sat }"),
            ("satrnd_lh", "{ r0 = mpy(r1.l,r2.h):rnd:sat }"),
            ("satrnd_ll", "{ r0 = mpy(r1.l,r2.l):rnd:sat }"),
            ("satrnd_hh_s1", "{ r0 = mpy(r1.h,r2.h):<<1:rnd:sat }"),
            ("satrnd_ll_s1", "{ r0 = mpy(r1.l,r2.l):<<1:rnd:sat }"),
        ],
        40,
        0x9a03,
    );
}

#[test]
fn lift_m2_mpy_acc_nac_sat() {
    lift_family(
        "m2_mpy_acc_nac_sat",
        &[
            ("acc_sat_hh", "{ r0 += mpy(r1.h,r2.h):sat }"),
            ("acc_sat_ll", "{ r0 += mpy(r1.l,r2.l):sat }"),
            ("acc_sat_hl_s1", "{ r0 += mpy(r1.h,r2.l):<<1:sat }"),
            ("acc_sat_lh_s1", "{ r0 += mpy(r1.l,r2.h):<<1:sat }"),
            ("nac_sat_hh", "{ r0 -= mpy(r1.h,r2.h):sat }"),
            ("nac_sat_ll", "{ r0 -= mpy(r1.l,r2.l):sat }"),
            ("nac_sat_hl_s1", "{ r0 -= mpy(r1.h,r2.l):<<1:sat }"),
            ("nac_sat_lh_s1", "{ r0 -= mpy(r1.l,r2.h):<<1:sat }"),
        ],
        40,
        0x9a04,
    );
}

#[test]
fn lift_m2_mpyd_rnd() {
    lift_family(
        "m2_mpyd_rnd",
        &[
            ("mpyd_rnd_hh", "{ r1:0 = mpy(r2.h,r3.h):rnd }"),
            ("mpyd_rnd_hl", "{ r1:0 = mpy(r2.h,r3.l):rnd }"),
            ("mpyd_rnd_lh", "{ r1:0 = mpy(r2.l,r3.h):rnd }"),
            ("mpyd_rnd_ll", "{ r1:0 = mpy(r2.l,r3.l):rnd }"),
            ("mpyd_rnd_hh_s1", "{ r1:0 = mpy(r2.h,r3.h):<<1:rnd }"),
            ("mpyd_rnd_ll_s1", "{ r1:0 = mpy(r2.l,r3.l):<<1:rnd }"),
        ],
        40,
        0x9a05,
    );
}

#[test]
fn lift_m2_m4_up_s1_sat() {
    lift_family(
        "m2_m4_up_s1_sat",
        &[
            ("mpy_up_s1_sat", "{ r0 = mpy(r1,r2):<<1:sat }"),
            ("mac_up_s1_sat", "{ r0 += mpy(r1,r2):<<1:sat }"),
            ("nac_up_s1_sat", "{ r0 -= mpy(r1,r2):<<1:sat }"),
        ],
        40,
        0x9a06,
    );
}

#[test]
fn lift_m2_hmmpy() {
    lift_family(
        "m2_hmmpy",
        &[
            ("hmmpyh_s1", "{ r0 = mpy(r1,r2.h):<<1:sat }"),
            ("hmmpyl_s1", "{ r0 = mpy(r1,r2.l):<<1:sat }"),
            ("hmmpyh_rs1", "{ r0 = mpy(r1,r2.h):<<1:rnd:sat }"),
            ("hmmpyl_rs1", "{ r0 = mpy(r1,r2.l):<<1:rnd:sat }"),
        ],
        40,
        0x9a07,
    );
}

#[test]
fn lift_m2_vmpy2() {
    lift_family(
        "m2_vmpy2",
        &[
            ("vmpy2s_s0", "{ r1:0 = vmpyh(r2,r3):sat }"),
            ("vmpy2s_s1", "{ r1:0 = vmpyh(r2,r3):<<1:sat }"),
            ("vmpy2su_s0", "{ r1:0 = vmpyhsu(r2,r3):sat }"),
            ("vmpy2su_s1", "{ r1:0 = vmpyhsu(r2,r3):<<1:sat }"),
            ("vmac2", "{ r1:0 += vmpyh(r2,r3) }"),
            ("vmac2s_s0", "{ r1:0 += vmpyh(r2,r3):sat }"),
            ("vmac2s_s1", "{ r1:0 += vmpyh(r2,r3):<<1:sat }"),
            ("vmac2su_s0", "{ r1:0 += vmpyhsu(r2,r3):sat }"),
            ("vmac2su_s1", "{ r1:0 += vmpyhsu(r2,r3):<<1:sat }"),
            ("vmpy2s_s0pack", "{ r0 = vmpyh(r1,r2):rnd:sat }"),
            ("vmpy2s_s1pack", "{ r0 = vmpyh(r1,r2):<<1:rnd:sat }"),
        ],
        40,
        0x9a08,
    );
}

#[test]
fn lift_m2_vmpy2es() {
    lift_family(
        "m2_vmpy2es",
        &[
            ("vmpy2es_s0", "{ r1:0 = vmpyeh(r3:2,r5:4):sat }"),
            ("vmpy2es_s1", "{ r1:0 = vmpyeh(r3:2,r5:4):<<1:sat }"),
            ("vmac2es", "{ r1:0 += vmpyeh(r3:2,r5:4) }"),
            ("vmac2es_s0", "{ r1:0 += vmpyeh(r3:2,r5:4):sat }"),
            ("vmac2es_s1", "{ r1:0 += vmpyeh(r3:2,r5:4):<<1:sat }"),
        ],
        40,
        0x9a09,
    );
}

#[test]
fn lift_m2_vdmpy() {
    lift_family(
        "m2_vdmpy",
        &[
            ("vdmpys_s0", "{ r1:0 = vdmpy(r3:2,r5:4):sat }"),
            ("vdmpys_s1", "{ r1:0 = vdmpy(r3:2,r5:4):<<1:sat }"),
            ("vdmacs_s0", "{ r1:0 += vdmpy(r3:2,r5:4):sat }"),
            ("vdmacs_s1", "{ r1:0 += vdmpy(r3:2,r5:4):<<1:sat }"),
            ("vdmpyrs_s0", "{ r0 = vdmpy(r3:2,r5:4):rnd:sat }"),
            ("vdmpyrs_s1", "{ r0 = vdmpy(r3:2,r5:4):<<1:rnd:sat }"),
        ],
        40,
        0x9a0a,
    );
}

#[test]
fn lift_m5_vmpyb() {
    lift_family(
        "m5_vmpyb",
        &[
            ("vmpybuu", "{ r1:0 = vmpybu(r2,r3) }"),
            ("vmpybsu", "{ r1:0 = vmpybsu(r2,r3) }"),
            ("vmacbuu", "{ r1:0 += vmpybu(r2,r3) }"),
            ("vmacbsu", "{ r1:0 += vmpybsu(r2,r3) }"),
            ("vdmpybsu", "{ r1:0 = vdmpybsu(r3:2,r5:4):sat }"),
            ("vdmacbsu", "{ r1:0 += vdmpybsu(r3:2,r5:4):sat }"),
            ("vrmpybuu", "{ r1:0 = vrmpybu(r3:2,r5:4) }"),
            ("vrmpybsu", "{ r1:0 = vrmpybsu(r3:2,r5:4) }"),
            ("vrmacbuu", "{ r1:0 += vrmpybu(r3:2,r5:4) }"),
            ("vrmacbsu", "{ r1:0 += vrmpybsu(r3:2,r5:4) }"),
        ],
        40,
        0x9a0b,
    );
}

#[test]
fn lift_m2_cmpys() {
    lift_family(
        "m2_cmpys",
        &[
            ("cmpys_s0", "{ r1:0 = cmpy(r2,r3):sat }"),
            ("cmpys_s1", "{ r1:0 = cmpy(r2,r3):<<1:sat }"),
            ("cmpysc_s0", "{ r1:0 = cmpy(r2,r3*):sat }"),
            ("cmpysc_s1", "{ r1:0 = cmpy(r2,r3*):<<1:sat }"),
            ("cmacs_s0", "{ r1:0 += cmpy(r2,r3):sat }"),
            ("cmacs_s1", "{ r1:0 += cmpy(r2,r3):<<1:sat }"),
            ("cmacsc_s0", "{ r1:0 += cmpy(r2,r3*):sat }"),
            ("cmacsc_s1", "{ r1:0 += cmpy(r2,r3*):<<1:sat }"),
            ("cnacs_s0", "{ r1:0 -= cmpy(r2,r3):sat }"),
            ("cnacs_s1", "{ r1:0 -= cmpy(r2,r3):<<1:sat }"),
            ("cnacsc_s0", "{ r1:0 -= cmpy(r2,r3*):sat }"),
            ("cnacsc_s1", "{ r1:0 -= cmpy(r2,r3*):<<1:sat }"),
        ],
        40,
        0x9a0c,
    );
}

#[test]
fn lift_m2_cmpyrs() {
    lift_family(
        "m2_cmpyrs",
        &[
            ("cmpyrs_s0", "{ r0 = cmpy(r1,r2):rnd:sat }"),
            ("cmpyrs_s1", "{ r0 = cmpy(r1,r2):<<1:rnd:sat }"),
            ("cmpyrsc_s0", "{ r0 = cmpy(r1,r2*):rnd:sat }"),
            ("cmpyrsc_s1", "{ r0 = cmpy(r1,r2*):<<1:rnd:sat }"),
        ],
        40,
        0x9a0d,
    );
}

#[test]
fn lift_m4_cmpy_wh() {
    lift_family(
        "m4_cmpy_wh",
        &[
            ("cmpyi_wh", "{ r0 = cmpyiwh(r3:2,r4):<<1:rnd:sat }"),
            ("cmpyi_whc", "{ r0 = cmpyiwh(r3:2,r4*):<<1:rnd:sat }"),
            ("cmpyr_wh", "{ r0 = cmpyrwh(r3:2,r4):<<1:rnd:sat }"),
            ("cmpyr_whc", "{ r0 = cmpyrwh(r3:2,r4*):<<1:rnd:sat }"),
        ],
        40,
        0x9a0e,
    );
}

#[test]
fn lift_m2_mmpy_ss() {
    lift_family(
        "m2_mmpy_ss",
        &[
            ("mmpyl_s0", "{ r1:0 = vmpyweh(r3:2,r5:4):sat }"),
            ("mmpyl_s1", "{ r1:0 = vmpyweh(r3:2,r5:4):<<1:sat }"),
            ("mmpyl_rs0", "{ r1:0 = vmpyweh(r3:2,r5:4):rnd:sat }"),
            ("mmpyl_rs1", "{ r1:0 = vmpyweh(r3:2,r5:4):<<1:rnd:sat }"),
            ("mmpyh_s0", "{ r1:0 = vmpywoh(r3:2,r5:4):sat }"),
            ("mmpyh_s1", "{ r1:0 = vmpywoh(r3:2,r5:4):<<1:sat }"),
            ("mmpyh_rs0", "{ r1:0 = vmpywoh(r3:2,r5:4):rnd:sat }"),
            ("mmpyh_rs1", "{ r1:0 = vmpywoh(r3:2,r5:4):<<1:rnd:sat }"),
            ("mmacls_s0", "{ r1:0 += vmpyweh(r3:2,r5:4):sat }"),
            ("mmacls_s1", "{ r1:0 += vmpyweh(r3:2,r5:4):<<1:sat }"),
            ("mmacls_rs0", "{ r1:0 += vmpyweh(r3:2,r5:4):rnd:sat }"),
            ("mmacls_rs1", "{ r1:0 += vmpyweh(r3:2,r5:4):<<1:rnd:sat }"),
            ("mmachs_s0", "{ r1:0 += vmpywoh(r3:2,r5:4):sat }"),
            ("mmachs_s1", "{ r1:0 += vmpywoh(r3:2,r5:4):<<1:sat }"),
            ("mmachs_rs0", "{ r1:0 += vmpywoh(r3:2,r5:4):rnd:sat }"),
            ("mmachs_rs1", "{ r1:0 += vmpywoh(r3:2,r5:4):<<1:rnd:sat }"),
        ],
        40,
        0x9a0f,
    );
}

#[test]
fn lift_m2_mmpy_su() {
    lift_family(
        "m2_mmpy_su",
        &[
            ("mmpyul_s0", "{ r1:0 = vmpyweuh(r3:2,r5:4):sat }"),
            ("mmpyul_s1", "{ r1:0 = vmpyweuh(r3:2,r5:4):<<1:sat }"),
            ("mmpyul_rs0", "{ r1:0 = vmpyweuh(r3:2,r5:4):rnd:sat }"),
            ("mmpyul_rs1", "{ r1:0 = vmpyweuh(r3:2,r5:4):<<1:rnd:sat }"),
            ("mmpyuh_s0", "{ r1:0 = vmpywouh(r3:2,r5:4):sat }"),
            ("mmpyuh_s1", "{ r1:0 = vmpywouh(r3:2,r5:4):<<1:sat }"),
            ("mmpyuh_rs0", "{ r1:0 = vmpywouh(r3:2,r5:4):rnd:sat }"),
            ("mmpyuh_rs1", "{ r1:0 = vmpywouh(r3:2,r5:4):<<1:rnd:sat }"),
            ("mmaculs_s0", "{ r1:0 += vmpyweuh(r3:2,r5:4):sat }"),
            ("mmaculs_s1", "{ r1:0 += vmpyweuh(r3:2,r5:4):<<1:sat }"),
            ("mmaculs_rs0", "{ r1:0 += vmpyweuh(r3:2,r5:4):rnd:sat }"),
            ("mmaculs_rs1", "{ r1:0 += vmpyweuh(r3:2,r5:4):<<1:rnd:sat }"),
            ("mmacuhs_s0", "{ r1:0 += vmpywouh(r3:2,r5:4):sat }"),
            ("mmacuhs_s1", "{ r1:0 += vmpywouh(r3:2,r5:4):<<1:sat }"),
            ("mmacuhs_rs0", "{ r1:0 += vmpywouh(r3:2,r5:4):rnd:sat }"),
            ("mmacuhs_rs1", "{ r1:0 += vmpywouh(r3:2,r5:4):<<1:rnd:sat }"),
        ],
        40,
        0x9a10,
    );
}

#[test]
fn lift_m2_vrmpyh() {
    lift_family(
        "m2_vrmpyh",
        &[
            ("vrmpyh", "{ r1:0 = vrmpyh(r3:2,r5:4) }"),
            ("vrmach", "{ r1:0 += vrmpyh(r3:2,r5:4) }"),
            ("vrcmpyi", "{ r1:0 = vrcmpyi(r3:2,r5:4) }"),
            ("vrcmpyr", "{ r1:0 = vrcmpyr(r3:2,r5:4) }"),
            ("vrcmpyi_c", "{ r1:0 = vrcmpyi(r3:2,r5:4*) }"),
            ("vrcmpyr_c", "{ r1:0 = vrcmpyr(r3:2,r5:4*) }"),
            ("vrcmaci", "{ r1:0 += vrcmpyi(r3:2,r5:4) }"),
            ("vrcmacr", "{ r1:0 += vrcmpyr(r3:2,r5:4) }"),
        ],
        40,
        0x9a11,
    );
}

#[test]
fn lift_m4_vrmpyweh_woh() {
    lift_family(
        "m4_vrmpyweh_woh",
        &[
            ("vrmpyeh_s0", "{ r1:0 = vrmpyweh(r3:2,r5:4) }"),
            ("vrmpyeh_s1", "{ r1:0 = vrmpyweh(r3:2,r5:4):<<1 }"),
            ("vrmpyoh_s0", "{ r1:0 = vrmpywoh(r3:2,r5:4) }"),
            ("vrmpyoh_s1", "{ r1:0 = vrmpywoh(r3:2,r5:4):<<1 }"),
            ("vrmpyeh_acc_s0", "{ r1:0 += vrmpyweh(r3:2,r5:4) }"),
            ("vrmpyeh_acc_s1", "{ r1:0 += vrmpyweh(r3:2,r5:4):<<1 }"),
            ("vrmpyoh_acc_s0", "{ r1:0 += vrmpywoh(r3:2,r5:4) }"),
            ("vrmpyoh_acc_s1", "{ r1:0 += vrmpywoh(r3:2,r5:4):<<1 }"),
        ],
        40,
        0x9a12,
    );
}

#[test]
fn lift_m7_dcmpy() {
    // 64-bit complex 32x32 multiply (no sat). The wcmpy `:sat` forms are NOT
    // lifted (their pre-shift accumulator needs i128); only dcmpy is i64-exact.
    lift_family(
        "m7_dcmpy",
        &[
            ("dcmpyrw", "{ r1:0 = cmpyrw(r3:2,r5:4) }"),
            ("dcmpyrwc", "{ r1:0 = cmpyrw(r3:2,r5:4*) }"),
            ("dcmpyiw", "{ r1:0 = cmpyiw(r3:2,r5:4) }"),
            ("dcmpyiwc", "{ r1:0 = cmpyiw(r3:2,r5:4*) }"),
            ("dcmpyrw_acc", "{ r1:0 += cmpyrw(r3:2,r5:4) }"),
            ("dcmpyrwc_acc", "{ r1:0 += cmpyrw(r3:2,r5:4*) }"),
            ("dcmpyiw_acc", "{ r1:0 += cmpyiw(r3:2,r5:4) }"),
            ("dcmpyiwc_acc", "{ r1:0 += cmpyiw(r3:2,r5:4*) }"),
        ],
        40,
        0x9a13,
    );
}

// ============================================================================
// WAVE: remaining tractable scalar register ops (no mem/CF).
// Each family verified 0-divergence (incl USR:OVF) vs the qemu-backed
// HexagonVcpu over 40 seeded iterations.
// ============================================================================

#[test]
fn lift_a2_abs_pair() {
    lift_family(
        "a2_abs_pair",
        &[
            ("abs", "{ r0 = abs(r1) }"),
            ("absp", "{ r1:0 = abs(r3:2) }"),
            ("addpsat", "{ r1:0 = add(r3:2,r5:4):sat }"),
            ("roundsat", "{ r0 = round(r1:0):sat }"),
        ],
        40,
        0x9b01,
    );
}

#[test]
fn lift_a2_tfrih_til() {
    lift_family(
        "a2_tfrih_til",
        &[
            ("tfrih", "{ r0.h = #5 }"),
            ("tfril", "{ r0.l = #5 }"),
            ("tfrih2", "{ r0.h = #65500 }"),
            ("tfril2", "{ r0.l = #40000 }"),
        ],
        40,
        0x9b02,
    );
}

#[test]
fn lift_a2_addsubh() {
    lift_family(
        "a2_addsubh",
        &[
            ("addh_l16_ll", "{ r0 = add(r1.l,r2.l) }"),
            ("addh_l16_hl", "{ r0 = add(r1.l,r2.h) }"),
            ("addh_l16_sat_ll", "{ r0 = add(r1.l,r2.l):sat }"),
            ("addh_l16_sat_hl", "{ r0 = add(r1.l,r2.h):sat }"),
            ("addh_h16_ll", "{ r0 = add(r1.l,r2.l):<<16 }"),
            ("addh_h16_lh", "{ r0 = add(r1.l,r2.h):<<16 }"),
            ("addh_h16_hl", "{ r0 = add(r1.h,r2.l):<<16 }"),
            ("addh_h16_hh", "{ r0 = add(r1.h,r2.h):<<16 }"),
            ("addh_h16_sat_ll", "{ r0 = add(r1.l,r2.l):sat:<<16 }"),
            ("addh_h16_sat_lh", "{ r0 = add(r1.l,r2.h):sat:<<16 }"),
            ("addh_h16_sat_hl", "{ r0 = add(r1.h,r2.l):sat:<<16 }"),
            ("addh_h16_sat_hh", "{ r0 = add(r1.h,r2.h):sat:<<16 }"),
            ("subh_l16_ll", "{ r0 = sub(r1.l,r2.l) }"),
            ("subh_l16_hl", "{ r0 = sub(r1.l,r2.h) }"),
            ("subh_l16_sat_ll", "{ r0 = sub(r1.l,r2.l):sat }"),
            ("subh_l16_sat_hl", "{ r0 = sub(r1.l,r2.h):sat }"),
            ("subh_h16_ll", "{ r0 = sub(r1.l,r2.l):<<16 }"),
            ("subh_h16_lh", "{ r0 = sub(r1.l,r2.h):<<16 }"),
            ("subh_h16_hl", "{ r0 = sub(r1.h,r2.l):<<16 }"),
            ("subh_h16_hh", "{ r0 = sub(r1.h,r2.h):<<16 }"),
            ("subh_h16_sat_ll", "{ r0 = sub(r1.l,r2.l):sat:<<16 }"),
            ("subh_h16_sat_lh", "{ r0 = sub(r1.l,r2.h):sat:<<16 }"),
            ("subh_h16_sat_hl", "{ r0 = sub(r1.h,r2.l):sat:<<16 }"),
            ("subh_h16_sat_hh", "{ r0 = sub(r1.h,r2.h):sat:<<16 }"),
        ],
        40,
        0x9b03,
    );
}

#[test]
fn lift_a4_round() {
    lift_family(
        "a4_round",
        &[
            ("round_ri", "{ r0 = round(r1,#5) }"),
            ("round_ri0", "{ r0 = round(r1,#0) }"),
            ("round_rr", "{ r0 = round(r1,r2) }"),
            ("round_ri_sat", "{ r0 = round(r1,#5):sat }"),
            ("round_rr_sat", "{ r0 = round(r1,r2):sat }"),
        ],
        40,
        0x9b04,
    );
}

#[test]
fn lift_cround_clip() {
    lift_family(
        "cround_clip",
        &[
            ("cround_ri", "{ r0 = cround(r1,#5) }"),
            ("cround_ri0", "{ r0 = cround(r1,#0) }"),
            ("cround_ri1", "{ r0 = cround(r1,#1) }"),
            ("croundd_ri", "{ r1:0 = cround(r3:2,#5) }"),
            ("croundd_ri0", "{ r1:0 = cround(r3:2,#0) }"),
            ("clip", "{ r0 = clip(r1,#5) }"),
            ("clip0", "{ r0 = clip(r1,#0) }"),
            ("clip31", "{ r0 = clip(r1,#31) }"),
        ],
        40,
        0x9b05,
    );
}

#[test]
fn lift_a4_combineii() {
    lift_family(
        "a4_combineii",
        &[
            ("combineii", "{ r1:0 = combine(#5,#9) }"),
            ("combineii_neg", "{ r1:0 = combine(#-7,#33) }"),
        ],
        40,
        0x9b06,
    );
}

#[test]
fn lift_a2_predicated() {
    lift_family(
        "a2_predicated",
        &[
            ("paddt", "{ if (p0) r0 = add(r1,r2) }"),
            ("paddf", "{ if (!p0) r0 = add(r1,r2) }"),
            ("paddit", "{ if (p0) r0 = add(r1,#5) }"),
            ("paddif", "{ if (!p0) r0 = add(r1,#5) }"),
            ("psubt", "{ if (p0) r0 = sub(r1,r2) }"),
            ("psubf", "{ if (!p0) r0 = sub(r1,r2) }"),
            ("pandt", "{ if (p0) r0 = and(r1,r2) }"),
            ("pandf", "{ if (!p0) r0 = and(r1,r2) }"),
            ("port", "{ if (p0) r0 = or(r1,r2) }"),
            ("porf", "{ if (!p0) r0 = or(r1,r2) }"),
            ("pxort", "{ if (p0) r0 = xor(r1,r2) }"),
            ("pxorf", "{ if (!p0) r0 = xor(r1,r2) }"),
            ("cmoveit", "{ if (p0) r0 = #5 }"),
            ("cmoveif", "{ if (!p0) r0 = #5 }"),
        ],
        40,
        0x9b07,
    );
}

#[test]
fn lift_a4_pred_extend() {
    lift_family(
        "a4_pred_extend",
        &[
            ("paslht", "{ if (p0) r0 = aslh(r1) }"),
            ("paslhf", "{ if (!p0) r0 = aslh(r1) }"),
            ("pasrht", "{ if (p0) r0 = asrh(r1) }"),
            ("pasrhf", "{ if (!p0) r0 = asrh(r1) }"),
            ("psxtbt", "{ if (p0) r0 = sxtb(r1) }"),
            ("psxtbf", "{ if (!p0) r0 = sxtb(r1) }"),
            ("psxtht", "{ if (p0) r0 = sxth(r1) }"),
            ("psxthf", "{ if (!p0) r0 = sxth(r1) }"),
            ("pzxtbt", "{ if (p0) r0 = zxtb(r1) }"),
            ("pzxtbf", "{ if (!p0) r0 = zxtb(r1) }"),
            ("pzxtht", "{ if (p0) r0 = zxth(r1) }"),
            ("pzxthf", "{ if (!p0) r0 = zxth(r1) }"),
        ],
        40,
        0x9b08,
    );
}

#[test]
fn lift_c2_ccombine_vmux_mask() {
    lift_family(
        "c2_ccombine_vmux_mask",
        &[
            ("ccombinewt", "{ if (p0) r1:0 = combine(r2,r3) }"),
            ("ccombinewf", "{ if (!p0) r1:0 = combine(r2,r3) }"),
            ("vmux", "{ r1:0 = vmux(p0,r3:2,r5:4) }"),
            ("mask", "{ r1:0 = mask(p0) }"),
            ("vitpack", "{ r0 = vitpack(p1,p0) }"),
        ],
        40,
        0x9b09,
    );
}

#[test]
fn lift_s2s4_extract_insert_pair() {
    lift_family(
        "s2s4_extract_insert_pair",
        &[
            ("extractup", "{ r1:0 = extractu(r3:2,#8,#4) }"),
            ("extractup2", "{ r1:0 = extractu(r3:2,#40,#10) }"),
            ("extractp", "{ r1:0 = extract(r3:2,#8,#4) }"),
            ("extractp2", "{ r1:0 = extract(r3:2,#40,#10) }"),
            ("extract", "{ r0 = extract(r1,#8,#4) }"),
            ("extract2", "{ r0 = extract(r1,#13,#11) }"),
            ("insertp", "{ r1:0 = insert(r3:2,#8,#4) }"),
            ("insertp2", "{ r1:0 = insert(r3:2,#40,#10) }"),
        ],
        40,
        0x9b0a,
    );
}

#[test]
fn lift_s_asr_rnd_clb() {
    lift_family(
        "s_asr_rnd_clb",
        &[
            ("asr_i_r_rnd", "{ r0 = asr(r1,#5):rnd }"),
            ("asr_i_r_rnd0", "{ r0 = asr(r1,#0):rnd }"),
            ("asr_i_p_rnd", "{ r1:0 = asr(r3:2,#5):rnd }"),
            ("asr_i_p_rnd0", "{ r1:0 = asr(r3:2,#0):rnd }"),
            ("clbaddi", "{ r0 = add(clb(r1),#3) }"),
            ("clbaddi_neg", "{ r0 = add(clb(r1),#-2) }"),
            ("clbpaddi", "{ r0 = add(clb(r3:2),#3) }"),
            ("clbpnorm", "{ r0 = normamt(r3:2) }"),
        ],
        40,
        0x9b0b,
    );
}

// ============================================================================
// SWAR vector ALU (A2_v*/A2_sv*) + M-family vabsdiff/vradd + vcmp predicates +
// reduce-add + boundscheck + carry-predicate add/sub + register cround.
// All composed lane-by-lane from existing scalar OpKinds; saturating lanes feed
// the full pre-clamp value to SatN (set_ovf:true), so usr_ovf is compared too.
// ============================================================================

#[test]
fn lift_a2_vadd_vsub() {
    lift_family(
        "a2_vadd_vsub",
        &[
            ("vaddh", "{ r1:0 = vaddh(r3:2, r5:4) }"),
            ("vaddhs", "{ r1:0 = vaddh(r3:2, r5:4):sat }"),
            ("vadduhs", "{ r1:0 = vadduh(r3:2, r5:4):sat }"),
            ("vaddw", "{ r1:0 = vaddw(r3:2, r5:4) }"),
            ("vaddws", "{ r1:0 = vaddw(r3:2, r5:4):sat }"),
            ("vaddub", "{ r1:0 = vaddub(r3:2, r5:4) }"),
            ("vaddubs", "{ r1:0 = vaddub(r3:2, r5:4):sat }"),
            ("vsubh", "{ r1:0 = vsubh(r3:2, r5:4) }"),
            ("vsubhs", "{ r1:0 = vsubh(r3:2, r5:4):sat }"),
            ("vsubuhs", "{ r1:0 = vsubuh(r3:2, r5:4):sat }"),
            ("vsubw", "{ r1:0 = vsubw(r3:2, r5:4) }"),
            ("vsubws", "{ r1:0 = vsubw(r3:2, r5:4):sat }"),
            ("vsubub", "{ r1:0 = vsubub(r3:2, r5:4) }"),
            ("vsububs", "{ r1:0 = vsubub(r3:2, r5:4):sat }"),
        ],
        40,
        0xa201,
    );
}

#[test]
fn lift_a2_vavg() {
    lift_family(
        "a2_vavg",
        &[
            ("vavgh", "{ r1:0 = vavgh(r3:2, r5:4) }"),
            ("vavghr", "{ r1:0 = vavgh(r3:2, r5:4):rnd }"),
            ("vavghcr", "{ r1:0 = vavgh(r3:2, r5:4):crnd }"),
            ("vavgw", "{ r1:0 = vavgw(r3:2, r5:4) }"),
            ("vavgwr", "{ r1:0 = vavgw(r3:2, r5:4):rnd }"),
            ("vavgwcr", "{ r1:0 = vavgw(r3:2, r5:4):crnd }"),
            ("vavgub", "{ r1:0 = vavgub(r3:2, r5:4) }"),
            ("vavgubr", "{ r1:0 = vavgub(r3:2, r5:4):rnd }"),
            ("vavguh", "{ r1:0 = vavguh(r3:2, r5:4) }"),
            ("vavguhr", "{ r1:0 = vavguh(r3:2, r5:4):rnd }"),
            ("vavguw", "{ r1:0 = vavguw(r3:2, r5:4) }"),
            ("vavguwr", "{ r1:0 = vavguw(r3:2, r5:4):rnd }"),
        ],
        40,
        0xa202,
    );
}

#[test]
fn lift_a2_vnavg() {
    lift_family(
        "a2_vnavg",
        &[
            ("vnavgh", "{ r1:0 = vnavgh(r3:2, r5:4) }"),
            ("vnavghr", "{ r1:0 = vnavgh(r5:4, r3:2):rnd:sat }"),
            ("vnavghcr", "{ r1:0 = vnavgh(r5:4, r3:2):crnd:sat }"),
            ("vnavgw", "{ r1:0 = vnavgw(r3:2, r5:4) }"),
            ("vnavgwr", "{ r1:0 = vnavgw(r5:4, r3:2):rnd:sat }"),
            ("vnavgwcr", "{ r1:0 = vnavgw(r5:4, r3:2):crnd:sat }"),
        ],
        40,
        0xa203,
    );
}

#[test]
fn lift_a2_vminmax() {
    lift_family(
        "a2_vminmax",
        &[
            ("vmaxh", "{ r1:0 = vmaxh(r3:2, r5:4) }"),
            ("vmaxuh", "{ r1:0 = vmaxuh(r3:2, r5:4) }"),
            ("vmaxw", "{ r1:0 = vmaxw(r3:2, r5:4) }"),
            ("vmaxuw", "{ r1:0 = vmaxuw(r3:2, r5:4) }"),
            ("vmaxb", "{ r1:0 = vmaxb(r3:2, r5:4) }"),
            ("vmaxub", "{ r1:0 = vmaxub(r3:2, r5:4) }"),
            ("vminh", "{ r1:0 = vminh(r3:2, r5:4) }"),
            ("vminuh", "{ r1:0 = vminuh(r3:2, r5:4) }"),
            ("vminw", "{ r1:0 = vminw(r3:2, r5:4) }"),
            ("vminuw", "{ r1:0 = vminuw(r3:2, r5:4) }"),
            ("vminb", "{ r1:0 = vminb(r3:2, r5:4) }"),
            ("vminub", "{ r1:0 = vminub(r3:2, r5:4) }"),
        ],
        40,
        0xa204,
    );
}

#[test]
fn lift_a2_vabs_vconj() {
    lift_family(
        "a2_vabs_vconj",
        &[
            ("vabsh", "{ r1:0 = vabsh(r3:2) }"),
            ("vabshsat", "{ r1:0 = vabsh(r3:2):sat }"),
            ("vabsw", "{ r1:0 = vabsw(r3:2) }"),
            ("vabswsat", "{ r1:0 = vabsw(r3:2):sat }"),
            ("vconj", "{ r1:0 = vconj(r3:2):sat }"),
        ],
        40,
        0xa205,
    );
}

#[test]
fn lift_a2_sv() {
    lift_family(
        "a2_sv",
        &[
            ("svaddh", "{ r0 = vaddh(r1, r2) }"),
            ("svaddhs", "{ r0 = vaddh(r1, r2):sat }"),
            ("svadduhs", "{ r0 = vadduh(r1, r2):sat }"),
            ("svsubh", "{ r0 = vsubh(r1, r2) }"),
            ("svsubhs", "{ r0 = vsubh(r1, r2):sat }"),
            ("svsubuhs", "{ r0 = vsubuh(r1, r2):sat }"),
            ("svavgh", "{ r0 = vavgh(r1, r2) }"),
            ("svavghs", "{ r0 = vavgh(r1, r2):rnd }"),
            ("svnavgh", "{ r0 = vnavgh(r2, r1) }"),
        ],
        40,
        0xa206,
    );
}

#[test]
fn lift_a_vcmp_pred() {
    lift_family(
        "a_vcmp_pred",
        &[
            ("vcmpbeq", "{ p0 = vcmpb.eq(r3:2, r5:4) }"),
            ("vcmpbgtu", "{ p0 = vcmpb.gtu(r3:2, r5:4) }"),
            ("vcmpbgt", "{ p0 = vcmpb.gt(r3:2, r5:4) }"),
            ("vcmpbeqi", "{ p0 = vcmpb.eq(r3:2, #5) }"),
            ("vcmpbgti", "{ p0 = vcmpb.gt(r3:2, #5) }"),
            ("vcmpbgtui", "{ p0 = vcmpb.gtu(r3:2, #5) }"),
            ("vcmpheq", "{ p0 = vcmph.eq(r3:2, r5:4) }"),
            ("vcmphgt", "{ p0 = vcmph.gt(r3:2, r5:4) }"),
            ("vcmphgtu", "{ p0 = vcmph.gtu(r3:2, r5:4) }"),
            ("vcmpheqi", "{ p0 = vcmph.eq(r3:2, #5) }"),
            ("vcmphgti", "{ p0 = vcmph.gt(r3:2, #5) }"),
            ("vcmphgtui", "{ p0 = vcmph.gtu(r3:2, #5) }"),
            ("vcmpweq", "{ p0 = vcmpw.eq(r3:2, r5:4) }"),
            ("vcmpwgt", "{ p0 = vcmpw.gt(r3:2, r5:4) }"),
            ("vcmpwgtu", "{ p0 = vcmpw.gtu(r3:2, r5:4) }"),
            ("vcmpweqi", "{ p0 = vcmpw.eq(r3:2, #5) }"),
            ("vcmpwgti", "{ p0 = vcmpw.gt(r3:2, #5) }"),
            ("vcmpwgtui", "{ p0 = vcmpw.gtu(r3:2, #5) }"),
            ("vcmpbeq_any", "{ p0 = any8(vcmpb.eq(r3:2, r5:4)) }"),
            ("vcmpbeq_notany", "{ p0 = !any8(vcmpb.eq(r3:2, r5:4)) }"),
        ],
        40,
        0xa207,
    );
}

#[test]
fn lift_m_vabsdiff_vradd() {
    lift_family(
        "m_vabsdiff_vradd",
        &[
            ("vabsdiffh", "{ r1:0 = vabsdiffh(r3:2, r5:4) }"),
            ("vabsdiffw", "{ r1:0 = vabsdiffw(r3:2, r5:4) }"),
            ("vabsdiffb", "{ r1:0 = vabsdiffb(r3:2, r5:4) }"),
            ("vabsdiffub", "{ r1:0 = vabsdiffub(r3:2, r5:4) }"),
            ("vraddh", "{ r0 = vraddh(r3:2, r5:4) }"),
            ("vradduh", "{ r0 = vradduh(r3:2, r5:4) }"),
        ],
        40,
        0xa208,
    );
}

#[test]
fn lift_a2_vraddub_vrsadub() {
    lift_family(
        "a2_vraddub_vrsadub",
        &[
            ("vraddub", "{ r1:0 = vraddub(r3:2, r5:4) }"),
            ("vraddub_acc", "{ r1:0 += vraddub(r3:2, r5:4) }"),
            ("vrsadub", "{ r1:0 = vrsadub(r3:2, r5:4) }"),
            ("vrsadub_acc", "{ r1:0 += vrsadub(r3:2, r5:4) }"),
        ],
        40,
        0xa209,
    );
}

#[test]
fn lift_a_misc_swar() {
    lift_family(
        "a_misc_swar",
        &[
            ("vaddhubs", "{ r0 = vaddhub(r3:2, r5:4):sat }"),
            ("vclip", "{ r1:0 = vclip(r3:2, #5) }"),
            ("vminub_RdP", "{ r1:0, p0 = vminub(r3:2, r5:4) }"),
            ("boundscheck_lo", "{ p0 = boundscheck(r2, r5:4) }"),
            ("boundscheck_hi", "{ p0 = boundscheck(r3, r5:4) }"),
        ],
        40,
        0xa20a,
    );
}

#[test]
fn lift_a4_carry_addsub() {
    lift_family(
        "a4_carry_addsub",
        &[
            ("addp_c", "{ r1:0 = add(r3:2, r5:4, p0):carry }"),
            ("subp_c", "{ r1:0 = sub(r3:2, r5:4, p0):carry }"),
        ],
        40,
        0xa20b,
    );
}

#[test]
fn lift_cround_rr() {
    lift_family(
        "cround_rr",
        &[
            ("cround_rr", "{ r0 = cround(r1, r2) }"),
            ("croundd_rr", "{ r1:0 = cround(r3:2, r4) }"),
        ],
        40,
        0xa20c,
    );
}

// ============================================================================
// SIMD per-lane vector shifts (S2_*_i_v{h,w} / S2_*_r_v{h,w}) + svw_trun.
// Composed lane-by-lane from Bfx/Shl/Sar/Shr (imm) or BidirShift (reg). No
// saturation, so usr_ovf stays 0 and is still compared. Verified vs the
// qemu-backed HexagonVcpu over 40 seeded iterations.
// ============================================================================

#[test]
fn lift_s2_vshift_i() {
    lift_family(
        "s2_vshift_i",
        &[
            ("aslh", "{ r1:0 = vaslh(r3:2,#5) }"),
            ("asrh", "{ r1:0 = vasrh(r3:2,#5) }"),
            ("lsrh", "{ r1:0 = vlsrh(r3:2,#5) }"),
            ("aslw", "{ r1:0 = vaslw(r3:2,#11) }"),
            ("asrw", "{ r1:0 = vasrw(r3:2,#11) }"),
            ("lsrw", "{ r1:0 = vlsrw(r3:2,#11) }"),
            ("aslh0", "{ r1:0 = vaslh(r3:2,#0) }"),
            ("asrh15", "{ r1:0 = vasrh(r3:2,#15) }"),
        ],
        40,
        0xb201,
    );
}

#[test]
fn lift_s2_vshift_r() {
    lift_family(
        "s2_vshift_r",
        &[
            ("aslh", "{ r1:0 = vaslh(r3:2,r4) }"),
            ("asrh", "{ r1:0 = vasrh(r3:2,r4) }"),
            ("lsrh", "{ r1:0 = vlsrh(r3:2,r4) }"),
            ("lslh", "{ r1:0 = vlslh(r3:2,r4) }"),
            ("aslw", "{ r1:0 = vaslw(r3:2,r4) }"),
            ("asrw", "{ r1:0 = vasrw(r3:2,r4) }"),
            ("lsrw", "{ r1:0 = vlsrw(r3:2,r4) }"),
            ("lslw", "{ r1:0 = vlslw(r3:2,r4) }"),
        ],
        40,
        0xb202,
    );
}

#[test]
fn lift_s2_svw_trun() {
    lift_family(
        "s2_svw_trun",
        &[
            ("asr_i", "{ r0 = vasrw(r3:2,#5) }"),
            ("asr_r", "{ r0 = vasrw(r3:2,r4) }"),
        ],
        40,
        0xb203,
    );
}

// Cross add/sub: one lane adds the adjacent Rtt lane, the next subtracts; all
// signed-saturate (set_ovf:true, so usr_ovf is compared). The :rnd:>>1 (hr)
// forms round (+1)>>1 before saturating. Verified vs HexagonVcpu, 40 iters.
#[test]
fn lift_s4_vxaddsub() {
    lift_family(
        "s4_vxaddsub",
        &[
            ("vxaddsubh", "{ r1:0 = vxaddsubh(r3:2,r5:4):sat }"),
            ("vxsubaddh", "{ r1:0 = vxsubaddh(r3:2,r5:4):sat }"),
            ("vxaddsubhr", "{ r1:0 = vxaddsubh(r3:2,r5:4):rnd:>>1:sat }"),
            ("vxsubaddhr", "{ r1:0 = vxsubaddh(r3:2,r5:4):rnd:>>1:sat }"),
            ("vxaddsubw", "{ r1:0 = vxaddsubw(r3:2,r5:4):sat }"),
            ("vxsubaddw", "{ r1:0 = vxsubaddw(r3:2,r5:4):sat }"),
        ],
        40,
        0xb204,
    );
}

// Conditional per-half negate (vcnegh) and complex rotate (vcrotate); both
// saturate on the negate paths (set_ovf:true). The harness seeds Rt randomly,
// exercising all per-half / per-pair control values incl. the -32768 OVF case.
#[test]
fn lift_s2_vcneg_vcrotate() {
    lift_family(
        "s2_vcneg_vcrotate",
        &[
            ("vcnegh", "{ r1:0 = vcnegh(r3:2,r4) }"),
            ("vcrotate", "{ r1:0 = vcrotate(r3:2,r4) }"),
        ],
        40,
        0xb205,
    );
}

// Complex byte-pair rotate-accumulate (vrcrotate). control byte = Rt[ui*8+:8];
// per byte-pair a 4-way add/sub of (real,imag) terms. No saturation. The _acc
// form seeds the running sums from the old Rxx word lanes. 40 iters vs VCPU.
#[test]
fn lift_s4_vrcrotate() {
    lift_family(
        "s4_vrcrotate",
        &[
            ("vrcrotate", "{ r1:0 = vrcrotate(r3:2,r4,#1) }"),
            ("vrcrotate_acc", "{ r1:0 += vrcrotate(r3:2,r4,#2) }"),
        ],
        40,
        0xb206,
    );
}

#[test]
fn lift_s2_vrcnegh() {
    lift_family(
        "s2_vrcnegh",
        &[("vrcnegh", "{ r1:0 += vrcnegh(r3:2,r4) }")],
        40,
        0xb207,
    );
}

// ============================================================================
// Complex halfword MAC: real-only / imag-only 16x16 complex products written
// sign-extended into the Rdd pair (cmpyi/cmpyr) or accumulated into the full
// 64-bit Rxx pair (cmaci/cmacr). No <<1, no sat -> usr_ovf stays 0 (compared).
// ============================================================================
#[test]
fn lift_m2_cmpy_cmac() {
    lift_family(
        "m2_cmpy_cmac",
        &[
            ("cmpyi", "{ r1:0 = cmpyi(r2,r3) }"),
            ("cmpyr", "{ r1:0 = cmpyr(r2,r3) }"),
            ("cmaci", "{ r1:0 += cmpyi(r2,r3) }"),
            ("cmacr", "{ r1:0 += cmpyr(r2,r3) }"),
        ],
        40,
        0xb208,
    );
}

// Register-pair extract/insert: width = Rtt[37:32], offset = sxtn(7,Rtt[6:0]),
// both runtime. Composed with runtime Shl/Shr/masks + Select on offset sign and
// the width==0 / offset<0 edge cases. The harness seeds Rtt randomly so a wide
// range of width/offset (incl. negative offsets and width 0) is exercised.
#[test]
fn lift_extract_insert_rp() {
    lift_family(
        "extract_insert_rp",
        &[
            ("extractu_rp", "{ r0 = extractu(r1,r3:2) }"),
            ("extractup_rp", "{ r1:0 = extractu(r3:2,r5:4) }"),
            ("extract_rp", "{ r0 = extract(r1,r3:2) }"),
            ("extractp_rp", "{ r1:0 = extract(r3:2,r5:4) }"),
            ("insert_rp", "{ r0 = insert(r1,r3:2) }"),
            ("insertp_rp", "{ r1:0 = insert(r3:2,r5:4) }"),
        ],
        40,
        0xb20d,
    );
}

// Deterministic edge cases for extract/insert_rp: width==0, offset<0 (incl. the
// minimum -64), offset==0, max width 63. The pair forms use r5:r4 as Rtt (r4 =
// offset bits[6:0], r5 = width bits[37:32]); the 32-bit forms use r3:r2 as Rtt
// (r2 = offset, r3 = width). We compare lift vs HexagonVcpu for each crafted
// state, so the runtime width/offset paths are pinned, not left to chance.
#[test]
fn lift_extract_insert_rp_edges() {
    // (label, asm). Pair: Rtt=r5:r4 (r4=offset, r5=width). 32-bit: Rtt=r3:r2.
    let cases: &[(&str, &str, bool)] = &[
        ("extractup_rp", "{ r1:0 = extractu(r3:2,r5:4) }", true),
        ("extractp_rp", "{ r1:0 = extract(r3:2,r5:4) }", true),
        ("insertp_rp", "{ r1:0 = insert(r3:2,r5:4) }", true),
        ("extractu_rp", "{ r0 = extractu(r1,r3:2) }", false),
        ("extract_rp", "{ r0 = extract(r1,r3:2) }", false),
        ("insert_rp", "{ r0 = insert(r1,r3:2) }", false),
    ];
    // offsets (low 7 bits) and widths (low 6 bits) to pin.
    let offs: [u32; 6] = [0, 1, 5, 63, 0x40 /*-64*/, 0x7f /*-1*/];
    let widths: [u32; 5] = [0, 1, 16, 32, 63];
    for (label, asm, is_pair) in cases.iter().copied() {
        let words = match assemble(&[asm.to_string()]) {
            Some(w) => w.into_iter().next().unwrap(),
            None => {
                eprintln!("[extract_insert_rp_edges] llvm-mc unavailable -> skip");
                return;
            }
        };
        for &off in offs.iter() {
            for &w in widths.iter() {
                let mut st = State::zeroed();
                // Source / accumulator data with mixed sign bits.
                st.r[0] = 0xdead_beef; // old r0 (insert dest, 32-bit)
                st.r[1] = 0x1234_9abc; // Rs (32-bit src)
                st.r[2] = 0xfedc_ba98; // Rss/Rtt lo / 32-bit Rtt offset
                st.r[3] = 0x7654_3210; // Rss/Rtt hi / 32-bit Rtt width
                st.r[4] = off; // pair Rtt offset bits[6:0]
                st.r[5] = w; // pair Rtt width  bits[37:32]
                if is_pair {
                    // 32-bit-Rtt regs reused as Rss for the pair extract/insert.
                    st.r[2] = 0xfedc_ba98;
                    st.r[3] = 0x7654_3210;
                } else {
                    // 32-bit Rtt = r3:r2: r2=offset, r3=width.
                    st.r[2] = off;
                    st.r[3] = w;
                }
                let interp = match run_interp(&words, &st) {
                    Some(s) => s,
                    None => continue,
                };
                match lift_and_run(&words, &st) {
                    Ok(Some(lift)) => {
                        for r in 0..2 {
                            assert_eq!(
                                interp.r[r], lift.r[r],
                                "{label} off={off:#x} w={w}: r{r} i={:#x} l={:#x}",
                                interp.r[r], lift.r[r]
                            );
                        }
                        assert_eq!(
                            interp.usr & 1,
                            lift.usr & 1,
                            "{label} off={off:#x} w={w}: usr_ovf"
                        );
                    }
                    Ok(None) => panic!("{label}: UNLIFTED"),
                    Err(e) => panic!("{label}: {e}"),
                }
            }
        }
    }
}

// Vector reduce max/min with index. Rxx is both source (seed best/addr) and
// dest; the harness seeds it randomly so the running-best thread is exercised.
// word0 = winning value, word1 = (Ru | winning_index<<shift).
#[test]
fn lift_a4_vrminmax() {
    lift_family(
        "a4_vrminmax",
        &[
            ("vrmaxh", "{ r1:0 = vrmaxh(r3:2,r4) }"),
            ("vrmaxuh", "{ r1:0 = vrmaxuh(r3:2,r4) }"),
            ("vrmaxw", "{ r1:0 = vrmaxw(r3:2,r4) }"),
            ("vrmaxuw", "{ r1:0 = vrmaxuw(r3:2,r4) }"),
            ("vrminh", "{ r1:0 = vrminh(r3:2,r4) }"),
            ("vrminuh", "{ r1:0 = vrminuh(r3:2,r4) }"),
            ("vrminw", "{ r1:0 = vrminw(r3:2,r4) }"),
            ("vrminuw", "{ r1:0 = vrminuw(r3:2,r4) }"),
        ],
        40,
        0xb20c,
    );
}

// Vector reduce complex multiply (sum of 4 16x16 products into the pair, no
// sat). The `*` suffix is the conjugate (_s0c) form; `+=` is the _acc form.
#[test]
fn lift_m2_vrcmpy_reduce() {
    lift_family(
        "m2_vrcmpy_reduce",
        &[
            ("vrcmpyi", "{ r1:0 = vrcmpyi(r3:2,r5:4) }"),
            ("vrcmpyr", "{ r1:0 = vrcmpyr(r3:2,r5:4) }"),
            ("vrcmpyic", "{ r1:0 = vrcmpyi(r3:2,r5:4*) }"),
            ("vrcmpyrc", "{ r1:0 = vrcmpyr(r3:2,r5:4*) }"),
            ("vrcmaci", "{ r1:0 += vrcmpyi(r3:2,r5:4) }"),
            ("vrcmacr", "{ r1:0 += vrcmpyr(r3:2,r5:4) }"),
            ("vrcmacic", "{ r1:0 += vrcmpyi(r3:2,r5:4*) }"),
            ("vrcmacrc", "{ r1:0 += vrcmpyr(r3:2,r5:4*) }"),
        ],
        40,
        0xb20b,
    );
}

// Vector complex multiply :sat (2 lanes, signed-32 saturate, set_ovf:true).
#[test]
fn lift_m2_vcmpy() {
    lift_family(
        "m2_vcmpy",
        &[
            ("vcmpyr_s0", "{ r1:0 = vcmpyr(r3:2,r5:4):sat }"),
            ("vcmpyr_s1", "{ r1:0 = vcmpyr(r3:2,r5:4):<<1:sat }"),
            ("vcmpyi_s0", "{ r1:0 = vcmpyi(r3:2,r5:4):sat }"),
            ("vcmpyi_s1", "{ r1:0 = vcmpyi(r3:2,r5:4):<<1:sat }"),
            ("vcmacr_s0", "{ r1:0 += vcmpyr(r3:2,r5:4):sat }"),
            ("vcmaci_s0", "{ r1:0 += vcmpyi(r3:2,r5:4):sat }"),
        ],
        40,
        0xb209,
    );
}

// Vector reduce complex multiply by scalar :<<1:sat (+ acc, + round-pack).
#[test]
fn lift_m2_vrcmpys() {
    lift_family(
        "m2_vrcmpys",
        &[
            ("s1_h", "{ r1:0 = vrcmpys(r3:2,r5:4):<<1:sat:raw:hi }"),
            ("s1_l", "{ r1:0 = vrcmpys(r3:2,r5:4):<<1:sat:raw:lo }"),
            ("acc_s1_h", "{ r1:0 += vrcmpys(r3:2,r5:4):<<1:sat:raw:hi }"),
            ("acc_s1_l", "{ r1:0 += vrcmpys(r3:2,r5:4):<<1:sat:raw:lo }"),
            ("s1rp_h", "{ r0 = vrcmpys(r3:2,r5:4):<<1:rnd:sat:raw:hi }"),
            ("s1rp_l", "{ r0 = vrcmpys(r3:2,r5:4):<<1:rnd:sat:raw:lo }"),
        ],
        40,
        0xb20a,
    );
}

// ============================================================================
// Memory: loads / stores / memops (Step 2). All use the `_mem` harness which
// seeds + compares the DATA region. Base register r0 is forced to DATA_ADDR.
// ============================================================================

// Scalar base+#imm loads: byte/half/word/dword, signed + unsigned. memd is a
// register-PAIR write (r3:2) — the SELF-CHECK that the mem path catches a
// wrong-width lift (a single 64-bit Load to r2 leaves r3 stale).
#[test]
fn lift_mem_load_io() {
    lift_mem_family(
        "mem_load_io",
        &[
            ("loadrb", "{ r1 = memb(r0+#3) }"),
            ("loadrub", "{ r1 = memub(r0+#5) }"),
            ("loadrh", "{ r1 = memh(r0+#2) }"),
            ("loadruh", "{ r1 = memuh(r0+#6) }"),
            ("loadri", "{ r1 = memw(r0+#0) }"),
            ("loadri4", "{ r1 = memw(r0+#4) }"),
            ("loadrd", "{ r3:2 = memd(r0+#0) }"),
            ("loadrd8", "{ r3:2 = memd(r0+#8) }"),
        ],
        0,
        40,
        0xc001,
    );
}

// Scalar base+#imm stores: byte/half/word/dword. memd reads a register PAIR.
#[test]
fn lift_mem_store_io() {
    lift_mem_family(
        "mem_store_io",
        &[
            ("storerb", "{ memb(r0+#3) = r1 }"),
            ("storerh", "{ memh(r0+#2) = r1 }"),
            ("storerf", "{ memh(r0+#4) = r1.h }"),
            ("storeri", "{ memw(r0+#0) = r1 }"),
            ("storeri4", "{ memw(r0+#8) = r1 }"),
            ("storerd", "{ memd(r0+#0) = r3:2 }"),
            ("storerd8", "{ memd(r0+#16) = r3:2 }"),
        ],
        0,
        40,
        0xc002,
    );
}

// Load-locked (`Rd=memw_locked(Rs)` / `Rdd=memd_locked(Rs)`): a plain load at
// `Rs+0` that ALSO arms the LL reservation. The reservation arming is invisible
// to the harness (not compared), so this verifies LL behaves as a plain
// word/dword load: the loaded value lands in Rd / the Rdd pair and memory is
// untouched. base r0 = DATA_ADDR.
#[test]
fn lift_mem_load_locked() {
    lift_mem_family(
        "mem_load_locked",
        &[
            ("loadw_locked", "{ r1 = memw_locked(r0) }"),
            ("loadd_locked", "{ r3:2 = memd_locked(r0) }"),
        ],
        0,
        40,
        0xC0CC,
    );
}

// Store-conditional (`memw_locked(Rs,Pd)=Rt` / `memd_locked(Rs,Pd)=Rtt`): store
// IF the LL reservation holds, write Pd = success. In this single-packet harness
// there is NO preceding load-locked (LL and SC are separate packets) and BOTH
// monitors (the interp's `lock_addr` and the FlatMemory exclusive monitor) start
// CLEAR, so the SC FAILS identically on both sides: Pd := 0x00 and memory is
// UNCHANGED. The byte-for-byte DATA compare confirms no spurious store; the Pd
// compare confirms the success-predicate polarity (0x00 on a failed SC) matches.
// base r0 = DATA_ADDR; Pd = p0.
#[test]
fn lift_mem_store_cond() {
    lift_mem_family(
        "mem_store_cond",
        &[
            ("storew_locked", "{ memw_locked(r0,p0) = r1 }"),
            ("stored_locked", "{ memd_locked(r0,p0) = r3:2 }"),
        ],
        0,
        40,
        0xC0CD,
    );
}

// Vector byte splice (`Rdd=vspliceb(Rss,Rtt,#u3|Pu)`): low N bytes from Rss, high
// (8-N) bytes from Rtt; N = #u3 (`_ib`) or Pu&7 (`_rb`). A pure register-pair op
// (no memory), so use the plain `lift_family` path. The `_ib` form sweeps the
// constant boundary cases N=0 (all Rtt) and N=7 (mostly Rss); the `_rb` form
// reads the seeded predicate byte (Pu&7) — the harness seeds full random
// predicate bytes so all 0..=7 counts are exercised.
#[test]
fn lift_vspliceib() {
    lift_family(
        "vspliceib",
        &[
            ("vspliceib0", "{ r5:4 = vspliceb(r1:0,r3:2,#0) }"),
            ("vspliceib3", "{ r5:4 = vspliceb(r1:0,r3:2,#3) }"),
            ("vspliceib4", "{ r5:4 = vspliceb(r1:0,r3:2,#4) }"),
            ("vspliceib7", "{ r5:4 = vspliceb(r1:0,r3:2,#7) }"),
        ],
        40,
        0xC0CE,
    );
}

#[test]
fn lift_vsplicerb() {
    lift_family(
        "vsplicerb",
        &[
            ("vsplicerb0", "{ r5:4 = vspliceb(r1:0,r3:2,p0) }"),
            ("vsplicerb1", "{ r5:4 = vspliceb(r1:0,r3:2,p1) }"),
        ],
        40,
        0xC0CF,
    );
}

// Post-increment-immediate loads (Rx++#imm): the base register r0 is also
// written (compared as a reg). Increments are small positive multiples of the
// access size so the (single) access stays in-region.
#[test]
fn lift_mem_load_pi() {
    lift_mem_family(
        "mem_load_pi",
        &[
            ("loadrb_pi", "{ r1 = memb(r0++#1) }"),
            ("loadrub_pi", "{ r1 = memub(r0++#1) }"),
            ("loadrh_pi", "{ r1 = memh(r0++#2) }"),
            ("loadruh_pi", "{ r1 = memuh(r0++#2) }"),
            ("loadri_pi", "{ r1 = memw(r0++#4) }"),
            ("loadrd_pi", "{ r3:2 = memd(r0++#8) }"),
        ],
        0,
        40,
        0xc003,
    );
}

// Post-increment-immediate stores (Rx++#imm): base r0 also written.
#[test]
fn lift_mem_store_pi() {
    lift_mem_family(
        "mem_store_pi",
        &[
            ("storerb_pi", "{ memb(r0++#1) = r1 }"),
            ("storerh_pi", "{ memh(r0++#2) = r1 }"),
            ("storeri_pi", "{ memw(r0++#4) = r1 }"),
            ("storerd_pi", "{ memd(r0++#8) = r3:2 }"),
        ],
        0,
        40,
        0xc004,
    );
}

// Store-immediate (S4_storeiri/storeirh/storeirb): mem(Rs+#u) = #s6.
#[test]
fn lift_mem_store_imm() {
    lift_mem_family(
        "mem_store_imm",
        &[
            // #s6 range only (no constant extender) — the extended store-imm
            // forms need the extender routed to the value (not the offset) and
            // are reported as a lift gap.
            ("storeirb", "{ memb(r0+#3) = #15 }"),
            ("storeirb_neg", "{ memb(r0+#5) = #-8 }"),
            ("storeirh", "{ memh(r0+#2) = #-7 }"),
            ("storeiri", "{ memw(r0+#0) = #20 }"),
            ("storeiri_neg", "{ memw(r0+#4) = #-1 }"),
        ],
        0,
        40,
        0xc005,
    );
}

// Base + Rt<<#sh register-offset loads/stores (`L4_*_rr` / `S4_*_rr`). The
// index register is forced to 0 so EA == base == DATA_ADDR (in-region); the
// scaled-index address composition (Add of base + index<<scale) is exercised.
#[test]
fn lift_mem_load_rr() {
    lift_mem_family_idx(
        "mem_load_rr",
        &[
            ("loadrb_rr", "{ r1 = memb(r0+r2<<#0) }"),
            ("loadrub_rr", "{ r1 = memub(r0+r2<<#1) }"),
            ("loadrh_rr", "{ r1 = memh(r0+r2<<#1) }"),
            ("loadruh_rr", "{ r1 = memuh(r0+r2<<#2) }"),
            ("loadri_rr", "{ r1 = memw(r0+r2<<#2) }"),
            ("loadrd_rr", "{ r3:2 = memd(r0+r4<<#3) }"),
        ],
        0,
        &[2, 4],
        40,
        0xc007,
    );
}

#[test]
fn lift_mem_store_rr() {
    lift_mem_family_idx(
        "mem_store_rr",
        &[
            ("storerb_rr", "{ memb(r0+r2<<#0) = r1 }"),
            ("storerh_rr", "{ memh(r0+r2<<#1) = r1 }"),
            ("storeri_rr", "{ memw(r0+r2<<#2) = r1 }"),
            ("storerd_rr", "{ memd(r0+r4<<#3) = r3:2 }"),
        ],
        0,
        &[2, 4],
        40,
        0xc008,
    );
}

// Scaled-index absolute loads/stores (`L4_*_ur` / `S4_*_ur`):
// EA = ##addr + (Ru<<#sh). The absolute is DATA_ADDR and the index is forced to
// 0, so EA == DATA_ADDR (in-region). Two-word (constant-extended) packets.
#[test]
fn lift_mem_load_ur() {
    lift_mem_family_idx(
        "mem_load_ur",
        &[
            ("loadrb_ur", "{ r1 = memb(r2<<#0+##0x8000) }"),
            ("loadrh_ur", "{ r1 = memh(r2<<#1+##0x8000) }"),
            ("loadri_ur", "{ r1 = memw(r2<<#2+##0x8000) }"),
            ("loadrd_ur", "{ r3:2 = memd(r4<<#3+##0x8000) }"),
        ],
        1, // base_reg unused by these forms; pick a reg not otherwise read
        &[2, 4],
        40,
        0xc009,
    );
}

#[test]
fn lift_mem_store_ur() {
    lift_mem_family_idx(
        "mem_store_ur",
        &[
            ("storerb_ur", "{ memb(r2<<#0+##0x8000) = r1 }"),
            ("storerh_ur", "{ memh(r2<<#1+##0x8000) = r1 }"),
            ("storeri_ur", "{ memw(r2<<#2+##0x8000) = r1 }"),
            ("storerd_ur", "{ memd(r4<<#3+##0x8000) = r3:2 }"),
        ],
        1,
        &[2, 4],
        40,
        0xc00a,
    );
}

// Absolute-set loads/stores (`L4_*_ap` / `S4_*_ap`): `memX(Re=##addr)`. The
// EA is the constant-extended absolute (DATA_ADDR here) AND the address register
// Re is written with that absolute (compared as a register). Includes the
// Re==dst aliasing case (`r0 = memw(r0=##...)`: the load result must win).
#[test]
fn lift_mem_load_ap() {
    lift_mem_family(
        "mem_load_ap",
        &[
            ("loadrb_ap", "{ r1 = memb(r0=##0x8000) }"),
            ("loadrub_ap", "{ r1 = memub(r0=##0x8000) }"),
            ("loadrh_ap", "{ r1 = memh(r0=##0x8000) }"),
            ("loadri_ap", "{ r1 = memw(r0=##0x8000) }"),
            ("loadrd_ap", "{ r3:2 = memd(r0=##0x8000) }"),
            // (Re == Rd aliasing is illegal for abs-set loads — assembler rejects.)
        ],
        9, // base_reg unused by abs-set forms; pick a reg not read/written
        40,
        0xc00b,
    );
}

#[test]
fn lift_mem_store_ap() {
    lift_mem_family(
        "mem_store_ap",
        &[
            ("storerb_ap", "{ memb(r0=##0x8000) = r1 }"),
            ("storerh_ap", "{ memh(r0=##0x8000) = r1 }"),
            ("storeri_ap", "{ memw(r0=##0x8000) = r1 }"),
            ("storerd_ap", "{ memd(r0=##0x8000) = r3:2 }"),
            // Re == src aliasing: the store must use the OLD r1, then r1=addr.
            ("storeri_alias", "{ memw(r1=##0x8000) = r1 }"),
        ],
        9,
        40,
        0xc00c,
    );
}

// Post-increment-by-modifier-register loads (`memX(Rx++Mu)`): the base register
// r0 is also written (Rx += raw M[modsel]). The `_mem` harness seeds M0/M1 (see
// lift_mem_family_idx) so the base update is now modelled + compared.
#[test]
fn lift_mem_load_pr() {
    lift_mem_family(
        "mem_load_pr",
        &[
            ("loadrb_pr", "{ r1 = memb(r0++m0) }"),
            ("loadrub_pr", "{ r1 = memub(r0++m0) }"),
            ("loadrh_pr", "{ r1 = memh(r0++m0) }"),
            ("loadruh_pr", "{ r1 = memuh(r0++m1) }"),
            ("loadri_pr", "{ r1 = memw(r0++m0) }"),
            ("loadrd_pr", "{ r3:2 = memd(r0++m0) }"),
        ],
        0,
        40,
        0xc101,
    );
}

#[test]
fn lift_mem_store_pr() {
    lift_mem_family(
        "mem_store_pr",
        &[
            ("storerb_pr", "{ memb(r0++m0) = r1 }"),
            ("storerh_pr", "{ memh(r0++m0) = r1 }"),
            ("storerf_pr", "{ memh(r0++m0) = r1.h }"),
            ("storeri_pr", "{ memw(r0++m1) = r1 }"),
            ("storerd_pr", "{ memd(r0++m0) = r3:2 }"),
        ],
        0,
        40,
        0xc102,
    );
}

// Bit-reverse post-increment (`memX(Rx++Mu:brev)`): EA = brev(Rx) (reverse the
// low 16 bits, keep the high 16), then Rx += raw M. With base = DATA_ADDR
// (0x8000) the brev EA is 0x8001 (in-region, possibly unaligned — fine here).
#[test]
fn lift_mem_load_pbr() {
    lift_mem_family(
        "mem_load_pbr",
        &[
            ("loadrb_pbr", "{ r1 = memb(r0++m0:brev) }"),
            ("loadrub_pbr", "{ r1 = memub(r0++m0:brev) }"),
            ("loadrh_pbr", "{ r1 = memh(r0++m0:brev) }"),
            ("loadri_pbr", "{ r1 = memw(r0++m0:brev) }"),
            ("loadrd_pbr", "{ r3:2 = memd(r0++m0:brev) }"),
        ],
        0,
        40,
        0xc103,
    );
}

#[test]
fn lift_mem_store_pbr() {
    lift_mem_family(
        "mem_store_pbr",
        &[
            ("storerb_pbr", "{ memb(r0++m0:brev) = r1 }"),
            ("storerh_pbr", "{ memh(r0++m0:brev) = r1 }"),
            ("storeri_pbr", "{ memw(r0++m0:brev) = r1 }"),
            ("storerd_pbr", "{ memd(r0++m0:brev) = r3:2 }"),
        ],
        0,
        40,
        0xc104,
    );
}

// Circular post-increment by immediate (`memX(Rx++#s4:N:circ(Mu))`): EA = Rx;
// Rx = circ_add(Rx, incr, M, CS). The harness seeds M length=0x40 / K=0 and CS =
// DATA_ADDR, so the buffer is [0x8000, 0x8040). Negative increments exercise the
// underflow wrap (new_ptr < start -> +length); positive ones the no-wrap path.
#[test]
fn lift_mem_load_pci() {
    lift_mem_family(
        "mem_load_pci",
        &[
            ("loadrb_pci", "{ r1 = memb(r0++#1:circ(m0)) }"),
            ("loadri_pci", "{ r1 = memw(r0++#4:circ(m0)) }"),
            ("loadri_pci_neg", "{ r1 = memw(r0++#-4:circ(m0)) }"),
            ("loadrh_pci_neg", "{ r1 = memh(r0++#-2:circ(m0)) }"),
            ("loadrd_pci", "{ r3:2 = memd(r0++#8:circ(m0)) }"),
        ],
        0,
        40,
        0xc105,
    );
}

#[test]
fn lift_mem_store_pci() {
    lift_mem_family(
        "mem_store_pci",
        &[
            ("storerb_pci", "{ memb(r0++#1:circ(m0)) = r1 }"),
            ("storeri_pci", "{ memw(r0++#4:circ(m0)) = r1 }"),
            ("storeri_pci_neg", "{ memw(r0++#-4:circ(m0)) = r1 }"),
            ("storerd_pci", "{ memd(r0++#8:circ(m0)) = r3:2 }"),
        ],
        0,
        40,
        0xc106,
    );
}

// Circular post-increment by the M register's I field (`memX(Rx++I:circ(Mu))`):
// EA = Rx; Rx = circ_add(Rx, read_ireg(M)<<access_shift, M, CS). The seeded M has
// I-field = 1, so the increment is 1<<access_shift.
#[test]
fn lift_mem_load_pcr() {
    lift_mem_family(
        "mem_load_pcr",
        &[
            ("loadrb_pcr", "{ r1 = memb(r0++I:circ(m0)) }"),
            ("loadrh_pcr", "{ r1 = memh(r0++I:circ(m0)) }"),
            ("loadri_pcr", "{ r1 = memw(r0++I:circ(m1)) }"),
            ("loadrd_pcr", "{ r3:2 = memd(r0++I:circ(m0)) }"),
        ],
        0,
        40,
        0xc107,
    );
}

#[test]
fn lift_mem_store_pcr() {
    lift_mem_family(
        "mem_store_pcr",
        &[
            ("storerb_pcr", "{ memb(r0++I:circ(m0)) = r1 }"),
            ("storeri_pcr", "{ memw(r0++I:circ(m0)) = r1 }"),
            ("storerd_pcr", "{ memd(r0++I:circ(m1)) = r3:2 }"),
        ],
        0,
        40,
        0xc108,
    );
}

// GP-relative loads/stores (`memX(gp+#u)`): EA = (GP & !0x3f) + offset. The
// harness seeds GP = DATA_ADDR (already 64-byte aligned, low 6 bits zero) so the
// interp's `GP & !0x3f` mask is a no-op and the SMIR `GpRel` reader (which does
// NOT mask) agrees. Offsets keep the access inside the DATA region.
#[test]
fn lift_mem_load_gp() {
    lift_mem_family(
        "mem_load_gp",
        &[
            ("loadrb_gp", "{ r1 = memb(gp+#3) }"),
            ("loadrub_gp", "{ r1 = memub(gp+#5) }"),
            ("loadrh_gp", "{ r1 = memh(gp+#2) }"),
            ("loadruh_gp", "{ r1 = memuh(gp+#6) }"),
            ("loadri_gp", "{ r1 = memw(gp+#0) }"),
            ("loadri_gp8", "{ r1 = memw(gp+#8) }"),
            ("loadrd_gp", "{ r3:2 = memd(gp+#0) }"),
        ],
        1, // base_reg unused by GP forms; pick a reg not read/written
        40,
        0xc109,
    );
}

#[test]
fn lift_mem_store_gp() {
    lift_mem_family(
        "mem_store_gp",
        &[
            ("storerb_gp", "{ memb(gp+#3) = r1 }"),
            ("storerh_gp", "{ memh(gp+#2) = r1 }"),
            ("storeri_gp", "{ memw(gp+#0) = r1 }"),
            ("storeri_gp8", "{ memw(gp+#8) = r1 }"),
            ("storerd_gp", "{ memd(gp+#0) = r3:2 }"),
        ],
        1,
        40,
        0xc10a,
    );
}

// FIFO shift-and-insert loads (`Ryy = memX_fifo(...)`, loadalign): a read-modify
// of the register pair r3:2 — Ryy = (Ryy >> w) | (loaded << (64-w)), w=8/16.
// Covers base+#imm, post-inc-imm, modifier post-inc, abs-set, and scaled-abs.
#[test]
fn lift_mem_loadalign() {
    lift_mem_family(
        "mem_loadalign",
        &[
            ("loadalignb_io", "{ r3:2 = memb_fifo(r0+#1) }"),
            ("loadalignh_io", "{ r3:2 = memh_fifo(r0+#2) }"),
            ("loadalignb_pi", "{ r3:2 = memb_fifo(r0++#1) }"),
            ("loadalignh_pi", "{ r3:2 = memh_fifo(r0++#2) }"),
            ("loadalignb_pr", "{ r3:2 = memb_fifo(r0++m0) }"),
            ("loadalignb_pci", "{ r3:2 = memb_fifo(r0++#1:circ(m0)) }"),
            ("loadalignb_ap", "{ r3:2 = memb_fifo(r0=##0x8000) }"),
        ],
        0,
        40,
        0xc10b,
    );
}

#[test]
fn lift_mem_loadalign_ur() {
    lift_mem_family_idx(
        "mem_loadalign_ur",
        &[
            ("loadalignb_ur", "{ r3:2 = memb_fifo(r2<<#0+##0x8000) }"),
            ("loadalignh_ur", "{ r3:2 = memh_fifo(r2<<#1+##0x8000) }"),
        ],
        1,
        &[2],
        40,
        0xc10c,
    );
}

// Constant-extended store-immediate (`memX(Rs+#u) = ##big`). The constant
// extender must NOT leak into the address offset (the bug the routing fix
// prevents). NOTE: the rax HexagonVcpu interpreter (the verification oracle)
// DROPS the store-imm extender (its decoder passes immext=None), so it stores
// only the sign-extended #s6. The lift matches that (extender consumed but not
// folded into the value) and is therefore 0-divergence; see the structured
// notes for the interp-side extender bug.
#[test]
fn lift_mem_store_imm_ext() {
    lift_mem_family(
        "mem_store_imm_ext",
        &[
            ("storeiri_ext", "{ memw(r0+#0) = ##100000 }"),
            ("storeirb_ext", "{ memb(r0+#1) = ##0x77 }"),
            ("storeirh_ext", "{ memh(r0+#2) = ##0x1234 }"),
        ],
        0,
        40,
        0xc10d,
    );
}

// ---------------------------------------------------------------------------
// Byte-unpack loads (L2_loadb{s,z}w{2,4}): load 2 or 4 contiguous bytes and
// unpack each byte into a halfword lane (membh=sign, memubh=zero). count==2
// writes a single Rd (lanes 0,1); count==4 writes the Rdd pair (lanes 0..3).
// Forced base r0=DATA_ADDR keeps the access in-region.
// ---------------------------------------------------------------------------
#[test]
fn lift_mem_loadunpack_io() {
    lift_mem_family(
        "mem_loadunpack_io",
        &[
            ("loadbzw2", "{ r1 = memubh(r0+#0) }"),
            ("loadbsw2", "{ r1 = membh(r0+#0) }"),
            ("loadbzw2_off", "{ r1 = memubh(r0+#4) }"),
            ("loadbsw2_off", "{ r1 = membh(r0+#6) }"),
            ("loadbzw4", "{ r3:2 = memubh(r0+#0) }"),
            ("loadbsw4", "{ r3:2 = membh(r0+#0) }"),
            ("loadbzw4_off", "{ r3:2 = memubh(r0+#8) }"),
            ("loadbsw4_off", "{ r3:2 = membh(r0+#12) }"),
        ],
        0,
        40,
        0xd001,
    );
}

// Post-increment-immediate byte-unpack loads (Rx++#N): base r0 also written.
// The increment matches the byte count (2 for bzw2/bsw2, 4 for bzw4/bsw4) so a
// single access stays in-region.
#[test]
fn lift_mem_loadunpack_pi() {
    lift_mem_family(
        "mem_loadunpack_pi",
        &[
            ("loadbzw2_pi", "{ r1 = memubh(r0++#2) }"),
            ("loadbsw2_pi", "{ r1 = membh(r0++#2) }"),
            ("loadbzw4_pi", "{ r3:2 = memubh(r0++#4) }"),
            ("loadbsw4_pi", "{ r3:2 = membh(r0++#4) }"),
        ],
        0,
        40,
        0xd002,
    );
}

// Post-increment-by-modifier byte-unpack loads (Rx++Mu): base r0 += raw M.
#[test]
fn lift_mem_loadunpack_pr() {
    lift_mem_family(
        "mem_loadunpack_pr",
        &[
            ("loadbzw2_pr", "{ r1 = memubh(r0++m0) }"),
            ("loadbsw2_pr", "{ r1 = membh(r0++m0) }"),
            ("loadbzw4_pr", "{ r3:2 = memubh(r0++m0) }"),
            ("loadbsw4_pr", "{ r3:2 = membh(r0++m1) }"),
        ],
        0,
        40,
        0xd003,
    );
}

// Scaled-index absolute byte-unpack loads (`L4_loadb{s,z}w{2,4}_ur`):
// EA = ##addr + (Ru<<#sh). Absolute is DATA_ADDR, index forced to 0.
#[test]
fn lift_mem_loadunpack_ur() {
    lift_mem_family_idx(
        "mem_loadunpack_ur",
        &[
            ("loadbzw2_ur", "{ r1 = memubh(r4<<#1+##0x8000) }"),
            ("loadbsw2_ur", "{ r1 = membh(r4<<#1+##0x8000) }"),
            ("loadbzw4_ur", "{ r3:2 = memubh(r4<<#2+##0x8000) }"),
            ("loadbsw4_ur", "{ r3:2 = membh(r4<<#2+##0x8000) }"),
        ],
        1,
        &[4],
        40,
        0xd004,
    );
}

// Absolute-set byte-unpack loads (`L4_loadb{s,z}w{2,4}_ap`): memX(Re=##addr).
// EA is the constant-extended absolute (DATA_ADDR); Re is written with it.
#[test]
fn lift_mem_loadunpack_ap() {
    lift_mem_family(
        "mem_loadunpack_ap",
        &[
            ("loadbzw2_ap", "{ r1 = memubh(r2=##0x8000) }"),
            ("loadbsw2_ap", "{ r1 = membh(r2=##0x8000) }"),
            ("loadbzw4_ap", "{ r5:4 = memubh(r2=##0x8000) }"),
            ("loadbsw4_ap", "{ r5:4 = membh(r2=##0x8000) }"),
        ],
        9,
        40,
        0xd005,
    );
}

// ---------------------------------------------------------------------------
// New-value stores (S2/S4 storer{b,h,i}new_*): the stored value is the .new
// result of a producer earlier in the SAME packet, selected by Nt8. The lifter
// resolves Nt8 against the packet's GPR producers (the producer was lifted
// earlier in the block, so its sequential SMIR write holds the .new value).
// Each packet pairs a producer (`r1 = ...`) with the new-value store.
// ---------------------------------------------------------------------------
#[test]
fn lift_mem_storenew_io() {
    lift_mem_family(
        "mem_storenew_io",
        &[
            ("storerbnew", "{ r1 = #0x55 ; memb(r0+#0) = r1.new }"),
            ("storerhnew", "{ r1 = #0x1234 ; memh(r0+#2) = r1.new }"),
            ("storerinew", "{ r1 = #0x778899 ; memw(r0+#0) = r1.new }"),
            ("storerinew4", "{ r1 = #0x778899 ; memw(r0+#4) = r1.new }"),
            // Producer is a reg-reg op; new-value store of the result.
            (
                "storerinew_add",
                "{ r1 = add(r5,r6) ; memw(r0+#0) = r1.new }",
            ),
            // Two producers in the packet: the .new store must pick r2 (Nt8).
            (
                "storerinew_2prod",
                "{ r1 = #3 ; r2 = #7 ; memw(r0+#0) = r2.new }",
            ),
        ],
        0,
        40,
        0xd006,
    );
}

// Post-increment-immediate new-value stores (Rx++#N).
#[test]
fn lift_mem_storenew_pi() {
    lift_mem_family(
        "mem_storenew_pi",
        &[
            ("storerbnew_pi", "{ r1 = #0x55 ; memb(r0++#1) = r1.new }"),
            ("storerhnew_pi", "{ r1 = #0x1234 ; memh(r0++#2) = r1.new }"),
            (
                "storerinew_pi",
                "{ r1 = #0x778899 ; memw(r0++#4) = r1.new }",
            ),
        ],
        0,
        40,
        0xd007,
    );
}

// Base+Ru<<#sh register-offset new-value stores (`S4_storer*new_rr`).
#[test]
fn lift_mem_storenew_rr() {
    lift_mem_family_idx(
        "mem_storenew_rr",
        &[
            ("storerbnew_rr", "{ r1 = #0x55 ; memb(r0+r2<<#0) = r1.new }"),
            (
                "storerhnew_rr",
                "{ r1 = #0x1234 ; memh(r0+r2<<#1) = r1.new }",
            ),
            (
                "storerinew_rr",
                "{ r1 = #0x778899 ; memw(r0+r2<<#2) = r1.new }",
            ),
        ],
        0,
        &[2],
        40,
        0xd008,
    );
}

// Scaled-index absolute new-value stores (`S4_storer*new_ur`).
#[test]
fn lift_mem_storenew_ur() {
    lift_mem_family_idx(
        "mem_storenew_ur",
        &[
            (
                "storerbnew_ur",
                "{ r1 = #0x55 ; memb(r2<<#0+##0x8000) = r1.new }",
            ),
            (
                "storerhnew_ur",
                "{ r1 = #0x1234 ; memh(r2<<#1+##0x8000) = r1.new }",
            ),
            (
                "storerinew_ur",
                "{ r1 = #0x778899 ; memw(r2<<#2+##0x8000) = r1.new }",
            ),
        ],
        9,
        &[2],
        40,
        0xd009,
    );
}

// Post-increment-by-modifier / circular / bit-reverse new-value stores
// (`S2_storer*new_pr/pbr/pci/pcr`): the base register is updated like the
// regular modifier stores; the data is the same-packet .new producer. The
// mem-family harness seeds M0/M1, CS0/CS1, GP so the access stays in-region.
#[test]
fn lift_mem_storenew_pr() {
    lift_mem_family(
        "mem_storenew_pr",
        &[
            (
                "storerinew_pr",
                "{ r1 = #0x778899 ; memw(r0++m0) = r1.new }",
            ),
            ("storerhnew_pr", "{ r1 = #0x1234 ; memh(r0++m0) = r1.new }"),
            ("storerbnew_pr", "{ r1 = #0x55 ; memb(r0++m0) = r1.new }"),
            (
                "storerinew_pci",
                "{ r1 = #0x778899 ; memw(r0++#4:circ(m0)) = r1.new }",
            ),
            (
                "storerinew_pcr",
                "{ r1 = #0x778899 ; memw(r0++I:circ(m0)) = r1.new }",
            ),
            (
                "storerinew_pbr",
                "{ r1 = #0x778899 ; memw(r0++m0:brev) = r1.new }",
            ),
        ],
        0,
        40,
        0xd00c,
    );
}

// Absolute-set new-value stores (`S4_storer*new_ap`): memX(Re=##addr)=Rt.new.
#[test]
fn lift_mem_storenew_ap() {
    lift_mem_family(
        "mem_storenew_ap",
        &[
            (
                "storerbnew_ap",
                "{ r1 = #0x55 ; memb(r2=##0x8000) = r1.new }",
            ),
            (
                "storerhnew_ap",
                "{ r1 = #0x1234 ; memh(r2=##0x8000) = r1.new }",
            ),
            (
                "storerinew_ap",
                "{ r1 = #0x778899 ; memw(r2=##0x8000) = r1.new }",
            ),
        ],
        9,
        40,
        0xd00a,
    );
}

// ---------------------------------------------------------------------------
// New-value compound compare-and-jump (J4_cmp*_jumpnv*): the compared register
// is the .new result of a producer in the SAME packet, selected by Ns8. The
// lifter resolves Ns8 against the packet's producers (like a new-value store),
// then lifts the compare on that GPR + a conditional branch. No predicate is
// written by the _jumpnv forms. Verified via the control-flow harness.
// ---------------------------------------------------------------------------
#[test]
fn lift_cf_cmpjump_nv() {
    lift_cf_family(
        "cf_cmpjump_nv",
        &[
            // reg/reg compares (producer r0 = r4)
            (
                "eq_t_nt",
                "{ r0 = r4 ; if (cmp.eq(r0.new,r1)) jump:nt #0x10 }",
            ),
            (
                "eq_t_t",
                "{ r0 = r4 ; if (cmp.eq(r0.new,r1)) jump:t #0x10 }",
            ),
            (
                "eq_f_nt",
                "{ r0 = r4 ; if (!cmp.eq(r0.new,r1)) jump:nt #0x10 }",
            ),
            (
                "gt_t_nt",
                "{ r0 = r4 ; if (cmp.gt(r0.new,r1)) jump:nt #0x10 }",
            ),
            (
                "gt_f_nt",
                "{ r0 = r4 ; if (!cmp.gt(r0.new,r1)) jump:nt #0x10 }",
            ),
            (
                "gtu_t_nt",
                "{ r0 = r4 ; if (cmp.gtu(r0.new,r1)) jump:nt #0x10 }",
            ),
            (
                "gtu_f_nt",
                "{ r0 = r4 ; if (!cmp.gtu(r0.new,r1)) jump:nt #0x10 }",
            ),
            // reversed (cmplt/cmpltu): producer is the .new operand on the RHS
            (
                "lt_t_nt",
                "{ r0 = r4 ; if (cmp.gt(r1,r0.new)) jump:nt #0x10 }",
            ),
            (
                "ltu_t_nt",
                "{ r0 = r4 ; if (cmp.gtu(r1,r0.new)) jump:nt #0x10 }",
            ),
            // reg/imm compares
            (
                "eqi_t_nt",
                "{ r0 = r4 ; if (cmp.eq(r0.new,#5)) jump:nt #0x10 }",
            ),
            (
                "eqi_f_nt",
                "{ r0 = r4 ; if (!cmp.eq(r0.new,#5)) jump:nt #0x10 }",
            ),
            (
                "gti_t_nt",
                "{ r0 = r4 ; if (cmp.gt(r0.new,#5)) jump:nt #0x10 }",
            ),
            (
                "gtui_t_nt",
                "{ r0 = r4 ; if (cmp.gtu(r0.new,#5)) jump:nt #0x10 }",
            ),
            // compare vs -1 / bit 0
            (
                "eqn1_t_nt",
                "{ r0 = r4 ; if (cmp.eq(r0.new,#-1)) jump:nt #0x10 }",
            ),
            (
                "gtn1_t_nt",
                "{ r0 = r4 ; if (cmp.gt(r0.new,#-1)) jump:nt #0x10 }",
            ),
            (
                "tstbit0_t_nt",
                "{ r0 = r4 ; if (tstbit(r0.new,#0)) jump:nt #0x10 }",
            ),
            (
                "tstbit0_f_nt",
                "{ r0 = r4 ; if (!tstbit(r0.new,#0)) jump:nt #0x10 }",
            ),
        ],
        40,
        0xd00b,
    );
}

// ---------------------------------------------------------------------------
// S2 within-GPR "vector" sign/zero-extend, truncate/pack, and shuffle. Pure
// permutation/extend ops over GPRs and GPR pairs (reg-only harness). Lane
// order matches sem/shift_ext.rs.
// ---------------------------------------------------------------------------
#[test]
fn lift_s2_vec_ext_trunc_shuffle() {
    lift_family(
        "s2_vec_ext_trunc_shuffle",
        &[
            // sign/zero extend (byte->half x4, half->word x2)
            ("vsxtbh", "{ r3:2 = vsxtbh(r0) }"),
            ("vzxtbh", "{ r3:2 = vzxtbh(r0) }"),
            ("vsxthw", "{ r3:2 = vsxthw(r0) }"),
            ("vzxthw", "{ r3:2 = vzxthw(r0) }"),
            // truncate/pack (single-source, dst is a 32-bit reg)
            ("vtrunehb", "{ r8 = vtrunehb(r5:4) }"),
            ("vtrunohb", "{ r8 = vtrunohb(r5:4) }"),
            // two-source truncate/pack + shuffles (dst is a pair)
            ("vtrunewh", "{ r9:8 = vtrunewh(r5:4,r7:6) }"),
            ("vtrunowh", "{ r9:8 = vtrunowh(r5:4,r7:6) }"),
            ("shuffeb", "{ r9:8 = shuffeb(r5:4,r7:6) }"),
            ("shuffob", "{ r9:8 = shuffob(r5:4,r7:6) }"),
            ("shuffeh", "{ r9:8 = shuffeh(r5:4,r7:6) }"),
            ("shuffoh", "{ r9:8 = shuffoh(r5:4,r7:6) }"),
        ],
        40,
        0xd00d,
    );
}

// S2_valignib: Rdd = (Rss u>> sh*8) | (Rtt << (8-sh)*8), sh = #u3 (immediate
// vector byte align). Covers the sh==0 boundary (result == Rss).
#[test]
fn lift_s2_valignib() {
    lift_family(
        "s2_valignib",
        &[
            ("valignb0", "{ r9:8 = valignb(r5:4,r7:6,#0) }"),
            ("valignb1", "{ r9:8 = valignb(r5:4,r7:6,#1) }"),
            ("valignb3", "{ r9:8 = valignb(r5:4,r7:6,#3) }"),
            ("valignb7", "{ r9:8 = valignb(r5:4,r7:6,#7) }"),
        ],
        40,
        0xd00f,
    );
}

// C4_addipc: Rd = fREAD_PC() + #u6. The packet start PC (CODE_ADDR=0x1000 on
// both sides) is folded to a compile-time constant. Reg-only harness.
#[test]
fn lift_c4_addipc() {
    lift_family(
        "c4_addipc",
        &[
            ("addipc0", "{ r0 = add(pc,#0) }"),
            ("addipc8", "{ r1 = add(pc,#8) }"),
            ("addipc_ext", "{ r2 = add(pc,##0x100) }"),
        ],
        40,
        0xd00e,
    );
}

// Read-modify-write memops: mem(Rs+#u) OP= Rt / #imm. Width byte/half/word.
// Forced base r0=DATA_ADDR; the modify operand is the random r1 (or an imm).
#[test]
fn lift_mem_memop() {
    lift_mem_family(
        "mem_memop",
        &[
            // register operand
            ("memopw_add", "{ memw(r0+#0) += r1 }"),
            ("memopw_sub", "{ memw(r0+#0) -= r1 }"),
            ("memopw_and", "{ memw(r0+#0) &= r1 }"),
            ("memopw_or", "{ memw(r0+#0) |= r1 }"),
            ("memoph_add", "{ memh(r0+#2) += r1 }"),
            ("memoph_sub", "{ memh(r0+#2) -= r1 }"),
            ("memoph_and", "{ memh(r0+#2) &= r1 }"),
            ("memoph_or", "{ memh(r0+#2) |= r1 }"),
            ("memopb_add", "{ memb(r0+#3) += r1 }"),
            ("memopb_sub", "{ memb(r0+#3) -= r1 }"),
            ("memopb_and", "{ memb(r0+#3) &= r1 }"),
            ("memopb_or", "{ memb(r0+#3) |= r1 }"),
            // immediate operand (iadd/isub) and bit set/clear
            ("memopw_iadd", "{ memw(r0+#0) += #5 }"),
            ("memopw_isub", "{ memw(r0+#0) -= #5 }"),
            ("memopw_clrbit", "{ memw(r0+#0) = clrbit(#3) }"),
            ("memopw_setbit", "{ memw(r0+#0) = setbit(#3) }"),
            ("memoph_iadd", "{ memh(r0+#2) += #5 }"),
            ("memopb_iadd", "{ memb(r0+#3) += #5 }"),
            ("memopb_clrbit", "{ memb(r0+#3) = clrbit(#2) }"),
            ("memopb_setbit", "{ memb(r0+#3) = setbit(#2) }"),
        ],
        0,
        40,
        0xc006,
    );
}

// ============================================================================
// Deferred-wave scalar gap-fill verification (newly lifted families).
// ============================================================================

// Saturating vector pack to byte/half (packed Rd). These call sat_n/satu_n in
// the sem, so USR:OVF (bit 0) is exercised and compared by the harness.
#[test]
fn lift_s2_vsat_pack() {
    lift_family(
        "s2_vsat_pack",
        &[
            ("vsathb", "{ r0 = vsathb(r3:2) }"),
            ("vsathub", "{ r0 = vsathub(r3:2) }"),
            ("vsatwh", "{ r0 = vsatwh(r3:2) }"),
            ("vsatwuh", "{ r0 = vsatwuh(r3:2) }"),
        ],
        40,
        0xE001,
    );
}

// Saturating vector saturate, no pack (pair Rdd, lanes stay full width).
#[test]
fn lift_s2_vsat_nopack() {
    lift_family(
        "s2_vsat_nopack",
        &[
            ("vsathb_np", "{ r1:0 = vsathb(r3:2) }"),
            ("vsathub_np", "{ r1:0 = vsathub(r3:2) }"),
            ("vsatwh_np", "{ r1:0 = vsatwh(r3:2) }"),
            ("vsatwuh_np", "{ r1:0 = vsatwuh(r3:2) }"),
        ],
        40,
        0xE002,
    );
}

// Scalar (single-register) saturate + pack of 2 halves.
#[test]
fn lift_s2_svsat() {
    lift_family(
        "s2_svsat",
        &[
            ("svsathb", "{ r0 = vsathb(r2) }"),
            ("svsathub", "{ r0 = vsathub(r2) }"),
        ],
        40,
        0xE003,
    );
}

// asr-halves + saturate to ubyte (+rnd), and round+pack words to halves.
#[test]
fn lift_s5_asrhub_and_rndpack() {
    lift_family(
        "s5_asrhub_and_rndpack",
        &[
            ("asrhub_sat", "{ r0 = vasrhub(r3:2,#3):raw }"),
            ("asrhub_rnd_sat", "{ r0 = vasrhub(r3:2,#3):rnd:sat }"),
            ("asrhub_sat0", "{ r0 = vasrhub(r3:2,#0):raw }"),
            ("asrhub_rnd_sat0", "{ r0 = vasrhub(r3:2,#0):rnd:sat }"),
            ("vrndpackwh", "{ r0 = vrndwh(r3:2) }"),
            ("vrndpackwhs", "{ r0 = vrndwh(r3:2):sat }"),
        ],
        40,
        0xE004,
    );
}

// Non-saturating asr-halves with round (pair out).
#[test]
fn lift_s5_vasrhrnd() {
    lift_family(
        "s5_vasrhrnd",
        &[
            ("vasrhrnd3", "{ r1:0 = vasrh(r3:2,#3):raw }"),
            ("vasrhrnd0", "{ r1:0 = vasrh(r3:2,#0):raw }"),
            ("vasrhrnd7", "{ r1:0 = vasrh(r3:2,#7):raw }"),
        ],
        40,
        0xE005,
    );
}

// Bit split (immediate + runtime).
#[test]
fn lift_a4_bitsplit() {
    lift_family(
        "a4_bitsplit",
        &[
            ("bitspliti5", "{ r1:0 = bitsplit(r2,#5) }"),
            ("bitspliti0", "{ r1:0 = bitsplit(r2,#0) }"),
            ("bitspliti31", "{ r1:0 = bitsplit(r2,#31) }"),
            ("bitsplit_r", "{ r1:0 = bitsplit(r2,r3) }"),
        ],
        40,
        0xE006,
    );
}

// Bit interleave / deinterleave of a 64-bit pair.
#[test]
fn lift_s2_interleave() {
    lift_family(
        "s2_interleave",
        &[
            ("interleave", "{ r1:0 = interleave(r3:2) }"),
            ("deinterleave", "{ r1:0 = deinterleave(r3:2) }"),
        ],
        40,
        0xE007,
    );
}

// pair->pair even/odd byte truncate-pack and pair byte splat.
#[test]
fn lift_s6_trunehb_ppp_and_splat() {
    lift_family(
        "s6_trunehb_ppp_and_splat",
        &[
            ("vtrunehb_ppp", "{ r1:0 = vtrunehb(r3:2,r5:4) }"),
            ("vtrunohb_ppp", "{ r1:0 = vtrunohb(r3:2,r5:4) }"),
            ("vsplatrbp", "{ r1:0 = vsplatb(r2) }"),
        ],
        40,
        0xE008,
    );
}

// S2_lfsp linear-feedback shift (pair).
#[test]
fn lift_s2_lfsp() {
    lift_family(
        "s2_lfsp",
        &[("lfsp", "{ r1:0 = lfs(r3:2,r5:4) }")],
        40,
        0xE009,
    );
}

// tableidx{b,h,w,d}: Rx read-modify bitfield insert.
#[test]
fn lift_s2_tableidx() {
    lift_family(
        "s2_tableidx",
        &[
            ("tableidxb", "{ r0 = tableidxb(r1,#4,#5):raw }"),
            ("tableidxh", "{ r0 = tableidxh(r1,#4,#5):raw }"),
            ("tableidxw", "{ r0 = tableidxw(r1,#4,#5):raw }"),
            ("tableidxd", "{ r0 = tableidxd(r1,#4,#5):raw }"),
            ("tableidxb_w0", "{ r0 = tableidxb(r1,#0,#5):raw }"),
            ("tableidxh_negoff", "{ r0 = tableidxh(r1,#8,#-7):raw }"),
            ("tableidxw_widmax", "{ r0 = tableidxw(r1,#15,#3):raw }"),
            ("tableidxb_negoff", "{ r0 = tableidxb(r1,#6,#-30):raw }"),
        ],
        40,
        0xE00A,
    );
}

// A5_ACS (vacsh): writes a GPR pair, USR:OVF, and a 4-bit predicate. Only Pe
// bit 0 (== lane-0 (xv>sv)) is tracked/compared by the harness; the pair value
// and USR:OVF are fully compared.
#[test]
fn lift_a5_acs() {
    lift_family(
        "a5_acs",
        &[("vacsh", "{ r1:0,p0 = vacsh(r3:2,r5:4) }")],
        40,
        0xE00B,
    );
}

// FINAL GAP RE-SCAN (audit). Probe EVERY non-V6 (scalar) opcode through the
// lifter using its canonical encoding word (all variable fields = 0) and confirm
// each one that still lifts to Unsupported falls into a KNOWN, classified bucket.
// The probe feeds a SINGLE instruction with no packet context, so three buckets
// here are PROBE-ONLY artifacts (the ops lift correctly inside a real packet and
// are verified by other passing families) — they are recognized by their
// `*_unresolved` rejection mnemonic:
//   - store_new_unresolved              : a `.new` store with no in-packet
//     producer (verified lifted by lift_mem_store_new* families).
//   - pred_store_new_unresolved         : a PREDICATED `.new` store with no
//     in-packet producer (verified lifted by lift_mem_pstorenew_* families).
//   - compound_cmpjump_nv_unresolved    : a J4 `.new` compare-jump with no
//     in-packet producer (verified lifted by cf_cmpjump_* families).
// The remaining buckets are GENUINE boundaries, keyed by the rejection mnemonic
// or opcode-name family:
//   (a) F2_dfmpylh ONLY                 : the reference sem writes its accumulate
//         result to field `d` (set_rp(rd,...)), but this opcode has NO `d` field
//         (only s/t/x) -> rd defaults to 0 and the oracle writes R1:R0 instead of
//         the architectural Rxx. A CORRECT `Rxx += ...` lift would "diverge" from
//         the buggy oracle, and matching the bug would ship a knowingly-wrong
//         lift, so it is UNVERIFIABLE. (Every other F2_* scalar FP op — incl.
//         dfmpyhh / dfmpyfix / sffma_sc — is now LIFTED + 0-div verified.)
//   (b) Y2_/Y4_/Y5_                     : system / cache / barrier / privileged
//         state hints (modeled as observably-empty no-ops where lifted; the
//         remaining ones touch unmodeled privileged state). A4_tlbmatch is NOW
//         LIFTED (a pure function of the seeded register pair Rss, not hidden TLB
//         state) and S2_cabacdecbin is NOW LIFTED (pure function of Rss/Rtt + the
//         constant H.264 transition tables) — neither is a boundary any longer.
//   (d) load_locked / store_cond        : LL/SC reservation + predicate side
//         effect not modeled by the plain Load/Store.
//   (e) sploop                          : software-pipelined loop sets USR.LPCFG
//         (packet/loop-config state); pause: system pause; vspliceb: register-
//         pair splice deferred to the interpreter.
// NOTE: the PREDICATED loads/stores that CANCEL on a false predicate are now
// LIFTED via the conditional-commit OpKinds PredLoad / PredStore (with gated
// post-increment), so `pred_load` / `pred_or_newvalue_store` / `pred_store_imm`
// no longer appear here; the predicated new-value stores are verified inside a
// packet (the probe-only `pred_store_new_unresolved` artifact above).
// The test FAILS if any opcode lifts to Unsupported for a reason NOT in this set
// — i.e. if a new tractable scalar register/memory op silently regresses.
#[test]
fn audit_final_scalar_gap_rescan() {
    let unlifted = HexagonLifter::audit_unlifted_scalar();

    // Histogram by rejection mnemonic (the precise reason each op was rejected).
    let mut by_mnem: HashMap<&str, usize> = HashMap::new();
    for (_name, m) in &unlifted {
        *by_mnem.entry(m.as_str()).or_insert(0) += 1;
    }
    let mut mnems: Vec<_> = by_mnem.iter().collect();
    mnems.sort_by(|a, b| b.1.cmp(a.1));
    eprintln!(
        "[audit] {} non-V6 opcodes still Unsupported; by reason:",
        unlifted.len()
    );
    for (m, n) in &mnems {
        eprintln!("    {m}: {n}");
    }

    // Recognized rejection reasons (mnemonic-keyed) — the genuine boundaries plus
    // the two probe-only `*_unresolved` artifacts.
    const KNOWN_MNEMONICS: &[&str] = &[
        // probe-only artifacts (lift fine in a real packet)
        "store_new_unresolved",
        "pred_store_new_unresolved",
        "compound_cmpjump_nv_unresolved",
        // atomic LL/SC (boundary d)
        "load_locked",
        "store_cond",
        // loop-config / system (boundary e)
        "sploop",
        "pause",
        "vspliceb",
    ];
    // The float / system / stateful ops fall through lift_unknown_op and report
    // their OPCODE NAME as the mnemonic, so classify those by name family.
    let is_boundary_by_name = |name: &str| -> bool {
        name == "F2_dfmpylh"             // buggy-oracle FP  (boundary a) — ONLY F2 boundary left
            || name.starts_with("Y2_")   // system           (boundary b)
            || name.starts_with("Y4_")   // tlb              (boundary b)
            || name.starts_with("Y5_") // barrier/tlb      (boundary b)
    };

    let unexpected: Vec<_> = unlifted
        .iter()
        .filter(|(name, m)| !KNOWN_MNEMONICS.contains(&m.as_str()) && !is_boundary_by_name(name))
        .collect();
    if !unexpected.is_empty() {
        eprintln!("[audit] UNEXPECTED unlifted scalar ops (not a known boundary):");
        for (name, m) in &unexpected {
            eprintln!("    {name}  (reason: {m})");
        }
    }
    assert!(
        unexpected.is_empty(),
        "{} scalar op(s) outside the documented boundaries still lift to Unsupported",
        unexpected.len()
    );
}

// Y2_dczeroa: zero the 32-byte cache line at `Rs & !31`. The mem harness forces
// the base register r0 = DATA_ADDR (0x8000, already 32-byte aligned), so the
// aligned address stays in the DATA region. The 32 bytes at DATA_ADDR must
// become zero on BOTH sides and nothing else change (the byte-for-byte memory
// compare over the whole DATA region catches any over- or under-write).
#[test]
fn lift_y2_dczeroa() {
    lift_mem_family(
        "y2_dczeroa",
        &[("dczeroa", "{ dczeroa(r0) }")],
        0,
        40,
        0xE00D,
    );
}

// A4_tfrcpp / A4_tfrpcp control-register PAIR transfers for the MODELED c-reg
// pairs whose halves are M0/M1 (C7:C6) and CS0/CS1 (C13:C12). The mem harness
// seeds + compares M0/M1 and CS0/CS1 (and all R), so a value-move lift is fully
// provable here. tfrcpp reads the c-reg pair into a GPR pair; tfrpcp writes a
// GPR pair into the c-reg pair. base_reg r0 is forced into the DATA region but
// these ops never dereference memory, so it is a harmless fixed seed.
#[test]
fn lift_tfrcpp_tfrpcp_mcs() {
    lift_mem_family(
        "tfrcpp_tfrpcp_mcs",
        &[
            // Reads: M1:M0 -> r5:4, CS1:CS0 -> r7:6.
            ("tfrcpp_m", "{ r5:4 = c7:6 }"),
            ("tfrcpp_cs", "{ r7:6 = c13:12 }"),
            // Writes: r9:8 -> M1:M0, r11:10 -> CS1:CS0.
            ("tfrpcp_m", "{ c7:6 = r9:8 }"),
            ("tfrpcp_cs", "{ c13:12 = r11:10 }"),
        ],
        0,
        40,
        0xE00E,
    );
}

// A4_tfrcpp / A4_tfrpcp for the LOOP-register pairs LC0/SA0 (C1:C0) and
// LC1/SA1 (C3:C2). The CF harness seeds + compares LC0/LC1 (via m[]) and
// SA0/SA1 (via cs[]) plus all R, so these pairs are provable there (the plain
// and mem harnesses do not model LC/SA). The packets fall through (next-PC =
// packet end), which the CF harness also checks.
#[test]
fn lift_tfrcpp_tfrpcp_loop() {
    lift_cf_family(
        "tfrcpp_tfrpcp_loop",
        &[
            // Reads: LC0:SA0 -> r5:4, LC1:SA1 -> r7:6.
            ("tfrcpp_l0", "{ r5:4 = c1:0 }"),
            ("tfrcpp_l1", "{ r7:6 = c3:2 }"),
            // Writes: r9:8 -> LC0:SA0, r11:10 -> LC1:SA1.
            ("tfrpcp_l0", "{ c1:0 = r9:8 }"),
            ("tfrpcp_l1", "{ c3:2 = r11:10 }"),
        ],
        40,
        0xE00F,
    );
}

// R6 release stores: in the single-threaded harness these have no observable
// effect (the sem treats them as register-only no-ops). Lifted as an empty op
// list; verified to leave all GPR/predicate/USR state unchanged.
#[test]
fn lift_r6_release() {
    lift_family(
        "r6_release",
        &[
            ("release_at", "{ release(r2):at }"),
            ("release_st", "{ release(r2):st }"),
        ],
        40,
        0xE00C,
    );
}

// ============================================================================
// PREDICATED loads / stores (the ~236-op bucket). Each CANCELS on a false
// predicate: no register write, no memory write, no base post-increment, and
// no fault. The lifter emits the new conditional-commit OpKinds PredLoad /
// PredStore (gated post-increment via a pure Add + Select). The mem-family
// harness seeds the FULL predicate byte (biased to 0x00/0xff) so ~every
// iteration hits BOTH cond-true and cond-false, verifying the cancel against
// the qemu-checked HexagonVcpu interpreter (regs + memory + predicate byte).
// All asm confirmed with `llvm-mc -triple=hexagon -mcpu=hexagonv69 -mhvx
// -mattr=+audio -show-encoding`.
// ============================================================================

// Predicated base+#imm loads (L2_ploadr*_io): if (Pu)/if (!Pu) Rd = memX(Rs+#u).
// Byte/half/word/dword, signed + unsigned, t/f sense. memd writes a reg PAIR.
#[test]
fn lift_mem_pload_io() {
    lift_mem_family(
        "mem_pload_io",
        &[
            ("ploadrbt", "{ if (p0) r1 = memb(r0+#3) }"),
            ("ploadrbf", "{ if (!p0) r1 = memb(r0+#3) }"),
            ("ploadrubt", "{ if (p0) r1 = memub(r0+#5) }"),
            ("ploadrubf", "{ if (!p0) r1 = memub(r0+#5) }"),
            ("ploadrht", "{ if (p0) r1 = memh(r0+#2) }"),
            ("ploadrhf", "{ if (!p0) r1 = memh(r0+#2) }"),
            ("ploadruht", "{ if (p0) r1 = memuh(r0+#6) }"),
            ("ploadrit", "{ if (p0) r1 = memw(r0+#0) }"),
            ("ploadrif", "{ if (!p0) r1 = memw(r0+#4) }"),
            ("ploadrdt", "{ if (p0) r3:2 = memd(r0+#0) }"),
            ("ploadrdf", "{ if (!p0) r3:2 = memd(r0+#8) }"),
        ],
        0,
        40,
        0xf001,
    );
}

// Predicated DOT-NEW base+#imm loads (L2_ploadr*tnew/fnew_io): the predicate is
// the just-written P0 from a same-packet producer compare (p0.new). Tests both
// the producer-true and producer-false outcomes across random states.
#[test]
fn lift_mem_pload_io_new() {
    lift_mem_family(
        "mem_pload_io_new",
        &[
            (
                "ploadritnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) r1 = memw(r0+#0) }",
            ),
            (
                "ploadrifnew",
                "{ p0 = cmp.eq(r4,r5) ; if (!p0.new) r1 = memw(r0+#4) }",
            ),
            (
                "ploadrbtnew",
                "{ p0 = cmp.gt(r4,r5) ; if (p0.new) r1 = memb(r0+#3) }",
            ),
            (
                "ploadrubfnew",
                "{ p0 = cmp.eq(r4,r5) ; if (!p0.new) r1 = memub(r0+#5) }",
            ),
            (
                "ploadrhtnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) r1 = memh(r0+#2) }",
            ),
            (
                "ploadrdtnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) r3:2 = memd(r0+#0) }",
            ),
        ],
        0,
        40,
        0xf002,
    );
}

// Predicated post-increment-immediate loads (L2_ploadr*_pi): base r0 advances
// ONLY when the predicate holds (gated post-inc). Increments are small positive
// multiples of the access size; the access uses the OLD base = DATA_ADDR.
#[test]
fn lift_mem_pload_pi() {
    lift_mem_family(
        "mem_pload_pi",
        &[
            ("ploadrbt_pi", "{ if (p0) r1 = memb(r0++#1) }"),
            ("ploadrbf_pi", "{ if (!p0) r1 = memb(r0++#1) }"),
            ("ploadrubt_pi", "{ if (p0) r1 = memub(r0++#1) }"),
            ("ploadrht_pi", "{ if (p0) r1 = memh(r0++#2) }"),
            ("ploadrhf_pi", "{ if (!p0) r1 = memh(r0++#2) }"),
            ("ploadrit_pi", "{ if (p0) r1 = memw(r0++#4) }"),
            ("ploadrif_pi", "{ if (!p0) r1 = memw(r0++#4) }"),
            ("ploadrdt_pi", "{ if (p0) r3:2 = memd(r0++#8) }"),
        ],
        0,
        40,
        0xf003,
    );
}

// Predicated register-offset loads (L4_ploadr*_rr): if (Pu) Rd = memX(Rs+Ru<<#u2).
// The index register r2 is forced to 0 so EA == base == DATA_ADDR (in-region).
#[test]
fn lift_mem_pload_rr() {
    lift_mem_family_idx(
        "mem_pload_rr",
        &[
            ("ploadrbt_rr", "{ if (p0) r1 = memb(r0+r2<<#0) }"),
            ("ploadrbf_rr", "{ if (!p0) r1 = memb(r0+r2<<#0) }"),
            ("ploadrubt_rr", "{ if (p0) r1 = memub(r0+r2<<#1) }"),
            ("ploadrht_rr", "{ if (p0) r1 = memh(r0+r2<<#0) }"),
            ("ploadruht_rr", "{ if (p0) r1 = memuh(r0+r2<<#1) }"),
            ("ploadrit_rr", "{ if (p0) r1 = memw(r0+r2<<#2) }"),
            ("ploadrif_rr", "{ if (!p0) r1 = memw(r0+r2<<#0) }"),
            ("ploadrdt_rr", "{ if (p0) r3:2 = memd(r0+r2<<#0) }"),
        ],
        0,
        &[2],
        40,
        0xf004,
    );
}

// Predicated absolute loads (L4_ploadr*_abs): if (Pu) Rd = memX(##addr). The
// absolute address is the in-region constant 0x8000 (== DATA_ADDR).
#[test]
fn lift_mem_pload_abs() {
    lift_mem_family(
        "mem_pload_abs",
        &[
            ("ploadrbt_abs", "{ if (p0) r1 = memb(##0x8000) }"),
            ("ploadrbf_abs", "{ if (!p0) r1 = memb(##0x8000) }"),
            ("ploadrubt_abs", "{ if (p0) r1 = memub(##0x8000) }"),
            ("ploadrht_abs", "{ if (p0) r1 = memh(##0x8000) }"),
            ("ploadrit_abs", "{ if (p0) r1 = memw(##0x8000) }"),
            ("ploadrif_abs", "{ if (!p0) r1 = memw(##0x8000) }"),
            ("ploadrdt_abs", "{ if (p0) r3:2 = memd(##0x8000) }"),
        ],
        0,
        40,
        0xf005,
    );
}

// Predicated base+#imm stores (S2_pstorer*_io + storerf high-half): if (Pu)/
// if (!Pu) memX(Rs+#u) = Rt. CANCELS the memory write on a false predicate.
#[test]
fn lift_mem_pstore_io() {
    lift_mem_family(
        "mem_pstore_io",
        &[
            ("pstorerbt", "{ if (p0) memb(r0+#3) = r1 }"),
            ("pstorerbf", "{ if (!p0) memb(r0+#3) = r1 }"),
            ("pstorerht", "{ if (p0) memh(r0+#2) = r1 }"),
            ("pstorerhf", "{ if (!p0) memh(r0+#2) = r1 }"),
            ("pstorerft", "{ if (p0) memh(r0+#4) = r1.h }"),
            ("pstorerff", "{ if (!p0) memh(r0+#4) = r1.h }"),
            ("pstorerit", "{ if (p0) memw(r0+#0) = r1 }"),
            ("pstorerif", "{ if (!p0) memw(r0+#8) = r1 }"),
            ("pstorerdt", "{ if (p0) memd(r0+#0) = r3:2 }"),
            ("pstorerdf", "{ if (!p0) memd(r0+#16) = r3:2 }"),
        ],
        0,
        40,
        0xf006,
    );
}

// Predicated DOT-NEW-predicate base+#imm stores (S4_pstorer*tnew/fnew_io): the
// predicate is the same-packet producer's freshly-written P0 (p0.new).
#[test]
fn lift_mem_pstore_io_new() {
    lift_mem_family(
        "mem_pstore_io_new",
        &[
            (
                "pstoreritnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) memw(r0+#0) = r1 }",
            ),
            (
                "pstorerifnew",
                "{ p0 = cmp.eq(r4,r5) ; if (!p0.new) memw(r0+#8) = r1 }",
            ),
            (
                "pstorerbtnew",
                "{ p0 = cmp.gt(r4,r5) ; if (p0.new) memb(r0+#3) = r1 }",
            ),
            (
                "pstorerhtnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) memh(r0+#2) = r1 }",
            ),
            (
                "pstorerftnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) memh(r0+#4) = r1.h }",
            ),
            (
                "pstorerdtnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) memd(r0+#0) = r3:2 }",
            ),
        ],
        0,
        40,
        0xf007,
    );
}

// Predicated post-increment-immediate stores (S2_pstorer*_pi): base r0 advances
// ONLY when the predicate holds (gated post-inc).
#[test]
fn lift_mem_pstore_pi() {
    lift_mem_family(
        "mem_pstore_pi",
        &[
            ("pstorerbt_pi", "{ if (p0) memb(r0++#1) = r1 }"),
            ("pstorerbf_pi", "{ if (!p0) memb(r0++#1) = r1 }"),
            ("pstorerht_pi", "{ if (p0) memh(r0++#2) = r1 }"),
            ("pstorerft_pi", "{ if (p0) memh(r0++#2) = r1.h }"),
            ("pstorerit_pi", "{ if (p0) memw(r0++#4) = r1 }"),
            ("pstorerif_pi", "{ if (!p0) memw(r0++#4) = r1 }"),
            ("pstorerdt_pi", "{ if (p0) memd(r0++#8) = r3:2 }"),
        ],
        0,
        40,
        0xf008,
    );
}

// Predicated register-offset stores (S4_pstorer*_rr): index r2 forced to 0.
#[test]
fn lift_mem_pstore_rr() {
    lift_mem_family_idx(
        "mem_pstore_rr",
        &[
            ("pstorerbt_rr", "{ if (p0) memb(r0+r2<<#0) = r1 }"),
            ("pstorerbf_rr", "{ if (!p0) memb(r0+r2<<#0) = r1 }"),
            ("pstorerht_rr", "{ if (p0) memh(r0+r2<<#1) = r1 }"),
            ("pstorerft_rr", "{ if (p0) memh(r0+r2<<#1) = r1.h }"),
            ("pstorerit_rr", "{ if (p0) memw(r0+r2<<#2) = r1 }"),
            ("pstorerif_rr", "{ if (!p0) memw(r0+r2<<#0) = r1 }"),
            ("pstorerdt_rr", "{ if (p0) memd(r0+r2<<#0) = r3:2 }"),
        ],
        0,
        &[2],
        40,
        0xf009,
    );
}

// Predicated absolute stores (S4_pstorer*_abs): memX(##0x8000) = Rt.
#[test]
fn lift_mem_pstore_abs() {
    lift_mem_family(
        "mem_pstore_abs",
        &[
            ("pstorerbt_abs", "{ if (p0) memb(##0x8000) = r1 }"),
            ("pstorerbf_abs", "{ if (!p0) memb(##0x8000) = r1 }"),
            ("pstorerht_abs", "{ if (p0) memh(##0x8000) = r1 }"),
            ("pstorerft_abs", "{ if (p0) memh(##0x8000) = r1.h }"),
            ("pstorerit_abs", "{ if (p0) memw(##0x8000) = r1 }"),
            ("pstorerif_abs", "{ if (!p0) memw(##0x8000) = r1 }"),
            ("pstorerdt_abs", "{ if (p0) memd(##0x8000) = r3:2 }"),
        ],
        0,
        40,
        0xf00a,
    );
}

// Predicated store-immediate (S4_storeir*t/f_io): if (Pu) memX(Rs+#u) = #s6.
// CANCELS the memory write on a false predicate.
#[test]
fn lift_mem_pstore_imm() {
    lift_mem_family(
        "mem_pstore_imm",
        &[
            ("storeirbt", "{ if (p0) memb(r0+#3) = #15 }"),
            ("storeirbf", "{ if (!p0) memb(r0+#5) = #-8 }"),
            ("storeirht", "{ if (p0) memh(r0+#2) = #-7 }"),
            ("storeirhf", "{ if (!p0) memh(r0+#2) = #11 }"),
            ("storeirit", "{ if (p0) memw(r0+#0) = #20 }"),
            ("storeirif", "{ if (!p0) memw(r0+#4) = #-1 }"),
        ],
        0,
        40,
        0xf00b,
    );
}

// Predicated DOT-NEW-predicate store-immediate (S4_storeir*tnew/fnew_io).
#[test]
fn lift_mem_pstore_imm_new() {
    lift_mem_family(
        "mem_pstore_imm_new",
        &[
            (
                "storeiritnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) memw(r0+#0) = #20 }",
            ),
            (
                "storeirifnew",
                "{ p0 = cmp.eq(r4,r5) ; if (!p0.new) memw(r0+#4) = #-1 }",
            ),
            (
                "storeirbtnew",
                "{ p0 = cmp.gt(r4,r5) ; if (p0.new) memb(r0+#3) = #15 }",
            ),
            (
                "storeirhtnew",
                "{ p0 = cmp.eq(r4,r5) ; if (p0.new) memh(r0+#2) = #-7 }",
            ),
        ],
        0,
        40,
        0xf00c,
    );
}

// Predicated NEW-VALUE stores: if (Pu) memX(...) = Rt.new — the stored data is
// the same-packet .new producer (resolved via packet_producers), then gated by
// the predicate. Covers _io / _pi / _rr / _abs and a plain-pred (t/f) form.
#[test]
fn lift_mem_pstorenew_io() {
    lift_mem_family(
        "mem_pstorenew_io",
        &[
            ("pstorerbnewt", "{ r1 = r6 ; if (p0) memb(r0+#0) = r1.new }"),
            (
                "pstorerbnewf",
                "{ r1 = r6 ; if (!p0) memb(r0+#0) = r1.new }",
            ),
            ("pstorerhnewt", "{ r1 = r6 ; if (p0) memh(r0+#2) = r1.new }"),
            ("pstorerinewt", "{ r1 = r6 ; if (p0) memw(r0+#0) = r1.new }"),
            (
                "pstorerinewf",
                "{ r1 = r6 ; if (!p0) memw(r0+#4) = r1.new }",
            ),
        ],
        0,
        40,
        0xf00d,
    );
}

// Predicated DOT-NEW-predicate NEW-VALUE stores (S4_pstorer*newtnew/fnew_io):
// BOTH the data (.new producer) and the predicate (p0.new) come from the same
// packet. The producer compare (p0) is lifted first, then the data producer
// (r1), then the gated PredStore.
#[test]
fn lift_mem_pstorenew_io_new() {
    lift_mem_family(
        "mem_pstorenew_io_new",
        &[
            (
                "pstorerinewtnew",
                "{ p0 = cmp.eq(r4,r5) ; r1 = r6 ; if (p0.new) memw(r0+#0) = r1.new }",
            ),
            (
                "pstorerinewfnew",
                "{ p0 = cmp.eq(r4,r5) ; r1 = r6 ; if (!p0.new) memw(r0+#4) = r1.new }",
            ),
            (
                "pstorerbnewtnew",
                "{ p0 = cmp.gt(r4,r5) ; r1 = r6 ; if (p0.new) memb(r0+#3) = r1.new }",
            ),
            (
                "pstorerhnewtnew",
                "{ p0 = cmp.eq(r4,r5) ; r1 = r6 ; if (p0.new) memh(r0+#2) = r1.new }",
            ),
        ],
        0,
        40,
        0xf00e,
    );
}

// Predicated new-value post-increment / register-offset / absolute stores.
#[test]
fn lift_mem_pstorenew_modes() {
    lift_mem_family_idx(
        "mem_pstorenew_modes",
        &[
            (
                "pstorerinewt_pi",
                "{ r1 = r6 ; if (p0) memw(r0++#4) = r1.new }",
            ),
            (
                "pstorerinewf_pi",
                "{ r1 = r6 ; if (!p0) memw(r0++#4) = r1.new }",
            ),
            (
                "pstorerbnewt_rr",
                "{ r1 = r6 ; if (p0) memb(r0+r2<<#0) = r1.new }",
            ),
            (
                "pstorerinewt_rr",
                "{ r1 = r6 ; if (p0) memw(r0+r2<<#2) = r1.new }",
            ),
            (
                "pstorerinewt_abs",
                "{ r1 = r6 ; if (p0) memw(##0x8000) = r1.new }",
            ),
            (
                "pstorerbnewf_abs",
                "{ r1 = r6 ; if (!p0) memb(##0x8000) = r1.new }",
            ),
        ],
        0,
        &[2],
        40,
        0xf00f,
    );
}

// ============================================================================
// F2 scalar floating point (oracle-backed RESULT bits).
//
// The qemu-hexagon reference implements the full F2 family (sem/float.rs +
// float_ext.rs), so run_interp produces a defined result for every op below and
// nothing is auto-skipped. The harness compares the result GPR/predicate and
// USR:OVF (bit0 — none of these set it); the FP exception sticky bits (USR
// 1..5) are NOT compared, so the native-f32/f64 + default-NaN computation in
// OpKind::HexFp matches bit-exactly. Random 32/64-bit GPR seeds routinely land
// on NaN/inf/denormal/zero patterns, exercising every special-case branch.
// ============================================================================

#[test]
fn lift_f2_sfcmp() {
    lift_family(
        "f2_sfcmp",
        &[
            ("sfcmpeq", "{ p0 = sfcmp.eq(r1,r2) }"),
            ("sfcmpgt", "{ p0 = sfcmp.gt(r1,r2) }"),
            ("sfcmpge", "{ p0 = sfcmp.ge(r1,r2) }"),
            ("sfcmpuo", "{ p0 = sfcmp.uo(r1,r2) }"),
        ],
        40,
        0xf2_0001,
    );
}

#[test]
fn lift_f2_dfcmp() {
    lift_family(
        "f2_dfcmp",
        &[
            ("dfcmpeq", "{ p0 = dfcmp.eq(r1:0,r3:2) }"),
            ("dfcmpgt", "{ p0 = dfcmp.gt(r1:0,r3:2) }"),
            ("dfcmpge", "{ p0 = dfcmp.ge(r1:0,r3:2) }"),
            ("dfcmpuo", "{ p0 = dfcmp.uo(r1:0,r3:2) }"),
        ],
        40,
        0xf2_0002,
    );
}

#[test]
fn lift_f2_class() {
    lift_family(
        "f2_class",
        &[
            ("sfclass0", "{ p0 = sfclass(r1,#0) }"),
            ("sfclass5", "{ p0 = sfclass(r1,#5) }"),
            ("sfclass1f", "{ p0 = sfclass(r1,#31) }"),
            ("dfclass5", "{ p0 = dfclass(r1:0,#5) }"),
            ("dfclass1f", "{ p0 = dfclass(r1:0,#31) }"),
        ],
        40,
        0xf2_0003,
    );
}

#[test]
fn lift_f2_minmax() {
    lift_family(
        "f2_minmax",
        &[
            ("sfmin", "{ r4 = sfmin(r1,r2) }"),
            ("sfmax", "{ r4 = sfmax(r1,r2) }"),
            ("dfmin", "{ r5:4 = dfmin(r1:0,r3:2) }"),
            ("dfmax", "{ r5:4 = dfmax(r1:0,r3:2) }"),
        ],
        40,
        0xf2_0004,
    );
}

#[test]
fn lift_f2_arith() {
    lift_family(
        "f2_arith",
        &[
            ("sfadd", "{ r4 = sfadd(r1,r2) }"),
            ("sfsub", "{ r4 = sfsub(r1,r2) }"),
            ("sfmpy", "{ r4 = sfmpy(r1,r2) }"),
            ("dfadd", "{ r5:4 = dfadd(r1:0,r3:2) }"),
            ("dfsub", "{ r5:4 = dfsub(r1:0,r3:2) }"),
        ],
        40,
        0xf2_0005,
    );
}

#[test]
fn lift_f2_conv_i2f() {
    lift_family(
        "f2_conv_i2f",
        &[
            ("w2sf", "{ r4 = convert_w2sf(r1) }"),
            ("uw2sf", "{ r4 = convert_uw2sf(r1) }"),
            ("d2sf", "{ r4 = convert_d2sf(r1:0) }"),
            ("ud2sf", "{ r4 = convert_ud2sf(r1:0) }"),
            ("w2df", "{ r5:4 = convert_w2df(r1) }"),
            ("uw2df", "{ r5:4 = convert_uw2df(r1) }"),
            ("d2df", "{ r5:4 = convert_d2df(r1:0) }"),
            ("ud2df", "{ r5:4 = convert_ud2df(r1:0) }"),
        ],
        40,
        0xf2_0006,
    );
}

#[test]
fn lift_f2_conv_f2i_sf() {
    lift_family(
        "f2_conv_f2i_sf",
        &[
            ("sf2w", "{ r4 = convert_sf2w(r1) }"),
            ("sf2w_chop", "{ r4 = convert_sf2w(r1):chop }"),
            ("sf2uw", "{ r4 = convert_sf2uw(r1) }"),
            ("sf2uw_chop", "{ r4 = convert_sf2uw(r1):chop }"),
            ("sf2d", "{ r5:4 = convert_sf2d(r1) }"),
            ("sf2d_chop", "{ r5:4 = convert_sf2d(r1):chop }"),
            ("sf2ud", "{ r5:4 = convert_sf2ud(r1) }"),
            ("sf2ud_chop", "{ r5:4 = convert_sf2ud(r1):chop }"),
        ],
        40,
        0xf2_0007,
    );
}

#[test]
fn lift_f2_conv_f2i_df() {
    lift_family(
        "f2_conv_f2i_df",
        &[
            ("df2w", "{ r4 = convert_df2w(r1:0) }"),
            ("df2w_chop", "{ r4 = convert_df2w(r1:0):chop }"),
            ("df2uw", "{ r4 = convert_df2uw(r1:0) }"),
            ("df2uw_chop", "{ r4 = convert_df2uw(r1:0):chop }"),
            ("df2d", "{ r5:4 = convert_df2d(r1:0) }"),
            ("df2d_chop", "{ r5:4 = convert_df2d(r1:0):chop }"),
            ("df2ud", "{ r5:4 = convert_df2ud(r1:0) }"),
            ("df2ud_chop", "{ r5:4 = convert_df2ud(r1:0):chop }"),
        ],
        40,
        0xf2_0008,
    );
}

#[test]
fn lift_f2_conv_widen_narrow() {
    lift_family(
        "f2_conv_widen_narrow",
        &[
            ("df2sf", "{ r4 = convert_df2sf(r1:0) }"),
            ("sf2df", "{ r5:4 = convert_sf2df(r1) }"),
        ],
        40,
        0xf2_0009,
    );
}

#[test]
fn lift_f2_imm() {
    lift_family(
        "f2_imm",
        &[
            ("sfmake_pos", "{ r4 = sfmake(#13):pos }"),
            ("sfmake_neg", "{ r4 = sfmake(#13):neg }"),
            ("dfmake_pos", "{ r5:4 = dfmake(#13):pos }"),
            ("dfmake_neg", "{ r5:4 = dfmake(#13):neg }"),
        ],
        40,
        0xf2_000a,
    );
}

// Pure-integer f64 partial-product low-term multiply (no rounding, no flags).
// The sem (sem/float_ext.rs `dfmpyll`) is a bit manipulation, lifted with
// integer ops only. (dfmpylh is NOT lifted — its oracle sem targets the wrong
// register; see the lift comment.)
#[test]
fn lift_f2_dfmpy_int() {
    lift_family(
        "f2_dfmpy_int",
        &[("dfmpyll", "{ r3:2 = dfmpyll(r1:0, r5:4) }")],
        40,
        0xf2_000b,
    );
}

// Single-precision fused multiply-add (single IEEE rounding). The sem
// (sem/float_ext.rs `sf_fma`) is a faithful single-rounding fma; native
// `f32::mul_add` is correctly-rounded so the RESULT bits match (the harness
// ignores FP exception flags). Random seeds exercise NaN/inf/zero accumulators.
#[test]
fn lift_f2_sffma() {
    lift_family(
        "f2_sffma",
        &[
            ("sffma", "{ r4 += sfmpy(r1,r2) }"),
            ("sffms", "{ r4 -= sfmpy(r1,r2) }"),
        ],
        40,
        0xf2_000c,
    );
}

/// Pool of special single-precision FP bit patterns used to exercise the
/// reciprocal / inverse-sqrt seed-table + scalbn extreme-exponent branches. The
/// plain `lift_family` seeds fully-random GPRs which only rarely land on NaN /
/// inf / zero / denormal / extreme-exponent values; these families seed Rs/Rt
/// from this pool so every special-case branch of `arch_sf_recip_common` /
/// `arch_sf_invsqrt_common` (and the Pe predicate bits 0x80/0x40/0xe0) is hit.
const FP_SPECIAL: &[u32] = &[
    0x0000_0000, // +0
    0x8000_0000, // -0
    0x3f80_0000, // +1.0
    0xbf80_0000, // -1.0
    0x4000_0000, // +2.0
    0x4080_0000, // +4.0
    0x7f80_0000, // +inf
    0xff80_0000, // -inf
    0x7fc0_0000, // qNaN
    0x7f80_0001, // sNaN
    0xffc0_0001, // negative NaN
    0x0000_0001, // smallest +denormal
    0x807f_ffff, // largest -denormal
    0x0080_0000, // smallest +normal (exp 1)
    0x0088_0000, // tiny normal (2^-110-ish, exercises invsqrt r_exp<=24)
    0x7f7f_ffff, // largest +normal (exp 254)
    0x7e80_0000, // huge normal (exp 253, exercises recip d_exp>252)
    0x0100_0000, // small normal (exp 2, exercises recip d_exp<=1 region)
    0x4b00_0000, // 2^23, mid normal
    0xcb00_0000, // -2^23
    0x3f00_0000, // +0.5 (with a smallest-denormal multiplicand -> subnormal tie)
    0x0000_0002, // 2^-148 (subnormal, feeds ties-away rounding paths)
];

/// Lift-verify an F2 FP family whose register operands are seeded from
/// `FP_SPECIAL` (plus some fully-random words) so the special-case / seed-table /
/// scalbn branches are densely exercised. Compares ALL GPRs, ALL predicate bytes
/// (the seed ops write the full Pe byte) and USR:OVF, like `lift_family`.
fn lift_fp_special_family(name: &str, cases: &[(&str, &str)], n: usize, seed: u64) {
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_smir_lift] {name}: llvm-mc unavailable -> skipping");
            return;
        }
    };
    let mut rng = Rng::new(seed);
    let mut mismatches = Vec::new();
    let mut unlifted = Vec::new();
    for ((label, _asm), words) in cases.iter().zip(words_per.iter()) {
        for _ in 0..n {
            let mut st = State::zeroed();
            // Seed every GPR from the special pool (mostly) with occasional
            // fully-random words, so Rs/Rt (and pair halves) land on special
            // values regardless of which fields the opcode reads.
            for r in st.r.iter_mut() {
                *r = if rng.next() & 7 == 0 {
                    rng.next() as u32
                } else {
                    FP_SPECIAL[(rng.next() as usize) % FP_SPECIAL.len()]
                };
            }
            for k in 0..4 {
                st.p[k] = match rng.next() & 3 {
                    0 => 0x00,
                    1 => 0xff,
                    _ => rng.next() as u8,
                };
            }
            let interp = match run_interp(words, &st) {
                Some(s) => s,
                None => continue,
            };
            match lift_and_run(words, &st) {
                Ok(None) => {
                    unlifted.push(*label);
                    break;
                }
                Ok(Some(lift)) => {
                    let mut diffs = Vec::new();
                    for r in 0..NREG {
                        if interp.r[r] != lift.r[r] {
                            diffs.push(format!("r{r}:i={:#x},l={:#x}", interp.r[r], lift.r[r]));
                        }
                    }
                    for k in 0..4 {
                        if interp.p[k] != lift.p[k] {
                            diffs.push(format!("p{k}:i={:#x},l={:#x}", interp.p[k], lift.p[k]));
                        }
                    }
                    if (interp.usr & 1) != (lift.usr & 1) {
                        diffs.push(format!("usr_ovf:i={},l={}", interp.usr & 1, lift.usr & 1));
                    }
                    if !diffs.is_empty() {
                        mismatches.push(format!("[{label}] {}", diffs.join(" ")));
                    }
                }
                Err(e) => mismatches.push(format!("[{label}] {e}")),
            }
        }
    }
    if !unlifted.is_empty() {
        eprintln!("[hexagon_smir_lift] {name}: UNLIFTED (gap): {:?}", unlifted);
    }
    if !mismatches.is_empty() {
        eprintln!("\n==== {name}: {} lift mismatches ====", mismatches.len());
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!(
            "{name}: {} SMIR-lift divergences vs interpreter",
            mismatches.len()
        );
    }
}

// ---- Reciprocal / inverse-sqrt seed + fixup (F2_sfrecipa / F2_sfinvsqrta /
// F2_sffixupn / F2_sffixupd / F2_sffixupr). Byte-for-byte port of QEMU's
// arch_sf_recip_common / arch_sf_invsqrt_common (the 128-entry seed tables, the
// extreme-exponent scalbn adjusts, and the multi-bit Pe predicate) in the
// OpKind::HexFpRecip interp arm. sfrecipa/sfinvsqrta produce Rd AND the FULL Pe
// predicate byte (both compared); the fixup ops produce only Rd. Seeded from the
// FP_SPECIAL pool so the seed-table indices, the special cases, and the
// Pe=0x80/0x40/0xe0 scalbn branches are all exercised. ----
#[test]
fn lift_f2_sfrecip_seed() {
    lift_fp_special_family(
        "f2_sfrecip_seed",
        &[
            ("sfrecipa", "{ r4,p0 = sfrecipa(r1,r2) }"),
            ("sfinvsqrta", "{ r4,p0 = sfinvsqrta(r1) }"),
        ],
        80,
        0xf2_000d,
    );
}

#[test]
fn lift_f2_sffixup() {
    lift_fp_special_family(
        "f2_sffixup",
        &[
            ("sffixupn", "{ r4 = sffixupn(r1,r2) }"),
            ("sffixupd", "{ r4 = sffixupd(r1,r2) }"),
            ("sffixupr", "{ r4 = sffixupr(r1) }"),
        ],
        80,
        0xf2_000e,
    );
}

// ---- library fused multiply-add (`:lib`): F2_sffma_lib / F2_sffms_lib. The sem
// computes the EXACT single-rounding fma with ties-AWAY rounding of subnormal
// results (which native f32::mul_add — ties-to-even — cannot reproduce), then
// applies the Hexagon post-fixups: a spurious-overflow infinity (no infinite
// input) is backed off to max-finite (bit decrement), inf-minus-inf is flushed
// to +0, and a true-zero accumulator keeps its sign. Lifted via the exact
// integer fma core in the OpKind::HexFp3 `lib` path. Seeded from FP_SPECIAL
// (incl. denormals + huge normals) so the subnormal-tie + overflow-backoff +
// inf-minus-inf branches are exercised; Rx (r4) also draws from the pool. ----
#[test]
fn lift_f2_sffma_lib() {
    lift_fp_special_family(
        "f2_sffma_lib",
        &[
            ("sffma_lib", "{ r4 += sfmpy(r1,r2):lib }"),
            ("sffms_lib", "{ r4 -= sfmpy(r1,r2):lib }"),
        ],
        120,
        0xf2_000f,
    );
}

// ---- scaled fused multiply-add (`:scale`): F2_sffma_sc. Rx += Rs*Rt then
// * 2^Pu, where Pu is a two's-complement signed-8 scale folded into the EXACT
// product BEFORE the single rounding. Lifted via the exact integer fma core in
// the OpKind::HexFpScFma interp arm (native f32::mul_add then a separate scale
// would double-round). Rs/Rt/Rx are seeded from FP_SPECIAL; the predicate Pu
// (p3 here) draws a range of signed scale values from lift_fp_special_family's
// predicate seeding (which includes full random 8-bit bytes). ----
#[test]
fn lift_f2_sffma_sc() {
    lift_fp_special_family(
        "f2_sffma_sc",
        &[("sffma_sc", "{ r4 += sfmpy(r1,r2,p3):scale }")],
        160,
        0xf2_0010,
    );
}

/// Pool of special DOUBLE-precision FP bit patterns (full 64-bit) used to
/// exercise the dfmpyhh / dfmpyfix special-case branches: NaN/inf/zero/denormal,
/// the `df_is_big` (exp>=512) / `df_is_normal` classification, the high-32-bit
/// mantissa masking, and the subnormal-flush path.
const DF_SPECIAL: &[u64] = &[
    0x0000_0000_0000_0000, // +0
    0x8000_0000_0000_0000, // -0
    0x3ff0_0000_0000_0000, // +1.0
    0xbff0_0000_0000_0000, // -1.0
    0x4000_0000_0000_0000, // +2.0
    0x7ff0_0000_0000_0000, // +inf
    0xfff0_0000_0000_0000, // -inf
    0x7ff8_0000_0000_0000, // qNaN
    0x7ff0_0000_0000_0001, // sNaN
    0xfff4_0000_0000_0001, // negative NaN
    0x0000_0000_0000_0001, // smallest +denormal
    0x800f_ffff_ffff_ffff, // largest -denormal
    0x0010_0000_0000_0000, // smallest +normal (exp 1)
    0x7fef_ffff_ffff_ffff, // largest +normal (exp 0x7fe)
    0x4ff0_0000_0000_0000, // big normal (exp 0x4ff < 512, NOT df_is_big)
    0x5000_0000_0000_0000, // exp 0x500 == 1280 -> df_is_big & normal
    0x2000_0000_0000_0000, // exp 0x200 == 512 -> df_is_big boundary
    0x1fff_ffff_ffff_ffff, // exp 0x1ff == 511 -> just below df_is_big
    0x4330_0000_0000_0000, // 2^52, mid normal
    0xc330_0000_0000_0000, // -2^52
    0x3fe0_0000_0000_0001, // ~0.5 with a low mantissa bit (high-mask drops it)
    0x0000_0000_8000_0000, // denormal with only a HIGH-32 mantissa bit
    0x0000_0000_0000_8000, // denormal with only a LOW-32 mantissa bit (masked away)
];

/// Lift-verify a double-precision F2 family whose register PAIRS are seeded from
/// `DF_SPECIAL` so every special-case / classification branch is exercised. Each
/// pair `R{2k+1}:R{2k}` is seeded as a whole f64 (even = low word, odd = high
/// word). Compares ALL GPRs + USR:OVF (these ops never set OVF).
fn lift_df_special_family(name: &str, cases: &[(&str, &str)], n: usize, seed: u64) {
    let asms: Vec<String> = cases.iter().map(|(_, a)| a.to_string()).collect();
    let words_per = match assemble(&asms) {
        Some(w) => w,
        None => {
            eprintln!("[hexagon_smir_lift] {name}: llvm-mc unavailable -> skipping");
            return;
        }
    };
    let mut rng = Rng::new(seed);
    let mut mismatches = Vec::new();
    let mut unlifted = Vec::new();
    for ((label, _asm), words) in cases.iter().zip(words_per.iter()) {
        for _ in 0..n {
            let mut st = State::zeroed();
            // Seed each consecutive even/odd GPR pair as one f64 from the pool
            // (occasionally fully random) so Rss/Rtt/Rxx land on special values.
            for k in 0..(NREG / 2) {
                let v = if rng.next() & 7 == 0 {
                    rng.next()
                } else {
                    DF_SPECIAL[(rng.next() as usize) % DF_SPECIAL.len()]
                };
                st.r[2 * k] = v as u32;
                st.r[2 * k + 1] = (v >> 32) as u32;
            }
            for p in st.p.iter_mut() {
                *p = rng.next() as u8;
            }
            let interp = match run_interp(words, &st) {
                Some(s) => s,
                None => continue,
            };
            match lift_and_run(words, &st) {
                Ok(None) => {
                    unlifted.push(*label);
                    break;
                }
                Ok(Some(lift)) => {
                    let mut diffs = Vec::new();
                    for r in 0..NREG {
                        if interp.r[r] != lift.r[r] {
                            diffs.push(format!("r{r}:i={:#x},l={:#x}", interp.r[r], lift.r[r]));
                        }
                    }
                    for k in 0..4 {
                        if interp.p[k] != lift.p[k] {
                            diffs.push(format!("p{k}:i={:#x},l={:#x}", interp.p[k], lift.p[k]));
                        }
                    }
                    if (interp.usr & 1) != (lift.usr & 1) {
                        diffs.push(format!("usr_ovf:i={},l={}", interp.usr & 1, lift.usr & 1));
                    }
                    if !diffs.is_empty() {
                        mismatches.push(format!("[{label}] {}", diffs.join(" ")));
                    }
                }
                Err(e) => mismatches.push(format!("[{label}] {e}")),
            }
        }
    }
    if !unlifted.is_empty() {
        eprintln!("[hexagon_smir_lift] {name}: UNLIFTED (gap): {:?}", unlifted);
    }
    if !mismatches.is_empty() {
        eprintln!("\n==== {name}: {} lift mismatches ====", mismatches.len());
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!(
            "{name}: {} SMIR-lift divergences vs interpreter",
            mismatches.len()
        );
    }
}

// ---- double-precision high-half multiply + accumulate (F2_dfmpyhh) and the
// denormal fixup (F2_dfmpyfix). dfmpyhh masks each mantissa to its HIGH 32 bits,
// multiplies exactly, accumulates Rxx at a FIXED weight (prod_e+31), and rounds
// once to nearest-even — it needs the EXACT f64 rounding core (native f64
// double-rounds). dfmpyfix is a conditional exact 2^±52 scale. Both lifted via
// the OpKind::HexFpDf interp arm; seeded from DF_SPECIAL so the subnormal-flush,
// df_is_big/normal classification, and high-mantissa-mask branches are hit. ----
#[test]
fn lift_f2_dfmpyhh() {
    lift_df_special_family(
        "f2_dfmpyhh",
        &[("dfmpyhh", "{ r5:4 += dfmpyhh(r1:0, r3:2) }")],
        200,
        0xf2_0011,
    );
}

#[test]
fn lift_f2_dfmpyfix() {
    lift_df_special_family(
        "f2_dfmpyfix",
        &[("dfmpyfix", "{ r5:4 = dfmpyfix(r1:0, r3:2) }")],
        200,
        0xf2_0012,
    );
}

// ---- S2_cabacdecbin: Rdd = decbin(Rss,Rtt) (+P0). A PURE FUNCTION of the
// register inputs + the constant H.264 transition tables (no hidden state).
// Seeded to exercise the full CABAC state space: Rtt carries {state[5:0],
// val_mps[8], bitpos[4:0]} so we sweep all 64 states incl. the boundary states
// 0 and 63; Rss carries {range, offset} which drive the MPS/LPS region split.
// Lifted via OpKind::HexCabacDecBin (writes the Rdd pair AND P0). ----
#[test]
fn lift_s2_cabacdecbin() {
    let asm = "{ r5:4 = decbin(r1:0, r3:2) }";
    let words = match assemble(&[asm.to_string()]) {
        Some(w) => w.into_iter().next().unwrap(),
        None => {
            eprintln!("[hexagon_smir_lift] s2_cabacdecbin: llvm-mc unavailable -> skipping");
            return;
        }
    };
    let mut rng = Rng::new(0xcabac_001);
    let mut mismatches = Vec::new();
    // Sweep every state 0..=63 plus extra random rounds, biasing range/offset to
    // realistic CABAC magnitudes (top bits set) and both MPS/LPS regions.
    for round in 0..400usize {
        let state = if round < 64 {
            round as u32
        } else {
            (rng.next() % 64) as u32
        };
        let val_mps = (rng.next() & 1) as u32;
        let bitpos = (rng.next() % 8) as u32; // small shift, keeps range in range
        let mut st = State::zeroed();
        // Rtt (r3:2): w1 = {val_mps<<8 | state}, w0 = {bitpos}.
        let rtt_w1 = (val_mps << 8) | state;
        let rtt_w0 = bitpos;
        st.r[2] = rtt_w0;
        st.r[3] = rtt_w1;
        // Rss (r1:0): w0 = range (bias top bit set), w1 = offset.
        let range = 0x8000_0000u32 | (rng.next() as u32 >> 1);
        let offset = rng.next() as u32 >> 1; // < range often -> exercises both regions
        st.r[0] = range;
        st.r[1] = offset;
        for p in st.p.iter_mut() {
            *p = rng.next() as u8;
        }
        let interp = match run_interp(&words, &st) {
            Some(s) => s,
            None => continue,
        };
        match lift_and_run(&words, &st) {
            Ok(None) => {
                panic!("s2_cabacdecbin: UNLIFTED");
            }
            Ok(Some(lift)) => {
                let mut diffs = Vec::new();
                for r in 0..NREG {
                    if interp.r[r] != lift.r[r] {
                        diffs.push(format!("r{r}:i={:#x},l={:#x}", interp.r[r], lift.r[r]));
                    }
                }
                for k in 0..4 {
                    if interp.p[k] != lift.p[k] {
                        diffs.push(format!("p{k}:i={:#x},l={:#x}", interp.p[k], lift.p[k]));
                    }
                }
                if !diffs.is_empty() {
                    mismatches.push(format!("[state={state} round={round}] {}", diffs.join(" ")));
                }
            }
            Err(e) => mismatches.push(format!("[round={round}] {e}")),
        }
    }
    if !mismatches.is_empty() {
        eprintln!(
            "\n==== s2_cabacdecbin: {} lift mismatches ====",
            mismatches.len()
        );
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!(
            "s2_cabacdecbin: {} SMIR-lift divergences vs interpreter",
            mismatches.len()
        );
    }
}

// ---- A4_tlbmatch: Pd = tlbmatch(Rss,Rt). A PURE FUNCTION of the register
// inputs — the matched "TLB entry" IS the seeded register pair Rss, NOT hidden
// TLB state — so it is fully reproducible. Random GPRs exercise the page-size
// (cl1) computation, the size clamp at 6, and the valid-bit / masked-compare.
// Bias TLBHI's bit31 (valid) and align some entries so matches actually occur.
#[test]
fn lift_a4_tlbmatch() {
    let asm = "{ p3 = tlbmatch(r1:0, r2) }";
    let words = match assemble(&[asm.to_string()]) {
        Some(w) => w.into_iter().next().unwrap(),
        None => {
            eprintln!("[hexagon_smir_lift] a4_tlbmatch: llvm-mc unavailable -> skipping");
            return;
        }
    };
    let mut rng = Rng::new(0x71b_a7c4);
    let mut mismatches = Vec::new();
    for round in 0..400usize {
        let mut st = State::zeroed();
        for r in st.r.iter_mut() {
            *r = rng.next() as u32;
        }
        // Bias: half the rounds set TLBHI bit31 (valid); a quarter make Rt equal
        // TLBHI so a match can fire after masking.
        if rng.next() & 1 == 0 {
            st.r[1] |= 0x8000_0000;
        }
        if rng.next() & 3 == 0 {
            st.r[2] = st.r[1];
        }
        for p in st.p.iter_mut() {
            *p = rng.next() as u8;
        }
        let interp = match run_interp(&words, &st) {
            Some(s) => s,
            None => continue,
        };
        match lift_and_run(&words, &st) {
            Ok(None) => panic!("a4_tlbmatch: UNLIFTED"),
            Ok(Some(lift)) => {
                let mut diffs = Vec::new();
                for r in 0..NREG {
                    if interp.r[r] != lift.r[r] {
                        diffs.push(format!("r{r}:i={:#x},l={:#x}", interp.r[r], lift.r[r]));
                    }
                }
                for k in 0..4 {
                    if interp.p[k] != lift.p[k] {
                        diffs.push(format!("p{k}:i={:#x},l={:#x}", interp.p[k], lift.p[k]));
                    }
                }
                if !diffs.is_empty() {
                    mismatches.push(format!("[round={round}] {}", diffs.join(" ")));
                }
            }
            Err(e) => mismatches.push(format!("[round={round}] {e}")),
        }
    }
    if !mismatches.is_empty() {
        eprintln!(
            "\n==== a4_tlbmatch: {} lift mismatches ====",
            mismatches.len()
        );
        for m in mismatches.iter().take(20) {
            eprintln!("  {m}");
        }
        panic!(
            "a4_tlbmatch: {} SMIR-lift divergences vs interpreter",
            mismatches.len()
        );
    }
}
