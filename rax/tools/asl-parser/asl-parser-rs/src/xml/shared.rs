//! Shared pseudocode XML parsing.
//!
//! This module parses `shared_pseudocode.xml` files to extract common ASL functions
//! and definitions used by instructions.

use super::AslChunk;
use once_cell::sync::Lazy;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

// Pre-compiled regexes for performance
static RE_IMPL: Lazy<Regex> = Lazy::new(|| Regex::new(r"impl-\w+\.").unwrap());
static RE_FILE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^#]+#").unwrap());
static RE_TYPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^(enumeration|type)\s+(\S+)").unwrap());
static RE_VAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^(\S+)\s+([a-zA-Z_]\w+);").unwrap());
static RE_ARRAY: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^array\s+(\S+)\s+([a-zA-Z_]\w+)").unwrap());
static RE_ACCESSOR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^(\w\S+)\s+([a-zA-Z_]\w+)\s*$").unwrap());
static RE_ARR_ACCESSOR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^(\w\S+)\s+([a-zA-Z_]\w+)\[").unwrap());

/// Standard library functions to skip (they're built into the language).
const STDLIB_FUNCTIONS: &[&str] = &[
    "shared/functions/common/SInt",
    "shared/functions/common/UInt",
    "shared/functions/common/Ones",
    "shared/functions/common/Zeros",
    "shared/functions/common/IsOnes",
    "shared/functions/common/IsZero",
    "shared/functions/common/SignExtend",
    "shared/functions/common/ZeroExtend",
    "shared/functions/common/Replicate",
    "shared/functions/common/RoundDown",
    "shared/functions/common/RoundUp",
    "shared/functions/common/RoundTowardsZero",
];

/// Read the ARM proprietary notice from notice.xml.
pub fn read_notice(path: &Path) -> Result<String, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read notice.xml: {}", e))?;

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);

    let mut notice = vec!["/".repeat(72), "// Proprietary Notice".to_string()];
    let mut in_para = false;
    let mut para_text = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"para" => {
                in_para = true;
                para_text.clear();
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"para" => {
                if in_para {
                    let para = para_text
                        .replace("&#8217;", "'")
                        .replace("&#8220;", "\"")
                        .replace("&#8221;", "\"")
                        .replace("&#8482;", "(TM)")
                        .replace("&#169;", "(C)")
                        .replace("&#174;", "(R)");
                    for line in para.lines() {
                        notice.push(format!("// {}", line.trim_end()));
                    }
                    in_para = false;
                }
            }
            Ok(Event::Text(e)) if in_para => {
                para_text.push_str(&e.unescape().map_err(|e| format!("XML text error: {}", e))?);
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => {}
        }
    }

    notice.push("/".repeat(72));
    Ok(notice.join("\n"))
}

