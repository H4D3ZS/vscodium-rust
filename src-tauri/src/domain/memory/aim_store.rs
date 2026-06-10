/// Kortex AIM Store — persistent binary Neural Weight-Map.
///
/// Format (little-endian):
///   [4]  magic  = b"AIM\x01"
///   [4]  version = 1u32
///   [8]  written_at  = Unix timestamp seconds (u64)
///   [4]  entry_count (u32)
///   for each entry:
///     [4]  key_len (u32)
///     [key_len]  key bytes (UTF-8 path or gist id)
///     [4]  val_len (u32)
///     [val_len]  value bytes (semantic gist, UTF-8)
///     [8]  mtime (u64) — file mtime at index time; 0 if unknown
///     [4]  weight (f32 as IEEE-754 LE) — relevance weight 0.0–1.0
///
/// Entries with expired TTL are silently dropped on load.

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const MAGIC: &[u8; 4] = b"AIM\x01";
const VERSION: u32 = 1;
const TTL_SECS: u64 = 86400 * 7; // 7-day TTL; re-index after a week

#[derive(Debug, Clone)]
pub struct AimEntry {
    pub key: String,
    pub gist: String,
    pub mtime: u64,
    pub weight: f32,
}

/// Read-only view of a .aim file for the in-IDE viewer.
#[derive(Debug, serde::Serialize)]
pub struct AimInspection {
    pub magic_ok: bool,
    pub version: u32,
    pub written_at: u64,
    pub entry_count: usize,
    pub size_bytes: u64,
    pub entries: Vec<AimInspectEntry>,
}

#[derive(Debug, serde::Serialize)]
pub struct AimInspectEntry {
    pub key: String,
    pub gist: String,
    pub mtime: u64,
    pub weight: f32,
}

#[derive(Debug, Default)]
pub struct AimStore {
    pub entries: HashMap<String, AimEntry>,
    pub written_at: u64,
}

impl AimStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, gist: impl Into<String>, mtime: u64, weight: f32) {
        let k = key.into();
        self.entries.insert(k.clone(), AimEntry { key: k, gist: gist.into(), mtime, weight });
    }

    /// Load from a .aim file. Returns empty store on any error or format mismatch.
    pub fn load(path: &Path) -> Self {
        match Self::try_load(path) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("[AIM] Failed to load {:?}: {}", path, e);
                Self::new()
            }
        }
    }

    fn try_load(path: &Path) -> io::Result<Self> {
        let data = fs::read(path)?;
        let mut cur = &data[..];

        // Magic
        let mut magic = [0u8; 4];
        cur.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "bad magic"));
        }

        // Version
        let version = read_u32(&mut cur)?;
        if version != VERSION {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "unsupported version"));
        }

        let written_at = read_u64(&mut cur)?;
        let now = unix_now();

        // Drop the whole cache if it's past TTL
        if now.saturating_sub(written_at) > TTL_SECS {
            tracing::info!("[AIM] Cache expired (written {}s ago), evicting", now - written_at);
            return Ok(Self::new());
        }

        let count = read_u32(&mut cur)? as usize;
        let mut entries = HashMap::with_capacity(count);

        for _ in 0..count {
            let key = read_string(&mut cur)?;
            let gist = read_string(&mut cur)?;
            let mtime = read_u64(&mut cur)?;
            let weight = read_f32(&mut cur)?;
            entries.insert(key.clone(), AimEntry { key, gist, mtime, weight });
        }

        Ok(Self { entries, written_at })
    }

    /// Inspect a .aim for VIEWING — parses the header + entries WITHOUT TTL
    /// eviction (so expired caches are still viewable) and caps the entry list
    /// so the payload stays sane. Used by the `aim_inspect` Tauri command.
    pub fn inspect(path: &Path, max_entries: usize) -> io::Result<AimInspection> {
        let data = fs::read(path)?;
        let size_bytes = data.len() as u64;
        let mut cur = &data[..];

        let mut magic = [0u8; 4];
        cur.read_exact(&mut magic)?;
        let magic_ok = &magic == MAGIC;
        if !magic_ok {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not an AIM file (bad magic)"));
        }
        let version = read_u32(&mut cur)?;
        let written_at = read_u64(&mut cur)?;
        let entry_count = read_u32(&mut cur)? as usize;

        let mut entries = Vec::new();
        for _ in 0..entry_count {
            let key = read_string(&mut cur)?;
            let gist = read_string(&mut cur)?;
            let mtime = read_u64(&mut cur)?;
            let weight = read_f32(&mut cur)?;
            if entries.len() < max_entries {
                entries.push(AimInspectEntry {
                    key,
                    gist: gist.chars().take(400).collect(),
                    mtime,
                    weight,
                });
            }
        }
        Ok(AimInspection { magic_ok, version, written_at, entry_count, size_bytes, entries })
    }

    /// Serialize to .aim file atomically (temp file + rename).
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let tmp = path.with_extension("aim_tmp");
        let mut buf = Vec::with_capacity(self.entries.len() * 128 + 20);

        // Header
        buf.extend_from_slice(MAGIC);
        buf.extend_from_slice(&VERSION.to_le_bytes());
        buf.extend_from_slice(&unix_now().to_le_bytes());
        buf.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());

        for entry in self.entries.values() {
            write_string(&mut buf, &entry.key);
            write_string(&mut buf, &entry.gist);
            buf.extend_from_slice(&entry.mtime.to_le_bytes());
            buf.extend_from_slice(&entry.weight.to_le_bytes());
        }

        let mut f = fs::File::create(&tmp)?;
        f.write_all(&buf)?;
        f.flush()?;
        f.sync_all()?;
        drop(f);

        fs::rename(&tmp, path)?;
        tracing::info!("[AIM] Saved {} entries to {:?}", self.entries.len(), path);
        Ok(())
    }

    /// Invalidate entries whose on-disk mtime differs from recorded mtime.
    pub fn invalidate_stale(&mut self, project_root: &Path) {
        self.entries.retain(|key, entry| {
            let p: PathBuf = project_root.join(key);
            match fs::metadata(&p).and_then(|m| m.modified()) {
                Ok(modified) => {
                    let disk_mtime = modified.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
                    disk_mtime == entry.mtime
                }
                Err(_) => false,
            }
        });
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn read_u32(cur: &mut &[u8]) -> io::Result<u32> {
    let mut b = [0u8; 4];
    cur.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

fn read_u64(cur: &mut &[u8]) -> io::Result<u64> {
    let mut b = [0u8; 8];
    cur.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}

fn read_f32(cur: &mut &[u8]) -> io::Result<f32> {
    let mut b = [0u8; 4];
    cur.read_exact(&mut b)?;
    Ok(f32::from_le_bytes(b))
}

fn read_string(cur: &mut &[u8]) -> io::Result<String> {
    let len = read_u32(cur)? as usize;
    let mut buf = vec![0u8; len];
    cur.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_string(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(&(s.len() as u32).to_le_bytes());
    buf.extend_from_slice(s.as_bytes());
}
