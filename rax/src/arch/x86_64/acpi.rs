//! ACPI firmware table builders.
//!
//! This module provides pure functions that build standard ACPI table byte
//! blobs (`Vec<u8>`) with correct headers and checksums. Every ACPI table
//! carries a single-byte checksum chosen such that the modular sum of all
//! bytes in the table equals zero.
//!
//! The tables produced here mirror what a minimal firmware would hand to a
//! Linux guest: an RSDP pointing at an RSDT/XSDT, which in turn reference the
//! FADT (FACP), MADT (APIC), HPET and MCFG tables. The FADT references a DSDT
//! and FACS.
//!
//! All builders are host-independent and operate purely on bytes, so they can
//! be unit-tested without any guest memory. A top-level orchestrator,
//! [`build_acpi_tables`], computes the guest-physical placement of every table
//! given a base load address and returns the RSDP together with a list of
//! `(guest_phys_addr, blob)` pairs ready to be written into guest memory.

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Hardware constants
// ---------------------------------------------------------------------------

/// Physical address of the local APIC register block.
pub const LOCAL_APIC_BASE: u32 = 0xFEE0_0000;
/// Physical address of the I/O APIC register block.
pub const IO_APIC_BASE: u32 = 0xFEC0_0000;
/// Physical address of the HPET register block.
pub const HPET_BASE: u64 = 0xFED0_0000;

/// Default I/O APIC identifier.
pub const IO_APIC_ID: u8 = 0;

// Legacy ACPI PM register block addresses. These are the canonical PIIX4
// values used by QEMU's "pc" machine; they keep the FADT self-consistent even
// though the emulator does not necessarily decode these ports.
const PM1A_EVT_BLK: u32 = 0x0600;
const PM1A_CNT_BLK: u32 = 0x0604;
const PM_TMR_BLK: u32 = 0x0608;
const SCI_INT: u16 = 9;
const SMI_CMD: u32 = 0x00B2;
const ACPI_ENABLE: u8 = 0xF1;
const ACPI_DISABLE: u8 = 0xF0;

/// OEM identifier placed in every SDT header (6 bytes, space padded).
const OEM_ID: &[u8; 6] = b"RAXVMM";
/// OEM table identifier placed in every SDT header (8 bytes, space padded).
const OEM_TABLE_ID: &[u8; 8] = b"RAXACPI ";
/// OEM revision used across all tables.
const OEM_REVISION: u32 = 1;
/// Creator (compiler/assembler) identifier.
const CREATOR_ID: &[u8; 4] = b"RAX ";
/// Creator revision.
const CREATOR_REVISION: u32 = 1;

/// Length of a common ACPI System Description Table header.
pub const SDT_HEADER_LEN: usize = 36;

// ---------------------------------------------------------------------------
// Checksum helpers
// ---------------------------------------------------------------------------

/// Compute the ACPI checksum byte over `len` bytes starting at `offset`.
///
/// The returned value is the byte which, when stored in the checksum field,
/// makes the modular (8-bit) sum of all bytes in the range equal zero.
fn compute_checksum(bytes: &[u8]) -> u8 {
    let sum = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    0u8.wrapping_sub(sum)
}

/// Write the one-byte checksum for a range `[start, end)` into `bytes[csum_off]`.
///
/// The checksum field must already be zeroed (which it is when freshly pushed).
fn set_checksum(bytes: &mut [u8], start: usize, end: usize, csum_off: usize) {
    bytes[csum_off] = 0;
    bytes[csum_off] = compute_checksum(&bytes[start..end]);
}

// ---------------------------------------------------------------------------
// SDT header builder
// ---------------------------------------------------------------------------

/// Append a common ACPI SDT header to `out`.
///
/// The `length` and `checksum` fields are written as placeholders (zero) and
/// must be finalized by the caller once the full table body is known, via
/// [`finalize_sdt`].
///
/// Header layout (36 bytes):
/// - signature[4]
/// - length (u32)
/// - revision (u8)
/// - checksum (u8)
/// - oem_id[6]
/// - oem_table_id[8]
/// - oem_revision (u32)
/// - creator_id[4]
/// - creator_revision (u32)
fn push_sdt_header(out: &mut Vec<u8>, signature: &[u8; 4], revision: u8) {
    out.extend_from_slice(signature); // 0..4 signature
    out.extend_from_slice(&0u32.to_le_bytes()); // 4..8 length (placeholder)
    out.push(revision); // 8 revision
    out.push(0); // 9 checksum (placeholder)
    out.extend_from_slice(OEM_ID); // 10..16 oem id
    out.extend_from_slice(OEM_TABLE_ID); // 16..24 oem table id
    out.extend_from_slice(&OEM_REVISION.to_le_bytes()); // 24..28 oem revision
    out.extend_from_slice(CREATOR_ID); // 28..32 creator id
    out.extend_from_slice(&CREATOR_REVISION.to_le_bytes()); // 32..36 creator revision
}

