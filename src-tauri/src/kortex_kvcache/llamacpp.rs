//! Thin async HTTP client for the llama-server endpoints we need from KDKVC.
//!
//! Endpoints (llama.cpp `server` since b3300+):
//!
//!   POST /tokenize                              → token IDs for a string
//!   POST /slots/{id}?action=save  {filename}    → write slot KV to disk
//!   POST /slots/{id}?action=restore {filename}  → load slot KV from disk
//!   POST /slots/{id}?action=erase               → wipe slot KV

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LlamaCppClient {
    pub base_url: String,
    pub slot_id: u32,
    http: Client,
}

impl LlamaCppClient {
    pub fn new(base_url: impl Into<String>, slot_id: u32) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .expect("reqwest client build");
        Self {
            base_url: base_url.into(),
            slot_id,
            http,
        }
    }

    /// Tokenize `text` using the model loaded by llama-server.
    pub async fn tokenize(&self, text: &str) -> Result<Vec<u32>> {
        #[derive(Serialize)]
        struct Req<'a> {
            content: &'a str,
            add_special: bool,
        }
        #[derive(Deserialize)]
        struct Resp {
            tokens: Vec<i64>,
        }
        let url = format!("{}/tokenize", self.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .post(&url)
            .json(&Req {
                content: text,
                add_special: true,
            })
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("/tokenize failed: {} {}", status, body));
        }
        let r: Resp = resp.json().await?;
        Ok(r.tokens.into_iter().map(|x| x as u32).collect())
    }

    /// Save the slot KV state to `<slot_save_path>/<filename>` on the server.
    pub async fn save_slot(&self, filename: &str) -> Result<()> {
        let url = format!(
            "{}/slots/{}?action=save",
            self.base_url.trim_end_matches('/'),
            self.slot_id
        );
        let body = serde_json::json!({ "filename": filename });
        let resp = self.http.post(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let txt = resp.text().await.unwrap_or_default();
            return Err(anyhow!("/slots/{}?save failed: {} {}", self.slot_id, status, txt));
        }
        Ok(())
    }

    /// Restore the slot KV state from `<slot_save_path>/<filename>`.
    pub async fn restore_slot(&self, filename: &str) -> Result<RestoredSlot> {
        let url = format!(
            "{}/slots/{}?action=restore",
            self.base_url.trim_end_matches('/'),
            self.slot_id
        );
        let body = serde_json::json!({ "filename": filename });
        let resp = self.http.post(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let txt = resp.text().await.unwrap_or_default();
            return Err(anyhow!("/slots/{}?restore failed: {} {}", self.slot_id, status, txt));
        }
        let v: serde_json::Value = resp.json().await.unwrap_or(serde_json::json!({}));
        let n_restored = v
            .get("n_restored")
            .or_else(|| v.get("n_tokens"))
            .and_then(|x| x.as_u64())
            .map(|x| x as u32);
        Ok(RestoredSlot {
            n_restored,
            raw: v,
        })
    }

    pub async fn erase_slot(&self) -> Result<()> {
        let url = format!(
            "{}/slots/{}?action=erase",
            self.base_url.trim_end_matches('/'),
            self.slot_id
        );
        let resp = self.http.post(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let txt = resp.text().await.unwrap_or_default();
            return Err(anyhow!("/slots/{}?erase failed: {} {}", self.slot_id, status, txt));
        }
        Ok(())
    }

    /// Quick health probe used by the proxy on startup.
    pub async fn health(&self) -> Result<()> {
        let url = format!("{}/health", self.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .get(&url)
            .timeout(Duration::from_secs(3))
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(anyhow!("/health returned {}", resp.status()));
        }
        Ok(())
    }

    /// Fetch the upstream server's `/props` endpoint to derive a stable model
    /// identity for cache binding. llama-server exposes the loaded model's
    /// path, alias, and other fields here.
    pub async fn props(&self) -> Result<ServerProps> {
        let url = format!("{}/props", self.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(anyhow!("/props returned {}", resp.status()));
        }
        let v: serde_json::Value = resp.json().await?;
        Ok(ServerProps::from_value(&v))
    }
}

