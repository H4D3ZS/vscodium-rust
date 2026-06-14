//! Intel 82093AA I/O APIC (I/O Advanced Programmable Interrupt Controller).
//!
//! The I/O APIC receives interrupt signals from external/system devices and
//! routes them, via a programmable redirection table, to one or more Local
//! APICs. It replaces the legacy 8259 PIC cascade for interrupt routing in
//! APIC mode.
//!
//! MMIO interface (default base 0xFEC00000, 0x20 bytes):
//! - IOREGSEL (offset 0x00): an 8-bit index latch selecting which internal
//!   32-bit register the data window addresses.
//! - IOWIN    (offset 0x10): the 32-bit data window into the indexed register.
//!
//! Internal (indexed) registers:
//! - 0x00       IOAPICID   - bits 24-27 hold the APIC ID.
//! - 0x01       IOAPICVER  - version in bits 0-7, max redirection entry
//!   (number of RTEs minus one) in bits 16-23.
//! - 0x02       IOAPICARB  - arbitration ID in bits 24-27.
//! - 0x10-0x3F  Redirection table - 24 entries, each a 64-bit value split
//!   into two consecutive 32-bit halves (low half at the even index).
//!
//! Redirection-table entry (RTE) layout:
//! - bits  0-7  vector
//! - bits  8-10 delivery mode
//! - bit  11    destination mode (0 = physical, 1 = logical)
//! - bit  12    delivery status (read-only)
//! - bit  13    interrupt input pin polarity (0 = high active, 1 = low active)
//! - bit  14    remote IRR (read-only, level-triggered EOI tracking)
//! - bit  15    trigger mode (0 = edge, 1 = level)
//! - bit  16    mask (1 = masked)
//! - bits 56-63 destination field

use super::bus::MmioDevice;

/// Fixed MMIO base address of the I/O APIC.
pub const IOAPIC_BASE: u64 = 0xFEC00000;
/// Size of the MMIO window (covers IOREGSEL and IOWIN).
pub const IOAPIC_SIZE: u64 = 0x20;

/// Offset of the index-select register (IOREGSEL).
const IOREGSEL: u64 = 0x00;
/// Offset of the data window register (IOWIN).
const IOWIN: u64 = 0x10;

/// Indexed register: IOAPIC identification.
const REG_ID: u32 = 0x00;
/// Indexed register: IOAPIC version.
const REG_VER: u32 = 0x01;
/// Indexed register: IOAPIC arbitration.
const REG_ARB: u32 = 0x02;
/// First indexed register holding a redirection-table half.
const REG_REDTBL_BASE: u32 = 0x10;

/// Reported hardware version (matches the 82093AA / modern I/O APICs).
const IOAPIC_VERSION: u32 = 0x20;
/// Number of redirection-table entries (input interrupt lines / GSIs).
pub const IOAPIC_NUM_RTE: usize = 24;
/// Max redirection entry = number of entries minus one (reported in IOAPICVER).
const MAX_REDIRECTION_ENTRY: u32 = (IOAPIC_NUM_RTE as u32) - 1;

// Redirection-table entry field masks/positions (within the 64-bit entry).
const RTE_VECTOR_MASK: u64 = 0xFF;
const RTE_DELIVERY_MODE_SHIFT: u64 = 8;
const RTE_DELIVERY_STATUS: u64 = 1 << 12; // read-only
const RTE_REMOTE_IRR: u64 = 1 << 14; // read-only
const RTE_TRIGGER_MODE: u64 = 1 << 15;
const RTE_MASK: u64 = 1 << 16;
const RTE_DEST_SHIFT: u64 = 56;

/// Bits within a redirection-table entry that software may write. The delivery
/// status (bit 12) and remote IRR (bit 14) bits are read-only.
const RTE_WRITABLE_MASK: u64 = !(RTE_DELIVERY_STATUS | RTE_REMOTE_IRR);

/// A pending interrupt delivery produced by an asserted, unmasked input line.
///
/// The VMM polls these out and routes them to the appropriate Local APIC(s)
/// using the destination/trigger information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingIrq {
    /// Source global system interrupt (input pin) that produced this delivery.
    pub gsi: u8,
    /// Interrupt vector to inject into the target LAPIC.
    pub vector: u8,
    /// Destination field (physical APIC ID or logical destination).
    pub destination: u8,
    /// Trigger mode: false = edge, true = level.
    pub level_triggered: bool,
}

/// Intel 82093AA I/O APIC.
pub struct IoApic {
    /// IOREGSEL index latch (selects the indexed register for IOWIN).
    ioregsel: u8,
    /// APIC ID (bits 24-27 of IOAPICID).
    id: u32,
    /// Redirection table: 24 entries, each a full 64-bit value.
    redirection_table: [u64; IOAPIC_NUM_RTE],
    /// Current asserted state of each input line (for level-trigger tracking).
    line_asserted: [bool; IOAPIC_NUM_RTE],
    /// Pending deliveries awaiting the VMM to route them to LAPIC(s).
    pending: Vec<PendingIrq>,
}

