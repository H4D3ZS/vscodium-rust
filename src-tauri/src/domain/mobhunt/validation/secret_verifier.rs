use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretVerificationResult {
    pub secret_type: String,
    pub masked_value: String,
    pub is_valid: bool,
    pub access_level: String, // "none", "read", "write", "admin", "exposed"
    pub details: String,
    pub error: Option<String>,
}

pub struct SecretVerifier {
    client: reqwest::Client,
}

impl Default for SecretVerifier {
    fn default() -> Self {
        Self::new(None)
    }
}

impl SecretVerifier {
    pub fn new(bundle_id: Option<&str>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("VscodiumMobHunt/1.0 (Defensive Security Research)"),
        );
        if let Some(bid) = bundle_id {
            if let Ok(hv) = HeaderValue::from_str(bid) {
                headers.insert("X-Ios-Bundle-Identifier", hv.clone());
                headers.insert("X-Android-Package", hv);
            }
        }

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(8))
            .default_headers(headers)
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Non-destructively verify a secret against official read-only metadata endpoints
    pub async fn verify(&self, secret_type: &str, secret_value: &str) -> SecretVerificationResult {
        let masked = if secret_value.len() > 10 {
            format!("{}...{}", &secret_value[..6], &secret_value[secret_value.len() - 4..])
        } else {
            "***".to_string()
        };

        match secret_type {
            "google_api_key" => self.validate_google_api_key(secret_value, masked).await,
            "stripe_secret_key" | "stripe_key" => self.validate_stripe_key(secret_value, masked).await,
            "firebase_url" => self.validate_firebase_url(secret_value, masked).await,
            "firebase_key" => self.validate_firebase_fcm(secret_value, masked).await,
            "slack_token" => self.validate_slack_token(secret_value, masked).await,
            "discord_webhook" => self.validate_discord_webhook(secret_value, masked).await,
            "github_token" => self.validate_github_token(secret_value, masked).await,
            "openai_key" => self.validate_openai_key(secret_value, masked).await,
            "anthropic_api_key" => self.validate_anthropic_key(secret_value, masked).await,
            "gemini_api_key" => self.validate_gemini_key(secret_value, masked).await,
            "sendgrid_key" => self.validate_sendgrid_key(secret_value, masked).await,
            "aws_access_key" => self.validate_aws_key_format(secret_value, masked),
            _ => SecretVerificationResult {
                secret_type: secret_type.to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "No automated live validator configured for this secret type.".to_string(),
                error: None,
            },
        }
    }

    async fn validate_google_api_key(&self, key: &str, masked: String) -> SecretVerificationResult {
        let geocode_url = format!("https://maps.googleapis.com/maps/api/geocode/json?address=test&key={}", key);
        let id_url = format!("https://www.googleapis.com/identitytoolkit/v3/relyingparty/getProjectConfig?key={}", key);

        let mut working = Vec::new();

        if let Ok(res) = self.client.get(&geocode_url).send().await {
            if res.status().is_success() {
                if let Ok(json) = res.json::<serde_json::Value>().await {
                    let status = json.get("status").and_then(|s| s.as_str()).unwrap_or("");
                    if status == "OK" || status == "ZERO_RESULTS" {
                        working.push("Geocoding API");
                    }
                }
            }
        }

        if let Ok(res) = self.client.get(&id_url).send().await {
            if res.status().is_success() {
                if let Ok(json) = res.json::<serde_json::Value>().await {
                    if json.get("error").is_none() {
                        working.push("Identity Toolkit API");
                    }
                }
            }
        }

        let is_valid = !working.is_empty();
        SecretVerificationResult {
            secret_type: "google_api_key".to_string(),
            masked_value: masked,
            is_valid,
            access_level: if is_valid { "read".to_string() } else { "none".to_string() },
            details: if is_valid {
                format!("Active Google APIs: {}", working.join(", "))
            } else {
                "Google APIs returned access denied or invalid key".to_string()
            },
            error: None,
        }
    }

    async fn validate_stripe_key(&self, key: &str, masked: String) -> SecretVerificationResult {
        let res = self
            .client
            .get("https://api.stripe.com/v1/balance")
            .basic_auth(key, Some(""))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let currency = json["available"][0]["currency"].as_str().unwrap_or("usd");
                    let amount = json["available"][0]["amount"].as_i64().unwrap_or(0) as f64 / 100.0;
                    SecretVerificationResult {
                        secret_type: "stripe_key".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: if key.starts_with("sk_live") { "admin".to_string() } else { "test".to_string() },
                        details: format!("Balance query successful: {:.2} {}", amount, currency.to_uppercase()),
                        error: None,
                    }
                } else {
                    SecretVerificationResult {
                        secret_type: "stripe_key".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: "admin".to_string(),
                        details: "Stripe key is active and authenticated".to_string(),
                        error: None,
                    }
                }
            }
            Ok(resp) => SecretVerificationResult {
                secret_type: "stripe_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "none".to_string(),
                details: format!("Stripe API returned HTTP {}", resp.status()),
                error: None,
            },
            Err(e) => SecretVerificationResult {
                secret_type: "stripe_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Network error during Stripe validation".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_firebase_url(&self, raw_url: &str, masked: String) -> SecretVerificationResult {
        let base_url = if !raw_url.starts_with("http") {
            format!("https://{}", raw_url)
        } else {
            raw_url.to_string()
        };

        let target = if base_url.ends_with(".json") {
            base_url
        } else {
            format!("{}/.json", base_url.trim_end_matches('/'))
        };

        match self.client.get(&target).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status == 200 {
                    let text = resp.text().await.unwrap_or_default();
                    let is_empty = text == "null" || text.trim() == "{}";
                    SecretVerificationResult {
                        secret_type: "firebase_url".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: "exposed".to_string(),
                        details: if !is_empty {
                            format!("CRITICAL: Realtime Database allows public unauthenticated reads! Sample: {}", &text[..text.len().min(120)])
                        } else {
                            "Publicly accessible Realtime Database, but root is currently empty.".to_string()
                        },
                        error: None,
                    }
                } else if status == 401 {
                    SecretVerificationResult {
                        secret_type: "firebase_url".to_string(),
                        masked_value: masked,
                        is_valid: false,
                        access_level: "secured".to_string(),
                        details: "Firebase security rules enforce authentication (HTTP 401 Unauthorized)".to_string(),
                        error: None,
                    }
                } else {
                    SecretVerificationResult {
                        secret_type: "firebase_url".to_string(),
                        masked_value: masked,
                        is_valid: false,
                        access_level: "unknown".to_string(),
                        details: format!("HTTP {} from Firebase endpoint", status),
                        error: None,
                    }
                }
            }
            Err(e) => SecretVerificationResult {
                secret_type: "firebase_url".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection failure to Firebase database".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_firebase_fcm(&self, key: &str, masked: String) -> SecretVerificationResult {
        let res = self
            .client
            .post("https://fcm.googleapis.com/fcm/send")
            .header(AUTHORIZATION, format!("key={}", key))
            .header(CONTENT_TYPE, "application/json")
            .body(r#"{"to":"test_dummy_token"}"#)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status() != 401 => SecretVerificationResult {
                secret_type: "firebase_key".to_string(),
                masked_value: masked,
                is_valid: true,
                access_level: "write".to_string(),
                details: "FCM server key authenticated successfully (can send push notifications)".to_string(),
                error: None,
            },
            Ok(_) => SecretVerificationResult {
                secret_type: "firebase_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "none".to_string(),
                details: "FCM server key authentication rejected (HTTP 401)".to_string(),
                error: None,
            },
            Err(e) => SecretVerificationResult {
                secret_type: "firebase_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection error validating FCM key".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_slack_token(&self, token: &str, masked: String) -> SecretVerificationResult {
        let res = self
            .client
            .get("https://slack.com/api/auth.test")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if json.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
                        let user = json.get("user").and_then(|u| u.as_str()).unwrap_or("unknown");
                        let team = json.get("team").and_then(|t| t.as_str()).unwrap_or("unknown");
                        return SecretVerificationResult {
                            secret_type: "slack_token".to_string(),
                            masked_value: masked,
                            is_valid: true,
                            access_level: "read".to_string(),
                            details: format!("Valid Slack token for user: {}, team: {}", user, team),
                            error: None,
                        };
                    }
                }
                SecretVerificationResult {
                    secret_type: "slack_token".to_string(),
                    masked_value: masked,
                    is_valid: false,
                    access_level: "none".to_string(),
                    details: "Slack token authentication rejected".to_string(),
                    error: None,
                }
            }
            Err(e) => SecretVerificationResult {
                secret_type: "slack_token".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection error validating Slack token".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_discord_webhook(&self, webhook_url: &str, masked: String) -> SecretVerificationResult {
        match self.client.get(webhook_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let name = json.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                    SecretVerificationResult {
                        secret_type: "discord_webhook".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: "write".to_string(),
                        details: format!("Active Discord webhook for bot/webhook name: {}", name),
                        error: None,
                    }
                } else {
                    SecretVerificationResult {
                        secret_type: "discord_webhook".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: "write".to_string(),
                        details: "Valid Discord webhook reachable".to_string(),
                        error: None,
                    }
                }
            }
            Ok(resp) => SecretVerificationResult {
                secret_type: "discord_webhook".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "none".to_string(),
                details: format!("Discord webhook returned HTTP {}", resp.status()),
                error: None,
            },
            Err(e) => SecretVerificationResult {
                secret_type: "discord_webhook".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection error verifying Discord webhook".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_github_token(&self, token: &str, masked: String) -> SecretVerificationResult {
        let res = self
            .client
            .get("https://api.github.com/user")
            .header(AUTHORIZATION, format!("token {}", token))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let scopes = resp
                    .headers()
                    .get("x-oauth-scopes")
                    .and_then(|s| s.to_str().ok())
                    .unwrap_or("none")
                    .to_string();

                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let login = json.get("login").and_then(|l| l.as_str()).unwrap_or("unknown");
                    SecretVerificationResult {
                        secret_type: "github_token".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: if scopes.contains("repo") { "admin".to_string() } else { "read".to_string() },
                        details: format!("Valid GitHub token for user: {}, scopes: {}", login, scopes),
                        error: None,
                    }
                } else {
                    SecretVerificationResult {
                        secret_type: "github_token".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: "read".to_string(),
                        details: "Valid GitHub personal access token".to_string(),
                        error: None,
                    }
                }
            }
            Ok(resp) => SecretVerificationResult {
                secret_type: "github_token".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "none".to_string(),
                details: format!("GitHub API returned HTTP {}", resp.status()),
                error: None,
            },
            Err(e) => SecretVerificationResult {
                secret_type: "github_token".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection error verifying GitHub token".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_openai_key(&self, key: &str, masked: String) -> SecretVerificationResult {
        let res = self
            .client
            .get("https://api.openai.com/v1/models")
            .header(AUTHORIZATION, format!("Bearer {}", key))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => SecretVerificationResult {
                secret_type: "openai_key".to_string(),
                masked_value: masked,
                is_valid: true,
                access_level: "write".to_string(),
                details: "Active OpenAI API key verified (can make billable completion and embedding queries)".to_string(),
                error: None,
            },
            Ok(resp) => SecretVerificationResult {
                secret_type: "openai_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "none".to_string(),
                details: format!("OpenAI API returned HTTP {}", resp.status()),
                error: None,
            },
            Err(e) => SecretVerificationResult {
                secret_type: "openai_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection error validating OpenAI key".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_anthropic_key(&self, key: &str, masked: String) -> SecretVerificationResult {
        let payload = serde_json::json!({
            "model": "claude-3-haiku-20240307",
            "max_tokens": 1,
            "messages": [{"role": "user", "content": "ping"}]
        });

        let res = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .header(CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status() != 401 && resp.status() != 403 => SecretVerificationResult {
                secret_type: "anthropic_api_key".to_string(),
                masked_value: masked,
                is_valid: true,
                access_level: "write".to_string(),
                details: "Active Anthropic API key verified (can query Claude models)".to_string(),
                error: None,
            },
            Ok(_) => SecretVerificationResult {
                secret_type: "anthropic_api_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "none".to_string(),
                details: "Anthropic API key authentication rejected (HTTP 401/403)".to_string(),
                error: None,
            },
            Err(e) => SecretVerificationResult {
                secret_type: "anthropic_api_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection error validating Anthropic key".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_gemini_key(&self, key: &str, masked: String) -> SecretVerificationResult {
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models?key={}", key);
        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => SecretVerificationResult {
                secret_type: "gemini_api_key".to_string(),
                masked_value: masked,
                is_valid: true,
                access_level: "write".to_string(),
                details: "Active Gemini API key verified (can invoke Google Generative AI models)".to_string(),
                error: None,
            },
            Ok(resp) => SecretVerificationResult {
                secret_type: "gemini_api_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "none".to_string(),
                details: format!("Gemini API returned HTTP {}", resp.status()),
                error: None,
            },
            Err(e) => SecretVerificationResult {
                secret_type: "gemini_api_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection error validating Gemini key".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    async fn validate_sendgrid_key(&self, key: &str, masked: String) -> SecretVerificationResult {
        let res = self
            .client
            .get("https://api.sendgrid.com/v3/user/profile")
            .header(AUTHORIZATION, format!("Bearer {}", key))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let email = json.get("email").and_then(|e| e.as_str()).unwrap_or("unknown");
                    SecretVerificationResult {
                        secret_type: "sendgrid_key".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: "read".to_string(),
                        details: format!("Valid SendGrid key for account: {}", email),
                        error: None,
                    }
                } else {
                    SecretVerificationResult {
                        secret_type: "sendgrid_key".to_string(),
                        masked_value: masked,
                        is_valid: true,
                        access_level: "read".to_string(),
                        details: "Active SendGrid API key verified".to_string(),
                        error: None,
                    }
                }
            }
            Ok(resp) => SecretVerificationResult {
                secret_type: "sendgrid_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "none".to_string(),
                details: format!("SendGrid API returned HTTP {}", resp.status()),
                error: None,
            },
            Err(e) => SecretVerificationResult {
                secret_type: "sendgrid_key".to_string(),
                masked_value: masked,
                is_valid: false,
                access_level: "unknown".to_string(),
                details: "Connection error validating SendGrid key".to_string(),
                error: Some(e.to_string()),
            },
        }
    }

    fn validate_aws_key_format(&self, key: &str, masked: String) -> SecretVerificationResult {
        let is_format_valid = key.starts_with("AKIA") && key.len() == 20 && key.chars().all(|c| c.is_ascii_alphanumeric());
        SecretVerificationResult {
            secret_type: "aws_access_key".to_string(),
            masked_value: masked,
            is_valid: is_format_valid,
            access_level: if is_format_valid { "unknown".to_string() } else { "none".to_string() },
            details: if is_format_valid {
                "Valid AWS IAM Access Key ID format (AKIA). Live credential test requires corresponding AWS Secret Key.".to_string()
            } else {
                "Invalid AWS Access Key format".to_string()
            },
            error: None,
        }
    }
}
