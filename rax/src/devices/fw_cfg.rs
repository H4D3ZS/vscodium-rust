//! QEMU-compatible fw_cfg device.
//!
//! The fw_cfg device is QEMU's firmware configuration interface, used to pass
//! configuration data, firmware images, ACPI tables, etc. from the hypervisor
//! to the guest firmware (SeaBIOS, OVMF, etc.).
//!
//! This implementation exposes the traditional I/O port interface:
//! - Port 0x510: selector register (16-bit write selects an item key and
//!   resets the read offset for that item).
//! - Port 0x511: data register (8-bit; sequential reads return successive
//!   bytes of the currently selected item and advance an internal offset).
//!
//! Reads past the end of the selected item return 0.
//!
//! Well-known items:
//! - 0x0000 SIGNATURE: the ASCII bytes b"QEMU".
//! - 0x0001 ID: a little-endian u32 feature bitmap.
//! - 0x0019 FILE_DIR: the file directory (see `FILE_DIR` layout below).
//!
//! In addition to the well-known items, a generic key -> bytes store is
//! provided via [`FwCfg::add_bytes`] and a file registry via
//! [`FwCfg::add_file`] which auto-assigns selectors >= 0x20 and records the
//! entry in the file directory so firmware/ACPI blobs can be exposed later.

use std::collections::BTreeMap;

use crate::devices::bus::IoDevice;

/// Selector (control) port: a 16-bit write selects an item and resets offset.
pub const FW_CFG_PORT_SEL: u16 = 0x510;
/// Data port: 8-bit sequential reads return bytes of the selected item.
pub const FW_CFG_PORT_DATA: u16 = 0x511;

/// Signature item key. Reads back as the ASCII bytes b"QEMU".
pub const FW_CFG_SIGNATURE: u16 = 0x0000;
/// ID item key. Reads back as a little-endian u32 feature bitmap.
pub const FW_CFG_ID: u16 = 0x0001;
/// File directory item key.
pub const FW_CFG_FILE_DIR: u16 = 0x0019;

/// First selector value handed out to files via `add_file`.
pub const FW_CFG_FILE_FIRST: u16 = 0x0020;

/// Feature bit: traditional (non-DMA) interface is supported.
pub const FW_CFG_FEATURE_TRADITIONAL: u32 = 0x1;

/// Maximum length of a fw_cfg file name (including the trailing NUL slot).
const FW_CFG_FILE_NAME_LEN: usize = 56;

/// A single entry in the fw_cfg file directory.
#[derive(Clone, Debug)]
struct FileEntry {
    /// Selector key the firmware uses to read the file's contents.
    select: u16,
    /// Size of the file in bytes.
    size: u32,
    /// NUL-terminated, NUL-padded file name.
    name: [u8; FW_CFG_FILE_NAME_LEN],
}

/// QEMU-compatible fw_cfg device.
pub struct FwCfg {
    /// Feature bitmap returned for the ID item.
    id: u32,
    /// Currently selected item key.
    selector: u16,
    /// Read offset into the currently selected item.
    offset: usize,
    /// Generic key -> raw bytes store (used for both well-known and custom keys).
    items: BTreeMap<u16, Vec<u8>>,
    /// File directory entries, in insertion order.
    files: Vec<FileEntry>,
    /// Next selector value to hand out to `add_file`.
    next_file_selector: u16,
}

impl FwCfg {
    /// Create a new fw_cfg device pre-populated with the well-known
    /// SIGNATURE and ID items.
    pub fn new() -> Self {
        let mut dev = FwCfg {
            id: FW_CFG_FEATURE_TRADITIONAL,
            selector: 0,
            offset: 0,
            items: BTreeMap::new(),
            files: Vec::new(),
            next_file_selector: FW_CFG_FILE_FIRST,
        };

        // SIGNATURE: ASCII "QEMU".
        dev.items.insert(FW_CFG_SIGNATURE, b"QEMU".to_vec());
        // ID: little-endian u32 feature bitmap.
        dev.items.insert(FW_CFG_ID, dev.id.to_le_bytes().to_vec());
        // FILE_DIR: starts empty (count == 0).
        dev.rebuild_file_dir();

        dev
    }

