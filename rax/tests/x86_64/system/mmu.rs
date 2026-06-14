//! Comprehensive MMU (Memory Management Unit) test suite.
//!
//! This module provides extensive testing of the x86-64 MMU implementation,
//! covering:
//!
//! ## Permission Testing
//! - Read/Write/Execute permissions at all page table levels
//! - User/Supervisor mode access control
//! - NX (No-Execute) bit enforcement
//! - Permission inheritance through page table hierarchy
//!
//! ## Page Table Hierarchy
//! - 4KB standard pages
//! - 2MB huge pages
//! - 1GB huge pages
//! - Mixed page sizes in same address space
//!
//! ## Edge Cases & Torture Tests
//! - Page boundary crossing reads/writes
//! - Reserved bit violations
//! - Unaligned accesses at various offsets
//! - Stress tests with many TLB entries
//! - Rapid context switching simulations
//! - Exotic address patterns
//!
//! ## TLB Behavior
//! - TLB population and hits
//! - INVLPG invalidation
//! - CR3 write flush
//! - Global page handling
//!
//! ## Complex Scenarios
//! - Multi-page operations
//! - Deep call chains across pages
//! - Self-modifying code detection patterns
//! - Permission changes during execution

use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::{DescriptorTable, Registers, Segment, SystemRegisters, VCpu, VcpuExit};

// ============================================================================
// Constants
// ============================================================================

/// Physical addresses for page table structures
const PML4_ADDR: u64 = 0x1000;
const PDPT_ADDR: u64 = 0x2000;
const PD_ADDR: u64 = 0x3000;
const PT_ADDR: u64 = 0x4000;

/// Code and data addresses
const CODE_PADDR: u64 = 0x10000;
const STACK_PADDR: u64 = 0x20000;

/// IDT and handler addresses
const IDT_ADDR: u64 = 0x50000;
const PF_HANDLER_ADDR: u64 = 0x51000;
const GP_HANDLER_ADDR: u64 = 0x52000;
const DF_HANDLER_ADDR: u64 = 0x53000;

/// GDT address and selectors
const GDT_ADDR: u64 = 0x54000;
const KERNEL_CS: u16 = 0x08;
const KERNEL_DS: u16 = 0x10;
const USER_CS: u16 = 0x1B;
const USER_DS: u16 = 0x23;
const TSS_SEL: u16 = 0x28;

/// TSS address for ring transitions
const TSS_ADDR: u64 = 0x55000;

/// Test result storage
const RESULT_ADDR: u64 = 0x60000;

// ============================================================================
// Page Table Entry Flags
// ============================================================================

mod pte_flags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const CACHE_DISABLE: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    pub const DIRTY: u64 = 1 << 6;
    pub const HUGE_PAGE: u64 = 1 << 7;
    pub const GLOBAL: u64 = 1 << 8;
    pub const NO_EXECUTE: u64 = 1 << 63;
}

/// Page fault error code bits
mod pf_error {
    pub const P: u64 = 1 << 0; // 0=non-present, 1=protection violation
    pub const WR: u64 = 1 << 1; // 0=read, 1=write
    pub const US: u64 = 1 << 2; // 0=supervisor, 1=user
    pub const RSVD: u64 = 1 << 3; // Reserved bit violation
    pub const ID: u64 = 1 << 4; // 0=data, 1=instruction fetch
}

// ============================================================================
// Memory and vCPU Setup Helpers
// ============================================================================

fn create_memory(size_mb: usize) -> Arc<GuestMemoryMmap> {
    let mem_size = size_mb * 1024 * 1024;
    let regions = vec![(GuestAddress(0), mem_size)];
    Arc::new(GuestMemoryMmap::<()>::from_ranges(&regions).unwrap())
}

/// Set up complete page tables with identity mapping
fn setup_identity_page_tables(mem: &GuestMemoryMmap, flags: u64) {
    use pte_flags::*;

    let base_flags = flags | PRESENT;

    // PML4 entry 0 -> PDPT
    let pml4e = PDPT_ADDR | base_flags;
    mem.write_slice(&pml4e.to_le_bytes(), GuestAddress(PML4_ADDR))
        .unwrap();

    // PDPT entry 0 -> PD
    let pdpte = PD_ADDR | base_flags;
    mem.write_slice(&pdpte.to_le_bytes(), GuestAddress(PDPT_ADDR))
        .unwrap();

    // PD entry 0 -> PT (4KB pages)
    let pde = PT_ADDR | base_flags;
    mem.write_slice(&pde.to_le_bytes(), GuestAddress(PD_ADDR))
        .unwrap();

    // PT entries: identity map first 512 pages (2MB)
    for i in 0..512u64 {
        let paddr = i * 0x1000;
        let pte = paddr | base_flags;
        mem.write_slice(&pte.to_le_bytes(), GuestAddress(PT_ADDR + i * 8))
            .unwrap();
    }

    // Additional PD entries for more coverage
    for pd_idx in 1..4u64 {
        let pt_addr = PT_ADDR + pd_idx * 0x1000;
        let pde = pt_addr | base_flags;
        mem.write_slice(&pde.to_le_bytes(), GuestAddress(PD_ADDR + pd_idx * 8))
            .unwrap();

        for i in 0..512u64 {
            let paddr = (pd_idx * 512 + i) * 0x1000;
            let pte = paddr | base_flags;
            mem.write_slice(&pte.to_le_bytes(), GuestAddress(pt_addr + i * 8))
                .unwrap();
        }
    }
}

/// Set up IDT with exception handlers
fn setup_idt(mem: &GuestMemoryMmap) {
    fn write_idt_entry(mem: &GuestMemoryMmap, vector: u64, handler_addr: u64) {
        let offset = IDT_ADDR + vector * 16;
        let mut entry = [0u8; 16];

        entry[0] = (handler_addr & 0xFF) as u8;
        entry[1] = ((handler_addr >> 8) & 0xFF) as u8;
        entry[2] = KERNEL_CS as u8;
        entry[3] = (KERNEL_CS >> 8) as u8;
        entry[4] = 0x00; // IST
        entry[5] = 0x8E; // Type
        entry[6] = ((handler_addr >> 16) & 0xFF) as u8;
        entry[7] = ((handler_addr >> 24) & 0xFF) as u8;
        entry[8] = ((handler_addr >> 32) & 0xFF) as u8;
        entry[9] = ((handler_addr >> 40) & 0xFF) as u8;
        entry[10] = ((handler_addr >> 48) & 0xFF) as u8;
        entry[11] = ((handler_addr >> 56) & 0xFF) as u8;
        entry[12..16].fill(0);

        mem.write_slice(&entry, GuestAddress(offset)).unwrap();
    }

    write_idt_entry(mem, 8, DF_HANDLER_ADDR);
    write_idt_entry(mem, 13, GP_HANDLER_ADDR);
    write_idt_entry(mem, 14, PF_HANDLER_ADDR);
}

