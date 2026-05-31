//! Headless CLI wrapper around `kortex_kvcache`.
//!
//! Lets users (and the launch-kortex.ps1 script) start the KV-cache proxy
//! without booting the Tauri IDE. Mirrors the layout of `kortex_gac_cli.rs`.
//!
//! Usage:
//!   kortex-kvcache-cli serve --upstream http://127.0.0.1:8081 \
//!                            [--port 8090] [--host 127.0.0.1] \
//!                            [--base <dir>] [--max-gb 16] [--slot-id 0]
//!   kortex-kvcache-cli stats --base <dir>
//!   kortex-kvcache-cli clear --base <dir>

use std::path::PathBuf;
use std::process::ExitCode;

use vscode_rust_app_lib::kortex_kvcache::{
    proxy::{self, ProxyState, SharedProxy},
    types::KvCacheOptions,
    CacheStore, LlamaCppClient,
};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage();
        return ExitCode::from(2);
    }
    let result = match args[0].as_str() {
        "serve" => cmd_serve(&args[1..]),
        "stats" => cmd_stats(&args[1..]),
        "clear" => cmd_clear(&args[1..]),
        "help" | "-h" | "--help" => {
            print_usage();
            Ok(())
        }
        other => Err(anyhow::anyhow!("unknown subcommand '{}'", other)),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kortex-kvcache-cli: {}", e);
            ExitCode::from(1)
        }
    }
}

fn print_usage() {
    eprintln!("usage:");
    eprintln!("  kortex-kvcache-cli serve --upstream http://127.0.0.1:8081");
    eprintln!("    [--host 127.0.0.1] [--port 8090] [--base <dir>] [--max-gb 16] [--slot-id 0]");
    eprintln!("  kortex-kvcache-cli stats --base <dir>");
    eprintln!("  kortex-kvcache-cli clear --base <dir>");
}

// ─────────────────────────── serve ──────────────────────────────────────────

fn cmd_serve(rest: &[String]) -> anyhow::Result<()> {
    let mut opts = KvCacheOptions::default();
    let mut base: Option<PathBuf> = None;
    let mut i = 0;
    while i < rest.len() {
        let arg = &rest[i];
        match arg.as_str() {
            "--upstream" => {
                opts.upstream_url = need_val(rest, &mut i, "--upstream")?;
            }
            "--host" => opts.proxy_host = need_val(rest, &mut i, "--host")?,
            "--port" => {
                opts.proxy_port = need_val(rest, &mut i, "--port")?
                    .parse()
                    .map_err(|e| anyhow::anyhow!("--port: {}", e))?;
            }
            "--base" => {
                base = Some(PathBuf::from(need_val(rest, &mut i, "--base")?));
            }
            "--max-gb" => {
                let gb: u64 = need_val(rest, &mut i, "--max-gb")?
                    .parse()
                    .map_err(|e| anyhow::anyhow!("--max-gb: {}", e))?;
                opts.max_bytes = gb.saturating_mul(1024 * 1024 * 1024);
            }
            "--slot-id" => {
                opts.slot_id = need_val(rest, &mut i, "--slot-id")?
                    .parse()
                    .map_err(|e| anyhow::anyhow!("--slot-id: {}", e))?;
            }
            "--min-tokens" => {
                opts.min_tokens = need_val(rest, &mut i, "--min-tokens")?
                    .parse()
                    .map_err(|e| anyhow::anyhow!("--min-tokens: {}", e))?;
            }
            "--align" => {
                opts.boundary_align_tokens = need_val(rest, &mut i, "--align")?
                    .parse()
                    .map_err(|e| anyhow::anyhow!("--align: {}", e))?;
            }
            other => return Err(anyhow::anyhow!("unknown flag '{}'", other)),
        }
        i += 1;
    }

    if let Some(b) = base {
        opts.index_dir = b.join("index");
        opts.slot_dir = b.join("slots");
    }

    eprintln!(
        "[kortex-kvcache-cli] serve {}:{} → {} (index={}, slots={}, budget={} GB)",
        opts.proxy_host,
        opts.proxy_port,
        opts.upstream_url,
        opts.index_dir.display(),
        opts.slot_dir.display(),
        opts.max_bytes / (1024 * 1024 * 1024)
    );

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(serve_async(opts))
}

async fn serve_async(opts: KvCacheOptions) -> anyhow::Result<()> {
    // Smoke-check the upstream so we fail loudly instead of returning 502s.
    let probe = LlamaCppClient::new(&opts.upstream_url, opts.slot_id);
    if let Err(e) = probe.health().await {
        return Err(anyhow::anyhow!("upstream not healthy: {}", e));
    }

    let store = CacheStore::open(opts.clone())?;
    let state: SharedProxy = std::sync::Arc::new(ProxyState::new(opts, store));

    let _handle = proxy::serve(state.clone()).await?;

    // Block on Ctrl-C. tokio's signal handling differs by platform but ctrl_c
    // covers both Windows and Unix and is enough for a CLI wrapper.
    tokio::signal::ctrl_c().await.ok();
    eprintln!("[kortex-kvcache-cli] shutting down...");
    proxy::shutdown(&state).await;
    Ok(())
}

// ─────────────────────────── stats ──────────────────────────────────────────

fn cmd_stats(rest: &[String]) -> anyhow::Result<()> {
    let base = parse_base_required(rest)?;
    let mut opts = KvCacheOptions::default();
    opts.index_dir = base.join("index");
    opts.slot_dir = base.join("slots");
    let store = CacheStore::open(opts)?;
    let stats = store.stats();
    let total = stats.hits + stats.misses;
    let hit_rate = if total > 0 {
        (stats.hits as f64) / (total as f64) * 100.0
    } else {
        0.0
    };
    println!("entries        : {}", stats.entries);
    println!(
        "on-disk bytes  : {} ({:.2} GB)",
        stats.total_bytes,
        stats.total_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!("hits           : {}", stats.hits);
    println!("misses         : {}", stats.misses);
    println!("hit rate       : {:.1}%", hit_rate);
    println!("saves          : {}", stats.saves);
    println!("evictions      : {}", stats.evictions);
    println!("tokens skipped : {}", stats.tokens_skipped);
    Ok(())
}

// ─────────────────────────── clear ──────────────────────────────────────────

fn cmd_clear(rest: &[String]) -> anyhow::Result<()> {
    let base = parse_base_required(rest)?;
    let index_dir = base.join("index");
    let slot_dir = base.join("slots");
    let _ = std::fs::remove_dir_all(&index_dir);
    let _ = std::fs::remove_dir_all(&slot_dir);
    std::fs::create_dir_all(&index_dir)?;
    std::fs::create_dir_all(&slot_dir)?;
    println!("cleared {} and {}", index_dir.display(), slot_dir.display());
    Ok(())
}

// ─────────────────────────── helpers ────────────────────────────────────────

fn parse_base_required(rest: &[String]) -> anyhow::Result<PathBuf> {
    let mut i = 0;
    while i < rest.len() {
        if rest[i] == "--base" {
            return Ok(PathBuf::from(need_val(rest, &mut i, "--base")?));
        }
        i += 1;
    }
    Err(anyhow::anyhow!("missing required --base <dir>"))
}

fn need_val(args: &[String], i: &mut usize, flag: &str) -> anyhow::Result<String> {
    *i += 1;
    args.get(*i)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("{} expects a value", flag))
}
