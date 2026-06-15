//! Encoding analysis utilities.
//!
//! This module provides utilities for analyzing instruction encodings,
//! including opcode patterns, field extraction, and guard conditions.

use crate::syntax::{InstructionEncoding, InstructionField, MaskBit};
use crate::testgen::types::*;

/// Analyze an instruction field and determine its properties
pub fn analyze_field(field: &InstructionField) -> FieldAnalysis {
    let name = field.name.to_string();
    let start = field.begin;
    let width = field.width;

    // Basic field analysis - semantics will be refined by InstructionAnalyzer
    FieldAnalysis {
        name,
        start,
        width,
        semantics: FieldSemantics::Unknown,
        constraints: Vec::new(),
        special_values: Vec::new(),
    }
}

/// Build an encoding value from opcode pattern and field values
pub fn build_encoding(opcode_pattern: &OpcodePattern, fields: &[(&FieldAnalysis, u64)]) -> u64 {
    let mut value = opcode_pattern.fixed_value;

    for (field, field_val) in fields {
        let mask = ((1u64 << field.width) - 1) << field.start;
        value = (value & !mask) | ((field_val << field.start) & mask);
    }

    value
}

/// Extract a field value from an encoding
pub fn extract_field(encoding: u64, field: &FieldAnalysis) -> u64 {
    let mask = (1u64 << field.width) - 1;
    (encoding >> field.start) & mask
}

/// Check if an encoding matches the opcode pattern
pub fn matches_pattern(encoding: u64, pattern: &OpcodePattern) -> bool {
    (encoding & pattern.fixed_mask) == pattern.fixed_value
}

/// Generate all possible values for a field (for exhaustive testing)
pub fn generate_field_values(field: &FieldAnalysis) -> Vec<u64> {
    let max_val = (1u64 << field.width) - 1;
    (0..=max_val).collect()
}

/// Generate boundary values for a field
pub fn generate_boundary_values(field: &FieldAnalysis) -> Vec<(u64, BoundaryType)> {
    let max_val = (1u64 << field.width) - 1;
    let mut values = Vec::new();

    // Always include min and max
    values.push((0, BoundaryType::Min));
    values.push((max_val, BoundaryType::Max));

    // For larger fields, include more boundaries
    if field.width > 1 {
        values.push((1, BoundaryType::PowerOfTwo));

        // Midpoint
        let mid = max_val / 2;
        if mid > 1 && mid < max_val {
            values.push((mid, BoundaryType::PowerOfTwoMinusOne));
        }
        let mid_plus = mid + 1;
        if mid_plus > 1 && mid_plus < max_val {
            values.push((mid_plus, BoundaryType::PowerOfTwo));
        }
    }

    // For 5-bit register fields, 31 is special
    if field.width == 5 {
        values.push((30, BoundaryType::PowerOfTwoMinusOne)); // Last "normal" register
                                                             // 31 is already included as max
    }

    values
}

/// Format an encoding as a binary string with field annotations
pub fn format_encoding_binary(encoding: u64, width: u32, fields: &[FieldAnalysis]) -> String {
    let mut result = String::new();

    for i in (0..width).rev() {
        let bit = (encoding >> i) & 1;
        result.push(if bit == 1 { '1' } else { '0' });

        // Add space every 8 bits for readability
        if i > 0 && i % 8 == 0 {
            result.push(' ');
        }
    }

    result
}

/// Format an encoding as hex
pub fn format_encoding_hex(encoding: u64, width: u32) -> String {
    match width {
        16 => format!("0x{:04X}", encoding as u16),
        32 => format!("0x{:08X}", encoding as u32),
        64 => format!("0x{:016X}", encoding),
        _ => format!("0x{:X}", encoding),
    }
}
