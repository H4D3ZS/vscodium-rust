//! Boundary value generation for test cases.
//!
//! Generates interesting boundary values based on field semantics:
//! - Register fields: 0, 1, 30, 31 (special handling for ZR/SP)
//! - Immediates: 0, 1, midpoint, max
//! - Size selectors: all values
//! - Conditions: all 16 conditions

use crate::testgen::types::*;

/// Generate boundary values for a field based on its semantics
pub fn generate_boundary_values(field: &FieldAnalysis) -> Vec<BoundaryValue> {
    let max_val = (1u64 << field.width) - 1;
    let mut values = Vec::new();

    match &field.semantics {
        FieldSemantics::GpRegister {
            allows_sp,
            allows_zr,
            ..
        } => {
            // Register fields (5-bit): test 0, 1, 30, 31
            values.push(BoundaryValue {
                value: 0,
                boundary_type: BoundaryType::Min,
                provenance_note: "register index 0 (first register)".to_string(),
            });
            values.push(BoundaryValue {
                value: 1,
                boundary_type: BoundaryType::PowerOfTwo,
                provenance_note: "register index 1 (second register)".to_string(),
            });
            if field.width >= 5 {
                values.push(BoundaryValue {
                    value: 30,
                    boundary_type: BoundaryType::PowerOfTwoMinusOne,
                    provenance_note: "register index 30 (LR in some contexts)".to_string(),
                });
                let meaning = if *allows_zr {
                    "register index 31 (ZR - zero register)"
                } else if *allows_sp {
                    "register index 31 (SP - stack pointer)"
                } else {
                    "register index 31 (special)"
                };
                values.push(BoundaryValue {
                    value: 31,
                    boundary_type: BoundaryType::Max,
                    provenance_note: meaning.to_string(),
                });
            }
        }

        FieldSemantics::SimdRegister { .. } => {
            // SIMD register fields (5-bit)
            values.push(BoundaryValue {
                value: 0,
                boundary_type: BoundaryType::Min,
                provenance_note: "SIMD register V0".to_string(),
            });
            values.push(BoundaryValue {
                value: 1,
                boundary_type: BoundaryType::PowerOfTwo,
                provenance_note: "SIMD register V1".to_string(),
            });
            if field.width >= 5 {
                values.push(BoundaryValue {
                    value: 30,
                    boundary_type: BoundaryType::PowerOfTwoMinusOne,
                    provenance_note: "SIMD register V30".to_string(),
                });
                values.push(BoundaryValue {
                    value: 31,
                    boundary_type: BoundaryType::Max,
                    provenance_note: "SIMD register V31".to_string(),
                });
            }
        }

        FieldSemantics::Immediate {
            signed,
            scale,
            offset,
        } => {
            // Immediate values: 0, 1, midpoint, max
            values.push(BoundaryValue {
                value: 0,
                boundary_type: BoundaryType::Zero,
                provenance_note: "immediate value 0".to_string(),
            });
            values.push(BoundaryValue {
                value: 1,
                boundary_type: BoundaryType::PowerOfTwo,
                provenance_note: "immediate value 1".to_string(),
            });

            // Midpoint
            let mid = max_val / 2;
            if mid > 1 {
                values.push(BoundaryValue {
                    value: mid,
                    boundary_type: BoundaryType::PowerOfTwoMinusOne,
                    provenance_note: format!("immediate midpoint ({})", mid),
                });
            }

            // For signed, midpoint+1 is first "negative" value
            if *signed && mid + 1 < max_val {
                values.push(BoundaryValue {
                    value: mid + 1,
                    boundary_type: BoundaryType::PowerOfTwo,
                    provenance_note: format!("signed immediate sign boundary ({})", mid + 1),
                });
            }

            // Max value
            values.push(BoundaryValue {
                value: max_val,
                boundary_type: BoundaryType::Max,
                provenance_note: format!("maximum immediate ({})", max_val),
            });

            // Power of 2 boundaries for larger immediates
            if field.width >= 4 {
                for pow in 2..field.width {
                    let pow_val = 1u64 << pow;
                    if pow_val < max_val && !values.iter().any(|v| v.value == pow_val) {
                        values.push(BoundaryValue {
                            value: pow_val,
                            boundary_type: BoundaryType::PowerOfTwo,
                            provenance_note: format!("power of 2 (2^{} = {})", pow, pow_val),
                        });
                    }
                    let pow_minus_1 = pow_val - 1;
                    if pow_minus_1 > 1 && !values.iter().any(|v| v.value == pow_minus_1) {
                        values.push(BoundaryValue {
                            value: pow_minus_1,
                            boundary_type: BoundaryType::PowerOfTwoMinusOne,
                            provenance_note: format!("2^{} - 1 = {}", pow, pow_minus_1),
                        });
                    }
                }
            }
        }

        FieldSemantics::SizeSelector => {
            // Size fields: test ALL values (usually small)
            for v in 0..=max_val {
                let note = match v {
                    0 => "8-bit / byte size".to_string(),
                    1 => "16-bit / halfword size".to_string(),
                    2 => "32-bit / word size".to_string(),
                    3 => "64-bit / doubleword size".to_string(),
                    _ => format!("size variant {}", v),
                };
                values.push(BoundaryValue {
                    value: v,
                    boundary_type: if v == 0 {
                        BoundaryType::Min
                    } else if v == max_val {
                        BoundaryType::Max
                    } else {
                        BoundaryType::PowerOfTwo
                    },
                    provenance_note: note,
                });
            }
        }

        FieldSemantics::Condition => {
            // Condition codes: test ALL 16
            let cond_names = [
                "EQ (equal)",
                "NE (not equal)",
                "CS/HS (carry set)",
                "CC/LO (carry clear)",
                "MI (minus/negative)",
                "PL (plus/positive)",
                "VS (overflow set)",
                "VC (overflow clear)",
                "HI (unsigned higher)",
                "LS (unsigned lower or same)",
                "GE (signed >=)",
                "LT (signed <)",
                "GT (signed >)",
                "LE (signed <=)",
                "AL (always)",
                "NV (never, reserved)",
            ];
            for (v, name) in cond_names.iter().enumerate() {
                if (v as u64) <= max_val {
                    values.push(BoundaryValue {
                        value: v as u64,
                        boundary_type: if v == 0 {
                            BoundaryType::Min
                        } else if v as u64 == max_val {
                            BoundaryType::Max
                        } else {
                            BoundaryType::PowerOfTwo
                        },
                        provenance_note: format!("condition {}", name),
                    });
                }
            }
        }

        FieldSemantics::ShiftType => {
            // Shift types: test all 4
            let shift_names = [
                "LSL (logical shift left)",
                "LSR (logical shift right)",
                "ASR (arithmetic shift right)",
                "ROR (rotate right)",
            ];
            for (v, name) in shift_names.iter().enumerate() {
                if (v as u64) <= max_val {
                    values.push(BoundaryValue {
                        value: v as u64,
                        boundary_type: if v == 0 {
                            BoundaryType::Min
                        } else if v as u64 == max_val {
                            BoundaryType::Max
                        } else {
                            BoundaryType::PowerOfTwo
                        },
                        provenance_note: format!("shift type {}", name),
                    });
                }
            }
        }

        FieldSemantics::Option => {
            // Option fields: test all values (usually 3-bit = 8 values)
            for v in 0..=max_val {
                values.push(BoundaryValue {
                    value: v,
                    boundary_type: if v == 0 {
                        BoundaryType::Min
                    } else if v == max_val {
                        BoundaryType::Max
                    } else {
                        BoundaryType::PowerOfTwo
                    },
                    provenance_note: format!("option {}", v),
                });
            }
        }

        FieldSemantics::OpcodeSelector | FieldSemantics::Unknown => {
            // Generic field: test min, max, and some boundaries
            values.push(BoundaryValue {
                value: 0,
                boundary_type: BoundaryType::Min,
                provenance_note: "minimum value".to_string(),
            });

            if max_val > 0 {
                values.push(BoundaryValue {
                    value: max_val,
                    boundary_type: BoundaryType::Max,
                    provenance_note: format!("maximum value ({})", max_val),
                });
            }

            if max_val > 1 {
                values.push(BoundaryValue {
                    value: 1,
                    boundary_type: BoundaryType::PowerOfTwo,
                    provenance_note: "value 1".to_string(),
                });
            }

            // Midpoint for larger fields
            if field.width >= 4 {
                let mid = max_val / 2;
                if mid > 1 && mid < max_val {
                    values.push(BoundaryValue {
                        value: mid,
                        boundary_type: BoundaryType::PowerOfTwoMinusOne,
                        provenance_note: format!("midpoint ({})", mid),
                    });
                }
            }
        }

        _ => {
            // Default: just min and max
            values.push(BoundaryValue {
                value: 0,
                boundary_type: BoundaryType::Min,
                provenance_note: "minimum".to_string(),
            });
            if max_val > 0 {
                values.push(BoundaryValue {
                    value: max_val,
                    boundary_type: BoundaryType::Max,
                    provenance_note: "maximum".to_string(),
                });
            }
        }
    }

    // Sort by value and remove duplicates
    values.sort_by_key(|v| v.value);
    values.dedup_by_key(|v| v.value);

    values
}

