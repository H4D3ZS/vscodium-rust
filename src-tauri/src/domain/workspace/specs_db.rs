use rusqlite::{params, Connection, Result as SqlResult};
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub specs: String,
    pub status: String,
    pub preferred_provider: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkItem {
    pub id: i64,
    pub work_type: String, // GenerateTest, GenerateCode, Testing
    pub data_id: i64,      // function_id or file_id
    pub status: String,    // Pending, Processing, Done, Failed
    pub logs: String,
    pub current_log: String, // For real-time streaming
    pub preview_code: String, // Live code preview
    pub retries: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRecord {
    pub id: i64,
    pub project_id: i64,
    pub path: String,
    pub content: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionRecord {
    pub id: i64,
    pub file_id: i64,
    pub name: String,
    pub signature: String,
    pub code: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestRecord {
    pub id: i64,
    pub function_id: i64,
    pub code: Option<String>,
    pub status: String,
}

pub struct SpecDb {
    conn: Mutex<Connection>,
}

impl SpecDb {
    pub fn new(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(path).map_err(|e| anyhow!(e))?;
        let db = Self { conn: Mutex::new(conn) };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                specs TEXT NOT NULL,
                status TEXT NOT NULL,
                preferred_provider TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| anyhow!(e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                path TEXT NOT NULL,
                content TEXT,
                status TEXT NOT NULL,
                FOREIGN KEY(project_id) REFERENCES projects(id)
            )",
            [],
        ).map_err(|e| anyhow!(e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS functions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                signature TEXT NOT NULL,
                code TEXT,
                status TEXT NOT NULL,
                FOREIGN KEY(file_id) REFERENCES files(id)
            )",
            [],
        ).map_err(|e| anyhow!(e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tests (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                function_id INTEGER NOT NULL,
                code TEXT,
                status TEXT NOT NULL,
                FOREIGN KEY(function_id) REFERENCES functions(id)
            )",
            [],
        ).map_err(|e| anyhow!(e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS work_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                work_type TEXT NOT NULL,
                data_id INTEGER NOT NULL,
                status TEXT NOT NULL,
                logs TEXT,
                current_log TEXT DEFAULT '',
                preview_code TEXT DEFAULT '',
                retries INTEGER DEFAULT 0
            )",
            [],
        ).map_err(|e| anyhow!(e))?;

        // Migration: Ensure preferred_provider exists in projects
        let _ = conn.execute("ALTER TABLE projects ADD COLUMN preferred_provider TEXT", []);
        
        // Migration: Ensure current_log exists in work_items
        let _ = conn.execute("ALTER TABLE work_items ADD COLUMN current_log TEXT DEFAULT ''", []);

        // Migration: Ensure preview_code exists in work_items
        let _ = conn.execute("ALTER TABLE work_items ADD COLUMN preview_code TEXT DEFAULT ''", []);

        Ok(())
    }

    pub fn create_project(&self, name: &str, specs: &str, provider: Option<&str>) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO projects (name, specs, status, preferred_provider) VALUES (?1, ?2, 'Pending', ?3)",
            params![name, specs, provider],
        ).map_err(|e| anyhow!(e))?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_projects(&self) -> Result<Vec<Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, specs, status, preferred_provider, created_at FROM projects").map_err(|e| anyhow!(e))?;
        let rows = stmt.query_map([], |row| {
             Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                specs: row.get(2)?,
                status: row.get(3)?,
                preferred_provider: row.get(4)?,
                created_at: row.get(5)?,
            })
        }).map_err(|e| anyhow!(e))?;

        let mut projects = Vec::new();
        for row in rows {
            projects.push(row.map_err(|e| anyhow!(e))?);
        }
        Ok(projects)
    }

    pub fn get_project(&self, id: i64) -> Result<Project> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, specs, status, preferred_provider, created_at FROM projects WHERE id = ?1",
            params![id],
            |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    specs: row.get(2)?,
                    status: row.get(3)?,
                    preferred_provider: row.get(4)?,
                    created_at: row.get(5)?,
                })
            },
        ).map_err(|e| anyhow!(e))
    }

    pub fn set_project_provider(&self, id: i64, provider: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE projects SET preferred_provider = ?1 WHERE id = ?2",
            params![provider, id],
        ).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn delete_project(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute("DELETE FROM work_items WHERE data_id IN (SELECT id FROM functions WHERE file_id IN (SELECT id FROM files WHERE project_id = ?1))", params![id]);
        let _ = conn.execute("DELETE FROM tests WHERE function_id IN (SELECT id FROM functions WHERE file_id IN (SELECT id FROM files WHERE project_id = ?1))", params![id]);
        let _ = conn.execute("DELETE FROM functions WHERE file_id IN (SELECT id FROM files WHERE project_id = ?1)", params![id]);
        let _ = conn.execute("DELETE FROM files WHERE project_id = ?1", params![id]);
        conn.execute("DELETE FROM projects WHERE id = ?1", params![id]).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn delete_work_item(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM work_items WHERE id = ?1", params![id]).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn clear_history(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let _ = conn.execute("DELETE FROM tests", []);
        let _ = conn.execute("DELETE FROM work_items", []);
        let _ = conn.execute("DELETE FROM functions", []);
        let _ = conn.execute("DELETE FROM files", []);
        let _ = conn.execute("DELETE FROM projects", []);
        Ok(())
    }

    pub fn add_work_item(&self, work_type: &str, data_id: i64) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        
        // Deduplication
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM work_items WHERE work_type = ?1 AND data_id = ?2 AND status IN ('Pending', 'Processing'))",
            params![work_type, data_id],
            |row| row.get(0)
        ).map_err(|e| anyhow!(e))?;
        
        if exists {
            return conn.query_row(
                "SELECT id FROM work_items WHERE work_type = ?1 AND data_id = ?2 AND status IN ('Pending', 'Processing') LIMIT 1",
                params![work_type, data_id],
                |row| row.get(0)
            ).map_err(|e| anyhow!(e));
        }

        conn.execute(
            "INSERT INTO work_items (work_type, data_id, status, logs, retries) VALUES (?1, ?2, 'Pending', '', 0)",
            params![work_type, data_id],
        ).map_err(|e| anyhow!(e))?;
        Ok(conn.last_insert_rowid())
    }

    pub fn fetch_pending_work(&self) -> Result<Option<WorkItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, work_type, data_id, status, logs, current_log, preview_code, retries FROM work_items WHERE status = 'Pending' LIMIT 1").map_err(|e| anyhow!(e))?;
        let mut rows = stmt.query_map([], |row| {
            Ok(WorkItem {
                id: row.get(0)?,
                work_type: row.get(1)?,
                data_id: row.get(2)?,
                status: row.get(3)?,
                logs: row.get(4).unwrap_or_default(),
                current_log: row.get(5).unwrap_or_default(),
                preview_code: row.get(6).unwrap_or_default(),
                retries: row.get(7)?,
            })
        }).map_err(|e| anyhow!(e))?;

        if let Some(res) = rows.next() {
            let work = res.map_err(|e| anyhow!(e))?;
            // Mark as processing
            conn.execute("UPDATE work_items SET status = 'Processing' WHERE id = ?1", params![work.id]).map_err(|e| anyhow!(e))?;
            Ok(Some(work))
        } else {
            Ok(None)
        }
    }

    pub fn update_work_status(&self, id: i64, status: &str, logs: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE work_items SET status = ?1, logs = ?2 WHERE id = ?3",
            params![status, logs, id],
        ).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn update_work_log(&self, id: i64, current_log: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE work_items SET current_log = ?1 WHERE id = ?2",
            params![current_log, id],
        ).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn update_preview_code(&self, id: i64, code: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE work_items SET preview_code = ?1 WHERE id = ?2",
            params![code, id],
        ).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn increment_retry(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE work_items SET retries = retries + 1, status = 'Pending' WHERE id = ?1",
            params![id],
        ).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn add_file(&self, project_id: i64, path: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO files (project_id, path, status) VALUES (?1, ?2, 'Pending')",
            params![project_id, path],
        ).map_err(|e| anyhow!(e))?;
        Ok(conn.last_insert_rowid())
    }

    pub fn add_function(&self, file_id: i64, name: &str, signature: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO functions (file_id, name, signature, status) VALUES (?1, ?2, ?3, 'Pending')",
            params![file_id, name, signature],
        ).map_err(|e| anyhow!(e))?;
        Ok(conn.last_insert_rowid())
    }

    pub fn save_test_code(&self, function_id: i64, code: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tests (function_id, code, status) VALUES (?1, ?2, 'Done')",
            params![function_id, code],
        ).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn save_function_code(&self, id: i64, code: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE functions SET code = ?1 WHERE id = ?2",
            params![code, id],
        ).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn update_function_status(&self, id: i64, status: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE functions SET status = ?1 WHERE id = ?2",
            params![status, id],
        ).map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn get_function(&self, id: i64) -> Result<FunctionRecord> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, file_id, name, signature, code, status FROM functions WHERE id = ?1",
            params![id],
            |row| {
                Ok(FunctionRecord {
                    id: row.get(0)?,
                    file_id: row.get(1)?,
                    name: row.get(2)?,
                    signature: row.get(3).unwrap_or_default(),
                    code: row.get(4)?,
                    status: row.get(5)?,
                })
            },
        ).map_err(|e| anyhow!(e))
    }

    pub fn get_file(&self, id: i64) -> Result<FileRecord> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, project_id, path, content, status FROM files WHERE id = ?1",
            params![id],
            |row| {
                Ok(FileRecord {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    path: row.get(2)?,
                    content: row.get(3)?,
                    status: row.get(4)?,
                })
            },
        ).map_err(|e| anyhow!(e))
    }

    pub fn get_tests_for_function(&self, function_id: i64) -> Result<Vec<TestRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, function_id, code, status FROM tests WHERE function_id = ?1").map_err(|e| anyhow!(e))?;
        let rows = stmt.query_map(params![function_id], |row| {
            Ok(TestRecord {
                id: row.get(0)?,
                function_id: row.get(1)?,
                code: row.get(2)?,
                status: row.get(3)?,
            })
        }).map_err(|e| anyhow!(e))?;

        let mut tests = Vec::new();
        for row in rows {
            tests.push(row.map_err(|e| anyhow!(e))?);
        }
        Ok(tests)
    }

    pub fn get_file_full_content(&self, file_id: i64) -> Result<String> {
        let conn = self.conn.lock().unwrap();
        
        let path: String = conn.query_row(
            "SELECT path FROM files WHERE id = ?1",
            params![file_id],
            |row| row.get(0)
        ).map_err(|e| anyhow!(e))?;

        let mut stmt = conn.prepare("SELECT signature, code FROM functions WHERE file_id = ?1").map_err(|e| anyhow!(e))?;
        let funcs = stmt.query_map(params![file_id], |row| {
            let sig: String = row.get(0)?;
            let code: Option<String> = row.get(1)?;
            Ok((sig, code))
        }).map_err(|e| anyhow!(e))?.collect::<SqlResult<Vec<_>>>().map_err(|e| anyhow!(e))?;

        let mut full_content = String::new();
        // Add minimal header
        full_content.push_str(&format!("// Generated by Antigravity: {}\n\n", path));

        for (sig, code) in funcs {
            if let Some(c) = code {
                full_content.push_str(&c);
                full_content.push_str("\n\n");
            } else {
                // Phase 3: Stub out the signature if logic isn't written yet
                full_content.push_str(&sig);
                full_content.push_str(" {\n    // Implementation pending...\n}\n\n");
            }
        }

        Ok(full_content)
    }

    pub fn get_work_items_for_project(&self, project_id: i64) -> Result<Vec<WorkItem>> {
        let conn = self.conn.lock().unwrap();
        // Return work items for this project. Since project_id isn't directly in work_items,
        // we join with files/functions or just fetch all and filter by data_id if we know the context.
        // Actually, let's keep it simple: assume data_id points to project_id for Analyzation/Structure/Design/Frontend,
        // and for Backend it points to function_id. 
        // A better way is to fetch everything and filter logically in Rust for now if the schema is simple.
        
        // For now, let's just fetch all work items that belong to this project's functions or the project itself.
        let mut stmt = conn.prepare(
            "SELECT id, work_type, data_id, status, logs, current_log, preview_code, retries FROM work_items
             WHERE (work_type IN ('Analyzation', 'Structure', 'Design', 'Frontend', 'GenerateLayout') AND data_id = ?1)
             OR (work_type IN ('Backend', 'GenerateTest', 'GenerateCode', 'Testing') AND data_id IN (SELECT f.id FROM functions f JOIN files fl ON f.file_id = fl.id WHERE fl.project_id = ?1))"
        ).map_err(|e| anyhow!(e))?;
        
        let items = stmt.query_map(params![project_id], |row| {
             Ok(WorkItem {
                id: row.get(0)?,
                work_type: row.get(1)?,
                data_id: row.get(2)?,
                status: row.get(3)?,
                logs: row.get(4).unwrap_or_default(),
                current_log: row.get(5).unwrap_or_default(),
                preview_code: row.get(6).unwrap_or_default(),
                retries: row.get(7)?,
            })
        }).map_err(|e| anyhow!(e))?.collect::<SqlResult<Vec<_>>>().map_err(|e| anyhow!(e))?;
        Ok(items)
    }

    pub fn get_project_files(&self, project_id: i64) -> Result<Vec<Value>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, path, status FROM files WHERE project_id = ?1").map_err(|e| anyhow!(e))?;
        let rows = stmt.query_map(params![project_id], |row| {
            let file_id: i64 = row.get(0)?;
            let path: String = row.get(1)?;
            let status: String = row.get(2)?;
            
            Ok(json!({
                "id": file_id,
                "path": path,
                "status": status
            }))
        }).map_err(|e| anyhow!(e))?.collect::<SqlResult<Vec<_>>>().map_err(|e| anyhow!(e))?;
        Ok(rows)
    }

    pub fn get_next_pending_function(&self, project_id: i64) -> Result<Option<FunctionRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT f.id, f.file_id, f.name, f.signature, f.status FROM functions f
             JOIN files fl ON f.file_id = fl.id
             WHERE fl.project_id = ?1 AND f.status = 'Pending'
             LIMIT 1"
        ).map_err(|e| anyhow!(e))?;

        let mut rows = stmt.query_map(params![project_id], |row| {
             Ok(FunctionRecord {
                 id: row.get(0)?,
                 file_id: row.get(1)?,
                 name: row.get(2)?,
                 signature: row.get(3).unwrap_or_default(),
                 code: None,
                 status: row.get(4)?,
             })
        }).map_err(|e| anyhow!(e))?;
        
        if let Some(row) = rows.next() {
            Ok(Some(row.map_err(|e| anyhow!(e))?))
        } else {
            Ok(None)
        }
    }

    pub fn get_extended_project_layout(&self, project_id: i64) -> Result<Value> {
        let conn = self.conn.lock().unwrap();
        
        // Fetch files
        let mut stmt = conn.prepare("SELECT id, path, status FROM files WHERE project_id = ?1").map_err(|e| anyhow!(e))?;
        let files = stmt.query_map(params![project_id], |row| {
             let file_id: i64 = row.get(0)?;
             let path: String = row.get(1)?;
             let status: String = row.get(2)?;
             Ok((file_id, path, status))
        }).map_err(|e| anyhow!(e))?.collect::<SqlResult<Vec<_>>>().map_err(|e| anyhow!(e))?;

        let mut output = Vec::new();
        for (f_id, f_path, f_status) in files {
            // Fetch functions for this file
            let mut fstmt = conn.prepare("SELECT id, name, signature, status FROM functions WHERE file_id = ?1").map_err(|e| anyhow!(e))?;
            let functions = fstmt.query_map(params![f_id], |row| {
                Ok(json!({
                    "id": row.get::<_, i64>(0)?,
                    "name": row.get::<_, String>(1)?,
                    "signature": row.get::<_, String>(2)?,
                    "status": row.get::<_, String>(3)?
                }))
            }).map_err(|e| anyhow!(e))?.collect::<SqlResult<Vec<Value>>>().map_err(|e| anyhow!(e))?;

            output.push(json!({
                "id": f_id,
                "path": f_path,
                "status": f_status,
                "functions": functions
            }));
        }

        Ok(json!(output))
    }
}
