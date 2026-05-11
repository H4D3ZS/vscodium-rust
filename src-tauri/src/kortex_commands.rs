use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

fn find_json_end(bytes: &[u8]) -> Option<usize> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escaped = false;

    for (i, &b) in bytes.iter().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        match b {
            b'\\' => escaped = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => brace_count += 1,
            b'}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
    }
    None
}

#[tauri::command]
pub async fn load_kortex_memory(path: String) -> Result<Vec<f32>, String> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Ok(vec![0.0; 1536]);
    }

    let bytes = fs::read(&path_buf).map_err(|e| e.to_string())?;
    
    let header_end = find_json_end(&bytes).ok_or_else(|| "Invalid .aim format: No JSON header found".to_string())?;

    if header_end + (1536 * 4) > bytes.len() {
        return Ok(vec![0.0; 1536]);
    }

    let tensor_bytes = &bytes[header_end..header_end + (1536 * 4)];
    let mut tensor = Vec::with_capacity(1536);
    for chunk in tensor_bytes.chunks_exact(4) {
        let val = f32::from_le_bytes(chunk.try_into().unwrap());
        tensor.push(val);
    }

    Ok(tensor)
}

#[tauri::command]
pub async fn load_kortex_metadata(path: String) -> Result<Value, String> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Ok(json!({}));
    }

    let bytes = fs::read(&path_buf).map_err(|e| e.to_string())?;
    
    let header_end = find_json_end(&bytes).ok_or_else(|| "Invalid .aim format: No JSON header found".to_string())?;

    let header_json = String::from_utf8_lossy(&bytes[..header_end]);
    let metadata: Value = serde_json::from_str(&header_json).map_err(|e| e.to_string())?;
    
    Ok(metadata)
}

#[tauri::command]
pub async fn save_kortex_memory(path: String, gist_token: Vec<f32>, metadata: Value) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    if let Some(parent) = path_buf.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    // Ensure we have a valid JSON header
    let header_json = serde_json::to_string(&metadata).map_err(|e| e.to_string())?;
    
    let mut bytes = header_json.into_bytes();
    
    for val in gist_token {
        bytes.extend_from_slice(&val.to_le_bytes());
    }

    fs::write(&path_buf, bytes).map_err(|e| e.to_string())?;
    println!("[KORTEX] Saved consciousness to {}", path);
    
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────
//  Kortex telemetry .aim VFS hookup
// ─────────────────────────────────────────────────────────────────────────
//
// The `.aim` files are the durable side of the neural VFS: gist tokens live
// there, and now so do the inference telemetry traces (tok/s, prefill ms,
// cache hit rate, model identity) that the IDE just started collecting.
//
// File format: same shape as the gist .aim → a JSON header that describes
// the stream + the raw byte payload. Here the payload is a tightly packed
// ring buffer of `TelemetrySample`s, JSON-encoded as a single array; the
// gist .aim's binary tensor payload is unrelated, so we keep them in
// separate files (`telemetry.aim`, `airi_memory.aim`).

/// One inference completion as recorded by the frontend services. Mirrors
/// `CompletionSample` in `src/kortex/throughput.ts` so the disk format is a
/// trivial 1:1 of what's already in the renderer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySample {
    pub wall_clock_ms: u64,
    #[serde(default)]
    pub prefill_ms: Option<u64>,
    pub output_tokens: u64,
    pub input_tokens: u64,
    pub backend: String,
    pub cache_hit: bool,
    pub tokens_skipped: u64,
    #[serde(default)]
    pub model_id: Option<String>,
    pub ts_unix_ms: u64,
}

/// Header for `telemetry.aim`. Includes a schema version so v1 readers can
/// gracefully degrade when v2 adds fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelemetryAimHeader {
    /// "kortex.telemetry/v1"
    schema: String,
    /// Max number of samples we'll keep in the ring buffer on disk. Older
    /// samples get evicted FIFO. Default 4096 ≈ days of normal use at a few
    /// requests per minute.
    capacity: usize,
    /// Lifetime tally — survives ring-buffer eviction so the UI can show
    /// "you've routed 1.4M tokens through Kortex" even after old samples
    /// roll off.
    lifetime_samples: u64,
    lifetime_output_tokens: u64,
    lifetime_input_tokens: u64,
    lifetime_tokens_skipped: u64,
    lifetime_cache_hits: u64,
    /// Tracks the most recent KDKVC model identity stamp, so the user can
    /// confirm "my .aim VFS believes the cache is bound to this model" even
    /// when the proxy isn't running right now.
    #[serde(default)]
    last_model_id: Option<String>,
    #[serde(default)]
    last_quant_signature: Option<String>,
    #[serde(default)]
    last_tokenizer_hash: Option<String>,
}

