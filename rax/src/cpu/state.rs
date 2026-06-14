//! Backend-agnostic CPU state types.

use serde::{Deserialize, Serialize};

/// General-purpose registers.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    // APX Extended General Purpose Registers (R16-R31)
    pub r16: u64,
    pub r17: u64,
    pub r18: u64,
    pub r19: u64,
    pub r20: u64,
    pub r21: u64,
    pub r22: u64,
    pub r23: u64,
    pub r24: u64,
    pub r25: u64,
    pub r26: u64,
    pub r27: u64,
    pub r28: u64,
    pub r29: u64,
    pub r30: u64,
    pub r31: u64,
    pub rip: u64,
    pub rflags: u64,
    /// XMM registers (128-bit each, stored as [low, high])
    /// XMM0-XMM15: bits 127:0 of ZMM0-ZMM15
    pub xmm: [[u64; 2]; 16],
    /// YMM upper 128-bits (bits 255:128 of YMM/ZMM registers 0-15)
    pub ymm_high: [[u64; 2]; 16],
    /// ZMM upper 256-bits (bits 511:256 of ZMM registers 0-15)
    /// Stored as [bits 319:256, bits 383:320, bits 447:384, bits 511:448]
    pub zmm_high: [[u64; 4]; 16],
    /// ZMM16-ZMM31 (full 512-bit registers, AVX-512 extended)
    /// Stored as [bits 63:0, bits 127:64, bits 191:128, bits 255:192,
    ///            bits 319:256, bits 383:320, bits 447:384, bits 511:448]
    pub zmm_ext: [[u64; 8]; 16],
    /// Opmask registers k0-k7 (64-bit each for AVX-512)
    pub k: [u64; 8],
    /// MMX registers (64-bit each, aliased to low 64 bits of x87 FPU stack)
    pub mm: [u64; 8],
}

/// Segment descriptor.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Segment {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub type_: u8,
    pub present: bool,
    pub dpl: u8,
    pub db: bool,
    pub s: bool,
    pub l: bool,
    pub g: bool,
    pub avl: bool,
    pub unusable: bool,
}

/// Descriptor table register (GDTR/IDTR).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DescriptorTable {
    pub base: u64,
    pub limit: u16,
}

/// System registers (control registers, segment registers, etc.).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SystemRegisters {
    pub cs: Segment,
    pub ds: Segment,
    pub es: Segment,
    pub fs: Segment,
    pub gs: Segment,
    pub ss: Segment,
    pub tr: Segment,
    pub ldt: Segment,
    pub gdt: DescriptorTable,
    pub idt: DescriptorTable,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
    /// IA32_STAR MSR (0xC0000081)
    pub star: u64,
    /// IA32_LSTAR MSR (0xC0000082)
    pub lstar: u64,
    /// IA32_CSTAR MSR (0xC0000083)
    pub cstar: u64,
    /// IA32_FMASK MSR (0xC0000084)
    pub fmask: u64,
    /// IA32_SYSENTER_CS MSR (0x174)
    pub sysenter_cs: u64,
    /// IA32_SYSENTER_ESP MSR (0x175)
    pub sysenter_esp: u64,
    /// IA32_SYSENTER_EIP MSR (0x176)
    pub sysenter_eip: u64,
    /// Debug registers DR0-DR3: breakpoint linear addresses
    pub dr0: u64,
    pub dr1: u64,
    pub dr2: u64,
    pub dr3: u64,
    /// Debug status register (DR6)
    pub dr6: u64,
    /// Debug control register (DR7)
    pub dr7: u64,
}

/// Complete x86_64 CPU state snapshot.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct X86_64CpuState {
    pub regs: Registers,
    pub sregs: SystemRegisters,
}

