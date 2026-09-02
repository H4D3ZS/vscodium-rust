//! Pure-Rust WebDriverAgent HTTP client — the touch/keyboard control layer for
//! the iPhone mirror (the "scrcpy for iOS" input side).
//!
//! WebDriverAgent (WDA) runs *on the device* and exposes a JSON HTTP API on
//! port 8100. We reach it over USB via `ios forward 8100 8100`, then drive it
//! with `reqwest` — no Appium, no Node, no Java. Coordinates are in points
//! (WDA's space); the frontend scales click positions into this space using
//! `window_size`.

use std::time::Duration;

use reqwest::Client;
use serde_json::{json, Value};

pub struct WdaClient {
    base_url: String,
    http: Client,
    session_id: Option<String>,
}

impl WdaClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        // keep-alive is on by default in reqwest; pool_idle_timeout keeps the
        // USB-forwarded connection warm between taps for low latency.
        let http = Client::builder()
            .pool_idle_timeout(Duration::from_secs(90))
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| Client::new());
        Self { base_url: base_url.into(), http, session_id: None }
    }

    /// True if WDA answers `/status` (i.e. it's running and forwarded).
    pub async fn is_ready(&self) -> bool {
        self.http
            .get(format!("{}/status", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// Forget the cached session so the next call re-creates one (WDA restarted).
    pub fn reset_session(&mut self) {
        self.session_id = None;
    }

    async fn ensure_session(&mut self) -> Result<String, String> {
        if let Some(id) = &self.session_id {
            return Ok(id.clone());
        }
        let body = json!({ "capabilities": { "alwaysMatch": {}, "firstMatch": [ {} ] } });
        let resp = self
            .http
            .post(format!("{}/session", self.base_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("WDA session: {e}"))?;
        let v: Value = resp.json().await.map_err(|e| format!("WDA session json: {e}"))?;
        // WDA reports sessionId at the top level and/or under `value`.
        let id = v
            .get("sessionId")
            .and_then(|s| s.as_str())
            .or_else(|| v.get("value").and_then(|val| val.get("sessionId")).and_then(|s| s.as_str()))
            .ok_or_else(|| format!("WDA returned no sessionId: {v}"))?
            .to_string();
        self.session_id = Some(id.clone());
        Ok(id)
    }

    /// Device screen size in points — the WDA coordinate space to scale into.
    pub async fn window_size(&mut self) -> Result<(f64, f64), String> {
        let id = self.ensure_session().await?;
        let v: Value = self
            .http
            .get(format!("{}/session/{}/window/size", self.base_url, id))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;
        let w = v["value"]["width"].as_f64().ok_or("WDA window/size: no width")?;
        let h = v["value"]["height"].as_f64().ok_or("WDA window/size: no height")?;
        Ok((w, h))
    }

    pub async fn tap(&mut self, x: f64, y: f64) -> Result<(), String> {
        let id = self.ensure_session().await?;
        self.post(&format!("/session/{}/wda/tap/0", id), json!({ "x": x, "y": y })).await
    }

    pub async fn swipe(&mut self, fx: f64, fy: f64, tx: f64, ty: f64, duration: f64) -> Result<(), String> {
        let id = self.ensure_session().await?;
        self.post(
            &format!("/session/{}/wda/dragfromtoforduration", id),
            json!({ "fromX": fx, "fromY": fy, "toX": tx, "toY": ty, "duration": duration }),
        )
        .await
    }

    pub async fn type_text(&mut self, text: &str) -> Result<(), String> {
        let id = self.ensure_session().await?;
        let keys: Vec<String> = text.chars().map(|c| c.to_string()).collect();
        self.post(&format!("/session/{}/wda/keys", id), json!({ "value": keys })).await
    }

    /// Press the Home button (session-less endpoint).
    pub async fn home(&self) -> Result<(), String> {
        self.post("/wda/homescreen", json!({})).await
    }

    async fn post(&self, path: &str, body: Value) -> Result<(), String> {
        let resp = self
            .http
            .post(format!("{}{}", self.base_url, path))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("WDA {path}: {e}"))?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            Err(format!("WDA {path} -> {status}: {text}"))
        }
    }
}
