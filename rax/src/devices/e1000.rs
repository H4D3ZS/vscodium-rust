//! Intel 82540EM Gigabit Ethernet controller ("e1000") device model.
//!
//! This implements the BAR0 MMIO register file of the 82540EM as an
//! [`MmioDevice`]. The register window is 128 KiB; register accesses are
//! 32-bit and little-endian. Guest physical memory (for descriptor rings and
//! frame buffers) is accessed through a small [`Mem`] abstraction modeled on
//! the virtio transport so the descriptor-ring logic is unit-testable against a
//! plain `Vec<u8>` backing store.
//!
//! ## What is implemented
//!
//! * Core control/status registers: `CTRL`, `STATUS`.
//! * EEPROM access: `EECD` and the `EERD` read protocol. A 64-word EEPROM image
//!   is initialised with a programmed MAC address in words 0..3, and the guest
//!   reads the MAC out via `EERD`.
//! * Interrupt registers: `ICR`, `ICS`, `IMS`, `IMC`, with masking semantics and
//!   read-to-clear `ICR`.
//! * Receive/transmit control: `RCTL`, `TCTL` (including the enable bits).
//! * Receive descriptor ring: `RDBAL`/`RDBAH`/`RDLEN`/`RDH`/`RDT`.
//! * Transmit descriptor ring: `TDBAL`/`TDBAH`/`TDLEN`/`TDH`/`TDT`.
//! * The 128-entry Multicast Table Array (`MTA`).
//! * Receive Address registers `RAL0`/`RAH0` (the MAC the EEPROM is shadowed
//!   into on reset).
//! * TX descriptor processing: writing `TDT` walks the descriptor ring from
//!   `TDH` to `TDT`, reads legacy and TCP/IP-context + data descriptors via the
//!   [`Mem`] abstraction, reassembles frames spanning multiple descriptors
//!   (honouring the EOP bit), writes back the descriptor done status, and pushes
//!   each completed frame onto an output queue. The transmit-descriptor-done
//!   interrupt cause (`TXDW`) is raised in `ICR`.
//! * RX path: [`Self::receive_frame`] injects an externally supplied frame into
//!   the next free RX descriptor, copies it into the guest buffer via [`Mem`],
//!   writes back the descriptor length / status (DD | EOP), advances `RDH`, and
//!   raises the `RXT0` interrupt cause.
//! * Interrupt mask gating: a cause asserted in `ICR` only produces a pending
//!   interrupt when the corresponding `IMS` bit is set; `IMC` clears mask bits.
//!
//! ## What is deferred
//!
//! * Actual MSI/MSI-X / legacy line interrupt delivery to an interrupt
//!   controller: this model only computes the pending-interrupt condition and
//!   exposes it via [`Self::interrupt_pending`]. Wiring it to the VMM's IRQ line
//!   is left to the integration layer.
//! * Hardware checksum/segmentation offload: TCP/IP context descriptors are
//!   parsed and skipped, but no checksum insertion or TSO is performed.
//! * Receive-side flow control, VLAN tag stripping/insertion, the packet split
//!   ("extended") descriptor formats, RSS, and statistics counters beyond what
//!   is needed for the tests.
//! * PHY/MDIO management registers (`MDIC`), the second EEPROM-shadowed receive
//!   address pair beyond RAL0/RAH0, and the flash interface.

use crate::devices::bus::MmioDevice;

/// Abstraction over guest physical memory used by the descriptor-ring code.
///
/// Mirrors the virtio transport's `Mem` trait so the DMA-driven TX/RX paths can
/// be exercised against a flat `Vec<u8>` in unit tests. A real VMM implements
/// this over its guest-memory mapping.
pub trait Mem {
    /// Read bytes starting at guest physical address `gpa` into `buf`.
    /// Returns false on an out-of-bounds access.
    fn read(&self, gpa: u64, buf: &mut [u8]) -> bool;
    /// Write `buf` to guest physical address `gpa`. Returns false on an
    /// out-of-bounds access.
    fn write(&mut self, gpa: u64, buf: &[u8]) -> bool;
}

/// A flat little-endian byte buffer used as guest memory in tests. GPA 0 maps
/// to byte 0 of the buffer.
pub struct VecMem {
    pub bytes: Vec<u8>,
}

impl VecMem {
    pub fn new(size: usize) -> Self {
        VecMem {
            bytes: vec![0u8; size],
        }
    }
}

impl Mem for VecMem {
    fn read(&self, gpa: u64, buf: &mut [u8]) -> bool {
        let start = gpa as usize;
        let end = match start.checked_add(buf.len()) {
            Some(e) => e,
            None => return false,
        };
        if end > self.bytes.len() {
            return false;
        }
        buf.copy_from_slice(&self.bytes[start..end]);
        true
    }

    fn write(&mut self, gpa: u64, buf: &[u8]) -> bool {
        let start = gpa as usize;
        let end = match start.checked_add(buf.len()) {
            Some(e) => e,
            None => return false,
        };
        if end > self.bytes.len() {
            return false;
        }
        self.bytes[start..end].copy_from_slice(buf);
        true
    }
}

// ---- Register offsets (BAR0, byte offsets) ---------------------------------

/// Device Control.
pub const CTRL: u64 = 0x0000;
/// Device Status.
pub const STATUS: u64 = 0x0008;
/// EEPROM/Flash Control & Data.
pub const EECD: u64 = 0x0010;
/// EEPROM Read.
pub const EERD: u64 = 0x0014;
/// Interrupt Cause Read (read-to-clear).
pub const ICR: u64 = 0x00C0;
/// Interrupt Cause Set.
pub const ICS: u64 = 0x00C8;
/// Interrupt Mask Set/Read.
pub const IMS: u64 = 0x00D0;
/// Interrupt Mask Clear.
pub const IMC: u64 = 0x00D8;
/// Receive Control.
pub const RCTL: u64 = 0x0100;
/// Transmit Control.
pub const TCTL: u64 = 0x0400;
/// Receive Descriptor Base Address Low.
pub const RDBAL: u64 = 0x2800;
/// Receive Descriptor Base Address High.
pub const RDBAH: u64 = 0x2804;
/// Receive Descriptor Length.
pub const RDLEN: u64 = 0x2808;
/// Receive Descriptor Head.
pub const RDH: u64 = 0x2810;
/// Receive Descriptor Tail.
pub const RDT: u64 = 0x2818;
/// Transmit Descriptor Base Address Low.
pub const TDBAL: u64 = 0x3800;
/// Transmit Descriptor Base Address High.
pub const TDBAH: u64 = 0x3804;
/// Transmit Descriptor Length.
pub const TDLEN: u64 = 0x3808;
/// Transmit Descriptor Head.
pub const TDH: u64 = 0x3810;
/// Transmit Descriptor Tail.
pub const TDT: u64 = 0x3818;
/// Receive Address Low (entry 0).
pub const RAL0: u64 = 0x5400;
/// Receive Address High (entry 0).
pub const RAH0: u64 = 0x5404;
/// Multicast Table Array base (128 32-bit entries: 0x5200..0x5400).
pub const MTA_BASE: u64 = 0x5200;
/// Number of MTA entries.
pub const MTA_LEN: usize = 128;

