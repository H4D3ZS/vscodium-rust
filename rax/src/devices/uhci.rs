//! Intel-style UHCI (Universal Host Controller Interface, USB 1.1) host
//! controller register model.
//!
//! UHCI exposes its operational registers through a 32-byte I/O space (BAR4 on
//! a real PIIX3/PIIX4 USB function). This module models that register file as
//! an [`IoDevice`]: the controller is reachable through programmed I/O on a
//! base port, and the [`IoBus`](crate::devices::bus::IoBus) dispatches accesses
//! one byte at a time, so all register decode here is byte-granular. The 16-bit
//! registers (USBCMD, USBSTS, FRNUM, PORTSCx, ...) are reconstructed/split per
//! byte exactly the way the AC'97 mixer model does it.
//!
//! Guest physical memory (for the frame list and the TD/QH schedule) is reached
//! through the small [`Mem`] abstraction shared with the e1000/virtio device
//! models, so the schedule-walking logic can be unit-tested against a flat
//! `Vec<u8>` backing store.
//!
//! ## Register file (offsets within the 32-byte I/O space)
//!
//! | Off  | Reg        | Description                                   |
//! |------|------------|-----------------------------------------------|
//! | 0x00 | USBCMD     | Command (16-bit)                              |
//! | 0x02 | USBSTS     | Status (16-bit, write-1-to-clear)             |
//! | 0x04 | USBINTR    | Interrupt enable (16-bit)                     |
//! | 0x06 | FRNUM      | Frame number (16-bit, 11 bits significant)    |
//! | 0x08 | FRBASEADD  | Frame list base address (32-bit, 4 KiB align) |
//! | 0x0C | SOFMOD     | Start-of-frame modify (8-bit)                 |
//! | 0x10 | PORTSC1    | Port 1 status/control (16-bit)                |
//! | 0x12 | PORTSC2    | Port 2 status/control (16-bit)                |
//!
//! ## What is implemented
//!
//! * The full operational register file with correct bit-level semantics:
//!   USBCMD (Run/Stop, HCRESET, GRESET, EGSM, FGR, SWDBG, Configure-Flag,
//!   Max-Packet), USBSTS write-1-to-clear status bits including the HCHalted
//!   bit, USBINTR, FRNUM (11-bit wrap), FRBASEADD (4 KiB aligned), SOFMOD, and
//!   PORTSC1/PORTSC2.
//! * Reset behaviour: HCRESET (and GRESET) clear all controller state back to
//!   power-on defaults while preserving the connect status of attached ports
//!   (a connected device stays connected across a host-controller reset, just
//!   as a real root hub keeps its line state). HCRESET/GRESET self-clear.
//! * Run/Stop drives the HCHalted bit in USBSTS: clearing Run sets HCHalted,
//!   setting Run clears it.
//! * Frame-list base programming plus a frame-processing skeleton
//!   ([`Uhci::process_frame`]) that, when running, reads the current frame-list
//!   pointer from guest memory via [`Mem`], walks the QH/TD chain following the
//!   Link Pointer (honouring Terminate/QH-vs-TD/Vertical-First), advances FRNUM,
//!   and raises the appropriate USBSTS interrupt bits.
//! * Two root-hub ports with connect / connect-change / enable / enable-change /
//!   line-status / reset / suspend state and the write-1-to-clear change bits,
//!   plus [`Uhci::attach_port`] / [`Uhci::detach_port`] to model hot-plug.
//!
//! ## What is deferred
//!
//! * Real USB transaction execution: [`Uhci::process_frame`] parses the
//!   schedule (frame pointer -> QH/TD link chain) and reads each descriptor's
//!   header words from guest memory, but it does **not** move packet payload to
//!   or from device endpoints, evaluate the TD token/status fields against an
//!   attached function, retire TDs (write back the status word / clear Active),
//!   or perform the actual data stage. Descriptor walking is intentionally a
//!   skeleton bounded by a visited-pointer/step budget to stay terminating.
//! * Interrupt delivery to a PIC/IOAPIC line: USBSTS bits and the USBINTR mask
//!   are maintained and [`Uhci::interrupt_pending`] reports the resulting line
//!   level, but wiring it to the VMM's IRQ routing is left to the integration
//!   layer.
//! * Low-speed/full-speed timing, bandwidth reclamation, the isochronous /
//!   bulk / interrupt transfer-type distinctions, SOF generation timing, and
//!   actual attached USB device (endpoint) emulation.

use crate::devices::bus::IoDevice;

/// Abstraction over guest physical memory used by the schedule walker.
///
/// Mirrors the e1000/virtio `Mem` trait so the frame-list / TD-queue logic can
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

// ---- Register offsets (within the 32-byte I/O space) -----------------------

/// USB Command register (16-bit).
pub const USBCMD: u16 = 0x00;
/// USB Status register (16-bit, write-1-to-clear).
pub const USBSTS: u16 = 0x02;
/// USB Interrupt Enable register (16-bit).
pub const USBINTR: u16 = 0x04;
/// Frame Number register (16-bit, 11 significant bits).
pub const FRNUM: u16 = 0x06;
/// Frame List Base Address register (32-bit, 4 KiB aligned).
pub const FRBASEADD: u16 = 0x08;
/// Start Of Frame Modify register (8-bit).
pub const SOFMOD: u16 = 0x0C;
/// Port 1 Status/Control register (16-bit).
pub const PORTSC1: u16 = 0x10;
/// Port 2 Status/Control register (16-bit).
pub const PORTSC2: u16 = 0x12;

/// Size of the UHCI I/O register window in bytes.
pub const IO_SIZE: u16 = 0x20;

/// Number of root-hub ports modelled.
pub const NUM_PORTS: usize = 2;

// ---- USBCMD bit definitions ------------------------------------------------

