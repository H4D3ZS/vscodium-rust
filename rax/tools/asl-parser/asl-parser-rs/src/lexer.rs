//! Lexer for ASL with indentation-based syntax handling.
//!
//! ASL uses Python-like significant indentation to delimit blocks.
//! This lexer wraps a base token stream and injects synthetic `Indent`
//! and `Dedent` tokens at appropriate positions.

use crate::span::Span;
use logos::Logos;
use std::fmt;

// ============================================================================
// Token Kind
// ============================================================================

/// The kind of token produced by the lexer.
#[derive(Logos, Debug, Clone, PartialEq, Eq, Hash)]
#[logos(skip r"[ \t]+")]
#[allow(missing_docs)]
pub enum TokenKind {
    // ========================================================================
    // Keywords
    // ========================================================================
    #[token("AND")]
    And,
    #[token("DIV")]
    Div,
    #[token("EOR")]
    Eor,
    #[token("IMPLEMENTATION_DEFINED")]
    ImplementationDefined,
    #[token("IN")]
    In,
    #[token("MOD")]
    Mod,
    #[token("NOT")]
    Not,
    #[token("OR")]
    Or,
    #[token("QUOT")]
    Quot,
    #[token("REM")]
    Rem,
    #[token("SEE")]
    See,
    #[token("UNDEFINED")]
    Undefined,
    #[token("UNKNOWN")]
    Unknown,
    #[token("UNPREDICTABLE")]
    Unpredictable,
    #[token("__builtin")]
    Builtin,
    #[token("__conditional")]
    Conditional,
    #[token("__config")]
    Config,
    #[token("__decode")]
    Decode,
    #[token("__encoding")]
    Encoding,
    #[token("__event")]
    Event,
    #[token("__execute")]
    Execute,
    #[token("__exslock")]
    Exslock,
    #[token("__field")]
    Field,
    #[token("__guard")]
    Guard,
    #[token("__instruction")]
    Instruction,
    #[token("__instruction_set")]
    InstructionSet,
    #[token("__map")]
    Map,
    #[token("__newmap")]
    NewMap,
    #[token("__newevent")]
    NewEvent,
    #[token("__NOP")]
    Nop,
    #[token("__opcode")]
    Opcode,
    #[token("__operator1")]
    Operator1,
    #[token("__operator2")]
    Operator2,
    #[token("__postdecode")]
    PostDecode,
    #[token("__readwrite")]
    ReadWrite,
    #[token("__register")]
    Register,
    #[token("__UNALLOCATED")]
    Unallocated,
    #[token("__unpredictable")]
    UnpredictableKw,
    #[token("__UNPREDICTABLE")]
    UnpredictableDecode,
    #[token("__unpredictable_unless")]
    UnpredictableUnless,
    #[token("__write")]
    Write,
    #[token("array")]
    Array,
    #[token("assert")]
    Assert,
    #[token("case")]
    Case,
    #[token("catch")]
    Catch,
    #[token("constant")]
    Constant,
    #[token("do")]
    Do,
    #[token("downto")]
    Downto,
    #[token("else")]
    Else,
    #[token("elsif")]
    Elsif,
    #[token("enumeration")]
    Enumeration,
    #[token("for")]
    For,
    #[token("if")]
    If,
    #[token("is")]
    Is,
    #[token("let")]
    Let,
    #[token("of")]
    Of,
    #[token("otherwise")]
    Otherwise,
    #[token("repeat")]
    Repeat,
    #[token("return")]
    Return,
    #[token("then")]
    Then,
    #[token("throw")]
    Throw,
    #[token("to")]
    To,
    #[token("try")]
    Try,
    #[token("type")]
    Type,
    #[token("typeof")]
    Typeof,
    #[token("until")]
    Until,
    #[token("when")]
    When,
    #[token("while")]
    While,

    // ========================================================================
    // Architecture qualifiers
    // ========================================================================
    #[token("AArch32")]
    AArch32,
    #[token("AArch64")]
    AArch64,

    // ========================================================================
    // Instruction sets
    // ========================================================================
    #[token("A32")]
    A32,
    #[token("A64")]
    A64,
    #[token("T16")]
    T16,
    #[token("T32")]
    T32,