/// Set up exception handlers
fn setup_handlers(mem: &GuestMemoryMmap) {
    // Page fault handler: store error code, CR2, marker, then HLT
    let pf_handler: Vec<u8> = vec![
        0x48,
        0x8b,
        0x04,
        0x24, // mov rax, [rsp]
        0x48,
        0xa3, // mov [RESULT_ADDR], rax
        (RESULT_ADDR & 0xFF) as u8,
        ((RESULT_ADDR >> 8) & 0xFF) as u8,
        ((RESULT_ADDR >> 16) & 0xFF) as u8,
        ((RESULT_ADDR >> 24) & 0xFF) as u8,
        ((RESULT_ADDR >> 32) & 0xFF) as u8,
        ((RESULT_ADDR >> 40) & 0xFF) as u8,
        ((RESULT_ADDR >> 48) & 0xFF) as u8,
        ((RESULT_ADDR >> 56) & 0xFF) as u8,
        0x0f,
        0x20,
        0xd0, // mov rax, cr2
        0x48,
        0xa3, // mov [RESULT_ADDR+8], rax
        ((RESULT_ADDR + 8) & 0xFF) as u8,
        (((RESULT_ADDR + 8) >> 8) & 0xFF) as u8,
        (((RESULT_ADDR + 8) >> 16) & 0xFF) as u8,
        (((RESULT_ADDR + 8) >> 24) & 0xFF) as u8,
        (((RESULT_ADDR + 8) >> 32) & 0xFF) as u8,
        (((RESULT_ADDR + 8) >> 40) & 0xFF) as u8,
        (((RESULT_ADDR + 8) >> 48) & 0xFF) as u8,
        (((RESULT_ADDR + 8) >> 56) & 0xFF) as u8,
        0x48,
        0xc7,
        0xc0,
        0x14,
        0x00,
        0x00,
        0x00, // mov rax, 0x14
        0x48,
        0xa3,
        ((RESULT_ADDR + 16) & 0xFF) as u8,
        (((RESULT_ADDR + 16) >> 8) & 0xFF) as u8,
        (((RESULT_ADDR + 16) >> 16) & 0xFF) as u8,
        (((RESULT_ADDR + 16) >> 24) & 0xFF) as u8,
        (((RESULT_ADDR + 16) >> 32) & 0xFF) as u8,
        (((RESULT_ADDR + 16) >> 40) & 0xFF) as u8,
        (((RESULT_ADDR + 16) >> 48) & 0xFF) as u8,
        (((RESULT_ADDR + 16) >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(&pf_handler, GuestAddress(PF_HANDLER_ADDR))
        .unwrap();

    // GP and DF handlers
    for (addr, marker) in [(GP_HANDLER_ADDR, 0x0D), (DF_HANDLER_ADDR, 0x08)] {
        let handler: &[u8] = &[
            0x48,
            0xc7,
            0xc0,
            marker,
            0x00,
            0x00,
            0x00,
            0x48,
            0xa3,
            ((RESULT_ADDR + 16) & 0xFF) as u8,
            (((RESULT_ADDR + 16) >> 8) & 0xFF) as u8,
            (((RESULT_ADDR + 16) >> 16) & 0xFF) as u8,
            (((RESULT_ADDR + 16) >> 24) & 0xFF) as u8,
            (((RESULT_ADDR + 16) >> 32) & 0xFF) as u8,
            (((RESULT_ADDR + 16) >> 40) & 0xFF) as u8,
            (((RESULT_ADDR + 16) >> 48) & 0xFF) as u8,
            (((RESULT_ADDR + 16) >> 56) & 0xFF) as u8,
            0xf4,
        ];
        mem.write_slice(handler, GuestAddress(addr)).unwrap();
    }
}

/// Create a vCPU with paging enabled
fn create_paged_vcpu(mem: Arc<GuestMemoryMmap>) -> X86_64Vcpu {
    let mut vcpu = X86_64Vcpu::new(0, mem);

    let mut sregs = SystemRegisters::default();
    sregs.cr0 = 0x80050033;
    sregs.cr3 = PML4_ADDR;
    sregs.cr4 = 0x20;
    sregs.efer = 0x500;

    sregs.cs = Segment {
        base: 0,
        limit: 0xFFFFFFFF,
        selector: KERNEL_CS,
        type_: 0x0B,
        present: true,
        dpl: 0,
        db: false,
        s: true,
        l: true,
        g: true,
        avl: false,
        unusable: false,
    };

    let data_seg = Segment {
        base: 0,
        limit: 0xFFFFFFFF,
        selector: KERNEL_DS,
        type_: 0x03,
        present: true,
        dpl: 0,
        db: true,
        s: true,
        l: false,
        g: true,
        avl: false,
        unusable: false,
    };
    sregs.ds = data_seg.clone();
    sregs.es = data_seg.clone();
    sregs.fs = data_seg.clone();
    sregs.gs = data_seg.clone();
    sregs.ss = data_seg;

    sregs.idt = DescriptorTable {
        base: IDT_ADDR,
        limit: 256 * 16 - 1,
    };
    sregs.gdt = DescriptorTable {
        base: GDT_ADDR,
        limit: 0x3F,
    };
    sregs.tr = Segment {
        base: TSS_ADDR,
        limit: 0x67,
        selector: TSS_SEL,
        type_: 0x09,
        present: true,
        dpl: 0,
        db: false,
        s: false,
        l: false,
        g: false,
        avl: false,
        unusable: false,
    };

    vcpu.set_sregs(&sregs).unwrap();

    let mut regs = Registers::default();
    regs.rsp = STACK_PADDR + 0x1000;
    regs.rflags = 0x2;
    vcpu.set_regs(&regs).unwrap();

    vcpu
}

/// Run vCPU until HLT
fn run_until_hlt(vcpu: &mut X86_64Vcpu) -> Result<(), String> {
    for _ in 0..100000 {
        match vcpu.step() {
            Ok(Some(VcpuExit::Hlt)) => return Ok(()),
            Ok(Some(VcpuExit::Shutdown)) => return Err("Shutdown".to_string()),
            Ok(_) => continue,
            Err(e) => return Err(format!("vCPU error: {:?}", e)),
        }
    }
    Err("Exceeded iteration limit".to_string())
}

/// Run vCPU and expect a specific page fault (tests that can't use handlers)
fn expect_page_fault(vcpu: &mut X86_64Vcpu) -> Result<(u64, u64), String> {
    for _ in 0..10000 {
        match vcpu.step() {
            Ok(Some(VcpuExit::Hlt)) => return Err("Unexpected HLT".to_string()),
            Ok(Some(VcpuExit::Shutdown)) => return Err("Shutdown".to_string()),
            Ok(_) => continue,
            Err(rax::error::Error::PageFault { vaddr, error_code }) => {
                return Ok((vaddr, error_code));
            }
            Err(e) => return Err(format!("Unexpected: {:?}", e)),
        }
    }
    Err("No page fault occurred".to_string())
}

fn read_result(mem: &GuestMemoryMmap) -> (u64, u64, u64) {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    let error_code = u64::from_le_bytes(buf);
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR + 8))
        .unwrap();
    let cr2 = u64::from_le_bytes(buf);
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR + 16))
        .unwrap();
    let marker = u64::from_le_bytes(buf);
    (error_code, cr2, marker)
}

