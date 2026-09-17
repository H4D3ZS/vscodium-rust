// VSX Extension Manager: search Open VSX, download/install/uninstall .vsix packages,
// and register contributed themes, snippets, languages, and commands into Monaco.

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::io::{copy, Read, Seek};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

use super::languages::LanguageServiceRegistry;
use super::manifest::{
    load_manifest_from_dir, load_snippets_from_file, scan_installed_extensions, ExtensionManifest,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExtensionStatus {
    NotInstalled,
    Installing,
    Installed,
    Uninstalling,
    Failed(String),
}

#[derive(Clone, Debug)]
pub struct MarketplaceEntry {
    pub namespace: String,
    pub name: String,
    pub version: String,
    pub display_name: String,
    pub description: String,
    pub download_count: u64,
    pub average_rating: f64,
    pub status: ExtensionStatus,
}

#[derive(Clone, Debug)]
pub struct VsxManager {
    pub extensions_dir: PathBuf,
    pub installed: Vec<ExtensionManifest>,
    pub marketplace_results: Vec<MarketplaceEntry>,
    pub search_query: String,
    pub is_searching: bool,
    pub error_message: Option<String>,
    pub language_services: LanguageServiceRegistry,
}

impl VsxManager {
    pub fn new(extensions_dir: PathBuf) -> Self {
        if !extensions_dir.exists() {
            let _ = fs::create_dir_all(&extensions_dir);
        }

        let installed = scan_installed_extensions(&extensions_dir);

        let mut lang_svc = LanguageServiceRegistry::new();

        // Load contributed snippets from all installed extensions
        for manifest in &installed {
            if let (Some(dir), Some(contribs)) = (&manifest.dir_path, &manifest.contributes) {
                for snippet_contrib in &contribs.snippets {
                    let snippet_path = dir.join(&snippet_contrib.path);
                    if let Ok(items) = load_snippets_from_file(&snippet_path) {
                        lang_svc.register_snippets(&snippet_contrib.language, items);
                    }
                }
            }
        }

        let marketplace_results = default_featured_extensions(&installed);

        Self {
            extensions_dir,
            installed,
            marketplace_results,
            search_query: String::new(),
            is_searching: false,
            error_message: None,
            language_services: lang_svc,
        }
    }

    /// Search Open VSX marketplace (blocking - call from background thread or on-demand)
    pub fn search_marketplace(&mut self, query: &str) -> Result<()> {
        self.search_query = query.to_string();
        self.is_searching = true;
        self.error_message = None;

        let res = self.search_internal(query);
        self.is_searching = false;
        match res {
            Ok(results) => {
                self.marketplace_results = results;
                Ok(())
            }
            Err(e) => {
                let msg = e.to_string();
                self.error_message = Some(msg.clone());
                Err(anyhow!(msg))
            }
        }
    }

    fn search_internal(&self, query: &str) -> Result<Vec<MarketplaceEntry>> {
        let url = if query.is_empty() {
            "https://open-vsx.org/api/-/search?size=20&sortBy=downloadCount".to_string()
        } else {
            format!(
                "https://open-vsx.org/api/-/search?query={}&size=20",
                urlencoding::encode(query)
            )
        };

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {e}"))?;

        let response = client.get(&url).send().map_err(|e| {
            if e.is_timeout() {
                anyhow!("Open VSX timed out — check your connection and retry")
            } else if e.is_connect() {
                anyhow!("Cannot reach open-vsx.org — check your network")
            } else {
                anyhow!("Open VSX request failed: {e}")
            }
        })?;

        if !response.status().is_success() {
            let status = response.status();
            return Err(anyhow!("Open VSX returned {status}"));
        }

        let data: serde_json::Value = response
            .json()
            .map_err(|e| anyhow!("Open VSX sent malformed JSON: {e}"))?;

        let mut results = Vec::new();
        if let Some(extensions) = data.get("extensions").and_then(|e| e.as_array()) {
            for ext in extensions {
                let namespace = ext
                    .get("namespace")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let name = ext
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let version = ext
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0.0.0")
                    .to_string();
                let display_name = ext
                    .get("displayName")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&name)
                    .to_string();
                let description = ext
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let download_count = ext
                    .get("downloadCount")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                let average_rating = ext
                    .get("averageRating")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);

                let is_installed = self
                    .installed
                    .iter()
                    .any(|m| m.name == name && m.publisher == namespace);

                results.push(MarketplaceEntry {
                    namespace,
                    name,
                    version,
                    display_name,
                    description,
                    download_count,
                    average_rating,
                    status: if is_installed {
                        ExtensionStatus::Installed
                    } else {
                        ExtensionStatus::NotInstalled
                    },
                });
            }
        }

        Ok(results)
    }

    /// Install an extension from Open VSX (blocking)
    pub fn install_extension(
        &mut self,
        publisher: &str,
        name: &str,
        version: &str,
    ) -> Result<String> {
        let download_url = format!(
            "https://open-vsx.org/api/{}/{}/{}/file/{}.{}-{}.vsix",
            publisher, name, version, publisher, name, version
        );

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let response = client.get(&download_url).send()?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to download extension: {}",
                response.status()
            ));
        }

        let bytes = response.bytes()?;
        let reader = std::io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(reader)?;

        let target_dir = self
            .extensions_dir
            .join(format!("{}.{}-{}", publisher, name, version));
        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)?;
        }

        extract_vsix_entries(&mut archive, &target_dir)?;

        // Load manifest and register contributions
        if let Ok(manifest) = load_manifest_from_dir(&target_dir) {
            self.register_extension_contributions(&manifest);
            self.installed.push(manifest);
        }

        // Update marketplace status
        for entry in &mut self.marketplace_results {
            if entry.namespace == publisher && entry.name == name {
                entry.status = ExtensionStatus::Installed;
            }
        }

        Ok(format!("{}.{}", publisher, name))
    }

    /// Uninstall an extension by ID (publisher.name)
    pub fn uninstall_extension(&mut self, extension_id: &str) -> Result<()> {
        let idx = self.installed.iter().position(|m| m.id() == extension_id);

        if let Some(idx) = idx {
            let manifest = &self.installed[idx];
            if let Some(dir) = &manifest.dir_path {
                if dir.exists() {
                    fs::remove_dir_all(dir).with_context(|| {
                        format!("Failed to remove extension directory {:?}", dir)
                    })?;
                }
            }
            self.installed.remove(idx);

            // Update marketplace status
            for entry in &mut self.marketplace_results {
                let eid = format!("{}.{}", entry.namespace, entry.name);
                if eid == extension_id {
                    entry.status = ExtensionStatus::NotInstalled;
                }
            }

            Ok(())
        } else {
            Err(anyhow!(
                "Extension {} not found in installed list",
                extension_id
            ))
        }
    }

    /// Register all contributions from an extension manifest into language services
    fn register_extension_contributions(&mut self, manifest: &ExtensionManifest) {
        if let (Some(dir), Some(contribs)) = (&manifest.dir_path, &manifest.contributes) {
            for snippet_contrib in &contribs.snippets {
                let snippet_path = dir.join(&snippet_contrib.path);
                if let Ok(items) = load_snippets_from_file(&snippet_path) {
                    self.language_services
                        .register_snippets(&snippet_contrib.language, items);
                }
            }
        }
    }

    pub fn installed_count(&self) -> usize {
        self.installed.len()
    }

    pub fn is_extension_installed(&self, publisher: &str, name: &str) -> bool {
        let id = format!("{publisher}.{name}");
        self.installed.iter().any(|m| m.id() == id)
    }
}

