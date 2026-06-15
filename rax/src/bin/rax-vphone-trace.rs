use std::path::PathBuf;

use clap::Parser;
use rax::Result;
use rax::vphone_trace::{
    emit_trace_jsonl, enrich_findings_from_trace, load_vphone_image, summary_json, write_findings,
};

#[derive(Parser, Debug)]
#[command(
    name = "rax-vphone-trace",
    about = "vphone600ap trace sidecar for Virtual-iPhone-Emulator"
)]
struct Cli {
    /// Raw iOS kernelcache payload, e.g. Virtual-iPhone-Emulator/vm/kernelcache.raw.bin
    #[arg(long)]
    kernel: PathBuf,
    /// Raw flattened device tree payload, not IM4P-wrapped.
    #[arg(long)]
    dtb: PathBuf,
    /// JSONL trace output path.
    #[arg(long)]
    trace: PathBuf,
    /// Optional Virtual-iPhone-Emulator/vm/progress.log to rank current spin PCs.
    #[arg(long)]
    progress_log: Option<PathBuf>,
    /// Optional live rax JSONL trace captured with RAX_TRACE_JSONL, used to enrich hot PCs.
    #[arg(long)]
    runtime_trace: Option<PathBuf>,
    /// JSON findings output path for the C++ emulator handoff.
    #[arg(long)]
    findings: Option<PathBuf>,
    /// Maximum synthetic loop events to write from the progress-log ranking.
    #[arg(long, default_value = "100000")]
    max_insns: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let image = load_vphone_image(&cli.kernel, &cli.dtb)?;
    let mut findings = emit_trace_jsonl(
        &image,
        cli.progress_log.as_deref(),
        &cli.trace,
        cli.max_insns,
    )?;
    if let Some(path) = &cli.runtime_trace {
        enrich_findings_from_trace(&mut findings, path)?;
    }
    if let Some(path) = &cli.findings {
        write_findings(path, &findings)?;
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&summary_json(&image, &findings))
            .map_err(|e| rax::error::Error::InvalidConfig(e.to_string()))?
    );
    Ok(())
}
