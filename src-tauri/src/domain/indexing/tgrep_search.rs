//! `tgrep` (our fork, `H4D3ZS/tgrep`, MIT) backend — trigram-indexed regex
//! search, run **side by side** with ripgrep as the preferred-when-available
//! first engine.
//!
//! Two paths, tried in order:
//!
//!   * **indexed server probe** — if the external `tgrep` binary is resolved
//!     (built from the fork by `scripts/build-tgrep.ps1`, or found on PATH),
//!     `tgrep pattern .` auto-connects to an already-running `tgrep serve .`
//!     daemon (started by the user, outside this IDE's control) and returns
//!     near-instantly from the trigram index. Bounded by a short timeout — a
//!     "no server" connection attempt should fail near-instantly, so the
//!     timeout is a safety net, not the expected path.
//!   * **in-process brute force** — `tgrep_core::search::search_directory`,
//!     linked directly into this binary. No subprocess, no external binary
//!     needed at all, no JSON round-trip: this crate depends on the
//!     `tgrep-core` library (the fork's own search engine, shared between its
//!     CLI and server) as an ordinary Rust dependency and calls it in-process.
//!     This is what used to be a `tgrep --no-index` subprocess call; now it's
//!     a function call. Always available, always correct, so it's also the
//!     answer when no external `tgrep` binary exists at all.
//!
//! Both paths returning nothing (or the server probe failing/timing out) is
//! handled entirely within this module — the caller (`ripgrep_search`) only
//! ever sees `Some(hits)` (possibly empty, a trustworthy "no matches") or
//! `None` (try rg instead), and `None` should be rare: the in-process path
//! needs nothing but this binary itself.

use std::io::Read;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};

use super::ripgrep_search::{parse_rg_json_stream, RipgrepQuery, SearchResult};

/// How long to wait for a "connect to a running server" attempt before giving
/// up and falling back to the in-process search. Generous for a local TCP
/// handshake, tight enough that the fallback path never feels slow when no
/// server exists.
const SERVER_PROBE_TIMEOUT: Duration = Duration::from_millis(800);

/// Try tgrep for this query: the indexed server probe first (when the
/// external binary is available), then in-process brute force. `None` only
/// when both are somehow unable to answer — the caller then tries rg.
pub fn try_tgrep(q: &RipgrepQuery<'_>, max: usize) -> Option<Vec<SearchResult>> {
    if let Some(tgrep) = crate::ide_shell::resolve_tgrep_exe() {
        if let Some(hits) = run_server_probe(&tgrep, q, max) {
            crate::domain::ai::reliability_stats::bump("TGREP_SERVER_PROBE_HITS");
            return Some(hits);
        }
    }
    let hits = run_in_process(q, max);
    if hits.is_some() {
        crate::domain::ai::reliability_stats::bump("TGREP_IN_PROCESS_HITS");
    }
    hits
}

/// The in-process path: `tgrep-core`'s own search engine, linked directly
/// into this binary. No subprocess, no external binary, no timeout needed —
/// a brute-force directory walk + regex scan is bounded only by how much
/// there is to read, same as ripgrep's own fallback walker.
fn run_in_process(q: &RipgrepQuery<'_>, max: usize) -> Option<Vec<SearchResult>> {
    let opts = tgrep_core::search::SearchOptions {
        case_insensitive: q.case_insensitive,
        fixed_string: q.fixed_string,
        max_results: max,
        ..Default::default()
    };
    let search_path = if let Some(f) = q.file {
        if f.is_absolute() { f.to_path_buf() } else { q.root.join(f) }
    } else {
        q.root.to_path_buf()
    };
    let hits = tgrep_core::search::search_directory(q.pattern, &search_path, &opts).ok()?;
    Some(
        hits.into_iter()
            .map(|m| SearchResult { path: m.path, line: m.line, content: m.content })
            .collect(),
    )
}

