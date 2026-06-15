//! Structured test output generation.
//!
//! Generates tests in a hierarchical directory structure:
//!
//! ```text
//! tests/arm/
//! ├── mod.rs
//! ├── a64/
//! │   ├── mod.rs
//! │   ├── integer/
//! │   │   ├── mod.rs
//! │   │   ├── arithmetic.rs
//! │   │   ├── logical.rs
//! │   │   └── ...
//! │   ├── memory/
//! │   ├── branch/
//! │   ├── float/
//! │   ├── vector/
//! │   └── system/
//! ├── a32/
//! │   ├── mod.rs
//! │   ├── data_processing/
//! │   ├── memory/
//! │   └── ...
//! └── ...
//! ```

use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;

use crate::syntax::InstructionSet;
use crate::testgen::types::*;

/// Category for organizing tests
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TestCategory {
    /// Instruction set (a64, a32, t32, t16)
    pub iset: String,
    /// Main category (integer, memory, branch, float, vector, system)
    pub category: String,
    /// Subcategory (arithmetic, logical, single, pair, etc.)
    pub subcategory: String,
}

impl TestCategory {
    /// Get the file path for this category
    pub fn file_path(&self) -> PathBuf {
        PathBuf::from(&self.iset)
            .join(&self.category)
            .join(format!("{}.rs", self.subcategory))
    }

    /// Get the module path (for mod.rs imports)
    pub fn module_path(&self) -> String {
        format!("{}::{}::{}", self.iset, self.category, self.subcategory)
    }
}

/// Categorize an instruction based on its name
pub fn categorize_instruction(name: &str, iset: InstructionSet) -> TestCategory {
    let iset_str = match iset {
        InstructionSet::A64 => "a64",
        InstructionSet::A32 => "a32",
        InstructionSet::T32 => "t32",
        InstructionSet::T16 => "t16",
    };

    let (category, subcategory) = match iset {
        InstructionSet::A64 => categorize_a64(name),
        InstructionSet::A32 | InstructionSet::T32 | InstructionSet::T16 => categorize_a32(name),
    };

    TestCategory {
        iset: iset_str.to_string(),
        category,
        subcategory,
    }
}