/// Size of the BAR0 MMIO register window (128 KiB).
pub const MMIO_SIZE: u64 = 128 * 1024;

// ---- Register bit definitions ----------------------------------------------

/// `EERD.START`: begin an EEPROM read.
pub const EERD_START: u32 = 1 << 0;
/// `EERD.DONE`: EEPROM read completed (82540 uses bit 4).
pub const EERD_DONE: u32 = 1 << 4;
/// Shift for the EEPROM word address in `EERD`.
pub const EERD_ADDR_SHIFT: u32 = 8;
/// Mask for the EEPROM word address in `EERD` (8 bits on the 82540).
pub const EERD_ADDR_MASK: u32 = 0xFF;
/// Shift for the read-out EEPROM data in `EERD`.
pub const EERD_DATA_SHIFT: u32 = 16;

// EECD bits for the 93C46-style Microwire serial EEPROM that the Linux e1000
// driver bit-bangs on the 82540EM (it does NOT use EERD on this MAC type).
/// Serial clock.
pub const EECD_SK: u32 = 1 << 0;
/// Chip select.
pub const EECD_CS: u32 = 1 << 1;
/// Data in (host -> EEPROM).
pub const EECD_DI: u32 = 1 << 2;
/// Data out (EEPROM -> host).
pub const EECD_DO: u32 = 1 << 3;
/// Request EEPROM access (software semaphore).
pub const EECD_REQ: u32 = 1 << 6;
/// Grant EEPROM access.
pub const EECD_GNT: u32 = 1 << 7;
/// Microwire READ opcode (110b), shifted MSB-first as 3 opcode bits.
const MICROWIRE_READ_OPCODE: u16 = 0b110;
/// 93C46 address width for a 64-word EEPROM.
const MICROWIRE_ADDR_BITS: u8 = 6;

/// `RCTL.EN`: receiver enable.
pub const RCTL_EN: u32 = 1 << 1;
/// `TCTL.EN`: transmitter enable.
pub const TCTL_EN: u32 = 1 << 1;

/// `STATUS.LU`: link up.
pub const STATUS_LU: u32 = 1 << 1;
/// `STATUS.FD`: full duplex.
pub const STATUS_FD: u32 = 1 << 0;

// ---- Interrupt cause / mask bits -------------------------------------------

/// Transmit Descriptor Written Back.
pub const ICR_TXDW: u32 = 1 << 0;
/// Transmit Queue Empty.
pub const ICR_TXQE: u32 = 1 << 1;
/// Link Status Change.
pub const ICR_LSC: u32 = 1 << 2;
/// Receiver Overrun.
pub const ICR_RXO: u32 = 1 << 6;
/// Receiver Timer Interrupt (a frame was received).
pub const ICR_RXT0: u32 = 1 << 7;

// ---- TX descriptor fields --------------------------------------------------

/// Size of a single descriptor (legacy / context / data are all 16 bytes).
const DESC_SIZE: u64 = 16;

/// `TDESC.CMD.EOP`: end of packet (last descriptor of a frame).
const TXD_CMD_EOP: u8 = 1 << 0;
/// `TDESC.CMD.RS`: report status (write back DD).
const TXD_CMD_RS: u8 = 1 << 3;
/// `TDESC.STA.DD`: descriptor done (written back by hardware).
const TXD_STAT_DD: u8 = 1 << 0;

/// Bit in `dtyp`/`cmd` byte selecting the descriptor type for the
/// extended/data path. The legacy descriptor has bit 5 (DEXT) clear in the
/// command byte; an extended descriptor sets DEXT and uses the `dtyp` field.
const TXD_CMD_DEXT: u8 = 1 << 5;

/// Extended descriptor type: TCP/IP context descriptor.
const TXD_DTYP_CONTEXT: u8 = 0x0;
/// Extended descriptor type: data descriptor.
const TXD_DTYP_DATA: u8 = 0x1;

// ---- RX descriptor fields --------------------------------------------------

/// `RDESC.STA.DD`: descriptor done.
const RXD_STAT_DD: u8 = 1 << 0;
/// `RDESC.STA.EOP`: end of packet.
const RXD_STAT_EOP: u8 = 1 << 1;

/// Maximum frame size we will reassemble / accept (guards against malformed
/// descriptor rings producing unbounded allocations).
const MAX_FRAME: usize = 16 * 1024;

/// The Intel 82540EM PCI device identity, useful to the integration layer.
pub const VENDOR_ID: u16 = 0x8086;
pub const DEVICE_ID: u16 = 0x100E;

/// A 16-bit EEPROM image. Word layout follows the 82540EM: words 0..2 hold the
/// MAC address (low-to-high, little-endian per word), word 3 is the device ID
/// area, etc. We populate the MAC and a valid checksum word.
const EEPROM_WORDS: usize = 64;

/// Intel 82540EM e1000 network controller.
pub struct E1000 {
    // -- Control / status --
    ctrl: u32,
    status: u32,

