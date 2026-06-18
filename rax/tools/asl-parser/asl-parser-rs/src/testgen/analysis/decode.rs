//! Decode block analysis.
//!
//! This module analyzes decode statements to extract:
//! - Variable computations
//! - UNDEFINED conditions
//! - Field transformations (sign extend, zero extend, shift, etc.)

use crate::syntax::{BinOp, Expr, Stmt};
use crate::testgen::types::*;

/// Represents a decoded variable and how it was computed
#[derive(Debug, Clone)]
pub struct DecodedVariable {
    /// Variable name
    pub name: String,
    /// How it was computed
    pub computation: VariableComputation,
    /// Source fields/variables used
    pub sources: Vec<String>,
    /// Computed value (if constant)
    pub constant_value: Option<i128>,
}

/// How a variable is computed during decode
#[derive(Debug, Clone)]
pub enum VariableComputation {
    /// Direct field extraction: d = UInt(Rd)
    FieldExtract { field: String },
    /// Zero extension: imm = ZeroExtend(imm12, 64)
    ZeroExtend { source: String, to_bits: u32 },
    /// Sign extension: offset = SignExtend(imm9, 64)
    SignExtend { source: String, to_bits: u32 },
    /// Left shift: imm = imm12 << 12
    LeftShift { source: String, amount: u32 },
    /// Concatenation: size = sf:opc
    Concatenate { sources: Vec<String> },
    /// Arithmetic: esize = 8 << UInt(size)
    Arithmetic { op: String, operands: Vec<String> },
    /// Conditional: datasize = if Q == '1' then 128 else 64
    Conditional {
        condition: String,
        if_true: String,
        if_false: String,
    },
    /// Constant value
    Constant(i128),
    /// Complex expression we can't simplify
    Complex(String),
}

/// Extract all decoded variables from decode statements
pub fn extract_decoded_variables(stmts: &[Stmt]) -> Vec<DecodedVariable> {
    let mut variables = Vec::new();

    for stmt in stmts {
        if let Some(var) = extract_variable_from_stmt(stmt) {
            variables.push(var);
        }
    }

    variables
}