fn categorize_a64(name: &str) -> (String, String) {
    // Check for SVE instructions first - they have distinctive naming patterns
    // Examples: ADD_Z.P.ZZ__, LDNT1D_Z.P.BR_Contiguous, CMPEQ_P.P.ZZ__, FMLA_Z.ZZZi_S
    // Also scalar SVE ops: ADDPL_R.RI__, CNTB_R.S__, PRFB_I.P.AI_S, RDVL_R.I__
    let is_sve = name.contains("_Z.")
        || name.contains("_P.")
        || name.contains(".Z__")
        || name.contains(".P__")
        || name.contains("_R.")
        || name.contains("_I.P.")
        || name.contains("_F__")
        || name.starts_with("PRF")
        || name.starts_with("CNT")
        || name.starts_with("RDVL")
        || name.starts_with("ADDVL")
        || name.starts_with("ADDPL")
        || name.starts_with("SQINC")
        || name.starts_with("SQDEC")
        || name.starts_with("UQINC")
        || name.starts_with("UQDEC")
        || name.starts_with("INC")
        || name.starts_with("DEC")
        || name.starts_with("CTERM")
        || name.starts_with("SETFFR")
        || name.starts_with("WRFFR")
        || name.starts_with("PFALSE");

    if is_sve {
        // SVE instruction - categorize by operation type
        let subcategory = if name.starts_with("LD")
            || name.starts_with("LDNT")
            || name.starts_with("LDFF")
            || name.starts_with("LD1")
            || name.starts_with("LD2")
            || name.starts_with("LD3")
            || name.starts_with("LD4")
        {
            "load"
        } else if name.starts_with("ST") || name.starts_with("STNT") {
            "store"
        } else if name.starts_with("PRF") {
            "prefetch"
        } else if name.starts_with("ADD")
            || name.starts_with("SUB")
            || name.starts_with("MUL")
            || name.starts_with("MLA")
            || name.starts_with("MLS")
        {
            "arithmetic"
        } else if name.starts_with("AND")
            || name.starts_with("ORR")
            || name.starts_with("EOR")
            || name.starts_with("BIC")
            || name.starts_with("NOT")
        {
            "logical"
        } else if name.starts_with("CMP") || name.starts_with("FCM") || name.starts_with("CTERM") {
            "compare"
        } else if name.starts_with("F") {
            "float"
        } else if name.contains("CVT") || name.contains("XTN") || name.contains("XTL") {
            "convert"
        } else if name.contains("_P.P.")
            || name.starts_with("BRK")
            || name.starts_with("PTEST")
            || name.starts_with("PFALSE")
            || name.starts_with("PTRUE")
            || name.starts_with("SETFFR")
            || name.starts_with("WRFFR")
            || name.starts_with("RDFFR")
        {
            "predicate"
        } else if name.starts_with("CNT")
            || name.starts_with("RDVL")
            || name.starts_with("INC")
            || name.starts_with("DEC")
            || name.starts_with("SQINC")
            || name.starts_with("SQDEC")
            || name.starts_with("UQINC")
            || name.starts_with("UQDEC")
        {
            "scalar"
        } else {
            "other"
        };
        return ("sve".to_string(), subcategory.to_string());
    }

    // Parse the hierarchical name: aarch64_category_subcategory_...
    let parts: Vec<&str> = name.split('_').collect();

    if parts.len() < 3 {
        return ("misc".to_string(), "other".to_string());
    }

    // Skip "aarch64" prefix
    let main_cat = parts.get(1).unwrap_or(&"misc");
    let sub_cat = parts.get(2).unwrap_or(&"other");

    let (category, subcategory) = match *main_cat {
        "integer" => {
            let subcat = match *sub_cat {
                "arithmetic" => {
                    // Further classify arithmetic
                    if name.contains("add_sub") {
                        "add_sub"
                    } else if name.contains("mul") || name.contains("div") {
                        "mul_div"
                    } else if name.contains("address") {
                        "address"
                    } else {
                        "other"
                    }
                }
                "logical" => "logical",
                "shift" => "shift",
                "conditional" => "conditional",
                "bitfield" | "ins" | "extract" => "bitfield",
                "tags" => "tags",
                "pac" => "pac",
                "flags" => "flags",
                _ => "other",
            };
            ("integer".to_string(), subcat.to_string())
        }
        "memory" => {
            let subcat = match *sub_cat {
                "single" => "single",
                "pair" => "pair",
                "atomicops" => "atomic",
                "exclusive" => "exclusive",
                "ordered" => "ordered",
                "vector" => "vector",
                "literal" => "literal",
                _ => "other",
            };
            ("memory".to_string(), subcat.to_string())
        }
        "branch" => {
            let subcat = match *sub_cat {
                "unconditional" => "unconditional",
                "conditional" => "conditional",
                _ => "other",
            };
            ("branch".to_string(), subcat.to_string())
        }
        "float" => {
            let subcat = match *sub_cat {
                "arithmetic" => "arithmetic",
                "convert" => "convert",
                "compare" => "compare",
                "move" => "move_",
                _ => "other",
            };
            ("float".to_string(), subcat.to_string())
        }
        "vector" => {
            let subcat = match *sub_cat {
                "arithmetic" => {
                    // Further classify vector arithmetic
                    if name.contains("add") || name.contains("sub") {
                        "add_sub"
                    } else if name.contains("mul") {
                        "mul"
                    } else if name.contains("cmp") {
                        "compare"
                    } else {
                        "arithmetic"
                    }
                }
                "shift" => "shift",
                "transfer" => "transfer",
                "reduce" => "reduce",
                "crypto" => "crypto",
                _ => "other",
            };
            ("vector".to_string(), subcat.to_string())
        }
        "system" => {
            let subcat = match *sub_cat {
                "exceptions" => "exceptions",
                "register" => "register",
                "hints" => "hints",
                _ => "other",
            };
            ("system".to_string(), subcat.to_string())
        }
        _ => ("misc".to_string(), "other".to_string()),
    };

    (category, subcategory)
}

