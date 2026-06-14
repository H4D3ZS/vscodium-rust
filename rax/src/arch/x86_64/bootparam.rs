//! Local boot parameter definitions for cross-platform x86_64 emulation.
//!
//! These structures match the Linux kernel boot protocol and are used
//! when the `linux-loader` crate's x86 modules are not available on the host.

use vm_memory::{Address, Bytes, GuestAddress, GuestMemory, GuestMemoryMmap};

/// E820 memory type: usable RAM
pub const E820_RAM: u32 = 1;
/// E820 memory type: reserved
pub const E820_RESERVED: u32 = 2;

/// E820 memory map entry (matches Linux kernel struct boot_e820_entry)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BootE820Entry {
    pub addr: u64,
    pub size: u64,
    pub type_: u32,
}

/// Screen info structure (matches Linux kernel struct screen_info)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ScreenInfo {
    pub orig_x: u8,
    pub orig_y: u8,
    pub ext_mem_k: u16,
    pub orig_video_page: u16,
    pub orig_video_mode: u8,
    pub orig_video_cols: u8,
    pub flags: u8,
    pub unused2: u8,
    pub orig_video_ega_bx: u16,
    pub unused3: u16,
    pub orig_video_lines: u8,
    pub orig_video_isVGA: u8,
    pub orig_video_points: u16,
    // Additional fields truncated - we only use the ones above
    // Linux screen_info is 0x40 bytes. Fields above total 18 bytes (0x12),
    // so we need 46 bytes of padding to reach 0x40.
    pub _pad: [u8; 46], // Padding to reach offset 0x40
}

impl Default for ScreenInfo {
    fn default() -> Self {
        // Safety: ScreenInfo is repr(C, packed) with all-zero being valid
        unsafe { core::mem::zeroed() }
    }
}

/// Setup header (matches Linux kernel struct setup_header at offset 0x1f1)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SetupHeader {
    pub setup_sects: u8,
    pub root_flags: u16,
    pub syssize: u32,
    pub ram_size: u16,
    pub vid_mode: u16,
    pub root_dev: u16,
    pub boot_flag: u16,
    pub jump: u16,
    pub header: u32, // "HdrS" magic
    pub version: u16,
    pub realmode_swtch: u32,
    pub start_sys_seg: u16,
    pub kernel_version: u16,
    pub type_of_loader: u8,
    pub loadflags: u8,
    pub setup_move_size: u16,
    pub code32_start: u32,
    pub ramdisk_image: u32,
    pub ramdisk_size: u32,
    pub bootsect_kludge: u32,
    pub heap_end_ptr: u16,
    pub ext_loader_ver: u8,
    pub ext_loader_type: u8,
    pub cmd_line_ptr: u32,
    pub initrd_addr_max: u32,
    pub kernel_alignment: u32,
    pub relocatable_kernel: u8,
    pub min_alignment: u8,
    pub xloadflags: u16,
    pub cmdline_size: u32,
    pub hardware_subarch: u32,
    pub hardware_subarch_data: u64,
    pub payload_offset: u32,
    pub payload_length: u32,
    pub setup_data: u64,
    pub pref_address: u64,
    pub init_size: u32,
    pub handover_offset: u32,
    pub kernel_info_offset: u32,
}

/// Boot parameters structure (matches Linux kernel struct boot_params)
/// Layout based on Linux kernel's arch/x86/include/uapi/asm/bootparam.h
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BootParams {
    pub screen_info: ScreenInfo,    // Offset 0x000, size 0x40
    pub _pad1: [u8; 0x1e8 - 0x40],  // Padding from 0x40 to 0x1e8
    pub e820_entries: u8,           // Offset 0x1e8
    pub _pad2: [u8; 0x1f1 - 0x1e9], // Padding from 0x1e9 to 0x1f1
    pub hdr: SetupHeader,           // Offset 0x1f1
    pub _pad3: [u8; 0x2d0 - (0x1f1 + core::mem::size_of::<SetupHeader>())],
    pub e820_table: [BootE820Entry; 128], // Offset 0x2d0
}

impl Default for BootParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

impl core::fmt::Debug for BootParams {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BootParams")
            .field("e820_entries", &self.e820_entries)
            .finish()
    }
}

/// Result from kernel loading
#[derive(Debug)]
pub struct KernelLoaderResult {
    pub kernel_load: GuestAddress,
    pub kernel_end: u64,
    pub setup_header: Option<SetupHeader>,
    pub pvh_boot_cap: PvhBootCapability,
}

/// PVH boot capability (not used for standard Linux boot)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PvhBootCapability {
    PvhEntryNotPresent,
}

/// Load command line to guest memory
pub fn load_cmdline(
    mem: &GuestMemoryMmap,
    addr: GuestAddress,
    cmdline: &str,
) -> Result<(), crate::error::Error> {
    let bytes = cmdline.as_bytes();
    mem.write_slice(bytes, addr)?;
    // Write null terminator
    mem.write_obj(0u8, addr.checked_add(bytes.len() as u64).unwrap())?;
    Ok(())
}

/// Write boot parameters to guest memory
pub fn write_boot_params(
    mem: &GuestMemoryMmap,
    params: &BootParams,
    addr: GuestAddress,
) -> Result<(), crate::error::Error> {
    let bytes = unsafe {
        core::slice::from_raw_parts(
            params as *const BootParams as *const u8,
            core::mem::size_of::<BootParams>(),
        )
    };
    mem.write_slice(bytes, addr)?;
    Ok(())
}

/// Load a bzImage kernel from bytes
pub fn load_bzimage_from_bytes(
    mem: &GuestMemoryMmap,
    data: &[u8],
    kernel_start: GuestAddress,
) -> Result<KernelLoaderResult, crate::error::Error> {
    if data.len() < 0x250 {
        return Err(crate::error::Error::KernelLoad("kernel too small".into()));
    }

    // Check for bzImage magic "HdrS" at offset 0x202
    if &data[0x202..0x206] != b"HdrS" {
        return Err(crate::error::Error::KernelLoad("not a bzImage".into()));
    }

    // Parse setup header
    let setup_sects = if data[0x1f1] == 0 {
        4
    } else {
        data[0x1f1] as usize
    };
    let setup_size = (setup_sects + 1) * 512;

    if data.len() < setup_size {
        return Err(crate::error::Error::KernelLoad("kernel truncated".into()));
    }

    // Extract setup header (starts at offset 0x1f1)
    let hdr_bytes = &data[0x1f1..0x1f1 + core::mem::size_of::<SetupHeader>()];
    let setup_header: SetupHeader =
        unsafe { std::ptr::read_unaligned(hdr_bytes.as_ptr() as *const SetupHeader) };

    // Load protected mode kernel (after setup)
    let kernel_data = &data[setup_size..];
    mem.write_slice(kernel_data, kernel_start)?;

    Ok(KernelLoaderResult {
        kernel_load: kernel_start,
        kernel_end: kernel_start.raw_value() + kernel_data.len() as u64,
        setup_header: Some(setup_header),
        pvh_boot_cap: PvhBootCapability::PvhEntryNotPresent,
    })
}

/// Load a bzImage kernel from a file path
pub fn load_bzimage(
    mem: &GuestMemoryMmap,
    kernel_path: &std::path::Path,
    kernel_start: GuestAddress,
) -> Result<KernelLoaderResult, crate::error::Error> {
    let data = std::fs::read(kernel_path)?;
    load_bzimage_from_bytes(mem, &data, kernel_start)
}
