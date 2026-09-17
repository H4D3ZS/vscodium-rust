use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CanvasTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
    Accent,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CanvasChartKind {
    #[default]
    Bar,
    Line,
    Pie,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CanvasStatItem {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub tone: Option<CanvasTone>,
    #[serde(default)]
    pub hint: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CanvasChartSeries {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub values: Vec<f64>,
    #[serde(default)]
    pub tone: Option<CanvasTone>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CanvasProgressItem {
    pub label: String,
    pub value: f64,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub tone: Option<CanvasTone>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CanvasTodoItem {
    pub text: String,
    #[serde(default)]
    pub done: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CanvasKvPair {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CanvasTimelineItem {
    pub title: String,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CanvasBlock {
    Stats {
        #[serde(default)]
        items: Vec<CanvasStatItem>,
    },
    Table {
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        columns: Vec<String>,
        #[serde(default)]
        rows: Vec<Vec<String>>,
    },
    Chart {
        #[serde(default)]
        chart: CanvasChartKind,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        labels: Vec<String>,
        #[serde(default)]
        series: Vec<CanvasChartSeries>,
    },
    Markdown {
        content: String,
    },
    Callout {
        #[serde(default)]
        tone: CanvasTone,
        #[serde(default)]
        title: Option<String>,
        content: String,
    },
    Progress {
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        items: Vec<CanvasProgressItem>,
    },
    Todo {
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        items: Vec<CanvasTodoItem>,
    },
    Kv {
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        pairs: Vec<CanvasKvPair>,
    },
    Code {
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        language: Option<String>,
        content: String,
    },
    Timeline {
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        items: Vec<CanvasTimelineItem>,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CanvasSpec {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default, rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(default)]
    pub blocks: Vec<CanvasBlock>,
}

impl CanvasSpec {
    pub fn summary(&self) -> String {
        let n = self.blocks.len();
        format!("{} block{}", n, if n == 1 { "" } else { "s" })
    }
}

pub fn canvas_slug(title: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for c in title.to_lowercase().chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            out.push(c);
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    while out.starts_with('-') {
        out.remove(0);
    }
    let slug: String = out.chars().take(60).collect();
    if slug.is_empty() {
        "canvas".to_string()
    } else {
        slug
    }
}

fn as_string(v: Option<&Value>, fallback: &str) -> String {
    match v {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Null) | None => fallback.to_string(),
        Some(_) => fallback.to_string(),
    }
}

fn as_number(v: Option<&Value>) -> f64 {
    match v {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(Value::String(s)) => {
            let cleaned: String = s
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '+' || *c == '-')
                .collect();
            cleaned.parse::<f64>().unwrap_or(0.0)
        }
        _ => 0.0,
    }
}

fn as_array(v: Option<&Value>) -> Vec<Value> {
    match v {
        Some(Value::Array(a)) => a.clone(),
        _ => Vec::new(),
    }
}

fn as_tone(v: Option<&Value>) -> Option<CanvasTone> {
    let s = as_string(v, "").to_lowercase();
    match s.as_str() {
        "neutral" => Some(CanvasTone::Neutral),
        "success" => Some(CanvasTone::Success),
        "warning" => Some(CanvasTone::Warning),
        "danger" => Some(CanvasTone::Danger),
        "info" => Some(CanvasTone::Info),
        "accent" => Some(CanvasTone::Accent),
        "green" | "ok" | "good" | "pass" => Some(CanvasTone::Success),
        "yellow" | "orange" | "warn" => Some(CanvasTone::Warning),
        "red" | "error" | "critical" | "fail" | "high" => Some(CanvasTone::Danger),
        "blue" => Some(CanvasTone::Info),
        "purple" => Some(CanvasTone::Accent),
        _ => None,
    }
}

fn canonical_block_type(raw: &str) -> String {
    let t: String = raw
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_ascii_punctuation() && !c.is_whitespace() && !c.is_control())
        .collect();
    match t.as_str() {
        "stats" | "stat" | "metrics" | "metric" | "cards" => "stats".to_string(),
        "table" | "datatable" | "grid" => "table".to_string(),
        "chart" | "barchart" | "linechart" | "piechart" | "graph" => "chart".to_string(),
        "markdown" | "text" | "md" | "paragraph" | "section" => "markdown".to_string(),
        "callout" | "alert" | "note" | "banner" | "warning" => "callout".to_string(),
        "progress" | "progressbar" | "bars" => "progress".to_string(),
        "todo" | "todos" | "tasks" | "checklist" | "list" => "todo".to_string(),
        "kv" | "keyvalue" | "properties" | "details" | "facts" => "kv".to_string(),
        "code" | "codeblock" | "snippet" | "diff" => "code".to_string(),
        "timeline" | "steps" | "history" => "timeline".to_string(),
        _ => t,
    }
}

fn stat_item(it: &Value) -> CanvasStatItem {
    CanvasStatItem {
        label: as_string(
            it.get("label")
                .or_else(|| it.get("name"))
                .or_else(|| it.get("title")),
            "stat",
        ),
        value: match it.get("value") {
            Some(Value::Number(n)) => n.to_string(),
            other => as_string(other, "—"),
        },
        tone: as_tone(it.get("tone").or_else(|| it.get("color"))),
        hint: match it.get("hint") {
            Some(h) => Some(as_string(Some(h), "")),
            None => None,
        },
    }
}

fn normalize_block(raw: &Value) -> Option<CanvasBlock> {
    raw.as_object()?;
    let type_str = as_string(raw.get("type"), "");
    let canonical = canonical_block_type(&type_str);

    match canonical.as_str() {
        "stats" => {
            let items: Vec<CanvasStatItem> = as_array(
                raw.get("items")
                    .or_else(|| raw.get("stats"))
                    .or_else(|| raw.get("cards")),
            )
            .iter()
            .map(stat_item)
            .collect();
            if items.is_empty() {
                None
            } else {
                Some(CanvasBlock::Stats { items })
            }
        }
        "table" => {
            let columns: Vec<String> = as_array(
                raw.get("columns")
                    .or_else(|| raw.get("headers"))
                    .or_else(|| raw.get("header")),
            )
            .iter()
            .map(|c| as_string(Some(c), ""))
            .collect();
            let mut rows: Vec<Vec<String>> = as_array(raw.get("rows").or_else(|| raw.get("data")))
                .iter()
                .map(|r| match r {
                    Value::Array(cells) => cells.iter().map(|c| as_string(Some(c), "")).collect(),
                    Value::Object(map) => {
                        if columns.is_empty() {
                            map.values().map(|v| as_string(Some(v), "")).collect()
                        } else {
                            columns
                                .iter()
                                .map(|c| {
                                    let v = map.get(c).or_else(|| map.get(&c.to_lowercase()));
                                    as_string(v, "")
                                })
                                .collect()
                        }
                    }
                    other => vec![as_string(Some(other), "")],
                })
                .collect();
            let mut columns = columns;
            if columns.is_empty() && !rows.is_empty() {
                let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
                for i in 0..width {
                    columns.push(format!("Col {}", i + 1));
                }
            }
            rows.retain(|r| !r.is_empty());
            let title = match raw.get("title") {
                Some(t) => Some(as_string(Some(t), "")),
                None => None,
            };
            if columns.is_empty() || rows.is_empty() {
                None
            } else {
                Some(CanvasBlock::Table {
                    title,
                    columns,
                    rows,
                })
            }
        }
        "chart" => {
            let kind_mod = as_string(
                raw.get("chart")
                    .or_else(|| raw.get("kind"))
                    .or_else(|| raw.get("variant")),
                "bar",
            )
            .to_lowercase();
            let chart = if kind_mod.contains("pie") || kind_mod.contains("donut") {
                CanvasChartKind::Pie
            } else if kind_mod.contains("line") || kind_mod.contains("area") {
                CanvasChartKind::Line
            } else {
                CanvasChartKind::Bar
            };
            let mut labels: Vec<String> = as_array(
                raw.get("labels")
                    .or_else(|| raw.get("categories"))
                    .or_else(|| raw.get("x")),
            )
            .iter()
            .map(|l| as_string(Some(l), ""))
            .collect();
            let mut series: Vec<CanvasChartSeries> = as_array(raw.get("series"))
                .iter()
                .map(|s| CanvasChartSeries {
                    name: match s.get("name") {
                        Some(n) => Some(as_string(Some(n), "")),
                        None => None,
                    },
                    values: as_array(
                        s.get("values")
                            .or_else(|| s.get("data"))
                            .or_else(|| s.get("y")),
                    )
                    .iter()
                    .map(|v| as_number(Some(v)))
                    .collect(),
                    tone: as_tone(s.get("tone").or_else(|| s.get("color"))),
                })
                .filter(|s| !s.values.is_empty())
                .collect();
            if series.is_empty() {
                if let Some(Value::Array(vals)) = raw.get("values") {
                    series.push(CanvasChartSeries {
                        name: None,
                        values: vals.iter().map(|v| as_number(Some(v))).collect(),
                        tone: None,
                    });
                }
            }
            if series.is_empty() {
                if let Some(Value::Array(pts)) = raw.get("data") {
                    if !pts.is_empty() && pts[0].is_object() {
                        if labels.is_empty() {
                            labels = pts
                                .iter()
                                .map(|p| as_string(p.get("label").or_else(|| p.get("name")), ""))
                                .collect();
                        }
                        series.push(CanvasChartSeries {
                            name: None,
                            values: pts
                                .iter()
                                .map(|p| {
                                    as_number(
                                        p.get("value")
                                            .or_else(|| p.get("count"))
                                            .or_else(|| p.get("y")),
                                    )
                                })
                                .collect(),
                            tone: None,
                        });
                    }
                }
            }
            if series.is_empty() {
                return None;
            }
            if labels.is_empty() {
                labels = (1..=series[0].values.len())
                    .map(|i| i.to_string())
                    .collect();
            }
            let title = match raw.get("title") {
                Some(t) => Some(as_string(Some(t), "")),
                None => None,
            };
            Some(CanvasBlock::Chart {
                chart,
                title,
                labels,
                series,
            })
        }
        "markdown" => {
            let content = as_string(
                raw.get("content")
                    .or_else(|| raw.get("text"))
                    .or_else(|| raw.get("markdown"))
                    .or_else(|| raw.get("body")),
                "",
            );
            if content.is_empty() {
                None
            } else {
                Some(CanvasBlock::Markdown { content })
            }
        }
        "callout" => {
            let content = as_string(
                raw.get("content")
                    .or_else(|| raw.get("text"))
                    .or_else(|| raw.get("message"))
                    .or_else(|| raw.get("body")),
                "",
            );
            if content.is_empty() {
                None
            } else {
                let tone = as_tone(
                    raw.get("tone")
                        .or_else(|| raw.get("color"))
                        .or_else(|| raw.get("severity"))
                        .or_else(|| raw.get("level")),
                )
                .unwrap_or(CanvasTone::Info);
                let title = match raw.get("title") {
                    Some(t) => Some(as_string(Some(t), "")),
                    None => None,
                };
                Some(CanvasBlock::Callout {
                    tone,
                    title,
                    content,
                })
            }
        }
        "progress" => {
            let items: Vec<CanvasProgressItem> =
                as_array(raw.get("items").or_else(|| raw.get("bars")))
                    .iter()
                    .map(|it| CanvasProgressItem {
                        label: as_string(it.get("label").or_else(|| it.get("name")), ""),
                        value: as_number(
                            it.get("value")
                                .or_else(|| it.get("percent"))
                                .or_else(|| it.get("progress")),
                        ),
                        max: it.get("max").map(|m| as_number(Some(m))),
                        tone: as_tone(it.get("tone").or_else(|| it.get("color"))),
                    })
                    .filter(|it| !it.label.is_empty())
                    .collect();
            if items.is_empty() {
                None
            } else {
                let title = match raw.get("title") {
                    Some(t) => Some(as_string(Some(t), "")),
                    None => None,
                };
                Some(CanvasBlock::Progress { title, items })
            }
        }
        "todo" => {
            let items: Vec<CanvasTodoItem> = as_array(
                raw.get("items")
                    .or_else(|| raw.get("todos"))
                    .or_else(|| raw.get("tasks")),
            )
            .iter()
            .map(|it| {
                if let Value::String(s) = it {
                    CanvasTodoItem {
                        text: s.clone(),
                        done: false,
                    }
                } else {
                    CanvasTodoItem {
                        text: as_string(
                            it.get("text")
                                .or_else(|| it.get("title"))
                                .or_else(|| it.get("label"))
                                .or_else(|| it.get("task")),
                            "",
                        ),
                        done: matches!(
                            it.get("done")
                                .or_else(|| it.get("completed"))
                                .or_else(|| it.get("checked")),
                            Some(Value::Bool(true))
                        ),
                    }
                }
            })
            .filter(|it| !it.text.is_empty())
            .collect();
            if items.is_empty() {
                None
            } else {
                let title = match raw.get("title") {
                    Some(t) => Some(as_string(Some(t), "")),
                    None => None,
                };
                Some(CanvasBlock::Todo { title, items })
            }
        }
        "kv" => {
            let mut pairs: Vec<CanvasKvPair> = Vec::new();
            if let Some(Value::Array(arr)) = raw
                .get("pairs")
                .or_else(|| raw.get("items"))
                .or_else(|| raw.get("entries"))
            {
                for p in arr {
                    let key = as_string(
                        p.get("key")
                            .or_else(|| p.get("label"))
                            .or_else(|| p.get("name")),
                        "",
                    );
                    if !key.is_empty() {
                        pairs.push(CanvasKvPair {
                            key,
                            value: as_string(p.get("value").or_else(|| p.get("val")), ""),
                        });
                    }
                }
            } else if let Some(Value::Object(map)) = raw
                .get("pairs")
                .or_else(|| raw.get("items"))
                .or_else(|| raw.get("entries"))
            {
                for (k, v) in map {
                    pairs.push(CanvasKvPair {
                        key: k.clone(),
                        value: as_string(Some(v), ""),
                    });
                }
            }
            if pairs.is_empty() {
                None
            } else {
                let title = match raw.get("title") {
                    Some(t) => Some(as_string(Some(t), "")),
                    None => None,
                };
                Some(CanvasBlock::Kv { title, pairs })
            }
        }
        "code" => {
            let content = as_string(
                raw.get("content")
                    .or_else(|| raw.get("code"))
                    .or_else(|| raw.get("text")),
                "",
            );
            if content.is_empty() {
                None
            } else {
                let title = match raw.get("title") {
                    Some(t) => Some(as_string(Some(t), "")),
                    None => None,
                };
                let language = match raw.get("language") {
                    Some(l) => Some(as_string(Some(l), "")),
                    None => None,
                };
                Some(CanvasBlock::Code {
                    title,
                    language,
                    content,
                })
            }
        }
        "timeline" => {
            let items: Vec<CanvasTimelineItem> = as_array(
                raw.get("items")
                    .or_else(|| raw.get("steps"))
                    .or_else(|| raw.get("events")),
            )
            .iter()
            .map(|it| {
                if let Value::String(s) = it {
                    CanvasTimelineItem {
                        title: s.clone(),
                        detail: None,
                        status: None,
                    }
                } else {
                    let status_raw = as_string(it.get("status"), "").to_lowercase();
                    let status = if status_raw.contains("done") || status_raw.contains("complete") {
                        Some("done".to_string())
                    } else if status_raw.contains("active")
                        || status_raw.contains("progress")
                        || status_raw.contains("running")
                    {
                        Some("active".to_string())
                    } else if !status_raw.is_empty() {
                        Some("pending".to_string())
                    } else {
                        None
                    };
                    let detail = match it.get("detail").or_else(|| it.get("description")) {
                        Some(d) => Some(as_string(Some(d), "")),
                        None => None,
                    };
                    CanvasTimelineItem {
                        title: as_string(
                            it.get("title")
                                .or_else(|| it.get("name"))
                                .or_else(|| it.get("text")),
                            "",
                        ),
                        detail,
                        status,
                    }
                }
            })
            .filter(|it| !it.title.is_empty())
            .collect();
            if items.is_empty() {
                None
            } else {
                let title = match raw.get("title") {
                    Some(t) => Some(as_string(Some(t), "")),
                    None => None,
                };
                Some(CanvasBlock::Timeline { title, items })
            }
        }
        _ => {
            let content = as_string(raw.get("content").or_else(|| raw.get("text")), "");
            if content.is_empty() {
                None
            } else {
                Some(CanvasBlock::Markdown { content })
            }
        }
    }
}

pub fn normalize_canvas_spec(raw: &Value) -> Option<CanvasSpec> {
    let mut obj = raw.clone();
    if obj.is_string() {
        if let Ok(parsed) = serde_json::from_str::<Value>(&as_string(Some(&obj), "")) {
            obj = parsed;
        } else {
            let src = as_string(Some(&obj), "");
            if let Some(start) = src.find("```") {
                let rest = &src[start + 3..];
                if let Some(end) = rest.find("```") {
                    let inner = &rest[..end];
                    let inner = inner.strip_prefix("json").unwrap_or(inner).trim_start();
                    if let Ok(parsed) = serde_json::from_str::<Value>(inner) {
                        obj = parsed;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
    if !obj.is_object() {
        return None;
    }
    if obj.get("blocks").is_none() {
        if let Some(Value::Object(canvas)) = obj.get("canvas") {
            if canvas.get("blocks").is_some() {
                obj = Value::Object(canvas.clone());
            }
        }
    }
    if obj.get("blocks").is_none() {
        if let Some(Value::Object(spec)) = obj.get("spec") {
            if spec.get("blocks").is_some() {
                obj = Value::Object(spec.clone());
            }
        }
    }

    let title = as_string(
        obj.get("title").or_else(|| obj.get("name")),
        "Untitled Canvas",
    );
    let mut blocks: Vec<CanvasBlock> = as_array(
        obj.get("blocks")
            .or_else(|| obj.get("sections"))
            .or_else(|| obj.get("components")),
    )
    .iter()
    .filter_map(normalize_block)
    .collect();

    if blocks.is_empty() {
        let content = as_string(
            obj.get("content")
                .or_else(|| obj.get("markdown"))
                .or_else(|| obj.get("text")),
            "",
        );
        if !content.is_empty() {
            blocks.push(CanvasBlock::Markdown { content });
        }
    }
    if blocks.is_empty() {
        return None;
    }

    let id = as_string(obj.get("id"), &canvas_slug(&title));
    let subtitle = obj.get("subtitle").map(|s| as_string(Some(s), ""));
    Some(CanvasSpec {
        id,
        title,
        subtitle,
        updated_at: chrono::Utc::now().timestamp_millis(),
        blocks,
    })
}

pub fn spec_to_json(spec: &CanvasSpec) -> String {
    serde_json::to_string_pretty(spec).unwrap_or_default()
}
