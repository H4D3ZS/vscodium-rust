//! Architecture abstraction layer.

pub mod arm;
pub mod fdt;
pub mod hexagon;
pub mod riscv;
pub mod x86_64;

use vm_memory::{GuestAddress, GuestMemoryMmap};

#[cfg(all(feature = "kvm", target_os = "linux"))]
use crate::backend::kvm::KvmVm;
use crate::config::{ArchKind, VmConfig};
use crate::cpu::CpuState;
use crate::devices::bus::{IoBus, MmioBus};
use crate::error::Result;

// Re-export ARM boot info
pub use arm::ArmBootInfo;

/// Boot information for x86_64 kernel loading.
pub struct X86_64BootInfo {
    pub entry_point: u64,
    pub boot_params_addr: GuestAddress,
    pub tss_addr: u64,
    pub identity_map_addr: u64,
}

/// Boot information for Hexagon bare-metal loading.
pub struct HexagonBootInfo {
    pub entry_point: u64,
    pub load_addr: u64,
    pub image_size: u64,
}

/// Boot information for RISC-V bare-metal loading.
pub struct RiscVBootInfo {
    pub entry_point: u64,
    pub load_addr: u64,
    pub image_size: u64,
}

/// Boot information returned after image loading.
pub enum BootInfo {
    X86_64(X86_64BootInfo),
    Hexagon(HexagonBootInfo),
    Arm(ArmBootInfo),
    RiscV(RiscVBootInfo),
}

impl BootInfo {
    pub fn entry_point(&self) -> u64 {
        match self {
            BootInfo::X86_64(info) => info.entry_point,
            BootInfo::Hexagon(info) => info.entry_point,
            BootInfo::Arm(info) => info.entry_point,
            BootInfo::RiscV(info) => info.entry_point,
        }
    }

    pub fn as_x86_64(&self) -> Option<&X86_64BootInfo> {
        match self {
            BootInfo::X86_64(info) => Some(info),
            _ => None,
        }
    }

    pub fn as_hexagon(&self) -> Option<&HexagonBootInfo> {
        match self {
            BootInfo::Hexagon(info) => Some(info),
            _ => None,
        }
    }

    pub fn as_arm(&self) -> Option<&ArmBootInfo> {
        match self {
            BootInfo::Arm(info) => Some(info),
            _ => None,
        }
    }

    pub fn as_riscv(&self) -> Option<&RiscVBootInfo> {
        match self {
            BootInfo::RiscV(info) => Some(info),
            _ => None,
        }
    }
}

/// Architecture abstraction trait.
pub trait Arch: Send + Sync {
    /// Architecture name.
    fn name(&self) -> &'static str;

    /// Setup architecture-specific I/O devices.
    fn setup_devices(&self, io_bus: &mut IoBus, mmio_bus: &mut MmioBus) -> Result<()>;

    /// Optional MMIO base for the serial device.
    fn serial_mmio_base(&self) -> Option<u64> {
        None
    }

    /// Optional IRQ line for the serial device.
    fn serial_irq(&self) -> Option<u32> {
        None
    }

    /// Guest physical address where platform RAM begins. x86 RAM starts at 0;
    /// ARM platforms place RAM above the device MMIO window.
    fn ram_base(&self) -> u64 {
        0
    }

    /// Load kernel and prepare boot environment.
    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo>;

    /// Initialize VM-level state (IRQ chip, PIT, TSS, identity map).
    /// This is KVM-specific.
    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, vm: &KvmVm, boot: &BootInfo) -> Result<()>;

    /// Get initial CPU state for booting.
    /// Writes necessary structures (GDT, page tables) to guest memory
    /// and returns the initial CPU state.
    fn initial_cpu_state(&self, mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState>;
}

/// Create an architecture implementation from kind.
pub fn from_kind(kind: ArchKind) -> Box<dyn Arch> {
    match kind {
        ArchKind::X86_64 => Box::new(x86_64::X86_64Arch::new()),
        ArchKind::Hexagon => Box::new(hexagon::HexagonArch::new()),
        ArchKind::Aarch64 => Box::new(arm::Aarch64Arch::new()),
        ArchKind::Armv7a => Box::new(arm::Armv7aArch::new()),
        ArchKind::Armv8a32 => Box::new(arm::Armv8a32Arch::new()),
        ArchKind::CortexM => Box::new(arm::CortexMArch::new()),
        ArchKind::CortexR => Box::new(arm::CortexRArch::new()),
        ArchKind::Riscv64 => Box::new(riscv::Riscv64Arch::new()),
    }
}
