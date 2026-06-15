//! ARMv6 (ARM1176 / Samsung S3C64xx) vCPU: boots 32-bit ARM Linux on the
//! software emulator.
//!
//! Drives the AArch32 [`Armv7Cpu`] + [`Executor`] over guest memory with:
//!
//! - the ARMv6 short-descriptor MMU ([`crate::arm::mmu_v6`]) applied in the
//!   memory bridge, configured from the CPU's CP15 state each step,
//! - architectural exception delivery (SVC, undef, data/prefetch aborts with
//!   DFSR/DFAR/IFSR/IFAR, IRQ from the VICs),
//! - the S3C64xx platform devices (PL192 VIC pair, Samsung UART console,
//!   PWM timer block, SYSCON/clock block) served at their physical windows.
//!
//! Boot protocol (DT): r0 = 0, r1 = 0xFFFF_FFFF, r2 = DTB physical address,
//! SVC mode, IRQ/FIQ masked, MMU off.

use std::sync::{Arc, Mutex, OnceLock};

use tracing::debug;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use crate::arm::execution::{ArmMemory, MemoryError};
use crate::arm::mmu_v6::{self, V6Access, V6Fault, V6MmuConfig};
use crate::arm::{
    Armv7Cpu, Decoder, ExceptionType, ExecResult, ExecutionState, Executor, Mnemonic,
    ProcessorMode,
};
use crate::cpu::{Aarch32CpuState, Aarch32Registers, Aarch32SystemRegisters, CpuState, VCpu,
    VcpuExit};
use crate::devices::s3c64xx::{Pl192Vic, S3cPwmTimer, S3cSyscon, S3cUart};
use crate::error::{Error, Result};

/// Physical RAM window (SMDK6410: 128MB at 0x50000000).
pub const S3C_RAM_BASE: u32 = 0x5000_0000;
pub const S3C_RAM_SIZE: u32 = 0x0800_0000;

const VIC0_BASE: u32 = 0x7120_0000;
const VIC1_BASE: u32 = 0x7130_0000;
const SYSCON_BASE: u32 = 0x7E00_F000;
const UART0_BASE: u32 = 0x7F00_5000;
const PWM_BASE: u32 = 0x7F00_6000;

/// VIC line assignments (S3C64xx interrupt map).
const VIC0_TIMER_LINES: [u32; 5] = [23, 24, 25, 27, 28];
const VIC1_UART0_LINE: u32 = 5;

/// Instructions per `run()` batch before yielding to the VMM loop.
const BATCH: u32 = 65_536;

/// Last memory fault recorded by the bridge (for DFSR/DFAR reporting).
#[derive(Clone, Copy, Default)]
struct LastFault {
    addr: u32,
    fsr: u32,
    domain: u32,
    /// 0=read 1=write 2=execute, bit2 set if privileged.
    access: u32,
}

/// Memory bridge: VA->PA via the v6 MMU, device windows at their physical
/// addresses, RAM through the VMM's guest memory.
struct S3cBridge {
    mem: Arc<GuestMemoryMmap>,
    inner: std::cell::RefCell<BridgeInner>,
}

/// Mutable bridge internals (RefCell: the executor's read path is `&self`).
struct BridgeInner {
    mmu: V6MmuConfig,
    privileged: bool,
    last_fault: LastFault,
    vic0: Pl192Vic,
    vic1: Pl192Vic,
    pwm: S3cPwmTimer,
    syscon: S3cSyscon,
    uart: Arc<OnceLock<Arc<Mutex<S3cUart>>>>,
    /// Debug: user-mode write watchpoint (RAX_WATCH_WRITE=<hex VA>).
    watch_va: Option<u32>,
    watch_hit: bool,
    watch_pa: u32,
    watch_access: u32,
}

impl S3cBridge {
    fn translate(
        mem: &GuestMemoryMmap,
        inner: &mut BridgeInner,
        va: u32,
        access: V6Access,
    ) -> std::result::Result<u32, MemoryError> {
        let walk = |pa: u32| -> Option<u32> {
            let mut buf = [0u8; 4];
            mem.read_slice(&mut buf, GuestAddress(pa as u64)).ok()?;
            Some(u32::from_le_bytes(buf))
        };
        match mmu_v6::translate_v6(&inner.mmu, va, inner.privileged, access, walk) {
            Ok(t) => Ok(t.pa),
            Err(V6Fault { fsr, domain }) => {
                inner.last_fault = LastFault {
                    addr: va,
                    fsr,
                    domain,
                    access: match access {
                        V6Access::Read => 0,
                        V6Access::Write => 1,
                        V6Access::Execute => 2,
                    } | if inner.privileged { 4 } else { 0 },
                };
                Err(MemoryError::PermissionDenied(va))
            }
        }
    }

}

