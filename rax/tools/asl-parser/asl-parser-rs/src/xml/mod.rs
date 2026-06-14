//! XML extraction module for ARM Architecture Specification Language.
//!
//! This module provides functionality to:
//! - Download ARM XML specification files from ARM's website
//! - Extract ASL code from XML files
//! - Parse instruction definitions, system registers, and shared pseudocode
//!
//! This is a pure Rust implementation equivalent to the Python scripts in mra_tools.

#[cfg(feature = "xml-extract")]
pub mod download;
pub mod instrs;
pub mod regs;
pub mod shared;

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;

// Pre-compiled regexes for performance
static RE_TYPE_VAR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([^a-zA-Z0-9_])type([^a-zA-Z0-9_])").unwrap());
static RE_DEPS: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-zA-Z_]\w+(\.\w+)?\[?)").unwrap());

/// Default ARM specification version.
pub const DEFAULT_VERSION: &str = "00bet9";

/// Specification file types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecType {
    /// A64 instruction set
    A64,
    /// AArch32 instruction set
    AArch32,
    /// System registers
    SysReg,
}

impl SpecType {
    /// Get the archive filename for this spec type.
    pub fn archive_name(&self, version: &str) -> String {
        match self {
            SpecType::A64 => format!("A64_v85A_ISA_xml_{}.tar.gz", version),
            SpecType::AArch32 => format!("AArch32_v85A_ISA_xml_{}.tar.gz", version),
            SpecType::SysReg => format!("ARMv85A-SysReg-{}.tar.gz", version),
        }
    }

    /// Get the extracted directory name for this spec type.
    pub fn extracted_dir(&self, version: &str) -> String {
        match self {
            SpecType::A64 => format!("ISA_v85A_A64_xml_{}", version),
            SpecType::AArch32 => format!("ISA_v85A_AArch32_xml_{}", version),
            SpecType::SysReg => format!("SysReg_v85A_xml-{}", version),
        }
    }
}

/// Represents a chunk of ASL code with its metadata.
#[derive(Debug, Clone)]
pub struct AslChunk {
    /// Name/path of the chunk (e.g., "shared/functions/system/PSTATE")
    pub name: String,
    /// The actual ASL source code
    pub code: String,
    /// Set of names defined by this chunk
    pub defs: HashSet<String>,
    /// Set of names this chunk depends on
    pub deps: HashSet<String>,
}

impl AslChunk {
    /// Create a new ASL chunk.
    pub fn new(name: String, code: String, defs: HashSet<String>, deps: HashSet<String>) -> Self {
        Self {
            name,
            code,
            defs,
            deps,
        }
    }

    /// Apply the type variable workaround (rename 'type' to 'type1').
    pub fn patch_type_var(&mut self) {
        self.code = RE_TYPE_VAR
            .replace_all(&self.code, "${1}type1${2}")
            .to_string();
    }

    /// Patch dependencies by scanning for references in code.
    pub fn patch_dependencies(&mut self, chunks: &HashMap<String, AslChunk>) {
        for line in self.code.lines() {
            // Drop comments
            let line = line.split("//").next().unwrap_or("");
            for cap in RE_DEPS.captures_iter(line) {
                let n = &cap[1];
                if let Some(chunk) = chunks.get(n) {
                    self.deps.insert(chunk.name.clone());
                    self.deps.insert(n.to_string());
                }
            }
        }
        // Remove self-dependencies
        for def in &self.defs {
            self.deps.remove(def);
        }
    }
}

/// Represents an instruction encoding field.
#[derive(Debug, Clone)]
pub struct EncodingField {
    /// High bit position
    pub hi: u32,
    /// Low bit position
    pub lo: u32,
    /// Field name (or "_" for unnamed)
    pub name: String,
    /// Whether this field is split across multiple boxes
    pub split: bool,
    /// Constant pattern (e.g., "1x0")
    pub consts: String,
}