    // ========================================================================
    // Operators and punctuation
    // ========================================================================
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token(",")]
    Comma,
    #[token(";")]
    Semi,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("!=")]
    Ne,
    #[token("<=")]
    Le,
    #[token(">=")]
    Ge,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("^")]
    Caret,
    #[token("!")]
    Bang,
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,
    #[token("++")]
    PlusPlus,
    #[token("+:")]
    PlusColon,
    #[token("=>")]
    FatArrow,
    #[token("&")]
    Amp,

    // ========================================================================
    // Literals
    // ========================================================================
    /// Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    /// Natural number literal
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    NatLit(u128),

    /// Hexadecimal literal (stored as string to support arbitrary precision)
    #[regex(r"0x[0-9a-fA-F]+", |lex| lex.slice().to_string())]
    HexLit(String),

    /// Real number literal
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().to_string())]
    RealLit(String),

    /// Binary literal (e.g., '1010')
    #[regex(r"'[01 ]+'", priority = 3, callback = |lex| lex.slice().to_string())]
    BinLit(String),

    /// Mask literal (e.g., '10xx') - lower priority than BinLit
    #[regex(r"'[01x ]+'", priority = 2, callback = |lex| lex.slice().to_string())]
    MaskLit(String),

    /// Constraint pattern (e.g., '11!= 00') - patterns with != constraints
    #[regex(r"'[01x !=]+'", priority = 1, callback = |lex| lex.slice().to_string())]
    ConstraintLit(String),

    /// String literal
    #[regex(r#""[^"]*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLit(String),

    // ========================================================================
    // Special tokens (synthetic)
    // ========================================================================
    /// Newline (tracked for indentation handling)
    #[regex(r"\n|\r\n")]
    Newline,

    /// Synthetic indent token
    Indent,

    /// Synthetic dedent token
    Dedent,

    /// End of file
    Eof,

    /// Error token
    Error,

    // ========================================================================
    // Comments
    // ========================================================================
    /// Line comment
    #[regex(r"//[^\n]*")]
    LineComment,

    /// Block comment (handled specially for nesting)
    #[token("/*", parse_block_comment)]
    BlockComment,
}

