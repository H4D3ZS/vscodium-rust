//! AArch64 vCPU: drives the software [`crate::arm::aarch64::AArch64Cpu`]
//! system emulator over guest memory and maps its exits onto [`VcpuExit`].
//!
//! Guest memory is bridged through [`GuestBridge`], an [`ArmMemory`] over the
//! VMM's [`GuestMemoryMmap`] with two device windows intercepted:
//!
//! - the GICv3 distributor/redistributor frames are routed into the CPU's
//!   shared [`Gic`] model (the same instance the ICC system registers act on),
//! - the PL011 window is serviced synchronously against the VMM's shared
//!   [`Pl011`] device, so console TX appears immediately and RX state is the
//!   one the VMM feeds from the host terminal. After every UART access the
//!   bridge re-mirrors the UART interrupt level onto GIC SPI 33.
//!
//! PSCI calls (the device tree advertises `method = "hvc"`) surface as
//! [`CpuExit::Hvc`]/[`CpuExit::Smc`] and are implemented here.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use tracing::{debug, info};
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use crate::arch::arm::{AARCH64_GICD_BASE, AARCH64_GICR_BASE, AARCH64_UART_IRQ};
use crate::arm::aarch64::{AArch64Config, AArch64Cpu, Gic};
use crate::arm::cpu_trait::{ArmCpu, CpuExit, ProcessorState};
use crate::arm::memory::{ArmMemory, MemResult, MemoryError, MmioHandler};
use crate::cpu::{CpuState, VCpu, VcpuExit};
use crate::devices::pl011::{Pl011, Pl011MmioDevice};
use crate::error::{Error, Result};
use crate::vphone_trace::{TraceEvent, known_device_for_gpa, record_runtime_event, runtime_trace_enabled};

/// GICv3 distributor frame size.
const GICD_SIZE: u64 = 0x1_0000;
/// GICv3 redistributor frame size (RD + SGI frames, one CPU).
const GICR_SIZE: u64 = 0x2_0000;
/// PL011 window.
const UART_BASE: u64 = crate::arch::arm::AARCH64_UART_BASE;
const UART_SIZE: u64 = 0x1000;

/// Instructions executed per `run()` call before yielding to the VMM loop
/// (console polling, checkpoint hotkeys). ~1-2ms of guest time.
const BATCH: u32 = 65_536;

/// Boot-debug physical watchpoint (RAX_WATCH=hex PA): the bridge flags a hit,
/// the run loop attributes it to the guest PC.
static WATCH_HIT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn watch_addr() -> Option<u64> {
    static ADDR: OnceLock<Option<u64>> = OnceLock::new();
    *ADDR.get_or_init(|| {
        std::env::var("RAX_WATCH")
            .ok()
            .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
    })
}

/// Boot-debug register tracer (RAX_TRACE_REG=reg:start:end, decimal): logs
/// every change of X<reg> within the instruction-count window.
fn trace_reg() -> Option<(u8, u64, u64)> {
    static CFG: OnceLock<Option<(u8, u64, u64)>> = OnceLock::new();
    *CFG.get_or_init(|| {
        let s = std::env::var("RAX_TRACE_REG").ok()?;
        let mut it = s.split(':');
        Some((
            it.next()?.parse().ok()?,
            it.next()?.parse().ok()?,
            it.next()?.parse().ok()?,
        ))
    })
}

/// Shared parts the bridge needs that only exist after construction.
#[derive(Default)]
struct LateBound {
    gic: OnceLock<Arc<Mutex<Gic>>>,
    uart: OnceLock<Arc<Mutex<Pl011>>>,
    trace_pc: AtomicU64,
}

/// Guest memory bridge: RAM via [`GuestMemoryMmap`], GIC and UART windows
/// intercepted.
struct GuestBridge {
    mem: Arc<GuestMemoryMmap>,
    late: Arc<LateBound>,
    /// Local exclusive monitor: (address, size) of the open reservation.
    exclusive: Mutex<Option<(u64, u8)>>,
}

impl std::fmt::Debug for GuestBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuestBridge").finish()
    }
}

#[inline]
fn in_window(addr: u64, base: u64, size: u64) -> bool {
    addr >= base && addr < base + size
}

impl GuestBridge {
    fn current_trace_pc(&self) -> u64 {
        self.late.trace_pc.load(Ordering::Relaxed)
    }

