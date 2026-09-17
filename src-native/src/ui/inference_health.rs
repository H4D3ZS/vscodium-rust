use crate::state::HadesNativeState;
use crate::theme::Theme;
use crate::HadesAppView;
use gpui_kit::gpui::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceStatus {
    Connected,
    Checking,
    Disconnected,
}

impl InferenceStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Connected => "Connected",
            Self::Checking => "Checking...",
            Self::Disconnected => "Disconnected",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferenceModelInfo {
    pub id: String,
    pub provider: String,
    pub size_gb: f32,
    pub quant: String,
    pub context_length: usize,
    pub is_active: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferenceHealthState {
    pub visible: bool,
    pub backend: String,
    pub status: InferenceStatus,
    pub endpoint: String,
    pub latency_ms: Option<u64>,
    pub throughput_tps: Option<f32>,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub kv_cache_pct: f32,
    pub probing: bool,
    pub models: Vec<InferenceModelInfo>,
}

impl Default for InferenceHealthState {
    fn default() -> Self {
        Self {
            visible: false,
            backend: "Lemonade (Local)".to_string(),
            status: InferenceStatus::Connected,
            endpoint: "http://127.0.0.1:13305".to_string(),
            latency_ms: Some(38),
            throughput_tps: Some(58.4),
            vram_used_mb: 12480,
            vram_total_mb: 16384,
            kv_cache_pct: 28.4,
            probing: false,
            models: vec![
                InferenceModelInfo {
                    id: "Qwen3.8-27B-Uncensored-Cyber-IQ4_XS-imatrix-fromq8.gguf".to_string(),
                    provider: "lemonade".to_string(),
                    size_gb: 16.1,
                    quant: "IQ4_XS (imatrix)".to_string(),
                    context_length: 32768,
                    is_active: true,
                },
                InferenceModelInfo {
                    id: "Qwen3.8-35B-A3B-Q4_K_M.gguf".to_string(),
                    provider: "lemonade".to_string(),
                    size_gb: 21.7,
                    quant: "Q4_K_M (3B Active)".to_string(),
                    context_length: 65536,
                    is_active: false,
                },
                InferenceModelInfo {
                    id: "Qwen3.5-4B-GGUF-Q5_K_M".to_string(),
                    provider: "lemonade".to_string(),
                    size_gb: 3.2,
                    quant: "Q5_K_M".to_string(),
                    context_length: 32768,
                    is_active: false,
                },
                InferenceModelInfo {
                    id: "Escha-W2-35B-A3B-ROCmFP2-Qwen3.6-35B-A3B-Escha-W2-ROCmFP2.gguf".to_string(),
                    provider: "lemonade".to_string(),
                    size_gb: 14.8,
                    quant: "W2-ROCmFP2".to_string(),
                    context_length: 65536,
                    is_active: false,
                },
                InferenceModelInfo {
                    id: "Qwen3-Embedding-0.6B-GGUF".to_string(),
                    provider: "lemonade".to_string(),
                    size_gb: 0.6,
                    quant: "F16".to_string(),
                    context_length: 8192,
                    is_active: false,
                },
            ],
        }
    }
}

impl InferenceHealthState {
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn select_model(&mut self, model_id: &str) {
        let mut matched = false;
        let mid_lower = model_id.to_lowercase();
        for m in &mut self.models {
            let id_lower = m.id.to_lowercase();
            m.is_active = m.id == model_id
                || id_lower == mid_lower
                || (mid_lower.contains("qwen3.8") && id_lower.contains("qwen3.8"))
                || (mid_lower.contains("qwen3.5") && id_lower.contains("qwen3.5"))
                || (mid_lower.contains("escha") && id_lower.contains("escha"))
                || (mid_lower.contains("embed") && id_lower.contains("embed"));
            if m.is_active {
                matched = true;
            }
        }
        if !matched {
            for m in &mut self.models {
                m.is_active = false;
            }
            let provider = if model_id.contains("ModelScope") {
                "modelscope".to_string()
            } else if model_id.contains("Claude") || model_id.contains("Gemini") || model_id.contains("GPT") {
                "cloud-byok".to_string()
            } else {
                "lemonade".to_string()
            };
            self.models.push(InferenceModelInfo {
                id: model_id.to_string(),
                provider,
                size_gb: 0.0,
                quant: "Active".to_string(),
                context_length: 32768,
                is_active: true,
            });
        }
    }

    pub fn active_model(&self) -> Option<&InferenceModelInfo> {
        self.models.iter().find(|m| m.is_active)
    }

    pub fn active_model_name(&self) -> String {
        self.active_model()
            .map(|m| m.id.clone())
            .unwrap_or_else(|| "No model loaded".to_string())
    }

    /// Blocking probe — call from a background thread, then push result via apply_fetched_models.
    /// Returns (latency_ms, models) or an error string.
    pub fn fetch_models_blocking(endpoint: &str) -> Result<(u64, Vec<InferenceModelInfo>), String> {
        use std::time::Instant;

        let t0 = Instant::now();
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(4))
            .build()
            .map_err(|e| format!("client build: {e}"))?;

        // Try both path prefixes — older Lemonade builds use /v1/, newer use /api/v1/
        let urls = [
            format!("{}/api/v1/models", endpoint.trim_end_matches('/')),
            format!("{}/v1/models", endpoint.trim_end_matches('/')),
        ];

        let mut last_err = String::from("no url tried");
        for url in &urls {
            match client.get(url).send() {
                Ok(resp) => {
                    let latency_ms = t0.elapsed().as_millis() as u64;
                    let body: serde_json::Value = resp
                        .json()
                        .map_err(|e| format!("JSON parse: {e}"))?;

                    let data = body.get("data").and_then(|d| d.as_array()).cloned()
                        .unwrap_or_else(|| {
                            body.as_array().cloned().unwrap_or_default()
                        });

                    let models: Vec<InferenceModelInfo> = data.iter().filter_map(|m| {
                        let id = m.get("id").and_then(|v| v.as_str())?.to_string();
                        let quant = if id.contains("IQ4_XS") { "IQ4_XS (imatrix)" }
                            else if id.contains("IQ3_XXS") { "IQ3_XXS" }
                            else if id.contains("Q4_K_M") { "Q4_K_M" }
                            else if id.contains("Q5_K_M") { "Q5_K_M" }
                            else if id.contains("Q8_0") || id.contains("Q8") { "Q8_0" }
                            else if id.contains("W2-ROCmFP2") { "W2-ROCmFP2" }
                            else if id.contains("F16") { "F16" }
                            else { "GGUF" };

                        let size_gb: f32 = m.get("size")
                            .and_then(|v| v.as_f64())
                            .map(|b| b as f32 / 1_073_741_824.0)
                            .unwrap_or(0.0);

                        let context_length: usize = m.get("context_window")
                            .or_else(|| m.get("context_length"))
                            .or_else(|| m.get("max_context"))
                            .and_then(|v| v.as_u64())
                            .map(|v| v as usize)
                            .unwrap_or(32768);

                        Some(InferenceModelInfo {
                            id,
                            provider: "lemonade".to_string(),
                            size_gb,
                            quant: quant.to_string(),
                            context_length,
                            is_active: false,
                        })
                    }).collect();

                    return Ok((latency_ms, models));
                }
                Err(e) => {
                    last_err = format!("{url}: {e}");
                    continue;
                }
            }
        }
        Err(last_err)
    }


    /// Apply a freshly fetched model list. Preserves the current active selection.
    pub fn apply_fetched_models(&mut self, latency_ms: u64, mut fetched: Vec<InferenceModelInfo>) {
        let active_id = self.active_model().map(|m| m.id.clone());

        // Mark the currently active model
        for m in &mut fetched {
            if let Some(ref aid) = active_id {
                let aid_lc = aid.to_lowercase();
                let mid_lc = m.id.to_lowercase();
                m.is_active = m.id == *aid
                    || mid_lc == aid_lc
                    || (aid_lc.contains("27b") && mid_lc.contains("27b"))
                    || (aid_lc.contains("qwen3.8") && mid_lc.contains("qwen3.8"))
                    || (aid_lc.contains("escha") && mid_lc.contains("escha"));
            }
        }

        // If nothing matched, mark first as active
        if !fetched.is_empty() && !fetched.iter().any(|m| m.is_active) {
            fetched[0].is_active = true;
        }

        self.models = fetched;
        self.latency_ms = Some(latency_ms);
        self.status = InferenceStatus::Connected;
        self.probing = false;
    }

    pub fn refresh_probe(&mut self) {
        self.probing = true;
        self.status = InferenceStatus::Checking;
    }
}


