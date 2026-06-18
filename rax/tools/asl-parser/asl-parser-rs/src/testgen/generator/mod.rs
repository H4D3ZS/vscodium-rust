//! Test case generation.
//!
//! This module generates test cases from analyzed instructions:
//! - Encoding tests (field extraction, boundaries, invalid)
//! - Execution tests (register/memory/flag effects)

pub mod boundary;
pub mod encoding;
pub mod execution;

use crate::syntax::Instruction;
use crate::testgen::types::*;
use crate::testgen::TestGenConfig;

/// Generates encoding test cases
pub struct EncodingTestGenerator {
    seed: u64,
}

impl EncodingTestGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate all encoding tests for an instruction
    pub fn generate(
        &self,
        instr: &Instruction,
        analysis: &InstructionAnalysis,
        config: &TestGenConfig,
    ) -> Vec<EncodingTest> {
        let mut tests = Vec::new();

        for enc_analysis in &analysis.encodings {
            // Skip if not in target instruction sets
            if !config.instruction_sets.contains(&enc_analysis.iset) {
                continue;
            }

            // Generate field boundary tests
            tests.extend(encoding::generate_field_boundary_tests(instr, enc_analysis));

            // Generate field combination tests
            tests.extend(encoding::generate_field_combination_tests(
                instr,
                enc_analysis,
                config,
            ));

            // Generate special value tests
            tests.extend(encoding::generate_special_value_tests(instr, enc_analysis));

            // Generate guard condition tests
            if enc_analysis.guard.is_some() {
                tests.extend(encoding::generate_guard_tests(instr, enc_analysis));
            }

            // Generate invalid encoding tests (UNDEFINED)
            if config.include_negative_tests {
                tests.extend(encoding::generate_invalid_tests(instr, enc_analysis));
            }

            // Generate unpredictable bit tests
            if config.include_negative_tests && !enc_analysis.unpredictable_bits.is_empty() {
                tests.extend(encoding::generate_unpredictable_tests(instr, enc_analysis));
            }
        }

        tests
    }
}

/// Generates execution test cases
pub struct ExecutionTestGenerator {
    // Configuration for execution test generation
}

impl ExecutionTestGenerator {
    pub fn new() -> Self {
        Self {}
    }

    /// Generate all execution tests for an instruction
    pub fn generate(
        &self,
        instr: &Instruction,
        analysis: &InstructionAnalysis,
        config: &TestGenConfig,
    ) -> Vec<ExecutionTest> {
        let mut tests = Vec::new();

        for enc_analysis in &analysis.encodings {
            // Skip if not in target instruction sets
            if !config.instruction_sets.contains(&enc_analysis.iset) {
                continue;
            }

            // Generate oracle-verified tests (with expected values)
            if config.include_oracle_tests {
                tests.extend(execution::generate_oracle_tests(
                    instr,
                    enc_analysis,
                    &analysis.semantics,
                ));
            }

            // Generate register operation tests
            tests.extend(execution::generate_register_tests(
                instr,
                enc_analysis,
                &analysis.semantics,
            ));

            // Generate flag computation tests
            if !analysis.semantics.flag_effects.is_empty() {
                tests.extend(execution::generate_flag_tests(
                    instr,
                    enc_analysis,
                    &analysis.semantics,
                ));
            }

            // Generate memory operation tests
            if config.include_memory_tests {
                if !analysis.semantics.memory_reads.is_empty()
                    || !analysis.semantics.memory_writes.is_empty()
                {
                    tests.extend(execution::generate_memory_tests(
                        instr,
                        enc_analysis,
                        &analysis.semantics,
                    ));
                }
            }

            // Generate exception path tests
            if config.include_exception_tests && !analysis.exceptions.is_empty() {
                tests.extend(execution::generate_exception_tests(
                    instr,
                    enc_analysis,
                    &analysis.exceptions,
                ));
            }
        }

        tests
    }
}

impl Default for ExecutionTestGenerator {
    fn default() -> Self {
        Self::new()
    }
}
