//! Semantic analysis of execute blocks.
//!
//! This module analyzes execute statements to extract:
//! - Register reads and writes
//! - Memory operations
//! - Flag effects
//! - System register accesses

use crate::syntax::{Expr, LValExpr, Stmt};
use crate::testgen::types::*;

/// Analyze an execute block and extract semantic information
pub fn analyze_execute_block(stmts: &[Stmt]) -> SemanticAnalysis {
    let mut analyzer = ExecuteAnalyzer::new();
    analyzer.analyze_stmts(stmts);
    analyzer.into_analysis()
}

/// Analyzer for execute block semantics
struct ExecuteAnalyzer {
    register_reads: Vec<RegisterAccess>,
    register_writes: Vec<RegisterAccess>,
    memory_reads: Vec<MemoryAccess>,
    memory_writes: Vec<MemoryAccess>,
    flag_effects: Vec<FlagEffect>,
    system_accesses: Vec<SystemAccess>,
    pc_relative_ops: Vec<PcRelativeOp>,
    computed_values: Vec<ComputedValue>,
    /// Current conditional context (if inside an if/case)
    in_conditional: bool,
    /// Current condition string
    condition_str: Option<String>,
}

impl ExecuteAnalyzer {
    fn new() -> Self {
        Self {
            register_reads: Vec::new(),
            register_writes: Vec::new(),
            memory_reads: Vec::new(),
            memory_writes: Vec::new(),
            flag_effects: Vec::new(),
            system_accesses: Vec::new(),
            pc_relative_ops: Vec::new(),
            computed_values: Vec::new(),
            in_conditional: false,
            condition_str: None,
        }
    }

    fn into_analysis(self) -> SemanticAnalysis {
        SemanticAnalysis {
            register_reads: self.register_reads,
            register_writes: self.register_writes,
            memory_reads: self.memory_reads,
            memory_writes: self.memory_writes,
            flag_effects: self.flag_effects,
            system_accesses: self.system_accesses,
            pc_relative_ops: self.pc_relative_ops,
            computed_values: self.computed_values,
        }
    }

    fn analyze_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign { target, value } => {
                // Analyze the value expression for reads
                self.analyze_expr_reads(value);
                // Analyze the target for writes
                self.analyze_lval_write(target);
            }

            Stmt::VarDeclInit { name, value, .. } => {
                self.analyze_expr_reads(value);
                self.computed_values.push(ComputedValue {
                    name: name.to_string(),
                    computation: format!("{:?}", value),
                    inputs: self.extract_expr_inputs(value),
                });
            }

            Stmt::ConstDecl { name, value, .. } => {
                self.analyze_expr_reads(value);
                self.computed_values.push(ComputedValue {
                    name: name.to_string(),
                    computation: format!("{:?}", value),
                    inputs: self.extract_expr_inputs(value),
                });
            }

            Stmt::Call { name, args } => {
                // Analyze call arguments
                for arg in args {
                    self.analyze_expr_reads(arg);
                }

                // Check for special calls that affect state
                let func_name = name.name.as_str();
                match func_name {
                    // System register checks
                    "CheckSPAlignment" | "CheckFPAdvSIMDEnabled64" | "CheckSVEEnabled" => {
                        self.system_accesses.push(SystemAccess {
                            name: func_name.to_string(),
                            is_write: false,
                            privileged: false,
                        });
                    }
                    _ => {}
                }
            }

            Stmt::If {
                branches,
                else_body,
            } => {
                let was_conditional = self.in_conditional;
                self.in_conditional = true;

                for (cond, body) in branches {
                    self.condition_str = Some(format!("{:?}", cond));
                    self.analyze_expr_reads(cond);
                    self.analyze_stmts(body);
                }

                if let Some(else_stmts) = else_body {
                    self.condition_str = Some("else".to_string());
                    self.analyze_stmts(else_stmts);
                }

                self.in_conditional = was_conditional;
                self.condition_str = None;
            }

            Stmt::Case { expr, alts } => {
                self.analyze_expr_reads(expr);

                let was_conditional = self.in_conditional;
                self.in_conditional = true;

                for alt in alts {
                    self.condition_str = Some(format!("{:?}", alt.patterns));
                    self.analyze_stmts(&alt.body);
                }

                self.in_conditional = was_conditional;
                self.condition_str = None;
            }