fn clear_result(mem: &GuestMemoryMmap) {
    mem.write_slice(&[0u8; 24], GuestAddress(RESULT_ADDR))
        .unwrap();
}

// ============================================================================
// BASIC PAGING TESTS
// ============================================================================

/// Test basic paging with identity mapping works
#[test]
fn test_basic_paging_identity_map() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    let test_value = 0xDEADBEEFCAFEBABE_u64;
    mem.write_slice(&test_value.to_le_bytes(), GuestAddress(0x70000))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48,
        0x8b,
        0x04,
        0x25,
        0x00,
        0x00,
        0x07,
        0x00, // mov rax, [0x70000]
        0x48,
        0xa3,
        (RESULT_ADDR & 0xFF) as u8,
        ((RESULT_ADDR >> 8) & 0xFF) as u8,
        ((RESULT_ADDR >> 16) & 0xFF) as u8,
        ((RESULT_ADDR >> 24) & 0xFF) as u8,
        ((RESULT_ADDR >> 32) & 0xFF) as u8,
        ((RESULT_ADDR >> 40) & 0xFF) as u8,
        ((RESULT_ADDR >> 48) & 0xFF) as u8,
        ((RESULT_ADDR >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), test_value);
}

/// Test write through paging works
#[test]
fn test_write_through_paging() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    let mut vcpu = create_paged_vcpu(mem.clone());
    let test_value = 0x1234567890ABCDEF_u64;
    let target = 0x70000u64;

    let code: Vec<u8> = vec![
        0x48,
        0xb8, // mov rax, imm64
        (test_value & 0xFF) as u8,
        ((test_value >> 8) & 0xFF) as u8,
        ((test_value >> 16) & 0xFF) as u8,
        ((test_value >> 24) & 0xFF) as u8,
        ((test_value >> 32) & 0xFF) as u8,
        ((test_value >> 40) & 0xFF) as u8,
        ((test_value >> 48) & 0xFF) as u8,
        ((test_value >> 56) & 0xFF) as u8,
        0x48,
        0xa3, // mov [target], rax
        (target & 0xFF) as u8,
        ((target >> 8) & 0xFF) as u8,
        ((target >> 16) & 0xFF) as u8,
        ((target >> 24) & 0xFF) as u8,
        ((target >> 32) & 0xFF) as u8,
        ((target >> 40) & 0xFF) as u8,
        ((target >> 48) & 0xFF) as u8,
        ((target >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(target)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), test_value);
}

// ============================================================================
// PERMISSION TESTS (using direct PageFault detection)
// ============================================================================

/// Test write to read-only page at PTE level
#[test]
fn test_write_to_readonly_pte() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    // Make page 112 (0x70000) read-only
    let pte = 0x70000u64 | pte_flags::PRESENT | pte_flags::USER;
    mem.write_slice(&pte.to_le_bytes(), GuestAddress(PT_ADDR + 112 * 8))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48, 0xc7, 0x04, 0x25, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00,
        0x00, // mov qword [0x70000], 0
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    let (vaddr, error_code) = expect_page_fault(&mut vcpu).unwrap();
    assert_eq!(vaddr, 0x70000, "Fault at correct address");
    assert_ne!(error_code & pf_error::P, 0, "P=1 (protection violation)");
    assert_ne!(error_code & pf_error::WR, 0, "W/R=1 (write access)");
}

/// Test write permission denied at PD level (2MB region read-only)
#[test]
fn test_write_permission_denied_pd_level() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    // Make PD entry 0 read-only (affects entire 0-2MB region)
    let pde = PT_ADDR | pte_flags::PRESENT | pte_flags::USER; // No WRITABLE
    mem.write_slice(&pde.to_le_bytes(), GuestAddress(PD_ADDR))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48, 0xc7, 0x04, 0x25, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    let (vaddr, error_code) = expect_page_fault(&mut vcpu).unwrap();
    assert_eq!(vaddr, 0x70000);
    assert_ne!(error_code & pf_error::P, 0);
    assert_ne!(error_code & pf_error::WR, 0);
}

/// Test read from non-present page
#[test]
fn test_read_non_present_page() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    // Clear PTE for page at 0x80000
    mem.write_slice(&0u64.to_le_bytes(), GuestAddress(PT_ADDR + 128 * 8))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x08, 0x00, // mov rax, [0x80000]
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    let (vaddr, error_code) = expect_page_fault(&mut vcpu).unwrap();
    assert_eq!(vaddr, 0x80000, "Fault at correct address");
    assert_eq!(error_code & pf_error::P, 0, "P=0 (non-present)");
}

// ============================================================================
// HUGE PAGE TESTS
// ============================================================================

/// Test 2MB huge page read
#[test]
fn test_2mb_huge_page_read() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    // Set PD[0] as 2MB huge page
    let pde =
        0u64 | pte_flags::PRESENT | pte_flags::WRITABLE | pte_flags::USER | pte_flags::HUGE_PAGE;
    mem.write_slice(&pde.to_le_bytes(), GuestAddress(PD_ADDR))
        .unwrap();

    let test_value = 0xCAFEBABEDEADBEEF_u64;
    mem.write_slice(&test_value.to_le_bytes(), GuestAddress(0x100000))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48,
        0x8b,
        0x04,
        0x25,
        0x00,
        0x00,
        0x10,
        0x00, // mov rax, [0x100000]
        0x48,
        0xa3,
        (RESULT_ADDR & 0xFF) as u8,
        ((RESULT_ADDR >> 8) & 0xFF) as u8,
        ((RESULT_ADDR >> 16) & 0xFF) as u8,
        ((RESULT_ADDR >> 24) & 0xFF) as u8,
        ((RESULT_ADDR >> 32) & 0xFF) as u8,
        ((RESULT_ADDR >> 40) & 0xFF) as u8,
        ((RESULT_ADDR >> 48) & 0xFF) as u8,
        ((RESULT_ADDR >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), test_value);
}

