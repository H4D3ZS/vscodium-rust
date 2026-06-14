//! NVMe 1.4 controller device (BAR0 MMIO register file).
//!
//! This implements the *controller register interface* and admin/IO command
//! state machine of an NVM Express controller, per the "NVM Express Base
//! Specification, Revision 1.4". The controller is wired into the machine as an
//! [`MmioDevice`] occupying the BAR0 memory window (>= 0x1000 bytes: the
//! controller registers live in 0x00..0x1000 and the doorbell array starts at
//! 0x1000).
//!
//! A single namespace (NSID 1) is backed by an in-memory disk (`Vec<u8>`), in
//! the same style as the virtio-blk device.
//!
//! # Controller register layout (offsets from BAR0)
//!
//! | Offset | Name   | Dir | Notes                                              |
//! |--------|--------|-----|----------------------------------------------------|
//! | 0x00   | CAP    | RO  | Controller Capabilities (64-bit)                   |
//! | 0x08   | VS     | RO  | Version (= 0x0001_0400 for 1.4)                    |
//! | 0x0C   | INTMS  | RW  | Interrupt Mask Set (write-1-to-set)                |
//! | 0x10   | INTMC  | RW  | Interrupt Mask Clear (write-1-to-clear)            |
//! | 0x14   | CC     | RW  | Controller Configuration (EN, IOSQES, IOCQES, ...) |
//! | 0x1C   | CSTS   | RO  | Controller Status (RDY, CFS, ...)                  |
//! | 0x24   | AQA    | RW  | Admin Queue Attributes (ASQS/ACQS)                 |
//! | 0x28   | ASQ    | RW  | Admin Submission Queue Base Address (64-bit)       |
//! | 0x30   | ACQ    | RW  | Admin Completion Queue Base Address (64-bit)       |
//! | 0x1000+| SQyTDBL/CQyHDBL | RW | Doorbell array (stride per CAP.DSTRD)       |
//!
//! All registers are little-endian. 64-bit registers (CAP/ASQ/ACQ) may be
//! accessed as two 32-bit halves.
//!
//! # State machine
//!
//! The controller follows the spec's enable handshake:
//!
//! * On `CC.EN` 0 -> 1 the controller initializes the admin queues from
//!   AQA/ASQ/ACQ and asserts `CSTS.RDY`.
//! * On `CC.EN` 1 -> 0 (controller reset) the controller tears down all queue
//!   state and clears `CSTS.RDY`.
//!
//! # Doorbells
//!
//! Each queue has a doorbell at `0x1000 + (2*qid + (0 for SQ, 1 for CQ)) *
//! (4 << CAP.DSTRD)`. A submission-queue tail doorbell write triggers
//! processing of the newly-submitted command(s); a completion-queue head
//! doorbell write advances the consumer head.
//!
//! # What is deferred / partial
//!
//! * **PRP handling** uses only PRP1 (a single contiguous buffer). PRP2 and PRP
//!   lists (for transfers spanning more than two pages) are *not* walked; a
//!   command whose transfer would require them is failed. This is sufficient
//!   for single-page round trips, which the tests exercise.
//! * **Interrupts** (MSI-X / pin) are surfaced through
//!   [`NvmeController::take_interrupt`] for an orchestrator to inject; the
//!   device does not raise a guest interrupt line itself. CQ phase tags and the
//!   completion entries themselves are written to guest memory exactly.
//! * Only the admin commands actually used to bring a controller up are
//!   implemented (Identify Controller / Namespace / Active NS list, Create I/O
//!   Completion/Submission Queue, Set/Get Features). Other admin opcodes are
//!   completed with an Invalid Opcode status.
//! * NVM I/O implements Read (0x02), Write (0x01) and Flush (0x00, no-op).
//!   Dataset Management, Compare, Write Zeroes, etc. are not implemented.
//! * SGLs are not supported (the controller advertises PRP-only).
//!
//! These omissions are isolated to the DMA / command-payload path; the register
//! interface, EN->RDY handshake, queue setup, and doorbell tracking are
//! complete.

use super::bus::MmioDevice;

// Reuse the guest-memory DMA abstraction defined for the virtio transport so
// the command path can be unit-tested against a plain byte buffer.
pub use super::virtio::{Mem, VecMem};

// ---- Controller register offsets -------------------------------------------

pub const REG_CAP: u64 = 0x00; // Controller Capabilities (64-bit)
pub const REG_VS: u64 = 0x08; // Version
pub const REG_INTMS: u64 = 0x0c; // Interrupt Mask Set
pub const REG_INTMC: u64 = 0x10; // Interrupt Mask Clear
pub const REG_CC: u64 = 0x14; // Controller Configuration
pub const REG_CSTS: u64 = 0x1c; // Controller Status
pub const REG_AQA: u64 = 0x24; // Admin Queue Attributes
pub const REG_ASQ: u64 = 0x28; // Admin Submission Queue Base (64-bit)
pub const REG_ACQ: u64 = 0x30; // Admin Completion Queue Base (64-bit)

/// First byte of the doorbell array.
pub const REG_DOORBELL_BASE: u64 = 0x1000;

/// Size of the BAR0 MMIO window we claim (registers + a generous doorbell
/// array). 0x2000 leaves room for 0x1000/8 = 512 doorbells at stride 0.
pub const BAR0_SIZE: u64 = 0x2000;

// ---- Version ---------------------------------------------------------------

/// VS register value for NVMe 1.4: MJR=1, MNR=4, TER=0 => 0x0001_0400.
pub const NVME_VERSION_1_4: u32 = 0x0001_0400;

// ---- CC (Controller Configuration) bit fields ------------------------------

/// CC.EN (bit 0): enable.
pub const CC_EN: u32 = 1 << 0;
/// CC.CSS (bits 6:4): I/O command set selected.
pub const CC_CSS_SHIFT: u32 = 4;
pub const CC_CSS_MASK: u32 = 0x7;
/// CC.MPS (bits 10:7): host memory page size = 2^(12 + MPS).
pub const CC_MPS_SHIFT: u32 = 7;
pub const CC_MPS_MASK: u32 = 0xf;
/// CC.IOSQES (bits 19:16): I/O submission queue entry size = 2^IOSQES.
pub const CC_IOSQES_SHIFT: u32 = 16;
/// CC.IOCQES (bits 23:20): I/O completion queue entry size = 2^IOCQES.
pub const CC_IOCQES_SHIFT: u32 = 20;
pub const CC_QES_MASK: u32 = 0xf;
/// CC.SHN (bits 15:14): shutdown notification.
pub const CC_SHN_SHIFT: u32 = 14;
pub const CC_SHN_MASK: u32 = 0x3;

// ---- CSTS (Controller Status) bit fields -----------------------------------

/// CSTS.RDY (bit 0): controller ready (follows CC.EN once queues are armed).
pub const CSTS_RDY: u32 = 1 << 0;
/// CSTS.CFS (bit 1): controller fatal status.
pub const CSTS_CFS: u32 = 1 << 1;
/// CSTS.SHST (bits 3:2): shutdown status. 0b10 = shutdown complete.
pub const CSTS_SHST_SHIFT: u32 = 2;
pub const CSTS_SHST_COMPLETE: u32 = 0b10 << CSTS_SHST_SHIFT;

// ---- Command / completion entry geometry -----------------------------------