/// Finalize an SDT-style table: write the real length at offset 4 and the
/// checksum at offset 9 so the whole table sums to zero.
fn finalize_sdt(table: &mut Vec<u8>) {
    let len = table.len() as u32;
    table[4..8].copy_from_slice(&len.to_le_bytes());
    let total = table.len();
    set_checksum(table, 0, total, 9);
}

// ---------------------------------------------------------------------------
// RSDP
// ---------------------------------------------------------------------------

/// Length of an ACPI 2.0+ RSDP structure.
pub const RSDP_LEN: usize = 36;

/// Build a Root System Description Pointer (RSDP), revision 2 (ACPI 2.0+).
///
/// Layout:
/// - signature "RSD PTR " (8 bytes)
/// - checksum (u8) over the first 20 bytes (ACPI 1.0 portion)
/// - oem_id[6]
/// - revision (u8) = 2
/// - rsdt_address (u32)
/// - length (u32) = 36
/// - xsdt_address (u64)
/// - extended_checksum (u8) over the whole 36-byte structure
/// - reserved[3]
pub fn build_rsdp(rsdt_addr: u32, xsdt_addr: u64) -> Vec<u8> {
    let mut rsdp = Vec::with_capacity(RSDP_LEN);
    rsdp.extend_from_slice(b"RSD PTR "); // 0..8 signature
    rsdp.push(0); // 8 checksum (ACPI 1.0)
    rsdp.extend_from_slice(OEM_ID); // 9..15 oem id
    rsdp.push(2); // 15 revision (ACPI 2.0+)
    rsdp.extend_from_slice(&rsdt_addr.to_le_bytes()); // 16..20 rsdt address
    rsdp.extend_from_slice(&(RSDP_LEN as u32).to_le_bytes()); // 20..24 length
    rsdp.extend_from_slice(&xsdt_addr.to_le_bytes()); // 24..32 xsdt address
    rsdp.push(0); // 32 extended checksum
    rsdp.extend_from_slice(&[0u8; 3]); // 33..36 reserved

    // ACPI 1.0 checksum covers the first 20 bytes.
    set_checksum(&mut rsdp, 0, 20, 8);
    // Extended checksum covers the full structure.
    set_checksum(&mut rsdp, 0, RSDP_LEN, 32);
    rsdp
}

// ---------------------------------------------------------------------------
// RSDT / XSDT
// ---------------------------------------------------------------------------

/// Build a Root System Description Table (RSDT) with 32-bit child pointers.
pub fn build_rsdt(entries: &[u32]) -> Vec<u8> {
    let mut t = Vec::new();
    push_sdt_header(&mut t, b"RSDT", 1);
    for &e in entries {
        t.extend_from_slice(&e.to_le_bytes());
    }
    finalize_sdt(&mut t);
    t
}

/// Build an Extended System Description Table (XSDT) with 64-bit child pointers.
pub fn build_xsdt(entries: &[u64]) -> Vec<u8> {
    let mut t = Vec::new();
    push_sdt_header(&mut t, b"XSDT", 1);
    for &e in entries {
        t.extend_from_slice(&e.to_le_bytes());
    }
    finalize_sdt(&mut t);
    t
}

// ---------------------------------------------------------------------------
// FADT (FACP)
// ---------------------------------------------------------------------------

/// A Generic Address Structure (GAS) as used by ACPI 2.0+ FADT fields.
fn push_gas(out: &mut Vec<u8>, space_id: u8, bit_width: u8, address: u64) {
    out.push(space_id); // address space id (1 = system I/O, 0 = system memory)
    out.push(bit_width); // register bit width
    out.push(0); // register bit offset
    out.push(0); // access size
    out.extend_from_slice(&address.to_le_bytes());
}