impl BridgeInner {
    /// Device window read at a PHYSICAL address. Returns None for RAM.
    fn dev_read(&mut self, pa: u32) -> Option<u32> {
        match pa {
            _ if (VIC0_BASE..VIC0_BASE + 0x1000).contains(&pa) => {
                Some(self.vic0.read(pa - VIC0_BASE))
            }
            _ if (VIC1_BASE..VIC1_BASE + 0x1000).contains(&pa) => {
                Some(self.vic1.read(pa - VIC1_BASE))
            }
            _ if (SYSCON_BASE..SYSCON_BASE + 0x1000).contains(&pa) => {
                Some(self.syscon.read(pa - SYSCON_BASE))
            }
            _ if (UART0_BASE..UART0_BASE + 0x100).contains(&pa) => Some(
                self.uart
                    .get()
                    .and_then(|u| u.lock().ok().map(|mut u| u.read(pa - UART0_BASE)))
                    .unwrap_or(0),
            ),
            _ if (PWM_BASE..PWM_BASE + 0x1000).contains(&pa) => {
                Some(self.pwm.read(pa - PWM_BASE))
            }
            // RAM?
            _ if (S3C_RAM_BASE..S3C_RAM_BASE + S3C_RAM_SIZE).contains(&pa) => None,
            // Anything else: open bus, reads as zero (probing drivers cope
            // better with RAZ/WI than with aborts).
            _ => Some(0),
        }
    }

    fn dev_write(&mut self, pa: u32, value: u32) -> bool {
        match pa {
            _ if (VIC0_BASE..VIC0_BASE + 0x1000).contains(&pa) => {
                self.vic0.write(pa - VIC0_BASE, value);
                true
            }
            _ if (VIC1_BASE..VIC1_BASE + 0x1000).contains(&pa) => {
                self.vic1.write(pa - VIC1_BASE, value);
                true
            }
            _ if (SYSCON_BASE..SYSCON_BASE + 0x1000).contains(&pa) => {
                self.syscon.write(pa - SYSCON_BASE, value);
                true
            }
            _ if (UART0_BASE..UART0_BASE + 0x100).contains(&pa) => {
                if let Some(u) = self.uart.get() {
                    if let Ok(mut u) = u.lock() {
                        u.write(pa - UART0_BASE, value);
                    }
                }
                true
            }
            _ if (PWM_BASE..PWM_BASE + 0x1000).contains(&pa) => {
                self.pwm.write(pa - PWM_BASE, value);
                true
            }
            _ if (S3C_RAM_BASE..S3C_RAM_BASE + S3C_RAM_SIZE).contains(&pa) => false,
            _ => true, // WI
        }
    }

    fn read_pa(
        &mut self,
        mem: &GuestMemoryMmap,
        pa: u32,
        buf: &mut [u8],
    ) -> std::result::Result<(), MemoryError> {
        if buf.len() <= 4 && !(S3C_RAM_BASE..S3C_RAM_BASE + S3C_RAM_SIZE).contains(&pa) {
            if let Some(v) = self.dev_read(pa & !0x3) {
                let lane = (pa & 0x3) as usize;
                let bytes = v.to_le_bytes();
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = *bytes.get(lane + i).unwrap_or(&0);
                }
                return Ok(());
            }
        }
        mem.read_slice(buf, GuestAddress(pa as u64))
            .map_err(|_| MemoryError::BusError(pa))
    }

    fn write_pa(
        &mut self,
        mem: &GuestMemoryMmap,
        pa: u32,
        data: &[u8],
    ) -> std::result::Result<(), MemoryError> {
        if data.len() <= 4 {
            let reg = pa & !0x3;
            let lane = (pa & 0x3) as usize;
            // Only consult devices when the address is outside RAM.
            if !(S3C_RAM_BASE..S3C_RAM_BASE + S3C_RAM_SIZE).contains(&pa) {
                let mut cur = self.dev_read(reg).unwrap_or(0).to_le_bytes();
                for (i, b) in data.iter().enumerate() {
                    if lane + i < 4 {
                        cur[lane + i] = *b;
                    }
                }
                self.dev_write(reg, u32::from_le_bytes(cur));
                return Ok(());
            }
        }
        mem.write_slice(data, GuestAddress(pa as u64))
            .map_err(|_| MemoryError::BusError(pa))
    }
}

