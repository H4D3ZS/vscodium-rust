//! Intel 82077AA Floppy Disk Controller (FDC) emulation.
//!
//! Models the single-chip 82077AA FDC register file at I/O ports 0x3F0-0x3F7
//! together with the command FIFO state machine that drives it. The controller
//! is presented to software through four meaningful registers:
//!
//! - 0x3F2 DOR  (Digital Output Register): drive select (bits 1:0), reset
//!   (bit 2, active low), DMA/IRQ enable (bit 3), motor-on (bits 7:4).
//! - 0x3F4 MSR  (Main Status Register, read): per-drive busy bits, command
//!   busy (CB), non-DMA (NDM), data-input/output direction (DIO) and request
//!   for master (RQM).
//! - 0x3F5 Data/FIFO: the bidirectional command/parameter/result byte stream.
//! - 0x3F7 DIR (read, disk change) / CCR (write, data-rate select).
//!
//! ## Command FIFO state machine
//!
//! Each command issued through 0x3F5 begins with a command byte that selects a
//! command requiring N parameter bytes. Once all parameters have been written
//! the controller executes and (for most commands) produces M result bytes that
//! are read back through 0x3F5, with MSR.DIO set to indicate the controller is
//! presenting data to the host. RQM gates each transfer step.
//!
//! ## Media model
//!
//! A single 1.44MB 3.5" floppy is modelled in memory as a flat `Vec<u8>` of
//! 2880 sectors x 512 bytes, addressed by CHS geometry: 80 cylinders, 2 heads,
//! 18 sectors per track (sectors numbered from 1). Data transfers move whole
//! sectors directly between the image and the FIFO (programmed-I/O style); DMA
//! wiring is intentionally out of scope for this device model.

use super::bus::IoDevice;

// --- I/O port layout (base 0x3F0) ----------------------------------------

const PORT_BASE: u16 = 0x3F0;
const PORT_SRA: u16 = 0x3F0; // Status Register A (read only, PS/2)
const PORT_SRB: u16 = 0x3F1; // Status Register B (read only, PS/2)
const PORT_DOR: u16 = 0x3F2; // Digital Output Register
const PORT_TDR: u16 = 0x3F3; // Tape Drive Register
const PORT_MSR: u16 = 0x3F4; // Main Status Register (read) / DSR (write)
const PORT_FIFO: u16 = 0x3F5; // Data / command FIFO
const PORT_DIR: u16 = 0x3F7; // Digital Input Register (read) / CCR (write)

// --- DOR bits ------------------------------------------------------------

const DOR_DRIVE_SEL: u8 = 0x03; // bits 1:0 drive select
const DOR_RESET_N: u8 = 0x04; // bit 2: 0 = reset asserted, 1 = normal
const DOR_DMA_EN: u8 = 0x08; // bit 3: DMA & IRQ enable
const DOR_MOTOR_BASE: u8 = 0x10; // bit 4 = drive 0 motor (shifts left per drive)

// --- MSR bits ------------------------------------------------------------

const MSR_DRV0_BUSY: u8 = 0x01; // bit 0: drive 0 seeking/busy
const MSR_DRV1_BUSY: u8 = 0x02;
const MSR_DRV2_BUSY: u8 = 0x04;
const MSR_DRV3_BUSY: u8 = 0x08;
const MSR_CB: u8 = 0x10; // bit 4: command busy (controller executing)
const MSR_NDM: u8 = 0x20; // bit 5: non-DMA execution phase
const MSR_DIO: u8 = 0x40; // bit 6: 1 = FDC->host (read result), 0 = host->FDC
const MSR_RQM: u8 = 0x80; // bit 7: request for master (data port ready)

// --- ST0 (Status Register 0) bits ---------------------------------------

const ST0_SEEK_END: u8 = 0x20; // SE: seek/recalibrate complete
const ST0_IC_NORMAL: u8 = 0x00; // interrupt code: normal termination
const ST0_IC_ABNORMAL: u8 = 0x40; // interrupt code: abnormal termination

// --- ST3 (Status Register 3, SENSE DRIVE STATUS) bits --------------------

const ST3_TRACK0: u8 = 0x10; // TK0: head at track 0
const ST3_READY: u8 = 0x20; // RDY: drive ready
const ST3_WRITE_PROTECT: u8 = 0x40; // WP: write protected

// --- Command opcodes (low 5 bits of the command byte) --------------------

const CMD_MASK: u8 = 0x1F;
const CMD_READ_TRACK: u8 = 0x02;
const CMD_SPECIFY: u8 = 0x03;
const CMD_SENSE_DRIVE_STATUS: u8 = 0x04;
const CMD_WRITE_DATA: u8 = 0x05;
const CMD_READ_DATA: u8 = 0x06;
const CMD_RECALIBRATE: u8 = 0x07;
const CMD_SENSE_INTERRUPT: u8 = 0x08;
const CMD_WRITE_DELETED: u8 = 0x09;
const CMD_READ_ID: u8 = 0x0A;
const CMD_READ_DELETED: u8 = 0x0C;
const CMD_FORMAT_TRACK: u8 = 0x0D;
const CMD_DUMPREG: u8 = 0x0E;
const CMD_SEEK: u8 = 0x0F;
const CMD_VERSION: u8 = 0x10;
const CMD_CONFIGURE: u8 = 0x13;

/// Value reported by the VERSION command for the 82077AA.
const VERSION_82077: u8 = 0x90;

// --- Media geometry (1.44MB 3.5") ----------------------------------------

const CYLINDERS: u8 = 80;
const HEADS: u8 = 2;
const SECTORS_PER_TRACK: u8 = 18;
const SECTOR_SIZE: usize = 512;
const TOTAL_SECTORS: usize = CYLINDERS as usize * HEADS as usize * SECTORS_PER_TRACK as usize; // 2880
const IMAGE_SIZE: usize = TOTAL_SECTORS * SECTOR_SIZE; // 1,474,560 bytes

