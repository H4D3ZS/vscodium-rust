pub mod analysis;
pub mod models;
pub mod reporting;
pub mod validation;

use anyhow::{anyhow, Result};
use models::{AppMetadata, GateStatus, Platform, ScanReport, Severity};
use rayon::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

pub struct MobHuntEngine;

struct CodeScanChunk {
    findings: Vec<models::Finding>,
    endpoints: Vec<String>,
    secrets: Vec<models::Finding>,
    sdk_names: Vec<String>,
}

impl MobHuntEngine {
    /// Run full static security audit on an extracted application bundle directory with parallelized file inspection
    pub fn scan_bundle_dir(bundle_dir: &Path) -> Result<ScanReport> {
        let start = Instant::now();
        let mut app_meta = AppMetadata::default();
        let mut findings = Vec::new();
        let mut all_endpoints = Vec::new();
        let mut all_secrets = Vec::new();

        let mut detected_platform = Platform::CrossPlatform;

        // Fast platform detection & file categorization pass
        let mut code_paths: Vec<PathBuf> = Vec::with_capacity(1024);
        let mut special_paths: Vec<PathBuf> = Vec::with_capacity(64);

        for entry in WalkDir::new(bundle_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let file_name_lower = file_name.to_ascii_lowercase();

            // Detect On-Device AI / LLM models
            if file_name_lower.ends_with(".mlmodel")
                || file_name_lower.ends_with(".mlmodelc")
                || file_name_lower.ends_with(".onnx")
                || file_name_lower.ends_with(".tflite")
                || file_name_lower.ends_with(".gguf")
            {
                app_meta.ai_models_found.push(file_name.to_string());
            }

            if file_name == "Info.plist" {
                detected_platform = Platform::Ios;
                special_paths.push(path.to_path_buf());
            } else if file_name == "AndroidManifest.xml" {
                detected_platform = Platform::Android;
                special_paths.push(path.to_path_buf());
            } else if file_name_lower == "network_security_config.xml"
                || file_name_lower == "libflutter.so"
                || file_name == "Flutter"
                || file_name_lower == "libapp.so"
                || file_name == "App"
                || file_name_lower.ends_with(".p12")
                || file_name_lower.ends_with(".pfx")
                || file_name_lower.contains("entitlements")
                || file_name_lower.ends_with(".xcent")
                || (path.extension().is_none() && path.metadata().map(|m| m.len()).unwrap_or(0) > 1024 * 50)
            {
                special_paths.push(path.to_path_buf());
            } else if file_name_lower.ends_with(".swift")
                || file_name_lower.ends_with(".m")
                || file_name_lower.ends_with(".h")
                || file_name_lower.ends_with(".java")
                || file_name_lower.ends_with(".kt")
                || file_name_lower.ends_with(".smali")
                || file_name_lower.ends_with(".xml")
                || file_name_lower.ends_with(".json")
                || file_name_lower.ends_with(".plist")
                || file_name_lower.ends_with(".js")
                || file_name_lower.ends_with(".ts")
                || file_name_lower.ends_with(".dart")
                || file_name_lower.ends_with(".html")
                || file_name_lower.ends_with(".txt")
                || file_name_lower.ends_with(".properties")
            {
                code_paths.push(path.to_path_buf());
            }

            // Framework identification
            if let Some(parent) = path.parent() {
                if let Some(pname) = parent.file_name().and_then(|n| n.to_str()) {
                    if pname.ends_with(".framework") {
                        if let Some(sdk) = analysis::sdk_profiler::match_framework(pname) {
                            if !app_meta.frameworks.contains(&sdk.name) {
                                app_meta.frameworks.push(sdk.name);
                            }
                        }
                    }
                }
            }
        }
        app_meta.platform = Some(detected_platform);

        // Process special configuration / binary files
        for path in &special_paths {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let file_name_lower = file_name.to_ascii_lowercase();

            if file_name == "Info.plist" {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let mut plist_findings = analysis::ios::audit_info_plist(&content, &path.display().to_string(), &mut app_meta);
                    findings.append(&mut plist_findings);
                }
            } else if file_name == "AndroidManifest.xml" {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let mut manifest_findings = analysis::android::audit_manifest(&content, &path.display().to_string(), &mut app_meta);
                    findings.append(&mut manifest_findings);

                    // Content Provider manifest audit
                    let mut cp_findings = analysis::content_provider::audit_manifest_providers(&content, &path.display().to_string());
                    findings.append(&mut cp_findings);
                }
            } else if file_name_lower == "network_security_config.xml" {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let mut nsc_findings = analysis::android::audit_network_security_config(&content, &path.display().to_string());
                    findings.append(&mut nsc_findings);
                }
            } else if file_name_lower.contains("entitlements") || file_name_lower.ends_with(".xcent") {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let mut ent_findings = analysis::ios::audit_entitlements(&content, &path.display().to_string());
                    findings.append(&mut ent_findings);
                }
            } else if file_name_lower == "libflutter.so" || file_name == "Flutter" || file_name_lower == "libapp.so" || file_name == "App" {
                if let Ok(bytes) = std::fs::read(path) {
                    let mut flutter_res = analysis::flutter::audit_flutter_binary(path, &bytes);
                    if let Some(hash) = flutter_res.engine_version_or_hash {
                        app_meta.frameworks.push(format!("Flutter Engine ({})", &hash[..8.min(hash.len())]));
                    }
                    for ep in flutter_res.endpoints {
                        if !all_endpoints.contains(&ep) {
                            all_endpoints.push(ep);
                        }
                    }
                    all_secrets.append(&mut flutter_res.secrets);
                    findings.append(&mut flutter_res.findings);
                }
            } else if file_name_lower.ends_with(".p12") || file_name_lower.ends_with(".pfx") {
                if let Some(p12_finding) = analysis::p12_cracker::audit_p12_file(path, Some(&app_meta.identifier), Some(&app_meta.name)) {
                    findings.push(p12_finding);
                }
            } else if path.extension().is_none() && path.metadata().map(|m| m.len()).unwrap_or(0) > 1024 * 50 {
                if let Ok(bytes) = std::fs::read(path) {
                    if let Some(encrypted) = analysis::ios::check_macho_encryption(&bytes) {
                        app_meta.is_encrypted = encrypted;
                    }
                }
            }
        }

        // Parallel processing of all code and configuration files across CPU threads
        let chunks: Vec<CodeScanChunk> = code_paths
            .par_iter()
            .map(|path| {
                let mut chunk = CodeScanChunk {
                    findings: Vec::new(),
                    endpoints: Vec::new(),
                    secrets: Vec::new(),
                    sdk_names: Vec::new(),
                };

                let Ok(content) = std::fs::read_to_string(path) else {
                    return chunk;
                };

                let path_str = path.display().to_string();

                // 1. Secrets scanning
                let mut secrets = analysis::shared::scan_secrets(&path_str, &content, detected_platform);
                chunk.secrets.extend(secrets.clone());
                chunk.findings.append(&mut secrets);

                // 2. Endpoints extraction
                let eps = analysis::shared::extract_endpoints(&content);
                chunk.endpoints.extend(eps);

                // 3. MobSF SAST rules
                let mut mobsf_findings = analysis::mobsf_rules::scan_mobsf_rules(&path_str, &content, detected_platform);
                chunk.findings.append(&mut mobsf_findings);

                // 4. SSL Pinning detector
                let mut ssl_findings = analysis::ssl_pin::audit_ssl_pinning(&content, &path_str);
                chunk.findings.append(&mut ssl_findings);

                // 5. WebView & JS bridge
                let mut webview_findings = analysis::webview::audit_webview_and_bridges(&content, &path_str);
                chunk.findings.append(&mut webview_findings);

                // 6. Deep links & Intents
                let mut deeplink_findings = analysis::deeplink::audit_deeplinks_and_intents(&content, &path_str);
                chunk.findings.append(&mut deeplink_findings);

                // 7. Cryptography vulnerabilities
                let mut crypto_findings = analysis::crypto::audit_cryptography(&content, &path_str);
                chunk.findings.append(&mut crypto_findings);

                // 8. Business logic flaws
                let mut logic_findings = analysis::logic_flaws::audit_logic_flaws(&content, &path_str);
                chunk.findings.append(&mut logic_findings);

                // 9. Content Provider SQLi and Path Traversal
                let mut cp_findings = analysis::content_provider::audit_provider_source_code(&content, &path_str);
                chunk.findings.append(&mut cp_findings);

                // 10. Insecure Storage Auditing (SharedPreferences / Keychain)
                let mut store_findings = match detected_platform {
                    Platform::Android => analysis::storage_audit::audit_android_storage(&content, &path_str),
                    Platform::Ios => analysis::storage_audit::audit_ios_storage(&content, &path_str),
                    _ => {
                        let mut both = analysis::storage_audit::audit_android_storage(&content, &path_str);
                        both.extend(analysis::storage_audit::audit_ios_storage(&content, &path_str));
                        both
                    }
                };
                chunk.findings.append(&mut store_findings);

                // 11. Third-party SDK signature match
                if let Some(sdk) = analysis::sdk_profiler::match_package_or_string(&content) {
                    chunk.sdk_names.push(sdk.name);
                }

                chunk
            })
            .collect();

        // Merge parallel results
        for mut chunk in chunks {
            findings.append(&mut chunk.findings);
            all_secrets.append(&mut chunk.secrets);
            for ep in chunk.endpoints {
                if !all_endpoints.contains(&ep) {
                    all_endpoints.push(ep);
                }
            }
            for sdk_name in chunk.sdk_names {
                if !app_meta.frameworks.contains(&sdk_name) {
                    app_meta.frameworks.push(sdk_name);
                }
            }
        }

        // Run security risk audit across all discovered third-party SDKs
        let mut sdk_infos = Vec::new();
        for fw in &app_meta.frameworks {
            if let Some(sig) = analysis::sdk_profiler::match_framework(fw).or_else(|| analysis::sdk_profiler::match_package_or_string(fw)) {
                sdk_infos.push(sig);
            }
        }
        let mut sdk_findings = analysis::sdk_profiler::audit_discovered_sdks(&sdk_infos, &bundle_dir.display().to_string());
        findings.append(&mut sdk_findings);

        // Apply 7-Question Gate evaluation to all findings
        for f in &mut findings {
            f.gate = validation::evaluate_gates(f);
        }

        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;
        let mut info_count = 0;
        let mut gate_passed_count = 0;

        for f in &findings {
            match f.severity {
                Severity::Critical => critical_count += 1,
                Severity::High => high_count += 1,
                Severity::Medium => medium_count += 1,
                Severity::Low => low_count += 1,
                Severity::Info => info_count += 1,
            }
            if f.gate.status == GateStatus::Passed {
                gate_passed_count += 1;
            }
        }

        Ok(ScanReport {
            target_app: app_meta,
            total_findings: findings.len(),
            critical_count,
            high_count,
            medium_count,
            low_count,
            info_count,
            gate_passed_count,
            findings,
            api_endpoints: all_endpoints,
            hardcoded_secrets: all_secrets,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }


    /// Extract an IPA archive and run the full MobHunt static audit
    pub fn scan_ipa(ipa_path: &Path) -> Result<ScanReport> {
        let file = File::open(ipa_path).map_err(|e| anyhow!("Failed to open IPA {}: {e}", ipa_path.display()))?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| anyhow!("Failed to read IPA as zip: {e}"))?;

        let temp_dir = tempfile::tempdir().map_err(|e| anyhow!("Failed to create temp dir: {e}"))?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => temp_dir.path().join(path),
                None => continue,
            };

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Self::scan_bundle_dir(temp_dir.path())
    }

    /// Extract an APK archive and run the full MobHunt static audit
    pub fn scan_apk(apk_path: &Path) -> Result<ScanReport> {
        let file = File::open(apk_path).map_err(|e| anyhow!("Failed to open APK {}: {e}", apk_path.display()))?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| anyhow!("Failed to read APK as zip: {e}"))?;

        let temp_dir = tempfile::tempdir().map_err(|e| anyhow!("Failed to create temp dir: {e}"))?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => temp_dir.path().join(path),
                None => continue,
            };

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Self::scan_bundle_dir(temp_dir.path())
    }

    /// Asynchronously verify a discovered secret using non-destructive metadata queries
    pub async fn verify_secret(secret_type: &str, secret_value: &str, bundle_id: Option<&str>) -> validation::SecretVerificationResult {
        let verifier = validation::SecretVerifier::new(bundle_id);
        verifier.verify(secret_type, secret_value).await
    }

    /// Synthesize an actionable exploit Proof-of-Concept command or payload for a finding
    pub fn generate_poc(finding: &models::Finding, package_or_bundle_id: Option<&str>) -> Option<validation::poc_generator::ExploitPoc> {
        validation::poc_generator::generate_poc_for_finding(finding, package_or_bundle_id)
    }

    /// Generate a complete HackerOne markdown bug bounty report
    pub fn generate_h1_report(finding: &models::Finding, app_meta: Option<&models::AppMetadata>) -> String {
        let default_meta = models::AppMetadata::default();
        reporting::h1::generate_h1_report(finding, app_meta.unwrap_or(&default_meta))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use models::Finding;

    #[test]
    fn test_cvss_calculation() {
        let res = reporting::calculate_cvss("AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N").unwrap();
        assert_eq!(res.severity, Severity::Critical);
        assert!(res.score >= 9.0);

        let res_local = reporting::calculate_cvss("AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N").unwrap();
        assert_eq!(res_local.severity, Severity::Medium);
        assert!(res_local.score >= 5.0 && res_local.score <= 6.5);
    }

    #[test]
    fn test_secret_scanner() {
        let content = "let openai_key = \"sk-live0123456789abcdef0123456789abcdef\";\nlet aws_id = \"AKIAIOSFODNN7EXAMPLE\";";
        let findings = analysis::shared::scan_secrets("Config.swift", content, Platform::Ios);
        assert_eq!(findings.len(), 2);
        assert!(findings.iter().any(|f| f.title.contains("OpenAI")));
        assert!(findings.iter().any(|f| f.title.contains("AWS")));
    }

    #[test]
    fn test_shannon_entropy() {
        let low_ent = analysis::shared::shannon_entropy("aaaaaaaaaaaaaa");
        let high_ent = analysis::shared::shannon_entropy("A9k#mZ$82x!L@q1-");
        assert!(high_ent > low_ent);
        assert!(high_ent > 3.5);
    }

    #[test]
    fn test_7_question_gate_never_submit() {
        let finding = models::Finding {
            id: "TEST-1".into(),
            rule_id: "XP-02".into(),
            title: "Missing SSL Certificate Pinning".into(),
            vuln_class: "cert_pinning".into(),
            severity: Severity::Low,
            platform: Platform::Ios,
            file_path: "Network.swift".into(),
            line: 10,
            snippet: "SecTrustEvaluate(trust, &result)".into(),
            description: "Missing pinning allows MITM on compromised roots.".into(),
            impact: "Defense in depth".into(),
            remediation: "Add pinning".into(),
            cvss: models::CvssData::default(),
            gate: models::GateResult {
                status: models::GateStatus::Passed,
                notes: vec![],
                chain_eligible: true,
            },
            reproduction_steps: vec![],
            metadata: std::collections::HashMap::new(),
        };

        let gate = validation::evaluate_gates(&finding);
        match gate.status {
            models::GateStatus::KilledNeverSubmit(_) => (),
            other => panic!("Expected KilledNeverSubmit, got {:?}", other),
        }
    }

    #[test]
    fn test_report_generation() {
        let finding = Finding {
            id: "FIND-01".into(),
            rule_id: "IOS-03".into(),
            title: "App Transport Security Disabled".into(),
            vuln_class: "cleartext_traffic".into(),
            severity: Severity::High,
            platform: Platform::Ios,
            file_path: "Info.plist".into(),
            line: 1,
            snippet: "<key>NSAllowsArbitraryLoads</key><true/>".into(),
            description: "Allows HTTP traffic across all domains.".into(),
            impact: "Cleartext credentials intercepted.".into(),
            remediation: "Enable ATS".into(),
            cvss: models::CvssData {
                score: 7.5,
                severity: Severity::High,
                vector: "CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:N/A:N".into(),
                exploitability: 1.6,
                impact: 5.9,
            },
            gate: models::GateResult {
                status: models::GateStatus::Passed,
                notes: vec![],
                chain_eligible: true,
            },
            reproduction_steps: vec!["Open Info.plist".into(), "Inspect ATS settings".into()],
            metadata: std::collections::HashMap::new(),
        };
        let app = AppMetadata {
            name: "SuperApp".into(),
            identifier: "com.example.superapp".into(),
            version: "2.4.1".into(),
            build: "102".into(),
            ..Default::default()
        };

        let report = reporting::generate_h1_report(&finding, &app);
        assert!(report.contains("# App Transport Security Disabled"));
        assert!(report.contains("com.example.superapp"));
        assert!(report.contains("Steps to Reproduce"));
    }

    #[test]
    fn test_mobsf_sast_integration() {
        let code = r#"
            SSLContext ctx = SSLContext.getInstance("TLS");
            ctx.init(null, new TrustManager[]{ new TrustAllSSLSocket() }, null);
            openFileOutput("userData.txt", Context.MODE_WORLD_READABLE);
        "#;
        let findings = analysis::mobsf_rules::scan_mobsf_rules("SecurityManager.java", code, Platform::Android);
        assert!(findings.iter().any(|f| f.rule_id == "MOBSF-AND-01"));
        assert!(findings.iter().any(|f| f.rule_id == "MOBSF-AND-03"));
    }

    #[test]
    fn test_flutter_binary_audit() {
        let dummy_dart_data = b"package:flutter_app/services/auth_service.dart\x00https://myproject.firebaseio.com\x00MethodChannel(\"com.app/biometrics\")\x00session_verify_cert_chain";
        let path = std::path::Path::new("libapp.so");
        let result = analysis::flutter::audit_flutter_binary(path, dummy_dart_data);
        assert!(result.dart_classes.contains(&"services/auth_service".to_string()));
        assert!(result.findings.iter().any(|f| f.rule_id == "FLUTTER-FIREBASE-RTDB"));
        assert!(result.findings.iter().any(|f| f.rule_id == "FLUTTER-METHOD-CHANNEL"));
    }

    #[test]
    fn test_webview_bridge_audit() {
        let code = r#"
            webView.addJavascriptInterface(new JsBridge(), "AndroidBridge");
            webView.getSettings().setAllowUniversalAccessFromFileURLs(true);
            webView.loadUrl(intent.getStringExtra("target_url"));
        "#;
        let findings = analysis::webview::audit_webview_and_bridges(code, "WebActivity.java");
        assert!(findings.iter().any(|f| f.rule_id == "WV-UNIVERSAL-ACCESS"));
        assert!(findings.iter().any(|f| f.rule_id == "WV-DEEPLINK-LOADURL"));
    }

    #[test]
    fn test_crypto_audit() {
        let code = r#"
            Cipher c = Cipher.getInstance("DES/ECB/PKCS5Padding");
            MessageDigest md = MessageDigest.getInstance("MD5");
        "#;
        let findings = analysis::crypto::audit_cryptography(code, "Hasher.java");
        assert!(findings.iter().any(|f| f.rule_id == "CRYPTO-WEAK-DES"));
        assert!(findings.iter().any(|f| f.rule_id == "CRYPTO-BROKEN-MD5"));
    }

    #[test]
    fn test_logic_flaw_audit() {
        let code = r#"
            boolean isAdmin = true;
            boolean isPaid = true;
        "#;
        let findings = analysis::logic_flaws::audit_logic_flaws(code, "UserAccount.java");
        assert!(findings.iter().any(|f| f.rule_id == "LOGIC-AUTH-BYPASS"));
        assert!(findings.iter().any(|f| f.rule_id == "LOGIC-PAYMENT-BYPASS"));
    }

    #[tokio::test]
    async fn test_secret_verifier_aws_format() {
        let verifier = validation::SecretVerifier::default();
        let res = verifier.verify("aws_access_key", "AKIAIOSFODNN7EXAMPLE").await;
        assert!(res.is_valid);
        assert_eq!(res.secret_type, "aws_access_key");

        let invalid_res = verifier.verify("aws_access_key", "invalid_key_123").await;
        assert!(!invalid_res.is_valid);
    }

    #[test]
    fn test_p12_candidates_generation() {
        let candidates = analysis::p12_cracker::generate_p12_candidates(Some("com.target.wallet"), Some("TargetWallet"));
        assert!(candidates.contains(&"".to_string()));
        assert!(candidates.contains(&"1234".to_string()));
        assert!(candidates.contains(&"wallet".to_string()));
        assert!(candidates.contains(&"TargetWallet".to_string()));
    }

    #[test]
    fn test_sdk_profiling() {
        let fw = analysis::sdk_profiler::match_framework("FirebaseAnalytics.framework").unwrap();
        assert_eq!(fw.name, "Firebase Analytics");
        assert_eq!(fw.category, analysis::sdk_profiler::SdkCategory::Analytics);

        let sql = analysis::sdk_profiler::match_package_or_string("import net.zetetic.database.sqlcipher;").unwrap();
        assert_eq!(sql.name, "SQLCipher");
        assert_eq!(sql.category, analysis::sdk_profiler::SdkCategory::Database);

        let findings = analysis::sdk_profiler::audit_discovered_sdks(&[sql], "SQLCipher.java");
        assert!(findings.iter().any(|f| f.rule_id == "SDK-AUDIT-SQLCIPHER"));
    }

    #[test]
    fn test_macho_cryptid_patch() {
        // Construct a minimal synthetic 64-bit Mach-O header with LC_ENCRYPTION_INFO_64
        let mut macho = vec![0u8; 64];
        // MH_MAGIC_64 (0xfeedfacf)
        macho[0..4].copy_from_slice(&0xfeedfacf_u32.to_le_bytes());
        // ncmds = 1 at offset 16
        macho[16..20].copy_from_slice(&1_u32.to_le_bytes());
        // Load command at offset 32: cmd = 0x2C (LC_ENCRYPTION_INFO_64), cmdsize = 24
        macho[32..36].copy_from_slice(&0x2C_u32.to_le_bytes());
        macho[36..40].copy_from_slice(&24_u32.to_le_bytes());
        // cryptid is at load_cmd_offset + 16 = 48; set to 1 (encrypted)
        macho[48..52].copy_from_slice(&1_u32.to_le_bytes());

        let patched = crate::domain::sentinel::jbops::patch_macho_cryptid_to_zero(&mut macho);
        assert!(patched);
        // cryptid should now be 0
        let cryptid = u32::from_le_bytes([macho[48], macho[49], macho[50], macho[51]]);
        assert_eq!(cryptid, 0);
    }

    #[test]
    fn test_content_provider_audit() {
        let manifest = r#"
            <manifest package="com.target.app">
                <application>
                    <provider
                        android:name="com.target.app.UserDataProvider"
                        android:authorities="com.target.app.user"
                        android:exported="true" />
                </application>
            </manifest>
        "#;
        let findings = analysis::content_provider::audit_manifest_providers(manifest, "AndroidManifest.xml");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "SEC-AND-CP-001");
        assert!(findings[0].title.contains("UserDataProvider"));

        let code = r#"
            Cursor c = db.rawQuery("SELECT * FROM users WHERE id = " + userId, null);
        "#;
        let sqli_findings = analysis::content_provider::audit_provider_source_code(code, "UserDataProvider.java");
        assert!(sqli_findings.iter().any(|f| f.rule_id == "SEC-AND-CP-SQLI"));
    }

    #[test]
    fn test_storage_audit() {
        let android_code = r#"
            getSharedPreferences("user_prefs", Context.MODE_WORLD_READABLE);
            prefs.edit().putString("auth_token", jwtToken).apply();
        "#;
        let android_findings = analysis::storage_audit::audit_android_storage(android_code, "AuthManager.java");
        assert!(android_findings.iter().any(|f| f.rule_id == "SEC-AND-STORE-001"));
        assert!(android_findings.iter().any(|f| f.rule_id == "SEC-AND-STORE-PLAINTEXT"));

        let ios_code = r#"
            let query: [String: Any] = [kSecAttrAccessible: kSecAttrAccessibleAlways]
            UserDefaults.standard.set(apiSecret, forKey: "api_key")
        "#;
        let ios_findings = analysis::storage_audit::audit_ios_storage(ios_code, "KeychainHelper.swift");
        assert!(ios_findings.iter().any(|f| f.rule_id == "SEC-IOS-STORE-ALWAYS"));
        assert!(ios_findings.iter().any(|f| f.rule_id == "SEC-IOS-STORE-NSUSERDEFAULTS"));
    }

    #[test]
    fn test_poc_generator() {
        let mut finding = Finding::new(
            "SEC-AND-CP-001",
            "Exported Unprotected ContentProvider: UserProvider",
            "authority: 'com.target.app.provider'",
            Severity::High,
            Platform::Android,
        );
        let poc = validation::poc_generator::generate_poc_for_finding(&finding, Some("com.target.app")).unwrap();
        assert_eq!(poc.poc_type, validation::poc_generator::PocType::AdbCommand);
        assert!(poc.payload_or_command.contains("adb shell content query --uri content://com.target.app.provider/"));

        finding.rule_id = "SEC-SEC-OPENAI".to_string();
        finding.title = "Hardcoded OpenAI API Key".to_string();
        finding.snippet = "let key = \"sk-live12345678901234567890123456789012\";".to_string();
        let secret_poc = validation::poc_generator::generate_poc_for_finding(&finding, None).unwrap();
        assert_eq!(secret_poc.poc_type, validation::poc_generator::PocType::CurlCommand);
        assert!(secret_poc.payload_or_command.contains("curl -s https://api.openai.com/v1/models"));
    }
}

