//! ASL Parser CLI
//!
//! A command-line tool for parsing ARM Architecture Specification Language files.

use asl_parser::testgen::{OutputFormat as TestOutputFormat, TestGenConfig, TestGenerator};
use asl_parser::{parse_definitions, parse_instructions, parse_registers};
use clap::{Parser, Subcommand, ValueEnum};
use miette::{IntoDiagnostic, Result, WrapErr};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "asl-parser",
    about = "Parser for ARM Architecture Specification Language (ASL)",
    version,
    author
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse definition files (types, functions, constants, etc.)
    #[command(alias = "defs")]
    Definitions {
        /// Input ASL file(s)
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output format
        #[arg(short, long, default_value = "summary")]
        format: OutputFormat,
    },

    /// Parse instruction definitions
    #[command(alias = "inst")]
    Instructions {
        /// Input ASL file(s)
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output format
        #[arg(short, long, default_value = "summary")]
        format: OutputFormat,
    },

    /// Parse register definitions
    #[command(alias = "regs")]
    Registers {
        /// Input ASL file(s)
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output format
        #[arg(short, long, default_value = "summary")]
        format: OutputFormat,
    },

    /// Lex a file and print tokens (for debugging)
    Lex {
        /// Input ASL file
        file: PathBuf,
    },

    /// Download ARM XML specification files
    #[cfg(feature = "xml-extract")]
    Download {
        /// Output directory for downloaded files
        #[arg(short, long, default_value = "xml")]
        output: PathBuf,

        /// ARM specification version
        #[arg(short, long, default_value = asl_parser::xml::DEFAULT_VERSION)]
        version: String,

        /// Specification type(s) to download
        #[arg(short, long, value_enum)]
        spec: Option<Vec<SpecTypeArg>>,
    },

    /// Extract ASL from downloaded XML files
    #[cfg(feature = "xml-extract")]
    Extract {
        /// Input XML directory (containing downloaded specs)
        #[arg(short, long, default_value = "xml")]
        input: PathBuf,

        /// Output directory for ASL files
        #[arg(short, long, default_value = "asl")]
        output: PathBuf,

        /// ARM specification version
        #[arg(long, default_value = asl_parser::xml::DEFAULT_VERSION)]
        version: String,

        /// Demangle instruction ASL code
        #[arg(long)]
        demangle: bool,

        /// Filter to specific architecture(s)
        #[arg(long)]
        arch: Option<Vec<String>>,
    },

    /// Full pipeline: download, extract, and parse
    #[cfg(feature = "xml-extract")]
    Pipeline {
        /// Working directory for all files
        #[arg(short, long, default_value = ".")]
        workdir: PathBuf,

        /// ARM specification version
        #[arg(long, default_value = asl_parser::xml::DEFAULT_VERSION)]
        version: String,

        /// Demangle instruction ASL code
        #[arg(long)]
        demangle: bool,

        /// Output format for parsed results
        #[arg(short, long, default_value = "summary")]
        format: OutputFormat,
    },

    /// Generate test cases from parsed instruction definitions
    #[command(alias = "tests")]
    TestGen {
        /// Input ASL instruction file
        #[arg(required = true)]
        input: PathBuf,

        /// Output directory for generated tests
        #[arg(short, long, default_value = "generated_tests")]
        output: PathBuf,

        /// Maximum number of tests per instruction
        #[arg(long, default_value = "200")]
        max_tests: usize,

        /// Include negative tests (UNDEFINED encodings)
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        include_negative: bool,

        /// Include execution tests (register/flag effects)
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        include_execution: bool,

        /// Filter to specific instruction name (regex pattern)
        #[arg(long)]
        filter: Option<String>,

        /// Output format (rust, json, text)
        #[arg(short, long, default_value = "rust")]
        format: TestGenOutputFormat,

        /// Target instruction set (a64, a32, t32, t16, all)
        #[arg(long, default_value = "a64")]
        iset: InstructionSetArg,

        /// Generate structured hierarchical output (tests/arm/a64/integer/...)
        #[arg(long, default_value = "false")]
        structured: bool,
    },

    /// Generate system register tests from arm_regs.asl
    #[command(alias = "sysreg-tests")]
    SysRegGen {
        /// Input ASL register file (arm_regs.asl)
        #[arg(required = true)]
        input: PathBuf,

        /// Output directory for generated tests
        #[arg(short, long, default_value = "generated_tests")]
        output: PathBuf,
    },
}

#[derive(Clone, Copy, ValueEnum, Default)]
enum TestGenOutputFormat {
    /// Generate Rust test code
    #[default]
    Rust,
    /// Generate JSON output
    Json,
    /// Generate human-readable text
    Text,
}