/// Hexagon general and control registers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HexagonRegisters {
    pub r: [u32; 32],
    /// Predicate registers P0..P3. Each is an 8-bit register (hardware width);
    /// scalar compares write 0x00/0xff, vector compares set per-lane bits, and
    /// conditional execution tests only the least-significant bit.
    pub p: [u8; 4],
    pub c: [u32; 32],
    /// HVX vector registers V0..V31. Each is 1024-bit (128 bytes), stored as 32
    /// little-endian u32 words.
    pub v: [[u32; 32]; 32],
    /// HVX vector predicate registers Q0..Q3 (128-bit, one bit per vector byte),
    /// stored as 4 u32 words each.
    pub q: [[u32; 4]; 4],
}

impl Default for HexagonRegisters {
    fn default() -> Self {
        HexagonRegisters {
            r: [0u32; 32],
            p: [0u8; 4],
            c: [0u32; 32],
            v: [[0u32; 32]; 32],
            q: [[0u32; 4]; 4],
        }
    }
}

impl HexagonRegisters {
    pub fn pc(&self) -> u32 {
        self.c[9]
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.c[9] = pc;
    }

    pub fn usr(&self) -> u32 {
        self.c[8]
    }

    pub fn set_usr(&mut self, value: u32) {
        self.c[8] = value;
    }

    pub fn predicate(&self, index: usize) -> u8 {
        self.p[index]
    }

    pub fn set_predicate(&mut self, index: usize, value: u8) {
        self.p[index] = value;
        self.c[4] = self.pack_predicates();
    }

    pub fn control(&self, index: usize) -> u32 {
        if index == 4 {
            self.pack_predicates()
        } else {
            self.c[index]
        }
    }

    pub fn set_control(&mut self, index: usize, value: u32) {
        if index == 4 {
            self.unpack_predicates(value);
        } else if index == 11 {
            // GP (C11): the low 6 bits are hardwired to zero (64-byte aligned).
            self.c[11] = value & !0x3f;
        } else {
            self.c[index] = value;
        }
    }

    fn pack_predicates(&self) -> u32 {
        let mut value = 0u32;
        for (idx, byte) in self.p.iter().enumerate() {
            value |= (*byte as u32) << (8 * idx);
        }
        value
    }

    fn unpack_predicates(&mut self, value: u32) {
        for idx in 0..self.p.len() {
            self.p[idx] = (value >> (8 * idx)) as u8;
        }
        self.c[4] = value;
    }
}

/// Complete Hexagon CPU state snapshot.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HexagonCpuState {
    pub regs: HexagonRegisters,
}

// =============================================================================
// RISC-V CPU State
// =============================================================================

/// RISC-V architectural register file (RV32/RV64; values stored XLEN-wide,
/// zero-extended to 64 bits on RV32).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiscVRegisters {
    /// Integer registers `x0..x31` (`x0` is hardwired zero).
    pub x: [u64; 32],
    /// Program counter.
    pub pc: u64,
    /// Floating-point registers `f0..f31` (raw bits, NaN-boxed for single).
    pub f: [u64; 32],
    /// Floating-point control/status (`frm` || `fflags`).
    pub fcsr: u32,
}

impl Default for RiscVRegisters {
    fn default() -> Self {
        RiscVRegisters {
            x: [0u64; 32],
            pc: 0,
            f: [0u64; 32],
            fcsr: 0,
        }
    }
}

impl RiscVRegisters {
    /// Program counter.
    pub fn pc(&self) -> u64 {
        self.pc
    }
    /// Set the program counter.
    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }
}

/// Complete RISC-V CPU state snapshot.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RiscVCpuState {
    pub regs: RiscVRegisters,
}

// =============================================================================
// ARM64 (AArch64) CPU State
// =============================================================================

/// AArch64 general-purpose and system registers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Aarch64Registers {
    /// General-purpose registers X0-X30
    pub x: [u64; 31],
    /// Stack pointer (SP_EL0 or SP_ELx depending on mode)
    pub sp: u64,
    /// Program counter
    pub pc: u64,
    /// Process state (PSTATE/CPSR equivalent)
    pub pstate: u64,
    /// SIMD/FP registers V0-V31 (128-bit each, stored as [low, high])
    pub v: [[u64; 2]; 32],
    /// Floating-point control register
    pub fpcr: u32,
    /// Floating-point status register
    pub fpsr: u32,
}