    fn record_read(&self, addr: u64, size: usize, value: u64, mapped: bool) {
        if !runtime_trace_enabled() {
            return;
        }
        record_runtime_event(&TraceEvent::MemRead {
            pc: self.current_trace_pc(),
            gpa: addr,
            size: size as u8,
            value,
            mapped,
            device: known_device_for_gpa(addr).map(str::to_string),
        });
    }

    fn record_write(&self, addr: u64, data: &[u8], mapped: bool) {
        if !runtime_trace_enabled() {
            return;
        }
        record_runtime_event(&TraceEvent::MemWrite {
            pc: self.current_trace_pc(),
            gpa: addr,
            size: data.len() as u8,
            value: le_value(data),
            mapped,
            device: known_device_for_gpa(addr).map(str::to_string),
        });
    }

    /// Recompute the PL011's level-triggered SPI after a UART access.
    fn sync_uart_irq(&self) {
        let (Some(gic), Some(uart)) = (self.late.gic.get(), self.late.uart.get()) else {
            return;
        };
        let level = uart.lock().map(|u| u.irq_pending()).unwrap_or(false);
        if let Ok(mut gic) = gic.lock() {
            if level {
                gic.set_pending(AARCH64_UART_IRQ);
            } else {
                gic.clear_pending(AARCH64_UART_IRQ);
            }
        }
    }

    fn gic_read(&self, addr: u64, buf: &mut [u8]) {
        let Some(gic) = self.late.gic.get() else {
            buf.fill(0);
            return;
        };
        let Ok(gic) = gic.lock() else {
            buf.fill(0);
            return;
        };
        // Serve arbitrary widths out of the 32-bit register file.
        for (i, b) in buf.iter_mut().enumerate() {
            let byte_addr = addr + i as u64;
            let reg_addr = byte_addr & !0x3;
            let lane = (byte_addr & 0x3) as usize;
            let value = if in_window(reg_addr, AARCH64_GICD_BASE, GICD_SIZE) {
                gic.read_dist(reg_addr - AARCH64_GICD_BASE)
            } else {
                gic.read_redist(0, reg_addr - AARCH64_GICR_BASE)
            };
            *b = value.to_le_bytes()[lane];
        }
    }

    fn gic_write(&self, addr: u64, data: &[u8]) {
        let Some(gic) = self.late.gic.get() else {
            return;
        };
        let Ok(mut gic) = gic.lock() else {
            return;
        };
        // Fold writes into whole 32-bit registers (read-modify-write for
        // partial widths).
        let mut offset = 0usize;
        while offset < data.len() {
            let byte_addr = addr + offset as u64;
            let reg_addr = byte_addr & !0x3;
            let lane = (byte_addr & 0x3) as usize;
            let take = (4 - lane).min(data.len() - offset);

            let dist = in_window(reg_addr, AARCH64_GICD_BASE, GICD_SIZE);
            let mut value = if dist {
                gic.read_dist(reg_addr - AARCH64_GICD_BASE)
            } else {
                gic.read_redist(0, reg_addr - AARCH64_GICR_BASE)
            };
            let mut bytes = value.to_le_bytes();
            bytes[lane..lane + take].copy_from_slice(&data[offset..offset + take]);
            value = u32::from_le_bytes(bytes);

            if dist {
                gic.write_dist(reg_addr - AARCH64_GICD_BASE, value);
            } else {
                gic.write_redist(0, reg_addr - AARCH64_GICR_BASE, value);
            }
            offset += take;
        }
    }
}

