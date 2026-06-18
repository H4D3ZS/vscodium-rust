//! Legacy IDE/ATA (PIIX-style) disk controller.
//!
//! This implements the ATA register file and a PIO data-transfer state machine
//! for a single channel (configurable I/O bases) backed by an in-memory disk
//! image (`Vec<u8>`), following the same in-memory-disk pattern as the NVMe and
//! virtio-blk devices in this tree.
//!
//! # Register layout
//!
//! The *command block* lives at a base port (0x1F0 for the primary channel,
//! 0x170 for the secondary):
//!
//! | Offset | Read              | Write             |
//! |--------|-------------------|-------------------|
//! | 0x0    | Data (16-bit PIO) | Data (16-bit PIO) |
//! | 0x1    | Error             | Features          |
//! | 0x2    | Sector Count      | Sector Count      |
//! | 0x3    | LBA low           | LBA low           |
//! | 0x4    | LBA mid           | LBA mid           |
//! | 0x5    | LBA high          | LBA high          |
//! | 0x6    | Drive/Head        | Drive/Head        |
//! | 0x7    | Status            | Command           |
//!
//! The *control block* lives at a separate base (0x3F6 for the primary
//! channel, 0x376 for the secondary): reading returns the Alternate Status
//! (identical to Status but without side effects), writing sets the Device
//! Control register (nIEN interrupt mask + SRST software reset).
//!
//! # Supported commands
//!
//! * `IDENTIFY DEVICE` (0xEC) — returns a 256-word identification block.
//! * `READ SECTORS` (0x20) / `READ SECTORS (no retry)` (0x21).
//! * `READ MULTIPLE` (0xC4).
//! * `WRITE SECTORS` (0x30) / `WRITE SECTORS (no retry)` (0x31).
//! * `WRITE MULTIPLE` (0xC5).
//! * `READ SECTORS EXT` (0x24) / `WRITE SECTORS EXT` (0x34) — LBA48.
//! * `SET FEATURES` (0xEF), `SET MULTIPLE MODE` (0xC6), `FLUSH CACHE` (0xE7/0xEA),
//!   `INITIALIZE DEVICE PARAMETERS` (0x91) — accepted as no-ops (success).
//!
//! Only LBA addressing is implemented; CHS translation is not. Data transfers
//! are PIO only (no bus-master DMA).
//!
//! # Interrupts
//!
//! Following the repo convention, the device exposes a pollable interrupt latch
//! ([`IdeController::has_pending_interrupt`] / [`IdeController::clear_interrupt`])
//! that an orchestrator drains to inject IRQ14 (primary) / IRQ15 (secondary).
//! The `nIEN` bit in the Device Control register masks interrupt generation.

use std::sync::Arc;

use super::bus::IoDevice;

/// Logical sector size in bytes.
pub const SECTOR_SIZE: usize = 512;
/// Words per sector (PIO data port is 16-bit).
const SECTOR_WORDS: usize = SECTOR_SIZE / 2;
/// CD/DVD logical block size in bytes (ATAPI medium).
const CD_BLOCK: usize = 2048;

// ---- Status register bits (0x1F7 read / 0x3F6 alt-status) ----
const ST_ERR: u8 = 0x01; // Error
const ST_DSC: u8 = 0x10; // Drive Seek Complete (a.k.a. service)
#[allow(dead_code)] // documented status bit; set by hardware on device fault
const ST_DF: u8 = 0x20; // Device Fault
const ST_DRDY: u8 = 0x40; // Device Ready
const ST_BSY: u8 = 0x80; // Busy
const ST_DRQ: u8 = 0x08; // Data Request (buffer ready for transfer)

// ---- Error register bits (0x1F1 read) ----
const ERR_ABRT: u8 = 0x04; // Command aborted
const ERR_IDNF: u8 = 0x10; // ID (sector) not found

// ---- Device Control register bits (0x3F6 write) ----
const DC_NIEN: u8 = 0x02; // Interrupt disable (when set)
const DC_SRST: u8 = 0x04; // Software reset (when set)

// ---- Drive/Head register bits (0x1F6) ----
const DH_LBA: u8 = 0x40; // LBA addressing mode (vs CHS)
const DH_DEV: u8 = 0x10; // Drive select (0 = master, 1 = slave)

// ---- ATA commands ----
const CMD_READ_SECTORS: u8 = 0x20;
const CMD_READ_SECTORS_NR: u8 = 0x21;
const CMD_READ_MULTIPLE: u8 = 0xC4;
const CMD_READ_SECTORS_EXT: u8 = 0x24;
const CMD_WRITE_SECTORS: u8 = 0x30;
const CMD_WRITE_SECTORS_NR: u8 = 0x31;
const CMD_WRITE_MULTIPLE: u8 = 0xC5;
const CMD_WRITE_SECTORS_EXT: u8 = 0x34;
const CMD_IDENTIFY: u8 = 0xEC;
const CMD_SET_FEATURES: u8 = 0xEF;
const CMD_SET_MULTIPLE: u8 = 0xC6;
const CMD_FLUSH_CACHE: u8 = 0xE7;
const CMD_FLUSH_CACHE_EXT: u8 = 0xEA;
const CMD_INIT_PARAMS: u8 = 0x91;
const CMD_NOP: u8 = 0x00;
// ---- ATAPI (packet) commands ----
const CMD_PACKET: u8 = 0xA0; // send a SCSI command packet
const CMD_IDENTIFY_PACKET: u8 = 0xA1; // IDENTIFY PACKET DEVICE
const CMD_DEVICE_RESET: u8 = 0x08; // ATAPI DEVICE RESET

// ---- SCSI / MMC packet opcodes (byte 0 of the 12-byte command packet) ----
const SCSI_TEST_UNIT_READY: u8 = 0x00;
const SCSI_REQUEST_SENSE: u8 = 0x03;
const SCSI_INQUIRY: u8 = 0x12;
const SCSI_START_STOP_UNIT: u8 = 0x1B;
const SCSI_PREVENT_ALLOW: u8 = 0x1E;
const SCSI_READ_CAPACITY: u8 = 0x25;
const SCSI_READ_10: u8 = 0x28;
const SCSI_READ_12: u8 = 0xA8;
const SCSI_READ_TOC: u8 = 0x43;
const SCSI_MODE_SENSE_6: u8 = 0x1A;
const SCSI_MODE_SENSE_10: u8 = 0x5A;
const SCSI_GET_CONFIGURATION: u8 = 0x46;
const SCSI_GET_EVENT_STATUS: u8 = 0x4A;
const SCSI_READ_DISC_INFO: u8 = 0x51;

/// What the PIO buffer is being used for, which determines the transfer
/// direction and what to do once the buffer is drained/filled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Transfer {
    /// No transfer in progress.
    None,
    /// Host reads words out of the buffer (READ SECTORS, IDENTIFY, ...).
    PioIn,
    /// Host writes words into the buffer (WRITE SECTORS).
    PioOut,
}

/// A legacy IDE/ATA controller channel backed by an in-memory disk image.
///
/// The disk image length divided by [`SECTOR_SIZE`] is the reported LBA
/// capacity (a partial trailing sector is ignored for addressing purposes).
pub struct IdeController {
    /// Command-block base port (0x1F0 primary, 0x170 secondary).
    cmd_base: u16,
    /// Control-block base port (0x3F6 primary, 0x376 secondary).
    ctrl_base: u16,

    // ---- Backing disk (master / device 0 only) ----
    disk: Vec<u8>,

    // ---- Task-file registers ----
    features: u8,
    error: u8,
    sector_count: u16, // 8-bit LBA28, 16-bit LBA48 (high byte from "previous")
    lba_low: u16,      // current | (previous << 8)
    lba_mid: u16,
    lba_high: u16,
    drive_head: u8, // includes LBA bits 24-27, DEV select, LBA mode bit
    status: u8,
    device_control: u8,

    /// HOB (high-order byte) flag from Device Control read-back select; when a
    /// register that has a "previous content" latch is read with HOB set the
    /// high byte is returned. We track LBA48 register pairs via the 16-bit
    /// fields above and use this to select which byte to return.
    hob: bool,