impl Default for Aarch64Registers {
    fn default() -> Self {
        Aarch64Registers {
            x: [0u64; 31],
            sp: 0,
            pc: 0,
            pstate: 0x3C5, // EL1h mode, all interrupts masked
            v: [[0u64; 2]; 32],
            fpcr: 0,
            fpsr: 0,
        }
    }
}

// PSTATE bit definitions for AArch64
impl Aarch64Registers {
    // PSTATE condition flags
    pub const PSTATE_N: u64 = 1 << 31; // Negative
    pub const PSTATE_Z: u64 = 1 << 30; // Zero
    pub const PSTATE_C: u64 = 1 << 29; // Carry
    pub const PSTATE_V: u64 = 1 << 28; // Overflow

    // PSTATE interrupt masks
    pub const PSTATE_D: u64 = 1 << 9; // Debug mask
    pub const PSTATE_A: u64 = 1 << 8; // SError mask
    pub const PSTATE_I: u64 = 1 << 7; // IRQ mask
    pub const PSTATE_F: u64 = 1 << 6; // FIQ mask

    // PSTATE execution state
    pub const PSTATE_SS: u64 = 1 << 21; // Software step
    pub const PSTATE_IL: u64 = 1 << 20; // Illegal execution state
    pub const PSTATE_nRW: u64 = 1 << 4; // Execution state (0=AArch64, 1=AArch32)
    pub const PSTATE_EL_MASK: u64 = 0x3 << 2; // Exception level
    pub const PSTATE_SP: u64 = 1 << 0; // Stack pointer select

    // Security/Speculation
    pub const PSTATE_PAN: u64 = 1 << 22; // Privileged Access Never
    pub const PSTATE_UAO: u64 = 1 << 23; // User Access Override
    pub const PSTATE_DIT: u64 = 1 << 24; // Data Independent Timing
    pub const PSTATE_TCO: u64 = 1 << 25; // Tag Check Override
    pub const PSTATE_BTYPE_MASK: u64 = 0x3 << 10; // Branch Type (for BTI)
    pub const PSTATE_SSBS: u64 = 1 << 12; // Speculative Store Bypass Safe

    /// Get the link register (X30/LR)
    pub fn lr(&self) -> u64 {
        self.x[30]
    }

    /// Set the link register (X30/LR)
    pub fn set_lr(&mut self, value: u64) {
        self.x[30] = value;
    }

    /// Get the frame pointer (X29/FP)
    pub fn fp(&self) -> u64 {
        self.x[29]
    }

    /// Set the frame pointer (X29/FP)
    pub fn set_fp(&mut self, value: u64) {
        self.x[29] = value;
    }

    /// Get current exception level (0-3)
    pub fn el(&self) -> u8 {
        ((self.pstate >> 2) & 0x3) as u8
    }

    /// Set exception level
    pub fn set_el(&mut self, el: u8) {
        self.pstate = (self.pstate & !Self::PSTATE_EL_MASK) | (((el & 0x3) as u64) << 2);
    }

    /// Check if IRQs are masked
    pub fn irq_masked(&self) -> bool {
        (self.pstate & Self::PSTATE_I) != 0
    }

    /// Check if FIQs are masked
    pub fn fiq_masked(&self) -> bool {
        (self.pstate & Self::PSTATE_F) != 0
    }

    /// Get condition flags as (N, Z, C, V)
    pub fn flags(&self) -> (bool, bool, bool, bool) {
        (
            (self.pstate & Self::PSTATE_N) != 0,
            (self.pstate & Self::PSTATE_Z) != 0,
            (self.pstate & Self::PSTATE_C) != 0,
            (self.pstate & Self::PSTATE_V) != 0,
        )
    }