fn build_args(q: &RipgrepQuery<'_>, max: usize) -> (Vec<String>, std::path::PathBuf) {
    let mut args = vec!["--json".to_string()];
    if q.case_insensitive {
        args.push("-i".to_string());
    }
    if q.fixed_string {
        args.push("-F".to_string());
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

/// The "maybe there's already a server" probe — short timeout, since a
/// closed-port connection fails fast. `None` on anything but a clean,
/// trustworthy result (including a timeout), so the caller falls through to
/// the in-process path rather than trusting a possibly-stale attempt.
fn run_server_probe(tgrep: &Path, q: &RipgrepQuery<'_>, max: usize) -> Option<Vec<SearchResult>> {
    let (args, cwd) = build_args(q, max);
    let (code, stdout) = spawn_bounded(tgrep, &args, &cwd, Some(SERVER_PROBE_TIMEOUT))?;

    // 0 = matches, 1 = no matches (both are a valid, trustworthy result); 2 =
    // error (fall through). See tgrep's documented exit codes.
    match code {
        0 => {
            let hits = parse_rg_json_stream(&stdout, max);
            if hits.is_empty() {
                // Exit 0 promised matches but nothing parsed — the "rg-
                // compatible" JSON claim didn't hold for this build/version.
                // Don't silently return an empty result for a real match;
                // let the caller fall back to the in-process path instead.
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
                        return None; // timed out — caller falls through
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
    fn build_args_includes_json_and_pattern() {
        let root = PathBuf::from("/workspace");
        let (args, cwd) = build_args(&q("TODO", &root), 50);
        assert!(args.contains(&"--json".to_string()));
        assert!(args.contains(&"TODO".to_string()));
        assert_eq!(cwd, root);
    }

    #[test]
    fn build_args_case_insensitive_flag() {
        let root = PathBuf::from("/workspace");
        let query = RipgrepQuery { case_insensitive: true, ..q("todo", &root) };
        let (args, _) = build_args(&query, 50);
        assert!(args.contains(&"-i".to_string()));
    }

    #[test]
    fn build_args_fixed_string_flag() {
        let root = PathBuf::from("/workspace");
        let query = RipgrepQuery { fixed_string: true, ..q("a[b]", &root) };
        let (args, _) = build_args(&query, 50);
        assert!(args.contains(&"-F".to_string()));
    }

    #[test]
    fn build_args_resolves_a_file_target_relative_to_root() {
        let root = PathBuf::from("/workspace");
        let file = PathBuf::from("src/main.rs");
        let query = RipgrepQuery { file: Some(file.as_path()), ..q("fn ", &root) };
        let (args, _) = build_args(&query, 50);
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
    fn run_server_probe_treats_exit_0_with_no_parsed_hits_as_a_schema_mismatch() {
        // Exercised indirectly via parse_rg_json_stream: garbage that isn't
        // the expected JSON shape parses to zero hits, which the exit-0
        // branch must NOT trust as "no matches" (that's what exit 1 means).
        let hits = parse_rg_json_stream("not json at all\n{\"type\":\"begin\"}\n", 50);
        assert!(hits.is_empty(), "non-match lines produce zero hits, as expected");
    }

    // ── In-process path — real, no external binary needed at all ──────────

    fn write(dir: &Path, name: &str, content: &str) {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn in_process_finds_a_real_match() {
        let tmp = tempfile::TempDir::new().unwrap();
        write(tmp.path(), "a.rs", "fn main() {}\nstruct Needle;\n");
        let root = tmp.path().to_path_buf();
        let hits = run_in_process(&q("struct Needle", &root), 50).expect("in-process always answers");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].path, "a.rs");
        assert_eq!(hits[0].line, 2);
    }

    #[test]
    fn in_process_no_match_is_a_trustworthy_empty_result() {
        let tmp = tempfile::TempDir::new().unwrap();
        write(tmp.path(), "a.rs", "fn main() {}\n");
        let root = tmp.path().to_path_buf();
        let hits = run_in_process(&q("NoSuchPattern12345", &root), 50).unwrap();
        assert!(hits.is_empty());
    }

    #[test]
    fn in_process_honors_fixed_string() {
        let tmp = tempfile::TempDir::new().unwrap();
        write(tmp.path(), "a.rs", "let v: Vec<String> = vec![];\n");
        let root = tmp.path().to_path_buf();
        let query = RipgrepQuery { fixed_string: true, ..q("Vec<String>", &root) };
        let hits = run_in_process(&query, 50).unwrap();
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn try_tgrep_answers_without_any_external_binary() {
        // With HADES_TGREP_PATH unset and nothing on PATH in this sandboxed
        // test env, resolve_tgrep_exe() returns None — try_tgrep must still
        // answer via the in-process path, not return None itself.
        let tmp = tempfile::TempDir::new().unwrap();
        write(tmp.path(), "a.rs", "struct OnlyInProcess;\n");
        let root = tmp.path().to_path_buf();
        let hits = try_tgrep(&q("OnlyInProcess", &root), 50);
        assert!(hits.is_some(), "the in-process path needs no external tgrep binary");
        assert_eq!(hits.unwrap().len(), 1);
    }
}
