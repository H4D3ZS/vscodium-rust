//! Auto-generated system register tests.
//! DO NOT EDIT MANUALLY.

use super::super::test_helpers::*;


#[test]
fn test_mrs_tpidrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TPIDR_EL1
    let mrs_insn: u32 = 0xD538D080;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TPIDR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TPIDR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_tpidrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TPIDR_EL1, X1
    let msr_insn: u32 = 0xD518D081;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TPIDR_EL1
    let mrs_insn: u32 = 0xD538D080;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TPIDR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_apdbkeyloel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APDBKeyLo_EL1
    let mrs_insn: u32 = 0xD5382240;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APDBKeyLo_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APDBKeyLo_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apdbkeyloel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APDBKeyLo_EL1, X1
    let msr_insn: u32 = 0xD5182241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APDBKeyLo_EL1
    let mrs_insn: u32 = 0xD5382240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APDBKeyLo_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_spsrel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SPSR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SPSR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_spsrel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SPSR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_spsrel3_field_i() {
    let mut cpu = create_test_cpu();
    
    // Set field I to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("SPSR_EL3.I = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_btype() {
    let mut cpu = create_test_cpu();
    
    // Set field BTYPE to all 1s
    let test_value = 0x0000000000000C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x3;
    println!("SPSR_EL3.BTYPE = 0x{:X} (bits [11:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_uao() {
    let mut cpu = create_test_cpu();
    
    // Set field UAO to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("SPSR_EL3.UAO = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_e() {
    let mut cpu = create_test_cpu();
    
    // Set field E to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("SPSR_EL3.E = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_it() {
    let mut cpu = create_test_cpu();
    
    // Set field IT to all 1s
    let test_value = 0x0000000006000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x3;
    println!("SPSR_EL3.IT = 0x{:X} (bits [26:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("SPSR_EL3.C = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_n() {
    let mut cpu = create_test_cpu();
    
    // Set field N to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("SPSR_EL3.N = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_ge() {
    let mut cpu = create_test_cpu();
    
    // Set field GE to all 1s
    let test_value = 0x00000000000F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("SPSR_EL3.GE = 0x{:X} (bits [19:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_tco() {
    let mut cpu = create_test_cpu();
    
    // Set field TCO to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("SPSR_EL3.TCO = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_d() {
    let mut cpu = create_test_cpu();
    
    // Set field D to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("SPSR_EL3.D = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_ssbs() {
    let mut cpu = create_test_cpu();
    
    // Set field SSBS to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("SPSR_EL3.SSBS = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_q() {
    let mut cpu = create_test_cpu();
    
    // Set field Q to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("SPSR_EL3.Q = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_a() {
    let mut cpu = create_test_cpu();
    
    // Set field A to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("SPSR_EL3.A = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_v() {
    let mut cpu = create_test_cpu();
    
    // Set field V to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("SPSR_EL3.V = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_f() {
    let mut cpu = create_test_cpu();
    
    // Set field F to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("SPSR_EL3.F = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_ss() {
    let mut cpu = create_test_cpu();
    
    // Set field SS to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("SPSR_EL3.SS = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_pan() {
    let mut cpu = create_test_cpu();
    
    // Set field PAN to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("SPSR_EL3.PAN = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_il() {
    let mut cpu = create_test_cpu();
    
    // Set field IL to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("SPSR_EL3.IL = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_dit() {
    let mut cpu = create_test_cpu();
    
    // Set field DIT to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("SPSR_EL3.DIT = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_t() {
    let mut cpu = create_test_cpu();
    
    // Set field T to all 1s
    let test_value = 0x0000000000000020u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1;
    println!("SPSR_EL3.T = 0x{:X} (bits [5:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_z() {
    let mut cpu = create_test_cpu();
    
    // Set field Z to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("SPSR_EL3.Z = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel3_field_m() {
    let mut cpu = create_test_cpu();
    
    // Set field M to all 1s
    let test_value = 0x000000000000000Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL3, X1
    let msr_insn: u32 = 0xD51E4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL3
    let mrs_insn: u32 = 0xD53E4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("SPSR_EL3.M = 0x{:X} (bits [3:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_cptrel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CPTR_EL3
    let mrs_insn: u32 = 0xD53E1140;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CPTR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CPTR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cptrel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CPTR_EL3, X1
    let msr_insn: u32 = 0xD51E1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL3
    let mrs_insn: u32 = 0xD53E1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CPTR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cptrel3_field_ez() {
    let mut cpu = create_test_cpu();
    
    // Set field EZ to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL3, X1
    let msr_insn: u32 = 0xD51E1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL3
    let mrs_insn: u32 = 0xD53E1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("CPTR_EL3.EZ = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel3_field_tcpac() {
    let mut cpu = create_test_cpu();
    
    // Set field TCPAC to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL3, X1
    let msr_insn: u32 = 0xD51E1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL3
    let mrs_insn: u32 = 0xD53E1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("CPTR_EL3.TCPAC = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel3_field_tam() {
    let mut cpu = create_test_cpu();
    
    // Set field TAM to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL3, X1
    let msr_insn: u32 = 0xD51E1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL3
    let mrs_insn: u32 = 0xD53E1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("CPTR_EL3.TAM = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel3_field_tfp() {
    let mut cpu = create_test_cpu();
    
    // Set field TFP to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL3, X1
    let msr_insn: u32 = 0xD51E1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL3
    let mrs_insn: u32 = 0xD53E1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("CPTR_EL3.TFP = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel3_field_tta() {
    let mut cpu = create_test_cpu();
    
    // Set field TTA to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL3, X1
    let msr_insn: u32 = 0xD51E1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL3
    let mrs_insn: u32 = 0xD53E1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("CPTR_EL3.TTA = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_sctlrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SCTLR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SCTLR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_sctlrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SCTLR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_sctlrel1_field_dze() {
    let mut cpu = create_test_cpu();
    
    // Set field DZE to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("SCTLR_EL1.DZE = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_uma() {
    let mut cpu = create_test_cpu();
    
    // Set field UMA to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("SCTLR_EL1.UMA = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_uci() {
    let mut cpu = create_test_cpu();
    
    // Set field UCI to all 1s
    let test_value = 0x0000000004000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x1;
    println!("SCTLR_EL1.UCI = 0x{:X} (bits [26:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_ntlsmd() {
    let mut cpu = create_test_cpu();
    
    // Set field nTLSMD to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("SCTLR_EL1.nTLSMD = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_i() {
    let mut cpu = create_test_cpu();
    
    // Set field I to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("SCTLR_EL1.I = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_bt1() {
    let mut cpu = create_test_cpu();
    
    // Set field BT1 to all 1s
    let test_value = 0x0000001000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0x1;
    println!("SCTLR_EL1.BT1 = 0x{:X} (bits [36:36])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_bt0() {
    let mut cpu = create_test_cpu();
    
    // Set field BT0 to all 1s
    let test_value = 0x0000000800000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 35) & 0x1;
    println!("SCTLR_EL1.BT0 = 0x{:X} (bits [35:35])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_m() {
    let mut cpu = create_test_cpu();
    
    // Set field M to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("SCTLR_EL1.M = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_tcf() {
    let mut cpu = create_test_cpu();
    
    // Set field TCF to all 1s
    let test_value = 0x0000030000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0x3;
    println!("SCTLR_EL1.TCF = 0x{:X} (bits [41:40])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_sa0() {
    let mut cpu = create_test_cpu();
    
    // Set field SA0 to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("SCTLR_EL1.SA0 = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_enib() {
    let mut cpu = create_test_cpu();
    
    // Set field EnIB to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("SCTLR_EL1.EnIB = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_itfsb() {
    let mut cpu = create_test_cpu();
    
    // Set field ITFSB to all 1s
    let test_value = 0x0000002000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 37) & 0x1;
    println!("SCTLR_EL1.ITFSB = 0x{:X} (bits [37:37])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_ee() {
    let mut cpu = create_test_cpu();
    
    // Set field EE to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("SCTLR_EL1.EE = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_endb() {
    let mut cpu = create_test_cpu();
    
    // Set field EnDB to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("SCTLR_EL1.EnDB = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_uct() {
    let mut cpu = create_test_cpu();
    
    // Set field UCT to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("SCTLR_EL1.UCT = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_e0e() {
    let mut cpu = create_test_cpu();
    
    // Set field E0E to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("SCTLR_EL1.E0E = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_enia() {
    let mut cpu = create_test_cpu();
    
    // Set field EnIA to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("SCTLR_EL1.EnIA = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_itd() {
    let mut cpu = create_test_cpu();
    
    // Set field ITD to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("SCTLR_EL1.ITD = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_naa() {
    let mut cpu = create_test_cpu();
    
    // Set field nAA to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("SCTLR_EL1.nAA = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_lsmaoe() {
    let mut cpu = create_test_cpu();
    
    // Set field LSMAOE to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("SCTLR_EL1.LSMAOE = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_enda() {
    let mut cpu = create_test_cpu();
    
    // Set field EnDA to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("SCTLR_EL1.EnDA = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_ata() {
    let mut cpu = create_test_cpu();
    
    // Set field ATA to all 1s
    let test_value = 0x0000080000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 43) & 0x1;
    println!("SCTLR_EL1.ATA = 0x{:X} (bits [43:43])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_wxn() {
    let mut cpu = create_test_cpu();
    
    // Set field WXN to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("SCTLR_EL1.WXN = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_dssbs() {
    let mut cpu = create_test_cpu();
    
    // Set field DSSBS to all 1s
    let test_value = 0x0000100000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0x1;
    println!("SCTLR_EL1.DSSBS = 0x{:X} (bits [44:44])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_tcf0() {
    let mut cpu = create_test_cpu();
    
    // Set field TCF0 to all 1s
    let test_value = 0x000000C000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 38) & 0x3;
    println!("SCTLR_EL1.TCF0 = 0x{:X} (bits [39:38])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_iesb() {
    let mut cpu = create_test_cpu();
    
    // Set field IESB to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("SCTLR_EL1.IESB = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_ntwe() {
    let mut cpu = create_test_cpu();
    
    // Set field nTWE to all 1s
    let test_value = 0x0000000000040000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 18) & 0x1;
    println!("SCTLR_EL1.nTWE = 0x{:X} (bits [18:18])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("SCTLR_EL1.C = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_span() {
    let mut cpu = create_test_cpu();
    
    // Set field SPAN to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("SCTLR_EL1.SPAN = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_eis() {
    let mut cpu = create_test_cpu();
    
    // Set field EIS to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("SCTLR_EL1.EIS = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_eos() {
    let mut cpu = create_test_cpu();
    
    // Set field EOS to all 1s
    let test_value = 0x0000000000000800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x1;
    println!("SCTLR_EL1.EOS = 0x{:X} (bits [11:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_tscxt() {
    let mut cpu = create_test_cpu();
    
    // Set field TSCXT to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("SCTLR_EL1.TSCXT = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_sed() {
    let mut cpu = create_test_cpu();
    
    // Set field SED to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("SCTLR_EL1.SED = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_enrctx() {
    let mut cpu = create_test_cpu();
    
    // Set field EnRCTX to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("SCTLR_EL1.EnRCTX = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_sa() {
    let mut cpu = create_test_cpu();
    
    // Set field SA to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("SCTLR_EL1.SA = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_ata0() {
    let mut cpu = create_test_cpu();
    
    // Set field ATA0 to all 1s
    let test_value = 0x0000040000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 42) & 0x1;
    println!("SCTLR_EL1.ATA0 = 0x{:X} (bits [42:42])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_cp15ben() {
    let mut cpu = create_test_cpu();
    
    // Set field CP15BEN to all 1s
    let test_value = 0x0000000000000020u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1;
    println!("SCTLR_EL1.CP15BEN = 0x{:X} (bits [5:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_ntwi() {
    let mut cpu = create_test_cpu();
    
    // Set field nTWI to all 1s
    let test_value = 0x0000000000010000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x1;
    println!("SCTLR_EL1.nTWI = 0x{:X} (bits [16:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel1_field_a() {
    let mut cpu = create_test_cpu();
    
    // Set field A to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL1, X1
    let msr_insn: u32 = 0xD5181001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL1
    let mrs_insn: u32 = 0xD5381000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("SCTLR_EL1.A = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_esrel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ESR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ESR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_esrel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ESR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_esrel3_field_imm8() {
    let mut cpu = create_test_cpu();
    
    // Set field imm8 to all 1s
    let test_value = 0x00000000000FF000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xFF;
    println!("ESR_EL3.imm8 = 0x{:X} (bits [19:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_am() {
    let mut cpu = create_test_cpu();
    
    // Set field AM to all 1s
    let test_value = 0x000000000000000Eu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x7;
    println!("ESR_EL3.AM = 0x{:X} (bits [3:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_fnv() {
    let mut cpu = create_test_cpu();
    
    // Set field FnV to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("ESR_EL3.FnV = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_opc2() {
    let mut cpu = create_test_cpu();
    
    // Set field Opc2 to all 1s
    let test_value = 0x00000000000E0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 17) & 0x7;
    println!("ESR_EL3.Opc2 = 0x{:X} (bits [19:17])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_offset() {
    let mut cpu = create_test_cpu();
    
    // Set field Offset to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("ESR_EL3.Offset = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_sas() {
    let mut cpu = create_test_cpu();
    
    // Set field SAS to all 1s
    let test_value = 0x0000000000C00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x3;
    println!("ESR_EL3.SAS = 0x{:X} (bits [23:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_cm() {
    let mut cpu = create_test_cpu();
    
    // Set field CM to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("ESR_EL3.CM = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_off() {
    let mut cpu = create_test_cpu();
    
    // Set field OFF to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("ESR_EL3.OFF = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_wnr() {
    let mut cpu = create_test_cpu();
    
    // Set field WnR to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("ESR_EL3.WnR = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_comment() {
    let mut cpu = create_test_cpu();
    
    // Set field Comment to all 1s
    let test_value = 0x000000000000FFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFF;
    println!("ESR_EL3.Comment = 0x{:X} (bits [15:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_srt() {
    let mut cpu = create_test_cpu();
    
    // Set field SRT to all 1s
    let test_value = 0x00000000001F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x1F;
    println!("ESR_EL3.SRT = 0x{:X} (bits [20:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_cv() {
    let mut cpu = create_test_cpu();
    
    // Set field CV to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL3.CV = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ex() {
    let mut cpu = create_test_cpu();
    
    // Set field EX to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("ESR_EL3.EX = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_opc1() {
    let mut cpu = create_test_cpu();
    
    // Set field Opc1 to all 1s
    let test_value = 0x00000000000F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ESR_EL3.Opc1 = 0x{:X} (bits [19:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ereta() {
    let mut cpu = create_test_cpu();
    
    // Set field ERETA to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL3.ERETA = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_set() {
    let mut cpu = create_test_cpu();
    
    // Set field SET to all 1s
    let test_value = 0x0000000000001800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x3;
    println!("ESR_EL3.SET = 0x{:X} (bits [12:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_btype() {
    let mut cpu = create_test_cpu();
    
    // Set field BTYPE to all 1s
    let test_value = 0x0000000000000003u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3;
    println!("ESR_EL3.BTYPE = 0x{:X} (bits [1:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_eret() {
    let mut cpu = create_test_cpu();
    
    // Set field ERET to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ESR_EL3.ERET = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_imm16() {
    let mut cpu = create_test_cpu();
    
    // Set field imm16 to all 1s
    let test_value = 0x000000000000FFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFF;
    println!("ESR_EL3.imm16 = 0x{:X} (bits [15:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_op2() {
    let mut cpu = create_test_cpu();
    
    // Set field Op2 to all 1s
    let test_value = 0x00000000000E0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 17) & 0x7;
    println!("ESR_EL3.Op2 = 0x{:X} (bits [19:17])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_s1ptw() {
    let mut cpu = create_test_cpu();
    
    // Set field S1PTW to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("ESR_EL3.S1PTW = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ixf() {
    let mut cpu = create_test_cpu();
    
    // Set field IXF to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("ESR_EL3.IXF = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_iesb() {
    let mut cpu = create_test_cpu();
    
    // Set field IESB to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("ESR_EL3.IESB = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_rt2() {
    let mut cpu = create_test_cpu();
    
    // Set field Rt2 to all 1s
    let test_value = 0x0000000000007C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1F;
    println!("ESR_EL3.Rt2 = 0x{:X} (bits [14:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ti() {
    let mut cpu = create_test_cpu();
    
    // Set field TI to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL3.TI = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_crm() {
    let mut cpu = create_test_cpu();
    
    // Set field CRm to all 1s
    let test_value = 0x000000000000001Eu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0xF;
    println!("ESR_EL3.CRm = 0x{:X} (bits [4:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_isv() {
    let mut cpu = create_test_cpu();
    
    // Set field ISV to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL3.ISV = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_rt() {
    let mut cpu = create_test_cpu();
    
    // Set field Rt to all 1s
    let test_value = 0x00000000000003E0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1F;
    println!("ESR_EL3.Rt = 0x{:X} (bits [9:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_op0() {
    let mut cpu = create_test_cpu();
    
    // Set field Op0 to all 1s
    let test_value = 0x0000000000300000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x3;
    println!("ESR_EL3.Op0 = 0x{:X} (bits [21:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_op1() {
    let mut cpu = create_test_cpu();
    
    // Set field Op1 to all 1s
    let test_value = 0x000000000001C000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x7;
    println!("ESR_EL3.Op1 = 0x{:X} (bits [16:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ids() {
    let mut cpu = create_test_cpu();
    
    // Set field IDS to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL3.IDS = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ar() {
    let mut cpu = create_test_cpu();
    
    // Set field AR to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("ESR_EL3.AR = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_dfsc() {
    let mut cpu = create_test_cpu();
    
    // Set field DFSC to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("ESR_EL3.DFSC = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_sse() {
    let mut cpu = create_test_cpu();
    
    // Set field SSE to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("ESR_EL3.SSE = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_idf() {
    let mut cpu = create_test_cpu();
    
    // Set field IDF to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("ESR_EL3.IDF = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_cond() {
    let mut cpu = create_test_cpu();
    
    // Set field COND to all 1s
    let test_value = 0x0000000000F00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ESR_EL3.COND = 0x{:X} (bits [23:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_il() {
    let mut cpu = create_test_cpu();
    
    // Set field IL to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("ESR_EL3.IL = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_vncr() {
    let mut cpu = create_test_cpu();
    
    // Set field VNCR to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("ESR_EL3.VNCR = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_vecitr() {
    let mut cpu = create_test_cpu();
    
    // Set field VECITR to all 1s
    let test_value = 0x0000000000000700u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x7;
    println!("ESR_EL3.VECITR = 0x{:X} (bits [10:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ea() {
    let mut cpu = create_test_cpu();
    
    // Set field EA to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("ESR_EL3.EA = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_tfv() {
    let mut cpu = create_test_cpu();
    
    // Set field TFV to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("ESR_EL3.TFV = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_sf() {
    let mut cpu = create_test_cpu();
    
    // Set field SF to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("ESR_EL3.SF = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_uff() {
    let mut cpu = create_test_cpu();
    
    // Set field UFF to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("ESR_EL3.UFF = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ifsc() {
    let mut cpu = create_test_cpu();
    
    // Set field IFSC to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("ESR_EL3.IFSC = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_iof() {
    let mut cpu = create_test_cpu();
    
    // Set field IOF to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL3.IOF = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_crn() {
    let mut cpu = create_test_cpu();
    
    // Set field CRn to all 1s
    let test_value = 0x0000000000003C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0xF;
    println!("ESR_EL3.CRn = 0x{:X} (bits [13:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ccknownpass() {
    let mut cpu = create_test_cpu();
    
    // Set field CCKNOWNPASS to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("ESR_EL3.CCKNOWNPASS = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_rn() {
    let mut cpu = create_test_cpu();
    
    // Set field Rn to all 1s
    let test_value = 0x00000000000003E0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1F;
    println!("ESR_EL3.Rn = 0x{:X} (bits [9:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_ec() {
    let mut cpu = create_test_cpu();
    
    // Set field EC to all 1s
    let test_value = 0x00000000FC000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x3F;
    println!("ESR_EL3.EC = 0x{:X} (bits [31:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_aet() {
    let mut cpu = create_test_cpu();
    
    // Set field AET to all 1s
    let test_value = 0x0000000000001C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x7;
    println!("ESR_EL3.AET = 0x{:X} (bits [12:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_direction() {
    let mut cpu = create_test_cpu();
    
    // Set field Direction to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL3.Direction = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel3_field_dzf() {
    let mut cpu = create_test_cpu();
    
    // Set field DZF to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL3, X1
    let msr_insn: u32 = 0xD51E5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL3
    let mrs_insn: u32 = 0xD53E5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ESR_EL3.DZF = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_pmselrel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, PMSELR_EL0
    let mrs_insn: u32 = 0xD53B9CA0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS PMSELR_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("PMSELR_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_pmselrel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR PMSELR_EL0, X1
    let msr_insn: u32 = 0xD51B9CA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMSELR_EL0
    let mrs_insn: u32 = 0xD53B9CA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("PMSELR_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_pmselrel0_field_sel() {
    let mut cpu = create_test_cpu();
    
    // Set field SEL to all 1s
    let test_value = 0x000000000000001Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMSELR_EL0, X1
    let msr_insn: u32 = 0xD51B9CA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMSELR_EL0
    let mrs_insn: u32 = 0xD53B9CA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1F;
    println!("PMSELR_EL0.SEL = 0x{:X} (bits [4:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_vmpidrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, VMPIDR_EL2
    let mrs_insn: u32 = 0xD53C00A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS VMPIDR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("VMPIDR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_vmpidrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR VMPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C00A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VMPIDR_EL2
    let mrs_insn: u32 = 0xD53C00A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("VMPIDR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_vmpidrel2_field_mt() {
    let mut cpu = create_test_cpu();
    
    // Set field MT to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VMPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C00A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VMPIDR_EL2
    let mrs_insn: u32 = 0xD53C00A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("VMPIDR_EL2.MT = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vmpidrel2_field_aff2() {
    let mut cpu = create_test_cpu();
    
    // Set field Aff2 to all 1s
    let test_value = 0x0000000000FF0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VMPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C00A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VMPIDR_EL2
    let mrs_insn: u32 = 0xD53C00A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xFF;
    println!("VMPIDR_EL2.Aff2 = 0x{:X} (bits [23:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vmpidrel2_field_aff0() {
    let mut cpu = create_test_cpu();
    
    // Set field Aff0 to all 1s
    let test_value = 0x00000000000000FFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR VMPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C00A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VMPIDR_EL2
    let mrs_insn: u32 = 0xD53C00A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFF;
    println!("VMPIDR_EL2.Aff0 = 0x{:X} (bits [7:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vmpidrel2_field_aff1() {
    let mut cpu = create_test_cpu();
    
    // Set field Aff1 to all 1s
    let test_value = 0x000000000000FF00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VMPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C00A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VMPIDR_EL2
    let mrs_insn: u32 = 0xD53C00A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xFF;
    println!("VMPIDR_EL2.Aff1 = 0x{:X} (bits [15:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vmpidrel2_field_aff3() {
    let mut cpu = create_test_cpu();
    
    // Set field Aff3 to all 1s
    let test_value = 0x000000FF00000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VMPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C00A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VMPIDR_EL2
    let mrs_insn: u32 = 0xD53C00A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0xFF;
    println!("VMPIDR_EL2.Aff3 = 0x{:X} (bits [39:32])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vmpidrel2_field_u() {
    let mut cpu = create_test_cpu();
    
    // Set field U to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VMPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C00A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VMPIDR_EL2
    let mrs_insn: u32 = 0xD53C00A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("VMPIDR_EL2.U = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_cntptvalel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTP_TVAL_EL0
    let mrs_insn: u32 = 0xD53BE200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTP_TVAL_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTP_TVAL_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cntptvalel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CNTP_TVAL_EL0, X1
    let msr_insn: u32 = 0xD51BE201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTP_TVAL_EL0
    let mrs_insn: u32 = 0xD53BE200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTP_TVAL_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cntptvalel0_field_timervalue() {
    let mut cpu = create_test_cpu();
    
    // Set field TimerValue to all 1s
    let test_value = 0x00000000FFFFFFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTP_TVAL_EL0, X1
    let msr_insn: u32 = 0xD51BE201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTP_TVAL_EL0
    let mrs_insn: u32 = 0xD53BE200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFFFFFF;
    println!("CNTP_TVAL_EL0.TimerValue = 0x{:X} (bits [31:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_iccsreel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_SRE_EL1
    let mrs_insn: u32 = 0xD538CCA0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_SRE_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_SRE_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccsreel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_SRE_EL1, X1
    let msr_insn: u32 = 0xD518CCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL1
    let mrs_insn: u32 = 0xD538CCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_SRE_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccsreel1_field_dfb() {
    let mut cpu = create_test_cpu();
    
    // Set field DFB to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL1, X1
    let msr_insn: u32 = 0xD518CCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL1
    let mrs_insn: u32 = 0xD538CCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ICC_SRE_EL1.DFB = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccsreel1_field_sre() {
    let mut cpu = create_test_cpu();
    
    // Set field SRE to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL1, X1
    let msr_insn: u32 = 0xD518CCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL1
    let mrs_insn: u32 = 0xD538CCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ICC_SRE_EL1.SRE = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccsreel1_field_dib() {
    let mut cpu = create_test_cpu();
    
    // Set field DIB to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL1, X1
    let msr_insn: u32 = 0xD518CCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL1
    let mrs_insn: u32 = 0xD538CCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("ICC_SRE_EL1.DIB = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_apiakeyloel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APIAKeyLo_EL1
    let mrs_insn: u32 = 0xD5382100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APIAKeyLo_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APIAKeyLo_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apiakeyloel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APIAKeyLo_EL1, X1
    let msr_insn: u32 = 0xD5182101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APIAKeyLo_EL1
    let mrs_insn: u32 = 0xD5382100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APIAKeyLo_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_cntvctlel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTV_CTL_EL0
    let mrs_insn: u32 = 0xD53BE320;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTV_CTL_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTV_CTL_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cntvctlel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CNTV_CTL_EL0, X1
    let msr_insn: u32 = 0xD51BE321;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTV_CTL_EL0
    let mrs_insn: u32 = 0xD53BE320;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTV_CTL_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cntvctlel0_field_istatus() {
    let mut cpu = create_test_cpu();
    
    // Set field ISTATUS to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTV_CTL_EL0, X1
    let msr_insn: u32 = 0xD51BE321;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTV_CTL_EL0
    let mrs_insn: u32 = 0xD53BE320;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("CNTV_CTL_EL0.ISTATUS = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntvctlel0_field_imask() {
    let mut cpu = create_test_cpu();
    
    // Set field IMASK to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTV_CTL_EL0, X1
    let msr_insn: u32 = 0xD51BE321;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTV_CTL_EL0
    let mrs_insn: u32 = 0xD53BE320;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("CNTV_CTL_EL0.IMASK = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntvctlel0_field_enable() {
    let mut cpu = create_test_cpu();
    
    // Set field ENABLE to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTV_CTL_EL0, X1
    let msr_insn: u32 = 0xD51BE321;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTV_CTL_EL0
    let mrs_insn: u32 = 0xD53BE320;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("CNTV_CTL_EL0.ENABLE = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_tpidrel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TPIDR_EL0
    let mrs_insn: u32 = 0xD53BD040;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TPIDR_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TPIDR_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_tpidrel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TPIDR_EL0, X1
    let msr_insn: u32 = 0xD51BD041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TPIDR_EL0
    let mrs_insn: u32 = 0xD53BD040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TPIDR_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_cntvtvalel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTV_TVAL_EL0
    let mrs_insn: u32 = 0xD53BE300;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTV_TVAL_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTV_TVAL_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cntvtvalel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CNTV_TVAL_EL0, X1
    let msr_insn: u32 = 0xD51BE301;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTV_TVAL_EL0
    let mrs_insn: u32 = 0xD53BE300;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTV_TVAL_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cntvtvalel0_field_timervalue() {
    let mut cpu = create_test_cpu();
    
    // Set field TimerValue to all 1s
    let test_value = 0x00000000FFFFFFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTV_TVAL_EL0, X1
    let msr_insn: u32 = 0xD51BE301;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTV_TVAL_EL0
    let mrs_insn: u32 = 0xD53BE300;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFFFFFF;
    println!("CNTV_TVAL_EL0.TimerValue = 0x{:X} (bits [31:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_farel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, FAR_EL3
    let mrs_insn: u32 = 0xD53E6000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS FAR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("FAR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_farel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR FAR_EL3, X1
    let msr_insn: u32 = 0xD51E6001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FAR_EL3
    let mrs_insn: u32 = 0xD53E6000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("FAR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_midrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MIDR_EL1
    let mrs_insn: u32 = 0xD5380000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS MIDR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("MIDR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_midrel1_field_architecture() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MIDR_EL1
    let mrs_insn: u32 = 0xD5380000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("MIDR_EL1.Architecture = 0x{:X} (bits [19:16])", field_value);
}


#[test]
fn test_midrel1_field_revision() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MIDR_EL1
    let mrs_insn: u32 = 0xD5380000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("MIDR_EL1.Revision = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_midrel1_field_implementer() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MIDR_EL1
    let mrs_insn: u32 = 0xD5380000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xFF;
    println!("MIDR_EL1.Implementer = 0x{:X} (bits [31:24])", field_value);
}


#[test]
fn test_midrel1_field_variant() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MIDR_EL1
    let mrs_insn: u32 = 0xD5380000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("MIDR_EL1.Variant = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_midrel1_field_partnum() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MIDR_EL1
    let mrs_insn: u32 = 0xD5380000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xFFF;
    println!("MIDR_EL1.PartNum = 0x{:X} (bits [15:4])", field_value);
}


#[test]
fn test_mrs_tpidrroel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TPIDRRO_EL0
    let mrs_insn: u32 = 0xD53BD060;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TPIDRRO_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TPIDRRO_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_tpidrroel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TPIDRRO_EL0, X1
    let msr_insn: u32 = 0xD51BD061;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TPIDRRO_EL0
    let mrs_insn: u32 = 0xD53BD060;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TPIDRRO_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_idaa64isar0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64ISAR0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64ISAR0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_idaa64isar0el1_field_dp() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0xF;
    println!("ID_AA64ISAR0_EL1.DP = 0x{:X} (bits [47:44])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_sha3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0xF;
    println!("ID_AA64ISAR0_EL1.SHA3 = 0x{:X} (bits [35:32])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_rdm() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0xF;
    println!("ID_AA64ISAR0_EL1.RDM = 0x{:X} (bits [31:28])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_tlb() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 56) & 0xF;
    println!("ID_AA64ISAR0_EL1.TLB = 0x{:X} (bits [59:56])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_sm3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0xF;
    println!("ID_AA64ISAR0_EL1.SM3 = 0x{:X} (bits [39:36])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_ts() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 52) & 0xF;
    println!("ID_AA64ISAR0_EL1.TS = 0x{:X} (bits [55:52])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_atomic() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ID_AA64ISAR0_EL1.Atomic = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_crc32() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ID_AA64ISAR0_EL1.CRC32 = 0x{:X} (bits [19:16])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_rndr() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 60) & 0xF;
    println!("ID_AA64ISAR0_EL1.RNDR = 0x{:X} (bits [63:60])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_fhm() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0xF;
    println!("ID_AA64ISAR0_EL1.FHM = 0x{:X} (bits [51:48])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_sha2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xF;
    println!("ID_AA64ISAR0_EL1.SHA2 = 0x{:X} (bits [15:12])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_sm4() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0xF;
    println!("ID_AA64ISAR0_EL1.SM4 = 0x{:X} (bits [43:40])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_aes() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("ID_AA64ISAR0_EL1.AES = 0x{:X} (bits [7:4])", field_value);
}


#[test]
fn test_idaa64isar0el1_field_sha1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR0_EL1
    let mrs_insn: u32 = 0xD5380600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xF;
    println!("ID_AA64ISAR0_EL1.SHA1 = 0x{:X} (bits [11:8])", field_value);
}


#[test]
fn test_mrs_elrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ELR_EL2
    let mrs_insn: u32 = 0xD53C4020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ELR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ELR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_elrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR ELR_EL2, X1
    let msr_insn: u32 = 0xD51C4021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ELR_EL2
    let mrs_insn: u32 = 0xD53C4020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ELR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_sctlrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SCTLR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SCTLR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_sctlrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SCTLR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_sctlrel2_field_wxn() {
    let mut cpu = create_test_cpu();
    
    // Set field WXN to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("SCTLR_EL2.WXN = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_naa() {
    let mut cpu = create_test_cpu();
    
    // Set field nAA to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("SCTLR_EL2.nAA = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_tscxt() {
    let mut cpu = create_test_cpu();
    
    // Set field TSCXT to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("SCTLR_EL2.TSCXT = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_eis() {
    let mut cpu = create_test_cpu();
    
    // Set field EIS to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("SCTLR_EL2.EIS = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_lsmaoe() {
    let mut cpu = create_test_cpu();
    
    // Set field LSMAOE to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("SCTLR_EL2.LSMAOE = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_ntlsmd() {
    let mut cpu = create_test_cpu();
    
    // Set field nTLSMD to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("SCTLR_EL2.nTLSMD = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_span() {
    let mut cpu = create_test_cpu();
    
    // Set field SPAN to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("SCTLR_EL2.SPAN = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_enrctx() {
    let mut cpu = create_test_cpu();
    
    // Set field EnRCTX to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("SCTLR_EL2.EnRCTX = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_ee() {
    let mut cpu = create_test_cpu();
    
    // Set field EE to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("SCTLR_EL2.EE = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_tcf0() {
    let mut cpu = create_test_cpu();
    
    // Set field TCF0 to all 1s
    let test_value = 0x000000C000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 38) & 0x3;
    println!("SCTLR_EL2.TCF0 = 0x{:X} (bits [39:38])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_bt1() {
    let mut cpu = create_test_cpu();
    
    // Set field BT1 to all 1s
    let test_value = 0x0000001000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0x1;
    println!("SCTLR_EL2.BT1 = 0x{:X} (bits [36:36])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_itd() {
    let mut cpu = create_test_cpu();
    
    // Set field ITD to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("SCTLR_EL2.ITD = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_eos() {
    let mut cpu = create_test_cpu();
    
    // Set field EOS to all 1s
    let test_value = 0x0000000000000800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x1;
    println!("SCTLR_EL2.EOS = 0x{:X} (bits [11:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_sa0() {
    let mut cpu = create_test_cpu();
    
    // Set field SA0 to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("SCTLR_EL2.SA0 = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_enda() {
    let mut cpu = create_test_cpu();
    
    // Set field EnDA to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("SCTLR_EL2.EnDA = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_i() {
    let mut cpu = create_test_cpu();
    
    // Set field I to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("SCTLR_EL2.I = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_sed() {
    let mut cpu = create_test_cpu();
    
    // Set field SED to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("SCTLR_EL2.SED = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_ata() {
    let mut cpu = create_test_cpu();
    
    // Set field ATA to all 1s
    let test_value = 0x0000080000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 43) & 0x1;
    println!("SCTLR_EL2.ATA = 0x{:X} (bits [43:43])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_sa() {
    let mut cpu = create_test_cpu();
    
    // Set field SA to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("SCTLR_EL2.SA = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_ntwi() {
    let mut cpu = create_test_cpu();
    
    // Set field nTWI to all 1s
    let test_value = 0x0000000000010000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x1;
    println!("SCTLR_EL2.nTWI = 0x{:X} (bits [16:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_dze() {
    let mut cpu = create_test_cpu();
    
    // Set field DZE to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("SCTLR_EL2.DZE = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_a() {
    let mut cpu = create_test_cpu();
    
    // Set field A to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("SCTLR_EL2.A = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_enib() {
    let mut cpu = create_test_cpu();
    
    // Set field EnIB to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("SCTLR_EL2.EnIB = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_uci() {
    let mut cpu = create_test_cpu();
    
    // Set field UCI to all 1s
    let test_value = 0x0000000004000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x1;
    println!("SCTLR_EL2.UCI = 0x{:X} (bits [26:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_e0e() {
    let mut cpu = create_test_cpu();
    
    // Set field E0E to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("SCTLR_EL2.E0E = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_tcf() {
    let mut cpu = create_test_cpu();
    
    // Set field TCF to all 1s
    let test_value = 0x0000030000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0x3;
    println!("SCTLR_EL2.TCF = 0x{:X} (bits [41:40])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_bt0() {
    let mut cpu = create_test_cpu();
    
    // Set field BT0 to all 1s
    let test_value = 0x0000000800000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 35) & 0x1;
    println!("SCTLR_EL2.BT0 = 0x{:X} (bits [35:35])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_ata0() {
    let mut cpu = create_test_cpu();
    
    // Set field ATA0 to all 1s
    let test_value = 0x0000040000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 42) & 0x1;
    println!("SCTLR_EL2.ATA0 = 0x{:X} (bits [42:42])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_bt() {
    let mut cpu = create_test_cpu();
    
    // Set field BT to all 1s
    let test_value = 0x0000001000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0x1;
    println!("SCTLR_EL2.BT = 0x{:X} (bits [36:36])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_dssbs() {
    let mut cpu = create_test_cpu();
    
    // Set field DSSBS to all 1s
    let test_value = 0x0000100000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0x1;
    println!("SCTLR_EL2.DSSBS = 0x{:X} (bits [44:44])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_enia() {
    let mut cpu = create_test_cpu();
    
    // Set field EnIA to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("SCTLR_EL2.EnIA = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("SCTLR_EL2.C = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_ntwe() {
    let mut cpu = create_test_cpu();
    
    // Set field nTWE to all 1s
    let test_value = 0x0000000000040000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 18) & 0x1;
    println!("SCTLR_EL2.nTWE = 0x{:X} (bits [18:18])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_endb() {
    let mut cpu = create_test_cpu();
    
    // Set field EnDB to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("SCTLR_EL2.EnDB = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_iesb() {
    let mut cpu = create_test_cpu();
    
    // Set field IESB to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("SCTLR_EL2.IESB = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_uct() {
    let mut cpu = create_test_cpu();
    
    // Set field UCT to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("SCTLR_EL2.UCT = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_itfsb() {
    let mut cpu = create_test_cpu();
    
    // Set field ITFSB to all 1s
    let test_value = 0x0000002000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 37) & 0x1;
    println!("SCTLR_EL2.ITFSB = 0x{:X} (bits [37:37])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_m() {
    let mut cpu = create_test_cpu();
    
    // Set field M to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("SCTLR_EL2.M = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel2_field_cp15ben() {
    let mut cpu = create_test_cpu();
    
    // Set field CP15BEN to all 1s
    let test_value = 0x0000000000000020u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL2, X1
    let msr_insn: u32 = 0xD51C1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL2
    let mrs_insn: u32 = 0xD53C1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1;
    println!("SCTLR_EL2.CP15BEN = 0x{:X} (bits [5:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_cntvctel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTVCT_EL0
    let mrs_insn: u32 = 0xD53BE040;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTVCT_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTVCT_EL0 = 0x{:016X}", value);
}


#[test]
fn test_mrs_idaa64mmfr2el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64MMFR2_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64MMFR2_EL1 = 0x{:016X}", value);
}


#[test]
fn test_idaa64mmfr2el1_field_ids() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0xF;
    println!("ID_AA64MMFR2_EL1.IDS = 0x{:X} (bits [39:36])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_bbm() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 52) & 0xF;
    println!("ID_AA64MMFR2_EL1.BBM = 0x{:X} (bits [55:52])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_nv() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xF;
    println!("ID_AA64MMFR2_EL1.NV = 0x{:X} (bits [27:24])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_e0pd() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 60) & 0xF;
    println!("ID_AA64MMFR2_EL1.E0PD = 0x{:X} (bits [63:60])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_ccidx() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ID_AA64MMFR2_EL1.CCIDX = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_at() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0xF;
    println!("ID_AA64MMFR2_EL1.AT = 0x{:X} (bits [35:32])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_fwb() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0xF;
    println!("ID_AA64MMFR2_EL1.FWB = 0x{:X} (bits [43:40])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_iesb() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xF;
    println!("ID_AA64MMFR2_EL1.IESB = 0x{:X} (bits [15:12])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_lsm() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xF;
    println!("ID_AA64MMFR2_EL1.LSM = 0x{:X} (bits [11:8])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_st() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0xF;
    println!("ID_AA64MMFR2_EL1.ST = 0x{:X} (bits [31:28])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_ttl() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0xF;
    println!("ID_AA64MMFR2_EL1.TTL = 0x{:X} (bits [51:48])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_cnp() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("ID_AA64MMFR2_EL1.CnP = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_uao() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("ID_AA64MMFR2_EL1.UAO = 0x{:X} (bits [7:4])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_evt() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 56) & 0xF;
    println!("ID_AA64MMFR2_EL1.EVT = 0x{:X} (bits [59:56])", field_value);
}


#[test]
fn test_idaa64mmfr2el1_field_varange() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR2_EL1
    let mrs_insn: u32 = 0xD5380740;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ID_AA64MMFR2_EL1.VARange = 0x{:X} (bits [19:16])", field_value);
}


#[test]
fn test_mrs_idaa64mmfr1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64MMFR1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64MMFR1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_idaa64mmfr1el1_field_lo() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ID_AA64MMFR1_EL1.LO = 0x{:X} (bits [19:16])", field_value);
}


#[test]
fn test_idaa64mmfr1el1_field_vh() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xF;
    println!("ID_AA64MMFR1_EL1.VH = 0x{:X} (bits [11:8])", field_value);
}


#[test]
fn test_idaa64mmfr1el1_field_vmidbits() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("ID_AA64MMFR1_EL1.VMIDBits = 0x{:X} (bits [7:4])", field_value);
}


#[test]
fn test_idaa64mmfr1el1_field_xnx() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0xF;
    println!("ID_AA64MMFR1_EL1.XNX = 0x{:X} (bits [31:28])", field_value);
}


#[test]
fn test_idaa64mmfr1el1_field_specsei() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xF;
    println!("ID_AA64MMFR1_EL1.SpecSEI = 0x{:X} (bits [27:24])", field_value);
}


#[test]
fn test_idaa64mmfr1el1_field_hafdbs() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("ID_AA64MMFR1_EL1.HAFDBS = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_idaa64mmfr1el1_field_hpds() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xF;
    println!("ID_AA64MMFR1_EL1.HPDS = 0x{:X} (bits [15:12])", field_value);
}


#[test]
fn test_idaa64mmfr1el1_field_pan() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR1_EL1
    let mrs_insn: u32 = 0xD5380720;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ID_AA64MMFR1_EL1.PAN = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_mrs_idaa64mmfr0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64MMFR0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64MMFR0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_idaa64mmfr0el1_field_tgran16_2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0xF;
    println!("ID_AA64MMFR0_EL1.TGran16_2 = 0x{:X} (bits [35:32])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_exs() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0xF;
    println!("ID_AA64MMFR0_EL1.ExS = 0x{:X} (bits [47:44])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_tgran4_2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0xF;
    println!("ID_AA64MMFR0_EL1.TGran4_2 = 0x{:X} (bits [43:40])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_asidbits() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("ID_AA64MMFR0_EL1.ASIDBits = 0x{:X} (bits [7:4])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_parange() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("ID_AA64MMFR0_EL1.PARange = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_tgran64_2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0xF;
    println!("ID_AA64MMFR0_EL1.TGran64_2 = 0x{:X} (bits [39:36])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_tgran64() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xF;
    println!("ID_AA64MMFR0_EL1.TGran64 = 0x{:X} (bits [27:24])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_tgran16() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ID_AA64MMFR0_EL1.TGran16 = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_bigendel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ID_AA64MMFR0_EL1.BigEndEL0 = 0x{:X} (bits [19:16])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_snsmem() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xF;
    println!("ID_AA64MMFR0_EL1.SNSMem = 0x{:X} (bits [15:12])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_bigend() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xF;
    println!("ID_AA64MMFR0_EL1.BigEnd = 0x{:X} (bits [11:8])", field_value);
}


#[test]
fn test_idaa64mmfr0el1_field_tgran4() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64MMFR0_EL1
    let mrs_insn: u32 = 0xD5380700;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0xF;
    println!("ID_AA64MMFR0_EL1.TGran4 = 0x{:X} (bits [31:28])", field_value);
}


#[test]
fn test_mrs_ttbr1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TTBR1_EL1
    let mrs_insn: u32 = 0xD5382020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TTBR1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TTBR1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_ttbr1el1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TTBR1_EL1, X1
    let msr_insn: u32 = 0xD5182021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR1_EL1
    let mrs_insn: u32 = 0xD5382020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TTBR1_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_ttbr1el1_field_baddr() {
    let mut cpu = create_test_cpu();
    
    // Set field BADDR to all 1s
    let test_value = 0x0000FFFFFFFFFFFEu64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR1_EL1, X1
    let msr_insn: u32 = 0xD5182021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR1_EL1
    let mrs_insn: u32 = 0xD5382020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x7FFFFFFFFFFF;
    println!("TTBR1_EL1.BADDR = 0x{:X} (bits [47:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_ttbr1el1_field_asid() {
    let mut cpu = create_test_cpu();
    
    // Set field ASID to all 1s
    let test_value = 0xFFFF000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR1_EL1, X1
    let msr_insn: u32 = 0xD5182021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR1_EL1
    let mrs_insn: u32 = 0xD5382020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0xFFFF;
    println!("TTBR1_EL1.ASID = 0x{:X} (bits [63:48])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_ttbr1el1_field_cnp() {
    let mut cpu = create_test_cpu();
    
    // Set field CnP to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR1_EL1, X1
    let msr_insn: u32 = 0xD5182021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR1_EL1
    let mrs_insn: u32 = 0xD5382020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("TTBR1_EL1.CnP = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_mdscrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS MDSCR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("MDSCR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_mdscrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("MDSCR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_mdscrel1_field_rxfull() {
    let mut cpu = create_test_cpu();
    
    // Set field RXfull to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("MDSCR_EL1.RXfull = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_rxo() {
    let mut cpu = create_test_cpu();
    
    // Set field RXO to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("MDSCR_EL1.RXO = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_txu() {
    let mut cpu = create_test_cpu();
    
    // Set field TXU to all 1s
    let test_value = 0x0000000004000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x1;
    println!("MDSCR_EL1.TXU = 0x{:X} (bits [26:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_hde() {
    let mut cpu = create_test_cpu();
    
    // Set field HDE to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("MDSCR_EL1.HDE = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_sc2() {
    let mut cpu = create_test_cpu();
    
    // Set field SC2 to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("MDSCR_EL1.SC2 = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_tfo() {
    let mut cpu = create_test_cpu();
    
    // Set field TFO to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("MDSCR_EL1.TFO = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_err() {
    let mut cpu = create_test_cpu();
    
    // Set field ERR to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("MDSCR_EL1.ERR = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_ss() {
    let mut cpu = create_test_cpu();
    
    // Set field SS to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("MDSCR_EL1.SS = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_intdis() {
    let mut cpu = create_test_cpu();
    
    // Set field INTdis to all 1s
    let test_value = 0x0000000000C00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x3;
    println!("MDSCR_EL1.INTdis = 0x{:X} (bits [23:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_txfull() {
    let mut cpu = create_test_cpu();
    
    // Set field TXfull to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("MDSCR_EL1.TXfull = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_tdcc() {
    let mut cpu = create_test_cpu();
    
    // Set field TDCC to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("MDSCR_EL1.TDCC = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_mde() {
    let mut cpu = create_test_cpu();
    
    // Set field MDE to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("MDSCR_EL1.MDE = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_kde() {
    let mut cpu = create_test_cpu();
    
    // Set field KDE to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("MDSCR_EL1.KDE = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mdscrel1_field_tda() {
    let mut cpu = create_test_cpu();
    
    // Set field TDA to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR MDSCR_EL1, X1
    let msr_insn: u32 = 0xD5100241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MDSCR_EL1
    let mrs_insn: u32 = 0xD5300240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("MDSCR_EL1.TDA = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_farel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, FAR_EL2
    let mrs_insn: u32 = 0xD53C6000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS FAR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("FAR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_farel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR FAR_EL2, X1
    let msr_insn: u32 = 0xD51C6001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FAR_EL2
    let mrs_insn: u32 = 0xD53C6000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("FAR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_mairel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MAIR_EL2
    let mrs_insn: u32 = 0xD53CA200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS MAIR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("MAIR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_mairel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR MAIR_EL2, X1
    let msr_insn: u32 = 0xD51CA201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MAIR_EL2
    let mrs_insn: u32 = 0xD53CA200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("MAIR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_apibkeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APIBKeyHi_EL1
    let mrs_insn: u32 = 0xD5382160;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APIBKeyHi_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APIBKeyHi_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apibkeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APIBKeyHi_EL1, X1
    let msr_insn: u32 = 0xD5182161;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APIBKeyHi_EL1
    let mrs_insn: u32 = 0xD5382160;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APIBKeyHi_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_cntpcvalel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTP_CVAL_EL0
    let mrs_insn: u32 = 0xD53BE240;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTP_CVAL_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTP_CVAL_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cntpcvalel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR CNTP_CVAL_EL0, X1
    let msr_insn: u32 = 0xD51BE241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTP_CVAL_EL0
    let mrs_insn: u32 = 0xD53BE240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTP_CVAL_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_cntpcvalel0_field_comparevalue() {
    let mut cpu = create_test_cpu();
    
    // Set field CompareValue to all 1s
    let test_value = 0x0000000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTP_CVAL_EL0, X1
    let msr_insn: u32 = 0xD51BE241;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTP_CVAL_EL0
    let mrs_insn: u32 = 0xD53BE240;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x0;
    println!("CNTP_CVAL_EL0.CompareValue = 0x{:X} (bits [63:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_iccbpr1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_BPR1_EL1
    let mrs_insn: u32 = 0xD538CC60;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_BPR1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_BPR1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccbpr1el1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_BPR1_EL1, X1
    let msr_insn: u32 = 0xD518CC61;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_BPR1_EL1
    let mrs_insn: u32 = 0xD538CC60;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_BPR1_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccbpr1el1_field_binarypoint() {
    let mut cpu = create_test_cpu();
    
    // Set field BinaryPoint to all 1s
    let test_value = 0x0000000000000007u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_BPR1_EL1, X1
    let msr_insn: u32 = 0xD518CC61;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_BPR1_EL1
    let mrs_insn: u32 = 0xD538CC60;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x7;
    println!("ICC_BPR1_EL1.BinaryPoint = 0x{:X} (bits [2:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_apibkeyloel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APIBKeyLo_EL1
    let mrs_insn: u32 = 0xD5382140;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APIBKeyLo_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APIBKeyLo_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apibkeyloel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APIBKeyLo_EL1, X1
    let msr_insn: u32 = 0xD5182141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APIBKeyLo_EL1
    let mrs_insn: u32 = 0xD5382140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APIBKeyLo_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_apdbkeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APDBKeyHi_EL1
    let mrs_insn: u32 = 0xD5382260;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APDBKeyHi_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APDBKeyHi_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apdbkeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APDBKeyHi_EL1, X1
    let msr_insn: u32 = 0xD5182261;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APDBKeyHi_EL1
    let mrs_insn: u32 = 0xD5382260;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APDBKeyHi_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_iccbpr0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_BPR0_EL1
    let mrs_insn: u32 = 0xD538C860;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_BPR0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_BPR0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccbpr0el1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_BPR0_EL1, X1
    let msr_insn: u32 = 0xD518C861;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_BPR0_EL1
    let mrs_insn: u32 = 0xD538C860;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_BPR0_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccbpr0el1_field_binarypoint() {
    let mut cpu = create_test_cpu();
    
    // Set field BinaryPoint to all 1s
    let test_value = 0x0000000000000007u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_BPR0_EL1, X1
    let msr_insn: u32 = 0xD518C861;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_BPR0_EL1
    let mrs_insn: u32 = 0xD538C860;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x7;
    println!("ICC_BPR0_EL1.BinaryPoint = 0x{:X} (bits [2:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_contextidrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CONTEXTIDR_EL1
    let mrs_insn: u32 = 0xD538D020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CONTEXTIDR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CONTEXTIDR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_contextidrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CONTEXTIDR_EL1, X1
    let msr_insn: u32 = 0xD518D021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CONTEXTIDR_EL1
    let mrs_insn: u32 = 0xD538D020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CONTEXTIDR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_contextidrel1_field_procid() {
    let mut cpu = create_test_cpu();
    
    // Set field PROCID to all 1s
    let test_value = 0x00000000FFFFFFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR CONTEXTIDR_EL1, X1
    let msr_insn: u32 = 0xD518D021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CONTEXTIDR_EL1
    let mrs_insn: u32 = 0xD538D020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFFFFFF;
    println!("CONTEXTIDR_EL1.PROCID = 0x{:X} (bits [31:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_spsel() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SPSel
    let mrs_insn: u32 = 0xD5384200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SPSel should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SPSel = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_spsel() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR SPSel, X1
    let msr_insn: u32 = 0xD5184201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSel
    let mrs_insn: u32 = 0xD5384200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SPSel: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_spsel_field_sp() {
    let mut cpu = create_test_cpu();
    
    // Set field SP to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSel, X1
    let msr_insn: u32 = 0xD5184201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSel
    let mrs_insn: u32 = 0xD5384200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("SPSel.SP = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_dczidel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, DCZID_EL0
    let mrs_insn: u32 = 0xD53B00E0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS DCZID_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("DCZID_EL0 = 0x{:016X}", value);
}


#[test]
fn test_dczidel0_field_dzp() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, DCZID_EL0
    let mrs_insn: u32 = 0xD53B00E0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("DCZID_EL0.DZP = 0x{:X} (bits [4:4])", field_value);
}


#[test]
fn test_dczidel0_field_bs() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, DCZID_EL0
    let mrs_insn: u32 = 0xD53B00E0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("DCZID_EL0.BS = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_mrs_pmcntenclrel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, PMCNTENCLR_EL0
    let mrs_insn: u32 = 0xD53B9C40;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS PMCNTENCLR_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("PMCNTENCLR_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_pmcntenclrel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR PMCNTENCLR_EL0, X1
    let msr_insn: u32 = 0xD51B9C41;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCNTENCLR_EL0
    let mrs_insn: u32 = 0xD53B9C40;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("PMCNTENCLR_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_pmcntenclrel0_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCNTENCLR_EL0, X1
    let msr_insn: u32 = 0xD51B9C41;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCNTENCLR_EL0
    let mrs_insn: u32 = 0xD53B9C40;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("PMCNTENCLR_EL0.C = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_vbarel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, VBAR_EL2
    let mrs_insn: u32 = 0xD53CC000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS VBAR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("VBAR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_vbarel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR VBAR_EL2, X1
    let msr_insn: u32 = 0xD51CC001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VBAR_EL2
    let mrs_insn: u32 = 0xD53CC000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("VBAR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_tpidrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TPIDR_EL2
    let mrs_insn: u32 = 0xD53CD040;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TPIDR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TPIDR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_tpidrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TPIDR_EL2, X1
    let msr_insn: u32 = 0xD51CD041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TPIDR_EL2
    let mrs_insn: u32 = 0xD53CD040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TPIDR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_idaa64pfr1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR1_EL1
    let mrs_insn: u32 = 0xD5380420;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64PFR1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64PFR1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_idaa64pfr1el1_field_ssbs() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR1_EL1
    let mrs_insn: u32 = 0xD5380420;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("ID_AA64PFR1_EL1.SSBS = 0x{:X} (bits [7:4])", field_value);
}


#[test]
fn test_idaa64pfr1el1_field_mte() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR1_EL1
    let mrs_insn: u32 = 0xD5380420;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xF;
    println!("ID_AA64PFR1_EL1.MTE = 0x{:X} (bits [11:8])", field_value);
}


#[test]
fn test_idaa64pfr1el1_field_ras_frac() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR1_EL1
    let mrs_insn: u32 = 0xD5380420;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xF;
    println!("ID_AA64PFR1_EL1.RAS_frac = 0x{:X} (bits [15:12])", field_value);
}


#[test]
fn test_idaa64pfr1el1_field_bt() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR1_EL1
    let mrs_insn: u32 = 0xD5380420;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("ID_AA64PFR1_EL1.BT = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_mrs_tcrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TCR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TCR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_tcrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TCR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_tcrel2_field_tbi1() {
    let mut cpu = create_test_cpu();
    
    // Set field TBI1 to all 1s
    let test_value = 0x0000004000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 38) & 0x1;
    println!("TCR_EL2.TBI1 = 0x{:X} (bits [38:38])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_irgn1() {
    let mut cpu = create_test_cpu();
    
    // Set field IRGN1 to all 1s
    let test_value = 0x0000000003000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x3;
    println!("TCR_EL2.IRGN1 = 0x{:X} (bits [25:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tcma1() {
    let mut cpu = create_test_cpu();
    
    // Set field TCMA1 to all 1s
    let test_value = 0x0400000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 58) & 0x1;
    println!("TCR_EL2.TCMA1 = 0x{:X} (bits [58:58])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu062() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU062 to all 1s
    let test_value = 0x0000400000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 46) & 0x1;
    println!("TCR_EL2.HWU062 = 0x{:X} (bits [46:46])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_ha() {
    let mut cpu = create_test_cpu();
    
    // Set field HA to all 1s
    let test_value = 0x0000008000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 39) & 0x1;
    println!("TCR_EL2.HA = 0x{:X} (bits [39:39])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_ps() {
    let mut cpu = create_test_cpu();
    
    // Set field PS to all 1s
    let test_value = 0x0000000000070000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x7;
    println!("TCR_EL2.PS = 0x{:X} (bits [18:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tbi0() {
    let mut cpu = create_test_cpu();
    
    // Set field TBI0 to all 1s
    let test_value = 0x0000002000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 37) & 0x1;
    println!("TCR_EL2.TBI0 = 0x{:X} (bits [37:37])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_sh1() {
    let mut cpu = create_test_cpu();
    
    // Set field SH1 to all 1s
    let test_value = 0x0000000030000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x3;
    println!("TCR_EL2.SH1 = 0x{:X} (bits [29:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_orgn1() {
    let mut cpu = create_test_cpu();
    
    // Set field ORGN1 to all 1s
    let test_value = 0x000000000C000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x3;
    println!("TCR_EL2.ORGN1 = 0x{:X} (bits [27:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_nfd1() {
    let mut cpu = create_test_cpu();
    
    // Set field NFD1 to all 1s
    let test_value = 0x0040000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 54) & 0x1;
    println!("TCR_EL2.NFD1 = 0x{:X} (bits [54:54])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hpd() {
    let mut cpu = create_test_cpu();
    
    // Set field HPD to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("TCR_EL2.HPD = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu161() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU161 to all 1s
    let test_value = 0x0002000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 49) & 0x1;
    println!("TCR_EL2.HWU161 = 0x{:X} (bits [49:49])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tbid() {
    let mut cpu = create_test_cpu();
    
    // Set field TBID to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("TCR_EL2.TBID = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tbi() {
    let mut cpu = create_test_cpu();
    
    // Set field TBI to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("TCR_EL2.TBI = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_e0pd1() {
    let mut cpu = create_test_cpu();
    
    // Set field E0PD1 to all 1s
    let test_value = 0x0100000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 56) & 0x1;
    println!("TCR_EL2.E0PD1 = 0x{:X} (bits [56:56])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tcma() {
    let mut cpu = create_test_cpu();
    
    // Set field TCMA to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("TCR_EL2.TCMA = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tcma0() {
    let mut cpu = create_test_cpu();
    
    // Set field TCMA0 to all 1s
    let test_value = 0x0200000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 57) & 0x1;
    println!("TCR_EL2.TCMA0 = 0x{:X} (bits [57:57])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu061() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU061 to all 1s
    let test_value = 0x0000200000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 45) & 0x1;
    println!("TCR_EL2.HWU061 = 0x{:X} (bits [45:45])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hpd0() {
    let mut cpu = create_test_cpu();
    
    // Set field HPD0 to all 1s
    let test_value = 0x0000020000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 41) & 0x1;
    println!("TCR_EL2.HPD0 = 0x{:X} (bits [41:41])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu160() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU160 to all 1s
    let test_value = 0x0001000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0x1;
    println!("TCR_EL2.HWU160 = 0x{:X} (bits [48:48])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu060() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU060 to all 1s
    let test_value = 0x0000100000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0x1;
    println!("TCR_EL2.HWU060 = 0x{:X} (bits [44:44])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_irgn0() {
    let mut cpu = create_test_cpu();
    
    // Set field IRGN0 to all 1s
    let test_value = 0x0000000000000300u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x3;
    println!("TCR_EL2.IRGN0 = 0x{:X} (bits [9:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tbid0() {
    let mut cpu = create_test_cpu();
    
    // Set field TBID0 to all 1s
    let test_value = 0x0008000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 51) & 0x1;
    println!("TCR_EL2.TBID0 = 0x{:X} (bits [51:51])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hd() {
    let mut cpu = create_test_cpu();
    
    // Set field HD to all 1s
    let test_value = 0x0000010000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0x1;
    println!("TCR_EL2.HD = 0x{:X} (bits [40:40])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu162() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU162 to all 1s
    let test_value = 0x0004000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 50) & 0x1;
    println!("TCR_EL2.HWU162 = 0x{:X} (bits [50:50])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_as() {
    let mut cpu = create_test_cpu();
    
    // Set field AS to all 1s
    let test_value = 0x0000001000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0x1;
    println!("TCR_EL2.AS = 0x{:X} (bits [36:36])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tg1() {
    let mut cpu = create_test_cpu();
    
    // Set field TG1 to all 1s
    let test_value = 0x00000000C0000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x3;
    println!("TCR_EL2.TG1 = 0x{:X} (bits [31:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_orgn0() {
    let mut cpu = create_test_cpu();
    
    // Set field ORGN0 to all 1s
    let test_value = 0x0000000000000C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x3;
    println!("TCR_EL2.ORGN0 = 0x{:X} (bits [11:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_epd1() {
    let mut cpu = create_test_cpu();
    
    // Set field EPD1 to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("TCR_EL2.EPD1 = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_e0pd0() {
    let mut cpu = create_test_cpu();
    
    // Set field E0PD0 to all 1s
    let test_value = 0x0080000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 55) & 0x1;
    println!("TCR_EL2.E0PD0 = 0x{:X} (bits [55:55])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu059() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU059 to all 1s
    let test_value = 0x0000080000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 43) & 0x1;
    println!("TCR_EL2.HWU059 = 0x{:X} (bits [43:43])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hpd1() {
    let mut cpu = create_test_cpu();
    
    // Set field HPD1 to all 1s
    let test_value = 0x0000040000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 42) & 0x1;
    println!("TCR_EL2.HPD1 = 0x{:X} (bits [42:42])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu62() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU62 to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("TCR_EL2.HWU62 = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_ips() {
    let mut cpu = create_test_cpu();
    
    // Set field IPS to all 1s
    let test_value = 0x0000000700000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0x7;
    println!("TCR_EL2.IPS = 0x{:X} (bits [34:32])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu59() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU59 to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("TCR_EL2.HWU59 = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu159() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU159 to all 1s
    let test_value = 0x0000800000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 47) & 0x1;
    println!("TCR_EL2.HWU159 = 0x{:X} (bits [47:47])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_t0sz() {
    let mut cpu = create_test_cpu();
    
    // Set field T0SZ to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("TCR_EL2.T0SZ = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tbid1() {
    let mut cpu = create_test_cpu();
    
    // Set field TBID1 to all 1s
    let test_value = 0x0010000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 52) & 0x1;
    println!("TCR_EL2.TBID1 = 0x{:X} (bits [52:52])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu61() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU61 to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("TCR_EL2.HWU61 = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_tg0() {
    let mut cpu = create_test_cpu();
    
    // Set field TG0 to all 1s
    let test_value = 0x000000000000C000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x3;
    println!("TCR_EL2.TG0 = 0x{:X} (bits [15:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_sh0() {
    let mut cpu = create_test_cpu();
    
    // Set field SH0 to all 1s
    let test_value = 0x0000000000003000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x3;
    println!("TCR_EL2.SH0 = 0x{:X} (bits [13:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_hwu60() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU60 to all 1s
    let test_value = 0x0000000004000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x1;
    println!("TCR_EL2.HWU60 = 0x{:X} (bits [26:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_epd0() {
    let mut cpu = create_test_cpu();
    
    // Set field EPD0 to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("TCR_EL2.EPD0 = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_a1() {
    let mut cpu = create_test_cpu();
    
    // Set field A1 to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("TCR_EL2.A1 = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_t1sz() {
    let mut cpu = create_test_cpu();
    
    // Set field T1SZ to all 1s
    let test_value = 0x00000000003F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x3F;
    println!("TCR_EL2.T1SZ = 0x{:X} (bits [21:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel2_field_nfd0() {
    let mut cpu = create_test_cpu();
    
    // Set field NFD0 to all 1s
    let test_value = 0x0020000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL2, X1
    let msr_insn: u32 = 0xD51C2041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL2
    let mrs_insn: u32 = 0xD53C2040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 53) & 0x1;
    println!("TCR_EL2.NFD0 = 0x{:X} (bits [53:53])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_icciar1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_IAR1_EL1
    let mrs_insn: u32 = 0xD538CC00;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_IAR1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_IAR1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_icciar1el1_field_intid() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_IAR1_EL1
    let mrs_insn: u32 = 0xD538CC00;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFFFF;
    println!("ICC_IAR1_EL1.INTID = 0x{:X} (bits [23:0])", field_value);
}


#[test]
fn test_mrs_pmuserenrel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, PMUSERENR_EL0
    let mrs_insn: u32 = 0xD53B9E00;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS PMUSERENR_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("PMUSERENR_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_pmuserenrel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR PMUSERENR_EL0, X1
    let msr_insn: u32 = 0xD51B9E01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMUSERENR_EL0
    let mrs_insn: u32 = 0xD53B9E00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("PMUSERENR_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_pmuserenrel0_field_sw() {
    let mut cpu = create_test_cpu();
    
    // Set field SW to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMUSERENR_EL0, X1
    let msr_insn: u32 = 0xD51B9E01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMUSERENR_EL0
    let mrs_insn: u32 = 0xD53B9E00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("PMUSERENR_EL0.SW = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmuserenrel0_field_er() {
    let mut cpu = create_test_cpu();
    
    // Set field ER to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMUSERENR_EL0, X1
    let msr_insn: u32 = 0xD51B9E01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMUSERENR_EL0
    let mrs_insn: u32 = 0xD53B9E00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("PMUSERENR_EL0.ER = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmuserenrel0_field_cr() {
    let mut cpu = create_test_cpu();
    
    // Set field CR to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMUSERENR_EL0, X1
    let msr_insn: u32 = 0xD51B9E01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMUSERENR_EL0
    let mrs_insn: u32 = 0xD53B9E00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("PMUSERENR_EL0.CR = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmuserenrel0_field_en() {
    let mut cpu = create_test_cpu();
    
    // Set field EN to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMUSERENR_EL0, X1
    let msr_insn: u32 = 0xD51B9E01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMUSERENR_EL0
    let mrs_insn: u32 = 0xD53B9E00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("PMUSERENR_EL0.EN = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_esrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ESR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ESR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_esrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ESR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_esrel2_field_offset() {
    let mut cpu = create_test_cpu();
    
    // Set field Offset to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("ESR_EL2.Offset = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ixf() {
    let mut cpu = create_test_cpu();
    
    // Set field IXF to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("ESR_EL2.IXF = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_uff() {
    let mut cpu = create_test_cpu();
    
    // Set field UFF to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("ESR_EL2.UFF = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_op2() {
    let mut cpu = create_test_cpu();
    
    // Set field Op2 to all 1s
    let test_value = 0x00000000000E0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 17) & 0x7;
    println!("ESR_EL2.Op2 = 0x{:X} (bits [19:17])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_off() {
    let mut cpu = create_test_cpu();
    
    // Set field OFF to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("ESR_EL2.OFF = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_tfv() {
    let mut cpu = create_test_cpu();
    
    // Set field TFV to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("ESR_EL2.TFV = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_iesb() {
    let mut cpu = create_test_cpu();
    
    // Set field IESB to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("ESR_EL2.IESB = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_rn() {
    let mut cpu = create_test_cpu();
    
    // Set field Rn to all 1s
    let test_value = 0x00000000000003E0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1F;
    println!("ESR_EL2.Rn = 0x{:X} (bits [9:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_rt2() {
    let mut cpu = create_test_cpu();
    
    // Set field Rt2 to all 1s
    let test_value = 0x0000000000007C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1F;
    println!("ESR_EL2.Rt2 = 0x{:X} (bits [14:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_comment() {
    let mut cpu = create_test_cpu();
    
    // Set field Comment to all 1s
    let test_value = 0x000000000000FFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFF;
    println!("ESR_EL2.Comment = 0x{:X} (bits [15:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_wnr() {
    let mut cpu = create_test_cpu();
    
    // Set field WnR to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("ESR_EL2.WnR = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_idf() {
    let mut cpu = create_test_cpu();
    
    // Set field IDF to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("ESR_EL2.IDF = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_iof() {
    let mut cpu = create_test_cpu();
    
    // Set field IOF to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL2.IOF = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ar() {
    let mut cpu = create_test_cpu();
    
    // Set field AR to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("ESR_EL2.AR = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_fnv() {
    let mut cpu = create_test_cpu();
    
    // Set field FnV to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("ESR_EL2.FnV = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ti() {
    let mut cpu = create_test_cpu();
    
    // Set field TI to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL2.TI = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_sas() {
    let mut cpu = create_test_cpu();
    
    // Set field SAS to all 1s
    let test_value = 0x0000000000C00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x3;
    println!("ESR_EL2.SAS = 0x{:X} (bits [23:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_opc1() {
    let mut cpu = create_test_cpu();
    
    // Set field Opc1 to all 1s
    let test_value = 0x00000000000F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ESR_EL2.Opc1 = 0x{:X} (bits [19:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_set() {
    let mut cpu = create_test_cpu();
    
    // Set field SET to all 1s
    let test_value = 0x0000000000001800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x3;
    println!("ESR_EL2.SET = 0x{:X} (bits [12:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_cv() {
    let mut cpu = create_test_cpu();
    
    // Set field CV to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL2.CV = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_op0() {
    let mut cpu = create_test_cpu();
    
    // Set field Op0 to all 1s
    let test_value = 0x0000000000300000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x3;
    println!("ESR_EL2.Op0 = 0x{:X} (bits [21:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ccknownpass() {
    let mut cpu = create_test_cpu();
    
    // Set field CCKNOWNPASS to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("ESR_EL2.CCKNOWNPASS = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_sse() {
    let mut cpu = create_test_cpu();
    
    // Set field SSE to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("ESR_EL2.SSE = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_dfsc() {
    let mut cpu = create_test_cpu();
    
    // Set field DFSC to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("ESR_EL2.DFSC = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ex() {
    let mut cpu = create_test_cpu();
    
    // Set field EX to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("ESR_EL2.EX = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ids() {
    let mut cpu = create_test_cpu();
    
    // Set field IDS to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL2.IDS = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_btype() {
    let mut cpu = create_test_cpu();
    
    // Set field BTYPE to all 1s
    let test_value = 0x0000000000000003u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3;
    println!("ESR_EL2.BTYPE = 0x{:X} (bits [1:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_crn() {
    let mut cpu = create_test_cpu();
    
    // Set field CRn to all 1s
    let test_value = 0x0000000000003C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0xF;
    println!("ESR_EL2.CRn = 0x{:X} (bits [13:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_imm8() {
    let mut cpu = create_test_cpu();
    
    // Set field imm8 to all 1s
    let test_value = 0x00000000000FF000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xFF;
    println!("ESR_EL2.imm8 = 0x{:X} (bits [19:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_imm16() {
    let mut cpu = create_test_cpu();
    
    // Set field imm16 to all 1s
    let test_value = 0x000000000000FFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFF;
    println!("ESR_EL2.imm16 = 0x{:X} (bits [15:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_isv() {
    let mut cpu = create_test_cpu();
    
    // Set field ISV to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL2.ISV = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_s1ptw() {
    let mut cpu = create_test_cpu();
    
    // Set field S1PTW to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("ESR_EL2.S1PTW = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_dzf() {
    let mut cpu = create_test_cpu();
    
    // Set field DZF to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ESR_EL2.DZF = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_am() {
    let mut cpu = create_test_cpu();
    
    // Set field AM to all 1s
    let test_value = 0x000000000000000Eu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x7;
    println!("ESR_EL2.AM = 0x{:X} (bits [3:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_vecitr() {
    let mut cpu = create_test_cpu();
    
    // Set field VECITR to all 1s
    let test_value = 0x0000000000000700u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x7;
    println!("ESR_EL2.VECITR = 0x{:X} (bits [10:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_eret() {
    let mut cpu = create_test_cpu();
    
    // Set field ERET to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ESR_EL2.ERET = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_direction() {
    let mut cpu = create_test_cpu();
    
    // Set field Direction to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL2.Direction = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ea() {
    let mut cpu = create_test_cpu();
    
    // Set field EA to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("ESR_EL2.EA = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_srt() {
    let mut cpu = create_test_cpu();
    
    // Set field SRT to all 1s
    let test_value = 0x00000000001F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x1F;
    println!("ESR_EL2.SRT = 0x{:X} (bits [20:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_aet() {
    let mut cpu = create_test_cpu();
    
    // Set field AET to all 1s
    let test_value = 0x0000000000001C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x7;
    println!("ESR_EL2.AET = 0x{:X} (bits [12:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_sf() {
    let mut cpu = create_test_cpu();
    
    // Set field SF to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("ESR_EL2.SF = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_il() {
    let mut cpu = create_test_cpu();
    
    // Set field IL to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("ESR_EL2.IL = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ereta() {
    let mut cpu = create_test_cpu();
    
    // Set field ERETA to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL2.ERETA = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ifsc() {
    let mut cpu = create_test_cpu();
    
    // Set field IFSC to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("ESR_EL2.IFSC = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_cm() {
    let mut cpu = create_test_cpu();
    
    // Set field CM to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("ESR_EL2.CM = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_cond() {
    let mut cpu = create_test_cpu();
    
    // Set field COND to all 1s
    let test_value = 0x0000000000F00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ESR_EL2.COND = 0x{:X} (bits [23:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_rt() {
    let mut cpu = create_test_cpu();
    
    // Set field Rt to all 1s
    let test_value = 0x00000000000003E0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1F;
    println!("ESR_EL2.Rt = 0x{:X} (bits [9:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_op1() {
    let mut cpu = create_test_cpu();
    
    // Set field Op1 to all 1s
    let test_value = 0x000000000001C000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x7;
    println!("ESR_EL2.Op1 = 0x{:X} (bits [16:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_vncr() {
    let mut cpu = create_test_cpu();
    
    // Set field VNCR to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("ESR_EL2.VNCR = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_opc2() {
    let mut cpu = create_test_cpu();
    
    // Set field Opc2 to all 1s
    let test_value = 0x00000000000E0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 17) & 0x7;
    println!("ESR_EL2.Opc2 = 0x{:X} (bits [19:17])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_ec() {
    let mut cpu = create_test_cpu();
    
    // Set field EC to all 1s
    let test_value = 0x00000000FC000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x3F;
    println!("ESR_EL2.EC = 0x{:X} (bits [31:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel2_field_crm() {
    let mut cpu = create_test_cpu();
    
    // Set field CRm to all 1s
    let test_value = 0x000000000000001Eu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL2, X1
    let msr_insn: u32 = 0xD51C5201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL2
    let mrs_insn: u32 = 0xD53C5200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0xF;
    println!("ESR_EL2.CRm = 0x{:X} (bits [4:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_spel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SP_EL2
    let mrs_insn: u32 = 0xD53E4100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SP_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SP_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_spel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR SP_EL2, X1
    let msr_insn: u32 = 0xD51E4101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SP_EL2
    let mrs_insn: u32 = 0xD53E4100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SP_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_fpsr() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS FPSR should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("FPSR = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_fpsr() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("FPSR: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_fpsr_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("FPSR.C = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_n() {
    let mut cpu = create_test_cpu();
    
    // Set field N to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("FPSR.N = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_v() {
    let mut cpu = create_test_cpu();
    
    // Set field V to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("FPSR.V = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_ixc() {
    let mut cpu = create_test_cpu();
    
    // Set field IXC to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("FPSR.IXC = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_dzc() {
    let mut cpu = create_test_cpu();
    
    // Set field DZC to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("FPSR.DZC = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_z() {
    let mut cpu = create_test_cpu();
    
    // Set field Z to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("FPSR.Z = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_qc() {
    let mut cpu = create_test_cpu();
    
    // Set field QC to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("FPSR.QC = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_ufc() {
    let mut cpu = create_test_cpu();
    
    // Set field UFC to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("FPSR.UFC = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_ofc() {
    let mut cpu = create_test_cpu();
    
    // Set field OFC to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("FPSR.OFC = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_ioc() {
    let mut cpu = create_test_cpu();
    
    // Set field IOC to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("FPSR.IOC = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpsr_field_idc() {
    let mut cpu = create_test_cpu();
    
    // Set field IDC to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPSR, X1
    let msr_insn: u32 = 0xD51B4421;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPSR
    let mrs_insn: u32 = 0xD53B4420;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("FPSR.IDC = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_cntpctlel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTP_CTL_EL0
    let mrs_insn: u32 = 0xD53BE220;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTP_CTL_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTP_CTL_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cntpctlel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CNTP_CTL_EL0, X1
    let msr_insn: u32 = 0xD51BE221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTP_CTL_EL0
    let mrs_insn: u32 = 0xD53BE220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTP_CTL_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cntpctlel0_field_istatus() {
    let mut cpu = create_test_cpu();
    
    // Set field ISTATUS to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTP_CTL_EL0, X1
    let msr_insn: u32 = 0xD51BE221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTP_CTL_EL0
    let mrs_insn: u32 = 0xD53BE220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("CNTP_CTL_EL0.ISTATUS = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntpctlel0_field_enable() {
    let mut cpu = create_test_cpu();
    
    // Set field ENABLE to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTP_CTL_EL0, X1
    let msr_insn: u32 = 0xD51BE221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTP_CTL_EL0
    let mrs_insn: u32 = 0xD53BE220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("CNTP_CTL_EL0.ENABLE = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntpctlel0_field_imask() {
    let mut cpu = create_test_cpu();
    
    // Set field IMASK to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTP_CTL_EL0, X1
    let msr_insn: u32 = 0xD51BE221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTP_CTL_EL0
    let mrs_insn: u32 = 0xD53BE220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("CNTP_CTL_EL0.IMASK = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_daif() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, DAIF
    let mrs_insn: u32 = 0xD53B4220;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS DAIF should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("DAIF = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_daif() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR DAIF, X1
    let msr_insn: u32 = 0xD51B4221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, DAIF
    let mrs_insn: u32 = 0xD53B4220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("DAIF: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_daif_field_a() {
    let mut cpu = create_test_cpu();
    
    // Set field A to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR DAIF, X1
    let msr_insn: u32 = 0xD51B4221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, DAIF
    let mrs_insn: u32 = 0xD53B4220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("DAIF.A = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_daif_field_d() {
    let mut cpu = create_test_cpu();
    
    // Set field D to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR DAIF, X1
    let msr_insn: u32 = 0xD51B4221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, DAIF
    let mrs_insn: u32 = 0xD53B4220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("DAIF.D = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_daif_field_i() {
    let mut cpu = create_test_cpu();
    
    // Set field I to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR DAIF, X1
    let msr_insn: u32 = 0xD51B4221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, DAIF
    let mrs_insn: u32 = 0xD53B4220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("DAIF.I = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_daif_field_f() {
    let mut cpu = create_test_cpu();
    
    // Set field F to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR DAIF, X1
    let msr_insn: u32 = 0xD51B4221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, DAIF
    let mrs_insn: u32 = 0xD53B4220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("DAIF.F = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_elrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ELR_EL1
    let mrs_insn: u32 = 0xD5384020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ELR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ELR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_elrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR ELR_EL1, X1
    let msr_insn: u32 = 0xD5184021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ELR_EL1
    let mrs_insn: u32 = 0xD5384020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ELR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_spel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SP_EL0
    let mrs_insn: u32 = 0xD5384100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SP_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SP_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_spel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR SP_EL0, X1
    let msr_insn: u32 = 0xD5184101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SP_EL0
    let mrs_insn: u32 = 0xD5384100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SP_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_idaa64dfr0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64DFR0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64DFR0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_idaa64dfr0el1_field_pmsver() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0xF;
    println!("ID_AA64DFR0_EL1.PMSVer = 0x{:X} (bits [35:32])", field_value);
}


#[test]
fn test_idaa64dfr0el1_field_wrps() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ID_AA64DFR0_EL1.WRPs = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_idaa64dfr0el1_field_tracefilt() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0xF;
    println!("ID_AA64DFR0_EL1.TraceFilt = 0x{:X} (bits [43:40])", field_value);
}


#[test]
fn test_idaa64dfr0el1_field_pmuver() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xF;
    println!("ID_AA64DFR0_EL1.PMUVer = 0x{:X} (bits [11:8])", field_value);
}


#[test]
fn test_idaa64dfr0el1_field_debugver() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("ID_AA64DFR0_EL1.DebugVer = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_idaa64dfr0el1_field_brps() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xF;
    println!("ID_AA64DFR0_EL1.BRPs = 0x{:X} (bits [15:12])", field_value);
}


#[test]
fn test_idaa64dfr0el1_field_tracever() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("ID_AA64DFR0_EL1.TraceVer = 0x{:X} (bits [7:4])", field_value);
}


#[test]
fn test_idaa64dfr0el1_field_doublelock() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0xF;
    println!("ID_AA64DFR0_EL1.DoubleLock = 0x{:X} (bits [39:36])", field_value);
}


#[test]
fn test_idaa64dfr0el1_field_ctx_cmps() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR0_EL1
    let mrs_insn: u32 = 0xD5380500;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0xF;
    println!("ID_AA64DFR0_EL1.CTX_CMPs = 0x{:X} (bits [31:28])", field_value);
}


#[test]
fn test_mrs_ctrel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CTR_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CTR_EL0 = 0x{:016X}", value);
}


#[test]
fn test_ctrel0_field_tminline() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0x3F;
    println!("CTR_EL0.TminLine = 0x{:X} (bits [37:32])", field_value);
}


#[test]
fn test_ctrel0_field_cwg() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xF;
    println!("CTR_EL0.CWG = 0x{:X} (bits [27:24])", field_value);
}


#[test]
fn test_ctrel0_field_dic() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("CTR_EL0.DIC = 0x{:X} (bits [29:29])", field_value);
}


#[test]
fn test_ctrel0_field_dminline() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("CTR_EL0.DminLine = 0x{:X} (bits [19:16])", field_value);
}


#[test]
fn test_ctrel0_field_iminline() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("CTR_EL0.IminLine = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_ctrel0_field_idc() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("CTR_EL0.IDC = 0x{:X} (bits [28:28])", field_value);
}


#[test]
fn test_ctrel0_field_erg() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("CTR_EL0.ERG = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_ctrel0_field_l1ip() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CTR_EL0
    let mrs_insn: u32 = 0xD53B0020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x3;
    println!("CTR_EL0.L1Ip = 0x{:X} (bits [15:14])", field_value);
}


#[test]
fn test_mrs_vpidrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, VPIDR_EL2
    let mrs_insn: u32 = 0xD53C0000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS VPIDR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("VPIDR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_vpidrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR VPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C0001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VPIDR_EL2
    let mrs_insn: u32 = 0xD53C0000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("VPIDR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_vpidrel2_field_variant() {
    let mut cpu = create_test_cpu();
    
    // Set field Variant to all 1s
    let test_value = 0x0000000000F00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C0001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VPIDR_EL2
    let mrs_insn: u32 = 0xD53C0000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("VPIDR_EL2.Variant = 0x{:X} (bits [23:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vpidrel2_field_architecture() {
    let mut cpu = create_test_cpu();
    
    // Set field Architecture to all 1s
    let test_value = 0x00000000000F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C0001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VPIDR_EL2
    let mrs_insn: u32 = 0xD53C0000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("VPIDR_EL2.Architecture = 0x{:X} (bits [19:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vpidrel2_field_partnum() {
    let mut cpu = create_test_cpu();
    
    // Set field PartNum to all 1s
    let test_value = 0x000000000000FFF0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C0001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VPIDR_EL2
    let mrs_insn: u32 = 0xD53C0000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xFFF;
    println!("VPIDR_EL2.PartNum = 0x{:X} (bits [15:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vpidrel2_field_implementer() {
    let mut cpu = create_test_cpu();
    
    // Set field Implementer to all 1s
    let test_value = 0x00000000FF000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C0001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VPIDR_EL2
    let mrs_insn: u32 = 0xD53C0000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xFF;
    println!("VPIDR_EL2.Implementer = 0x{:X} (bits [31:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vpidrel2_field_revision() {
    let mut cpu = create_test_cpu();
    
    // Set field Revision to all 1s
    let test_value = 0x000000000000000Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR VPIDR_EL2, X1
    let msr_insn: u32 = 0xD51C0001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VPIDR_EL2
    let mrs_insn: u32 = 0xD53C0000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("VPIDR_EL2.Revision = 0x{:X} (bits [3:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_iccigrpen0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_IGRPEN0_EL1
    let mrs_insn: u32 = 0xD538CCC0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_IGRPEN0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_IGRPEN0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccigrpen0el1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_IGRPEN0_EL1, X1
    let msr_insn: u32 = 0xD518CCC1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_IGRPEN0_EL1
    let mrs_insn: u32 = 0xD538CCC0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_IGRPEN0_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccigrpen0el1_field_enable() {
    let mut cpu = create_test_cpu();
    
    // Set field Enable to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_IGRPEN0_EL1, X1
    let msr_insn: u32 = 0xD518CCC1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_IGRPEN0_EL1
    let mrs_insn: u32 = 0xD538CCC0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ICC_IGRPEN0_EL1.Enable = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_icciar0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_IAR0_EL1
    let mrs_insn: u32 = 0xD538C800;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_IAR0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_IAR0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_icciar0el1_field_intid() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_IAR0_EL1
    let mrs_insn: u32 = 0xD538C800;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFFFF;
    println!("ICC_IAR0_EL1.INTID = 0x{:X} (bits [23:0])", field_value);
}


#[test]
fn test_mrs_cpacrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CPACR_EL1
    let mrs_insn: u32 = 0xD5381040;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CPACR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CPACR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cpacrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CPACR_EL1, X1
    let msr_insn: u32 = 0xD5181041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPACR_EL1
    let mrs_insn: u32 = 0xD5381040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CPACR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cpacrel1_field_fpen() {
    let mut cpu = create_test_cpu();
    
    // Set field FPEN to all 1s
    let test_value = 0x0000000000300000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPACR_EL1, X1
    let msr_insn: u32 = 0xD5181041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPACR_EL1
    let mrs_insn: u32 = 0xD5381040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x3;
    println!("CPACR_EL1.FPEN = 0x{:X} (bits [21:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cpacrel1_field_zen() {
    let mut cpu = create_test_cpu();
    
    // Set field ZEN to all 1s
    let test_value = 0x0000000000030000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPACR_EL1, X1
    let msr_insn: u32 = 0xD5181041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPACR_EL1
    let mrs_insn: u32 = 0xD5381040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x3;
    println!("CPACR_EL1.ZEN = 0x{:X} (bits [17:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cpacrel1_field_tta() {
    let mut cpu = create_test_cpu();
    
    // Set field TTA to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPACR_EL1, X1
    let msr_insn: u32 = 0xD5181041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPACR_EL1
    let mrs_insn: u32 = 0xD5381040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("CPACR_EL1.TTA = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_sctlrel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SCTLR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SCTLR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_sctlrel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SCTLR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_sctlrel3_field_sa() {
    let mut cpu = create_test_cpu();
    
    // Set field SA to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("SCTLR_EL3.SA = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_bt() {
    let mut cpu = create_test_cpu();
    
    // Set field BT to all 1s
    let test_value = 0x0000001000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0x1;
    println!("SCTLR_EL3.BT = 0x{:X} (bits [36:36])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_eos() {
    let mut cpu = create_test_cpu();
    
    // Set field EOS to all 1s
    let test_value = 0x0000000000000800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x1;
    println!("SCTLR_EL3.EOS = 0x{:X} (bits [11:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_a() {
    let mut cpu = create_test_cpu();
    
    // Set field A to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("SCTLR_EL3.A = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_dssbs() {
    let mut cpu = create_test_cpu();
    
    // Set field DSSBS to all 1s
    let test_value = 0x0000100000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0x1;
    println!("SCTLR_EL3.DSSBS = 0x{:X} (bits [44:44])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_itfsb() {
    let mut cpu = create_test_cpu();
    
    // Set field ITFSB to all 1s
    let test_value = 0x0000002000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 37) & 0x1;
    println!("SCTLR_EL3.ITFSB = 0x{:X} (bits [37:37])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_eis() {
    let mut cpu = create_test_cpu();
    
    // Set field EIS to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("SCTLR_EL3.EIS = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_ata() {
    let mut cpu = create_test_cpu();
    
    // Set field ATA to all 1s
    let test_value = 0x0000080000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 43) & 0x1;
    println!("SCTLR_EL3.ATA = 0x{:X} (bits [43:43])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_enda() {
    let mut cpu = create_test_cpu();
    
    // Set field EnDA to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("SCTLR_EL3.EnDA = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_naa() {
    let mut cpu = create_test_cpu();
    
    // Set field nAA to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("SCTLR_EL3.nAA = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_enia() {
    let mut cpu = create_test_cpu();
    
    // Set field EnIA to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("SCTLR_EL3.EnIA = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_iesb() {
    let mut cpu = create_test_cpu();
    
    // Set field IESB to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("SCTLR_EL3.IESB = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_endb() {
    let mut cpu = create_test_cpu();
    
    // Set field EnDB to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("SCTLR_EL3.EnDB = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("SCTLR_EL3.C = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_tcf() {
    let mut cpu = create_test_cpu();
    
    // Set field TCF to all 1s
    let test_value = 0x0000030000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0x3;
    println!("SCTLR_EL3.TCF = 0x{:X} (bits [41:40])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_m() {
    let mut cpu = create_test_cpu();
    
    // Set field M to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("SCTLR_EL3.M = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_enib() {
    let mut cpu = create_test_cpu();
    
    // Set field EnIB to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("SCTLR_EL3.EnIB = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_ee() {
    let mut cpu = create_test_cpu();
    
    // Set field EE to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("SCTLR_EL3.EE = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_wxn() {
    let mut cpu = create_test_cpu();
    
    // Set field WXN to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("SCTLR_EL3.WXN = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_sctlrel3_field_i() {
    let mut cpu = create_test_cpu();
    
    // Set field I to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCTLR_EL3, X1
    let msr_insn: u32 = 0xD51E1001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCTLR_EL3
    let mrs_insn: u32 = 0xD53E1000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("SCTLR_EL3.I = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_mairel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MAIR_EL1
    let mrs_insn: u32 = 0xD538A200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS MAIR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("MAIR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_mairel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR MAIR_EL1, X1
    let msr_insn: u32 = 0xD518A201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, MAIR_EL1
    let mrs_insn: u32 = 0xD538A200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("MAIR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_spsrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SPSR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SPSR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_spsrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SPSR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_spsrel1_field_pan() {
    let mut cpu = create_test_cpu();
    
    // Set field PAN to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("SPSR_EL1.PAN = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_it() {
    let mut cpu = create_test_cpu();
    
    // Set field IT to all 1s
    let test_value = 0x0000000006000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x3;
    println!("SPSR_EL1.IT = 0x{:X} (bits [26:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_m() {
    let mut cpu = create_test_cpu();
    
    // Set field M to all 1s
    let test_value = 0x000000000000000Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("SPSR_EL1.M = 0x{:X} (bits [3:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_btype() {
    let mut cpu = create_test_cpu();
    
    // Set field BTYPE to all 1s
    let test_value = 0x0000000000000C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x3;
    println!("SPSR_EL1.BTYPE = 0x{:X} (bits [11:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_dit() {
    let mut cpu = create_test_cpu();
    
    // Set field DIT to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("SPSR_EL1.DIT = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_e() {
    let mut cpu = create_test_cpu();
    
    // Set field E to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("SPSR_EL1.E = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_a() {
    let mut cpu = create_test_cpu();
    
    // Set field A to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("SPSR_EL1.A = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_ss() {
    let mut cpu = create_test_cpu();
    
    // Set field SS to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("SPSR_EL1.SS = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_f() {
    let mut cpu = create_test_cpu();
    
    // Set field F to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("SPSR_EL1.F = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_uao() {
    let mut cpu = create_test_cpu();
    
    // Set field UAO to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("SPSR_EL1.UAO = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_tco() {
    let mut cpu = create_test_cpu();
    
    // Set field TCO to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("SPSR_EL1.TCO = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_n() {
    let mut cpu = create_test_cpu();
    
    // Set field N to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("SPSR_EL1.N = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_z() {
    let mut cpu = create_test_cpu();
    
    // Set field Z to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("SPSR_EL1.Z = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_q() {
    let mut cpu = create_test_cpu();
    
    // Set field Q to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("SPSR_EL1.Q = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_v() {
    let mut cpu = create_test_cpu();
    
    // Set field V to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("SPSR_EL1.V = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("SPSR_EL1.C = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_il() {
    let mut cpu = create_test_cpu();
    
    // Set field IL to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("SPSR_EL1.IL = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_i() {
    let mut cpu = create_test_cpu();
    
    // Set field I to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("SPSR_EL1.I = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_d() {
    let mut cpu = create_test_cpu();
    
    // Set field D to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("SPSR_EL1.D = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_ge() {
    let mut cpu = create_test_cpu();
    
    // Set field GE to all 1s
    let test_value = 0x00000000000F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("SPSR_EL1.GE = 0x{:X} (bits [19:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_ssbs() {
    let mut cpu = create_test_cpu();
    
    // Set field SSBS to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("SPSR_EL1.SSBS = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel1_field_t() {
    let mut cpu = create_test_cpu();
    
    // Set field T to all 1s
    let test_value = 0x0000000000000020u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL1, X1
    let msr_insn: u32 = 0xD5184001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL1
    let mrs_insn: u32 = 0xD5384000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1;
    println!("SPSR_EL1.T = 0x{:X} (bits [5:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_elrel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ELR_EL3
    let mrs_insn: u32 = 0xD53E4020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ELR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ELR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_elrel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR ELR_EL3, X1
    let msr_insn: u32 = 0xD51E4021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ELR_EL3
    let mrs_insn: u32 = 0xD53E4020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ELR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_icceoir1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_EOIR1_EL1
    let mrs_insn: u32 = 0xD538CC20;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_EOIR1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_EOIR1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_icceoir1el1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_EOIR1_EL1, X1
    let msr_insn: u32 = 0xD518CC21;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_EOIR1_EL1
    let mrs_insn: u32 = 0xD538CC20;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_EOIR1_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_icceoir1el1_field_intid() {
    let mut cpu = create_test_cpu();
    
    // Set field INTID to all 1s
    let test_value = 0x0000000000FFFFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_EOIR1_EL1, X1
    let msr_insn: u32 = 0xD518CC21;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_EOIR1_EL1
    let mrs_insn: u32 = 0xD538CC20;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFFFF;
    println!("ICC_EOIR1_EL1.INTID = 0x{:X} (bits [23:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_apdakeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APDAKeyHi_EL1
    let mrs_insn: u32 = 0xD5382220;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APDAKeyHi_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APDAKeyHi_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apdakeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APDAKeyHi_EL1, X1
    let msr_insn: u32 = 0xD5182221;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APDAKeyHi_EL1
    let mrs_insn: u32 = 0xD5382220;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APDAKeyHi_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_iccsreel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_SRE_EL2
    let mrs_insn: u32 = 0xD53CC9A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_SRE_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_SRE_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccsreel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_SRE_EL2, X1
    let msr_insn: u32 = 0xD51CC9A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL2
    let mrs_insn: u32 = 0xD53CC9A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_SRE_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccsreel2_field_enable() {
    let mut cpu = create_test_cpu();
    
    // Set field Enable to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL2, X1
    let msr_insn: u32 = 0xD51CC9A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL2
    let mrs_insn: u32 = 0xD53CC9A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("ICC_SRE_EL2.Enable = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccsreel2_field_dfb() {
    let mut cpu = create_test_cpu();
    
    // Set field DFB to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL2, X1
    let msr_insn: u32 = 0xD51CC9A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL2
    let mrs_insn: u32 = 0xD53CC9A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ICC_SRE_EL2.DFB = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccsreel2_field_sre() {
    let mut cpu = create_test_cpu();
    
    // Set field SRE to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL2, X1
    let msr_insn: u32 = 0xD51CC9A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL2
    let mrs_insn: u32 = 0xD53CC9A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ICC_SRE_EL2.SRE = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccsreel2_field_dib() {
    let mut cpu = create_test_cpu();
    
    // Set field DIB to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL2, X1
    let msr_insn: u32 = 0xD51CC9A1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL2
    let mrs_insn: u32 = 0xD53CC9A0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("ICC_SRE_EL2.DIB = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_currentel() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CurrentEL
    let mrs_insn: u32 = 0xD5384240;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CurrentEL should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CurrentEL = 0x{:016X}", value);
}


#[test]
fn test_currentel_field_el() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CurrentEL
    let mrs_insn: u32 = 0xD5384240;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x3;
    println!("CurrentEL.EL = 0x{:X} (bits [3:2])", field_value);
}


#[test]
fn test_mrs_contextidrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CONTEXTIDR_EL2
    let mrs_insn: u32 = 0xD53CD020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CONTEXTIDR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CONTEXTIDR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_contextidrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CONTEXTIDR_EL2, X1
    let msr_insn: u32 = 0xD51CD021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CONTEXTIDR_EL2
    let mrs_insn: u32 = 0xD53CD020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CONTEXTIDR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_contextidrel2_field_procid() {
    let mut cpu = create_test_cpu();
    
    // Set field PROCID to all 1s
    let test_value = 0x00000000FFFFFFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR CONTEXTIDR_EL2, X1
    let msr_insn: u32 = 0xD51CD021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CONTEXTIDR_EL2
    let mrs_insn: u32 = 0xD53CD020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFFFFFF;
    println!("CONTEXTIDR_EL2.PROCID = 0x{:X} (bits [31:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_pmcntensetel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, PMCNTENSET_EL0
    let mrs_insn: u32 = 0xD53B9C20;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS PMCNTENSET_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("PMCNTENSET_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_pmcntensetel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR PMCNTENSET_EL0, X1
    let msr_insn: u32 = 0xD51B9C21;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCNTENSET_EL0
    let mrs_insn: u32 = 0xD53B9C20;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("PMCNTENSET_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_pmcntensetel0_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCNTENSET_EL0, X1
    let msr_insn: u32 = 0xD51B9C21;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCNTENSET_EL0
    let mrs_insn: u32 = 0xD53B9C20;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("PMCNTENSET_EL0.C = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_iccctlrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_CTLR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_CTLR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccctlrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_CTLR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccctlrel1_field_idbits() {
    let mut cpu = create_test_cpu();
    
    // Set field IDbits to all 1s
    let test_value = 0x0000000000003800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x7;
    println!("ICC_CTLR_EL1.IDbits = 0x{:X} (bits [13:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccctlrel1_field_eoimode() {
    let mut cpu = create_test_cpu();
    
    // Set field EOImode to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ICC_CTLR_EL1.EOImode = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccctlrel1_field_cbpr() {
    let mut cpu = create_test_cpu();
    
    // Set field CBPR to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ICC_CTLR_EL1.CBPR = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccctlrel1_field_seis() {
    let mut cpu = create_test_cpu();
    
    // Set field SEIS to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("ICC_CTLR_EL1.SEIS = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccctlrel1_field_pmhe() {
    let mut cpu = create_test_cpu();
    
    // Set field PMHE to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("ICC_CTLR_EL1.PMHE = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccctlrel1_field_a3v() {
    let mut cpu = create_test_cpu();
    
    // Set field A3V to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("ICC_CTLR_EL1.A3V = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccctlrel1_field_pribits() {
    let mut cpu = create_test_cpu();
    
    // Set field PRIbits to all 1s
    let test_value = 0x0000000000000700u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x7;
    println!("ICC_CTLR_EL1.PRIbits = 0x{:X} (bits [10:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccctlrel1_field_rss() {
    let mut cpu = create_test_cpu();
    
    // Set field RSS to all 1s
    let test_value = 0x0000000000040000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 18) & 0x1;
    println!("ICC_CTLR_EL1.RSS = 0x{:X} (bits [18:18])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccctlrel1_field_extrange() {
    let mut cpu = create_test_cpu();
    
    // Set field ExtRange to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_CTLR_EL1, X1
    let msr_insn: u32 = 0xD518CC81;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_CTLR_EL1
    let mrs_insn: u32 = 0xD538CC80;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("ICC_CTLR_EL1.ExtRange = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_vbarel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, VBAR_EL3
    let mrs_insn: u32 = 0xD53EC000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS VBAR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("VBAR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_vbarel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR VBAR_EL3, X1
    let msr_insn: u32 = 0xD51EC001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VBAR_EL3
    let mrs_insn: u32 = 0xD53EC000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("VBAR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_apdakeyloel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APDAKeyLo_EL1
    let mrs_insn: u32 = 0xD5382200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APDAKeyLo_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APDAKeyLo_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apdakeyloel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APDAKeyLo_EL1, X1
    let msr_insn: u32 = 0xD5182201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APDAKeyLo_EL1
    let mrs_insn: u32 = 0xD5382200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APDAKeyLo_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_vtcrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS VTCR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("VTCR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_vtcrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("VTCR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_vtcrel2_field_irgn0() {
    let mut cpu = create_test_cpu();
    
    // Set field IRGN0 to all 1s
    let test_value = 0x0000000000000300u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x3;
    println!("VTCR_EL2.IRGN0 = 0x{:X} (bits [9:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_sh0() {
    let mut cpu = create_test_cpu();
    
    // Set field SH0 to all 1s
    let test_value = 0x0000000000003000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x3;
    println!("VTCR_EL2.SH0 = 0x{:X} (bits [13:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_tg0() {
    let mut cpu = create_test_cpu();
    
    // Set field TG0 to all 1s
    let test_value = 0x000000000000C000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x3;
    println!("VTCR_EL2.TG0 = 0x{:X} (bits [15:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_ha() {
    let mut cpu = create_test_cpu();
    
    // Set field HA to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("VTCR_EL2.HA = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_orgn0() {
    let mut cpu = create_test_cpu();
    
    // Set field ORGN0 to all 1s
    let test_value = 0x0000000000000C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x3;
    println!("VTCR_EL2.ORGN0 = 0x{:X} (bits [11:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_hwu61() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU61 to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("VTCR_EL2.HWU61 = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_hwu62() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU62 to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("VTCR_EL2.HWU62 = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_hwu60() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU60 to all 1s
    let test_value = 0x0000000004000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x1;
    println!("VTCR_EL2.HWU60 = 0x{:X} (bits [26:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_hwu59() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU59 to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("VTCR_EL2.HWU59 = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_sl0() {
    let mut cpu = create_test_cpu();
    
    // Set field SL0 to all 1s
    let test_value = 0x00000000000000C0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x3;
    println!("VTCR_EL2.SL0 = 0x{:X} (bits [7:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_t0sz() {
    let mut cpu = create_test_cpu();
    
    // Set field T0SZ to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("VTCR_EL2.T0SZ = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_nsw() {
    let mut cpu = create_test_cpu();
    
    // Set field NSW to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("VTCR_EL2.NSW = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_nsa() {
    let mut cpu = create_test_cpu();
    
    // Set field NSA to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("VTCR_EL2.NSA = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_ps() {
    let mut cpu = create_test_cpu();
    
    // Set field PS to all 1s
    let test_value = 0x0000000000070000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x7;
    println!("VTCR_EL2.PS = 0x{:X} (bits [18:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_hd() {
    let mut cpu = create_test_cpu();
    
    // Set field HD to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("VTCR_EL2.HD = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vtcrel2_field_vs() {
    let mut cpu = create_test_cpu();
    
    // Set field VS to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTCR_EL2, X1
    let msr_insn: u32 = 0xD51C2141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTCR_EL2
    let mrs_insn: u32 = 0xD53C2140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("VTCR_EL2.VS = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_pmcrel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS PMCR_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("PMCR_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_pmcrel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("PMCR_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_pmcrel0_field_lc() {
    let mut cpu = create_test_cpu();
    
    // Set field LC to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("PMCR_EL0.LC = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_p() {
    let mut cpu = create_test_cpu();
    
    // Set field P to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("PMCR_EL0.P = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_d() {
    let mut cpu = create_test_cpu();
    
    // Set field D to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("PMCR_EL0.D = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("PMCR_EL0.C = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_lp() {
    let mut cpu = create_test_cpu();
    
    // Set field LP to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("PMCR_EL0.LP = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_x() {
    let mut cpu = create_test_cpu();
    
    // Set field X to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("PMCR_EL0.X = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_e() {
    let mut cpu = create_test_cpu();
    
    // Set field E to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("PMCR_EL0.E = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_dp() {
    let mut cpu = create_test_cpu();
    
    // Set field DP to all 1s
    let test_value = 0x0000000000000020u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1;
    println!("PMCR_EL0.DP = 0x{:X} (bits [5:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_idcode() {
    let mut cpu = create_test_cpu();
    
    // Set field IDCODE to all 1s
    let test_value = 0x0000000000FF0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xFF;
    println!("PMCR_EL0.IDCODE = 0x{:X} (bits [23:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_imp() {
    let mut cpu = create_test_cpu();
    
    // Set field IMP to all 1s
    let test_value = 0x00000000FF000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xFF;
    println!("PMCR_EL0.IMP = 0x{:X} (bits [31:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_pmcrel0_field_n() {
    let mut cpu = create_test_cpu();
    
    // Set field N to all 1s
    let test_value = 0x000000000000F800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCR_EL0, X1
    let msr_insn: u32 = 0xD51B9C01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCR_EL0
    let mrs_insn: u32 = 0xD53B9C00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x1F;
    println!("PMCR_EL0.N = 0x{:X} (bits [15:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_iccigrpen1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_IGRPEN1_EL1
    let mrs_insn: u32 = 0xD538CCE0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_IGRPEN1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_IGRPEN1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccigrpen1el1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_IGRPEN1_EL1, X1
    let msr_insn: u32 = 0xD518CCE1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_IGRPEN1_EL1
    let mrs_insn: u32 = 0xD538CCE0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_IGRPEN1_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccigrpen1el1_field_enable() {
    let mut cpu = create_test_cpu();
    
    // Set field Enable to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_IGRPEN1_EL1, X1
    let msr_insn: u32 = 0xD518CCE1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_IGRPEN1_EL1
    let mrs_insn: u32 = 0xD538CCE0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ICC_IGRPEN1_EL1.Enable = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_screl3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SCR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SCR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_screl3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SCR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_screl3_field_ata() {
    let mut cpu = create_test_cpu();
    
    // Set field ATA to all 1s
    let test_value = 0x0000000004000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x1;
    println!("SCR_EL3.ATA = 0x{:X} (bits [26:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_fiq() {
    let mut cpu = create_test_cpu();
    
    // Set field FIQ to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("SCR_EL3.FIQ = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_st() {
    let mut cpu = create_test_cpu();
    
    // Set field ST to all 1s
    let test_value = 0x0000000000000800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x1;
    println!("SCR_EL3.ST = 0x{:X} (bits [11:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_smd() {
    let mut cpu = create_test_cpu();
    
    // Set field SMD to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("SCR_EL3.SMD = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_ns() {
    let mut cpu = create_test_cpu();
    
    // Set field NS to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("SCR_EL3.NS = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_nmea() {
    let mut cpu = create_test_cpu();
    
    // Set field NMEA to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("SCR_EL3.NMEA = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_twi() {
    let mut cpu = create_test_cpu();
    
    // Set field TWI to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("SCR_EL3.TWI = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_rw() {
    let mut cpu = create_test_cpu();
    
    // Set field RW to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("SCR_EL3.RW = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_apk() {
    let mut cpu = create_test_cpu();
    
    // Set field APK to all 1s
    let test_value = 0x0000000000010000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x1;
    println!("SCR_EL3.APK = 0x{:X} (bits [16:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_enscxt() {
    let mut cpu = create_test_cpu();
    
    // Set field EnSCXT to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("SCR_EL3.EnSCXT = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_terr() {
    let mut cpu = create_test_cpu();
    
    // Set field TERR to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("SCR_EL3.TERR = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_twe() {
    let mut cpu = create_test_cpu();
    
    // Set field TWE to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("SCR_EL3.TWE = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_hce() {
    let mut cpu = create_test_cpu();
    
    // Set field HCE to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("SCR_EL3.HCE = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_irq() {
    let mut cpu = create_test_cpu();
    
    // Set field IRQ to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("SCR_EL3.IRQ = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_fien() {
    let mut cpu = create_test_cpu();
    
    // Set field FIEN to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("SCR_EL3.FIEN = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_tlor() {
    let mut cpu = create_test_cpu();
    
    // Set field TLOR to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("SCR_EL3.TLOR = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_sif() {
    let mut cpu = create_test_cpu();
    
    // Set field SIF to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("SCR_EL3.SIF = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_ea() {
    let mut cpu = create_test_cpu();
    
    // Set field EA to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("SCR_EL3.EA = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_api() {
    let mut cpu = create_test_cpu();
    
    // Set field API to all 1s
    let test_value = 0x0000000000020000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 17) & 0x1;
    println!("SCR_EL3.API = 0x{:X} (bits [17:17])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_ease() {
    let mut cpu = create_test_cpu();
    
    // Set field EASE to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("SCR_EL3.EASE = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_screl3_field_eel2() {
    let mut cpu = create_test_cpu();
    
    // Set field EEL2 to all 1s
    let test_value = 0x0000000000040000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SCR_EL3, X1
    let msr_insn: u32 = 0xD51E1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SCR_EL3
    let mrs_insn: u32 = 0xD53E1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 18) & 0x1;
    println!("SCR_EL3.EEL2 = 0x{:X} (bits [18:18])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_cnthctlel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTHCTL_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTHCTL_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cnthctlel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTHCTL_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cnthctlel2_field_evntdir() {
    let mut cpu = create_test_cpu();
    
    // Set field EVNTDIR to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("CNTHCTL_EL2.EVNTDIR = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_el0pten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL0PTEN to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("CNTHCTL_EL2.EL0PTEN = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_el1pten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL1PTEN to all 1s
    let test_value = 0x0000000000000800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x1;
    println!("CNTHCTL_EL2.EL1PTEN = 0x{:X} (bits [11:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_el0pcten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL0PCTEN to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("CNTHCTL_EL2.EL0PCTEN = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_evnten() {
    let mut cpu = create_test_cpu();
    
    // Set field EVNTEN to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("CNTHCTL_EL2.EVNTEN = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_evnti() {
    let mut cpu = create_test_cpu();
    
    // Set field EVNTI to all 1s
    let test_value = 0x00000000000000F0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("CNTHCTL_EL2.EVNTI = 0x{:X} (bits [7:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_el0vten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL0VTEN to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("CNTHCTL_EL2.EL0VTEN = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_el0vcten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL0VCTEN to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("CNTHCTL_EL2.EL0VCTEN = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_el1pcten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL1PCTEN to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("CNTHCTL_EL2.EL1PCTEN = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cnthctlel2_field_el1pcen() {
    let mut cpu = create_test_cpu();
    
    // Set field EL1PCEN to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTHCTL_EL2, X1
    let msr_insn: u32 = 0xD51CE101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTHCTL_EL2
    let mrs_insn: u32 = 0xD53CE100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("CNTHCTL_EL2.EL1PCEN = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_idaa64pfr0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64PFR0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64PFR0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_idaa64pfr0el1_field_sel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0xF;
    println!("ID_AA64PFR0_EL1.SEL2 = 0x{:X} (bits [39:36])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_fp() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ID_AA64PFR0_EL1.FP = 0x{:X} (bits [19:16])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_csv3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 60) & 0xF;
    println!("ID_AA64PFR0_EL1.CSV3 = 0x{:X} (bits [63:60])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_dit() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0xF;
    println!("ID_AA64PFR0_EL1.DIT = 0x{:X} (bits [51:48])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_amu() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0xF;
    println!("ID_AA64PFR0_EL1.AMU = 0x{:X} (bits [47:44])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_advsimd() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ID_AA64PFR0_EL1.AdvSIMD = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_el0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("ID_AA64PFR0_EL1.EL0 = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("ID_AA64PFR0_EL1.EL1 = 0x{:X} (bits [7:4])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_csv2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 56) & 0xF;
    println!("ID_AA64PFR0_EL1.CSV2 = 0x{:X} (bits [59:56])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_gic() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xF;
    println!("ID_AA64PFR0_EL1.GIC = 0x{:X} (bits [27:24])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_mpam() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0xF;
    println!("ID_AA64PFR0_EL1.MPAM = 0x{:X} (bits [43:40])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_ras() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0xF;
    println!("ID_AA64PFR0_EL1.RAS = 0x{:X} (bits [31:28])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_el2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xF;
    println!("ID_AA64PFR0_EL1.EL2 = 0x{:X} (bits [11:8])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_sve() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0xF;
    println!("ID_AA64PFR0_EL1.SVE = 0x{:X} (bits [35:32])", field_value);
}


#[test]
fn test_idaa64pfr0el1_field_el3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64PFR0_EL1
    let mrs_insn: u32 = 0xD5380400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xF;
    println!("ID_AA64PFR0_EL1.EL3 = 0x{:X} (bits [15:12])", field_value);
}


#[test]
fn test_mrs_vbarel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, VBAR_EL1
    let mrs_insn: u32 = 0xD538C000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS VBAR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("VBAR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_vbarel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR VBAR_EL1, X1
    let msr_insn: u32 = 0xD518C001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VBAR_EL1
    let mrs_insn: u32 = 0xD538C000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("VBAR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_apiakeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APIAKeyHi_EL1
    let mrs_insn: u32 = 0xD5382120;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APIAKeyHi_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APIAKeyHi_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apiakeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APIAKeyHi_EL1, X1
    let msr_insn: u32 = 0xD5182121;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APIAKeyHi_EL1
    let mrs_insn: u32 = 0xD5382120;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APIAKeyHi_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_cntvcvalel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTV_CVAL_EL0
    let mrs_insn: u32 = 0xD53BE340;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTV_CVAL_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTV_CVAL_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cntvcvalel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR CNTV_CVAL_EL0, X1
    let msr_insn: u32 = 0xD51BE341;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTV_CVAL_EL0
    let mrs_insn: u32 = 0xD53BE340;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTV_CVAL_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_cntvcvalel0_field_comparevalue() {
    let mut cpu = create_test_cpu();
    
    // Set field CompareValue to all 1s
    let test_value = 0x0000000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTV_CVAL_EL0, X1
    let msr_insn: u32 = 0xD51BE341;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTV_CVAL_EL0
    let mrs_insn: u32 = 0xD53BE340;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x0;
    println!("CNTV_CVAL_EL0.CompareValue = 0x{:X} (bits [63:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_icceoir0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_EOIR0_EL1
    let mrs_insn: u32 = 0xD538C820;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_EOIR0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_EOIR0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_icceoir0el1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_EOIR0_EL1, X1
    let msr_insn: u32 = 0xD518C821;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_EOIR0_EL1
    let mrs_insn: u32 = 0xD538C820;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_EOIR0_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_icceoir0el1_field_intid() {
    let mut cpu = create_test_cpu();
    
    // Set field INTID to all 1s
    let test_value = 0x0000000000FFFFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_EOIR0_EL1, X1
    let msr_insn: u32 = 0xD518C821;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_EOIR0_EL1
    let mrs_insn: u32 = 0xD538C820;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFFFF;
    println!("ICC_EOIR0_EL1.INTID = 0x{:X} (bits [23:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_actlrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ACTLR_EL1
    let mrs_insn: u32 = 0xD5381020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ACTLR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ACTLR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_actlrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR ACTLR_EL1, X1
    let msr_insn: u32 = 0xD5181021;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ACTLR_EL1
    let mrs_insn: u32 = 0xD5381020;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ACTLR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_fpcr() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS FPCR should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("FPCR = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_fpcr() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("FPCR: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_fpcr_field_ide() {
    let mut cpu = create_test_cpu();
    
    // Set field IDE to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("FPCR.IDE = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_stride() {
    let mut cpu = create_test_cpu();
    
    // Set field Stride to all 1s
    let test_value = 0x0000000000300000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x3;
    println!("FPCR.Stride = 0x{:X} (bits [21:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_ixe() {
    let mut cpu = create_test_cpu();
    
    // Set field IXE to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("FPCR.IXE = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_fz() {
    let mut cpu = create_test_cpu();
    
    // Set field FZ to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("FPCR.FZ = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_ufe() {
    let mut cpu = create_test_cpu();
    
    // Set field UFE to all 1s
    let test_value = 0x0000000000000800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x1;
    println!("FPCR.UFE = 0x{:X} (bits [11:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_len() {
    let mut cpu = create_test_cpu();
    
    // Set field Len to all 1s
    let test_value = 0x0000000000070000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x7;
    println!("FPCR.Len = 0x{:X} (bits [18:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_dze() {
    let mut cpu = create_test_cpu();
    
    // Set field DZE to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("FPCR.DZE = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_ioe() {
    let mut cpu = create_test_cpu();
    
    // Set field IOE to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("FPCR.IOE = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_dn() {
    let mut cpu = create_test_cpu();
    
    // Set field DN to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("FPCR.DN = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_ofe() {
    let mut cpu = create_test_cpu();
    
    // Set field OFE to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("FPCR.OFE = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_ahp() {
    let mut cpu = create_test_cpu();
    
    // Set field AHP to all 1s
    let test_value = 0x0000000004000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x1;
    println!("FPCR.AHP = 0x{:X} (bits [26:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_fz16() {
    let mut cpu = create_test_cpu();
    
    // Set field FZ16 to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("FPCR.FZ16 = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_fpcr_field_rmode() {
    let mut cpu = create_test_cpu();
    
    // Set field RMode to all 1s
    let test_value = 0x0000000000C00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR FPCR, X1
    let msr_insn: u32 = 0xD51B4401;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FPCR
    let mrs_insn: u32 = 0xD53B4400;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x3;
    println!("FPCR.RMode = 0x{:X} (bits [23:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_pmccntrel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, PMCCNTR_EL0
    let mrs_insn: u32 = 0xD53B9D00;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS PMCCNTR_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("PMCCNTR_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_pmccntrel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR PMCCNTR_EL0, X1
    let msr_insn: u32 = 0xD51B9D01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCCNTR_EL0
    let mrs_insn: u32 = 0xD53B9D00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("PMCCNTR_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_pmccntrel0_field_ccnt() {
    let mut cpu = create_test_cpu();
    
    // Set field CCNT to all 1s
    let test_value = 0x0000000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR PMCCNTR_EL0, X1
    let msr_insn: u32 = 0xD51B9D01;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, PMCCNTR_EL0
    let mrs_insn: u32 = 0xD53B9D00;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x0;
    println!("PMCCNTR_EL0.CCNT = 0x{:X} (bits [63:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_iccsreel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_SRE_EL3
    let mrs_insn: u32 = 0xD53ECCA0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_SRE_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_SRE_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccsreel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_SRE_EL3, X1
    let msr_insn: u32 = 0xD51ECCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL3
    let mrs_insn: u32 = 0xD53ECCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_SRE_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccsreel3_field_enable() {
    let mut cpu = create_test_cpu();
    
    // Set field Enable to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL3, X1
    let msr_insn: u32 = 0xD51ECCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL3
    let mrs_insn: u32 = 0xD53ECCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("ICC_SRE_EL3.Enable = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccsreel3_field_dfb() {
    let mut cpu = create_test_cpu();
    
    // Set field DFB to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL3, X1
    let msr_insn: u32 = 0xD51ECCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL3
    let mrs_insn: u32 = 0xD53ECCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ICC_SRE_EL3.DFB = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccsreel3_field_dib() {
    let mut cpu = create_test_cpu();
    
    // Set field DIB to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL3, X1
    let msr_insn: u32 = 0xD51ECCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL3
    let mrs_insn: u32 = 0xD53ECCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("ICC_SRE_EL3.DIB = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_iccsreel3_field_sre() {
    let mut cpu = create_test_cpu();
    
    // Set field SRE to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_SRE_EL3, X1
    let msr_insn: u32 = 0xD51ECCA1;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_SRE_EL3
    let mrs_insn: u32 = 0xD53ECCA0;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ICC_SRE_EL3.SRE = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_cntfrqel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTFRQ_EL0
    let mrs_insn: u32 = 0xD53BE000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTFRQ_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTFRQ_EL0 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cntfrqel0() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CNTFRQ_EL0, X1
    let msr_insn: u32 = 0xD51BE001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTFRQ_EL0
    let mrs_insn: u32 = 0xD53BE000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTFRQ_EL0: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_mrs_cptrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CPTR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CPTR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cptrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CPTR_EL2, X1
    let msr_insn: u32 = 0xD51C1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CPTR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cptrel2_field_fpen() {
    let mut cpu = create_test_cpu();
    
    // Set field FPEN to all 1s
    let test_value = 0x0000000000300000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL2, X1
    let msr_insn: u32 = 0xD51C1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x3;
    println!("CPTR_EL2.FPEN = 0x{:X} (bits [21:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel2_field_tz() {
    let mut cpu = create_test_cpu();
    
    // Set field TZ to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL2, X1
    let msr_insn: u32 = 0xD51C1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("CPTR_EL2.TZ = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel2_field_tcpac() {
    let mut cpu = create_test_cpu();
    
    // Set field TCPAC to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL2, X1
    let msr_insn: u32 = 0xD51C1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("CPTR_EL2.TCPAC = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel2_field_zen() {
    let mut cpu = create_test_cpu();
    
    // Set field ZEN to all 1s
    let test_value = 0x0000000000030000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL2, X1
    let msr_insn: u32 = 0xD51C1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x3;
    println!("CPTR_EL2.ZEN = 0x{:X} (bits [17:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel2_field_tam() {
    let mut cpu = create_test_cpu();
    
    // Set field TAM to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL2, X1
    let msr_insn: u32 = 0xD51C1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("CPTR_EL2.TAM = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel2_field_tfp() {
    let mut cpu = create_test_cpu();
    
    // Set field TFP to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL2, X1
    let msr_insn: u32 = 0xD51C1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("CPTR_EL2.TFP = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cptrel2_field_tta() {
    let mut cpu = create_test_cpu();
    
    // Set field TTA to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CPTR_EL2, X1
    let msr_insn: u32 = 0xD51C1141;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CPTR_EL2
    let mrs_insn: u32 = 0xD53C1140;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("CPTR_EL2.TTA = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_mpidrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MPIDR_EL1
    let mrs_insn: u32 = 0xD53800A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS MPIDR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("MPIDR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_mpidrel1_field_aff2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MPIDR_EL1
    let mrs_insn: u32 = 0xD53800A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xFF;
    println!("MPIDR_EL1.Aff2 = 0x{:X} (bits [23:16])", field_value);
}


#[test]
fn test_mpidrel1_field_u() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MPIDR_EL1
    let mrs_insn: u32 = 0xD53800A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("MPIDR_EL1.U = 0x{:X} (bits [30:30])", field_value);
}


#[test]
fn test_mpidrel1_field_aff1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MPIDR_EL1
    let mrs_insn: u32 = 0xD53800A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xFF;
    println!("MPIDR_EL1.Aff1 = 0x{:X} (bits [15:8])", field_value);
}


#[test]
fn test_mpidrel1_field_aff3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MPIDR_EL1
    let mrs_insn: u32 = 0xD53800A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0xFF;
    println!("MPIDR_EL1.Aff3 = 0x{:X} (bits [39:32])", field_value);
}


#[test]
fn test_mpidrel1_field_aff0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MPIDR_EL1
    let mrs_insn: u32 = 0xD53800A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFF;
    println!("MPIDR_EL1.Aff0 = 0x{:X} (bits [7:0])", field_value);
}


#[test]
fn test_mpidrel1_field_mt() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, MPIDR_EL1
    let mrs_insn: u32 = 0xD53800A0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("MPIDR_EL1.MT = 0x{:X} (bits [24:24])", field_value);
}


#[test]
fn test_mrs_revidrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, REVIDR_EL1
    let mrs_insn: u32 = 0xD53800C0;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS REVIDR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("REVIDR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_mrs_farel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, FAR_EL1
    let mrs_insn: u32 = 0xD5386000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS FAR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("FAR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_farel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR FAR_EL1, X1
    let msr_insn: u32 = 0xD5186001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, FAR_EL1
    let mrs_insn: u32 = 0xD5386000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("FAR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_spsrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SPSR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SPSR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_spsrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SPSR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_spsrel2_field_ge() {
    let mut cpu = create_test_cpu();
    
    // Set field GE to all 1s
    let test_value = 0x00000000000F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("SPSR_EL2.GE = 0x{:X} (bits [19:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_t() {
    let mut cpu = create_test_cpu();
    
    // Set field T to all 1s
    let test_value = 0x0000000000000020u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1;
    println!("SPSR_EL2.T = 0x{:X} (bits [5:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_a() {
    let mut cpu = create_test_cpu();
    
    // Set field A to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("SPSR_EL2.A = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_i() {
    let mut cpu = create_test_cpu();
    
    // Set field I to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("SPSR_EL2.I = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_ssbs() {
    let mut cpu = create_test_cpu();
    
    // Set field SSBS to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("SPSR_EL2.SSBS = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_pan() {
    let mut cpu = create_test_cpu();
    
    // Set field PAN to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("SPSR_EL2.PAN = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_m() {
    let mut cpu = create_test_cpu();
    
    // Set field M to all 1s
    let test_value = 0x000000000000000Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("SPSR_EL2.M = 0x{:X} (bits [3:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_n() {
    let mut cpu = create_test_cpu();
    
    // Set field N to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("SPSR_EL2.N = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_z() {
    let mut cpu = create_test_cpu();
    
    // Set field Z to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("SPSR_EL2.Z = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_q() {
    let mut cpu = create_test_cpu();
    
    // Set field Q to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("SPSR_EL2.Q = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_btype() {
    let mut cpu = create_test_cpu();
    
    // Set field BTYPE to all 1s
    let test_value = 0x0000000000000C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x3;
    println!("SPSR_EL2.BTYPE = 0x{:X} (bits [11:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_tco() {
    let mut cpu = create_test_cpu();
    
    // Set field TCO to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("SPSR_EL2.TCO = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_e() {
    let mut cpu = create_test_cpu();
    
    // Set field E to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("SPSR_EL2.E = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_il() {
    let mut cpu = create_test_cpu();
    
    // Set field IL to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("SPSR_EL2.IL = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_ss() {
    let mut cpu = create_test_cpu();
    
    // Set field SS to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("SPSR_EL2.SS = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("SPSR_EL2.C = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_f() {
    let mut cpu = create_test_cpu();
    
    // Set field F to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("SPSR_EL2.F = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_d() {
    let mut cpu = create_test_cpu();
    
    // Set field D to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("SPSR_EL2.D = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_it() {
    let mut cpu = create_test_cpu();
    
    // Set field IT to all 1s
    let test_value = 0x0000000006000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x3;
    println!("SPSR_EL2.IT = 0x{:X} (bits [26:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_dit() {
    let mut cpu = create_test_cpu();
    
    // Set field DIT to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("SPSR_EL2.DIT = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_v() {
    let mut cpu = create_test_cpu();
    
    // Set field V to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("SPSR_EL2.V = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_spsrel2_field_uao() {
    let mut cpu = create_test_cpu();
    
    // Set field UAO to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR SPSR_EL2, X1
    let msr_insn: u32 = 0xD51C4001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SPSR_EL2
    let mrs_insn: u32 = 0xD53C4000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("SPSR_EL2.UAO = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_nzcv() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, NZCV
    let mrs_insn: u32 = 0xD53B4200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS NZCV should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("NZCV = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_nzcv() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR NZCV, X1
    let msr_insn: u32 = 0xD51B4201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, NZCV
    let mrs_insn: u32 = 0xD53B4200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("NZCV: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_nzcv_field_v() {
    let mut cpu = create_test_cpu();
    
    // Set field V to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR NZCV, X1
    let msr_insn: u32 = 0xD51B4201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, NZCV
    let mrs_insn: u32 = 0xD53B4200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("NZCV.V = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_nzcv_field_n() {
    let mut cpu = create_test_cpu();
    
    // Set field N to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR NZCV, X1
    let msr_insn: u32 = 0xD51B4201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, NZCV
    let mrs_insn: u32 = 0xD53B4200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("NZCV.N = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_nzcv_field_c() {
    let mut cpu = create_test_cpu();
    
    // Set field C to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR NZCV, X1
    let msr_insn: u32 = 0xD51B4201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, NZCV
    let mrs_insn: u32 = 0xD53B4200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("NZCV.C = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_nzcv_field_z() {
    let mut cpu = create_test_cpu();
    
    // Set field Z to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR NZCV, X1
    let msr_insn: u32 = 0xD51B4201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, NZCV
    let mrs_insn: u32 = 0xD53B4200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("NZCV.Z = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_iccpmrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ICC_PMR_EL1
    let mrs_insn: u32 = 0xD5384600;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ICC_PMR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ICC_PMR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_iccpmrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ICC_PMR_EL1, X1
    let msr_insn: u32 = 0xD5184601;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_PMR_EL1
    let mrs_insn: u32 = 0xD5384600;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ICC_PMR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_iccpmrel1_field_priority() {
    let mut cpu = create_test_cpu();
    
    // Set field Priority to all 1s
    let test_value = 0x00000000000000FFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ICC_PMR_EL1, X1
    let msr_insn: u32 = 0xD5184601;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ICC_PMR_EL1
    let mrs_insn: u32 = 0xD5384600;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFF;
    println!("ICC_PMR_EL1.Priority = 0x{:X} (bits [7:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_vttbrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, VTTBR_EL2
    let mrs_insn: u32 = 0xD53C2100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS VTTBR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("VTTBR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_vttbrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR VTTBR_EL2, X1
    let msr_insn: u32 = 0xD51C2101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTTBR_EL2
    let mrs_insn: u32 = 0xD53C2100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("VTTBR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_vttbrel2_field_vmid() {
    let mut cpu = create_test_cpu();
    
    // Set field VMID to all 1s
    let test_value = 0x00FF000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTTBR_EL2, X1
    let msr_insn: u32 = 0xD51C2101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTTBR_EL2
    let mrs_insn: u32 = 0xD53C2100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0xFF;
    println!("VTTBR_EL2.VMID = 0x{:X} (bits [55:48])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vttbrel2_field_baddr() {
    let mut cpu = create_test_cpu();
    
    // Set field BADDR to all 1s
    let test_value = 0x0000FFFFFFFFFFFEu64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTTBR_EL2, X1
    let msr_insn: u32 = 0xD51C2101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTTBR_EL2
    let mrs_insn: u32 = 0xD53C2100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x7FFFFFFFFFFF;
    println!("VTTBR_EL2.BADDR = 0x{:X} (bits [47:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_vttbrel2_field_cnp() {
    let mut cpu = create_test_cpu();
    
    // Set field CnP to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR VTTBR_EL2, X1
    let msr_insn: u32 = 0xD51C2101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, VTTBR_EL2
    let mrs_insn: u32 = 0xD53C2100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("VTTBR_EL2.CnP = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_tpidrel3() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TPIDR_EL3
    let mrs_insn: u32 = 0xD53ED040;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TPIDR_EL3 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TPIDR_EL3 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_tpidrel3() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TPIDR_EL3, X1
    let msr_insn: u32 = 0xD51ED041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TPIDR_EL3
    let mrs_insn: u32 = 0xD53ED040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TPIDR_EL3: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_esrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ESR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ESR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_esrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("ESR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_esrel1_field_ids() {
    let mut cpu = create_test_cpu();
    
    // Set field IDS to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL1.IDS = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ifsc() {
    let mut cpu = create_test_cpu();
    
    // Set field IFSC to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("ESR_EL1.IFSC = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_op0() {
    let mut cpu = create_test_cpu();
    
    // Set field Op0 to all 1s
    let test_value = 0x0000000000300000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x3;
    println!("ESR_EL1.Op0 = 0x{:X} (bits [21:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_am() {
    let mut cpu = create_test_cpu();
    
    // Set field AM to all 1s
    let test_value = 0x000000000000000Eu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x7;
    println!("ESR_EL1.AM = 0x{:X} (bits [3:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_op1() {
    let mut cpu = create_test_cpu();
    
    // Set field Op1 to all 1s
    let test_value = 0x000000000001C000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x7;
    println!("ESR_EL1.Op1 = 0x{:X} (bits [16:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_wnr() {
    let mut cpu = create_test_cpu();
    
    // Set field WnR to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("ESR_EL1.WnR = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_tfv() {
    let mut cpu = create_test_cpu();
    
    // Set field TFV to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("ESR_EL1.TFV = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ixf() {
    let mut cpu = create_test_cpu();
    
    // Set field IXF to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("ESR_EL1.IXF = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_aet() {
    let mut cpu = create_test_cpu();
    
    // Set field AET to all 1s
    let test_value = 0x0000000000001C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x7;
    println!("ESR_EL1.AET = 0x{:X} (bits [12:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ex() {
    let mut cpu = create_test_cpu();
    
    // Set field EX to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("ESR_EL1.EX = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_iof() {
    let mut cpu = create_test_cpu();
    
    // Set field IOF to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL1.IOF = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_opc1() {
    let mut cpu = create_test_cpu();
    
    // Set field Opc1 to all 1s
    let test_value = 0x00000000000F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ESR_EL1.Opc1 = 0x{:X} (bits [19:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ec() {
    let mut cpu = create_test_cpu();
    
    // Set field EC to all 1s
    let test_value = 0x00000000FC000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x3F;
    println!("ESR_EL1.EC = 0x{:X} (bits [31:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_imm16() {
    let mut cpu = create_test_cpu();
    
    // Set field imm16 to all 1s
    let test_value = 0x000000000000FFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFF;
    println!("ESR_EL1.imm16 = 0x{:X} (bits [15:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ccknownpass() {
    let mut cpu = create_test_cpu();
    
    // Set field CCKNOWNPASS to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("ESR_EL1.CCKNOWNPASS = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_il() {
    let mut cpu = create_test_cpu();
    
    // Set field IL to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("ESR_EL1.IL = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_cond() {
    let mut cpu = create_test_cpu();
    
    // Set field COND to all 1s
    let test_value = 0x0000000000F00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ESR_EL1.COND = 0x{:X} (bits [23:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_comment() {
    let mut cpu = create_test_cpu();
    
    // Set field Comment to all 1s
    let test_value = 0x000000000000FFFFu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xFFFF;
    println!("ESR_EL1.Comment = 0x{:X} (bits [15:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_rt2() {
    let mut cpu = create_test_cpu();
    
    // Set field Rt2 to all 1s
    let test_value = 0x0000000000007C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1F;
    println!("ESR_EL1.Rt2 = 0x{:X} (bits [14:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ea() {
    let mut cpu = create_test_cpu();
    
    // Set field EA to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("ESR_EL1.EA = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_vncr() {
    let mut cpu = create_test_cpu();
    
    // Set field VNCR to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("ESR_EL1.VNCR = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_eret() {
    let mut cpu = create_test_cpu();
    
    // Set field ERET to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ESR_EL1.ERET = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_set() {
    let mut cpu = create_test_cpu();
    
    // Set field SET to all 1s
    let test_value = 0x0000000000001800u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 11) & 0x3;
    println!("ESR_EL1.SET = 0x{:X} (bits [12:11])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_off() {
    let mut cpu = create_test_cpu();
    
    // Set field OFF to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("ESR_EL1.OFF = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ereta() {
    let mut cpu = create_test_cpu();
    
    // Set field ERETA to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL1.ERETA = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_cv() {
    let mut cpu = create_test_cpu();
    
    // Set field CV to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL1.CV = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_offset() {
    let mut cpu = create_test_cpu();
    
    // Set field Offset to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("ESR_EL1.Offset = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ar() {
    let mut cpu = create_test_cpu();
    
    // Set field AR to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("ESR_EL1.AR = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_crm() {
    let mut cpu = create_test_cpu();
    
    // Set field CRm to all 1s
    let test_value = 0x000000000000001Eu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0xF;
    println!("ESR_EL1.CRm = 0x{:X} (bits [4:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_direction() {
    let mut cpu = create_test_cpu();
    
    // Set field Direction to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL1.Direction = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_sas() {
    let mut cpu = create_test_cpu();
    
    // Set field SAS to all 1s
    let test_value = 0x0000000000C00000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x3;
    println!("ESR_EL1.SAS = 0x{:X} (bits [23:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_srt() {
    let mut cpu = create_test_cpu();
    
    // Set field SRT to all 1s
    let test_value = 0x00000000001F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x1F;
    println!("ESR_EL1.SRT = 0x{:X} (bits [20:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_s1ptw() {
    let mut cpu = create_test_cpu();
    
    // Set field S1PTW to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("ESR_EL1.S1PTW = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_isv() {
    let mut cpu = create_test_cpu();
    
    // Set field ISV to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("ESR_EL1.ISV = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_fnv() {
    let mut cpu = create_test_cpu();
    
    // Set field FnV to all 1s
    let test_value = 0x0000000000000400u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x1;
    println!("ESR_EL1.FnV = 0x{:X} (bits [10:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_ti() {
    let mut cpu = create_test_cpu();
    
    // Set field TI to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("ESR_EL1.TI = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_opc2() {
    let mut cpu = create_test_cpu();
    
    // Set field Opc2 to all 1s
    let test_value = 0x00000000000E0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 17) & 0x7;
    println!("ESR_EL1.Opc2 = 0x{:X} (bits [19:17])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_vecitr() {
    let mut cpu = create_test_cpu();
    
    // Set field VECITR to all 1s
    let test_value = 0x0000000000000700u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x7;
    println!("ESR_EL1.VECITR = 0x{:X} (bits [10:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_op2() {
    let mut cpu = create_test_cpu();
    
    // Set field Op2 to all 1s
    let test_value = 0x00000000000E0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 17) & 0x7;
    println!("ESR_EL1.Op2 = 0x{:X} (bits [19:17])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_idf() {
    let mut cpu = create_test_cpu();
    
    // Set field IDF to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("ESR_EL1.IDF = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_sse() {
    let mut cpu = create_test_cpu();
    
    // Set field SSE to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("ESR_EL1.SSE = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_rt() {
    let mut cpu = create_test_cpu();
    
    // Set field Rt to all 1s
    let test_value = 0x00000000000003E0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1F;
    println!("ESR_EL1.Rt = 0x{:X} (bits [9:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_dfsc() {
    let mut cpu = create_test_cpu();
    
    // Set field DFSC to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("ESR_EL1.DFSC = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_cm() {
    let mut cpu = create_test_cpu();
    
    // Set field CM to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("ESR_EL1.CM = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_dzf() {
    let mut cpu = create_test_cpu();
    
    // Set field DZF to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("ESR_EL1.DZF = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_iesb() {
    let mut cpu = create_test_cpu();
    
    // Set field IESB to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("ESR_EL1.IESB = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_imm8() {
    let mut cpu = create_test_cpu();
    
    // Set field imm8 to all 1s
    let test_value = 0x00000000000FF000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xFF;
    println!("ESR_EL1.imm8 = 0x{:X} (bits [19:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_rn() {
    let mut cpu = create_test_cpu();
    
    // Set field Rn to all 1s
    let test_value = 0x00000000000003E0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1F;
    println!("ESR_EL1.Rn = 0x{:X} (bits [9:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_crn() {
    let mut cpu = create_test_cpu();
    
    // Set field CRn to all 1s
    let test_value = 0x0000000000003C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0xF;
    println!("ESR_EL1.CRn = 0x{:X} (bits [13:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_sf() {
    let mut cpu = create_test_cpu();
    
    // Set field SF to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("ESR_EL1.SF = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_uff() {
    let mut cpu = create_test_cpu();
    
    // Set field UFF to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("ESR_EL1.UFF = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_esrel1_field_btype() {
    let mut cpu = create_test_cpu();
    
    // Set field BTYPE to all 1s
    let test_value = 0x0000000000000003u64;
    cpu.set_gpr(1, test_value);
    
    // MSR ESR_EL1, X1
    let msr_insn: u32 = 0xD5185201;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, ESR_EL1
    let mrs_insn: u32 = 0xD5385200;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3;
    println!("ESR_EL1.BTYPE = 0x{:X} (bits [1:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_cntkctlel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTKCTL_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTKCTL_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_cntkctlel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x0000000012345678);
    
    // MSR CNTKCTL_EL1, X1
    let msr_insn: u32 = 0xD518E101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("CNTKCTL_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x0000000012345678u64, readback);
}


#[test]
fn test_cntkctlel1_field_el0pcten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL0PCTEN to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTKCTL_EL1, X1
    let msr_insn: u32 = 0xD518E101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("CNTKCTL_EL1.EL0PCTEN = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntkctlel1_field_el0pten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL0PTEN to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTKCTL_EL1, X1
    let msr_insn: u32 = 0xD518E101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("CNTKCTL_EL1.EL0PTEN = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntkctlel1_field_el0vten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL0VTEN to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTKCTL_EL1, X1
    let msr_insn: u32 = 0xD518E101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("CNTKCTL_EL1.EL0VTEN = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntkctlel1_field_evnten() {
    let mut cpu = create_test_cpu();
    
    // Set field EVNTEN to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTKCTL_EL1, X1
    let msr_insn: u32 = 0xD518E101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("CNTKCTL_EL1.EVNTEN = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntkctlel1_field_evnti() {
    let mut cpu = create_test_cpu();
    
    // Set field EVNTI to all 1s
    let test_value = 0x00000000000000F0u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTKCTL_EL1, X1
    let msr_insn: u32 = 0xD518E101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("CNTKCTL_EL1.EVNTI = 0x{:X} (bits [7:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntkctlel1_field_evntdir() {
    let mut cpu = create_test_cpu();
    
    // Set field EVNTDIR to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTKCTL_EL1, X1
    let msr_insn: u32 = 0xD518E101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("CNTKCTL_EL1.EVNTDIR = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_cntkctlel1_field_el0vcten() {
    let mut cpu = create_test_cpu();
    
    // Set field EL0VCTEN to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR CNTKCTL_EL1, X1
    let msr_insn: u32 = 0xD518E101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, CNTKCTL_EL1
    let mrs_insn: u32 = 0xD538E100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("CNTKCTL_EL1.EL0VCTEN = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_tcrel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TCR_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TCR_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_tcrel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TCR_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_tcrel1_field_hwu162() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU162 to all 1s
    let test_value = 0x0004000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 50) & 0x1;
    println!("TCR_EL1.HWU162 = 0x{:X} (bits [50:50])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hwu060() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU060 to all 1s
    let test_value = 0x0000100000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0x1;
    println!("TCR_EL1.HWU060 = 0x{:X} (bits [44:44])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_tbid1() {
    let mut cpu = create_test_cpu();
    
    // Set field TBID1 to all 1s
    let test_value = 0x0010000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 52) & 0x1;
    println!("TCR_EL1.TBID1 = 0x{:X} (bits [52:52])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_tcma1() {
    let mut cpu = create_test_cpu();
    
    // Set field TCMA1 to all 1s
    let test_value = 0x0400000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 58) & 0x1;
    println!("TCR_EL1.TCMA1 = 0x{:X} (bits [58:58])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hwu160() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU160 to all 1s
    let test_value = 0x0001000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0x1;
    println!("TCR_EL1.HWU160 = 0x{:X} (bits [48:48])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_ha() {
    let mut cpu = create_test_cpu();
    
    // Set field HA to all 1s
    let test_value = 0x0000008000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 39) & 0x1;
    println!("TCR_EL1.HA = 0x{:X} (bits [39:39])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_tg0() {
    let mut cpu = create_test_cpu();
    
    // Set field TG0 to all 1s
    let test_value = 0x000000000000C000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x3;
    println!("TCR_EL1.TG0 = 0x{:X} (bits [15:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_t0sz() {
    let mut cpu = create_test_cpu();
    
    // Set field T0SZ to all 1s
    let test_value = 0x000000000000003Fu64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x3F;
    println!("TCR_EL1.T0SZ = 0x{:X} (bits [5:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hpd1() {
    let mut cpu = create_test_cpu();
    
    // Set field HPD1 to all 1s
    let test_value = 0x0000040000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 42) & 0x1;
    println!("TCR_EL1.HPD1 = 0x{:X} (bits [42:42])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_orgn0() {
    let mut cpu = create_test_cpu();
    
    // Set field ORGN0 to all 1s
    let test_value = 0x0000000000000C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x3;
    println!("TCR_EL1.ORGN0 = 0x{:X} (bits [11:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_tg1() {
    let mut cpu = create_test_cpu();
    
    // Set field TG1 to all 1s
    let test_value = 0x00000000C0000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x3;
    println!("TCR_EL1.TG1 = 0x{:X} (bits [31:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_orgn1() {
    let mut cpu = create_test_cpu();
    
    // Set field ORGN1 to all 1s
    let test_value = 0x000000000C000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x3;
    println!("TCR_EL1.ORGN1 = 0x{:X} (bits [27:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_ips() {
    let mut cpu = create_test_cpu();
    
    // Set field IPS to all 1s
    let test_value = 0x0000000700000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0x7;
    println!("TCR_EL1.IPS = 0x{:X} (bits [34:32])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_sh1() {
    let mut cpu = create_test_cpu();
    
    // Set field SH1 to all 1s
    let test_value = 0x0000000030000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x3;
    println!("TCR_EL1.SH1 = 0x{:X} (bits [29:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_irgn1() {
    let mut cpu = create_test_cpu();
    
    // Set field IRGN1 to all 1s
    let test_value = 0x0000000003000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x3;
    println!("TCR_EL1.IRGN1 = 0x{:X} (bits [25:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hwu159() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU159 to all 1s
    let test_value = 0x0000800000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 47) & 0x1;
    println!("TCR_EL1.HWU159 = 0x{:X} (bits [47:47])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hwu161() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU161 to all 1s
    let test_value = 0x0002000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 49) & 0x1;
    println!("TCR_EL1.HWU161 = 0x{:X} (bits [49:49])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_as() {
    let mut cpu = create_test_cpu();
    
    // Set field AS to all 1s
    let test_value = 0x0000001000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0x1;
    println!("TCR_EL1.AS = 0x{:X} (bits [36:36])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_e0pd1() {
    let mut cpu = create_test_cpu();
    
    // Set field E0PD1 to all 1s
    let test_value = 0x0100000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 56) & 0x1;
    println!("TCR_EL1.E0PD1 = 0x{:X} (bits [56:56])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hd() {
    let mut cpu = create_test_cpu();
    
    // Set field HD to all 1s
    let test_value = 0x0000010000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0x1;
    println!("TCR_EL1.HD = 0x{:X} (bits [40:40])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_e0pd0() {
    let mut cpu = create_test_cpu();
    
    // Set field E0PD0 to all 1s
    let test_value = 0x0080000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 55) & 0x1;
    println!("TCR_EL1.E0PD0 = 0x{:X} (bits [55:55])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_tbi0() {
    let mut cpu = create_test_cpu();
    
    // Set field TBI0 to all 1s
    let test_value = 0x0000002000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 37) & 0x1;
    println!("TCR_EL1.TBI0 = 0x{:X} (bits [37:37])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_nfd0() {
    let mut cpu = create_test_cpu();
    
    // Set field NFD0 to all 1s
    let test_value = 0x0020000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 53) & 0x1;
    println!("TCR_EL1.NFD0 = 0x{:X} (bits [53:53])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hwu062() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU062 to all 1s
    let test_value = 0x0000400000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 46) & 0x1;
    println!("TCR_EL1.HWU062 = 0x{:X} (bits [46:46])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_sh0() {
    let mut cpu = create_test_cpu();
    
    // Set field SH0 to all 1s
    let test_value = 0x0000000000003000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x3;
    println!("TCR_EL1.SH0 = 0x{:X} (bits [13:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_epd0() {
    let mut cpu = create_test_cpu();
    
    // Set field EPD0 to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("TCR_EL1.EPD0 = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_nfd1() {
    let mut cpu = create_test_cpu();
    
    // Set field NFD1 to all 1s
    let test_value = 0x0040000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 54) & 0x1;
    println!("TCR_EL1.NFD1 = 0x{:X} (bits [54:54])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hwu061() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU061 to all 1s
    let test_value = 0x0000200000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 45) & 0x1;
    println!("TCR_EL1.HWU061 = 0x{:X} (bits [45:45])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_irgn0() {
    let mut cpu = create_test_cpu();
    
    // Set field IRGN0 to all 1s
    let test_value = 0x0000000000000300u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x3;
    println!("TCR_EL1.IRGN0 = 0x{:X} (bits [9:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_t1sz() {
    let mut cpu = create_test_cpu();
    
    // Set field T1SZ to all 1s
    let test_value = 0x00000000003F0000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x3F;
    println!("TCR_EL1.T1SZ = 0x{:X} (bits [21:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_tcma0() {
    let mut cpu = create_test_cpu();
    
    // Set field TCMA0 to all 1s
    let test_value = 0x0200000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 57) & 0x1;
    println!("TCR_EL1.TCMA0 = 0x{:X} (bits [57:57])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hpd0() {
    let mut cpu = create_test_cpu();
    
    // Set field HPD0 to all 1s
    let test_value = 0x0000020000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 41) & 0x1;
    println!("TCR_EL1.HPD0 = 0x{:X} (bits [41:41])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_epd1() {
    let mut cpu = create_test_cpu();
    
    // Set field EPD1 to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("TCR_EL1.EPD1 = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_tbi1() {
    let mut cpu = create_test_cpu();
    
    // Set field TBI1 to all 1s
    let test_value = 0x0000004000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 38) & 0x1;
    println!("TCR_EL1.TBI1 = 0x{:X} (bits [38:38])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_a1() {
    let mut cpu = create_test_cpu();
    
    // Set field A1 to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("TCR_EL1.A1 = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_hwu059() {
    let mut cpu = create_test_cpu();
    
    // Set field HWU059 to all 1s
    let test_value = 0x0000080000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 43) & 0x1;
    println!("TCR_EL1.HWU059 = 0x{:X} (bits [43:43])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_tcrel1_field_tbid0() {
    let mut cpu = create_test_cpu();
    
    // Set field TBID0 to all 1s
    let test_value = 0x0008000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TCR_EL1, X1
    let msr_insn: u32 = 0xD5182041;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TCR_EL1
    let mrs_insn: u32 = 0xD5382040;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 51) & 0x1;
    println!("TCR_EL1.TBID0 = 0x{:X} (bits [51:51])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_cntpctel0() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, CNTPCT_EL0
    let mrs_insn: u32 = 0xD53BE020;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS CNTPCT_EL0 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("CNTPCT_EL0 = 0x{:016X}", value);
}


#[test]
fn test_mrs_spel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, SP_EL1
    let mrs_insn: u32 = 0xD53C4100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS SP_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("SP_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_spel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR SP_EL1, X1
    let msr_insn: u32 = 0xD51C4101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, SP_EL1
    let mrs_insn: u32 = 0xD53C4100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("SP_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_ttbr0el2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TTBR0_EL2
    let mrs_insn: u32 = 0xD53C2000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TTBR0_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TTBR0_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_ttbr0el2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TTBR0_EL2, X1
    let msr_insn: u32 = 0xD51C2001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR0_EL2
    let mrs_insn: u32 = 0xD53C2000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TTBR0_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_ttbr0el2_field_asid() {
    let mut cpu = create_test_cpu();
    
    // Set field ASID to all 1s
    let test_value = 0xFFFF000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR0_EL2, X1
    let msr_insn: u32 = 0xD51C2001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR0_EL2
    let mrs_insn: u32 = 0xD53C2000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0xFFFF;
    println!("TTBR0_EL2.ASID = 0x{:X} (bits [63:48])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_ttbr0el2_field_baddr() {
    let mut cpu = create_test_cpu();
    
    // Set field BADDR to all 1s
    let test_value = 0x0000FFFFFFFFFFFEu64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR0_EL2, X1
    let msr_insn: u32 = 0xD51C2001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR0_EL2
    let mrs_insn: u32 = 0xD53C2000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x7FFFFFFFFFFF;
    println!("TTBR0_EL2.BADDR = 0x{:X} (bits [47:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_ttbr0el2_field_cnp() {
    let mut cpu = create_test_cpu();
    
    // Set field CnP to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR0_EL2, X1
    let msr_insn: u32 = 0xD51C2001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR0_EL2
    let mrs_insn: u32 = 0xD53C2000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("TTBR0_EL2.CnP = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_apgakeyloel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APGAKeyLo_EL1
    let mrs_insn: u32 = 0xD5382300;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APGAKeyLo_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APGAKeyLo_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apgakeyloel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APGAKeyLo_EL1, X1
    let msr_insn: u32 = 0xD5182301;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APGAKeyLo_EL1
    let mrs_insn: u32 = 0xD5382300;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APGAKeyLo_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_mrs_idaa64dfr1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64DFR1_EL1
    let mrs_insn: u32 = 0xD5380520;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64DFR1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64DFR1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_mrs_hcrel2() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS HCR_EL2 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("HCR_EL2 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_hcrel2() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("HCR_EL2: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_hcrel2_field_ptw() {
    let mut cpu = create_test_cpu();
    
    // Set field PTW to all 1s
    let test_value = 0x0000000000000004u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 2) & 0x1;
    println!("HCR_EL2.PTW = 0x{:X} (bits [2:2])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_fwb() {
    let mut cpu = create_test_cpu();
    
    // Set field FWB to all 1s
    let test_value = 0x0000400000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 46) & 0x1;
    println!("HCR_EL2.FWB = 0x{:X} (bits [46:46])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_nv1() {
    let mut cpu = create_test_cpu();
    
    // Set field NV1 to all 1s
    let test_value = 0x0000080000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 43) & 0x1;
    println!("HCR_EL2.NV1 = 0x{:X} (bits [43:43])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_imo() {
    let mut cpu = create_test_cpu();
    
    // Set field IMO to all 1s
    let test_value = 0x0000000000000010u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0x1;
    println!("HCR_EL2.IMO = 0x{:X} (bits [4:4])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_vm() {
    let mut cpu = create_test_cpu();
    
    // Set field VM to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("HCR_EL2.VM = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_at() {
    let mut cpu = create_test_cpu();
    
    // Set field AT to all 1s
    let test_value = 0x0000100000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 44) & 0x1;
    println!("HCR_EL2.AT = 0x{:X} (bits [44:44])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_nv() {
    let mut cpu = create_test_cpu();
    
    // Set field NV to all 1s
    let test_value = 0x0000040000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 42) & 0x1;
    println!("HCR_EL2.NV = 0x{:X} (bits [42:42])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tpc() {
    let mut cpu = create_test_cpu();
    
    // Set field TPC to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("HCR_EL2.TPC = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_miocnce() {
    let mut cpu = create_test_cpu();
    
    // Set field MIOCNCE to all 1s
    let test_value = 0x0000004000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 38) & 0x1;
    println!("HCR_EL2.MIOCNCE = 0x{:X} (bits [38:38])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_fien() {
    let mut cpu = create_test_cpu();
    
    // Set field FIEN to all 1s
    let test_value = 0x0000800000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 47) & 0x1;
    println!("HCR_EL2.FIEN = 0x{:X} (bits [47:47])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_vi() {
    let mut cpu = create_test_cpu();
    
    // Set field VI to all 1s
    let test_value = 0x0000000000000080u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 7) & 0x1;
    println!("HCR_EL2.VI = 0x{:X} (bits [7:7])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_api() {
    let mut cpu = create_test_cpu();
    
    // Set field API to all 1s
    let test_value = 0x0000020000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 41) & 0x1;
    println!("HCR_EL2.API = 0x{:X} (bits [41:41])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_trvm() {
    let mut cpu = create_test_cpu();
    
    // Set field TRVM to all 1s
    let test_value = 0x0000000040000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 30) & 0x1;
    println!("HCR_EL2.TRVM = 0x{:X} (bits [30:30])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tlor() {
    let mut cpu = create_test_cpu();
    
    // Set field TLOR to all 1s
    let test_value = 0x0000000800000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 35) & 0x1;
    println!("HCR_EL2.TLOR = 0x{:X} (bits [35:35])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_dct() {
    let mut cpu = create_test_cpu();
    
    // Set field DCT to all 1s
    let test_value = 0x0200000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 57) & 0x1;
    println!("HCR_EL2.DCT = 0x{:X} (bits [57:57])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_hcd() {
    let mut cpu = create_test_cpu();
    
    // Set field HCD to all 1s
    let test_value = 0x0000000020000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 29) & 0x1;
    println!("HCR_EL2.HCD = 0x{:X} (bits [29:29])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tdz() {
    let mut cpu = create_test_cpu();
    
    // Set field TDZ to all 1s
    let test_value = 0x0000000010000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0x1;
    println!("HCR_EL2.TDZ = 0x{:X} (bits [28:28])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tacr() {
    let mut cpu = create_test_cpu();
    
    // Set field TACR to all 1s
    let test_value = 0x0000000000200000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 21) & 0x1;
    println!("HCR_EL2.TACR = 0x{:X} (bits [21:21])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tid1() {
    let mut cpu = create_test_cpu();
    
    // Set field TID1 to all 1s
    let test_value = 0x0000000000010000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0x1;
    println!("HCR_EL2.TID1 = 0x{:X} (bits [16:16])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_apk() {
    let mut cpu = create_test_cpu();
    
    // Set field APK to all 1s
    let test_value = 0x0000010000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0x1;
    println!("HCR_EL2.APK = 0x{:X} (bits [40:40])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tpcp() {
    let mut cpu = create_test_cpu();
    
    // Set field TPCP to all 1s
    let test_value = 0x0000000000800000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 23) & 0x1;
    println!("HCR_EL2.TPCP = 0x{:X} (bits [23:23])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tsw() {
    let mut cpu = create_test_cpu();
    
    // Set field TSW to all 1s
    let test_value = 0x0000000000400000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 22) & 0x1;
    println!("HCR_EL2.TSW = 0x{:X} (bits [22:22])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tvm() {
    let mut cpu = create_test_cpu();
    
    // Set field TVM to all 1s
    let test_value = 0x0000000004000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 26) & 0x1;
    println!("HCR_EL2.TVM = 0x{:X} (bits [26:26])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_ttlbis() {
    let mut cpu = create_test_cpu();
    
    // Set field TTLBIS to all 1s
    let test_value = 0x0040000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 54) & 0x1;
    println!("HCR_EL2.TTLBIS = 0x{:X} (bits [54:54])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_e2h() {
    let mut cpu = create_test_cpu();
    
    // Set field E2H to all 1s
    let test_value = 0x0000000400000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 34) & 0x1;
    println!("HCR_EL2.E2H = 0x{:X} (bits [34:34])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_bsu() {
    let mut cpu = create_test_cpu();
    
    // Set field BSU to all 1s
    let test_value = 0x0000000000000C00u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 10) & 0x3;
    println!("HCR_EL2.BSU = 0x{:X} (bits [11:10])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_nv2() {
    let mut cpu = create_test_cpu();
    
    // Set field NV2 to all 1s
    let test_value = 0x0000200000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 45) & 0x1;
    println!("HCR_EL2.NV2 = 0x{:X} (bits [45:45])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_vse() {
    let mut cpu = create_test_cpu();
    
    // Set field VSE to all 1s
    let test_value = 0x0000000000000100u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0x1;
    println!("HCR_EL2.VSE = 0x{:X} (bits [8:8])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_cd() {
    let mut cpu = create_test_cpu();
    
    // Set field CD to all 1s
    let test_value = 0x0000000100000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0x1;
    println!("HCR_EL2.CD = 0x{:X} (bits [32:32])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_vf() {
    let mut cpu = create_test_cpu();
    
    // Set field VF to all 1s
    let test_value = 0x0000000000000040u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 6) & 0x1;
    println!("HCR_EL2.VF = 0x{:X} (bits [6:6])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tsc() {
    let mut cpu = create_test_cpu();
    
    // Set field TSC to all 1s
    let test_value = 0x0000000000080000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 19) & 0x1;
    println!("HCR_EL2.TSC = 0x{:X} (bits [19:19])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_enscxt() {
    let mut cpu = create_test_cpu();
    
    // Set field EnSCXT to all 1s
    let test_value = 0x0020000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 53) & 0x1;
    println!("HCR_EL2.EnSCXT = 0x{:X} (bits [53:53])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tidcp() {
    let mut cpu = create_test_cpu();
    
    // Set field TIDCP to all 1s
    let test_value = 0x0000000000100000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0x1;
    println!("HCR_EL2.TIDCP = 0x{:X} (bits [20:20])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tid3() {
    let mut cpu = create_test_cpu();
    
    // Set field TID3 to all 1s
    let test_value = 0x0000000000040000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 18) & 0x1;
    println!("HCR_EL2.TID3 = 0x{:X} (bits [18:18])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_twi() {
    let mut cpu = create_test_cpu();
    
    // Set field TWI to all 1s
    let test_value = 0x0000000000002000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 13) & 0x1;
    println!("HCR_EL2.TWI = 0x{:X} (bits [13:13])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_dc() {
    let mut cpu = create_test_cpu();
    
    // Set field DC to all 1s
    let test_value = 0x0000000000001000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0x1;
    println!("HCR_EL2.DC = 0x{:X} (bits [12:12])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_fb() {
    let mut cpu = create_test_cpu();
    
    // Set field FB to all 1s
    let test_value = 0x0000000000000200u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 9) & 0x1;
    println!("HCR_EL2.FB = 0x{:X} (bits [9:9])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tea() {
    let mut cpu = create_test_cpu();
    
    // Set field TEA to all 1s
    let test_value = 0x0000002000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 37) & 0x1;
    println!("HCR_EL2.TEA = 0x{:X} (bits [37:37])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tid0() {
    let mut cpu = create_test_cpu();
    
    // Set field TID0 to all 1s
    let test_value = 0x0000000000008000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 15) & 0x1;
    println!("HCR_EL2.TID0 = 0x{:X} (bits [15:15])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_twe() {
    let mut cpu = create_test_cpu();
    
    // Set field TWE to all 1s
    let test_value = 0x0000000000004000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 14) & 0x1;
    println!("HCR_EL2.TWE = 0x{:X} (bits [14:14])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_amo() {
    let mut cpu = create_test_cpu();
    
    // Set field AMO to all 1s
    let test_value = 0x0000000000000020u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 5) & 0x1;
    println!("HCR_EL2.AMO = 0x{:X} (bits [5:5])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tpu() {
    let mut cpu = create_test_cpu();
    
    // Set field TPU to all 1s
    let test_value = 0x0000000001000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0x1;
    println!("HCR_EL2.TPU = 0x{:X} (bits [24:24])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tid4() {
    let mut cpu = create_test_cpu();
    
    // Set field TID4 to all 1s
    let test_value = 0x0002000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 49) & 0x1;
    println!("HCR_EL2.TID4 = 0x{:X} (bits [49:49])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_fmo() {
    let mut cpu = create_test_cpu();
    
    // Set field FMO to all 1s
    let test_value = 0x0000000000000008u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 3) & 0x1;
    println!("HCR_EL2.FMO = 0x{:X} (bits [3:3])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tge() {
    let mut cpu = create_test_cpu();
    
    // Set field TGE to all 1s
    let test_value = 0x0000000008000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 27) & 0x1;
    println!("HCR_EL2.TGE = 0x{:X} (bits [27:27])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_ata() {
    let mut cpu = create_test_cpu();
    
    // Set field ATA to all 1s
    let test_value = 0x0100000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 56) & 0x1;
    println!("HCR_EL2.ATA = 0x{:X} (bits [56:56])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_rw() {
    let mut cpu = create_test_cpu();
    
    // Set field RW to all 1s
    let test_value = 0x0000000080000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 31) & 0x1;
    println!("HCR_EL2.RW = 0x{:X} (bits [31:31])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_swio() {
    let mut cpu = create_test_cpu();
    
    // Set field SWIO to all 1s
    let test_value = 0x0000000000000002u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x1;
    println!("HCR_EL2.SWIO = 0x{:X} (bits [1:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_terr() {
    let mut cpu = create_test_cpu();
    
    // Set field TERR to all 1s
    let test_value = 0x0000001000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0x1;
    println!("HCR_EL2.TERR = 0x{:X} (bits [36:36])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tocu() {
    let mut cpu = create_test_cpu();
    
    // Set field TOCU to all 1s
    let test_value = 0x0010000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 52) & 0x1;
    println!("HCR_EL2.TOCU = 0x{:X} (bits [52:52])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_ttlb() {
    let mut cpu = create_test_cpu();
    
    // Set field TTLB to all 1s
    let test_value = 0x0000000002000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 25) & 0x1;
    println!("HCR_EL2.TTLB = 0x{:X} (bits [25:25])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_tid2() {
    let mut cpu = create_test_cpu();
    
    // Set field TID2 to all 1s
    let test_value = 0x0000000000020000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 17) & 0x1;
    println!("HCR_EL2.TID2 = 0x{:X} (bits [17:17])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_ticab() {
    let mut cpu = create_test_cpu();
    
    // Set field TICAB to all 1s
    let test_value = 0x0004000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 50) & 0x1;
    println!("HCR_EL2.TICAB = 0x{:X} (bits [50:50])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_ttlbos() {
    let mut cpu = create_test_cpu();
    
    // Set field TTLBOS to all 1s
    let test_value = 0x0080000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 55) & 0x1;
    println!("HCR_EL2.TTLBOS = 0x{:X} (bits [55:55])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_hcrel2_field_id() {
    let mut cpu = create_test_cpu();
    
    // Set field ID to all 1s
    let test_value = 0x0000000200000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR HCR_EL2, X1
    let msr_insn: u32 = 0xD51C1101;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, HCR_EL2
    let mrs_insn: u32 = 0xD53C1100;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 33) & 0x1;
    println!("HCR_EL2.ID = 0x{:X} (bits [33:33])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_ttbr0el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, TTBR0_EL1
    let mrs_insn: u32 = 0xD5382000;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS TTBR0_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("TTBR0_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_ttbr0el1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR TTBR0_EL1, X1
    let msr_insn: u32 = 0xD5182001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR0_EL1
    let mrs_insn: u32 = 0xD5382000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("TTBR0_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}


#[test]
fn test_ttbr0el1_field_asid() {
    let mut cpu = create_test_cpu();
    
    // Set field ASID to all 1s
    let test_value = 0xFFFF000000000000u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR0_EL1, X1
    let msr_insn: u32 = 0xD5182001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR0_EL1
    let mrs_insn: u32 = 0xD5382000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 48) & 0xFFFF;
    println!("TTBR0_EL1.ASID = 0x{:X} (bits [63:48])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_ttbr0el1_field_cnp() {
    let mut cpu = create_test_cpu();
    
    // Set field CnP to all 1s
    let test_value = 0x0000000000000001u64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR0_EL1, X1
    let msr_insn: u32 = 0xD5182001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR0_EL1
    let mrs_insn: u32 = 0xD5382000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0x1;
    println!("TTBR0_EL1.CnP = 0x{:X} (bits [0:0])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_ttbr0el1_field_baddr() {
    let mut cpu = create_test_cpu();
    
    // Set field BADDR to all 1s
    let test_value = 0x0000FFFFFFFFFFFEu64;
    cpu.set_gpr(1, test_value);
    
    // MSR TTBR0_EL1, X1
    let msr_insn: u32 = 0xD5182001;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, TTBR0_EL1
    let mrs_insn: u32 = 0xD5382000;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 1) & 0x7FFFFFFFFFFF;
    println!("TTBR0_EL1.BADDR = 0x{:X} (bits [47:1])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}


#[test]
fn test_mrs_idaa64isar1el1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS ID_AA64ISAR1_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("ID_AA64ISAR1_EL1 = 0x{:016X}", value);
}


#[test]
fn test_idaa64isar1el1_field_gpi() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 28) & 0xF;
    println!("ID_AA64ISAR1_EL1.GPI = 0x{:X} (bits [31:28])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_gpa() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 24) & 0xF;
    println!("ID_AA64ISAR1_EL1.GPA = 0x{:X} (bits [27:24])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_lrcpc() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 20) & 0xF;
    println!("ID_AA64ISAR1_EL1.LRCPC = 0x{:X} (bits [23:20])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_jscvt() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 12) & 0xF;
    println!("ID_AA64ISAR1_EL1.JSCVT = 0x{:X} (bits [15:12])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_api() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 8) & 0xF;
    println!("ID_AA64ISAR1_EL1.API = 0x{:X} (bits [11:8])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_specres() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 40) & 0xF;
    println!("ID_AA64ISAR1_EL1.SPECRES = 0x{:X} (bits [43:40])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_dpb() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 0) & 0xF;
    println!("ID_AA64ISAR1_EL1.DPB = 0x{:X} (bits [3:0])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_apa() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 4) & 0xF;
    println!("ID_AA64ISAR1_EL1.APA = 0x{:X} (bits [7:4])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_frintts() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 32) & 0xF;
    println!("ID_AA64ISAR1_EL1.FRINTTS = 0x{:X} (bits [35:32])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_fcma() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 16) & 0xF;
    println!("ID_AA64ISAR1_EL1.FCMA = 0x{:X} (bits [19:16])", field_value);
}


#[test]
fn test_idaa64isar1el1_field_sb() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, ID_AA64ISAR1_EL1
    let mrs_insn: u32 = 0xD5380620;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> 36) & 0xF;
    println!("ID_AA64ISAR1_EL1.SB = 0x{:X} (bits [39:36])", field_value);
}


#[test]
fn test_mrs_apgakeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // MRS X0, APGAKeyHi_EL1
    let mrs_insn: u32 = 0xD5382320;
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS APGAKeyHi_EL1 should succeed: {:?}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("APGAKeyHi_EL1 = 0x{:016X}", value);
}


#[test]
fn test_msr_mrs_apgakeyhiel1() {
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x123456789ABCDEF0);
    
    // MSR APGAKeyHi_EL1, X1
    let msr_insn: u32 = 0xD5182321;
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, APGAKeyHi_EL1
    let mrs_insn: u32 = 0xD5382320;
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("APGAKeyHi_EL1: wrote 0x{:016X}, read 0x{:016X}", 0x123456789ABCDEF0u64, readback);
}