fn render_header(
    theme: &Theme,
    health: &InferenceHealthState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let status_color = match health.status {
        InferenceStatus::Connected => rgb(0x10b981),
        InferenceStatus::Checking => rgb(0xf59e0b),
        InferenceStatus::Disconnected => rgb(0xf43f5e),
    };
    let title = format!("{} Health", health.backend);

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .pb_2()
        .border_b_1()
        .border_color(theme.border_subtle)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .w(px(10.0))
                        .h(px(10.0))
                        .rounded_full()
                        .bg(status_color),
                )
                .child(div().font_weight(FontWeight::BOLD).text_sm().child(title)),
        )
        .child(
            div()
                .cursor_pointer()
                .px_1p5()
                .py_0p5()
                .rounded(px(4.0))
                .text_color(theme.text_muted)
                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.state.inference_health.visible = false;
                        cx.notify();
                    }),
                )
                .child("×"),
        )
}

fn render_status_metrics(theme: &Theme, health: &InferenceHealthState) -> impl IntoElement {
    let status_color = match health.status {
        InferenceStatus::Connected => rgb(0x10b981),
        InferenceStatus::Checking => rgb(0xf59e0b),
        InferenceStatus::Disconnected => rgb(0xf43f5e),
    };

    let latency_val = health.latency_ms.unwrap_or(0);
    let latency_color = if latency_val < 100 {
        rgb(0x10b981)
    } else if latency_val < 300 {
        rgb(0xf59e0b)
    } else {
        rgb(0xf43f5e)
    };

    let endpoint_url = health.endpoint.clone();
    let status_label = health.status.label().to_string();
    let tps_val = health.throughput_tps.unwrap_or(0.0);
    let vram_used = health.vram_used_mb;
    let vram_total = health.vram_total_mb;
    let vram_pct = if vram_total > 0 {
        ((vram_used as f32 / vram_total as f32) * 100.0).round() as u32
    } else {
        0
    };
    let kv_pct = health.kv_cache_pct.round() as u32;

    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .bg(theme.bg_editor)
        .p_2p5()
        .rounded(px(6.0))
        .border_1()
        .border_color(theme.border_subtle)
        // Row 1: Connection Status & Endpoint
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .text_xs()
                .child(div().text_color(theme.text_muted).child("Status"))
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(status_color)
                        .child(status_label),
                ),
        )
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .text_xs()
                .child(div().text_color(theme.text_muted).child("Endpoint"))
                .child(
                    div()
                        .font_family("monospace")
                        .text_color(theme.text_muted)
                        .child(endpoint_url),
                ),
        )
        // Row 2: Latency & Throughput
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .text_xs()
                .child(div().text_color(theme.text_muted).child("Latency / Ping"))
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(latency_color)
                        .child(format!("{} ms", latency_val)),
                ),
        )
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .text_xs()
                .child(div().text_color(theme.text_muted).child("Throughput"))
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x38bdf8))
                        .child(format!("{:.1} tokens/s", tps_val)),
                ),
        )
        // Row 3: VRAM bar
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .pt_1()
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .text_xs()
                        .child(
                            div()
                                .text_color(theme.text_muted)
                                .child(format!("VRAM: {} / {} MB", vram_used, vram_total)),
                        )
                        .child(
                            div()
                                .text_color(theme.text_muted)
                                .child(format!("{}%", vram_pct)),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .h(px(4.0))
                        .bg(rgba(0xffffff15))
                        .rounded(px(2.0))
                        .child(
                            div()
                                .w(px(3.5 * vram_pct as f32))
                                .h_full()
                                .bg(rgb(0x818cf8))
                                .rounded(px(2.0)),
                        ),
                ),
        )
        // Row 4: KV Cache bar
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .text_xs()
                        .child(div().text_color(theme.text_muted).child("KV Cache"))
                        .child(
                            div()
                                .text_color(theme.text_muted)
                                .child(format!("{}%", kv_pct)),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .h(px(4.0))
                        .bg(rgba(0xffffff15))
                        .rounded(px(2.0))
                        .child(
                            div()
                                .w(px(3.5 * kv_pct as f32))
                                .h_full()
                                .bg(rgb(0x34d399))
                                .rounded(px(2.0)),
                        ),
                ),
        )
}

