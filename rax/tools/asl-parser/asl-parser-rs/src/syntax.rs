//! AST (Abstract Syntax Tree) definitions for ASL.
//!
//! This module contains all the types that represent parsed ASL programs.
//! The structure closely mirrors the original Haskell implementation while
//! using idiomatic Rust patterns.

use smol_str::SmolStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::span::Spanned;

// ============================================================================
// Identifiers
// ============================================================================

/// An identifier string.
pub type Identifier = SmolStr;

/// Architecture qualifier for identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArchQualifier {
    /// AArch32 architecture
    AArch32,
    /// AArch64 architecture
    AArch64,
    /// No specific architecture
    Any,
}

impl Default for ArchQualifier {
    fn default() -> Self {
        Self::Any
    }
}

/// A qualified identifier with optional architecture qualifier.
///
/// Examples: `AArch64.CheckSPAlignment`, `MyFunction`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QualifiedIdentifier {
    /// The architecture qualifier
    pub qualifier: ArchQualifier,
    /// The identifier name
    pub name: Identifier,
}

impl QualifiedIdentifier {
    /// Create a new qualified identifier with no architecture qualifier.
    pub fn new(name: impl Into<Identifier>) -> Self {
        Self {
            qualifier: ArchQualifier::Any,
            name: name.into(),
        }
    }

    /// Create a new qualified identifier with an architecture qualifier.
    pub fn with_qualifier(qualifier: ArchQualifier, name: impl Into<Identifier>) -> Self {
        Self {
            qualifier,
            name: name.into(),
        }
    }
}

impl From<&str> for QualifiedIdentifier {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl std::fmt::Display for QualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.qualifier {
            ArchQualifier::AArch32 => write!(f, "AArch32.{}", self.name),
            ArchQualifier::AArch64 => write!(f, "AArch64.{}", self.name),
            ArchQualifier::Any => write!(f, "{}", self.name),
        }
    }
}

/// A symbol declaration: (identifier, type) pair.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SymbolDecl {
    /// The name of the symbol
    pub name: Identifier,
    /// The type of the symbol
    pub ty: Type,
}

impl SymbolDecl {
    /// Create a new symbol declaration.
    pub fn new(name: impl Into<Identifier>, ty: Type) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }
}

// ============================================================================
// Binary Literals and Masks
// ============================================================================

/// A single bit value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MaskBit {
    /// Bit must be 0
    Zero,
    /// Bit must be 1
    One,
    /// Bit can be either (don't care / wildcard)
    Either,
}

/// A bit vector literal (e.g., `'1010'`).
pub type BitVector = Vec<bool>;

/// A mask literal with wildcards (e.g., `'10xx'`).
pub type Mask = Vec<MaskBit>;

// ============================================================================
// Types
// ============================================================================

/// ASL type representation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    /// Named type reference (e.g., `integer`, `boolean`, `MyType`)
    Ref(QualifiedIdentifier),

    /// Parameterized type (e.g., `bits(N)`, `array(N)`)
    Fun {
        /// The type constructor name
        name: Identifier,
        /// The type argument expression
        arg: Box<Expr>,
    },

    /// Type inference from expression (e.g., `typeof(x)`)
    Of(Box<Expr>),

    /// Register type with bit fields
    Register {
        /// Width in bits
        width: u64,
        /// Named fields
        fields: Vec<RegField>,
    },

    /// Array type
    Array {
        /// Element type
        element: Box<Type>,
        /// Index type
        index: IndexType,
    },

    /// Tuple type (e.g., `(integer, bits(32))`)
    Tuple(Vec<Type>),
}

impl Type {
    /// Create a simple type reference.
    pub fn simple(name: &str) -> Self {
        Self::Ref(QualifiedIdentifier::new(name))
    }

    /// Create a `bits(N)` type.
    pub fn bits(width: Expr) -> Self {
        Self::Fun {
            name: "bits".into(),
            arg: Box::new(width),
        }
    }

    /// Create an integer type.
    pub fn integer() -> Self {
        Self::simple("integer")
    }

    /// Create a boolean type.
    pub fn boolean() -> Self {
        Self::simple("boolean")
    }
}

/// A field in a register type.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegField {
    /// Field name
    pub name: Identifier,
    /// Bit slices this field covers
    pub slices: Vec<Slice>,
}