    // ---- PIO transfer state ----
    /// Sector buffer; sized to a single sector and refilled as needed.
    buffer: Vec<u8>,
    /// Read/write cursor (in bytes) into `buffer`.
    buf_pos: usize,
    /// Bytes currently valid in `buffer`.
    buf_len: usize,
    /// Active transfer mode.
    transfer: Transfer,
    /// Sectors remaining in the current multi-sector command.
    sectors_left: u32,
    /// Current LBA for the in-progress transfer.
    current_lba: u64,
    /// True while the in-progress command uses LBA48 addressing.
    lba48: bool,

    /// Multi-sector "block size" set by SET MULTIPLE MODE (sectors per DRQ).
    /// Not used to change the PIO chunking here (we always transfer a sector
    /// at a time) but tracked so IDENTIFY can report it.
    multiple_sectors: u8,

    // ---- Interrupt latch (poll convention) ----
    irq_pending: bool,

    // ---- ATAPI CD-ROM (master device) ----
    /// When `Some`, the master device is an ATAPI CD-ROM whose medium is this
    /// ISO image (read-only, 2048-byte logical blocks). When `None`, the master
    /// is an ATA disk backed by `disk` (existing behavior).
    cdrom: Option<Arc<Vec<u8>>>,
    /// True while a PACKET command is waiting for the 12-byte command packet to
    /// be written by the host (PIO-out into `packet`).
    awaiting_packet: bool,
    /// The 12-byte SCSI command packet being assembled / last received.
    packet: [u8; 12],
    /// Full PIO-in result of the current ATAPI command; transferred to the host
    /// in DRQ blocks bounded by the byte-count limit.
    atapi_data: Vec<u8>,
    /// Read cursor into `atapi_data`.
    atapi_pos: usize,
    /// True while an ATAPI data-in transfer is in progress (selects the ATAPI
    /// chunk-advance path on a PIO-in block boundary).
    atapi_in: bool,
}

impl IdeController {
    /// Create a primary-channel controller (command block 0x1F0-0x1F7,
    /// control block 0x3F6) backed by `disk`.
    pub fn new_primary(disk: Vec<u8>) -> Self {
        Self::new(0x1F0, 0x3F6, disk)
    }

    /// Create a secondary-channel controller (command block 0x170-0x177,
    /// control block 0x376) backed by `disk`.
    pub fn new_secondary(disk: Vec<u8>) -> Self {
        Self::new(0x170, 0x376, disk)
    }

    /// Create a primary-channel controller with `sectors` blank sectors.
    pub fn with_sectors(sectors: u64) -> Self {
        Self::new_primary(vec![0u8; (sectors as usize) * SECTOR_SIZE])
    }

    /// Construct a controller at explicit `cmd_base` / `ctrl_base` ports.
    pub fn new(cmd_base: u16, ctrl_base: u16, disk: Vec<u8>) -> Self {
        IdeController {
            cmd_base,
            ctrl_base,
            disk,
            features: 0,
            error: 0,
            sector_count: 0,
            lba_low: 0,
            lba_mid: 0,
            lba_high: 0,
            drive_head: 0,
            // DRDY + DSC asserted at idle once "spun up".
            status: ST_DRDY | ST_DSC,
            device_control: 0,
            hob: false,
            buffer: vec![0u8; SECTOR_SIZE],
            buf_pos: 0,
            buf_len: 0,
            transfer: Transfer::None,
            sectors_left: 0,
            current_lba: 0,
            lba48: false,
            multiple_sectors: 0,
            irq_pending: false,
            cdrom: None,
            awaiting_packet: false,
            packet: [0u8; 12],
            atapi_data: Vec::new(),
            atapi_pos: 0,
            atapi_in: false,
        }
    }

    /// Attach an ATAPI CD-ROM medium (an ISO image) to the master device. The
    /// channel then presents an ATAPI CD-ROM (signature 0x14EB, IDENTIFY PACKET,
    /// the SCSI packet command set) instead of an ATA disk. The image uses
    /// 2048-byte logical blocks and is read-only.
    pub fn attach_cdrom(&mut self, iso: Arc<Vec<u8>>) {
        self.cdrom = Some(iso);
        // Present the ATAPI signature immediately (as after a power-on reset).
        self.lba_mid = 0x14;
        self.lba_high = 0xEB;
        self.sector_count = 0x01;
        self.status = ST_DRDY | ST_DSC;
    }

    /// True if the master device is an ATAPI CD-ROM.
    fn is_atapi(&self) -> bool {
        self.cdrom.is_some()
    }

    /// Capacity of the backing disk in whole [`SECTOR_SIZE`]-byte sectors.
    pub fn capacity_sectors(&self) -> u64 {
        (self.disk.len() / SECTOR_SIZE) as u64
    }

    /// Immutable view of the backing disk (for tests / inspection).
    pub fn disk(&self) -> &[u8] {
        &self.disk
    }

    /// Mutable view of the backing disk (for tests / host-side seeding).
    pub fn disk_mut(&mut self) -> &mut [u8] {
        &mut self.disk
    }

    /// Current Status register value (no side effects).
    pub fn status(&self) -> u8 {
        self.status
    }

    /// True if a channel interrupt (IRQ14 primary / IRQ15 secondary) is pending
    /// and not yet acked. Honors the `nIEN` interrupt mask.
    pub fn has_pending_interrupt(&self) -> bool {
        self.irq_pending && (self.device_control & DC_NIEN) == 0
    }

    /// Clear the pending-interrupt latch (an orchestrator calls this after
    /// injecting the interrupt).
    pub fn clear_interrupt(&mut self) {
        self.irq_pending = false;
    }

    /// True if this channel is the legacy primary (IRQ14); false for secondary
    /// (IRQ15). Useful for an orchestrator deciding which line to raise.
    pub fn is_primary(&self) -> bool {
        self.cmd_base == 0x1F0
    }

    // ---- Internal helpers --------------------------------------------------

    /// Raise the channel interrupt latch (subject to `nIEN`).
    fn raise_irq(&mut self) {
        self.irq_pending = true;
    }

    /// True if device 0 (master) is selected. We only model a master drive; if
    /// the slave is selected there is "no device".
    fn master_selected(&self) -> bool {
        (self.drive_head & DH_DEV) == 0
    }

    /// Abort the current command, setting ERR/ABRT and clearing BSY/DRQ.
    fn abort(&mut self, extra_err: u8) {
        self.transfer = Transfer::None;
        self.sectors_left = 0;
        self.buf_pos = 0;
        self.buf_len = 0;
        self.error = ERR_ABRT | extra_err;
        self.status = (self.status & !(ST_BSY | ST_DRQ | ST_DSC)) | ST_DRDY | ST_ERR;
        self.raise_irq();
    }

    /// Compute the starting LBA from the task-file registers for the current
    /// command. `lba48` selects 48-bit addressing (using the latched high
    /// bytes) vs 28-bit.
    fn compute_lba(&self, lba48: bool) -> u64 {
        if lba48 {
            let low = (self.lba_low & 0xFF) as u64;
            let mid = (self.lba_mid & 0xFF) as u64;
            let high = (self.lba_high & 0xFF) as u64;
            let low_p = ((self.lba_low >> 8) & 0xFF) as u64;
            let mid_p = ((self.lba_mid >> 8) & 0xFF) as u64;
            let high_p = ((self.lba_high >> 8) & 0xFF) as u64;
            low | (mid << 8) | (high << 16) | (low_p << 24) | (mid_p << 32) | (high_p << 40)
        } else {
            let low = (self.lba_low & 0xFF) as u64;
            let mid = (self.lba_mid & 0xFF) as u64;
            let high = (self.lba_high & 0xFF) as u64;
            let top = (self.drive_head & 0x0F) as u64; // LBA bits 24-27
            low | (mid << 8) | (high << 16) | (top << 24)
        }
    }

    /// Count of sectors requested by the current command. A count of 0 means
    /// the maximum (256 for LBA28, 65536 for LBA48).
    fn requested_sectors(&self, lba48: bool) -> u32 {
        if lba48 {
            let n = self.sector_count & 0xFFFF;
            if n == 0 { 65536 } else { n as u32 }
        } else {
            let n = self.sector_count & 0xFF;
            if n == 0 { 256 } else { n as u32 }
        }
    }

