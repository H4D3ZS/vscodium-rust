//! Intruder / Automate — payload-set fuzzing backend.
//!
//! Takes a base request with a `§` marker, substitutes each payload from a set,
//! fires them concurrently, and returns a results table. The frontend sorts by
//! length/status to spot the anomaly (the classic Burp Intruder workflow).
//! Authorized testing only.

use crate::repeater::{send, ManualRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Placeholder marker dropped into url/body/header values where payloads go.
pub const MARKER: &str = "§";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntruderRequest {
    pub method: String,
    /// May contain `§` to mark the injection point.
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: String,
    pub payloads: Vec<String>,
    /// Optional substring to flag in the response body (grep-match).
    #[serde(default)]
    pub grep: Option<String>,
    #[serde(default)]
    pub follow_redirects: bool,
    #[serde(default)]
    pub concurrency: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntruderHit {
    pub index: usize,
    pub payload: String,
    pub status: u16,
    pub length: usize,
    pub duration_ms: u64,
    pub grep_match: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntruderResult {
    pub total: usize,
    pub hits: Vec<IntruderHit>,
    /// Indices that deviate from the modal (status, length) baseline — the
    /// "interesting" rows worth a manual look.
    pub anomalies: Vec<usize>,
}

fn sub(s: &str, payload: &str) -> String {
    s.replace(MARKER, payload)
}

const MAX_PAYLOADS: usize = 5_000;

pub async fn run(req: IntruderRequest) -> Result<IntruderResult, String> {
    if req.payloads.is_empty() {
        return Err("no payloads provided".into());
    }
    let payloads: Vec<String> = req.payloads.into_iter().take(MAX_PAYLOADS).collect();
    let total = payloads.len();
    let concurrency = req.concurrency.unwrap_or(10).clamp(1, 50);
    let sem = Arc::new(Semaphore::new(concurrency));
    let grep = req.grep.filter(|g| !g.is_empty()).map(Arc::new);

    let mut tasks = Vec::with_capacity(total);
    for (index, payload) in payloads.into_iter().enumerate() {
        let permit = sem.clone();
        let method = req.method.clone();
        let url = sub(&req.url, &payload);
        let headers: Vec<(String, String)> = req
            .headers
            .iter()
            .map(|(k, v)| (k.clone(), sub(v, &payload)))
            .collect();
        let body = sub(&req.body, &payload);
        let follow = req.follow_redirects;
        let grep = grep.clone();

        tasks.push(tokio::spawn(async move {
            let _p = permit.acquire_owned().await.ok();
            let mr = ManualRequest {
                method,
                url,
                headers,
                body,
                follow_redirects: follow,
            };
            match send(mr).await {
                Ok(resp) => IntruderHit {
                    index,
                    payload,
                    status: resp.status,
                    length: resp.body_bytes,
                    duration_ms: resp.duration_ms,
                    grep_match: grep
                        .as_ref()
                        .map(|g| resp.body.contains(g.as_str()))
                        .unwrap_or(false),
                    error: None,
                },
                Err(e) => IntruderHit {
                    index,
                    payload,
                    status: 0,
                    length: 0,
                    duration_ms: 0,
                    grep_match: false,
                    error: Some(e),
                },
            }
        }));
    }

    let mut hits = Vec::with_capacity(total);
    for t in tasks {
        if let Ok(h) = t.await {
            hits.push(h);
        }
    }
    hits.sort_by_key(|h| h.index);

    let anomalies = detect_anomalies(&hits);
    Ok(IntruderResult {
        total,
        hits,
        anomalies,
    })
}

/// Flag rows whose (status, length) differs from the most common pair, plus any
/// grep match or error. This is the cheap "find the needle" heuristic.
fn detect_anomalies(hits: &[IntruderHit]) -> Vec<usize> {
    use std::collections::HashMap;
    let mut freq: HashMap<(u16, usize), usize> = HashMap::new();
    for h in hits {
        *freq.entry((h.status, h.length)).or_insert(0) += 1;
    }
    let modal = freq.iter().max_by_key(|(_, c)| **c).map(|(k, _)| *k);

    hits.iter()
        .filter(|h| {
            h.grep_match
                || h.error.is_some()
                || modal.map_or(false, |m| (h.status, h.length) != m)
        })
        .map(|h| h.index)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_substitution() {
        assert_eq!(sub("/u?id=§", "5"), "/u?id=5");
        assert_eq!(sub("no marker", "x"), "no marker");
    }

    #[test]
    fn anomaly_is_the_outlier() {
        let hits = vec![
            mk(0, 200, 100),
            mk(1, 200, 100),
            mk(2, 200, 100),
            mk(3, 500, 4000),
        ];
        assert_eq!(detect_anomalies(&hits), vec![3]);
    }

    fn mk(index: usize, status: u16, length: usize) -> IntruderHit {
        IntruderHit {
            index,
            payload: format!("p{index}"),
            status,
            length,
            duration_ms: 1,
            grep_match: false,
            error: None,
        }
    }
}
