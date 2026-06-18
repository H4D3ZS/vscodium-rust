//! Terminal I/O tools: send data, read output, create, toggle, get state.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use tauri::{Emitter, Manager};
use super::registry::AiTools;

impl AiTools {
    pub(crate) async fn terminal_send_data(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let term_id_opt = args.get("term_id").and_then(|v| v.as_str());
        let data = args.get("data").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing data"))?;
        let state = h.state::<crate::EditorState>();
        let mut writers = state.terminal.writers.lock().await;
        if writers.is_empty() {
            drop(writers);
            h.emit("terminal-create", json!({}))?;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            writers = state.terminal.writers.lock().await;
        }
        let target_id = term_id_opt.map(|s| s.to_string())
            .or_else(|| writers.keys().next().cloned());
        if let Some(id) = target_id {
            if let Some(writer) = writers.get_mut(&id) {
                let payload = if data.ends_with('\n') { data.to_string() } else { format!("{}\n", data) };
                writer.write_all(payload.as_bytes())?;
                writer.flush()?;
                Ok(json!({ "status": "success", "term_id": id, "info": format!("Data sent to terminal '{}'.", id) }))
            } else {
                Err(anyhow!("Terminal '{}' not found in writers", id))
            }
        } else {
            Err(anyhow!("No terminal available"))
        }
    }

    pub(crate) async fn terminal_get_state(&self, _args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let state = h.state::<crate::EditorState>();
        let buffers = state.terminal.buffers.lock().await;
        let ids: Vec<&String> = buffers.keys().collect();
        Ok(json!({ "status": "success", "terminals": ids, "count": ids.len() }))
    }

    pub(crate) async fn terminal_create(&self, _args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let id = format!("term-{}", uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>());
        h.emit("terminal-create", json!({ "id": id }))?;
        Ok(json!({ "status": "success", "term_id": id }))
    }

    pub(crate) async fn terminal_read_output(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let term_id = args.get("term_id").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing term_id"))?;
        let state = h.state::<crate::EditorState>();
        let buffers = state.terminal.buffers.lock().await;
        if let Some(history) = buffers.get(term_id) {
            let output: String = history.join("");
            Ok(json!({ "status": "success", "output": output.chars().take(10000).collect::<String>() }))
        } else {
            Err(anyhow!("Terminal '{}' not found", term_id))
        }
    }

    pub(crate) async fn terminal_toggle(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let term_id = args.get("term_id").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing term_id"))?;
        h.emit("terminal-toggle", json!({ "id": term_id }))?;
        Ok(json!({ "status": "success", "toggled": term_id }))
    }
}
