//! Bundled language servers — shipped in `binaries/lsp/` or auto-downloaded to
//! `{app_config}/lsp/`. Covers Rust, TS/JS (web + React Native), Python, Go,
//! Kotlin, Java/Android (jdtls), Flutter/Dart, and Swift (macOS / PATH).

use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::{copy, Cursor};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// Known IDE-managed language servers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BundledLspId {
    RustAnalyzer,
    TypeScript,
    Python,
    Go,
    Kotlin,
    Java,
    Dart,
    Swift,
    Cpp,
    CSharp,
    Ruby,
    Php,
    Lua,
    Zig,
    Bash,
    WebMarkup,
    Elixir,
    R,
}

impl BundledLspId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RustAnalyzer => "rust-analyzer",
            Self::TypeScript => "typescript-language-server",
            Self::Python => "pyright",
            Self::Go => "gopls",
            Self::Kotlin => "kotlin-language-server",
            Self::Java => "jdtls",
            Self::Dart => "dart-language-server",
            Self::Swift => "sourcekit-lsp",
            Self::Cpp => "clangd",
            Self::CSharp => "csharp-ls",
            Self::Ruby => "ruby-lsp",
            Self::Php => "intelephense",
            Self::Lua => "lua-language-server",
            Self::Zig => "zls",
            Self::Bash => "bash-language-server",
            Self::WebMarkup => "vscode-html-language-server",
            Self::Elixir => "elixir-ls",
            Self::R => "languageserver",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::RustAnalyzer => "Rust (rust-analyzer)",
            Self::TypeScript => "TypeScript / JavaScript",
            Self::Python => "Python (pyright)",
            Self::Go => "Go (gopls)",
            Self::Kotlin => "Kotlin",
            Self::Java => "Java / Android (jdtls)",
            Self::Dart => "Dart / Flutter",
            Self::Swift => "Swift / iOS",
            Self::Cpp => "C / C++ (clangd)",
            Self::CSharp => "C# / .NET",
            Self::Ruby => "Ruby",
            Self::Php => "PHP",
            Self::Lua => "Lua",
            Self::Zig => "Zig",
            Self::Bash => "Shell / Bash",
            Self::WebMarkup => "HTML / CSS / JSON",
            Self::Elixir => "Elixir / Phoenix",
            Self::R => "R",
        }
    }

    pub fn stacks(self) -> &'static [&'static str] {
        match self {
            Self::RustAnalyzer => &["Rust", "Tauri"],
            Self::TypeScript => &["Web", "React", "Next.js", "React Native", "Node", "Vue"],
            Self::Python => &["Python", "Django", "FastAPI"],
            Self::Go => &["Go"],
            Self::Kotlin => &["Kotlin", "Android (Gradle)"],
            Self::Java => &["Java", "Android", "Spring"],
            Self::Dart => &["Flutter", "Dart"],
            Self::Swift => &["Swift", "iOS", "macOS"],
            Self::Cpp => &["C", "C++", "CMake", "Embedded"],
            Self::CSharp => &["C#", ".NET", "Unity", "ASP.NET"],
            Self::Ruby => &["Ruby", "Rails"],
            Self::Php => &["PHP", "Laravel", "WordPress"],
            Self::Lua => &["Lua", "Neovim", "Roblox"],
            Self::Zig => &["Zig"],
            Self::Bash => &["Bash", "Shell", "DevOps"],
            Self::WebMarkup => &["HTML", "CSS", "JSON", "Static sites"],
            Self::Elixir => &["Elixir", "Phoenix", "LiveView"],
            Self::R => &["R", "Data science"],
        }
    }

    fn bundle_dir(self) -> &'static str {
        match self {
            Self::RustAnalyzer => "rust-analyzer",
            Self::TypeScript => "typescript-language-server",
            Self::Python => "pyright",
            Self::Go => "gopls",
            Self::Kotlin => "kotlin-language-server",
            Self::Java => "jdtls",
            Self::Dart => "dart-sdk",
            Self::Swift => "sourcekit-lsp",
            Self::Cpp => "clangd",
            Self::CSharp => "csharp-ls",
            Self::Ruby => "ruby-lsp",
            Self::Php => "intelephense",
            Self::Lua => "lua-language-server",
            Self::Zig => "zls",
            Self::Bash => "bash-language-server",
            Self::WebMarkup => "vscode-html-language-server",
            Self::Elixir => "elixir-ls",
            Self::R => "languageserver",
        }
    }

    #[cfg(windows)]
    pub fn exe_leaf(self) -> &'static str {
        match self {
            Self::RustAnalyzer => "rust-analyzer.exe",
            Self::TypeScript => "typescript-language-server.cmd",
            Self::Python => "pyright-langserver.cmd",
            Self::Go => "gopls.exe",
            Self::Kotlin => "kotlin-language-server.exe",
            Self::Java => "jdtls.cmd",
            Self::Dart => "dart.exe",
            Self::Swift => "sourcekit-lsp.exe",
            Self::Cpp => "clangd.exe",
            Self::CSharp => "csharp-ls.exe",
            Self::Ruby => "ruby-lsp.cmd",
            Self::Php => "intelephense.cmd",
            Self::Lua => "lua-language-server.exe",
            Self::Zig => "zls.exe",
            Self::Bash => "bash-language-server.cmd",
            Self::WebMarkup => "vscode-html-language-server.cmd",
            Self::Elixir => "elixir-ls.cmd",
            Self::R => "languageserver.cmd",
        }
    }

    /// Unix bundles ship native binaries or executable shell wrappers with no
    /// extension (generated by scripts/fetch-lsp-binaries.mjs).
    #[cfg(not(windows))]
    pub fn exe_leaf(self) -> &'static str {
        match self {
            Self::RustAnalyzer => "rust-analyzer",
            Self::TypeScript => "typescript-language-server",
            Self::Python => "pyright-langserver",
            Self::Go => "gopls",
            Self::Kotlin => "kotlin-language-server",
            Self::Java => "jdtls",
            Self::Dart => "dart",
            Self::Swift => "sourcekit-lsp",
            Self::Cpp => "clangd",
            Self::CSharp => "csharp-ls",
            Self::Ruby => "ruby-lsp",
            Self::Php => "intelephense",
            Self::Lua => "lua-language-server",
            Self::Zig => "zls",
            Self::Bash => "bash-language-server",
            Self::WebMarkup => "vscode-html-language-server",
            Self::Elixir => "elixir-ls",
            Self::R => "languageserver",
        }
    }

    pub fn path_fallbacks(self) -> &'static [&'static str] {
        match self {
            Self::RustAnalyzer => &["rust-analyzer"],
            Self::TypeScript => &["typescript-language-server", "tsserver"],
            Self::Python => &["pyright-langserver", "pylsp", "python-lsp-server"],
            Self::Go => &["gopls"],
            Self::Kotlin => &["kotlin-language-server"],
            Self::Java => &["jdtls"],
            Self::Dart => &["dart"],
            Self::Swift => &["sourcekit-lsp"],
            Self::Cpp => &["clangd"],
            Self::CSharp => &["csharp-ls", "OmniSharp"],
            Self::Ruby => &["ruby-lsp", "solargraph"],
            Self::Php => &["intelephense", "phpactor"],
            Self::Lua => &["lua-language-server"],
            Self::Zig => &["zls"],
            Self::Bash => &["bash-language-server"],
            Self::WebMarkup => &["vscode-html-language-server"],
            Self::Elixir => &["elixir-ls"],
            Self::R => &["R", "languageserver"],
        }
    }
}