/// Build a Fixed ACPI Description Table (FADT / "FACP").
///
/// The table is revision 6 and includes both the legacy 32-bit
/// FIRMWARE_CTRL / DSDT fields and the 64-bit X_ counterparts, along with the
/// standard PM register block addresses (PIIX4 values) and a sensible set of
/// fixed feature fields.
pub fn build_fadt(facs_addr: u32, dsdt_addr: u32) -> Vec<u8> {
    let mut t = Vec::new();
    push_sdt_header(&mut t, b"FACP", 6);

    // 36: FIRMWARE_CTRL (FACS, 32-bit)
    t.extend_from_slice(&facs_addr.to_le_bytes());
    // 40: DSDT (32-bit)
    t.extend_from_slice(&dsdt_addr.to_le_bytes());
    // 44: reserved (was INT_MODEL in ACPI 1.0)
    t.push(0);
    // 45: Preferred_PM_Profile (1 = Desktop)
    t.push(1);
    // 46: SCI_INT (u16)
    t.extend_from_slice(&SCI_INT.to_le_bytes());
    // 48: SMI_CMD (u32)
    t.extend_from_slice(&SMI_CMD.to_le_bytes());
    // 52: ACPI_ENABLE
    t.push(ACPI_ENABLE);
    // 53: ACPI_DISABLE
    t.push(ACPI_DISABLE);
    // 54: S4BIOS_REQ
    t.push(0);
    // 55: PSTATE_CNT
    t.push(0);
    // 56: PM1a_EVT_BLK (u32)
    t.extend_from_slice(&PM1A_EVT_BLK.to_le_bytes());
    // 60: PM1b_EVT_BLK (u32)
    t.extend_from_slice(&0u32.to_le_bytes());
    // 64: PM1a_CNT_BLK (u32)
    t.extend_from_slice(&PM1A_CNT_BLK.to_le_bytes());
    // 68: PM1b_CNT_BLK (u32)
    t.extend_from_slice(&0u32.to_le_bytes());
    // 72: PM2_CNT_BLK (u32)
    t.extend_from_slice(&0u32.to_le_bytes());
    // 76: PM_TMR_BLK (u32)
    t.extend_from_slice(&PM_TMR_BLK.to_le_bytes());
    // 80: GPE0_BLK (u32)
    t.extend_from_slice(&0u32.to_le_bytes());
    // 84: GPE1_BLK (u32)
    t.extend_from_slice(&0u32.to_le_bytes());
    // 88: PM1_EVT_LEN
    t.push(4);
    // 89: PM1_CNT_LEN
    t.push(2);
    // 90: PM2_CNT_LEN
    t.push(0);
    // 91: PM_TMR_LEN
    t.push(4);
    // 92: GPE0_BLK_LEN
    t.push(0);
    // 93: GPE1_BLK_LEN
    t.push(0);
    // 94: GPE1_BASE
    t.push(0);
    // 95: CST_CNT
    t.push(0);
    // 96: P_LVL2_LAT (u16)
    t.extend_from_slice(&0u16.to_le_bytes());
    // 98: P_LVL3_LAT (u16)
    t.extend_from_slice(&0u16.to_le_bytes());
    // 100: FLUSH_SIZE (u16)
    t.extend_from_slice(&0u16.to_le_bytes());
    // 102: FLUSH_STRIDE (u16)
    t.extend_from_slice(&0u16.to_le_bytes());
    // 104: DUTY_OFFSET
    t.push(0);
    // 105: DUTY_WIDTH
    t.push(0);
    // 106: DAY_ALRM
    t.push(0);
    // 107: MON_ALRM
    t.push(0);
    // 108: CENTURY
    t.push(0);
    // 109: IAPC_BOOT_ARCH (u16) - bit1 = 8042 present
    t.extend_from_slice(&0x0002u16.to_le_bytes());
    // 111: reserved
    t.push(0);
    // 112: Flags (u32): WBINVD | PROC_C1 | SLP_BUTTON | TMR_VAL_EXT | RESET_REG_SUP
    //   WBINVD=1<<0, PROC_C1=1<<2, SLP_BUTTON=1<<5, TMR_VAL_EXT=1<<8, RESET_REG_SUP=1<<10
    let flags: u32 = (1 << 0) | (1 << 2) | (1 << 5) | (1 << 8) | (1 << 10);
    t.extend_from_slice(&flags.to_le_bytes());
    // 116: RESET_REG (GAS, 12 bytes) - system I/O port 0xCF9
    push_gas(&mut t, 1, 8, 0x0000_0CF9);
    // 128: RESET_VALUE
    t.push(0x06);
    // 129: ARM_BOOT_ARCH (u16)
    t.extend_from_slice(&0u16.to_le_bytes());
    // 131: FADT minor version
    t.push(0);
    // 132: X_FIRMWARE_CTRL (u64)
    t.extend_from_slice(&(facs_addr as u64).to_le_bytes());
    // 140: X_DSDT (u64)
    t.extend_from_slice(&(dsdt_addr as u64).to_le_bytes());
    // 148: X_PM1a_EVT_BLK (GAS)
    push_gas(&mut t, 1, 32, PM1A_EVT_BLK as u64);
    // 160: X_PM1b_EVT_BLK (GAS)
    push_gas(&mut t, 0, 0, 0);
    // 172: X_PM1a_CNT_BLK (GAS)
    push_gas(&mut t, 1, 16, PM1A_CNT_BLK as u64);
    // 184: X_PM1b_CNT_BLK (GAS)
    push_gas(&mut t, 0, 0, 0);
    // 196: X_PM2_CNT_BLK (GAS)
    push_gas(&mut t, 0, 0, 0);
    // 208: X_PM_TMR_BLK (GAS)
    push_gas(&mut t, 1, 32, PM_TMR_BLK as u64);
    // 220: X_GPE0_BLK (GAS)
    push_gas(&mut t, 0, 0, 0);
    // 232: X_GPE1_BLK (GAS)
    push_gas(&mut t, 0, 0, 0);
    // 244: SLEEP_CONTROL_REG (GAS)
    push_gas(&mut t, 0, 0, 0);
    // 256: SLEEP_STATUS_REG (GAS)
    push_gas(&mut t, 0, 0, 0);
    // 268: Hypervisor Vendor Identity (u64)
    t.extend_from_slice(b"RAXVMM\0\0");

    finalize_sdt(&mut t);
    t
}

// ---------------------------------------------------------------------------
// MADT (APIC)
// ---------------------------------------------------------------------------

/// MADT flag: legacy dual-8259 PICs are present (PCAT_COMPAT).
const MADT_PCAT_COMPAT: u32 = 1 << 0;

