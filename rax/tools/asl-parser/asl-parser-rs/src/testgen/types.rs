//! Core types for test generation.
//!
//! This module defines all the data structures used to represent
//! test cases, analysis results, and test suites.
//!
//! The key innovation is the Provenance system which tracks exactly
//! why each test exists and what specification requirement it validates.

use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::syntax::{Expr, InstructionSet, Stmt};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Provenance System - Tracks specification requirements for every test
// ============================================================================

/// Tracks why a test exists and what spec requirement it validates.
/// Every generated test must have clear provenance from the ASL specification.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Provenance {
    /// Source instruction name (e.g., "aarch64_integer_arithmetic_add_sub_immediate")
    pub instruction: String,
    /// Source encoding name (e.g., "ADD_32_addsub_imm")
    pub encoding: String,
    /// The ASL source text that defines this requirement
    pub asl_source: String,
    /// The specific requirement being tested
    pub requirement: Requirement,
    /// Human-readable explanation of why this test exists
    pub explanation: String,
}

impl Provenance {
    /// Create a new provenance record
    pub fn new(
        instruction: impl Into<String>,
        encoding: impl Into<String>,
        asl_source: impl Into<String>,
        requirement: Requirement,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            instruction: instruction.into(),
            encoding: encoding.into(),
            asl_source: asl_source.into(),
            requirement,
            explanation: explanation.into(),
        }
    }

    /// Format as a doc comment for generated tests
    pub fn as_doc_comment(&self) -> String {
        format!(
            "/// Provenance: {}\n/// ASL: `{}`\n/// Requirement: {:?}\n/// {}",
            self.encoding, self.asl_source, self.requirement, self.explanation
        )
    }
}

impl fmt::Display for Provenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}::{}] {} - {}",
            self.instruction, self.encoding, self.asl_source, self.explanation
        )
    }
}

/// The specific requirement from the specification being tested.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Requirement {
    /// Basic encoding test for instructions with no variable fields
    BasicEncoding,

    /// Field must be extracted correctly from encoding
    /// ASL: "field Rd 0 +: 5" → test all boundary values
    FieldExtraction {
        field: String,
        bit_start: u32,
        bit_width: u32,
    },

    /// Boundary value test for a field
    /// Tests min, max, zero, special values
    FieldBoundary {
        field: String,
        value: u64,
        boundary: BoundaryType,
    },

    /// Special value with semantic meaning (e.g., Rd=31 is ZR)
    FieldSpecial {
        field: String,
        value: u64,
        meaning: String,
    },

    /// Guard condition must be enforced
    /// ASL: "if size:Q == '110' then UNDEFINED"
    GuardCondition {
        condition: String,
        triggers_undefined: bool,
    },

    /// Decode computation must produce correct value
    /// ASL: "integer esize = 8 << UInt(size)"
    DecodeComputation { variable: String, formula: String },

    /// Register read must occur from correct source
    /// ASL: "X[n]" in execute block
    RegisterRead {
        reg_type: RegType,
        source_field: String,
    },

    /// Register write must occur to correct destination
    /// ASL: "X[d] = result"
    RegisterWrite {
        reg_type: RegType,
        dest_field: String,
    },

    /// Special register handling (ZR, SP)
    RegisterSpecial { reg: SpecialReg, behavior: String },

    /// Memory access with specific properties
    /// ASL: "Mem[address, datasize DIV 8, acctype]"
    MemoryAccess {
        op: MemOp,
        size_bits: u32,
        addressing: String,
    },

    /// Flag must be computed correctly
    /// ASL: "if setflags then PSTATE.<N,Z,C,V> = nzcv"
    FlagComputation {
        flag: ProcessorFlag,
        scenario: FlagScenario,
    },

    /// Encoding must trigger UNDEFINED
    UndefinedEncoding { condition: String },

    /// Unpredictable bit must have expected value
    /// ASL: "unpredictable_unless 15 == '0'"
    UnpredictableBit { index: u32, expected: bool },

    /// Size/datasize selection
    /// Tests different operand sizes (32-bit vs 64-bit, etc.)
    SizeVariant { size_field: String, size_bits: u32 },

    /// Shift/extend operation
    ShiftOperation {
        shift_type: String,
        amount_field: String,
    },

    /// Conditional execution
    ConditionalExecution { condition_field: String },
}

