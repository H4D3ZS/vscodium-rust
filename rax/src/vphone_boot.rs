//! Dynamic iOS/XNU boot harness for the vphone600ap kernelcache.
//!
//! Where [`crate::vphone_trace`] is a *static* analyzer (it parses the Mach-O and
//! ranks PCs from a progress log, but never runs a single instruction), this
//! module actually *executes* the kernelcache on rax's oracle-verified AArch64
//! interpreter. It maps the Mach-O segments at their slid addresses, seeds the
//! EL1 boot state the XNU entry expects (PC, SP, X0=boot_args, PSTATE=EL1h,
//! SCTLR/VBAR/TPIDR), and single-steps the real CPU, recording every access to
//! the Apple device windows.
//!
//! The deliverable is the thing the C++ emulator's incomplete JIT can't give:
//! a faithful instruction stream that says *which* device poll the kernel spins
//! on (populating `last_mmio` / device), instead of silently NOPing the
//! instruction it doesn't know and looping forever.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::arm::aarch64::{AArch64Config, AArch64Cpu};
use crate::arm::cpu_trait::{ArmCpu, CpuExit, ProcessorState};
use crate::arm::memory::{ArmMemory, ExclusiveMonitor, MemResult, MemoryError, MmioHandler};
use crate::error::{Error, Result};
use crate::vphone_trace::{
    PcFinding, TraceEvent, TraceFindings, VphoneImage, known_device_for_gpa, load_vphone_image,
    record_runtime_event, runtime_trace_enabled,
};

/// Verbose boot diagnostics, compiled in only with `--features vphone-diag`.
/// Release builds emit nothing, so internal boot addresses / seed logic are not
/// printed (and the format strings are not embedded in the shipped binary).
#[cfg(feature = "vphone-diag")]
macro_rules! diag {
    ($($a:tt)*) => { eprintln!($($a)*) };
}
#[cfg(not(feature = "vphone-diag"))]
macro_rules! diag {
    ($($a:tt)*) => {{}};
}
pub(crate) use diag;

/// Result of a dynamic boot run.
pub struct BootRun {
    pub findings: TraceFindings,
    pub instructions: u64,
    pub stop_reason: String,
    pub last_device: Option<(u64, String)>,
    /// Final architectural register snapshot, for post-mortem diagnosis.
    pub regs: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub el: u8,
    pub sctlr_el1: u64,
}

/// Guest physical memory backed by a flat RAM window with the Apple device
/// windows (all above `0x2_0000_0000`, i.e. above RAM) intercepted and recorded.
///
/// A device read returns 0 and a write is dropped — the same "quiet device"
/// behavior the C++ stub has, so a kernel poll spins exactly as it does on the
/// real emulator, but here every access is attributed to the executing PC.
struct TracingMem {
    base: u64,
    data: Vec<u8>,
    exclusive: ExclusiveMonitor,
    /// PC of the instruction currently executing, for access attribution.
    cur_pc: Arc<AtomicU64>,
    /// Per-device-GPA access counts (shared with the run loop).
    dev_counts: Arc<std::sync::Mutex<HashMap<u64, u64>>>,
    /// Most recent device GPA touched (shared with the run loop).
    last_dev: Arc<AtomicU64>,
    /// Whether the last_dev slot holds a valid address.
    last_dev_valid: Arc<std::sync::atomic::AtomicBool>,
    /// Free-running platform-timer counter (Apple timer window reads).
    timer: Arc<AtomicU64>,
    /// Optional RAM write watchpoint [watch, watch+8): records (pc, addr, value).
    watch: Option<u64>,
    watch_hits: Arc<std::sync::Mutex<Vec<(u64, u64, u64)>>>,
}

impl std::fmt::Debug for TracingMem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TracingMem")
            .field("base", &self.base)
            .field("size", &self.data.len())
            .finish()
    }
}