    /// Set condition flags
    pub fn set_flags(&mut self, n: bool, z: bool, c: bool, v: bool) {
        self.pstate &= !(Self::PSTATE_N | Self::PSTATE_Z | Self::PSTATE_C | Self::PSTATE_V);
        if n {
            self.pstate |= Self::PSTATE_N;
        }
        if z {
            self.pstate |= Self::PSTATE_Z;
        }
        if c {
            self.pstate |= Self::PSTATE_C;
        }
        if v {
            self.pstate |= Self::PSTATE_V;
        }
    }
}

/// AArch64 system registers.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Aarch64SystemRegisters {
    /// System Control Register (EL1)
    pub sctlr_el1: u64,
    /// Translation Control Register (EL1)
    pub tcr_el1: u64,
    /// Translation Table Base Register 0 (EL1)
    pub ttbr0_el1: u64,
    /// Translation Table Base Register 1 (EL1)
    pub ttbr1_el1: u64,
    /// Memory Attribute Indirection Register (EL1)
    pub mair_el1: u64,
    /// Vector Base Address Register (EL1)
    pub vbar_el1: u64,
    /// Exception Syndrome Register (EL1)
    pub esr_el1: u64,
    /// Fault Address Register (EL1)
    pub far_el1: u64,
    /// Exception Link Register (EL1)
    pub elr_el1: u64,
    /// Saved Program Status Register (EL1)
    pub spsr_el1: u64,
    /// Stack Pointer (EL0)
    pub sp_el0: u64,
    /// Stack Pointer (EL1)
    pub sp_el1: u64,
    /// Thread ID Register (EL0, read-write)
    pub tpidr_el0: u64,
    /// Thread ID Register (EL1)
    pub tpidr_el1: u64,
    /// Thread ID Register (EL0, read-only)
    pub tpidrro_el0: u64,
    /// Counter-timer Physical Timer Control Register
    pub cntp_ctl_el0: u64,
    /// Counter-timer Physical Timer CompareValue Register
    pub cntp_cval_el0: u64,
    /// Counter-timer Virtual Timer Control Register
    pub cntv_ctl_el0: u64,
    /// Counter-timer Virtual Timer CompareValue Register
    pub cntv_cval_el0: u64,
}

/// Complete AArch64 CPU state snapshot.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Aarch64CpuState {
    pub regs: Aarch64Registers,
    pub sregs: Aarch64SystemRegisters,
}

// =============================================================================
// ARM32 (AArch32) CPU State
// =============================================================================

/// AArch32 (ARM32) general-purpose registers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Aarch32Registers {
    /// General-purpose registers R0-R12
    pub r: [u32; 13],
    /// Stack pointer (R13/SP)
    pub sp: u32,
    /// Link register (R14/LR)
    pub lr: u32,
    /// Program counter (R15/PC)
    pub pc: u32,
    /// Current Program Status Register
    pub cpsr: u32,
    /// Saved Program Status Register (for exception modes)
    pub spsr: u32,
    /// VFP/NEON single-precision registers S0-S31 (or D0-D15 pairs)
    pub s: [u32; 32],
    /// VFP/NEON double-precision high registers D16-D31 (ARMv7 NEON)
    pub d_high: [u64; 16],
    /// Floating-point Status and Control Register
    pub fpscr: u32,
}

impl Default for Aarch32Registers {
    fn default() -> Self {
        Aarch32Registers {
            r: [0u32; 13],
            sp: 0,
            lr: 0,
            pc: 0,
            cpsr: 0x1D3, // Supervisor mode, IRQ/FIQ disabled, ARM state
            spsr: 0,
            s: [0u32; 32],
            d_high: [0u64; 16],
            fpscr: 0,
        }
    }
}

