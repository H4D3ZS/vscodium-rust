//! Integration tests for parsing ARM ASL definition files.
//!
//! Tests parsing of arm_defs.asl which contains:
//! - Constants (e.g., EL0, EL1, EL2, EL3)
//! - Enumerations (e.g., ArchVersion, Fault, AccType)
//! - Type definitions (e.g., FullAddress, FaultRecord)
//! - Functions and procedures
//! - Variables and arrays

use asl_parser::{parse_definitions, Definition};
use std::fs;
use std::path::Path;

/// Path to the ARM definitions ASL file
const ARM_DEFS_PATH: &str = "../../../docs/architecture/arm/asl/arm_defs.asl";

/// Helper to get the absolute path to arm_defs.asl
fn get_defs_path() -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    Path::new(&manifest_dir).join(ARM_DEFS_PATH)
}

/// Test that arm_defs.asl can be parsed without errors
#[test]
fn test_parse_arm_defs_file() {
    let path = get_defs_path();
    if !path.exists() {
        eprintln!("Skipping test: {} not found", path.display());
        return;
    }

    let source = fs::read_to_string(&path).expect("Failed to read arm_defs.asl");
    let result = parse_definitions(&source);

    match result {
        Ok(defs) => {
            println!("Successfully parsed {} definitions", defs.len());
            assert!(defs.len() > 100, "Expected at least 100 definitions");
        }
        Err(e) => {
            // Print context around the error
            let start = e.span.start.saturating_sub(100);
            let end = (e.span.end + 100).min(source.len());
            eprintln!("Parse error at {:?}: {:?}", e.span, e.kind);
            eprintln!("Context: ...{}...", &source[start..end]);
            panic!("Failed to parse arm_defs.asl: {:?}", e);
        }
    }
}

/// Test that specific constants are parsed correctly
#[test]
fn test_parse_exception_level_constants() {
    let source = "constant bits(2) EL3 = '11';
constant bits(2) EL2 = '10';
constant bits(2) EL1 = '01';
constant bits(2) EL0 = '00';
";

    let defs = parse_definitions(source).expect("Failed to parse constants");
    assert_eq!(defs.len(), 4, "Expected 4 constants");

    let el_names: Vec<_> = defs
        .iter()
        .filter_map(|d| {
            if let Definition::Const { name, .. } = d {
                Some(name.as_str())
            } else {
                None
            }
        })
        .collect();

    assert!(el_names.contains(&"EL0"));
    assert!(el_names.contains(&"EL1"));
    assert!(el_names.contains(&"EL2"));
    assert!(el_names.contains(&"EL3"));
}

/// Test that enumerations are parsed correctly
#[test]
fn test_parse_enumeration() {
    let source = "enumeration ArchVersion {
    ARMv8p0
    , ARMv8p1
    , ARMv8p2
    , ARMv8p3
    , ARMv8p4
    , ARMv8p5
};
";

    let defs = parse_definitions(source).expect("Failed to parse enumeration");
    assert_eq!(defs.len(), 1, "Expected 1 enumeration");

    if let Definition::TypeEnum { name, variants } = &defs[0] {
        assert_eq!(name.as_str(), "ArchVersion");
        assert_eq!(variants.len(), 6);
        assert!(variants.iter().any(|v| v == "ARMv8p0"));
        assert!(variants.iter().any(|v| v == "ARMv8p5"));
    } else {
        panic!("Expected TypeEnum, got {:?}", defs[0]);
    }
}

/// Test that shift type enumeration is parsed correctly
#[test]
fn test_parse_shift_type_enum() {
    let source =
        "enumeration ShiftType {ShiftType_LSL, ShiftType_LSR, ShiftType_ASR, ShiftType_ROR};
";

    let defs = parse_definitions(source).expect("Failed to parse ShiftType enum");
    assert_eq!(defs.len(), 1);

    if let Definition::TypeEnum { name, variants } = &defs[0] {
        assert_eq!(name.as_str(), "ShiftType");
        assert_eq!(variants.len(), 4);
    } else {
        panic!("Expected TypeEnum");
    }
}

/// Test that type aliases are parsed correctly
#[test]
fn test_parse_type_alias() {
    let source = "type PARTIDtype = bits(16);
type PMGtype = bits(8);
";

    let defs = parse_definitions(source).expect("Failed to parse type aliases");
    assert_eq!(defs.len(), 2);

    for def in &defs {
        if let Definition::TypeAlias { name, .. } = def {
            assert!(name == "PARTIDtype" || name == "PMGtype");
        } else {
            panic!("Expected TypeAlias, got {:?}", def);
        }
    }
}

/// Test that function definitions are parsed correctly
#[test]
fn test_parse_function() {
    let source = "boolean IsEventRegisterSet()
    return EventRegister == '1';
";

    let defs = parse_definitions(source).expect("Failed to parse function");
    assert_eq!(defs.len(), 1);

    if let Definition::Callable {
        name,
        params,
        returns,
        ..
    } = &defs[0]
    {
        assert_eq!(name.name.as_str(), "IsEventRegisterSet");
        assert!(params.is_empty());
        assert_eq!(returns.len(), 1); // boolean return
    } else {
        panic!("Expected Callable, got {:?}", defs[0]);
    }
}

