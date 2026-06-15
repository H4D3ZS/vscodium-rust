//! Definition parsing for ASL.

use super::Parser;
use crate::error::Result;
use crate::lexer::TokenKind;
use crate::syntax::*;

impl<'src> Parser<'src> {
    /// Parse a list of definitions.
    pub fn parse_definitions(&mut self) -> Result<Vec<Definition>> {
        let mut defs = Vec::new();

        while !self.at_eof() {
            defs.push(self.parse_definition()?);
        }

        Ok(defs)
    }

    /// Parse a single definition.
    pub fn parse_definition(&mut self) -> Result<Definition> {
        match self.current_kind().clone() {
            // __builtin type Name;
            TokenKind::Builtin => {
                self.advance();
                self.expect(&TokenKind::Type)?;
                let name = self.expect_ident()?;
                self.add_type_ident(&name);
                self.eat(&TokenKind::Semi);
                Ok(Definition::TypeBuiltin(name))
            }

            // type declarations
            TokenKind::Type => {
                self.advance(); // consume 'type' keyword
                self.parse_type_definition()
            }

            // enumeration Type { variants }
            TokenKind::Enumeration => {
                self.advance();
                let name = self.expect_ident()?;
                self.add_type_ident(&name);
                self.expect(&TokenKind::LBrace)?;
                let variants = self.parse_list1(|p| p.expect_ident())?;
                self.expect(&TokenKind::RBrace)?;
                self.eat(&TokenKind::Semi);
                Ok(Definition::TypeEnum { name, variants })
            }

            // constant Type Name = Value;
            TokenKind::Constant => {
                self.advance();
                let ty = self.parse_type()?;
                let name = self.expect_ident()?;
                self.expect(&TokenKind::Eq)?;
                let value = self.parse_expr()?;
                self.eat(&TokenKind::Semi);
                Ok(Definition::Const { name, ty, value })
            }

            // Array definition - two possible syntaxes:
            // 1. array Type Name[IndexType];
            // 2. array [IndexRange] of Type Name;  (for register arrays)
            TokenKind::Array => {
                self.advance();

                if self.check(&TokenKind::LBracket) {
                    // Syntax 2: array [IndexRange] of Type Name;
                    self.advance();
                    let index = self.parse_index_type()?;
                    self.expect(&TokenKind::RBracket)?;
                    self.expect(&TokenKind::Of)?;

                    // The type might be __register with fields
                    let ty = if self.check(&TokenKind::Register) {
                        let (width, fields) = self.parse_register_type_only()?;
                        Type::Register {
                            width: width as u64,
                            fields: fields
                                .into_iter()
                                .map(|f| RegField {
                                    name: f.name.unwrap_or_default(),
                                    slices: vec![Slice::Range {
                                        hi: Box::new(Expr::int(f.hi as i128)),
                                        lo: Box::new(Expr::int(f.lo as i128)),
                                    }],
                                })
                                .collect(),
                        }
                    } else {
                        self.parse_type()?
                    };

                    let name = self.expect_ident()?;
                    self.eat(&TokenKind::Semi);
                    Ok(Definition::Array { name, ty, index })
                } else {
                    // Syntax 1: array Type Name[IndexType];
                    let ty = self.parse_type()?;
                    let name = self.expect_ident()?;
                    self.expect(&TokenKind::LBracket)?;
                    let index = self.parse_index_type()?;
                    self.expect(&TokenKind::RBracket)?;
                    self.eat(&TokenKind::Semi);
                    Ok(Definition::Array { name, ty, index })
                }
            }

            // Register definitions - convert to variable with register type
            TokenKind::Register => {
                let reg = self.parse_register_definition_inline()?;
                match reg {
                    RegisterDefinition::Single(r) => Ok(Definition::Variable {
                        name: QualifiedIdentifier::new(r.name.clone()),
                        ty: Type::Register {
                            width: r.width as u64,
                            fields: r
                                .fields
                                .into_iter()
                                .map(|f| RegField {
                                    name: f.name.unwrap_or_default(),
                                    slices: vec![Slice::Range {
                                        hi: Box::new(Expr::int(f.hi as i128)),
                                        lo: Box::new(Expr::int(f.lo as i128)),
                                    }],
                                })
                                .collect(),
                        },
                    }),
                    RegisterDefinition::Array(arr) => Ok(Definition::Array {
                        name: arr.register.name,
                        ty: Type::Register {
                            width: arr.register.width as u64,
                            fields: arr
                                .register
                                .fields
                                .into_iter()
                                .map(|f| RegField {
                                    name: f.name.unwrap_or_default(),
                                    slices: vec![Slice::Range {
                                        hi: Box::new(Expr::int(f.hi as i128)),
                                        lo: Box::new(Expr::int(f.lo as i128)),
                                    }],
                                })
                                .collect(),
                        },
                        index: IndexType::Range {
                            lo: Box::new(Expr::int(arr.index_min as i128)),
                            hi: Box::new(Expr::int(arr.index_max as i128)),
                        },
                    }),
                }
            }

            // Config variables
            TokenKind::Config | TokenKind::Exslock => {
                // Skip config marker
                self.advance();
                self.parse_callable_or_var()
            }

            // Instruction definition
            TokenKind::Instruction => {
                let instr = self.parse_instruction()?;
                Ok(Definition::Instruction(instr))
            }

            // Decode definition
            TokenKind::Decode => self.parse_decode_definition(),

            // Type definition with type1 keyword
            TokenKind::Ident(s) if s == "type1" => {
                self.advance();
                self.parse_type_definition()
            }

            // Callable definitions, getters, setters, or global variables
            TokenKind::Ident(_) | TokenKind::AArch32 | TokenKind::AArch64 | TokenKind::LParen => {
                self.parse_callable_or_var()
            }

            _ => Err(self.error(format!(
                "unexpected token in definition: {:?}",
                self.current_kind()
            ))),
        }
    }