#[derive(Clone, Copy, ValueEnum, Default)]
enum InstructionSetArg {
    /// A64 instruction set only
    #[default]
    A64,
    /// A32 instruction set only
    A32,
    /// Thumb-32 instruction set only
    T32,
    /// Thumb-16 instruction set only
    T16,
    /// All instruction sets
    All,
}

#[derive(Clone, Copy, ValueEnum, Default)]
enum OutputFormat {
    /// Print a summary of parsed items
    #[default]
    Summary,
    /// Print detailed debug output
    Debug,
    /// Print as JSON (requires 'serde' feature)
    #[cfg(feature = "serde")]
    Json,
}

#[cfg(feature = "xml-extract")]
#[derive(Clone, Copy, ValueEnum)]
enum SpecTypeArg {
    /// A64 instruction set
    A64,
    /// AArch32 instruction set
    AArch32,
    /// System registers
    SysReg,
    /// All specifications
    All,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Definitions { files, format } => {
            for file in files {
                parse_defs_file(&file, format)?;
            }
        }
        Command::Instructions { files, format } => {
            for file in files {
                parse_instrs_file(&file, format)?;
            }
        }
        Command::Registers { files, format } => {
            for file in files {
                parse_regs_file(&file, format)?;
            }
        }
        Command::Lex { file } => {
            lex_file(&file)?;
        }
        #[cfg(feature = "xml-extract")]
        Command::Download {
            output,
            version,
            spec,
        } => {
            download_specs(&output, &version, spec)?;
        }
        #[cfg(feature = "xml-extract")]
        Command::Extract {
            input,
            output,
            version,
            demangle,
            arch,
        } => {
            extract_asl(&input, &output, &version, demangle, arch)?;
        }
        #[cfg(feature = "xml-extract")]
        Command::Pipeline {
            workdir,
            version,
            demangle,
            format,
        } => {
            run_pipeline(&workdir, &version, demangle, format)?;
        }
        Command::TestGen {
            input,
            output,
            max_tests,
            include_negative,
            include_execution,
            filter,
            format,
            iset,
            structured,
        } => {
            generate_tests(
                &input,
                &output,
                max_tests,
                include_negative,
                include_execution,
                filter,
                format,
                iset,
                structured,
            )?;
        }
        Command::SysRegGen { input, output } => {
            generate_sysreg_tests(&input, &output)?;
        }
    }

    Ok(())
}

fn parse_defs_file(path: &PathBuf, format: OutputFormat) -> Result<()> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read file: {}", path.display()))?;

    let defs = parse_definitions(&source).map_err(|e| {
        miette::miette!(
            labels = vec![miette::LabeledSpan::at(
                e.span.start..e.span.end,
                e.kind.to_string()
            )],
            "parse error in {}",
            path.display()
        )
        .with_source_code(source.clone())
    })?;

    match format {
        OutputFormat::Summary => {
            println!("Parsed {} definitions from {}", defs.len(), path.display());
            for def in &defs {
                print_def_summary(def);
            }
        }
        OutputFormat::Debug => {
            println!("=== {} ===", path.display());
            for def in &defs {
                println!("{:#?}", def);
            }
        }
        #[cfg(feature = "serde")]
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&defs).into_diagnostic()?;
            println!("{}", json);
        }
    }

    Ok(())
}

fn parse_instrs_file(path: &PathBuf, format: OutputFormat) -> Result<()> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read file: {}", path.display()))?;

    let instrs = parse_instructions(&source).map_err(|e| {
        miette::miette!(
            labels = vec![miette::LabeledSpan::at(
                e.span.start..e.span.end,
                e.kind.to_string()
            )],
            "parse error in {}",
            path.display()
        )
        .with_source_code(source.clone())
    })?;

    match format {
        OutputFormat::Summary => {
            println!(
                "Parsed {} instructions from {}",
                instrs.len(),
                path.display()
            );
            for instr in &instrs {
                println!(
                    "  {} ({} encoding{})",
                    instr.name,
                    instr.encodings.len(),
                    if instr.encodings.len() == 1 { "" } else { "s" }
                );
            }
        }
        OutputFormat::Debug => {
            println!("=== {} ===", path.display());
            for instr in &instrs {
                println!("{:#?}", instr);
            }
        }
        #[cfg(feature = "serde")]
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&instrs).into_diagnostic()?;
            println!("{}", json);
        }
    }

    Ok(())
}

