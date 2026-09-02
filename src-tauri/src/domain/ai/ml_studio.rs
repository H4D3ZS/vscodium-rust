//! ML Studio — dataset prep, training jobs, inference for `.hades/ml/` projects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, LazyLock, Mutex};

use tauri::State;

use crate::jobs::JobManager;
use crate::process_ext::hidden_command;
use crate::pytorch_commands;

static ML_TRAIN_PIDS: LazyLock<Mutex<HashMap<String, u32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlStudioConfig {
    pub epochs: u32,
    pub learning_rate: f64,
    pub hidden_size: u32,
    pub val_ratio: f64,
    pub embed_model: String,
    #[serde(default = "default_early_stop")]
    pub early_stop_patience: u32,
    #[serde(default = "default_model_template")]
    pub model_template: String,
    #[serde(default = "default_model_source")]
    pub model_source: String,
}

fn default_early_stop() -> u32 {
    3
}
fn default_model_template() -> String {
    "tabular_mlp".to_string()
}
fn default_model_source() -> String {
    "builtin".to_string()
}

impl Default for MlStudioConfig {
    fn default() -> Self {
        Self {
            epochs: 20,
            learning_rate: 0.001,
            hidden_size: 64,
            val_ratio: 0.2,
            embed_model: "nomic-embed-text".to_string(),
            early_stop_patience: 3,
            model_template: "tabular_mlp".to_string(),
            model_source: "builtin".to_string(),
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
pub struct MlConfusionMatrix {
    pub labels: Vec<String>,
    pub matrix: Vec<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlRunMetrics {
    pub status: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
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
    #[serde(default)]
    pub early_stopped: Option<bool>,
    #[serde(default)]
    pub val_acc: Option<f64>,
    #[serde(default)]
    pub val_loss: Option<f64>,
    #[serde(default)]
    pub confusion_matrix: Option<MlConfusionMatrix>,
    #[serde(default)]
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
    for name in [
        "train_classifier.py",
        "train_vision.py",
        "inference.py",
        "ml_toolkit.py",
    ] {
        let src = scripts_dir().join(name);
        if src.exists() {
            std::fs::copy(&src, dst.join(name)).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn resolve_toolkit(root: &str) -> Result<PathBuf, String> {
    let local = ml_root(root).join("scripts").join("ml_toolkit.py");
    if local.exists() {
        return Ok(local);
    }
    let bundled = scripts_dir().join("ml_toolkit.py");
    if bundled.exists() {
        return Ok(bundled);
    }
    Err("ml_toolkit.py missing".to_string())
}

fn run_toolkit_at(script: &Path, args: &[&str]) -> Result<serde_json::Value, String> {
    let py = resolve_python()?;
    let out = hidden_command(&py)
        .arg("-u")
        .arg(script)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    let text = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(text.trim()).map_err(|e| format!("parse toolkit output: {e}"))
}

fn toolkit(root: &str, args: &[&str]) -> Result<serde_json::Value, String> {
    let script = resolve_toolkit(root)?;
    run_toolkit_at(&script, args)
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
            "# HADES ML Studio\n\nA friend of AI engineers — train, experiment, and export without leaving the IDE.\n\nPlace CSV datasets in `data/` or images in `data/images/<class>/`. Use PyTorch ML Studio to prepare, train, compare runs, and infer.\n",
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
    let cfg = read_config(&ml.join("config.json"));
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
    "embed_model": "{embed_model}",
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
        embed_model = cfg.embed_model.replace('"', ""),
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
    resume_run_id: Option<String>,
    worker_id: Option<String>,
    job_manager: State<'_, Arc<JobManager>>,
) -> Result<serde_json::Value, String> {
    let ml = ml_root(&root);
    let manifest_path = ml.join("manifest.json");
    if !manifest_path.exists() {
        return Err("No prepared dataset — run Prepare Data first".to_string());
    }
    let cfg = read_config(&ml.join("config.json"));
    let run_id = if let Some(ref rid) = resume_run_id {
        let ckpt = ml.join("runs").join(rid).join("checkpoint.pt");
        if !ckpt.exists() {
            return Err(format!("No checkpoint.pt for run {rid} — train from scratch instead"));
        }
        rid.clone()
    } else {
        format!("run-{}", chrono::Utc::now().timestamp())
    };
    let run_dir = ml.join("runs").join(&run_id);
    std::fs::create_dir_all(&run_dir).map_err(|e| e.to_string())?;

    let mut manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if let Some(obj) = manifest.as_object_mut() {
        obj.insert("run_id".to_string(), serde_json::Value::String(run_id.clone()));
        if resume_run_id.is_some() {
            obj.insert(
                "resume_from".to_string(),
                serde_json::Value::String(run_id.clone()),
            );
        } else {
            obj.remove("resume_from");
        }
    }
    std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap())
        .map_err(|e| e.to_string())?;

    let py = resolve_python()?;
    let image_dir = ml.join("data").join("images");
    let use_vision = image_dir.is_dir()
        && std::fs::read_dir(&image_dir)
            .map(|d| d.filter_map(|e| e.ok()).any(|e| e.path().is_dir()))
            .unwrap_or(false);
    let script_name = if use_vision { "train_vision.py" } else { "train_classifier.py" };
    let train_script = ml.join("scripts").join(script_name);
    let script = if train_script.exists() {
        train_script
    } else {
        scripts_dir().join(script_name)
    };
    if !script.exists() {
        return Err("train_classifier.py missing".to_string());
    }

    let epochs = cfg.epochs.max(1);

    if let Some(ref wid) = worker_id {
        return ml_studio_train_remote(
            &root,
            &run_id,
            wid,
            &script,
            epochs,
            (*job_manager).clone(),
        )
        .await;
    }

    let job_id = format!("ml-train-{}", run_id);
    job_manager.register(job_id.clone(), "PyTorch training".to_string()).await;

    let root_clone = root.clone();
    let jobs = (*job_manager).clone();
    let job_id_spawn = job_id.clone();
    let run_id_spawn = run_id.clone();

    tokio::task::spawn_blocking(move || {
        let mut child = hidden_command(&py)
            .arg(script.as_os_str())
            .arg(&root_clone)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        let pid = child.id();
        if let Ok(mut pids) = ML_TRAIN_PIDS.lock() {
            pids.insert(run_id_spawn.clone(), pid);
        }

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
        if let Ok(mut pids) = ML_TRAIN_PIDS.lock() {
            pids.remove(&run_id_spawn);
        }
        rt.block_on(jobs.complete(
            &job_id_spawn,
            status.success(),
            if status.success() { "done" } else { "error" },
        ));
        Ok::<(), String>(())
    });

    Ok(serde_json::json!({
        "ok": true,
        "job_id": job_id,
        "run_id": run_id,
        "resumed": resume_run_id.is_some(),
    }))
}

#[tauri::command]
pub async fn ml_studio_cancel_train(root: String, run_id: String) -> Result<serde_json::Value, String> {
    let pid = ML_TRAIN_PIDS
        .lock()
        .map_err(|e| e.to_string())?
        .get(&run_id)
        .copied();
    if let Some(pid) = pid {
        #[cfg(windows)]
        {
            let _ = hidden_command("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .output();
        }
        #[cfg(not(windows))]
        {
            let _ = hidden_command("kill")
                .args(["-9", &pid.to_string()])
                .output();
        }
        if let Ok(mut pids) = ML_TRAIN_PIDS.lock() {
            pids.remove(&run_id);
        }
        let live_path = ml_root(&root).join("runs").join(&run_id).join("live_metrics.json");
        if live_path.exists() {
            if let Ok(text) = std::fs::read_to_string(&live_path) {
                if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(obj) = v.as_object_mut() {
                        obj.insert("status".to_string(), serde_json::json!("cancelled"));
                    }
                    let _ = std::fs::write(&live_path, serde_json::to_string_pretty(&v).unwrap_or_default());
                }
            }
        }
        Ok(serde_json::json!({ "ok": true, "cancelled": true }))
    } else {
        Err(format!("No active training process for run {run_id}"))
    }
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
        .args([
            "-m",
            "pip",
            "install",
            "pandas",
            "scikit-learn",
            "onnx",
            "onnxscript",
            "optuna",
            "pillow",
            "timm",
            "transformers",
            "torchvision",
        ])
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

#[tauri::command]
pub async fn ml_studio_dataset_stats(
    root: String,
    csv_name: String,
    target_column: Option<String>,
) -> Result<serde_json::Value, String> {
    let csv = if csv_name.contains('/') || csv_name.contains('\\') {
        PathBuf::from(&csv_name)
    } else {
        ml_root(&root).join("data").join(&csv_name)
    };
    let csv_str = csv.to_string_lossy().into_owned();
    let mut args: Vec<String> = vec![
        "dataset_stats".into(),
        "--csv".into(),
        csv_str,
    ];
    if let Some(ref t) = target_column {
        args.push("--target".into());
        args.push(t.clone());
    }
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    toolkit(&root, &arg_refs)
}

#[tauri::command]
pub async fn ml_studio_model_summary(root: String, run_id: String) -> Result<serde_json::Value, String> {
    let model = ml_root(&root).join("runs").join(&run_id).join("model.pt");
    if !model.exists() {
        return Err(format!("No model for run {run_id}"));
    }
    let model_str = model.to_string_lossy().into_owned();
    toolkit(
        &root,
        &["model_summary", "--model", &model_str],
    )
}

#[tauri::command]
pub async fn ml_studio_export_model(
    root: String,
    run_id: String,
    format: String,
) -> Result<serde_json::Value, String> {
    let model = ml_root(&root).join("runs").join(&run_id).join("model.pt");
    let model_str = model.to_string_lossy().into_owned();
    toolkit(
        &root,
        &["export", "--model", &model_str, "--format", &format],
    )
}

#[tauri::command]
pub async fn ml_studio_pretrained_gallery(root: String) -> Result<serde_json::Value, String> {
    toolkit(&root, &["gallery"])
}

#[tauri::command]
pub async fn ml_studio_hpo(
    root: String,
    mode: String,
    trials: u32,
) -> Result<serde_json::Value, String> {
    toolkit(
        &root,
        &[
            "hpo",
            "--root",
            &root,
            "--mode",
            &mode,
            "--trials",
            &trials.to_string(),
        ],
    )
}

#[tauri::command]
pub async fn ml_studio_lr_finder(root: String, steps: u32) -> Result<serde_json::Value, String> {
    toolkit(
        &root,
        &["lr_finder", "--root", &root, "--steps", &steps.to_string()],
    )
}

#[tauri::command]
pub async fn ml_studio_grad_check(root: String, run_id: String) -> Result<serde_json::Value, String> {
    let model = ml_root(&root).join("runs").join(&run_id).join("model.pt");
    let model_str = model.to_string_lossy().into_owned();
    toolkit(&root, &["grad_check", "--model", &model_str])
}

#[tauri::command]
pub async fn ml_studio_benchmark(
    root: String,
    run_id: String,
    iterations: u32,
) -> Result<serde_json::Value, String> {
    let model = ml_root(&root).join("runs").join(&run_id).join("model.pt");
    let model_str = model.to_string_lossy().into_owned();
    let iters = iterations.to_string();
    toolkit(
        &root,
        &["benchmark", "--model", &model_str, "--iterations", &iters],
    )
}

#[tauri::command]
pub async fn ml_studio_list_experiments(root: String) -> Result<Vec<serde_json::Value>, String> {
    let runs_dir = ml_root(&root).join("runs");
    if !runs_dir.exists() {
        return Ok(vec![]);
    }
    let cfg = read_config(&ml_root(&root).join("config.json"));
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&runs_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            continue;
        }
        let id = entry.file_name().to_string_lossy().to_string();
        let run_path = entry.path();
        let mut row = serde_json::json!({ "id": id });
        for name in ["experiment.json", "metrics.json", "config_snapshot.json"] {
            let p = run_path.join(name);
            if p.exists() {
                if let Ok(text) = std::fs::read_to_string(&p) {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                        row[name.trim_end_matches(".json")] = v;
                    }
                }
            }
        }
        if row.get("metrics").is_none() {
            let mp = run_path.join("metrics.json");
            if mp.exists() {
                if let Ok(text) = std::fs::read_to_string(&mp) {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                        row["metrics"] = v.clone();
                        row["val_acc"] = v.get("val_acc").cloned().unwrap_or_default();
                    }
                }
            }
        }
        if !run_path.join("config_snapshot.json").exists() {
            let snap = serde_json::to_string_pretty(&cfg).map_err(|e| e.to_string())?;
            let _ = std::fs::write(run_path.join("config_snapshot.json"), snap);
        }
        out.push(row);
    }
    out.sort_by(|a, b| {
        b.get("experiment")
            .and_then(|e| e.get("created_at"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            .cmp(
                &a.get("experiment")
                    .and_then(|e| e.get("created_at"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0),
            )
    });
    Ok(out)
}

#[tauri::command]
pub async fn ml_studio_export_report(root: String, run_id: String) -> Result<String, String> {
    let run_dir = ml_root(&root).join("runs").join(&run_id);
    let metrics_path = run_dir.join("metrics.json");
    let exp_path = run_dir.join("experiment.json");
    let metrics: serde_json::Value = if metrics_path.exists() {
        serde_json::from_str(&std::fs::read_to_string(&metrics_path).map_err(|e| e.to_string())?)
            .unwrap_or_default()
    } else {
        serde_json::json!({})
    };
    let experiment: serde_json::Value = if exp_path.exists() {
        serde_json::from_str(&std::fs::read_to_string(&exp_path).map_err(|e| e.to_string())?)
            .unwrap_or_default()
    } else {
        serde_json::json!({})
    };
    let md = format!(
        "# ML Experiment Report — {run_id}\n\n\
         ## Summary\n\
         - Val accuracy: {}\n\
         - Best epoch: {}\n\
         - Git commit: {}\n\
         - Device: {}\n\n\
         ## Config\n```json\n{}\n```\n\n\
         ## Confusion matrix\n```json\n{}\n```\n",
        metrics.get("val_acc").and_then(|v| v.as_f64()).map(|v| format!("{v:.4}")).unwrap_or_else(|| "n/a".into()),
        metrics.get("best_epoch").map(|v| v.to_string()).unwrap_or_else(|| "?".into()),
        experiment.get("git_hash").and_then(|v| v.as_str()).unwrap_or("unknown"),
        metrics.get("device").and_then(|v| v.as_str()).unwrap_or("unknown"),
        serde_json::to_string_pretty(&experiment.get("config").unwrap_or(&serde_json::json!({}))).unwrap_or_default(),
        serde_json::to_string_pretty(&metrics.get("confusion_matrix").unwrap_or(&serde_json::json!({}))).unwrap_or_default(),
    );
    let report_path = run_dir.join("report.md");
    std::fs::write(&report_path, &md).map_err(|e| e.to_string())?;
    Ok(md)
}

#[tauri::command]
pub async fn ml_studio_model_graph(root: String, run_id: String) -> Result<serde_json::Value, String> {
    let model = ml_root(&root).join("runs").join(&run_id).join("model.pt");
    if !model.exists() {
        return Err(format!("No model for run {run_id}"));
    }
    let model_str = model.to_string_lossy().into_owned();
    toolkit(&root, &["model_graph", "--model", &model_str])
}

#[tauri::command]
pub async fn ml_studio_augment_preview(root: String, samples: u32) -> Result<serde_json::Value, String> {
    toolkit(
        &root,
        &[
            "augment_preview",
            "--root",
            &root,
            "--samples",
            &samples.to_string(),
        ],
    )
}

#[tauri::command]
pub async fn ml_studio_load_pretrained(
    root: String,
    model_id: String,
    source: String,
) -> Result<serde_json::Value, String> {
    toolkit(
        &root,
        &[
            "load_pretrained",
            "--root",
            &root,
            "--model-id",
            &model_id,
            "--source",
            &source,
        ],
    )
}

#[tauri::command]
pub async fn ml_studio_compare_runs(
    root: String,
    run_ids: Vec<String>,
) -> Result<serde_json::Value, String> {
    let runs = run_ids.join(",");
    toolkit(&root, &["compare_runs", "--root", &root, "--runs", &runs])
}

// ── Remote training workers (TorchStudio-class) ─────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlRemoteWorker {
    pub id: String,
    pub host: String,
    pub user: String,
    pub port: u16,
    pub remote_root: String,
    pub python: String,
    #[serde(default)]
    pub cuda: bool,
}

fn workers_path(root: &str) -> PathBuf {
    ml_root(root).join("workers.json")
}

fn read_workers(root: &str) -> Vec<MlRemoteWorker> {
    let p = workers_path(root);
    if !p.exists() {
        return Vec::new();
    }
    std::fs::read_to_string(&p)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn write_workers(root: &str, workers: &[MlRemoteWorker]) -> Result<(), String> {
    let p = workers_path(root);
    std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(&p, serde_json::to_string_pretty(workers).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ml_studio_list_workers(root: String) -> Result<Vec<MlRemoteWorker>, String> {
    Ok(read_workers(&root))
}

#[tauri::command]
pub fn ml_studio_save_worker(root: String, worker: MlRemoteWorker) -> Result<(), String> {
    let mut workers = read_workers(&root);
    if let Some(i) = workers.iter().position(|w| w.id == worker.id) {
        workers[i] = worker;
    } else {
        workers.push(worker);
    }
    write_workers(&root, &workers)
}

async fn ml_studio_train_remote(
    root: &str,
    run_id: &str,
    worker_id: &str,
    script: &Path,
    epochs: u32,
    job_manager: Arc<JobManager>,
) -> Result<serde_json::Value, String> {
    let worker = read_workers(root)
        .into_iter()
        .find(|w| w.id == worker_id)
        .ok_or_else(|| format!("Worker '{worker_id}' not found — add in ML Studio"))?;

    let job_id = format!("ml-train-remote-{}", run_id);
    job_manager
        .register(job_id.clone(), format!("Remote train on {}", worker.host))
        .await;

    let ml_local = ml_root(root);
    let remote_ml = format!(
        "{}/.hades/ml",
        worker.remote_root.trim_end_matches('/')
    );
    let target = format!("{}@{}", worker.user, worker.host);
    let port = worker.port.to_string();

    // Sync project ML folder to remote worker
    if which::which("rsync").is_ok() {
        let mut cmd = hidden_command("rsync");
        cmd.args([
            "-az",
            "-e",
            &format!(
                "ssh -p {} -o BatchMode=yes -o StrictHostKeyChecking=accept-new",
                port
            ),
            &format!("{}/", ml_local.display()),
            &format!("{target}:{remote_ml}/"),
        ]);
        let out = cmd.output().map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }
    }

    let remote_script = format!(
        "{}/scripts/{}",
        remote_ml,
        script.file_name().and_then(|s| s.to_str()).unwrap_or("train_classifier.py")
    );
    let remote_cmd = format!(
        "cd {} && {} -u {} {}",
        worker.remote_root,
        worker.python,
        remote_script,
        worker.remote_root
    );

    let jobs = job_manager.clone();
    let job_id_spawn = job_id.clone();
    let host = worker.host.clone();
    tokio::task::spawn_blocking(move || {
        let mut cmd = hidden_command("ssh");
        cmd.args([
            "-p",
            &port,
            "-o",
            "BatchMode=yes",
            "-o",
            "ConnectTimeout=30",
            &target,
            &remote_cmd,
        ]);
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        let rt = tokio::runtime::Handle::current();
        rt.block_on(jobs.update(&job_id_spawn, 5, &format!("Training on {host}")));
        match cmd.output() {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    if let Some(json) = line.strip_prefix("ML_METRIC:") {
                        if let Ok(row) = serde_json::from_str::<MlEpochMetric>(json) {
                            let pct = ((row.epoch as f32 / epochs as f32) * 100.0) as u8;
                            rt.block_on(jobs.update(
                                &job_id_spawn,
                                pct.min(99),
                                &format!("epoch {} loss {:.3}", row.epoch, row.val_loss),
                            ));
                        }
                    }
                }
                rt.block_on(jobs.complete(
                    &job_id_spawn,
                    out.status.success(),
                    if out.status.success() {
                        "Remote training complete"
                    } else {
                        "Remote training failed"
                    },
                ));
            }
            Err(e) => {
                rt.block_on(jobs.complete(&job_id_spawn, false, &e.to_string()));
            }
        }
    });

    Ok(serde_json::json!({
        "ok": true,
        "job_id": job_id,
        "run_id": run_id,
        "remote": true,
        "worker": worker_id,
    }))
}

#[tauri::command]
pub async fn ml_studio_export_onnx(root: String, run_id: String) -> Result<serde_json::Value, String> {
    let model = ml_root(&root).join("runs").join(&run_id).join("model.pt");
    if !model.exists() {
        return Err(format!("No model for run {run_id}"));
    }
    toolkit(
        &root,
        &[
            "export_onnx",
            "--model",
            &model.to_string_lossy(),
        ],
    )
}

#[tauri::command]
pub async fn ml_studio_netron_url(root: String, run_id: String) -> Result<serde_json::Value, String> {
    let onnx = ml_studio_export_onnx(root, run_id).await?;
    let path = onnx
        .get("onnx_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    Ok(serde_json::json!({
        "onnx_path": path,
        "netron_embed": format!("https://netron.app/?embed=1"),
        "note": "Open Netron with exported ONNX from the run folder",
    }))
}
