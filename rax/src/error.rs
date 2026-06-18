use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(all(feature = "kvm", target_os = "linux"))]
    #[error("KVM error: {0}")]
    Kvm(#[from] kvm_ioctls::Error),
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    #[error("Linux loader error: {0}")]
    LinuxLoader(#[from] linux_loader::loader::Error),
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    #[error("Linux boot config error: {0}")]
    LinuxBoot(#[from] linux_loader::configurator::Error),
    #[error("Guest memory error: {0}")]
    GuestMemory(#[from] vm_memory::GuestMemoryError),
    #[error("Guest memory creation error: {0}")]
    GuestMemoryCreate(#[from] vm_memory::mmap::FromRangesError),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Device port range overlaps existing device: base=0x{base:x}, len=0x{len:x}")]
    DeviceOverlap { base: u16, len: u16 },
    #[error("Device MMIO range overlaps existing device: base=0x{base:x}, len=0x{len:x}")]
    MmioOverlap { base: u64, len: u64 },
    #[error("No device mapped for port=0x{port:x}, size={size}")]
    DeviceNotFound { port: u16, size: u8 },
    #[error("Kernel load error: {0}")]
    KernelLoad(String),
    #[error("Emulator error: {0}")]
    Emulator(String),
    #[error("Page fault at vaddr {vaddr:#x} (error_code={error_code:#x})")]
    PageFault { vaddr: u64, error_code: u64 },
}

pub type Result<T> = std::result::Result<T, Error>;