/// Index type for arrays.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IndexType {
    /// Integer range (e.g., `0..31`)
    Range {
        /// Lower bound
        lo: Box<Expr>,
        /// Upper bound
        hi: Box<Expr>,
    },
    /// Enumeration type
    Enum(Identifier),
}

// ============================================================================
// Expressions
// ============================================================================

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BinOp {
    // Comparison
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `<`
    Lt,
    /// `<=`
    Le,

    // Arithmetic
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `^` (power)
    Pow,

    // Shifts
    /// `<<`
    Shl,
    /// `>>`
    Shr,

    // Logical
    /// `&&`
    And,
    /// `||`
    Or,

    // Bitwise
    /// `AND`
    BitAnd,
    /// `OR`
    BitOr,
    /// `EOR` (exclusive or)
    BitXor,

    // Integer division
    /// `QUOT`
    Quot,
    /// `REM`
    Rem,
    /// `DIV`
    IntDiv,
    /// `MOD`
    Mod,

    // Concatenation
    /// `++` (array/string concatenation)
    Concat,
    /// `:` (bit concatenation)
    BitConcat,

    // Slicing
    /// `+:` (slice offset)
    SliceOffset,
}

impl BinOp {
    /// Get the precedence of this operator (higher = binds tighter).
    ///
    /// Precedence matches the Java ANTLR grammar (ASL.g4 lines 205-210):
    /// 1. ^ (power) - lowest precedence
    /// 2. *, /
    /// 3. +, -
    /// 4. >>, <<, QUOT, REM, DIV, MOD, OR, EOR, AND, ++, : (all same level)
    /// 5. ==, !=, >, >=, <, <=
    /// 6. &&, || - highest precedence (both same level)
    #[rustfmt::skip]
    pub const fn precedence(self) -> u8 {
        match self {
            // Lowest precedence (listed first in Java grammar)
            Self::Pow                            => 1,
            Self::Mul | Self::Div                => 2,
            Self::Add | Self::Sub                => 3,
            // All these operators at same level (Java grammar line 208)
            Self::Shl | Self::Shr |
            Self::Quot | Self::Rem |
            Self::IntDiv | Self::Mod |
            Self::BitOr | Self::BitXor |
            Self::BitAnd |
            Self::Concat | Self::BitConcat       => 4,
            // Comparison operators
            Self::Eq | Self::Ne |
            Self::Lt | Self::Le |
            Self::Gt | Self::Ge                  => 5,
            // Logical operators - highest precedence (both same level)
            Self::Or | Self::And                 => 6,
            // SliceOffset (+:) is not a general binary operator in expressions,
            // only used in slice syntax, but included here for completeness
            Self::SliceOffset                    => 3,  // Same as Add/Sub
        }
    }

    /// Returns true if this operator is right-associative.
    pub const fn is_right_assoc(self) -> bool {
        matches!(self, Self::Pow)
    }
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnOp {
    /// `-` (negation)
    Neg,
    /// `!` or `NOT` (logical not)
    Not,
}

/// A slice/index specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Slice {
    /// Single index (e.g., `x[i]`)
    Single(Box<Expr>),
    /// Range slice (e.g., `x[hi:lo]`)
    Range {
        /// Upper bound
        hi: Box<Expr>,
        /// Lower bound
        lo: Box<Expr>,
    },
    /// Offset slice (e.g., `x[base+:count]`)
    Offset {
        /// Base position
        base: Box<Expr>,
        /// Number of bits
        count: Box<Expr>,
    },
}

/// An element in a set literal or IN expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SetElement {
    /// Single value
    Single(Box<Expr>),
    /// Range of values (e.g., `1..10`)
    Range {
        /// Lower bound
        lo: Box<Expr>,
        /// Upper bound
        hi: Box<Expr>,
    },
}

