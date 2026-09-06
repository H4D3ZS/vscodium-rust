//! Multi-vector synthesis and discrepancy aggregator.
//!
//! Maintains a persistent in-memory findings log, assembles the context
//! prompt for the local Ollama engine, and exposes Tauri commands for the
//! TriageWorkbench UI canvas.
//!
//! DDD placement note: business state (`OrchestratorState`) lives here rather
//! than in `domain/` because it is tightly coupled to the infrastructure
//! pipeline; domain consumers only see the `CanvasSnapshot` value type.

use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;

use crate::infrastructure::telemetry_pipeline::{FindingTier, TelemetryEvent, TelemetryPipeline};

// ── Value types (shared with frontend) ───────────────────────────────────────

/// A single resolved finding, ready for canvas rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasFinding {
    pub id: String,
    pub tier: FindingTier,
    pub category: String,
    pub title: String,
    pub description: String,
    pub affected_paths: Vec<String>,
    pub recommendation: String,
    pub timestamp_ns: u64,
    /// Raw analyzer output for the code visualizer panel.
    pub raw: Option<String>,
}

/// Dependency edge for the chain view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub risk: String,
}

/// Full snapshot emitted to the UI canvas on every synthesis cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasSnapshot {
    pub run_id: String,
    pub generated_at_ns: u64,
    pub risk_score: u8,
    pub summary: String,
    pub findings: Vec<CanvasFinding>,
    pub dependency_chains: Vec<DependencyEdge>,
    /// Counts per tier for the summary bar.
    pub tier_counts: TierCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TierCounts {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub info: usize,
}

impl TierCounts {
    fn from_findings(findings: &[CanvasFinding]) -> Self {
        let mut tc = Self::default();
        for f in findings {
            match f.tier {
                FindingTier::Critical => tc.critical += 1,
                FindingTier::High => tc.high += 1,
                FindingTier::Medium => tc.medium += 1,
                FindingTier::Low => tc.low += 1,
                FindingTier::Info => tc.info += 1,
            }
        }
        tc
    }
}

// ── Orchestrator state ────────────────────────────────────────────────────────

/// Drift record: minor individual data point registered between synthesis cycles.
#[derive(Debug, Clone)]
struct DriftPoint {
    category: String,
    key: String,
    value: String,
    timestamp_ns: u64,
}

pub struct OrchestratorState {
    pipeline: TelemetryPipeline,
    /// Ring buffer of raw telemetry events from the most recent pipeline run.
    event_log: VecDeque<TelemetryEvent>,
    /// Minor drift points accumulated between synthesis cycles.
    drift_log: Vec<DriftPoint>,
    /// Most recent synthesised snapshot.
    last_snapshot: Option<CanvasSnapshot>,
    /// Total synthesis cycles completed.
    run_count: u64,
}

impl OrchestratorState {
    pub fn new(workspace_root: PathBuf, config_dir: PathBuf, inference_url: &str) -> Self {
        let aim_path = config_dir.join("telemetry.aim");
        Self {
            pipeline: TelemetryPipeline::new(aim_path, workspace_root, inference_url),
            event_log: VecDeque::with_capacity(512),
            drift_log: Vec::new(),
            last_snapshot: None,
            run_count: 0,
        }
    }

    /// Register a minor configuration drift point.
    pub fn register_drift(&mut self, category: &str, key: &str, value: &str) {
        let timestamp_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_nanos() as u64;
        self.drift_log.push(DriftPoint {
            category: category.to_string(),
            key: key.to_string(),
            value: value.to_string(),
            timestamp_ns,
        });
        // Keep the drift log bounded.
        if self.drift_log.len() > 1000 {
            self.drift_log.drain(0..200);
        }
    }

    /// Run the full pipeline and synthesise a new snapshot.
    pub async fn run_cycle(&mut self) -> Result<CanvasSnapshot, String> {
        // 1. Collect raw events from all registered analyzers.
        let raw_events = self.pipeline.run().await;

        // 2. Merge into the ring buffer (keep last 512 events).
        for ev in &raw_events {
            if self.event_log.len() >= 512 {
                self.event_log.pop_front();
            }
            self.event_log.push_back(ev.clone());
        }

        // 3. Build unified event list: recent ring + any critical drift points.
        let mut synthesis_events: Vec<TelemetryEvent> = self.event_log.iter().cloned().collect();
        for dp in &self.drift_log {
            synthesis_events.push(TelemetryEvent::new(
                "drift",
                FindingTier::Info,
                &dp.category,
                format!("config drift: {}", dp.key),
                format!("value={}", dp.value),
            ));
        }

        // 4. Send to Ollama for synthesis.
        let ollama_json = self.pipeline.synthesize(&synthesis_events).await?;

        // 5. Parse the JSON response into a CanvasSnapshot.
        let snapshot = parse_synthesis_response(&ollama_json, self.run_count)?;

        self.last_snapshot = Some(snapshot.clone());
        self.run_count += 1;
        Ok(snapshot)
    }