fn parse_regs_file(path: &PathBuf, format: OutputFormat) -> Result<()> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read file: {}", path.display()))?;

    let regs = parse_registers(&source).map_err(|e| {
        miette::miette!(
            labels = vec![miette::LabeledSpan::at(
                e.span.start..e.span.end,
                e.kind.to_string()
            )],
            "parse error in {}",
            path.display()
        )
        .with_source_code(source.clone())
    })?;

    match format {
        OutputFormat::Summary => {
            println!(
                "Parsed {} register definitions from {}",
                regs.len(),
                path.display()
            );
            for reg in &regs {
                match reg {
                    asl_parser::RegisterDefinition::Single(r) => {
                        println!("  {} ({}b, {} fields)", r.name, r.width, r.fields.len());
                    }
                    asl_parser::RegisterDefinition::Array(arr) => {
                        println!(
                            "  {}[{}..{}] ({}b, {} fields)",
                            arr.register.name,
                            arr.index_min,
                            arr.index_max,
                            arr.register.width,
                            arr.register.fields.len()
                        );
                    }
                }
            }
        }
        OutputFormat::Debug => {
            println!("=== {} ===", path.display());
            for reg in &regs {
                println!("{:#?}", reg);
            }
        }
        #[cfg(feature = "serde")]
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&regs).into_diagnostic()?;
            println!("{}", json);
        }
    }

    Ok(())
}

fn lex_file(path: &PathBuf) -> Result<()> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read file: {}", path.display()))?;

    let mut lexer = asl_parser::lexer::Lexer::new(&source);
    let tokens = lexer.collect_tokens();

    println!("=== Tokens from {} ===", path.display());
    for tok in tokens {
        let text = &source[tok.span.start..tok.span.end.min(source.len())];
        println!(
            "{:4}..{:<4} {:20} {:?}",
            tok.span.start,
            tok.span.end,
            format!("{:?}", tok.kind),
            if text.len() > 30 {
                format!("{}...", &text[..30])
            } else {
                text.to_string()
            }
        );
    }

    Ok(())
}

#[cfg(feature = "xml-extract")]
fn download_specs(output: &PathBuf, version: &str, spec: Option<Vec<SpecTypeArg>>) -> Result<()> {
    use asl_parser::xml::download::{download_spec, extract_archive};
    use asl_parser::xml::SpecType;

    let specs = spec.unwrap_or_else(|| vec![SpecTypeArg::All]);

    let download_a64 = specs
        .iter()
        .any(|s| matches!(s, SpecTypeArg::A64 | SpecTypeArg::All));
    let download_a32 = specs
        .iter()
        .any(|s| matches!(s, SpecTypeArg::AArch32 | SpecTypeArg::All));
    let download_sysreg = specs
        .iter()
        .any(|s| matches!(s, SpecTypeArg::SysReg | SpecTypeArg::All));

    if download_a64 || download_a32 || download_sysreg {
        fs::create_dir_all(output)
            .into_diagnostic()
            .wrap_err("Failed to create output directory")?;
    }

    if download_a64 {
        let archive =
            download_spec(SpecType::A64, version, output).map_err(|e| miette::miette!("{}", e))?;
        extract_archive(&archive, output).map_err(|e| miette::miette!("{}", e))?;
    }

    if download_a32 {
        let archive = download_spec(SpecType::AArch32, version, output)
            .map_err(|e| miette::miette!("{}", e))?;
        extract_archive(&archive, output).map_err(|e| miette::miette!("{}", e))?;
    }

    if download_sysreg {
        let archive = download_spec(SpecType::SysReg, version, output)
            .map_err(|e| miette::miette!("{}", e))?;
        extract_archive(&archive, output).map_err(|e| miette::miette!("{}", e))?;
    }

    println!("Download complete!");
    Ok(())
}

