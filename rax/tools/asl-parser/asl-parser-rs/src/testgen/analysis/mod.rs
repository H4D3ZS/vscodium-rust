//! Instruction analysis for test generation.
//!
//! This module extracts all testable properties from ASL instruction definitions:
//! - Encoding patterns and fields
//! - Decode-time computations
//! - Execute-time semantics (register/memory/flag effects)
//! - Guard conditions and exception paths

pub mod decode;
pub mod encoding;
pub mod semantic;

use std::collections::HashSet;

use crate::syntax::{Expr, Instruction, InstructionEncoding, Stmt};
use crate::testgen::types::*;

/// Analyzes ASL instructions to extract all testable properties.
pub struct InstructionAnalyzer {
    /// Known register field names (Rd, Rn, Rm, Rt, etc.)
    register_fields: HashSet<&'static str>,
    /// Known immediate field patterns
    immediate_patterns: HashSet<&'static str>,
    /// Known size field names
    size_fields: HashSet<&'static str>,
}

impl InstructionAnalyzer {
    /// Create a new instruction analyzer
    pub fn new() -> Self {
        let mut register_fields = HashSet::new();
        // Common register field names in ARM ASL
        for name in &[
            "Rd", "Rn", "Rm", "Rt", "Rt2", "Rs", "Ra", // GP registers
            "Vd", "Vn", "Vm", "Vt", "Vt2", // SIMD registers
            "Zd", "Zn", "Zm", "Zt", // SVE registers
            "Pd", "Pn", "Pm", "Pg", // Predicate registers
            "Xd", "Xn", "Xm", "Xt", // Explicit 64-bit
            "Wd", "Wn", "Wm", "Wt", // Explicit 32-bit
        ] {
            register_fields.insert(*name);
        }

        let mut immediate_patterns = HashSet::new();
        for name in &[
            "imm", "imm3", "imm4", "imm5", "imm6", "imm7", "imm8", "imm9", "imm12", "imm13",
            "imm14", "imm16", "imm19", "imm26", "immr", "imms", "immlo", "immhi", "uimm", "simm",
        ] {
            immediate_patterns.insert(*name);
        }

        let mut size_fields = HashSet::new();
        for name in &["size", "sz", "sf", "Q", "opc", "S"] {
            size_fields.insert(*name);
        }

        Self {
            register_fields,
            immediate_patterns,
            size_fields,
        }
    }

    /// Analyze an instruction and extract all testable properties
    pub fn analyze(&self, instr: &Instruction) -> InstructionAnalysis {
        let mut encodings = Vec::new();
        let mut all_dependencies = HashSet::new();

        // Analyze each encoding
        for enc in &instr.encodings {
            let enc_analysis = self.analyze_encoding(enc, &instr.post_decode);
            encodings.push(enc_analysis);
        }

        // Analyze execute block semantics
        let semantics = self.analyze_execute(&instr.execute);

        // Extract control flow paths
        let control_paths = self.extract_control_paths(&instr.execute);

        // Extract exception conditions
        let exceptions = self.extract_exceptions(&instr.execute);

        // Collect dependencies
        self.collect_dependencies(&instr.execute, &mut all_dependencies);

        InstructionAnalysis {
            name: instr.name.to_string(),
            encodings,
            semantics,
            control_paths,
            exceptions,
            dependencies: all_dependencies,
        }
    }

    /// Analyze a single encoding
    fn analyze_encoding(
        &self,
        enc: &InstructionEncoding,
        post_decode: &[Stmt],
    ) -> EncodingAnalysis {
        // Extract opcode pattern
        let opcode_pattern = OpcodePattern::from_mask(&enc.opcode);

        // Analyze fields
        let fields: Vec<FieldAnalysis> = enc
            .fields
            .iter()
            .map(|f| self.analyze_field(f, &enc.decode))
            .collect();

        // Analyze guard condition
        let guard = enc.guard.as_ref().map(|g| self.analyze_guard(g));

        // Extract decode computations
        let decode_computations = self.extract_decode_computations(&enc.decode, post_decode);

        // Extract invalid conditions (UNDEFINED in decode)
        let invalid_conditions = self.extract_invalid_conditions(&enc.decode);

        // Map unpredictable bits
        let unpredictable_bits: Vec<UnpredictableBit> = enc
            .unpredictable
            .iter()
            .map(|u| UnpredictableBit {
                index: u.index,
                expected: u.value,
            })
            .collect();

        EncodingAnalysis {
            name: enc.name.to_string(),
            iset: enc.iset,
            opcode_pattern,
            fields,
            guard,
            decode_computations,
            invalid_conditions,
            unpredictable_bits,
        }
    }

