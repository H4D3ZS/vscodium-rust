//! Upstream capability detection for the Kortex cache proxy.
//!
//! Different local servers expose different primitives:
//!   - llama.cpp (and Lemonade's `llamacpp` recipe): `/tokenize` +
//!     `/slots/{id}?action=save|restore` → full KDKVC KV-slot reuse (Tier `Kv`).
//!   - Ollama, Lemonade Ryzen-AI/NPU: no slot API → response cache (Tier
//!     `Response`, not yet implemented — safe passthrough for now).
//!
//! `resolve_tier` turns a requested [`CacheTier`] (usually `Auto`) into a
//! concrete tier by probing the upstream. The guiding rule from the design doc:
//! **a detection failure degrades to a LESS aggressive tier, never a wrong one.**

use std::path::Path;

use super::llamacpp::LlamaCppClient;
use super::types::CacheTier;

/// Filename used for the non-destructive slot-save probe. Written into the
/// server's slot-save directory (== `opts.slot_dir`) and removed immediately.
const PROBE_SLOT_FILE: &str = "__kortex_capability_probe__.slotbin";

/// Resolve a requested tier to a concrete one. `Auto` triggers a live probe;
/// any explicit choice is honored as-is (forcing `Kv` on a server without slots
/// is safe — runtime saves just no-op, they never corrupt output).
pub async fn resolve_tier(
    requested: CacheTier,
    client: &LlamaCppClient,
    slot_dir: &Path,
) -> CacheTier {
    match requested {
        CacheTier::Auto => probe_tier(client, slot_dir).await,
        explicit => explicit,
    }
}

/// Probe the upstream and classify it. Never returns `Auto`.
pub async fn probe_tier(client: &LlamaCppClient, slot_dir: &Path) -> CacheTier {
    // 1. Tokenizer is mandatory for KV reuse (we hash token IDs, not text).
    //    No tokenizer → this isn't a llama.cpp-class server → response tier.
    if let Err(e) = client.tokenize("kortex capability probe").await {
        tracing::info!("[kortex-cache] no /tokenize ({}); tier=Response", e);
        return CacheTier::Response;
    }

    // 2. Slot save/restore. Probe non-destructively: snapshot the (empty) slot
    //    to a scratch file, then delete it.
    match client.save_slot(PROBE_SLOT_FILE).await {
        Ok(()) => {
            let _ = std::fs::remove_file(slot_dir.join(PROBE_SLOT_FILE));
            let _ = client.erase_slot().await; // leave the slot as we found it
            tracing::info!("[kortex-cache] slot API available; tier=Kv");
            CacheTier::Kv
        }
        Err(e) => {
            let msg = e.to_string();
            if slot_error_means_unsupported(&msg) {
                tracing::info!("[kortex-cache] slot API unsupported ({}); tier=Response", msg);
                CacheTier::Response
            } else {
                // The endpoint exists but couldn't snapshot right now (commonly
                // an empty slot at probe time). Stay optimistic: /tokenize
                // already proved a llama.cpp-class server, and a real save
                // happens once tokens exist. A wrong guess here degrades to a
                // safe no-op (save fails → miss), never corruption.
                tracing::info!(
                    "[kortex-cache] slot probe inconclusive ({}); assuming tier=Kv",
                    msg
                );
                CacheTier::Kv
            }
        }
    }
}

/// Decide whether a `save_slot` error means the server lacks slot support (as
/// opposed to a transient/state error like an empty slot). Kept pure so it can
/// be unit-tested without a live server — this is the fragile,
/// version-sensitive part of detection.
pub fn slot_error_means_unsupported(err_msg: &str) -> bool {
    let m = err_msg.to_ascii_lowercase();
    // Capability signals: llama.cpp built without slot save, or a server that
    // doesn't route /slots at all.
    const UNSUPPORTED_MARKERS: &[&str] = &[
        "slot-save-path",
        "not support",
        "unsupported",
        "not enabled",
        "disabled",
        "not implemented",
        "unknown action",
        "no route",
        "not found",
        " 404",
        " 405",
        " 501",
    ];
    UNSUPPORTED_MARKERS.iter().any(|marker| m.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_unsupported_slot_errors() {
        // Messages a server WITHOUT slot support would produce.
        for msg in [
            "/slots/0?save failed: 501 slot save is not supported; start with --slot-save-path",
            "/slots/0?save failed: 404 Not Found",
            "/slots/0?save failed: 405 Method Not Allowed",
            "slot restore is disabled on this server",
            "error: unknown action",
        ] {
            assert!(
                slot_error_means_unsupported(msg),
                "should be classified unsupported: {msg}"
            );
        }
    }

    #[test]
    fn does_not_classify_transient_state_errors_as_unsupported() {
        // Endpoint EXISTS, but the save failed for a state reason — we must stay
        // optimistic (Kv), not demote the server to Response.
        for msg in [
            "/slots/0?save failed: 400 slot is empty",
            "/slots/0?save failed: 500 internal error writing file",
            "connection reset by peer",
            "timed out",
        ] {
            assert!(
                !slot_error_means_unsupported(msg),
                "should NOT be classified unsupported: {msg}"
            );
        }
    }

    #[tokio::test]
    async fn resolve_tier_honors_explicit_choices_without_probing() {
        // Unreachable upstream — if these probed, they'd hit the network. They
        // must return the explicit choice immediately instead.
        let client = LlamaCppClient::new("http://127.0.0.1:1", 0);
        let dir = std::env::temp_dir();
        assert_eq!(resolve_tier(CacheTier::Off, &client, &dir).await, CacheTier::Off);
        assert_eq!(resolve_tier(CacheTier::Kv, &client, &dir).await, CacheTier::Kv);
        assert_eq!(
            resolve_tier(CacheTier::Response, &client, &dir).await,
            CacheTier::Response
        );
    }
}
