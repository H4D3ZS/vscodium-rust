//! Instruction parsing for ASL.

use super::Parser;
use crate::error::Result;
use crate::lexer::TokenKind;
use crate::syntax::*;

impl<'src> Parser<'src> {
    /// Parse a list of instructions.
    pub fn parse_instructions(&mut self) -> Result<Vec<Instruction>> {
        let mut instrs = Vec::new();

        while !self.at_eof() {
            instrs.push(self.parse_instruction()?);
        }

        Ok(instrs)
    }

    /// Parse a single instruction definition.
    pub fn parse_instruction(&mut self) -> Result<Instruction> {
        self.expect(&TokenKind::Instruction)?;
        // Instruction names can have dots (e.g., LDNT1D_Z.P.BR_Contiguous)
        let name = self.parse_dotted_name()?;

        // Parse instruction body (indented block)
        self.expect(&TokenKind::Indent)?;

        let mut encodings = Vec::new();
        let mut post_decode = Vec::new();
        let mut execute = Vec::new();
        let mut conditional = false;

        while !self.check(&TokenKind::Dedent) && !self.at_eof() {
            match self.current_kind().clone() {
                TokenKind::Encoding => {
                    encodings.push(self.parse_encoding()?);
                }
                TokenKind::PostDecode => {
                    self.advance();
                    // Post-decode block can be empty (just a comment)
                    post_decode = self.parse_optional_indented_block(|p| p.parse_stmt())?;
                }
                TokenKind::Execute => {
                    self.advance();
                    if self.eat(&TokenKind::Conditional).is_some() {
                        conditional = true;
                    }
                    // Execute block can be empty (just a comment)
                    execute = self.parse_optional_indented_block(|p| p.parse_stmt())?;
                }
                _ => {
                    return Err(self.error(format!(
                        "unexpected token in instruction: {:?}",
                        self.current_kind()
                    )));
                }
            }
        }

        self.expect(&TokenKind::Dedent)?;

        Ok(Instruction {
            name,
            encodings,
            post_decode,
            execute,
            conditional,
        })
    }

    /// Parse an encoding within an instruction.
    fn parse_encoding(&mut self) -> Result<InstructionEncoding> {
        self.expect(&TokenKind::Encoding)?;
        // Encoding names can have dots
        let name = self.parse_dotted_name()?;

        self.expect(&TokenKind::Indent)?;

        let mut iset = InstructionSet::A64;
        let mut fields = Vec::new();
        let mut opcode = Vec::new();
        let mut guard = None;
        let mut unpredictable = Vec::new();
        let mut decode = Vec::new();

        while !self.check(&TokenKind::Dedent) && !self.at_eof() {
            match self.current_kind().clone() {
                TokenKind::InstructionSet => {
                    self.advance();
                    iset = self.parse_instruction_set()?;
                }
                TokenKind::Field => {
                    fields.push(self.parse_instruction_field()?);
                }
                TokenKind::Opcode => {
                    self.advance();
                    opcode = self.parse_opcode_mask()?;
                }
                TokenKind::Guard => {
                    self.advance();
                    guard = Some(self.parse_expr()?);
                }
                TokenKind::UnpredictableUnless => {
                    unpredictable.push(self.parse_unpredictable_unless()?);
                }
                TokenKind::Decode => {
                    self.advance();
                    // Decode block can be empty (just a comment)
                    decode = self.parse_optional_indented_block(|p| p.parse_stmt())?;
                }
                _ => {
                    return Err(self.error(format!(
                        "unexpected token in encoding: {:?}",
                        self.current_kind()
                    )));
                }
            }
        }

        self.expect(&TokenKind::Dedent)?;

        Ok(InstructionEncoding {
            name,
            iset,
            fields,
            opcode,
            guard,
            unpredictable,
            decode,
        })
    }

    /// Parse an instruction set specifier.
    fn parse_instruction_set(&mut self) -> Result<InstructionSet> {
        match self.current_kind() {
            TokenKind::A32 => {
                self.advance();
                Ok(InstructionSet::A32)
            }
            TokenKind::A64 => {
                self.advance();
                Ok(InstructionSet::A64)
            }
            TokenKind::T16 => {
                self.advance();
                Ok(InstructionSet::T16)
            }
            TokenKind::T32 => {
                self.advance();
                Ok(InstructionSet::T32)
            }
            _ => Err(self.error("expected instruction set (A32, A64, T16, or T32)")),
        }
    }

    /// Parse an instruction field declaration.
    fn parse_instruction_field(&mut self) -> Result<InstructionField> {
        self.expect(&TokenKind::Field)?;
        let name = self.expect_ident()?;
        let begin = self.expect_nat()? as u32;
        self.expect(&TokenKind::PlusColon)?;
        let width = self.expect_nat()? as u32;

        Ok(InstructionField { name, begin, width })
    }

    /// Parse an opcode mask.
    fn parse_opcode_mask(&mut self) -> Result<Mask> {
        match self.current_kind().clone() {
            TokenKind::MaskLit(s) | TokenKind::BinLit(s) => {
                self.advance();
                self.parse_mask(&s)
            }
            _ => Err(self.error("expected opcode mask literal")),
        }
    }

    /// Parse an unpredictable_unless condition.
    fn parse_unpredictable_unless(&mut self) -> Result<UnpredictableUnless> {
        self.expect(&TokenKind::UnpredictableUnless)?;
        let index = self.expect_nat()? as u32;
        self.expect(&TokenKind::EqEq)?;

        // Parse the expected bit value
        let value = match self.current_kind().clone() {
            TokenKind::NatLit(0) => {
                self.advance();
                false
            }
            TokenKind::NatLit(1) => {
                self.advance();
                true
            }
            TokenKind::BinLit(s) => {
                self.advance();
                let bits = self.parse_bitvector(&s)?;
                bits.first().copied().unwrap_or(false)
            }
            _ => return Err(self.error("expected 0 or 1")),
        };

        Ok(UnpredictableUnless { index, value })
    }
}