#[cfg(feature = "xml-extract")]
fn extract_asl(
    input: &PathBuf,
    output: &PathBuf,
    version: &str,
    demangle: bool,
    arch: Option<Vec<String>>,
) -> Result<()> {
    use asl_parser::xml::{extract_from_dirs, regs, shared, ExtractConfig, SpecType};

    // Find XML directories
    let a64_dir = input.join(SpecType::A64.extracted_dir(version));
    let a32_dir = input.join(SpecType::AArch32.extracted_dir(version));
    let sysreg_dir = input.join(SpecType::SysReg.extracted_dir(version));

    let mut dirs: Vec<&std::path::Path> = Vec::new();
    if a64_dir.exists() {
        dirs.push(a64_dir.as_path());
    }
    if a32_dir.exists() {
        dirs.push(a32_dir.as_path());
    }

    if dirs.is_empty() {
        return Err(miette::miette!(
            "No XML directories found in {}. Run 'download' first.",
            input.display()
        ));
    }

    let config = ExtractConfig {
        demangle,
        architectures: arch.unwrap_or_default(),
        ..Default::default()
    };

    println!("Extracting ASL from XML files...");
    let result = extract_from_dirs(&dirs, &config).map_err(|e| miette::miette!("{}", e))?;

    // Create output directory
    fs::create_dir_all(output)
        .into_diagnostic()
        .wrap_err("Failed to create output directory")?;

    // Write definitions
    let defs_path = output.join("arm_defs.asl");
    let defs_content = result.generate_defs_asl();
    fs::write(&defs_path, &defs_content)
        .into_diagnostic()
        .wrap_err("Failed to write arm_defs.asl")?;
    println!(
        "Wrote {} ({} bytes)",
        defs_path.display(),
        defs_content.len()
    );

    // Write instructions
    let instrs_path = output.join("arm_instrs.asl");
    let instrs_content = result.generate_instrs_asl();
    fs::write(&instrs_path, &instrs_content)
        .into_diagnostic()
        .wrap_err("Failed to write arm_instrs.asl")?;
    println!(
        "Wrote {} ({} bytes)",
        instrs_path.display(),
        instrs_content.len()
    );

    // Extract and write registers if sysreg directory exists
    if sysreg_dir.exists() {
        println!("Extracting register definitions...");
        let registers = regs::read_registers(&sysreg_dir).map_err(|e| miette::miette!("{}", e))?;

        // Read notice for registers
        let notice = if let Ok(n) = shared::read_notice(&a64_dir.join("notice.xml")) {
            n
        } else {
            result.notice.clone()
        };

        let regs_content = regs::generate_registers_asl(&registers, &notice);
        let regs_path = output.join("arm_regs.asl");
        fs::write(&regs_path, &regs_content)
            .into_diagnostic()
            .wrap_err("Failed to write arm_regs.asl")?;
        println!(
            "Wrote {} ({} bytes)",
            regs_path.display(),
            regs_content.len()
        );
    }

    println!("Extraction complete!");
    Ok(())
}

#[cfg(feature = "xml-extract")]
fn run_pipeline(
    workdir: &PathBuf,
    version: &str,
    demangle: bool,
    format: OutputFormat,
) -> Result<()> {
    let xml_dir = workdir.join("xml");
    let asl_dir = workdir.join("asl");

    // Step 1: Download
    println!("=== Step 1: Downloading XML specifications ===");
    download_specs(&xml_dir, version, Some(vec![SpecTypeArg::All]))?;

    // Step 2: Extract
    println!("\n=== Step 2: Extracting ASL from XML ===");
    extract_asl(&xml_dir, &asl_dir, version, demangle, None)?;

    // Step 3: Parse
    println!("\n=== Step 3: Parsing extracted ASL ===");

    let defs_path = asl_dir.join("arm_defs.asl");
    if defs_path.exists() {
        println!("\n--- arm_defs.asl ---");
        parse_defs_file(&defs_path, format)?;
    }

    let regs_path = asl_dir.join("arm_regs.asl");
    if regs_path.exists() {
        println!("\n--- arm_regs.asl ---");
        parse_regs_file(&regs_path, format)?;
    }

    let instrs_path = asl_dir.join("arm_instrs.asl");
    if instrs_path.exists() {
        println!("\n--- arm_instrs.asl ---");
        parse_instrs_file(&instrs_path, format)?;
    }

    println!("\n=== Pipeline complete! ===");
    Ok(())
}