/// ASL expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expr {
    /// String literal
    LitString(SmolStr),

    /// Integer literal
    LitInt(i128),

    /// Real number literal (mantissa, exponent)
    LitReal {
        /// Integer part
        integer: i128,
        /// Fractional part (as integer, to be divided appropriately)
        fraction: u64,
    },

    /// Binary bit vector literal
    LitBits(BitVector),

    /// Mask literal with wildcards
    LitMask(Mask),

    /// Variable or function reference
    Var(QualifiedIdentifier),

    /// Implementation-defined value
    ImpDef {
        /// Optional description
        desc: Option<SmolStr>,
        /// The type
        ty: Box<Type>,
    },

    /// Bit/array slice (e.g., `x[3:0]`, `x[i+:4]`)
    Slice {
        /// The expression being sliced
        base: Box<Expr>,
        /// The slice specifications
        slices: Vec<Slice>,
    },

    /// Array indexing (e.g., `arr[i]`)
    Index {
        /// The array expression
        base: Box<Expr>,
        /// The index expressions
        indices: Vec<Slice>,
    },

    /// Unary operation
    Unary {
        /// The operator
        op: UnOp,
        /// The operand
        operand: Box<Expr>,
    },

    /// Binary operation
    Binary {
        /// The operator
        op: BinOp,
        /// Left operand
        lhs: Box<Expr>,
        /// Right operand
        rhs: Box<Expr>,
    },

    /// Member access (e.g., `x.field`)
    Member {
        /// The base expression
        base: Box<Expr>,
        /// The field name
        field: Identifier,
    },

    /// Multiple member access (e.g., `x.[field1, field2]`)
    Members {
        /// The base expression
        base: Box<Expr>,
        /// The field names
        fields: Vec<Identifier>,
    },

    /// Bit field access (e.g., `x.<bit1, bit2>`)
    MemberBits {
        /// The base expression
        base: Box<Expr>,
        /// The bit field names
        fields: Vec<Identifier>,
    },

    /// Mask pattern match (e.g., `x IN '10xx'`)
    InMask {
        /// The expression to test
        expr: Box<Expr>,
        /// The mask pattern
        mask: Mask,
    },

    /// Set membership test (e.g., `x IN {1, 2, 3..10}`)
    InSet {
        /// The expression to test
        expr: Box<Expr>,
        /// The set elements
        elements: Vec<SetElement>,
    },

    /// Function or procedure call
    Call {
        /// The callee (function/procedure name)
        name: QualifiedIdentifier,
        /// The arguments
        args: Vec<Expr>,
    },

    /// Unknown value of a type (e.g., `bits(32) UNKNOWN`)
    Unknown(Box<Type>),

    /// Tuple expression (e.g., `(a, b, c)`)
    Tuple(Vec<Expr>),

    /// Discard pattern (`-`), used in l-value contexts like `(x, -) = f()`
    Discard,

    /// Conditional expression (e.g., `if c then a else b`)
    If {
        /// Condition/result pairs
        branches: Vec<(Expr, Expr)>,
        /// Else branch
        else_branch: Box<Expr>,
    },
}

impl Expr {
    /// Create an integer literal expression.
    pub fn int(value: i128) -> Self {
        Self::LitInt(value)
    }

    /// Create a variable reference expression.
    pub fn var(name: &str) -> Self {
        Self::Var(QualifiedIdentifier::new(name))
    }

    /// Create a binary operation expression.
    pub fn binary(op: BinOp, lhs: Expr, rhs: Expr) -> Self {
        Self::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    /// Create a function call expression.
    pub fn call(callee: impl Into<QualifiedIdentifier>, args: Vec<Expr>) -> Self {
        Self::Call {
            name: callee.into(),
            args,
        }
    }
}

// ============================================================================
// L-Value Expressions
// ============================================================================

/// An L-value expression (can appear on the left side of assignment).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LValExpr {
    /// Discard/ignore placeholder (`-`)
    Ignore,

    /// Variable reference
    Var(QualifiedIdentifier),

    /// Member access (e.g., `x.field`)
    Member {
        /// The base l-value
        base: Box<LValExpr>,
        /// The field name
        field: Identifier,
    },

    /// Multiple member access (e.g., `x.[field1, field2]`)
    Members {
        /// The base l-value
        base: Box<LValExpr>,
        /// The field names
        fields: Vec<Identifier>,
    },

    /// Array indexing (e.g., `arr[i]`)
    Index {
        /// The base l-value
        base: Box<LValExpr>,
        /// The indices
        indices: Vec<Slice>,
    },

    /// Bit slice (e.g., `x<3:0>`)
    Slice {
        /// The base l-value
        base: Box<LValExpr>,
        /// The slices
        slices: Vec<Slice>,
    },