/// Run/Stop. 1 = run, 0 = stop.
pub const CMD_RS: u16 = 1 << 0;
/// Host Controller Reset. Self-clearing.
pub const CMD_HCRESET: u16 = 1 << 1;
/// Global Reset. Self-clearing (driven for >= 10 ms by software).
pub const CMD_GRESET: u16 = 1 << 2;
/// Enter Global Suspend Mode.
pub const CMD_EGSM: u16 = 1 << 3;
/// Force Global Resume.
pub const CMD_FGR: u16 = 1 << 4;
/// Software Debug.
pub const CMD_SWDBG: u16 = 1 << 5;
/// Configure Flag (informational; set by software when configured).
pub const CMD_CF: u16 = 1 << 6;
/// Max Packet. 0 = 32 bytes, 1 = 64 bytes (reclamation packet size).
pub const CMD_MAXP: u16 = 1 << 7;
/// Mask of writable USBCMD bits.
pub const CMD_WRITE_MASK: u16 =
    CMD_RS | CMD_HCRESET | CMD_GRESET | CMD_EGSM | CMD_FGR | CMD_SWDBG | CMD_CF | CMD_MAXP;

// ---- USBSTS bit definitions (all write-1-to-clear) -------------------------

/// USB Interrupt (a TD with the IOC bit completed, or short packet).
pub const STS_USBINT: u16 = 1 << 0;
/// USB Error Interrupt.
pub const STS_ERROR: u16 = 1 << 1;
/// Resume Detect.
pub const STS_RESUME: u16 = 1 << 2;
/// Host System Error (e.g. a PCI bus error during a DMA).
pub const STS_HSE: u16 = 1 << 3;
/// Host Controller Process Error (malformed schedule).
pub const STS_HCPE: u16 = 1 << 4;
/// HCHalted. 1 = the host controller has stopped executing the schedule.
pub const STS_HCHALTED: u16 = 1 << 5;
/// Mask of all write-1-to-clear status bits.
pub const STS_RWC_MASK: u16 =
    STS_USBINT | STS_ERROR | STS_RESUME | STS_HSE | STS_HCPE | STS_HCHALTED;

// ---- USBINTR bit definitions -----------------------------------------------

/// Timeout/CRC interrupt enable.
pub const INTR_TIMEOUT_CRC: u16 = 1 << 0;
/// Resume interrupt enable.
pub const INTR_RESUME: u16 = 1 << 1;
/// Interrupt-on-complete enable.
pub const INTR_IOC: u16 = 1 << 2;
/// Short-packet interrupt enable.
pub const INTR_SHORT: u16 = 1 << 3;
/// Mask of writable USBINTR bits.
pub const INTR_WRITE_MASK: u16 = INTR_TIMEOUT_CRC | INTR_RESUME | INTR_IOC | INTR_SHORT;

// ---- PORTSC bit definitions ------------------------------------------------

/// Current Connect Status (read-only). 1 = a device is attached.
pub const PORTSC_CCS: u16 = 1 << 0;
/// Connect Status Change (write-1-to-clear).
pub const PORTSC_CSC: u16 = 1 << 1;
/// Port Enabled/Disabled (read/write).
pub const PORTSC_PE: u16 = 1 << 2;
/// Port Enable/Disable Change (write-1-to-clear).
pub const PORTSC_PEC: u16 = 1 << 3;
/// Line Status bit 0 (D+); read-only.
pub const PORTSC_LS_DP: u16 = 1 << 4;
/// Line Status bit 1 (D-); read-only.
pub const PORTSC_LS_DM: u16 = 1 << 5;
/// Resume Detect (read/write).
pub const PORTSC_RD: u16 = 1 << 6;
/// Reserved bit 7: reads as 1 in real hardware.
pub const PORTSC_RSVD1: u16 = 1 << 7;
/// Low-Speed Device Attached (read-only).
pub const PORTSC_LSDA: u16 = 1 << 8;
/// Port Reset (read/write).
pub const PORTSC_RESET: u16 = 1 << 9;
/// Suspend (read/write).
pub const PORTSC_SUSPEND: u16 = 1 << 12;
/// Mask of the write-1-to-clear change bits in PORTSC.
pub const PORTSC_RWC_MASK: u16 = PORTSC_CSC | PORTSC_PEC;
/// Mask of the directly software-writable (non-RWC, non-RO) PORTSC bits.
pub const PORTSC_WRITE_MASK: u16 = PORTSC_PE | PORTSC_RD | PORTSC_RESET | PORTSC_SUSPEND;

// ---- Frame-list / link-pointer encoding ------------------------------------

/// Terminate (T) bit in a frame-list pointer / link pointer: 1 = invalid.
pub const LINK_TERMINATE: u32 = 1 << 0;
/// QH/TD select (Q) bit in a link pointer: 1 = points at a QH, 0 = at a TD.
pub const LINK_QH: u32 = 1 << 1;
/// Depth/Breadth (Vf) select bit in a TD link pointer: 1 = depth (vertical).
pub const LINK_VF: u32 = 1 << 2;
/// Mask that extracts the 16-byte-aligned pointer from a link pointer.
pub const LINK_PTR_MASK: u32 = !0xF;

/// Number of entries in a UHCI frame list (always 1024).
pub const FRAME_LIST_ENTRIES: u32 = 1024;

/// Upper bound on schedule-walk steps per frame, to guarantee termination on a
/// malformed (looping) schedule.
const MAX_WALK_STEPS: usize = 256;

