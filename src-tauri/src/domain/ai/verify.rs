//! Programmatic verification loop — the IDE's structural advantage.
//!
//! The 2026 literature is blunt: an agent's self-check misses subtle errors, and
//! the fix is an *external* verifier grounded in something real (ProgCo runs the
//! program; SAFE grounds claims in evidence). A coding IDE already owns the best
//! external verifier there is — **the compiler and the test suite** — so a
//! generated patch can be *proven* before it's accepted, catching the failure
//! modes benchmarks flag: functionally-wrong-but-valid patches and uncompilable
//! code that reasoning hallucination alone produces.
//!
//! This module is the deterministic orchestration: run the configured checks
//! (build / test / lint) through a [`CheckRunner`], gate acceptance on the
//! result, and drive a **capped Reflexion loop** — on failure, hand the failing
//! output back to a fixer and re-verify, up to `max_retries`. The runner is a
//! trait so the policy is unit-tested with a mock; `CommandCheckRunner` is the
//! real one that shells out to the project's build/test/lint commands.
//!
//! Opt-in via `KORTEX_VERIFY`.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckKind {
    Build,
    Test,
    Lint,
}

impl CheckKind {
    pub fn label(self) -> &'static str {
        match self {
            CheckKind::Build => "build",
            CheckKind::Test => "test",
            CheckKind::Lint => "lint",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckResult {
    pub kind: CheckKind,
    pub passed: bool,
    /// One-line summary for the report/UI.
    pub summary: String,
    /// Trailing diagnostic output (bounded) fed back to the fixer on failure.
    pub details: String,
}

/// Runs one check and reports pass/fail. Implemented over the real toolchain;
/// mocked in tests.
pub trait CheckRunner {
    fn run(&self, kind: CheckKind) -> CheckResult;
}

#[derive(Debug, Clone)]
pub struct VerifyConfig {
    pub enabled: bool,
    pub run_build: bool,
    pub run_test: bool,
    pub run_lint: bool,
    /// Max Reflexion fix→re-verify cycles. 0 = verify once, never retry.
    pub max_retries: usize,
    /// Lint failures warn but don't block acceptance (build/test are hard gates).
    pub lint_is_advisory: bool,
}

impl Default for VerifyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            run_build: true,
            run_test: true,
            run_lint: false,
            max_retries: 2,
            lint_is_advisory: true,
        }
    }
}

impl VerifyConfig {
    /// Deliberately **opt-in**, unlike the rest of the reliability cluster: a
    /// `cargo build`/`cargo test` on a large workspace can legitimately run
    /// minutes, so defaulting this on risks the agent stalling on every
    /// `verify_implementation` call. `CommandCheckRunner` bounds each check
    /// with a timeout regardless — turn it on with `KORTEX_VERIFY=1` once
    /// you've set `KORTEX_VERIFY_TIMEOUT_SECS` to something that fits your
    /// build (default 120s, generous for an incremental build).
    pub fn from_env() -> Self {
        let mut cfg = Self {
            enabled: matches!(
                std::env::var("KORTEX_VERIFY").ok().as_deref(),
                Some("1") | Some("true") | Some("on")
            ),
            ..Self::default()
        };
        if let Ok(v) = std::env::var("KORTEX_VERIFY_RETRIES") {
            if let Ok(n) = v.trim().parse() {
                cfg.max_retries = n;
            }
        }
        cfg.run_lint = matches!(
            std::env::var("KORTEX_VERIFY_LINT").ok().as_deref(),
            Some("1") | Some("true") | Some("on")
        ) || cfg.run_lint;
        cfg
    }