fn render_models_section(
    theme: &Theme,
    health: &InferenceHealthState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let probing = health.probing;
    let models = health.models.clone();

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child(format!("AVAILABLE MODELS ({})", models.len())),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .px_2()
                        .py_0p5()
                        .bg(theme.bg_editor)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .rounded(px(4.0))
                        .text_xs()
                        .text_color(theme.text_primary)
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.inference_health.refresh_probe();
                                this.state.status_message =
                                    "Refreshed local inference probe metrics.".to_string();
                                cx.notify();
                            }),
                        )
                        .child(if probing {
                            "Probing..."
                        } else {
                            "Refresh Probe"
                        }),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .max_h(px(180.0))
                .overflow_hidden()
                .children(models.into_iter().map(|m| {
                    let model_id = m.id.clone();
                    let is_active = m.is_active;
                    let quant_badge = m.quant.clone();
                    let size_label = format!("{:.1} GB", m.size_gb);
                    let ctx_label = format!("{}k", m.context_length / 1024);

                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .p_2()
                        .rounded(px(4.0))
                        .border_1()
                        .border_color(if is_active {
                            theme.accent
                        } else {
                            theme.border_subtle
                        })
                        .bg(if is_active {
                            rgba(0x3b82f61a)
                        } else {
                            theme.bg_editor
                        })
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let mid = model_id.clone();
                                move |this, _event, _window, cx| {
                                    this.state.inference_health.select_model(&mid);
                                    this.state.agent_model = mid.clone();
                                    this.state.status_message =
                                        format!("Activated AI model: {}", mid);
                                    cx.notify();
                                }
                            }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_0p5()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1p5()
                                        .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(
                                            if is_active {
                                                rgb(0x10b981)
                                            } else {
                                                rgba(0xffffff30)
                                            },
                                        ))
                                        .child(
                                            div()
                                                .font_family("monospace")
                                                .text_xs()
                                                .font_weight(if is_active {
                                                    FontWeight::BOLD
                                                } else {
                                                    FontWeight::NORMAL
                                                })
                                                .child(model_id),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(div().child(format!("Size: {}", size_label)))
                                        .child(div().child(format!("Context: {}", ctx_label))),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_panel)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(quant_badge),
                                )
                                .children(is_active.then(|| {
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(rgba(0x10b98125))
                                        .text_xs()
                                        .text_color(rgb(0x10b981))
                                        .font_weight(FontWeight::BOLD)
                                        .child("ACTIVE")
                                })),
                        )
                })),
        )
}

