//! SQLite persistence for the fingerprint database.
//!
//! Stores components, vulnerability fingerprints, matched hypotheses, and
//! payout history. Payout history feeds the matcher's expected-value weighting
//! so the queue is prioritized by what has historically paid, not by chance.
//!
//! SQLite (not sled) is used because the queries are relational: find every
//! vulnerability for a component, join payout history to rank classes.
//!
//! Authorized testing / bug bounty use only.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

use rusqlite::{params, Connection};

use crate::domain::fingerprint::model::*;

pub struct FingerprintStore {
    conn: Arc<Mutex<Connection>>,
}

/// Serialize a `T: Serialize` to a JSON scalar and parse it back — used for the
/// enum columns so they round-trip losslessly without hand-written mappings.
fn json_of<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string(v).unwrap_or_else(|_| "\"\"".into())
}

fn parse_json<T: serde::de::DeserializeOwned>(s: &str) -> rusqlite::Result<T> {
    serde_json::from_str(s).map_err(|_| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            rusqlite::types::Type::Text,
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "json decode",
            )),
        )
    })
}

impl FingerprintStore {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let store = Self {
            conn: Arc::new(Mutex::new(Connection::open(path)?)),
        };
        store.init()?;
        Ok(store)
    }

    pub fn in_memory() -> rusqlite::Result<Self> {
        let store = Self {
            conn: Arc::new(Mutex::new(Connection::open_in_memory()?)),
        };
        store.init()?;
        Ok(store)
    }

    fn init(&self) -> rusqlite::Result<()> {
        let conn = self.lock();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS components (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                name TEXT NOT NULL,
                version_range TEXT,
                signature TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS vuln_fingerprints (
                id TEXT PRIMARY KEY,
                component_id TEXT NOT NULL,
                class TEXT NOT NULL,
                root_cause TEXT NOT NULL,
                confidence TEXT NOT NULL,
                first_seen_app TEXT NOT NULL,
                first_seen_at INTEGER NOT NULL,
                repro TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS app_matches (
                id TEXT PRIMARY KEY,
                app_id TEXT NOT NULL,
                vuln_fingerprint_id TEXT NOT NULL,
                match_confidence REAL NOT NULL,
                verification_result TEXT NOT NULL,
                verified_at INTEGER
            );
            CREATE TABLE IF NOT EXISTS payout_records (
                id TEXT PRIMARY KEY,
                vuln_fingerprint_id TEXT NOT NULL,
                program TEXT NOT NULL,
                amount REAL NOT NULL,
                submitted_at INTEGER NOT NULL
            );",
        )
    }

    fn lock(&self) -> MutexGuard<'_, Connection> {
        self.conn
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn insert_component(&self, c: &ComponentFingerprint) -> rusqlite::Result<()> {
        let conn = self.lock();
        conn.execute(
            "INSERT OR REPLACE INTO components (id, kind, name, version_range, signature)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                c.id,
                json_of(&c.kind),
                c.name,
                c.version_range,
                json_of(&c.signature),
            ],
        )?;
        Ok(())
    }

    pub fn list_components(&self) -> rusqlite::Result<Vec<ComponentFingerprint>> {
        let conn = self.lock();
        let mut stmt =
            conn.prepare("SELECT id, kind, name, version_range, signature FROM components")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let kind: String = row.get(1)?;
            let name: String = row.get(2)?;
            let version_range: Option<String> = row.get(3)?;
            let signature: String = row.get(4)?;
            Ok((id, kind, name, version_range, signature))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (id, kind, name, version_range, signature) = row?;
            out.push(ComponentFingerprint {
                id,
                kind: parse_json(&kind)?,
                name,
                version_range,
                signature: parse_json(&signature)?,
            });
        }
        Ok(out)
    }

    pub fn insert_vuln(&self, v: &VulnFingerprint) -> rusqlite::Result<()> {
        let conn = self.lock();
        conn.execute(
            "INSERT OR REPLACE INTO vuln_fingerprints
                (id, component_id, class, root_cause, confidence, first_seen_app, first_seen_at, repro)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                v.id,
                v.component_id,
                json_of(&v.class),
                json_of(&v.root_cause),
                json_of(&v.confidence),
                v.first_seen_app,
                v.first_seen_at,
                json_of(&v.repro),
            ],
        )?;
        Ok(())
    }

    pub fn list_vulns(&self) -> rusqlite::Result<Vec<VulnFingerprint>> {
        self.vulns_where("", &[])
    }

    pub fn vulns_for_component(
        &self,
        component_id: &str,
    ) -> rusqlite::Result<Vec<VulnFingerprint>> {
        self.vulns_where("WHERE component_id = ?1", &[&component_id])
    }

    fn vulns_where(
        &self,
        clause: &str,
        args: &[&dyn rusqlite::ToSql],
    ) -> rusqlite::Result<Vec<VulnFingerprint>> {
        let conn = self.lock();
        let sql = format!(
            "SELECT id, component_id, class, root_cause, confidence, first_seen_app, first_seen_at, repro
             FROM vuln_fingerprints {clause}"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(args, |row| {
            let id: String = row.get(0)?;
            let component_id: String = row.get(1)?;
            let class: String = row.get(2)?;
            let root_cause: String = row.get(3)?;
            let confidence: String = row.get(4)?;
            let first_seen_app: String = row.get(5)?;
            let first_seen_at: i64 = row.get(6)?;
            let repro: String = row.get(7)?;
            Ok((
                id,
                component_id,
                class,
                root_cause,
                confidence,
                first_seen_app,
                first_seen_at,
                repro,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (
                id,
                component_id,
                class,
                root_cause,
                confidence,
                first_seen_app,
                first_seen_at,
                repro,
            ) = row?;
            out.push(VulnFingerprint {
                id,
                component_id,
                class: parse_json(&class)?,
                root_cause: parse_json(&root_cause)?,
                confidence: parse_json(&confidence)?,
                first_seen_app,
                first_seen_at,
                repro: parse_json(&repro)?,
            });
        }
        Ok(out)
    }

    pub fn record_match(
        &self,
        app_id: &str,
        vuln_fingerprint_id: &str,
        score: f32,
        status: VerificationStatus,
    ) -> rusqlite::Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let conn = self.lock();
        conn.execute(
            "INSERT INTO app_matches (id, app_id, vuln_fingerprint_id, match_confidence, verification_result, verified_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id,
                app_id,
                vuln_fingerprint_id,
                score,
                json_of(&status),
                None::<i64>,
            ],
        )?;
        Ok(())
    }

    pub fn record_payout(
        &self,
        vuln_fingerprint_id: &str,
        program: &str,
        amount: f64,
    ) -> rusqlite::Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let conn = self.lock();
        conn.execute(
            "INSERT INTO payout_records (id, vuln_fingerprint_id, program, amount, submitted_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                id,
                vuln_fingerprint_id,
                program,
                amount,
                chrono::Utc::now().timestamp(),
            ],
        )?;
        Ok(())
    }

    /// (class, submissions, total_usd) aggregated from payout history. Feed
    /// this into the matcher as expected-value weights via [`payout_weights`].
    pub fn payout_stats(&self) -> rusqlite::Result<Vec<(VulnClass, usize, f64)>> {
        let conn = self.lock();
        let mut stmt = conn.prepare(
            "SELECT v.class, COUNT(*), COALESCE(SUM(p.amount), 0)
             FROM payout_records p
             JOIN vuln_fingerprints v ON p.vuln_fingerprint_id = v.id
             GROUP BY v.class",
        )?;
        let rows = stmt.query_map([], |row| {
            let class: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            let total: f64 = row.get(2)?;
            Ok((class, count as usize, total))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (class, count, total) = row?;
            out.push((parse_json::<VulnClass>(&class)?, count, total));
        }
        Ok(out)
    }

    /// Matcher weights: average payout per class, capped and scaled so a
    /// well-paying class raises priority without dominating confidence.
    pub fn payout_weights(&self) -> rusqlite::Result<HashMap<VulnClass, f32>> {
        let mut map = HashMap::new();
        for (class, count, total) in self.payout_stats()? {
            if count > 0 {
                let avg = (total / count as f64) as f32;
                let weight = (avg / 10_000.0).clamp(0.0, 2.0);
                map.insert(class, weight);
            }
        }
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn comp() -> ComponentFingerprint {
        ComponentFingerprint {
            id: "comp-1".into(),
            kind: ComponentKind::BackendFramework,
            name: "In-house API".into(),
            version_range: None,
            signature: DetectionSignature::NetworkHostPattern {
                pattern: "*.example.com".into(),
            },
        }
    }

    fn vuln(id: &str, class: VulnClass) -> VulnFingerprint {
        VulnFingerprint {
            id: id.into(),
            component_id: "comp-1".into(),
            class,
            root_cause: RootCause {
                endpoint_pattern: "/api/v1/things/{id}".into(),
                trigger: TriggerCondition::MissingOwnershipCheck {
                    id_field: "id".into(),
                },
                methods: vec!["GET".into()],
            },
            confidence: ConfidenceTier::Probable,
            first_seen_app: "com.a".into(),
            first_seen_at: 0,
            repro: ReproTemplate {
                steps: vec![],
                placeholders: vec![],
                canary: None,
            },
        }
    }

    #[test]
    fn roundtrip_components_and_vulns() {
        let store = FingerprintStore::in_memory().unwrap();
        store.insert_component(&comp()).unwrap();
        store
            .insert_vuln(&vuln("v1", VulnClass::AgenticInjection))
            .unwrap();

        let comps = store.list_components().unwrap();
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].name, "In-house API");

        let vulns = store.vulns_for_component("comp-1").unwrap();
        assert_eq!(vulns.len(), 1);
        assert_eq!(vulns[0].class, VulnClass::AgenticInjection);
        assert_eq!(vulns[0].root_cause.endpoint_pattern, "/api/v1/things/{id}");
    }

    #[test]
    fn payout_stats_drive_weights() {
        let store = FingerprintStore::in_memory().unwrap();
        store.insert_vuln(&vuln("v-bola", VulnClass::Bola)).unwrap();
        store
            .insert_vuln(&vuln("v-header", VulnClass::HeaderTrust))
            .unwrap();

        store.record_payout("v-bola", "Program A", 5_000.0).unwrap();
        store.record_payout("v-bola", "Program B", 7_000.0).unwrap();
        store
            .record_payout("v-header", "Program A", 1_000.0)
            .unwrap();

        let stats = store.payout_stats().unwrap();
        assert_eq!(stats.len(), 2);

        let weights = store.payout_weights().unwrap();
        assert!(weights[&VulnClass::Bola] > weights[&VulnClass::HeaderTrust]);
    }
}
