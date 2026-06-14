//! SMIR execution context.
//!
//! This module defines the execution context that holds all state during SMIR execution.

use std::collections::{HashMap, HashSet};

use crate::smir::flags::FlagState;
use crate::smir::memory::{ExclusiveMonitor, SmirMemory};
use crate::smir::types::*;

// ============================================================================
// Exit Reason
// ============================================================================

/// Reason for execution exit
#[derive(Clone, Debug)]
pub enum ExitReason {
    /// Reached instruction limit
    InsnLimit,
    /// Halt instruction
    Halt,
    /// System call
    Syscall { num: u64, args: Vec<u64> },
    /// Breakpoint hit
    Breakpoint { addr: GuestAddr },
    /// Memory fault
    MemoryFault { addr: GuestAddr, write: bool },
    /// Undefined instruction
    Undefined { addr: GuestAddr, opcode: u32 },
    /// External interrupt
    Interrupt { vector: u8 },
    /// Self-modifying code detected
    SmcDetected { addr: GuestAddr },
    /// Single step complete
    SingleStep,
    /// Function return
    Return { to: GuestAddr },
    /// Block not found (needs lifting)
    BlockNotFound { addr: GuestAddr },
}

// ============================================================================
// Debug State
// ============================================================================

/// Debug state
#[derive(Clone, Debug, Default)]
pub struct DebugState {
    /// Single-step mode
    pub single_step: bool,
    /// Breakpoints
    pub breakpoints: HashSet<GuestAddr>,
    /// Trace enabled
    pub trace_enabled: bool,
}

impl DebugState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_breakpoint(&mut self, addr: GuestAddr) {
        self.breakpoints.insert(addr);
    }

    pub fn remove_breakpoint(&mut self, addr: GuestAddr) {
        self.breakpoints.remove(&addr);
    }

    pub fn has_breakpoint(&self, addr: GuestAddr) -> bool {
        self.breakpoints.contains(&addr)
    }
}

// ============================================================================
// Virtual Register File
// ============================================================================

pub type VecValue = [u64; 16]; // 1024-bit: fits x86 ZMM (512) AND Hexagon HVX (1024)

/// Virtual register file
#[derive(Clone, Debug, Default)]
pub struct VRegFile {
    /// Scalar register values
    values: HashMap<VirtualId, u64>,
    /// Vector register values (up to 512-bit)
    vec_values: HashMap<VirtualId, VecValue>,
}

impl VRegFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, id: VirtualId) -> u64 {
        self.values.get(&id).copied().unwrap_or(0)
    }

    pub fn set(&mut self, id: VirtualId, value: u64) {
        self.values.insert(id, value);
    }

    pub fn get_vec(&self, id: VirtualId) -> VecValue {
        self.vec_values.get(&id).copied().unwrap_or([0; 16])
    }

    pub fn set_vec(&mut self, id: VirtualId, value: VecValue) {
        self.vec_values.insert(id, value);
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.vec_values.clear();
    }
}

// ============================================================================
// Architecture-Specific Register State
// ============================================================================

/// Architecture-specific register state
#[derive(Clone, Debug)]
pub enum ArchRegState {
    X86_64(X86RegState),
    Aarch64(Aarch64RegState),
    Hexagon(HexagonRegState),
    RiscV(RiscVRegState),
}

impl ArchRegState {
    /// Get the program counter
    pub fn pc(&self) -> GuestAddr {
        match self {
            ArchRegState::X86_64(x86) => x86.rip,
            ArchRegState::Aarch64(arm) => arm.pc,
            ArchRegState::Hexagon(hex) => hex.pc as u64,
            ArchRegState::RiscV(rv) => rv.pc,
        }
    }

    /// Set the program counter
    pub fn set_pc(&mut self, pc: GuestAddr) {
        match self {
            ArchRegState::X86_64(x86) => x86.rip = pc,
            ArchRegState::Aarch64(arm) => arm.pc = pc,
            ArchRegState::Hexagon(hex) => hex.pc = pc as u32,
            ArchRegState::RiscV(rv) => rv.pc = pc,
        }
    }