fn categorize_a32(name: &str) -> (String, String) {
    // A32 names are like: aarch32_ADD_i_A, aarch32_LDR_r_A, aarch32_VMOV_A
    let parts: Vec<&str> = name.split('_').collect();

    if parts.len() < 2 {
        return ("misc".to_string(), "other".to_string());
    }

    let mnemonic = parts.get(1).unwrap_or(&"unknown").to_uppercase();

    // Categorize by mnemonic
    let (category, subcategory) = match mnemonic.as_str() {
        // Data processing - arithmetic
        "ADD" | "ADC" | "SUB" | "SBC" | "RSB" | "RSC" => {
            ("data_processing".to_string(), "add_sub".to_string())
        }

        // Data processing - logical
        "AND" | "ORR" | "EOR" | "BIC" | "ORN" => {
            ("data_processing".to_string(), "logical".to_string())
        }

        // Data processing - compare
        "CMP" | "CMN" | "TST" | "TEQ" => ("data_processing".to_string(), "compare".to_string()),

        // Data processing - move
        "MOV" | "MVN" | "MOVT" | "MOVW" => ("data_processing".to_string(), "move_".to_string()),

        // Multiply
        "MUL" | "MLA" | "MLS" | "SMULL" | "UMULL" | "SMLAL" | "UMLAL" | "SMULBB" | "SMULBT"
        | "SMULTB" | "SMULTT" | "SMULWB" | "SMULWT" | "SMLABB" | "SMLABT" | "SMLATB" | "SMLATT"
        | "SMLAWB" | "SMLAWT" | "SMLAD" | "SMLALD" | "SMLSD" | "SMLSLD" | "SMMUL" | "SMMLA"
        | "SMMLS" | "SMUAD" | "SMUSD" | "SDIV" | "UDIV" => {
            ("multiply".to_string(), "mul".to_string())
        }

        // Load
        m if m.starts_with("LDR") || m.starts_with("LDM") || m.starts_with("POP") => {
            ("memory".to_string(), "load".to_string())
        }
        "LDA" | "LDAB" | "LDAH" | "LDAEX" | "LDAEXB" | "LDAEXD" | "LDAEXH" => {
            ("memory".to_string(), "load".to_string())
        }

        // Store
        m if m.starts_with("STR") || m.starts_with("STM") || m.starts_with("PUSH") => {
            ("memory".to_string(), "store".to_string())
        }
        "STL" | "STLB" | "STLH" | "STLEX" | "STLEXB" | "STLEXD" | "STLEXH" => {
            ("memory".to_string(), "store".to_string())
        }

        // Branch
        "B" | "BL" | "BX" | "BLX" | "BXJ" => ("branch".to_string(), "branch".to_string()),
        "CBZ" | "CBNZ" | "TBB" | "TBH" => ("branch".to_string(), "compare_branch".to_string()),

        // SIMD/VFP - too many to list, use prefix matching
        m if m.starts_with('V') => {
            let subcat = if m.starts_with("VADD") || m.starts_with("VSUB") {
                "add_sub"
            } else if m.starts_with("VMUL") || m.starts_with("VMLA") || m.starts_with("VMLS") {
                "mul"
            } else if m.starts_with("VMOV") {
                "move_"
            } else if m.starts_with("VLD") {
                "load"
            } else if m.starts_with("VST") {
                "store"
            } else if m.starts_with("VCVT") {
                "convert"
            } else if m.starts_with("VCMP") {
                "compare"
            } else {
                "other"
            };
            ("simd".to_string(), subcat.to_string())
        }

        // System
        "MRS" | "MSR" | "CPS" | "SETEND" | "SVC" | "HVC" | "SMC" | "BKPT" | "HLT" | "DMB"
        | "DSB" | "ISB" | "PLD" | "PLDW" | "PLI" | "CLREX" | "WFE" | "WFI" | "SEV" | "SEVL"
        | "YIELD" | "NOP" | "DBG" => ("system".to_string(), "system".to_string()),

        // Bit manipulation
        "CLZ" | "RBIT" | "REV" | "REV16" | "REVSH" | "BFC" | "BFI" | "SBFX" | "UBFX" | "SSAT"
        | "SSAT16" | "USAT" | "USAT16" | "PKHBT" | "PKHTB" => (
            "data_processing".to_string(),
            "bit_manipulation".to_string(),
        ),

        // Saturating arithmetic
        "QADD" | "QSUB" | "QDADD" | "QDSUB" | "QADD8" | "QADD16" | "QSUB8" | "QSUB16" | "QASX"
        | "QSAX" | "UQADD8" | "UQADD16" | "UQSUB8" | "UQSUB16" | "UQASX" | "UQSAX" => {
            ("data_processing".to_string(), "saturating".to_string())
        }

        // Parallel add/sub
        "SADD8" | "SADD16" | "SSUB8" | "SSUB16" | "SASX" | "SSAX" | "UADD8" | "UADD16"
        | "USUB8" | "USUB16" | "UASX" | "USAX" | "SHADD8" | "SHADD16" | "SHSUB8" | "SHSUB16"
        | "SHASX" | "SHSAX" | "UHADD8" | "UHADD16" | "UHSUB8" | "UHSUB16" | "UHASX" | "UHSAX" => {
            ("data_processing".to_string(), "parallel".to_string())
        }

        // Extend
        "SXTB" | "SXTB16" | "SXTH" | "SXTAB" | "SXTAB16" | "SXTAH" | "UXTB" | "UXTB16" | "UXTH"
        | "UXTAB" | "UXTAB16" | "UXTAH" => ("data_processing".to_string(), "extend".to_string()),

        // Shift/Rotate
        "LSL" | "LSR" | "ASR" | "ROR" | "RRX" => {
            ("data_processing".to_string(), "shift".to_string())
        }

        // Coprocessor
        "CDP" | "CDP2" | "MCR" | "MCR2" | "MCRR" | "MCRR2" | "MRC" | "MRC2" | "MRRC" | "MRRC2"
        | "LDC" | "LDC2" | "STC" | "STC2" => ("coprocessor".to_string(), "coprocessor".to_string()),

        _ => ("misc".to_string(), "other".to_string()),
    };

    (category, subcategory)
}

