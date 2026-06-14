//! Production-grade test generation engine for ARM instruction validation.
//!
//! This module generates comprehensive test cases from ASL instruction specifications
//! for validating emulator implementations. It analyzes:
//!
//! - Instruction encodings and field extraction
//! - Decode-time computations
//! - Execute-time semantics (register/memory/flag effects)
//! - Guard conditions and exception paths
//! - Edge cases and boundary conditions

pub mod analysis;
pub mod generator;
pub mod oracle;
pub mod output;
pub mod qemu;
pub mod sysreg;
pub mod types;

pub use analysis::InstructionAnalyzer;
pub use generator::{EncodingTestGenerator, ExecutionTestGenerator};
pub use output::TestOutputFormatter;
pub use sysreg::{build_sysreg_encodings, generate_all_sysreg_tests, SysRegEncoding};
pub use types::*;

use crate::syntax::{Definition, Instruction};

/// Main test generator engine that orchestrates all test generation.
pub struct TestGenerator {
    /// Analyzer for extracting semantic information from instructions
    analyzer: InstructionAnalyzer,
    /// Generator for encoding/decoding tests
    encoding_gen: EncodingTestGenerator,
    /// Generator for execution tests
    execution_gen: ExecutionTestGenerator,
    /// Configuration options
    config: TestGenConfig,
}

/// Configuration for test generation.
#[derive(Debug, Clone)]
pub struct TestGenConfig {
    /// Maximum number of encoding tests per instruction
    pub max_encoding_tests: usize,
    /// Whether to generate exhaustive field combinations
    pub exhaustive_fields: bool,
    /// Whether to include negative test cases (invalid encodings)
    pub include_negative_tests: bool,
    /// Whether to generate execution trace tests
    pub generate_execution_tests: bool,
    /// Whether to include memory operation tests
    pub include_memory_tests: bool,
    /// Whether to include exception path tests
    pub include_exception_tests: bool,
    /// Whether to include oracle-verified tests with expected values
    pub include_oracle_tests: bool,
    /// Random seed for reproducible test generation
    pub seed: u64,
    /// Target instruction sets to generate tests for
    pub instruction_sets: Vec<crate::syntax::InstructionSet>,
}

impl Default for TestGenConfig {
    fn default() -> Self {
        Self {
            max_encoding_tests: 1000,
            exhaustive_fields: false,
            include_negative_tests: true,
            generate_execution_tests: true,
            include_memory_tests: true,
            include_exception_tests: true,
            include_oracle_tests: true,
            seed: 0xDEADBEEF,
            instruction_sets: vec![
                crate::syntax::InstructionSet::A64,
                crate::syntax::InstructionSet::A32,
                crate::syntax::InstructionSet::T32,
                crate::syntax::InstructionSet::T16,
            ],
        }
    }
}

impl TestGenerator {
    /// Create a new test generator with default configuration.
    pub fn new() -> Self {
        Self::with_config(TestGenConfig::default())
    }

    /// Create a new test generator with custom configuration.
    pub fn with_config(config: TestGenConfig) -> Self {
        Self {
            analyzer: InstructionAnalyzer::new(),
            encoding_gen: EncodingTestGenerator::new(config.seed),
            execution_gen: ExecutionTestGenerator::new(),
            config,
        }
    }

    /// Generate all tests for a single instruction.
    pub fn generate_instruction_tests(&mut self, instr: &Instruction) -> InstructionTestSuite {
        // Step 1: Analyze the instruction to extract all testable properties
        let analysis = self.analyzer.analyze(instr);

        // Step 2: Generate encoding tests
        let encoding_tests = self.encoding_gen.generate(instr, &analysis, &self.config);

        // Step 3: Generate execution tests if enabled
        let execution_tests = if self.config.generate_execution_tests {
            self.execution_gen.generate(instr, &analysis, &self.config)
        } else {
            Vec::new()
        };

        InstructionTestSuite {
            instruction_name: instr.name.to_string(),
            analysis,
            encoding_tests,
            execution_tests,
        }
    }