impl S3cBridge {
    fn access(
        &self,
        addr: u32,
        access: V6Access,
        data: Option<&[u8]>,
        out: Option<&mut [u8]>,
    ) -> std::result::Result<(), MemoryError> {
        let mut inner = self.inner.borrow_mut();
        let watched = inner.watch_va.is_some_and(|w| addr & !3 == w);
        let pa = Self::translate(&self.mem, &mut inner, addr, access)?;
        if watched {
            inner.watch_hit = true;
            inner.watch_pa = pa;
            inner.watch_access = match access {
                V6Access::Read => 0,
                V6Access::Write => 1,
                V6Access::Execute => 2,
            } | if inner.privileged { 4 } else { 0 };
        }
        if let Some(buf) = out {
            inner.read_pa(&self.mem, pa, buf)
        } else if let Some(d) = data {
            inner.write_pa(&self.mem, pa, d)
        } else {
            Ok(())
        }
    }
}

impl ArmMemory for S3cBridge {
    fn read_word(&self, addr: u32) -> std::result::Result<u32, MemoryError> {
        let mut b = [0u8; 4];
        self.access(addr, V6Access::Read, None, Some(&mut b))?;
        Ok(u32::from_le_bytes(b))
    }

    fn write_word(&mut self, addr: u32, value: u32) -> std::result::Result<(), MemoryError> {
        self.access(addr, V6Access::Write, Some(&value.to_le_bytes()), None)
    }

    fn read_halfword(&self, addr: u32) -> std::result::Result<u16, MemoryError> {
        let mut b = [0u8; 2];
        self.access(addr, V6Access::Read, None, Some(&mut b))?;
        Ok(u16::from_le_bytes(b))
    }

    fn write_halfword(&mut self, addr: u32, value: u16) -> std::result::Result<(), MemoryError> {
        self.access(addr, V6Access::Write, Some(&value.to_le_bytes()), None)
    }

    fn read_byte(&self, addr: u32) -> std::result::Result<u8, MemoryError> {
        let mut b = [0u8; 1];
        self.access(addr, V6Access::Read, None, Some(&mut b))?;
        Ok(b[0])
    }

    fn write_byte(&mut self, addr: u32, value: u8) -> std::result::Result<(), MemoryError> {
        self.access(addr, V6Access::Write, Some(&[value]), None)
    }
}

/// ARMv6 vCPU backed by the AArch32 interpreter.
pub struct Armv6Vcpu {
    id: u32,
    cpu: Armv7Cpu,
    bridge: S3cBridge,
    decoder: Decoder,
    uart: Arc<OnceLock<Arc<Mutex<S3cUart>>>>,
    insn_count: u64,
    excl: crate::arm::instructions::ExclusiveMonitor,
    pc_ring: [u32; 512],
    pc_ring_idx: usize,
    vector_log_budget: u32,
    trace_log_budget: u32,
    trace_pcs: Vec<u32>,
    fault_log_budget: u32,
    last_heartbeat: std::time::Instant,
    shutdown: bool,
}

impl Armv6Vcpu {
    pub fn new(id: u32, mem: Arc<GuestMemoryMmap>) -> Self {
        let uart = Arc::new(OnceLock::new());
        let bridge = S3cBridge {
            mem,
            inner: std::cell::RefCell::new(BridgeInner {
                mmu: V6MmuConfig::default(),
                privileged: true,
                last_fault: LastFault::default(),
                vic0: Pl192Vic::new(),
                vic1: Pl192Vic::new(),
                pwm: S3cPwmTimer::new(),
                syscon: S3cSyscon::new(),
                uart: uart.clone(),
                watch_va: std::env::var("RAX_WATCH_WRITE")
                    .ok()
                    .and_then(|v| u32::from_str_radix(v.trim_start_matches("0x"), 16).ok()),
                watch_hit: false,
                watch_pa: 0,
                watch_access: 0,
            }),
        };
        let mut cpu = Armv7Cpu::new();
        // Identify as an ARM1176JZF-S (ARMv6K), the CPU this kernel's
        // proc-v6 support matches. The v6 cache-type register format matters
        // to cacheid_init().
        cpu.cp15.midr = 0x410F_B767;
        cpu.cp15.ctr = 0x1D15_2152;
        Armv6Vcpu {
            id,
            cpu,
            bridge,
            decoder: Decoder::new_aarch32(),
            uart,
            insn_count: 0,
            excl: crate::arm::instructions::ExclusiveMonitor::default(),
            pc_ring: [0; 512],
            pc_ring_idx: 0,
            vector_log_budget: 60,
            trace_log_budget: 300,
            trace_pcs: std::env::var("RAX_TRACE_PC")
                .ok()
                .map(|v| {
                    v.split(',')
                        .filter_map(|a| u32::from_str_radix(a.trim_start_matches("0x"), 16).ok())
                        .collect()
                })
                .unwrap_or_default(),
            fault_log_budget: 32,
            last_heartbeat: std::time::Instant::now(),
            shutdown: false,
        }
    }

