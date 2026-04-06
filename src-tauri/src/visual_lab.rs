use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VisualNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub data: Value,
    pub position: NodePosition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VisualEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub animated: bool,
    #[serde(rename = "markerEnd", skip_serializing_if = "Option::is_none")]
    pub marker_end: Option<Value>,
    pub style: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VisualGraph {
    pub nodes: Vec<VisualNode>,
    pub edges: Vec<VisualEdge>,
}

pub fn parse_json_to_graph(data: Value) -> VisualGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    process_json_value(
        &data,
        "JSON Root".to_string(), // Match screenshot root label
        None,
        &mut nodes,
        &mut edges,
        0.0,
        0.0,
        420.0, // Horizontal step
        60.0,  // Vertical sibling spread
    );

    VisualGraph { nodes, edges }
}

fn process_json_value(
    value: &Value,
    label: String,
    parent_id: Option<String>,
    nodes: &mut Vec<VisualNode>,
    edges: &mut Vec<VisualEdge>,
    x: f32,
    y: f32,
    spacing_x: f32,
    spacing_y: f32,
) -> String {
    let id = Uuid::new_v4().to_string();

    nodes.push(VisualNode {
        id: id.clone(),
        node_type: "jsonNode".to_string(),
        data: json!({
            "label": label,
            "value": if value.is_object() || value.is_array() { "".to_string() } else { value.to_string().replace("\"", "") },
            "type": if value.is_object() { "object" } else if value.is_array() { "array" } else { "value" }
        }),
        position: NodePosition { x, y },
    });

    if let Some(pid) = parent_id {
        edges.push(VisualEdge {
            id: format!("e-{}-{}", pid, id),
            source: pid,
            target: id.clone(),
            animated: false,
            marker_end: None, // No arrows in screenshot
            style: json!({
                "stroke": "#3b82f6",
                "strokeWidth": 1.5,
                "strokeDasharray": "4 4",
                "opacity": 0.5
            }),
        });
    }

    match value {
        Value::Object(map) => {
            let next_x = x + spacing_x;
            let start_y = y - (map.len() as f32 - 1.0) * spacing_y / 2.0;
            for (index, (key, val)) in map.iter().enumerate() {
                process_json_value(
                    val,
                    key.clone(),
                    Some(id.clone()),
                    nodes,
                    edges,
                    next_x,
                    start_y + (index as f32 * spacing_y),
                    spacing_x * 0.9,
                    spacing_y * 1.1,
                );
            }
        }
        Value::Array(arr) => {
            let next_x = x + spacing_x;
            let start_y = y - (arr.len() as f32 - 1.0) * spacing_y / 2.0;
            for (index, val) in arr.iter().enumerate() {
                process_json_value(
                    val,
                    format!("[{}]", index),
                    Some(id.clone()),
                    nodes,
                    edges,
                    next_x,
                    start_y + (index as f32 * spacing_y),
                    spacing_x * 0.9,
                    spacing_y * 1.1,
                );
            }
        }
        _ => {}
    }

    id
}

pub fn parse_sql_to_graph(sql: &str) -> VisualGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    let mut x_offset = 0.0;
    let mut table_map = HashMap::new(); // table_name -> id

    // Match CREATE TABLE ... ( ... )
    let re_table = regex::Regex::new(r"(?i)CREATE\s+TABLE\s+(\w+)\s*\(([\s\S]*?)\);").unwrap();
    let re_column = regex::Regex::new(r"^\s*(\w+)\s+(\w+)(?:\(.*\))?").unwrap();
    let re_fk =
        regex::Regex::new(r"(?i)FOREIGN\s+KEY\s*\((\w+)\)\s*REFERENCES\s*(\w+)\s*\((\w+)\)")
            .unwrap();

    for cap in re_table.captures_iter(sql) {
        let table_name = cap[1].to_string();
        let body = &cap[2];

        let id = Uuid::new_v4().to_string();
        table_map.insert(table_name.clone(), id.clone());
        let mut columns = Vec::new();

        for line in body.lines() {
            let line = line.trim();
            if let Some(col_cap) = re_column.captures(line) {
                columns.push(json!({
                    "name": col_cap[1].to_string(),
                    "type": col_cap[2].to_string(),
                    "isPk": line.to_uppercase().contains("PRIMARY KEY")
                }));
            }
        }

        nodes.push(VisualNode {
            id: id.clone(),
            node_type: "erdNode".to_string(),
            data: json!({
                "label": table_name,
                "columns": columns
            }),
            position: NodePosition {
                x: x_offset,
                y: 0.0,
            },
        });

        x_offset += 450.0;
    }

    // Second pass for edges (FKs)
    for cap in re_table.captures_iter(sql) {
        let table_name = cap[1].to_string();
        let body = &cap[2];
        let source_id = table_map.get(&table_name).unwrap();

        for line in body.lines() {
            if let Some(fk_cap) = re_fk.captures(line.trim()) {
                let target_table = &fk_cap[2];
                if let Some(target_id) = table_map.get(target_table) {
                    edges.push(VisualEdge {
                        id: format!("e-fk-{}-{}", source_id, target_id),
                        source: source_id.clone(),
                        target: target_id.clone(),
                        animated: true,
                        marker_end: Some(json!({ "type": "arrowclosed", "color": "#10b981" })),
                        style: json!({ "stroke": "#10b981", "strokeWidth": 2, "strokeDasharray": "5 5" }),
                    });
                }
            }
        }
    }

    VisualGraph { nodes, edges }
}