    /// Get the stack pointer
    pub fn sp(&self) -> u64 {
        match self {
            ArchRegState::X86_64(x86) => x86.gpr[4], // RSP
            ArchRegState::Aarch64(arm) => arm.sp,
            ArchRegState::Hexagon(hex) => hex.sp as u64,
            ArchRegState::RiscV(rv) => rv.x[2], // x2 = sp
        }
    }

    /// Set the stack pointer
    pub fn set_sp(&mut self, sp: u64) {
        match self {
            ArchRegState::X86_64(x86) => x86.gpr[4] = sp,
            ArchRegState::Aarch64(arm) => arm.sp = sp,
            ArchRegState::Hexagon(hex) => hex.sp = sp as u32,
            ArchRegState::RiscV(rv) => rv.x[2] = sp,
        }
    }
}

/// x86_64 register state
#[derive(Clone, Debug, Default)]
pub struct X86RegState {
    /// General purpose registers: RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8-R31
    pub gpr: [u64; 32],
    /// Instruction pointer
    pub rip: u64,
    /// Flags register
    pub rflags: u64,
    /// FS base
    pub fs_base: u64,
    /// GS base
    pub gs_base: u64,
    /// XMM/YMM/ZMM registers (up to 512-bit each)
    pub xmm: [VecValue; 32],
    /// AVX-512 opmask registers K0-K7.
    pub k: [u64; 8],
    /// MXCSR (SSE control/status)
    pub mxcsr: u32,
}

impl X86RegState {
    pub fn new() -> Self {
        let mut state = Self::default();
        // Initialize RFLAGS with required bits
        state.rflags = 0x202; // IF=1, reserved bit 1 always set
        // Initialize MXCSR with default
        state.mxcsr = 0x1F80; // Default rounding, all exceptions masked
        state
    }

    /// Get GPR by index
    pub fn get_gpr(&self, idx: u8) -> u64 {
        self.gpr[idx as usize & 0x1F]
    }

    /// Set GPR by index
    pub fn set_gpr(&mut self, idx: u8, val: u64) {
        self.gpr[idx as usize & 0x1F] = val;
    }

    /// Get GPR by name (RAX=0, RCX=1, etc.)
    pub fn get_gpr_by_reg(&self, reg: X86Reg) -> u64 {
        match reg {
            X86Reg::Rax => self.gpr[0],
            X86Reg::Rcx => self.gpr[1],
            X86Reg::Rdx => self.gpr[2],
            X86Reg::Rbx => self.gpr[3],
            X86Reg::Rsp => self.gpr[4],
            X86Reg::Rbp => self.gpr[5],
            X86Reg::Rsi => self.gpr[6],
            X86Reg::Rdi => self.gpr[7],
            X86Reg::R8 => self.gpr[8],
            X86Reg::R9 => self.gpr[9],
            X86Reg::R10 => self.gpr[10],
            X86Reg::R11 => self.gpr[11],
            X86Reg::R12 => self.gpr[12],
            X86Reg::R13 => self.gpr[13],
            X86Reg::R14 => self.gpr[14],
            X86Reg::R15 => self.gpr[15],
            X86Reg::R16 => self.gpr[16],
            X86Reg::R17 => self.gpr[17],
            X86Reg::R18 => self.gpr[18],
            X86Reg::R19 => self.gpr[19],
            X86Reg::R20 => self.gpr[20],
            X86Reg::R21 => self.gpr[21],
            X86Reg::R22 => self.gpr[22],
            X86Reg::R23 => self.gpr[23],
            X86Reg::R24 => self.gpr[24],
            X86Reg::R25 => self.gpr[25],
            X86Reg::R26 => self.gpr[26],
            X86Reg::R27 => self.gpr[27],
            X86Reg::R28 => self.gpr[28],
            X86Reg::R29 => self.gpr[29],
            X86Reg::R30 => self.gpr[30],
            X86Reg::R31 => self.gpr[31],
            X86Reg::Rip => self.rip,
            X86Reg::Rflags => self.rflags,
            X86Reg::FsBase => self.fs_base,
            X86Reg::GsBase => self.gs_base,
            X86Reg::Xmm(n) | X86Reg::Ymm(n) | X86Reg::Zmm(n) => self.xmm[n as usize][0],
            X86Reg::K(n) => self.k[n as usize & 0x7],
            _ => 0,
        }
    }

