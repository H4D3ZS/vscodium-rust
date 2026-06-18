use crate::error::Result;
#[cfg(all(feature = "kvm", target_os = "linux"))]
use kvm_bindings::kvm_userspace_memory_region;
#[cfg(all(feature = "kvm", target_os = "linux"))]
use kvm_ioctls::VmFd;
use tracing::info;
#[cfg(all(feature = "kvm", target_os = "linux"))]
use vm_memory::{Address, GuestAddress, GuestMemory, GuestMemoryMmap, GuestMemoryRegion};
#[cfg(not(all(feature = "kvm", target_os = "linux")))]
use vm_memory::{GuestAddress, GuestMemoryMmap};

pub const PAGE_SIZE: u64 = 4096;

/// Start of the gap for KVM's TSS and identity map (must not be registered as guest memory)
/// This is in the MMIO gap below 4GB. KVM needs space for TSS (3 pages) + identity map pages.
/// We leave a generous gap of 1MB to be safe. Located at 3GB mark to avoid any MMIO conflicts.
pub const KVM_TSS_IDENTITY_GAP_START: u64 = 0xC0000000; // 3GB
/// End of the gap (exclusive) - 1MB gap
pub const KVM_TSS_IDENTITY_GAP_END: u64 = 0xC0100000; // 3GB + 1MB

pub fn align_up(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    let mask = align - 1;
    (value + mask) & !mask
}

pub fn align_down(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    value & !(align - 1)
}

pub struct GuestMemoryWrapper {
    mem: GuestMemoryMmap,
    /// The size reported to the guest via e820
    reported_size: u64,
    /// The actual allocated size (includes padding for per-CPU overflow)
    actual_size: u64,
}

impl GuestMemoryWrapper {
    pub fn new(size_bytes: u64) -> Result<Self> {
        let reported_size = align_up(size_bytes, PAGE_SIZE);

        // Add extra padding for per-CPU allocation that may end up slightly past
        // the reported memory size. The Linux kernel's per-CPU allocator sometimes
        // computes addresses that land just past the end of RAM. By allocating
        // extra physical memory (but not reporting it to the kernel via e820),
        // these accesses will succeed.
        //
        // Observed offsets past RAM:
        // - 64GB guest: ~200MB past end
        // - 32GB guest: ~1.5GB past end
        // - 1GB guest: ~2GB past end
        // Use 4GB padding as a safe margin.
        let padding = 4 * 1024 * 1024 * 1024u64; // 4GB
        let actual_size = align_up(reported_size + padding, PAGE_SIZE);

        info!(
            reported_bytes = reported_size,
            actual_bytes = actual_size,
            padding_bytes = padding,
            "allocating guest memory with per-CPU padding"
        );
        let mem = GuestMemoryMmap::from_ranges(&[(GuestAddress(0), actual_size as usize)])?;
        Ok(GuestMemoryWrapper {
            mem,
            reported_size,
            actual_size,
        })
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    pub fn register(&self, vm_fd: &VmFd) -> Result<()> {
        use tracing::debug;

        // Register memory up to (but not past) the KVM TSS/identity gap.
        // KVM's vCPU creation fails with EEXIST if any memory slot is registered
        // at addresses >= the TSS/identity map address. We leave a gap for KVM's
        // internal structures and do NOT register memory above this gap.
        //
        // The backing memory above the gap is still allocated (via mmap), which
        // allows the host to handle any overflow accesses if needed, but KVM
        // won't try to map it as guest RAM.
        let mut slot = 0u32;

        for region in self.mem.iter() {
            let region_start = region.start_addr().raw_value();
            let region_end = region_start + region.len();
            let host_base = region.as_ptr() as u64;

            // Only register memory up to the gap - do NOT register anything at or above it
            let effective_end = region_end.min(KVM_TSS_IDENTITY_GAP_START);

            if region_start >= effective_end {
                // This entire region is at or above the gap, skip it
                debug!(
                    region_start = format!("{:#x}", region_start),
                    region_end = format!("{:#x}", region_end),
                    gap_start = format!("{:#x}", KVM_TSS_IDENTITY_GAP_START),
                    "skipping memory region at/above TSS gap"
                );
                continue;
            }

            let size = effective_end - region_start;
            let host_addr = host_base;

            info!(
                slot,
                guest_start = format!("{:#x}", region_start),
                guest_end = format!("{:#x}", effective_end),
                size = size,
                "registering guest memory"
            );

            let mem_region = kvm_userspace_memory_region {
                slot,
                guest_phys_addr: region_start,
                memory_size: size,
                userspace_addr: host_addr,
                flags: 0,
            };
            debug!(
                slot,
                guest_addr = format!("{:#x}", region_start),
                "calling set_user_memory_region"
            );
            unsafe { vm_fd.set_user_memory_region(mem_region)? };
            debug!(slot, "set_user_memory_region succeeded");

            slot += 1;
        }
        Ok(())
    }

    pub fn memory(&self) -> &GuestMemoryMmap {
        &self.mem
    }

    /// Returns the size reported to the guest via e820.
    /// This is less than the actual allocated size (which includes padding).
    pub fn size(&self) -> u64 {
        self.reported_size
    }

    /// Returns the actual allocated size including padding.
    #[allow(dead_code)]
    pub fn actual_size(&self) -> u64 {
        self.actual_size
    }

    /// Read all memory contents into a byte vector (for snapshotting).
    /// Returns the reported_size worth of memory (excludes padding).
    pub fn read_all(&self) -> Vec<u8> {
        use vm_memory::Bytes;
        let mut buf = vec![0u8; self.reported_size as usize];
        let _ = self.mem.read_slice(&mut buf, GuestAddress(0));
        buf
    }

    /// Write memory contents from a byte vector (for snapshot restore).
    pub fn write_all(&self, data: &[u8]) -> Result<()> {
        use vm_memory::Bytes;
        let size = data.len().min(self.reported_size as usize);
        self.mem
            .write_slice(&data[..size], GuestAddress(0))
            .map_err(|e| crate::error::Error::Emulator(format!("Failed to write memory: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn align_up_down() {
        assert_eq!(align_up(1, 4096), 4096);
        assert_eq!(align_down(4097, 4096), 4096);
    }
}