/// Subset of llama-server's `/props` response we use to fingerprint the
/// currently-loaded model for KDKVC binding.
///
/// Field names vary slightly across llama.cpp releases (`default_generation_settings`,
/// `model_path`, `model_alias`, `chat_template`, etc.). We pull whatever's
/// present and let [`ServerProps::derive_identity`] do the best-effort hash.
#[derive(Debug, Clone, Default)]
pub struct ServerProps {
    pub model_path: Option<String>,
    pub model_alias: Option<String>,
    pub chat_template: Option<String>,
    pub n_ctx: Option<u32>,
    /// Full raw JSON for forward compatibility — the identity hash uses this
    /// so adding new fields to /props in a future llama.cpp release simply
    /// makes the hash more specific without breaking older caches.
    pub raw: serde_json::Value,
}

impl ServerProps {
    fn from_value(v: &serde_json::Value) -> Self {
        let pick_str = |path: &[&str]| -> Option<String> {
            let mut cur = v;
            for p in path {
                cur = cur.get(p)?;
            }
            cur.as_str().map(|s| s.to_string())
        };
        let pick_u32 = |path: &[&str]| -> Option<u32> {
            let mut cur = v;
            for p in path {
                cur = cur.get(p)?;
            }
            cur.as_u64().map(|x| x as u32)
        };
        Self {
            model_path: pick_str(&["model_path"])
                .or_else(|| pick_str(&["default_generation_settings", "model"])),
            model_alias: pick_str(&["model_alias"]).or_else(|| pick_str(&["model"])),
            chat_template: pick_str(&["chat_template"]),
            n_ctx: pick_u32(&["default_generation_settings", "n_ctx"])
                .or_else(|| pick_u32(&["n_ctx"])),
            raw: v.clone(),
        }
    }

    /// Build a [`ModelIdentity`] from the props. The `model_id` is the basename
    /// of `model_path` (or the alias if path is missing), the `tokenizer_hash`
    /// is a SHA-256 over `model_path + chat_template` (chat template change =
    /// different tokenization boundary in practice), and `quant_signature` is
    /// best-effort parsed out of the model filename (e.g. `Q4_K_M`).
    pub fn derive_identity(&self) -> super::types::ModelIdentity {
        use sha2::{Digest, Sha256};

        let model_id = self
            .model_path
            .as_deref()
            .and_then(|p| std::path::Path::new(p).file_name().map(|x| x.to_string_lossy().into_owned()))
            .or_else(|| self.model_alias.clone())
            .unwrap_or_else(|| "unknown-model".to_string());

        let mut h = Sha256::new();
        if let Some(p) = &self.model_path {
            h.update(p.as_bytes());
        }
        h.update(b"\x00");
        if let Some(t) = &self.chat_template {
            h.update(t.as_bytes());
        }
        let digest = h.finalize();
        let mut tokenizer_hash = String::with_capacity(64);
        for b in digest.iter() {
            tokenizer_hash.push_str(&format!("{:02x}", b));
        }

        let quant_signature = parse_quant_signature(&model_id);

        super::types::ModelIdentity {
            model_id,
            tokenizer_hash,
            quant_signature,
        }
    }
}