    /// Refresh the bridge's MMU view from CP15 and the current mode.
    fn sync_mmu(&mut self) {
        let cp = &self.cpu.cp15;
        let mut inner = self.bridge.inner.borrow_mut();
        inner.mmu = V6MmuConfig {
            enabled: cp.sctlr.m(),
            ttbr0: cp.ttbr0 as u32,
            ttbr1: cp.ttbr1 as u32,
            ttbcr_n: cp.ttbcr & 0x7,
            dacr: cp.dacr,
            afe: false,
        };
        inner.privileged = self.cpu.cpsr.mode != ProcessorMode::User as u8;
    }

    /// Update VIC line levels from device state and return whether the CPU
    /// IRQ input is asserted.
    fn sync_irqs(&mut self) -> bool {
        let mut inner = self.bridge.inner.borrow_mut();
        for (t, line) in VIC0_TIMER_LINES.iter().enumerate() {
            let lvl = inner.pwm.irq_pending(t);
            inner.vic0.set_line(*line, lvl);
        }
        let uart_lvl = self
            .uart
            .get()
            .and_then(|u| u.lock().ok().map(|u| u.irq_pending()))
            .unwrap_or(false);
        inner.vic1.set_line(VIC1_UART0_LINE, uart_lvl);
        inner.vic0.irq_asserted() || inner.vic1.irq_asserted()
    }

    fn take_exception(&mut self, exc: ExceptionType) {
        let vbar = self.cpu.cp15.sctlr.vector_base();
        let mut exec = Executor::with_vbar(&mut self.cpu, &mut self.bridge, vbar);
        exec.take_exception(exc);
    }