/// Which phase of the command protocol the controller is currently in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    /// Idle: waiting for a command byte (RQM=1, DIO=0).
    Command,
    /// Collecting parameter bytes for the in-flight command (RQM=1, DIO=0).
    Parameters,
    /// Streaming sector data host->FDC (write) (RQM=1, DIO=0).
    WriteExecution,
    /// Streaming sector data FDC->host (read) (RQM=1, DIO=1).
    ReadExecution,
    /// Presenting result bytes back to the host (RQM=1, DIO=1).
    Result,
}

/// Per-drive seek state.
#[derive(Clone, Copy, Debug, Default)]
struct DriveState {
    cylinder: u8,
}

/// 82077AA floppy disk controller.
pub struct Fdc {
    // --- registers ---
    dor: u8,
    dsr: u8, // Data-rate Select Register (write side of 0x3F4)
    ccr: u8, // Configuration Control Register (write side of 0x3F7)

    // --- FIFO / protocol state ---
    phase: Phase,
    command: u8,     // command byte currently being processed
    params: Vec<u8>, // accumulated parameter bytes
    params_needed: usize,
    result: Vec<u8>, // result bytes pending read-back (front = next)
    result_pos: usize,

    // --- data transfer buffer (execution phase) ---
    xfer: Vec<u8>,
    xfer_pos: usize,
    xfer_lba: usize, // image byte offset the transfer started at (write commit)

    // --- SPECIFY / CONFIGURE retained parameters (for DUMPREG) ---
    srt_hut: u8, // step-rate / head-unload (SPECIFY byte 1)
    hlt_nd: u8,  // head-load-time / non-DMA (SPECIFY byte 2)
    config_b0: u8,
    config_b1: u8, // EIS/EFIFO/POLL/FIFOTHR
    config_b2: u8, // PRETRK

    // --- per-drive state ---
    drives: [DriveState; 4],
    end_cylinder: u8, // result CHS cylinder
    end_head: u8,
    end_sector: u8,

    // --- interrupt bookkeeping ---
    /// Number of SENSE INTERRUPT results still pending after a reset (the
    /// 82077 generates four). Decremented as SENSE INTERRUPT is issued.
    reset_senses_pending: u8,
    /// Per-drive seek-completion latch consumed by SENSE INTERRUPT STATUS.
    seek_complete: [bool; 4],
    /// IRQ6 line state (level). Set on command/seek completion or reset,
    /// cleared when consumed.
    irq_pending: bool,
    /// ST0 value latched by the last seek/recalibrate for SENSE INTERRUPT.
    last_st0: u8,

    // --- media ---
    image: Vec<u8>,
    write_protect: bool,
    disk_changed: bool,
}

impl Default for Fdc {
    fn default() -> Self {
        Self::new()
    }
}

impl Fdc {
    pub fn new() -> Self {
        Fdc {
            dor: 0,
            dsr: 0,
            ccr: 0,
            phase: Phase::Command,
            command: 0,
            params: Vec::new(),
            params_needed: 0,
            result: Vec::new(),
            result_pos: 0,
            xfer: Vec::new(),
            xfer_pos: 0,
            xfer_lba: 0,
            srt_hut: 0,
            hlt_nd: 0,
            config_b0: 0,
            config_b1: 0,
            config_b2: 0,
            drives: [DriveState::default(); 4],
            end_cylinder: 0,
            end_head: 0,
            end_sector: 1,
            reset_senses_pending: 0,
            seek_complete: [false; 4],
            irq_pending: false,
            last_st0: 0,
            image: vec![0u8; IMAGE_SIZE],
            write_protect: false,
            disk_changed: true,
        }
    }

    /// Construct an FDC with a pre-populated media image. The image is padded
    /// or truncated to the fixed 1.44MB geometry.
    pub fn with_image(mut data: Vec<u8>) -> Self {
        let mut fdc = Self::new();
        data.resize(IMAGE_SIZE, 0);
        fdc.image = data;
        fdc
    }

    /// Returns the fixed image size in bytes (1.44MB).
    pub fn image_size(&self) -> usize {
        IMAGE_SIZE
    }

    /// Borrow the raw media image.
    pub fn image(&self) -> &[u8] {
        &self.image
    }

    /// Set the write-protect state of the inserted media.
    pub fn set_write_protect(&mut self, wp: bool) {
        self.write_protect = wp;
    }

    /// Poll method: is IRQ6 currently asserted by the controller? The line is
    /// not cleared by this call (it models the level state); it is cleared when
    /// software consumes it via SENSE INTERRUPT STATUS or reads result bytes.
    pub fn irq6_pending(&self) -> bool {
        self.irq_pending
    }

    /// Explicitly acknowledge/clear the IRQ6 line (e.g. from the interrupt
    /// controller plumbing once the edge has been latched).
    pub fn clear_irq(&mut self) {
        self.irq_pending = false;
    }

    /// Currently selected drive (DOR bits 1:0).
    fn selected_drive(&self) -> usize {
        (self.dor & DOR_DRIVE_SEL) as usize
    }

    /// Is the motor for drive `n` spinning (DOR bit 4+n)?
    pub fn motor_on(&self, drive: usize) -> bool {
        if drive >= 4 {
            return false;
        }
        self.dor & (DOR_MOTOR_BASE << drive) != 0
    }

    /// Compute the linear byte offset of a CHS sector. Returns None if the
    /// address is outside the modelled geometry. Sectors are 1-based.
    fn chs_to_offset(cyl: u8, head: u8, sector: u8) -> Option<usize> {
        if cyl >= CYLINDERS || head >= HEADS || sector == 0 || sector > SECTORS_PER_TRACK {
            return None;
        }
        let lba = (cyl as usize * HEADS as usize + head as usize) * SECTORS_PER_TRACK as usize
            + (sector as usize - 1);
        Some(lba * SECTOR_SIZE)
    }

    // --- DOR handling -----------------------------------------------------