            Stmt::For {
                start, end, body, ..
            } => {
                self.analyze_expr_reads(start);
                self.analyze_expr_reads(end);
                self.analyze_stmts(body);
            }

            Stmt::While { cond, body } => {
                self.analyze_expr_reads(cond);
                self.analyze_stmts(body);
            }

            Stmt::Repeat { body, cond } => {
                self.analyze_stmts(body);
                self.analyze_expr_reads(cond);
            }

            Stmt::Return(Some(expr)) => {
                self.analyze_expr_reads(expr);
            }

            Stmt::Assert(expr) => {
                self.analyze_expr_reads(expr);
            }

            _ => {}
        }
    }

    /// Analyze an expression for register/memory reads
    fn analyze_expr_reads(&mut self, expr: &Expr) {
        match expr {
            Expr::Call { name, args } => {
                let func_name = name.name.as_str();

                match func_name {
                    // Register reads
                    "X" | "Xread" => {
                        if let Some(reg) = self.extract_register_index(args.first()) {
                            self.register_reads.push(RegisterAccess {
                                register: RegisterId::GpFromField(reg),
                                width: 64,
                                conditional: self.in_conditional,
                                condition: self.condition_str.clone(),
                            });
                        }
                    }
                    "W" => {
                        if let Some(reg) = self.extract_register_index(args.first()) {
                            self.register_reads.push(RegisterAccess {
                                register: RegisterId::GpFromField(reg),
                                width: 32,
                                conditional: self.in_conditional,
                                condition: self.condition_str.clone(),
                            });
                        }
                    }
                    "V" | "Vread" => {
                        if let Some(reg) = self.extract_register_index(args.first()) {
                            self.register_reads.push(RegisterAccess {
                                register: RegisterId::SimdFromField(reg),
                                width: 128,
                                conditional: self.in_conditional,
                                condition: self.condition_str.clone(),
                            });
                        }
                    }
                    "SP" => {
                        self.register_reads.push(RegisterAccess {
                            register: RegisterId::Sp,
                            width: 64,
                            conditional: self.in_conditional,
                            condition: self.condition_str.clone(),
                        });
                    }
                    "PC" => {
                        self.register_reads.push(RegisterAccess {
                            register: RegisterId::Pc,
                            width: 64,
                            conditional: self.in_conditional,
                            condition: self.condition_str.clone(),
                        });
                    }
                    // Memory reads
                    "Mem" | "MemU" | "MemO" | "MemA" => {
                        if args.len() >= 2 {
                            let size = self.extract_memory_size(args.get(1));
                            let addressing = self.extract_addressing_info(args.first());
                            self.memory_reads.push(MemoryAccess {
                                size,
                                sign_extend: false,
                                addressing,
                                alignment: None,
                                atomic: func_name == "MemA",
                                ordering: None,
                            });
                        }
                    }
                    // SVE/predicate registers
                    "Z" => {
                        if let Some(reg) = self.extract_register_index(args.first()) {
                            self.register_reads.push(RegisterAccess {
                                register: RegisterId::SimdFromField(reg),
                                width: 0, // VL-dependent
                                conditional: self.in_conditional,
                                condition: self.condition_str.clone(),
                            });
                        }
                    }
                    "P" => {
                        if let Some(reg) = self.extract_register_index(args.first()) {
                            self.register_reads.push(RegisterAccess {
                                register: RegisterId::SimdFromField(format!("P{}", reg)),
                                width: 0, // PL-dependent
                                conditional: self.in_conditional,
                                condition: self.condition_str.clone(),
                            });
                        }
                    }
                    _ => {}
                }

                // Recurse into arguments
                for arg in args {
                    self.analyze_expr_reads(arg);
                }
            }

            Expr::Member { base, field } => {
                // Check for PSTATE.N, PSTATE.Z, etc.
                if let Expr::Var(qid) = base.as_ref() {
                    if qid.name.as_str() == "PSTATE" {
                        // This is a flag read
                        if let Some(flag) = self.field_to_flag(field.as_str()) {
                            // Track this as reading the flag
                        }
                    }
                }
                self.analyze_expr_reads(base);
            }

            Expr::Binary { lhs, rhs, .. } => {
                self.analyze_expr_reads(lhs);
                self.analyze_expr_reads(rhs);
            }

            Expr::Unary { operand, .. } => {
                self.analyze_expr_reads(operand);
            }

            Expr::Slice { base, .. } | Expr::Index { base, .. } => {
                self.analyze_expr_reads(base);
            }

            Expr::If {
                branches,
                else_branch,
            } => {
                for (cond, result) in branches {
                    self.analyze_expr_reads(cond);
                    self.analyze_expr_reads(result);
                }
                self.analyze_expr_reads(else_branch);
            }

            Expr::Tuple(exprs) => {
                for e in exprs {
                    self.analyze_expr_reads(e);
                }
            }

            _ => {}
        }
    }

    /// Analyze an l-value for register/memory writes
    fn analyze_lval_write(&mut self, lval: &LValExpr) {
        match lval {
            LValExpr::Var(qid) => {
                let name = qid.name.as_str();
                // Check for PSTATE writes
                if name == "PSTATE" {
                    // This should be handled by member access
                }
            }

            LValExpr::Index { base, indices } => {
                // Check for register array writes like X[d], V[t], etc.
                if let LValExpr::Var(qid) = base.as_ref() {
                    let name = qid.name.as_str();
                    match name {
                        "X" => {
                            if let Some(idx) = self.extract_lval_index(indices.first()) {
                                self.register_writes.push(RegisterAccess {
                                    register: RegisterId::GpFromField(idx),
                                    width: 64,
                                    conditional: self.in_conditional,
                                    condition: self.condition_str.clone(),
                                });
                            }
                        }
                        "V" => {
                            if let Some(idx) = self.extract_lval_index(indices.first()) {
                                self.register_writes.push(RegisterAccess {
                                    register: RegisterId::SimdFromField(idx),
                                    width: 128,
                                    conditional: self.in_conditional,
                                    condition: self.condition_str.clone(),
                                });
                            }
                        }
                        "Z" => {
                            if let Some(idx) = self.extract_lval_index(indices.first()) {
                                self.register_writes.push(RegisterAccess {
                                    register: RegisterId::SimdFromField(idx),
                                    width: 0, // VL-dependent
                                    conditional: self.in_conditional,
                                    condition: self.condition_str.clone(),
                                });
                            }
                        }
                        "P" => {
                            if let Some(idx) = self.extract_lval_index(indices.first()) {
                                self.register_writes.push(RegisterAccess {
                                    register: RegisterId::SimdFromField(format!("P{}", idx)),
                                    width: 0, // PL-dependent
                                    conditional: self.in_conditional,
                                    condition: self.condition_str.clone(),
                                });
                            }
                        }
                        "Mem" | "MemU" | "MemO" | "MemA" => {
                            // Memory write - extract size from indices
                            let size = 8; // Default, should be extracted from context
                            self.memory_writes.push(MemoryAccess {
                                size,
                                sign_extend: false,
                                addressing: AddressingInfo::Base {
                                    reg: "address".to_string(),
                                },
                                alignment: None,
                                atomic: name == "MemA",
                                ordering: None,
                            });
                        }
                        "SP" => {
                            self.register_writes.push(RegisterAccess {
                                register: RegisterId::Sp,
                                width: 64,
                                conditional: self.in_conditional,
                                condition: self.condition_str.clone(),
                            });
                        }
                        _ => {}
                    }
                }
            }

            LValExpr::Member { base, field } => {
                // Check for PSTATE.<flag> writes
                if let LValExpr::Var(qid) = base.as_ref() {
                    if qid.name.as_str() == "PSTATE" {
                        if let Some(flag) = self.field_to_flag(field.as_str()) {
                            self.flag_effects.push(FlagEffect {
                                flag,
                                computation: FlagComputation::Complex(format!(
                                    "PSTATE.{} = ...",
                                    field
                                )),
                                conditional: self.in_conditional,
                            });
                        }
                    }
                }
            }

            LValExpr::MemberBits { base, fields } => {
                // PSTATE.<N,Z,C,V> = nzcv pattern
                if let LValExpr::Var(qid) = base.as_ref() {
                    if qid.name.as_str() == "PSTATE" {
                        for field in fields {
                            if let Some(flag) = self.field_to_flag(field.as_str()) {
                                self.flag_effects.push(FlagEffect {
                                    flag,
                                    computation: FlagComputation::Complex(
                                        "from NZCV result".to_string(),
                                    ),
                                    conditional: self.in_conditional,
                                });
                            }
                        }
                    }
                }
            }

            LValExpr::Tuple(lvals) => {
                for lv in lvals {
                    self.analyze_lval_write(lv);
                }
            }

            LValExpr::Slice { base, .. } => {
                self.analyze_lval_write(base);
            }

            _ => {}
        }
    }

    /// Extract register index from an expression
    fn extract_register_index(&self, expr: Option<&Expr>) -> Option<String> {
        match expr? {
            Expr::Var(qid) => Some(qid.name.to_string()),
            Expr::LitInt(n) => Some(n.to_string()),
            _ => None,
        }
    }

    /// Extract index from l-value slice
    fn extract_lval_index(&self, slice: Option<&crate::syntax::Slice>) -> Option<String> {
        match slice? {
            crate::syntax::Slice::Single(expr) => match expr.as_ref() {
                Expr::Var(qid) => Some(qid.name.to_string()),
                Expr::LitInt(n) => Some(n.to_string()),
                _ => None,
            },
            _ => None,
        }
    }

    /// Extract memory size from expression
    fn extract_memory_size(&self, expr: Option<&Expr>) -> u32 {
        match expr {
            Some(Expr::LitInt(n)) => *n as u32,
            Some(Expr::Binary { op, lhs, rhs }) => {
                use crate::syntax::BinOp;
                if *op == BinOp::Div {
                    // pattern: datasize DIV 8
                    if let (Expr::Var(_), Expr::LitInt(8)) = (lhs.as_ref(), rhs.as_ref()) {
                        return 0; // Size depends on datasize variable
                    }
                }
                0
            }
            _ => 0,
        }
    }

    /// Extract addressing information from expression
    fn extract_addressing_info(&self, expr: Option<&Expr>) -> AddressingInfo {
        match expr {
            Some(Expr::Var(qid)) => AddressingInfo::Base {
                reg: qid.name.to_string(),
            },
            Some(Expr::Binary { op, lhs, rhs }) => {
                use crate::syntax::BinOp;
                if *op == BinOp::Add {
                    if let Expr::Var(base) = lhs.as_ref() {
                        if let Expr::Var(offset) = rhs.as_ref() {
                            return AddressingInfo::BaseRegister {
                                base: base.name.to_string(),
                                offset: offset.name.to_string(),
                            };
                        } else {
                            return AddressingInfo::BaseImmediate {
                                reg: base.name.to_string(),
                                offset_field: format!("{:?}", rhs),
                            };
                        }
                    }
                }
                AddressingInfo::Base {
                    reg: "complex".to_string(),
                }
            }
            _ => AddressingInfo::Base {
                reg: "unknown".to_string(),
            },
        }
    }

    /// Convert field name to processor flag
    fn field_to_flag(&self, field: &str) -> Option<ProcessorFlag> {
        match field {
            "N" => Some(ProcessorFlag::N),
            "Z" => Some(ProcessorFlag::Z),
            "C" => Some(ProcessorFlag::C),
            "V" => Some(ProcessorFlag::V),
            _ => None,
        }
    }

    /// Extract input variable names from an expression
    fn extract_expr_inputs(&self, expr: &Expr) -> Vec<String> {
        let mut inputs = Vec::new();
        self.collect_expr_vars(expr, &mut inputs);
        inputs
    }

    /// Collect variable names from expression
    fn collect_expr_vars(&self, expr: &Expr, vars: &mut Vec<String>) {
        match expr {
            Expr::Var(qid) => {
                vars.push(qid.name.to_string());
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.collect_expr_vars(arg, vars);
                }
            }
            Expr::Binary { lhs, rhs, .. } => {
                self.collect_expr_vars(lhs, vars);
                self.collect_expr_vars(rhs, vars);
            }
            Expr::Unary { operand, .. } => {
                self.collect_expr_vars(operand, vars);
            }
            Expr::Slice { base, .. } | Expr::Index { base, .. } | Expr::Member { base, .. } => {
                self.collect_expr_vars(base, vars);
            }
            Expr::If {
                branches,
                else_branch,
            } => {
                for (cond, result) in branches {
                    self.collect_expr_vars(cond, vars);
                    self.collect_expr_vars(result, vars);
                }
                self.collect_expr_vars(else_branch, vars);
            }
            Expr::Tuple(exprs) => {
                for e in exprs {
                    self.collect_expr_vars(e, vars);
                }
            }
            _ => {}
        }
    }
}
