use crate::state::HadesNativeState;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AiThought {
    pub logic: String,
    pub action: String,
    pub confidence: Option<f32>,
    pub step: Option<String>,
}

impl Default for AiThought {
    fn default() -> Self {
        Self {
            logic: "Evaluating AST symbol dependencies and active workspace context...".to_string(),
            action: "Inspect module exports and resolve references".to_string(),
            confidence: Some(0.96),
            step: Some("Step 2/4".to_string()),
        }
    }
}

impl AiThought {
    pub fn new(
        logic: impl Into<String>,
        action: impl Into<String>,
        confidence: Option<f32>,
    ) -> Self {
        Self {
            logic: logic.into(),
            action: action.into(),
            confidence,
            step: None,
        }
    }

    pub fn parse(raw: &str) -> Self {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Self::default();
        }

        // Try JSON parsing if it resembles a JSON object
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                let logic = val
                    .get("logic")
                    .or_else(|| val.get("reasoning"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(trimmed)
                    .to_string();

                let action = val
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Reasoning")
                    .to_string();

                let confidence = val
                    .get("confidence")
                    .and_then(|v| v.as_f64())
                    .map(|c| c as f32);

                let step = val
                    .get("step")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                return Self {
                    logic,
                    action,
                    confidence,
                    step,
                };
            }
        }

        // Structured prefix extraction
        if let Some((action_part, logic_part)) = trimmed.split_once("->") {
            Self {
                logic: logic_part.trim().to_string(),
                action: action_part.trim().to_string(),
                confidence: Some(0.95),
                step: None,
            }
        } else {
            Self {
                logic: trimmed.to_string(),
                action: "Autonomous Reasoning".to_string(),
                confidence: Some(0.95),
                step: None,
            }
        }
    }
}

pub fn render_thought_process_hud(
    state: &HadesNativeState,
    thought: &AiThought,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let confidence_pct = thought
        .confidence
        .map(|c| (c.clamp(0.0, 1.0) * 100.0).round() as i32);
    let logic_text = thought.logic.clone();
    let action_text = thought.action.clone();
    let step_label = thought.step.clone();

    div()
        .id("thought_process_hud")
        .absolute()
        .bottom(px(36.0))
        .right(px(24.0))
        .w(px(350.0))
        .max_h(px(400.0))
        .bg(rgba(0x0a0f1dF2))
        .border_1()
        .border_color(rgba(0xa855f750))
        .rounded(px(14.0))
        .shadow_lg()
        .flex()
        .flex_col()
        .overflow_hidden()
        // ── HUD Header ──────────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_3p5()
                .py_2p5()
                .bg(rgba(0xa855f718))
                .border_b_1()
                .border_color(rgba(0xa855f730))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        // Glowing Brain/AI Chip
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(22.0))
                                .h(px(22.0))
                                .rounded_full()
                                .bg(rgba(0xa855f733))
                                .border_1()
                                .border_color(rgb(0xa855f7))
                                .child(ui_icon(IconName::Bot, 13.0, rgb(0xc084fc))),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xf3e8ff))
                                .child("COGNITIVE PROCESS"),
                        )
                        .children(step_label.map(|s| {
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_full()
                                .bg(rgba(0xa855f730))
                                .text_xs()
                                .text_color(rgb(0xd8b4fe))
                                .child(s)
                        })),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        // Pulsing Activity Dots
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .child(div().w(px(5.0)).h(px(5.0)).rounded_full().bg(rgb(0xc084fc)))
                                .child(
                                    div()
                                        .w(px(5.0))
                                        .h(px(5.0))
                                        .rounded_full()
                                        .bg(rgba(0xc084fc80)),
                                )
                                .child(
                                    div()
                                        .w(px(5.0))
                                        .h(px(5.0))
                                        .rounded_full()
                                        .bg(rgba(0xc084fc40)),
                                ),
                        )
                        // Close / Dismiss button
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.current_thought = None;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::X, rgb(0xd8b4fe))),
                        ),
                ),
        )
        // ── HUD Body ────────────────────────────────────────────────────────
        .child(
            div()
                .id("thought_process_scroll")
                .flex()
                .flex_col()
                .p_3p5()
                .gap_3()
                .overflow_y_scroll()
                // Internal Logic Block
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x94a3b8))
                                .child(ui_icon(IconName::Cpu, 12.0, rgb(0x94a3b8)))
                                .child("INTERNAL LOGIC"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0xe2e8f0))
                                .line_height(px(18.0))
                                .font_family(".SystemUIFont")
                                .child(format!("\"{logic_text}\"")),
                        ),
                )
                // Proposed Action Block
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xa855f7))
                                .child(ui_icon(IconName::Sparkles, 12.0, rgb(0xa855f7)))
                                .child("PROPOSED ACTION"),
                        )
                        .child(
                            div()
                                .p_2p5()
                                .rounded(px(8.0))
                                .bg(rgba(0xa855f718))
                                .border_1()
                                .border_color(rgba(0xa855f740))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(0xf3e8ff))
                                        .child(action_text),
                                ),
                        ),
                )
                // Confidence Indicator
                .children(confidence_pct.map(|pct| {
                    let pct_f = (pct as f32) / 100.0;
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .text_color(rgb(0x94a3b8))
                                        .child("CONFIDENCE"),
                                )
                                .child(div().text_color(rgb(0xc084fc)).child(format!("{pct}%"))),
                        )
                        .child(
                            div()
                                .w_full()
                                .h(px(4.0))
                                .bg(rgba(0xffffff15))
                                .rounded_full()
                                .overflow_hidden()
                                .child(
                                    div()
                                        .w(px(320.0 * pct_f))
                                        .h_full()
                                        .bg(rgb(0xa855f7))
                                        .rounded_full(),
                                ),
                        )
                })),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_parse_thought_plain_text() {
        let text = "Analyzing memory requirements for zero-copy buffer swap";
        let thought = AiThought::parse(text);
        assert_eq!(thought.logic, text);
        assert_eq!(thought.action, "Autonomous Reasoning");
        assert_eq!(thought.confidence, Some(0.95));
    }

    #[test]
    fn test_parse_thought_json() {
        let json_str = r#"{"logic": "Synthesize syntax tree for new macro", "action": "Generate AST tokens", "confidence": 0.98, "step": "Phase 2/3"}"#;
        let thought = AiThought::parse(json_str);
        assert_eq!(thought.logic, "Synthesize syntax tree for new macro");
        assert_eq!(thought.action, "Generate AST tokens");
        assert_eq!(thought.confidence, Some(0.98));
        assert_eq!(thought.step.as_deref(), Some("Phase 2/3"));
    }

    #[test]
    fn test_parse_thought_structured_arrow() {
        let str_format = "Apply Diff -> Verify all hunk offsets and line numbers";
        let thought = AiThought::parse(str_format);
        assert_eq!(thought.action, "Apply Diff");
        assert_eq!(thought.logic, "Verify all hunk offsets and line numbers");
    }
}
