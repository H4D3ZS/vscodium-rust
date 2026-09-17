use rusqlite::Connection;
use std::path::PathBuf;

fn main() {
    let appdata = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    let db_path = appdata.join("com.hades.vscode-rust-app").join("specs.db");
    let conn = Connection::open(&db_path).expect("Failed to open DB");

    let mut stmt = conn
        .prepare("SELECT id, work_type, status, logs FROM work_items WHERE status = 'Failed'")
        .expect("Prepare failed");
    let rows = stmt
        .query_map([], |row| {
            let id: i64 = row.get(0)?;
            let work_type: String = row.get(1)?;
            let status: String = row.get(2)?;
            let logs: String = row.get(3)?;
            Ok((id, work_type, status, logs))
        })
        .expect("Query failed");

    for row in rows {
        let (id, work_type, status, logs) = row.unwrap();
        println!("Task ID: {}, Type: {}, Status: {}", id, work_type, status);
        println!("Logs: {}", logs);
        println!("--------------------------------------------------");
    }
}