/// Submission queue entry size in bytes (fixed by the spec at 64).
pub const SQE_SIZE: u64 = 64;
/// Completion queue entry size in bytes (fixed by the spec at 16).
pub const CQE_SIZE: u64 = 16;
/// Logical block size for namespace 1 (512 bytes, LBADS=9).
pub const LBA_SIZE: u64 = 512;
/// Memory page size assumed for PRP handling (CC.MPS=0 => 4 KiB).
pub const PAGE_SIZE: u64 = 4096;

// ---- Admin opcodes (figure "Opcodes for Admin Commands") -------------------

pub const ADMIN_DELETE_IO_SQ: u8 = 0x00;
pub const ADMIN_CREATE_IO_SQ: u8 = 0x01;
pub const ADMIN_DELETE_IO_CQ: u8 = 0x04;
pub const ADMIN_CREATE_IO_CQ: u8 = 0x05;
pub const ADMIN_IDENTIFY: u8 = 0x06;
pub const ADMIN_SET_FEATURES: u8 = 0x09;
pub const ADMIN_GET_FEATURES: u8 = 0x0a;

// ---- NVM I/O opcodes --------------------------------------------------------

pub const NVM_FLUSH: u8 = 0x00;
pub const NVM_WRITE: u8 = 0x01;
pub const NVM_READ: u8 = 0x02;

// ---- Identify CNS values ----------------------------------------------------

pub const CNS_NAMESPACE: u32 = 0x00;
pub const CNS_CONTROLLER: u32 = 0x01;
pub const CNS_ACTIVE_NS_LIST: u32 = 0x02;

// ---- Set/Get Features feature ids ------------------------------------------

/// Number of Queues (FID 0x07).
pub const FID_NUM_QUEUES: u8 = 0x07;

// ---- Status codes (CQE DW3 bits 31:17 are the status field; SC bits 8:1) ---

/// Generic Command Status / Successful Completion.
pub const SC_SUCCESS: u16 = 0x00;
/// Invalid Command Opcode.
pub const SC_INVALID_OPCODE: u16 = 0x01;
/// Invalid Field in Command.
pub const SC_INVALID_FIELD: u16 = 0x02;
/// Data Transfer Error.
pub const SC_DATA_XFER_ERROR: u16 = 0x04;
/// LBA Out of Range (Command Specific, status type 0x01 => 0x80 here when ORed
/// with the SCT). For simplicity we keep all SCs in the generic type.
pub const SC_LBA_OUT_OF_RANGE: u16 = 0x80;

/// A parsed 64-byte NVMe submission queue entry (the fields we use).
#[derive(Clone, Copy, Debug, Default)]
pub struct SubmissionEntry {
    /// CDW0: opcode (bits 7:0).
    pub opcode: u8,
    /// CDW0: command identifier (bits 31:16).
    pub cid: u16,
    /// CDW1: namespace identifier.
    pub nsid: u32,
    /// DWORD 6/7: PRP entry 1 (64-bit).
    pub prp1: u64,
    /// DWORD 8/9: PRP entry 2 (64-bit).
    pub prp2: u64,
    /// DWORDs 10..15, command specific.
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
}

impl SubmissionEntry {
    /// Parse a submission entry from a 64-byte little-endian buffer.
    pub fn parse(buf: &[u8]) -> Option<SubmissionEntry> {
        if buf.len() < SQE_SIZE as usize {
            return None;
        }
        let dw = |i: usize| u32::from_le_bytes(buf[i * 4..i * 4 + 4].try_into().unwrap());
        let cdw0 = dw(0);
        Some(SubmissionEntry {
            opcode: (cdw0 & 0xff) as u8,
            cid: (cdw0 >> 16) as u16,
            nsid: dw(1),
            prp1: u64::from_le_bytes(buf[24..32].try_into().unwrap()),
            prp2: u64::from_le_bytes(buf[32..40].try_into().unwrap()),
            cdw10: dw(10),
            cdw11: dw(11),
            cdw12: dw(12),
            cdw13: dw(13),
            cdw14: dw(14),
            cdw15: dw(15),
        })
    }
}

/// State of one queue pair side (SQ or CQ).
#[derive(Clone, Copy, Debug, Default)]
pub struct Queue {
    /// Guest physical base address of the contiguous queue.
    pub base: u64,
    /// Number of entries (size). 0 => unused.
    pub size: u32,
    /// Producer index written via the tail doorbell (SQ) — next slot the host
    /// will fill. For a CQ, `tail` is where the controller posts next.
    pub tail: u32,
    /// Consumer index. For an SQ this is what the controller has consumed; for
    /// a CQ it is the host head written via the head doorbell.
    pub head: u32,
    /// Current phase tag the controller writes into posted CQEs (CQ only).
    pub phase: bool,
    /// For an I/O SQ: the CQ id its completions are posted to.
    pub cqid: u16,
    /// True once the queue has been created/armed.
    pub active: bool,
}

impl Queue {
    fn reset(&mut self) {
        *self = Queue::default();
    }
}

/// The NVMe controller: register file, queue state, and the backing namespace.
pub struct NvmeController<M: Mem> {
    base: u64,
    mem: M,

    // ---- Raw register state ----
    cc: u32,
    csts: u32,
    intms: u32,
    aqa: u32,
    asq: u64,
    acq: u64,

    // ---- Admin queues (index 0 of each array) ----
    admin_sq: Queue,
    admin_cq: Queue,

    // ---- I/O queues, indexed by queue id (1-based; slot 0 unused) ----
    io_sqs: Vec<Queue>,
    io_cqs: Vec<Queue>,

    // ---- Namespace 1 backing disk ----
    disk: Vec<u8>,

    // ---- Interrupt aggregation (un-acked completions pending) ----
    interrupt_pending: bool,

    // Negotiated number of I/O queues (Set Features 0x07), 1-based count.
    max_io_queues: u16,
}

impl<M: Mem> NvmeController<M> {
    /// Build a controller at BAR0 `base`, with guest memory `mem` and a
    /// namespace backed by `disk` bytes.
    pub fn new(base: u64, mem: M, disk: Vec<u8>) -> Self {
        let max_io_queues = 4;
        NvmeController {
            base,
            mem,
            cc: 0,
            csts: 0,
            intms: 0,
            aqa: 0,
            asq: 0,
            acq: 0,
            admin_sq: Queue::default(),
            admin_cq: Queue::default(),
            io_sqs: vec![Queue::default(); (max_io_queues as usize) + 1],
            io_cqs: vec![Queue::default(); (max_io_queues as usize) + 1],
            disk,
            interrupt_pending: false,
            max_io_queues,
        }
    }

    /// Build a controller whose namespace has `blocks` logical blocks of
    /// [`LBA_SIZE`] bytes.
    pub fn with_blocks(base: u64, mem: M, blocks: u64) -> Self {
        Self::new(base, mem, vec![0u8; (blocks * LBA_SIZE) as usize])
    }

    /// Capacity of namespace 1 in logical blocks.
    pub fn namespace_blocks(&self) -> u64 {
        self.disk.len() as u64 / LBA_SIZE
    }

    /// Immutable view of the backing disk (for tests / inspection).
    pub fn disk(&self) -> &[u8] {
        &self.disk
    }

    /// Borrow the guest memory backing.
    pub fn mem(&self) -> &M {
        &self.mem
    }

    /// True if a completion interrupt is pending and not yet acked.
    pub fn take_interrupt(&self) -> bool {
        self.interrupt_pending
    }

    /// Clear the pending-interrupt latch (an orchestrator calls this after
    /// injecting the interrupt).
    pub fn clear_interrupt(&mut self) {
        self.interrupt_pending = false;
    }