fn parse_block_comment(lex: &mut logos::Lexer<TokenKind>) -> logos::FilterResult<(), ()> {
    let mut depth = 1;
    let remainder = lex.remainder();
    let mut chars = remainder.chars().peekable();
    let mut consumed = 0;

    while depth > 0 {
        match chars.next() {
            Some('/') if chars.peek() == Some(&'*') => {
                chars.next();
                consumed += 2;
                depth += 1;
            }
            Some('*') if chars.peek() == Some(&'/') => {
                chars.next();
                consumed += 2;
                depth -= 1;
            }
            Some(c) => {
                consumed += c.len_utf8();
            }
            None => {
                // Unterminated comment
                lex.bump(consumed);
                return logos::FilterResult::Error(());
            }
        }
    }

    lex.bump(consumed);
    logos::FilterResult::Skip
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Keywords
            Self::And => write!(f, "AND"),
            Self::Div => write!(f, "DIV"),
            Self::Eor => write!(f, "EOR"),
            Self::ImplementationDefined => write!(f, "IMPLEMENTATION_DEFINED"),
            Self::In => write!(f, "IN"),
            Self::Mod => write!(f, "MOD"),
            Self::Not => write!(f, "NOT"),
            Self::Or => write!(f, "OR"),
            Self::Quot => write!(f, "QUOT"),
            Self::Rem => write!(f, "REM"),
            Self::See => write!(f, "SEE"),
            Self::Undefined => write!(f, "UNDEFINED"),
            Self::Unknown => write!(f, "UNKNOWN"),
            Self::Unpredictable => write!(f, "UNPREDICTABLE"),
            Self::Builtin => write!(f, "__builtin"),
            Self::Conditional => write!(f, "__conditional"),
            Self::Config => write!(f, "__config"),
            Self::Decode => write!(f, "__decode"),
            Self::Encoding => write!(f, "__encoding"),
            Self::Event => write!(f, "__event"),
            Self::Execute => write!(f, "__execute"),
            Self::Exslock => write!(f, "__exslock"),
            Self::Field => write!(f, "__field"),
            Self::Guard => write!(f, "__guard"),
            Self::Instruction => write!(f, "__instruction"),
            Self::InstructionSet => write!(f, "__instruction_set"),
            Self::Map => write!(f, "__map"),
            Self::NewMap => write!(f, "__newmap"),
            Self::NewEvent => write!(f, "__newevent"),
            Self::Nop => write!(f, "__NOP"),
            Self::Opcode => write!(f, "__opcode"),
            Self::Operator1 => write!(f, "__operator1"),
            Self::Operator2 => write!(f, "__operator2"),
            Self::PostDecode => write!(f, "__postdecode"),
            Self::ReadWrite => write!(f, "__readwrite"),
            Self::Register => write!(f, "__register"),
            Self::Unallocated => write!(f, "__UNALLOCATED"),
            Self::UnpredictableKw => write!(f, "__unpredictable"),
            Self::UnpredictableDecode => write!(f, "__UNPREDICTABLE"),
            Self::UnpredictableUnless => write!(f, "__unpredictable_unless"),
            Self::Write => write!(f, "__write"),
            Self::Array => write!(f, "array"),
            Self::Assert => write!(f, "assert"),
            Self::Case => write!(f, "case"),
            Self::Catch => write!(f, "catch"),
            Self::Constant => write!(f, "constant"),
            Self::Do => write!(f, "do"),
            Self::Downto => write!(f, "downto"),
            Self::Else => write!(f, "else"),
            Self::Elsif => write!(f, "elsif"),
            Self::Enumeration => write!(f, "enumeration"),
            Self::For => write!(f, "for"),
            Self::If => write!(f, "if"),
            Self::Is => write!(f, "is"),
            Self::Let => write!(f, "let"),
            Self::Of => write!(f, "of"),
            Self::Otherwise => write!(f, "otherwise"),
            Self::Repeat => write!(f, "repeat"),
            Self::Return => write!(f, "return"),
            Self::Then => write!(f, "then"),
            Self::Throw => write!(f, "throw"),
            Self::To => write!(f, "to"),
            Self::Try => write!(f, "try"),
            Self::Type => write!(f, "type"),
            Self::Typeof => write!(f, "typeof"),
            Self::Until => write!(f, "until"),
            Self::When => write!(f, "when"),
            Self::While => write!(f, "while"),

            // Architecture
            Self::AArch32 => write!(f, "AArch32"),
            Self::AArch64 => write!(f, "AArch64"),
            Self::A32 => write!(f, "A32"),
            Self::A64 => write!(f, "A64"),
            Self::T16 => write!(f, "T16"),
            Self::T32 => write!(f, "T32"),

            // Punctuation
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Comma => write!(f, ","),
            Self::Semi => write!(f, ";"),
            Self::Colon => write!(f, ":"),
            Self::Dot => write!(f, "."),
            Self::DotDot => write!(f, ".."),
            Self::Eq => write!(f, "="),
            Self::EqEq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Le => write!(f, "<="),
            Self::Ge => write!(f, ">="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Caret => write!(f, "^"),
            Self::Bang => write!(f, "!"),
            Self::AndAnd => write!(f, "&&"),
            Self::OrOr => write!(f, "||"),
            Self::Shl => write!(f, "<<"),
            Self::Shr => write!(f, ">>"),
            Self::PlusPlus => write!(f, "++"),
            Self::PlusColon => write!(f, "+:"),
            Self::FatArrow => write!(f, "=>"),
            Self::Amp => write!(f, "&"),

            // Literals
            Self::Ident(s) => write!(f, "{s}"),
            Self::NatLit(n) => write!(f, "{n}"),
            Self::HexLit(s) => write!(f, "{s}"),
            Self::RealLit(s) => write!(f, "{s}"),
            Self::BinLit(s) => write!(f, "{s}"),
            Self::MaskLit(s) => write!(f, "{s}"),
            Self::ConstraintLit(s) => write!(f, "{s}"),
            Self::StringLit(s) => write!(f, "\"{s}\""),

            // Special
            Self::Newline => write!(f, "newline"),
            Self::Indent => write!(f, "indent"),
            Self::Dedent => write!(f, "dedent"),
            Self::Eof => write!(f, "end of file"),
            Self::Error => write!(f, "error"),
            Self::LineComment => write!(f, "comment"),
            Self::BlockComment => write!(f, "comment"),
        }
    }
}

impl TokenKind {
    /// Returns true if this token opens a bracket-like grouping.
    fn is_bracket_opener(&self) -> bool {
        matches!(self, Self::LParen | Self::LBracket | Self::LBrace)
    }

    /// Returns true if this token closes a bracket-like grouping.
    fn is_bracket_closer(&self) -> bool {
        matches!(self, Self::RParen | Self::RBracket | Self::RBrace)
    }

    /// Returns true if this token starts a statement condition (if/while/elsif).
    fn is_condition_start(&self) -> bool {
        matches!(self, Self::If | Self::Elsif | Self::While)
    }

    /// Returns true if this token ends a statement condition (then/do).
    fn is_condition_end(&self) -> bool {
        matches!(self, Self::Then | Self::Do)
    }

    /// Returns true if this is a comment token.
    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }
}

// ============================================================================
// Token
// ============================================================================

/// A token with its source span.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// The source span
    pub span: Span,
}

impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Check if this token is of a specific kind.
    pub fn is(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.kind) == std::mem::discriminant(kind)
    }
}

// ============================================================================
// Lexer (with indentation handling)
// ============================================================================

