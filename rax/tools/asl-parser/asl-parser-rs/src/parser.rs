//! Recursive descent parser for ASL.
//!
//! This parser handles the complete ASL grammar including expressions,
//! statements, type definitions, function definitions, instructions,
//! and register declarations.

mod defs;
mod expr;
mod instr;
mod regs;
mod stmt;
mod types;

use crate::error::{ParseError, Result};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::span::Span;
use crate::syntax::*;
use smol_str::SmolStr;
use std::collections::HashSet;

// ============================================================================
// Public API
// ============================================================================

/// Parse ASL definitions from source code.
pub fn parse_definitions(source: &str) -> Result<Vec<Definition>> {
    let mut parser = Parser::new(source);
    parser.parse_definitions()
}

/// Parse ASL instructions from source code.
pub fn parse_instructions(source: &str) -> Result<Vec<Instruction>> {
    let mut parser = Parser::new(source);
    parser.parse_instructions()
}

/// Parse ASL register definitions from source code.
pub fn parse_registers(source: &str) -> Result<Vec<RegisterDefinition>> {
    let mut parser = Parser::new(source);
    parser.parse_registers()
}

// ============================================================================
// Parser
// ============================================================================

/// The ASL parser.
pub struct Parser<'src> {
    /// The lexer
    lexer: Lexer<'src>,
    /// The current token
    current: Token,
    /// The previous token's span (for error recovery)
    prev_span: Span,
    /// Set of known type identifiers (for disambiguation)
    type_idents: HashSet<SmolStr>,
}

impl<'src> Parser<'src> {
    /// Create a new parser for the given source.
    pub fn new(source: &'src str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token();

        // Initialize with built-in type names
        let mut type_idents = HashSet::new();
        for name in &[
            "integer", "boolean", "real", "string", "bit", "bits", "signal", "__RAM", "__NOP",
        ] {
            type_idents.insert(SmolStr::new(name));
        }

        Self {
            lexer,
            current,
            prev_span: Span::new(0, 0),
            type_idents,
        }
    }

    // ========================================================================
    // Token handling
    // ========================================================================

    /// Get the current token kind.
    fn current_kind(&self) -> &TokenKind {
        &self.current.kind
    }

    /// Get the current token span.
    fn current_span(&self) -> Span {
        self.current.span
    }

    /// Advance to the next token.
    fn advance(&mut self) {
        self.prev_span = self.current.span;
        self.current = self.lexer.next_token();
    }

