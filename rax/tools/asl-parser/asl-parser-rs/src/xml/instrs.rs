//! Instruction XML parsing.
//!
//! This module parses ARM instruction XML files to extract:
//! - Instruction encodings and fields
//! - Decode ASL code
//! - Execute ASL code

use super::{AslChunk, EncodingField, ExtractConfig, InstructionEncoding, XmlInstruction};
use once_cell::sync::Lazy;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

// Pre-compiled regexes for performance
static RE_SPLIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\w+)[<\[]").unwrap());
static RE_IMPL: Lazy<Regex> = Lazy::new(|| Regex::new(r"impl-\w+\.").unwrap());
static RE_FILE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^#]+#").unwrap());

/// Read all instructions from a directory containing ARM XML files.
pub fn read_instructions(
    dir: &Path,
    chunks: &HashMap<String, AslChunk>,
    config: &ExtractConfig,
) -> Result<Vec<XmlInstruction>, String> {
    let mut instructions = Vec::new();

    // Read all XML files in the directory
    let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "xml") {
            let filename = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();

            // Skip non-instruction files
            if filename == "onebigfile"
                || filename == "shared_pseudocode"
                || filename == "notice"
                || filename.contains("encindex")
            {
                continue;
            }

            match read_instruction_file(&path, chunks, config) {
                Ok(Some(instr)) => {
                    // Filter by architecture if specified
                    if !config.architectures.is_empty() {
                        let valid_isets: Vec<&str> = config
                            .architectures
                            .iter()
                            .flat_map(|arch| match arch.as_str() {
                                "AArch32" => vec!["T16", "T32", "A32"],
                                "AArch64" => vec!["A64"],
                                _ => vec![],
                            })
                            .collect();

                        let filtered_encs: Vec<_> = instr
                            .encodings
                            .iter()
                            .filter(|e| {
                                valid_isets.is_empty()
                                    || valid_isets.contains(&e.instruction_set.as_str())
                            })
                            .cloned()
                            .collect();

                        if !filtered_encs.is_empty() {
                            let mut filtered_instr = instr;
                            filtered_instr.encodings = filtered_encs;
                            instructions.push(filtered_instr);
                        }
                    } else {
                        instructions.push(instr);
                    }
                }
                Ok(None) => {
                    // Alias or skipped instruction
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(instructions)
}

/// Read a single instruction from an XML file.
fn read_instruction_file(
    path: &Path,
    chunks: &HashMap<String, AslChunk>,
    config: &ExtractConfig,
) -> Result<Option<XmlInstruction>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(false);

    // Track context
    let mut in_execute = false;
    let mut in_postdecode = false;
    let mut in_iclass = false;
    let mut in_regdiagram = false;
    let mut in_box = false;
    let mut in_decode_ps = false;

    // Current state
    let mut execute_code = String::new();
    let mut execute_name = String::new();
    let mut postdecode_code = String::new();
    let mut postdecode_name = String::new();

    let mut current_iclass_isa = String::new();
    let mut current_fields: Vec<EncodingField> = Vec::new();
    let mut current_decode_code = String::new();
    let mut current_decode_name = String::new();
    let mut current_enc_name = String::new();
    let mut current_form = String::new();

    // Box state
    let mut box_hi: u32 = 0;
    let mut box_width: u32 = 1;
    let mut box_name = String::new();
    let mut box_usename = false;
    let mut box_psbits = String::new();
    let mut box_consts = String::new();

    // Encodings collected
    let mut encodings: Vec<InstructionEncoding> = Vec::new();

    // Dependencies tracking
    let mut current_links: HashSet<String> = HashSet::new();
    let mut current_anchors: HashSet<String> = HashSet::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"pstext" => {
                        // Check section attribute
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"section" {
                                let section = String::from_utf8_lossy(&attr.value);
                                if section == "Execute" {
                                    in_execute = true;
                                    execute_code.clear();
                                    current_links.clear();
                                    current_anchors.clear();
                                } else if section == "Postdecode" {
                                    in_postdecode = true;
                                    postdecode_code.clear();
                                } else if section == "Decode" {
                                    in_decode_ps = true;
                                    current_decode_code.clear();
                                }
                            }
                        }
                    }
                    b"ps" => {
                        let mut ps_name = String::new();
                        let mut ps_secttype = String::new();

                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"name" => {
                                    ps_name = String::from_utf8_lossy(&attr.value)
                                        .replace(".txt", "")
                                        .replace("/instrs", "")
                                        .replace("/Op_", "/");
                                }
                                b"secttype" => {
                                    ps_secttype = String::from_utf8_lossy(&attr.value).to_string();
                                }
                                _ => {}
                            }
                        }

                        // Use secttype to determine which section this belongs to
                        if ps_secttype == "Operation" {
                            execute_name = ps_name;
                        } else if ps_secttype == "Postdecode" {
                            postdecode_name = ps_name;
                        } else if in_execute {
                            execute_name = ps_name;
                        } else if in_postdecode {
                            postdecode_name = ps_name;
                        } else if !ps_name.is_empty() {
                            current_decode_name = ps_name;
                        }
                    }
                    b"iclass" => {
                        in_iclass = true;
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"isa" {
                                current_iclass_isa =
                                    String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    b"regdiagram" => {
                        in_regdiagram = true;
                        current_fields.clear();
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"form" => {
                                    current_form = String::from_utf8_lossy(&attr.value).to_string();
                                }
                                b"psname" => {
                                    current_enc_name =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                }
                                _ => {}
                            }
                        }
                    }
                    b"box" if in_regdiagram => {
                        in_box = true;
                        box_hi = 0;
                        box_width = 1;
                        box_name = "_".to_string();
                        box_usename = false;
                        box_psbits.clear();
                        box_consts.clear();

                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"hibit" => {
                                    if let Ok(v) =
                                        String::from_utf8_lossy(&attr.value).parse::<u32>()
                                    {
                                        box_hi = v;
                                    }
                                }
                                b"width" => {
                                    if let Ok(v) =
                                        String::from_utf8_lossy(&attr.value).parse::<u32>()
                                    {
                                        box_width = v;
                                    }
                                }
                                b"name" => {
                                    box_name = String::from_utf8_lossy(&attr.value)
                                        .replace("&lt;", "<")
                                        .replace("&gt;", ">")
                                        .replace("&amp;", "&");
                                }
                                b"usename" => {
                                    box_usename = String::from_utf8_lossy(&attr.value) == "1";
                                }
                                b"psbits" => {
                                    box_psbits = String::from_utf8_lossy(&attr.value).to_string();
                                }
                                _ => {}
                            }
                        }
                    }
                    b"c" if in_box => {
                        // Will collect text content
                    }
                    b"a" => {
                        // Link - collect dependency
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"link" {
                                let link = String::from_utf8_lossy(&attr.value).to_string();
                                if !link.starts_with("SEE") {
                                    current_links.insert(clean_link(&link));
                                }
                            }
                        }
                    }
                    b"anchor" => {
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
                    b"pstext" => {
                        in_execute = false;
                        in_postdecode = false;
                        in_decode_ps = false;
                    }
                    b"iclass" => {
                        // Finalize the encoding
                        if in_iclass && !current_fields.is_empty() {
                            let is_t16 = current_form == "16";
                            let instruction_set = if is_t16 {
                                "T16".to_string()
                            } else {
                                current_iclass_isa.clone()
                            };

                            // Pad T16 with zeros
                            if is_t16 {
                                current_fields.push(EncodingField {
                                    hi: 15,
                                    lo: 0,
                                    name: "_".to_string(),
                                    split: false,
                                    consts: "0".repeat(16),
                                });
                            }

                            let mut decode_chunk = AslChunk::new(
                                current_decode_name.clone(),
                                current_decode_code.clone(),
                                current_anchors.clone(),
                                current_links.clone(),
                            );
                            decode_chunk.patch_type_var();

                            encodings.push(InstructionEncoding {
                                name: if instruction_set.starts_with('T')
                                    || instruction_set == "A32"
                                {
                                    current_decode_name.clone()
                                } else {
                                    current_enc_name.clone()
                                },
                                instruction_set,
                                fields: current_fields.clone(),
                                decode: decode_chunk,
                            });
                        }
                        in_iclass = false;
                        current_fields.clear();
                        current_links.clear();
                        current_anchors.clear();
                    }
                    b"regdiagram" => {
                        in_regdiagram = false;
                    }
                    b"box" if in_box => {
                        // Finalize box/field
                        let field_name = if box_usename {
                            // Workaround: rename 'type' to 'type1'
                            if box_name == "type" {
                                "type1".to_string()
                            } else {
                                box_name.clone()
                            }
                        } else {
                            "_".to_string()
                        };

                        let lo = box_hi - box_width + 1;
                        let ignore = !box_psbits.is_empty() && box_psbits.chars().all(|c| c == 'x');

                        // Normalize constraint patterns: (0), (1) become x (don't care)
                        let normalized_consts: String = box_consts
                            .replace("(0)", "x")
                            .replace("(1)", "x")
                            .replace("(x)", "x");

                        let consts = if ignore {
                            "x".repeat(box_width as usize)
                        } else if normalized_consts.is_empty() {
                            "x".repeat(box_width as usize)
                        } else {
                            normalized_consts
                        };

                        // Handle split fields (e.g., imm8<7:1> and imm8<0>)
                        let (final_name, split) = if let Some(cap) = RE_SPLIT.captures(&field_name)
                        {
                            (cap[1].to_string(), true)
                        } else {
                            (field_name, false)
                        };

                        // Try to merge with previous field if it's a split continuation
                        if split && !current_fields.is_empty() {
                            let last = current_fields.last_mut().unwrap();
                            if last.split && last.name == final_name && last.lo == box_hi + 1 {
                                // Merge: extend the previous field
                                last.hi = box_hi;
                                last.consts = format!("{}{}", last.consts, consts);
                                in_box = false;
                                continue;
                            }
                        }

                        // Discard != patterns
                        let final_consts = if consts.starts_with("!=") {
                            "x".repeat(box_width as usize)
                        } else {
                            consts
                        };

                        current_fields.push(EncodingField {
                            hi: box_hi,
                            lo,
                            name: final_name,
                            split,
                            consts: final_consts,
                        });

                        in_box = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().map_err(|e| format!("XML text error: {}", e))?;
                if in_execute {
                    execute_code.push_str(&text);
                } else if in_postdecode {
                    postdecode_code.push_str(&text);
                } else if in_decode_ps {
                    current_decode_code.push_str(&text);
                } else if in_box {
                    // Clean up whitespace from opcode pattern
                    for c in text.chars() {
                        if !c.is_whitespace() {
                            box_consts.push(c);
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => {}
        }
    }

    // No execute section means this is an alias - skip it
    if execute_code.is_empty() {
        return Ok(None);
    }

    // Apply include/exclude filters
    if let Some(ref include) = config.include_regex {
        let re = Regex::new(include).map_err(|e| format!("Invalid include regex: {}", e))?;
        if !re.is_match(&execute_name) {
            return Ok(None);
        }
    }
    if let Some(ref exclude) = config.exclude_regex {
        let re = Regex::new(exclude).map_err(|e| format!("Invalid exclude regex: {}", e))?;
        if re.is_match(&execute_name) {
            return Ok(None);
        }
    }

    // Process execute code
    let (conditional, execute_code) = if config.demangle {
        demangle_execute(&execute_code)
    } else {
        (false, execute_code)
    };

    let mut execute_chunk = AslChunk::new(
        execute_name.clone(),
        execute_code,
        HashSet::new(),
        HashSet::new(),
    );
    execute_chunk.patch_type_var();
    execute_chunk.patch_dependencies(chunks);

    let post_decode = if !postdecode_code.is_empty() {
        let mut chunk = AslChunk::new(
            postdecode_name,
            postdecode_code,
            HashSet::new(),
            HashSet::new(),
        );
        chunk.patch_type_var();
        chunk.patch_dependencies(chunks);
        Some(chunk)
    } else {
        None
    };

    Ok(Some(XmlInstruction {
        name: execute_name,
        encodings,
        post_decode,
        conditional,
        execute: execute_chunk,
    }))
}

/// Clean a link reference.
fn clean_link(link: &str) -> String {
    let link = RE_IMPL.replace_all(link, "").to_string();
    RE_FILE.replace_all(&link, "").to_string()
}

/// Demangle execute ASL code.
///
/// Handles patterns like:
/// ```text
/// if ConditionPassed() then
///     EncodingSpecificOperations();
///     ...
/// ```
fn demangle_execute(code: &str) -> (bool, String) {
    let lines: Vec<&str> = code.lines().collect();
    if lines.is_empty() {
        return (false, code.to_string());
    }

    let mut conditional = false;
    let mut result_lines = lines.clone();

    // Check for conditional pattern
    if lines[0] == "if ConditionPassed() then" {
        conditional = true;
        result_lines = result_lines[1..]
            .iter()
            .map(|l| if l.starts_with("    ") { &l[4..] } else { *l })
            .collect();
    }

    // Remove EncodingSpecificOperations() call
    if !result_lines.is_empty() && result_lines[0].starts_with("EncodingSpecificOperations();") {
        let rest = result_lines[0]["EncodingSpecificOperations();".len()..].trim();
        if rest.is_empty() {
            result_lines = result_lines[1..].to_vec();
        } else {
            result_lines[0] = rest;
        }
    }

    (conditional, result_lines.join("\n"))
}