pub fn bundled_id_from_str(s: &str) -> Option<BundledLspId> {
    all_bundled_lsp_ids()
        .iter()
        .copied()
        .find(|id| id.as_str() == s)
}

pub fn all_bundled_lsp_ids() -> &'static [BundledLspId] {
    &[
        BundledLspId::RustAnalyzer,
        BundledLspId::TypeScript,
        BundledLspId::Python,
        BundledLspId::Go,
        BundledLspId::Kotlin,
        BundledLspId::Java,
        BundledLspId::Dart,
        BundledLspId::Swift,
        BundledLspId::Cpp,
        BundledLspId::CSharp,
        BundledLspId::Ruby,
        BundledLspId::Php,
        BundledLspId::Lua,
        BundledLspId::Zig,
        BundledLspId::Bash,
        BundledLspId::WebMarkup,
        BundledLspId::Elixir,
        BundledLspId::R,
    ]
}

/// Search roots next to the IDE exe (installer layout).
pub fn installer_lsp_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.join("binaries").join("lsp"));
            roots.push(dir.join("resources").join("binaries").join("lsp"));
            roots.push(dir.join("lsp"));
        }
    }
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        roots.push(PathBuf::from(manifest).join("binaries").join("lsp"));
    }
    roots
}

fn cache_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("lsp")
}

