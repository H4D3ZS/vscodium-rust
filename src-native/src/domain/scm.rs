#[derive(Clone, Debug)]
pub struct GitFileChange {
    pub path: String,
    pub status: GitStatusKind,
    pub staged: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitStatusKind {
    Modified,
    Added,
    Deleted,
    Untracked,
    Renamed,
}

#[derive(Clone, Debug)]
pub struct ScmStore {
    pub branch: String,
    pub changes: Vec<GitFileChange>,
    pub commit_message: String,
    pub is_committing: bool,
}

impl Default for ScmStore {
    fn default() -> Self {
        Self {
            branch: "main".to_string(),
            changes: Vec::new(),
            commit_message: String::new(),
            is_committing: false,
        }
    }
}
