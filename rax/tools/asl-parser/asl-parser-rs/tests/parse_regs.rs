//! Integration tests for parsing ARM ASL register definition files.
//!
//! Tests parsing of arm_regs.asl which contains system register definitions
//! with their fields, widths, and array specifications.

use asl_parser::{parse_registers, RegisterDefinition};
use std::fs;
use std::path::Path;

/// Path to the ARM registers ASL file
const ARM_REGS_PATH: &str = "../../../docs/architecture/arm/asl/arm_regs.asl";

/// Helper to get the absolute path to arm_regs.asl
fn get_regs_path() -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    Path::new(&manifest_dir).join(ARM_REGS_PATH)
}

/// Test that arm_regs.asl can be parsed without errors
#[test]
fn test_parse_arm_regs_file() {
    let path = get_regs_path();
    if !path.exists() {
        eprintln!("Skipping test: {} not found", path.display());
        return;
    }

    let source = fs::read_to_string(&path).expect("Failed to read arm_regs.asl");
    let result = parse_registers(&source);

    match result {
        Ok(regs) => {
            println!("Successfully parsed {} register definitions", regs.len());
            assert!(regs.len() > 50, "Expected at least 50 register definitions");
        }
        Err(e) => {
            // Print context around the error
            let start = e.span.start.saturating_sub(100);
            let end = (e.span.end + 100).min(source.len());
            eprintln!("Parse error at {:?}: {:?}", e.span, e.kind);
            eprintln!("Context: ...{}...", &source[start..end]);
            panic!("Failed to parse arm_regs.asl: {:?}", e);
        }
    }
}

/// Test parsing a simple 32-bit register with fields
#[test]
fn test_parse_simple_register() {
    let source =
        "__register 32 { 31:31 M, 24:24 MT, 15:8 Aff1, 23:16 Aff2, 7:0 Aff0, 30:30 U } MPIDR;
";

    let regs = parse_registers(source).expect("Failed to parse register");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Single(reg) = &regs[0] {
        assert_eq!(reg.name.as_str(), "MPIDR");
        assert_eq!(reg.width, 32);
        assert!(!reg.fields.is_empty());

        // Check some specific fields
        let field_names: Vec<_> = reg
            .fields
            .iter()
            .filter_map(|f| f.name.as_ref().map(|n| n.as_str()))
            .collect();
        assert!(field_names.contains(&"M"));
        assert!(field_names.contains(&"Aff0"));
        assert!(field_names.contains(&"Aff1"));
        assert!(field_names.contains(&"Aff2"));
    } else {
        panic!("Expected Single register, got {:?}", regs[0]);
    }
}

/// Test parsing a 64-bit register
#[test]
fn test_parse_64bit_register() {
    let source = "__register 64 { 63:53 RESS, 52:12 BADDR } VNCR_EL2;
";

    let regs = parse_registers(source).expect("Failed to parse 64-bit register");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Single(reg) = &regs[0] {
        assert_eq!(reg.name.as_str(), "VNCR_EL2");
        assert_eq!(reg.width, 64);
        assert_eq!(reg.fields.len(), 2);
    } else {
        panic!("Expected Single register");
    }
}

/// Test parsing a register array
#[test]
fn test_parse_register_array() {
    let source = "array [0..15] of __register 32 { 31:2 VA, 31:0 ContextID } DBGBVR;
";

    let regs = parse_registers(source).expect("Failed to parse register array");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Array(arr) = &regs[0] {
        assert_eq!(arr.register.name.as_str(), "DBGBVR");
        assert_eq!(arr.register.width, 32);
        assert_eq!(arr.index_min, 0);
        assert_eq!(arr.index_max, 15);
    } else {
        panic!("Expected Array register, got {:?}", regs[0]);
    }
}

/// Test parsing register with empty field list
#[test]
fn test_parse_register_empty_fields() {
    let source = "__register 32 {  } ACTLR;
";

    let regs = parse_registers(source).expect("Failed to parse register with empty fields");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Single(reg) = &regs[0] {
        assert_eq!(reg.name.as_str(), "ACTLR");
        assert_eq!(reg.width, 32);
        assert!(reg.fields.is_empty());
    } else {
        panic!("Expected Single register");
    }
}