    /// Parse a type definition.
    fn parse_type_definition(&mut self) -> Result<Definition> {
        // Type name can be qualified (e.g., AArch32.SErrorSyndrome)
        let qid = self.parse_qualified_ident()?;
        // Register the full name as a type identifier
        self.add_type_ident(&qid.to_string());

        if self.eat(&TokenKind::Semi).is_some() {
            // Abstract type: type Name;
            Ok(Definition::TypeAbstract(qid.name.clone()))
        } else if self.eat(&TokenKind::Eq).is_some() {
            // Type alias: type Name = Type;
            let ty = self.parse_type()?;
            self.eat(&TokenKind::Semi);
            Ok(Definition::TypeAlias {
                name: qid.name.clone(),
                ty,
            })
        } else if self.eat(&TokenKind::Is).is_some() {
            // Struct type: type Name is (fields)
            self.expect(&TokenKind::LParen)?;
            let fields = self.parse_list0(&TokenKind::RParen, |p| p.parse_typed_name())?;
            self.expect(&TokenKind::RParen)?;
            self.eat(&TokenKind::Semi);
            Ok(Definition::TypeStruct { name: qid, fields })
        } else {
            Err(self.error("expected ';', '=', or 'is' after type name"))
        }
    }

    /// Parse a callable definition, getter, setter, or global variable.
    fn parse_callable_or_var(&mut self) -> Result<Definition> {
        // This is complex because we need to distinguish between:
        // - ReturnType Name(args) body      (function)
        // - Name(args) body                 (procedure)
        // - ReturnType Name body            (getter)
        // - ReturnType Name[args] body      (indexed getter)
        // - Name = ReturnType value body    (setter)
        // - Name[args] = Type value body    (indexed setter)
        // - Type Name;                      (global variable)

        // Special case: tuple return type starts with (
        if self.check(&TokenKind::LParen) {
            return self.parse_def_with_return_type();
        }

        // For identifiers, we need to distinguish between:
        // - Ident(args)  -> procedure
        // - Ident[args] = ... -> setter
        // - Ident = ... -> setter
        // - Ident Ident... -> type followed by name (function/getter/variable)
        if matches!(
            self.current_kind(),
            TokenKind::Ident(_) | TokenKind::AArch32 | TokenKind::AArch64
        ) {
            // Parse the first qualified identifier
            let qid = self.parse_qualified_ident()?;

            // Check what follows
            if self.check(&TokenKind::LParen) {
                // Could be procedure Name(args) OR parameterized type bits(N) Name...
                // Check if this is a known type that takes parameters
                if self.is_param_type(&qid.name) {
                    // It's a parameterized type like bits(N)
                    self.advance(); // consume (
                    let arg = self.parse_expr()?;
                    self.expect(&TokenKind::RParen)?;
                    let ty = Type::Fun {
                        name: qid.name,
                        arg: Box::new(arg),
                    };
                    return self.parse_def_after_type(vec![ty]);
                } else {
                    // It's a procedure call
                    self.advance(); // consume (
                    let params = self.parse_list0(&TokenKind::RParen, |p| p.parse_typed_name())?;
                    self.expect(&TokenKind::RParen)?;

                    let body = if self.eat(&TokenKind::Semi).is_some() {
                        Vec::new()
                    } else {
                        self.parse_optional_indented_block(|p| p.parse_stmt())?
                    };

                    return Ok(Definition::Callable {
                        name: qid,
                        params,
                        returns: Vec::new(),
                        body,
                    });
                }
            } else if self.check(&TokenKind::LBracket) {
                // Indexed setter: Name[args] = Type value body or Name[] = Type value body
                // Can also be just a declaration ending with ;
                self.advance(); // consume [
                let params = if self.check(&TokenKind::RBracket) {
                    Vec::new()
                } else {
                    self.parse_setter_args()?
                };
                self.expect(&TokenKind::RBracket)?;
                self.expect(&TokenKind::Eq)?;

                let value = self.parse_typed_name()?;
                let body = if self.eat(&TokenKind::Semi).is_some() {
                    // Declaration only, no body
                    Vec::new()
                } else {
                    self.parse_optional_indented_block(|p| p.parse_stmt())?
                };

                return Ok(Definition::Setter {
                    name: qid,
                    params: if params.is_empty() {
                        None
                    } else {
                        Some(params)
                    },
                    value,
                    body,
                });
            } else if self.check(&TokenKind::Eq) {
                // Simple setter: Name = Type value body
                // Can also be just a declaration ending with ;
                self.advance(); // consume =
                let value = self.parse_typed_name()?;
                let body = if self.eat(&TokenKind::Semi).is_some() {
                    // Declaration only, no body
                    Vec::new()
                } else {
                    self.parse_optional_indented_block(|p| p.parse_stmt())?
                };

                return Ok(Definition::Setter {
                    name: qid,
                    params: None,
                    value,
                    body,
                });
            } else {
                // First identifier is a type name, what follows is the function/getter/variable name
                let ty = Type::Ref(qid);
                return self.parse_def_after_type(vec![ty]);
            }
        }

        Err(self.error("expected definition"))
    }