    /// Register raw bytes under an arbitrary selector key.
    ///
    /// This overwrites any existing item with the same key.
    pub fn add_bytes(&mut self, key: u16, data: Vec<u8>) {
        self.items.insert(key, data);
    }

    /// Register a named file blob, auto-assigning a selector >= 0x20 and
    /// recording it in the file directory. Returns the assigned selector.
    pub fn add_file(&mut self, name: &str, data: Vec<u8>) -> u16 {
        let select = self.next_file_selector;
        self.next_file_selector = self
            .next_file_selector
            .checked_add(1)
            .expect("fw_cfg: ran out of file selectors");

        let size = data.len() as u32;

        // Build the fixed-size, NUL-padded name field.
        let mut name_buf = [0u8; FW_CFG_FILE_NAME_LEN];
        let name_bytes = name.as_bytes();
        // Leave room for at least an implicit terminator: copy up to len-1.
        let copy_len = name_bytes.len().min(FW_CFG_FILE_NAME_LEN - 1);
        name_buf[..copy_len].copy_from_slice(&name_bytes[..copy_len]);

        self.items.insert(select, data);
        self.files.push(FileEntry {
            select,
            size,
            name: name_buf,
        });
        self.rebuild_file_dir();

        select
    }

    /// Rebuild the FILE_DIR item from the current set of files.
    ///
    /// Layout (all multi-byte integers big-endian):
    ///   u32 count
    ///   count * {
    ///       u32 size
    ///       u16 select
    ///       u16 reserved (0)
    ///       u8  name[56]
    ///   }
    fn rebuild_file_dir(&mut self) {
        let mut dir = Vec::with_capacity(4 + self.files.len() * (4 + 2 + 2 + FW_CFG_FILE_NAME_LEN));
        dir.extend_from_slice(&(self.files.len() as u32).to_be_bytes());
        for entry in &self.files {
            dir.extend_from_slice(&entry.size.to_be_bytes());
            dir.extend_from_slice(&entry.select.to_be_bytes());
            dir.extend_from_slice(&0u16.to_be_bytes()); // reserved
            dir.extend_from_slice(&entry.name);
        }
        self.items.insert(FW_CFG_FILE_DIR, dir);
    }

    /// Select an item key and reset the read offset.
    fn select(&mut self, key: u16) {
        self.selector = key;
        self.offset = 0;
    }

    /// Read the next byte from the currently selected item, advancing the
    /// internal offset. Reads past the end of the item (or of an unknown
    /// item) return 0.
    fn read_data(&mut self) -> u8 {
        let byte = self
            .items
            .get(&self.selector)
            .and_then(|data| data.get(self.offset).copied())
            .unwrap_or(0);
        self.offset = self.offset.saturating_add(1);
        byte
    }
}

impl Default for FwCfg {
    fn default() -> Self {
        Self::new()
    }
}

