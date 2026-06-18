//! AHCI 1.3 (Serial ATA Advanced Host Controller Interface) host controller.
//!
//! This implements the *ABAR (AHCI Base Address) MMIO register file* and the
//! command-issue path of a single-port SATA HBA, per the "Serial ATA AHCI 1.3.1
//! Specification". The controller is wired into the machine as an
//! [`MmioDevice`] occupying the ABAR memory window. The HBA exposes generic
//! host control registers (CAP/GHC/IS/PI/VS/CAP2) followed by an array of
//! per-port register banks (port 0 lives at offset 0x100, stride 0x80).
//!
//! A single SATA disk is modelled as an in-memory `Vec<u8>`, in the same style
//! as the [`crate::devices::nvme`] namespace and the [`crate::devices::ide`]
//! disk. The data path uses the [`Mem`] guest-memory abstraction borrowed from
//! the virtio transport so the scatter/gather (PRDT) walking can be unit-tested
//! against a plain byte buffer.
//!
//! # Register layout (offsets from ABAR)
//!
//! ## Generic Host Control
//!
//! | Offset | Name  | Dir  | Notes                                              |
//! |--------|-------|------|----------------------------------------------------|
//! | 0x00   | CAP   | RO   | Host Capabilities (NP, NCS, S64A, SAM, SSS, ...)   |
//! | 0x04   | GHC   | RW   | Global Host Control (AE, IE, HR)                   |
//! | 0x08   | IS    | RWC  | Interrupt Status (one bit per port, write-1-clear) |
//! | 0x0C   | PI    | RO   | Ports Implemented bitmap                           |
//! | 0x10   | VS    | RO   | Version (= 0x0001_0300 for AHCI 1.3)              |
//! | 0x24   | CAP2  | RO   | Host Capabilities Extended                         |
//!
//! ## Per-port registers (port `p` at `0x100 + p*0x80`)
//!
//! | Rel    | Name    | Dir  | Notes                                            |
//! |--------|---------|------|--------------------------------------------------|
//! | 0x00   | PxCLB   | RW   | Command List Base (low 32)                       |
//! | 0x04   | PxCLBU  | RW   | Command List Base (high 32)                      |
//! | 0x08   | PxFB    | RW   | FIS Base (low 32)                                |
//! | 0x0C   | PxFBU   | RW   | FIS Base (high 32)                               |
//! | 0x10   | PxIS    | RWC  | Interrupt Status (write-1-clear)                 |
//! | 0x14   | PxIE    | RW   | Interrupt Enable                                 |
//! | 0x18   | PxCMD   | RW   | Command and Status (ST, FRE, FR, CR, ...)        |
//! | 0x20   | PxTFD   | RO   | Task File Data (status + error)                  |
//! | 0x24   | PxSIG   | RO   | Signature (0x0000_0101 for a SATA disk)          |
//! | 0x28   | PxSSTS  | RO   | SATA Status (DET / SPD / IPM)                     |
//! | 0x2C   | PxSCTL  | RW   | SATA Control                                     |
//! | 0x30   | PxSERR  | RWC  | SATA Error                                        |
//! | 0x34   | PxSACT  | RW   | SATA Active (NCQ tag bitmap)                      |
//! | 0x38   | PxCI    | RW   | Command Issue (one bit per command slot)         |
//!
//! All registers are little-endian, 32-bit. Sub-word and byte accesses are
//! serviced read-modify-write against the containing 32-bit word.
//!
//! # Command-issue path
//!
//! When the host sets a bit in `PxCI` (with `PxCMD.ST` and `PxCMD.FRE` set),
//! the controller, for each issued slot:
//!
//! 1. reads the 32-byte *command header* from the command list (at `PxCLB +
//!    slot*32`) to recover the command-table base (`CTBA`) and the PRDT length;
//! 2. reads the 64-byte *Command FIS* (a Register Host-to-Device FIS) from the
//!    head of the command table;
//! 3. decodes the ATA command — `READ DMA EXT` (0x25), `WRITE DMA EXT` (0x35)
//!    or `IDENTIFY DEVICE` (0xEC);
//! 4. walks the Physical Region Descriptor Table (PRDT) entries that follow the
//!    command FIS, DMAing the payload to/from guest memory scatter/gather;
//! 5. writes a Device-to-Host Register FIS into the received-FIS area, updates
//!    `PxTFD`, clears the `PxCI` bit, and raises `PxIS` (and the HBA-level `IS`
//!    bit) — surfaced through [`AhciController::interrupt_pending`].
//!
//! # What is deferred / partial
//!
//! * **One port, one command slot exercised at a time.** The register file
//!   advertises 32 command slots (`CAP.NCS`) and `PI` carries a single
//!   implemented port; the issue loop iterates every set `PxCI` bit, but the
//!   tests drive slot 0. NCQ (`FPDMA QUEUED`, `PxSACT`) is *not* executed —
//!   `PxSACT` is stored but the queued-command protocol is not run.
//! * **PRDT walking is real but bounded.** Each PRD entry's DBA/DBC are honored
//!   (scatter/gather across multiple entries works); the per-entry "interrupt
//!   on completion" flag and the 4 MiB-per-entry maximum are not enforced. A
//!   transfer whose PRDT byte-count disagrees with the ATA sector count is
//!   clamped to the smaller of the two.
//! * **Received FIS area:** only the D2H Register FIS (offset 0x40 of the FIS
//!   receive area) is written back. The PIO Setup, DMA Setup and Set Device
//!   Bits FIS slots are not produced.
//! * **Only three ATA opcodes** are decoded (READ DMA EXT / WRITE DMA EXT /
//!   IDENTIFY DEVICE). Any other command FIS completes with the ATA error
//!   (ABRT) bit set in `PxTFD`.
//! * **No COMRESET / port reset state machine, no staggered spin-up, no
//!   hot-plug.** Device presence is surfaced statically in `PxSSTS`/`PxSIG`.
//! * **Interrupts** are aggregated and surfaced through
//!   [`AhciController::interrupt_pending`] for an orchestrator to inject; the
//!   device does not drive a guest interrupt line itself.
//!
//! These omissions are isolated to the queued-command and FIS-receive
//! sub-paths; the register interface, AE/HR handshake, command-list/FIS-base
//! programming, the `ST`/`FRE` -> `CR`/`FR` running bits, and the single-shot
//! DMA command round trip are complete.

use super::bus::MmioDevice;

// Reuse the guest-memory DMA abstraction defined for the virtio transport so
// the command path can be unit-tested against a plain byte buffer.
pub use super::virtio::{Mem, VecMem};

// ---- Generic Host Control register offsets ---------------------------------

pub const REG_CAP: u64 = 0x00; // Host Capabilities
pub const REG_GHC: u64 = 0x04; // Global Host Control
pub const REG_IS: u64 = 0x08; // Interrupt Status (per-port bits, RWC)
pub const REG_PI: u64 = 0x0c; // Ports Implemented
pub const REG_VS: u64 = 0x10; // Version
pub const REG_CCC_CTL: u64 = 0x14; // Command Completion Coalescing Control
pub const REG_CCC_PORTS: u64 = 0x18; // Command Completion Coalescing Ports
pub const REG_EM_LOC: u64 = 0x1c; // Enclosure Management Location
pub const REG_EM_CTL: u64 = 0x20; // Enclosure Management Control
pub const REG_CAP2: u64 = 0x24; // Host Capabilities Extended
pub const REG_BOHC: u64 = 0x28; // BIOS/OS Handoff Control and Status

/// First byte of the per-port register region.
pub const PORT_BASE: u64 = 0x100;
/// Stride between adjacent per-port register banks.
pub const PORT_STRIDE: u64 = 0x80;

/// Size of the ABAR MMIO window we claim: generic registers + 32 port banks.
/// 0x100 + 32 * 0x80 = 0x1100.
pub const ABAR_SIZE: u64 = 0x1100;

// ---- Per-port register offsets (relative to the port bank) -----------------