fn bundle_subdir(id: BundledLspId) -> PathBuf {
    PathBuf::from(id.bundle_dir())
}

fn find_in_roots(roots: &[PathBuf], rel: &Path) -> Option<PathBuf> {
    for root in roots {
        let cand = root.join(rel);
        if cand.is_file() {
            return Some(cand);
        }
    }
    None
}

fn all_roots(config_dir: &Path) -> Vec<PathBuf> {
    let mut roots = installer_lsp_roots();
    roots.push(cache_dir(config_dir));
    roots
}

/// Resolve a bundled / cached server executable (no download).
pub fn resolve_bundled_exe(id: BundledLspId, config_dir: &Path) -> Option<PathBuf> {
    let leaf = id.exe_leaf();
    let roots = all_roots(config_dir);

    if id == BundledLspId::Java {
        return resolve_jdtls_launcher(&roots);
    }

    if id == BundledLspId::Dart {
        for root in &roots {
            let dart = root.join("dart-sdk").join("bin").join("dart.exe");
            if dart.is_file() {
                return Some(dart);
            }
            let nested = root.join("dart-sdk").join("dart-sdk").join("bin").join("dart.exe");
            if nested.is_file() {
                return Some(nested);
            }
        }
        return None;
    }

    for root in &roots {
        let direct = root.join(leaf);
        if direct.is_file() {
            return Some(direct);
        }
        let nested = root.join(id.bundle_dir()).join(leaf);
        if nested.is_file() {
            return Some(nested);
        }
        if matches!(id, BundledLspId::Kotlin) {
            let alt = root.join(id.bundle_dir()).join("bin").join("kotlin-language-server.exe");
            if alt.is_file() {
                return Some(alt);
            }
        }
    }

    let cached = cache_dir(config_dir).join(id.bundle_dir()).join(leaf);
    if cached.is_file() {
        return Some(cached);
    }
    None
}

fn resolve_jre_java(roots: &[PathBuf]) -> Option<PathBuf> {
    for root in roots {
        let java = root.join("jre").join("bin").join("java.exe");
        if java.is_file() {
            return Some(java);
        }
        let nested = root.join("jre").join("jdk-17").join("bin").join("java.exe");
        if nested.is_file() {
            return Some(nested);
        }
    }
    which::which("java").ok()
}

fn resolve_jdtls_launcher(roots: &[PathBuf]) -> Option<PathBuf> {
    for root in roots {
        let plugins = root.join("jdtls").join("plugins");
        if !plugins.is_dir() {
            continue;
        }
        if let Ok(rd) = fs::read_dir(&plugins) {
            for entry in rd.flatten() {
                let name = entry.file_name();
                let n = name.to_string_lossy();
                if n.starts_with("org.eclipse.equinox.launcher_") && n.ends_with(".jar") {
                    return Some(entry.path());
                }
            }
        }
    }
    None
}

fn jdtls_config_dir(roots: &[PathBuf]) -> Option<PathBuf> {
    for root in roots {
        #[cfg(windows)]
        let cfg = root.join("jdtls").join("config_win");
        #[cfg(target_os = "macos")]
        let cfg = root.join("jdtls").join("config_mac");
        #[cfg(all(unix, not(target_os = "macos")))]
        let cfg = root.join("jdtls").join("config_linux");
        if cfg.is_dir() {
            return Some(cfg);
        }
    }
    None
}

fn jdtls_workspace_data(config_dir: &Path, workspace_root: &Path) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    workspace_root.to_string_lossy().hash(&mut hasher);
    let hash = hasher.finish();
    let dir = config_dir
        .join("jdtls-workspaces")
        .join(format!("{:016x}", hash));
    let _ = fs::create_dir_all(&dir);
    dir
}

