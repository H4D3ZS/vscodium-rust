use futures::channel::oneshot;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_expanded: Option<bool>,
    pub children: Option<Vec<FileEntry>>,
}

pub enum EditorCommand {
    Ping(oneshot::Sender<String>),
    OpenProject(PathBuf, oneshot::Sender<Result<(), String>>),
    GetFileTree(oneshot::Sender<Result<Vec<FileEntry>, String>>),
}

#[derive(Serialize, Clone)]
pub struct TerminalDataPayload {
    pub term_id: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub theme: String,
    pub font_size: u32,
}

#[derive(Serialize, Clone)]
pub struct HuntProgress {
    pub msg: String,
}

