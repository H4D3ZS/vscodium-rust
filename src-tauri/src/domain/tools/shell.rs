//! ShellTranslator: cross-platform command/path translation for the run tool.

pub struct ShellTranslator;

impl ShellTranslator {
    pub fn find_sh_path() -> Option<String> {
        crate::ide_shell::resolve_sh_exe()
            .or_else(|| crate::ide_shell::resolve_git_bash_exe())
            .map(|p| p.to_string_lossy().to_string())
            .or_else(|| {
                if cfg!(target_os = "windows") {
                    let common_paths = [
                        "C:\\Program Files\\Git\\bin\\sh.exe",
                        "C:\\Program Files (x86)\\Git\\bin\\sh.exe",
                        "C:\\Program Files\\Git\\usr\\bin\\sh.exe",
                    ];
                    for path in &common_paths {
                        if std::path::Path::new(path).exists() {
                            return Some(path.to_string());
                        }
                    }
                    which::which("sh").ok().map(|p| p.to_string_lossy().to_string())
                } else {
                    None
                }
            })
    }

    pub fn translate_command(command: &str, shell_hint: &str) -> (String, Vec<String>) {
        let mut final_command = command.to_string();
        let shell_hint = Self::effective_shell_hint(command, shell_hint);
        
        if cfg!(target_os = "windows") {
            match shell_hint {
                "bash" | "sh" => {
                    final_command = Self::normalize_windows_paths_for_bash(&final_command);
                    final_command = Self::translate_unix_commands_for_bash(&final_command);
                    // Only use SH if it actually exists, otherwise fallback to native
                    if let Some(sh_path) = Self::find_sh_path() {
                        return (sh_path, vec!["-c".to_string(), final_command]);
                    }
                    ("powershell".to_string(), vec!["-Command".to_string(), final_command])
                }
                "cmd" => {
                    ("cmd".to_string(), vec!["/c".to_string(), final_command])
                }
                _ => {
                    // NATIVE WINDOWS DEFAULT: Use PowerShell and map common unix-isms to windows
                    if final_command.starts_with("ls ") || final_command == "ls" {
                         final_command = final_command.replace("ls ", "dir /b ").replace("ls", "dir /b");
                    } else if final_command.starts_with("cat ") {
                         final_command = final_command.replace("cat ", "type ");
                    } else if final_command.starts_with("pwd") {
                         final_command = final_command.replace("pwd", "echo %cd%");
                    } else if final_command.starts_with("rm -rf ") {
                         final_command = final_command.replace("rm -rf ", "rmdir /s /q ");
                    } else if final_command.starts_with("rm ") {
                         final_command = final_command.replace("rm ", "del /f /q ");
                    } else if final_command.starts_with("cp ") {
                         final_command = final_command.replace("cp ", "copy ");
                    } else if final_command.starts_with("mv ") {
                         final_command = final_command.replace("mv ", "move ");
                    }
                    
                    ("powershell".to_string(), vec!["-Command".to_string(), final_command])
                }
            }
        } else {
            // Linux/macOS
            ("sh".to_string(), vec!["-c".to_string(), final_command])
        }
    }

    /// Git Bash treats `\` in `C:\Users\...` as escapes. Convert drive paths to
    /// `/c/Users/...` before `sh -c`. **Never** blanket-replace `\` → `/` — that
    /// destroys grep regex, Python `\n`, and JSON escapes in one-liners.
    pub fn normalize_windows_paths_for_bash(command: &str) -> String {
        if !cfg!(windows) {
            return command.to_string();
        }
        if command.contains("python -c") || command.contains("python3 -c") {
            return Self::normalize_python_command_paths(command);
        }
        let chars: Vec<char> = command.chars().collect();
        let mut out = String::with_capacity(command.len());
        let mut i = 0;
        while i < chars.len() {
            if Self::is_windows_drive_at(&chars, i) {
                out.push('/');
                out.push(chars[i].to_ascii_lowercase());
                out.push('/');
                i += 2;
                if i < chars.len() && (chars[i] == '\\' || chars[i] == '/') {
                    i += 1;
                }
                continue;
            }
            out.push(chars[i]);
            i += 1;
        }
        use regex::Regex;
        let path_token = Regex::new("/[a-z]/[^\\s\"']+").expect("path token");
        path_token
            .replace_all(&out, |caps: &regex::Captures| caps[0].replace('\\', "/"))
            .into_owned()
    }