    /// Analyze a single field
    fn analyze_field(
        &self,
        field: &crate::syntax::InstructionField,
        decode_stmts: &[Stmt],
    ) -> FieldAnalysis {
        let name = field.name.to_string();
        let start = field.begin;
        let width = field.width;

        // Determine field semantics based on name and usage
        let semantics = self.infer_field_semantics(&name, width, decode_stmts);

        // Determine constraints from decode block
        let constraints = self.extract_field_constraints(&name, decode_stmts);

        // Identify special values
        let special_values = self.identify_special_values(&name, &semantics, width);

        FieldAnalysis {
            name,
            start,
            width,
            semantics,
            constraints,
            special_values,
        }
    }

    /// Infer what a field represents based on its name and usage
    fn infer_field_semantics(
        &self,
        name: &str,
        width: u32,
        _decode_stmts: &[Stmt],
    ) -> FieldSemantics {
        // Check for register fields
        if self.register_fields.contains(name) {
            // Determine if it's GP, SIMD, etc. based on prefix
            if name.starts_with('V') || name.starts_with('Z') {
                return FieldSemantics::SimdRegister { vector_size: None };
            }
            // Default to GP register - width 5 is typical for A64 registers
            return FieldSemantics::GpRegister {
                is_64bit: true,                          // Will be refined by sf field
                allows_sp: name == "Rn" || name == "Xn", // Rn often allows SP
                allows_zr: name == "Rd" || name == "Rt", // Rd often uses ZR
            };
        }

        // Check for immediate patterns
        if name.starts_with("imm") || self.immediate_patterns.contains(name) {
            return FieldSemantics::Immediate {
                signed: name.starts_with("simm"),
                scale: 0,
                offset: 0,
            };
        }

        // Check for size fields
        if self.size_fields.contains(name) {
            return FieldSemantics::SizeSelector;
        }

        // Check for condition fields
        if name == "cond" {
            return FieldSemantics::Condition;
        }

        // Check for shift fields
        if name == "shift" || name == "sh" {
            return FieldSemantics::ShiftType;
        }

        // Check for option fields
        if name == "option" || name == "opt" {
            return FieldSemantics::Option;
        }

        // Default to unknown
        FieldSemantics::Unknown
    }

    /// Extract constraints on a field from decode statements
    fn extract_field_constraints(&self, field_name: &str, stmts: &[Stmt]) -> Vec<FieldConstraint> {
        let mut constraints = Vec::new();

        for stmt in stmts {
            match stmt {
                Stmt::If { branches, .. } => {
                    // Look for patterns like "if Rd == '11111' then UNDEFINED"
                    for (cond, body) in branches {
                        if self.is_undefined_body(body) {
                            if let Some(constraint) =
                                self.extract_constraint_from_condition(field_name, cond)
                            {
                                // This is an invalid constraint, invert it for valid values
                                constraints.push(constraint);
                            }
                        }
                    }
                }
                Stmt::Case { expr, alts } => {
                    // Look for case statements that constrain field values
                    // TODO: Extract constraints from case patterns
                }
                _ => {}
            }
        }

        constraints
    }

    /// Check if a statement body is UNDEFINED
    fn is_undefined_body(&self, body: &[Stmt]) -> bool {
        body.iter()
            .any(|s| matches!(s, Stmt::Undefined | Stmt::Unpredictable))
    }

    /// Extract a constraint from a condition expression
    fn extract_constraint_from_condition(
        &self,
        _field_name: &str,
        _cond: &Expr,
    ) -> Option<FieldConstraint> {
        // TODO: Implement expression analysis to extract constraints
        // For now, return None
        None
    }