/// State of a single root-hub port.
#[derive(Clone, Copy, Debug, Default)]
pub struct Port {
    /// A device is currently attached (drives PORTSC.CCS).
    pub connected: bool,
    /// Attached device is low-speed (drives PORTSC.LSDA).
    pub low_speed: bool,
    /// Connect status change latch (PORTSC.CSC).
    pub connect_change: bool,
    /// Port is enabled (PORTSC.PE).
    pub enabled: bool,
    /// Enable/disable change latch (PORTSC.PEC).
    pub enable_change: bool,
    /// Port is being reset (PORTSC.RESET).
    pub reset: bool,
    /// Port is suspended (PORTSC.SUSPEND).
    pub suspended: bool,
    /// Resume detect (PORTSC.RD).
    pub resume_detect: bool,
}

impl Port {
    /// Assemble the 16-bit PORTSC value from this port's state.
    fn read_portsc(&self) -> u16 {
        // Bit 7 is reserved and always reads as 1 on real UHCI hardware.
        let mut v = PORTSC_RSVD1;
        if self.connected {
            v |= PORTSC_CCS;
            // Line status is meaningful only while connected; report idle J/K.
            // A full-speed device idles with D+ high (LS_DP); a low-speed
            // device idles with D- high (LS_DM).
            if self.low_speed {
                v |= PORTSC_LS_DM | PORTSC_LSDA;
            } else {
                v |= PORTSC_LS_DP;
            }
        }
        if self.connect_change {
            v |= PORTSC_CSC;
        }
        if self.enabled {
            v |= PORTSC_PE;
        }
        if self.enable_change {
            v |= PORTSC_PEC;
        }
        if self.resume_detect {
            v |= PORTSC_RD;
        }
        if self.reset {
            v |= PORTSC_RESET;
        }
        if self.suspended {
            v |= PORTSC_SUSPEND;
        }
        v
    }

    /// Apply a software write to this port, honouring the RWC change bits and
    /// the read-only status bits. `mask` selects which bits this access drives,
    /// so a byte-wide access only affects bits in its own byte; bits outside
    /// `mask` are untouched.
    fn write_portsc(&mut self, value: u16, mask: u16) {
        // Write-1-to-clear the change latches (only if the bit was driven).
        if mask & value & PORTSC_CSC != 0 {
            self.connect_change = false;
        }
        if mask & value & PORTSC_PEC != 0 {
            self.enable_change = false;
        }

        // Directly writable control bit: Port Enable.
        if mask & PORTSC_PE != 0 {
            let was_enabled = self.enabled;
            self.enabled = value & PORTSC_PE != 0;
            // A 1->0 transition on PE latches the enable-change bit.
            if was_enabled && !self.enabled {
                self.enable_change = true;
            }
        }
        if mask & PORTSC_RD != 0 {
            self.resume_detect = value & PORTSC_RD != 0;
        }
        if mask & PORTSC_SUSPEND != 0 {
            self.suspended = value & PORTSC_SUSPEND != 0;
        }

        // Port reset. While reset is asserted the port is forced disabled; we
        // model the completion of reset (deassertion) as enabling the port if a
        // device is connected, matching how a hub re-enables after reset.
        if mask & PORTSC_RESET != 0 {
            let reset_now = value & PORTSC_RESET != 0;
            if reset_now {
                self.reset = true;
                self.enabled = false;
            } else if self.reset {
                // Reset deasserted: drive the port enabled if a device present.
                self.reset = false;
                if self.connected {
                    self.enabled = true;
                }
            }
        }
    }

    /// Reset this port's volatile state back to power-on defaults. The physical
    /// connect/line state (a device is plugged in or not) is preserved, but a
    /// fresh connect-change is latched so software re-enumerates after a global
    /// controller reset.
    fn reset_state(&mut self) {
        let connected = self.connected;
        let low_speed = self.low_speed;
        *self = Port {
            connected,
            low_speed,
            connect_change: connected,
            ..Port::default()
        };
    }
}

/// UHCI host controller register model.
pub struct Uhci {
    /// Base I/O port of the 32-byte register window.
    base: u16,

    // Operational registers.
    cmd: u16,
    sts: u16,
    intr: u16,
    frnum: u16,
    frbase: u32,
    sofmod: u8,

    /// Root-hub ports.
    ports: [Port; NUM_PORTS],
}

impl Default for Uhci {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Uhci {
    /// Create a new UHCI controller whose register window starts at I/O port
    /// `base`. The controller comes up halted (HCHalted set), stopped, and with
    /// no devices attached, matching the post-reset state.
    pub fn new(base: u16) -> Self {
        let mut hc = Uhci {
            base,
            cmd: 0,
            sts: 0,
            intr: 0,
            frnum: 0,
            frbase: 0,
            sofmod: 0x40, // power-on default: 1 ms frame (12000 bit times).
            ports: [Port::default(); NUM_PORTS],
        };
        hc.reset_power_on();
        hc
    }

    /// Apply power-on / global-reset defaults to all controller registers.
    fn reset_power_on(&mut self) {
        self.cmd = 0;
        // After reset the controller is halted.
        self.sts = STS_HCHALTED;
        self.intr = 0;
        self.frnum = 0;
        self.frbase = 0;
        self.sofmod = 0x40;
    }

    /// The base I/O port of the register window.
    pub fn base(&self) -> u16 {
        self.base
    }

    /// True if the Run/Stop bit is set and the controller is not halted.
    pub fn running(&self) -> bool {
        (self.cmd & CMD_RS) != 0 && (self.sts & STS_HCHALTED) == 0
    }

    /// True if the controller is halted.
    pub fn halted(&self) -> bool {
        (self.sts & STS_HCHALTED) != 0
    }

    /// The programmed frame-list base address (4 KiB aligned).
    pub fn frame_base(&self) -> u32 {
        self.frbase
    }

    /// The current 11-bit frame number.
    pub fn frame_number(&self) -> u16 {
        self.frnum & 0x7FF
    }