/// Represents an instruction encoding.
#[derive(Debug, Clone)]
pub struct InstructionEncoding {
    /// Encoding name
    pub name: String,
    /// Instruction set (A64, A32, T32, T16)
    pub instruction_set: String,
    /// Fields in this encoding
    pub fields: Vec<EncodingField>,
    /// Decode ASL code
    pub decode: AslChunk,
}

/// Represents a complete instruction with all encodings.
#[derive(Debug, Clone)]
pub struct XmlInstruction {
    /// Instruction name/path
    pub name: String,
    /// All encodings for this instruction
    pub encodings: Vec<InstructionEncoding>,
    /// Post-decode code (if any)
    pub post_decode: Option<AslChunk>,
    /// Whether this instruction is conditional
    pub conditional: bool,
    /// Execute ASL code
    pub execute: AslChunk,
}

/// Configuration for ASL extraction.
#[derive(Debug, Clone, Default)]
pub struct ExtractConfig {
    /// Whether to demangle instruction ASL
    pub demangle: bool,
    /// Whether to use alternative slice syntax
    pub alt_slice_syntax: bool,
    /// Filter to specific architectures (empty = all)
    pub architectures: Vec<String>,
    /// Regex to include instructions by name
    pub include_regex: Option<String>,
    /// Regex to exclude instructions by name
    pub exclude_regex: Option<String>,
}

/// Result of extracting ASL from XML files.
#[derive(Debug, Default)]
pub struct ExtractResult {
    /// Shared pseudocode chunks (topologically sorted)
    pub shared: Vec<AslChunk>,
    /// Instruction definitions
    pub instructions: Vec<XmlInstruction>,
    /// Notice/license text
    pub notice: String,
}

impl ExtractResult {
    /// Generate the shared definitions ASL file content.
    pub fn generate_defs_asl(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.notice);
        output.push_str("\n\n");
        for chunk in &self.shared {
            output.push_str(&chunk.code);
            output.push('\n');
        }
        output.push_str(&"/".repeat(72));
        output.push_str("\n// End\n");
        output.push_str(&"/".repeat(72));
        output.push('\n');
        output
    }

    /// Generate the instructions ASL file content.
    pub fn generate_instrs_asl(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.notice);
        output.push_str("\n\n");
        for instr in &self.instructions {
            output.push_str(&format_instruction_asl(instr));
            output.push('\n');
        }
        output.push_str(&"/".repeat(72));
        output.push_str("\n// End\n");
        output.push_str(&"/".repeat(72));
        output.push('\n');
        output
    }
}

/// Format an instruction as ASL syntax.
fn format_instruction_asl(instr: &XmlInstruction) -> String {
    let mut out = String::new();
    let name = deslash(&instr.name);
    out.push_str(&format!("__instruction {}\n", name));

    for enc in &instr.encodings {
        out.push_str(&format!("    __encoding {}\n", deslash(&enc.name)));
        out.push_str(&format!(
            "        __instruction_set {}\n",
            enc.instruction_set
        ));

        for field in &enc.fields {
            if field.name != "_" {
                let width = field.hi - field.lo + 1;
                out.push_str(&format!(
                    "        __field {} {} +: {}\n",
                    field.name, field.lo, width
                ));
            }
        }

        // Build opcode pattern
        let pattern: String = enc
            .fields
            .iter()
            .map(|f| f.consts.clone())
            .collect::<String>();
        let pattern_parts: Vec<&str> = (0..pattern.len())
            .step_by(8)
            .map(|i| &pattern[i..(i + 8).min(pattern.len())])
            .collect();
        out.push_str(&format!("        __opcode '{}'\n", pattern_parts.join(" ")));

        // Guard
        let has_cond = enc.fields.iter().any(|f| f.name == "cond");
        let guard = if enc.instruction_set == "A32" && has_cond {
            "cond != '1111'"
        } else {
            "TRUE"
        };
        out.push_str(&format!("        __guard {}\n", guard));

        out.push_str("        __decode\n");
        for line in enc.decode.code.lines() {
            out.push_str(&format!("            {}\n", line));
        }
        out.push('\n');
    }

    if let Some(ref post) = instr.post_decode {
        out.push_str("    __postdecode\n");
        for line in post.code.lines() {
            out.push_str(&format!("        {}\n", line));
        }
    }

    if instr.conditional {
        out.push_str("    __execute __conditional\n");
    } else {
        out.push_str("    __execute\n");
    }
    for line in instr.execute.code.lines() {
        out.push_str(&format!("        {}\n", line));
    }

    out
}

