use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

fn find_json_end(bytes: &[u8]) -> Option<usize> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escaped = false;

    for (i, &b) in bytes.iter().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        match b {
            b'\\' => escaped = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => brace_count += 1,
            b'}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
    }
    None
}

#[tauri::command]
pub async fn load_kortex_memory(path: String) -> Result<Vec<f32>, String> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Ok(vec![0.0; 1536]);
    }

    let bytes = fs::read(&path_buf).map_err(|e| e.to_string())?;
    
    let header_end = find_json_end(&bytes).ok_or_else(|| "Invalid .aim format: No JSON header found".to_string())?;

    if header_end + (1536 * 4) > bytes.len() {
        return Ok(vec![0.0; 1536]);
    }

    let tensor_bytes = &bytes[header_end..header_end + (1536 * 4)];
    let mut tensor = Vec::with_capacity(1536);
    for chunk in tensor_bytes.chunks_exact(4) {
        let val = f32::from_le_bytes(chunk.try_into().unwrap());
        tensor.push(val);
    }

    Ok(tensor)
}

#[tauri::command]
pub async fn load_kortex_metadata(path: String) -> Result<Value, String> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Ok(json!({}));
    }

    let bytes = fs::read(&path_buf).map_err(|e| e.to_string())?;
    
    let header_end = find_json_end(&bytes).ok_or_else(|| "Invalid .aim format: No JSON header found".to_string())?;

    let header_json = String::from_utf8_lossy(&bytes[..header_end]);
    let metadata: Value = serde_json::from_str(&header_json).map_err(|e| e.to_string())?;
    
    Ok(metadata)
}

#[tauri::command]
pub async fn save_kortex_memory(path: String, gist_token: Vec<f32>, metadata: Value) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    if let Some(parent) = path_buf.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    // Ensure we have a valid JSON header
    let header_json = serde_json::to_string(&metadata).map_err(|e| e.to_string())?;
    
    let mut bytes = header_json.into_bytes();
    
    for val in gist_token {
        bytes.extend_from_slice(&val.to_le_bytes());
    }

    fs::write(&path_buf, bytes).map_err(|e| e.to_string())?;
    println!("[KORTEX] Saved consciousness to {}", path);
    
    Ok(())
}