// CPSR bit definitions for AArch32
impl Aarch32Registers {
    // Condition flags
    pub const CPSR_N: u32 = 1 << 31; // Negative
    pub const CPSR_Z: u32 = 1 << 30; // Zero
    pub const CPSR_C: u32 = 1 << 29; // Carry
    pub const CPSR_V: u32 = 1 << 28; // Overflow
    pub const CPSR_Q: u32 = 1 << 27; // Saturation (sticky)

    // IT block state (Thumb-2)
    pub const CPSR_IT_MASK: u32 = 0x0600FC00; // IT[7:0] in bits [26:25,15:10]

    // GE flags for SIMD
    pub const CPSR_GE_MASK: u32 = 0xF << 16;

    // Execution state and interrupts
    pub const CPSR_E: u32 = 1 << 9; // Endianness (0=LE, 1=BE)
    pub const CPSR_A: u32 = 1 << 8; // Async abort mask
    pub const CPSR_I: u32 = 1 << 7; // IRQ mask
    pub const CPSR_F: u32 = 1 << 6; // FIQ mask
    pub const CPSR_T: u32 = 1 << 5; // Thumb state

    // Processor modes
    pub const MODE_USR: u8 = 0b10000; // User
    pub const MODE_FIQ: u8 = 0b10001; // FIQ
    pub const MODE_IRQ: u8 = 0b10010; // IRQ
    pub const MODE_SVC: u8 = 0b10011; // Supervisor
    pub const MODE_MON: u8 = 0b10110; // Monitor (TrustZone)
    pub const MODE_ABT: u8 = 0b10111; // Abort
    pub const MODE_HYP: u8 = 0b11010; // Hypervisor
    pub const MODE_UND: u8 = 0b11011; // Undefined
    pub const MODE_SYS: u8 = 0b11111; // System

    /// Check if in Thumb state
    pub fn is_thumb(&self) -> bool {
        (self.cpsr & Self::CPSR_T) != 0
    }

    /// Set Thumb state
    pub fn set_thumb(&mut self, thumb: bool) {
        if thumb {
            self.cpsr |= Self::CPSR_T;
        } else {
            self.cpsr &= !Self::CPSR_T;
        }
    }

    /// Get processor mode (lower 5 bits of CPSR)
    pub fn mode(&self) -> u8 {
        (self.cpsr & 0x1F) as u8
    }

    /// Set processor mode
    pub fn set_mode(&mut self, mode: u8) {
        self.cpsr = (self.cpsr & !0x1F) | ((mode & 0x1F) as u32);
    }

    /// Check if IRQs are masked
    pub fn irq_masked(&self) -> bool {
        (self.cpsr & Self::CPSR_I) != 0
    }

    /// Check if FIQs are masked
    pub fn fiq_masked(&self) -> bool {
        (self.cpsr & Self::CPSR_F) != 0
    }

    /// Get condition flags as (N, Z, C, V)
    pub fn flags(&self) -> (bool, bool, bool, bool) {
        (
            (self.cpsr & Self::CPSR_N) != 0,
            (self.cpsr & Self::CPSR_Z) != 0,
            (self.cpsr & Self::CPSR_C) != 0,
            (self.cpsr & Self::CPSR_V) != 0,
        )
    }

    /// Set condition flags
    pub fn set_flags(&mut self, n: bool, z: bool, c: bool, v: bool) {
        self.cpsr &= !(Self::CPSR_N | Self::CPSR_Z | Self::CPSR_C | Self::CPSR_V);
        if n {
            self.cpsr |= Self::CPSR_N;
        }
        if z {
            self.cpsr |= Self::CPSR_Z;
        }
        if c {
            self.cpsr |= Self::CPSR_C;
        }
        if v {
            self.cpsr |= Self::CPSR_V;
        }
    }

    /// Get GE flags for SIMD byte operations
    pub fn ge_flags(&self) -> u8 {
        ((self.cpsr >> 16) & 0xF) as u8
    }