fn generate_tests(
    input: &PathBuf,
    output: &PathBuf,
    max_tests: usize,
    include_negative: bool,
    include_execution: bool,
    filter: Option<String>,
    format: TestGenOutputFormat,
    iset: InstructionSetArg,
    structured: bool,
) -> Result<()> {
    use asl_parser::syntax::InstructionSet;
    use regex::Regex;

    println!("=== Test Generation ===");
    println!("Input: {}", input.display());
    println!("Output: {}", output.display());

    // Read and parse the instruction file
    let source = fs::read_to_string(input)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read file: {}", input.display()))?;

    println!("Parsing instructions...");
    let instrs = parse_instructions(&source).map_err(|e| {
        miette::miette!(
            labels = vec![miette::LabeledSpan::at(
                e.span.start..e.span.end,
                e.kind.to_string()
            )],
            "parse error in {}",
            input.display()
        )
        .with_source_code(source.clone())
    })?;

    println!("Parsed {} instructions", instrs.len());

    // Apply filter if provided
    let filter_regex = filter.as_ref().map(|f| {
        Regex::new(f).unwrap_or_else(|_| {
            eprintln!("Warning: Invalid filter regex '{}', ignoring", f);
            Regex::new(".*").unwrap()
        })
    });

    let filtered_instrs: Vec<_> = instrs
        .iter()
        .filter(|i| {
            filter_regex
                .as_ref()
                .map(|r| r.is_match(i.name.as_str()))
                .unwrap_or(true)
        })
        .collect();

    if filtered_instrs.is_empty() {
        println!("No instructions match the filter");
        return Ok(());
    }

    println!(
        "Processing {} instructions (after filter)",
        filtered_instrs.len()
    );

    // Configure test generator
    let target_isets = match iset {
        InstructionSetArg::A64 => vec![InstructionSet::A64],
        InstructionSetArg::A32 => vec![InstructionSet::A32],
        InstructionSetArg::T32 => vec![InstructionSet::T32],
        InstructionSetArg::T16 => vec![InstructionSet::T16],
        InstructionSetArg::All => vec![
            InstructionSet::A64,
            InstructionSet::A32,
            InstructionSet::T32,
            InstructionSet::T16,
        ],
    };

    let config = TestGenConfig {
        max_encoding_tests: max_tests,
        include_negative_tests: include_negative,
        generate_execution_tests: include_execution,
        instruction_sets: target_isets.clone(),
        ..Default::default()
    };

    let mut generator = TestGenerator::with_config(config);

    // Generate tests for each instruction
    let mut all_suites = Vec::new();
    for instr in &filtered_instrs {
        let suite = generator.generate_instruction_tests(instr);
        if !suite.encoding_tests.is_empty() || !suite.execution_tests.is_empty() {
            println!(
                "  {} - {} encoding tests, {} execution tests",
                instr.name,
                suite.encoding_tests.len(),
                suite.execution_tests.len()
            );
            all_suites.push(suite);
        }
    }

    if all_suites.is_empty() {
        println!("No tests generated");
        return Ok(());
    }

    // Deduplicate test suites based on encoding analysis to avoid duplicate test names
    let mut seen_encodings = std::collections::HashSet::new();
    let unique_suites: Vec<_> = all_suites
        .into_iter()
        .filter(|suite| {
            // Create a key from instruction name + first encoding value
            let key = if let Some(first_test) = suite.encoding_tests.first() {
                format!("{}_{:08x}", suite.instruction_name, first_test.encoding)
            } else {
                suite.instruction_name.clone()
            };
            seen_encodings.insert(key)
        })
        .collect();

    let all_suites = unique_suites;

    // Create output directory
    fs::create_dir_all(output)
        .into_diagnostic()
        .wrap_err("Failed to create output directory")?;

    // Handle structured vs single-file output
    if structured {
        use asl_parser::testgen::output::{generate_structured_output, write_structured_output};

        println!("\nGenerating structured output...");
        let files = generate_structured_output(&all_suites);

        // Also generate helper files
        let helper_files = generate_helper_files(&target_isets);

        write_structured_output(output, &files)
            .into_diagnostic()
            .wrap_err("Failed to write structured output")?;

        // Write helper files
        for (path, content) in helper_files {
            let full_path = output.join(path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)
                    .into_diagnostic()
                    .wrap_err("Failed to create helper directory")?;
            }
            fs::write(&full_path, content)
                .into_diagnostic()
                .wrap_err_with(|| {
                    format!("Failed to write helper file: {}", full_path.display())
                })?;
        }

        println!(
            "\nGenerated {} test suites ({} total tests) in {} files",
            all_suites.len(),
            all_suites
                .iter()
                .map(|s| s.encoding_tests.len() + s.execution_tests.len())
                .sum::<usize>(),
            files.len()
        );
        println!("Output directory: {}", output.display());
    } else {
        // Single-file output (original behavior)
        let output_format = match format {
            TestGenOutputFormat::Rust => TestOutputFormat::Rust,
            TestGenOutputFormat::Json => TestOutputFormat::Json,
            TestGenOutputFormat::Text => TestOutputFormat::Text,
        };

        let output_content =
            asl_parser::testgen::TestOutputFormatter::format(&all_suites, output_format);

        let ext = match format {
            TestGenOutputFormat::Rust => "rs",
            TestGenOutputFormat::Json => "json",
            TestGenOutputFormat::Text => "txt",
        };

        let output_file = output.join(format!("generated_tests.{}", ext));
        fs::write(&output_file, &output_content)
            .into_diagnostic()
            .wrap_err("Failed to write output file")?;

        println!(
            "\nGenerated {} test suites ({} total tests)",
            all_suites.len(),
            all_suites
                .iter()
                .map(|s| s.encoding_tests.len() + s.execution_tests.len())
                .sum::<usize>()
        );
        println!("Output: {}", output_file.display());
    }

    Ok(())
}