/// Launch spec: command + args for `LspClient::start`.
pub struct ResolvedLaunch {
    pub id: String,
    pub command: String,
    pub args: Vec<String>,
    pub source: String,
}

pub fn resolve_launch(
    id: BundledLspId,
    config_dir: &Path,
    workspace_root: Option<&Path>,
) -> Option<ResolvedLaunch> {
    let roots = all_roots(config_dir);

    if id == BundledLspId::Java {
        return resolve_jdtls_launch(config_dir, workspace_root, &roots);
    }

    if id == BundledLspId::Swift {
        return resolve_swift_launch(&roots);
    }

    if id == BundledLspId::Dart {
        if let Some(dart) = resolve_bundled_exe(id, config_dir) {
            return Some(ResolvedLaunch {
                id: id.as_str().to_string(),
                command: dart.to_string_lossy().to_string(),
                args: vec![
                    "language-server".into(),
                    "--protocol=lsp".into(),
                ],
                source: "bundled".into(),
            });
        }
    }

    if let Some(exe) = resolve_bundled_exe(id, config_dir) {
        let (command, args) = launch_for_path(id, &exe)?;
        return Some(ResolvedLaunch {
            id: id.as_str().to_string(),
            command,
            args,
            source: "bundled".into(),
        });
    }

    // PATH fallback (dev machines with global installs — VS Code / SDK PATH)
    for name in id.path_fallbacks() {
        if let Ok(cmd) = which::which(name) {
            let p = cmd.to_string_lossy().to_string();
            if let Some((command, args)) = launch_for_path(id, Path::new(&p)) {
                return Some(ResolvedLaunch {
                    id: id.as_str().to_string(),
                    command,
                    args,
                    source: "path".into(),
                });
            }
        }
    }
    None
}

fn resolve_jdtls_launch(
    config_dir: &Path,
    workspace_root: Option<&Path>,
    roots: &[PathBuf],
) -> Option<ResolvedLaunch> {
    let java = resolve_jre_java(roots)?;
    let launcher = resolve_jdtls_launcher(roots)?;
    let config = jdtls_config_dir(roots)?;
    let data = workspace_root
        .map(|r| jdtls_workspace_data(config_dir, r))
        .unwrap_or_else(|| config_dir.join("jdtls-workspaces").join("default"));
    let _ = fs::create_dir_all(&data);

    Some(ResolvedLaunch {
        id: BundledLspId::Java.as_str().to_string(),
        command: java.to_string_lossy().to_string(),
        args: vec![
            "-Declipse.application=org.eclipse.jdt.ls.core.id1".into(),
            "-Dosgi.bundles.defaultStartLevel=4".into(),
            "-Declipse.product=org.eclipse.jdt.ls.core.product".into(),
            "-Xmx1G".into(),
            "--add-modules=ALL-SYSTEM".into(),
            "--add-opens".into(),
            "java.base/java.util=ALL-UNNAMED".into(),
            "--add-opens".into(),
            "java.base/java.lang=ALL-UNNAMED".into(),
            "-jar".into(),
            launcher.to_string_lossy().to_string(),
            "-configuration".into(),
            config.to_string_lossy().to_string(),
            "-data".into(),
            data.to_string_lossy().to_string(),
        ],
        source: "bundled".into(),
    })
}

fn resolve_swift_launch(roots: &[PathBuf]) -> Option<ResolvedLaunch> {
    let leaf = if cfg!(windows) { "sourcekit-lsp.exe" } else { "sourcekit-lsp" };
    for root in roots {
        let exe = root.join("sourcekit-lsp").join(leaf);
        if exe.is_file() {
            return Some(ResolvedLaunch {
                id: BundledLspId::Swift.as_str().to_string(),
                command: exe.to_string_lossy().to_string(),
                args: vec![],
                source: "bundled".into(),
            });
        }
    }
    #[cfg(target_os = "macos")]
    {
        if which::which("sourcekit-lsp").is_ok() {
            return Some(ResolvedLaunch {
                id: BundledLspId::Swift.as_str().to_string(),
                command: "xcrun".into(),
                args: vec!["sourcekit-lsp".into()],
                source: "xcode".into(),
            });
        }
    }
    which::which("sourcekit-lsp").ok().map(|p| ResolvedLaunch {
        id: BundledLspId::Swift.as_str().to_string(),
        command: p.to_string_lossy().to_string(),
        args: vec![],
        source: "path".into(),
    })
}