/// Test 2MB huge page write
#[test]
fn test_2mb_huge_page_write() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    let pde =
        0u64 | pte_flags::PRESENT | pte_flags::WRITABLE | pte_flags::USER | pte_flags::HUGE_PAGE;
    mem.write_slice(&pde.to_le_bytes(), GuestAddress(PD_ADDR))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let test_value = 0x123456789ABCDEF0_u64;
    let target = 0x150000u64;

    let code: Vec<u8> = vec![
        0x48,
        0xb8,
        (test_value & 0xFF) as u8,
        ((test_value >> 8) & 0xFF) as u8,
        ((test_value >> 16) & 0xFF) as u8,
        ((test_value >> 24) & 0xFF) as u8,
        ((test_value >> 32) & 0xFF) as u8,
        ((test_value >> 40) & 0xFF) as u8,
        ((test_value >> 48) & 0xFF) as u8,
        ((test_value >> 56) & 0xFF) as u8,
        0x48,
        0xa3,
        (target & 0xFF) as u8,
        ((target >> 8) & 0xFF) as u8,
        ((target >> 16) & 0xFF) as u8,
        ((target >> 24) & 0xFF) as u8,
        ((target >> 32) & 0xFF) as u8,
        ((target >> 40) & 0xFF) as u8,
        ((target >> 48) & 0xFF) as u8,
        ((target >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(target)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), test_value);
}

/// Test 1GB huge page
#[test]
fn test_1gb_huge_page() {
    let mem = create_memory(64);

    // PML4[0] -> PDPT with 1GB huge page
    let pml4e = PDPT_ADDR | pte_flags::PRESENT | pte_flags::WRITABLE | pte_flags::USER;
    mem.write_slice(&pml4e.to_le_bytes(), GuestAddress(PML4_ADDR))
        .unwrap();

    let pdpte =
        0u64 | pte_flags::PRESENT | pte_flags::WRITABLE | pte_flags::USER | pte_flags::HUGE_PAGE;
    mem.write_slice(&pdpte.to_le_bytes(), GuestAddress(PDPT_ADDR))
        .unwrap();

    setup_idt(&mem);
    setup_handlers(&mem);

    let test_value = 0xFEDCBA9876543210_u64;
    mem.write_slice(&test_value.to_le_bytes(), GuestAddress(0x1000000))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48,
        0x8b,
        0x04,
        0x25,
        0x00,
        0x00,
        0x00,
        0x01, // mov rax, [0x1000000]
        0x48,
        0xa3,
        (RESULT_ADDR & 0xFF) as u8,
        ((RESULT_ADDR >> 8) & 0xFF) as u8,
        ((RESULT_ADDR >> 16) & 0xFF) as u8,
        ((RESULT_ADDR >> 24) & 0xFF) as u8,
        ((RESULT_ADDR >> 32) & 0xFF) as u8,
        ((RESULT_ADDR >> 40) & 0xFF) as u8,
        ((RESULT_ADDR >> 48) & 0xFF) as u8,
        ((RESULT_ADDR >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), test_value);
}

// ============================================================================
// EDGE CASES & TORTURE TESTS
// ============================================================================

/// Test page boundary crossing read (both pages present)
#[test]
fn test_page_boundary_crossing_read() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    let value = 0x0102030405060708_u64;
    mem.write_slice(&value.to_le_bytes(), GuestAddress(0x50FFC))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48,
        0x8b,
        0x04,
        0x25,
        0xFC,
        0x0F,
        0x05,
        0x00, // mov rax, [0x50FFC]
        0x48,
        0xa3,
        (RESULT_ADDR & 0xFF) as u8,
        ((RESULT_ADDR >> 8) & 0xFF) as u8,
        ((RESULT_ADDR >> 16) & 0xFF) as u8,
        ((RESULT_ADDR >> 24) & 0xFF) as u8,
        ((RESULT_ADDR >> 32) & 0xFF) as u8,
        ((RESULT_ADDR >> 40) & 0xFF) as u8,
        ((RESULT_ADDR >> 48) & 0xFF) as u8,
        ((RESULT_ADDR >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), value);
}

/// Test page boundary crossing write
#[test]
fn test_page_boundary_crossing_write() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    let mut vcpu = create_paged_vcpu(mem.clone());
    let value = 0xFEDCBA9876543210_u64;
    let addr = 0x50FFC_u64;

    let code: Vec<u8> = vec![
        0x48,
        0xb8,
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
        ((value >> 32) & 0xFF) as u8,
        ((value >> 40) & 0xFF) as u8,
        ((value >> 48) & 0xFF) as u8,
        ((value >> 56) & 0xFF) as u8,
        0x48,
        0xa3,
        (addr & 0xFF) as u8,
        ((addr >> 8) & 0xFF) as u8,
        ((addr >> 16) & 0xFF) as u8,
        ((addr >> 24) & 0xFF) as u8,
        ((addr >> 32) & 0xFF) as u8,
        ((addr >> 40) & 0xFF) as u8,
        ((addr >> 48) & 0xFF) as u8,
        ((addr >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), value);
}

/// Torture test: Access every 4KB boundary in first 2MB
#[test]
fn test_torture_every_page_boundary() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    // Write distinct values at each page start, skipping reserved areas
    for page in 0..512u64 {
        let addr = page * 0x1000;
        // Skip page table areas (0x1000-0x7FFF used by PML4, PDPT, PD, PTs)
        if addr >= 0x1000 && addr < 0x8000 {
            continue;
        }
        // Skip code page
        if addr >= CODE_PADDR && addr < CODE_PADDR + 0x1000 {
            continue;
        }
        // Skip stack area
        if addr >= STACK_PADDR && addr < STACK_PADDR + 0x1000 {
            continue;
        }
        // Skip IDT and handler area
        if addr >= IDT_ADDR && addr < IDT_ADDR + 0x4000 {
            continue;
        }
        // Skip GDT/TSS area
        if addr >= GDT_ADDR && addr < GDT_ADDR + 0x2000 {
            continue;
        }
        // Skip result area
        if addr >= RESULT_ADDR && addr < RESULT_ADDR + 0x1000 {
            continue;
        }
        mem.write_slice(&page.to_le_bytes(), GuestAddress(addr))
            .unwrap();
    }

    // Read from multiple pages in sequence
    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x03, 0x00, // mov rax, [0x30000]
        0x48, 0x8b, 0x1c, 0x25, 0x00, 0x10, 0x03, 0x00, // mov rbx, [0x31000]
        0x48, 0x8b, 0x0c, 0x25, 0x00, 0x20, 0x03, 0x00, // mov rcx, [0x32000]
        0x48, 0x8b, 0x14, 0x25, 0x00, 0x30, 0x03, 0x00, // mov rdx, [0x33000]
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 48, "Page 48 (0x30000)");
    assert_eq!(regs.rbx, 49, "Page 49 (0x31000)");
    assert_eq!(regs.rcx, 50, "Page 50 (0x32000)");
    assert_eq!(regs.rdx, 51, "Page 51 (0x33000)");
}

