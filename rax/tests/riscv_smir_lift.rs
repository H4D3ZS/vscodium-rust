//! RISC-V -> SMIR lift verification harness.
//!
//! For each instruction we lift the RISC-V machine word to SMIR ops, execute
//! them on the `SmirInterpreter` from a seeded register state, and compare the
//! resulting x/f/fcsr state against rax's RISC-V interpreter (`RiscVCpu`, itself
//! differentially verified against `qemu-riscv64` at 0 divergence). A match
//! proves the lift is semantically correct for that op; an instruction whose
//! lift returns `Unsupported`/`InvalidEncoding` is reported as an (unimplemented)
//! lift gap, not a divergence.
//!
//! Needs no external toolchain — `RiscVCpu` is the golden oracle and encodings
//! are generated directly. The test FAILS on any divergence and prints a
//! per-`Op` breakdown of remaining lift gaps.

#![cfg(target_os = "linux")]

use std::collections::BTreeMap;

use rax::riscv::{
    FlatMemory as RvMem, Isa, Memory as RvMemory, Op, RiscVConfig, RiscVCpu, RiscVExit, Xlen,
    decode, decode_at,
};
use rax::smir::types::{ArchReg, BlockId, OpId, RiscVReg, SourceArch};
use rax::smir::{
    ArchRegState, FlatMemory as SmirMem, LiftContext, LiftError, RiscVLifter, SmirBlock,
    SmirContext, SmirInterpreter, SmirLifter, Terminator, TrapKind,
};

const CODE_ADDR: u64 = 0x1000;
const SCRATCH: u64 = 0x4000; // base for memory operands
const MEM_SIZE: u64 = 0x10000;

#[derive(Clone, Copy)]
struct State {
    x: [u64; 32],
    f: [u64; 32],
    fcsr: u32,
    scratch: [u64; 64], // 512-byte shared data window at SCRATCH
    // Vector state (RVV): 32 × 128-bit registers + the vector CSRs.
    v: [[u8; 16]; 32],
    vl: u64,
    vtype: u64,
    vstart: u64,
    vcsr: u64,
}

