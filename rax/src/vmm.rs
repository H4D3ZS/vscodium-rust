use std::sync::Arc;
#[cfg(feature = "debug")]
use std::sync::mpsc::TryRecvError;
#[cfg(feature = "debug")]
use std::thread::JoinHandle;

use tracing::{debug, info};
use vm_memory::Bytes;

use crate::arch::{
    self, Arch, ArmBootInfo, BootInfo, HexagonBootInfo, RiscVBootInfo, X86_64BootInfo,
};
use crate::backend::emulator::x86_64::get_total_instruction_count;
#[cfg(all(feature = "hvf", target_os = "macos", target_arch = "x86_64"))]
use crate::backend::hvf::HvfVm;
#[cfg(all(feature = "kvm", target_os = "linux"))]
use crate::backend::kvm::KvmVm;
use crate::backend::{self, Vm};
#[cfg(any(
    all(feature = "kvm", target_os = "linux"),
    all(feature = "hvf", target_os = "macos")
))]
use crate::config::BackendKind;
use crate::config::{ArchKind, CheckpointConfig, VmConfig};
use crate::console::{Console, ConsoleAction, ESCAPE_HELP};
use crate::cpu::{CpuState, VCpu, VcpuExit};
use crate::devices::bus::{IoBus, IoDevice, IoRange, MmioBus, MmioRange, SharedIoDevice};
use crate::devices::lapic::{
    IpiRequest, IpiTarget, LAPIC_BASE, LAPIC_SIZE, LapicDevice, LocalApic,
};
use crate::devices::pci::{PCI_CONFIG_ADDRESS, PciStub};
use crate::devices::pic::{DualPic, MasterPicDevice, SlavePicDevice};
use crate::devices::pit::Pit;
use crate::devices::pl011::{Pl011, Pl011MmioDevice};
use crate::devices::s3c64xx::S3cUart;
use crate::devices::serial::{Serial16550, SerialMmioDevice};
use crate::error::{Error, Result};
#[cfg(feature = "debug")]
use crate::gdb::{self, GdbCommand, GdbResponse, VmmGdbChannels};
use crate::memory::GuestMemoryWrapper;
use crate::snapshot::{
    DEFAULT_CHECKPOINT_FILE, DeviceState, EmulatorState, Snapshot, SnapshotConfig,
};
use crate::terminal::RawTty;

const SERIAL_BASE: u16 = 0x3f8;

/// Physical aperture diverted from RAM to PCI BAR-mapped MMIO when the optional
/// PCI devices are attached. Chosen high (just below the IOAPIC/HPET/LAPIC fixed
/// MMIO at 0xFEC00000+) so it sits above the guest's RAM and per-CPU overflow
/// window, and inside the 32-bit PCI memory window the kernel advertises.
const PCI_MMIO_AP_BASE: u64 = 0xFC00_0000;
const PCI_MMIO_AP_END: u64 = 0xFE00_0000;

/// Set by the SIGUSR1 handler to request a checkpoint from the run loop. This is
/// the non-keyboard "event" trigger: `kill -USR1 <pid>` dumps a checkpoint to
/// the configured `--snapshot-out` path, the same as the `Ctrl-A s` hotkey.
static CHECKPOINT_SIGNAL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(unix)]
extern "C" fn sigusr1_handler(_sig: libc::c_int) {
    CHECKPOINT_SIGNAL.store(true, std::sync::atomic::Ordering::SeqCst);
}

/// A zeroed [`BootInfo`] for the given arch, used when resuming from a
/// checkpoint (where no kernel is loaded, so there is no real boot info — the
/// machine state comes entirely from the checkpoint).
fn null_boot_info(arch: ArchKind) -> BootInfo {
    match arch {
        ArchKind::Hexagon => BootInfo::Hexagon(HexagonBootInfo {
            entry_point: 0,
            load_addr: 0,
            image_size: 0,
        }),
        ArchKind::Riscv64 => BootInfo::RiscV(RiscVBootInfo {
            entry_point: 0,
            load_addr: 0,
            image_size: 0,
        }),
        ArchKind::Aarch64
        | ArchKind::Armv7a
        | ArchKind::Armv8a32
        | ArchKind::CortexM
        | ArchKind::CortexR => BootInfo::Arm(ArmBootInfo {
            entry_point: 0,
            load_addr: 0,
            image_size: 0,
            dtb_addr: None,
            initial_sp: None,
        }),
        ArchKind::X86_64 => BootInfo::X86_64(X86_64BootInfo {
            entry_point: 0,
            boot_params_addr: vm_memory::GuestAddress(0),
            tss_addr: 0,
            identity_map_addr: 0,
        }),
    }
}

/// Guest-memory DMA adapter for the PCI device models (e.g. the NVMe/AHCI
/// admin/command queues). Wraps the guest RAM mmap behind the device models'
/// `virtio::Mem` trait so a bus-master device can read/write guest physical
/// memory; the same mmap the CPU sees, so DMA is coherent.
struct GuestDmaMem(Arc<vm_memory::GuestMemoryMmap>);

impl crate::devices::virtio::Mem for GuestDmaMem {
    fn read(&self, gpa: u64, buf: &mut [u8]) -> bool {
        self.0.read_slice(buf, vm_memory::GuestAddress(gpa)).is_ok()
    }
    fn write(&mut self, gpa: u64, buf: &[u8]) -> bool {
        self.0
            .write_slice(buf, vm_memory::GuestAddress(gpa))
            .is_ok()
    }
}

/// AC'97 has two separate I/O BARs (the 16-bit codec mixer "NAM" and the
/// bus-master "NABM" register blocks). The PCI bridge maps one handler per BAR,
/// so the device is shared behind an Arc<Mutex<Ac97>> and exposed through two
/// thin window adapters that receive BAR-relative offsets.
type SharedAc97 = Arc<std::sync::Mutex<crate::devices::ac97::Ac97>>;

/// BAR0 window: the 16-bit codec mixer (NAM). The bus dispatches byte-wise, so
/// each access maps to the appropriate byte of the 16-bit mixer register.
struct Ac97NamWindow(SharedAc97);
impl IoDevice for Ac97NamWindow {
    fn read(&mut self, off: u16) -> u8 {
        match self.0.lock() {
            Ok(d) => {
                let word = d.mixer_read(off & !1);
                if off & 1 == 0 {
                    word as u8
                } else {
                    (word >> 8) as u8
                }
            }
            Err(_) => 0xff,
        }
    }
    fn write(&mut self, off: u16, value: u8) {
        if let Ok(mut d) = self.0.lock() {
            let reg = off & !1;
            let cur = d.mixer_read(reg);
            let new = if off & 1 == 0 {
                (cur & 0xFF00) | u16::from(value)
            } else {
                (cur & 0x00FF) | (u16::from(value) << 8)
            };
            d.mixer_write(reg, new);
        }
    }
}

/// BAR1 window: the bus-master register block (NABM).
struct Ac97NabmWindow(SharedAc97);
impl IoDevice for Ac97NabmWindow {
    fn read(&mut self, off: u16) -> u8 {
        self.0.lock().map(|d| d.nabm_read_u8(off)).unwrap_or(0xff)
    }
    fn write(&mut self, off: u16, value: u8) {
        if let Ok(mut d) = self.0.lock() {
            d.nabm_write_u8(off, value);
        }
    }
}