/// A boundary value with provenance information
#[derive(Debug, Clone)]
pub struct BoundaryValue {
    pub value: u64,
    pub boundary_type: BoundaryType,
    pub provenance_note: String,
}

/// Generate test value combinations for multiple fields
/// Uses pairwise/boundary combination strategy to avoid explosion
pub fn generate_field_combinations(
    fields: &[FieldAnalysis],
    max_combinations: usize,
) -> Vec<Vec<(String, u64, String)>> {
    let boundary_values: Vec<Vec<BoundaryValue>> =
        fields.iter().map(|f| generate_boundary_values(f)).collect();

    let mut combinations = Vec::new();

    // Strategy: For each field, generate tests with boundary values
    // while other fields use "neutral" values (typically 0 or 1)

    // First pass: test each field's boundaries independently
    for (field_idx, field) in fields.iter().enumerate() {
        let field_boundaries = &boundary_values[field_idx];

        for bv in field_boundaries {
            let mut combo = Vec::new();

            for (other_idx, other_field) in fields.iter().enumerate() {
                if other_idx == field_idx {
                    // Use the boundary value for this field
                    combo.push((field.name.clone(), bv.value, bv.provenance_note.clone()));
                } else {
                    // Use neutral value (0) for other fields
                    combo.push((other_field.name.clone(), 0, "neutral value".to_string()));
                }
            }

            combinations.push(combo);
        }
    }

    // Second pass: pairwise combinations of interesting values
    // Only if we haven't exceeded max_combinations
    if combinations.len() < max_combinations && fields.len() >= 2 {
        // Generate pairwise combinations for register fields
        let reg_fields: Vec<usize> = fields
            .iter()
            .enumerate()
            .filter(|(_, f)| matches!(f.semantics, FieldSemantics::GpRegister { .. }))
            .map(|(i, _)| i)
            .collect();

        if reg_fields.len() >= 2 {
            // Test same register for source/dest scenarios
            for &idx1 in &reg_fields {
                for &idx2 in &reg_fields {
                    if idx1 != idx2 {
                        // Same register (e.g., Rd == Rn)
                        for reg_val in &[0u64, 1, 31] {
                            let mut combo = Vec::new();
                            for (i, field) in fields.iter().enumerate() {
                                if i == idx1 || i == idx2 {
                                    combo.push((
                                        field.name.clone(),
                                        *reg_val,
                                        format!("same register test (reg={})", reg_val),
                                    ));
                                } else {
                                    combo.push((field.name.clone(), 0, "neutral".to_string()));
                                }
                            }
                            if !combinations
                                .iter()
                                .any(|c| c.iter().zip(combo.iter()).all(|(a, b)| a.1 == b.1))
                            {
                                combinations.push(combo);
                            }
                        }
                    }
                }
            }
        }
    }

    // Limit combinations
    if combinations.len() > max_combinations {
        combinations.truncate(max_combinations);
    }

    combinations
}