impl TracingMem {
    fn new(base: u64, size: usize, cur_pc: Arc<AtomicU64>, watch: Option<u64>) -> Self {
        Self {
            base,
            data: vec![0u8; size],
            exclusive: ExclusiveMonitor::new(),
            cur_pc,
            dev_counts: Arc::new(std::sync::Mutex::new(HashMap::new())),
            last_dev: Arc::new(AtomicU64::new(0)),
            last_dev_valid: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            timer: Arc::new(AtomicU64::new(0)),
            watch,
            watch_hits: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    #[inline]
    fn in_ram(&self, addr: u64, len: usize) -> bool {
        addr >= self.base
            && (addr - self.base)
                .checked_add(len as u64)
                .map(|end| end <= self.data.len() as u64)
                .unwrap_or(false)
    }

    /// Load image bytes directly into RAM without recording (setup phase).
    fn load_bytes(&mut self, addr: u64, bytes: &[u8]) -> Result<()> {
        if !self.in_ram(addr, bytes.len()) {
            return Err(Error::KernelLoad(format!(
                "load at {addr:#x} ({} bytes) is outside the {} MiB RAM window at {:#x}",
                bytes.len(),
                self.data.len() / (1024 * 1024),
                self.base
            )));
        }
        let off = (addr - self.base) as usize;
        self.data[off..off + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    fn note_device(&self, gpa: u64, size: usize, value: u64, write: bool) {
        let device = known_device_for_gpa(gpa).map(str::to_string);
        // Track the hottest device window and the most recent one.
        if device.is_some() {
            if let Ok(mut counts) = self.dev_counts.lock() {
                *counts.entry(gpa).or_insert(0) += 1;
            }
            self.last_dev.store(gpa, Ordering::Relaxed);
            self.last_dev_valid.store(true, Ordering::Relaxed);
        }
        if runtime_trace_enabled() {
            let pc = self.cur_pc.load(Ordering::Relaxed);
            let event = if write {
                TraceEvent::MemWrite {
                    pc,
                    gpa,
                    size: size as u8,
                    value,
                    mapped: false,
                    device,
                }
            } else {
                TraceEvent::MemRead {
                    pc,
                    gpa,
                    size: size as u8,
                    value,
                    mapped: false,
                    device,
                }
            };
            record_runtime_event(&event);
        }
    }
}

// Apple SoC device window bases (from the C++ DeviceEmulator map + vphone_trace).
const APPLE_UART_BASE: u64 = 0x2352_00000;
const APPLE_UART_SIZE: u64 = 0x4000;
const APPLE_AIC_BASE: u64 = 0x28E1_00000;
const APPLE_AIC_SIZE: u64 = 0x10000;
const APPLE_TIMER_BASE: u64 = 0x23E1_00000;
const APPLE_TIMER_SIZE: u64 = 0x1000;

impl TracingMem {
    /// Service a read to a device window. PL011-compatible UART (the layout the
    /// C++ DeviceTree advertises), AIC (no pending IRQ), and a free-running
    /// timer counter. Everything else reads as 0.
    fn device_read(&self, addr: u64, size: usize) -> u64 {
        if (APPLE_UART_BASE..APPLE_UART_BASE + APPLE_UART_SIZE).contains(&addr) {
            let off = addr - APPLE_UART_BASE;
            return match off {
                // PL011 UARTFR (flags): TXFE set (0x80), TXFF/RXFF clear, RXFE set
                // (0x10) — i.e. "ready to transmit, nothing to receive".
                0x18 => 0x90,
                _ => 0,
            };
        }
        if (APPLE_AIC_BASE..APPLE_AIC_BASE + APPLE_AIC_SIZE).contains(&addr) {
            // No interrupt is pending in M1 (delivery comes later); WHOAMI/IACK
            // and event registers all read 0 so the kernel's IRQ probe passes.
            return 0;
        }
        if (APPLE_TIMER_BASE..APPLE_TIMER_BASE + APPLE_TIMER_SIZE).contains(&addr) {
            // Free-running counter so a kernel busy-wait on the platform timer
            // makes progress instead of stalling.
            let _ = size;
            return self.timer.fetch_add(1, Ordering::Relaxed);
        }
        0
    }

    /// Service a write to a device window. A UART data-register write is the
    /// Darwin serial console — emit it to stdout immediately.
    fn device_write(&self, addr: u64, data: &[u8]) {
        if (APPLE_UART_BASE..APPLE_UART_BASE + APPLE_UART_SIZE).contains(&addr) {
            let off = addr - APPLE_UART_BASE;
            // PL011 UARTDR is offset 0x00; the low byte is the character.
            if off == 0x00 {
                if let Some(&b) = data.first() {
                    use std::io::Write as _;
                    let mut out = std::io::stdout().lock();
                    let _ = out.write_all(&[b]);
                    let _ = out.flush();
                }
            }
        }
    }
}

impl ArmMemory for TracingMem {
    fn read(&self, addr: u64, buf: &mut [u8]) -> MemResult<()> {
        if self.in_ram(addr, buf.len()) {
            let off = (addr - self.base) as usize;
            buf.copy_from_slice(&self.data[off..off + buf.len()]);
            return Ok(());
        }
        // Out-of-RAM read: a device window or unmapped MMIO.
        let value = self.device_read(addr, buf.len());
        self.note_device(addr, buf.len(), value, false);
        let bytes = value.to_le_bytes();
        for (i, b) in buf.iter_mut().enumerate() {
            *b = if i < 8 { bytes[i] } else { 0 };
        }
        Ok(())
    }

    fn write(&mut self, addr: u64, data: &[u8]) -> MemResult<()> {
        if self.in_ram(addr, data.len()) {
            let off = (addr - self.base) as usize;
            self.data[off..off + data.len()].copy_from_slice(data);
            if let Some(w) = self.watch {
                if addr < w + 8 && addr + data.len() as u64 > w {
                    let mut v = 0u64;
                    for (i, &b) in data.iter().take(8).enumerate() {
                        v |= (b as u64) << (i * 8);
                    }
                    if let Ok(mut hits) = self.watch_hits.lock() {
                        if hits.len() < 256 {
                            hits.push((self.cur_pc.load(Ordering::Relaxed), addr, v));
                        }
                    }
                }
            }
            return Ok(());
        }
        let value = {
            let mut v = 0u64;
            for (i, &b) in data.iter().take(8).enumerate() {
                v |= (b as u64) << (i * 8);
            }
            v
        };
        self.device_write(addr, data);
        self.note_device(addr, data.len(), value, true);
        Ok(())
    }

    fn mark_exclusive(&mut self, addr: u64, size: u8) {
        self.exclusive.mark_exclusive(addr, size, 0);
    }
    fn check_exclusive(&mut self, addr: u64, size: u8) -> bool {
        self.exclusive.check_and_clear(addr, size, 0)
    }
    fn clear_exclusive(&mut self) {
        self.exclusive.clear();
    }
    fn requires_alignment(&self) -> bool {
        false
    }
    fn is_big_endian(&self) -> bool {
        false
    }
    fn register_mmio(&mut self, _base: u64, _size: u64, _handler: Box<dyn MmioHandler>) {}
    fn unregister_mmio(&mut self, _base: u64) {}
}

/// Map the Mach-O kernelcache segments into `mem` at `vmaddr + slide`, returning
/// nothing on success. Reuses the segment table already parsed into the image.
fn map_kernel(mem: &mut TracingMem, image: &VphoneImage, kernel_path: &Path) -> Result<()> {
    let mut file_bytes = Vec::new();
    File::open(kernel_path)?.read_to_end(&mut file_bytes)?;
    let slide = image.kernel.slide;
    for seg in &image.kernel.segments {
        if seg.filesize == 0 {
            continue;
        }
        let src_start = seg.fileoff as usize;
        let src_end = src_start
            .checked_add(seg.filesize as usize)
            .ok_or_else(|| Error::KernelLoad("segment fileoff overflow".to_string()))?;
        if src_end > file_bytes.len() {
            return Err(Error::KernelLoad(format!(
                "segment {} file range {src_start:#x}..{src_end:#x} exceeds kernel size",
                seg.name
            )));
        }
        let dest = seg.vmaddr.wrapping_add(slide);
        mem.load_bytes(dest, &file_bytes[src_start..src_end])?;
    }
    Ok(())
}

/// Parse the kernel's true entry PC from `LC_UNIXTHREAD` (ARM_THREAD_STATE64),
/// as a *virtual* (pre-slide) address. The static path hardcodes a layout
/// constant; the Mach-O is authoritative. Returns None if not present.
fn parse_unixthread_entry_va(bytes: &[u8]) -> Option<u64> {
    const MH_MAGIC_64: u32 = 0xFEED_FACF;
    const LC_UNIXTHREAD: u32 = 0x5;
    let rd32 = |o: usize| -> Option<u32> {
        bytes
            .get(o..o + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    let rd64 = |o: usize| -> Option<u64> {
        bytes.get(o..o + 8).map(|b| {
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        })
    };
    if rd32(0)? != MH_MAGIC_64 {
        return None;
    }
    let ncmds = rd32(16)?;
    let mut off = 32usize;
    for _ in 0..ncmds {
        let cmd = rd32(off)?;
        let cmdsize = rd32(off + 4)? as usize;
        if cmdsize < 8 {
            return None;
        }
        if cmd == LC_UNIXTHREAD {
            // cmd(4) cmdsize(4) flavor(4) count(4) then ARM_THREAD_STATE64:
            // x[29] (232) + fp(8) + lr(8) + sp(8) + pc(8). pc at +272 from cmd.
            return rd64(off + 272);
        }
        off = off.checked_add(cmdsize)?;
    }
    None
}

/// Configuration for a dynamic boot run.
pub struct BootConfig {
    pub kernel: std::path::PathBuf,
    pub dtb: std::path::PathBuf,
    pub ram_mib: usize,
    pub max_insns: u64,
    /// Break after this many consecutive WFI/WFE with no interrupt source.
    pub wfi_budget: u64,
    /// Apply the iBoot-equivalent boot-CPU seed (CpuDataEntries[0]).
    pub seed: bool,
    /// Optional RAM write watchpoint (8 bytes) for diagnosis.
    pub watch: Option<u64>,
    /// Optional PC to stop at and dump X0/X3 as C-strings (e.g. a panic entry).
    pub break_pc: Option<u64>,
}

// iBoot-equivalent boot seeds, specific to this kernelcache (iOS 26.1
// vphone600ap), in the same spirit as the C++ Styx digest-specific seeds.
//
// The reset vector in __TEXT_BOOT_EXEC reads MPIDR_EL1 and searches the
// `CpuDataEntries` table for the entry whose cpu id (at +0x1C8) matches. That
// table is zeroed in the kernelcache because iBoot normally fills it before
// entering the kernel. Seed entry[0] with a pointer to a zeroed cpu_data
// scratch page so the boot CPU (id 0) is found and the search loop exits.
const CPU_DATA_ENTRIES: u64 = 0x837e_cee0;
const CPU_DATA_SCRATCH: u64 = 0x8090_0000;

/// Execute the kernelcache on the AArch64 interpreter and return findings.
pub fn boot_and_trace(cfg: &BootConfig) -> Result<BootRun> {
    let image = load_vphone_image(&cfg.kernel, &cfg.dtb)?;
    let mut layout = image.layout.clone();

    // Prefer the Mach-O's own entry (LC_UNIXTHREAD) over the hardcoded layout
    // constant: starting one instruction off lands mid-function with garbage
    // registers. The thread-state PC is a virtual address; slide it.
    {
        let mut kbytes = Vec::new();
        File::open(&cfg.kernel)?.read_to_end(&mut kbytes)?;
        if let Some(entry_va) = parse_unixthread_entry_va(&kbytes) {
            let real_entry = entry_va.wrapping_add(image.kernel.slide);
            if real_entry != layout.kernel_entry {
                diag!(
                    "[vphone-boot] LC_UNIXTHREAD entry {real_entry:#x} (va {entry_va:#x}) overrides layout entry {:#x}",
                    layout.kernel_entry
                );
            } else {
                diag!("[vphone-boot] LC_UNIXTHREAD entry {real_entry:#x} matches layout entry");
            }
            layout.kernel_entry = real_entry;
        } else {
            diag!(
                "[vphone-boot] no LC_UNIXTHREAD; using layout entry {:#x}",
                layout.kernel_entry
            );
        }
    }

    let cur_pc = Arc::new(AtomicU64::new(layout.kernel_entry));
    let mut mem = TracingMem::new(
        layout.ram_base,
        cfg.ram_mib * 1024 * 1024,
        Arc::clone(&cur_pc),
        cfg.watch,
    );

    // Lay down the kernel image, boot_args, and DTB.
    map_kernel(&mut mem, &image, &cfg.kernel)?;
    mem.load_bytes(layout.boot_args_base, &image.boot_args)?;
    let mut dtb_bytes = Vec::new();
    File::open(&cfg.dtb)?.read_to_end(&mut dtb_bytes)?;
    mem.load_bytes(layout.dtb_base, &dtb_bytes)?;

    if cfg.seed {
        // Each 0x10-byte CpuDataEntries slot holds its cpu_data pointer at +8
        // (the trampoline does `LDR Xn,[Xtable,#8]`). Point entry[0].cpu_data at
        // the zeroed scratch page; its cpu id field (at +0x1C8) is 0 == MPIDR&0xff
        // for the boot CPU, so the trampoline's search matches and exits.
        let slot = CPU_DATA_ENTRIES + 8;
        mem.load_bytes(slot, &CPU_DATA_SCRATCH.to_le_bytes())?;

        // Minimal cpu_data struct fields the boot trampoline consumes right after
        // finding its entry (iBoot would have set these up):
        //   +0x18, +0x28 → per-CPU stack tops (loaded into SP)
        //   +0xB8        → a pointer that must be non-null (CBZ → panic otherwise)
        // Stacks live in dedicated zeroed RAM pages below boot_args.
        const BOOT_STACK_TOP: u64 = 0x8068_0000; // grows down, below SP_BASE
        const INTR_STACK_TOP: u64 = 0x8060_0000;
        // +0xB8 is validated against one of two known kernel pointers
        // (ADRP@0x835c0118+0x834 and ADRP@0x835c0128+0xA04, page 0x81C48000):
        // 0x81C48834 / 0x81C48A04. Use the first so the check passes.
        const CPU_DATA_B8: u64 = 0x81C4_8834;
        mem.load_bytes(CPU_DATA_SCRATCH + 0x18, &INTR_STACK_TOP.to_le_bytes())?;
        mem.load_bytes(CPU_DATA_SCRATCH + 0x28, &BOOT_STACK_TOP.to_le_bytes())?;
        mem.load_bytes(CPU_DATA_SCRATCH + 0xB8, &CPU_DATA_B8.to_le_bytes())?;
        diag!(
            "[vphone-boot] seeded CpuDataEntries[0].cpu_data={CPU_DATA_SCRATCH:#x} at {slot:#x} (stacks + aux)"
        );
    }

    #[cfg(feature = "vphone-diag")]
    {
        let mut chk = [0u8; 8];
        let _ = ArmMemory::read(&mem, CPU_DATA_ENTRIES, &mut chk);
        diag!(
            "[vphone-boot] readback @{CPU_DATA_ENTRIES:#x} = {:#x}",
            u64::from_le_bytes(chk)
        );
    }

    // Share the device-tracking handles before the memory is boxed into the CPU.
    let dev_counts = Arc::clone(&mem.dev_counts);
    let last_dev = Arc::clone(&mem.last_dev);
    let last_dev_valid = Arc::clone(&mem.last_dev_valid);
    let watch_hits = Arc::clone(&mem.watch_hits);

    // XNU touches many Apple IMPLEMENTATION DEFINED system registers no public
    // reference models; treat unmodeled regs as RAZ/WI so boot proceeds instead
    // of aborting on the first one. (Strict behavior stays the default elsewhere.)
    crate::arm::aarch64::cpu::set_lenient_sysregs(true);

    // Build the CPU as ARMv9.0-A (closest to the A18) and seed XNU EL1 state.
    let config = AArch64Config::v9_0();
    let mut cpu = AArch64Cpu::new(config, Box::new(mem));
    cpu.set_pstate(ProcessorState::from_pstate(layout.pstate));
    cpu.set_sp(layout.sp);
    cpu.set_pc(layout.kernel_entry);
    cpu.set_gpr(0, layout.boot_args_base);
    {
        let bank = cpu.sysregs_mut().bank_mut(1);
        bank.sctlr = layout.sctlr_el1;
        bank.vbar = layout.vbar_el1;
        bank.tpidr = layout.tpidr_el1;
    }
    // Apple silicon runs the architected timer at 24 MHz; iBoot programs CNTFRQ
    // to match. XNU's _enable_timebase_event_stream derives the event-stream
    // divider bit from CNTFRQ and panics ("invalid bit index") if it isn't the
    // expected power-of-two-friendly 24 MHz. rax defaults to 62.5 MHz, so set it.
    cpu.sysregs_mut().cntfrq_el0 = 24_000_000;

    // Emit the boot layout up front so a JSONL consumer has the same metadata
    // header the static path produced.
    if runtime_trace_enabled() {
        record_runtime_event(&TraceEvent::Metadata {
            layout: layout.clone(),
            kernel: image.kernel.clone(),
            dtb: image.dtb.clone(),
        });
    }

    // Step the real interpreter, building a PC histogram.
    let mut pc_counts: HashMap<u64, u64> = HashMap::new();
    let mut executed: u64 = 0;
    let mut wfi_run: u64 = 0;
    let mut stop_reason = format!("reached max_insns={}", cfg.max_insns);

    while executed < cfg.max_insns {
        let pc = cpu.get_pc();
        if Some(pc) == cfg.break_pc {
            // Read C-strings at X0 and X3 (common format-arg registers).
            let read_cstr = |cpu: &AArch64Cpu, addr: u64| -> String {
                let mut s = String::new();
                for i in 0..160u64 {
                    match cpu.read_memory(addr + i, 1) {
                        Ok(b) if b[0] != 0 => s.push(b[0] as char),
                        _ => break,
                    }
                }
                s
            };
            let x0 = cpu.get_gpr(0);
            let x1 = cpu.get_gpr(1);
            let x3 = cpu.get_gpr(3);
            diag!("[vphone-boot] break {pc:#x}: X0={x0:#x} \"{}\"", read_cstr(&cpu, x0));
            diag!("[vphone-boot] break {pc:#x}: X1={x1:#x} \"{}\"", read_cstr(&cpu, x1));
            diag!("[vphone-boot] break {pc:#x}: X2={:#x} X4={:#x}", cpu.get_gpr(2), cpu.get_gpr(4));
            diag!("[vphone-boot] break {pc:#x}: X3={x3:#x} \"{}\"", read_cstr(&cpu, x3));
            diag!("[vphone-boot] break {pc:#x}: X30(lr)={:#x}", cpu.get_gpr(30));
            stop_reason = format!("hit break pc {pc:#x}");
            executed += 1;
            break;
        }
        cur_pc.store(pc, Ordering::Relaxed);
        *pc_counts.entry(pc).or_insert(0) += 1;

        // step_system (not step): full OS delivery — timer ticks, WFI wake on
        // pending interrupt, IRQ vectoring, SVC/page-fault → exception vectors.
        match cpu.step_system() {
            Ok(CpuExit::Continue) => {
                wfi_run = 0;
            }
            Ok(CpuExit::Wfi) | Ok(CpuExit::Wfe) | Ok(CpuExit::Halt) => {
                wfi_run += 1;
                if wfi_run >= cfg.wfi_budget {
                    stop_reason = format!(
                        "CPU parked in WFI/WFE/HLT at pc={pc:#x} (no interrupt source; need M1 AIC/timer)"
                    );
                    executed += 1;
                    break;
                }
            }
            Ok(CpuExit::Undefined(insn)) => {
                stop_reason =
                    format!("undefined instruction {insn:#010x} at pc={pc:#x} (rax decode gap)");
                executed += 1;
                break;
            }
            Ok(CpuExit::Svc(n)) => {
                stop_reason = format!("SVC #{n} at pc={pc:#x} (userspace transition reached)");
                executed += 1;
                break;
            }
            Ok(CpuExit::Hvc(n)) => {
                stop_reason = format!("HVC #{n} at pc={pc:#x}");
                executed += 1;
                break;
            }
            Ok(CpuExit::Smc(n)) => {
                stop_reason = format!("SMC #{n} at pc={pc:#x} (secure monitor call; need M2)");
                executed += 1;
                break;
            }
            Ok(CpuExit::MemoryFault(info)) => {
                stop_reason = format!(
                    "memory fault {:?} at {:#x} (pc={pc:#x})",
                    info.fault_type, info.address
                );
                executed += 1;
                break;
            }
            Ok(other) => {
                // ExceptionTaken / InterruptPending / etc. — keep going; the CPU
                // has updated its own PC to the vector or handler.
                let _ = other;
                wfi_run = 0;
            }
            Err(e) => {
                stop_reason = format!("interpreter error at pc={pc:#x}: {e}");
                executed += 1;
                break;
            }
        }
        executed += 1;
    }

    // Rank the hottest PCs and attach the hottest device window as the likely
    // poll target (the spin loop's last device read).
    let mut ranked: Vec<(u64, u64)> = pc_counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));

    let hottest_device: Option<(u64, String)> = {
        let counts = dev_counts.lock().ok();
        counts.and_then(|c| {
            c.iter()
                .max_by_key(|(_, n)| **n)
                .and_then(|(gpa, _)| known_device_for_gpa(*gpa).map(|d| (*gpa, d.to_string())))
        })
    };
    let last_device: Option<(u64, String)> = if last_dev_valid.load(Ordering::Relaxed) {
        let gpa = last_dev.load(Ordering::Relaxed);
        known_device_for_gpa(gpa).map(|d| (gpa, d.to_string()))
    } else {
        None
    };

    let top_spinning_pcs: Vec<PcFinding> = ranked
        .iter()
        .take(24)
        .map(|(pc, count)| PcFinding {
            pc: *pc,
            count: *count,
            likely_area: "executed under rax AArch64 interpreter".to_string(),
            last_mmio: hottest_device.as_ref().map(|(gpa, _)| *gpa),
            last_mmio_device: hottest_device.as_ref().map(|(_, d)| d.clone()),
            last_sysreg: None,
            recommendation: match &hottest_device {
                Some((gpa, dev)) => format!(
                    "Implement the {dev} window at {gpa:#x} (M1) so this poll completes."
                ),
                None => "No device window touched yet; investigate the stop reason.".to_string(),
            },
        })
        .collect();

    let focus = match &hottest_device {
        Some((gpa, dev)) => vec![
            format!("Hottest device poll: {dev} at {gpa:#x}"),
            "This is real execution (not the static progress-log histogram).".to_string(),
        ],
        None => vec![
            "No Apple device window was read before the run stopped.".to_string(),
            "This is real execution (not the static progress-log histogram).".to_string(),
        ],
    };

    let findings = TraceFindings {
        target: "iPhone17,3/vphone600ap iOS 26.1 23B85".to_string(),
        progress_log: None,
        total_pc_samples: executed,
        top_spinning_pcs,
        likely_device_focus: focus,
        next_action: format!(
            "Stop reason: {stop_reason}. Add the indicated M1 device or close the decode gap, then re-run."
        ),
    };

    if cfg.watch.is_some() {
        if let Ok(hits) = watch_hits.lock() {
            diag!("[vphone-boot] watchpoint hits: {}", hits.len());
            for (pc, addr, val) in hits.iter().take(16) {
                diag!("[vphone-boot]   write {val:#x} -> {addr:#x} from pc={pc:#x}");
            }
        }
    }

    let mut regs = [0u64; 31];
    for (i, r) in regs.iter_mut().enumerate() {
        *r = cpu.get_gpr(i as u8);
    }

    Ok(BootRun {
        findings,
        instructions: executed,
        stop_reason,
        last_device,
        regs,
        sp: cpu.get_sp(),
        pc: cpu.get_pc(),
        el: cpu.current_el(),
        sctlr_el1: cpu.sysregs().bank(1).sctlr,
    })
}