    // -- EEPROM --
    eeprom: [u16; EEPROM_WORDS],
    eecd: u32,
    eerd: u32,
    /// Microwire bit-bang state: shift register of DI bits received this frame.
    ee_shift: u16,
    /// Number of command bits clocked in since chip-select went high.
    ee_count: u8,
    /// True once a READ command has been decoded and data is shifting out.
    ee_reading: bool,
    /// Latched EEPROM word being shifted out on DO (MSB first).
    ee_dataout: u16,
    /// Count of data bits already shifted out (0..16).
    ee_dataout_count: u8,
    /// Previous serial-clock level, for rising-edge detection.
    ee_prev_sk: bool,
    /// Current DO (data-out) level presented to the host.
    ee_do: bool,
    /// Programmed MAC address (also shadowed into RAL0/RAH0 on reset).
    mac: [u8; 6],

    // -- Interrupts --
    icr: u32,
    ims: u32,

    // -- Receive --
    rctl: u32,
    rdbal: u32,
    rdbah: u32,
    rdlen: u32,
    rdh: u32,
    rdt: u32,

    // -- Transmit --
    tctl: u32,
    tdbal: u32,
    tdbah: u32,
    tdlen: u32,
    tdh: u32,
    tdt: u32,

    // -- Receive address / multicast filter --
    ral0: u32,
    rah0: u32,
    mta: [u32; MTA_LEN],

    // -- TX reassembly + output queue --
    /// Partially assembled current TX frame (descriptors before EOP).
    tx_partial: Vec<u8>,
    /// Completed frames the device has "transmitted", oldest first.
    tx_queue: Vec<Vec<u8>>,
}

impl Default for E1000 {
    fn default() -> Self {
        Self::new([0x52, 0x54, 0x00, 0x12, 0x34, 0x56])
    }
}

impl E1000 {
    /// Construct a device with the given MAC address programmed into the
    /// emulated EEPROM (and shadowed into RAL0/RAH0).
    pub fn new(mac: [u8; 6]) -> Self {
        let mut eeprom = [0u16; EEPROM_WORDS];
        // Words 0..2: MAC address, little-endian within each 16-bit word.
        eeprom[0] = u16::from(mac[0]) | (u16::from(mac[1]) << 8);
        eeprom[1] = u16::from(mac[2]) | (u16::from(mac[3]) << 8);
        eeprom[2] = u16::from(mac[4]) | (u16::from(mac[5]) << 8);
        // Word 3: device id (informational).
        eeprom[3] = DEVICE_ID;

        // Word 0x3F holds the EEPROM checksum: the 16-bit sum of words 0..0x3F
        // must equal 0xBABA.
        let mut sum: u16 = 0;
        for &w in eeprom.iter().take(EEPROM_WORDS - 1) {
            sum = sum.wrapping_add(w);
        }
        eeprom[EEPROM_WORDS - 1] = 0xBABAu16.wrapping_sub(sum);

        // Shadow the MAC into RAL0/RAH0 with the Address Valid bit set.
        let ral0 = u32::from(mac[0])
            | (u32::from(mac[1]) << 8)
            | (u32::from(mac[2]) << 16)
            | (u32::from(mac[3]) << 24);
        let rah0 = u32::from(mac[4]) | (u32::from(mac[5]) << 8) | (1 << 31); // AV

        E1000 {
            ctrl: 0,
            // Link up, full duplex by default so guests see a usable link.
            status: STATUS_LU | STATUS_FD,
            eeprom,
            eecd: 0,
            eerd: 0,
            ee_shift: 0,
            ee_count: 0,
            ee_reading: false,
            ee_dataout: 0,
            ee_dataout_count: 0,
            ee_prev_sk: false,
            ee_do: false,
            mac,
            icr: 0,
            ims: 0,
            rctl: 0,
            rdbal: 0,
            rdbah: 0,
            rdlen: 0,
            rdh: 0,
            rdt: 0,
            tctl: 0,
            tdbal: 0,
            tdbah: 0,
            tdlen: 0,
            tdh: 0,
            tdt: 0,
            ral0,
            rah0,
            mta: [0u32; MTA_LEN],
            tx_partial: Vec::new(),
            tx_queue: Vec::new(),
        }
    }

    /// The programmed MAC address.
    pub fn mac(&self) -> [u8; 6] {
        self.mac
    }

    /// Whether the receiver is enabled (`RCTL.EN`).
    pub fn rx_enabled(&self) -> bool {
        self.rctl & RCTL_EN != 0
    }

    /// Whether the transmitter is enabled (`TCTL.EN`).
    pub fn tx_enabled(&self) -> bool {
        self.tctl & TCTL_EN != 0
    }

    /// Pop the oldest completed transmit frame, if any.
    pub fn take_tx_frame(&mut self) -> Option<Vec<u8>> {
        if self.tx_queue.is_empty() {
            None
        } else {
            Some(self.tx_queue.remove(0))
        }
    }

    /// Number of completed transmit frames pending in the queue.
    pub fn tx_queue_len(&self) -> usize {
        self.tx_queue.len()
    }

    /// The raw interrupt cause register (does not clear it).
    pub fn icr(&self) -> u32 {
        self.icr
    }

    /// The interrupt mask register.
    pub fn ims(&self) -> u32 {
        self.ims
    }

    /// True when an unmasked interrupt cause is asserted (i.e. the device would
    /// drive its interrupt line / send an MSI). Delivery itself is deferred to
    /// the integration layer.
    pub fn interrupt_pending(&self) -> bool {
        self.icr & self.ims != 0
    }

    // ---- Guest base addresses -------------------------------------------

    fn rx_base(&self) -> u64 {
        (u64::from(self.rdbah) << 32) | u64::from(self.rdbal)
    }

    fn tx_base(&self) -> u64 {
        (u64::from(self.tdbah) << 32) | u64::from(self.tdbal)
    }

    /// Number of descriptors in the RX ring (each descriptor is 16 bytes).
    fn rx_ring_count(&self) -> u32 {
        self.rdlen / DESC_SIZE as u32
    }

    /// Number of descriptors in the TX ring.
    fn tx_ring_count(&self) -> u32 {
        self.tdlen / DESC_SIZE as u32
    }

    // ---- EEPROM ----------------------------------------------------------