fn launch_for_path(id: BundledLspId, path: &Path) -> Option<(String, Vec<String>)> {
    match id {
        BundledLspId::RustAnalyzer | BundledLspId::Go | BundledLspId::Kotlin
        | BundledLspId::Cpp | BundledLspId::CSharp | BundledLspId::Lua | BundledLspId::Zig => {
            Some((path.to_string_lossy().to_string(), vec![]))
        }
        BundledLspId::TypeScript | BundledLspId::Python | BundledLspId::Ruby | BundledLspId::Php
        | BundledLspId::Bash | BundledLspId::WebMarkup | BundledLspId::Elixir | BundledLspId::R => {
            if path.extension().and_then(|e| e.to_str()) == Some("cmd") {
                Some((
                    "cmd".into(),
                    vec!["/C".into(), path.to_string_lossy().to_string(), "--stdio".into()],
                ))
            } else {
                Some((path.to_string_lossy().to_string(), vec!["--stdio".into()]))
            }
        }
        BundledLspId::Dart => Some((
            path.to_string_lossy().to_string(),
            vec!["language-server".into(), "--protocol=lsp".into()],
        )),
        BundledLspId::Java | BundledLspId::Swift => None,
    }
}

fn workspace_has(root: &Path, rel: &str) -> bool {
    root.join(rel).exists()
}

fn read_text(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn file_contains(path: &Path, needle: &str) -> bool {
    read_text(path)
        .map(|s| s.contains(needle))
        .unwrap_or(false)
}

pub fn glob_exists(root: &Path, pattern: &str) -> bool {
    glob::glob(&format!("{}/{}", root.display(), pattern))
        .ok()
        .map(|g| g.filter_map(Result::ok).next().is_some())
        .unwrap_or(false)
}

pub fn has_web_marker(root: &Path) -> bool {
    for name in [
        "vite.config.ts",
        "vite.config.js",
        "next.config.js",
        "next.config.mjs",
        "next.config.ts",
        "nuxt.config.ts",
        "angular.json",
        "svelte.config.js",
        "astro.config.mjs",
        "webpack.config.js",
        "index.html",
    ] {
        if workspace_has(root, name) {
            return true;
        }
    }
    false
}

fn package_json_deps(root: &Path) -> String {
    read_text(&root.join("package.json")).unwrap_or_default().to_lowercase()
}

fn is_react_native(root: &Path) -> bool {
    let pj = package_json_deps(root);
    pj.contains("react-native") || pj.contains("expo")
}

fn is_android_gradle(root: &Path) -> bool {
    for name in ["build.gradle", "build.gradle.kts", "settings.gradle.kts"] {
        let p = root.join(name);
        if p.is_file()
            && (file_contains(&p, "com.android.application")
                || file_contains(&p, "com.android.library")
                || file_contains(&p, "android {"))
        {
            return true;
        }
    }
    if workspace_has(root, "AndroidManifest.xml") {
        return true;
    }
    workspace_has(root, "app/build.gradle") || workspace_has(root, "app/build.gradle.kts")
}

fn uses_kotlin_gradle(root: &Path) -> bool {
    for name in ["build.gradle", "build.gradle.kts", "settings.gradle.kts"] {
        let p = root.join(name);
        if p.is_file()
            && (file_contains(&p, "org.jetbrains.kotlin")
                || file_contains(&p, "kotlin-android")
                || file_contains(&p, "kotlin(\"android\")"))
        {
            return true;
        }
    }
    false
}

fn has_swift_project(root: &Path) -> bool {
    if workspace_has(root, "Package.swift") {
        return true;
    }
    if let Ok(rd) = fs::read_dir(root) {
        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".xcodeproj") || name.ends_with(".xcworkspace") {
                return true;
            }
        }
    }
    false
}