    /// Set GPR by name
    pub fn set_gpr_by_reg(&mut self, reg: X86Reg, val: u64) {
        match reg {
            X86Reg::Rax => self.gpr[0] = val,
            X86Reg::Rcx => self.gpr[1] = val,
            X86Reg::Rdx => self.gpr[2] = val,
            X86Reg::Rbx => self.gpr[3] = val,
            X86Reg::Rsp => self.gpr[4] = val,
            X86Reg::Rbp => self.gpr[5] = val,
            X86Reg::Rsi => self.gpr[6] = val,
            X86Reg::Rdi => self.gpr[7] = val,
            X86Reg::R8 => self.gpr[8] = val,
            X86Reg::R9 => self.gpr[9] = val,
            X86Reg::R10 => self.gpr[10] = val,
            X86Reg::R11 => self.gpr[11] = val,
            X86Reg::R12 => self.gpr[12] = val,
            X86Reg::R13 => self.gpr[13] = val,
            X86Reg::R14 => self.gpr[14] = val,
            X86Reg::R15 => self.gpr[15] = val,
            X86Reg::R16 => self.gpr[16] = val,
            X86Reg::R17 => self.gpr[17] = val,
            X86Reg::R18 => self.gpr[18] = val,
            X86Reg::R19 => self.gpr[19] = val,
            X86Reg::R20 => self.gpr[20] = val,
            X86Reg::R21 => self.gpr[21] = val,
            X86Reg::R22 => self.gpr[22] = val,
            X86Reg::R23 => self.gpr[23] = val,
            X86Reg::R24 => self.gpr[24] = val,
            X86Reg::R25 => self.gpr[25] = val,
            X86Reg::R26 => self.gpr[26] = val,
            X86Reg::R27 => self.gpr[27] = val,
            X86Reg::R28 => self.gpr[28] = val,
            X86Reg::R29 => self.gpr[29] = val,
            X86Reg::R30 => self.gpr[30] = val,
            X86Reg::R31 => self.gpr[31] = val,
            X86Reg::Rip => self.rip = val,
            X86Reg::Rflags => self.rflags = val,
            X86Reg::FsBase => self.fs_base = val,
            X86Reg::GsBase => self.gs_base = val,
            X86Reg::Xmm(n) | X86Reg::Ymm(n) | X86Reg::Zmm(n) => self.xmm[n as usize][0] = val,
            X86Reg::K(n) => self.k[n as usize & 0x7] = val,
            _ => {}
        }
    }
}

/// AArch64 register state
#[derive(Clone, Debug, Default)]
pub struct Aarch64RegState {
    /// General purpose registers X0-X30
    pub x: [u64; 31],
    /// Stack pointer
    pub sp: u64,
    /// Program counter
    pub pc: u64,
    /// NZCV flags
    pub nzcv: u32,
    /// FPCR
    pub fpcr: u32,
    /// FPSR
    pub fpsr: u32,
    /// SIMD registers V0-V31 (128-bit each)
    pub v: [[u64; 2]; 32],
}

impl Aarch64RegState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get X register (X31 returns 0 as ZR)
    pub fn get_x(&self, n: u8) -> u64 {
        if n >= 31 {
            0 // XZR
        } else {
            self.x[n as usize]
        }
    }

    /// Set X register (writes to X31 are ignored)
    pub fn set_x(&mut self, n: u8, val: u64) {
        if n < 31 {
            self.x[n as usize] = val;
        }
    }
}