    /// Array destructuring (e.g., `[a, b, c]`)
    Array(Vec<LValExpr>),

    /// Tuple destructuring (e.g., `(a, b, c)`)
    Tuple(Vec<LValExpr>),

    /// Bit field access (e.g., `x.<bit1, bit2>`)
    MemberBits {
        /// The base l-value
        base: Box<LValExpr>,
        /// The field names
        fields: Vec<Identifier>,
    },

    /// Concatenation l-value (e.g., `<a, b, c>`)
    Concat(Vec<LValExpr>),
}

// ============================================================================
// Statements
// ============================================================================

/// A case alternative in a case statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CaseAlternative {
    /// The patterns to match (None for `otherwise`)
    pub patterns: Option<Vec<CasePattern>>,
    /// Optional guard expression (after `&&`)
    pub guard: Option<Expr>,
    /// The statements to execute
    pub body: Vec<Stmt>,
}

/// A pattern in a case statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CasePattern {
    /// Integer literal
    Int(i128),
    /// Hexadecimal literal
    Hex(i128),
    /// Binary literal
    Bits(BitVector),
    /// Mask literal
    Mask(Mask),
    /// Identifier (variable binding or constant)
    Ident(Identifier),
    /// Range pattern (lo..hi)
    Range {
        /// Lower bound
        lo: Box<CasePattern>,
        /// Upper bound
        hi: Box<CasePattern>,
    },
    /// Tuple pattern
    Tuple(Vec<CasePattern>),
    /// Negated pattern (-x)
    Neg(Box<CasePattern>),
}

/// A catch alternative in a try-catch statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CatchAlternative {
    /// The guard expression (None for `otherwise`)
    pub guard: Option<Expr>,
    /// The statements to execute
    pub body: Vec<Stmt>,
}

/// ASL statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Stmt {
    /// Variable declaration without initialization (e.g., `bits(32) x, y;`)
    VarsDecl {
        /// The type
        ty: Type,
        /// The variable names
        names: Vec<Identifier>,
    },

    /// Variable declaration with initialization (e.g., `bits(32) x = expr;`)
    VarDeclInit {
        /// The variable name
        name: Identifier,
        /// The type
        ty: Type,
        /// The initial value
        value: Expr,
    },

    /// Constant declaration (e.g., `constant integer N = 32;`)
    ConstDecl {
        /// The constant name
        name: Identifier,
        /// The type
        ty: Type,
        /// The value
        value: Expr,
    },

    /// Assignment (e.g., `x = expr;`)
    Assign {
        /// The target l-value
        target: LValExpr,
        /// The value to assign
        value: Expr,
    },

    /// Procedure call (e.g., `Proc(args);`)
    Call {
        /// The procedure name
        name: QualifiedIdentifier,
        /// The arguments
        args: Vec<Expr>,
    },

    /// Return statement
    Return(Option<Expr>),

    /// Assert statement
    Assert(Expr),

    /// UNPREDICTABLE statement
    Unpredictable,

    /// UNDEFINED statement
    Undefined,

    /// IMPLEMENTATION_DEFINED statement
    ImpDef(SmolStr),

    /// SEE statement
    See(SmolStr),

    /// If-elsif-else statement
    If {
        /// Condition/body pairs for if and elsif branches
        branches: Vec<(Expr, Vec<Stmt>)>,
        /// Optional else body
        else_body: Option<Vec<Stmt>>,
    },

    /// Case statement
    Case {
        /// The expression to match
        expr: Expr,
        /// The alternatives
        alts: Vec<CaseAlternative>,
    },

    /// For loop
    For {
        /// Loop variable
        var: Identifier,
        /// Start value
        start: Expr,
        /// Direction (true = to, false = downto)
        ascending: bool,
        /// End value
        end: Expr,
        /// Loop body
        body: Vec<Stmt>,
    },

    /// Field definition
    Field {
        /// Field name
        name: Identifier,
        /// Field slice expression
        expr: Expr,
    },

    /// While loop
    While {
        /// Loop condition
        cond: Expr,
        /// Loop body
        body: Vec<Stmt>,
    },

    /// Repeat-until loop
    Repeat {
        /// Loop body
        body: Vec<Stmt>,
        /// Exit condition
        cond: Expr,
    },

    /// Throw statement
    Throw(Identifier),

    /// Try-catch statement
    Try {
        /// The try body
        body: Vec<Stmt>,
        /// The exception variable
        exc_var: Identifier,
        /// The catch alternatives
        catches: Vec<CatchAlternative>,
    },

    /// Local enumeration definition
    DefEnum {
        /// The enum name
        name: Identifier,
        /// The enum variants
        variants: Vec<Identifier>,
    },
}