    fn write_dor(&mut self, value: u8) {
        let was_reset = self.dor & DOR_RESET_N == 0; // reset currently asserted?
        let now_reset = value & DOR_RESET_N == 0;
        self.dor = value;

        // A 0->1 transition on the RESET_N bit (i.e. coming OUT of reset)
        // triggers the controller reset sequence and pends four SENSE
        // INTERRUPT results plus an IRQ6.
        if was_reset && !now_reset {
            self.reset_controller();
        }
        // Entering reset just parks the protocol state machine.
        if now_reset {
            self.enter_idle();
        }
    }

    /// Perform a controller reset: clear protocol state, latch four pending
    /// sense-interrupt results and assert IRQ6.
    fn reset_controller(&mut self) {
        self.enter_idle();
        self.reset_senses_pending = 4;
        for d in &mut self.drives {
            d.cylinder = 0;
        }
        self.seek_complete = [false; 4];
        // After reset ST0 reports "ready changed" style polling interrupt.
        self.last_st0 = 0xC0; // IC = 11b (polling/ready change) per datasheet
        self.irq_pending = true;
    }

    /// Return the protocol machine to the idle command phase.
    fn enter_idle(&mut self) {
        self.phase = Phase::Command;
        self.params.clear();
        self.params_needed = 0;
        self.result.clear();
        self.result_pos = 0;
        self.xfer.clear();
        self.xfer_pos = 0;
    }

    // --- MSR computation --------------------------------------------------

    fn read_msr(&self) -> u8 {
        let mut msr = MSR_RQM; // data register is essentially always ready
        match self.phase {
            Phase::Command => {
                // Idle, accepting a command byte: DIO=0 (host->FDC), CB=0.
            }
            Phase::Parameters => {
                msr |= MSR_CB; // command in progress
            }
            Phase::WriteExecution => {
                msr |= MSR_CB | MSR_NDM; // host->FDC, DIO stays 0
            }
            Phase::ReadExecution => {
                msr |= MSR_CB | MSR_NDM | MSR_DIO; // FDC->host
            }
            Phase::Result => {
                msr |= MSR_CB | MSR_DIO; // FDC->host result bytes
            }
        }
        // Per-drive busy bits: reflect drives with a pending seek-complete.
        for (i, &done) in self.seek_complete.iter().enumerate() {
            if done {
                msr |= match i {
                    0 => MSR_DRV0_BUSY,
                    1 => MSR_DRV1_BUSY,
                    2 => MSR_DRV2_BUSY,
                    _ => MSR_DRV3_BUSY,
                };
            }
        }
        msr
    }

    // --- FIFO read (0x3F5) ------------------------------------------------

    fn read_fifo(&mut self) -> u8 {
        match self.phase {
            Phase::ReadExecution => {
                let byte = self.xfer.get(self.xfer_pos).copied().unwrap_or(0);
                self.xfer_pos += 1;
                if self.xfer_pos >= self.xfer.len() {
                    // Transfer finished -> move to the result phase.
                    self.begin_rw_result();
                }
                byte
            }
            Phase::Result => {
                let byte = self.result.get(self.result_pos).copied().unwrap_or(0);
                self.result_pos += 1;
                if self.result_pos >= self.result.len() {
                    // All result bytes consumed: back to idle.
                    self.enter_idle();
                }
                byte
            }
            _ => 0,
        }
    }

    // --- FIFO write (0x3F5) -----------------------------------------------

    fn write_fifo(&mut self, value: u8) {
        match self.phase {
            Phase::Command => self.start_command(value),
            Phase::Parameters => {
                self.params.push(value);
                if self.params.len() >= self.params_needed {
                    self.execute_command();
                }
            }
            Phase::WriteExecution => {
                if self.xfer_pos < self.xfer.len() {
                    self.xfer[self.xfer_pos] = value;
                    self.xfer_pos += 1;
                }
                if self.xfer_pos >= self.xfer.len() {
                    self.commit_write();
                    self.begin_rw_result();
                }
            }
            // Writes during the result/read-execution phase are ignored.
            Phase::ReadExecution | Phase::Result => {}
        }
    }

    /// Decode a freshly received command byte and set up its parameter phase.
    fn start_command(&mut self, value: u8) {
        self.command = value;
        self.params.clear();
        let cmd = value & CMD_MASK;
        self.params_needed = match cmd {
            CMD_SPECIFY => 2,
            CMD_SENSE_DRIVE_STATUS => 1,
            CMD_RECALIBRATE => 1,
            CMD_SENSE_INTERRUPT => 0,
            CMD_SEEK => 2,
            CMD_READ_DATA | CMD_WRITE_DATA | CMD_READ_DELETED | CMD_WRITE_DELETED
            | CMD_READ_TRACK => 8,
            CMD_READ_ID => 1,
            CMD_FORMAT_TRACK => 5,
            CMD_CONFIGURE => 3,
            CMD_VERSION => 0,
            CMD_DUMPREG => 0,
            _ => 0, // invalid command: handled in execute with no params
        };

        if self.params_needed == 0 {
            self.execute_command();
        } else {
            self.phase = Phase::Parameters;
        }
    }

    /// All parameter bytes present (or none required): run the command.
    fn execute_command(&mut self) {
        let cmd = self.command & CMD_MASK;
        match cmd {
            CMD_SPECIFY => self.cmd_specify(),
            CMD_SENSE_DRIVE_STATUS => self.cmd_sense_drive_status(),
            CMD_RECALIBRATE => self.cmd_recalibrate(),
            CMD_SENSE_INTERRUPT => self.cmd_sense_interrupt(),
            CMD_SEEK => self.cmd_seek(),
            CMD_READ_DATA | CMD_READ_DELETED | CMD_READ_TRACK => self.cmd_read_data(),
            CMD_WRITE_DATA | CMD_WRITE_DELETED => self.cmd_write_data(),
            CMD_READ_ID => self.cmd_read_id(),
            CMD_FORMAT_TRACK => self.cmd_format_track(),
            CMD_CONFIGURE => self.cmd_configure(),
            CMD_VERSION => self.cmd_version(),
            CMD_DUMPREG => self.cmd_dumpreg(),
            _ => self.cmd_invalid(),
        }
    }

