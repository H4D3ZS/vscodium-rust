//! RISC-V vector (RVV 1.0) differential test harness: rax vs. qemu-riscv64.
//!
//! Dedicated to the V extension data path. Uses `tools/riscv-diff/voracle.c`,
//! a static RV64 ELF whose prologue loads v0..v31 and the test vtype/vl, runs
//! one vector instruction, and captures the vector state from the signal-frame
//! V context. For each case we run the same instruction on `RiscVCpu` from the
//! identical state and compare the full vector register file, `vl`/`vtype`, the
//! integer/FP registers, and the shared scratch window.
//!
//! Tail/mask policy is undisturbed (vta=0, vma=0) so results are deterministic.
//! Self-skips if the toolchain or qemu is unavailable.

#![cfg(target_os = "linux")]

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use rax::riscv::{FlatMemory, RiscVConfig, RiscVCpu, RiscVExit};

const WIRE_MAGIC: u32 = 0x5652_4332; // 'V','R','C','2'
const SCRATCH_ADDR: u64 = 0x20_0000;
const SCRATCH_BASE: u64 = SCRATCH_ADDR + 64;
const VLENB: usize = 16;
const INSN_ADDR: u64 = 0x1000;

#[repr(C)]
#[derive(Clone, Copy)]
struct VState {
    x: [u64; 32],
    f: [u64; 32],
    vtype: u64,
    vl: u64,
    vstart: u64,
    fcsr: u64,
    vcsr: u64,
    v: [u64; 64],
    scratch: [u64; 32],
}