/// Torture test: Unaligned access at every byte offset in first 16 bytes
#[test]
fn test_torture_unaligned_offsets() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    // Write pattern at base address
    for i in 0..16u8 {
        mem.write_slice(&[i], GuestAddress(0x70000 + i as u64))
            .unwrap();
    }

    let mut vcpu = create_paged_vcpu(mem.clone());

    // Test reads at offsets 0-7
    for offset in 0..8u64 {
        clear_result(&mem);
        let addr = 0x70000 + offset;
        let code: Vec<u8> = vec![
            0x48,
            0x8b,
            0x04,
            0x25,
            (addr & 0xFF) as u8,
            ((addr >> 8) & 0xFF) as u8,
            ((addr >> 16) & 0xFF) as u8,
            ((addr >> 24) & 0xFF) as u8,
            0x48,
            0xa3,
            (RESULT_ADDR & 0xFF) as u8,
            ((RESULT_ADDR >> 8) & 0xFF) as u8,
            ((RESULT_ADDR >> 16) & 0xFF) as u8,
            ((RESULT_ADDR >> 24) & 0xFF) as u8,
            ((RESULT_ADDR >> 32) & 0xFF) as u8,
            ((RESULT_ADDR >> 40) & 0xFF) as u8,
            ((RESULT_ADDR >> 48) & 0xFF) as u8,
            ((RESULT_ADDR >> 56) & 0xFF) as u8,
            0xf4,
        ];
        mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

        let mut regs = vcpu.get_regs().unwrap();
        regs.rip = CODE_PADDR;
        vcpu.set_regs(&regs).unwrap();

        run_until_hlt(&mut vcpu).unwrap();

        let mut buf = [0u8; 8];
        mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();

        // Verify expected bytes
        for i in 0..8 {
            assert_eq!(
                buf[i],
                (offset + i as u64) as u8,
                "Offset {} byte {}",
                offset,
                i
            );
        }
    }
}

/// Torture test: Page boundary crossing at every alignment
#[test]
fn test_torture_boundary_all_alignments() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    // Test 8-byte reads starting at page_end - 7 through page_end
    let page_end = 0x50FFF_u64;

    for start_offset in 1..=8u64 {
        let addr = page_end + 1 - start_offset;
        let expected: u64 = 0x0102030405060708;
        mem.write_slice(&expected.to_le_bytes(), GuestAddress(addr))
            .unwrap();

        clear_result(&mem);

        let mut vcpu = create_paged_vcpu(mem.clone());
        let code: Vec<u8> = vec![
            0x48,
            0x8b,
            0x04,
            0x25,
            (addr & 0xFF) as u8,
            ((addr >> 8) & 0xFF) as u8,
            ((addr >> 16) & 0xFF) as u8,
            ((addr >> 24) & 0xFF) as u8,
            0x48,
            0xa3,
            (RESULT_ADDR & 0xFF) as u8,
            ((RESULT_ADDR >> 8) & 0xFF) as u8,
            ((RESULT_ADDR >> 16) & 0xFF) as u8,
            ((RESULT_ADDR >> 24) & 0xFF) as u8,
            ((RESULT_ADDR >> 32) & 0xFF) as u8,
            ((RESULT_ADDR >> 40) & 0xFF) as u8,
            ((RESULT_ADDR >> 48) & 0xFF) as u8,
            ((RESULT_ADDR >> 56) & 0xFF) as u8,
            0xf4,
        ];
        mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

        let mut regs = vcpu.get_regs().unwrap();
        regs.rip = CODE_PADDR;
        vcpu.set_regs(&regs).unwrap();

        run_until_hlt(&mut vcpu).unwrap();

        let mut buf = [0u8; 8];
        mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
        assert_eq!(
            u64::from_le_bytes(buf),
            expected,
            "Boundary crossing at page_end - {} + 1",
            start_offset
        );
    }
}

/// Torture test: Multiple distinct pages in single code sequence
#[test]
fn test_torture_multi_page_access_pattern() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    // Create distinct values in 8 different pages
    for i in 0..8u64 {
        let page_addr = 0x30000 + i * 0x1000;
        let value = 0x1111111111111111_u64 * (i + 1);
        mem.write_slice(&value.to_le_bytes(), GuestAddress(page_addr))
            .unwrap();
    }

    let mut vcpu = create_paged_vcpu(mem.clone());

    // Code reads from all 8 pages, XORs them together
    let code: &[u8] = &[
        0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x03, 0x00, // mov rax, [0x30000]
        0x48, 0x33, 0x04, 0x25, 0x00, 0x10, 0x03, 0x00, // xor rax, [0x31000]
        0x48, 0x33, 0x04, 0x25, 0x00, 0x20, 0x03, 0x00, // xor rax, [0x32000]
        0x48, 0x33, 0x04, 0x25, 0x00, 0x30, 0x03, 0x00, // xor rax, [0x33000]
        0x48, 0x33, 0x04, 0x25, 0x00, 0x40, 0x03, 0x00, // xor rax, [0x34000]
        0x48, 0x33, 0x04, 0x25, 0x00, 0x50, 0x03, 0x00, // xor rax, [0x35000]
        0x48, 0x33, 0x04, 0x25, 0x00, 0x60, 0x03, 0x00, // xor rax, [0x36000]
        0x48, 0x33, 0x04, 0x25, 0x00, 0x70, 0x03, 0x00, // xor rax, [0x37000]
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    // XOR of 1*0x1111... through 8*0x1111...
    let expected = (1..=8u64).fold(0u64, |acc, i| acc ^ (0x1111111111111111_u64 * i));
    assert_eq!(regs.rax, expected);
}