/// Register type for provenance tracking
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RegType {
    /// 64-bit general purpose (X0-X30)
    Gp64,
    /// 32-bit general purpose (W0-W30)
    Gp32,
    /// SIMD/FP 128-bit (V0-V31)
    Simd128,
    /// SIMD/FP 64-bit (D0-D31)
    Simd64,
    /// SIMD/FP 32-bit (S0-S31)
    Simd32,
    /// SIMD/FP 16-bit (H0-H31)
    Simd16,
    /// SIMD/FP 8-bit (B0-B31)
    Simd8,
    /// SVE scalable vector
    Sve,
    /// SVE predicate
    SvePredicate,
}

/// Special registers with unique behavior
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpecialReg {
    /// Zero register (XZR/WZR) - reads as 0, writes discarded
    Zr,
    /// Stack pointer (SP) - may have alignment requirements
    Sp,
    /// Program counter (PC)
    Pc,
    /// Link register (LR/X30)
    Lr,
}

/// Memory operation type
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MemOp {
    Load,
    Store,
    LoadPair,
    StorePair,
    LoadAcquire,
    StoreRelease,
    LoadExclusive,
    StoreExclusive,
    Prefetch,
    Atomic,
}

/// Flag computation scenario
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FlagScenario {
    /// Result is zero (Z=1)
    ZeroResult,
    /// Result is non-zero (Z=0)
    NonZeroResult,
    /// Result is negative (N=1)
    NegativeResult,
    /// Result is positive (N=0)
    PositiveResult,
    /// Unsigned overflow (C=1 for add)
    UnsignedOverflow,
    /// No unsigned overflow (C=0)
    NoUnsignedOverflow,
    /// Signed overflow (V=1)
    SignedOverflow,
    /// No signed overflow (V=0)
    NoSignedOverflow,
    /// Carry out from shift
    ShiftCarryOut,
    /// No carry from shift
    ShiftNoCarry,
}

// ============================================================================
// Instruction Analysis Types
// ============================================================================

/// Complete analysis of an instruction's testable properties.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InstructionAnalysis {
    /// Name of the instruction
    pub name: String,
    /// Analysis for each encoding
    pub encodings: Vec<EncodingAnalysis>,
    /// Semantic analysis of the execute block
    pub semantics: SemanticAnalysis,
    /// All identified control flow paths through the instruction
    pub control_paths: Vec<ControlPath>,
    /// Exception conditions that can be triggered
    pub exceptions: Vec<ExceptionCondition>,
    /// Dependencies on shared pseudocode functions
    pub dependencies: HashSet<String>,
}

/// Analysis of a single instruction encoding.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EncodingAnalysis {
    /// Encoding name
    pub name: String,
    /// Target instruction set
    pub iset: InstructionSet,
    /// Fixed opcode bits (value, mask)
    pub opcode_pattern: OpcodePattern,
    /// Extracted fields with their properties
    pub fields: Vec<FieldAnalysis>,
    /// Guard condition (if any)
    pub guard: Option<GuardAnalysis>,
    /// Decode-time computations
    pub decode_computations: Vec<DecodeComputation>,
    /// Invalid encoding conditions (for negative tests)
    pub invalid_conditions: Vec<InvalidCondition>,
    /// Unpredictable bit conditions
    pub unpredictable_bits: Vec<UnpredictableBit>,
}

/// Opcode pattern with fixed and variable bits.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpcodePattern {
    /// Bit width (16, 32)
    pub width: u32,
    /// Fixed bits that must match (value)
    pub fixed_value: u64,
    /// Mask indicating which bits are fixed
    pub fixed_mask: u64,
}

impl OpcodePattern {
    /// Create from a mask vector.
    pub fn from_mask(mask: &[crate::syntax::MaskBit]) -> Self {
        let width = mask.len() as u32;
        let mut fixed_value = 0u64;
        let mut fixed_mask = 0u64;

        for (i, bit) in mask.iter().enumerate() {
            let bit_pos = mask.len() - 1 - i; // MSB first in mask
            match bit {
                crate::syntax::MaskBit::Zero => {
                    fixed_mask |= 1 << bit_pos;
                    // fixed_value bit stays 0
                }
                crate::syntax::MaskBit::One => {
                    fixed_mask |= 1 << bit_pos;
                    fixed_value |= 1 << bit_pos;
                }
                crate::syntax::MaskBit::Either => {
                    // Variable bit, not in mask
                }
            }
        }

        Self {
            width,
            fixed_value,
            fixed_mask,
        }
    }