/// Ranked workspace → language server mapping (highest score first).
pub fn workspace_lsp_candidates(root: &Path) -> Vec<(BundledLspId, u8, &'static str)> {
    let mut scores: Vec<(BundledLspId, u8, &'static str)> = Vec::new();

    if workspace_has(root, "Cargo.toml") {
        scores.push((BundledLspId::RustAnalyzer, 100, "Cargo.toml"));
    }
    if workspace_has(root, "pubspec.yaml") {
        scores.push((BundledLspId::Dart, 98, "pubspec.yaml (Flutter/Dart)"));
    }
    if is_android_gradle(root) {
        if uses_kotlin_gradle(root) {
            scores.push((BundledLspId::Kotlin, 96, "Android Gradle + Kotlin"));
        }
        scores.push((BundledLspId::Java, 94, "Android / Java Gradle"));
    } else if workspace_has(root, "build.gradle") || workspace_has(root, "build.gradle.kts") {
        if uses_kotlin_gradle(root) {
            scores.push((BundledLspId::Kotlin, 92, "Gradle + Kotlin"));
        }
        scores.push((BundledLspId::Java, 88, "Gradle / Java"));
    }
    if workspace_has(root, "pom.xml") {
        scores.push((BundledLspId::Java, 90, "Maven pom.xml"));
    }
    if is_react_native(root) {
        scores.push((BundledLspId::TypeScript, 95, "React Native / Expo"));
    }
    if workspace_has(root, "go.mod") {
        scores.push((BundledLspId::Go, 85, "go.mod"));
    }
    if workspace_has(root, "package.json")
        || workspace_has(root, "tsconfig.json")
        || workspace_has(root, "jsconfig.json")
        || has_web_marker(root)
    {
        scores.push((BundledLspId::TypeScript, 80, "Web / TS / JS project"));
    }
    if workspace_has(root, "pyproject.toml")
        || workspace_has(root, "requirements.txt")
        || workspace_has(root, "setup.py")
    {
        scores.push((BundledLspId::Python, 75, "Python project"));
    }
    if has_swift_project(root) {
        scores.push((BundledLspId::Swift, 93, "Swift / Xcode project"));
    }
    if workspace_has(root, "CMakeLists.txt") || workspace_has(root, "compile_commands.json") {
        scores.push((BundledLspId::Cpp, 87, "C/C++ (CMake)"));
    }
    if glob_exists(root, "*.csproj") || glob_exists(root, "*.sln") || workspace_has(root, "global.json") {
        scores.push((BundledLspId::CSharp, 86, "C# / .NET solution"));
    }
    if workspace_has(root, "Gemfile") {
        scores.push((BundledLspId::Ruby, 84, "Ruby / Rails"));
    }
    if workspace_has(root, "composer.json") {
        scores.push((BundledLspId::Php, 83, "PHP / Composer"));
    }
    if workspace_has(root, "mix.exs") {
        scores.push((BundledLspId::Elixir, 82, "Elixir / Phoenix"));
    }
    if workspace_has(root, "build.zig") || workspace_has(root, "build.zig.zon") {
        scores.push((BundledLspId::Zig, 81, "Zig project"));
    }
    if has_web_marker(root) && !workspace_has(root, "package.json") {
        scores.push((BundledLspId::WebMarkup, 78, "HTML/CSS static site"));
    }
    if workspace_has(root, "DESCRIPTION") && workspace_has(root, "NAMESPACE") {
        scores.push((BundledLspId::R, 77, "R package"));
    }
    if glob_exists(root, "*.lua") && scores.iter().all(|(id, _, _)| *id != BundledLspId::Lua) {
        scores.push((BundledLspId::Lua, 70, "Lua sources"));
    }

    scores.sort_by(|a, b| b.1.cmp(&a.1));
    scores.dedup_by(|a, b| a.0 == b.0);
    scores
}

/// Primary language server for this workspace.
pub fn workspace_lsp_id(root: &Path) -> Option<BundledLspId> {
    workspace_lsp_candidates(root)
        .into_iter()
        .next()
        .map(|(id, _, _)| id)
}

fn mirror_base() -> String {
    std::env::var("LSP_BUNDLE_MIRROR")
        .or_else(|_| std::env::var("CYBERIFRIT_LSP_MIRROR"))
        .unwrap_or_else(|_| "https://github.com/rust-lang/rust-analyzer/releases/download".to_string())
}

/// rust-analyzer release triple for the *current* platform.
fn rust_analyzer_triple() -> &'static str {
    if cfg!(all(target_os = "windows", target_arch = "aarch64")) {
        "aarch64-pc-windows-msvc"
    } else if cfg!(target_os = "windows") {
        "x86_64-pc-windows-msvc"
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "aarch64-apple-darwin"
    } else if cfg!(target_os = "macos") {
        "x86_64-apple-darwin"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64-unknown-linux-gnu"
    } else {
        "x86_64-unknown-linux-gnu"
    }
}