    /// Load one sector at `self.current_lba` into the transfer buffer.
    /// Returns false (and aborts) if the LBA is out of range.
    fn load_sector_into_buffer(&mut self) -> bool {
        let lba = self.current_lba;
        let cap = self.capacity_sectors();
        if lba >= cap {
            self.abort(ERR_IDNF);
            return false;
        }
        let start = (lba as usize) * SECTOR_SIZE;
        self.buffer[..SECTOR_SIZE].copy_from_slice(&self.disk[start..start + SECTOR_SIZE]);
        self.buf_pos = 0;
        self.buf_len = SECTOR_SIZE;
        true
    }

    /// Flush the just-filled transfer buffer to `self.current_lba`.
    /// Returns false (and aborts) if the LBA is out of range.
    fn flush_buffer_to_sector(&mut self) -> bool {
        let lba = self.current_lba;
        let cap = self.capacity_sectors();
        if lba >= cap {
            self.abort(ERR_IDNF);
            return false;
        }
        let start = (lba as usize) * SECTOR_SIZE;
        self.disk[start..start + SECTOR_SIZE].copy_from_slice(&self.buffer[..SECTOR_SIZE]);
        true
    }

    /// Begin a multi-sector READ command (PIO-in).
    fn start_read(&mut self, lba48: bool) {
        if !self.master_selected() {
            self.abort(0);
            return;
        }
        let lba = self.compute_lba(lba48);
        let count = self.requested_sectors(lba48);
        if lba >= self.capacity_sectors() {
            self.current_lba = lba;
            self.abort(ERR_IDNF);
            return;
        }
        self.lba48 = lba48;
        self.current_lba = lba;
        self.sectors_left = count;
        self.error = 0;
        self.status &= !ST_ERR;
        // Load the first sector and signal DRQ.
        if !self.load_sector_into_buffer() {
            return;
        }
        self.transfer = Transfer::PioIn;
        self.status = (self.status & !ST_BSY) | ST_DRDY | ST_DRQ | ST_DSC;
        // A data-transfer interrupt is asserted when each block of data is
        // ready to be read by the host.
        self.raise_irq();
    }

    /// Begin a multi-sector WRITE command (PIO-out).
    fn start_write(&mut self, lba48: bool) {
        if !self.master_selected() {
            self.abort(0);
            return;
        }
        let lba = self.compute_lba(lba48);
        let count = self.requested_sectors(lba48);
        if lba >= self.capacity_sectors() {
            self.current_lba = lba;
            self.abort(ERR_IDNF);
            return;
        }
        self.lba48 = lba48;
        self.current_lba = lba;
        self.sectors_left = count;
        self.error = 0;
        self.status &= !ST_ERR;
        // Request the first sector of data from the host.
        self.buf_pos = 0;
        self.buf_len = SECTOR_SIZE;
        self.transfer = Transfer::PioOut;
        self.status = (self.status & !ST_BSY) | ST_DRDY | ST_DRQ | ST_DSC;
        // No interrupt on the initial WRITE DRQ assertion (host writes first).
    }

    /// Build the 256-word IDENTIFY DEVICE block into the transfer buffer.
    fn start_identify(&mut self) {
        if !self.master_selected() {
            // No device on the slave: command aborts.
            self.abort(0);
            return;
        }
        let mut words = [0u16; SECTOR_WORDS];

        let total = self.capacity_sectors();
        // LBA28-addressable sectors are capped at 0x0FFF_FFFF.
        let lba28 = total.min(0x0FFF_FFFF) as u32;

        // Word 0: general configuration. 0x0040 => fixed (non-removable) device.
        words[0] = 0x0040;
        // Words 1/3/6: obsolete CHS geometry. Provide a plausible default.
        words[1] = 16383; // cylinders
        words[3] = 16; // heads
        words[6] = 63; // sectors per track

        // Words 10-19: serial number (20 ASCII chars, byte-swapped per word).
        put_ata_string(&mut words[10..20], "RAX-IDE-0000000001  ");
        // Words 23-26: firmware revision (8 chars).
        put_ata_string(&mut words[23..27], "1.0     ");
        // Words 27-46: model number (40 chars).
        put_ata_string(
            &mut words[27..47],
            "RAX Virtual ATA Disk                    ",
        );

        // Word 47: max sectors per READ/WRITE MULTIPLE (low byte), 0x80 marker.
        words[47] = 0x8000 | 16;
        // Word 49: capabilities. Bit 9 = LBA supported, bit 8 = DMA supported.
        words[49] = (1 << 9) | (1 << 8);
        // Word 50: capabilities (mandatory bit 14 set).
        words[50] = 0x4000;
        // Word 51: PIO data transfer cycle timing mode.
        words[51] = 0x0200;
        // Word 53: bits 0/1/2 validity for words 54-58 / 64-70 / 88.
        words[53] = 0x0007;
        // Words 54-58: current CHS (mirror of 1/3/6 and capacity in sectors).
        words[54] = 16383;
        words[55] = 16;
        words[56] = 63;
        let chs_cap = 16383u32 * 16 * 63;
        words[57] = (chs_cap & 0xFFFF) as u16;
        words[58] = (chs_cap >> 16) as u16;
        // Word 59: multiple-sector setting; bit 8 set => "valid", low byte =
        // current block size.
        if self.multiple_sectors != 0 {
            words[59] = 0x0100 | self.multiple_sectors as u16;
        }
        // Words 60-61: total addressable sectors for LBA28.
        words[60] = (lba28 & 0xFFFF) as u16;
        words[61] = (lba28 >> 16) as u16;
        // Word 63: multiword DMA. Bits 0-2 = modes 0-2 supported; bit 8 set =>
        // mode 0 selected.
        words[63] = 0x0007 | 0x0100;
        // Word 64: PIO modes 3 & 4 supported.
        words[64] = 0x0003;
        // Words 65-68: DMA / PIO cycle times (ns).
        words[65] = 120;
        words[66] = 120;
        words[67] = 120;
        words[68] = 120;
        // Word 80: major version (ATA-4 .. ATA-8 bits).
        words[80] = 0x00F0;
        // Word 81: minor version.
        words[81] = 0x0000;
        // Word 82: command set 1. Bit 14 = NOP, bit 12 = write cache, etc.
        words[82] = (1 << 14) | (1 << 12);
        // Word 83: command set 2. Bit 10 = LBA48 supported, bit 14 mandatory.
        words[83] = (1 << 14) | (1 << 10);
        // Word 84: command set 3. Bit 14 mandatory one.
        words[84] = 1 << 14;
        // Words 85-87 mirror 82-84 as "enabled".
        words[85] = (1 << 14) | (1 << 12);
        words[86] = 1 << 10;
        words[87] = 1 << 14;
        // Word 88: Ultra DMA modes; bits 0-2 supported.
        words[88] = 0x0007;
        // Words 100-103: 48-bit total addressable sectors.
        words[100] = (total & 0xFFFF) as u16;
        words[101] = ((total >> 16) & 0xFFFF) as u16;
        words[102] = ((total >> 32) & 0xFFFF) as u16;
        words[103] = ((total >> 48) & 0xFFFF) as u16;

        // Serialize little-endian words into the byte buffer.
        for (i, w) in words.iter().enumerate() {
            let b = w.to_le_bytes();
            self.buffer[i * 2] = b[0];
            self.buffer[i * 2 + 1] = b[1];
        }
        self.buf_pos = 0;
        self.buf_len = SECTOR_SIZE;
        self.sectors_left = 0;
        self.transfer = Transfer::PioIn;
        self.error = 0;
        self.status = (self.status & !(ST_BSY | ST_ERR)) | ST_DRDY | ST_DRQ | ST_DSC;
        self.raise_irq();
    }

