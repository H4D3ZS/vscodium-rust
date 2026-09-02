//! Minimal CLI wrapper around the `kortex_gac` library so launch scripts and
//! CI pipelines can profile/plan without booting the Tauri IDE.
//!
//! Usage:
//!   kortex-gac-cli profile --model <gguf>
//!   kortex-gac-cli plan --profile <profile.aim> --vram-mb 8192 --theta 0.85 \
//!                       --backend vulkan --output <plan.json>
//!   kortex-gac-cli launch --plan <plan.json> --model <gguf> [--server <bin>] \
//!                         [--port 8081] [--ctx 8192]
//!
//! Argument parsing is hand-rolled to keep the dep tree identical to the
//! library crate (no clap, no structopt).

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use vscode_rust_app_lib::kortex_gac::{
    build_argv, default_profile_path, launcher, plan_tiers, planner, profile_gguf, read_profile,
    render_args, resolve_server_binary, write_profile, LaunchOpts, PlanOptions, ProfilerConfig,
    TierPlan,
};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage();
        return ExitCode::from(2);
    }
    let result = match args[0].as_str() {
        "profile" => cmd_profile(&args[1..]),
        "plan" => cmd_plan(&args[1..]),
        "launch" => cmd_launch(&args[1..]),
        "render" => cmd_render(&args[1..]),
        "help" | "-h" | "--help" => {
            print_usage();
            Ok(())
        }
        other => Err(anyhow::anyhow!("unknown subcommand '{}'", other)),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kortex-gac-cli: {}", e);
            ExitCode::from(1)
        }
    }
}

fn print_usage() {
    eprintln!("usage:");
    eprintln!("  kortex-gac-cli profile --model <gguf>");
    eprintln!("  kortex-gac-cli plan --profile <p.aim> [--vram-mb 8192] [--theta 0.85]");
    eprintln!("                      [--backend vulkan|cuda|rocm|metal|sycl] [--output <p.json>]");
    eprintln!("  kortex-gac-cli render --plan <plan.json>");
    eprintln!("  kortex-gac-cli launch --plan <plan.json> --model <gguf>");
    eprintln!("                        [--server <llama-server>] [--port 8081] [--ctx 8192]");
    eprintln!("                        [--threads 0] [--batch 512] [--flash-attn] [--no-wait]");
}

// ─── profile ────────────────────────────────────────────────────────────────

fn cmd_profile(args: &[String]) -> anyhow::Result<()> {
    let mut model: Option<PathBuf> = None;
    let mut sample_rows: u32 = 256;
    let mut seed: u64 = 0xC0FFEE;
    let mut output: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--model" => {
                model = Some(PathBuf::from(arg_value(args, &mut i)?));
            }
            "--sample-rows" => {
                sample_rows = arg_value(args, &mut i)?.parse()?;
            }
            "--seed" => {
                seed = arg_value(args, &mut i)?.parse()?;
            }
            "--output" => {
                output = Some(PathBuf::from(arg_value(args, &mut i)?));
            }
            other => return Err(anyhow::anyhow!("unknown flag '{}'", other)),
        }
    }
    let model = model.ok_or_else(|| anyhow::anyhow!("--model is required"))?;
    let cfg = ProfilerConfig { sample_rows, seed };
    let profile = profile_gguf(&model, &cfg)?;
    let out = output.unwrap_or_else(|| default_profile_path(&model));
    write_profile(&profile, &out)?;
    println!("{}", out.display());
    Ok(())
}

// ─── plan ───────────────────────────────────────────────────────────────────

fn cmd_plan(args: &[String]) -> anyhow::Result<()> {
    let mut profile_path: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut opts = PlanOptions::default();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--profile" => profile_path = Some(PathBuf::from(arg_value(args, &mut i)?)),
            "--output" => output = Some(PathBuf::from(arg_value(args, &mut i)?)),
            "--vram-mb" => opts.vram_total_mb = arg_value(args, &mut i)?.parse()?,
            "--kv-reserve" => opts.kv_reserve_frac = arg_value(args, &mut i)?.parse()?,
            "--theta" => opts.theta = arg_value(args, &mut i)?.parse()?,
            "--d-eff-global" => opts.d_eff_global = Some(arg_value(args, &mut i)?.parse()?),
            "--safe-mult" => opts.safe_mult = arg_value(args, &mut i)?.parse()?,
            "--unsafe-mult" => opts.unsafe_mult = arg_value(args, &mut i)?.parse()?,
            "--backend" => opts.backend = arg_value(args, &mut i)?,
            other => return Err(anyhow::anyhow!("unknown flag '{}'", other)),
        }
    }
    let profile_path = profile_path.ok_or_else(|| anyhow::anyhow!("--profile is required"))?;
    let profile = read_profile(&profile_path)?;
    let plan = plan_tiers(&profile, &opts)?;
    let json = serde_json::to_string_pretty(&plan)?;

    if let Some(out) = output {
        std::fs::write(&out, &json)?;
        println!("{}", out.display());
    } else {
        println!("{}", json);
    }

    eprintln!(
        "[kortex-gac-cli] plan: GPU={:.2} GB, CPU={:.2} GB, spread→GPU={}, tight→CPU={}, theta={}, d_bar_crit={:.4}",
        plan.total_gpu_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
        plan.total_cpu_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
        plan.routing_counts.spread_to_gpu,
        plan.routing_counts.tight_to_cpu,
        plan.theta,
        plan.d_bar_critical,
    );
    Ok(())
}

