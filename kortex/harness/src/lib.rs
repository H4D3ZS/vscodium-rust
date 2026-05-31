use anyhow::Result;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

pub mod vedic;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub level: String,
    pub span: Option<Span>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Span {
    pub file_name: String,
    pub line_start: usize,
    pub column_start: usize,
}

pub struct ShadowVFS {
    _temp_dir: TempDir,
    pub mount_path: PathBuf,
}

impl ShadowVFS {
    pub fn new(root: &Path) -> Result<Self> {
        let temp = tempfile::tempdir()?;
        let mount_path = temp.path().to_path_buf();
        
        // Mirror basic structure
        self::mirror_recursive(root, &mount_path, 0)?;
        
        println!("[Harness] Shadow VFS created at {:?}", mount_path);
        
        Ok(Self {
            _temp_dir: temp,
            mount_path,
        })
    }

    pub fn apply_patch(&self, relative_path: &Path, content: &str) -> Result<()> {
        let full_path = self.mount_path.join(relative_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full_path, content)?;
        Ok(())
    }
}

fn mirror_recursive(source: &Path, target: &Path, depth: usize) -> Result<()> {
    if depth > 5 { return Ok(()); } // Safety limit
    
    if !target.exists() {
        fs::create_dir_all(target)?;
    }
    
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        
        // Skip noise
        if name == "target" || name == ".git" || name == ".hades_cache" || name == ".aim" {
            continue;
        }
        
        let target_path = target.join(name);
        if path.is_dir() {
            // Only mirror src and top-level files for speed, unless deep mirror is needed
            if name == "src" || depth == 0 {
                mirror_recursive(&path, &target_path, depth + 1)?;
            }
        } else {
            fs::copy(&path, &target_path)?;
        }
    }
    Ok(())
}

pub struct SymbolicVerifier;

impl SymbolicVerifier {
    pub fn verify_cargo(path: &Path, original_root: &Path) -> Result<Vec<Diagnostic>> {
        let target_dir = original_root.join("target");
        
        let output = Command::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .env("CARGO_TARGET_DIR", target_dir) // Share target dir for speed
            .current_dir(path)
            .output()?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut diagnostics = Vec::new();
        
        for line in stdout.lines() {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
                if val["reason"] == "compiler-message" {
                    let msg = &val["message"];
                    if let Ok(diag) = serde_json::from_value::<Diagnostic>(msg.clone()) {
                        diagnostics.push(diag);
                    }
                }
            }
        }
        
        Ok(diagnostics)
    }
}

pub struct ReasoningLoop {
    original_root: PathBuf,
}

impl ReasoningLoop {
    pub fn new(root: &Path) -> Self {
        Self {
            original_root: root.to_path_buf(),
        }
    }

    /// Verifies a code change in a temporary shadow VFS.
    /// Returns the diagnostics if there are errors.
    pub fn verify_candidate(&self, relative_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let shadow = ShadowVFS::new(&self.original_root)?;
        shadow.apply_patch(relative_path, content)?;
        
        SymbolicVerifier::verify_cargo(&shadow.mount_path, &self.original_root)
    }
}

/// The Stoic Filter for distinguishing between Verifiable Truth (Katalepsis) and Opinion (Doxa).
pub struct KatalepsisFilter;

impl KatalepsisFilter {
    /// Evaluates the "Verity" of a code transformation.
    /// Returns a score from 0.0 to 1.0. 1.0 means Absolute Truth (no errors/warnings).
    pub fn evaluate_verity(diagnostics: &[Diagnostic]) -> f32 {
        if diagnostics.is_empty() {
            return 1.0;
        }
        
        let has_errors = diagnostics.iter().any(|d| d.level == "error");
        if has_errors {
            return 0.0; // Rejected as Doxa
        }
        
        // Return fractional for warnings
        let warning_count = diagnostics.len() as f32;
        (1.0 - (warning_count * 0.1)).max(0.1)
    }
}

/// A geometric embedder that maps tree structures into Pythagorean space (unit hypercube).
/// This eliminates file-structure hallucinations by providing absolute geometric coordinates.
pub struct PythagoreanEmbedder {
    dimension: usize,
}

impl PythagoreanEmbedder {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    /// Generates a coordinate vector for a node in the tree.
    /// Formula: P(node) = P(parent) + (depth_weight * child_vector)
    pub fn embed_path(&self, relative_path: &Path) -> Vec<f32> {
        let mut embedding = vec![0.0f32; self.dimension];
        let components: Vec<_> = relative_path.components().collect();
        
        for (depth, component) in components.iter().enumerate() {
            let name = component.as_os_str().to_string_lossy();
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            name.hash(&mut hasher);
            let seed = hasher.finish();
            
            // Generate a pseudo-random unit vector for this branch
            let branch_vector = self.generate_branch_vector(seed);
            let weight = 1.0 / (depth as f32 + 1.0).powi(2); // Quadratic distance decay
            
            for (i, val) in branch_vector.iter().enumerate() {
                embedding[i] += weight * val;
            }
        }
        
        // Normalize to unit sphere for distance consistency
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in embedding.iter_mut() {
                *val /= norm;
            }
        }
        
        embedding
    }

    fn generate_branch_vector(&self, seed: u64) -> Vec<f32> {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        let mut rng = StdRng::seed_from_u64(seed);
        let mut vec = vec![0.0f32; self.dimension];
        for i in 0..self.dimension {
            vec[i] = rng.gen_range(-1.0..1.0);
        }
        vec
    }
}

/// A universal representation of code logic across multiple languages.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogicComponent {
    pub name: String,
    pub kind: String, // "closure", "unsafe_gate", "logic_flow"
    pub language: String,
    pub range: (usize, usize),
}

pub struct UniversalLogic;

impl UniversalLogic {
    pub fn map_content(language: &str, content: &str) -> Vec<LogicComponent> {
        let mut components = Vec::new();
        
        match language {
            "rust" | "rs" => {
                if content.contains("unsafe") {
                    components.push(LogicComponent {
                        name: "UnsafeBlock".to_string(),
                        kind: "unsafe_gate".to_string(),
                        language: "rust".to_string(),
                        range: (0, 0),
                    });
                }
            },
            "typescript" | "ts" | "tsx" => {
                 if content.contains("eval(") || content.contains("dangerouslySetInnerHTML") {
                    components.push(LogicComponent {
                        name: "DangerousSuture".to_string(),
                        kind: "unsafe_gate".to_string(),
                        language: "typescript".to_string(),
                        range: (0, 0),
                    });
                }
            },
            "python" | "py" => {
                if content.contains("exec(") || content.contains("pickle.load") {
                    components.push(LogicComponent {
                        name: "PythonExecGate".to_string(),
                        kind: "unsafe_gate".to_string(),
                        language: "python".to_string(),
                        range: (0, 0),
                    });
                }
            },
            _ => {}
        }
        
        components
    }
}