/// The ASL lexer with indentation-aware token generation.
///
/// This lexer wraps the base Logos-generated lexer and injects synthetic
/// `Indent` and `Dedent` tokens based on changes in indentation level.
pub struct Lexer<'src> {
    /// The source text
    source: &'src str,
    /// The base Logos lexer
    inner: logos::Lexer<'src, TokenKind>,
    /// Stack of indentation levels
    indent_stack: Vec<usize>,
    /// Pending tokens to emit
    pending: Vec<Token>,
    /// Current bracket nesting depth (parens, brackets, braces)
    bracket_depth: usize,
    /// Current condition nesting depth (if/while/elsif before then/do)
    condition_depth: usize,
    /// Whether we're at the start of a line
    at_line_start: bool,
    /// Current position for synthetic tokens
    current_pos: usize,
    /// Whether we've hit EOF
    eof: bool,
}

impl<'src> Lexer<'src> {
    /// Create a new lexer for the given source text.
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            inner: TokenKind::lexer(source),
            indent_stack: vec![0],
            pending: Vec::new(),
            bracket_depth: 0,
            condition_depth: 0,
            at_line_start: true,
            current_pos: 0,
            eof: false,
        }
    }

    /// Get the source text.
    pub fn source(&self) -> &'src str {
        self.source
    }

    /// Get the column (indentation level) of a position in the source.
    fn column_of(&self, pos: usize) -> usize {
        // Find the start of the current line
        let line_start = self.source[..pos].rfind('\n').map(|p| p + 1).unwrap_or(0);

        // Count characters (not bytes) from line start to pos
        self.source[line_start..pos].chars().count()
    }

    /// Emit pending dedent tokens down to the given level.
    fn emit_dedents_to(&mut self, level: usize, pos: usize) {
        while self.indent_stack.len() > 1 && *self.indent_stack.last().unwrap() > level {
            self.indent_stack.pop();
            self.pending
                .push(Token::new(TokenKind::Dedent, Span::new(pos, pos)));
        }
    }

    /// Get the next token from the base lexer, skipping comments.
    fn next_base_token(&mut self) -> Option<Token> {
        loop {
            match self.inner.next() {
                Some(Ok(kind)) if kind.is_comment() => {
                    // Skip comments
                    continue;
                }
                Some(Ok(kind)) => {
                    let span = Span::from(self.inner.span());
                    return Some(Token::new(kind, span));
                }
                Some(Err(())) => {
                    let span = Span::from(self.inner.span());
                    return Some(Token::new(TokenKind::Error, span));
                }
                None => return None,
            }
        }
    }

    /// Update nesting depths based on the token kind.
    fn update_nesting(&mut self, tok: &Token) {
        if tok.kind.is_bracket_opener() {
            self.bracket_depth += 1;
        } else if tok.kind.is_bracket_closer() {
            self.bracket_depth = self.bracket_depth.saturating_sub(1);
        } else if tok.kind.is_condition_start() {
            self.condition_depth += 1;
        } else if tok.kind.is_condition_end() {
            self.condition_depth = self.condition_depth.saturating_sub(1);
        }
    }

    /// Get the next token, handling indentation.
    pub fn next_token(&mut self) -> Token {
        // Return pending tokens first
        if let Some(tok) = self.pending.pop() {
            // Still need to track nesting for tokens returned from pending
            self.update_nesting(&tok);
            return tok;
        }

        // Check for EOF
        if self.eof {
            return Token::new(
                TokenKind::Eof,
                Span::new(self.source.len(), self.source.len()),
            );
        }

        // Get next token from base lexer
        let tok = match self.next_base_token() {
            Some(tok) => tok,
            None => {
                // EOF - emit remaining dedents
                self.eof = true;
                let pos = self.source.len();
                self.emit_dedents_to(0, pos);
                if let Some(tok) = self.pending.pop() {
                    return tok;
                }
                return Token::new(TokenKind::Eof, Span::new(pos, pos));
            }
        };

        self.current_pos = tok.span.end;

        // Handle newline
        if tok.kind == TokenKind::Newline {
            // If we're inside brackets or conditions, ignore newlines for indentation
            if self.bracket_depth > 0 || self.condition_depth > 0 {
                return self.next_token();
            }
            self.at_line_start = true;
            return self.next_token();
        }

        // Handle indentation at line start
        let tok = if self.at_line_start && self.bracket_depth == 0 && self.condition_depth == 0 {
            self.at_line_start = false;
            let col = self.column_of(tok.span.start);
            let current_indent = *self.indent_stack.last().unwrap();

            if col > current_indent {
                // Indent
                self.indent_stack.push(col);
                let start = tok.span.start;
                self.pending.push(tok);
                return Token::new(TokenKind::Indent, Span::new(start, start));
            } else if col < current_indent {
                // Dedent(s) - need to emit dedents before the token
                self.emit_dedents_to(col, tok.span.start);
                // Insert the token at the beginning so it comes out last
                self.pending.insert(0, tok);
                // Return the first dedent (last item pushed = top of stack)
                if let Some(dedent) = self.pending.pop() {
                    return dedent;
                }
                // This shouldn't happen, but return EOF if it does
                return Token::new(
                    TokenKind::Eof,
                    Span::new(self.source.len(), self.source.len()),
                );
            } else {
                tok
            }
        } else {
            tok
        };

        // Track nesting depth for brackets and conditions separately
        self.update_nesting(&tok);

        tok
    }

    /// Peek at the next token without consuming it.
    pub fn peek(&mut self) -> Token {
        let tok = self.next_token();
        self.pending.push(tok.clone());
        tok
    }

    /// Collect all remaining tokens into a vector.
    pub fn collect_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok.kind == TokenKind::Eof {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        tokens
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok.kind == TokenKind::Eof {
            None
        } else {
            Some(tok)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "if x then return 1;";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect_tokens();

        assert!(matches!(tokens[0].kind, TokenKind::If));
        assert!(matches!(tokens[1].kind, TokenKind::Ident(_)));
        assert!(matches!(tokens[2].kind, TokenKind::Then));
        assert!(matches!(tokens[3].kind, TokenKind::Return));
        assert!(matches!(tokens[4].kind, TokenKind::NatLit(1)));
        assert!(matches!(tokens[5].kind, TokenKind::Semi));
    }

    #[test]
    fn test_indent_dedent() {
        let source = "func()\n    x = 1;\n    y = 2;";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect_tokens();

        // Should have: func ( ) INDENT x = 1 ; y = 2 ; DEDENT EOF
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(kinds.iter().any(|k| matches!(k, TokenKind::Indent)));
        assert!(kinds.iter().any(|k| matches!(k, TokenKind::Dedent)));
    }

    #[test]
    fn test_nested_comments() {
        let source = "x /* outer /* inner */ outer */ y";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect_tokens();

        // Should skip comments and get: x y EOF
        assert!(matches!(&tokens[0].kind, TokenKind::Ident(s) if s == "x"));
        assert!(matches!(&tokens[1].kind, TokenKind::Ident(s) if s == "y"));
    }

    #[test]
    fn test_binary_literal() {
        let source = "'1010 1100'";
        let mut lexer = Lexer::new(source);
        let tok = lexer.next_token();
        assert!(matches!(
            tok.kind,
            TokenKind::BinLit(_) | TokenKind::MaskLit(_)
        ));
    }

    #[test]
    fn test_multiline_if_condition() {
        let source = "if A && (B || C)\n    && D then\n    x = 1;";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect_tokens();

        // Print all tokens for debugging
        for (i, tok) in tokens.iter().enumerate() {
            eprintln!("{}: {:?}", i, tok.kind);
        }

        // Find the position of the second AndAnd (the one after newline)
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        let mut and_count = 0;
        for (i, k) in kinds.iter().enumerate() {
            if matches!(k, TokenKind::AndAnd) {
                and_count += 1;
                if and_count == 2 {
                    // The token before the second && should NOT be Indent
                    assert!(
                        !matches!(kinds[i - 1], TokenKind::Indent),
                        "Indent should not appear before && in multi-line if. Prev token: {:?}",
                        kinds[i - 1]
                    );
                }
            }
        }
    }

    #[test]
    fn test_multiline_if_in_function() {
        // Simulates the actual case: if statement inside an indented function body
        let source = "func Test()\n    if A && (B || C)\n        && D then\n        x = 1;";
        let mut lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect_tokens();

        // Print all tokens for debugging
        eprintln!("Tokens for multiline if in function:");
        for (i, tok) in tokens.iter().enumerate() {
            eprintln!("{}: {:?}", i, tok.kind);
        }

        // Find the position of the second AndAnd (the one after newline)
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        let mut and_count = 0;
        for (i, k) in kinds.iter().enumerate() {
            if matches!(k, TokenKind::AndAnd) {
                and_count += 1;
                if and_count == 2 {
                    // The token before the second && should NOT be Indent
                    assert!(
                        !matches!(kinds[i - 1], TokenKind::Indent),
                        "Indent should not appear before && in multi-line if. Prev token: {:?}",
                        kinds[i - 1]
                    );
                }
            }
        }
    }
}