/// Hexagon register state
#[derive(Clone, Debug, Default)]
pub struct HexagonRegState {
    /// General purpose registers R0-R31
    pub r: [u32; 32],
    /// Predicate registers P0-P3 (full 8-bit value: 0x00/0xff for scalar
    /// compares, per-lane masks for vector compares — matches the Hexagon
    /// byte-granular predicate model).
    pub p: [u8; 4],
    /// Program counter
    pub pc: u32,
    /// Global pointer
    pub gp: u32,
    /// Link register (R31)
    pub lr: u32,
    /// Stack pointer (R29)
    pub sp: u32,
    /// Frame pointer (R30)
    pub fp: u32,
    /// Loop count registers
    pub lc: [u32; 2],
    /// Loop start address registers
    pub sa: [u32; 2],
    /// User status register
    pub usr: u32,
    /// HVX vector registers V0-V31 (1024-bit each)
    pub v: [VecValue; 32],
    /// HVX vector predicate registers Q0-Q3 (128-bit; stored in lanes 0-1)
    pub q: [VecValue; 4],
    /// Modifier registers M0/M1
    pub m: [u32; 2],
    /// Circular-buffer start registers CS0/CS1
    pub cs: [u32; 2],
}

impl HexagonRegState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_r(&self, n: u8) -> u32 {
        self.r[n as usize & 0x1F]
    }

    pub fn set_r(&mut self, n: u8, val: u32) {
        self.r[n as usize & 0x1F] = val;
    }

    pub fn get_v(&self, n: u8) -> VecValue {
        self.v[n as usize & 0x1F]
    }

    pub fn set_v(&mut self, n: u8, val: VecValue) {
        self.v[n as usize & 0x1F] = val;
    }

    pub fn get_q(&self, n: u8) -> VecValue {
        self.q[n as usize & 0x3]
    }

    pub fn set_q(&mut self, n: u8, val: VecValue) {
        self.q[n as usize & 0x3] = val;
    }
}

/// RISC-V register state
#[derive(Clone, Debug)]
pub struct RiscVRegState {
    /// Integer registers x0-x31 (x0 is hardwired to 0)
    pub x: [u64; 32],
    /// Floating-point registers f0-f31
    pub f: [u64; 32],
    /// Program counter
    pub pc: u64,
    /// Floating-point CSR
    pub fcsr: u32,
    /// Vector registers v0-v31 (VLEN = 128 → 16 bytes each, little-endian).
    pub v: [[u8; 16]; 32],
    /// Vector length CSR (`vl`): active element count.
    pub vl: u64,
    /// Vector type CSR (`vtype`): SEW/LMUL/ta/ma/vill encoding.
    pub vtype: u64,
    /// Vector start index CSR (`vstart`).
    pub vstart: u64,
    /// Packed `vcsr` = {vxrm[2:1], vxsat[0]} (fixed-point rounding/saturation).
    pub vcsr: u64,
}

impl Default for RiscVRegState {
    fn default() -> Self {
        Self::new()
    }
}

impl RiscVRegState {
    pub fn new() -> Self {
        RiscVRegState {
            x: [0; 32],
            f: [0; 32],
            pc: 0,
            fcsr: 0,
            v: [[0; 16]; 32],
            vl: 0,
            vtype: 0,
            vstart: 0,
            vcsr: 0,
        }
    }

    /// Get integer register (x0 always returns 0)
    pub fn get_x(&self, n: u8) -> u64 {
        if n == 0 { 0 } else { self.x[n as usize & 0x1F] }
    }

    /// Set integer register (writes to x0 are ignored)
    pub fn set_x(&mut self, n: u8, val: u64) {
        if n != 0 {
            self.x[n as usize & 0x1F] = val;
        }
    }
}

// ============================================================================
// Execution Context
// ============================================================================

/// Execution context for SMIR
pub struct SmirContext {
    /// Source architecture
    pub source_arch: SourceArch,
    /// Virtual register file
    pub vregs: VRegFile,
    /// Architecture-specific register state
    pub arch_regs: ArchRegState,
    /// Flag state
    pub flags: FlagState,
    /// Current program counter (guest)
    pub pc: GuestAddr,
    /// Instruction count
    pub insn_count: u64,
    /// Cycle count (estimated)
    pub cycle_count: u64,
    /// Exit reason (when execution stops)
    pub exit_reason: Option<ExitReason>,
    /// Debug state
    pub debug: DebugState,
    /// Exclusive monitor (for ARM LL/SC)
    pub exclusive_monitor: ExclusiveMonitor,
}

