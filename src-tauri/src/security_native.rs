//! Rust-native security probes — lightweight alternatives to external CLI tools.

use crate::chunk_secrets::{dedupe_findings, scan_content, ChunkScanSummary, MAX_FILE_BYTES};
use crate::security_patterns::{resolve_url, script_src_re, source_map_re};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const XSS_CANARY: &str = "HADESXSS7k9m2p";
const MAX_XSS_PARAMS: usize = 24;
const MAX_CONCURRENT_FETCHES: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XssProbeHit {
    pub param: String,
    pub payload: String,
    pub reflected: bool,
    pub url: String,
    pub severity: String,
    pub bounty_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XssProbeSummary {
    pub target: String,
    pub params_tested: usize,
    pub hits: Vec<XssProbeHit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BountyScanSummary {
    pub chunk: ChunkScanSummary,
    pub xss: Option<XssProbeSummary>,
}

fn parse_query_params(url: &str) -> Vec<(String, String)> {
    let Some(qs) = url.split('?').nth(1) else {
        return vec![];
    };
    let qs = qs.split('#').next().unwrap_or(qs);
    qs.split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?.trim();
            if key.is_empty() {
                return None;
            }
            let val = parts.next().unwrap_or("").trim();
            Some((key.to_string(), val.to_string()))
        })
        .collect()
}

fn inject_param(url: &str, param: &str, value: &str) -> String {
    let (base, rest) = url.split_once('?').unwrap_or((url, ""));
    let fragment = rest.split('#').nth(1).unwrap_or("");
    let qs = rest.split('#').next().unwrap_or(rest);

    let mut pairs: Vec<(String, String)> = qs
        .split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            Some((parts.next()?.to_string(), parts.next().unwrap_or("").to_string()))
        })
        .collect();

    let mut found = false;
    for (k, v) in &mut pairs {
        if k == param {
            *v = value.to_string();
            found = true;
            break;
        }
    }
    if !found {
        pairs.push((param.to_string(), value.to_string()));
    }

    let query = pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");
    if fragment.is_empty() {
        format!("{base}?{query}")
    } else {
        format!("{base}?{query}#{fragment}")
    }
}

fn xss_payloads() -> &'static [&'static str] {
    &[
        XSS_CANARY,
        "<script>alert(1)</script>",
        "\"><svg/onload=alert(1)>",
        "'\"><img src=x onerror=alert(1)>",
    ]
}

pub async fn xss_probe_url(
    target_url: &str,
    client: &reqwest::Client,
) -> Result<XssProbeSummary, String> {
    let params = parse_query_params(target_url);
    if params.is_empty() {
        return Ok(XssProbeSummary {
            target: target_url.to_string(),
            params_tested: 0,
            hits: vec![],
        });
    }

    let mut hits = Vec::new();
    let tested = params.len().min(MAX_XSS_PARAMS);

    for (param, _) in params.iter().take(MAX_XSS_PARAMS) {
        for payload in xss_payloads() {
            let probe_url = inject_param(target_url, param, payload);
            let Ok(resp) = client
                .get(&probe_url)
                .header("User-Agent", "HADES-NativeXSS/1.0 (authorized testing)")
                .send()
                .await
            else {
                continue;
            };
            let Ok(body) = resp.text().await else { continue };

            let reflected = body.contains(payload)
                || (payload.contains(XSS_CANARY) && body.contains(XSS_CANARY));
            if reflected {
                hits.push(XssProbeHit {
                    param: param.clone(),
                    payload: payload.to_string(),
                    reflected: true,
                    url: probe_url,
                    severity: if payload.contains('<') || payload.contains('"') {
                        "HIGH".into()
                    } else {
                        "MEDIUM".into()
                    },
                    bounty_hint: "Reflected input in response — verify HTML context escaping; DalFox/Vega for full PoC.".into(),
                });
                break;
            }
        }
    }

    Ok(XssProbeSummary {
        target: target_url.to_string(),
        params_tested: tested,
        hits,
    })
}

pub async fn bounty_scan_url(
    origin_url: &str,
    client: &reqwest::Client,
    include_xss: bool,
) -> Result<BountyScanSummary, String> {
    let resp = client
        .get(origin_url)
        .header("User-Agent", "HADES-BountyScan/1.0")
        .send()
        .await
        .map_err(|e| format!("fetch {origin_url}: {e}"))?;
    let html = resp.text().await.map_err(|e| e.to_string())?;

    let script_urls: Vec<String> = script_src_re()
        .captures_iter(&html)
        .filter_map(|c| {
            let rel = c.get(1)?.as_str().to_string();
            Some(resolve_url(origin_url, &rel))
        })
        .collect();

    let mut files_scanned = 1usize;
    let mut bytes_scanned = html.len() as u64;
    let mut findings = scan_content(origin_url, &html);
    let mut source_maps_found = 0usize;

    let mut fetch_urls: Vec<String> = script_urls.iter().take(64).cloned().collect();
    let mut fetched = HashSet::new();

    while !fetch_urls.is_empty() && files_scanned < 80 {
        let batch: Vec<_> = fetch_urls
            .drain(..fetch_urls.len().min(MAX_CONCURRENT_FETCHES))
            .filter(|u| fetched.insert(u.clone()))
            .collect();

        let tasks: Vec<_> = batch
            .into_iter()
            .map(|url| {
                let c = client.clone();
                async move {
                    let body = c
                        .get(&url)
                        .header("User-Agent", "HADES-BountyScan/1.0")
                        .send()
                        .await
                        .ok()?
                        .text()
                        .await
                        .ok()?;
                    Some((url, body))
                }
            })
            .collect();

        for item in futures::future::join_all(tasks).await.into_iter().flatten() {
            let (url, body) = item;
            if body.len() as u64 > MAX_FILE_BYTES {
                continue;
            }
            bytes_scanned += body.len() as u64;
            files_scanned += 1;
            findings.extend(scan_content(&url, &body));

            if let Some(cap) = source_map_re()
                .captures(&body)
                .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            {
                let map_url = resolve_url(&url, &cap);
                if fetched.insert(map_url.clone()) {
                    fetch_urls.push(map_url);
                }
            }
        }
    }

    source_maps_found = findings.iter().filter(|f| f.kind == "source_map_url").count();
    dedupe_findings(&mut findings);

    let xss = if include_xss {
        Some(xss_probe_url(origin_url, client).await?)
    } else {
        None
    };

    Ok(BountyScanSummary {
        chunk: ChunkScanSummary {
            files_scanned,
            bytes_scanned,
            findings,
            source_maps_found,
            script_urls,
        },
        xss,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inject_param_replaces_value() {
        let u = inject_param("https://x.com/?q=1&page=2", "q", "PAYLOAD");
        assert!(u.contains("q=PAYLOAD"));
        assert!(u.contains("page=2"));
    }

    #[test]
    fn parse_params_from_url() {
        let p = parse_query_params("https://a.com/search?q=test&page=1");
        assert_eq!(p.len(), 2);
    }
}
