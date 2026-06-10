//! Antigravity IDE on-disk conventions (reference: Antigravity_IDE install).
//! Brain artifacts, trajectory, lifecycle hooks, autonomy policies — no binary import.

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn brain_root(root: &Path, cascade_id: &str) -> PathBuf {
    root.join(".agent").join("brain").join(cascade_id)
}

pub fn hooks_path(root: &Path) -> PathBuf {
    root.join(".agent").join("hooks.json")
}

pub fn autonomy_path(root: &Path) -> PathBuf {
    root.join(".agent").join("autonomy.json")
}

pub fn trajectory_path(root: &Path, cascade_id: &str) -> PathBuf {
    root.join(".agent").join("runs").join(format!("{cascade_id}.json"))
}

// ── Brain artifacts (`.agent/brain/{cascadeId}/`) ────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactMetadata {
    #[serde(rename = "artifactType")]
    pub artifact_type: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(default, rename = "requestFeedback")]
    pub request_feedback: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BrainArtifactInfo {
    pub name: String,
    pub path: String,
    pub artifact_type: String,
    pub summary: Option<String>,
    pub updated_at: String,
    pub is_media: bool,
}

fn iso_now() -> String {
    chrono::Local::now().to_rfc3339()
}

pub fn ensure_brain_dir(root: &Path, cascade_id: &str) -> Result<PathBuf> {
    let dir = brain_root(root, cascade_id);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn save_brain_artifact(
    root: &Path,
    cascade_id: &str,
    filename: &str,
    content: &str,
    artifact_type: &str,
    summary: Option<&str>,
) -> Result<PathBuf> {
    let dir = ensure_brain_dir(root, cascade_id)?;
    let safe = sanitize_filename(filename);
    let path = dir.join(&safe);
    fs::write(&path, content)?;
    let meta = ArtifactMetadata {
        artifact_type: artifact_type.to_string(),
        summary: summary.map(str::to_string),
        updated_at: iso_now(),
        request_feedback: Some(false),
    };
    let meta_path = dir.join(format!("{safe}.metadata.json"));
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;
    Ok(path)
}

pub fn save_brain_media(root: &Path, cascade_id: &str, png_b64: &str, summary: Option<&str>) -> Result<PathBuf> {
    let dir = ensure_brain_dir(root, cascade_id)?;
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let name = format!("media__{ts}.png");
    let bytes = B64.decode(png_b64.trim()).map_err(|e| anyhow!("Invalid base64: {e}"))?;
    let path = dir.join(&name);
    fs::write(&path, bytes)?;
    let meta = ArtifactMetadata {
        artifact_type: "ARTIFACT_TYPE_MEDIA_SCREENSHOT".into(),
        summary: summary.map(str::to_string),
        updated_at: iso_now(),
        request_feedback: None,
    };
    fs::write(
        dir.join(format!("{name}.metadata.json")),
        serde_json::to_string_pretty(&meta)?,
    )?;
    Ok(path)
}

pub fn list_brain_artifacts(root: &Path, cascade_id: &str) -> Vec<BrainArtifactInfo> {
    let dir = brain_root(root, cascade_id);
    if !dir.is_dir() {
        return Vec::new();
    }
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_file() {
                continue;
            }
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            if name.ends_with(".metadata.json") {
                continue;
            }
            let is_media = name.starts_with("media__") && name.ends_with(".png");
            let meta_path = dir.join(format!("{name}.metadata.json"));
            let (artifact_type, summary, updated_at) = if meta_path.is_file() {
                if let Ok(raw) = fs::read_to_string(&meta_path) {
                    if let Ok(m) = serde_json::from_str::<ArtifactMetadata>(&raw) {
                        (m.artifact_type, m.summary, m.updated_at)
                    } else {
                        ("ARTIFACT_TYPE_UNKNOWN".into(), None, iso_now())
                    }
                } else {
                    ("ARTIFACT_TYPE_UNKNOWN".into(), None, iso_now())
                }
            } else {
                let t = match name.as_str() {
                    "task.md" => "ARTIFACT_TYPE_TASK",
                    "implementation_plan.md" => "ARTIFACT_TYPE_IMPLEMENTATION_PLAN",
                    "walkthrough.md" => "ARTIFACT_TYPE_WALKTHROUGH",
                    _ => "ARTIFACT_TYPE_FILE",
                };
                (t.to_string(), None, iso_now())
            };
            out.push(BrainArtifactInfo {
                name: name.clone(),
                path: p.to_string_lossy().to_string(),
                artifact_type,
                summary,
                updated_at,
                is_media,
            });
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

pub fn list_brain_cascades(root: &Path) -> Vec<String> {
    let dir = root.join(".agent").join("brain");
    if !dir.is_dir() {
        return Vec::new();
    }
    fs::read_dir(&dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().is_dir())
                .filter_map(|e| {
                    e.file_name()
                        .into_string()
                        .ok()
                })
                .collect()
        })
        .unwrap_or_default()
}