    fn kinds(&self) -> Vec<CheckKind> {
        let mut v = Vec::new();
        if self.run_build {
            v.push(CheckKind::Build);
        }
        if self.run_test {
            v.push(CheckKind::Test);
        }
        if self.run_lint {
            v.push(CheckKind::Lint);
        }
        v
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifyReport {
    /// Verification passes performed (1 + number of retries taken).
    pub attempts: usize,
    /// The checks from the final attempt.
    pub checks: Vec<CheckResult>,
    /// True when every hard gate (build/test) passed on the final attempt.
    pub accepted: bool,
}

impl VerifyReport {
    /// Names of the hard-gate checks that failed on the final attempt.
    pub fn failing(&self, cfg: &VerifyConfig) -> Vec<&'static str> {
        self.checks
            .iter()
            .filter(|c| !c.passed && is_hard_gate(c.kind, cfg))
            .map(|c| c.kind.label())
            .collect()
    }
}

fn is_hard_gate(kind: CheckKind, cfg: &VerifyConfig) -> bool {
    !(kind == CheckKind::Lint && cfg.lint_is_advisory)
}

/// Run one pass of the configured checks.
pub fn run_checks(runner: &dyn CheckRunner, cfg: &VerifyConfig) -> Vec<CheckResult> {
    cfg.kinds().into_iter().map(|k| runner.run(k)).collect()
}

/// Do the checks accept (every hard gate passed)?
pub fn accepts(checks: &[CheckResult], cfg: &VerifyConfig) -> bool {
    checks
        .iter()
        .filter(|c| is_hard_gate(c.kind, cfg))
        .all(|c| c.passed)
}

/// Concatenate the failing checks' diagnostics for the fixer.
pub fn failure_feedback(checks: &[CheckResult], cfg: &VerifyConfig) -> String {
    let mut out = String::new();
    for c in checks {
        if !c.passed && is_hard_gate(c.kind, cfg) {
            out.push_str(&format!("[{} FAILED] {}\n{}\n\n", c.kind.label(), c.summary, c.details));
        }
    }
    out.trim_end().to_string()
}

/// The Reflexion loop: verify; if a hard gate fails and retries remain, hand the
/// diagnostics to `fix` (which regenerates + applies a patch and returns `true`
/// if it changed anything) and re-verify. Stops when accepted, out of retries,
/// or the fixer gives up.
pub fn run_verification_loop(
    runner: &dyn CheckRunner,
    cfg: &VerifyConfig,
    mut fix: impl FnMut(&str) -> bool,
) -> VerifyReport {
    let mut attempts = 0usize;
    let mut checks = run_checks(runner, cfg);
    attempts += 1;

    while !accepts(&checks, cfg) && attempts <= cfg.max_retries {
        let feedback = failure_feedback(&checks, cfg);
        if !fix(&feedback) {
            break; // fixer couldn't or wouldn't produce a change → stop
        }
        checks = run_checks(runner, cfg);
        attempts += 1;
    }

    let accepted = accepts(&checks, cfg);
    VerifyReport { attempts, checks, accepted }
}

// ── Real runner ──────────────────────────────────────────────────────────────

/// Runs the project's actual build/test/lint commands in `cwd`. A command that
/// isn't configured (`None`) is reported as a trivially-passing check so it
/// never blocks. Exit code 0 = pass. Output is tail-bounded so a huge log can't
/// blow the context when fed back to the fixer. Each command is **bounded by a
/// timeout** — a hung or runaway build/test process is killed and reported as
/// a failed (not stalled) check, so the agent loop can never wedge on this.
pub struct CommandCheckRunner {
    pub cwd: std::path::PathBuf,
    pub build: Option<Vec<String>>,
    pub test: Option<Vec<String>>,
    pub lint: Option<Vec<String>>,
    /// Max chars of tail output kept per check.
    pub tail: usize,
    /// Kill and report failure if a single check runs longer than this.
    pub timeout: std::time::Duration,
}

impl CommandCheckRunner {
    pub fn new(cwd: impl Into<std::path::PathBuf>) -> Self {
        let timeout_secs: u64 = std::env::var("KORTEX_VERIFY_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(120);
        Self {
            cwd: cwd.into(),
            build: None,
            test: None,
            lint: None,
            tail: 4000,
            timeout: std::time::Duration::from_secs(timeout_secs),
        }
    }

    fn cmd_for(&self, kind: CheckKind) -> Option<&Vec<String>> {
        match kind {
            CheckKind::Build => self.build.as_ref(),
            CheckKind::Test => self.test.as_ref(),
            CheckKind::Lint => self.lint.as_ref(),
        }
    }

    /// Spawn `argv`, drain stdout/stderr concurrently (so a chatty process
    /// can't deadlock on a full pipe buffer while we're not reading), and kill
    /// it if `timeout` elapses before it exits. Returns `(exit_code, output,
    /// timed_out)`; `exit_code` is `None` only on a spawn failure.
    fn run_bounded(&self, argv: &[String]) -> Result<(Option<i32>, String, bool), std::io::Error> {
        use std::io::Read;
        use std::process::Stdio;

        let (prog, args) = argv.split_first().expect("non-empty argv checked by caller");
        let mut child = std::process::Command::new(prog)
            .args(args)
            .current_dir(&self.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Drain both streams on background threads so the child never blocks
        // writing to a full pipe while we poll for exit.
        let mut stdout = child.stdout.take();
        let mut stderr = child.stderr.take();
        let out_handle = std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(s) = stdout.as_mut() {
                let _ = s.read_to_end(&mut buf);
            }
            buf
        });
        let err_handle = std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(s) = stderr.as_mut() {
                let _ = s.read_to_end(&mut buf);
            }
            buf
        });

