//! CPU abstraction layer.
//!
//! This module provides backend-agnostic types and traits for CPU emulation.

pub mod exit;
pub mod state;

pub use exit::VcpuExit;
pub use state::{
    Aarch32CpuState, Aarch32Registers, Aarch32SystemRegisters, Aarch64CpuState, Aarch64Registers,
    Aarch64SystemRegisters, CortexMCpuState, CortexMRegisters, CortexMSystemRegisters, CpuState,
    DescriptorTable, HexagonCpuState, HexagonRegisters, Registers, RiscVCpuState, RiscVRegisters,
    Segment, SystemRegisters, X86_64CpuState,
};

use crate::error::{Error, Result};

/// Abstract vCPU interface.
///
/// This trait is implemented by both KVM and emulator backends.
pub trait VCpu: Send {
    /// Run the vCPU until an exit condition.
    fn run(&mut self) -> Result<VcpuExit>;

    /// Get general-purpose registers (x86_64 only).
    fn get_regs(&self) -> Result<Registers> {
        match self.get_state()? {
            CpuState::X86_64(state) => Ok(state.regs),
            _ => Err(Error::InvalidConfig(
                "register access is only supported for x86_64".to_string(),
            )),
        }
    }

    /// Set general-purpose registers (x86_64 only).
    fn set_regs(&mut self, regs: &Registers) -> Result<()> {
        match self.get_state()? {
            CpuState::X86_64(state) => self.set_state(&CpuState::x86_64(regs.clone(), state.sregs)),
            _ => Err(Error::InvalidConfig(
                "register access is only supported for x86_64".to_string(),
            )),
        }
    }

    /// Get system registers (x86_64 only).
    fn get_sregs(&self) -> Result<SystemRegisters> {
        match self.get_state()? {
            CpuState::X86_64(state) => Ok(state.sregs),
            _ => Err(Error::InvalidConfig(
                "system register access is only supported for x86_64".to_string(),
            )),
        }
    }

    /// Set system registers (x86_64 only).
    fn set_sregs(&mut self, sregs: &SystemRegisters) -> Result<()> {
        match self.get_state()? {
            CpuState::X86_64(state) => self.set_state(&CpuState::x86_64(state.regs, sregs.clone())),
            _ => Err(Error::InvalidConfig(
                "system register access is only supported for x86_64".to_string(),
            )),
        }
    }

    /// Get complete CPU state.
    fn get_state(&self) -> Result<CpuState>;

    /// Set complete CPU state.
    fn set_state(&mut self, state: &CpuState) -> Result<()>;

    /// Complete an I/O in operation by providing the data read from the device.
    fn complete_io_in(&mut self, data: &[u8]);

    /// Inject an external interrupt (hardware IRQ).
    /// Returns Ok(true) if the interrupt was injected, Ok(false) if interrupts are disabled.
    fn inject_interrupt(&mut self, vector: u8) -> Result<bool> {
        // Default implementation does nothing
        let _ = vector;
        Ok(false)
    }

    /// Check if interrupts are enabled and can be injected.
    fn can_inject_interrupt(&self) -> bool {
        false
    }

    /// Inject a Non-Maskable Interrupt (NMI).
    /// NMIs are delivered regardless of the IF flag.
    /// Returns Ok(true) if delivered, Ok(false) if blocked (e.g., during NMI handling).
    fn inject_nmi(&mut self) -> Result<bool> {
        Ok(false)
    }

    /// Attach the shared PL011 console device so the vCPU's memory bridge can
    /// service UART MMIO synchronously against the same instance the VMM
    /// feeds with host console input. Default no-op — only the AArch64
    /// emulator backend uses it.
    fn attach_pl011(
        &mut self,
        _base: u64,
        _uart: std::sync::Arc<std::sync::Mutex<crate::devices::pl011::Pl011>>,
    ) {
    }

    /// Attach the shared Samsung S3C UART console device (ARMv6/S3C64xx
    /// machines). Default no-op.
    fn attach_s3c_uart(
        &mut self,
        _uart: std::sync::Arc<std::sync::Mutex<crate::devices::s3c64xx::S3cUart>>,
    ) {
    }

    /// Attach the PCI host bridge so the emulator MMU can divert a physical
    /// MMIO aperture from RAM to PCI device BAR handlers. `ap_base..ap_end`
    /// bounds that aperture. Default no-op — only the x86_64 emulator backend
    /// implements MMIO-BAR routing.
    fn set_pci_bridge(
        &mut self,
        _bridge: std::sync::Arc<std::sync::Mutex<crate::devices::pci::PciStub>>,
        _ap_base: u64,
        _ap_end: u64,
    ) {
    }

    /// Enable or disable single-step mode for debugging.
    #[cfg(feature = "debug")]
    fn set_single_step(&mut self, enabled: bool) {
        let _ = enabled;
    }

    /// Check if single-step mode is enabled.
    #[cfg(feature = "debug")]
    fn is_single_step(&self) -> bool {
        false
    }

    /// Invalidate any cached instruction decodes for the given address.
    /// Called when modifying code memory (e.g., for software breakpoints).
    #[cfg(feature = "debug")]
    fn invalidate_code_cache(&mut self, addr: u64) {
        let _ = addr;
    }

    /// Get vCPU ID.
    fn id(&self) -> u32;

    /// Get current instruction count (for snapshotting).
    fn instruction_count(&self) -> u64 {
        0
    }

    /// Get extended emulator state for snapshotting.
    /// Returns None for backends that don't support it.
    fn get_emulator_state(&self) -> Option<crate::snapshot::EmulatorState> {
        None
    }

    /// Set extended emulator state (for snapshot restore).
    fn set_emulator_state(&mut self, _state: &crate::snapshot::EmulatorState) -> Result<()> {
        Ok(())
    }
}