    /// Check if an identifier is a parameterized type like bits, signal, etc.
    fn is_param_type(&self, name: &str) -> bool {
        matches!(name, "bits" | "bit" | "signal" | "__RAM")
    }

    /// Parse a definition that starts with a return type.
    fn parse_def_with_return_type(&mut self) -> Result<Definition> {
        let returns = self.parse_return_type()?;
        self.parse_def_after_type(returns)
    }

    /// Parse a definition after the return type has been parsed.
    fn parse_def_after_type(&mut self, returns: Vec<Type>) -> Result<Definition> {
        // Parse the name
        let name = self.parse_qualified_ident()?;

        // What follows determines what kind of definition this is
        if self.eat(&TokenKind::LParen).is_some() {
            // Function: ReturnType Name(args) [body]
            let params = self.parse_list0(&TokenKind::RParen, |p| p.parse_typed_name())?;
            self.expect(&TokenKind::RParen)?;

            let body = if self.eat(&TokenKind::Semi).is_some() {
                Vec::new()
            } else {
                self.parse_optional_indented_block(|p| p.parse_stmt())?
            };

            Ok(Definition::Callable {
                name,
                params,
                returns,
                body,
            })
        } else if self.eat(&TokenKind::LBracket).is_some() {
            // Indexed getter: ReturnType Name[args] body or ReturnType Name[] body
            // Can also be just a declaration ending with ;
            let params = self.parse_list0(&TokenKind::RBracket, |p| p.parse_typed_name())?;
            self.expect(&TokenKind::RBracket)?;

            let body = if self.eat(&TokenKind::Semi).is_some() {
                // Declaration only, no body
                Vec::new()
            } else {
                self.parse_optional_indented_block(|p| p.parse_stmt())?
            };

            Ok(Definition::Getter {
                name,
                params: if params.is_empty() {
                    None
                } else {
                    Some(params)
                },
                returns,
                body,
            })
        } else if self.check(&TokenKind::Indent) {
            // Getter: ReturnType Name body
            let body = self.parse_stmt_block()?;

            Ok(Definition::Getter {
                name,
                params: None,
                returns,
                body,
            })
        } else if self.eat(&TokenKind::Semi).is_some() {
            // Global variable: Type Name;
            if returns.len() == 1 {
                Ok(Definition::Variable {
                    name,
                    ty: returns.into_iter().next().unwrap(),
                })
            } else {
                Err(self.error("invalid variable declaration"))
            }
        } else {
            Err(self.error("expected '(', '[', or statement block"))
        }
    }