impl Default for TelemetryAimHeader {
    fn default() -> Self {
        Self {
            schema: "kortex.telemetry/v1".to_string(),
            capacity: 4096,
            lifetime_samples: 0,
            lifetime_output_tokens: 0,
            lifetime_input_tokens: 0,
            lifetime_tokens_skipped: 0,
            lifetime_cache_hits: 0,
            last_model_id: None,
            last_quant_signature: None,
            last_tokenizer_hash: None,
        }
    }
}

/// In-memory mirror of the disk file. We hold it in a global Mutex so the
/// recordCompletion path doesn't have to do disk IO on every token round —
/// instead we flush opportunistically (every N samples, on stop, on shutdown).
struct TelemetryState {
    header: TelemetryAimHeader,
    samples: VecDeque<TelemetrySample>,
    /// Number of samples appended since the last successful disk flush. Used
    /// to decide when to rewrite the file.
    dirty_count: u32,
    /// Absolute path of `telemetry.aim` once it's been resolved.
    path: Option<PathBuf>,
}

impl TelemetryState {
    fn new() -> Self {
        Self {
            header: TelemetryAimHeader::default(),
            samples: VecDeque::with_capacity(4096),
            dirty_count: 0,
            path: None,
        }
    }
}

static TELEMETRY: OnceLock<Mutex<TelemetryState>> = OnceLock::new();

fn telemetry_state() -> &'static Mutex<TelemetryState> {
    TELEMETRY.get_or_init(|| Mutex::new(TelemetryState::new()))
}

/// Resolve the default telemetry.aim path. Lives next to the gist .aim
/// (`<userprofile>/.kortex/telemetry.aim`) so a single directory contains
/// the whole neural VFS state.
fn default_telemetry_aim_path() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".kortex").join("telemetry.aim")
}

/// Serialise the header + sample ring into the .aim envelope format used
/// elsewhere (JSON header followed by payload). Payload here is a UTF-8 JSON
/// array of samples — easy to debug, and at 4096 entries × ~150 B/entry
/// (~600 KB) it's well under the file-size limits we care about.
fn write_telemetry_aim(path: &Path, state: &TelemetryState) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    let header_bytes = serde_json::to_vec(&state.header).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    let samples_vec: Vec<&TelemetrySample> = state.samples.iter().collect();
    let payload_bytes = serde_json::to_vec(&samples_vec).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;

    // Two-step atomic write: write to .tmp, fsync, rename. Same pattern as
    // KDKVC's index files; protects against partial writes if the process
    // crashes mid-flush.
    let tmp = path.with_extension("aim.tmp");
    {
        use std::io::Write;
        let mut f = fs::File::create(&tmp)?;
        f.write_all(&header_bytes)?;
        f.write_all(&payload_bytes)?;
        let _ = f.sync_all();
    }
    if path.exists() {
        let _ = fs::remove_file(path);
    }
    fs::rename(&tmp, path)
}

fn read_telemetry_aim(path: &Path) -> Result<(TelemetryAimHeader, Vec<TelemetrySample>), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let header_end = find_json_end(&bytes)
        .ok_or_else(|| "Invalid .aim format: No JSON header found".to_string())?;
    let header: TelemetryAimHeader = serde_json::from_slice(&bytes[..header_end])
        .map_err(|e| format!("Failed to parse telemetry.aim header: {}", e))?;
    let samples: Vec<TelemetrySample> = if header_end >= bytes.len() {
        Vec::new()
    } else {
        serde_json::from_slice(&bytes[header_end..])
            .map_err(|e| format!("Failed to parse telemetry.aim samples: {}", e))?
    };
    Ok((header, samples))
}