    /// Identify special values for a field
    fn identify_special_values(
        &self,
        name: &str,
        semantics: &FieldSemantics,
        width: u32,
    ) -> Vec<SpecialValue> {
        let mut special = Vec::new();
        let max_val = (1u64 << width) - 1;

        match semantics {
            FieldSemantics::GpRegister {
                allows_sp,
                allows_zr,
                ..
            } => {
                // Register 31 is special in A64
                if width == 5 {
                    if *allows_zr {
                        special.push(SpecialValue {
                            value: 31,
                            meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded"
                                .to_string(),
                            generates_different_path: true,
                        });
                    }
                    if *allows_sp {
                        special.push(SpecialValue {
                            value: 31,
                            meaning: "Stack pointer (SP) - may require alignment".to_string(),
                            generates_different_path: true,
                        });
                    }
                }
            }
            FieldSemantics::SizeSelector => {
                // Size fields typically have all values meaningful
                for v in 0..=max_val {
                    special.push(SpecialValue {
                        value: v,
                        meaning: format!("Size variant {}", v),
                        generates_different_path: true,
                    });
                }
            }
            FieldSemantics::Condition => {
                // All 16 condition codes are meaningful
                let cond_names = [
                    "EQ", "NE", "CS/HS", "CC/LO", "MI", "PL", "VS", "VC", "HI", "LS", "GE", "LT",
                    "GT", "LE", "AL", "NV",
                ];
                for (v, name) in cond_names.iter().enumerate() {
                    special.push(SpecialValue {
                        value: v as u64,
                        meaning: format!("Condition {}", name),
                        generates_different_path: true,
                    });
                }
            }
            FieldSemantics::ShiftType => {
                let shift_names = ["LSL", "LSR", "ASR", "ROR"];
                for (v, name) in shift_names.iter().enumerate() {
                    special.push(SpecialValue {
                        value: v as u64,
                        meaning: format!("Shift type {}", name),
                        generates_different_path: true,
                    });
                }
            }
            _ => {}
        }

        special
    }

    /// Analyze a guard condition
    fn analyze_guard(&self, guard: &Expr) -> GuardAnalysis {
        let original = format!("{:?}", guard); // TODO: Better expression formatting
        let constraints = self.extract_guard_constraints(guard);

        GuardAnalysis {
            original,
            constraints,
        }
    }

    /// Extract constraints from a guard expression
    fn extract_guard_constraints(&self, expr: &Expr) -> Vec<GuardConstraint> {
        match expr {
            Expr::Var(qid) if qid.name.as_str() == "TRUE" => {
                vec![] // TRUE guard means no constraints
            }
            Expr::Binary { op, lhs, rhs } => {
                use crate::syntax::BinOp;
                match op {
                    BinOp::And => {
                        // Recurse into both sides
                        let mut constraints = self.extract_guard_constraints(lhs);
                        constraints.extend(self.extract_guard_constraints(rhs));
                        constraints
                    }
                    BinOp::Or => {
                        let left = self.extract_guard_constraints(lhs);
                        let right = self.extract_guard_constraints(rhs);
                        if !left.is_empty() || !right.is_empty() {
                            vec![GuardConstraint::Or(
                                left.into_iter().chain(right.into_iter()).collect(),
                            )]
                        } else {
                            vec![]
                        }
                    }
                    BinOp::Eq | BinOp::Ne => {
                        // Try to extract field equality/inequality
                        if let Some(constraint) = self.extract_comparison_constraint(expr) {
                            vec![constraint]
                        } else {
                            vec![GuardConstraint::Complex(format!("{:?}", expr))]
                        }
                    }
                    _ => vec![GuardConstraint::Complex(format!("{:?}", expr))],
                }
            }
            Expr::Unary { op, operand } => {
                use crate::syntax::UnOp;
                match op {
                    UnOp::Not => {
                        let inner = self.extract_guard_constraints(operand);
                        if inner.len() == 1 {
                            vec![GuardConstraint::Not(Box::new(
                                inner.into_iter().next().unwrap(),
                            ))]
                        } else {
                            vec![GuardConstraint::Complex(format!("{:?}", expr))]
                        }
                    }
                    _ => vec![GuardConstraint::Complex(format!("{:?}", expr))],
                }
            }
            _ => vec![GuardConstraint::Complex(format!("{:?}", expr))],
        }
    }