    /// Check if a value matches this pattern.
    pub fn matches(&self, value: u64) -> bool {
        (value & self.fixed_mask) == self.fixed_value
    }

    /// Generate a valid encoding with the given field values.
    pub fn encode(&self, field_values: &[(u32, u32, u64)]) -> u64 {
        let mut value = self.fixed_value;
        for &(start, width, field_val) in field_values {
            let mask = ((1u64 << width) - 1) << start;
            value = (value & !mask) | ((field_val << start) & mask);
        }
        value
    }
}

/// Analysis of an instruction field.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldAnalysis {
    /// Field name
    pub name: String,
    /// Start bit position (LSB)
    pub start: u32,
    /// Field width in bits
    pub width: u32,
    /// Field semantics (what this field represents)
    pub semantics: FieldSemantics,
    /// Valid value constraints
    pub constraints: Vec<FieldConstraint>,
    /// Special values that trigger different behavior
    pub special_values: Vec<SpecialValue>,
}

/// What a field represents semantically.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FieldSemantics {
    /// General purpose register index
    GpRegister {
        is_64bit: bool,
        allows_sp: bool,
        allows_zr: bool,
    },
    /// SIMD/FP register index
    SimdRegister { vector_size: Option<u32> },
    /// Immediate value
    Immediate {
        signed: bool,
        scale: u32,
        offset: i64,
    },
    /// Shift type selector
    ShiftType,
    /// Condition code
    Condition,
    /// System register encoding
    SystemRegister,
    /// Memory addressing mode
    AddressingMode,
    /// Opcode selector (chooses instruction variant)
    OpcodeSelector,
    /// Size field (determines operand width)
    SizeSelector,
    /// Option field (various instruction options)
    Option,
    /// Unknown/generic field
    Unknown,
}

/// Constraint on field values.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FieldConstraint {
    /// Field must equal a specific value
    Equals(u64),
    /// Field must not equal a specific value
    NotEquals(u64),
    /// Field must be in range [min, max]
    Range { min: u64, max: u64 },
    /// Field must be one of these values
    OneOf(Vec<u64>),
    /// Field must not be one of these values
    NotOneOf(Vec<u64>),
    /// Field must satisfy relation with another field
    RelatedTo {
        field: String,
        relation: FieldRelation,
    },
}

/// Relationship between fields.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FieldRelation {
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

/// A special value for a field that triggers specific behavior.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpecialValue {
    /// The value
    pub value: u64,
    /// What it means
    pub meaning: String,
    /// Whether it triggers different behavior worth testing
    pub generates_different_path: bool,
}

/// Analysis of a guard condition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GuardAnalysis {
    /// Original expression (for reference)
    pub original: String,
    /// Simplified constraints
    pub constraints: Vec<GuardConstraint>,
}

/// A constraint from a guard condition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GuardConstraint {
    /// Field equals value
    FieldEquals { field: String, value: u64 },
    /// Field not equals value
    FieldNotEquals { field: String, value: u64 },
    /// Compound AND constraint
    And(Vec<GuardConstraint>),
    /// Compound OR constraint
    Or(Vec<GuardConstraint>),
    /// Negated constraint
    Not(Box<GuardConstraint>),
    /// Complex constraint we couldn't simplify
    Complex(String),
}

/// A computation performed during decode.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DecodeComputation {
    /// Variable being assigned
    pub target: String,
    /// Type of computation
    pub computation: ComputationType,
    /// Source fields/values
    pub sources: Vec<String>,
}

/// Type of decode-time computation.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ComputationType {
    /// Direct field extraction
    FieldExtract,
    /// Zero extension
    ZeroExtend { to_bits: u32 },
    /// Sign extension
    SignExtend { to_bits: u32 },
    /// Left shift
    LeftShift { amount: u32 },
    /// Concatenation of fields
    Concatenate,
    /// Decoding to enum value
    DecodeEnum { enum_name: String },
    /// Arithmetic operation
    Arithmetic { op: String },
    /// Complex computation (ASL expression)
    Complex { description: String },
}

/// Condition that makes an encoding invalid.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InvalidCondition {
    /// Description of what makes it invalid
    pub description: String,
    /// Field constraints that trigger invalidity
    pub constraints: Vec<FieldConstraint>,
    /// What happens (UNDEFINED, UNPREDICTABLE, etc.)
    pub result: InvalidResult,
}