    /// Begin (and immediately complete) an EEPROM read when `EERD.START` is
    /// written. The data word is latched into the data field and `DONE` set.
    /// Drive the Microwire (93C46) EEPROM state machine from a write to EECD.
    /// Implements the READ protocol the Linux e1000 driver bit-bangs on the
    /// 82540EM (which does not use EERD): a 3-bit opcode (110b = READ) and a
    /// 6-bit address are clocked in on DI, then 16 data bits are clocked out on
    /// DO, MSB first, one bit per serial-clock rising edge while CS is asserted.
    fn eecd_write(&mut self, value: u32) {
        // Software EEPROM access semaphore: grant whenever requested.
        let mut stored = value;
        if value & EECD_REQ != 0 {
            stored |= EECD_GNT;
        } else {
            stored &= !EECD_GNT;
        }

        let cs = value & EECD_CS != 0;
        let sk = value & EECD_SK != 0;
        let di = value & EECD_DI != 0;

        if !cs {
            // Deselect resets the frame.
            self.ee_shift = 0;
            self.ee_count = 0;
            self.ee_reading = false;
            self.ee_dataout_count = 0;
            self.ee_do = false;
        } else if sk && !self.ee_prev_sk {
            // Rising clock edge with the chip selected.
            if self.ee_reading {
                // Shift out the next data bit, MSB first.
                self.ee_do = (self.ee_dataout >> (15 - self.ee_dataout_count)) & 1 != 0;
                self.ee_dataout_count += 1;
                if self.ee_dataout_count >= 16 {
                    self.ee_reading = false;
                    self.ee_count = 0;
                    self.ee_shift = 0;
                }
            } else {
                // Shift in a command bit.
                self.ee_shift = (self.ee_shift << 1) | (di as u16);
                self.ee_count += 1;
                self.ee_do = false;
                let cmd_bits = 3 + MICROWIRE_ADDR_BITS; // 3 opcode + 6 address bits
                if self.ee_count >= cmd_bits {
                    let opcode = (self.ee_shift >> MICROWIRE_ADDR_BITS) & 0b111;
                    let addr = (self.ee_shift & ((1 << MICROWIRE_ADDR_BITS) - 1)) as usize;
                    if opcode == MICROWIRE_READ_OPCODE {
                        self.ee_dataout = if addr < EEPROM_WORDS {
                            self.eeprom[addr]
                        } else {
                            0
                        };
                        self.ee_dataout_count = 0;
                        self.ee_reading = true;
                    } else {
                        // Writes/erases are not modeled; wait for deselect.
                        self.ee_count = 0;
                        self.ee_shift = 0;
                    }
                }
            }
        }

        self.ee_prev_sk = sk;
        // DO is reflected back to the guest at read time from `ee_do`.
        self.eecd = stored & !EECD_DO;
    }

    fn start_eeprom_read(&mut self, value: u32) {
        if value & EERD_START == 0 {
            // No read requested; just store the address bits, clear DONE.
            self.eerd = value & !EERD_DONE;
            return;
        }
        let addr = ((value >> EERD_ADDR_SHIFT) & EERD_ADDR_MASK) as usize;
        let data = if addr < EEPROM_WORDS {
            self.eeprom[addr]
        } else {
            0
        };
        // Clear START, set DONE, keep the address bits, place data in [31:16].
        self.eerd = (value & !EERD_START) | EERD_DONE | (u32::from(data) << EERD_DATA_SHIFT);
    }

    // ---- Interrupt helpers ----------------------------------------------

    /// Assert one or more interrupt causes in `ICR`.
    fn raise_interrupt(&mut self, cause: u32) {
        self.icr |= cause;
    }

    // ---- Register access -------------------------------------------------

    /// Read a 32-bit register at `offset`. Some registers have read side
    /// effects (e.g. `ICR` clears on read).
    fn read_reg(&mut self, offset: u64) -> u32 {
        match offset {
            CTRL => self.ctrl,
            STATUS => self.status,
            // Reflect the live Microwire data-out bit back to the guest.
            EECD => (self.eecd & !EECD_DO) | if self.ee_do { EECD_DO } else { 0 },
            EERD => self.eerd,
            ICR => {
                // Reading ICR returns the current causes and clears them.
                let v = self.icr;
                self.icr = 0;
                v
            }
            // ICS / IMS / IMC: IMS reads back the mask; ICS/IMC read as the mask
            // (real hw returns the mask for IMS; ICS/IMC are write-mostly).
            IMS => self.ims,
            ICS => self.ims,
            IMC => self.ims,
            RCTL => self.rctl,
            TCTL => self.tctl,
            RDBAL => self.rdbal,
            RDBAH => self.rdbah,
            RDLEN => self.rdlen,
            RDH => self.rdh,
            RDT => self.rdt,
            TDBAL => self.tdbal,
            TDBAH => self.tdbah,
            TDLEN => self.tdlen,
            TDH => self.tdh,
            TDT => self.tdt,
            RAL0 => self.ral0,
            RAH0 => self.rah0,
            o if (MTA_BASE..MTA_BASE + (MTA_LEN as u64) * 4).contains(&o) => {
                let idx = ((o - MTA_BASE) / 4) as usize;
                self.mta[idx]
            }
            _ => 0,
        }
    }

    /// Write a 32-bit register at `offset`. The actual DMA-driven side effects
    /// (TX/RX) need a [`Mem`]; that path is in [`Self::write_reg_mem`]. This
    /// method handles the non-DMA registers and is what the byte-oriented
    /// [`MmioDevice`] entry point uses (TX is kicked separately).
    fn write_reg(&mut self, offset: u64, value: u32) {
        match offset {
            CTRL => self.ctrl = value,
            STATUS => { /* status is read-only */ }
            EECD => self.eecd_write(value),
            EERD => self.start_eeprom_read(value),
            ICR => {
                // Writing 1s to ICR clears those cause bits.
                self.icr &= !value;
            }
            ICS => {
                // Writing ICS sets the corresponding cause bits in ICR.
                self.icr |= value;
            }
            IMS => {
                // Set mask bits.
                self.ims |= value;
            }
            IMC => {
                // Clear mask bits.
                self.ims &= !value;
            }
            RCTL => self.rctl = value,
            TCTL => self.tctl = value,
            RDBAL => self.rdbal = value & !0xF, // 16-byte aligned
            RDBAH => self.rdbah = value,
            RDLEN => self.rdlen = value & 0x000F_FF80, // multiple of 128 bytes
            RDH => self.rdh = value,
            RDT => self.rdt = value,
            TDBAL => self.tdbal = value & !0xF,
            TDBAH => self.tdbah = value,
            TDLEN => self.tdlen = value & 0x000F_FF80,
            TDH => self.tdh = value,
            // TDT is handled by the Mem-aware path; store the value here so a
            // plain register write (no DMA) still reflects the tail.
            TDT => self.tdt = value,
            RAL0 => self.ral0 = value,
            RAH0 => self.rah0 = value,
            o if (MTA_BASE..MTA_BASE + (MTA_LEN as u64) * 4).contains(&o) => {
                let idx = ((o - MTA_BASE) / 4) as usize;
                self.mta[idx] = value;
            }
            _ => {}
        }
    }

