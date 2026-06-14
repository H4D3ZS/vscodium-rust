//! XML file download functionality.
//!
//! Downloads ARM architecture specification XML files from ARM's developer website.

use super::SpecType;
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Archive;

/// Re-export DEFAULT_VERSION from parent module.
pub use super::DEFAULT_VERSION;

/// Base URL for ARM developer downloads.
const BASE_URL: &str =
    "https://developer.arm.com/-/media/developer/products/architecture/armv8-a-architecture";

/// User-Agent to use for downloads (mimics curl to avoid Cloudflare blocks).
const USER_AGENT: &str = "curl/8.7.1";

/// Download a file from the given URL.
pub fn download_file(url: &str, output_path: &Path) -> Result<(), String> {
    println!("Downloading {} ...", url);

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(url)
        .send()
        .map_err(|e| format!("Failed to download {}: {}", url, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Download failed with status {}: {}",
            response.status(),
            url
        ));
    }

    let bytes = response
        .bytes()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let mut file = File::create(output_path)
        .map_err(|e| format!("Failed to create file {}: {}", output_path.display(), e))?;

    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    println!("Downloaded to {}", output_path.display());
    Ok(())
}

/// Download ARM specification archive.
pub fn download_spec(spec: SpecType, version: &str, output_dir: &Path) -> Result<PathBuf, String> {
    let filename = spec.archive_name(version);
    let url = format!("{}/{}", BASE_URL, filename);
    let output_path = output_dir.join(&filename);

    // Skip if already downloaded
    if output_path.exists() {
        println!("Archive already exists: {}", output_path.display());
        return Ok(output_path);
    }

    download_file(&url, &output_path)?;
    Ok(output_path)
}

/// Extract a tar.gz archive.
pub fn extract_archive(archive_path: &Path, output_dir: &Path) -> Result<PathBuf, String> {
    println!("Extracting {} ...", archive_path.display());

    let file = File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);

    // First pass: find nested tar.gz files and the actual root directory
    let mut nested_tars: Vec<String> = Vec::new();
    let mut actual_root: Option<String> = None;

    for entry in archive
        .entries()
        .map_err(|e| format!("Failed to read archive: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path().map_err(|e| format!("Invalid path: {}", e))?;
        let path_str = path.to_string_lossy();

        // Look for nested tar.gz files (but not _OPT or _diff variants)
        if path_str.ends_with(".tar.gz")
            && !path_str.contains("_OPT")
            && !path_str.contains("_diff")
        {
            nested_tars.push(path_str.to_string());
        }
    }

    // Second pass: extract
    let file = File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);

    archive
        .unpack(output_dir)
        .map_err(|e| format!("Failed to extract archive: {}", e))?;

    // Extract nested tar.gz files if found
    for nested_path in &nested_tars {
        // Remove leading ./ if present
        let clean_path = nested_path.trim_start_matches("./");
        let nested_tar = output_dir.join(clean_path);
        if nested_tar.exists() {
            println!("Extracting nested archive {} ...", nested_tar.display());
            let file = File::open(&nested_tar)
                .map_err(|e| format!("Failed to open nested archive: {}", e))?;
            let gz = GzDecoder::new(file);
            let mut archive = Archive::new(gz);

            // Find root directory in nested archive
            let file2 = File::open(&nested_tar)
                .map_err(|e| format!("Failed to open nested archive: {}", e))?;
            let gz2 = GzDecoder::new(file2);
            let mut archive2 = Archive::new(gz2);
            for entry in archive2
                .entries()
                .map_err(|e| format!("Failed to read nested archive: {}", e))?
            {
                let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
                let path = entry.path().map_err(|e| format!("Invalid path: {}", e))?;
                if let Some(first) = path.components().next() {
                    let first_str = first.as_os_str().to_string_lossy().to_string();
                    if first_str != "." && first_str != ".." {
                        actual_root = Some(first_str);
                        break;
                    }
                }
            }

            // Reopen and extract
            let file = File::open(&nested_tar)
                .map_err(|e| format!("Failed to open nested archive: {}", e))?;
            let gz = GzDecoder::new(file);
            let mut archive = Archive::new(gz);
            archive
                .unpack(output_dir)
                .map_err(|e| format!("Failed to extract nested archive: {}", e))?;
        }
    }

    // If we found an actual root from nested archives, use that
    // Otherwise, look for extracted directories matching expected patterns
    let extracted_dir = if let Some(root) = actual_root {
        output_dir.join(root)
    } else {
        // No nested archives - find the extracted directory
        // Look for directories matching ISA_* or SysReg_* patterns
        let mut found_dir = output_dir.to_path_buf();
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if (name.starts_with("ISA_") || name.starts_with("SysReg_"))
                    && entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
                {
                    found_dir = entry.path();
                    break;
                }
            }
        }
        found_dir
    };

    println!("Extracted to {}", extracted_dir.display());
    Ok(extracted_dir)
}

/// Download and extract all specification files.
pub fn download_and_extract_all(
    version: &str,
    output_dir: &Path,
) -> Result<DownloadedSpecs, String> {
    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let mut specs = DownloadedSpecs::default();

    // Download and extract A64
    let a64_archive = download_spec(SpecType::A64, version, output_dir)?;
    specs.a64 = Some(extract_archive(&a64_archive, output_dir)?);

    // Download and extract AArch32
    let a32_archive = download_spec(SpecType::AArch32, version, output_dir)?;
    specs.aarch32 = Some(extract_archive(&a32_archive, output_dir)?);

    // Download and extract SysReg
    let sysreg_archive = download_spec(SpecType::SysReg, version, output_dir)?;
    specs.sysreg = Some(extract_archive(&sysreg_archive, output_dir)?);

    Ok(specs)
}

/// Paths to downloaded and extracted specification directories.
#[derive(Debug, Default)]
pub struct DownloadedSpecs {
    /// A64 instruction set XML directory
    pub a64: Option<PathBuf>,
    /// AArch32 instruction set XML directory
    pub aarch32: Option<PathBuf>,
    /// System registers XML directory
    pub sysreg: Option<PathBuf>,
}

impl DownloadedSpecs {
    /// Get all available directories as a slice.
    pub fn all_dirs(&self) -> Vec<&Path> {
        let mut dirs = Vec::new();
        if let Some(ref p) = self.a64 {
            dirs.push(p.as_path());
        }
        if let Some(ref p) = self.aarch32 {
            dirs.push(p.as_path());
        }
        if let Some(ref p) = self.sysreg {
            dirs.push(p.as_path());
        }
        dirs
    }

    /// Get instruction set directories only.
    pub fn instruction_dirs(&self) -> Vec<&Path> {
        let mut dirs = Vec::new();
        if let Some(ref p) = self.a64 {
            dirs.push(p.as_path());
        }
        if let Some(ref p) = self.aarch32 {
            dirs.push(p.as_path());
        }
        dirs
    }
}

/// Read the support ASL files from mra_tools.
pub fn read_support_files(support_dir: &Path) -> Result<String, String> {
    let mut content = String::new();

    let files = [
        "aes.asl",
        "barriers.asl",
        "debug.asl",
        "feature.asl",
        "fetchdecode.asl",
        "hints.asl",
        "interrupts.asl",
        "memory.asl",
        "stubs.asl",
        "usermode.asl",
    ];

    for filename in &files {
        let path = support_dir.join(filename);
        if path.exists() {
            let file_content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
            content.push_str(&file_content);
            content.push('\n');
        }
    }

    Ok(content)
}