    /// Execute one instruction with full system semantics.
    fn step(&mut self) -> StepOutcome {
        self.sync_mmu();
        self.bridge.inner.borrow_mut().pwm.tick(1);

        // IRQ delivery. A pending interrupt wakes WFI even while CPSR.I is
        // set (the architectural WFI wake condition); it is only DELIVERED
        // once IRQs are unmasked.
        let irq_pending = self.sync_irqs();
        if irq_pending {
            self.cpu.is_halted = false;
        }
        if irq_pending && !self.cpu.cpsr.i {
            let from = self.cpu.regs[15];
            self.take_exception(ExceptionType::Irq);
            if self.fault_log_budget > 0 {
                debug!(
                    from = format!("{from:#x}"),
                    to = format!("{:#x}", self.cpu.regs[15]),
                    insns = self.insn_count,
                    "irq delivered"
                );
            }
            return StepOutcome::Progress;
        }

        if self.cpu.is_halted {
            // WFI: idle until an interrupt line rises. Let time advance
            // faster while idle so timer deadlines arrive.
            self.bridge.inner.borrow_mut().pwm.tick(256);
            return StepOutcome::Idle;
        }

        let pc = self.cpu.regs[15];
        self.pc_ring[self.pc_ring_idx] = pc;
        self.pc_ring_idx = (self.pc_ring_idx + 1) % self.pc_ring.len();
        let is_thumb = self.cpu.cpsr.t;

        // Fetch (with prefetch-abort delivery). Decode bytes are kept in
        // MEMORY order (hw1 then hw2) — the decoder reads hw1 from bytes[0..2]
        // and hw2 from bytes[2..4], so a 32-bit Thumb word must not be packed
        // as (hw1<<16)|hw2 and little-endian-serialised (that swaps the
        // halfwords).
        let mut insn_len = 4u32;
        let mut decode_bytes = [0u8; 4];
        let raw = if is_thumb {
            let hw1 = match self.bridge.read_halfword(pc) {
                Ok(v) => v,
                Err(_) => return self.prefetch_abort(pc),
            };
            decode_bytes[0..2].copy_from_slice(&hw1.to_le_bytes());
            if (hw1 >> 11) >= 0x1D {
                let hw2 = match self.bridge.read_halfword(pc.wrapping_add(2)) {
                    Ok(v) => v,
                    Err(_) => return self.prefetch_abort(pc),
                };
                decode_bytes[2..4].copy_from_slice(&hw2.to_le_bytes());
                ((hw1 as u32) << 16) | hw2 as u32
            } else {
                insn_len = 2;
                hw1 as u32
            }
        } else {
            let w = match self.bridge.read_word(pc) {
                Ok(v) => v,
                Err(_) => return self.prefetch_abort(pc),
            };
            decode_bytes.copy_from_slice(&w.to_le_bytes());
            w
        };
        // Patch up the fetch's fault classification (translate() recorded a
        // data-style fault; prefetch aborts report via IFSR/IFAR instead).

        if self.trace_pcs.contains(&pc) && self.trace_log_budget > 0 {
            self.trace_log_budget -= 1;
            let regs: Vec<String> =
                (0..16).map(|i| format!("r{i}={:#x}", self.cpu.regs[i])).collect();
            debug!(regs = regs.join(" "), cpsr = format!("{:#x}", self.cpu.cpsr.to_u32()), insns = self.insn_count, "trace-pc hit");
        }
        if pc >= 0xFFFF_0000 && self.vector_log_budget > 0 {
            self.vector_log_budget -= 1;
            debug!(
                pc = format!("{pc:#x}"),
                raw = format!("{raw:#010x}"),
                lr = format!("{:#x}", self.cpu.regs[14]),
                sp = format!("{:#x}", self.cpu.regs[13]),
                cpsr = format!("{:#x}", self.cpu.cpsr.to_u32()),
                insns = self.insn_count,
                "vector-page exec"
            );
        }

        let state = if is_thumb {
            ExecutionState::Thumb
        } else {
            ExecutionState::Aarch32
        };
        let slice: &[u8] = if insn_len == 2 {
            &decode_bytes[..2]
        } else {
            &decode_bytes
        };
        self.decoder.set_state(state);
        let insn = match self.decoder.decode(slice) {
            Ok(i) => i,
            Err(_) => {
                self.take_exception(ExceptionType::UndefinedInstruction);
                return StepOutcome::Progress;
            }
        };

        let vbar = self.cpu.cp15.sctlr.vector_base();
        let mut exec = Executor::with_vbar(&mut self.cpu, &mut self.bridge, vbar);
        // Exclusive monitor state must survive across instructions (the
        // executor is rebuilt per step).
        exec.exclusive_monitor = self.excl.clone();
        let result = exec.execute(&insn);
        self.excl = exec.exclusive_monitor.clone();
        self.insn_count += 1;

        if self.bridge.inner.borrow().watch_hit {
            let (wpa, wacc) = {
                let mut i = self.bridge.inner.borrow_mut();
                i.watch_hit = false;
                (i.watch_pa, i.watch_access)
            };
            if self.trace_log_budget > 0 {
                self.trace_log_budget -= 1;
                let mut b = [0u8; 4];
                let _ = self.bridge.mem.read_slice(&mut b, GuestAddress(wpa as u64));
                debug!(
                    pc = format!("{pc:#x}"),
                    raw = format!("{raw:#010x}"),
                    pa = format!("{wpa:#x}"),
                    access = wacc,
                    val = format!("{:#x}", u32::from_le_bytes(b)),
                    insns = self.insn_count,
                    "watch hit"
                );
            }
        }

        match result {
            ExecResult::Continue => {
                self.cpu.regs[15] = self.cpu.regs[15].wrapping_add(insn_len);
                if self.cpu.cpsr.in_it_block() {
                    self.cpu.cpsr.advance_it_state();
                }
                StepOutcome::Progress
            }
            ExecResult::Branch(target) => {
                if insn.mnemonic == Mnemonic::RFE {
                    // RFE restored CPSR (including T) itself.
                    self.cpu.regs[15] = target & !1;
                } else if target & 1 != 0 {
                    self.cpu.cpsr.t = true;
                    self.cpu.regs[15] = target & !1;
                } else {
                    // ARM<->Thumb interworking for BX-style branches is
                    // handled by the executor through the T bit in `target`;
                    // word-aligned targets stay in the current state unless
                    // the instruction switched it.
                    self.cpu.regs[15] = target;
                }
                StepOutcome::Progress
            }
            ExecResult::Exception(exc) => {
                // regs[15] still points at the SVC itself; take_exception
                // computes LR = pc + 4 (the next instruction). Advancing here
                // too would double-count and skip an instruction on return.
                self.take_exception(exc);
                StepOutcome::Progress
            }
            ExecResult::Halt => {
                self.cpu.is_halted = true;
                StepOutcome::Idle
            }
            ExecResult::Undefined => {
                self.take_exception(ExceptionType::UndefinedInstruction);
                StepOutcome::Progress
            }
            ExecResult::MemoryFault(_) => {
                let f = self.bridge.inner.borrow().last_fault;
                if self.fault_log_budget > 0 {
                    self.fault_log_budget -= 1;
                    debug!(
                        pc = format!("{:#x}", self.cpu.regs[15]),
                        addr = format!("{:#x}", f.addr),
                        fsr = format!("{:#x}", f.fsr),
                        insns = self.insn_count,
                        "data abort"
                    );
                    if f.addr < 0x1000 {
                        let mut trace = Vec::with_capacity(self.pc_ring.len());
                        for i in 0..self.pc_ring.len() {
                            let idx = (self.pc_ring_idx + i) % self.pc_ring.len();
                            trace.push(format!("{:#x}", self.pc_ring[idx]));
                        }
                        let regs: Vec<String> =
                            (0..16).map(|i| format!("r{i}={:#x}", self.cpu.regs[i])).collect();
                        debug!(trace = trace.join(" "), regs = regs.join(" "), "pc trace before data abort");
                    }
                }
                // DFSR: FS[3:0] | domain[7:4] | WnR (bit 11). The kernel's
                // fault handler tests WnR to raise FAULT_FLAG_WRITE (COW and
                // dirty-bit emulation depend on it).
                self.cpu.cp15.dfsr =
                    f.fsr | (f.domain << 4) | if f.access & 3 == 1 { 1 << 11 } else { 0 };
                self.cpu.cp15.dfar = f.addr;
                self.take_exception(ExceptionType::DataAbort(f.addr));
                StepOutcome::Progress
            }
        }
    }

