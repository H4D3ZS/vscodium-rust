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
    // MongoDB documents are usually stored as JSON or BSON.
    // We'll treat it as high-level collection/document visualization.
    match serde_json::from_str::<Value>(content) {
        Ok(val) => parse_json_to_graph(val),
        Err(_) => {
            // If it's multiple documents (line delimited), wrap them
            let wrapped = format!("[{}]", content.replace("}\n{", "},{"));
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
