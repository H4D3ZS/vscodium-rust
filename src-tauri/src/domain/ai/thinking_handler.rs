//! Unified thinking model handler.
//!
//! Instead of ad-hoc string matching per model family (qwen3, gemma4, deepseek-r1),
//! this module detects the thinking format from the content itself and handles it
//! uniformly. This means ANY thinking model works without code changes.
//!
//! Supported formats:
//! - `<think>...</think>` tags in content (Gemma 4, Qwen3, DeepSeek R1)
//! - `<|channel>thought...<channel|>` channels (Gemma 4 native)
//! - Empty content with non-empty `reasoning_content` (OpenAI-compat)
//! - Empty content with non-empty `message.thinking` (Ollama native)

/// Result of thinking extraction.
pub struct ThinkingResult {
    /// The clean content after stripping thinking blocks.
    pub clean_content: String,
    /// The extracted thinking/reasoning text.
    pub thinking: String,
    /// Which format was detected.
    pub format: ThinkingFormat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThinkingFormat {
    /// No thinking detected — content is plain.
    None,
    /// `<think>...</think>` tags found and stripped.
    ThinkTags,
    /// `<|channel>thought...<channel|>` channels found and stripped.
    ChannelTags,
    /// Content was empty; thinking came from `reasoning_content` field.
    ReasoningContent,
    /// Content was empty; thinking came from `message.thinking` field.
    NativeThinking,
}

/// Extract thinking from content. Handles all formats uniformly.
///
/// This is the SINGLE entry point for thinking model handling. Call this
/// instead of per-model-family stripping logic.
pub fn extract_thinking(content: &str, reasoning_buf: &str) -> ThinkingResult {
    // 1. Check for `<think>...</think>` tags
    if let Some(result) = extract_think_tags(content) {
        return result;
    }

    // 2. Check for `<|channel>thought...<channel|>` format
    if let Some(result) = extract_channel_tags(content) {
        return result;
    }

    // 3. If content is empty but reasoning_buf has content (OpenAI-compat / Ollama native)
    if content.trim().is_empty() && !reasoning_buf.trim().is_empty() {
        return ThinkingResult {
            clean_content: String::new(),
            thinking: reasoning_buf.trim().to_string(),
            format: ThinkingFormat::ReasoningContent,
        };
    }

    // 4. No thinking detected
    ThinkingResult {
        clean_content: content.to_string(),
        thinking: String::new(),
        format: ThinkingFormat::None,
    }
}

/// Extract `<think>...</think>` tags from content.
fn extract_think_tags(content: &str) -> Option<ThinkingResult> {
    let mut clean = String::new();
    let mut thinking = String::new();
    let mut pos = 0usize;
    let mut found = false;

    while pos < content.len() {
        if let Some(open) = content[pos..].find("<think>") {
            found = true;
            let open_abs = pos + open;
            clean.push_str(&content[pos..open_abs]);
            let after_open = open_abs + "<think>".len();
            if let Some(close) = content[after_open..].find("</think>") {
                let close_abs = after_open + close;
                let thought = &content[after_open..close_abs];
                if !thought.trim().is_empty() {
                    if !thinking.is_empty() { thinking.push_str("\n\n"); }
                    thinking.push_str(thought.trim());
                }
                pos = close_abs + "</think>".len();
            } else {
                // Unclosed tag — treat rest as thinking
                thinking.push_str(&content[after_open..]);
                pos = content.len();
            }
        } else {
            clean.push_str(&content[pos..]);
            break;
        }
    }

    if !found { return None; }

    Some(ThinkingResult {
        clean_content: clean.trim().to_string(),
        thinking,
        format: ThinkingFormat::ThinkTags,
    })
}

/// Extract `<|channel>thought...<channel|>` tags from content.
fn extract_channel_tags(content: &str) -> Option<ThinkingResult> {
    let mut clean = String::new();
    let mut thinking = String::new();
    let mut pos = 0usize;
    let mut found = false;

    while pos < content.len() {
        if let Some(rel) = content[pos..].find("<|channel>thought") {
            found = true;
            let start = pos + rel;
            clean.push_str(&content[pos..start]);
            let after_marker = start + "<|channel>thought".len();
            if let Some(end_rel) = content[after_marker..].find("<channel|>") {
                let end = after_marker + end_rel;
                let thought = content[after_marker..end]
                    .trim_start_matches('\n')
                    .trim();
                if !thought.is_empty() {
                    if !thinking.is_empty() { thinking.push_str("\n\n"); }
                    thinking.push_str(thought);
                }
                pos = end + "<channel|>".len();
            } else {
                // Unclosed channel — treat rest as thinking
                thinking.push_str(&content[after_marker..]);
                pos = content.len();
            }
        } else {
            clean.push_str(&content[pos..]);
            break;
        }
    }

    if !found { return None; }

    Some(ThinkingResult {
        clean_content: clean.trim().to_string(),
        thinking,
        format: ThinkingFormat::ChannelTags,
    })
}

/// Check if a model name is a known thinking model (for sampling parameters).
/// This is ONLY used for sampling config, not for content handling.
pub fn is_thinking_model(model: &str) -> bool {
    let m = model.to_lowercase();
    m.contains("qwen3") || m.contains("qwq")
        || m.contains("deepseek-r1") || m.contains("r1:")
        || m.contains("gemma4") || m.contains("gemma-4")
        || m.contains("phi-4") || m.contains("phi4")
        || m.contains("think")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_think_tags() {
        let result = extract_thinking("<think>reasoning</think>answer", "");
        assert_eq!(result.format, ThinkingFormat::ThinkTags);
        assert_eq!(result.clean_content, "answer");
        assert_eq!(result.thinking, "reasoning");
    }

    #[test]
    fn test_think_tags_only() {
        let result = extract_thinking("<think>reasoning</think>", "");
        assert_eq!(result.format, ThinkingFormat::ThinkTags);
        assert_eq!(result.clean_content, "");
        assert_eq!(result.thinking, "reasoning");
    }

    #[test]
    fn test_channel_tags() {
        let result = extract_thinking("<|channel>thoughtreasoning<channel|>answer", "");
        assert_eq!(result.format, ThinkingFormat::ChannelTags);
        assert_eq!(result.clean_content, "answer");
        assert_eq!(result.thinking, "reasoning");
    }

    #[test]
    fn test_reasoning_content() {
        let result = extract_thinking("", "reasoning from model");
        assert_eq!(result.format, ThinkingFormat::ReasoningContent);
        assert_eq!(result.clean_content, "");
        assert_eq!(result.thinking, "reasoning from model");
    }

    #[test]
    fn test_no_thinking() {
        let result = extract_thinking("just plain text", "");
        assert_eq!(result.format, ThinkingFormat::None);
        assert_eq!(result.clean_content, "just plain text");
        assert!(result.thinking.is_empty());
    }

    #[test]
    fn test_is_thinking_model() {
        assert!(is_thinking_model("gemma-4-12B-v2"));
        assert!(is_thinking_model("qwen3.5-MTP"));
        assert!(is_thinking_model("deepseek-r1:8b"));
        assert!(!is_thinking_model("llama3:8b"));
        assert!(!is_thinking_model("codellama:7b"));
    }
}