impl Default for IoApic {
    fn default() -> Self {
        Self::new()
    }
}

impl IoApic {
    pub fn new() -> Self {
        IoApic {
            ioregsel: 0,
            id: 0,
            // Power-on default: every entry masked (bit 16 set), as on real HW.
            redirection_table: [RTE_MASK; IOAPIC_NUM_RTE],
            line_asserted: [false; IOAPIC_NUM_RTE],
            pending: Vec::new(),
        }
    }

    /// Read an indexed (internal) 32-bit register.
    fn read_indexed(&self, index: u32) -> u32 {
        match index {
            REG_ID => self.id,
            REG_VER => IOAPIC_VERSION | (MAX_REDIRECTION_ENTRY << 16),
            // Arbitration ID mirrors the APIC ID field (bits 24-27).
            REG_ARB => self.id,
            idx if idx >= REG_REDTBL_BASE
                && (idx as usize) < REG_REDTBL_BASE as usize + IOAPIC_NUM_RTE * 2 =>
            {
                let offset = (idx - REG_REDTBL_BASE) as usize;
                let entry = offset / 2;
                let entry_val = self.redirection_table[entry];
                if offset % 2 == 0 {
                    entry_val as u32 // low half
                } else {
                    (entry_val >> 32) as u32 // high half
                }
            }
            _ => 0,
        }
    }

    /// Write an indexed (internal) 32-bit register.
    fn write_indexed(&mut self, index: u32, value: u32) {
        match index {
            REG_ID => {
                // Only bits 24-27 are writable.
                self.id = value & 0x0F00_0000;
            }
            // Version and arbitration registers are read-only.
            REG_VER | REG_ARB => {}
            idx if idx >= REG_REDTBL_BASE
                && (idx as usize) < REG_REDTBL_BASE as usize + IOAPIC_NUM_RTE * 2 =>
            {
                let offset = (idx - REG_REDTBL_BASE) as usize;
                let entry = offset / 2;
                let old = self.redirection_table[entry];
                let new = if offset % 2 == 0 {
                    // Low half: preserve read-only bits (delivery status, IRR).
                    let masked = (value as u64) & (RTE_WRITABLE_MASK & 0xFFFF_FFFF);
                    let ro = old & (!RTE_WRITABLE_MASK & 0xFFFF_FFFF);
                    (old & 0xFFFF_FFFF_0000_0000) | masked | ro
                } else {
                    // High half: destination field, no read-only bits here.
                    (old & 0x0000_0000_FFFF_FFFF) | ((value as u64) << 32)
                };
                self.redirection_table[entry] = new;

                // If the line is currently asserted and was just unmasked,
                // (re)evaluate it so a previously-blocked level/edge delivers.
                if offset % 2 == 0 && self.line_asserted[entry] {
                    self.evaluate_entry(entry);
                }
            }
            _ => {}
        }
    }

    /// Assert or de-assert an input line (GSI). When an unmasked entry is
    /// asserted, a pending delivery is recorded for the VMM to route.
    pub fn set_irq(&mut self, gsi: u8, asserted: bool) {
        let entry = gsi as usize;
        if entry >= IOAPIC_NUM_RTE {
            return;
        }
        self.line_asserted[entry] = asserted;
        if asserted {
            self.evaluate_entry(entry);
        }
    }

    /// Evaluate a redirection entry and, if unmasked, record a pending delivery.
    fn evaluate_entry(&mut self, entry: usize) {
        let rte = self.redirection_table[entry];
        if rte & RTE_MASK != 0 {
            // Masked: do not deliver.
            return;
        }
        let vector = (rte & RTE_VECTOR_MASK) as u8;
        let destination = ((rte >> RTE_DEST_SHIFT) & 0xFF) as u8;
        let level_triggered = rte & RTE_TRIGGER_MODE != 0;
        self.pending.push(PendingIrq {
            gsi: entry as u8,
            vector,
            destination,
            level_triggered,
        });
    }

    /// Whether there are pending deliveries awaiting routing.
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Borrow the queued pending deliveries without consuming them.
    pub fn pending(&self) -> &[PendingIrq] {
        &self.pending
    }

    /// Take (and clear) all pending deliveries for the VMM to route to LAPICs.
    pub fn take_pending(&mut self) -> Vec<PendingIrq> {
        std::mem::take(&mut self.pending)
    }

