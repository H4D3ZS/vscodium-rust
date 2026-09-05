//! Web search and crawl — LLM-friendly markdown output.
//!
//! Provides clean markdown generation from HTML, structured data extraction,
//! and deep crawling capabilities.

use anyhow::{anyhow, Result};
use regex::Regex;
use serde_json::{json, Value};
use std::time::Duration;
use url::Url;

const WEB_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug, Clone)]
pub struct CrawlResult {
    pub url: String,
    pub title: String,
    pub markdown: String,
    pub fit_markdown: String,
    pub raw_html: String,
    pub metadata: Value,
    pub links_count: usize,
    pub images_count: usize,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CrawlOptions {
    pub fit_markdown: bool,
    pub fit_query: String,
    pub extract_metadata: bool,
    pub max_depth: usize,
    pub max_pages: usize,
}

pub struct Crawl4AI {
    client: reqwest::Client,
}

impl Crawl4AI {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(WEB_UA)
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;
        Ok(Self { client })
    }

    pub async fn crawl(&self, url: &str, options: CrawlOptions) -> Result<CrawlResult> {
        let resp = self.client
            .get(url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            .send()
            .await
            .map_err(|e| anyhow!("Request failed for {}: {}", url, e))?;

        let final_url = resp.url().to_string();
        let status = resp.status();
        if !status.is_success() {
            return Ok(CrawlResult {
                url: final_url, title: String::new(), markdown: String::new(),
                fit_markdown: String::new(), raw_html: String::new(), metadata: json!({}),
                links_count: 0, images_count: 0, success: false,
                error: Some(format!("HTTP {}", status)),
            });
        }

        let html = resp.text().await.map_err(|e| anyhow!("Read failed: {}", e))?;
        let title = extract_title(&html);
        let metadata = if options.extract_metadata { extract_metadata(&html) } else { json!({}) };
        let links_count = count_links(&html);
        let images_count = count_images(&html);
        let markdown = html_to_markdown(&html);
        let fit_markdown = if options.fit_markdown && !options.fit_query.is_empty() {
            generate_fit_markdown(&markdown, &options.fit_query)
        } else {
            markdown.clone()
        };

        Ok(CrawlResult {
            url: final_url, title, markdown, fit_markdown, raw_html: html, metadata,
            links_count, images_count, success: true, error: None,
        })
    }

    pub async fn crawl_batch(&self, urls: &[String], options: CrawlOptions) -> Vec<CrawlResult> {
        let mut results = Vec::with_capacity(urls.len());
        for url in urls {
            match self.crawl(url, options.clone()).await {
                Ok(r) => results.push(r),
                Err(e) => results.push(CrawlResult {
                    url: url.clone(), title: String::new(), markdown: String::new(),
                    fit_markdown: String::new(), raw_html: String::new(), metadata: json!({}),
                    links_count: 0, images_count: 0, success: false, error: Some(e.to_string()),
                }),
            }
        }
        results
    }

    pub async fn deep_crawl(&self, start_url: &str, max_depth: usize, max_pages: usize, options: CrawlOptions) -> Result<Vec<CrawlResult>> {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut results = Vec::new();
        queue.push_back((start_url.to_string(), 0));
        visited.insert(start_url.to_string());

        let base_domain = Url::parse(start_url)
            .ok().and_then(|u| u.host_str().map(|s| s.to_string()))
            .unwrap_or_default();

        while let Some((url, depth)) = queue.pop_front() {
            if results.len() >= max_pages || depth > max_depth { continue; }
            let result = self.crawl(&url, options.clone()).await.unwrap_or_else(|e| CrawlResult {
                url: url.clone(), title: String::new(), markdown: String::new(),
                fit_markdown: String::new(), raw_html: String::new(), metadata: json!({}),
                links_count: 0, images_count: 0, success: false, error: Some(e.to_string()),
            });
            if depth < max_depth {
                let html = &result.raw_html;
                let link_re = Regex::new(r#"(?i)href="(https?://[^"]*)""#).unwrap();
                for caps in link_re.captures_iter(html) {
                    if let Some(href) = caps.get(1) {
                        let href_str = href.as_str().to_string();
                        if !visited.contains(&href_str) {
                            if let Ok(parsed) = Url::parse(&href_str) {
                                if parsed.host_str() == Some(&base_domain) {
                                    visited.insert(href_str.clone());
                                    queue.push_back((href_str, depth + 1));
                                }
                            }
                        }
                    }
                }
            }
            results.push(result);
        }
        Ok(results)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML → Markdown
// ═══════════════════════════════════════════════════════════════════════════

fn html_to_markdown(html: &str) -> String {
    let cleaned = remove_noise_tags(html);
    let mut md = String::new();
    let mut in_pre = false;

    for line in cleaned.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !md.ends_with("\n\n") { md.push('\n'); }
            continue;
        }

        // Pre/code blocks
        if trimmed.starts_with("<pre") || trimmed.starts_with("<code") {
            in_pre = true;
            md.push_str("```\n");
            continue;
        }
        if trimmed.starts_with("</pre") || trimmed.starts_with("</code") {
            in_pre = false;
            md.push_str("\n```\n");
            continue;
        }
        if in_pre {
            md.push_str(&strip_tags(trimmed));
            md.push('\n');
            continue;
        }

        // Headings
        if let Some(level) = extract_heading_level(trimmed) {
            let text = extract_text_content(trimmed);
            if !text.is_empty() {
                md.push_str(&"#".repeat(level));
                md.push(' ');
                md.push_str(&text);
                md.push_str("\n\n");
            }
            continue;
        }

        // Lists
        if trimmed.starts_with("<li") {
            let text = extract_text_content(trimmed);
            md.push_str("- ");
            md.push_str(&text);
            md.push('\n');
            continue;
        }

        // Tables
        if trimmed.starts_with("<table") || trimmed.starts_with("</table") {
            continue; // Handled separately
        }
        if trimmed.starts_with("<tr") {
            md.push_str("| ");
            continue;
        }
        if trimmed.starts_with("<td") || trimmed.starts_with("<th") {
            let text = extract_text_content(trimmed);
            md.push_str(&text);
            md.push_str(" | ");
            continue;
        }

        // Blockquotes
        if trimmed.starts_with("<blockquote") {
            let text = extract_text_content(trimmed);
            for line in text.lines() {
                md.push_str("> ");
                md.push_str(line);
                md.push('\n');
            }
            md.push('\n');
            continue;
        }

        // Paragraphs
        if trimmed.starts_with("<p") || trimmed.starts_with("<div") {
            let text = extract_text_content(trimmed);
            if !text.is_empty() {
                md.push_str(&text);
                md.push_str("\n\n");
            }
            continue;
        }

        // Links
        if trimmed.starts_with("<a") {
            let (href, text) = extract_link(trimmed);
            if !href.is_empty() && !text.is_empty() {
                md.push_str(&format!("[{}]({})", text, href));
            }
            continue;
        }

        // Images
        if trimmed.starts_with("<img") {
            let (src, alt) = extract_image(trimmed);
            if !src.is_empty() {
                md.push_str(&format!("![{}]({})", alt, src));
                md.push('\n');
            }
            continue;
        }

        // HR
        if trimmed.starts_with("<hr") {
            md.push_str("\n---\n\n");
            continue;
        }

        // Skip other HTML tags
        if trimmed.starts_with('<') { continue; }

        // Plain text
        let text = strip_tags(trimmed);
        if !text.is_empty() {
            md.push_str(&text);
            md.push('\n');
        }
    }

    let re = Regex::new(r"\n{3,}").unwrap();
    let result = re.replace_all(&md, "\n\n");
    result.trim().to_string()
}

fn remove_noise_tags(html: &str) -> String {
    let mut s = html.to_string();
    for tag in &["script", "style", "noscript", "svg", "head", "header", "footer", "nav"] {
        let re = Regex::new(&format!(r"(?is)<{0}[^>]*>.*?</{0}>", tag)).unwrap();
        s = re.replace_all(&s, " ").into_owned();
    }
    s
}

fn strip_tags(s: &str) -> String {
    let re = Regex::new(r"(?s)<[^>]*>").unwrap();
    let stripped = re.replace_all(s, " ");
    let decoded = decode_html_entities(&stripped);
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_html_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("&#8211;", "\u{2013}")
        .replace("&#8212;", "\u{2014}")
        .replace("&#8230;", "\u{2026}")
}

fn extract_heading_level(s: &str) -> Option<usize> {
    let re = Regex::new(r"(?i)<h([1-6])").ok()?;
    let caps = re.captures(s)?;
    caps.get(1)?.as_str().parse().ok()
}

fn extract_text_content(s: &str) -> String {
    let re = Regex::new(r"(?s)>(.*?)<").ok().unwrap();
    let mut text = String::new();
    for caps in re.captures_iter(s) {
        let inner = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let stripped = strip_tags(inner);
        if !stripped.is_empty() {
            if !text.is_empty() { text.push(' '); }
            text.push_str(&stripped);
        }
    }
    text
}

fn extract_link(s: &str) -> (String, String) {
    let text = extract_text_content(s);
    let href_re = Regex::new(r#"(?i)href="([^"]*)""#).ok().unwrap();
    let href = href_re.captures(s)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_default();
    (href, text)
}

fn extract_image(s: &str) -> (String, String) {
    let src_re = Regex::new(r#"(?i)src="([^"]*)""#).ok().unwrap();
    let alt_re = Regex::new(r#"(?i)alt="([^"]*)""#).ok().unwrap();
    let src = src_re.captures(s).and_then(|c| c.get(1)).map(|m| m.as_str().to_string()).unwrap_or_default();
    let alt = alt_re.captures(s).and_then(|c| c.get(1)).map(|m| m.as_str().to_string()).unwrap_or_default();
    (src, alt)
}

fn count_links(html: &str) -> usize {
    let re = Regex::new(r#"(?i)<a\s+[^>]*href="#).unwrap();
    re.find_iter(html).count()
}

fn count_images(html: &str) -> usize {
    let re = Regex::new(r#"(?i)<img\s+"#).unwrap();
    re.find_iter(html).count()
}

// ═══════════════════════════════════════════════════════════════════════════
// Fit Markdown (BM25-like filtering)
// ═══════════════════════════════════════════════════════════════════════════

fn generate_fit_markdown(markdown: &str, query: &str) -> String {
    let query_words: Vec<String> = query.to_lowercase().split_whitespace()
        .filter(|w| w.len() > 2).map(|s| s.to_string()).collect();
    if query_words.is_empty() { return markdown.to_string(); }

    let paragraphs: Vec<&str> = markdown.split("\n\n").collect();
    let mut scored: Vec<(usize, f64)> = paragraphs.iter().enumerate()
        .map(|(i, p)| (i, score_paragraph(p, &query_words)))
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut selected = Vec::new();
    for (i, score) in &scored {
        if *score > 0.3 || selected.len() < 3 {
            selected.push(*i);
        }
    }
    if selected.is_empty() {
        selected = (0..paragraphs.len().min(3)).collect();
    }
    selected.sort();
    selected.iter().filter_map(|i| paragraphs.get(*i)).copied().collect::<Vec<_>>().join("\n\n")
}

fn score_paragraph(paragraph: &str, query_words: &[String]) -> f64 {
    let lower = paragraph.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    let total = words.len() as f64;
    if total == 0.0 { return 0.0; }
    let mut score = 0.0;
    for qw in query_words {
        let count = words.iter().filter(|w| w.contains(qw.as_str())).count() as f64;
        score += (count / total) * if lower.contains(qw.as_str()) { 1.5 } else { 1.0 };
    }
    score / query_words.len() as f64
}

// ═══════════════════════════════════════════════════════════════════════════
// Metadata extraction
// ═══════════════════════════════════════════════════════════════════════════

fn extract_title(html: &str) -> String {
    let re = Regex::new(r"(?is)<title[^>]*>(.*?)</title>").unwrap();
    re.captures(html).and_then(|c| c.get(1)).map(|m| strip_tags(m.as_str())).unwrap_or_default()
}

fn extract_metadata(html: &str) -> Value {
    let mut meta = json!({});
    let re = Regex::new(r#"(?i)<meta\s+([^>]*?)/?>"#).unwrap();
    for caps in re.captures_iter(html) {
        let attrs = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let name = extract_attr(attrs, "name").or_else(|| extract_attr(attrs, "property"));
        let content = extract_attr(attrs, "content");
        if let (Some(n), Some(c)) = (name, content) {
            if !n.is_empty() && !c.is_empty() { meta[&n] = json!(c); }
        }
    }
    meta
}

fn extract_attr(attrs: &str, name: &str) -> Option<String> {
    let pattern = format!(r#"(?i){}="([^"]*)""#, name);
    Regex::new(&pattern).ok()?.captures(attrs)?.get(1).map(|m| m.as_str().to_string())
}

// ═══════════════════════════════════════════════════════════════════════════
// Tool functions
// ═══════════════════════════════════════════════════════════════════════════

pub async fn crawl_url_tool(args: &Value) -> Result<Value> {
    let url = args.get("url").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing url"))?;
    let options = CrawlOptions {
        fit_markdown: args.get("fit_markdown").and_then(|v| v.as_bool()).unwrap_or(false),
        fit_query: args.get("fit_query").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        extract_metadata: args.get("extract_metadata").and_then(|v| v.as_bool()).unwrap_or(true),
        ..Default::default()
    };
    let crawler = Crawl4AI::new()?;
    let result = crawler.crawl(url, options).await?;
    Ok(json!({
        "status": if result.success { "success" } else { "error" },
        "url": result.url, "title": result.title, "markdown": result.markdown,
        "fit_markdown": result.fit_markdown, "metadata": result.metadata,
        "links_count": result.links_count, "images_count": result.images_count,
        "error": result.error,
    }))
}

pub async fn deep_crawl_tool(args: &Value) -> Result<Value> {
    let url = args.get("url").and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing url"))?;
    let max_depth = args.get("max_depth").and_then(|v| v.as_u64()).unwrap_or(2) as usize;
    let max_pages = args.get("max_pages").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let crawler = Crawl4AI::new()?;
    let results = crawler.deep_crawl(url, max_depth, max_pages, CrawlOptions::default()).await?;
    let summaries: Vec<Value> = results.iter().map(|r| json!({
        "url": r.url, "title": r.title, "markdown_length": r.markdown.len(), "success": r.success,
    })).collect();
    Ok(json!({ "status": "success", "pages_crawled": results.len(), "results": summaries }))
}