// MADT entry type identifiers.
const MADT_TYPE_LOCAL_APIC: u8 = 0;
const MADT_TYPE_IO_APIC: u8 = 1;
const MADT_TYPE_INT_SRC_OVERRIDE: u8 = 2;

/// Append a Processor Local APIC entry (type 0, length 8).
fn push_local_apic(out: &mut Vec<u8>, processor_uid: u8, apic_id: u8) {
    out.push(MADT_TYPE_LOCAL_APIC);
    out.push(8); // length
    out.push(processor_uid);
    out.push(apic_id);
    out.extend_from_slice(&1u32.to_le_bytes()); // flags: bit0 = Enabled
}

/// Append an I/O APIC entry (type 1, length 12).
fn push_io_apic(out: &mut Vec<u8>, io_apic_id: u8, address: u32, gsi_base: u32) {
    out.push(MADT_TYPE_IO_APIC);
    out.push(12); // length
    out.push(io_apic_id);
    out.push(0); // reserved
    out.extend_from_slice(&address.to_le_bytes());
    out.extend_from_slice(&gsi_base.to_le_bytes());
}

/// Append an Interrupt Source Override entry (type 2, length 10).
fn push_int_src_override(out: &mut Vec<u8>, bus: u8, source: u8, gsi: u32, flags: u16) {
    out.push(MADT_TYPE_INT_SRC_OVERRIDE);
    out.push(10); // length
    out.push(bus); // 0 = ISA
    out.push(source); // source IRQ
    out.extend_from_slice(&gsi.to_le_bytes());
    out.extend_from_slice(&flags.to_le_bytes());
}

/// Build a Multiple APIC Description Table (MADT / "APIC").
///
/// Contains:
/// - the local APIC physical address and PCAT_COMPAT flag,
/// - one Processor Local APIC entry per CPU (`cpu_count`),
/// - a single I/O APIC entry at [`IO_APIC_BASE`] with GSI base 0,
/// - the standard ISA interrupt source overrides:
///     - IRQ0 -> GSI2 (PIT timer remap),
///     - IRQ9 active-high, level-triggered (SCI).
pub fn build_madt(cpu_count: u32) -> Vec<u8> {
    let mut t = Vec::new();
    push_sdt_header(&mut t, b"APIC", 5);

    // 36: Local APIC address (u32)
    t.extend_from_slice(&LOCAL_APIC_BASE.to_le_bytes());
    // 40: Flags (u32)
    t.extend_from_slice(&MADT_PCAT_COMPAT.to_le_bytes());

    // Processor Local APIC entries, one per CPU.
    for cpu in 0..cpu_count {
        push_local_apic(&mut t, cpu as u8, cpu as u8);
    }

    // I/O APIC.
    push_io_apic(&mut t, IO_APIC_ID, IO_APIC_BASE, 0);

    // Interrupt source overrides.
    // IRQ0 (PIT) is wired to GSI2; flags 0 = conforms to bus (edge/high).
    push_int_src_override(&mut t, 0, 0, 2, 0);
    // IRQ9 (SCI): active-high (0b01), level-triggered (0b11 << 2) => 0x000D.
    push_int_src_override(&mut t, 0, 9, 9, 0x000D);

    finalize_sdt(&mut t);
    t
}

/// Count the Processor Local APIC entries in a built MADT blob.
///
/// Useful for tests; walks the variable-length entry list following the
/// 44-byte MADT fixed header.
pub fn madt_count_local_apics(madt: &[u8]) -> usize {
    count_madt_entries(madt, MADT_TYPE_LOCAL_APIC)
}

/// Count the I/O APIC entries in a built MADT blob.
pub fn madt_count_io_apics(madt: &[u8]) -> usize {
    count_madt_entries(madt, MADT_TYPE_IO_APIC)
}

fn count_madt_entries(madt: &[u8], entry_type: u8) -> usize {
    let mut count = 0;
    let mut off = 44; // fixed MADT header: 36 (SDT) + 4 (lapic addr) + 4 (flags)
    while off + 2 <= madt.len() {
        let etype = madt[off];
        let elen = madt[off + 1] as usize;
        if elen == 0 || off + elen > madt.len() {
            break;
        }
        if etype == entry_type {
            count += 1;
        }
        off += elen;
    }
    count
}

// ---------------------------------------------------------------------------
// HPET
// ---------------------------------------------------------------------------