    /// Set GE flags
    pub fn set_ge_flags(&mut self, ge: u8) {
        self.cpsr = (self.cpsr & !Self::CPSR_GE_MASK) | (((ge & 0xF) as u32) << 16);
    }

    /// Check if in a privileged mode
    pub fn is_privileged(&self) -> bool {
        self.mode() != Self::MODE_USR
    }

    /// Get big-endian state
    pub fn is_big_endian(&self) -> bool {
        (self.cpsr & Self::CPSR_E) != 0
    }
}

/// AArch32 system registers (CP15).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Aarch32SystemRegisters {
    /// System Control Register (SCTLR)
    pub sctlr: u32,
    /// Translation Table Base Register 0
    pub ttbr0: u32,
    /// Translation Table Base Register 1
    pub ttbr1: u32,
    /// Translation Table Base Control Register
    pub ttbcr: u32,
    /// Domain Access Control Register
    pub dacr: u32,
    /// Data Fault Status Register
    pub dfsr: u32,
    /// Instruction Fault Status Register
    pub ifsr: u32,
    /// Data Fault Address Register
    pub dfar: u32,
    /// Instruction Fault Address Register
    pub ifar: u32,
    /// Vector Base Address Register
    pub vbar: u32,
    /// Context ID Register
    pub contextidr: u32,
    /// Primary Region Remap Register
    pub prrr: u32,
    /// Normal Memory Remap Register
    pub nmrr: u32,
}

/// Complete AArch32 CPU state snapshot.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Aarch32CpuState {
    pub regs: Aarch32Registers,
    pub sregs: Aarch32SystemRegisters,
}

// =============================================================================
// Cortex-M CPU State (ARMv6-M/ARMv7-M/ARMv8-M)
// =============================================================================

/// Cortex-M core registers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CortexMRegisters {
    /// General-purpose registers R0-R12
    pub r: [u32; 13],
    /// Main Stack Pointer (MSP) - R13 when using MSP
    pub msp: u32,
    /// Process Stack Pointer (PSP) - R13 when using PSP
    pub psp: u32,
    /// Link Register (R14/LR)
    pub lr: u32,
    /// Program Counter (R15/PC)
    pub pc: u32,
    /// Program Status Register (combines APSR, IPSR, EPSR)
    pub xpsr: u32,
    /// CONTROL register
    pub control: u32,
    /// Priority Mask Register
    pub primask: u32,
    /// Fault Mask Register (ARMv7-M and later)
    pub faultmask: u32,
    /// Base Priority Register (ARMv7-M and later)
    pub basepri: u32,
    /// FP registers S0-S31 (if FPU present, ARMv7-M and later)
    pub s: [u32; 32],
    /// Floating-point Status and Control Register
    pub fpscr: u32,
}

impl Default for CortexMRegisters {
    fn default() -> Self {
        CortexMRegisters {
            r: [0u32; 13],
            msp: 0,
            psp: 0,
            lr: 0xFFFF_FFFF, // Reset value indicating return to Thread mode
            pc: 0,
            xpsr: 0x0100_0000, // Thumb bit set (Cortex-M only runs Thumb)
            control: 0,
            primask: 0,
            faultmask: 0,
            basepri: 0,
            s: [0u32; 32],
            fpscr: 0,
        }
    }
}

// xPSR bit definitions for Cortex-M
impl CortexMRegisters {
    // APSR (Application PSR) - condition flags
    pub const XPSR_N: u32 = 1 << 31; // Negative
    pub const XPSR_Z: u32 = 1 << 30; // Zero
    pub const XPSR_C: u32 = 1 << 29; // Carry
    pub const XPSR_V: u32 = 1 << 28; // Overflow
    pub const XPSR_Q: u32 = 1 << 27; // Saturation (DSP extension)

    // GE flags (ARMv7E-M with DSP)
    pub const XPSR_GE_MASK: u32 = 0xF << 16;