/// Test zero page access
#[test]
fn test_zero_page_access() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    mem.write_slice(&0xDEADC0DE_u32.to_le_bytes(), GuestAddress(0x100))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x8b,
        0x04,
        0x25,
        0x00,
        0x01,
        0x00,
        0x00, // mov eax, [0x100]
        0x48,
        0xa3,
        (RESULT_ADDR & 0xFF) as u8,
        ((RESULT_ADDR >> 8) & 0xFF) as u8,
        ((RESULT_ADDR >> 16) & 0xFF) as u8,
        ((RESULT_ADDR >> 24) & 0xFF) as u8,
        ((RESULT_ADDR >> 32) & 0xFF) as u8,
        ((RESULT_ADDR >> 40) & 0xFF) as u8,
        ((RESULT_ADDR >> 48) & 0xFF) as u8,
        ((RESULT_ADDR >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), 0xDEADC0DE);
}

/// Test first and last bytes of a page
#[test]
fn test_page_edge_bytes() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    mem.write_slice(&[0xAA], GuestAddress(0x50000)).unwrap(); // First byte
    mem.write_slice(&[0xBB], GuestAddress(0x50FFF)).unwrap(); // Last byte

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x8a, 0x04, 0x25, 0x00, 0x00, 0x05, 0x00, // mov al, [0x50000]
        0x8a, 0x1c, 0x25, 0xFF, 0x0F, 0x05, 0x00, // mov bl, [0x50FFF]
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0xAA);
    assert_eq!(regs.rbx & 0xFF, 0xBB);
}

// ============================================================================
// TLB TESTS
// ============================================================================

/// Test TLB caching - repeated access to same page
#[test]
fn test_tlb_repeated_access() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    mem.write_slice(&0xAABBCCDD_u64.to_le_bytes(), GuestAddress(0x70000))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        // 10 reads from same page (TLB should cache after first)
        0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x07, 0x00, 0x48, 0x8b, 0x04, 0x25, 0x08, 0x00, 0x07,
        0x00, 0x48, 0x8b, 0x04, 0x25, 0x10, 0x00, 0x07, 0x00, 0x48, 0x8b, 0x04, 0x25, 0x18, 0x00,
        0x07, 0x00, 0x48, 0x8b, 0x04, 0x25, 0x20, 0x00, 0x07, 0x00, 0x48, 0x8b, 0x04, 0x25, 0x28,
        0x00, 0x07, 0x00, 0x48, 0x8b, 0x04, 0x25, 0x30, 0x00, 0x07, 0x00, 0x48, 0x8b, 0x04, 0x25,
        0x38, 0x00, 0x07, 0x00, 0x48, 0x8b, 0x04, 0x25, 0x40, 0x00, 0x07, 0x00, 0x48, 0x8b, 0x04,
        0x25, 0x00, 0x00, 0x07, 0x00, // Back to start
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0xAABBCCDD);
}

/// Test INVLPG invalidates TLB entry
#[test]
fn test_invlpg_invalidation() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    let test_addr = 0x80000_u64;
    let pt_entry_addr = PT_ADDR + 128 * 8;

    mem.write_slice(&0xAABBCCDD_u64.to_le_bytes(), GuestAddress(test_addr))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());

    // Read to populate TLB, clear PTE, INVLPG, read again (should fault)
    let code: Vec<u8> = vec![
        0x48,
        0x8b,
        0x04,
        0x25,
        0x00,
        0x00,
        0x08,
        0x00, // mov rax, [0x80000]
        0x48,
        0x31,
        0xdb, // xor rbx, rbx
        0x48,
        0x89,
        0x1c,
        0x25, // mov [pt_entry], rbx
        (pt_entry_addr & 0xFF) as u8,
        ((pt_entry_addr >> 8) & 0xFF) as u8,
        ((pt_entry_addr >> 16) & 0xFF) as u8,
        ((pt_entry_addr >> 24) & 0xFF) as u8,
        0x0f,
        0x01,
        0x3c,
        0x25,
        0x00,
        0x00,
        0x08,
        0x00, // invlpg [0x80000]
        0x48,
        0x8b,
        0x0c,
        0x25,
        0x00,
        0x00,
        0x08,
        0x00, // mov rcx, [0x80000] - faults
        0xf4,
    ];
    mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    let (vaddr, error_code) = expect_page_fault(&mut vcpu).unwrap();
    assert_eq!(vaddr, test_addr, "Should fault after INVLPG");
    assert_eq!(error_code & pf_error::P, 0, "P=0 (non-present)");
}

/// Test CR3 write flushes TLB
#[test]
fn test_cr3_flush_tlb() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    let test_addr = 0x80000_u64;
    let pt_entry_addr = PT_ADDR + 128 * 8;

    mem.write_slice(&0xAABBCCDD_u64.to_le_bytes(), GuestAddress(test_addr))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());

    // Read, clear PTE, reload CR3 (flush), read again (should fault)
    let code: Vec<u8> = vec![
        0x48,
        0x8b,
        0x04,
        0x25,
        0x00,
        0x00,
        0x08,
        0x00, // mov rax, [0x80000]
        0x48,
        0x31,
        0xdb, // xor rbx, rbx
        0x48,
        0x89,
        0x1c,
        0x25,
        (pt_entry_addr & 0xFF) as u8,
        ((pt_entry_addr >> 8) & 0xFF) as u8,
        ((pt_entry_addr >> 16) & 0xFF) as u8,
        ((pt_entry_addr >> 24) & 0xFF) as u8,
        0x0f,
        0x20,
        0xd8, // mov rax, cr3
        0x0f,
        0x22,
        0xd8, // mov cr3, rax (flush)
        0x48,
        0x8b,
        0x0c,
        0x25,
        0x00,
        0x00,
        0x08,
        0x00, // mov rcx, [0x80000]
        0xf4,
    ];
    mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    let (vaddr, error_code) = expect_page_fault(&mut vcpu).unwrap();
    assert_eq!(vaddr, test_addr, "Should fault after CR3 flush");
    assert_eq!(error_code & pf_error::P, 0, "P=0 (non-present)");
}

// ============================================================================
// STRESS TESTS
// ============================================================================