/// Extract a decoded variable from a statement
fn extract_variable_from_stmt(stmt: &Stmt) -> Option<DecodedVariable> {
    match stmt {
        Stmt::VarDeclInit { name, value, .. } | Stmt::ConstDecl { name, value, .. } => {
            let (computation, sources, constant) = analyze_computation(value);
            Some(DecodedVariable {
                name: name.to_string(),
                computation,
                sources,
                constant_value: constant,
            })
        }
        Stmt::Assign { target, value } => {
            if let crate::syntax::LValExpr::Var(qid) = target {
                let (computation, sources, constant) = analyze_computation(value);
                Some(DecodedVariable {
                    name: qid.name.to_string(),
                    computation,
                    sources,
                    constant_value: constant,
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Analyze a computation expression
fn analyze_computation(expr: &Expr) -> (VariableComputation, Vec<String>, Option<i128>) {
    match expr {
        Expr::LitInt(n) => (VariableComputation::Constant(*n), vec![], Some(*n)),

        Expr::Var(qid) => (
            VariableComputation::FieldExtract {
                field: qid.name.to_string(),
            },
            vec![qid.name.to_string()],
            None,
        ),

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

            match func_name {
                "UInt" | "SInt" => {
                    let field = sources.first().cloned().unwrap_or_default();
                    (VariableComputation::FieldExtract { field }, sources, None)
                }
                "ZeroExtend" => {
                    let source = sources.first().cloned().unwrap_or_default();
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
                    (
                        VariableComputation::ZeroExtend { source, to_bits },
                        sources,
                        None,
                    )
                }
                "SignExtend" => {
                    let source = sources.first().cloned().unwrap_or_default();
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
                    (
                        VariableComputation::SignExtend { source, to_bits },
                        sources,
                        None,
                    )
                }
                "LSL" => {
                    let source = sources.first().cloned().unwrap_or_default();
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
                    (
                        VariableComputation::LeftShift { source, amount },
                        sources,
                        None,
                    )
                }
                _ => (
                    VariableComputation::Complex(format!("{}(...)", func_name)),
                    sources,
                    None,
                ),
            }
        }

        Expr::Binary { op, lhs, rhs } => {
            let mut sources = Vec::new();
            collect_vars(lhs, &mut sources);
            collect_vars(rhs, &mut sources);

            match op {
                BinOp::BitConcat => (
                    VariableComputation::Concatenate {
                        sources: sources.clone(),
                    },
                    sources,
                    None,
                ),
                BinOp::Shl => {
                    let source = if let Expr::Var(qid) = lhs.as_ref() {
                        qid.name.to_string()
                    } else {
                        String::new()
                    };
                    let amount = if let Expr::LitInt(n) = rhs.as_ref() {
                        *n as u32
                    } else {
                        0
                    };
                    (
                        VariableComputation::LeftShift { source, amount },
                        sources,
                        None,
                    )
                }
                _ => (
                    VariableComputation::Arithmetic {
                        op: format!("{:?}", op),
                        operands: sources.clone(),
                    },
                    sources,
                    None,
                ),
            }
        }

        Expr::If {
            branches,
            else_branch,
        } => {
            if branches.len() == 1 {
                let (cond, if_true) = &branches[0];
                (
                    VariableComputation::Conditional {
                        condition: format!("{:?}", cond),
                        if_true: format!("{:?}", if_true),
                        if_false: format!("{:?}", else_branch),
                    },
                    vec![],
                    None,
                )
            } else {
                (
                    VariableComputation::Complex(format!("{:?}", expr)),
                    vec![],
                    None,
                )
            }
        }

        _ => (
            VariableComputation::Complex(format!("{:?}", expr)),
            vec![],
            None,
        ),
    }
}

/// Collect variable names from an expression
fn collect_vars(expr: &Expr, vars: &mut Vec<String>) {
    match expr {
        Expr::Var(qid) => {
            vars.push(qid.name.to_string());
        }
        Expr::Call { args, .. } => {
            for arg in args {
                collect_vars(arg, vars);
            }
        }
        Expr::Binary { lhs, rhs, .. } => {
            collect_vars(lhs, vars);
            collect_vars(rhs, vars);
        }
        Expr::Unary { operand, .. } => {
            collect_vars(operand, vars);
        }
        _ => {}
    }
}

/// Extract UNDEFINED conditions from decode statements  
pub fn extract_undefined_conditions(stmts: &[Stmt]) -> Vec<UndefinedCondition> {
    let mut conditions = Vec::new();

    for stmt in stmts {
        extract_undefined_from_stmt(stmt, &mut conditions);
    }

    conditions
}

/// A condition that triggers UNDEFINED
#[derive(Debug, Clone)]
pub struct UndefinedCondition {
    /// The condition expression
    pub condition: String,
    /// Simplified field constraints
    pub constraints: Vec<(String, UndefinedConstraint)>,
}

/// Constraint type for UNDEFINED conditions
#[derive(Debug, Clone)]
pub enum UndefinedConstraint {
    Equals(u64),
    NotEquals(u64),
    BitPattern(String),
    Complex(String),
}

fn extract_undefined_from_stmt(stmt: &Stmt, conditions: &mut Vec<UndefinedCondition>) {
    match stmt {
        Stmt::If {
            branches,
            else_body,
        } => {
            for (cond, body) in branches {
                // Check if this branch leads to UNDEFINED
                if body.iter().any(|s| matches!(s, Stmt::Undefined)) {
                    conditions.push(UndefinedCondition {
                        condition: format!("{:?}", cond),
                        constraints: extract_constraints_from_condition(cond),
                    });
                } else {
                    // Recurse into body
                    for s in body {
                        extract_undefined_from_stmt(s, conditions);
                    }
                }
            }

            // Check else body
            if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    extract_undefined_from_stmt(s, conditions);
                }
            }
        }

        Stmt::Case { alts, .. } => {
            for alt in alts {
                for s in &alt.body {
                    extract_undefined_from_stmt(s, conditions);
                }
            }
        }

        Stmt::Undefined => {
            // Unconditional UNDEFINED - shouldn't normally happen in well-formed ASL
            conditions.push(UndefinedCondition {
                condition: "unconditional".to_string(),
                constraints: vec![],
            });
        }

        _ => {}
    }
}

/// Try to extract field constraints from a condition expression
fn extract_constraints_from_condition(expr: &Expr) -> Vec<(String, UndefinedConstraint)> {
    let mut constraints = Vec::new();

    match expr {
        Expr::Binary { op, lhs, rhs } => match op {
            BinOp::Eq => {
                if let (Expr::Var(qid), Expr::LitBits(bits)) = (lhs.as_ref(), rhs.as_ref()) {
                    let pattern: String = bits.iter().map(|b| if *b { '1' } else { '0' }).collect();
                    constraints.push((
                        qid.name.to_string(),
                        UndefinedConstraint::BitPattern(pattern),
                    ));
                } else if let (Expr::Var(qid), Expr::LitInt(n)) = (lhs.as_ref(), rhs.as_ref()) {
                    constraints
                        .push((qid.name.to_string(), UndefinedConstraint::Equals(*n as u64)));
                }
            }
            BinOp::Ne => {
                if let (Expr::Var(qid), Expr::LitInt(n)) = (lhs.as_ref(), rhs.as_ref()) {
                    constraints.push((
                        qid.name.to_string(),
                        UndefinedConstraint::NotEquals(*n as u64),
                    ));
                }
            }
            BinOp::And => {
                constraints.extend(extract_constraints_from_condition(lhs));
                constraints.extend(extract_constraints_from_condition(rhs));
            }
            _ => {}
        },
        _ => {}
    }

    constraints
}
