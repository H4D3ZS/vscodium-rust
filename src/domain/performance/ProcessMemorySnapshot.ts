/**
 * Domain entity: live memory picture of the IDE process tree.
 *
 * WHY separate host vs children?
 * Tauri runs the UI in WebView2 child processes. Showing only the Rust host
 * (~300 MB) is misleading; Task Manager totals the whole tree (~800 MB–1 GB).
 */

export interface ProcessMemoryLineItem {
    name: string;
    pid: number;
    working_set_mb: number;
    private_mb: number;
}

export interface ProcessMemorySnapshot {
    host_working_set_mb: number;
    child_working_set_mb: number;
    total_working_set_mb: number;
    total_private_mb: number;
    child_process_count: number;
    cpu_usage_percent: number;
    system_total_ram_gb: number;
    system_available_ram_gb: number;
    breakdown: ProcessMemoryLineItem[];
}

/** Shape returned by `get_process_stats` Tauri command (includes legacy fields). */
export interface ProcessStatsDto {
    memory_mb: number;
    cpu_usage: number;
    total_ram_gb: number;
    available_ram_gb: number;
    snapshot: ProcessMemorySnapshot;
}

export function formatMemoryTooltip(s: ProcessMemorySnapshot): string {
    const lines = [
        `Total working set: ${s.total_working_set_mb.toFixed(0)} MB (host ${s.host_working_set_mb.toFixed(0)} + children ${s.child_working_set_mb.toFixed(0)})`,
        `Private bytes sum: ${s.total_private_mb.toFixed(0)} MB`,
        `Child processes: ${s.child_process_count}`,
        `CPU: ${s.cpu_usage_percent.toFixed(0)}%`,
        `System free: ${s.system_available_ram_gb} GB`,
    ];
    if (s.breakdown.length > 0) {
        lines.push('', 'Breakdown (top processes):');
        for (const row of s.breakdown.slice(0, 6)) {
            lines.push(`  ${row.name} (${row.pid}): ${row.working_set_mb.toFixed(0)} MB`);
        }
    }
    return lines.join('\n');
}