    pub fn last_snapshot(&self) -> Option<&CanvasSnapshot> {
        self.last_snapshot.as_ref()
    }

    pub fn drift_count(&self) -> usize {
        self.drift_log.len()
    }

    pub fn event_count(&self) -> usize {
        self.event_log.len()
    }
}

// ── JSON response parser ──────────────────────────────────────────────────────

fn parse_synthesis_response(raw: &str, run_id: u64) -> Result<CanvasSnapshot, String> {
    // Strip markdown fences if the model wrapped the JSON.
    let clean = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let v: serde_json::Value =
        serde_json::from_str(clean).map_err(|e| format!("synthesis JSON parse failed: {e}"))?;

    let generated_at_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_nanos() as u64;

    let findings: Vec<CanvasFinding> = v["findings"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let tier_str = f["tier"].as_str().unwrap_or("info");
            let tier = match tier_str {
                "critical" => FindingTier::Critical,
                "high" => FindingTier::High,
                "medium" => FindingTier::Medium,
                "low" => FindingTier::Low,
                _ => FindingTier::Info,
            };
            CanvasFinding {
                id: format!("run{run_id}-{i}"),
                tier,
                category: f["category"].as_str().unwrap_or("general").to_string(),
                title: f["title"].as_str().unwrap_or("Untitled finding").to_string(),
                description: f["description"].as_str().unwrap_or("").to_string(),
                affected_paths: f["affected_paths"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|p| p.as_str())
                    .map(String::from)
                    .collect(),
                recommendation: f["recommendation"].as_str().unwrap_or("").to_string(),
                timestamp_ns: generated_at_ns,
                raw: None,
            }
        })
        .collect();

    let dependency_chains: Vec<DependencyEdge> = v["dependency_chains"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|e| DependencyEdge {
            from: e["from"].as_str().unwrap_or("?").to_string(),
            to: e["to"].as_str().unwrap_or("?").to_string(),
            risk: e["risk"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    let tier_counts = TierCounts::from_findings(&findings);

    Ok(CanvasSnapshot {
        run_id: format!("run-{run_id}"),
        generated_at_ns,
        risk_score: v["risk_score"].as_u64().unwrap_or(0).min(100) as u8,
        summary: v["summary"].as_str().unwrap_or("No summary available.").to_string(),
        findings,
        dependency_chains,
        tier_counts,
    })
}

// ── Tauri commands ────────────────────────────────────────────────────────────

/// Global handle — initialised once in setup, shared via `Arc<Mutex<>>`.
pub type OrchestratorHandle = Arc<Mutex<OrchestratorState>>;

pub fn init(workspace_root: PathBuf, config_dir: PathBuf, inference_url: &str) -> OrchestratorHandle {
    Arc::new(Mutex::new(OrchestratorState::new(
        workspace_root,
        config_dir,
        inference_url,
    )))
}

/// Trigger a full analysis cycle. Returns the new CanvasSnapshot.
/// Blocks until Ollama responds. The Semaphore in TelemetryPipeline ensures
/// this is safe to invoke concurrently — the second caller waits.
#[tauri::command]
pub async fn triage_run(
    orch_state: State<'_, OrchestratorHandle>,
    app: AppHandle,
) -> Result<CanvasSnapshot, String> {
    let snapshot = orch_state.lock().await.run_cycle().await?;
    // Emit the snapshot so any mounted TriageWorkbench receives it in real time.
    app.emit("triage:snapshot", &snapshot)
        .map_err(|e| e.to_string())?;
    Ok(snapshot)
}

/// Return the most recent snapshot without running a new cycle.
#[tauri::command]
pub async fn triage_last_snapshot(
    orch_state: State<'_, OrchestratorHandle>,
) -> Result<Option<CanvasSnapshot>, String> {
    Ok(orch_state.lock().await.last_snapshot().cloned())
}

/// Register a configuration drift point from the frontend.
#[tauri::command]
pub async fn triage_register_drift(
    orch_state: State<'_, OrchestratorHandle>,
    category: String,
    key: String,
    value: String,
) -> Result<(), String> {
    orch_state.lock().await.register_drift(&category, &key, &value);
    Ok(())
}

/// Return high-level queue stats (no secrets, safe to expose).
#[tauri::command]
pub async fn triage_stats(
    orch_state: State<'_, OrchestratorHandle>,
) -> Result<serde_json::Value, String> {
    let orch = orch_state.lock().await;
    Ok(serde_json::json!({
        "event_count": orch.event_count(),
        "drift_count": orch.drift_count(),
        "last_run_id": orch.last_snapshot().map(|s| s.run_id.clone()),
    }))
}