    // ICI/IT state (for interrupt-continuable instructions and IT blocks)
    pub const XPSR_ICI_IT_MASK: u32 = 0x0600FC00;

    // EPSR
    pub const XPSR_T: u32 = 1 << 24; // Thumb bit (always 1 on Cortex-M)

    // IPSR - exception number
    pub const XPSR_EXCEPTION_MASK: u32 = 0x1FF;

    // CONTROL register bits
    pub const CONTROL_NPRIV: u32 = 1 << 0; // Thread mode privilege (0=privileged, 1=unprivileged)
    pub const CONTROL_SPSEL: u32 = 1 << 1; // Stack pointer select (0=MSP, 1=PSP)
    pub const CONTROL_FPCA: u32 = 1 << 2; // FP context active (M4F+)

    // Special EXC_RETURN values
    pub const EXC_RETURN_HANDLER_MSP: u32 = 0xFFFFFFF1;
    pub const EXC_RETURN_THREAD_MSP: u32 = 0xFFFFFFF9;
    pub const EXC_RETURN_THREAD_PSP: u32 = 0xFFFFFFFD;
    // With FPU (M4F+)
    pub const EXC_RETURN_HANDLER_MSP_FP: u32 = 0xFFFFFFE1;
    pub const EXC_RETURN_THREAD_MSP_FP: u32 = 0xFFFFFFE9;
    pub const EXC_RETURN_THREAD_PSP_FP: u32 = 0xFFFFFFED;

    // Exception numbers
    pub const EXC_RESET: u32 = 1;
    pub const EXC_NMI: u32 = 2;
    pub const EXC_HARDFAULT: u32 = 3;
    pub const EXC_MEMMANAGE: u32 = 4;
    pub const EXC_BUSFAULT: u32 = 5;
    pub const EXC_USAGEFAULT: u32 = 6;
    pub const EXC_SVCALL: u32 = 11;
    pub const EXC_DEBUGMON: u32 = 12;
    pub const EXC_PENDSV: u32 = 14;
    pub const EXC_SYSTICK: u32 = 15;
    pub const EXC_IRQ0: u32 = 16; // First external IRQ

    /// Get current stack pointer based on CONTROL.SPSEL
    pub fn sp(&self) -> u32 {
        if (self.control & Self::CONTROL_SPSEL) != 0 {
            self.psp
        } else {
            self.msp
        }
    }

    /// Set current stack pointer based on CONTROL.SPSEL
    pub fn set_sp(&mut self, value: u32) {
        if (self.control & Self::CONTROL_SPSEL) != 0 {
            self.psp = value;
        } else {
            self.msp = value;
        }
    }

    /// Check if in privileged mode
    pub fn is_privileged(&self) -> bool {
        // In Handler mode, always privileged
        // In Thread mode, check CONTROL.nPRIV
        self.in_handler_mode() || (self.control & Self::CONTROL_NPRIV) == 0
    }

    /// Check if in handler mode (processing an exception)
    pub fn in_handler_mode(&self) -> bool {
        (self.xpsr & Self::XPSR_EXCEPTION_MASK) != 0
    }

    /// Check if in thread mode
    pub fn in_thread_mode(&self) -> bool {
        !self.in_handler_mode()
    }

    /// Get current exception number (0 if in thread mode)
    pub fn exception_number(&self) -> u32 {
        self.xpsr & Self::XPSR_EXCEPTION_MASK
    }

    /// Get condition flags as (N, Z, C, V)
    pub fn flags(&self) -> (bool, bool, bool, bool) {
        (
            (self.xpsr & Self::XPSR_N) != 0,
            (self.xpsr & Self::XPSR_Z) != 0,
            (self.xpsr & Self::XPSR_C) != 0,
            (self.xpsr & Self::XPSR_V) != 0,
        )
    }