pub const PX_CLB: u64 = 0x00; // Command List Base (low)
pub const PX_CLBU: u64 = 0x04; // Command List Base (high)
pub const PX_FB: u64 = 0x08; // FIS Base (low)
pub const PX_FBU: u64 = 0x0c; // FIS Base (high)
pub const PX_IS: u64 = 0x10; // Interrupt Status (RWC)
pub const PX_IE: u64 = 0x14; // Interrupt Enable
pub const PX_CMD: u64 = 0x18; // Command and Status
pub const PX_TFD: u64 = 0x20; // Task File Data
pub const PX_SIG: u64 = 0x24; // Signature
pub const PX_SSTS: u64 = 0x28; // SATA Status
pub const PX_SCTL: u64 = 0x2c; // SATA Control
pub const PX_SERR: u64 = 0x30; // SATA Error
pub const PX_SACT: u64 = 0x34; // SATA Active
pub const PX_CI: u64 = 0x38; // Command Issue

// ---- Version ---------------------------------------------------------------

/// VS register value for AHCI 1.3: MJR=1, MNR=3 => 0x0001_0300.
pub const AHCI_VERSION_1_3: u32 = 0x0001_0300;

// ---- GHC (Global Host Control) bit fields ----------------------------------

/// GHC.HR (bit 0): HBA reset. Write-1 to reset; self-clears when complete.
pub const GHC_HR: u32 = 1 << 0;
/// GHC.IE (bit 1): global interrupt enable.
pub const GHC_IE: u32 = 1 << 1;
/// GHC.AE (bit 31): AHCI enable.
pub const GHC_AE: u32 = 1 << 31;

// ---- CAP (Host Capabilities) bit fields ------------------------------------

/// CAP.NP (bits 4:0): number of ports, 0-based.
pub const CAP_NP_MASK: u32 = 0x1f;
/// CAP.NCS (bits 12:8): number of command slots, 0-based.
pub const CAP_NCS_SHIFT: u32 = 8;
pub const CAP_NCS_MASK: u32 = 0x1f;
/// CAP.SSS (bit 27): supports staggered spin-up.
pub const CAP_SSS: u32 = 1 << 27;
/// CAP.SAM (bit 18): supports AHCI mode only (no legacy).
pub const CAP_SAM: u32 = 1 << 18;
/// CAP.S64A (bit 31): supports 64-bit addressing.
pub const CAP_S64A: u32 = 1 << 31;

// ---- PxCMD (Port Command and Status) bit fields ----------------------------

/// PxCMD.ST (bit 0): start (process the command list).
pub const PXCMD_ST: u32 = 1 << 0;
/// PxCMD.SUD (bit 1): spin-up device.
pub const PXCMD_SUD: u32 = 1 << 1;
/// PxCMD.POD (bit 2): power-on device.
pub const PXCMD_POD: u32 = 1 << 2;
/// PxCMD.FRE (bit 4): FIS receive enable.
pub const PXCMD_FRE: u32 = 1 << 4;
/// PxCMD.FR (bit 14): FIS receive running (read-only status).
pub const PXCMD_FR: u32 = 1 << 14;
/// PxCMD.CR (bit 15): command list running (read-only status).
pub const PXCMD_CR: u32 = 1 << 15;

// ---- PxIS (Port Interrupt Status) bit fields -------------------------------

/// PxIS.DHRS (bit 0): Device-to-Host Register FIS interrupt.
pub const PXIS_DHRS: u32 = 1 << 0;
/// PxIS.PSS (bit 1): PIO Setup FIS interrupt.
pub const PXIS_PSS: u32 = 1 << 1;
/// PxIS.DSS (bit 2): DMA Setup FIS interrupt.
pub const PXIS_DSS: u32 = 1 << 2;
/// PxIS.TFES (bit 30): Task File Error Status.
pub const PXIS_TFES: u32 = 1 << 30;

// ---- PxSSTS (SATA Status) fields -------------------------------------------

/// PxSSTS.DET (bits 3:0) = 3: device present and Phy communication established.
pub const SSTS_DET_PRESENT: u32 = 0x3;
/// PxSSTS.SPD (bits 7:4) = 3: Gen3 (6 Gbps) negotiated.
pub const SSTS_SPD_GEN3: u32 = 0x3 << 4;
/// PxSSTS.IPM (bits 11:8) = 1: interface in active power-management state.
pub const SSTS_IPM_ACTIVE: u32 = 0x1 << 8;

/// Composite PxSSTS value reported for a present, active Gen3 device.
pub const SSTS_PRESENT: u32 = SSTS_DET_PRESENT | SSTS_SPD_GEN3 | SSTS_IPM_ACTIVE;

// ---- PxSIG (signature) constants -------------------------------------------

/// Signature for a non-packet SATA disk (sector count 1, LBA 0x000001).
pub const SIG_SATA_DISK: u32 = 0x0000_0101;
/// Signature for an ATAPI device (unused; for reference).
pub const SIG_SATAPI: u32 = 0xEB14_0101;

// ---- PxTFD (Task File Data) status bits (ATA status register) --------------

/// ATA STATUS.ERR (bit 0): error.
pub const ATA_ERR: u8 = 1 << 0;
/// ATA STATUS.DRQ (bit 3): data request.
pub const ATA_DRQ: u8 = 1 << 3;
/// ATA STATUS.DF (bit 5): device fault.
pub const ATA_DF: u8 = 1 << 5;
/// ATA STATUS.DRDY (bit 6): device ready.
pub const ATA_DRDY: u8 = 1 << 6;
/// ATA STATUS.BSY (bit 7): busy.
pub const ATA_BSY: u8 = 1 << 7;

/// ATA ERROR.ABRT (bit 2): command aborted.
pub const ATA_ERR_ABRT: u8 = 1 << 2;

// ---- ATA command opcodes (in the Command FIS) ------------------------------

pub const ATA_CMD_READ_DMA_EXT: u8 = 0x25;
pub const ATA_CMD_WRITE_DMA_EXT: u8 = 0x35;
pub const ATA_CMD_IDENTIFY: u8 = 0xec;

// ---- FIS types -------------------------------------------------------------

/// Register FIS - Host to Device.
pub const FIS_TYPE_REG_H2D: u8 = 0x27;
/// Register FIS - Device to Host.
pub const FIS_TYPE_REG_D2H: u8 = 0x34;

// ---- Geometry --------------------------------------------------------------

/// Logical sector size (512 bytes).
pub const SECTOR_SIZE: u64 = 512;
/// Command-header size in the command list (bytes).
pub const CMD_HEADER_SIZE: u64 = 32;
/// Command-FIS region size at the head of a command table (bytes).
pub const CMD_FIS_SIZE: u64 = 64;
/// ATAPI command region in a command table (bytes).
pub const ATAPI_CMD_SIZE: u64 = 16;
/// Reserved region after the ATAPI command, before the PRDT (bytes).
pub const CMD_TABLE_RSV: u64 = 48;
/// Offset of the first PRD entry within a command table.
pub const PRDT_OFFSET: u64 = CMD_FIS_SIZE + ATAPI_CMD_SIZE + CMD_TABLE_RSV; // 0x80
/// Size of one Physical Region Descriptor (bytes).
pub const PRD_SIZE: u64 = 16;
/// Offset of the D2H Register FIS within the FIS receive area.
pub const RFIS_D2H_OFFSET: u64 = 0x40;

/// A parsed AHCI command header (first 8 bytes + CTBA we use).
///
/// Command-header layout (32 bytes, little-endian):
///   DW0: bits 4:0 CFL (command FIS length in DWORDs), bit5 ATAPI, bit6 Write,
///        bit7 Prefetchable, ..., bits 31:16 PRDTL (PRD table length, entries)
///   DW1: PRDBC (PRD byte count transferred) — written by HBA
///   DW2: CTBA  (command table base, low 32, 128-byte aligned)
///   DW3: CTBAU (command table base, high 32)
#[derive(Clone, Copy, Debug, Default)]
pub struct CommandHeader {
    /// Command FIS length, in DWORDs (bits 4:0 of DW0).
    pub cfl: u8,
    /// Write bit (DW0 bit 6): transfer is host -> device.
    pub write: bool,
    /// ATAPI bit (DW0 bit 5).
    pub atapi: bool,
    /// PRD table length, in entries (DW0 bits 31:16).
    pub prdtl: u16,
    /// Command table base address (CTBA | CTBAU << 32).
    pub ctba: u64,
}