    /// Perform a software reset (SRST). Resets the task-file to its
    /// diagnostic-passed signature for an ATA (non-packet) device.
    fn software_reset(&mut self) {
        self.transfer = Transfer::None;
        self.sectors_left = 0;
        self.buf_pos = 0;
        self.buf_len = 0;
        self.features = 0;
        self.error = 0x01; // diagnostic code: device 0 passed
        self.sector_count = 0x01;
        self.lba_low = 0x01;
        if self.is_atapi() {
            // ATAPI (PACKET) device signature: 0x14 in LBA mid, 0xEB in LBA high.
            self.lba_mid = 0x14;
            self.lba_high = 0xEB;
        } else {
            self.lba_mid = 0x00; // ATA signature (0x0000 in mid/high)
            self.lba_high = 0x00;
        }
        self.drive_head = 0;
        self.hob = false;
        self.awaiting_packet = false;
        self.atapi_in = false;
        self.atapi_data.clear();
        self.atapi_pos = 0;
        self.status = ST_DRDY | ST_DSC;
        // SRST does not generate an interrupt.
    }

    // ---- ATAPI (CD-ROM packet) -------------------------------------------

    /// Build the 256-word IDENTIFY PACKET DEVICE block for the ATAPI CD-ROM.
    fn start_atapi_identify(&mut self) {
        let mut words = [0u16; SECTOR_WORDS];
        // Word 0: general config. 10b<<14 = ATAPI; bits 12-8 = 0x05 (CD-ROM);
        // bits 6-5 = DRQ type (1 = IRQ within 3ms); bits 1-0 = 00 (12-byte cmd).
        words[0] = 0x8580;
        put_ata_string(&mut words[10..20], "RAX-CDROM-00000001  ");
        put_ata_string(&mut words[23..27], "1.0     ");
        put_ata_string(
            &mut words[27..47],
            "RAX Virtual CD-ROM                      ",
        );
        // Word 49: capabilities — bit 9 LBA, bit 8 DMA.
        words[49] = (1 << 9) | (1 << 8);
        words[50] = 0x4000;
        words[53] = 0x0006;
        words[63] = 0x0007; // multiword DMA modes 0-2
        words[64] = 0x0003; // PIO modes 3,4
        words[80] = 0x001E; // ATA versions
        words[82] = 1 << 4; // PACKET feature set
        words[83] = 1 << 14;
        words[84] = 1 << 14;
        words[85] = 1 << 4;
        words[86] = 0;
        words[87] = 1 << 14;
        words[88] = 0x0007;

        if self.buffer.len() < SECTOR_SIZE {
            self.buffer.resize(SECTOR_SIZE, 0);
        }
        for (i, w) in words.iter().enumerate() {
            let b = w.to_le_bytes();
            self.buffer[i * 2] = b[0];
            self.buffer[i * 2 + 1] = b[1];
        }
        self.buf_pos = 0;
        self.buf_len = SECTOR_SIZE;
        self.sectors_left = 0;
        self.atapi_in = false;
        self.transfer = Transfer::PioIn;
        self.error = 0;
        self.sector_count = 0x02; // data to host
        self.status = (self.status & !(ST_BSY | ST_ERR)) | ST_DRDY | ST_DRQ | ST_DSC;
        self.raise_irq();
    }

    /// Begin a PACKET (0xA0) command: request the 12-byte command packet from
    /// the host via PIO-out (no interrupt; the host polls DRQ and writes it).
    fn begin_packet(&mut self) {
        self.error = 0;
        self.packet = [0u8; 12];
        self.buf_pos = 0;
        self.buf_len = 12;
        self.awaiting_packet = true;
        self.transfer = Transfer::PioOut;
        // Interrupt reason: C/D=1 (command), IO=0 (from host).
        self.sector_count = 0x01;
        self.status = (self.status & !(ST_BSY | ST_ERR)) | ST_DRDY | ST_DRQ;
        // Per ATAPI, no interrupt is raised for the command-packet request.
    }

    /// Execute the SCSI command packet now in `self.packet` and stage its result.
    fn execute_packet(&mut self) {
        self.awaiting_packet = false;
        self.transfer = Transfer::None;
        self.buf_pos = 0;
        self.buf_len = 0;
        let pkt = self.packet;
        let total_blocks = self
            .cdrom
            .as_ref()
            .map_or(0, |c| (c.len() / CD_BLOCK) as u64);

        match pkt[0] {
            SCSI_TEST_UNIT_READY | SCSI_START_STOP_UNIT | SCSI_PREVENT_ALLOW => {
                self.atapi_complete_ok();
            }
            SCSI_REQUEST_SENSE => {
                let alloc = pkt[4] as usize;
                let mut sense = vec![0u8; 18];
                sense[0] = 0x70; // current error, valid
                sense[7] = 10; // additional sense length
                sense.truncate(alloc.min(18).max(0));
                self.atapi_start_data(sense);
            }
            SCSI_INQUIRY => {
                let alloc = pkt[4] as usize;
                let mut inq = vec![0u8; 36];
                inq[0] = 0x05; // peripheral device type: CD-ROM
                inq[1] = 0x80; // RMB: removable medium
                inq[2] = 0x00; // version
                inq[3] = 0x21; // response data format (2) + HiSup
                inq[4] = 31; // additional length
                inq[8..16].copy_from_slice(b"RAX     ");
                inq[16..32].copy_from_slice(b"Virtual CD-ROM  ");
                inq[32..36].copy_from_slice(b"1.0 ");
                let n = if alloc == 0 { 36 } else { alloc.min(36) };
                inq.truncate(n);
                self.atapi_start_data(inq);
            }
            SCSI_READ_CAPACITY => {
                let last = total_blocks.saturating_sub(1) as u32;
                let mut cap = vec![0u8; 8];
                cap[0..4].copy_from_slice(&last.to_be_bytes());
                cap[4..8].copy_from_slice(&(CD_BLOCK as u32).to_be_bytes());
                self.atapi_start_data(cap);
            }
            SCSI_READ_10 => {
                let lba = u32::from_be_bytes([pkt[2], pkt[3], pkt[4], pkt[5]]) as u64;
                let len = u16::from_be_bytes([pkt[7], pkt[8]]) as u64;
                self.atapi_read(lba, len, total_blocks);
            }
            SCSI_READ_12 => {
                let lba = u32::from_be_bytes([pkt[2], pkt[3], pkt[4], pkt[5]]) as u64;
                let len = u32::from_be_bytes([pkt[6], pkt[7], pkt[8], pkt[9]]) as u64;
                self.atapi_read(lba, len, total_blocks);
            }
            SCSI_READ_TOC => {
                // Minimal TOC: header + track 1 + lead-out. (MSF or LBA per pkt[1].)
                let alloc = u16::from_be_bytes([pkt[7], pkt[8]]) as usize;
                let mut toc = vec![
                    0x00, 0x12, // TOC data length (18)
                    0x01, 0x01, // first track, last track
                    // Track 1 descriptor
                    0x00, 0x14, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                    // Lead-out descriptor (track 0xAA)
                    0x00, 0x14, 0xAA, 0x00, 0x00, 0x00, 0x00, 0x00,
                ];
                let last = (total_blocks.saturating_sub(1)) as u32;
                toc[16..20].copy_from_slice(&last.to_be_bytes());
                let n = if alloc == 0 {
                    toc.len()
                } else {
                    alloc.min(toc.len())
                };
                toc.truncate(n);
                self.atapi_start_data(toc);
            }
            SCSI_MODE_SENSE_6 => {
                let alloc = pkt[4] as usize;
                // Mode parameter header (6): data length, medium type, dev params,
                // block descriptor length.
                let mut data = vec![0u8; 4];
                data[0] = (data.len() - 1) as u8;
                let n = if alloc == 0 {
                    data.len()
                } else {
                    alloc.min(data.len())
                };
                data.truncate(n);
                self.atapi_start_data(data);
            }
            SCSI_MODE_SENSE_10 => {
                let alloc = u16::from_be_bytes([pkt[7], pkt[8]]) as usize;
                let mut data = vec![0u8; 8];
                let l = (data.len() - 2) as u16;
                data[0..2].copy_from_slice(&l.to_be_bytes());
                let n = if alloc == 0 {
                    data.len()
                } else {
                    alloc.min(data.len())
                };
                data.truncate(n);
                self.atapi_start_data(data);
            }
            SCSI_GET_CONFIGURATION | SCSI_GET_EVENT_STATUS | SCSI_READ_DISC_INFO => {
                // Return an empty (but successful) response: just the length field.
                let alloc = u16::from_be_bytes([pkt[7], pkt[8]]) as usize;
                let data = vec![0u8; alloc.min(8)];
                if data.is_empty() {
                    self.atapi_complete_ok();
                } else {
                    self.atapi_start_data(data);
                }
            }
            _ => {
                // Unsupported command: complete with no data (best effort).
                self.atapi_complete_ok();
            }
        }
    }