    /// Set condition flags
    pub fn set_flags(&mut self, n: bool, z: bool, c: bool, v: bool) {
        self.xpsr &= !(Self::XPSR_N | Self::XPSR_Z | Self::XPSR_C | Self::XPSR_V);
        if n {
            self.xpsr |= Self::XPSR_N;
        }
        if z {
            self.xpsr |= Self::XPSR_Z;
        }
        if c {
            self.xpsr |= Self::XPSR_C;
        }
        if v {
            self.xpsr |= Self::XPSR_V;
        }
    }

    /// Check if using PSP
    pub fn using_psp(&self) -> bool {
        (self.control & Self::CONTROL_SPSEL) != 0
    }
}

/// Cortex-M system registers (SCS - System Control Space).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CortexMSystemRegisters {
    /// Interrupt Control and State Register
    pub icsr: u32,
    /// Vector Table Offset Register
    pub vtor: u32,
    /// Application Interrupt and Reset Control Register
    pub aircr: u32,
    /// System Control Register
    pub scr: u32,
    /// Configuration and Control Register
    pub ccr: u32,
    /// System Handler Priority Registers
    pub shpr: [u32; 3],
    /// System Handler Control and State Register
    pub shcsr: u32,
    /// Configurable Fault Status Register
    pub cfsr: u32,
    /// HardFault Status Register
    pub hfsr: u32,
    /// Debug Fault Status Register
    pub dfsr: u32,
    /// MemManage Fault Address Register
    pub mmfar: u32,
    /// BusFault Address Register
    pub bfar: u32,
    /// Auxiliary Fault Status Register
    pub afsr: u32,
    /// Coprocessor Access Control Register
    pub cpacr: u32,
}

/// Complete Cortex-M CPU state snapshot.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CortexMCpuState {
    pub regs: CortexMRegisters,
    pub sregs: CortexMSystemRegisters,
}

// =============================================================================
// Architecture-agnostic CPU State Enum
// =============================================================================

/// Architecture-specific CPU state snapshot.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CpuState {
    X86_64(X86_64CpuState),
    Hexagon(HexagonCpuState),
    Aarch64(Aarch64CpuState),
    Aarch32(Aarch32CpuState),
    CortexM(CortexMCpuState),
    RiscV(RiscVCpuState),
}

impl CpuState {
    pub fn x86_64(regs: Registers, sregs: SystemRegisters) -> Self {
        CpuState::X86_64(X86_64CpuState { regs, sregs })
    }

    pub fn hexagon(regs: HexagonRegisters) -> Self {
        CpuState::Hexagon(HexagonCpuState { regs })
    }

    pub fn riscv(regs: RiscVRegisters) -> Self {
        CpuState::RiscV(RiscVCpuState { regs })
    }

    pub fn as_riscv(&self) -> Option<&RiscVCpuState> {
        match self {
            CpuState::RiscV(state) => Some(state),
            _ => None,
        }
    }

    pub fn aarch64(regs: Aarch64Registers, sregs: Aarch64SystemRegisters) -> Self {
        CpuState::Aarch64(Aarch64CpuState { regs, sregs })
    }

    pub fn aarch32(regs: Aarch32Registers, sregs: Aarch32SystemRegisters) -> Self {
        CpuState::Aarch32(Aarch32CpuState { regs, sregs })
    }

    pub fn cortex_m(regs: CortexMRegisters, sregs: CortexMSystemRegisters) -> Self {
        CpuState::CortexM(CortexMCpuState { regs, sregs })
    }

    pub fn as_aarch64(&self) -> Option<&Aarch64CpuState> {
        match self {
            CpuState::Aarch64(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_aarch32(&self) -> Option<&Aarch32CpuState> {
        match self {
            CpuState::Aarch32(state) => Some(state),
            _ => None,
        }
    }

    pub fn as_cortex_m(&self) -> Option<&CortexMCpuState> {
        match self {
            CpuState::CortexM(state) => Some(state),
            _ => None,
        }
    }
}