    /// Compose ST0 for the currently selected drive and head.
    fn make_st0(&self, ic: u8, extra: u8) -> u8 {
        let drive = (self.dor & DOR_DRIVE_SEL) as u8;
        let head = self.end_head & 1;
        ic | extra | (head << 2) | drive
    }

    /// Present a result-byte vector to the host and enter the result phase.
    fn set_result(&mut self, bytes: Vec<u8>) {
        self.result = bytes;
        self.result_pos = 0;
        self.phase = Phase::Result;
    }

    /// Build the 7-byte read/write result (ST0, ST1, ST2, C, H, R, N) and
    /// switch to the result phase. Used after a data transfer completes.
    fn begin_rw_result(&mut self) {
        let st0 = self.make_st0(ST0_IC_NORMAL, 0);
        let n = 2u8; // 512-byte sectors
        let result = vec![
            st0,
            0, // ST1
            0, // ST2
            self.end_cylinder,
            self.end_head,
            self.end_sector,
            n,
        ];
        self.irq_pending = true;
        self.set_result(result);
    }

    // --- individual commands ---------------------------------------------

    fn cmd_specify(&mut self) {
        // params[0] = SRT (bits 7:4) / HUT (bits 3:0)
        // params[1] = HLT (bits 7:1) / ND (bit 0)
        self.srt_hut = self.params[0];
        self.hlt_nd = self.params[1];
        // SPECIFY produces no result bytes and no interrupt.
        self.enter_idle();
    }

    fn cmd_configure(&mut self) {
        self.config_b0 = self.params[0];
        self.config_b1 = self.params[1];
        self.config_b2 = self.params[2];
        // CONFIGURE produces no result bytes and no interrupt.
        self.enter_idle();
    }

    fn cmd_version(&mut self) {
        self.set_result(vec![VERSION_82077]);
    }

    fn cmd_dumpreg(&mut self) {
        // 10 result bytes per the 82077AA DUMPREG layout.
        let result = vec![
            self.drives[0].cylinder,
            self.drives[1].cylinder,
            self.drives[2].cylinder,
            self.drives[3].cylinder,
            self.srt_hut,
            self.hlt_nd,
            self.end_sector, // EOT (last used) - representative
            self.config_b1,  // PERP/lock combined field (representative)
            self.config_b1,  // CONFIGURE byte
            self.config_b2,  // PRETRK
        ];
        self.set_result(result);
    }

    fn cmd_sense_drive_status(&mut self) {
        // params[0]: bit 2 = head, bits 1:0 = drive select.
        let p = self.params[0];
        let drive = (p & 0x03) as usize;
        let head = (p >> 2) & 1;
        let mut st3 = (head << 2) | (drive as u8);
        st3 |= ST3_READY; // drive always ready in this model
        if self.drives[drive].cylinder == 0 {
            st3 |= ST3_TRACK0;
        }
        if self.write_protect {
            st3 |= ST3_WRITE_PROTECT;
        }
        // SENSE DRIVE STATUS returns ST3 and generates no interrupt.
        self.set_result(vec![st3]);
    }

    fn cmd_recalibrate(&mut self) {
        // params[0]: bits 1:0 = drive select.
        let drive = (self.params[0] & 0x03) as usize;
        self.drives[drive].cylinder = 0;
        self.end_cylinder = 0;
        self.end_head = 0;
        self.seek_complete[drive] = true;
        self.last_st0 = ST0_IC_NORMAL | ST0_SEEK_END | drive as u8;
        self.irq_pending = true;
        // Recalibrate has no result phase; results read via SENSE INTERRUPT.
        self.enter_idle();
    }

    fn cmd_seek(&mut self) {
        // params[0]: bit 2 = head, bits 1:0 = drive; params[1] = cylinder.
        let p = self.params[0];
        let drive = (p & 0x03) as usize;
        let head = (p >> 2) & 1;
        let cyl = self.params[1];
        let target = cyl.min(CYLINDERS - 1);
        self.drives[drive].cylinder = target;
        self.end_cylinder = target;
        self.end_head = head;
        self.seek_complete[drive] = true;
        self.last_st0 = ST0_IC_NORMAL | ST0_SEEK_END | (head << 2) | drive as u8;
        self.irq_pending = true;
        // SEEK has no result phase; results read via SENSE INTERRUPT.
        self.enter_idle();
    }

    fn cmd_sense_interrupt(&mut self) {
        // After a reset the 82077 reports four polling interrupts: ST0 with
        // IC = 11b and an incrementing drive number, PCN = 0.
        if self.reset_senses_pending > 0 {
            let idx = 4 - self.reset_senses_pending;
            self.reset_senses_pending -= 1;
            let st0 = 0xC0 | idx; // IC=11b, drive = idx
            let pcn = 0u8;
            if self.reset_senses_pending == 0 {
                self.irq_pending = false;
            }
            self.set_result(vec![st0, pcn]);
            return;
        }

        // Otherwise report the last seek/recalibrate completion if any.
        let mut serviced = None;
        for d in 0..4 {
            if self.seek_complete[d] {
                serviced = Some(d);
                break;
            }
        }

        match serviced {
            Some(d) => {
                self.seek_complete[d] = false;
                let st0 = self.last_st0;
                let pcn = self.drives[d].cylinder;
                self.irq_pending = false;
                self.set_result(vec![st0, pcn]);
            }
            None => {
                // No interrupt pending: report invalid (ST0 = 0x80).
                self.irq_pending = false;
                self.set_result(vec![0x80]);
            }
        }
    }

    fn cmd_read_id(&mut self) {
        // params[0]: bit 2 = head, bits 1:0 = drive.
        let p = self.params[0];
        let drive = (p & 0x03) as usize;
        let head = (p >> 2) & 1;
        let cyl = self.drives[drive].cylinder;
        self.end_cylinder = cyl;
        self.end_head = head;
        self.end_sector = 1; // first sector of the track
        let st0 = self.make_st0(ST0_IC_NORMAL, 0);
        let result = vec![st0, 0, 0, cyl, head, 1, 2];
        self.irq_pending = true;
        self.set_result(result);
    }