/// What happens with an invalid encoding.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InvalidResult {
    Undefined,
    Unpredictable,
    Unallocated,
    ConstraintViolation,
}

/// An unpredictable bit condition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnpredictableBit {
    /// Bit index
    pub index: u32,
    /// Expected value for predictable behavior
    pub expected: bool,
}

// ============================================================================
// Semantic Analysis Types
// ============================================================================

/// Semantic analysis of instruction execution.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemanticAnalysis {
    /// Register reads
    pub register_reads: Vec<RegisterAccess>,
    /// Register writes
    pub register_writes: Vec<RegisterAccess>,
    /// Memory reads
    pub memory_reads: Vec<MemoryAccess>,
    /// Memory writes
    pub memory_writes: Vec<MemoryAccess>,
    /// Flag effects
    pub flag_effects: Vec<FlagEffect>,
    /// System register accesses
    pub system_accesses: Vec<SystemAccess>,
    /// PC-relative operations
    pub pc_relative_ops: Vec<PcRelativeOp>,
    /// Computed values that should be tested
    pub computed_values: Vec<ComputedValue>,
}

impl Default for SemanticAnalysis {
    fn default() -> Self {
        Self {
            register_reads: Vec::new(),
            register_writes: Vec::new(),
            memory_reads: Vec::new(),
            memory_writes: Vec::new(),
            flag_effects: Vec::new(),
            system_accesses: Vec::new(),
            pc_relative_ops: Vec::new(),
            computed_values: Vec::new(),
        }
    }
}

/// Register access information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterAccess {
    /// Register identifier (field name or explicit register)
    pub register: RegisterId,
    /// Access width in bits
    pub width: u32,
    /// Whether this is conditional
    pub conditional: bool,
    /// Condition for access (if conditional)
    pub condition: Option<String>,
}

/// Register identifier.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RegisterId {
    /// General purpose register from field
    GpFromField(String),
    /// Explicit GP register
    Gp(u8),
    /// Stack pointer
    Sp,
    /// Zero register
    Zr,
    /// Program counter
    Pc,
    /// Link register
    Lr,
    /// SIMD register from field
    SimdFromField(String),
    /// Explicit SIMD register
    Simd(u8),
    /// System register
    System(String),
}

/// Memory access information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryAccess {
    /// Access size in bytes
    pub size: u32,
    /// Whether sign-extending load
    pub sign_extend: bool,
    /// Addressing mode used
    pub addressing: AddressingInfo,
    /// Alignment requirement
    pub alignment: Option<u32>,
    /// Whether access is atomic
    pub atomic: bool,
    /// Memory ordering (for atomics)
    pub ordering: Option<MemoryOrdering>,
}

/// Addressing mode information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AddressingInfo {
    /// Base register only
    Base { reg: String },
    /// Base + immediate offset
    BaseImmediate { reg: String, offset_field: String },
    /// Base + register offset
    BaseRegister { base: String, offset: String },
    /// Base + scaled register
    BaseScaledRegister {
        base: String,
        offset: String,
        scale: u32,
    },
    /// PC-relative
    PcRelative { offset_field: String },
    /// Pre-indexed
    PreIndex { base: String, offset_field: String },
    /// Post-indexed
    PostIndex { base: String, offset_field: String },
    /// Literal (PC-relative with computed offset)
    Literal { offset_field: String },
}

/// Memory ordering for atomic operations.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MemoryOrdering {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}

/// Flag effect information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FlagEffect {
    /// Which flag
    pub flag: ProcessorFlag,
    /// How it's computed
    pub computation: FlagComputation,
    /// Whether conditional
    pub conditional: bool,
}

/// Processor flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProcessorFlag {
    N, // Negative
    Z, // Zero
    C, // Carry
    V, // Overflow
}

/// How a flag is computed.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FlagComputation {
    /// Result of addition (for C, V)
    AddResult,
    /// Result of subtraction (for C, V)
    SubResult,
    /// From MSB of result (for N)
    ResultMsb,
    /// Zero test of result (for Z)
    ZeroTest,
    /// Direct set to value
    Set(bool),
    /// Shifted out bit (for C in shifts)
    ShiftedBit,
    /// From specific bit of result
    FromBit(u32),
    /// Complex computation
    Complex(String),
}

/// System register/state access.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemAccess {
    /// Register name
    pub name: String,
    /// Read or write
    pub is_write: bool,
    /// Whether privileged
    pub privileged: bool,
}

