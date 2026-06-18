use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use rax::config::{Endianness, HexagonIsa};
use rax::isa_oracle::{
    ArmState, OracleIsa, OracleOptions, OracleSeed, RiscVIsaProfile, decode_to_json,
    decode_to_json_with_seed, parse_hex_bytes,
};
use rax::riscv::Xlen;
use rax::smir::SourceArch;

#[derive(Parser, Debug)]
#[command(about = "Decode ISA bytes with rax decoders and emit a JSON oracle record")]
struct Cli {
    #[arg(value_enum)]
    isa: CliIsa,

    /// Instruction or packet bytes as hex. Separators and 0x prefixes are accepted.
    bytes: String,

    /// Guest PC used for relative control-flow summaries.
    #[arg(long, default_value = "0x1000", value_parser = parse_u64)]
    pc: u64,

    /// Read bytes from a file instead of the hex argument.
    #[arg(long)]
    bytes_file: Option<PathBuf>,

    /// ARM decoder state.
    #[arg(long, value_enum, default_value = "aarch64")]
    arm_state: CliArmState,

    /// Hexagon ISA version.
    #[arg(long, value_enum, default_value = "v68")]
    hexagon_isa: CliHexagonIsa,

    /// Hexagon packet byte order.
    #[arg(long, value_enum, default_value = "little")]
    hexagon_endian: CliEndian,

    /// RISC-V XLEN.
    #[arg(long, value_enum, default_value = "rv64")]
    riscv_xlen: CliRiscvXlen,

    /// RISC-V ISA profile.
    #[arg(long, value_enum, default_value = "rv64gc")]
    riscv_isa: CliRiscvIsa,

    /// Source architecture for the smir subcommand.
    #[arg(long, value_enum, default_value = "x86-64")]
    smir_source: CliSmirSource,

    /// Do not include SMIR lift output alongside native decode output.
    #[arg(long)]
    no_smir: bool,

    /// JSON seed for optional side-effect execution through the SMIR interpreter.
    #[arg(long)]
    seed_json: Option<PathBuf>,

    /// Pretty-print JSON.
    #[arg(long)]
    pretty: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliIsa {
    X86_64,
    X86,
    Arm,
    Hexagon,
    Riscv,
    Smir,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliArmState {
    Aarch64,
    Aarch32,
    Thumb,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliHexagonIsa {
    V4,
    V5,
    V55,
    V60,
    V62,
    V65,
    V66,
    V67,
    V68,
    V69,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliEndian {
    Little,
    Big,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliRiscvXlen {
    Rv32,
    Rv64,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliRiscvIsa {
    Rv64gc,
    Rv64i,
    Rv64imac,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliSmirSource {
    #[value(name = "x86-64", alias = "x86_64", alias = "x86")]
    X86_64,
    Aarch64,
    Hexagon,
    Riscv64,
    Riscv32,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("rax-isa-oracle: {err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let bytes = if let Some(path) = &cli.bytes_file {
        fs::read(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?
    } else {
        parse_hex_bytes(&cli.bytes)?
    };

    let opts = OracleOptions {
        isa: match cli.isa {
            CliIsa::X86_64 | CliIsa::X86 => OracleIsa::X86_64,
            CliIsa::Arm => OracleIsa::Arm,
            CliIsa::Hexagon => OracleIsa::Hexagon,
            CliIsa::Riscv => OracleIsa::RiscV,
            CliIsa::Smir => OracleIsa::Smir,
        },
        pc: cli.pc,
        arm_state: match cli.arm_state {
            CliArmState::Aarch64 => ArmState::Aarch64,
            CliArmState::Aarch32 => ArmState::Aarch32,
            CliArmState::Thumb => ArmState::Thumb,
        },
        hexagon_isa: match cli.hexagon_isa {
            CliHexagonIsa::V4 => HexagonIsa::V4,
            CliHexagonIsa::V5 => HexagonIsa::V5,
            CliHexagonIsa::V55 => HexagonIsa::V55,
            CliHexagonIsa::V60 => HexagonIsa::V60,
            CliHexagonIsa::V62 => HexagonIsa::V62,
            CliHexagonIsa::V65 => HexagonIsa::V65,
            CliHexagonIsa::V66 => HexagonIsa::V66,
            CliHexagonIsa::V67 => HexagonIsa::V67,
            CliHexagonIsa::V68 => HexagonIsa::V68,
            CliHexagonIsa::V69 => HexagonIsa::V69,
        },
        hexagon_endian: match cli.hexagon_endian {
            CliEndian::Little => Endianness::Little,
            CliEndian::Big => Endianness::Big,
        },
        riscv_xlen: match cli.riscv_xlen {
            CliRiscvXlen::Rv32 => Xlen::Rv32,
            CliRiscvXlen::Rv64 => Xlen::Rv64,
        },
        riscv_isa: match cli.riscv_isa {
            CliRiscvIsa::Rv64gc => RiscVIsaProfile::Rv64Gc,
            CliRiscvIsa::Rv64i => RiscVIsaProfile::Rv64I,
            CliRiscvIsa::Rv64imac => RiscVIsaProfile::Rv64Imac,
        },
        smir_source: match cli.smir_source {
            CliSmirSource::X86_64 => SourceArch::X86_64,
            CliSmirSource::Aarch64 => SourceArch::Aarch64,
            CliSmirSource::Hexagon => SourceArch::Hexagon,
            CliSmirSource::Riscv64 => SourceArch::RiscV64,
            CliSmirSource::Riscv32 => SourceArch::RiscV32,
        },
        include_smir: !cli.no_smir,
    };

    let seed = if let Some(path) = &cli.seed_json {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("failed to read seed JSON {}: {e}", path.display()))?;
        let value: serde_json::Value = serde_json::from_str(&raw)
            .map_err(|e| format!("failed to parse seed JSON {}: {e}", path.display()))?;
        Some(OracleSeed::from_json(&value)?)
    } else {
        None
    };

    let value = if let Some(seed) = &seed {
        decode_to_json_with_seed(&bytes, &opts, Some(seed))?
    } else {
        decode_to_json(&bytes, &opts)?
    };
    if cli.pretty {
        println!(
            "{}",
            serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?
        );
    } else {
        println!(
            "{}",
            serde_json::to_string(&value).map_err(|e| e.to_string())?
        );
    }
    Ok(())
}

fn parse_u64(raw: &str) -> Result<u64, String> {
    let s = raw.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).map_err(|e| e.to_string())
    } else {
        s.parse::<u64>().map_err(|e| e.to_string())
    }
}