    /// READ(10)/READ(12): stage `len` 2048-byte blocks from `lba` of the medium.
    fn atapi_read(&mut self, lba: u64, len: u64, total_blocks: u64) {
        if len == 0 {
            self.atapi_complete_ok();
            return;
        }
        if lba.saturating_add(len) > total_blocks {
            // Out of range → CHECK CONDITION (ILLEGAL REQUEST / LBA out of range).
            self.error = 0x50; // sense key 5 (illegal request) in bits 7-4
            self.status = (self.status & !(ST_BSY | ST_DRQ)) | ST_DRDY | ST_ERR | ST_DSC;
            self.sector_count = 0x03;
            self.raise_irq();
            return;
        }
        let start = (lba as usize) * CD_BLOCK;
        let end = ((lba + len) as usize) * CD_BLOCK;
        let data = self
            .cdrom
            .as_ref()
            .map(|c| c[start..end.min(c.len())].to_vec())
            .unwrap_or_default();
        self.atapi_start_data(data);
    }

    /// Complete an ATAPI command that returns no data (status only).
    fn atapi_complete_ok(&mut self) {
        self.transfer = Transfer::None;
        self.atapi_in = false;
        self.error = 0;
        self.sector_count = 0x03; // C/D=1, IO=1 (command complete)
        self.status = (self.status & !(ST_BSY | ST_DRQ | ST_ERR)) | ST_DRDY | ST_DSC;
        self.raise_irq();
    }

    /// Begin a PIO-in data phase returning `data` to the host, chunked by the
    /// host's byte-count limit (the LBA mid/high registers set before PACKET).
    fn atapi_start_data(&mut self, data: Vec<u8>) {
        if data.is_empty() {
            self.atapi_complete_ok();
            return;
        }
        self.atapi_data = data;
        self.atapi_pos = 0;
        self.atapi_in = true;
        self.error = 0;
        self.stage_atapi_chunk();
    }

    /// Stage the next DRQ block of `atapi_data` into the transfer buffer.
    fn stage_atapi_chunk(&mut self) {
        let remaining = self.atapi_data.len() - self.atapi_pos;
        // Byte-count limit the host requested (LBA mid/high before PACKET).
        let mut limit = ((self.lba_high & 0xFF) as usize) << 8 | (self.lba_mid & 0xFF) as usize;
        if limit == 0 {
            limit = 0xFFFE;
        }
        let mut chunk = remaining.min(limit);
        if chunk % 2 != 0 && chunk < remaining {
            chunk -= 1; // keep DRQ blocks word-aligned
        }
        if chunk == 0 {
            chunk = remaining;
        }
        if self.buffer.len() < chunk {
            self.buffer.resize(chunk, 0);
        }
        self.buffer[..chunk]
            .copy_from_slice(&self.atapi_data[self.atapi_pos..self.atapi_pos + chunk]);
        self.buf_pos = 0;
        self.buf_len = chunk;
        // Report the actual byte count for this DRQ block in LBA mid/high.
        self.lba_mid = (chunk & 0xFF) as u16;
        self.lba_high = ((chunk >> 8) & 0xFF) as u16;
        self.sector_count = 0x02; // C/D=0, IO=1 (data to host)
        self.transfer = Transfer::PioIn;
        self.status = (self.status & !(ST_BSY | ST_ERR)) | ST_DRDY | ST_DRQ | ST_DSC;
        self.raise_irq();
    }

    /// Advance to the next ATAPI DRQ block (or complete the transfer).
    fn atapi_in_block_complete(&mut self) {
        self.atapi_pos += self.buf_len;
        if self.atapi_pos >= self.atapi_data.len() {
            // All data transferred → command complete.
            self.atapi_data.clear();
            self.atapi_pos = 0;
            self.atapi_complete_ok();
        } else {
            self.stage_atapi_chunk();
        }
    }

    /// Dispatch a command written to the command register (0x1F7).
    fn execute_command(&mut self, cmd: u8) {
        // Clear stale error on a fresh command.
        self.error = 0;
        self.status &= !ST_ERR;
        self.hob = false;

        // ATAPI (CD-ROM master) command handling. Only the master is a device;
        // the slave is absent.
        if self.is_atapi() && self.master_selected() {
            match cmd {
                CMD_IDENTIFY => {
                    // IDENTIFY DEVICE on a packet device aborts and leaves the
                    // ATAPI signature in the LBA mid/high registers — this is how
                    // hosts distinguish ATAPI from ATA.
                    self.lba_mid = 0x14;
                    self.lba_high = 0xEB;
                    self.abort(0);
                    return;
                }
                CMD_IDENTIFY_PACKET => {
                    self.start_atapi_identify();
                    return;
                }
                CMD_PACKET => {
                    self.begin_packet();
                    return;
                }
                CMD_DEVICE_RESET => {
                    self.software_reset();
                    return;
                }
                CMD_NOP => {
                    self.abort(0);
                    return;
                }
                _ => {
                    self.abort(0);
                    return;
                }
            }
        }

        match cmd {
            CMD_READ_SECTORS | CMD_READ_SECTORS_NR | CMD_READ_MULTIPLE => self.start_read(false),
            CMD_READ_SECTORS_EXT => self.start_read(true),
            CMD_WRITE_SECTORS | CMD_WRITE_SECTORS_NR | CMD_WRITE_MULTIPLE => {
                self.start_write(false)
            }
            CMD_WRITE_SECTORS_EXT => self.start_write(true),
            CMD_IDENTIFY => self.start_identify(),
            CMD_SET_MULTIPLE => {
                // Sector count holds the requested block size (0 disables).
                self.multiple_sectors = (self.sector_count & 0xFF) as u8;
                self.status = (self.status & !(ST_BSY | ST_DRQ)) | ST_DRDY | ST_DSC;
                self.raise_irq();
            }
            CMD_SET_FEATURES | CMD_FLUSH_CACHE | CMD_FLUSH_CACHE_EXT | CMD_INIT_PARAMS
            | CMD_NOP => {
                if !self.master_selected() {
                    self.abort(0);
                    return;
                }
                self.status = (self.status & !(ST_BSY | ST_DRQ)) | ST_DRDY | ST_DSC;
                self.raise_irq();
            }
            _ => {
                // Unknown / unsupported command: abort.
                self.abort(0);
            }
        }
    }

    // ---- PIO data port (0x1F0) ---------------------------------------------

    /// Read a 16-bit word from the PIO data port. Advances the buffer cursor
    /// and, on a sector boundary, either loads the next sector or ends the
    /// transfer.
    fn read_data_word(&mut self) -> u16 {
        if self.transfer != Transfer::PioIn || self.buf_pos + 2 > self.buf_len {
            return 0xFFFF;
        }
        let lo = self.buffer[self.buf_pos] as u16;
        let hi = self.buffer[self.buf_pos + 1] as u16;
        self.buf_pos += 2;
        let word = lo | (hi << 8);

        if self.buf_pos >= self.buf_len {
            // Finished a sector's worth of data.
            self.on_pio_in_sector_complete();
        }
        word
    }

    /// Read one byte from the PIO data buffer. The IDE data register is a single
    /// fixed port whose successive byte reads return consecutive buffer bytes;
    /// the host (or the I/O bus) repeats the read at the same port for word/dword
    /// PIO. On a block boundary the next block is staged or the transfer ends.
    fn read_data_byte(&mut self) -> u8 {
        if self.transfer != Transfer::PioIn || self.buf_pos >= self.buf_len {
            return 0xFF;
        }
        let b = self.buffer[self.buf_pos];
        self.buf_pos += 1;
        if self.buf_pos >= self.buf_len {
            self.on_pio_in_sector_complete();
        }
        b
    }