/// Generate system register tests from arm_regs.asl
fn generate_sysreg_tests(input: &PathBuf, output: &PathBuf) -> Result<()> {
    use asl_parser::testgen::sysreg;

    println!("Parsing register definitions from {}...", input.display());

    let source = fs::read_to_string(input)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read file: {}", input.display()))?;

    let regs = parse_registers(&source).map_err(|e| {
        miette::miette!(
            labels = vec![miette::LabeledSpan::at(
                e.span.start..e.span.end,
                e.kind.to_string()
            )],
            "parse error in {}",
            input.display()
        )
        .with_source_code(source.clone())
    })?;

    println!("Parsed {} register definitions", regs.len());

    // Create output directory
    fs::create_dir_all(output)
        .into_diagnostic()
        .wrap_err("failed to create output directory")?;

    // Generate A64 MRS/MSR tests
    let a64_test_code = sysreg::generate_all_sysreg_tests(&regs);
    let a64_output_file = output.join("sysreg_a64_tests.rs");
    fs::write(&a64_output_file, &a64_test_code)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write {}", a64_output_file.display()))?;
    println!(
        "Generated A64 system register tests: {}",
        a64_output_file.display()
    );

    // Generate A32 CP15 tests
    let a32_test_code = sysreg::generate_all_cp15_tests(&regs);
    let a32_output_file = output.join("sysreg_a32_tests.rs");
    fs::write(&a32_output_file, &a32_test_code)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write {}", a32_output_file.display()))?;
    println!(
        "Generated A32 CP15 register tests: {}",
        a32_output_file.display()
    );

    // Generate mod.rs for the sysreg tests
    let mod_content = r#"//! System register tests.
//!
//! Contains tests for both A64 (MRS/MSR) and A32 (MCR/MRC) system registers.

mod sysreg_a64_tests;
mod sysreg_a32_tests;
"#;
    let mod_file = output.join("mod.rs");
    fs::write(&mod_file, mod_content)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write {}", mod_file.display()))?;

    Ok(())
}

/// Generate helper files for the test infrastructure
fn generate_helper_files(
    target_isets: &[asl_parser::syntax::InstructionSet],
) -> Vec<(&'static str, String)> {
    use asl_parser::syntax::InstructionSet;
    let mut files = Vec::new();

    let has_a64 = target_isets.contains(&InstructionSet::A64);
    let has_a32 = target_isets.iter().any(|i| {
        matches!(
            i,
            InstructionSet::A32 | InstructionSet::T32 | InstructionSet::T16
        )
    });

    // A64 test helpers (only if generating A64 tests)
    if !has_a64 {
        return files;
    }

    let a64_helpers = r#"//! A64 test helpers.
//!
//! Common test infrastructure for AArch64 instruction tests.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use rax::arm::{AArch64Config, AArch64Cpu, FlatMemory};

// Re-export types so tests can use them directly
pub use rax::arm::{ArmCpu, CpuExit};

/// Create a test CPU with default configuration
pub fn create_test_cpu() -> AArch64Cpu {
    let memory = FlatMemory::new(0, 0x1000_0000);
    AArch64Cpu::new(AArch64Config::default(), Box::new(memory))
}

/// Write an instruction to memory
pub fn write_insn(cpu: &mut AArch64Cpu, addr: u64, insn: u32) {
    cpu.write_memory(addr, &insn.to_le_bytes()).unwrap();
}

/// Execute instructions until reaching target address
pub fn run_to(cpu: &mut AArch64Cpu, target_pc: u64) -> CpuExit {
    loop {
        let exit = cpu.step().unwrap();
        if cpu.get_pc() >= target_pc || !matches!(exit, CpuExit::Continue) {
            return exit;
        }
    }
}

/// Set a general purpose register (X0-X30)
pub fn set_x(cpu: &mut AArch64Cpu, reg: u8, value: u64) {
    cpu.set_gpr(reg, value);
}

/// Get a general purpose register (X0-X30)
pub fn get_x(cpu: &AArch64Cpu, reg: u8) -> u64 {
    cpu.get_gpr(reg)
}

/// Set the 32-bit view of a register (W0-W30)
pub fn set_w(cpu: &mut AArch64Cpu, reg: u8, value: u32) {
    cpu.set_gpr(reg, value as u64);
}

/// Get the 32-bit view of a register (W0-W30)
pub fn get_w(cpu: &AArch64Cpu, reg: u8) -> u32 {
    cpu.get_gpr(reg) as u32
}

/// Set SIMD register (V0-V31)
pub fn set_qreg(cpu: &mut AArch64Cpu, reg: u8, value: u128) {
    let low = value as u64;
    let high = (value >> 64) as u64;
    cpu.set_simd_reg(reg, low, high).unwrap();
}

/// Get SIMD register (V0-V31)
pub fn get_qreg(cpu: &AArch64Cpu, reg: u8) -> u128 {
    let (low, high) = cpu.get_simd_reg(reg).unwrap();
    (high as u128) << 64 | (low as u128)
}
"#;
    files.push(("test_helpers.rs", a64_helpers.to_string()));

    // A32/T32/T16 test helpers (only if generating 32-bit tests)
    if !has_a32 {
        return files;
    }

    let a32_helpers = r#"//! A32/T32/T16 test helpers.
//!
//! Common test infrastructure for AArch32 instruction tests.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use rax::arm::{Armv7Cpu, Executor, ExecResult, ExceptionType};
use rax::arm::decoder::{Aarch32Decoder, ThumbDecoder};
use rax::arm::execution::{ArmMemory, FlatMemory};
pub use rax::arm::CpuExit;

/// Processor state for flag assertions
#[derive(Debug, Clone, Copy, Default)]
pub struct PState {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
}

/// Test context combining CPU and memory
pub struct TestCpu {
    pub cpu: Armv7Cpu,
    pub mem: FlatMemory,
}

impl TestCpu {
    /// Execute one instruction
    pub fn step(&mut self) -> Result<CpuExit, String> {
        step(self)
    }
    
    /// Write bytes to memory (for compatibility with generated tests)
    pub fn write_memory(&mut self, addr: u64, bytes: &[u8]) -> Result<(), String> {
        for (i, &b) in bytes.iter().enumerate() {
            self.mem.write_byte((addr as u32) + i as u32, b)
                .map_err(|e| format!("write error: {:?}", e))?;
        }
        Ok(())
    }
    
    /// Read bytes from memory (for compatibility with generated tests)
    pub fn read_memory(&self, addr: u64, size: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.mem.read_byte((addr as u32) + i as u32).unwrap_or(0));
        }
        result
    }
    
    /// Get processor state flags (for compatibility with generated tests)
    pub fn get_pstate(&self) -> PState {
        PState {
            n: self.cpu.cpsr.n,
            z: self.cpu.cpsr.z,
            c: self.cpu.cpsr.c,
            v: self.cpu.cpsr.v,
        }
    }
}

