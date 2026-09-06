//! Deterministic structural compression of tool/function schemas.
//!
//! Turns an OpenAI-style tool array (each ~500–2000 tokens of JSON Schema) into
//! a Hermes-style compact signature block (~30–60 tokens each). No model, no
//! embeddings, no retrieval — pure structure walking, so it is fast, reversible
//! (`full_schema` keeps the original), and trivially testable.
//!
//! Example
//! -------
//! ```text
//! {"type":"function","function":{"name":"edit_file","description":"Replace a
//!  string in a file. The old_string must be unique.","parameters":{"type":
//!  "object","properties":{"path":{"type":"string"},"old_string":{"type":
//!  "string"},"replace_all":{"type":"boolean"}},"required":["path","old_string"]}}}
//! ```
//! becomes
//! ```text
//! edit_file(path: str!, old_string: str!, replace_all?: bool) — Replace a string in a file.
//! ```

use serde_json::Value;
use std::collections::BTreeMap;

/// One tool reduced to a compact, human- and model-readable signature, with the
/// original schema retained for on-demand rehydration.
#[derive(Debug, Clone)]
pub struct ToolDigest {
    pub name: String,
    /// `name(arg: type!, arg?: type) — first sentence of the description`
    pub signature: String,
    /// The verbatim original entry, so `expand` can hand the model the full
    /// schema when it actually needs it.
    pub full_schema: Value,
}

/// Compress every tool in an OpenAI `tools` array. Entries that don't look like
/// a function tool are passed through untouched in `full_schema` with an empty
/// signature (the caller decides whether to keep them).
pub fn digest_tools(tools: &[Value]) -> Vec<ToolDigest> {
    tools.iter().map(digest_one).collect()
}

fn digest_one(tool: &Value) -> ToolDigest {
    // OpenAI shape: {"type":"function","function":{name,description,parameters}}
    // Anthropic shape: {name,description,input_schema}
    let func = tool.get("function").unwrap_or(tool);
    let name = func
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("<unnamed>")
        .to_string();
    let desc = func
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let schema = func
        .get("parameters")
        .or_else(|| func.get("input_schema"))
        .cloned()
        .unwrap_or(Value::Null);

    let params = signature_params(&schema);
    let purpose = first_sentence(desc);
    let signature = if purpose.is_empty() {
        format!("{name}({params})")
    } else {
        format!("{name}({params}) — {purpose}")
    };

    ToolDigest {
        name,
        signature,
        full_schema: tool.clone(),
    }
}

/// Render a JSON-Schema `object` as `a: type!, b?: type` (required marked `!`).
fn signature_params(schema: &Value) -> String {
    let Some(props) = schema.get("properties").and_then(Value::as_object) else {
        return String::new();
    };
    let required: Vec<&str> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();

    // BTreeMap so output is deterministic regardless of serde map order.
    let ordered: BTreeMap<&String, &Value> = props.iter().collect();
    let mut out = Vec::with_capacity(ordered.len());
    for (key, spec) in ordered {
        let ty = short_type(spec);
        if required.contains(&key.as_str()) {
            out.push(format!("{key}: {ty}!")); // `!` after the type = required
        } else {
            out.push(format!("{key}?: {ty}")); // `?` after the name = optional
        }
    }
    out.join(", ")
}

/// Collapse a property spec to one short token: `str`, `int`, `bool`, `str[]`,
/// `a|b|c` for small enums, `obj` for nested objects.
fn short_type(spec: &Value) -> String {
    if let Some(en) = spec.get("enum").and_then(Value::as_array) {
        let vals: Vec<String> = en
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string).or_else(|| Some(v.to_string())))
            .collect();
        if vals.len() <= 6 {
            return vals.join("|");
        }
        return format!("enum[{}]", vals.len());
    }
    match spec.get("type").and_then(Value::as_str) {
        Some("string") => "str".into(),
        Some("integer") => "int".into(),
        Some("number") => "num".into(),
        Some("boolean") => "bool".into(),
        Some("object") => "obj".into(),
        Some("array") => {
            let inner = spec
                .get("items")
                .map(short_type)
                .unwrap_or_else(|| "any".into());
            format!("{inner}[]")
        }
        Some(other) => other.into(),
        None => {
            // anyOf / oneOf / no explicit type
            if spec.get("anyOf").is_some() || spec.get("oneOf").is_some() {
                "any".into()
            } else {
                "any".into()
            }
        }
    }
}

/// First sentence (up to the first `. ` or newline), capped so a rambling
/// description can't blow the budget back up.
fn first_sentence(s: &str) -> String {
    let s = s.replace('\n', " ");
    let cut = s
        .find(". ")
        .map(|i| i + 1)
        .or_else(|| s.find('\n'))
        .unwrap_or(s.len());
    let mut out = s[..cut].trim().to_string();
    if out.len() > 160 {
        out.truncate(157);
        out.push('…');
    }
    out
}

/// Rough token estimate for reporting (chars / 4). Not used for correctness.
pub fn approx_tokens(s: &str) -> usize {
    (s.len() / 4).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn edit_tool() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "edit_file",
                "description": "Replace a string in a file. The old_string must be unique.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string"},
                        "old_string": {"type": "string"},
                        "replace_all": {"type": "boolean"}
                    },
                    "required": ["path", "old_string"]
                }
            }
        })
    }

    #[test]
    fn compresses_to_signature() {
        let d = digest_one(&edit_tool());
        assert_eq!(d.name, "edit_file");
        assert_eq!(
            d.signature,
            "edit_file(old_string: str!, path: str!, replace_all?: bool) — Replace a string in a file."
        );
    }

    #[test]
    fn signature_is_far_smaller() {
        let full = serde_json::to_string(&edit_tool()).unwrap();
        let d = digest_one(&edit_tool());
        assert!(approx_tokens(&d.signature) * 3 < approx_tokens(&full));
    }

    #[test]
    fn enums_render_inline() {
        let t = json!({
            "function": {
                "name": "set_mode",
                "description": "Set the mode.",
                "parameters": {
                    "type": "object",
                    "properties": { "mode": {"type": "string", "enum": ["read", "write", "admin"]} },
                    "required": ["mode"]
                }
            }
        });
        assert_eq!(digest_one(&t).signature, "set_mode(mode: read|write|admin!) — Set the mode.");
    }

    #[test]
    fn arrays_and_optionals() {
        let t = json!({
            "function": {
                "name": "grep",
                "description": "Search files",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "pattern": {"type": "string"},
                        "globs": {"type": "array", "items": {"type": "string"}}
                    },
                    "required": ["pattern"]
                }
            }
        });
        assert_eq!(digest_one(&t).signature, "grep(globs?: str[], pattern: str!) — Search files");
    }

    #[test]
    fn full_schema_is_retained() {
        let d = digest_one(&edit_tool());
        assert_eq!(d.full_schema, edit_tool());
    }
}