/// Attach the optional PCI device models to the host bridge. Their BARs are
/// pre-assigned (memory BARs inside [`PCI_MMIO_AP_BASE`], I/O BARs at fixed
/// ports) so the guest sees valid firmware resources; the bridge tracks any
/// reprogramming. Device models are built with a zero base so the bridge can
/// feed them BAR-relative offsets wherever the BAR lands.
fn attach_pci_devices(
    pci: &Arc<std::sync::Mutex<PciStub>>,
    mem: Arc<vm_memory::GuestMemoryMmap>,
) -> Result<()> {
    use crate::devices::pci::{Bar, ConfigSpace};
    let mut bridge = pci
        .lock()
        .map_err(|_| Error::Emulator("pci bridge lock poisoned".to_string()))?;

    // Intel 82540EM (e1000) NIC at 00:01.0 — BAR0 is a 128 KiB MMIO register
    // block, pre-assigned at the base of the MMIO aperture.
    let mac = [0x52, 0x54, 0x00, 0x12, 0x34, 0x56];
    let mut e1000_cfg = ConfigSpace::device(0x8086, 0x100E, 0x02_00_00, 0x00);
    e1000_cfg.set_bar(0, Bar::mem32(0x2_0000));
    e1000_cfg.set_u32(0x10, PCI_MMIO_AP_BASE as u32);
    e1000_cfg.set_u16(0x04, 0x0006); // memory-space + bus-master enable
    e1000_cfg.set_u8(0x3d, 0x01); // interrupt pin INTA#
    bridge.attach_mmio(
        0,
        1,
        0,
        e1000_cfg,
        0,
        Box::new(crate::devices::e1000::E1000::new(mac)),
    );

    // Intel PIIX-style UHCI USB controller at 00:02.0 — BAR4 is a 32-byte I/O
    // window pre-assigned at port 0xC000, reached via the IoBus PCI fallback.
    let mut uhci_cfg = ConfigSpace::device(0x8086, 0x7020, 0x0C_03_00, 0x00);
    uhci_cfg.set_bar(4, Bar::io(0x20));
    uhci_cfg.set_u32(0x10 + 4 * 4, 0x0000_C001); // I/O base 0xC000 (bit0 = I/O)
    uhci_cfg.set_u16(0x04, 0x0001); // I/O-space enable
    bridge.attach_pio(
        0,
        2,
        0,
        uhci_cfg,
        4,
        Box::new(crate::devices::uhci::Uhci::new(0)),
    );

    // NVMe controller at 00:03.0 — BAR0 is an 8 KiB MMIO register block. The
    // controller DMAs its admin queues from guest RAM via the Mem adapter. With
    // no disk attached it enumerates with zero namespaces.
    let mut nvme_cfg = ConfigSpace::device(0x8086, 0x5845, 0x01_08_02, 0x00);
    nvme_cfg.set_bar(0, Bar::mem32(0x2000));
    nvme_cfg.set_u32(0x10, (PCI_MMIO_AP_BASE + 0x10_0000) as u32);
    nvme_cfg.set_u16(0x04, 0x0006); // memory-space + bus-master enable
    bridge.attach_mmio(
        0,
        3,
        0,
        nvme_cfg,
        0,
        Box::new(crate::devices::nvme::NvmeController::new(
            0,
            GuestDmaMem(mem.clone()),
            Vec::new(),
        )),
    );

    // AHCI SATA controller at 00:04.0 — ABAR (BAR5) is an 8 KiB MMIO block. No
    // SATA drive attached, so it enumerates with no ports populated.
    let mut ahci_cfg = ConfigSpace::device(0x8086, 0x2922, 0x01_06_01, 0x00);
    ahci_cfg.set_bar(5, Bar::mem32(0x2000));
    ahci_cfg.set_u32(0x10 + 5 * 4, (PCI_MMIO_AP_BASE + 0x20_0000) as u32);
    ahci_cfg.set_u16(0x04, 0x0006);
    bridge.attach_mmio(
        0,
        4,
        0,
        ahci_cfg,
        5,
        Box::new(crate::devices::ahci::AhciController::new(
            0,
            GuestDmaMem(mem),
            Vec::new(),
        )),
    );

    // Intel ICH AC'97 audio at 00:05.0 — two I/O BARs: BAR0 = NAM (256-byte
    // codec mixer), BAR1 = NABM (64-byte bus-master). One shared device, two
    // window endpoints.
    let ac97 = Arc::new(std::sync::Mutex::new(crate::devices::ac97::Ac97::new()));
    let mut ac97_cfg = ConfigSpace::device(0x8086, 0x2415, 0x04_01_00, 0x00);
    ac97_cfg.set_bar(0, Bar::io(0x100));
    ac97_cfg.set_bar(1, Bar::io(0x40));
    ac97_cfg.set_u32(0x10, 0x0000_D001); // NAM  @ I/O 0xD000
    ac97_cfg.set_u32(0x14, 0x0000_D101); // NABM @ I/O 0xD100
    ac97_cfg.set_u16(0x04, 0x0001); // I/O-space enable
    bridge.attach_pio(
        0,
        5,
        0,
        ac97_cfg.clone(),
        0,
        Box::new(Ac97NamWindow(ac97.clone())),
    );
    bridge.attach_pio(0, 5, 0, ac97_cfg, 1, Box::new(Ac97NabmWindow(ac97)));

    Ok(())
}

/// Wrapper to make Pit implement IoDevice via shared reference
struct PitDevice {
    pit: Arc<std::sync::Mutex<Pit>>,
}

impl PitDevice {
    fn new(pit: Arc<std::sync::Mutex<Pit>>) -> Self {
        PitDevice { pit }
    }
}

impl IoDevice for PitDevice {
    fn read(&mut self, port: u16) -> u8 {
        if let Ok(mut pit) = self.pit.lock() {
            pit.read(port)
        } else {
            0xFF
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        if let Ok(mut pit) = self.pit.lock() {
            pit.write(port, value);
        }
    }
}

pub struct Vmm {
    vm: Box<dyn Vm>,
    guest_mem: GuestMemoryWrapper,
    io_bus: IoBus,
    mmio_bus: MmioBus,
    serial: Arc<std::sync::Mutex<Serial16550>>,
    /// PL011 console device (AArch64 machines only).
    pl011: Option<Arc<std::sync::Mutex<Pl011>>>,
    /// Samsung S3C UART console device (ARMv6 machines only).
    s3c_uart: Option<Arc<std::sync::Mutex<S3cUart>>>,
    pit: Arc<std::sync::Mutex<Pit>>,
    pic: Arc<std::sync::Mutex<DualPic>>,
    lapic: Arc<std::sync::Mutex<LocalApic>>,
    vcpus: Vec<Box<dyn VCpu>>,
    arch: Box<dyn Arch>,
    boot_info: BootInfo,
    serial_mmio_base: Option<u64>,
    serial_irq: Option<u32>,
    /// GDB server channels (when --features debug and --gdb is used).
    #[cfg(feature = "debug")]
    gdb_channels: Option<VmmGdbChannels>,
    /// GDB server thread handle.
    #[cfg(feature = "debug")]
    gdb_thread: Option<JoinHandle<()>>,
    /// Whether in single-step mode for GDB.
    #[cfg(feature = "debug")]
    gdb_single_step: bool,
    /// Whether GDB requested a stop.
    #[cfg(feature = "debug")]
    gdb_stopped: bool,
    /// Software breakpoints: addr -> original byte.
    #[cfg(feature = "debug")]
    gdb_breakpoints: std::collections::HashMap<u64, u8>,
    /// Wait for GDB connection before starting.
    #[cfg(feature = "debug")]
    wait_gdb: bool,
    /// Snapshot configuration
    snapshot_config: Option<SnapshotConfig>,
    /// Last instruction count when snapshot was taken
    last_snapshot_insn: u64,
    /// Machine-defining config embedded into every checkpoint this VM writes,
    /// so the resulting `.rxc` resumes self-contained.
    checkpoint_config: CheckpointConfig,
    /// Where hotkey/signal-triggered checkpoints are written.
    snapshot_out: std::path::PathBuf,
}

impl Vmm {
    /// Build a fresh machine: load the kernel and set boot-time CPU state.
    pub fn new(config: VmConfig) -> Result<Self> {
        Self::build(config, false)
    }