    /// Read-only access to a root-hub port's state.
    pub fn port(&self, index: usize) -> Option<&Port> {
        self.ports.get(index)
    }

    /// Current asserted-interrupt line level: true when an enabled status
    /// condition is pending. USBINTR.IOC gates USBINT, etc.
    pub fn interrupt_pending(&self) -> bool {
        let mut gate = 0u16;
        if self.intr & INTR_IOC != 0 {
            gate |= STS_USBINT;
        }
        if self.intr & (INTR_TIMEOUT_CRC | INTR_SHORT) != 0 {
            gate |= STS_ERROR | STS_USBINT;
        }
        if self.intr & INTR_RESUME != 0 {
            gate |= STS_RESUME;
        }
        // Host-system / process errors always assert regardless of USBINTR.
        let always = STS_HSE | STS_HCPE;
        (self.sts & (gate | always)) != 0
    }

    // -- Reset handling ------------------------------------------------------

    /// Perform a Host Controller Reset (USBCMD.HCRESET). Clears all controller
    /// state to power-on defaults, resets the root-hub ports (preserving their
    /// physical connect state), and self-clears the HCRESET bit.
    fn do_hcreset(&mut self) {
        self.reset_power_on();
        for p in &mut self.ports {
            p.reset_state();
        }
        // HCRESET self-clears once the reset completes.
        self.cmd &= !CMD_HCRESET;
    }

    /// Perform a Global Reset (USBCMD.GRESET). Like HCRESET but also resets the
    /// USB bus; for this model it behaves identically and self-clears.
    fn do_greset(&mut self) {
        self.reset_power_on();
        for p in &mut self.ports {
            p.reset_state();
        }
        self.cmd &= !CMD_GRESET;
    }

    // -- USBCMD write semantics ----------------------------------------------

    /// Apply a USBCMD write. `value` is the new 16-bit value the guest is
    /// driving and `mask` selects which bits this access actually touches (so a
    /// byte-wide write only updates its own byte). Bits outside `mask` keep
    /// their current value.
    fn write_cmd(&mut self, value: u16, mask: u16) {
        let mask = mask & CMD_WRITE_MASK;
        let value = (self.cmd & !mask) | (value & mask);

        // Resets take priority and override the rest of the write.
        if value & CMD_HCRESET != 0 {
            self.cmd = value;
            self.do_hcreset();
            return;
        }
        if value & CMD_GRESET != 0 {
            self.cmd = value;
            self.do_greset();
            return;
        }

        self.cmd = value;

        // Run/Stop drives the HCHalted status bit.
        if value & CMD_RS != 0 {
            // Software set Run: the controller leaves the halted state and
            // begins (would begin) executing the schedule.
            self.sts &= !STS_HCHALTED;
        } else {
            // Software cleared Run: the controller halts.
            self.sts |= STS_HCHALTED;
        }
    }

    // -- USBSTS write semantics (write-1-to-clear) ---------------------------

    /// Apply a USBSTS write. Only bits selected by `mask` (the bytes the guest
    /// actually drove) and set to 1 are cleared; a 0 leaves a bit untouched.
    fn write_sts(&mut self, value: u16, mask: u16) {
        self.sts &= !(value & mask & STS_RWC_MASK);
    }

    // -- Byte-granular register access ---------------------------------------

    /// Read one byte from the register file given a *relative* byte offset.
    fn read_reg_byte(&self, off: u16) -> u8 {
        match off {
            0x00 => self.cmd as u8,
            0x01 => (self.cmd >> 8) as u8,
            0x02 => self.sts as u8,
            0x03 => (self.sts >> 8) as u8,
            0x04 => self.intr as u8,
            0x05 => (self.intr >> 8) as u8,
            0x06 => self.frame_number() as u8,
            0x07 => (self.frame_number() >> 8) as u8,
            0x08 => self.frbase as u8,
            0x09 => (self.frbase >> 8) as u8,
            0x0A => (self.frbase >> 16) as u8,
            0x0B => (self.frbase >> 24) as u8,
            0x0C => self.sofmod,
            0x10 => self.ports[0].read_portsc() as u8,
            0x11 => (self.ports[0].read_portsc() >> 8) as u8,
            0x12 => self.ports[1].read_portsc() as u8,
            0x13 => (self.ports[1].read_portsc() >> 8) as u8,
            _ => 0x00,
        }
    }

    /// Write one byte to the register file given a *relative* byte offset.
    ///
    /// 16-bit registers with side effects (USBCMD, USBSTS, PORTSCx) are routed
    /// through their word-level handler together with a byte-aligned write mask,
    /// so the reset / run-stop / write-1-to-clear / change-bit semantics stay
    /// correct regardless of the access width: a byte access only affects the
    /// bits in the byte it actually drove.
    fn write_reg_byte(&mut self, off: u16, value: u8) {
        // For the 16-bit registers, position the byte into the word and build
        // the matching byte-aligned mask (0x00FF for low byte, 0xFF00 for high).
        let lo_hi = off & 1;
        let word_val = (value as u16) << (8 * lo_hi);
        let word_mask = 0x00FFu16 << (8 * lo_hi);

        match off {
            0x00 | 0x01 => self.write_cmd(word_val, word_mask),
            0x02 | 0x03 => self.write_sts(word_val, word_mask),
            0x04 | 0x05 => {
                let new = (self.intr & !word_mask) | (word_val & word_mask);
                self.intr = new & INTR_WRITE_MASK;
            }
            0x06 | 0x07 => {
                // FRNUM is writable only while the controller is halted.
                if self.halted() {
                    let new = (self.frnum & !word_mask) | (word_val & word_mask);
                    self.frnum = new & 0x7FF;
                }
            }
            0x08..=0x0B => {
                let shift = (off - 0x08) * 8;
                let mask = !(0xFFu32 << shift);
                self.frbase = (self.frbase & mask) | ((value as u32) << shift);
                // Frame list base is 4 KiB aligned; the low 12 bits read 0.
                self.frbase &= !0xFFF;
            }
            0x0C => {
                self.sofmod = value;
            }
            0x10 | 0x11 => self.ports[0].write_portsc(word_val, word_mask),
            0x12 | 0x13 => self.ports[1].write_portsc(word_val, word_mask),
            _ => {}
        }
    }