/// Download rust-analyzer for the current OS/arch into cache if missing.
/// Windows assets are `.zip`; macOS/Linux assets are single-binary `.gz`.
pub async fn ensure_rust_analyzer(config_dir: &Path) -> Result<PathBuf, String> {
    if let Some(p) = resolve_bundled_exe(BundledLspId::RustAnalyzer, config_dir) {
        return Ok(p);
    }
    let leaf = BundledLspId::RustAnalyzer.exe_leaf();
    let dest_dir = cache_dir(config_dir).join("rust-analyzer");
    let dest_exe = dest_dir.join(leaf);
    if dest_exe.is_file() {
        return Ok(dest_exe);
    }
    fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;

    let triple = rust_analyzer_triple();
    let ext = if cfg!(windows) { "zip" } else { "gz" };
    let client = reqwest::Client::builder()
        .user_agent("vscodium-rust-ide/1.0")
        .build()
        .map_err(|e| e.to_string())?;
    let release: serde_json::Value = client
        .get("https://api.github.com/repos/rust-lang/rust-analyzer/releases/latest")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let tag = release["tag_name"].as_str().unwrap_or("nightly");
    let asset_url = release["assets"]
        .as_array()
        .and_then(|a| {
            a.iter().find(|x| {
                x["name"]
                    .as_str()
                    .map(|n| n.contains(triple) && n.ends_with(ext))
                    .unwrap_or(false)
            })
        })
        .and_then(|x| x["browser_download_url"].as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}/{}/rust-analyzer-{}.{}", mirror_base(), tag, triple, ext));

    if cfg!(windows) {
        download_zip_exe(&client, &asset_url, &dest_dir, leaf).await?;
    } else {
        download_gz_exe(&client, &asset_url, &dest_exe).await?;
    }
    Ok(dest_exe)
}

/// gopls has no official prebuilt release binaries — the supported install is
/// `go install golang.org/x/tools/gopls@latest`. Use the local Go toolchain.
pub async fn ensure_gopls(config_dir: &Path) -> Result<PathBuf, String> {
    if let Some(p) = resolve_bundled_exe(BundledLspId::Go, config_dir) {
        return Ok(p);
    }
    let leaf = BundledLspId::Go.exe_leaf();
    let dest_dir = cache_dir(config_dir).join("gopls");
    let dest_exe = dest_dir.join(leaf);
    if dest_exe.is_file() {
        return Ok(dest_exe);
    }
    fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;

    let go = which::which("go").map_err(|_| {
        "gopls requires a Go toolchain. Install Go (https://go.dev/dl) and reopen the workspace, \
         or install gopls yourself: `go install golang.org/x/tools/gopls@latest`"
            .to_string()
    })?;

    let dest_dir_owned = dest_dir.clone();
    let output = tokio::task::spawn_blocking(move || {
        std::process::Command::new(go)
            .args(["install", "golang.org/x/tools/gopls@latest"])
            .env("GOBIN", &dest_dir_owned)
            .output()
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| format!("go install gopls: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "go install gopls failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    if dest_exe.is_file() {
        Ok(dest_exe)
    } else {
        Err("go install completed but gopls binary not found in cache".to_string())
    }
}

/// Decompress a single-binary `.gz` asset (rust-analyzer's Unix format) and
/// mark it executable.
async fn download_gz_exe(
    client: &reqwest::Client,
    url: &str,
    dest_exe: &Path,
) -> Result<(), String> {
    let bytes = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("download {url}: {e}"))?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let mut decoder = flate2::read::GzDecoder::new(Cursor::new(bytes));
    let mut out = Vec::new();
    copy(&mut decoder, &mut out).map_err(|e| format!("gunzip: {e}"))?;
    fs::write(dest_exe, &out).map_err(|e| e.to_string())?;
    set_executable(dest_exe);
    Ok(())
}

#[cfg(unix)]
fn set_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) {}

async fn download_zip_exe(
    client: &reqwest::Client,
    url: &str,
    dest_dir: &Path,
    exe_name: &str,
) -> Result<(), String> {
    let bytes = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("download {url}: {e}"))?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let reader = Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader).map_err(|e| format!("zip: {e}"))?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = file.name().to_string();
        if name.ends_with(exe_name) || name.contains(exe_name) {
            let out = dest_dir.join(exe_name);
            let mut outfile = File::create(&out).map_err(|e| e.to_string())?;
            copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
            drop(outfile);
            set_executable(&out);
            return Ok(());
        }
    }
    Err(format!("{exe_name} not found in archive from {url}"))
}