impl IoDevice for FwCfg {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            FW_CFG_PORT_DATA => self.read_data(),
            // The selector register is write-only on real hardware; reads are
            // not meaningful, return 0.
            _ => 0,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        // The selector is a 16-bit register at 0x510. The IoBus dispatches
        // byte-by-byte (low byte to 0x510, high byte to 0x511), so accept the
        // low byte at 0x510 and the high byte at 0x511 to reconstruct the
        // 16-bit selector while still keeping 0x511 usable for byte reads.
        match port {
            FW_CFG_PORT_SEL => {
                // Low byte of the selector; resets offset and selects.
                self.select((self.selector & 0xff00) | value as u16);
            }
            FW_CFG_PORT_DATA => {
                // High byte of the selector. Re-select to keep offset reset.
                self.select((self.selector & 0x00ff) | ((value as u16) << 8));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::bus::IoDevice;

    /// Write a 16-bit selector value the way the guest would (16-bit OUT to
    /// 0x510), which the byte-oriented bus splits into low byte at 0x510 and
    /// high byte at 0x511.
    fn select16(dev: &mut FwCfg, key: u16) {
        IoDevice::write(dev, FW_CFG_PORT_SEL, (key & 0xff) as u8);
        IoDevice::write(dev, FW_CFG_PORT_DATA, (key >> 8) as u8);
    }

    fn read_item(dev: &mut FwCfg, len: usize) -> Vec<u8> {
        (0..len)
            .map(|_| IoDevice::read(dev, FW_CFG_PORT_DATA))
            .collect()
    }

    #[test]
    fn signature_reads_qemu_byte_by_byte() {
        let mut dev = FwCfg::new();
        select16(&mut dev, FW_CFG_SIGNATURE);
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), b'Q');
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), b'E');
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), b'M');
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), b'U');
        // Reads past the end return 0.
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), 0);
    }

    #[test]
    fn id_reads_feature_bitmap_le() {
        let mut dev = FwCfg::new();
        select16(&mut dev, FW_CFG_ID);
        let bytes = read_item(&mut dev, 4);
        let id = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        assert_eq!(id, FW_CFG_FEATURE_TRADITIONAL);
    }

    #[test]
    fn selector_write_resets_offset() {
        let mut dev = FwCfg::new();
        select16(&mut dev, FW_CFG_SIGNATURE);
        // Consume two bytes.
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), b'Q');
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), b'E');
        // Re-selecting the same item resets the offset to the start.
        select16(&mut dev, FW_CFG_SIGNATURE);
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), b'Q');
    }

    #[test]
    fn add_bytes_round_trip_via_selector_and_data() {
        let mut dev = FwCfg::new();
        let key = 0x4242u16;
        let payload = vec![0xde, 0xad, 0xbe, 0xef];
        dev.add_bytes(key, payload.clone());

        select16(&mut dev, key);
        let read_back = read_item(&mut dev, payload.len());
        assert_eq!(read_back, payload);
        // Reads past the end return 0.
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), 0);
    }

    #[test]
    fn add_file_appears_in_directory_with_be_size_and_select() {
        let mut dev = FwCfg::new();
        let data = vec![1u8, 2, 3, 4, 5];
        let name = "etc/acpi/tables";
        let select = dev.add_file(name, data.clone());
        assert!(select >= FW_CFG_FILE_FIRST);

        // The file's contents are readable via its assigned selector.
        select16(&mut dev, select);
        assert_eq!(read_item(&mut dev, data.len()), data);

        // The directory reports the file with big-endian size/select.
        select16(&mut dev, FW_CFG_FILE_DIR);
        let count_bytes = read_item(&mut dev, 4);
        let count = u32::from_be_bytes([
            count_bytes[0],
            count_bytes[1],
            count_bytes[2],
            count_bytes[3],
        ]);
        assert_eq!(count, 1);

        let size_bytes = read_item(&mut dev, 4);
        let size = u32::from_be_bytes([size_bytes[0], size_bytes[1], size_bytes[2], size_bytes[3]]);
        assert_eq!(size, data.len() as u32);

        let select_bytes = read_item(&mut dev, 2);
        let dir_select = u16::from_be_bytes([select_bytes[0], select_bytes[1]]);
        assert_eq!(dir_select, select);

        let reserved_bytes = read_item(&mut dev, 2);
        assert_eq!(reserved_bytes, vec![0, 0]);

        let name_bytes = read_item(&mut dev, FW_CFG_FILE_NAME_LEN);
        let actual_name: Vec<u8> = name_bytes
            .iter()
            .take_while(|&&b| b != 0)
            .copied()
            .collect();
        assert_eq!(actual_name, name.as_bytes());
    }

    #[test]
    fn add_file_assigns_incrementing_selectors() {
        let mut dev = FwCfg::new();
        let s0 = dev.add_file("a", vec![0]);
        let s1 = dev.add_file("b", vec![0]);
        assert_eq!(s0, FW_CFG_FILE_FIRST);
        assert_eq!(s1, FW_CFG_FILE_FIRST + 1);

        // Directory now reports two files.
        select16(&mut dev, FW_CFG_FILE_DIR);
        let count_bytes = read_item(&mut dev, 4);
        let count = u32::from_be_bytes([
            count_bytes[0],
            count_bytes[1],
            count_bytes[2],
            count_bytes[3],
        ]);
        assert_eq!(count, 2);
    }

    #[test]
    fn unknown_selector_reads_zero() {
        let mut dev = FwCfg::new();
        select16(&mut dev, 0xfffe);
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), 0);
        assert_eq!(IoDevice::read(&mut dev, FW_CFG_PORT_DATA), 0);
    }
}