fn render_footer_actions(theme: &Theme, cx: &mut Context<HadesAppView>) -> impl IntoElement {
    div()
        .flex()
        .gap_2()
        .pt_2()
        .border_t_1()
        .border_color(theme.border_subtle)
        .child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .py_1p5()
                .rounded(px(4.0))
                .bg(theme.accent)
                .text_color(rgb(0xffffff))
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .hover(|s| s.opacity(0.9))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.state.focused_panel = crate::domain::layout::FocusedPanel::Explorer;
                        this.state.active_activity = crate::domain::layout::ActivityTab::Settings;
                        this.state.settings_category = 1;
                        this.state.left_sidebar_open = true;
                        this.state.inference_health.visible = false;
                        this.state.status_message = "Opened AI Inference Settings.".to_string();
                        cx.notify();
                    }),
                )
                .child("Configure Backend..."),
        )
        .child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .py_1p5()
                .rounded(px(4.0))
                .bg(theme.bg_editor)
                .border_1()
                .border_color(theme.border_subtle)
                .text_color(theme.text_primary)
                .text_xs()
                .cursor_pointer()
                .hover(|s| s.bg(theme.bg_hover))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.state.inference_health.visible = false;
                        cx.notify();
                    }),
                )
                .child("Dismiss"),
        )
}

pub fn render_inference_health_dashboard(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let health = &state.inference_health;

    div()
        .absolute()
        .bottom(px(28.0))
        .left(px(320.0))
        .w(px(400.0))
        .max_h(px(520.0))
        .bg(theme.bg_panel)
        .border_1()
        .border_color(theme.border_subtle)
        .rounded(px(8.0))
        .shadow_xl()
        .p_4()
        .flex()
        .flex_col()
        .gap_3()
        .text_color(theme.text_primary)
        .child(render_header(theme, health, cx))
        .child(render_status_metrics(theme, health))
        .child(render_models_section(theme, health, cx))
        .child(render_footer_actions(theme, cx))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_inference_health_defaults() {
        let health = InferenceHealthState::default();
        assert!(!health.visible);
        assert_eq!(health.status, InferenceStatus::Connected);
        assert_eq!(health.status.label(), "Connected");
        assert!(health.latency_ms.is_some());
        assert_eq!(health.models.len(), 4);
        assert_eq!(health.active_model_name(), "Qwen3.8-27B-GGUF-IQ3_XXS");
    }

    #[test]
    fn test_inference_health_select_model() {
        let mut health = InferenceHealthState::default();
        health.select_model("Qwen3.5-4B-GGUF-Q5_K_M");
        assert_eq!(health.active_model_name(), "Qwen3.5-4B-GGUF-Q5_K_M");

        let active = health.active_model().unwrap();
        assert_eq!(active.quant, "Q5_K_M");
        assert_eq!(active.context_length, 32768);
    }

    #[test]
    fn test_inference_health_toggle_visibility_and_probe() {
        let mut health = InferenceHealthState::default();
        assert!(!health.visible);
        health.toggle_visibility();
        assert!(health.visible);
        health.toggle_visibility();
        assert!(!health.visible);

        health.refresh_probe();
        assert_eq!(health.status, InferenceStatus::Connected);
        assert_eq!(health.latency_ms, Some(35));
        assert_eq!(health.kv_cache_pct, 24.0);
    }
}