/// Test parsing SCTLR_EL1 (complex register with many fields)
#[test]
fn test_parse_sctlr_register() {
    let source = "__register 64 { 14:14 DZE, 9:9 UMA, 26:26 UCI, 28:28 nTLSMD, 12:12 I, 36:36 BT1, 35:35 BT0, 0:0 M } SCTLR_EL1;
";

    let regs = parse_registers(source).expect("Failed to parse SCTLR_EL1");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Single(reg) = &regs[0] {
        assert_eq!(reg.name.as_str(), "SCTLR_EL1");
        assert_eq!(reg.width, 64);

        // Check important control bits
        let field_names: Vec<_> = reg
            .fields
            .iter()
            .filter_map(|f| f.name.as_ref().map(|n| n.as_str()))
            .collect();
        assert!(field_names.contains(&"M")); // MMU enable
        assert!(field_names.contains(&"I")); // Instruction cache enable
    } else {
        panic!("Expected Single register");
    }
}

/// Test parsing SPSR register (with complex field definitions)
#[test]
fn test_parse_spsr_register() {
    let source = "__register 32 { 7:7 I, 11:10 BTYPE, 23:23 UAO, 9:9 E, 29:29 C, 31:31 N, 19:16 GE, 27:27 Q, 8:8 A, 28:28 V, 6:6 F, 30:30 Z, 3:0 M } SPSR_EL3;
";

    let regs = parse_registers(source).expect("Failed to parse SPSR_EL3");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Single(reg) = &regs[0] {
        assert_eq!(reg.name.as_str(), "SPSR_EL3");
        assert_eq!(reg.width, 32);

        // Check NZCV flags
        let field_names: Vec<_> = reg
            .fields
            .iter()
            .filter_map(|f| f.name.as_ref().map(|n| n.as_str()))
            .collect();
        assert!(field_names.contains(&"N"));
        assert!(field_names.contains(&"Z"));
        assert!(field_names.contains(&"C"));
        assert!(field_names.contains(&"V"));
        assert!(field_names.contains(&"M")); // Mode bits
    } else {
        panic!("Expected Single register");
    }
}

/// Test parsing GIC registers
#[test]
fn test_parse_gic_registers() {
    let source = "__register 32 { 24:0 INTID } GICV_DIR;
__register 32 { 12:0 INTID } GICD_SETSPI_NSR;
__register 32 { 23:0 INTID } GICC_DIR;
";

    let regs = parse_registers(source).expect("Failed to parse GIC registers");
    assert_eq!(regs.len(), 3);

    for reg_def in &regs {
        if let RegisterDefinition::Single(reg) = reg_def {
            assert!(
                reg.name.starts_with("GIC"),
                "Expected GIC register, got {}",
                reg.name
            );
            assert_eq!(reg.width, 32);

            // All have INTID field
            assert!(reg
                .fields
                .iter()
                .any(|f| f.name.as_ref().map(|n| n.as_str()) == Some("INTID")));
        }
    }
}

/// Test parsing Performance Monitor registers array
#[test]
fn test_parse_pmevtyper_array() {
    let source = "array [0..30] of __register 32 { 28:28 NSU, 27:27 NSH, 15:10, 9:0 evtCount, 25:25 MT, 30:30 U, 29:29 NSK, 31:31 P } PMEVTYPER;
";

    let regs = parse_registers(source).expect("Failed to parse PMEVTYPER array");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Array(arr) = &regs[0] {
        assert_eq!(arr.register.name.as_str(), "PMEVTYPER");
        assert_eq!(arr.index_min, 0);
        assert_eq!(arr.index_max, 30);
        assert_eq!(arr.register.width, 32);
    } else {
        panic!("Expected Array register");
    }
}

/// Test parsing Counter-timer registers
#[test]
fn test_parse_counter_timer_registers() {
    let source = "__register 64 { 63:0 CompareValue } CNTV_CVAL;
__register 32 { 31:0 TimerValue } CNTHPS_TVAL_EL2;
";

    let regs = parse_registers(source).expect("Failed to parse counter-timer registers");
    assert_eq!(regs.len(), 2);

    // Check 64-bit register
    if let RegisterDefinition::Single(reg) = &regs[0] {
        assert_eq!(reg.name.as_str(), "CNTV_CVAL");
        assert_eq!(reg.width, 64);
    }

    // Check 32-bit register
    if let RegisterDefinition::Single(reg) = &regs[1] {
        assert_eq!(reg.name.as_str(), "CNTHPS_TVAL_EL2");
        assert_eq!(reg.width, 32);
    }
}

