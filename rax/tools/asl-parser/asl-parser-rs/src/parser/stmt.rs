//! Statement parsing for ASL.

use super::Parser;
use crate::error::Result;
use crate::lexer::TokenKind;
use crate::syntax::*;
use smol_str::SmolStr;

impl<'src> Parser<'src> {
    /// Parse a block of statements (from an indented block).
    pub fn parse_stmt_block(&mut self) -> Result<Vec<Stmt>> {
        self.parse_indented_block(|p| p.parse_stmt())
    }

    /// Parse a single statement.
    pub fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.current_kind().clone() {
            // Return statement
            TokenKind::Return => {
                self.advance();
                let expr = if self.check(&TokenKind::Semi) || self.check(&TokenKind::Dedent) {
                    None
                } else {
                    Some(self.parse_expr()?)
                };
                self.eat(&TokenKind::Semi);
                Ok(Stmt::Return(expr))
            }

            // Assert statement
            TokenKind::Assert => {
                self.advance();
                let expr = self.parse_expr()?;
                self.eat(&TokenKind::Semi);
                Ok(Stmt::Assert(expr))
            }

            // UNPREDICTABLE
            TokenKind::Unpredictable => {
                self.advance();
                self.eat(&TokenKind::Semi);
                Ok(Stmt::Unpredictable)
            }

            // UNDEFINED
            TokenKind::Undefined => {
                self.advance();
                self.eat(&TokenKind::Semi);
                Ok(Stmt::Undefined)
            }

            // IMPLEMENTATION_DEFINED
            TokenKind::ImplementationDefined => {
                self.advance();
                let desc = if let TokenKind::StringLit(s) = self.current_kind() {
                    let s = s.clone();
                    self.advance();
                    SmolStr::new(&s)
                } else {
                    SmolStr::new("")
                };
                self.eat(&TokenKind::Semi);
                Ok(Stmt::ImpDef(desc))
            }

            // SEE
            TokenKind::See => {
                self.advance();
                // SEE can be followed by:
                // - A string literal: SEE "description"
                // - An identifier: SEE encoding_name
                // - A parenthesized identifier: SEE(encoding_name)
                let msg = if let TokenKind::StringLit(s) = self.current_kind() {
                    let s = s.clone();
                    self.advance();
                    SmolStr::new(&s)
                } else if self.eat(&TokenKind::LParen).is_some() {
                    // Parenthesized form: SEE(name)
                    let s = if let TokenKind::Ident(name) = self.current_kind() {
                        let name = name.clone();
                        self.advance();
                        name
                    } else {
                        String::new()
                    };
                    self.expect(&TokenKind::RParen)?;
                    SmolStr::new(&s)
                } else if let TokenKind::Ident(s) = self.current_kind() {
                    // Plain identifier
                    let s = s.clone();
                    self.advance();
                    SmolStr::new(&s)
                } else {
                    SmolStr::new("")
                };
                self.eat(&TokenKind::Semi);
                Ok(Stmt::See(msg))
            }

            // If statement
            TokenKind::If => self.parse_if_stmt(),

            // Case statement
            TokenKind::Case => self.parse_case_stmt(),

            // For loop
            TokenKind::For => self.parse_for_stmt(),

            // While loop
            TokenKind::While => self.parse_while_stmt(),

            // Repeat loop
            TokenKind::Repeat => self.parse_repeat_stmt(),

            // Try-catch statement
            TokenKind::Try => self.parse_try_stmt(),

            // Constant declaration
            TokenKind::Constant => {
                self.advance();
                let ty = self.parse_type()?;
                let name = self.expect_ident()?;
                self.expect(&TokenKind::Eq)?;
                let value = self.parse_expr()?;
                self.eat(&TokenKind::Semi);
                Ok(Stmt::ConstDecl { name, ty, value })
            }

            // Local enumeration
            TokenKind::Enumeration => {
                self.advance();
                let name = self.expect_ident()?;
                self.expect(&TokenKind::LBrace)?;
                let variants = self.parse_list1(|p| p.expect_ident())?;
                self.expect(&TokenKind::RBrace)?;
                self.eat(&TokenKind::Semi);
                Ok(Stmt::DefEnum { name, variants })
            }

            // Let binding (same as constant)
            TokenKind::Let => {
                self.advance();
                let name = self.expect_ident()?;
                self.expect(&TokenKind::Eq)?;
                let value = self.parse_expr()?;
                self.eat(&TokenKind::Semi);
                // Infer type from expression
                let ty = Type::Of(Box::new(value.clone()));
                Ok(Stmt::ConstDecl { name, ty, value })
            }

            // Throw statement
            TokenKind::Throw => {
                self.advance();
                let exc = self.expect_ident()?;
                self.eat(&TokenKind::Semi);
                Ok(Stmt::Throw(exc))
            }

            // Variable declaration or assignment
            // This is tricky because we need to distinguish:
            // - Type var;
            // - Type var = expr;
            // - var = expr;
            // - expr.field = expr;
            // - func(args);
            _ => self.parse_decl_or_assign_stmt(),
        }
    }

    /// Parse a declaration or assignment statement.
    fn parse_decl_or_assign_stmt(&mut self) -> Result<Stmt> {
        // Special case: discard assignment - = expr
        if self.check(&TokenKind::Minus) {
            self.advance();
            if self.eat(&TokenKind::Eq).is_some() {
                // Discard assignment: - = expr
                let value = self.parse_expr()?;
                self.eat(&TokenKind::Semi);
                return Ok(Stmt::Assign {
                    target: LValExpr::Ignore,
                    value,
                });
            } else {
                // Not a discard, this shouldn't happen in statement context
                return Err(self.error("expected '=' after '-' in discard assignment"));
            }
        }

        // Special case: tuple starting with ( could be assignment like (a, b) = expr
        // or declaration like (Type1, Type2) name = expr
        // We need to parse and then decide based on what follows
        if self.check(&TokenKind::LParen) {
            // Parse as expression first
            let expr = self.parse_expr()?;

            if self.eat(&TokenKind::Eq).is_some() {
                // Tuple assignment: (a, b) = expr
                let target = self.expr_to_lval(expr)?;
                let value = self.parse_expr()?;
                self.eat(&TokenKind::Semi);
                return Ok(Stmt::Assign { target, value });
            } else if let Some(name) = self.get_ident() {
                // Tuple type declaration: (T1, T2) name
                // The expr should be a tuple of type references
                let ty = self.expr_to_tuple_type(expr)?;
                let name = SmolStr::new(name);
                self.advance();

                if self.eat(&TokenKind::Eq).is_some() {
                    let value = self.parse_expr()?;
                    self.eat(&TokenKind::Semi);
                    return Ok(Stmt::VarDeclInit { name, ty, value });
                } else {
                    self.eat(&TokenKind::Semi);
                    return Ok(Stmt::VarsDecl {
                        ty,
                        names: vec![name],
                    });
                }
            } else {
                return Err(self.error("expected assignment or declaration after tuple"));
            }
        }

        // Try to parse as type first (for known types)
        if self.is_type_start() {
            let ty = self.parse_type()?;

            // If followed by an identifier, it's a declaration
            if let Some(name) = self.get_ident() {
                let first_name = SmolStr::new(name);
                self.advance();

                if self.eat(&TokenKind::Eq).is_some() {
                    // Type name = expr;
                    let value = self.parse_expr()?;
                    self.eat(&TokenKind::Semi);
                    return Ok(Stmt::VarDeclInit {
                        name: first_name,
                        ty,
                        value,
                    });
                } else if self.eat(&TokenKind::Comma).is_some() {
                    // Type name1, name2, ...;
                    let mut names = vec![first_name];
                    names.extend(self.parse_list1(|p| p.expect_ident())?);
                    self.eat(&TokenKind::Semi);
                    return Ok(Stmt::VarsDecl { ty, names });
                } else {
                    // Type name;
                    self.eat(&TokenKind::Semi);
                    return Ok(Stmt::VarsDecl {
                        ty,
                        names: vec![first_name],
                    });
                }
            }

            // Not followed by identifier - convert type to expression and continue
            // This handles cases like bits(32) Foo() - unlikely but possible
            let expr = self.type_to_expr(ty)?;
            return self.finish_expr_stmt(expr);
        }

        // Parse as expression (could be assignment, call, or declaration with qualified type)
        let expr = self.parse_expr()?;

        // Check if this is actually a type followed by variable name
        // Pattern: QualifiedIdent identifier (no parens after first ident)
        if let Some(name) = self.get_ident() {
            // The parsed expression might be a type reference
            if let Ok(ty) = self.expr_to_type(expr.clone()) {
                let first_name = SmolStr::new(name);
                self.advance();

                if self.eat(&TokenKind::Eq).is_some() {
                    // Type name = expr;
                    let value = self.parse_expr()?;
                    self.eat(&TokenKind::Semi);
                    return Ok(Stmt::VarDeclInit {
                        name: first_name,
                        ty,
                        value,
                    });
                } else if self.eat(&TokenKind::Comma).is_some() {
                    // Type name1, name2, ...;
                    let mut names = vec![first_name];
                    names.extend(self.parse_list1(|p| p.expect_ident())?);
                    self.eat(&TokenKind::Semi);
                    return Ok(Stmt::VarsDecl { ty, names });
                } else {
                    // Type name;
                    self.eat(&TokenKind::Semi);
                    return Ok(Stmt::VarsDecl {
                        ty,
                        names: vec![first_name],
                    });
                }
            }
        }

        self.finish_expr_stmt(expr)
    }

    /// Finish parsing an expression statement (assignment or call).
    fn finish_expr_stmt(&mut self, expr: Expr) -> Result<Stmt> {
        if self.eat(&TokenKind::Eq).is_some() {
            // Assignment
            let target = self.expr_to_lval(expr)?;
            let value = self.parse_expr()?;
            self.eat(&TokenKind::Semi);
            Ok(Stmt::Assign { target, value })
        } else if let Expr::Call { name, args } = expr {
            // Procedure call
            self.eat(&TokenKind::Semi);
            Ok(Stmt::Call { name, args })
        } else {
            Err(self.error("expected assignment or procedure call"))
        }
    }

    /// Convert a type to an expression (for when we mistakenly parsed as type).
    fn type_to_expr(&self, ty: Type) -> Result<Expr> {
        match ty {
            Type::Ref(qid) => Ok(Expr::Var(qid)),
            Type::Fun { name, arg } => Ok(Expr::Call {
                name: QualifiedIdentifier::new(name),
                args: vec![*arg],
            }),
            _ => Err(self.error("cannot convert type to expression")),
        }
    }

    /// Convert an expression to a tuple type (for tuple declarations).
    fn expr_to_tuple_type(&self, expr: Expr) -> Result<Type> {
        match expr {
            Expr::Tuple(exprs) => {
                let types: Result<Vec<Type>> =
                    exprs.into_iter().map(|e| self.expr_to_type(e)).collect();
                Ok(Type::Tuple(types?))
            }
            _ => self.expr_to_type(expr),
        }
    }

    /// Convert an expression to a type.
    fn expr_to_type(&self, expr: Expr) -> Result<Type> {
        match expr {
            Expr::Var(qid) => Ok(Type::Ref(qid)),
            Expr::Call { name, args } if args.len() == 1 => Ok(Type::Fun {
                name: name.name,
                arg: Box::new(args.into_iter().next().unwrap()),
            }),
            _ => Err(self.error("expected type")),
        }
    }

    /// Convert an expression to an l-value.
    fn expr_to_lval(&self, expr: Expr) -> Result<LValExpr> {
        match expr {
            Expr::Var(name) => Ok(LValExpr::Var(name)),
            Expr::Member { base, field } => {
                let base_lval = self.expr_to_lval(*base)?;
                Ok(LValExpr::Member {
                    base: Box::new(base_lval),
                    field,
                })
            }
            Expr::Members { base, fields } => {
                let base_lval = self.expr_to_lval(*base)?;
                Ok(LValExpr::Members {
                    base: Box::new(base_lval),
                    fields,
                })
            }
            Expr::Index { base, indices } => {
                let base_lval = self.expr_to_lval(*base)?;
                Ok(LValExpr::Index {
                    base: Box::new(base_lval),
                    indices,
                })
            }
            Expr::Slice { base, slices } => {
                let base_lval = self.expr_to_lval(*base)?;
                Ok(LValExpr::Slice {
                    base: Box::new(base_lval),
                    slices,
                })
            }
            Expr::Tuple(exprs) => {
                let lvals: Result<Vec<_>> =
                    exprs.into_iter().map(|e| self.expr_to_lval(e)).collect();
                Ok(LValExpr::Tuple(lvals?))
            }
            Expr::MemberBits { base, fields } => {
                let base_lval = self.expr_to_lval(*base)?;
                Ok(LValExpr::MemberBits {
                    base: Box::new(base_lval),
                    fields,
                })
            }
            Expr::Discard => Ok(LValExpr::Ignore),
            _ => Err(self.error("invalid l-value expression")),
        }
    }

    /// Parse an if statement.
    ///
    /// Handles both block form and single-line form:
    /// - `if cond then <newline> INDENT stmts DEDENT`
    /// - `if cond then stmt;`
    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        self.expect(&TokenKind::If)?;

        let mut branches = Vec::new();

        // First condition
        let cond = self.parse_expr()?;
        self.expect(&TokenKind::Then)?;
        let body = self.parse_stmt_or_block()?;
        branches.push((cond, body));

        // Elsif branches
        while self.eat(&TokenKind::Elsif).is_some() {
            let cond = self.parse_expr()?;
            self.expect(&TokenKind::Then)?;
            let body = self.parse_stmt_or_block()?;
            branches.push((cond, body));
        }

        // Else branch
        let else_body = if self.eat(&TokenKind::Else).is_some() {
            Some(self.parse_stmt_or_block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            branches,
            else_body,
        })
    }

    /// Parse either a single statement or an indented block.
    fn parse_stmt_or_block(&mut self) -> Result<Vec<Stmt>> {
        if self.check(&TokenKind::Indent) {
            self.parse_stmt_block()
        } else {
            // Single statement on the same line
            let stmt = self.parse_stmt()?;
            Ok(vec![stmt])
        }
    }

    /// Parse a case statement.
    fn parse_case_stmt(&mut self) -> Result<Stmt> {
        self.expect(&TokenKind::Case)?;
        let expr = self.parse_expr()?;
        self.expect(&TokenKind::Of)?;

        let alts = self.parse_indented_block(|p| p.parse_case_alternative())?;

        Ok(Stmt::Case { expr, alts })
    }

    /// Parse a case alternative.
    ///
    /// Supports optional guard expression: `when pattern && guard_expr body`
    fn parse_case_alternative(&mut self) -> Result<CaseAlternative> {
        if self.eat(&TokenKind::When).is_some() {
            let patterns = self.parse_list1(|p| p.parse_case_pattern())?;

            // Parse optional guard expression (after &&)
            let guard = if self.eat(&TokenKind::AndAnd).is_some() {
                Some(self.parse_expr()?)
            } else {
                None
            };

            // Fat arrow is optional - body can follow on next line with indent
            self.eat(&TokenKind::FatArrow);
            let body = self.parse_case_body()?;
            Ok(CaseAlternative {
                patterns: Some(patterns),
                guard,
                body,
            })
        } else if self.eat(&TokenKind::Otherwise).is_some() {
            // Fat arrow is optional
            self.eat(&TokenKind::FatArrow);
            let body = self.parse_case_body()?;
            Ok(CaseAlternative {
                patterns: None,
                guard: None,
                body,
            })
        } else {
            Err(self.error("expected 'when' or 'otherwise'"))
        }
    }

    /// Parse case body (either single statement(s) on same line or indented block).
    fn parse_case_body(&mut self) -> Result<Vec<Stmt>> {
        if self.check(&TokenKind::Indent) {
            self.parse_stmt_block()
        } else {
            // Parse one or more statements on the same line
            let mut stmts = Vec::new();
            loop {
                // Check for end of case BEFORE trying to parse
                if self.check(&TokenKind::When)
                    || self.check(&TokenKind::Otherwise)
                    || self.check(&TokenKind::Dedent)
                    || self.check(&TokenKind::Indent)
                    || self.at_eof()
                {
                    break;
                }
                stmts.push(self.parse_stmt()?);
            }
            Ok(stmts)
        }
    }

    /// Parse a case pattern.
    fn parse_case_pattern(&mut self) -> Result<CasePattern> {
        // Handle negation
        if self.eat(&TokenKind::Minus).is_some() {
            let inner = self.parse_case_pattern()?;
            return Ok(CasePattern::Neg(Box::new(inner)));
        }

        match self.current_kind().clone() {
            TokenKind::NatLit(n) => {
                self.advance();
                let first = CasePattern::Int(n as i128);
                self.maybe_parse_pattern_range(first)
            }
            TokenKind::HexLit(s) => {
                self.advance();
                let (value, bits) = self.parse_hex_literal(&s)?;
                let first = if let Some(v) = value {
                    CasePattern::Hex(v)
                } else {
                    CasePattern::Bits(bits)
                };
                self.maybe_parse_pattern_range(first)
            }
            TokenKind::BinLit(s) => {
                self.advance();
                let bits = self.parse_bitvector(&s)?;
                Ok(CasePattern::Bits(bits))
            }
            TokenKind::MaskLit(s) => {
                self.advance();
                let mask = self.parse_mask(&s)?;
                Ok(CasePattern::Mask(mask))
            }
            TokenKind::Ident(name) => {
                self.advance();
                let first = CasePattern::Ident(SmolStr::new(&name));
                self.maybe_parse_pattern_range(first)
            }
            TokenKind::LParen => {
                self.advance();
                let patterns = self.parse_list1(|p| p.parse_case_pattern())?;
                self.expect(&TokenKind::RParen)?;
                Ok(CasePattern::Tuple(patterns))
            }
            _ => Err(self.error("expected case pattern")),
        }
    }

    /// Maybe parse a range pattern.
    fn maybe_parse_pattern_range(&mut self, first: CasePattern) -> Result<CasePattern> {
        if self.eat(&TokenKind::DotDot).is_some() {
            let second = self.parse_case_pattern()?;
            Ok(CasePattern::Range {
                lo: Box::new(first),
                hi: Box::new(second),
            })
        } else {
            Ok(first)
        }
    }

    /// Parse a for loop.
    fn parse_for_stmt(&mut self) -> Result<Stmt> {
        self.expect(&TokenKind::For)?;
        let var = self.expect_ident()?;
        self.expect(&TokenKind::Eq)?;
        let start = self.parse_expr()?;

        let ascending = if self.eat(&TokenKind::To).is_some() {
            true
        } else if self.eat(&TokenKind::Downto).is_some() {
            false
        } else {
            return Err(self.error("expected 'to' or 'downto'"));
        };

        let end = self.parse_expr()?;

        let body = self.parse_stmt_or_block()?;

        Ok(Stmt::For {
            var,
            start,
            ascending,
            end,
            body,
        })
    }

    /// Parse a while loop.
    fn parse_while_stmt(&mut self) -> Result<Stmt> {
        self.expect(&TokenKind::While)?;
        let cond = self.parse_expr()?;
        self.expect(&TokenKind::Do)?;
        let body = self.parse_stmt_or_block()?;

        Ok(Stmt::While { cond, body })
    }

    /// Parse a repeat loop.
    fn parse_repeat_stmt(&mut self) -> Result<Stmt> {
        self.expect(&TokenKind::Repeat)?;
        let body = self.parse_stmt_block()?;
        self.expect(&TokenKind::Until)?;
        let cond = self.parse_expr()?;
        self.eat(&TokenKind::Semi);

        Ok(Stmt::Repeat { body, cond })
    }

    /// Parse a try-catch statement.
    fn parse_try_stmt(&mut self) -> Result<Stmt> {
        self.expect(&TokenKind::Try)?;
        let body = self.parse_stmt_block()?;
        self.expect(&TokenKind::Catch)?;
        let exc_var = self.expect_ident()?;

        let catches = self.parse_indented_block(|p| p.parse_catch_alternative())?;

        Ok(Stmt::Try {
            body,
            exc_var,
            catches,
        })
    }

    /// Parse a catch alternative.
    fn parse_catch_alternative(&mut self) -> Result<CatchAlternative> {
        if self.eat(&TokenKind::When).is_some() {
            let guard = self.parse_expr()?;
            // Fat arrow is optional
            self.eat(&TokenKind::FatArrow);
            let body = self.parse_case_body()?;
            Ok(CatchAlternative {
                guard: Some(guard),
                body,
            })
        } else if self.eat(&TokenKind::Otherwise).is_some() {
            // Fat arrow is optional
            self.eat(&TokenKind::FatArrow);
            let body = self.parse_case_body()?;
            Ok(CatchAlternative { guard: None, body })
        } else {
            Err(self.error("expected 'when' or 'otherwise'"))
        }
    }

    /// Check if the current position looks like the start of a type.
    pub fn is_type_start(&self) -> bool {
        match self.current_kind() {
            TokenKind::Ident(name) => self.type_idents.contains(name.as_str()),
            TokenKind::LParen => true,
            TokenKind::Array => true,
            TokenKind::Register => true,
            TokenKind::Typeof => true,
            TokenKind::AArch32 | TokenKind::AArch64 => {
                // Architecture qualifiers are more commonly function/variable prefixes
                // than type prefixes. Only treat as type start if we know for sure.
                // For now, default to false - parse as expression first.
                false
            }
            _ => false,
        }
    }
}