// ============================================================================
// Definitions
// ============================================================================

/// A setter argument (with optional reference marker).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SetterArg {
    /// The symbol declaration
    pub decl: SymbolDecl,
    /// Whether this is a reference argument
    pub is_ref: bool,
}

/// Top-level ASL definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Definition {
    /// Builtin type declaration (e.g., `__builtin type bits;`)
    TypeBuiltin(Identifier),

    /// Abstract type declaration (e.g., `type MyType;`)
    TypeAbstract(Identifier),

    /// Type alias (e.g., `type Alias = ExistingType;`)
    TypeAlias {
        /// The alias name
        name: Identifier,
        /// The aliased type
        ty: Type,
    },

    /// Struct type definition
    TypeStruct {
        /// The struct name
        name: QualifiedIdentifier,
        /// The fields
        fields: Vec<SymbolDecl>,
    },

    /// Enumeration type definition
    TypeEnum {
        /// The enum name
        name: Identifier,
        /// The variants
        variants: Vec<Identifier>,
    },

    /// Global variable declaration
    Variable {
        /// The variable name
        name: QualifiedIdentifier,
        /// The type
        ty: Type,
    },

    /// Global constant definition
    Const {
        /// The constant name
        name: Identifier,
        /// The type
        ty: Type,
        /// The value
        value: Expr,
    },

    /// Array declaration
    Array {
        /// The array name
        name: Identifier,
        /// Element type
        ty: Type,
        /// Index type
        index: IndexType,
    },

    /// Function or procedure definition
    Callable {
        /// The name
        name: QualifiedIdentifier,
        /// Parameters
        params: Vec<SymbolDecl>,
        /// Return types (empty for procedures)
        returns: Vec<Type>,
        /// Body statements
        body: Vec<Stmt>,
    },

    /// Getter definition (e.g., `bits(64) SP ...`)
    Getter {
        /// The name
        name: QualifiedIdentifier,
        /// Index parameters (for indexed getters)
        params: Option<Vec<SymbolDecl>>,
        /// Return types
        returns: Vec<Type>,
        /// Body statements
        body: Vec<Stmt>,
    },

    /// Setter definition (e.g., `SP = bits(64) value ...`)
    Setter {
        /// The name
        name: QualifiedIdentifier,
        /// Index parameters (for indexed setters)
        params: Option<Vec<SetterArg>>,
        /// The value parameter
        value: SymbolDecl,
        /// Body statements
        body: Vec<Stmt>,
    },

    /// Instruction definition
    Instruction(Instruction),

    /// Decode definition
    Decode {
        /// Target instruction set
        iset: InstructionSet,
        /// Cases
        cases: Vec<DecodeCase>,
    },
}

// ============================================================================
// Instructions
// ============================================================================

/// Instruction set architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InstructionSet {
    /// AArch32 ARM mode
    A32,
    /// AArch32 Thumb mode (32-bit)
    T32,
    /// AArch32 Thumb mode (16-bit)
    T16,
    /// AArch64
    A64,
}

impl std::fmt::Display for InstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A32 => write!(f, "A32"),
            Self::T32 => write!(f, "T32"),
            Self::T16 => write!(f, "T16"),
            Self::A64 => write!(f, "A64"),
        }
    }
}

/// A field in an instruction encoding.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InstructionField {
    /// Field name
    pub name: Identifier,
    /// Start bit position
    pub begin: u32,
    /// Field width in bits
    pub width: u32,
}

/// An unpredictable condition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnpredictableUnless {
    /// Bit index
    pub index: u32,
    /// Expected value
    pub value: bool,
}

