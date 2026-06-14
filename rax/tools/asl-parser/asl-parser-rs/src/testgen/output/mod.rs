//! Test output generation.
//!
//! This module formats generated test cases into various output formats,
//! with a focus on Rust test files compatible with the rax emulator.

pub mod rust;
pub mod structured;

use crate::testgen::types::*;

// Re-export for convenience
pub use structured::{generate_structured_output, write_structured_output, OutputFile};

/// Formats test suites into output
pub struct TestOutputFormatter;

impl TestOutputFormatter {
    /// Format test suites into the specified format
    pub fn format(suites: &[InstructionTestSuite], format: OutputFormat) -> String {
        match format {
            OutputFormat::Rust => rust::format_test_suites(suites),
            OutputFormat::Json => Self::format_json(suites),
            OutputFormat::Text => Self::format_text(suites),
            _ => unimplemented!("Output format {:?} not yet implemented", format),
        }
    }

    fn format_json(suites: &[InstructionTestSuite]) -> String {
        #[cfg(feature = "serde")]
        {
            serde_json::to_string_pretty(suites).unwrap_or_else(|e| format!("JSON error: {}", e))
        }
        #[cfg(not(feature = "serde"))]
        {
            "JSON output requires 'serde' feature".to_string()
        }
    }

    fn format_text(suites: &[InstructionTestSuite]) -> String {
        let mut output = String::new();

        for suite in suites {
            output.push_str(&format!("=== {} ===\n", suite.instruction_name));
            output.push_str(&format!("Encodings: {}\n", suite.analysis.encodings.len()));
            output.push_str(&format!("Encoding tests: {}\n", suite.encoding_tests.len()));
            output.push_str(&format!(
                "Execution tests: {}\n\n",
                suite.execution_tests.len()
            ));

            for test in &suite.encoding_tests {
                output.push_str(&format!("  [E] {}: {}\n", test.id, test.description));
            }

            for test in &suite.execution_tests {
                output.push_str(&format!("  [X] {}: {}\n", test.id, test.description));
            }

            output.push('\n');
        }

        output
    }
}