    /// Write one byte into the PIO data buffer (counterpart of [`read_data_byte`]).
    fn write_data_byte(&mut self, b: u8) {
        if self.transfer != Transfer::PioOut || self.buf_pos >= self.buf_len {
            return;
        }
        self.buffer[self.buf_pos] = b;
        self.buf_pos += 1;
        if self.buf_pos >= self.buf_len {
            self.on_pio_out_sector_complete();
        }
    }

    /// Called when the host has drained a full sector during a PIO-in transfer.
    fn on_pio_in_sector_complete(&mut self) {
        if self.atapi_in {
            self.atapi_in_block_complete();
            return;
        }
        if self.sectors_left > 0 {
            self.sectors_left -= 1;
        }
        if self.sectors_left == 0 {
            // Transfer complete.
            self.transfer = Transfer::None;
            self.status = (self.status & !(ST_DRQ | ST_BSY)) | ST_DRDY | ST_DSC;
        } else {
            // Advance and stage the next sector.
            self.current_lba += 1;
            if self.load_sector_into_buffer() {
                self.status = (self.status & !ST_BSY) | ST_DRDY | ST_DRQ | ST_DSC;
                // Each block ready raises an interrupt.
                self.raise_irq();
            }
        }
    }

    /// Write a 16-bit word to the PIO data port. Fills the buffer; on a sector
    /// boundary the buffer is flushed to the disk.
    fn write_data_word(&mut self, word: u16) {
        if self.transfer != Transfer::PioOut || self.buf_pos + 2 > self.buf_len {
            return;
        }
        self.buffer[self.buf_pos] = (word & 0xFF) as u8;
        self.buffer[self.buf_pos + 1] = (word >> 8) as u8;
        self.buf_pos += 2;

        if self.buf_pos >= self.buf_len {
            self.on_pio_out_sector_complete();
        }
    }

    /// Called when the host has filled a full sector during a PIO-out transfer.
    fn on_pio_out_sector_complete(&mut self) {
        // ATAPI: the host just delivered the 12-byte command packet.
        if self.awaiting_packet {
            self.packet.copy_from_slice(&self.buffer[..12]);
            self.execute_packet();
            return;
        }
        // Flush the staged sector to the backing disk.
        if !self.flush_buffer_to_sector() {
            return; // abort() already handled status/irq
        }
        if self.sectors_left > 0 {
            self.sectors_left -= 1;
        }
        if self.sectors_left == 0 {
            self.transfer = Transfer::None;
            self.status = (self.status & !(ST_DRQ | ST_BSY)) | ST_DRDY | ST_DSC;
            // Completion interrupt for the final block.
            self.raise_irq();
        } else {
            self.current_lba += 1;
            self.buf_pos = 0;
            self.buf_len = SECTOR_SIZE;
            self.status = (self.status & !ST_BSY) | ST_DRDY | ST_DRQ | ST_DSC;
            // Interrupt to request the next block of data.
            self.raise_irq();
        }
    }

    // ---- Register read/write dispatch --------------------------------------

    fn read_register(&mut self, port: u16) -> u8 {
        // Control block: alternate status (read).
        if port == self.ctrl_base {
            return self.status;
        }
        let off = port.wrapping_sub(self.cmd_base);
        match off {
            0x1 => self.error,
            0x2 => {
                if self.hob {
                    ((self.sector_count >> 8) & 0xFF) as u8
                } else {
                    (self.sector_count & 0xFF) as u8
                }
            }
            0x3 => {
                if self.hob {
                    ((self.lba_low >> 8) & 0xFF) as u8
                } else {
                    (self.lba_low & 0xFF) as u8
                }
            }
            0x4 => {
                if self.hob {
                    ((self.lba_mid >> 8) & 0xFF) as u8
                } else {
                    (self.lba_mid & 0xFF) as u8
                }
            }
            0x5 => {
                if self.hob {
                    ((self.lba_high >> 8) & 0xFF) as u8
                } else {
                    (self.lba_high & 0xFF) as u8
                }
            }
            0x6 => self.drive_head | DH_LBA | 0xA0, // bits 7,5 are obsolete-1
            0x7 => {
                // Reading the (primary) Status register acknowledges a pending
                // interrupt.
                self.irq_pending = false;
                self.status
            }
            _ => 0xFF,
        }
    }

    fn write_register(&mut self, port: u16, value: u8) {
        // Control block: device control register (write).
        if port == self.ctrl_base {
            let was_srst = (self.device_control & DC_SRST) != 0;
            self.device_control = value;
            let now_srst = (value & DC_SRST) != 0;
            // SRST is asserted on the 0->1 transition; the device performs the
            // reset while SRST is held and presents its signature when cleared.
            if !was_srst && now_srst {
                self.software_reset();
            }
            // HOB: reading is selected by Device Control bit 7.
            self.hob = (value & 0x80) != 0;
            return;
        }
        let off = port.wrapping_sub(self.cmd_base);
        match off {
            0x1 => self.features = value,
            0x2 => {
                // 16-bit latch: new value shifts the previous content up (LBA48).
                self.sector_count = (self.sector_count << 8) | value as u16;
            }
            0x3 => {
                self.lba_low = (self.lba_low << 8) | value as u16;
            }
            0x4 => {
                self.lba_mid = (self.lba_mid << 8) | value as u16;
            }
            0x5 => {
                self.lba_high = (self.lba_high << 8) | value as u16;
            }
            0x6 => {
                // Keep only the meaningful bits (LBA top nibble, DEV, LBA mode).
                self.drive_head = value & (0x0F | DH_DEV | DH_LBA);
            }
            0x7 => self.execute_command(value),
            _ => {}
        }
    }
}

impl IoDevice for IdeController {
    fn read(&mut self, port: u16) -> u8 {
        if port == self.cmd_base {
            // Data register: return the next buffer byte. Word/dword PIO is the
            // I/O layer repeating this read at the same port (the VMM does not
            // increment the port for the IDE data register), so consecutive
            // reads yield consecutive buffer bytes — the correct PIO semantics.
            self.read_data_byte()
        } else {
            self.read_register(port)
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        if port == self.cmd_base {
            // Data register: store the next buffer byte (see `read`).
            self.write_data_byte(value);
        } else {
            self.write_register(port, value);
        }
    }
}

impl IdeController {
    /// 16-bit read of the data port (PIO `insw`). Other ports are byte-wide;
    /// reading them via this helper returns the byte zero-extended.
    pub fn read16(&mut self, port: u16) -> u16 {
        if port == self.cmd_base {
            self.read_data_word()
        } else {
            self.read_register(port) as u16
        }
    }

