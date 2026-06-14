//! Dynamic iOS boot harness CLI: actually executes the vphone600ap kernelcache
//! on rax's AArch64 interpreter and reports which device poll the kernel hits.
//!
//! Unlike `rax-vphone-trace` (a static analyzer over a progress log), this runs
//! real instructions. With `--trace`, it writes a rax-compatible JSONL stream of
//! the device accesses observed during execution.

use std::path::PathBuf;

use clap::Parser;
use rax::Result;
use rax::vphone_boot::{BootConfig, boot_and_trace};

#[derive(Parser, Debug)]
#[command(
    name = "rax-vphone-boot",
    about = "Execute the vphone600ap iOS kernelcache on the rax AArch64 interpreter"
)]
struct Cli {
    /// Raw iOS kernelcache payload (Mach-O), e.g. Virtual-iPhone-Emulator/vm/kernelcache.raw.bin
    #[arg(long)]
    kernel: PathBuf,
    /// Raw flattened device tree payload (not IM4P-wrapped).
    #[arg(long)]
    dtb: PathBuf,
    /// Optional JSONL trace output path (device accesses + metadata).
    #[arg(long)]
    trace: Option<PathBuf>,
    /// JSON findings output path.
    #[arg(long)]
    findings: Option<PathBuf>,
    /// RAM window size in MiB mapped at 0x80000000.
    #[arg(long, default_value = "1536")]
    ram_mib: usize,
    /// Maximum instructions to execute before stopping.
    #[arg(long, default_value = "5000000")]
    max_insns: u64,
    /// Stop after this many consecutive WFI/WFE/HLT (no interrupt source yet).
    #[arg(long, default_value = "4096")]
    wfi_budget: u64,
    /// Apply the experimental iBoot-equivalent boot-CPU seed (CpuDataEntries[0]).
    /// Note: the kernel appears to clear/populate this table itself, so the
    /// static seed is currently a no-op; kept for experimentation.
    #[arg(long, default_value = "false")]
    seed: bool,
    /// RAM write watchpoint address (hex, e.g. 0x837ecee0): logs writes to it.
    #[arg(long)]
    watch: Option<String>,
    /// Stop at this PC (hex) and dump X0/X3 as C-strings (e.g. panic entry).
    #[arg(long)]
    break_pc: Option<String>,
}

/// Verbose diagnostics, compiled in only with `--features vphone-diag`. Default
/// release builds stay quiet (no boot-internals in stderr or in the binary's
/// string table).
#[cfg(feature = "vphone-diag")]
macro_rules! diag {
    ($($a:tt)*) => { eprintln!($($a)*) };
}
#[cfg(not(feature = "vphone-diag"))]
macro_rules! diag {
    ($($a:tt)*) => {{}};
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // The JSONL writer in vphone_trace initializes once from RAX_TRACE_JSONL; set
    // it from --trace before the first recorded event so the run is captured.
    if let Some(trace) = &cli.trace {
        // SAFETY: single-threaded startup, before any trace writer access.
        unsafe {
            std::env::set_var("RAX_TRACE_JSONL", trace);
        }
    }

    let cfg = BootConfig {
        kernel: cli.kernel,
        dtb: cli.dtb,
        ram_mib: cli.ram_mib,
        max_insns: cli.max_insns,
        wfi_budget: cli.wfi_budget,
        seed: cli.seed,
        watch: cli.watch.as_deref().and_then(|s| {
            u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()
        }),
        break_pc: cli.break_pc.as_deref().and_then(|s| {
            u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()
        }),
    };

    let run = boot_and_trace(&cfg)?;

    diag!(
        "[rax-vphone-boot] executed {} instructions; stop: {}",
        run.instructions, run.stop_reason
    );
    if let Some((gpa, dev)) = &run.last_device {
        diag!("[rax-vphone-boot] last device access: {dev} at {gpa:#x}");
    }
    diag!(
        "[rax-vphone-boot] final state: pc={:#x} sp={:#x} EL{} sctlr_el1={:#x}",
        run.pc, run.sp, run.el, run.sctlr_el1
    );
    for row in 0..8 {
        let mut line = String::new();
        for col in 0..4 {
            let i = row * 4 + col;
            if i < 31 {
                line.push_str(&format!("x{i:<2}={:#018x}  ", run.regs[i]));
            }
        }
        diag!("[rax-vphone-boot]   {line}");
    }

    let json = serde_json::to_string_pretty(&run.findings)
        .map_err(|e| rax::error::Error::InvalidConfig(e.to_string()))?;
    if let Some(path) = &cli.findings {
        std::fs::write(path, format!("{json}\n"))?;
    }
    println!("{json}");
    Ok(())
}