    /// CAP register value (64-bit). Encodes:
    /// * MQES (bits 15:0): max queue entries supported, 0-based.
    /// * CQR  (bit 16): contiguous queues required = 1.
    /// * TO   (bits 31:24): timeout in 500ms units.
    /// * DSTRD(bits 35:32): doorbell stride = 0 => 4 bytes.
    /// * CSS  (bits 44:37): command sets supported; bit 37 = NVM command set.
    /// * MPSMIN (bits 51:48) / MPSMAX (bits 55:52): 0 => 4 KiB only.
    pub fn cap(&self) -> u64 {
        let mqes: u64 = 0xff; // 256 entries (0-based 255).
        let cqr: u64 = 1 << 16;
        let to: u64 = 1 << 24; // 500ms.
        let dstrd: u64 = 0 << 32; // stride 0 => 4-byte doorbells.
        let css_nvm: u64 = 1 << 37; // NVM command set supported.
        mqes | cqr | to | dstrd | css_nvm
    }

    /// Doorbell stride in bytes: `4 << CAP.DSTRD`. We use DSTRD=0 => 4.
    fn doorbell_stride(&self) -> u64 {
        4
    }

    // ---- CC field accessors ----

    fn cc_en(&self) -> bool {
        self.cc & CC_EN != 0
    }

    // ---- Enable / reset handshake ----

    /// Apply a write to CC, handling the EN edge transitions.
    fn write_cc(&mut self, value: u32) {
        let was_en = self.cc_en();
        self.cc = value;
        let now_en = self.cc_en();

        if !was_en && now_en {
            self.enable();
        } else if was_en && !now_en {
            self.controller_reset();
        }

        // Shutdown notification (CC.SHN != 0) => report shutdown complete.
        let shn = (value >> CC_SHN_SHIFT) & CC_SHN_MASK;
        if shn != 0 {
            self.csts = (self.csts & !(0b11 << CSTS_SHST_SHIFT)) | CSTS_SHST_COMPLETE;
        }
    }

    /// Controller enable: arm the admin queues from AQA/ASQ/ACQ and set RDY.
    fn enable(&mut self) {
        // AQA: ASQS in bits 11:0, ACQS in bits 27:16 (0-based sizes).
        let asqs = (self.aqa & 0xfff) + 1;
        let acqs = ((self.aqa >> 16) & 0xfff) + 1;

        self.admin_sq = Queue {
            base: self.asq,
            size: asqs,
            tail: 0,
            head: 0,
            phase: false,
            cqid: 0,
            active: true,
        };
        self.admin_cq = Queue {
            base: self.acq,
            size: acqs,
            tail: 0,
            head: 0,
            phase: true, // initial phase tag is 1.
            cqid: 0,
            active: true,
        };

        self.csts |= CSTS_RDY;
    }

    /// Controller reset (CC.EN 1->0): tear down all queue state, clear RDY.
    fn controller_reset(&mut self) {
        self.admin_sq.reset();
        self.admin_cq.reset();
        for q in &mut self.io_sqs {
            q.reset();
        }
        for q in &mut self.io_cqs {
            q.reset();
        }
        self.csts &= !CSTS_RDY;
        self.csts &= !CSTS_CFS;
        self.interrupt_pending = false;
    }

    // ---- Register read/write -----------------------------------------------

    /// Read a 32-bit register at BAR-relative `offset`.
    fn read_reg32(&self, offset: u64) -> u32 {
        match offset {
            REG_CAP => (self.cap() & 0xffff_ffff) as u32,
            o if o == REG_CAP + 4 => (self.cap() >> 32) as u32,
            REG_VS => NVME_VERSION_1_4,
            REG_INTMS | REG_INTMC => self.intms,
            REG_CC => self.cc,
            REG_CSTS => self.csts,
            REG_AQA => self.aqa,
            REG_ASQ => (self.asq & 0xffff_ffff) as u32,
            o if o == REG_ASQ + 4 => (self.asq >> 32) as u32,
            REG_ACQ => (self.acq & 0xffff_ffff) as u32,
            o if o == REG_ACQ + 4 => (self.acq >> 32) as u32,
            // Doorbells are write-mostly; reads return the cached value.
            o if o >= REG_DOORBELL_BASE => self.read_doorbell(o),
            _ => 0,
        }
    }

    /// Resolve a doorbell read to the cached producer/consumer index.
    fn read_doorbell(&self, offset: u64) -> u32 {
        let stride = self.doorbell_stride();
        let idx = (offset - REG_DOORBELL_BASE) / stride;
        let qid = (idx / 2) as usize;
        let is_cq = idx % 2 == 1;
        if is_cq {
            // CQyHDBL: host head.
            if qid == 0 {
                self.admin_cq.head
            } else {
                self.io_cqs.get(qid).map_or(0, |q| q.head)
            }
        } else {
            // SQyTDBL: producer tail.
            if qid == 0 {
                self.admin_sq.tail
            } else {
                self.io_sqs.get(qid).map_or(0, |q| q.tail)
            }
        }
    }

    /// Write a 32-bit `value` to the register at BAR-relative `offset`.
    fn write_reg32(&mut self, offset: u64, value: u32) {
        match offset {
            // CAP / VS are read-only.
            REG_CAP | REG_VS => {}
            o if o == REG_CAP + 4 => {}
            REG_INTMS => self.intms |= value,  // write-1-to-set
            REG_INTMC => self.intms &= !value, // write-1-to-clear
            REG_CC => self.write_cc(value),
            REG_CSTS => {} // status is controller-owned (read-only to host).
            REG_AQA => self.aqa = value,
            REG_ASQ => self.asq = (self.asq & !0xffff_ffff) | value as u64,
            o if o == REG_ASQ + 4 => self.asq = (self.asq & 0xffff_ffff) | ((value as u64) << 32),
            REG_ACQ => self.acq = (self.acq & !0xffff_ffff) | value as u64,
            o if o == REG_ACQ + 4 => self.acq = (self.acq & 0xffff_ffff) | ((value as u64) << 32),
            o if o >= REG_DOORBELL_BASE => self.write_doorbell(o, value),
            _ => {}
        }
    }

    /// Handle a doorbell write: update the producer/consumer index and, for an
    /// SQ tail write, process the newly-submitted commands.
    fn write_doorbell(&mut self, offset: u64, value: u32) {
        let stride = self.doorbell_stride();
        let idx = (offset - REG_DOORBELL_BASE) / stride;
        let qid = (idx / 2) as usize;
        let is_cq = idx % 2 == 1;

        if is_cq {
            // CQyHDBL: advance the host head pointer.
            if qid == 0 {
                self.admin_cq.head = value;
            } else if let Some(q) = self.io_cqs.get_mut(qid) {
                q.head = value;
            }
            return;
        }

        // SQyTDBL: record the new tail and run the submission queue.
        if qid == 0 {
            self.admin_sq.tail = value;
            self.process_admin_sq();
        } else if qid < self.io_sqs.len() {
            self.io_sqs[qid].tail = value;
            self.process_io_sq(qid as u16);
        }
    }

    // ---- Command processing ------------------------------------------------

    /// Read one submission entry at SQ slot `slot` of the queue based at `base`.
    fn read_sqe(&self, base: u64, slot: u32) -> Option<SubmissionEntry> {
        let mut buf = [0u8; SQE_SIZE as usize];
        let gpa = base + slot as u64 * SQE_SIZE;
        if !self.mem.read(gpa, &mut buf) {
            return None;
        }
        SubmissionEntry::parse(&buf)
    }