// ─── render ─────────────────────────────────────────────────────────────────

fn cmd_render(args: &[String]) -> anyhow::Result<()> {
    let mut plan_path: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--plan" => plan_path = Some(PathBuf::from(arg_value(args, &mut i)?)),
            other => return Err(anyhow::anyhow!("unknown flag '{}'", other)),
        }
    }
    let plan_path = plan_path.ok_or_else(|| anyhow::anyhow!("--plan is required"))?;
    let plan: TierPlan = serde_json::from_str(&std::fs::read_to_string(&plan_path)?)?;
    let argv = render_args(&plan);
    for a in argv {
        println!("{}", a);
    }
    Ok(())
}

// ─── launch ─────────────────────────────────────────────────────────────────

fn cmd_launch(args: &[String]) -> anyhow::Result<()> {
    let mut plan_path: Option<PathBuf> = None;
    let mut model_path: Option<PathBuf> = None;
    let mut server_binary: Option<PathBuf> = None;
    let mut opts = LaunchOpts::default();
    let mut wait = true;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--plan" => plan_path = Some(PathBuf::from(arg_value(args, &mut i)?)),
            "--model" => model_path = Some(PathBuf::from(arg_value(args, &mut i)?)),
            "--server" => server_binary = Some(PathBuf::from(arg_value(args, &mut i)?)),
            "--port" => opts.port = arg_value(args, &mut i)?.parse()?,
            "--host" => opts.host = arg_value(args, &mut i)?,
            "--ctx" => opts.ctx_size = arg_value(args, &mut i)?.parse()?,
            "--threads" => opts.n_threads = arg_value(args, &mut i)?.parse()?,
            "--batch" => opts.batch_size = arg_value(args, &mut i)?.parse()?,
            "--flash-attn" => opts.flash_attn = true,
            "--no-wait" => wait = false,
            "--extra" => opts.extra_args.push(arg_value(args, &mut i)?),
            other => return Err(anyhow::anyhow!("unknown flag '{}'", other)),
        }
    }
    let plan_path = plan_path.ok_or_else(|| anyhow::anyhow!("--plan is required"))?;
    let model_path = model_path.ok_or_else(|| anyhow::anyhow!("--model is required"))?;

    let plan: TierPlan = serde_json::from_str(&std::fs::read_to_string(&plan_path)?)?;
    opts.model_path = model_path;
    opts.server_binary = resolve_server_binary(server_binary.as_deref().map(Path::new))?;

    // Print the full argv so the user can see what's about to run.
    let argv = build_argv(&plan, &opts);
    eprintln!(
        "[kortex-gac-cli] launching {} {}",
        opts.server_binary.display(),
        argv.join(" ")
    );

    let port = launcher::launch(&plan, &opts)?;
    if wait {
        let host = opts.host.clone();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        rt.block_on(async { launcher::await_healthy(&host, port, 120).await })?;
    }
    if let Some(info) = launcher::current_server_info() {
        println!("{}", serde_json::to_string_pretty(&info)?);
    }
    Ok(())
}

// ─── helpers ────────────────────────────────────────────────────────────────

fn arg_value(args: &[String], i: &mut usize) -> anyhow::Result<String> {
    let flag = args[*i].clone();
    *i += 1;
    if *i >= args.len() {
        return Err(anyhow::anyhow!("missing value for {}", flag));
    }
    let v = args[*i].clone();
    *i += 1;
    Ok(v)
}

// Force the planner module to stay linked even though we don't call it directly
// at the top level (rust-analyzer otherwise warns about unused import paths).
#[allow(dead_code)]
fn _keep_planner_link() {
    let _ = planner::backend_buffer_name;
}
