//! Process-global stash of full message/turn text that history compaction
//! replaced with a short summary, so a `recall({"id": "<id>"})` call can bring
//! the original back. Same shape as `stash` but keyed by an opaque id and
//! bounded on total bytes rather than a model count.

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

/// ~2 MB of stashed turns before the oldest are dropped.
const MAX_BYTES: usize = 2 * 1024 * 1024;

#[derive(Default)]
struct TurnStash {
    text: HashMap<String, String>,
    order: VecDeque<String>,
    bytes: usize,
}

static STASH: Mutex<Option<TurnStash>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut TurnStash) -> R) -> R {
    let mut g = STASH.lock().unwrap_or_else(|p| p.into_inner());
    f(g.get_or_insert_with(TurnStash::default))
}

/// Stash `full` under `id` (a caller-chosen short key). Overwrites an existing id.
pub fn put(id: &str, full: &str) {
    if id.is_empty() || full.is_empty() {
        return;
    }
    with(|s| {
        if let Some(old) = s.text.insert(id.to_string(), full.to_string()) {
            s.bytes = s.bytes.saturating_sub(old.len());
            if let Some(pos) = s.order.iter().position(|k| k == id) {
                s.order.remove(pos);
            }
        }
        s.order.push_back(id.to_string());
        s.bytes += full.len();
        while s.bytes > MAX_BYTES {
            match s.order.pop_front() {
                Some(k) => {
                    if let Some(v) = s.text.remove(&k) {
                        s.bytes = s.bytes.saturating_sub(v.len());
                    }
                }
                None => break,
            }
        }
    });
}

/// The full text stashed under `id`, if still resident.
pub fn get(id: &str) -> Option<String> {
    with(|s| s.text.get(id).cloned())
}

/// A short, monotonically-unique id for a compacted turn (`t1`, `t2`, …).
pub fn next_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(1);
    format!("t{}", N.fetch_add(1, Ordering::Relaxed))
}

#[cfg(test)]
pub(crate) fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    static L: std::sync::Mutex<()> = std::sync::Mutex::new(());
    L.lock().unwrap_or_else(|p| p.into_inner())
}

#[cfg(test)]
pub fn clear() {
    with(|s| {
        s.text.clear();
        s.order.clear();
        s.bytes = 0;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get() {
        let _g = super::test_lock();
        clear();
        put("t1", "hello world");
        assert_eq!(get("t1").as_deref(), Some("hello world"));
        assert_eq!(get("nope"), None);
    }

    #[test]
    fn overwrite_updates_bytes() {
        let _g = super::test_lock();
        clear();
        put("t1", "aaaa");
        put("t1", "bb");
        assert_eq!(get("t1").as_deref(), Some("bb"));
    }

    #[test]
    fn byte_budget_evicts_oldest() {
        let _g = super::test_lock();
        clear();
        let big = "x".repeat(MAX_BYTES / 2 + 1);
        put("a", &big);
        put("b", &big);
        put("c", &big); // pushes total over budget → "a" (and maybe "b") evicted
        assert!(get("a").is_none());
        assert!(get("c").is_some());
    }

    #[test]
    fn ids_are_unique_and_ordered() {
        let a = next_id();
        let b = next_id();
        assert_ne!(a, b);
        assert!(a.starts_with('t') && b.starts_with('t'));
    }

    #[test]
    fn empty_inputs_noop() {
        let _g = super::test_lock();
        clear();
        put("", "x");
        put("t", "");
        assert!(get("").is_none());
        assert!(get("t").is_none());
    }
}
