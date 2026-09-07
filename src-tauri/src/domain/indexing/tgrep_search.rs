//! `tgrep` (microsoft/tgrep) backend — trigram-indexed regex search, run
//! **side by side** with ripgrep as the preferred-when-available first engine.
//!
//! tgrep's own numbers: up to ~52x rg on large repos, because it searches a
//! pre-built trigram index instead of scanning every file per query. Two modes:
//!
//!   * **indexed** — `tgrep pattern .` auto-connects to an already-running
//!     `tgrep serve .` daemon (started by the user, outside this IDE's
//!     control) and returns near-instantly from the index.
//!   * **one-shot** — `tgrep pattern . --no-index` brute-force scans, no
//!     server required at all; this is the guaranteed-safe fallback within
//!     tgrep itself.
//!
//! This module never manages the server's lifecycle (spawning/health-checking
//! a long-lived daemon is a real feature but a separate, riskier one — not
//! done here). Instead: try the indexed form first, **bounded by a short
//! timeout** (a "no server" connection attempt should fail near-instantly, so
//! the timeout is a safety net, not the expected path), and if that doesn't
//! come back clean, retry with `--no-index`. Either attempt failing for any
//! reason — tgrep absent, spawn error, non-zero-but-not-"no matches" exit,
//! or a JSON-shape mismatch — returns `None` so the caller falls through to
//! ripgrep. tgrep being installed is the rare case; every path here is written
//! so its absence, or any hiccup, is silently invisible to the caller.

use std::io::Read;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};

use super::ripgrep_search::{parse_rg_json_stream, RipgrepQuery, SearchResult};

/// How long to wait for a "connect to a running server" attempt before giving
/// up and falling back to `--no-index`. Generous for a local TCP handshake,
/// tight enough that the fallback path never feels slow when no server exists.
const SERVER_PROBE_TIMEOUT: Duration = Duration::from_millis(800);

/// Try tgrep for this query. `None` on anything but a clean result — the
/// caller (`ripgrep_search`) then tries rg unchanged.
pub fn try_tgrep(q: &RipgrepQuery<'_>, max: usize) -> Option<Vec<SearchResult>> {
    // Fixed-string semantics aren't confirmed for tgrep's flag set — skip
    // rather than risk mis-searching (`fixed_string` mismatches are a
    // correctness bug, not a missed optimization).
    if q.fixed_string {
        return None;
    }
    let tgrep = crate::ide_shell::resolve_tgrep_exe()?;

    if let Some(hits) = run_tgrep(&tgrep, q, max, false) {
        return Some(hits);
    }
    run_tgrep(&tgrep, q, max, true)
}

fn build_args(q: &RipgrepQuery<'_>, max: usize, no_index: bool) -> (Vec<String>, std::path::PathBuf) {
    let mut args = vec!["--json".to_string()];
    if q.case_insensitive {
        args.push("-i".to_string());
    }
    if no_index {
        args.push("--no-index".to_string());
    }
    args.push(q.pattern.to_string());

    let search_path = if let Some(f) = q.file {
        if f.is_absolute() { f.to_path_buf() } else { q.root.join(f) }
    } else {
        q.root.to_path_buf()
    };
    args.push(search_path.to_string_lossy().to_string());
    let _ = max; // tgrep's max-results flag name isn't confirmed; capped client-side in parse_rg_json_stream
    (args, q.root.to_path_buf())
}