    /// Drain the admin submission queue up to its current tail, executing each
    /// command and posting a completion to the admin CQ.
    fn process_admin_sq(&mut self) {
        if !self.admin_sq.active || self.admin_sq.size == 0 {
            return;
        }
        while self.admin_sq.head != self.admin_sq.tail {
            let slot = self.admin_sq.head;
            let entry = match self.read_sqe(self.admin_sq.base, slot) {
                Some(e) => e,
                None => break,
            };
            let (status, dw0) = self.exec_admin(&entry);
            self.admin_sq.head = (self.admin_sq.head + 1) % self.admin_sq.size;
            self.post_completion(true, 0, &entry, status, dw0);
        }
    }

    /// Drain an I/O submission queue, posting completions to its bound CQ.
    fn process_io_sq(&mut self, sqid: u16) {
        let sq = match self.io_sqs.get(sqid as usize).copied() {
            Some(q) if q.active && q.size != 0 => q,
            _ => return,
        };
        let cqid = sq.cqid;
        let mut head = sq.head;
        while head != self.io_sqs[sqid as usize].tail {
            let entry = match self.read_sqe(sq.base, head) {
                Some(e) => e,
                None => break,
            };
            let (status, dw0) = self.exec_nvm(&entry);
            head = (head + 1) % sq.size;
            self.post_completion(false, cqid, &entry, status, dw0);
        }
        self.io_sqs[sqid as usize].head = head;
    }

    /// Post a 16-byte completion entry to the target CQ and advance its tail /
    /// flip the phase bit on wrap.
    ///
    /// CQE layout (little-endian):
    ///   DW0  command specific result
    ///   DW1  reserved
    ///   DW2  SQ head pointer (bits 15:0) | SQ identifier (bits 31:16)
    ///   DW3  command id (bits 15:0) | phase tag (bit 16) | status (bits 31:17)
    fn post_completion(
        &mut self,
        admin: bool,
        cqid: u16,
        entry: &SubmissionEntry,
        status: u16,
        dw0: u32,
    ) {
        // Resolve the target CQ and the SQ head/id to report.
        let (cq, sq_head, sqid) = if admin {
            (&mut self.admin_cq, self.admin_sq.head, 0u16)
        } else {
            let sqid = self.sqid_for_cqid(cqid);
            let sq_head = self.io_sqs.get(sqid as usize).map_or(0, |q| q.head);
            let cq = match self.io_cqs.get_mut(cqid as usize) {
                Some(q) if q.active => q,
                _ => return,
            };
            (cq, sq_head, sqid)
        };

        if cq.size == 0 || !cq.active {
            return;
        }

        let slot = cq.tail;
        let phase = cq.phase;
        let gpa = cq.base + slot as u64 * CQE_SIZE;

        // Advance the CQ tail; flip phase on wrap.
        let next = (cq.tail + 1) % cq.size;
        if next == 0 {
            cq.phase = !cq.phase;
        }
        cq.tail = next;

        let mut cqe = [0u8; CQE_SIZE as usize];
        cqe[0..4].copy_from_slice(&dw0.to_le_bytes());
        // DW1 reserved (0).
        let dw2 = (sq_head & 0xffff) as u32 | ((sqid as u32) << 16);
        cqe[8..12].copy_from_slice(&dw2.to_le_bytes());
        let phase_bit = if phase { 1u32 << 16 } else { 0 };
        // Status field occupies bits 31:17 (SCT|SC, with the DNR/More flags in
        // the high bits). We place the SC (and any SCT) into bits 31:17 by
        // shifting our combined status left by 17.
        let dw3 = (entry.cid as u32) | phase_bit | ((status as u32) << 17);
        cqe[12..16].copy_from_slice(&dw3.to_le_bytes());

        let _ = self.mem.write(gpa, &cqe);
        self.interrupt_pending = true;
    }

    /// Find the I/O SQ bound to completion queue `cqid` (first match). Falls
    /// back to `cqid` itself which is the common 1:1 mapping.
    fn sqid_for_cqid(&self, cqid: u16) -> u16 {
        for (i, q) in self.io_sqs.iter().enumerate() {
            if q.active && q.cqid == cqid {
                return i as u16;
            }
        }
        cqid
    }

    // ---- Admin command execution -------------------------------------------

    /// Execute an admin command, returning (status code, completion DW0).
    fn exec_admin(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        match e.opcode {
            ADMIN_IDENTIFY => self.admin_identify(e),
            ADMIN_CREATE_IO_CQ => self.admin_create_io_cq(e),
            ADMIN_CREATE_IO_SQ => self.admin_create_io_sq(e),
            ADMIN_DELETE_IO_CQ => self.admin_delete_io_cq(e),
            ADMIN_DELETE_IO_SQ => self.admin_delete_io_sq(e),
            ADMIN_SET_FEATURES => self.admin_set_features(e),
            ADMIN_GET_FEATURES => self.admin_get_features(e),
            _ => (SC_INVALID_OPCODE, 0),
        }
    }

    /// Identify: write a 4096-byte data structure selected by CDW10.CNS to PRP1.
    fn admin_identify(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        let cns = e.cdw10 & 0xff;
        let mut data = vec![0u8; PAGE_SIZE as usize];
        match cns {
            CNS_CONTROLLER => self.fill_identify_controller(&mut data),
            CNS_NAMESPACE => {
                if e.nsid != 1 {
                    // Identify of an inactive/invalid namespace returns zeros.
                } else {
                    self.fill_identify_namespace(&mut data);
                }
            }
            CNS_ACTIVE_NS_LIST => {
                // List of active NSIDs > the one in CDW1 (NSID). We have only
                // NSID 1; report it if the requested base is below 1.
                if e.nsid < 1 {
                    data[0..4].copy_from_slice(&1u32.to_le_bytes());
                }
            }
            _ => return (SC_INVALID_FIELD, 0),
        }
        if !self.mem.write(e.prp1, &data) {
            return (SC_DATA_XFER_ERROR, 0);
        }
        (SC_SUCCESS, 0)
    }

    /// Populate an Identify Controller (CNS 01h) data structure with sane,
    /// minimal fields.
    fn fill_identify_controller(&self, d: &mut [u8]) {
        // VID (0x00) / SSVID (0x02): use a recognizable vendor id.
        d[0x00..0x02].copy_from_slice(&0x1b36u16.to_le_bytes()); // Red Hat / QEMU
        d[0x02..0x04].copy_from_slice(&0x1af4u16.to_le_bytes());
        // SN (0x04, 20 bytes ASCII, space padded).
        write_ascii(&mut d[0x04..0x18], b"RAX-NVME-0001");
        // MN (0x18, 40 bytes ASCII).
        write_ascii(&mut d[0x18..0x40], b"rax NVMe Controller");
        // FR (0x40, 8 bytes ASCII firmware revision).
        write_ascii(&mut d[0x40..0x48], b"1.0");
        // RAB (0x48): recommended arbitration burst.
        d[0x48] = 0;
        // VER (0x80): the version this controller is compliant with.
        d[0x80..0x84].copy_from_slice(&NVME_VERSION_1_4.to_le_bytes());
        // SQES (0x200): bits 3:0 min = 6 (64 bytes), bits 7:4 max = 6.
        d[0x200] = 0x66;
        // CQES (0x201): min = 4 (16 bytes), max = 4.
        d[0x201] = 0x44;
        // NN (0x204, 4 bytes): number of namespaces = 1.
        d[0x204..0x208].copy_from_slice(&1u32.to_le_bytes());
    }

