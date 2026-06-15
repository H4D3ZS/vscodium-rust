//! RISC-V vCPU: drives the [`crate::riscv::RiscVCpu`] interpreter over guest
//! memory and maps its exits onto [`VcpuExit`].
//!
//! Guest memory is bridged through [`GuestBridge`], a [`Memory`] implementation
//! over the VMM's [`GuestMemoryMmap`]. Accesses to the 16550 UART window are
//! intercepted: status-register reads are serviced synchronously (the
//! transmitter is always ready), and THR writes are buffered into a shared sink
//! that the run loop drains into a [`VcpuExit::MmioWrite`] so the VMM's serial
//! device produces console output. (The library interpreter executes one whole
//! instruction per step and cannot suspend mid-instruction, so MMIO *writes* are
//! surfaced after the step rather than during it.)

use std::sync::{Arc, Mutex};

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use crate::cpu::{CpuState, RiscVRegisters, VCpu, VcpuExit};
use crate::error::{Error, Result};
use crate::riscv::{MemError, MemResult, Memory, RiscVConfig, RiscVCpu, RiscVExit};

/// 16550 UART MMIO base/length (matches `Riscv64Arch::serial_mmio_base`).
const UART_BASE: u64 = 0x1000_0000;
const UART_LEN: u64 = 8;
/// 16550 Line Status Register offset and the "ready to transmit" bits.
const LSR_OFFSET: u64 = 5;
const LSR_THRE_TEMT: u8 = 0x60;
/// Bound on instructions executed per `run()` call (keeps the loop responsive).
const MAX_ITERS: u64 = 2_000_000;

/// A pending MMIO write surfaced to the run loop.
type MmioSink = Arc<Mutex<Option<(u64, Vec<u8>)>>>;

/// Guest memory bridge: RAM via [`GuestMemoryMmap`], with the UART window
/// intercepted.
struct GuestBridge {
    mem: Arc<GuestMemoryMmap>,
    pending: MmioSink,
}

impl std::fmt::Debug for GuestBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuestBridge").finish()
    }
}

#[inline]
fn in_uart(addr: u64) -> bool {
    addr >= UART_BASE && addr < UART_BASE + UART_LEN
}

impl Memory for GuestBridge {
    fn read(&self, addr: u64, buf: &mut [u8]) -> MemResult<()> {
        if in_uart(addr) {
            for (i, b) in buf.iter_mut().enumerate() {
                let off = (addr + i as u64) - UART_BASE;
                *b = if off == LSR_OFFSET { LSR_THRE_TEMT } else { 0 };
            }
            return Ok(());
        }
        self.mem
            .read_slice(buf, GuestAddress(addr))
            .map_err(|_| MemError::OutOfBounds {
                addr,
                size: buf.len(),
            })
    }

    fn write(&mut self, addr: u64, data: &[u8]) -> MemResult<()> {
        if in_uart(addr) {
            *self.pending.lock().unwrap() = Some((addr, data.to_vec()));
            return Ok(());
        }
        self.mem
            .write_slice(data, GuestAddress(addr))
            .map_err(|_| MemError::OutOfBounds {
                addr,
                size: data.len(),
            })
    }
}

/// RISC-V vCPU backed by the software interpreter.
pub struct RiscVVcpu {
    id: u32,
    cpu: RiscVCpu,
    pending: MmioSink,
    halted: bool,
}

impl RiscVVcpu {
    pub fn new(id: u32, mem: Arc<GuestMemoryMmap>) -> Self {
        let pending: MmioSink = Arc::new(Mutex::new(None));
        let bridge = GuestBridge {
            mem,
            pending: pending.clone(),
        };
        let cpu = RiscVCpu::new(RiscVConfig::rv64gc(), Box::new(bridge));
        RiscVVcpu {
            id,
            cpu,
            pending,
            halted: false,
        }
    }
}

impl VCpu for RiscVVcpu {
    fn run(&mut self) -> Result<VcpuExit> {
        if self.halted {
            return Ok(VcpuExit::Hlt);
        }
        for _ in 0..MAX_ITERS {
            let exit = self.cpu.step();
            // Surface any UART output produced by this instruction first.
            if let Some((addr, data)) = self.pending.lock().unwrap().take() {
                return Ok(VcpuExit::MmioWrite { addr, data });
            }
            match exit {
                RiscVExit::Continue => {}
                RiscVExit::Ecall => {
                    // Environment call: treat as a request to power down.
                    self.halted = true;
                    return Ok(VcpuExit::Shutdown);
                }
                RiscVExit::Ebreak => {
                    self.halted = true;
                    return Ok(VcpuExit::Debug);
                }
                RiscVExit::Wfi => return Ok(VcpuExit::Hlt),
                RiscVExit::Trap(t) => {
                    self.halted = true;
                    return Ok(VcpuExit::Unknown(format!(
                        "riscv trap: cause={} tval={:#x} pc={:#x}",
                        t.cause,
                        t.tval,
                        self.cpu.pc()
                    )));
                }
            }
        }
        Ok(VcpuExit::Hlt)
    }

    fn get_state(&self) -> Result<CpuState> {
        let mut regs = RiscVRegisters::default();
        for i in 0..32u8 {
            regs.x[i as usize] = self.cpu.x(i);
            regs.f[i as usize] = self.cpu.f(i);
        }
        regs.pc = self.cpu.pc();
        regs.fcsr = self.cpu.fcsr();
        Ok(CpuState::riscv(regs))
    }

    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        let state = match state {
            CpuState::RiscV(s) => s,
            _ => {
                return Err(Error::Emulator(
                    "expected riscv state for riscv vCPU".to_string(),
                ));
            }
        };
        for i in 0..32u8 {
            self.cpu.set_x(i, state.regs.x[i as usize]);
            self.cpu.set_f(i, state.regs.f[i as usize]);
        }
        self.cpu.set_pc(state.regs.pc);
        self.cpu.set_fcsr(state.regs.fcsr);
        self.halted = false;
        Ok(())
    }

    fn complete_io_in(&mut self, _data: &[u8]) {
        // UART reads are serviced synchronously inside the bridge; no resume
        // state is pending.
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn instruction_count(&self) -> u64 {
        self.cpu.instret()
    }
}