/// Load `telemetry.aim` (or the supplied path) from disk into the in-memory
/// state. Safe to call from frontend boot; missing files yield a clean empty
/// state without erroring.
#[tauri::command]
pub async fn aim_load_telemetry(path: Option<String>) -> Result<Value, String> {
    let path_buf = path.map(PathBuf::from).unwrap_or_else(default_telemetry_aim_path);
    let mut guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    guard.path = Some(path_buf.clone());

    if !path_buf.exists() {
        guard.samples.clear();
        guard.header = TelemetryAimHeader::default();
        guard.dirty_count = 0;
        return Ok(serde_json::to_value(&*guard).unwrap_or_else(|_| json!({})));
    }

    let (header, samples) = read_telemetry_aim(&path_buf)?;
    guard.header = header;
    guard.samples = samples.into_iter().collect();
    guard.dirty_count = 0;
    Ok(serde_json::to_value(&*guard).unwrap_or_else(|_| json!({})))
}

/// Append a single sample. Lifetime counters are bumped unconditionally;
/// the ring buffer evicts the oldest entry once it overflows `capacity`.
/// Every 16th append triggers a disk flush so we don't lose more than that
/// many samples on a hard crash.
#[tauri::command]
pub async fn aim_append_telemetry(sample: TelemetrySample) -> Result<Value, String> {
    let mut guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    guard.header.lifetime_samples = guard.header.lifetime_samples.saturating_add(1);
    guard.header.lifetime_output_tokens =
        guard.header.lifetime_output_tokens.saturating_add(sample.output_tokens);
    guard.header.lifetime_input_tokens =
        guard.header.lifetime_input_tokens.saturating_add(sample.input_tokens);
    guard.header.lifetime_tokens_skipped =
        guard.header.lifetime_tokens_skipped.saturating_add(sample.tokens_skipped);
    if sample.cache_hit {
        guard.header.lifetime_cache_hits = guard.header.lifetime_cache_hits.saturating_add(1);
    }

    let cap = guard.header.capacity.max(1);
    guard.samples.push_back(sample);
    while guard.samples.len() > cap {
        guard.samples.pop_front();
    }
    guard.dirty_count = guard.dirty_count.saturating_add(1);

    if guard.dirty_count >= 16 {
        let path = guard.path.clone().unwrap_or_else(default_telemetry_aim_path);
        // Best-effort flush; failures must not break the inference path
        // that called us. The next flush attempt will catch up.
        let _ = write_telemetry_aim(&path, &guard);
        guard.dirty_count = 0;
        guard.path = Some(path);
    }
    Ok(serde_json::to_value(&*guard).unwrap_or_else(|_| json!({})))
}

/// Update the cached "this is the model the KDKVC proxy is currently bound
/// to" stamp. Called by `kortex_kvcache_start` so even when the proxy is
/// down, the IDE can read the .aim VFS and know what was last loaded.
#[tauri::command]
pub async fn aim_set_bound_model(
    model_id: String,
    quant_signature: Option<String>,
    tokenizer_hash: Option<String>,
) -> Result<(), String> {
    let mut guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    guard.header.last_model_id = if model_id.is_empty() { None } else { Some(model_id) };
    guard.header.last_quant_signature = quant_signature;
    guard.header.last_tokenizer_hash = tokenizer_hash;
    guard.dirty_count = guard.dirty_count.saturating_add(1);
    let path = guard.path.clone().unwrap_or_else(default_telemetry_aim_path);
    let _ = write_telemetry_aim(&path, &guard);
    guard.dirty_count = 0;
    guard.path = Some(path);
    Ok(())
}

/// Read-only snapshot of the live state. Frontend uses this to render the
/// "Neural VFS state" panel in `KortexInferencePanel.tsx`.
#[tauri::command]
pub async fn aim_telemetry_snapshot() -> Result<Value, String> {
    let guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(&*guard).unwrap_or_else(|_| json!({})))
}

/// Force a flush. Wired into `kortex_kvcache_stop` and the IDE shutdown hook
/// so we never lose unflushed samples on a clean exit.
#[tauri::command]
pub async fn aim_flush_telemetry() -> Result<(), String> {
    let guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    let path = guard.path.clone().unwrap_or_else(default_telemetry_aim_path);
    write_telemetry_aim(&path, &guard).map_err(|e| e.to_string())?;
    Ok(())
}