/// PC-relative operation.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PcRelativeOp {
    /// Type of operation
    pub op_type: PcRelativeType,
    /// Offset source
    pub offset_source: String,
}

/// Types of PC-relative operations.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PcRelativeType {
    Branch,
    Adr,
    AdrPage,
    Load,
}

/// A computed value during execution.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComputedValue {
    /// Name/description
    pub name: String,
    /// Computation type
    pub computation: String,
    /// Input values
    pub inputs: Vec<String>,
}

// ============================================================================
// Control Flow Analysis
// ============================================================================

/// A control flow path through the instruction.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ControlPath {
    /// Path identifier
    pub id: usize,
    /// Conditions required for this path
    pub conditions: Vec<PathCondition>,
    /// Effects specific to this path
    pub effects: PathEffects,
    /// Whether this path is exceptional
    pub is_exceptional: bool,
}

/// A condition for taking a control path.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PathCondition {
    /// Variable/field being tested
    pub variable: String,
    /// Condition type
    pub condition: ConditionType,
}

/// Type of condition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConditionType {
    Equals(i128),
    NotEquals(i128),
    LessThan(i128),
    GreaterThan(i128),
    BitSet(u32),
    BitClear(u32),
    InRange { lo: i128, hi: i128 },
    True,
    False,
}

/// Effects of a specific control path.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PathEffects {
    /// Register writes on this path
    pub writes: Vec<String>,
    /// Memory accesses on this path
    pub memory_ops: Vec<String>,
    /// Exceptions raised on this path
    pub exceptions: Vec<String>,
}

/// Exception condition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExceptionCondition {
    /// Exception type
    pub exception_type: ExceptionType,
    /// Condition that triggers it
    pub trigger: String,
    /// Required processor state
    pub required_state: Option<String>,
}

/// Types of exceptions.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExceptionType {
    Undefined,
    Unpredictable,
    DataAbort,
    PrefetchAbort,
    AlignmentFault,
    PermissionFault,
    Supervisor,
    HypervisorCall,
    SecureMonitor,
    BreakpointDebug,
    WatchpointDebug,
    SoftwareStep,
}

// ============================================================================
// Test Case Types
// ============================================================================

/// Complete test suite for an instruction.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InstructionTestSuite {
    /// Instruction name
    pub instruction_name: String,
    /// Full analysis results
    pub analysis: InstructionAnalysis,
    /// Encoding/decoding tests
    pub encoding_tests: Vec<EncodingTest>,
    /// Execution tests
    pub execution_tests: Vec<ExecutionTest>,
}

/// A test case for encoding/decoding.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EncodingTest {
    /// Test identifier
    pub id: String,
    /// Provenance - why this test exists
    pub provenance: Provenance,
    /// Test description
    pub description: String,
    /// Target encoding name
    pub encoding_name: String,
    /// Instruction set
    pub iset: InstructionSet,
    /// The encoded instruction bytes
    pub encoding: u64,
    /// Encoding width in bits
    pub encoding_width: u32,
    /// Field values used to generate this encoding
    pub field_values: HashMap<String, u64>,
    /// Expected field values after decode
    pub expected_fields: HashMap<String, u64>,
    /// Expected decode computations
    pub expected_decode: HashMap<String, i128>,
    /// Test type
    pub test_type: EncodingTestType,
    /// Expected result (pass/fail/undefined/unpredictable)
    pub expected_result: ExpectedResult,
}

/// Type of encoding test.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EncodingTestType {
    /// Normal valid encoding
    Valid,
    /// Boundary value for a field
    Boundary {
        field: String,
        boundary: BoundaryType,
    },
    /// Guard condition test
    Guard,
    /// Invalid encoding (should fail decode)
    Invalid,
    /// Unpredictable encoding
    Unpredictable,
    /// Special value test
    Special { field: String, meaning: String },
}

/// Boundary type for boundary value tests.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BoundaryType {
    Min,
    Max,
    Zero,
    AllOnes,
    PowerOfTwo,
    PowerOfTwoMinusOne,
}

impl BoundaryType {
    /// Short name for use in test IDs
    pub fn short_name(&self) -> &'static str {
        match self {
            BoundaryType::Min => "min",
            BoundaryType::Max => "max",
            BoundaryType::Zero => "zero",
            BoundaryType::AllOnes => "ones",
            BoundaryType::PowerOfTwo => "poweroftwo",
            BoundaryType::PowerOfTwoMinusOne => "poweroftwominusone",
        }
    }
}

