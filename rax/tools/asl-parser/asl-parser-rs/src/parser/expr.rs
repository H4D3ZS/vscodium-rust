//! Expression parsing for ASL.

use super::Parser;
use crate::error::Result;
use crate::lexer::TokenKind;
use crate::syntax::*;
use smol_str::SmolStr;

impl<'src> Parser<'src> {
    /// Parse an expression.
    pub fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_expr_prec(0)
    }

    /// Parse an expression with minimum precedence (Pratt parser).
    fn parse_expr_prec(&mut self, min_prec: u8) -> Result<Expr> {
        let mut lhs = self.parse_unary_expr()?;

        loop {
            let Some(op) = self.peek_binop() else {
                break;
            };

            let op_prec = op.precedence();
            if op_prec < min_prec {
                break;
            }

            // Consume the operator
            self.advance();

            // Handle right-associativity
            let next_prec = if op.is_right_assoc() {
                op_prec
            } else {
                op_prec + 1
            };

            let rhs = self.parse_expr_prec(next_prec)?;
            lhs = Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    /// Peek at the current token to see if it's a binary operator.
    ///
    /// Note: `+:` (PlusColon/SliceOffset) is NOT a general binary operator.
    /// It's only used in slice syntax like `x[base+:count]` and is handled
    /// separately in slice parsing.
    fn peek_binop(&self) -> Option<BinOp> {
        match self.current_kind() {
            TokenKind::OrOr => Some(BinOp::Or),
            TokenKind::AndAnd => Some(BinOp::And),
            TokenKind::EqEq => Some(BinOp::Eq),
            TokenKind::Ne => Some(BinOp::Ne),
            TokenKind::Lt => Some(BinOp::Lt),
            TokenKind::Le => Some(BinOp::Le),
            TokenKind::Gt => Some(BinOp::Gt),
            TokenKind::Ge => Some(BinOp::Ge),
            TokenKind::Colon => Some(BinOp::BitConcat),
            TokenKind::PlusPlus => Some(BinOp::Concat),
            TokenKind::Or => Some(BinOp::BitOr),
            TokenKind::Eor => Some(BinOp::BitXor),
            TokenKind::And => Some(BinOp::BitAnd),
            TokenKind::Quot => Some(BinOp::Quot),
            TokenKind::Rem => Some(BinOp::Rem),
            TokenKind::Div => Some(BinOp::IntDiv),
            TokenKind::Mod => Some(BinOp::Mod),
            TokenKind::Shl => Some(BinOp::Shl),
            TokenKind::Shr => Some(BinOp::Shr),
            TokenKind::Plus => Some(BinOp::Add),
            TokenKind::Minus => Some(BinOp::Sub),
            TokenKind::Star => Some(BinOp::Mul),
            TokenKind::Slash => Some(BinOp::Div),
            TokenKind::Caret => Some(BinOp::Pow),
            _ => None,
        }
    }

    /// Parse a unary expression.
    fn parse_unary_expr(&mut self) -> Result<Expr> {
        match self.current_kind() {
            TokenKind::Minus => {
                self.advance();
                // Check if this is a discard pattern (minus followed by comma or rparen)
                // This only makes sense in tuple contexts like (a, -)
                if matches!(self.current_kind(), TokenKind::Comma | TokenKind::RParen) {
                    return Ok(Expr::Discard);
                }
                let operand = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnOp::Neg,
                    operand: Box::new(operand),
                })
            }
            TokenKind::Bang | TokenKind::Not => {
                self.advance();
                let operand = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnOp::Not,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_postfix_expr(),
        }
    }

    /// Parse a postfix expression (member access, indexing, slicing).
    fn parse_postfix_expr(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary_expr()?;

        loop {
            match self.current_kind() {
                // Member access: expr.field or expr.[f1, f2] or expr.<f1, f2>
                TokenKind::Dot => {
                    self.advance();
                    if self.check(&TokenKind::LBracket) {
                        // Multiple member access: expr.[f1, f2]
                        self.advance();
                        let fields = self.parse_list1(|p| p.expect_ident())?;
                        self.expect(&TokenKind::RBracket)?;
                        expr = Expr::Members {
                            base: Box::new(expr),
                            fields,
                        };
                    } else if self.check(&TokenKind::Lt) {
                        // Multiple member bits access: expr.<f1, f2>
                        self.advance();
                        let fields = self.parse_list1(|p| p.expect_ident())?;
                        self.expect(&TokenKind::Gt)?;
                        expr = Expr::MemberBits {
                            base: Box::new(expr),
                            fields,
                        };
                    } else {
                        let field = self.expect_ident()?;
                        expr = Expr::Member {
                            base: Box::new(expr),
                            field,
                        };
                    }
                }

                // Array indexing or slicing: expr[...] or expr[]
                TokenKind::LBracket => {
                    self.advance();

                    // Handle empty brackets (getter call)
                    let slices = if self.check(&TokenKind::RBracket) {
                        Vec::new()
                    } else {
                        self.parse_list1(|p| p.parse_slice())?
                    };
                    self.expect(&TokenKind::RBracket)?;

                    // Determine if this is indexing or slicing
                    let is_slice = slices.iter().any(|s| !matches!(s, Slice::Single(_)));
                    if is_slice {
                        expr = Expr::Slice {
                            base: Box::new(expr),
                            slices,
                        };
                    } else {
                        expr = Expr::Index {
                            base: Box::new(expr),
                            indices: slices,
                        };
                    }
                }

                // Bit slice: expr<slices>
                // This is tricky because < can also be a comparison operator.
                //
                // Strategy: Try to parse as slice. If we parse content and find
                // that it contains : or +: (Range or Offset slices), it's definitely
                // a slice. If all elements are Single and we see >, it's still a slice.
                // If all elements are Single and we DON'T see >, it was actually
                // a comparison and we need to handle that.
                TokenKind::Lt => {
                    self.advance(); // consume <

                    // Parse the slice content
                    let slices = self.parse_list1(|p| p.parse_inner_slice())?;

                    // Check if any slice has : or +: (Range or Offset)
                    let has_range_or_offset = slices
                        .iter()
                        .any(|s| matches!(s, Slice::Range { .. } | Slice::Offset { .. }));

                    if self.eat(&TokenKind::Gt).is_some() {
                        // Successfully closed with > - it's a slice
                        let is_single_access =
                            slices.len() == 1 && matches!(&slices[0], Slice::Single(_));
                        if is_single_access {
                            expr = Expr::Index {
                                base: Box::new(expr),
                                indices: slices,
                            };
                        } else {
                            expr = Expr::Slice {
                                base: Box::new(expr),
                                slices,
                            };
                        }
                    } else if has_range_or_offset {
                        // Has : or +: but no > - syntax error
                        return Err(self.error("expected '>' to close slice"));
                    } else {
                        // All Single slices and no > - this was actually a comparison
                        // We need to reconstruct: expr < first_element ...
                        // The first_element is in slices[0] as Single
                        // Convert back to comparison expression
                        if slices.len() == 1 {
                            if let Slice::Single(inner) = slices.into_iter().next().unwrap() {
                                // Reconstruct: expr < inner
                                expr = Expr::Binary {
                                    op: BinOp::Lt,
                                    lhs: Box::new(expr),
                                    rhs: inner,
                                };
                                // Now continue with the Pratt parser for the rest
                                // The next token should be || or similar
                                // Continue the postfix loop to pick up any remaining operators
                                continue;
                            }
                        }
                        return Err(self.error("ambiguous < - could be slice or comparison"));
                    }
                }

                // Function call: expr(args)
                TokenKind::LParen if self.is_callable_context(&expr) => {
                    self.advance();
                    let args = self.parse_list0(&TokenKind::RParen, |p| p.parse_expr())?;
                    self.expect(&TokenKind::RParen)?;

                    // Convert to a call expression
                    if let Expr::Var(name) = expr {
                        expr = Expr::Call { name, args };
                    } else {
                        return Err(self.error("expected function name before '('"));
                    }
                }

                _ => break,
            }
        }

        // Handle IN operator for set membership
        if self.eat(&TokenKind::In).is_some() {
            return self.parse_in_expr(expr);
        }

        Ok(expr)
    }

    /// Check if an expression can be followed by a call.
    fn is_callable_context(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Var(_))
    }

    /// Parse a slice specification.
    pub fn parse_slice(&mut self) -> Result<Slice> {
        let first = self.parse_slice_expr()?;
        self.continue_parse_slice(first)
    }

    /// Continue parsing a slice after the first expression.
    fn continue_parse_slice(&mut self, first: Expr) -> Result<Slice> {
        if self.eat(&TokenKind::Colon).is_some() {
            // Range slice: hi:lo
            let lo = self.parse_slice_expr()?;
            Ok(Slice::Range {
                hi: Box::new(first),
                lo: Box::new(lo),
            })
        } else if self.eat(&TokenKind::PlusColon).is_some() {
            // Offset slice: base+:count
            let count = self.parse_slice_expr()?;
            Ok(Slice::Offset {
                base: Box::new(first),
                count: Box::new(count),
            })
        } else {
            // Single index
            Ok(Slice::Single(Box::new(first)))
        }
    }

    /// Parse a slice expression (restricted to avoid ambiguity with ':').
    fn parse_slice_expr(&mut self) -> Result<Expr> {
        // Parse a simpler expression that doesn't include ':'
        let mut expr = self.parse_slice_unary()?;

        loop {
            match self.current_kind() {
                TokenKind::Plus => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::Add, expr, rhs);
                }
                TokenKind::Minus => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::Sub, expr, rhs);
                }
                TokenKind::Star => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::Mul, expr, rhs);
                }
                TokenKind::Slash => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::Div, expr, rhs);
                }
                TokenKind::Caret => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::Pow, expr, rhs);
                }
                TokenKind::Div => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::IntDiv, expr, rhs);
                }
                TokenKind::Mod => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::Mod, expr, rhs);
                }
                TokenKind::Shl => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::Shl, expr, rhs);
                }
                TokenKind::Shr => {
                    self.advance();
                    let rhs = self.parse_slice_unary()?;
                    expr = Expr::binary(BinOp::Shr, expr, rhs);
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse a unary slice expression.
    fn parse_slice_unary(&mut self) -> Result<Expr> {
        if self.eat(&TokenKind::Minus).is_some() {
            let operand = self.parse_slice_primary()?;
            Ok(Expr::Unary {
                op: UnOp::Neg,
                operand: Box::new(operand),
            })
        } else if self.eat(&TokenKind::Bang).is_some() {
            let operand = self.parse_slice_primary()?;
            Ok(Expr::Unary {
                op: UnOp::Not,
                operand: Box::new(operand),
            })
        } else {
            self.parse_slice_primary()
        }
    }

    /// Parse a primary slice expression.
    fn parse_slice_primary(&mut self) -> Result<Expr> {
        let mut expr = match self.current_kind().clone() {
            TokenKind::NatLit(n) => {
                self.advance();
                Expr::LitInt(n as i128)
            }
            TokenKind::HexLit(s) => {
                self.advance();
                let (value, bits) = self.parse_hex_literal(&s)?;
                if let Some(v) = value {
                    Expr::LitInt(v)
                } else {
                    Expr::LitBits(bits)
                }
            }
            TokenKind::Ident(_) | TokenKind::AArch32 | TokenKind::AArch64 => {
                let qid = self.parse_qualified_ident()?;
                let mut result = Expr::Var(qid.clone());

                // Handle member access: expr.field
                while self.eat(&TokenKind::Dot).is_some() {
                    let field = self.expect_ident()?;
                    result = Expr::Member {
                        base: Box::new(result),
                        field,
                    };
                }

                // Check for function call
                if self.eat(&TokenKind::LParen).is_some() {
                    let args = self.parse_list0(&TokenKind::RParen, |p| p.parse_expr())?;
                    self.expect(&TokenKind::RParen)?;
                    Expr::Call { name: qid, args }
                } else {
                    result
                }
            }
            TokenKind::LParen => {
                self.advance();
                let inner = self.parse_slice_expr()?;
                self.expect(&TokenKind::RParen)?;
                inner
            }
            // If expression (for conditional indices)
            TokenKind::If => self.parse_if_expr()?,
            _ => return Err(self.error("expected slice expression")),
        };

        // Handle postfix operations on the primary expression (indexing/slicing)
        while self.check(&TokenKind::LBracket) {
            self.advance();
            if self.check(&TokenKind::RBracket) {
                // Empty brackets - getter call with no args
                self.advance();
                expr = Expr::Index {
                    base: Box::new(expr),
                    indices: Vec::new(),
                };
            } else {
                let slices = self.parse_list1(|p| p.parse_inner_slice())?;
                self.expect(&TokenKind::RBracket)?;

                // Determine if this is indexing or slicing
                let is_slice = slices.iter().any(|s| !matches!(s, Slice::Single(_)));
                if is_slice {
                    expr = Expr::Slice {
                        base: Box::new(expr),
                        slices,
                    };
                } else {
                    expr = Expr::Index {
                        base: Box::new(expr),
                        indices: slices,
                    };
                }
            }
        }

        Ok(expr)
    }

    /// Parse an inner slice (used inside [...] in slice context).
    /// This handles simpler expressions without full postfix to avoid infinite recursion.
    fn parse_inner_slice(&mut self) -> Result<Slice> {
        let first = self.parse_inner_slice_expr()?;

        if self.eat(&TokenKind::Colon).is_some() {
            // Range slice: hi:lo
            let lo = self.parse_inner_slice_expr()?;
            Ok(Slice::Range {
                hi: Box::new(first),
                lo: Box::new(lo),
            })
        } else if self.eat(&TokenKind::PlusColon).is_some() {
            // Offset slice: base+:count
            let count = self.parse_inner_slice_expr()?;
            Ok(Slice::Offset {
                base: Box::new(first),
                count: Box::new(count),
            })
        } else {
            Ok(Slice::Single(Box::new(first)))
        }
    }

    /// Parse a simple expression inside a slice (no postfix operators).
    fn parse_inner_slice_expr(&mut self) -> Result<Expr> {
        let mut expr = self.parse_inner_slice_unary()?;

        loop {
            match self.current_kind() {
                TokenKind::Plus => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::Add, expr, rhs);
                }
                TokenKind::Minus => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::Sub, expr, rhs);
                }
                TokenKind::Star => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::Mul, expr, rhs);
                }
                TokenKind::Caret => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::Pow, expr, rhs);
                }
                TokenKind::Slash => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::Div, expr, rhs);
                }
                TokenKind::Div => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::IntDiv, expr, rhs);
                }
                TokenKind::Mod => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::Mod, expr, rhs);
                }
                TokenKind::Shl => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::Shl, expr, rhs);
                }
                TokenKind::Shr => {
                    self.advance();
                    let rhs = self.parse_inner_slice_unary()?;
                    expr = Expr::binary(BinOp::Shr, expr, rhs);
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse a unary expression in inner slice context.
    fn parse_inner_slice_unary(&mut self) -> Result<Expr> {
        if self.eat(&TokenKind::Minus).is_some() {
            let operand = self.parse_inner_slice_primary()?;
            Ok(Expr::Unary {
                op: UnOp::Neg,
                operand: Box::new(operand),
            })
        } else {
            self.parse_inner_slice_primary()
        }
    }

    /// Parse a primary expression in inner slice context (with member access).
    fn parse_inner_slice_primary(&mut self) -> Result<Expr> {
        let mut expr = match self.current_kind().clone() {
            TokenKind::NatLit(n) => {
                self.advance();
                Expr::LitInt(n as i128)
            }
            TokenKind::HexLit(s) => {
                self.advance();
                let (value, bits) = self.parse_hex_literal(&s)?;
                if let Some(v) = value {
                    Expr::LitInt(v)
                } else {
                    Expr::LitBits(bits)
                }
            }
            TokenKind::RealLit(s) => {
                self.advance();
                let parts: Vec<&str> = s.split('.').collect();
                let integer: i128 = parts[0].parse().unwrap_or(0);
                let fraction: u64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                Expr::LitReal { integer, fraction }
            }
            TokenKind::Ident(_) | TokenKind::AArch32 | TokenKind::AArch64 => {
                let qid = self.parse_qualified_ident()?;
                // Check for function call
                if self.eat(&TokenKind::LParen).is_some() {
                    let args = self.parse_list0(&TokenKind::RParen, |p| p.parse_expr())?;
                    self.expect(&TokenKind::RParen)?;
                    Expr::Call { name: qid, args }
                } else {
                    Expr::Var(qid)
                }
            }
            TokenKind::LParen => {
                self.advance();
                let inner = self.parse_inner_slice_expr()?;
                self.expect(&TokenKind::RParen)?;
                inner
            }
            // If expression (for conditional slices)
            TokenKind::If => self.parse_if_expr()?,
            _ => return Err(self.error("expected expression in slice")),
        };

        // Handle member access: expr.field
        while self.eat(&TokenKind::Dot).is_some() {
            let field = self.expect_ident()?;
            expr = Expr::Member {
                base: Box::new(expr),
                field,
            };
        }

        Ok(expr)
    }

    /// Parse an IN expression (set membership or mask match).
    fn parse_in_expr(&mut self, expr: Expr) -> Result<Expr> {
        match self.current_kind().clone() {
            TokenKind::MaskLit(s) | TokenKind::BinLit(s) => {
                self.advance();
                let mask = self.parse_mask(&s)?;
                Ok(Expr::InMask {
                    expr: Box::new(expr),
                    mask,
                })
            }
            TokenKind::LBrace => {
                self.advance();
                let elements = self.parse_list1(|p| p.parse_set_element())?;
                self.expect(&TokenKind::RBrace)?;
                Ok(Expr::InSet {
                    expr: Box::new(expr),
                    elements,
                })
            }
            _ => Err(self.error("expected set or mask pattern after 'IN'")),
        }
    }

    /// Parse a set element (single value or range).
    fn parse_set_element(&mut self) -> Result<SetElement> {
        let first = self.parse_expr()?;
        if self.eat(&TokenKind::DotDot).is_some() {
            let second = self.parse_expr()?;
            Ok(SetElement::Range {
                lo: Box::new(first),
                hi: Box::new(second),
            })
        } else {
            Ok(SetElement::Single(Box::new(first)))
        }
    }

    /// Parse a primary expression.
    fn parse_primary_expr(&mut self) -> Result<Expr> {
        match self.current_kind().clone() {
            // Literals
            TokenKind::NatLit(n) => {
                self.advance();
                Ok(Expr::LitInt(n as i128))
            }
            TokenKind::HexLit(s) => {
                self.advance();
                let (value, bits) = self.parse_hex_literal(&s)?;
                if let Some(v) = value {
                    Ok(Expr::LitInt(v))
                } else {
                    Ok(Expr::LitBits(bits))
                }
            }
            TokenKind::RealLit(s) => {
                self.advance();
                let parts: Vec<&str> = s.split('.').collect();
                let integer: i128 = parts[0].parse().unwrap_or(0);
                let fraction: u64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                Ok(Expr::LitReal { integer, fraction })
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Expr::LitString(SmolStr::new(&s)))
            }
            TokenKind::BinLit(s) => {
                self.advance();
                let bits = self.parse_bitvector(&s)?;
                Ok(Expr::LitBits(bits))
            }
            TokenKind::MaskLit(s) => {
                self.advance();
                let mask = self.parse_mask(&s)?;
                Ok(Expr::LitMask(mask))
            }

            // Identifiers and qualified identifiers
            // Also handles type UNKNOWN and type IMPLEMENTATION_DEFINED
            TokenKind::Ident(_) | TokenKind::AArch32 | TokenKind::AArch64 => {
                // Parse as qualified identifier first
                let qid = self.parse_qualified_ident()?;

                // Check if followed by UNKNOWN - this means it's a type
                if self.eat(&TokenKind::Unknown).is_some() {
                    let ty = Type::Ref(qid);
                    return Ok(Expr::Unknown(Box::new(ty)));
                }

                // Check if followed by IMPLEMENTATION_DEFINED
                if self.eat(&TokenKind::ImplementationDefined).is_some() {
                    let desc = if let TokenKind::StringLit(s) = self.current_kind() {
                        let s = s.clone();
                        self.advance();
                        Some(SmolStr::new(&s))
                    } else {
                        None
                    };
                    let ty = Type::Ref(qid);
                    return Ok(Expr::ImpDef {
                        desc,
                        ty: Box::new(ty),
                    });
                }

                // Check if it's a parameterized type like bits(N) followed by UNKNOWN
                if self.check(&TokenKind::LParen) && self.is_param_type_name(&qid.name) {
                    self.advance(); // consume (
                    let arg = self.parse_expr()?;
                    self.expect(&TokenKind::RParen)?;

                    if self.eat(&TokenKind::Unknown).is_some() {
                        let ty = Type::Fun {
                            name: qid.name,
                            arg: Box::new(arg),
                        };
                        return Ok(Expr::Unknown(Box::new(ty)));
                    }

                    if self.eat(&TokenKind::ImplementationDefined).is_some() {
                        let desc = if let TokenKind::StringLit(s) = self.current_kind() {
                            let s = s.clone();
                            self.advance();
                            Some(SmolStr::new(&s))
                        } else {
                            None
                        };
                        let ty = Type::Fun {
                            name: qid.name,
                            arg: Box::new(arg),
                        };
                        return Ok(Expr::ImpDef {
                            desc,
                            ty: Box::new(ty),
                        });
                    }

                    // It's actually a function call like bits(32) which is a type-as-expression
                    return Ok(Expr::Call {
                        name: QualifiedIdentifier::new(qid.name),
                        args: vec![arg],
                    });
                }

                // Just a variable reference
                Ok(Expr::Var(qid))
            }

            // Parenthesized expression or tuple
            TokenKind::LParen => {
                self.advance();

                if self.check(&TokenKind::RParen) {
                    // Empty tuple
                    self.advance();
                    return Ok(Expr::Tuple(Vec::new()));
                }

                let first = self.parse_expr()?;

                if self.check(&TokenKind::Comma) {
                    // Tuple
                    self.advance();
                    let mut elements = vec![first];
                    if !self.check(&TokenKind::RParen) {
                        elements.extend(self.parse_list1(|p| p.parse_expr())?);
                    }
                    self.expect(&TokenKind::RParen)?;
                    Ok(Expr::Tuple(elements))
                } else {
                    // Parenthesized expression
                    self.expect(&TokenKind::RParen)?;
                    Ok(first)
                }
            }

            // Conditional expression
            TokenKind::If => self.parse_if_expr(),

            // Unknown value - this shouldn't be reached normally since
            // type UNKNOWN is parsed as a type followed by UNKNOWN keyword
            TokenKind::Unknown => {
                self.advance();
                // Try to create an unknown of inferred type
                Err(self.error("UNKNOWN requires a type prefix like 'integer UNKNOWN'"))
            }

            // Implementation defined
            TokenKind::ImplementationDefined => {
                self.advance();
                let _desc = if let TokenKind::StringLit(s) = self.current_kind() {
                    let s = s.clone();
                    self.advance();
                    Some(SmolStr::new(&s))
                } else {
                    None
                };
                // Type context needed
                Err(self.error("IMPLEMENTATION_DEFINED requires type context"))
            }

            // Grouped field reference: <f1, f2, ...>
            // Used in l-value contexts like (a, <f1,f2>) = expr
            TokenKind::Lt => {
                self.advance();
                let fields = self.parse_list1(|p| p.expect_ident())?;
                self.expect(&TokenKind::Gt)?;
                // Represent as a special tuple of variable references
                let exprs: Vec<Expr> = fields
                    .into_iter()
                    .map(|f| Expr::Var(QualifiedIdentifier::new(f)))
                    .collect();
                Ok(Expr::Tuple(exprs))
            }

            _ => Err(self.error(format!(
                "unexpected token in expression: {:?}",
                self.current_kind()
            ))),
        }
    }

    /// Parse an if-then-else expression.
    fn parse_if_expr(&mut self) -> Result<Expr> {
        self.expect(&TokenKind::If)?;

        let mut branches = Vec::new();

        // First condition
        let cond = self.parse_expr()?;
        self.expect(&TokenKind::Then)?;
        let then_expr = self.parse_expr()?;
        branches.push((cond, then_expr));

        // Elsif branches
        while self.eat(&TokenKind::Elsif).is_some() {
            let cond = self.parse_expr()?;
            self.expect(&TokenKind::Then)?;
            let then_expr = self.parse_expr()?;
            branches.push((cond, then_expr));
        }

        // Else branch (required in expressions)
        self.expect(&TokenKind::Else)?;
        let else_expr = self.parse_expr()?;

        Ok(Expr::If {
            branches,
            else_branch: Box::new(else_expr),
        })
    }

    /// Check if an identifier is a parameterized type name.
    fn is_param_type_name(&self, name: &str) -> bool {
        matches!(name, "bits" | "bit" | "signal" | "__RAM")
    }

    /// Parse an expression that might be a type with UNKNOWN/IMPLEMENTATION_DEFINED.
    pub fn parse_expr_or_typed_unknown(&mut self) -> Result<Expr> {
        // First try to parse a type
        if self.is_type_start() {
            let ty = self.parse_type()?;

            if self.eat(&TokenKind::Unknown).is_some() {
                return Ok(Expr::Unknown(Box::new(ty)));
            }

            if self.eat(&TokenKind::ImplementationDefined).is_some() {
                let desc = if let TokenKind::StringLit(s) = self.current_kind() {
                    let s = s.clone();
                    self.advance();
                    Some(SmolStr::new(&s))
                } else {
                    None
                };
                return Ok(Expr::ImpDef {
                    desc,
                    ty: Box::new(ty),
                });
            }

            // It wasn't UNKNOWN or IMPLEMENTATION_DEFINED, need to handle as expr
            // This is a bit tricky - for now return an error
            return Err(self.error("expected expression"));
        }

        self.parse_expr()
    }
}