impl CommandHeader {
    /// Parse a command header from a 32-byte little-endian buffer.
    pub fn parse(buf: &[u8]) -> Option<CommandHeader> {
        if buf.len() < CMD_HEADER_SIZE as usize {
            return None;
        }
        let dw0 = u32::from_le_bytes(buf[0..4].try_into().unwrap());
        let ctba_lo = u32::from_le_bytes(buf[8..12].try_into().unwrap());
        let ctba_hi = u32::from_le_bytes(buf[12..16].try_into().unwrap());
        Some(CommandHeader {
            cfl: (dw0 & 0x1f) as u8,
            atapi: dw0 & (1 << 5) != 0,
            write: dw0 & (1 << 6) != 0,
            prdtl: (dw0 >> 16) as u16,
            ctba: (ctba_lo as u64) | ((ctba_hi as u64) << 32),
        })
    }
}

/// A parsed Register Host-to-Device FIS (the fields we use).
///
/// H2D Register FIS layout (20 bytes, little-endian):
///   byte 0: FIS type (0x27)
///   byte 1: bit7 = C (1 => command, written into the command register)
///   byte 2: command (ATA opcode)
///   byte 3: features (low)
///   bytes 4..7: LBA[7:0], LBA[15:8], LBA[23:16], device
///   bytes 8..11: LBA[31:24], LBA[39:32], LBA[47:40], features (high)
///   bytes 12..13: count (low), count (high)
#[derive(Clone, Copy, Debug, Default)]
pub struct FisH2D {
    /// ATA command opcode (byte 2).
    pub command: u8,
    /// Device/head register (byte 7) — bit 6 = LBA mode.
    pub device: u8,
    /// 48-bit LBA assembled from bytes 4..6 and 8..10.
    pub lba: u64,
    /// 16-bit sector count assembled from bytes 12..13 (0 => 65536 for EXT).
    pub count: u32,
}

impl FisH2D {
    /// Parse an H2D Register FIS from its leading bytes.
    pub fn parse(buf: &[u8]) -> Option<FisH2D> {
        if buf.len() < 16 || buf[0] != FIS_TYPE_REG_H2D {
            return None;
        }
        let lba = (buf[4] as u64)
            | ((buf[5] as u64) << 8)
            | ((buf[6] as u64) << 16)
            | ((buf[8] as u64) << 24)
            | ((buf[9] as u64) << 32)
            | ((buf[10] as u64) << 40);
        let count = (buf[12] as u32) | ((buf[13] as u32) << 8);
        Some(FisH2D {
            command: buf[2],
            device: buf[7],
            lba,
            count,
        })
    }
}

/// A single Physical Region Descriptor (16 bytes, little-endian):
///   DW0: DBA   (data base address, low 32)
///   DW1: DBAU  (data base address, high 32)
///   DW2: reserved
///   DW3: bit31 I (interrupt on completion), bits 21:0 DBC (byte count, 0-based)
#[derive(Clone, Copy, Debug, Default)]
pub struct Prd {
    pub dba: u64,
    pub byte_count: u32,
}

impl Prd {
    /// Parse a PRD from a 16-byte little-endian buffer.
    pub fn parse(buf: &[u8]) -> Option<Prd> {
        if buf.len() < PRD_SIZE as usize {
            return None;
        }
        let dba_lo = u32::from_le_bytes(buf[0..4].try_into().unwrap());
        let dba_hi = u32::from_le_bytes(buf[4..8].try_into().unwrap());
        let dw3 = u32::from_le_bytes(buf[12..16].try_into().unwrap());
        // DBC is bits 21:0 and stores byte_count - 1.
        let dbc = (dw3 & 0x3f_ffff) + 1;
        Some(Prd {
            dba: (dba_lo as u64) | ((dba_hi as u64) << 32),
            byte_count: dbc,
        })
    }
}

/// Per-port register and DMA state. A single port is modelled with one disk.
struct Port {
    clb: u64,  // command list base (PxCLB | PxCLBU << 32)
    fb: u64,   // FIS base (PxFB | PxFBU << 32)
    is: u32,   // interrupt status (RWC)
    ie: u32,   // interrupt enable
    cmd: u32,  // command and status (ST/FRE etc.); CR/FR are derived
    tfd: u32,  // task file data (status in bits 7:0, error in bits 15:8)
    sig: u32,  // signature
    ssts: u32, // SATA status
    sctl: u32, // SATA control
    serr: u32, // SATA error (RWC)
    sact: u32, // SATA active
    ci: u32,   // command issue
    /// True if a SATA disk is attached to this port.
    present: bool,
}

impl Port {
    fn new(present: bool) -> Self {
        let (sig, ssts, tfd) = if present {
            // Device ready, not busy.
            (SIG_SATA_DISK, SSTS_PRESENT, ATA_DRDY as u32)
        } else {
            // No device: DET=0, BSY-ish idle task file.
            (0, 0, 0x7f)
        };
        Port {
            clb: 0,
            fb: 0,
            is: 0,
            ie: 0,
            cmd: 0,
            tfd,
            sig,
            ssts,
            sctl: 0,
            serr: 0,
            sact: 0,
            ci: 0,
            present,
        }
    }

    /// PxCMD as read by the host: stored bits plus the derived CR/FR running
    /// status bits (CR follows ST, FR follows FRE).
    fn cmd_read(&self) -> u32 {
        let mut v = self.cmd & !(PXCMD_CR | PXCMD_FR);
        if self.cmd & PXCMD_ST != 0 {
            v |= PXCMD_CR;
        }
        if self.cmd & PXCMD_FRE != 0 {
            v |= PXCMD_FR;
        }
        v
    }
}

/// The AHCI host controller: generic registers, one port, and a backing disk.
pub struct AhciController<M: Mem> {
    base: u64,
    mem: M,

    // ---- Generic host control register state ----
    ghc: u32,
    is: u32, // HBA interrupt status (per-port bits, RWC)

    // ---- The single implemented port (port 0) ----
    port: Port,

    // ---- Port 0 backing disk ----
    disk: Vec<u8>,

    // ---- Number of command slots advertised (CAP.NCS), 1-based ----
    num_slots: u32,

    // ---- Interrupt aggregation ----
    interrupt_pending: bool,
}

impl<M: Mem> AhciController<M> {
    /// Build a controller at ABAR `base`, with guest memory `mem` and a single
    /// SATA disk backed by `disk` bytes (on port 0).
    pub fn new(base: u64, mem: M, disk: Vec<u8>) -> Self {
        // A port only reports a present device when media is actually attached.
        // With no disk, PxSSTS.DET reads 0 so the guest's libata probe skips the
        // port instead of timing out on IDENTIFY for a phantom drive.
        let present = !disk.is_empty();
        AhciController {
            base,
            mem,
            ghc: 0,
            is: 0,
            port: Port::new(present),
            disk,
            num_slots: 32,
            interrupt_pending: false,
        }
    }

    /// Build a controller whose disk has `sectors` logical blocks of
    /// [`SECTOR_SIZE`] bytes.
    pub fn with_sectors(base: u64, mem: M, sectors: u64) -> Self {
        Self::new(base, mem, vec![0u8; (sectors * SECTOR_SIZE) as usize])
    }

    /// Capacity of the port-0 disk in logical sectors.
    pub fn disk_sectors(&self) -> u64 {
        self.disk.len() as u64 / SECTOR_SIZE
    }

    /// Immutable view of the backing disk (for tests / inspection).
    pub fn disk(&self) -> &[u8] {
        &self.disk
    }

    /// Borrow the guest memory backing.
    pub fn mem(&self) -> &M {
        &self.mem
    }

    /// True if an interrupt is pending (a completion raised PxIS) and the
    /// relevant enables are set. An orchestrator polls this and injects.
    pub fn interrupt_pending(&self) -> bool {
        self.interrupt_pending
    }

    /// Clear the pending-interrupt latch (called after the interrupt is
    /// injected). Does not clear the sticky PxIS / IS register bits, which the
    /// guest acknowledges by write-1-clear.
    pub fn clear_interrupt(&mut self) {
        self.interrupt_pending = false;
    }

    /// CAP register value. Encodes:
    /// * NP   (bits 4:0): number of ports, 0-based => 1 port.
    /// * NCS  (bits 12:8): number of command slots, 0-based => 32 slots.
    /// * SAM  (bit 18): AHCI-only.
    /// * SSS  (bit 27): supports staggered spin-up = 0 here.
    /// * S64A (bit 31): supports 64-bit addressing.
    pub fn cap(&self) -> u32 {
        let np: u32 = 0; // 0-based => 1 port implemented.
        let ncs: u32 = (self.num_slots - 1) << CAP_NCS_SHIFT; // 0-based slot count.
        np | ncs | CAP_SAM | CAP_S64A
    }