/// Expected result of a test.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExpectedResult {
    /// Test should pass
    Pass,
    /// Should trigger UNDEFINED
    Undefined,
    /// Should trigger UNPREDICTABLE
    Unpredictable,
    /// Should fail decode (unallocated)
    Unallocated,
    /// Should trigger specific exception
    Exception(ExceptionType),
}

/// A test case for instruction execution.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecutionTest {
    /// Test identifier
    pub id: String,
    /// Provenance - why this test exists
    pub provenance: Provenance,
    /// Test description
    pub description: String,
    /// Target encoding name
    pub encoding_name: String,
    /// Instruction set
    pub iset: InstructionSet,
    /// The encoded instruction
    pub encoding: u64,
    /// Encoding width
    pub encoding_width: u32,
    /// Field values used to generate this encoding
    pub field_values: HashMap<String, u64>,
    /// Initial processor state
    pub initial_state: ProcessorState,
    /// Expected final processor state
    pub expected_state: ProcessorState,
    /// Expected memory changes
    pub expected_memory: Vec<MemoryChange>,
    /// Assertions to verify (in addition to state comparison)
    pub assertions: Vec<TestAssertion>,
    /// Control path being tested
    pub path_id: Option<usize>,
    /// Test category
    pub category: ExecutionTestCategory,
}

/// An assertion to verify in a test
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TestAssertion {
    /// What is being checked
    pub check: AssertionCheck,
    /// Expected value
    pub expected: AssertionValue,
    /// Message if assertion fails
    pub message: String,
}

/// What to check in an assertion
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AssertionCheck {
    /// Check a GP register (by index)
    GpReg(u8),
    /// Check a GP register (32-bit view)
    GpReg32(u8),
    /// Check a SIMD register
    SimdReg(u8),
    /// Check SP
    Sp,
    /// Check PC
    Pc,
    /// Check NZCV flags
    Nzcv,
    /// Check individual flag
    Flag(ProcessorFlag),
    /// Check memory at address
    Memory { address: u64, size: u32 },
    /// Check that register is unchanged
    Unchanged(u8),
}

/// Expected value for an assertion
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AssertionValue {
    /// Exact 64-bit value
    U64(u64),
    /// Exact 32-bit value (for W registers)
    U32(u32),
    /// Exact 128-bit value (for SIMD)
    U128(u128),
    /// Boolean value (for flags)
    Bool(bool),
    /// Byte slice (for memory)
    Bytes(Vec<u8>),
    /// Zero
    Zero,
    /// Non-zero (any value)
    NonZero,
    /// Unchanged from initial
    Unchanged,
}

/// Category of execution test.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExecutionTestCategory {
    /// Normal execution
    Normal,
    /// Edge case (overflow, boundary, etc.)
    EdgeCase,
    /// Flag computation
    Flags,
    /// Memory operation
    Memory,
    /// Exception path
    Exception,
    /// Conditional execution
    Conditional,
    /// SIMD/vector operation
    Simd,
}

/// Processor state for execution tests.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProcessorState {
    /// General purpose registers (X0-X30)
    pub gp_regs: HashMap<u8, u64>,
    /// SIMD registers (V0-V31, as 128-bit values)
    pub simd_regs: HashMap<u8, u128>,
    /// Stack pointer
    pub sp: Option<u64>,
    /// Program counter
    pub pc: Option<u64>,
    /// NZCV flags
    pub nzcv: Option<u8>,
    /// FPCR
    pub fpcr: Option<u32>,
    /// FPSR
    pub fpsr: Option<u32>,
    /// Current exception level
    pub el: Option<u8>,
    /// Memory state (address -> value)
    pub memory: HashMap<u64, Vec<u8>>,
    /// System registers
    pub system_regs: HashMap<String, u64>,
}

/// A memory change from execution.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryChange {
    /// Address
    pub address: u64,
    /// New value
    pub value: Vec<u8>,
    /// Size in bytes
    pub size: u32,
}

// ============================================================================
// Output Format
// ============================================================================

/// Output format for test cases.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    /// JSON format (for machine consumption)
    Json,
    /// Rust test code
    Rust,
    /// C test code
    C,
    /// Binary format (for direct loading)
    Binary,
    /// Human-readable text
    Text,
}
