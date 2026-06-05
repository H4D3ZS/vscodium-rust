use std::path::Path;

use crate::lsp_bundle::{self, ResolvedLaunch};

/// Legacy alias — use `ResolvedLaunch` from lsp_bundle.
pub struct LspLaunch {
    pub id: String,
    pub command: String,
    pub args: Vec<String>,
}

impl From<ResolvedLaunch> for LspLaunch {
    fn from(r: ResolvedLaunch) -> Self {
        Self {
            id: r.id,
            command: r.command,
            args: r.args,
        }
    }
}

/// Pick the best language server using IDE bundles first, then PATH.
pub fn detect_workspace_lsp(root: &Path, config_dir: &Path) -> Option<LspLaunch> {
    let id = lsp_bundle::workspace_lsp_id(root)?;
    lsp_bundle::resolve_launch(id, config_dir).map(Into::into)
}

pub async fn detect_workspace_lsp_async(root: &Path, config_dir: &Path) -> Result<LspLaunch, String> {
    let launch = lsp_bundle::ensure_workspace_lsp(root, config_dir).await?;
    Ok(launch.into())
}