/// Read shared pseudocode from an XML file.
///
/// Returns a map of chunk names to ASL chunks, plus a set of all defined names.
pub fn read_shared_pseudocode(
    path: &Path,
) -> Result<(HashMap<String, AslChunk>, HashSet<String>), String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read shared_pseudocode: {}", e))?;

    let mut asl: HashMap<String, AslChunk> = HashMap::new();
    let mut names: HashSet<String> = HashSet::new();

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(false);

    let mut current_ps: Option<PsSection> = None;
    let mut in_pstext = false;
    let mut pstext_content = String::new();
    let mut current_links: HashSet<String> = HashSet::new();
    let mut current_anchors: HashSet<String> = HashSet::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"ps" => {
                        // Start of a pseudocode section
                        let mut name = String::new();
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                name = String::from_utf8_lossy(&attr.value)
                                    .replace(".txt", "")
                                    .replace("/instrs", "")
                                    .replace("/Op_", "/");
                            }
                        }
                        current_ps = Some(PsSection { name });
                        current_links.clear();
                        current_anchors.clear();
                    }
                    b"pstext" => {
                        in_pstext = true;
                        pstext_content.clear();
                    }
                    b"a" if in_pstext => {
                        // Dependency link
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"link" {
                                let link = String::from_utf8_lossy(&attr.value).to_string();
                                current_links.insert(clean_link(&link));
                            }
                        }
                    }
                    b"anchor" if in_pstext => {
                        // Definition anchor
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"link" {
                                let anchor = String::from_utf8_lossy(&attr.value).to_string();
                                current_anchors.insert(clean_link(&anchor));
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                match e.name().as_ref() {
                    b"ps" => {
                        if let Some(ps) = current_ps.take() {
                            // Skip stdlib functions
                            if STDLIB_FUNCTIONS.contains(&ps.name.as_str()) {
                                continue;
                            }

                            let mut defs = current_anchors.clone();
                            let mut deps = current_links.clone();

                            // Filter out SEE dependencies (they're just documentation)
                            deps.retain(|d| !d.starts_with("SEE"));

                            // Extract additional definitions from code patterns
                            extract_definitions(&pstext_content, &mut defs, &mut names);

                            // Create chunk
                            let mut chunk =
                                AslChunk::new(ps.name.clone(), pstext_content.clone(), defs, deps);

                            // Apply workarounds
                            apply_workarounds(&mut chunk);

                            // Patch type variable usage
                            chunk.patch_type_var();

                            asl.insert(ps.name, chunk);
                        }
                    }
                    b"pstext" => {
                        in_pstext = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) if in_pstext => {
                pstext_content
                    .push_str(&e.unescape().map_err(|e| format!("XML text error: {}", e))?);
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => {}
        }
    }

    Ok((asl, names))
}

struct PsSection {
    name: String,
}

/// Clean a link reference (remove impl- prefixes and file references).
fn clean_link(link: &str) -> String {
    let link = RE_IMPL.replace_all(link, "").to_string();
    RE_FILE.replace_all(&link, "").to_string()
}

/// Extract additional definitions from code patterns.
fn extract_definitions(code: &str, defs: &mut HashSet<String>, names: &mut HashSet<String>) {
    // Type and enumeration definitions
    for cap in RE_TYPE.captures_iter(code) {
        let name = cap[2].to_string();
        defs.insert(name.clone());
        names.insert(name);
    }

    // Variable definitions
    for cap in RE_VAR.captures_iter(code) {
        if &cap[1] != "type" {
            let name = cap[2].to_string();
            defs.insert(name.clone());
            names.insert(name);
        }
    }

    // Array definitions
    for cap in RE_ARRAY.captures_iter(code) {
        let name = format!("{}[", &cap[2]);
        defs.insert(name.clone());
        names.insert(name);
    }

    // Variable accessors
    for cap in RE_ACCESSOR.captures_iter(code) {
        let name = cap[2].to_string();
        defs.insert(name.clone());
        names.insert(name);
    }

    // Array accessors
    for cap in RE_ARR_ACCESSOR.captures_iter(code) {
        let name = format!("{}[", &cap[2]);
        defs.insert(name.clone());
        names.insert(name);
    }
}

/// Apply various workarounds for known issues in the ARM XML.
fn apply_workarounds(chunk: &mut AslChunk) {
    // Patch SCTLR[] definition
    if chunk.name == "aarch64/functions/sysregisters/SCTLR" {
        chunk.code = chunk.code.replace("bits(32) r;", "bits(64) r;");
    }

    // Patch AArch64.CheckUnallocatedSystemAccess
    if chunk.name == "aarch64/functions/system/AArch64.CheckUnallocatedSystemAccess" {
        chunk.code = chunk
            .code
            .replace("bits(2) op0,", "bits(2) el, bits(2) op0,");
    }

    // Patch AArch64.CheckSystemAccess
    if chunk.name == "aarch64/functions/system/AArch64.CheckSystemAccess" {
        chunk.code = chunk.code.replace(
            "AArch64.CheckSVESystemRegisterTraps(op0, op1, crn, crm, op2);",
            "AArch64.CheckSVESystemRegisterTraps(op0, op1, crn, crm, op2, read);",
        );
    }

    // Add PSTATE definition/dependency
    if chunk.name == "shared/functions/system/PSTATE" {
        chunk.defs.insert("PSTATE".to_string());
    }
    if chunk.code.contains("PSTATE") {
        chunk.deps.insert("PSTATE".to_string());
    }

    // ProcState SP field workaround
    if chunk.name == "shared/functions/system/ProcState" {
        chunk.deps.remove("SP");
        chunk.deps.remove("SP.write.none");
    }

    // Unpredictable PSTATE workaround
    if chunk.defs.contains("Unpredictable_WBOVERLAPST") {
        chunk.deps.remove("PSTATE");
    }
}

/// Convert bit slices from angle bracket to square bracket syntax.
#[allow(dead_code)]
pub fn patch_slices(code: &str) -> String {
    let re_parts = r"[0-9a-zA-Z_+*\-()[\]., ]+";
    let re_part = format!(r"{}(:{})?", re_parts, re_parts);
    let re_full = format!(r"<({})(?:,({}))*>", re_part, re_part);

    let re = Regex::new(&re_full).unwrap();
    let mut result = code.to_string();

    // Apply twice to handle nested slices
    for _ in 0..2 {
        result = re.replace_all(&result, "[$1]").to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_link() {
        assert_eq!(clean_link("impl-shared.Foo"), "Foo");
        assert_eq!(clean_link("file.xml#Bar"), "Bar");
        assert_eq!(clean_link("impl-aarch64.file.xml#Baz"), "Baz");
    }

    #[test]
    fn test_patch_slices() {
        assert_eq!(patch_slices("x<7:0>"), "x[7:0]");
        assert_eq!(patch_slices("x<7>"), "x[7]");
        assert_eq!(patch_slices("x<7:0, 15:8>"), "x[7:0, 15:8]");
    }
}