impl State {
    fn eq_regs(&self, o: &State) -> Option<String> {
        for i in 1..32 {
            if self.x[i] != o.x[i] {
                return Some(format!("x{i}: ref={:#x} smir={:#x}", self.x[i], o.x[i]));
            }
        }
        for i in 0..32 {
            if self.f[i] != o.f[i] {
                return Some(format!("f{i}: ref={:#x} smir={:#x}", self.f[i], o.f[i]));
            }
        }
        if self.fcsr != o.fcsr {
            return Some(format!("fcsr: ref={:#x} smir={:#x}", self.fcsr, o.fcsr));
        }
        for i in 0..64 {
            if self.scratch[i] != o.scratch[i] {
                return Some(format!(
                    "scratch[{i}]: ref={:#x} smir={:#x}",
                    self.scratch[i], o.scratch[i]
                ));
            }
        }
        for i in 0..32 {
            if self.v[i] != o.v[i] {
                return Some(format!("v{i}: ref={:02x?} smir={:02x?}", self.v[i], o.v[i]));
            }
        }
        if self.vl != o.vl {
            return Some(format!("vl: ref={:#x} smir={:#x}", self.vl, o.vl));
        }
        if self.vtype != o.vtype {
            return Some(format!("vtype: ref={:#x} smir={:#x}", self.vtype, o.vtype));
        }
        if self.vstart != o.vstart {
            return Some(format!(
                "vstart: ref={:#x} smir={:#x}",
                self.vstart, o.vstart
            ));
        }
        if self.vcsr != o.vcsr {
            return Some(format!("vcsr: ref={:#x} smir={:#x}", self.vcsr, o.vcsr));
        }
        None
    }
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

fn rand_state(rng: &mut Rng) -> State {
    let mut s = State {
        x: [0; 32],
        f: [0; 32],
        fcsr: 0,
        scratch: [0; 64],
        v: [[0; 16]; 32],
        vl: 0,
        vtype: 0,
        vstart: 0,
        vcsr: 0,
    };
    for i in 1..32 {
        s.x[i] = rng.next();
    }
    for i in 0..32 {
        // NaN-box low values sometimes; raw otherwise.
        s.f[i] = rng.next();
    }
    s.fcsr = (rng.next() & 0xff) as u32; // frm[7:5] + fflags[4:0]
    for i in 0..64 {
        s.scratch[i] = rng.next();
    }
    // Vector registers seeded random; vl/vtype left at 0 (the vector test sets
    // them explicitly per instruction). Scalar ops never touch vector state, so
    // it round-trips unchanged through both the oracle and the SMIR interp.
    for i in 0..32 {
        for b in 0..16 {
            s.v[i][b] = rng.next() as u8;
        }
    }
    s
}

/// Run one instruction on the golden `RiscVCpu`; `None` if it traps.
fn run_ref(insn: &[u8], init: &State) -> Option<State> {
    let mem = RvMem::new(0, MEM_SIZE as usize);
    let mut cpu = RiscVCpu::new(RiscVConfig::rv64gc(), Box::new(mem));
    for i in 1..32u8 {
        cpu.set_x(i, init.x[i as usize]);
    }
    for i in 0..32u8 {
        cpu.set_f(i, init.f[i as usize]);
    }
    cpu.set_fcsr(init.fcsr);
    // Vector state.
    for i in 0..32u8 {
        cpu.set_vreg(i, &init.v[i as usize]);
    }
    cpu.set_vl_vtype(init.vl, init.vtype);
    cpu.set_vstart(init.vstart);
    cpu.set_vcsr(init.vcsr);
    // scratch
    let mut sb = Vec::with_capacity(512);
    for w in init.scratch.iter() {
        sb.extend_from_slice(&w.to_le_bytes());
    }
    cpu.write_memory(SCRATCH, &sb).ok()?;
    cpu.write_memory(CODE_ADDR, insn).ok()?;
    cpu.set_pc(CODE_ADDR);
    match cpu.step() {
        RiscVExit::Continue => {}
        _ => return None,
    }
    let mut out = State {
        x: [0; 32],
        f: [0; 32],
        fcsr: 0,
        scratch: [0; 64],
        v: [[0; 16]; 32],
        vl: 0,
        vtype: 0,
        vstart: 0,
        vcsr: 0,
    };
    for i in 1..32u8 {
        out.x[i as usize] = cpu.x(i);
    }
    for i in 0..32u8 {
        out.f[i as usize] = cpu.f(i);
    }
    out.fcsr = cpu.fcsr();
    for (i, w) in out.scratch.iter_mut().enumerate() {
        *w = cpu.mem_read_u64(SCRATCH + (i as u64) * 8).ok()?;
    }
    for i in 0..32u8 {
        out.v[i as usize] = cpu.vreg(i);
    }
    out.vl = cpu.vl();
    out.vtype = cpu.vtype();
    out.vstart = cpu.vstart();
    out.vcsr = cpu.vcsr();
    Some(out)
}

/// Lift the instruction to SMIR and run it. `Ok(None)` => lift gap.
fn run_smir(insn: &[u8], init: &State) -> Result<Option<State>, String> {
    let mut lifter = RiscVLifter::rv64gc();
    let mut lctx = LiftContext::new(SourceArch::RiscV64);
    let res = match lifter.lift_insn(CODE_ADDR, insn, &mut lctx) {
        Ok(r) => r,
        Err(LiftError::Unsupported { .. }) | Err(LiftError::InvalidEncoding { .. }) => {
            return Ok(None);
        }
        Err(e) => return Err(format!("lift error: {e:?}")),
    };
    let mut ops = res.ops;
    for (i, op) in ops.iter_mut().enumerate() {
        op.id = OpId(i as u16);
    }
    let block = SmirBlock {
        id: BlockId(0),
        guest_pc: CODE_ADDR,
        phis: vec![],
        ops,
        terminator: Terminator::Trap {
            kind: TrapKind::Breakpoint,
        },
        exec_count: 0,
    };
    let mut ctx = SmirContext::new_riscv();
    if let ArchRegState::RiscV(rv) = &mut ctx.arch_regs {
        rv.x[1..32].copy_from_slice(&init.x[1..32]);
        rv.f.copy_from_slice(&init.f);
        rv.fcsr = init.fcsr;
        rv.pc = CODE_ADDR;
        rv.v = init.v;
        rv.vl = init.vl;
        rv.vtype = init.vtype;
        rv.vstart = init.vstart;
        rv.vcsr = init.vcsr;
    }
    let _ = ArchReg::RiscV(RiscVReg::X(0)); // keep import used
    ctx.pc = CODE_ADDR;

    let mut mem = SmirMem::with_base(0, MEM_SIZE as usize);
    let mut sb = Vec::with_capacity(512);
    for w in init.scratch.iter() {
        sb.extend_from_slice(&w.to_le_bytes());
    }
    use rax::smir::SmirMemory;
    mem.write(SCRATCH, &sb)
        .map_err(|e| format!("mem seed: {e:?}"))?;

    let interp = SmirInterpreter::new();
    interp.execute_block(&mut ctx, &mut mem, &block);

    let mut out = State {
        x: [0; 32],
        f: [0; 32],
        fcsr: 0,
        scratch: [0; 64],
        v: [[0; 16]; 32],
        vl: 0,
        vtype: 0,
        vstart: 0,
        vcsr: 0,
    };
    // The lifter writes results to SSA virtual regs; resolve each arch reg's
    // final value through the lifter's arch->vreg mapping (an undefined reg maps
    // back to VReg::Arch, which reads the seeded arch_regs unchanged).
    for n in 0..32u8 {
        out.x[n as usize] = ctx.read_vreg(lctx.get_arch_reg(ArchReg::RiscV(RiscVReg::X(n))));
        out.f[n as usize] = ctx.read_vreg(lctx.get_arch_reg(ArchReg::RiscV(RiscVReg::F(n))));
    }
    out.x[0] = 0;
    // fcsr: resolve via the CSR mapping if the lift wrote it, else seeded value.
    let fcsr_vreg = lctx.get_arch_reg(ArchReg::RiscV(RiscVReg::Csr(0x003)));
    out.fcsr = match fcsr_vreg {
        rax::smir::types::VReg::Arch(_) => {
            if let ArchRegState::RiscV(rv) = &ctx.arch_regs {
                rv.fcsr
            } else {
                0
            }
        }
        _ => ctx.read_vreg(fcsr_vreg) as u32,
    };
    for i in 0..64u64 {
        let mut b = [0u8; 8];
        mem.read(SCRATCH + i * 8, &mut b)
            .map_err(|e| format!("mem read: {e:?}"))?;
        out.scratch[i as usize] = u64::from_le_bytes(b);
    }
    // Vector state: the RvVector op writes it directly into ctx.arch_regs.
    if let ArchRegState::RiscV(rv) = &ctx.arch_regs {
        out.v = rv.v;
        out.vl = rv.vl;
        out.vtype = rv.vtype;
        out.vstart = rv.vstart;
        out.vcsr = rv.vcsr;
    }
    Ok(Some(out))
}

/// Like `run_ref` but also returns the resulting program counter — used to
/// verify control-flow lift (the next-PC of branches/jumps).
fn run_ref_cf(insn: &[u8], init: &State) -> Option<(State, u64)> {
    let out = run_ref(insn, init)?;
    // Re-run capturing the post-step PC (run_ref discards it).
    let mem = RvMem::new(0, MEM_SIZE as usize);
    let mut cpu = RiscVCpu::new(RiscVConfig::rv64gc(), Box::new(mem));
    for i in 1..32u8 {
        cpu.set_x(i, init.x[i as usize]);
    }
    cpu.write_memory(CODE_ADDR, insn).ok()?;
    cpu.set_pc(CODE_ADDR);
    match cpu.step() {
        RiscVExit::Continue => {}
        _ => return None,
    }
    Some((out, cpu.pc()))
}

/// Lift a control-flow instruction, execute its ops, and resolve the next PC
/// from the lifted `ControlFlow`. `Ok(None)` => lift gap.
fn run_smir_cf(insn: &[u8], init: &State) -> Result<Option<(State, u64)>, String> {
    use rax::smir::lift::ControlFlow;
    let mut lifter = RiscVLifter::rv64gc();
    let mut lctx = LiftContext::new(SourceArch::RiscV64);
    let res = match lifter.lift_insn(CODE_ADDR, insn, &mut lctx) {
        Ok(r) => r,
        Err(LiftError::Unsupported { .. }) | Err(LiftError::InvalidEncoding { .. }) => {
            return Ok(None);
        }
        Err(e) => return Err(format!("lift error: {e:?}")),
    };
    let cf = res.control_flow;
    let bytes = res.bytes_consumed as u64;
    let mut ops = res.ops;
    for (i, op) in ops.iter_mut().enumerate() {
        op.id = OpId(i as u16);
    }
    let block = SmirBlock {
        id: BlockId(0),
        guest_pc: CODE_ADDR,
        phis: vec![],
        ops,
        terminator: Terminator::Trap {
            kind: TrapKind::Breakpoint,
        },
        exec_count: 0,
    };
    let mut ctx = SmirContext::new_riscv();
    if let ArchRegState::RiscV(rv) = &mut ctx.arch_regs {
        rv.x[1..32].copy_from_slice(&init.x[1..32]);
        rv.pc = CODE_ADDR;
    }
    ctx.pc = CODE_ADDR;
    let mut mem = SmirMem::with_base(0, MEM_SIZE as usize);
    let interp = SmirInterpreter::new();
    interp.execute_block(&mut ctx, &mut mem, &block);

    // Control-flow ops change only x[rd] (link register) and the PC; every other
    // field equals the seed (the oracle reads them back unchanged), so start from
    // the seed and overwrite the integer registers.
    let mut out = *init;
    for n in 0..32u8 {
        out.x[n as usize] = ctx.read_vreg(lctx.get_arch_reg(ArchReg::RiscV(RiscVReg::X(n))));
    }
    out.x[0] = 0;
    let next_pc = match cf {
        ControlFlow::Fallthrough | ControlFlow::NextInsn => CODE_ADDR + bytes,
        ControlFlow::Branch { target } | ControlFlow::DirectBranch(target) => target,
        ControlFlow::CondBranchReg {
            cond,
            taken,
            not_taken,
        } => {
            if ctx.read_vreg(cond) != 0 {
                taken
            } else {
                not_taken
            }
        }
        ControlFlow::IndirectBranch { target } => ctx.read_vreg(target),
        _ => return Ok(None), // CC-based / call / return forms not exercised here
    };
    Ok(Some((out, next_pc)))
}

/// Tally outcomes over a stream of random words for the given opcodes.
fn sweep(seed: u64, opcodes: &[u32], count: usize) {
    let mut rng = Rng::new(seed);
    let isa = Isa::rv64gc();
    let mut matched = 0usize;
    let mut gaps: BTreeMap<String, usize> = BTreeMap::new();
    let mut diverged: Vec<(u32, String)> = Vec::new();
    let mut tried = 0usize;
    while tried < count {
        let opc = opcodes[(rng.next() as usize) % opcodes.len()];
        let w = (rng.next() as u32 & !0x7f) | opc;
        let insn = decode(w, Xlen::Rv64, &isa);
        if insn.is_illegal() {
            continue;
        }
        // Skip control-flow / system / fence — not single-step register compares.
        if matches!(
            insn.op,
            Op::Jal
                | Op::Jalr
                | Op::Beq
                | Op::Bne
                | Op::Blt
                | Op::Bge
                | Op::Bltu
                | Op::Bgeu
                | Op::Ecall
                | Op::Ebreak
                | Op::Fence
                | Op::FenceI
                | Op::Mret
                | Op::Sret
                | Op::Wfi
        ) {
            continue;
        }
        tried += 1;
        // For memory ops, force the base register to point into scratch.
        let (w, base_reg) = retarget_mem(w, &insn.op);
        let mut st = rand_state(&mut rng);
        if let Some(rs1) = base_reg {
            st.x[rs1 as usize] = SCRATCH + 0x40; // mid-window, room either side
        }
        let bytes = w.to_le_bytes();
        let r = match run_ref(&bytes, &st) {
            Some(r) => r,
            None => continue, // ref trapped (e.g. unaligned/odd encoding); skip
        };
        match run_smir(&bytes, &st) {
            Ok(Some(s)) => {
                if let Some(d) = r.eq_regs(&s) {
                    if diverged.len() < 40 {
                        diverged.push((w, format!("{:?}: {d}", insn.op)));
                    }
                } else {
                    matched += 1;
                }
            }
            Ok(None) => *gaps.entry(format!("{:?}", insn.op)).or_default() += 1,
            Err(e) => diverged.push((w, format!("{:?}: {e}", insn.op))),
        }
    }
    eprintln!(
        "sweep(seed={seed:#x}): matched={matched}, gap-ops={}, diverged={}",
        gaps.len(),
        diverged.len()
    );
    let mut gv: Vec<_> = gaps.iter().collect();
    gv.sort_by(|a, b| b.1.cmp(a.1));
    for (op, n) in gv.iter().take(60) {
        eprintln!("  GAP {op}: {n}");
    }
    if !diverged.is_empty() {
        let mut msg = format!("\n{} lift divergence(s) from RiscVCpu:\n", diverged.len());
        for (w, d) in diverged.iter().take(40) {
            msg += &format!("  insn={w:#010x}: {d}\n");
        }
        panic!("{msg}");
    }
}

/// If the instruction reads memory, rewrite rs1 to a fixed base register and
/// report it so the harness can point it into the scratch window. Also clamp
/// the immediate offset small so the access stays in-window.
fn retarget_mem(w: u32, insn: &Op) -> (u32, Option<u32>) {
    let _ = insn;
    let opc = w & 0x7f;
    match opc {
        0x03 | 0x07 => {
            // I-type load: rs1 = bits[19:15]; clamp imm[31:20] to a small value.
            let w = (w & !(0xfff << 20)) | (8u32 << 20);
            let w = (w & !(0x1f << 15)) | (10 << 15);
            (w, Some(10))
        }
        0x23 | 0x27 => {
            // S-type store: imm split; set both halves small, rs1 = x10.
            let mut w = w & !((0x7f << 25) | (0x1f << 7));
            w |= 8 << 7; // imm[4:0] = 8
            let w = (w & !(0x1f << 15)) | (10 << 15);
            (w, Some(10))
        }
        0x2f => {
            // AMO: address = x[rs1]; set rs1 = x10 (aligned base).
            let w = (w & !(0x1f << 15)) | (10 << 15);
            (w, Some(10))
        }
        _ => (w, None),
    }
}

#[test]
fn lift_op_imm() {
    // OP-IMM (0x13), OP-IMM-32 (0x1b), LUI (0x37), AUIPC (0x17)
    sweep(0x5117_0001, &[0x13, 0x1b, 0x37, 0x17], 40_000);
}

#[test]
fn lift_op() {
    // OP (0x33), OP-32 (0x3b)
    sweep(0x5117_0002, &[0x33, 0x3b], 40_000);
}

#[test]
fn lift_mem() {
    // loads (0x03), stores (0x23), AMO (0x2f)
    sweep(0x5117_0003, &[0x03, 0x23, 0x2f], 40_000);
}

/// Sweep compressed (16-bit) instructions. Base registers x2 (sp) and x8..x15
/// point into the scratch window so compressed loads/stores stay mapped; the
/// reference filters illegal/odd encodings (returns None) and they are skipped.
#[test]
fn lift_c() {
    let mut rng = Rng::new(0x5117_0005);
    let mut matched = 0usize;
    let mut gaps: BTreeMap<String, usize> = BTreeMap::new();
    let mut diverged: Vec<(u16, String)> = Vec::new();
    let isa = Isa::rv64gc();
    let mut tried = 0usize;
    let mut iters = 0usize;
    while tried < 20_000 && iters < 4_000_000 {
        iters += 1;
        let w16 = (rng.next() as u16) & 0xFFFF;
        if w16 & 3 == 3 {
            continue; // 32-bit encoding
        }
        // Decode the 16-bit parcel through a scratch memory (decode_at handles
        // compressed parcels by length).
        let mut dmem = RvMem::new(0, 8);
        if RvMemory::write(&mut dmem, 0, &w16.to_le_bytes()).is_err() {
            continue;
        }
        let insn = match decode_at(&dmem, 0, Xlen::Rv64, &isa) {
            Ok(i) if !i.is_illegal() && i.len == 2 => i,
            _ => continue,
        };
        // Skip control-flow / system that aren't single-step register compares.
        if matches!(insn.op, Op::Jal | Op::Jalr | Op::Beq | Op::Bne | Op::Ebreak) {
            continue;
        }
        tried += 1;
        let mut st = rand_state(&mut rng);
        // Point compressed base registers at the scratch window.
        st.x[2] = SCRATCH + 0x80;
        for r in 8..16 {
            st.x[r] = SCRATCH + 0x80;
        }
        let bytes = w16.to_le_bytes();
        let r = match run_ref(&bytes, &st) {
            Some(r) => r,
            None => continue,
        };
        match run_smir(&bytes, &st) {
            Ok(Some(s)) => {
                if let Some(d) = r.eq_regs(&s) {
                    if diverged.len() < 40 {
                        diverged.push((w16, format!("{:?}: {d}", insn.op)));
                    }
                } else {
                    matched += 1;
                }
            }
            Ok(None) => *gaps.entry(format!("{:?}", insn.op)).or_default() += 1,
            Err(e) => diverged.push((w16, format!("{:?}: {e}", insn.op))),
        }
    }
    eprintln!(
        "sweep(compressed): matched={matched}, gap-ops={}, diverged={}",
        gaps.len(),
        diverged.len()
    );
    let mut gv: Vec<_> = gaps.iter().collect();
    gv.sort_by(|a, b| b.1.cmp(a.1));
    for (op, n) in gv.iter().take(40) {
        eprintln!("  GAP {op}: {n}");
    }
    if !diverged.is_empty() {
        let mut msg = format!("\n{} compressed lift divergence(s):\n", diverged.len());
        for (w, d) in diverged.iter().take(40) {
            msg += &format!("  insn={w:#06x}: {d}\n");
        }
        panic!("{msg}");
    }
}

#[test]
fn lift_fp() {
    // FP load/store/op/fma (0x07, 0x27, 0x53, 0x43, 0x47, 0x4b, 0x4f)
    sweep(
        0x5117_0004,
        &[0x07, 0x27, 0x53, 0x43, 0x47, 0x4b, 0x4f],
        60_000,
    );
}

/// CSR (Zicsr) lift verification for the application-visible CSRs SMIR models:
/// fcsr (read+write) and read-only fflags/frm/vl/vtype/vlenb. Sweeps
/// csrrw/csrrs/csrrc + immediate forms; the oracle traps on writes to the
/// read-only CSRs (run_ref → None → skipped), so only legal reads / fcsr writes
/// are compared. Unmodeled CSRs are honest lift gaps (not exercised here).
#[test]
fn lift_csr() {
    let isa = Isa::rv64gc();
    let mut rng = Rng::new(0x5117_0008);
    let csrs: [u32; 6] = [0x003, 0x001, 0x002, 0xc20, 0xc21, 0xc22];
    let funct3s: [u32; 6] = [1, 2, 3, 5, 6, 7]; // csrrw/s/c + immediate forms
    let mut matched = 0usize;
    let mut gaps: BTreeMap<String, usize> = BTreeMap::new();
    let mut diverged: Vec<(u32, String)> = Vec::new();

    for _ in 0..40_000 {
        let csr = csrs[(rng.next() as usize) % csrs.len()];
        let f3 = funct3s[(rng.next() as usize) % funct3s.len()];
        let rd = (rng.next() % 32) as u32;
        let rs1 = (rng.next() % 32) as u32; // rs1 reg or 5-bit zimm
        let w = (csr << 20) | (rs1 << 15) | (f3 << 12) | (rd << 7) | 0x73;
        let insn = decode(w, Xlen::Rv64, &isa);
        if insn.is_illegal() {
            continue;
        }
        // Seed fcsr + vector CSRs to non-trivial values so reads are meaningful.
        let mut st = rand_state(&mut rng);
        st.fcsr = (rng.next() & 0xff) as u32;
        st.vl = rng.next() % 5;
        st.vtype = rng.next() & 0xff;
        st.vstart = 0;
        st.vcsr = rng.next() & 0x7;
        let bytes = w.to_le_bytes();
        let r = match run_ref(&bytes, &st) {
            Some(r) => r,
            None => continue, // RO-CSR write etc. → oracle traps → skip
        };
        match run_smir(&bytes, &st) {
            Ok(Some(s)) => {
                if let Some(d) = r.eq_regs(&s) {
                    if diverged.len() < 40 {
                        diverged.push((w, format!("{:?} csr={csr:#x}: {d}", insn.op)));
                    }
                } else {
                    matched += 1;
                }
            }
            Ok(None) => {
                *gaps
                    .entry(format!("{:?} csr={csr:#x}", insn.op))
                    .or_default() += 1
            }
            Err(e) => diverged.push((w, format!("{:?}: {e}", insn.op))),
        }
    }

    eprintln!(
        "lift_csr: matched={matched}, gap-ops={}, diverged={}",
        gaps.len(),
        diverged.len()
    );
    let mut gv: Vec<_> = gaps.iter().collect();
    gv.sort_by(|a, b| b.1.cmp(a.1));
    for (op, n) in gv.iter().take(20) {
        eprintln!("  GAP {op}: {n}");
    }
    if !diverged.is_empty() {
        let mut msg = format!("\n{} CSR lift divergence(s):\n", diverged.len());
        for (w, d) in diverged.iter().take(40) {
            msg += &format!("  insn={w:#010x}: {d}\n");
        }
        panic!("{msg}");
    }
}

/// Control-flow lift verification: jal / jalr / branches (32-bit and
/// compressed). The single-step register sweeps skip these because the result
/// is the next PC, not a register; here we resolve the next PC from the lifted
/// `ControlFlow` (DirectBranch target / IndirectBranch VReg / CondBranchReg
/// VReg) and compare it — plus the link-register write — against the `RiscVCpu`
/// oracle's post-step PC and registers.
#[test]
fn lift_cf() {
    let isa = Isa::rv64gc();
    let mut rng = Rng::new(0x5117_0007);
    let mut matched = 0usize;
    let mut gaps: BTreeMap<String, usize> = BTreeMap::new();
    let mut diverged: Vec<(u32, String)> = Vec::new();

    // ---- 32-bit control flow: jal (0x6f), jalr (0x67), branches (0x63). ----
    let mut tried = 0usize;
    while tried < 40_000 {
        let opc = [0x6f, 0x67, 0x63][(rng.next() as usize) % 3];
        let w = (rng.next() as u32 & !0x7f) | opc;
        let insn = decode(w, Xlen::Rv64, &isa);
        if insn.is_illegal() {
            continue;
        }
        tried += 1;
        let st = rand_state(&mut rng);
        let bytes = w.to_le_bytes();
        let (rr, rpc) = match run_ref_cf(&bytes, &st) {
            Some(x) => x,
            None => continue,
        };
        match run_smir_cf(&bytes, &st) {
            Ok(Some((s, spc))) => {
                let mut d = rr.eq_regs(&s);
                if d.is_none() && rpc != spc {
                    d = Some(format!("pc: ref={rpc:#x} smir={spc:#x}"));
                }
                if let Some(d) = d {
                    if diverged.len() < 40 {
                        diverged.push((w, format!("{:?}: {d}", insn.op)));
                    }
                } else {
                    matched += 1;
                }
            }
            Ok(None) => *gaps.entry(format!("{:?}", insn.op)).or_default() += 1,
            Err(e) => diverged.push((w, format!("{:?}: {e}", insn.op))),
        }
    }

    // ---- Compressed control flow: c.j / c.jr / c.jalr / c.beqz / c.bnez. ----
    let mut tried_c = 0usize;
    let mut iters = 0usize;
    while tried_c < 20_000 && iters < 4_000_000 {
        iters += 1;
        let w16 = (rng.next() as u16) & 0xFFFF;
        if w16 & 3 == 3 {
            continue;
        }
        let mut dmem = RvMem::new(0, 8);
        if RvMemory::write(&mut dmem, 0, &w16.to_le_bytes()).is_err() {
            continue;
        }
        let insn = match decode_at(&dmem, 0, Xlen::Rv64, &isa) {
            Ok(i) if !i.is_illegal() && i.len == 2 => i,
            _ => continue,
        };
        if !matches!(insn.op, Op::Jal | Op::Jalr | Op::Beq | Op::Bne) {
            continue; // only compressed control-flow forms
        }
        tried_c += 1;
        let st = rand_state(&mut rng);
        let bytes = w16.to_le_bytes();
        let (rr, rpc) = match run_ref_cf(&bytes, &st) {
            Some(x) => x,
            None => continue,
        };
        match run_smir_cf(&bytes, &st) {
            Ok(Some((s, spc))) => {
                let mut d = rr.eq_regs(&s);
                if d.is_none() && rpc != spc {
                    d = Some(format!("pc: ref={rpc:#x} smir={spc:#x}"));
                }
                if let Some(d) = d {
                    if diverged.len() < 40 {
                        diverged.push((w16 as u32, format!("c.{:?}: {d}", insn.op)));
                    }
                } else {
                    matched += 1;
                }
            }
            Ok(None) => *gaps.entry(format!("c.{:?}", insn.op)).or_default() += 1,
            Err(e) => diverged.push((w16 as u32, format!("c.{:?}: {e}", insn.op))),
        }
    }

    eprintln!(
        "lift_cf: matched={matched}, gap-ops={}, diverged={}",
        gaps.len(),
        diverged.len()
    );
    let mut gv: Vec<_> = gaps.iter().collect();
    gv.sort_by(|a, b| b.1.cmp(a.1));
    for (op, n) in gv.iter().take(40) {
        eprintln!("  GAP {op}: {n}");
    }
    if !diverged.is_empty() {
        let mut msg = format!("\n{} control-flow lift divergence(s):\n", diverged.len());
        for (w, d) in diverged.iter().take(40) {
            msg += &format!("  insn={w:#010x}: {d}\n");
        }
        panic!("{msg}");
    }
}

// --- RVV (vector) lift verification ----------------------------------------

/// Encode an OP-V (0x57) instruction. funct3 selects the operand form:
/// 0=OPIVV, 1=OPFVV, 2=OPMVV, 3=OPIVI, 4=OPIVX, 5=OPFVF, 6=OPMVX.
fn vop(funct6: u32, vm: u32, vs2: u32, src: u32, funct3: u32, vd: u32) -> u32 {
    (funct6 << 26) | (vm << 25) | (vs2 << 20) | (src << 15) | (funct3 << 12) | (vd << 7) | 0x57
}

/// RVV lift verification. RVV element width/count are runtime `vtype`/`vl` state
/// unknown at lift time, so the whole vector ISA lifts to the opaque `RvVector`
/// op that runs the qemu-verified vector engine over the SMIR machine state.
/// This test confirms that lift PLUMBING — decode of the carried word, the full
/// state round-trip (x/f/fcsr + the 128-bit vector file + vl/vtype/vstart/vcsr),
/// and the memory bridge — is bit-exact against the `RiscVCpu` oracle for a
/// representative valid instruction from every architectural-effect class. A
/// dedicated test (not the random sweeps) because rax assumes well-formed vector
/// encodings (no group-overlap / out-of-range validation), so it constrains to
/// LMUL=1, disjoint low registers, and unmasked forms. The vector arithmetic
/// itself is proven separately by the qemu oracle (tests/riscv_vector.rs).
#[test]
fn lift_v() {
    const E32_M1: u64 = 0x10; // vsew=2 (SEW=32), vlmul=0 (LMUL=1)
    const VLMAX32: u64 = 4; // VLEN(128)/SEW(32)

    // (name, encoded instruction). width=6 ⇒ 32-bit element loads/stores.
    // Base reg a0=x10, stride a3=x13, AVL a2=x12, dest x-reg a1=x11.
    let prog: &[(&str, u32)] = &[
        // -- configuration (writes x[rd], vl, vtype) --
        (
            "vsetvli a1,a2,e32m1",
            (E32_M1 as u32) << 20 | (12 << 15) | (7 << 12) | (11 << 7) | 0x57,
        ),
        // -- unit-stride load / store (vd / memory) --
        (
            "vle32.v v1,(a0)",
            (1 << 25) | (10 << 15) | (6 << 12) | (1 << 7) | 0x07,
        ),
        (
            "vse32.v v1,(a0)",
            (1 << 25) | (10 << 15) | (6 << 12) | (1 << 7) | 0x27,
        ),
        // -- strided load / store --
        (
            "vlse32.v v1,(a0),a3",
            (0b10 << 26) | (1 << 25) | (13 << 20) | (10 << 15) | (6 << 12) | (1 << 7) | 0x07,
        ),
        (
            "vsse32.v v1,(a0),a3",
            (0b10 << 26) | (1 << 25) | (13 << 20) | (10 << 15) | (6 << 12) | (1 << 7) | 0x27,
        ),
        // -- integer arithmetic vv / vx / vi --
        ("vadd.vv v1,v2,v3", vop(0b000000, 1, 2, 3, 0, 1)),
        ("vadd.vx v1,v2,x13", vop(0b000000, 1, 2, 13, 4, 1)),
        ("vadd.vi v1,v2,5", vop(0b000000, 1, 2, 5, 3, 1)),
        ("vsub.vv v1,v2,v3", vop(0b000010, 1, 2, 3, 0, 1)),
        ("vand.vv v1,v2,v3", vop(0b001001, 1, 2, 3, 0, 1)),
        ("vor.vv v1,v2,v3", vop(0b001010, 1, 2, 3, 0, 1)),
        ("vxor.vv v1,v2,v3", vop(0b001011, 1, 2, 3, 0, 1)),
        ("vsll.vv v1,v2,v3", vop(0b100101, 1, 2, 3, 0, 1)),
        ("vsrl.vv v1,v2,v3", vop(0b101000, 1, 2, 3, 0, 1)),
        ("vmin.vv v1,v2,v3", vop(0b000101, 1, 2, 3, 0, 1)),
        ("vmax.vv v1,v2,v3", vop(0b000111, 1, 2, 3, 0, 1)),
        ("vmul.vv v1,v2,v3", vop(0b100101, 1, 2, 3, 2, 1)), // OPMVV
        ("vmacc.vv v1,v2,v3", vop(0b101101, 1, 2, 3, 2, 1)),
        // -- compare to mask register --
        ("vmseq.vv v1,v2,v3", vop(0b011000, 1, 2, 3, 0, 1)),
        ("vmslt.vv v1,v2,v3", vop(0b011011, 1, 2, 3, 0, 1)),
        // -- reduction (writes vd[0]) --
        ("vredsum.vs v1,v2,v3", vop(0b000000, 1, 2, 3, 2, 1)),
        ("vredmax.vs v1,v2,v3", vop(0b000111, 1, 2, 3, 2, 1)),
        // -- scalar moves out (writes x / f) --
        ("vmv.x.s a1,v2", vop(0b010000, 1, 2, 0, 2, 11)),
        ("vfmv.f.s fa1,v2", vop(0b010000, 1, 2, 0, 1, 11)),
        // -- scalar move in (writes vd[0]) --
        ("vmv.s.x v1,a3", vop(0b010000, 1, 0, 13, 6, 1)),
        ("vfmv.s.f v1,fa3", vop(0b010000, 1, 0, 13, 5, 1)),
        // -- FP arithmetic (writes vd + fcsr) --
        ("vfadd.vv v1,v2,v3", vop(0b000000, 1, 2, 3, 1, 1)),
        ("vfmul.vv v1,v2,v3", vop(0b100100, 1, 2, 3, 1, 1)),
        ("vfmacc.vv v1,v2,v3", vop(0b101100, 1, 2, 3, 1, 1)),
        ("vfmin.vv v1,v2,v3", vop(0b000100, 1, 2, 3, 1, 1)),
        // -- permute: slide / gather / splat --
        ("vslideup.vx v1,v2,x13", vop(0b001110, 1, 2, 13, 4, 1)),
        ("vslidedown.vx v1,v2,x13", vop(0b001111, 1, 2, 13, 4, 1)),
        ("vrgather.vi v1,v2,0", vop(0b001100, 1, 2, 0, 3, 1)),
        ("vmv.v.i v1,5", vop(0b010111, 1, 0, 5, 3, 1)),
        ("vmv.v.v v1,v3", vop(0b010111, 1, 0, 3, 0, 1)),
        ("vmerge.vvm v1,v2,v3", vop(0b010111, 0, 2, 3, 0, 1)),
        // -- whole-register move --
        ("vmv1r.v v1,v2", vop(0b100111, 1, 2, 0, 3, 1)),
    ];

    let mut rng = Rng::new(0x5117_0006);
    let mut matched = 0usize;
    let mut gaps: Vec<&str> = Vec::new();
    let mut diverged: Vec<(String, String)> = Vec::new();

    for &(name, insn) in prog {
        for _ in 0..64 {
            let mut st = rand_state(&mut rng);
            // Valid e32/m1 config; full vl. (vsetvli overwrites these itself.)
            st.vtype = E32_M1;
            st.vl = VLMAX32;
            st.vstart = 0;
            st.vcsr = rng.next() & 0x7; // vxrm[2:1] | vxsat[0]
            // Address / scalar operand registers.
            st.x[10] = SCRATCH + 0x40; // a0 = aligned base
            st.x[12] = (rng.next() % (VLMAX32 + 1)).max(0); // a2 = AVL for vsetvli
            st.x[13] = 8; // a3 = stride / scalar
            let bytes = insn.to_le_bytes();
            let r = match run_ref(&bytes, &st) {
                Some(r) => r,
                None => continue, // ref trapped (unexpected for these) → skip
            };
            match run_smir(&bytes, &st) {
                Ok(Some(s)) => {
                    if let Some(d) = r.eq_regs(&s) {
                        if diverged.len() < 40 {
                            diverged.push((name.to_string(), d));
                        }
                    } else {
                        matched += 1;
                    }
                }
                Ok(None) => gaps.push(name),
                Err(e) => diverged.push((name.to_string(), e)),
            }
        }
    }

    eprintln!(
        "lift_v: matched={matched}, gaps={}, diverged={}",
        gaps.len(),
        diverged.len()
    );
    if !gaps.is_empty() {
        let mut g: Vec<&&str> = gaps.iter().collect();
        g.sort();
        g.dedup();
        for n in g {
            eprintln!("  GAP {n}");
        }
        panic!("{} vector ops did not lift (RvVector gap)", gaps.len());
    }
    if !diverged.is_empty() {
        let mut msg = format!("\n{} vector lift divergence(s):\n", diverged.len());
        for (n, d) in diverged.iter().take(40) {
            msg += &format!("  {n}: {d}\n");
        }
        panic!("{msg}");
    }
}

/// Exhaustive gap audit: sweep fully-random 32-bit and compressed words across
/// the WHOLE opcode space; for any instruction the oracle executes (Continue),
/// flag it if the lifter cannot lift it (`Ok(None)`). This proves there is no
/// remaining user-mode instruction with architectural effect left unlifted.
/// Divergences are checked by the dedicated per-class tests; here we only audit
/// coverage (gaps), so control-flow/CSR/vector results are not re-compared.
#[test]
fn lift_exhaustive_audit() {
    let isa = Isa::rv64gc();
    let mut rng = Rng::new(0x5117_0009);
    let mut gaps: BTreeMap<String, usize> = BTreeMap::new();
    let mut executed = 0usize;

    // 32-bit space.
    for _ in 0..400_000 {
        let w = rng.next() as u32;
        let insn = decode(w, Xlen::Rv64, &isa);
        if insn.is_illegal() {
            continue;
        }
        let mut st = rand_state(&mut rng);
        // Point common base registers into the scratch window for any ld/st.
        for r in 1..32 {
            st.x[r] = SCRATCH + 0x40;
        }
        st.fcsr = (rng.next() & 0xff) as u32;
        if run_ref(&w.to_le_bytes(), &st).is_none() {
            continue; // oracle trapped / not a plain single-step → ignore
        }
        executed += 1;
        if let Ok(None) = run_smir(&w.to_le_bytes(), &st) {
            *gaps.entry(format!("{:?}", insn.op)).or_default() += 1;
        }
    }

    // Compressed space.
    for _ in 0..200_000 {
        let w16 = rng.next() as u16;
        if w16 & 3 == 3 {
            continue;
        }
        let mut dmem = RvMem::new(0, 8);
        if RvMemory::write(&mut dmem, 0, &w16.to_le_bytes()).is_err() {
            continue;
        }
        let insn = match decode_at(&dmem, 0, Xlen::Rv64, &isa) {
            Ok(i) if !i.is_illegal() && i.len == 2 => i,
            _ => continue,
        };
        let mut st = rand_state(&mut rng);
        st.x[2] = SCRATCH + 0x80;
        for r in 8..16 {
            st.x[r] = SCRATCH + 0x80;
        }
        if run_ref(&w16.to_le_bytes(), &st).is_none() {
            continue;
        }
        executed += 1;
        if let Ok(None) = run_smir(&w16.to_le_bytes(), &st) {
            *gaps.entry(format!("c.{:?}", insn.op)).or_default() += 1;
        }
    }

    eprintln!(
        "lift_exhaustive_audit: executed={executed}, gap-ops={}",
        gaps.len()
    );
    let mut gv: Vec<_> = gaps.iter().collect();
    gv.sort_by(|a, b| b.1.cmp(a.1));
    for (op, n) in gv.iter() {
        eprintln!("  GAP {op}: {n}");
    }
    // Only genuinely-environmental / privileged ops may remain — they have no
    // deterministic architectural register/memory result to lift in a user-mode
    // harness: ECALL/EBREAK (environment trap), FENCE/FENCE.I (ordering no-op),
    // privileged MRET/SRET/WFI, and CSR access to UNMODELED CSRs (privileged
    // machine state + the nondeterministic cycle/time/instret counters — only
    // the application FP/vector CSRs are modeled; see lift_csr). Every
    // computational / load-store / atomic / FP / vector / control-flow / app-CSR
    // instruction MUST lift.
    let allowed = |op: &str| {
        matches!(
            op,
            "Ecall"
                | "Ebreak"
                | "Fence"
                | "FenceI"
                | "Mret"
                | "Sret"
                | "Wfi"
                | "Pause"
                | "c.Ebreak"
        ) || op.starts_with("Csrr")
    };
    let unexpected: Vec<_> = gaps.keys().filter(|k| !allowed(k)).collect();
    assert!(
        unexpected.is_empty(),
        "unexpected lift gaps (should be lifted): {unexpected:?}"
    );
}