impl SmirContext {
    /// Create a new x86_64 context
    pub fn new_x86_64() -> Self {
        SmirContext {
            source_arch: SourceArch::X86_64,
            vregs: VRegFile::new(),
            arch_regs: ArchRegState::X86_64(X86RegState::new()),
            flags: FlagState::new(),
            pc: 0,
            insn_count: 0,
            cycle_count: 0,
            exit_reason: None,
            debug: DebugState::new(),
            exclusive_monitor: ExclusiveMonitor::new(),
        }
    }

    /// Create a new AArch64 context
    pub fn new_aarch64() -> Self {
        SmirContext {
            source_arch: SourceArch::Aarch64,
            vregs: VRegFile::new(),
            arch_regs: ArchRegState::Aarch64(Aarch64RegState::new()),
            flags: FlagState::new(),
            pc: 0,
            insn_count: 0,
            cycle_count: 0,
            exit_reason: None,
            debug: DebugState::new(),
            exclusive_monitor: ExclusiveMonitor::new(),
        }
    }

    /// Create a new Hexagon context
    pub fn new_hexagon() -> Self {
        SmirContext {
            source_arch: SourceArch::Hexagon,
            vregs: VRegFile::new(),
            arch_regs: ArchRegState::Hexagon(HexagonRegState::new()),
            flags: FlagState::new(),
            pc: 0,
            insn_count: 0,
            cycle_count: 0,
            exit_reason: None,
            debug: DebugState::new(),
            exclusive_monitor: ExclusiveMonitor::new(),
        }
    }

    /// Create a new RISC-V context
    pub fn new_riscv() -> Self {
        SmirContext {
            source_arch: SourceArch::RiscV64,
            vregs: VRegFile::new(),
            arch_regs: ArchRegState::RiscV(RiscVRegState::new()),
            flags: FlagState::new(),
            pc: 0,
            insn_count: 0,
            cycle_count: 0,
            exit_reason: None,
            debug: DebugState::new(),
            exclusive_monitor: ExclusiveMonitor::new(),
        }
    }

    /// Read a virtual register
    pub fn read_vreg(&self, vreg: VReg) -> u64 {
        match vreg {
            VReg::Virtual(id) => self.vregs.get(id),
            VReg::Imm(val) => val as u64,
            VReg::Arch(reg) => self.read_arch_reg(reg),
        }
    }

    /// Write a virtual register
    pub fn write_vreg(&mut self, vreg: VReg, value: u64) {
        match vreg {
            VReg::Virtual(id) => self.vregs.set(id, value),
            VReg::Imm(_) => {} // Ignore writes to immediates
            VReg::Arch(reg) => self.write_arch_reg(reg, value),
        }
    }

