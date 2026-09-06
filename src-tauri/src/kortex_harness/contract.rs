//! The Hermes-style tool-calling contract the harness injects, plus a GBNF
//! grammar that constrains a small model to emit either prose or a single
//! well-formed `<tool_call>`.
//!
//! Rationale: a 3B-active model driving a coding agent is unreliable at
//! free-form JSON tool calls. Giving it (a) a fixed, terse convention and
//! (b) a grammar that makes malformed calls unrepresentable is what turns
//! "flaky" into "works". This is the same discipline Nous' Hermes format uses.

/// System-prompt fragment. Prepended ahead of the compressed tool block.
pub const CONTRACT: &str = r#"# Tools

The tools available to you are listed below as compact signatures:

  name(arg: type!, arg?: type) — purpose

`arg: type!` is required; `arg?: type` is optional. `type` is one of:
str, int, num, bool, obj, `type[]` for a list, or `a|b|c` for an enumerated
choice.

To call a tool, emit exactly one tool call and then stop, in this form:

<tool_call>{"name": "<tool>", "arguments": { ... }}</tool_call>

Rules:
- One tool call per turn. Wait for the <tool_response> before the next.
- Arguments must match the signature. Do not invent arguments.
- If you need a tool's full JSON schema, call:  expand({"tool": "<name>"})
- If no tool is needed, reply normally in prose. Never mix prose and a tool call
  in the same turn.
"#;

/// The synthetic tool that rehydrates a full schema on demand. Added to the
/// live `tools` array so native tool-calling can still invoke it.
pub fn expand_tool_def() -> serde_json::Value {
    serde_json::json!({
        "type": "function",
        "function": {
            "name": "expand",
            "description": "Return the full JSON schema for a tool whose signature was compacted. Call this before using a tool if you are unsure of its exact arguments.",
            "parameters": {
                "type": "object",
                "properties": {
                    "tool": { "type": "string", "description": "The tool name to expand." }
                },
                "required": ["tool"]
            }
        }
    })
}

/// Build a GBNF grammar that accepts free prose OR a single tool call whose
/// `name` is one of `tool_names`. Pass to llama-server as `grammar` (or
/// `--grammar`) for constrained decoding.
///
/// Kept intentionally loose on the `arguments` object (any JSON value) — over-
/// constraining per-tool arg shapes is brittle and better handled by validation
/// after the fact.
pub fn tool_call_grammar(tool_names: &[&str]) -> String {
    let names = tool_names
        .iter()
        .map(|n| format!("\"\\\"{}\\\"\"", n.replace('"', "")))
        .collect::<Vec<_>>()
        .join(" | ");
    let name_rule = if names.is_empty() {
        "string".to_string()
    } else {
        format!("( {names} )")
    };

    format!(
        r####"
root        ::= prose | toolcall
prose       ::= [^<] [^<]*                    # any text that isn't a tool call
toolcall    ::= "<tool_call>" ws obj ws "</tool_call>"
obj         ::= "{{" ws "\"name\"" ws ":" ws name ws "," ws "\"arguments\"" ws ":" ws value ws "}}"
name        ::= {name_rule}
value       ::= object | array | string | number | "true" | "false" | "null"
object      ::= "{{" ws ( pair ( ws "," ws pair )* )? ws "}}"
pair        ::= string ws ":" ws value
array       ::= "[" ws ( value ( ws "," ws value )* )? ws "]"
string      ::= "\"" ( [^"\\] | "\\" . )* "\""
number      ::= "-"? ( "0" | [1-9] [0-9]* ) ( "." [0-9]+ )? ( [eE] [-+]? [0-9]+ )?
ws          ::= [ \t\n]*
"####
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_mentions_the_call_form() {
        assert!(CONTRACT.contains("<tool_call>"));
        assert!(CONTRACT.contains("expand("));
    }

    #[test]
    fn grammar_lists_the_names() {
        let g = tool_call_grammar(&["read_file", "edit_file", "bash"]);
        assert!(g.contains(r#""\"read_file\"""#));
        assert!(g.contains(r#""\"bash\"""#));
        assert!(g.contains("toolcall"));
    }

    #[test]
    fn grammar_handles_empty() {
        let g = tool_call_grammar(&[]);
        assert!(g.contains("name        ::= string"));
    }
}
