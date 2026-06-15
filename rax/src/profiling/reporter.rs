//! Profiling report generation (console and JSON output).

use std::path::Path;

use serde::Serialize;

use super::collector::{self, Profiler};
use super::mnemonics::{self, InstructionCategory};
use super::stats::{InstructionReport, OpcodeKey};

/// Print live statistics to stderr.
pub fn print_live_stats() {
    collector::with_profiler(|profiler| {
        let elapsed = profiler.elapsed();
        let total = collector::total_instructions();
        let (cache_hits, cache_misses) = collector::cache_stats();

        let ips = if elapsed.as_secs_f64() > 0.0 {
            total as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        let cache_total = cache_hits + cache_misses;
        let cache_hit_rate = if cache_total > 0 {
            cache_hits as f64 / cache_total as f64 * 100.0
        } else {
            0.0
        };

        eprintln!();
        eprintln!(
            "=== Profiling Stats ({:.2}s, {} insns, {:.2}M IPS) ===",
            elapsed.as_secs_f64(),
            format_count(total),
            ips / 1_000_000.0
        );

        // Collect top instructions by count
        let mut top: Vec<(usize, &super::stats::OpcodeStats)> = profiler
            .stats()
            .iter()
            .enumerate()
            .filter(|(_, s)| s.count > 0)
            .collect();
        top.sort_by(|a, b| b.1.count.cmp(&a.1.count));

        eprintln!();
        eprintln!("Top 10 Instructions by Count:");
        eprintln!(
            "{:<12} {:<30} {:>12} {:>10} {:>10} {:>10}",
            "Opcode", "Mnemonic", "Count", "Mean(ns)", "StdDev", "P99(ns)"
        );
        eprintln!("{}", "-".repeat(86));

        for (idx, stats) in top.iter().take(10) {
            if let Some(key) = OpcodeKey::from_index(*idx) {
                let mnemonic = mnemonics::get_mnemonic(&key);
                // Truncate mnemonic if too long
                let mnemonic_display = if mnemonic.len() > 28 {
                    format!("{}...", &mnemonic[..25])
                } else {
                    mnemonic.to_string()
                };
                eprintln!(
                    "{:<12} {:<30} {:>12} {:>10.1} {:>10.1} {:>10}",
                    key.format(),
                    mnemonic_display,
                    format_count(stats.count),
                    stats.mean_nanos(),
                    stats.stddev(),
                    stats.p99()
                );
            }
        }

        // Memory stats
        let (total_reads, total_writes, total_bytes_read, total_bytes_written) =
            compute_memory_totals(profiler);

        eprintln!();
        eprintln!(
            "Decode Cache: {:.2}% hit rate ({} hits, {} misses)",
            cache_hit_rate,
            format_count(cache_hits),
            format_count(cache_misses)
        );
        eprintln!(
            "Memory: {} reads ({}), {} writes ({})",
            format_count(total_reads),
            format_bytes(total_bytes_read),
            format_count(total_writes),
            format_bytes(total_bytes_written)
        );
    });
}

/// Print a full summary report (called at shutdown).
pub fn print_summary(profiler: &Profiler) {
    let elapsed = profiler.elapsed();
    let total = collector::total_instructions();
    let (cache_hits, cache_misses) = collector::cache_stats();

    let ips = if elapsed.as_secs_f64() > 0.0 {
        total as f64 / elapsed.as_secs_f64()
    } else {
        0.0
    };

    let cache_total = cache_hits + cache_misses;
    let cache_hit_rate = if cache_total > 0 {
        cache_hits as f64 / cache_total as f64 * 100.0
    } else {
        0.0
    };

    eprintln!();
    eprintln!("╔════════════════════════════════════════════════════════════════════════════════╗");
    eprintln!("║                         PROFILING SUMMARY REPORT                               ║");
    eprintln!("╠════════════════════════════════════════════════════════════════════════════════╣");
    eprintln!(
        "║ Duration: {:<15} Total Instructions: {:<20} ║",
        format!("{:.3}s", elapsed.as_secs_f64()),
        format_count(total)
    );
    eprintln!(
        "║ IPS: {:<20} Decode Cache Hit Rate: {:<15} ║",
        format!("{:.2}M", ips / 1_000_000.0),
        format!("{:.2}%", cache_hit_rate)
    );
    eprintln!("╠════════════════════════════════════════════════════════════════════════════════╣");

    // Category breakdown
    let categories = compute_category_breakdown(profiler);
    eprintln!("║                           INSTRUCTION CATEGORIES                               ║");
    eprintln!("╠════════════════════════════════════════════════════════════════════════════════╣");
    eprintln!(
        "║ Arithmetic: {:<12} Logic: {:<12} Data Movement: {:<12} ║",
        format_count(categories.arithmetic),
        format_count(categories.logic),
        format_count(categories.data_movement)
    );
    eprintln!(
        "║ Control Flow: {:<10} SIMD: {:<13} FPU: {:<17} ║",
        format_count(categories.control_flow),
        format_count(categories.simd),
        format_count(categories.fpu)
    );
    eprintln!(
        "║ System: {:<16} String: {:<12} I/O: {:<17} ║",
        format_count(categories.system),
        format_count(categories.string_ops),
        format_count(categories.io)
    );
    eprintln!("╠════════════════════════════════════════════════════════════════════════════════╣");

    // Memory stats
    let (total_reads, total_writes, total_bytes_read, total_bytes_written) =
        compute_memory_totals(profiler);
    eprintln!("║                             MEMORY ACCESS                                      ║");
    eprintln!("╠════════════════════════════════════════════════════════════════════════════════╣");
    eprintln!(
        "║ Reads: {} ({})          Writes: {} ({})         ║",
        format_count(total_reads),
        format_bytes(total_bytes_read),
        format_count(total_writes),
        format_bytes(total_bytes_written)
    );
    eprintln!("╠════════════════════════════════════════════════════════════════════════════════╣");

    // Top 15 instructions
    let mut top: Vec<(usize, &super::stats::OpcodeStats)> = profiler
        .stats()
        .iter()
        .enumerate()
        .filter(|(_, s)| s.count > 0)
        .collect();
    top.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    eprintln!("║                        TOP 15 INSTRUCTIONS BY COUNT                            ║");
    eprintln!("╠════════════════════════════════════════════════════════════════════════════════╣");
    eprintln!(
        "║ {:<10} {:<25} {:>10} {:>8} {:>8} {:>8} ║",
        "Opcode", "Mnemonic", "Count", "Mean", "StdDev", "P99"
    );
    eprintln!(
        "║ {:─<10} {:─<25} {:─>10} {:─>8} {:─>8} {:─>8} ║",
        "", "", "", "", "", ""
    );

    for (idx, stats) in top.iter().take(15) {
        if let Some(key) = OpcodeKey::from_index(*idx) {
            let mnemonic = mnemonics::get_mnemonic(&key);
            let mnemonic_display = if mnemonic.len() > 23 {
                format!("{}...", &mnemonic[..20])
            } else {
                mnemonic.to_string()
            };
            eprintln!(
                "║ {:<10} {:<25} {:>10} {:>8.0} {:>8.1} {:>8} ║",
                key.format(),
                mnemonic_display,
                format_count_short(stats.count),
                stats.mean_nanos(),
                stats.stddev(),
                stats.p99()
            );
        }
    }

    eprintln!("╚════════════════════════════════════════════════════════════════════════════════╝");
}

/// JSON report structure.
#[derive(Serialize)]
pub struct ProfileReport {
    pub metadata: ReportMetadata,
    pub summary: SummaryStats,
    pub categories: CategoryBreakdown,
    pub instructions: Vec<InstructionReport>,
}

#[derive(Serialize)]
pub struct ReportMetadata {
    pub timestamp: String,
    pub duration_secs: f64,
    pub total_instructions: u64,
}

#[derive(Serialize)]
pub struct SummaryStats {
    pub instructions_per_second: f64,
    pub decode_cache_hit_rate: f64,
    pub decode_cache_hits: u64,
    pub decode_cache_misses: u64,
    pub memory_reads_total: u64,
    pub memory_writes_total: u64,
    pub bytes_read_total: u64,
    pub bytes_written_total: u64,
}

#[derive(Serialize, Default)]
pub struct CategoryBreakdown {
    pub arithmetic: u64,
    pub logic: u64,
    pub data_movement: u64,
    pub control_flow: u64,
    pub simd: u64,
    pub fpu: u64,
    pub system: u64,
    pub string_ops: u64,
    pub io: u64,
    pub other: u64,
}

/// Export profiling data to a JSON file.
pub fn export_json(path: &Path) -> std::io::Result<()> {
    let report = collector::with_profiler(|profiler| {
        let elapsed = profiler.elapsed();
        let total = collector::total_instructions();
        let (cache_hits, cache_misses) = collector::cache_stats();

        let cache_total = cache_hits + cache_misses;
        let cache_hit_rate = if cache_total > 0 {
            cache_hits as f64 / cache_total as f64
        } else {
            0.0
        };

        let ips = if elapsed.as_secs_f64() > 0.0 {
            total as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        let (total_reads, total_writes, total_bytes_read, total_bytes_written) =
            compute_memory_totals(profiler);

        let categories = compute_category_breakdown(profiler);

        // Collect all instruction stats
        let mut instructions: Vec<InstructionReport> = profiler
            .stats()
            .iter()
            .enumerate()
            .filter(|(_, s)| s.count > 0)
            .filter_map(|(idx, stats)| {
                OpcodeKey::from_index(idx).map(|key| {
                    let mnemonic = mnemonics::get_mnemonic(&key);
                    InstructionReport::from_stats(&key, stats, mnemonic)
                })
            })
            .collect();

        // Sort by count descending
        instructions.sort_by(|a, b| b.count.cmp(&a.count));

        ProfileReport {
            metadata: ReportMetadata {
                timestamp: chrono_timestamp(),
                duration_secs: elapsed.as_secs_f64(),
                total_instructions: total,
            },
            summary: SummaryStats {
                instructions_per_second: ips,
                decode_cache_hit_rate: cache_hit_rate,
                decode_cache_hits: cache_hits,
                decode_cache_misses: cache_misses,
                memory_reads_total: total_reads,
                memory_writes_total: total_writes,
                bytes_read_total: total_bytes_read,
                bytes_written_total: total_bytes_written,
            },
            categories,
            instructions,
        }
    });

    match report {
        Some(report) => {
            let json = serde_json::to_string_pretty(&report)?;
            std::fs::write(path, json)
        }
        None => Ok(()),
    }
}

// Helper functions

fn compute_memory_totals(profiler: &Profiler) -> (u64, u64, u64, u64) {
    let mut total_reads = 0u64;
    let mut total_writes = 0u64;
    let mut total_bytes_read = 0u64;
    let mut total_bytes_written = 0u64;

    for stats in profiler.stats().iter() {
        total_reads += stats.mem_reads;
        total_writes += stats.mem_writes;
        total_bytes_read += stats.bytes_read;
        total_bytes_written += stats.bytes_written;
    }

    (
        total_reads,
        total_writes,
        total_bytes_read,
        total_bytes_written,
    )
}

fn compute_category_breakdown(profiler: &Profiler) -> CategoryBreakdown {
    let mut categories = CategoryBreakdown::default();

    for (idx, stats) in profiler.stats().iter().enumerate() {
        if stats.count == 0 {
            continue;
        }

        if let Some(key) = OpcodeKey::from_index(idx) {
            match mnemonics::categorize(&key) {
                InstructionCategory::Arithmetic => categories.arithmetic += stats.count,
                InstructionCategory::Logic => categories.logic += stats.count,
                InstructionCategory::DataMovement => categories.data_movement += stats.count,
                InstructionCategory::ControlFlow => categories.control_flow += stats.count,
                InstructionCategory::Simd => categories.simd += stats.count,
                InstructionCategory::Fpu => categories.fpu += stats.count,
                InstructionCategory::System => categories.system += stats.count,
                InstructionCategory::StringOp => categories.string_ops += stats.count,
                InstructionCategory::Io => categories.io += stats.count,
                InstructionCategory::Other => categories.other += stats.count,
            }
        }
    }

    categories
}

fn format_count(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.2}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.2}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.2}K", n as f64 / 1_000.0)
    } else {
        format!("{}", n)
    }
}

fn format_count_short(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        format!("{}", n)
    }
}

fn format_bytes(n: u64) -> String {
    if n >= 1_099_511_627_776 {
        format!("{:.2}TB", n as f64 / 1_099_511_627_776.0)
    } else if n >= 1_073_741_824 {
        format!("{:.2}GB", n as f64 / 1_073_741_824.0)
    } else if n >= 1_048_576 {
        format!("{:.2}MB", n as f64 / 1_048_576.0)
    } else if n >= 1_024 {
        format!("{:.2}KB", n as f64 / 1_024.0)
    } else {
        format!("{}B", n)
    }
}

fn chrono_timestamp() -> String {
    // Simple ISO 8601 timestamp without external dependency
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    // Convert to a simple timestamp string
    format!("{}", secs)
}