    /// Memory-aware register write. Identical to [`Self::write_reg`] except a
    /// write to `TDT` (advancing the transmit tail) drives TX descriptor
    /// processing against `mem`. Use this from a VMM that has guest memory
    /// available; the bare [`MmioDevice::write`] path defers TX until
    /// [`Self::process_tx`] is called explicitly.
    pub fn write_reg_mem<M: Mem>(&mut self, offset: u64, value: u32, mem: &mut M) {
        if offset == TDT {
            self.tdt = value;
            self.process_tx(mem);
        } else {
            self.write_reg(offset, value);
        }
    }

    // ---- Transmit --------------------------------------------------------

    /// Walk the TX descriptor ring from `TDH` to `TDT`, assembling frames and
    /// pushing each completed (EOP) frame onto the output queue. Writes back the
    /// descriptor done status when `RS` is set and raises `TXDW`.
    ///
    /// Returns the number of frames transmitted.
    pub fn process_tx<M: Mem>(&mut self, mem: &mut M) -> usize {
        if !self.tx_enabled() {
            return 0;
        }
        let count = self.tx_ring_count();
        if count == 0 {
            return 0;
        }
        let base = self.tx_base();
        let mut frames = 0usize;
        let mut wrote_back = false;

        // Process descriptors until head catches up with tail.
        let mut guard = count; // bound the loop to one full ring traversal
        while self.tdh != self.tdt && guard > 0 {
            guard -= 1;
            let idx = self.tdh as u64;
            let gpa = base + idx * DESC_SIZE;
            let mut raw = [0u8; DESC_SIZE as usize];
            if !mem.read(gpa, &mut raw) {
                break;
            }

            // The command byte sits at offset 11 in both legacy and extended
            // layouts; DEXT (bit 5) distinguishes them.
            let cmd = raw[11];
            let dext = cmd & TXD_CMD_DEXT != 0;

            if dext {
                // Extended descriptor. The DCMD byte is at offset 11 (DEXT in
                // bit 5). The descriptor type (DTYP) lives in bits [23:20] of
                // the dword at offset 8, i.e. the high nibble of byte 10.
                let dtyp = (raw[10] >> 4) & 0x0F;
                match dtyp {
                    TXD_DTYP_CONTEXT => {
                        // TCP/IP context descriptor: parsed and skipped (offload
                        // is deferred). Nothing is appended to the frame.
                    }
                    TXD_DTYP_DATA => {
                        let addr = u64::from_le_bytes(raw[0..8].try_into().unwrap());
                        // DTALEN is in the low 20 bits of the dword at offset 8.
                        let dtalen =
                            u32::from_le_bytes(raw[8..12].try_into().unwrap()) & 0x000F_FFFF;
                        self.append_tx_data(mem, addr, dtalen);
                        // For data descriptors, EOP is bit 0 of the DCMD byte
                        // which is the same offset 11 here.
                        if cmd & TXD_CMD_EOP != 0 {
                            self.finish_tx_frame();
                            frames += 1;
                        }
                    }
                    _ => {}
                }
            } else {
                // Legacy descriptor: buffer addr (0..8), length (8..10),
                // CMD (11), STA written back at 12.
                let addr = u64::from_le_bytes(raw[0..8].try_into().unwrap());
                let len = u16::from_le_bytes(raw[8..10].try_into().unwrap()) as u32;
                self.append_tx_data(mem, addr, len);
                if cmd & TXD_CMD_EOP != 0 {
                    self.finish_tx_frame();
                    frames += 1;
                }
            }

            // Write back descriptor-done status if requested.
            if cmd & TXD_CMD_RS != 0 {
                // STA byte is at offset 12 in both layouts.
                let _ = mem.write(gpa + 12, &[TXD_STAT_DD]);
                wrote_back = true;
            }

            // Advance head (wraps within the ring).
            self.tdh = (self.tdh + 1) % count;
        }

        if frames > 0 {
            self.raise_interrupt(ICR_TXDW);
        } else if wrote_back {
            self.raise_interrupt(ICR_TXDW);
        }
        frames
    }

    /// Append the buffer described by (`addr`, `len`) to the in-progress frame.
    fn append_tx_data<M: Mem>(&mut self, mem: &M, addr: u64, len: u32) {
        let len = len as usize;
        if len == 0 {
            return;
        }
        if self.tx_partial.len() + len > MAX_FRAME {
            // Malformed / oversized; drop the buffer rather than allocate
            // unbounded memory.
            return;
        }
        let mut buf = vec![0u8; len];
        if mem.read(addr, &mut buf) {
            self.tx_partial.extend_from_slice(&buf);
        }
    }

    /// Complete the in-progress frame and move it onto the output queue.
    fn finish_tx_frame(&mut self) {
        if !self.tx_partial.is_empty() {
            let frame = std::mem::take(&mut self.tx_partial);
            self.tx_queue.push(frame);
        } else {
            self.tx_partial.clear();
        }
    }

    // ---- Receive ---------------------------------------------------------