    fn cmd_read_data(&mut self) {
        // params: [0]=HD/DS, [1]=C, [2]=H, [3]=R, [4]=N, [5]=EOT, [6]=GPL, [7]=DTL
        let head = (self.params[0] >> 2) & 1;
        let cyl = self.params[1];
        let r = self.params[3];
        let eot = self.params[5];
        let start = match Self::chs_to_offset(cyl, head, r) {
            Some(off) => off,
            None => {
                let st0 = self.make_st0(ST0_IC_ABNORMAL, 0);
                self.irq_pending = true;
                self.set_result(vec![st0, 0x04, 0, cyl, head, r, 2]);
                return;
            }
        };

        // Number of sectors to read: from R up to and including EOT.
        let last = eot.max(r);
        let count = (last - r + 1) as usize;
        let count = count.min(SECTORS_PER_TRACK as usize); // clamp to a track
        let mut buf = Vec::with_capacity(count * SECTOR_SIZE);
        for i in 0..count {
            let off = start + i * SECTOR_SIZE;
            if off + SECTOR_SIZE <= self.image.len() {
                buf.extend_from_slice(&self.image[off..off + SECTOR_SIZE]);
            } else {
                buf.extend(std::iter::repeat(0).take(SECTOR_SIZE));
            }
        }

        // Result CHS advances to the sector after the last one transferred.
        self.end_cylinder = cyl;
        self.end_head = head;
        self.end_sector = r + count as u8;

        self.xfer = buf;
        self.xfer_pos = 0;
        self.phase = Phase::ReadExecution;
        if self.xfer.is_empty() {
            self.begin_rw_result();
        }
    }

    fn cmd_write_data(&mut self) {
        let head = (self.params[0] >> 2) & 1;
        let cyl = self.params[1];
        let r = self.params[3];
        let eot = self.params[5];
        let start = match Self::chs_to_offset(cyl, head, r) {
            Some(off) => off,
            None => {
                let st0 = self.make_st0(ST0_IC_ABNORMAL, 0);
                self.irq_pending = true;
                self.set_result(vec![st0, 0x04, 0, cyl, head, r, 2]);
                return;
            }
        };

        if self.write_protect {
            let st0 = self.make_st0(ST0_IC_ABNORMAL, 0);
            // ST1 bit 1 = NW (not writable / write protected).
            self.irq_pending = true;
            self.set_result(vec![st0, 0x02, 0, cyl, head, r, 2]);
            return;
        }

        let last = eot.max(r);
        let count = ((last - r + 1) as usize).min(SECTORS_PER_TRACK as usize);

        self.end_cylinder = cyl;
        self.end_head = head;
        self.end_sector = r + count as u8;

        // Allocate a host->FDC staging buffer; data is committed to the image
        // once the final byte arrives.
        self.xfer = vec![0u8; count * SECTOR_SIZE];
        self.xfer_pos = 0;
        self.xfer_lba = start;
        self.phase = Phase::WriteExecution;
        if self.xfer.is_empty() {
            self.commit_write();
            self.begin_rw_result();
        }
    }

    /// Copy the staged write buffer into the media image.
    fn commit_write(&mut self) {
        let start = self.xfer_lba;
        let end = (start + self.xfer.len()).min(self.image.len());
        let n = end - start;
        self.image[start..end].copy_from_slice(&self.xfer[..n]);
    }

    fn cmd_format_track(&mut self) {
        // params: [0]=HD/DS, [1]=N, [2]=SC, [3]=GPL, [4]=D (fill byte).
        let head = (self.params[0] >> 2) & 1;
        let drive = (self.params[0] & 0x03) as usize;
        let fill = self.params[4];
        let cyl = self.drives[drive].cylinder;
        if let Some(start) = Self::chs_to_offset(cyl, head, 1) {
            let track_bytes = SECTORS_PER_TRACK as usize * SECTOR_SIZE;
            let end = (start + track_bytes).min(self.image.len());
            for b in &mut self.image[start..end] {
                *b = fill;
            }
        }
        self.end_cylinder = cyl;
        self.end_head = head;
        self.end_sector = 1;
        let st0 = self.make_st0(ST0_IC_NORMAL, 0);
        self.irq_pending = true;
        self.set_result(vec![st0, 0, 0, cyl, head, 1, 2]);
    }

    fn cmd_invalid(&mut self) {
        // Invalid command: ST0 = 0x80 (IC = 11b, invalid).
        self.set_result(vec![0x80]);
    }

    // --- DIR / CCR --------------------------------------------------------

    fn read_dir(&self) -> u8 {
        // Bit 7 = disk change line. Other bits read as 0 in this model.
        if self.disk_changed { 0x80 } else { 0x00 }
    }
}

impl IoDevice for Fdc {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            PORT_SRA => 0x00, // PS/2 Status Register A (not modelled)
            PORT_SRB => 0x00, // PS/2 Status Register B (not modelled)
            PORT_DOR => self.dor,
            PORT_TDR => 0x00,
            PORT_MSR => self.read_msr(),
            PORT_FIFO => self.read_fifo(),
            PORT_DIR => self.read_dir(),
            _ => 0xFF,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        match port {
            PORT_DOR => self.write_dor(value),
            PORT_TDR => {}
            PORT_MSR => self.dsr = value, // write side of 0x3F4 is the DSR
            PORT_FIFO => self.write_fifo(value),
            PORT_DIR => self.ccr = value, // write side of 0x3F7 is the CCR
            _ => {}
        }
        let _ = PORT_BASE; // base documented for the register map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Issue the standard "exit reset" DOR sequence: assert reset, then bring
    /// the controller out of reset with DMA/IRQ enabled and drive 0 motor on.
    fn out_of_reset(fdc: &mut Fdc) {
        IoDevice::write(fdc, PORT_DOR, 0x00); // reset asserted (RESET_N low)
        IoDevice::write(fdc, PORT_DOR, DOR_RESET_N | DOR_DMA_EN | DOR_MOTOR_BASE);
    }