/// Output file with its content
#[derive(Debug)]
pub struct OutputFile {
    /// File path relative to output directory
    pub path: PathBuf,
    /// File content
    pub content: String,
}

/// Maximum file size in bytes (5MB)
const MAX_FILE_SIZE: usize = 5 * 1024 * 1024;

/// Generate structured test files, splitting large files to stay under 5MB
pub fn generate_structured_output(suites: &[InstructionTestSuite]) -> Vec<OutputFile> {
    // Group suites by category
    let mut by_category: HashMap<TestCategory, Vec<&InstructionTestSuite>> = HashMap::new();

    for suite in suites {
        let iset = suite
            .analysis
            .encodings
            .first()
            .map(|e| e.iset)
            .unwrap_or(InstructionSet::A64);
        let category = categorize_instruction(&suite.instruction_name, iset);
        by_category.entry(category).or_default().push(suite);
    }

    let mut files = Vec::new();

    // Track what modules we need at each level (including split parts)
    let mut iset_categories: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    // Generate test files for each category, splitting if needed
    for (category, suites) in &by_category {
        let split_files = generate_category_files_split(category, suites);

        for (idx, content) in split_files.into_iter().enumerate() {
            let (path, module_name) = if idx == 0 && content.len() < MAX_FILE_SIZE {
                // Single file, use original name
                (category.file_path(), category.subcategory.clone())
            } else {
                // Split file, use numbered suffix
                let part_name = format!("{}_part{}", category.subcategory, idx + 1);
                let path = PathBuf::from(&category.iset)
                    .join(&category.category)
                    .join(format!("{}.rs", part_name));
                (path, part_name)
            };

            files.push(OutputFile { path, content });

            // Track for mod.rs generation
            iset_categories
                .entry(category.iset.clone())
                .or_default()
                .entry(category.category.clone())
                .or_default()
                .push(module_name);
        }
    }

    // Deduplicate module names in case we added both original and parts
    for (_, categories) in iset_categories.iter_mut() {
        for (_, subcats) in categories.iter_mut() {
            subcats.sort();
            subcats.dedup();
        }
    }

    // Generate mod.rs files
    // Root mod.rs
    let mut root_mod = String::new();
    writeln!(root_mod, "//! Auto-generated ARM instruction tests.").unwrap();
    writeln!(root_mod, "//!").unwrap();
    writeln!(
        root_mod,
        "//! Generated by asl-parser from ARM ASL specifications."
    )
    .unwrap();
    writeln!(root_mod, "//! DO NOT EDIT MANUALLY.").unwrap();
    writeln!(root_mod).unwrap();

    // Include test helper modules based on which ISETs are present
    let mut isets: Vec<_> = iset_categories.keys().collect();
    isets.sort();

    // Add A64 helpers if we have A64 tests
    if isets.iter().any(|s| *s == "a64") {
        writeln!(root_mod, "pub mod test_helpers;").unwrap();
    }

    // Add A32/T32/T16 helpers if we have any 32-bit tests
    if isets
        .iter()
        .any(|s| *s == "a32" || *s == "t32" || *s == "t16")
    {
        writeln!(root_mod, "pub mod test_helpers_32;").unwrap();
    }

    writeln!(root_mod).unwrap();

    for iset in &isets {
        writeln!(root_mod, "pub mod {};", iset).unwrap();
    }
    files.push(OutputFile {
        path: PathBuf::from("mod.rs"),
        content: root_mod,
    });

    // Iset-level mod.rs
    for (iset, categories) in &iset_categories {
        let mut iset_mod = String::new();
        writeln!(iset_mod, "//! {} instruction tests.", iset.to_uppercase()).unwrap();
        writeln!(iset_mod).unwrap();
        let mut cats: Vec<_> = categories.keys().collect();
        cats.sort();
        for cat in cats {
            writeln!(iset_mod, "pub mod {};", cat).unwrap();
        }
        files.push(OutputFile {
            path: PathBuf::from(iset).join("mod.rs"),
            content: iset_mod,
        });

        // Category-level mod.rs
        for (cat, subcats) in categories {
            let mut cat_mod = String::new();
            writeln!(cat_mod, "//! {} {} tests.", iset.to_uppercase(), cat).unwrap();
            writeln!(cat_mod).unwrap();
            let mut sorted_subcats = subcats.clone();
            sorted_subcats.sort();
            for subcat in &sorted_subcats {
                writeln!(cat_mod, "pub mod {};", subcat).unwrap();
            }
            files.push(OutputFile {
                path: PathBuf::from(iset).join(cat).join("mod.rs"),
                content: cat_mod,
            });
        }
    }

    files
}