/// Generate flag-exercising values for arithmetic operations
pub fn generate_flag_test_values() -> Vec<FlagTestValue> {
    vec![
        // Zero flag tests
        FlagTestValue {
            operand1: 0,
            operand2: 0,
            expected_flags: FlagExpectation {
                n: false,
                z: true,
                c: false, // No carry from 0 + 0
                v: false,
            },
            scenario: FlagScenario::ZeroResult,
            note: "0 + 0 = 0 (Z=1)".to_string(),
        },
        FlagTestValue {
            operand1: 1,
            operand2: 0xFFFFFFFF_FFFFFFFFu64,
            expected_flags: FlagExpectation {
                n: false,
                z: true,
                c: true,
                v: false,
            },
            scenario: FlagScenario::ZeroResult,
            note: "1 + (-1) = 0 (Z=1, C=1)".to_string(),
        },
        // Negative flag tests
        FlagTestValue {
            operand1: 0x8000_0000_0000_0000,
            operand2: 0,
            expected_flags: FlagExpectation {
                n: true,
                z: false,
                c: false,
                v: false,
            },
            scenario: FlagScenario::NegativeResult,
            note: "negative value (N=1)".to_string(),
        },
        // Unsigned overflow (carry)
        FlagTestValue {
            operand1: 0xFFFFFFFF_FFFFFFFF,
            operand2: 1,
            expected_flags: FlagExpectation {
                n: false,
                z: true,
                c: true,
                v: false,
            },
            scenario: FlagScenario::UnsignedOverflow,
            note: "max + 1 = 0 (C=1, Z=1)".to_string(),
        },
        FlagTestValue {
            operand1: 0xFFFFFFFF_FFFFFFFF,
            operand2: 2,
            expected_flags: FlagExpectation {
                n: false,
                z: false,
                c: true,
                v: false,
            },
            scenario: FlagScenario::UnsignedOverflow,
            note: "max + 2 = 1 (C=1)".to_string(),
        },
        // Signed overflow
        FlagTestValue {
            operand1: 0x7FFFFFFF_FFFFFFFF, // Max positive
            operand2: 1,
            expected_flags: FlagExpectation {
                n: true,
                z: false,
                c: false,
                v: true,
            },
            scenario: FlagScenario::SignedOverflow,
            note: "max_signed + 1 = min_signed (V=1, N=1)".to_string(),
        },
        FlagTestValue {
            operand1: 0x8000_0000_0000_0000, // Min negative
            operand2: 0xFFFFFFFF_FFFFFFFF,   // -1
            expected_flags: FlagExpectation {
                n: false,
                z: false,
                c: true,
                v: true,
            },
            scenario: FlagScenario::SignedOverflow,
            note: "min_signed + (-1) = max_signed (V=1)".to_string(),
        },
        // No overflow cases
        FlagTestValue {
            operand1: 100,
            operand2: 50,
            expected_flags: FlagExpectation {
                n: false,
                z: false,
                c: false,
                v: false,
            },
            scenario: FlagScenario::PositiveResult,
            note: "100 + 50 = 150 (no flags)".to_string(),
        },
    ]
}

/// Test value for flag computation
#[derive(Debug, Clone)]
pub struct FlagTestValue {
    pub operand1: u64,
    pub operand2: u64,
    pub expected_flags: FlagExpectation,
    pub scenario: FlagScenario,
    pub note: String,
}

/// Expected flag values
#[derive(Debug, Clone)]
pub struct FlagExpectation {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
}

impl FlagExpectation {
    /// Convert to NZCV nibble
    pub fn to_nzcv(&self) -> u8 {
        let mut nzcv = 0u8;
        if self.n {
            nzcv |= 0b1000;
        }
        if self.z {
            nzcv |= 0b0100;
        }
        if self.c {
            nzcv |= 0b0010;
        }
        if self.v {
            nzcv |= 0b0001;
        }
        nzcv
    }
}