    fn prefetch_abort(&mut self, pc: u32) -> StepOutcome {
        let f = self.bridge.inner.borrow().last_fault;
        if self.fault_log_budget > 0 {
            self.fault_log_budget -= 1;
            debug!(
                pc = format!("{pc:#x}"),
                fsr = format!("{:#x}", f.fsr),
                lr = format!("{:#x}", self.cpu.regs[14]),
                insns = self.insn_count,
                "prefetch abort"
            );
            if pc < 0x1000 {
                let mut trace = Vec::with_capacity(self.pc_ring.len());
                for i in 0..self.pc_ring.len() {
                    let idx = (self.pc_ring_idx + i) % self.pc_ring.len();
                    trace.push(format!("{:#x}", self.pc_ring[idx]));
                }
                let regs: Vec<String> =
                    (0..16).map(|i| format!("r{i}={:#x}", self.cpu.regs[i])).collect();
                debug!(trace = trace.join(" "), regs = regs.join(" "), "pc trace before abort");
            }
        }
        self.cpu.cp15.ifsr = f.fsr | (f.domain << 4);
        self.cpu.cp15.ifar = pc;
        self.take_exception(ExceptionType::PrefetchAbort(pc));
        StepOutcome::Progress
    }

    fn heartbeat(&mut self) {
        if self.last_heartbeat.elapsed() >= std::time::Duration::from_secs(2) {
            debug!(
                insns = self.insn_count,
                pc = format!("{:#x}", self.cpu.regs[15]),
                lr = format!("{:#x}", self.cpu.regs[14]),
                cpsr = format!("{:#x}", self.cpu.cpsr.to_u32()),
                dfar = format!("{:#x}", self.cpu.cp15.dfar),
                dfsr = format!("{:#x}", self.cpu.cp15.dfsr),
                ifar = format!("{:#x}", self.cpu.cp15.ifar),
                ttbr0 = format!("{:#x}", self.cpu.cp15.ttbr0),
                mmu = self.bridge.inner.borrow().mmu.enabled,
                "armv6 emulator heartbeat"
            );
            {
                // Decode the most recent data fault VA against the live
                // tables (debug aid).
                let inner = self.bridge.inner.borrow();
                let va = self.cpu.cp15.dfar;
                let base = inner.mmu.ttbr0 & !0x3FFF;
                let l1a = base | (((va >> 20) & 0xFFF) << 2);
                let rd = |pa: u32| -> u32 {
                    let mut b = [0u8; 4];
                    let _ = self.bridge.mem.read_slice(&mut b, GuestAddress(pa as u64));
                    u32::from_le_bytes(b)
                };
                let l1 = rd(l1a);
                let l2a = (l1 & 0xFFFF_FC00) | (((va >> 12) & 0xFF) << 2);
                let (l2, linux_pte) = if l1 & 3 == 1 {
                    (rd(l2a), rd(l2a.wrapping_sub(2048)))
                } else {
                    (0, 0)
                };
                debug!(
                    va = format!("{va:#x}"),
                    l1 = format!("{l1:#010x}"),
                    l2 = format!("{l2:#010x}"),
                    dacr = format!("{:#x}", inner.mmu.dacr),
                    linux_pte = format!("{linux_pte:#010x}"),
                    access = inner.last_fault.access,
                    "fault VA table walk"
                );
            }
            self.last_heartbeat = std::time::Instant::now();
            // Debug aid: RAX_DUMP_PHYS=<hex-pa>:<hex-len>:<path> snapshots
            // guest physical memory each heartbeat (e.g. the printk buffer).
            if let Ok(spec) = std::env::var("RAX_DUMP_PHYS") {
                let parts: Vec<&str> = spec.split(':').collect();
                if parts.len() == 3 {
                    if let (Ok(pa), Ok(len)) = (
                        u64::from_str_radix(parts[0].trim_start_matches("0x"), 16),
                        usize::from_str_radix(parts[1].trim_start_matches("0x"), 16),
                    ) {
                        let mut buf = vec![0u8; len];
                        let mem = self.bridge.mem.clone();
                        if mem.read_slice(&mut buf, GuestAddress(pa)).is_ok() {
                            let _ = std::fs::write(parts[2], &buf);
                        }
                    }
                }
            }
        }
    }
}

