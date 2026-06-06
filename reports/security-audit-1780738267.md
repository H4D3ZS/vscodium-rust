# Security Audit Report

- **Scope:** `src-tauri/src`
- **Depth:** deep
- **Files scanned:** 137
- **Total findings:** 74

## Summary by severity

| Severity | Count |
|----------|------:|
| HIGH | 21 |
| MEDIUM | 7 |
| LOW | 16 |
| INFO | 30 |

## Dependency posture

- `package.json` present — Run `npm audit` / `pnpm audit` to check npm dependencies for known CVEs.

## Findings

### SEC-002 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_auth.rs:170`
- **Evidence:** `win.eval(&js).map_err(|e| e.to_string())?;`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-035 — Unsafe Deserialization [Unsafe Deserialization]
- **Severity:** HIGH  ·  **CWE:** CWE-502
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7499`
- **Evidence:** `("Unsafe Deserialization", "HIGH", "CWE-502", r"(?i)(pickle\.loads|cPickle\.loads|Marshal\.load|unserialize\s*\(|ObjectInputStream)", "Use safe loaders / allow-`
- **Remediation:** Use safe loaders / allow-lists / signed payloads.

### SEC-036 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7537`
- **Evidence:** `assert!(category_matches("Command Injection", "os.system(user_input)"));`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-037 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7538`
- **Evidence:** `assert!(category_matches("Command Injection", "child_process.exec(cmd)"));`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-038 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7539`
- **Evidence:** `assert!(category_matches("SQL Injection", r#"db.execute("SELECT * FROM users WHERE id=" + id)"#));`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-040 — Unsafe Deserialization [Unsafe Deserialization]
- **Severity:** HIGH  ·  **CWE:** CWE-502
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7541`
- **Evidence:** `assert!(category_matches("Unsafe Deserialization", "data = pickle.loads(blob)"));`
- **Remediation:** Use safe loaders / allow-lists / signed payloads.

### SEC-043 — Disabled TLS Verification [Disabled TLS Verification]
- **Severity:** HIGH  ·  **CWE:** CWE-295
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7544`
- **Evidence:** `assert!(category_matches("Disabled TLS", "requests.get(url, verify=False)"));`
- **Remediation:** Never disable certificate validation.

### SEC-044 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7545`
- **Evidence:** `assert!(category_matches("Dangerous Eval", "eval(user_code)"));`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-050 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\auth_commands.rs:128`
- **Evidence:** `let _ = window_for_eval.eval(&observer_script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-051 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\auth_commands.rs:129`
- **Evidence:** `let _ = window_for_eval.eval(&script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-052 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\auth_commands.rs:953`
- **Evidence:** `let _ = window_for_eval.eval(&observer_script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-053 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\auth_commands.rs:954`
- **Evidence:** `let _ = window_for_eval.eval(&script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-054 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\browser_actuation\chatgpt_bridge.rs:57`
- **Evidence:** `let _ = window_clone.eval(script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-055 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\browser_actuation\chatgpt_bridge.rs:87`
- **Evidence:** `let _ = window.eval(script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-057 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\browser_actuation\claude_bridge.rs:57`
- **Evidence:** `let _ = window_clone.eval(script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-058 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\browser_actuation\claude_bridge.rs:87`
- **Evidence:** `let _ = window.eval(script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-060 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\browser_actuation\gemini_bridge.rs:57`
- **Evidence:** `let _ = window_clone.eval(script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-061 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\browser_actuation\gemini_bridge.rs:87`
- **Evidence:** `let _ = window.eval(script);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-064 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\git_commands.rs:239`
- **Evidence:** `.map_err(|e| format!("Failed to delete checkpoint: {}", e))`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-072 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\security_generators.rs:80`
- **Evidence:** `Socket s=new Socket("{h}",{port}); Process p=Runtime.getRuntime().exec("{sh}");`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-074 — Disabled TLS Verification [Disabled TLS Verification]
- **Severity:** HIGH  ·  **CWE:** CWE-295
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\web_commands.rs:52`
- **Evidence:** `.danger_accept_invalid_certs(true)`
- **Remediation:** Never disable certificate validation.

### SEC-001 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\auth.rs:29`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-034 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7498`
- **Evidence:** `("Cross-Site Scripting", "MEDIUM", "CWE-79", r"(?i)(innerHTML\s*=|dangerouslySetInnerHTML|document\.write\s*\()", "Escape/encode output; prefer textContent or a`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-039 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7540`
- **Evidence:** `assert!(category_matches("Cross-Site Scripting", "el.innerHTML = userdata"));`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-041 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7542`
- **Evidence:** `assert!(category_matches("Weak Cryptographic Hash", "let h = md5(password);"));`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-042 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7543`
- **Evidence:** `assert!(category_matches("Insecure Randomness", "const token = Math.random()"));`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-056 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\browser_actuation\claude_bridge.rs:50`
- **Evidence:** `promptBox.innerHTML = event.data.content;`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-059 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\browser_actuation\gemini_bridge.rs:50`
- **Evidence:** `promptBox.innerHTML = event.data.content;`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-011 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:685`
- **Evidence:** `unsafe { std::env::set_var("AIRI_YOLO_MODE", if enabled { "1" } else { "0" }); }`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-045 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:7546`
- **Evidence:** `assert!(category_matches("unsafe block", "unsafe { *ptr }"));`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-046 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ane.rs:73`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-047 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ane.rs:132`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-048 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ane.rs:232`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-049 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\architecture\infrastructure\performance\sysinfo_process_memory_repository.rs:47`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-062 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\emulator_stream.rs:72`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-063 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\emulator_stream.rs:89`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-065 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\lib.rs:152`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-066 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\lib.rs:174`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-067 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\lib.rs:186`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-068 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\lib.rs:206`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-069 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\main.rs:24`
- **Evidence:** `unsafe { std::env::set_var(key, items.join(",")) };`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-070 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\performance_commands.rs:65`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-071 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\security_distiller.rs:12`
- **Evidence:** `if content.contains("unsafe {") {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-073 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\vision_bridge.rs:26`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-003 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_auth.rs:262`
- **Evidence:** `assert!(v["supports_api_key"].as_bool().unwrap());`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-004 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_auth.rs:263`
- **Evidence:** `assert!(v["supports_webview"].as_bool().unwrap());`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-005 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_auth.rs:282`
- **Evidence:** `let serialized = serde_json::to_string(&keys).unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-006 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_auth.rs:283`
- **Evidence:** `let deserialized: ApiKeys = serde_json::from_str(&serialized).unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-007 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_auth.rs:324`
- **Evidence:** `.expect("IDE Ollama HTTP request failed — is `ollama serve` running?");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-008 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_auth.rs:334`
- **Evidence:** `.expect("Failed to parse Ollama JSON response");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-009 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_auth.rs:340`
- **Evidence:** `.expect("Could not find /message/content in IDE Ollama response");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-010 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:537`
- **Evidence:** `.expect("ollama_http_sem not closed")`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-012 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:763`
- **Evidence:** `let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-013 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:2992`
- **Evidence:** `let h_lock = self.app_handle.read().unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-014 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:3762`
- **Evidence:** `let last_msg = messages.last_mut().unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-015 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:3770`
- **Evidence:** `let last_msg = messages.last_mut().unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-016 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:4315`
- **Evidence:** `.store_message(messages.last().unwrap())`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-017 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:4952`
- **Evidence:** `let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-018 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:5124`
- **Evidence:** `let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-019 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:5747`
- **Evidence:** `let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-020 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:5792`
- **Evidence:** `let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-021 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:5817`
- **Evidence:** `let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-022 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:5981`
- **Evidence:** `let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-023 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:6303`
- **Evidence:** `val.as_array().unwrap().clone()`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-024 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:6450`
- **Evidence:** `let key = obj.keys().next().unwrap().clone();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-025 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_engine.rs:6453`
- **Evidence:** `let value = obj.get(&key).unwrap().clone();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-026 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:2915`
- **Evidence:** `.unwrap()`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-027 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:3148`
- **Evidence:** `.expect("tokio rt for subagent");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-028 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:3235`
- **Evidence:** `let key = google_key.unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-029 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:3281`
- **Evidence:** `crate::image_gen::analyze_with_gemini(google_key.unwrap(), &full, question)`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-030 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:3321`
- **Evidence:** `.unwrap()`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-031 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:3323`
- **Evidence:** `&& !glob::Pattern::new(pattern).unwrap().matches(file_name)`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-032 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:3366`
- **Evidence:** `let re_rust = regex::Regex::new(r"use\s+([^;]+);").unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-033 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\ai_tools.rs:3367`
- **Evidence:** `let re_ts = regex::Regex::new(r#"import.*from\s+['"]([^'"]+)['"]"#).unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