    /// Get the I/O APIC ID (bits 24-27 of IOAPICID).
    pub fn apic_id(&self) -> u8 {
        ((self.id >> 24) & 0x0F) as u8
    }
}

impl MmioDevice for IoApic {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        let offset = addr.wrapping_sub(IOAPIC_BASE);
        // Registers are 32-bit; determine the aligned register and byte offset.
        let reg_base = offset & !0x3;
        let byte_off = (offset & 0x3) as usize;

        let value = match reg_base {
            IOREGSEL => self.ioregsel as u32,
            IOWIN => self.read_indexed(self.ioregsel as u32),
            _ => 0,
        };

        // Extract the requested byte width from the 32-bit register value.
        for (i, byte) in data.iter_mut().enumerate() {
            let shift = (byte_off + i) * 8;
            *byte = if shift < 32 {
                ((value >> shift) & 0xFF) as u8
            } else {
                0
            };
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        let offset = addr.wrapping_sub(IOAPIC_BASE);
        let reg_base = offset & !0x3;
        let byte_off = (offset & 0x3) as usize;

        match reg_base {
            IOREGSEL => {
                // IOREGSEL is an 8-bit latch; a write to its low byte sets it.
                if byte_off == 0 {
                    if let Some(b) = data.first() {
                        self.ioregsel = *b;
                    }
                }
            }
            IOWIN => {
                // Read-modify-write to support sub-32-bit accesses.
                let mut value = self.read_indexed(self.ioregsel as u32);
                for (i, byte) in data.iter().enumerate() {
                    let shift = (byte_off + i) * 8;
                    if shift < 32 {
                        value &= !(0xFFu32 << shift);
                        value |= (*byte as u32) << shift;
                    }
                }
                self.write_indexed(self.ioregsel as u32, value);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: program IOREGSEL then write a 32-bit value through IOWIN.
    fn write_reg(io: &mut IoApic, index: u8, value: u32) {
        io.write(IOAPIC_BASE + IOREGSEL, &[index]);
        io.write(IOAPIC_BASE + IOWIN, &value.to_le_bytes());
    }

    /// Helper: program IOREGSEL then read a 32-bit value through IOWIN.
    fn read_reg(io: &mut IoApic, index: u8) -> u32 {
        io.write(IOAPIC_BASE + IOREGSEL, &[index]);
        let mut buf = [0u8; 4];
        io.read(IOAPIC_BASE + IOWIN, &mut buf);
        u32::from_le_bytes(buf)
    }

    #[test]
    fn test_ioregsel_iowin_indirect_access() {
        let mut io = IoApic::new();

        // Select IOAPICID and write an ID (bits 24-27 writable).
        write_reg(&mut io, REG_ID as u8, 0x0F00_0000);
        assert_eq!(read_reg(&mut io, REG_ID as u8), 0x0F00_0000);
        assert_eq!(io.apic_id(), 0x0F);

        // IOREGSEL itself reads back the latched index.
        io.write(IOAPIC_BASE + IOREGSEL, &[REG_VER as u8]);
        let mut sel = [0u8; 4];
        io.read(IOAPIC_BASE + IOREGSEL, &mut sel);
        assert_eq!(sel[0], REG_VER as u8);
    }

    #[test]
    fn test_id_only_writable_bits() {
        let mut io = IoApic::new();
        // Lower/other bits must be masked off; only 24-27 stick.
        write_reg(&mut io, REG_ID as u8, 0xFFFF_FFFF);
        assert_eq!(read_reg(&mut io, REG_ID as u8), 0x0F00_0000);
    }

    #[test]
    fn test_version_and_max_redir() {
        let mut io = IoApic::new();
        let ver = read_reg(&mut io, REG_VER as u8);
        // Version in bits 0-7.
        assert_eq!(ver & 0xFF, IOAPIC_VERSION);
        // Max redirection entry in bits 16-23 = 23 (24 entries - 1).
        assert_eq!((ver >> 16) & 0xFF, 23);
    }

    #[test]
    fn test_version_register_readonly() {
        let mut io = IoApic::new();
        let before = read_reg(&mut io, REG_VER as u8);
        write_reg(&mut io, REG_VER as u8, 0xDEAD_BEEF);
        assert_eq!(read_reg(&mut io, REG_VER as u8), before);
    }

    #[test]
    fn test_rte_low_high_half_write_readback() {
        let mut io = IoApic::new();
        // Redirection entry 5: low half index = 0x10 + 5*2 = 0x1A, high = 0x1B.
        let low_idx = (REG_REDTBL_BASE + 5 * 2) as u8;
        let high_idx = low_idx + 1;

        // Low half: vector 0x42, trigger=level (bit15), unmasked (bit16=0).
        let low = 0x0000_8042u32;
        // High half: destination = 0x03 in bits 56-63 -> bits 24-31 of high word.
        let high = 0x0300_0000u32;

        write_reg(&mut io, low_idx, low);
        write_reg(&mut io, high_idx, high);

        assert_eq!(read_reg(&mut io, low_idx), low);
        assert_eq!(read_reg(&mut io, high_idx), high);

        // Full 64-bit entry reflects both halves.
        assert_eq!(
            io.redirection_table[5],
            ((high as u64) << 32) | (low as u64)
        );
    }

    #[test]
    fn test_rte_readonly_bits_preserved() {
        let mut io = IoApic::new();
        let low_idx = REG_REDTBL_BASE as u8; // entry 0 low half
        // Pre-set read-only bits (delivery status + remote IRR) in the entry.
        io.redirection_table[0] = RTE_DELIVERY_STATUS | RTE_REMOTE_IRR;
        // Software attempts to write them as 1 and also set vector.
        write_reg(&mut io, low_idx, 0xFFFF_FFFF);
        let rte = io.redirection_table[0];
        // Read-only bits keep their prior value (here: set), writable bits change.
        assert_eq!(rte & RTE_DELIVERY_STATUS, RTE_DELIVERY_STATUS);
        assert_eq!(rte & RTE_REMOTE_IRR, RTE_REMOTE_IRR);
        assert_eq!(rte & RTE_VECTOR_MASK, 0xFF);
    }

    #[test]
    fn test_byte_access_widths() {
        let mut io = IoApic::new();
        let low_idx = REG_REDTBL_BASE as u8;

        // Write the low half one byte at a time through IOWIN. Byte 1 (bits
        // 8-15) avoids the read-only bits 12 (delivery status) and 14 (remote
        // IRR), so the written value survives the writable-bit masking.
        io.write(IOAPIC_BASE + IOREGSEL, &[low_idx]);
        io.write(IOAPIC_BASE + IOWIN, &[0x55]); // byte 0
        io.write(IOAPIC_BASE + IOWIN + 1, &[0x23]); // byte 1
        // bytes 2-3 stay 0; bit16 (mask) cleared.

        let v = read_reg(&mut io, low_idx);
        assert_eq!(v & 0xFFFF, 0x2355);

        // 16-bit read of the low word.
        io.write(IOAPIC_BASE + IOREGSEL, &[low_idx]);
        let mut buf = [0u8; 2];
        io.read(IOAPIC_BASE + IOWIN, &mut buf);
        assert_eq!(u16::from_le_bytes(buf), 0x2355);
    }

    #[test]
    fn test_mask_gates_delivery() {
        let mut io = IoApic::new();
        // Entries default masked: asserting should produce nothing.
        io.set_irq(3, true);
        assert!(!io.has_pending());
        assert!(io.take_pending().is_empty());

        // Unmask entry 3 with a vector, then assert.
        let low_idx = (REG_REDTBL_BASE + 3 * 2) as u8;
        write_reg(&mut io, low_idx, 0x0000_0030); // vector 0x30, mask=0
        io.set_irq(3, true);
        assert!(io.has_pending());
    }

    #[test]
    fn test_set_irq_pending_vector() {
        let mut io = IoApic::new();
        let entry = 7usize;
        let low_idx = (REG_REDTBL_BASE + (entry as u32) * 2) as u8;
        let high_idx = low_idx + 1;

        // vector 0x71, level-triggered, unmasked.
        write_reg(&mut io, low_idx, 0x0000_8071);
        // destination = 0x02.
        write_reg(&mut io, high_idx, 0x0200_0000);

        io.set_irq(entry as u8, true);

        // pending() borrows without consuming.
        assert_eq!(io.pending().len(), 1);
        let p = io.pending()[0];
        assert_eq!(p.gsi, entry as u8);
        assert_eq!(p.vector, 0x71);
        assert_eq!(p.destination, 0x02);
        assert!(p.level_triggered);

        // take_pending() drains the queue.
        let drained = io.take_pending();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].vector, 0x71);
        assert!(!io.has_pending());
    }

    #[test]
    fn test_unmask_while_asserted_delivers() {
        let mut io = IoApic::new();
        let entry = 10usize;
        let low_idx = (REG_REDTBL_BASE + (entry as u32) * 2) as u8;

        // Assert while masked: no delivery.
        io.set_irq(entry as u8, true);
        assert!(!io.has_pending());

        // Now unmask with a vector while the line is still asserted.
        write_reg(&mut io, low_idx, 0x0000_0050); // vector 0x50, mask=0
        assert!(io.has_pending());
        assert_eq!(io.take_pending()[0].vector, 0x50);
    }

    #[test]
    fn test_set_irq_out_of_range_ignored() {
        let mut io = IoApic::new();
        io.set_irq(99, true);
        assert!(!io.has_pending());
    }
}