pub fn bundled_or_error(id: BundledLspId, config_dir: &Path) -> Result<(), String> {
    if resolve_bundled_exe(id, config_dir).is_some() || resolve_launch(id, config_dir, None).is_some() {
        Ok(())
    } else {
        Err(format!(
            "{} is not bundled. Run `node scripts/fetch-lsp-binaries.mjs` (any OS) or scripts/fetch-lsp-binaries.ps1 (Windows) before release.",
            id.label()
        ))
    }
}

/// Ensure the IDE-managed server for this workspace is present (download if needed).
pub async fn ensure_workspace_lsp(root: &Path, config_dir: &Path) -> Result<ResolvedLaunch, String> {
    let id = workspace_lsp_id(root).ok_or(
        "No language server mapping for this workspace. Open a Rust, TS/JS, Flutter, Android, Java, Kotlin, Go, Python, or Swift project.",
    )?;

    match id {
        BundledLspId::RustAnalyzer => {
            let _ = ensure_rust_analyzer(config_dir).await?;
        }
        BundledLspId::Go => {
            let _ = ensure_gopls(config_dir).await?;
        }
        BundledLspId::TypeScript
        | BundledLspId::Python
        | BundledLspId::Kotlin
        | BundledLspId::Java
        | BundledLspId::Dart
        | BundledLspId::Swift
        | BundledLspId::Cpp
        | BundledLspId::CSharp
        | BundledLspId::Ruby
        | BundledLspId::Php
        | BundledLspId::Lua
        | BundledLspId::Zig
        | BundledLspId::Bash
        | BundledLspId::WebMarkup
        | BundledLspId::Elixir
        | BundledLspId::R => {
            bundled_or_error(id, config_dir)?;
        }
    }

    resolve_launch(id, config_dir, Some(root))
        .ok_or_else(|| format!("Failed to resolve {}", id.label()))
}

pub fn detect_workspace_lsp_json(root: &Path, config_dir: &Path) -> serde_json::Value {
    let candidates: Vec<_> = workspace_lsp_candidates(root)
        .into_iter()
        .map(|(id, score, reason)| {
            serde_json::json!({
                "id": id.as_str(),
                "label": id.label(),
                "stacks": id.stacks(),
                "score": score,
                "reason": reason,
                "installed": resolve_bundled_exe(id, config_dir).is_some() || resolve_launch(id, config_dir, Some(root)).is_some(),
            })
        })
        .collect();
    let primary = candidates.first().cloned();
    serde_json::json!({
        "primary": primary,
        "candidates": candidates,
    })
}

pub fn bundle_status(config_dir: &Path) -> serde_json::Value {
    let mut servers = Vec::new();
    for &id in all_bundled_lsp_ids() {
        let resolved = resolve_bundled_exe(id, config_dir);
        let launchable = resolve_launch(id, config_dir, None).is_some();
        servers.push(serde_json::json!({
            "id": id.as_str(),
            "label": id.label(),
            "stacks": id.stacks(),
            "installed": resolved.is_some() || launchable,
            "path": resolved.map(|p| p.to_string_lossy().to_string()),
            "launchable": launchable,
        }));
    }
    serde_json::json!({ "servers": servers, "managed": true })
}