    /// Parse setter arguments (which can include reference markers).
    fn parse_setter_args(&mut self) -> Result<Vec<SetterArg>> {
        self.parse_list1(|p| p.parse_setter_arg())
    }

    /// Parse a single setter argument.
    fn parse_setter_arg(&mut self) -> Result<SetterArg> {
        let ty = self.parse_type()?;

        // Check for reference marker
        let is_ref = self.eat(&TokenKind::Amp).is_some();

        let name = self.expect_ident()?;

        Ok(SetterArg {
            decl: SymbolDecl { name, ty },
            is_ref,
        })
    }

    /// Parse a decode definition.
    fn parse_decode_definition(&mut self) -> Result<Definition> {
        self.expect(&TokenKind::Decode)?;
        let iset = match self.current_kind() {
            TokenKind::A64 => {
                self.advance();
                InstructionSet::A64
            }
            TokenKind::A32 => {
                self.advance();
                InstructionSet::A32
            }
            TokenKind::T32 => {
                self.advance();
                InstructionSet::T32
            }
            TokenKind::T16 => {
                self.advance();
                InstructionSet::T16
            }
            _ => return Err(self.error("expected instruction set after __decode")),
        };
        // Parse the decode body which contains decode statements
        let body = self.parse_indented_block(|p| p.parse_decode_stmt())?;

        // The top-level should contain a single case statement typically
        // Convert to DecodeCase format for backwards compatibility
        let cases = body
            .into_iter()
            .filter_map(|stmt| {
                if let DecodeStmt::Case { fields, branches } = stmt {
                    Some(DecodeCase { fields, branches })
                } else {
                    None
                }
            })
            .collect();

        Ok(Definition::Decode { iset, cases })
    }