impl ArmMemory for GuestBridge {
    fn read(&self, addr: u64, buf: &mut [u8]) -> MemResult<()> {
        if in_window(addr, UART_BASE, UART_SIZE) {
            if let Some(uart) = self.late.uart.get() {
                let mut dev = Pl011MmioDevice::new(UART_BASE, uart.clone());
                crate::devices::bus::MmioDevice::read(&mut dev, addr, buf);
            } else {
                buf.fill(0);
            }
            self.sync_uart_irq();
            self.record_read(addr, buf.len(), le_value(buf), true);
            return Ok(());
        }
        if in_window(addr, AARCH64_GICD_BASE, GICD_SIZE)
            || in_window(addr, AARCH64_GICR_BASE, GICR_SIZE)
        {
            self.gic_read(addr, buf);
            self.record_read(addr, buf.len(), le_value(buf), true);
            return Ok(());
        }
        match self.mem.read_slice(buf, GuestAddress(addr)) {
            Ok(()) => {
                self.record_read(addr, buf.len(), le_value(buf), true);
                Ok(())
            }
            Err(_) => {
                self.record_read(addr, buf.len(), 0, false);
                Err(MemoryError::OutOfBounds {
                addr,
                size: buf.len(),
            })
            }
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) -> MemResult<()> {
        // Boot-debug physical watchpoint (set RAX_WATCH=hexaddr).
        if let Some(w) = watch_addr() {
            if addr < w + 8 && addr + data.len() as u64 > w {
                WATCH_HIT.store(true, Ordering::Relaxed);
                debug!(addr = format!("{addr:#x}"), ?data, "watch: store");
            }
        }
        if in_window(addr, UART_BASE, UART_SIZE) {
            if let Some(uart) = self.late.uart.get() {
                let mut dev = Pl011MmioDevice::new(UART_BASE, uart.clone());
                crate::devices::bus::MmioDevice::write(&mut dev, addr, data);
            }
            self.sync_uart_irq();
            self.record_write(addr, data, true);
            return Ok(());
        }
        if in_window(addr, AARCH64_GICD_BASE, GICD_SIZE)
            || in_window(addr, AARCH64_GICR_BASE, GICR_SIZE)
        {
            self.gic_write(addr, data);
            self.record_write(addr, data, true);
            return Ok(());
        }
        match self.mem.write_slice(data, GuestAddress(addr)) {
            Ok(()) => {
                self.record_write(addr, data, true);
                Ok(())
            }
            Err(_) => {
                self.record_write(addr, data, false);
                Err(MemoryError::OutOfBounds {
                addr,
                size: data.len(),
            })
            }
        }
    }

    fn mark_exclusive(&mut self, addr: u64, size: u8) {
        *self.exclusive.lock().unwrap() = Some((addr, size));
    }

    fn check_exclusive(&mut self, addr: u64, size: u8) -> bool {
        let mut excl = self.exclusive.lock().unwrap();
        match excl.take() {
            Some((a, s)) if a == addr && s == size => true,
            _ => false,
        }
    }

    fn clear_exclusive(&mut self) {
        *self.exclusive.lock().unwrap() = None;
    }

    fn requires_alignment(&self) -> bool {
        // AArch64 permits unaligned accesses to normal memory when SCTLR.A=0
        // (the kernel relies on this, e.g. unaligned u64 loads while scanning
        // the FDT). Alignment-checked instruction forms enforce their own
        // rules in the CPU.
        false
    }

    fn register_mmio(&mut self, _base: u64, _size: u64, _handler: Box<dyn MmioHandler>) {
        // Device windows are fixed for this machine.
    }

    fn unregister_mmio(&mut self, _base: u64) {}
}

fn le_value(data: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    let len = data.len().min(bytes.len());
    bytes[..len].copy_from_slice(&data[..len]);
    u64::from_le_bytes(bytes)
}

/// AArch64 vCPU backed by the software system emulator.
pub struct Aarch64Vcpu {
    id: u32,
    cpu: AArch64Cpu,
    late: Arc<LateBound>,
    /// Total instructions executed (also read for snapshot bookkeeping).
    insn_count: Arc<AtomicU64>,
    /// Last heartbeat log (wall clock).
    last_heartbeat: std::time::Instant,
    /// Last value seen by the register tracer.
    trace_reg_last: u64,
    shutdown: bool,
}

impl Aarch64Vcpu {
    pub fn new(id: u32, mem: Arc<GuestMemoryMmap>) -> Self {
        let late = Arc::new(LateBound::default());
        let bridge = GuestBridge {
            mem,
            late: late.clone(),
            exclusive: Mutex::new(None),
        };
        let cpu = AArch64Cpu::new(AArch64Config::default(), Box::new(bridge));
        // The CPU owns the GIC; hand the bridge a reference to the same
        // instance so MMIO and ICC system registers agree.
        if let Some(gic) = cpu.gic_handle() {
            let _ = late.gic.set(gic);
        }
        Aarch64Vcpu {
            id,
            cpu,
            late,
            insn_count: Arc::new(AtomicU64::new(0)),
            last_heartbeat: std::time::Instant::now(),
            trace_reg_last: 0,
            shutdown: false,
        }
    }

