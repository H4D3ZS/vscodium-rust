//! 24/7 autonomous supervisor.
//!
//! Maintains a durable task queue and works it unattended, forever, using the
//! local **Sentient** agent (Lemonade / BYOK). For each task it:
//!   1. snapshots the current git branch and starts an isolated branch,
//!   2. runs the task prompt through `Sentient::autonomous_loop` (its own tool
//!      loop does the edits / commands),
//!   3. runs the verify gate (`cargo check` / `npm run typecheck`),
//!   4. on green → checkpoint commit on the isolated branch; on red → revert.
//!
//! It never pushes, never holds the state lock during long work, and recovers from
//! every error path (agent error, timeout, verify failure) by requeuing or failing
//! the task — so it survives being left running overnight.

use std::{
    collections::VecDeque,
    path::PathBuf,
    process::Stdio,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;

use crate::domain::ai::engine::types::{AiRequest, ChatMessage, MessageContent};

const SUPERVISOR_SYSTEM: &str = "You are an autonomous background engineer. \
Complete the task end to end using your tools: read the relevant files, make \
the change, and make sure it is coherent. Do not ask questions. When the task \
is fully done, stop.";

// ── Task model ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Queued,
    Running,
    Verifying,
    Committed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorTask {
    pub id: String,
    pub prompt: String,
    pub status: TaskStatus,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub commit: Option<String>,
    pub created_ns: u64,
    #[serde(default)]
    pub attempts: u32,
    #[serde(default)]
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorConfig {
    /// Max assistant turns before a task attempt is abandoned.
    pub max_steps_per_task: u32,
    /// Wall-clock budget per task attempt.
    pub task_timeout_secs: u64,
    /// How many providers to try before marking the task failed.
    pub max_attempts: u32,
    /// Cooldown applied to a provider that stalled / rate-limited.
    pub provider_cooldown_secs: u64,
    /// Branch name prefix for isolated work.
    pub branch_prefix: String,
    /// Commit + verify, or just verify (false = dry, never touches git).
    pub auto_commit: bool,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        Self {
            max_steps_per_task: 30,
            task_timeout_secs: 900,
            max_attempts: 3,
            provider_cooldown_secs: 300,
            branch_prefix: "auto/supervisor-".into(),
            auto_commit: true,
        }
    }
}

// ── State ─────────────────────────────────────────────────────────────────────

pub struct SupervisorState {
    queue: VecDeque<SupervisorTask>,
    history: Vec<SupervisorTask>,
    running: Option<SupervisorTask>,
    paused: bool,
    config: SupervisorConfig,
    state: Arc<crate::EditorState>,
    workspace_root: PathBuf,
    queue_file: PathBuf,
    seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PersistedQueue {
    queue: Vec<SupervisorTask>,
    history: Vec<SupervisorTask>,
    seq: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SupervisorSnapshot {
    pub paused: bool,
    pub running: Option<SupervisorTask>,
    pub queue: Vec<SupervisorTask>,
    pub history: Vec<SupervisorTask>,
    pub config: SupervisorConfig,
}

impl SupervisorState {
    fn now_ns() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_nanos() as u64
    }

    fn snapshot(&self) -> SupervisorSnapshot {
        SupervisorSnapshot {
            paused: self.paused,
            running: self.running.clone(),
            queue: self.queue.iter().cloned().collect(),
            history: self.history.iter().rev().take(50).cloned().collect(),
            config: self.config.clone(),
        }
    }

    fn persist(&self) {
        let data = PersistedQueue {
            queue: self.queue.iter().cloned().collect(),
            history: self.history.clone(),
            seq: self.seq,
        };
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let _ = std::fs::write(&self.queue_file, json);
        }
    }

    fn enqueue(&mut self, prompt: String) -> String {
        self.seq += 1;
        let id = format!("task_{:04}", self.seq);
        self.queue.push_back(SupervisorTask {
            id: id.clone(),
            prompt,
            status: TaskStatus::Queued,
            provider: None,
            branch: None,
            commit: None,
            created_ns: Self::now_ns(),
            attempts: 0,
            last_error: None,
        });
        self.persist();
        id
    }
}

pub type SupervisorHandle = Arc<Mutex<SupervisorState>>;

/// Initialise the supervisor, load any persisted queue, and spawn the always-on loop.
pub fn init(
    app: AppHandle,
    state_arc: Arc<crate::EditorState>,
    workspace_root: PathBuf,
    config_dir: PathBuf,
) -> SupervisorHandle {
    let queue_file = config_dir.join("supervisor_queue.json");
    let loaded: PersistedQueue = std::fs::read_to_string(&queue_file)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    // Any task left "running"/"verifying" from a previous session is requeued.
    let mut queue: VecDeque<SupervisorTask> = VecDeque::new();
    for mut t in loaded.queue {
        if matches!(t.status, TaskStatus::Running | TaskStatus::Verifying) {
            t.status = TaskStatus::Queued;
        }
        queue.push_back(t);
    }

    let state = Arc::new(Mutex::new(SupervisorState {
        queue,
        history: loaded.history,
        running: None,
        paused: false,
        config: SupervisorConfig::default(),
        state: state_arc,
        workspace_root,
        queue_file,
        seq: loaded.seq,
    }));

    let loop_state = state.clone();
    tauri::async_runtime::spawn(async move {
        run_loop(loop_state, app).await;
    });

    state
}

// ── The always-on loop ────────────────────────────────────────────────────────

async fn run_loop(handle: SupervisorHandle, app: AppHandle) {
    loop {
        // Briefly lock to pull the next task (skip while paused).
        let next = {
            let mut s = handle.lock().await;
            if s.paused {
                None
            } else {
                s.queue.pop_front()
            }
        };

        let mut task = match next {
            Some(t) => t,
            None => {
                tokio::time::sleep(Duration::from_secs(3)).await;
                continue;
            }
        };

        // Snapshot the bits we need without holding the lock during long work.
        let (state, workspace_root, config) = {
            let s = handle.lock().await;
            (s.state.clone(), s.workspace_root.clone(), s.config.clone())
        };

        task.status = TaskStatus::Running;
        task.attempts += 1;
        set_running(&handle, Some(task.clone()), &app).await;

        let result = execute_task(&state, &workspace_root, &config, &mut task).await;

        match result {
            Ok(commit) => {
                task.status = TaskStatus::Committed;
                task.commit = commit;
                task.last_error = None;
            }
            Err(TaskError::Retryable(msg)) if task.attempts < config.max_attempts => {
                task.status = TaskStatus::Queued;
                task.last_error = Some(msg);
                let mut s = handle.lock().await;
                s.queue.push_back(task.clone());
                s.running = None;
                s.persist();
                drop(s);
                emit(&app, &handle).await;
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
            Err(e) => {
                task.status = TaskStatus::Failed;
                task.last_error = Some(e.message());
            }
        }

        // Record terminal result.
        {
            let mut s = handle.lock().await;
            s.running = None;
            s.history.push(task.clone());
            if s.history.len() > 500 {
                s.history.drain(0..100);
            }
            s.persist();
        }
        emit(&app, &handle).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn set_running(handle: &SupervisorHandle, task: Option<SupervisorTask>, app: &AppHandle) {
    {
        let mut s = handle.lock().await;
        s.running = task;
        s.persist();
    }
    emit(app, handle).await;
}

async fn emit(app: &AppHandle, handle: &SupervisorHandle) {
    let snap = handle.lock().await.snapshot();
    let _ = app.emit("supervisor:update", &snap);
}

// ── Task execution ────────────────────────────────────────────────────────────

enum TaskError {
    /// Worth retrying on a different provider (timeout, no tab, disconnect).
    Retryable(String),
    /// Terminal (verify failed, git error).
    Fatal(String),
}

impl TaskError {
    fn message(&self) -> String {
        match self {
            TaskError::Retryable(m) | TaskError::Fatal(m) => m.clone(),
        }
    }
}

/// Run one full attempt: isolate branch → run local agent → verify → commit/revert.
async fn execute_task(
    state: &Arc<crate::EditorState>,
    workspace_root: &PathBuf,
    config: &SupervisorConfig,
    task: &mut SupervisorTask,
) -> Result<Option<String>, TaskError> {
    // 1. Local agent: the user's active model on the local backend. (BYOK models
    //    still work — autonomous_loop resolves provider-specific routing.)
    let model = state.ai.current_model.lock().await.clone();
    let provider = "lemonade".to_string();
    task.provider = Some(provider.clone());

    // 2. Isolate work on a fresh branch (only when auto-commit + inside a git repo).
    let is_git = git(workspace_root, &["rev-parse", "--is-inside-work-tree"]).await.is_ok();
    let original_branch = if is_git {
        git(workspace_root, &["rev-parse", "--abbrev-ref", "HEAD"]).await.ok()
    } else {
        None
    };
    let branch = format!("{}{}", config.branch_prefix, task.id);
    if config.auto_commit && is_git {
        // Defensive: clean any leftover dirty state, then branch from the current ref.
        let _ = git(workspace_root, &["reset", "--hard"]).await;
        if let Some(orig) = &original_branch {
            let _ = git(workspace_root, &["checkout", orig]).await;
        }
        // Delete a stale branch of the same name, then create fresh.
        let _ = git(workspace_root, &["branch", "-D", &branch]).await;
        git(workspace_root, &["checkout", "-b", &branch])
            .await
            .map_err(|e| TaskError::Fatal(format!("git branch failed: {e}")))?;
        task.branch = Some(branch.clone());
    }

    // 3. Run the task through the local Sentient agent. Its own tool loop makes
    //    the edits / runs commands; it returns when the task is done. We bound it
    //    with the per-task wall-clock budget.
    let req = AiRequest {
        provider: provider.clone(),
        model: model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(SUPERVISOR_SYSTEM.to_string())),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(task.prompt.clone())),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: true,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Agent".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };
    let run = tokio::time::timeout(
        Duration::from_secs(config.task_timeout_secs),
        state.ai.engine.clone().autonomous_loop(req, None),
    )
    .await;
    match run {
        Err(_) => return Err(TaskError::Retryable("task timed out".into())),
        Ok(Err(e)) => return Err(TaskError::Retryable(format!("agent error: {e}"))),
        Ok(Ok(_final_text)) => { /* agent finished — proceed to verify */ }
    }

    // 4. Verify gate.
    task.status = TaskStatus::Verifying;
    if let Err(report) = verify(workspace_root).await {
        // Revert the agent's changes so the tree is never left broken.
        if config.auto_commit && is_git {
            let _ = git(workspace_root, &["reset", "--hard"]).await;
            if let Some(orig) = &original_branch {
                let _ = git(workspace_root, &["checkout", orig]).await;
            }
            let _ = git(workspace_root, &["branch", "-D", &branch]).await;
        }
        return Err(TaskError::Fatal(format!("verify failed: {report}")));
    }

    // 5. Commit on green.
    if config.auto_commit && is_git {
        git(workspace_root, &["add", "-A"]).await.ok();
        let msg = format!("auto({}): {}", task.id, first_line(&task.prompt));
        // `commit` returns non-zero if nothing changed — treat that as success.
        let _ = git(workspace_root, &["commit", "-m", &msg]).await;
        let commit = git(workspace_root, &["rev-parse", "--short", "HEAD"]).await.ok();
        return Ok(commit.map(|c| c.trim().to_string()));
    }

    Ok(None)
}

fn first_line(s: &str) -> String {
    s.lines().next().unwrap_or("").chars().take(72).collect()
}

// ── Verify gate ───────────────────────────────────────────────────────────────

/// Run the project's verify commands. Returns Err(report) on the first failure.
async fn verify(workspace_root: &PathBuf) -> Result<(), String> {
    // Rust: cargo check in src-tauri if present, else workspace root.
    let src_tauri = workspace_root.join("src-tauri");
    if src_tauri.join("Cargo.toml").exists() {
        run_check(&src_tauri, "cargo", &["check", "--quiet"]).await?;
    } else if workspace_root.join("Cargo.toml").exists() {
        run_check(workspace_root, "cargo", &["check", "--quiet"]).await?;
    }
    // TS: npm run typecheck if a package.json declares it.
    if workspace_root.join("package.json").exists() {
        // Best-effort; if the script is missing npm exits non-zero and we report it.
        run_check(workspace_root, "npm", &["run", "typecheck"]).await?;
    }
    Ok(())
}

async fn run_check(cwd: &PathBuf, program: &str, args: &[&str]) -> Result<(), String> {
    let out = tokio::process::Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("{program} spawn failed: {e}"))?;
    if out.status.success() {
        Ok(())
    } else {
        let mut tail = String::from_utf8_lossy(&out.stderr).to_string();
        if tail.len() > 1500 {
            tail = tail[tail.len() - 1500..].to_string();
        }
        Err(format!("{program} {}: {tail}", args.join(" ")))
    }
}

async fn git(cwd: &PathBuf, args: &[&str]) -> Result<String, String> {
    let out = tokio::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("git spawn failed: {e}"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

// ── Tauri commands ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn supervisor_enqueue(
    sup: State<'_, SupervisorHandle>,
    prompt: String,
) -> Result<String, String> {
    let id = sup.lock().await.enqueue(prompt);
    Ok(id)
}

#[tauri::command]
pub async fn supervisor_status(
    sup: State<'_, SupervisorHandle>,
) -> Result<SupervisorSnapshot, String> {
    Ok(sup.lock().await.snapshot())
}

#[tauri::command]
pub async fn supervisor_pause(sup: State<'_, SupervisorHandle>) -> Result<(), String> {
    let mut s = sup.lock().await;
    s.paused = true;
    s.persist();
    Ok(())
}

#[tauri::command]
pub async fn supervisor_resume(sup: State<'_, SupervisorHandle>) -> Result<(), String> {
    let mut s = sup.lock().await;
    s.paused = false;
    Ok(())
}

#[tauri::command]
pub async fn supervisor_skip(
    sup: State<'_, SupervisorHandle>,
    task_id: String,
) -> Result<bool, String> {
    let mut s = sup.lock().await;
    let before = s.queue.len();
    s.queue.retain(|t| t.id != task_id);
    let removed = s.queue.len() != before;
    if removed {
        s.persist();
    }
    Ok(removed)
}

#[tauri::command]
pub async fn supervisor_clear(sup: State<'_, SupervisorHandle>) -> Result<(), String> {
    let mut s = sup.lock().await;
    s.queue.clear();
    s.persist();
    Ok(())
}

#[tauri::command]
pub async fn supervisor_set_config(
    sup: State<'_, SupervisorHandle>,
    config: SupervisorConfig,
) -> Result<(), String> {
    let mut s = sup.lock().await;
    s.config = config;
    Ok(())
}
