use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SdkCategory {
    Analytics,
    CrashReporting,
    Networking,
    Payment,
    Auth,
    CrossPlatform,
    DeepLinking,
    Maps,
    Push,
    Database,
}

impl SdkCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            SdkCategory::Analytics => "Analytics & Tracking",
            SdkCategory::CrashReporting => "Crash Reporting",
            SdkCategory::Networking => "Networking",
            SdkCategory::Payment => "Payment & Billing",
            SdkCategory::Auth => "Authentication",
            SdkCategory::CrossPlatform => "Cross-Platform Framework",
            SdkCategory::DeepLinking => "Deep Linking",
            SdkCategory::Maps => "Maps & Geolocation",
            SdkCategory::Push => "Push Notifications",
            SdkCategory::Database => "Local Database",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkInfo {
    pub name: String,
    pub category: SdkCategory,
    pub identifier: String,
}

pub struct SdkSignature {
    pub name: &'static str,
    pub category: SdkCategory,
    pub ios_framework: Option<&'static str>,
    pub android_package: Option<&'static str>,
    pub binary_string: Option<&'static str>,
}

pub static SDK_SIGNATURES: &[SdkSignature] = &[
    // Analytics
    SdkSignature { name: "Firebase Analytics", category: SdkCategory::Analytics, ios_framework: Some("FirebaseAnalytics"), android_package: Some("com.google.firebase.analytics"), binary_string: Some("FirebaseAnalytics") },
    SdkSignature { name: "Facebook SDK", category: SdkCategory::Analytics, ios_framework: Some("FBSDKCoreKit"), android_package: Some("com.facebook.sdk"), binary_string: Some("com.facebook.sdk") },
    SdkSignature { name: "Adjust", category: SdkCategory::Analytics, ios_framework: Some("Adjust"), android_package: Some("com.adjust.sdk"), binary_string: Some("com.adjust.sdk") },
    SdkSignature { name: "AppsFlyer", category: SdkCategory::Analytics, ios_framework: Some("AppsFlyerLib"), android_package: Some("com.appsflyer"), binary_string: Some("AppsFlyerLib") },
    SdkSignature { name: "Mixpanel", category: SdkCategory::Analytics, ios_framework: Some("Mixpanel"), android_package: Some("com.mixpanel"), binary_string: Some("Mixpanel") },
    SdkSignature { name: "Amplitude", category: SdkCategory::Analytics, ios_framework: Some("Amplitude"), android_package: Some("com.amplitude"), binary_string: Some("Amplitude") },
    SdkSignature { name: "Segment", category: SdkCategory::Analytics, ios_framework: Some("Analytics"), android_package: Some("com.segment.analytics"), binary_string: Some("com.segment.analytics") },

    // Crash Reporting
    SdkSignature { name: "Firebase Crashlytics", category: SdkCategory::CrashReporting, ios_framework: Some("FirebaseCrashlytics"), android_package: Some("com.google.firebase.crashlytics"), binary_string: Some("FirebaseCrashlytics") },
    SdkSignature { name: "Sentry", category: SdkCategory::CrashReporting, ios_framework: Some("Sentry"), android_package: Some("io.sentry"), binary_string: Some("io.sentry") },
    SdkSignature { name: "Bugsnag", category: SdkCategory::CrashReporting, ios_framework: Some("Bugsnag"), android_package: Some("com.bugsnag"), binary_string: Some("Bugsnag") },

    // Networking
    SdkSignature { name: "Alamofire", category: SdkCategory::Networking, ios_framework: Some("Alamofire"), android_package: None, binary_string: Some("Alamofire") },
    SdkSignature { name: "OkHttp", category: SdkCategory::Networking, ios_framework: None, android_package: Some("okhttp3"), binary_string: Some("okhttp3") },
    SdkSignature { name: "Retrofit", category: SdkCategory::Networking, ios_framework: None, android_package: Some("retrofit2"), binary_string: Some("retrofit2") },
    SdkSignature { name: "Moya", category: SdkCategory::Networking, ios_framework: Some("Moya"), android_package: None, binary_string: Some("Moya") },

    // Payment
    SdkSignature { name: "Stripe", category: SdkCategory::Payment, ios_framework: Some("Stripe"), android_package: Some("com.stripe.android"), binary_string: Some("com.stripe.android") },
    SdkSignature { name: "Braintree", category: SdkCategory::Payment, ios_framework: Some("BraintreeCore"), android_package: Some("com.braintreepayments"), binary_string: Some("com.braintreepayments") },

    // Auth
    SdkSignature { name: "Auth0", category: SdkCategory::Auth, ios_framework: Some("Auth0"), android_package: Some("com.auth0.android"), binary_string: Some("com.auth0.android") },
    SdkSignature { name: "AppAuth", category: SdkCategory::Auth, ios_framework: Some("AppAuth"), android_package: Some("net.openid.appauth"), binary_string: Some("net.openid.appauth") },

    // Cross-Platform
    SdkSignature { name: "React Native", category: SdkCategory::CrossPlatform, ios_framework: Some("React"), android_package: Some("com.facebook.react"), binary_string: Some("com.facebook.react") },
    SdkSignature { name: "Flutter", category: SdkCategory::CrossPlatform, ios_framework: Some("Flutter"), android_package: Some("io.flutter"), binary_string: Some("io.flutter") },
    SdkSignature { name: "Cordova", category: SdkCategory::CrossPlatform, ios_framework: Some("Cordova"), android_package: Some("org.apache.cordova"), binary_string: Some("org.apache.cordova") },
    SdkSignature { name: "Xamarin", category: SdkCategory::CrossPlatform, ios_framework: None, android_package: Some("mono.android"), binary_string: Some("mono.android") },

    // Deep Linking
    SdkSignature { name: "Branch Metrics", category: SdkCategory::DeepLinking, ios_framework: Some("Branch"), android_package: Some("io.branch"), binary_string: Some("io.branch") },
    SdkSignature { name: "Firebase Dynamic Links", category: SdkCategory::DeepLinking, ios_framework: Some("FirebaseDynamicLinks"), android_package: Some("com.google.firebase.dynamiclinks"), binary_string: Some("FirebaseDynamicLinks") },

    // Maps
    SdkSignature { name: "Google Maps", category: SdkCategory::Maps, ios_framework: Some("GoogleMaps"), android_package: Some("com.google.android.gms.maps"), binary_string: Some("GoogleMaps") },
    SdkSignature { name: "Mapbox", category: SdkCategory::Maps, ios_framework: Some("Mapbox"), android_package: Some("com.mapbox"), binary_string: Some("com.mapbox") },

    // Push
    SdkSignature { name: "OneSignal", category: SdkCategory::Push, ios_framework: Some("OneSignal"), android_package: Some("com.onesignal"), binary_string: Some("com.onesignal") },
    SdkSignature { name: "Pusher", category: SdkCategory::Push, ios_framework: Some("PusherSwift"), android_package: Some("com.pusher"), binary_string: Some("com.pusher") },

    // Database
    SdkSignature { name: "Realm", category: SdkCategory::Database, ios_framework: Some("Realm"), android_package: Some("io.realm"), binary_string: Some("io.realm") },
    SdkSignature { name: "SQLCipher", category: SdkCategory::Database, ios_framework: Some("SQLCipher"), android_package: Some("net.zetetic"), binary_string: Some("SQLCipher") },
];