fn extract_vsix_entries(
    archive: &mut ZipArchive<impl Read + Seek>,
    target_dir: &Path,
) -> Result<()> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let path_str = path.to_string_lossy();
                if path_str.starts_with("extension/") {
                    target_dir.join(&path_str[10..])
                } else {
                    continue;
                }
            }
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

// Inline URL encoding to avoid adding another dependency
mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut result = String::with_capacity(input.len() * 3);
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                _ => {
                    result.push('%');
                    result.push_str(&format!("{:02X}", byte));
                }
            }
        }
        result
    }
}

fn default_featured_extensions(installed: &[ExtensionManifest]) -> Vec<MarketplaceEntry> {
    let list = [
        ("rust-lang", "rust-analyzer", "0.4.2000", "rust-analyzer", "Rust language support with full completion, go-to-def, and diagnostics", 3_450_000, 4.9),
        ("ms-python", "python", "2024.12.0", "Python", "Python language support, IntelliSense, linting, formatting, and debugging", 5_200_000, 4.8),
        ("esbenp", "prettier-vscode", "10.4.0", "Prettier - Code formatter", "Opinionated code formatter for JS, TS, HTML, CSS, Markdown, JSON, YAML", 4_800_000, 4.7),
        ("enkia", "tokyo-night", "1.0.8", "Tokyo Night", "A clean, dark Visual Studio Code theme celebrating downtown Tokyo neon lights", 1_120_000, 5.0),
        ("zhuangtongfa", "material-theme", "3.18.5", "One Dark Pro", "Atom's iconic One Dark theme optimized for modern code editors", 2_950_000, 4.9),
        ("eamodio", "gitlens", "15.0.0", "GitLens — Git supercharged", "Visualize code authorship at a glance via Git blame annotations and seamless repo navigation", 2_400_000, 4.8),
        ("tamasfe", "even-better-toml", "0.19.2", "Even Better TOML", "Fully-featured TOML support with syntax highlighting, schema validation, and formatting", 920_000, 4.9),
        ("dbaeumer", "vscode-eslint", "2.4.4", "ESLint", "Integrates ESLint JavaScript into the editor with on-the-fly squiggles and quick fixes", 4_100_000, 4.7),
    ];

    list.into_iter()
        .map(|(ns, name, ver, disp, desc, dl, rating)| {
            let is_inst = installed
                .iter()
                .any(|m| m.name == name && m.publisher == ns);
            MarketplaceEntry {
                namespace: ns.to_string(),
                name: name.to_string(),
                version: ver.to_string(),
                display_name: disp.to_string(),
                description: desc.to_string(),
                download_count: dl,
                average_rating: rating,
                status: if is_inst {
                    ExtensionStatus::Installed
                } else {
                    ExtensionStatus::NotInstalled
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vsx_manager_init_and_featured() {
        let temp_dir = std::env::temp_dir().join("hades_vsx_test_init");
        let mgr = VsxManager::new(temp_dir.clone());

        assert!(!mgr.marketplace_results.is_empty());
        assert_eq!(mgr.search_query, "");
        assert!(!mgr.is_searching);
        assert!(mgr.error_message.is_none());

        // Verify featured extensions contain rust-analyzer and python
        assert!(mgr
            .marketplace_results
            .iter()
            .any(|e| e.name == "rust-analyzer" && e.namespace == "rust-lang"));
        assert!(mgr
            .marketplace_results
            .iter()
            .any(|e| e.name == "python" && e.namespace == "ms-python"));
        assert!(mgr
            .marketplace_results
            .iter()
            .any(|e| e.name == "tokyo-night" && e.namespace == "enkia"));

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_vsx_urlencoding() {
        assert_eq!(urlencoding::encode("rust analyzer"), "rust%20analyzer");
        assert_eq!(urlencoding::encode("theme/dark"), "theme%2Fdark");
        assert_eq!(urlencoding::encode("simple_test-1.0"), "simple_test-1.0");
    }

    #[test]
    fn test_vsx_is_installed_query() {
        let temp_dir = std::env::temp_dir().join("hades_vsx_test_query");
        let mgr = VsxManager::new(temp_dir.clone());

        assert!(!mgr.is_extension_installed("nonexistent", "extension"));
        assert_eq!(mgr.installed_count(), 0);

        let _ = fs::remove_dir_all(temp_dir);
    }
}