/// Build an HPET description table.
///
/// Layout after the SDT header:
/// - Event Timer Block ID (u32)
/// - Base Address (GAS, 12 bytes) in system memory space
/// - HPET Number (u8)
/// - Minimum Tick (u16)
/// - Page Protection (u8)
pub fn build_hpet() -> Vec<u8> {
    let mut t = Vec::new();
    push_sdt_header(&mut t, b"HPET", 1);

    // Event Timer Block ID. Encodes the hardware revision / vendor / counter
    // size / comparator count. This is the standard Intel ICH value used by
    // QEMU (hw rev 1, 3 comparators, 64-bit main counter, vendor 0x8086).
    // bits[31:16]=vendor 0x8086, bit15=legacy capable, bit13=count size cap,
    // bits[12:8]=number of comparators (here 2 => 3 timers), bits[7:0]=hw rev.
    let event_timer_block_id: u32 = (0x8086 << 16) | (1 << 15) | (1 << 13) | (2 << 8) | 0x01;
    t.extend_from_slice(&event_timer_block_id.to_le_bytes());

    // Base address GAS: system memory space (0), 64-bit width.
    push_gas(&mut t, 0, 64, HPET_BASE);

    // HPET sequence number.
    t.push(0);
    // Minimum clock tick in periodic mode.
    t.extend_from_slice(&0x0080u16.to_le_bytes());
    // Page protection and OEM attributes (0 = no guarantee).
    t.push(0);

    finalize_sdt(&mut t);
    t
}

// ---------------------------------------------------------------------------
// MCFG (PCI ECAM)
// ---------------------------------------------------------------------------

/// Build an MCFG table describing a single PCI ECAM allocation.
///
/// `ecam_base` is the physical base of the enhanced configuration space; the
/// segment group is 0 and the bus range is `[bus_start, bus_end]`.
pub fn build_mcfg(ecam_base: u64, bus_start: u8, bus_end: u8) -> Vec<u8> {
    let mut t = Vec::new();
    push_sdt_header(&mut t, b"MCFG", 1);

    // 36: reserved (u64)
    t.extend_from_slice(&0u64.to_le_bytes());

    // Allocation entry (16 bytes).
    t.extend_from_slice(&ecam_base.to_le_bytes()); // base address
    t.extend_from_slice(&0u16.to_le_bytes()); // PCI segment group
    t.push(bus_start); // start bus number
    t.push(bus_end); // end bus number
    t.extend_from_slice(&0u32.to_le_bytes()); // reserved

    finalize_sdt(&mut t);
    t
}

// ---------------------------------------------------------------------------
// DSDT / FACS
// ---------------------------------------------------------------------------

/// Build a minimal but valid DSDT: a standard SDT header followed by a trivial
/// AML body.
///
/// The AML body defines a single empty top-level scope `\_SB_` via a
/// zero-length `Scope` term, which is the smallest well-formed payload that a
/// guest AML interpreter will accept.
pub fn build_dsdt() -> Vec<u8> {
    let mut t = Vec::new();
    push_sdt_header(&mut t, b"DSDT", 2);

    // AML for: Scope (\_SB) {}
    //   0x10            ScopeOp
    //   0x05            PkgLength = 5 (covers length byte + 4 name bytes)
    //   0x5C 0x2F ...   actually use the simple rooted name form below.
    //
    // Encoding used here:
    //   ScopeOp (0x10)
    //   PkgLength (0x05) - one-byte form, value 5 = self + name (4 bytes)
    //   NameString: '\' (0x5C) "_SB_" -> but RootChar + NameSeg = 5 bytes,
    //   which would not fit in PkgLength 5. Use the un-rooted NameSeg form
    //   "_SB_" (4 bytes) so total body = 1 (pkglen) + 4 (name) = 5.
    let aml: [u8; 6] = [
        0x10, // ScopeOp
        0x05, // PkgLength (covers this byte + 4 name bytes)
        b'_', b'S', b'B', b'_',
    ];
    t.extend_from_slice(&aml);

    finalize_sdt(&mut t);
    t
}

/// Length of a FACS structure (revision 2 uses 64 bytes).
pub const FACS_LEN: usize = 64;

/// Build a Firmware ACPI Control Structure (FACS).
///
/// Note: the FACS is NOT a standard SDT — it has no SDT header and no overall
/// checksum field. It carries its own signature and length.
pub fn build_facs() -> Vec<u8> {
    let mut t = vec![0u8; FACS_LEN];
    t[0..4].copy_from_slice(b"FACS"); // signature
    t[4..8].copy_from_slice(&(FACS_LEN as u32).to_le_bytes()); // length
    // 8..12 hardware signature (0)
    // 12..16 firmware waking vector (0)
    // 16..20 global lock (0)
    // 20..24 flags (0)
    // 24..32 X firmware waking vector (0)
    t[32] = 2; // version
    // remainder reserved / zero
    t
}

// ---------------------------------------------------------------------------
// Orchestrator
// ---------------------------------------------------------------------------

/// Default ECAM base used when building the MCFG table (matches the common
/// QEMU "q35"-style 256MB window just below the 4GB MMIO hole).
pub const DEFAULT_ECAM_BASE: u64 = 0xB000_0000;
/// Last PCI bus exposed via ECAM.
pub const DEFAULT_PCI_BUS_END: u8 = 0xFF;

/// Align `addr` up to a 16-byte boundary (ACPI tables are conventionally
/// 16-byte aligned in firmware memory).
fn align16(addr: u64) -> u64 {
    (addr + 15) & !15
}