    /// CAP2 register value. No extended capabilities advertised.
    pub fn cap2(&self) -> u32 {
        0
    }

    /// Ports Implemented bitmap: only port 0.
    pub fn pi(&self) -> u32 {
        1
    }

    /// True if AHCI mode is enabled (GHC.AE).
    fn ae(&self) -> bool {
        self.ghc & GHC_AE != 0
    }

    // ---- Register read ------------------------------------------------------

    /// Read a 32-bit register at ABAR-relative `offset`.
    fn read_reg32(&self, offset: u64) -> u32 {
        if offset >= PORT_BASE {
            return self.read_port_reg(offset);
        }
        match offset {
            REG_CAP => self.cap(),
            REG_GHC => self.ghc,
            REG_IS => self.is,
            REG_PI => self.pi(),
            REG_VS => AHCI_VERSION_1_3,
            REG_CCC_CTL | REG_CCC_PORTS | REG_EM_LOC | REG_EM_CTL | REG_BOHC => 0,
            REG_CAP2 => self.cap2(),
            _ => 0,
        }
    }

    /// Read a per-port register. Only port 0 is implemented; other ports read
    /// as zero.
    fn read_port_reg(&self, offset: u64) -> u32 {
        let port_index = (offset - PORT_BASE) / PORT_STRIDE;
        let rel = (offset - PORT_BASE) % PORT_STRIDE;
        if port_index != 0 {
            return 0;
        }
        let p = &self.port;
        match rel {
            PX_CLB => (p.clb & 0xffff_ffff) as u32,
            PX_CLBU => (p.clb >> 32) as u32,
            PX_FB => (p.fb & 0xffff_ffff) as u32,
            PX_FBU => (p.fb >> 32) as u32,
            PX_IS => p.is,
            PX_IE => p.ie,
            PX_CMD => p.cmd_read(),
            PX_TFD => p.tfd,
            PX_SIG => p.sig,
            PX_SSTS => p.ssts,
            PX_SCTL => p.sctl,
            PX_SERR => p.serr,
            PX_SACT => p.sact,
            PX_CI => p.ci,
            _ => 0,
        }
    }

    // ---- Register write -----------------------------------------------------

    /// Write a 32-bit `value` to the register at ABAR-relative `offset`.
    fn write_reg32(&mut self, offset: u64, value: u32) {
        if offset >= PORT_BASE {
            self.write_port_reg(offset, value);
            return;
        }
        match offset {
            // CAP / PI / VS / CAP2 are read-only.
            REG_CAP | REG_PI | REG_VS | REG_CAP2 => {}
            REG_GHC => self.write_ghc(value),
            // IS is per-port interrupt status, write-1-to-clear.
            REG_IS => self.is &= !value,
            _ => {}
        }
    }

    /// Apply a write to GHC, handling AE and HR (reset) edges.
    fn write_ghc(&mut self, value: u32) {
        if value & GHC_HR != 0 {
            // HBA reset: clear all state and self-clear HR. After reset AHCI
            // mode is disabled (AE=0) until re-enabled by software.
            self.hba_reset();
            return;
        }
        // AE and IE are writable; HR reads back 0 once reset is complete.
        self.ghc = value & !GHC_HR;
    }

    /// Full HBA reset (GHC.HR): return all registers to their power-on state.
    fn hba_reset(&mut self) {
        let present = self.port.present;
        self.ghc = 0;
        self.is = 0;
        self.port = Port::new(present);
        self.interrupt_pending = false;
    }

    /// Write a per-port register (port 0 only).
    fn write_port_reg(&mut self, offset: u64, value: u32) {
        let port_index = (offset - PORT_BASE) / PORT_STRIDE;
        let rel = (offset - PORT_BASE) % PORT_STRIDE;
        if port_index != 0 {
            return;
        }
        match rel {
            PX_CLB => {
                self.port.clb = (self.port.clb & !0xffff_ffff) | value as u64;
            }
            PX_CLBU => {
                self.port.clb = (self.port.clb & 0xffff_ffff) | ((value as u64) << 32);
            }
            PX_FB => {
                self.port.fb = (self.port.fb & !0xffff_ffff) | value as u64;
            }
            PX_FBU => {
                self.port.fb = (self.port.fb & 0xffff_ffff) | ((value as u64) << 32);
            }
            // PxIS / PxSERR are write-1-to-clear.
            PX_IS => self.port.is &= !value,
            PX_IE => self.port.ie = value,
            PX_CMD => {
                // ST/FRE (and other software-owned bits) are writable; CR/FR are
                // read-only status the controller derives. Mask the read-only
                // bits out of the stored value.
                self.port.cmd = value & !(PXCMD_CR | PXCMD_FR);
            }
            // PxTFD / PxSIG / PxSSTS are read-only (controller owned).
            PX_TFD | PX_SIG | PX_SSTS => {}
            PX_SCTL => self.port.sctl = value,
            PX_SERR => self.port.serr &= !value,
            PX_SACT => self.port.sact = value,
            PX_CI => {
                // The host sets bits to issue commands. Bits are only writable
                // to 1 (the controller clears them on completion); OR the new
                // bits in, then process if the port is started.
                self.port.ci |= value;
                self.process_commands();
            }
            _ => {}
        }
    }

    // ---- Command processing -------------------------------------------------

    /// Process all issued command slots (set PxCI bits) if the port is running.
    fn process_commands(&mut self) {
        // Commands only run when AHCI mode is enabled, the port is started
        // (ST), and FIS receive is enabled (FRE) so we can post the D2H FIS.
        if !self.ae() {
            return;
        }
        if self.port.cmd & PXCMD_ST == 0 || self.port.cmd & PXCMD_FRE == 0 {
            return;
        }
        if !self.port.present {
            return;
        }

        let slots = self.num_slots;
        for slot in 0..slots {
            let bit = 1u32 << slot;
            if self.port.ci & bit == 0 {
                continue;
            }
            self.run_slot(slot);
            // Clear the command-issue bit: the command has completed.
            self.port.ci &= !bit;
        }
    }

    /// Execute a single command slot.
    fn run_slot(&mut self, slot: u32) {
        // 1. Read the command header from the command list.
        let hdr_gpa = self.port.clb + (slot as u64) * CMD_HEADER_SIZE;
        let mut hdr_buf = [0u8; CMD_HEADER_SIZE as usize];
        if !self.mem.read(hdr_gpa, &mut hdr_buf) {
            return self.complete_error();
        }
        let hdr = match CommandHeader::parse(&hdr_buf) {
            Some(h) => h,
            None => return self.complete_error(),
        };

        // 2. Read the Command FIS at the head of the command table.
        let mut fis_buf = [0u8; CMD_FIS_SIZE as usize];
        if !self.mem.read(hdr.ctba, &mut fis_buf) {
            return self.complete_error();
        }
        let fis = match FisH2D::parse(&fis_buf) {
            Some(f) => f,
            None => return self.complete_error(),
        };

        // 3. Decode and dispatch on the ATA opcode.
        let result = match fis.command {
            ATA_CMD_READ_DMA_EXT => self.cmd_read_dma(&hdr, &fis),
            ATA_CMD_WRITE_DMA_EXT => self.cmd_write_dma(&hdr, &fis),
            ATA_CMD_IDENTIFY => self.cmd_identify(&hdr),
            _ => Err(()),
        };

        match result {
            Ok(prdbc) => {
                // Record the transferred byte count in command-header DW1
                // (PRDBC) and post a successful completion.
                self.write_prdbc(hdr_gpa, prdbc);
                self.complete_ok();
            }
            Err(()) => self.complete_error(),
        }
    }

    /// READ DMA EXT (0x25): disk -> guest memory via the PRDT.
    fn cmd_read_dma(&mut self, hdr: &CommandHeader, fis: &FisH2D) -> Result<u32, ()> {
        let (start, total) = self.transfer_bounds(fis)?;
        let segs = self.prdt_segments(hdr);
        let mut disk_off = start as usize;
        let mut remaining = total as usize;
        let mut moved = 0u32;
        for (gpa, len) in segs {
            if remaining == 0 {
                break;
            }
            let n = (len as usize).min(remaining);
            let dend = disk_off + n;
            if !self.mem.write(gpa, &self.disk[disk_off..dend]) {
                return Err(());
            }
            disk_off = dend;
            remaining -= n;
            moved += n as u32;
        }
        Ok(moved)
    }

