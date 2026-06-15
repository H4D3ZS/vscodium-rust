//! # ASL Parser
//!
//! A parser for the ARM Architecture Specification Language (ASL).
//!
//! ASL is a domain-specific language used by ARM to formally describe the semantics
//! of their instruction set architectures. This parser handles the complete ASL grammar
//! including its Python-like indentation-based syntax.
//!
//! ## Features
//!
//! - Full ASL grammar support including registers, instructions, and definitions
//! - Indentation-based parsing with proper INDENT/DEDENT handling
//! - Rich error messages with source locations
//! - Zero-copy parsing where possible
//! - XML extraction from ARM specification files (with `xml-extract` feature)
//!
//! ## Example
//!
//! ```rust,ignore
//! use asl_parser::{parse_definitions, parse_instructions, parse_registers};
//!
//! let source = r#"
//! bits(32) MyFunction(bits(32) x)
//!     return x;
//! "#;
//!
//! let defs = parse_definitions(source)?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod error;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod syntax;
pub mod testgen;
pub mod xml;

pub use error::{ParseError, Result};
pub use parser::{parse_definitions, parse_instructions, parse_registers};
pub use syntax::*;
