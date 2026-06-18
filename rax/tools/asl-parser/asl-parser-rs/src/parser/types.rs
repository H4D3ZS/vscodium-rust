//! Type parsing for ASL.

use super::Parser;
use crate::error::Result;
use crate::lexer::TokenKind;
use crate::syntax::*;

impl<'src> Parser<'src> {
    /// Parse a type.
    pub fn parse_type(&mut self) -> Result<Type> {
        match self.current_kind().clone() {
            // Tuple type: (Type1, Type2, ...)
            TokenKind::LParen => {
                self.advance();
                if self.check(&TokenKind::RParen) {
                    self.advance();
                    return Ok(Type::Tuple(Vec::new()));
                }

                let first = self.parse_type()?;
                if self.check(&TokenKind::Comma) {
                    self.advance();
                    let mut types = vec![first];
                    types.extend(self.parse_list1(|p| p.parse_type())?);
                    self.expect(&TokenKind::RParen)?;
                    Ok(Type::Tuple(types))
                } else {
                    self.expect(&TokenKind::RParen)?;
                    Ok(first)
                }
            }

            // typeof expression
            TokenKind::Typeof => {
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RParen)?;
                Ok(Type::Of(Box::new(expr)))
            }

            // Register type: __register N { fields }
            TokenKind::Register => {
                self.advance();
                let width = self.expect_nat()? as u64;

                let fields = if self.eat(&TokenKind::LBrace).is_some() {
                    let fields = self.parse_list0(&TokenKind::RBrace, |p| p.parse_reg_field())?;
                    self.expect(&TokenKind::RBrace)?;
                    fields
                } else {
                    Vec::new()
                };

                Ok(Type::Register { width, fields })
            }

            // Array type: array [IndexType] of ElementType
            TokenKind::Array => {
                self.advance();
                self.expect(&TokenKind::LBracket)?;
                let index = self.parse_index_type()?;
                self.expect(&TokenKind::RBracket)?;
                self.expect(&TokenKind::Of)?;
                let element = self.parse_type()?;
                Ok(Type::Array {
                    element: Box::new(element),
                    index,
                })
            }

            // Named or parameterized type
            TokenKind::Ident(_) | TokenKind::AArch32 | TokenKind::AArch64 => {
                let qid = self.parse_qualified_ident()?;

                // Check for parameterized type: bits(N), etc.
                if self.eat(&TokenKind::LParen).is_some() {
                    let arg = self.parse_expr()?;
                    self.expect(&TokenKind::RParen)?;
                    Ok(Type::Fun {
                        name: qid.name,
                        arg: Box::new(arg),
                    })
                } else {
                    Ok(Type::Ref(qid))
                }
            }

            _ => Err(self.error(format!("expected type, found {:?}", self.current_kind()))),
        }
    }

    /// Parse an index type for arrays.
    pub fn parse_index_type(&mut self) -> Result<IndexType> {
        // Could be a range (expr..expr) or an enumeration type name
        let first = self.parse_expr()?;

        if self.eat(&TokenKind::DotDot).is_some() {
            let second = self.parse_expr()?;
            Ok(IndexType::Range {
                lo: Box::new(first),
                hi: Box::new(second),
            })
        } else {
            // Must be a type name
            if let Expr::Var(qid) = first {
                Ok(IndexType::Enum(qid.name))
            } else {
                Err(self.error("expected enumeration type or range"))
            }
        }
    }

    /// Parse a register field.
    fn parse_reg_field(&mut self) -> Result<RegField> {
        let name = self.expect_ident()?;

        // Parse slices
        let mut slices = Vec::new();
        if self.eat(&TokenKind::LBracket).is_some() {
            slices = self.parse_list1(|p| p.parse_slice())?;
            self.expect(&TokenKind::RBracket)?;
        }

        Ok(RegField { name, slices })
    }

    /// Parse a symbol declaration (identifier: Type).
    pub fn parse_symbol_decl(&mut self) -> Result<SymbolDecl> {
        // In ASL, declarations are: Type name
        // But we also need to handle: name: Type in some contexts

        // Try to parse Type name format first
        let ty = self.parse_type()?;
        let name = self.expect_ident()?;

        Ok(SymbolDecl { name, ty })
    }

    /// Parse a symbol declaration in "Type name" format.
    pub fn parse_typed_name(&mut self) -> Result<SymbolDecl> {
        let ty = self.parse_type()?;
        let name = self.expect_ident()?;
        Ok(SymbolDecl { name, ty })
    }

    /// Parse a return type (which might be a tuple).
    pub fn parse_return_type(&mut self) -> Result<Vec<Type>> {
        if self.check(&TokenKind::LParen) {
            // Could be tuple type
            self.advance();
            let types = self.parse_list1(|p| p.parse_type())?;
            self.expect(&TokenKind::RParen)?;
            Ok(types)
        } else {
            let ty = self.parse_type()?;
            Ok(vec![ty])
        }
    }
}