    /// Populate an Identify Namespace (CNS 00h) data structure for NSID 1.
    fn fill_identify_namespace(&self, d: &mut [u8]) {
        let nblocks = self.namespace_blocks();
        // NSZE (0x00, 8 bytes): namespace size in logical blocks.
        d[0x00..0x08].copy_from_slice(&nblocks.to_le_bytes());
        // NCAP (0x08): namespace capacity.
        d[0x08..0x10].copy_from_slice(&nblocks.to_le_bytes());
        // NUSE (0x10): namespace utilization.
        d[0x10..0x18].copy_from_slice(&nblocks.to_le_bytes());
        // NLBAF (0x19): number of LBA formats supported, 0-based => 0 (one).
        d[0x19] = 0;
        // FLBAS (0x1a): formatted LBA size => format 0.
        d[0x1a] = 0;
        // LBAF0 (0x80, 4 bytes): MS=0, LBADS in bits 23:16 = 9 (512 bytes),
        // RP=0 in bits 25:24.
        let lbads: u32 = 9 << 16;
        d[0x80..0x84].copy_from_slice(&lbads.to_le_bytes());
    }

    /// Create I/O Completion Queue (opcode 05h).
    ///
    /// CDW10: QID (bits 15:0), QSIZE (bits 31:16, 0-based).
    /// CDW11: PC (bit 0, physically contiguous), IEN (bit 1), IV (bits 31:16).
    /// PRP1 : the CQ base address (PC must be 1).
    fn admin_create_io_cq(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        let qid = (e.cdw10 & 0xffff) as usize;
        let qsize = ((e.cdw10 >> 16) & 0xffff) + 1;
        let pc = e.cdw11 & 1 != 0;
        if qid == 0 || qid >= self.io_cqs.len() || !pc || e.prp1 == 0 {
            return (SC_INVALID_FIELD, 0);
        }
        self.io_cqs[qid] = Queue {
            base: e.prp1,
            size: qsize,
            tail: 0,
            head: 0,
            phase: true,
            cqid: qid as u16,
            active: true,
        };
        (SC_SUCCESS, 0)
    }

    /// Create I/O Submission Queue (opcode 01h).
    ///
    /// CDW10: QID (bits 15:0), QSIZE (bits 31:16, 0-based).
    /// CDW11: PC (bit 0), CQID (bits 31:16).
    /// PRP1 : the SQ base address.
    fn admin_create_io_sq(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        let qid = (e.cdw10 & 0xffff) as usize;
        let qsize = ((e.cdw10 >> 16) & 0xffff) + 1;
        let pc = e.cdw11 & 1 != 0;
        let cqid = (e.cdw11 >> 16) as u16;
        if qid == 0
            || qid >= self.io_sqs.len()
            || !pc
            || e.prp1 == 0
            || cqid as usize >= self.io_cqs.len()
            || !self.io_cqs[cqid as usize].active
        {
            return (SC_INVALID_FIELD, 0);
        }
        self.io_sqs[qid] = Queue {
            base: e.prp1,
            size: qsize,
            tail: 0,
            head: 0,
            phase: false,
            cqid,
            active: true,
        };
        (SC_SUCCESS, 0)
    }

    /// Delete I/O Completion Queue (opcode 04h).
    fn admin_delete_io_cq(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        let qid = (e.cdw10 & 0xffff) as usize;
        if qid == 0 || qid >= self.io_cqs.len() || !self.io_cqs[qid].active {
            return (SC_INVALID_FIELD, 0);
        }
        // The spec requires all associated SQs be deleted first.
        if self
            .io_sqs
            .iter()
            .any(|q| q.active && q.cqid as usize == qid)
        {
            return (SC_INVALID_FIELD, 0);
        }
        self.io_cqs[qid].reset();
        (SC_SUCCESS, 0)
    }

    /// Delete I/O Submission Queue (opcode 00h).
    fn admin_delete_io_sq(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        let qid = (e.cdw10 & 0xffff) as usize;
        if qid == 0 || qid >= self.io_sqs.len() || !self.io_sqs[qid].active {
            return (SC_INVALID_FIELD, 0);
        }
        self.io_sqs[qid].reset();
        (SC_SUCCESS, 0)
    }

    /// Set Features (opcode 09h). Only Number of Queues (07h) is meaningful;
    /// others are accepted as no-ops. Returns the negotiated value in DW0.
    fn admin_set_features(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        let fid = (e.cdw10 & 0xff) as u8;
        if fid == FID_NUM_QUEUES {
            // CDW11: NSQR (bits 15:0), NCQR (bits 31:16), both 0-based requested
            // counts. We grant min(requested, max-1) and report 0-based counts.
            let granted = (self.max_io_queues - 1) as u32; // 0-based.
            let dw0 = granted | (granted << 16);
            return (SC_SUCCESS, dw0);
        }
        (SC_SUCCESS, 0)
    }

    /// Get Features (opcode 0Ah). Mirrors Set Features for Number of Queues.
    fn admin_get_features(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        let fid = (e.cdw10 & 0xff) as u8;
        if fid == FID_NUM_QUEUES {
            let granted = (self.max_io_queues - 1) as u32;
            let dw0 = granted | (granted << 16);
            return (SC_SUCCESS, dw0);
        }
        (SC_SUCCESS, 0)
    }

    // ---- NVM I/O command execution -----------------------------------------

    /// Execute an NVM command (Read/Write/Flush) against namespace 1.
    fn exec_nvm(&mut self, e: &SubmissionEntry) -> (u16, u32) {
        match e.opcode {
            NVM_FLUSH => (SC_SUCCESS, 0),
            NVM_READ => self.nvm_rw(e, true),
            NVM_WRITE => self.nvm_rw(e, false),
            _ => (SC_INVALID_OPCODE, 0),
        }
    }

    /// Read/Write data between guest memory (PRP1) and the namespace.
    ///
    /// CDW10/11: starting LBA (64-bit). CDW12: NLB (bits 15:0, 0-based count).
    ///
    /// Only PRP1 is used: the transfer must fit in a single page-contiguous
    /// region addressed by PRP1 (and, when the transfer crosses the first page
    /// boundary, PRP2 pointing at the next contiguous page). Larger transfers
    /// requiring a PRP list are rejected (deferred).
    fn nvm_rw(&mut self, e: &SubmissionEntry, read: bool) -> (u16, u32) {
        if e.nsid != 1 {
            return (SC_INVALID_FIELD, 0);
        }
        let slba = (e.cdw10 as u64) | ((e.cdw11 as u64) << 32);
        let nlb = (e.cdw12 & 0xffff) as u64 + 1; // 0-based count.
        let total = nlb * LBA_SIZE;

        let start = match slba.checked_mul(LBA_SIZE) {
            Some(s) => s,
            None => return (SC_LBA_OUT_OF_RANGE, 0),
        };
        let end = match start.checked_add(total) {
            Some(s) => s,
            None => return (SC_LBA_OUT_OF_RANGE, 0),
        };
        if end > self.disk.len() as u64 {
            return (SC_LBA_OUT_OF_RANGE, 0);
        }

        // Resolve the guest buffers for this transfer using PRP1 (+PRP2 for the
        // second page). Anything larger is deferred.
        let segments = match self.prp_segments(e.prp1, e.prp2, total) {
            Some(s) => s,
            None => return (SC_DATA_XFER_ERROR, 0),
        };

        let mut disk_off = start as usize;
        for (gpa, len) in segments {
            let len = len as usize;
            let dend = disk_off + len;
            if read {
                if !self.mem.write(gpa, &self.disk[disk_off..dend]) {
                    return (SC_DATA_XFER_ERROR, 0);
                }
            } else if !self.mem.read(gpa, &mut self.disk[disk_off..dend]) {
                return (SC_DATA_XFER_ERROR, 0);
            }
            disk_off = dend;
        }
        (SC_SUCCESS, 0)
    }