    /// 16-bit write of the data port (PIO `outsw`).
    pub fn write16(&mut self, port: u16, value: u16) {
        if port == self.cmd_base {
            self.write_data_word(value);
        } else {
            self.write_register(port, (value & 0xFF) as u8);
        }
    }
}

/// Encode an ASCII string into ATA "string" words. ATA string fields store
/// characters in big-endian byte order within each 16-bit word (the first
/// character occupies the high byte). The slice length determines how many
/// words (2 chars each) are written; the input is padded with spaces.
fn put_ata_string(words: &mut [u16], s: &str) {
    let bytes = s.as_bytes();
    for (i, w) in words.iter_mut().enumerate() {
        let hi = bytes.get(i * 2).copied().unwrap_or(b' ');
        let lo = bytes.get(i * 2 + 1).copied().unwrap_or(b' ');
        *w = ((hi as u16) << 8) | (lo as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 16-sector (8 KiB) controller for tests.
    fn make_ide() -> IdeController {
        IdeController::with_sectors(16)
    }

    /// Drain `n` words from the data port via the 16-bit helper.
    fn drain_words(ide: &mut IdeController, n: usize) -> Vec<u16> {
        (0..n).map(|_| ide.read16(0x1F0)).collect()
    }

    /// Decode an ATA string field back into bytes (high byte first).
    fn ata_string_bytes(words: &[u16]) -> Vec<u8> {
        let mut out = Vec::new();
        for w in words {
            out.push((w >> 8) as u8);
            out.push((w & 0xFF) as u8);
        }
        out
    }

    #[test]
    fn capacity_from_disk_length() {
        let ide = IdeController::with_sectors(2048);
        assert_eq!(ide.capacity_sectors(), 2048);
        assert_eq!(ide.disk().len(), 2048 * SECTOR_SIZE);
    }

    #[test]
    fn initial_status_drdy() {
        let ide = make_ide();
        assert_eq!(ide.status() & ST_BSY, 0, "not busy at idle");
        assert_ne!(ide.status() & ST_DRDY, 0, "ready at idle");
        assert_eq!(ide.status() & ST_DRQ, 0, "no DRQ at idle");
    }

    #[test]
    fn identify_returns_sane_block() {
        let mut ide = IdeController::with_sectors(2048);
        // Select master, LBA mode.
        ide.write(0x1F6, DH_LBA);
        // Issue IDENTIFY.
        ide.write(0x1F7, CMD_IDENTIFY);
        // DRQ must be set, data ready.
        assert_ne!(ide.status() & ST_DRQ, 0, "DRQ set after IDENTIFY");
        assert_eq!(ide.status() & ST_ERR, 0, "no error after IDENTIFY");

        let words = drain_words(&mut ide, SECTOR_WORDS);

        // After draining, DRQ clears.
        assert_eq!(ide.status() & ST_DRQ, 0, "DRQ clears after drain");

        // Word 0 general config.
        assert_eq!(words[0], 0x0040);

        // LBA28 capacity in words 60-61.
        let lba28 = (words[60] as u32) | ((words[61] as u32) << 16);
        assert_eq!(lba28, 2048, "LBA28 capacity matches");

        // LBA48 capacity in words 100-103.
        let lba48 = (words[100] as u64)
            | ((words[101] as u64) << 16)
            | ((words[102] as u64) << 32)
            | ((words[103] as u64) << 48);
        assert_eq!(lba48, 2048, "LBA48 capacity matches");

        // Model string (words 27-46) contains "RAX".
        let model = ata_string_bytes(&words[27..47]);
        let model_str = String::from_utf8_lossy(&model);
        assert!(
            model_str.contains("RAX"),
            "model string present: {model_str:?}"
        );

        // Serial (words 10-19) is non-empty / non-space.
        let serial = ata_string_bytes(&words[10..20]);
        assert!(serial.iter().any(|&b| b != b' '), "serial present");

        // LBA capability bit (word 49 bit 9).
        assert_ne!(words[49] & (1 << 9), 0, "LBA supported bit set");
        // Multiword DMA supported (word 63 low bits).
        assert_ne!(words[63] & 0x0007, 0, "MWDMA modes advertised");
        // LBA48 supported (word 83 bit 10).
        assert_ne!(words[83] & (1 << 10), 0, "LBA48 supported bit set");
    }

    #[test]
    fn identify_on_slave_aborts() {
        let mut ide = make_ide();
        ide.write(0x1F6, DH_DEV); // select slave (no device)
        ide.write(0x1F7, CMD_IDENTIFY);
        assert_ne!(ide.status() & ST_ERR, 0, "ERR set for absent slave");
        assert_ne!(ide.error & ERR_ABRT, 0, "ABRT set");
    }

    #[test]
    fn lba28_read_sector_round_trip() {
        let mut ide = make_ide();
        // Seed sector 5 directly on the backing disk with a recognizable pattern.
        let lba = 5u32;
        let pattern: Vec<u8> = (0..SECTOR_SIZE).map(|i| (i as u8) ^ 0x5A).collect();
        {
            let disk = ide.disk_mut();
            disk[lba as usize * SECTOR_SIZE..(lba as usize + 1) * SECTOR_SIZE]
                .copy_from_slice(&pattern);
        }

        // Program LBA28 read of 1 sector at LBA 5.
        ide.write(0x1F6, DH_LBA | (((lba >> 24) & 0x0F) as u8)); // drive/head + LBA top
        ide.write(0x1F2, 1); // sector count = 1
        ide.write(0x1F3, (lba & 0xFF) as u8); // LBA low
        ide.write(0x1F4, ((lba >> 8) & 0xFF) as u8); // LBA mid
        ide.write(0x1F5, ((lba >> 16) & 0xFF) as u8); // LBA high
        ide.write(0x1F7, CMD_READ_SECTORS);

        assert_ne!(ide.status() & ST_DRQ, 0, "DRQ set after READ SECTORS");
        assert_eq!(ide.status() & ST_ERR, 0, "no error");
        assert!(ide.has_pending_interrupt(), "IRQ raised when data ready");

        // Drain a sector's worth of words and reconstruct the bytes.
        let words = drain_words(&mut ide, SECTOR_WORDS);
        let mut got = Vec::with_capacity(SECTOR_SIZE);
        for w in &words {
            got.push((w & 0xFF) as u8);
            got.push((w >> 8) as u8);
        }
        assert_eq!(got, pattern, "round-tripped sector matches");
        assert_eq!(ide.status() & ST_DRQ, 0, "DRQ clears at end of transfer");
    }

    #[test]
    fn lba28_write_sector_round_trip() {
        let mut ide = make_ide();
        let lba = 7u32;
        let pattern: Vec<u8> = (0..SECTOR_SIZE)
            .map(|i| (i as u8).wrapping_mul(3))
            .collect();

        // Program LBA28 write of 1 sector at LBA 7.
        ide.write(0x1F6, DH_LBA | (((lba >> 24) & 0x0F) as u8));
        ide.write(0x1F2, 1);
        ide.write(0x1F3, (lba & 0xFF) as u8);
        ide.write(0x1F4, ((lba >> 8) & 0xFF) as u8);
        ide.write(0x1F5, ((lba >> 16) & 0xFF) as u8);
        ide.write(0x1F7, CMD_WRITE_SECTORS);

        // Device requests data: DRQ set, no error.
        assert_ne!(ide.status() & ST_DRQ, 0, "DRQ set awaiting write data");
        assert_eq!(ide.status() & ST_ERR, 0, "no error");

        // Feed a sector's worth of words.
        for chunk in pattern.chunks(2) {
            let word = (chunk[0] as u16) | ((chunk[1] as u16) << 8);
            ide.write16(0x1F0, word);
        }

        // Transfer complete: DRQ clears, completion IRQ raised.
        assert_eq!(ide.status() & ST_DRQ, 0, "DRQ clears after write");
        assert!(ide.has_pending_interrupt(), "completion IRQ raised");

        // Backing disk must hold the pattern at LBA 7.
        let off = lba as usize * SECTOR_SIZE;
        assert_eq!(&ide.disk()[off..off + SECTOR_SIZE], &pattern[..]);
    }

    #[test]
    fn multi_sector_read_round_trip() {
        let mut ide = make_ide();
        // Seed sectors 2 and 3.
        let mut expected = Vec::new();
        for s in 2u32..4 {
            let pat: Vec<u8> = (0..SECTOR_SIZE)
                .map(|i| (i as u8).wrapping_add(s as u8))
                .collect();
            let off = s as usize * SECTOR_SIZE;
            ide.disk_mut()[off..off + SECTOR_SIZE].copy_from_slice(&pat);
            expected.extend_from_slice(&pat);
        }

        ide.write(0x1F6, DH_LBA);
        ide.write(0x1F2, 2); // 2 sectors
        ide.write(0x1F3, 2); // LBA low = 2
        ide.write(0x1F4, 0);
        ide.write(0x1F5, 0);
        ide.write(0x1F7, CMD_READ_SECTORS);

        let words = drain_words(&mut ide, SECTOR_WORDS * 2);
        let mut got = Vec::new();
        for w in &words {
            got.push((w & 0xFF) as u8);
            got.push((w >> 8) as u8);
        }
        assert_eq!(got, expected, "two-sector read matches");
        assert_eq!(ide.status() & ST_DRQ, 0, "DRQ clears after 2 sectors");
    }

    #[test]
    fn read_out_of_range_aborts() {
        let mut ide = make_ide(); // 16 sectors
        ide.write(0x1F6, DH_LBA);
        ide.write(0x1F2, 1);
        ide.write(0x1F3, 200); // LBA 200 >> 16
        ide.write(0x1F4, 0);
        ide.write(0x1F5, 0);
        ide.write(0x1F7, CMD_READ_SECTORS);
        assert_ne!(ide.status() & ST_ERR, 0, "ERR set on OOB read");
        assert_ne!(ide.error & ERR_ABRT, 0, "ABRT set");
        assert_eq!(ide.status() & ST_DRQ, 0, "no DRQ on aborted read");
    }

    #[test]
    fn status_bsy_drq_transitions() {
        let mut ide = make_ide();
        // Idle: DRDY, no BSY, no DRQ.
        assert_eq!(ide.status() & ST_BSY, 0);
        assert_eq!(ide.status() & ST_DRQ, 0);
        assert_ne!(ide.status() & ST_DRDY, 0);

        // After READ: DRQ set (data ready), not BSY.
        ide.write(0x1F6, DH_LBA);
        ide.write(0x1F2, 1);
        ide.write(0x1F3, 0);
        ide.write(0x1F4, 0);
        ide.write(0x1F5, 0);
        ide.write(0x1F7, CMD_READ_SECTORS);
        assert_ne!(ide.status() & ST_DRQ, 0, "DRQ during transfer");
        assert_eq!(
            ide.status() & ST_BSY,
            0,
            "BSY not asserted in PIO-ready state"
        );

        // Drain partially: DRQ stays set until the sector is fully read.
        let _ = ide.read16(0x1F0);
        assert_ne!(ide.status() & ST_DRQ, 0, "DRQ stays until sector drained");

        // Drain the rest.
        let _ = drain_words(&mut ide, SECTOR_WORDS - 1);
        assert_eq!(ide.status() & ST_DRQ, 0, "DRQ clears after full sector");
    }

    #[test]
    fn drive_select_reflected_in_register() {
        let mut ide = make_ide();
        ide.write(0x1F6, DH_LBA | DH_DEV);
        let dh = ide.read(0x1F6);
        assert_ne!(dh & DH_DEV, 0, "DEV bit reflected");
        assert_ne!(dh & DH_LBA, 0, "LBA bit reflected");
        assert!(!ide.master_selected(), "slave selected");

        ide.write(0x1F6, DH_LBA);
        assert!(ide.master_selected(), "master selected");
    }

    #[test]
    fn srst_resets_signature() {
        let mut ide = make_ide();
        // Dirty some registers.
        ide.write(0x1F2, 0xAB);
        ide.write(0x1F3, 0xCD);
        ide.write(0x1F6, DH_DEV | DH_LBA);

        // Assert SRST (0->1) then deassert.
        ide.write(0x3F6, DC_SRST);
        ide.write(0x3F6, 0);

        // ATA reset signature: sector count 1, LBA 1/0/0, status DRDY.
        assert_eq!(ide.read(0x1F2), 0x01, "sector count signature");
        assert_eq!(ide.read(0x1F3), 0x01, "LBA low signature");
        assert_eq!(ide.read(0x1F4), 0x00, "LBA mid signature");
        assert_eq!(ide.read(0x1F5), 0x00, "LBA high signature");
        assert_ne!(ide.status() & ST_DRDY, 0, "DRDY after reset");
        assert_eq!(ide.status() & ST_BSY, 0, "not BSY after reset");
    }

    #[test]
    fn nien_masks_interrupt() {
        let mut ide = make_ide();
        // Mask interrupts via nIEN.
        ide.write(0x3F6, DC_NIEN);
        ide.write(0x1F6, DH_LBA);
        ide.write(0x1F2, 1);
        ide.write(0x1F3, 0);
        ide.write(0x1F4, 0);
        ide.write(0x1F5, 0);
        ide.write(0x1F7, CMD_READ_SECTORS);
        // Internally latched, but masked from the poll.
        assert!(!ide.has_pending_interrupt(), "nIEN masks the IRQ");

        // Unmask: pending IRQ now visible.
        ide.write(0x3F6, 0);
        assert!(ide.has_pending_interrupt(), "IRQ visible once unmasked");

        // Reading the status register acks the interrupt.
        let _ = ide.read(0x1F7);
        assert!(!ide.has_pending_interrupt(), "status read acks IRQ");
    }

    #[test]
    fn lba48_read_round_trip() {
        let mut ide = IdeController::with_sectors(64);
        let lba = 40u64;
        let pattern: Vec<u8> = (0..SECTOR_SIZE).map(|i| (i as u8) ^ 0xA5).collect();
        let off = lba as usize * SECTOR_SIZE;
        ide.disk_mut()[off..off + SECTOR_SIZE].copy_from_slice(&pattern);

        // LBA48 programming: write high bytes first, then low bytes.
        ide.write(0x1F6, DH_LBA);
        // Sector count: high then low (count = 1).
        ide.write(0x1F2, 0x00); // count high
        ide.write(0x1F2, 0x01); // count low
        // LBA low/mid/high: high bytes (bits 24-47) then low bytes (0-23).
        ide.write(0x1F3, 0x00); // LBA bits 24-31
        ide.write(0x1F4, 0x00); // LBA bits 32-39
        ide.write(0x1F5, 0x00); // LBA bits 40-47
        ide.write(0x1F3, (lba & 0xFF) as u8); // LBA bits 0-7
        ide.write(0x1F4, ((lba >> 8) & 0xFF) as u8); // LBA bits 8-15
        ide.write(0x1F5, ((lba >> 16) & 0xFF) as u8); // LBA bits 16-23
        ide.write(0x1F7, CMD_READ_SECTORS_EXT);

        assert_eq!(ide.status() & ST_ERR, 0, "no error on LBA48 read");
        assert_ne!(ide.status() & ST_DRQ, 0, "DRQ set on LBA48 read");

        let words = drain_words(&mut ide, SECTOR_WORDS);
        let mut got = Vec::new();
        for w in &words {
            got.push((w & 0xFF) as u8);
            got.push((w >> 8) as u8);
        }
        assert_eq!(got, pattern, "LBA48 round-trip matches");
    }

    #[test]
    fn write_then_read_back_via_ports() {
        let mut ide = make_ide();
        let lba = 9u32;
        let pattern: Vec<u8> = (0..SECTOR_SIZE)
            .map(|i| (i as u8).wrapping_add(0x11))
            .collect();

        // WRITE SECTORS at LBA 9.
        ide.write(0x1F6, DH_LBA);
        ide.write(0x1F2, 1);
        ide.write(0x1F3, (lba & 0xFF) as u8);
        ide.write(0x1F4, 0);
        ide.write(0x1F5, 0);
        ide.write(0x1F7, CMD_WRITE_SECTORS);
        for chunk in pattern.chunks(2) {
            ide.write16(0x1F0, (chunk[0] as u16) | ((chunk[1] as u16) << 8));
        }

        // Now READ SECTORS back at LBA 9 and compare.
        ide.write(0x1F6, DH_LBA);
        ide.write(0x1F2, 1);
        ide.write(0x1F3, (lba & 0xFF) as u8);
        ide.write(0x1F4, 0);
        ide.write(0x1F5, 0);
        ide.write(0x1F7, CMD_READ_SECTORS);
        let words = drain_words(&mut ide, SECTOR_WORDS);
        let mut got = Vec::new();
        for w in &words {
            got.push((w & 0xFF) as u8);
            got.push((w >> 8) as u8);
        }
        assert_eq!(got, pattern, "data written via ports reads back");
    }

    #[test]
    fn unknown_command_aborts() {
        let mut ide = make_ide();
        ide.write(0x1F6, DH_LBA);
        ide.write(0x1F7, 0xFD); // bogus command
        assert_ne!(ide.status() & ST_ERR, 0, "ERR on unknown command");
        assert_ne!(ide.error & ERR_ABRT, 0, "ABRT on unknown command");
    }

    #[test]
    fn secondary_channel_ports() {
        let mut ide = IdeController::new_secondary(vec![0u8; 16 * SECTOR_SIZE]);
        assert!(!ide.is_primary());
        // Identify on the secondary command block base.
        ide.write(0x176, DH_LBA);
        ide.write(0x177, CMD_IDENTIFY);
        assert_ne!(ide.status() & ST_DRQ, 0, "DRQ on secondary IDENTIFY");
        let w0 = ide.read16(0x170);
        assert_eq!(w0, 0x0040, "secondary data port serves IDENTIFY");
    }
}