    /// Read an architecture register
    pub fn read_arch_reg(&self, reg: ArchReg) -> u64 {
        match (&self.arch_regs, reg) {
            (ArchRegState::X86_64(x86), ArchReg::X86(r)) => x86.get_gpr_by_reg(r),
            (ArchRegState::Aarch64(arm), ArchReg::Arm(r)) => match r {
                ArmReg::X(n) => arm.get_x(n),
                ArmReg::Sp => arm.sp,
                ArmReg::Pc => arm.pc,
                ArmReg::Nzcv => arm.nzcv as u64,
                ArmReg::Fpcr => arm.fpcr as u64,
                ArmReg::Fpsr => arm.fpsr as u64,
                ArmReg::V(n) => arm.v[n as usize][0],
                _ => 0,
            },
            (ArchRegState::Hexagon(hex), ArchReg::Hexagon(r)) => match r {
                HexagonReg::R(n) => hex.get_r(n) as u64,
                HexagonReg::P(n) => hex.p[n as usize] as u64,
                HexagonReg::Pc => hex.pc as u64,
                HexagonReg::Gp => hex.gp as u64,
                HexagonReg::Lr => hex.lr as u64,
                HexagonReg::Sp => hex.sp as u64,
                HexagonReg::Fp => hex.fp as u64,
                HexagonReg::Lc0 => hex.lc[0] as u64,
                HexagonReg::Lc1 => hex.lc[1] as u64,
                HexagonReg::Sa0 => hex.sa[0] as u64,
                HexagonReg::Sa1 => hex.sa[1] as u64,
                HexagonReg::Usr => hex.usr as u64,
                HexagonReg::M(n) => hex.m[n as usize & 1] as u64,
                HexagonReg::Cs(n) => hex.cs[n as usize & 1] as u64,
                // Vector regs are accessed via read_vec; scalar read returns low 64 bits.
                HexagonReg::V(n) => hex.get_v(n)[0],
                HexagonReg::Q(n) => hex.get_q(n)[0],
            },
            (ArchRegState::RiscV(rv), ArchReg::RiscV(r)) => match r {
                RiscVReg::X(n) => rv.get_x(n),
                RiscVReg::F(n) => rv.f[n as usize],
                RiscVReg::Pc => rv.pc,
                // Vector register: low 64 bits (the opaque RvVector op accesses
                // the full 128-bit `rv.v` directly).
                RiscVReg::V(n) => {
                    u64::from_le_bytes(rv.v[n as usize & 0x1f][0..8].try_into().unwrap())
                }
                // Floating-point CSRs alias `fcsr` (fflags[4:0], frm[7:5]).
                RiscVReg::Csr(0x001) => (rv.fcsr & 0x1f) as u64, // fflags
                RiscVReg::Csr(0x002) => ((rv.fcsr >> 5) & 0x7) as u64, // frm
                RiscVReg::Csr(0x003) => rv.fcsr as u64,          // fcsr
                // Vector CSRs.
                RiscVReg::Csr(0x008) => rv.vstart,   // vstart
                RiscVReg::Csr(0x009) => rv.vcsr & 1, // vxsat
                RiscVReg::Csr(0x00a) => (rv.vcsr >> 1) & 3, // vxrm
                RiscVReg::Csr(0x00f) => rv.vcsr,     // vcsr
                RiscVReg::Csr(0xc20) => rv.vl,       // vl
                RiscVReg::Csr(0xc21) => rv.vtype,    // vtype
                RiscVReg::Csr(0xc22) => 16,          // vlenb = VLEN/8 (VLEN=128)
                _ => 0,
            },
            _ => 0, // Architecture mismatch
        }
    }

    /// Write an architecture register
    pub fn write_arch_reg(&mut self, reg: ArchReg, value: u64) {
        match (&mut self.arch_regs, reg) {
            (ArchRegState::X86_64(x86), ArchReg::X86(r)) => x86.set_gpr_by_reg(r, value),
            (ArchRegState::Aarch64(arm), ArchReg::Arm(r)) => match r {
                ArmReg::X(n) => arm.set_x(n, value),
                ArmReg::Sp => arm.sp = value,
                ArmReg::Pc => arm.pc = value,
                ArmReg::Nzcv => arm.nzcv = value as u32,
                ArmReg::Fpcr => arm.fpcr = value as u32,
                ArmReg::Fpsr => arm.fpsr = value as u32,
                ArmReg::V(n) => arm.v[n as usize][0] = value,
                _ => {}
            },
            (ArchRegState::Hexagon(hex), ArchReg::Hexagon(r)) => match r {
                HexagonReg::R(n) => hex.set_r(n, value as u32),
                HexagonReg::P(n) => hex.p[n as usize] = value as u8,
                HexagonReg::Pc => hex.pc = value as u32,
                HexagonReg::Gp => hex.gp = value as u32,
                HexagonReg::Lr => hex.lr = value as u32,
                HexagonReg::Sp => hex.sp = value as u32,
                HexagonReg::Fp => hex.fp = value as u32,
                HexagonReg::Lc0 => hex.lc[0] = value as u32,
                HexagonReg::Lc1 => hex.lc[1] = value as u32,
                HexagonReg::Sa0 => hex.sa[0] = value as u32,
                HexagonReg::Sa1 => hex.sa[1] = value as u32,
                HexagonReg::Usr => hex.usr = value as u32,
                HexagonReg::M(n) => hex.m[n as usize & 1] = value as u32,
                HexagonReg::Cs(n) => hex.cs[n as usize & 1] = value as u32,
                // Vector regs are written via write_vec; scalar write sets low 64 bits.
                HexagonReg::V(n) => {
                    let mut v = hex.get_v(n);
                    v[0] = value;
                    hex.set_v(n, v);
                }
                HexagonReg::Q(n) => {
                    let mut q = hex.get_q(n);
                    q[0] = value;
                    hex.set_q(n, q);
                }
            },
            (ArchRegState::RiscV(rv), ArchReg::RiscV(r)) => match r {
                RiscVReg::X(n) => rv.set_x(n, value),
                RiscVReg::F(n) => rv.f[n as usize] = value,
                RiscVReg::Pc => rv.pc = value,
                // Vector register: write the low 64 bits.
                RiscVReg::V(n) => {
                    rv.v[n as usize & 0x1f][0..8].copy_from_slice(&value.to_le_bytes())
                }
                // Floating-point CSRs alias `fcsr` (fflags[4:0], frm[7:5]).
                RiscVReg::Csr(0x001) => rv.fcsr = (rv.fcsr & !0x1f) | (value as u32 & 0x1f),
                RiscVReg::Csr(0x002) => rv.fcsr = (rv.fcsr & !0xe0) | ((value as u32 & 0x7) << 5),
                RiscVReg::Csr(0x003) => rv.fcsr = value as u32 & 0xff,
                // Vector CSRs.
                RiscVReg::Csr(0x008) => rv.vstart = value,
                RiscVReg::Csr(0x00f) => rv.vcsr = value,
                RiscVReg::Csr(0xc20) => rv.vl = value,
                RiscVReg::Csr(0xc21) => rv.vtype = value,
                _ => {}
            },
            _ => {} // Architecture mismatch
        }
    }