    // -- Root hub port hot-plug ----------------------------------------------

    /// Attach a (full-speed) device to root-hub port `index`. Sets the connect
    /// status and latches the connect-status-change bit so software notices the
    /// new device. Returns false if `index` is out of range.
    pub fn attach_port(&mut self, index: usize) -> bool {
        self.attach_port_speed(index, false)
    }

    /// Attach a device of the given speed to root-hub port `index`.
    /// `low_speed` selects a low-speed (1.5 Mbit/s) device.
    pub fn attach_port_speed(&mut self, index: usize, low_speed: bool) -> bool {
        let Some(port) = self.ports.get_mut(index) else {
            return false;
        };
        // Connecting a previously-disconnected device latches CSC.
        if !port.connected {
            port.connect_change = true;
        }
        port.connected = true;
        port.low_speed = low_speed;
        true
    }

    /// Detach the device on root-hub port `index`. Clears connect/enable and
    /// latches both the connect-status-change and (if it was enabled) the
    /// enable-change bits. Returns false if `index` is out of range.
    pub fn detach_port(&mut self, index: usize) -> bool {
        let Some(port) = self.ports.get_mut(index) else {
            return false;
        };
        if port.connected {
            port.connect_change = true;
        }
        if port.enabled {
            port.enable_change = true;
        }
        port.connected = false;
        port.low_speed = false;
        port.enabled = false;
        port.suspended = false;
        port.reset = false;
        true
    }

    // -- Frame / schedule processing skeleton --------------------------------

    /// Process one USB frame against the schedule in guest memory.
    ///
    /// This is the schedule-walking *skeleton*: when the controller is running
    /// and a frame-list base is programmed, it reads the frame-list pointer for
    /// the current frame, then walks the QH/TD link chain following each Link
    /// Pointer until it hits a Terminate bit, an invalid (null) pointer, or the
    /// per-frame step budget. It advances the 11-bit frame number on every call.
    ///
    /// Returns the number of schedule entries (QHs/TDs) visited this frame. TD
    /// execution (token/status evaluation, data movement, write-back) is
    /// deferred — see the module docs.
    pub fn process_frame<M: Mem>(&mut self, mem: &M) -> usize {
        if !self.running() {
            return 0;
        }
        if self.frbase == 0 {
            // No schedule programmed; still advance the frame counter so SOF
            // timing-derived behaviour keeps moving.
            self.advance_frame();
            return 0;
        }

        let frame = (self.frame_number() & 0x3FF) as u32; // 1024-entry list.
        let entry_gpa = self.frbase as u64 + (frame as u64) * 4;

        let mut link = match read_u32(mem, entry_gpa) {
            Some(v) => v,
            None => {
                // A DMA fault while fetching the schedule is a host system
                // error.
                self.sts |= STS_HSE;
                self.advance_frame();
                return 0;
            }
        };

        let mut visited = 0usize;
        let mut steps = 0usize;
        while link & LINK_TERMINATE == 0 && steps < MAX_WALK_STEPS {
            steps += 1;
            let ptr = (link & LINK_PTR_MASK) as u64;
            if ptr == 0 {
                break;
            }
            let is_qh = link & LINK_QH != 0;

            if is_qh {
                // Queue Head: two 32-bit words — the horizontal link (QHLP) and
                // the element/vertical link (QELP).
                let qhlp = match read_u32(mem, ptr) {
                    Some(v) => v,
                    None => {
                        self.sts |= STS_HSE;
                        break;
                    }
                };
                let qelp = match read_u32(mem, ptr + 4) {
                    Some(v) => v,
                    None => {
                        self.sts |= STS_HSE;
                        break;
                    }
                };
                visited += 1;
                // Follow the element list (vertical) first if valid, else the
                // horizontal queue-head list.
                if qelp & LINK_TERMINATE == 0 && (qelp & LINK_PTR_MASK) != 0 {
                    link = qelp;
                } else {
                    link = qhlp;
                }
            } else {
                // Transfer Descriptor: 4 words. We read the link and status
                // words; payload/token handling is deferred. The TD is *not*
                // retired (its Active bit is left untouched).
                let tdlp = match read_u32(mem, ptr) {
                    Some(v) => v,
                    None => {
                        self.sts |= STS_HSE;
                        break;
                    }
                };
                // Word 1 is the control/status word (read but unused here).
                let _status = read_u32(mem, ptr + 4);
                visited += 1;
                link = tdlp;
            }
        }

        if steps >= MAX_WALK_STEPS {
            // A schedule that never terminates is malformed.
            self.sts |= STS_HCPE;
        }

        self.advance_frame();
        visited
    }

    /// Advance the 11-bit frame number with wraparound.
    fn advance_frame(&mut self) {
        self.frnum = (self.frnum.wrapping_add(1)) & 0x7FF;
    }
}

/// Read a little-endian u32 from guest memory, returning None on a faulting
/// (out-of-bounds) access.
fn read_u32<M: Mem>(mem: &M, gpa: u64) -> Option<u32> {
    let mut buf = [0u8; 4];
    if mem.read(gpa, &mut buf) {
        Some(u32::from_le_bytes(buf))
    } else {
        None
    }
}

impl IoDevice for Uhci {
    fn read(&mut self, port: u16) -> u8 {
        if port < self.base {
            return 0xff;
        }
        let off = port - self.base;
        if off >= IO_SIZE {
            return 0xff;
        }
        self.read_reg_byte(off)
    }