/// An instruction encoding.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InstructionEncoding {
    /// Encoding name
    pub name: Identifier,
    /// Target instruction set
    pub iset: InstructionSet,
    /// Instruction fields
    pub fields: Vec<InstructionField>,
    /// Opcode mask
    pub opcode: Mask,
    /// Guard condition
    pub guard: Option<Expr>,
    /// Unpredictable conditions
    pub unpredictable: Vec<UnpredictableUnless>,
    /// Decode statements
    pub decode: Vec<Stmt>,
}

/// An instruction definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Instruction {
    /// Instruction name
    pub name: Identifier,
    /// Encodings
    pub encodings: Vec<InstructionEncoding>,
    /// Post-decode statements
    pub post_decode: Vec<Stmt>,
    /// Execute statements
    pub execute: Vec<Stmt>,
    /// Whether this is a conditional instruction
    pub conditional: bool,
}

/// A decode field specification - either a numeric slice, field reference, or concatenation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DecodeSlice {
    /// Numeric slice: `start +: width` (e.g., `29 +: 3`)
    Slice {
        /// Start bit position
        start: u32,
        /// Width in bits
        width: u32,
    },
    /// Field reference (e.g., `op` referring to a previously extracted field)
    Field(Identifier),
    /// Concatenation of fields (e.g., `D:Rd`)
    Concat(Vec<DecodeSlice>),
}

/// A decode field in a decode case (kept for backwards compat).
pub type DecodeField = DecodeSlice;

/// A pattern in a when clause.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DecodePattern {
    /// Wildcard pattern _
    Wildcard,
    /// Literal pattern (binary/mask literal)
    Lit(String),
    /// Negated pattern (e.g., `!'000'`)
    Not(Box<DecodePattern>),
}

/// Legacy pattern alias
pub type Pattern = DecodePattern;

/// A decode statement (inside decode blocks).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DecodeStmt {
    /// Nested case statement: `case (fields) of branches`
    Case {
        /// The field slices to match on
        fields: Vec<DecodeSlice>,
        /// The when/otherwise branches
        branches: Vec<DecodeBranch>,
    },
    /// Field extraction: `__field name start +: width`
    Field {
        /// Field name
        name: Identifier,
        /// Start bit position
        start: u32,
        /// Width in bits
        width: u32,
    },
    /// Encoding reference: `__encoding name`
    Encoding(Identifier),
    /// Unallocated marker
    Unallocated,
    /// Unpredictable marker
    Unpredictable,
    /// No operation (placeholder)
    Nop,
}

/// A decode branch (when or otherwise).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DecodeBranch {
    /// The patterns to match (None for `otherwise`)
    pub patterns: Option<Vec<DecodePattern>>,
    /// The body statements
    pub body: Vec<DecodeStmt>,
}

/// A when branch in a decode case (legacy).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WhenBranch {
    /// The pattern tuple
    pub pattern: Vec<DecodePattern>,
    /// The body statements (as decode statements)
    pub body: Vec<DecodeStmt>,
}

/// A decode case.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DecodeCase {
    /// The pattern fields
    pub fields: Vec<DecodeSlice>,
    /// The when branches
    pub branches: Vec<DecodeBranch>,
}

// ============================================================================
// Registers
// ============================================================================

/// A register field.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterField {
    /// Field name (None for unnamed/reserved fields)
    pub name: Option<Identifier>,
    /// Low bit position
    pub lo: u32,
    /// High bit position
    pub hi: u32,
}

/// A register definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Register {
    /// Register name
    pub name: Identifier,
    /// Width in bits
    pub width: u32,
    /// Named fields
    pub fields: Vec<RegisterField>,
}

/// A register array definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterArray {
    /// Minimum index
    pub index_min: i64,
    /// Maximum index
    pub index_max: i64,
    /// Register definition
    pub register: Register,
}

/// Top-level register definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RegisterDefinition {
    /// Single register
    Single(Register),
    /// Array of registers
    Array(RegisterArray),
}

// ============================================================================
// Spanned Aliases
// ============================================================================

/// An expression with source location.
pub type SpannedExpr = Spanned<Expr>;

/// A statement with source location.
pub type SpannedStmt = Spanned<Stmt>;

/// A definition with source location.
pub type SpannedDefinition = Spanned<Definition>;

/// An instruction with source location.
pub type SpannedInstruction = Spanned<Instruction>;

/// A register definition with source location.
pub type SpannedRegisterDefinition = Spanned<RegisterDefinition>;
