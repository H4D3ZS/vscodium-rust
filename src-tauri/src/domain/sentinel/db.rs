//! Sentinel persistence — SQLite (rusqlite). Targets, findings, reports.
//! Replaces the FlutterSentinel dual Postgres/SQLite backend with a single
//! self-contained DB the IDE owns.

use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};
use std::path::Path;
use std::sync::Mutex;

fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Row shape for a scanned mobile app (acquired + analyzed artifact).
#[derive(Debug, Clone, Default)]
pub struct MobileAssetRow {
    pub app_name: String,
    pub package: String,
    pub version: String,
    pub apk_name: String,
    pub file_path: String,
    pub scan_ref: String,
    pub method: String,
    pub status: String,
    pub findings_count: i64,
    pub severity: String,
    pub endpoints_count: i64,
}

fn mobile_asset_map(r: &rusqlite::Row<'_>) -> rusqlite::Result<Value> {
    Ok(json!({
        "id": r.get::<_, String>(0)?,
        "app_name": r.get::<_, String>(1)?,
        "package": r.get::<_, String>(2)?,
        "version": r.get::<_, String>(3)?,
        "apk_name": r.get::<_, String>(4)?,
        "file_path": r.get::<_, String>(5)?,
        "scan_ref": r.get::<_, String>(6)?,
        "method": r.get::<_, String>(7)?,
        "status": r.get::<_, String>(8)?,
        "findings_count": r.get::<_, i64>(9)?,
        "severity": r.get::<_, String>(10)?,
        "endpoints_count": r.get::<_, i64>(11)?,
        "created_at": r.get::<_, String>(12)?,
    }))
}

pub struct SentinelDb {
    conn: Mutex<Connection>,
}