    /// Parse a decode statement (case, __field, __encoding, etc.).
    fn parse_decode_stmt(&mut self) -> Result<DecodeStmt> {
        match self.current_kind().clone() {
            // Nested case statement
            TokenKind::Case => {
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let fields = self.parse_decode_fields()?;
                self.expect(&TokenKind::RParen)?;
                self.expect(&TokenKind::Of)?;
                let branches = self.parse_decode_branches()?;
                Ok(DecodeStmt::Case { fields, branches })
            }
            // Field extraction: __field name start +: width
            TokenKind::Field => {
                self.advance();
                let name = self.expect_ident()?;
                let start = self.expect_nat()? as u32;
                self.expect(&TokenKind::PlusColon)?;
                let width = self.expect_nat()? as u32;
                Ok(DecodeStmt::Field { name, start, width })
            }
            // Encoding reference: __encoding name (name can be qualified like MLA_Z.P.ZZZ__ or include numbers like LD1SB_Z.P.BZ_D.64.unscaled)
            TokenKind::Encoding => {
                self.advance();
                // Parse the encoding name, which can contain dots, identifiers, and numbers
                let mut name = self.expect_ident()?;
                while self.eat(&TokenKind::Dot).is_some() {
                    // Next part can be an identifier or a number
                    if let Some(ident) = self.get_ident() {
                        name = smol_str::SmolStr::new(format!("{}.{}", name, ident));
                        self.advance();
                    } else if let TokenKind::NatLit(n) = self.current_kind() {
                        name = smol_str::SmolStr::new(format!("{}.{}", name, n));
                        self.advance();
                    } else {
                        return Err(
                            self.error("expected identifier or number after '.' in encoding name")
                        );
                    }
                }
                Ok(DecodeStmt::Encoding(name))
            }
            // __UNALLOCATED
            TokenKind::Unallocated => {
                self.advance();
                Ok(DecodeStmt::Unallocated)
            }
            // __UNPREDICTABLE (in decode context)
            TokenKind::UnpredictableDecode => {
                self.advance();
                Ok(DecodeStmt::Unpredictable)
            }
            // __NOP
            TokenKind::Nop => {
                self.advance();
                Ok(DecodeStmt::Nop)
            }
            _ => Err(self.error(format!(
                "expected decode statement, found {:?}",
                self.current_kind()
            ))),
        }
    }

    /// Parse decode field specifications.
    ///
    /// Can be either:
    /// - Numeric slices: `29 +: 3, 25 +: 4`
    /// - Field references: `op, size` (referring to previously extracted fields)
    fn parse_decode_fields(&mut self) -> Result<Vec<DecodeSlice>> {
        if self.check(&TokenKind::RParen) {
            // Empty field list: case () of
            return Ok(Vec::new());
        }

        let mut fields = Vec::new();
        loop {
            let slice = self.parse_decode_slice()?;
            fields.push(slice);
            if !self.eat(&TokenKind::Comma).is_some() {
                break;
            }
        }
        Ok(fields)
    }

    /// Parse a single decode slice (numeric, field reference, or concatenation).
    fn parse_decode_slice(&mut self) -> Result<DecodeSlice> {
        let first = self.parse_decode_slice_atom()?;

        // Check for concatenation with ':'
        if self.check(&TokenKind::Colon) {
            let mut parts = vec![first];
            while self.eat(&TokenKind::Colon).is_some() {
                parts.push(self.parse_decode_slice_atom()?);
            }
            Ok(DecodeSlice::Concat(parts))
        } else {
            Ok(first)
        }
    }

    /// Parse a single decode slice atom (without concatenation).
    fn parse_decode_slice_atom(&mut self) -> Result<DecodeSlice> {
        // Check if it's a numeric slice or a field reference
        if let TokenKind::NatLit(_) = self.current_kind() {
            let start = self.expect_nat()? as u32;
            // Check if this is a slice (N +: M) or just a number used as a field
            if self.eat(&TokenKind::PlusColon).is_some() {
                let width = self.expect_nat()? as u32;
                Ok(DecodeSlice::Slice { start, width })
            } else {
                // Just a number, treat as a field reference
                Ok(DecodeSlice::Field(smol_str::SmolStr::new(format!(
                    "{}",
                    start
                ))))
            }
        } else if let Some(name) = self.get_ident() {
            let name = smol_str::SmolStr::new(name);
            self.advance();
            Ok(DecodeSlice::Field(name))
        } else {
            Err(self.error("expected numeric slice (N +: M) or field name"))
        }
    }

    /// Parse decode branches (when/otherwise).
    fn parse_decode_branches(&mut self) -> Result<Vec<DecodeBranch>> {
        // Check if we have an indented block or inline branches
        if self.check(&TokenKind::Indent) {
            self.parse_indented_block(|p| p.parse_decode_branch())
        } else {
            // Single inline branch after =>
            let branch = self.parse_decode_branch()?;
            Ok(vec![branch])
        }
    }