/// Generate category file(s), splitting into multiple files if content exceeds MAX_FILE_SIZE
fn generate_category_files_split(
    category: &TestCategory,
    suites: &[&InstructionTestSuite],
) -> Vec<String> {
    use std::collections::HashSet;

    // First, try generating as a single file
    let single_content = generate_category_file(category, suites);

    if single_content.len() <= MAX_FILE_SIZE {
        return vec![single_content];
    }

    // Need to split - generate each suite's tests separately and batch them
    // Use a shared set for deduplication across all parts
    let mut seen_fn_names: HashSet<String> = HashSet::new();
    let mut result = Vec::new();
    let mut current_content = generate_category_header(category);
    let mut current_size = current_content.len();

    for suite in suites {
        let mut suite_content = String::new();
        super::rust::write_instruction_tests_with_dedup(
            &mut suite_content,
            suite,
            &mut seen_fn_names,
        );

        // Check if adding this suite would exceed the limit
        if current_size + suite_content.len() > MAX_FILE_SIZE && current_size > 1000 {
            // Save current file and start a new one
            result.push(current_content);
            current_content = generate_category_header(category);
            current_size = current_content.len();
        }

        current_content.push_str(&suite_content);
        current_size += suite_content.len();
    }

    // Don't forget the last file
    if current_size > 500 {
        result.push(current_content);
    }

    result
}