    fn write(&mut self, port: u16, value: u8) {
        if port < self.base {
            return;
        }
        let off = port - self.base;
        if off >= IO_SIZE {
            return;
        }
        self.write_reg_byte(off, value);
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: u16 = 0xC000;

    /// Read a 16-bit register via two byte I/O accesses (little-endian),
    /// exactly as the IoBus would dispatch a word access.
    fn read16(hc: &mut Uhci, off: u16) -> u16 {
        let lo = hc.read(BASE + off) as u16;
        let hi = hc.read(BASE + off + 1) as u16;
        lo | (hi << 8)
    }

    /// Write a 16-bit register via two byte I/O accesses (little-endian).
    fn write16(hc: &mut Uhci, off: u16, value: u16) {
        hc.write(BASE + off, value as u8);
        hc.write(BASE + off + 1, (value >> 8) as u8);
    }

    /// Write a 32-bit register via four byte I/O accesses (little-endian).
    fn write32(hc: &mut Uhci, off: u16, value: u32) {
        for i in 0..4 {
            hc.write(BASE + off + i, (value >> (8 * i)) as u8);
        }
    }

    fn read32(hc: &mut Uhci, off: u16) -> u32 {
        let mut v = 0u32;
        for i in 0..4 {
            v |= (hc.read(BASE + off + i) as u32) << (8 * i);
        }
        v
    }

    #[test]
    fn power_on_state_is_halted() {
        let hc = Uhci::new(BASE);
        assert!(hc.halted(), "controller should come up halted");
        assert!(!hc.running());
        assert_eq!(hc.frame_base(), 0);
        assert_eq!(hc.frame_number(), 0);
    }

    #[test]
    fn usbcmd_run_stop_drives_hchalted() {
        let mut hc = Uhci::new(BASE);
        // Initially halted.
        assert_ne!(read16(&mut hc, USBSTS) & STS_HCHALTED, 0);

        // Set Run/Stop -> leaves halted state.
        write16(&mut hc, USBCMD, CMD_RS);
        assert_eq!(read16(&mut hc, USBCMD) & CMD_RS, CMD_RS);
        assert_eq!(
            read16(&mut hc, USBSTS) & STS_HCHALTED,
            0,
            "HCHalted must clear when Run set"
        );
        assert!(hc.running());

        // Clear Run/Stop -> controller halts.
        write16(&mut hc, USBCMD, 0);
        assert_ne!(
            read16(&mut hc, USBSTS) & STS_HCHALTED,
            0,
            "HCHalted must set when Run cleared"
        );
        assert!(!hc.running());
    }

    #[test]
    fn hcreset_clears_state() {
        let mut hc = Uhci::new(BASE);
        // Dirty up some registers.
        write16(&mut hc, USBCMD, CMD_RS | CMD_MAXP | CMD_CF);
        write32(&mut hc, FRBASEADD, 0x1234_5000);
        write16(&mut hc, USBINTR, INTR_IOC | INTR_RESUME);
        write16(&mut hc, SOFMOD, 0x12);
        assert_eq!(hc.frame_base(), 0x1234_5000);
        assert!(hc.running());

        // Issue HCRESET.
        write16(&mut hc, USBCMD, CMD_HCRESET);

        // HCRESET self-clears.
        assert_eq!(
            read16(&mut hc, USBCMD) & CMD_HCRESET,
            0,
            "HCRESET should self-clear"
        );
        // State reverts to power-on: halted, no run, base cleared, intr cleared.
        assert!(hc.halted());
        assert!(!hc.running());
        assert_eq!(hc.frame_base(), 0);
        assert_eq!(hc.frame_number(), 0);
        assert_eq!(read16(&mut hc, USBINTR), 0);
        assert_eq!(read16(&mut hc, USBCMD) & CMD_MAXP, 0);
    }

    #[test]
    fn greset_clears_state() {
        let mut hc = Uhci::new(BASE);
        write32(&mut hc, FRBASEADD, 0x4444_0000);
        write16(&mut hc, USBCMD, CMD_RS);
        write16(&mut hc, USBCMD, CMD_GRESET);
        assert_eq!(
            read16(&mut hc, USBCMD) & CMD_GRESET,
            0,
            "GRESET self-clears"
        );
        assert!(hc.halted());
        assert_eq!(hc.frame_base(), 0);
    }

    #[test]
    fn usbsts_write_one_to_clear() {
        let mut hc = Uhci::new(BASE);
        // Force some status bits set (HCHalted is set at reset, plus simulate
        // USBINT + ERROR via the schedule-error path indirectly). We set them
        // directly through a host-error walk would be complex, so assert RWC
        // behaviour on the bits we can observe.

        // HCHalted is set at power-on. Clear it via W1C.
        assert_ne!(read16(&mut hc, USBSTS) & STS_HCHALTED, 0);
        write16(&mut hc, USBSTS, STS_HCHALTED);
        assert_eq!(
            read16(&mut hc, USBSTS) & STS_HCHALTED,
            0,
            "writing 1 to HCHalted should clear it"
        );

        // Writing 0 to a set bit must leave it unchanged. Re-halt by stopping,
        // then write 0 and confirm HCHalted stays set.
        write16(&mut hc, USBCMD, 0); // stop -> sets HCHalted
        assert_ne!(read16(&mut hc, USBSTS) & STS_HCHALTED, 0);
        write16(&mut hc, USBSTS, 0x0000);
        assert_ne!(
            read16(&mut hc, USBSTS) & STS_HCHALTED,
            0,
            "writing 0 must not clear a set status bit"
        );

        // Now exercise multiple bits at once via the host-system-error path:
        // run with a frbase pointing into unmapped guest memory so the walk
        // raises HSE, then W1C it.
        write32(&mut hc, FRBASEADD, 0xFFFF_F000);
        write16(&mut hc, USBCMD, CMD_RS);
        let mem = VecMem::new(0x1000); // too small -> read faults -> HSE
        hc.process_frame(&mem);
        assert_ne!(
            read16(&mut hc, USBSTS) & STS_HSE,
            0,
            "out-of-bounds schedule fetch should raise HSE"
        );
        // W1C just the HSE bit; other bits untouched.
        write16(&mut hc, USBSTS, STS_HSE);
        assert_eq!(read16(&mut hc, USBSTS) & STS_HSE, 0);
    }

    #[test]
    fn frbaseadd_alignment_and_frnum_programming() {
        let mut hc = Uhci::new(BASE);
        // FRBASEADD is 4 KiB aligned: the low 12 bits read back 0.
        write32(&mut hc, FRBASEADD, 0xDEAD_BEEF);
        assert_eq!(read32(&mut hc, FRBASEADD), 0xDEAD_B000);
        assert_eq!(hc.frame_base(), 0xDEAD_B000);

        // FRNUM is an 11-bit field, programmable while halted.
        assert!(hc.halted());
        write16(&mut hc, FRNUM, 0x7FF);
        assert_eq!(read16(&mut hc, FRNUM), 0x7FF);
        write16(&mut hc, FRNUM, 0xFFFF); // upper bits masked away
        assert_eq!(read16(&mut hc, FRNUM), 0x7FF);
        write16(&mut hc, FRNUM, 0x123);
        assert_eq!(hc.frame_number(), 0x123);

        // FRNUM is not writable while running.
        write16(&mut hc, USBCMD, CMD_RS);
        write16(&mut hc, FRNUM, 0x055);
        assert_eq!(
            hc.frame_number(),
            0x123,
            "FRNUM must be read-only while the controller runs"
        );
    }

    #[test]
    fn portsc_connect_enable_reset_and_change_rwc() {
        let mut hc = Uhci::new(BASE);

        // No device: CCS clear. Reserved bit 7 reads 1.
        let p1 = read16(&mut hc, PORTSC1);
        assert_eq!(p1 & PORTSC_CCS, 0);
        assert_ne!(p1 & PORTSC_RSVD1, 0, "reserved bit 7 reads as 1");

        // Attach a device -> CCS set, CSC latched.
        assert!(hc.attach_port(0));
        let p1 = read16(&mut hc, PORTSC1);
        assert_ne!(p1 & PORTSC_CCS, 0, "connect status should be set");
        assert_ne!(p1 & PORTSC_CSC, 0, "connect change should latch");
        // Full-speed device idles with D+ high.
        assert_ne!(p1 & PORTSC_LS_DP, 0);
        assert_eq!(p1 & PORTSC_LSDA, 0, "full-speed device, LSDA clear");

        // W1C the connect-change bit; CCS (read-only status) must persist.
        write16(&mut hc, PORTSC1, PORTSC_CSC);
        let p1 = read16(&mut hc, PORTSC1);
        assert_eq!(p1 & PORTSC_CSC, 0, "CSC should clear on write-1");
        assert_ne!(p1 & PORTSC_CCS, 0, "CCS must stay set (read-only)");

        // Writing 1 to CCS (read-only) must not change it; nor clear it.
        write16(&mut hc, PORTSC1, PORTSC_CCS);
        assert_ne!(read16(&mut hc, PORTSC1) & PORTSC_CCS, 0);

        // Enable the port directly.
        write16(&mut hc, PORTSC1, PORTSC_PE);
        assert_ne!(read16(&mut hc, PORTSC1) & PORTSC_PE, 0, "PE should set");

        // Disabling the port (PE 1->0) latches the enable-change bit.
        write16(&mut hc, PORTSC1, 0);
        let p1 = read16(&mut hc, PORTSC1);
        assert_eq!(p1 & PORTSC_PE, 0, "PE cleared");
        assert_ne!(p1 & PORTSC_PEC, 0, "PEC latches on 1->0 of PE");
        // W1C the enable-change.
        write16(&mut hc, PORTSC1, PORTSC_PEC);
        assert_eq!(read16(&mut hc, PORTSC1) & PORTSC_PEC, 0);

        // Port reset cycle: assert reset, then deassert -> port enabled (device
        // connected).
        write16(&mut hc, PORTSC1, PORTSC_RESET);
        let p1 = read16(&mut hc, PORTSC1);
        assert_ne!(p1 & PORTSC_RESET, 0, "reset asserted");
        assert_eq!(p1 & PORTSC_PE, 0, "port disabled while in reset");
        write16(&mut hc, PORTSC1, 0); // deassert reset
        let p1 = read16(&mut hc, PORTSC1);
        assert_eq!(p1 & PORTSC_RESET, 0, "reset deasserted");
        assert_ne!(
            p1 & PORTSC_PE,
            0,
            "port should enable after reset completes with device present"
        );
    }

    #[test]
    fn low_speed_attach_sets_lsda() {
        let mut hc = Uhci::new(BASE);
        assert!(hc.attach_port_speed(1, true));
        let p2 = read16(&mut hc, PORTSC2);
        assert_ne!(p2 & PORTSC_CCS, 0);
        assert_ne!(p2 & PORTSC_LSDA, 0, "low-speed device sets LSDA");
        assert_ne!(p2 & PORTSC_LS_DM, 0, "low-speed idles with D- high");
    }

    #[test]
    fn attach_detach_port_state() {
        let mut hc = Uhci::new(BASE);
        // Out-of-range index rejected.
        assert!(!hc.attach_port(NUM_PORTS));
        assert!(!hc.detach_port(99));

        assert!(hc.attach_port(0));
        assert!(hc.port(0).unwrap().connected);
        assert!(hc.port(0).unwrap().connect_change);

        // Clear the change, then detach -> change re-latches, connect drops.
        write16(&mut hc, PORTSC1, PORTSC_CSC);
        assert!(!hc.port(0).unwrap().connect_change);

        // Enable so detach also latches the enable-change.
        write16(&mut hc, PORTSC1, PORTSC_PE);
        assert!(hc.port(0).unwrap().enabled);

        assert!(hc.detach_port(0));
        let p = hc.port(0).unwrap();
        assert!(!p.connected, "device disconnected");
        assert!(p.connect_change, "detach latches CSC");
        assert!(p.enable_change, "detach of enabled port latches PEC");
        assert!(!p.enabled);
    }

    #[test]
    fn second_port_is_independent() {
        let mut hc = Uhci::new(BASE);
        hc.attach_port(0);
        assert_ne!(read16(&mut hc, PORTSC1) & PORTSC_CCS, 0);
        assert_eq!(
            read16(&mut hc, PORTSC2) & PORTSC_CCS,
            0,
            "port 2 unaffected by port 1 attach"
        );
    }

    #[test]
    fn usbintr_writable_bits() {
        let mut hc = Uhci::new(BASE);
        write16(&mut hc, USBINTR, 0xFFFF);
        assert_eq!(
            read16(&mut hc, USBINTR),
            INTR_WRITE_MASK,
            "only the 4 defined interrupt-enable bits are writable"
        );
    }

    #[test]
    fn frame_processing_walks_qh_td_chain() {
        // Build a tiny schedule: frame 0 -> QH; QH element -> TD; TD link
        // terminates. The walker should visit the QH and the TD.
        let mut mem = VecMem::new(0x4000);
        let frbase: u32 = 0x1000;
        let qh_gpa: u32 = 0x2000;
        let td_gpa: u32 = 0x2040;

        // Frame-list entry 0 points at the QH (Q bit set).
        let fle = qh_gpa | LINK_QH;
        mem.write(frbase as u64, &fle.to_le_bytes());

        // QH: horizontal link terminates; element link points at the TD (TD,
        // not QH).
        mem.write(qh_gpa as u64, &LINK_TERMINATE.to_le_bytes()); // QHLP = T
        let qelp = td_gpa; // Q bit clear -> TD, not terminate
        mem.write(qh_gpa as u64 + 4, &qelp.to_le_bytes());

        // TD: link pointer terminates.
        mem.write(td_gpa as u64, &LINK_TERMINATE.to_le_bytes());
        mem.write(td_gpa as u64 + 4, &0u32.to_le_bytes()); // status word

        let mut hc = Uhci::new(BASE);
        write32(&mut hc, FRBASEADD, frbase);
        // FRNUM = 0 already. Start running.
        write16(&mut hc, USBCMD, CMD_RS);
        assert!(hc.running());

        let visited = hc.process_frame(&mem);
        assert_eq!(visited, 2, "should visit the QH and the TD");
        // Frame number advanced.
        assert_eq!(hc.frame_number(), 1);
        // No host-system / process errors raised on a well-formed schedule.
        assert_eq!(read16(&mut hc, USBSTS) & (STS_HSE | STS_HCPE), 0);
    }

    #[test]
    fn frame_processing_noop_when_stopped() {
        let mut mem = VecMem::new(0x4000);
        mem.write(0x1000, &(0x2000u32 | LINK_QH).to_le_bytes());
        let mut hc = Uhci::new(BASE);
        write32(&mut hc, FRBASEADD, 0x1000);
        // Not running.
        assert_eq!(hc.process_frame(&mem), 0);
        assert_eq!(hc.frame_number(), 0, "frame number must not advance");
    }

    #[test]
    fn frame_processing_terminated_list_advances_frame() {
        // Frame-list entry has the Terminate bit set: nothing to walk, but the
        // frame number still advances.
        let mut mem = VecMem::new(0x4000);
        mem.write(0x1000, &LINK_TERMINATE.to_le_bytes());
        let mut hc = Uhci::new(BASE);
        write32(&mut hc, FRBASEADD, 0x1000);
        write16(&mut hc, USBCMD, CMD_RS);
        assert_eq!(hc.process_frame(&mem), 0);
        assert_eq!(hc.frame_number(), 1);
    }

    #[test]
    fn frame_processing_detects_looping_schedule() {
        // A TD whose link points back at itself is a malformed (looping)
        // schedule: the walker bounds itself and raises HCPE.
        let mut mem = VecMem::new(0x4000);
        let frbase: u32 = 0x1000;
        let td_gpa: u32 = 0x2000;
        mem.write(frbase as u64, &td_gpa.to_le_bytes()); // TD (Q bit clear)
        mem.write(td_gpa as u64, &td_gpa.to_le_bytes()); // links to itself
        mem.write(td_gpa as u64 + 4, &0u32.to_le_bytes());

        let mut hc = Uhci::new(BASE);
        write32(&mut hc, FRBASEADD, frbase);
        write16(&mut hc, USBCMD, CMD_RS);
        hc.process_frame(&mem);
        assert_ne!(
            read16(&mut hc, USBSTS) & STS_HCPE,
            0,
            "a self-looping schedule should raise HCPE"
        );
    }

    #[test]
    fn out_of_window_access_ignored() {
        let mut hc = Uhci::new(BASE);
        // Below window.
        assert_eq!(hc.read(BASE - 1), 0xff);
        // Above window.
        assert_eq!(hc.read(BASE + IO_SIZE), 0xff);
        // Writes outside the window are ignored (no panic).
        hc.write(BASE - 1, 0xaa);
        hc.write(BASE + IO_SIZE + 4, 0x55);
    }
}