    /// Boot-debug heartbeat: where is the guest and how fast is it going?
    fn heartbeat(&mut self) {
        if self.last_heartbeat.elapsed() >= std::time::Duration::from_secs(2) {
            debug!(
                insns = self.cpu.instruction_count(),
                pc = format!("{:#x}", self.cpu.get_pc()),
                el = self.cpu.current_el(),
                lr = format!("{:#x}", self.cpu.get_lr()),
                x0 = format!("{:#x}", self.cpu.get_gpr(0)),
                x1 = format!("{:#x}", self.cpu.get_gpr(1)),
                x2 = format!("{:#x}", self.cpu.get_gpr(2)),
                x3 = format!("{:#x}", self.cpu.get_gpr(3)),
                "aarch64 emulator heartbeat"
            );
            self.last_heartbeat = std::time::Instant::now();
        }
    }

    /// Mirror the UART interrupt level into the GIC. Called at batch
    /// boundaries to pick up console input queued by the VMM.
    fn sync_uart_irq(&self) {
        let (Some(gic), Some(uart)) = (self.late.gic.get(), self.late.uart.get()) else {
            return;
        };
        let level = uart.lock().map(|u| u.irq_pending()).unwrap_or(false);
        if let Ok(mut gic) = gic.lock() {
            if level {
                gic.set_pending(AARCH64_UART_IRQ);
            } else {
                gic.clear_pending(AARCH64_UART_IRQ);
            }
        }
    }