/// Convert a path name to an identifier (replace / and - with _).
fn deslash(name: &str) -> String {
    name.replace("/instrs", "")
        .replace('/', "_")
        .replace('-', "_")
}

/// Extract ASL from ARM XML specification directories.
///
/// # Arguments
/// * `dirs` - List of directories containing XML files (A64, A32, SysReg)
/// * `config` - Extraction configuration
///
/// # Returns
/// Extracted ASL code organized by type.
pub fn extract_from_dirs(dirs: &[&Path], config: &ExtractConfig) -> Result<ExtractResult, String> {
    let mut result = ExtractResult::default();

    // Find and read notice.xml
    for dir in dirs {
        let notice_path = dir.join("notice.xml");
        if notice_path.exists() {
            result.notice = shared::read_notice(&notice_path)?;
            break;
        }
    }

    // Read shared pseudocode from all directories
    let mut all_shared = HashMap::new();
    let mut all_names = HashSet::new();

    for dir in dirs {
        let shared_path = dir.join("shared_pseudocode.xml");
        if shared_path.exists() {
            let (shared, names) = shared::read_shared_pseudocode(&shared_path)?;
            for (k, v) in shared {
                all_shared.insert(k, v);
            }
            all_names.extend(names);
        }
    }

    // Build chunk lookup
    let mut chunks: HashMap<String, AslChunk> = HashMap::new();
    for chunk in all_shared.values() {
        for def in &chunk.defs {
            chunks.insert(def.clone(), chunk.clone());
        }
    }

    // Patch dependencies
    for chunk in all_shared.values_mut() {
        chunk.patch_dependencies(&chunks);
    }

    // Read instructions
    for dir in dirs {
        let instrs = instrs::read_instructions(dir, &chunks, config)?;
        result.instructions.extend(instrs);
    }

    // Compute reachable shared code
    let roots: HashSet<String> = compute_roots(&result.instructions, &all_shared);
    let live = compute_reachable(&all_shared, &roots);

    // Sort topologically and collect
    result.shared = live
        .into_iter()
        .filter_map(|name| all_shared.get(&name).cloned())
        .collect();

    Ok(result)
}

/// Compute root dependencies from instructions.
fn compute_roots(instrs: &[XmlInstruction], shared: &HashMap<String, AslChunk>) -> HashSet<String> {
    let mut roots = HashSet::new();

    for instr in instrs {
        for enc in &instr.encodings {
            roots.extend(enc.decode.deps.iter().cloned());
        }
        if let Some(ref post) = instr.post_decode {
            roots.extend(post.deps.iter().cloned());
        }
        roots.extend(instr.execute.deps.iter().cloned());
    }

    // Also include all shared chunks as potential roots
    roots.extend(shared.keys().cloned());

    roots
}

/// Compute reachable chunks from roots using topological sort.
fn compute_reachable(shared: &HashMap<String, AslChunk>, roots: &HashSet<String>) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut sorted = Vec::new();

    fn visit(
        name: &str,
        shared: &HashMap<String, AslChunk>,
        visited: &mut HashSet<String>,
        sorted: &mut Vec<String>,
        path: &mut Vec<String>,
    ) {
        if path.contains(&name.to_string()) || visited.contains(name) {
            return;
        }

        visited.insert(name.to_string());

        if let Some(chunk) = shared.get(name) {
            path.push(name.to_string());
            for dep in &chunk.deps {
                visit(dep, shared, visited, sorted, path);
            }
            path.pop();
            sorted.push(name.to_string());
        }
    }

    for root in roots {
        let mut path = Vec::new();
        visit(root, shared, &mut visited, &mut sorted, &mut path);
    }

    sorted
}
