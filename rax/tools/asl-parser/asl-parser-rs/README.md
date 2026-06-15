# ASL Parser (Rust)

A native Rust parser for the ARM Architecture Specification Language (ASL).

This is a complete end-to-end Rust implementation that can:
1. Download ARM XML specification files from ARM's developer website
2. Extract ASL source code from the XML files
3. Parse the ASL into a structured AST

## Features

- **Full ASL grammar support**: Parses definitions, instructions, and registers
- **Indentation-aware lexer**: Handles Python-like significant whitespace with INDENT/DEDENT tokens
- **Comprehensive AST**: Rich type system mirroring the original Haskell implementation
- **Beautiful error messages**: Uses `miette` for rich diagnostic output
- **CLI tool**: Parse files and inspect results from the command line
- **Optional JSON output**: Serialize AST to JSON with the `serde` feature
- **XML extraction**: Download and parse ARM XML specs directly (with `xml-extract` feature)

## Usage

### As a Library

```rust
use asl_parser::{parse_definitions, parse_instructions, parse_registers};

// Parse function definitions
let source = r#"
bits(32) MyFunction(bits(32) x)
    return x;
"#;

match parse_definitions(source) {
    Ok(defs) => {
        for def in defs {
            println!("{:?}", def);
        }
    }
    Err(e) => eprintln!("Parse error: {}", e),
}
```

### As a CLI Tool

```bash
# Parse definitions
asl-parser definitions path/to/file.asl

# Parse instructions
asl-parser instructions path/to/instrs.asl

# Parse registers
asl-parser registers path/to/regs.asl

# Debug lexer output
asl-parser lex path/to/file.asl

# Get JSON output (with serde feature)
asl-parser definitions --format json path/to/file.asl
```

### XML Extraction (with xml-extract feature)

```bash
# Download ARM XML specification files
asl-parser download --output xml --version 00bet9

# Extract ASL from downloaded XML
asl-parser extract --input xml --output asl --demangle

# Full pipeline: download, extract, and parse in one command
asl-parser pipeline --workdir . --version 00bet9 --demangle
```

## Building

```bash
cargo build --release
```

With JSON support:
```bash
cargo build --release --features serde
```

With XML extraction support (for downloading and extracting from ARM specs):
```bash
cargo build --release --features xml-extract
```

With all features:
```bash
cargo build --release --features serde,xml-extract
```

## Project Structure

```
src/
├── lib.rs          # Library entry point
├── main.rs         # CLI entry point
├── syntax.rs       # AST type definitions
├── lexer.rs        # Tokenizer with indentation handling
├── span.rs         # Source location utilities
├── error.rs        # Error types and diagnostics
├── parser/
│   ├── mod.rs      # Parser core
│   ├── expr.rs     # Expression parsing
│   ├── stmt.rs     # Statement parsing
│   ├── types.rs    # Type parsing
│   ├── defs.rs     # Definition parsing
│   ├── instr.rs    # Instruction parsing
│   └── regs.rs     # Register parsing
└── xml/            # XML extraction (requires xml-extract feature)
    ├── mod.rs      # Module and common types
    ├── download.rs # HTTP download and archive extraction
    ├── shared.rs   # Shared pseudocode XML parsing
    ├── instrs.rs   # Instruction XML parsing
    └── regs.rs     # System register XML parsing
```

## AST Overview

The AST closely follows the structure of the original Haskell implementation:

- **Expressions**: Literals, variables, operators, function calls, slices, etc.
- **Statements**: Variable declarations, assignments, control flow, etc.
- **Types**: Named types, parameterized types (`bits(N)`), register types, etc.
- **Definitions**: Functions, procedures, getters, setters, type definitions
- **Instructions**: Encodings, decode/execute blocks
- **Registers**: Single registers and register arrays with fields

## Indentation Handling

ASL uses significant indentation like Python. The lexer handles this by:

1. Tracking indentation levels on a stack
2. Emitting synthetic `INDENT` tokens when indentation increases
3. Emitting synthetic `DEDENT` tokens when indentation decreases
4. Ignoring newlines inside bracketed expressions

## License

BSD-3-Clause (same as the original project)