    /// Inject an externally received frame into the RX ring.
    ///
    /// Copies `frame` into the guest buffer of the descriptor at `RDH`, writes
    /// back the length and DD|EOP status, advances `RDH`, and raises `RXT0`.
    ///
    /// Returns `true` if the frame was accepted, `false` if the receiver is
    /// disabled, the ring is empty/full, or the DMA failed.
    pub fn receive_frame<M: Mem>(&mut self, mem: &mut M, frame: &[u8]) -> bool {
        if !self.rx_enabled() {
            return false;
        }
        let count = self.rx_ring_count();
        if count == 0 {
            return false;
        }
        // The ring is full when advancing head would reach the tail. The driver
        // owns descriptors in [RDH, RDT); hardware writes the descriptor at RDH.
        // When RDH == RDT the ring has no software-owned descriptors available.
        if self.rdh == self.rdt {
            // No free descriptor: signal overrun.
            self.raise_interrupt(ICR_RXO);
            return false;
        }
        if frame.len() > MAX_FRAME {
            return false;
        }

        let base = self.rx_base();
        let idx = self.rdh as u64;
        let gpa = base + idx * DESC_SIZE;
        let mut raw = [0u8; DESC_SIZE as usize];
        if !mem.read(gpa, &mut raw) {
            return false;
        }
        // RX descriptor: buffer addr (0..8), length (8..10) written back,
        // status (12) written back.
        let buf_addr = u64::from_le_bytes(raw[0..8].try_into().unwrap());
        if !mem.write(buf_addr, frame) {
            return false;
        }
        // Write back length.
        let len = frame.len() as u16;
        if !mem.write(gpa + 8, &len.to_le_bytes()) {
            return false;
        }
        // Write back status: DD | EOP.
        if !mem.write(gpa + 12, &[RXD_STAT_DD | RXD_STAT_EOP]) {
            return false;
        }

        // Advance head.
        self.rdh = (self.rdh + 1) % count;
        self.raise_interrupt(ICR_RXT0);
        true
    }
}

// ---- MmioDevice integration -------------------------------------------------

impl MmioDevice for E1000 {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        // Registers are 32-bit; compute the aligned register offset and the
        // byte position of this access within it (handles sub-word reads).
        let offset = addr;
        let aligned = offset & !0x3;
        let byte_in_reg = (offset & 0x3) as usize;
        let value = self.read_reg(aligned);
        let bytes = value.to_le_bytes();
        for (i, out) in data.iter_mut().enumerate() {
            let pos = byte_in_reg + i;
            *out = if pos < 4 { bytes[pos] } else { 0 };
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        let offset = addr;
        let aligned = offset & !0x3;
        let byte_in_reg = (offset & 0x3) as usize;

        // Read-modify-write so sub-word writes preserve the untouched bytes.
        // `read_reg` of ICR has clear-on-read semantics; avoid it for RMW by
        // using the stored value directly for the registers we know. For
        // simplicity we re-read via a side-effect-free snapshot.
        let mut bytes = self.peek_reg(aligned).to_le_bytes();
        for (i, byte) in data.iter().enumerate() {
            let pos = byte_in_reg + i;
            if pos < 4 {
                bytes[pos] = *byte;
            }
        }
        self.write_reg(aligned, u32::from_le_bytes(bytes));
    }
}

