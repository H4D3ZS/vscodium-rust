//! Shared env-flag resolution: explicit `1`/`true`/`on` forces on, explicit
//! `0`/`false`/`off` forces off, unset falls back to `default`.
//!
//! Used by the compute-cost/reliability cluster so each lever can ship
//! **on by default at its already-local-scoped call site** (see
//! `docs/kortex-compute-cost.md` "Local by default") while a user can still
//! force it off (or force it on somewhere it defaults off) with one env var —
//! never silently unreachable either way.

/// Resolve a tri-state env flag against `default_when_unset`.
pub fn on(key: &str, default_when_unset: bool) -> bool {
    match std::env::var(key).ok().as_deref() {
        Some("1") | Some("true") | Some("on") => true,
        Some("0") | Some("false") | Some("off") => false,
        _ => default_when_unset,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static L: std::sync::Mutex<()> = std::sync::Mutex::new(());
        L.lock().unwrap_or_else(|p| p.into_inner())
    }

    #[test]
    fn unset_falls_back_to_default() {
        let _g = env_lock();
        std::env::remove_var("KORTEX_TEST_FLAG_A");
        assert!(on("KORTEX_TEST_FLAG_A", true));
        assert!(!on("KORTEX_TEST_FLAG_A", false));
    }

    #[test]
    fn explicit_on_wins_over_false_default() {
        let _g = env_lock();
        std::env::set_var("KORTEX_TEST_FLAG_B", "1");
        assert!(on("KORTEX_TEST_FLAG_B", false));
        std::env::remove_var("KORTEX_TEST_FLAG_B");
    }

    #[test]
    fn explicit_off_wins_over_true_default() {
        let _g = env_lock();
        std::env::set_var("KORTEX_TEST_FLAG_C", "off");
        assert!(!on("KORTEX_TEST_FLAG_C", true));
        std::env::remove_var("KORTEX_TEST_FLAG_C");
    }
}
