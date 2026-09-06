//! Asynchronous telemetry ingestion pipeline.
//!
//! Spawns local static-analysis binaries, streams their output into the `.aim`
//! disk memory-map to stay under the RAM budget, then forwards a fixed-size
//! context window to the local inference layer.
//!
//! Concurrency ceiling = 1 (global Semaphore): deep-dive tasks never overlap,
//! preventing memory exhaustion on 4–8 GB machines.

use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use memmap2::MmapMut;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::Semaphore,
};

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FindingTier {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl FindingTier {
    pub fn priority(&self) -> u8 {
        match self {
            Self::Critical => 0,
            Self::High => 1,
            Self::Medium => 2,
            Self::Low => 3,
            Self::Info => 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    /// Monotonic nanoseconds since UNIX epoch.
    pub timestamp_ns: u64,
    pub source: String,
    pub tier: FindingTier,
    pub category: String,
    pub title: String,
    pub description: String,
    /// Raw analyzer output (truncated to 4096 bytes).
    pub raw: Option<String>,
}

impl TelemetryEvent {
    pub fn new(
        source: impl Into<String>,
        tier: FindingTier,
        category: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let timestamp_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_nanos() as u64;
        Self {
            timestamp_ns,
            source: source.into(),
            tier,
            category: category.into(),
            title: title.into(),
            description: description.into(),
            raw: None,
        }
    }

    pub fn with_raw(mut self, raw: impl Into<String>) -> Self {
        let s: String = raw.into();
        // Hard cap: 4 096 bytes — avoid blowing the aim segment budget.
        self.raw = Some(if s.len() > 4096 { s[..4096].to_string() } else { s });
        self
    }
}

// ── Analyzer descriptor ───────────────────────────────────────────────────────

/// Static configuration for a local analyzer binary.
#[derive(Debug, Clone)]
pub struct AnalyzerSpec {
    /// Binary name / absolute path.
    pub exe: &'static str,
    /// Arguments template. `{target}` is replaced with the workspace root.
    pub args: &'static [&'static str],
    pub category: &'static str,
    pub tier: FindingTier,
    /// Maximum wall-clock time before the subprocess is killed.
    pub timeout: Duration,
}

/// Built-in analyzer registry — passive, read-only tools only.
/// No network calls, no writes to the workspace.
pub static BUILTIN_ANALYZERS: &[AnalyzerSpec] = &[
    AnalyzerSpec {
        exe: "rg",
        args: &["--json", "--type", "rust", "-e", "unsafe\\s*\\{", "{target}"],
        category: "memory-safety",
        tier: FindingTier::High,
        timeout: Duration::from_secs(10),
    },
    AnalyzerSpec {
        exe: "rg",
        args: &["--json", "-e", "TODO|FIXME|HACK|XXX", "{target}"],
        category: "technical-debt",
        tier: FindingTier::Low,
        timeout: Duration::from_secs(8),
    },
    AnalyzerSpec {
        exe: "rg",
        args: &[
            "--json", "--type", "ts", "--type", "tsx",
            "-e", "console\\.log|debugger", "{target}",
        ],
        category: "debug-artifacts",
        tier: FindingTier::Info,
        timeout: Duration::from_secs(8),
    },
];

// ── AIM segment writer ────────────────────────────────────────────────────────

/// Lightweight ring-buffer writer over a memory-mapped `.aim` segment.
///
/// Layout (all little-endian):
/// - `[0..8]`   magic  = `b"AIMEVENT"`
/// - `[8..12]`  head   = u32 (byte offset of next write, wraps at BODY_SIZE)
/// - `[12..16]` count  = u32 (total events written, monotonic)
/// - `[16..]`   body   = newline-delimited JSON events
struct AimSegmentWriter {
    mmap: MmapMut,
}

const MAGIC: &[u8; 8] = b"AIMEVENT";
const HEADER_SIZE: usize = 16;
const SEGMENT_SIZE: usize = 2 * 1024 * 1024; // 2 MiB per-pipeline segment
const BODY_SIZE: usize = SEGMENT_SIZE - HEADER_SIZE;

impl AimSegmentWriter {
    fn open(path: &PathBuf) -> std::io::Result<Self> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        file.set_len(SEGMENT_SIZE as u64)?;
        // SAFETY: file is exclusively owned by this writer during the pipeline run.
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };

        // Initialise header if magic is missing.
        if &mmap[0..8] != MAGIC {
            mmap[0..8].copy_from_slice(MAGIC);
            mmap[8..12].copy_from_slice(&0u32.to_le_bytes()); // head
            mmap[12..16].copy_from_slice(&0u32.to_le_bytes()); // count
        }

        Ok(Self { mmap })
    }

    /// Append a serialised event (newline-terminated) using ring semantics.
    fn append(&mut self, event: &TelemetryEvent) {
        let Ok(mut json) = serde_json::to_string(event) else { return };
        json.push('\n');
        let bytes = json.as_bytes();
        if bytes.len() > BODY_SIZE { return; } // single event too large — drop

        let head = u32::from_le_bytes(self.mmap[8..12].try_into().unwrap()) as usize;
        let count = u32::from_le_bytes(self.mmap[12..16].try_into().unwrap());
        let body_start = HEADER_SIZE;

        let write_at = HEADER_SIZE + head;
        let space_left = BODY_SIZE - head;

        if bytes.len() <= space_left {
            self.mmap[write_at..write_at + bytes.len()].copy_from_slice(bytes);
        } else {
            // Wrap around: write what fits, then continue from body start.
            self.mmap[write_at..write_at + space_left].copy_from_slice(&bytes[..space_left]);
            let remainder = bytes.len() - space_left;
            self.mmap[body_start..body_start + remainder].copy_from_slice(&bytes[space_left..]);
        }

        let new_head = (head + bytes.len()) % BODY_SIZE;
        self.mmap[8..12].copy_from_slice(&(new_head as u32).to_le_bytes());
        self.mmap[12..16].copy_from_slice(&(count + 1).to_le_bytes());
        let _ = self.mmap.flush_async_range(0, HEADER_SIZE + new_head);
    }

    fn event_count(&self) -> u32 {
        u32::from_le_bytes(self.mmap[12..16].try_into().unwrap())
    }
}