/// Heuristic: pull a quant tag out of a GGUF filename. Matches common patterns
/// like `Q4_K_M`, `Q5_K_S`, `IQ2_XXS`, `IQ4_NL`, `Q8_0`, `F16`, `BF16`. Falls
/// back to empty string when nothing recognizable is present, which under
/// `SameModel` policy is fine (quant doesn't gate matches in that mode).
pub fn parse_quant_signature(filename: &str) -> String {
    let upper = filename.to_ascii_uppercase();
    // Order matters: longer matches first.
    const NEEDLES: &[&str] = &[
        "IQ2_XXS", "IQ2_XS", "IQ2_S", "IQ2_M",
        "IQ3_XXS", "IQ3_XS", "IQ3_S", "IQ3_M",
        "IQ4_XS", "IQ4_NL",
        "Q2_K_S", "Q3_K_S", "Q3_K_M", "Q3_K_L",
        "Q4_K_S", "Q4_K_M",
        "Q5_K_S", "Q5_K_M",
        "Q6_K", "Q8_0",
        "Q2_K", "Q3_K", "Q4_K", "Q5_K",
        "Q4_0", "Q4_1", "Q5_0", "Q5_1",
        "BF16", "F16", "F32",
    ];
    for n in NEEDLES {
        if upper.contains(n) {
            return n.to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quant_signature_picks_longest_match() {
        assert_eq!(parse_quant_signature("Llama-3.1-8B-Q4_K_M.gguf"), "Q4_K_M");
        assert_eq!(parse_quant_signature("model-iq2_xxs.gguf"), "IQ2_XXS");
        assert_eq!(parse_quant_signature("model-Q4_0.gguf"), "Q4_0");
        assert_eq!(parse_quant_signature("model-q8_0.gguf"), "Q8_0");
        assert_eq!(parse_quant_signature("model-bf16.gguf"), "BF16");
        assert_eq!(parse_quant_signature("model-f16.gguf"), "F16");
    }

    #[test]
    fn parse_quant_signature_empty_on_unknown() {
        assert_eq!(parse_quant_signature("totally-unknown.gguf"), "");
        assert_eq!(parse_quant_signature(""), "");
    }

    #[test]
    fn parse_quant_signature_distinguishes_overlapping_tags() {
        // Q4_K should not be confused with Q4_K_M (the longer match wins).
        assert_eq!(parse_quant_signature("model-Q4_K_M.gguf"), "Q4_K_M");
        assert_eq!(parse_quant_signature("model-Q4_K.gguf"), "Q4_K");
    }

    #[test]
    fn props_from_value_picks_obvious_fields() {
        let v = serde_json::json!({
            "model_path": "/models/Llama-3.1-8B-Q4_K_M.gguf",
            "model_alias": "llama-3.1-8b",
            "chat_template": "<|begin_of_text|>{{ ... }}",
            "default_generation_settings": {
                "n_ctx": 8192
            }
        });
        let p = ServerProps::from_value(&v);
        assert_eq!(p.model_path.as_deref(), Some("/models/Llama-3.1-8B-Q4_K_M.gguf"));
        assert_eq!(p.model_alias.as_deref(), Some("llama-3.1-8b"));
        assert_eq!(p.n_ctx, Some(8192));
        assert!(p.chat_template.is_some());
    }

    #[test]
    fn derive_identity_is_stable_for_same_props() {
        let v = serde_json::json!({
            "model_path": "/models/Llama-3.1-8B-Q4_K_M.gguf",
            "chat_template": "tmpl_v1",
        });
        let a = ServerProps::from_value(&v).derive_identity();
        let b = ServerProps::from_value(&v).derive_identity();
        assert_eq!(a, b);
        assert_eq!(a.model_id, "Llama-3.1-8B-Q4_K_M.gguf");
        assert_eq!(a.quant_signature, "Q4_K_M");
        assert_eq!(a.tokenizer_hash.len(), 64);
    }

    #[test]
    fn derive_identity_differs_when_template_changes() {
        let v1 = serde_json::json!({
            "model_path": "/models/foo.gguf",
            "chat_template": "<|begin|>",
        });
        let v2 = serde_json::json!({
            "model_path": "/models/foo.gguf",
            "chat_template": "<|start|>",
        });
        let a = ServerProps::from_value(&v1).derive_identity();
        let b = ServerProps::from_value(&v2).derive_identity();
        // Same model_id (filename) and quant, but tokenizer_hash must differ.
        assert_eq!(a.model_id, b.model_id);
        assert_ne!(a.tokenizer_hash, b.tokenizer_hash);
    }

    #[test]
    fn derive_identity_falls_back_to_alias_when_path_missing() {
        let v = serde_json::json!({
            "model_alias": "my-cool-model"
        });
        let id = ServerProps::from_value(&v).derive_identity();
        assert_eq!(id.model_id, "my-cool-model");
        assert_eq!(id.quant_signature, "");
    }
}

#[derive(Debug, Clone)]
pub struct RestoredSlot {
    pub n_restored: Option<u32>,
    pub raw: serde_json::Value,
}
