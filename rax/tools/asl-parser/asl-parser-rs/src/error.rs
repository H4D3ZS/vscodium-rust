//! Error types for the ASL parser.

use crate::lexer::TokenKind;
use crate::span::Span;
use miette::Diagnostic;
use std::fmt;
use thiserror::Error;

/// Result type alias for parser operations.
pub type Result<T> = std::result::Result<T, ParseError>;

/// Parse error with rich diagnostic information.
#[derive(Error, Debug, Clone)]
pub struct ParseError {
    /// The kind of error
    pub kind: ParseErrorKind,
    /// The source span where the error occurred
    pub span: Span,
    /// Optional help message
    pub help: Option<String>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Diagnostic for ParseError {
    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.help
            .as_ref()
            .map(|h| Box::new(h.clone()) as Box<dyn fmt::Display>)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        Some(Box::new(std::iter::once(miette::LabeledSpan::new(
            Some(self.kind.to_string()),
            self.span.start,
            self.span.len(),
        ))))
    }
}

impl ParseError {
    /// Create a new parse error.
    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self {
            kind,
            span,
            help: None,
        }
    }

    /// Add a help message to this error.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Create an "unexpected token" error.
    pub fn unexpected(found: TokenKind, expected: &[TokenKind], span: Span) -> Self {
        Self::new(
            ParseErrorKind::UnexpectedToken {
                found,
                expected: expected.to_vec(),
            },
            span,
        )
    }

    /// Create an "unexpected end of file" error.
    pub fn unexpected_eof(expected: &[TokenKind], span: Span) -> Self {
        Self::new(
            ParseErrorKind::UnexpectedEof {
                expected: expected.to_vec(),
            },
            span,
        )
    }

    /// Create an "invalid literal" error.
    pub fn invalid_literal(msg: impl Into<String>, span: Span) -> Self {
        Self::new(ParseErrorKind::InvalidLiteral(msg.into()), span)
    }

    /// Create a custom error.
    pub fn custom(msg: impl Into<String>, span: Span) -> Self {
        Self::new(ParseErrorKind::Custom(msg.into()), span)
    }
}

/// The kind of parse error.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    /// Unexpected token encountered
    UnexpectedToken {
        /// The token that was found
        found: TokenKind,
        /// The tokens that were expected
        expected: Vec<TokenKind>,
    },

    /// Unexpected end of file
    UnexpectedEof {
        /// The tokens that were expected
        expected: Vec<TokenKind>,
    },

    /// Invalid literal value
    InvalidLiteral(String),

    /// Indentation error
    IndentationError(String),

    /// Invalid operator
    InvalidOperator(String),

    /// Invalid type
    InvalidType(String),

    /// Lexer error
    LexerError(String),

    /// Custom error message
    Custom(String),
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken { found, expected } => {
                write!(f, "unexpected token `{found}`")?;
                if !expected.is_empty() {
                    write!(f, ", expected ")?;
                    if expected.len() == 1 {
                        write!(f, "`{}`", expected[0])?;
                    } else {
                        write!(f, "one of ")?;
                        for (i, tok) in expected.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "`{tok}`")?;
                        }
                    }
                }
                Ok(())
            }
            Self::UnexpectedEof { expected } => {
                write!(f, "unexpected end of file")?;
                if !expected.is_empty() {
                    write!(f, ", expected ")?;
                    if expected.len() == 1 {
                        write!(f, "`{}`", expected[0])?;
                    } else {
                        write!(f, "one of ")?;
                        for (i, tok) in expected.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "`{tok}`")?;
                        }
                    }
                }
                Ok(())
            }
            Self::InvalidLiteral(msg) => write!(f, "invalid literal: {msg}"),
            Self::IndentationError(msg) => write!(f, "indentation error: {msg}"),
            Self::InvalidOperator(msg) => write!(f, "invalid operator: {msg}"),
            Self::InvalidType(msg) => write!(f, "invalid type: {msg}"),
            Self::LexerError(msg) => write!(f, "lexer error: {msg}"),
            Self::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

/// A collection of parse errors.
#[derive(Debug, Default)]
pub struct ParseErrors {
    errors: Vec<ParseError>,
}

impl ParseErrors {
    /// Create a new empty error collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an error to the collection.
    pub fn push(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    /// Returns true if there are no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Iterate over the errors.
    pub fn iter(&self) -> impl Iterator<Item = &ParseError> {
        self.errors.iter()
    }

    /// Convert to a vector of errors.
    pub fn into_vec(self) -> Vec<ParseError> {
        self.errors
    }
}

impl IntoIterator for ParseErrors {
    type Item = ParseError;
    type IntoIter = std::vec::IntoIter<ParseError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a> IntoIterator for &'a ParseErrors {
    type Item = &'a ParseError;
    type IntoIter = std::slice::Iter<'a, ParseError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}

impl Extend<ParseError> for ParseErrors {
    fn extend<T: IntoIterator<Item = ParseError>>(&mut self, iter: T) {
        self.errors.extend(iter);
    }
}
