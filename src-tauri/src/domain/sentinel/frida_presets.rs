//! Dynamic Frida Hook Presets for Mobile Vulnerability Research & Bug Bounty
//!
//! Provides pre-built, production-grade Frida bypass scripts that can be directly
//! injected via USB/Frida onto connected iOS and Android test devices.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FridaPresetKind {
    FlutterBoringSsl,
    AndroidOkHttpTrustManager,
    IosSecTrustPinning,
    UniversalJailbreakRoot,
}

impl FridaPresetKind {
    pub fn name(&self) -> &'static str {
        match self {
            FridaPresetKind::FlutterBoringSsl => "Flutter BoringSSL Pinning Bypass",
            FridaPresetKind::AndroidOkHttpTrustManager => "Android Universal TLS & OkHttp Pinning Bypass",
            FridaPresetKind::IosSecTrustPinning => "iOS Universal SecTrust Pinning Bypass",
            FridaPresetKind::UniversalJailbreakRoot => "Universal Jailbreak & Root Detection Bypass",
        }
    }

    pub fn script_source(&self) -> &'static str {
        match self {
            FridaPresetKind::FlutterBoringSsl => FLUTTER_BORINGSSL_BYPASS,
            FridaPresetKind::AndroidOkHttpTrustManager => ANDROID_UNIVERSAL_PINNING_BYPASS,
            FridaPresetKind::IosSecTrustPinning => IOS_SECTRUST_PINNING_BYPASS,
            FridaPresetKind::UniversalJailbreakRoot => UNIVERSAL_JAILBREAK_ROOT_BYPASS,
        }
    }
}

pub static FLUTTER_BORINGSSL_BYPASS: &str = r#"/*
 * Flutter BoringSSL Certificate Verification Bypass
 * Hooks session_verify_cert_chain across libflutter.so and Flutter.framework
 */
(function() {
    console.log("[+] [MobHunt] Initializing Flutter BoringSSL Bypass...");

    function hookFlutter() {
        var m = Process.findModuleByName("libflutter.so") || Process.findModuleByName("Flutter");
        if (!m) {
            return false;
        }

        console.log("[+] Found Flutter module at: " + m.base);

        var symbols = m.enumerateSymbols();
        var hooked = 0;

        for (var i = 0; i < symbols.length; i++) {
            var sym = symbols[i];
            if (sym.name.indexOf("session_verify_cert_chain") !== -1 ||
                sym.name.indexOf("ssl_crypto_x509_session_verify_cert_chain") !== -1) {
                console.log("[*] Hooking verification symbol: " + sym.name + " at " + sym.address);
                Interceptor.attach(sym.address, {
                    onLeave: function(retval) {
                        retval.replace(ptr(1));
                    }
                });
                hooked++;
            }
        }

        // Pattern scanning fallback for stripped binaries
        if (hooked === 0) {
            console.log("[*] Flutter symbols stripped. Scanning for BoringSSL handshake patterns...");
            Memory.scan(m.base, m.size, "ff 83 00 d1 1f 04 00 71", {
                onMatch: function(address, size) {
                    console.log("[+] Potential verify pattern at: " + address);
                    Interceptor.attach(address, {
                        onLeave: function(retval) {
                            retval.replace(ptr(1));
                        }
                    });
                },
                onComplete: function() {}
            });
        }

        console.log("[+] Flutter BoringSSL bypass active! Traffic can now be intercepted.");
        return true;
    }

    if (!hookFlutter()) {
        var interval = setInterval(function() {
            if (hookFlutter()) {
                clearInterval(interval);
            }
        }, 200);
    }
})();
"#;

pub static ANDROID_UNIVERSAL_PINNING_BYPASS: &str = r#"/*
 * Android Universal TLS & OkHttp Certificate Pinning Bypass
 */
Java.perform(function() {
    console.log("[+] [MobHunt] Initializing Android Universal Pinning Bypass...");

    // 1. OkHttp3 CertificatePinner
    try {
        var CertificatePinner = Java.use("okhttp3.CertificatePinner");
        CertificatePinner.check.overload("java.lang.String", "java.util.List").implementation = function(hostname, peerCertificates) {
            console.log("[*] OkHttp3 CertificatePinner.check bypassed for: " + hostname);
            return;
        };
    } catch(e) {}

    // 2. TrustManagerImpl
    try {
        var TrustManagerImpl = Java.use("com.android.org.conscrypt.TrustManagerImpl");
        TrustManagerImpl.verifyChain.implementation = function(untrustedChain, trustAnchorChain, host, clientAuth, ocspData, tlsSctData) {
            console.log("[*] Conscrypt TrustManagerImpl.verifyChain bypassed for host: " + host);
            return untrustedChain;
        };
    } catch(e) {}

    // 3. Trustkit
    try {
        var TrustKit = Java.use("com.datatheorem.android.trustkit.pinning.OkHostnameVerifier");
        TrustKit.verify.overload("java.lang.String", "javax.net.ssl.SSLSession").implementation = function(host, session) {
            console.log("[*] TrustKit OkHostnameVerifier bypassed for host: " + host);
            return true;
        };
    } catch(e) {}

    console.log("[+] Android Pinning Bypass successfully injected.");
});
"#;

