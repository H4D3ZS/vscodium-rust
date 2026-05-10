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
}

#[derive(Debug, Clone)]
pub struct RestoredSlot {
    pub n_restored: Option<u32>,
    pub raw: serde_json::Value,
}