    /// Parse a single decode branch (when or otherwise).
    fn parse_decode_branch(&mut self) -> Result<DecodeBranch> {
        if self.eat(&TokenKind::When).is_some() {
            self.expect(&TokenKind::LParen)?;
            let patterns = self.parse_list0(&TokenKind::RParen, |p| p.parse_decode_pattern())?;
            self.expect(&TokenKind::RParen)?;
            self.expect(&TokenKind::FatArrow)?;
            let body = self.parse_decode_body()?;
            Ok(DecodeBranch {
                patterns: Some(patterns),
                body,
            })
        } else if self.eat(&TokenKind::Otherwise).is_some() {
            self.expect(&TokenKind::FatArrow)?;
            let body = self.parse_decode_body()?;
            Ok(DecodeBranch {
                patterns: None,
                body,
            })
        } else {
            Err(self.error("expected 'when' or 'otherwise'"))
        }
    }

    /// Parse decode body (inline statements or indented block).
    fn parse_decode_body(&mut self) -> Result<Vec<DecodeStmt>> {
        if self.check(&TokenKind::Indent) {
            self.parse_indented_block(|p| p.parse_decode_stmt())
        } else {
            // Parse inline decode statements on same line
            let mut stmts = Vec::new();
            while !self.at_decode_branch_end() {
                stmts.push(self.parse_decode_stmt()?);
            }
            Ok(stmts)
        }
    }

    /// Check if we're at the end of a decode branch.
    fn at_decode_branch_end(&self) -> bool {
        self.check(&TokenKind::When)
            || self.check(&TokenKind::Otherwise)
            || self.check(&TokenKind::Dedent)
            || self.at_eof()
    }

    /// Parse a decode pattern (wildcard, literal, or negated).
    fn parse_decode_pattern(&mut self) -> Result<DecodePattern> {
        // Check for negation
        if self.eat(&TokenKind::Bang).is_some() {
            let inner = self.parse_decode_pattern()?;
            return Ok(DecodePattern::Not(Box::new(inner)));
        }

        if let Some(s) = self.get_ident() {
            if s == "_" {
                self.advance();
                Ok(DecodePattern::Wildcard)
            } else {
                Err(self.error("expected _, literal, or !pattern in decode pattern"))
            }
        } else if let TokenKind::StringLit(s) = self.current_kind() {
            let s = s.clone();
            self.advance();
            Ok(DecodePattern::Lit(s))
        } else if let TokenKind::BinLit(s) = self.current_kind() {
            let s = s.clone();
            self.advance();
            Ok(DecodePattern::Lit(s))
        } else if let TokenKind::MaskLit(s) = self.current_kind() {
            let s = s.clone();
            self.advance();
            Ok(DecodePattern::Lit(s))
        } else if let TokenKind::ConstraintLit(s) = self.current_kind() {
            let s = s.clone();
            self.advance();
            Ok(DecodePattern::Lit(s))
        } else {
            Err(self.error("expected decode pattern"))
        }
    }

    /// Parse a when branch (legacy, for backwards compatibility).
    fn parse_when_branch(&mut self) -> Result<WhenBranch> {
        self.expect(&TokenKind::When)?;
        self.expect(&TokenKind::LParen)?;
        let pattern = self.parse_list0(&TokenKind::RParen, |p| p.parse_decode_pattern())?;
        self.expect(&TokenKind::RParen)?;
        self.expect(&TokenKind::FatArrow)?;
        let body = self.parse_decode_body()?;
        Ok(WhenBranch { pattern, body })
    }

    /// Parse a decode case (legacy, for backwards compatibility).
    fn parse_decode_case(&mut self) -> Result<DecodeCase> {
        self.expect(&TokenKind::Case)?;
        self.expect(&TokenKind::LParen)?;
        let fields = self.parse_decode_fields()?;
        self.expect(&TokenKind::RParen)?;
        self.expect(&TokenKind::Of)?;
        let branches = self.parse_decode_branches()?;
        Ok(DecodeCase { fields, branches })
    }
}
