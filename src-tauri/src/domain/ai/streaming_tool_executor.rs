//! Parallel execution of read-only agent tools during a single model turn.

use futures::future::join_all;
use serde_json::Value;
use std::collections::HashMap;

use crate::tool_invoker::ToolInvoker;

const PARALLEL_SAFE: &[&str] = &[
    "view_file",
    "read_file",
    "read_file_lines",
    "grep",
    "ripgrep_raw_search",
    "semantic_search",
    "search_codebase",
    "codebase_search",
    "list_files",
    "find_by_name",
    "find_symbol",
    "search_files",
    "list_dir",
    "get_diagnostics",
    "search_skills",
    "use_skill",
    "browser_read_dom",
    "browser_get_content_summary",
];

pub fn is_parallel_safe(tool_name: &str) -> bool {
    let canonical = crate::tool_aliases::canonical_tool_name(tool_name);
    PARALLEL_SAFE.contains(&canonical)
}

/// Prefetch results for read-only tools when the model issued several at once.
pub async fn prefetch_parallel_tools(
    invoker: &ToolInvoker,
    tool_calls: &[(String, String)],
) -> HashMap<usize, Result<Value, String>> {
    if tool_calls.len() < 2 {
        return HashMap::new();
    }
    let all_safe = tool_calls.iter().all(|(n, _)| is_parallel_safe(n));
    if !all_safe {
        return HashMap::new();
    }
    let batch: Vec<(usize, String, String)> = tool_calls
        .iter()
        .enumerate()
        .map(|(i, (n, a))| (i, n.clone(), a.clone()))
        .collect();
    execute_parallel_batch(invoker, &batch)
        .await
        .into_iter()
        .collect()
}

pub async fn execute_parallel_batch(
    invoker: &ToolInvoker,
    calls: &[(usize, String, String)],
) -> Vec<(usize, Result<Value, String>)> {
    let futures: Vec<_> = calls
        .iter()
        .map(|(idx, name, args)| {
            let inv = invoker;
            let n = name.clone();
            let a = args.clone();
            let i = *idx;
            async move {
                let result = inv.execute_tool(&n, &a).await.map_err(|e| e.to_string());
                (i, result)
            }
        })
        .collect();
    join_all(futures).await
}
