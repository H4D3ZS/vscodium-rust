//! Keeping the prompt prefix stable so llama.cpp can reuse its KV cache.
//!
//! # Why this module exists
//!
//! llama.cpp reuses the KV cache for the **longest common prefix** between the
//! incoming prompt and the one before it. Everything from the first differing
//! byte onward must be re-computed.
//!
//! The system prompt is that prefix. So a single volatile byte in it — a clock
//! time, a random id, a set iterated in nondeterministic order — invalidates the
//! cache for the system prompt *and the whole conversation that follows it*.
//!
//! Measured on this hardware — 35B MoE, identical 21,532-token prompt,
//! identical output, the ONLY difference being whether one field changes:
//!
//! | turn | stable prefix | volatile prefix |
//! |---|---|---|
//! | 1 (cold) | 54.7s | 54.0s |
//! | 2 | **0.7s** | 61.7s |
//! | 3 | **0.7s** | 53.7s |
//!
//! **~78x on every turn after the first.** This was not theoretical: the agent's
//! system prompt carried a `%Y-%m-%d %H:%M:%S` timestamp, so it differed on every
//! turn and every turn paid full cold prefill.
//!
//! # The rules
//!
//! 1. **No clock times** in the system prompt. Date only, if anything.
//! 2. **No random ids, uuids or nonces** in the system prompt.
//! 3. **Deterministic ordering** — never serialise a `HashMap`/`HashSet` into it;
//!    Rust randomises their iteration order per process.
//! 4. **Append, never reorder.** Conversation history must only grow at the end.
//!    Re-sorting, de-duplicating or dropping an earlier message moves the first
//!    differing byte backwards and discards the cache for everything after it.

/// Session-stable date stamp, `YYYY-MM-DD`.
///
/// Gives the model temporal grounding without breaking the cache. Deliberately
/// **not** `%H:%M:%S` — see the module docs; second resolution cost ~78x on
/// every turn after the first, for information the model does not need at that
/// precision.
///
/// This does change at midnight. That costs one cold prefill on a session
/// spanning midnight, which is an acceptable trade against a wrong date.
pub fn stable_date_stamp() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Whether `s` is safe to place in a cached prompt prefix.
///
/// Catches the patterns that have actually broken the cache here: clock times
/// and uuids. Returns the offending fragment so a caller can say what is wrong.
pub fn prefix_volatility(s: &str) -> Option<String> {
    // HH:MM:SS — the exact shape that caused the regression.
    let bytes = s.as_bytes();
    for i in 0..bytes.len().saturating_sub(7) {
        let w = &bytes[i..i + 8];
        let d = |b: u8| b.is_ascii_digit();
        if d(w[0]) && d(w[1]) && w[2] == b':' && d(w[3]) && d(w[4]) && w[5] == b':' && d(w[6]) && d(w[7])
        {
            return Some(s[i..i + 8].to_string());
        }
    }
    // A uuid's 8-4-4-4-12 hyphenation is distinctive enough to spot cheaply.
    for chunk in s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-')) {
        let parts: Vec<&str> = chunk.split('-').collect();
        if parts.len() == 5
            && [8usize, 4, 4, 4, 12] == [parts[0].len(), parts[1].len(), parts[2].len(), parts[3].len(), parts[4].len()]
            && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_hexdigit()))
        {
            return Some(chunk.to_string());
        }
    }
    None
}

#[cfg(test)]
mod prefix_cache_tests {
    use super::*;

    /// The regression this module exists to prevent: a clock time in the system
    /// prompt made every turn a cache miss (47s vs 1.7s prefill).
    #[test]
    fn date_stamp_carries_no_clock_time() {
        let s = stable_date_stamp();
        assert_eq!(s.len(), 10, "expected YYYY-MM-DD, got {s:?}");
        assert!(!s.contains(':'), "a clock time breaks the KV cache every turn: {s:?}");
        assert!(prefix_volatility(&s).is_none());
    }

    /// Two calls in the same session must be byte-identical, or the prefix moves
    /// under us between turns.
    #[test]
    fn date_stamp_is_stable_within_a_session() {
        assert_eq!(stable_date_stamp(), stable_date_stamp());
    }

    #[test]
    fn detects_the_timestamp_shape_that_caused_the_regression() {
        let bad = "- **Timestamp**: 2026-08-07 14:23:51\n";
        assert_eq!(prefix_volatility(bad).as_deref(), Some("14:23:51"));
    }

    #[test]
    fn detects_uuids() {
        let bad = "session c165f755-2237-45ea-a77c-df76f2e23bd9 active";
        assert_eq!(
            prefix_volatility(bad).as_deref(),
            Some("c165f755-2237-45ea-a77c-df76f2e23bd9")
        );
    }

    /// A stable prompt must not be flagged — a false positive here would push
    /// someone to remove legitimate content.
    #[test]
    fn stable_prompt_content_is_not_flagged() {
        let good = format!(
            "### DYNAMIC ENVIRONMENT CONTEXT:\n- **Current OS**: windows\n\
             - **Project Root**: C:\\Users\\HADES\\Documents\\vscodium-rust\n\
             - **Date**: {}\n- **File System Awareness**: AIM BRAIN active (4912 files indexed).\n",
            stable_date_stamp()
        );
        assert_eq!(prefix_volatility(&good), None);
        // Version numbers and line ranges must not look like clock times.
        assert_eq!(prefix_volatility("v1.2.3 lines 10-20 ratio 4:3"), None);
    }
}
