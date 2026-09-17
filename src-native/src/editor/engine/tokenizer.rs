// Pure Rust Syntax Tokenizer for Monaco Editor Engine.
// Provides fast token classification for Rust, TypeScript, Python, JSON, TOML, Markdown, and Shell.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Type,
    Function,
    String,
    Number,
    Comment,
    Operator,
    Macro,
    Punctuation,
    Plain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenSpan {
    pub token_type: TokenType,
    pub text: String,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize_line(line: &str, language: &str) -> Vec<TokenSpan> {
        if line.is_empty() {
            return Vec::new();
        }

        // Performance guard (VS Code editor.maxTokenizationLineLength):
        // Lines longer than 20,000 chars or plain text are treated as Plain text
        // to avoid millions of substring allocations and UI thread hangs.
        if language == "plaintext" || line.len() > 20_000 {
            return vec![TokenSpan {
                token_type: TokenType::Plain,
                text: line.to_string(),
                start_col: 0,
                end_col: line.len(),
            }];
        }

        let trimmed = line.trim_start();
        let indent_len = line.len() - trimmed.len();

        let mut spans = Vec::new();
        if indent_len > 0 {
            spans.push(TokenSpan {
                token_type: TokenType::Plain,
                text: line[..indent_len].to_string(),
                start_col: 0,
                end_col: indent_len,
            });
        }

        // Line-comment detection
        if trimmed.starts_with("//")
            || ((language == "python" || language == "toml" || language == "bash")
                && trimmed.starts_with('#'))
        {
            spans.push(TokenSpan {
                token_type: TokenType::Comment,
                text: trimmed.to_string(),
                start_col: indent_len,
                end_col: line.len(),
            });
            return spans;
        }

        let chars: Vec<char> = line[indent_len..].chars().collect();
        let mut idx = 0;
        let base_col = indent_len;

        while idx < chars.len() {
            let c = chars[idx];

            // Whitespace
            if c.is_whitespace() {
                let start = idx;
                while idx < chars.len() && chars[idx].is_whitespace() {
                    idx += 1;
                }
                let text: String = chars[start..idx].iter().collect();
                spans.push(TokenSpan {
                    token_type: TokenType::Plain,
                    text,
                    start_col: base_col + start,
                    end_col: base_col + idx,
                });
                continue;
            }

            // Comments
            if (c == '/' && idx + 1 < chars.len() && chars[idx + 1] == '/')
                || ((language == "python" || language == "toml" || language == "bash") && c == '#')
            {
                let text: String = chars[idx..].iter().collect();
                spans.push(TokenSpan {
                    token_type: TokenType::Comment,
                    text,
                    start_col: base_col + idx,
                    end_col: base_col + chars.len(),
                });
                break;
            }

            // String literals
            if c == '"' || c == '\'' || c == '`' {
                let quote = c;
                let start = idx;
                idx += 1;
                let mut escaped = false;
                while idx < chars.len() {
                    let cur = chars[idx];
                    if escaped {
                        escaped = false;
                    } else if cur == '\\' {
                        escaped = true;
                    } else if cur == quote {
                        idx += 1;
                        break;
                    }
                    idx += 1;
                }
                let text: String = chars[start..idx].iter().collect();
                spans.push(TokenSpan {
                    token_type: TokenType::String,
                    text,
                    start_col: base_col + start,
                    end_col: base_col + idx,
                });
                continue;
            }

            // Numbers
            if c.is_ascii_digit() {
                let start = idx;
                while idx < chars.len()
                    && (chars[idx].is_alphanumeric() || chars[idx] == '.' || chars[idx] == '_')
                {
                    idx += 1;
                }
                let text: String = chars[start..idx].iter().collect();
                spans.push(TokenSpan {
                    token_type: TokenType::Number,
                    text,
                    start_col: base_col + start,
                    end_col: base_col + idx,
                });
                continue;
            }

            // Identifiers / Keywords / Types
            if c.is_alphabetic() || c == '_' {
                let start = idx;
                while idx < chars.len() && (chars[idx].is_alphanumeric() || chars[idx] == '_') {
                    idx += 1;
                }
                let mut word: String = chars[start..idx].iter().collect();

                // Macro check in Rust (e.g. println!)
                let is_macro = if idx < chars.len() && chars[idx] == '!' {
                    idx += 1;
                    word.push('!');
                    true
                } else {
                    false
                };

                let token_type = if is_macro {
                    TokenType::Macro
                } else if Self::is_keyword(&word, language) {
                    TokenType::Keyword
                } else if Self::is_type(&word, language) {
                    TokenType::Type
                } else if idx < chars.len() && chars[idx] == '(' {
                    TokenType::Function
                } else {
                    TokenType::Plain
                };

                spans.push(TokenSpan {
                    token_type,
                    text: word,
                    start_col: base_col + start,
                    end_col: base_col + idx,
                });
                continue;
            }

            // Operators & Punctuation
            let start = idx;
            let token_type = match c {
                '{' | '}' | '(' | ')' | '[' | ']' | ';' | ',' | '.' => {
                    idx += 1;
                    TokenType::Punctuation
                }
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '%' | '?'
                | ':' | '~' => {
                    while idx < chars.len() && "+-*/=<>!&|^%?:~".contains(chars[idx]) {
                        idx += 1;
                    }
                    TokenType::Operator
                }
                _ => {
                    idx += 1;
                    TokenType::Plain
                }
            };

            let text: String = chars[start..idx].iter().collect();
            spans.push(TokenSpan {
                token_type,
                text,
                start_col: base_col + start,
                end_col: base_col + idx,
            });
        }

        spans
    }

    fn is_keyword(word: &str, language: &str) -> bool {
        match language {
            "rust" => matches!(
                word,
                "as" | "async"
                    | "await"
                    | "break"
                    | "const"
                    | "continue"
                    | "crate"
                    | "dyn"
                    | "else"
                    | "enum"
                    | "extern"
                    | "false"
                    | "fn"
                    | "for"
                    | "if"
                    | "impl"
                    | "in"
                    | "let"
                    | "loop"
                    | "match"
                    | "mod"
                    | "move"
                    | "mut"
                    | "pub"
                    | "ref"
                    | "return"
                    | "self"
                    | "static"
                    | "struct"
                    | "super"
                    | "trait"
                    | "true"
                    | "type"
                    | "unsafe"
                    | "use"
                    | "where"
                    | "while"
            ),
            "typescript" | "javascript" => matches!(
                word,
                "async"
                    | "await"
                    | "break"
                    | "case"
                    | "catch"
                    | "class"
                    | "const"
                    | "continue"
                    | "debugger"
                    | "default"
                    | "delete"
                    | "do"
                    | "else"
                    | "export"
                    | "extends"
                    | "false"
                    | "finally"
                    | "for"
                    | "function"
                    | "if"
                    | "import"
                    | "in"
                    | "instanceof"
                    | "let"
                    | "new"
                    | "null"
                    | "return"
                    | "super"
                    | "switch"
                    | "this"
                    | "throw"
                    | "true"
                    | "try"
                    | "typeof"
                    | "var"
                    | "void"
                    | "while"
                    | "with"
                    | "yield"
                    | "interface"
                    | "type"
                    | "enum"
                    | "implements"
            ),
            "python" => matches!(
                word,
                "False"
                    | "None"
                    | "True"
                    | "and"
                    | "as"
                    | "assert"
                    | "async"
                    | "await"
                    | "break"
                    | "class"
                    | "continue"
                    | "def"
                    | "del"
                    | "elif"
                    | "else"
                    | "except"
                    | "finally"
                    | "for"
                    | "from"
                    | "global"
                    | "if"
                    | "import"
                    | "in"
                    | "is"
                    | "lambda"
                    | "nonlocal"
                    | "not"
                    | "or"
                    | "pass"
                    | "raise"
                    | "return"
                    | "try"
                    | "while"
                    | "with"
                    | "yield"
            ),
            _ => false,
        }
    }

    fn is_type(word: &str, language: &str) -> bool {
        match language {
            "rust" => {
                matches!(
                    word,
                    "bool"
                        | "char"
                        | "i8"
                        | "i16"
                        | "i32"
                        | "i64"
                        | "i128"
                        | "isize"
                        | "u8"
                        | "u16"
                        | "u32"
                        | "u64"
                        | "u128"
                        | "usize"
                        | "f32"
                        | "f64"
                        | "str"
                        | "String"
                        | "Option"
                        | "Result"
                        | "Vec"
                        | "Box"
                        | "Rc"
                        | "Arc"
                        | "Mutex"
                        | "RefCell"
                        | "Some"
                        | "None"
                        | "Ok"
                        | "Err"
                        | "Self"
                ) || (word
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
                    && !word.ends_with('!'))
            }
            "typescript" | "javascript" => {
                matches!(
                    word,
                    "string"
                        | "number"
                        | "boolean"
                        | "any"
                        | "void"
                        | "never"
                        | "unknown"
                        | "object"
                        | "Promise"
                        | "Array"
                        | "Map"
                        | "Set"
                        | "Record"
                ) || word
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
            }
            "python" => matches!(
                word,
                "int"
                    | "float"
                    | "str"
                    | "bool"
                    | "list"
                    | "dict"
                    | "set"
                    | "tuple"
                    | "Optional"
                    | "Union"
                    | "Any"
                    | "List"
                    | "Dict"
                    | "Set"
                    | "Tuple"
            ),
            _ => word
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_rust_line() {
        let spans = Tokenizer::tokenize_line("pub fn test_run() -> bool {", "rust");
        let types: Vec<TokenType> = spans.iter().map(|s| s.token_type).collect();
        assert!(types.contains(&TokenType::Keyword)); // pub, fn
        assert!(types.contains(&TokenType::Function)); // test_run
        assert!(types.contains(&TokenType::Type)); // bool
        assert!(types.contains(&TokenType::Operator)); // ->
        assert!(types.contains(&TokenType::Punctuation)); // (, ), {
    }

    #[test]
    fn test_tokenize_comment_and_string() {
        let spans = Tokenizer::tokenize_line("    // this is a comment", "rust");
        assert_eq!(spans[1].token_type, TokenType::Comment);

        let spans_str = Tokenizer::tokenize_line("let x = \"hello world\";", "rust");
        assert!(spans_str.iter().any(|s| s.token_type == TokenType::String));
    }
}