/// Test parsing Activity Monitor registers array
#[test]
fn test_parse_amevtyper_array() {
    let source = "array [0..15] of __register 64 { 15:0 evtCount } AMEVTYPER0_EL0;
";

    let regs = parse_registers(source).expect("Failed to parse AMEVTYPER array");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Array(arr) = &regs[0] {
        assert_eq!(arr.register.name.as_str(), "AMEVTYPER0_EL0");
        assert_eq!(arr.index_min, 0);
        assert_eq!(arr.index_max, 15);
        assert_eq!(arr.register.width, 64);
    } else {
        panic!("Expected Array register");
    }
}

/// Test parsing ICV (Interrupt Controller Virtual) registers array
#[test]
fn test_parse_icv_array() {
    let source = "array [0..3] of __register 32 {  } ICV_AP1R;
";

    let regs = parse_registers(source).expect("Failed to parse ICV array");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Array(arr) = &regs[0] {
        assert_eq!(arr.register.name.as_str(), "ICV_AP1R");
        assert_eq!(arr.index_min, 0);
        assert_eq!(arr.index_max, 3);
        assert!(arr.register.fields.is_empty());
    } else {
        panic!("Expected Array register");
    }
}

/// Count and categorize registers in the full arm_regs.asl file
#[test]
fn test_count_register_types() {
    let path = get_regs_path();
    if !path.exists() {
        eprintln!("Skipping test: {} not found", path.display());
        return;
    }

    let source = fs::read_to_string(&path).expect("Failed to read arm_regs.asl");
    let regs = match parse_registers(&source) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Parse failed, skipping count test: {:?}", e);
            return;
        }
    };

    let mut single_32 = 0;
    let mut single_64 = 0;
    let mut arrays = 0;

    for reg_def in &regs {
        match reg_def {
            RegisterDefinition::Single(reg) => {
                if reg.width == 32 {
                    single_32 += 1;
                } else if reg.width == 64 {
                    single_64 += 1;
                }
            }
            RegisterDefinition::Array(_) => {
                arrays += 1;
            }
        }
    }

    println!("\nRegister counts:");
    println!("  32-bit registers: {}", single_32);
    println!("  64-bit registers: {}", single_64);
    println!("  Register arrays: {}", arrays);
    println!("  Total: {}", regs.len());

    assert!(single_32 > 0, "Expected at least some 32-bit registers");
    assert!(single_64 > 0, "Expected at least some 64-bit registers");
}

/// Test that field bit ranges are parsed correctly
#[test]
fn test_field_bit_ranges() {
    let source =
        "__register 32 { 31:24 high_byte, 23:16 mid_high, 15:8 mid_low, 7:0 low_byte } TEST_REG;
";

    let regs = parse_registers(source).expect("Failed to parse register with bit ranges");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Single(reg) = &regs[0] {
        assert_eq!(reg.fields.len(), 4);

        // Check that fields have correct bit ranges
        for field in &reg.fields {
            if let Some(name) = &field.name {
                match name.as_str() {
                    "high_byte" => {
                        assert_eq!(field.hi, 31);
                        assert_eq!(field.lo, 24);
                    }
                    "low_byte" => {
                        assert_eq!(field.hi, 7);
                        assert_eq!(field.lo, 0);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Test parsing register with overlapping fields (allowed in ASL)
#[test]
fn test_overlapping_fields() {
    let source = "__register 32 { 31:2 VA, 31:0 ContextID } DBGBVR_SINGLE;
";

    let regs = parse_registers(source).expect("Failed to parse register with overlapping fields");
    assert_eq!(regs.len(), 1);

    if let RegisterDefinition::Single(reg) = &regs[0] {
        assert_eq!(reg.fields.len(), 2);
        // Both fields should be present even though they overlap
        let field_names: Vec<_> = reg
            .fields
            .iter()
            .filter_map(|f| f.name.as_ref().map(|n| n.as_str()))
            .collect();
        assert!(field_names.contains(&"VA"));
        assert!(field_names.contains(&"ContextID"));
    }
}