impl SentinelDb {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init()?;
        Ok(db)
    }

    pub fn open_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS targets (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                root_domain TEXT,
                platform TEXT,
                scope TEXT,
                program TEXT,
                scope_type TEXT,
                status TEXT DEFAULT 'active',
                created_at TEXT,
                updated_at TEXT,
                metadata TEXT DEFAULT '{}'
            );
            CREATE TABLE IF NOT EXISTS findings (
                id TEXT PRIMARY KEY,
                target_id TEXT,
                title TEXT NOT NULL,
                severity TEXT,
                cwe TEXT,
                owasp TEXT,
                description TEXT,
                evidence TEXT,
                status TEXT DEFAULT 'open',
                created_at TEXT
            );
            CREATE TABLE IF NOT EXISTS reports (
                id TEXT PRIMARY KEY,
                target_id TEXT,
                platform TEXT,
                title TEXT,
                body TEXT,
                status TEXT DEFAULT 'draft',
                created_at TEXT
            );
            CREATE TABLE IF NOT EXISTS mobile_assets (
                id TEXT PRIMARY KEY,
                app_name TEXT,
                package TEXT,
                version TEXT,
                apk_name TEXT,
                file_path TEXT,
                scan_ref TEXT,
                method TEXT,
                status TEXT DEFAULT 'scanned',
                findings_count INTEGER DEFAULT 0,
                severity TEXT,
                endpoints_count INTEGER DEFAULT 0,
                created_at TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_findings_target ON findings(target_id);
            CREATE INDEX IF NOT EXISTS idx_reports_target ON reports(target_id);
            CREATE INDEX IF NOT EXISTS idx_mobile_created ON mobile_assets(created_at);
            "#,
        )?;
        Ok(())
    }

    // ── Targets ──

    pub fn create_target(
        &self,
        name: &str,
        root_domain: Option<&str>,
        platform: Option<&str>,
        scope: Option<&str>,
        program: Option<&str>,
        scope_type: Option<&str>,
    ) -> Result<Value> {
        let id = uuid::Uuid::new_v4().to_string();
        let ts = now();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO targets (id, name, root_domain, platform, scope, program, scope_type, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'active', ?8, ?8)",
            params![
                id,
                name,
                root_domain.unwrap_or(""),
                platform.unwrap_or(""),
                scope.unwrap_or(""),
                program.unwrap_or(""),
                scope_type.unwrap_or(""),
                ts,
            ],
        )?;
        Ok(self.target_row(&conn, &id)?.unwrap_or(json!({ "id": id })))
    }

    pub fn update_target(
        &self,
        id: &str,
        name: Option<&str>,
        root_domain: Option<&str>,
        program: Option<&str>,
        status: Option<&str>,
    ) -> Result<Value> {
        let conn = self.conn.lock().unwrap();
        let existing = self
            .target_row(&conn, id)?
            .ok_or_else(|| anyhow!("target not found: {id}"))?;
        let cur = |k: &str| {
            existing
                .get(k)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };
        conn.execute(
            "UPDATE targets SET name=?1, root_domain=?2, program=?3, status=?4, updated_at=?5 WHERE id=?6",
            params![
                name.unwrap_or(&cur("name")),
                root_domain.unwrap_or(&cur("root_domain")),
                program.unwrap_or(&cur("program")),
                status.unwrap_or(&cur("status")),
                now(),
                id
            ],
        )?;
        self.target_row(&conn, id)?
            .ok_or_else(|| anyhow!("target vanished: {id}"))
    }

    pub fn delete_target(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM findings WHERE target_id=?1", params![id])?;
        conn.execute("DELETE FROM reports WHERE target_id=?1", params![id])?;
        conn.execute("DELETE FROM targets WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn get_target(&self, id: &str) -> Result<Option<Value>> {
        let conn = self.conn.lock().unwrap();
        self.target_row(&conn, id)
    }

    pub fn list_targets(&self) -> Result<Vec<Value>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, root_domain, platform, scope, program, scope_type, status, created_at, updated_at, metadata
             FROM targets ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(json!({
                "id": r.get::<_, String>(0)?,
                "name": r.get::<_, String>(1)?,
                "root_domain": r.get::<_, String>(2)?,
                "platform": r.get::<_, String>(3)?,
                "scope": r.get::<_, String>(4)?,
                "program": r.get::<_, String>(5)?,
                "scope_type": r.get::<_, String>(6)?,
                "status": r.get::<_, String>(7)?,
                "created_at": r.get::<_, String>(8)?,
                "updated_at": r.get::<_, String>(9)?,
            }))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    fn target_row(&self, conn: &Connection, id: &str) -> Result<Option<Value>> {
        conn.query_row(
            "SELECT id, name, root_domain, platform, scope, program, scope_type, status, created_at, updated_at, metadata
             FROM targets WHERE id=?1",
            params![id],
            |r| {
                Ok(json!({
                    "id": r.get::<_, String>(0)?,
                    "name": r.get::<_, String>(1)?,
                    "root_domain": r.get::<_, String>(2)?,
                    "platform": r.get::<_, String>(3)?,
                    "scope": r.get::<_, String>(4)?,
                    "program": r.get::<_, String>(5)?,
                    "scope_type": r.get::<_, String>(6)?,
                    "status": r.get::<_, String>(7)?,
                    "created_at": r.get::<_, String>(8)?,
                    "updated_at": r.get::<_, String>(9)?,
                }))
            },
        )
        .optional()
        .map_err(|e| anyhow!(e))
    }

    // ── Findings ──

    pub fn create_finding(&self, f: &FindingRow) -> Result<Value> {
        let id = uuid::Uuid::new_v4().to_string();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO findings (id, target_id, title, severity, cwe, owasp, description, evidence, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                id,
                f.target_id,
                f.title,
                f.severity,
                f.cwe,
                f.owasp,
                f.description,
                f.evidence,
                f.status,
                now()
            ],
        )?;
        self.finding_row(&conn, &id)?
            .ok_or_else(|| anyhow!("finding vanished"))
    }

    pub fn list_findings(&self, target_id: Option<&str>) -> Result<Vec<Value>> {
        let conn = self.conn.lock().unwrap();
        let (sql, val): (&str, Option<String>) = match target_id {
            Some(t) => (
                "SELECT id, target_id, title, severity, cwe, owasp, description, evidence, status, created_at
                 FROM findings WHERE target_id=?1 ORDER BY created_at DESC",
                Some(t.to_string()),
            ),
            None => (
                "SELECT id, target_id, title, severity, cwe, owasp, description, evidence, status, created_at
                 FROM findings ORDER BY created_at DESC",
                None,
            ),
        };
        let mut out = Vec::new();
        match val {
            Some(v) => {
                let mut stmt = conn.prepare(sql)?;
                let rows = stmt.query_map(params![v], finding_map)?;
                for row in rows {
                    out.push(row?);
                }
            }
            None => {
                let mut stmt = conn.prepare(sql)?;
                let rows = stmt.query_map([], finding_map)?;
                for row in rows {
                    out.push(row?);
                }
            }
        }
        Ok(out)
    }

    pub fn update_finding_status(&self, id: &str, status: &str) -> Result<Value> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE findings SET status=?1 WHERE id=?2",
            params![status, id],
        )?;
        self.finding_row(&conn, id)?
            .ok_or_else(|| anyhow!("finding not found: {id}"))
    }

    fn finding_row(&self, conn: &Connection, id: &str) -> Result<Option<Value>> {
        conn.query_row(
            "SELECT id, target_id, title, severity, cwe, owasp, description, evidence, status, created_at
             FROM findings WHERE id=?1",
            params![id],
            finding_map,
        )
        .optional()
        .map_err(|e| anyhow!(e))
    }

    // ── Reports ──

    pub fn create_report(
        &self,
        target_id: &str,
        platform: &str,
        title: &str,
        body: &str,
        status: &str,
    ) -> Result<Value> {
        let id = uuid::Uuid::new_v4().to_string();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO reports (id, target_id, platform, title, body, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, target_id, platform, title, body, status, now()],
        )?;
        self.report_row(&conn, &id)?
            .ok_or_else(|| anyhow!("report vanished"))
    }

    pub fn list_reports(&self) -> Result<Vec<Value>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, target_id, platform, title, body, status, created_at FROM reports ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(json!({
                "id": r.get::<_, String>(0)?,
                "target_id": r.get::<_, String>(1)?,
                "platform": r.get::<_, String>(2)?,
                "title": r.get::<_, String>(3)?,
                "body": r.get::<_, String>(4)?,
                "status": r.get::<_, String>(5)?,
                "created_at": r.get::<_, String>(6)?,
            }))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    fn report_row(&self, conn: &Connection, id: &str) -> Result<Option<Value>> {
        conn.query_row(
            "SELECT id, target_id, platform, title, body, status, created_at FROM reports WHERE id=?1",
            params![id],
            |r| {
                Ok(json!({
                    "id": r.get::<_, String>(0)?,
                    "target_id": r.get::<_, String>(1)?,
                    "platform": r.get::<_, String>(2)?,
                    "title": r.get::<_, String>(3)?,
                    "body": r.get::<_, String>(4)?,
                    "status": r.get::<_, String>(5)?,
                    "created_at": r.get::<_, String>(6)?,
                }))
            },
        )
        .optional()
        .map_err(|e| anyhow!(e))
    }

    // ── Mobile assets ──

    pub fn create_mobile_asset(&self, asset: &MobileAssetRow) -> Result<Value> {
        let id = uuid::Uuid::new_v4().to_string();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO mobile_assets (
                id, app_name, package, version, apk_name, file_path, scan_ref,
                method, status, findings_count, severity, endpoints_count, created_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
            params![
                id,
                asset.app_name,
                asset.package,
                asset.version,
                asset.apk_name,
                asset.file_path,
                asset.scan_ref,
                asset.method,
                asset.status,
                asset.findings_count,
                asset.severity,
                asset.endpoints_count,
                now(),
            ],
        )?;
        self.mobile_asset_row(&conn, &id)?
            .ok_or_else(|| anyhow!("asset vanished"))
    }

    pub fn list_mobile_assets(&self) -> Result<Vec<Value>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, app_name, package, version, apk_name, file_path, scan_ref,
                    method, status, findings_count, severity, endpoints_count, created_at
             FROM mobile_assets ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], mobile_asset_map)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn delete_mobile_asset(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM mobile_assets WHERE id=?1", params![id])?;
        Ok(())
    }

    fn mobile_asset_row(&self, conn: &Connection, id: &str) -> Result<Option<Value>> {
        conn.query_row(
            "SELECT id, app_name, package, version, apk_name, file_path, scan_ref,
                    method, status, findings_count, severity, endpoints_count, created_at
             FROM mobile_assets WHERE id=?1",
            params![id],
            mobile_asset_map,
        )
        .optional()
        .map_err(|e| anyhow!(e))
    }

    // ── Stats ──

    pub fn stats(&self) -> Result<Value> {
        let conn = self.conn.lock().unwrap();
        let targets: i64 = conn.query_row("SELECT COUNT(*) FROM targets", [], |r| r.get(0))?;
        let findings: i64 = conn.query_row("SELECT COUNT(*) FROM findings", [], |r| r.get(0))?;
        let open: i64 = conn.query_row(
            "SELECT COUNT(*) FROM findings WHERE status='open'",
            [],
            |r| r.get(0),
        )?;
        let reports: i64 = conn.query_row("SELECT COUNT(*) FROM reports", [], |r| r.get(0))?;
        let mut sev: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        {
            let mut stmt =
                conn.prepare("SELECT severity, COUNT(*) FROM findings GROUP BY severity")?;
            let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))?;
            for row in rows {
                if let Ok((k, v)) = row {
                    *sev.entry(k).or_insert(0) += v;
                }
            }
        }
        Ok(json!({
            "targets": targets,
            "findings": findings,
            "open_findings": open,
            "reports": reports,
            "by_severity": sev,
        }))
    }
}

fn finding_map(r: &rusqlite::Row<'_>) -> rusqlite::Result<Value> {
    Ok(json!({
        "id": r.get::<_, String>(0)?,
        "target_id": r.get::<_, String>(1)?,
        "title": r.get::<_, String>(2)?,
        "severity": r.get::<_, String>(3)?,
        "cwe": r.get::<_, String>(4)?,
        "owasp": r.get::<_, String>(5)?,
        "description": r.get::<_, String>(6)?,
        "evidence": r.get::<_, String>(7)?,
        "status": r.get::<_, String>(8)?,
        "created_at": r.get::<_, String>(9)?,
    }))
}

#[derive(Debug, Default)]
pub struct FindingRow {
    pub target_id: String,
    pub title: String,
    pub severity: String,
    pub cwe: String,
    pub owasp: String,
    pub description: String,
    pub evidence: String,
    pub status: String,
}

impl FindingRow {
    pub fn new(target_id: String, title: String) -> Self {
        Self {
            target_id,
            title,
            ..Default::default()
        }
    }
}