/// One bounded attempt. `use_no_index=false` is the "maybe there's already a
/// server" probe (short timeout, since a closed-port connection fails fast);
/// `true` is the guaranteed one-shot path (no timeout beyond a normal process
/// run — brute-force scanning a big repo can legitimately take real time,
/// same as rg itself, which also runs unbounded).
fn run_tgrep(tgrep: &Path, q: &RipgrepQuery<'_>, max: usize, use_no_index: bool) -> Option<Vec<SearchResult>> {
    let (args, cwd) = build_args(q, max, use_no_index);
    let timeout = if use_no_index { None } else { Some(SERVER_PROBE_TIMEOUT) };
    let (code, stdout) = spawn_bounded(tgrep, &args, &cwd, timeout)?;

    // 0 = matches, 1 = no matches (both are a valid, trustworthy result); 2 =
    // error (fall through). See tgrep's documented exit codes.
    match code {
        0 => {
            let hits = parse_rg_json_stream(&stdout, max);
            if hits.is_empty() {
                // Exit 0 promised matches but nothing parsed — the "rg-
                // compatible" JSON claim didn't hold for this build/version.
                // Don't silently return an empty result for a real match;
                // let the caller fall back to rg instead.
                None
            } else {
                Some(hits)
            }
        }
        1 => Some(Vec::new()), // confirmed no matches
        _ => None,             // error, or the probe attempt timed out/failed
    }
}