impl VState {
    fn zeroed() -> Self {
        VState {
            x: [0; 32],
            f: [0; 32],
            vtype: 0,
            vl: 0,
            vstart: 0,
            fcsr: 0,
            vcsr: 0,
            v: [0; 64],
            scratch: [0; 32],
        }
    }
    fn vreg_bytes(&self, r: usize) -> [u8; VLENB] {
        let mut out = [0u8; VLENB];
        out[..8].copy_from_slice(&self.v[r * 2].to_le_bytes());
        out[8..].copy_from_slice(&self.v[r * 2 + 1].to_le_bytes());
        out
    }
    fn set_vreg_bytes(&mut self, r: usize, b: &[u8; VLENB]) {
        self.v[r * 2] = u64::from_le_bytes(b[..8].try_into().unwrap());
        self.v[r * 2 + 1] = u64::from_le_bytes(b[8..].try_into().unwrap());
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VInCase {
    insn: u32,
    insn_len: u32,
    st: VState,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct VOutCase {
    st: VState,
    trapped: u32,
    valid: u32,
}

fn as_bytes<T: Copy>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
}
fn read_struct<T: Copy>(buf: &[u8], off: usize) -> T {
    unsafe { std::ptr::read_unaligned(buf[off..].as_ptr() as *const T) }
}

struct Rng(u64);
impl Rng {
    fn new(s: u64) -> Self {
        Rng(s)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn voracle_path() -> Option<PathBuf> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/riscv-diff");
    let oracle = dir.join("voracle");
    let needs = match (oracle.metadata(), dir.join("voracle.c").metadata()) {
        (Ok(o), Ok(s)) => match (o.modified(), s.modified()) {
            (Ok(om), Ok(sm)) => om < sm,
            _ => true,
        },
        _ => true,
    };
    if needs {
        let cc = std::env::var("RISCV64_CC").unwrap_or_else(|_| "riscv64-linux-gnu-gcc".into());
        let status = Command::new(cc)
            .args(["-static", "-O2", "-march=rv64gc", "-mabi=lp64d", "-o"])
            .arg(&oracle)
            .arg(dir.join("voracle.c"))
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

fn run_oracle(oracle: &PathBuf, cases: &[(u32, VState)]) -> Option<Vec<VOutCase>> {
    let mut payload = Vec::with_capacity(8 + cases.len() * std::mem::size_of::<VInCase>());
    payload.extend_from_slice(&WIRE_MAGIC.to_le_bytes());
    payload.extend_from_slice(&(cases.len() as u32).to_le_bytes());
    for (insn, st) in cases {
        let ic = VInCase {
            insn: *insn,
            insn_len: 4,
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
    if !child.wait().ok()?.success() || out.len() < 8 {
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
        res.push(read_struct::<VOutCase>(&out, off));
        off += std::mem::size_of::<VOutCase>();
    }
    Some(res)
}

fn run_vrax(insn: u32, input: &VState) -> Option<VState> {
    let mem = FlatMemory::new(0, 0x21_0000);
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
    cpu.set_vcsr(input.vcsr);
    cpu.set_vl_vtype(input.vl, input.vtype);
    for r in 0..32usize {
        cpu.set_vreg(r as u8, &input.vreg_bytes(r));
    }
    let mut sb = Vec::with_capacity(256);
    for w in input.scratch.iter() {
        sb.extend_from_slice(&w.to_le_bytes());
    }
    cpu.write_memory(SCRATCH_ADDR, &sb).ok()?;
    cpu.write_memory(INSN_ADDR, &insn.to_le_bytes()).ok()?;
    cpu.set_pc(INSN_ADDR);
    match cpu.step() {
        RiscVExit::Continue => {}
        _ => return None,
    }
    let mut out = VState::zeroed();
    for i in 1..32u8 {
        out.x[i as usize] = cpu.x(i);
    }
    for i in 0..32u8 {
        out.f[i as usize] = cpu.f(i);
    }
    out.fcsr = cpu.fcsr() as u64;
    out.vcsr = cpu.vcsr();
    out.vl = cpu.vl();
    out.vtype = cpu.vtype();
    for r in 0..32usize {
        out.set_vreg_bytes(r, &cpu.vreg(r as u8));
    }
    for (i, w) in out.scratch.iter_mut().enumerate() {
        *w = cpu.mem_read_u64(SCRATCH_ADDR + (i as u64) * 8).ok()?;
    }
    Some(out)
}

struct Mismatch {
    label: String,
    insn: u32,
    detail: String,
}

fn compare(label: &str, insn: u32, input: &VState, oracle: &VOutCase, ms: &mut Vec<Mismatch>) {
    let rax = run_vrax(insn, input);
    if oracle.trapped != 0 {
        if rax.is_some() {
            ms.push(Mismatch {
                label: label.into(),
                insn,
                detail: format!("hw faulted sig {} but rax executed", oracle.trapped),
            });
        }
        return;
    }
    let rax = match rax {
        Some(s) => s,
        None => {
            ms.push(Mismatch {
                label: label.into(),
                insn,
                detail: "hw executed but rax rejected".into(),
            });
            return;
        }
    };
    let mut d = Vec::new();
    for i in 1..32usize {
        if i == 3 || i == 4 {
            continue;
        }
        if rax.x[i] != oracle.st.x[i] {
            d.push(format!(
                "x{i}: rax={:#x} hw={:#x}",
                rax.x[i], oracle.st.x[i]
            ));
        }
    }
    if rax.vl != oracle.st.vl {
        d.push(format!("vl: rax={} hw={}", rax.vl, oracle.st.vl));
    }
    if rax.vtype != oracle.st.vtype {
        d.push(format!(
            "vtype: rax={:#x} hw={:#x}",
            rax.vtype, oracle.st.vtype
        ));
    }
    if rax.fcsr != oracle.st.fcsr {
        d.push(format!(
            "fcsr: rax={:#x} hw={:#x}",
            rax.fcsr, oracle.st.fcsr
        ));
    }
    if rax.vcsr != oracle.st.vcsr {
        d.push(format!(
            "vcsr: rax={:#x} hw={:#x}",
            rax.vcsr, oracle.st.vcsr
        ));
    }
    for i in 0..32usize {
        if rax.f[i] != oracle.st.f[i] {
            d.push(format!(
                "f{i}: rax={:#x} hw={:#x}",
                rax.f[i], oracle.st.f[i]
            ));
        }
    }
    for r in 0..32usize {
        if rax.vreg_bytes(r) != oracle.st.vreg_bytes(r) {
            d.push(format!(
                "v{r}: rax={:02x?} hw={:02x?}",
                rax.vreg_bytes(r),
                oracle.st.vreg_bytes(r)
            ));
        }
    }
    for i in 0..32usize {
        if rax.scratch[i] != oracle.st.scratch[i] {
            d.push(format!(
                "scratch[{i}]: rax={:#x} hw={:#x}",
                rax.scratch[i], oracle.st.scratch[i]
            ));
        }
    }
    if !d.is_empty() {
        ms.push(Mismatch {
            label: label.into(),
            insn,
            detail: d.join(", "),
        });
    }
}

fn run_batch(batch: &[(String, u32, VState)]) {
    let oracle = match voracle_path() {
        Some(p) => p,
        None => {
            eprintln!("riscv_vector: voracle/toolchain unavailable; skipping");
            return;
        }
    };
    let cases: Vec<(u32, VState)> = batch.iter().map(|(_, i, s)| (*i, *s)).collect();
    let outs = match run_oracle(&oracle, &cases) {
        Some(o) => o,
        None => {
            eprintln!("riscv_vector: voracle run failed; skipping");
            return;
        }
    };
    let mut ms = Vec::new();
    for ((label, insn, st), oc) in batch.iter().zip(outs.iter()) {
        compare(label, *insn, st, oc, &mut ms);
    }
    if !ms.is_empty() {
        let mut msg = format!("\n{} vector divergence(s) from qemu-riscv64:\n", ms.len());
        for m in ms.iter().take(30) {
            msg += &format!("  [{}] insn={:#010x}: {}\n", m.label, m.insn, m.detail);
        }
        if ms.len() > 30 {
            msg += &format!("  ... and {} more\n", ms.len() - 30);
        }
        panic!("{msg}");
    }
}

// ---------------------------------------------------------------------------
// Test-case construction.
// ---------------------------------------------------------------------------

/// vtype for (sew_log2, lmul=m1, vta=0, vma=0).
fn vtype_m1(sew_log2: u32) -> u64 {
    (sew_log2 << 3) as u64
}
fn vlmax(sew_log2: u32) -> u64 {
    16 >> sew_log2 // VLEN/8 / sew_bytes, for m1
}

fn rand_vstate(rng: &mut Rng, sew_log2: u32, vl: u64) -> VState {
    let mut st = VState::zeroed();
    for i in 0..64 {
        st.v[i] = rng.next();
    }
    for i in 1..32usize {
        if i == 3 || i == 4 {
            continue;
        }
        st.x[i] = rng.next();
    }
    st.vtype = vtype_m1(sew_log2);
    st.vl = vl;
    st
}

const VPOOL: [u32; 6] = [1, 2, 3, 5, 6, 7]; // vector regs (avoid v0 = mask, v4)
const XPOOL: [u32; 5] = [1, 5, 6, 7, 10];

fn op_iv(funct6: u32, vm: u32, vs2: u32, src: u32, funct3: u32, vd: u32) -> u32 {
    (funct6 << 26) | (vm << 25) | (vs2 << 20) | (src << 15) | (funct3 << 12) | (vd << 7) | 0x57
}

#[test]
fn diff_v_arith() {
    let mut rng = Rng::new(0x7EC_704);
    let mut batch = Vec::new();
    let ops: &[(&str, u32, bool, bool)] = &[
        // (name, funct6, has_vv, has_vi)
        ("vadd", 0b000000, true, true),
        ("vsub", 0b000010, true, false),
        ("vrsub", 0b000011, false, true),
        ("vand", 0b001001, true, true),
        ("vor", 0b001010, true, true),
        ("vxor", 0b001011, true, true),
        ("vminu", 0b000100, true, false),
        ("vmin", 0b000101, true, false),
        ("vmaxu", 0b000110, true, false),
        ("vmax", 0b000111, true, false),
        ("vsll", 0b100101, true, true),
        ("vsrl", 0b101000, true, true),
        ("vsra", 0b101001, true, true),
    ];
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, vmax.max(1) - 0, (vmax / 2).max(1)] {
            for &(name, f6, has_vv, has_vi) in ops {
                for _ in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let rs1 = XPOOL[(rng.next() % 5) as usize];
                    let st = rand_vstate(&mut rng, sew_log2, vl);
                    // OPIVV (funct3=000)
                    if has_vv {
                        batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b000, vd), st));
                    }
                    // OPIVX (funct3=100)
                    batch.push((format!("{name}.vx"), op_iv(f6, 1, vs2, rs1, 0b100, vd), st));
                    // OPIVI (funct3=011)
                    if has_vi {
                        let imm = (rng.next() & 0x1f) as u32;
                        batch.push((format!("{name}.vi"), op_iv(f6, 1, vs2, imm, 0b011, vd), st));
                    }
                    // masked OPIVV (vm=0, vd != v0)
                    if has_vv && vd != 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next(); // mask in v0
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vv.m"),
                            op_iv(f6, 0, vs2, vs1, 0b000, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_merge() {
    let mut rng = Rng::new(0x7EC_70E);
    let mut batch = Vec::new();
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for _ in 0..10 {
                let vd = VPOOL[(rng.next() % 6) as usize];
                let vs2 = VPOOL[(rng.next() % 6) as usize];
                let vs1 = VPOOL[(rng.next() % 6) as usize];
                let rs1 = XPOOL[(rng.next() % 5) as usize];
                let imm = (rng.next() & 0x1f) as u32;
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                st.v[0] = rng.next(); // mask
                st.v[1] = rng.next();
                // vmerge.vvm / vxm / vim (vm=0)
                batch.push((
                    "vmerge.vvm".into(),
                    op_iv(0b010111, 0, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vmerge.vxm".into(),
                    op_iv(0b010111, 0, vs2, rs1, 0b100, vd),
                    st,
                ));
                batch.push((
                    "vmerge.vim".into(),
                    op_iv(0b010111, 0, vs2, imm, 0b011, vd),
                    st,
                ));
                // vmv.v.v / vx / vi (vm=1, vs2=0)
                batch.push(("vmv.v.v".into(), op_iv(0b010111, 1, 0, vs1, 0b000, vd), st));
                batch.push(("vmv.v.x".into(), op_iv(0b010111, 1, 0, rs1, 0b100, vd), st));
                batch.push(("vmv.v.i".into(), op_iv(0b010111, 1, 0, imm, 0b011, vd), st));
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_compare() {
    let mut rng = Rng::new(0x7EC_710);
    let mut batch = Vec::new();
    // (name, funct6, has_vv, has_vi)
    let ops: &[(&str, u32, bool, bool)] = &[
        ("vmseq", 0b011000, true, true),
        ("vmsne", 0b011001, true, true),
        ("vmsltu", 0b011010, true, false),
        ("vmslt", 0b011011, true, false),
        ("vmsleu", 0b011100, true, true),
        ("vmsle", 0b011101, true, true),
        ("vmsgtu", 0b011110, false, true),
        ("vmsgt", 0b011111, false, true),
    ];
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for &(name, f6, has_vv, has_vi) in ops {
                for _ in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let rs1 = XPOOL[(rng.next() % 5) as usize];
                    let st = rand_vstate(&mut rng, sew_log2, vl);
                    if has_vv {
                        batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b000, vd), st));
                    }
                    batch.push((format!("{name}.vx"), op_iv(f6, 1, vs2, rs1, 0b100, vd), st));
                    if has_vi {
                        let imm = (rng.next() & 0x1f) as u32;
                        batch.push((format!("{name}.vi"), op_iv(f6, 1, vs2, imm, 0b011, vd), st));
                    }
                    // masked vv (vd != v0)
                    if has_vv && vd != 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vv.m"),
                            op_iv(f6, 0, vs2, vs1, 0b000, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_muldiv() {
    let mut rng = Rng::new(0x7EC_712);
    let mut batch = Vec::new();
    let ops: &[(&str, u32)] = &[
        ("vmul", 0b100101),
        ("vmulh", 0b100111),
        ("vmulhu", 0b100100),
        ("vmulhsu", 0b100110),
        ("vdivu", 0b100000),
        ("vdiv", 0b100001),
        ("vremu", 0b100010),
        ("vrem", 0b100011),
    ];
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for &(name, f6) in ops {
                for k in 0..8 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let rs1 = XPOOL[(rng.next() % 5) as usize];
                    let mut st = rand_vstate(&mut rng, sew_log2, vl);
                    // Exercise divide-by-zero / overflow corners.
                    if k % 3 == 0 {
                        st.x[rs1 as usize] = 0;
                        st.v[vs1 as usize * 2] = 0;
                        st.v[vs1 as usize * 2 + 1] = 0;
                    }
                    batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b010, vd), st));
                    batch.push((format!("{name}.vx"), op_iv(f6, 1, vs2, rs1, 0b110, vd), st));
                    if vd != 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vv.m"),
                            op_iv(f6, 0, vs2, vs1, 0b010, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

/// NaN-box every f register with a fresh random value of element width `eb`
/// (so an `OPFVF` scalar read from any f register is well-formed), and inject a
/// few "interesting" FP datums into the low v-register elements.
fn fp_setup(st: &mut VState, rng: &mut Rng, eb: usize) {
    for i in 0..32usize {
        let r = rng.next();
        st.f[i] = match eb {
            2 => 0xffff_ffff_ffff_0000 | (r & 0xffff),
            4 => 0xffff_ffff_0000_0000 | (r & 0xffff_ffff),
            _ => r,
        };
    }
    // Splice some canonical values (±0, ±inf, qNaN, small ints) across lanes so
    // equality / ordering / special-case paths are exercised, not just randoms.
    let specials: &[u64] = match eb {
        2 => &[
            0x0000, 0x8000, 0x7c00, 0xfc00, 0x7e00, 0x3c00, 0xbc00, 0x0001,
        ],
        4 => &[
            0x0000_0000,
            0x8000_0000,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_0000,
            0x3f80_0000,
            0xbf80_0000,
            0x0000_0001,
        ],
        _ => &[
            0x0000_0000_0000_0000,
            0x8000_0000_0000_0000,
            0x7ff0_0000_0000_0000,
            0xfff0_0000_0000_0000,
            0x7ff8_0000_0000_0000,
            0x3ff0_0000_0000_0000,
            0xbff0_0000_0000_0000,
            0x0000_0000_0000_0001,
        ],
    };
    let per = 16 / eb; // elements per 128-bit register
    for (k, &val) in specials.iter().enumerate() {
        let elem = k % (per * 8); // spread over v0..v8 worth of lanes
        let byte = elem * eb;
        let widx = byte / 8;
        let shift = (byte % 8) * 8;
        if widx < 64 {
            let m = if eb == 8 {
                u64::MAX
            } else {
                ((1u64 << (eb * 8)) - 1) << shift
            };
            st.v[widx] = (st.v[widx] & !m) | ((val << shift) & m);
        }
    }
}

#[test]
fn diff_v_fp() {
    let mut rng = Rng::new(0x7EC_730);
    let mut batch = Vec::new();
    // (name, funct6, vf_only)
    let bin_ops: &[(&str, u32, bool)] = &[
        ("vfadd", 0b000000, false),
        ("vfsub", 0b000010, false),
        ("vfrsub", 0b100111, true),
        ("vfmul", 0b100100, false),
        ("vfdiv", 0b100000, false),
        ("vfrdiv", 0b100001, true),
        ("vfmin", 0b000100, false),
        ("vfmax", 0b000110, false),
        ("vfsgnj", 0b001000, false),
        ("vfsgnjn", 0b001001, false),
        ("vfsgnjx", 0b001010, false),
    ];
    let cmp_ops: &[(&str, u32, bool)] = &[
        ("vmfeq", 0b011000, false),
        ("vmfle", 0b011001, false),
        ("vmflt", 0b011011, false),
        ("vmfne", 0b011100, false),
        ("vmfgt", 0b011101, true),
        ("vmfge", 0b011111, true),
    ];
    let fpool: [u32; 5] = [0, 1, 8, 15, 20]; // f-register sources for vf forms
    // FP element widths only: SEW 16 / 32 / 64.
    for sew_log2 in 1..4u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for &(name, f6, vf_only) in bin_ops.iter() {
                for k in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let rs1 = fpool[(rng.next() % 5) as usize];
                    let frm = rng.next() % 5;
                    let mut st = rand_vstate(&mut rng, sew_log2, vl);
                    st.fcsr = frm << 5;
                    fp_setup(&mut st, &mut rng, eb);
                    if !vf_only {
                        batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b001, vd), st));
                    }
                    batch.push((format!("{name}.vf"), op_iv(f6, 1, vs2, rs1, 0b101, vd), st));
                    if !vf_only && vd != 0 && k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vv.m"),
                            op_iv(f6, 0, vs2, vs1, 0b001, vd),
                            stm,
                        ));
                    }
                }
            }
            for &(name, f6, vf_only) in cmp_ops.iter() {
                for _ in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let rs1 = fpool[(rng.next() % 5) as usize];
                    let mut st = rand_vstate(&mut rng, sew_log2, vl);
                    fp_setup(&mut st, &mut rng, eb);
                    if !vf_only {
                        batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b001, vd), st));
                    }
                    batch.push((format!("{name}.vf"), op_iv(f6, 1, vs2, rs1, 0b101, vd), st));
                }
            }
            // vfsqrt.v (OPFVV unary, vs1 field = 0b00000, funct6 = 0b010011).
            for _ in 0..6 {
                let vd = VPOOL[(rng.next() % 6) as usize];
                let vs2 = VPOOL[(rng.next() % 6) as usize];
                let frm = rng.next() % 5;
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                st.fcsr = frm << 5;
                fp_setup(&mut st, &mut rng, eb);
                batch.push(("vfsqrt.v".into(), op_iv(0b010011, 1, vs2, 0, 0b001, vd), st));
            }
            // Fused multiply-add family (reads vd as a source operand).
            let fma_ops: &[(&str, u32)] = &[
                ("vfmacc", 0b101100),
                ("vfnmacc", 0b101101),
                ("vfmsac", 0b101110),
                ("vfnmsac", 0b101111),
                ("vfmadd", 0b101000),
                ("vfnmadd", 0b101001),
                ("vfmsub", 0b101010),
                ("vfnmsub", 0b101011),
            ];
            for &(name, f6) in fma_ops {
                for k in 0..5 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let rs1 = fpool[(rng.next() % 5) as usize];
                    let frm = rng.next() % 5;
                    let mut st = rand_vstate(&mut rng, sew_log2, vl);
                    st.fcsr = frm << 5;
                    fp_setup(&mut st, &mut rng, eb);
                    batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b001, vd), st));
                    batch.push((format!("{name}.vf"), op_iv(f6, 1, vs2, rs1, 0b101, vd), st));
                    if vd != 0 && k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vv.m"),
                            op_iv(f6, 0, vs2, vs1, 0b001, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_redux() {
    let mut rng = Rng::new(0x7EC_740);
    let mut batch = Vec::new();
    // OPMVV integer reductions (funct3 = 0b010); vd[0] = vs1[0] OP fold(vs2[0..vl]).
    let ops: &[(&str, u32)] = &[
        ("vredsum", 0b000000),
        ("vredand", 0b000001),
        ("vredor", 0b000010),
        ("vredxor", 0b000011),
        ("vredminu", 0b000100),
        ("vredmin", 0b000101),
        ("vredmaxu", 0b000110),
        ("vredmax", 0b000111),
    ];
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1), 1] {
            for &(name, f6) in ops {
                for k in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let st = rand_vstate(&mut rng, sew_log2, vl);
                    batch.push((format!("{name}.vs"), op_iv(f6, 1, vs2, vs1, 0b010, vd), st));
                    if vd != 0 && k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vs.m"),
                            op_iv(f6, 0, vs2, vs1, 0b010, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_fp_redux() {
    let mut rng = Rng::new(0x7EC_750);
    let mut batch = Vec::new();
    // OPFVV floating-point reductions (.vs form, funct3 = 0b001).
    let ops: &[(&str, u32)] = &[
        ("vfredusum", 0b000001),
        ("vfredosum", 0b000011),
        ("vfredmin", 0b000101),
        ("vfredmax", 0b000111),
    ];
    for sew_log2 in 1..4u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1), 1] {
            for &(name, f6) in ops {
                for k in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let frm = rng.next() % 5;
                    let mut st = rand_vstate(&mut rng, sew_log2, vl);
                    st.fcsr = frm << 5;
                    fp_setup(&mut st, &mut rng, eb);
                    batch.push((format!("{name}.vs"), op_iv(f6, 1, vs2, vs1, 0b001, vd), st));
                    if vd != 0 && k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vs.m"),
                            op_iv(f6, 0, vs2, vs1, 0b001, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_scalar_move() {
    let mut rng = Rng::new(0x7EC_760);
    let mut batch = Vec::new();
    let fpool: [u32; 5] = [0, 1, 8, 15, 20];
    for sew_log2 in 0..4u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1), 0] {
            for _ in 0..8 {
                let vd = VPOOL[(rng.next() % 6) as usize];
                let vs2 = VPOOL[(rng.next() % 6) as usize];
                let xr = XPOOL[(rng.next() % 5) as usize];
                let fr = fpool[(rng.next() % 5) as usize];
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                fp_setup(&mut st, &mut rng, eb.max(2));
                // vmv.x.s rd, vs2  (funct3=010, vs1=0); dest is integer reg `xr`.
                batch.push(("vmv.x.s".into(), op_iv(0b010000, 1, vs2, 0, 0b010, xr), st));
                // vmv.s.x vd, rs1  (funct3=110, vs2=0); source integer reg `xr`.
                batch.push(("vmv.s.x".into(), op_iv(0b010000, 1, 0, xr, 0b110, vd), st));
                // FP moves valid only for SEW in {16,32,64}.
                if sew_log2 >= 1 {
                    // vfmv.f.s fd, vs2  (funct3=001, vs1=0); dest f reg `fr`.
                    batch.push(("vfmv.f.s".into(), op_iv(0b010000, 1, vs2, 0, 0b001, fr), st));
                    // vfmv.s.f vd, fs1 (funct3=101, vs2=0); source f reg `fr`.
                    batch.push(("vfmv.s.f".into(), op_iv(0b010000, 1, 0, fr, 0b101, vd), st));
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_mask_logic() {
    let mut rng = Rng::new(0x7EC_770);
    let mut batch = Vec::new();
    // OPMVV mask-register logicals (funct3 = 0b010), always unmasked (vm=1).
    let ops: &[(&str, u32)] = &[
        ("vmandn", 0b011000),
        ("vmand", 0b011001),
        ("vmor", 0b011010),
        ("vmxor", 0b011011),
        ("vmorn", 0b011100),
        ("vmnand", 0b011101),
        ("vmnor", 0b011110),
        ("vmxnor", 0b011111),
    ];
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1), 1] {
            for &(name, f6) in ops {
                for _ in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    // Random mask-register contents in all source lanes.
                    let st = rand_vstate(&mut rng, sew_log2, vl);
                    batch.push((format!("{name}.mm"), op_iv(f6, 1, vs2, vs1, 0b010, vd), st));
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_ext() {
    let mut rng = Rng::new(0x7EC_780);
    let mut batch = Vec::new();
    // (name, vs1-selector, factor, min_sew_log2)
    let ops: &[(&str, u32, u32)] = &[
        ("vzext.vf2", 0b00110, 1),
        ("vsext.vf2", 0b00111, 1),
        ("vzext.vf4", 0b00100, 2),
        ("vsext.vf4", 0b00101, 2),
        ("vzext.vf8", 0b00010, 3),
        ("vsext.vf8", 0b00011, 3),
    ];
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for &(name, sel, min_log2) in ops {
                if sew_log2 < min_log2 {
                    continue; // SEW too narrow for this extension factor
                }
                for k in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    // Source EMUL < 1 here, so the destination must not overlap the
                    // source register group: pick vs2 distinct from vd.
                    let mut vs2 = VPOOL[(rng.next() % 6) as usize];
                    if vs2 == vd {
                        vs2 = VPOOL[((rng.next() % 5) + 1) as usize % 6];
                        if vs2 == vd {
                            vs2 = if vd == VPOOL[0] { VPOOL[1] } else { VPOOL[0] };
                        }
                    }
                    let st = rand_vstate(&mut rng, sew_log2, vl);
                    batch.push((name.into(), op_iv(0b010010, 1, vs2, sel, 0b010, vd), st));
                    if vd != 0 && k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.m"),
                            op_iv(0b010010, 0, vs2, sel, 0b010, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_mask_ops() {
    let mut rng = Rng::new(0x7EC_790);
    let mut batch = Vec::new();
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1), 1] {
            for _ in 0..8 {
                let vd = VPOOL[(rng.next() % 6) as usize];
                // vd must not overlap vs2 for the set/iota ops.
                let mut vs2 = VPOOL[(rng.next() % 6) as usize];
                if vs2 == vd {
                    vs2 = if vd == VPOOL[0] { VPOOL[1] } else { VPOOL[0] };
                }
                let rd = XPOOL[(rng.next() % 5) as usize];
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                st.v[0] = rng.next(); // v0 mask + general source bits
                // vcpop.m / vfirst.m write x[rd].
                batch.push((
                    "vcpop.m".into(),
                    op_iv(0b010000, 1, vs2, 0b10000, 0b010, rd),
                    st,
                ));
                batch.push((
                    "vfirst.m".into(),
                    op_iv(0b010000, 1, vs2, 0b10001, 0b010, rd),
                    st,
                ));
                batch.push((
                    "vcpop.m.m".into(),
                    op_iv(0b010000, 0, vs2, 0b10000, 0b010, rd),
                    st,
                ));
                batch.push((
                    "vfirst.m.m".into(),
                    op_iv(0b010000, 0, vs2, 0b10001, 0b010, rd),
                    st,
                ));
                // vmsbf/vmsof/vmsif.m and viota.m / vid.v write a vector register.
                batch.push((
                    "vmsbf.m".into(),
                    op_iv(0b010100, 1, vs2, 0b00001, 0b010, vd),
                    st,
                ));
                batch.push((
                    "vmsof.m".into(),
                    op_iv(0b010100, 1, vs2, 0b00010, 0b010, vd),
                    st,
                ));
                batch.push((
                    "vmsif.m".into(),
                    op_iv(0b010100, 1, vs2, 0b00011, 0b010, vd),
                    st,
                ));
                batch.push((
                    "viota.m".into(),
                    op_iv(0b010100, 1, vs2, 0b10000, 0b010, vd),
                    st,
                ));
                batch.push((
                    "vid.v".into(),
                    op_iv(0b010100, 1, 0, 0b10001, 0b010, vd),
                    st,
                ));
                // Masked forms (vm=0, vd != v0); v0 supplies the active mask.
                if vd != 0 {
                    batch.push((
                        "vmsbf.m.m".into(),
                        op_iv(0b010100, 0, vs2, 0b00001, 0b010, vd),
                        st,
                    ));
                    batch.push((
                        "vmsif.m.m".into(),
                        op_iv(0b010100, 0, vs2, 0b00011, 0b010, vd),
                        st,
                    ));
                    batch.push((
                        "vmsof.m.m".into(),
                        op_iv(0b010100, 0, vs2, 0b00010, 0b010, vd),
                        st,
                    ));
                    batch.push((
                        "viota.m.m".into(),
                        op_iv(0b010100, 0, vs2, 0b10000, 0b010, vd),
                        st,
                    ));
                    batch.push((
                        "vid.v.m".into(),
                        op_iv(0b010100, 0, 0, 0b10001, 0b010, vd),
                        st,
                    ));
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_slide() {
    let mut rng = Rng::new(0x7EC_7A0);
    let mut batch = Vec::new();
    let fpool: [u32; 5] = [0, 1, 8, 15, 20];
    for sew_log2 in 0..4u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for k in 0..8 {
                let vd = VPOOL[(rng.next() % 6) as usize];
                let mut vs2 = VPOOL[(rng.next() % 6) as usize];
                if vs2 == vd {
                    vs2 = if vd == VPOOL[0] { VPOOL[1] } else { VPOOL[0] };
                }
                let rs1 = XPOOL[(rng.next() % 5) as usize];
                let fr = fpool[(rng.next() % 5) as usize];
                let imm = (rng.next() % (vmax + 2)) as u32; // small slide amount
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                fp_setup(&mut st, &mut rng, eb.max(2));
                st.x[rs1 as usize] = rng.next() % (vmax + 3); // small vx offset
                batch.push((
                    "vslideup.vx".into(),
                    op_iv(0b001110, 1, vs2, rs1, 0b100, vd),
                    st,
                ));
                batch.push((
                    "vslidedown.vx".into(),
                    op_iv(0b001111, 1, vs2, rs1, 0b100, vd),
                    st,
                ));
                batch.push((
                    "vslideup.vi".into(),
                    op_iv(0b001110, 1, vs2, imm, 0b011, vd),
                    st,
                ));
                batch.push((
                    "vslidedown.vi".into(),
                    op_iv(0b001111, 1, vs2, imm, 0b011, vd),
                    st,
                ));
                // slide-by-1 with scalar insertion (x[rs1] arbitrary here)
                let mut st2 = st;
                st2.x[rs1 as usize] = rng.next();
                batch.push((
                    "vslide1up.vx".into(),
                    op_iv(0b001110, 1, vs2, rs1, 0b110, vd),
                    st2,
                ));
                batch.push((
                    "vslide1down.vx".into(),
                    op_iv(0b001111, 1, vs2, rs1, 0b110, vd),
                    st2,
                ));
                if sew_log2 >= 1 {
                    // FP slide-by-1 valid only for SEW in {16,32,64}.
                    batch.push((
                        "vfslide1up.vf".into(),
                        op_iv(0b001110, 1, vs2, fr, 0b101, vd),
                        st2,
                    ));
                    batch.push((
                        "vfslide1down.vf".into(),
                        op_iv(0b001111, 1, vs2, fr, 0b101, vd),
                        st2,
                    ));
                }
                if vd != 0 && k % 2 == 0 {
                    let mut stm = st;
                    stm.v[0] = rng.next();
                    stm.v[1] = rng.next();
                    batch.push((
                        "vslideup.vx.m".into(),
                        op_iv(0b001110, 0, vs2, rs1, 0b100, vd),
                        stm,
                    ));
                    batch.push((
                        "vslidedown.vi.m".into(),
                        op_iv(0b001111, 0, vs2, imm, 0b011, vd),
                        stm,
                    ));
                    batch.push((
                        "vslide1up.vx.m".into(),
                        op_iv(0b001110, 0, vs2, rs1, 0b110, vd),
                        stm,
                    ));
                }
            }
        }
    }
    run_batch(&batch);
}

/// Fill register `vsreg`'s lanes with small indices (mod `vmax`+2, so both
/// in-range and just-out-of-range indices are exercised) at element width `ieb`.
fn fill_indices(st: &mut VState, vsreg: u32, ieb: usize, vmax: u64, rng: &mut Rng) {
    let mut bytes = [0u8; VLENB];
    let n = VLENB / ieb;
    for e in 0..n {
        let idx = (rng.next() % (vmax + 2)).to_le_bytes();
        bytes[e * ieb..e * ieb + ieb].copy_from_slice(&idx[..ieb]);
    }
    st.set_vreg_bytes(vsreg as usize, &bytes);
}

#[test]
fn diff_v_gather() {
    let mut rng = Rng::new(0x7EC_7B0);
    let mut batch = Vec::new();
    for sew_log2 in 0..4u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for k in 0..8 {
                // vd, vs2, vs1 all distinct (gather forbids dest/source overlap).
                let vd = VPOOL[(rng.next() % 6) as usize];
                let mut vs2 = VPOOL[(rng.next() % 6) as usize];
                while vs2 == vd {
                    vs2 = VPOOL[(rng.next() % 6) as usize];
                }
                let mut vs1 = VPOOL[(rng.next() % 6) as usize];
                while vs1 == vd || vs1 == vs2 {
                    vs1 = VPOOL[(rng.next() % 6) as usize];
                }
                let rs1 = XPOOL[(rng.next() % 5) as usize];
                let imm = (rng.next() % (vmax + 2)) as u32;
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                fill_indices(&mut st, vs1, eb, vmax, &mut rng);
                st.x[rs1 as usize] = rng.next() % (vmax + 2);
                batch.push((
                    "vrgather.vv".into(),
                    op_iv(0b001100, 1, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vrgather.vx".into(),
                    op_iv(0b001100, 1, vs2, rs1, 0b100, vd),
                    st,
                ));
                batch.push((
                    "vrgather.vi".into(),
                    op_iv(0b001100, 1, vs2, imm, 0b011, vd),
                    st,
                ));
                // vrgatherei16: 16-bit indices. At SEW=8 the index EMUL is 2
                // (two-register group); restrict to SEW>=16 so vs1 is one register.
                if sew_log2 >= 1 {
                    let mut st16 = rand_vstate(&mut rng, sew_log2, vl);
                    fill_indices(&mut st16, vs1, 2, vmax, &mut rng);
                    batch.push((
                        "vrgatherei16.vv".into(),
                        op_iv(0b001110, 1, vs2, vs1, 0b000, vd),
                        st16,
                    ));
                }
                if vd != 0 && k % 2 == 0 {
                    let mut stm = st;
                    stm.v[0] = rng.next();
                    stm.v[1] = rng.next();
                    // keep index lanes small after perturbing v0/v1
                    fill_indices(&mut stm, vs1, eb, vmax, &mut rng);
                    batch.push((
                        "vrgather.vv.m".into(),
                        op_iv(0b001100, 0, vs2, vs1, 0b000, vd),
                        stm,
                    ));
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_compress() {
    let mut rng = Rng::new(0x7EC_7C0);
    let mut batch = Vec::new();
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1), 1] {
            for _ in 0..10 {
                // vd distinct from vs2 and the vs1 mask source.
                let vd = VPOOL[(rng.next() % 6) as usize];
                let mut vs2 = VPOOL[(rng.next() % 6) as usize];
                while vs2 == vd {
                    vs2 = VPOOL[(rng.next() % 6) as usize];
                }
                let mut vs1 = VPOOL[(rng.next() % 6) as usize];
                while vs1 == vd || vs1 == vs2 {
                    vs1 = VPOOL[(rng.next() % 6) as usize];
                }
                let st = rand_vstate(&mut rng, sew_log2, vl);
                // vcompress.vm is always unmasked (vm=1).
                batch.push((
                    "vcompress.vm".into(),
                    op_iv(0b010111, 1, vs2, vs1, 0b010, vd),
                    st,
                ));
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_carry() {
    let mut rng = Rng::new(0x7EC_7D0);
    let mut batch = Vec::new();
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for _ in 0..8 {
                let vd = VPOOL[(rng.next() % 6) as usize]; // != v0 (carry source)
                let vs2 = VPOOL[(rng.next() % 6) as usize];
                let vs1 = VPOOL[(rng.next() % 6) as usize];
                let rs1 = XPOOL[(rng.next() % 5) as usize];
                let imm = (rng.next() & 0x1f) as u32;
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                st.v[0] = rng.next(); // v0 carry-in bits
                st.v[1] = rng.next();
                // vadc (write data) — vvm/vxm/vim, vm=0.
                batch.push((
                    "vadc.vvm".into(),
                    op_iv(0b010000, 0, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vadc.vxm".into(),
                    op_iv(0b010000, 0, vs2, rs1, 0b100, vd),
                    st,
                ));
                batch.push((
                    "vadc.vim".into(),
                    op_iv(0b010000, 0, vs2, imm, 0b011, vd),
                    st,
                ));
                batch.push((
                    "vsbc.vvm".into(),
                    op_iv(0b010010, 0, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vsbc.vxm".into(),
                    op_iv(0b010010, 0, vs2, rs1, 0b100, vd),
                    st,
                ));
                // vmadc/vmsbc (write mask) — with carry (vm=0) and without (vm=1).
                batch.push((
                    "vmadc.vvm".into(),
                    op_iv(0b010001, 0, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vmadc.vv".into(),
                    op_iv(0b010001, 1, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vmadc.vim".into(),
                    op_iv(0b010001, 0, vs2, imm, 0b011, vd),
                    st,
                ));
                batch.push((
                    "vmadc.vi".into(),
                    op_iv(0b010001, 1, vs2, imm, 0b011, vd),
                    st,
                ));
                batch.push((
                    "vmsbc.vvm".into(),
                    op_iv(0b010011, 0, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vmsbc.vv".into(),
                    op_iv(0b010011, 1, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vmadc.vxm".into(),
                    op_iv(0b010001, 0, vs2, rs1, 0b100, vd),
                    st,
                ));
                batch.push((
                    "vmsbc.vx".into(),
                    op_iv(0b010011, 1, vs2, rs1, 0b100, vd),
                    st,
                ));
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_satadd() {
    let mut rng = Rng::new(0x7EC_7E0);
    let mut batch = Vec::new();
    let ops: &[(&str, u32, bool, bool)] = &[
        // (name, funct6, has_vi, signed)
        ("vsaddu", 0b100000, true, false),
        ("vsadd", 0b100001, true, true),
        ("vssubu", 0b100010, false, false),
        ("vssub", 0b100011, false, true),
    ];
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for &(name, f6, has_vi, _signed) in ops {
                for k in 0..6 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let rs1 = XPOOL[(rng.next() % 5) as usize];
                    let imm = (rng.next() & 0x1f) as u32;
                    let mut st = rand_vstate(&mut rng, sew_log2, vl);
                    // Start vxsat = 0 sometimes, 1 sometimes, to verify stickiness.
                    st.vcsr = (rng.next() & 1) | ((rng.next() & 3) << 1);
                    batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b000, vd), st));
                    batch.push((format!("{name}.vx"), op_iv(f6, 1, vs2, rs1, 0b100, vd), st));
                    if has_vi {
                        batch.push((format!("{name}.vi"), op_iv(f6, 1, vs2, imm, 0b011, vd), st));
                    }
                    if vd != 0 && k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vv.m"),
                            op_iv(f6, 0, vs2, vs1, 0b000, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_avg() {
    let mut rng = Rng::new(0x7EC_7F0);
    let mut batch = Vec::new();
    let ops: &[(&str, u32)] = &[
        ("vaaddu", 0b001000),
        ("vaadd", 0b001001),
        ("vasubu", 0b001010),
        ("vasub", 0b001011),
    ];
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for vxrm in 0..4u64 {
                for &(name, f6) in ops {
                    for k in 0..4 {
                        let vd = VPOOL[(rng.next() % 6) as usize];
                        let vs2 = VPOOL[(rng.next() % 6) as usize];
                        let vs1 = VPOOL[(rng.next() % 6) as usize];
                        let rs1 = XPOOL[(rng.next() % 5) as usize];
                        let mut st = rand_vstate(&mut rng, sew_log2, vl);
                        st.vcsr = vxrm << 1; // vxrm in [2:1], vxsat = 0
                        batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b010, vd), st));
                        batch.push((format!("{name}.vx"), op_iv(f6, 1, vs2, rs1, 0b110, vd), st));
                        if vd != 0 && k % 2 == 0 {
                            let mut stm = st;
                            stm.v[0] = rng.next();
                            stm.v[1] = rng.next();
                            batch.push((
                                format!("{name}.vv.m"),
                                op_iv(f6, 0, vs2, vs1, 0b010, vd),
                                stm,
                            ));
                        }
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_scale() {
    let mut rng = Rng::new(0x7EC_800);
    let mut batch = Vec::new();
    for sew_log2 in 0..4u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for vxrm in 0..4u64 {
                for k in 0..4 {
                    let vd = VPOOL[(rng.next() % 6) as usize];
                    let vs2 = VPOOL[(rng.next() % 6) as usize];
                    let vs1 = VPOOL[(rng.next() % 6) as usize];
                    let rs1 = XPOOL[(rng.next() % 5) as usize];
                    let imm = (rng.next() & 0x1f) as u32;
                    let mut st = rand_vstate(&mut rng, sew_log2, vl);
                    st.vcsr = (vxrm << 1) | (rng.next() & 1);
                    // vssrl / vssra: vv/vx/vi
                    batch.push((
                        "vssrl.vv".into(),
                        op_iv(0b101010, 1, vs2, vs1, 0b000, vd),
                        st,
                    ));
                    batch.push((
                        "vssrl.vx".into(),
                        op_iv(0b101010, 1, vs2, rs1, 0b100, vd),
                        st,
                    ));
                    batch.push((
                        "vssrl.vi".into(),
                        op_iv(0b101010, 1, vs2, imm, 0b011, vd),
                        st,
                    ));
                    batch.push((
                        "vssra.vv".into(),
                        op_iv(0b101011, 1, vs2, vs1, 0b000, vd),
                        st,
                    ));
                    batch.push((
                        "vssra.vx".into(),
                        op_iv(0b101011, 1, vs2, rs1, 0b100, vd),
                        st,
                    ));
                    batch.push((
                        "vssra.vi".into(),
                        op_iv(0b101011, 1, vs2, imm, 0b011, vd),
                        st,
                    ));
                    // vsmul: vv/vx (signed fractional multiply, saturating)
                    batch.push((
                        "vsmul.vv".into(),
                        op_iv(0b100111, 1, vs2, vs1, 0b000, vd),
                        st,
                    ));
                    batch.push((
                        "vsmul.vx".into(),
                        op_iv(0b100111, 1, vs2, rs1, 0b100, vd),
                        st,
                    ));
                    if vd != 0 && k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            "vssra.vv.m".into(),
                            op_iv(0b101011, 0, vs2, vs1, 0b000, vd),
                            stm,
                        ));
                        batch.push((
                            "vsmul.vv.m".into(),
                            op_iv(0b100111, 0, vs2, vs1, 0b000, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

// Disjoint (vd-group, vs2-group, vs1) register triples for widening ops, where
// the destination and `.w` source span two registers each.
const WIDEN_REGS: [(u32, u32, u32); 4] = [(2, 8, 12), (4, 16, 20), (6, 24, 28), (10, 14, 18)];

#[test]
fn diff_v_widen_add() {
    let mut rng = Rng::new(0x7EC_810);
    let mut batch = Vec::new();
    let ops: &[(&str, u32)] = &[
        ("vwaddu", 0b110000),
        ("vwadd", 0b110001),
        ("vwsubu", 0b110010),
        ("vwsub", 0b110011),
        ("vwaddu.w", 0b110100),
        ("vwadd.w", 0b110101),
        ("vwsubu.w", 0b110110),
        ("vwsub.w", 0b110111),
    ];
    for sew_log2 in 0..3u32 {
        // widening valid only for SEW <= 32
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for &(name, f6) in ops {
                for k in 0..4 {
                    let (vd, vs2, vs1) = WIDEN_REGS[(rng.next() % 4) as usize];
                    let rs1 = XPOOL[(rng.next() % 5) as usize];
                    let st = rand_vstate(&mut rng, sew_log2, vl);
                    batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b010, vd), st));
                    batch.push((format!("{name}.vx"), op_iv(f6, 1, vs2, rs1, 0b110, vd), st));
                    if k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vv.m"),
                            op_iv(f6, 0, vs2, vs1, 0b010, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_widen_mul() {
    let mut rng = Rng::new(0x7EC_820);
    let mut batch = Vec::new();
    // (name, funct6, has_vv, has_vx)
    let ops: &[(&str, u32, bool, bool)] = &[
        ("vwmulu", 0b111000, true, true),
        ("vwmulsu", 0b111010, true, true),
        ("vwmul", 0b111011, true, true),
        ("vwmaccu", 0b111100, true, true),
        ("vwmacc", 0b111101, true, true),
        ("vwmaccus", 0b111110, false, true), // vx only
        ("vwmaccsu", 0b111111, true, true),
    ];
    for sew_log2 in 0..3u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for &(name, f6, has_vv, has_vx) in ops {
                for k in 0..4 {
                    let (vd, vs2, vs1) = WIDEN_REGS[(rng.next() % 4) as usize];
                    let rs1 = XPOOL[(rng.next() % 5) as usize];
                    let st = rand_vstate(&mut rng, sew_log2, vl);
                    if has_vv {
                        batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b010, vd), st));
                    }
                    if has_vx {
                        batch.push((format!("{name}.vx"), op_iv(f6, 1, vs2, rs1, 0b110, vd), st));
                    }
                    if has_vv && k % 2 == 0 {
                        let mut stm = st;
                        stm.v[0] = rng.next();
                        stm.v[1] = rng.next();
                        batch.push((
                            format!("{name}.vv.m"),
                            op_iv(f6, 0, vs2, vs1, 0b010, vd),
                            stm,
                        ));
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_narrow() {
    let mut rng = Rng::new(0x7EC_830);
    let mut batch = Vec::new();
    // (name, funct6, is_clip)
    let ops: &[(&str, u32, bool)] = &[
        ("vnsrl", 0b101100, false),
        ("vnsra", 0b101101, false),
        ("vnclipu", 0b101110, true),
        ("vnclip", 0b101111, true),
    ];
    for sew_log2 in 0..3u32 {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for vxrm in 0..4u64 {
                for &(name, f6, is_clip) in ops {
                    for k in 0..3 {
                        // (vd narrow, vs2 wide group, vs1 shift)
                        let (vd, vs2, vs1) = WIDEN_REGS[(rng.next() % 4) as usize];
                        let rs1 = XPOOL[(rng.next() % 5) as usize];
                        let imm = (rng.next() & 0x1f) as u32;
                        let mut st = rand_vstate(&mut rng, sew_log2, vl);
                        if is_clip {
                            st.vcsr = (vxrm << 1) | (rng.next() & 1);
                        }
                        batch.push((format!("{name}.wv"), op_iv(f6, 1, vs2, vs1, 0b000, vd), st));
                        batch.push((format!("{name}.wx"), op_iv(f6, 1, vs2, rs1, 0b100, vd), st));
                        batch.push((format!("{name}.wi"), op_iv(f6, 1, vs2, imm, 0b011, vd), st));
                        if k % 2 == 0 {
                            let mut stm = st;
                            stm.v[0] = rng.next();
                            stm.v[1] = rng.next();
                            batch.push((
                                format!("{name}.wv.m"),
                                op_iv(f6, 0, vs2, vs1, 0b000, vd),
                                stm,
                            ));
                        }
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_fcvt() {
    let mut rng = Rng::new(0x7EC_840);
    let mut batch = Vec::new();
    // (name, vs1-selector)
    let ops: &[(&str, u32)] = &[
        ("vfcvt.xu.f.v", 0b00000),
        ("vfcvt.x.f.v", 0b00001),
        ("vfcvt.f.xu.v", 0b00010),
        ("vfcvt.f.x.v", 0b00011),
        ("vfcvt.rtz.xu.f.v", 0b00110),
        ("vfcvt.rtz.x.f.v", 0b00111),
    ];
    for sew_log2 in 1..4u32 {
        // FP widths only: SEW 16/32/64
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for frm in 0..5u64 {
                for &(name, sel) in ops {
                    for k in 0..3 {
                        let vd = VPOOL[(rng.next() % 6) as usize];
                        let vs2 = VPOOL[(rng.next() % 6) as usize];
                        let mut st = rand_vstate(&mut rng, sew_log2, vl);
                        st.fcsr = frm << 5;
                        fp_setup(&mut st, &mut rng, eb);
                        batch.push((name.into(), op_iv(0b010010, 1, vs2, sel, 0b001, vd), st));
                        if vd != 0 && k % 2 == 0 {
                            let mut stm = st;
                            stm.v[0] = rng.next();
                            stm.v[1] = rng.next();
                            batch.push((
                                format!("{name}.m"),
                                op_iv(0b010010, 0, vs2, sel, 0b001, vd),
                                stm,
                            ));
                        }
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_fwcvt() {
    let mut rng = Rng::new(0x7EC_850);
    let mut batch = Vec::new();
    let ops: &[(&str, u32)] = &[
        ("vfwcvt.xu.f.v", 0b01000),
        ("vfwcvt.x.f.v", 0b01001),
        ("vfwcvt.f.xu.v", 0b01010),
        ("vfwcvt.f.x.v", 0b01011),
        ("vfwcvt.f.f.v", 0b01100),
        ("vfwcvt.rtz.xu.f.v", 0b01110),
        ("vfwcvt.rtz.x.f.v", 0b01111),
    ];
    for sew_log2 in 1..3u32 {
        // source SEW 16/32 -> dest 32/64
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for frm in 0..5u64 {
                for &(name, sel) in ops {
                    for k in 0..2 {
                        // (vd = 2*SEW group, vs2 = SEW source)
                        let (vd, vs2, _) = WIDEN_REGS[(rng.next() % 4) as usize];
                        let mut st = rand_vstate(&mut rng, sew_log2, vl);
                        st.fcsr = frm << 5;
                        // Seed the source register with assorted FP/int patterns.
                        fp_setup(&mut st, &mut rng, eb);
                        st.v[vs2 as usize * 2] = rng.next();
                        st.v[vs2 as usize * 2 + 1] = rng.next();
                        batch.push((name.into(), op_iv(0b010010, 1, vs2, sel, 0b001, vd), st));
                        if k == 0 {
                            let mut stm = st;
                            stm.v[0] = rng.next();
                            stm.v[1] = rng.next();
                            batch.push((
                                format!("{name}.m"),
                                op_iv(0b010010, 0, vs2, sel, 0b001, vd),
                                stm,
                            ));
                        }
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_fncvt() {
    let mut rng = Rng::new(0x7EC_860);
    let mut batch = Vec::new();
    let ops: &[(&str, u32)] = &[
        ("vfncvt.xu.f.w", 0b10000),
        ("vfncvt.x.f.w", 0b10001),
        ("vfncvt.f.xu.w", 0b10010),
        ("vfncvt.f.x.w", 0b10011),
        ("vfncvt.f.f.w", 0b10100),
        ("vfncvt.rod.f.f.w", 0b10101),
        ("vfncvt.rtz.xu.f.w", 0b10110),
        ("vfncvt.rtz.x.f.w", 0b10111),
    ];
    for sew_log2 in 1..3u32 {
        // dest SEW 16/32, source 2*SEW = 32/64
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for frm in 0..5u64 {
                for &(name, sel) in ops {
                    for k in 0..2 {
                        // (vd = SEW dest, vs2 = 2*SEW wide source group)
                        let (vd, vs2, _) = WIDEN_REGS[(rng.next() % 4) as usize];
                        let mut st = rand_vstate(&mut rng, sew_log2, vl);
                        st.fcsr = frm << 5;
                        fp_setup(&mut st, &mut rng, eb * 2);
                        st.v[vs2 as usize * 2] = rng.next();
                        st.v[vs2 as usize * 2 + 1] = rng.next();
                        batch.push((name.into(), op_iv(0b010010, 1, vs2, sel, 0b001, vd), st));
                        if k == 0 && vd != 0 {
                            let mut stm = st;
                            stm.v[0] = rng.next();
                            stm.v[1] = rng.next();
                            batch.push((
                                format!("{name}.m"),
                                op_iv(0b010010, 0, vs2, sel, 0b001, vd),
                                stm,
                            ));
                        }
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_fwarith() {
    let mut rng = Rng::new(0x7EC_870);
    let mut batch = Vec::new();
    // (name, funct6, is_w)
    let ops: &[(&str, u32, bool)] = &[
        ("vfwadd", 0b110000, false),
        ("vfwsub", 0b110010, false),
        ("vfwmul", 0b111000, false),
        ("vfwadd.w", 0b110100, true),
        ("vfwsub.w", 0b110110, true),
    ];
    for sew_log2 in 1..3u32 {
        // SEW 16/32 -> 2*SEW 32/64
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for frm in 0..5u64 {
                for &(name, f6, _is_w) in ops {
                    for k in 0..2 {
                        let (vd, vs2, vs1) = WIDEN_REGS[(rng.next() % 4) as usize];
                        let fr = [0u32, 1, 8, 15][(rng.next() % 4) as usize];
                        let mut st = rand_vstate(&mut rng, sew_log2, vl);
                        st.fcsr = frm << 5;
                        fp_setup(&mut st, &mut rng, eb);
                        for r in [vs2, vs1] {
                            st.v[r as usize * 2] = rng.next();
                            st.v[r as usize * 2 + 1] = rng.next();
                        }
                        batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b001, vd), st));
                        batch.push((format!("{name}.vf"), op_iv(f6, 1, vs2, fr, 0b101, vd), st));
                        if k == 0 {
                            let mut stm = st;
                            stm.v[0] = rng.next();
                            stm.v[1] = rng.next();
                            batch.push((
                                format!("{name}.vv.m"),
                                op_iv(f6, 0, vs2, vs1, 0b001, vd),
                                stm,
                            ));
                        }
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_fwmac() {
    let mut rng = Rng::new(0x7EC_880);
    let mut batch = Vec::new();
    let ops: &[(&str, u32)] = &[
        ("vfwmacc", 0b111100),
        ("vfwnmacc", 0b111101),
        ("vfwmsac", 0b111110),
        ("vfwnmsac", 0b111111),
    ];
    for sew_log2 in 1..3u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for frm in 0..5u64 {
                for &(name, f6) in ops {
                    for k in 0..2 {
                        let (vd, vs2, vs1) = WIDEN_REGS[(rng.next() % 4) as usize];
                        let fr = [0u32, 1, 8, 15][(rng.next() % 4) as usize];
                        let mut st = rand_vstate(&mut rng, sew_log2, vl);
                        st.fcsr = frm << 5;
                        fp_setup(&mut st, &mut rng, eb);
                        for r in [vs2, vs1, vd] {
                            st.v[r as usize * 2] = rng.next();
                            st.v[r as usize * 2 + 1] = rng.next();
                        }
                        batch.push((format!("{name}.vv"), op_iv(f6, 1, vs2, vs1, 0b001, vd), st));
                        batch.push((format!("{name}.vf"), op_iv(f6, 1, vs2, fr, 0b101, vd), st));
                        if k == 0 {
                            let mut stm = st;
                            stm.v[0] = rng.next();
                            stm.v[1] = rng.next();
                            batch.push((
                                format!("{name}.vv.m"),
                                op_iv(f6, 0, vs2, vs1, 0b001, vd),
                                stm,
                            ));
                        }
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_wredux() {
    let mut rng = Rng::new(0x7EC_890);
    let mut batch = Vec::new();
    for sew_log2 in 1..3u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1), 1] {
            for frm in 0..5u64 {
                // distinct vd / vs2 / vs1
                let vd = VPOOL[(rng.next() % 6) as usize];
                let mut vs2 = VPOOL[(rng.next() % 6) as usize];
                while vs2 == vd {
                    vs2 = VPOOL[(rng.next() % 6) as usize];
                }
                let mut vs1 = VPOOL[(rng.next() % 6) as usize];
                while vs1 == vd || vs1 == vs2 {
                    vs1 = VPOOL[(rng.next() % 6) as usize];
                }
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                st.fcsr = frm << 5;
                fp_setup(&mut st, &mut rng, eb);
                for r in [vs2, vs1] {
                    st.v[r as usize * 2] = rng.next();
                    st.v[r as usize * 2 + 1] = rng.next();
                }
                // Integer widening sum reductions (.vs).
                batch.push((
                    "vwredsumu.vs".into(),
                    op_iv(0b110000, 1, vs2, vs1, 0b000, vd),
                    st,
                ));
                batch.push((
                    "vwredsum.vs".into(),
                    op_iv(0b110001, 1, vs2, vs1, 0b000, vd),
                    st,
                ));
                // FP widening sum reductions (.vs).
                batch.push((
                    "vfwredusum.vs".into(),
                    op_iv(0b110001, 1, vs2, vs1, 0b001, vd),
                    st,
                ));
                batch.push((
                    "vfwredosum.vs".into(),
                    op_iv(0b110011, 1, vs2, vs1, 0b001, vd),
                    st,
                ));
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_loadstore_adv() {
    let mut rng = Rng::new(0x7EC_8A0);
    let mut batch = Vec::new();
    let widths: [(u32, u32); 4] = [(0, 0), (5, 1), (6, 2), (7, 3)];
    for (w3, sew_log2) in widths {
        let eb = 1u64 << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for _ in 0..6 {
                let vd = VPOOL[(rng.next() % 6) as usize];
                let idxreg = if vd == 2 { 3 } else { 2 }; // index vreg distinct from vd
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                st.x[10] = SCRATCH_BASE; // base address
                st.x[11] = eb * 2; // strided byte-stride (rs2 = x11)
                for s in st.scratch.iter_mut() {
                    *s = rng.next();
                }
                // Fill the index register with in-window byte offsets (multiples of eb).
                let mut ib = [0u8; VLENB];
                let n = VLENB / eb as usize;
                for e in 0..n {
                    let off = ((rng.next() % (16 - eb).max(1)) * eb) as u64;
                    ib[e * eb as usize..(e + 1) * eb as usize]
                        .copy_from_slice(&off.to_le_bytes()[..eb as usize]);
                }
                st.set_vreg_bytes(idxreg as usize, &ib);

                // strided
                let vlse = (0b10 << 26)
                    | (1 << 25)
                    | (11 << 20)
                    | (10 << 15)
                    | (w3 << 12)
                    | (vd << 7)
                    | 0x07;
                let vsse = (0b10 << 26)
                    | (1 << 25)
                    | (11 << 20)
                    | (10 << 15)
                    | (w3 << 12)
                    | (vd << 7)
                    | 0x27;
                batch.push(("vlse".into(), vlse, st));
                batch.push(("vsse".into(), vsse, st));
                // indexed (unordered mop=01); index EEW = data width here
                let vlxei = (0b01 << 26)
                    | (1 << 25)
                    | (idxreg << 20)
                    | (10 << 15)
                    | (w3 << 12)
                    | (vd << 7)
                    | 0x07;
                let vsxei = (0b01 << 26)
                    | (1 << 25)
                    | (idxreg << 20)
                    | (10 << 15)
                    | (w3 << 12)
                    | (vd << 7)
                    | 0x27;
                batch.push(("vlxei".into(), vlxei, st));
                batch.push(("vsxei".into(), vsxei, st));
                // fault-only-first load (lumop=10000); scratch never faults, so it
                // must match plain vle and leave vl unchanged.
                let vleff =
                    (1 << 25) | (0b10000 << 20) | (10 << 15) | (w3 << 12) | (vd << 7) | 0x07;
                batch.push(("vleff".into(), vleff, st));
                // mask load/store (lumop=01011, width 0)
                let vlm = (1 << 25) | (0b01011 << 20) | (10 << 15) | (vd << 7) | 0x07;
                let vsm = (1 << 25) | (0b01011 << 20) | (10 << 15) | (vd << 7) | 0x27;
                batch.push(("vlm.v".into(), vlm, st));
                batch.push(("vsm.v".into(), vsm, st));
            }
        }
    }
    // whole-register load/store (nf+1 regs), independent of vtype.
    for nf in [0u32, 1] {
        for _ in 0..6 {
            let vd = if nf == 0 {
                VPOOL[(rng.next() % 6) as usize]
            } else {
                2
            };
            let mut st = rand_vstate(&mut rng, 0, 16);
            st.x[10] = SCRATCH_BASE;
            for s in st.scratch.iter_mut() {
                *s = rng.next();
            }
            let vlre = (nf << 29) | (1 << 25) | (0b01000 << 20) | (10 << 15) | (vd << 7) | 0x07;
            let vsre = (nf << 29) | (1 << 25) | (0b01000 << 20) | (10 << 15) | (vd << 7) | 0x27;
            batch.push(("vlre.v".into(), vlre, st));
            batch.push(("vsre.v".into(), vsre, st));
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_class_mvr() {
    let mut rng = Rng::new(0x7EC_8B0);
    let mut batch = Vec::new();
    // vfclass.v across SEW 16/32/64
    for sew_log2 in 1..4u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for k in 0..6 {
                let vd = VPOOL[(rng.next() % 6) as usize];
                let vs2 = VPOOL[(rng.next() % 6) as usize];
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                fp_setup(&mut st, &mut rng, eb);
                st.v[vs2 as usize * 2] = rng.next();
                st.v[vs2 as usize * 2 + 1] = rng.next();
                batch.push((
                    "vfclass.v".into(),
                    op_iv(0b010011, 1, vs2, 0b10000, 0b001, vd),
                    st,
                ));
                if vd != 0 && k % 2 == 0 {
                    let mut stm = st;
                    stm.v[0] = rng.next();
                    stm.v[1] = rng.next();
                    batch.push((
                        "vfclass.v.m".into(),
                        op_iv(0b010011, 0, vs2, 0b10000, 0b001, vd),
                        stm,
                    ));
                }
            }
        }
    }
    // vmv<nr>r.v whole-register move (simm = nreg-1 in {0,1,3,7}).
    for &simm in &[0u32, 1, 3, 7] {
        for _ in 0..4 {
            let st = rand_vstate(&mut rng, 0, 16);
            // vd=8, vs2=16 are aligned to 8, valid for all nreg.
            batch.push((
                format!("vmv{}r.v", simm + 1),
                op_iv(0b100111, 1, 16, simm, 0b011, 8),
                st,
            ));
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_estimate() {
    let mut rng = Rng::new(0x7EC_8C0);
    let mut batch = Vec::new();
    let ops: &[(&str, u32)] = &[("vfrsqrt7.v", 0b00100), ("vfrec7.v", 0b00101)];
    for sew_log2 in 1..4u32 {
        let eb = 1usize << sew_log2;
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for frm in 0..5u64 {
                for &(name, sel) in ops {
                    for k in 0..4 {
                        let vd = VPOOL[(rng.next() % 6) as usize];
                        let vs2 = VPOOL[(rng.next() % 6) as usize];
                        let mut st = rand_vstate(&mut rng, sew_log2, vl);
                        st.fcsr = frm << 5;
                        fp_setup(&mut st, &mut rng, eb);
                        st.v[vs2 as usize * 2] = rng.next();
                        st.v[vs2 as usize * 2 + 1] = rng.next();
                        batch.push((name.into(), op_iv(0b010011, 1, vs2, sel, 0b001, vd), st));
                        if vd != 0 && k % 2 == 0 {
                            let mut stm = st;
                            stm.v[0] = rng.next();
                            stm.v[1] = rng.next();
                            batch.push((
                                format!("{name}.m"),
                                op_iv(0b010011, 0, vs2, sel, 0b001, vd),
                                stm,
                            ));
                        }
                    }
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_segment() {
    let mut rng = Rng::new(0x7EC_8D0);
    let mut batch = Vec::new();
    let widths: [(u32, u32); 3] = [(0, 0), (5, 1), (6, 2)]; // eb 1/2/4 (fit window)
    for (w3, sew_log2) in widths {
        let eb = 1u64 << sew_log2;
        for nf in 2..=4u32 {
            let nf_field = nf - 1;
            let vmax = vlmax(sew_log2);
            // keep vl * nf * eb within the 256-byte scratch window
            let maxvl = (192 / (nf as u64 * eb)).min(vmax).max(1);
            for vl in [maxvl, (maxvl / 2).max(1)] {
                for _ in 0..3 {
                    let vd = 8u32; // fields 8..8+nf-1
                    let idxreg = 2u32;
                    let mut st = rand_vstate(&mut rng, sew_log2, vl);
                    st.x[10] = SCRATCH_BASE;
                    st.x[11] = nf as u64 * eb; // strided element stride = segment size
                    for s in st.scratch.iter_mut() {
                        *s = rng.next();
                    }
                    // index register: contiguous segment byte offsets (in-window)
                    let mut ib = [0u8; VLENB];
                    for e in 0..(VLENB / eb as usize) {
                        let off = (e as u64 * nf as u64 * eb) % 192;
                        ib[e * eb as usize..(e + 1) * eb as usize]
                            .copy_from_slice(&off.to_le_bytes()[..eb as usize]);
                    }
                    st.set_vreg_bytes(idxreg as usize, &ib);

                    let seg = |mop: u32, rs2: u32, op: u32| {
                        (nf_field << 29)
                            | (mop << 26)
                            | (1 << 25)
                            | (rs2 << 20)
                            | (10 << 15)
                            | (w3 << 12)
                            | (vd << 7)
                            | op
                    };
                    batch.push(("vlseg".into(), seg(0b00, 0, 0x07), st)); // unit-stride
                    batch.push(("vsseg".into(), seg(0b00, 0, 0x27), st));
                    batch.push(("vlsseg".into(), seg(0b10, 11, 0x07), st)); // strided
                    batch.push(("vssseg".into(), seg(0b10, 11, 0x27), st));
                    batch.push(("vlxseg".into(), seg(0b01, idxreg, 0x07), st)); // indexed
                    batch.push(("vsxseg".into(), seg(0b01, idxreg, 0x27), st));
                }
            }
        }
    }
    run_batch(&batch);
}

#[test]
fn diff_v_loadstore() {
    let mut rng = Rng::new(0x7EC_705);
    let mut batch = Vec::new();
    // width funct3: 0=8,5=16,6=32,7=64; matching SEW so EEW==SEW.
    let widths: [(u32, u32); 4] = [(0, 0), (5, 1), (6, 2), (7, 3)];
    for (w3, sew_log2) in widths {
        let vmax = vlmax(sew_log2);
        for vl in [vmax, (vmax / 2).max(1)] {
            for _ in 0..15 {
                let vd = VPOOL[(rng.next() % 6) as usize];
                let mut st = rand_vstate(&mut rng, sew_log2, vl);
                st.x[10] = SCRATCH_BASE;
                for s in st.scratch.iter_mut() {
                    *s = rng.next();
                }
                // vle{w}.v vd, (x10)  : opcode 0x07, funct3=w3, vm=1, lumop=0
                let vle = (1u32 << 25) | (10 << 15) | (w3 << 12) | (vd << 7) | 0x07;
                batch.push(("vle".into(), vle, st));
                // vse{w}.v vd, (x10)  : opcode 0x27
                let vse = (1u32 << 25) | (10 << 15) | (w3 << 12) | (vd << 7) | 0x27;
                batch.push(("vse".into(), vse, st));
            }
        }
    }
    run_batch(&batch);
}
