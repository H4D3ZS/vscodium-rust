//! Lightweight BFS crawler — extracts parametric paths for Vega injection modules.

use crate::vega::model::{FuzzableParam, ParamLocation, PathState};
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::sync::OnceLock;
use url::Url;

const DEFAULT_MAX_PAGES: usize = 48;
const DEFAULT_MAX_DEPTH: u32 = 3;

#[derive(Debug, Clone)]
pub struct CrawlConfig {
    pub seed: String,
    pub max_pages: usize,
    pub max_depth: u32,
    pub same_host_only: bool,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            seed: String::new(),
            max_pages: DEFAULT_MAX_PAGES,
            max_depth: DEFAULT_MAX_DEPTH,
            same_host_only: true,
        }
    }
}

fn href_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r#"(?i)href\s*=\s*['"]([^#'"]+)['"]"#).expect("href re"))
}

fn form_action_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?i)<form[^>]*action\s*=\s*['"]([^'"]*)['"][^>]*>"#).expect("form re")
    })
}

fn normalize_url(base: &Url, href: &str) -> Option<String> {
    let href = href.trim();
    if href.is_empty() || href.starts_with('#') || href.starts_with("javascript:") || href.starts_with("mailto:") {
        return None;
    }
    Url::parse(href)
        .or_else(|_| base.join(href))
        .ok()
        .map(|u| u.to_string())
}

fn host_allowed(seed_host: &str, candidate: &Url, same_host: bool) -> bool {
    if !same_host {
        return true;
    }
    candidate.host_str().map(|h| h == seed_host).unwrap_or(false)
}

fn path_states_from_url(raw: &str) -> Vec<PathState> {
    let Ok(parsed) = Url::parse(raw) else {
        return vec![];
    };
    let mut out = Vec::new();
    if let Some(q) = parsed.query() {
        let params: Vec<FuzzableParam> = q
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let name = parts.next()?.trim();
                if name.is_empty() {
                    return None;
                }
                let value = parts.next().unwrap_or("").trim();
                Some(FuzzableParam {
                    name: name.to_string(),
                    value: value.to_string(),
                    location: ParamLocation::Query,
                })
            })
            .collect();
        if !params.is_empty() {
            for (i, _) in params.iter().enumerate() {
                out.push(PathState {
                    uri: raw.to_string(),
                    is_post_target: false,
                    params: params.clone(),
                    fuzz_index: Some(i),
                    ..Default::default()
                });
            }
        }
    }
    if out.is_empty() {
        out.push(PathState {
            uri: raw.to_string(),
            is_post_target: false,
            params: vec![],
            fuzz_index: None,
            ..Default::default()
        });
    }
    out
}

pub async fn crawl(
    client: &reqwest::Client,
    config: &CrawlConfig,
) -> Result<Vec<PathState>, String> {
    let seed_url = Url::parse(&config.seed).map_err(|e| format!("bad seed URL: {e}"))?;
    let seed_host = seed_url
        .host_str()
        .ok_or_else(|| "seed URL has no host".to_string())?
        .to_string();

    let mut seen_urls = HashSet::new();
    let mut seen_paths = HashSet::new();
    let mut queue: VecDeque<(String, u32)> = VecDeque::new();
    queue.push_back((config.seed.clone(), 0));
    seen_urls.insert(config.seed.clone());

    let mut path_states = Vec::new();

    while let Some((url, depth)) = queue.pop_front() {
        if path_states.len() >= config.max_pages * 4 {
            break;
        }

        for ps in path_states_from_url(&url) {
            let key = format!("{}:{:?}", ps.uri, ps.fuzz_index);
            if seen_paths.insert(key) {
                path_states.push(ps);
            }
        }

        if depth >= config.max_depth {
            continue;
        }

        let Ok(resp) = client
            .get(&url)
            .header("User-Agent", "APEX-Vega-Crawler/1.0 (authorized testing)")
            .send()
            .await
        else {
            continue;
        };
        let Ok(body) = resp.text().await else { continue };
        let base = Url::parse(&url).ok();

        if let Some(base) = base.as_ref() {
            for cap in href_re().captures_iter(&body) {
                if let Some(href) = cap.get(1).map(|m| m.as_str()) {
                    if let Some(abs) = normalize_url(base, href) {
                        if let Ok(u) = Url::parse(&abs) {
                            if host_allowed(&seed_host, &u, config.same_host_only) && seen_urls.insert(abs.clone()) {
                                queue.push_back((abs, depth + 1));
                            }
                        }
                    }
                }
            }
            for cap in form_action_re().captures_iter(&body) {
                if let Some(action) = cap.get(1).map(|m| m.as_str()) {
                    let action = if action.is_empty() { url.as_str() } else { action };
                    if let Some(abs) = normalize_url(base, action) {
                        if let Ok(u) = Url::parse(&abs) {
                            if host_allowed(&seed_host, &u, config.same_host_only) {
                                let ps = PathState {
                                    uri: abs.clone(),
                                    is_post_target: true,
                                    params: vec![FuzzableParam {
                                        name: "body".into(),
                                        value: "1".into(),
                                        location: ParamLocation::Post,
                                    }],
                                    fuzz_index: Some(0),
                                    ..Default::default()
                                };
                                let key = format!("{}:post", ps.uri);
                                if seen_paths.insert(key) {
                                    path_states.push(ps);
                                }
                            }
                        }
                    }
                }
            }
        }

        if seen_urls.len() >= config.max_pages {
            break;
        }
    }

    Ok(path_states)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_query_params() {
        let states = path_states_from_url("http://x.test/page?id=1&name=foo");
        assert!(states.len() >= 2);
        assert!(states.iter().all(|s| s.is_parametric()));
    }
}
