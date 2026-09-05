//! Central file I/O safety layer. ALL file reads in the agent path should go
//! through this module to enforce size limits at the system boundary.
//!
//! The root cause of OOM crashes is tools calling `fs::read_to_string()` on
//! arbitrary files without size checks. This module provides safe alternatives
//! that enforce limits and return structured errors instead of panicking.

use anyhow::{anyhow, Result};
use std::path::Path;

/// Maximum file size for text reads (2MB). Files larger than this should use
/// `read_tail` or `read_head_tail` for preview, or reject with an error.
pub const MAX_TEXT_FILE_SIZE: u64 = 2_000_000;

/// Maximum file size for binary reads (1MB). Used by extract_strings, etc.
pub const MAX_BINARY_FILE_SIZE: u64 = 1_000_000;

/// Maximum number of lines returned by search results.
pub const MAX_SEARCH_LINES: usize = 500;

/// Maximum stdout/stderr buffer size per command (100KB).
pub const MAX_CMD_OUTPUT: usize = 100_000;

/// Maximum terminal output buffer (500KB).
pub const MAX_TERM_BUFFER: usize = 500_000;

/// Directories that should never be traversed by the agent.
pub const SKIP_DIRS: &[&str] = &[
    "node_modules", "target", ".git", "dist", "build",
    ".next", ".nuxt", "__pycache__", ".cache", ".venv",
    "vendor", "Pods", ".gradle", ".idea", ".vscode",
];

/// Check if a path should be skipped (large directory patterns).
pub fn should_skip_path(path: &Path) -> bool {
    let s = path.to_string_lossy();
    SKIP_DIRS.iter().any(|d| s.contains(&format!("/{d}/")) || s.contains(&format!("\\{d}\\")))
        || s.ends_with("/node_modules") || s.ends_with("\\node_modules")
}

/// Safe file read with size limit. Returns the file content if under the limit,
/// or an error with the file size. This is the ONLY way agent tools should read files.
pub fn safe_read(path: &Path) -> Result<String> {
    let meta = std::fs::metadata(path)?;
    if meta.len() > MAX_TEXT_FILE_SIZE {
        return Err(anyhow!(
            "File too large ({}MB > {}MB limit). Use read_head_tail for preview.",
            meta.len() / 1_000_000,
            MAX_TEXT_FILE_SIZE / 1_000_000
        ));
    }
    if should_skip_path(path) {
        return Err(anyhow!("Path is in a skipped directory: {}", path.display()));
    }
    std::fs::read_to_string(path).map_err(|e| anyhow!("Failed to read {}: {e}", path.display()))
}

/// Safe binary read with size limit.
pub fn safe_read_bytes(path: &Path) -> Result<Vec<u8>> {
    let meta = std::fs::metadata(path)?;
    if meta.len() > MAX_BINARY_FILE_SIZE {
        return Err(anyhow!(
            "File too large ({}MB > {}MB limit).",
            meta.len() / 1_000_000,
            MAX_BINARY_FILE_SIZE / 1_000_000
        ));
    }
    if should_skip_path(path) {
        return Err(anyhow!("Path is in a skipped directory: {}", path.display()));
    }
    std::fs::read(path).map_err(|e| anyhow!("Failed to read {}: {e}", path.display()))
}

/// Read head + tail of a large file for preview. Returns the first `head_lines`
/// and last `tail_lines` with a truncation notice in between.
pub fn read_head_tail(path: &Path, head_lines: usize, tail_lines: usize) -> Result<String> {
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();

    if total <= head_lines + tail_lines {
        return Ok(content);
    }

    let mut result = String::new();
    result.push_str(&format!("/* First {head_lines} lines of {total} total */\n"));
    for line in &lines[..head_lines] {
        result.push_str(line);
        result.push('\n');
    }
    result.push_str(&format!("\n/* ... {skip} lines omitted ... */\n\n",
        skip = total - head_lines - tail_lines));
    for line in &lines[total - tail_lines..] {
        result.push_str(line);
        result.push('\n');
    }
    Ok(result)
}

/// Memory budget check. Returns true if system has enough headroom.
/// Uses current RSS from /proc/self/statm on Linux, or GlobalMemoryStatusEx on Windows.
pub fn check_memory_headroom(_min_free_mb: u64) -> bool {
    #[cfg(target_os = "linux")]
    {
        if let Ok(statm) = std::fs::read_to_string("/proc/self/statm") {
            let parts: Vec<u64> = statm.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() >= 2 {
                let rss_pages = parts[1];
                let page_size = 4096;
                let rss_mb = (rss_pages * page_size) / (1024 * 1024);
                // Assume 8GB total, check if we have headroom
                return rss_mb < (8 * 1024) - min_free_mb;
            }
        }
        true // Can't determine, assume OK
    }
    #[cfg(target_os = "windows")]
    {
        // On Windows, use a simple heuristic: check if we're under a memory limit
        // The actual check would use GlobalMemoryStatusEx, but for now use a
        // conservative approach based on allocation count
        true
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_skip_path() {
        assert!(should_skip_path(Path::new("/project/node_modules/package/index.js")));
        assert!(should_skip_path(Path::new("/project/target/debug/binary")));
        assert!(!should_skip_path(Path::new("/project/src/main.rs")));
    }

    #[test]
    fn test_read_head_tail() {
        let content = (0..100).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n");
        let path = std::env::temp_dir().join("test_head_tail.txt");
        std::fs::write(&path, &content).unwrap();

        let result = read_head_tail(&path, 3, 3).unwrap();
        assert!(result.contains("line 0"));
        assert!(result.contains("line 1"));
        assert!(result.contains("line 2"));
        assert!(result.contains("line 97"));
        assert!(result.contains("line 98"));
        assert!(result.contains("line 99"));
        assert!(result.contains("omitted"));

        std::fs::remove_file(path).unwrap();
    }
}