    /// Check if the current token matches the expected kind.
    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(self.current_kind()) == std::mem::discriminant(kind)
    }

    /// Check if the current token is an identifier with a specific name.
    #[allow(dead_code)]
    fn check_ident(&self, name: &str) -> bool {
        matches!(self.current_kind(), TokenKind::Ident(s) if s == name)
    }

    /// Check if we're at end of file.
    fn at_eof(&self) -> bool {
        matches!(self.current_kind(), TokenKind::Eof)
    }

    /// Consume the current token if it matches, return error otherwise.
    fn expect(&mut self, kind: &TokenKind) -> Result<Token> {
        if self.check(kind) {
            let tok = self.current.clone();
            self.advance();
            Ok(tok)
        } else if self.at_eof() {
            Err(ParseError::unexpected_eof(
                &[kind.clone()],
                self.current_span(),
            ))
        } else {
            Err(ParseError::unexpected(
                self.current_kind().clone(),
                &[kind.clone()],
                self.current_span(),
            ))
        }
    }

    /// Consume the current token if it matches, return None otherwise.
    fn eat(&mut self, kind: &TokenKind) -> Option<Token> {
        if self.check(kind) {
            let tok = self.current.clone();
            self.advance();
            Some(tok)
        } else {
            None
        }
    }

    /// Expect an identifier token.
    fn expect_ident(&mut self) -> Result<Identifier> {
        match self.current_kind().clone() {
            TokenKind::Ident(name) => {
                self.advance();
                Ok(SmolStr::new(&name))
            }
            _ if self.at_eof() => Err(ParseError::unexpected_eof(
                &[TokenKind::Ident(String::new())],
                self.current_span(),
            )),
            _ => Err(ParseError::unexpected(
                self.current_kind().clone(),
                &[TokenKind::Ident(String::new())],
                self.current_span(),
            )),
        }
    }

    /// Get the identifier if current token is one.
    fn get_ident(&self) -> Option<&str> {
        match self.current_kind() {
            TokenKind::Ident(name) => Some(name),
            _ => None,
        }
    }

    /// Parse a dotted name (e.g., `LDNT1D_Z.P.BR_Contiguous` or `LDFF1D_Z.P.BZ_D.64.scaled`).
    fn parse_dotted_name(&mut self) -> Result<Identifier> {
        // First part must be an identifier
        let first = self.expect_ident()?;
        let mut name = first.to_string();

        // Subsequent parts can be identifiers or numbers
        while self.eat(&TokenKind::Dot).is_some() {
            match self.current_kind().clone() {
                TokenKind::Ident(s) => {
                    self.advance();
                    name.push('.');
                    name.push_str(&s);
                }
                TokenKind::NatLit(n) => {
                    self.advance();
                    name.push('.');
                    name.push_str(&n.to_string());
                }
                _ => {
                    return Err(
                        self.error("expected identifier or number after '.' in dotted name")
                    );
                }
            }
        }
        Ok(SmolStr::new(&name))
    }

    /// Check if current identifier is a known type.
    fn is_type_ident(&self) -> bool {
        match self.current_kind() {
            TokenKind::Ident(name) => self.type_idents.contains(name.as_str()),
            _ => false,
        }
    }

    /// Register a new type identifier.
    fn add_type_ident(&mut self, name: &str) {
        self.type_idents.insert(SmolStr::new(name));
    }

    // ========================================================================
    // Helper parsing methods
    // ========================================================================

    /// Parse a qualified identifier (e.g., `AArch64.Foo` or just `Foo`).
    fn parse_qualified_ident(&mut self) -> Result<QualifiedIdentifier> {
        let qualifier = match self.current_kind() {
            TokenKind::AArch32 => {
                self.advance();
                self.expect(&TokenKind::Dot)?;
                ArchQualifier::AArch32
            }
            TokenKind::AArch64 => {
                self.advance();
                self.expect(&TokenKind::Dot)?;
                ArchQualifier::AArch64
            }
            _ => ArchQualifier::Any,
        };

        let name = self.expect_ident()?;
        Ok(QualifiedIdentifier { qualifier, name })
    }

    /// Parse a comma-separated list with at least one element.
    fn parse_list1<T, F>(&mut self, parse_elem: F) -> Result<Vec<T>>
    where
        F: Fn(&mut Self) -> Result<T>,
    {
        let mut items = vec![parse_elem(self)?];
        while self.eat(&TokenKind::Comma).is_some() {
            items.push(parse_elem(self)?);
        }
        Ok(items)
    }

    /// Parse a comma-separated list (possibly empty, within delimiters).
    fn parse_list0<T, F>(&mut self, close: &TokenKind, parse_elem: F) -> Result<Vec<T>>
    where
        F: Fn(&mut Self) -> Result<T>,
    {
        if self.check(close) {
            return Ok(Vec::new());
        }
        self.parse_list1(parse_elem)
    }

    /// Parse an indented block of items.
    fn parse_indented_block<T, F>(&mut self, parse_item: F) -> Result<Vec<T>>
    where
        F: Fn(&mut Self) -> Result<T>,
    {
        self.expect(&TokenKind::Indent)?;
        let mut items = Vec::new();
        while !self.check(&TokenKind::Dedent) && !self.at_eof() {
            items.push(parse_item(self)?);
        }
        self.expect(&TokenKind::Dedent)?;
        Ok(items)
    }

    /// Parse an optional indented block.
    fn parse_optional_indented_block<T, F>(&mut self, parse_item: F) -> Result<Vec<T>>
    where
        F: Fn(&mut Self) -> Result<T>,
    {
        if self.check(&TokenKind::Indent) {
            self.parse_indented_block(parse_item)
        } else {
            Ok(Vec::new())
        }
    }

    /// Parse a natural number literal.
    fn expect_nat(&mut self) -> Result<u128> {
        match self.current_kind().clone() {
            TokenKind::NatLit(n) => {
                self.advance();
                Ok(n)
            }
            _ => Err(ParseError::unexpected(
                self.current_kind().clone(),
                &[TokenKind::NatLit(0)],
                self.current_span(),
            )),
        }
    }

    /// Parse a string literal.
    #[allow(dead_code)]
    fn expect_string(&mut self) -> Result<SmolStr> {
        match self.current_kind().clone() {
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(SmolStr::new(&s))
            }
            _ => Err(ParseError::unexpected(
                self.current_kind().clone(),
                &[TokenKind::StringLit(String::new())],
                self.current_span(),
            )),
        }
    }

    /// Parse a binary literal.
    fn expect_bin_lit(&mut self) -> Result<SmolStr> {
        match self.current_kind().clone() {
            TokenKind::BinLit(s) => {
                self.advance();
                // Strip quotes
                let inner = s
                    .strip_prefix('\'')
                    .and_then(|s| s.strip_suffix('\''))
                    .unwrap_or(&s);
                Ok(SmolStr::new(inner))
            }
            _ => Err(ParseError::unexpected(
                self.current_kind().clone(),
                &[TokenKind::BinLit(String::new())],
                self.current_span(),
            )),
        }
    }

    /// Parse a mask literal.
    fn expect_mask_lit(&mut self) -> Result<SmolStr> {
        match self.current_kind().clone() {
            TokenKind::MaskLit(s) => {
                self.advance();
                // Strip quotes
                let inner = s
                    .strip_prefix('\'')
                    .and_then(|s| s.strip_suffix('\''))
                    .unwrap_or(&s);
                Ok(SmolStr::new(inner))
            }
            _ => Err(ParseError::unexpected(
                self.current_kind().clone(),
                &[TokenKind::MaskLit(String::new())],
                self.current_span(),
            )),
        }
    }

    /// Parse a binary literal into a bit vector.
    fn parse_bitvector(&self, s: &str) -> Result<BitVector> {
        let mut bits = Vec::new();
        for c in s.chars() {
            match c {
                '0' => bits.push(false),
                '1' => bits.push(true),
                ' ' | '\'' => {} // Skip spaces and quotes
                _ => {
                    return Err(ParseError::invalid_literal(
                        format!("invalid character in binary literal: '{c}'"),
                        self.current_span(),
                    ));
                }
            }
        }
        Ok(bits)
    }

    /// Parse a hex literal string into either an integer (if small enough) or a bitvector.
    /// Returns (value_if_fits_i128, bitvector)
    fn parse_hex_literal(&self, s: &str) -> Result<(Option<i128>, BitVector)> {
        // Strip the 0x prefix
        let hex_str = s.strip_prefix("0x").unwrap_or(s);

        // Convert each hex digit to 4 bits
        let mut bits = Vec::new();
        for c in hex_str.chars() {
            let nibble = match c.to_ascii_lowercase() {
                '0' => [false, false, false, false],
                '1' => [false, false, false, true],
                '2' => [false, false, true, false],
                '3' => [false, false, true, true],
                '4' => [false, true, false, false],
                '5' => [false, true, false, true],
                '6' => [false, true, true, false],
                '7' => [false, true, true, true],
                '8' => [true, false, false, false],
                '9' => [true, false, false, true],
                'a' => [true, false, true, false],
                'b' => [true, false, true, true],
                'c' => [true, true, false, false],
                'd' => [true, true, false, true],
                'e' => [true, true, true, false],
                'f' => [true, true, true, true],
                _ => {
                    return Err(ParseError::invalid_literal(
                        format!("invalid hex character: '{c}'"),
                        self.current_span(),
                    ))
                }
            };
            bits.extend(nibble);
        }

        // Try to parse as i128 if small enough
        let value = if bits.len() <= 128 {
            // Try to convert to i128
            i128::from_str_radix(hex_str, 16).ok()
        } else {
            None
        };

        Ok((value, bits))
    }

    /// Parse a mask literal.
    fn parse_mask(&self, s: &str) -> Result<Mask> {
        let mut bits = Vec::new();
        for c in s.chars() {
            match c {
                '0' => bits.push(MaskBit::Zero),
                '1' => bits.push(MaskBit::One),
                'x' | 'X' => bits.push(MaskBit::Either),
                ' ' | '\'' => {} // Skip spaces and quotes
                _ => {
                    return Err(ParseError::invalid_literal(
                        format!("invalid character in mask literal: '{c}'"),
                        self.current_span(),
                    ));
                }
            }
        }
        Ok(bits)
    }

    /// Create an error for the current position.
    fn error(&self, msg: impl Into<String>) -> ParseError {
        ParseError::custom(msg, self.current_span())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualified_function_call() {
        let source = "bits(32) test()\n    AArch64.Foo();";
        let result = parse_definitions(source);
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }
}