    /// SMCCC/PSCI 1.1 over HVC/SMC. Returns Some(exit) when the call shuts
    /// the machine down.
    fn handle_psci(&mut self) -> Option<VcpuExit> {
        const PSCI_VERSION: u32 = 0x8400_0000;
        const CPU_ON_32: u32 = 0x8400_0003;
        const CPU_ON_64: u32 = 0xC400_0003;
        const MIGRATE_INFO_TYPE: u32 = 0x8400_0006;
        const SYSTEM_OFF: u32 = 0x8400_0008;
        const SYSTEM_RESET: u32 = 0x8400_0009;
        const PSCI_FEATURES: u32 = 0x8400_000A;

        const NOT_SUPPORTED: u64 = -1i64 as u64;
        const DENIED: u64 = -3i64 as u64;

        let func = self.cpu.get_gpr(0) as u32;
        let result = match func {
            PSCI_VERSION => 0x0001_0001, // PSCI 1.1
            MIGRATE_INFO_TYPE => 2,      // no trusted OS
            SYSTEM_OFF => {
                info!("PSCI SYSTEM_OFF: guest requested power-off");
                return Some(VcpuExit::Shutdown);
            }
            SYSTEM_RESET => {
                info!("PSCI SYSTEM_RESET: guest requested reset; shutting down");
                return Some(VcpuExit::Shutdown);
            }
            PSCI_FEATURES => {
                let queried = self.cpu.get_gpr(1) as u32;
                match queried {
                    PSCI_VERSION | SYSTEM_OFF | SYSTEM_RESET | PSCI_FEATURES
                    | MIGRATE_INFO_TYPE => 0,
                    _ => NOT_SUPPORTED,
                }
            }
            CPU_ON_32 | CPU_ON_64 => DENIED, // single-vCPU machine
            _ => {
                debug!(func = format!("{func:#x}"), "unhandled SMCCC/PSCI call");
                NOT_SUPPORTED
            }
        };
        self.cpu.set_gpr(0, result);
        None
    }
}

impl VCpu for Aarch64Vcpu {
    fn run(&mut self) -> Result<VcpuExit> {
        if self.shutdown {
            return Ok(VcpuExit::Shutdown);
        }

        // Pick up console input the VMM queued since the last batch.
        self.sync_uart_irq();

        for _ in 0..BATCH {
            let pc = self.cpu.get_pc();
            self.late.trace_pc.store(pc, Ordering::Relaxed);
            if runtime_trace_enabled() {
                record_runtime_event(&TraceEvent::Insn {
                    pc,
                    mnemonic: None,
                    regs_changed: None,
                });
            }
            match self.cpu.step_system() {
                Ok(CpuExit::Continue) => {
                    if let Some((reg, start, end)) = trace_reg() {
                        let n = self.cpu.instruction_count();
                        if n >= start && n <= end {
                            let v = self.cpu.get_gpr(reg);
                            if v != self.trace_reg_last {
                                let regs: Vec<String> = (0..31)
                                    .map(|i| format!("{:#x}", self.cpu.get_gpr(i)))
                                    .collect();
                                debug!(
                                    insns = n,
                                    pc = format!("{:#x}", self.cpu.get_pc()),
                                    value = format!("{v:#x}"),
                                    regs = regs.join(","),
                                    "trace: register changed"
                                );
                                self.trace_reg_last = v;
                            }
                        }
                    }
                    if watch_addr().is_some() && WATCH_HIT.swap(false, Ordering::Relaxed) {
                        let trail: Vec<String> = self
                            .cpu
                            .recent_pcs()
                            .iter()
                            .map(|p| format!("{p:#x}"))
                            .collect();
                        let regs: Vec<String> =
                            (0..31).map(|i| format!("{:#x}", self.cpu.get_gpr(i))).collect();
                        debug!(
                            pc = format!("{:#x}", self.cpu.get_pc()),
                            insns = self.cpu.instruction_count(),
                            trail = trail.join(","),
                            regs = regs.join(","),
                            "watch: store attributed (pc is the NEXT instruction)"
                        );
                    }
                }
                Ok(CpuExit::Wfi) | Ok(CpuExit::Wfe) => {
                    // Idle: give the VMM loop a turn (console, checkpoints).
                    self.insn_count
                        .store(self.cpu.instruction_count(), Ordering::Relaxed);
                    self.heartbeat();
                    return Ok(VcpuExit::Hlt);
                }
                Ok(CpuExit::Hvc(_)) | Ok(CpuExit::Smc(_)) => {
                    if let Some(exit) = self.handle_psci() {
                        self.shutdown = true;
                        return Ok(exit);
                    }
                }
                Ok(CpuExit::Halt) | Ok(CpuExit::Shutdown) => {
                    self.shutdown = true;
                    return Ok(VcpuExit::Shutdown);
                }
                Ok(other) => {
                    return Ok(VcpuExit::Unknown(format!(
                        "aarch64 exit {:?} at pc={:#x}",
                        other,
                        self.cpu.get_pc()
                    )));
                }
                Err(e) => {
                    return Ok(VcpuExit::Unknown(format!(
                        "aarch64 emulation error at pc={:#x}: {e}",
                        self.cpu.get_pc()
                    )));
                }
            }
        }
        self.insn_count
            .store(self.cpu.instruction_count(), Ordering::Relaxed);
        self.heartbeat();
        Ok(VcpuExit::Hlt)
    }

    fn get_state(&self) -> Result<CpuState> {
        use crate::cpu::{Aarch64Registers, Aarch64SystemRegisters};
        let mut regs = Aarch64Registers::default();
        for i in 0..31 {
            regs.x[i] = self.cpu.get_gpr(i as u8);
        }
        regs.pc = self.cpu.get_pc();
        regs.sp = self.cpu.get_sp();
        regs.pstate = self.cpu.get_pstate().to_pstate();
        let sregs = Aarch64SystemRegisters::default();
        Ok(CpuState::aarch64(regs, sregs))
    }

    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        let state = match state {
            CpuState::Aarch64(s) => s,
            _ => {
                return Err(Error::Emulator(
                    "expected aarch64 state for aarch64 vCPU".to_string(),
                ));
            }
        };
        self.cpu.reset();
        self.cpu
            .set_pstate(ProcessorState::from_pstate(state.regs.pstate));
        for i in 0..31 {
            self.cpu.set_gpr(i as u8, state.regs.x[i]);
        }
        self.cpu.set_pc(state.regs.pc);
        self.cpu.set_sp(state.regs.sp);
        self.shutdown = false;
        Ok(())
    }

    fn complete_io_in(&mut self, _data: &[u8]) {
        // Device reads are serviced synchronously inside the bridge.
    }

    fn attach_pl011(&mut self, _base: u64, uart: Arc<Mutex<Pl011>>) {
        let _ = self.late.uart.set(uart);
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn instruction_count(&self) -> u64 {
        self.insn_count.load(Ordering::Relaxed)
    }
}