/// Identifies third-party SDKs from framework bundle names (iOS)
pub fn match_framework(framework_name: &str) -> Option<SdkInfo> {
    let clean = framework_name.trim_end_matches(".framework");
    for sig in SDK_SIGNATURES {
        if let Some(fw) = sig.ios_framework {
            if fw.eq_ignore_ascii_case(clean) {
                return Some(SdkInfo {
                    name: sig.name.to_string(),
                    category: sig.category,
                    identifier: clean.to_string(),
                });
            }
        }
    }
    None
}

/// Identifies third-party SDKs from DEX packages or code imports (Android)
pub fn match_package_or_string(text: &str) -> Option<SdkInfo> {
    for sig in SDK_SIGNATURES {
        if let Some(pkg) = sig.android_package {
            if text.contains(pkg) {
                return Some(SdkInfo {
                    name: sig.name.to_string(),
                    category: sig.category,
                    identifier: pkg.to_string(),
                });
            }
        }
        if let Some(s) = sig.binary_string {
            if text.contains(s) {
                return Some(SdkInfo {
                    name: sig.name.to_string(),
                    category: sig.category,
                    identifier: s.to_string(),
                });
            }
        }
    }
    None
}

/// Analyzes discovered third-party SDKs for high-impact security configurations and bug bounty implications
pub fn audit_discovered_sdks(sdks: &[SdkInfo], file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for sdk in sdks {
        if !seen.insert(sdk.name.clone()) {
            continue;
        }

        // Check for SQLCipher: Verify if encryption key is properly protected
        if sdk.name == "SQLCipher" {
            findings.push(Finding {
                id: "SDK-SQLCIPHER-FOUND".to_string(),
                rule_id: "SDK-AUDIT-SQLCIPHER".to_string(),
                title: "SQLCipher Encrypted Database Engine Detected".to_string(),
                vuln_class: "MASVS-STORAGE-1".to_string(),
                severity: Severity::Low,
                platform: Platform::CrossPlatform,
                file_path: file_path.to_string(),
                line: 1,
                snippet: "SQLCipher".to_string(),
                description: "Application embeds SQLCipher for database encryption. Audit the codebase to ensure the encryption passphrase is not hardcoded, derived from static strings, or stored in unencrypted SharedPreferences/UserDefaults.".to_string(),
                impact: "If the SQLCipher key is hardcoded or predictable, all encrypted sqlite database tables can be decrypted offline.".to_string(),
                remediation: "Generate a 256-bit AES key at first launch, store it in the OS Keychain or Android Keystore with biometric/user authentication required.".to_string(),
                cvss: CvssData {
                    score: 4.3,
                    severity: Severity::Low,
                    vector: "CVSS:3.1/AV:L/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N".to_string(),
                    exploitability: 2.5,
                    impact: 1.8,
                },
                gate: GateResult {
                    status: GateStatus::Passed,
                    notes: vec!["SQLCipher library detected".to_string()],
                    chain_eligible: true,
                },
                reproduction_steps: vec![
                    "Search codebase for calls to `SQLiteDatabase.openOrCreateDatabase(..., passphrase, ...)` or `sqlite3_key`.".to_string(),
                    "Verify how the passphrase argument is generated and stored.".to_string(),
                ],
                metadata: HashMap::new(),
            });
        }

        // Check for Cordova / PhoneGap legacy cross-platform web bridges
        if sdk.name == "Cordova" {
            findings.push(Finding {
                id: "SDK-CORDOVA-LEGACY".to_string(),
                rule_id: "SDK-AUDIT-CORDOVA".to_string(),
                title: "Apache Cordova / PhoneGap Legacy Hybrid Framework Detected".to_string(),
                vuln_class: "MASVS-PLATFORM-6".to_string(),
                severity: Severity::Medium,
                platform: Platform::CrossPlatform,
                file_path: file_path.to_string(),
                line: 1,
                snippet: "org.apache.cordova".to_string(),
                description: "Application uses Apache Cordova hybrid web framework. Cordova bridges full native Android/iOS capabilities to the web layer via `CordovaPlugin` message handlers, creating a wide attack surface if XSS or unvalidated deep links occur.".to_string(),
                impact: "XSS in any bundled web asset or remote webview page can execute native shell commands, access geolocation, camera, and local files.".to_string(),
                remediation: "Audit `config.xml` allow-navigation and access origin whitelists. Enforce strict Content-Security-Policy (CSP) headers.".to_string(),
                cvss: CvssData {
                    score: 6.8,
                    severity: Severity::Medium,
                    vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:N".to_string(),
                    exploitability: 2.8,
                    impact: 4.0,
                },
                gate: GateResult {
                    status: GateStatus::Passed,
                    notes: vec!["Apache Cordova framework located".to_string()],
                    chain_eligible: true,
                },
                reproduction_steps: vec![
                    "Inspect `assets/www/index.html` and `res/xml/config.xml` for unrestricted `<allow-navigation href=\"*\" />`.".to_string(),
                ],
                metadata: HashMap::new(),
            });
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_matching() {
        let sdk = match_framework("FirebaseAnalytics.framework").unwrap();
        assert_eq!(sdk.name, "Firebase Analytics");
        assert_eq!(sdk.category, SdkCategory::Analytics);

        let sdk_sql = match_framework("SQLCipher").unwrap();
        assert_eq!(sdk_sql.name, "SQLCipher");
        assert_eq!(sdk_sql.category, SdkCategory::Database);
    }

    #[test]
    fn test_package_matching() {
        let sdk = match_package_or_string("import com.adjust.sdk.Adjust;").unwrap();
        assert_eq!(sdk.name, "Adjust");
        assert_eq!(sdk.category, SdkCategory::Analytics);
    }
}