/// Wipe the ring buffer but keep lifetime counters. Useful when the user
/// wants to start a fresh measurement window without forgetting their
/// historical totals.
#[tauri::command]
pub async fn aim_clear_telemetry_samples() -> Result<(), String> {
    let mut guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    guard.samples.clear();
    guard.dirty_count = guard.dirty_count.saturating_add(1);
    let path = guard.path.clone().unwrap_or_else(default_telemetry_aim_path);
    let _ = write_telemetry_aim(&path, &guard);
    guard.dirty_count = 0;
    guard.path = Some(path);
    Ok(())
}

// Manual Serialize impl so the snapshot includes both header fields and the
// current ring buffer in a single object the frontend can consume directly.
impl Serialize for TelemetryState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TelemetryState", 11)?;
        s.serialize_field("schema", &self.header.schema)?;
        s.serialize_field("capacity", &self.header.capacity)?;
        s.serialize_field("lifetime_samples", &self.header.lifetime_samples)?;
        s.serialize_field("lifetime_output_tokens", &self.header.lifetime_output_tokens)?;
        s.serialize_field("lifetime_input_tokens", &self.header.lifetime_input_tokens)?;
        s.serialize_field("lifetime_tokens_skipped", &self.header.lifetime_tokens_skipped)?;
        s.serialize_field("lifetime_cache_hits", &self.header.lifetime_cache_hits)?;
        s.serialize_field("last_model_id", &self.header.last_model_id)?;
        s.serialize_field("last_quant_signature", &self.header.last_quant_signature)?;
        s.serialize_field("last_tokenizer_hash", &self.header.last_tokenizer_hash)?;
        let samples_vec: Vec<&TelemetrySample> = self.samples.iter().collect();
        s.serialize_field("samples", &samples_vec)?;
        s.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn make_sample(ts: u64, out: u64, hit: bool) -> TelemetrySample {
        TelemetrySample {
            wall_clock_ms: 1000,
            prefill_ms: Some(100),
            output_tokens: out,
            input_tokens: 50,
            backend: "llama.cpp".to_string(),
            cache_hit: hit,
            tokens_skipped: if hit { 200 } else { 0 },
            model_id: Some("test.gguf".to_string()),
            ts_unix_ms: ts,
        }
    }

    #[test]
    fn round_trips_through_disk() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("telemetry.aim");
        let mut state = TelemetryState::new();
        state.samples.push_back(make_sample(1, 10, false));
        state.samples.push_back(make_sample(2, 20, true));
        state.header.lifetime_samples = 2;
        state.header.lifetime_cache_hits = 1;
        state.header.last_model_id = Some("qwen3-coder.Q4_K_M.gguf".to_string());

        write_telemetry_aim(&path, &state).unwrap();
        let (header, samples) = read_telemetry_aim(&path).unwrap();
        assert_eq!(header.lifetime_samples, 2);
        assert_eq!(header.lifetime_cache_hits, 1);
        assert_eq!(header.last_model_id.as_deref(), Some("qwen3-coder.Q4_K_M.gguf"));
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].output_tokens, 10);
        assert_eq!(samples[1].cache_hit, true);
    }

    #[test]
    fn missing_file_yields_clean_state() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nope.aim");
        assert!(!path.exists());
        let result = read_telemetry_aim(&path);
        assert!(result.is_err()); // missing file is an error from the reader; the command wraps this.
    }

    #[test]
    fn corrupt_header_returns_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("bad.aim");
        fs::write(&path, b"not even json").unwrap();
        let result = read_telemetry_aim(&path);
        assert!(result.is_err());
    }

    #[test]
    fn write_is_atomic_when_target_exists() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("telemetry.aim");
        let mut state = TelemetryState::new();
        state.samples.push_back(make_sample(1, 10, false));
        write_telemetry_aim(&path, &state).unwrap();

        // Re-write with a different payload; the .tmp must not linger.
        state.samples.push_back(make_sample(2, 99, true));
        write_telemetry_aim(&path, &state).unwrap();

        let tmp = path.with_extension("aim.tmp");
        assert!(!tmp.exists(), "tmp file should be renamed away");

        let (_, samples) = read_telemetry_aim(&path).unwrap();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[1].output_tokens, 99);
    }
}
