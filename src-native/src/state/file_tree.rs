use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct ExplorerCreateState {
    pub is_dir: bool,
    pub name: String,
    pub parent_dir: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn populate_tree(dir: &Path, out: &mut Vec<FileNode>, depth_limit: usize) {
        if depth_limit == 0 || !dir.is_dir() {
            return;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut items = Vec::new();
            for entry in entries.flatten() {
                // Prevent infinite recursion on Windows directory junctions and symlinks
                if let Ok(ft) = entry.file_type() {
                    if ft.is_symlink() {
                        continue;
                    }
                }
                let p = entry.path();
                let name = p
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if name == ".git" || name == "target" || name == "node_modules" {
                    continue;
                }
                let is_dir = p.is_dir();
                let mut children = Vec::new();
                if is_dir && depth_limit > 1 {
                    Self::populate_tree(&p, &mut children, depth_limit - 1);
                }
                items.push(FileNode {
                    name,
                    path: p.to_string_lossy().to_string(),
                    is_dir,
                    is_expanded: false,
                    children,
                });
            }
            items.sort_by(|a, b| {
                if a.is_dir == b.is_dir {
                    a.name.cmp(&b.name)
                } else if a.is_dir {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            *out = items;
        }
    }

    pub fn toggle_in_nodes(nodes: &mut [FileNode], path: &str) -> bool {
        for node in nodes.iter_mut() {
            if node.path == path {
                node.is_expanded = !node.is_expanded;
                if node.is_dir && node.is_expanded && node.children.is_empty() {
                    Self::populate_tree(Path::new(&node.path), &mut node.children, 1);
                }
                return true;
            }
            if node.is_dir && Self::toggle_in_nodes(&mut node.children, path) {
                return true;
            }
        }
        false
    }
}
