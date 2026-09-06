//! Semantic-anchor KV checkpoints (plan §2.6).
//!
//! KDKVC saves one slot at completion and matches by longest **token** prefix.
//! That is optimal when turn N+1 = turn N + more, but weak when the agent edits
//! mid-context (retries a tool, prunes a `<think>` block, a tool result
//! changed): longest-prefix then matches only up to the first divergence, which
//! can be very early.
//!
//! Fix: after a normal save, also write a few extra index entries keyed on the
//! token prefix **at a recent message boundary**, all pointing at the same
//! `.slotbin` file. On restore, llama-server loads the full slot and trims its
//! KV to the common prefix with the incoming request — so an anchor entry lets
//! a mid-edited turn still restore everything up to that boundary.
//!
//! This module is the pure part: given the rendered prefix text, pick which
//! byte offsets to checkpoint at. Tokenising those offsets and writing the
//! aliased entries is done in `proxy.rs` (needs the llama client + store), and
//! only when `KORTEX_KV_ANCHORS=1`.

/// Default cap on anchors per save. Two or three is plenty — the point is to
/// cover "the last edit was in the last message or two", not to snapshot the
/// whole conversation.
pub const DEFAULT_MAX_ANCHORS: usize = 3;

/// Config resolved from the environment. Off unless `KORTEX_KV_ANCHORS=1`.
#[derive(Debug, Clone, Copy)]
pub struct AnchorConfig {
    pub enabled: bool,
    pub max_anchors: usize,
    /// Minimum token gap between an anchor and the tail save — an anchor within
    /// a handful of tokens of the tail buys nothing and just bloats the index.
    pub min_gap_tokens: u32,
}

impl AnchorConfig {
    pub fn from_env() -> Self {
        let enabled = matches!(
            std::env::var("KORTEX_KV_ANCHORS").ok().as_deref(),
            Some("1") | Some("true") | Some("on") | Some("yes")
        );
        let max_anchors = std::env::var("KORTEX_KV_ANCHORS_MAX")
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .filter(|n| *n > 0 && *n <= 16)
            .unwrap_or(DEFAULT_MAX_ANCHORS);
        let min_gap_tokens = std::env::var("KORTEX_KV_ANCHORS_MIN_GAP")
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(64);
        Self {
            enabled,
            max_anchors,
            min_gap_tokens,
        }
    }
}

impl Default for AnchorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_anchors: DEFAULT_MAX_ANCHORS,
            min_gap_tokens: 64,
        }
    }
}

/// Byte offsets into `render` just past each of the last `max` message
/// boundaries, **nearest the end first**, excluding the final boundary
/// (`render.len()` — that's the tail the normal save already covers) and
/// offset 0.
///
/// `extract_chat_prefix_text` writes one message per line as
/// `<role>content</role>\n`, so every `\n` is a message boundary. Byte offsets
/// (not char offsets) are what `str` slicing wants; the render is ASCII-framed
/// but content may be UTF-8, so we only ever cut at a `\n` which is always a
/// char boundary.
pub fn tail_boundary_offsets(render: &str, max: usize) -> Vec<usize> {
    if max == 0 || render.is_empty() {
        return Vec::new();
    }
    let mut offsets: Vec<usize> = render
        .match_indices('\n')
        .map(|(i, _)| i + 1) // just past the newline
        .filter(|&off| off > 0 && off < render.len())
        .collect();
    offsets.reverse(); // nearest the end first
    offsets.truncate(max);
    offsets
}

#[cfg(test)]
mod tests {
    use super::*;

    const R: &str = "<user>a</user>\n<assistant>b</assistant>\n<user>c</user>\n";

    #[test]
    fn offsets_are_tail_first_and_exclude_the_end() {
        let offs = tail_boundary_offsets(R, 3);
        // three '\n's; the last is at render.len() and is excluded → 2 offsets.
        assert_eq!(offs.len(), 2);
        // nearest the end first
        assert!(offs[0] > offs[1]);
        // each lands right after a newline
        for &o in &offs {
            assert_eq!(&R[o - 1..o], "\n");
        }
        // first one is just past the 2nd newline (end of the assistant msg)
        let second_nl = R.match_indices('\n').nth(1).unwrap().0;
        assert_eq!(offs[0], second_nl + 1);
    }

    #[test]
    fn respects_max() {
        assert_eq!(tail_boundary_offsets(R, 1).len(), 1);
        assert_eq!(tail_boundary_offsets(R, 0).len(), 0);
    }

    #[test]
    fn slicing_at_an_offset_is_valid_utf8_boundary() {
        let r = "<user>héllo 世界</user>\n<user>next</user>\n";
        for o in tail_boundary_offsets(r, 5) {
            assert!(r.is_char_boundary(o));
            let _ = &r[..o]; // must not panic
        }
    }

    #[test]
    fn empty_and_no_newline() {
        assert!(tail_boundary_offsets("", 3).is_empty());
        assert!(tail_boundary_offsets("no newline here", 3).is_empty());
        // a single trailing newline is the excluded final boundary
        assert!(tail_boundary_offsets("one line\n", 3).is_empty());
    }

    #[test]
    fn config_defaults_off() {
        let c = AnchorConfig::default();
        assert!(!c.enabled);
        assert_eq!(c.max_anchors, DEFAULT_MAX_ANCHORS);
    }
}