    fn cmd(fdc: &mut Fdc, byte: u8) {
        IoDevice::write(fdc, PORT_FIFO, byte);
    }

    fn read_result(fdc: &mut Fdc) -> Vec<u8> {
        let mut out = Vec::new();
        // Drain while the controller is presenting result/exec data (DIO=1).
        while IoDevice::read(fdc, PORT_MSR) & (MSR_DIO | MSR_RQM) == (MSR_DIO | MSR_RQM) {
            out.push(IoDevice::read(fdc, PORT_FIFO));
        }
        out
    }

    // ---- VERSION ----

    #[test]
    fn version_returns_0x90() {
        let mut fdc = Fdc::new();
        cmd(&mut fdc, CMD_VERSION); // 0x10
        // After issuing VERSION, controller presents one result byte.
        assert_eq!(IoDevice::read(&mut fdc, PORT_MSR) & MSR_DIO, MSR_DIO);
        let result = read_result(&mut fdc);
        assert_eq!(result, vec![0x90]);
    }

    // ---- Reset + SENSE INTERRUPT ----

    #[test]
    fn reset_pends_four_sense_interrupts_and_irq() {
        let mut fdc = Fdc::new();
        out_of_reset(&mut fdc);
        assert!(fdc.irq6_pending(), "reset should assert IRQ6");
        assert_eq!(fdc.reset_senses_pending, 4);

        // Four SENSE INTERRUPT commands each return ST0 (IC=11b) + PCN.
        for i in 0..4u8 {
            cmd(&mut fdc, CMD_SENSE_INTERRUPT); // 0x08
            let r = read_result(&mut fdc);
            assert_eq!(r.len(), 2, "sense interrupt returns ST0 + PCN");
            assert_eq!(r[0] & 0xC0, 0xC0, "IC = 11b after reset");
            assert_eq!(r[0] & 0x03, i, "drive index increments");
            assert_eq!(r[1], 0, "PCN = 0 after reset");
        }
        // IRQ cleared once all four pending senses consumed.
        assert!(!fdc.irq6_pending());
        assert_eq!(fdc.reset_senses_pending, 0);
    }

    // ---- SPECIFY / CONFIGURE accept parameters ----

    #[test]
    fn specify_accepts_params() {
        let mut fdc = Fdc::new();
        cmd(&mut fdc, CMD_SPECIFY); // 0x03
        // Two parameter bytes, MSR must keep RQM and DIO=0 (host->FDC).
        assert_eq!(
            IoDevice::read(&mut fdc, PORT_MSR) & (MSR_RQM | MSR_DIO),
            MSR_RQM
        );
        cmd(&mut fdc, 0xDF); // SRT/HUT
        cmd(&mut fdc, 0x02); // HLT/ND
        assert_eq!(fdc.srt_hut, 0xDF);
        assert_eq!(fdc.hlt_nd, 0x02);
        // No result phase: back to idle command phase (DIO=0).
        assert_eq!(fdc.phase, Phase::Command);
    }

    #[test]
    fn configure_accepts_params() {
        let mut fdc = Fdc::new();
        cmd(&mut fdc, CMD_CONFIGURE); // 0x13
        cmd(&mut fdc, 0x00);
        cmd(&mut fdc, 0x57); // EIS|EFIFO|threshold
        cmd(&mut fdc, 0x00); // PRETRK
        assert_eq!(fdc.config_b1, 0x57);
        assert_eq!(fdc.phase, Phase::Command);
    }

    // ---- RECALIBRATE / SEEK + SENSE INTERRUPT ----