// ── Trajectory (`.agent/runs/{cascadeId}.json`) ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryStep {
    pub id: String,
    pub kind: String,
    pub title: String,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub tool: Option<String>,
    pub timestamp: i64,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub subagent_id: Option<String>,
    #[serde(default)]
    pub media_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentState {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub role: Option<String>,
    pub status: String,
    #[serde(default)]
    pub parent_id: Option<String>,
    pub started_at: i64,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub progress: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryRecord {
    pub id: String,
    pub objective: String,
    pub status: String,
    pub started_at: i64,
    #[serde(default)]
    pub finished_at: Option<i64>,
    #[serde(default)]
    pub steps: Vec<TrajectoryStep>,
    #[serde(default)]
    pub subagents: Vec<SubagentState>,
    #[serde(default)]
    pub artifact_paths: Vec<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

pub fn load_trajectory(root: &Path, cascade_id: &str) -> Option<TrajectoryRecord> {
    let path = trajectory_path(root, cascade_id);
    if !path.is_file() {
        return None;
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
}

pub fn save_trajectory(root: &Path, record: &TrajectoryRecord) -> Result<PathBuf> {
    let dir = root.join(".agent").join("runs");
    fs::create_dir_all(&dir)?;
    let path = trajectory_path(root, &record.id);
    fs::write(&path, serde_json::to_string_pretty(record)?)?;
    Ok(path)
}

pub fn append_trajectory_step(root: &Path, cascade_id: &str, step: TrajectoryStep) -> Result<TrajectoryRecord> {
    let mut record = load_trajectory(root, cascade_id).unwrap_or_else(|| TrajectoryRecord {
        id: cascade_id.to_string(),
        objective: "Agent mission".into(),
        status: "running".into(),
        started_at: step.timestamp,
        finished_at: None,
        steps: Vec::new(),
        subagents: Vec::new(),
        artifact_paths: Vec::new(),
        summary: None,
    });
    if let Some(ref mp) = step.media_path {
        if !record.artifact_paths.contains(mp) {
            record.artifact_paths.push(mp.clone());
        }
    }
    record.steps.push(step);
    save_trajectory(root, &record)?;
    Ok(record)
}

pub fn upsert_subagent(root: &Path, cascade_id: &str, sub: SubagentState) -> Result<()> {
    let mut record = load_trajectory(root, cascade_id).unwrap_or_else(|| TrajectoryRecord {
        id: cascade_id.to_string(),
        objective: "Agent mission".into(),
        status: "running".into(),
        started_at: sub.started_at,
        finished_at: None,
        steps: Vec::new(),
        subagents: Vec::new(),
        artifact_paths: Vec::new(),
        summary: None,
    });
    if let Some(i) = record.subagents.iter().position(|s| s.id == sub.id) {
        record.subagents[i] = sub;
    } else {
        record.subagents.push(sub);
    }
    save_trajectory(root, &record)?;
    Ok(())
}

pub fn list_trajectories(root: &Path) -> Vec<TrajectoryRecord> {
    let dir = root.join(".agent").join("runs");
    if !dir.is_dir() {
        return Vec::new();
    }
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(raw) = fs::read_to_string(&p) {
                if let Ok(r) = serde_json::from_str::<TrajectoryRecord>(&raw) {
                    out.push(r);
                } else if let Ok(simple) = serde_json::from_str::<Value>(&raw) {
                    // Legacy AgentRunRecord
                    if let (Some(id), Some(obj)) = (simple.get("id"), simple.as_object()) {
                        out.push(TrajectoryRecord {
                            id: id.as_str().unwrap_or("run").into(),
                            objective: obj
                                .get("objective")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Mission")
                                .into(),
                            status: obj
                                .get("status")
                                .and_then(|v| v.as_str())
                                .unwrap_or("completed")
                                .into(),
                            started_at: obj.get("started_at").and_then(|v| v.as_i64()).unwrap_or(0),
                            finished_at: obj.get("finished_at").and_then(|v| v.as_i64()),
                            steps: Vec::new(),
                            subagents: Vec::new(),
                            artifact_paths: Vec::new(),
                            summary: obj.get("summary").and_then(|v| v.as_str()).map(str::to_string),
                        });
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    out
}

// ── Lifecycle hooks (`.agent/hooks.json`) ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHookCommand {
    #[serde(rename = "type")]
    pub hook_type: String,
    pub command: String,
    #[serde(default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleMatcher {
    pub matcher: String,
    pub hooks: Vec<LifecycleHookCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHookGroup {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default, rename = "PreToolUse")]
    pub pre_tool_use: Vec<LifecycleMatcher>,
    #[serde(default, rename = "PostToolUse")]
    pub post_tool_use: Vec<LifecycleMatcher>,
    #[serde(default, rename = "PreInvocation")]
    pub pre_invocation: Vec<LifecycleMatcher>,
    #[serde(default, rename = "PostInvocation")]
    pub post_invocation: Vec<LifecycleMatcher>,
    #[serde(default, rename = "Stop")]
    pub stop: Vec<LifecycleMatcher>,
}

pub fn load_lifecycle_hooks(root: &Path) -> Value {
    let path = hooks_path(root);
    if !path.is_file() {
        return json!({});
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_else(|| json!({}))
}

pub fn save_lifecycle_hooks(root: &Path, hooks: &Value) -> Result<PathBuf> {
    let dir = root.join(".agent");
    fs::create_dir_all(&dir)?;
    let path = hooks_path(root);
    fs::write(&path, serde_json::to_string_pretty(hooks)?)?;
    Ok(path)
}

#[derive(Debug, Clone, Serialize)]
pub struct LifecycleHookResult {
    pub hook_name: String,
    pub event: String,
    pub matcher: String,
    pub command: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

fn matcher_matches(matcher: &str, context: &str) -> bool {
    if matcher == "*" {
        return true;
    }
    if matcher.contains('|') {
        return matcher.split('|').any(|m| matcher_matches(m.trim(), context));
    }
    glob::Pattern::new(matcher)
        .map(|p| p.matches(context))
        .unwrap_or(false)
}

pub fn dispatch_lifecycle_hooks(
    root: &Path,
    event: &str,
    context: &str,
) -> Result<Vec<LifecycleHookResult>> {
    let hooks = load_lifecycle_hooks(root);
    let mut out = Vec::new();
    let Some(obj) = hooks.as_object() else {
        return Ok(out);
    };
    for (name, cfg) in obj {
        let Ok(group) = serde_json::from_value::<LifecycleHookGroup>(cfg.clone()) else {
            continue;
        };
        if !group.enabled {
            continue;
        }
        let matchers: &[LifecycleMatcher] = match event {
            "PreToolUse" => &group.pre_tool_use,
            "PostToolUse" => &group.post_tool_use,
            "PreInvocation" => &group.pre_invocation,
            "PostInvocation" => &group.post_invocation,
            "Stop" => &group.stop,
            _ => continue,
        };
        for m in matchers {
            if !matcher_matches(&m.matcher, context) {
                continue;
            }
            for hook in &m.hooks {
                if hook.hook_type != "command" {
                    continue;
                }
                let timeout = hook.timeout.unwrap_or(30);
                let mut cmd = if cfg!(target_os = "windows") {
                    let mut c = Command::new("cmd");
                    c.args(["/C", &hook.command]);
                    c
                } else {
                    let mut c = Command::new("sh");
                    c.args(["-c", &hook.command]);
                    c
                };
                cmd.current_dir(root)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());
                let output = cmd.output();
                let (exit_code, stdout, stderr) = match output {
                    Ok(o) => (
                        o.status.code().unwrap_or(-1),
                        String::from_utf8_lossy(&o.stdout).to_string(),
                        String::from_utf8_lossy(&o.stderr).to_string(),
                    ),
                    Err(e) => (-1, String::new(), e.to_string()),
                };
                let _ = timeout; // best-effort; full timeout wiring later
                out.push(LifecycleHookResult {
                    hook_name: name.clone(),
                    event: event.to_string(),
                    matcher: m.matcher.clone(),
                    command: hook.command.clone(),
                    exit_code,
                    stdout,
                    stderr,
                });
            }
        }
    }
    Ok(out)
}

// ── Autonomy policies (`.agent/autonomy.json`) ────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyPolicies {
    /// Secure mode disables turbo options (Antigravity enterprise pattern).
    #[serde(default)]
    pub secure_mode: bool,
    /// always_ask | turbo | auto
    #[serde(default = "default_artifact_review")]
    pub artifact_review: String,
    /// off | auto | eager | proceed_in_sandbox
    #[serde(default = "default_terminal_auto")]
    pub terminal_auto: String,
    /// disabled | always_ask | turbo | model_decides
    #[serde(default = "default_browser_js")]
    pub browser_js: String,
    /// allow | ask | deny
    #[serde(default = "default_file_access")]
    pub file_access: String,
}

fn default_artifact_review() -> String {
    "always_ask".into()
}
fn default_terminal_auto() -> String {
    "auto".into()
}
fn default_browser_js() -> String {
    "always_ask".into()
}
fn default_file_access() -> String {
    "ask".into()
}

impl Default for AutonomyPolicies {
    fn default() -> Self {
        Self {
            secure_mode: false,
            artifact_review: default_artifact_review(),
            terminal_auto: default_terminal_auto(),
            browser_js: default_browser_js(),
            file_access: default_file_access(),
        }
    }
}

pub fn load_autonomy_policies(root: &Path) -> AutonomyPolicies {
    let path = autonomy_path(root);
    if !path.is_file() {
        return AutonomyPolicies::default();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

pub fn save_autonomy_policies(root: &Path, policies: &AutonomyPolicies) -> Result<PathBuf> {
    let dir = root.join(".agent");
    fs::create_dir_all(&dir)?;
    let path = autonomy_path(root);
    fs::write(&path, serde_json::to_string_pretty(policies)?)?;
    Ok(path)
}

/// Map Antigravity Secure / Auto / Turbo presets onto policy struct.
pub fn apply_autonomy_preset(preset: &str, secure: bool) -> AutonomyPolicies {
    let mut p = AutonomyPolicies {
        secure_mode: secure,
        ..Default::default()
    };
    match preset {
        "turbo" | "full" => {
            if !secure {
                p.artifact_review = "turbo".into();
                p.browser_js = "turbo".into();
            }
            p.terminal_auto = "eager".into();
            p.file_access = "allow".into();
        }
        "auto" | "sandboxed" => {
            p.artifact_review = "auto".into();
            p.terminal_auto = "auto".into();
            p.browser_js = "model_decides".into();
            p.file_access = "ask".into();
        }
        _ => {
            // secure / strict
            p.artifact_review = "always_ask".into();
            p.terminal_auto = "off".into();
            p.browser_js = "always_ask".into();
            p.file_access = "deny".into();
        }
    }
    p
}

pub fn ensure_antigravity_scaffold(root: &Path) -> Result<()> {
    fs::create_dir_all(root.join(".agent").join("brain"))?;
    fs::create_dir_all(root.join(".agent").join("runs"))?;
    fs::create_dir_all(root.join(".agent").join("workflows"))?;
    fs::create_dir_all(root.join(".agent").join("agents"))?;
    let hooks = hooks_path(root);
    if !hooks.is_file() {
        save_lifecycle_hooks(
            root,
            &json!({
                "exampleHook": {
                    "enabled": false,
                    "PreToolUse": [{ "matcher": "run_command", "hooks": [{ "type": "command", "command": "echo pre-tool", "timeout": 10 }] }],
                    "PostToolUse": [],
                    "PreInvocation": [],
                    "PostInvocation": [],
                    "Stop": []
                }
            }),
        )?;
    }
    let autonomy = autonomy_path(root);
    if !autonomy.is_file() {
        save_autonomy_policies(root, &AutonomyPolicies::default())?;
    }
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect()
}
