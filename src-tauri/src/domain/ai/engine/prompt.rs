//! Tool-list construction for prompts + builtin/MCP tool summaries.
use futures::StreamExt;
use serde_json::{json, Value};
use std::path::Path;
use super::types::*;
use super::sentient::Sentient;

impl Sentient {
    /// Dynamically get AI tools and MCP tools available
    pub async fn get_available_tools(&self) -> Vec<Value> {
        let mut tools = self
            .ai_tools
            .list_tools()
            .into_iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.input_schema
                    }
                })
            })
            .collect::<Vec<_>>();

        if let Ok(mcp_tools) = self.mcp_registry.list_tools().await {
            for tool in mcp_tools {
                tools.push(json!({"type": "function", "function": tool}));
            }
        }

        // Add offensive specialized tools
        for tool in self.get_offensive_tools() {
            tools.push(tool);
        }

        tools
    }

    /// Offensive tools are defined in `AiTools::list_tools()` with live execution handlers.
    pub(crate) fn get_offensive_tools(&self) -> Vec<Value> {
        vec![]
    }

    pub(crate) fn is_cyberifrit_managed_ollama_url(url: &str) -> bool {
        let u = url.to_lowercase();
        u.contains("ai.cyberifrit.xyz") || u.contains("api.cyberifrit.xyz")
    }

    pub(crate) fn ollama_auth_hint(base_url: &str, status_code: u16) -> &'static str {
        if Self::is_cyberifrit_managed_ollama_url(base_url) {
            return match status_code {
                401 => "Sign in to Cyber-Ifrit (Settings → Account) to use Cyber-Ifrit Cloud.",
                402 => "Your plan does not include Cyber-Ifrit Cloud. Start the free 1-day trial, subscribe to Pro+, or use Local Ollama / your own API keys.",
                403 => "Cyber-Ifrit Cloud access denied. Sync Settings → Account or upgrade your plan.",
                _ => "Cyber-Ifrit Cloud auth failed — sign in and sync Settings → Account.",
            };
        }
        match status_code {
            401 | 403 => "Server replied with auth failure. If your Ollama proxy requires a bearer token, paste it in Settings → Providers.",
            402 => "Connected, but this endpoint rejected the request (HTTP 402). Check your subscription or proxy policy.",
            _ => "Server returned a non-2xx status. See the body preview below.",
        }
    }

    /// Resolve Ollama base URL: request override first, then engine state.
    pub(crate) async fn resolved_ollama_base(&self, req: &AiRequest) -> String {
        let raw = if let Some(u) = req.ollama_url.as_ref().filter(|u| !u.trim().is_empty()) {
            u.clone()
        } else {
            self.ollama_url.lock().await.clone()
        };
        normalize_ollama_base_url(&raw)
    }

    /// Ollama bearer: explicit `ollama` key first, else Supabase session JWT on managed cloud.
    pub(crate) fn ollama_bearer_for_base(&self, base_url: &str) -> String {
        let k = self.get_key_for_provider("ollama");
        if !k.trim().is_empty() {
            return k;
        }
        String::new()
    }

    pub(crate) fn get_key_for_provider(&self, provider: &str) -> String {
        let provider_base = provider.split(':').next().unwrap_or(provider).to_lowercase();
        
        let env_var = match provider_base.as_str() {
            "anthropic" => "ANTHROPIC_API_KEY",
            "google" | "gemini" => "GOOGLE_API_KEY",
            "groq" => "GROQ_API_KEY",
            "openrouter" => "OPENROUTER_API_KEY",
            "deepseek" => "DEEPSEEK_API_KEY",
            // Xiaomi MiMo Token Plan key (Bearer, OpenAI-compatible).
            "mimo" | "xiaomi" => "MIMO_API_KEY",
            // Interface AI / highwayapi.ai — OpenAI-compatible, serves Claude
            // Opus 4.8 (BYO key; free + paid base URLs).
            "highwayapi" | "interfaceai" | "jiekou" => "HIGHWAYAPI_API_KEY",
            // Cyber-Ifrit Cloud subscription token (a JWT, stored like an API key).
            "cyberifrit" | "cyber-ifrit" | "cyberifrit-cloud" => "CYBERIFRIT_API_KEY",
            // Local DeepSeek-V2 server is keyless by default. If the user
            // fronted it with an auth proxy they can still set this var.
            "deepseek-ane" | "deepseek_ane" | "ds2-ane" => "DEEPSEEK_ANE_API_KEY",
            "xai" => "XAI_API_KEY",
            "cerebras" => "CEREBRAS_API_KEY",
            "alibaba" => "ALIBABA_API_KEY",
            "nvidia" => "NVIDIA_API_KEY",
            "apiradar" => "APIRADAR_API_KEY",
            "mistral" => "MISTRAL_API_KEY",
            "openai" => "OPENAI_API_KEY",
            "ollama" => "OLLAMA_API_KEY",
            "vllm" => "VLLM_API_KEY",
            "lmstudio" | "lm-studio" | "lm_studio" => "LMSTUDIO_API_KEY",
            "litellm" | "lite-llm" | "lite_llm" => "LITELLM_API_KEY",
            _ => "OPENAI_API_KEY",
        };
        
        if let Ok(val) = std::env::var(env_var) {
            if !val.is_empty() { return val; }
        }

        // Fallback to api_keys.json in config dir
        let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");
        if let Ok(content) = std::fs::read_to_string(&keys_path) {
            if let Ok(keys) = serde_json::from_str::<Value>(&content) {
                // If provider is gemini, check both "google" and "gemini" keys
                let lookup = if provider_base == "gemini" { "google" } else { &provider_base };
                if let Some(key) = keys[lookup].as_str() {
                    if !key.is_empty() { return key.to_string(); }
                }
                if let Some(key) = keys[provider_base.clone()].as_str() {
                    if !key.is_empty() { return key.to_string(); }
                }
            }
        }

        self.api_key.clone()
    }

    pub(crate) fn get_endpoint(&self, provider: &str, req: &AiRequest) -> String {
        let provider_base = if req.model.to_lowercase().contains("claude-opus-4-8") {
            "highwayapi".to_string()
        } else {
            provider.split(':').next().unwrap_or(provider).to_lowercase()
        };
        match provider_base.as_str() {
            // Headless web-chat models are fronted by the local OpenAI shim (:1539),
            // which drives the logged-in claude.ai / deepseek session. Keyless.
            "webchat" | "webchat-claude" | "webchat-deepseek" => {
                "http://127.0.0.1:1539/v1/chat/completions".to_string()
            }
            "google" | "gemini" => {
                if let Ok(url) = std::env::var("GOOGLE_BASE_URL") {
                    if !url.is_empty() {
                        let base = url.trim().trim_end_matches('/').to_string();
                        if base.ends_with("/v1/chat/completions") { return base; }
                        else if base.ends_with("/v1") { return format!("{}/chat/completions", base); }
                        else { return format!("{}/v1/chat/completions", base); }
                    }
                }
                let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");
                if let Ok(content) = std::fs::read_to_string(&keys_path) {
                    if let Ok(keys) = serde_json::from_str::<Value>(&content) {
                        if let Some(custom_url) = keys["google_base_url"].as_str() {
                            if !custom_url.is_empty() {
                                let base = custom_url.trim().trim_end_matches('/').to_string();
                                if base.ends_with("/v1/chat/completions") { return base; }
                                else if base.ends_with("/v1") { return format!("{}/chat/completions", base); }
                                else { return format!("{}/v1/chat/completions", base); }
                            }
                        }
                    }
                }

                // Cloud goes DIRECT — never auto-route through the local :1536 AIM proxy.
                // The proxy is for LOCAL Ollama context injection; sending a cloud request
                // through it double-injects context the IDE already adds in-process AND
                // hangs the request to the 60s timeout if the proxy is up but not
                // forwarding (the cause of "Gemini just times out / takes a minute").
                "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions".to_string()
            }
            "anthropic" => {
                if let Ok(url) = std::env::var("ANTHROPIC_BASE_URL") {
                    if !url.is_empty() { return url; }
                }
                let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");
                if let Ok(content) = std::fs::read_to_string(&keys_path) {
                    if let Ok(keys) = serde_json::from_str::<Value>(&content) {
                        if let Some(custom_url) = keys["anthropic_base_url"].as_str() {
                            if !custom_url.is_empty() { return custom_url.to_string(); }
                        }
                    }
                }
                "https://api.anthropic.com/v1/messages".to_string()
            }
            // JieKou AI / Highway API — OpenAI-compatible Claude Opus 4.8.
            // Docs use base URL https://api.highwayapi.ai/openai and curl the
            // concrete endpoint /openai/v1/chat/completions. Older builds sent
            // the model to freeapi.highwayapi.ai, whose catalog can reject
            // claude-opus-4-8 with "model not found".
            "highwayapi" | "interfaceai" | "jiekou" => {
                let configured = std::env::var("HIGHWAYAPI_BASE_URL").ok()
                    .filter(|s| !s.trim().is_empty())
                    .or_else(|| {
                        self.brain_dir.parent()
                            .map(|p| p.join("api_keys.json"))
                            .and_then(|p| std::fs::read_to_string(p).ok())
                            .and_then(|c| serde_json::from_str::<Value>(&c).ok())
                            .and_then(|k| k["highwayapi_base_url"].as_str().map(|s| s.to_string()))
                            .filter(|s| !s.trim().is_empty())
                    })
                    .unwrap_or_else(|| "https://api.highwayapi.ai/openai".to_string());
                let base = configured.trim().trim_end_matches('/').to_string();
                if base.ends_with("/v1/chat/completions") || base.ends_with("/chat/completions") {
                    base
                } else if base.ends_with("/v1") {
                    format!("{}/chat/completions", base)
                } else {
                    format!("{}/v1/chat/completions", base)
                }
            }
            "mistral" => "https://api.mistral.ai/v1/chat/completions".to_string(),
            "groq" => "https://api.groq.com/openai/v1/chat/completions".to_string(),
            "openrouter" => "https://openrouter.ai/api/v1/chat/completions".to_string(),
            "deepseek" => "https://api.deepseek.com/v1/chat/completions".to_string(),
            // Xiaomi MiMo — OpenAI-compatible (Token Plan / coding subscription).
            // env MIMO_BASE_URL → keys.mimo_base_url → default. Third-party use is
            // explicitly permitted (unlike Anthropic's subscription).
            "mimo" | "xiaomi" => {
                let configured = std::env::var("MIMO_BASE_URL").ok()
                    .filter(|s| !s.trim().is_empty())
                    .or_else(|| {
                        self.brain_dir.parent()
                            .map(|p| p.join("api_keys.json"))
                            .and_then(|p| std::fs::read_to_string(p).ok())
                            .and_then(|c| serde_json::from_str::<Value>(&c).ok())
                            .and_then(|k| k["mimo_base_url"].as_str().map(|s| s.to_string()))
                            .filter(|s| !s.trim().is_empty())
                    })
                    .unwrap_or_else(|| "https://api.xiaomimimo.com/v1".to_string());
                let base = configured.trim().trim_end_matches('/').to_string();
                if base.ends_with("/chat/completions") { base }
                else if base.ends_with("/v1") { format!("{}/chat/completions", base) }
                else { format!("{}/v1/chat/completions", base) }
            }
            // Cyber-Ifrit Cloud — our hosted brain (Neural VFS + routing) on the AMD MI300X
            // backend. DYNAMIC config so the production endpoint is never the only hardcoded
            // option (anti-bypass per the open-core strategy): env CYBERIFRIT_BASE_URL →
            // keys.cyberifrit_base_url → default. OpenAI-compatible; auth via subscription JWT.
            "cyberifrit" | "cyber-ifrit" | "cyberifrit-cloud" => {
                let configured = std::env::var("CYBERIFRIT_BASE_URL").ok()
                    .filter(|s| !s.trim().is_empty())
                    .or_else(|| {
                        self.brain_dir.parent()
                            .map(|p| p.join("api_keys.json"))
                            .and_then(|p| std::fs::read_to_string(p).ok())
                            .and_then(|c| serde_json::from_str::<Value>(&c).ok())
                            .and_then(|k| k["cyberifrit_base_url"].as_str().map(|s| s.to_string()))
                            .filter(|s| !s.trim().is_empty())
                    })
                    .unwrap_or_else(|| "https://ai.cyberifrit.xyz".to_string());
                let base = configured.trim().trim_end_matches('/').to_string();
                if base.ends_with("/chat/completions") { base }
                else if base.ends_with("/v1") { format!("{}/chat/completions", base) }
                else { format!("{}/v1/chat/completions", base) }
            }
            "openwebui" | "openwebui-claude" | "openwebui-gpt" | "openwebui-gemini" => "http://127.0.0.1:8080/api/chat/completions".to_string(),

            // Local DeepSeek V2 running on Apple Silicon (M1/M2/M3) via either
            // llama.cpp + Metal or MLX-LM. Both expose an OpenAI-compatible
            // server. Default port is 8080 (llama-server), overridable via
            // the DEEPSEEK_ANE_URL env var or `ollama_url` field on the
            // request (we reuse the field for any local OpenAI-compatible
            // endpoint to avoid threading another override through the API).
            "deepseek-ane" | "deepseek_ane" | "ds2-ane" => {
                let base = std::env::var("DEEPSEEK_ANE_URL")
                    .ok()
                    .or_else(|| req.ollama_url.clone())
                    .unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
                let base = base.trim().trim_end_matches('/').to_string();
                // Accept either bare host (http://127.0.0.1:8080) or fully
                // qualified path. Append /v1/chat/completions if not present.
                if base.ends_with("/v1/chat/completions") {
                    base
                } else if base.ends_with("/v1") {
                    format!("{}/chat/completions", base)
                } else {
                    format!("{}/v1/chat/completions", base)
                }
            }
            "apiradar" => "https://apiradar.live/api/v1/chat/completions".to_string(),
            "xai" => "https://api.x.ai/v1/chat/completions".to_string(),
            "cerebras" => "https://api.cerebras.ai/v1/chat/completions".to_string(),
            "nvidia" => "https://integrate.api.nvidia.com/v1/chat/completions".to_string(),
            "alibaba" => {
                "https://dashscope-us.aliyuncs.com/compatible-mode/v1/chat/completions".to_string()
            }
            "antigravity" => "http://127.0.0.1:1536/v1/chat/completions".to_string(),
            // vLLM — OpenAI-compat server, URL from ollama_url field or env
            "vllm" => {
                let base = std::env::var("VLLM_URL")
                    .ok()
                    .or_else(|| req.ollama_url.clone())
                    .unwrap_or_else(|| "http://127.0.0.1:8000".to_string());
                let base = base.trim().trim_end_matches('/').to_string();
                if base.ends_with("/v1/chat/completions") { base }
                else if base.ends_with("/v1") { format!("{}/chat/completions", base) }
                else { format!("{}/v1/chat/completions", base) }
            }
            // LM Studio — OpenAI-compat server on port 1234 by default
            "lmstudio" | "lm-studio" | "lm_studio" => {
                let base = std::env::var("LMSTUDIO_URL")
                    .ok()
                    .or_else(|| req.ollama_url.clone())
                    .unwrap_or_else(|| "http://127.0.0.1:1234".to_string());
                let base = base.trim().trim_end_matches('/').to_string();
                if base.ends_with("/v1/chat/completions") { base }
                else if base.ends_with("/v1") { format!("{}/chat/completions", base) }
                else { format!("{}/v1/chat/completions", base) }
            }
            // LiteLLM proxy — OpenAI-compat proxy on port 4000 by default
            "litellm" | "lite-llm" | "lite_llm" => {
                let base = std::env::var("LITELLM_URL")
                    .ok()
                    .or_else(|| req.ollama_url.clone())
                    .unwrap_or_else(|| "http://127.0.0.1:4000".to_string());
                let base = base.trim().trim_end_matches('/').to_string();
                if base.ends_with("/v1/chat/completions") { base }
                else if base.ends_with("/v1") { format!("{}/chat/completions", base) }
                else { format!("{}/v1/chat/completions", base) }
            }
            "ollama" => {
                let base = normalize_ollama_base_url(
                    &req.ollama_url
                        .clone()
                        .unwrap_or_else(|| "http://127.0.0.1:11434".to_string()),
                );
                let root = base
                    .trim_end_matches('/')
                    .trim_end_matches("/v1/chat/completions")
                    .trim_end_matches("/chat/completions")
                    .trim_end_matches("/v1")
                    .trim_end_matches("/api/chat");
                let m = req.model.to_lowercase();
                if Self::is_cyberifrit_managed_ollama_url(root)
                    || m.contains("bugtrace")
                    || req.model.contains("hf.co/")
                {
                    format!("{}/v1/chat/completions", root)
                } else {
                    format!("{}/api/chat", root)
                }
            }
            _ => {
                if let Ok(url) = std::env::var("OPENAI_BASE_URL") {
                    if !url.is_empty() { return url; }
                }
                let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");
                if let Ok(content) = std::fs::read_to_string(&keys_path) {
                    if let Ok(keys) = serde_json::from_str::<Value>(&content) {
                        if let Some(custom_url) = keys["openai_base_url"].as_str() {
                            if !custom_url.is_empty() {
                                let base = custom_url.trim().trim_end_matches('/').to_string();
                                if base.ends_with("/v1/chat/completions") { return base; }
                                else if base.ends_with("/v1") { return format!("{}/chat/completions", base); }
                                else { return format!("{}/v1/chat/completions", base); }
                            }
                        }
                    }
                }

                // Cloud goes DIRECT — never auto-route through the local :1536 AIM proxy
                // (redundant context + hangs to the timeout if the proxy is up but stalls).
                "https://api.openai.com/v1/chat/completions".to_string()
            }
        }
    }


    /// Cursor-style zero-grep: block *orientation* recon when AIM already has the tree.
    /// Targeted grep/glob/search remain available — only blind "what's in this repo?" is blocked.
    pub(crate) async fn intercept_zero_grep_orientation(
        &self,
        tool_name: &str,
        args: &Value,
        iteration: usize,
    ) -> Option<Value> {
        let indexed = self.memory_store.get_project_tree().await.len();
        if indexed == 0 {
            return None;
        }

        let summary = self.memory_store.get_project_tree_summary().await;
        let block = |hint: &str| {
            Some(json!({
                "status": "blocked_zero_grep",
                "message": format!(
                    "ZERO-GREP (AIM — {indexed} files). {hint}\n\nTree: {summary}\n\n\
                     Structure is in ### BRAIN. For targeted work you MAY still use grep/glob \
                     with a specific pattern or path — not repo-wide orientation."
                ),
                "indexed_files": indexed,
            }))
        };

        // Full-tree listing — never needed when AIM has the tree (any iteration).
        if tool_name == "list_dir_tree" {
            return block("list_dir_tree is orientation-only — use ### BRAIN for structure.");
        }

        if tool_name == "list_files" || tool_name == "list_directory" {
            let path = args
                .get("path")
                .or_else(|| args.get("directory"))
                .and_then(|v| v.as_str())
                .unwrap_or(".");
            if Self::is_root_orientation_path(path, &self.ai_tools.get_root_path()) {
                return block(&format!(
                    "Root list_files('{path}') blocked — paths are in ### BRAIN. \
                     list_files(subdir) is OK for a specific folder you already identified."
                ));
            }
            return None;
        }

        if tool_name == "glob" || tool_name == "find_by_name" || tool_name == "find" {
            let pattern = args
                .get("pattern")
                .or_else(|| args.get("pattern_or_path"))
                .or_else(|| args.get("glob_pattern"))
                .and_then(|v| v.as_str())
                .unwrap_or("*");
            if iteration <= 1 && Self::is_broad_glob_pattern(pattern) {
                return block(&format!(
                    "Broad glob '{pattern}' blocked on early iteration — use ### BRAIN or a scoped glob like `backend/**/*.rs`."
                ));
            }
            return None;
        }

        if tool_name == "grep" {
            let pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
            if iteration <= 1 && Self::is_orientation_grep_pattern(pattern) {
                return block(&format!(
                    "Orientation grep '{pattern}' blocked — use semantic_search/aim_query_spans first, \
                     or grep a specific symbol you already know (e.g. a function name)."
                ));
            }
            return None;
        }

        if tool_name == "run_command" || tool_name == "bash" {
            if let Some(cmd) = args.get("command").and_then(|v| v.as_str()) {
                if Self::is_shell_recon_command(cmd) {
                    return block(&format!(
                        "Shell recon blocked (`{cmd}`). Tree is in ### BRAIN. \
                         run_command for cargo/npm/git/test is always OK."
                    ));
                }
            }
        }

        None
    }

    pub(crate) fn is_root_orientation_path(path: &str, root: &Path) -> bool {
        let root_norm = root.to_string_lossy().trim_end_matches(['/', '\\']).to_string();
        let path_norm = path.trim().trim_end_matches(['/', '\\']);
        path_norm.is_empty()
            || path_norm == "."
            || path_norm == "./"
            || path_norm == "/"
            || Path::new(path_norm) == root
            || path_norm.eq_ignore_ascii_case(&root_norm)
            || root.ends_with(path_norm)
    }

    /// Repo-wide or extension-only globs used to "discover" a codebase.
    pub(crate) fn is_broad_glob_pattern(pattern: &str) -> bool {
        let p = pattern.trim();
        p.is_empty()
            || p == "*"
            || p == "**"
            || p == "**/*"
            || p == "*.*"
            || p.starts_with("*.") && !p.contains('/') && !p.contains('\\')
            || (p.contains('*') && !p.contains('/') && !p.contains('\\') && p.len() <= 6)
    }

    /// Grep patterns used to map a repo instead of finding one known thing.
    pub(crate) fn is_orientation_grep_pattern(pattern: &str) -> bool {
        let p = pattern.trim();
        if p.is_empty() || p == ".*" || p == "." || p.len() < 3 {
            return true;
        }
        let lower = p.to_ascii_lowercase();
        const ORIENT: &[&str] = &[
            "todo", "fixme", "hack", "xxx", "import ", "require(", "require ",
            "from ", "class ", "function ", "def ", "fn ", "struct ", "interface ",
            "export ", "module.exports", "@ts-ignore", "eslint-disable",
        ];
        ORIENT.iter().any(|o| lower.contains(o))
    }

    /// True when a shell command is filesystem orientation (ls/find/tree), not build/test/git.
    pub(crate) fn is_shell_recon_command(cmd: &str) -> bool {
        let c = cmd.trim().to_lowercase();
        if c.is_empty() {
            return false;
        }
        // Build / test / VCS — always allowed
        const ALLOW: &[&str] = &[
            "cargo ", "cargo\n", "npm ", "pnpm ", "yarn ", "npx ", "node --",
            "git ", "pytest", "python -m", "python3 -m", "dotnet ", "go test",
            "go build", "make ", "vitest", "jest ", "tsc", "eslint", "prettier",
            "webpack", "vite ", "flutter ", "gradle ", "mvn ", "composer ",
        ];
        if ALLOW.iter().any(|a| c.contains(a)) {
            return false;
        }
        const RECON: &[&str] = &[
            " ls", "ls ", "ls\n", "ls\r", "\nls", "&& ls", "; ls", "| ls",
            " dir", "dir ", "dir\n", "dir /", "dir/w",
            "tree ", "tree\n", " find ", "find ", " fd ", "fdfind ",
            "get-childitem", "gci ", "get-childitem",
            "rg --files", "rg -l ''", "glob ", "pwd", "where.exe ",
            "select-string", "sls ",
        ];
        if c == "ls" || c == "dir" || c == "pwd" || c == "tree" {
            return true;
        }
        // `dir path\to\file` or `ls README.md` — existence check, not tree recon.
        if c.starts_with("dir ") || c.starts_with("dir\t") {
            let rest = cmd.trim()[3..].trim();
            if !rest.is_empty()
                && !rest.starts_with('/')
                && !rest.starts_with('-')
                && !rest.eq_ignore_ascii_case("/s")
                && !rest.eq_ignore_ascii_case("/w")
            {
                return false;
            }
        }
        if c.starts_with("ls ") || c.starts_with("ls\t") {
            let rest = cmd.trim()[2..].trim();
            if !rest.is_empty() && !rest.starts_with('-') {
                return false;
            }
        }
        RECON.iter().any(|r| c.contains(r))
            || (c.starts_with("cd ") && (c.contains("&& ls") || c.contains("; ls") || c.contains("dir")))
    }

    /// Intercept `run_command` shell file-write patterns and convert to `write_to_file` args.
    /// Returns Some(args) if the command looks like a file write, None otherwise.
    pub(crate) fn try_intercept_file_write(cmd: &str) -> Option<Value> {
        let cmd = cmd.trim();

        // Pattern: echo "content" > file.ext  or  echo 'content' > file.ext
        // Also handles: printf "content" > file.ext
        let write_re_simple = ["echo ", "printf "];
        for prefix in &write_re_simple {
            if cmd.starts_with(prefix) {
                // Find the > redirect
                if let Some(arrow_pos) = cmd.rfind(" > ") {
                    let file_path = cmd[arrow_pos + 3..].trim().trim_matches('"').trim_matches('\'');
                    if Self::looks_like_file_path(file_path) {
                        // Extract the content between quotes after the prefix
                        let after_prefix = cmd[prefix.len()..arrow_pos].trim();
                        let content = after_prefix.trim_matches('"').trim_matches('\'')
                            .replace("\\n", "\n")
                            .replace("\\t", "\t");
                        return Some(json!({ "path": file_path, "content": content }));
                    }
                }
            }
        }

        // Pattern: cat > file.ext << 'EOF' ... EOF  (heredoc — too complex, skip)
        // Pattern: PowerShell Set-Content / Out-File
        if cmd.contains("Set-Content") || cmd.contains("Out-File") {
            // Extract -Path and -Value/-InputObject
            let path = Self::extract_ps_param(cmd, &["-Path", "-FilePath", "-LiteralPath"]);
            let value = Self::extract_ps_param(cmd, &["-Value", "-InputObject"]);
            if let (Some(p), Some(v)) = (path, value) {
                if Self::looks_like_file_path(&p) {
                    return Some(json!({ "path": p, "content": v }));
                }
            }
        }

        None
    }

    pub(crate) fn looks_like_file_path(s: &str) -> bool {
        !s.is_empty()
            && s.contains('.')
            && !s.contains(' ')
            && !s.starts_with("http")
            && s.len() < 250
    }

    pub(crate) fn extract_ps_param(cmd: &str, param_names: &[&str]) -> Option<String> {
        for name in param_names {
            if let Some(pos) = cmd.find(name) {
                let after = cmd[pos + name.len()..].trim_start();
                // Could be -Path "value" or -Path value
                if after.starts_with('"') {
                    if let Some(end) = after[1..].find('"') {
                        return Some(after[1..1 + end].to_string());
                    }
                } else if after.starts_with('\'') {
                    if let Some(end) = after[1..].find('\'') {
                        return Some(after[1..1 + end].to_string());
                    }
                } else {
                    let end = after.find(|c: char| c.is_whitespace()).unwrap_or(after.len());
                    return Some(after[..end].to_string());
                }
            }
        }
        None
    }

    /// Scan AI text response for code blocks annotated with file paths and return
    /// write_to_file tool calls for them. This is the "Cursor Apply" behavior —
    /// when the model shows code in its text but doesn't call a write tool, we catch it.
    pub(crate) fn try_extract_file_writes_from_text(&self, content: &str) -> Vec<ToolCall> {
        let mut writes = Vec::new();
        let mut search_pos = 0;

        while let Some(backtick_pos) = content[search_pos..].find("```") {
            let block_start = search_pos + backtick_pos;
            let after_backticks = block_start + 3;

            // Get the header line (e.g. "python src/main.py" or "rust src/lib.rs")
            let header_end = content[after_backticks..]
                .find('\n')
                .map(|p| after_backticks + p)
                .unwrap_or(content.len());
            let header = content[after_backticks..header_end].trim().to_string();

            let content_start = if header_end < content.len() { header_end + 1 } else { content.len() };
            let rest = &content[content_start..];

            if let Some(close_pos) = rest.find("```") {
                let block_body = rest[..close_pos].trim();

                if !block_body.is_empty() {
                    // Try existing patterns first (header-based)
                    if let Some(file_path) = Self::extract_file_path_from_block_header(&header, block_body) {
                        let id = format!("auto-write-{}", writes.len());
                        writes.push(ToolCall {
                            id,
                            type_field: "function".to_string(),
                            context: None,
                            function: ToolFunction {
                                name: "write_to_file".to_string(),
                                arguments: json!({
                                    "path": file_path,
                                    "content": block_body
                                }).to_string(),
                            },
                        });
                    } else {
                        // Pattern 4: Infer file path from surrounding text context
                        let preceding_text = &content[..block_start];
                        let lang = header.split_whitespace().next().unwrap_or("");
                        if let Some(file_path) = Self::infer_file_path_from_context(preceding_text, block_body, lang) {
                            let id = format!("auto-write-{}", writes.len());
                            writes.push(ToolCall {
                                id,
                                type_field: "function".to_string(),
                                context: None,
                                function: ToolFunction {
                                    name: "write_to_file".to_string(),
                                    arguments: json!({
                                        "path": file_path,
                                        "content": block_body
                                    }).to_string(),
                                },
                            });
                        }
                    }
                }
                search_pos = content_start + close_pos + 3;
            } else {
                break;
            }
        }

        writes
    }

    pub(crate) fn extract_file_path_from_block_header(header: &str, body: &str) -> Option<String> {
        // Pattern 1: ```rust src/lib.rs  or  ```python main.py  (lang + space + path)
        let parts: Vec<&str> = header.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let path = parts[1].trim();
            if Self::looks_like_file_path(path) && path.contains('/') || path.contains('\\') {
                return Some(path.to_string());
            }
        }

        // Pattern 2: header IS a file path (no language prefix, e.g. ```src/main.py)
        if parts.len() == 1 && Self::looks_like_file_path(header) && (header.contains('/') || header.contains('\\') || header.contains('.')) {
            // Make sure it's not just a language name like "python" or "rust"
            let lang_names = ["python", "rust", "javascript", "typescript", "go", "java", "cpp", "c", "html", "css", "json", "yaml", "toml", "bash", "sh", "powershell", "sql"];
            if !lang_names.contains(&header.to_lowercase().as_str()) {
                return Some(header.to_string());
            }
        }

        // Pattern 3: First line of body is a file path comment
        // // file: path.rs   # file: path.py   # path: path.py   /* file: path.js */
        if let Some(first_line) = body.lines().next() {
            let comment_prefixes = [
                "// file:", "# file:", "// filename:", "# filename:",
                "// path:", "# path:", "/* file:", "-- file:", "<!-- file:",
            ];
            for prefix in &comment_prefixes {
                let lower = first_line.to_lowercase();
                if let Some(idx) = lower.find(prefix) {
                    let path = first_line[idx + prefix.len()..].trim()
                        .trim_end_matches("*/").trim_end_matches("-->").trim();
                    if Self::looks_like_file_path(path) {
                        return Some(path.to_string());
                    }
                }
            }
        }

        None
    }

    /// Pattern 4: Detect file path from surrounding text (e.g., "build calculator.py")
    /// Called by try_extract_file_writes_from_text with the text BEFORE the code block.
    pub(crate) fn infer_file_path_from_context(preceding_text: &str, block_body: &str, lang: &str) -> Option<String> {
        // Look for patterns like "create/write/build/save X.py" or "X.py that..."
        let patterns = [
            r"(?:create|write|build|save|implement|add)\s+(?:a\s+|an\s+|the\s+)?([a-zA-Z0-9_./-]+\.[a-zA-Z]{1,5})\b",
            r"([a-zA-Z0-9_./-]+\.(?:py|js|ts|tsx|jsx|rs|go|java|cpp|c|h|hpp|rb|php|sh|bash|css|html|json|yaml|yml|toml|sql|md))\s+(?:that|which|with|containing|implementing|wrapping)",
        ];

        let lang_to_ext: std::collections::HashMap<&str, &str> = [
            ("python", ".py"), ("javascript", ".js"), ("typescript", ".ts"),
            ("rust", ".rs"), ("go", ".go"), ("java", ".java"), ("cpp", ".cpp"),
            ("c", ".c"), ("ruby", ".rb"), ("php", ".php"), ("bash", ".sh"),
            ("html", ".html"), ("css", ".css"), ("json", ".json"),
        ].iter().cloned().collect();

        for pattern in &patterns {
            if let Some(caps) = regex::Regex::new(pattern).ok()?.captures(preceding_text) {
                if let Some(m) = caps.get(1) {
                    let path = m.as_str().to_string();
                    if Self::looks_like_file_path(&path) {
                        return Some(path);
                    }
                }
            }
        }

        // Fallback: if lang is known, infer filename from code content
        if let Some(ext) = lang_to_ext.get(lang.to_lowercase().as_str()) {
            // Look for class/function names that could be the filename
            if let Some(first_line) = block_body.lines().next() {
                // class Calculator → calculator.py
                if let Some(caps) = regex::Regex::new(r"class\s+(\w+)").ok()?.captures(first_line) {
                    if let Some(name) = caps.get(1) {
                        let snake = name.as_str().chars().fold(String::new(), |mut acc, c| {
                            if c.is_uppercase() && !acc.is_empty() { acc.push('_'); }
                            acc.push(c.to_lowercase().next().unwrap_or(c));
                            acc
                        });
                        return Some(format!("{}{}", snake, ext));
                    }
                }
                // def calculate → calculate.py
                if let Some(caps) = regex::Regex::new(r"(?:pub\s+)?(?:async\s+)?fn\s+(\w+)|def\s+(\w+)").ok()?.captures(first_line) {
                    if let Some(name) = caps.get(1).or_else(|| caps.get(2)) {
                        return Some(format!("{}{}", name.as_str(), ext));
                    }
                }
            }
        }

        None
    }

    pub(crate) fn try_parse_markdown_tool_calls(&self, content: &str) -> Vec<ToolCall> {
        let mut tools = Vec::new();

        // 0. XML-style tool calls: <tool_call>{"name":...}</tool_call> or <function>...</function>
        //    Common in Qwen, DeepSeek, and some fine-tuned Llama models.
        for tag in &["tool_call", "function_call", "function", "invoke"] {
            let open = format!("<{}>", tag);
            let close = format!("</{}>", tag);
            let mut pos = 0;
            while let Some(start) = content[pos..].find(&open) {
                let abs_start = pos + start + open.len();
                if let Some(end) = content[abs_start..].find(&close) {
                    let block = content[abs_start..abs_start + end].trim();
                    self.parse_json_to_tools(block, &mut tools);
                    pos = abs_start + end + close.len();
                } else {
                    break;
                }
            }
        }
        if !tools.is_empty() { return tools; }

        // 1. Aggressive Markdown code block parsing
        let mut search_pos = 0;
        while let Some(start) = content[search_pos..].find("```") {
            let actual_start = search_pos + start;
            let after_backticks = actual_start + 3;
            
            let possible_json_start = if let Some(newline_pos) = content[after_backticks..].find('\n') {
                let identifier = content[after_backticks..after_backticks + newline_pos].trim();
                // Strip common identifiers if they exist or if the content starts with { right after identifying text
                if identifier == "json" || identifier == "rust" || identifier == "javascript" || identifier == "typescript" {
                    after_backticks + newline_pos + 1
                } else {
                    // Check if content on same line starts with {
                    if identifier.starts_with('{') {
                        after_backticks
                    } else {
                         after_backticks + newline_pos + 1
                    }
                }
            } else {
                after_backticks
            };

            let rest = if possible_json_start < content.len() { &content[possible_json_start..] } else { "" };
            
            if let Some(end) = rest.find("```") {
                let json_block = rest[..end].trim();
                search_pos = possible_json_start + end + 3;
                self.parse_json_to_tools(json_block, &mut tools);
            } else {
                // Unclosed block - try parsing rest of content
                let json_block = rest.trim();
                self.parse_json_to_tools(json_block, &mut tools);
                break;
            }
        }
    
        // 2. DEEP FALLBACK: If nothing found, or to catch stray JSON outside blocks,
        // use a recursive bracket matcher to find any legitimate { "name": ... } objects.
        if tools.is_empty() {
             self.extract_json_objects_robustly(content, &mut tools);
        }

        tools
    }

    /// Recursively find potential JSON objects in text that look like tool calls.
    pub(crate) fn extract_json_objects_robustly(&self, content: &str, tools: &mut Vec<ToolCall>) {
        let mut pos = 0;
        while let Some(start_idx) = content[pos..].find('{') {
            let actual_start = pos + start_idx;
            let mut depth = 0;
            let mut end_idx = None;
            let mut in_string = false;
            let mut escaped = false;

            for (i, c) in content[actual_start..].char_indices() {
                if escaped {
                    escaped = false;
                    continue;
                }
                if c == '\\' {
                    escaped = true;
                    continue;
                }
                if c == '"' {
                    in_string = !in_string;
                    continue;
                }
                if !in_string {
                    if c == '{' {
                        depth += 1;
                    } else if c == '}' {
                        depth -= 1;
                        if depth == 0 {
                            end_idx = Some(actual_start + i + 1);
                            break;
                        }
                    }
                }
            }

            if let Some(end) = end_idx {
                let candidate = &content[actual_start..end];
                if let Ok(val) = serde_json::from_str::<Value>(candidate) {
                    let old_len = tools.len();
                    self.parse_single_json_item_to_tools(val, tools);
                    if tools.len() > old_len {
                         // Successfully found a tool, advance past it
                         pos = end;
                         continue;
                    }
                }
            }
        }
    }

    pub(crate) fn parse_json_to_tools(&self, json_block: &str, tools: &mut Vec<ToolCall>) {
        // Try parsing the full block first (valid if it's one object or an array)
        if let Ok(val) = serde_json::from_str::<Value>(json_block) {
            self.parse_single_json_item_to_tools(val, tools);
        } else {
            // Try splitting by newline for NDJSON inside the block
            for line in json_block.lines() {
                let line = line.trim();
                if !line.is_empty() {
                    if let Ok(val) = serde_json::from_str::<Value>(line) {
                        self.parse_single_json_item_to_tools(val, tools);
                    }
                }
            }
        }
    }

    pub(crate) fn parse_single_json_item_to_tools(&self, val: Value, tools: &mut Vec<ToolCall>) {
        let items = if val.is_array() {
            val.as_array().unwrap().clone()
        } else {
            vec![val]
        };

        for item in items {
            let name = item
                .get("name")
                .or_else(|| item.get("tool"))
                .or_else(|| item.get("call"))
                .or_else(|| item.get("method"))
                .or_else(|| item.get("function").and_then(|f| f.get("name")))
                .and_then(|v| v.as_str());

            let arguments = item
                .get("arguments")
                .or_else(|| item.get("args"))
                .or_else(|| item.get("params"))
                .or_else(|| item.get("parameters"))
                .or_else(|| item.get("inputs"))
                .or_else(|| item.get("function").and_then(|f| f.get("arguments")));

            if let Some(name) = name {
                let args_str = match arguments {
                    Some(Value::String(s)) => s.clone(),
                    Some(obj) => obj.to_string(),
                    None => "{}".to_string(),
                };

                tools.push(ToolCall {
                    id: format!("call_{}", uuid::Uuid::new_v4()),
                    type_field: "function".to_string(),
                    function: ToolFunction {
                        name: name.to_string(),
                        arguments: args_str,
                    },
                    context: None,
                });
            }
        }
    }

    pub async fn summarize_builtin_tools(&self) -> String {
        let mut summary = String::new();
        let tools = self.ai_tools.list_tools();
        for tool in tools {
            summary.push_str(&format!(
                "- `{}`: {}\n  JSON Schema: {}\n",
                tool.name,
                tool.description,
                serde_json::to_string(&tool.input_schema).unwrap_or_else(|_| "{}".to_string())
            ));
        }
        summary
    }

    pub async fn summarize_mcp_tools(&self) -> String {
        let mut summary = String::new();
        if let Ok(mcp_tools) = self.mcp_registry.list_tools().await {
            if !mcp_tools.is_empty() {
                summary.push_str("\n\n### REGISTERED MCP TOOLS:\n");
                for tool in mcp_tools {
                    summary.push_str(&format!(
                        "- `{}`: {}\n",
                        tool["name"].as_str().unwrap_or("unknown"),
                        tool["description"].as_str().unwrap_or("No description")
                    ));
                }
                summary
                    .push_str("\nYou can invoke these MCP tools using your native tool calling capabilities. Do NOT output raw JSON blocks to invoke them.");
            }
        }
        summary
    }

}