    #[test]
    fn recalibrate_then_sense_interrupt() {
        let mut fdc = Fdc::new();
        // Move drive 0 off track 0 first.
        fdc.drives[0].cylinder = 40;
        cmd(&mut fdc, CMD_RECALIBRATE); // 0x07
        cmd(&mut fdc, 0x00); // drive 0
        assert!(fdc.irq6_pending());
        assert_eq!(fdc.drives[0].cylinder, 0);
        cmd(&mut fdc, CMD_SENSE_INTERRUPT);
        let r = read_result(&mut fdc);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0] & ST0_SEEK_END, ST0_SEEK_END, "seek end set");
        assert_eq!(r[1], 0, "PCN back at track 0");
        assert!(!fdc.irq6_pending());
    }

    #[test]
    fn seek_updates_cylinder() {
        let mut fdc = Fdc::new();
        cmd(&mut fdc, CMD_SEEK); // 0x0F
        cmd(&mut fdc, 0x00); // head 0 / drive 0
        cmd(&mut fdc, 35); // cylinder 35
        assert_eq!(fdc.drives[0].cylinder, 35);
        assert!(fdc.irq6_pending());
        cmd(&mut fdc, CMD_SENSE_INTERRUPT);
        let r = read_result(&mut fdc);
        assert_eq!(r[1], 35, "PCN reflects new cylinder");
    }

    // ---- READ ID returns CHS ----

    #[test]
    fn read_id_returns_chs() {
        let mut fdc = Fdc::new();
        // Seek drive 0 to cylinder 5 first.
        cmd(&mut fdc, CMD_SEEK);
        cmd(&mut fdc, 0x00);
        cmd(&mut fdc, 5);
        cmd(&mut fdc, CMD_SENSE_INTERRUPT);
        let _ = read_result(&mut fdc);

        cmd(&mut fdc, CMD_READ_ID); // 0x0A
        cmd(&mut fdc, 0x04); // head 1, drive 0
        let r = read_result(&mut fdc);
        // ST0, ST1, ST2, C, H, R, N
        assert_eq!(r.len(), 7);
        assert_eq!(r[3], 5, "cylinder");
        assert_eq!(r[4], 1, "head");
        assert_eq!(r[5], 1, "sector (R)");
        assert_eq!(r[6], 2, "N = 512-byte sectors");
    }

    // ---- READ DATA round trip ----

    #[test]
    fn read_data_round_trip() {
        // Build an image with a recognisable pattern at CHS (3, 1, 7).
        let mut image = vec![0u8; IMAGE_SIZE];
        let off = Fdc::chs_to_offset(3, 1, 7).unwrap();
        for i in 0..SECTOR_SIZE {
            image[off + i] = (i as u8).wrapping_add(0xA0);
        }
        let mut fdc = Fdc::with_image(image);
        out_of_reset(&mut fdc);
        for _ in 0..4 {
            cmd(&mut fdc, CMD_SENSE_INTERRUPT);
            let _ = read_result(&mut fdc);
        }

        // READ DATA single sector at C=3 H=1 R=7, EOT=7.
        cmd(&mut fdc, 0x06); // READ DATA (MFM bit normally set: 0xE6 also ok)
        cmd(&mut fdc, 0x04); // HD=1 (head 1), DS=0
        cmd(&mut fdc, 3); // C
        cmd(&mut fdc, 1); // H
        cmd(&mut fdc, 7); // R
        cmd(&mut fdc, 2); // N
        cmd(&mut fdc, 7); // EOT
        cmd(&mut fdc, 0x1B); // GPL
        cmd(&mut fdc, 0xFF); // DTL

        // Execution phase: DIO=1 (FDC->host). Read 512 data bytes.
        let mut data = Vec::new();
        for _ in 0..SECTOR_SIZE {
            assert_eq!(IoDevice::read(&mut fdc, PORT_MSR) & MSR_DIO, MSR_DIO);
            data.push(IoDevice::read(&mut fdc, PORT_FIFO));
        }
        for i in 0..SECTOR_SIZE {
            assert_eq!(data[i], (i as u8).wrapping_add(0xA0), "byte {i}");
        }
        // Then 7 result bytes.
        let r = read_result(&mut fdc);
        assert_eq!(r.len(), 7);
        assert_eq!(r[0] & 0xC0, ST0_IC_NORMAL, "normal termination");
        assert_eq!(r[3], 3, "result cylinder");
    }

    // ---- WRITE DATA round trip ----

    #[test]
    fn write_data_round_trip() {
        let mut fdc = Fdc::new();
        out_of_reset(&mut fdc);
        for _ in 0..4 {
            cmd(&mut fdc, CMD_SENSE_INTERRUPT);
            let _ = read_result(&mut fdc);
        }

        cmd(&mut fdc, 0x05); // WRITE DATA
        cmd(&mut fdc, 0x00); // head 0, drive 0
        cmd(&mut fdc, 10); // C
        cmd(&mut fdc, 0); // H
        cmd(&mut fdc, 3); // R
        cmd(&mut fdc, 2); // N
        cmd(&mut fdc, 3); // EOT (single sector)
        cmd(&mut fdc, 0x1B); // GPL
        cmd(&mut fdc, 0xFF); // DTL

        // Execution phase: DIO=0 (host->FDC). Stream 512 bytes.
        for i in 0..SECTOR_SIZE {
            let msr = IoDevice::read(&mut fdc, PORT_MSR);
            assert_eq!(msr & MSR_RQM, MSR_RQM);
            assert_eq!(msr & MSR_DIO, 0, "write execution DIO=0");
            IoDevice::write(&mut fdc, PORT_FIFO, (i as u8) ^ 0x5A);
        }
        // Result phase appears.
        let r = read_result(&mut fdc);
        assert_eq!(r.len(), 7);

        // Verify the bytes landed in the image at CHS (10,0,3).
        let off = Fdc::chs_to_offset(10, 0, 3).unwrap();
        for i in 0..SECTOR_SIZE {
            assert_eq!(fdc.image()[off + i], (i as u8) ^ 0x5A, "image byte {i}");
        }
    }

    #[test]
    fn write_then_read_back_same_sector() {
        let mut fdc = Fdc::new();
        // Write a sector.
        cmd(&mut fdc, 0x05);
        cmd(&mut fdc, 0x04); // head 1
        cmd(&mut fdc, 20);
        cmd(&mut fdc, 1);
        cmd(&mut fdc, 9);
        cmd(&mut fdc, 2);
        cmd(&mut fdc, 9);
        cmd(&mut fdc, 0x1B);
        cmd(&mut fdc, 0xFF);
        for i in 0..SECTOR_SIZE {
            IoDevice::write(&mut fdc, PORT_FIFO, (i as u8).wrapping_mul(3));
        }
        let _ = read_result(&mut fdc);

        // Read it back.
        cmd(&mut fdc, 0x06);
        cmd(&mut fdc, 0x04);
        cmd(&mut fdc, 20);
        cmd(&mut fdc, 1);
        cmd(&mut fdc, 9);
        cmd(&mut fdc, 2);
        cmd(&mut fdc, 9);
        cmd(&mut fdc, 0x1B);
        cmd(&mut fdc, 0xFF);
        let mut data = Vec::new();
        for _ in 0..SECTOR_SIZE {
            data.push(IoDevice::read(&mut fdc, PORT_FIFO));
        }
        let _ = read_result(&mut fdc);
        for i in 0..SECTOR_SIZE {
            assert_eq!(data[i], (i as u8).wrapping_mul(3), "round-trip byte {i}");
        }
    }

    // ---- SENSE DRIVE STATUS ----

    #[test]
    fn sense_drive_status_reports_track0_and_ready() {
        let mut fdc = Fdc::new();
        cmd(&mut fdc, CMD_SENSE_DRIVE_STATUS); // 0x04
        cmd(&mut fdc, 0x00); // drive 0, head 0
        let r = read_result(&mut fdc);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0] & ST3_READY, ST3_READY, "ready");
        assert_eq!(r[0] & ST3_TRACK0, ST3_TRACK0, "track 0 at cylinder 0");

        // Move off track 0 -> TK0 clears.
        fdc.drives[0].cylinder = 12;
        cmd(&mut fdc, CMD_SENSE_DRIVE_STATUS);
        cmd(&mut fdc, 0x00);
        let r = read_result(&mut fdc);
        assert_eq!(r[0] & ST3_TRACK0, 0, "not at track 0");
    }

    #[test]
    fn sense_drive_status_reports_write_protect() {
        let mut fdc = Fdc::new();
        fdc.set_write_protect(true);
        cmd(&mut fdc, CMD_SENSE_DRIVE_STATUS);
        cmd(&mut fdc, 0x00);
        let r = read_result(&mut fdc);
        assert_eq!(r[0] & ST3_WRITE_PROTECT, ST3_WRITE_PROTECT);
    }

    // ---- DUMPREG ----

    #[test]
    fn dumpreg_returns_ten_bytes() {
        let mut fdc = Fdc::new();
        cmd(&mut fdc, CMD_SPECIFY);
        cmd(&mut fdc, 0xCF);
        cmd(&mut fdc, 0x06);
        cmd(&mut fdc, CMD_DUMPREG); // 0x0E
        let r = read_result(&mut fdc);
        assert_eq!(r.len(), 10);
        assert_eq!(r[4], 0xCF, "SRT/HUT echoed");
        assert_eq!(r[5], 0x06, "HLT/ND echoed");
    }

    // ---- MSR RQM/DIO transitions ----

    #[test]
    fn msr_rqm_dio_transitions() {
        let mut fdc = Fdc::new();
        // Idle: RQM=1, DIO=0.
        let msr = IoDevice::read(&mut fdc, PORT_MSR);
        assert_eq!(msr & MSR_RQM, MSR_RQM);
        assert_eq!(msr & MSR_DIO, 0);
        assert_eq!(msr & MSR_CB, 0, "not busy when idle");

        // Issue a command needing params -> CB set, DIO still 0.
        cmd(&mut fdc, CMD_SPECIFY);
        let msr = IoDevice::read(&mut fdc, PORT_MSR);
        assert_eq!(msr & MSR_CB, MSR_CB, "busy collecting params");
        assert_eq!(msr & MSR_DIO, 0, "params are host->FDC");

        cmd(&mut fdc, 0x00);
        cmd(&mut fdc, 0x00);
        // SPECIFY ends without a result phase -> idle again.
        let msr = IoDevice::read(&mut fdc, PORT_MSR);
        assert_eq!(msr & MSR_CB, 0);

        // VERSION -> result phase: DIO flips to 1.
        cmd(&mut fdc, CMD_VERSION);
        let msr = IoDevice::read(&mut fdc, PORT_MSR);
        assert_eq!(msr & MSR_DIO, MSR_DIO, "result phase DIO=1");
        assert_eq!(msr & MSR_CB, MSR_CB);
        let _ = IoDevice::read(&mut fdc, PORT_FIFO);
        // After consuming the lone result byte -> idle.
        let msr = IoDevice::read(&mut fdc, PORT_MSR);
        assert_eq!(msr & MSR_DIO, 0);
        assert_eq!(msr & MSR_CB, 0);
    }

    // ---- Motor / drive select via DOR ----

    #[test]
    fn dor_motor_and_drive_select() {
        let mut fdc = Fdc::new();
        // Select drive 2, motor 2 on.
        IoDevice::write(
            &mut fdc,
            PORT_DOR,
            DOR_RESET_N | 0x02 | (DOR_MOTOR_BASE << 2),
        );
        assert_eq!(fdc.selected_drive(), 2);
        assert!(fdc.motor_on(2));
        assert!(!fdc.motor_on(0));
        assert!(!fdc.motor_on(1));

        // Read DOR back.
        assert_eq!(
            IoDevice::read(&mut fdc, PORT_DOR),
            DOR_RESET_N | 0x02 | (DOR_MOTOR_BASE << 2)
        );

        // Turn all motors off, select drive 1.
        IoDevice::write(&mut fdc, PORT_DOR, DOR_RESET_N | 0x01);
        assert_eq!(fdc.selected_drive(), 1);
        assert!(!fdc.motor_on(2));
    }

    // ---- CHS geometry helper ----

    #[test]
    fn chs_to_offset_geometry() {
        assert_eq!(Fdc::chs_to_offset(0, 0, 1), Some(0));
        // Second sector follows the first.
        assert_eq!(Fdc::chs_to_offset(0, 0, 2), Some(SECTOR_SIZE));
        // Head 1 sector 1 follows the whole head-0 track.
        assert_eq!(
            Fdc::chs_to_offset(0, 1, 1),
            Some(SECTORS_PER_TRACK as usize * SECTOR_SIZE)
        );
        // Last valid sector.
        assert!(Fdc::chs_to_offset(79, 1, 18).is_some());
        // Out of range cases.
        assert_eq!(Fdc::chs_to_offset(80, 0, 1), None);
        assert_eq!(Fdc::chs_to_offset(0, 2, 1), None);
        assert_eq!(Fdc::chs_to_offset(0, 0, 0), None);
        assert_eq!(Fdc::chs_to_offset(0, 0, 19), None);
    }

    #[test]
    fn image_size_is_1_44mb() {
        let fdc = Fdc::new();
        assert_eq!(fdc.image_size(), 1_474_560);
        assert_eq!(fdc.image().len(), 1_474_560);
    }

    #[test]
    fn read_data_out_of_range_aborts() {
        let mut fdc = Fdc::new();
        cmd(&mut fdc, 0x06);
        cmd(&mut fdc, 0x00);
        cmd(&mut fdc, 99); // invalid cylinder
        cmd(&mut fdc, 0);
        cmd(&mut fdc, 1);
        cmd(&mut fdc, 2);
        cmd(&mut fdc, 1);
        cmd(&mut fdc, 0x1B);
        cmd(&mut fdc, 0xFF);
        let r = read_result(&mut fdc);
        assert_eq!(r.len(), 7);
        assert_eq!(r[0] & 0x40, 0x40, "abnormal termination IC");
    }

    #[test]
    fn invalid_command_returns_st0_invalid() {
        let mut fdc = Fdc::new();
        cmd(&mut fdc, 0x01); // not a valid FDC command
        let r = read_result(&mut fdc);
        assert_eq!(r, vec![0x80]);
    }
}