/// The full set of ACPI structures to place in guest memory.
pub struct AcpiTables {
    /// The RSDP blob and the guest-physical address it should be written to.
    pub rsdp: (u64, Vec<u8>),
    /// All other tables as `(guest_phys_addr, blob)` pairs (DSDT, FACS, FADT,
    /// MADT, HPET, MCFG, RSDT, XSDT).
    pub tables: Vec<(u64, Vec<u8>)>,
}

/// Build the complete ACPI table set, computing internal pointer addresses
/// relative to `base` (the guest-physical address at which the blob region
/// begins).
///
/// The layout places the RSDP first, then the child tables, then the RSDT and
/// XSDT (which reference the children). Returns the RSDP placement together
/// with the list of `(guest_phys_addr, blob)` pairs for every other table.
///
/// `cpu_count` controls the number of Processor Local APIC entries in the MADT.
/// `ecam_base` / `bus_end` parameterize the MCFG allocation.
pub fn build_acpi_tables(
    base: u64,
    cpu_count: u32,
    ecam_base: u64,
    bus_end: u8,
) -> (Vec<u8>, Vec<(u64, Vec<u8>)>) {
    // 1. Build the leaf tables whose addresses the FADT references.
    let facs = build_facs();
    let dsdt = build_dsdt();

    // Place RSDP first, then FACS, DSDT, then tables referenced by RSDT/XSDT.
    let mut cursor = align16(base);

    let rsdp_addr = cursor;
    cursor = align16(cursor + RSDP_LEN as u64);

    let facs_addr = cursor;
    cursor = align16(cursor + facs.len() as u64);

    let dsdt_addr = cursor;
    cursor = align16(cursor + dsdt.len() as u64);

    // 2. Build the FADT now that FACS/DSDT addresses are known.
    let fadt = build_fadt(facs_addr as u32, dsdt_addr as u32);
    let fadt_addr = cursor;
    cursor = align16(cursor + fadt.len() as u64);

    let madt = build_madt(cpu_count);
    let madt_addr = cursor;
    cursor = align16(cursor + madt.len() as u64);

    let hpet = build_hpet();
    let hpet_addr = cursor;
    cursor = align16(cursor + hpet.len() as u64);

    let mcfg = build_mcfg(ecam_base, 0, bus_end);
    let mcfg_addr = cursor;
    cursor = align16(cursor + mcfg.len() as u64);

    // 3. Build RSDT / XSDT referencing the child tables (FADT, MADT, HPET, MCFG).
    let child_addrs_32: Vec<u32> = vec![
        fadt_addr as u32,
        madt_addr as u32,
        hpet_addr as u32,
        mcfg_addr as u32,
    ];
    let child_addrs_64: Vec<u64> = vec![fadt_addr, madt_addr, hpet_addr, mcfg_addr];

    let rsdt = build_rsdt(&child_addrs_32);
    let rsdt_addr = cursor;
    cursor = align16(cursor + rsdt.len() as u64);

    let xsdt = build_xsdt(&child_addrs_64);
    let xsdt_addr = cursor;

    // 4. Build the RSDP referencing RSDT (32-bit) and XSDT (64-bit).
    let rsdp = build_rsdp(rsdt_addr as u32, xsdt_addr);

    let tables = vec![
        (facs_addr, facs),
        (dsdt_addr, dsdt),
        (fadt_addr, fadt),
        (madt_addr, madt),
        (hpet_addr, hpet),
        (mcfg_addr, mcfg),
        (rsdt_addr, rsdt),
        (xsdt_addr, xsdt),
    ];

    let _ = rsdp_addr; // RSDP placement is returned via the first element of the pair
    (rsdp, tables)
}