    /// Try to extract a field comparison constraint
    fn extract_comparison_constraint(&self, _expr: &Expr) -> Option<GuardConstraint> {
        // TODO: Implement proper expression analysis
        None
    }

    /// Extract decode-time computations from decode statements
    fn extract_decode_computations(
        &self,
        decode_stmts: &[Stmt],
        post_decode_stmts: &[Stmt],
    ) -> Vec<DecodeComputation> {
        let mut computations = Vec::new();

        // Process decode statements
        for stmt in decode_stmts.iter().chain(post_decode_stmts.iter()) {
            if let Some(comp) = self.extract_computation_from_stmt(stmt) {
                computations.push(comp);
            }
        }

        computations
    }

    /// Extract a computation from a statement
    fn extract_computation_from_stmt(&self, stmt: &Stmt) -> Option<DecodeComputation> {
        match stmt {
            Stmt::VarDeclInit { name, value, .. } => {
                let (computation, sources) = self.analyze_computation_expr(value);
                Some(DecodeComputation {
                    target: name.to_string(),
                    computation,
                    sources,
                })
            }
            Stmt::ConstDecl { name, value, .. } => {
                let (computation, sources) = self.analyze_computation_expr(value);
                Some(DecodeComputation {
                    target: name.to_string(),
                    computation,
                    sources,
                })
            }
            Stmt::Assign { target, value } => {
                if let crate::syntax::LValExpr::Var(qid) = target {
                    let (computation, sources) = self.analyze_computation_expr(value);
                    Some(DecodeComputation {
                        target: qid.name.to_string(),
                        computation,
                        sources,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Analyze a computation expression
    fn analyze_computation_expr(&self, expr: &Expr) -> (ComputationType, Vec<String>) {
        match expr {
            Expr::Call { name, args } => {
                let func_name = name.name.as_str();
                let sources: Vec<String> = args
                    .iter()
                    .filter_map(|a| {
                        if let Expr::Var(qid) = a {
                            Some(qid.name.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                let computation = match func_name {
                    "UInt" => ComputationType::FieldExtract,
                    "SInt" => ComputationType::FieldExtract,
                    "ZeroExtend" => {
                        let to_bits = args
                            .get(1)
                            .and_then(|e| {
                                if let Expr::LitInt(n) = e {
                                    Some(*n as u32)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(64);
                        ComputationType::ZeroExtend { to_bits }
                    }
                    "SignExtend" => {
                        let to_bits = args
                            .get(1)
                            .and_then(|e| {
                                if let Expr::LitInt(n) = e {
                                    Some(*n as u32)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(64);
                        ComputationType::SignExtend { to_bits }
                    }
                    "LSL" => {
                        let amount = args
                            .get(1)
                            .and_then(|e| {
                                if let Expr::LitInt(n) = e {
                                    Some(*n as u32)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(0);
                        ComputationType::LeftShift { amount }
                    }
                    "Replicate" | "Ones" | "Zeros" => ComputationType::Complex {
                        description: format!("{}(...)", func_name),
                    },
                    _ => ComputationType::Complex {
                        description: format!("{}(...)", func_name),
                    },
                };

                (computation, sources)
            }
            Expr::Binary { op, lhs, rhs } => {
                use crate::syntax::BinOp;
                let mut sources = Vec::new();

                if let Expr::Var(qid) = lhs.as_ref() {
                    sources.push(qid.name.to_string());
                }
                if let Expr::Var(qid) = rhs.as_ref() {
                    sources.push(qid.name.to_string());
                }

                let computation = match op {
                    BinOp::BitConcat => ComputationType::Concatenate,
                    BinOp::Shl => {
                        let amount = if let Expr::LitInt(n) = rhs.as_ref() {
                            *n as u32
                        } else {
                            0
                        };
                        ComputationType::LeftShift { amount }
                    }
                    _ => ComputationType::Arithmetic {
                        op: format!("{:?}", op),
                    },
                };

                (computation, sources)
            }
            Expr::Var(qid) => (ComputationType::FieldExtract, vec![qid.name.to_string()]),
            Expr::LitInt(_) => (
                ComputationType::Complex {
                    description: "literal".to_string(),
                },
                vec![],
            ),
            _ => (
                ComputationType::Complex {
                    description: format!("{:?}", expr),
                },
                vec![],
            ),
        }
    }

    /// Extract conditions that make an encoding invalid
    fn extract_invalid_conditions(&self, stmts: &[Stmt]) -> Vec<InvalidCondition> {
        let mut conditions = Vec::new();

        for stmt in stmts {
            self.extract_invalid_from_stmt(stmt, &mut conditions);
        }

        conditions
    }

    /// Extract invalid conditions from a single statement
    fn extract_invalid_from_stmt(&self, stmt: &Stmt, conditions: &mut Vec<InvalidCondition>) {
        match stmt {
            Stmt::If {
                branches,
                else_body,
            } => {
                for (cond, body) in branches {
                    // Check if this branch leads to UNDEFINED
                    let result = if body.iter().any(|s| matches!(s, Stmt::Undefined)) {
                        Some(InvalidResult::Undefined)
                    } else if body.iter().any(|s| matches!(s, Stmt::Unpredictable)) {
                        Some(InvalidResult::Unpredictable)
                    } else {
                        None
                    };

                    if let Some(result) = result {
                        conditions.push(InvalidCondition {
                            description: format!("{:?}", cond),
                            constraints: vec![], // TODO: Parse condition into constraints
                            result,
                        });
                    }

                    // Recurse into body
                    for s in body {
                        self.extract_invalid_from_stmt(s, conditions);
                    }
                }

                // Check else body
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.extract_invalid_from_stmt(s, conditions);
                    }
                }
            }
            Stmt::Undefined => {
                conditions.push(InvalidCondition {
                    description: "Unconditional UNDEFINED".to_string(),
                    constraints: vec![],
                    result: InvalidResult::Undefined,
                });
            }
            Stmt::Unpredictable => {
                conditions.push(InvalidCondition {
                    description: "Unconditional UNPREDICTABLE".to_string(),
                    constraints: vec![],
                    result: InvalidResult::Unpredictable,
                });
            }
            Stmt::Case { alts, .. } => {
                for alt in alts {
                    for s in &alt.body {
                        self.extract_invalid_from_stmt(s, conditions);
                    }
                }
            }
            _ => {}
        }
    }

    /// Analyze execute block semantics
    fn analyze_execute(&self, stmts: &[Stmt]) -> SemanticAnalysis {
        semantic::analyze_execute_block(stmts)
    }

    /// Extract control flow paths from execute block
    fn extract_control_paths(&self, stmts: &[Stmt]) -> Vec<ControlPath> {
        // TODO: Implement full control path extraction
        // For now, return a single default path
        vec![ControlPath {
            id: 0,
            conditions: vec![],
            effects: PathEffects {
                writes: vec![],
                memory_ops: vec![],
                exceptions: vec![],
            },
            is_exceptional: false,
        }]
    }

    /// Extract exception conditions from execute block
    fn extract_exceptions(&self, stmts: &[Stmt]) -> Vec<ExceptionCondition> {
        let mut exceptions = Vec::new();

        for stmt in stmts {
            self.extract_exceptions_from_stmt(stmt, &mut exceptions);
        }

        exceptions
    }

    /// Extract exceptions from a statement
    fn extract_exceptions_from_stmt(&self, stmt: &Stmt, exceptions: &mut Vec<ExceptionCondition>) {
        match stmt {
            Stmt::Undefined => {
                exceptions.push(ExceptionCondition {
                    exception_type: ExceptionType::Undefined,
                    trigger: "Unconditional".to_string(),
                    required_state: None,
                });
            }
            Stmt::Unpredictable => {
                exceptions.push(ExceptionCondition {
                    exception_type: ExceptionType::Unpredictable,
                    trigger: "Unconditional".to_string(),
                    required_state: None,
                });
            }
            Stmt::If {
                branches,
                else_body,
            } => {
                for (_, body) in branches {
                    for s in body {
                        self.extract_exceptions_from_stmt(s, exceptions);
                    }
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.extract_exceptions_from_stmt(s, exceptions);
                    }
                }
            }
            Stmt::Case { alts, .. } => {
                for alt in alts {
                    for s in &alt.body {
                        self.extract_exceptions_from_stmt(s, exceptions);
                    }
                }
            }
            Stmt::For { body, .. } | Stmt::While { body, .. } | Stmt::Repeat { body, .. } => {
                for s in body {
                    self.extract_exceptions_from_stmt(s, exceptions);
                }
            }
            _ => {}
        }
    }

    /// Collect function dependencies from execute block
    fn collect_dependencies(&self, stmts: &[Stmt], deps: &mut HashSet<String>) {
        for stmt in stmts {
            self.collect_dependencies_from_stmt(stmt, deps);
        }
    }

    /// Collect dependencies from a statement
    fn collect_dependencies_from_stmt(&self, stmt: &Stmt, deps: &mut HashSet<String>) {
        match stmt {
            Stmt::Call { name, args } => {
                deps.insert(name.name.to_string());
                for arg in args {
                    self.collect_dependencies_from_expr(arg, deps);
                }
            }
            Stmt::VarDeclInit { value, .. } | Stmt::ConstDecl { value, .. } => {
                self.collect_dependencies_from_expr(value, deps);
            }
            Stmt::Assign { value, .. } => {
                self.collect_dependencies_from_expr(value, deps);
            }
            Stmt::If {
                branches,
                else_body,
            } => {
                for (cond, body) in branches {
                    self.collect_dependencies_from_expr(cond, deps);
                    for s in body {
                        self.collect_dependencies_from_stmt(s, deps);
                    }
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_dependencies_from_stmt(s, deps);
                    }
                }
            }
            Stmt::Case { expr, alts } => {
                self.collect_dependencies_from_expr(expr, deps);
                for alt in alts {
                    for s in &alt.body {
                        self.collect_dependencies_from_stmt(s, deps);
                    }
                }
            }
            Stmt::For {
                start, end, body, ..
            } => {
                self.collect_dependencies_from_expr(start, deps);
                self.collect_dependencies_from_expr(end, deps);
                for s in body {
                    self.collect_dependencies_from_stmt(s, deps);
                }
            }
            Stmt::While { cond, body } => {
                self.collect_dependencies_from_expr(cond, deps);
                for s in body {
                    self.collect_dependencies_from_stmt(s, deps);
                }
            }
            Stmt::Repeat { body, cond } => {
                for s in body {
                    self.collect_dependencies_from_stmt(s, deps);
                }
                self.collect_dependencies_from_expr(cond, deps);
            }
            Stmt::Return(Some(expr)) | Stmt::Assert(expr) => {
                self.collect_dependencies_from_expr(expr, deps);
            }
            _ => {}
        }
    }

    /// Collect dependencies from an expression
    fn collect_dependencies_from_expr(&self, expr: &Expr, deps: &mut HashSet<String>) {
        match expr {
            Expr::Call { name, args } => {
                deps.insert(name.name.to_string());
                for arg in args {
                    self.collect_dependencies_from_expr(arg, deps);
                }
            }
            Expr::Binary { lhs, rhs, .. } => {
                self.collect_dependencies_from_expr(lhs, deps);
                self.collect_dependencies_from_expr(rhs, deps);
            }
            Expr::Unary { operand, .. } => {
                self.collect_dependencies_from_expr(operand, deps);
            }
            Expr::If {
                branches,
                else_branch,
            } => {
                for (cond, result) in branches {
                    self.collect_dependencies_from_expr(cond, deps);
                    self.collect_dependencies_from_expr(result, deps);
                }
                self.collect_dependencies_from_expr(else_branch, deps);
            }
            Expr::Slice { base, .. } | Expr::Index { base, .. } | Expr::Member { base, .. } => {
                self.collect_dependencies_from_expr(base, deps);
            }
            Expr::Tuple(exprs) => {
                for e in exprs {
                    self.collect_dependencies_from_expr(e, deps);
                }
            }
            _ => {}
        }
    }
}

impl Default for InstructionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