pub static IOS_SECTRUST_PINNING_BYPASS: &str = r#"/*
 * iOS Universal SecTrust Pinning Bypass
 * Intercepts Security framework verification routines
 */
(function() {
    console.log("[+] [MobHunt] Initializing iOS SecTrust Pinning Bypass...");

    var secTrustEvaluateWithError = Module.findExportByName("Security", "SecTrustEvaluateWithError");
    if (secTrustEvaluateWithError) {
        Interceptor.attach(secTrustEvaluateWithError, {
            onEnter: function(args) {
                this.errorPtr = args[1];
            },
            onLeave: function(retval) {
                if (this.errorPtr && !this.errorPtr.isNull()) {
                    this.errorPtr.writePointer(NULL);
                }
                retval.replace(ptr(1)); // true / errSecSuccess
            }
        });
        console.log("[*] Hooked SecTrustEvaluateWithError");
    }

    var secTrustEvaluate = Module.findExportByName("Security", "SecTrustEvaluate");
    if (secTrustEvaluate) {
        Interceptor.attach(secTrustEvaluate, {
            onEnter: function(args) {
                this.resultPtr = args[1];
            },
            onLeave: function(retval) {
                if (this.resultPtr && !this.resultPtr.isNull()) {
                    this.resultPtr.writeInt(1); // kSecTrustResultProceed
                }
                retval.replace(ptr(0)); // errSecSuccess
            }
        });
        console.log("[*] Hooked SecTrustEvaluate");
    }

    console.log("[+] iOS TLS pinning bypass successfully enabled.");
})();
"#;

pub static UNIVERSAL_JAILBREAK_ROOT_BYPASS: &str = r#"/*
 * Universal iOS Jailbreak & Android Root Detection Bypass
 */
if (ObjC.available) {
    console.log("[+] [MobHunt] Initializing iOS Jailbreak Detection Bypass...");

    var jbPaths = [
        "/Applications/Cydia.app",
        "/Applications/Sileo.app",
        "/Applications/Zebra.app",
        "/Library/MobileSubstrate/MobileSubstrate.dylib",
        "/bin/bash",
        "/usr/sbin/sshd",
        "/etc/apt",
        "/usr/bin/ssh",
        "/private/var/lib/apt"
    ];

    var hookNSFileManager = ObjC.classes.NSFileManager;
    if (hookNSFileManager) {
        Interceptor.attach(hookNSFileManager["- fileExistsAtPath:"].implementation, {
            onEnter: function(args) {
                var path = ObjC.Object(args[2]).toString();
                this.isJb = false;
                for (var i = 0; i < jbPaths.length; i++) {
                    if (path.indexOf(jbPaths[i]) !== -1) {
                        this.isJb = true;
                        break;
                    }
                }
            },
            onLeave: function(retval) {
                if (this.isJb) {
                    retval.replace(ptr(0)); // Return NO
                }
            }
        });
    }
}

if (Java.available) {
    Java.perform(function() {
        console.log("[+] [MobHunt] Initializing Android Root Detection Bypass...");

        var rootBinaries = ["su", "busybox", "magisk"];
        var File = Java.use("java.io.File");
        File.exists.implementation = function() {
            var name = this.getName();
            for (var i = 0; i < rootBinaries.length; i++) {
                if (name === rootBinaries[i]) {
                    return false;
                }
            }
            return this.exists();
        };
    });
}
"#;

/// Generates a combined script or retrieves a specific Frida preset
pub fn get_frida_preset(kind: FridaPresetKind) -> &'static str {
    kind.script_source()
}

/// Returns all available Frida bypass presets
pub fn all_presets() -> &'static [FridaPresetKind] {
    &[
        FridaPresetKind::FlutterBoringSsl,
        FridaPresetKind::AndroidOkHttpTrustManager,
        FridaPresetKind::IosSecTrustPinning,
        FridaPresetKind::UniversalJailbreakRoot,
    ]
}