    /// Compute the list of (gpa, len) guest segments for a transfer of `total`
    /// bytes using PRP1 (and PRP2 for the part past the first page boundary).
    ///
    /// Returns `None` (=> fail the command) if the transfer would require a PRP
    /// list (more than two pages), which is deferred.
    fn prp_segments(&self, prp1: u64, prp2: u64, total: u64) -> Option<Vec<(u64, u64)>> {
        if prp1 == 0 || total == 0 {
            return None;
        }
        // Bytes available in PRP1's page before the next page boundary.
        let first_page_room = PAGE_SIZE - (prp1 % PAGE_SIZE);
        if total <= first_page_room {
            return Some(vec![(prp1, total)]);
        }
        // Spills into a second buffer described by PRP2.
        let remaining = total - first_page_room;
        if remaining > PAGE_SIZE {
            // Would need a PRP list (3+ pages): deferred.
            return None;
        }
        if prp2 == 0 {
            return None;
        }
        Some(vec![(prp1, first_page_room), (prp2, remaining)])
    }

    // ---- Test / introspection accessors ------------------------------------

    /// Admin submission/completion queue state (for tests).
    pub fn admin_sq(&self) -> &Queue {
        &self.admin_sq
    }
    pub fn admin_cq(&self) -> &Queue {
        &self.admin_cq
    }
    /// I/O submission queue `qid` state (for tests).
    pub fn io_sq(&self, qid: usize) -> Option<&Queue> {
        self.io_sqs.get(qid)
    }
    /// I/O completion queue `qid` state (for tests).
    pub fn io_cq(&self, qid: usize) -> Option<&Queue> {
        self.io_cqs.get(qid)
    }
    /// Current CSTS value (for tests).
    pub fn csts(&self) -> u32 {
        self.csts
    }
}

/// Write an ASCII string into a fixed field, space-padded (NVMe string fields
/// are space-padded, not NUL-terminated).
fn write_ascii(field: &mut [u8], s: &[u8]) {
    for b in field.iter_mut() {
        *b = b' ';
    }
    let n = s.len().min(field.len());
    field[..n].copy_from_slice(&s[..n]);
}

impl<M: Mem + Send> MmioDevice for NvmeController<M> {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        let offset = addr.wrapping_sub(self.base);
        // All registers are 32-bit little-endian; service partial / byte-wise
        // accesses by extracting the requested bytes of the word.
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

    fn dev(blocks: u64) -> NvmeController<VecMem> {
        // Generous guest memory for queues + data buffers.
        NvmeController::with_blocks(BASE, VecMem::new(0x10_0000), blocks)
    }

    fn read32(d: &mut NvmeController<VecMem>, off: u64) -> u32 {
        let mut buf = [0u8; 4];
        d.read(BASE + off, &mut buf);
        u32::from_le_bytes(buf)
    }

    fn read64(d: &mut NvmeController<VecMem>, off: u64) -> u64 {
        (read32(d, off) as u64) | ((read32(d, off + 4) as u64) << 32)
    }

    fn write32(d: &mut NvmeController<VecMem>, off: u64, value: u32) {
        d.write(BASE + off, &value.to_le_bytes());
    }

    fn write64(d: &mut NvmeController<VecMem>, off: u64, value: u64) {
        write32(d, off, value as u32);
        write32(d, off + 4, (value >> 32) as u32);
    }

    // ---- Register basics ----

    #[test]
    fn cap_reports_capabilities() {
        let mut d = dev(64);
        let cap = read64(&mut d, REG_CAP);
        // MQES (0-based) = 255 => 256 entries.
        assert_eq!(cap & 0xffff, 0xff);
        // CQR bit set.
        assert_ne!(cap & (1 << 16), 0);
        // DSTRD = 0.
        assert_eq!((cap >> 32) & 0xf, 0);
        // NVM command set supported (bit 37).
        assert_ne!(cap & (1 << 37), 0);
    }

    #[test]
    fn version_is_1_4() {
        let mut d = dev(64);
        assert_eq!(read32(&mut d, REG_VS), 0x0001_0400);
        assert_eq!(read32(&mut d, REG_VS), NVME_VERSION_1_4);
    }

    #[test]
    fn cap_vs_are_read_only() {
        let mut d = dev(64);
        let cap_lo = read32(&mut d, REG_CAP);
        write32(&mut d, REG_CAP, 0xdead_beef);
        assert_eq!(read32(&mut d, REG_CAP), cap_lo);
        write32(&mut d, REG_VS, 0x1234);
        assert_eq!(read32(&mut d, REG_VS), NVME_VERSION_1_4);
    }

    #[test]
    fn intms_set_and_clear() {
        let mut d = dev(64);
        // INTMS is write-1-to-set.
        write32(&mut d, REG_INTMS, 0b1010);
        assert_eq!(read32(&mut d, REG_INTMS), 0b1010);
        write32(&mut d, REG_INTMS, 0b0100);
        assert_eq!(read32(&mut d, REG_INTMS), 0b1110);
        // INTMC is write-1-to-clear.
        write32(&mut d, REG_INTMC, 0b0010);
        assert_eq!(read32(&mut d, REG_INTMS), 0b1100);
    }

    // ---- EN -> RDY handshake ----

    #[test]
    fn enable_sets_ready_disable_clears() {
        let mut d = dev(64);
        // Initially not ready.
        assert_eq!(read32(&mut d, REG_CSTS) & CSTS_RDY, 0);

        // Program admin queues then enable.
        write32(&mut d, REG_AQA, ((31u32) << 16) | 31); // 32 entries each.
        write64(&mut d, REG_ASQ, 0x1000);
        write64(&mut d, REG_ACQ, 0x2000);
        write32(&mut d, REG_CC, CC_EN);

        assert_ne!(read32(&mut d, REG_CSTS) & CSTS_RDY, 0, "RDY follows EN");
        assert!(d.admin_sq().active);
        assert_eq!(d.admin_sq().base, 0x1000);
        assert_eq!(d.admin_sq().size, 32);
        assert_eq!(d.admin_cq().base, 0x2000);
        assert_eq!(d.admin_cq().size, 32);

        // Controller reset: EN 1 -> 0 clears RDY and tears down queues.
        write32(&mut d, REG_CC, 0);
        assert_eq!(read32(&mut d, REG_CSTS) & CSTS_RDY, 0);
        assert!(!d.admin_sq().active);
    }

    #[test]
    fn aqa_asq_acq_programming() {
        let mut d = dev(64);
        write32(&mut d, REG_AQA, 0x003f_001f); // ACQS=0x3f(64), ASQS=0x1f(32)
        assert_eq!(read32(&mut d, REG_AQA), 0x003f_001f);
        write64(&mut d, REG_ASQ, 0x1234_5678_9abc_d000);
        write64(&mut d, REG_ACQ, 0x0000_0000_dead_b000);
        assert_eq!(read64(&mut d, REG_ASQ), 0x1234_5678_9abc_d000);
        assert_eq!(read64(&mut d, REG_ACQ), 0x0000_0000_dead_b000);

        write32(&mut d, REG_CC, CC_EN);
        // ASQS 0x1f => 32 entries, ACQS 0x3f => 64 entries.
        assert_eq!(d.admin_sq().size, 32);
        assert_eq!(d.admin_cq().size, 64);
    }

    // ---- Doorbells ----