/// Generate the header for a category file
fn generate_category_header(category: &TestCategory) -> String {
    let mut output = String::new();

    // File header
    writeln!(
        output,
        "//! {} {} {} tests.",
        category.iset.to_uppercase(),
        category.category,
        category.subcategory
    )
    .unwrap();
    writeln!(output, "//!").unwrap();
    writeln!(output, "//! Auto-generated from ARM ASL specifications.").unwrap();
    writeln!(output, "//! DO NOT EDIT MANUALLY.").unwrap();
    writeln!(output).unwrap();

    // Imports
    writeln!(output, "#![allow(unused_imports)]").unwrap();
    writeln!(output, "#![allow(dead_code)]").unwrap();
    writeln!(output).unwrap();

    // Use the common test infrastructure
    // Path is relative to tests/arm.rs crate root -> generated module -> helpers
    if category.iset == "a64" {
        writeln!(output, "use crate::generated::test_helpers::*;").unwrap();
    } else {
        writeln!(output, "use crate::generated::test_helpers_32::*;").unwrap();
    }
    writeln!(output).unwrap();

    output
}

/// Generate a single category file
fn generate_category_file(category: &TestCategory, suites: &[&InstructionTestSuite]) -> String {
    use std::collections::HashSet;

    let mut output = generate_category_header(category);
    let mut seen_fn_names: HashSet<String> = HashSet::new();

    // Generate tests for each suite with deduplication across all suites
    for suite in suites {
        super::rust::write_instruction_tests_with_dedup(&mut output, suite, &mut seen_fn_names);
    }

    output
}

/// Write test files to disk
pub fn write_structured_output(
    base_path: &std::path::Path,
    files: &[OutputFile],
) -> std::io::Result<()> {
    use std::fs;

    for file in files {
        let full_path = base_path.join(&file.path);

        // Create parent directories
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&full_path, &file.content)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_a64() {
        assert_eq!(
            categorize_a64("aarch64_integer_arithmetic_add_sub_immediate"),
            ("integer".to_string(), "add_sub".to_string())
        );
        assert_eq!(
            categorize_a64("aarch64_memory_single_general_immediate"),
            ("memory".to_string(), "single".to_string())
        );
        assert_eq!(
            categorize_a64("aarch64_vector_crypto_sha256"),
            ("vector".to_string(), "crypto".to_string())
        );
    }

    #[test]
    fn test_categorize_a32() {
        assert_eq!(
            categorize_a32("aarch32_ADD_i_A"),
            ("data_processing".to_string(), "add_sub".to_string())
        );
        assert_eq!(
            categorize_a32("aarch32_LDR_r_A"),
            ("memory".to_string(), "load".to_string())
        );
        assert_eq!(
            categorize_a32("aarch32_VMOV_A"),
            ("simd".to_string(), "move_".to_string())
        );
    }
}