/// Test that tuple-returning functions are parsed correctly
#[test]
fn test_parse_tuple_function() {
    let source = "(bits(N), boolean) SignedSatQ(integer i, integer N)
    if i > 2^(N-1) - 1 then
        result = 2^(N-1) - 1;  saturated = TRUE;
    elsif i < -(2^(N-1)) then
        result = -(2^(N-1));  saturated = TRUE;
    else
        result = i;  saturated = FALSE;
    return (result<N-1:0>, saturated);
";

    let defs = parse_definitions(source).expect("Failed to parse tuple function");
    assert_eq!(defs.len(), 1);

    if let Definition::Callable {
        name,
        params,
        returns,
        ..
    } = &defs[0]
    {
        assert_eq!(name.name.as_str(), "SignedSatQ");
        assert_eq!(params.len(), 2); // i, N
        assert_eq!(returns.len(), 2); // bits(N), boolean
    } else {
        panic!("Expected Callable, got {:?}", defs[0]);
    }
}

/// Test that array declarations are parsed correctly
#[test]
fn test_parse_array() {
    let source = "array bits(64) _R[0..30];
";

    let defs = parse_definitions(source).expect("Failed to parse array");
    assert_eq!(defs.len(), 1);

    if let Definition::Array { name, .. } = &defs[0] {
        assert_eq!(name.as_str(), "_R");
    } else {
        panic!("Expected Array, got {:?}", defs[0]);
    }
}

/// Test that variable declarations are parsed correctly
#[test]
fn test_parse_variable() {
    let source = "boolean InGuardedPage;
bits(1) EventRegister;
";

    let defs = parse_definitions(source).expect("Failed to parse variables");
    assert_eq!(defs.len(), 2);

    let var_names: Vec<_> = defs
        .iter()
        .filter_map(|d| {
            if let Definition::Variable { name, .. } = d {
                Some(name.name.as_str())
            } else {
                None
            }
        })
        .collect();

    assert!(var_names.contains(&"InGuardedPage"));
    assert!(var_names.contains(&"EventRegister"));
}

/// Test that mode constants are parsed correctly
#[test]
fn test_parse_mode_constants() {
    let source = "constant bits(5) M32_User    = '10000';
constant bits(5) M32_FIQ     = '10001';
constant bits(5) M32_IRQ     = '10010';
constant bits(5) M32_Svc     = '10011';
constant bits(5) M32_Monitor = '10110';
";

    let defs = parse_definitions(source).expect("Failed to parse mode constants");
    assert_eq!(defs.len(), 5);

    for def in &defs {
        if let Definition::Const { name, .. } = def {
            assert!(name.starts_with("M32_"));
        } else {
            panic!("Expected Const, got {:?}", def);
        }
    }
}

/// Test parsing of complex type definitions
#[test]
fn test_parse_struct_type() {
    let source = "type Permissions is (
    boolean ap_table
    , boolean xn_table
    , boolean xxn_table
    , boolean pxn_table
);
";

    let defs = parse_definitions(source).expect("Failed to parse struct type");
    assert_eq!(defs.len(), 1);

    if let Definition::TypeStruct { name, fields } = &defs[0] {
        assert_eq!(name.name.as_str(), "Permissions");
        assert_eq!(fields.len(), 4);
    } else {
        panic!("Expected TypeStruct, got {:?}", defs[0]);
    }
}

/// Test that procedures (void functions) are parsed correctly
#[test]
fn test_parse_procedure() {
    let source = "CheckAdvSIMDOrVFPEnabled(boolean include_fpexc_check, boolean advsimd)
    AArch32.CheckAdvSIMDOrFPEnabled(include_fpexc_check, advsimd);
    return;
";

    let defs = parse_definitions(source).expect("Failed to parse procedure");
    assert_eq!(defs.len(), 1);

    if let Definition::Callable {
        name,
        params,
        returns,
        ..
    } = &defs[0]
    {
        assert_eq!(name.name.as_str(), "CheckAdvSIMDOrVFPEnabled");
        assert_eq!(params.len(), 2);
        assert!(returns.is_empty()); // Procedure has no return type
    } else {
        panic!("Expected Callable, got {:?}", defs[0]);
    }
}

/// Count different definition types in the full arm_defs.asl file
#[test]
fn test_count_definition_types() {
    let path = get_defs_path();
    if !path.exists() {
        eprintln!("Skipping test: {} not found", path.display());
        return;
    }

    let source = fs::read_to_string(&path).expect("Failed to read arm_defs.asl");
    let defs = match parse_definitions(&source) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Parse failed, skipping count test: {:?}", e);
            return;
        }
    };

    let mut counts = std::collections::HashMap::new();

    for def in &defs {
        let kind = match def {
            Definition::TypeBuiltin(_) => "builtin",
            Definition::TypeAbstract(_) => "abstract_type",
            Definition::TypeAlias { .. } => "type_alias",
            Definition::TypeStruct { .. } => "struct",
            Definition::TypeEnum { .. } => "enum",
            Definition::Variable { .. } => "variable",
            Definition::Const { .. } => "constant",
            Definition::Array { .. } => "array",
            Definition::Callable { returns, .. } => {
                if returns.is_empty() {
                    "procedure"
                } else {
                    "function"
                }
            }
            Definition::Getter { .. } => "getter",
            Definition::Setter { .. } => "setter",
            Definition::Instruction(_) => "instruction",
            Definition::Decode { .. } => "decode",
        };
        *counts.entry(kind).or_insert(0) += 1;
    }

    println!("\nDefinition type counts:");
    for (kind, count) in &counts {
        println!("  {}: {}", kind, count);
    }

    // Verify we have a reasonable number of each type
    assert!(
        counts.get("constant").unwrap_or(&0) > &10,
        "Expected at least 10 constants"
    );
    assert!(
        counts.get("enum").unwrap_or(&0) > &5,
        "Expected at least 5 enums"
    );
    assert!(
        counts.get("function").unwrap_or(&0) > &50,
        "Expected at least 50 functions"
    );
}