    /// WRITE DMA EXT (0x35): guest memory -> disk via the PRDT.
    fn cmd_write_dma(&mut self, hdr: &CommandHeader, fis: &FisH2D) -> Result<u32, ()> {
        let (start, total) = self.transfer_bounds(fis)?;
        let segs = self.prdt_segments(hdr);
        let mut disk_off = start as usize;
        let mut remaining = total as usize;
        let mut moved = 0u32;
        for (gpa, len) in segs {
            if remaining == 0 {
                break;
            }
            let n = (len as usize).min(remaining);
            let dend = disk_off + n;
            if !self.mem.read(gpa, &mut self.disk[disk_off..dend]) {
                return Err(());
            }
            disk_off = dend;
            remaining -= n;
            moved += n as u32;
        }
        Ok(moved)
    }

    /// IDENTIFY DEVICE (0xEC): build the 512-byte block and DMA it to the PRDT.
    fn cmd_identify(&mut self, hdr: &CommandHeader) -> Result<u32, ()> {
        let block = self.build_identify();
        let segs = self.prdt_segments(hdr);
        let mut src_off = 0usize;
        let mut moved = 0u32;
        for (gpa, len) in segs {
            if src_off >= block.len() {
                break;
            }
            let n = (len as usize).min(block.len() - src_off);
            if !self.mem.write(gpa, &block[src_off..src_off + n]) {
                return Err(());
            }
            src_off += n;
            moved += n as u32;
        }
        Ok(moved)
    }

    /// Resolve the (disk byte offset, byte length) for a READ/WRITE DMA EXT,
    /// validating against the disk capacity. Returns Err on out-of-range.
    fn transfer_bounds(&self, fis: &FisH2D) -> Result<(u64, u64), ()> {
        // EXT count: 0 means 65536 sectors.
        let count = if fis.count == 0 { 65536 } else { fis.count } as u64;
        let total = count * SECTOR_SIZE;
        let start = fis.lba.checked_mul(SECTOR_SIZE).ok_or(())?;
        let end = start.checked_add(total).ok_or(())?;
        if end > self.disk.len() as u64 {
            return Err(());
        }
        Ok((start, total))
    }

    /// Walk the PRDT for `hdr`, returning the list of (gpa, byte_len) segments.
    /// Entries that fail to read or have a zero count are skipped.
    fn prdt_segments(&self, hdr: &CommandHeader) -> Vec<(u64, u32)> {
        let mut segs = Vec::with_capacity(hdr.prdtl as usize);
        let prdt_base = hdr.ctba + PRDT_OFFSET;
        for i in 0..hdr.prdtl as u64 {
            let mut buf = [0u8; PRD_SIZE as usize];
            if !self.mem.read(prdt_base + i * PRD_SIZE, &mut buf) {
                break;
            }
            if let Some(prd) = Prd::parse(&buf) {
                segs.push((prd.dba, prd.byte_count));
            }
        }
        segs
    }

    /// Write the PRD byte count into command-header DW1 (PRDBC).
    fn write_prdbc(&mut self, hdr_gpa: u64, prdbc: u32) {
        let _ = self.mem.write(hdr_gpa + 4, &prdbc.to_le_bytes());
    }

    // ---- Completion / FIS receive ------------------------------------------

    /// Post a successful Device-to-Host Register FIS and raise the DHRS
    /// interrupt.
    fn complete_ok(&mut self) {
        let status = ATA_DRDY;
        self.port.tfd = status as u32; // error byte = 0.
        self.post_d2h_fis(status, 0);
        self.raise_port_interrupt(PXIS_DHRS);
    }

    /// Post an errored D2H Register FIS (ABRT) and raise the task-file-error
    /// interrupt.
    fn complete_error(&mut self) {
        let status = ATA_DRDY | ATA_ERR;
        let error = ATA_ERR_ABRT;
        // PxTFD: status in bits 7:0, error in bits 15:8.
        self.port.tfd = (status as u32) | ((error as u32) << 8);
        self.post_d2h_fis(status, error);
        self.raise_port_interrupt(PXIS_DHRS | PXIS_TFES);
    }

    /// Write a Device-to-Host Register FIS into the received-FIS area at
    /// `PxFB + 0x40` (the RFIS slot), if FIS receive is enabled.
    fn post_d2h_fis(&mut self, status: u8, error: u8) {
        if self.port.cmd & PXCMD_FRE == 0 {
            return;
        }
        let mut fis = [0u8; 20];
        fis[0] = FIS_TYPE_REG_D2H;
        // byte 1: bit6 = Interrupt bit.
        fis[1] = 1 << 6;
        fis[2] = status;
        fis[3] = error;
        let _ = self.mem.write(self.port.fb + RFIS_D2H_OFFSET, &fis);
    }

    /// Set the given PxIS bits and, if enabled, latch the HBA interrupt.
    fn raise_port_interrupt(&mut self, bits: u32) {
        self.port.is |= bits;
        // HBA IS bit for port 0.
        self.is |= 1;
        // An interrupt is pending if the global IE is set, the port's PxIE
        // covers one of the raised bits, and AHCI mode is enabled.
        if self.ghc & GHC_IE != 0 && self.port.ie & bits != 0 {
            self.interrupt_pending = true;
        }
    }

    /// Build a 512-byte ATA IDENTIFY DEVICE block describing the port-0 disk.
    fn build_identify(&self) -> Vec<u8> {
        let mut words = [0u16; 256];
        let total = self.disk_sectors();
        let lba28 = total.min(0x0fff_ffff) as u32;

        // Word 0: general configuration; 0x0040 => fixed (non-removable).
        words[0] = 0x0040;
        words[1] = 16383; // cylinders
        words[3] = 16; // heads
        words[6] = 63; // sectors per track
        put_ata_string(&mut words[10..20], "RAX-AHCI-0000000001 ");
        put_ata_string(&mut words[23..27], "1.0     ");
        put_ata_string(
            &mut words[27..47],
            "RAX Virtual SATA Disk                   ",
        );
        // Word 47: max sectors per READ/WRITE MULTIPLE.
        words[47] = 0x8000 | 16;
        // Word 49: capabilities. Bit 9 = LBA, bit 8 = DMA.
        words[49] = (1 << 9) | (1 << 8);
        words[50] = 0x4000;
        words[53] = 0x0007;
        // Words 60-61: LBA28 total sectors.
        words[60] = (lba28 & 0xffff) as u16;
        words[61] = (lba28 >> 16) as u16;
        // Word 80: major version (ATA-8 etc.).
        words[80] = 0x00f0;
        // Word 82/83: command sets; bit 10 of 83 = LBA48 supported.
        words[82] = (1 << 14) | (1 << 12);
        words[83] = (1 << 14) | (1 << 10);
        words[84] = 1 << 14;
        words[85] = (1 << 14) | (1 << 12);
        words[86] = 1 << 10;
        words[87] = 1 << 14;
        // Words 100-103: LBA48 total sectors.
        words[100] = (total & 0xffff) as u16;
        words[101] = ((total >> 16) & 0xffff) as u16;
        words[102] = ((total >> 32) & 0xffff) as u16;
        words[103] = ((total >> 48) & 0xffff) as u16;

        let mut out = vec![0u8; 512];
        for (i, w) in words.iter().enumerate() {
            let b = w.to_le_bytes();
            out[i * 2] = b[0];
            out[i * 2 + 1] = b[1];
        }
        out
    }

    // ---- Test / introspection accessors ------------------------------------

    /// Current PxIS value (for tests).
    pub fn port_is(&self) -> u32 {
        self.port.is
    }
    /// Current PxCI value (for tests).
    pub fn port_ci(&self) -> u32 {
        self.port.ci
    }
    /// Current PxTFD value (for tests).
    pub fn port_tfd(&self) -> u32 {
        self.port.tfd
    }
    /// HBA-level IS value (for tests).
    pub fn hba_is(&self) -> u32 {
        self.is
    }
}

/// Write an ATA string into a fixed field; ATA string fields are byte-swapped
/// per 16-bit word and space-padded.
fn put_ata_string(words: &mut [u16], s: &str) {
    let bytes = s.as_bytes();
    for (i, w) in words.iter_mut().enumerate() {
        let hi = bytes.get(i * 2).copied().unwrap_or(b' ');
        let lo = bytes.get(i * 2 + 1).copied().unwrap_or(b' ');
        *w = ((hi as u16) << 8) | (lo as u16);
    }
}