    #[test]
    fn doorbell_write_tracking() {
        let mut d = dev(64);
        // Bring the controller up with tiny admin queues so the SQ doorbell
        // doesn't trigger command processing (head==tail boundary handled).
        write32(&mut d, REG_AQA, (3u32 << 16) | 3);
        write64(&mut d, REG_ASQ, 0x1000);
        write64(&mut d, REG_ACQ, 0x2000);
        write32(&mut d, REG_CC, CC_EN);

        // CQ0 head doorbell at 0x1000 + 1*4 = 0x1004.
        write32(&mut d, REG_DOORBELL_BASE + 4, 2);
        assert_eq!(d.admin_cq().head, 2);
        assert_eq!(read32(&mut d, REG_DOORBELL_BASE + 4), 2);

        // SQ0 tail doorbell at 0x1000 (no commands queued => no processing).
        write32(&mut d, REG_DOORBELL_BASE, 0);
        assert_eq!(d.admin_sq().tail, 0);
    }

    // ---- Namespace capacity ----

    #[test]
    fn namespace_capacity() {
        let d = dev(2048);
        assert_eq!(d.namespace_blocks(), 2048);
        assert_eq!(d.disk().len(), 2048 * LBA_SIZE as usize);
    }

    // ---- Admin Identify flow ----

    /// Bring the controller up with admin queues, write an SQE into the admin
    /// SQ, ring the doorbell, and return the controller for inspection.
    fn bring_up(d: &mut NvmeController<VecMem>, asq: u64, acq: u64, entries: u32) {
        write32(d, REG_AQA, ((entries - 1) << 16) | (entries - 1));
        write64(d, REG_ASQ, asq);
        write64(d, REG_ACQ, acq);
        write32(d, REG_CC, CC_EN);
    }

    /// Build a 64-byte SQE in `mem` at `gpa`.
    #[allow(clippy::too_many_arguments)]
    fn make_sqe(
        opcode: u8,
        cid: u16,
        nsid: u32,
        prp1: u64,
        prp2: u64,
        cdw10: u32,
        cdw11: u32,
        cdw12: u32,
    ) -> [u8; 64] {
        let mut sqe = [0u8; 64];
        let cdw0 = (opcode as u32) | ((cid as u32) << 16);
        sqe[0..4].copy_from_slice(&cdw0.to_le_bytes());
        sqe[4..8].copy_from_slice(&nsid.to_le_bytes());
        sqe[24..32].copy_from_slice(&prp1.to_le_bytes());
        sqe[32..40].copy_from_slice(&prp2.to_le_bytes());
        sqe[40..44].copy_from_slice(&cdw10.to_le_bytes());
        sqe[44..48].copy_from_slice(&cdw11.to_le_bytes());
        sqe[48..52].copy_from_slice(&cdw12.to_le_bytes());
        sqe
    }

    /// Decode a CQE's status code (bits 31:17 of DW3 >> 17) and phase bit.
    fn read_cqe_status(d: &NvmeController<VecMem>, acq: u64, slot: u64) -> (u16, bool, u16) {
        let mut dw3 = [0u8; 4];
        d.mem().read(acq + slot * CQE_SIZE + 12, &mut dw3);
        let dw3 = u32::from_le_bytes(dw3);
        let cid = (dw3 & 0xffff) as u16;
        let phase = (dw3 >> 16) & 1 != 0;
        let status = ((dw3 >> 17) & 0x7fff) as u16;
        (status, phase, cid)
    }

    #[test]
    fn admin_identify_controller_flow() {
        let mut d = dev(0x1000); // 0x1000 blocks.
        let asq = 0x1000u64;
        let acq = 0x2000u64;
        let ident_buf = 0x4000u64;

        bring_up(&mut d, asq, acq, 16);

        // Identify Controller (CNS=1) into ident_buf.
        let sqe = make_sqe(ADMIN_IDENTIFY, 0x55, 0, ident_buf, 0, CNS_CONTROLLER, 0, 0);
        d.mem_mut_write(asq, &sqe);

        // Ring the admin SQ tail doorbell to slot 1.
        write32(&mut d, REG_DOORBELL_BASE, 1);

        // Admin SQ consumed the entry.
        assert_eq!(d.admin_sq().head, 1);
        // Admin CQ advanced its tail.
        assert_eq!(d.admin_cq().tail, 1);
        // Completion: success, phase 1 (initial), cid echoed.
        let (status, phase, cid) = read_cqe_status(&d, acq, 0);
        assert_eq!(status, SC_SUCCESS);
        assert!(phase);
        assert_eq!(cid, 0x55);
        // Interrupt pending.
        assert!(d.take_interrupt());

        // Identify Controller data: VER field at 0x80 == 1.4, NN at 0x204 == 1,
        // SQES at 0x200 == 0x66.
        let mut ver = [0u8; 4];
        d.mem().read(ident_buf + 0x80, &mut ver);
        assert_eq!(u32::from_le_bytes(ver), NVME_VERSION_1_4);
        let mut nn = [0u8; 4];
        d.mem().read(ident_buf + 0x204, &mut nn);
        assert_eq!(u32::from_le_bytes(nn), 1);
        let mut sqes = [0u8; 1];
        d.mem().read(ident_buf + 0x200, &mut sqes);
        assert_eq!(sqes[0], 0x66);
    }

    #[test]
    fn admin_identify_namespace_reports_size() {
        let mut d = dev(0x800);
        let asq = 0x1000u64;
        let acq = 0x2000u64;
        let buf = 0x4000u64;
        bring_up(&mut d, asq, acq, 16);

        let sqe = make_sqe(ADMIN_IDENTIFY, 1, 1, buf, 0, CNS_NAMESPACE, 0, 0);
        d.mem_mut_write(asq, &sqe);
        write32(&mut d, REG_DOORBELL_BASE, 1);

        let (status, _, _) = read_cqe_status(&d, acq, 0);
        assert_eq!(status, SC_SUCCESS);
        // NSZE at offset 0 == 0x800 blocks.
        let mut nsze = [0u8; 8];
        d.mem().read(buf, &mut nsze);
        assert_eq!(u64::from_le_bytes(nsze), 0x800);
        // LBAF0 at 0x80: LBADS == 9.
        let mut lbaf = [0u8; 4];
        d.mem().read(buf + 0x80, &mut lbaf);
        assert_eq!((u32::from_le_bytes(lbaf) >> 16) & 0xff, 9);
    }

    #[test]
    fn invalid_admin_opcode_completes_with_error() {
        let mut d = dev(64);
        let asq = 0x1000u64;
        let acq = 0x2000u64;
        bring_up(&mut d, asq, acq, 16);

        let sqe = make_sqe(0xfe, 7, 0, 0x4000, 0, 0, 0, 0);
        d.mem_mut_write(asq, &sqe);
        write32(&mut d, REG_DOORBELL_BASE, 1);

        let (status, _, cid) = read_cqe_status(&d, acq, 0);
        assert_eq!(status, SC_INVALID_OPCODE);
        assert_eq!(cid, 7);
    }

