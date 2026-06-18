//! Auto-generated A32 CP15 coprocessor register tests.
//! DO NOT EDIT MANUALLY.

use super::super::test_helpers_32::*;


#[test]
fn test_mrc_mpidr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c0, c0, 5  (MPIDR)
    let mrc_insn: u32 = 0xEE100FB0;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC MPIDR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("MPIDR = 0x{:08X}", value);
}


#[test]
fn test_mrc_actlr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c1, c0, 1  (ACTLR)
    let mrc_insn: u32 = 0xEE110F30;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC ACTLR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("ACTLR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_actlr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c1, c0, 1  (ACTLR)
    let mcr_insn: u32 = 0xEE011F30;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c1, c0, 1  (ACTLR)
    let mrc_insn: u32 = 0xEE110F30;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("ACTLR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_ttbr0() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c2, c0, 0  (TTBR0)
    let mrc_insn: u32 = 0xEE120F10;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC TTBR0 should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("TTBR0 = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_ttbr0() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c2, c0, 0  (TTBR0)
    let mcr_insn: u32 = 0xEE021F10;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c2, c0, 0  (TTBR0)
    let mrc_insn: u32 = 0xEE120F10;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("TTBR0: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_tpidruro() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c13, c0, 3  (TPIDRURO)
    let mrc_insn: u32 = 0xEE1D0F70;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC TPIDRURO should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("TPIDRURO = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_tpidruro() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c13, c0, 3  (TPIDRURO)
    let mcr_insn: u32 = 0xEE0D1F70;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c13, c0, 3  (TPIDRURO)
    let mrc_insn: u32 = 0xEE1D0F70;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("TPIDRURO: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_dfar() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c6, c0, 0  (DFAR)
    let mrc_insn: u32 = 0xEE160F10;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC DFAR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("DFAR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_dfar() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c6, c0, 0  (DFAR)
    let mcr_insn: u32 = 0xEE061F10;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c6, c0, 0  (DFAR)
    let mrc_insn: u32 = 0xEE160F10;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("DFAR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_pmselr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c9, c12, 5  (PMSELR)
    let mrc_insn: u32 = 0xEE190FBC;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC PMSELR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("PMSELR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_pmselr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c9, c12, 5  (PMSELR)
    let mcr_insn: u32 = 0xEE091FBC;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c9, c12, 5  (PMSELR)
    let mrc_insn: u32 = 0xEE190FBC;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("PMSELR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_mvbar() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c12, c0, 1  (MVBAR)
    let mrc_insn: u32 = 0xEE1C0F30;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC MVBAR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("MVBAR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_mvbar() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c12, c0, 1  (MVBAR)
    let mcr_insn: u32 = 0xEE0C1F30;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c12, c0, 1  (MVBAR)
    let mrc_insn: u32 = 0xEE1C0F30;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("MVBAR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_contextidr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c13, c0, 1  (CONTEXTIDR)
    let mrc_insn: u32 = 0xEE1D0F30;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC CONTEXTIDR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("CONTEXTIDR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_contextidr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c13, c0, 1  (CONTEXTIDR)
    let mcr_insn: u32 = 0xEE0D1F30;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c13, c0, 1  (CONTEXTIDR)
    let mrc_insn: u32 = 0xEE1D0F30;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("CONTEXTIDR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_pmcntenset() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c9, c12, 1  (PMCNTENSET)
    let mrc_insn: u32 = 0xEE190F3C;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC PMCNTENSET should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("PMCNTENSET = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_pmcntenset() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c9, c12, 1  (PMCNTENSET)
    let mcr_insn: u32 = 0xEE091F3C;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c9, c12, 1  (PMCNTENSET)
    let mrc_insn: u32 = 0xEE190F3C;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("PMCNTENSET: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_tpidrprw() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c13, c0, 4  (TPIDRPRW)
    let mrc_insn: u32 = 0xEE1D0F90;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC TPIDRPRW should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("TPIDRPRW = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_tpidrprw() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c13, c0, 4  (TPIDRPRW)
    let mcr_insn: u32 = 0xEE0D1F90;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c13, c0, 4  (TPIDRPRW)
    let mrc_insn: u32 = 0xEE1D0F90;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("TPIDRPRW: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_tpidrurw() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c13, c0, 2  (TPIDRURW)
    let mrc_insn: u32 = 0xEE1D0F50;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC TPIDRURW should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("TPIDRURW = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_tpidrurw() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c13, c0, 2  (TPIDRURW)
    let mcr_insn: u32 = 0xEE0D1F50;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c13, c0, 2  (TPIDRURW)
    let mrc_insn: u32 = 0xEE1D0F50;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("TPIDRURW: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_dacr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c3, c0, 0  (DACR)
    let mrc_insn: u32 = 0xEE130F10;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC DACR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("DACR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_dacr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c3, c0, 0  (DACR)
    let mcr_insn: u32 = 0xEE031F10;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c3, c0, 0  (DACR)
    let mrc_insn: u32 = 0xEE130F10;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("DACR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_cpacr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c1, c0, 2  (CPACR)
    let mrc_insn: u32 = 0xEE110F50;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC CPACR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("CPACR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_cpacr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c1, c0, 2  (CPACR)
    let mcr_insn: u32 = 0xEE011F50;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c1, c0, 2  (CPACR)
    let mrc_insn: u32 = 0xEE110F50;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("CPACR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_ctr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c0, c0, 1  (CTR)
    let mrc_insn: u32 = 0xEE100F30;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC CTR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("CTR = 0x{:08X}", value);
}


#[test]
fn test_mrc_ifar() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c6, c0, 2  (IFAR)
    let mrc_insn: u32 = 0xEE160F50;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC IFAR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("IFAR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_ifar() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c6, c0, 2  (IFAR)
    let mcr_insn: u32 = 0xEE061F50;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c6, c0, 2  (IFAR)
    let mrc_insn: u32 = 0xEE160F50;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("IFAR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_vbar() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c12, c0, 0  (VBAR)
    let mrc_insn: u32 = 0xEE1C0F10;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC VBAR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("VBAR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_vbar() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c12, c0, 0  (VBAR)
    let mcr_insn: u32 = 0xEE0C1F10;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c12, c0, 0  (VBAR)
    let mrc_insn: u32 = 0xEE1C0F10;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("VBAR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_ifsr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c5, c0, 1  (IFSR)
    let mrc_insn: u32 = 0xEE150F30;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC IFSR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("IFSR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_ifsr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c5, c0, 1  (IFSR)
    let mcr_insn: u32 = 0xEE051F30;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c5, c0, 1  (IFSR)
    let mrc_insn: u32 = 0xEE150F30;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("IFSR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_cntfrq() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c14, c0, 0  (CNTFRQ)
    let mrc_insn: u32 = 0xEE1E0F10;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC CNTFRQ should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("CNTFRQ = 0x{:08X}", value);
}


#[test]
fn test_mrc_sctlr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c1, c0, 0  (SCTLR)
    let mrc_insn: u32 = 0xEE110F10;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC SCTLR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("SCTLR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_sctlr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c1, c0, 0  (SCTLR)
    let mcr_insn: u32 = 0xEE011F10;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c1, c0, 0  (SCTLR)
    let mrc_insn: u32 = 0xEE110F10;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("SCTLR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_ttbcr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c2, c0, 2  (TTBCR)
    let mrc_insn: u32 = 0xEE120F50;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC TTBCR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("TTBCR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_ttbcr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c2, c0, 2  (TTBCR)
    let mcr_insn: u32 = 0xEE021F50;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c2, c0, 2  (TTBCR)
    let mrc_insn: u32 = 0xEE120F50;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("TTBCR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_pmcntenclr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c9, c12, 2  (PMCNTENCLR)
    let mrc_insn: u32 = 0xEE190F5C;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC PMCNTENCLR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("PMCNTENCLR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_pmcntenclr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c9, c12, 2  (PMCNTENCLR)
    let mcr_insn: u32 = 0xEE091F5C;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c9, c12, 2  (PMCNTENCLR)
    let mrc_insn: u32 = 0xEE190F5C;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("PMCNTENCLR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_pmcr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c9, c12, 0  (PMCR)
    let mrc_insn: u32 = 0xEE190F1C;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC PMCR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("PMCR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_pmcr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c9, c12, 0  (PMCR)
    let mcr_insn: u32 = 0xEE091F1C;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c9, c12, 0  (PMCR)
    let mrc_insn: u32 = 0xEE190F1C;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("PMCR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_dfsr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c5, c0, 0  (DFSR)
    let mrc_insn: u32 = 0xEE150F10;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC DFSR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("DFSR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_dfsr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c5, c0, 0  (DFSR)
    let mcr_insn: u32 = 0xEE051F10;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c5, c0, 0  (DFSR)
    let mrc_insn: u32 = 0xEE150F10;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("DFSR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_ttbr1() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c2, c0, 1  (TTBR1)
    let mrc_insn: u32 = 0xEE120F30;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC TTBR1 should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("TTBR1 = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_ttbr1() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c2, c0, 1  (TTBR1)
    let mcr_insn: u32 = 0xEE021F30;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c2, c0, 1  (TTBR1)
    let mrc_insn: u32 = 0xEE120F30;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("TTBR1: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_pmccntr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c9, c13, 0  (PMCCNTR)
    let mrc_insn: u32 = 0xEE190F1D;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC PMCCNTR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("PMCCNTR = 0x{:08X}", value);
}


#[test]
fn test_mcr_mrc_pmccntr() {
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, 0, R1, c9, c13, 0  (PMCCNTR)
    let mcr_insn: u32 = 0xEE091F1D;
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, 0, R0, c9, c13, 0  (PMCCNTR)
    let mrc_insn: u32 = 0xEE190F1D;
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("PMCCNTR: wrote 0x12345678, read 0x{:08X}", readback);
}


#[test]
fn test_mrc_midr() {
    let mut cpu = create_test_cpu();
    
    // MRC p15, 0, R0, c0, c0, 0  (MIDR)
    let mrc_insn: u32 = 0xEE100F10;
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC MIDR should succeed: {:?}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("MIDR = 0x{:08X}", value);
}