    /// Request an exit
    pub fn request_exit(&mut self, reason: ExitReason) {
        self.exit_reason = Some(reason);
    }

    /// Clear the vregs (for new block)
    pub fn clear_vregs(&mut self) {
        self.vregs.clear();
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x86_context() {
        let mut ctx = SmirContext::new_x86_64();

        // Write via arch reg
        ctx.write_arch_reg(ArchReg::X86(X86Reg::Rax), 42);
        assert_eq!(ctx.read_arch_reg(ArchReg::X86(X86Reg::Rax)), 42);

        // Write via vreg
        let vreg = VReg::Arch(ArchReg::X86(X86Reg::Rbx));
        ctx.write_vreg(vreg, 100);
        assert_eq!(ctx.read_vreg(vreg), 100);

        // Opmask register
        let kreg = VReg::Arch(ArchReg::X86(X86Reg::K(3)));
        ctx.write_vreg(kreg, 0x55aa);
        assert_eq!(ctx.read_vreg(kreg), 0x55aa);

        // Virtual register
        let virt = VReg::Virtual(VirtualId(0));
        ctx.write_vreg(virt, 999);
        assert_eq!(ctx.read_vreg(virt), 999);

        // Immediate
        assert_eq!(ctx.read_vreg(VReg::Imm(123)), 123);
    }

    #[test]
    fn test_aarch64_context() {
        let mut ctx = SmirContext::new_aarch64();

        // X0
        ctx.write_arch_reg(ArchReg::Arm(ArmReg::X(0)), 42);
        assert_eq!(ctx.read_arch_reg(ArchReg::Arm(ArmReg::X(0))), 42);

        // X31 (ZR) should always read as 0
        ctx.write_arch_reg(ArchReg::Arm(ArmReg::X(31)), 100);
        assert_eq!(ctx.read_arch_reg(ArchReg::Arm(ArmReg::X(31))), 0);
    }

    #[test]
    fn test_riscv_context() {
        let mut ctx = SmirContext::new_riscv();

        // x1 (ra)
        ctx.write_arch_reg(ArchReg::RiscV(RiscVReg::X(1)), 42);
        assert_eq!(ctx.read_arch_reg(ArchReg::RiscV(RiscVReg::X(1))), 42);

        // x0 should always read as 0
        ctx.write_arch_reg(ArchReg::RiscV(RiscVReg::X(0)), 100);
        assert_eq!(ctx.read_arch_reg(ArchReg::RiscV(RiscVReg::X(0))), 0);
    }
}
