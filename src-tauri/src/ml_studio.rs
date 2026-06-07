//! ML Studio — dataset prep, training jobs, inference for `.hades/ml/` projects.

use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;

use tauri::State;

use crate::jobs::JobManager;
use crate::process_ext::hidden_command;
use crate::pytorch_commands;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlStudioConfig {
    pub epochs: u32,
    pub learning_rate: f64,
    pub hidden_size: u32,
    pub val_ratio: f64,
    pub embed_model: String,
}

impl Default for MlStudioConfig {
    fn default() -> Self {
        Self {
            epochs: 20,
            learning_rate: 0.001,
            hidden_size: 64,
            val_ratio: 0.2,
            embed_model: "nomic-embed-text".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlDatasetEntry {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlRunSummary {
    pub id: String,
    pub model_path: Option<String>,
    pub metrics_path: Option<String>,
    pub val_acc: Option<f64>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlEpochMetric {
    pub epoch: u32,
    pub train_loss: f64,
    pub val_loss: f64,
    pub val_acc: f64,
    pub lr: f64,
    pub samples_per_sec: f64,
    pub gpu_mem_mb: Option<f64>,
    pub epoch_secs: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlRunMetrics {
    pub status: String,
    pub run_id: String,
    pub total_epochs: u32,
    pub current_epoch: Option<u32>,
    pub lr: Option<f64>,
    pub device: Option<String>,
    pub best_val_acc: Option<f64>,
    pub best_val_loss: Option<f64>,
    pub best_epoch: Option<u32>,
    pub stale_epochs: Option<u32>,
    pub early_stop_patience: Option<u32>,
    pub early_stop: Option<bool>,
    pub history: Vec<MlEpochMetric>,
}

fn ml_root(root: &str) -> PathBuf {
    PathBuf::from(root).join(".hades").join("ml")
}

fn scripts_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../scripts/ml")
}

fn resolve_python() -> Result<String, String> {
    pytorch_commands::resolve_python().ok_or_else(|| "Python not found".to_string())
}

fn read_config(path: &Path) -> MlStudioConfig {
    if path.exists() {
        if let Ok(text) = std::fs::read_to_string(path) {
            if let Ok(cfg) = serde_json::from_str(&text) {
                return cfg;
            }
        }
    }
    MlStudioConfig::default()
}

fn write_config(path: &Path, cfg: &MlStudioConfig) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

fn copy_scripts_to(ml: &Path) -> Result<(), String> {
    let dst = ml.join("scripts");
    std::fs::create_dir_all(&dst).map_err(|e| e.to_string())?;
    for name in ["train_classifier.py", "inference.py"] {
        let src = scripts_dir().join(name);
        if src.exists() {
            std::fs::copy(&src, dst.join(name)).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn csv_columns(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let first = text.lines().next().ok_or("empty csv")?;
    Ok(first
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect())
}

#[tauri::command]
pub async fn ml_studio_init(root: String) -> Result<serde_json::Value, String> {
    let ml = ml_root(&root);
    for sub in ["data", "models", "runs", "scripts"] {
        std::fs::create_dir_all(ml.join(sub)).map_err(|e| e.to_string())?;
    }
    copy_scripts_to(&ml)?;
    let cfg_path = ml.join("config.json");
    if !cfg_path.exists() {
        write_config(&cfg_path, &MlStudioConfig::default())?;
    }
    let readme = ml.join("README.md");
    if !readme.exists() {
        std::fs::write(
            &readme,
            "# HADES ML Studio\n\nPlace CSV datasets in `data/`. Use Settings → PyTorch ML Studio to prepare, train, and infer.\n",
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(serde_json::json!({ "ok": true, "path": ml.to_string_lossy() }))
}

#[tauri::command]
pub async fn ml_studio_get_config(root: String) -> Result<MlStudioConfig, String> {
    Ok(read_config(&ml_root(&root).join("config.json")))
}

#[tauri::command]
pub async fn ml_studio_save_config(root: String, config: MlStudioConfig) -> Result<(), String> {
    write_config(&ml_root(&root).join("config.json"), &config)
}

#[tauri::command]
pub async fn ml_studio_list_data(root: String) -> Result<Vec<MlDatasetEntry>, String> {
    let data_dir = ml_root(&root).join("data");
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&data_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("csv") {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let size_bytes = path.metadata().map(|m| m.len()).unwrap_or(0);
        let columns = csv_columns(&path).unwrap_or_default();
        out.push(MlDatasetEntry {
            name,
            path: path.to_string_lossy().to_string(),
            size_bytes,
            columns,
        });
    }
    Ok(out)
}

#[tauri::command]
pub async fn ml_studio_prepare_dataset(
    root: String,
    csv_name: String,
    target_column: String,
    val_ratio: Option<f64>,
) -> Result<serde_json::Value, String> {
    let ml = ml_root(&root);
    std::fs::create_dir_all(ml.join("data")).map_err(|e| e.to_string())?;
    let src = if csv_name.contains('/') || csv_name.contains('\\') {
        PathBuf::from(&csv_name)
    } else {
        ml.join("data").join(&csv_name)
    };
    if !src.exists() {
        return Err(format!("CSV not found: {}", src.display()));
    }
    let cols = csv_columns(&src)?;
    if !cols.iter().any(|c| c == &target_column) {
        return Err(format!("Target column '{target_column}' not in CSV"));
    }

    let ratio = val_ratio.unwrap_or(0.2).clamp(0.05, 0.5);
    let py = resolve_python()?;
    let script = format!(
        r#"
import json, pandas as pd
from pathlib import Path
src = Path(r"{src}")
df = pd.read_csv(src)
if len(df) < 4:
    raise SystemExit("Need at least 4 rows")
df = df.sample(frac=1, random_state=42).reset_index(drop=True)
n = max(1, int(len(df) * {ratio}))
val = df.iloc[:n]
train = df.iloc[n:]
data = Path(r"{data}")
train.to_csv(data / "train.csv", index=False)
val.to_csv(data / "val.csv", index=False)
manifest = {{
    "source_csv": str(src),
    "train_csv": ".hades/ml/data/train.csv",
    "val_csv": ".hades/ml/data/val.csv",
    "target_column": "{target}",
    "rows_train": len(train),
    "rows_val": len(val),
}}
Path(r"{manifest}").write_text(json.dumps(manifest, indent=2), encoding="utf-8")
print(json.dumps({{"ok": True, **manifest}}))
"#,
        src = src.display(),
        data = ml.join("data").display(),
        manifest = ml.join("manifest.json").display(),
        ratio = ratio,
        target = target_column.replace('"', ""),
    );
    let out = hidden_command(&py)
        .args(["-c", &script])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    let text = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(text.trim()).map_err(|e| format!("parse prepare output: {e}"))
}


#[tauri::command]
pub async fn ml_studio_train(
    root: String,
    job_manager: State<'_, Arc<JobManager>>,
) -> Result<serde_json::Value, String> {
    let ml = ml_root(&root);
    let manifest_path = ml.join("manifest.json");
    if !manifest_path.exists() {
        return Err("No prepared dataset — run Prepare Data first".to_string());
    }
    let cfg = read_config(&ml.join("config.json"));
    let run_id = format!("run-{}", chrono::Utc::now().timestamp());
    let run_dir = ml.join("runs").join(&run_id);
    std::fs::create_dir_all(&run_dir).map_err(|e| e.to_string())?;

    let mut manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if let Some(obj) = manifest.as_object_mut() {
        obj.insert("run_id".to_string(), serde_json::Value::String(run_id.clone()));
    }
    std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap())
        .map_err(|e| e.to_string())?;

    let py = resolve_python()?;
    let train_script = ml.join("scripts").join("train_classifier.py");
    let script = if train_script.exists() {
        train_script
    } else {
        scripts_dir().join("train_classifier.py")
    };
    if !script.exists() {
        return Err("train_classifier.py missing".to_string());
    }

    let job_id = format!("ml-train-{}", run_id);
    job_manager.register(job_id.clone(), "PyTorch training".to_string()).await;

    let root_clone = root.clone();
    let jobs = job_manager.inner().clone();
    let job_id_spawn = job_id.clone();
    let epochs = cfg.epochs.max(1);

    tokio::task::spawn_blocking(move || {
        let mut child = hidden_command(&py)
            .arg(script.as_os_str())
            .arg(&root_clone)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        let stdout = child.stdout.take();
        let rt = tokio::runtime::Handle::current();
        if let Some(out) = stdout {
            let reader = BufReader::new(out);
            for line in reader.lines().filter_map(|l| l.ok()) {
                if let Some(json) = line.strip_prefix("ML_METRIC:") {
                    if let Ok(row) = serde_json::from_str::<MlEpochMetric>(json) {
                        let pct = ((row.epoch as f32 / epochs as f32) * 100.0) as u8;
                        rt.block_on(jobs.update(
                            &job_id_spawn,
                            pct.min(99),
                            &format!(
                                "epoch {} loss {:.3} acc {:.1}%",
                                row.epoch,
                                row.val_loss,
                                row.val_acc * 100.0
                            ),
                        ));
                    }
                } else if line.contains("epoch=") {
                    if let Some(part) = line.split('/').nth(1) {
                        if let Some(ep) = part.split_whitespace().next() {
                            if let Ok(cur) = ep.parse::<u32>() {
                                let pct = ((cur as f32 / epochs as f32) * 100.0) as u8;
                                rt.block_on(jobs.update(&job_id_spawn, pct.min(99), "training"));
                            }
                        }
                    }
                }
            }
        }
        let status = child.wait().map_err(|e| e.to_string())?;
        rt.block_on(jobs.complete(
            &job_id_spawn,
            status.success(),
            if status.success() { "done" } else { "error" },
        ));
        Ok::<(), String>(())
    });

    Ok(serde_json::json!({ "ok": true, "job_id": job_id, "run_id": run_id }))
}

#[tauri::command]
pub async fn ml_studio_list_runs(root: String) -> Result<Vec<MlRunSummary>, String> {
    let runs_dir = ml_root(&root).join("runs");
    if !runs_dir.exists() {
        return Ok(vec![]);
    }
    let mut runs = Vec::new();
    for entry in std::fs::read_dir(&runs_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            continue;
        }
        let id = entry.file_name().to_string_lossy().to_string();
        let run_path = entry.path();
        let model = run_path.join("model.pt");
        let metrics = run_path.join("metrics.json");
        let mut val_acc = None;
        if metrics.exists() {
            if let Ok(text) = std::fs::read_to_string(&metrics) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                    val_acc = v.get("val_acc").and_then(|x| x.as_f64());
                }
            }
        }
        let created_at = entry
            .metadata()
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        runs.push(MlRunSummary {
            id,
            model_path: model.exists().then(|| model.to_string_lossy().to_string()),
            metrics_path: metrics.exists().then(|| metrics.to_string_lossy().to_string()),
            val_acc,
            created_at,
        });
    }
    runs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(runs)
}

#[tauri::command]
pub async fn ml_studio_get_run_metrics(root: String, run_id: String) -> Result<MlRunMetrics, String> {
    let run_dir = ml_root(&root).join("runs").join(&run_id);
    let live = run_dir.join("live_metrics.json");
    let final_m = run_dir.join("metrics.json");
    let path = if live.exists() { live } else { final_m };
    if !path.exists() {
        return Err(format!("No metrics yet for run {run_id}"));
    }
    let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| format!("parse metrics: {e}"))
}

#[tauri::command]
pub async fn ml_studio_get_active_run(root: String) -> Result<Option<String>, String> {
    let manifest_path = ml_root(&root).join("manifest.json");
    if !manifest_path.exists() {
        return Ok(None);
    }
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    Ok(manifest
        .get("run_id")
        .and_then(|v| v.as_str())
        .map(String::from))
}

#[tauri::command]
pub async fn ml_studio_infer(
    root: String,
    run_id: String,
    input: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let model_path = ml_root(&root).join("runs").join(&run_id).join("model.pt");
    if !model_path.exists() {
        return Err(format!("Model not found for run {run_id}"));
    }
    let py = resolve_python()?;
    let infer_script = ml_root(&root).join("scripts").join("inference.py");
    let script = if infer_script.exists() {
        infer_script
    } else {
        scripts_dir().join("inference.py")
    };
    let payload = serde_json::to_string(&input).map_err(|e| e.to_string())?;
    let out = hidden_command(&py)
        .arg(script.as_os_str())
        .arg(model_path.as_os_str())
        .arg(&payload)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    let text = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(text.trim()).map_err(|e| format!("parse inference: {e}"))
}

#[tauri::command]
pub async fn ml_studio_install_deps() -> Result<serde_json::Value, String> {
    let py = resolve_python()?;
    let out = hidden_command(&py)
        .args(["-m", "pip", "install", "pandas"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "ok": out.status.success(),
        "stdout": String::from_utf8_lossy(&out.stdout),
        "stderr": String::from_utf8_lossy(&out.stderr),
    }))
}