    #[test]
    fn set_features_num_queues_returns_count() {
        let mut d = dev(64);
        let asq = 0x1000u64;
        let acq = 0x2000u64;
        bring_up(&mut d, asq, acq, 16);

        // Request a pile of queues; controller grants its maximum.
        let sqe = make_sqe(
            ADMIN_SET_FEATURES,
            3,
            0,
            0,
            0,
            FID_NUM_QUEUES as u32,
            0xffff_ffff,
            0,
        );
        d.mem_mut_write(asq, &sqe);
        write32(&mut d, REG_DOORBELL_BASE, 1);

        let (status, _, _) = read_cqe_status(&d, acq, 0);
        assert_eq!(status, SC_SUCCESS);
        // DW0 of the completion holds the granted (0-based) counts.
        let mut dw0 = [0u8; 4];
        d.mem().read(acq, &mut dw0);
        let dw0 = u32::from_le_bytes(dw0);
        let nsqr = dw0 & 0xffff;
        let ncqr = dw0 >> 16;
        assert_eq!(nsqr, ncqr);
        // Granted = max_io_queues - 1 == 3.
        assert_eq!(nsqr, 3);
    }

    // ---- I/O queue creation + NVM read/write round trip ----

    #[test]
    fn create_io_queues_and_round_trip_data() {
        let mut d = dev(64);
        // Memory map:
        //   admin SQ @ 0x1000, admin CQ @ 0x2000
        //   I/O SQ   @ 0x3000, I/O CQ   @ 0x4000
        //   data buf @ 0x8000
        let asq = 0x1000u64;
        let acq = 0x2000u64;
        let io_sq = 0x3000u64;
        let io_cq = 0x4000u64;
        let data = 0x8000u64;
        bring_up(&mut d, asq, acq, 16);

        // 1. Create I/O CQ qid=1, size=8 (0-based 7), PC=1, base=io_cq.
        let cdw10 = 1 | (7u32 << 16);
        let sqe = make_sqe(
            ADMIN_CREATE_IO_CQ,
            0x10,
            0,
            io_cq,
            0,
            cdw10,
            1, /*PC*/
            0,
        );
        d.mem_mut_write(asq + 0 * SQE_SIZE, &sqe);
        // 2. Create I/O SQ qid=1, size=8, PC=1, cqid=1, base=io_sq.
        let cdw10 = 1 | (7u32 << 16);
        let cdw11 = 1 | (1u32 << 16); // PC=1, CQID=1.
        let sqe = make_sqe(ADMIN_CREATE_IO_SQ, 0x11, 0, io_sq, 0, cdw10, cdw11, 0);
        d.mem_mut_write(asq + 1 * SQE_SIZE, &sqe);

        // Ring admin SQ doorbell to slot 2 (both admin commands).
        write32(&mut d, REG_DOORBELL_BASE, 2);

        // Both admin completions succeeded.
        let (s0, _, c0) = read_cqe_status(&d, acq, 0);
        let (s1, _, c1) = read_cqe_status(&d, acq, 1);
        assert_eq!(s0, SC_SUCCESS);
        assert_eq!(s1, SC_SUCCESS);
        assert_eq!(c0, 0x10);
        assert_eq!(c1, 0x11);
        assert!(d.io_cq(1).unwrap().active);
        assert!(d.io_sq(1).unwrap().active);
        assert_eq!(d.io_sq(1).unwrap().cqid, 1);

        // Stage a recognizable pattern and WRITE it to LBA 2 via the I/O SQ.
        let pattern: Vec<u8> = (0..LBA_SIZE as usize).map(|i| (i & 0xff) as u8).collect();
        d.mem_mut_write(data, &pattern);

        // NVM Write: nsid=1, prp1=data, slba=2 (cdw10 low), nlb=0 (one block).
        let write_sqe = make_sqe(
            NVM_WRITE, 0x20, 1, data, 0, 2, /*slba lo*/
            0, /*slba hi*/
            0, /*nlb=0 => 1 block*/
        );
        d.mem_mut_write(io_sq + 0 * SQE_SIZE, &write_sqe);

        // I/O SQ1 tail doorbell: index = 2*1 + 0 = 2 => offset 0x1000 + 2*4.
        let io_sq1_tail_db = REG_DOORBELL_BASE + 2 * 4;
        write32(&mut d, io_sq1_tail_db, 1);

        // The disk now holds the pattern at LBA 2.
        let off = 2 * LBA_SIZE as usize;
        assert_eq!(&d.disk()[off..off + LBA_SIZE as usize], &pattern[..]);

        // A completion should have landed in the I/O CQ (slot 0).
        let (sw, _, cw) = read_cqe_status(&d, io_cq, 0);
        assert_eq!(sw, SC_SUCCESS);
        assert_eq!(cw, 0x20);

        // Now READ it back into a different buffer and verify.
        let rbuf = 0x9000u64;
        let read_sqe = make_sqe(NVM_READ, 0x21, 1, rbuf, 0, 2, 0, 0);
        d.mem_mut_write(io_sq + 1 * SQE_SIZE, &read_sqe);
        write32(&mut d, io_sq1_tail_db, 2);

        let mut readback = vec![0u8; LBA_SIZE as usize];
        d.mem().read(rbuf, &mut readback);
        assert_eq!(readback, pattern);

        // Second I/O completion at CQ slot 1.
        let (sr, _, cr) = read_cqe_status(&d, io_cq, 1);
        assert_eq!(sr, SC_SUCCESS);
        assert_eq!(cr, 0x21);
    }

    #[test]
    fn nvm_read_out_of_range_fails() {
        let mut d = dev(4); // only 4 blocks.
        let asq = 0x1000u64;
        let acq = 0x2000u64;
        let io_sq = 0x3000u64;
        let io_cq = 0x4000u64;
        bring_up(&mut d, asq, acq, 16);

        // Create CQ1 + SQ1.
        let sqe = make_sqe(ADMIN_CREATE_IO_CQ, 1, 0, io_cq, 0, 1 | (7u32 << 16), 1, 0);
        d.mem_mut_write(asq, &sqe);
        let sqe = make_sqe(
            ADMIN_CREATE_IO_SQ,
            2,
            0,
            io_sq,
            0,
            1 | (7u32 << 16),
            1 | (1u32 << 16),
            0,
        );
        d.mem_mut_write(asq + SQE_SIZE, &sqe);
        write32(&mut d, REG_DOORBELL_BASE, 2);

        // Read LBA 100 (out of range for a 4-block namespace).
        let read_sqe = make_sqe(NVM_READ, 0x30, 1, 0x8000, 0, 100, 0, 0);
        d.mem_mut_write(io_sq, &read_sqe);
        write32(&mut d, REG_DOORBELL_BASE + 2 * 4, 1);

        let (status, _, _) = read_cqe_status(&d, io_cq, 0);
        assert_eq!(status, SC_LBA_OUT_OF_RANGE);
    }

    #[test]
    fn submission_entry_parse_extracts_fields() {
        let sqe = make_sqe(NVM_WRITE, 0xabcd, 1, 0xdead_beef_0000, 0x1234, 7, 8, 9);
        let e = SubmissionEntry::parse(&sqe).unwrap();
        assert_eq!(e.opcode, NVM_WRITE);
        assert_eq!(e.cid, 0xabcd);
        assert_eq!(e.nsid, 1);
        assert_eq!(e.prp1, 0xdead_beef_0000);
        assert_eq!(e.prp2, 0x1234);
        assert_eq!(e.cdw10, 7);
        assert_eq!(e.cdw11, 8);
        assert_eq!(e.cdw12, 9);
        // Too-short buffer rejected.
        assert!(SubmissionEntry::parse(&sqe[..32]).is_none());
    }
}

// Test-only helper: write directly into the controller's guest memory. Defined
// outside the `tests` module so the `cfg(test)` impl can call it but it does not
// leak into non-test builds.
#[cfg(test)]
impl<M: Mem> NvmeController<M> {
    fn mem_mut_write(&mut self, gpa: u64, buf: &[u8]) {
        self.mem.write(gpa, buf);
    }
}