/// Create a test CPU with default configuration (A32 mode)
pub fn create_test_cpu() -> TestCpu {
    let cpu = Armv7Cpu::new();
    let mem = FlatMemory::new(0x1000_0000, 0);  // size, base
    TestCpu { cpu, mem }
}

/// Create a test CPU in Thumb mode
pub fn create_thumb_cpu() -> TestCpu {
    let mut cpu = Armv7Cpu::new();
    cpu.cpsr.t = true;
    let mem = FlatMemory::new(0x1000_0000, 0);  // size, base
    TestCpu { cpu, mem }
}

/// Write a 32-bit instruction to memory
pub fn write_insn(ctx: &mut TestCpu, addr: u32, insn: u32) {
    ctx.mem.write_word(addr, insn).unwrap();
}

/// Write a 16-bit Thumb instruction to memory
pub fn write_insn16(ctx: &mut TestCpu, addr: u32, insn: u16) {
    ctx.mem.write_halfword(addr, insn).unwrap();
}

/// Write bytes to memory
pub fn write_memory(ctx: &mut TestCpu, addr: u32, bytes: &[u8]) {
    for (i, &b) in bytes.iter().enumerate() {
        ctx.mem.write_byte(addr + i as u32, b).unwrap();
    }
}

/// Read bytes from memory
pub fn read_memory(ctx: &TestCpu, addr: u32, size: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(size);
    for i in 0..size {
        result.push(ctx.mem.read_byte(addr + i as u32).unwrap());
    }
    result
}

/// Set a general purpose register (R0-R14)
pub fn set_w(ctx: &mut TestCpu, reg: u8, value: u32) {
    ctx.cpu.set_reg(reg as usize, value);
}

/// Get a general purpose register (R0-R14)
pub fn get_w(ctx: &TestCpu, reg: u8) -> u32 {
    ctx.cpu.reg(reg as usize)
}

/// Set the stack pointer (R13/SP)
pub fn set_sp(ctx: &mut TestCpu, value: u32) {
    ctx.cpu.set_reg(13, value);
}

/// Get the stack pointer (R13/SP)
pub fn get_sp(ctx: &TestCpu) -> u32 {
    ctx.cpu.reg(13)
}

/// Set the link register (R14/LR)
pub fn set_lr(ctx: &mut TestCpu, value: u32) {
    ctx.cpu.set_reg(14, value);
}

/// Get the link register (R14/LR)
pub fn get_lr(ctx: &TestCpu) -> u32 {
    ctx.cpu.reg(14)
}

/// Set the PC register
pub fn set_pc(ctx: &mut TestCpu, value: u32) {
    ctx.cpu.set_reg(15, value);
}

