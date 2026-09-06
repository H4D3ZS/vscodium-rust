//! Process-global stash of the full tool schemas that `compress_openai_request`
//! compacted away, so an `expand({"tool": name})` call can be answered without a
//! round-trip to the client — the "context lives compressed, expands on demand"
//! half of the harness.
//!
//! Keyed by model id (the one field present on both sides: the proxy that
//! compacts and the agent loop that dispatches `expand`). Bounded so a long-
//! running process with many models can't grow it without limit.
//!
//! Two maps:
//!   * `schemas`  — model → { tool_name → full JSON schema }
//!   * `sticky`   — model → { tool_names the model asked to expand }
//! `is_sticky` is read back by `compress_openai_request` so an expanded tool
//! stays inline (full schema) for the rest of that model's session.

use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Mutex;

/// Max distinct model ids retained. Old ids are evicted FIFO.
const MAX_MODELS: usize = 8;

#[derive(Default)]
struct Stash {
    schemas: HashMap<String, HashMap<String, Value>>,
    sticky: HashMap<String, HashSet<String>>,
    /// Insertion order of model ids, for FIFO eviction.
    order: VecDeque<String>,
}

static STASH: Mutex<Option<Stash>> = Mutex::new(None);

fn with_stash<R>(f: impl FnOnce(&mut Stash) -> R) -> R {
    let mut guard = STASH.lock().unwrap_or_else(|p| p.into_inner());
    f(guard.get_or_insert_with(Stash::default))
}

fn touch(s: &mut Stash, model: &str) {
    if let Some(pos) = s.order.iter().position(|m| m == model) {
        s.order.remove(pos);
    }
    s.order.push_back(model.to_string());
    while s.order.len() > MAX_MODELS {
        if let Some(old) = s.order.pop_front() {
            s.schemas.remove(&old);
            s.sticky.remove(&old);
        }
    }
}

/// Record the schemas compacted for `model`. Merges into any existing entry so
/// a later request with a different tool subset doesn't drop earlier ones.
pub fn put(model: &str, compacted: &HashMap<String, Value>) {
    if model.is_empty() || compacted.is_empty() {
        return;
    }
    with_stash(|s| {
        touch(s, model);
        let entry = s.schemas.entry(model.to_string()).or_default();
        for (name, schema) in compacted {
            entry.insert(name.clone(), schema.clone());
        }
    });
}

/// Full schema for `tool` under `model`, if it was compacted.
pub fn get(model: &str, tool: &str) -> Option<Value> {
    with_stash(|s| s.schemas.get(model).and_then(|m| m.get(tool)).cloned())
}

/// Mark `tool` sticky for `model` — `compress_openai_request` will keep it
/// inline (uncompacted) on subsequent requests for that model.
pub fn mark_sticky(model: &str, tool: &str) {
    if model.is_empty() || tool.is_empty() {
        return;
    }
    with_stash(|s| {
        touch(s, model);
        s.sticky.entry(model.to_string()).or_default().insert(tool.to_string());
    });
}

/// Has the model asked to expand this tool before?
pub fn is_sticky(model: &str, tool: &str) -> bool {
    with_stash(|s| s.sticky.get(model).is_some_and(|set| set.contains(tool)))
}

/// Test/maintenance: drop everything.
#[cfg(test)]
pub fn clear() {
    with_stash(|s| {
        s.schemas.clear();
        s.sticky.clear();
        s.order.clear();
    });
}

/// Serialises tests that mutate the process-global stash (the test runner is
/// multi-threaded). Callers: `stash::tests` and the `kortex_harness::tests`
/// cases that go through `compress_openai_request` / `expand_tool`.
#[cfg(test)]
pub(crate) fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    static L: std::sync::Mutex<()> = std::sync::Mutex::new(());
    L.lock().unwrap_or_else(|p| p.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn map(pairs: &[(&str, Value)]) -> HashMap<String, Value> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
    }

    #[test]
    fn put_get_roundtrip() {
        let _g = super::test_lock();
        clear();
        put("m1", &map(&[("web_fetch", json!({"a": 1})), ("grep", json!({"b": 2}))]));
        assert_eq!(get("m1", "web_fetch"), Some(json!({"a": 1})));
        assert_eq!(get("m1", "missing"), None);
        assert_eq!(get("other", "web_fetch"), None);
    }

    #[test]
    fn put_merges() {
        let _g = super::test_lock();
        clear();
        put("m1", &map(&[("a", json!(1))]));
        put("m1", &map(&[("b", json!(2))]));
        assert_eq!(get("m1", "a"), Some(json!(1)));
        assert_eq!(get("m1", "b"), Some(json!(2)));
    }

    #[test]
    fn sticky_flow() {
        let _g = super::test_lock();
        clear();
        assert!(!is_sticky("m1", "web_fetch"));
        mark_sticky("m1", "web_fetch");
        assert!(is_sticky("m1", "web_fetch"));
        assert!(!is_sticky("m1", "grep"));
        assert!(!is_sticky("m2", "web_fetch"));
    }

    #[test]
    fn fifo_eviction_past_max_models() {
        let _g = super::test_lock();
        clear();
        for i in 0..(MAX_MODELS + 3) {
            put(&format!("m{i}"), &map(&[("t", json!(i))]));
        }
        assert_eq!(get("m0", "t"), None, "oldest evicted");
        assert_eq!(get("m1", "t"), None);
        assert!(get(&format!("m{}", MAX_MODELS + 2), "t").is_some(), "newest kept");
    }

    #[test]
    fn empty_inputs_are_noops() {
        let _g = super::test_lock();
        clear();
        put("", &map(&[("t", json!(1))]));
        put("m1", &HashMap::new());
        assert_eq!(get("m1", "t"), None);
        mark_sticky("", "t");
        assert!(!is_sticky("", "t"));
    }
}
