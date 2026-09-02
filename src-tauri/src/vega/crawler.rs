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

/// Capture each `<form ...>...</form>` block: group 1 = form tag attributes,
/// group 2 = inner HTML (the inputs).
fn form_block_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r#"(?is)<form\b([^>]*)>(.*?)</form>"#).expect("form block re"))
}

/// Capture each opening `<input|textarea|select ...>` tag: group 1 = tag name,
/// group 2 = its attributes.
fn input_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?is)<(input|textarea|select)\b([^>]*)>"#).expect("input re")
    })
}

/// Capture every `name="value"` (single/double/bare) attribute pair in a tag.
fn attr_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?is)([\w:.-]+)\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s'">]+))"#)
            .expect("attr re")
    })
}

/// Parse a tag's attribute string into a lowercase-keyed map (first wins).
fn parse_attrs(s: &str) -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    for c in attr_re().captures_iter(s) {
        let key = c.get(1).map(|x| x.as_str().to_lowercase()).unwrap_or_default();
        if key.is_empty() {
            continue;
        }
        let val = c
            .get(2)
            .or_else(|| c.get(3))
            .or_else(|| c.get(4))
            .map(|x| x.as_str().to_string())
            .unwrap_or_default();
        m.entry(key).or_insert(val);
    }
    m
}

/// A single `<input>`-like field harvested from a form.
struct FormField {
    name: String,
    value: String,
    /// True for text-like inputs we should mutate; false for submit/hidden/etc.
    fuzzable: bool,
}

/// Extract the fillable fields from a form's inner HTML, in document order,
/// deduped by name. Submit/button/image/reset/file inputs are kept (so the
/// server's branch logic still fires) but are not marked fuzzable. Hidden
/// fields (CSRF tokens, etc.) are sent verbatim but not fuzzed.
fn extract_form_fields(inner: &str) -> Vec<FormField> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for cap in input_re().captures_iter(inner) {
        let attrs = parse_attrs(cap.get(2).map(|m| m.as_str()).unwrap_or(""));
        let Some(name) = attrs.get("name").map(|s| s.trim().to_string()) else {
            continue;
        };
        if name.is_empty() || !seen.insert(name.clone()) {
            continue;
        }
        let itype = attrs.get("type").map(|s| s.to_lowercase()).unwrap_or_default();
        let non_fuzzable = matches!(
            itype.as_str(),
            "submit" | "button" | "image" | "reset" | "file" | "hidden"
        );
        let value = attrs.get("value").cloned().unwrap_or_default();
        out.push(FormField {
            name,
            // Seed empty fuzzable fields with "1" so there's a baseline value to
            // diff against; keep submit/hidden values exactly as authored.
            value: if value.is_empty() && !non_fuzzable {
                "1".into()
            } else {
                value
            },
            fuzzable: !non_fuzzable,
        });
    }
    out
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
            for cap in form_block_re().captures_iter(&body) {
                let form_attrs = parse_attrs(cap.get(1).map(|m| m.as_str()).unwrap_or(""));
                let inner = cap.get(2).map(|m| m.as_str()).unwrap_or("");

                let is_post = form_attrs
                    .get("method")
                    .map(|m| m.eq_ignore_ascii_case("post"))
                    .unwrap_or(false);
                let action = form_attrs
                    .get("action")
                    .filter(|a| !a.trim().is_empty())
                    .map(|a| a.as_str())
                    .unwrap_or(url.as_str());

                let Some(abs) = normalize_url(base, action) else { continue };
                let Ok(u) = Url::parse(&abs) else { continue };
                if !host_allowed(&seed_host, &u, config.same_host_only) {
                    continue;
                }

                let fields = extract_form_fields(inner);
                if fields.is_empty() {
                    continue;
                }
                let location = if is_post {
                    ParamLocation::Post
                } else {
                    ParamLocation::Query
                };
                // The full field set is sent with every request; we only swap the
                // one being fuzzed. Build it once, then emit a PathState per
                // fuzzable field so each gets its own injection pass.
                let params: Vec<FuzzableParam> = fields
                    .iter()
                    .map(|f| FuzzableParam {
                        name: f.name.clone(),
                        value: f.value.clone(),
                        location,
                    })
                    .collect();

                for (i, f) in fields.iter().enumerate() {
                    if !f.fuzzable {
                        continue;
                    }
                    let tag = if is_post { "post" } else { "get" };
                    let key = format!("{}:{}:{}", abs, tag, f.name);
                    if !seen_paths.insert(key) {
                        continue;
                    }
                    path_states.push(PathState {
                        uri: abs.clone(),
                        is_post_target: is_post,
                        params: params.clone(),
                        fuzz_index: Some(i),
                        ..Default::default()
                    });
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