impl E1000 {
    /// Side-effect-free register read used to support read-modify-write of
    /// sub-word MMIO accesses (must not clear `ICR`).
    fn peek_reg(&self, offset: u64) -> u32 {
        match offset {
            CTRL => self.ctrl,
            STATUS => self.status,
            EECD => (self.eecd & !EECD_DO) | if self.ee_do { EECD_DO } else { 0 },
            EERD => self.eerd,
            ICR => self.icr,
            IMS | ICS | IMC => self.ims,
            RCTL => self.rctl,
            TCTL => self.tctl,
            RDBAL => self.rdbal,
            RDBAH => self.rdbah,
            RDLEN => self.rdlen,
            RDH => self.rdh,
            RDT => self.rdt,
            TDBAL => self.tdbal,
            TDBAH => self.tdbah,
            TDLEN => self.tdlen,
            TDH => self.tdh,
            TDT => self.tdt,
            RAL0 => self.ral0,
            RAH0 => self.rah0,
            o if (MTA_BASE..MTA_BASE + (MTA_LEN as u64) * 4).contains(&o) => {
                let idx = ((o - MTA_BASE) / 4) as usize;
                self.mta[idx]
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MAC: [u8; 6] = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc];

    fn dev() -> E1000 {
        E1000::new(TEST_MAC)
    }

    /// Write a full 32-bit register through the MMIO byte interface.
    fn write32(d: &mut E1000, off: u64, value: u32) {
        d.write(off, &value.to_le_bytes());
    }

    /// Read a full 32-bit register through the MMIO byte interface.
    fn read32(d: &mut E1000, off: u64) -> u32 {
        let mut buf = [0u8; 4];
        d.read(off, &mut buf);
        u32::from_le_bytes(buf)
    }

    // ---- Basic register read/write -------------------------------------

    #[test]
    fn ctrl_read_write_roundtrip() {
        let mut d = dev();
        write32(&mut d, CTRL, 0xDEAD_BEEF);
        assert_eq!(read32(&mut d, CTRL), 0xDEAD_BEEF);
    }

    #[test]
    fn status_is_link_up_and_read_only() {
        let mut d = dev();
        let st = read32(&mut d, STATUS);
        assert_ne!(st & STATUS_LU, 0, "link should be up");
        // Status is read-only: writes are ignored.
        write32(&mut d, STATUS, 0x0);
        assert_ne!(read32(&mut d, STATUS) & STATUS_LU, 0);
    }

    #[test]
    fn subword_write_preserves_other_bytes() {
        let mut d = dev();
        write32(&mut d, CTRL, 0x1122_3344);
        // Write only the lowest byte.
        d.write(CTRL, &[0xFF]);
        assert_eq!(read32(&mut d, CTRL), 0x1122_33FF);
        // Read just the high byte.
        let mut b = [0u8; 1];
        d.read(CTRL + 3, &mut b);
        assert_eq!(b[0], 0x11);
    }

    // ---- EEPROM / MAC via EERD -----------------------------------------

    #[test]
    fn eerd_reads_mac_words() {
        let mut d = dev();
        let mut mac = [0u8; 6];
        for word in 0..3u32 {
            // Issue an EEPROM read of `word`.
            write32(&mut d, EERD, EERD_START | (word << EERD_ADDR_SHIFT));
            let r = read32(&mut d, EERD);
            assert_ne!(r & EERD_DONE, 0, "EERD.DONE must be set after read");
            let data = (r >> EERD_DATA_SHIFT) as u16;
            mac[(word * 2) as usize] = (data & 0xFF) as u8;
            mac[(word * 2 + 1) as usize] = (data >> 8) as u8;
        }
        assert_eq!(mac, TEST_MAC);
        assert_eq!(d.mac(), TEST_MAC);
    }

    #[test]
    fn mac_shadowed_into_receive_address() {
        let mut d = dev();
        let ral = read32(&mut d, RAL0);
        let rah = read32(&mut d, RAH0);
        assert_eq!(ral & 0xFF, TEST_MAC[0] as u32);
        assert_eq!((ral >> 8) & 0xFF, TEST_MAC[1] as u32);
        assert_eq!((ral >> 16) & 0xFF, TEST_MAC[2] as u32);
        assert_eq!((ral >> 24) & 0xFF, TEST_MAC[3] as u32);
        assert_eq!(rah & 0xFF, TEST_MAC[4] as u32);
        assert_eq!((rah >> 8) & 0xFF, TEST_MAC[5] as u32);
        assert_ne!(rah & (1 << 31), 0, "Address Valid bit should be set");
    }

    #[test]
    fn eeprom_checksum_word_is_baba() {
        let d = dev();
        let mut sum: u16 = 0;
        for &w in d.eeprom.iter() {
            sum = sum.wrapping_add(w);
        }
        assert_eq!(sum, 0xBABA);
    }

    // ---- MTA -----------------------------------------------------------

    #[test]
    fn mta_read_write() {
        let mut d = dev();
        let off = MTA_BASE + 4 * 5;
        write32(&mut d, off, 0xA5A5_5A5A);
        assert_eq!(read32(&mut d, off), 0xA5A5_5A5A);
        // Other MTA entries untouched.
        assert_eq!(read32(&mut d, MTA_BASE), 0);
    }

    // ---- RCTL / TCTL enable bits ---------------------------------------

    #[test]
    fn rctl_tctl_enable_bits() {
        let mut d = dev();
        assert!(!d.rx_enabled());
        assert!(!d.tx_enabled());
        write32(&mut d, RCTL, RCTL_EN);
        write32(&mut d, TCTL, TCTL_EN);
        assert!(d.rx_enabled());
        assert!(d.tx_enabled());
        assert_eq!(read32(&mut d, RCTL) & RCTL_EN, RCTL_EN);
        assert_eq!(read32(&mut d, TCTL) & TCTL_EN, TCTL_EN);
    }

    // ---- Interrupt mask gating -----------------------------------------

    #[test]
    fn ims_imc_mask_gating() {
        let mut d = dev();
        // Raise a cause directly via ICS.
        write32(&mut d, ICS, ICR_RXT0);
        assert_ne!(d.icr() & ICR_RXT0, 0);
        // Masked off => no pending interrupt.
        assert!(!d.interrupt_pending());
        // Unmask RXT0 via IMS.
        write32(&mut d, IMS, ICR_RXT0);
        assert!(d.interrupt_pending());
        // Clearing the mask via IMC suppresses it again.
        write32(&mut d, IMC, ICR_RXT0);
        assert!(!d.interrupt_pending());
    }

    #[test]
    fn icr_read_clears() {
        let mut d = dev();
        write32(&mut d, ICS, ICR_TXDW);
        assert_ne!(read32(&mut d, ICR) & ICR_TXDW, 0);
        // Read-to-clear: a second read returns 0.
        assert_eq!(read32(&mut d, ICR), 0);
    }

    #[test]
    fn icr_write_clears_bits() {
        let mut d = dev();
        write32(&mut d, ICS, ICR_TXDW | ICR_RXT0);
        // Writing 1 to a cause bit clears it.
        write32(&mut d, ICR, ICR_TXDW);
        let v = read32(&mut d, ICR);
        assert_eq!(v & ICR_TXDW, 0);
        assert_ne!(v & ICR_RXT0, 0);
    }

    // ---- TX descriptor processing --------------------------------------

    /// Lay out a single-descriptor TX ring in memory and program the device.
    /// Returns (mem, ring_base, buf_addr).
    fn setup_tx_ring(d: &mut E1000, ring_entries: u32) -> (VecMem, u64, u64) {
        let ring_base = 0x1000u64;
        let buf_addr = 0x8000u64;
        let mem = VecMem::new(0x20000);
        write32(d, TDBAL, ring_base as u32);
        write32(d, TDBAH, 0);
        write32(d, TDLEN, ring_entries * DESC_SIZE as u32);
        write32(d, TDH, 0);
        write32(d, TDT, 0);
        write32(d, TCTL, TCTL_EN);
        (mem, ring_base, buf_addr)
    }

    /// Build a legacy TX descriptor into `mem` at `gpa`.
    fn write_legacy_txd(mem: &mut VecMem, gpa: u64, buf: u64, len: u16, cmd: u8) {
        let mut raw = [0u8; 16];
        raw[0..8].copy_from_slice(&buf.to_le_bytes());
        raw[8..10].copy_from_slice(&len.to_le_bytes());
        raw[11] = cmd;
        assert!(mem.write(gpa, &raw));
    }

    #[test]
    fn tdt_write_drives_tx_processing() {
        let mut d = dev();
        let (mut mem, ring_base, buf_addr) = setup_tx_ring(&mut d, 8);

        // Frame payload in guest memory.
        let payload = b"hello-e1000-frame";
        assert!(mem.write(buf_addr, payload));

        // One legacy descriptor: EOP + RS.
        write_legacy_txd(
            &mut mem,
            ring_base,
            buf_addr,
            payload.len() as u16,
            TXD_CMD_EOP | TXD_CMD_RS,
        );

        // Advancing TDT to 1 (via the Mem-aware path) processes descriptor 0.
        d.write_reg_mem(TDT, 1, &mut mem);

        // A frame should be queued.
        assert_eq!(d.tx_queue_len(), 1);
        let frame = d.take_tx_frame().unwrap();
        assert_eq!(&frame, payload);

        // Head advanced to tail.
        assert_eq!(read32(&mut d, TDH), 1);

        // Descriptor-done written back (STA byte at offset 12).
        let mut sta = [0u8; 1];
        assert!(mem.read(ring_base + 12, &mut sta));
        assert_ne!(sta[0] & TXD_STAT_DD, 0);

        // TXDW interrupt cause raised.
        assert_ne!(d.icr() & ICR_TXDW, 0);
    }

    #[test]
    fn tx_multi_descriptor_frame_reassembled() {
        let mut d = dev();
        let (mut mem, ring_base, _buf) = setup_tx_ring(&mut d, 8);

        let part_a = b"AAAA";
        let part_b = b"BBBBBB";
        let buf_a = 0x8000u64;
        let buf_b = 0x9000u64;
        assert!(mem.write(buf_a, part_a));
        assert!(mem.write(buf_b, part_b));

        // Descriptor 0: part A, not EOP. Descriptor 1: part B, EOP + RS.
        write_legacy_txd(&mut mem, ring_base, buf_a, part_a.len() as u16, TXD_CMD_RS);
        write_legacy_txd(
            &mut mem,
            ring_base + DESC_SIZE,
            buf_b,
            part_b.len() as u16,
            TXD_CMD_EOP | TXD_CMD_RS,
        );

        d.write_reg_mem(TDT, 2, &mut mem);

        assert_eq!(d.tx_queue_len(), 1);
        let frame = d.take_tx_frame().unwrap();
        assert_eq!(&frame, b"AAAABBBBBB");
    }

    #[test]
    fn tx_disabled_does_nothing() {
        let mut d = dev();
        let (mut mem, ring_base, buf_addr) = setup_tx_ring(&mut d, 8);
        // Disable TX.
        write32(&mut d, TCTL, 0);
        write_legacy_txd(&mut mem, ring_base, buf_addr, 4, TXD_CMD_EOP | TXD_CMD_RS);
        d.write_reg_mem(TDT, 1, &mut mem);
        assert_eq!(d.tx_queue_len(), 0);
    }

    #[test]
    fn tx_context_descriptor_skipped() {
        let mut d = dev();
        let (mut mem, ring_base, buf_addr) = setup_tx_ring(&mut d, 8);
        let payload = b"payload!";
        assert!(mem.write(buf_addr, payload));

        // Descriptor 0: TCP/IP context (DEXT set, DTYP=context in byte10[7:4]).
        let mut ctx = [0u8; 16];
        ctx[10] = TXD_DTYP_CONTEXT << 4;
        ctx[11] = TXD_CMD_DEXT;
        assert!(mem.write(ring_base, &ctx));

        // Descriptor 1: data descriptor (DEXT set, DTYP=data, EOP, RS).
        // dword@8: DTALEN in [19:0], DTYP in [23:20], DCMD in [31:24].
        let mut data = [0u8; 16];
        data[0..8].copy_from_slice(&buf_addr.to_le_bytes());
        let dword8 = (payload.len() as u32 & 0x000F_FFFF)
            | ((TXD_DTYP_DATA as u32) << 20)
            | (((TXD_CMD_DEXT | TXD_CMD_EOP | TXD_CMD_RS) as u32) << 24);
        data[8..12].copy_from_slice(&dword8.to_le_bytes());
        assert!(mem.write(ring_base + DESC_SIZE, &data));

        d.write_reg_mem(TDT, 2, &mut mem);
        assert_eq!(d.tx_queue_len(), 1);
        assert_eq!(&d.take_tx_frame().unwrap(), payload);
    }

    // ---- RX path -------------------------------------------------------

    fn setup_rx_ring(d: &mut E1000, ring_entries: u32) -> VecMem {
        let ring_base = 0x4000u64;
        let mut mem = VecMem::new(0x20000);
        // Each descriptor points at its own 2 KiB buffer.
        for i in 0..ring_entries as u64 {
            let buf = 0x10000 + i * 0x800;
            let gpa = ring_base + i * DESC_SIZE;
            assert!(mem.write(gpa, &buf.to_le_bytes()));
        }
        write32(d, RDBAL, ring_base as u32);
        write32(d, RDBAH, 0);
        write32(d, RDLEN, ring_entries * DESC_SIZE as u32);
        write32(d, RDH, 0);
        // Tail set so head..tail leaves descriptors available.
        write32(d, RDT, ring_entries - 1);
        write32(d, RCTL, RCTL_EN);
        mem
    }

    #[test]
    fn rx_injection_sets_rxt0_and_advances_rdh() {
        let mut d = dev();
        let mut mem = setup_rx_ring(&mut d, 8);

        let frame = b"incoming-ethernet-frame";
        let head_before = read32(&mut d, RDH);
        assert!(d.receive_frame(&mut mem, frame));

        // RXT0 raised.
        assert_ne!(d.icr() & ICR_RXT0, 0);

        // RDH advanced by one.
        assert_eq!(read32(&mut d, RDH), head_before + 1);

        // Descriptor 0: buffer holds the frame, length + status written back.
        let buf_addr = 0x10000u64;
        let mut got = vec![0u8; frame.len()];
        assert!(mem.read(buf_addr, &mut got));
        assert_eq!(&got, frame);

        let mut lenb = [0u8; 2];
        assert!(mem.read(0x4000 + 8, &mut lenb));
        assert_eq!(u16::from_le_bytes(lenb) as usize, frame.len());

        let mut sta = [0u8; 1];
        assert!(mem.read(0x4000 + 12, &mut sta));
        assert_ne!(sta[0] & RXD_STAT_DD, 0);
        assert_ne!(sta[0] & RXD_STAT_EOP, 0);
    }

    #[test]
    fn rx_disabled_rejects_frame() {
        let mut d = dev();
        let mut mem = setup_rx_ring(&mut d, 8);
        write32(&mut d, RCTL, 0); // disable
        assert!(!d.receive_frame(&mut mem, b"frame"));
        assert_eq!(d.icr() & ICR_RXT0, 0);
    }

    #[test]
    fn rx_full_ring_signals_overrun() {
        let mut d = dev();
        let mut mem = setup_rx_ring(&mut d, 8);
        // Make ring "full": RDH == RDT.
        let head = read32(&mut d, RDH);
        write32(&mut d, RDT, head);
        assert!(!d.receive_frame(&mut mem, b"frame"));
        assert_ne!(d.icr() & ICR_RXO, 0);
        assert_eq!(d.icr() & ICR_RXT0, 0);
    }

    #[test]
    fn rx_then_interrupt_pending_when_unmasked() {
        let mut d = dev();
        let mut mem = setup_rx_ring(&mut d, 8);
        write32(&mut d, IMS, ICR_RXT0);
        assert!(!d.interrupt_pending());
        assert!(d.receive_frame(&mut mem, b"abc"));
        assert!(d.interrupt_pending());
    }
}