pub fn parse_mongodb_to_graph(content: &str) -> VisualGraph {
    let mut clean_content = content.trim().to_string();

    // Try to extract data from db.collection.insertMany([...]) or insertOne({...})
    let re_insert_many =
        regex::Regex::new(r"(?i)db\.\w+\.insertMany\s*\(\s*(\[[\s\S]*?\])\s*\)").unwrap();
    let re_insert_one =
        regex::Regex::new(r"(?i)db\.\w+\.insertOne\s*\(\s*(\{[\s\S]*?\})\s*\)").unwrap();

    if let Some(cap) = re_insert_many.captures(&clean_content) {
        clean_content = cap[1].to_string();
    } else if let Some(cap) = re_insert_one.captures(&clean_content) {
        clean_content = cap[1].to_string();
    }

    // Strip MongoDB specific types like ObjectId("..."), ISODate("...")
    let re_object_id = regex::Regex::new(r#"ObjectId\s*\(\s*"([^"]*)"\s*\)"#).unwrap();
    let re_iso_date = regex::Regex::new(r#"ISODate\s*\(\s*"([^"]*)"\s*\)"#).unwrap();

    clean_content = re_object_id
        .replace_all(&clean_content, "\"$1\"")
        .to_string();
    clean_content = re_iso_date
        .replace_all(&clean_content, "\"$1\"")
        .to_string();

    match serde_json::from_str::<Value>(&clean_content) {
        Ok(val) => parse_json_to_graph(val),
        Err(_) => {
            // Last resort: handle line-delimited JSON or partially valid content
            let wrapped = format!("[{}]", clean_content.replace("}\n{", "},{"));
            if let Ok(val) = serde_json::from_str::<Value>(&wrapped) {
                parse_json_to_graph(val)
            } else {
                VisualGraph {
                    nodes: vec![],
                    edges: vec![],
                }
            }
        }
    }
}

pub fn generate_neural_omni_graph(slots: Vec<crate::memory_store::SemanticSlot>) -> VisualGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut tag_to_nodes: HashMap<String, Vec<String>> = HashMap::new();

    for (idx, slot) in slots.iter().enumerate() {
        let node_id = slot.id.clone();
        
        let angle = idx as f32 * 0.5;
        let radius = 100.0 + (idx as f32 * 20.0);
        let x = radius * angle.cos();
        let y = radius * angle.sin();

        let weight = if slot.category == "fix_lessons" { 1.0 } else { 0.5 };

        nodes.push(VisualNode {
            id: node_id.clone(),
            node_type: "neuralNode".to_string(),
            data: json!({
                "label": slot.content.split('/').last().unwrap_or(&slot.content),
                "category": slot.category,
                "weight": weight,
                "tags": slot.tags,
                "lastUpdated": slot.timestamp
            }),
            position: NodePosition { x, y },
        });

        for tag in &slot.tags {
            tag_to_nodes.entry(tag.clone()).or_default().push(node_id.clone());
        }
    }

    for (tag, node_ids) in tag_to_nodes {
        if node_ids.len() > 1 {
            for i in 0..node_ids.len() {
                for j in i + 1..node_ids.len() {
                    edges.push(VisualEdge {
                        id: format!("neural-{}-{}-{}", tag, node_ids[i], node_ids[j]),
                        source: node_ids[i].clone(),
                        target: node_ids[j].clone(),
                        animated: true,
                        marker_end: None,
                        style: json!({
                            "stroke": "#f472b6",
                            "strokeWidth": 1,
                            "opacity": 0.3,
                            "label": tag
                        }),
                    });
                }
            }
        }
    }

    VisualGraph { nodes, edges }
}