    fn is_windows_drive_at(chars: &[char], i: usize) -> bool {
        if i + 1 >= chars.len() {
            return false;
        }
        if !chars[i].is_ascii_alphabetic() || chars[i + 1] != ':' {
            return false;
        }
        // Skip URL schemes (http://, https://, ftp://, …).
        if i + 2 < chars.len() && chars[i + 2] == '/' {
            if i + 3 < chars.len() && chars[i + 3] == '/' {
                return false;
            }
            if i > 0 && chars[i - 1].is_ascii_alphabetic() {
                return false;
            }
        }
        if i > 0 {
            let prev = chars[i - 1];
            if !(prev.is_whitespace()
                || prev == '"'
                || prev == '\''
                || prev == '('
                || prev == '['
                || prev == '=')
            {
                return false;
            }
        }
        true
    }

    /// `C:\Users\foo` → `C:/Users/foo` inside `python -c` strings (Windows-openable).
    fn normalize_python_command_paths(command: &str) -> String {
        use regex::Regex;
        let re = Regex::new(r#"([A-Za-z]):\\([^\s"']+)""#).expect("python path");
        re.replace_all(command, |caps: &regex::Captures| {
            format!(
                "{}:/{}",
                &caps[1],
                caps[2].replace('\\', "/")
            )
        })
        .into_owned()
    }

    /// Strip malformed proxy env vars that make curl resolve host `http` (exit 6).
    pub fn sanitize_proxy_env(cmd: &mut std::process::Command) {
        const KEYS: &[&str] = &[
            "HTTP_PROXY", "HTTPS_PROXY", "ALL_PROXY", "http_proxy", "https_proxy", "all_proxy",
        ];
        for key in KEYS {
            if let Ok(v) = std::env::var(key) {
                let t = v.trim();
                if t.is_empty() {
                    continue;
                }
                let bad = t.eq_ignore_ascii_case("http")
                    || t.eq_ignore_ascii_case("https")
                    || (!t.contains("://")
                        && !t.starts_with("socks")
                        && !t.contains('.')
                        && !t.contains(':'));
                if bad {
                    cmd.env(key, "");
                }
            }
        }
    }

    pub(crate) fn extract_curl_url(command: &str) -> Option<String> {
        use regex::Regex;
        let re = Regex::new(r#"https?://[^\s"'<>]+"#).ok()?;
        re.find(command).map(|m| m.as_str().trim_end_matches('\\').to_string())
    }

    /// Heuristic: route through Git Bash when the command uses POSIX tooling.
    pub fn prefers_git_bash(command: &str) -> bool {
        let c = command.to_lowercase();
        const MARKERS: &[&str] = &[
            "curl ", "grep ", "rg ", "head ", "tail ", "sed ", "awk ", "sort ", "uniq ",
            "wc ", "find ", "chmod ", "export ", "source ", "&&", "||", "| ", "python -c",
            "pip install", "npm run", "cargo ", "./", "sh ", "bash ", "index_bundle",
        ];
        MARKERS.iter().any(|m| c.contains(m))
    }

    fn effective_shell_hint<'a>(command: &'a str, shell_hint: &'a str) -> &'a str {
        if !cfg!(windows) {
            return shell_hint;
        }
        if shell_hint == "bash" || shell_hint == "sh" {
            return shell_hint;
        }
        if shell_hint == "run_command"
            && Self::find_sh_path().is_some()
            && Self::prefers_git_bash(command)
        {
            return "bash";
        }
        shell_hint
    }

    /// Map cmd.exe habits to POSIX when we know we're in bash/sh.
    fn translate_unix_commands_for_bash(command: &str) -> String {
        let mut c = command.to_string();
        if c.trim() == "dir /b" || c.trim() == "dir" {
            return if c.contains("/b") { "ls -1".to_string() } else { "ls -la".to_string() };
        }
        if c.contains("dir /b") {
            c = c.replace("dir /b", "ls -1");
        }
        if c.starts_with("type ") {
            c = c.replacen("type ", "cat ", 1);
        }
        c
    }
}