        let deadline = std::time::Instant::now() + self.timeout;
        let mut timed_out = false;
        loop {
            match child.try_wait() {
                Ok(Some(_status)) => break,
                Ok(None) => {
                    if std::time::Instant::now() >= deadline {
                        timed_out = true;
                        let _ = child.kill();
                        let _ = child.wait(); // reap
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(_) => break, // can't observe status — stop waiting
            }
        }

        let status = child.try_wait().ok().flatten();
        let mut out_bytes = out_handle.join().unwrap_or_default();
        let err_bytes = err_handle.join().unwrap_or_default();
        out_bytes.extend_from_slice(&err_bytes);
        let text = String::from_utf8_lossy(&out_bytes).into_owned();

        Ok((status.and_then(|s| s.code()), text, timed_out))
    }
}

impl CheckRunner for CommandCheckRunner {
    fn run(&self, kind: CheckKind) -> CheckResult {
        let Some(argv) = self.cmd_for(kind) else {
            return CheckResult {
                kind,
                passed: true,
                summary: format!("no {} command configured — skipped", kind.label()),
                details: String::new(),
            };
        };
        if argv.is_empty() {
            return CheckResult {
                kind,
                passed: true,
                summary: "empty command — skipped".into(),
                details: String::new(),
            };
        }
        let output = self.run_bounded(argv);
        match output {
            Ok((code, text, timed_out)) => {
                let details = tail_chars(&text, self.tail);
                if timed_out {
                    return CheckResult {
                        kind,
                        passed: false,
                        summary: format!(
                            "{} exceeded {}s — killed",
                            argv.join(" "),
                            self.timeout.as_secs()
                        ),
                        details,
                    };
                }
                let passed = code == Some(0);
                CheckResult {
                    kind,
                    passed,
                    summary: format!(
                        "{} {} (exit {})",
                        argv.join(" "),
                        if passed { "passed" } else { "FAILED" },
                        code.unwrap_or(-1)
                    ),
                    details,
                }
            }
            Err(e) => CheckResult {
                kind,
                passed: false,
                summary: format!("could not run `{}`: {e}", argv.join(" ")),
                details: String::new(),
            },
        }
    }
}

/// Auto-detect the project's build/test commands from files present at `root`
/// (Cargo.toml → `cargo build`/`cargo test`; package.json → `npm run
/// build`/`npm test` when those scripts exist; pyproject.toml/setup.py →
/// pytest). Returns a runner with whichever commands it could infer; anything
/// it can't infer stays `None` (skipped, never a false failure).
pub fn detect_runner(root: &std::path::Path) -> CommandCheckRunner {
    let mut runner = CommandCheckRunner::new(root);

    if root.join("Cargo.toml").is_file() {
        runner.build = Some(vec!["cargo".into(), "build".into(), "--quiet".into()]);
        runner.test = Some(vec!["cargo".into(), "test".into(), "--quiet".into()]);
        return runner;
    }
    if root.join("package.json").is_file() {
        if let Ok(text) = std::fs::read_to_string(root.join("package.json")) {
            if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&text) {
                let scripts = pkg.get("scripts").cloned().unwrap_or_default();
                let has = |k: &str| scripts.get(k).and_then(|v| v.as_str()).is_some();
                if has("build") {
                    runner.build = Some(vec!["npm".into(), "run".into(), "build".into()]);
                }
                if has("test") {
                    runner.test = Some(vec!["npm".into(), "test".into()]);
                }
                if has("lint") {
                    runner.lint = Some(vec!["npm".into(), "run".into(), "lint".into()]);
                }
            }
        }
        return runner;
    }
    if root.join("pyproject.toml").is_file() || root.join("setup.py").is_file() {
        runner.test = Some(vec!["python".into(), "-m".into(), "pytest".into(), "-q".into()]);
        return runner;
    }
    runner // nothing detected — every check is a no-op skip
}