// ── Pipeline ──────────────────────────────────────────────────────────────────

pub struct TelemetryPipeline {
    /// Concurrency ceiling: exactly 1 concurrent heavy task.
    semaphore: Arc<Semaphore>,
    aim_path: PathBuf,
    workspace_root: PathBuf,
    inference_url: String,
}

impl TelemetryPipeline {
    pub fn new(aim_path: PathBuf, workspace_root: PathBuf, inference_url: &str) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(1)),
            aim_path,
            workspace_root,
            inference_url: inference_url.trim_end_matches('/').to_string(),
        }
    }

    /// Run all registered analyzers sequentially (Semaphore(1) enforced).
    /// Returns the collected events.
    pub async fn run(&self) -> Vec<TelemetryEvent> {
        // Acquire the single permit — if another run is in progress, this
        // await blocks until it finishes. Never two deep-dives at once.
        let _permit = self.semaphore.acquire().await.expect("semaphore closed");

        let mut events: Vec<TelemetryEvent> = Vec::new();
        let target = self.workspace_root.to_string_lossy().to_string();

        let mut aim = match AimSegmentWriter::open(&self.aim_path) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("[telemetry] aim open failed: {e}");
                return events;
            }
        };

        for spec in BUILTIN_ANALYZERS {
            let resolved_args: Vec<String> = spec
                .args
                .iter()
                .map(|a| a.replace("{target}", &target))
                .collect();

            if let Some(ev) = run_analyzer(spec, &resolved_args).await {
                aim.append(&ev);
                events.push(ev);
            }
        }

        println!(
            "[telemetry] run complete — {} new events, {} total in aim segment",
            events.len(),
            aim.event_count()
        );

        events
    }

    /// Send collected events to the local model for synthesis, capped at num_ctx 8192.
    /// Returns a structured JSON string on success.
    pub async fn synthesize(&self, events: &[TelemetryEvent]) -> Result<String, String> {
        if events.is_empty() {
            return Ok("{}".to_string());
        }

        let context = build_synthesis_prompt(events);
        let body = serde_json::json!({
            "model": "qwen2.5-coder:7b",
            "stream": false,
            "options": { "num_ctx": 8192 },
            "prompt": context,
        });

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client
            .post(format!("{}/api/generate", self.inference_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("local inference unreachable: {e}"))?;

        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        Ok(json["response"].as_str().unwrap_or("{}").to_string())
    }
}