impl<M: Mem + Send> MmioDevice for AhciController<M> {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        let offset = addr.wrapping_sub(self.base);
        let aligned = offset & !0x3;
        let byte_in_reg = (offset & 0x3) as usize;
        let bytes = self.read_reg32(aligned).to_le_bytes();
        for (i, out) in data.iter_mut().enumerate() {
            let pos = byte_in_reg + i;
            *out = if pos < 4 { bytes[pos] } else { 0 };
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        let offset = addr.wrapping_sub(self.base);
        // Read-modify-write so a sub-word access preserves the other bytes.
        let aligned = offset & !0x3;
        let byte_in_reg = (offset & 0x3) as usize;
        let mut bytes = self.read_reg32(aligned).to_le_bytes();
        for (i, byte) in data.iter().enumerate() {
            let pos = byte_in_reg + i;
            if pos < 4 {
                bytes[pos] = *byte;
            }
        }
        self.write_reg32(aligned, u32::from_le_bytes(bytes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::bus::MmioDevice;

    const BASE: u64 = 0xfeb0_0000;

    /// Per-port register absolute offset for port 0.
    fn px(rel: u64) -> u64 {
        PORT_BASE + rel
    }

    fn dev(sectors: u64) -> AhciController<VecMem> {
        AhciController::with_sectors(BASE, VecMem::new(0x10_0000), sectors)
    }

    fn read32(d: &mut AhciController<VecMem>, off: u64) -> u32 {
        let mut buf = [0u8; 4];
        d.read(BASE + off, &mut buf);
        u32::from_le_bytes(buf)
    }

    fn write32(d: &mut AhciController<VecMem>, off: u64, value: u32) {
        d.write(BASE + off, &value.to_le_bytes());
    }

    // ---- Generic host control register basics ----

    #[test]
    fn cap_reports_capabilities() {
        let mut d = dev(64);
        let cap = read32(&mut d, REG_CAP);
        // NP (0-based) == 0 => 1 port.
        assert_eq!(cap & CAP_NP_MASK, 0);
        // NCS (0-based) == 31 => 32 command slots.
        assert_eq!((cap >> CAP_NCS_SHIFT) & CAP_NCS_MASK, 31);
        // 64-bit addressing and AHCI-only mode supported.
        assert_ne!(cap & CAP_S64A, 0);
        assert_ne!(cap & CAP_SAM, 0);
    }

    #[test]
    fn version_is_1_3() {
        let mut d = dev(64);
        assert_eq!(read32(&mut d, REG_VS), 0x0001_0300);
        assert_eq!(read32(&mut d, REG_VS), AHCI_VERSION_1_3);
    }

    #[test]
    fn ports_implemented_is_port0() {
        let mut d = dev(64);
        assert_eq!(read32(&mut d, REG_PI), 1);
    }

    #[test]
    fn cap2_reads_zero() {
        let mut d = dev(64);
        assert_eq!(read32(&mut d, REG_CAP2), 0);
    }

    #[test]
    fn cap_pi_vs_are_read_only() {
        let mut d = dev(64);
        let cap = read32(&mut d, REG_CAP);
        write32(&mut d, REG_CAP, 0xdead_beef);
        assert_eq!(read32(&mut d, REG_CAP), cap);
        write32(&mut d, REG_PI, 0xffff_ffff);
        assert_eq!(read32(&mut d, REG_PI), 1);
        write32(&mut d, REG_VS, 0x1234);
        assert_eq!(read32(&mut d, REG_VS), AHCI_VERSION_1_3);
    }

    // ---- GHC AE / IE / HR ----

    #[test]
    fn ghc_ae_and_ie_writable() {
        let mut d = dev(64);
        assert_eq!(read32(&mut d, REG_GHC), 0);
        write32(&mut d, REG_GHC, GHC_AE);
        assert_ne!(read32(&mut d, REG_GHC) & GHC_AE, 0, "AE latches");
        write32(&mut d, REG_GHC, GHC_AE | GHC_IE);
        assert_ne!(read32(&mut d, REG_GHC) & GHC_IE, 0, "IE latches");
    }

    #[test]
    fn ghc_hr_resets_and_self_clears() {
        let mut d = dev(64);
        // Program some state.
        write32(&mut d, REG_GHC, GHC_AE | GHC_IE);
        write32(&mut d, px(PX_CLB), 0x1_0000);
        write32(&mut d, px(PX_CMD), PXCMD_ST | PXCMD_FRE);
        // Issue HBA reset.
        write32(&mut d, REG_GHC, GHC_HR);
        // HR self-clears; AE is back to 0; port state cleared.
        assert_eq!(read32(&mut d, REG_GHC) & GHC_HR, 0, "HR self-clears");
        assert_eq!(read32(&mut d, REG_GHC) & GHC_AE, 0, "AE cleared by reset");
        assert_eq!(read32(&mut d, px(PX_CLB)), 0, "PxCLB cleared by reset");
        assert_eq!(read32(&mut d, px(PX_CMD)), 0, "PxCMD cleared by reset");
        // Device presence survives the reset.
        assert_eq!(read32(&mut d, px(PX_SIG)), SIG_SATA_DISK);
    }

    // ---- Device presence (PxSIG / PxSSTS) ----

    #[test]
    fn port_signature_is_sata_disk() {
        let mut d = dev(64);
        assert_eq!(read32(&mut d, px(PX_SIG)), SIG_SATA_DISK);
        assert_eq!(read32(&mut d, px(PX_SIG)), 0x0000_0101);
    }

    #[test]
    fn port_ssts_reports_device_present() {
        let mut d = dev(64);
        let ssts = read32(&mut d, px(PX_SSTS));
        // DET == 3 (present + Phy).
        assert_eq!(ssts & 0xf, SSTS_DET_PRESENT);
        // IPM == active.
        assert_ne!(ssts & (0xf << 8), 0);
    }

    #[test]
    fn port_tfd_reports_ready() {
        let mut d = dev(64);
        // Device ready, not busy.
        assert_eq!(
            read32(&mut d, px(PX_TFD)) & (ATA_DRDY as u32),
            ATA_DRDY as u32
        );
        assert_eq!(read32(&mut d, px(PX_TFD)) & (ATA_BSY as u32), 0);
    }

    // ---- PxCMD ST / FRE -> CR / FR ----

    #[test]
    fn pxcmd_st_fre_drive_cr_fr() {
        let mut d = dev(64);
        assert_eq!(read32(&mut d, px(PX_CMD)), 0);
        // Set FRE first; FR (FIS receive running) should follow.
        write32(&mut d, px(PX_CMD), PXCMD_FRE);
        let v = read32(&mut d, px(PX_CMD));
        assert_ne!(v & PXCMD_FRE, 0);
        assert_ne!(v & PXCMD_FR, 0, "FR follows FRE");
        assert_eq!(v & PXCMD_CR, 0, "CR not yet set");
        // Set ST; CR (command list running) should follow.
        write32(&mut d, px(PX_CMD), PXCMD_ST | PXCMD_FRE);
        let v = read32(&mut d, px(PX_CMD));
        assert_ne!(v & PXCMD_CR, 0, "CR follows ST");
        assert_ne!(v & PXCMD_FR, 0);
        // Clearing ST clears CR.
        write32(&mut d, px(PX_CMD), PXCMD_FRE);
        assert_eq!(
            read32(&mut d, px(PX_CMD)) & PXCMD_CR,
            0,
            "CR clears with ST"
        );
    }

    // ---- PxCLB / PxFB programming (incl. 64-bit halves) ----

    #[test]
    fn pxclb_pxfb_programming() {
        let mut d = dev(64);
        write32(&mut d, px(PX_CLB), 0xabcd_1000);
        write32(&mut d, px(PX_CLBU), 0x0000_0001);
        assert_eq!(read32(&mut d, px(PX_CLB)), 0xabcd_1000);
        assert_eq!(read32(&mut d, px(PX_CLBU)), 0x0000_0001);

        write32(&mut d, px(PX_FB), 0x1234_2000);
        write32(&mut d, px(PX_FBU), 0x0000_00ff);
        assert_eq!(read32(&mut d, px(PX_FB)), 0x1234_2000);
        assert_eq!(read32(&mut d, px(PX_FBU)), 0x0000_00ff);
    }

    #[test]
    fn pxis_pxserr_write_one_clear() {
        let mut d = dev(64);
        // These start at 0; force-raise via an interrupt path is covered later.
        // Directly verify the RWC semantics: a write of 1 clears whatever is set.
        // Program some IS bits by issuing a command (see round-trip test); here
        // just verify writing 0 leaves them, and the register is host-writable
        // only to clear. We set IE then ensure write-1-clear of IS.
        write32(&mut d, px(PX_IE), 0xffff_ffff);
        assert_eq!(read32(&mut d, px(PX_IE)), 0xffff_ffff);
        // SERR write-1-clear: nothing set so it stays 0.
        write32(&mut d, px(PX_SERR), 0xffff_ffff);
        assert_eq!(read32(&mut d, px(PX_SERR)), 0);
    }

    // ---- Helpers to build command structures in guest memory ----

    /// Lay out a command header at `clb` slot 0 pointing at command table
    /// `ctba`, with `prdtl` PRD entries and the write flag.
    fn write_cmd_header(
        d: &mut AhciController<VecMem>,
        clb: u64,
        ctba: u64,
        prdtl: u16,
        write: bool,
    ) {
        let mut hdr = [0u8; CMD_HEADER_SIZE as usize];
        // CFL = 5 DWORDs (a 20-byte H2D register FIS).
        let mut dw0 = 5u32;
        if write {
            dw0 |= 1 << 6;
        }
        dw0 |= (prdtl as u32) << 16;
        hdr[0..4].copy_from_slice(&dw0.to_le_bytes());
        hdr[8..12].copy_from_slice(&(ctba as u32).to_le_bytes());
        hdr[12..16].copy_from_slice(&((ctba >> 32) as u32).to_le_bytes());
        d.mem_write(clb, &hdr);
    }

    /// Build an H2D Register FIS for `command` at the head of command table
    /// `ctba`.
    fn write_cmd_fis(d: &mut AhciController<VecMem>, ctba: u64, command: u8, lba: u64, count: u16) {
        let mut fis = [0u8; CMD_FIS_SIZE as usize];
        fis[0] = FIS_TYPE_REG_H2D;
        fis[1] = 1 << 7; // C bit: this is a command.
        fis[2] = command;
        fis[4] = (lba & 0xff) as u8;
        fis[5] = ((lba >> 8) & 0xff) as u8;
        fis[6] = ((lba >> 16) & 0xff) as u8;
        fis[7] = 1 << 6; // device: LBA mode.
        fis[8] = ((lba >> 24) & 0xff) as u8;
        fis[9] = ((lba >> 32) & 0xff) as u8;
        fis[10] = ((lba >> 40) & 0xff) as u8;
        fis[12] = (count & 0xff) as u8;
        fis[13] = ((count >> 8) & 0xff) as u8;
        d.mem_write(ctba, &fis);
    }

    /// Write a single PRD entry into command table `ctba` at PRD index `i`.
    fn write_prd(d: &mut AhciController<VecMem>, ctba: u64, i: u64, dba: u64, bytes: u32) {
        let mut prd = [0u8; PRD_SIZE as usize];
        prd[0..4].copy_from_slice(&(dba as u32).to_le_bytes());
        prd[4..8].copy_from_slice(&((dba >> 32) as u32).to_le_bytes());
        // DBC stores byte_count - 1 in bits 21:0.
        let dbc = bytes - 1;
        prd[12..16].copy_from_slice(&dbc.to_le_bytes());
        d.mem_write(ctba + PRDT_OFFSET + i * PRD_SIZE, &prd);
    }

    /// Bring the port up: AHCI enabled, interrupts enabled, program CLB/FB,
    /// then set FRE + ST.
    fn bring_up(d: &mut AhciController<VecMem>, clb: u64, fb: u64) {
        write32(d, REG_GHC, GHC_AE | GHC_IE);
        write32(d, px(PX_CLB), clb as u32);
        write32(d, px(PX_CLBU), (clb >> 32) as u32);
        write32(d, px(PX_FB), fb as u32);
        write32(d, px(PX_FBU), (fb >> 32) as u32);
        write32(d, px(PX_IE), 0xffff_ffff);
        write32(d, px(PX_CMD), PXCMD_ST | PXCMD_FRE);
    }

    // ---- Command-issue: IDENTIFY ----

    #[test]
    fn identify_command_round_trip() {
        let mut d = dev(0x800); // 0x800 sectors.
        let clb = 0x1000u64;
        let fb = 0x2000u64;
        let ctba = 0x3000u64;
        let buf = 0x8000u64;
        bring_up(&mut d, clb, fb);

        // One PRD pointing at a 512-byte buffer.
        write_cmd_header(&mut d, clb, ctba, 1, false);
        write_cmd_fis(&mut d, ctba, ATA_CMD_IDENTIFY, 0, 0);
        write_prd(&mut d, ctba, 0, buf, 512);

        // Issue slot 0.
        write32(&mut d, px(PX_CI), 1);

        // Command completed: PxCI cleared, PxIS DHRS set, HBA IS bit set.
        assert_eq!(
            read32(&mut d, px(PX_CI)) & 1,
            0,
            "PxCI cleared on completion"
        );
        assert_ne!(read32(&mut d, px(PX_IS)) & PXIS_DHRS, 0, "DHRS raised");
        assert_ne!(read32(&mut d, REG_IS) & 1, 0, "HBA IS port0 bit set");
        assert!(d.interrupt_pending(), "interrupt pending");
        // No task-file error.
        assert_eq!(read32(&mut d, px(PX_TFD)) & (ATA_ERR as u32), 0);

        // IDENTIFY word 0 == 0x0040, LBA48 sector count at words 100-103.
        let mut w0 = [0u8; 2];
        d.mem().read(buf, &mut w0);
        assert_eq!(u16::from_le_bytes(w0), 0x0040);
        let mut sectors = [0u8; 8];
        d.mem().read(buf + 100 * 2, &mut sectors);
        assert_eq!(u64::from_le_bytes(sectors), 0x800);
    }

    // ---- Command-issue: WRITE then READ DMA EXT round trip ----

    #[test]
    fn write_then_read_dma_ext_round_trip() {
        let mut d = dev(64);
        let clb = 0x1000u64;
        let fb = 0x2000u64;
        let ctba = 0x3000u64;
        let wbuf = 0x8000u64;
        let rbuf = 0x9000u64;
        bring_up(&mut d, clb, fb);

        // Stage a recognizable 512-byte pattern in guest memory.
        let pattern: Vec<u8> = (0..512usize).map(|i| (i & 0xff) as u8).collect();
        d.mem_write(wbuf, &pattern);

        // WRITE DMA EXT: LBA 5, 1 sector, from wbuf.
        write_cmd_header(&mut d, clb, ctba, 1, true);
        write_cmd_fis(&mut d, ctba, ATA_CMD_WRITE_DMA_EXT, 5, 1);
        write_prd(&mut d, ctba, 0, wbuf, 512);
        write32(&mut d, px(PX_CI), 1);

        // Completion + disk now holds the pattern at LBA 5.
        assert_eq!(read32(&mut d, px(PX_CI)) & 1, 0);
        assert_eq!(read32(&mut d, px(PX_TFD)) & (ATA_ERR as u32), 0);
        let off = 5 * SECTOR_SIZE as usize;
        assert_eq!(&d.disk()[off..off + 512], &pattern[..]);

        // READ DMA EXT: same LBA into rbuf via a fresh command table.
        let ctba2 = 0x4000u64;
        write_cmd_header(&mut d, clb, ctba2, 1, false);
        write_cmd_fis(&mut d, ctba2, ATA_CMD_READ_DMA_EXT, 5, 1);
        write_prd(&mut d, ctba2, 0, rbuf, 512);
        // Clear prior PxIS then issue again.
        write32(&mut d, px(PX_IS), 0xffff_ffff);
        write32(&mut d, px(PX_CI), 1);

        assert_eq!(read32(&mut d, px(PX_CI)) & 1, 0);
        assert_ne!(read32(&mut d, px(PX_IS)) & PXIS_DHRS, 0);
        let mut readback = vec![0u8; 512];
        d.mem().read(rbuf, &mut readback);
        assert_eq!(readback, pattern);
    }

    // ---- Scatter/gather across multiple PRD entries ----

    #[test]
    fn read_dma_ext_scatter_gather() {
        let mut d = dev(64);
        let clb = 0x1000u64;
        let fb = 0x2000u64;
        let ctba = 0x3000u64;
        bring_up(&mut d, clb, fb);

        // Seed two sectors (LBA 0..1) on the disk with a pattern.
        let pattern: Vec<u8> = (0..1024usize).map(|i| ((i * 7) & 0xff) as u8).collect();
        // Write via the WRITE path through one contiguous buffer first.
        let staging = 0x8000u64;
        d.mem_write(staging, &pattern);
        write_cmd_header(&mut d, clb, ctba, 1, true);
        write_cmd_fis(&mut d, ctba, ATA_CMD_WRITE_DMA_EXT, 0, 2);
        write_prd(&mut d, ctba, 0, staging, 1024);
        write32(&mut d, px(PX_CI), 1);
        assert_eq!(read32(&mut d, px(PX_TFD)) & (ATA_ERR as u32), 0);

        // READ DMA EXT of both sectors split across TWO PRD entries
        // (512 bytes into buf_a, 512 into buf_b).
        let buf_a = 0xa000u64;
        let buf_b = 0xb000u64;
        let ctba2 = 0x4000u64;
        write_cmd_header(&mut d, clb, ctba2, 2, false);
        write_cmd_fis(&mut d, ctba2, ATA_CMD_READ_DMA_EXT, 0, 2);
        write_prd(&mut d, ctba2, 0, buf_a, 512);
        write_prd(&mut d, ctba2, 1, buf_b, 512);
        write32(&mut d, px(PX_CI), 1);

        assert_eq!(read32(&mut d, px(PX_CI)) & 1, 0);
        let mut a = vec![0u8; 512];
        let mut b = vec![0u8; 512];
        d.mem().read(buf_a, &mut a);
        d.mem().read(buf_b, &mut b);
        assert_eq!(&a[..], &pattern[0..512]);
        assert_eq!(&b[..], &pattern[512..1024]);
    }

    // ---- Out-of-range read errors via PxTFD ----

    #[test]
    fn read_dma_ext_out_of_range_errors() {
        let mut d = dev(4); // only 4 sectors.
        let clb = 0x1000u64;
        let fb = 0x2000u64;
        let ctba = 0x3000u64;
        bring_up(&mut d, clb, fb);

        write_cmd_header(&mut d, clb, ctba, 1, false);
        // LBA 100 is past the 4-sector disk.
        write_cmd_fis(&mut d, ctba, ATA_CMD_READ_DMA_EXT, 100, 1);
        write_prd(&mut d, ctba, 0, 0x8000, 512);
        write32(&mut d, px(PX_CI), 1);

        // Command still clears PxCI, but PxTFD reports ERR/ABRT and TFES rises.
        assert_eq!(read32(&mut d, px(PX_CI)) & 1, 0);
        let tfd = read32(&mut d, px(PX_TFD));
        assert_ne!(tfd & (ATA_ERR as u32), 0, "ERR set");
        assert_eq!((tfd >> 8) & 0xff, ATA_ERR_ABRT as u32, "ABRT in error byte");
        assert_ne!(read32(&mut d, px(PX_IS)) & PXIS_TFES, 0, "TFES raised");
    }

    // ---- Commands do not run unless ST + FRE are set ----

    #[test]
    fn ci_does_not_run_without_st() {
        let mut d = dev(64);
        let clb = 0x1000u64;
        let fb = 0x2000u64;
        let ctba = 0x3000u64;
        // Enable AHCI + program bases + FRE, but DO NOT set ST.
        write32(&mut d, REG_GHC, GHC_AE | GHC_IE);
        write32(&mut d, px(PX_CLB), clb as u32);
        write32(&mut d, px(PX_FB), fb as u32);
        write32(&mut d, px(PX_IE), 0xffff_ffff);
        write32(&mut d, px(PX_CMD), PXCMD_FRE);

        write_cmd_header(&mut d, clb, ctba, 1, false);
        write_cmd_fis(&mut d, ctba, ATA_CMD_IDENTIFY, 0, 0);
        write_prd(&mut d, ctba, 0, 0x8000, 512);
        write32(&mut d, px(PX_CI), 1);

        // ST not set: the command stays pending (PxCI still 1, no interrupt).
        assert_eq!(
            read32(&mut d, px(PX_CI)) & 1,
            1,
            "PxCI remains pending without ST"
        );
        assert_eq!(read32(&mut d, px(PX_IS)), 0);
    }

    // ---- IS write-1-clear acknowledges interrupts ----

    #[test]
    fn pxis_and_hba_is_write_one_clear() {
        let mut d = dev(64);
        let clb = 0x1000u64;
        let fb = 0x2000u64;
        let ctba = 0x3000u64;
        bring_up(&mut d, clb, fb);

        write_cmd_header(&mut d, clb, ctba, 1, false);
        write_cmd_fis(&mut d, ctba, ATA_CMD_IDENTIFY, 0, 0);
        write_prd(&mut d, ctba, 0, 0x8000, 512);
        write32(&mut d, px(PX_CI), 1);

        // Both PxIS and HBA IS are set after completion.
        assert_ne!(read32(&mut d, px(PX_IS)) & PXIS_DHRS, 0);
        assert_ne!(read32(&mut d, REG_IS) & 1, 0);

        // Acknowledge by writing 1s.
        write32(&mut d, px(PX_IS), PXIS_DHRS);
        write32(&mut d, REG_IS, 1);
        assert_eq!(read32(&mut d, px(PX_IS)) & PXIS_DHRS, 0, "PxIS cleared");
        assert_eq!(read32(&mut d, REG_IS) & 1, 0, "HBA IS cleared");
    }

    // ---- Disk capacity ----

    #[test]
    fn disk_capacity() {
        let d = dev(2048);
        assert_eq!(d.disk_sectors(), 2048);
        assert_eq!(d.disk().len(), 2048 * SECTOR_SIZE as usize);
    }

    // ---- Structure parsing unit tests ----

    #[test]
    fn command_header_parse() {
        let mut hdr = [0u8; 32];
        let dw0 = 5u32 | (1 << 6) | (3u32 << 16); // CFL=5, write, PRDTL=3.
        hdr[0..4].copy_from_slice(&dw0.to_le_bytes());
        hdr[8..12].copy_from_slice(&0x1234_5000u32.to_le_bytes());
        hdr[12..16].copy_from_slice(&0x0000_0001u32.to_le_bytes());
        let h = CommandHeader::parse(&hdr).unwrap();
        assert_eq!(h.cfl, 5);
        assert!(h.write);
        assert!(!h.atapi);
        assert_eq!(h.prdtl, 3);
        assert_eq!(h.ctba, 0x1_1234_5000);
        assert!(CommandHeader::parse(&hdr[..16]).is_none());
    }

    #[test]
    fn fis_h2d_parse() {
        let mut fis = [0u8; 20];
        fis[0] = FIS_TYPE_REG_H2D;
        fis[2] = ATA_CMD_READ_DMA_EXT;
        fis[4] = 0x10;
        fis[5] = 0x20;
        fis[6] = 0x30;
        fis[8] = 0x40;
        fis[12] = 8;
        let f = FisH2D::parse(&fis).unwrap();
        assert_eq!(f.command, ATA_CMD_READ_DMA_EXT);
        // LBA[7:0]=0x10, [15:8]=0x20, [23:16]=0x30, [31:24]=0x40 => 0x40302010.
        assert_eq!(f.lba, 0x4030_2010);
        assert_eq!(f.count, 8);
        // Wrong FIS type rejected.
        fis[0] = 0x00;
        assert!(FisH2D::parse(&fis).is_none());
    }

    #[test]
    fn prd_parse_byte_count_is_one_based() {
        let mut prd = [0u8; 16];
        prd[0..4].copy_from_slice(&0x8000u32.to_le_bytes());
        // DBC = 511 => byte_count 512.
        prd[12..16].copy_from_slice(&511u32.to_le_bytes());
        let p = Prd::parse(&prd).unwrap();
        assert_eq!(p.dba, 0x8000);
        assert_eq!(p.byte_count, 512);
    }
}

// Test-only helper: write directly into the controller's guest memory. Defined
// outside the `tests` module so the `cfg(test)` impl can call it but it does not
// leak into non-test builds.
#[cfg(test)]
impl<M: Mem> AhciController<M> {
    fn mem_write(&mut self, gpa: u64, buf: &[u8]) {
        self.mem.write(gpa, buf);
    }
}