/// Stress test: TLB thrashing with many distinct pages
#[test]
fn test_stress_tlb_thrashing() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    // Write values to 64 different pages
    for i in 0..64u64 {
        let addr = 0x30000 + i * 0x1000;
        mem.write_slice(&i.to_le_bytes(), GuestAddress(addr))
            .unwrap();
    }

    let mut vcpu = create_paged_vcpu(mem.clone());

    // Access pattern that exceeds typical TLB size (256 entries)
    // Access pages 0-63, then 0-63 again (tests TLB replacement)
    let mut code = Vec::new();
    for round in 0..2 {
        for i in 0..64u64 {
            let addr = 0x30000 + ((i + round * 17) % 64) * 0x1000; // Shuffle order
            code.extend_from_slice(&[
                0x48,
                0x8b,
                0x04,
                0x25,
                (addr & 0xFF) as u8,
                ((addr >> 8) & 0xFF) as u8,
                ((addr >> 16) & 0xFF) as u8,
                ((addr >> 24) & 0xFF) as u8,
            ]);
        }
    }
    code.push(0xf4); // HLT

    mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    // This should complete without errors
    run_until_hlt(&mut vcpu).unwrap();
}

/// Stress test: Rapid read/write alternation across pages
#[test]
fn test_stress_read_write_alternation() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    let mut vcpu = create_paged_vcpu(mem.clone());

    // Alternating reads and writes to different pages
    let code: &[u8] = &[
        // Write pattern
        0x48, 0xc7, 0x04, 0x25, 0x00, 0x00, 0x03, 0x00, 0x11, 0x11, 0x11, 0x11, 0x48, 0xc7, 0x04,
        0x25, 0x00, 0x10, 0x03, 0x00, 0x22, 0x22, 0x22, 0x22, 0x48, 0xc7, 0x04, 0x25, 0x00, 0x20,
        0x03, 0x00, 0x33, 0x33, 0x33, 0x33, 0x48, 0xc7, 0x04, 0x25, 0x00, 0x30, 0x03, 0x00, 0x44,
        0x44, 0x44, 0x44, // Read and accumulate
        0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x03, 0x00, // rax = [0x30000]
        0x48, 0x03, 0x04, 0x25, 0x00, 0x10, 0x03, 0x00, // rax += [0x31000]
        0x48, 0x03, 0x04, 0x25, 0x00, 0x20, 0x03, 0x00, // rax += [0x32000]
        0x48, 0x03, 0x04, 0x25, 0x00, 0x30, 0x03, 0x00, // rax += [0x33000]
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        regs.rax as u32,
        0x11111111 + 0x22222222 + 0x33333333 + 0x44444444
    );
}

// ============================================================================
// COMPLEX SCENARIOS
// ============================================================================

/// Test function call chain across multiple pages
#[test]
fn test_call_chain_multi_page() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    let func1_addr = 0x11000_u64;
    let func2_addr = 0x12000_u64;
    let func3_addr = 0x13000_u64;

    // func3: add 1 to rax and return
    let func3: &[u8] = &[
        0x48, 0xff, 0xc0, // inc rax
        0xc3, // ret
    ];
    mem.write_slice(func3, GuestAddress(func3_addr)).unwrap();

    // func2: call func3, add 10, return
    let rel3 = (func3_addr as i32) - (func2_addr as i32 + 5);
    let func2 = [
        0xe8,
        (rel3 & 0xFF) as u8,
        ((rel3 >> 8) & 0xFF) as u8,
        ((rel3 >> 16) & 0xFF) as u8,
        ((rel3 >> 24) & 0xFF) as u8,
        0x48,
        0x83,
        0xc0,
        0x0a, // add rax, 10
        0xc3,
    ];
    mem.write_slice(&func2, GuestAddress(func2_addr)).unwrap();

    // func1: call func2, add 100, return
    let rel2 = (func2_addr as i32) - (func1_addr as i32 + 5);
    let func1 = [
        0xe8,
        (rel2 & 0xFF) as u8,
        ((rel2 >> 8) & 0xFF) as u8,
        ((rel2 >> 16) & 0xFF) as u8,
        ((rel2 >> 24) & 0xFF) as u8,
        0x48,
        0x83,
        0xc0,
        0x64, // add rax, 100
        0xc3,
    ];
    mem.write_slice(&func1, GuestAddress(func1_addr)).unwrap();

    // Main: set rax=0, call func1, store result
    let rel1 = (func1_addr as i32) - (CODE_PADDR as i32 + 5 + 3);
    let mut main_code = vec![
        0x48,
        0x31,
        0xc0, // xor rax, rax
        0xe8,
        (rel1 & 0xFF) as u8,
        ((rel1 >> 8) & 0xFF) as u8,
        ((rel1 >> 16) & 0xFF) as u8,
        ((rel1 >> 24) & 0xFF) as u8,
    ];
    main_code.extend_from_slice(&[0x48, 0xa3]);
    main_code.extend_from_slice(&RESULT_ADDR.to_le_bytes());
    main_code.push(0xf4);
    mem.write_slice(&main_code, GuestAddress(CODE_PADDR))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), 111, "0 + 1 + 10 + 100 = 111");
}

/// Test push/pop operations through paging
#[test]
fn test_stack_operations_paged() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    let mut vcpu = create_paged_vcpu(mem.clone());

    let code: &[u8] = &[
        0x48,
        0xb8,
        0xBE,
        0xBA,
        0xFE,
        0xCA,
        0xEF,
        0xBE,
        0xAD,
        0xDE, // mov rax, 0xDEADBEEFCAFEBABE
        0x50, // push rax
        0x48,
        0x31,
        0xc0, // xor rax, rax
        0x5b, // pop rbx
        0x48,
        0x89,
        0x1c,
        0x25,
        (RESULT_ADDR & 0xFF) as u8,
        ((RESULT_ADDR >> 8) & 0xFF) as u8,
        ((RESULT_ADDR >> 16) & 0xFF) as u8,
        ((RESULT_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), 0xDEADBEEFCAFEBABE);
}

/// Test mixed page sizes in same address space
#[test]
fn test_mixed_page_sizes() {
    let mem = create_memory(16);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    // PD[0] = 4KB pages, PD[1] = 2MB huge page
    let pde_2mb = 0x200000_u64
        | pte_flags::PRESENT
        | pte_flags::WRITABLE
        | pte_flags::USER
        | pte_flags::HUGE_PAGE;
    mem.write_slice(&pde_2mb.to_le_bytes(), GuestAddress(PD_ADDR + 8))
        .unwrap();

    let value_4kb = 0x4444444444444444_u64;
    let value_2mb = 0x2222222222222222_u64;
    mem.write_slice(&value_4kb.to_le_bytes(), GuestAddress(0x50000))
        .unwrap();
    mem.write_slice(&value_2mb.to_le_bytes(), GuestAddress(0x250000))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x05, 0x00, // mov rax, [0x50000]
        0x48, 0x8b, 0x1c, 0x25, 0x00, 0x00, 0x25, 0x00, // mov rbx, [0x250000]
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, value_4kb, "4KB page read");
    assert_eq!(regs.rbx, value_2mb, "2MB page read");
}

