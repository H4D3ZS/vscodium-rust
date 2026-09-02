//! VS Code-style LSP catalog — popular marketplace servers users can install.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspPreset {
    pub id: String,
    pub name: String,
    pub languages: Vec<String>,
    /// "npm" | "path"
    pub install_kind: String,
    pub npm_package: Option<String>,
    pub path_commands: Vec<String>,
    pub default_args: Vec<String>,
    pub note: String,
}

pub fn lsp_presets() -> Vec<LspPreset> {
    vec![
        preset("yaml-language-server", "YAML", &["yaml", "yml"], "npm", Some("yaml-language-server"), &[], &["--stdio"], "Open VSX / Red Hat YAML"),
        preset("dockerfile-language-server", "Dockerfile", &["dockerfile"], "npm", Some("dockerfile-language-server-nodejs"), &[], &["--stdio"], "Dockerfile LSP"),
        preset("prisma-language-server", "Prisma", &["prisma"], "npm", Some("@prisma/language-server"), &[], &["--stdio"], "Prisma schema"),
        preset("svelte-language-server", "Svelte", &["svelte"], "npm", Some("svelte-language-server"), &[], &["--stdio"], "Svelte components"),
        preset("vue-language-server", "Vue (Volar)", &["vue"], "npm", Some("@vue/language-server"), &[], &["--stdio"], "Vue 3 / Volar"),
        preset("tailwindcss-language-server", "Tailwind CSS", &["css", "html"], "npm", Some("@tailwindcss/language-server"), &[], &["--stdio"], "Tailwind v4 classes"),
        preset("graphql-language-service", "GraphQL", &["graphql"], "npm", Some("graphql-language-service-cli"), &[], &["server", "-m", "stream"], "GraphQL schemas"),
        preset("bash-language-server-user", "Bash (npm)", &["shellscript", "bash"], "npm", Some("bash-language-server"), &[], &["start"], "Shell scripts via npm"),
        preset("terraform-ls", "Terraform", &["terraform", "hcl"], "path", None, &["terraform-ls"], &["serve"], "HashiCorp terraform-ls"),
        preset("marksman", "Markdown", &["markdown"], "path", None, &["marksman"], &["server"], "Markdown LSP"),
        preset("texlab", "LaTeX", &["latex", "tex"], "path", None, &["texlab"], &[], "LaTeX documents"),
        preset("haskell-language-server", "Haskell", &["haskell"], "path", None, &["haskell-language-server-wrapper", "haskell-language-server"], &["--lsp"], "HLS via stack/cabal PATH"),
        preset("phpactor", "PHP (phpactor)", &["php"], "path", None, &["phpactor"], &["language-server"], "Alternative to intelephense"),
        preset("solargraph", "Ruby (Solargraph)", &["ruby"], "path", None, &["solargraph"], &["stdio"], "Alternative to ruby-lsp"),
        preset("jedi-language-server", "Python (Jedi)", &["python"], "npm", Some("jedi-language-server"), &[], &[], "Lightweight Python LSP"),
        preset("biome", "Biome", &["javascript", "typescript", "json"], "path", None, &["biome"], &["lsp-proxy", "--stdio"], "Biome formatter/linter LSP"),
        preset("clangd-user", "C/C++ (clangd)", &["c", "cpp"], "path", None, &["clangd"], &[], "Import custom clangd from PATH"),
        preset("omnisharp", "C# (OmniSharp)", &["csharp"], "path", None, &["OmniSharp"], &["-lsp"], ".NET — use with dotnet SDK on PATH"),
    ]
}

fn preset(
    id: &str,
    name: &str,
    languages: &[&str],
    install_kind: &str,
    npm_package: Option<&str>,
    path_commands: &[&str],
    default_args: &[&str],
    note: &str,
) -> LspPreset {
    LspPreset {
        id: id.to_string(),
        name: name.to_string(),
        languages: languages.iter().map(|s| s.to_string()).collect(),
        install_kind: install_kind.to_string(),
        npm_package: npm_package.map(String::from),
        path_commands: path_commands.iter().map(|s| s.to_string()).collect(),
        default_args: default_args.iter().map(|s| s.to_string()).collect(),
        note: note.to_string(),
    }
}

pub fn preset_file_extensions(preset_id: &str) -> Vec<String> {
    match preset_id {
        "yaml-language-server" => vec!["yaml", "yml"],
        "dockerfile-language-server" => vec!["dockerfile"],
        "prisma-language-server" => vec!["prisma"],
        "svelte-language-server" => vec!["svelte"],
        "vue-language-server" => vec!["vue"],
        "tailwindcss-language-server" => vec!["css", "html", "htm"],
        "graphql-language-service" => vec!["graphql", "gql"],
        "bash-language-server-user" => vec!["sh", "bash", "zsh"],
        "terraform-ls" => vec!["tf", "hcl"],
        "marksman" => vec!["md", "markdown"],
        "texlab" => vec!["tex", "latex"],
        "jedi-language-server" => vec!["py", "pyi"],
        "biome" => vec!["js", "jsx", "ts", "tsx", "json", "jsonc"],
        "haskell-language-server" => vec!["hs", "lhs"],
        "phpactor" => vec!["php"],
        "solargraph" => vec!["rb"],
        "clangd-user" => vec!["c", "cpp", "cc", "cxx", "h", "hpp"],
        "omnisharp" => vec!["cs"],
        _ => vec![],
    }
    .into_iter()
    .map(String::from)
    .collect()
}

pub fn find_preset(id: &str) -> Option<LspPreset> {
    lsp_presets().into_iter().find(|p| p.id == id)
}
