//! Register definition parsing for ASL.

use super::Parser;
use crate::error::Result;
use crate::lexer::TokenKind;
use crate::syntax::*;

impl<'src> Parser<'src> {
    /// Parse a list of register definitions.
    pub fn parse_registers(&mut self) -> Result<Vec<RegisterDefinition>> {
        let mut regs = Vec::new();

        while !self.at_eof() {
            regs.push(self.parse_register_definition()?);
        }

        Ok(regs)
    }

    /// Parse a single register definition.
    pub fn parse_register_definition(&mut self) -> Result<RegisterDefinition> {
        // Check for array
        if self.check(&TokenKind::Array) {
            return self.parse_register_array();
        }

        self.parse_single_register()
    }

    /// Parse a single register (for use in definitions mode).
    pub fn parse_register_definition_inline(&mut self) -> Result<RegisterDefinition> {
        // Check for array
        if self.check(&TokenKind::Array) {
            return self.parse_register_array();
        }

        self.parse_single_register()
    }

    /// Parse just the register type (width and fields, without name).
    /// Returns (width, fields).
    pub fn parse_register_type_only(&mut self) -> Result<(u32, Vec<RegisterField>)> {
        self.expect(&TokenKind::Register)?;
        let width = self.expect_nat()? as u32;

        // Parse fields
        let fields = if self.eat(&TokenKind::LBrace).is_some() {
            let fields = self.parse_list0(&TokenKind::RBrace, |p| p.parse_register_field())?;
            self.expect(&TokenKind::RBrace)?;
            fields
        } else {
            Vec::new()
        };

        Ok((width, fields))
    }

    /// Parse a single register definition.
    fn parse_single_register(&mut self) -> Result<RegisterDefinition> {
        let (width, fields) = self.parse_register_type_only()?;

        let name = self.expect_ident()?;
        self.eat(&TokenKind::Semi);

        Ok(RegisterDefinition::Single(Register {
            name,
            width,
            fields,
        }))
    }

    /// Parse a register array definition.
    fn parse_register_array(&mut self) -> Result<RegisterDefinition> {
        self.expect(&TokenKind::Array)?;
        self.expect(&TokenKind::LBracket)?;

        let index_min = self.expect_nat()? as i64;
        self.expect(&TokenKind::DotDot)?;
        let index_max = self.expect_nat()? as i64;

        self.expect(&TokenKind::RBracket)?;
        self.expect(&TokenKind::Of)?;

        self.expect(&TokenKind::Register)?;
        let width = self.expect_nat()? as u32;

        // Parse fields
        let fields = if self.eat(&TokenKind::LBrace).is_some() {
            let fields = self.parse_list0(&TokenKind::RBrace, |p| p.parse_register_field())?;
            self.expect(&TokenKind::RBrace)?;
            fields
        } else {
            Vec::new()
        };

        let name = self.expect_ident()?;
        self.eat(&TokenKind::Semi);

        Ok(RegisterDefinition::Array(RegisterArray {
            index_min,
            index_max,
            register: Register {
                name,
                width,
                fields,
            },
        }))
    }

    /// Parse a register field.
    ///
    /// Supports formats:
    /// - `hi:lo name` - range format
    /// - `base+:count name` - offset format
    /// - `hi:lo` or `base+:count` - unnamed fields
    fn parse_register_field(&mut self) -> Result<RegisterField> {
        let first = self.expect_nat()? as u32;

        let (lo, hi) = if self.eat(&TokenKind::Colon).is_some() {
            // Range format: hi:lo
            let second = self.expect_nat()? as u32;
            (second, first)
        } else if self.eat(&TokenKind::PlusColon).is_some() {
            // Offset format: base+:count
            let count = self.expect_nat()? as u32;
            let base = first;
            (base, base + count - 1)
        } else {
            // Single bit
            (first, first)
        };

        // Optional field name
        let name = if let TokenKind::Ident(s) = self.current_kind() {
            let s = s.clone();
            self.advance();
            Some(s.into())
        } else {
            None
        };

        Ok(RegisterField { name, lo, hi })
    }
}