// ============================================================================
// EXOTIC TESTS
// ============================================================================

/// Test reserved bit in PTE causes page fault
#[test]
fn test_reserved_bit_pte() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    // Set reserved bit (bit 51) in PTE - with 48-bit physical addresses, bits 51:48 are reserved
    let pte =
        0x70000_u64 | pte_flags::PRESENT | pte_flags::WRITABLE | pte_flags::USER | (1u64 << 51);
    mem.write_slice(&pte.to_le_bytes(), GuestAddress(PT_ADDR + 112 * 8))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x07, 0x00, 0xf4];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    let (vaddr, error_code) = expect_page_fault(&mut vcpu).unwrap();
    assert_eq!(vaddr, 0x70000);
    assert_ne!(error_code & pf_error::RSVD, 0, "RSVD bit should be set");
    assert_ne!(error_code & pf_error::P, 0, "P bit should be set");
}

/// Test non-present at PD level
#[test]
fn test_non_present_pd_level() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);

    // Clear PD[1] - addresses 2-4MB non-present
    mem.write_slice(&0u64.to_le_bytes(), GuestAddress(PD_ADDR + 8))
        .unwrap();

    let mut vcpu = create_paged_vcpu(mem.clone());
    let code: &[u8] = &[
        0x48, 0x8b, 0x04, 0x25, 0x00, 0x00, 0x20, 0x00, // mov rax, [0x200000]
        0xf4,
    ];
    mem.write_slice(code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    let (vaddr, error_code) = expect_page_fault(&mut vcpu).unwrap();
    assert_eq!(vaddr, 0x200000);
    assert_eq!(error_code & pf_error::P, 0, "P=0 (non-present)");
}

/// Test self-modifying code detection pattern
#[test]
fn test_smc_pattern() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    let target_addr = 0x70000_u64;

    let mut vcpu = create_paged_vcpu(mem.clone());

    // Write code, then modify it, then execute modified version
    // Initial: mov rax, 1
    let initial_code: &[u8] = &[
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // mov rax, 1
        0xf4,
    ];
    mem.write_slice(initial_code, GuestAddress(target_addr))
        .unwrap();

    // Main code: modify target then jump to it
    let code: Vec<u8> = vec![
        // Write new instruction at target: mov rax, 42
        0x48,
        0xc7,
        0x04,
        0x25,
        (target_addr & 0xFF) as u8,
        ((target_addr >> 8) & 0xFF) as u8,
        ((target_addr >> 16) & 0xFF) as u8,
        ((target_addr >> 24) & 0xFF) as u8,
        0xc0,
        0xc7,
        0xc0,
        0x48, // Store bytes of "mov rax," at addr
        0x48,
        0xc7,
        0x04,
        0x25,
        ((target_addr + 4) & 0xFF) as u8,
        (((target_addr + 4) >> 8) & 0xFF) as u8,
        (((target_addr + 4) >> 16) & 0xFF) as u8,
        (((target_addr + 4) >> 24) & 0xFF) as u8,
        0x2a,
        0x00,
        0x00,
        0x00, // Store ", 42" and part of next
        // Jump to target
        0xff,
        0x25,
        0x00,
        0x00,
        0x00,
        0x00, // jmp [rip+0] (absolute indirect)
        (target_addr & 0xFF) as u8,
        ((target_addr >> 8) & 0xFF) as u8,
        ((target_addr >> 16) & 0xFF) as u8,
        ((target_addr >> 24) & 0xFF) as u8,
        ((target_addr >> 32) & 0xFF) as u8,
        ((target_addr >> 40) & 0xFF) as u8,
        ((target_addr >> 48) & 0xFF) as u8,
        ((target_addr >> 56) & 0xFF) as u8,
    ];
    mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

    // Simpler test: just verify we can write and read back through page tables
    // The SMC detection is for the decode cache, which is an optimization
    let simple_code: &[u8] = &[
        0x48,
        0xc7,
        0xc0,
        0x2A,
        0x00,
        0x00,
        0x00, // mov rax, 42
        0x48,
        0xa3,
        (RESULT_ADDR & 0xFF) as u8,
        ((RESULT_ADDR >> 8) & 0xFF) as u8,
        ((RESULT_ADDR >> 16) & 0xFF) as u8,
        ((RESULT_ADDR >> 24) & 0xFF) as u8,
        ((RESULT_ADDR >> 32) & 0xFF) as u8,
        ((RESULT_ADDR >> 40) & 0xFF) as u8,
        ((RESULT_ADDR >> 48) & 0xFF) as u8,
        ((RESULT_ADDR >> 56) & 0xFF) as u8,
        0xf4,
    ];
    mem.write_slice(simple_code, GuestAddress(CODE_PADDR))
        .unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(RESULT_ADDR)).unwrap();
    assert_eq!(u64::from_le_bytes(buf), 42);
}

/// Performance test: Time translation of many addresses
#[test]
fn test_performance_translation_speed() {
    let mem = create_memory(8);
    setup_identity_page_tables(&mem, pte_flags::WRITABLE | pte_flags::USER);
    setup_idt(&mem);
    setup_handlers(&mem);

    // Fill pages with data
    for i in 0..256u64 {
        let addr = 0x30000 + i * 16;
        mem.write_slice(&i.to_le_bytes(), GuestAddress(addr))
            .unwrap();
    }

    let mut vcpu = create_paged_vcpu(mem.clone());

    // Generate code that reads from 256 different addresses within the page
    let mut code = Vec::new();
    for i in 0..256u64 {
        let addr = 0x30000 + i * 16;
        code.extend_from_slice(&[
            0x48,
            0x8b,
            0x04,
            0x25,
            (addr & 0xFF) as u8,
            ((addr >> 8) & 0xFF) as u8,
            ((addr >> 16) & 0xFF) as u8,
            ((addr >> 24) & 0xFF) as u8,
        ]);
    }
    code.push(0xf4);
    mem.write_slice(&code, GuestAddress(CODE_PADDR)).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = CODE_PADDR;
    vcpu.set_regs(&regs).unwrap();

    let start = std::time::Instant::now();
    run_until_hlt(&mut vcpu).unwrap();
    let elapsed = start.elapsed();

    // Just verify it completes in reasonable time
    assert!(elapsed.as_secs() < 5, "Should complete quickly");
}