    /// Generate tests for all instructions in a list of definitions.
    pub fn generate_from_definitions(&mut self, defs: &[Definition]) -> Vec<InstructionTestSuite> {
        let mut suites = Vec::new();

        for def in defs {
            if let Definition::Instruction(instr) = def {
                let suite = self.generate_instruction_tests(instr);
                suites.push(suite);
            }
        }

        suites
    }

    /// Generate tests and return as formatted output.
    pub fn generate_formatted(&mut self, defs: &[Definition], format: OutputFormat) -> String {
        let suites = self.generate_from_definitions(defs);
        TestOutputFormatter::format(&suites, format)
    }
}

impl Default for TestGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_instructions;

    /// Sample ASL instruction for testing
    const SAMPLE_ADD_IMM: &str = r#"
__instruction aarch64_integer_arithmetic_add_sub_immediate
    __encoding ADD_32_addsub_imm
        __instruction_set A64
        __field sf 31 +: 1
        __field op 30 +: 1
        __field S 29 +: 1
        __field sh 22 +: 1
        __field imm12 10 +: 12
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00100010 0xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer datasize = 32;
            boolean setflags = (S == '1');

    __execute
        bits(32) result;
        bits(32) operand1 = X[n];
        bits(32) operand2 = Zeros();
        (result, -) = AddWithCarry(operand1, operand2, '0');
        X[d] = result;
"#;

    #[test]
    fn test_parse_and_generate_tests() {
        let instrs = parse_instructions(SAMPLE_ADD_IMM).expect("should parse sample instruction");
        assert_eq!(instrs.len(), 1, "should parse one instruction");

        let mut generator = TestGenerator::new();
        let suite = generator.generate_instruction_tests(&instrs[0]);

        assert_eq!(
            suite.instruction_name,
            "aarch64_integer_arithmetic_add_sub_immediate"
        );
        assert!(
            !suite.encoding_tests.is_empty(),
            "should generate encoding tests"
        );
    }

    #[test]
    fn test_generate_boundary_tests() {
        let instrs = parse_instructions(SAMPLE_ADD_IMM).expect("should parse sample instruction");

        let config = TestGenConfig {
            max_encoding_tests: 50,
            include_negative_tests: false,
            generate_execution_tests: false,
            ..Default::default()
        };

        let mut generator = TestGenerator::with_config(config);
        let suite = generator.generate_instruction_tests(&instrs[0]);

        // Check that we have boundary tests for fields
        let boundary_tests: Vec<_> = suite
            .encoding_tests
            .iter()
            .filter(|t| matches!(t.test_type, EncodingTestType::Boundary { .. }))
            .collect();

        assert!(!boundary_tests.is_empty(), "should generate boundary tests");
    }

    #[test]
    fn test_output_formatting() {
        let instrs = parse_instructions(SAMPLE_ADD_IMM).expect("should parse sample instruction");

        let config = TestGenConfig {
            max_encoding_tests: 5,
            include_negative_tests: false,
            generate_execution_tests: false,
            ..Default::default()
        };

        let mut generator = TestGenerator::with_config(config);
        let suite = generator.generate_instruction_tests(&instrs[0]);

        // Test Rust output
        let rust_output = TestOutputFormatter::format(&[suite.clone()], OutputFormat::Rust);
        assert!(
            rust_output.contains("#[test]"),
            "should contain test annotations"
        );
        assert!(
            rust_output.contains("create_test_cpu"),
            "should contain helper functions"
        );
        assert!(
            rust_output.contains("Provenance"),
            "should contain provenance comments"
        );

        // Test Text output
        let text_output = TestOutputFormatter::format(&[suite], OutputFormat::Text);
        assert!(
            text_output.contains("aarch64_integer_arithmetic_add_sub_immediate"),
            "should contain instruction name"
        );
    }
}