// ── Analyzer runner ───────────────────────────────────────────────────────────

async fn run_analyzer(spec: &AnalyzerSpec, args: &[String]) -> Option<TelemetryEvent> {
    let mut cmd = Command::new(spec.exe);
    cmd.args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null());

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return None, // binary not installed — skip silently
    };

    let stdout = child.stdout.take()?;
    let mut reader = BufReader::new(stdout).lines();
    let mut lines: Vec<String> = Vec::new();
    let deadline = tokio::time::Instant::now() + spec.timeout;

    loop {
        let next = tokio::time::timeout_at(deadline, reader.next_line());
        match next.await {
            Ok(Ok(Some(line))) => {
                lines.push(line);
                if lines.len() >= 200 { break; } // hard cap: 200 lines per analyzer
            }
            _ => break,
        }
    }
    let _ = child.kill().await;

    if lines.is_empty() {
        return None;
    }

    let summary = format!("{} matches from `{} {}`", lines.len(), spec.exe, args.join(" "));
    let raw = lines.join("\n");

    Some(
        TelemetryEvent::new(spec.exe, spec.tier.clone(), spec.category, &summary, &summary)
            .with_raw(raw),
    )
}

// ── Prompt builder ────────────────────────────────────────────────────────────

fn build_synthesis_prompt(events: &[TelemetryEvent]) -> String {
    // Serialize events as compact JSON array, truncated so total stays ≤ 6000 chars
    // (leaves headroom for system prompt + response within num_ctx 8192).
    let mut acc = String::with_capacity(6000);
    for ev in events {
        if let Ok(s) = serde_json::to_string(ev) {
            if acc.len() + s.len() > 5500 { break; }
            acc.push_str(&s);
            acc.push('\n');
        }
    }

    format!(
        r#"You are a static-analysis synthesizer for a Rust/TypeScript IDE workbench.
Analyse the following raw telemetry events and identify complex logic anomalies,
dependency chains, and risk patterns. Return ONLY a valid JSON object matching
this schema exactly — no prose, no markdown, no code fences:

{{
  "summary": "<one-sentence overall assessment>",
  "risk_score": <integer 0-100>,
  "findings": [
    {{
      "tier": "critical|high|medium|low|info",
      "category": "<string>",
      "title": "<string>",
      "description": "<string>",
      "affected_paths": ["<path>"],
      "recommendation": "<string>"
    }}
  ],
  "dependency_chains": [
    {{ "from": "<symbol>", "to": "<symbol>", "risk": "<string>" }}
  ],
  "generated_at_ns": <unix-nanoseconds>
}}

EVENTS:
{acc}"#
    )
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    #[test]
    fn aim_segment_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("telemetry.aim");

        let mut writer = AimSegmentWriter::open(&path).unwrap();
        let ev = TelemetryEvent::new("test", FindingTier::High, "cat", "title", "desc");
        writer.append(&ev);
        assert_eq!(writer.event_count(), 1);

        // Re-open and verify magic is preserved.
        let writer2 = AimSegmentWriter::open(&path).unwrap();
        assert_eq!(writer2.event_count(), 1);
    }

    #[test]
    fn synthesis_prompt_stays_under_limit() {
        let events: Vec<TelemetryEvent> = (0..50)
            .map(|i| TelemetryEvent::new("rg", FindingTier::Low, "cat", format!("t{i}"), format!("d{i}")))
            .collect();
        let prompt = build_synthesis_prompt(&events);
        assert!(prompt.len() < 8000, "prompt too long: {} chars", prompt.len());
    }
}