    /// Build a machine to resume from a checkpoint. Skips kernel loading,
    /// backend VM init, and boot-time CPU state — the entire machine image
    /// (RAM, registers, devices) is restored from the checkpoint afterwards via
    /// [`restore_snapshot`](Self::restore_snapshot).
    pub fn new_resume(config: VmConfig) -> Result<Self> {
        Self::build(config, true)
    }

    fn build(config: VmConfig, resume: bool) -> Result<Self> {
        info!(
            vcpus = config.vcpus,
            mem_bytes = config.memory.bytes(),
            resume = resume,
            "initializing VMM"
        );

        // Initialize instruction tracing if requested (requires trace feature)
        #[cfg(feature = "trace")]
        if let Some(ref trace_path) = config.trace {
            crate::trace::init(trace_path)
                .map_err(|e| Error::InvalidConfig(format!("failed to open trace file: {}", e)))?;
            info!(trace_path = ?trace_path, "instruction tracing enabled");
        }
        #[cfg(not(feature = "trace"))]
        if config.trace.is_some() {
            return Err(Error::InvalidConfig(
                "--trace requires building with --features trace".to_string(),
            ));
        }

        // Create backend
        let backend = backend::create(&config)?;
        info!(backend = backend.name(), "using backend");

        // Create VM
        let vm = backend.create_vm()?;

        // Allocate guest memory
        let guest_mem = GuestMemoryWrapper::new(config.memory.bytes())?;

        // Register memory with VM (backend-specific)
        #[cfg(all(feature = "kvm", target_os = "linux"))]
        if matches!(config.backend, BackendKind::Kvm) {
            let kvm_vm = vm
                .as_any()
                .downcast_ref::<KvmVm>()
                .ok_or_else(|| Error::InvalidConfig("expected KVM VM".to_string()))?;
            guest_mem.register(kvm_vm.vm_fd())?;
        }
        #[cfg(all(feature = "hvf", target_os = "macos", target_arch = "x86_64"))]
        if matches!(config.backend, BackendKind::Hvf) {
            use crate::backend::hvf::HvfVm;
            let hvf_vm = vm
                .as_any()
                .downcast_ref::<HvfVm>()
                .ok_or_else(|| Error::InvalidConfig("expected HVF VM".to_string()))?;
            hvf_vm.register_memory(&guest_mem)?;
        }
        // Emulator accesses memory directly, no registration needed

        // Setup architecture
        let arch = arch::from_kind(config.arch);
        info!(arch = arch.name(), "selected architecture");

        // ARM64 HVF: map only the platform RAM window into the guest so that
        // device MMIO below it (UART, ...) traps out as data aborts.
        #[cfg(all(feature = "hvf", target_os = "macos", target_arch = "aarch64"))]
        if matches!(config.backend, BackendKind::Hvf) {
            use crate::backend::hvf::HvfArm64Vm;
            let hvf_vm = vm
                .as_any()
                .downcast_ref::<HvfArm64Vm>()
                .ok_or_else(|| Error::InvalidConfig("expected ARM64 HVF VM".to_string()))?;
            hvf_vm.register_memory(&guest_mem, arch.ram_base(), config.memory.bytes())?;
        }

        // Setup I/O devices
        let mut io_bus = IoBus::new();
        let mut mmio_bus = MmioBus::new();
        arch.setup_devices(&mut io_bus, &mut mmio_bus)?;

        // PCI host bridge (x86). Owned here so it can be shared with the
        // emulator MMU for BAR-mapped MMIO routing. The bridge (config ports
        // 0xCF8-0xCFF) is always present on x86; the optional device models are
        // attached only with --pci-devices.
        let pci_bridge: Option<Arc<std::sync::Mutex<PciStub>>> = if config.arch == ArchKind::X86_64
        {
            let pci = Arc::new(std::sync::Mutex::new(PciStub::new()));
            if config.pci_devices {
                attach_pci_devices(&pci, Arc::new(guest_mem.memory().clone()))?;
                info!("attached optional PCI devices (e1000, uhci, nvme, ahci, ac97)");
            }
            io_bus.register(
                IoRange {
                    base: PCI_CONFIG_ADDRESS,
                    len: 8,
                },
                Box::new(SharedIoDevice::new(pci.clone())),
            )?;
            // Dynamically-assigned PCI I/O BARs are reached via this fallback.
            io_bus.set_pci(pci.clone());
            Some(pci)
        } else {
            None
        };

        let serial_mmio_base = arch.serial_mmio_base();
        let serial_irq = arch.serial_irq();

        // Create serial device with input enabled. AArch64 uses a PL011 at
        // the platform UART base; everything else keeps the 16550.
        let pl011 = if config.arch == ArchKind::Aarch64 {
            let uart = Arc::new(std::sync::Mutex::new(Pl011::new()));
            if let Some(base) = serial_mmio_base {
                mmio_bus.register(
                    MmioRange { base, len: 0x1000 },
                    Box::new(Pl011MmioDevice::new(base, uart.clone())),
                )?;
            }
            Some(uart)
        } else {
            None
        };

        // ARMv6/S3C64xx console: the UART lives inside the vCPU's memory
        // bridge; the VMM only feeds it host console input.
        let s3c_uart = if config.arch == ArchKind::Armv7a {
            Some(Arc::new(std::sync::Mutex::new(S3cUart::new())))
        } else {
            None
        };

        let serial = Arc::new(std::sync::Mutex::new(Serial16550::new(SERIAL_BASE)));
        if let Ok(mut serial_guard) = serial.lock() {
            if let Some(base) = serial_mmio_base {
                serial_guard.set_mmio_base(base);
            }
        }
        if pl011.is_none() && s3c_uart.is_none() {
            if let Some(base) = serial_mmio_base {
                mmio_bus.register(
                    MmioRange { base, len: 8 },
                    Box::new(SerialMmioDevice::new(serial.clone())),
                )?;
            }
        }

        // Create PIT (Programmable Interval Timer) at ports 0x40-0x43
        let pit = Arc::new(std::sync::Mutex::new(Pit::new()));

        // Create PIC (Programmable Interrupt Controller)
        let pic = Arc::new(std::sync::Mutex::new(DualPic::new()));

        // Register PIT on I/O bus
        io_bus.register(
            IoRange { base: 0x40, len: 4 },
            Box::new(PitDevice::new(pit.clone())),
        )?;

        // Register master PIC (0x20-0x21)
        io_bus.register(
            IoRange { base: 0x20, len: 2 },
            Box::new(MasterPicDevice::new(pic.clone())),
        )?;

        // Register slave PIC (0xA0-0xA1)
        io_bus.register(
            IoRange { base: 0xA0, len: 2 },
            Box::new(SlavePicDevice::new(pic.clone())),
        )?;

        // Create and register Local APIC at 0xFEE00000
        let lapic = Arc::new(std::sync::Mutex::new(LocalApic::new(0)));
        mmio_bus.register(
            MmioRange {
                base: LAPIC_BASE,
                len: LAPIC_SIZE,
            },
            Box::new(LapicDevice::new(lapic.clone())),
        )?;

        // Load kernel — skipped when resuming (the kernel is already present in
        // the RAM image restored from the checkpoint).
        let boot_info = if resume {
            null_boot_info(config.arch)
        } else {
            arch.load_kernel(guest_mem.memory(), &config)?
        };

        // Initialize VM (backend-specific). Skipped on resume: the emulator
        // backend needs none, and a resumed machine takes its full CPU/segment
        // state from the checkpoint rather than the boot-time init.
        #[cfg(all(feature = "kvm", target_os = "linux"))]
        if !resume && matches!(config.backend, BackendKind::Kvm) {
            let kvm_vm = vm
                .as_any()
                .downcast_ref::<KvmVm>()
                .ok_or_else(|| Error::InvalidConfig("expected KVM VM".to_string()))?;
            arch.init_vm(kvm_vm, &boot_info)?;
        }
        // Emulator doesn't need VM-level initialization

        debug!("creating vCPUs");
        // Create vCPUs
        let mem_arc = Arc::new(guest_mem.memory().clone());
        let mut vcpus = Vec::with_capacity(config.vcpus as usize);
        for cpu_id in 0..config.vcpus {
            debug!(cpu_id, "creating vCPU");
            let mut vcpu = vm.create_vcpu(cpu_id as u32, mem_arc.clone())?;
            debug!(cpu_id, "created vCPU, setting initial state");

            // Setup initial CPU state for BSP (cpu 0). Skipped on resume — the
            // checkpoint restore sets the full register file afterwards.
            if cpu_id == 0 && !resume {
                let initial_state = arch.initial_cpu_state(guest_mem.memory(), &boot_info)?;
                vcpu.set_state(&initial_state)?;
            }

            // Share the PL011 console device with the vCPU so its memory
            // bridge services UART MMIO against the instance the VMM feeds.
            if let (Some(pl011), Some(base)) = (&pl011, serial_mmio_base) {
                vcpu.attach_pl011(base, pl011.clone());
            }
            if let Some(uart) = &s3c_uart {
                vcpu.attach_s3c_uart(uart.clone());
            }

            debug!(vcpu_id = cpu_id, "created vCPU");
            vcpus.push(vcpu);
        }

        // Hand the PCI host bridge to the BSP's MMU so BAR-mapped MMIO in the
        // aperture is routed to PCI device handlers (emulator backend only; the
        // default VCpu::set_pci_bridge is a no-op for KVM/HVF). Only needed when
        // device models are actually attached.
        if config.pci_devices {
            if let Some(ref pci) = pci_bridge {
                if let Some(vcpu) = vcpus.get_mut(0) {
                    vcpu.set_pci_bridge(pci.clone(), PCI_MMIO_AP_BASE, PCI_MMIO_AP_END);
                }
            }
        }

        // Initialize GDB server if configured
        #[cfg(feature = "debug")]
        let (gdb_channels, gdb_thread) = if let Some(port) = config.gdb_port {
            let (gdb_ch, vmm_ch) = gdb::create_channels();
            let handle = gdb::spawn_server(port, gdb_ch, config.wait_gdb)
                .map_err(|e| Error::InvalidConfig(format!("failed to start GDB server: {}", e)))?;
            (Some(vmm_ch), Some(handle))
        } else {
            (None, None)
        };

        #[cfg(not(feature = "debug"))]
        if config.gdb_port.is_some() {
            return Err(Error::InvalidConfig(
                "--gdb requires building with --features debug".to_string(),
            ));
        }

        // Capture the machine-defining config for embedding in checkpoints, and
        // resolve where hotkey/signal checkpoints will be written.
        let checkpoint_config = config.to_checkpoint();
        let snapshot_out = config
            .snapshot_out
            .clone()
            .unwrap_or_else(|| std::path::PathBuf::from(DEFAULT_CHECKPOINT_FILE));

        Ok(Vmm {
            vm,
            guest_mem,
            io_bus,
            mmio_bus,
            serial,
            pl011,
            s3c_uart,
            pit,
            pic,
            lapic,
            vcpus,
            arch,
            boot_info,
            serial_mmio_base,
            serial_irq,
            #[cfg(feature = "debug")]
            gdb_channels,
            #[cfg(feature = "debug")]
            gdb_thread,
            #[cfg(feature = "debug")]
            gdb_single_step: false,
            #[cfg(feature = "debug")]
            gdb_stopped: false,
            #[cfg(feature = "debug")]
            gdb_breakpoints: std::collections::HashMap::new(),
            #[cfg(feature = "debug")]
            wait_gdb: config.wait_gdb,
            snapshot_config: if config.snapshot_interval > 0 || !config.snapshot_at.is_empty() {
                Some(SnapshotConfig {
                    interval: config.snapshot_interval,
                    at_instructions: config.snapshot_at.clone(),
                    output_dir: config
                        .snapshot_dir
                        .as_ref()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| ".".to_string()),
                    prefix: "snapshot".to_string(),
                })
            } else {
                None
            },
            last_snapshot_insn: 0,
            checkpoint_config,
            snapshot_out,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        info!("starting vCPU 0");

        // Wait for GDB connection if --wait-gdb was specified
        #[cfg(feature = "debug")]
        if self.wait_gdb {
            if let Some(ref channels) = self.gdb_channels {
                info!("Waiting for GDB to connect and send command...");
                // Block until we receive a command from GDB
                match channels.cmd_rx.recv() {
                    Ok(cmd) => {
                        info!("GDB connected, received initial command");
                        // Handle the initial command and stop
                        self.gdb_stopped = true;
                        if let Some(should_break) = self.handle_gdb_command(cmd)? {
                            if should_break {
                                return Ok(());
                            }
                        }
                    }
                    Err(_) => {
                        info!("GDB channel closed before connection");
                    }
                }
            }
        }

        // Put the host terminal into raw mode for a faithful interactive serial
        // console (restored on drop, panic, or fatal signal) and start the
        // console mux that reads stdin and forwards bytes to the guest UART,
        // intercepting the Ctrl-A escape prefix for host commands.
        let _raw_tty = RawTty::enable();
        let mut console = Console::spawn();

        // SIGUSR1 requests a checkpoint (the non-keyboard trigger).
        #[cfg(unix)]
        unsafe {
            libc::signal(libc::SIGUSR1, sigusr1_handler as libc::sighandler_t);
        }

        loop {
            // Drain host stdin through the console mux: forward guest bytes to the
            // UART and act on host escape commands. Then drive the serial RX/TX
            // interrupt line through the inline PIC — the SAME path the PIT uses.
            // (The backend's set_irq_line() only pushes to an `irq_pending` vec
            // that nothing drains, so serial IRQs reach the guest only via the
            // inline PIC.) Release the serial lock before taking the PIC lock to
            // keep a consistent lock order.
            {
                let (guest_bytes, actions) = console.poll();
                let mut quit = false;
                for action in actions {
                    match action {
                        ConsoleAction::Help => eprint!("{}", ESCAPE_HELP),
                        ConsoleAction::Quit => {
                            info!("console quit requested (Ctrl-A x)");
                            quit = true;
                        }
                        ConsoleAction::Snapshot => self.console_checkpoint(),
                    }
                }
                if quit {
                    break;
                }
                // SIGUSR1-triggered checkpoint (same effect as Ctrl-A s).
                if CHECKPOINT_SIGNAL.swap(false, std::sync::atomic::Ordering::SeqCst) {
                    self.console_checkpoint();
                }
                if let Some(ref uart) = self.s3c_uart {
                    // ARMv6: queue console input; the vCPU's own IRQ scan
                    // raises the VIC line.
                    if !guest_bytes.is_empty() {
                        if let Ok(mut u) = uart.lock() {
                            u.queue_input(&guest_bytes);
                        }
                    }
                } else if let Some(ref pl011) = self.pl011 {
                    // AArch64: feed the PL011 and drive its SPI line through
                    // the VM's interrupt controller (in-kernel GIC on HVF).
                    let pending = if let Ok(mut uart) = pl011.lock() {
                        if !guest_bytes.is_empty() {
                            debug!(n = guest_bytes.len(), "console input queued to PL011");
                            uart.queue_input(&guest_bytes);
                        }
                        uart.irq_pending()
                    } else {
                        false
                    };
                    if let Some(irq) = self.serial_irq {
                        if let Err(e) = self.vm.set_irq_line(irq, pending) {
                            debug!(irq, pending, error = %e, "failed to drive UART irq line");
                        }
                    }
                } else {
                    let pending = if let Ok(mut serial) = self.serial.lock() {
                        if !guest_bytes.is_empty() {
                            serial.queue_input(&guest_bytes);
                        }
                        serial.has_pending_interrupt()
                    } else {
                        false
                    };
                    if let Some(irq) = self.serial_irq {
                        if let Ok(mut pic) = self.pic.lock() {
                            pic.set_irq(irq as u8, pending);
                        }
                    }
                }
            }

            // Tick the PIT and check for timer interrupts
            if let Ok(mut pit) = self.pit.lock() {
                if pit.tick() {
                    // Timer fired - raise IRQ 0 via the PIC
                    if let Ok(mut pic) = self.pic.lock() {
                        pic.set_irq(0, true);
                    }
                }
            }

            // Tick the LAPIC timer and check for timer interrupts
            {
                let vcpu = self
                    .vcpus
                    .get_mut(0)
                    .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;

                if let Ok(mut lapic) = self.lapic.lock() {
                    if let Some(vector) = lapic.tick() {
                        // LAPIC timer interrupt - inject directly if possible
                        if vcpu.can_inject_interrupt() {
                            if vcpu.inject_interrupt(vector).unwrap_or(false) {
                                lapic.clear_timer_pending();
                            }
                        }
                    } else if lapic.has_pending_timer() && vcpu.can_inject_interrupt() {
                        // Previously pending timer interrupt
                        let vector = lapic.timer_vector();
                        if vcpu.inject_interrupt(vector).unwrap_or(false) {
                            lapic.clear_timer_pending();
                        }
                    }
                }
            }

            // Handle LAPIC IPI delivery
            {
                let vcpu = self
                    .vcpus
                    .get_mut(0)
                    .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;

                if let Ok(mut lapic) = self.lapic.lock() {
                    // Handle pending NMI from self-IPI
                    if lapic.has_pending_nmi() {
                        if vcpu.inject_nmi().unwrap_or(false) {
                            lapic.clear_pending_nmi();
                        }
                    }

                    // Check for pending interrupts in IRR (from self-IPIs)
                    if vcpu.can_inject_interrupt() {
                        if let Some(vector) = lapic.get_pending_vector() {
                            if vcpu.inject_interrupt(vector).unwrap_or(false) {
                                lapic.ack_interrupt(vector);
                            }
                        }
                    }

                    // Handle IPI requests (for multi-CPU, but log for now)
                    if let Some(ipi) = lapic.take_pending_ipi() {
                        match ipi {
                            IpiRequest::Fixed { vector, ref target }
                            | IpiRequest::LowestPriority { vector, ref target } => {
                                // For single-CPU, only AllIncludingSelf matters
                                // (self-targeting is already handled via IRR)
                                match target {
                                    IpiTarget::AllIncludingSelf => {
                                        // Already delivered to self via IRR
                                        debug!(
                                            "IPI Fixed vector {:#x} to all (self already in IRR)",
                                            vector
                                        );
                                    }
                                    IpiTarget::AllExcludingSelf => {
                                        // No other CPUs in single-CPU mode
                                        debug!(
                                            "IPI Fixed vector {:#x} to all-except-self (no other CPUs)",
                                            vector
                                        );
                                    }
                                    _ => {
                                        debug!("IPI Fixed vector {:#x} to {:?}", vector, target);
                                    }
                                }
                            }
                            IpiRequest::Nmi { ref target } => {
                                match target {
                                    IpiTarget::AllIncludingSelf => {
                                        // Self-NMI already pending
                                        debug!("IPI NMI to all (self already pending)");
                                    }
                                    IpiTarget::AllExcludingSelf => {
                                        debug!("IPI NMI to all-except-self (no other CPUs)");
                                    }
                                    _ => {
                                        debug!("IPI NMI to {:?}", target);
                                    }
                                }
                            }
                            IpiRequest::Init { ref target } => {
                                // INIT is used to reset CPUs - primarily for SMP startup
                                debug!("IPI INIT to {:?} (ignored in single-CPU mode)", target);
                            }
                            IpiRequest::Sipi { vector, ref target } => {
                                // SIPI starts AP CPUs - not applicable for single-CPU
                                debug!(
                                    "IPI SIPI vector={:#x} (start addr={:#x}) to {:?} (ignored in single-CPU mode)",
                                    vector,
                                    (vector as u32) * 0x1000,
                                    target
                                );
                            }
                            IpiRequest::Smi { ref target } => {
                                // SMI triggers System Management Mode
                                debug!("IPI SMI to {:?} (SMM not implemented)", target);
                            }
                        }
                    }
                }
            }

            // Check for pending PIC interrupts and inject them
            {
                let vcpu = self
                    .vcpus
                    .get_mut(0)
                    .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;

                let can_inject = vcpu.can_inject_interrupt();
                if let Ok(mut pic) = self.pic.lock() {
                    if pic.has_pending() {
                        if can_inject {
                            if let Some(vector) = pic.get_pending_vector() {
                                let _ = vcpu.inject_interrupt(vector);
                            }
                        }
                    }
                }
            }

            // Handle GDB commands (non-blocking)
            #[cfg(feature = "debug")]
            if let Some(ref channels) = self.gdb_channels {
                match channels.cmd_rx.try_recv() {
                    Ok(cmd) => {
                        if let Some(should_break) = self.handle_gdb_command(cmd)? {
                            if should_break {
                                break;
                            }
                            continue;
                        }
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => {
                        // GDB disconnected, continue without GDB
                        debug!("GDB disconnected");
                    }
                }
            }

            // If GDB stopped us, wait for continue/step command
            #[cfg(feature = "debug")]
            if self.gdb_stopped {
                if let Some(ref channels) = self.gdb_channels {
                    match channels.cmd_rx.recv() {
                        Ok(cmd) => {
                            if let Some(should_break) = self.handle_gdb_command(cmd)? {
                                if should_break {
                                    break;
                                }
                                // Command handled but didn't resume execution, wait for more
                                continue;
                            }
                            // Command returned None = resume execution (step/continue)
                            // Fall through to run the VCPU
                        }
                        Err(_) => {
                            // GDB disconnected
                            self.gdb_stopped = false;
                        }
                    }
                } else {
                    continue;
                }
            }

            let exit = {
                let vcpu = self
                    .vcpus
                    .get_mut(0)
                    .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;
                vcpu.run()?
            };

            // Check if we should take a snapshot (outside vcpu borrow scope)
            if self.snapshot_config.is_some() {
                let insn_count = get_total_instruction_count();
                self.maybe_snapshot(insn_count)?;
            }

            // Re-borrow vcpu for exit handling
            let vcpu = self
                .vcpus
                .get_mut(0)
                .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;

            match exit {
                VcpuExit::Hlt => {
                    // HLT is normal - kernel waits for interrupts
                    continue;
                }
                VcpuExit::Shutdown => {
                    match vcpu.get_state()? {
                        CpuState::X86_64(state) => {
                            let regs = state.regs;
                            let sregs = state.sregs;
                            info!(
                                rip = format!("{:#x}", regs.rip),
                                rsp = format!("{:#x}", regs.rsp),
                                rsi = format!("{:#x}", regs.rsi),
                                rflags = format!("{:#x}", regs.rflags),
                                cr0 = format!("{:#x}", sregs.cr0),
                                cr3 = format!("{:#x}", sregs.cr3),
                                cr4 = format!("{:#x}", sregs.cr4),
                                cs_sel = format!("{:#x}", sregs.cs.selector),
                                cs_base = format!("{:#x}", sregs.cs.base),
                                ds_sel = format!("{:#x}", sregs.ds.selector),
                                gdt_base = format!("{:#x}", sregs.gdt.base),
                                gdt_limit = format!("{:#x}", sregs.gdt.limit),
                                "vCPU shutdown"
                            );
                        }
                        CpuState::Hexagon(state) => {
                            let regs = state.regs;
                            let sp = regs.r[29];
                            info!(
                                pc = format!("{:#x}", regs.pc()),
                                sp = format!("{:#x}", sp),
                                usr = format!("{:#x}", regs.usr()),
                                "vCPU shutdown"
                            );
                        }
                        CpuState::Aarch64(state) => {
                            let regs = state.regs;
                            info!(
                                pc = format!("{:#x}", regs.pc),
                                sp = format!("{:#x}", regs.sp),
                                pstate = format!("{:#x}", regs.pstate),
                                "vCPU shutdown"
                            );
                        }
                        _ => {
                            info!("vCPU shutdown (unsupported architecture)");
                        }
                    }
                    break;
                }
                VcpuExit::IoIn { port, size } => {
                    debug!(port = port, size = size, "PIO read");
                    let is_serial = port >= SERIAL_BASE && port < SERIAL_BASE + 8;
                    let mut data = vec![0u8; size as usize];
                    if is_serial {
                        if let Ok(mut serial) = self.serial.lock() {
                            for (i, byte) in data.iter_mut().enumerate() {
                                *byte = IoDevice::read(&mut *serial, port + i as u16);
                            }
                        }
                    } else if port == 0x1F0 || port == 0x170 {
                        // IDE/ATAPI data register: a single fixed port carries the
                        // whole PIO word/dword (insw/insd). Read every byte from the
                        // SAME port — unlike the byte-lane model the bus uses for
                        // multi-byte ports — so consecutive reads drain the device's
                        // PIO buffer in order.
                        for i in 0..data.len() {
                            self.io_bus.read(port, &mut data[i..i + 1])?;
                        }
                    } else {
                        self.io_bus.read(port, &mut data)?;
                    }
                    vcpu.complete_io_in(&data);
                }
                VcpuExit::IoInString { port, size, count } => {
                    // Batched `rep ins`: read count*size bytes, all from the same
                    // fixed port (the `rep` reads DX repeatedly), in one exit. The
                    // vCPU writes the whole destination block via complete_io_in.
                    let total = (count as usize).saturating_mul(size as usize);
                    let mut data = vec![0u8; total];
                    for i in 0..total {
                        self.io_bus.read(port, &mut data[i..i + 1])?;
                    }
                    vcpu.complete_io_in(&data);
                }
                VcpuExit::IoOut { port, data } => {
                    debug!(port = port, size = data.len(), "PIO write");

                    // ACPI shutdown port (0x604, value 0x2000 = S5 power off)
                    if port == 0x604 && data.len() >= 2 {
                        let val = u16::from_le_bytes([data[0], data[1]]);
                        if val == 0x2000 {
                            info!("ACPI shutdown requested");
                            break;
                        }
                    }

                    let is_serial = port >= SERIAL_BASE && port < SERIAL_BASE + 8;
                    if is_serial {
                        let pending = if let Ok(mut serial) = self.serial.lock() {
                            for (i, byte) in data.iter().enumerate() {
                                IoDevice::write(&mut *serial, port + i as u16, *byte);
                            }
                            serial.has_pending_interrupt()
                        } else {
                            false
                        };
                        // Drive the serial IRQ (TX-empty / line-status) through the
                        // inline PIC, same as the RX path above.
                        if let Some(irq) = self.serial_irq {
                            if let Ok(mut pic) = self.pic.lock() {
                                pic.set_irq(irq as u8, pending);
                            }
                        }
                    } else if port == 0xE9 {
                        // Bochs debug port - output directly
                        for byte in &data {
                            eprint!("{}", *byte as char);
                        }
                    } else if port == 0x1F0 || port == 0x170 {
                        // IDE/ATAPI data register: write every byte of the PIO
                        // word/dword to the SAME fixed port (see the IoIn path).
                        for i in 0..data.len() {
                            self.io_bus.write(port, &data[i..i + 1])?;
                        }
                    } else {
                        self.io_bus.write(port, &data)?;
                    }
                }
                VcpuExit::MmioRead { addr, size } => {
                    let mut data = vec![0u8; size as usize];
                    self.mmio_bus.read(addr, &mut data)?;
                    vcpu.complete_io_in(&data);
                }
                VcpuExit::MmioWrite { addr, data } => {
                    self.mmio_bus.write(addr, &data)?;
                }
                VcpuExit::SystemEvent { .. } => break,
                VcpuExit::FailEntry { reason } => {
                    return Err(Error::KernelLoad(format!(
                        "vCPU fail entry: reason={reason:#x}"
                    )));
                }
                VcpuExit::InternalError => {
                    return Err(Error::KernelLoad("vCPU internal error".to_string()));
                }
                VcpuExit::Debug => {
                    // INT3 breakpoint - kernel debugging, just continue
                    continue;
                }
                VcpuExit::Exception(vector) => {
                    // Check for breakpoint exception (vector 3) when GDB is attached
                    #[cfg(feature = "debug")]
                    if vector == 3 && self.gdb_channels.is_some() {
                        // INT3 was hit - RIP is now AFTER the INT3, back up 1 byte
                        let bp_addr = vcpu.get_regs().map(|r| r.rip).unwrap_or(0) - 1;

                        // Check if this is one of our breakpoints
                        if let Some(orig_byte) = self.gdb_breakpoints.get(&bp_addr) {
                            debug!(addr = format!("{:#x}", bp_addr), "Software breakpoint hit");
                            // Restore original byte so we can re-execute the instruction
                            let mem = self.guest_mem.memory();
                            let _ =
                                mem.write_slice(&[*orig_byte], vm_memory::GuestAddress(bp_addr));
                            debug!(
                                addr = format!("{:#x}", bp_addr),
                                orig = format!("{:#x}", orig_byte),
                                "Restored original byte"
                            );
                            // Invalidate decode cache so the CPU re-reads the instruction
                            vcpu.invalidate_code_cache(bp_addr);

                            // Back up RIP to point at the breakpoint address (so user can re-execute)
                            let mut regs = vcpu.get_regs().unwrap_or_default();
                            regs.rip = bp_addr;
                            let _ = vcpu.set_regs(&regs);
                        } else {
                            // Natural INT3 in the code (not our breakpoint)
                            // Don't back up RIP - leave it past the INT3 so continue works
                            debug!(
                                addr = format!("{:#x}", bp_addr),
                                "Natural INT3 instruction hit"
                            );
                        }

                        // Notify GDB
                        if let Some(ref channels) = self.gdb_channels {
                            let _ = channels.resp_tx.send(GdbResponse::StopReply(5));
                            // SIGTRAP
                        }
                        self.gdb_stopped = true;
                        self.gdb_single_step = false;
                        continue;
                    }
                    // Other exceptions - just continue for now
                    continue;
                }
                #[cfg(feature = "debug")]
                VcpuExit::GdbBreakpoint { addr } => {
                    debug!(addr = format!("{:#x}", addr), "GDB breakpoint hit");
                    if let Some(ref channels) = self.gdb_channels {
                        let _ = channels.resp_tx.send(GdbResponse::StopReply(5));
                        // SIGTRAP
                    }
                    self.gdb_stopped = true;
                    self.gdb_single_step = false;
                    continue;
                }
                #[cfg(feature = "debug")]
                VcpuExit::GdbStep => {
                    debug!("GDB single step complete");

                    // Re-apply all breakpoints (in case we stepped over one)
                    let mem = self.guest_mem.memory();
                    for (addr, _orig) in &self.gdb_breakpoints {
                        let _ = mem.write_slice(&[0xCC], vm_memory::GuestAddress(*addr));
                        // Invalidate decode cache for this breakpoint address
                        vcpu.invalidate_code_cache(*addr);
                    }

                    if let Some(ref channels) = self.gdb_channels {
                        let _ = channels.resp_tx.send(GdbResponse::StopReply(5));
                        // SIGTRAP
                    }
                    self.gdb_stopped = true;
                    self.gdb_single_step = false;
                    continue;
                }
                exit => return Err(Error::KernelLoad(format!("unhandled exit: {exit:?}"))),
            }
        }

        // Flush and close trace file
        #[cfg(feature = "trace")]
        crate::trace::close();

        Ok(())
    }

    pub fn boot_info(&self) -> &BootInfo {
        &self.boot_info
    }

    pub fn guest_mem(&self) -> &GuestMemoryWrapper {
        &self.guest_mem
    }

    pub fn arch(&self) -> &dyn Arch {
        self.arch.as_ref()
    }

    /// Capture all stateful device models into a serializable [`DeviceState`].
    fn capture_devices(&self) -> Result<DeviceState> {
        let lock_err =
            |what: &str| Error::Emulator(format!("device {what} lock poisoned during snapshot"));
        let pic = self.pic.lock().map_err(|_| lock_err("pic"))?.clone();
        let pit = self.pit.lock().map_err(|_| lock_err("pit"))?.clone();
        let serial = self.serial.lock().map_err(|_| lock_err("serial"))?.clone();
        let lapic = self.lapic.lock().map_err(|_| lock_err("lapic"))?.clone();
        Ok(DeviceState {
            pic,
            pit,
            serial,
            lapic,
        })
    }

    /// Take a full, self-contained checkpoint of the current VM state.
    pub fn take_snapshot(&self, insn_count: u64) -> Result<Snapshot> {
        let vcpu = self
            .vcpus
            .get(0)
            .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;

        let cpu_state = vcpu.get_state()?;
        let emulator_state = vcpu.get_emulator_state().unwrap_or_default();
        let devices = self.capture_devices()?;
        let memory = self.guest_mem.read_all();

        Snapshot::new(
            self.checkpoint_config.clone(),
            insn_count,
            crate::timing::elapsed_nanos(),
            cpu_state,
            emulator_state,
            devices,
            &memory,
        )
    }

    /// Write a checkpoint to the configured `--snapshot-out` path (default
    /// `./checkpoint.rxc`). Invoked from the `Ctrl-A s` console hotkey and the
    /// SIGUSR1 signal. The machine is effectively paused while this runs because
    /// the run loop is single-threaded — the vCPU is not executing during the
    /// poll phase where this is called.
    pub fn console_checkpoint(&self) {
        let insn = get_total_instruction_count();
        let path = self.snapshot_out.clone();
        info!(path = %path.display(), insn = insn, "checkpoint requested");
        match self.take_snapshot(insn) {
            Ok(snapshot) => match snapshot.save(&path) {
                Ok(()) => {
                    // \r\n so the message renders cleanly in raw terminal mode.
                    eprint!("\r\n[rax] checkpoint written: {}\r\n", path.display());
                    info!(summary = %snapshot.summary(), "checkpoint written");
                }
                Err(e) => eprint!("\r\n[rax] checkpoint save failed: {}\r\n", e),
            },
            Err(e) => eprint!("\r\n[rax] checkpoint capture failed: {}\r\n", e),
        }
    }

    /// Check if we should take a snapshot and do so if needed
    fn maybe_snapshot(&mut self, insn_count: u64) -> Result<()> {
        let should_snapshot = if let Some(ref config) = self.snapshot_config {
            // Debug: print instruction count periodically
            if insn_count % 1_000_000_000 == 0 {
                eprintln!(
                    "[SNAPSHOT] insn_count={} interval={} at={:?}",
                    insn_count, config.interval, config.at_instructions
                );
            }
            // Check interval-based snapshotting
            if config.interval > 0 && insn_count >= self.last_snapshot_insn + config.interval {
                true
            } else {
                // Check if we passed any specific instruction counts
                config
                    .at_instructions
                    .iter()
                    .any(|&at| at > self.last_snapshot_insn && at <= insn_count)
            }
        } else {
            false
        };

        if should_snapshot {
            let snapshot = self.take_snapshot(insn_count)?;
            let filename = self
                .snapshot_config
                .as_ref()
                .map(|c| c.filename(insn_count))
                .unwrap_or_else(|| format!("snapshot_{:012}.snap", insn_count));

            info!(
                filename = %filename,
                insn_count = insn_count,
                summary = %snapshot.summary(),
                "saving snapshot"
            );

            snapshot.save(&filename)?;
            self.last_snapshot_insn = insn_count;
        }

        Ok(())
    }

    /// Restore VM state from a checkpoint: registers + emulator state, all
    /// writable guest RAM, and every device model.
    pub fn restore_snapshot(&mut self, snapshot: &Snapshot) -> Result<()> {
        // Restore CPU state
        let vcpu = self
            .vcpus
            .get_mut(0)
            .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;
        vcpu.set_state(&snapshot.cpu_state)?;
        vcpu.set_emulator_state(&snapshot.emulator_state)?;

        // Re-anchor the clock so the real-time TSC and restored device timers
        // continue from the checkpoint instead of jumping backward to ~0.
        crate::timing::set_resume_base(snapshot.elapsed_nanos);

        // Decompress and restore guest RAM (this also restores the kernel/initrd
        // images that were loaded into it — read-only regions need not be
        // re-loaded from disk).
        let memory = snapshot.decompress_memory()?;
        self.guest_mem.write_all(&memory)?;

        // Restore device models. The Vmm and the I/O/MMIO buses share the same
        // Arc<Mutex<_>> for each device, so assigning through the lock updates
        // both views.
        let lock_err =
            |what: &str| Error::Emulator(format!("device {what} lock poisoned during restore"));
        *self.pic.lock().map_err(|_| lock_err("pic"))? = snapshot.devices.pic.clone();
        *self.pit.lock().map_err(|_| lock_err("pit"))? = snapshot.devices.pit.clone();
        *self.serial.lock().map_err(|_| lock_err("serial"))? = snapshot.devices.serial.clone();
        *self.lapic.lock().map_err(|_| lock_err("lapic"))? = snapshot.devices.lapic.clone();

        info!(
            insn_count = snapshot.instruction_count,
            summary = %snapshot.summary(),
            "restored checkpoint"
        );

        Ok(())
    }

    /// Handle a GDB command from the debug server.
    /// Returns Some(true) to break the run loop, Some(false) to continue, None for commands that resume execution.
    #[cfg(feature = "debug")]
    fn handle_gdb_command(&mut self, cmd: GdbCommand) -> Result<Option<bool>> {
        use crate::gdb::protocol::encode_hex;
        use crate::gdb::registers::{pack_registers, unpack_registers};

        let channels = match &self.gdb_channels {
            Some(ch) => ch,
            None => return Ok(None),
        };

        match cmd {
            GdbCommand::Continue => {
                self.gdb_stopped = false;
                self.gdb_single_step = false;
                // Disable single-step mode on vcpu
                if let Some(vcpu) = self.vcpus.get_mut(0) {
                    vcpu.set_single_step(false);
                }
                return Ok(None); // Resume execution
            }
            GdbCommand::Step => {
                self.gdb_stopped = false;
                self.gdb_single_step = true;
                // Enable single-step mode on vcpu
                if let Some(vcpu) = self.vcpus.get_mut(0) {
                    vcpu.set_single_step(true);
                }
                return Ok(None); // Resume execution for one instruction
            }
            GdbCommand::Interrupt => {
                // Debugger requested pause (Ctrl+C / suspend)
                self.gdb_stopped = true;
                self.gdb_single_step = false;
                if let Some(vcpu) = self.vcpus.get_mut(0) {
                    vcpu.set_single_step(false);
                }
                let _ = channels.resp_tx.send(GdbResponse::StopReply(2)); // SIGINT
            }
            GdbCommand::ReadRegisters => {
                let vcpu = self
                    .vcpus
                    .get(0)
                    .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;
                match vcpu.get_state()? {
                    CpuState::X86_64(state) => {
                        let hex = pack_registers(&state);
                        let _ = channels.resp_tx.send(GdbResponse::Registers(hex));
                    }
                    _ => {
                        let _ = channels.resp_tx.send(GdbResponse::Error(1));
                    }
                }
            }
            GdbCommand::WriteRegisters(data) => {
                let vcpu = self
                    .vcpus
                    .get_mut(0)
                    .ok_or_else(|| Error::InvalidConfig("no vcpu available".to_string()))?;
                match vcpu.get_state()? {
                    CpuState::X86_64(mut state) => {
                        let hex = String::from_utf8_lossy(&data).to_string();
                        if unpack_registers(&hex, &mut state) {
                            vcpu.set_state(&CpuState::X86_64(state))?;
                            let _ = channels.resp_tx.send(GdbResponse::Ok);
                        } else {
                            let _ = channels.resp_tx.send(GdbResponse::Error(1));
                        }
                    }
                    _ => {
                        let _ = channels.resp_tx.send(GdbResponse::Error(1));
                    }
                }
            }
            GdbCommand::ReadMemory { addr, len } => {
                let mem = self.guest_mem.memory();
                let mut data = vec![0u8; len];
                if mem
                    .read_slice(&mut data, vm_memory::GuestAddress(addr))
                    .is_ok()
                {
                    let hex = encode_hex(&data);
                    let _ = channels.resp_tx.send(GdbResponse::Memory(hex));
                } else {
                    let _ = channels.resp_tx.send(GdbResponse::Error(1));
                }
            }
            GdbCommand::WriteMemory { addr, data } => {
                let mem = self.guest_mem.memory();
                if mem
                    .write_slice(&data, vm_memory::GuestAddress(addr))
                    .is_ok()
                {
                    let _ = channels.resp_tx.send(GdbResponse::Ok);
                } else {
                    let _ = channels.resp_tx.send(GdbResponse::Error(1));
                }
            }
            GdbCommand::SetBreakpoint { addr } => {
                // Read original byte and patch with INT3 (0xCC)
                let mem = self.guest_mem.memory();
                let mut orig = [0u8; 1];
                if mem
                    .read_slice(&mut orig, vm_memory::GuestAddress(addr))
                    .is_ok()
                {
                    if mem
                        .write_slice(&[0xCC], vm_memory::GuestAddress(addr))
                        .is_ok()
                    {
                        // Store original byte for later restoration
                        self.gdb_breakpoints.insert(addr, orig[0]);
                        debug!(
                            addr = format!("{:#x}", addr),
                            orig = format!("{:#x}", orig[0]),
                            "Set breakpoint"
                        );
                        // Invalidate decode cache so CPU re-reads the instruction
                        if let Some(vcpu) = self.vcpus.get_mut(0) {
                            vcpu.invalidate_code_cache(addr);
                        }
                        let _ = channels.resp_tx.send(GdbResponse::Ok);
                    } else {
                        let _ = channels.resp_tx.send(GdbResponse::Error(1));
                    }
                } else {
                    let _ = channels.resp_tx.send(GdbResponse::Error(1));
                }
            }
            GdbCommand::RemoveBreakpoint { addr } => {
                // Restore original byte from tracking map
                let mem = self.guest_mem.memory();
                if let Some(orig) = self.gdb_breakpoints.remove(&addr) {
                    if mem
                        .write_slice(&[orig], vm_memory::GuestAddress(addr))
                        .is_ok()
                    {
                        debug!(
                            addr = format!("{:#x}", addr),
                            orig = format!("{:#x}", orig),
                            "Removed breakpoint"
                        );
                        // Invalidate decode cache so CPU re-reads the instruction
                        if let Some(vcpu) = self.vcpus.get_mut(0) {
                            vcpu.invalidate_code_cache(addr);
                        }
                        let _ = channels.resp_tx.send(GdbResponse::Ok);
                    } else {
                        // Failed to write, put it back in the map
                        self.gdb_breakpoints.insert(addr, orig);
                        let _ = channels.resp_tx.send(GdbResponse::Error(1));
                    }
                } else {
                    // No breakpoint at this address
                    debug!(addr = format!("{:#x}", addr), "No breakpoint to remove");
                    let _ = channels.resp_tx.send(GdbResponse::Ok);
                }
            }
            GdbCommand::QueryHaltReason => {
                let _ = channels.resp_tx.send(GdbResponse::StopReply(5)); // SIGTRAP
            }
            GdbCommand::Detach => {
                self.gdb_stopped = false;
                self.gdb_single_step = false;
                let _ = channels.resp_tx.send(GdbResponse::Detached);
                return Ok(Some(false));
            }
            GdbCommand::Kill => {
                return Ok(Some(true)); // Break run loop
            }
        }

        Ok(Some(false)) // Don't break, but don't resume execution either
    }
}