fn tail_chars(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count <= n {
        return s.to_string();
    }
    let skip = count - n;
    format!("…{}", s.chars().skip(skip).collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// Mock runner scripted with a sequence of pass/fail per kind.
    struct Mock {
        build: RefCell<Vec<bool>>,
        test: RefCell<Vec<bool>>,
        lint: RefCell<Vec<bool>>,
    }
    impl Mock {
        fn new(build: Vec<bool>, test: Vec<bool>, lint: Vec<bool>) -> Self {
            Self {
                build: RefCell::new(build),
                test: RefCell::new(test),
                lint: RefCell::new(lint),
            }
        }
        fn pop(v: &RefCell<Vec<bool>>) -> bool {
            let mut b = v.borrow_mut();
            if b.is_empty() { true } else { b.remove(0) }
        }
    }
    impl CheckRunner for Mock {
        fn run(&self, kind: CheckKind) -> CheckResult {
            let passed = match kind {
                CheckKind::Build => Mock::pop(&self.build),
                CheckKind::Test => Mock::pop(&self.test),
                CheckKind::Lint => Mock::pop(&self.lint),
            };
            CheckResult {
                kind,
                passed,
                summary: format!("{} {}", kind.label(), if passed { "ok" } else { "fail" }),
                details: if passed { String::new() } else { format!("error in {}", kind.label()) },
            }
        }
    }

    fn cfg() -> VerifyConfig {
        VerifyConfig { enabled: true, run_lint: true, ..Default::default() }
    }

    #[test]
    fn accepts_when_all_pass() {
        let m = Mock::new(vec![true], vec![true], vec![true]);
        let checks = run_checks(&m, &cfg());
        assert!(accepts(&checks, &cfg()));
    }

    #[test]
    fn rejects_on_build_failure() {
        let m = Mock::new(vec![false], vec![true], vec![true]);
        let checks = run_checks(&m, &cfg());
        assert!(!accepts(&checks, &cfg()));
        assert_eq!(failure_feedback(&checks, &cfg()).contains("build FAILED"), true);
    }

    #[test]
    fn lint_advisory_does_not_block() {
        // build+test pass, lint fails, but lint is advisory → accepted
        let m = Mock::new(vec![true], vec![true], vec![false]);
        let checks = run_checks(&m, &cfg());
        assert!(accepts(&checks, &cfg()), "advisory lint must not block");
    }

    #[test]
    fn reflexion_fixes_then_accepts() {
        // build fails first, passes after the fix; test always passes.
        let m = Mock::new(vec![false, true], vec![true, true], vec![true, true]);
        let mut fix_calls = 0;
        let report = run_verification_loop(&m, &cfg(), |feedback| {
            assert!(feedback.contains("build FAILED"));
            fix_calls += 1;
            true // "applied a fix"
        });
        assert!(report.accepted);
        assert_eq!(report.attempts, 2);
        assert_eq!(fix_calls, 1);
    }

    #[test]
    fn reflexion_gives_up_after_max_retries() {
        // build always fails; max_retries=2 → 3 attempts total then stop.
        let m = Mock::new(vec![false, false, false, false], vec![true; 4], vec![true; 4]);
        let mut fix_calls = 0;
        let report = run_verification_loop(&m, &cfg(), |_| {
            fix_calls += 1;
            true
        });
        assert!(!report.accepted);
        assert_eq!(report.attempts, 3); // 1 initial + 2 retries
        assert_eq!(fix_calls, 2);
        assert_eq!(report.failing(&cfg()), vec!["build"]);
    }

    #[test]
    fn stops_when_fixer_gives_up() {
        let m = Mock::new(vec![false, false], vec![true; 2], vec![true; 2]);
        let report = run_verification_loop(&m, &cfg(), |_| false); // fixer can't fix
        assert!(!report.accepted);
        assert_eq!(report.attempts, 1, "no re-verify after the fixer gives up");
    }

    #[test]
    fn command_runner_skips_unconfigured() {
        let r = CommandCheckRunner::new(".");
        let res = r.run(CheckKind::Build);
        assert!(res.passed, "no build command → skipped, not failed");
        assert!(res.summary.contains("skipped"));
    }

    #[test]
    #[cfg(windows)]
    fn command_runner_captures_real_pass_and_fail() {
        let mut r = CommandCheckRunner::new(".");
        r.build = Some(vec!["cmd".into(), "/C".into(), "exit 0".into()]);
        let ok = r.run(CheckKind::Build);
        assert!(ok.passed);

        r.build = Some(vec!["cmd".into(), "/C".into(), "exit 1".into()]);
        let fail = r.run(CheckKind::Build);
        assert!(!fail.passed);
        assert!(fail.summary.contains("FAILED"));
    }

    #[test]
    #[cfg(windows)]
    fn command_runner_kills_on_timeout() {
        let mut r = CommandCheckRunner::new(".");
        r.timeout = std::time::Duration::from_millis(300);
        // ~2s of pings — comfortably longer than the 300ms budget above.
        r.build = Some(vec!["cmd".into(), "/C".into(), "ping -n 3 127.0.0.1 >NUL".into()]);
        let res = r.run(CheckKind::Build);
        assert!(!res.passed, "a hung command must be reported as failed, not block forever");
        assert!(res.summary.contains("exceeded") || res.summary.contains("killed"));
    }
}