/// Convenience wrapper returning a structured [`AcpiTables`] with the RSDP
/// placed at `base`.
pub fn build_acpi_tables_struct(base: u64, cpu_count: u32) -> AcpiTables {
    let rsdp_addr = align16(base);
    let (rsdp, tables) = build_acpi_tables(base, cpu_count, DEFAULT_ECAM_BASE, DEFAULT_PCI_BUS_END);
    AcpiTables {
        rsdp: (rsdp_addr, rsdp),
        tables,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Sum of all bytes in `t`, modulo 256.
    fn byte_sum(t: &[u8]) -> u8 {
        t.iter().fold(0u8, |a, &b| a.wrapping_add(b))
    }

    /// Helper: read a little-endian u32 from `t` at `off`.
    fn rd_u32(t: &[u8], off: usize) -> u32 {
        u32::from_le_bytes(t[off..off + 4].try_into().unwrap())
    }

    fn rd_u64(t: &[u8], off: usize) -> u64 {
        u64::from_le_bytes(t[off..off + 8].try_into().unwrap())
    }

    #[test]
    fn sdt_header_checksums_sum_to_zero() {
        // Every SDT-style table must have a full-table checksum of zero.
        let tables: Vec<(&str, Vec<u8>)> = vec![
            ("RSDT", build_rsdt(&[0x1000, 0x2000])),
            ("XSDT", build_xsdt(&[0x1000, 0x2000])),
            ("FACP", build_fadt(0x1000, 0x2000)),
            ("APIC", build_madt(4)),
            ("HPET", build_hpet()),
            ("MCFG", build_mcfg(0xB000_0000, 0, 0xFF)),
            ("DSDT", build_dsdt()),
        ];
        for (name, t) in tables {
            assert_eq!(byte_sum(&t), 0, "{name} checksum does not sum to 0");
        }
    }

    #[test]
    fn signatures_are_correct() {
        assert_eq!(&build_rsdt(&[])[0..4], b"RSDT");
        assert_eq!(&build_xsdt(&[])[0..4], b"XSDT");
        assert_eq!(&build_fadt(0x1000, 0x2000)[0..4], b"FACP");
        assert_eq!(&build_madt(1)[0..4], b"APIC");
        assert_eq!(&build_hpet()[0..4], b"HPET");
        assert_eq!(&build_mcfg(0xB000_0000, 0, 0xFF)[0..4], b"MCFG");
        assert_eq!(&build_dsdt()[0..4], b"DSDT");
        assert_eq!(&build_facs()[0..4], b"FACS");
    }

    #[test]
    fn rsdp_signature_and_checksums() {
        let rsdp = build_rsdp(0x1234, 0x5678_9ABC);
        assert_eq!(&rsdp[0..8], b"RSD PTR ");
        assert_eq!(rsdp.len(), RSDP_LEN);
        assert_eq!(rsdp[15], 2, "RSDP revision must be 2 (ACPI 2.0+)");

        // ACPI 1.0 checksum: first 20 bytes sum to 0.
        assert_eq!(byte_sum(&rsdp[0..20]), 0, "RSDP v1 checksum invalid");
        // Extended checksum: whole structure sums to 0.
        assert_eq!(byte_sum(&rsdp), 0, "RSDP extended checksum invalid");

        // Addresses are stored correctly.
        assert_eq!(rd_u32(&rsdp, 16), 0x1234);
        assert_eq!(rd_u64(&rsdp, 24), 0x5678_9ABC);
    }

    #[test]
    fn madt_has_expected_entries() {
        let cpu_count = 8u32;
        let madt = build_madt(cpu_count);

        assert_eq!(
            madt_count_local_apics(&madt),
            cpu_count as usize,
            "MADT must contain one LAPIC per CPU"
        );
        assert_eq!(
            madt_count_io_apics(&madt),
            1,
            "MADT must contain exactly one I/O APIC entry"
        );

        // Local APIC address and flags.
        assert_eq!(rd_u32(&madt, 36), LOCAL_APIC_BASE);
        assert_eq!(rd_u32(&madt, 40), MADT_PCAT_COMPAT);

        // Two interrupt source overrides expected.
        assert_eq!(count_madt_entries(&madt, MADT_TYPE_INT_SRC_OVERRIDE), 2);
    }

    #[test]
    fn madt_io_apic_address_is_correct() {
        let madt = build_madt(2);
        // Walk to the IO APIC entry and verify its address field.
        let mut off = 44;
        let mut found = false;
        while off + 2 <= madt.len() {
            let etype = madt[off];
            let elen = madt[off + 1] as usize;
            if etype == MADT_TYPE_IO_APIC {
                // type(1) len(1) id(1) reserved(1) address(4) gsi(4)
                assert_eq!(rd_u32(&madt, off + 4), IO_APIC_BASE);
                assert_eq!(rd_u32(&madt, off + 8), 0, "IO APIC GSI base must be 0");
                found = true;
            }
            off += elen;
        }
        assert!(found, "no IO APIC entry found");
    }

    #[test]
    fn xsdt_length_matches_entry_count() {
        for n in [0usize, 1, 4] {
            let entries: Vec<u64> = (0..n as u64).map(|i| 0x1000 + i * 0x100).collect();
            let xsdt = build_xsdt(&entries);
            let len = rd_u32(&xsdt, 4) as usize;
            assert_eq!(len, xsdt.len(), "XSDT length field must match blob size");
            assert_eq!(
                len,
                SDT_HEADER_LEN + n * 8,
                "XSDT length must be header + n*8"
            );
        }
    }

    #[test]
    fn rsdt_length_matches_entry_count() {
        for n in [0usize, 1, 4] {
            let entries: Vec<u32> = (0..n as u32).map(|i| 0x1000 + i * 0x100).collect();
            let rsdt = build_rsdt(&entries);
            let len = rd_u32(&rsdt, 4) as usize;
            assert_eq!(len, rsdt.len());
            assert_eq!(len, SDT_HEADER_LEN + n * 4);
        }
    }

    #[test]
    fn fadt_references_facs_and_dsdt() {
        let fadt = build_fadt(0xAABB, 0xCCDD);
        // FIRMWARE_CTRL at offset 36, DSDT at offset 40.
        assert_eq!(rd_u32(&fadt, 36), 0xAABB);
        assert_eq!(rd_u32(&fadt, 40), 0xCCDD);
        // X_FIRMWARE_CTRL at 132, X_DSDT at 140.
        assert_eq!(rd_u64(&fadt, 132), 0xAABB);
        assert_eq!(rd_u64(&fadt, 140), 0xCCDD);
        // Length field is consistent.
        assert_eq!(rd_u32(&fadt, 4) as usize, fadt.len());
    }

    #[test]
    fn facs_has_no_sdt_checksum_but_valid_length() {
        let facs = build_facs();
        assert_eq!(facs.len(), FACS_LEN);
        assert_eq!(rd_u32(&facs, 4) as usize, FACS_LEN);
        assert_eq!(facs[32], 2, "FACS version should be 2");
    }

    #[test]
    fn hpet_fields_are_correct() {
        let hpet = build_hpet();
        // Base address GAS starts at offset 40 (36 header + 4 event timer id).
        // GAS: space_id(1) bit_width(1) bit_offset(1) access(1) address(8)
        assert_eq!(hpet[40], 0, "HPET base must be system memory space");
        assert_eq!(rd_u64(&hpet, 44), HPET_BASE);
    }

    #[test]
    fn mcfg_allocation_entry_is_correct() {
        let mcfg = build_mcfg(0xB000_0000, 0, 0xFF);
        // After 36-byte header + 8-byte reserved => allocation at offset 44.
        assert_eq!(rd_u64(&mcfg, 44), 0xB000_0000);
        assert_eq!(
            u16::from_le_bytes(mcfg[52..54].try_into().unwrap()),
            0,
            "PCI segment group must be 0"
        );
        assert_eq!(mcfg[54], 0, "bus start");
        assert_eq!(mcfg[55], 0xFF, "bus end");
    }

    #[test]
    fn dsdt_has_valid_header_and_body() {
        let dsdt = build_dsdt();
        assert!(dsdt.len() > SDT_HEADER_LEN, "DSDT must have an AML body");
        assert_eq!(rd_u32(&dsdt, 4) as usize, dsdt.len());
        assert_eq!(byte_sum(&dsdt), 0);
    }

    #[test]
    fn orchestrator_places_tables_and_pointers_resolve() {
        let base = 0x000F_0000u64;
        let cpu_count = 4u32;
        let (rsdp, tables) =
            build_acpi_tables(base, cpu_count, DEFAULT_ECAM_BASE, DEFAULT_PCI_BUS_END);

        // RSDP is valid.
        assert_eq!(&rsdp[0..8], b"RSD PTR ");
        assert_eq!(byte_sum(&rsdp), 0);

        // Build an address -> blob lookup.
        let map: std::collections::HashMap<u64, &Vec<u8>> =
            tables.iter().map(|(a, b)| (*a, b)).collect();

        // The RSDP's RSDT pointer must resolve to an "RSDT" table.
        let rsdt_addr = rd_u32(&rsdp, 16) as u64;
        let rsdt = map.get(&rsdt_addr).expect("RSDT address must resolve");
        assert_eq!(&rsdt[0..4], b"RSDT");

        // The RSDP's XSDT pointer must resolve to an "XSDT" table.
        let xsdt_addr = rd_u64(&rsdp, 24);
        let xsdt = map.get(&xsdt_addr).expect("XSDT address must resolve");
        assert_eq!(&xsdt[0..4], b"XSDT");

        // Every XSDT child pointer must resolve to a real table whose checksum
        // sums to zero.
        let entry_count = (xsdt.len() - SDT_HEADER_LEN) / 8;
        assert_eq!(entry_count, 4, "XSDT should reference FADT/MADT/HPET/MCFG");
        let mut sigs = Vec::new();
        for i in 0..entry_count {
            let ptr = rd_u64(xsdt, SDT_HEADER_LEN + i * 8);
            let child = map.get(&ptr).expect("XSDT child must resolve");
            assert_eq!(byte_sum(child), 0, "child checksum must be 0");
            sigs.push(std::str::from_utf8(&child[0..4]).unwrap().to_string());
        }
        assert!(sigs.contains(&"FACP".to_string()));
        assert!(sigs.contains(&"APIC".to_string()));
        assert!(sigs.contains(&"HPET".to_string()));
        assert!(sigs.contains(&"MCFG".to_string()));

        // The FADT's FACS/DSDT pointers must resolve.
        let fadt = map
            .values()
            .find(|t| &t[0..4] == b"FACP")
            .expect("FADT present");
        let facs_addr = rd_u32(fadt, 36) as u64;
        let dsdt_addr = rd_u32(fadt, 40) as u64;
        assert_eq!(&map.get(&facs_addr).expect("FACS resolves")[0..4], b"FACS");
        assert_eq!(&map.get(&dsdt_addr).expect("DSDT resolves")[0..4], b"DSDT");

        // The MADT must contain cpu_count LAPICs.
        let madt = map
            .values()
            .find(|t| &t[0..4] == b"APIC")
            .expect("MADT present");
        assert_eq!(madt_count_local_apics(madt), cpu_count as usize);
    }

    #[test]
    fn orchestrator_struct_wrapper_is_consistent() {
        let base = 0x0010_0000u64;
        let at = build_acpi_tables_struct(base, 2);
        assert_eq!(&at.rsdp.1[0..8], b"RSD PTR ");
        // RSDP placed at the (16-byte aligned) base.
        assert_eq!(at.rsdp.0, base);
        assert!(!at.tables.is_empty());
    }
}