enum StepOutcome {
    Progress,
    Idle,
}

impl VCpu for Armv6Vcpu {
    fn run(&mut self) -> Result<VcpuExit> {
        if self.shutdown {
            return Ok(VcpuExit::Shutdown);
        }
        for _ in 0..BATCH {
            match self.step() {
                StepOutcome::Progress => {}
                StepOutcome::Idle => {
                    self.heartbeat();
                    return Ok(VcpuExit::Hlt);
                }
            }
        }
        self.heartbeat();
        Ok(VcpuExit::Hlt)
    }

    fn get_state(&self) -> Result<CpuState> {
        let mut regs = Aarch32Registers::default();
        for i in 0..13 {
            regs.r[i] = self.cpu.regs[i];
        }
        regs.sp = self.cpu.regs[13];
        regs.lr = self.cpu.regs[14];
        regs.pc = self.cpu.regs[15];
        regs.cpsr = self.cpu.cpsr.to_u32();
        let sregs = Aarch32SystemRegisters::default();
        Ok(CpuState::Aarch32(Aarch32CpuState { regs, sregs }))
    }

    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        let state = match state {
            CpuState::Aarch32(s) => s,
            _ => {
                return Err(Error::Emulator(
                    "expected aarch32 state for armv6 vCPU".to_string(),
                ));
            }
        };
        // Apply CPSR first so banked registers land in the right mode.
        let cpsr = crate::arm::Psr::from_u32(state.regs.cpsr);
        if let Some(mode) = ProcessorMode::from_bits(cpsr.mode) {
            self.cpu.change_mode(mode);
        }
        self.cpu.cpsr = cpsr;
        for i in 0..13 {
            self.cpu.regs[i] = state.regs.r[i];
        }
        self.cpu.regs[13] = state.regs.sp;
        self.cpu.regs[14] = state.regs.lr;
        self.cpu.regs[15] = state.regs.pc;
        self.shutdown = false;
        Ok(())
    }

    fn complete_io_in(&mut self, _data: &[u8]) {}

    fn attach_s3c_uart(&mut self, uart: Arc<Mutex<S3cUart>>) {
        let _ = self.uart.set(uart);
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn instruction_count(&self) -> u64 {
        self.insn_count
    }
}
