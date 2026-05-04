use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::io::copy;
use zip::ZipArchive;
use anyhow::{Result, anyhow};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketplaceExtension {
    pub namespace: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "downloadCount")]
    pub download_count: Option<u64>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f64>,
}

pub async fn search_extensions(query: String) -> Result<Vec<MarketplaceExtension>> {
    let client = Client::new();
    let url = format!("https://open-vsx.org/api/-/search?query={}&size=20", urlencoding::encode(&query));
    fetch_extensions_from_url(client, url).await
}

pub async fn get_popular_extensions() -> Result<Vec<MarketplaceExtension>> {
    let client = Client::new();
    let url = "https://open-vsx.org/api/-/search?size=20&sortBy=downloadCount".to_string();
    fetch_extensions_from_url(client, url).await
}

async fn fetch_extensions_from_url(client: Client, url: String) -> Result<Vec<MarketplaceExtension>> {
    let response = client.get(url).send().await?;
    let data: serde_json::Value = response.json().await?;
    
    let mut results = Vec::new();
    if let Some(extensions) = data.get("extensions").and_then(|e| e.as_array()) {
        for ext in extensions {
            if let Ok(mut m) = serde_json::from_value::<MarketplaceExtension>(ext.clone()) {
                // Heuristic: if icon_url is missing, try to find it in files/icon or icons/small
                if m.icon_url.is_none() {
                    m.icon_url = ext.get("files").and_then(|f| f.get("icon")).and_then(|v| v.as_str()).map(|s| s.to_string())
                        .or_else(|| ext.get("icons").and_then(|i| i.get("small")).and_then(|v| v.as_str()).map(|s| s.to_string()));
                }
                results.push(m);
            }
        }
    }
    
    Ok(results)
}

pub async fn install_extension(
    publisher: String, 
    name: String, 
    version: String,
    extensions_dir: PathBuf
) -> Result<String> {
    let client = Client::new();
    let download_url = format!(
        "https://open-vsx.org/api/{}/{}/{}/file/{}.{}-{}.vsix",
        publisher, name, version, publisher, name, version
    );
    
    let response = client.get(download_url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!("Failed to download extension: {}", response.status()));
    }
    
    let bytes = response.bytes().await?;
    let reader = std::io::Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader)?;
    
    let target_dir = extensions_dir.join(format!("{}.{}-{}", publisher, name, version));
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                // VS Code extensions in .vsix are usually under an "extension/" folder
                let path_str = path.to_string_lossy();
                if path_str.starts_with("extension/") {
                    target_dir.join(&path_str[10..])
                } else {
                    continue; // Skip other files like [Content_Types].xml
                }
            },
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }
    
    Ok(format!("{}.{}", publisher, name))
}
pub async fn get_extension_details(publisher: String, name: String) -> Result<serde_json::Value> {
    let client = Client::new();
    let url = format!("https://open-vsx.org/api/{}/{}", urlencoding::encode(&publisher), urlencoding::encode(&name));
    let response = client.get(url).send().await?;
    let mut data: serde_json::Value = response.json().await?;
    
    // Try to fetch README content if it exists in files
    if let Some(readme_url) = data.get("files").and_then(|f| f.get("readme")).and_then(|r| r.as_str()) {
        if let Ok(readme_response) = client.get(readme_url).send().await {
            if let Ok(readme_text) = readme_response.text().await {
                if let Some(obj) = data.as_object_mut() {
                    obj.insert("readme".to_string(), serde_json::Value::String(readme_text));
                }
            }
        }
    }
    
    Ok(data)
}