/// Get the PC register
pub fn get_pc(ctx: &TestCpu) -> u32 {
    ctx.cpu.get_pc()
}

/// Execute one instruction and return the exit status
pub fn step(ctx: &mut TestCpu) -> Result<CpuExit, String> {
    let pc = ctx.cpu.get_pc();
    let is_thumb = ctx.cpu.cpsr.t;
    
    // Fetch and decode instruction
    let decoded = if is_thumb {
        let hw = ctx.mem.read_halfword(pc).map_err(|e| format!("fetch error: {:?}", e))?;
        // Check for 32-bit Thumb instruction
        if (hw >> 11) >= 0x1D {
            let hw2 = ctx.mem.read_halfword(pc + 2).map_err(|e| format!("fetch error: {:?}", e))?;
            let insn32 = ((hw as u32) << 16) | (hw2 as u32);
            ThumbDecoder::decode_32bit(insn32).map_err(|e| format!("decode error: {:?}", e))?
        } else {
            ThumbDecoder::decode_16bit(hw).map_err(|e| format!("decode error: {:?}", e))?
        }
    } else {
        let insn = ctx.mem.read_word(pc).map_err(|e| format!("fetch error: {:?}", e))?;
        Aarch32Decoder::decode(insn).map_err(|e| format!("decode error: {:?}", e))?
    };
    
    // Calculate instruction size for PC advance
    let insn_size = if is_thumb {
        let hw = ctx.mem.read_halfword(pc).unwrap();
        if (hw >> 11) >= 0x1D { 4 } else { 2 }
    } else {
        4
    };
    
    // Execute
    let mut exec = Executor::new(&mut ctx.cpu, &mut ctx.mem);
    match exec.execute(&decoded) {
        ExecResult::Continue => {
            // Advance PC
            ctx.cpu.set_reg(15, pc + insn_size);
            Ok(CpuExit::Continue)
        }
        ExecResult::Branch(addr) => {
            ctx.cpu.set_reg(15, addr);
            Ok(CpuExit::Continue)
        }
        ExecResult::Halt => Ok(CpuExit::Halt),
        ExecResult::Undefined => Ok(CpuExit::Undefined(0)),
        ExecResult::Exception(exc) => {
            match exc {
                ExceptionType::SupervisorCall(imm) => Ok(CpuExit::Svc(imm)),
                ExceptionType::UndefinedInstruction => Ok(CpuExit::Undefined(0)),
                ExceptionType::Breakpoint(imm) => Ok(CpuExit::Breakpoint(imm as u32)),
                _ => Err(format!("exception: {:?}", exc)),
            }
        }
        ExecResult::MemoryFault(e) => Err(format!("memory fault: {:?}", e)),
    }
}
"#;
    files.push(("test_helpers_32.rs", a32_helpers.to_string()));

    files
}

fn print_def_summary(def: &asl_parser::Definition) {
    use asl_parser::Definition;
    match def {
        Definition::TypeBuiltin(name) => {
            println!("  builtin type {}", name);
        }
        Definition::TypeAbstract(name) => {
            println!("  type {}", name);
        }
        Definition::TypeAlias { name, .. } => {
            println!("  type {} = ...", name);
        }
        Definition::TypeStruct { name, fields } => {
            println!("  type {} is ({} fields)", name, fields.len());
        }
        Definition::TypeEnum { name, variants } => {
            println!("  enumeration {} {{ {} variants }}", name, variants.len());
        }
        Definition::Variable { name, .. } => {
            println!("  var {}", name);
        }
        Definition::Const { name, .. } => {
            println!("  constant {}", name);
        }
        Definition::Array { name, .. } => {
            println!("  array {}", name);
        }
        Definition::Callable {
            name,
            params,
            returns,
            ..
        } => {
            let kind = if returns.is_empty() {
                "procedure"
            } else {
                "function"
            };
            println!("  {} {}({} params)", kind, name, params.len());
        }
        Definition::Getter { name, params, .. } => {
            let idx = params.as_ref().map(|p| p.len()).unwrap_or(0);
            if idx > 0 {
                println!("  getter {}[{}]", name, idx);
            } else {
                println!("  getter {}", name);
            }
        }
        Definition::Setter { name, params, .. } => {
            let idx = params.as_ref().map(|p| p.len()).unwrap_or(0);
            if idx > 0 {
                println!("  setter {}[{}]", name, idx);
            } else {
                println!("  setter {}", name);
            }
        }
        Definition::Instruction(instr) => {
            println!(
                "  instruction {} ({} encodings)",
                instr.name,
                instr.encodings.len()
            );
        }
        Definition::Decode { iset, cases } => {
            println!("  decode {} ({} cases)", iset, cases.len());
        }
    }
}