/// Spawn `argv`, draining stdout/stderr concurrently so the child can never
/// block on a full pipe, bounded by `timeout` when given. Returns `(exit_code,
/// stdout)` on a clean exit within budget; `None` on spawn failure, a kill
/// signal, or an exceeded timeout.
fn spawn_bounded(
    prog: &Path,
    args: &[String],
    cwd: &Path,
    timeout: Option<Duration>,
) -> Option<(i32, String)> {
    let mut child = std::process::Command::new(prog)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    let mut stdout_pipe = child.stdout.take();
    let mut stderr_pipe = child.stderr.take();
    let out_handle = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(s) = stdout_pipe.as_mut() {
            let _ = s.read_to_end(&mut buf);
        }
        buf
    });
    let err_handle = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(s) = stderr_pipe.as_mut() {
            let _ = s.read_to_end(&mut buf);
        }
        buf
    });

    let deadline = timeout.map(|t| Instant::now() + t);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let out = out_handle.join().unwrap_or_default();
                let _ = err_handle.join(); // drained, not surfaced (rg's path ignores stderr too)
                return Some((status.code().unwrap_or(-1), String::from_utf8_lossy(&out).into_owned()));
            }
            Ok(None) => {
                if let Some(dl) = deadline {
                    if Instant::now() >= dl {
                        let _ = child.kill();
                        let _ = child.wait();
                        return None; // timed out — caller retries with --no-index
                    }
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(_) => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn q<'a>(pattern: &'a str, root: &'a Path) -> RipgrepQuery<'a> {
        RipgrepQuery {
            pattern,
            root,
            include: None,
            max_results: 100,
            case_insensitive: false,
            fixed_string: false,
            file: None,
        }
    }

    #[test]
    fn fixed_string_queries_are_skipped_not_attempted() {
        // No tgrep binary needs to exist for this — fixed_string short-
        // circuits before resolve_tgrep_exe is even consulted.
        let root = PathBuf::from(".");
        let query = RipgrepQuery { fixed_string: true, ..q("literal[text]", &root) };
        assert!(try_tgrep(&query, 50).is_none());
    }

    #[test]
    fn build_args_includes_json_and_pattern() {
        let root = PathBuf::from("/workspace");
        let (args, cwd) = build_args(&q("TODO", &root), 50, false);
        assert!(args.contains(&"--json".to_string()));
        assert!(args.contains(&"TODO".to_string()));
        assert!(!args.contains(&"--no-index".to_string()));
        assert_eq!(cwd, root);
    }

    #[test]
    fn build_args_no_index_mode() {
        let root = PathBuf::from("/workspace");
        let (args, _) = build_args(&q("TODO", &root), 50, true);
        assert!(args.contains(&"--no-index".to_string()));
    }

    #[test]
    fn build_args_case_insensitive_flag() {
        let root = PathBuf::from("/workspace");
        let query = RipgrepQuery { case_insensitive: true, ..q("todo", &root) };
        let (args, _) = build_args(&query, 50, false);
        assert!(args.contains(&"-i".to_string()));
    }

    #[test]
    fn build_args_resolves_a_file_target_relative_to_root() {
        let root = PathBuf::from("/workspace");
        let file = PathBuf::from("src/main.rs");
        let query = RipgrepQuery { file: Some(file.as_path()), ..q("fn ", &root) };
        let (args, _) = build_args(&query, 50, false);
        let last = args.last().unwrap();
        assert!(last.ends_with("src/main.rs") || last.ends_with("src\\main.rs"));
    }

    #[test]
    fn spawn_bounded_returns_none_for_a_nonexistent_binary() {
        let bogus = PathBuf::from("this-binary-does-not-exist-anywhere-12345");
        let cwd = PathBuf::from(".");
        assert!(spawn_bounded(&bogus, &["x".into()], &cwd, None).is_none());
    }

    #[test]
    #[cfg(windows)]
    fn spawn_bounded_captures_real_exit_code_and_stdout() {
        let cmd = PathBuf::from("cmd");
        let cwd = PathBuf::from(".");
        let (code, out) = spawn_bounded(&cmd, &["/C".into(), "echo hello".into()], &cwd, None).unwrap();
        assert_eq!(code, 0);
        assert!(out.contains("hello"));
    }

    #[test]
    #[cfg(windows)]
    fn spawn_bounded_times_out_a_long_running_process() {
        let cmd = PathBuf::from("cmd");
        let cwd = PathBuf::from(".");
        let res = spawn_bounded(
            &cmd,
            &["/C".into(), "ping -n 3 127.0.0.1 >NUL".into()],
            &cwd,
            Some(Duration::from_millis(200)),
        );
        assert!(res.is_none(), "must not block past the timeout");
    }

    #[test]
    fn run_tgrep_treats_exit_0_with_no_parsed_hits_as_a_schema_mismatch() {
        // Exercised indirectly via parse_rg_json_stream: garbage that isn't
        // the expected JSON shape parses to zero hits, which run_tgrep's exit
        // code 0 branch must NOT trust as "no matches" (that's what exit 1
        // means) — verified at the parser level here since spawning tgrep
        // itself isn't available in this environment.
        let hits = parse_rg_json_stream("not json at all\n{\"type\":\"begin\"}\n", 50);
        assert!(hits.is_empty(), "non-match lines produce zero hits, as expected");
    }

    /// End-to-end against a REAL vendored tgrep binary — confirms the JSON
    /// schema assumption (tgrep really does emit rg-compatible `--json`) and
    /// exit-code semantics (0/1) against actual tgrep output, not a mock.
    /// `#[ignore]`d because it needs `scripts/fetch-tgrep.ts` to have run
    /// first (the binary isn't committed) — CI and fresh clones skip it;
    /// `cargo test -- --ignored tgrep_real` runs it once vendored.
    #[test]
    #[ignore]
    fn real_tgrep_binary_end_to_end() {
        let Some(tgrep) = crate::ide_shell::resolve_tgrep_exe() else {
            panic!("run scripts/fetch-tgrep.ts first, or set HADES_TGREP_PATH");
        };
        // A known-real symbol in this workspace: AbstainConfig in abstain.rs.
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/domain/ai");
        let query = q("pub struct AbstainConfig", &root);
        let hits = run_tgrep(&tgrep, &query, 50, true).expect("real tgrep search should find a real hit");
        assert!(hits.iter().any(|h| h.path.ends_with("abstain.rs") && h.line > 0));

        // A pattern that genuinely doesn't exist must come back as a clean
        // empty result (exit 1), not a fallback-triggering None.
        let miss = q("ThisPatternDoesNotExistAnywhere12345", &root);
        let empty = run_tgrep(&tgrep, &miss, 50, true).expect("a real no-match is Some(empty), not None");
        assert!(empty.is_empty());
    }
}
